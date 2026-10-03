# Planetary clocks: the body table, the solar day, local mean solar time, the lunar clock and the dated moment

Backs `hc-planetary`'s `bodies` (`Body`, `ALL`, `by_id`, `solar_day_days`,
`derived_solar_day_days`, `year_in_local_days`, `ClockEpoch`), `clock`
(`BodyClock`, `LocalTime`), `moon` (`lunation_meeus`, `lunation_brown`,
`age_days`, `selenographic_colongitude`, `mean_solar_time`) and `dated`
(`DatedCalendar`, `ALL`, `by_id`), and the exports `hc_bodies`, `hc_body_time`
and `hc_circad_date`. No calendar identifier is registered for the first
three: they are rates and readings of the rotation of bodies. The identifiers
`dated` lists are `darian-titan`, `gregorian-io`, `gregorian-europa`,
`gregorian-ganymede`, `gregorian-callisto` and `martiana`, each of which is a
calendar documented elsewhere:
[circad-calendars.md](circad-calendars.md) for the five circad calendars
and [mars-timekeeping.md](mars-timekeeping.md) for Mars (the Mars Sol Date,
Coordinated Mars Time, true solar time, the Mars year, the mission sols and
Martiana). This document does not repeat either.

## What it is

A planet's **sidereal day** is one turn of its rotation against the fixed
stars. Its **solar day** is the interval from one noon to the next, which
differs from the sidereal day because the planet has moved along its orbit and
the Sun has moved in its sky. NASA's Planetary Fact Sheet draws the same line:
its "Rotation Period" is the time "relative to the fixed background stars (not
relative to the Sun)", negative for a retrograde rotation, and its "Length of
Day" is the time for the Sun to return to noon at a point on the equator
[nssdc-factsheet-notes]. On Earth the two are 23.9345 h and 24 h; on Venus the
sheets give −5 832.6 h and 2 802.0 h, and the solar day is the shorter of the
two, because Venus turns the other way round [nssdc-planet-sheets].

Three things follow that a clock on another body has to settle:

- **The rate.** A body's hour, as a twenty-fourth of its solar day, is a
  physical quantity. It is measured from the rotation and the year, and it is
  not the Earth's: 61.6 minutes on Mars, 15.97 hours on Titan, 4.86 Earth days
  on Venus.
- **The zero point.** Where local midnight at the prime meridian falls is a
  convention. The IAU Working Group on Cartographic Coordinates and Rotational
  Elements (WGCCRE) reports the poles and prime meridians of the planets,
  satellites and minor planets about every three years, and its latest report
  is that of 2015, published in 2018 [usgs-iau-wgccre, archinal2018]. A prime
  meridian is a place on the surface. A time scale also needs an instant at
  which that place had midnight, and only the Earth (Universal Time) and Mars
  (Coordinated Mars Time, [mars-timekeeping.md](mars-timekeeping.md)) have one
  agreed.
- **Mean or true.** A mean solar clock follows a fictitious Sun that moves
  uniformly. A sundial follows the real Sun, which runs ahead of or behind it
  by the equation of time. Only Mars has a series for the difference in this
  crate.

The **Moon** is the special case, and its solar day is a month. It is tidally
locked, so it turns once on its axis in one sidereal month (27.321 66 days),
while the Earth–Moon pair moves on around the Sun; the Moon must go on a
little farther to bring the Sun back, so the mean synodic month, 29.530 59
days, is "nearly 2.21 days longer" [nasa-eclipse-moon-orbit]. New Moon is, by
the astronomical convention, the instant at which the geocentric ecliptic
longitudes of the Sun and Moon are equal, and a lunation runs from one to the
next [nasa-eclipse-moon-orbit]. Lunations are numbered in two common ways, and
the two differ by a constant: Brown's number 1 began at the first new moon of
1923 (about 02:41 UTC on 17 January) and Meeus's number 0 at the first new
moon of 2000 (about 18:14 UTC on 6 January), so that Brown's number is Meeus's
plus 953 [wikipedia-new-moon]. Lunar observers plan by the **selenographic
colongitude of the Sun**, which says where the terminator lies and so which
craters are lit at what angle.

No standard lunar time exists. The US Office of Science and Technology
Policy's memorandum of 2 April 2024 asks NASA for a strategy to implement
lunar timing standardization by 31 December 2026 and describes Coordinated
Lunar Time as the standard to be established; that is as the crate's module
documentation reports it, since the memorandum is a PDF and was not read
[ostp-2024-celestial-time]. The IAU's XXXII General Assembly, which
concluded on 15 August 2024, accepted a Resolution II, which defines a Lunar
Celestial Reference System and Lunar Coordinate Time (TCL), and a Resolution
III "on the establishment of a coordinated lunar time standard by
international agreement" [iau-2024-press-release]. TCL is a relativistic
coordinate time like TCG, not a civil scale, again as the crate reports it
[iau-2024-resolution-ii]. How a lunar clock ticks against an Earth clock
is the subject of [relativity.md](relativity.md).

A **dated moment on another body** is a day count and a fraction of that day,
written in a calendar whose day is that body's. The crate's `dated` module
lists the calendars for which it can say what the date is at an instant; their
rules are in the two documents named above.

## How it works

### From the sidereal rotation and the year to the solar day

Let *P*sid be the sidereal rotation period, signed, positive when the body
turns the same way it orbits, and *P*orb its sidereal period around the
**Sun**. The Sun's direction against the stars changes by one turn in *P*orb,
and the body's by one turn in *P*sid, so the Sun returns to the same place in
the body's sky at the rate of the difference of the two:

```text
1 / P_solar = 1 / P_sid − 1 / P_orb         |P_solar| is the solar day
```

The module documentation of `bodies` gives the two points where this goes
wrong in a careless version, and the tests pin both.

- **The year is the planet's, also for a moon.** What moves the Sun in a
  moon's sky is its planet's year. With the period around the primary, every
  tidally locked moon would give a rate of zero and an infinite day. With the
  planet's year the Moon's solar day comes out as the synodic month, which is
  what it is.
- **The rotation carries a sign.** For a retrograde rotator the two
  reciprocals add in magnitude, and the solar day is shorter than the sidereal
  day, not longer. The signed result is negative, which says the Sun rises in
  the west; `solar_day_days` returns the magnitude.

Two consequences are checked by the tests. A body in a *P*sid = 2⁄3 *P*orb
resonance, which is Mercury's, has 1/*P*solar = 1/(2*P*orb), a solar day of
two years and a year of half a solar day. A tidally locked moon has a solar
day a little longer than its orbit, by the ratio of its orbit to its planet's
year: Titan's 15.969 against 15.945 days.

The table's **measured** day takes the place of the derived one in three rows,
because there the measurement is the primary datum and the derivation inherits
the rounding of a rotation period quoted to six figures: Earth's 86 400 s,
Mars's sol as the Mars24 ratio `SOL_IN_DAYS` × 86 400 s (88 775.244 147 s,
against the 88 775.244 s of the quoted sol;
[mars-timekeeping.md](mars-timekeeping.md)), and the Moon's mean synodic
month, `hc_astro::MEAN_SYNODIC_MONTH` = 29.530 588 861 days.
`year_in_local_days` is the heliocentric year divided by the solar day that
applies.

### The clock

A `BodyClock` is a body and its solar day in days. The clock counts days from
a `ClockEpoch`: the TT offset from J2000.0 of a local mean midnight at the
prime meridian, the number given to the day that begins there, and a basis,
`Standard` or `Convention`. For an instant *t* in TT days from J2000.0 and a
west longitude λ, the continuous day index is

```text
index = (t − epoch_offset) / P_solar + day_number − s·λ / 360
```

with *s* = +1 for a prograde rotator and −1 for a retrograde one, so that
local time runs ahead towards the east on Earth and towards the west on Venus,
and a caller who passes an east longitude does not have to think about it. The
whole part of the index is the local day number and the fraction is how far
through that day the place is. The instant is TAI, which has no rotational
content, and the step to TT is the fixed 32.184 s. The reading is shown on a
24-hour face of twenty-fourths and sixtieths of the *local* day, truncated so
that the last moment of a day never prints as 24:00:00. A local second is
therefore *P*solar seconds of SI time, with *P*solar in Earth days: 116.75 s
on Venus.

The zero points are these. Earth's is 2000-01-01T00:00 TT (an offset of −0.5
day) numbered by Rata Die, `Standard`; the clock is uniform, so it parts
company with UT1 by ΔT ([earth-rotation.md](earth-rotation.md), and
[time-scales.md](../time-scales.md)). Mars's is the Mars Sol Date's, from
the `mars` module, `Standard`. The Moon's is Meeus's mean new moon of lunation
0, `Convention`. For every other row the crate declares J2000.0 to be local
mean midnight at the prime meridian and numbers the days from 0, `Convention`:
only the rate is physical there, and the module says so in every row's `note`.

### The lunar clock and the lunation number

`moon::mean_solar_time` is `BodyClock` for the Moon at a west longitude. Its
day is the mean synodic month and its midnight at the prime meridian is a mean
new moon, which is physically right, since the near side faces away from the
Sun then. Its zero is the mean new moon of Meeus's formula (49.1), JDE
2451550.097 66 + 29.530 588 861 *k*, at *k* = 0; Meeus's chapter 49 was not
read here (`hc-astro` cites it). It is therefore a mean clock.

The lunation number is not. `lunation_meeus` is `hc_astro::lunar`'s
`lunation_containing`: the *k* whose true new moon, with the periodic terms,
is at or before the instant and the next is after it. A true new moon is never
more than about fourteen hours from the mean one, as `hc-astro`'s
documentation has it, so the clock's day number and the lunation agree except
near a new moon, where they can differ by one (see the lunar example below).
`lunation_brown` is that plus the constant 953. `age_days` is the time since
that true new moon and runs to about 29.5 days or a little more, because the
true lunation is not the mean; `illuminated_fraction` is `hc-astro`'s.

**The selenographic colongitude** is Meeus's chapter 53 applied to
`hc-astro`'s positions of the Moon and the Sun: the optical-libration formulas
with the Sun in place of the Earth give the selenographic longitude *l*₀ and
latitude *b*₀ of the sub-solar point, with the lunar equator inclined 1.542
42° to the ecliptic, and the colongitude is 90° − *l*₀. By that rule it is
270° at new moon, 0° at first quarter, 90° at full moon and 180° at last
quarter, and advances about 12.2° a day. The sub-solar latitude stays within
about 1.6°.

### The dated moment

`dated::DatedCalendar` is a two-variant enum: `Circad(CircadCalendar)` for the
five circad calendars in `circad::all()`'s order, then `Martiana`. `ALL` has
six entries, `id` gives the identifier, and `by_id` finds one by the
workspace's name rule (`hc_core::catalogue::matches`: surrounding white space
ignored, ASCII case folded). A boundary that selects a calendar by a string
looks it up here and keeps no list of its own (policy §5).

`hc_circad_date`'s line, from `hyper_calendar::planetary_lines`, has ten
tab-separated columns: the calendar, the year, the month, the day of the
month, the month's name, the name of the day's place in the week, the count
the date is numbered by, the fraction of that day elapsed, `1` for a leap
year, and the source. The count differs by calendar:

- For a **circad calendar** the day is a circad, a fixed fraction of the
  moon's solar day, and the count is the circad number from the calendar's
  epoch (a Julian Circad). `circad_and_fraction_at` turns the instant into a
  count and the fraction elapsed, `date_from_circad` into a date, and the week
  is eight circads.
- For **Martiana** the day is a sol at Airy-0. The count is the Darian sol
  number, from `darian::sol_from_mars_sol_date` of the Mars Sol Date, the
  fraction is the Mars Sol Date's fractional part, and
  `martiana::date_from_sol` gives the date. The week cell is empty on the
  epagomenal sol of every tenth year, which belongs to no week.

The instant of every `planetary` export is a POSIX second as a double. It is
read as UTC and carried to TAI through the leap-second table with the last
published offset held, UTC being taken as TAI before 1961. It is refused as
out of range outside 100 Julian years either side of J2000.0 (`SPAN_DAYS`, 36 525
TT days: 1899-12-31T12:00 TT to 2100-01-01T12:00 TT), which is the span
over which Allison and McEwen state their Mars series. The body table has no
span of its own, and the same limit holds for it.

**Worked example.** Venus's solar day from its sidereal rotation and its year,
then a clock reading.

1. The table row is *P*sid = −5 832.5 h and *P*orb = 224.701 d, the year
   around the Sun. *P*sid in days is −5 832.5 / 24 = −243.020 833 d.
2. The rates, in turns per day: 1/(−243.020 833) = −0.004 114 874 and
   1/224.701 = +0.004 450 358. They have opposite signs, so subtracting
   *adds* their magnitudes:
   1/*P*solar = −0.004 114 874 − 0.004 450 358 = −0.008 565 232.
3. The magnitude is 1/0.008 565 232 = 116.751 07 d, which is 2 802.026 h or
   10 087 292.4 s. It is the length of day, and it is under half the sidereal
   day, which is 243.02 d. The negative sign says the Sun rises in the west. A
   version that gave the rotation no sign would compute 1/243.0208 − 1/224.701
   and report |*P*| = 2 980.76 d, 25.5 times too long.
4. The sheet's own figure is 116.750 d and 2 802.0 h [nssdc-planet-sheets].
   The crate's is 0.001 07 d (92 s) longer, 9 parts in 10⁶: the rotation
   period is quoted to five figures. With the Venus sheet's −5 832.6 h it
   would be 116.752 03 d (2 802.049 h), and with the IAU rate that the source
   field names, −5 832.444 h, 116.750 53 d; all three are within 0.002 d.
5. The Venusian year is 224.701 / 116.751 07 = 1.9246 solar days: one Venus
   year is not two Venus days, which is what `year_in_local_days` returns.
6. The clock. A local hour is 116.751 07 / 24 = 4.864 63 d = 420 303.85 s. At
   2024-01-11T11:57:00Z, 8 775.998 7 TT days from J2000.0, the index at the
   prime meridian is 8 775.998 7 / 116.751 07 = 75.168 47: day 75, fraction
   0.168 47, which is 4.043 17 local hours, 04:02:35. A place at 90° W has the
   sign of the shift reversed: 75.168 47 + 90/360 = 75.418 47, the same day at
   10:02:35, six local hours *later*. On a prograde body such as Mars the same
   step is earlier, not later.

**The same arithmetic on the Moon.** The Moon's row is *P*sid = 655.720 h,
which is 27.3217 d, and the year is the Earth's 365.256 d. From the sheet's
periods alone, 1/(1/27.3217 − 1/365.256) = 29.530 636 d. The table carries the
mean synodic month instead, 29.530 588 861 d, so the clock's day is 708.734 h
and its hour 29.531 Earth hours, or 106 310 s; the sheet's day is 708.7 h
[nssdc-planet-sheets]. The mean new moon of lunation 0 is JDE
2451550.097 66, 2000-01-06 at about 14:19:34 UTC, and the clock reads 00:00:00
of day 0 there. The true new moon of that lunation is at 18:14 UTC
[wikipedia-new-moon]. At 18:14:00 UTC the clock reads day 0, 00:07:56,
while the Meeus lunation has just become 0 and its age is 0.0002 d; between
14:19 and 18:14 the lunar clock is already on day 0 and the lunation is still
−1. The difference is the 3.9 h by which a true new moon lagged the mean one,
which is 0.0055 of a lunar day.

**A dated moment.** `hc_circad_date("darian-titan", unix)` at
2002-12-18T10:42:00Z, the superior conjunction on which Gangale calibrates the
Titan calendar [gangale-titan], writes year 209, month 9 (Aries), day 13, the
week name *Solis*, count 144 096, fraction 0.000 003, not a leap year. That is
13 Aries 209, Julian Circad 144 096. For `martiana`, at Perseverance's
landing, 2021-02-18T20:43:48Z, it writes year 219, month 1 (Sagittarius), day
11, *Sol Saturni*, count 146 433, fraction 0.4470, a leap year; the Darian
calendar, for the same sol, reads 13 Sagittarius 219
([mars-timekeeping.md](mars-timekeeping.md)): the two part by a sol at each
century not divisible by 500, which is two sols by year 219.

## What is carried

**The body table** has 22 rows, ordered outward from the Sun with each
planet's moons after it. Each has an identifier (the lower-case English name),
a kind (`star`, `planet`, `dwarf-planet` or `moon`), its primary, a signed
sidereal rotation period, a signed sidereal orbit period about the primary, an
axial tilt, a semi-major axis, an optional measured solar day, a source string
and a clock epoch. The derived quantities, on the table's figures:

| Body | Rotation (h) | Orbit about primary (d) | Solar day (h) | Year in local days | Day origin |
| --- | --- | --- | --- | --- | --- |
| Sun | 609.12 | none | none | none | no clock |
| Mercury | 1 407.5088 | 87.969 | 4 222.555 | 0.499 99 | derived |
| Venus | −5 832.5 | 224.701 | 2 802.026 | 1.9246 | derived |
| Earth | 23.9345 | 365.256 | 24 | 365.256 | measured |
| Moon | 655.720 | 27.3217 | 708.734 | 12.3687 | measured |
| Mars | 24.6229 | 686.980 | 24.6598 | 668.599 | measured |
| Phobos | 7.653 84 | 0.318 91 | 7.657 39 | 2 153.15 | derived |
| Deimos | 30.298 56 | 1.262 44 | 30.354 34 | 543.17 | derived |
| Ceres | 9.074 170 | 1 681.63 | 9.0762 | 4 446.69 | derived |
| Jupiter | 9.9250 | 4 332.589 | 9.925 95 | 10 475.79 | derived |
| Io | 42.459 31 | 1.769 138 | 42.4767 | 2 447.98 | derived |
| Europa | 85.228 35 | 3.551 181 | 85.2983 | 1 219.04 | derived |
| Ganymede | 171.709 27 | 7.154 553 | 171.9933 | 604.571 | derived |
| Callisto | 400.536 41 | 16.689 017 | 402.0852 | 258.607 | derived |
| Saturn | 10.656 | 10 755.699 | 10.656 44 | 24 223.55 | derived |
| Enceladus | 32.885 23 | 1.370 218 | 32.8894 | 7 848.63 | derived |
| Titan | 382.690 10 | 15.945 421 | 383.2583 | 673.532 | derived |
| Uranus | −17.24 | 30 685.4 | 17.2396 | 42 718.49 | derived |
| Neptune | 16.11 | 60 189.018 | 16.1102 | 89 666.07 | derived |
| Triton | −141.044 50 | −5.876 854 | 141.0307 | 10 242.71 | derived |
| Pluto | −153.2928 | 90 560 | 153.2820 | 14 179.36 | derived |
| Charon | −153.2928 | 6.3872 | 153.2820 | 14 179.36 | derived |

Earth, the Moon and Mars use the measured day. The orbit column is the period
about the row's primary (the Moon's is its orbit of the Earth), and the last
column divides the year about the Sun, which for a moon is its planet's, by
the solar day: the Moon's 12.3687 is lunations per Earth year, and Phobos's
2 153.15 is Phobos's days in a Martian year. Triton is the only retrograde
orbit, and Venus, Uranus, Pluto, Triton and Charon the retrograde rotators.
Only Earth and Mars have a `Standard` clock zero.

**Contested figures are carried as they stand,** with the conflict in the
row's `source`, as policy §5 and §3 ask:

- Mercury: 1 407.5088 h, which the row calls the IAU 2015 rate. NASA's sheet
  prints 1 407.6 h and a length of day of 4 222.6 h, which 1 407.5088 h gives
  (4 222.56 h) and 1 407.6 h does not (4 223.38 h).
- Venus: −5 832.5 h from the comparison table; the Venus sheet says −5 832.6
  h, and the row names the IAU's −5 832.444 h.
- Neptune: 16.11 h, which the sheet prints; the row says that JPL publishes it
  too and that the IAU's 2015 report adopts 15.9663 h after Karkoschka (2011),
  and that the two conflict. Neither was checked here. The derived solar day
  would be 15.966 48 h.
- The Sun: 609.12 h is the rate at 16° of latitude, as NASA's footnote says;
  the same footnote gives the rate at latitude *L* as (14.37 − 2.33 sin²*L* −
  1.56 sin⁴*L*) degrees a day [nssdc-planet-sheets], which at the equator is
  25.05 d. The Sun has no solar day, and `BodyClock::new` returns `None` for
  it.
- Pluto: signed negative because of its 119.51° tilt; Charon takes Pluto's
  rotation, since the two are locked.
- Jupiter: 9.9250 h, which the sheet marks as System III (1965.0) coordinates.
  Saturn: 10.656 h, marked on the sheet too, and the row says of it that
  Cassini showed it is not the interior's; the Saturn footnote and the Cassini
  result were not read.

**The clock and the readings.**

- `BodyClock::new`, `for_id`, `at_prime_meridian`, `at_west_longitude`,
  `at_east_longitude`, `prime_meridian_index`, `is_standardised`,
  `solar_day_seconds`.
- `LocalTime`: `day_number`, `day_fraction`, `decimal_hours`, `hour`,
  `minute`, `second`, `solar_day_seconds`, `local_hour_seconds`,
  `since_local_midnight` and a `Display` as `HH:MM:SS`.
- `Body::local_day_index` (the day index at the prime meridian from a TT
  offset), the raw mechanism of `BodyClock`.
- `moon`: `lunation_meeus`, `lunation_brown`, `BROWN_MINUS_MEEUS_LUNATION`
  (953), `age_days`, `illuminated_fraction`,
  `subsolar_selenographic_position`, `selenographic_colongitude`,
  `subsolar_latitude`, `LUNAR_EQUATOR_INCLINATION` (1.542 42°),
  `mean_solar_time`, `universal_time_moment` (the TAI to UT bridge, through
  `hc-astro`'s ΔT model) and `COORDINATED_LUNAR_TIME_STATUS`, the sentence
  that says no LTC was defined as of 2026-09-26.
- `dated`: `DatedCalendar`, `ALL`, `by_id`.

**The exports** carry `hc_bodies` (one 12-column line per body: the
identifier, name, kind, primary, sidereal rotation, solar day in seconds,
`measured` or `derived`, year in local days, `standard` or `convention`, the
zero point's note, the source, and on the Moon's row the Coordinated Lunar
Time status), `hc_body_time` (8 columns: the day number, the fraction, the
reading and its decimal hours, the solar day and the local hour in seconds,
the basis and the note; the Sun is `HC_ERR_NO_DATA` and an unknown body
`HC_ERR_UNKNOWN`) and `hc_circad_date` (10 columns, above). The functions of
`moon` are not exported: the Moon appears in `hc_bodies` with its status cell
and in `hc_body_time` as one more body.

**Not carried**, with the reason:

- *An ephemeris or propagator for any body but Mars.* The table is data; not
  yet done (the crate README says so).
- *True solar time on any body but Mars.* The equation of time needs the
  eccentricity and the obliquity of each orbit; the table carries the
  obliquity and no eccentricity, and no source for other bodies' series was
  read. Not yet done.
- *The IAU rotational elements*: the pole directions and the prime meridians'
  angles *W*₀ and *W*-dot with their periodic terms, from which a body's own
  rotation is defined. The report's tables were not read (the report is a PDF
  and only the abstract page was). The crate holds one mean rate per body, and
  its clock zeros are declared, not the IAU's.
- *Rotation that varies with latitude* for the Sun, Jupiter and Saturn, beyond
  the Sun's law quoted above: not yet done.
- *A clock zero for any body but Earth and Mars.* None is standardised; the
  crate says so (policy §4) and does not invent one.
- *Coordinated Lunar Time and TCL.* No LTC was defined as of 2026-09-26; TCL
  is a coordinate time, `hc-relativity`'s subject
  ([relativity.md](relativity.md)).
- *Bodies other than the 22.* Other moons (the Saturnian sheet lists Dione,
  Hyperion and many more [nssdc-satellite-sheets]), asteroids and comets: not
  yet done. Hyperion's row prints "C" where the others print "S", and what
  that code means on the sheet was not looked up.
- *Lunar surface mission days* (the Chang'e 4 and Yutu-2 count): no published
  list of boundaries was found; the roadmap row says so
  ([calendars.md](../calendars.md)).

## Accuracy

The table is data, and what can be wrong is the data; the derived solar day,
the clock and the lunar numbers are arithmetic on it.

**The table against the Planetary Fact Sheet.** Every figure of the 22 rows
that the sheets print was compared with the sheets (comparison table, the
per-planet sheets, the Mars sheet's moons, the Pluto sheet's Charon and the
three satellite sheets [nssdc-factsheets, nssdc-planet-sheets,
nssdc-satellite-sheets]) on 2026-10-03. Rotation, orbit period, tilt and
semi-major axis, where the sheets print them, of the Sun, the planets, the
Moon, Phobos, Deimos, Io, Europa, Ganymede, Callisto, Enceladus, Titan, Triton
and Charon agree in every printed digit except these:

| Quantity | Crate | NASA sheet | Difference |
| --- | --- | --- | --- |
| Mercury sidereal rotation | 1 407.5088 h | 1 407.6 h | the crate's choice, documented |
| Venus sidereal rotation | −5 832.5 h | −5 832.5 h comparison table; −5 832.6 h Venus sheet | the sheets disagree; documented |
| Venus semi-major axis | 108.209 × 10⁶ km | 108.210 × 10⁶ km (the comparison table prints 108.2) | 1 000 km, 10⁻⁵ |
| Pluto semi-major axis | 5 906.4 × 10⁶ km | 5 869.656 × 10⁶ km on the Pluto sheet; 5 906.4 in the comparison table | 0.6 %; not used by any derivation |
| Europa sidereal rotation | 85.228 35 h | 3.551 181 d × 24 = 85.228 344 h | 0.02 s |
| Ceres orbit period | 1 681.63 d | 1 679.853 d, the JPL Small-Body Database's osculating orbit at epoch JD 2461200.5 (TDB) [jpl-sbdb-ceres] | 0.1 %; the orbit drifts with the epoch |
| Ceres semi-major axis | 414.0 × 10⁶ km | 2.765 553 au = 413.7 × 10⁶ km [jpl-sbdb-ceres] | 0.08 % |

The satellite sheets print the rotation as "S", synchronous, and the table
carries 24 × the orbit period in hours; the Mars sheet prints Phobos's and
Deimos's rotation periods equal to their orbit periods. The sheets print no axial tilt for any satellite: the table's 0.0 for
Phobos, Deimos, the Galilean moons, Enceladus, Triton and Charon, and Titan's
0.3°, have no source in what was read (the Saturnian sheet's 0.33 for Titan is
an orbital inclination, not an obliquity), and `axial_tilt_degrees` and
`semi_major_axis_km` are used by no derivation in the crate. Charon's negative
sign is the crate's, from the locking; the Charon sheet's rotation period is
positive, 6.3872 d. Ceres's rotation, 9.074 170 h, is the Small-Body
Database's, whose cited source is Nature 537 (2016) 515–517; the row cites
Konopliv et al. (2018) [konopliv2018], which was not read.

**The derived day against the sheets' "Length of Day".** The table's derived
solar day against the comparison table and per-planet sheets, in hours:

| Body | Derived | NASA | Difference |
| --- | --- | --- | --- |
| Mercury | 4 222.555 | 4 222.6 | −0.045 (−1.1 × 10⁻⁵) |
| Venus | 2 802.026 | 2 802.0 | +0.026 (+9.2 × 10⁻⁶) |
| Moon | 708.734 | 708.7 | +0.034 (the sheet's three figures) |
| Mars | 24.6598 | 24.6597 | +0.0001 |
| Jupiter | 9.925 95 | 9.9259 | +0.0001 |
| Saturn | 10.656 44 | 10.656 | +0.0004 |
| Uranus | 17.2396 | 17.24 | −0.0004 |
| Neptune | 16.1102 | 16.11 | +0.0002 |
| Pluto | 153.2820 | 153.2820 | 0 |

Earth is 24 by definition; the derived figure from 23.9345 h and 365.256 d is
24.000 03 h, 0.1 s. Each difference is within the rounding of the sheet's own
figure, the Moon's 708.7 and Uranus's 17.24 having four significant digits;
Mercury and Venus differ by 1.1 and 0.9 parts in 10⁵, which is what figures
quoted to five or six digits can give. The tests hold the measured day against
the derived one to 0.2 s for Earth, 0.4 s for Mars and 1 s for the Moon; the
actual gaps are 0.10 s, 0.225 s (88 775.019 s derived against 88 775.244) and
0.72 s. The sheets' orbital periods in the comparison table are *tropical*
(equinox to equinox, "also known as the tropical orbit period"
[nssdc-factsheet-notes]), as against the sidereal periods the crate takes from
the per-planet sheets, which is the right one for the Sun's return among the
stars; for Saturn the difference moves the solar day by 1.3 ms.

**The synodic month.** The Moon's measured day is Meeus's constant,
29.530 588 861 d, to be compared with NASA's 29.530 59 [nasa-eclipse-moon-orbit] and with
the Moon sheet's 29.53 [nssdc-planet-sheets], which agree. The same page gives
the sidereal month as 27.321 66 d (the table's 27.3217) and, for a span of
5 000 years, a shortest and longest lunation of 29.265 74 d and 29.840 89 d: the
true month swings by about 13.8 h over a mean of 29.53 d, which is why the
lunar clock and the true lunation part. `hc-astro`'s documentation of the
constant (`lunar.rs`) gives the range as 29.27 to 29.83 d, which is Meeus's,
not read here.

**The error of a clock that runs on these figures.** A relative error ε in a
solar day grows into a phase error of ε × *N* local days after *N* days. The
stored digits alone set a floor: Venus's −5 832.5 (±0.05 h, 9 × 10⁻⁶) drifts
0.003 local day in a century (36 525 d); Jupiter's 9.9250 h
(±0.000 05 h, 5 × 10⁻⁶) 0.44 of a Jovian day in the 88 314 days of a century; Uranus's −17.24
(±0.005 h, 2.9 × 10⁻⁴) about 15 days. These are rounding of what is stored,
not the uncertainty of the measurement, which is the source's. The crate
states the same thing in words: parts in 10⁵, fine for a day's length and
useless for carrying a day count across a century. The exports refuse an
instant beyond 100 Julian years of J2000.0, not because the arithmetic stops
there but because the Mars series does.

**The lunar numbers.** The lunation number agrees with the published new moons
of 2000-01-06 18:14 UT (lunation 0), 1923-01-17 (Brown 1, Meeus −952;
published as about 02:41 UTC [wikipedia-new-moon]; the test places the day,
not the minute) and the new moons of 2024-01-11 11:57 UT and later, though
`hc-astro`'s truncated series puts a conjunction about 20 minutes from the
published instant, so the age at the published instant is a hair short of a
whole lunation. The colongitude reproduces Meeus's example 53.a as the test
quotes it (Meeus not read), 1992 April 12.0 TD (1992-04-11T23:59:01 UTC):
longitude 67.92° against 67.9°, latitude +1.48° against +1.46° and colongitude
22.08° against 22.1°. The four phases of 2024 (11 January 11:57, 18 January
03:52, 25 January 17:54 and 2 February 23:18 UT) give colongitudes of 273.6°,
354.8°, 86.8° and 186.7°, against the rule of thumb's 270°, 0°, 90° and 180°;
the error is the libration in longitude, and the test allows 8°. The age of
the Moon at the instant of the full moon of 25 January 2024 is 14.248 days.

**Differences found between the code or its documents and the sources.**

- `bodies.rs`, the Mercury row's comment, and the README: using NSSDC's
  1 407.6 h "would stretch the derived solar day by half an hour". By the
  crate's own formula 1 407.6 h gives 4 223.376 h against 4 222.555 h, 0.82 h,
  49 minutes, and the ratio to the orbit is 2.0004 against 2.00002.
- `bodies.rs`, the same comment: the IAU rate is written 6.138 5108 degrees a
  day. 360 × 24 / 6.138 5108 = 1 407.5075 h, 4.7 s from the 1 407.5088 h the
  row carries (which is 6.138 505 1 degrees a day). The effect on the solar
  day is 0.012 h. The IAU report was not read, so which figure it prints is
  not settled here.
- `bodies.rs`, the satellite rows and the README: satellite rotation rates are
  said to be "cross-checked against the IAU WGCCRE". Every satellite row
  equals 24 × the NASA orbit period, so no IAU value enters, and the report's
  tables were not read here.
- `bodies.rs`, the Ceres row: the rotation is attributed to Konopliv et al.
  (2018), and the JPL database lists another source for the same figure
  (above).
- `bodies.rs` test
  `a_retrograde_rotators_solar_day_is_shorter_than_its_sidereal_day`: the
  comment calls Pluto's gap "twenty-odd seconds"; the assertion beneath it,
  and the arithmetic, give 38.9 s.
- `bodies.rs`, the `measured_solar_day_seconds` documentation: the derivation
  from the table is "a third of a second per sol" short for Mars, "four hours
  over the span of the Mars Sol Date". It is 0.225 s per sol, about three
  hours over 52 000 sols.
- `clock.rs`, the Moon's `day_number` and `bodies.rs`'s Moon row say the day
  number "is the Meeus lunation number". It is that only away from a new moon:
  the clock's days begin at mean new moons and the lunations at true ones, up
  to about fourteen hours apart (the lunar example above). The test
  `the_lunar_clock_numbers_its_days_by_the_meeus_lunation` allows a difference
  of one, and checks three dates away from a new moon.
- `moon.rs`, the test `meeus_lunation_zero_is_the_new_moon_of_january_2000`
  says "Meeus (49.1): the new moon of 2000 January 6, at 18:14 UT". Formula
  (49.1) gives the *mean* new moon, JDE 2451550.097 66 (14:19 UTC); 18:14 UT
  is the true one [wikipedia-new-moon].
- `clock.rs`, the test `a_local_hour_is_a_twenty_fourth_of_the_bodys_own_day`
  checks `(hour − seconds) / seconds < 1e-3` with no absolute value, so it
  cannot fail for a local hour that is too short. The `local_hour_seconds`
  documentation prints Titan's as 57 490 s; it is 57 488.7 s.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [nssdc-factsheets] | The comparison table: rotation period, length of day, orbital period (tropical), distance from the Sun and obliquity of the planets and the Moon | Yes, 2026-09-26 and again 2026-10-03 |
| [nssdc-planet-sheets] | The per-body sheets of the Sun, the planets and the Moon, with Phobos and Deimos on the Mars sheet and Charon on the Pluto sheet: the sidereal orbit and rotation periods, the length of day, the obliquity, the semi-major axis, and the footnotes on the Sun, Jupiter, Saturn, Uranus and Neptune | Yes, 2026-10-03 |
| [nssdc-satellite-sheets] | The Jovian, Saturnian and Neptunian satellite sheets: orbit periods, semi-major axes, the synchronous "S" | Yes, 2026-10-03 |
| [nssdc-factsheet-notes] | The definitions of rotation period, length of day, orbital period and obliquity | Yes, 2026-10-03 |
| [nasa-eclipse-moon-orbit] | The mean synodic month 29.530 59 d, the sidereal month 27.321 66 d, the definition of new moon, the range of the lunation | Yes, 2026-10-03 |
| [wikipedia-new-moon] | The Brown and Meeus lunation numbers and BLN = LN + 953; the instants of Brown 1 and Meeus 0 | Yes, 2026-10-03; secondary |
| [usgs-iau-wgccre] | That the 2015 report, published in 2018, is the latest of the triennial reports | Yes, 2026-10-03 |
| [archinal2018] | The IAU WGCCRE report on cartographic coordinates and rotational elements: the abstract (the report updates Mercury, Mars, Phobos, Deimos, Ceres, Europa, Neptune, Pluto and Charon among others) | The abstract page only, 2026-10-03, through the USGS publication record; the report's tables were not read |
| [jpl-sbdb-ceres] | Ceres's rotation period and osculating orbit | Yes, 2026-10-03 |
| [iau-2024-press-release] | Resolutions II and III of 2024 and their date | Yes, 2026-10-03 |
| [ostp-2024-celestial-time] | The memorandum's date, the 31 December 2026 deadline, 58.7 µs per day | No: a PDF; cited through the code's module documentation |
| [iau-2024-resolution-ii] | TCL as a coordinate time | No: a PDF |
| [meeus1998] | Chapters 47, 49 and 53: the lunar arguments, the mean phase (49.1), the selenographic position of the Sun, the inclination 1.542 42° | No |
| [konopliv2018] | The Ceres rotation as the row cites it | No |
| [gangale-titan] | The Titan calibration of 2002-12-18 | Yes, 2026-09-26 per [circad-calendars.md](circad-calendars.md); not re-read |

The Mars figures (the sol, the Mars Sol Date) are those of
[mars-timekeeping.md](mars-timekeeping.md) and its sources. The Springer
page of the IAU report redirected to an authorisation endpoint and was not
followed. A correction to the report (2019) is listed in a search of the
publisher's pages; it was not read.

## Code

`crates/hc-planetary/src/bodies.rs`, `clock.rs`, `moon.rs` and `dated.rs`, and
in `crates/hyper-calendar/src/planetary_lines.rs` the lines `bodies_lines`,
`body_time_line` and `circad_date_line`. The tests that anchor them:

- `bodies`: `a_solar_day_on_mercury_is_two_mercurian_years`,
  `venus_rotates_backwards_and_still_gets_a_sensible_solar_day`,
  `the_earths_solar_day_comes_out_at_exactly_twenty_four_hours`,
  `the_moons_solar_day_is_the_synodic_month`,
  `the_martian_solar_day_derived_from_the_table_is_the_sol`,
  `a_measured_solar_day_agrees_with_the_one_derived_from_the_table`,
  `a_retrograde_rotators_solar_day_is_shorter_than_its_sidereal_day`,
  `a_prograde_rotators_solar_day_is_longer_than_its_sidereal_day`,
  `a_tidally_locked_moons_solar_day_is_a_little_longer_than_its_orbit`,
  `the_synodic_relation_holds_in_reverse_for_every_body` and
  `only_earth_and_mars_have_a_standardised_clock_zero`.
- `clock`: `the_generic_mars_clock_reproduces_coordinated_mars_time`,
  `the_generic_mars_clock_reproduces_local_mean_solar_time`,
  `the_earth_clock_numbers_its_days_by_rata_die`,
  `the_earth_clock_is_uniform_and_so_parts_company_with_ut1`,
  `longitude_runs_the_other_way_on_a_retrograde_body` and
  `a_local_day_takes_a_solar_day_of_si_time`.
- `moon`: `meeus_lunation_zero_is_the_new_moon_of_january_2000`,
  `the_brown_lunation_count_begins_in_january_1923`,
  `the_meeus_worked_example_for_the_suns_selenographic_position_reproduces`,
  `the_colongitude_is_two_hundred_and_seventy_at_new_moon`,
  `the_lunar_mean_solar_day_is_the_synodic_month`,
  `the_lunar_clock_numbers_its_days_by_the_meeus_lunation` and
  `lunar_midnight_at_the_prime_meridian_is_new_moon`.
- `dated`: `every_circad_calendar_and_martiana_is_listed_once` and the
  catalogue tests; in `planetary_lines`, `titans_calibration_is_209_aries_13`,
  `every_circad_calendar_answers`,
  `the_body_table_names_its_measured_days_and_the_lunar_status` and
  `the_span_is_a_century_either_side_of_j2000`.
