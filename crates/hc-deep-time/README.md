# hc-deep-time

The spans an ordinary calendar cannot reach, in both directions — Planck time
to 10¹⁰⁰ years — for the `hyper-calendar` workspace.

`hc_core::Duration` is exact: `i128` seconds plus attoseconds. It is the right
representation for everything a clock can measure and the wrong one for
everything it cannot. A Planck time is twenty-six decades below an attosecond,
and it is not exactly known anyway; the age of the universe is 13.787 ± 0.020
Gyr, which is five significant figures and not eighteen. So values out there
live here instead, as *published magnitudes with error bars*, and the type
system stops the error bars being dropped on the way.

## What it covers

| Module | Holds |
| --- | --- |
| `magnitude` | `DeepTime` — seconds as an `Uncertain` plus a figure count; seventeen unit constructors; `log10_ratio`, `orders_of_magnitude_between`, logarithmic rendering; lossy conversion to and from `hc_core::Duration` |
| `constants` | The Planck units from CODATA 2022, each with its standard uncertainty and its source |
| `universe` | Eleven cosmic epochs and nine dated events, Planck epoch to the present |
| `future` | Eight dated events and four eras, out past 10¹⁰⁰ years, plus `black_hole_lifetime` |
| `geologic` | 175 intervals of the ICS chart in five ranks, queryable as a tree |
| `archaeology` | The BP convention, the calibrated/uncalibrated distinction, eleven conventional periods |
| `periods` | Two long astronomical recurrences, the precession of the equinoxes and the galactic year, with their spreads and whether they drift |
| `names` | The chart's interval names in fourteen other languages, from the ICS's own translations |
| `timeline` | All four chronologies queried together, with the uncertainty carried through; a moment up to a century ahead (`PRESENT_HORIZON_YEARS`) is still in the intervals that end at the present |

Highlights:

- `DeepTime::from_gigayears(13.787, 0.020)` renders as `4.3508e17 s ± 6.3e14`,
  never as eighteen digits, and `.logarithmic()` gives `10^(17.639 ± 0.001) s`.
- The ratio of the age of the universe to the Planck time is 8.07×10⁶⁰, or
  60.907 decades, and there is a test that says so.
- `Bp::uncalibrated(3200.0, 50.0).to_calendar_year()` returns
  `Err(CalibrationRequired)`. Radiocarbon years are not calendar years, and
  this crate will not pretend otherwise.
- `black_hole_lifetime(1.0)` is 2.095×10⁶⁷ years with a relative uncertainty of
  2.2×10⁻⁵ — exactly `G`'s, because `G` is the only uncertain constant in the
  formula once the mass is written as `GM☉/G`.

## Sources, table by table

**Planck units and constants** — CODATA 2022, from the NIST *Fundamental
Physical Constants — Complete Listing*,
<https://physics.nist.gov/cuu/Constants/Table/allascii.txt>, retrieved
2026-09-26; the underlying paper is Mohr, Newell, Taylor & Tiesinga,
arXiv:2409.03787 (`codata2022`). Planck time
5.391247(60)×10⁻⁴⁴ s, length 1.616255(18)×10⁻³⁵ m, mass 2.176434(24)×10⁻⁸ kg,
temperature 1.416784(16)×10³² K, `G` = 6.67430(15)×10⁻¹¹ m³ kg⁻¹ s⁻². The
Planck energy in joules is derived as `m_P c²` and checked against NIST's
tabulated 1.220890(14)×10¹⁹ GeV.

**Cosmic chronology** — Planck Collaboration, *Planck 2018 results. VI.
Cosmological parameters*, A&A 641, A6 (2020), arXiv:1807.06209
(`planck2018-vi`), Table 2,
`TT,TE,EE+lowE+lensing+BAO` throughout: age 13.787 ± 0.020 Gyr, `H0` = 67.66 ±
0.42, `Ωm` = 0.3111 ± 0.0056, `z*` = 1089.80 ± 0.21, `z_eq` = 3387 ± 21. Ages
in years for matter–radiation equality (5.14×10⁴ yr), recombination (3.72×10⁵
yr), reionisation and the first galaxies were obtained by integrating the flat
ΛCDM Friedmann equation with those parameters; run to `a = 1` the same integral
returns 13.786 Gyr against the published 13.787, which is the check. Epochs
before nucleosynthesis are the conventional decades of Kolb & Turner, *The
Early Universe* (1990), and carry a 100 % relative uncertainty, which is this
crate's way of writing "a decade, not a measurement". Other entries: Cyburt et
al., Rev. Mod. Phys. 88, 015004 (2016) for nucleosynthesis; Bromm & Larson,
ARA&A 42, 79 (2004) for Population III; Carniani et al., Nature 633, 318 (2024)
for JADES-GS-z14-0 at *z* = 14.32; Xiang & Rix, Nature 603, 599 (2022) for the
Milky Way's thick disc; Connelly et al., Science 338, 651 (2012) for the
CAI age of 4567.30 ± 0.16 Ma.

**Future chronology** — Schröder & Connon Smith, *Distant future of the Sun and
Earth revisited*, MNRAS 386, 155 (2008), Table 1, for the Sun (main sequence
ends at a model age of 10.00 Gyr, tip of the red giant branch 7.59 ± 0.05 Gyr
from now, white dwarf at 12.30 Gyr). Adams & Laughlin, *A dying universe*, Rev.
Mod. Phys. 69, 337 (1997), §VI, for the cosmological decades: Stelliferous
6 < η < 14, Degenerate 15 < η < 37, Black Hole 38 < η < 100, Dark η > 100 —
including the unassigned gaps, which are theirs. Super-Kamiokande
Collaboration, Phys. Rev. D 102, 112011 (2020), for the proton lifetime limit
of 2.4×10³⁴ years at 90 % confidence. Hawking (1974) and Page, Phys. Rev. D 13,
198 (1976), for the evaporation formula.

**Geological time scale** — International Chronostratigraphic Chart
**v2026/06**, <http://www.stratigraphy.org/ICSchart/ChronostratChart2026-06.pdf>,
retrieved 2026-09-26 (`ics-chart-2026-06`), cited as Cohen, Harper, Gibbard &
Car (2025, updated), *The ICS international chronostratigraphic chart this
decade*, Episodes 48, 105–115 (`cohen2025`). Every boundary age and every
published uncertainty is transcribed from that chart, whose own ages come
largely from Gradstein et al., *A Geologic Time Scale 2020* (not read here).
The version is part of the public API (`geologic::CHART_VERSION`) because
boundaries move, and each edition carried has its own name:
`geologic::ICS_CHART_2026_06` (`ics-chart-2026-06`) and
`geologic::ICS_CHART_2024_12` (`ics-chart-2024-12`), the chart v2024/12,
<https://stratigraphy.org/ICSchart/ChronostratChart2024-12.pdf>, retrieved
2026-09-26. The two printed charts differ in three numbers and no others: the
Anisian base at 246.7 against 247.0 Ma, the Olenekian base at 249.9 against
250.8 Ma, and the Wuchiapingian base at 259.51 ± 0.21 against 259.857 ± 0.084
Ma.

**Interval names in other languages** — the ICS's chart vocabulary,
`chart.ttl` at <https://github.com/i-c-stratigraphy/chart> (commit
`81618a8`, 2026-07-27, CC BY 4.0), for Czech, German, Spanish, French,
Indonesian, Italian, Japanese, Korean, Dutch, Polish, Portuguese, Russian,
Turkish and simplified Chinese; the Japanese checked against the Geological
Society of Japan's 国際年代層序表 v2024/12 and the Chinese against the ICS's
国际年代地层表 v2023/09. The cosmic, future and archaeological names are not
translated, because no published translation of them was read.

**Archaeology** — Stuiver & Polach, *Radiocarbon* 19, 355 (1977), for the
conventions behind a reported radiocarbon age; Reimer et al., *Radiocarbon* 62,
725 (2020), for IntCal20, which this crate names and does not implement; Walker
et al., *J. Quaternary Sci.* 24, 3 (2009), for the Holocene GSSP at 11 700 b2k;
Harmand et al., Nature 521, 310 (2015), for the Lomekwi 3 tools at 3.3 Ma. The
rest of the period boundaries are handbook conventions for Southwest Asia and
Europe, and their error bars are the spread between handbooks rather than any
measurement.

## Accuracy claimed

- Constants are transcribed exactly as CODATA 2022 prints them, to seven
  digits, and `planck_time_from_definition()` reproduces the tabulated Planck
  time from `ħ`, `G` and `c` to within the quoted uncertainty.
- Integrated cosmic ages are good to about 1 % for `t ≲ 10⁶` yr and 1.5 %
  later, dominated by `H0` and `Ωm`; the integration itself agrees with
  Planck's published age to 0.01 %.
- Geological boundaries carry the chart's own uncertainties and nothing
  narrower. Where the chart prints none the crate stores zero, and the module
  documentation explains that a zero means either a GSSA defined by decree or a
  GSSP whose numerical age is untabulated — a distinction the chart does not
  carry in machine-readable form, and neither does this crate.
- Error propagation is `hc_uncertainty`'s first-order delta method, with its
  independence assumption. Two ages both derived from the age of the universe
  are *not* independent, and `timeline::span_between` says so where it
  overstates the error.

## What it deliberately does not do

- **No cosmology solver.** The Friedmann integration that produced the early
  ages was run once, offline, and its inputs and result are documented in
  `universe`. The crate carries values; it does not recompute them.
- **No radiocarbon calibration.** IntCal20, SHCal20 and Marine20 are large
  datasets with their own release cadence, and a calibrated date is usually a
  multi-modal distribution rather than a Gaussian. `archaeology` models the
  distinction and refuses the conversion.
- **No exact Planck-scale duration.** `DeepTime::to_duration` truncates below
  one attosecond and drops the error bar entirely; the doc comment says so at
  the call site.
- **No calendars.** Everything is a span in SI seconds or a count of years
  before a stated datum. Turning a calendar year into a fixed day is
  `hc-calendar`'s job.
- **No regional archaeology beyond one sequence.** The period table is
  Southwest Asian and European and every entry names its region, because the
  Bronze Age begins eleven centuries apart in Anatolia and in Britain and never
  at all in most of the Americas.
