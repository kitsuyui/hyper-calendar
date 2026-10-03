# hc-uncertainty

Time that is not exactly known — significant figures, error bars, interval
arithmetic, fuzzy instants and EDTF — for the `hyper-calendar` workspace.

Almost nothing outside a laboratory has an exact timestamp. A charter is dated
"in the third year of the king"; a sample comes back as `3200 ± 50 BP`; a
catalogue says `1667 or 1668`; the universe is 13.8 billion years old, which is
three digits and not eleven. A date library that can only hold exact instants
must either refuse these or invent the missing precision. Inventing it is
worse.

It depends on `hc-core`, and on `hc-calendar` for the proleptic Gregorian
day arithmetic that places an EDTF date: `hc_calendar::gregorian` is the one
implementation of it in the workspace (policy §2), and this crate adds only
a thin adapter that reaches the years EDTF's `Y` form writes beyond its
±9 999 999 range.

```rust
use hc_uncertainty::{Significant, Uncertain};

// Three significant figures stay three when the value is written out.
assert_eq!(Significant::new(13.8e9, 3)?.to_string(), "1.38e10");

// Independent Gaussian errors add in quadrature: 0.3 and 0.4 give 0.5.
let sum = Uncertain::new(10.0, 0.3)?.checked_add(Uncertain::new(20.0, 0.4)?)?;
assert_eq!(sum.value, 30.0);
assert!((sum.std_dev - 0.5).abs() < 1e-12);
# Ok::<(), hc_uncertainty::UncertaintyError>(())
```

## What it covers

| Module | Type | What it holds |
| --- | --- | --- |
| `sig_figs` | `Significant` | A value plus the number of digits claimed, propagated through arithmetic **and through rendering**. |
| `quantity` | `Uncertain` | A Gaussian `value ± σ`, with first-order (delta-method) propagation for `+ - * /`, `powf`, `ln`, `exp`, and the inverse-variance weighted mean. |
| `interval` | `DurationInterval` | A closed `[lo, hi]` of `hc_core::Duration` with guaranteed-enclosure interval arithmetic. |
| `fuzzy` | `FuzzyInstant` | Exact, resolved, bounded, Gaussian, open-ended or unknown instants, with Allen's thirteen interval relations over pairs of them. |
| `edtf` | `EdtfValue` | A subset of the ISO 8601-2 Extended Date/Time Format, parsed and rendered. |

Highlights:

- `Significant::new(13.8e9, 3).to_string()` is `"1.38e10"`, never
  `"13800000000"`. Scientific notation is chosen exactly when plain decimal
  would show a digit that is not claimed. The printed form, `rounded()`,
  `decimal_exponent()` and `last_significant_place()` all read one rounded
  numeral: `9.96` to two figures is `10` everywhere, `2.5` to one figure is
  `2`. An exact value (`MAX_FIGURES`) prints the shortest numeral that reads
  back as the same `f64`: `0.1`, not `0.10000000000000001`.
- `Uncertain::combine` is the inverse-variance weighted mean — the estimator a
  review paper uses to merge two published values — and `to_significant`
  quotes the error bar to two digits and lets its last digit fix the last
  significant place of the value (CODATA's convention, allowed by GUM 7.2.6;
  the Particle Data Group varies the digits of σ with its leading three
  digits and that is not implemented).
- `FuzzyInstant::relations` returns a **set** of Allen relations, not one.
  Two fully determined spans give a singleton; `Before(1500)` against a known
  span that ends before 1500 gives five; two `Unknown`s give all thirteen.
- EDTF round-trips: `parse(s).to_string() == s` for every supported form,
  covered by a test enumerating them.

## What it does not carry

- **No calendars.** Everything is expressed against `hc_core::Instant` and
  `hc_core::Duration`. Turning "the third century BC" into a pair of instants
  belongs to a calendar crate; EDTF's proleptic Gregorian dates are placed
  with `hc-calendar`'s arithmetic, not a copy of it.
- **Not carried: Monte Carlo, or any other distribution.** `Uncertain` is a
  linear approximation and says so. It assumes independent inputs —
  `x.checked_add(x)` gives `σ√2`, not `2σ`; use `scaled` when the correlation
  is total — and it degrades once `σ/|x|` passes roughly 0.1 (a figure of this
  crate's; no source read gives it). A wider `±`
  would not fix that; a distribution would, and is not yet done.
- **No tightening of interval arithmetic.** `A - A` is not zero. The
  dependency problem is inherent to the method, so there is no "simplify"
  operation that would pretend otherwise.
- **Not carried: a disjunctive fuzzy instant.** An EDTF set collapses to its
  hull, losing the gaps, and `to_fuzzy_instant` documents that at the call
  site. `FuzzyInstant` holds only intervals, and a set of them is not yet
  done.
- **Not carried: parts of EDTF**, rejected rather than half-parsed (policy §4):
  - times of day (Level 0, `1985-04-12T23:20:30Z`), not yet done;
  - seasons and sub-year divisions (`2001-21`, `2001-34`), whose boundaries
    are conventions that differ by hemisphere and publisher: policy §5 would
    give each its own name, none has been added, and no boundary is guessed;
  - component-level qualification (Level 2, `2004-06~-11`: the year and the
    month approximate, the day not), which implies a support that is not an
    interval;
  - exponential years and significant digits (Level 2, `Y-17E7`, `1950S2`),
    not yet done;
  - an unspecified digit that is not at the end of its component (`1984-1X`,
    `1X84`).

## Accuracy claimed

- `Significant` rounds by NIST SP 811, B.7.1 on the shortest decimal numeral
  of the value: below one half down, above one half up, exactly one half to
  the even digit (`2.5` to one figure is `2`, `3.5` is `4`, `2.675` to three
  figures is `2.68`). It is stable under repetition. The figure count is
  capped at 17, the most an IEEE-754 double can identify. Before this
  revision the rounding was half-away-from-zero while the printed form used
  Rust's ties-to-even on the binary value, so `2.5` printed `2` and rounded
  to `3`, and a value that rounded into a new decade printed an extra figure
  (`9.96` to two figures printed `10.0`).
- `Uncertain` propagation is exact for linear functions and correct to `O(σ²)`
  otherwise. No `f32` appears anywhere.
- `DurationInterval` is exact: bounds are `hc_core::Duration`, and the
  midpoint is halved componentwise rather than through an attosecond total, so
  it stays correct past the 5.4×10¹² years at which such a total overflows.
- EDTF dates are placed by counting 86 400-second days from the 1970 epoch on
  TAI. This ignores leap seconds, so a converted date is displaced from true
  TAI by `TAI − UTC`: under 40 seconds since UTC began in 1961, and undefined
  before that, when there was no UTC to be displaced from. At a resolution of one day that is irrelevant, and an exact
  conversion would need a UTC table covering barely 1 % of EDTF's range.
- The Gaussian-to-support cut is three σ (99.73 %); the flat-range-to-σ
  conversion is `w/√12`, the true standard deviation of a uniform
  distribution.

## Where the reference data came from

- Allen's thirteen relations, their names and their symbols: James F. Allen,
  *Maintaining knowledge about temporal intervals*, CACM 26(11), 1983.
- The uniform-distribution factor `w/√12`: JCGM 100:2008, the *GUM*, §4.3.7.
- Rounding: NIST Guide to the SI (SP 811), Appendix B.7.1, rules 1 to 3
  (`nist-sp811-b7`), read on nist.gov (HTML) on 2026-10-03; its examples 6.974 951 5 and
  6.974 950 5 are tests here. The even-digit rule on exactly one half is
  that source's; other conventions (round half up) exist and are not carried.
- Two digits of the error bar, and the estimate rounded to match: GUM 7.2.6
  (JCGM 100:2008, `jcgm100-gum-7-2-6`), including its example 10.057 62 Ω with 27 mΩ reported as
  10.058 Ω. **Not read from the source**: the JCGM's HTML at iso.org answered
  the fetch with a bot challenge and the PDF is not opened (policy), so the
  sentence and the example are as a search summary quoted them; the example
  is a test and agrees with the rule as implemented.
- The Particle Data Group's variable rule (two digits of σ when its leading
  three digits are 100–354, one when 355–949, and 950–999 rounded up to 1000
  with two digits kept) was read only as a quotation of PDG 2011 §5.3 in the
  r-quantities/errors issue 45 on GitHub (`pdg-rounding-quoted-errors-45`);
  the PDG's own text is a PDF and was
  not opened. It is not implemented.
- The two proleptic Gregorian Rata Die formulas, and the `1945-11-12 = RD
  710347` and `1970-01-01 = RD 719163` anchors used to test them: Reingold and
  Dershowitz, *Calendrical Calculations*, 4th ed., §2.3 and Appendix C. They
  are carried privately here only because this crate depends on `hc-core`
  alone; they are not re-exported.
- The date syntax: ISO 8601-2:2019, as summarised by the Library of Congress
  EDTF specification.

## Feature flags

| Feature | Effect |
| --- | --- |
| `std` (default) | platform floating-point math; implies `alloc` |
| `alloc` | the EDTF set and list forms (`[...]`, `{...}`); without it they return `UncertaintyError::Unsupported` rather than parsing less |
| `edtf` (default) | the `edtf` module, and with it the `hc-calendar` dependency its Gregorian day arithmetic comes from |
| `libm` | software floating-point math through `hc-core`, for `no_std` targets |

A build with neither `std` nor `libm` does not compile: `hc-core` refuses
it. So a `no_std` build is `--no-default-features --features libm`, with
`alloc` added for the set and list forms.
