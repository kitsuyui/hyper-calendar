# hc-seasons

Seasonal subdivisions: what a year is made of once you stop counting months.

A calendar names days. This crate names the *parts of the year* underneath
that naming — the 24 solar terms, the 72 pentads, the Japanese 雑節, the 六曜
cycle, the Moon's phases, the zodiac in its three incompatible divisions, and
the four seasons under each of the three definitions that disagree about them.
It contains no calendar.

The solar terms, the pentads, the meridians and the zodiac are written up in
[`docs/systems/solar-terms-and-pentads.md`](../../docs/systems/solar-terms-and-pentads.md):
the rules from the sources, a worked example against the 暦要項, what is
carried and what is not, how the instants compare with the published
almanacs, and the sources, keyed in
[`docs/references.bib`](../../docs/references.bib). This README summarises it
and states the crate's own facts.

| Module | Covers |
| --- | --- |
| `solar_terms` | 二十四節気, both orderings, the 節気 / 中気 split |
| `pentads` | 七十二候, both the Chinese and the Japanese name sets |
| `zassetsu` | 節分, 彼岸, 社日, 八十八夜, 入梅, 半夏生, 土用 + 丑の日, 二百十日, 二百二十日 |
| `rokuyo` | 六曜: 先勝 友引 先負 仏滅 大安 赤口 |
| `san_fu` | 三伏 (初伏, 中伏, 末伏) counted in 庚 days from the summer solstice and 立秋, and 數九, the nine nines from the winter solstice |
| `quarter_days` | the quarter days and term days of England and Wales, Ireland and Scotland, traditional and under the 1990 Act |
| `moon_calendar` | phase names, 月齢, illuminated fraction, a month's four principal phases, 十五夜, 十三夜 |
| `seasons` | astronomical, meteorological and East Asian seasons |
| `zodiac` | 黄道十二宮: the tropical Western signs, the sidereal rāśi with the ayanamsa, the Indian solar months, and the Chinese 十二次 |
| `lunisolar` | a minimal month/day derivation for 六曜 and the moon-viewing nights — see Known gaps |

## A day is not an instant

Every event here starts as an astronomical instant in Universal Time and ends
as a calendar day. The step between them is a choice of meridian, and it
changes answers. Beijing is an hour behind Tokyo, so over 1950–2049 the two
almanacs put a solar term on different dates **98 times out of 2400**. A
lunar month boundary shifts the same way, which is why Chinese and Japanese
new year occasionally differ.

So nothing here guesses. Every function that turns an instant into an `Rd`
takes a `Meridian`,
and `Meridian::JAPAN` (UTC+9) and `Meridian::CHINA` (UTC+8) are the meridians
the respective national almanacs are computed at. `Meridian` is a fixed
offset, not a time zone: no daylight saving, no political history. That is
`hc-tz`'s job.

## One computation, three zodiacs

The crate already found the 24 solar terms as the instants the Sun's apparent
longitude reaches a multiple of 15°. A zodiac sign is the same search at a
multiple of 30°, so `zodiac` adds naming and a zero point and no new
astronomy. The three zero points in use are 15° and 24° apart:

| Division | Zero point | Boundaries |
| --- | --- | --- |
| `zodiac::tropical` | the March equinox | **exactly the twelve 中気** |
| `zodiac::sidereal` | the fixed stars, an *ayanamsa* behind the equinox | about 24° later |
| `zodiac::chinese_twelve` | 大雪 at 255° | **exactly the twelve 節気** |

The first and third rows are identities, not resemblances, and tests assert
them: Aries opens at 春分, Cancer at 夏至, Libra at 秋分, Capricorn at 冬至,
and each 次 opens at a 節気 and holds a 中気 in its middle. A 次 is therefore a
tropical sign rotated back fifteen degrees, and is the same interval as the
節月 of the four-pillar reckoning.

### Why the sidereal one is here

Twenty-four degrees out of thirty is four fifths. **For 295 days of 2024 the
tropical and sidereal signs disagree** — most of tropical Aries is sidereal
Pisces, most of tropical Taurus is sidereal Aries — so a library that shipped
only one of them would be answering "which sign" with one of two answers and
not saying which. `SiderealSign` is also exactly the **rāśi**, and the solar
calendars of Tamil Nadu, West Bengal, Assam, Odisha and Kerala take their
months from it, so `zodiac::rashi` is month names over the same boundaries and
no second algorithm.

The ayanamsa is a parameter: four named anchors are shipped, `Ayanamsa::new`
takes any other, and the document says where each anchor comes from. Moving
from Lahiri to Raman — 1.45° — moves **all twelve** month boundaries by a day
or more.

The Chinese zodiac **animal** of a year is not here and is not a 次. It is the
earthly branch of the sexagenary year, a counting cycle with no angle in it,
and it lives in `hc_calendar::cycle` — `sexagenary_year` and
`Sexagenary::zodiac_animal`. The 次 do carry the matching 十二辰 branch, which
runs *backwards* against them (星紀 is 丑, 玄枵 is 子), and the docs say so
loudly enough that nobody should read a birth animal out of this module.

## Accuracy, and the measurement that proves it

`hc-astro`'s apparent solar longitude is VSOP87, good to about 1″ — under
half a minute of solar motion — so an event within about a minute of local
midnight can still be assigned the wrong *day*. The document measures that
two ways. Against the 春分の日 and 秋分の日 the National Astronomical
Observatory of Japan publishes, `tests/japanese_equinox_days.rs` finds **0
disagreements in 240 days** of 1980–2099, and shows that computing the same
holiday in Universal Time instead of JST would get 88 of them wrong, which is
what the `Meridian` argument exists to prevent. Against the 暦要項's times
for 2024–2026, `tests/rekiyoko_solar_terms.rs` finds every one of the 72
terms on the published day. For the 54 terms up to 2026-04-01, where
`hc-astro` reads the observed ΔT, 53 are on the published minute and the
residual is **−30 s to +29 s with a mean of +1.1 s**; the polynomial ΔT
alone would leave those at a mean of −4.3 s. The 18 terms after the table's
end, still on the polynomial, run **6.4 s early** on average, 16 of them on
the published minute.

Terms are 15 days apart, pentads 5 and signs 30, so the *term*, *pentad* or
*sign* is never wrong; only its day, and only at a midnight boundary. Lunar
conjunctions land within about a minute, so month boundaries, phase dates,
十五夜 and 六曜 are firmer than the solar-term dates. 月齢 and the illuminated
fraction are quoted for **local noon**, as NAOJ quotes its 正午月齢 for 12:00
JST; `moon_age_at` takes any instant. The sidereal boundaries carry a second,
independent uncertainty, the few tens of arcseconds by which published values
of a named ayanamsa disagree.

`TropicalSign::conventional_period` ships the fixed dates newspapers print
— "Aries: March 21 – April 19" — and `tests/zodiac_conventional_dates.rs`
measures them against the computed ingresses at three meridians and prints
the table; the document reads the result. Greenwich and Tokyo put a sign
boundary on different dates **447 times in 1200 sign-years, 37.2 %**, so no
fixed list could be right everywhere at once.

## The reference data, and where it came from

* **The 24 terms, the 72 pentads, the zodiac signs, the rāśi, the ayanamsa
  anchors and the 十二次** are sourced, column by column, in the document.
  Two facts worth restating here: three of the 24 terms are written
  differently in traditional Chinese and in Japanese shinjitai (驚蟄/啓蟄,
  小滿/小満, 處暑/処暑), and **only 21 of the 72 pentads** are written
  identically in the Chinese and the Japanese list, which is why shipping one
  set and calling it "the 72 pentads" is the usual mistake.
* **雑節 dates** and the **equinox-day table** are checked against the
  National Astronomical Observatory of Japan's 暦要項.
* **土用の丑の日** is checked against the published eel days for 2023–2025,
  two of which had a 二の丑.
* **中秋の名月 and 十三夜** against the published dates for 2020–2025.
* **Lunar new year** for 2015–2026, which every almanac agrees about, anchors
  the lunisolar derivation.

None of these reference values was produced by this crate.

## Rules expressed as data

`ZassetsuRule` has four shapes and all twenty-one 雑節 are one of them:

| Shape | Used by |
| --- | --- |
| `SolarLongitude(deg)` | the four 土用 entries (27°, 117°, 207°, 297°), 入梅 (80°), 半夏生 (100°) |
| `OffsetFromTerm { term, days }` | 節分 (−1 from a 立 term), the three 彼岸 days (−3, 0, +3 from an equinox) |
| `NightsFromBeginningOfSpring(n)` | 八十八夜 (88), 二百十日 (210), 二百二十日 (220) |
| `NearestStemDay { term, stem }` | 社日, the 戊 day nearest an equinox |

`Ayanamsa` is the same idea in the other module: an anchor value at an anchor
Julian date and nothing else, so Lahiri, Raman, Krishnamurti and Fagan–Bradley
are four rows of data over one precession series, and a fifth is a constructor
call rather than a pull request.

So `zassetsu::day_of` is one `match` over data, and the four 土用 entries being
exactly 18° before their closing term is a *test*, not four magic numbers.

## What this crate deliberately does not do

* **No 平気.** The 5° and 15° divisions here are 定気, arcs of the ecliptic.
  The older almanacs divided the year equally in *time*, and those give
  different dates; the document says which almanacs and until when. Not
  implemented.
* **No calendar.** Nothing here implements `hc_calendar::Calendar`. A solar
  term is not a date system; it is a subdivision that several date systems
  refer to.
* **No holidays.** 春分の日 is a public holiday *because* it is the equinox,
  but the holiday law, the substitute-holiday rules and the national calendar
  live in `hc-holiday`.
* **No time zones.** `Meridian` is a fixed offset with no history.
* **No default meridian, and no default season definition.** A caller who has
  not said which they mean has not decided yet, and a library that decided for
  them would be asserting something it cannot know.
* **Japanese 雑節 only, and the Chinese 三伏 and 數九.** The 雑節 list is
  Japan's; of China's own, the dog days and the nines are in `san_fu`, and
  China's 入梅 and 出梅 and Korea's are not carried.
* **The Sun only, in the zodiac too.** Every sign function here places the
  **Sun**. There is no Moon sign, no planet in a sign, no ascendant, no house
  system and no chart. That matters most for the sidereal side: in Jyotiṣa a
  person's *janma rāśi* is the **Moon's** rāśi, not the Sun's, so
  `sidereal::sign_on_day` is not the birth sign an Indian almanac would give.
  A Moon sign needs `hc_astro::lunar_longitude` minus the ayanamsa, which is
  two lines this crate deliberately does not write, because once it has a Moon
  sign it is being asked for a chart.
* **No constellation boundaries.** A sign is a 30° arc of the ecliptic. The
  IAU constellations are irregular polygons of very different sizes, and the
  Sun passes through thirteen of them — Ophiuchus among them — for between
  seven and forty-five days each. The recurring "there is a thirteenth sign"
  story is a statement about constellations, not about signs, and nothing here
  computes constellation membership.
* **No zodiacal 平気 either.** Like the solar terms, the 30° divisions are
  定気, arcs of the ecliptic, so signs are 29 to 32 days long. Equal-time
  divisions are not implemented.
* **No Indian calendar.** `zodiac::rashi` names the twelve solar months and
  finds their boundaries. It does not number days within a month, number
  years, know the Kollam, Bengali San or Śaka epochs, or implement the
  regional rule for a saṅkrānti that falls late in the day. Those are
  `hc-calendars-regional`'s. Nor is any of it the lunisolar Hindu calendar,
  whose months begin at a new or full moon, or the national civil Śaka
  calendar (CLDR `indian`), which has fixed month lengths tied to the tropical
  equinox.
* **No astrology.** The element, modality and ruling planet are shipped as the
  data of a naming scheme with citations. The crate makes no claim about what
  any of it means, and there is no interpretation, compatibility or forecast
  anywhere in it.
* **No pre-1873 六曜.** The daily six-day cycle is a Meiji-era popularisation;
  earlier forms had different names, order and length. Answers before 1873 are
  extrapolations of the modern rule, not what any surviving almanac says.

## Known gaps

* **`lunisolar` is a minimal derivation.** 六曜 is a function of the lunisolar
  month and day, and this crate computes it from its own minimal derivation
  rather than from `hc-calendars-lunar`: new moon starts the month, the 中気
  it contains numbers it, a month without a 中気 is a leap month repeating the
  previous number. Routing every 六曜 through the full calendar would make each
  one pay for a new-moon search; the module documentation records the cost.
  With the `lunar` feature, `lunisolar::exact_lunisolar_day` reads the same
  day from the unbounded Tenpō calendar, and a test counts the difference: 89
  of the 3,653 days of 2024–2033 differ, all in one run from 25 August to 21
  November 2033.

  What the derivation does not implement: the leap month is properly the
  *first* 中気-less month after the eleventh, decided by looking at the whole
  year between two winter solstices, and a month can occasionally hold two
  中気 — the case that has made 天保暦's rule formally ambiguous since 1844
  and is why Japan's official calendar has no legal lunisolar definition
  today. This code decides month by month and takes the later 中気 when a
  month holds two. It reproduces every lunar new year 2015–2026 and the 2023
  閏二月, and it puts the winter solstice in month 11 for every year
  1990–2039.
* **社日's tie-break.** When the equinox falls on a 癸 day the two 戊 days are
  exactly five days either side. This crate takes the earlier; sources differ,
  and there is no switch for it.
* **`pentad_moment` near 280°.** A leap year whose 1 January falls just before
  the 280° crossing contains that pentad twice, and `pentad_moment` returns
  the first. `pentads_in_year` walks the year instead and cannot
  double-count.
* **Southern-hemisphere East Asian seasons** are a mechanical flip. The 立
  terms describe the Chinese agricultural year and have no southern form; the
  flip is offered because refusing would be more annoying than useful.
* **No `Calendar` impl, no registry entry.** Deliberate; see above.
* **Ayanamsa anchors disagree at the tens-of-arcseconds level.** The values
  here are the Swiss Ephemeris ones; other published tables for the same named
  ayanamsa differ by a few tens of arcseconds, which is a few minutes of time
  in the Sun's motion and can move a saṅkrānti *day* at a midnight boundary.
  There is no way to resolve this, because the disagreement is about a
  convention and not about the sky, so the anchor is a public field of
  `Ayanamsa` and the README says which one was picked.
* **`sidereal::ingress_moment` assumes one saṅkrānti per Gregorian year.**
  True for the Lahiri anchor from roughly 1100 CE onwards, because on
  1 January the Sun stands near 256° sidereal, far from a boundary. Around the
  year 1000 the ayanamsa was near 10°, which put a boundary on 1 January
  itself, and a Gregorian year near then can hold a saṅkrānti twice or not at
  all. `sidereal::signs_in_year` walks the year instead and cannot
  double-count. This is the same shape of caveat as `pentad_moment` near 280°.
* **The 十二次 / Western sign equation is traditional, not exact.** Under the
  定気 rule a 次 and the sign it is equated with overlap for about half their
  length; the test measures that and finds agreement on roughly half the days
  of a year. Nothing here pretends the two arcs coincide.

## Sources

The solar terms, pentads, meridians and zodiac cite their sources in the
document, with keys in `docs/references.bib`. The rest of the crate cites:

* Jean Meeus, *Astronomical Algorithms*, 2nd ed., Willmann-Bell 1998, through
  `hc-astro`.
* Edward M. Reingold and Nachum Dershowitz, *Calendrical Calculations*, 4th
  ed., Cambridge 2018 — the Rata Die pivot and the Gregorian arithmetic that
  `gregorian.rs` adapts from `hc_calendar::gregorian`.
* National Astronomical Observatory of Japan, 暦要項 (*Calendar Essentials*),
  published annually in the *Official Gazette* — the 雑節 dates and the
  春分の日 / 秋分の日 table.

## Testing

Unit tests, integration tests and documentation tests; the zodiac work
accounts for the largest share. Exact counts are not quoted here, because a
number in prose drifts away from the code within a release and says nothing
a reader can use — `cargo test -p hc-seasons` is the authority.

```
cargo test -p hc-seasons
cargo clippy -p hc-seasons --all-targets --all-features -- -D warnings
cargo fmt -p hc-seasons -- --check
cargo build -p hc-seasons --no-default-features --features alloc,libm

# The drift table above, printed:
cargo test -p hc-seasons --test zodiac_conventional_dates -- --nocapture
```
