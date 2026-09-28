# China's holiday arrangements and working weekends

Backs the `CHINA` table (`CN`) in `hc-holiday`, and through it the Shanghai
and Shenzhen stock exchanges (`XSHG`, `XSHE`), which include it.

## What it is

China's public holidays rest on two kinds of document. The statute, the
全国年节及纪念日放假办法 (Measures for holidays on national festivals and
commemoration days), names the holidays and how many days each carries.
The State Council's General Office then publishes, for every coming year,
关于XXXX年部分节假日安排的通知 (Notice on the arrangement of some holidays
in the year), which turns those days into long holidays by moving the
rest of a Saturday or Sunday to a weekday beside the festival and working
the weekend day instead — 调休, "adjusted rest" — so that a three-day
statutory holiday becomes a seven- or eight-day break bracketed by working
Saturdays and Sundays. A caller who asks whether a given Sunday in China
was a working day cannot answer from the statute; only the year's notice
says.

**The statute.** Promulgated by the 政务院 on 23 December 1949 and revised
four times by State Council decree [gov-cn-holiday-measures-2024]:

| Revision | Decree | In force | What changed |
| --- | --- | --- | --- |
| 18 September 1999 | 国务院令第270号 | 1999 | Labour Day and National Day to three days each, the origin of the "golden weeks" made by moving the weekends |
| 14 December 2007 | 国务院令第513号 | 1 January 2008 | Qingming, the Dragon Boat Festival and the Mid-Autumn Festival added at one day each; the Spring Festival moved to 除夕、正月初一、初二; Labour Day cut to one day; eleven days [gov-cn-holiday-measures-2007] |
| 11 December 2013 | 国务院令第644号 | 1 January 2014 | The Spring Festival back to 正月初一、初二、初三, so the eve is a working day again; still eleven [gov-cn-holiday-measures-2013] |
| 10 November 2024 | 国务院令第795号 | 1 January 2025 | The eve restored, so the Spring Festival is four days (除夕、正月初一至初三), and 2 May added to Labour Day: thirteen days. A new Article 7 lets the holidays be arranged with 调休 into longer breaks and says that, save in particular cases, the stretch of working days around a statutory holiday should not exceed six [gov-cn-holiday-measures-2024] |

As revised in 2024, Article 2 gives the seven holidays for all citizens:
元旦 (1 January), 春节 (four days), 清明节 (the day of the solar term),
劳动节 (1 and 2 May), 端午节 and 中秋节 (one day each in the Chinese
calendar), and 国庆节 (1 to 3 October). Article 3 gives holidays to some
citizens only — half a day for women on 8 March, half a day for those over
fourteen on 4 May, a day for children under fourteen on 1 June, half a day
for serving soldiers on 1 August. Article 4 leaves the festivals of the
ethnic minorities to the local governments of the areas where they live.
Article 5 lists commemorations that carry no day off. Article 6 says that a
holiday for all citizens falling on a Saturday or Sunday is made up on a
working day (补假), and that one for some citizens is not
[gov-cn-holiday-measures-2024].

**The annual notice.** Each notice is numbered 国办发明电〔year〕N号 and dated
from late October to mid-December of the year before — the 2024 notice on
25 October 2023, the 2008 notice on 15 December 2007. For each festival it
gives one sentence of the form "2月10日至17日放假调休，共8天。2月4日（星期日）、
2月18日（星期日）上班": the days off as a continuous span with its length,
and the weekend days worked. The span holds the statutory days, the
make-up days Article 6 owes for statutory days on the weekend, the
weekend days that fall inside it anyway, and the weekdays paid for by the
Saturdays and Sundays worked. A festival that needs no adjustment is
marked 与周末连休, "joined with the weekend", or since 2024 不调休. Since
2024 the notices also encourage employers to use paid annual leave to
lengthen the breaks — the 2024 notice for the eve of the Spring Festival,
9 February, which the statute of the time did not give and the notice
did not order [gov-cn-holiday-notice-2024, gov-cn-holiday-notice-2026].

Three times an arrangement was changed after it was published. In 2015
the State Council itself gave 3 to 5 September off for the 70th
anniversary of the victory of 1945, with Sunday 6 September worked
[gov-cn-victory-day-2015]. In March 2019 the General Office lengthened
that year's Labour Day from 1 May alone to 1 to 4 May, working Sundays
28 April and 5 May [gov-cn-labour-day-change-2019]. On 26 January 2020 it
extended the Spring Festival, then ending on 30 January, to 2 February for
the epidemic [gov-cn-spring-festival-extension-2020].

## How it works

**From the notice to the table.** The `CHINA` table carries the statute as
rules and the notices as data. The statutory days are ordinary rules,
bounded to the years each revision was in force and beginning at 1999:
1 January; 除夕 as the day before the Chinese New Year for 2008 to 2013 and
from 2025; the first and second days of the first month from 1999 and the
third for 1999 to 2007 and from 2014; Qingming as the solar term at the
Chinese meridian from 2008; 1 May, with 2 and 3 May for 1999 to 2007 and
2 May from 2025; the fifth of the fifth month and the fifteenth of the
eighth from 2008; 1 to 3 October. A year before 1999 meets one more rule,
`CN_UNREAD`, `Rule::UNREAD`, a listing that covers no year at all, so that the
engine reports the statutory days of that year as a gap instead of
answering with the 1999 text's days.

Each notice is then two lists. `CN_DAYS_OFF` holds one row per span the
notice gives off — the Gregorian year, the first and last month and day,
and the festival — and `CN_WORKDAYS` one row per weekend day it works. The
spans are entered as the notice prints them, statutory days included:
the rows are under the festival's own name so that the engine, which
drops a second entry of the same name on the same day, lists the
statutory day once. A span that crosses a New Year is split at it and
each part keyed by its own year; a change notice is entered as the
changed span, not beside the original. Both are `Listing`s, spans and
named dates, and each festival's rows are read by a `Rule::Listed` with
`first_year` 2008 and `last_year` 2026, so a year
outside that range is reported as a gap rather than answered with nothing.

**A working day is an entry.** The weekend days worked are entries of
`Kind::Workday`, built with `HolidayRule::workday`, under the name
"Adjusted working day, *festival*" (*festival*调休上班). `Kind::is_day_off`
is false for them, so they never count as holidays and are never lent to a
table that includes China's. `HolidayCalendar::is_designated_workday`
says whether a day is one, and `is_business_day` is

```text
!is_holiday(day) && (!is_weekend(day) || is_designated_workday(day))
```

so a worked Sunday is a business day and business-day arithmetic steps
onto it. The choice of an entry over an exception in the weekend policy
is [ADR 0009](../adr/0009-a-working-day-is-an-entry.md).

**An exchange takes the days off and not the working days.** A table that
includes another receives only the included entries for which
`is_day_off` holds, so Shanghai and Shenzhen close on every day the
arrangement gives off and stay closed on the Sundays it works — which
their own notices list as 周末休市, "weekend closure" — and add the one
day the exchanges close that the arrangement did not: 9 February 2024,
the eve of the Spring Festival [sse-closure-notices, szse-closure-notices].

**Worked example: the Spring Festival of 2024.** The notice of 25 October
2023 [gov-cn-holiday-notice-2024] says

> 二、春节：2月10日至17日放假调休，共8天。2月4日（星期日）、2月18日（星期日）上班。

10 February 2024 is a Saturday and 正月初一; under the 2013 text the
statutory days are 10, 11 and 12 February, two of them on the weekend, so
Article 6 owes two working days. The span 10 to 17 February holds those
three, the four weekdays 13 to 16 February — two owed by Article 6 and two
paid for by the Sundays worked — and Saturday 17 February. In the table
that is the row `(Spring, 2024, 2, 10, 2, 17)` in `CN_DAYS_OFF` and the
rows `(Spring, 2024, 2, 4)` and `(Spring, 2024, 2, 18)` in `CN_WORKDAYS`.
The statutory rules give 10, 11 and 12 February as "Spring Festival" as
well; the engine keeps one entry for each.

Ask the engine about Sunday 4 February 2024 with a calendar built for
`CHINA` and 2024: `is_weekend` is true, `is_designated_workday` is true,
`is_business_day` is true, and `add_business_days` from Friday 2 February
by one lands on it. Friday 9 February, the eve, is a business day: the
notice encouraged employers to give it and did not order it. Friday
16 February is a holiday. Built for `SHANGHAI_STOCK_EXCHANGE` instead,
the same Sunday is not a business day — the working day was not lent and
the weekend stands — 9 February is closed by the exchange's own rule, and
Monday 19 February is the first trading day after the break.

## What is carried

- **The statutory days** as rules across the 1999, 2007, 2013 and 2024
  texts, bounded as above, and beginning at 1999. The 1999 text's days
  are what the later decrees' amendments imply, and no earlier text was
  read — the 1949 text is reported to have given National Day two days —
  so 1949 to 1998 are not carried and a year before 1999 is a reported
  gap for "Statutory holidays" rather than an answer.
- **The arrangements for 2008 to 2026**, nineteen notices, as 135 spans
  and 123 weekend days worked:

  | Year | Notice | Issued | Days off | Weekend days worked | Changed by |
  | --- | --- | --- | --- | --- | --- |
  | 2008 | 国办发明电〔2007〕52号 | 2007-12-15 | 27 | 5 | |
  | 2009 | 国办发明电〔2008〕42号 | 2008-12-04 | 27 | 6 | |
  | 2010 | 国办发明电〔2009〕27号 | 2009-12-08 | 29 | 8 | |
  | 2011 | 国办发明电〔2010〕40号 | 2010-12-10 | 29 | 5 | |
  | 2012 | 国办发明电〔2011〕45号 | 2011-12-06 | 27 | 7 | |
  | 2013 | 国办发明电〔2012〕33号 | 2012-12-08 | 29 | 12 | |
  | 2014 | 国办发明电〔2013〕28号 | 2013-12-11 | 22 | 5 | |
  | 2015 | 国办发明电〔2014〕28号 | 2014-12-16 | 26 | 5 | 国发明电〔2015〕1号, 3 to 5 September off, Sunday 6 September worked |
  | 2016 | 国办发明电〔2015〕18号 | 2015-12-10 | 24 | 6 | |
  | 2017 | 国办发明电〔2016〕17号 | 2016-12-01 | 24 | 5 | |
  | 2018 | 国办发明电〔2017〕12号 | 2017-11-30 | 23 | 6 | |
  | 2019 | 国办发明电〔2018〕15号 | 2018-12-04 | 24 | 7 | 国办发明电〔2019〕3号, Labour Day 1 to 4 May, Sundays 28 April and 5 May worked |
  | 2020 | 国办发明电〔2019〕16号 | 2019-11-21 | 30 | 6 | 国办发明电〔2020〕1号, the Spring Festival to 2 February, so Saturday 1 February is off rather than worked |
  | 2021 | 国办发明电〔2020〕27号 | 2020-11-25 | 31 | 7 | |
  | 2022 | 国办发明电〔2021〕11号 | 2021-10-25 | 31 | 7 | |
  | 2023 | 国办发明电〔2022〕16号 | 2022-12-08 | 27 | 7 | |
  | 2024 | 国办发明电〔2023〕7号 | 2023-10-25 | 28 | 8 | |
  | 2025 | 国办发明电〔2024〕12号 | 2024-11-12 | 28 | 5 | |
  | 2026 | 国办发明电〔2025〕7号 | 2025-11-04 | 33 | 6 | |

  The days-off count is the sum of the spans as carried, changes applied,
  the days of the arrangement for 2019 that fall in December 2018 counted
  with 2019. The 2019 and 2023 arrangements begin on 30 and 31 December of
  the year before, and the 2012 arrangement works 31 December 2011; those
  rows are keyed by 2018, 2022 and 2011.
- **The gap rule.** A year without a notice here — 2007 and earlier, 2027
  and later — is a gap: `HolidayCalendar::is_complete` is false and
  `gaps` names the arrangement. A year before 1999 is a gap for the
  statutory days as well, under the name "Statutory holidays", and lists
  no holiday at all. The notice for 2027 had not been published when this
  was written; the one for 2026 came on 4 November 2025.
- **Not carried, and why:**
  - The arrangements before 2008, including the two days of the 2008
    notice that fall in 2007 — Monday 31 December 2007 off and Saturday
    29 December 2007 worked — because `CN_ARRANGED_FIRST` is 2008 and a
    2007 with only those two rows would look complete when its own
    arrangement is absent; the module says so at the constant. 2007 stays
    a gap, and a caller who asks about its last week gets the gap, not
    the two days.
  - The statute of 1949 to 1998, because the 1949 text and the 1999
    revision were not read; what the table knows of the 1999 text is what
    the 2007 decree amended, which is enough for 1999 to 2007 and says
    nothing about the years before.
  - Before 2008, the commemorations of Article 5 whose dates the
    办法 alone gives, and the "其他" days it leaves unnamed. The five
    named by their dates, 二七纪念日 (7 February), 五卅纪念日 (30 May),
    七七抗战纪念日 (7 July), 九三抗战胜利纪念日 (3 September) and 九一八纪念日
    (18 September), and 护士节, 12 May, which the National Health
    Commission's notices call "国际护士节" [nhc-nurses-day-2024], are
    carried as observances from 2008, the first year of the text read, the
    2007 decision's, which the 2013 and 2024 texts repeat word for word;
    each year before is a gap. 植树节, 12 March, is carried from 1979, by
    the Standing Committee's resolution of 23 February 1979
    [npcsc-arbor-day-1979]; 教师节, 10 September, from 1985, by its decision
    of 21 January 1985 [moe-teachers-day-chronology]; and 记者节, 8 November,
    from 2000, the year the State Council approved the day at the
    中国记协's request [dangyuan-journalists-day]. None gives a day off.
  - Tibet's 藏历新年 and 雪顿节 under Article 4, whose notices read are
    Lhasa's and Nyingchi's: those of prefecture-level cities, which ISO
    3166-2 does not code, and no instrument of the region was reached
    (xizang.gov.cn refused the connection). Inner Mongolia, whose 2024
    notice lists the national days alone and for which no festival day
    off was found; and the other provinces' autonomous prefectures and
    counties, which ISO 3166-2 does not code either. The State Council's
    notice of the 1950s on the 开斋节 and 古尔邦节 of the Muslim
    nationalities, which a Guangzhou notice of 1995 (穗府办〔1995〕14号)
    cites by its spirit, was not found.
  - The employers' leave the notices encourage on the eve of the Spring
    Festival 2024 and around other breaks, which is not a day off.

### The days for some citizens and the autonomous regions' days

Article 3 gives four days to some citizens only, and Article 4 leaves
the minority festivals to the regions where each minority lives. Each is
a rule of `CHINA`, given to a group of people or scoped to a region
([ADR 0011](../adr/0011-a-day-for-one-group-is-a-scoped-rule.md)), so
that the calendar for everyone, nationwide, does not have it:

| Article | Day | For | Effect | Kind | Group or region | From, a gap before |
| --- | --- | --- | --- | --- | --- | --- |
| 3 (一) | 妇女节, 8 March | 妇女 | 放假半天 | `half-day` | `women` | 1999 |
| 3 (二) | 青年节, 4 May | 14周岁以上的青年 | 放假半天 | `half-day` | `youth` | 1999 |
| 3 (三) | 儿童节, 1 June | 不满14周岁的少年儿童 | 放假1天 | `public` | `children` | 1999 |
| 3 (四) | 中国人民解放军建军纪念日, 1 August | 现役军人 | 放假半天 | `half-day` | `military` | 1999 |

A half day is its own kind, which business-day arithmetic counts as a
working day ([ADR 0012](../adr/0012-a-half-day-off-is-its-own-kind.md)).
Article 6 makes up a day for everyone that falls on a weekend and says a
day for some citizens is not made up; the table has no substitution rule
in any case. The first year is the statutory days' own, 1999: the text
of Article 3 is the one the 2007 decision reprints and the 2024
consolidated text keeps, and no decision read changed it. Every year
before 1999 is a gap for each of the four, the 1949 text not having been
read.

The autonomous regions' festivals, as the instruments read give them:

| Region | Instrument | Festival | Days off | Years carried | Otherwise |
| --- | --- | --- | --- | --- | --- |
| 广西 `CN-GX` | 广西壮族自治区少数民族习惯节日放假办法, 令第98号 of 7 January 2014, in force 1 March 2014 [gx-minority-festival-measures-2014]: two days for every citizen of the region, the dates announced each year | 壮族三月三 | 11–12 April 2024 [gx-sanyuesan-2024]; 17–20 April 2026, the 18th and 19th and the 补休 on the Friday and Monday [gx-sanyuesan-2026] | 2024, 2026 | 2014–2023 and 2025 gaps, the notices not read; 2027 on a gap |
| 新疆 `CN-XJ` | 新疆维吾尔自治区少数民族习惯节日放假办法 as amended by 令第174号 of 21 December 2011, in force 1 January 2012 [xj-minority-festival-measures-2011]: 肉孜节 one day and 古尔邦节 three, for the cadres and workers of every nationality in the region | 肉孜节 | 21 April 2023 (the festival on Saturday the 22nd, the day off moved); 10 April 2024; 29–31 March 2025; 20–22 March 2026 [xj-holidays-2023, xj-holidays-2024, xj-holidays-2025, xj-holidays-2026] | 2023–2026 | every year before 2023 a gap, the 1999 办法 not read; 2027 on a gap |
| 新疆 `CN-XJ` | the same | 古尔邦节 | 28–30 June 2023; 17–19 June 2024; 6–10 June 2025; 27–31 May 2026 | 2023–2026 | the same |
| 宁夏 `CN-NX` | no standing instrument found; each year's notice of the 自治区人民政府办公厅, "经自治区党委和政府同意" [nx-holidays-2023, nx-holidays-2024, nx-holidays-2025, nx-holidays-2026] | 开斋节 | none in 2023 and 2026, a Saturday (周末正常休息); 10–11 April 2024; 31 March – 1 April 2025 | 2023–2026 | every year before 2023 a gap; 2027 on a gap |
| 宁夏 `CN-NX` | the same | 古尔邦节 | 29–30 June 2023; 17–18 June 2024; 6 June 2025; 27–28 May 2026 | 2023–2026 | the same |
| 西藏 `CN-XZ` | no instrument of the region reached; the notices of Lhasa (拉萨市) for 2022 to 2026 and Nyingchi (林芝市) for 2026, for 驻市各单位 | 藏历新年, 雪顿节 | — | not carried | a prefecture's days, with no ISO 3166-2 code |
| 内蒙古 `CN-NM` | the notice for 2024, 内政办发电〔2023〕8号, the national days alone | — | — | — | no festival found |

The days are the notices' own, the weekend days inside a span of days off
among them. The Hijri dates the festivals fall on were each year's
announcement, and no rule predicts them; so the regions' days are exact
in the years read and gaps in the others.

## Accuracy

Every notice in the table above, the three change notices and the 2007,
2013 and 2024 decrees were read on gov.cn on 2026-09-25 and compared with
`CN_DAYS_OFF` and `CN_WORKDAYS` row by row. Every span and every weekend
day worked for 2008 to 2026 agrees with its notice. The notices name 125
weekend days worked; the table holds 123, because Saturday 1 February 2020
became a day off under the extension and Saturday 29 December 2007 is
outside the range.

Known points a reader may stumble on:

- The 2014, 2015 and 2017 notices word a Monday off as 补休 (a make-up
  day, for 7 April 2014, 6 April and 22 June 2015 and 2 January 2017)
  rather than as part of a span. Each is carried as a one-day span; they
  are days off either way.
- The 2020 extension notice names only the new end of the holiday,
  2 February, and the return to work on 3 February. That Saturday
  1 February is no longer worked follows from the holiday running through
  it; the notice does not say so in words.
- The tests `shanghai_closes_on_the_days_its_notices_list_from_2014_to_2026`
  and `shenzhen_closes_on_the_days_its_notices_list_from_2015_to_2026`
  compare the exchanges' closures — China's days off plus 9 February
  2024 — with the exchanges' own notices, which is a second reading of
  the arrangement for those years.
- The `sources` string of the table names the four decrees, 第270号,
  第513号, 第644号 and 第795号, and says that 第795号 is the text in force
  and that the 1949 and 1999 texts were not read.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [gov-cn-holiday-measures-2024] | The statute as in force: the seven holidays and thirteen days, Articles 3 to 7, the revision history | Yes, 2026-09-25, the consolidated text in the State Council Gazette; Articles 3, 4 and 6 again 2026-09-29, and Article 5 in it and in the 2007 and 2013 texts the same day |
| [npcsc-arbor-day-1979] | 植树节 on 12 March, by the resolution of 23 February 1979 | Yes, 2026-09-29, a law site's copy (secondary) |
| [moe-teachers-day-chronology] | 教师节 on 10 September, by the decision of 21 January 1985 | Yes, 2026-09-29 |
| [dangyuan-journalists-day] | 记者节 on 8 November, approved by the State Council in 2000 | Yes, 2026-09-29 (secondary) |
| [nhc-nurses-day-2024] | "5月12日是国际护士节" | Yes, 2026-09-29, the Xinjiang commission's repost |
| [gx-minority-festival-measures-2014] | 广西's 壮族三月三, two days for every citizen | Yes, 2026-09-29, on gov.cn |
| [gx-sanyuesan-2024] | 广西's days of 2024 | Yes, 2026-09-29, the notice as reposted by 光明网 |
| [gx-sanyuesan-2026] | 广西's days of 2026 | Yes, 2026-09-29, a press report of the notice; the notice on gxzf.gov.cn timed out |
| [xj-minority-festival-measures-2011] | 新疆's 肉孜节 and 古尔邦节 | Yes, 2026-09-29, the amending decision on gov.cn; the consolidated text, a PDF, not opened |
| [xj-holidays-2023], [xj-holidays-2024], [xj-holidays-2025], [xj-holidays-2026] | 新疆's days of each year | Yes, 2026-09-29 |
| [nx-holidays-2023], [nx-holidays-2024], [nx-holidays-2025], [nx-holidays-2026] | 宁夏's days of each year | Yes, 2026-09-29 |
| [cn-xz-lhasa-notices] | Tibet's notices, a prefecture's, not carried | Yes, 2026-09-29, Lhasa's and Nyingchi's pages; xizang.gov.cn refused |
| [gov-cn-holiday-measures-2013] | The 2013 text's Spring Festival without the eve, eleven days | Yes, 2026-09-25 |
| [gov-cn-holiday-measures-2007] | The 2007 text: the three festivals added, the eve, Labour Day at one day, Article 6's make-up rule | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2008] | The arrangement for 2008, and its two days in December 2007 | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2009] | The arrangement for 2009 | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2010] | The arrangement for 2010 | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2011] | The arrangement for 2011 | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2012] | The arrangement for 2012, with 31 December 2011 worked | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2013] | The arrangement for 2013 | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2014] | The arrangement for 2014 | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2015] | The arrangement for 2015 | Yes, 2026-09-25 |
| [gov-cn-victory-day-2015] | 3 to 5 September 2015 off and 6 September worked | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2016] | The arrangement for 2016 | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2017] | The arrangement for 2017 | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2018] | The arrangement for 2018 | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2019] | The arrangement for 2019, from 30 December 2018 | Yes, 2026-09-25 |
| [gov-cn-labour-day-change-2019] | Labour Day 2019 as 1 to 4 May | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2020] | The arrangement for 2020 as first published | Yes, 2026-09-25 |
| [gov-cn-spring-festival-extension-2020] | The Spring Festival of 2020 to 2 February | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2021] | The arrangement for 2021 | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2022] | The arrangement for 2022 | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2023] | The arrangement for 2023, from 31 December 2022 | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2024] | The arrangement for 2024, the worked example, the eve encouraged and not given | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2025] | The arrangement for 2025, the first under the 2024 text | Yes, 2026-09-25 |
| [gov-cn-holiday-notice-2026] | The arrangement for 2026 | Yes, 2026-09-25 |
| [sse-closure-notices] | Shanghai's closures as the consumer: 周末休市 for the weekend days worked, 9 February 2024 | The 2026 Spring Festival announcement and the list page read 2026-09-25; the yearly notices for 2014 to 2026 as the exchange table cites them, read 2026-09-23 |
| [szse-closure-notices] | Shenzhen's closures, the same days | The 2026 notice read 2026-09-25; the yearly notices for 2015 to 2026 as the exchange table cites them |

The 1999 revision (国务院令第270号) and the 1949 text were not read; what
this document says of them is what the later decrees' amendments imply
and what the table's rules assume. The date 18 September 1999 and the
1949 promulgation are as the 2024 consolidated text's history line gives
them.

## Code

`crates/hc-holiday/src/countries/china_scoped.rs`: the days of Article 3
in `CN_SCOPED_RULES`, each given to its group with `for_groups`, and the
autonomous regions' days in `CN_REGION_RULES`, read from the listing
`CN_REGION_DAYS` and scoped with `in_regions`, and the commemorations of
Article 5 in `CN_COMMEMORATIONS`; `CN_ALL_RULES` joins them
to `CN_RULES`, and is what `CHINA` carries. The groups are
`hc_holiday::group`, and the `Scope` a calendar is evaluated in is in
`rule.rs`. Anchors: `crates/hc-holiday/tests/groups.rs` (each day of
Article 3 in its group's calendar and not in everyone's, 1999, 2008 and
2026; nothing in 1998; a half day a business day and Children's Day a day
off for children) and `crates/hc-holiday/tests/china_regions.rs` (every
region's days in a year read, a region's days its own, the gaps), and
`china_s_article_5_commemorations_are_observances_without_a_day_off` in
`crates/hc-holiday/tests/countries.rs`.

`crates/hc-holiday/src/countries/asia.rs`: the statutory rules in
`CN_RULES`, bounded below by `CN_STATUTE_FIRST` with `CN_UNREAD` for the
years before it; the arrangements in `CN_DAYS_OFF` and `CN_WORKDAYS`,
keyed by the festival names of the module `cn` and read through
`cn_off`, `cn_work` and `cn_arranged`; the table `CHINA`. `Listing` and
`Rule::Listed` are in `rule.rs`. The engine's part is `Kind::Workday`
and `HolidayRule::workday` in `rule.rs`, `is_designated_workday` and
`is_business_day` in `engine.rs`, and the `is_day_off` filter in
`evaluate_with_includes`. The exchanges are `SHANGHAI_STOCK_EXCHANGE` and
`SHENZHEN_STOCK_EXCHANGE` in `exchanges.rs`, sharing `XSHG_RULES`.

Anchors, in `crates/hc-holiday/tests/countries.rs`:
`china_statutory_holidays` (the days across the 2013 and 2024 texts, the
eve and Qingming as working days outside their years),
`china_keeps_each_years_arrangement` (the 2024 Spring Festival of the
worked example, the three changed years, the December 2018 days, the gaps
at 2007 and 2027, 1998 as a gap for the statutory days with nothing
answered and 1999 as answered) and
`a_chinese_working_day_is_a_weekend_day_and_never_a_day_off` (every
`Workday` entry a Saturday or Sunday, a business day and not a holiday;
123 of them). In `crates/hc-holiday/src/engine.rs`:
`a_working_weekend_day_counts_and_is_not_lent`. In
`crates/hc-holiday/tests/exchanges.rs`:
`shanghai_closes_on_the_days_its_notices_list_from_2014_to_2026` and
`shenzhen_closes_on_the_days_its_notices_list_from_2015_to_2026`.
