# hc-astro

The astronomical engine under `hyper-calendar`. It answers the four questions
a calendar actually asks of the sky:

* Where is the Sun on the ecliptic, and **when does it next reach a given
  longitude**? That search is what the 24 solar terms, the equinoxes and the
  solstices are made of.
* Where is the Moon, and **when is the next conjunction**? That is what the
  Chinese, Dangi, Vietnamese and observational Hijri calendars are made of.
* How bright is the Moon?
* When does the Sun or Moon cross the horizon at a given place?

It contains no calendar. Nothing here knows what a month is.

## Time scales

Every public function takes and returns a `hc_calendar::fixed::Moment` in
**Universal Time**. The series themselves are stated in Terrestrial Time; the
conversion happens inside, through `time::delta_t`. Published worked examples
are almost always quoted in TT, so `time::universal_time` bridges them.

`ut1::Ut1` is the UT1 time scale for `hc_core::Instant`, read as `TT − ΔT`
from the ΔT below: within 0.1 s of the IERS EOP 20 C04 series from 1974
through 2026-04-01, 0.06 s late on 2026-07-01 where the predictions
answer, and behind it by 8.9 s where the polynomial takes over in October
2033.
`ut1::Ut1Offsets` reads UT1 as `UTC + DUT1` from a series the caller
supplies instead.

### ΔT

ΔT = TT − UT1 comes from three sources, and `time::delta_t_regime` says
which answered:

* **Observed, 1974-01-01 to 2026-04-01.** The USNO's `deltat.data`
  (<https://maia.usno.navy.mil/ser7/deltat.data>, retrieved 2026-09-25),
  one sample a year at 1 January plus the last month observed, in
  `delta_t_table`, interpolated linearly between samples. The table's ends
  are `time::TABULATED_DELTA_T_FIRST` and `TABULATED_DELTA_T_LAST`. At the
  samples the value is the observation itself; between them a straight
  line misses the USNO's monthly values by at most 0.09 s. It is not
  extrapolated.
* **Predicted, from there to 2033-10-01.** The USNO's `deltat.preds`
  (<https://maia.usno.navy.mil/ser7/deltat.preds>, retrieved 2026-09-25),
  every quarterly row with the error the file states for it, in the same
  module, interpolated the same way; `time::delta_t_predicted` gives the
  value with its error, and `PREDICTED_DELTA_T_FIRST` and
  `PREDICTED_DELTA_T_LAST` are the file's ends. The file's rows begin in
  July 2022 and overlap the observations, and there the observation wins.
  The overlap measures the predictions: they ran below the observations by
  up to 0.12 s, and from late 2023 to early 2025 by up to 2.7 times the
  error the file stated, so the error column is the USNO's estimate and
  not a bound. The predictions are not extrapolated either.
* **Fitted, outside both.** A named model of the historical record, from
  the table `delta_t_model::DELTA_T_MODELS`. `time::delta_t` uses
  `espenak-meeus-2006`, the Espenak–Meeus NASA polynomial set: thirteen
  segments over −500…+2150, and the parabola ΔT = −20 + 32u²,
  u = (year − 1820)/100, outside that, fifteen expressions in all. The
  segments are independent least-squares fits and meet at the joins to
  within a couple of tenths of a second, which this crate does not smooth.
  `time::delta_t_polynomial` is this fit alone, for any year. It is the
  default because it is the one model carried that answers for every year.
  `time::delta_t_with` takes a model by name instead, and the other one
  carried is `morrison-stephenson-2021`, the cubic spline of Morrison,
  Stephenson, Hohenkerk and Zawilski's Table S15 (v. 2020) over
  −720…2019, which answers `None` outside that span. The two differ by
  264 s at −500; over 1974–2019 the spline stays within 0.24 s of the
  USNO's observations.

The observations meet the polynomial to 0.1 s at their start, where the
polynomial was fitted to the same observations, and the predictions to
0.04 s at their end, inside the 0.22 s the USNO states there. At the
predictions' end the join is not close: the polynomial's 2005–2050 segment
is a forecast made in 2006, and the Earth has since rotated faster than it
forecast, so on 2033-10-01 the polynomial is 8.9 s above the last
prediction, and the step is left visible rather than blended or offset.
Concretely, for 2024 the polynomial gives 73.9 s where 69.2 s was observed,
which put every solar term of that year 4.7 s early in Universal Time; the
72 terms of 2024–2026 that the 暦要項 publishes to the minute sit at a mean
of +1.1 s for the 54 inside the observations and −0.1 s for the 18 on the
predictions, where the polynomial alone would put them at −4.3 s and
−6.4 s (the measurement is in
[`docs/systems/solar-terms-and-pentads.md`](../../docs/systems/solar-terms-and-pentads.md)).
A caller computing past the predictions' end gets an instant about nine
seconds early, growing at the forecast's slope of about 0.7 s a year, until
the tables are extended; extending them is a data change, not a code
change. In the atomic era ΔT = 32.184 s + (TAI − UTC) − DUT1, and the
tests check the observed table against `hc-core`'s leap-second table and
the IERS series through that identity rather than carrying a second copy
of the same measurement.

## Accuracy claimed, and over what era

| Quantity | Claimed | Verified against |
| --- | --- | --- |
| Apparent solar longitude | **~1″** | Meeus example 25.b; the VSOP87 truncation measured against the full series at 0.23″ |
| Equinox / solstice times | **under a minute** | USNO and IMCCE published tables, nine events 2000–2024, which give the minute |
| Apparent lunar longitude | **~10″** | the Σl, Σb and Σr sums of Meeus example 47.a |
| Lunar distance | **~0.2 km** on 368 000 | Meeus example 47.a |
| New moon / quarter times | **~2 s of the published series** | Meeus examples 49.a and 49.b |
| Illuminated fraction | **~0.001** | Meeus example 48.a |
| Sunrise / sunset | **under a minute** | NAOJ 暦計算室, 「日の出入り＠東京(東京都)」, 2024-01-01, at its point 35.6581° N, 139.7414° E, published to the minute |
| Mean obliquity | **0.01″** near J2000, arcseconds over ±10 000 years | Meeus example 22.a |
| Nutation | **0.5″** in Δψ, **0.1″** in Δε | Meeus example 22.a |
| Earth Rotation Angle, IAU 2006 GMST | **10⁻⁹ degree** of the published expressions | ERFA's `eraEra00` and `eraGmst06` test values |
| IAU 1982 GMST (Meeus 12.4) | **0.7 µs** of time of ERFA's `eraGmst82` | ERFA's test value at 2006-01-01 |
| UT2, UT1R, UT1S corrections to the caller's UT1 | exact as conventions; the tidal sum to **10⁻¹² s** of the IERS model | the USNO formula by hand; the test case of the IERS routine `RG_ZONT2.F` |
| Local apparent (sundial) time | the equation of time's, **under a second** | Meeus example 28.a, within 0.5 s |
| Temporal hours | the sunrise and sunset's, **under a minute** over twelve | NAOJ 暦計算室, Tokyo, 2024-01-01 |

The era over which all of this holds is roughly **1000 BCE to 3000 CE**.
Inside it the limiting factor is the series; outside it the limiting factor is
ΔT, whose own uncertainty reaches hours before 500 BCE — so a lunisolar date
computed for the Warring States period is a plausible reconstruction, not a
fact. `time::delta_t_regime` reports whether a year is answered from the
observed table, from the USNO's predictions, from the fitted polynomials,
or from the extrapolation beyond them, and `time::is_fitted_year` whether it is inside the span the
polynomials were fitted to at all.

### The Sun is VSOP87, cut where it was measured

`solar_longitude` is the Earth's heliocentric position from VSOP87 turned
around — Meeus's chapter 25 in its higher-accuracy form — with the series
truncated at an amplitude of 10⁻⁷ and the cost of the cut measured against
the full theory: 0.23″ in longitude over 1000–3000, which is under six
seconds of the Sun's motion. What is left in an equinox time is ΔT and the
half-minute the published tables round to. The chapter's low-accuracy
series, good to 0.01° and a quarter of an hour, is not enough for the
equinox calendars, which ask which side of a sunset an equinox fell on.

For a solar term that falls within a minute of local midnight, the *day*
this crate assigns it is still decided by ΔT and the truncation rather than
by the sky, and the calendars built on top of this say so where it matters.

## What this crate deliberately does not do

* **No ephemeris.** No planets, no eclipse circumstances, no VSOP87 beyond
  the Earth's own series, no ELP-2000 beyond Meeus's sixty-term truncation.
* **No atmosphere.** Refraction at the horizon is the conventional 34′, full
  stop. Real refraction depends on temperature and pressure and can move an
  observed sunrise by more than a minute — a larger error than any of the
  astronomy above. Height above sea level is modelled only through the dip of
  the horizon; terrain is not modelled at all.
* **No topocentric positions.** Rise and set use geocentric coordinates, with
  the Moon's parallax folded into the horizon altitude (Meeus's
  `h₀ = 0.7275π − 34′`) rather than applied to the position.
* **No leap seconds.** A day here is exactly 86 400 seconds; ΔT carries the
  whole of the Earth's rotational irregularity. Leap seconds live in
  `hc-core`.
* **No invented answers at the poles.** `sunrise`, `sunset`, `dawn`, `dusk`,
  `moonrise` and `moonset` return `Option`, and the `None` is the point: a
  polar day and a polar night are real, and so is a British midsummer with no
  astronomical darkness.

## Known gaps

* `moonrise` and `moonset` scan the local day at twenty-minute steps before
  refining. Above about 70° latitude the Moon's altitude can cross the horizon
  more than twice in a day; only the first crossing of each kind is returned.
* The "local day" for rise and set runs from local *mean* solar midnight,
  derived from longitude alone. This crate knows nothing about time zones;
  that is `hc-tz`'s job.
* `solar_longitude_after` and `moon_phase_at_or_after` are "at or after" up to
  floating-point noise. If the argument is the answer to within a rounding
  error, the search may step a whole cycle forward. Search from slightly
  earlier if you need "on or before".
* Obliquity uses Laskar's expansion, which diverges outside ±10 000 years from
  J2000 and must not be believed there.

## Sources

* Jean Meeus, *Astronomical Algorithms*, 2nd ed., Willmann-Bell 1998 —
  chapters 12, 13, 15, 22, 25, 28, 47, 48 and 49.
* Edward M. Reingold and Nachum Dershowitz, *Calendrical Calculations*, 4th
  ed., Cambridge 2018 — the Rata Die pivot, the wrap-aware angular inversion,
  and the mean tropical year and synodic month used to seed the searches.
* P. Bretagnon and G. Francou, VSOP87, *A&A* 202, 309 (1988) (not read
  here), as the file `VSOP87D.ear` of CDS VizieR catalogue VI/81,
  <https://cdsarc.cds.unistra.fr/ftp/VI/81/VSOP87D.ear>, retrieved
  2026-09-26 (`bretagnon1988`).
* Fred Espenak and Jean Meeus, "Polynomial Expressions for Delta T",
  <https://eclipse.gsfc.nasa.gov/SEhelp/deltatpoly2004.html>, retrieved
  2026-09-26, adapted from *Five Millennium Canon of Solar Eclipses*,
  NASA/TP-2006-214141 (not read here) (`espenak-meeus-2006`). The page
  states the 2005–2050 segment as derived from *estimated* values for 2010
  and 2050: a forecast.
* L. V. Morrison, F. R. Stephenson, C. Y. Hohenkerk and M. Zawilski,
  "Addendum 2020 to 'Measurement of the Earth's rotation: 720 BC to AD
  2015'", *Proc. R. Soc. A* 477 (2021) 20200776, Table S15 v. 2020
  (`morrison2021`), read as the file `Table-S15.2020.txt`; the analysis it
  revises is Stephenson, Morrison and Hohenkerk, *Proc. R. Soc. A* 472
  (2016) 20160404 (`stephenson2016`, not read here).
* United States Naval Observatory, *Delta T: deltat.data*,
  <https://maia.usno.navy.mil/ser7/deltat.data>, retrieved 2026-09-25 — the
  observed ΔT of 1974–2026; checked against the IERS EOP 20 C04 series,
  <https://hpiers.obspm.fr/iers/eop/eopc04/eopc04.1962-now>, retrieved the
  same day.
* United States Naval Observatory, *Delta T: deltat.preds*,
  <https://maia.usno.navy.mil/ser7/deltat.preds>, retrieved 2026-09-25 —
  the predicted ΔT of 2022–2033 with its stated error.
* Reference event times: the USNO "Earth's Seasons" table, the IMCCE, and
  NAOJ 暦計算室, 「日の出入り＠東京(東京都) 令和6年(2024)01月」,
  <https://eco.mtk.nao.ac.jp/koyomi/dni/2024/s1301.html>, retrieved
  2026-09-26 (`nao-koyomi-dni-tokyo-2024`), for the Tokyo rise/set anchors.

Every reference value in the test suite is taken from one of these. None of
them was produced by this crate.
