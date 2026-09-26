# Hours of the day: local mean time, sundial time, temporal and Italian hours, and religious times

Backs `hc-astro::solar_time`. No calendar identifier is registered: these
are readings of the time of day, not calendars.

## What it is

Before railways and telegraphs made a shared clock necessary, every town
kept its own time, and "noon" meant the moment the Sun stood highest there.
Two readings came out of that, and both survive as the definitions that
standard time was built on:

- **Local apparent time**, or sundial time, is the hour angle of the true
  Sun at the place, plus twelve hours. A sundial reads it directly. Its
  hours are not all the same length over the year, because the Sun's
  motion in right ascension is not uniform.
- **Local mean time** is the same reading taken from a fictitious mean Sun
  that moves uniformly, so that every day is 24 equal hours. It differs
  from Universal Time, the mean time at Greenwich, only by the longitude:
  an hour for every 15°.

The difference between the two is the **equation of time**: the sundial
runs up to about 16 minutes ahead of the mean clock in early November and
about 14 minutes behind it in mid February.

Older still are hours counted from the Sun's own events:

- **Temporal (seasonal) hours**, the hours of antiquity and of Jewish
  law: the daylight from sunrise to sunset is twelve hours and the night
  twelve more, so that a summer day's hour is long and a summer night's
  short.
- **Italian hours** (*ore italiane*): 24 equal hours counted from the
  "zero hour" half an hour after sunset.

And some religious times are fixed by the Sun's altitude or by a shadow,
with an angle each community chooses: the Islamic afternoon prayer
(*ʿaṣr*) by the length of a shadow, and the Jewish evening by a depression
of the Sun below the horizon.

## How it works

Reingold and Dershowitz state the relations as three functions in
`calendar-code2` [reingold2018code]; the book's chapter 14 explains them
[reingold2018].

1. `local-from-universal`: local mean time = UT + λ/360°, the longitude
   λ east-positive, as a fraction of a day.
2. `apparent-from-local`: apparent time = local mean time + E, with the
   equation of time E evaluated at the Universal Time of the instant.
3. `local-from-apparent` and `universal-from-apparent`: the inverses.

### Local mean and sundial time

**Worked example.** Padua, which `calendar-code2` places at 11°53′9″ E.
11.885 83° / 360° = 0.033 016 day = 47 min 32.6 s, so when Greenwich mean
time is 00:00, Padua's local mean time is 00:47:32.6. Meeus's example 28.a
gives the equation of time as +13 min 42.6 s at 0h TD on 13 October 1992
[meeus1998]; with that value a sundial at Padua reads 00:47:32.6 +
13:42.6 = 01:01:15.2. (It reads the hour angle of a Sun below the horizon;
at that hour only the arithmetic, not the sundial, is available.)

### Temporal hours

`daytime-temporal-hour` is (sunset − sunrise)/12; `nighttime-temporal-hour`
is (next sunrise − sunset)/12; `standard-from-sundial` turns a reading in
temporal hours into clock time, with hour 6 at sunrise, 18 at sunset and 0
in the middle of the night before [reingold2018code]. Where there is no
sunrise or no sunset the book returns its `bogus` value.

**Worked example.** NAOJ gives Tokyo's sunrise on 1 January 2024 as 06:50
JST and its sunset as 16:38 [nao-koyomi-dni-tokyo-2024]: 9 h 48 min of
daylight, so a daytime temporal hour is 49 minutes, and temporal 9:00,
three hours after sunrise, falls at 06:50 + 3 × 49 min = 09:17 JST.

### Italian hours

`local-zero-hour` is the moment the Sun's centre is 16′ below the
horizon, its upper limb on the geometric horizon, plus 30 minutes, at
Padua; `italian-from-local` counts the hours since the last zero hour, and
the date turns at the zero hour [reingold2018code].

**Worked example.** If the 16′ sunset at a place is at 18:10 local mean
time, the zero hour is 18:40 and the next day begins then; local mean noon
of that day is 17 h 20 min after it, 17:20 in Italian hours.

### Religious times

In `calendar-code2` [reingold2018code]:

- `alt-asr`, the **Shafiʿi** rule: ʿaṣr is when a vertical object's shadow
  equals its noon shadow plus its own height. With the noon altitude A,
  the Sun's altitude h then has cot h = cot A + 1.
- `asr`, the **Hanafi** rule: the noon shadow plus twice the height,
  cot h = cot A + 2. It is always the later of the two.
- `jewish-dusk`, at a depression of 4°40′, attributed to the **Vilna Gaon**.
- `jewish-sabbath-ends`, at 7°5′, attributed to **Berthold Cohn**.

Each angle is an authority's convention, and other communities use other
ones; the book is the source of all four, and the authorities' own texts
were not read.

## What is carried

In `hc-astro::solar_time`:

- `local_mean_time` and `universal_from_local_mean_time`, and
  `zone_from_longitude`, the offset alone.
- `local_apparent_time`, from Universal Time; `apparent_from_local_mean_time`;
  `local_mean_from_apparent_time` and `universal_from_local_apparent_time`,
  the inverses.

A local reading is returned as a `Moment`, the shape of a day count with a
fraction, but it is not Universal Time: its `day()` is the local date and
its `day_fraction()` the local time of day. The functions take a
`riseset::Location` rather than a bare longitude, as the rise-and-set
functions do, and read only its longitude.

**What differs from `calendar-code2`.** The equation of time is this
crate's `solar::equation_of_time`, Meeus's (28.1) on the VSOP87 Sun
[meeus1998], rather than the book's shorter series from Meeus's page 185,
which it states as an approximation. And the inverse from sundial time is
solved exactly, by iterating, where `local-from-apparent` evaluates the
equation of time at the sundial reading as if it were mean time; that
shortcut is off by the equation's change over its own size, up to about
0.4 s.

Standard (zone) time is not carried here: a zone is a civil decision, and
`hc-tz` owns it.

Temporal and Italian hours, and the religious times:

- `daytime_temporal_hour`, `nighttime_temporal_hour`, as fractions of a
  day; `universal_from_temporal_time`, which returns Universal Time where
  `standard-from-sundial` returns standard time; and `temporal_time`, its
  inverse, which the book does not have.
- `italian_zero_hour`, `italian_time` and `universal_from_italian_time`,
  with the 16′ and the half hour as named constants. The place is the
  caller's, where the book fixes it at Padua: a location is continuous,
  so under [policy.md](../policy.md) §5 it is a parameter. A reading is an
  `ItalianTime`, a date and the hours since the zero hour, rather than a
  moment, because the day between two zero hours is not exactly 24 hours
  and a moment would roll into the next date a minute early or late.
- `asr_shafii`, `asr_hanafi`, `jewish_dusk_vilna_gaon` and
  `jewish_sabbath_ends_cohn`, one function per convention, with the angles
  as named constants. `jewish-morning-end`, the end of the fourth temporal
  hour of the day in the book, is not carried: the book names no authority for it.

Where the event a reckoning needs does not happen — no sunrise or sunset
under the midnight sun or the polar night, no 16′ or 4°40′ depression on a
white night, no Sun at noon for a shadow — the function returns a
`MissingSolarEvent` naming the event and the day. The book returns
`bogus` there; a length of daylight that is not there is not zero, and no
number is returned for it.

All of these are in Universal Time, and use the rise-and-set geometry of
`hc-astro::riseset`: the depressions are of the Sun's centre below the
geometric horizon with no refraction, as the book takes them, while
sunrise and sunset are the refracted upper limb on the visible horizon.

## Accuracy

The relations are exact definitions; the accuracy is the equation of
time's. Against Meeus's example 28.a, +13 min 42.6 s on 1992 October 13.0
TD, the sundial at Greenwich is within 0.5 s. At Padua on seven days of
2024, the sundial reads 12:00 at the Sun's transit as `riseset::solar_noon`
finds it to within 2 s, which is the agreement between the equation of time
and the transit search, not an error of either against the sky. The
inverses return the reading they were given to 0.1 ms.

The temporal hours are as good as the sunrise and sunset under them, which
are within a minute of NAOJ's; Tokyo's daytime hour on 1 January 2024 is
within 0.17 min of the 49.0 min that NAOJ's published minutes give. The
Italian zero hour and the religious times put the Sun's centre at the
stated altitude to 10⁻⁴ degree, and ʿaṣr's shadow at its stated length to
10⁻³ of the object's height. No published table of Italian hours or of
these religious times was read, so beyond NAOJ's sunrise and sunset they
are checked against their own definitions, not against a published value.

## Sources

- [reingold2018code] — `local-from-universal`, `universal-from-local`,
  `apparent-from-local`, `local-from-apparent`, `apparent-from-universal`,
  `universal-from-apparent`, `equation-of-time` and the location of Padua,
  read in `calendar.l` on 2026-09-26.
- [reingold2018] — the book those functions come from; chapter 14 not
  read here.
- [meeus1998] — the equation of time, (28.1), and example 28.a.
- [reingold2018code], again — `daytime-temporal-hour`,
  `nighttime-temporal-hour`, `standard-from-sundial`, `local-zero-hour`,
  `italian-from-local`, `local-from-italian`, `padua`, `asr`, `alt-asr`,
  `jewish-dusk`, `jewish-sabbath-ends` and `dusk`, read on 2026-09-26.
- [nao-koyomi-dni-tokyo-2024] — Tokyo's sunrise and sunset on
  1 January 2024, the anchor of the temporal hour.

## Code

`crates/hc-astro/src/solar_time.rs`. The tests that anchor it:
`padua_keeps_greenwich_time_plus_forty_seven_and_a_half_minutes`,
`a_greenwich_sundial_runs_ahead_by_meeus_example_28a`,
`the_sundial_reads_noon_at_the_suns_transit`,
`apparent_and_mean_time_invert_each_other`,
`a_tokyo_new_years_temporal_hour_matches_the_national_ephemeris`,
`temporal_hours_six_and_eighteen_are_sunrise_and_sunset`,
`temporal_time_inverts_its_universal_time`,
`temporal_hours_are_refused_under_the_midnight_sun_and_the_polar_night`,
`the_italian_zero_hour_is_half_an_hour_after_the_suns_limb_meets_the_horizon`,
`italian_hours_count_a_day_from_one_zero_hour_to_the_next`,
`asr_is_where_the_shadow_rule_puts_it` and
`the_jewish_evening_times_sit_at_their_angles_in_order`.
