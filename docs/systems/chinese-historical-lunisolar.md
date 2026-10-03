# The Chinese lunisolar calendars from 104 BCE to 597: eleven systems on their own arithmetic

Backs the identifiers `chinese-taichu`, `chinese-sifen`, `chinese-qianxiang`,
`chinese-jingchu`, `chinese-yuanjia`, `chinese-daming`, `chinese-xinghe`,
`chinese-tianhe`, `chinese-kaihuang`, `chinese-sanji` and `chinese-zhengguang` in
`hc-calendars-lunar`
(`chinese_historical`). The modern rule of 1645 on is in
[east-asian-lunisolar.md](east-asian-lunisolar.md), the Japanese systems that
run on the same engine are in [japanese-lunisolar.md](japanese-lunisolar.md).

## What it is

China did not always compute its calendar from the sky. Between the Han
reform of 104 BCE and the first calendars of the Tang, every calendar in
use was an arithmetic: a month length and a year length, each a ratio of
whole numbers printed in the dynastic history's treatise on calendars
(律曆志), and an epoch, a past instant at which a new moon and a winter
solstice fell together at midnight. Months began at the mean conjunction
(平朔) and the twelve *zhōngqì* (中氣) stood at equal twelfths of the year
(平氣); a month with no *zhōngqì* was the leap month. True conjunctions
(定朔) came in with Fù Rénjūn's 戊寅曆 of 619, were dropped again in 645
and were fixed by Li Chunfeng's 麟德曆 of 665 [wikipedia-zh-dingshuo,
wikipedia-zh-wuyinyuanli, wikipedia-zh-lindeli]; the systems here are the
ones before that, which keep the mean motions.

Which system was in force depended on the state, and from the third century
on several states kept different ones at once:

| System | Constants (歲, 朔) | Who | Ran |
| --- | --- | --- | --- |
| 太初曆 (the 三統曆 is Liu Xin's revision of its text) | 365 385/1539, 29 43/81 | Dèng Píng and Luòxià Hóng for the Han court | 太初元年五月 (104 BCE) to 元和二年 (85), 188 years [hanshu-lulizhi, wikipedia-zh-taichuli] |
| 四分曆 | 365 1/4, 29 499/940 | Biān Xīn (編訢) and Lǐ Fàn (李梵) | The Eastern Han from 元和二年二月四日甲寅 (18 March 85); the Wei to 237; the Shu to its end in 263; the Wu in 222 [hhs-lulizhi-xia, wikipedia-zh-sifen] |
| 乾象曆 | 365 145/589, 29 773/1457 | Liú Hóng | The Wu from 黃武二年正月 (223) to its end in 280 [jinshu-lulizhi, wikipedia-zh-qianxiang] |
| 景初曆 | 365 455/1843, 29 2419/4559 | Yáng Wěi | The Wei from 景初元年 (237), the Jin as the 泰始曆 from 265, the Liu Song to 444 [jinshu-lulizhi, wikipedia-zh-jingchu] |
| 元嘉曆 | 365 75/304, 29 399/752 | Hé Chéngtiān | The Southern Dynasties from 元嘉二十二年 (445) to 510 [songshu-lulizhi, wikipedia-zh-yuanjia] |
| 大明曆 | 365 9589/39491, 29 2090/3939 | Zǔ Chōngzhī | The Liang from 天監九年 (510) to the Chen's end in 589 [songshu-lulizhi-daming, wikipedia-zh-daming] |
| 興和曆 | 365 4117/16860, 29 110647/208530 | Lǐ Yèxīng | The Eastern Wei, 540 to 550 [weishu-lulizhi-xia, wikipedia-zh-xinghe] |
| 天和曆 | 365 5731/23460, 29 153991/290160 | Zhēn Luán | The Northern Zhou, 天和元年 (566) to 宣政元年 (578) [suishu-lulizhi-zhong, wikipedia-zh-tianhe] |
| 開皇曆 | 365 25063/102960, 29 96529/181920 | Zhāng Bīn | The Sui from 開皇四年 (584) to 開皇十六年 (596) [suishu-lulizhi-zhong, wikipedia-zh-kaihuang] |
| 三紀甲子元曆 | 365 605/2451, 29 3247/6063 | Jiāng Jí | The Later Qin, 384 to 417 [jinshu-lulizhi] |
| 正光曆 (the 壬子元曆) | 365 1477/6060, 29 39769/74952 | Lǐ Yèxīng and Zhāng Lóngxiáng | The Northern Wei from 523, the Eastern Wei to 539 and the Western Wei to 558 [weishu-lulizhi-shang, wikipedia-zh-zhengguang] |

Liu Yuk Tung's list of the systems adopted in China by year, which
this document reads the spans against, names more: in the north the 玄始曆
of the Northern Liang and Northern Wei, the 天保曆 of
the Northern Qi, the 大象曆 and 大業曆 of the Sui, and after 619 the
Tang's, the Song's, the Yuan's 授時曆 and the Ming's 大統曆
[liu-chinese-calendar-tables]; those are not carried, and *What is carried*
says why for each.

## How it works

**A system is two ratios and an epoch.** The month and the year are the
lengths the treatise prints, and every conjunction and every winter
solstice follows from them:

- the *n*-th new moon after the epoch falls at epoch + *n* × 朔 days, and the
  month begins on the day that contains it, counted from midnight;
- the *i*-th *zhōngqì* after the epoch falls at epoch + *i* × 歲 / 12 days,
  and belongs to the day that contains it;
- the epoch is a midnight at which a new moon and a *zhōngqì* are together:
  a winter solstice (冬至) in every system but the 元嘉曆, where it is
  雨水, the 中氣 that opens its year. The Chinese Wikipedia says the
  latter's epoch is 「正月朔旦夜半雨水時刻」 [wikipedia-zh-yuanjia];
- a month that contains no *zhōngqì* is the leap month and takes the number
  of the month before it; the other months are numbered by the *zhōngqì*
  they hold, 雨水 making a month the first. The year begins with the
  first month.

Nothing else enters. There is no equation of the Sun or the Moon, no
meridian, no astronomy: a day is the day of the count. The engine is the
one the Japanese systems run on, with the conjunction at its mean and the
terms at their mean.

**An instant on midnight.** Some of the exact instants fall on the stroke of
midnight, every 81st new moon of the 太初曆 and every 940th of the 四分曆,
for example, and the system puts such an instant on the day that begins. A
floating-point product cannot be trusted on that edge, so the epoch carries
a millionth of a day more than its exact value (`BOUNDARY_NUDGE`). Over
the span of every system here the nearest instant that is not on an edge
is more than 9 × 10⁻⁵ of a day from midnight (the 天和曆's, 8 seconds), a
hundred times the nudge. A test compares every month of every
system with an integer computation of the same ratios and finds them the
same.

**Reading an epoch from a treatise.** The treatises state their epoch as a
distance from a past 上元, at which the new moon, the solstice and the
sexagenary day 甲子 began together, and number the cycles after it (紀 or
蔀) by their first days. The code takes what the text says of a named
year and carries it to a day:

1. the treatise says how many years the named year is from the 上元
   (積年), and the cycle of 紀法 or 蔀法 years the epoch begins is the
   remainder: for the 景初曆, 4 046 years counted up to 景初元年 are 4 045
   before it, two 紀 of 1 843 years and 359 more, in the third 紀, whose
   first day is named 甲申;
2. the solstice of that year, 359 years of 365 455/1 843 days after the
   cycle's first day, falls on the day whose sexagenary name is 甲申 plus
   the whole days of the distance, and in December: of the days within a
   fortnight of the 25th it is the one with that name;
3. the cycle's first day is the whole days of the distance before it.

For the 景初曆, 359 years of 673 150/1 843 days are 131 123 days and 1 161/1 843
of a day. The solstice that begins 景初元年 falls on the day named 甲申 plus
131 123, which is 丁未, and the one with that name within a fortnight of 25
December 236 is 23 December. The 紀's first day is 131 123 days before it,
25 December 124 BCE (Julian), a 甲申 day. The 四分曆 gives its epoch in a sentence, the
冬十有一月甲子夜半朔旦冬至 of the 庚辰 year, the forty-fifth of the Han
[hhs-lulizhi-xia]: 25 December 162 BCE. The 太初曆's epoch is the same
sentence in the Han shu, the 十一月甲子朔旦冬至 of 元封七年 before 太初元年
[hanshu-lulizhi]: 25 December 105 BCE; the 乾象曆's 內紀 begins on that day.
The 元嘉曆's 元嘉二十年 is the 231st year of its 甲午紀, from which its 雨水
falls on 20 February 212 [songshu-lulizhi]. The 大明曆 puts its 上元 51 939
years before 大明七年 [songshu-lulizhi-daming]; the model starts one 紀 of
39 491 years later, the nearest instant at which both phases recur. The
興和曆, the 天和曆 and the 開皇曆 count in 蔀 of 16 860, 23 460 and 102 960
years, after which the days, the months and the years all come out whole, and
state their distance to a named year in years [weishu-lulizhi-xia,
suishu-lulizhi-zhong]; the 蔀 beginnings, each of whose names follows from
the days of one 蔀, are the epochs.

No epoch is fitted. The tables the systems are measured against were not
used to find one: the measurement agrees with what the text says.

**Worked example: the leap month of 445.** The 元嘉曆's epoch is 20 February
212, the 甲午 day on which a new moon and 雨水 coincide at midnight. The
month is 22 207/752 days and the year 111 035/304.

1. *The new moon.* The 2 886th after the epoch falls 2 886 × 22 207/752 =
   85 225 101/376 days after it, 101/376 of a day past midnight, on the day
   85 225 days after 20 February 212, 21 June 445 (Julian), 己未. The one
   before, the 2 885th, falls on 22 May, 己丑, 0.74 of a day past midnight.
2. *The terms.* A *zhōngqì* falls every 111 035/3 648 = 30.437 days from the
   epoch. The one before, 夏至, is 2 800 of them on, 232/1000 of a day past
   midnight on 20 June 445; the next, 大暑, 2 801 on, 0.67 of a day past
   midnight on 20 July.
3. *The month.* The month that began on 22 May holds 夏至, on its last day,
   20 June, and is the fifth. The month that begins on 21 June and ends on
   19 July holds no *zhōngqì*, since 大暑 is on 20 July, the day the next
   begins: it is the leap month, 閏五月, and 20 July begins the sixth
   Liu's table has the same month, 21 June 445, as the leap month after
   the fifth, and `yuanjia_years_are_the_tables` holds it.
4. *The year.* The first month is the one with 雨水 in it. The year begins
   on 24 January 445 and the leap month falls in it.

**The year number.** A year is numbered by the proleptic Gregorian year in
which its first month begins, as the Japanese systems number theirs:
the year that begins on 24 January 445 is 445. The sexagenary name of the
year, which the display writes, counts from the same epoch as the modern
Chinese calendar's.

## What is carried

- **The nine identifiers**, in `hc-calendars-lunar::chinese_historical`, one
  module and one calendar each, on the engine of
  `lunisolar::LunisolarCalendar` with `ConjunctionMode::Mean`,
  `SolarTermMode::Mean` and no 進朔. Each is a separate identifier and not a
  parameter of one, because the systems disagree where two were in force at
  once (policy §5): in 223–263 the Wei kept the 景初曆 from 237, the Wu the
  乾象曆 and the Shu the 四分曆, three calendars for one day, and the three
  differ on days of the months.

  | Identifier | From | To | Months |
  | --- | --- | --- | --- |
  | `chinese-taichu` | 20 June 104 BCE, the 五月 of 太初元年, 辛酉 | 17 March 85 | 2 323 |
  | `chinese-sifen` | 18 March 85, 元和二年二月四日甲寅 | 14 February 264 | 2 212 |
  | `chinese-qianxiang` | 18 February 223, 黃武二年正月 | 5 February 281 | 717 |
  | `chinese-jingchu` | 10 February 240 | 23 January 445 | 2 535 |
  | `chinese-yuanjia` | 24 January 445 | 25 January 510 | 804 |
  | `chinese-daming` | 26 January 510 | 9 February 590 | 990 |
  | `chinese-xinghe` | 25 January 540 | 22 January 551 | 136 |
  | `chinese-tianhe` | 6 February 566 | 11 February 579 | 161 |
  | `chinese-kaihuang` | 17 February 584 | 23 January 597 | 160 |
  | `chinese-sanji` | 8 February 384 | 20 February 418 | 421 |
  | `chinese-zhengguang` | 1 February 523 | 23 January 559 | 445 |

  The dates are Julian, the calendar of the sources. The spans of the first
  six abut where one system took over from another in the same state:
  the 四分曆 begins the day after the 太初曆's last, the 元嘉曆 the day after
  the 景初曆's and the 大明曆 the day after the 元嘉曆's. The 四分曆 of 85
  begins on the fourth day of the second month, as the Chinese Wikipedia
  gives it, 「元和二年二月四日甲寅」 [wikipedia-zh-sifen]: both calendars
  count the second month of 85 from the 15th of March, so the day is the
  same in either and the change is not seen in the dates.
- **The range of each is the span it ran**, and every day outside it is
  refused: before it with `BeforeEpoch`, after it with
  `AfterSupportedRange`. The ends are the first and last days of the Chinese
  years of Liu's tables [liu-chinese-calendar-tables] where a state's
  calendar began or ended with a year, and the day the text names where it
  did not, the 四分曆 and 太初曆 above.
- **Names.** The months are 正月 to 十二月 and the leap month is 閏, as for the
  modern calendar; the native language is Chinese (`zh`). The year's
  stem and branch and the related Gregorian year are extra fields, as for
  the modern Chinese calendar.
- **Not carried, and why.**
  - *The Qin and the early Han, 221–104 BCE.* The 顓頊曆 of the Qin put the
    year's first month at 十月 and its leap month at the year's end, 後九月.
    Li Zhonglin's reconstruction from the bamboo calendars unearthed since
    1972 uses the same arithmetic with three epochs and a leap month fixed
    by a 3–3–3–2–3–3–2 pattern in the 章, not by the *zhōngqì*, and a year
    that begins in the tenth month [liu-chinese-calendar-tables, the page on
    the reconstruction, read; Li's paper of 2012 was not read]. The engine
    here numbers months from the first and finds the leap month by the
    terms, so a calendar of that shape is a different structure; it is a
    roadmap row.
  - *The Xin numbering, 9–23.* From 15 January 9 to 2 December 23 the months
    of the Han calendar carry the Xin's names, a month higher than the
    arithmetic's, and the 太初曆 here numbers them the Han way. The days and
    the leap months are the table's in every one of those months; only the
    number differs, in 184 months of the table's. A date in that span is
    the Han's name for the month, not the Xin's.
  - *The Wei's three years, 237–239.* The Wei began the year with the 丑
    month in 景初元年 and put the third month of that year as the fourth
    [jinshu-lulizhi]; the 景初曆 span begins on 10 February 240, after it.
    The days of 237–239 are the system's (the 24 months of those years in
    Liu's table agree) and the numbers are not, so they are refused.
  - *The 興和曆's first year.* Li compiled it in 539 and the Eastern Wei kept
    the 正光曆 until 540, which the table's months agree with: the 興和曆
    reproduces them in 534–539 as well, so the span from 540 is the one the
    sources date.
  - *The other northern systems and the Sui's last.* The 玄始曆's constants
    are on Wikipedia (365 17589/72000, 29 47251/89052) and its epoch is not
    read; the 天保曆's
    constants are in the 隋書 in part (章歲 676, 度法 23 660, 斗分 5 787) and
    its month is not given; the 大象曆's 日法 53 563 and 章歲 448 are in the
    隋書 and its epoch's day name is not; the 大業曆's constants are on
    Wikipedia (365 10363/42640, 29 607/1144) and its text is not in the
    隋書 transcription read; the Northern Liang's 玄始曆 of 412–439, whose
    months the 三紀甲子元曆 gives but for one in 347, was not read. Each is
    a roadmap row naming what is missing.
  - *The true-conjunction systems, 619 on.* The 戊寅曆, 麟德曆, 大衍曆 and
    the Tang's later systems, the Song's, the Yuan's 授時曆 and the Ming's
    大統曆 keep the true conjunction by tables or by interpolation, which
    this engine can carry for the 宣明曆 only as modern astronomy or a
    single sine; their treatises are on Wikisource and were not
    implemented. Roadmap rows.

## Accuracy

The reference is Liu Yuk Tung's *Chinese–Western Calendar Conversion Table*,
which gives the Julian date of the first day of every month with its
leap month, year by year, for 722 BCE to 2200 CE from Zhang Peiyu's
《三千五百年历日天象》 (1997) with its author's corrections [liu-chinese-calendar-tables];
and, for the 太初 years, the Chinese Wikipedia's table [wikipedia-zh-taichu-era].
Neither is independent of the other sources of the calendar, and neither
names the system for a month: they say what the calendar was, and the
arithmetic of each treatise is measured against it. Liu says that he
studied about twenty of the ancient systems and could reproduce the
calendars before 665 by computing with them, and that those data match the
book's almost completely before 619 [liu-chinese-calendar-tables]; the
ratios and epochs he used are not in the pages read.

The tables were read page by page in the browser pane, every year of
the periods named, and compared by script with the integer arithmetic
described above; the figures below are the result. The tests hold a few years
of them as anchors and compare the engine with the arithmetic on every month.

| System | Months compared | Result |
| --- | --- | --- |
| 太初曆 | 11 February 102 BCE to 17 March 85, 2 314 of the span's 2 323 months in Liu's tables, and the 8 months of 104 BCE in the Wikipedia table, which Liu's does not give | Every first day and leap month agrees but one: the table's first month of 85 begins on 13 February, a day before the arithmetic's. The numbers agree but for the Xin numbering of 9–23 (184 months) |
| 四分曆 | The Eastern Han from 18 March 85 to 219, 1 668 months; the Wei 220 and the Shu 221–263 together, 544 months | Every first day, leap month and number: 2 212 of 2 212 |
| | The Wei's own 220–236, 210 months | 210 of 210 |
| | The Wei court's records of 黃初二年 and 三年 in the 晉書: 戊辰 on the 29th of the sixth month of 221, 癸未 on the 15th of the seventh, 丙寅 on the first of the first month of 222, 乙巳 on the 15th of the eleventh | All four; a fifth, 庚申 on the 29th of the eleventh month of 222, falls on the first day of the twelfth here and in the table: the day's name agrees and the number the record gives it is a day short |
| 乾象曆 | The Wu, 223 to 280, 717 months | 715 of 717: two months begin a day from the table's, 244-12 (a day later in the table) and 247-9 (a day earlier), both within 33 minutes of midnight |
| 景初曆 | The Wei 240–265, 322 months; the Jin 265–419, 1 917; the Song 420–444, 309 | 322 of 322; 1 914 of 1 917; 308 of 309. The four that part, in 278, 314 (two) and 430, are a day from the table's. The months of 237–239, 24 of them, agree in day and leap month |
| 元嘉曆 | 445 to 509, 804 months | 804 of 804 |
| 大明曆 | 510 to 589, 990 months | 990 of 990 |
| 興和曆 | The Eastern Wei, 534 to 550, 210 months, of which 136 are in the span | 210 of 210, those of 534–539 included |
| 天和曆 | The Northern Zhou, 566 to 578, 161 months | 161 of 161 |
| 開皇曆 | The Sui, 584 to 596, 160 months | 160 of 160 |
| 三紀甲子元曆 | The Later Qin, 384 to 417, 421 months | 421 of 421; the Northern Liang's 412–439 (347 months), which Liu's list gives the 玄始曆, 346 of 347 |
| 正光曆 | The Northern Wei, 523 to 534, 149 months; the Eastern Wei 534–550 (210) and the Western Wei 535–557 (296); the Northern Zhou's 557–565 | 149 of 149; 210 of 210 and 296 of 296; every month of 557–565 (the 興和曆 gives the same). The Northern Wei's 520–522, 37 months, 36 of 37 |
| Whether the arithmetic answers outside its span | The 興和曆 over the Western Wei 535–557 (296 months, 296); the 開皇曆 over the Sui's 581–583 and 597–617 (298 months, 242) | The 興和曆 gives the Western Wei's months, which the Western Wei kept under the 正光曆; the 開皇曆 does not give the 大業曆's, 48 of the 260 months of 597–617 differ |

The two months of the 乾象曆 are within a minute of midnight under the
system's own arithmetic, one before and one after; the table's data
put them on the other side, in opposite directions, so the table
carries something the arithmetic does not. No source read says what. The
four of the 景初曆 are the same kind of case. Neither the table nor the
engine is shown to be right.

Where the sources are thin: the Wikipedia table names no source for its
months. Its sexagenary day names and its sizes of months agree with the
arithmetic's on all 45 months it gives, which checks the day names as well as
the first days. The dates here are Julian, the calendar of the sources;
the engine counts fixed days, and `julian::to_fixed` converts every date in
the tests.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [hanshu-lulizhi] | The 太初曆's epoch, 日法, 統法, 月法 and 周天; 「一月之日二十九日八十一分日之四十三」 | Yes, 2026-10-03, on Wikisource |
| [hhs-lulizhi-xia] | The 四分曆's constants, its 蔀首, and the epoch in 「冬十有一月甲子夜半朔旦冬至」 | Yes, 2026-10-03, on Wikisource |
| [jinshu-lulizhi] | The 乾象曆's and 景初曆's constants and epochs, the Wei court's records of 221–222, the 景初 year of 237–239, the states' systems | Yes, 2026-10-03, on Wikisource |
| [songshu-lulizhi] | The 元嘉曆's constants and epoch | Yes, 2026-09-26 and 2026-10-03, on Wikisource |
| [songshu-lulizhi-daming] | The 大明曆's constants and epoch | Yes, 2026-10-03, on Wikisource |
| [weishu-lulizhi-xia] | The 興和曆's constants and epoch | Yes, 2026-10-03, on Wikisource |
| [weishu-lulizhi-shang] | The 正光曆's constants and epoch | Yes, 2026-10-03, on Wikisource |
| [suishu-lulizhi-zhong] | The 開皇曆's and 天和曆's constants and epochs; the 天保曆, 大象曆's summaries | Yes, 2026-10-03, on Wikisource; the transcription ends before the 大業曆 |
| [liu-chinese-calendar-tables] | The first day of every month of 104 BCE to 597 and the leap months, the list of adopted systems | Yes, 2026-10-03, in the browser pane; the author's program and the data it computes from are GPL-3.0 and were not used |
| [wikipedia-zh-taichu-era] | The 45 months of 太初元年 to 四年 with their sexagenary days | Yes, 2026-10-03 |
| [wikipedia-zh-taichuli], [wikipedia-zh-sifen], [wikipedia-zh-qianxiang], [wikipedia-zh-jingchu], [wikipedia-zh-yuanjia], [wikipedia-zh-daming], [wikipedia-zh-xinghe], [wikipedia-zh-tianhe], [wikipedia-zh-kaihuang] | The constants and the years each ran; 「曆元在正月朔旦夜半雨水時刻」 for the 元嘉曆; 「元和二年二月四日甲寅」 | Yes, 2026-10-03 |
| [wikipedia-zh-daye], [wikipedia-zh-xuanshi], [wikipedia-zh-zhengguang] | The constants of the 大業曆 and 玄始曆 and the dates of the 正光曆; not carried | Yes, 2026-10-03 |
| [wikipedia-zh-dingshuo], [wikipedia-zh-wuyinyuanli], [wikipedia-zh-lindeli] | True conjunction from 619, back to the mean in 645, and 麟德曆 in 665 | Yes, see [east-asian-lunisolar.md](east-asian-lunisolar.md) |

The 後漢書, the 漢書 and the other treatises were read in the Wikisource
transcriptions, whose pages warn of their quality and carry corrections of
the texts in notes; the numbers used here were each compared with the
ratio they imply (the 興和曆's 通數 and 周天 are both 6 158 017, and the 蔀's
days and months are whole numbers of each ratio's) and agree.

## Code

`crates/hc-calendars-lunar/src/chinese_historical.rs`: the modules
`taichu`, `sifen`, `qianxiang`, `jingchu`, `yuanjia`, `daming`, `xinghe`,
`tianhe`, `kaihuang`, `sanji` and `zhengguang`, each with its `EARLIEST`, `LATEST`, `EPOCH`,
`MODEL`, `PARAMETERS`, `ENGINE` and calendar type; the engine is
`lunisolar.rs`.

Anchors, in `crates/hc-calendars-lunar/tests/chinese_historical.rs`:
`the_four_years_of_taichu_are_the_wikipedia_table`,
`the_sifen_began_on_the_fourth_of_the_second_month_of_85`,
`every_epoch_is_the_sexagenary_day_the_treatise_names`,
`the_wei_courts_records_of_221_and_222_fall_on_their_named_days`,
`the_last_taichu_years_are_the_tables`, `sifen_years_are_the_tables`,
`qianxiang_years_are_the_tables`, `jingchu_years_are_the_tables`,
`yuanjia_years_are_the_tables`, `daming_years_are_the_tables`,
`xinghe_years_are_the_tables`, `tianhe_years_are_the_tables`,
`kaihuang_years_are_the_tables`, `sanji_years_are_the_tables`,
`zhengguang_years_are_the_tables`,
`the_engine_agrees_with_integer_arithmetic_on_every_month`,
`months_years_and_leap_months_have_the_shape_of_the_rule`,
`every_day_round_trips`, `the_spans_abut_and_refuse_outside`.
