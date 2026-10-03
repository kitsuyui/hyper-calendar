# Holiday identifiers: how a line of one export is joined to a line of another

Backs `hc-holiday`'s `id` module, `HolidayRule::id` and `Holiday::id`, and
the identifier column that `hc_holidays_in_year`, `hc_holidays_on`,
`hc_holidays_on_in` and `hc_common_worship_on` write. No calendar
identifier is registered: an identifier labels a holiday of a table, and
the table is named by its own.

## What it is

A holiday of a table has a name for people, its English name and its
local name, and had no name for programs. Two exports that describe the
same entry therefore joined on the English name: the *Common Worship*
ranks of `hc_common_worship_on` found their entry in `hc_holidays_on` by
comparing the title with the name, and `hc_holidays_on_in` found a day's
name in a locale by the table's code and the English name. A name is a
poor key. A statute's name is reworded, a state spells *Mothers' Day*
and *Mother's Day* of one observance, a name carries an apostrophe or a
macron its source printed in one way or another, and an English name that
a page shows is one a page may want to change.

An identifier is a holiday's name for programs: lower-case ASCII,
hyphenated, one to a holiday within its table. It is written as the last
column of the holiday lines, and joins a line to any other line of the
same table that is about the same holiday.

## How it works

The identifier of a rule is the one it sets, `HolidayRule::with_id`, and
otherwise the kebab-case of its English name: the ASCII letters and
digits in lower case, a Latin letter with a diacritic as its base letter
(`ß` as `ss`, `æ` as `ae`), an apostrophe within a word dropped, and every
other run of characters one hyphen, none at either end. So:

| English name | Identifier |
| --- | --- |
| `New Year's Day` | `new-years-day` |
| `Tōkanya` | `tokanya` |
| `Kurban Bayramı` | `kurban-bayrami` |
| `Prešeren Day` | `preseren-day` |
| `Nayrouz (New Year)` | `nayrouz-new-year` |
| `Mothers' Day`, `Mother's Day`, `Mothers Day` | `mothers-day` |

The fold is what makes spelling variants one holiday. Two rules of one
table with the same English name are one holiday already — a statute's
successive forms, a nationwide rule and a regional one — and share the
identifier; a rule that sets its own identifier shares it with no other
name, which a test holds (`crates/hc-holiday/tests/ids.rs`). A rule sets
one only where the fold would be wrong: where two days of one table would
fold to the same identifier, or, following `docs/policy.md` §5, where two
readings of one day are registered as separate conventions and the
English name would not tell them apart.

A worked example. The *Common Worship* table writes `Christmas Day` on
25 December 2025. Its rule is the first line of the table in
`common_worship::CELEBRATIONS`, which sets the identifier `christmas-day`
beside the title. `hc_holidays_on` for that day writes the line of the
table `common-worship` with `christmas-day` as its twelfth column;
`hc_common_worship_on` writes `Christmas Day`, `principal-feast`,
`Principal Feast`, `christmas-day`. The page joins the two on the last
cell, and the same identifier is the tenth cell of the year's line in
`hc_holidays_in_year`.

A gap carries the identifier of its rule, so that a page can match a gap
to the entry it replaces in another year; the one gap that is no rule's,
a subdivision whose sources were not read, is `unread-subdivision`, and a
year whose weekend law in the region was not read is `unread-weekend`
([ADR 0015](../adr/0015-a-region-may-keep-a-weekend-of-its-own.md)).

## What is carried

- **`hc_holiday::id`**: `HolidayId`, the identifier a rule has, which
  compares, hashes and orders by what it renders to, and `Kebab`, the
  fold.
- **Every rule of every table**: the 9 554 rules of the 311 tables need not
  set one; the `common-worship` table sets all forty, since the ranks are
  joined on them.
- **The columns.** `hc_holidays_in_year`, cells 10 (`id`) and 11
  (`source`); `hc_holidays_on`, cell 12; `hc_holidays_on_in`, cell 14 after
  the two names; `hc_common_worship_on`, cell 4. Each was the last of its
  line, or the last but one, so that a parser reading the cells it knew
  reads what it did. Since audit 10 d3 the three lines of holidays end with
  one more cell, `bridged` (`1` for an entry a bridge policy made, Japan's
  国民の休日, else `0`), after the instrument in `hc_holidays_in_year`
  (cell 12) and after the identifier in `hc_holidays_on` (cell 13) and
  `hc_holidays_on_in` (cell 15).
- **The names in a locale** are keyed by identifier: `hc-i18n`'s
  `holiday_names` carries the seven Coptic feasts by `nayrouz-new-year`,
  `feast-of-the-cross` and so on.

Not carried: an identifier across tables. `new-years-day` of `JP` and of
`CN` are two holidays, as their tables are; a page joins on the table's
code and the identifier.

## Accuracy

The identifier is a name, and has no claim to accuracy but uniqueness and
stability. Uniqueness within a table holds by the fold except where
spelling variants are one holiday, which they are in every case the tests
meet: each of the 51 groups of English names that fold together, all in India's
and the United States' tables, is one observance spelled several ways
(`Fathers' Day`, `Fathers Day`, `Father's Day`). Stability: an identifier
derived from a name changes when the name does, which is why a rule that
a page keys on may set it. The other rules' names were not each read for
whether two names that differ by more than the fold are one day.

## Sources

None beyond the tables' own: an identifier is the library's. The
conventions it follows are `docs/policy.md` §5, for the separately named
readings, and `hc_core::catalogue::matches`, for matching an identifier
in either case with white space around it.

## Code

`crates/hc-holiday/src/id.rs`: `HolidayId`, `Kebab`. `rule.rs`:
`HolidayRule::id`, `with_id`. `engine.rs`: `Holiday::id`, `Gap::id` and
`Gap::kind`, `UNREAD_SUBDIVISION_ID`. `common_worship.rs`:
`Celebration::id`. `crates/hyper-calendar/src/holiday_lines.rs`: the
columns and the join in `common_worship_lines` and `holidays_on_in_lines`.
Anchors: `crates/hc-holiday/tests/ids.rs`
(`the_kebab_case_of_a_name_is_as_documented`,
`every_rule_has_a_well_formed_identifier`,
`within_a_table_an_identifier_names_one_holiday`),
`a_common_worship_line_joins_its_holidays_on_entry_by_identifier` and
`a_year_s_lines_carry_an_identifier_and_a_source_and_are_filtered_by_kind`
in `holiday_lines.rs`, and `crates/hyper-calendar/tests/holiday_names.rs`.
