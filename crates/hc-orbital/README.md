# hc-orbital

Milankovitch orbital elements for the `hyper-calendar` workspace: the
eccentricity of Earth's orbit, the obliquity of its axis, the longitude of
perihelion from the moving equinox and the climatic precession *e* sin ϖ,
for a million years either side of 1950, and the daily insolation that
follows from them.

The solution is Berger's 1978 trigonometric series — 19 terms for the
eccentricity vector, 47 for the obliquity, 78 for the general precession —
the one the PMIP palaeoclimate experiments prescribe and the NCAR and NASA
GISS climate models carry.

## What it covers

| Module | Holds |
| --- | --- |
| `elements` | `elements_at(years_before_1950)`: *e*, ε, ϖ, *e* sin ϖ, each an `Uncertain` whose error bar is a measured spread; `EPOCH_YEAR` (1950), `VALID_SPAN` (±1 000 000 years), the year conversion, `SOURCE` |
| `insolation` | `daily_insolation(elements, latitude, solar longitude, solar constant)` and `insolation_65n_june`; the solar constants of the 1978 tables, the 1991 tables and the 2014 program, by name |
| `series` | The three coefficient tables as the author deposited them |

```rust
use hc_orbital::{SOLAR_CONSTANT_BERGER_LOUTRE_1991, elements_at, insolation_65n_june};

let lgm = elements_at(21_000.0).unwrap();          // the Last Glacial Maximum
assert!((lgm.eccentricity.value - 0.018994).abs() < 1e-6);
assert!((lgm.obliquity_degrees.value - 22.949).abs() < 1e-3);
let w = insolation_65n_june(11_000.0, SOLAR_CONSTANT_BERGER_LOUTRE_1991).unwrap();
assert!(w > 520.0);                                  // the early-Holocene peak
assert!(elements_at(1_000_001.0).is_err());          // past the span: refused
```

## Epoch and span

"Present" is 1950 CE, the series' own `t = 0`, which is also the
radiocarbon BP datum `hc-deep-time` uses. Arguments are years before 1950,
negative in the future; `years_before_present_from_year` converts from a
calendar year in astronomical numbering.

The span is ±1 000 000 years, the bound the author's program, GISS and NCAR
state. Beyond it the crate refuses rather than extrapolates (ADR 0006).
Inside it the accuracy is not uniform, and the error bars say so: they are
the measured disagreement with Berger & Loutre's 1991 solution, in three
tiers — to 100 kyr, to 800 kyr, to 1 Myr.

## Sources

**The series** — Berger, A. (1978), *Long-term variations of daily
insolation and Quaternary climatic changes*, J. Atmos. Sci. 35, 2362–2367.
The paper itself could not be read (the publisher's site refused the
request); its coefficient tables were read from the author's own deposit,
`INSOL.IN` and the program `insol14.f` ("BERGER 78 version 2014"), Zenodo
record 7198109, CC BY 4.0, and the combination of the sums, the sign of
time and the 180° between the heliocentric and geocentric perihelion follow
that program. The transcription was generated from the file and is checked
against NASA GISS's evaluation of the same series to seven figures and
against the author's own 1978 tables (NOAA paleoclimatology `bein1.dat`).

**The anchors** — the PMIP mid-Holocene and Last Glacial Maximum
parameters (Braconnot et al. 2012); NOAA's `orbit91` (Berger & Loutre
1991) for the 65° N July insolation and for the accuracy measurement;
Meeus, *Astronomical Algorithms*, for the present-day obliquity and
eccentricity.

## Accuracy claimed

- Against the author's own tabulation of this solution: *e* to 5 × 10⁻⁷,
  ε to 0.0005°, ϖ to 0.005°, *e* sin ϖ to 5 × 10⁻⁶ — the tables' own
  rounding — over 0–100 kyr BP, and the insolation to ±0.25 W m⁻², the
  rounding of a table printed in whole langleys per day.
- Against the 1991 solution: *e* within 0.002, ε within 0.05°, *e* sin ϖ
  within 0.0025 to 100 kyr BP; 0.009, 0.20°, 0.008 to 800 kyr; 0.010,
  0.27°, 0.015 to 1 Myr. The longitude of perihelion is ill-conditioned
  when *e* is small and its error bar grows accordingly, to 180° when *e*
  falls below the precession's spread.
- The future half of the span is unmeasured (the 1991 table stops at 1950)
  and is given the past's figures.

## What it deliberately does not do

- **No later solution.** Berger & Loutre 1991 and Laskar et al. 2004 are
  the successors; the latter is the modern reference and is data by the
  megabyte. Either would be a second named solution, not a parameter.
- **No calendar dates.** Insolation is asked for at a solar longitude, not
  a date; the 365-day mean-anomaly convention Berger's program uses for
  dates is not a calendar and is left out.
- **No mid-month tables.** The convention (λ = 0°, 30°, … for March,
  April, …) is documented and the longitudes are the caller's to pass.
