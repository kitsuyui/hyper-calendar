# The Hindu calendars: amānta and pūrṇimānta months, the solar months, the nakṣatras, the yoga and the karaṇa, ayanāṃśa, the sixty year names north and south

Backs the identifiers `hindu-lunar`, `hindu-lunar-surya-siddhanta`,
`hindu-lunar-purnimanta`,
`hindu-solar-tamil`, `hindu-solar-malayalam`, `hindu-solar-bengali`,
`hindu-solar-vikrami`, `hindu-solar-surya-siddhanta`, `hindu-old-solar`
and `hindu-old-lunar` in `hc-calendars-indic`, and the crate's `tithi`, `nakshatra`,
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

**Who still computes by the Siddhānta.** The Committee sent the almanac
makers a questionnaire, and its fourteenth question asked "Whether
calculations are based on modern method or the old Siddhantic method"
[crc1955, Annexure VI]. Most of the replies printed say the modern
method. Four name the *Sūrya Siddhānta*: the *Gupta Press Panjika* of
Calcutta, a Bengali solar almanac that also gives the amānta and
pūrṇimānta lunar months (reply 5, p. 24); the *Hosaritti Panchanga* of
Haveri, Dharwar, in Sanskrit, Marathi and Kannada, pūrṇimānta (reply 43,
p. 30); the *Bhagyodaya Panchang* alias *Chintaharan Jantri* of Sitapur,
in Hindi, pūrṇimānta, in the Vikrama era (reply 47, p. 31); and the *Sri
Sringagiri Sri Jagat Guru Srimath Panchangam* of Kollegal, in Kannada,
amānta, in the Kali Yuga era (reply 48, p. 31). Several more answer
"Old Siddhantic method" without naming the text, and the *Gupta Press
Panjika* gives its year as "365d .258756481 mean solar days", the
Siddhānta's sidereal year to the ninth decimal. The report adds that the calendar makers take the Moon from
the Siddhāntas' formulae with "certain corrections called *bija*
introduced by later Indian astronomers" [crc1955, p. 3]. That is the
evidence this document has for the claim that the Siddhānta's calendar is
kept: four almanacs in 1953–54, by their own account. No table of any of
them was read, and whether any keeps today the months and tithis
`hindu-lunar-surya-siddhanta` computes — with Reingold and Dershowitz's
*bīja* and their sunrise — is not known here.

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

**The Siddhānta's Moon and sunrise.** The *Sūrya Siddhānta*'s Moon is
built as its Sun is: a mean Moon round a sidereal month of
1 577 917 828⁄57 753 336 days (27.321 674), corrected by an epicycle of
32⁄360 of the deferent that shrinks by 1⁄96 as the anomaly grows, the
anomaly turning once in an anomalistic month of
1 577 917 828⁄(57 753 336 − 488 199) days, "with bija correction"
[reingold2018code, `hindu-sidereal-month`, `hindu-anomalistic-month`,
`hindu-lunar-longitude`]. The interval from the creation to the Kali
Yuga epoch is a whole number of sidereal months and 25 926 790 776¾
anomalistic ones, so the mean Moon stands at zero at the epoch and its
anomaly at three quarters, and the module counts from there, as it does
for the Sun. The elongation of this Moon from this Sun gives the tithi,
and its return to zero the conjunction. The day is read at the
Siddhānta's own sunrise at Ujjain, not the true one: six in the morning
by Ujjain's clock, less a "gross approximation" to the equation of time
from the Sun's equation of centre, plus the ascensional difference from
the Sun's declination by the table's sines — the obliquity 24° as
1397⁄3438 of the radius — and a quarter of the difference between the
solar and the sidereal day, turned into time at 1 577 917 828⁄1 582 237 828
of a day to a sidereal revolution [reingold2018code, `hindu-sunrise`,
`hindu-equation-of-time`, `hindu-ascensional-difference`,
`hindu-tropical-longitude`, `hindu-rising-sign`, `hindu-daily-motion`,
`hindu-solar-sidereal-difference`]. It has no refraction and no
semidiameter, and over 2000–2009 it falls from 7.5 minutes before to 15.8
minutes after `hc-astro`'s sunrise at Ujjain. The months, the naming and
the tithi at sunrise are then the rules above, and
`hindu-lunar-surya-siddhanta` is the same engine as `hindu-lunar` on this
sky [reingold2018code, `hindu-lunar-from-fixed`, `fixed-from-hindu-lunar`].
Its year is the Siddhānta's: the Kali Yuga solar year in which the
month's name falls, less 3179 for the Śaka year, as `hindu-calendar-year`
reckons it [reingold2018code, `hindu-solar-era`].

**Worked example: Chaitra śukla 1 of Śaka 1947 on the Siddhānta.** The
Siddhānta's conjunction falls at 11:18 UT on 29 March 2025, with its Sun
in Mīna; its Meṣa saṅkrānti at 00:08 UT on 14 April, before the next
conjunction, so the month holds that saṅkrānti and is Chaitra, and not
intercalary. On 30 March its sunrise at Ujjain is at 01:01 UT, 06:04 by
Ujjain's clock, ten minutes after the true sunrise; the Moon then stands
at 352.87° and the Sun at 345.28°, an elongation of 7.58°, inside the
first tithi. So 30 March 2025 is Chaitra śukla 1 of Śaka 1947, Vikrama
2082, the new year's day — the same day the true Sun and Moon give, at
Ujjain and at the Central Station.

The two longitudes at that sunrise can be followed by hand, from the
constants above and the table of sines of 24 steps of 3° 45′ in a radius
of 3438′ [reingold2018code, `hindu-sine-table`, `hindu-true-position`]:

1. *The instant.* 01:01.07 UT on RD 739 340 is 06:04.1 by Ujjain's clock,
   75° 46′ 6″ east: RD 739 340.252 88, which is 1 872 299.252 88 days
   after the Kali Yuga epoch, RD −1 132 959.
2. *The mean Sun.* The days over the sidereal year, 365.258 756, are
   5 125.953 09 revolutions; the fraction, 0.953 09 of 360°, is 343.114°.
3. *The Sun's anomaly.* The days over the anomalistic year, 365.258 789,
   plus the 0.785 75 of a revolution at the epoch, leave 265.818°.
4. *Its sine.* 265.818° is 70.885 steps of the table, between the
   entries −3409′ and −3431′; interpolated, −3428.5′, or −0.997 23 of the
   radius.
5. *The epicycle.* The Sun's is 14° of the deferent, shrinking by 1⁄42
   of itself times the sine's size: 14° × (1 − 0.997 23⁄42) = 13.668°. The
   equation's sine is −0.997 23 × 13.668⁄360 of the radius, −130.16′.
6. *The equation.* 130.16′ lies between the table's 0′ and 225′, 0.578 5
   of a step, 2.169°; negative, as its sine is.
7. *The true Sun.* The mean less the equation: 343.114° + 2.169° =
   345.283°.

The Moon is the same with its own constants: 68 527.984 10 sidereal
months give a mean Moon of 354.280°; the anomaly, with the three
quarters at the epoch, is 163.848°, whose sine, between the entries
1105′ and 890′, is 956.03′; the epicycle of 32° shrinks by 1⁄96 of the
sine's size to 31.907°, the equation's sine is 84.73′, and the equation
1.412°, so the Moon stands at 354.280° − 1.412° = 352.867°. The hand
computation stops at the searches: the conjunction and the saṅkrānti are
found by bisecting the elongation and the Sun's longitude, and the
Siddhānta's sunrise by the formula above, and those three are the
module's results, not steps worked here. The two skies do not always agree:
over 2000–2030 they give different dates at Ujjain on 1 389 of 11 323
days, 100 of them in another month, and in Śaka 1904 the Siddhānta's
Pauṣa, from 15 January 1983, holds both the Makara and the Kumbha
saṅkrāntis, so that Māgha is expunged in a year with an intercalary
Āśvina and an intercalary Phālguna.

**The Siddhānta's solar calendar.** Reingold and Dershowitz's modern
Hindu solar calendar is the solar months on the same Sun and the same
sunrise [reingold2018code, `hindu-solar-from-fixed`,
`fixed-from-hindu-solar`, `hindu-zodiac`, `hindu-calendar-year`]. A
month is the Sun's stay in a sign, Meṣa first, and a day is read at the
sunrise that closes it, the Siddhānta's sunrise of the next day: the
month begins on the day whose closing sunrise is the first to see the
Sun in the new sign, so a saṅkrānti between two sunrises begins the
month on the day the first of them opens. That is the Orissa rule of
the table above, and the book's comment names it so ("Orissa rule").
The year is the Siddhānta's solar year: the Kali Yuga years elapsed at
the closing sunrise less the part of a year the Sun's longitude stands
from Meṣa, rounded, less 3179 for the Śaka year. To find a date the book
starts three days before the mean month begins and steps forward to the
first day whose closing sunrise sees the month's sign. The months are
named by their signs, Meṣa to Mīna, as the Old Hindu solar calendar's
are.

**Worked example: Meṣa 1 of Śaka 1947 on the Siddhānta.** The
Siddhānta's Meṣa saṅkrānti falls at 00:08 UT on 14 April 2025. Its
sunrise at Ujjain that morning is at 00:51 UT, 05:54 by Ujjain's clock,
after the saṅkrānti; its sunrise on the 13th came before it. So the
sunrise of the 14th is the first to see the Sun in Meṣa, the day it
closes, 13 April, is Meṣa 1, and the Siddhānta's year then is Kali Yuga
5126, Śaka 1947. The same months closed by the true sunrise at Ujjain,
`VIKRAMI` rebuilt there on the Siddhānta's Sun, part from these on 2 of
the 372 month starts of 2000–2030: the Vṛścika saṅkrānti of 17 November
2022 at 01:22.5 UT fell after the true sunrise, 01:11.6, and before the
Siddhānta's, 01:24.9, so this calendar begins Vṛścika on 16 November and
the other on the 17th; the Dhanus saṅkrānti of 16 December 2024 is the
other.

**Ujjain, and the book's astronomical calendars.** Reingold and
Dershowitz also compute the lunisolar calendar and the Tamil solar
calendar from the true Sun and Moon, reading them at Ujjain
[reingold2018code, `astro-hindu-lunar-from-fixed`,
`astro-hindu-solar-from-fixed`], and their errata say the two work as
advertised only in Universal Time [reingold2018errata, correction 15].
With that correction they are `hindu-lunar` and `hindu-solar-tamil` read
at Ujjain instead of the Central Station, 6.7° west, whose sunrise and
sunset come about 27 minutes later, and they are not registered: a place
is a parameter and not a convention (policy §5), and
`HinduLunarCalendar::UJJAIN` and `TAMIL` rebuilt at `places::UJJAIN` are
the calendars. The place moves 302 of the 11 323 lunisolar dates of
2000–2030, 2.7%, where a tithi ends between the two sunrises on the day or
the day before, and 5 of the 372 Tamil month starts of Śaka 1922–1952. The book's own ayanāṃśa, zero at the Meṣa
saṅkrānti of 285 CE, and its Tamil sunset of the Sun's centre on a flat
horizon, are not carried either; with the Lahiri ayanāṃśa and the
library's sunset the rebuilt calendars give the book's value on every
sample date but the four the errata explain (below).

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

**The yoga and the karaṇa.** The other two limbs of the almanac are the
*yoga* and the *karaṇa* [sewell1896, Arts. 9 and 10]. A yoga is the time
in which "the sum of the motions, of the sun and moon is increased by
13°20′" [sewell1896, Art. 9]: the sum of the two sidereal longitudes,
taken modulo 360°, is cut into twenty-seven arcs, Viṣkambha, Prīti,
Āyuṣmān, Saubhāgya, Śobhana, Atigaṇḍa, Sukarmā, Dhṛti, Śūla, Gaṇḍa,
Vṛddhi, Dhruva, Vyāghāta, Harṣaṇa, Vajra, Siddhi, Vyatīpāta, Varīyān,
Parigha, Śiva, Siddha, Sādhya, Śubha, Śukla, Brahma, Indra and Vaidhṛti
[wikipedia-nityayoga]. It marks no event in the sky; S. B. Dikshit
thought it "useful only in astrology" [wikipedia-nityayoga, quoting his
*Bhāratīya Jyotiṣ Śāstra*, not read]. A karaṇa is half a tithi, the
Moon's gain of 6° on the Sun, sixty to a month [sewell1896, Art. 10].
Seven movable names, Bava, Bālava, Kaulava, Taitila, Gara, Vaṇija and
Viṣṭi, run eight times round the fifty-six halves from the second half
of śukla 1 to the first half of kṛṣṇa 14; four fixed names take the other
four: Śakuni the second half of kṛṣṇa 14, Catuṣpada and Nāga the two
halves of amāvāsyā, Kiṃstughna the first half of śukla 1
[wikipedia-karana; sewell1896, Art. 40]. Sewell and Dikshit note that the
*Sūrya Siddhānta* orders the fixed four Śakuni, Nāga, Catuṣpada,
Kiṃstughna, and follow the practice of western India, which Varāhamihira
and Brahmagupta support, of putting Catuṣpada before Nāga [sewell1896,
Art. 40, note]; so do Reingold and Dershowitz's `karana`
[reingold2018code] and Drik Panchang. As with the tithi, a day carries the
yoga and the karaṇa in progress at its sunrise, and the almanac prints the
moment each ends: Drik Panchang's page for New Delhi on 1 January 2025
gives "Yoga Vyaghata upto 05:07 PM" and "Karana Balava upto 02:55 PM",
then Kaulava to 02:24 AM on the 2nd, the end of the tithi
[drik-day-panchang-2025].

**Worked example: the end of Vyāghāta on 1 January 2025.** At 11:30 UT,
17:00 IST, the Sun's apparent longitude is 281.302° and the Moon's
300.379°, and the Lahiri ayanāṃśa is 24.206°. Their sidereal longitudes
are 257.096° and 276.173°, and the sum, less 360°, is 173.269°. The arcs
are counted from zero, so the thirteenth, Vyāghāta, runs from 160° to
173°20′, 173.333°: the yoga is still Vyāghāta, 0.064° short of its end.
The sum grows by about 14° a day, 0.6° an hour, so the end comes some six
and a half minutes later, at 11:36.6 UT, 17:06.6 IST — the page's "upto
05:07 PM". The figures are the library's own, rounded; the test
`panchanga::the_yogas_of_january_2025_end_when_drik_panchang_says` holds
all thirty-two printed ends of the month to within a minute.

**Worked example: Bālava on 1 January 2025.** The karaṇa counts the
elongation, the Moon's longitude less the Sun's, which needs no ayanāṃśa.
At the Central Station's sunrise, 01:11 UT (06:41 IST), the library puts
the elongation at 13.716°. A karaṇa is 6°, so this is the third
half-tithi, 12° to 18°: the first half of śukla 2. The third half is the
second of the fifty-six movable halves, which begin at the second half
of śukla 1 with Bava, so its name is the second movable one, Bālava —
Table VIII's name for the first half of śukla 2 [sewell1896, Table VIII,
col. 4] and the page's "Karana Balava". It ends when the elongation
reaches 18°: at 11:30 UT the longitudes above give 300.379° − 281.302° =
19.077°, so the elongation grows by 5.361° in 10 h 19 min, about 0.52° an
hour, and the remaining 4.284° take some 8 h 14 min, which puts the end
near 09:25 UT, 14:55 IST. The library's search finds 14:56 IST, a minute
after the page's "upto 02:55 PM". By 11:30 UT the elongation is in the
fourth half, 18° to 24°: Kaulava, the page's next karaṇa. The test
`panchanga::the_first_of_january_2025_carries_vyaghata_and_balava` holds
the day's name, and
`panchanga::the_karanas_of_january_2025_end_when_drik_panchang_says` the
month's fifty-nine printed ends.

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
- **`hindu-lunar-surya-siddhanta`**, as `SiddhantaLunarCalendar`, the
  amānta calendar on the *Sūrya Siddhānta*'s Sun, Moon and sunrise,
  through the same engine, `amanta`, as `hindu-lunar`, so that the two
  differ only in their sky. `UJJAIN`, the registered one, reads the day at
  the Siddhānta's sunrise at Ujjain; `new` takes another place, whose
  latitude and longitude the Siddhānta's sunrise uses. The date is
  `HinduLunarDate`, the era `saka`, with `vikrama-year` and `samvatsara`,
  as on `hindu-lunar`. Its range is Kali Yuga 1 to 10 000, 3101 BCE to
  6899 CE, as for the Old Hindu calendars: the reckoning is arithmetic,
  so the range is the library's choice, and dates before the text and its
  *bīja* existed are the rule's, proleptically, and nobody's record. Its
  period of use is unrecorded: the almanacs of 1953–54 above give none.
- **`hindu-lunar-purnimanta`**, as `HinduPurnimantaCalendar`, a renaming
  of the amānta calendar's months day for day: every conversion goes
  through the amānta calendar, so the two can never disagree about a
  tithi, only about the month's name. The adhika month keeps its own
  name in both.
- **`hindu-solar-surya-siddhanta`**, as `SiddhantaSolarCalendar`, the
  solar months on the *Sūrya Siddhānta*'s Sun under the Orissa rule, the
  day closed by the Siddhānta's sunrise at Ujjain, the months named by
  their signs and the year the Siddhānta's Śaka year, era `saka`, as the
  book computes it. `UJJAIN` is the registered one; `new` takes another
  place for the sunrise. Its range is Kali Yuga 1 to 10 000, as for
  `hindu-lunar-surya-siddhanta`, and for the same reason; its period of
  use is unrecorded. It is not `VIKRAMI` rebuilt at Ujjain on the
  Siddhānta's Sun: that calendar closes the day by the true sunrise, names
  the months with the Vikrami names and counts the Gregorian year of the
  Meṣa saṅkrānti plus 57, and it is not registered.
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
- **At the WebAssembly and C boundaries**, `hc_hindu_lunar_date` gives the
  amānta date at a place the caller names, on the true sky in the zodiac
  of a named ayanāṃśa or on the Siddhānta's, which is where a place other
  than the Central Station or Ujjain is given; `hc_surya_siddhanta_at` and
  `hc_surya_siddhanta_sunrise` give the Siddhānta's Sun, Moon, tithi and
  sign at an instant and its sunrise at a place.
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
  `sunset_of`, and `Prevalence` — dawn (four ghaṭikās before sunrise,
  the Calendar Reform Committee's aruṇodaya for Naraka Caturdaśī
  [crc1955]), sunrise, midday, afternoon (seven tenths of the daylight),
  evening (an hour after sunset), midnight —
  the part of the day a festival's tithi must hold, which `hc-holiday`
  uses and this document does not cover.
- **`nakshatra`**: the twenty-seven as constants `ASHVINI` to `REVATI`,
  `nakshatra_at` and `nakshatra_span` for the Moon, `solar_nakshatra_at`,
  `solar_nakshatra_ingress_after` and `solar_nakshatra_span` for the Sun,
  all under any ayanāṃśa.
- **`surya_siddhanta`**: `solar_longitude`, `sign_at` and
  `ingress_after` for the Siddhānta's Sun, `lunar_longitude`,
  `lunar_phase`, `tithi_at` and `conjunction_at_or_after` for its Moon,
  and `sunrise` at any place, all read at Ujjain's meridian, 75°46′6″ E as
  the book gives it.
- **`places`**: `CENTRAL_STATION` (23.183 333° N, 82.5° E), `UJJAIN`,
  `KATHMANDU`, all at sea level so that sunrise is the almanac's. `UJJAIN` is the book's `ujjain`, 23°9′ N, 75°46′6″ E
  [reingold2018code], the longitude `surya_siddhanta` uses. The city's
  modern coordinates, 23.1765° N, 75.7885° E, lie about a minute of arc
  away, five seconds of sunrise; over 1700–2299 reading at them instead
  would move the sunrise tithi at Ujjain on sixteen days — 17 June 1770,
  22 February 1802, 17 January 1831, 20 May 1859, 26 August 1867,
  7 January 1995, 23 September 2089, 15 October 2097 and eight days after
  2100 — measured by comparing `tithi_of_day` at the two places on every
  day of the range. Of the registered calendars only
  `hindu-lunar-surya-siddhanta` reads at Ujjain, and at the Siddhānta's
  sunrise there, not the true one.
- **Range** roughly Gregorian 1700 to 2299 for the true calendars — Śaka
  1622 to 2221 for the lunisolar ones, Chaitra śukla 1 in March 1700 to
  the eve of the one in March 2300, the era years the offsets give for the
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
  - The Odia Anka is [odia-anka.md](odia-anka.md)'s, and the other eras
    of Sewell and Dikshit's Art. 71 over these months — the Kārttikādi
    Vikrama, the Rājyābhiṣeka Śaka, the Saptarṣi, the Magi San, the
    Faṣlī years — are [indian-eras.md](indian-eras.md)'s.
  - *Festival observance*: which part of the day a tithi must hold, and
    the Smārta and Vaiṣṇava readings, are `hc-holiday`'s rules, not
    dates.
  - *The nakṣatra names in the regional languages*, and the twenty-seven
    ñāṭṭuvēla's names and farming lore: the Malayalam article was read for
    what a ñāṭṭuvēla is and no further.
  - *The book's astronomical calendars at Ujjain* as identifiers of their
    own: `hindu-lunar` and `hindu-solar-tamil` at another place, above.
  - *The Siddhānta's pūrṇimānta calendar* and the eras over its months:
    `hindu-lunar-surya-siddhanta` is the amānta form alone, the one the
    book computes.
  - *The Kollam, Bengali and Tamil calendars of other places*: the
    registered four are all read at the Central Station, as the almanac
    computes them, and not at Chennai, Thiruvananthapuram or Kolkata.

- **The yoga and the karaṇa**, as functions in `panchanga`, not as
  calendars: `yoga_at` and `yoga_of_day` give the yoga, 1 to 27, at a
  moment or a day's sunrise for any ayanāṃśa, and `yoga_span` when it
  began and ends; `karana_at`, `karana_of_day` and `karana_span` the same
  for the half-tithi, 1 to 60, and `karana_name` its name, 0 to 10, as
  Reingold and Dershowitz's `karana` maps it [reingold2018code]. The
  names are Drik Panchang's, in English and in its Hindi edition's
  Devanagari [drik-day-panchang-2025]. *Not carried*: the twenty-eight
  weekday-and-nakṣatra yogas of Śrīpati's *Jyotiṣa Ratnamāla*, which
  Wikipedia describes beside the astronomical ones [wikipedia-nityayoga],
  and the auspicious yogas of Sewell and Dikshit's Art. 39; the yoga and
  karaṇa of the *Sūrya Siddhānta*'s Sun and Moon, which no function here
  computes although both bodies are carried.

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
| Every day of Śaka 1622–2221, March 1700 to March 2300, round-trips on the amānta engine | `hindu_lunar::every_day_of_the_range_round_trips` | all in a release build, about three minutes of one core spread over the machine's threads; in a debug one every 211th day and every Chaitra śukla 1 with the day before it |
| The pūrṇimānta name of every dark fortnight of the two years, 25 *vadi* rows | `hindu_purnimanta::every_dark_fortnight_carries_the_name_the_rashtriya_panchang_gives_it` | 25 of 25 |
| The first day of every solar month of both years, Tamil, Bengali and Vikrami | `hindu_solar::the_tamil_months_begin_where_the_rashtriya_panchang_says` and the Bengali and Vikrami tests | 24 of 24 each |
| The first day of every Malayalam month of both years | `the_malayalam_months_begin_where_the_rashtriya_panchang_says_save_medam` | 22 of 24; see below |
| Bengali San 1430, Kollam 1199, Vikrama 2080 and the Tamil Śaka 1945 open on the almanac's days | `the_eras_begin_where_the_almanac_says` | all |
| Every day of 2022–2025 round-trips in each of the five solar reckonings, directly and through its fields | `hindu_solar::every_day_of_four_years_round_trips_in_every_reckoning` | all in a release build; in a debug one every eleventh day and each year's first day and eve |
| The Tiruvaḷḷuvar year 2052 begins on Thai 1, 14 January 2021, as reported that day, three months before the Tamil Śaka year 1943 at Chithirai 1; Chithirai to Margazhi carry the Gregorian year plus 31 and Thai to Panguni one more | `the_tiruvalluvar_year_turns_at_thai_and_the_saka_year_at_chithirai` | all |
| The southern rule on Sewell and Dikshit's worked examples: Angiras (Śaka 1674, 1752), Rudhirodgarin (1725, 1803–04), Chitrabhanu (1744, 1822) | `samvatsara::sewell_and_dikshits_rule_names_their_own_examples` | all |
| The Tamil year's name on printed days: Rudhirodgarin on 30 May 1803 and 30 March 1804; Śobhana (Śobhakṛt) on 13 April 2024 and Krodhin from the 14th; Viśvāvasu from 14 April 2025; Parābhava from 14 April 2026 | `the_tamil_years_carry_their_printed_names` | all names; Sewell and Dikshit's two days are a day earlier in the month by the modern Sun, 18 Vaikasi and 19 Panguni for their 19th and 20th |
| குரோதி in Tamil and Krodhin in English on 14 April 2024, சோபகிருது the day before | `hyper-calendar`'s `the_tamil_year_is_named_in_tamil_and_through_the_fallback` | all |
| The Ugādi year's name as the Telugu almanac prints it: Śobhana (Śobhakṛt) on 8 April 2024, Krodhin from 9 April to 29 March 2025, Viśvāvasu from 30 March 2025, Parābhava from 20 March 2026, the day `hindu-lunar` opens Śaka 1948 | `hindu_lunar::the_ugadi_years_carry_their_printed_names` | all |
| The three rules on Sewell and Dikshit's three examples: the name, the days to its end to the hundredth of a pala, and the name expunged | `barhaspatya::the_three_rules_work_sewell_and_dikshits_examples` | all |
| Every expunction by the *Sūrya Siddhānta* from Śaka 200 to 1800 at the mean saṅkrānti, without the *bīja* to 1500 and with it after, against Art. 60's nineteen | `the_siddhantas_expunged_names_are_sewell_and_dikshits_list` | 19 of 19, and no others |
| The same at the apparent saṅkrānti, Table I's: the seven years Art. 60 marks with an asterisk a year later, on the next name, the other twelve the same | `table_i_counts_from_the_apparent_sankranti_and_differs_where_marked` | all |
| Art. 60's Ārya column | `the_arya_column_of_the_list_is_a_year_after_the_arya_rule` | 0 of 19: see below |
| The 33 sample dates of *Calendrical Calculations*, as its published code computes them [reingold2018code, `dates.l`]: the Old Hindu solar and lunisolar calendars | `every_sample_date_agrees_or_is_refused_or_is_a_known_difference` (`crates/hyper-calendar/tests/rd_sample_dates.rs`) | 33 of 33 each, 586 BCE to 2094 |
| The same dates, the book's modern solar calendar: `hindu-solar-surya-siddhanta` | the same test, and `hindu_solar_siddhanta::the_books_sample_dates_are_reproduced_both_ways` | 33 of 33, 586 BCE to 2094, each converting back |
| Every day of Kali Yuga 1–10000 round-trips on the Siddhānta's solar calendar | `hindu_solar_siddhanta::every_day_of_the_range_round_trips` | all in a release build; in a debug one every 37th day and every Meṣa 1 with the day before it |
| Meṣa 1 of Śaka 1947 on the Siddhānta, worked above: the saṅkrānti at 00:08 UT on 14 April 2025, before that morning's sunrise at 00:51 UT and after the one before | `mesha_1_of_saka_1947_follows_the_siddhantas_sankranti` | all |
| The Siddhānta's sunrise against the true one as the day's end, the same months otherwise | `the_siddhantas_sunrise_moves_two_month_starts_of_thirty_one_years` | 2 of the 372 month starts of 2000–2030, 16 November 2022 and 15 December 2024, 60 days in all, in a release build |
| The same dates, the book's astronomical solar calendar, the Tamil rule at Ujjain on the true Sun: `TAMIL` rebuilt at `UJJAIN`, and the registered `hindu-solar-tamil` | the same test | 10 of 13 each; the other three are the reference code's time scale, below |
| The same dates, the book's astronomical lunisolar calendar at Ujjain: `HinduLunarCalendar::UJJAIN`, and the registered `hindu-lunar` | the same test | 12 of 13 each; the other is the reference code's time scale, below |
| The same dates, the book's modern lunisolar calendar on the Siddhānta's Sun and Moon: `hindu-lunar-surya-siddhanta` | the same test, and `hindu_lunar_siddhanta::the_books_sample_dates_are_reproduced_both_ways` | 33 of 33, 586 BCE to 2094, each converting back |
| Every day of Kali Yuga 1–10000 round-trips on the Siddhānta's lunisolar calendar | `hindu_lunar_siddhanta::every_day_of_the_range_round_trips` | all in a release build, about four minutes of one core spread over the machine's threads; in a debug one every 181st day and every Chaitra śukla 1 with the day before it |
| Chaitra śukla 1 of Śaka 1947 on the Siddhānta, worked above: the conjunction of 29 March 2025 at 11:18 UT in Mīna, the Meṣa saṅkrānti on 14 April, the sunrise at 01:01 UT, the elongation of 7.58° | `chaitra_sukla_1_of_saka_1947_worked_by_hand` | all |
| The Siddhānta's sunrise at Ujjain against `hc-astro`'s, 2000–2009 | `the_siddhantas_sunrise_at_ujjain_is_minutes_from_the_true_one` | from 7.5 minutes earlier to 15.8 later |
| The Siddhānta's dates against the true Sun and Moon at Ujjain | `the_siddhanta_and_the_true_sky_part_on_one_day_in_eight` | 44 of the 366 days of 2024; 1 389 of the 11 323 of 2000–2030, 100 in another month, in a release build |
| Śaka 1904 on the Siddhānta: Māgha expunged, intercalary Āśvina from 18 September 1982 and Phālguna from 13 February 1983 | `saka_1904_loses_magha_and_has_two_intercalary_months` | the rule only; no source read dates the year |
| Ujjain against the Central Station, the only difference of the book's astronomical calendars from the registered ones | `hindu_lunar::ujjain_and_the_central_station_part_on_one_day_in_forty`, `hindu_solar::the_tamil_rule_at_ujjain_moves_one_month_start_in_seventy` | 10 of the 366 lunisolar dates of 2024, 302 of the 11 323 of 2000–2030 in a release build; 5 of 372 Tamil month starts of Śaka 1922–1952 |
| Vijaya in the north and Chitrabhānu in the south in 1822; the gap of 11, 12 from Śaka 1779 current, 13 from 1865 expired, 14 from 1950 | `the_northern_and_southern_names_stand_eleven_then_twelve_then_thirteen_apart`, `hindu_purnimanta::the_northern_years_carry_the_names_sewell_and_dikshit_and_the_almanacs_print` | all |
| Pingala for Śaka 1946, as the Hrishikesh Panchang prints it, from 9 April 2024 to 29 March 2025 on `hindu-lunar-purnimanta`; Anala the day before | `printed_northern_years_carry_the_rules_names`, `the_northern_years_carry_the_names_sewell_and_dikshit_and_the_almanacs_print` | all |
| The moment: the Siddhānta's Meṣa saṅkrānti of 1514 against Table I's, Vṛṣa's end on 31 March 1514, the expunged Chitrabhānu and Subhānu at the next saṅkrānti; Vibhava 3.3 days and Śukla 364.3 days after the saṅkrānti of Śaka 1779 current; the worked example of 2024 | `the_moment_follows_the_rule_through_an_expunged_year`, `vibhava_begins_three_days_after_the_sankranti_of_1779`, `pingala_gives_way_to_kalayukta_a_fortnight_into_saka_1946` | the saṅkrānti of 1514 1½ minutes from the printed one, Vṛṣa's end within three minutes, the rest to the tenth of a day |
| The Sun's twenty-seven nakṣatra entries of 2025 | `nakshatra::the_suns_nakshatra_transits_of_2025_are_the_almanacs` | every entry 7 to 10½ minutes before Drik Panchang's, the spread under two minutes |
| The thirty-two yoga ends and fifty-nine karaṇa ends Drik Panchang prints for New Delhi, 1 to 30 January 2025 | `panchanga::the_yogas_of_january_2025_end_when_drik_panchang_says`, `the_karanas_of_january_2025_end_when_drik_panchang_says` | every yoga end from 56 seconds before the printed minute to 14 seconds after; every karaṇa end 0.6 to 1.6 minutes after it — the pages appear to truncate to the minute, and the yoga carries the two ayanāṃśas' 20″ twice |
| The karaṇa names Table VIII, cols. 4 and 5, prints for śukla 1 (Kiṃstughna, Bava), śukla 2 (Bālava, Kaulava), kṛṣṇa 14 (Viṣṭi, Śakuni) and amāvāsyā (Catuṣpada, Nāga), and the seven movable names eight times between; forty yogas each within Art. 9's 20 h 52 m 48 s to 24 h 36 m 24 s, widened for the true Moon | `panchanga::the_sixty_halves_take_the_names_table_viii_prints`, `a_yoga_lasts_about_a_day` | all; the yogas between 0.85 and 1.06 days |
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

**What the sample dates show.** `dates.l` holds the book's 33 sample
dates and the program that writes their tables, not the tables, so the
values were computed from the code itself and are carried with its
licence and the method in the data file's header
(`crates/hyper-calendar/tests/data/calendrica_sample_dates.txt`); the
printed appendix was not read. The Old Hindu calendars agree on every date
from 586 BCE, and so does the book's modern solar calendar,
`hindu-solar-surya-siddhanta`, which converts each back, a check of the
*Sūrya Siddhānta*'s Sun, its sunrise and the book's search for a month's
first day against the code's own. The book's astronomical
calendars differ four times, and all four for one reason, which the
book's errata give: the code reads the sign or the tithi at Ujjain's
sunset or sunrise in standard time, as `dawn` and `dusk` return it, but
passes that moment where Universal Time is wanted, five hours and three
minutes late; the errata say the calendars "work as advertised" only with
the conversion to Universal Time [reingold2018errata, correction 15]. The
saṅkrāntis of 1819, 1903 and 2038 fell 3.6, 1.9 and 4.1 hours after
Ujjain's sunset, so the Tamil rule begins those months the next day, as
this library does, and the code the same day. In 2094 the code's own
functions give tithi 5 at the sunrises of 17 and 18 July, the second a
repeated day, as this library does, and tithi 6 five hours later, which
is what the code prints. The book's third lunisolar column, its modern
calendar on the *Sūrya Siddhānta*'s Moon as well as Sun, is
`hindu-lunar-surya-siddhanta`, and it agrees on all 33 dates from 586 BCE,
leap months, a repeated tithi in 1560 and all, and converts each back;
since that calendar computes no astronomy of `hc-astro`'s, the agreement
is a check of the Siddhānta's Moon, its sunrise and the months' engine
against the code's own.

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
| [crc1955] | The Committee, its dates and members; the Central Station; tithis by modern computation; the lunar month named after the solar month of its new moon, adhika and śuddha; the 13°20′ nakṣatra divisions and the Sun's entries; the ayanāṃśa of 23°15′ on 21 March 1956; the solar-month conventions left to the pandits; the list of almanacs; Annexure VI, the questionnaire and the replies of the four almanacs that compute by the *Sūrya Siddhānta* (replies 5, 43, 47, 48, pp. 24, 30, 31), and the *bīja* the calendar makers apply to the Siddhāntas' Moon (p. 3); the General Rules for Religious Festivals and Naraka Caturdaśī's four ghaṭikās before sunrise | Yes, 2026-09-25, in the Internet Archive's OCR text; Annexure VI and p. 3 on 2026-09-27, and the festival rules the same day, in the same text as saved |
| [wikipedia-indian-national-calendar] | The civil calendar's adoption on 22 March 1957 and the Śaka offset | Yes, 2026-09-25 |
| [sewell1896] | The four regional rules and their names; kṣaya and adhika tithis; the naming of adhika and kṣaya months; the intervals between expunged months; the sixty-year cycle, its northern and southern reckonings and the southern rule, and the worked examples of 1752, 1803–04 and 1822; the northern cycle's length and expunction, the name coupled with the year, the three rules with their examples, the list of expunged names and Table I's reckoning; the yoga and its lengths (Art. 9, p. 3), the karaṇa (Art. 10), the karaṇa names and the *Sūrya Siddhānta*'s other order of the fixed four (Art. 40 and its note) and the names of each half-tithi (Table VIII, cols. 4 and 5) | Yes, 2026-09-25, in the Internet Archive's OCR text; Arts. 28, 32, 45, 48 and 50; Arts. 53–62 and the worked examples that name Angiras, Rudhirodgarin and Chitrabhanu on 2026-09-26, and Arts. 54–60, 75 and 120 again that day for the northern cycle; Arts. 9, 10 and 40 and the Art. 40 note on 2026-09-26, and Art. 9's lengths and Table VIII, cols. 4 and 5, on 2026-09-27. The sixty names are read off the OCR of Table I, col. 6, and Table XII, where the diacritics are lost; Art. 60's list off the OCR of its table |
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
| [reingold2018code] | `hindu-epoch`, `arya-solar-year`, `arya-lunar-month`, `old-hindu-lunar-leap-year?`, `hindu-sine-table`, `hindu-sidereal-year`, `hindu-anomalistic-year`, `hindu-true-position`, `ujjain`, `sidereal-start`, `hindu-lunar-station`, and `hindu-lunar-from-fixed` for the day read at sunrise; `hindu-solar-from-fixed`, `astro-hindu-solar-from-fixed`, `astro-hindu-lunar-from-fixed`, `alt-hindu-sunrise`, `astro-hindu-sunset`, `dawn` and `dusk` for the sample dates, and `dates.l`, run at commit `9afc1f3` to compute them; `hindu-sidereal-month`, `hindu-anomalistic-month`, `hindu-synodic-month`, `hindu-lunar-longitude`, `hindu-lunar-phase`, `hindu-lunar-day-from-moment`, `hindu-new-moon-before`, `hindu-calendar-year`, `hindu-solar-era`, `hindu-lunar-era`, `fixed-from-hindu-lunar`, `hindu-sunrise`, `hindu-equation-of-time`, `hindu-ascensional-difference`, `hindu-tropical-longitude`, `hindu-rising-sign`, `hindu-daily-motion` and `hindu-solar-sidereal-difference` for `hindu-lunar-surya-siddhanta`; `hindu-solar-from-fixed`, `fixed-from-hindu-solar`, `hindu-zodiac`, `hindu-calendar-year` and `hindu-solar-era` for `hindu-solar-surya-siddhanta` | Yes, 2026-09-25; `hindu-lunar-from-fixed` 2026-09-26; the sample-date functions, `dates.l` and the Siddhānta's Moon and sunrise 2026-09-27, and the Siddhānta's solar calendar again that day |
| [reingold2018errata] | Correction 15: the astronomical Hindu calendars need their sunrise and sunset converted from standard time to Universal Time | Yes, 2026-09-27, the version of 22 September 2026 |
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
| [drik-day-panchang-2025] | The yoga and karaṇa of every day of 1 to 30 January 2025 for New Delhi, with their end times, in English and in the Hindi edition; the English and Devanagari names | Yes, 2026-09-26 |
| [wikipedia-nityayoga] | The yoga's definition, the twenty-seven names in order, Wikipedia's copy of Sewell and Dikshit's table of lengths, the other system of twenty-eight | Yes, 2026-09-26 |
| [wikipedia-karana] | The karaṇa as half a tithi and the table of the sixty halves' names | Yes, 2026-09-26 |

Statements in the module documentation for which this document names no
source: the modern mean sidereal
year of 365.256 36 days; and that Drik Panchang's Lahiri anchor is about
20″ from the library's, which is inferred from the measured offset. The
Central Station's latitude as "the latitude of Ujjain" is the Committee's
phrase; the module states the coordinates only.

## Code

`crates/hc-calendars-indic/src/hindu_lunar.rs` (`HinduLunarCalendar`,
`MIN_YEAR`, `MAX_YEAR`, `MONTHS`), `amanta.rs` (the month engine the
two lunisolar skies share, `Sky`, `Amanta`), `hindu_lunar_siddhanta.rs`
(`SiddhantaLunarCalendar`, `KALI_SAKA_OFFSET`), `hindu_purnimanta.rs`,
`hindu_solar.rs` (`SankrantiRule`, `SolarModel`, `TAMIL`, `MALAYALAM`,
`BENGALI`, `VIKRAMI`, `ALL`, `samvatsara_of`), `hindu_solar_siddhanta.rs`
(`SiddhantaSolarCalendar`), `samvatsara.rs` (`NAMES`,
`southern_of_saka`), `barhaspatya.rs` (`MeanSignRule`, `SURYA_SIDDHANTA`,
`ARYA_SIDDHANTA`, `SURYA_SIDDHANTA_BIJA`, `northern_of_saka`,
`in_progress_at`), `hindu_old.rs` (`HINDU_EPOCH`,
`ARYA_SOLAR_YEAR`, `ARYA_LUNAR_MONTH`), `tithi.rs`, `nakshatra.rs`,
`panchanga.rs` (`yoga_at`, `yoga_span`, `karana_at`, `karana_name`,
`karana_span`, `YOGA_NAMES`, `KARANA_NAMES`), `surya_siddhanta.rs`
(`SIDEREAL_YEAR`, `ANOMALISTIC_YEAR`, `SIDEREAL_MONTH`,
`ANOMALISTIC_MONTH`, `SYNODIC_MONTH`, `UJJAIN_LONGITUDE_DEGREES`,
`lunar_longitude`, `conjunction_at_or_after`, `sunrise`) and
`places.rs`. Anchors:
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
`the_yogas_of_january_2025_end_when_drik_panchang_says`,
`the_karanas_of_january_2025_end_when_drik_panchang_says`,
`the_table_holds_the_classical_jyas`,
`the_mesha_sankranti_of_2024_is_later_than_the_lahiri_one`,
`the_books_sample_dates_are_reproduced_both_ways` (in both
`hindu_lunar_siddhanta` and `hindu_solar_siddhanta`),
`mesha_1_of_saka_1947_follows_the_siddhantas_sankranti`,
`the_siddhantas_sunrise_moves_two_month_starts_of_thirty_one_years`,
`chaitra_sukla_1_of_saka_1947_worked_by_hand`,
`the_siddhanta_and_the_true_sky_part_on_one_day_in_eight`,
`ujjain_and_the_central_station_part_on_one_day_in_forty`,
`the_epoch_is_friday_18_february_3102_bce_and_opens_year_zero`,
`the_mean_months_of_2024_fall_within_two_days_of_the_true_ones`. The
ayanāṃśas and the sidereal signs are `crates/hc-seasons/src/zodiac/sidereal.rs`,
the month-name traditions `rashi.rs` beside it; the festival rules that
read these calendars are `crates/hc-holiday/src/hindu.rs`.
