# The Japanese era system

Backs the identifiers `japanese`, `japanese-northern`, `japanese-southern`
and `japanese-proclaimed` in `hc-calendars-regional`, and the era table
`nengo` they read.

## What it is

A Japanese date names an era (元号, nengō), a year within it, a month and a
day: 令和8年9月21日. The year restarts at 1 whenever a new era is
proclaimed, and year 1 is written 元年. The first era is 大化, of 645; 令和,
the 248th, is the one in use [wikipedia-ja-gengo-list].

Three things make the system more than a list of names.

**An era was proclaimed on one day and counted from another.** Up to and
including 明治, an era was proclaimed part way through a lunisolar year and
the chronological tables give it the whole of that year: 安政 was proclaimed
on 嘉永7年11月27日, and the tables treat all of that year as 安政元年. The
days before the proclamation were dated in the old era at the time. The
edict of 明治 said so outright: it was issued on 慶応4年9月8日, 23 October
1868, and made 慶応4年 明治元年 back to its first day, 25 January 1868
[wikipedia-ja-meiji]. From 大正 an era began on the day it was proclaimed
and was not backdated; 明治 officially ended on 29 July 1912 and 大正 began
on the 30th [wikipedia-ja-gengo-list]. The same article sets these 公式
dates beside 改元当時 ones, the dates as they stood at the time: 明治 from
its proclamation, 明治元年9月8日, and from 大正 the changeover day belonging
to both eras, 明治 running to 明治45年7月30日.

**For sixty-one years there were two courts.** From 1331 to 1392 the
Southern Court of Go-Daigo's line and the Northern Court installed by the
Ashikaga each proclaimed eras. 建武 was common to both until the Southern
court replaced it in 1336. The schism ended on 元中9年閏10月5日, 19 November
1392 (Julian), when Go-Kameyama abdicated: 元中 was abolished and the
Northern era 明徳 kept [wikipedia-ja-genchu].

**Twice there was no era at all.** 白雉 lapsed after its fifth year, 655,
and no era was named until 朱鳥 in 686; 朱鳥 lapsed in 687 and none was named
until 大宝 in 701, from which the sequence is unbroken
[wikipedia-ja-gengo-list].

## How it works

An era is a row: its kanji, its reading, the court that proclaimed it, the
day it was proclaimed and the lunisolar year its 元年 occupies. Where the
chronologies disagree about a day, the row says so, and for 文中 only the
month is known [wikipedia-ja-gengo-list].

Converting a day takes two steps. The first is the calendar underneath:
the Gregorian calendar from 1 January 1873, and before it the lunisolar
system that was in force on the day — Tenpō, Kansei, Hōryaku, Jōkyō or
Senmyō, back to 862 ([japanese-lunisolar.md](japanese-lunisolar.md)). That
gives a lunisolar year, month and day. The second is the era, which is where
the readings differ:

- **Backdated**, the reading of the chronological tables: the era of a day
  is the last era whose 元年 is the day's lunisolar year or earlier. The
  year within the era is the day's year minus the 元年's, plus one.
- **Proclaimed**, the reading of a document's dateline: the era of a day is
  the last era proclaimed on or before it; the year number is counted the
  same way.
- From 大正 the two agree, because the eras were not backdated.

The court decides which rows are candidates. Before the schism there is one
stream. Inside it, the Northern and Southern streams each have their own
eras, and the unified reading has no answer. From the reunion every court
reads the Northern stream, because 明徳 is the era that carried on.

**Worked example: 桜田門外の変.** Take 24 March 1860, the day Ii Naosuke
was assassinated outside the Sakurada Gate.

1. The calendar in force was Tenpō-reki. Its year 1860 began on 23 January
   1860, and 24 March is sixty-one days later; the first two months have
   fifty-nine days between them, so the day is the third of the third
   month.
2. 万延 was proclaimed on the eighteenth of that month, 8 April 1860, and its
   元年 is the lunisolar year 1860 [wikipedia-ja-gengo-list].
3. Backdated: the last era whose 元年 is 1860 or earlier is 万延, and 1860 −
   1860 + 1 = 1, so the day is **万延元年3月3日**.
4. Proclaimed: the last era proclaimed by 24 March is 安政, whose 元年 is
   1854, and 1860 − 1854 + 1 = 7, so the day is **安政7年3月3日**, the date
   the narrative histories give.

The same steps for 1 May 1868 give 明治元年4月9日 backdated and 慶応4年4月9日
proclaimed, and for 19 November 1392 (Julian) in the Southern stream give
明徳3年 rather than a 元中 the court had abolished.

## What is carried

- **`japanese`** — the unified stream, backdated. It refuses the days from
  1331-09-11 to 1392-11-18 (Julian) with `UnknownEra`, since two courts
  answered them.
- **`japanese-northern`**, **`japanese-southern`** — each court's stream,
  backdated, answering inside the schism; both read the Northern stream
  from the reunion.
- **`japanese-proclaimed`** — the unified stream with each era from its
  proclamation.
- **Range.** 862-02-07, the first day of Senmyō-reki, to 9999-12-31. The
  eras reach back to 645, and `nengo::era_at` names the era in force, or
  none, on any day from then; the calendar refuses a day before 862 because
  no lunisolar system before Senmyō-reki is implemented.
- **The era table** — all 248 eras with kanji, reading, Hepburn
  romanisation and identifier, court, proclamation day in Julian or
  Gregorian as the source gives it, 元年, how firmly the day is known, and
  the two lapses. `nengo::era_at` answers under the proclaimed reading.
- **Constructible but not registered.** `JapaneseCalendar::with_reckoning`
  also builds the Northern and Southern streams read as proclaimed, with the
  identifiers `japanese-northern-proclaimed` and
  `japanese-southern-proclaimed`. They are not in the registry, because a
  registered calendar needs its month names in `hc-i18n`, which does not yet
  list them.
- **Not carried.** The 改元当時 reading's shared changeover day from 大正 on,
  under which 1912-07-30 is both 明治45年 and 大正元年; `japanese-proclaimed`
  follows 改元当時 for 明治 and the 公式 dates from 大正. The eras' own
  documents, the 改元定 and 改元詔書; any calendar before 862.

## Accuracy

The era table is transcribed, and every stored fixed day is checked against
its tabulated Western date, Julian to the start of 天正 and Gregorian from
the end of it (`every_stored_fixed_day_matches_its_tabulated_western_date`,
`the_julian_gregorian_switch_happens_where_the_source_says`). The modern
boundaries, the lapses, the schism's two ends and the reunion are asserted
day by day on both sides.

The era is only as good as the calendar underneath. From 1873 that is exact
integer arithmetic. From 1844 to 1872 it is the Tenpō calendar computed from
modern astronomy, with no table of the promulgated months to measure it
against. Before 1844 each system runs on its own constants and agrees with
the month table transcribed from 内田正男『日本暦日原典』 on 96.4% to 99.1%
of days, depending on the system ([japanese-lunisolar.md](japanese-lunisolar.md)).
Where that table and the computation disagree, the era and year are still
right and the month or day can be one off: the day of the reunion, which
the source gives as 閏10月5日, the Senmyō computation puts on the fifth of
the eleventh month.

The eras whose start the source marks as uncertain — 大化, and the Southern
eras from 建徳 on, for which no promulgation records survive — carry
`Certainty::Disputed` or `Certainty::MonthOnly`, and the table holds the
source's reading of them. 大化 is the clearest case: the source reads it as
proclaimed between 皇極4年6月19日 (17 July 645) and the end of that month
and in effect from 大化元年7月1日, 29 July 645, and the chronologies it
cites give the sixth month, the seventh or the first [wikipedia-ja-taika].

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [wikipedia-ja-gengo-list] | Every era's kanji, reading, court, changeover date in 和暦 and Western form; the lapses; the 公式 and 改元当時 dates of the modern eras; 令和 as the 248th era | Yes; the rows of 大化, 元中, 明徳, 応永, 明治 and 大正 re-read 2026-09-26; the chronologies it cites, 『続史愚抄』, 『南朝公卿補任』 and 『七巻冊子』, were not read |
| [wikipedia-ja-meiji] | 明治 proclaimed on 1868-10-23 and backdated to the first day of 慶応4年 | Yes, 2026-09-26 |
| [wikipedia-ja-genchu] | 元中 abolished on 元中9年閏10月5日, 1392-11-19 (Julian), and 明徳 kept | Yes, 2026-09-26 |
| [wikipedia-ja-taika] | The reading of 大化's start and the dates the chronologies give | Yes, 2026-09-26 |
| [uchida1975] | The month table the lunisolar calendars are measured against | Not read; the Japanese Wikipedia era articles transcribe it |
| [wikipedia-ja-meiji-kaireki], [nao-rekiwiki-meiji] | 明治5年太政官布告第337号, the change to the solar calendar | Yes, 2026-09-25 |

## Code

`crates/hc-calendars-regional/src/nengo/mod.rs` (`Nengo`, `Court`,
`Certainty`, `era_at`, `court_on`, `NANBOKUCHO_START`, `NANBOKUCHO_END`),
`nengo/table.rs` (`ALL`) and `japanese.rs` (`JapaneseCalendar`,
`EraReckoning`, `JapaneseDate`). Anchors:
`the_modern_eras_start_on_the_days_the_government_says`,
`the_first_era_is_the_one_whose_start_is_disputed`,
`the_two_court_period_starts_and_ends_where_the_sources_say`,
`the_southern_stream_ends_at_the_reunion`,
`no_era_is_in_force_after_hakuchi_and_shucho_lapse`,
`the_sakuradamon_incident_has_two_correct_dates`,
`meiji_as_proclaimed_begins_on_the_day_of_the_edict`,
`the_southern_calendar_follows_meitoku_from_the_reunion`,
`every_modern_era_boundary_holds_on_both_sides`,
`the_solar_calendar_began_the_day_after_meiji_five_twelfth_month_second`.
