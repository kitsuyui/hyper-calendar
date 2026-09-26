# Rising and setting: sunrise, sunset, twilight, moonrise and moonset, and the horizon they are measured against

Backs `hc-astro::riseset` and `hc-astro::horizon`. No calendar identifier
is registered: these are events of a day at a place, not calendars. The
calendars that begin their day at a sunset or read a tithi at a sunrise
use them.

## What it is

Sunrise is the moment the Sun's upper limb appears on the visible
horizon, and sunset the moment it disappears. Every authority read for
this document agrees on that much. They do not agree on where the visible
horizon is:

- **Refraction.** The air bends the light of a body on the horizon
  upward, so the Sun is seen above the horizon while it is geometrically
  below it. The USNO takes the bending at the horizon as 34′
  [usno-rst-definitions]. Meeus takes the same 34′ [meeus1998, ch. 15],
  and so does *Calendrical Calculations* [reingold2018code]. The NAOJ
  takes 35′8″ [nao-rekiwiki-hinode-teigi]. The value is a convention: the
  real bending depends on the air, and the USNO warns that a computed time
  "may be in error by a minute or more" for that reason alone.
- **The limb.** Every authority read takes the Sun's semidiameter as 16′.
  For the Moon the USNO uses its apparent radius, which runs from 15′ to
  17′ over the month, and Meeus's 0.2725 of the horizontal parallax is the
  same thing. *Calendrical Calculations* fixes it at 16′.
- **Height.** An observer above the sea sees the horizon below the
  horizontal, the *dip*, and so sees the Sun earlier in the morning and
  later in the evening. The USNO computes "the observer's eye … on the
  surface of the Earth", whatever the height, and its data service takes
  no height [usno-rst-definitions, usno-api-rstt]. The NAOJ also computes
  for 0 m, because the dip only appears when a point stands above its
  surroundings; where all the land around is equally high, the horizon is
  not lowered [nao-rekiwiki-hinode-teigi]. Those who do allow for height
  compute it differently (below).

The definition has also changed over time. The NAOJ's history of its own
almanac records sunrise defined by the Sun's centre with no refraction in
the Edo period, refraction added from the almanac for 1877, the upper
limb tried in the almanac for 1879, and the upper limb in use from the
almanac for 1903. It also notes that France still
defines sunrise by the Sun's centre [nao-rekiwiki-hinode-teigi].

Twilight is defined by the Sun's centre. Civil, nautical and astronomical
twilight end when the centre is 6°, 12° and 18° below the horizon
[usno-rst-definitions]. That is a geometric depression, with no refraction
and no dip. *Calendrical Calculations* takes its twilights the same way
[reingold2018code].

## How it works

A rising or a setting is the moment a body's geometric altitude *h*
crosses a fixed value *h*₀. For the Sun, *h*₀ is minus the sum of the
refraction, the semidiameter and whatever the height adds. The Moon is
near enough for its parallax to outweigh the refraction: its geocentric
centre is at *h*₀ = π − *s* − 34′ when the upper limb is on the horizon,
with π the horizontal parallax and *s* the semidiameter
[usno-rst-definitions].

The conventions differ in what the height adds:

- **The geometric dip.** A sphere of radius *R* seen from height *h*
  shows its horizon arccos(*R*/(*R* + *h*)) below the horizontal. That is
  1.926′·√*h* for *h* in metres [soma2001, eq. 6].
- **Calendrical Calculations.** The book's `refraction` is 34′ plus the
  geometric dip, with *R* = 6 372 km, plus a further 19″·√*h*. Its
  `sunrise` and `sunset` add 16′ to that, and `observed-lunar-altitude`
  adds the same 16′ to the Moon's topocentric altitude,
  *h* − arcsin(sin π · cos *h*) [reingold2018code]. The code does not say
  where the 19″ comes from, and the book's chapter was not read.
- **Sôma.** Sôma follows Smart in taking the grazing ray as an arc of
  constant curvature, whose bending he puts at about 1/13 of the angle
  the ray subtends at the Earth's centre. The dip that
  corrects an *observed altitude* is 1.77′·√*h* (eq. 15; Newcomb 1.76′,
  the *Explanatory Supplement* 1.75′). A rising or a setting also needs
  the bending of the ray between the horizon point and the observer, and
  that correction is 2.09′·√*h* (eq. 16; Newcomb 2.11′, the *Explanatory
  Supplement* 2.12′). Sôma shows that the geometric 1.926′·√*h* is the
  least the true correction can be, so the 1.77′ that the 理科年表 had
  applied to rise and set times is wrong [soma2001].

Between those, *Calendrical Calculations*' 1.926′ + 0.317′ = 2.24′ per
root metre is 7 % above Sôma's 2.09′, and the geometric dip is 8 % below it.

The event is then a root of *h*(*t*) − *h*₀. This library brackets it
between the Sun's lower transit and its upper transit (the Sun) or scans
the local day in twenty-minute steps (the Moon), and bisects to 10⁻⁷ day.
*Calendrical Calculations* finds the Sun's depression another way. Its
`approx-moment-of-depression` solves for the hour angle from the Sun's
declination and turns the answer from local sundial time into mean time
with its equation of time. It repeats until a step is under 30 s. It finds
the Moon's crossing by bisection until the bracket is under a minute and
returns the bracket's midpoint [reingold2018code].

### Worked example: sunset at Jerusalem on 1 January 2024

Take `calendar-code2`'s Jerusalem: 31.78° N, 35.24° E, 740 m.

The depression of the Sun's centre below the geometric horizon, under each
convention:

- The USNO's: 34′ + 16′ = 50′.
- The default here: 50′ plus the geometric dip,
  arccos(6 372 000/6 372 740) = 52.4′, which is 102.4′.
- *Calendrical Calculations*': 102.4′ plus 19″ × √740 = 516.9″ = 8.6′,
  which is 111.0′.

How fast the Sun sinks at that moment: its altitude falls at
15′ per minute of time × cos φ × cos δ × sin *H*. With φ = 31.78°, the
Sun's declination δ ≈ −23.0° on the day, and the hour angle *H* ≈ 75.9°
at a 50′ sunset (cos *H* = (sin *h*₀ − sin φ sin δ)/(cos φ cos δ)), that
is 15′ × 0.8501 × 0.9205 × 0.9697 = 11.4′ per minute.

So the geometric dip puts the sunset 52.4/11.4 = 4.6 minutes after the
USNO's. The book's 8.6′ more puts it a further 8.6/11.4 = 0.76 minute
(45 s) later.

The USNO publishes 16:46 at UT+2 [usno-api-rstt]. The three horizons here
give 16:45:48, 16:50:23 and 16:51:08. The 45 s between the last two is
the size of the steady 40–47 s by which this library's sunsets at
Jerusalem ran ahead of the book's sample dates before the book's horizon
was carried under its own name.

## What is carried

`hc-astro::horizon::Horizon` is a named horizon: the refraction, the
Sun's semidiameter, a `Dip` for the height and a `MoonLimb` for the Moon,
with its source. `horizon::HORIZONS` lists them and `horizon::by_id` finds
one:

| Identifier | Refraction | Height *h* | The Moon's limb |
| --- | --- | --- | --- |
| `geometric-dip` (the default) | 34′ | geometric dip, *R* = 6 372 km | 0.2725π, centre at 0.7275π − 34′ − dip |
| `usno` | 34′ | ignored | 0.2725π, centre at 0.7275π − 34′ |
| `calendrical-calculations` | 34′ | geometric dip + 19″·√*h* | 16′, parallax arcsin(sin π cos *h*) |

`riseset::sunrise`, `sunset`, `moonrise` and `moonset` use
`geometric-dip`, which is what they have always computed. The
`riseset::sunrise_with`, `sunset_with`, `moonrise_with` and
`moonset_with` variants take a horizon by name. At sea level the default
and `usno` are the same horizon.

`riseset::dawn` and `dusk` take a `Twilight`, whose depression is the
Sun's centre below the geometric horizon. No horizon applies to them.

The default is kept for the callers it already has: the observational
Hijri and Hebrew calendars, the Bahá'í day, the tithi at sunrise and the
hours of the day. Each part of it is sourced, the sea-level horizon from
Meeus and the USNO and the dip from `calendar-code2`. By Sôma's analysis
it undercounts the effect of height by 8 %, and its name says what it
does.

Not carried, and why:

- **The NAOJ's 35′8″.** The NAOJ publishes its times to the minute. Its
  1′8″ more refraction moves a Tokyo sunrise by about 6 s, so the
  published minutes cannot tell the two apart. The Tokyo anchors below
  hold the default to the NAOJ's minute instead.
- **Sôma's 2.09′·√*h*, Newcomb's 2.11′ and the *Explanatory
  Supplement*'s 2.12′.** No published table of rise or set times computed
  with any of them was read to anchor one. The *Explanatory Supplement*
  and Newcomb are cited only as Sôma reports them.
- **Sunrise by the Sun's centre**, the French definition. No French table
  was read.
- **Terrain.** A mountain or a building on the horizon is not modelled,
  as neither the USNO nor the NAOJ model it.
- **The book's standard-time day.** `calendar-code2`'s `moonrise` and
  `moonset` look for the event between two midnights of the zone's
  standard time. This library looks between two local mean midnights,
  because it knows no time zones. A moonrise in the minutes between the
  two midnights falls on a different day. The comparison below takes the
  event in the book's day.

## Accuracy

**Against the NAOJ, at sea level.** Tokyo's sunrise and sunset on
1 January 2024 are within a minute of the NAOJ's 06:50 and 16:38
[nao-koyomi-dni-tokyo-2024].

**Against the USNO, at a place 740 m up.** Four days of 2024 at
Jerusalem are the solstices, the equinoxes and New Year's Day. On them,
the sixteen sunrises, sunsets, moonrises and moonsets under the `usno`
horizon are within 0.49 minute of the USNO's published minutes, which is
the rounding [usno-api-rstt]. The default and `calendrical-calculations`
put the risings earlier and the settings later, by 3.7 to 6.2 minutes. The
USNO's times do not change with height, since its service takes none.

**Against *Calendrical Calculations*.** The book's code gives the sample
dates of its Appendix C, R.D. −214 193 to 764 652 (586 BCE to 2094), in
`crates/hyper-calendar/tests/data/calendrica_sample_dates.txt`. The test
`crates/hyper-calendar/tests/rd_sample_dates.rs` compares the book's
rise-and-set columns there with this library. The columns:

| Column | Place | Convention | Measured, before | Measured, under the book's horizon |
| --- | --- | --- | --- | --- |
| `set` | Jerusalem, 740 m, UT+2 | `sunset`, the book's horizon | −47 s to −7 s | within 2.8 s, once the book's sundial error is taken out |
| `dawn` | Paris, 27 m, UT+1 | 18° below the geometric horizon, no horizon | −1.2 s to +39 s | within 2.8 s, the same way |
| `mid-day` | Tehran, UT+3:30 | the Sun's transit | not compared | within 3 s, the same way |
| `moonrise`, `moonset` | Mecca, 298 m, UT+3 | the book's horizon | −47 s to +50 s | within 23 s, the book's final bracket |

The `set` column's steady gap at Jerusalem was the horizon: 19″·√740 is
8.6′ of arc, 40–47 s of time. At Mecca the 19″·√298 is 5.5′, and the
Moon's 16′ against 0.2725π differs by −0.8′ to +1.3′ over the month.

Two further differences are the book's methods, not its horizon, and the
test measures them rather than hiding them in a wide bound:

- **The sundial error.** The book puts its solar events on the clock
  through the equation of time. Its equation of time takes the mean Sun's
  longitude at dynamical time, while mean time runs with the Earth's
  rotation in Universal Time. In antiquity ΔT is hours, so the two
  drift apart: in 586 BCE, with ΔT = 18 496 s, the book's sundial
  time runs 38 s ahead of the Sun's hour angle. `hc-astro`'s
  `solar_time::local_apparent_time` is built the same way and shows the
  same 38 s. This library's sunrise, sunset and transit solve for the
  hour angle directly from sidereal time. The test takes the book's value
  as this library's event less the amount by which
  `local_apparent_time` there runs ahead of the hour angle, and the
  remainder is within 2.8 s at every date, 586 BCE included. After about
  1500 CE the correction is under a second.
- **The Moon's bisection.** The book halves a twelve-hour bracket ten
  times, to 42.2 s, and returns its midpoint, so its moonrise and moonset
  carry up to 21.1 s of the search. This library's crossing under the
  book's horizon is within 21.9 s of every value. The same ten halvings
  on this library's Moon reproduce the book's to 1.7 s, which is the two
  ephemerides' difference in the Moon's altitude at midnight, where the
  book starts its bracket.

2094 is compared like every other date for these columns. The book's
ΔT there carries the sign error its errata correct and is 62 s from this
library's [reingold2018errata]. That moves the Sun by 2.5″, a sixth of a
second of time, and the Moon's crossing by a few seconds, inside the
bounds.

## Sources

- [usno-rst-definitions] — the USNO's definitions of sunrise, sunset,
  moonrise, moonset and the three twilights; its observer on the surface
  of the Earth, and the warning that height is left out. Read
  2026-09-27.
- [usno-api-rstt] — the USNO's "Complete Sun and Moon Data for One Day"
  service, for Jerusalem on four days of 2024. The service's
  documentation was read, and the service answered the same times with a
  height of 740 m and of 3 000 m. Read 2026-09-27.
- [nao-rekiwiki-hinode-teigi] — the NAOJ's definition: the upper limb, a
  horizontal refraction of 35′8″, 0 m. Also the history of the Japanese
  almanac's definition and the French one. Read 2026-09-27.
- [soma2001] — the geometric dip, the dip for an observed altitude and
  the correction for a rising or a setting, with Newcomb's and the
  *Explanatory Supplement*'s coefficients as he reports them. Read
  2026-09-27.
- [reingold2018code] — `refraction`, `sunrise`, `sunset`, `dawn`, `dusk`,
  `approx-moment-of-depression`, `moment-of-depression`, `sine-offset`,
  `equation-of-time`, `sidereal-from-moment`, `lunar-altitude`,
  `lunar-parallax`, `topocentric-lunar-altitude`,
  `observed-lunar-altitude`, `moonrise`, `moonset`, `binary-search`, and
  the locations `jerusalem`, `paris`, `mecca` and `tehran`, read in
  `calendar.l` on 2026-09-27; `dates.l` for the columns.
- [reingold2018] — the book those functions come from; its chapter on
  astronomical events not read here.
- [reingold2018errata] — the ΔT correction for 2051–2150, as the sample
  date test cites it.
- [meeus1998] — chapter 15, the altitudes −0°50′ and 0.7275π − 34′. Not
  read for this document; cited as `hc-astro` has cited it.
- [nao-koyomi-dni-tokyo-2024] — Tokyo's sunrise and sunset on 1 January
  2024.

## Code

`crates/hc-astro/src/horizon.rs` and `crates/hc-astro/src/riseset.rs`.

The tests that anchor them are in `riseset`:
`sunrise_in_tokyo_on_new_years_day_matches_the_national_ephemeris`,
`sunset_in_tokyo_on_new_years_day_matches_the_national_ephemeris`,
`the_usno_horizon_matches_the_usno_at_an_elevated_site`,
`a_horizon_lowered_for_height_rises_earlier_and_sets_later_than_the_usno`
and `at_sea_level_the_default_is_the_usnos_horizon`.

In `horizon`:
`calendrical_calculations_adds_nineteen_seconds_of_arc_a_root_metre`,
`the_two_moons_differ_by_the_semidiameter_and_the_parallax_at_altitude`
and the catalogue's own tests.

In `crates/hyper-calendar/tests/rd_sample_dates.rs`:
`the_rising_and_setting_are_within_seconds_of_the_books`.
