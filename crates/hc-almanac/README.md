# hc-almanac

The divinatory annotations of the Japanese and Chinese almanac: 暦注
(almanac notes) and 選日 (selected days).

`hc-seasons` owns the astronomical subdivisions of the year: the 24 solar
terms, the 72 pentads, the 雑節. This crate owns what a printed almanac lays
*on top of* them. A page of the 神宮館 or 高島 almanac gives, in order:

1. the date;
2. the 干支;
3. the 十二直;
4. the 二十八宿;
5. the three 九星;
6. a paragraph of 暦注下段 and 選日.

Every one of those is a **rule over a cycle**, not an astronomical event.
So the crate is data plus one evaluator, not a function per annotation.

```rust
use hc_almanac::{LowerRegister, Meridian, Rd, day_notes::day_notes};

let notes = day_notes(Rd(738_886), Meridian::JAPAN); // 1 January 2024
assert_eq!(notes.sexagenary().index(), 0);           // 甲子
assert_eq!(notes.twelve_direct().japanese_name(), "建");
assert_eq!(notes.mansion().japanese_name(), "畢");
assert!(notes.lower_register().contains(LowerRegister::TENSHANICHI));
```

## Coverage

| Module | Annotations |
|---|---|
| `mansions` | 二十八宿 (28-day cycle, 四象 grouping, 吉凶, 和名) and the 二十七宿 of 宿曜道 |
| `mansion_undertakings` | what each mansion's day favours and forbids, list by list: 歳事暦's and うまずたゆまず's, as printed |
| `nine_stars` | 九星: 年家, 月家, 日家, with 五行, colour, direction and trigram |
| `twelve_directs` | 十二直 建除満平定執破危成納開閉 |
| `lower_register` | 21 of the 暦注下段; the three readings of 五墓日, each for everyone or by a birth year's 納音; the 三箇の悪日 by a birth year |
| `nayin` | 納音, the thirty sounds, 海中金 to 大海水, with the phase of each |
| `selected_days` | 15 選日 |
| `seven_luminaries` | 七曜 — the planetary association over `hc_calendar::Weekday` |
| `rokuyo` | 六曜 先勝 友引 先負 仏滅 大安 赤口 |
| `moon_viewing` | 十五夜 and 十三夜, the fifteenth of the eighth and the thirteenth of the ninth lunisolar month |
| `lunisolar` | the Japanese 旧暦 date the Moon-keyed annotations read, from `hc-calendars-lunar` |
| `day_notes` | every annotation for one day, from one shared context |
| `lucky_direction` | 恵方, the year's direction by its stem: 甲, 庚, 丙 or 壬, with the branches and the azimuth |
| `direction_deities` | the 八将神 by the year's branch, 金神 by its stem, 大金神 and 姫金神 by its branch; the 遊行 of 大将軍 and 金神, one `WanderingRule` per source's reading, and 金神's 間日 |
| `rounichi` | 臘日, one `RounichiRule` for each reading of six reckonings |
| `nine_periods` | 三元九運, the twenty-year periods from 上元一運 in 1864, turning at 立春 |
| `days_without_son` | 손 없는 날, the Korean lunar days ending in 9 and 0, on `dangi` |
| `vietnamese_days` | Ngày Tam Nương (3, 7, 13, 18, 22, 27) and ngày Nguyệt Kỵ (5, 14, 23) of the lunar month, on `vietnamese` |
| `first_month_counts` | 几龙治水, 几牛耕田, 几日得辛 and 几人分饼: the day of 正月 of the first 辰, 丑, 辛 and 丙 day, on `chinese` |

**暦注下段:** 大明日, 天恩日, 母倉日, 月徳日, 神吉日, 鬼宿日, 天赦日, 大禍日,
狼藉日, 滅門日, 帰忌日, 血忌日, 重日, 復日, 往亡日, 凶会日, 十死日, 受死日 (黒日),
天火日, 地火日, 五墓日.

**選日:** 一粒万倍日, 三隣亡, 不成就日, 八専, 八専の間日, 十方暮, 天一天上, 庚申,
甲子, 己巳, 寅の日, 巳の日, 大犯土, 小犯土, 犯土の間日. Plus the modern
"combination" days (天赦日＋一粒万倍日 and friends) in `day_notes`, clearly
labelled as commerce rather than tradition.

## Design

One `DayContext` holds the facts a rule can ask about — the sexagenary day, the
節月, the lunisolar date. One `AlmanacRule` enumerates the shapes a rule can take.
One `rule_applies` evaluates them. That is the same data/algorithm split
`hc_seasons::ZassetsuRule` makes for the 雑節, and it is what keeps thirty-six
annotations from becoming thirty-six chances to get the 節月 boundary wrong.

`rule_applies` returns `Option<bool>`: `None` means "this crate does not know",
which is a different answer from `Some(false)`, "the almanac says no".

## Provenance

**None of this is official.** The National Astronomical Observatory of Japan
publishes the solar terms, the 雑節 and the public holidays in the 暦要項 and
nothing else in this crate; its 暦Wiki covers 暦注 in general, 十二直, 二十八宿,
the 節月 and the 節切り/月切り vocabulary but publishes no 暦注下段 or 選日 rule
at all. The 中段
and 下段 were struck from the official calendar at the Meiji reform of 1873 as
superstition, and survived in commercial almanacs and, for a while, in illegally
printed おばけ暦.

Sources used, in rough order of weight. The system document
[`docs/systems/japanese-almanac-notes.md`](../../docs/systems/japanese-almanac-notes.md)
gives each with what it was used for and the date it was read.

* **国立天文台 暦計算室 暦Wiki** — 十二直 (the rule and the 「おどる」 repeat),
  二十八宿 (the continuous-counter statement and the 二十七宿 reset table), 節月,
  暦注. <https://eco.mtk.nao.ac.jp/koyomi/wiki/>
* **国立国会図書館「日本の暦」** — the 三箇の悪日 table, 往亡日, 天赦日, 帰忌日,
  母倉日, 月徳日, the two 一粒万倍日 methods, 八専, 十方暮, 天一天上, its own
  凶会日 and 五墓日 lists, and the glosses. <https://www.ndl.go.jp/koyomi/>
* **精選版日本国語大辞典 / デジタル大辞泉 via コトバンク** — the mansion 和名,
  the 鬼宿日 marriage exception, the 五墓日 variant.
* **岡田芳朗・阿久根末忠 (編著)『現代こよみ読み解き事典』(柏書房, 1993)** — the
  rule tables. Not read: reached only through Japanese Wikipedia 暦注下段 and
  こよみのページ, which both name it.
* **Publishers' own statements of which reading they print** — こよみる, 歳事暦,
  うまずたゆまず.
* **Published almanac date lists** — こよみる, 暦職人, 吉日カレンダー, arachne.jp,
  暦注下段ナビ, 開運道 and KOYOMI NOTE, used as *checks* and cited in the tests.

A warning worth repeating: most of the well-known Japanese 暦注 websites descend
from the same 岡田芳朗 lineage, so four agreeing pages are often one witness. The
tests therefore anchor on printed date lists as well as on rule statements,
because a citation cannot catch a transcription error and a published calendar
can. Where a table could be read two ways, the crate follows the lists:

* 一粒万倍日, 亥月 row: **酉・戌**, one branch from each of the two methods the
  National Diet Library says are now used together.
* 大明日: the **25**-entry list, which includes 己巳.
* 復日: the 節月 → stem mapping is a **cross product** (正月 takes 甲 *and* 庚),
  not a pairwise one.

## Regional and historical variation

| Where | What |
|---|---|
| 二十八宿 vs 二十七宿 | Japan used the 27 of 宿曜道 until 渋川春海's 貞享 reform of 1685 replaced them with the Chinese 28. Both are printed today. 牛宿 is the one the 27 drops. |
| Mansion 吉凶 | Publishers disagree on roughly a third of the entries. Only 鬼宿 and 牛宿 are agreed by every source; `Mansion::fortune_is_undisputed` says which. |
| 三隣亡 | The day-selection rule is unchanged from the Edo period; what flipped is the *meaning*. It was 三輪宝, 「屋立てよし」 — auspicious for building — until a copyist's よ/あ slip inverted it. |
| 五墓日 | Three 干支 sets in print (乙丑・辛未 in Wikipedia and the publishers, 乙未・辛丑 in 精選版日本国語大辞典, 乙未・丙辰・辛丑 in the National Diet Library) and two scopes (everyone, or only those whose birth-year 納音 matches). The register holds the publishers' set for everyone; each set is a `GraveDays` reading, and `GraveDays::applies_to_person` gives the per-person form, which the Library's list, giving no day to a phase, does not have. |
| 三箇の悪日 | Given by birth year in every table read; applied to everyone by many commercial almanacs, and by the register. `three_evil_day_for` gives the birth-year form. |
| 臘日 | Six reckonings: the second 辰 after 小寒, the 辰 nearest 大寒, the first 戌 after 大寒, the lunar 12月9日, Japanese Wikipedia's 「丑節9日」, and the Han third 戌 after 冬至. Where the wording admits two readings — whether "after" a term counts the term's own day, which of two equally near 辰 days is meant, whether 小寒's own day is the first of 丑月 — each reading is a `RounichiRule` of its own; none is a default. |
| Mansion undertakings | 歳事暦 and うまずたゆまず print the same lists but for 觜宿; 神仏.ネット writes sentences that differ in substance, and is not yet carried (see below). |
| 大明日 | 25-, 21- and 19-entry lists are published. The 25 is implemented and a published date settles it. |
| 遊行 of 大将軍 and 金神 | いい日本再発見's 大将軍 runs every sixty days; Japanese Wikipedia's 金神 has year-round runs and seasonal ones bounded by 立 terms and 土用, and does not say whether a seasonal run keeps days past its season, so that is two rules. Japanese Wikipedia's 大将軍 and 古文書ネット's seasonal rows are not yet carried (see below). |
| 凶会日 | The 宣明暦 table and the 貞享暦 table, each read by 節月 or by 旧暦 month. The 貞享 table by 節月 is `LowerRegister::KUENICHI`; by 旧暦 month it is `lower_register::KUENICHI_BY_LUNISOLAR_MONTH`, checked against こよみる's 2025 dates. |
| 日家九星 | A solstice on 癸巳 switches on the preceding 甲子 in one school and the following in another; both are `nine_stars::SwitchReading` entries. |
| 七曜 names | The 七曜 weekday names survive as living usage only in Japanese and Korean. Mainland China replaced them with 星期 (coined 1905); Taiwan used them under Japanese rule and now uses 星期 too. Both Chinese columns are given. |
| Meridian | Every 節月-keyed and Moon-keyed annotation takes a `Meridian`, because Tokyo and Beijing put the same solar-term instant on different days several times a century. |

## Accuracy

| Class | Annotations | Exactness |
|---|---|---|
| Pure day count | all 干支 rules, 七曜, 二十八宿, 納音, 恵方, 八将神, 金神, the 大将軍 遊行 | exact for ever |
| Year count from 立春 | 三元九運 | exact, the 立春 day as the 九星 year has it |
| Korean lunisolar | 손 없는 날 | `hc-calendars-lunar`'s `dangi`, 1645–2150 |
| 節月-keyed | 十二直, 九星, most of 下段 and 選日, 臘日 but the lunar rule, the 金神 遊行 and 間日 by season and 土用 | `hc-astro`'s VSOP87 solar series, good to about 1″ |
| Lunisolar | 六曜, 不成就日, 二十七宿, 十五夜, 十三夜, 臘日 on the lunar 12月9日 | the 天保暦's rules continued, `hc-calendars-lunar`'s `japanese_tenpo::UNBOUNDED_PARAMETERS` |

A term instant within about a minute of local midnight can still be assigned
the wrong *day*, which moves a 節月 boundary and with it every annotation
keyed to one. The sharpest real case the crate tests is 立秋 2025, which fell at 22:52 JST on 7
August: the whole of that day is 申月, which is what makes it the autumn 天赦日.

The lunisolar class reads the calendar of `hc-calendars-lunar` the
meridian names, `lunisolar::Reckoning`: at `Meridian::CHINA` and
`CHINA_BEFORE_1929` the `chinese` calendar, and at every other meridian
the Japanese one, the Tenpō rules continued past their 1872 abolition,
which is what Japanese almanacs have keyed 六曜 to ever since. No
lunisolar calendar is kept at any other offset, so `Meridian::UNIVERSAL`,
`INDIA` or a longitude reads the Japanese calendar for these annotations
and the 節月 at the offset. In 2033–34, where no numbering satisfies the 天保暦 rule, it gives 閏11月,
the resolution the 日本カレンダー暦文化振興協会 recommended; some published
calendars print another. Every reading opens a `hc_core::memo::scope`, and
3,653 consecutive days take 21 to 26 ms inside one in a release build, and
about 45 µs a day outside one.

The annotations read the calendars, not the plain 中気 rule applied month
by month, which is cheaper and wrong where a month holds two 中気: at the
Japanese meridian that rule numbers other months than the calendar on 89 of
the 3,653 days of 2024–2033, all from 25 August to 21 November 2033, and on
268 days in five runs over 1900–2100, and at the Chinese meridian on the
same 89 and on 329 in seven runs;
`docs/systems/zassetsu-and-rokuyo.md` lists them and the published calendar
they were checked against.

## Documented gaps

Things this crate does not yet carry, each for want of a source that
states it, because the sources read contradict each other, or because
another crate carries it:

* **A third placement of the 日家九星 閏.** The 陽遁/陰遁 switch is the 甲子 day
  *nearest* each solstice, and a period is normally 180 days. About one period
  in twenty-three runs **240 days** instead and holds a 閏, which the crate
  places as こよみのページ and Japanese Wikipedia both describe it. Wikipedia
  records a further placement — wherever a 甲午 falls within a day of a
  solstice — and says that rule alone leaves places that need adjusting,
  without saying how, so it is not yet carried: no source read says what
  the adjustments are. `DayStarPeriod::is_leap_period` tells a caller when a
  day is in a period that holds a 閏.
* **神仏.ネット's list of each mansion's undertakings.** Its entries are
  prose sentences under that site's copyright, not lists, and a list of
  undertakings drawn from them would be this crate's reading of them, not
  the site's. `mansion_undertakings` carries
  歳事暦's and うまずたゆまず's printed lists, and no function answers for
  "the" tradition; `Mansion::undisputed_note` gives the two statements
  every source makes.
* **The 神吉日 suppression rule.** Edo almanacs printed fewer than the 33 in 60
  the rule gives, because a 神吉日 overlapping certain 凶日 was dropped. Which
  ones is not known: Japanese Wikipedia says 「その規則は完全には判明していない」.
  All 33 are emitted.
* **The 宣明暦 table of 凶会日**, because no printed date tests it. Sources
  contradict each other about 節切り and 月切り, and one contradicts itself,
  so the 貞享暦 table is carried both ways: `LowerRegister::KUENICHI` is the
  節月 reading and is what a day's lower register holds; the 旧暦月 reading
  is the named rule `lower_register::KUENICHI_BY_LUNISOLAR_MONTH`, evaluated
  with `rule_applies`, so that a page does not print 凶会日 twice.
* **The day a year's 恵方 takes over**, because the customs read do not
  settle it. `lucky_direction_of_year` takes the year's number;
  the customs read use the Gregorian year, 恵方参り on New Year's Day and
  the 恵方巻 of 節分 facing the year already begun. The other 方位神 are
  `direction_deities`', whose year turns at 立春.
* **The 遊行 of the 方位神 as Japanese Wikipedia's 大将軍 page and
  古文書ネット give them.** Japanese Wikipedia's 大将軍 runs fall in the
  four 土用, and a 土用 of seventeen to nineteen days holds a run's first
  干支 in about three years in ten; the page does not say what happens in
  the others. 古文書ネット's 大将軍 page does not bound its seasons, and
  lists 30 August–3 September 2026, 旧7月 and after 立秋, under 夏; its
  seasonal 金神 rows count lunar months without saying where the leap
  months go, and its autumn run opens on 「辛申」, which is no 干支. Where
  two of Japanese Wikipedia's 金神 runs overlap, the page does not say
  where 金神 is, and `direction_deities` answers `None`.
* **土公神 and 歳禄神**, because no table of them was read.
* **The 大三元 of 540 years**, whose epoch the source of 三元九運 does not give,
  and where *son* is on the days that are not 손 없는 날, for which no table
  was found.
* **三伏 (初伏・中伏・末伏)**, which `hc_seasons::san_fu` carries: a period
  counted from the summer solstice and 立秋, not a rule over a cycle.
* **The astronomical mansion**, because the mansions' boundaries are
  unequal, were revised over history and depend on the star catalogue,
  and no catalogue of them is read or carried. `mansion_of` is the almanac's 28-day *counter* and has nothing
  to do with where the Moon is; the sidereal month is 27.32 days, so the
  two lap each other in about 1,128 days, a little over three years.
  `hc_astro::lunar_longitude` gives the Moon's side of it.

## The two places the bugs live

Both have dedicated tests.

**The 十二直 reset.** The cycle is anchored to the 節月, not free-running: in 寅月
the 寅 day is 建. At a 節気 both the day branch and the month branch advance by
one, the increments cancel, and **the same direct is printed two days running**.
A skip is structurally impossible. Checked over a decade of consecutive days:
exactly 120 repeats, one per sectional term, and never a skip.

**The 九星 陽遁/陰遁 reversal.** The switch is the *nearest* 甲子, not the last
one — the December solstice of 2023 fell on the 22nd, the preceding 甲子 was fifty
days back and the nearest was 1 January 2024, and published almanacs print 一白
水星 against 1 January. Checked against five published switch days from 2024 to
2026 and against both sides of both reversals (2025-12-20/21 and 2026-06-18/19),
where the doubled star appears.

## Testing

240 unit tests, the catalogues' generated checks among them, and 5 doc
tests. The anchors are published almanac dates, cited in the test doc
comments: whole published years of 一粒万倍日, 三隣亡 and 往亡日; the 2024
鬼宿日 list; the 2024 and 2025 天赦日; the 八専, 十方暮 and 天一天上
windows for 2025; the 二十七宿 for September 2026; the 1685 epoch of the
mansion cycle, which the crate reproduces 339 years back with the right
weekday; every 2025 五墓日 こよみる lists for each 納音 phase; こよみる's
臘日 candidates of 2024–2027, by both readings of each rule; 古文書ネット's
八将神 of 2026 and 金神 of 2025; and the 大将軍 遊行 of 2026 as
いい日本再発見 and 古文書ネット list them. No source read dates a 金神 遊行
or 間日, so those rules are checked against the table on days of 2025 and
2026 worked out from it.
