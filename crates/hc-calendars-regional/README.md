# hc-calendars-regional

Regional, cyclic and era calendars for `hyper-calendar`: Japanese imperial
eras, the Maya calendars, the two Aztec ones, the Balinese Pawukon, the
Javanese *pasaran*, the Burmese and Thai lunar calendars, and the sexagenary
cycle.

What they have in common is that **the day has a name before it has a
number**. A Maya day is *4 Ahau 8 Cumku*; a Balinese day is *Buda Kliwon
Dungulan*, a position in two of ten concurrent week cycles and one of thirty
*wuku*; a Japanese day
belongs to an era a government proclaimed. None of them is a count of years
from an epoch with months cut out of it, which is why none of them belongs in
`hc-calendars-solar` or `hc-calendars-lunar`.

| Identifier | What it is |
| --- | --- |
| `japanese` | Imperial era years (和暦); Gregorian from 1873, the five lunisolar calendars back to 862 before it |
| `japanese-northern`, `japanese-southern` | The same, with the Northern or the Southern Court's eras during the 南北朝 split |
| `japanese-proclaimed` | The same, with eras as proclaimed at the time rather than the retroactive official boundaries |
| `maya-longcount` | `baktun.katun.tun.uinal.kin`, in `DateFields::extra`, under the GMT correlation |
| `maya-longcount-gmt2` | The same under the GMT+2 correlation |
| `maya-tzolkin` | 13 numbers × 20 day-names = 260 days |
| `maya-haab` | 18 months of 20 days plus the 5-day Uayeb |
| `maya-round` | The 18 980-day Calendar Round |
| `aztec-tonalpohualli` | 260 days |
| `aztec-xiuhpohualli` | 365 days |
| `balinese-pawukon` | Thirty *wuku* and ten concurrent week cycles over 210 days |
| `javanese-pasaran` | The 5-day market week and the 35-day *wetonan* |
| `akan` | The Akan 6-day week and the 42-day *Adaduanan*, with the four *dabɔne* |
| `korean-regnal` | The Korean Empire's eras 建陽, 光武, 隆熙 on the Gregorian days of 1896–1910 |
| `chinese-regnal` | The Qing eras over the lunisolar calendar, 1645–1912; the Ming and Qing era table as data |
| `burmese` | The Burmese lunisolar calendar of the Myanmar Era: watat years, First Waso and the Nayon day, by the published arithmetic |
| `thai-lunar` | The Thai lunar calendar as Thailand publishes it: the adhikamāsa and adhikavāra years carried as data for 2535–2570 BE (1992–2027) |
| `sexagenary` | 干支 over years, months and days |

`register_all(&mut CalendarRegistry)` inserts every calendar in the table,
behind the `alloc` feature, exactly as `hc-calendars-solar` does;
[`docs/supported.md`](../../docs/supported.md) is the generated list.

## Cycles are not calendars

`hc_calendar::Calendar` demands a bijection between dates and fixed days, and
a cycle is by construction not one. Every cyclic date type here therefore
carries a **round number** — complete cycles elapsed since its epoch —
alongside the position within the cycle. The round lands in
`DateFields::year`; anything more year-like (the Haab's 365 days, the
Pawukon's 30 *wuku*) goes in `month` and `day`, and the concurrent cycles go
in `ExtraFields`.

## The Japanese calendar, and where it stops

`japanese` is the headline item and the one most implementations get wrong.
The CLDR identifier names a *year numbering*, not a structure, and the
structure underneath changed on a known day:

* **1873-01-01 onward** — proleptic Gregorian.
* **1844-02-18 to 1872-12-31** — the Tenpō lunisolar calendar, leap months
  and all, through `hc_calendars_lunar::japanese_tenpo`.
* **Before 1844-02-18** — refused.

明治5年12月2日 is 1872-12-31, the day after it is 明治6年1月1日 = 1873-01-01,
and 明治5年12月3日 does not exist. Running the Gregorian calendar backwards
through 明治 would invent it.

**Before 1844 the calendar in force is the one that answers.** Japan used
Kansei-reki (1798–1844), Hōryaku-reki (1755–1798), Jōkyō-reki (1685–1755)
and Senmyō-reki (862–1685) before Tenpō, and they differ in their solar
theory and their intercalation, so running Tenpō backwards would be wrong by
a day here and a whole month there with no warning. All four are implemented
in `hc-calendars-lunar`, and `japanese` asks whichever was in force on the
day, back to 貞観4年1月1日 = 862-02-07. Before that it refuses: a date in
650 gets `CalendarError::BeforeEpoch` instead of a plausible lie.

The **era table is complete regardless**: all 248 nengō from 大化 (645) to
令和 live in the `nengo` module, and `nengo::era_at` will name the era in
force on any day from 645 onward. It simply will not tell you the month and
day.

Era years are counted from the era's first *calendar* year, which is why
安政元年 is the lunisolar year that began in 1854 even though the era was
proclaimed on a day that falls in 1855 in the West. An era change happens
mid-year, so 1989 is both 昭和64年 and 平成元年; both are representable, and a
date on the wrong side of the boundary (昭和64年1月8日) is refused rather than
silently aliased.

The **Northern and Southern Courts** (1331–1392) ran two era systems at once.
This crate does not pick one: every era carries a `Court`, and `era_at` asks
which you mean. `Court::Unified` inside that window returns
`CalendarError::UnknownEra`.

## The Thai lunar calendar, as published

`thai-lunar` is twelve months of 29 and 30 days, month 8 doubled in an
adhikamāsa year (384 days) and month 7 given a 30th day in an adhikavāra one
(355). Which year is which is **not computed**: J. C. Eade's statement of the
*suriyayatra* rule reproduces his own year types but not, reliably, the
Buddhist holidays Thailand kept. So the year types are carried as data, from
2535 to 2570 BE (1992–2027), each read off the Makha, Visakha and Asalha
Bucha dates Thailand published for it — the three full moons fix the type,
and the step to the next year's Makha Bucha checks it — and the calendar
refuses what the table does not reach. The year after the table is carried
through its sixth month, which no year type changes; from month 7 it is
`AfterSupportedRange`.

A year runs from ขึ้น 1 ค่ำ เดือนอ้าย, in November or December, and is
numbered by the Buddhist Era of the Gregorian year its Makha Bucha falls in,
a convention of this crate. The first month 8 of an adhikamāsa year, the
extra one, is `Month::leap(8)`; the day is counted 1 to 30 through the month,
แรม 15 ค่ำ being day 30.

## Correlations, stated

* **Maya**: Goodman–Martínez–Thompson, **584 283**. `13.0.0.0.0` is
  2012-12-21. The alternative 584 285 would move every Western date two days
  later, and it is its own registered calendar, `maya-longcount-gmt2`, rather
  than a switch on the first: a correlation is a claim about history, and two
  claims get two names (policy §5).
* **Aztec**: the fall of Tenochtitlan, **13 August 1521 Julian**, dated
  *1 Coatl*, 2 Xocotlhuetzi — Caso's correlation, as tabulated in
  *Calendrical Calculations*. The 365-day year is modelled **without**
  intercalation, so it drifts a day every four years against the seasons.
* **Balinese Pawukon**: Julian Day Number 146.
* **Javanese pasaran**: anchored through the Pawukon, because they are the
  same five-day cycle; fixed day 0 works out as Ahad Legi.

## Accuracy

Everything except the pre-1873 half of `japanese` is exact integer
arithmetic: no floating point, no astronomy, no approximation. The lunisolar
half of `japanese` inherits the Tenpō calendar's model, where new moons are
good to about a minute and the apparent solar longitude to about 1″; a
computed month boundary can still differ by a day from what the Japanese
calendar bureau actually promulgated, because the bureau computed from its
own tables rather than from modern astronomy. No table of promulgated Tenpō
months ships with this crate, so no disagreement rate against one is claimed.

## Sources

* Nengō table, including the 和暦 and Western changeover dates, the
  Julian/Gregorian split, the Northern/Southern split and the disputed
  starts: Japanese Wikipedia, 元号一覧 (日本).
* Maya and Aztec ordinal arithmetic, the Pawukon's epoch and its three
  irregular cycles: Reingold and Dershowitz, *Calendrical Calculations*
  (4th ed., 2018), chapters 9, 10 and 11.
* Thai lunar year types: the Bank of Thailand's lists of financial-institution
  holidays for 1992–2022 as the Internet Archive keeps them, its
  notifications FPG 3/2565, FPG 8/2566, FPG 5/2567 and 31/2568 for
  2023–2026, and notification 37/2569 in the Royal Gazette of 25 August 2026
  for 2027, all retrieved 2026-09-23; the structure from the *Dictionary of
  Buddhism* (พจนานุกรมพุทธศาสน์ ฉบับประมวลศัพท์) and Thai Wikipedia,
  ปฏิทินจันทรคติไทย.
* Checked independently against five published Galungan dates (each must be
  Buda Kliwon Dungulan), the weton of 17 August 1945 (Jumat Legi, neptu 11),
  a published modern long count and Aztec date, and five Bakumatsu events
  whose 和暦 and Western dates are both in the standard histories.

## Deliberate omissions

* No Japanese lunisolar calendar before Senmyō (862), as above.
* No Thai lunar year before 2535 BE or after 2570 BE, and no *suriyayatra*
  arithmetic to extend it: a year is added when Thailand publishes it.
* No Javanese calendar proper: the Sultan Agung lunar year, its *windu* and
  its Anno Javanico era are a different calendar and are not here.
* No Maya "lord of the night" glyph cycle, no Aztec year bearer.
* The sexagenary **month** pillar follows the lunar month, not the solar
  terms. If you are casting a chart rather than reading a date, it is not
  the function you want; the module says so.
