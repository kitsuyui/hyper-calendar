# The 24 solar terms and 72 pentads, and the zodiac conventions

Backs `hc-seasons`: the modules `solar_terms`, `pentads`, `meridian` and
`zodiac` (`tropical`, `sidereal`, `rashi`, `chinese_twelve`), and the type
`Ayanamsa`.

## What it is

**The 24 節気.** The ecliptic, the Sun's apparent path against the stars,
cut into twenty-four arcs of 15° from the March equinox at 0°. The instant
the Sun's apparent longitude reaches a multiple of 15° is a solar term, and
the day that instant falls on is the term's date. East Asian almanacs have
printed the twenty-four since the Han dynasty as the frame of the
agricultural year; they are what makes a lunisolar calendar solar, since the
leap month is the month that contains none of the twelve 中気
[nao-rekiwiki-24sekki, reingold2018].

Counted from 立春 at 315°, the odd-numbered terms — 立春, 啓蟄, 清明, … —
are the 節気 (*jieqi*, sectional terms), and the even-numbered — 雨水, 春分,
穀雨, … — are the 中気 (*zhongqi*, principal terms); the 節気 open the
節月 of the sectional year and the 中気 govern intercalation
[nao-rekiwiki-24sekki]. In longitude the 中気 are the multiples of 30° and
the 節気 the odd multiples of 15°.

| Longitude | Traditional | Shinjitai | Class | Longitude | Traditional | Shinjitai | Class |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 315° | 立春 | 立春 | 節 | 135° | 立秋 | 立秋 | 節 |
| 330° | 雨水 | 雨水 | 中 | 150° | 處暑 | 処暑 | 中 |
| 345° | 驚蟄 | 啓蟄 | 節 | 165° | 白露 | 白露 | 節 |
| 0° | 春分 | 春分 | 中 | 180° | 秋分 | 秋分 | 中 |
| 15° | 清明 | 清明 | 節 | 195° | 寒露 | 寒露 | 節 |
| 30° | 穀雨 | 穀雨 | 中 | 210° | 霜降 | 霜降 | 中 |
| 45° | 立夏 | 立夏 | 節 | 225° | 立冬 | 立冬 | 節 |
| 60° | 小滿 | 小満 | 中 | 240° | 小雪 | 小雪 | 中 |
| 75° | 芒種 | 芒種 | 節 | 255° | 大雪 | 大雪 | 節 |
| 90° | 夏至 | 夏至 | 中 | 270° | 冬至 | 冬至 | 中 |
| 105° | 小暑 | 小暑 | 節 | 285° | 小寒 | 小寒 | 節 |
| 120° | 大暑 | 大暑 | 中 | 300° | 大寒 | 大寒 | 中 |

The traditional column is the 二十四節氣 as Hong Kong and Taiwan print them
[hko-24-solar-terms]; the shinjitai column is the 暦要項's [nao-rekiyoko-2024].
Three of the twenty-four differ between the scripts.

**定気 and 平気.** Two ways of placing the terms have been used. 平気法 (also
恒気法, 常気法) divides the *year* into twenty-four equal intervals of time,
about 15.2 days, from the winter solstice, ignoring the Sun's unequal motion;
定気法 (also 実気法) divides the *ecliptic* into twenty-four equal angles, so
that the intervals vary from about 14 to about 16 days, shortest near
perihelion in January and longest near aphelion in July. The two put the
equinoxes about two days apart. Japan placed the terms by 平気 through
寛政暦, and by 定気 from 天保暦 in 1844 [nao-rekiwiki-24sekki-sadamekata,
nao-rekiwiki-tenpo]; China by 定気 from 時憲暦 in 1645 [zhwiki-nongli].
Everything modern is 定気, and so is everything in this library.

**Who publishes them.** In Japan the National Astronomical Observatory
computes the terms in Japan Standard Time and publishes the dates and times
for the year after next as the 暦要項, in the *Official Gazette* each
February [nao-rekiyoko]; 春分の日 and 秋分の日 are public holidays *because*
the Act on National Holidays defines them as 春分日 and 秋分日, the days of
the equinoxes, and the Observatory's 暦要項 is what fixes them
[egov-shukujitsu-ho, nao-rekiyoko-2024-shukujitsu]. In China the Purple
Mountain Observatory of the Chinese Academy of Sciences computes the
calendar, and since 2017 the national standard GB/T 33661-2017, *Calculation
and promulgation of the Chinese calendar*, drafted by that observatory,
fixes the rules: the terms are the Sun's geocentric apparent longitude at
multiples of 15°, the times are Beijing time, the standard time of 120°E,
and the computation is to be accurate to one second [gbt33661,
ndls-gbt33661, zhwiki-nongli]. The Hong Kong Observatory publishes the same
dates for Hong Kong [hko-conversion-2024].

**The 72 候.** Each term is divided into three 候 of about five days — 初候,
次候, 末候 — and the seventy-two are named after what is supposed to be
happening in nature: 東風解凍, the east wind melts the ice; 魚上氷, the fish
rise to the ice. They were first printed in a calendar under the Northern
Wei 正光暦 and go back, through the 月令 of the *Book of Rites*, to the
*Lüshi Chunqiu* [nao-rekiwiki-72ko]. The list the Chinese calendars carried
is the one in the 時訓解 chapter of the 逸周書, which reads 「立春之日，
東風解凍。又五日，蟄蟲始振。又五日，魚上冰」 and so on round the year
[yizhoushu-shixun]; 宣明暦 brought it to Japan in 862, and it describes the
Yellow River valley, where hawks turn into doves and sparrows enter the sea
and become clams. Shibukawa Harumi rewrote the set for Japan's climate in
貞享暦 as the 本朝七十二候; 宝暦暦 revised it again, and that revision is the
one the 略本暦 printed from 1874 and the one Japanese almanacs still print
[nao-rekiwiki-72ko, wikipedia-ja-72ko]. Twenty-one of the seventy-two are
written the same way in both lists [wikipedia-ja-72ko].

The pentads were placed by 平気, a seventy-second of the year each, until
寛政暦; 天保暦 stopped assigning them dates at all, and the modern practice
of reading them off the Sun's longitude at 5° intervals is a reconstruction
that the Observatory's 暦Wiki describes as one of three candidate methods,
the one the modern 半夏生 uses [nao-rekiwiki-72ko].

**The zodiac.** A zodiac sign is the same division at 30°: twelve arcs, each
two terms wide. Three conventions differ only in where the arcs begin. The
*tropical* signs begin at the March equinox, so their boundaries are exactly
the twelve 中気 — Aries opens at 春分, Cancer at 夏至, Libra at 秋分,
Capricorn at 冬至 [ptolemy-tetrabiblos]. The *sidereal* signs, the rāśi of
Indian astronomy, begin at a point fixed against the stars, which
precession has carried about 24° west of the equinox; the offset is the
ayanāṃśa [crc1955, swisseph]. The Chinese 十二次 begin at 大雪, 255°, so
that each 次 opens at a 節気 and holds a 中気 in its middle. This library
carries all three; the twelve animals of the year are not a division of the
ecliptic and are not here.

## How it works

**The longitude.** `hc-astro` gives the Sun's apparent geocentric longitude
from VSOP87 with nutation and aberration, in the form of Meeus's chapter 25,
good to about 1″ [meeus1998]; the series is evaluated in Terrestrial Time
and the answer converted to Universal Time by ΔT, which `hc-astro` reads
from the USNO's observed values from 1974 to 2026-04-01, one a year,
interpolated [usno-deltat], from the USNO's predictions from there to
2033-10-01, one a quarter [usno-deltat-preds], and from the Espenak–Meeus
polynomials outside both. A term is
found by `solar_longitude_after`: from 1 January of the year, the days to go
are estimated at the mean rate of 365.242 189 ⁄ 360 days per degree, a
bracket of ±5 days is placed round the estimate, and the crossing is
bisected. Every term falls once in every Gregorian year, 小寒 in the first
week of January and 冬至 three weeks before the year ends.

**The day, and the meridian.** The instant is one; the day is not. A term
at 15:16 UT is 23:16 in Beijing and 00:16 the next morning in Tokyo, so the
Chinese and Japanese almanacs put it on different dates. The hour between
15:00 and 16:00 UT is one of twenty-four, and over 1950–2049 the two
meridians disagree on 98 of the 2 400 term dates. Every function in the
crate that returns a day therefore takes a `Meridian`, a fixed offset from
Universal Time with no daylight saving and no history: `Meridian::JAPAN` is
UTC+9, the 135°E meridian the 暦要項 is computed at; `Meridian::CHINA` is
UTC+8, the 120°E standard time GB/T 33661 prescribes [gbt33661,
zhwiki-nongli]; `Meridian::CHINA_BEFORE_1929` is Beijing local mean time,
1397⁄180 hours east, for the years before the 120° standard, as Reingold and
Dershowitz use it [reingold2018]; `Meridian::INDIA` is UTC+5:30, the 82°30′E
meridian of the Calendar Reform Committee, at which the *Indian
Astronomical Ephemeris* computes the saṅkrānti [crc1955,
imd-astronomical-ephemeris]. The library has no default that names a
country: `Meridian::default()` is Greenwich.

**The pentad.** A pentad begins where the Sun's longitude is a multiple of
5°: the first 候 of a term begins with the term, the second 5° later and the
third 10° later. It is the same search at a finer step, and its only
complication is that the Sun stands at about 280° on 1 January, so a
Gregorian year can contain the 280° pentad twice; `pentads_in_year` walks
the year and cannot double-count.

**The zodiac.** A tropical sign is the search at a multiple of 30°, and its
ingress instant is the instant of the 中気 it opens at, to the noise of the
search. A 次 is the search at a multiple of 30° offset by 15°, so its
opening instant is that of a 節気. A sidereal sign subtracts the ayanāṃśa
first: the sidereal longitude is the tropical one less *a*(t), where *a* is
fixed by one value at one epoch and carried to any other date by the IAU
2006 general precession in longitude, the polynomial of Capitaine, Wallace
and Chapront's equation (39), whose leading term is 5028.796 195″ per
century — the familiar fifty arcseconds a year [capitaine2003]. The anchors
are the Swiss Ephemeris's: Lahiri 22.460 148° at JD 2 415 020.0, Raman
21.010 833° and Krishnamurti 21.978 333° at the same epoch, Fagan–Bradley
24.042 044° at JD 2 433 282.5 [swisseph]. Lahiri, also Chitrapaksha, is the
Indian standard, adopted on the Calendar Reform Committee's recommendation,
which fixed it at 23°15′ for 21 March 1956 [crc1955]. `Ayanamsa::new` takes
any other anchor.

**Worked example: 大雪 of 2024.** 大雪 is the term at 255°. At 0h UT on
1 January 2024 the Sun stands near 280°, so it has 335° to go; at
1.0146 days per degree that is about 340 days, which lands the estimate on
5 December, and the bracket runs from 30 November to 10 December. Bisecting
the apparent longitude in that bracket, with ΔT = 69.2 s — the observed
value for the end of 2024, interpolated between the USNO's 1 January
samples of 69.18 s and 69.14 s — added to reach Terrestrial Time, gives

    255° at 2024-12-06 15:16:58 UT.

Read at Tokyo, nine hours east, that is 00:16:58 on 7 December; read at
Beijing, eight hours east, it is 23:16:58 on 6 December. The 暦要項 for
令和6年 prints 大雪 255度 12月07日 0時17分 [nao-rekiyoko-2024], and the Hong
Kong Observatory's table prints Heavy Snow on 2024/12/6
[hko-conversion-2024]: one instant, two dates, both right.

The same instant opens the first pentad of 大雪 — 閉塞成冬 in the Japanese
list, 鶡鴠不鳴 in the Chinese — on 7 December in Japan; the second, at 260°,
begins at 22:24 JST on 11 December and the third, at 265°, at 20:26 JST on
16 December. In the zodiac the instant is 15° into tropical Sagittarius
(240°–270°); it is the opening of the 次 星紀, which runs 大雪 to 小寒 and
holds 冬至; and with the Lahiri ayanāṃśa, 24.2° at that date, the sidereal
longitude is 230.8°, twenty-one degrees into Vṛścika.

## What is carried

- **`solar_terms`**: `SolarTerm`, the 24 terms, indexed internally by
  longitude ⁄ 15 and presented in either `TermOrder` — 春分-first, which is
  the longitude order, or 立春-first, the almanac order — with
  `TermKind::Principal` (中気) and `Sectional` (節気). Names in five
  `Naming` sets: traditional Chinese, Japanese shinjitai, pinyin with tone
  marks, Hepburn romaji, and English glosses on which nothing depends.
  `term_moment` (the UT instant), `term_day` and `term_event` (at a
  meridian), `term_in_effect`, `term_on_day`, `term_beginning_on`,
  `days_into_term`, and `terms_in_year`, which yields the year's 24 in date
  order from 小寒.
- **`pentads`**: `Pentad`, the 72 pentads at 5° each, `PentadPosition`
  (初候, 次候, 末候) and `PentadTradition`, a catalogue of name sets with two
  entries: `CHINESE`, the 時訓解 list, and `JAPANESE`, the list of the 1874
  略本暦, each with 72 names and 72 English glosses. `pentad_moment`,
  `pentad_day`, `pentad_event`, `pentad_in_effect`, `pentad_on_day`,
  `pentad_beginning_on`, `pentads_from` and `pentads_in_year`. Neither
  tradition is a default.
- **`meridian`**: `Meridian`, with `UNIVERSAL`, `JAPAN`, `CHINA`, `KOREA`,
  `INDIA` and `CHINA_BEFORE_1929`, constructors from seconds, hours and a
  longitude, and `day_of`, `midnight`, `noon` and `local_hours`.
- **`zodiac::tropical`**: `TropicalSign`, the twelve signs from Aries at 0°,
  with names in English, Latin, emblem and Japanese 黄道十二宮, the symbols
  U+2648 to U+2653 [unicode-misc-symbols], the element, modality and
  classical domicile ruler of each sign as Ptolemy assigns them — the
  solstitial, equinoctial, solid and bicorporeal classes of *Tetrabiblos*
  I.11, the houses of I.17 and the triangles of I.18 [ptolemy-tetrabiblos] —
  a modern rulership that reassigns three signs, and the fixed
  `conventional_period` dates newspapers print. `ingress_moment`,
  `ingress_day`, `sign_period`, `sign_at_moment`, `degrees_into_sign`,
  `sign_in_effect`, `sign_on_day`, `sign_beginning_on`, `days_into_sign`
  and `signs_in_year`, each returning or containing a `SignPeriod` whose
  `end` is the next sign's `start`.
- **`zodiac::sidereal`**: `Ayanamsa`, an anchor value at an anchor Julian
  date and nothing else, with `LAHIRI`, `RAMAN`, `KRISHNAMURTI` and
  `FAGAN_BRADLEY` as data and `new` for any other; `degrees_at`;
  `SiderealSign`, the twelve rāśi in IAST and Devanagari with their emblems
  and lords; `sidereal_longitude`, `ingress_after`, `ingress_moment`,
  `ingress_day`, `sign_period`, `sign_at_moment`, `sign_in_effect`,
  `sign_on_day`, `sign_beginning_on` and `signs_in_year`, every one taking
  the ayanāṃśa as an argument.
- **`zodiac::rashi`**: the rāśi as a solar month. `Rashi` is
  `SiderealSign`; `SolarMonthTradition` is a catalogue of month-name sets
  with `SANSKRIT`, `TAMIL`, `BENGALI` and `MALAYALAM` entries, each naming
  the rāśi whose month opens its year; `month_name`, `month_number`,
  `sankranti_moment`, `sankranti_day`, `month_on_day`, `month_in_effect`,
  `months_in_year`, and the two festival saṅkrānti `makara_sankranti` and
  `mesha_sankranti`. The month names and year openings are sourced in
  [hindu-calendars.md](hindu-calendars.md).
- **`zodiac::chinese_twelve`**: `ChineseStation`, the 十二次 from 星紀 at
  255°, with traditional, shinjitai, pinyin and gloss names, the
  `opening_term` and `midpoint_term` of each, the matching 十二辰 branch
  read from `hc_calendar::cycle` rather than copied, and the
  `traditional_zodiac_counterpart`, an equation of names and not of arcs.
  `station_moment`, `station_day`, `station_at_moment`,
  `degrees_into_station`, `station_period`, `station_in_effect`,
  `station_on_day` and `stations_in_year`.
- **Range.** The crate imposes none; the instants are good over the era
  `hc-astro` claims, roughly 1000 BCE to 3000 CE, outside which ΔT is the
  limit. The iteration functions assume the term and pentad order of a
  Gregorian year, which holds throughout that era; `sidereal::ingress_moment`
  assumes one saṅkrānti per Gregorian year, which for the Lahiri anchor holds
  from about 1100 CE.
- **Not carried, deliberately.** 平気: the equal division in time that
  every Japanese calendar before 1844 and every Chinese one before 1645
  printed; a term or pentad date for those years must come from the
  calendar's own system (see [japanese-lunisolar.md](japanese-lunisolar.md)).
  Regional pentad lists other than the two above — the Korean 칠십이후, the
  Jōkyō and Hōryaku sets the 暦Wiki tabulates, the variants between
  almanacs — since none has been carried from a source; the catalogue takes
  a third entry when one is. The Javanese *pranata mangsa*, whose two
  available tables disagree by a day throughout and neither of which adds
  up, as the roadmap in [calendars.md](../calendars.md) records. Any
  calendar: the Indian solar calendars that count days and years over the
  rāśi are `hc-calendars-indic`'s, and the lunisolar calendars that the 中気
  intercalate are `hc-calendars-lunar`'s. Any body but the Sun: no Moon
  sign, no ascendant, no chart. Constellation boundaries. Holidays.

## Accuracy

**Against the 暦要項.** The 72 terms of 2024, 2025 and 2026 are compared
with the times the Observatory published for those years, which it gives to
the minute in JST [nao-rekiyoko-2024, nao-rekiyoko-2025, nao-rekiyoko-2026],
by `tests/rekiyoko_solar_terms.rs`, which carries the 72 published minutes.
The observed ΔT table `hc-astro` reads ends on 2026-04-01, the last month
the USNO had observed when it was taken [usno-deltat], so the 54 terms up
to 清明 2026 are computed with the observed ΔT and the 18 after it with
the USNO's predicted ΔT [usno-deltat-preds], and the two groups are
counted apart:

| Measure | Observed ΔT, 54 terms | Predicted ΔT, 18 terms |
| --- | --- | --- |
| Term on the published day | 54 of 54 | 18 of 18 |
| Term at the published minute | 53 of 54 | 18 of 18 |
| A minute earlier than published | 1 | 0 |
| Computed instant less the published minute | −30 s to +29 s, mean +1.1 s | −30 s to +23 s, mean −0.1 s |

The published minute is the instant rounded to the nearest minute, since
the offsets are spread over one minute and not two. The bias is gone: the
one mismatch, 小雪 2025, is an instant the crate places 0.2 s past the
half-minute, and the means of +1.1 s and −0.1 s are inside the 1″, about
24 s, that the series is good to. The Espenak–Meeus polynomial alone would
put the same 72 terms at −36 s to +24 s with means of −4.3 s and −6.4 s
and 64 of them on the published minute, because its ΔT — 73.9 s for 2024,
75.1 s for 2026, a forecast made in 2006 — runs 4.7 to 6.1 s above the
observed 69.2 s and 69.1 s, and a ΔT five seconds too large puts every
Universal Time instant five seconds early. The predictions for 2026, 69.09
to 69.11 s, are within a few hundredths of a second of the observations
they overlap, which is why the second group shows no bias a minute can
see. After the predictions' end in October 2033 the polynomial answers
again, 8.9 s above the last prediction, and a term computed there is
about nine seconds early: still under a tenth of the minute the almanac
prints, and gone when the tables are extended.

**Against the equinox days.** Japan's 240 published equinox days of
1980–2099 — the 1980–2030 table and the published formula that reproduces
it and extends it — are matched in every case by `term_day` at
`Meridian::JAPAN`; computed in Universal Time instead, 88 of them would be
wrong. The tightest case in the span is the autumn equinox of 2012 at 23:49
JST, eleven minutes from midnight. The tests print the rate rather than
asserting zero, and a companion test asserts that any future disagreement
is a case within half an hour of local midnight.

**Against the Hong Kong tables.** The Observatory's conversion tables give
the term dates at UTC+8 without times; the 2024, 2025 and 2026 tables were
read for the four dates on which Beijing and Tokyo disagree in those years —
大雪 2024, 冬至 2025, 雨水 and 芒種 2026 — and `term_day` at `Meridian::CHINA`
gives each of them on the Hong Kong date and at `Meridian::JAPAN` on the
Tokyo date [hko-conversion-2024, hko-conversion-2025, hko-conversion-2026].

**The pentads** have no published instants to compare with — the 暦要項
does not print them — and inherit the terms' accuracy: the first pentad of
each term is asserted to begin on the term's own day for 1990, 2000, 2024
and 2030.

**The zodiac.** The tropical ingresses are the 中気 instants and share their
measurement. The sidereal boundaries carry a second uncertainty on top of
the series: published values for a named ayanāṃśa differ among themselves
by a few tens of arcseconds, and 20″ of solar longitude is about eight
minutes of time, so a saṅkrānti near local midnight can move by that
before the model's own error matters. The tests assert that Makara
Saṅkrānti falls on 14 or 15 January and Meṣa Saṅkrānti on 13 to 15 April in
every year 2015–2030, the window the published dates fall in; the Hindu
document records the panchang's printed ayanāṃśa matched within 10″
[hindu-calendars.md](hindu-calendars.md). The fixed dates newspapers print
for the tropical signs are not a reference but a measurement:
`tests/zodiac_conventional_dates.rs` reports that they disagree with the
computed ingress on 48.6% of sign-years of 2000–2029 at Greenwich, always by
a day early, and fit New York in 1900–1929 best, at 21.7%, where and when
they were settled.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [nao-rekiwiki-24sekki] | The definition; the 節気 / 中気 split counted from 立春 and its use in 節月 and 置閏; 中気-based intercalation as the reason the terms exist; the 暦要項 as where the times are published | Yes, 2026-09-25 |
| [nao-rekiwiki-24sekki-sadamekata] | 平気法 and 定気法: the equal division in time until 寛政暦, the equal division in angle from 天保暦; the 14-to-16-day intervals; the two-day difference at the equinoxes | Yes, 2026-09-25 |
| [nao-rekiwiki-72ko] | The 候 as thirds of a term; the 正光暦, the 月令 and the *Lüshi Chunqiu*; Shibukawa's revision and 宝暦暦's; 平気 division until 寛政暦 and 天保暦's dropping of pentad dates; the three modern methods and 半夏生 at 100°; the three-column table of the 宣明暦以前, 貞享暦 and 宝暦暦以降 sets | Yes, 2026-09-25 |
| [nao-rekiwiki-teiki-eikyo] | 定気 in 時憲暦 and 天保暦, and what it does to the leap month | Yes, 2026-09-25 |
| [nao-rekiwiki-tenpo] | 定気 from 天保暦, 1844 | Yes, 2026-09-25, for the Japanese lunisolar document |
| [nao-rekiyoko] | Publication in the *Official Gazette* each February | Yes, 2026-09-25 |
| [nao-rekiyoko-2024] | The 24 terms of 2024 with their longitudes and JST times, 大雪 at 12月07日 0時17分 among them; the shinjitai names | Yes, 2026-09-25 |
| [nao-rekiyoko-2024-shukujitsu] | 春分の日 3月20日 and 秋分の日 9月22日 of 2024 | Yes, 2026-09-25 |
| [nao-rekiyoko-2025] | The 24 terms of 2025 | Yes, 2026-09-25 |
| [nao-rekiyoko-2026] | The 24 terms of 2026 | Yes, 2026-09-25 |
| [egov-shukujitsu-ho] | 春分の日 as 春分日, 秋分の日 as 秋分日; the law's number | Yes, 2026-09-25, through the e-Gov law API |
| [hko-24-solar-terms] | The twelve major and twelve minor terms; the 15° division; the traditional-character names | Yes, 2026-09-25 |
| [hko-conversion-2024] | The 2024 term dates at UTC+8, Heavy Snow on 2024/12/6 | Yes, 2026-09-25 |
| [hko-conversion-2025] | The 2025 term dates, Winter Solstice on 2025/12/21 | Yes, 2026-09-25 |
| [hko-conversion-2026] | The 2026 term dates, Spring Showers on 2026/2/18 and Corn on Ear on 2026/6/5 | Yes, 2026-09-25 |
| [gbt33661] | The Chinese rules: 定気 at 15° of geocentric apparent longitude, Beijing time at 120°E, one-second precision | Not read; the public copy is a scan without text. Its scope and drafting body from [ndls-gbt33661], its rules from [zhwiki-nongli] |
| [ndls-gbt33661] | The standard's title, dates, scope and the Purple Mountain Observatory as its drafter | Yes, 2026-09-25 |
| [openstd-gbt33661] | The standard's dates and its department, the Chinese Academy of Sciences | Yes, 2026-09-25 |
| [zhwiki-nongli] | The rules as the standard states them: 定気 every 15°, 東經120度 as the standard from 1928, one-second precision, 冬至 as the first term; 時憲暦's adoption of 定気 in 1645; the Hong Kong Space Museum's note on Beijing local time against 120°E | Yes, 2026-09-25 |
| [yizhoushu-shixun] | The 時訓解 text of the 72 pentads, 立春之日東風解凍 to 大寒 | Yes, 2026-09-25, on Wikisource |
| [wikipedia-ja-72ko] | The 略本暦 and 宣明暦 lists side by side, 72 rows; 1874; Shibukawa's 本朝七十二候; the 21 shared names | Yes, 2026-09-25 |
| [yueling-72hou-jijie] | Attributed in the crate README as a source of the Chinese list; the readings 候雁北 and 麦秋至 that the crate's list carries and the 逸周書 does not | Not read |
| [reingold2018] | The 中気 rule; Beijing local mean time as 1397⁄180 hours; the search's mean tropical year | Not re-read for this document; the modules cite it |
| [meeus1998] | The apparent solar longitude, chapter 25, through `hc-astro` | Not read for this document; `hc-astro` cites it |
| [usno-deltat] | The observed ΔT that `hc-astro` reads from 1974-01-01 to 2026-04-01: 69.18 s at 2024-01-01, 69.11 s at 2026-01-01, 69.13 s at 2026-04-01 | Yes, 2026-09-25 |
| [usno-deltat-preds] | The predicted ΔT that `hc-astro` reads after 2026-04-01, to 2033-10-01: 69.09 s at 2026-04-02, 71.25 s at 2033-10-01 | Yes, 2026-09-25 |
| [capitaine2003] | The general precession in longitude, equation (39), 5028.796 195″ per century | Not read directly; the bibliographic record from Crossref, 2026-09-25; the module cites the equation |
| [swisseph] | The four ayanāṃśa anchors | Yes, 2026-09-25, for the Hindu document |
| [crc1955] | The Lahiri ayanāṃśa as the national standard, 23°15′ on 21 March 1956; the 82°30′E meridian | Yes, 2026-09-25, for the Hindu document |
| [imd-astronomical-ephemeris] | The saṅkrānti computed at the Indian meridian | Yes, 2026-09-25, for the Hindu document |
| [ptolemy-tetrabiblos] | I.11, the solstitial, equinoctial, solid and bicorporeal signs; I.17, the houses, Leo to the Sun and Cancer to the Moon and the five pairs; I.18, the four triangles | Yes, 2026-09-25, in Robbins's translation on LacusCurtius |
| [unicode-misc-symbols] | U+2648 ARIES to U+2653 PISCES | Yes, 2026-09-25 |

Statements in the module documentation and the crate README that no
source read here supports, or that the sources contradict, recorded so
that they are not mistaken for sourced:

- That pre-1685 Japanese almanacs used 平気 and later ones did not. The
  暦Wiki has 平気 through 寛政暦 and 定気 from 天保暦, 1844; 貞享暦 was
  定朔、平気. The Chinese date, 1645, is right.
- That the `CHINESE` pentad list is the 逸周書·時訓解 "in traditional
  characters". The names are in Japanese shinjitai, and five differ from
  the 時訓解 text read here: 候雁北 for 鴻雁來 (雨水次候), 麦秋至 for 小暑至
  (小満末候), 野鶏入水為蜃 for 雉入大水爲蜃 (立冬末候), 野鶏始雊 for 雉始雊
  (小寒末候), and 群 for 羣, a variant only. The first two are readings the
  README attributes to 月令七十二候集解, not read here.
- That the `JAPANESE` list is "本朝七十二候 of the 1874 略本暦". The
  Wikipedia article gives 本朝七十二候 as the title of Shibukawa's Jōkyō
  revision; the 暦Wiki's table gives the list the 略本暦 printed as the
  宝暦暦 revision, and it is that list the crate carries.
- The floor formulae for 春分の日 and 秋分の日 "that Japanese references
  give", in `tests/japanese_equinox_days.rs`; no reference is named, and
  none was read here. The 1980–2030 table they reproduce is the
  Observatory's, and the 2024 pair was re-read.
- That the Chinese calendar has been computed at 120°E "since 1929". The
  Chinese Wikipedia dates the rule to 民國十七年, 1928; Reingold and
  Dershowitz to 1929. The Hong Kong Space Museum, quoted there, gives the
  old Beijing meridian as 116°23′E, where the module has 116°25′E from
  Reingold and Dershowitz.
- That Japan has had no summer time since 1951 and China none since 1991.
- That the tropical and sidereal zodiacs "last agreed around the second
  century CE" (`tropical`) and "around 285 CE" (`sidereal`); the second is
  what the Lahiri anchor gives, and the two pages disagree.
- That the conventional zodiac dates were settled in the English-language
  press of the early twentieth century.
- That the 十二次 were originally the stages of Jupiter's circuit; that the
  十二辰 were counted clockwise as a division of the equator; that Chinese
  writing from the Ming dynasty on equates each 次 with a Western sign.
- That the elements fire, earth, air and water are Ptolemy's: his I.18
  names the four triangles by their signs and not by elements. The
  rulerships and the solstitial, solid and bicorporeal classes are his.
- The authorities of the `rashi` month traditions — the Tamil Nadu
  almanac, the West Bengal Bangabda, the Kollam era, the *Rashtriya
  Panchang*'s Punjab column — are sourced in
  [hindu-calendars.md](hindu-calendars.md), not here.

## Code

`crates/hc-seasons/src/solar_terms.rs`, `crates/hc-seasons/src/pentads.rs`,
`crates/hc-seasons/src/meridian.rs` and `crates/hc-seasons/src/zodiac/`
(`mod.rs`, `tropical.rs`, `sidereal.rs`, `rashi.rs`, `chinese_twelve.rs`).
`hc-astro`'s `solar::solar_longitude_after` and `solar::seasonal_event` are
the search.

Anchors in the modules:
`the_terms_of_2024_fall_where_the_japanese_almanac_puts_them`,
`the_terms_of_2000_fall_where_the_japanese_almanac_puts_them`,
`tokyo_and_beijing_do_not_always_agree_on_a_terms_date`,
`the_principal_terms_are_the_multiples_of_thirty_degrees`,
`three_terms_are_written_differently_in_the_two_scripts`;
`the_first_pentad_of_a_term_starts_on_the_terms_own_day`,
`the_seventy_two_pentads_partition_the_year`,
`the_two_traditions_disagree_about_most_of_the_year`;
`one_instant_falls_on_two_days_at_two_meridians`,
`the_pre_1929_chinese_meridian_is_beijing_local_mean_time`,
`the_indian_meridian_is_exactly_five_and_a_half_hours_east`;
`every_sign_opens_at_a_principal_solar_term`,
`a_sign_ingress_is_the_same_instant_as_its_opening_term`,
`the_classical_rulerships_are_symmetric_about_the_two_lights`,
`the_symbols_run_consecutively_from_the_aries_code_point`;
`the_ayanamsa_grows_by_about_fifty_arcseconds_a_year`,
`the_sidereal_longitude_is_the_tropical_one_less_the_ayanamsa`,
`makara_sankranti_falls_on_the_fourteenth_of_january`,
`the_two_festival_sankranti_land_in_mid_january_and_mid_april`;
`a_station_opens_at_a_sectional_term_and_holds_a_principal_one`,
`a_station_opens_at_the_same_instant_as_its_opening_term`.

The measurements, in `crates/hc-seasons/tests/`:
`japanese_equinox_days.rs` —
`the_published_equinox_days_of_1980_to_2030_are_reproduced`,
`the_published_equinox_days_of_1980_to_2099_are_reproduced`,
`every_disagreement_is_a_near_midnight_case`,
`computing_the_holiday_in_universal_time_would_get_it_wrong`;
`zodiac_conventional_dates.rs` —
`the_printed_zodiac_dates_disagree_with_the_sun_and_the_rate_is_reported`,
`the_modern_disagreement_is_almost_always_exactly_one_day_early`,
`the_printed_dates_cannot_be_right_at_every_meridian_at_once`;
`rekiyoko_solar_terms.rs` —
`every_term_of_2024_to_2026_falls_on_the_published_day`,
`inside_the_observed_table_the_terms_are_on_the_published_minute`,
`past_the_observed_table_the_terms_follow_the_predictions`,
`no_term_of_these_years_falls_to_the_polynomial`, which print the
minute-by-minute comparison above under `--nocapture`.
