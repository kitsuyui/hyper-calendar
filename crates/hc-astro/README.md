# hc-astro

The astronomical engine under `hyper-calendar`. It answers the four questions
a calendar actually asks of the sky:

* Where is the Sun on the ecliptic, and **when does it next reach a given
  longitude**? That search is what the 24 solar terms, the equinoxes and the
  solstices are made of.
* Where is the Moon, and **when is the next conjunction**? That is what the
  Chinese, Dangi, Hebrew and observational Hijri calendars are made of.
* How bright is the Moon?
* When does the Sun or Moon cross the horizon at a given place?

It contains no calendar. Nothing here knows what a month is.

## Time scales

Every public function takes and returns a `hc_calendar::fixed::Moment` in
**Universal Time**. The series themselves are stated in Terrestrial Time; the
conversion happens inside, through `time::delta_t`. Published worked examples
are almost always quoted in TT, so `time::universal_time` bridges them.

ΔT is the Espenak–Meeus NASA polynomial set: fifteen segments over
−500…+2150, and the parabola ΔT = −20 + 32u², u = (year − 1820)/100, outside
that. The segments are independent least-squares fits and meet at the joins
to within a couple of tenths of a second, which this crate does not smooth.

## Accuracy claimed, and over what era

| Quantity | Claimed | Verified against |
| --- | --- | --- |
| Apparent solar longitude | **~1″** | Meeus example 25.b; the VSOP87 truncation measured against the full series at 0.23″ |
| Equinox / solstice times | **under a minute** | USNO and IMCCE published tables, nine events 2000–2024, which give the minute |
| Apparent lunar longitude | **~10″** | the Σl, Σb and Σr sums of Meeus example 47.a |
| Lunar distance | **~0.2 km** on 368 000 | Meeus example 47.a |
| New moon / quarter times | **~2 s of the published series** | Meeus examples 49.a and 49.b |
| Illuminated fraction | **~0.001** | Meeus example 48.a |
| Sunrise / sunset | **~2 minutes** | the Japanese national ephemeris for Tokyo, 2024-01-01 |
| Mean obliquity | **0.01″** near J2000, arcseconds over ±10 000 years | Meeus example 22.a |
| Nutation | **0.5″** in Δψ, **0.1″** in Δε | Meeus example 22.a |

The era over which all of this holds is roughly **1000 BCE to 3000 CE**.
Inside it the limiting factor is the series; outside it the limiting factor is
ΔT, whose own uncertainty reaches hours before 500 BCE — so a lunisolar date
computed for the Warring States period is a plausible reconstruction, not a
fact. `time::is_fitted_year` reports whether a year is inside the span ΔT was
fitted to at all.

### The Sun is VSOP87, cut where it was measured

`solar_longitude` is the Earth's heliocentric position from VSOP87 turned
around — Meeus's chapter 25 in its higher-accuracy form — with the series
truncated at an amplitude of 10⁻⁷ and the cost of the cut measured against
the full theory: 0.23″ in longitude over 1000–3000, which is under six
seconds of the Sun's motion. What is left in an equinox time is ΔT and the
half-minute the published tables round to. It used to be the chapter's
low-accuracy series, good to 0.01° and a quarter of an hour, until the
equinox calendars asked which side of a sunset an equinox fell on.

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
* Fred Espenak and Jean Meeus, "Polynomial Expressions for Delta T", derived
  from *Five Millennium Canon of Solar Eclipses*, NASA/TP-2006-214141.
* Reference event times: the USNO "Earth's Seasons" table, the IMCCE, and the
  National Astronomical Observatory of Japan's ephemeris for the Tokyo
  rise/set anchors.

Every reference value in the test suite is taken from one of these. None of
them was produced by this crate.
