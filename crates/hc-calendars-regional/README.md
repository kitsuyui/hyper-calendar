# hc-calendars-regional

Regional, cyclic and era calendars for `hyper-calendar`: Japanese imperial
eras, the four Maya calendars, the two Aztec ones, the Balinese Pawukon, the
Javanese *pasaran*, and the sexagenary cycle.

What they have in common is that **the day has a name before it has a
number**. A Maya day is *4 Ahau 8 Cumku*; a Balinese day is *Buda Kliwon
Dungulan*, a position in three of ten concurrent week cycles; a Japanese day
belongs to an era a government proclaimed. None of them is a count of years
from an epoch with months cut out of it, which is why none of them belongs in
`hc-calendars-solar` or `hc-calendars-lunar`.

| Identifier | What it is |
| --- | --- |
| `japanese` | Imperial era years (和暦); Gregorian from 1873, Tenpō lunisolar before it |
| `maya-longcount` | `baktun.katun.tun.uinal.kin`, in `DateFields::extra` |
| `maya-tzolkin` | 13 numbers × 20 day-names = 260 days |
| `maya-haab` | 18 months of 20 days plus the 5-day Uayeb |
| `maya-round` | The 18 980-day Calendar Round |
| `aztec-tonalpohualli` | 260 days |
| `aztec-xiuhpohualli` | 365 days |
| `balinese-pawukon` | Ten concurrent week cycles over 210 days |
| `javanese-pasaran` | The 5-day market week and the 35-day *wetonan* |
| `sexagenary` | 干支 over years, months and days |

`register_all(&mut CalendarRegistry)` inserts all ten, behind the `alloc`
feature, exactly as `hc-calendars-solar` does.

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

**The gap is deliberate and it is the main limitation of this crate.**
Conversion stops at 天保15年1月1日 = 1844-02-18, the first day the Tenpō
calendar was in force, because Tenpō is the only pre-reform Japanese calendar
implemented anywhere in this workspace. Japan used Kansei-reki (1798–1844),
Hōryaku-reki (1755–1798), Jōkyō-reki (1685–1755) and Senmyō-reki (862–1685)
before it; they differ in their solar theory and their intercalation, so
running Tenpō backwards would be wrong by a day here and a whole month there
with no warning. A date such as 元禄15年12月14日 gets `CalendarError::BeforeEpoch`
instead of a plausible lie.

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

## Correlations, stated

* **Maya**: Goodman–Martínez–Thompson, **584 283**. `13.0.0.0.0` is
  2012-12-21. The alternative 584 285 would move every Western date two days
  later; the constant is exported as `maya::GMT_PLUS_TWO_CORRELATION` so the
  difference is visible in the source, but there is no switch, because a
  correlation is a claim about history.
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
good to about a minute but apparent solar longitude only to about 0.01°, so a
computed month boundary can differ by a day from what the Japanese calendar
bureau actually promulgated. No table of promulgated Tenpō months ships with
this crate, so no disagreement rate against one is claimed.

## Sources

* Nengō table, including the 和暦 and Western changeover dates, the
  Julian/Gregorian split, the Northern/Southern split and the disputed
  starts: Japanese Wikipedia, 元号一覧 (日本).
* Maya and Aztec ordinal arithmetic, the Pawukon's epoch and its three
  irregular cycles: Reingold and Dershowitz, *Calendrical Calculations*
  (4th ed., 2018), chapters 9, 10 and 11.
* Checked independently against five published Galungan dates (each must be
  Buda Kliwon Dungulan), the weton of 17 August 1945 (Jumat Legi, neptu 11),
  a published modern long count and Aztec date, and five Bakumatsu events
  whose 和暦 and Western dates are both in the standard histories.

## Deliberate omissions

* No pre-Tenpō Japanese lunisolar calendar, as above.
* No 改元当時 era table — only the 公式 (retroactive) boundaries, under which
  an era ends the day before the next begins.
* No Javanese calendar proper: the Sultan Agung lunar year, its *windu* and
  its Anno Javanico era are a different calendar and are not here.
* No Maya "lord of the night" glyph cycle, no Aztec year bearer.
* The sexagenary **month** pillar follows the lunar month, not the solar
  terms. If you are casting a chart rather than reading a date, it is not
  the function you want; the module says so.
