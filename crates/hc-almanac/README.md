# hc-almanac

暦注 and 選日 — the divinatory annotations of the Japanese and Chinese almanac.

`hc-seasons` owns the astronomical subdivisions of the year: the 24 solar terms,
the 72 pentads, the 雑節. This crate owns what a printed almanac lays *on top of*
them. A page of the 神宮館 or 高島 almanac gives the date, then the 干支, then
十二直, then 二十八宿, then the three 九星, then a paragraph of 暦注下段 and 選日.
Every one of those is a **rule over a cycle**, not an astronomical event — so the
crate is data plus one evaluator, not a function per annotation.

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
| `nine_stars` | 九星: 年家, 月家, 日家, with 五行, colour, direction and trigram |
| `twelve_directs` | 十二直 建除満平定執破危成納開閉 |
| `lower_register` | 21 of the 暦注下段 |
| `selected_days` | 15 選日 |
| `seven_luminaries` | 七曜 — the planetary association over `hc_calendar::Weekday` |
| `rokuyo` | 六曜 — re-exported from `hc-seasons`, where it is implemented |
| `day_notes` | every annotation for one day, from one shared context |

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
| 五墓日 | Three 干支 sets in print (乙丑・辛未 in Wikipedia and the publishers, 乙未・辛丑 in 精選版日本国語大辞典, 乙未・丙辰・辛丑 in the National Diet Library) and two scopes (everyone, or only those whose birth-year 納音 matches). The crate uses the publishers' set, unconditionally, and names the others. |
| 三箇の悪日 | Given by birth year in every table read; applied to everyone by many commercial almanacs, and by this crate. |
| 大明日 | 25-, 21- and 19-entry lists are published. The 25 is implemented and a published date settles it. |
| 凶会日 | The 宣明暦 table and the 貞享暦 table, each read by 節月 or by 旧暦 month. The 貞享 table by 節月 is `LowerRegister::KUENICHI`; by 旧暦 month it is `lower_register::KUENICHI_BY_LUNISOLAR_MONTH`, checked against こよみる's 2025 dates. |
| 日家九星 | A solstice on 癸巳 switches on the preceding 甲子 in one school and the following in another; both are `nine_stars::SwitchReading` entries. |
| 七曜 names | The 七曜 weekday names survive as living usage only in Japanese and Korean. Mainland China replaced them with 星期 (coined 1905); Taiwan used them under Japanese rule and now uses 星期 too. Both Chinese columns are given. |
| Meridian | Every 節月-keyed and Moon-keyed annotation takes a `Meridian`, because Tokyo and Beijing put the same solar-term instant on different days several times a century. |

## Accuracy

| Class | Annotations | Exactness |
|---|---|---|
| Pure day count | all 干支 rules, 七曜, 二十八宿 | exact for ever |
| 節月-keyed | 十二直, 九星, most of 下段 and 選日 | `hc-astro`'s VSOP87 solar series, good to about 1″ |
| Lunisolar | 六曜, 不成就日, 二十七宿 | `hc-seasons`' minimal 定気 derivation |

A term instant within about a minute of local midnight can still be assigned
the wrong *day*, which moves a 節月 boundary and with it every annotation
keyed to one. The sharpest real case the crate tests is 立秋 2025, which fell at 22:52 JST on 7
August: the whole of that day is 申月, which is what makes it the autumn 天赦日.

For the lunisolar class the divergence is measured rather than asserted, and
the measurement lives in `hc_seasons::lunisolar` beside both implementations.
Over the 3,653 days of 2024–2033, `hc-seasons`' minimal derivation and the
Japanese 旧暦 disagree on **89 days, 2.4%**, all in one contiguous run from 25
August to 21 November 2033. They differ about a whole month's numbering, not
about single days.

The reference matters. The right one is the unbounded Tenpō engine, which is
the Tenpō rules continued past their 1872 abolition, and is what Japanese
almanacs have keyed 六曜 to ever since. Japan computes the 旧暦 at 135°E and
China at 120°E, and that hour moves month boundaries on its own. Comparing the
derivation read at the Japanese meridian with `hc-calendars-lunar`'s Chinese
calendar therefore conflates a meridian difference with a method difference,
and gives 239 days rather than 89. Read at the Chinese meridian, the
derivation differs from the Chinese calendar on the same 89 days; the test in
`hc_almanac::context` guards that figure.

This crate uses the minimal derivation deliberately, so that 六曜, 不成就日 and
二十七宿 agree with one another and with the 六曜 a caller gets from
`hc-seasons`. Routing through the real calendar makes every annotation pay for
a new-moon search, which takes the seasons crate's own test suite from seconds
to over ten minutes, and routing only some of the annotations makes them
contradict each other. `hc-calendars-lunar` is
re-exported for callers who want the fuller article.

## Documented gaps

Things this crate deliberately does not do, rather than guessing:

* **A third placement of the 日家九星 閏.** The 陽遁/陰遁 switch is the 甲子 day
  *nearest* each solstice, and a period is normally 180 days. About one period
  in twenty-three runs **240 days** instead and holds a 閏, which the crate
  places as こよみのページ and Japanese Wikipedia both describe it. Wikipedia
  records a further placement — wherever a 甲午 falls within a day of a
  solstice — and says that rule alone leaves places that need adjusting,
  without saying how; it is not implemented. `DayStarPeriod::is_leap_period`
  tells a caller when a day is in a period that holds a 閏.
* **Per-mansion 吉凶 lists.** The 吉/凶 flag is shipped; the per-mansion lists of
  favoured and forbidden undertakings are not, because published tables diverge
  enough that picking one would be inventing a tradition. Only the two statements
  every source makes — 鬼宿 is best for everything but marriage, 牛宿 is
  auspicious in all things — are shipped, through `Mansion::undisputed_note`.
* **The 神吉日 suppression rule.** Edo almanacs printed fewer than the 33 in 60
  the rule gives, because a 神吉日 overlapping certain 凶日 was dropped. Which
  ones is not known: Japanese Wikipedia says 「その規則は完全には判明していない」.
  All 33 are emitted.
* **凶会日 in the lower register twice.** Sources contradict each other about 節切り
  and 月切り, and one contradicts itself. `LowerRegister::KUENICHI` is the 節月
  reading and is what a day's lower register holds; the 旧暦月 reading is the
  named rule `lower_register::KUENICHI_BY_LUNISOLAR_MONTH`, evaluated with
  `rule_applies`, so that a page does not print 凶会日 twice. The 宣明暦 table
  is not carried: no printed date tests it.
* **臘日.** At least four incompatible definitions are in print and many almanacs
  omit it. Not implemented.
* **三伏 (初伏・中伏・末伏).** A period counted from the summer solstice and 立秋
  rather than a rule over a cycle, so it is not an annotation here.
  `hc_seasons::san_fu` computes it.
* **納音.** Not modelled, so the per-person forms of 五墓日 and the 三箇の悪日
  cannot be filtered.
* **The astronomical mansion.** `mansion_of` is the almanac's 28-day *counter*
  and has nothing to do with where the Moon is; the sidereal month is 27.32 days,
  so the two lap each other in about 1,128 days, a little over three years.
  Finding the mansion the Moon is actually in means taking `hc_astro::lunar_longitude` and a star catalogue, and
  that is an `hc-astro` problem rather than a 暦注 one.

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

126 unit tests and 3 doc tests. The anchors are published almanac dates, cited in
the test doc comments: whole published years of 一粒万倍日, 三隣亡 and 往亡日;
the 2024 鬼宿日 list; the 2024 and 2025 天赦日; the 八専, 十方暮 and 天一天上
windows for 2025; the 二十七宿 for September 2026; and the 1685 epoch of the
mansion cycle, which the crate reproduces 339 years back with the right weekday.
