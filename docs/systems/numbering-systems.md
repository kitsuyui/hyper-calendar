# Numbering systems: digits, Han numerals, and which one a locale writes

Backs `hc-i18n`'s `numbering` module: `NumberingSystem` (`from_id`,
`for_locale`, `alternative_for_locale`, `id`, `is_algorithmic`, `digits`,
`write_integer`, `format_integer`, `parse_integer`, `writes_char`,
`continues_into`), the table `numbering::ALL` of 22 systems and `LATN`, the
Japanese era year (`write_japanese_era_year`, `parse_japanese_era_year`,
`FIRST_YEAR_MARKER`), the two tables of CLDR 48 that choose a system for a
locale (`data::DEFAULT_NUMBERING` and `data::OTHER_NUMBERING`), and the
exports `hc_numbering_systems`, `hc_format_number` and `hc_parse_number`
(`hyper_calendar::i18n_lines`) and column 13 of `hc_locale_info`. The Hebrew
numerals (`hebr`) and the Greek ones (`grek` and `greklow`) are submodules with
documents of their own, [hebrew-numerals.md](hebrew-numerals.md) and
[greek-numerals.md](greek-numerals.md); this document says what they are
beside the others. No calendar identifier is registered: a numbering system
writes the integers of a date, it does not count days.

## What it is

A language writes the number 2026 with a script's own digits, with letters, or
with words for the places. UTS #35 Part 3 calls the first kind *numeric*: "a
decimal based system that uses a predefined set of digits", such as the
Western digits, Thai digits or Devanagari digits. The second and third it
calls *algorithmic*: "the proper formatting and presentation of a numeric
quantity is based on some algorithm or set of rules", Chinese numerals, Hebrew
numerals and Roman numerals among them, and the rules are written in RBNF, the
rule-based number format [uts35-v48]. CLDR 48's `numberingSystems.xml` lists
97 systems: 78 numeric and 19 algorithmic [cldr48-supplemental].

The identifier of a system is a BCP 47 `-u-nu-` value: `ar-u-nu-latn` asks for
Western digits in Arabic. A locale also has a default system, from its
`defaultNumberingSystem`, which differs between files of one language: `ar` has
Western digits and `ar-EG` and `ar-SA` have Arabic-Indic ones. It may have
other systems for particular uses: `native`, the system of its own script's
digits where the default is another, `traditional`, and `finance`
[uts35-v48].

## How it works

The module knows 22 of the 97 systems.

- **Positional systems (15).** Ten digit code points substituted one for one:
  `latn`, `arab` (Arabic-Indic), `arabext` (extended Arabic-Indic, for Persian
  and Urdu), `beng`, `deva`, `fullwide`, `guru`, `hanidec` (the Han digits
  〇一二三四五六七八九, digit by digit), `java`, `mlym`, `mymr`, `tamldec`,
  `telu`, `thai` and `tibt`. Writing is a lookup of each decimal digit and
  reading is its inverse, so reading rejects any digit that is not the
  system's and a value that does not fit an `i64`. A negative number gets an
  ASCII hyphen.
- **Han numerals (4).** `jpan`, `jpanfin`, `hans` and `hant` spell the number
  in groups of four digits, each group with its units 千 百 十 inside it and
  the group itself followed by 万 (萬 in the traditional and financial
  styles), 億 (亿 in simplified Chinese) or 兆, for 10⁴, 10⁸ and 10¹². The
  styles differ in glyphs and in two conventions about the digit one:
  Japanese drops the one before every unit (110 is 百十), Chinese drops it
  only when ten leads the whole number (15 is 十五 but 115 is 一百一十五), and
  Chinese marks an interior gap with a zero where Japanese does not (105 is
  一百〇五 in Chinese and 百五 in Japanese). The financial style (大字) never
  drops the one. The largest number written is 10¹⁶ − 1.
- **Letters (3).** `hebr`, `grek` and `greklow`: see their own documents.
- **Choosing a system for a locale.** `NumberingSystem::for_locale` takes the
  `-u-nu-` key if it names a system the module knows; else the default of the
  first locale of the fallback chain that has one: a regional file whose system
  is not its language's (`DEFAULT_NUMBERING`, 74 rows, `ar-SA`'s `arab` beside
  `ar`'s `latn`), else a carried locale's own; then `latn`. A key that names a
  system the module does not know is ignored, so `ar-EG-u-nu-klingon` is
  `arab`.
  `alternative_for_locale` is the other system a locale writes where this
  crate carries it (`OTHER_NUMBERING`, 54 rows): CLDR's `native` system where it
  is not the default, else its `traditional` one.

**A worked example.** 2026, by hand, in Japanese: one group of four digits, a
thousands digit 2, a hundreds digit 0, a tens digit 2 and a units digit 6. A
two in the thousands place is 二千; a zero in the hundreds place writes nothing
in Japanese (a Chinese style would write the gap); the tens digit is 二十; the
units digit is 六. So `jpan` writes 二千二十六, and `hanidec` writes 二〇二六, digit
by digit. The calls below were made on 2026-10-04.

| System | 2026 | 105 | 110 | 10 005 |
| --- | --- | --- | --- | --- |
| `latn` | 2026 | 105 | 110 | 10005 |
| `arab` | ٢٠٢٦ | ١٠٥ | ١١٠ | ١٠٠٠٥ |
| `deva` | २०२६ | १०५ | ११० | १०००५ |
| `thai` | ๒๐๒๖ | ๑๐๕ | ๑๑๐ | ๑๐๐๐๕ |
| `hanidec` | 二〇二六 | 一〇五 | 一一〇 | 一〇〇〇五 |
| `jpan` | 二千二十六 | 百五 | 百十 | 一万五 |
| `jpanfin` | 弐阡弐拾六 | 壱百伍 | 壱百壱拾 | 壱萬伍 |
| `hans` | 二千〇二十六 | 一百〇五 | 一百一十 | 一万〇五 |
| `hant` | 二千〇二十六 | 一百〇五 | 一百一十 | 一萬〇五 |

The facade: `hc_format_number("jpan", 2026)` is the line `二千二十六` and the
identifier; `hc_parse_number("hans", "二千零二十六")` and
`hc_parse_number("hans", "二千〇二十六")` are both 2026, since reading accepts
either zero; `hc_format_number("klingon", 1)` is `HC_ERR_UNKNOWN`, and
`hc_numbering_systems` lists the 22 identifiers with whether each is
algorithmic and its ten digits, zero first, for a positional one.

The numbering system a locale writes also decides the digits of every numeric
field of a date, which `date-patterns.md` and `locale-fallback.md` describe: the
default system of each regional file is the one `hc_locale_info` writes in its
column 13.

## What is carried

- The 22 systems above, with the default of each of the 65 locale entries: 55
  write `latn`, four (`fa`, `pa-Arab`, `ps`, `ur-IN`) `arabext`, three
  (`mr`, `ne`, `sa`) `deva`, and `ar-EG` `arab`, `bn` `beng` and `my` `mymr`.
  Seventeen entries whose default is `latn` have an alternative that is
  carried: `ar` (`arab`), `bo` (`tibt`), `he` (`hebr`), `hi` (`deva`), `ja`
  (`jpan`), `jv` (`java`), `ml` (`mlym`), `pa-Guru` (`guru`), `ta`
  (`tamldec`), `te` (`telu`), `th` (`thai`), `ur` (`arabext`), and `yue-Hans`,
  `yue-Hant`, `zh-Hans`, `zh-Hant` and `zh-Hant-HK` (`hanidec`).
- Japanese era years: 元 for the first year (令和元年) and the Han numeral
  otherwise; a year below 1 is refused.

Not carried:

- **75 of CLDR 48's 97 systems.** 63 numeric ones (`adlm`, `bali`, `gujr`,
  `khmr`, `knda`, `laoo`, `orya`, `sinh` and the rest, whose locales
  `hc-i18n` does not carry) and 12 algorithmic ones: `armn`, `armnlow`, `cyrl`,
  `ethi`, `geor`, `hanidays`, `hansfin`, `hantfin`, `jpanyear`, `roman`,
  `romanlow` and `taml`. Not yet done; a positional system is one digit table
  and one row.
- **10¹⁶ and above in the Han styles.** CLDR's rules go on with 京 for 10¹⁶; the
  module refuses a number that large (`NumberOutOfRange`). Not yet done.
- **Grouping separators, a decimal separator, a sign other than an ASCII
  hyphen, currency, spelled-out ordinals, and the counting-rod and `hanidays`
  systems.** A calendar field is an integer and the surrounding pattern is
  `hc-format`'s, so no date needs the first five. Not yet done.
- **The `finance` category and the financial Chinese systems `hansfin` and
  `hantfin`.** The only financial style carried is `jpanfin`.

## Accuracy

The positional systems are lookups and their digit tables are the ones of
`numberingSystems.xml`; the test `every_registered_system_is_reachable_by_its_identifier`
and the round-trip tests hold them. The algorithmic Han styles were measured
on 2026-10-04 against CLDR 48's rules: the `ja`, `zh` and `zh_Hant` rule sets
`spellout-cardinal` and `spellout-cardinal-financial` of `common/rbnf`
[cldr48-rbnf-han], evaluated by a transcription of the RBNF substitutions (the
quotient `←←`, the remainder `→→`, the optional `[…]`, and `=%name=`) written
for this measurement, over 20 195 numbers: every number from 0 to 20 000 and a
spread of numbers around each power of ten up to 10¹⁶ − 1.

| System | Differences from CLDR 48's rules |
| --- | --- |
| `jpan` | 0 of 20 195 |
| `jpanfin` | 18 337 of 20 195, all of two kinds: the crate writes 阡 for 千 (2026 is 弐阡弐拾六 where the rules give 弐千弐拾六), and 壱拾 where the rules write 拾 for a tens digit of one (10 is 壱拾 where the rules give 拾); the rules keep the one before 百 and 千 (壱百, 壱千) |
| `hans`, `hant` | 4 480 of 20 195: in 4 444 the zero is 〇 where the rules write 零 (2026 is 二千〇二十六 where the rules give 二千零二十六); in 36, all at 10⁸ and above with a gap in the middle (100 001 005, 100 010 005 and the like), the crate also places the filler differently (100 001 005 is 一亿一千〇五 where the rules give 一亿零一千零五) |

Reading accepts both 〇 and 零 in every Han style, so a numeral CLDR writes is
read; what the module writes for `hans`, `hant` and `jpanfin` is not always
what CLDR's rules write. Below 10⁸ the only difference of `hans` and `hant` is
the glyph of the zero, and every year of every calendar the crate carries is
below that. The module's own comment gives the reason for the financial one,
which is a convention of its own: 壱拾 cannot be altered by a pen stroke as 拾
can.

## Sources

Read directly as HTML or XML on 2026-10-04.

- [uts35-v48]: UTS #35 Part 3 (Numbers), version 48.2, "Numbering Systems"
  (the `numberingSystem` element and its attributes) and "Default Numbering
  System" and "Other Numbering Systems" (`native`, `traditional`, `finance`).
- [cldr48-supplemental]: `common/supplemental/numberingSystems.xml`, tag
  `release-48`: the 97 systems, their types and, for the algorithmic ones, the
  rule set each names.
- [cldr48-rbnf-han]: `common/rbnf/ja.xml`, `zh.xml` and `zh_Hant.xml`, tag
  `release-48`, the rule sets named above. The Hebrew and Greek rules are
  [cldr48-rbnf], read for their own documents.
- The `-u-nu-` extension and the BCP 47 identifiers of the systems: UTS #35 Part
  1, as [uts35-v48] records its reading.

## Code

`crates/hc-i18n/src/numbering.rs` (with `numbering/hebrew.rs` and
`numbering/greek.rs`), the tables in `data/cldr48_locales.rs`
(`DEFAULT_NUMBERING`, `OTHER_NUMBERING`, generated by
`scripts/locales-cldr.py`), and the boundary in
`crates/hyper-calendar/src/i18n_lines.rs` (`format_number_line`,
`parse_number`, `numbering_systems_lines`, `locale_info_line`).

The tests that anchor it, in `numbering.rs`:
`a_regional_file_s_numbering_system_is_its_own`,
`the_entries_numbering_systems_are_cldr_s`,
`every_registered_system_is_reachable_by_its_identifier`,
`the_identifier_table_is_sorted_and_unique`,
`positional_systems_substitute_digit_for_digit`,
`positional_systems_round_trip_across_four_digits`,
`a_positional_parse_rejects_foreign_digits_and_overflow`,
`japanese_han_numerals_omit_the_one_before_a_unit`,
`chinese_han_numerals_keep_the_one_and_mark_interior_gaps`,
`financial_han_numerals_never_omit_the_one`,
`han_numerals_round_trip_over_a_wide_range`,
`han_numerals_refuse_magnitudes_they_have_no_unit_for`,
`a_han_parse_accepts_either_spelling_of_zero_and_rejects_junk`,
`the_first_year_of_a_japanese_era_is_written_gannen`,
`a_locale_picks_its_numbering_system_from_the_extension_first` and
`digit_tables_are_exposed_only_for_positional_systems`; and in
`i18n_lines.rs` `a_number_goes_into_a_system_and_back`.
