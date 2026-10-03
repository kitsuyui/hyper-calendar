# The East Asian lunisolar calendars: China, Korea and Vietnam on their meridians

Backs the identifiers `chinese`, `dangi` and `vietnamese` in
`hc-calendars-lunar`, and the engine `lunisolar` that they and the Japanese
Tenpō calendar configure. The Japanese systems, which run on the same engine
with their own constants, are in
[japanese-lunisolar.md](japanese-lunisolar.md).

## What it is

The Chinese calendar — 农历 *nónglì*, the "agricultural calendar", also 阴历,
旧历 or 夏历 — is the lunisolar calendar of China and, with local meridians and
local names, of Korea (음력, the calendar of 설날 Seollal and 추석 Chuseok) and
Vietnam (*âm lịch*, the calendar of Tết Nguyên Đán). Months begin at the new
moon, a year has twelve or thirteen of them, and the thirteenth is placed so
that the months keep step with the seasons. None of the three countries keeps
it as the civil calendar — all three date their laws and their working weeks
by the Gregorian calendar, Korea by statute [kasi-lunisolar-conversion] — but
all three fix their major festivals by it, and each computes and publishes
its own.

**The calendar as promulgated.** Each is somebody's computation, and the
somebody differs:

- **China.** Since 2017 the rules are a national standard, GB/T 33661-2017
  《农历的编算和颁行》 *Calculation and promulgation of the Chinese calendar*,
  drafted at the Purple Mountain Observatory of the Chinese Academy of
  Sciences (紫金山天文台) by 成灼, 傅燕宁, 夏芳 and 任树林, issued on 12 May
  2017, in force from 1 September 2017 and confirmed at review on 28 December
  2023 [samr-gbt33661]. The standard fixes the reference time as that of
  120°E (Beijing time), the day of the new moon as the first of the month,
  the month containing the winter solstice as the eleventh, the leap month
  by the "no zhōngqì" rule and the first month as the *yín* month
  [wikipedia-zh-nongli]. The observatory compiles the 中国天文年历, the
  *Chinese Astronomical Almanac*, from which the calendar is published
  [wikipedia-zh-nongli]. Hong Kong's Observatory publishes conversion tables
  for 1901–2100 that agree with the standard's calendar and are what this
  document checks against [hko-conversion, hko-conversion-tables].
- **Korea.** The Korea Astronomy and Space Science Institute (한국천문연구원,
  KASI) computes and publishes the lunisolar calendar; its conversion service
  covers 13 February −59 to 31 December 2050 and states that the country's
  official calendar is the Gregorian one, by article 5 of the 천문법 (Law
  14906 of 24 October 2017) [kasi-lunisolar-conversion]. The calendar it
  maintains is the Shíxiàn calendar as revised [wikipedia-en-korean-calendar].
  For the years before 1912 its data are the Qing calendar's, day for day
  (below, *The almanac before 1912*).
- **Vietnam.** The Democratic Republic of Vietnam's Council of Government
  decided on 8 August 1967 (Decision 121-CP) that the country "lies entirely
  in the seventh time zone" — *nằm hoàn toàn trong múi giờ thứ 7* — with
  effect from 1 January 1968 [wikipedia-vi-gio-viet-nam, vn-decision-121-cp],
  and the lunar calendar has been computed on that meridian since. The
  Republic of Vietnam in the south had kept UT+8 since 1 January 1960 under
  its own decree 362-TTP of 30 December 1959, and moved to UT+7 on 13 June
  1975 after unification [tienphong-two-zones]. The official calendar is the
  State Calendar Board's, the Ban Lịch Nhà nước, with the Information Centre
  of the Vietnam Academy of Science and Technology [lichhuongque-press]; that
  is a secondary statement and no publication of the Board was read here.

**The 1645 lower bound.** The rule these three share is not ancient. The
Chongzhen calendar treatise, compiled under Xu Guangqi with the Jesuits
Johann Schreck and Adam Schall von Bell between 1624 and 1644, replaced the
*píngqì* solar terms — equal twelfths of the tropical year — with *dìngqì*,
the terms at the Sun's true longitude, and the Shunzhi Emperor promulgated
it in the first year of the Qing under the name Shíxiàn (時憲曆)
[wikipedia-en-chongzhen-calendar]; that is 順治二年, 1645, and the change of
solar terms changed where the leap months fall [wikipedia-zh-nongli].
Joseon adopted the Shíxiàn calendar in 1653, the fourth year of Hyojong, on
Kim Yuk's proposal of 1645, and applied it in earnest only from 1725
[wikipedia-ko-siheollyeok, wikipedia-en-korean-calendar]. When the Vietnamese
court adopted it is not stated by any source read here. Before 1645 the
terms were mean, and the calendar of any year was what the bureau of the
day computed from its own tables; this library carries the modern rule from
1645 and, for the years before, the systems of
[chinese-historical-lunisolar.md](chinese-historical-lunisolar.md) as far back as
104 BCE and up to 597; 598 to 1644 is not carried (the roadmap rows say
what each system waits on).

**The almanac before 1912.** The rule of 1645 is the modern one, but the
calendar that carried it was not computed by modern astronomy. The Qing
calendar, the 時憲書 issued each year by the Bureau of Astronomy, was
computed after 1733 with the methods of the *Lìxiàng kǎochéng hòubiān* of
1742, Kepler's laws included, for the Beijing meridian and in Beijing's
apparent solar time; the Beiyang government's calendars of 1912–1928 moved
to a more accurate method and to Beijing mean time, and from 1929 the times
were those of 120°E [liu-chinese-calendar-computation]. The Purple Mountain
Observatory's day-by-day calendar for 1900–2025 prints the promulgated
calendar, not a recomputation: its compilation notes say that 1900–1911
follow the 《时宪书》 published by the Qing, 1912–1948 the 《中华民国历书》 of
the former Central Observatory and the 《国民历》 of the former Academia
Sinica, and 1949–2025 the model of GB/T 33661-2017, and that the dates of
the old almanacs are kept where they differ from a modern computation
[pmo-calendar-1900-2025]. The rules here, read at the meridians below,
reproduce every month of that table from 1900 to 2024 but one — the fourth
month of 1906, worked below — and that one is carried as data.

Before 1900 two records were read. Liu Yuk Tung's reconstruction of the
Qing calendar, 1645–1911, starts from a modern computation and corrects it
to the official calendar as two books give it — the Observatory's
《新编万年历》, taken from the 時憲書 but only from 1840, and Zhang Peiyu's
《三千五百年历日天象》, computed from the historical rules and checked
against surviving calendars — which agree from 1840 to 1911; the books
were not read here [liu-chinese-calendar-computation]. Against it the
rules at Beijing mean time part in 29 first days, 28 of them before 1900
and the 29th in 1906, and in 5 leap months. The Veritable Records, 《清實錄》, the court's day-by-day
record of each reign, open every month with its first day by sexagenary
name — 乾隆十九年八月戊申朔, "the eighth month of Qianlong 19, whose first
day was *wù-shēn*" — and they have all 34 as Liu has them [qing-shilu].
Those months are carried as data, each with its record: the 29 first days
as `MonthStartCorrection`s and the 5 leap months as corrections of the
solar-term day that placed them (*The almanac's terms*, below). Read month
by month, the Records' first days agree with the calendar so corrected in
every one of the 3 260 months of 1645–1911 whose first day the
transcription gives, the last reign's 宣統政紀 included, and with the rules
alone in all but 39: the 29 first days, and in each of the five years, the
leap month and the month beside it that the rules number differently
(*Accuracy*).

Korea kept the same almanac, not the same rules at its own meridian. KASI's
conversion data for 1900–1911 give the Qing first day of every month,
including five whose conjunction fell before midnight at Beijing and after
it at Seoul, and over 1653–1911 they side with Beijing in 80 of the 105
months where the two meridians disagree [kasi-lunisolar-conversion]; in 24
of the other 25 the almanac itself had Seoul's later day, and they are
among the corrections above. Asked on every day that begins a month under
one of the Beijing rules, the Seoul rules or the corrected calendar and not
under another — 216 days of 1653–1911 — KASI agrees with the corrected
calendar on 210, and on the first day of every leap month of 1653–1911 but
one. The three months it leaves the Qing almanac in are 1653, Joseon's
first year on the new rules, where KASI has 閏七月 from 23 August and 八月
from 21 September against the Records' 閏六月 from 24 July and 八月 from
22 September; the twelfth month of that year, from 19 January 1654, Seoul's
day, against 18 January; and the twelfth month of 1841, from 12 January
1842, the rules' day, against the Records' 11 January, *gēng-chén*. A
report on ICU4X's issue tracker says that the Joseon, Korean Empire and
colonial almanacs on KASI's site give their times in Beijing time until
1912 and that the Korean calendar matches the Qing one until then
[icu4x-issue-6455]; the almanac pages it points to returned a server error
here, so the statement rests on KASI's data, not on the almanacs.

KASI's reading is carried beside `dangi` as a parameter set,
`dangi::KASI_PARAMETERS`, and registered as `dangi-kasi` (`dangi::DangiKasiCalendar`), because the two readings disagree and nothing read says which Hanseong printed (docs/policy.md §5). It drops the
Qing almanac's first days of 八月 1653 and 十二月 1841 for the rules' days,
begins 十二月 of 1653 on 19 January 1654 as KASI does, and puts 處暑 of 1653
on 22 August, a day before the rules' 05:43 on the 23rd: KASI gives no term
days, and that day is the one that makes its months, 七月 from 24 July and
閏七月 from 23 August, so it is inferred and marked so. It begins with 1653,
Joseon's first year on the Shíxiàn rules: before it KASI's data are Korea's
older calendar — 1651's new year on 20 February and no 閏二月 — which no
parameter set here computes. On the 321 days of 1653–1911 that were queried
from KASI it gives KASI's date on every one; `dangi` differs on ten of
them, the days on either side of the departures above.

## How it works

### The rules

The rules are those *Calendrical Calculations* states for the Chinese
calendar, and the published code is what this library implements
[reingold2018, reingold2018code]:

1. **A month runs from new moon to new moon.** Its first day is the local
   day, at the calendar's meridian, that contains the astronomical
   conjunction (`chinese-new-moon-on-or-after`, `midnight-in-china`). A
   month is 29 or 30 days, never by alternation.
2. **Month 11 contains the winter solstice.** From one December solstice to
   the next is a *suì* (歲), which holds twelve or thirteen new moons
   (`chinese-winter-solstice-on-or-before`).
3. **A suì of thirteen new moons takes a leap month**, the first month in
   it that contains no *zhōngqì* (中氣), no major solar term
   (`chinese-no-major-solar-term?`, `chinese-prior-leap-month?`). The leap
   month repeats the number of the month before it: 閏二月 after 二月.
4. **The zhōngqì** are the twelve points at which the Sun's apparent
   longitude is a multiple of 30°, and which one a day stands under is read
   at local midnight of that day (`current-major-solar-term`). Each names a
   month: 雨水 (330°) belongs to month 1, 春分 (0°) to 2, 穀雨 (30°) to 3,
   and so round to 冬至 (270°) in month 11 and 大寒 (300°) in month 12. A
   month "contains" a zhōngqì when the term index at its own first midnight
   differs from that at the next month's.
5. **The year begins at the second new moon after the winter solstice** —
   the month after the solstice's month is 12 and the one after that is 1 —
   unless the suì has thirteen new moons and one of those two months is the
   leap month, in which case the year begins one new moon later
   (`chinese-new-year-in-sui`).

The **suì and the nián** are two years. The suì is the astronomical one,
solstice to solstice, and it is where the leap month is found; the *nián*
(年) is the civil one, from 正月初一 to the day before the next, and it is
what the date names. A leap month found in one suì is numbered and displayed
in whichever nián holds it, so a 閏十一月 or 閏十二月 belongs to the nián that
is ending, and `leap_month(year)` in the crate answers for the nián.

Under *dìngqì* the terms are unevenly spaced — the Sun moves slowest in
July, so the summer terms are up to 31.5 days apart and the winter ones
29.5 — which is why a month can hold two zhōngqì and a neighbouring month
none, and why the rule asks for thirteen new moons in the suì before it
calls a termless month a leap month. Under *píngqì* a month without a
zhōngqì was always the leap month. That difference is what
`SolarTermMode` chooses between.

### The meridian

Local midnight at a named place is where the day is read, so a conjunction
or a solstice that falls in the hour between two countries' midnights lands
on different days in the two calendars, and a solstice or a zhōngqì that
moves a day can move a leap month. The histories implemented are those of
the published code [reingold2018code, `chinese-location`, `korean-location`,
`vietnamese-location`]:

| Calendar | From | Offset from UT | What it is |
| --- | --- | --- | --- |
| Chinese | — | +7:45:40 | Beijing local mean time, 116°25′E |
| | 1929 | +8:00 | The 120°E standard zone |
| Dangi | — | +7:45:40 | Beijing local mean time: the Qing calendar |
| | 1 January 1912 | +9:00 | The 135°E zone, under the Governor-General |
| | 21 March 1954 | +8:30 | Back to 127°30′E, under Syngman Rhee |
| | 10 August 1961 | +9:00 | Back to 135°E, under Park Chung-hee, where it remains |
| Vietnamese | — | +8:00 | The 120°E zone, on which the calendar was computed before 1968 |
| | 1 January 1968 | +7:00 | The 105°E zone, by Decision 121-CP |

The Korean rows from 1912 are the published code's `korean-location`, and
its years are corroborated by the general history of Korean standard time
[wikipedia-en-time-in-south-korea]; the days within those years are the
code's and no other source read here gives them. Before 1912 the code reads
Seoul mean time, 126°58′E, and from 1 April 1908 the Korean Empire's
127°30′E zone; this library reads Beijing instead, because the calendar
Korea kept was the Qing almanac (above) and KASI's data for 1900–1911 have
Beijing's day in all five months where the code's offsets put a conjunction
past midnight — 17 January and 7 November 1904, 4 May 1905, 30 April 1908
and 20 December 1911 — and the almanac's day in the sixth month where the
Chinese and Korean first days are both tested, 24 April 1906. The 1908
zone was the clock's, not the calendar's. That the colonial calendar moved
to 135°E on 1 January 1912 rather than in 1913 is the code's; no month of
1912 or 1913 begins on a different day under the two, so nothing turns on
it. For China, the
reference of the calendar is recorded as moving from Beijing mean solar time
to UT+8 in 1928–1929, the exact date being unclear
[wikipedia-en-time-in-china], and the code takes the Gregorian year 1929.
Before standard time the same source describes Beijing's reference as
apparent solar time, and Liu dates the move from apparent to mean time to
the Republic's calendars after 1913 [liu-chinese-calendar-computation]; the
code, and this library, use mean time throughout. The difference, up to a
quarter of an hour, moves no month of 1900–1911 in the Observatory's table:
the one month that differs does so by the almanac's own astronomy, not its
clock (below).

Two simplifications of the crate's, both stated because they could matter
and measured because they do not: the crate keys each meridian era by year,
not by day, so it reads 1 January to 20 March 1954 at UT+8:30 and 1 January
to 9 August 1961 at UT+9 where the published code reads the earlier offset.
Every new moon and every zhōngqì in those two windows falls on the same day
under either offset, and so does every calendar date; the test
`the_year_keyed_eras_give_the_days_the_day_keyed_changes_give` reads each
window at both offsets and says so. And the Vietnamese "southern" parameter
set keeps UT+8 for all time. That is right for the calendar printed in
Saigon in 1968, and it carries no era for the south's civil time of UT+7
before 1 January 1960 [tienphong-two-zones]. Not carried: no southern
almanac was read that would put the calendar on UT+7, and the meridian of
the calendar is not the civil clock — the north kept UT+7 civil time from
1945 and, by the published code, computed its calendar on UT+8 until 1968.
Measured for this document, reading 1949–1959 at UT+7 instead of UT+8
would move six month boundaries by a day (13 to 14 August 1950, 4 to
5 June 1951, 9 to 10 August 1953, 2 to 3 November 1956, 1 to 2 March 1957
and 21 to 22 November 1957 — the last three in the Republic's own years),
change the zhōngqì index at midnight on nine days without moving a term's
month, and move no new year and no leap month.

### The model

The engine holds the rules and `LunisolarParameters` holds what differs:

- `meridians: &[MeridianEra]`, the table above, each era made by
  `MeridianEra::from_longitude` (a local mean time) or `from_zone` (a civil
  offset), and `meridian_era(rd)` picks the last era whose `from_year` is
  not after the day's Gregorian year.
- `solar_term_mode: SolarTermMode`, `Apparent` for *dìngqì* — all three
  calendars — and `Mean` for *píngqì*, which is offered because it is the
  rule that was in force before 1645, not because any registered calendar
  uses it.
- `mean_motion: Option<MeanMotionModel>`, `None` for all three, meaning the
  Sun and the Moon come from `hc-astro` at every call: the conjunction from
  `new_moon_at_or_after` and `new_moon_before`, the solstice from
  `solstice`, the term from `solar_longitude` at local midnight.
  `ConjunctionMode` — mean, true from a system's own tables, or true from
  modern astronomy — is a field of `MeanMotionModel` and so concerns only the
  Japanese historical systems.
- `month_start_corrections: &[MonthStartCorrection]`, the months whose
  first day a record of the promulgated calendar puts on another day than
  the rules, each with its `source`: `chinese::ALMANAC_CORRECTIONS`, 29
  entries of 1652–1906, which `dangi` shares because before 1912 it is the
  same calendar, and nothing for `vietnamese`. The engine applies a
  correction to every month boundary it computes, so the month before gains
  or loses the day with it, and a correction may move a day by one at most
  (`MonthStartCorrection::MAX_SHIFT`), which keeps the search for the
  neighbouring month local. It is data because it is a fact about a
  document, not a rule: the method that produced it, the bureau's tables,
  is not implemented.
- `major_term_corrections: &[MajorTermCorrection]`, the *zhōngqì* the
  promulgated calendar reckoned to another day than the rules, each with
  its term, its two days and its `source`: `chinese::ALMANAC_TERM_CORRECTIONS`,
  5 entries of 1645–1805, again shared by `dangi`. `major_solar_term` reads
  the index at each midnight between the two days as the almanac would,
  `computed_major_solar_term` is the rules alone, and a correction of 冬至
  would move the solstice's day too; none is carried. Only terms that move a
  leap month are carried, because the others change no date.
- `epoch: CHINESE_EPOCH`, RD −963 099 = 15 February 2637 BCE (year −2636
  proleptic Gregorian), the published code's `chinese-epoch`, from which all
  three count elapsed years; `year_offset` is what each adds to display its
  own number.
- `earliest` and `latest`, the range below.

### Worked example: the fourth month of 1906

The one month of 1900–2024 where the rules and the almanac part.

1. *The conjunction.* `hc-astro` puts the new moon of April 1906 at
   16:06:31 UT on 23 April: 23:52 in Beijing mean time (UT+7:45:40), 00:06
   on the 24th at 120°E and 00:34 on the 24th in Seoul mean time.
2. *The rule.* At Beijing the conjunction is eight minutes before midnight,
   so the month begins on 23 April, *dīng-yǒu*, and the third month, begun
   on 25 March, has twenty-nine days.
3. *The almanac.* The Purple Mountain Observatory's table, from the 時憲書
   of 光緒三十二年, has 三月 run thirty days, ending on 丁酉, 23 April, and
   四月初一 on 戊戌, 24 April; 閏四月 begins on 23 May and 五月 on 22 June,
   as the rule has them [pmo-calendar-1900-2025]. The Hong Kong
   Observatory's table has 24 April [hko-conversion-tables], and so do
   KASI's data for the Korean calendar [kasi-lunisolar-conversion].
4. *Why.* The almanac put the conjunction after its own midnight. Its
   clock does not explain that: the equation of time that afternoon is
   +1.6 minutes (`hc-astro`'s `equation_of_time`), so in Beijing apparent
   time, the almanac's, the modern conjunction is at 23:54, still on the
   23rd, and 116°25′ against 116°23′ is eight seconds. What is left, by
   elimination, is the 1742 method putting the conjunction at least six
   minutes later than modern astronomy does. That is an inference: no
   source read computes the bureau's conjunction of 1906, and the
   Observatory's compilation notes give only the general cause, that the
   almanacs before 1949 part from a modern computation in some dates
   because of 计算条件的限制, the limits of the conditions they were
   computed under (编制说明 §3) [pmo-calendar-1900-2025].
   It is the kind of error behind the 28 earlier months
   the Records and Liu's reconstruction move (*The almanac before 1912*),
   all with a conjunction within 23 minutes of Beijing mean midnight, 25 of
   them put after it by the almanac and 3 before
   [qing-shilu, liu-chinese-calendar-computation].
5. *What the crate does.* `chinese::ALMANAC_CORRECTIONS` holds the pair
   (23 April 1906, 24 April 1906) with its source; `chinese` and `dangi` then give 三月 thirty
   days and 四月 twenty-nine, and a parameter set built with no corrections
   still gives the rule's 23 April. The tests are
   `the_fourth_month_of_1906_began_on_the_day_the_almanac_gave` and
   `without_the_almanac_the_rules_miss_one_month_in_the_table`.

Nothing here counts as a second calendar under the policy's rule for
competing conventions. The Observatory, the Hong Kong Observatory and KASI
publish the same day, and no calendar printed or kept the rule's 23 April: a modern
recomputation of 1906 at Beijing mean time is this library's approximation
of the Qing calendar, not a convention anyone kept. The same holds of the
33 earlier months: the Records, Liu and, for all but three, KASI agree, and
no calendar read printed or kept the rule's days. So `chinese` carries the
almanac where a record of it was read, and the bare rule stays constructible
as a `LunisolarParameters` without corrections, as `vietnamese-south-1968`
does for a reckoning with no registered use.

### The almanac's terms: the leap month of 1727

A first day the almanac moved changes the lengths of two months; a term
it moved can carry a leap month a whole lunation. 雍正五年, 1727, is the case
where the term's day is read rather than inferred.

1. *The months.* The rules and the Records agree on the first days
   around it: 22 January, 23 March, 21 April and 21 May [qing-shilu].
2. *The rule.* `hc-astro` puts 春分 at 11:24 Beijing mean time on
   21 March, in the month from 21 February; 穀雨 at 00:26 on 21 April, the
   first day of the month that begins then; 小滿 at 01:18 on 22 May, in the
   month from 21 May. So the month from 23 March holds no *zhōngqì*, the
   suì has thirteen new moons, and it is 閏二月.
3. *The almanac.* The Records open 三月 with 戊子, 23 March, and 閏三月 with
   丁巳, 21 April [qing-shilu]; Liu's table has the same, and its
   calendrical solar terms — the terms of the Tychonic theory the bureau
   used before 1733 — put 穀雨 on 20 April, the day before
   [liu-chinese-calendar-computation]. KASI's Korean calendar has 閏三月 from
   21 April too [kasi-lunisolar-conversion].
4. *Why.* With 穀雨 on 20 April the month from 23 March holds it and is
   三月, and the month from 21 April holds nothing — 小滿 falls on the first
   day of the next — so it is the leap month. Twenty-six minutes after
   midnight by modern astronomy is before midnight by the bureau's Sun.
5. *What the crate does.* `chinese::ALMANAC_TERM_CORRECTIONS` holds
   (穀雨, 21 April 1727, 20 April 1727), and `chinese` and `dangi` give
   閏三月 from 21 April; the parameters without corrections give 閏二月 from
   23 March. Of the five, 1651's 春分 and 1661's 秋分 are Liu's term days in
   the same way; 1645's 大暑 is Lǐ Tiānjīng's rule, printed on the leap
   month's first day and counted to the month before, as Wāng Yuēzhēn's
   《歷代長術輯要》 explains in Liu's account (cited by Liu, not read), and
   carried as reckoned to the day before; 1805's 處暑, at 23:52 Beijing mean
   time on 23 August by the rules, is inferred: no table of the almanac's
   terms after 1733 was read, and the Records' 閏六月 from 26 July needs it
   on the 24th. Aslaksen takes the same month as his example of the
   meridian, with 處暑 at 0h07m on 24 August at 120°E and "about 7 minutes
   before midnight" at Beijing [aslaksen2010, §4.6], which supports a term
   within minutes of midnight, found independently. His explanation does
   not reproduce the Records under the rule here: he has Beijing's meridian
   make the leap month the one after the sixth, but with the term on
   23 August the month from 26 July holds it and the leap month is the one
   after the seventh, and it is the term on the 24th, the 120°E day, that
   gives 閏六月. The tests are
   `the_leap_months_the_almanac_moved_are_where_the_veritable_records_have_them`
   and `the_almanac_term_corrections_are_live_and_move_a_day_at_most`.

What is carried is the calendar kept, not always the calendar first
printed. The almanac for 康熙八年, computed by the old Dàtǒng method during the
Calendar Case, carried a 閏十二月; Ferdinand Verbiest showed that the
month held 雨水 and was the first month of 康熙九年, and the leap month was
moved to 康熙九年二月 [qingshigao-shixian, liu-chinese-calendar-computation].
The rules here give 閏二月 of 1670, the calendar as corrected, and so do the
Records, which open 康熙九年正月 on 己丑, 21 January 1670, the month the
first almanac had made 閏十二月 [qing-shilu]; that printing is not carried.

### Worked example: Seollal 1988

The conjunction that began the Year of the Dragon was at 15:54 UT on
Wednesday 17 February 1988. In China, at UT+8, that is 23:54 on the 17th, so
the month — and the year 4625, *wù-chén* — begins on 17 February, which is
the day the Hong Kong tables give [hko-conversion-tables]. In Korea, at UT+9,
the same instant is 00:54 on the 18th, so the month begins on 18 February and
Seollal falls a day after Chinese New Year. Nothing else differs: the
solstice and the zhōngqì of that suì fall on the same days at both meridians,
so the month numbers agree, the day-of-month is one behind in Korea until
the next conjunction, at 02:02 UT on 18 March, which both calendars put on
the same day, and the first month runs 30 days in China and 29 in Korea. The instants are `hc-astro`'s, through the crate's own functions.

Over 1900–2049 the crate finds the two new years on different days in nine
years — 1916, 1944, 1954, 1958, 1966, 1988, 1997, 2027 and 2028 — always
with Seollal the later by one day, since Korea's midnight comes first. The
count is asserted in `the_two_calendars_disagree_only_occasionally`; the
individual years are this document's listing and are not a test.

### Worked example: Tết 1985, a lunation before Chinese New Year

Here an hour moves not a month boundary but the winter solstice, and with it
the leap month and the new year.

1. *The solstice.* The December solstice of 1984 was at 16:23 UT on
   21 December: 23:23 at Hanoi, on the 21st, and 00:23 at Beijing, on the
   22nd.
2. *The new moons.* 22 November 1984 at 22:57 UT, 22 December at 11:47 UT,
   21 January 1985 at 02:28 UT, 19 February at 18:43 UT, 21 March at
   11:59 UT, 20 April at 05:22 UT. At both meridians those begin months on
   23 November, 22 December, 21 January, 20 February, 21 March and
   20 April; no conjunction falls in the hour between the two midnights.
3. *Which month is 11.* At Hanoi the solstice falls on 21 December, the last
   day of the month that began on 23 November, so that month is 11 and the
   month beginning 22 December is 12; the year 1985 — *Ất Sửu*, the Ox —
   begins with the next new moon, on 21 January. At Beijing the solstice
   falls on 22 December, the first day of the month beginning that day, so
   *that* month is 11, 21 January begins month 12 and the year 4622,
   *yǐ-chǒu*, begins on 20 February [hko-conversion-tables].
4. *Where the leap month goes.* At Beijing the suì from December 1983 to
   December 1984 has thirteen new moons, and the month beginning 23 November
   1984 holds no zhōngqì — 小雪 on 22 November is a minor term, and 冬至 fell
   after it ended — so it is 閏十月 of 4621, *jiǎ-zǐ*. At Hanoi that month
   holds 冬至, so 1984 has twelve months; instead the suì from December 1984
   to December 1985 has thirteen, and the month beginning 21 March 1985
   holds no zhōngqì, because 春分 fell at 23:14 on 20 March, the day before
   it began, and 穀雨 falls at 10:26 on 20 April, the day the next month
   begins. It is 閏二月, from 21 March to 19 April, exactly as the general
   account of the case has it [wikipedia-en-vietnamese-calendar].
5. *The result.* Tết on 21 January 1985 and Chinese New Year on 20 February,
   thirty days apart, and the two calendars a month apart in their numbering
   from 22 December 1984 until 20 April 1985, when both begin month 3. The
   test is `tet_1985_fell_a_whole_month_before_chinese_new_year`.

**Tết 1968**, the case everyone cites, is the simpler kind: the conjunction
of 29 January 1968 at 16:29 UT is 23:29 at UT+7 and 00:29 at UT+8, so the
north, on the new meridian from 1 January, kept Tết on 29 January and the
south on 30 January [wikipedia-vi-tet], and the crate reproduces both from
the two parameter sets. The same hour catches the conjunction of February
1969 as well: 16 February in the north against 17 February, Chinese New
Year, in the south. Nhân Dân dates Hồ Chí Minh's tree-planting at Vật Lại,
Ba Vì, "sáng 16-2-1969 (mồng 1 Tết)" [nhandan-tet-trong-cay-2019], which
attests the northern day; the southern one is as computed. Over 1968–2049
the crate finds Tết and Chinese New Year
apart in 1968, 1969, 2007 and 2030 by a day and in 1985 by a lunation; a
Vietnamese account names 2007 and 2030 as years the two calendars differ
[lichhuongque-press].

## What is carried

- **Identifiers**, all in `hc-calendars-lunar`, all with the date as
  `LunisolarDate { year, month: Month { ordinal, leap }, day }`, the day
  beginning at local midnight, and the sexagenary year, month and day
  through `LunisolarParameters::sexagenary_year`, `sexagenary_month` and
  `sexagenary_day`:

  | Identifier | What it is | Year number for the year that began 10 February 2024 |
  | --- | --- | --- |
  | `chinese` | CLDR's `chinese`; the rule at Beijing's meridian, with the almanac's first day in 29 months of 1652–1906 and its term day behind 5 leap months of 1645–1805 | 4661, `year_offset` 0 |
  | `dangi` | CLDR's `dangi`; the Chinese calendar before 1912, the rule at Korea's meridian since | 4357, `year_offset` −304 |
  | `vietnamese` | This library's own name, CLDR having none; the rule at Hanoi's | 2024, `year_offset` −2637 |

- **Year numbering, and whose it is.** The Chinese count is the elapsed
  years from the 2637 BCE epoch plus one, so that
  `hc_calendar::cycle::sexagenary_year` is right on it directly: 4661 is
  *jiǎ-chén*. It is the count for which *Calendrical Calculations* names the
  epoch, though the book itself writes a year as cycle and position — 4661
  is year 41 of cycle 78 — and its `korean-year`, 60 × cycle + year − 364,
  gives Dangi 4357 for the same year [reingold2018code]. Two other counts
  circulate: 4721 is 60 × cycle + year, and 4722 is the 黃帝紀元 of the
  almanacs, the Gregorian year plus 2698, which public calendars do not use
  [wikipedia-zh-nongli]. None is official, because the calendar has no
  official era. The Dangi count runs from the traditional foundation of
  Gojoseon in 2333 BCE and was the Republic of Korea's official year
  number under the Act on Era Names, Act No. 4 of 25 September 1948, until
  Act No. 775 of 2 December 1961 made the Common Era official from
  1 January 1962 [encykorea-dangun-giwon; the statutes were not read]; it
  is not in official use now. The
  Vietnamese number, the Gregorian year in which the lunisolar year begins,
  is this library's convention, adopted because Vietnam has no continuous
  era, and is labelled as such.
- **Range** 1645-01-01 to 2150-12-31 Gregorian for all three. The lower
  bound is the Shíxiàn reform, before which the terms were mean and the
  month numbering could differ; the upper is `hc-astro`'s
  `LATEST_FITTED_YEAR`, where its ΔT fit ends and the long-term parabola
  begins. Korea and Vietnam share the Chinese bound although Korea adopted
  the rule in 1653 and Vietnam at a date not established here: a date in
  the gap is what the rule gives, not what Hanseong or Huế proclaimed.
  Outside the range the calendars return `BeforeEpoch` or
  `AfterSupportedRange` rather than extrapolate.
- **Computed, not tabulated.** Every conjunction, solstice and solar
  longitude is computed from `hc-astro` when asked, which is why
  `is_astronomical` is true in the metadata and why the accuracy below is
  the accuracy of the astronomy at the day boundary. The stored facts are
  `chinese::ALMANAC_CORRECTIONS`, the 29 first days above, and
  `chinese::ALMANAC_TERM_CORRECTIONS`, the 5 terms, each with its source:

  | Source class | First days | Terms |
  | --- | --- | --- |
  | The Veritable Records [qing-shilu], with Liu's table and KASI's data [liu-chinese-calendar-computation, kasi-lunisolar-conversion] | 25 | — |
  | The Veritable Records and Liu's table; KASI differs | 3 (1652, 1653, 1841) | — |
  | The Purple Mountain Observatory's table of the 時憲書 [pmo-calendar-1900-2025], the Veritable Records, the Hong Kong Observatory and KASI | 1 (1906) | — |
  | The leap month from the Veritable Records; the term day from Liu only | — | 4 (1645, 1651, 1661, 1727) |
  | The leap month from the Veritable Records and Liu; the term day inferred | — | 1 (1805) |
- **Constructible but not registered.** `SolarTermMode::Mean` with any
  meridian, through `LunisolarParameters`, which is not a calendar anyone
  publishes; and the Republic of Vietnam's 1968 reckoning as
  `vietnamese::SOUTHERN_PARAMETERS` (id `vietnamese-south-1968`), kept as
  data so that the disagreement can be tested rather than described. The
  Republic's almanacs did publish that reckoning, which is what put Tết
  1968 on 30 January in the south; it is not registered because it differs
  from the northern calendar only from 1968 to 1975 and what was read of it
  is two new years, not an almanac or decree that fixes its meridian and
  its span, so a registered calendar would claim seven years of months
  that nothing here can check. It is a Researching row of the roadmap,
  `vietnamese-south`, which names the almanac it waits on.
- **The almanac's solar terms, 1645–1733**, as data:
  `chinese::almanac_solar_term_days(year)`, the twenty-four term days, 小寒
  to 冬至, of Liu's calendrical solar terms, the bureau's Tychonic terms
  before it went over to Kepler's laws, in his own encoding (the day of
  January of 小寒, then each interval less fourteen days)
  [liu-chinese-calendar-computation]. Read that way the table gives 春分 on
  20 March 1651, 秋分 on 23 September 1661 and 穀雨 on 20 April 1727, the
  three term days the carried leap months turn on. 88 of its major terms
  are a day from the rules; applied as corrections, they move no new year
  and no leap month but the five `ALMANAC_TERM_CORRECTIONS` already carry
  (`the_almanac_term_days_move_no_month_but_the_five_carried`). The rows
  for 1667–1669 are the Western recomputation, which Liu says the
  almanacs of those years did not print — they gave the Dàtǒng system's
  *píngqì* terms — and the function refuses those years.
- **Beside the Chinese calendar**, functions of `chinese` that read a
  person's or a year's place in it, the first two from the published code
  of *Calendrical Calculations* [reingold2018code]:
  - `reckoned_age`, the age by the Chinese count: one at birth and one more
    at each Chinese New Year (`chinese-age`). Wikipedia gives this as the
    pre-modern reckoning of *suì* in China, with the example of a child
    born in June 2000, a dragon year, who turns 13 *suì* at the lunar new
    year of 2012 [wikipedia-en-east-asian-age-reckoning]; the module
    reproduces it (Chinese New Year 2012 is 23 January). The age turns at
    正月初一 because that is `chinese-age`'s rule.
  - `reckoned_age_at_lichun`, the same count turning on the day of 立春,
    which 果壳's account of 虚岁 gives as the custom of some
    places, 「在有的地方，长虚岁的节点是立春」 [guokr-xusui, a popular
    secondary source]; `lichun_day` gives the day. A child born on 1 June
    2009 is two from 4 February 2010, the 立春 the *South China Morning
    Post* gives [scmp-double-spring-2009], ten days before the New Year
    count makes it two.
  - `reckoned_age_at_new_year_day`, one at birth and one more each
    1 January: the Korean age, "(Current Year − Birth
    Year) + 1", with a child born on 31 December two the next day
    [wikipedia-en-east-asian-age-reckoning], and the modern 虚岁 of Korea
    [wikipedia-zh-xusui]; and `year_age`, from nothing, the
    "year age" of some South Korean laws and the modern 虚岁 of Vietnam and
    parts of China.
  - `marriage_augury` and `MarriageAugury`, the year classed by where 立春,
    the minor term at 315°, falls in it (`chinese-year-marriage-augury`):
    `Widow` with none, `Blind` with one near the end, `Bright` with one
    near the start, `DoubleBright` with both. The test is the last minor
    term before the local midnight that begins each New Year
    (`current-minor-solar-term`): 小寒 when 立春 is still to come, 立春
    when it has passed. The names are the code's. Wikipedia's "Lichun"
    calls a year without 立春 a "widow year" (寡婦年) in the north and a
    "blind year" (盲年) in the south, unlucky for marriage
    [wikipedia-en-lichun], so "blind" means the code's `Widow` there.
    `MarriageAugury::chinese_names` carries the Chinese names with their
    script and region: 無春年, 寡婦年 (north) and 盲年 (south)
    [wikipedia-en-lichun], 无春年, 寡妇年 and 盲年 [wikipedia-zh-lichun] for
    `Widow`, and 雙春兼閏月 [hko-double-spring] and 双春年
    [wikipedia-zh-lichun] for `DoubleBright`. No source read names a year
    with one 立春, so `Blind` and `Bright` have none. The 丁酉 year of 2017,
    with 立春 on 正月初七 and 臘月十九, is a double spring, and the 戊戌 year
    after it has its only 立春 on its 除夕 [wikipedia-zh-shuangchun]; both
    are reproduced. The year that
    began on 10 February 2024 is a Widow Year [scmp-widow-year-2024], and
    the one that began on 26 January 2009 holds two 立春, 4 February 2009
    and 4 February 2010 [scmp-double-spring-2009]; both are reproduced.
- **The Vietnamese zodiac**, the cat for the rabbit
  and the water buffalo for the ox [wikipedia-en-vietnamese-zodiac,
  wikipedia-en-vietnamese-calendar], is `hc_calendar::cycle::VIETNAMESE_ZODIAC_ANIMALS`
  and `Sexagenary::vietnamese_zodiac_animal`; `zodiac_animal` stays the
  Chinese set. Nothing selects an age count or a zodiac by a string, so
  each is named by its function and has no string identifier (policy §5).
  Wikipedia's example, a book dated
  辛卯年, Tân Mão, 1831, is a year of the cat
  (`the_vietnamese_zodiac_has_the_cat_and_the_buffalo`); the `vi` locale of
  `hc-i18n` already writes the animals Trâu and Mèo.
- **Not carried, and why.**
  - Any calendar before 1645 by this rule. The calendars of 104 BCE to 597
    are `chinese_historical`, each on its own arithmetic
    ([chinese-historical-lunisolar.md](chinese-historical-lunisolar.md)); what is
    carried of 619–1644 is nothing, and the Tang's, the Song's, the Yuan's
    and the Ming's systems are roadmap rows. From 1645 a date is the modern rule at
    Beijing mean time corrected where a record says the almanac differed,
    and so corrected it agrees with Liu's reconstruction in every month of
    1645–1911 and with the Veritable Records wherever they were read
    (*Accuracy*). The bureau's own method,
    Tycho's tables to the 1730s and the *Hòubiān*'s after, is not
    implemented, and the Japanese document says what carrying a bureau's
    tables costs; so a month the records do not reach is not guaranteed.
  - The almanac's terms of 1667–1669, which the Dàtǒng method computed as
    *píngqì*: Liu gives them in a separate article, not in the table read;
    the months of those years are the same either way but for the 閏十二月
    of 康熙八年, which the calendar kept moved to 康熙九年二月 (above)
    [liu-chinese-calendar-computation]. And the almanac's terms after 1733,
    which Liu lists only where they differ, and which were not transcribed.
  - No Vietnamese table of the calendar's months was found.
  - Vietnamese names beyond the ones read. The `vi` locale writes the
    Chinese-family calendars as CLDR 48's `vi.xml` writes the Chinese
    calendar's long date, 'Ngày' dd 'tháng' M 'năm' U, *Ngày 18 tháng Tám
    năm Bính Ngọ 2026*, with the year by its stem and branch as `vi.xml`
    names the sixty (Giáp Tý to Quý Hợi), the leap month by its pattern
    `{0} Nhuận` and the months by the traditional names of the Vietnamese
    Wikipedia's "Nông lịch", tháng giêng, hai, ba, tư, năm, sáu, bảy, tám,
    chín, mười, mười một and chạp [wikipedia-vi-nong-lich]. `vi.xml` itself
    names the months by numerals, so the names are Wikipedia's and not
    CLDR's, and a Vietnamese almanac's wording was not read. The related
    Gregorian year follows the stem and branch, as the file's `yyyyMMMMd`
    item has it. The solar terms' Vietnamese names, which `vi.xml` gives
    (Lập Xuân to Đại Hàn), are not written by any date. Anchored by
    `the_vietnamese_calendar_is_written_in_vietnamese` and
    `the_twelve_months_have_their_traditional_names`
    (`crates/hyper-calendar/tests/vietnamese_names.rs`).
  - The precision and representation clauses of GB/T 33661-2017, which were
    not read (below).
  - The minor terms (節氣), which are in `hc-seasons`; the holidays keyed to
    these dates, which are in `hc-holiday`; the regnal eras, which are in
    `hc-calendars-regional`.

## Accuracy

| Measure | Result | Test |
| --- | --- | --- |
| The 33 sample dates of *Calendrical Calculations*, 586 BCE to 2094, as its published code computes them [reingold2018code, `dates.l`]: `chinese`, cycle, year, month, leap and day | 15 of the 15 in 1645–2150; the 18 before refused. The book's day names are `sexagenary`'s, 33 of 33 | `every_sample_date_agrees_or_is_refused_or_is_a_known_difference` (`crates/hyper-calendar/tests/rd_sample_dates.rs`) |
| Chinese New Year 2000, 2020, 2021, 2022, 2023, 2024, 2025 and 2026 against the Hong Kong Observatory's tables [hko-conversion-tables], and 1900 = 31 January against the date in general circulation | 9 of 9 | `other_published_new_years_are_reproduced`, `chinese_new_year_2024_was_the_tenth_of_february` |
| 閏二月 of 2023 beginning 22 March | Reproduced [hko-conversion-tables] | `twenty_twenty_three_had_a_leap_second_month` |
| A child born in June 2000 is 13 *suì* from the lunar new year of 2012 [wikipedia-en-east-asian-age-reckoning]; one at birth, two the day after a New Year's Eve birth | Reproduced | `a_child_born_in_june_2000_turns_thirteen_at_the_new_year_of_2012`, `a_child_born_on_new_years_eve_is_two_the_next_day` |
| The Widow Year from 10 February 2024 [scmp-widow-year-2024] and the double-spring year from 26 January 2009 [scmp-double-spring-2009] | Both | `the_published_widow_and_double_spring_years_are_reproduced` |
| Every double-bright year runs more than 366 days and every widow year fewer than 365, over the Chinese years beginning 1653–2142; all four auguries occur | All | `a_year_with_two_lichun_has_thirteen_months_and_one_with_none_twelve` |
| The double spring of 丁酉 2017, 立春 on 正月初七 and 臘月十九, and 戊戌's only 立春 on its 除夕 [wikipedia-zh-shuangchun] | Reproduced | `the_year_of_the_rooster_2017_had_two_lichun_and_the_dog_year_after_it_one_at_its_end` |
| The four age counts: 立春 on 4 February 2010 [scmp-double-spring-2009], a birth on 31 December two the next day and one on 1 January one that day [wikipedia-en-east-asian-age-reckoning, wikipedia-zh-xusui] | Reproduced | `the_age_counts_turn_at_their_own_boundaries` |
| `dangi-kasi` against the 321 days of 1653–1911 queried from KASI [kasi-lunisolar-conversion] | All 321; the test holds the twelve on either side of its departures and of 1906 | `the_kasi_reading_follows_kasi_where_the_qing_almanac_does_not`; every day of its range round-trips and from 1912 is `dangi`'s, `the_kasi_reading_is_registered_and_round_trips` |
| The almanac's term days of 1645–1733 [liu-chinese-calendar-computation] | The three term days of 1651, 1661 and 1727; 88 major terms a day from the rules, which move no month beyond the five carried | `the_almanac_term_days_give_the_three_days_the_leap_months_turn_on`, `the_almanac_term_days_move_no_month_but_the_five_carried` |
| Chinese New Year 1988 = 17 February and 1985 = 20 February, with the 12th month of 1985 beginning 21 January | Reproduced [hko-conversion-tables] | `seollal_1988_fell_a_day_after_chinese_new_year`, `tet_1985_fell_a_whole_month_before_chinese_new_year` |
| Seollal 1988 = 18 February and Seollal 2024 = 10 February, Dangi 4357 | Reproduced; both days are KASI's [kasi-lunisolar-conversion] | `seollal_1988_fell_a_day_after_chinese_new_year`, `seollal_2024_was_the_tenth_of_february_and_the_year_is_dangi_4357` |
| Every month of the 125 Chinese years from 31 January 1900 to 10 February 2024, 1 546 months, against the Purple Mountain Observatory's table of the promulgated calendar [pmo-calendar-1900-2025] | Every first day, leap month and length; the rules alone miss one, the fourth month of 1906, which the correction supplies; the table and the Hong Kong Observatory's agree on it and on the Beiyang months of 1914, 1916 and 1920 that a 120°E reading would move [hko-conversion-tables] | `the_chinese_calendar_is_the_purple_mountain_observatorys_from_1900_to_2024`, `without_the_almanac_the_rules_miss_one_month_in_the_table`, `the_fourth_month_of_1906_began_on_the_day_the_almanac_gave` |
| Every first day of a month of `dangi` from January 1900 to December 1913, 174 months, against KASI's conversion data, queried 2026-09-27 | All 174, with month and leap month; under the published code's Seoul and 127°30′E offsets five of them would be a day late. Measured for this document; the test holds the five and the fourth month of 1906 to KASI's days, and `dangi` to `chinese` on every day of 1900–1911, whose months the Observatory's table holds | `before_1912_the_months_begin_where_kasi_has_them_and_not_where_seoul_would` |
| The 29 first days of 1652–1906 and the 5 leap months of 1645–1805 where the rules and the almanac part, against the month's opening line in the Veritable Records [qing-shilu] | All 34 as the Records have them, by sexagenary day; the rules alone give none of them | `each_month_the_almanac_moved_begins_on_the_veritable_records_day`, `the_leap_months_the_almanac_moved_are_where_the_veritable_records_have_them` |
| Every month of 1645–1911 whose first day the transcription of the Veritable Records gives, 3 260 of the 3 303, 宣統政紀 included [qing-shilu] | All, by sexagenary day; the rules alone miss 39: the 29 first days, and 10 months of the five moved leap years, the leap month and the month beside it that the rules number differently. Every correction has a month of the Records to test it. Left out, as the data file's header lists: 2 lines 20 and 30 days from any possible first day, and 41 months for which the pattern found no opening line | `every_month_the_veritable_records_open_begins_on_their_day`, `without_the_corrections_the_rules_miss_the_months_the_corrections_carry` |
| The corrected calendar against Liu's reconstruction of the Qing calendar and the Republic's, every month of 1645–1935 [liu-chinese-calendar-computation] | Every first day and every leap month; without the corrections, 29 first days and 5 leap months differ. Measured for this document, not a test | — |
| The 105 months of 1653–1911 whose first day differs between a Beijing and a Seoul reading of the rules, against KASI | KASI has Beijing's day in 80 and Seoul's in 25; in 24 of the 25 the Records and Liu have that later day too, and the corrections carry it. Measured for this document, not a test | — |
| `dangi` against KASI on the 216 days of 1653–1911 that begin a month under one of the Beijing rules, the Seoul rules and the corrected calendar and not another, and on the first day of each of the 103 leap months any of them gives, queried 2026-09-27 [kasi-lunisolar-conversion] | 210 of 216 and 102 of 103; the rest are the three months of 1653 and 1841 KASI leaves the Qing almanac in (*The almanac before 1912*). Measured for this document; the 25 corrections KASI shares, three of the leap months and the three exceptions are a test | `before_1900_the_almanacs_months_are_kasis_but_for_three` |
| Korean and Chinese new years over 1900–2049 | Differ in 9 years, never by more than a day | `the_two_calendars_disagree_only_occasionally` |
| Tết 1968 = 29 January north, 30 January south | Reproduced [wikipedia-vi-tet] | `tet_1968_fell_on_different_days_in_the_north_and_the_south` |
| Tết 1969 = 16 February north, 17 February south and China | The northern day reproduced [nhandan-tet-trong-cay-2019]; the southern as computed | `tet_1969_fell_a_day_before_chinese_new_year` |
| Tết 1985 = 21 January, with 閏二月 from 21 March | Reproduced [wikipedia-en-vietnamese-calendar] | `tet_1985_fell_a_whole_month_before_chinese_new_year` |
| Tết 2024 = 10 February, numbered 2024 | Reproduced; no Vietnamese publication of the date was read | `tet_2024_was_the_tenth_of_february_and_the_year_is_numbered_2024` |
| Tết against Chinese New Year over 1968–2049 | Every difference is one day or one lunation | `the_calendar_sometimes_differs_from_the_chinese_one_since_1968` |
| Mean and apparent terms over the year 2000 | Disagree on more than ten days | `the_mean_and_apparent_solar_term_rules_disagree_somewhere` |
| The meridian tables | Read in order; the Korean half-hour eras in force in the years named | `the_meridian_table_is_read_in_order`, `the_half_hour_zones_are_read_from_the_table` |
| The corrections: each names a first day or a term day the rules really give, moves it one day, is sorted, and has a source; a term correction, of 冬至 or another term, moves the index at one midnight and the solstice's day with it | All | `the_almanac_corrections_are_live_and_move_a_day_at_most`, `the_almanac_term_corrections_are_live_and_move_a_day_at_most`, `a_term_correction_moves_the_index_and_a_solstice_correction_the_solstice` |
| The rules at 116°25′E against 116°23′E, 1645–1929 | One month apart, the second of 1687: 14 March at 116°25′ and 13 March at 116°23′; the Records have 13 March, 己酉, and the calendar follows them by a correction | `the_two_readings_of_the_beijing_meridian_differ_once` |
| The two partial years of the Korean table, 1 January–20 March 1954, 1 January–9 August 1961 | Every new moon, zhōngqì index and calendar date the same under the published code's offset and the table's | `the_year_keyed_eras_give_the_days_the_day_keyed_changes_give` |
| Round trips over the Qing years 1645–1911, where the corrections live, across 1929, across every Korean change, across 1968 and at both ends of the range | Every day in a release build; in a debug one a sample that holds every new year of 1645–1912, every day a correction moves and the day before each | `the_calendar_round_trips_over_the_qing_years` (in `chinese` and `dangi`), `the_calendar_round_trips_across_the_1929_meridian_change`, `the_calendar_round_trips_across_every_meridian_change`, `the_calendar_round_trips_across_the_1968_change`, `the_calendar_round_trips_at_both_ends_of_its_range` |
| Structure: months of 29 or 30 days, years of 12 or 13, the leap month after the month it repeats, never a leap first month, about 37 leap years a century | Holds over 4600–4700 | `months_are_twenty_nine_or_thirty_days_and_years_twelve_or_thirteen_months`, `every_leap_month_immediately_follows_the_month_it_repeats`, `leap_months_are_rare_and_never_the_first_month` |

**The minute at midnight.** The conjunctions from `hc-astro` land within
about a minute of the truth and the solar longitude within about 1″, under
half a minute of time. When a conjunction or a zhōngqì falls within about a
minute of local midnight, the day assigned can be wrong by one, and a wrong
day for a zhōngqì or the solstice can move a leap month by a month, as the
1985 example shows an hour doing. The Hong Kong Observatory says the same of
its own tables: the uncertainty decades ahead "may be up to a few minutes",
and it lists the new moons of 2057, 2089 and 2097 and certain solstices and
equinoxes between 2021 and 2084 as the cases where its dates may be a day
off [hko-conversion]. The margins do get small: 雨水 of 1985 fell at 00:07
Hanoi time. No test measures the margin; the published new years above are
evidence that the rule and the astronomy agree where they were checked, not
a guarantee elsewhere.

**The day boundary.** A calendar here is the rule at one meridian, and the
meridian is a table of years. The two windows in which that is coarser
than the published code are tested above to contain no event that moves.
The Chinese pre-1929 offset is a mean-time reading of a reference that was
apparent time before 1914; the Observatory's table, checked from 1900,
finds no month that the difference moves.

**Where the sources are thin or disagree.**

- The meridians in `vietnamese` are the calendar's, not the civil clock's.
  The north kept UT+7 civil time from 2 September 1945, with UT+8 only in
  the zones of fighting from 1947 [wikipedia-vi-gio-viet-nam]; the 1967
  decision moved the calendar's meridian, which is what the published code
  encodes and what accounts of the decision say
  [wikipedia-en-vietnamese-calendar], though the decision's text was not
  read [vn-decision-121-cp]; the south's civil time was UT+8 from 1 January
  1960 [tienphong-two-zones].
- The years the Dangi count was official: [wikipedia-en-korean-calendar]
  gives 1945 to 1961; [encykorea-dangun-giwon] dates it by the Act on Era
  Names of 25 September 1948 and its replacement of 2 December 1961,
  effective 1 January 1962, and `dangi` and this document follow the
  statutes' dates.
- Korea adopted the Shíxiàn rules in 1653 [wikipedia-ko-siheollyeok];
  Vietnam's date is not established by any source read here. In 1653–1654
  KASI's data differ from Liu's Qing calendar on four of the days queried,
  and nothing read here says whether Joseon's first Shíxiàn almanacs were
  computed at Hanseong or copied from Beijing's.
- The move of the Korean calendar to 135°E: the published code has
  1 January 1912 and the ICU4X report 1913 [icu4x-issue-6455]; no month of
  either year differs between the two, so nothing here decides it.
- The true conjunction was first used in China by the Wuyin calendar of
  619, given up in 645 after four long months in a row, and settled by the
  Línde calendar of 665 with 進朔 against such runs [wikipedia-zh-dingshuo,
  wikipedia-zh-wuyinyuanli, wikipedia-zh-lindeli]; the point bears on the
  Japanese systems, not on the three calendars here.
- Tết 2024 = 10 February is reproduced by the crate and matches the
  Chinese side of the published tables, but no Vietnamese publication of
  it was read.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [reingold2018] | The rules, the suì, the meridian histories | Not read directly; the published code was |
| [reingold2018code] | `chinese-location`, `korean-location`, `korean-year`, `vietnamese-location`, `chinese-epoch`, `current-major-solar-term`, `chinese-no-major-solar-term?`, `chinese-prior-leap-month?`, `chinese-winter-solstice-on-or-before`, `chinese-new-year-in-sui`, `chinese-new-year-on-or-before`, `chinese-from-fixed`; `chinese-age`, `chinese-year-marriage-augury`, `widow`, `blind`, `bright`, `double-bright`, `current-minor-solar-term` | Yes, 2026-09-25; the last seven 2026-09-26 |
| [wikipedia-en-east-asian-age-reckoning] | The pre-modern Chinese count of *suì* and the child born in June 2000; the Korean count from 1 January, its formula and the child born on 31 December; the "year age" | Yes, 2026-09-26, § People's Republic of China; the Korean section 2026-09-29 |
| [wikipedia-zh-xusui] | 虚岁 from 春节 and from 1 January, one at birth in Korea and nothing at birth in Vietnam and parts of China; the 除夕 and 1 January births | Yes, 2026-09-29 |
| [guokr-xusui] | 虚岁 turning at 立春 in some places | Yes, 2026-09-29; a popular secondary source |
| [wikipedia-en-lichun] | 無春年, 寡婦年 and 盲年, and marriage in such a year thought unlucky | Yes, 2026-09-26 |
| [wikipedia-zh-lichun] | 双春年, 无春年, 寡妇年 and 盲年 | Yes, 2026-09-29 |
| [hko-double-spring] | The name 雙春兼閏月 | Yes, 2026-09-29, the announcement of a video only |
| [wikipedia-zh-shuangchun] | 丁酉 2017 with two 立春, and 戊戌's 立春 on its 除夕 | Yes, 2026-09-29 |
| [wikipedia-en-vietnamese-zodiac] | The Vietnamese zodiac, with the water buffalo and the cat | Yes, 2026-09-29 |
| [scmp-widow-year-2024] | The Year of the Dragon of 2024 as a Widow Year | Yes, 2026-09-26 |
| [scmp-double-spring-2009] | The lunar year from 26 January 2009 with 立春 on 4 February 2009 and 4 February 2010 | Yes, 2026-09-26 |
| [samr-gbt33661] | The standard's number, title, drafting body and drafters, dates of issue, force and review | The catalogue entry, 2026-09-25; the standard itself was retrieved as a PDF that could not be read here |
| [wikipedia-zh-nongli] | The standard's rules; 順治二年 and the 定氣 reform; the 中國天文年曆; the year counts | Yes, 2026-09-25 |
| [wikipedia-en-chongzhen-calendar] | The Chongzhen treatise, its authors, *píngqì* to *dìngqì*, the Shunzhi promulgation | Yes, 2026-09-25 |
| [wikipedia-ko-siheollyeok] | Joseon's adoption in 1653, Kim Yuk, 정기법 | Yes, 2026-09-25 |
| [wikipedia-en-korean-calendar] | 1653 and 1725; the calendar as maintained by the government; the Dangi count | Yes, 2026-09-25 |
| [encykorea-dangun-giwon] | Act No. 4 of 25 September 1948 making the Dangi count official, and Act No. 775 of 2 December 1961 replacing it from 1 January 1962 | Yes, 2026-09-26; the statutes themselves were not read |
| [wikipedia-en-time-in-south-korea] | The years 1908, 1912, 1954 and 1961 and who changed the zone | Yes, 2026-09-25 |
| [wikipedia-en-time-in-china] | Beijing's reference before standard time; the calendar's reference moving to UT+8 in 1928–1929 | Yes, 2026-09-25 |
| [kasi-lunisolar-conversion] | KASI as the publisher; the range of its service; the Gregorian calendar as official under the 천문법; the lunar date of every month's first day of 1900–1913, of both sides of each Seoul–Beijing disagreement of 1653–1911, of both days of each of the 28 earlier corrections and the 5 leap months, of the 216 days and 103 leap months against which `dangi` was measured, of 24 April 1906 and of Seollal 1988 and 2024 | The page 2026-09-25; the dates 2026-09-27, through the service's own lookup (`/life/solc`); KASI's FAQ, its 월력요항 page and its almanac scans (`/almanac/pageView/27`) returned server errors |
| [pmo-calendar-1900-2025] | The promulgated calendar day by day for 1900–2025; the compilation notes on the 時憲書, the Republic's almanacs and GB/T 33661-2017 as its sources | Yes, 2026-09-27, the whole table read as text |
| [liu-chinese-calendar-computation] | The Qing calculation: Tycho's theory to the 1730s, the 1742 method, the Beijing meridian, apparent time; the Beiyang change to mean time; more than 200 corrections to a modern computation for 1645–1911 from two books; Lǐ Tiānjīng's rule in 1645; the Dàtǒng years 1667–1669 and the moved leap month of 1669–1670; the reconstruction measured against and its calendrical solar terms of 1645–1733, carried as `almanac_solar_term_days` | Yes, 2026-09-27; the terms table decoded 2026-09-29: the page, the table data its conversion page loads (`table_c.js`) and the calendrical solar terms and notes its calendar page loads (`index_c.js`); the books it draws on (the Observatory's 《新编万年历》, Zhang Peiyu's 《三千五百年历日天象》) were not read, and its Shixian computation pages would not load |
| [qing-shilu] | The first day of each month of 1644–1911 by its sexagenary name, and the leap months, as the Veritable Records open each month; the 34 months the corrections carry | Yes, 2026-09-27, in 殆知閣's transcription, every reign from 順治 to 光緒 and the 宣統政紀; the printed edition and the scans the transcription links to were not read |
| [aslaksen2010] | 1805 as the example of the meridian: 處暑 at 0h07m on 24 August at 120°E, about seven minutes before midnight at Beijing, and his reading of the leap month | Yes, 2026-09-27, §4.6 of the draft of 17 July 2010 |
| [qingshigao-shixian] | The Dàtǒng method restored during the Calendar Case, the 閏十二月 of 康熙八年 and its move to 康熙九年二月 on Verbiest's showing | Yes, 2026-09-27, 卷四十五 in Wikisource's text |
| [icu4x-issue-6455] | A report that the Korean almanacs to 1912 give Beijing time and match the Qing calendar | Yes, 2026-09-27; a secondary report, and the almanac pages it cites could not be opened |
| [vn-decision-121-cp] | The decision as the module cites it | Not read; the legal database refused the request on 2026-09-25 |
| [wikipedia-vi-gio-viet-nam] | The decision's date, number and wording; the north's time since 1945; the south's UT+8 from 1960 to 1975 | Yes, 2026-09-25 |
| [wikipedia-vi-tet] | The decision as the reason Tết 1968 fell on 29 January in the north and 30 January in the south | Yes, 2026-09-25 |
| [tienphong-two-zones] | Decree 362-TTP of 30 December 1959; UT+8 in the south from 1 January 1960; the return to UT+7 on 13 June 1975 | Yes, 2026-09-25 |
| [wikipedia-en-vietnamese-calendar] | The 1985 case and its leap month of 21 March to 19 April; the cat and the buffalo; the book dated 辛卯年, 1831 | Yes, 2026-09-25; re-read 2026-09-29 |
| [lichhuongque-press] | The Ban Lịch Nhà nước and the VAST Information Centre; 2007 and 2030 as years of difference | Yes, 2026-09-25; a secondary account |
| [hko-conversion] | The tables' range and the Observatory's own caveat about midnight | Yes, 2026-09-25 |
| [hko-conversion-tables] | The first day of the first lunar month in 1968, 1985, 1988, 2000 and 2020–2026; the 12th month of 1985 from 21 January; 閏二月 of 2023 from 22 March; the 4th month of 1906 from 24 April; the months of 1914, 1916 and 1920 beginning 17 November, 3 February and 10 November | Yes, 2026-09-25; 1906, 1914, 1916 and 1920 2026-09-27 |
| [nhandan-tet-trong-cay-2019] | Tết Kỷ Dậu on 16 February 1969 in the north | Yes, 2026-09-25 |
| [wikipedia-zh-dingshuo], [wikipedia-zh-wuyinyuanli], [wikipedia-zh-lindeli] | The history of the true conjunction that `lunisolar`'s `ConjunctionMode` summarises: 619, 645, 665 and 進朔 | Yes, 2026-09-25 |

## Code

`crates/hc-calendars-lunar/src/lunisolar.rs` holds the engine:
`MeridianEra`, `MonthStartCorrection`, `MajorTermCorrection`, `SolarTermMode`,
`ConjunctionMode` and `MeanMotionModel`,
`LunisolarParameters` with `meridian_era`, `midnight`,
`winter_solstice_on_or_before`, `new_moon_on_or_after`, `major_solar_term`,
`computed_major_solar_term`,
`has_no_major_solar_term`, `prior_leap_month`, `new_year_in_sui` and the
conversions, and `CHINESE_EPOCH`. `chinese.rs`, `dangi.rs` and
`vietnamese.rs` are parameter sets on it: `MERIDIANS`, `PARAMETERS`,
`ENGINE`, `new_year` (or `tet`), in `chinese` also `ALMANAC_CORRECTIONS`,
`ALMANAC_TERM_CORRECTIONS`, `almanac_solar_term_days`, `reckoned_age`,
`reckoned_age_at_lichun`, `lichun_day`, `reckoned_age_at_new_year_day`,
`year_age`, `marriage_augury`, `MarriageAugury` with `chinese_names` and
`AuguryName`, in `dangi` also `KASI_CORRECTIONS`, `KASI_TERM_CORRECTIONS`,
`KASI_PARAMETERS`, `KASI_ENGINE` and `DangiKasiCalendar`, and in `vietnamese` also
`SOUTHERN_MERIDIANS` and `SOUTHERN_PARAMETERS`.

Anchors: in `lunisolar`, `the_meridian_table_is_read_in_order`,
`a_term_correction_moves_the_index_and_a_solstice_correction_the_solstice`,
`month_eleven_contains_the_winter_solstice`,
`the_mean_and_apparent_solar_term_rules_disagree_somewhere`,
`a_leap_month_repeats_the_ordinal_of_the_month_before_it`,
`the_sexagenary_month_anchor_reproduces_the_traditional_rule`,
`month_eleven_always_carries_the_rat_branch`; in `chinese`,
`chinese_new_year_2024_was_the_tenth_of_february`,
`twenty_twenty_three_had_a_leap_second_month`,
`other_published_new_years_are_reproduced`,
`the_fourth_month_of_1906_began_on_the_day_the_almanac_gave`,
`the_almanac_corrections_are_live_and_move_a_day_at_most`,
`the_almanac_term_corrections_are_live_and_move_a_day_at_most`,
`each_month_the_almanac_moved_begins_on_the_veritable_records_day`,
`the_leap_months_the_almanac_moved_are_where_the_veritable_records_have_them`,
`the_two_readings_of_the_beijing_meridian_differ_once`,
`the_calendar_round_trips_across_the_1929_meridian_change`,
`the_calendar_round_trips_over_the_qing_years`,
`the_range_is_refused_rather_than_extrapolated`,
`the_almanac_term_days_give_the_three_days_the_leap_months_turn_on`,
`the_almanac_term_days_move_no_month_but_the_five_carried`,
`the_age_counts_turn_at_their_own_boundaries`,
`the_year_of_the_rooster_2017_had_two_lichun_and_the_dog_year_after_it_one_at_its_end`;
in `dangi`, `the_kasi_reading_follows_kasi_where_the_qing_almanac_does_not`,
`the_calendar_round_trips_over_the_qing_years`,
`seollal_1988_fell_a_day_after_chinese_new_year`,
`the_two_calendars_disagree_only_occasionally`,
`the_half_hour_zones_are_read_from_the_table`,
`the_year_keyed_eras_give_the_days_the_day_keyed_changes_give`,
`before_1912_the_months_begin_where_kasi_has_them_and_not_where_seoul_would`,
`before_1900_the_almanacs_months_are_kasis_but_for_three`,
`the_sexagenary_year_matches_the_chinese_one_despite_the_offset`,
`the_calendar_is_not_the_chinese_one_even_where_they_agree`; in
`vietnamese`, `tet_1968_fell_on_different_days_in_the_north_and_the_south`,
`tet_1969_fell_a_day_before_chinese_new_year`,
`tet_1985_fell_a_whole_month_before_chinese_new_year`,
`the_calendar_sometimes_differs_from_the_chinese_one_since_1968`,
`the_calendar_round_trips_across_the_1968_change`,
`the_vietnamese_zodiac_has_the_cat_and_the_buffalo`. In
`tests/calendars.rs`,
`the_four_lunisolar_calendars_agree_on_the_day_of_the_month_when_they_agree_at_all`
holds the three and the Tenpō calendar to the same month and day on more
than 1 800 of 2 000 days from 1860, and `tests/chinese_published.rs` holds
`chinese` to the Purple Mountain Observatory's table, 1900–2024, in
`tests/data/chinese_month_lengths_1900_2024.txt`, and `tests/chinese_qing.rs`
holds it to the Veritable Records' first days, 1645–1911, in
`tests/data/qing_veritable_records_month_starts.txt`. English month names are in `hc-i18n`.
