# Project policy

This document is the standing decision record for how `hyper-calendar` is built.
It exists so that a contributor — human or agent — can tell whether a change
belongs here without asking.

## 1. English is the repository language

All identifiers, comments, documentation, commit messages, issue text, test
names and error messages are in English.

This is not a claim that English is the important language. It is the opposite:
the *subject* of this library is that time is written differently everywhere, so
the library's own vocabulary has to stay out of the way. A single working
language keeps the code reviewable by anyone and keeps localisation where it
belongs — in data, behind `hc-i18n`, never hard-coded into logic.

Non-English text is welcome and expected **as data**: month names, era names,
holiday names, script samples and the test fixtures that verify them. A
Japanese era name belongs in a `&'static str` in a data table. It does not
belong in a function name.

## 2. Data is separated from algorithm

Every calendar, every holiday rule and every locale is expressed as data
interpreted by a small, shared algorithm. Concretely:

- Calendars implement one trait with two operations against a fixed day number
  (see [architecture.md](architecture.md)).
- Leap seconds are a table, not a formula.
- Holidays are rule values evaluated by one engine, not per-country code.
- Locale vocabulary is a static table with a fallback chain.

The test is simple: **adding Bolivia's holidays, or the Tibetan calendar, or
Welsh plural rules, should mean adding a data entry, not editing control flow.**
When a change forces a branch into shared logic, that is a signal the
abstraction is wrong, not that the new case is special.

## 3. Precision is stated, never implied

Anything this library returns carries a claim about how well it is known.

- Where a value is exact, it is computed exactly. Durations are integers of
  attoseconds, not floats. Calendar conversions are integer arithmetic.
- Where a value comes from a fitted physical model — ΔT, TDB, solar longitude,
  a sunrise time — the crate's README and the function's doc comment state the
  accuracy and the era over which it holds.
- Where a value is genuinely unknown, `hc-uncertainty` says so rather than
  returning a number with false confidence.

A wrong answer delivered confidently is worse than a refusal.

## 4. The library refuses to guess

Some questions have no answer, and the API says so instead of inventing one:

- **Future leap seconds.** The IERS announces them about six months ahead. Past
  that, `LeapPolicy::Strict` returns `AfterModelEnd`. A caller who wants a
  forecast must ask for one.
- **UT1 before it was measured.** `DUT1` is observational. The library
  interpolates within a supplied series and refuses to extrapolate outside it.
- **Historical proclamations.** Many calendars were, in practice, whatever an
  authority announced. Computed Hijri dates, pre-modern Chinese dates and
  pre-reform Julian dates can disagree with what was actually observed or
  decreed. Every such calendar is flagged `is_astronomical` or carries a
  documented range, and the README says what it can and cannot be trusted for.
- **Local time that does not exist.** When a DST transition skips an hour,
  `hc-tz` returns `Nonexistent` rather than silently shifting.

## 5. Modularity is a compile-time property

Requirement 8 of the original brief: not everything should be compiled into
everything. The workspace is split so that a caller who wants Gregorian dates
and nothing else pays for Gregorian dates and nothing else.

- Each capability is its own crate.
- The `hyper-calendar` facade exposes each as an optional feature.
- `default` is deliberately modest (`std`, `civil`, `format`, `i18n`).
- Every crate builds under `--no-default-features --features alloc`, and the
  core builds under `no_std` with `libm`.
- No crate depends on another unless it genuinely needs it. The dependency
  graph is a DAG and is documented in [architecture.md](architecture.md).

## 6. Correctness is demonstrated, not asserted

- Every conversion is round-trip tested over its whole supported range.
- Every algorithm is anchored to at least one independently published
  reference value, cited in a comment.
- Boundaries are tested explicitly: epochs, calendar reforms, leap days, leap
  months, leap seconds, the first and last supported day.
- Error paths are tested. A function that can return `MonthOutOfRange` has a
  test that makes it do so.
- CI runs `cargo test`, `cargo clippy -D warnings`, `cargo fmt --check`, a
  `no_std` build, a WebAssembly build, a shared-library build and
  `cargo audit` on every pull request. Coverage is reported by octocov with a
  70% floor.

## 7. `unwrap` and `expect` are forbidden outside tests

`.cargo/config.toml` sets `-Dclippy::unwrap_used` and
`-Dclippy::expect_used`. A library that panics is a library that cannot be
embedded in a WebAssembly runtime or behind an FFI boundary. Fallible
operations return `Result`; infallible ones are proved infallible by
construction.

The two `impl Add`/`impl Sub` operators on `Duration` are the deliberate
exception: they panic on overflow so that ordinary arithmetic reads normally,
and every one of them has a `checked_*` twin.

## 8. No dependencies without a reason

The workspace has exactly one optional external dependency: `libm`, for
floating-point math on `no_std` targets that lack it. Everything else —
parsing, formatting, locale data, astronomy, the TZif reader — is implemented
here.

This is a cost, and it is paid deliberately. A date library is a dependency of
everything else; it should not drag a tree behind it. It also keeps the
WebAssembly artefact small and the `cargo audit` surface near zero.

## 9. Scope

`hyper-calendar` computes and formats. It has no UI, no I/O beyond optionally
reading a TZif file, no clock (the caller supplies the current time), no
network access and no global state. A web front end will eventually consume it
through the WebAssembly or C surface; that front end is not part of this
repository.
