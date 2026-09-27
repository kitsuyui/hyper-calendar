# The *Common Worship* calendar: ranks and transfers

Backs `hc-holiday`'s `common_worship` module and its tradition table
`common-worship`.

## What it is

*Common Worship* is the Church of England's modern liturgy. Its
"Rules to Order the Christian Year" rank the celebrations of the year and
say when one must give way to another and where it goes [cw-rules]. The
ranks named there, from the highest, are:

- **Principal Feasts**: Christmas Day, the Epiphany, the Presentation of
  Christ in the Temple, the Annunciation, Easter Day, Ascension Day,
  Pentecost, Trinity Sunday and All Saints' Day. "These days ... may not
  be displaced by any other celebration", except the Annunciation.
- **Principal Holy Days**: Ash Wednesday, Maundy Thursday and Good Friday,
  which may not be displaced either.
- **Festivals**: twenty-eight, from the Naming and Circumcision of Jesus on
  1 January to the Holy Innocents on 28 December, among them the Baptism
  of Christ and Christ the King, which are Sundays. They "are not usually
  displaced".
- **Lesser Festivals** and **Commemorations**, which the calendar lists
  and "the minister may be selective in".

The Rules also permit moves that a church may make or not: the Epiphany
to a Sunday, All Saints to a Sunday, the Blessed Virgin Mary from
15 August to 8 September, a Festival off an ordinary Sunday.

## How it works

The transfers the Rules require:

1. **The Annunciation** (25 March), "falling on a Sunday, is transferred
   to the Monday following or, falling between Palm Sunday and the Second
   Sunday of Easter inclusive, is transferred to the Monday after the
   Second Sunday of Easter".
2. **St Joseph** (19 March), in that fortnight, goes to the same Monday,
   "or, if the Annunciation has already been moved to that date, to the
   first available day thereafter".
3. **St George** (23 April) and **St Mark** (25 April), in that
   fortnight, go to the Monday after the Second Sunday of Easter; "If
   both fall in this period, St George's Day is transferred to the Monday
   and St Mark's Day to the Tuesday".
4. **A Festival on a Sunday of Advent, Lent or Eastertide** goes to the
   Monday: Festivals "falling on a Sunday are to be kept on that day or
   transferred to the Monday ... But a Festival may not be celebrated on
   Sundays in Advent, Lent or Eastertide".
5. **A Festival on a Principal Feast or Principal Holy Day** is
   "transferred to the first available day".
6. **No saint's day in Easter Week**: "no saint's day may be celebrated in
   Easter Week".

Which Festivals the calendar can bring into rules 4 and 5 follows from the
dates. 19 March is always in Lent, so St Joseph on a Sunday goes to the
Monday. St Andrew (30 November) on a Sunday is the First Sunday of Advent
and goes to 1 December. Philip and James (1 May), Matthias (14 May), the
Visit of the Blessed Virgin Mary (31 May) and Barnabas (11 June) can fall
on Ascension Day, on a Sunday of Eastertide, Pentecost among them, or on
Trinity Sunday; in each case the next day is a weekday that holds no other
Festival, so the first available day is the next day. No other Festival
can meet a Principal Feast or a Principal Holy Day.

**What the Rules leave open.** Rule 3 sends St George and St Mark to fixed
days without saying what happens when another Festival is already there,
and rule 6 forbids Easter Week without naming a day. The years are those
of an Easter on 17 April or from 22 to 25 April:

| Easter | St George | St Mark | Philip and James |
| --- | --- | --- | --- |
| 17 April | to Monday 25 April, which is St Mark's Day: open | open | from Sunday 1 May to Monday 2 May |
| 22 April | Monday 30 April | to Tuesday 1 May: open | open |
| 23 April | to Monday 1 May: open | Tuesday 2 May | open |
| 24 April | Monday 2 May | Tuesday 3 May | on the Second Sunday of Easter, whose Monday is St George's: open |
| 25 April | Monday 3 May | Tuesday 4 May | in Easter Week: open |

The library reports these as gaps and does not choose.

**Worked example: 2025.** Easter was 20 April. Palm Sunday was 13 April
and the Second Sunday of Easter 27 April.

1. The Annunciation, Tuesday 25 March, and St Joseph, Wednesday
   19 March, lie before Palm Sunday and stay.
2. St George, Wednesday 23 April, and St Mark, Friday 25 April, both lie
   in the fortnight. St George goes to Monday 28 April and St Mark to
   Tuesday 29 April.
3. Philip and James, Thursday 1 May, is a weekday of Eastertide and stays.
4. Matthias, Wednesday 14 May, stays. The Ascension is 29 May.
5. The Visit, Saturday 31 May, stays; Barnabas, Wednesday 11 June, stays.
6. St Andrew, Sunday 30 November, is the First Sunday of Advent and goes
   to Monday 1 December.

Full Fact, quoting the Church, reported St George's Day 2025 on Monday
28 April [fullfact-st-george-2025].

## What is carried

- **`common_worship::CELEBRATIONS`**: the nine Principal Feasts, three
  Principal Holy Days and twenty-eight Festivals, each with its rank and
  the rule for the day it is kept on after the required transfers.
- **`common-worship`**, the same as a rule set, every entry religious and
  none a day off; the open years above are reported as gaps
  (`Rule::Unsettled`).

Not carried:

- **The permitted moves**, listed above, and a Festival moved for Corpus
  Christi, which a church may keep as a Festival or not.
- **The Lesser Festivals and Commemorations.** The Church's calendar page
  lists them; the minister chooses which are kept, and a Lesser Festival
  that meets a higher day "is normally omitted", which is a choice too.
- **Patronal and Dedication Festivals**, which are each church's own.
- **The Book of Common Prayer's calendar.**
- **The years before *Common Worship*.** The Rules are applied to every
  year the Gregorian computus gives; when they were authorized was not
  read.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| The Visit of the Blessed Virgin Mary on Monday 1 June 2026, Trinity Sunday having been 31 May; Barnabas on 11 June and Holy Cross Day, Matthew and Michael and All Angels on their days in 2026, as the Church's *Daily Prayer* prints them [cofe-daily-prayer-2026] | `the_common_worship_transfers_are_the_ones_the_church_printed` | 6 of 6 |
| St George on Monday 28 April 2025 [fullfact-st-george-2025] | the same | yes |
| The Annunciation of 2024 and 2008, St Joseph of 2008, Philip and James of 2008 and St Andrew of 2025, from the rules | the same | yes |
| The open years of 1962, 2000, 2011, 2022 and 2038 reported as gaps | `the_years_the_common_worship_rules_leave_open_are_gaps` | yes |
| 1900–2100: no Festival shares its day with another celebration, none is kept on a Sunday of Advent, Lent or Eastertide or in Easter Week, and every celebration is kept once a year or reported as a gap | `no_festival_is_kept_where_the_common_worship_rules_forbid_it` | yes |

The *Daily Prayer* pages of May 2026 and of the years before were no
longer online when read, so the transfers of the paschal fortnight rest
on the rules and one press report.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [cw-rules] | The ranks, the celebrations of each, the transfers and the Table of Transferences | Yes, 2026-09-26 and 2026-09-27 |
| [cofe-daily-prayer-2026] | The days of June and September 2026 | Yes, 2026-09-27 |
| [fullfact-st-george-2025] | St George's Day 2025 | Yes, 2026-09-27 (press) |

## Code

`crates/hc-holiday/src/common_worship.rs`: `CELEBRATIONS`, `Rank`,
`COMMON_WORSHIP` and the transfer functions. Anchors in
`crates/hc-holiday/tests/traditions.rs`:
`the_common_worship_transfers_are_the_ones_the_church_printed`,
`the_years_the_common_worship_rules_leave_open_are_gaps`,
`no_festival_is_kept_where_the_common_worship_rules_forbid_it`. The
WebAssembly and C export `hc_common_worship_on` writes the rank of each
celebration kept on a day, from `hyper_calendar::holiday_lines`; the
celebrations and the gaps are lines of `hc_holidays_on`.
