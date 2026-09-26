# hc-attributes

Cultural attributions to calendar units: birthstones, birth flowers, full-moon
names, traditional month names, and the planetary, deity and colour
associations of the seven-day week.

**This crate reports what traditions claim. It asserts none of them.** Nothing
here is a fact about a month. Everything here is a fact about somebody's list,
and the list is always named.

## There is no such thing as "the" birthstone for a month

There is the American list adopted in Kansas City in August 1912 and revised in
1952, 2002 and 2016. There is the British list the National Association of
Goldsmiths standardised in 1937, carried as the association printed it in 2007
(`birthstones-uk-2007`). There is Japan's list of 1958, substantially
revised on 20 December 2021 — ten stones added after sixty-three years, the most
recent such change anywhere. There are the older European stones all three
replaced. **They disagree in eleven months out of twelve.** Only January is
garnet everywhere.

So this crate has no `birthstone(month)` function and will not grow one:

```rust
use hc_attributes::birthstones::{BIRTHSTONES_JP_2021, BIRTHSTONES_US_2016, all_for_month, stones};
use hc_calendar::Month;

let december = Month::regular(12);
assert_eq!(
    stones(&BIRTHSTONES_US_2016, december),
    Ok(&["turquoise", "zircon", "tanzanite"][..])
);
assert_eq!(
    stones(&BIRTHSTONES_JP_2021, december),
    Ok(&["turquoise", "lapis lazuli", "zircon", "tanzanite"][..])
);

// Or ask everybody at once, which is usually the honest answer.
for (authority, entry) in all_for_month(december).expect("December is a real month") {
    println!("{}: {entry:?}", authority.english_name);
}
```

This is [`docs/policy.md`](../../docs/policy.md) §5 — *competing conventions get
names, not parameters* — applied to the case that shows it most plainly. A
parameter can be forgotten, and the caller who does not know the question exists
gets a silent default. A name cannot be selected by accident, appears in a
listing, is self-documenting at the call site, and asking for all of them and
comparing is one loop.

## The authority model

Every table is an `AttributionTable<N>`: one `Authority` value and `N` entries.
`N` is 12 for a month or zodiac table and 7 for a weekday table. One function,
`AttributionTable::at`, reads all of them — the same data-plus-one-evaluator
split `hc_almanac::rules::rule_applies` makes for thirty-six 暦注.

An `Authority` carries:

| Field | What it says |
|---|---|
| `id`, `english_name` | a stable identifier and a printable name |
| `body` | the organisation that issued it, or `None` if nobody did |
| `region` | where the list is in use — not where the stone comes from |
| `established`, `revised` | dates, to whatever precision the source gives |
| `validity` | the years the list was or is current |
| `provenance` | `Promulgated`, `Recorded`, `Vernacular`, `Contested` or `ModernInvention` |
| `source` | a citation |
| `caveat` | what to know before repeating the list |

`provenance` is the field that stops the crate from flattening a trade
association's press release, a monk's treatise and a poet's invention into one
undifferentiated pile of "tradition". A `Contested` authority always carries a
caveat, and a test enforces it.

An entry is `&[&str]`, never `&str`, because several months carry several
stones, and the order inside an entry is the publishing body's, not a ranking.

## Coverage

| Module | Authorities |
|---|---|
| `birthstones` | pre-1912 Western (Kunz), US 1912, UK as printed in 2007, JP 1958, US 2016, JP 2021 |
| `zodiac_stones` | the Western lapidary tradition by tropical sign (Kunz, 1913) |
| `birth_flowers` | Anglo-American, British florists' |
| `moon_names` | Carver 1778, Maine Farmers' Almanac 1937, Old Farmer's Almanac 1964 and current, plus the Harvest Moon *rule* |
| `month_names` | Old English (Bede), Frankish (Charlemagne), Finnish, Czech — with an English gloss per month |
| `weekday_attributions` | Greco-Roman planets, Germanic deities, Old English names, Japanese 七曜, Sanskrit *vāsara*, Thai colours, Thai deities, weekday stones |
| `gaps` | six subjects the crate declined to ship, with reasons |

Twenty-five named authorities in all, each with a source.

## The Harvest Moon is a rule, not a table row

It is defined as the full moon nearest the September equinox, so about one year
in four it falls in October and September's full moon takes its other name, the
Corn Moon. A table cannot express that, so `moon_names::harvest_moon` computes
it from `hc-seasons`' equinox instant and lunar phases:

```rust
use hc_attributes::moon_names::{harvest_moon_falls_in, september_moon_name};
use hc_seasons::Meridian;

assert_eq!(harvest_moon_falls_in(2024, Meridian::UNIVERSAL), 9);
assert_eq!(harvest_moon_falls_in(2025, Meridian::UNIVERSAL), 10);
assert_eq!(september_moon_name(2025, Meridian::UNIVERSAL), "Corn Moon");
```

This and `zodiac_stones::stones_on`, which asks `hc-seasons` for the sign a
day falls in, are the only computations in the crate. Everything else is a
static lookup.

## The moon names are a publication history, not an ethnographic record

Almanac publishers print Wolf, Snow, Worm and the rest as "Native American moon
names". The record does not support that, and the crate says so rather than
repeating it:

- **Jonathan Carver, 1778** gave twelve lunar-month names and named no nation,
  writing only of "the Indians". Seven of the twelve modern names appear in his
  list verbatim.
- **Vetromile (1856)** and **Peter Jones (1861)** published entirely different
  lists.
- **Richard Irving Dodge (1882)** had never met a Plains people with "a
  permanent, common, conventional name for any moon".
- **The Improved Order of Red Men**, an all-white fraternal order, adopted
  Carver's list and moved it onto Gregorian months.
- **The Maine Farmers' Almanac (1937)** printed a quite different set and called
  them the names of "our early English ancestors".
- **The Old Farmer's Almanac** first published twelve names in **1964** and has
  since changed three of its own: June → Strawberry, Hot → Buck, Travel →
  Beaver.
- "Harvest moon" and "hunter's moon" are recorded in **English** from 1706 and
  1710.

So each table is attributed to its publication, none is labelled with any
nation's name, and a test forbids one appearing.

## Documented gaps

Six subjects a caller might expect and will not find, each absent for a reason
about the sources rather than about effort. They are values in `gaps`, not
prose:

| Gap | Why |
|---|---|
| Japan's day-by-day 誕生花 (366 days) | At least four published Japanese lists disagree day by day, and NHK's ラジオ深夜便 — the most widely heard — has never published its method. |
| "Ayurvedic" birthstones | Retail versions disagree and none cites a text. |
| "Tibetan" birthstones | The same. |
| Hindu gemstones as a month list | The *navaratna* are keyed to the *navagraha* and an individual's chart, not to a birth month. A different key, not a missing list. |
| Aaron's breastplate as a month list | The Hebrew gem names have no settled identification; Josephus gave two lists himself. |
| The "Celtic tree calendar" | Robert Graves's construction in *The White Goddess* (1948). Not a Celtic survival, and thirteen months rather than twelve. |

## What this crate deliberately does not do

- **Pick a default.** No function returns "the" anything.
- **Average two lists.** Where publishers disagree within one country, both are
  shipped, or the disagreement goes in `gaps`.
- **Invent an authority.** See the Japanese 誕生花 gap above.
- **Launder a modern invention.** See the Celtic tree calendar.
- **Duplicate a neighbour.** Japan's 和風月名 (睦月…師走) are `hc-i18n`'s and stay
  there; the 七曜 as an almanac annotation are `hc-almanac`'s; 中秋の名月 and
  十三夜 are `hc-seasons`'. Tests forbid the 和風月名 appearing here.

## Accuracy

The tables are exact: they are transcriptions, and the tests check them against
the published lists — the ten stones the 2021 Japanese revision added, the four
stones the British list has that the current American list does not, the three
names the Old Farmer's Almanac changed after 1964.

The Harvest Moon inherits `hc-seasons`' accuracy: lunar phases within about a
minute, the September equinox within the minute the almanacs round to. The
rule compares two intervals of roughly a fortnight, so neither error can change
which month the answer falls in for any year tested (1900–2099).

## Where the data came from

Every table cites its source in the `Authority::source` field. The principal
ones:

- George F. Kunz, *The Curious Lore of Precious Stones* (Lippincott, 1913) —
  the pre-1912 stones (p. 315), the 1912 American list (pp. 319–320), the
  zodiacal stones (pp. 345–347) and the weekday stones.
- Gemological Institute of America, *Birthstones by Month*; National Jeweler on
  the 2016 spinel addition; Grande & Augustyn, *Gems and Gemstones* (2009),
  p. 335, on tanzanite.
- The National Association of Goldsmiths' birthstone page (archived 2007);
  standardisation date from *The Oxford Companion to the Decorative Arts* (OUP,
  1985), p. 513.
- 全国宝石卸商協同組合, 誕生石改訂事業 (20 December 2021); 組織沿革 for the 1958
  list; 巽忠春, *あなたの宝石* (徳間書店, 1962), p. 77, on why coral and jade
  were added.
- The Old Farmer's Almanac; History.com and Sky & Telescope on the moon-name
  publication history; Carver (1778), Dodge (1882) and the 1937 Maine Farmers'
  Almanac at first hand.
- Bede, *De temporum ratione* (725), ch. 15, tr. Faith Wallis (Liverpool, 1999).
- Einhard, *Vita Karoli Magni* (c. 830), ch. 29, trs. Turner (1880) and De Rynck
  (1999); forms per Braune, *Althochdeutsches Lesebuch*, 17th ed.
- Vojtěch Kebrle, "Česká jména měsíců", *Naše řeč* 23 (1939), pp. 65–67.
- Monier-Williams, *Sanskrit-English Dictionary* (1899), s.v. *vāsara*.
- J. F. Fleet, *Corpus Inscriptionum Indicarum* III (1888), the Eran pillar
  inscription of Budhagupta, 484 CE, the earliest dated Indian weekday.
- Denis Segaller, *Thai Ways* (Silkworm Books, 2005), on the day colours.

## Features

Every table is `&'static` data and every lookup is an array index, so the tables
need neither `std` nor `alloc`. Both features exist only to propagate to the
`hc-*` crates below. The Harvest Moon, `september_moon_name` and
`zodiac_stones::stones_on` reach the astronomy, and `hc-core` refuses to
compile with neither `std` nor a floating-point backend, so a `no_std` build
also enables `libm`, which passes through to `hc-core`:

```sh
cargo build -p hc-attributes --no-default-features --features alloc,libm
```
