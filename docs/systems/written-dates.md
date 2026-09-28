# Written dates, read back

Backs `hc-format`'s `label::parse_date`, the facade's
`lines::parse_date` and the export `hc_parse_date`: a date as a locale
writes it in one calendar, read back as the calendar's fields and the
fixed day, or refused with the reason.

## What it is

A date a reader types, pastes or copies from a document is text:
令和8年9月28日, 康熙五十二年十一月初一, *28 Eylül 2026*, ٢٨ سبتمبر ٢٠٢٦. The
library writes such text for every registered calendar in every carried
locale (`label::write_date`, which `hc_describe_day`'s formatted column
uses), from the vocabulary and date templates of `hc-i18n`. Reading it
back is the inverse, and it is harder in one way that matters here: the
text often does not say everything the calendar needs, and a guess would
return a day with confidence the text does not support.

UTS #35, Part 4, "Parsing Dates and Times" [uts35-dates-48], describes the
problem for CLDR's patterns. It splits the fields into numeric and symbolic
ones, and advises four things:

- try the locale's own pattern first;
- check symbols first, so that the Japanese 3月 is read as a month and not
  as a number and a literal;
- accept a name at every width, standalone as well as in a date, with or
  without its abbreviation period, in either case, where it is unique;
- warn that a date written with narrow names may read back as another
  date.

It also notes that `yy` keeps "just the two low-order digits of the
year". The advice for text that does not fit the pattern is heuristic
resynchronisation. This reader follows the first three points and not
the heuristics: §4 of the [policy](../policy.md) says the library refuses
to guess.

## How it works

The reader walks the same templates the renderer fills. For each placeholder
it tries everything the renderer could have written there, then keeps a
match only if the calendar agrees with all of it.

1. **Templates.** A locale states how it writes a calendar's dates as a
   [`TemplateChain`](../../crates/hc-i18n/src/names.rs):
   - the entry's templates, then the calendar's notation, then the
     locale's, then the default;
   - for each, a date template, and within it the year's (with the
     first-year template, `{era}元年`) and the day's.

   The reader tries each distinct date template, and inside it each year
   and day template.
2. **Placeholders.** At a placeholder the reader tries every candidate.
   - **Eras:** every era name in the locale's fallback chain, English's and
     the calendar's own. The calendar's own come through the new
     `DynCalendar::era_code`, which lists the codes `era_name` answers
     for, such as the 248 nengō.
   - **Months:** every month label at every width and in both contexts,
     in a year with and without the intercalary month, as the renderer
     writes it: the name, the calendar's own name, or the number, with the
     leap prefix. A label with Latin digits, 9月, is also tried with its
     number in the reader's other numbering systems, 九月.
   - **Days and extra fields:** the locale's day names, named cycle values
     (the Pawukon's days, a Tamil year name) and the sexagenary pairs.
   - **Numbers:** read in the locale's digits and in Latin digits, and in a
     locale written in Han characters also as Han numerals and positional
     Han digits. A Han numeral counts only if the system writes that value
     the same way, so 二〇二六 is not read as six.
   - **An absent field:** the renderer may write nothing, so the reader
     also tries matching nothing.

   Literal text matches as the renderer's space-collapsing sink leaves it.
   White space covers any run of it, and a separator (`,` `、` `،`) may be
   missing. A weekday name may come before or after the date.
3. **Resolution.** A complete match gives fields. The reader converts them
   to a fixed day and back, and keeps the day only if all of these hold:
   - the calendar's own fields for that day agree with everything the text
     wrote, including the era, the year, the month and its leap flag, the
     day, every extra field, and the kind of year a month name implies
     (*Adar II* only in a leap year);
   - the renderer, given those fields, would have taken the same template
     levels;
   - a weekday the text names is the day's.

   Where the text leaves something out, the reader fills it in and
   checks each option:
   - an implied era, `ad` or `am`;
   - a leap day;
   - a day that the extra fields determine (the Burmese and Thai lunar
     half and day);
   - a year written only through extra fields: an era's year that
     shifts by a constant, a cycle and a year of the cycle, as Meyer–Palmen
     and the Liberalia Triday write it, measured on three probe days
     centuries apart.
4. **Answer.** One day is the answer. Two or more days are
   `ambiguous`, and the two earliest are named. The reader also tries the
   day before and after when the text writes no day, and flips a flag the
   text does not write.

**Worked example.** 令和元年5月1日 in `ja`, calendar `japanese`:

- The entry states the date template `{year}{month}{day}`, the year
  template `{era}{year}年`, the first-year template `{era}元年` and the day
  template `{day}日`.
- The first-year template matches 令和 as the era `reiwa`, from `ja`'s
  era names, and 元年 as its literal, so the year is 1.
- The month label 5月, `ja`'s wide name for month 5, matches next.
- The day template reads the number 1 and the literal 日.
- The fields reiwa 1-5-1 convert to fixed day 737 180 and back to the
  same fields.
- The renderer, given them, writes the year with the first-year template,
  so the reading is kept: 1 May 2019.

令和1年5月1日 reads as the same day through the year template: the renderer
would not write it, but it says nothing the calendar contradicts.

### The refusals

| Refusal | Code | When | Example |
| --- | ---: | --- | --- |
| `empty` | 101 | the text is white space | |
| `not-recognised` | 102 | no template matches; the offset is where the furthest match stopped | *September 28, 2026 at noon* |
| `ambiguous` | 103 | two days read | 2026w39, a Stata week; 万延元年3月3日, since 1860 has a leap third month the Japanese templates write as the ordinary one |
| `two-digit-year` | 104 | the year has one or two digits, no sign and no era, the calendar's years have three digits or more on its sample day, and it has the same month and day a hundred or four hundred years on | *September 28, 26*; *September 28, 0026* reads as the year 26 |
| `year-not-written` | 105 | the year is not written, or only by a cycle that recurs, and two probe years give different days | 癸卯年闰二月初一; a Tzolkʼin day |
| `weekday-mismatch` | 106 | the weekday is not the day's | *Tuesday, September 28, 2026* |
| the calendar's | 1–11 | the fields read are not a day of the calendar | *February 30, 2026*, `day-out-of-range` |

An era name that two of a calendar's eras share — 洪武 is both `hongwu` and
`hongwu-1402` in `chinese-regnal`'s table — is tried as each. It is
refused as `ambiguous` only if both read as a day. The calendar is named
by the caller, so a name another calendar also uses (*AD* in `julian` and
`gregory`, 民國 only in `roc`) is not ambiguous at all.

## What is carried

- Every registered calendar, in every carried locale and under `native`.
  The locale is resolved as a rendered cell resolves it
  (`label::locale_for`), so what `hc_describe_day` writes, `hc_parse_date`
  reads.
- The lenient readings UTS #35 lists:
  - every width and both contexts;
  - either case;
  - an abbreviation with its period where the data has none (*Sep.*);
  - extra or missing white space.
- Latin digits in a locale with its own. In a locale written in Han
  characters, Han numerals, positional Han digits, 元 for the first year
  of an era, and the Chinese day names 初一 … 三十 for a lunisolar
  calendar whose data names none.
- A weekday before or after the date, which must be the day's.

Not carried:

- Unicode normalisation: the reader compares code points, so a
  decomposed *ü* does not match a composed one. This needs tables the
  workspace does not have (policy §9).
- Prefix matches of names (*Sept*, *Se*).
- Dates in a pattern other than the locale's own, such as *09/05/02*.
  UTS #35 resolves these by heuristics, and the reader does not guess.
- Times of day and zones.
- The Chinese calendar's year in Chinese, Japanese and Korean, which the
  templates write only by its stem and branch: `year-not-written`.

## Accuracy

`crates/hyper-calendar/tests/written_dates.rs` formats and reads back every
registered calendar. Each is formatted in every carried locale, the root
and `native`, on its sample days: the starts of 1 CE, of the Gregorian
reform, of 1900, of 1970 and of 2026, 28 September 2026 and 1 January
2100, where the calendar converts them, and its first and last days.

In a release build the sweep reads 204 calendars × 55 locale settings, on
1 647 calendar-days, which is 90 585 texts:

| Outcome | Texts |
| --- | ---: |
| Read back as the day written | 80 694 |
| `year-not-written`: cycles that recur, see below | 6 650 |
| `missing-field`: the 819-day count's station is not written | 1 155 |
| `two-digit-year`: years 0–99, on the calendars' first days | 1 092 |
| `ambiguous`: see below | 829 |
| `year-out-of-range`: the calendar does not convert its own last day's fields back | 165 |
| Read as a wrong day | 0 |

The `year-not-written` cases are all cycles that recur:

- the Akan, tonalpohualli, Pawukon, pasaran, Haabʼ, Tzolkʼin and Calendar
  Round counts;
- the year bearers of `mixtec-year` and `zapotec-yza`;
- the sexagenary day;
- the sexagenary year of `chinese`, `dangi` and `vietnamese` in the eight
  locales that write the year by it.

The `ambiguous` cases are all texts that name more than one day:

- `stata-week`, a week;
- `burmese`, a late Tagu written as the early one;
- `fasli-bombay` and `sur-san`, a doubled 3 June written as the ordinary
  one;
- `tibetan-bhutan`, a doubled lunar day;
- `tibetan-tsurphu`, a leap month that Tibetan writes as the ordinary
  one.

The test lists each of these calendars with its refusal and reason. It
fails on any other refusal, on a wrong day, and in a release build on a
listed refusal the sweep no longer meets.

The `year-out-of-range` cases are one day each, on the last day of
`hindu-lunar-purnimanta`, `odia-anka` and `saptarshi`. Each calendar
refuses to convert its own fields for that day back to it. That is a
limitation of those calendars, not of the reader.

A debug build reads every sample day in the calendar's own language, and
each other locale on one day of every fifth pairing of calendar and locale,
staggered: 3 847 texts, about seven seconds. The anchors are the dates of
this document and of the calendars' own system documents, all read in
either build:

- 令和元年5月1日; 嘉永三年一月一日, an era only the calendar's own table names;
  明治元年9月8日, 23 October 1868 ([japanese-eras.md](japanese-eras.md));
- 광무 1년 8월 14일 ([east-asian-eras.md](east-asian-eras.md));
- 康熙五十二年十一月初一 and 康熙52年十一月1日;
- 民國115年9月28日, *28 Eylül 2026*, ٢٨ سبتمبر ٢٠٢٦ and २८ सप्टेंबर, २०२६;
- *1 Adar II 5784*, with *1 Adar II 5785* refused, since 5785 is a common
  year.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [uts35-dates-48] | "Parsing Dates and Times": numeric and symbolic fields, symbols first, names at every width with and without an abbreviation marker, case and normalisation variants, the warning about narrow names; `yy` as the two low-order digits of the year | Yes, 2026-09-28, the section in the release-48 source of the specification (`docs/ldml/tr35-dates.md` of `unicode-org/cldr`) |
| The system documents of the calendars | The dated examples used as anchors: [japanese-eras.md](japanese-eras.md) for 万延元年3月3日 and 明治元年9月8日, [east-asian-eras.md](east-asian-eras.md) for 光武元年8月14日 | Their own sources, as those documents record |

The templates and names the reader walks are `hc-i18n`'s, sourced where
they are stated; this document adds no vocabulary.

## Code

- `hc-format`: `label::parse_date`, `label::DateRefusal` and
  `label::ParsedDate`, in `crates/hc-format/src/label/read.rs`, beside the
  renderer in `label.rs` whose templates and name writers it reuses.
- `hc-calendar`: `Calendar::era_code` and `DynCalendar::era_code`,
  implemented by `japanese` and its variants, `chinese-regnal`,
  `korean-regnal`, `meyer-palmen` and the Javanese calendars.
- `hc-i18n`: `names::for_each_era_name`.
- The facade: `lines::parse_date`, the export `hc_parse_date` in the
  `calendars` layer, and the binding's `parseDate`.
- Tests:
  - `crates/hyper-calendar/tests/written_dates.rs`, the sweep, the anchors
    and one test for each refusal;
  - `crates/hyper-calendar/tests/vocabulary.rs`, which holds `era_code` to
    every era a calendar names itself;
  - `lines`' `a_described_date_reads_back_as_its_line`;
  - the boundary tests in `hyper-calendar-wasm`, `hyper-calendar-ffi` and
    `js/hyper-calendar.test.js`.
