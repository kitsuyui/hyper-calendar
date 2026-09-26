# The Hindu calendars: amānta and pūrṇimānta months, the solar months, the nakṣatras, ayanāṃśa, the sixty year names north and south

Backs the identifiers `hindu-lunar`, `hindu-lunar-purnimanta`,
`hindu-solar-tamil`, `hindu-solar-malayalam`, `hindu-solar-bengali`,
`hindu-solar-vikrami`, `hindu-old-solar` and `hindu-old-lunar` in
`hc-calendars-indic`, and the crate's `tithi`, `nakshatra`,
`surya_siddhanta`, `samvatsara`, `barhaspatya` and `places` modules. Nepal's Bikram Sambat and Nepal
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
names and year: Tamil Chithirai to Panguni from the Meṣa saṅkrānti,
the year that Sewell and Dikshit count in the Śaka era (below);
Malayalam Chingam to Karkadakam from the Siṃha
saṅkrānti in August in the Kollam era, whose epoch is 825 CE
[wikipedia-malayalam-calendar]; Bengali Boishakh to Choitro from Meṣa in
the Bengali San, 593 less than the Gregorian year from Pohela Boishakh
on [wikipedia-bengali-calendars]; and the Vikrami solar months, the
Sanskrit lunar names Vaiśākha to Chaitra applied to the solar months from
Meṣa, in the Vikrama Saṃvat, 57 more than the Gregorian year from
Vaiśākha on [wikipedia-vikram-samvat]. The month names are `hc-seasons`'s
solar-month traditions.

**The sixty year names.** A year is also named from a list of sixty,
Prabhava to Kṣaya, "often known as the 'Brihaspati samvatsara chakra,'
the wheel or cycle of the years of Jupiter"; in the north a *saṃvatsara*
is the time Jupiter's mean motion takes through one sign, about 361.027
days by the *Sūrya Siddhānta*, so that about every 85 years two begin in
one solar year and the first of them is expunged [sewell1896, Arts. 53
and 54]. South of the Narmada the expunction was given up from Śaka 828
or 831, and there the names "are made to correspond with the luni-solar
year as well as the solar": the Tamil solar year and the Telugu and
Kannada lunisolar year that begins in it carry the same name. The rule is
"add 11 to the current Saka year, and divide by 60; the remainder is the
corresponding luni-solar cycle year", counted from Prabhava as 1, and "at
present the northern samvatsara has advanced by 12 on the southern"
[sewell1896, Art. 62]; their example of 1822 is Chitrabhanu in the south
and Vijaya in the north. The current Śaka year is the printed, expired one
plus one. So the Tamil year that opened on 14 April 2024, Śaka 1946
expired, is (1947 + 11) mod 60 = 38, Krodhin, which Tamil almanacs print
Krodhi, குரோதி [prokerala-tamil-2024]; the next, from 14 April 2025, is
the 39th, Viśvāvasu, printed Visuvasuva [pansalb-tamil-new-year-2025].
The names in Tamil script are the University of Madras's *Tamil Lexicon*,
whose entry வருஷம் lists all sixty in their three twenties, from பிரபவ to
அட்சய, and has each as a headword glossed "the Nth year of the Jupiter
cycle" [tamil-lexicon]. The lunisolar year that opens at Chaitra śukla 1
carries the same name, and the Telugu almanac prints it so: Prokerala's
Telugu calendar heads the Chaitra of Śaka 1946, from 9 April 2024,
"Krodhi Nama Samvatsaram", its Phālguna, to 29 March 2025, still Krodhi,
and the Chaitra of 1947, from 30 March 2025, "Viswavasu", and of 1948,
from 20 March 2026, "Parabhava" [prokerala-telugu-calendar].

**The northern cycle.** North of the Narmada the names keep Jupiter's
pace. A *Bārhaspatya* saṃvatsara is "the period during which the planet
Jupiter enters one sign of the zodiac and passes completely through it
with reference to his mean motion", about 361.026721 days by the
*Sūrya Siddhānta*, "about 4.232 days less than a solar year"; so each
year the next name begins 4.232 days earlier in it, and "when two
Barhaspatya samvatsaras begin during one solar year the first is said to
be expunged", which happens to a name that begins within about 4.232
days of a Meṣa saṅkrānti, the interval between expunctions being "sometimes
85 and sometimes 86 years" [sewell1896, Arts. 53 and 54]. The name
current at the beginning of a year is "in practice coupled with all the
days of that year", though inscriptions sometimes quote the one actually
current [sewell1896, Art. 55], and the year is the solar one: Sewell and
Dikshit's Table I gives the name "current at the beginning of the solar
year, i.e., at the true (or apparent) Mesha sankranti" [sewell1896,
Arts. 75 and 120]. Where lunisolar years were in use the expunction could
follow them instead — "there is evidence to show that in some places at
least, such was actually the case for a time" [sewell1896, Art. 57] — and
the different Siddhāntas, with their different years of Jupiter, expunge
different names [sewell1896, Art. 56].

Sewell and Dikshit give one procedure with each authority's numbers
[sewell1896, Art. 59]. Take the expired Kali year *K*, the expired Śaka
year plus 3179; multiply it by *m*, subtract *s* and divide by *d*. The
whole quotient, plus *K*, plus 27, divided by sixty, leaves the name
current at the apparent Meṣa saṅkrānti, counted from Prabhava as 1. The
remainder *r* of the first division gives the rest of that name's life:
(*d* − *r*) × 361 ⁄ *d* days, plus a few palas, after the saṅkrānti.

| Authority | *m* | *s* | *d* | palas added | Sewell and Dikshit's example |
| --- | --- | --- | --- | --- | --- |
| *Sūrya Siddhānta* | 211 | 108 | 18 000 | 15 | Śaka 233 expired: Raktākṣin, ending 3 d 32 gh 2.2 pa after the saṅkrānti; Krodhana, beginning within four days, expunged |
| First *Ārya Siddhānta* | 22 | 11 | 1 875 | 105 (1 gh 45 pa) | Śaka 230 expired: Durmati, ending 2 d 31 gh 55.56 pa after; Dundubhi expunged |
| *Sūrya Siddhānta* with the *bīja*, "for years after about 1500 A.D." | 117 | 60 | 10 000 | 15 | Śaka 1436 expired: Vṛṣa, ending 3 d 47 gh 40.8 pa after the saṅkrānti of 27 March 1514 (Julian), 44 gh 25 pa after mean sunrise at Ujjain; Chitrabhānu expunged |

Leaving out *s* and the palas gives the name at the mean saṅkrānti
instead, two days and a few hours after the apparent one [sewell1896,
Art. 59, notes]; the *Jyotiṣatattva* rule is the Ārya one at the mean
saṅkrānti, written for the current Śaka year. The 27 is the name current
at the Kali Yuga epoch, Vijaya, and *m* ⁄ *d* the Jovian years a solar
year holds beyond one; Sewell and Dikshit give the numbers and not their
derivation from the Siddhānta's revolutions of Jupiter, which Burgess's
translation of the *Sūrya Siddhānta* states [burgess1860, not read]. Their
Table I uses the Siddhānta without the *bīja* to A.D. 1500 and with it
after [sewell1896, Art. 58], and Art. 60 lists the names each rule
expunges from Śaka 232 to 1779, at the mean saṅkrānti; by that list the
last three the Siddhānta expunged are Yuvan in Śaka 1608 current (1685–86), Plava in
1693 (1770–71) and Vibhava in 1779 (1856–57). So in 1822 the north was
eleven names ahead of the south, Vijaya against Chitrabhānu, and from
1856–57 twelve, "at present the northern samvatsara has advanced by 12
on the southern" [sewell1896, Art. 62]. The rule expunged Manmatha in
Śaka 1864 expired (1942–43) and will expunge Durmati in 1949 (2027–28), so
the gap is thirteen from 1943 and fourteen from 2028.

**Worked example: Śaka 1946 in the north.** The Hrishikesh Panchang of
Varanasi titles its almanac for 2024–25 "श्री संवत् २०८१ शकः १९४६ पिङ्गल
नामाब्दः", Vikrama 2081, Śaka 1946, the year named Pingala
[hrishikesh-panchang-2081]. By the rule with the *bīja*: *K* = 1946 +
3179 = 5125; 117 × 5125 = 599 625, less 60 is 599 565, which divided by
10 000 is 59 remainder 9 565; 59 + 5125 + 27 = 5211, which leaves 51 on
division by sixty, and the 51st name is Pingala. Then (10 000 − 9 565) ×
361 ⁄ 10 000 = 15.7035 days, 15 d 42 gh 12.6 pa, and 15 palas more make
15 d 42 gh 27.6 pa. The Siddhānta's Meṣa saṅkrānti of 2024, by
`surya_siddhanta`, is 13 April at 17:55 Universal Time; Pingala ends
15.7077 days later, on 29 April at 10:54, and Kalayukta, the 52nd, begins.
Pingala itself had begun in May 2023, twenty days into the year the rule
names Anala. So the year is Pingala from its Chaitra śukla 1, 9 April
2024, although Kalayukta is in progress from 29 April: the name of the
year and the name of the day part company for most of every year.

**The Tiruvaḷḷuvar year.** Tamil Nadu's official count is the
Tiruvaḷḷuvar year, the Gregorian year plus 31, gazetted in 1971 and in
force from 1972 [wikipedia-valluvar-year; the Gazette not read]. It was
introduced on Thiruvalluvar Day, which was moved to Thai 1 in 1971
[tawiki-tiruvalluvar-aandu], and it begins at Thai 1, the Makara
saṅkrānti's month, in mid-January: Tamizhvalai's report of Thai 1,
14 January 2021, says that the year 2052 begins that day and works it as
2021 + 31 [tamizhvalai-2052]. Tamil Nadu's Act 2 of 2008 declared Thai 1
the Tamil New Year and made the Tamil year run from Thai 1 to the end of
Margazhi [tn-act-2-2008]; the act was repealed in 2011 and the new year
returned to Chithirai 1 [wikipedia-puthandu; the repealing act not
read]. So the Tiruvaḷḷuvar year and the Tamil year that opens at
Chithirai overlap for nine months: the Chithirai year of 14 April 2024 is
Tiruvaḷḷuvar 2055 until the end of Margazhi and 2056 from Thai 1,
14 January 2025.

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
[wikipedia-ml-njattuvela].

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
  second day to carry a tithi. The day begins at sunrise and is named by
  the civil day on whose sunrise it begins,
  `DayBoundary::Sunrise(DayNaming::ByStart)`, as Reingold and Dershowitz
  read a fixed day at "Sunrise that day" [reingold2018code,
  `hindu-lunar-from-fixed`]; every calendar of this document that begins
  at sunrise says the same. `RASHTRIYA`, the registered one, reads the
  day at the Central Station's sunrise with the Lahiri ayanāṃśa; `UJJAIN`
  reads it at Ujjain; `new` takes any place and any ayanāṃśa. The era is
  `saka`, and `vikrama-year` is an extra field. `new_year`,
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
  `SolarModel`. The eras are `saka` for the Tamil year (the Gregorian year
  of Chithirai less 78, as for the lunisolar calendars), `kollam` (the
  Gregorian year of Chingam less 824), `bangabda` (less 593) and `vs`
  (plus 57). The Tamil date carries the Tiruvaḷḷuvar year beside it as
  the extra field `tiruvalluvar-year`, derived and ignored on input:
  `tiruvalluvar_year_of` gives the Gregorian year of the last Thai 1 plus
  31, so it turns at Thai 1 while the calendar's own year turns at
  Chithirai 1. `new` gives the same reckoning at another
  place or with the *Sūrya Siddhānta*'s Sun, under an identifier of the
  caller's. A fifth rule, `CivilDay`, the midnight-to-midnight day of the
  saṅkrānti, exists for the Bikram Sambat and is that document's.
  `TAMIL` alone of the four names its years: `samvatsara_of` gives the
  position, 1 to 60, and the date's fields carry it as the extra
  `samvatsara`, derived and ignored on input; the calendar declares the
  sixty names as a cycle of kind `samvatsara`, in Sewell and Dikshit's
  Sanskrit forms without diacritics (Krodhin, Visvavasu), and `hc-i18n`'s
  Tamil gives them in the Lexicon's Tamil script. The Lexicon's spellings are kept where almanacs
  now print others — ஶ்ரீமுக for ஸ்ரீமுக, தாருண for தாரண, பார்த்திவ for
  பார்த்திப; Wikipedia's Tamil list [wikipedia-tamil-calendar] was read and
  not used, for spellings such as விசுவாசுவ that no other source read has.
- **The year's name on the lunisolar calendars.** `hindu-lunar` carries
  the southern name as the extra `samvatsara`, the Tamil year's rule on
  its own Śaka year, so that the year from Ugādi is named as the Telugu
  and Kannada almanacs name it; `hindu-lunar-purnimanta` carries the
  northern name under the same key, the name current at the apparent Meṣa
  saṅkrānti that falls in the year's Chaitra by the *Sūrya Siddhānta*
  with the *bīja*, as Table I gives it after 1500. Both declare the sixty
  names as a cycle and ignore the field on input;
  `HinduLunarCalendar::samvatsara_of` and
  `HinduPurnimantaCalendar::samvatsara_of` give it for a year. The same
  key holds different names on the two calendars on the same day, because
  the two reckonings are different, and a date says which calendar it is
  in.
- **`samvatsara`**: `NAMES`, `southern_of_saka` and `name`, the southern
  cycle.
- **`barhaspatya`**: the northern cycle. The three rules of Art. 59 as
  `MeanSignRule` data, `SURYA_SIDDHANTA`, `ARYA_SIDDHANTA` and
  `SURYA_SIDDHANTA_BIJA`, each with `current_at_sankranti`, `days_to_end`
  and `expunged_in` on the expired Kali year and `at_mean_sankranti` for
  the mean form; `northern_of_saka`, the name Table I couples with a year;
  and `in_progress_at`, the name in progress at a moment by a rule, from
  the Siddhānta's apparent Meṣa saṅkrānti. In a year that expunges a name,
  the rule gives the end of the first name and, from the next year's
  saṅkrānti, the end of the third; the expunged name is taken to begin at
  the one and to end 361 days, the rule's Jovian year, before the other.
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
  `KATHMANDU`, all at sea level so that sunrise is the almanac's. `UJJAIN` is the book's `ujjain`, 23°9′ N, 75°46′6″ E
  [reingold2018code], the longitude `surya_siddhanta` uses. The city's
  modern coordinates, 23.1765° N, 75.7885° E, lie about a minute of arc
  away, five seconds of sunrise; over 1700–2299 reading at them instead
  would move the sunrise tithi at Ujjain on sixteen days — 17 June 1770,
  22 February 1802, 17 January 1831, 20 May 1859, 26 August 1867,
  7 January 1995, 23 September 2089, 15 October 2097 and eight days after
  2100 — measured by comparing `tithi_of_day` at the two places on every
  day of the range. No registered calendar reads at Ujjain.
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
  - *The names in Telugu and Kannada script*: no list of the sixty in
    either script was read, so only the Sanskrit forms and the Tamil
    Lexicon's are carried.
  - *The expunction reckoned on the lunisolar year*, which Sewell and
    Dikshit say some places kept for a time (Art. 57), and the
    *Bṛhatsaṃhitā* rule, whose reading they dispute (Art. 59 d): neither
    has a table or example to hold it to. Nor the twelve-year cycle of
    Jupiter (Art. 63).
  - The Odia Anka is [odia-anka.md](odia-anka.md)'s.
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
| Bengali San 1430, Kollam 1199, Vikrama 2080 and the Tamil Śaka 1945 open on the almanac's days | `the_eras_begin_where_the_almanac_says` | all |
| The Tiruvaḷḷuvar year 2052 begins on Thai 1, 14 January 2021, as reported that day, three months before the Tamil Śaka year 1943 at Chithirai 1; Chithirai to Margazhi carry the Gregorian year plus 31 and Thai to Panguni one more | `the_tiruvalluvar_year_turns_at_thai_and_the_saka_year_at_chithirai` | all |
| The southern rule on Sewell and Dikshit's worked examples: Angiras (Śaka 1674, 1752), Rudhirodgarin (1725, 1803–04), Chitrabhanu (1744, 1822) | `samvatsara::sewell_and_dikshits_rule_names_their_own_examples` | all |
| The Tamil year's name on printed days: Rudhirodgarin on 30 May 1803 and 30 March 1804; Śobhana (Śobhakṛt) on 13 April 2024 and Krodhin from the 14th; Viśvāvasu from 14 April 2025; Parābhava from 14 April 2026 | `the_tamil_years_carry_their_printed_names` | all names; Sewell and Dikshit's two days are a day earlier in the month by the modern Sun, 18 Vaikasi and 19 Panguni for their 19th and 20th |
| குரோதி in Tamil and Krodhin in English on 14 April 2024, சோபகிருது the day before | `hyper-calendar`'s `the_tamil_year_is_named_in_tamil_and_through_the_fallback` | all |
| The Ugādi year's name as the Telugu almanac prints it: Śobhana (Śobhakṛt) on 8 April 2024, Krodhin from 9 April to 29 March 2025, Viśvāvasu from 30 March 2025, Parābhava from 20 March 2026, the day `hindu-lunar` opens Śaka 1948 | `hindu_lunar::the_ugadi_years_carry_their_printed_names` | all |
| The three rules on Sewell and Dikshit's three examples: the name, the days to its end to the hundredth of a pala, and the name expunged | `barhaspatya::the_three_rules_work_sewell_and_dikshits_examples` | all |
| Every expunction by the *Sūrya Siddhānta* from Śaka 200 to 1800 at the mean saṅkrānti, without the *bīja* to 1500 and with it after, against Art. 60's nineteen | `the_siddhantas_expunged_names_are_sewell_and_dikshits_list` | 19 of 19, and no others |
| The same at the apparent saṅkrānti, Table I's: the seven years Art. 60 marks with an asterisk a year later, on the next name, the other twelve the same | `table_i_counts_from_the_apparent_sankranti_and_differs_where_marked` | all |
| Art. 60's Ārya column | `the_arya_column_of_the_list_is_a_year_after_the_arya_rule` | 0 of 19: see below |
| Vijaya in the north and Chitrabhānu in the south in 1822; the gap of 11, 12 from Śaka 1779 current, 13 from 1865 expired, 14 from 1950 | `the_northern_and_southern_names_stand_eleven_then_twelve_then_thirteen_apart`, `hindu_purnimanta::the_northern_years_carry_the_names_sewell_and_dikshit_and_the_almanacs_print` | all |
| Pingala for Śaka 1946, as the Hrishikesh Panchang prints it, from 9 April 2024 to 29 March 2025 on `hindu-lunar-purnimanta`; Anala the day before | `printed_northern_years_carry_the_rules_names`, `the_northern_years_carry_the_names_sewell_and_dikshit_and_the_almanacs_print` | all |
| The moment: the Siddhānta's Meṣa saṅkrānti of 1514 against Table I's, Vṛṣa's end on 31 March 1514, the expunged Chitrabhānu and Subhānu at the next saṅkrānti; Vibhava 3.3 days and Śukla 364.3 days after the saṅkrānti of Śaka 1779 current; the worked example of 2024 | `the_moment_follows_the_rule_through_an_expunged_year`, `vibhava_begins_three_days_after_the_sankranti_of_1779`, `pingala_gives_way_to_kalayukta_a_fortnight_into_saka_1946` | the saṅkrānti of 1514 1½ minutes from the printed one, Vṛṣa's end within three minutes, the rest to the tenth of a day |
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
- *Sewell and Dikshit's Tamil days.* Their tables, computed with the old
  almanac's Sun, give 30 May 1803 as 19 Vaikasi and 30 March 1804 as
  20 Panguni; the registered reckoning's modern Sun begins both months a
  day later, so it reads 18 and 19. The year and its name agree.
- *Festivals kept on an afternoon or evening tithi.* The almanac lists
  Raksha Bandhan 2023 on 30 August and Vijayā Daśamī 2024 on 12 October,
  days on which the sunrise tithi was still the previous one. The dates
  are right; the observance is `hc-holiday`'s rule, and the test
  `a_festival_kept_on_an_afternoon_tithi_is_a_holiday_rule_not_a_date`
  holds both.
- *Art. 60's Ārya column.* The Ārya rule puts each of the column's
  nineteen expunctions a year earlier, on the name before: Dundubhi, the
  56th, in Śaka 231 current, where the column has Rudhirodgārin, the 57th,
  in 232. Sewell and Dikshit's own Example 2 is on the rule's side — in
  Śaka 230 expired, 231 current, "Dundubhi commences within four days of
  the Mesha sankranti" and "will be expunged" — and a year at the mean
  saṅkrānti cannot expunge later than at the apparent one, which comes two
  days before it. Their note to the list gives Śaka 231 and the 56th, 998
  and the 52nd, and 1339 and the 37th as others' reading of the
  *Bṛhatsaṃhitā* rule; the Ārya rule gives those three and the other
  sixteen one earlier too. The module carries the rule as stated and the
  test records the offset; the page image of the list was not read, only
  its OCR text.
- *Prokerala's northern names* follow the southern cycle thirteen names on
  in every year its list covers, from Vikrama 1995 on, without an
  expunction: Kalayukta for 2082 and Siddhārthin for 2083, as the rule
  has them, but Manmatha for 1999, where the rule, which expunged Manmatha
  that year, has Jaya, and Durmati for 2085 (2028–29), where it has
  Dundubhi [prokerala-hindu-calendar]. The two agree from Vikrama 2000 to
  2084 only; no almanac of 2028 exists yet to say which the north will
  print.
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
| [sewell1896] | The four regional rules and their names; kṣaya and adhika tithis; the naming of adhika and kṣaya months; the intervals between expunged months; the sixty-year cycle, its northern and southern reckonings and the southern rule, and the worked examples of 1752, 1803–04 and 1822; the northern cycle's length and expunction, the name coupled with the year, the three rules with their examples, the list of expunged names and Table I's reckoning | Yes, 2026-09-25, in the Internet Archive's OCR text; Arts. 28, 32, 45, 48 and 50; Arts. 53–62 and the worked examples that name Angiras, Rudhirodgarin and Chitrabhanu on 2026-09-26, and Arts. 54–60, 75 and 120 again that day for the northern cycle. The sixty names are read off the OCR of Table I, col. 6, and Table XII, where the diacritics are lost; Art. 60's list off the OCR of its table |
| [burgess1860] | The *Sūrya Siddhānta*'s revolutions of Jupiter, from which Sewell and Dikshit's numbers come | Not read; the module takes Sewell and Dikshit's numbers as they give them |
| [prokerala-telugu-calendar] | The Telugu year from Chaitra named Krodhi (Śaka 1946, from 9 April 2024, and its Phālguna to 29 March 2025), Viswavasu (1947, from 30 March 2025) and Parabhava (1948, from 20 March 2026) | Yes, 2026-09-26; the Telugu script on the pages was not relied on |
| [hrishikesh-panchang-2081] | Vikrama 2081, Śaka 1946, "पिङ्गल नामाब्दः" | The almanac's title as Exotic India lists it, 2026-09-26; the almanac itself not read |
| [prokerala-hindu-calendar] | The northern names it gives from Vikrama 1995 on, without expunction | Yes, 2026-09-26 |
| [tamil-lexicon] | The sixty year names in Tamil script, entry வருஷம், sense 2, and each as a headword | Yes, 2026-09-26, in the Digital Dictionaries of South Asia edition |
| [prokerala-tamil-2024] | Chithirai 2024 headed "Krodhi", Tamil New Year's Day 14 April | Yes, 2026-09-26 |
| [pansalb-tamil-new-year-2025] | "Tamil New Year (5127 – Visuvasuva)", 14 April 2025 | Yes, 2026-09-26 |
| [wikipedia-valluvar-year] | The Tiruvaḷḷuvar year as the Gregorian year plus 31; the Gazette of 1971 and its use from 1972 | Yes, 2026-09-26; the Gazette not read |
| [tawiki-tiruvalluvar-aandu] | Thiruvalluvar Day moved to Thai 1 in 1971, and the count introduced on it | Yes, 2026-09-26 |
| [tamizhvalai-2052] | The Tiruvaḷḷuvar year 2052 beginning on Thai 1, 14 January 2021 | Yes, 2026-09-26 |
| [tn-act-2-2008] | The Tamil year from Thai 1 to the end of Margazhi, section 3 | Yes, 2026-09-26, in PRS Legislative Research's copy of the Gazette Extraordinary |
| [wikipedia-puthandu] | The repeal of 2011 and the new year's return to Chithirai 1 | Yes, 2026-09-26; the repealing act not read |
| [reingold2018] | The Old Hindu calendars, the Siddhānta's Sun, the amānta rules, Ujjain | Not read directly; the published code was |
| [reingold2018code] | `hindu-epoch`, `arya-solar-year`, `arya-lunar-month`, `old-hindu-lunar-leap-year?`, `hindu-sine-table`, `hindu-sidereal-year`, `hindu-anomalistic-year`, `hindu-true-position`, `ujjain`, `sidereal-start`, `hindu-lunar-station`, and `hindu-lunar-from-fixed` for the day read at sunrise | Yes, 2026-09-25; `hindu-lunar-from-fixed` 2026-09-26 |
| [calcal-modern-hindu] | The line-by-line check of the Siddhānta constants the module records | Yes, 2026-09-25: the constants of `modern_hindu.R` are the book's |
| [wikipedia-hindu-calendar] | The twelve month names in Devanagari | Yes, 2026-09-25 |
| [wikipedia-adhik-maas] | The 32½-month average, the adhika and nija names, the pūrṇimānta placing of the adhika month | Yes, 2026-09-25 |
| [wikipedia-tithi] | The 12° definition, the tithi's range of length, kṣaya and adhika tithis | Yes, 2026-09-25 |
| [wikipedia-nakshatra] | The twenty-seven arcs of 13°20′, Aśvinī opposite Spica, the list | Yes, 2026-09-25 |
| [wikipedia-vikram-samvat] | The epoch of 57 BCE and the offset of 57 | Yes, 2026-09-25 |
| [wikipedia-malayalam-calendar] | The Kollam epoch of 825 CE, Chingam as the first month, Vishu in Medam | Yes, 2026-09-25 |
| [wikipedia-bengali-calendars] | The Bengali San's offset of 593, Boishakh first, the West Bengal reckoning as sidereal | Yes, 2026-09-25 |
| [wikipedia-tamil-calendar] | The month names Chithirai to Panguni and the year at the Meṣa saṅkrānti; its sixty-year table was compared with the Lexicon's and not used | Yes, 2026-09-25 and 2026-09-26 |
| [wikipedia-aryabhata-sine-table] | The twenty-four values of the sine table | Yes, 2026-09-25 |
| [wikipedia-thaipusam] | Thaipusam on Puṣya in Thai | Yes, 2026-09-25 |
| [wikipedia-onam] | Onam on Thiruvonam in Chingam | Yes, 2026-09-25 |
| [wikipedia-ml-njattuvela] | What a ñāṭṭuvēla is; the Thiruvathira ñāṭṭuvēla | Yes, 2026-09-25 |
| [swisseph] | The ayanāṃśa anchors `hc-seasons` carries, and Lahiri as the Spica tradition | Yes, 2026-09-25 |
| [drik-sun-nakshatra-2025] | The Sun's twenty-seven nakṣatra entries of 2025 for New Delhi | Yes, 2026-09-25; the five entries checked agree with the test's table |
| [drik-thaipusam-2024], [drik-thaipusam-2025] | Puṣya's beginning and end at Chennai, 25–26 January 2024 and 10–11 February 2025 | Yes, 2026-09-25 |

Statements in the module documentation for which this document names no
source: the modern mean sidereal
year of 365.256 36 days; and that Drik Panchang's Lahiri anchor is about
20″ from the library's, which is inferred from the measured offset. The
Central Station's latitude as "the latitude of Ujjain" is the Committee's
phrase; the module states the coordinates only.

## Code

`crates/hc-calendars-indic/src/hindu_lunar.rs` (`HinduLunarCalendar`,
`MIN_YEAR`, `MAX_YEAR`, `MONTHS`), `hindu_purnimanta.rs`,
`hindu_solar.rs` (`SankrantiRule`, `SolarModel`, `TAMIL`, `MALAYALAM`,
`BENGALI`, `VIKRAMI`, `ALL`, `samvatsara_of`), `samvatsara.rs` (`NAMES`,
`southern_of_saka`), `barhaspatya.rs` (`MeanSignRule`, `SURYA_SIDDHANTA`,
`ARYA_SIDDHANTA`, `SURYA_SIDDHANTA_BIJA`, `northern_of_saka`,
`in_progress_at`), `hindu_old.rs` (`HINDU_EPOCH`,
`ARYA_SOLAR_YEAR`, `ARYA_LUNAR_MONTH`), `tithi.rs`, `nakshatra.rs`,
`surya_siddhanta.rs` (`SIDEREAL_YEAR`, `ANOMALISTIC_YEAR`,
`UJJAIN_LONGITUDE_DEGREES`) and `places.rs`. Anchors:
`every_fortnight_of_two_years_begins_where_the_rashtriya_panchang_says`,
`saka_1945_has_the_intercalary_sravana_and_1946_none`,
`every_dark_fortnight_carries_the_name_the_rashtriya_panchang_gives_it`,
`the_tamil_months_begin_where_the_rashtriya_panchang_says`,
`the_malayalam_months_begin_where_the_rashtriya_panchang_says_save_medam`,
`the_eras_begin_where_the_almanac_says`,
`the_tiruvalluvar_year_turns_at_thai_and_the_saka_year_at_chithirai`,
`the_tamil_years_carry_their_printed_names`,
`sewell_and_dikshits_rule_names_their_own_examples`,
`the_ugadi_years_carry_their_printed_names`,
`the_three_rules_work_sewell_and_dikshits_examples`,
`the_siddhantas_expunged_names_are_sewell_and_dikshits_list`,
`table_i_counts_from_the_apparent_sankranti_and_differs_where_marked`,
`the_moment_follows_the_rule_through_an_expunged_year`,
`the_northern_years_carry_the_names_sewell_and_dikshit_and_the_almanacs_print`,
`the_suns_nakshatra_transits_of_2025_are_the_almanacs`,
`pushya_in_january_2024_begins_and_ends_when_the_almanac_says`,
`the_table_holds_the_classical_jyas`,
`the_mesha_sankranti_of_2024_is_later_than_the_lahiri_one`,
`the_epoch_is_friday_18_february_3102_bce_and_opens_year_zero`,
`the_mean_months_of_2024_fall_within_two_days_of_the_true_ones`. The
ayanāṃśas and the sidereal signs are `crates/hc-seasons/src/zodiac/sidereal.rs`,
the month-name traditions `rashi.rs` beside it; the festival rules that
read these calendars are `crates/hc-holiday/src/hindu.rs`.
