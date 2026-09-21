# hc-tz

Time zones for [`hyper-calendar`](https://github.com/kitsuyui/hyper-calendar):
UTC offsets, POSIX `TZ` rules, and a reader for the binary TZif (IANA) format.

A time zone is the rule that turns an instant into a wall-clock reading and
back. This crate holds three kinds of rule behind one object-safe trait,
`TimeZone`:

| Kind | Type | Knows about history? |
| --- | --- | --- |
| One offset, forever | `FixedTimeZone`, `Utc` | Nothing to know |
| A POSIX `TZ` string | `PosixTimeZone` | No — current rules only |
| A TZif file | `TzifTimeZone` | Yes — every recorded transition |

## The part most libraries get wrong

Instant → local reading is a function. Local reading → instant is not.

* When clocks go back, an hour repeats and a reading names **two** instants.
* When clocks go forward, an hour is skipped and a reading names **none**.

`TimeZone::resolve_local` therefore returns a three-way `LocalResolution`:

```rust
use hc_tz::{Disambiguation, LocalResolution, TimeZone, builtin};
use hc_calendar::{CivilDateTime, CivilTime, Rd};

fn example() -> Result<(), Box<dyn core::error::Error>> {
    let new_york = builtin::zone("America/New_York")?;

    // 2024-11-03 01:30 happened twice in New York.
    let repeated = CivilDateTime::new(Rd(739_193), CivilTime::hms(1, 30, 0)?);
    assert!(matches!(
        new_york.resolve_local(repeated),
        LocalResolution::Ambiguous { .. }
    ));

    // 2024-03-10 02:30 never happened at all.
    let skipped = CivilDateTime::new(Rd(738_955), CivilTime::hms(2, 30, 0)?);
    assert!(new_york.resolve_local(skipped).is_nonexistent());

    // The caller names the policy; the library never picks one silently.
    let instant = new_york.unix_at(repeated, Disambiguation::Earliest)?;
    assert_eq!(instant.seconds(), 1_730_611_800);
    Ok(())
}
```

`Nonexistent` reports the skipped local interval (`gap_start`, `gap_end`), the
instant the clocks jumped, both offsets, and the two instants the reading would
name under each — everything a caller needs to explain the problem to a user.
`Disambiguation` offers `Earliest`, `Latest`, `Reject` and `PushForward` (the
`compatible` policy of ECMAScript Temporal and `java.time`). It has no
`Default`, deliberately.

## What each module covers

* **`offset`** — `UtcOffset`, whole seconds, validated to
  `-25:59:59 ..= +25:59:59` (RFC 8536's range for a TZif `utoff`). Parses `Z`,
  `±hh`, `±hh:mm`, `±hhmm`, `±hh:mm:ss`, `±hhmmss`; renders any of them into a
  stack buffer, so `no_std` builds need no allocator. Applies to and from
  `hc_calendar::CivilDateTime`.
* **`zone`** — the `TimeZone` trait, `LocalResolution`, `Disambiguation`, and a
  reusable resolver that derives the three-way answer from nothing but a
  zone's instant-to-offset function.
* **`fixed`** — `FixedTimeZone` and `Utc`.
* **`posix`** — the full POSIX.1-2017 §8.3 grammar as extended by RFC 8536
  §3.3: `<>`-quoted abbreviations, the `Jn`, `n` and `Mm.w.d` rule forms, the
  `/time` suffix with hours from −167 to 167, the implied one-hour daylight
  offset, and tzcode's US fallback rules when a daylight name carries none.
  Parses, evaluates and renders (round-trip exact for every string in the
  built-in table).
* **`tzif`** — RFC 8536 versions 1, 2 and 3: header, transition times and
  types, local time type records, designation strings, leap-second records,
  the standard/wall and UT/local indicators, and the POSIX footer. Parsing
  borrows from a `&[u8]` and allocates nothing; a version 2 or 3 file is read
  through its 64-bit block, and the footer governs instants after the last
  recorded transition, as the RFC requires.
* **`system`** (`std` only) — reads bytes out of `/usr/share/zoneinfo` (or
  `$TZDIR`). Kept apart from the parser on purpose: a parser over a slice can
  be tested against a fixture, a loader cannot. Zone names are validated
  before being joined onto a path, because a name is usually data from outside
  the program.
* **`builtin`** — seventeen zones as POSIX strings, for targets with no zone
  database.

## Accuracy, and where the data came from

* Offsets, transition instants and the daylight flag are **exact integers**.
  No floating point appears anywhere in this crate.
* Everything works in **POSIX time**: every day is 86 400 seconds and leap
  seconds do not exist. That is the timeline civil zone rules are published
  in. Converting to elapsed physical time is `hc_core::unix`'s job and needs a
  leap-second policy, which is a separate decision. Applying an offset to a
  UTC `23:59:60` therefore collapses it onto the following second, exactly as
  POSIX time does.
* **A POSIX `TZ` string cannot express history.** It has one pair of rules and
  applies them to every year, so `builtin` and any `PosixTimeZone` are right
  about the present and wrong about the past: they do not know that the United
  States moved its transitions in 2007, that Brazil stopped changing its
  clocks in 2019, or that Russia abolished daylight saving in 2011. **For
  historical accuracy, read TZif data.** The README of the IANA database makes
  the same point about its own POSIX footers.
* Reference data: the POSIX strings in `builtin` are the footers of the
  corresponding files in the IANA time zone database, release 2026c, as
  shipped in `/usr/share/zoneinfo`. Transition instants asserted in the tests
  come from the published rules — the United States Energy Policy Act of 2005,
  EU Directive 2000/84/EC, the Australian and New Zealand state rules — and
  are cross-checked against the system database wherever one is present.
* Gregorian arithmetic (needed for `Mm.w.d` and `Jn` rules) is the standard
  Rata Die formulation from Reingold and Dershowitz, *Calendrical
  Calculations* (4th ed., §2.2). **It lives privately in this crate only until
  `hc-calendars-solar` lands**, at which point the private `gregorian` module
  should be deleted and its three functions taken from there instead.

## Deliberate omissions

* **No zone database is compiled in.** Seventeen POSIX strings are not a
  database; a real one is megabytes and belongs on disk or in a separate data
  crate.
* **No "local time zone" detection.** Reading `/etc/localtime` or `%TZ%` is a
  platform question, and a library that guesses the user's zone is a library
  that is wrong on servers.
* **No `right/` leap-second semantics.** `tzif` reads and reports leap-second
  records faithfully, but does not apply them: `hc_core::leap` is the one
  authoritative table in this workspace, and having two would mean having two
  answers.
* **No abbreviation-to-zone lookup.** `CST` is three different zones and
  `IST` is at least three; the mapping does not exist, so the crate does not
  pretend it does.
* **No `Instant`/TAI conversions.** Compose with `hc_core::unix` instead.

## Features

`default = ["std"]`, `std = ["alloc", ...]`, `alloc = [...]`. The crate builds
with `--no-default-features`; only the `system` module needs `std`.

## Tests

106 tests, covering the 2007 United States rule change, the ambiguous and
nonexistent hours at both ends of American and European daylight saving,
southern-hemisphere rules in Sydney and Auckland, Lord Howe Island's
half-hour shift, Kathmandu's `+05:45`, Cairo's last-Thursday-at-24:00 rule,
zones with no daylight saving at all, the three TZif versions against
handcrafted byte fixtures, and hour-by-hour round trips across whole years.
The tests that read `/usr/share/zoneinfo` skip cleanly when it is absent.
