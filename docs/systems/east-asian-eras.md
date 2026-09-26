# The Chinese and Korean regnal eras

Backs the identifiers `chinese-regnal` and `korean-regnal` in
`hc-calendars-regional`, and the Ming and Qing era table `chinese_regnal::ALL`.

## What it is

A Chinese date was written by reign era and year over the lunisolar
calendar's month and day: 康熙二十三年. From the Ming on, a reign had, with
few exceptions, one era, and the era began with the New Year after the
emperor's accession, *踰年改元*: the year of the accession kept the old
emperor's era to its end [wikipedia-ja-chinese-era-list]. The exceptions are
what make the table more than a list:

- **Mid-year changes.** 天順 was proclaimed in the first month of 1457 on the
  Yingzong Emperor's restoration, ending 景泰 in its eighth year; 泰昌 ran
  from the eighth to the twelfth month of 1620, after 萬曆's forty-eighth
  year; 崇德 replaced 天聰 on 天聰10年4月11日, 15 May 1636, when Hong Taiji
  renamed the state Qing [wikipedia-ja-chinese-era-list, zhwiki-chongde].
- **A restored era.** The Yongle Emperor abolished 建文 and counted 1402 as
  洪武35年, as if his predecessor had never reigned
  [wikipedia-ja-chinese-era-list].
- **An era proclaimed and never kept.** 祺祥, chosen for the Tongzhi
  Emperor in 1861, was replaced by 同治 before it came into use
  [wikipedia-ja-chinese-era-list]. 保慶, which rumour in 1899–1900 gave as the
  era of a planned successor, was never proclaimed [zhwiki-baoqing].
- **Several regimes at once.** Between 1644 and 1683 the Ming's 崇禎, the
  Shun of Li Zicheng with 永昌, the Qing's 順治 and the Southern Ming's
  弘光, 隆武, 紹武 and 永曆 overlap. 弘光 follows 踰年改元: the Hongguang
  Emperor took the throne in Nanjing in the fifth month of 1644 and fixed
  the next year as 弘光元年; 隆武 replaced it from the seventh month of 1645
  [zhwiki-hongguang].

The Qing ended with the abdication on 宣統3年12月25日, 12 February 1912
[wikipedia-ja-xuantong].

Korea dated by the Chinese eras on the lunisolar calendar until the end of
the nineteenth century, and in 1894–1895 by 開國, the count from the founding
of Joseon in 1392 [wikipedia-ja-gaeguk]. On 1 January 1896 it adopted the
Gregorian calendar and an era of its own, 建陽 [kowiki-geonyang]. Two
followed. 光武 was chosen over 慶德 on 14 August 1897; a decree of the 15th
made "this year" 光武元年 and set the proclamation for the 16th, when it was
announced at the altars [sillok-gojong]; the Korean Empire was proclaimed
that October. 隆熙 was chosen over 太始 on 2 August 1907, after the
abdication of Gojong [sillok-sunjong], and ended with the annexation on
29 August 1910 [wikipedia-en-korean-era-name]. Secondary accounts put the
first use of 光武 on the 16th [encykorea-gwangmu] or the 17th
[kowiki-gwangmu], and of 隆熙 on 3 August [kowiki-yunghui].

## How it works

**Chinese.** An era is a row: its characters and pinyin, its dynasty, the
Common Era year in which its first lunisolar year began, the year in which
its last began, the month of a mid-year proclamation where there was one,
and whether it was kept. The *backdated* reading, the one the dynasty's own
records use, gives a lunisolar year to the last kept era of the dynasty
whose span contains it; so a year shared by two eras goes to the later, and
1402 is 洪武, 1457 天順, 1620 泰昌 and 1636 崇德. The year within the era is
the year minus the era's first year, plus one.

The calendar reads the day's lunisolar year, month and day from `chinese`,
the Shíxiàn rules at the Beijing meridian, and then applies that rule for
the Qing.

**Worked example: 1 January 1700.**

1. `chinese` puts the day in the lunisolar year that began in 1699, month
   11, day 12.
2. The last kept Qing era whose span contains 1699 is 康熙, 1662–1722.
3. 1699 − 1662 + 1 = 38, so the day is **康熙38年11月12日**.

**Korean.** The three eras are on the Gregorian calendar, so an era is its
first day, and the era of a day is the last era begun on or before it. Its
year 1 is the Gregorian year it began in, and the change is not backdated:
13 August 1897 is 建陽2年8月13日 and 14 August is 光武元年8月14日.

## What is carried

- **`chinese-regnal`** — the Qing eras over `chinese`, from 1 January 1645,
  where the Shíxiàn calendar begins, to 12 February 1912, the abdication.
  Backdated reading only, which is the reading of the Qing's own records.
- **The era table** — 37 eras of the Ming (1368–1644), the Southern Ming
  (1644–1683), the Shun (1644–1645) and the Later Jin and Qing (1616–1911),
  as year data; `era_of_year` answers the backdated question for any of
  them. The Ming eras are not a calendar, because the Datong calendar the
  Ming kept is not in this workspace.
- **`korean-regnal`** — 建陽, 光武 and 隆熙 on the Gregorian days from
  1 January 1896 to 29 August 1910, each from the day it was chosen;
  `gaeguk_year` gives the 開國 count for any year.
- **Not carried.** The eras before the Ming, with their mid-year changes
  and concurrent regimes; 宣統 kept inside the Forbidden City after 1912 and
  its twelve days of 1917; 洪憲 of 1916 and Manchukuo's 大同 and 康德, which
  ran on the Gregorian calendar and belong to no dynasty here; the
  backdated reading of 光武, under which all of 1897 is 光武元年; the
  Republic of Korea's Dangi count, which is a naming of the Gregorian year
  and is described with `dangi` in [east-asian-lunisolar.md](east-asian-lunisolar.md).

## Accuracy

The day arithmetic under `chinese-regnal` is `chinese`'s, and is as good as
that calendar's agreement with the published tables
([east-asian-lunisolar.md](east-asian-lunisolar.md)); no Qing almanac was
read to measure the Qing days against.
The era boundaries are asserted: the backdated years of 1368, 1402, 1457,
1620, 1636, 1644, 1645, 1683, 1795, 1861 and 1899, the abdication day, and a
round trip every nineteenth day of the Qing range. The Korean eras'
boundaries are asserted on both sides of each change.

The era years come from secondary sources. The dynastic records and the
standard modern table, 方詩銘『中國歷史紀年表』, were not read, and would
replace them. For the Korean eras the annals were read for the days of
choice and decree; the day each era was first written on documents is
given differently by the two secondary sources read, and is not what the
module carries.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [wikipedia-ja-chinese-era-list] | The years of every Ming and Qing era and the months of the mid-year changes | Yes, 2026-09-22 |
| [wikipedia-ja-xuantong] | 宣統3年12月25日 as 12 February 1912 and 11月13日 as 1 January 1912 | Yes, 2026-09-22 |
| [wikipedia-en-chinese-era-names] | The characters and pinyin | Yes, 2026-09-22 |
| [zhwiki-hongguang] | 弘光 fixed in the fifth month of 1644 for 1645, and 隆武 from the seventh month of 1645 | Yes, 2026-09-26 |
| [zhwiki-chongde] | 崇德 from 天聰10年4月11日, 15 May 1636 | Yes, 2026-09-26 |
| [zhwiki-baoqing] | 保慶 as a rumoured era, never proclaimed | Yes, 2026-09-26 |
| [kowiki-geonyang] | 建陽 from 1 January 1896 with the Gregorian calendar | Yes, 2026-09-26 |
| [sillok-gojong] | 光武 chosen on 고종 34년 8월 14일, the decree of the 15th, the proclamation of the 16th; the entry of 1 January 1897 headed 建陽 2년 | Yes, 2026-09-26 |
| [sillok-sunjong] | 隆熙 chosen on 순종 즉위년 8월 2일 | Yes, 2026-09-26 |
| [encykorea-gwangmu] | 光武 in use from 16 August 1897 | Yes, 2026-09-26 |
| [kowiki-gwangmu], [kowiki-yunghui] | 光武 in use from 17 August 1897; 隆熙 from 3 August 1907 | Yes, 2026-09-26 |
| [wikipedia-ja-gaeguk] | 開國 counted from 1392 | Yes, 2026-09-22 |
| [wikipedia-en-korean-era-name] | The sequence of eras and the end in 1910 | Yes, 2026-09-22 |

## Code

`crates/hc-calendars-regional/src/chinese_regnal.rs` (`ChineseEra`,
`Dynasty`, `ALL`, `era_of_year`, `ChineseRegnalCalendar`) and
`korean_regnal.rs` (`KoreanEra`, `ALL`, `era_at`, `gaeguk_year`,
`KoreanRegnalCalendar`). Anchors:
`the_backdated_reading_gives_shared_years_to_the_later_era`,
`every_nineteenth_day_of_the_qing_round_trips`,
`the_three_eras_begin_on_the_days_carried`,
`the_calendar_runs_from_the_gregorian_adoption_to_the_annexation`,
`every_day_round_trips_through_the_calendar_and_its_fields`.
