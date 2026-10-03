# hc-humanize

Human-readable time: *3 days ago*, *in 2 hours*, *2 hours 30 minutes*,
*yesterday at 15:05*, *just over a week*.

`hc-calendar` can say that two instants are 34 200 seconds apart. Nobody says
that. This crate turns a `hc_core::Duration`, or a pair of `hc_calendar::Rd`
days, into the phrase a person would use — in their language, with the
grammar that language has.

It depends on `hc-core`, `hc-units`, `hc-calendar`, `hc-i18n` and
`hc-format`.

The grammar is the hard part, and it is why this crate sits on `hc-i18n`
rather than on a table of English strings. *3 дня*, *5 дней*, *21 день*;
*2 dni*, *5 dni*, *22 dni*; *يومين*, *3 أيام*, *11 يومًا*; *2 flynedd*,
*3 blynedd*, *8 mlynedd* — four different plural systems, and every one
of them goes through `hc_i18n::PluralRules`. Nothing here decides a plural
form by comparing a number to one.

## What it covers

| Module | Question | Example |
|---|---|---|
| `relative` | When was it, relative to now? | *3 days ago*, *in 2 hours*, *yesterday*, *last month* |
| `duration` | How long is it? | *1 hour, 2 minutes, and 3 seconds*, *2h30m*, *3 weeks* |
| `calendar_relative` | Which calendar day was it? | *yesterday at 15:05*, *last Tuesday*, *next month* |
| `approximate` | Roughly how long? | *about 3 hours*, *just over a week*, *nearly a year* |
| `unit_choice` | Which unit, rounded how? | 90 min → *2 hours* or *an hour and a half* |
| `natural` | What would Python's `humanize` say? | *a moment*, *1 year, 3 months*, *2 days, 1 hour and 33.12 seconds*, *1,000,000*, *1.2 billion*, *103rd*, *three*, *1 3/10*, *1.50 kV*, *2.9 KiB*, *3 секунды назад* |

`natural` is a second convention beside CLDR's, not a replacement for it:
the thresholds, arithmetic and strings of the Python `humanize` package,
release 4.16.0 — `naturaldelta`, `naturaltime`, `naturalday`, `naturaldate`,
`precisedelta`, `ordinal`, `intcomma`, `intword`, `apnumber`, `fractional`,
`scientific`, `metric`, `clamp`, `naturalsize` and `natural_list` — for code
being ported from Python that depends on those exact phrases. Its tests quote
`humanize`'s documented examples, and each function was run beside a
transcription of its source over hundreds of thousands of random cases
(`docs/systems/python-compatibility.md` has the counts). `precisedelta`
reproduces its rounding, carries and truncated remainders; `intword` reaches
the googol through `intword_digits`, which takes an integer of any length.

`humanize` translates through gettext catalogues, and all 35 that release
4.16.0 ships are carried, generated from their `.po` files by
`scripts/humanize-gettext.py` (`--check` verifies the file):
`NaturalPhrases::by_catalogue("ru_RU")` is what
`humanize.i18n.activate("ru_RU")` loads, and `Natural::for_language("de")`
finds the one catalogue of a language that has exactly one. A plural message
is chosen by the catalogue's own `Plural-Forms` expression, evaluated as
written (`natural::gettext`) and not by CLDR's categories, since the two
disagree for some languages and for some counts; a message the catalogue
leaves fuzzy or untranslated is English, as `msgfmt` compiles it. Python's
number separators, which it keeps in code and not in a catalogue, are
`NaturalPhrases::grouping`. A catalogue is data under policy §2: one
`const` in a generated file and no branch in any function.

Where `humanize`'s `main` branch has changed behaviour since the release
(the years of `naturaldelta`, `fractional`, a carry in `intword` and in
`metric`, the French decimal separator), this crate follows the release,
since that is what `pip install humanize` gives; `docs/python-parity.md`
lists each. `naturalday` writes days that are not today, tomorrow or
yesterday with `hc-format`'s `strftime`, so it and `naturaldate` come with
the `format` feature, the one that brings `hc-format`.

`relative` follows the CLDR `relativeTime` model properly. Units are second,
minute, hour, day, week, month, quarter and year; styles are `Long`, `Short`
and `Narrow`; and `Numeric::Always` / `Numeric::Auto` is the same choice
`Intl.RelativeTimeFormat` offers between the numeric pattern (*1 day ago*)
and the special word (*yesterday*). `Auto` uses a word only where the
language has one for exactly that offset, and falls back to the number
otherwise.

`unit_choice` makes both of its decisions parameters rather than opinions: a
`Thresholds` table says which unit a span belongs in, and a `RoundingPolicy`
(`Ceil`, `Floor`, `Nearest`, `Truncate`, `NearestHalf`) says how to round
into it. Three tables ship — `DEFAULT`, `EXACT`, `WITH_QUARTERS` — and a
caller with different needs passes their own rather than forking the crate.

## Data is not code

Every locale is **one `pattern::LocaleData` value of `&'static` strings** in
`data::LOCALES`. Lookup walks `Locale::fallback()` and takes the first entry
that carries the field asked for, then degrades narrow → short →
long within an entry before moving up the locale chain. No function in this
crate knows which languages exist, and none gains a branch when one is added.

### Adding a locale

1. Add the tag and its CLDR files to `LOCALES` in
   `scripts/humanize-cldr.py` and run it, which rewrites
   `src/data/cldr48.rs`; add the tag to `scripts/humanize-cldr-sample.py`
   and run it too.
2. In `src/data.rs`, take the new entry into `LOCALES`, keeping the array in
   tag order, or write a `const LocaleData` that adds the hedges, weekday
   phrases, idioms, compact suffixes and indefinite units to it, copying the
   nearest existing entry for shape.
3. A CLDR value a source argues against goes into
   `src/data/cldr48_overrides.tsv` with its reason, and into
   `docs/i18n.md`; the generator is then run again.

The consistency tests then check the new entry: every unit stated in the long
style, every `other` pattern able to take a number, no padded strings, no
placeholder in a special word, list patterns shaped `{0}<glue>{1}`, and a
language `hc-i18n` has plural rules for.

Locales shipped, 39 of them: `ar ar-EG cs cy de en en-001 en-GB es es-419
fil fr ha hi id it ja ko mn mr nl pa-Guru pcm pl pt pt-PT ru sw te th tr ur
ur-IN vi yue-Hans yue-Hant zh zh-Hant zh-Hant-HK`. The regional entries take
their CLDR part from their own files and everything CLDR has no field for
from their language's entry, so that `en-GB` writes *1 hour, 2 minutes and
3 seconds* and *3 mo ago*, as `en_001.xml` has them, and `zh-HK` *3 星期前*.
`hc-i18n`'s fallback chain applies CLDR's likely script to a Chinese,
Cantonese or Punjabi tag and CLDR's `parentLocales`, so `zh-Hans` and
`zh-CN` reach the `zh` entry, `zh-TW` reaches `zh-Hant` and `zh-HK` and
`zh-MO` `zh-Hant-HK`, `en-AU` and `en-IN` reach `en-001` and `es-MX`
`es-419`, `yue-CN` reaches `yue-Hans`, and
`pa` reaches `pa-Guru`; `pa-PK`, Punjabi in the Arabic script, whose CLDR
file states no fields, reaches root.

## Accuracy and provenance

The relative-time phrases follow the Unicode CLDR `<fields>` section of
`main/<locale>.xml`, and the undirected unit phrases follow the
`<unit type="duration-…">` section of the same file, with its list
patterns, the decimal separator of the digits `hc-i18n` writes and its
long `relative` date-time pattern, which UTS #35 Part 4 gives a relative
date with a time (*yesterday at 15:05*; `es` *ayer, 15:05*, not its
`atTime` *ayer a las 15:05*). All 39 locales take that part from their
CLDR 48 file, **generated**: `scripts/humanize-cldr.py` reads the files of
the `release-48` tag, resolves each value in all three styles as CLDR does
— a value a file marks `↑↑↑` is the parent file's, then what `root.xml`'s
aliases give (narrow from short, short from long), and last the style's
`other` — and writes `src/data/cldr48.rs`. Nothing here is a copy of
CLDR's 600.

The generator then applies one list, `src/data/cldr48_overrides.tsv`, 76
values, each a value of CLDR's that a source argues against, with its
reason, which the generated file repeats above the value and
[`docs/i18n.md`](../../docs/i18n.md) lists: Traditional Chinese's duration
quarter *{0} 刻*, a quarter of an hour; Simplified Chinese's two
alternatives in one value, *这一时间 / 此时*; capitals where the rest of the
file is lower-case, in the Vietnamese long day words, one Indonesian and
one Filipino word and two Swahili past patterns; a narrow month or year
that is also the narrow minute or hour, in English, Japanese, Dutch and
Hausa; a Turkish narrow hour; a past with the future's wording, in Arabic
and Nigerian Pidgin; Nigerian Pidgin futures with no space after the
number, and one without its *wé de kọm*; an Arabic dual with a numeral;
and, in Welsh, a misspelt duration quarter and a misplaced full stop.

`tests/cldr48_resolved.rs` checks every value the crate takes from CLDR —
the past, future and duration patterns of all eight units in each plural
category, their relative words, the list patterns, the decimal separator
and the relative date-time pattern, in all three styles — against CLDR
48's own resolution of them, read from the `cldr-json` 48.0.0 packages
(`cldr-json-48`) by `scripts/humanize-cldr-sample.py`, which shares no code
with the generator. The sample is kept in the repository, so the test
needs no network. A value may differ from CLDR's only where the override
list says so, and every override must differ from CLDR's, be the value the
crate carries and have its reason in the generated file.

Five kinds of string are **not** from CLDR, because CLDR has no field for
them, and are ordinary translations written in `src/data.rs`: the
approximation hedges (*just over*, *nearly*), the weekday phrases (*last
Monday*), the half-unit idioms of the long style (*half an hour*,
*anderthalb Stunden*), the compact suffixes of *2h30m* and the indefinite
units (*an hour*). `fil`, `ha`, `mr`, `pa-Guru`, `pcm`, `pt-PT`, `sw`,
`te`, `ur`, `yue-Hans` and `yue-Hant` carry none of them: their hedges are
root's language-free `~5` and `<5`, `pt-PT`'s are `pt`'s, and their counts
are written with a numeral rather than an indefinite article.

Unit lengths are the Gregorian means used by CLDR and ICU: 365.2425 days per
year, 31 556 952 s, which divides exactly by 12 and by 4 so that the month
and quarter means are exact integers too. A day is the nominal 86 400 s.

## What it deliberately does not do

- **It does not format dates or times.** That is `hc-format`. The
  exceptions are `calendar_relative::write_clock_time`, a documented `H:MM`
  convenience so that *yesterday at 15:05* works end to end, with no hour
  cycle and no day period; and `natural::Natural::naturalday`, which hands
  a day to `hc-format`'s `strftime` as Python's `humanize` does.
- **It does not know what "now" is.** Every entry point takes both ends, or a
  span, from the caller. A humaniser that read a clock could not be tested.
- **It does not do calendar arithmetic on months.** A bare span has no
  calendar to anchor a month to, so the default duration component set is
  days, hours, minutes and seconds. Weeks, months and quarters are available
  through `with_units` for callers who know what they are asking for.
- **It has no compact-notation plural operands** (`c`/`e`), inherited from
  `hc-i18n`, and no gendered agreement. Weekday phrases avoid agreement by
  periphrasis where a language inflects the demonstrative — Russian says
  *понедельник на прошлой неделе* rather than *в прошлый понедельник*,
  because the latter is wrong for *среда*.

### Known gaps

- **A style a file does not distinguish is not stated.** Korean,
  Vietnamese and Traditional Chinese state the long style alone; Swahili
  has no short style of its own; Portuguese, Punjabi, Cantonese in both
  scripts and Simplified Chinese have no narrow style of their own. Their
  CLDR files, overrides applied, say nothing there the wider style does
  not: Vietnamese's short day words and Traditional Chinese's short
  quarter are the long ones once overridden, and Swahili's short style is
  its long one once the long seconds are. Every such style answers through
  style fallback (narrow → short → long).
- **Capitals CLDR writes and no override lowers.** Filipino's
  *Samakalawa* and Nigerian Pidgin's relative words (*Yẹ́stadè*,
  *Lást wik*) are capitalised in every style, and their files write those
  words nowhere in lower case, so there is no value of the file's own to
  keep ([`docs/i18n.md`](../../docs/i18n.md)).
- **The half-unit idioms are the long style's.** A short or narrow half
  hour is the decimal, *1.5 hr*.
- **`ar`, `hi` and `th` state no compact suffixes**, so `DurationStyle::Compact`
  falls back to the root's Latin ones. Inside right-to-left text that needs
  bidi isolation the compact form does not carry; use `Narrow` there.
- **The root entry is language-free**, not English: an unknown locale gets
  `-3 d`, which is what CLDR's root says. An unknown locale that answered in
  English would be a bug only a speaker of the missing language could see.

## No allocator needed

Every formatter writes into a `core::fmt::Write` sink one piece at a time and
nothing is assembled into an intermediate buffer. The crate builds with
`--no-default-features` and with `--no-default-features --features alloc`,
given the floating-point math every `no_std` build of the workspace needs
from the `libm` feature, which passes through to `hc-core`; `alloc` adds
only the `String`-returning conveniences (`format`,
`format_amount`, `format_elapsed`) beside the `write` ones.

```rust
use hc_humanize::{RelativeTimeFormatter, TimeUnit};
use hc_i18n::Locale;

let formatter = RelativeTimeFormatter::new("ru".parse::<Locale>()?);
let mut text = String::new();
formatter.write(-5, TimeUnit::Day, &mut text)?;
assert_eq!(text, "5 дней назад");

// A traditional-script tag reaches the zh-Hant entry.
let mut text = String::new();
RelativeTimeFormatter::new("zh-TW".parse::<Locale>()?).write(-3, TimeUnit::Week, &mut text)?;
assert_eq!(text, "3 週前");
# Ok::<(), Box<dyn std::error::Error>>(())
```

| Feature | Effect |
| --- | --- |
| `std` (default) | implies `alloc` |
| `alloc` | the `String`-returning conveniences |
| `format` (default) | `naturalday` and `naturaldate`, through `hc-format`'s `strftime` |
| `libm` | software floating-point math through `hc-core`, for `no_std` targets |

## Spell checking

The foreign-language words the data contains that an English dictionary
would "correct" — *vor*, *als*, *hace*, *seconde*, *dne*, *mis*, *alle* and
the rest — are listed in the workspace's root
[`_typos.toml`](../../_typos.toml). `typos` does not merge nested
configuration files, so a word this crate adds belongs there too.
