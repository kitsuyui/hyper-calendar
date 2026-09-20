# hc-seasons

Seasonal subdivisions: what a year is made of once you stop counting months.

A calendar names days. This crate names the *parts of the year* underneath
that naming — the 24 solar terms, the 72 pentads, the Japanese 雑節, the 六曜
cycle, the Moon's phases, and the four seasons under each of the three
definitions that disagree about them. It contains no calendar.

| Module | Covers |
| --- | --- |
| `solar_terms` | 二十四節気, both orderings, the 節気 / 中気 split |
| `pentads` | 七十二候, both the Chinese and the Japanese name sets |
| `zassetsu` | 節分, 彼岸, 社日, 八十八夜, 入梅, 半夏生, 土用 + 丑の日, 二百十日, 二百二十日 |
| `rokuyo` | 六曜: 先勝 友引 先負 仏滅 大安 赤口 |
| `moon_calendar` | phase names, 月齢, illuminated fraction, a month's four principal phases, 十五夜, 十三夜 |
| `seasons` | astronomical, meteorological and East Asian seasons |
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

## Accuracy, and the measurement that proves it

`hc-astro`'s apparent solar longitude is Meeus's low-precision series, good to
about 0.01° — roughly a quarter of an hour of solar motion — with a measured
systematic bias of about **−4.5 minutes**. An event within about ten minutes
of local midnight can therefore be assigned the wrong *day*.

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
to fall inside the window where a −4.5-minute bias could move the date: the
tightest case in the modern record is the autumn equinox of 2012 at 23:49
JST, eleven minutes clear. The tests still measure and print the rate rather
than asserting zero, because the margin is eleven minutes and not a principle;
a companion test asserts that any future disagreement must be a case within
half an hour of midnight JST, so a real regression cannot hide behind the
documented bias. Computing the same holiday in Universal Time instead of JST
would get **89 of those 240 days wrong**, which is what the `Meridian`
argument exists to prevent.

Other accuracy notes:

* Solar term dates inherit the same −4.5-minute bias. Terms are 15 days
  apart, so the *term* is never wrong; only its day, and only at a midnight
  boundary.
* Lunar conjunctions land within about a minute, so month boundaries, phase
  dates, 十五夜 and 六曜 are firmer than the solar-term dates.
* 月齢 and the illuminated fraction are quoted for **local noon**. NAOJ quotes
  月齢 for local midnight, half a day less; `moon_age_at` takes any instant.

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

None of these reference values was produced by this crate.

## Rules expressed as data

`ZassetsuRule` has four shapes and all twenty-one 雑節 are one of them:

| Shape | Used by |
| --- | --- |
| `SolarLongitude(deg)` | the four 土用 entries (27°, 117°, 207°, 297°), 入梅 (80°), 半夏生 (100°) |
| `OffsetFromTerm { term, days }` | 節分 (−1 from a 立 term), the three 彼岸 days (−3, 0, +3 from an equinox) |
| `NightsFromBeginningOfSpring(n)` | 八十八夜 (88), 二百十日 (210), 二百二十日 (220) |
| `NearestStemDay { term, stem }` | 社日, the 戊 day nearest an equinox |

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

## Testing

125 unit tests, 8 integration tests and 7 documentation tests.

```
cargo test -p hc-seasons
cargo clippy -p hc-seasons --all-targets --all-features -- -D warnings
cargo fmt -p hc-seasons -- --check
cargo build -p hc-seasons --no-default-features --features alloc
```
