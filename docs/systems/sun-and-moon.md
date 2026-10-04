# The Sun and the Moon: apparent positions, the moments found by inverting a longitude, and the equation of time

Backs `hc-astro::solar` (the apparent longitude and position of the Sun,
`solar_longitude_after`, `equinox`, `solstice`, `seasonal_event`,
`equation_of_time`), `hc-astro::vsop87` (the Earth's heliocentric position),
`hc-astro::lunar` (the Moon's apparent position, `nth_moon_phase`,
`nth_new_moon`, `new_moon_at_or_after`, `new_moon_before`,
`moon_phase_at_or_after`, `moon_age`), the nutation, obliquity and
ecliptic-to-equatorial functions of `hc-astro::earth`, and the root-finders
of `hc-astro::search`; and the exports `hc_sky_at`, `hc_solar_terms_between`
and `hc_moon_phases_between`. No calendar identifier is registered: this is
the sky that the lunisolar calendars, the solar terms, the equinox
calendars, the Hindu calendars' true sky and the astronomical Easter are
read from. It takes its time scales as inputs, the ΔT of `hc-astro::time`
with its two tables and the `Moment` of `hc-calendar`:
[time-scales.md](../time-scales.md) places them. The neighbours are not
repeated here: [earth-rotation.md](earth-rotation.md) for the sidereal time
and the Earth Rotation Angle that turn a position into an hour angle,
[rise-and-set.md](rise-and-set.md) for what is done with the positions at a
place, [jupiter-ephemeris.md](jupiter-ephemeris.md) for Jupiter, which takes
this Earth, this FK5 step and this nutation, and
[orbital-elements.md](orbital-elements.md) for the Earth's orbit over
millions of years.

## What it is

An almanac prints where the Sun and the Moon are, and what it prints is not
a bare geometry. Three choices make a position *apparent*, and each moves
it by an amount that a calendar can feel:

- **The equinox.** Longitude is measured from the point where the Sun
  crosses the equator northward, and that point moves. Precession carries
  it westward along the ecliptic by about 50″ a year (5 028.796″ a century
  in the IAU 2006 polynomial that `earth` carries, [capitaine2003], not
  read); nutation, mostly the precession of the Moon's node over 18.6
  years, makes it swing by up to about 17″ either side of that steady
  drift (the leading term of the series below is 17.20″). The *mean* equinox of a date
  has the precession only; the *true* equinox has the nutation too. The
  longitude of an almanac is referred to the true equinox of date.
- **The light.** The Sun is seen where it was about 8.3 minutes ago while
  the Earth has moved on, and the net shift is the *aberration*, 20.5″
  backwards along the ecliptic: the Sun's own motion in those 8.3 minutes.
  A *geometric* position leaves it out; an *apparent* one puts it in. The
  size is Meeus's constant of aberration divided by the distance in AU
  (`solar::ABERRATION_ARCSECONDS`); the 8.3 minutes and the match with the
  Sun's motion are arithmetic on the Sun's 0.986° a day, not from a source.
- **The frame.** A planetary theory is fitted to a numerical integration
  and carries the integration's own ecliptic and equinox, the *dynamical*
  one. Almanacs refer to the FK5 star catalogue's frame, which differs by a
  rotation of about 0.09″.

The calendars read these positions as two kinds of event:

- **A solar term.** The 24 terms are the instants at which the Sun's
  apparent longitude passes the multiples of 15°, 春分 at 0° and 秋分 at
  180°; the Observatory defines each by the 視黄経 [nao-faq-24sekki]. The
  equinoxes and solstices are the terms at 0°, 90°, 180° and 270°.
- **A phase of the Moon.** New moon, first quarter, full moon and last
  quarter are the instants at which the Moon's apparent ecliptic longitude
  exceeds the Sun's by 0°, 90°, 180° and 270° [usno-moon-phases-faq,
  nao-faq-moon]. The difference is taken from the centre of the Earth, so
  that every observer has the same instant, and the Moon need not be
  visible from where the calendar is kept [nao-rekiwiki-moon-phases]. The
  Moon's orbit is inclined about 5.1° to the ecliptic, so the elongation
  in longitude is not the Moon's angular distance from the Sun on the sky.

Apparent solar time is what a sundial reads. The **equation of time** is
apparent solar time less mean solar time, mean solar time being that of a
fictitious Sun moving uniformly along the equator; the USNO defines each as
12 h plus the local hour angle of its Sun, and the difference as the sum of
two effects, the Sun's motion along a tilted ecliptic and along an
eccentric orbit, reaching about 16 minutes [usno-eqtime].

## How it works

**The time.** Every function takes a Universal Time `Moment`; the series
are stated in Terrestrial Time [vsop87-cds-notice], so each evaluation adds
the library's ΔT first (`time::delta_t`, `time::julian_centuries`). A ΔT
that is wrong by δ seconds moves every longitude event found here by δ
seconds in Universal Time.

**The Sun's geometric position: VSOP87D.** VSOP87 is Bretagnon and Francou's
planetary theory, fitted to the JPL integration DE200 [vsop87-cds-readme,
bretagnon1988]. Version D gives a planet's heliocentric longitude *L*,
latitude *B* and distance *R* in the mean equinox and ecliptic of the date
[vsop87-cds-notice]. Each coordinate is a sum over powers *T*ᵅ of series of
terms *A* cos(*B* + *C* *T*), *T* in Julian millennia of TT from J2000.0, α
from 0 to 5. The Earth's file holds 2 425 terms, which with 17 series
headers are the 2 442 records the ReadMe lists [vsop87-cds-readme]; the
module's comments count 1 080 for *L*, 348 for *B* and 997 for *R*.
`hc-astro::vsop87` keeps the 213 terms whose amplitude is at least 10⁻⁷, 130
for *L*, 13 for *B* and 70 for *R*, and the module documentation states that
the cut costs 0.23″ in longitude, 0.13″ in latitude and 10⁻⁶ AU against the
full series, measured by the module's author against the CDS file from 1000
to 3000. The Sun is the Earth turned around: longitude *L* + 180°, latitude
−*B*, the same distance (`solar::geometric_solar_ecliptic_at_centuries`).

**The apparent longitude.** `solar::solar_longitude_at_centuries` makes
four corrections to that geometric longitude, each from Meeus's chapters
25, 32 and 22 as the module cites them [meeus1998]:

1. The frame: FK5's longitude is the dynamical one less 0.090 33″, and its
   latitude differs by a periodic 0.039 16″ at most (Meeus 32.3). The
   VSOP87 notice gives the rotation to FK5 for its J2000 versions, whose
   element for the offset of the equinox is 4.4036 × 10⁻⁷ rad, 0.0908″
   [vsop87-cds-notice]: the size of the module's constant to 0.0005″.
2. The equinox: add the nutation in longitude Δψ, which turns the mean
   equinox of the date into the true one. `earth::nutation_at_centuries`
   carries Meeus's four terms, Δψ = −17.20″ sin Ω − 1.32″ sin 2*L* −
   0.23″ sin 2*L*′ + 0.21″ sin 2Ω, with Ω the longitude of the Moon's
   ascending node and *L*, *L*′ the mean longitudes of the Sun and the
   Moon. It stands at −14.03″ on 2000 January 1.5 and +16.03″ on 1992
   October 13.
3. The light: add −20.4898″ / *R*, the aberration (`ABERRATION_ARCSECONDS`
   in `solar`, Meeus 25.10).
4. Reduce to [0°, 360°).

No precession is applied to the Sun: VSOP87D is already of the date.
`earth::general_precession_arcseconds` (the IAU 2006 general precession
in longitude, 5 028.796 195″ a century, from [capitaine2003], not read) is
carried for the sidereal zodiac and the heliocentric Julian date and not
used here. The right ascension and declination, `solar::solar_position`,
come from the apparent longitude, the latitude and the *true* obliquity of
the date, ε = ε₀ + Δε, with the mean obliquity ε₀ from Laskar's
polynomial as Meeus (22.3) prints it [laskar1986], by tan α = (sin λ cos ε
− tan β sin ε) / cos λ and sin δ = sin β cos ε + cos β sin ε sin λ; at a
latitude β of zero these are the USNO's tan α = cos ε sin λ / cos λ and
sin δ = sin ε sin λ [usno-sun-approx].

**The Moon.** `lunar::lunar_longitude_at_centuries` evaluates the abridged
ELP-2000/82 series that Meeus prints as his tables 47.A and 47.B
[chapront1983, meeus1998]: from the five arguments of the lunar theory, the
mean longitude *L*′ and the elongation *D*, the Sun's mean anomaly *M*, the
Moon's mean anomaly *M*′ and the argument of latitude *F*: the sixty rows of
table 47.A each give a sine term in longitude and a cosine term in distance,
and the sixty of table 47.B a sine term in latitude. Each term with *M* is
scaled by *E* or *E*², the eccentricity factor, and the additive terms for
Venus, Jupiter and the Earth's flattening are added. The geometric longitude
is *L*′ + Σ*l* / 10⁶ degrees, the distance 385 000.56 km + Σ*r* / 10³ and
the latitude Σ*b* / 10⁶ degrees. The apparent longitude adds the same Δψ.
The module says that the annual aberration of the Moon's light is negligible
against the series' 10″ and adds nutation only; it does not mention the
light-time (1.3 s of the Moon's 0.55″ a second, about 0.7″, by arithmetic),
and neither was applied or isolated here. The phase angle,
`lunar::lunar_phase`, is the Moon's apparent longitude less the Sun's.

**Finding the moment a longitude is reached.** The Sun's longitude rises
without ever turning back, about 0.95° to 1.02° a day, and wraps from 360°
to 0°. `search::invert_angular` finds the moment at which such a function
reaches a target by the predicate of Reingold and Dershowitz,
*Calendrical Calculations* §14.4 as the module cites it [reingold2018]: at
a moment *t* the angle has *reached* the target when (*f*(*t*) − target)
mod 360° is below 180°. It halves a bracket that is not reached at its
start and reached at its end, until it is narrower than 10⁻⁷ day (8.6 ms),
at most 64 times. `solar::solar_longitude_after(target, moment)` makes
the bracket:

1. The longitude now, and the degrees still to go to the target.
2. A seed: the moment plus those degrees at the mean tropical year's
   rate, 365.242 189 / 360 days a degree.
3. A bracket of 5 days either side of the seed (not before `moment`).
4. The bisection.

`solar::seasonal_event(year, degrees)` is that search from 1 January of the
Gregorian year, and `equinox`, `solstice` and the 24 terms of
`hc-seasons` are the same search at other targets. The seed is wrong by
the difference of two equations of the centre, up to 3.9 days (measured
below), so the bracket always holds the answer.

**Two ways to a new moon.** The library finds the Moon's phases by two
routes, which are not the same code:

- *A closed form.* Meeus's chapter 49 gives the *time* of a phase as a
  series, not the Moon's position: the lunation number *k* (0 is the new
  moon of 2000 January 6, and a phase adds ¼, ½ or ¾), *T* = *k* / 1236.85,
  the mean phase JDE = 2 451 550.097 66 + 29.530 588 861 *k* + 0.000 154 37 *T*²
  − 0.000 000 150 *T*³ + 0.000 000 000 73 *T*⁴, and a sum of 24 periodic
  corrections in *M*, *M*′ and *F* (different for new moon, full moon and
  the quarters), a node term −0.000 17 sin Ω, fourteen planetary terms and,
  for the quarters, a term *W* with a sign for each quarter. The result is in TT,
  and `nth_moon_phase` converts it with ΔT. `nth_new_moon`,
  `new_moon_at_or_after`, `new_moon_before`, `lunation_containing` and
  `moon_age` use it, and so does `hc_moon_phases_between` and the new
  moons in `hc_sky_at`; it is what `hc-calendars-lunar`'s lunisolar
  calendars call. No longitude is inverted.
- *An inversion.* `lunar::moon_phase_at_or_after(degrees, moment)` finds
  the moment `lunar_phase` reaches any elongation, by the same bisection
  as the Sun: seed at 29.530 588 861 / 360 days a degree, ±2 days. It
  answers for any angle, a quarter, a full moon or a 14° elongation, and
  is what the astronomical Easter, the Hindu lunar calendar's conjunction
  and the days of the principal phases in `hc-seasons`' moon-phase layer
  call.

The closed form is fitted to the time of the event and the inversion
evaluates the position series and the VSOP87 Sun; the module documentation
says neither is derived from the other. How far they part is measured
below. `new_moon_at_or_after` starts two lunations before the mean-rate
estimate and steps forward by whole lunations until the closed form's
moment is not before the argument.

**The equation of time.** `solar::equation_of_time(moment)` returns, in
days, apparent solar time less mean solar time at Greenwich, from the
USNO's definitions [usno-eqtime]: the apparent Sun's hour angle *H* at
Greenwich is the apparent sidereal time of the UT1 moment, the IAU 1982
mean sidereal time plus Δψ cos ε (`earth::apparent_sidereal_time_iau1982`,
see [earth-rotation.md](earth-rotation.md)), less the Sun's right
ascension, which is taken at the TT of the moment; apparent solar time is
12 h + *H*, mean solar time is the UT1 moment's own day fraction, and the
difference is folded into (−½, ½] day. The module documentation states
how this differs from Meeus's (28.1), which evaluates the mean Sun at TT
and so comes out larger by two parts: 0.19 s in 2024, 2.8 s in 1000 CE and
38 s in 586 BCE together. Local apparent time, noon and the rise and set
functions all use this form.

**Worked example.** Four cases that the reader can follow by hand; the
tests named under Code hold the figures.

*1. The Sun's apparent longitude on 1992 October 13.0 TD* (Meeus's example
25.b, JDE 2 448 908.5, ΔT 58.9 s). *T* = −0.072 183 436 Julian centuries,
so τ = −0.007 218 343 6 millennia. The 213 terms give the Earth *L* =
19.907 318°, *B* = −0.716″, *R* = 0.997 608 73 AU; the Sun is at
199.907 318° and latitude +0.7″. FK5: −0.090 33″, 199.907 293°. Nutation: with
Ω = 264.657° the four terms are +17.125″, −0.911″, −0.225″ and +0.039″,
Δψ = +16.03″ = +0.004 453°. Aberration: −20.4898″ / 0.997 608 73 =
−20.539″ = −0.005 705°. Apparent: 199.907 293° + 0.004 453° − 0.005 705° =
199.906 041° = 199°54′21.75″. Meeus's printed value, as the test quotes it,
is 199°54′21.56″ from his own truncation of VSOP87, whose *L* is 19.907 372°
and *R* 0.997 608 53 AU: the two truncations stand 0.19″ apart. With the
true obliquity 23.440 137° the right ascension is 13 h 13 m 30.76 s and the
declination −7°47′01.83″, against his 13 h 13 m 30.749 s and −7°47′01.74″.

*2. The March equinox of 2024.* Start at 2024-01-01 00:00 UT, RD 738 886,
where λ = 280.039 00°. To go to 0°: 79.961°; at 1.014 562 days a degree,
81.1255 days, so the seed is RD 738 967.125 36 and the bracket RD
738 962.125 36 to 738 972.125 36. The first midpoint, 738 967.125 36, has
λ = 1.98°: reached, so it is the new end. The second, 738 964.625 36, has
λ = 359.499°: not reached, the new start. The third, 738 965.875 36
(0.741°), and the fourth, 738 965.250 36 (0.120°), are reached. After 27
halvings the bracket is narrower than 10⁻⁷ day: RD 738 965.129 49, 2024
March 20 at 03:06:28 UT, λ = 360.000 000°. ΔT is 69.17 s, so TT is
03:07:37. The USNO's table has 03:06 UT [usno-earth-seasons], and the 暦要項
12:06 JST, which is 03:06 UT [nao-rekiyoko-2024]: 28 s after the minute.

*3. The new moon of 1977 February.* Lunation *k* = −283, *T* = −0.228 807

05. The mean phase is JDE 2 443 192.941 02 and the periodic corrections sum

to −0.289 84 day, so JDE 2 443 192.651 18, 1977 February 18 03:37:42 TD, as
Meeus's example 49.a gives it (the test quotes the figures; the code is 0.23
s from it). ΔT is 47.66 s, so the moment is about 03:36:54.5 UT, RD 721
768.150 631. The inversion, started five days earlier, finds RD 721 768.150
683, 4.5 s later: the position series puts the Moon 2.3″ (0.000 64°) short
of conjunction at the closed form's moment. For a modern one, the new moon
of 2026 September 11 is 03:26:56 UT by the closed form and 03:26:51 by the
inversion, and the 暦要項 has 12:27 JST, 03:27 UT
[nao-rekiyoko-2026-sakugenbo].

*4. The equation of time on 2024 November 3, 0 h UT1.* ΔT is 69.144 s, so
*T* = 0.248 391 535. The Sun is at λ = 221.058 714°, β = −0.45″; the true
obliquity is ε = 23.438 581°, which gives α = 218.632 206° (and
δ = −15.145 275°). The UT1 moment's mean sidereal time is 42.746 599°, the
equation of the equinoxes Δψ cos ε = −2.862″ × 0.917 = −0.175 s of time,
so the apparent sidereal time is 42.745 869°. *H* = 42.745 869° −
218.632 206° = −175.886 337°, apparent solar time 12 h − 11.725 756 h =
0 h 16 m 27.28 s, mean solar time 0 h: *E* = +987.2 s = +16 min 27.2 s. JPL
Horizons' hour angle for the same instant is −11.725 752 512 h, 0.012 s
from it [jpl-horizons].

## What is carried

In `hc-astro::solar`: `solar_longitude` and `solar_longitude_at_centuries`,
`geometric_solar_ecliptic_at_centuries`,
`solar_mean_longitude_at_centuries`, `solar_latitude_at_centuries`,
`solar_radius_vector` and its centuries form, `solar_position`,
`equation_of_time`, `solar_longitude_after`, `Equinox` and `equinox`,
`Solstice` and `solstice`, `seasonal_event` and `seasonal_event_day`,
`MEAN_TROPICAL_YEAR`. In `hc-astro::vsop87`: `earth_heliocentric`, the 213
terms. In `hc-astro::lunar`: `lunar_longitude`, `lunar_latitude`,
`lunar_distance`, `lunar_parallax`, `lunar_position`, their centuries forms
and `geometric_lunar_longitude_at_centuries`, `lunar_phase`,
`lunar_illuminated_fraction`, `MoonPhase`, `nth_moon_phase`, `nth_new_moon`,
`lunation_containing`, `moon_age`, `new_moon_before`,
`new_moon_at_or_after`, `moon_phase_at_or_after`, `MEAN_SYNODIC_MONTH`. In
`hc-astro::earth`: `nutation`, `nutation_at_centuries`, `mean_obliquity`,
`true_obliquity`, `obliquity`, `equatorial_from_ecliptic`,
`general_precession_arcseconds`. In `hc-astro::search`: `invert_angular` and
the sign-change and angle-crossing searches `bisect_rising`,
`bisect_falling`, `bisect_until` and `next_angle_crossing`, which the
calendars with their own models of the sky use.

At the boundary: `hc_sky_at` writes the Sun's and the Moon's apparent
longitude, the distances, the Moon's latitude, elongation and illuminated
fraction, the neighbouring new moons and the ΔT it used;
`hc_solar_terms_between` and `hc_moon_phases_between` write the terms and
the phases of a span of up to 400 years, whole seconds rounded down. All
three answer for the years −1000 to 3000 and refuse outside; the functions
of `hc-astro` themselves compute outside it, with whatever ΔT the
extrapolation gives. The columns and ranges are in
[`crates/hyper-calendar-wasm/README.md`](../../crates/hyper-calendar-wasm/README.md).

**Not carried**, with the reason:

- *The full nutation series.* The 106-term IAU 1980 series, Meeus's
  Table 22.A of its 63 largest terms, and the IAU 2000A series are not
  carried (not yet done). The four-term series is good to 0.5″ in Δψ,
  12 s of the Sun's motion; the module's reason is that no calendar rule
  read for this library notices it.
- *A lunar theory longer than sixty terms*, the full ELP-2000/82 or
  its successors, and a numerical ephemeris (no series file read; the
  departures at 1001 BCE and in the year 1 are in the first table under
  Accuracy).
- *The Moon's light-time and aberration*: the module calls the aberration
  negligible against the series (not isolated here).
- *Eclipses*, solar or lunar: no model has been written.
- *The Sun's and Moon's topocentric positions.* The parallax enters the
  Moon's rising and setting through the horizon altitude only
  ([rise-and-set.md](rise-and-set.md)).
- *Meeus's chapter 27 equinox and solstice times and his chapter 28
  equation of time*, which the library does not use: the equinoxes are the
  inversion above, and the equation of time is the hour-angle form.
- *The planets other than Jupiter*: [jupiter-ephemeris.md](jupiter-ephemeris.md)
  records the series files not read.
- *The Sun's position outside the VSOP87 era.* The series is stated for
  about 4 000 years either side of J2000 for the Earth–Moon barycentre
  [vsop87-cds-notice]; the library does not refuse earlier or later
  moments, and past 3000 or before 1000 BCE no comparison was made.

## Accuracy

The README of `hc-astro` claims about 1″ for the Sun's apparent longitude
and about 10″ for the Moon's, an equinox or solstice under a minute and a
new moon within about 2 s of the published series, over roughly 1000 BCE to
3000 CE. These were measured on 2026-10-03 against JPL Horizons, the NAOJ
and the USNO, with probe programs in a scratch copy of the crates (the
data are the pages listed under Sources; the probes were not added to the
repository). Errors are *ours − the reference*; a later moment is positive.

**Against JPL Horizons (DE441), in Terrestrial Time.** Horizons' apparent
longitude and latitude are of the Earth's centre, with light-time,
deflection and stellar aberration [jpl-horizons-sun-moon]. Comparing in TT
leaves ΔT out. The Moon here is the apparent longitude with nutation only.

| Date (TT; Julian calendar before 1582) | Sun λ | Sun β | Moon λ | Moon β | Moon distance |
| --- | ---: | ---: | ---: | ---: | ---: |
| 12 Jan 1001 BCE | +1.05″ | +0.08″ | +129.10″ | −1.42″ | −5.4 km |
| 1 Jan 1 | +0.64″ | +0.22″ | +62.52″ | −4.88″ | +29.2 km |
| 27 Dec 999 | +0.05″ | −0.02″ | +9.54″ | −0.55″ | +18.5 km |
| 1 Jan 1500 | +0.06″ | +0.09″ | +2.53″ | −0.61″ | +3.9 km |
| 1 Jan 1900 | +0.06″ | +0.05″ | +1.94″ | +0.27″ | +6.9 km |
| 1 Jan 1950 | −0.05″ | +0.02″ | −2.85″ | −0.93″ | −25.3 km |
| 1 Jan 2000, 12:00 | −0.06″ | 0.00″ | −0.15″ | +1.46″ | +30.2 km |
| 1 Jan 2025 | +0.15″ | +0.03″ | +0.73″ | +0.65″ | −8.8 km |
| 1 Jan 2050 | +0.08″ | +0.05″ | +0.66″ | −0.35″ | −41.9 km |
| 1 Jan 2100 | +0.14″ | −0.10″ | +2.07″ | −2.02″ | +35.7 km |
| 1 Jan 2500 | +0.17″ | 0.00″ | −1.33″ | −2.20″ | +6.7 km |
| 1 Jan 3000 | +0.44″ | −0.12″ | −1.55″ | +0.73″ | −24.5 km |

The Sun is within 0.17″ from 1500 to 2500 (4 s of the Sun's motion), 0.44″
at 3000, 0.64″ in the year 1 and 1.05″ in 1001 BCE: the claimed 1″ holds
from the year 1 and just fails at the far end. Nutation of −14.03″ is in
the Sun at J2000.0 and the agreement there is 0.06″, so Horizons' column
is of the true equinox. The Moon is within 3″ of DE441 from 1500 to 3000,
within 9.5″ in 999, and then departs: 62.5″ in the year 1 and 129″ in 1001
BCE. At the Moon's elongation rate of about 0.51″ a second, that is 5 s
from 1500 to 3000, 19 s in 999, 2 minutes in the year 1 and 4 minutes in
1001 BCE. The cause was not isolated; the lunar theory's secular
acceleration, which DE441 does not share, is the likely one. The README's
10″ holds from about 1000 CE and not over the whole era it names. The
distance is within 42 km (0.01 %) from 999 to 3000 and 29 km in the year 1.
Differences of 0.7″ in the Moon (its light-time) are not separable from the
series' own error at these dates.

**Against the NAOJ's 暦要項, 2024 to 2026.** The Observatory prints each
term and each phase in 中央標準時 to the minute [nao-rekiyoko-2024,
nao-rekiyoko-2025, nao-rekiyoko-2026, nao-rekiyoko-2024-sakugenbo,
nao-rekiyoko-2025-sakugenbo, nao-rekiyoko-2026-sakugenbo]. Whether a minute
is rounded or truncated is not stated on the pages read; a mean residual
near zero says rounding. Rounding alone has an rms of 17.3 s.

| Events | Source of the time | n | Mean | rms | Largest |
| --- | --- | ---: | ---: | ---: | ---: |
| 24 solar terms of 2024, 2025 and 2026 | `solar_longitude_after` | 72 | +0.8 s | 16.5 s | 30.2 s |
| of them, on the observed ΔT table | | 54 | +1.1 s | 16.9 s | 30.2 s |
| of them, on the predicted ΔT | | 18 | −0.1 s | 15.2 s | 29.7 s |
| new moons, quarters and full moons | Meeus chapter 49 | 149 | +1.1 s | 18.2 s | 39.5 s |
| the same | `moon_phase_at_or_after` | 149 | +0.1 s | 18.2 s | 41.9 s |

The largest of the 72 term residuals is 0.2 s beyond the half minute that
rounding allows. They are the 72 terms that
`crates/hc-seasons/tests/rekiyoko_solar_terms.rs` holds, and
[solar-terms-and-pentads.md](solar-terms-and-pentads.md) has the account by
ΔT regime. The four phases separately, by the closed form
and the inversion, have means between −5.5 s and +3.6 s and rms 16.5 s to
20.7 s: neither route has a bias beyond the rounding.

**Against the USNO, 1700 to 2099.** Its Earth's Seasons and Dates of
Primary Phases of the Moon services give Universal Time to the minute
[usno-earth-seasons, usno-moon-phases-data]. Ten years read: 40 equinoxes
and solstices and 494 phases. The pages do not give the ΔT or the theory.

| Year | ΔT used here (1 June) | Seasons: mean, largest | Phases, closed form: mean, rms, largest | Phases, inversion: mean, rms, largest |
| --- | ---: | --- | --- | --- |
| 1700 | 8.9 s | +42.4 s, 58.6 s | +25.5 s, 31.7 s, 52.5 s | +23.8 s, 30.4 s, 55.5 s |
| 1800 | 13.6 s | −12.8 s, 25.9 s | −11.0 s, 21.4 s, 49.3 s | −9.8 s, 21.4 s, 41.1 s |
| 1900 | −2.2 s | +13.0 s, 35.7 s | +14.5 s, 21.7 s, 43.9 s | +11.9 s, 21.4 s, 40.2 s |
| 1950 | 29.2 s | +19.3 s, 38.9 s | +2.8 s, 19.7 s, 35.3 s | +2.8 s, 19.3 s, 38.5 s |
| 2000 | 63.9 s | +1.5 s, 28.3 s | +2.0 s, 17.3 s, 30.1 s | +1.9 s, 19.4 s, 46.3 s |
| 2024 | 69.2 s | +10.3 s, 33.0 s | +11.2 s, 21.8 s, 50.4 s | +9.7 s, 20.3 s, 42.6 s |
| 2025 | 69.1 s | +15.6 s, 29.3 s | +5.9 s, 18.9 s, 39.5 s | +5.0 s, 18.1 s, 40.6 s |
| 2026 | 69.1 s | +12.5 s, 32.6 s | +14.9 s, 24.8 s, 46.7 s | +14.6 s, 24.3 s, 44.4 s |
| 2050 | 93.8 s | +8.9 s, 21.1 s | +2.0 s, 19.0 s, 34.9 s | +0.5 s, 18.5 s, 37.7 s |
| 2099 | 201.4 s | −87.5 s, 102.3 s | −87.6 s, 89.0 s, 120.4 s | −87.8 s, 89.1 s, 109.1 s |

Four seasonal events or about fifty phases a year, each with the minute's
rounding of ±30 s. From 1700 to 2050 every event is within a minute of the
USNO's (the largest residual is 58.6 s), and the rms of 18 to 32 s is not
far above rounding's 17 s. Against this library the USNO's times of 2024
to 2026 are 6 to 16 s earlier on average, where the NAOJ's are 0 s; the
pages do not say why. At 2099 the library is 88 s from the USNO, which
would be a ΔT 88 s smaller than the 201.4 s the extrapolation here gives,
if it is all ΔT; the USNO's ΔT is not stated on the pages read. The ΔT
beyond 2033 is the library's documented forecast and its own limitation
([time-scales.md](../time-scales.md)).

**Between the two routes to a phase.** `nth_moon_phase` against
`moon_phase_at_or_after`, every fifth lunation from 1001 BCE to 3000 CE
and all four phases, 39 600 events, series minus inversion:

| Years | n | Mean | rms | Largest |
| --- | ---: | ---: | ---: | ---: |
| −1000 to −501 | 4 944 | −1.0 s | 15.3 s | 48.4 s |
| −500 to −1 | 4 948 | +2.1 s | 11.3 s | 36.3 s |
| 0 to 499 | 4 948 | −1.6 s | 10.6 s | 36.8 s |
| 500 to 999 | 4 948 | −1.1 s | 8.6 s | 38.4 s |
| 1000 to 1499 | 4 947 | +1.7 s | 7.2 s | 32.5 s |
| 1500 to 1999 | 4 945 | +1.2 s | 6.7 s | 31.1 s |
| 2000 to 2499 | 4 948 | +1.0 s | 6.7 s | 31.6 s |
| 2500 to 2999 | 4 948 | +2.9 s | 7.6 s | 29.6 s |
| 3000 | 24 | −2.8 s | 8.1 s | 18.9 s |

The routes part by 7 s rms in the centuries around the present and by
15 s in the first millennium BCE, never by a minute. In 1001 BCE the
position series is 4 minutes from DE441 and the routes are within 48 s of
each other, so the closed form must carry nearly the same departure (an
inference: the closed form was not itself compared with DE441), and their
agreement cannot show an error that both carry.

**Against Meeus's own examples**, the figures as the module tests quote
them (the book was not read):

| Example | Quantity | Measured | The test's bound |
| --- | --- | --- | --- |
| 25.a | Sun's mean longitude 201.807 20° | 201.807 20° | 10⁻⁴ ° |
| 25.b | apparent longitude 199°54′21.56″ | +0.19″ | 0.54″ |
| 25.b | right ascension, declination | 0.17″, 0.09″ | 1.1″ |
| 47.a | the three sums Σ*l*, Σ*b*, Σ*r* of the 120 rows | within 2 units | 2 units |
| 47.a | geometric longitude 133.162 655°, latitude −3.229 126° | 10⁻⁶ ° | 10⁻⁵ ° |
| 47.a | distance 368 409.7 km | 0.02 km | 0.2 km |
| 47.a | apparent longitude 133.167 265° | 0.12″ | 3.6″ |
| 49.a | new moon of 1977 February, JDE 2 443 192.651 18 | 0.23 s | 2 s |
| 49.b | last quarter of 2044 January, JDE 2 467 636.491 86 | 0.34 s | 2 s |

**The equation of time**, from the module's test against JPL Horizons'
apparent hour angle at Greenwich [jpl-horizons]: within 0.02 s on 2000
January 1 and 2024 November 3 (0.012 s in the example above), within
0.25 s on 1000 January 1 and within 0.7 s on 30 July 587 BCE, where the two
ΔT models are 185 s apart. In 2024 the equation of time peaks at +16.45
minutes about 3 November and bottoms at −14.20 minutes on 12 February, and
is −6.4 minutes on 31 July against the USNO's "about −7" read off its graph
[usno-eqtime]; it crosses zero four times, on about 15 April, 13 June,
1 September and 25 December.

**The seed of the search.** The comment on `solar_longitude_after` says the
seed is never more than about two days out because the equation of the
centre is bounded by 1.92°. Over every start in 2024 at six-hour steps and
every target at 5°, the largest distance between seed and answer is 3.88
days, from the difference of the equation of the centre at the start and at
the target. The 5-day bracket still holds it with 1.1 days to spare. For
the Moon, over starts every half day for 600 days from 2024 and targets at
15°, the seed is out by at most 1.27 days against a bracket of 2.

**Known disagreements:**

- The Moon's apparent longitude is 62″ and 129″ from DE441 in the year 1
  and in 1001 BCE, and 9.5″ in 999; the README's about 10″ is for the era
  from about 1000 CE, and it states the era of 1000 BCE to 3000 CE for all
  of its rows.
- The module documentation of `vsop87` measures the truncation from 1000 to
  3000; the era the exports serve begins in 1001 BCE. The comparison with
  Horizons covers the whole of it for the total error. The truncation alone
  was not re-measured, since the full series is a data file that was not
  downloaded.
- The seed comment, above.

## Sources

- [nao-faq-24sekki]: the terms as the Sun's apparent longitude at 15°
  steps, 春分 at 0°. Read 2026-10-03.
- [nao-faq-moon], [nao-rekiwiki-moon-phases]: the phases as the elongation
  of the apparent longitudes at 0°, 90°, 180° and 270°, from the centre of
  the Earth. Read 2026-10-03.
- [nao-rekiyoko-2024], [nao-rekiyoko-2025], [nao-rekiyoko-2026]: the 72
  terms of 2024 to 2026. [nao-rekiyoko-2024-sakugenbo],
  [nao-rekiyoko-2025-sakugenbo], [nao-rekiyoko-2026-sakugenbo]: the 149
  phases. Read again for this document on 2026-10-03, as the pages' own
  Shift_JIS HTML; the fetch tool's summary of the 2026 phase page named the
  phases wrongly and was not used.
- [usno-moon-phases-faq]: the phases defined by the apparent longitudes.
  [usno-moon-phases-data], [usno-earth-seasons]: the phases and seasons of
  ten years, from the services' data. [usno-eqtime]: the apparent and mean
  solar times, the two causes, the extremes. [usno-sun-approx]: the
  ecliptic to equatorial formulas. All read 2026-10-03.
- [vsop87-cds-readme], [vsop87-cds-notice]: the version, the frame, the time
  scale, the precision, the record count. Read in full 2026-10-03. The
  IMCCE's directory
  `https://ftp.imcce.fr/pub/ephem/planets/vsop87/` was listed and holds no
  `VSOP87.doc` under that name (404); no data file was opened.
- [jpl-horizons-sun-moon]: the Sun and the Moon at twelve dates, 1001 BCE to
  3000, through the fetch tool, which summarises (the digits agree with the
  series, so they were not altered). [jpl-horizons]: the hour angles of the
  equation-of-time test, read 2026-09-27 per its entry and not re-read.
- [meeus1998], *Astronomical Algorithms*, 2nd ed.: chapters 22, 25, 32, 47,
  48, 49 and the worked examples 22.a, 25.a, 25.b, 47.a, 48.a, 49.a and 49.b.
  **Not read.** Cited as `hc-astro` cites it and as its tests quote it.
- [bretagnon1988] (VSOP87), [chapront1983] (ELP-2000/82), [laskar1986]
  (the obliquity), [capitaine2003] (the IAU 2006 precession),
  [reingold2018] (§14.4, the predicate for a wrapping angle): **not read.**

## Code

`crates/hc-astro/src/solar.rs`, `vsop87.rs`, `lunar.rs`, `search.rs` and the
nutation, obliquity and equatorial functions of `earth.rs`; at the boundary
`crates/hyper-calendar/src/sky_lines.rs`, and in `astro_lines.rs`
`hc_equation_of_time`, `solar::equation_of_time` in seconds at an instant,
and in `season_lines.rs` `hc_principal_phases_in_month`, the principal phases
inside a Gregorian month at a meridian from `hc-seasons`'s
`moon_calendar::principal_phases_in_month`. The tests that anchor them: in
`solar`, `the_apparent_solar_longitude_matches_meeus_example_25b`,
`the_solar_position_matches_meeus_example_25b`,
`every_published_seasonal_event_falls_inside_the_claimed_accuracy`,
`solar_longitude_after_lands_on_its_target`,
`the_seasons_are_not_of_equal_length`,
`the_equation_of_time_is_horizons_hour_angle` and
`the_equation_of_time_reaches_its_known_annual_extremes`; in `vsop87`,
`meeus_example_25b_comes_out_of_the_series` and
`every_series_keeps_the_stated_number_of_terms`; in `lunar`,
`the_periodic_sums_match_meeus_example_47a`,
`the_lunar_position_matches_meeus_example_47a`,
`the_illuminated_fraction_matches_meeus_example_48a`,
`the_new_moon_of_february_1977_matches_meeus_example_49a`,
`the_last_quarter_of_january_2044_matches_meeus_example_49b` and
`the_two_independent_conjunction_algorithms_agree`; in `search`, the
bisection tests; in `earth`,
`the_nutation_in_longitude_matches_meeus_example_22a`; in
`crates/hc-seasons/tests/rekiyoko_solar_terms.rs`, the 72 terms; in
`sky_lines`, `september_2026_is_where_the_almanac_puts_it`.
