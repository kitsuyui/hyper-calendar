# hc-uncertainty

Time that is not exactly known — significant figures, error bars, interval
arithmetic, fuzzy instants and EDTF — for the `hyper-calendar` workspace.

Almost nothing outside a laboratory has an exact timestamp. A charter is dated
"in the third year of the king"; a sample comes back as `3200 ± 50 BP`; a
catalogue says `1667 or 1668`; the universe is 13.8 billion years old, which is
three digits and not eleven. A date library that can only hold exact instants
must either refuse these or invent the missing precision. Inventing it is
worse.

## What it covers

| Module | Type | What it holds |
| --- | --- | --- |
| `sig_figs` | `Significant` | A value plus the number of digits actually claimed, propagated through arithmetic **and through rendering**. |
| `quantity` | `Uncertain` | A Gaussian `value ± σ`, with first-order (delta-method) propagation for `+ - * /`, `powf`, `ln`, `exp`, and the inverse-variance weighted mean. |
| `interval` | `DurationInterval` | A closed `[lo, hi]` of `hc_core::Duration` with guaranteed-enclosure interval arithmetic. |
| `fuzzy` | `FuzzyInstant` | Exact, resolved, bounded, Gaussian, open-ended or unknown instants, with Allen's thirteen interval relations over pairs of them. |
| `edtf` | `EdtfValue` | ISO 8601-2 Extended Date/Time Format, parsed and rendered. |

Highlights:

- `Significant::new(13.8e9, 3).to_string()` is `"1.38e10"`, never
  `"13800000000"`. Scientific notation is chosen exactly when plain decimal
  would show a digit that is not claimed.
- `Uncertain::combine` is the inverse-variance weighted mean — the estimator a
  review paper uses to merge two published values — and `to_significant`
  applies the Particle Data Group's rule that the error bar fixes the last
  significant place of the value.
- `FuzzyInstant::relations` returns a **set** of Allen relations, not one.
  Two fully determined spans give a singleton; `Before(1500)` against a known
  span gives five; two `Unknown`s give all thirteen.
- EDTF round-trips: `parse(s).to_string() == s` for every supported form,
  covered by a test enumerating them.

## What it deliberately does not do

- **No calendars.** Everything is expressed against `hc_core::Instant` and
  `hc_core::Duration`. Turning "the third century BC" into a pair of instants
  belongs to a calendar crate.
- **No Monte Carlo.** `Uncertain` is a linear approximation and says so. It
  assumes independent inputs — `x.checked_add(x)` gives `σ√2`, not `2σ`; use
  `scaled` when the correlation is total — and it degrades once `σ/|x|` passes
  roughly 0.1. A wider `±` would not fix that; a distribution would, and that
  is out of scope.
- **No tightening of interval arithmetic.** `A - A` is not zero. The
  dependency problem is inherent to the method, so there is no "simplify"
  operation that would pretend otherwise.
- **No disjunctive fuzzy instant.** An EDTF set collapses to its hull, losing
  the gaps, and `to_fuzzy_instant` documents that at the call site.
- **EDTF gaps, rejected rather than half-parsed:** times of day
  (`1985-04-12T23:20:30Z`), seasons and sub-year divisions (`2001-21`),
  component-level qualification (`2004-06~-11`), and exponential years
  (`Y17E7S3`). Seasons are left out because their boundaries are a convention
  that differs by hemisphere and publisher; guessing one would be inventing
  data.

## Accuracy claimed

- `Significant` rounding is half-away-from-zero and stable under repetition;
  the figure count is capped at 17, the most an IEEE-754 double can identify.
- `Uncertain` propagation is exact for linear functions and correct to `O(σ²)`
  otherwise. No `f32` appears anywhere.
- `DurationInterval` is exact: bounds are `hc_core::Duration`, and the
  midpoint is halved componentwise rather than through an attosecond total, so
  it stays correct for spans of millions of years rather than failing past 5.4.
- EDTF dates are placed by counting 86 400-second days from the 1970 epoch on
  TAI. This ignores leap seconds, so a converted date is displaced from true
  TAI by the accumulated offset: under 40 seconds in the leap-second era, zero
  before 1972. At a resolution of one day that is irrelevant, and an exact
  conversion would need a UTC table covering barely 1 % of EDTF's range.
- The Gaussian-to-support cut is three σ (99.73 %); the flat-range-to-σ
  conversion is `w/√12`, the true standard deviation of a uniform
  distribution.

## Where the reference data came from

- Allen's thirteen relations, their names and their symbols: James F. Allen,
  *Maintaining knowledge about temporal intervals*, CACM 26(11), 1983.
- The uniform-distribution factor `w/√12`: JCGM 100:2008, the *GUM*, §4.3.7.
- Significant-figure and error-bar reporting conventions: the SI Brochure and
  the Particle Data Group's review style.
- The two proleptic Gregorian Rata Die formulas, and the `1945-11-12 = RD
  710347` and `1970-01-01 = RD 719163` anchors used to test them: Reingold and
  Dershowitz, *Calendrical Calculations*, 4th ed., §2.3 and Appendix C. They
  are carried privately here only because this crate depends on `hc-core`
  alone; they are not re-exported.
- The date syntax: ISO 8601-2:2019, as summarised by the Library of Congress
  EDTF specification.

## Feature flags

`default = ["std"]`, `std = ["alloc", ...]`, `alloc = [...]`. The crate builds
with `--no-default-features --features alloc`; the EDTF set and list forms
(`[...]`, `{...}`) need `alloc` and report
`UncertaintyError::Unsupported` without it rather than silently parsing less.
