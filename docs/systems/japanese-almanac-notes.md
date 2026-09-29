# The Japanese almanac notes: 暦注, 選日, 十二直, 二十八宿, 九星, 六曜, 七曜, 納音 and 臘日

Backs `hc-almanac`: the modules `lower_register`, `selected_days`,
`twelve_directs`, `mansions`, `mansion_undertakings`, `nine_stars`,
`rokuyo`, `seven_luminaries`, `nayin`, `rounichi`, `rules`, `context` and
`day_notes`.

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
  [nao-rekiwiki-28shuku]. Commercial almanacs print for each what its day
  favours and forbids — 角宿 good for cutting cloth and raising a
  ridgepole, bad for funerals [saijigoyomi-28shuku,
  linderabell-28shuku] — and neither the Observatory nor the Library
  publishes such a list.
- **九星**, the nine stars 一白水星 to 九紫火星 of the 後天定位盤, counted by
  year, by 節月 and by day [wikipedia-ja-kyusei].
- **六曜**, 先勝 to 赤口, keyed to the 旧暦 date; widespread from the end of
  the Edo period [ndl-koyomi-rokuyo]. 大安 is good for weddings and 友引 is
  a day funerals are avoided [ndl-koyomi-rokuyo], and funeral businesses
  and crematoria are sometimes closed on it [wikipedia-ja-rokuyo].
- **七曜**, the Sun, the Moon and the five planets that name the days of
  the week, brought to Japan with the 宿曜経 and used with the 二十七宿 for
  divination before they were a week [nao-rekiwiki-youbi-namae].
- **納音**, the sixty 干支 in thirty pairs, each given a phase and a
  qualifying name, 甲子・乙丑 海中金 to 壬戌・癸亥 大海水; a person's 納音 is
  their birth year's, and fortune-telling reads it [wikipedia-ja-nacchin,
  kotobank-nacchin]. The per-person 五墓日 turn on it.
- **臘日**, one of the 選日: the Chinese year-end sacrifice to the gods and
  the ancestors together, which did not reach Japan as a custom and
  survives as a note whose day is reckoned several ways and which many
  almanacs leave out [wikipedia-ja-rounichi].

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

**納音** pairs the sixty in order: position *n* of the sixty, 甲子 being 0,
has the 納音 ⌊*n*/2⌋, and the phase is the last character of its name
(海中**金**) [wikipedia-ja-nacchin, sanming-tonghui-nayin]. It is not the
stem's phase: 甲 is wood and 甲子 is metal.

**Two notes are given per person.** 五墓日 is one day of the sixty for each
納音 phase: the day whose stem has that phase and whose branch is one of
the earth branches 丑辰未戌 [wikipedia-ja-rekichu-gedan, koyomil-gomunichi].
The 三箇の悪日 are one 節月 per person, the one whose branch is the birth
year's; a 巳-year person keeps 大禍 on 申, 狼藉 on 酉 and 滅門 on 寅 days
in 巳月 [wikipedia-ja-rekichu-gedan, ndl-koyomi-gedan]. Many commercial
almanacs print both for everyone [koyomi8-rekichu-3].

**臘日** is reckoned six ways in the sources read: the second 辰 day
after 小寒, the 辰 day nearest 大寒, the first 戌 day after 大寒
[wikipedia-ja-rounichi, koyomil-rounichi, jpnculture-rounichi], the
ninth of the twelfth lunar month [koyomil-rounichi, jpnculture-rounichi],
「丑節9日」, the ninth day of the 節月 丑月, which 小寒 opens
[wikipedia-ja-rounichi], and, in Qin and Han China, the third 戌 day
after 冬至: 「腊，冬至后三戌腊祭百神」 [wikipedia-zh-laba]. 辰 and 戌 are earth branches, and earth
overcomes the water of winter [wikipedia-ja-rounichi]. こよみる calls
the 辰 nearest 大寒 the current mainstream, and the 神社暦's
[koyomil-rounichi]; 西野神社 prints it [nishinojinja-senjitsu].

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

10. **納音.** 甲子 is position 0, so the 納音 is the first, **海中金**.

`day_notes` returns exactly this: 建, 虚, the Sun, 天恩日 and 天赦日 in the
lower register, 一粒万倍日 and 甲子 among the 選日, 一白 for the day and 赤口;
`hc_almanac_day` writes the 納音 beside the 干支.

### A worked example: 臘日 in the winter of 2026

1 January 2024 was 甲子, branch 子; 1 January 2026 is 731 days later, and
731 mod 12 = 11, so its branch is 亥, and 辰, five branches on, falls on
6, 18 and 30 January; 戌, eleven on, on 12 and 24 January.

1. **小寒** fell on 5 January 2026, a 卯 day, not a 辰 one, so the first
   辰 after it is 6 January and the **second 辰 after 小寒** is
   **18 January**.
2. **大寒** fell on 20 January; the 辰 days round it are 18 January, two
   days before, and 30 January, ten after: the **辰 nearest 大寒** is
   **18 January** too.
3. The **first 戌 after 大寒** is **24 January**.
4. The twelfth lunar month began on 19 January, so its **ninth** is
   **27 January**.
5. **丑節9日** is **13 January** if 小寒's own day, 5 January, is the first
   of 丑月, and **14 January** if the first is the day after.

こよみる prints exactly the three days of steps 1–4 for 2026
[koyomil-rounichi]; it does not give 丑節9日. No term day of 2026 bears
its rule's sign and 大寒 is not six days from a 辰 day each side, so each
rule of steps 1–3 answers the same day by both of its readings.

### A worked example: 五墓日 for someone born in 1925

1925 is 乙丑, position 1, so its 納音 is 海中金 and its phase metal. By the
publishers' reading, metal's 五墓日 is 辛未, position 7. 1 January 2025
was 庚午, position 6, so 2 January 2025 was 辛未, and the day comes round
every sixty days: 3 March, 2 May, 1 July, 30 August, 29 October and
28 December — the seven days こよみる lists for a metal person in 2025
[koyomil-gomunichi].

## What is carried

**Twenty-one lower-register notes** in `LowerRegister::ALL` and **fifteen
選日** in `SelectedDay::ALL`, each a name, a gloss and an `AlmanacRule`
value; one function, `rule_applies`, evaluates every rule. The **十二直**,
the **二十八宿** and **二十七宿**, the **three 九星** with their attributions,
the **七曜** over `hc_calendar::Weekday`, **六曜**, and 十五夜 and 十三夜 in
`moon_viewing`. `day_notes` assembles a day's page.

Every 節月-keyed note takes a `Meridian`, because Tokyo and Beijing put the
same term on different days several times a century. The 旧暦-keyed ones —
六曜, 不成就日, the 旧暦 凶会日 and the 二十七宿 — read, through
`hc_almanac::lunisolar`, the calendar of `hc-calendars-lunar` the meridian
names: the Chinese one at `Meridian::CHINA`, the Japanese 旧暦 at
`Meridian::JAPAN` and every other offset; see
[zassetsu-and-rokuyo.md](zassetsu-and-rokuyo.md).

The **納音** in `nayin`: `Nayin`, its pair, its phase, and its names in
Japanese, in kana, in Hepburn and as 『三命通會』 heads them, and the day's
納音 as a line of `hc_almanac_day`. The **per-mansion lists** in
`mansion_undertakings`: 歳事暦's and うまずたゆまず's, each an
`UndertakingList` of 大吉, 吉, 凶 and 大凶 items and a remark for each of
the 28, in Japanese as printed, with no function for "the" list. **臘日**
in `rounichi`, one `RounichiRule` per reading of each reckoning, none a
default.

**Where publishers differ**, each reading is named in the module
documentation with its source. These are registered as readings of their
own, because a source states each:

| Note | Readings | Carried as |
| --- | --- | --- |
| 凶会日, which month | 節月 (歳事暦, うまずたゆまず, Japanese Wikipedia's table); 旧暦 month (こよみる, Japanese Wikipedia's prose) | `LowerRegister::KUENICHI` is the 節月 reading and is what a day's register holds; `lower_register::KUENICHI_BY_LUNISOLAR_MONTH` is the 旧暦 reading, a named rule outside the register so that a page does not print the note twice |
| 九星, a solstice on 癸巳 | The switch is the preceding 甲子 (こよみのページ); the following 甲子 (the base of Japanese Wikipedia's 閏 list) | `SwitchReading::MIZUNOTO_MI_BACK`, used by `day_star`, and `SwitchReading::MIZUNOTO_MI_FORWARD`; `day_star_by` takes either |
| 五墓日, which days | 乙丑, 丙戌, 戊辰, 辛未, 壬辰 (Japanese Wikipedia, こよみる, 歳事暦); 乙未 and 辛丑 for wood and metal (精選版日本国語大辞典); 乙未, 丙辰, 戊辰, 辛丑, 壬辰 (the National Diet Library) | `GraveDays::WIKIPEDIA`, which `LowerRegister::GOMUNICHI` holds; `GraveDays::NIKKOKU`; `GraveDays::NDL` |
| 五墓日, for whom | Everyone, as こよみのページ computes it; only a person whose birth-year 納音 has the day's phase (Japanese Wikipedia, 精選版日本国語大辞典, 歳事暦, こよみる) | `GraveDays::rule` for everyone, which the register holds; `GraveDays::applies_to_person` by a birth year, for the two readings that give one day to each phase — the Library's does not |
| 三箇の悪日 | Every row, for everyone, as many commercial almanacs print it; one 節月 per person, the one whose branch is their birth year's (the National Diet Library and every other table read) | the three `LowerRegister` entries for everyone; `lower_register::three_evil_day_for` by a birth year |
| 臘日 | Six reckonings, listed above | `RounichiRule::SECOND_DRAGON_AFTER_MINOR_COLD`, `SECOND_DRAGON_FROM_MINOR_COLD`, `DRAGON_NEAREST_MAJOR_COLD_EARLIER`, `DRAGON_NEAREST_MAJOR_COLD_LATER`, `FIRST_DOG_AFTER_MAJOR_COLD`, `FIRST_DOG_FROM_MAJOR_COLD`, `LUNAR_TWELFTH_NINTH`, `OX_MONTH_NINTH_FROM_MINOR_COLD`, `OX_MONTH_NINTH_AFTER_MINOR_COLD`, `THIRD_DOG_AFTER_WINTER_SOLSTICE`, `THIRD_DOG_FROM_WINTER_SOLSTICE` |
| 臘日, "after" a term | The term's own day not counted; counted, so that a term day bearing the sign is the first | the `-after-` rule and the `-from-` rule of each of the second-辰, first-戌 and third-戌 reckonings |
| 臘日, the nearest 辰 | Of two 辰 days six days either side of 大寒, the earlier; the later | `DRAGON_NEAREST_MAJOR_COLD_EARLIER`, `DRAGON_NEAREST_MAJOR_COLD_LATER` |
| 臘日, 「丑節9日」 | 小寒's own day the first of 丑月; the day after it the first | `OX_MONTH_NINTH_FROM_MINOR_COLD`, `OX_MONTH_NINTH_AFTER_MINOR_COLD` |
| The undertakings of each mansion | 歳事暦; うまずたゆまず | `UndertakingList::SAIJIGOYOMI`, `UndertakingList::LINDERABELL` |

The others are named and not yet registered, each for the reason its row
gives:

| Note | Carried | Named, not yet carried |
| --- | --- | --- |
| 大明日 | The 25-day list of Japanese Wikipedia, こよみる and 歳事暦 | The 21-day list (Japanese Wikipedia, こよみる) and the 19-day list (こよみる), because no printed date tests them |
| The undertakings of each mansion | 歳事暦's and うまずたゆまず's lists | 神仏.ネット's, because its entries are prose sentences under that site's copyright, not lists, and a list of undertakings drawn from them would be this library's reading of them, not the site's |
| 凶会日, which table | The 貞享暦 table, seventy days | The 宣明暦 table, with twelve more, because no printed date tests it |
| 一粒万倍日 | The union of the National Diet Library's two methods, which it says are now used together | Either method alone, because no printed date tests it |
| 九星 閏 | Last sixty days of a 240-day period, reversing at the 甲午 | A 閏 wherever a 甲午 falls within a day of a solstice, because Japanese Wikipedia says it needs adjustments it does not describe |

**Not yet carried at all**: the 神吉日 suppression rule, because no source
read states it; and the mansion the Moon is actually in, because the
mansions' boundaries depend on a star catalogue, and none is read or
carried. The
crate README lists these gaps.

**Where a 臘日 rule's wording admits two readings**, each is a rule:
a rule counting after 小寒, 大寒 or 冬至 has an `-after-` rule, which does
not count the term's own day, and a `-from-` rule, which counts it when it
bears the sign; the nearest-辰 rule has an `-earlier` and a `-later` rule
for a winter whose 辰 days lie six days either side of 大寒; and
「丑節9日」 has a rule that counts 小寒's own day as the first of 丑月 and
one that counts from the day after. At the Japanese meridian the two
readings differ in 15, 18, 18 and 15 winters of 1901–2100 for the
second-辰, nearest-辰, first-戌 and third-戌 rules, and agree in every
other; the two readings of 丑節9日 are always a day apart. The lunar rule
answers `None` where no ordinary twelfth month lies next to 大寒, which
no winter of 1901–2100 does. The
Han rule reads the astronomical 冬至; Qin and Han kept a calendar of their
own, and the one dated 臘 the source quotes, a 戊戌 in the twelfth month
of 秦二世元年 [wikipedia-zh-laba], is not the day either reading of the
rule gives for that winter with the true solstice, 24 January 209 BCE (proleptic
Gregorian), a 丙戌: a 戊戌 is the next 戌 day after it. The rule is
carried as the source states it; no 臘 of those centuries is answered as
the one kept.

**At the boundaries.** `hc_almanac_day` writes a day's page, one note a
line, from `day_notes`: the sexagenary day and its 納音, 十二直, 二十八宿 and 二十七宿,
the three 九星, 六曜, and every lower-register note, 選日 and combination
that falls, each with the Japanese name, the Hepburn reading, the verdict
and, for the lower register, whether 受死日 or 十死日 suppresses it in
print. The names in a locale are `hc_i18n::almanac`'s: Japanese, which the
names were compared with on 2026-09-27 against the 暦Wiki pages 十二直 and
星宿 [nao-rekiwiki-junichoku, nao-rekiwiki-28shuku] and the National Diet
Library's 六曜, 下段 and その他 [ndl-koyomi-rokuyo, ndl-koyomi-gedan,
ndl-koyomi-sonota] — which agree but for 狼藉日, written 狼籍日 on the
Library's page — and English, which writes the notes in their
romanisation, as English Wikipedia's "Rokuyō" does [wikipedia-rokuyo],
and the mansions by the asterisms' English names [wikipedia-twenty-eight-mansions];
and, for the 納音 alone, Chinese, as 『三命通會』 heads them in
Wikisource's simplified transcription [sanming-tonghui-nayin]. No other
language's names were read, so none is carried. The worked
example's day is the export's anchor too: arachne.jp's 2025 calendar
prints 赤口, 一粒万倍 and 天赦日 against 21 December 2025
[arachne-taian-2025-12], and マイナビニュース names 甲子 and 天恩日 on it as
well [mynavi-2025-12-21], which is the day's page as `hc_almanac_day`
writes it. 七曜 is not written: it is the weekday, which every locale's
data already names.

## Accuracy

The rules are exact arithmetic on the day count, the 節月 and the 旧暦 date;
what can go wrong is a transcribed table, the day a solar term is put on,
and the 旧暦 derivation.

**Tables against independent statements.**

- The 納音: Japanese Wikipedia's thirty rows and 『三命通會』's thirty
  section headings name the same pairs with the same phases
  (`the_pairs_are_the_tables`,
  `the_phase_is_the_last_character_in_both_namings`), and こよみる's
  table of birth years gives the phase of fourteen years from 1921 to
  1998 as the crate does (`the_published_birth_years_have_their_phases`).
- The per-mansion lists: 歳事暦's and うまずたゆまず's agree mansion by
  mansion but for order, spelling, a particle or a comma, and 觜宿, which
  歳事暦 alone says to avoid for 衣類の着初め
  (`the_two_lists_differ_only_where_the_documentation_says`). Both say
  that 鬼宿 is best for everything but marriage and give 牛宿 nothing to
  avoid, as `Mansion::undisputed_note` has it. They are one witness, not
  two; no printed almanac was compared.

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

- 五墓日 by 納音: every day of 2025 こよみる lists for each of the five
  phases, 31 in all, with a birth year of each from its own table, and no
  other day (`the_published_2025_grave_days_of_each_phase_match`).
- 臘日: こよみる's candidates for 2024 to 2027 by its four reckonings,
  sixteen days, among them the one day of 2025 on which the first two
  agree, each by both readings of its rule
  (`the_published_candidates_of_2024_to_2027_match`). No source read
  dates a winter in which the two readings of a rule part, so those
  winters are checked against the wording alone: the two readings of
  "after" part only where the term day bears the sign, and the counted
  reading then answers the term day
  (`the_two_readings_of_after_differ_only_on_a_term_day_of_the_sign`);
  the two nearest-辰 readings part only on a tie
  (`the_nearest_dragon_readings_differ_only_on_a_tie`). 「丑節9日」 and
  the Han reckoning have no dated modern example; the 丑節9日 of 2026 is
  worked out from 小寒 (`the_ninth_of_the_ox_month_is_counted_both_ways`).

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

**The 旧暦** is the 天保暦's rules continued, as
[zassetsu-and-rokuyo.md](zassetsu-and-rokuyo.md) describes, with 閏11月 in
2033–34. The plain 中気 rule applied month by month differs from it on 89
days of 2024–2033, all from 25 August to 21 November 2033; that document
lists the runs over 1900–2100 in which the two differ.

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
| [koyomil-gomunichi] | 五墓日 by 納音 phase, the birth years of each phase from 1921, and the 2025 dates of each | Yes, 2026-09-26 and 2026-09-29 |
| [koyomil-taikanichi] | The three 悪日 by birth year and for everyone | Yes, 2026-09-26 |
| [saijigoyomi-gedan] | The 貞享暦 凶会日 headed 節切り; 五墓日 by 納音; the three 悪日; the 25-day 大明日 | Yes, 2026-09-26 |
| [linderabell-kuenichi] | The 貞享暦 凶会日 「節切りの月毎」 | Yes, 2026-09-26 |
| [kotobank-gomunichi] | 精選版日本国語大辞典's 五墓日 | Yes, 2026-09-26 |
| [kotobank-kuenichi] | 精選版日本国語大辞典's 凶会日, with 旧暦正月 庚戌・辛卯・甲寅 | Yes, 2026-09-26 |
| [nao-rekiwiki-junichoku], [nao-rekiwiki-28shuku], [ndl-koyomi-rokuyo], [ndl-koyomi-gedan], [ndl-koyomi-sonota] | Again, for `hc_i18n::almanac`: the Japanese names of the 十二直, the 28 mansions, 六曜, sixteen 下段 notes and the 選日 | Yes, 2026-09-27 |
| [wikipedia-rokuyo] | English writing the six days in romanisation, Senshō to Shakkō | Yes, 2026-09-27 |
| [wikipedia-twenty-eight-mansions] | The asterisms' English names, Horn to Chariot | Yes, 2026-09-27 |
| [arachne-taian-2025-12] | 赤口, 一粒万倍 and 天赦日 on 21 December 2025 | Yes, 2026-09-27 |
| [mynavi-2025-12-21] | 一粒万倍日, 天赦日, 甲子 and 天恩日 on 21 December 2025 | Yes, 2026-09-27 |
| [wikipedia-ja-nacchin] | The thirty 納音, their readings and pairs | Yes, 2026-09-29 |
| [sanming-tonghui-nayin] | 『三命通會』 卷一's thirty section headings, as Wikisource transcribes them | Yes, 2026-09-29 |
| [kotobank-nacchin] | 精選版日本国語大辞典's 納音: 甲子・乙丑 海中金, 丙寅・丁卯 爐中火 | Yes, 2026-09-29 |
| [wikipedia-ja-rekichu-gedan], [ndl-koyomi-gedan], [kotobank-gomunichi] | Again: 五墓日 by phase (the Library's without one); the three 悪日 by birth year and the 巳-year example | Yes, 2026-09-29 |
| [wikipedia-ja-rounichi] | 臘日: the 臘祭, the four Japanese reckonings, 「丑節9日」 with no word on which day of 丑月 is the first, the earth branches | Yes, 2026-09-29 |
| [koyomil-rounichi] | 臘日: four reckonings with the lunar 12月9日, the 神社暦's, 臘八 not the 選日, the candidates of 2024–2027 | Yes, 2026-09-29 |
| [jpnculture-rounichi] | 臘日: the same four reckonings | Yes, 2026-09-29 |
| [nishinojinja-senjitsu] | 臘日 as 「大寒に近い辰の日」 | Yes, 2026-09-29 |
| [wikipedia-zh-laba] | The Qin and Han 臘 on the third 戌 after 冬至, 『説文解字』 quoted; 秦二世元年十二月戊戌 | Yes, 2026-09-29; the 『説文解字』 itself not read |
| [saijigoyomi-28shuku] | 歳事暦's list of each mansion's undertakings | Yes, 2026-09-29 |
| [linderabell-28shuku] | うまずたゆまず's list | Yes, 2026-09-29 |
| [shintobukkyo-28shuku] | 神仏.ネット's entries, as prose sentences; not yet carried, for the reason under What is carried | Yes, 2026-09-29 |
| [okada-akune1993] | The rule tables at the root of Japanese Wikipedia's and こよみのページ's | Not read; its record from CiNii Books, 2026-09-26 |
| [okada-kyureki-dokuhon] | Named by こよみのページ as a basis of its rules | Not read |

The published date lists the tests cite from 吉日カレンダー, zired, arachne.jp,
JAL SKYWARD+, こよみる's daily pages, 暦注下段ナビ, KOYOMI NOTE and 開運道 (which
states it follows 天象学会『萬年暦』, not read) were not re-read for this
document.

## Code

`crates/hc-almanac/src/`: `context.rs` (the 節月 and the shared day
context), `rules.rs` (`AlmanacRule` and `rule_applies`),
`lower_register.rs` (with `GraveDays` and `three_evil_day_for`),
`selected_days.rs`, `twelve_directs.rs`, `mansions.rs`,
`mansion_undertakings.rs`, `nine_stars.rs`, `rokuyo.rs`,
`seven_luminaries.rs`, `nayin.rs`, `rounichi.rs` and `day_notes.rs`.

Anchors:
`the_ndl_evil_gathering_table_is_the_jokyo_table_and_eleven_more`,
`the_lunisolar_reading_of_the_evil_gathering_matches_the_published_2025_dates`,
`the_published_days_of_heavens_pardon_match`,
`a_solar_term_that_arrives_at_ten_at_night_still_opens_its_month_that_day`,
`the_published_2026_days_of_going_out_and_perishing_match`,
`the_published_2024_lower_register_entries_match`,
`the_published_2025_grave_days_of_each_phase_match`,
`the_three_grave_day_readings_differ_where_their_sources_do`,
`the_three_evil_days_by_birth_year_keep_the_birth_years_month` (lower
register);
`the_pairs_are_the_tables`, `the_phase_is_the_last_character_in_both_namings`,
`the_published_birth_years_have_their_phases` (納音);
`the_published_candidates_of_2024_to_2027_match`,
`the_two_readings_of_after_differ_only_on_a_term_day_of_the_sign`,
`the_nearest_dragon_readings_differ_only_on_a_tie`,
`the_ninth_of_the_ox_month_is_counted_both_ways`,
`the_lunar_rule_is_the_ninth_of_the_twelfth_month` (臘日);
`the_two_lists_differ_only_where_the_documentation_says`,
`the_lists_agree_with_the_undisputed_notes` (the per-mansion lists);
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

The lines are `crates/hyper-calendar/src/almanac_lines.rs`'s
`almanac_day_lines`, anchored by
`the_twenty_first_of_december_2025_is_written_as_the_almanacs_print_it` and
`every_line_is_named_and_every_cycle_written`; the names are
`crates/hc-i18n/src/almanac.rs`, held to `hc-almanac` by
`the_almanac_vocabulary_names_what_hc_almanac_computes` in
`crates/hyper-calendar/tests/vocabulary.rs`.
