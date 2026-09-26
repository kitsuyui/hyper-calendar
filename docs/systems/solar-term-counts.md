# Days counted from a solar term: 寒食 and 한식, and the 伝統的七夕

Backs `hc-seasons`: the module `cold_food` and `moon_calendar`'s
`traditional_tanabata`; and in `hc-holiday`, the 寒食 of `chinese-folk` and
the 한식 of `korean-folk`.

The solar terms these days are counted from, the meridian a day is read
at, and how the instants compare with the published almanacs are in
[solar-terms-and-pentads.md](solar-terms-and-pentads.md); this document
does not repeat them.

## What it is

Two East Asian days are dated neither in a month nor on a solar term, but
by counting from one.

**寒食, the Cold Food Day**, when no fire was lit and food was eaten cold,
was kept in China from antiquity, reaching Korea as 한식 and Vietnam as
Tết Hàn thực. In China it was "冬至后一百五日", the 105th day after the
winter solstice, with some texts giving 106, and it fell a day or two
before 清明; after the 時憲曆 of 1645 shortened the interval between the
solstice and 清明, it was kept on the day before 清明 instead, "为了维持寒
食节在清明节前一、两日的风俗" [wikipedia-zh-hanshi]. Korea kept the count:
한식 is "동지로부터 105일째 되는 날", in the second or third lunar month,
and "어느 해나 청명절 바로 다음날이거나 같은 날에 든다" [aks-hansik]. The
Korea Astronomy and Space Science Institute prints its date in each year's
월력요항 [kasi-wollyeok]. Vietnam's Tết Hàn thực moved to a lunar date, the
third of the third month [vnplus-tet-am-lich].

**伝統的七夕**, the traditional Tanabata, is the National Astronomical
Observatory of Japan's statement of the old 七夕 — the seventh day of the
seventh lunisolar month — for a country that no longer keeps an official
lunisolar calendar [nao-faq-kyureki]. It is a count from the new moon
nearest 処暑, not a date in a month, so it does not depend on which of the
unofficial lunisolar reckonings is followed [nao-faq-tanabata].

## How it works

**寒食 and 한식.** Three reckonings, each carried under its own name as
[policy.md](../policy.md) §5 has it:

| Convention | Rule | Meridian |
| --- | --- | --- |
| `hanshi-solstice-105` | 105 days after the winter solstice | China, UTC+8 |
| `hanshi-eve-of-qingming` | the day before 清明 | China, UTC+8 |
| `hansik` | 105 days after 동지 | Korea, UTC+9 |

The phrase "the 105th day from the solstice" leaves open whether the
solstice's own day is the first of the 105. The Korean dates settle it:
KASI's 한식 is 5 April 2024, 5 April 2025 and 6 April 2026
[kasi-wollyeok], and 동지 fell on 22 December 2023, 21 December 2024 and 22
December 2025 at the Korean meridian. Each 한식 is 105 days *after* the
solstice's day; counting the solstice as day 1 would put each a day
earlier, on 4 April 2024, 4 April 2025 and 5 April 2026, which KASI does
not print. The same reading is taken for the Chinese count, whose "冬至后
一百五日" says "after" in so many words.

Worked example, 한식 2026. The winter solstice of 2025 fell 3 minutes after
midnight Korean time, so 동지 is 22 December 2025. Count 105 days on: 9 to
the end of December, 31 in January (40), 28 in February (68), 31 in March
(99), and 6 in April, which is 105 — 6 April 2026, a Monday, as KASI
prints it. 청명 was 5 April, so 한식 is the day after it.

That 한식 falls on 청명 or the day after is the modern solar terms at
work: from the winter solstice to 清明 is about 104 days when the terms
are the true ones, 定気, that this library computes, and 105 days after
the solstice therefore lands on 清明 or just past it. Under the equal
terms of the old calendars, 平気, the interval was seven terms of about
15.2 days, some 106½ days, which fits the texts' 寒食 a day or two *before*
清明; this is arithmetic on the two definitions, not a statement of the
sources, which say only that the 時憲曆 shortened the interval.

**伝統的七夕.** The Observatory's rule, in full: "二十四節気の処暑（太陽黄経が
150度になる瞬間）を含む日かそれよりも前で、処暑に最も近い朔（新月）の瞬間を含む
日から数えて7日目" [nao-faq-tanabata]. Find the day holding 処暑; take the
last new moon whose instant falls on that day or before it; the day
holding that new moon is day 1, and 伝統的七夕 is day 7, six days later.
The days are Japan Standard Time's.

Worked example, 2026. 処暑 is 23 August, at 11:19 JST. The last new moon
before the end of that day is that of 13 August, at 02:37 JST — 12 August
in Universal Time, the day of that year's total solar eclipse. Day 1 is 13
August, day 7 is 19 August, as the Observatory's table gives it. Read at
Universal Time, the new moon's day would be the 12th and the answer the
18th; the meridian matters.

## What is carried

`hc-seasons::cold_food::ColdFoodConvention` has the three conventions of
the table, each with its identifier, its name and its meridian, and gives
the day of any Gregorian year; `DAYS_AFTER_SOLSTICE` is the 105. In
`hc-holiday`, `chinese-folk` carries 寒食 as the day before 清明, the
reckoning kept today, and `korean-folk` carries 한식 as an offset of 105
days from 동지 at the Korean meridian, the same day as `Hansik`. The
106-day count is not carried: the source mentions it, "亦有去冬至一百六日
者", without saying who kept it. Vietnam's Tết Hàn thực is a lunar date and
is in `vietnamese-folk`, not here.

`hc-seasons::moon_calendar::traditional_tanabata` gives the 伝統的七夕 of
any year at any meridian; the Observatory's is `Meridian::JAPAN`. The 月遅れ
reckoning is in no source read and is not carried; the 七夕 of 7 July is in `hc-holiday`'s `gosekku`.

## Accuracy

한식 reproduces KASI's three dates of 2024–2026. The 2026 one depends on
a solstice three minutes after midnight, which the solar longitude places
correctly: a solstice three minutes earlier would have put 한식 on 5 April,
which KASI does not print. That 한식 falls on 청명 or the day after is
checked for every year of 1900–2100 at the Korean meridian, as the
Encyclopedia of Korean Culture says it always does, and that the Chinese
solstice count falls one or two days after the eve of 清明 for the same
years.

伝統的七夕 reproduces all forty dates of the Observatory's table,
2011–2050, and for 1990–2099 the day six before it is a new-moon day at
noon on or before 処暑. A new moon or 処暑 within about a minute of
midnight could be put on the wrong day, as for every solar-term date.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [wikipedia-zh-hanshi] | 冬至后第105日 and 一百零五日; the 106 of some texts; 清明前一或二日; the day before 清明 after the 時憲曆 of 1645; Vietnam's 三月初三. Secondary: the classical texts it cites were not read | Yes, 2026-09-26 |
| [aks-hansik] | 한식 "동지로부터 105일째 되는 날", in the second or third month, on 청명 or the day after, about 5 or 6 April | Yes, 2026-09-26 |
| [kasi-wollyeok] | 한식 on 5 April 2024, 5 April 2025 and 6 April 2026; also 단오 and 칠석 of those years, for `korean-folk` | Yes, 2026-09-26 |
| [vnplus-tet-am-lich] | Tết Hàn Thực on the third of the third month | Yes, 2026-09-26 |
| [nao-faq-tanabata] | The rule of 伝統的七夕 and its dates of 2011–2050 | Yes, 2026-09-26 |
| [nao-faq-kyureki] | No official lunisolar calculation in Japan today | Not re-read here; the 雑節 document cites it |

The Korean Wikipedia, "동지", gives 22 December for the solstice of 2025,
as a check of the computed day; the solstices themselves are
`hc-seasons`' own, compared with the almanacs in
[solar-terms-and-pentads.md](solar-terms-and-pentads.md).

## Code

`crates/hc-seasons/src/cold_food.rs`; `traditional_tanabata` in
`crates/hc-seasons/src/moon_calendar.rs`. In `hc-holiday`, the 寒食 entry
of `CHINESE_FOLK` and the 한식 entry of `KOREAN_FOLK` in
`crates/hc-holiday/src/traditions.rs`.

Anchors in `cold_food`: `hansik_falls_where_the_korean_almanac_puts_it`,
`the_solstice_itself_is_not_the_first_of_the_105_days`,
`hansik_is_on_cheongmyeong_or_the_day_after`,
`the_eve_of_qingming_is_the_day_before_it`. In `moon_calendar`:
`the_traditional_tanabata_falls_on_the_observatorys_forty_dates`,
`the_traditional_tanabata_is_the_seventh_day_from_a_new_moon_before_the_end_of_heat`.
In `hc-holiday`'s `tests/traditions.rs`:
`the_korean_folk_days_are_where_the_korean_almanac_puts_them` and
`the_chinese_folk_additions_fall_on_their_days`, which check both tables
against `ColdFoodConvention` for 1950–2100.
