# The Hindu calendars: amānta and pūrṇimānta months, the solar months, the nakṣatras, ayanāṃśa

Backs the identifiers `hindu-lunar`, `hindu-lunar-purnimanta`,
`hindu-solar-tamil`, `hindu-solar-malayalam`, `hindu-solar-bengali`,
`hindu-solar-vikrami`, `hindu-old-solar` and `hindu-old-lunar` in
`hc-calendars-indic`, and the crate's `tithi`, `nakshatra`,
`surya_siddhanta` and `places` modules. Nepal's Bikram Sambat and Nepal
Sambat are in the same crate and have their own,
[nepal-calendars.md](nepal-calendars.md).

## What it is

**The family.** India keeps two kinds of calendar on one sidereal zodiac.
The *lunisolar* calendar counts months from new moon to new moon and days
by the *tithi*, the lunar day; it is the calendar the festivals are dated
in, and it comes in two namings, *amānta* (the month ends at the new moon)
in the south and west and *pūrṇimānta* (the month ends at the full moon) in
the north. The *solar* calendars count a month as the Sun's stay in one of
the twelve signs of the sidereal zodiac, the *rāśis*, and each region has
its own rule for which civil day a month begins on when the Sun's entry —
the *saṅkrānti* — falls in the middle of one: Tamil Nadu, Kerala, Bengal
and Assam, and the Vikrami reckoning of Punjab, Haryana and Odisha are the
four this library carries. Both kinds hang on the *ayanāṃśa*, the angle by
which the sidereal zero point stands from the March equinox, which fixes
where every sign and every *nakṣatra* begins. The nakṣatras, twenty-seven
equal arcs of the same zodiac, are the third thing the almanacs print for
every day, beside the tithi.

**Who publishes it.** Every region has its almanacs — the *pañcāṅgas* —
and the Calendar Reform Committee's report lists some sixty of them, from
the *Chitrasala Panchang* in Marathi to the *Drigganitha Panchangam* in
Tamil, each computed by its own pandits from its own astronomical text
[crc1955]. Over them since 1957 stands the *Rashtriya Panchang*, the
national almanac, published by the Positional Astronomy Centre of the
India Meteorological Department in Kolkata from Śaka 1879 (1957–58) on, in
Hindi, English, Assamese, Bengali, Gujarati, Kannada, Malayalam, Marathi,
Oriya, Punjabi, Sanskrit, Tamil, Telugu and Urdu [pac-rashtriya-panchang].
It prints the tithi and nakṣatra of every day, the regional solar
calendars side by side in its "Regional Calendars" tables, the ayanāṃśa at
the head of each month and the festivals of the year, and its yearly
tables are the reference this library's tests compare against
[rashtriya-panchang-1945, rashtriya-panchang-1946]. The same Centre
publishes the *Indian Astronomical Ephemeris*, whose Part VI is the Indian
calendar and whose readers include the almanac makers
[imd-astronomical-ephemeris, pac-history].

**The Calendar Reform Committee.** The Council of Scientific and
Industrial Research appointed the Committee in November 1952 under
Meghnad Saha, with N. C. Lahiri as secretary; it reported on 10 November
1955 [crc1955]. It proposed two calendars. The civil one, the national
calendar of the Śaka era with months of 30 and 31 days beginning the day
after the vernal equinox, is in `hc-calendars-solar` as `indian` and is
not this document's subject; it came into use on 1 Chaitra 1879, 22 March
1957 [wikipedia-indian-national-calendar]. The religious one is what
matters here, because the Committee's decisions on it are the
*Rashtriya Panchang*'s rules:

- tithis and nakṣatras are to be computed by modern astronomy, "which
  would naturally agree with Nautical Almanacs", and "the tithi current
  for the Central Station (82½° E. Long, and 23° 11′ N. Lat.) should be
  the tithi for the whole of India" [crc1955, p. 4];
- the lunar months "commence from the moment of new-moon" and are named
  after the solar month the new moon falls in, a second new moon in one
  solar month opening the *adhika* or *mala* month and the next the
  *śuddha* one [crc1955, p. 8];
- the nakṣatra divisions are 13°20′ each, and both the Moon's exit from
  one and the Sun's entry into one are to be printed, with a variable
  ayanāṃśa "fixed with respect to the stars", of 23°15′0″ on 21 March 1956
  and growing at the mean rate of about 50″.27 a year [crc1955, p. 8];
- the day a solar month begins on is left alone: "different states, viz.,
  Bengal, Orissa, Tamil Nad use different conventions for defining the
  day of the solar saṃkrānti … let the orthodox Bengalee, Oriya, or Tamil
  Pandit argue it out amongst themselves" [crc1955, pp. 3–4].

The Central Station is the point on the meridian of Indian Standard Time,
82°30′ E, at the latitude of Ujjain; its local mean time is IST to the
second, since 82.5° is exactly five and a half hours. The classical
almanacs, and Reingold and Dershowitz after them, read the day at Ujjain
itself, the prime meridian of the Siddhāntas [reingold2018code, `ujjain`].

## How it works

**The tithi.** A tithi is the time the Moon takes to gain 12° of
longitude on the Sun: a thirtieth of the synodic month, from about
20 to 27 hours as the Moon's speed varies [wikipedia-tithi]. Thirty make
a lunar month: śukla 1 to 15 from the new moon to the full, kṛṣṇa 1 to 15
from the full moon to the next new, the last being *amāvāsyā* itself; the
library numbers them 1 to 30 straight through. A civil day carries the
tithi in progress at its sunrise. A tithi that begins after one sunrise
and ends before the next holds no sunrise and is *kṣaya*, expunged; one
that holds two sunrises is *adhika* or *vṛddhi*, repeated. Sewell and
Dikshit count "generally thirteen expunctions and seven repetitions" of
tithis in a year [sewell1896, Art. 32]; the library carries both, as a
date that names its tithi and whether the day is the second to carry it.

**Whose sunrise.** A tithi is one instant for the whole Earth, but a tithi
that ends within an hour of sunrise belongs to different days in Delhi and
Chennai. The registered calendars read the day at the Central Station's
sunrise, as the *Rashtriya Panchang* does; the classical reference, and
the calendar `HinduLunarCalendar::UJJAIN`, read it at Ujjain; a caller
with a local almanac can build the calendar for any place. The
conjunction a month begins after is the instant the Moon's elongation from
the Sun returns to zero, the same quantity the tithi is counted in, and
not the tabulated new moon of another series that can differ from it by
minutes: a month has to begin where its first tithi does.

**The amānta month and its name.** A month runs from conjunction to
conjunction and begins with the first day whose sunrise follows the
conjunction. It is named for the saṅkrānti that falls in it: the lunar
month in which the Sun enters Meṣa is Chaitra, the one with the Vṛṣabha
saṅkrānti is Vaiśākha, and so on round the twelve —
Chaitra, Vaiśākha, Jyeṣṭha, Āṣāḍha, Śrāvaṇa, Bhādrapada, Āśvina,
Kārtika, Mārgaśīrṣa, Pauṣa, Māgha, Phālguna [wikipedia-hindu-calendar].
The Committee's form of the rule, the month named after the solar month
its opening new moon falls in, is the same rule: a month whose new moon
falls while the Sun is in Meṣa is the month in which the Sun will enter
Vṛṣabha, and in the Bengal naming the solar month of Meṣa is itself called
Vaiśākha [crc1955, p. 8; sewell1896, Art. 48].

**Adhika and kṣaya months.** Twelve lunar months are about eleven days
short of the solar year, so about once in 32½ months a lunar month holds
no saṅkrānti at all [wikipedia-adhik-maas]. That month is *adhika*, the
intercalary one, and takes the name of the month that follows it, which
is then *nija* or *śuddha*; the *Rashtriya Panchang* prints "Sravana
(Mala)" and "Sravana (Suddha)". The library writes the adhika month as
`Month::leap(n)`, before the ordinary month of the same number. The
opposite case, a lunar month with two saṅkrāntis, is a *kṣaya* month: it
keeps the name of the first saṅkrānti and the name the second would have
given is dropped — "Margasirsha is the name of the month in which the
Dhanus sankranti occurs; the name Pausha is therefore expunged"
[sewell1896, Art. 48]. It can only happen where the Sun's stay in a sign
is shortest, near perihelion in the northern winter, and in Sewell and
Dikshit's tables of 300 to 1900 CE the months Mārgaśīrṣa, Pauṣa and Māgha
are never intercalary and the intervals between expunged months are 19,
46, 65, 76, 122 or 141 years [sewell1896, Art. 50]; the module states the
rarity in those terms.

**The year.** The year is the Śaka era and turns at Chaitra śukla 1, in
March or April; a Gregorian year *g* holds the turn of Śaka *g* − 78, so
that a date from January to March is in Śaka *g* − 79
[wikipedia-indian-national-calendar]. The Vikrama year of the same month
is 135 greater, its epoch being 57 BCE [wikipedia-vikram-samvat], and is
carried as the extra field `vikrama-year`. The library places a month in
its year by a rule of its own: Chaitra, ordinary or intercalary, opens
the year, and a month of any other name that begins between January and
March is the tail of the year before.

**The pūrṇimānta naming.** Northern India keeps the same tithis and the
same year and ends the month at the full moon, so the dark fortnight comes
*first* and carries the name of the bright fortnight that follows it: the
dark half that amānta calls Chaitra kṛṣṇa is pūrṇimānta Vaiśākha kṛṣṇa,
and Janmāṣṭamī is Śrāvaṇa kṛṣṇa 8 in the south and Bhādrapada kṛṣṇa 8 in
the north, on the same night. The intercalary month is the exception: it
is inserted whole, bright half first, between the two halves of the
ordinary month — "first half of Chaitra, then Adhika Chaitra, then second
half of Chaitra" [wikipedia-adhik-maas]. That is how the almanac's *vadi*
column labels Śaka 1945: Śrāvaṇa vadi from 4 July 2023, adhika Śrāvaṇa
sudi from the 18th and vadi from 2 August, nija Śrāvaṇa sudi from
17 August [rashtriya-panchang-1945].

**The solar months and the four rules.** The Sun's stay in a sidereal
sign is 29 to 32 days, longest near aphelion in July and shortest near
perihelion in January, so a solar month has no fixed length [crc1955,
p. 3]. Every reckoning agrees on the instant of the saṅkrānti; they differ
on the civil day. Sewell and Dikshit state the four conventions and name
them [sewell1896, Art. 28], and the library carries each as a
`SankrantiRule` that it read off twenty-four months of the almanac's
tables for each region rather than off a description:

| Sewell and Dikshit's rule | Their statement | Carried as | Registered |
| --- | --- | --- | --- |
| Tamil | "when a sankranti takes place after sunrise and before sunset the month begins on the same day, while if it takes place after sunset the month begins on the following day" | `BeforeSunset` | `hindu-solar-tamil` |
| Malabar | "the day between sunrise and sunset being divided into five parts, if a sankranti takes place within the first three of them the month begins on the same day, otherwise it begins on the following day" | `BeforeAfternoon`, three fifths of the daylight, the *aparāhṇa* | `hindu-solar-malayalam` |
| Bengal | "when a sankranti takes place between sunrise and midnight of a civil day the solar month begins on the following day; and when it occurs after midnight the month begins on the next following, or third, day" | `DayAfter`, the civil day after the saṅkrānti's | `hindu-solar-bengali` |
| Orissa | "the solar month … begins civilly on the same day as the sankranti, whether this takes place before midnight or not" | `SunriseDay`, the sunrise-to-sunrise day the saṅkrānti falls in, named by its date at sunrise | `hindu-solar-vikrami` |

The Bengal rule reads as two cases because Sewell and Dikshit count the
day from sunrise to sunrise: a saṅkrānti after midnight is still in the
previous day's night, and "the third day" from that day is the day after
the midnight-to-midnight day the saṅkrānti fell in. Counted in civil
days, both cases are the day after, which is what the almanac's Bengal
column shows and what the library does. The Orissa rule, on the same
sunrise-to-sunrise day, is the library's `SunriseDay`; the almanac
prints one column for Punjab, Haryana and Odisha, and the library
registers it under the Vikrami name. Each reckoning has its own month
names and year: Tamil Chithirai to Panguni from the Meṣa saṅkrānti in the
Tiruvaḷḷuvar year; Malayalam Chingam to Karkadakam from the Siṃha
saṅkrānti in August in the Kollam era, whose epoch is 825 CE
[wikipedia-malayalam-calendar]; Bengali Boishakh to Choitro from Meṣa in
the Bengali San, 593 less than the Gregorian year from Pohela Boishakh
on [wikipedia-bengali-calendars]; and the Vikrami solar months, the
Sanskrit lunar names Vaiśākha to Chaitra applied to the solar months from
Meṣa, in the Vikrama Saṃvat, 57 more than the Gregorian year from
Vaiśākha on [wikipedia-vikram-samvat]. The month names are `hc-seasons`'s
solar-month traditions.

**True Sun and mean Sun.** The registered solar calendars take their
saṅkrāntis from the true Sun of modern astronomy — `hc-astro`'s VSOP87
series, good to about 1″ — in the sidereal zodiac of an ayanāṃśa, which is
what the *Rashtriya Panchang* does. The traditional almanacs that still
compute by the *Sūrya Siddhānta* take them from that text's Sun instead,
and `SolarModel::SuryaSiddhanta` carries it: a mean Sun moving uniformly
round a sidereal year of 1 577 917 828⁄4 320 000 days (365.258 756),
corrected by an epicycle of 14⁄360 of the deferent that shrinks by 1⁄42
as the anomaly grows, the anomaly turning once in an anomalistic year of
1 577 917 828 000⁄4 319 999 613 days, all read off a sine table of
twenty-four entries at steps of 225′ in a radius of 3438′ [reingold2018code,
`hindu-sidereal-year`, `hindu-anomalistic-year`, `hindu-true-position`].
The table is Āryabhaṭa's — 225, 449, 671, 890, 1105, 1315, 1520, 1719,
1910, 2093, 2267, 2431, 2585, 2728, 2859, 2978, 3084, 3177, 3256, 3321,
3372, 3409, 3431, 3438 [wikipedia-aryabhata-sine-table] — and Reingold
and Dershowitz reproduce its rounding with a correction of 0.215 whose
sign flips at 1716 [reingold2018code, `hindu-sine-table`]; the library
holds the twenty-four values in a test. This zodiac is sidereal by
construction and has no ayanāṃśa, and its zero point is not any modern
one's: the Meṣa saṅkrānti of 2024 by the Siddhānta falls 139 minutes after
the Lahiri one. The *mean* positions are counted from the Kali Yuga epoch,
Friday 18 February 3102 BCE (Julian), the fixed day −1 132 959
[reingold2018code, `hindu-epoch`], where the book counts from a creation
1 955 880 000 sidereal years earlier; the two are the same to the
precision floating point keeps, which is the module's reason for the
change.

**Ayanāṃśa.** The sidereal zero point is a convention, fixed by one
number at one epoch and carried forward and back by precession, about
50″ a year. Lahiri's, also called Chitrapakṣa, puts the star Chitrā
(Spica) at sidereal 180°; the Committee adopted it as 23°15′0″ on
21 March 1956 [crc1955, p. 8], and one of the opinions appended to its
report notes that nearly sixty almanacs were already following it, with
an ayanāṃśa of nearly 23°12′ on 21 March 1954 [crc1955]. `hc-seasons`
carries four,
anchored as the Swiss Ephemeris anchors them [swisseph]: Lahiri at
22.460 148° on JD 2 415 020 (1900), Raman at 21.010 833° and Krishnamurti
at 21.978 333° on the same day, and Fagan–Bradley at 24.042 044° on
JD 2 433 282.5 (1950), each carried by the IAU 2006 precession. Reingold
and Dershowitz define their own, zero at the Meṣa saṅkrānti of 285 CE
[reingold2018code, `sidereal-start`]. Published values of a named
ayanāṃśa differ by a few tens of arcseconds, and 20″ of solar longitude
is about eight minutes of the Sun's motion, which is why the nakṣatra
transits in the accuracy section stand a fixed eight to ten minutes from
Drik Panchang's. The *Rashtriya Panchang* prints its value at the head of
the year: "Ayanamsa on 1st Chaitra" 24°11′39″ for Śaka 1946 (22 March
2024) and 24°12′35″ for 1947 [rashtriya-panchang-1946]. The Committee's
figure carried forward by hand — 23°15′0″ plus 68 years of 50″.27, that
is 56′58″ — gives 24°11′58″ for 2024, within 20″ of the printed value;
the library's Lahiri is within 10″ of it.

**The nakṣatras.** The sidereal ecliptic is cut into twenty-seven arcs of
13°20′ from the same zero point, Aśvinī first — the point opposite
Spica — and Revatī last [wikipedia-nakshatra]: Aśvinī, Bharaṇī, Kṛttikā,
Rohiṇī, Mṛgaśīrṣa, Ārdrā, Punarvasu, Puṣya, Āśleṣā, Maghā,
Pūrva Phalgunī, Uttara Phalgunī, Hasta, Citrā, Svātī, Viśākhā,
Anurādhā, Jyeṣṭhā, Mūla, Pūrvāṣāḍhā, Uttarāṣāḍhā, Śravaṇa, Dhaniṣṭhā,
Śatabhiṣā, Pūrva Bhādrapadā, Uttara Bhādrapadā, Revatī. The Moon crosses
one in about a day, so a nakṣatra, like a tithi, is held at one or two
sunrises or now and then at none; the Sun takes about thirteen and a
half days, a little more near aphelion, and the almanacs print its
entries as the Sun's nakṣatra transits. The Committee asked for both
[crc1955, p. 8]. Some festivals are fixed by the Moon's nakṣatra rather
than by a tithi: Thaipusam falls on Puṣya (Tamil Pusam) in the month of
Thai [wikipedia-thaipusam] and Onam on Śravaṇa (Malayalam Thiruvonam) in
Chingam [wikipedia-onam]. Kerala's farming calendar counts by the Sun's
nakṣatra: a ñāṭṭuvēla is the Sun's stay in one, twenty-seven to the year of
thirteen to fourteen days each, and the Thiruvathira ñāṭṭuvēla, the Sun in
Ārdrā at the monsoon's height, is the one the farmers hold the best
[wikipedia-ml-njattuvela]. The module once also named "the rain nakṣatras
of the Deccan"; no source for that was found, and it no longer does.

**The Old Hindu calendars.** Before the true positions, the almanacs
reckoned by mean motion, and the library carries the mean solar and
lunisolar calendars of the *Ārya Siddhānta* as Reingold and Dershowitz
state them [reingold2018code, `old-hindu-solar-from-fixed`,
`old-hindu-lunar-from-fixed`, `old-hindu-lunar-leap-year?`]: a sidereal
year of 1 577 917 500⁄4 320 000 days (365.258 68), a synodic month of
1 577 917 500⁄53 433 336 days, both from the Kali Yuga epoch, the day
beginning at mean sunrise a quarter day after midnight. The solar month
is a twelfth of the year and begins on the day the mean Sun enters it;
the lunar month is named for the solar month that begins within it, a
lunar month in which none begins taking the name of the month after it
as the intercalary one; the tithi is a thirtieth of the mean month, so
one is skipped now and then and none is ever repeated. The year is
0.002 3 days longer than the modern mean sidereal year of 365.256 36
days — a textbook constant the module names no source for — so the
calendar has gained some twelve days on the stars since the epoch, but
its zero point is not the modern one either and the two errors largely
cancel against the true saṅkrāntis.

**Worked example: the adhika Śrāvaṇa of Śaka 1945 (2023).** The
*Rashtriya Panchang*'s tables for 1945 give the following days
[rashtriya-panchang-1945]. Lunar: Āṣāḍha śukla 1 on 19 June, "Sravana
(Mala)" śukla 1 on 18 July, "Sravana (Suddha)" śukla 1 on 17 August,
Bhādrapada śukla 1 on 16 September. Solar, for the Karka, Siṃha and
Kanyā saṅkrāntis of July, August and September: Tamil Aadi, Aavani and
Purattasi from 17 July, 17 August and 17 September; the Punjab and Odisha
column's Śrāvaṇa, Bhādra and Āśvina from 16 July, 17 August and
17 September; Bengal's Shrabon, Bhadro and Ashwin from 18 July,
18 August and 18 September.

First place the saṅkrāntis from the three solar columns. The Karka
saṅkrānti began the Tamil month on 17 July, so it fell on the 17th before
sunset or on the 16th after sunset; it began Bengal's month on the 18th,
so its civil day was the 17th; it began the Vikrami month on the 16th,
so it fell in the sunrise-to-sunrise day of the 16th, that is before the
sunrise of the 17th. Only the small hours of 17 July satisfy all three.
The Siṃha saṅkrānti began the Tamil and Vikrami months on 17 August and
Bengal's on the 18th: the daytime of 17 August. The Kanyā saṅkrānti, by
the same three columns, the daytime of 17 September.

Now the lunar months, from the rule that a month's first day is the
first whose sunrise follows the conjunction. The month that opens on
18 July begins after a conjunction between the sunrises of 17 and
18 July, which is after the Karka saṅkrānti of the small hours of the
17th; it ends with the conjunction between the sunrises of 16 and
17 August, which is before the Siṃha saṅkrānti of the 17th's daytime. It
holds no saṅkrānti, so it is adhika. The month after it, 17 August to
15 September, holds the Siṃha saṅkrānti and is Śrāvaṇa, so the
intercalary month is adhika Śrāvaṇa and the month it precedes is nija
Śrāvaṇa. The month before, 19 June to 17 July, ends with the conjunction
after the Karka saṅkrānti and so holds it, and is Āṣāḍha. The year has
thirteen months and 384 days, from 22 March 2023 to 8 April 2024; the
tests
`saka_1945_has_the_intercalary_sravana_and_1946_none` and
`the_year_opens_at_chaitra_sukla_pratipada` hold these figures, and the
almanac's *vadi* column, above, shows how the north labels the same
weeks.

## What is carried

- **`hindu-lunar`**, the amānta calendar, as `HinduLunarCalendar`, with
  the month as `Month { ordinal, leap }` — adhika Śrāvaṇa is
  `Month::leap(5)` — the tithi 1 to 30 as the day, and `leap_day` for the
  second day to carry a tithi. `RASHTRIYA`, the registered one, reads the
  day at the Central Station's sunrise with the Lahiri ayanāṃśa; `UJJAIN`
  reads it at Ujjain; `new` takes any place and any ayanāṃśa. The era is
  `Saka`, and `vikrama-year` is an extra field. `new_year`,
  `leap_month_of`, `has_kshaya_month`, `month_span` and `days_in_year`
  answer the questions a festival rule asks. A kṣaya month is reported as
  the name not existing, `MonthOutOfRange`; a skipped tithi as
  `DayOutOfRange`.
- **`hindu-lunar-purnimanta`**, as `HinduPurnimantaCalendar`, a renaming
  of the amānta calendar's months day for day: every conversion goes
  through the amānta calendar, so the two can never disagree about a
  tithi, only about the month's name. The adhika month keeps its own
  name in both.
- **The four solar calendars**, as `HinduSolarCalendar` values `TAMIL`,
  `MALAYALAM`, `BENGALI` and `VIKRAMI`, each a month-name tradition from
  `hc-seasons`, a `SankrantiRule`, an era and offset, a place and a
  `SolarModel`. The eras are `Tiruvalluvar` (Gregorian year plus 31),
  `Kollam` (the Gregorian year of Chingam less 824), `Bangabda` (less
  593) and `VS` (plus 57). `new` gives the same reckoning at another
  place or with the *Sūrya Siddhānta*'s Sun, under an identifier of the
  caller's. A fifth rule, `CivilDay`, the midnight-to-midnight day of the
  saṅkrānti, exists for the Bikram Sambat and is that document's.
- **`hindu-old-solar`** and **`hindu-old-lunar`**, as
  `OldHinduSolarCalendar` and `OldHinduLunarCalendar`, in the Kali Yuga
  era, arithmetic and not astronomical.
- **`tithi`**: `tithi_at`, `tithi_of_day`, `paksha_of`, `sunrise_of` and
  `sunset_of`, and `Prevalence` — sunrise, midday, afternoon (seven
  tenths of the daylight), evening (an hour after sunset), midnight —
  the part of the day a festival's tithi must hold, which `hc-holiday`
  uses and this document does not cover.
- **`nakshatra`**: the twenty-seven as constants `ASHVINI` to `REVATI`,
  `nakshatra_at` and `nakshatra_span` for the Moon, `solar_nakshatra_at`,
  `solar_nakshatra_ingress_after` and `solar_nakshatra_span` for the Sun,
  all under any ayanāṃśa.
- **`surya_siddhanta`**: `solar_longitude`, `sign_at` and
  `ingress_after` for the Siddhānta's Sun, read at Ujjain's meridian,
  75°46′6″ E as the book gives it.
- **`places`**: `CENTRAL_STATION` (23.183 333° N, 82.5° E), `UJJAIN`,
  `NEW_DELHI`, `KATHMANDU`, all at sea level so that sunrise is the
  almanac's. `UJJAIN` is the book's `ujjain`, 23°9′ N, 75°46′6″ E
  [reingold2018code], the longitude `surya_siddhanta` uses. Until
  2026-09-25 the constant carried the city's modern coordinates,
  23.1765° N, 75.7885° E, under the same citation; the two differ by about
  a minute of arc, five seconds of sunrise, and over 1700–2299 the change
  moves the sunrise tithi at Ujjain on sixteen days — 17 June 1770,
  22 February 1802, 17 January 1831, 20 May 1859, 26 August 1867,
  7 January 1995, 23 September 2089, 15 October 2097 and eight days after
  2100 — measured for this document by comparing `tithi_of_day` at the two
  places on every day of the range. No registered calendar reads at
  Ujjain, so no test moved.
- **Range** Gregorian 1700 to 2299 for the true calendars — Śaka 1622 to
  2221 for the lunisolar ones, the era years the offsets give for the
  solar ones — "as far back as the lunar theory is worth asking", and
  refused outside; Kali Yuga 0 to 10 000 for the Old Hindu ones, whose
  lunisolar year 0 lacks the intercalary Chaitra its rule promises,
  because that month would begin before the epoch.
- **Computed, not tabulated.** Everything is computed from the sky or
  from the Siddhānta's arithmetic; the only tables are the tests'. The
  ayanāṃśa anchors and the precession series are `hc-seasons`'s.
- **Not carried, and why:**
  - *Regional almanacs' own readings*: a calendar read at another place
    or with another ayanāṃśa is a `new` away, but no local almanac's
    tables are carried, so none is registered.
  - *The Odia year counts* (the *aṅka*) and *the Tamil sixty-year names*:
    planned, and listed in `docs/calendars.md`.
  - *Festival observance*: which part of the day a tithi must hold, and
    the Smārta and Vaiṣṇava readings, are `hc-holiday`'s rules, not
    dates.
  - *The nakṣatra names in the regional languages*, and the twenty-seven
    ñāṭṭuvēla's names and farming lore: the Malayalam article was read for
    what a ñāṭṭuvēla is and no further.
  - *A Sūrya Siddhānta Moon*: only the Sun is carried, since the solar
    calendars are what the gazetted Bikram Sambat needed; the Siddhānta's
    tithis are not computed.
  - *The Kollam, Bengali and Tamil calendars of other places*: the
    registered four are all read at the Central Station, as the almanac
    computes them, and not at Chennai, Thiruvananthapuram or Kolkata.

## Accuracy

The reference is the *Rashtriya Panchang*'s own tables for Śaka 1945 and
1946 (22 March 2023 to 29 March 2025), and for the nakṣatras Drik
Panchang, an almanac that states its times to the minute. The tests
assert:

| Check | Test | Result |
| --- | --- | --- |
| The first day of every fortnight of 1945 and 1946, śukla 1 and kṛṣṇa 1 of 25 months | `hindu_lunar::every_fortnight_of_two_years_begins_where_the_rashtriya_panchang_says` | 49 of 52 on the day; the other three (31 August 2023, 22 June and 18 September 2024) begin with a tithi that holds no sunrise, and the table prints the day it begins and ends within, which the test names |
| Chaitra śukla 1 of 1945, 1946 and 1947 (22 March 2023, 9 April 2024, 30 March 2025); 384 and 355 days | `the_year_opens_at_chaitra_sukla_pratipada` | all |
| Adhika Śrāvaṇa of 1945, 18 July to 16 August 2023; none in 1946; no kṣaya month in either | `saka_1945_has_the_intercalary_sravana_and_1946_none` | all |
| Seven festivals the almanac dates by the sunrise tithi, Rāma Navamī to Holī | `festivals_the_panchang_dates_by_the_sunrise_tithi_fall_on_their_days` | 7 of 7 |
| The printed ayanāṃśa, 24°11′39″ and 24°12′35″ | `the_ayanamsa_the_panchang_prints_is_the_one_used` | within 10″ |
| A new moon after the local sunrise (11 June 2002, 05:17 IST) starts the month a day later; a conjunction within minutes of sunrise (Kathmandu, 1 October 2016) is read where the tithi is | `a_new_moon_after_the_next_local_sunrise_starts_the_month_a_day_later`, `a_month_begins_where_its_first_tithi_does` | both |
| Every day of the two years round-trips, with 10 to 40 repeated tithis | `a_sample_of_days_round_trips_including_repeated_tithis` | all |
| The pūrṇimānta name of every dark fortnight of the two years, 25 *vadi* rows | `hindu_purnimanta::every_dark_fortnight_carries_the_name_the_rashtriya_panchang_gives_it` | 25 of 25 |
| The first day of every solar month of both years, Tamil, Bengali and Vikrami | `hindu_solar::the_tamil_months_begin_where_the_rashtriya_panchang_says` and the Bengali and Vikrami tests | 24 of 24 each |
| The first day of every Malayalam month of both years | `the_malayalam_months_begin_where_the_rashtriya_panchang_says_save_medam` | 22 of 24; see below |
| Bengali San 1430, Kollam 1199, Vikrama 2080 and Tiruvaḷḷuvar 2054 open on the almanac's days | `the_eras_begin_where_the_almanac_says` | all |
| The Sun's twenty-seven nakṣatra entries of 2025 | `nakshatra::the_suns_nakshatra_transits_of_2025_are_the_almanacs` | every entry 7 to 10½ minutes before Drik Panchang's, the spread under two minutes |
| Puṣya in January 2024 and February 2025, Drik Panchang's Chennai times | `pushya_in_january_2024_begins_and_ends_when_the_almanac_says`, `pushya_in_february_2025_too` | within three minutes |
| The Siddhānta's sine table holds Āryabhaṭa's twenty-four values; its Meṣa saṅkrānti of 2024 is 139 minutes after Lahiri's | `surya_siddhanta::the_table_holds_the_classical_jyas`, `the_mesha_sankranti_of_2024_is_later_than_the_lahiri_one` | all; ±1 minute |
| The Old Hindu calendars: every one of the 3 652 952 days of the range round-trips; the epoch is Friday 18 February 3102 BCE; intercalary months come 71 in 190 years and precede their namesake; the mean months of Kali Yuga 5125 (2024–25) against the true Tamil months | `hindu_old::every_day_converts_and_converts_back` and the module's other tests, `the_mean_months_of_2024_fall_within_two_days_of_the_true_ones` | all; within two days, three days late in sum over the twelve |

**Known disagreements**, stated as the module documentation states them:

- *Kerala's Medam.* The almanac's Kerala column gives the day of the Meṣa
  saṅkrānti itself for Medam, 14 April 2023 and 13 April 2024, where the
  rule the other eleven rows of each year follow gives the day after,
  15 April 2023 and 14 April 2024 — the days Kerala keeps Vishu. The
  calendar follows the rule and the test names the two rows.
- *Drik Panchang's nakṣatra transits* are eight to ten minutes later than
  the library's throughout 2025, with a spread under two minutes across
  the year: the two Lahiri values stand about 20″ apart, the
  disagreement between published anchors of a named ayanāṃśa that
  `hc-seasons` describes, and not an error that grows through the year.
  The module infers the 20″ from the offset; Drik Panchang's page does
  not state its anchor.
- *Festivals kept on an afternoon or evening tithi.* The almanac lists
  Raksha Bandhan 2023 on 30 August and Vijayā Daśamī 2024 on 12 October,
  days on which the sunrise tithi was still the previous one. The dates
  are right; the observance is `hc-holiday`'s rule, and the test
  `a_festival_kept_on_an_afternoon_tithi_is_a_holiday_rule_not_a_date`
  holds both.
- *The Old Hindu calendars* have no published table for a modern year,
  because the almanac tabulates the true calendars and not the mean ones
  they replaced; they are held to what an arithmetic calendar must do and
  to the true months, as the table above says.

On 2026-09-25 the sources were re-read as far as they could be. The
Calendar Reform Committee's report was read in the Internet Archive's OCR
text, Sewell and Dikshit likewise, and Reingold and Dershowitz's published
code directly; the Positional Astronomy Centre's pages were read past a
certificate the fetcher could not verify, and they name the almanac's
editions and languages but serve no yearly PDF, so the *Rashtriya
Panchang* tables of Śaka 1945 and 1946 rest on the module author's reading
of 2026-09-22. Drik Panchang's 2025 transit table and its Thaipusam pages
for 2024 and 2025 give the times the tests hold.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [rashtriya-panchang-1945] | The first days of every fortnight and every regional solar month of Śaka 1945, the *vadi* column, the festival list, the adhika Śrāvaṇa; the worked example | Not re-read; the module's reading of 2026-09-22 |
| [rashtriya-panchang-1946] | The same for Śaka 1946, and the ayanāṃśa printed on 1 Chaitra of 1946 and 1947 | Not re-read; the module's reading of 2026-09-22 |
| [pac-rashtriya-panchang] | The almanac's publisher, its first year (Śaka 1879) and its languages | Yes, 2026-09-25, past an unverified certificate |
| [pac-history] | The Committee's office becoming the Nautical Almanac Unit in December 1955; the first *Indian Ephemeris and Nautical Almanac* for 1958 | Yes, 2026-09-25, past an unverified certificate |
| [imd-astronomical-ephemeris] | The *Indian Astronomical Ephemeris*, its parts and its readers | Yes, 2026-09-25 |
| [crc1955] | The Committee, its dates and members; the Central Station; tithis by modern computation; the lunar month named after the solar month of its new moon, adhika and śuddha; the 13°20′ nakṣatra divisions and the Sun's entries; the ayanāṃśa of 23°15′ on 21 March 1956; the solar-month conventions left to the pandits; the list of almanacs | Yes, 2026-09-25, in the Internet Archive's OCR text |
| [wikipedia-indian-national-calendar] | The civil calendar's adoption on 22 March 1957 and the Śaka offset | Yes, 2026-09-25 |
| [sewell1896] | The four regional rules and their names; kṣaya and adhika tithis; the naming of adhika and kṣaya months; the intervals between expunged months | Yes, 2026-09-25, in the Internet Archive's OCR text; Arts. 28, 32, 45, 48 and 50 |
| [reingold2018] | The Old Hindu calendars, the Siddhānta's Sun, the amānta rules, Ujjain | Not read directly; the published code was |
| [reingold2018code] | `hindu-epoch`, `arya-solar-year`, `arya-lunar-month`, `old-hindu-lunar-leap-year?`, `hindu-sine-table`, `hindu-sidereal-year`, `hindu-anomalistic-year`, `hindu-true-position`, `ujjain`, `sidereal-start`, `hindu-lunar-station` | Yes, 2026-09-25 |
| [calcal-modern-hindu] | The line-by-line check of the Siddhānta constants the module records | Yes, 2026-09-25: the constants of `modern_hindu.R` are the book's |
| [wikipedia-hindu-calendar] | The twelve month names in Devanagari | Yes, 2026-09-25 |
| [wikipedia-adhik-maas] | The 32½-month average, the adhika and nija names, the pūrṇimānta placing of the adhika month | Yes, 2026-09-25 |
| [wikipedia-tithi] | The 12° definition, the tithi's range of length, kṣaya and adhika tithis | Yes, 2026-09-25 |
| [wikipedia-nakshatra] | The twenty-seven arcs of 13°20′, Aśvinī opposite Spica, the list | Yes, 2026-09-25 |
| [wikipedia-vikram-samvat] | The epoch of 57 BCE and the offset of 57 | Yes, 2026-09-25 |
| [wikipedia-malayalam-calendar] | The Kollam epoch of 825 CE, Chingam as the first month, Vishu in Medam | Yes, 2026-09-25 |
| [wikipedia-bengali-calendars] | The Bengali San's offset of 593, Boishakh first, the West Bengal reckoning as sidereal | Yes, 2026-09-25 |
| [wikipedia-tamil-calendar] | The month names Chithirai to Panguni and the year at the Meṣa saṅkrānti | Yes, 2026-09-25; it does not give the Tiruvaḷḷuvar year |
| [wikipedia-aryabhata-sine-table] | The twenty-four values of the sine table | Yes, 2026-09-25 |
| [wikipedia-thaipusam] | Thaipusam on Puṣya in Thai | Yes, 2026-09-25 |
| [wikipedia-onam] | Onam on Thiruvonam in Chingam | Yes, 2026-09-25 |
| [wikipedia-ml-njattuvela] | What a ñāṭṭuvēla is; the Thiruvathira ñāṭṭuvēla | Yes, 2026-09-25 |
| [swisseph] | The ayanāṃśa anchors `hc-seasons` carries, and Lahiri as the Spica tradition | Yes, 2026-09-25 |
| [drik-sun-nakshatra-2025] | The Sun's twenty-seven nakṣatra entries of 2025 for New Delhi | Yes, 2026-09-25; the five entries checked agree with the test's table |
| [drik-thaipusam-2024], [drik-thaipusam-2025] | Puṣya's beginning and end at Chennai, 25–26 January 2024 and 10–11 February 2025 | Yes, 2026-09-25 |

Statements in the module documentation for which this document names no
source: that the Tiruvaḷḷuvar year is the Gregorian year plus 31 "as the
Tamil Nadu government's almanac counts them"; the modern mean sidereal
year of 365.256 36 days; and that Drik Panchang's Lahiri anchor is about
20″ from the library's, which is inferred from the measured offset. The
Central Station's latitude as "the latitude of Ujjain" is the Committee's
phrase; the module states the coordinates only.

## Code

`crates/hc-calendars-indic/src/hindu_lunar.rs` (`HinduLunarCalendar`,
`MIN_YEAR`, `MAX_YEAR`, `MONTHS`), `hindu_purnimanta.rs`,
`hindu_solar.rs` (`SankrantiRule`, `SolarModel`, `TAMIL`, `MALAYALAM`,
`BENGALI`, `VIKRAMI`, `ALL`), `hindu_old.rs` (`HINDU_EPOCH`,
`ARYA_SOLAR_YEAR`, `ARYA_LUNAR_MONTH`), `tithi.rs`, `nakshatra.rs`,
`surya_siddhanta.rs` (`SIDEREAL_YEAR`, `ANOMALISTIC_YEAR`,
`UJJAIN_LONGITUDE_DEGREES`) and `places.rs`. Anchors:
`every_fortnight_of_two_years_begins_where_the_rashtriya_panchang_says`,
`saka_1945_has_the_intercalary_sravana_and_1946_none`,
`every_dark_fortnight_carries_the_name_the_rashtriya_panchang_gives_it`,
`the_tamil_months_begin_where_the_rashtriya_panchang_says`,
`the_malayalam_months_begin_where_the_rashtriya_panchang_says_save_medam`,
`the_eras_begin_where_the_almanac_says`,
`the_suns_nakshatra_transits_of_2025_are_the_almanacs`,
`pushya_in_january_2024_begins_and_ends_when_the_almanac_says`,
`the_table_holds_the_classical_jyas`,
`the_mesha_sankranti_of_2024_is_later_than_the_lahiri_one`,
`the_epoch_is_friday_18_february_3102_bce_and_opens_year_zero`,
`the_mean_months_of_2024_fall_within_two_days_of_the_true_ones`. The
ayanāṃśas and the sidereal signs are `crates/hc-seasons/src/zodiac/sidereal.rs`,
the month-name traditions `rashi.rs` beside it; the festival rules that
read these calendars are `crates/hc-holiday/src/hindu.rs`.
