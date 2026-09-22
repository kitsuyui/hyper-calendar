# hc-seasons

Seasonal subdivisions: what a year is made of once you stop counting months.

A calendar names days. This crate names the *parts of the year* underneath
that naming — the 24 solar terms, the 72 pentads, the Japanese 雑節, the 六曜
cycle, the Moon's phases, the zodiac in its three incompatible divisions, and
the four seasons under each of the three definitions that disagree about them.
It contains no calendar.

| Module | Covers |
| --- | --- |
| `solar_terms` | 二十四節気, both orderings, the 節気 / 中気 split |
| `pentads` | 七十二候, both the Chinese and the Japanese name sets |
| `zassetsu` | 節分, 彼岸, 社日, 八十八夜, 入梅, 半夏生, 土用 + 丑の日, 二百十日, 二百二十日 |
| `rokuyo` | 六曜: 先勝 友引 先負 仏滅 大安 赤口 |
| `moon_calendar` | phase names, 月齢, illuminated fraction, a month's four principal phases, 十五夜, 十三夜 |
| `seasons` | astronomical, meteorological and East Asian seasons |
| `zodiac` | 黄道十二宮: the tropical Western signs, the sidereal rāśi with the ayanamsa, the Indian solar months, and the Chinese 十二次 |
| `lunisolar` | a minimal month/day derivation, here on sufferance — see below |

## A day is not an instant

Every event here starts as an astronomical instant in Universal Time and ends
as a calendar day. The step between them is a choice of meridian, and it
changes answers. Beijing is an hour behind Tokyo, so over 1950–2050 the two
almanacs put a solar term on different dates **99 times out of 2400**. A
lunar month boundary shifts the same way, which is why Chinese and Japanese
new year occasionally differ.

So nothing here guesses. Every function returning an `Rd` takes a `Meridian`,
and `Meridian::JAPAN` (UTC+9) and `Meridian::CHINA` (UTC+8) are the meridians
the respective national almanacs are computed at. `Meridian` is a fixed
offset, not a time zone: no daylight saving, no political history. That is
`hc-tz`'s job.

## One computation, three zodiacs

The crate already found the 24 solar terms as the instants the Sun's apparent
longitude reaches a multiple of 15°. A zodiac sign is the same search at a
multiple of 30°, so `zodiac` adds naming and a zero point and no new
astronomy. The three zero points in use are 15° and 24° apart:

| Division | Zero point | Boundaries |
| --- | --- | --- |
| `zodiac::tropical` | the March equinox | **exactly the twelve 中気** |
| `zodiac::sidereal` | the fixed stars, an *ayanamsa* behind the equinox | about 24° later |
| `zodiac::chinese_twelve` | 大雪 at 255° | **exactly the twelve 節気** |

The first and third rows are identities, not resemblances, and tests assert
them: Aries opens at 春分, Cancer at 夏至, Libra at 秋分, Capricorn at 冬至,
and each 次 opens at a 節気 and holds a 中気 in its middle. A 次 is therefore a
tropical sign rotated back fifteen degrees, and is the same interval as the
節月 of the four-pillar reckoning.

### Why the sidereal one is here

Twenty-four degrees out of thirty is four fifths. **For 293 days of 2024 the
tropical and sidereal signs disagree** — most of tropical Aries is sidereal
Pisces, most of tropical Taurus is sidereal Aries — so a library that shipped
only one of them would be answering "which sign" with one of two answers and
not saying which. `SiderealSign` is also exactly the **rāśi**, and the solar
calendars of Tamil Nadu, West Bengal, Assam, Odisha and Kerala take their
months from it, so `zodiac::rashi` is month names over the same boundaries and
no second algorithm.

The ayanamsa is a parameter. `Ayanamsa::LAHIRI` (Chitrapaksha) is the Indian
government standard adopted on the 1955 Calendar Reform Committee's
recommendation; `RAMAN`, `KRISHNAMURTI` and `FAGAN_BRADLEY` are shipped beside
it, and `Ayanamsa::new` takes any anchor at all, because every scheme in use
is the same IAU 2006 precession from a different anchor and only the anchor is
disputed. Moving from Lahiri to Raman — 1.45° — moves **all twelve** month
boundaries by a day or more.

The Chinese zodiac **animal** of a year is not here and is not a 次. It is the
earthly branch of the sexagenary year, a counting cycle with no angle in it,
and it lives in `hc_calendar::cycle` — `sexagenary_year` and
`Sexagenary::zodiac_animal`. The 次 do carry the matching 十二辰 branch, which
runs *backwards* against them (星紀 is 丑, 玄枵 is 子), and the docs say so
loudly enough that nobody should read a birth animal out of this module.

## Accuracy, and the measurement that proves it

`hc-astro`'s apparent solar longitude is VSOP87, good to about 1″ — under
half a minute of solar motion — and its seasonal events land within the
minute the almanacs round to, with no measured bias. An event within about a
minute of local midnight can therefore still be assigned the wrong *day*.

Japan's 春分の日 and 秋分の日 are the sharpest available test of that, because
the law defines them as "the day of the equinox" and the National
Astronomical Observatory of Japan computes the instant in JST and publishes
the resulting *date* in the *Official Gazette* a year ahead. If this crate and
the Observatory disagree, one of them is wrong about a public holiday.

`tests/japanese_equinox_days.rs` runs that comparison:

| Span | Days compared | Disagreements | Rate |
| --- | --- | --- | --- |
| 1980–2030 | 102 | **0** | **0.00 %** |
| 1980–2099 | 240 | **0** | **0.00 %** |

Zero, not "about right". The reason is that over this span no equinox happens
to fall inside the minute where the model could move the date: the tightest
case in the modern record is the autumn equinox of 2012 at 23:49 JST, eleven
minutes clear. The tests still measure and print the rate rather than
asserting zero, because the margin is eleven minutes and not a principle;
a companion test asserts that any future disagreement must be a case within
half an hour of midnight JST, so a real regression cannot hide behind the
documented bias. Computing the same holiday in Universal Time instead of JST
would get **89 of those 240 days wrong**, which is what the `Meridian`
argument exists to prevent.

Other accuracy notes:

* Solar term dates inherit the same accuracy. Terms are 15 days apart, so
  the *term* is never wrong; only its day, and only when its instant falls
  within a minute of a midnight boundary.
* Lunar conjunctions land within about a minute, so month boundaries, phase
  dates, 十五夜 and 六曜 are firmer than the solar-term dates.
* 月齢 and the illuminated fraction are quoted for **local noon**. NAOJ quotes
  月齢 for local midnight, half a day less; `moon_age_at` takes any instant.
* **Zodiac sign boundaries inherit the same bias.** A sign is 30 days wide, so
  the *sign* is never wrong; only its day, and only when an ingress lands
  within about ten minutes of local midnight. The sidereal boundaries carry a
  second, independent uncertainty on top: published values for a named
  ayanamsa disagree among themselves by a few tens of arcseconds, and 20″ of
  solar longitude is about **eight minutes** of time — many times the
  series' own error. A saṅkrānti near midnight moves for that reason before
  any other.

### The zodiac dates every newspaper prints, measured

Astrology columns print fixed dates — "Aries: March 21 – April 19" — that have
not been recomputed since the early twentieth century.
`TropicalSign::conventional_period` ships them as data and
`tests/zodiac_conventional_dates.rs` compares them against the computed
ingresses, 12 signs a year, and prints the result:

| Meridian | 1900–1929 | 1970–1999 | 2000–2029 | 2070–2099 |
| --- | --- | --- | --- | --- |
| Greenwich | 31.4 % | 25.6 % | **48.6 %** | 90.8 % |
| New York (UTC−5) | 21.9 % | 43.9 % | **68.6 %** | 97.8 % |
| Tokyo (UTC+9) | 66.4 % | 23.3 % | **22.5 %** | 61.7 % |

Percentages are sign-years on which the computed ingress day differs from the
printed date. Three things are worth reading out of that table:

* **The printed dates fit New York best around 1900**, which is where and when
  the convention was settled, and they have got steadily worse there ever
  since — 97.8 % wrong by the 2070s.
* **The day counts are not monotone**, because a day count is a rounded
  number and the rounding depends on the meridian: Tokyo's nine-hour offset
  catches the drift at a different point, so the printed dates fit Tokyo
  *best* in the late twentieth century.
* **What is monotone is the instant underneath.** The mean arrival of the Sun
  against the printed date falls at every meridian in every span, by
  **1.30 days between 1900–1929 and 2070–2099** — about three quarters of a
  day per century. That is not precession of the equinoxes; it is the
  Gregorian calendar. Between the 1900 and 2100 century rules no leap year is
  skipped (2000 was a leap year), so for two hundred years the calendar keeps
  a mean year of exactly 365.25 days and runs slow against the tropical year
  by 0.0078 days annually. 2100 will reset it.

Over 2000–2029 at Greenwich the split is 185 exact against 175 one day early
and nothing ever late or two days early: the drift is entirely one-sided and
entirely small. And a fixed date has no meridian, so no recomputed list could
be right everywhere at once — Greenwich and Tokyo put a sign boundary on
different dates **451 times in 1200 sign-years, 37.6 %**.

## The reference data, and where it came from

* **The 24 terms.** Names in traditional Chinese and in Japanese shinjitai as
  separate columns, because three of the 24 genuinely differ (驚蟄/啓蟄,
  小滿/小満, 處暑/処暑). Pinyin with tone marks and Hepburn romaji.
* **The 72 pentads.** Both sets: the classical Chinese one of 逸周書·時訓解 as
  the 宣明暦 transmitted it, and the 本朝七十二候 of Japan's 1874 略本暦
  revision. **Only 21 of the 72 are written identically**, which is why
  shipping one set and calling it "the 72 pentads" is the usual mistake. The
  Chinese set says hawks turn into doves and sparrows enter the sea and become
  clams; Japan replaced those because they are not observations of Japan.
* **雑節 dates**, **term dates** and the **equinox-day table** are checked
  against the National Astronomical Observatory of Japan's 暦要項.
* **土用の丑の日** is checked against the published eel days for 2015–2025,
  including the years with a 二の丑.
* **中秋の名月 and 十三夜** against the published dates for 2020–2025.
* **Lunar new year** for 2015–2026, which every almanac agrees about, anchors
  the lunisolar derivation.
* **The zodiac signs.** English and Latin names as separate columns, because
  English clipped exactly two of them (*Scorpius* → Scorpio, *Capricornus* →
  Capricorn); the 黄道十二宮 names Japanese almanacs print; the symbols
  U+2648 ♈ to U+2653 ♓, which a test checks against arithmetic rather than
  against a second copy of the table. Element, modality and the Ptolemaic
  domicile rulerships are data; the element is the index modulo four and the
  modality the index modulo three, and because three and four are coprime a
  test can assert that each of the twelve pairs occurs exactly once instead of
  trusting a hand-written table.
* **The rāśi** in IAST with diacritics and in Devanagari, because Meṣa and
  Mesa, Siṃha and Simha are different words. Nine of the twelve emblems are
  word for word the Western ones; the three that differ — Dhanus the bow not
  the archer, Kumbha the pot not the water-bearer, Makara a sea-creature not
  a goat-fish — differ in the same way, by naming the vessel instead of the
  person.
* **The ayanamsa anchors** are the Swiss Ephemeris values, the most widely
  deployed reference implementation, carried forward by IAU 2006 general
  precession (Capitaine, Wallace & Chapront 2003). Makara Saṅkrānti
  (mid-January) and Meṣa Saṅkrānti (mid-April) are checked against the
  published festival dates for 2015–2030.
* **The 十二次** with the traditional-character and shinjitai columns (they
  differ for 實沈/実沈 and 壽星/寿星), pinyin, the matching 十二辰 branch taken
  from `hc_calendar::cycle::readings::PINYIN` rather than copied, and the
  Ming-dynasty equation of each 次 with a Western sign — which the docs label
  an equation of *names*, since under the 定気 rule the arcs are 15° apart.

None of these reference values was produced by this crate.

## Rules expressed as data

`ZassetsuRule` has four shapes and all twenty-one 雑節 are one of them:

| Shape | Used by |
| --- | --- |
| `SolarLongitude(deg)` | the four 土用 entries (27°, 117°, 207°, 297°), 入梅 (80°), 半夏生 (100°) |
| `OffsetFromTerm { term, days }` | 節分 (−1 from a 立 term), the three 彼岸 days (−3, 0, +3 from an equinox) |
| `NightsFromBeginningOfSpring(n)` | 八十八夜 (88), 二百十日 (210), 二百二十日 (220) |
| `NearestStemDay { term, stem }` | 社日, the 戊 day nearest an equinox |

`Ayanamsa` is the same idea in the other module: an anchor value at an anchor
Julian date and nothing else, so Lahiri, Raman, Krishnamurti and Fagan–Bradley
are four rows of data over one precession series, and a fifth is a constructor
call rather than a pull request.

So `zassetsu::day_of` is one `match` over data, and the four 土用 entries being
exactly 18° before their closing term is a *test*, not four magic numbers.

## What this crate deliberately does not do

* **No 平気.** The 5° and 15° divisions here are 定気, arcs of the ecliptic.
  Pre-1685 Japanese and pre-1645 Chinese almanacs divided the year equally in
  *time*, and those give different dates. Not implemented.
* **No calendar.** Nothing here implements `hc_calendar::Calendar`. A solar
  term is not a date system; it is a subdivision that several date systems
  refer to.
* **No holidays.** 春分の日 is a public holiday *because* it is the equinox,
  but the holiday law, the substitute-holiday rules and the national calendar
  live in `hc-holiday`.
* **No time zones.** `Meridian` is a fixed offset with no history.
* **No default meridian, and no default season definition.** A caller who has
  not said which they mean has not decided yet, and a library that decided for
  them would be asserting something it cannot know.
* **Japanese 雑節 only.** China and Korea have their own 雜節 and they are not
  this list.
* **The Sun only, in the zodiac too.** Every sign function here places the
  **Sun**. There is no Moon sign, no planet in a sign, no ascendant, no house
  system and no chart. That matters most for the sidereal side: in Jyotiṣa a
  person's *janma rāśi* is the **Moon's** rāśi, not the Sun's, so
  `sidereal::sign_on_day` is not the birth sign an Indian almanac would give.
  A Moon sign needs `hc_astro::lunar_longitude` minus the ayanamsa, which is
  two lines this crate deliberately does not write, because once it has a Moon
  sign it is being asked for a chart.
* **No constellation boundaries.** A sign is a 30° arc of the ecliptic. The
  IAU constellations are irregular polygons of very different sizes, and the
  Sun passes through thirteen of them — Ophiuchus among them — for between
  seven and forty-five days each. The recurring "there is a thirteenth sign"
  story is a statement about constellations, not about signs, and nothing here
  computes constellation membership.
* **No zodiacal 平気 either.** Like the solar terms, the 30° divisions are
  定気, arcs of the ecliptic, so signs are 29 to 32 days long. Equal-time
  divisions are not implemented.
* **No Indian calendar.** `zodiac::rashi` names the twelve solar months and
  finds their boundaries. It does not number days within a month, number
  years, know the Kollam, Bengali San or Śaka epochs, or implement the
  regional rule for a saṅkrānti that falls late in the day. Those are
  `hc-calendars-regional`'s. Nor is any of it the lunisolar Hindu calendar,
  whose months begin at a new or full moon, or the national civil Śaka
  calendar (CLDR `indian`), which has fixed month lengths tied to the tropical
  equinox.
* **No astrology.** The element, modality and ruling planet are shipped as the
  data of a naming scheme with citations. The crate makes no claim about what
  any of it means, and there is no interpretation, compatibility or forecast
  anywhere in it.
* **No pre-1873 六曜.** The daily six-day cycle is a Meiji-era popularisation;
  earlier forms had different names, order and length. Answers before 1873 are
  extrapolations of the modern rule, not what any surviving almanac says.

## Known gaps

* **`lunisolar` is a stand-in.** 六曜 is a function of the lunisolar month and
  day, and `hc-calendars-lunar` — which will own the real Chinese, Dangi and
  Japanese lunisolar calendars — is being written separately and must not be
  depended on from here. So this crate carries a minimal derivation:
  new moon starts the month, the 中気 it contains numbers it, a month without
  a 中気 is a leap month repeating the previous number. **When
  `hc-calendars-lunar` lands, delete `lunisolar` and re-point `rokuyo` and
  `moon_calendar` at it.**

  What the stand-in does not implement: the leap month is properly the
  *first* 中気-less month after the eleventh, decided by looking at the whole
  year between two winter solstices, and a month can occasionally hold two
  中気 — the case that has made 天保暦's rule formally ambiguous since 1844
  and is why Japan's official calendar has no legal lunisolar definition
  today. This code decides month by month and takes the later 中気 when a
  month holds two. It reproduces every lunar new year 2015–2026 and the 2023
  閏二月, and it puts the winter solstice in month 11 for every year
  1990–2040.
* **社日's tie-break.** When the equinox falls on a 癸 day the two 戊 days are
  exactly five days either side. This crate takes the earlier; sources differ,
  and there is no switch for it.
* **`pentad_moment` near 280°.** A leap year whose 1 January falls just before
  the 280° crossing contains that pentad twice, and `pentad_moment` returns
  the first. `pentads_in_year` walks the year instead and cannot
  double-count.
* **Southern-hemisphere East Asian seasons** are a mechanical flip. The 立
  terms describe the Chinese agricultural year and have no southern form; the
  flip is offered because refusing would be more annoying than useful.
* **No `Calendar` impl, no registry entry.** Deliberate; see above.
* **Ayanamsa anchors disagree at the tens-of-arcseconds level.** The values
  here are the Swiss Ephemeris ones; other published tables for the same named
  ayanamsa differ by a few tens of arcseconds, which is a few minutes of time
  in the Sun's motion and can move a saṅkrānti *day* at a midnight boundary.
  There is no way to resolve this, because the disagreement is about a
  convention and not about the sky, so the anchor is a public field of
  `Ayanamsa` and the README says which one was picked.
* **`sidereal::ingress_moment` assumes one saṅkrānti per Gregorian year.**
  True for the Lahiri anchor from roughly 1100 CE onwards, because on
  1 January the Sun stands near 256° sidereal, far from a boundary. Around the
  year 1000 the ayanamsa was near 10°, which put a boundary on 1 January
  itself, and a Gregorian year near then can hold a saṅkrānti twice or not at
  all. `sidereal::signs_in_year` walks the year instead and cannot
  double-count. This is the same shape of caveat as `pentad_moment` near 280°.
* **The 十二次 / Western sign equation is traditional, not exact.** Under the
  定気 rule a 次 and the sign it is equated with overlap for about half their
  length; the test measures that and finds agreement on roughly half the days
  of a year. Nothing here pretends the two arcs coincide.

## Sources

* Jean Meeus, *Astronomical Algorithms*, 2nd ed., Willmann-Bell 1998, through
  `hc-astro`.
* Edward M. Reingold and Nachum Dershowitz, *Calendrical Calculations*, 4th
  ed., Cambridge 2018 — the Rata Die pivot, the Gregorian arithmetic repeated
  privately in `gregorian.rs`, and the Beijing local-mean-time meridian used
  for Chinese dates before 1929.
* National Astronomical Observatory of Japan, 暦要項 (*Calendar Essentials*),
  published annually in the *Official Gazette* — solar term dates, 雑節 dates,
  and the 春分の日 / 秋分の日 table.
* 国民の祝日に関する法律 (Act on National Holidays), 1948, for the legal
  definition of the two equinox holidays.
* 逸周書·時訓解 and 月令七十二候集解 for the Chinese pentads; the 1874 略本暦
  revision (本朝七十二候) for the Japanese ones.
* Claudius Ptolemy, *Tetrabiblos*, I.17–19, for the domicile rulerships and
  the element and modality assignments the `zodiac::tropical` data reproduces.
* N. Capitaine, P. T. Wallace and J. Chapront, "Expressions for IAU 2000
  precession quantities", *Astronomy & Astrophysics* 412 (2003), equation
  (39) — the general precession in longitude that carries every ayanamsa
  away from its anchor.
* Report of the Calendar Reform Committee, Council of Scientific and
  Industrial Research, Government of India, 1955, for the adoption of the
  Lahiri (Chitrapaksha) ayanamsa as the national standard, and the *Indian
  Astronomical Ephemeris* for the 82°30′E meridian the saṅkrānti are computed
  at. The numerical anchors for the four named ayanamsas are the Swiss
  Ephemeris ones.
* The Unicode Standard, Miscellaneous Symbols block, for U+2648 ♈ to
  U+2653 ♓.

## Testing

Unit tests, integration tests and documentation tests; the zodiac work
accounts for the largest share. Exact counts are not quoted here, because a
number in prose drifts away from the code within a release and says nothing
a reader can use — `cargo test -p hc-seasons` is the authority.

```
cargo test -p hc-seasons
cargo clippy -p hc-seasons --all-targets --all-features -- -D warnings
cargo fmt -p hc-seasons -- --check
cargo build -p hc-seasons --no-default-features --features alloc

# The drift table above, printed:
cargo test -p hc-seasons --test zodiac_conventional_dates -- --nocapture
```
