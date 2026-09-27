# Hours of the day: local mean time, sundial time, temporal, Italian and Edo hours, and religious times
# Hours of the day: local mean time, sundial time, temporal and Italian hours, religious times, and the Ethiopian and Swahili hours

Backs `hc-astro::solar_time`, and `hc-format::east_african_hours` with the
reckonings `ethiopian-hours` and `swahili-hours`. No calendar identifier is
registered: these are readings of the time of day, not calendars.

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
  law: the daylight is twelve hours and the night twelve more, so that a
  summer day's hour is long and a summer night's short. Jewish law has two
  reckonings of the daylight: the Vilna Gaon's (the Gra's) divides sunrise
  to sunset into twelve, and the Magen Avraham's divides daybreak to
  nightfall [wikipedia-zmanim].
- **Italian hours** (*ore italiane*): 24 equal hours counted from the
  "zero hour" half an hour after sunset.
- **The Edo 不定時法**, Japan's reckoning until the calendar reform of
  1872: the daylight from 明け六つ to 暮れ六つ in six hours and the night in
  six more, so that a summer day's hour was about 2 h 39 min and a summer
  night's about 1 h 21 min [astro-dic-futeijiho]. The hours were named by
  the strokes of the bell that opened them.

In Ethiopia and on the Swahili-speaking coast of East Africa the civil
clock itself is read differently: a twelve-hour dial counted from about
sunrise and about sunset, which near the equator stay close to 06:00 and
18:00 all year, so that the reading is the civil hour less six.

And some religious times are fixed by the Sun's altitude or by a shadow,
with an angle each community chooses: the Islamic afternoon prayer
(*ʿaṣr*) by the length of a shadow, and the Jewish evening by a depression
of the Sun below the horizon.

## How it works

Reingold and Dershowitz state the relations as three functions in
`calendar-code2` [reingold2018code]; the book's chapter 14 explains them
(chapter 14, not read) [reingold2018].

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

### 不定時法, the Edo hours

Each half of the day, 明け六つ to 暮れ六つ and 暮れ六つ to the next 明け六つ,
is divided into six equal hours, 一刻 each [nao-rekiwiki-futeiji]. The
hours are named by the strokes of the bell, nine at noon and midnight and
one fewer at each hour after, down to four, so a day runs 明六つ, 朝五つ,
朝四つ, 昼九つ, 昼八つ, 夕七つ, 暮六つ, 夜五つ, 夜四つ, 暁九つ, 暁八つ, 暁七つ.
The prefixes follow the Observatory's list, 今暁九時, 八時, 七時, 明六時,
朝五時, 四時, 昼九時, 八時, 夕七時, 暮六時, 夜五時, 四時, in which a prefix also
covers the unprefixed hour after it. The almanac itself kept equal hours,
except under the 天保暦: from its almanac of 1844 it gave times in the
unequal hours, with a fraction of an hour in tenths, 分, so that its
暮六時六分 is six tenths of an hour after 暮れ六つ, not 6:06
[nao-rekiwiki-futeiji]. Each hour is paired with an earthly branch,
卯 for 明け六つ, 午 for 昼九つ, 酉 for 暮れ六つ and 子 for 暁九つ
[wikipedia-ja-jikoku]. People took 明け六つ as the start of the day,
though the almanac's day began at midnight [nao-rekiwiki-futeiji].

Where 明け六つ and 暮れ六つ fall has had three rules [nao-rekiwiki-yoake]:

1. Before the 寛政暦 of 1798, a fixed 二刻半, two and a half of the
   hundred 刻 of a day, 36 minutes, before sunrise and after sunset.
2. The 寛政暦 and the 天保暦 turned that into an angle: the Sun's
   altitude 二刻半 after sunset at an equinox at the 改暦所 in Kyoto. At
   an equinox the declination is 0 and the hour angle at sunset 90°;
   二刻半 adds 360° × 2.5 / 100 = 9°, so `sin h = − cos φ sin 9°`, and with
   the 寛政暦書's φ = 35°00′36″, h = −7°21′41″. The 寛政暦書 prints 7度36分
   in its hundred-minute degree, which is 7°21′36″, but computes with the
   formula; the sunrise it adds 9° to is the Sun's centre on the
   horizon, with no refraction.
3. The Observatory's almanacs from 1912 print 夜明 and 日暮 at a
   depression of the Sun's centre of 7°21′40″, "明治五年以前明六つ暮六つと
   称したる時刻に相当す", and the 理科年表 still does
   [koyomi8-yoake-higure]. The Observatory's page traces the second of
   difference to a latitude of 35°0.8′ in a book of 1917, and says it has
   no clear record of why.

**Worked example.** Kyoto, 20 March 2020. こよみのページ puts 夜明, at
7°21′40″, at 5:28:47 JST, and the Sun's centre on the geometric horizon
at 6:04:43, 35 min 56 s later: the 二刻半 that the angle was made to
give [koyomi8-yoake-higure]. This library puts 日暮 that evening at
18:40:40. The daylight is 13 h 11 min 53 s, so each of its six hours is
2 h 11 min 59 s, and 昼九つ, three hours after 明け六つ, begins at 12:04:44,
a quarter of a minute after the Sun crosses the meridian at 12:04:29.

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

### Jewish times in temporal hours

Jewish law fixes several times of the day as a number of temporal hours
(*shaʿot zmaniyot*) from the start of the day [kosherjava-zmanim]
[hebcal-zmanim-api]:

| Time | Hours |
| --- | --- |
| Latest morning Shema (*sof zman kriʾat shemaʿ*) | 3 |
| Latest morning prayer (*sof zman tefillah*) | 4 |
| Earliest afternoon prayer (*minḥah gedolah*) | 6.5 |
| Preferred afternoon prayer (*minḥah ketanah*) | 9.5 |
| *Plag ha-minḥah*, the earliest start of the Sabbath | 10.75 |

Authorities disagree on what "the day" is, and each reckoning is a
convention under its authority's name:

- The **GRA** (the Vilna Gaon): sunrise to sunset, so an hour is a
  twelfth of the daylight and the count starts at sunrise.
- The **MGA** (the Magen Avraham): dawn to nightfall. KosherJava's
  `getShaahZmanisMGA` and Hebcal's `sofZmanShmaMGA` take dawn as 72
  minutes before sunrise and nightfall as 72 minutes after sunset, "the
  time it takes to walk 4 mil at 18 minutes a mil" [kosherjava-zmanim].
  Hebcal also gives the MGA day with dawn and nightfall both at 16.1°
  below the horizon (`sofZmanShmaMGA16Point1`) [hebcal-zmanim-api].

Dawn and nightfall are themselves conventions. KosherJava gives dawn
(*alos ha-shachar*) at 16.1°, the angle that corresponds to the 72
minutes, and nightfall (*tzais*) at 8.5°, "a time that Rabbi Meir Posen in
his *Ohr Meir* calculated that 3 small stars are visible", and also 72
minutes after sunset, Rabbeinu Tam's reckoning [kosherjava-zmanim].

**Worked example.** Hebcal gives sunrise in New York City on 1 January 2025
as 07:20 EST and sunset as 16:40 [hebcal-zmanim-api]: 560 minutes, so a
GRA hour is 46.67 minutes. The latest Shema by the GRA is three hours
after sunrise, 07:20 + 140 min = 09:40, and *minḥah gedolah* is
07:20 + 303.3 min = 12:23. Both are what Hebcal prints. With the 72-minute
dawn at 06:08 and nightfall at 17:52, the MGA day is 704 minutes, its hour
58.67 minutes, and the latest Shema by the MGA is 06:08 + 176 min = 09:04,
also Hebcal's time.
### The Ethiopian and Swahili hours

The dial reads the civil hour less six, on twelve hours: 07:00 is 1, noon
is 6, 18:00 is 12 and midnight 6 again. The Ethiopian day "begins at
sunrise or 0600 hours … 8 am is 2 o'clock for an Ethiopian, 10 am is
4 o'clock … 6 pm, which is 12 o'clock and then the counting begins again,
7 pm is 1 o'clock" [undp-eue-ethiopian-time], and "the daytime cycle begins
at dawn 12:00 (6:00:00 AM EAT) and ends at dusk 11:59:59 (5:59:59 PM EAT)"
[wikipedia-time-in-ethiopia]. In Swahili, "7:00 am is referred to as saa
moja asubuhi to mean that it is the first hour of the day. 7:00 pm is
called saa moja usiku to indicate that it is the first hour of the night",
and the lesson names every hour with its part of the day: *usiku*, night,
from 7 pm to 3 am; *alfajiri*, dawn, 4 to 6 am; *asubuhi*, morning, 7 to
11 am; *mchana*, afternoon, noon to 3 pm; *jioni*, evening, 4 to 6 pm
[ku-kiswahili-lesson-17].

The two agree on the dial and differ in where the halves meet. The
Ethiopian day half is 06:00 to 17:59:59. The Swahili lesson's day hours are
7 am to 6 pm, the night's 7 pm to 6 am, so 06:00 is the night's twelfth
hour, *saa kumi na mbili alfajiri*, and 18:00 the day's, *saa kumi na
mbili jioni*; between the whole hours this library gives a reading the
half of its civil hour, which the lesson's table implies and does not
state.

**Worked example.** 10:30 in Addis Ababa or Dar es Salaam: 10 − 6 = 4,
so 4:30 of the day, *saa nne na nusu asubuhi* in the lesson's own example.
Noon is 6:00 of the day, *saa sita mchana*, and midnight 6:00 of the night,
*saa sita usiku*. 06:15 is 12:15 in both, of the day in Ethiopia and of the
night, *alfajiri*, in Swahili.

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
crate's `solar::equation_of_time`, the Greenwich hour angle of the VSOP87
Sun plus twelve hours, less Universal Time, rather than the book's
shorter series from Meeus's page 185, which it states as an
approximation. The two also differ in time scale. Meeus's (28.1)
[meeus1998] and the book's series both take the mean Sun's longitude at
dynamical time, and the book reads the result against a Universal Time
clock; Universal Time is itself the hour angle of a mean Sun, through
sidereal time, so this library takes the mean Sun there, at Universal
Time, and the true Sun at Terrestrial Time, where its ephemeris is. The
book's choice puts its mean Sun ΔT late, which makes its equation of time
larger by ΔT's worth of the mean Sun's motion, less the difference of the
two mean-Sun polynomials, Meeus's L₀ and the IAU 1982 sidereal time's,
chiefly their squared terms: 0.2 s today, 3 s in 1000 CE and 38 s in
586 BCE, where the motion alone is 51 s and the polynomials take back
12 s. And the inverse from sundial time is
solved exactly, by iterating, where `local-from-apparent` evaluates the
equation of time at the sundial reading as if it were mean time; that
shortcut is off by the equation's change over its own size, up to about
0.4 s.

Standard (zone) time is not carried here: a zone is a civil decision, and
`hc-tz` owns it.

The Edo hours:

- `japanese_dawn_kansei` and `japanese_dusk_kansei`, 明け六つ and 暮れ六つ
  by the 寛政暦's rule, at `KANSEI_DEPRESSION_DEGREES`, the formula of
  rule 2 evaluated, 7°21′41.1″; and `japanese_dawn_naoj` and
  `japanese_dusk_naoj`, the Observatory's 夜明 and 日暮 at 7°21′40″. Two
  angles are two conventions, so two pairs of functions
  ([policy.md](../policy.md) §5), although they are a tenth of a second
  apart. The place is the caller's: the angle was fixed at Kyoto, but a
  depression can be read anywhere.
- `edo_time_kansei` and `universal_from_edo_time_kansei`, a reading and
  its inverse. A reading is an `EdoTime`: the day, which begins at its
  明け六つ, so that the hours after midnight belong to the day before the
  civil date; the hour, an `EdoHour`, with its name, its strokes and its
  branch; and the fraction of the hour gone, with the 天保暦's tenths.
- **Not carried: the fixed 二刻半 of the calendars before 1798**, rule 1.
  The page states it, but not the sunrise it was counted from in each of
  them, and which calendar a date of the 1700s used is the business of
  `japanese-lunisolar.md`, not of a clock.
- **Not carried: the 和時計.** Clocks were made that showed the unequal
  hours by changing their pace or their dial [nao-rekiwiki-futeiji], and
  the stepped settings they used, changed at intervals rather than daily,
  are in no source read. What is carried is the hour the almanac's angle
  defines, day by day.
- **Not carried: the spans of the branches.** The source that pairs each
  hour with a branch gives the branch's hour as about an hour either side
  of a clock time, not where it begins among the unequal hours, so
  `EdoHour::branch` is a name and nothing more. The fixed twelve 辰刻 of
  the almanac, and 更点, the night in five watches, are other reckonings.

Temporal and Italian hours, and the religious times:

- `daytime_temporal_hour`, `nighttime_temporal_hour`, as fractions of a
  day; `universal_from_temporal_time`, which returns Universal Time where
  `standard-from-sundial` returns standard time; and `temporal_time`, its
  inverse, which the book does not have. These are the Vilna Gaon's
  reckoning, sunrise to sunset, the one the book's
  `daytime-temporal-hour` computes.
- The Magen Avraham's reckoning, daybreak to nightfall, under each of the
  two pairs of daybreak and nightfall that the sources read give it:
  `temporal_hour_mga_72_minutes` and `temporal_hour_mga_16_1_degrees`, with
  `temporal_hour_gra`, the Vilna Gaon's, beside them. Each pair is a
  convention of its own name, not a parameter ([policy.md](../policy.md)
  §5). Other pairs, such as the 19.8° or 90-minute ones Hebcal and
  KosherJava also list, are not carried.
- `italian_zero_hour`, `italian_time` and `universal_from_italian_time`,
  with the 16′ and the half hour as named constants. The place is the
  caller's, where the book fixes it at Padua: a location is continuous,
  so under [policy.md](../policy.md) §5 it is a parameter. A reading is an
  `ItalianTime`, a date and the hours since the zero hour, rather than a
  moment, because the day between two zero hours is not exactly 24 hours
  and a moment would roll into the next date a minute early or late.
- `asr_shafii`, `asr_hanafi`, `jewish_dusk_vilna_gaon` and
  `jewish_sabbath_ends_cohn`, one function per convention, with the angles
  as named constants.
- The Jewish times in temporal hours as a table, `Zman`, with the
  identifiers `sof-zman-shma`, `sof-zman-tfila`, `mincha-gedola`,
  `mincha-ketana` and `plag-hamincha`, and one function per reckoning:
  `zman_gra`, `zman_mga_72_minutes` and `zman_mga_16_1_degrees`. The
  book's `jewish-morning-end`, the end of the fourth temporal hour, for
  which the book names no authority, is `zman_gra` of `sof-zman-tfila`:
  the end of the fourth hour from sunrise by the GRA. The rule is the
  *Shulchan Arukh*'s last time for the morning prayer, "until the end of
  four hours, which is a third of the day" (*Orach Chayim* 89:1)
  [shulchan-arukh-oc-89], and counting those hours from sunrise is the
  Vilna Gaon's reckoning as the *Mishnah Berurah* gives it (58:4)
  [mishnah-berurah-58].
- Dawn and nightfall: `jewish_dawn_16_1_degrees`, `jewish_dawn_72_minutes`,
  `jewish_nightfall_8_5_degrees` and `jewish_nightfall_72_minutes`.
  Candle lighting, the other angles and minute counts Hebcal prints
  (*misheyakir*, 7.083°, 42 and 50 minutes) and the Baal HaTanya's
  reckoning are not carried.

Where the event a reckoning needs does not happen — no sunrise or sunset
under the midnight sun or the polar night, no 16′ or 4°40′ depression on a
white night, no 16.1° dawn in London in June, no Sun at noon for a shadow —
the function returns a `MissingSolarEvent` naming the event and the day. The book returns
`bogus` there; a length of daylight that is not there is not zero, and no
number is returned for it.

All of these are in Universal Time, and use the rise-and-set geometry of
`hc-astro::riseset`: the depressions are of the Sun's centre below the
geometric horizon with no refraction, as the book takes them, while
sunrise and sunset are the refracted upper limb on the visible horizon of
the default `geometric-dip` horizon, which differs from the book's by
19″·√h of the observer's height; the horizons are in
[rise-and-set.md](rise-and-set.md).

In `hc-format::east_african_hours`:

- `Reckoning`, a table with `ETHIOPIAN` (`ethiopian-hours`) and `SWAHILI`
  (`swahili-hours`); `reading` from a civil time of day to an
  `HourReading`, the dial's hour 1 to 12, the civil minute, second and
  fraction, and the `Half`; `civil`, its inverse; and `period`, the
  Swahili part of the day by civil hour. The civil clock is the caller's:
  East Africa Time in both places, but the module reads any wall clock.
- **Not carried**: the Amharic names of the parts of the day. The one
  teaching source read [uw-lctl-amharic-telling-time] divides the day into
  "Tewat" to noon and "Ke se at behuwala" after it, and the night into
  "Mata" to midnight and "Lelit" after it, with noon as "Tewat sidist";
  pages seen only in a search summary put noon in the day, *ken*, instead,
  and no authority was found to settle it. Nor a reckoning from each day's
  actual sunrise, which descriptions that say "dawn" might suggest: no
  source read defines one, and it would be a third convention.

## Accuracy

The relations are exact definitions; the accuracy is the equation of
time's. Against Meeus's example 28.a, +13 min 42.6 s on 1992 October 13.0
TD, the sundial at Greenwich is within 0.5 s. At Padua on seven days of
2024, the sundial reads 12:00 at the Sun's transit as `riseset::solar_noon`
finds it to within 2 s, which is the tolerance of the transit search: the
equation of time is the hour angle the search solves for, so the
agreement holds at every date. Against JPL Horizons' apparent hour angle
of the Sun at Greenwich [jpl-horizons] the equation of time is within
0.02 s on 1 January 2000 and 3 November 2024, with Horizons' UTC made UT1
by the IERS's UT1 − UTC [iers-eopc04], and within 0.6 s on 30 July
587 BCE, where the two ΔT models are 185 s apart; the mean Sun at
dynamical time misses the same days by 0.19 s, 0.18 s and 38 s. The
book's `equation-of-time` takes its mean Sun at dynamical time, so its
sundial runs 38 s ahead of the transit at Jerusalem in 586 BCE, when ΔT
was five hours, and its solar events carry that lead; how it was
measured is in [rise-and-set.md](rise-and-set.md). The inverses return
the reading they were given to 0.1 ms.

The Edo hours: at Kyoto on 20 March and 22 September 2020, the Observatory's
夜明 falls 35 min 57.6 s and 36 min 1.6 s before the Sun's centre reaches
the geometric horizon, where こよみのページ computes 35 min 56 s and 36 min
0 s, and at 5:28:54 and 5:13:01 JST where it gives 5:28:47 and 5:12:54
[koyomi8-yoake-higure]; the page names neither the year, taken as 2020,
when it was written and when the equinoxes fell on those days, nor its
point in Kyoto, and a point 26″ of longitude east of the 改暦所 used here
accounts for the 7 s. At the 2024 solstice at Kyoto a daytime hour is
2 h 37.8 min and a night hour 1 h 22.3 min, against the "about 2 h 39 min"
and "about 1 h 21 min" of a source that names no place
[astro-dic-futeijiho]. The formula of rule 2 gives 7°21′41″ to the
arcsecond, as the Observatory's page does.

The temporal hours are as good as the sunrise and sunset under them, which
are within a minute of NAOJ's; Tokyo's daytime hour on 1 January 2024 is
within 0.17 min of the 49.0 min that NAOJ's published minutes give. The
Italian zero hour and the religious times put the Sun's centre at the
stated altitude to 10⁻⁴ degree, and ʿaṣr's shadow at its stated length to
10⁻³ of the object's height. No published table of Italian hours or of
the ʿaṣr and the book's two Jewish evening times was read, so beyond
NAOJ's sunrise and sunset they are checked against their own definitions,
not against a published value.

The Jewish times in temporal hours, dawn at 16.1° and 72 minutes and
nightfall at 8.5° and 72 minutes are checked against Hebcal's published
zmanim [hebcal-zmanim-api] for New York City on 1 January 2025, Jerusalem
on 21 June 2025 and London on 20 March 2025: all sixteen times on each day,
48 in all, are within 0.48 minutes of the printed minute. Hebcal prints no
16.1° dawn for London on 21 June 2025, and neither does this library.

The six-hour reckonings are exact: `reading` and `civil` invert each other
at every minute of the day and at the leap second, and the examples of
both sources are tests.

## Sources

- [reingold2018code] — `local-from-universal`, `universal-from-local`,
  `apparent-from-local`, `local-from-apparent`, `apparent-from-universal`,
  `universal-from-apparent`, `equation-of-time` and the location of Padua,
  read in `calendar.l` on 2026-09-26.
- [reingold2018] — the book those functions come from; chapter 14 not
  read here.
- [meeus1998] — the equation of time, (28.1), and example 28.a. Not read
  for this document; the values are as `hc-astro::solar` cites them.
- [jpl-horizons] — the Sun's apparent hour angle at Greenwich on
  30 July 587 BCE, 1 January 1000, 1 January 2000 and 3 November 2024,
  and Horizons' TDB − UT for each. Read 2026-09-27.
- [iers-eopc04] — UT1 − UTC on 1 January 2000 and 3 November 2024, in
  the copy of the series retrieved 2026-09-25.
- [reingold2018code], again — `daytime-temporal-hour`,
  `nighttime-temporal-hour`, `standard-from-sundial`, `local-zero-hour`,
  `italian-from-local`, `local-from-italian`, `padua`, `asr`, `alt-asr`,
  `jewish-dusk`, `jewish-sabbath-ends` and `dusk`, read on 2026-09-26.
- [nao-koyomi-dni-tokyo-2024] — Tokyo's sunrise and sunset on
  1 January 2024, the anchor of the temporal hour. Read 2026-09-26.
- [wikipedia-zmanim] — the two reckonings of the temporal hour, the Vilna
  Gaon's and the Magen Avraham's. Read 2026-09-27.
- [kosherjava-zmanim] — the hours of each time, the GRA's and the MGA's
  72-minute hours, dawn at 16.1° and 72 minutes, nightfall at 8.5° and 72
  minutes, with the authorities they attribute them to. Read 2026-09-27.
  The authorities' own texts (the Vilna Gaon, the Magen Avraham, Rabbi
  Meir Posen's *Ohr Meir*, Rabbeinu Tam) were not read.
- [hebcal-zmanim-api] — the same definitions in Hebcal's `Zmanim`, the
  MGA day at 16.1°, and the published times of the three places and days
  above. Read 2026-09-27.
- [nao-rekiwiki-yoake] — 暦Wiki, 「夜明と日暮」: the three rules for 明け六つ
  and 暮れ六つ, the formula, the 寛政暦書's latitude and its 7度36分, the
  almanac wording of 1912 and the 35°0.8′ of Ōtani's 『伊能忠敬』 (1917).
  Read 2026-09-27; the 寛政暦書, Ōtani and Watanabe's 『近世日本天文学史』
  not read.
- [nao-rekiwiki-futeiji] — 暦Wiki, 「定時法と不定時法」: six hours each to
  day and night, the hour names with their prefixes, the 天保暦's tenths,
  明け六つ as the start of the day, clocks that showed the unequal hours.
  Read 2026-09-27.
- [astro-dic-futeijiho] — 天文学辞典, 「不定時法」: the midsummer lengths of
  a day and a night hour. Read 2026-09-27.
- [wikipedia-ja-jikoku] — Wikipedia (ja), 「時刻」: the pairing of the bell
  hours with the branches. Read 2026-09-27.
- [koyomi8-yoake-higure] — こよみのページ, 「理科年表の「夜明」と「日暮」の
  角度」 (2020-02-12) and its 補稿 (2020-02-16): the 理科年表's wording, and
  the Kyoto equinox times. Read 2026-09-27.
- [shulchan-arukh-oc-89] — *Orach Chayim* 89:1, the time of the morning
  prayer, in Hebrew and in the Sefaria Community Translation. Read
  2026-09-27.
- [mishnah-berurah-58] — 58:4, the Magen Avraham's and the Vilna Gaon's
  starting points for the hours. Read in Hebrew 2026-09-27.
- [wikipedia-time-in-ethiopia] — the Ethiopian day and night halves and the
  six-hour difference. Read 2026-09-27.
- [undp-eue-ethiopian-time] — the Ethiopian hours' examples, from the UN
  Women's Association's guide as the UNDP Emergencies Unit for Ethiopia
  published it. The live page refuses the request; read in the Internet
  Archive's copy of 22 July 2011, on 2026-09-27.
- [ku-kiswahili-lesson-17] — the Swahili hours, their parts of the day,
  noon and midnight. Read 2026-09-27.
- [uw-lctl-amharic-telling-time] — the Amharic parts of the day, for the
  disagreement that keeps them out. Read 2026-09-27.

## Code

`crates/hc-astro/src/solar_time.rs`. The tests that anchor it:
`padua_keeps_greenwich_time_plus_forty_seven_and_a_half_minutes`,
`a_greenwich_sundial_runs_ahead_by_meeus_example_28a`,
`the_sundial_reads_noon_at_the_suns_transit`,
`apparent_and_mean_time_invert_each_other`,
`a_tokyo_new_years_temporal_hour_matches_the_national_ephemeris`,
`the_end_of_the_morning_in_jerusalem_matches_hebcal`,
`the_end_of_the_morning_is_four_temporal_hours_after_sunrise`,
`temporal_hours_six_and_eighteen_are_sunrise_and_sunset`,
`temporal_time_inverts_its_universal_time`,
`temporal_hours_are_refused_under_the_midnight_sun_and_the_polar_night`,
`the_italian_zero_hour_is_half_an_hour_after_the_suns_limb_meets_the_horizon`,
`italian_hours_count_a_day_from_one_zero_hour_to_the_next`,
`asr_is_where_the_shadow_rule_puts_it`,
`the_jewish_evening_times_sit_at_their_angles_in_order`,
`the_zmanim_fall_where_hebcal_prints_them`,
`the_mga_day_is_the_gra_day_and_two_twilights`,
`there_is_no_sixteen_degree_dawn_in_a_london_june`,
`the_kansei_depression_is_nine_degrees_of_hour_angle_after_an_equinox_sunset_in_kyoto`,
`kyoto_dawn_at_the_equinoxes_is_two_and_a_half_koku_before_the_centre_rises`,
`a_midsummer_edo_hour_is_about_two_hours_thirty_nine_minutes`,
`the_edo_hours_run_from_dawn_through_noon_and_midnight`,
`the_edo_hours_are_named_by_their_strokes_and_branches`,
`edo_time_inverts_its_universal_time` and
`the_edo_hours_are_refused_on_a_white_night`.
`asr_is_where_the_shadow_rule_puts_it` and
`the_jewish_evening_times_sit_at_their_angles_in_order`.

`crates/hc-format/src/east_african_hours.rs`, anchored by
`the_ethiopian_examples`, `the_swahili_examples`,
`the_reckonings_differ_only_at_the_twelfth_hours` and
`every_minute_round_trips`.
