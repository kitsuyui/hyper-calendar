# Taiwan's holidays, the make-up days and the swapped Saturdays

Backs the `TAIWAN` table (`TW`) in `hc-holiday`.

## What it is

Taiwan's days off are set in two layers. The statute says which days are
holidays; the Executive Yuan's personnel agency, the 行政院人事行政總處
(DGPA), says each year on which weekdays government offices close, and
that calendar is what employers, schools and the exchange follow.

**The statute.** Since 28 May 2025 it is the 紀念日及節日實施條例,
promulgated by 華總一義字第11400053171號 and in force that day
[tw-memorial-days-act-2025]. Article 4 gives a day off for six memorial
days: 開國紀念日 (1 January), 和平紀念日 (28 February), 孔子誕辰紀念日
(28 September), 國慶日 (10 October), 臺灣光復暨金門古寧頭大捷紀念日
(25 October) and 行憲紀念日 (25 December). Article 6 gives the festivals:
"除夕及春節：自農曆十二月末日之前一日至翌年一月三日，放假五日" — five days
from the day before the last day of the twelfth lunar month to the third
of the first — and one day each for 兒童節, 清明節, 勞動節, 端午節, 教師節
and 中秋節; when 兒童節 and 清明節 fall on one day, "於前一日放假。但逢星期四
時，於後一日放假" — the day before, or the day after when it is a
Thursday. Article 8 says a holiday that falls on the weekly rest days
"應予補假", shall be made up, and leaves the days to the authority.

Before the 條例 the list was a regulation, the 紀念日及節日實施辦法, repealed
on 6 June 2025 [tw-memorial-days-regulations]. It gave fewer days — the
three memorial days of 2025, 小年夜 and Labour Day for all were the 條例's
additions, and Labour Day was a day off "勞工放假", for workers only. Its
article 5, as amended on 25 September 2012, carried the Children's Day
coincidence rule; its article 5-1, as amended on 11 June 2014 and in
force from 1 January 2015, the make-up rule: "例假日為星期六者於前一個上班日
補假，為星期日者於次一個上班日補假。但農曆除夕及春節放假日逢例假日，均於次一個
上班日補假" — a Saturday holiday made up the working day before, a Sunday
one the working day after, and the Lunar New Year days always after.
Before 2015 the DGPA's calendars quote article 3 of the
公務人員週休二日實施辦法: "放假之紀念日及節日，逢星期六、星期日，均不予補假。
但春節及農曆除夕不在此限" — no make-up, except for the Lunar New Year
[tw-dgpa-calendars-2012-2014].

**The annual calendar.** The DGPA publishes the 政府行政機關辦公日曆表 for
each year in the year before; the calendar for 2026 was approved on
13 June 2025. Until 2025 it also moved days: a working day
trapped between a holiday and a weekend was given off, 調整放假, in
exchange for a Saturday worked, 補行上班 [tw-dgpa-office-calendars]. The
DGPA's 政府機關配合紀念日與節日補假及調整放假處理要點, as amended on 13 June
2025, "已刪除調整上班日為放假日並補行上班之規定" — no longer provides for the
swap — so from 2026 there are none; its point 3 restates the make-up rule,
now with the Lunar New Year days made up "於前一個或次一個上班日", before
or after, and its point 4 lets the Peace Memorial Day make-up move to the
Friday after the Lunar New Year break when that is the next working day
[tw-dgpa-make-up-points-2025].

## How it works

A year is the statutory days, moved by the make-up rule, plus the swaps
of that year's calendar until 2025.

1. **The dates.** The fixed days are Gregorian dates; the Lunar New Year
   days, 端午節 and 中秋節 are Chinese-calendar dates; 清明節 is the solar
   term at 15° of solar longitude on the China meridian; 兒童節 is 4 April
   moved by article 6's coincidence rule.
2. **The make-up.** A holiday on a Saturday is made up on the working day
   before, on a Sunday on the working day after, skipping days already
   off; the Lunar New Year days are made up after the run. From 2012 to
   2014 only 除夕 and 春節 are made up.
3. **The swaps**, 2017 to 2025: the calendar's weekdays off and the
   Saturdays worked, as listed.

**Worked example: 2027.** The Lunar New Year falls on Saturday 6 February
2027. Article 6's five days are Thursday 4 February (小年夜), Friday 5
(除夕), Saturday 6, Sunday 7 and Monday 8 (the first three days of the
year). Two of them, the 6th and the 7th, are rest days, and are made up
after the run: Tuesday 9 and Wednesday 10 February. 兒童節, 4 April, is a
Sunday, and 清明節 is Monday 5 April, so the make-up passes the Monday and
lands on Tuesday 6 April. 行憲紀念日, 25 December, is a Saturday, made up on
Friday 24 December; and 1 January 2028 is a Saturday, made up on Friday
31 December 2027. The DGPA's release for 116年 gives those three make-ups
[tw-dgpa-make-up-points-2025], and the calendar for 2027 lists seventeen
weekdays off: 1 January, 4, 5, 8, 9 and 10 February, 1 March (for 和平紀念日
on Sunday 28 February), 5 and 6 April, 30 April (for 勞動節 on Saturday
1 May), 9 June, 15 September, 28 September, 11 October (for 國慶日 on
Sunday 10 October), 25 October, 24 and 31 December — and no Saturday
worked [tw-dgpa-office-calendars].

## What is carried

- **The holidays of the 條例 from 2026 and of the 辦法 before**, each with
  its years: 和平紀念日 from 1997, 兒童節 from 2011, 小年夜 and Labour Day
  from 2026, and the three memorial days restored in 2025; 清明 as
  民族掃墓節 to 2025 and 清明節 from 2026.
- **The make-up rule** from 2012: the Lunar New Year days made up after,
  and the other holidays from 2015.
- **The swaps of 2017 to 2025**, from the calendars, as `Rule::Tabulated`
  rules; a year before 2017 is a gap.
- **Not carried, and why:**
  - 兒童節 in 2012, when 4 April was also 清明 and the coincidence rule
    entered the 辦法 only in September; what that year gave is not in the
    sources read, so it is a gap.
  - Point 4's moved Peace Memorial Day make-up and point 3's choice of a
    Lunar New Year make-up before the run: the calendars of 2026 and 2027
    used neither, and a later calendar that does will show as a
    disagreement in the test below.
  - Article 9's days for the police, the military, schools and the other
    services that keep their own.
  - The swaps before 2017 and the make-up days before 2012.

## Accuracy

`taiwan_matches_the_government_office_calendar_from_2017_to_2027` in
`crates/hc-holiday/tests/countries.rs` walks every day of every year from
2017 to 2027 and compares the weekdays that are not business days, and the
weekend days that are, with the DGPA's calendars transcribed into
`TW_OFFICE_CALENDARS`. All eleven years agree in both lists.
`taiwan_holidays_and_the_nearest_weekday_adjustment` pins the Saturday and
Sunday make-ups of 2025, 小年夜 2026 made up after the run, Children's Day
on the day after 清明 in 2024, and the years of the restored days.
`taiwan_made_up_only_the_lunar_new_year_before_2015` pins the change of
2015 and the gap of 2012.

The years 2012 to 2016 are answered for the statutory days and the
make-up only; the calendars of those years were read for article 3 of the
週休二日實施辦法 and not transcribed.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [tw-memorial-days-act-2025] | The 條例: promulgation, articles 4, 6, 8 and 9 | Yes, 2026-09-26, law.moj.gov.tw |
| [tw-memorial-days-regulations] | The 辦法: articles 5 and 5-1 and their amendments, the repeal | Yes, 2026-09-26, law.moj.gov.tw and its old versions |
| [tw-dgpa-make-up-points-2025] | The 處理要點 of 13 June 2025, the end of the swaps, the 116年 release | Yes, 2026-09-26, dgpa.gov.tw |
| [tw-dgpa-calendars-2012-2014] | Article 3 of the 週休二日實施辦法 as the 101年 to 103年 calendars quote it | Yes, 2026-09-26, dgpa.gov.tw |
| [tw-dgpa-office-calendars] | The weekdays off and Saturdays worked of 2017 to 2027 | Yes, 2026-09-23, data.gov.tw |

## Code

`crates/hc-holiday/src/countries/asia.rs`: `TW_RULES`, `tw_public` and
`TW_WEEKEND_MAKE_UP_FROM`, `tw_new_year_day`, `tw_childrens_day`,
`TW_ADJUSTED_OFF` and `TW_MADE_UP` with `tw_lookup`, `TW_SUBSTITUTION` and
the table `TAIWAN`. The engine's part is `SubstituteDirection::NearestWorkingDay`,
`HolidayRule::substitute_towards`, `Rule::Tabulated` and `Kind::Workday`.

Anchors, in `crates/hc-holiday/tests/countries.rs`:
`taiwan_matches_the_government_office_calendar_from_2017_to_2027`,
`taiwan_holidays_and_the_nearest_weekday_adjustment` and
`taiwan_made_up_only_the_lunar_new_year_before_2015`.
