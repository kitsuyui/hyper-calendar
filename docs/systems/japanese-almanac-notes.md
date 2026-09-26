# The Japanese almanac notes: 暦注, 選日, 十二直, 二十八宿, 九星, 六曜 and 七曜

Backs `hc-almanac`: the modules `lower_register`, `selected_days`,
`twelve_directs`, `mansions`, `nine_stars`, `rokuyo`, `seven_luminaries`,
`rules`, `context` and `day_notes`.

## What it is

A **暦注** is a note printed against a day in an almanac, saying what the
day is good or bad for. The National Astronomical Observatory of Japan's
暦Wiki sorts them by where they stood on the page: the 上段 carried the
干支, the 中段 the 十二直, and the 下段 everything else. The method of
assigning a note to days is the 撰日法; the notes came from Chinese works
such as the 大唐陰陽書, and a few, like 八十八夜, are Japanese
[nao-rekiwiki-rekichu]. At the Meiji reform the 中段 and 下段 notes were
struck from the official calendar as superstition [nao-rekiwiki-rekichu].
They survived in commercial almanacs, which is where they are printed
today, and nothing in them is published by the Observatory: its annual
暦要項 carries the solar terms, the 雑節 and the holidays and no 暦注
[nao-rekiyoko-2025].

The notes this library carries, and what each is in the world:

- **暦注下段**, the lower register: 天赦日, 大明日, 受死日 (the black dot),
  the three 悪日 and about twenty others, each a day good or bad in all
  things or for named undertakings [ndl-koyomi-gedan,
  wikipedia-ja-rekichu-gedan].
- **選日**, the other day-choosing notes: 一粒万倍日, 三隣亡, 不成就日, 八専,
  十方暮, 天一天上, 庚申, 甲子 and the 犯土. The National Diet Library groups
  them with the 中段 and 下段 as notes printed before 六曜 spread
  [ndl-koyomi-chudan, ndl-koyomi-sonota].
- **十二直**, the 中段: 建, 除, 満, 平, 定, 執, 破, 危, 成, 納, 開, 閉, one per
  day, named after the direction the handle of the Dipper points at dusk
  [ndl-koyomi-chudan, nao-rekiwiki-junichoku].
- **二十八宿**, the 28 lunar lodges. As a 暦注 they have been assigned, from
  the 貞享暦 on, as a continuous count of days, years and months; before it
  Japan used the 二十七宿 of 宿曜道, which restart every month
  [nao-rekiwiki-28shuku].
- **九星**, the nine stars 一白水星 to 九紫火星 of the 後天定位盤, counted by
  year, by 節月 and by day [wikipedia-ja-kyusei].
- **六曜**, 先勝 to 赤口, keyed to the 旧暦 date; widespread from the end of
  the Edo period [ndl-koyomi-rokuyo]. 大安 is good for weddings and 友引 is
  a day funerals are avoided [ndl-koyomi-rokuyo], and funeral businesses
  and crematoria are sometimes closed on it [wikipedia-ja-rokuyo].
- **七曜**, the Sun, the Moon and the five planets that name the days of
  the week, brought to Japan with the 宿曜経 and used with the 二十七宿 for
  divination before they were a week [nao-rekiwiki-youbi-namae].

Because nobody official publishes these rules, they reach a reader through
publishers, and publishers copy one another. Japanese Wikipedia's 暦注下段
and 一粒万倍日 articles list 岡田芳朗・阿久根末忠 (編著)『現代こよみ読み解き
事典』(1993) as their reference, and こよみのページ names the same book and
岡田芳朗『旧暦読本』 as the basis of its rules [wikipedia-ja-rekichu-gedan,
wikipedia-ja-ichiryu-manbai, koyomi8-rekichu-3]. Four agreeing pages can
therefore be one witness. The National Diet Library's exhibition and
精選版日本国語大辞典 are the independent statements; printed date lists are
the check that a rule was transcribed right.

## How it works

Almost every note is a rule over one of three things: the day's place in
the sixty-day cycle of 干支, the **節月** the day falls in, and — for a few —
the 旧暦 date.

**The 節月.** A 節月 opens on the day of a sectional solar term: 正月 at
立春, 二月 at 啓蟄, and so on to 十二月 at 小寒. It has no leap month. A rule
keyed to the 節月 is **節切り**, one keyed to the 暦月 is **月切り**, and one
keyed to neither is **不断** [nao-rekiwiki-setsugetsu]. The 節月 carries the
branch of its month: 正月 is 寅月, 十一月 子月.

**十二直.** In each 節月, the first day whose branch is the month's branch is
建, and the twelve follow in order. On the day a sectional term begins, the
day repeats the previous day's direct — it 「おどる」 — so that 建 always
falls on the month's own branch [nao-rekiwiki-junichoku, ndl-koyomi-chudan].
The position is therefore `(day branch − month branch) mod 12`.

**二十八宿.** Since 貞享2年正月朔日, 4 February 1685 Gregorian, which was set
to 星宿 and was a Sunday, the 28 have run as a plain count of days
[wikipedia-ja-28shuku, nao-rekiwiki-28shuku]. Twenty-eight is four weeks, so
each lodge always falls on the same weekday. The 二十七宿 instead start each
旧暦 month from a fixed lodge — 室 for 正月, 奎 for 二月 — and run on, 牛
omitted [nao-rekiwiki-28shuku].

**九星.** The year star counts backward one a year and turns at 立春; the
month star counts backward one a 節月 [wikipedia-ja-kyusei,
koyomi8-kyusei-hyo]. The day star counts forward (陽遁) from the 甲子 nearest
the December solstice, which is 一白, and backward (陰遁) from the 甲子
nearest the June solstice, which is 九紫 [wikipedia-ja-kyusei,
koyomi8-rekichu-1]. A period is normally 180 days, twenty nines, so the
star is printed twice at every reversal. こよみのページ finds the nearest
甲子 from the solstice's own position: 甲子 0 through 癸巳 29 take the
preceding 甲子, the rest the following one [koyomi8-rekichu-1]. Every eleven
or twelve years the drift of the solstice makes one period 240 days long;
its last sixty days are the **閏**. The first thirty continue the count;
from the 甲午 thirty days before the next switch the count runs the other
way, and that 甲午 repeats the star before it, 七赤 before a December switch
and 三碧 before a June one [koyomi8-rekichu-1, koyomi8-kyusei-hyo,
wikipedia-ja-kyusei].

**六曜.** 先勝 falls on the first day of 旧暦 正月 and 七月, 友引 on the first
of 二月 and 八月, and so on, then in order day by day
[ndl-koyomi-rokuyo]: the note is `(month + day) mod 6`, 0 being 大安.

**The lower register and the 選日** are tables: a list of 干支 (大明日, 天恩日,
神吉日, 五墓日), a branch or stem per 節月 (受死日, 母倉日, 月徳日, 復日,
一粒万倍日, 三隣亡), a 干支 per 節月 (天赦日, 凶会日), a count of days from
the sectional term (往亡日), a run of the sixty (八専, 十方暮, 天一天上), or a
list of 旧暦 days per 旧暦 month (不成就日) [ndl-koyomi-gedan,
ndl-koyomi-sonota, koyomi8-rekichu-3].

### A worked example: 21 December 2025

1. **干支.** 1 January 2024 was 甲子. 21 December 2025 is 720 days later,
   twelve whole cycles of sixty, so it is 甲子 again: stem 甲, branch 子.
2. **節月.** 大雪, the eleventh sectional term, fell on 7 December 2025
   [nao-rekiyoko-2025], and the next, 小寒, comes in January, so the day is
   in 十一月, 子月.
3. **十二直.** Day branch 子 minus month branch 子 is 0: **建**.
4. **二十八宿.** 1 January 2024 was 畢, position 18 counting 角 as 0. Add
   720 and reduce mod 28: 18 + 720 = 738, and 738 − 26 × 28 = 10, which is
   **虚**.
5. **七曜.** It is a Sunday: **日曜**, the Sun.
6. **天赦日.** Winter, 節月 十月 to 十二月, takes 甲子 [ndl-koyomi-gedan]: the
   day is **天赦日**. **天恩日**, whose fifteen days include 甲子, holds too.
7. **一粒万倍日.** The National Diet Library's two rows give 子月 亥 and 子
   [ndl-koyomi-sonota]; the day is 子: **一粒万倍日**. It is also **甲子**, a
   選日 in its own right.
8. **九星.** The December solstice fell on 22 December at 0:03 JST
   [nao-rekiyoko-2025]; that day is 乙丑, position 1, in the first half of
   the cycle, so the switch is the preceding 甲子 — this very day. It opens
   the 陽遁 with **一白**. The day before, 20 December, was the last day of
   a 陰遁, and also 一白: the doubled star.
9. **六曜.** The new moon fell at 10:43 JST on 20 December
   [nao-rekiyoko-2025-sakugenbo], which is therefore 旧暦 十一月一日, so
   the 21st is 十一月二日: 11 + 2 = 13, and 13 mod 6 = 1, **赤口**. The day
   before, 十一月一日, is 12 mod 6 = 0, 大安, as 十一月 must open.

`day_notes` returns exactly this: 建, 虚, the Sun, 天恩日 and 天赦日 in the
lower register, 一粒万倍日 and 甲子 among the 選日, 一白 for the day and 赤口.

## What is carried

**Twenty-one lower-register notes** in `LowerRegister::ALL` and **fifteen
選日** in `SelectedDay::ALL`, each a name, a gloss and an `AlmanacRule`
value; one function, `rule_applies`, evaluates every rule. The **十二直**,
the **二十八宿** and **二十七宿**, the **three 九星** with their attributions,
the **七曜** over `hc_calendar::Weekday`, and **六曜**, which is
`hc_seasons::rokuyo` re-exported. `day_notes` assembles a day's page.

Every 節月-keyed and 旧暦-keyed note takes a `Meridian`, because Tokyo and
Beijing put the same term on different days several times a century. The
旧暦 date is `hc-seasons`' minimal 定気 derivation, so that 六曜, 不成就日
and the 二十七宿 agree with one another.

**Where publishers differ**, each reading is named in the module
documentation with its source. Two are registered as readings of their own,
because a source states each and a published list tests it:

| Note | Readings | Carried as |
| --- | --- | --- |
| 凶会日, which month | 節月 (歳事暦, うまずたゆまず, Japanese Wikipedia's table); 旧暦 month (こよみる, Japanese Wikipedia's prose) | `LowerRegister::KUENICHI` is the 節月 reading and is what a day's register holds; `lower_register::KUENICHI_BY_LUNISOLAR_MONTH` is the 旧暦 reading, a named rule outside the register so that a page does not print the note twice |
| 九星, a solstice on 癸巳 | The switch is the preceding 甲子 (こよみのページ); the following 甲子 (the base of Japanese Wikipedia's 閏 list) | `SwitchReading::MIZUNOTO_MI_BACK`, used by `day_star`, and `SwitchReading::MIZUNOTO_MI_FORWARD`; `day_star_by` takes either |

The others are named and not registered, because no printed date tests the
alternative:

| Note | Carried | Named, not carried |
| --- | --- | --- |
| 大明日 | The 25-day list of Japanese Wikipedia, こよみる and 歳事暦 | The 21-day list (Japanese Wikipedia, こよみる) and the 19-day list (こよみる) |
| 五墓日, which days | 乙丑, 丙戌, 戊辰, 辛未, 壬辰 (Japanese Wikipedia, こよみる, 歳事暦) | 乙未 and 辛丑 for the wood and metal days (精選版日本国語大辞典); 乙未, 丙辰, 辛丑 (the National Diet Library's list) |
| 五墓日, for whom | Everyone, as こよみのページ computes it | Only a person whose birth-year 納音 matches (Japanese Wikipedia, 精選版日本国語大辞典, 歳事暦, こよみる) |
| 三箇の悪日 | Every row, for everyone, as many commercial almanacs print it | One 節月 per person, the one whose branch is their birth year's (the National Diet Library and every other table read) |
| 凶会日, which table | The 貞享暦 table, seventy days | The 宣明暦 table, with twelve more |
| 一粒万倍日 | The union of the National Diet Library's two methods, which it says are now used together | Either method alone |
| 九星 閏 | Last sixty days of a 240-day period, reversing at the 甲午 | A 閏 wherever a 甲午 falls within a day of a solstice, which Japanese Wikipedia says needs adjustments it does not describe |

**Not carried at all**: the 神吉日 suppression rule, which no source can
state; the per-mansion lists of undertakings, on which publishers diverge;
納音, so neither per-person form above can be computed; 臘日; the mansion the
Moon is actually in. The crate README lists these gaps.

## Accuracy

The rules are exact arithmetic on the day count, the 節月 and the 旧暦 date;
what can go wrong is a transcribed table, the day a solar term is put on,
and the 旧暦 derivation.

**Tables against independent statements.**

- The 凶会日 table is the National Diet Library's table less eleven
  entries: its 81 hold all seventy, and ten of the eleven extras are among
  the twelve Japanese Wikipedia brackets as struck by the 貞享暦. The
  eleventh, 甲辰 in 九月, stands where Wikipedia's bracketed entry is 庚寅,
  and Wikipedia's 丑節 癸丑 is not in the Library's table
  (`the_ndl_evil_gathering_table_is_the_jokyo_table_and_eleven_more`).
- The 一粒万倍日 table is, month by month, the union of the National Diet
  Library's two rows (`the_grain_table_is_the_union_of_the_two_ndl_methods`).
- The 十二直 anchor — 建 on the month's own branch, the repeat on every
  sectional term — agrees with the 暦Wiki's and the Library's tables; over
  ten years of consecutive days there are exactly 120 repeats and no skip.

**Against published date lists.**

- 凶会日 by 旧暦 month: all 33 days こよみる prints for 2025, four of them in
  the leap sixth month, and no other day
  (`the_lunisolar_reading_of_the_evil_gathering_matches_the_published_2025_dates`).
  The 節月 reading gives 30 days that year; the two share 22, so 19 days of
  2025 carry the note under one reading only.
- 九星 閏: Japanese Wikipedia lists the twenty seasons from 1882 to 2100 that
  hold a 閏. The forward reading of a 癸巳 solstice reproduces all twenty;
  the back reading reproduces fifteen and, in each of the other five, the
  alternative the list gives in parentheses
  (`both_readings_put_the_leap_where_the_published_list_does`). Under the
  back reading, 閏 periods open on 23 November 2019, 24 May 2031 and 26 May
  2042. The doubled 七赤 and 三碧 are checked on the 閏 of 2019 and of 2020.
  No almanac page for a day inside a 閏 has been checked.
- The switch days of 2024 to 2026, the doubled star at the 2025 and 2026
  reversals, and the year, month and day stars printed for a handful of
  days; the 2024 and 2025 天赦日; every 往亡日 of 2026; the whole 2024 and 2025
  一粒万倍日 and 2025 三隣亡; the 2025 八専, 十方暮 and 天一天上 windows; the 1685
  epoch of the 二十八宿, reproduced 339 years back with its Sunday. These
  lists are cited in the tests and were not re-read for this document.

**The day a term falls on.** 立秋 2025 fell at 22:52 JST on 7 August; the
whole of that day is 申月, which is what makes it the autumn 天赦日, and the
crate puts it there. A term within about a minute of local midnight can
still be put on the wrong day, which would move every 節切り note of that
boundary.

**The 旧暦.** Over 2024 to 2033 `hc-seasons`' derivation differs from the
Japanese 旧暦 on 89 days, all from 25 August to 21 November 2033; 六曜,
不成就日, the 二十七宿 and the 旧暦 凶会日 follow it there.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [nao-rekiwiki-rekichu] | 暦注; the 上中下 division; 撰日法; the 大唐陰陽書; the Meiji abolition | Yes, 2026-09-26 |
| [nao-rekiwiki-setsugetsu] | The 節月; 節切り, 月切り and 不断 | Yes, 2026-09-26 |
| [nao-rekiwiki-junichoku] | The 十二直, their readings, 「おどる」, the table by 節月 | Yes, 2026-09-26 |
| [nao-rekiwiki-28shuku] | The 28 in their four quarters; the continuous count from the 貞享暦; the 二十七宿 and their month table | Yes, 2026-09-26 |
| [nao-rekiwiki-youbi-namae] | The 七曜; the 宿曜経; their use with the 二十七宿 | Yes, 2026-09-26 |
| [nao-rekiyoko-2025] | 大雪 on 7 December and 冬至 at 0:03 JST on 22 December 2025 | Yes, 2026-09-26 |
| [nao-rekiyoko-2025-sakugenbo] | The new moon at 10:43 JST on 20 December 2025 | Yes, 2026-09-26 |
| [ndl-koyomi-rokuyo] | 六曜, its order by 旧暦 month, 大安 and 友引 | Yes, 2026-09-26 |
| [ndl-koyomi-chudan] | The 十二直 and the 月建; the repeat on the 節入り day | Yes, 2026-09-26 |
| [ndl-koyomi-gedan] | 天赦日 by season from 立春; 往亡日; the three 悪日 by birth year; its 凶会日 and 五墓日 lists | Yes, 2026-09-26 |
| [ndl-koyomi-sonota] | 八専, 十方暮, 天一天上, 三隣亡, the two 一粒万倍日 methods, 不成就日 as 月切り | Yes, 2026-09-26 |
| [wikipedia-ja-rekichu-gedan] | The 干支 lists; the 21-day 大明日; 五墓日 by 納音; the 凶会日 table with its bracketed entries, and its prose on 月切り and 72; the three 悪日 | Yes, 2026-09-26 |
| [wikipedia-ja-ichiryu-manbai] | The 一粒万倍日 table | Yes, 2026-09-26 |
| [wikipedia-ja-kyusei] | The attributions; the nearest-甲子 switch; the two readings of 癸巳; the 閏 and its doubled star; the list of 閏 seasons, 1882–2100 | Yes, 2026-09-26 |
| [wikipedia-ja-rokuyo] | 友引 and the closing of crematoria | Yes, 2026-09-26 |
| [wikipedia-ja-28shuku] | The 1685 epoch at 星宿, a Sunday | Yes, 2026-09-26 |
| [koyomi8-rekichu-1] | The 九星 procedure and its 閏; the 十二直 and 二十八宿 rules | Yes, 2026-09-26 |
| [koyomi8-rekichu-3] | The 下段 tables; 五墓日 for everyone; the three 悪日 by birth year and for everyone; the books its rules rest on | Yes, 2026-09-26 |
| [koyomi8-kyusei-hyo] | The 定気 節切り year; the 閏 procedure, called its own | Yes, 2026-09-26 |
| [koyomil-kuenichi] | The five readings of 凶会日 in use; its own, the 貞享暦 by 旧暦 month; the 2025 dates | Yes, 2026-09-26 |
| [koyomil-daimyonichi] | The 25-, 21- and 19-day 大明日 | Yes, 2026-09-26 |
| [koyomil-gomunichi] | 五墓日 by 納音 | Yes, 2026-09-26 |
| [koyomil-taikanichi] | The three 悪日 by birth year and for everyone | Yes, 2026-09-26 |
| [saijigoyomi-gedan] | The 貞享暦 凶会日 headed 節切り; 五墓日 by 納音; the three 悪日; the 25-day 大明日 | Yes, 2026-09-26 |
| [linderabell-kuenichi] | The 貞享暦 凶会日 「節切りの月毎」 | Yes, 2026-09-26 |
| [kotobank-gomunichi] | 精選版日本国語大辞典's 五墓日 | Yes, 2026-09-26 |
| [kotobank-kuenichi] | 精選版日本国語大辞典's 凶会日, with 旧暦正月 庚戌・辛卯・甲寅 | Yes, 2026-09-26 |
| [okada-akune1993] | The rule tables at the root of Japanese Wikipedia's and こよみのページ's | Not read; its record from CiNii Books, 2026-09-26 |
| [okada-kyureki-dokuhon] | Named by こよみのページ as a basis of its rules | Not read |

The published date lists the tests cite from 吉日カレンダー, zired, arachne.jp,
JAL SKYWARD+, こよみる's daily pages, 暦注下段ナビ, KOYOMI NOTE and 開運道 (which
states it follows 天象学会『萬年暦』, not read) were not re-read for this
document.

## Code

`crates/hc-almanac/src/`: `context.rs` (the 節月 and the shared day
context), `rules.rs` (`AlmanacRule` and `rule_applies`),
`lower_register.rs`, `selected_days.rs`, `twelve_directs.rs`,
`mansions.rs`, `nine_stars.rs`, `rokuyo.rs`, `seven_luminaries.rs` and
`day_notes.rs`.

Anchors:
`the_ndl_evil_gathering_table_is_the_jokyo_table_and_eleven_more`,
`the_lunisolar_reading_of_the_evil_gathering_matches_the_published_2025_dates`,
`the_published_days_of_heavens_pardon_match`,
`a_solar_term_that_arrives_at_ten_at_night_still_opens_its_month_that_day`,
`the_published_2026_days_of_going_out_and_perishing_match`,
`the_published_2024_lower_register_entries_match` (lower register);
`the_grain_table_is_the_union_of_the_two_ndl_methods`,
`the_published_2025_grain_days_match_month_by_month`,
`the_published_2025_sanrinbo_days_match_month_by_month`,
`the_leap_month_uses_the_row_of_the_month_it_follows` (選日);
`the_direct_repeats_on_the_day_a_sectional_term_begins`,
`the_directs_repeat_but_never_skip_at_a_sectional_term` (十二直);
`the_cycle_reproduces_the_1685_epoch_three_centuries_back`,
`the_twenty_seven_mansion_reset_reproduces_the_koyomi_wiki_examples`
(二十八宿);
`the_switch_is_the_nearest_sexagenary_head_not_the_preceding_one`,
`the_published_switch_days_match`,
`the_day_star_repeats_across_each_reversal`,
`both_readings_put_the_leap_where_the_published_list_does`,
`a_leap_doubles_seven_red_in_winter_and_three_jade_in_summer`,
`the_last_thirty_days_of_a_leap_period_count_the_other_way` (九星);
`the_twenty_first_of_december_2025_carries_the_strongest_combination`
(`day_notes`, the worked example's day).
