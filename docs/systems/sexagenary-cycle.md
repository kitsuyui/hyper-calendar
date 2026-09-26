# The sexagenary cycle: stems and branches for year, month, day and hour

Backs the identifier `sexagenary` in `hc-calendars-regional`, and the
module `cycle` of `hc-calendar` with its catalogue `cycle::readings`, which
hold the arithmetic and the spellings.

## What it is

干支 (Chinese *gānzhī*, Japanese *kanshi* or *eto*, Korean *gapja*,
Vietnamese *can chi*): the ten Heavenly Stems 十干 — 甲 乙 丙 丁 戊 己 庚 辛
壬 癸 — paired with the twelve Earthly Branches 十二支 — 子 丑 寅 卯 辰 巳 午
未 申 酉 戌 亥 — to count to sixty and repeat [nao-rekiwiki-kanshi]. East
Asia names four things with it:

- **Days.** The Shang oracle-bone inscriptions already date days by the
  cycle, and the Japanese almanac prints the day's pair after the date;
  at the Meiji calendar reform the pair was ordered kept precisely because
  it counts days continuously across the change [nao-rekiwiki-kanshi].
- **Years.** The year names began as positions of Jupiter and became a
  plain succession of the sixty pairs by the Later Han, which has run ever
  since: the 壬申 war, the 戊辰 war and the zodiac animal on a New Year card
  are all this count [nao-rekiwiki-kanshi].
- **Months** (月建干支), for the calendar's lunar months or for the solar
  months that begin at the twelve sectional terms (節月). A year has twelve
  months, so the month names close every five years, and the year's stem
  determines them [nao-rekiwiki-kanshi].
- **Hours.** The twelve branches name the twelve double hours of the day,
  子 from 23:00 to 01:00 under equal hours [nao-rekiwiki-junishi]; the
  hour's stem follows from the day's.

One stem and one branch each for the year, the month, the day and the
hour of a birth are the 四柱 (four pillars), or 八字 (eight characters), of
Chinese and Japanese fortune-telling (BaZi, 四柱推命). This library computes
the pillars and none of the reading.

## How it works

**The pairs.** Position *i* of the cycle, from 0 for 甲子 to 59 for 癸亥, has
stem `i mod 10` and branch `i mod 12`. Ten and twelve are both even, so a
stem and a branch of different parity never meet, and of the 120 possible
pairs only sixty occur [reingold2018code, `chinese-sexagesimal-name`].
The stems go in yang–yin pairs by phase — 甲 is 木の兄 (*kinoe*, the elder
brother of wood), 乙 木の弟 (*kinoto*, the younger) — which is where the
Japanese readings and the word *eto*, 兄弟, come from [nao-rekiwiki-jikkan].

**The day.** Reingold and Dershowitz name fixed day *n* by pair
`n − 45` counted from 1, so fixed day 46 is 甲子 [reingold2018code,
`chinese-day-name-epoch`, `chinese-day-name`]. Equivalently fixed day −14
is 甲子, and the position of any fixed day *n* is `(n + 14) mod 60`. It
needs no calendar, no place and no astronomy.

**The year, three ways.** The sixty names go in the same order whichever
day the year is taken to begin on, so the three conventions share their
arithmetic and differ only in where the boundary falls:

| Boundary | Position of the year | Used for |
| --- | --- | --- |
| 立春, the start of spring, when the Sun reaches 315°, about 4 February | `(g − 4) mod 60`, *g* the Gregorian year in which that 立春 fell | The four pillars, and the 節月 reckoning |
| 正月初一, the lunisolar new year, in late January or February | `(y − 1) mod 60`, *y* the Chinese year number [reingold2018code, `chinese-year-name`] | The civil and festival year: the animal of 春節, 설날 and Tết |
| 1 January | `(g − 4) mod 60`, *g* the Gregorian year | An approximation, right from 立春 to 31 December |

The two constants agree: 1984 − 4 = 1980 = 33 × 60, and the Chinese year
that began on 2 February 1984, 4621 in Reingold and Dershowitz's count from
2637 BCE, is 4621 − 1 = 4620 = 77 × 60. All three call 1984 甲子, and they
part only between 1 January and whichever of 立春 or the new year comes
later.

**The month: 五虎遁.** The first month of the year is the 寅 month; its
stem depends on the year's stem, and the months then run consecutively.
The National Astronomical Observatory of Japan tabulates the result
[nao-rekiwiki-kanshi]:

| Year stem | 甲 or 己 | 乙 or 庚 | 丙 or 辛 | 丁 or 壬 | 戊 or 癸 |
| --- | --- | --- | --- | --- | --- |
| First month | 丙寅 | 戊寅 | 庚寅 | 壬寅 | 甲寅 |

It is the cycle counted in months without a break: a year advances the
month count by twelve, which is two stems, and two stems is the step from
one column to the next. For the 節月 the month changes at the sectional
term — 寅 at 立春, 卯 at 啓蟄, … 子 at 大雪, 丑 at 小寒. For the lunar months
the same table is read by month number; a leap month is given no pair, or
the pair of the month it repeats [nao-rekiwiki-kanshi].

**The hour: 五鼠遁.** The same argument one level down. A day advances the
hour count by twelve double hours, two stems, so the 子 hour's stem depends
on the day's stem in the same five-column pattern shifted to start at 子:
甲 or 己 days begin with 甲子, 乙 or 庚 with 丙子, 丙 or 辛 with 戊子, 丁 or
壬 with 庚子, 戊 or 癸 with 壬子. In a formula, the 子 hour's stem is
`2 × (day stem mod 5) mod 10`, and the month's 寅 stem
`(2 × (year stem mod 5) + 2) mod 10`, stems counted from 0 for 甲.

**Which day an hour belongs to.** The 子 hour straddles midnight. Under one
convention, 早子時, the day pillar changes with it at 23:00, and a birth at
23:30 takes the next day's pillar and its hour stem; under the other,
夜子時, the day pillar waits for midnight and 23:00–24:00 is a late 子 of
the old day. The library carries both as `ZiHourConvention` and prefers
neither.

**Worked example: 23:30 on 10 February 1985, at 120° E.** Fixed day
724 682.

- *Year.* 立春 1985 fell at 21:11:48 UT on 3 February, 05:11:48 on
  4 February at 120° E; 10 February is after it, so the solar-term year is
  1985, and 1985 − 4 = 1981 ≡ 1 (mod 60): **乙丑**. The 1 January
  convention agrees. The lunisolar year does not: the new year of 乙丑 came
  on 20 February 1985, so 10 February is the 21st day of the twelfth month
  of Chinese year 4621, whose position is 4620 ≡ 0: 甲子.
- *Month.* Between 立春 and 啓蟄, so the first 節月, the 寅 month. The year
  stem is 乙, and 乙庚 years begin with 戊寅: **戊寅**. The lunar month is
  the twelfth month of the 甲子 year, whose first month is 丙寅, so it is
  丙 + 11 = 丁, with 丑: 丁丑.
- *Day.* 724 682 + 14 = 724 696 = 60 × 12 078 + 16: position 16, stem
  16 mod 10 = 6 (庚), branch 16 mod 12 = 4 (辰), 庚辰. The next day is 辛巳.
- *Hour.* 23:30 is in the 子 hour. Under 早子時 the day is already 辛巳, and
  a 辛 day begins with 戊子: pillars 乙丑 戊寅 辛巳 **戊子**. Under 夜子時 the
  day is still 庚辰, and a 庚 day begins with 丙子: pillars 乙丑 戊寅 庚辰
  **丙子**.

So one instant has two defensible charts, and a third answer — 甲子 丁丑
庚辰 — from anyone who takes the year and month from the lunisolar
calendar. In the readings the library carries, the early chart's day is
*xin-si* in toneless pinyin, かのと・み in Japanese kana, 신사 in Hangul and
*Tân Tỵ* in Vietnamese.

## What is carried

- **Identifier** `sexagenary`, in `hc-calendars-regional`: a calendar of
  *days*, the only one of the four that is a function of the fixed day
  alone. A date is the position in the cycle and the number of complete
  sixty-day cycles since fixed day −14, the library's own count, since
  almanacs name the day and never number the run; the `year` field carries
  it, and the range is unbounded.
- **`sexagenary::pillars`**: the year, lunar month and day of a fixed day,
  asking the `chinese` calendar for the year and month, and so inheriting
  its range, 1645 to 2150. The month is the *lunar* month's pair, which is
  what the Chinese calendar counts; for a chart the solar-term month is
  `cycle::month_pillar`.
- **In `hc_calendar::cycle`**: the pair itself, `Sexagenary`, with its
  phase, polarity and English zodiac animal; the day, `sexagenary_day`;
  the three year boundaries as three separately named functions,
  `sexagenary_year` for the lunisolar year, `sexagenary_year_from_solar_term_year`
  with `solar_term_year` and `sexagenary_year_at` for 立春, and
  `sexagenary_year_from_gregorian_year` for 1 January, because a single
  function could not tell from its argument which boundary a caller meant;
  the month, `month_pillar` and `first_month_stem`; the hour, `DoubleHour`,
  `hour_pillar` and `first_hour_stem`; which day an hour belongs to,
  `pillar_day` and `ZiHourConvention`; and `FourPillars`, which derives the
  month and hour stems from the year and day rather than accepting them,
  and checks a transcribed chart with `is_consistent`. The sixty-year
  cycle's ordinal, `sexagenary_year_cycle`, depends on the epoch of the
  year count it is given, and says so.
- **The readings**, one catalogue entry each, ten stems and twelve branches
  apiece, 甲 and 子 first: Han characters; Hanyu Pinyin without and with
  tones; the Japanese 訓読み in kana and romanised; the Japanese 音読み
  romanised; Korean in Hangul and in the Revised Romanization; Vietnamese
  in quốc ngữ. Toneless pinyin is the library's default for English text.
  Adding a reading is adding an entry; `hc-i18n` chooses one per locale.
- **Not carried**: the solar terms, which the month pillar and the 立春
  boundary need and `hc-calendar` does not compute — the caller passes the
  term, from `hc-seasons`; the meridian and time zone a chart is cast on,
  which move the pillars by as much as an hour; the zodiac animals in any
  language but English, which are words of a language, not readings of the
  cycle, and are `hc-i18n`'s; the almanac annotations built on the cycle,
  such as 十二直, which are `hc-almanac`'s; readings not yet written, such
  as Manchu, Mongolian or Tibetan; and every interpretation of a chart.

## Accuracy

The arithmetic is exact. What the tests check against a fact outside the
code:

| Check | Test | Result |
| --- | --- | --- |
| The day cycle's constant agrees with the Julian Day Number form, stem `(JDN + 9) mod 10`, branch `(JDN + 1) mod 12` | `the_day_pillar_agrees_with_the_julian_day_number_rule` | Holds |
| 1 January 1900 is 甲戌, 1 January 1970 辛巳 | `the_first_day_of_nineteen_hundred_was_a_jia_xu_day`, `the_unix_epoch_was_a_xin_si_day` | Holds |
| 1984 is 甲子 and 2024 甲辰 by every convention; the lunisolar new year of 2024, 10 February, is the first day of 甲辰 | `nineteen_eighty_four_is_jia_zi_by_every_route`, `two_thousand_and_twenty_four_is_jia_chen`, `the_year_of_the_wood_dragon_began_in_2024` | Holds |
| 15 January 1984 is still 癸亥 by 立春, though 甲子 by 1 January; 10 February 1985 is 乙丑 by 立春 and still 甲子 by the new year | `a_january_birthday_carries_the_previous_years_pillar`, `the_lunisolar_new_year_and_the_start_of_spring_disagree_for_days_at_a_time` | Holds |
| The first month of a 甲 year is 丙寅, from the table and from the Chinese calendar | `the_tiger_month_stem_follows_the_five_tigers_rule`, `the_first_month_of_a_jia_year_is_bing_yin`, `the_month_pillar_agrees_with_the_lunisolar_month_anchor` | Holds |
| The 子 hour's stem, and five days of hours closing the cycle | `the_rat_hour_stem_follows_the_five_rats_rule`, `five_days_of_hours_are_one_full_cycle` | Holds |
| The two 子-hour conventions give different day and hour pillars at 23:30 | `the_two_zi_schools_give_different_hour_pillars_for_the_same_instant`, `a_late_evening_birth_gets_the_next_days_pillars` | Holds |
| Every reading has 甲子 and 癸亥 where they belong; the kana and romanised kun readings agree; the toned and toneless pinyin agree; only the on readings collide | `every_reading_is_anchored_at_both_ends_of_the_cycle`, `the_kana_and_the_romanised_kun_readings_say_the_same_thing`, `the_toneless_pinyin_is_the_toned_pinyin_with_its_marks_removed`, `only_the_japanese_on_readings_fail_to_tell_the_sixty_pairs_apart` | Holds |

The lunisolar new-year dates in these tests are the `chinese` calendar's,
whose check against the Hong Kong Observatory's tables is in
[east-asian-lunisolar.md](east-asian-lunisolar.md). `hc-seasons` checks the
day stems a third way, through 三伏, whose published dates are defined by
the 庚 days (see [calendars.md](../calendars.md), stage 5).

Checked for this document and not by a test: the module's day constant
is Reingold and Dershowitz's [reingold2018code]; the five first months of
`first_month_stem` are the Observatory's table [nao-rekiwiki-kanshi]; the
Japanese kun and on readings of all ten stems and twelve branches are the
Observatory's, which writes 亥 in the historical kana ゐ where the library
has い [nao-rekiwiki-jikkan, nao-rekiwiki-junishi]; and the double hours
begin at the odd hours from 23:00 [nao-rekiwiki-junishi].

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [nao-rekiwiki-kanshi] | The cycle; its use for days since the Shang, for years since the Later Han, for months; the month table; leap months; the Meiji reform | Yes, 2026-09-26 |
| [nao-rekiwiki-jikkan] | The ten stems, their kun readings as phase and 兄 or 弟, their on readings | Yes, 2026-09-26 |
| [nao-rekiwiki-junishi] | The twelve branches, their kun and on readings, the double hours under equal hours | Yes, 2026-09-26 |
| [reingold2018code] | `chinese-sexagesimal-name`, `chinese-year-name`, `chinese-month-name`, `chinese-day-name-epoch`, `chinese-day-name` | Yes, 2026-09-26 |
| [reingold2018] | The 2637 BCE epoch of the Chinese year count | Not read directly |
| [pinyin-1958] | The pinyin readings | Not re-read here |
| [nikl-stdict] | The Hangul readings, entries 십간 and 십이지 | Not read: the dictionary's pages are rendered by script, and the entries could not be retrieved on 2026-09-26 |
| [rr-korean-2000] | The Revised Romanization readings | Not re-read here |
| Vietnamese almanacs (*lịch vạn niên*) | The quốc ngữ readings | Not read; the module names no edition |

The documentation of the functions and constants makes these statements,
for which no source read here was found:

- That 早子時 is the majority school and 夜子時 the minority (`ZiHourConvention`).
- That the 1 January convention is that of "newspapers, greeting cards,
  most software" (`sexagenary_year_from_gregorian_year` and the table of
  year boundaries).
- That the Julian Day Number rule is "published", and that 1 January 1900
  is 甲戌 "in the published tables"; no publication or table is named.
  Both agree with Reingold and Dershowitz.
- The verses 五虎遁年起月訣 and 五鼠遁日起時訣 as quoted in `cycle`. The
  month table they encode is the Observatory's; the verses' text was not
  found in a source read.
- The classical names of the double hours, 夜半, 雞鳴, 平旦 …, as the names
  "used in Han-period and later texts".
- That the characters of the stems and branches were untouched by the 1946
  Japanese and 1956 Chinese simplifications; that 金 among the five phases
  is read *gon* in the 呉音; that Vietnamese Mão is the standard form of 卯
  and Mẹo the southern one.

## Code

`crates/hc-calendars-regional/src/sexagenary.rs`,
`crates/hc-calendar/src/cycle.rs` and
`crates/hc-calendar/src/cycle/readings.rs`; the month pair of the
lunisolar calendars is `sexagenary_month` in
`crates/hc-calendars-lunar/src/lunisolar.rs`. Anchors in `cycle`:
`the_day_pillar_agrees_with_the_julian_day_number_rule`,
`the_first_day_of_nineteen_hundred_was_a_jia_xu_day`,
`nineteen_eighty_four_is_jia_zi_by_every_route`,
`the_three_year_conventions_share_their_arithmetic`,
`a_january_birthday_carries_the_previous_years_pillar`,
`the_tiger_month_stem_follows_the_five_tigers_rule`,
`the_rat_hour_stem_follows_the_five_rats_rule`,
`the_two_zi_schools_give_different_hour_pillars_for_the_same_instant`,
`the_four_pillars_assemble_from_the_pieces_a_caller_has`,
`a_mis_transcribed_chart_fails_the_consistency_check`; in `readings`,
`every_reading_is_anchored_at_both_ends_of_the_cycle`,
`only_the_japanese_on_readings_fail_to_tell_the_sixty_pairs_apart`; in
`sexagenary`, `the_epoch_is_a_jia_zi_day`,
`the_day_cycle_matches_the_anchor_in_hc_calendar`,
`the_year_of_the_wood_dragon_began_in_2024`,
`the_pillars_helper_inherits_the_chinese_calendars_range`.
