# hc-humanize

Human-readable time: *3 days ago*, *in 2 hours*, *2 hours 30 minutes*,
*yesterday at 15:05*, *just over a week*.

`hc-calendar` can say that two instants are 34 200 seconds apart. Nobody says
that. This crate turns a `hc_core::Duration`, or a pair of `hc_calendar::Rd`
days, into the phrase a person would use — in their language, with the
grammar that language actually has.

The grammar is the hard part, and it is why this crate sits on `hc-i18n`
rather than on a table of English strings. *3 дня*, *5 дней*, *21 день*;
*2 dni*, *5 dni*, *22 dni*; *يومين*, *3 أيام*, *11 يومًا*; *2 ddiwrnod*,
*3 diwrnod*, *8 o ddiwrnodau* — four different plural systems, and every one
of them goes through `hc_i18n::PluralRules`. Nothing here decides a plural
form by comparing a number to one.

## What it covers

| Module | Question | Example |
|---|---|---|
| `relative` | When was it, relative to now? | *3 days ago*, *in 2 hours*, *yesterday*, *last month* |
| `duration` | How long is it? | *1 hour, 2 minutes and 3 seconds*, *2h30m*, *3 weeks* |
| `calendar_relative` | Which calendar day was it? | *yesterday at 15:05*, *last Tuesday*, *next month* |
| `approximate` | Roughly how long? | *about 3 hours*, *just over a week*, *nearly a year* |
| `unit_choice` | Which unit, rounded how? | 90 min → *2 hours* or *an hour and a half* |

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
that actually carries the field asked for, then degrades narrow → short →
long within an entry before moving up the locale chain. No function in this
crate knows which languages exist, and none gains a branch when one is added.

### Adding a locale

1. Write a `const LocaleData` in `src/data.rs`, copying the nearest existing
   entry for shape.
2. Add its name to `LOCALES`, keeping the array in tag order.

The consistency tests then check the new entry: every unit stated in the long
style, every `other` pattern able to take a number, no padded strings, no
placeholder in a special word, list patterns shaped `{0}<glue>{1}`, and a
language `hc-i18n` has plural rules for.

Locales shipped: `ar cs cy de en es fr hi id it ja ko nl pl pt ru th tr vi zh
zh-Hant`. `zh-Hans` and `zh-CN` reach the `zh` entry by truncation.

## Accuracy and provenance

The relative-time phrases follow the Unicode CLDR `<fields>` section of
`main/<locale>.xml`, and the undirected unit phrases follow the
`<unit type="duration-…">` section of the same file. They are **hand-checked,
not generated**: a subset of 21 locales chosen to cover the plural systems
that matter, not a copy of CLDR's 600.

Three kinds of string are **not** from CLDR, because CLDR has no field for
them, and are ordinary translations kept in the same table: the approximation
hedges (*just over*, *nearly*), the half-unit idioms (*half an hour*,
*anderthalb Stunden*) and the compact suffixes of *2h30m*.

Unit lengths are the Gregorian means used by CLDR and ICU: 365.2425 days per
year, 31 556 952 s, which divides exactly by 12 and by 4 so that the month
and quarter means are exact integers too. A day is the nominal 86 400 s.

## What it deliberately does not do

- **It does not format dates or times.** That is `hc-format`. The one
  exception is `calendar_relative::write_clock_time`, a documented `H:MM`
  convenience so that *yesterday at 15:05* works end to end; it has no hour
  cycle and no day period.
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

- **The short style is stated for `en`, `de`, `es`, `fr` and `ru` only, and
  the narrow style for `en` and `ja`.** Every other style answers through
  style fallback (narrow → short → long), so most locales return their long
  forms in all three. That is correct for the languages that do not
  abbreviate and incomplete for the ones that do.
- **`ar`, `hi` and `th` state no compact suffixes**, so `DurationStyle::Compact`
  falls back to the root's Latin ones. Inside right-to-left text that needs
  bidi isolation the compact form does not carry; use `Narrow` there.
- **`zh-TW` and `zh-HK` resolve to Simplified.** CLDR would fix this with
  likely-subtags, which this workspace does not model; spell `zh-Hant` out.
- **The root entry is language-free**, not English: an unknown locale gets
  `-3 d`, which is what CLDR's root says. An unknown locale that answered in
  English would be a bug only a speaker of the missing language could see.

## No allocator needed

Every formatter writes into a `core::fmt::Write` sink one piece at a time and
nothing is assembled into an intermediate buffer. The crate builds with
`--no-default-features` and with `--no-default-features --features alloc`,
given the floating-point math every `no_std` build of the workspace needs
from `hc-core`'s `libm` feature (`--features hc-core/libm`); `alloc` adds
only the `String`-returning conveniences (`format`,
`format_amount`, `format_elapsed`) beside the `write` ones.

```rust
use hc_humanize::{RelativeTimeFormatter, TimeUnit};
use hc_i18n::Locale;

let formatter = RelativeTimeFormatter::new("ru".parse::<Locale>()?);
let mut text = String::new();
formatter.write(-5, TimeUnit::Day, &mut text)?;
assert_eq!(text, "5 дней назад");
```

## Spell checking

The foreign-language words the data contains that an English dictionary
would "correct" — *vor*, *als*, *hace*, *seconde*, *dne*, *mis*, *alle* and
the rest — are listed in the workspace's root
[`_typos.toml`](../../_typos.toml). `typos` does not merge nested
configuration files, so a word this crate adds belongs there too.
