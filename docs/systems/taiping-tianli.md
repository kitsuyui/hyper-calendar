# The Taiping Heavenly Calendar, 天曆

Backs the identifier `taiping-tianli` in `hc-calendars-solar`.

## What it is

The Taiping Heavenly Calendar is the calendar of the Taiping Heavenly
Kingdom. It was drawn up in the kingdom's first year, 辛開, and was in force
from its second, 壬子. It was a deliberate break with the Qing calendar: a
solar year with no leap day and no leap month, and none of the almanac's
lucky and unlucky days [wikipedia-zh-taiping-tianli]. It was kept at the
capital 天京 until the capital fell in 1864, and by the kingdom's remaining
armies until 1869 [luo-ergang-taiping-tianli].

## How it works

- **The year** is 366 days: the six odd months of 31 days and the six even
  months of 30, with no leap year and no leap month
  [wikipedia-zh-taiping-tianli].
- **The years** are counted from the kingdom's first year, 1851, and
  named by their stem and branch, with 好 for 丑, 榮 for 卯 and 開 for 亥,
  which were thought ill-sounding: 癸好三年, 癸開十三年
  [wikipedia-zh-taiping-tianli, luo-ergang-taiping-tianli].
- **The days** carry a stem and branch and one of the twenty-eight lunar
  mansions, which mark the week. Both are taken from the Qing almanac of
  咸豐元年. The mansions 房, 虛, 昴 and 星 fall on the Western Sunday
  [luo-ergang-taiping-tianli].
- **The slip.** From the calendar's first day, its stems, branches and
  weekdays were one day ahead of the Qing almanac's and the Western week's
  [luo-ergang-taiping-tianli].

**Worked example.** Three days that Luo dates:

1. **The first day.** Luo sets 壬子二年正月初一 against 咸豐元年十二月十四日
   and 3 February 1852. That day was 乙未 and a Tuesday. The calendar
   called it 丙申 and a Wednesday.
2. **The last day of the second year.** The year runs 366 days, to
   十二月三十日, the 31 + 30 + … + 30 = 366th day. 3 February 1852 + 365
   days is 2 February 1853. Luo gives that day as 庚子 on the Qing calendar
   and a Wednesday on the Western one. The calendar called it 辛好 and a
   Thursday: the true names are one day behind, as on the first day.
3. **The last dated use.** The nineteenth year's 四月十一日 is 17 × 366 +
   31 + 30 + 31 + 10 = 6 324 days after the first day. That is 28 May
   1869, the date Luo gives for it, 清同治八年四月十七日.

## What is carried

- `taiping::to_fixed` and `from_fixed`, and `TaipingCalendar`, from
  壬子二年正月初一 to the end of the nineteenth year. The usage is from the
  first day to 28 May 1869, Luo's last dated use.
- `year_name`, the year's stem and branch with 好, 榮 and 開, and
  `year_sexagenary`.
- `labelled_sexagenary_day` and `labelled_weekday`: the stem and branch
  and the weekday the calendar printed, one ahead of the true ones.

Not carried:

- **The 斡年.** Hong Rengan's reform of the ninth year put a 斡年 of 28-day
  months every forty years. It replaced an earlier plan of a year of 33-day
  months every forty [wikipedia-zh-taiping-tianli]. Neither fell within the
  calendar's use, and no source read says which year of the forty would
  have been the first.
- **The solar terms on fixed days**, 立春 on the first of the first month
  and 雨水 on the seventeenth, which the Wikipedia article states without a
  source.
- **The other view of the slip**, that the day names fell a day ahead only
  from 癸好三年二月十三日, the day Nanjing was taken; Luo records it and
  says the evidence refutes it [luo-ergang-taiping-tianli].
- **The first year**, 辛開元年, which was dated on the Qing calendar. The
  calendar printed for it in the third year was worked backwards. Luo warns
  that converting its dates by the new calendar gives nonsense
  [luo-ergang-taiping-tianli].

## Accuracy

The arithmetic is exact. The three dated pairs Luo gives agree with it:

| Check | Test | Result |
| --- | --- | --- |
| 壬子二年正月初一 is 3 February 1852, 乙未 and a Tuesday, called 丙申 and a Wednesday | `luo_sets_the_first_day_against_the_third_of_february_1852` | Holds |
| 壬子二年十二月三十日 is 2 February 1853, 庚子 and a Wednesday, called 辛好 and a Thursday | `luo_sets_the_last_day_of_the_second_year_a_day_ahead` | Holds |
| 己巳十九年四月十一日 is 28 May 1869; the year names Luo writes | `luo_dates_the_last_use_to_the_twenty_eighth_of_may_1869` | Holds |
| The months alternate 31 and 30; every day round-trips; the range is refused outside | `the_months_alternate_thirty_one_and_thirty_days`, `every_day_round_trips_and_the_range_is_refused_outside` | Holds |

Luo's full table, 《天曆考及天曆與夏陽曆日對照表》, was not read; the section
read gives the rule, the three pairs and the reasons for the day slip. The
source's table of each year's first day is not in the archived copy.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [luo-ergang-taiping-tianli] | The first and last days, the pair of 2 February 1853, the slip of the day names from the first day, the use to 1869 | Yes, 2026-09-28, in the Internet Archive's copy of 22 July 2015 |
| [wikipedia-zh-taiping-tianli] | The 366-day year and the months, the renamed branches, Hong Rengan's 斡年 | Yes, 2026-09-28; 張德堅's 《賊情彙纂》 and 吳善中's article, which it cites, were not read |

## Code

`crates/hc-calendars-solar/src/taiping.rs`; the anchors are the tests
named above.
