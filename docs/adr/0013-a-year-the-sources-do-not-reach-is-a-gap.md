# 0013 — A year the sources do not reach is a gap

**Status:** Accepted

## Context

A holiday rule had one pair of years, `valid_from` and `valid_until`, and
the engine skipped a rule silently in any year outside them. So
`.years(Some(2010), None)` said that the day did not exist before 2010,
whatever the reason the rule began there. Two different reasons had been
written the same way:

- **The day was established that year.** Ontario's Family Day was added
  by 2007, c. 16, in force for 2008; 2007 has no Family Day, and a source
  says so.
- **The sources read begin that year.** Employment New Zealand's list of
  anniversary days begins in 2010; the days are a century older, and what
  was kept in 2009 is not known.

To tell them apart, eleven modules had each written their own helper that
added a second rule, `Rule::UNREAD` over the earlier years, to report them
as a gap. Where an author forgot the helper, as New Zealand's, the Solomon
Islands' and Vanuatu's did, the unknown years looked like years without
the day. The same silence covered subdivisions: a region no rule was
scoped to answered with the nationwide days and no gap, as if its list
were complete, so an unread US state, a Mexican state, a New Zealand
region or an Andorran parish in a year not read looked settled. Policy §4
says a guess of absence is still a guess.

## Decision

A rule carries both facts, and the engine reports the difference.

- `valid_from` and `valid_until` are the day's **establishment** and
  abolition, which the rule's `source` or the table's `sources` give.
  Outside them the day is absent: that is an answer.
- `read_from` is the rule's **earliest supported year**, set by
  `HolidayRule::read_from`. Every earlier year that the establishment does
  not rule out is a gap, which the engine writes itself, with the rule's
  name and source. A day read in the law in force, with no source for the
  year it was set, has `read_from` and no `valid_from`; one whose
  establishment is known but whose early years were not read has both.
- `except_regions`, set by `HolidayRule::except_in`, is the subdivisions a
  nationwide rule does not apply in: a federal day a province's own law
  does not keep. Canada's federal days are the unscoped answer, for the
  employers the Canada Labour Code covers, and each province's list takes
  away the days its text leaves out.
- `RuleSet::subdivisions` says which subdivisions a table was read for.
  `Subdivisions::Undivided` is a table that has none — an exchange's, a
  tradition's. `Subdivisions::Read(list)` is a country's: the subdivisions
  its rules are scoped to or excepted from, and the listed ones, read and
  found to have no days of their own, are answered; any other region keeps
  the nationwide days and a gap, `UNREAD_SUBDIVISION`, in each year asked
  for.
- `Subdivisions::ReadFrom(list)` is `Read` with a first year for each
  listed subdivision: one read in the ordinances in force now, which give
  it no day of its own and say nothing of the regime before the one they
  began, is a gap before that year. Japan's twenty-six prefectures and six
  cities whose 休日条例 gives them none are read from the year of it, 1989
  to 1992. A region a rule is scoped to and `ReadFrom` does
  not list is read in every year, and the rule's own years say which it
  answers for.
- A rule counted from Easter is resolvable only where `Computus::easter`
  has an answer: 1583 to 4099 for the Gregorian computus, 326 to 4099 for
  the Julian. Outside them it is a gap, and so are the `Offset`, `Span`
  and `MovedByWeekday` rules built on it, as the rules of a calendar's
  range are.
- A gap two rules of one table write, with one identifier, is written
  once a year.
- The boundary refuses a region the table's country has no subdivision for
  (`US-ZZ`, `JP-99`, `JP garbage`, `JP-14-130-5`) as unknown, as it
  refuses a group that is no group. A subdivision that exists and was not
  read is the gap above; a typo does not look like one. The shape of a
  code is `hc_holiday::rule::is_region_code`, the subdivisions are
  `hc_i18n::place_names`' codes, the `subdivision-codes` feature the facade's
  `holiday` feature turns on, and the six outlying areas of the United States
  that ISO 3166-2 lists and CLDR does not, `ISO_SUBDIVISIONS_BEYOND_CLDR`; the
  facade joins them, so that every build of the layer answers one way.
- `Rule::NO_DAY` is a day a text read does not keep. With `read_from` and
  a region, it says that the province's text of 2026 leaves a federal day
  out and that the texts before it were not read.
- A year system of `hc-fiscal` carries the same two facts. `valid_from` is
  the year it was established, before which it is absent
  (`FiscalError::OutsideValidity`, the line status `outside-validity`);
  `read_from` is the first year the sources read reach, and `unread` the
  stretches after it that no source reaches. A label from the establishment to
  `read_from`, or in a stretch, is `FiscalError::NotRead` and the line status
  `gap`. A system whose establishment no source gives has no `valid_from`.
  The first year read is the first whole year of the earliest instrument,
  edition or page read.
- `hc_holidays_in_year` writes the year's gaps as `hc_holidays_on` does,
  and a gap line carries the rule's source.
- A day a gap leaves open is refused, not answered. `HolidayCalendar::day_off`
  and `business_day` say `Err(Unanswered::Gap)` for a day without an entry
  when a gap of a kind that stops work (or, for a weekend day, a work day)
  could fall on it, and the boundary writes `HC_ERR_NO_DATA`: a holiday the
  table could not place may be that very day, and `false` would be a guess.
  A day the table lists is answered whatever else is open; the arithmetic
  and `hc_holiday_next` and `hc_holiday_previous` refuse in the same way.
  A weekend law not read is `HC_ERR_OUT_OF_RANGE`, as
  ADR 0015 has it.
- **A gap carries the days it could fall on.** `Gap::window` is the first
  and the last day the holiday could fall on, both included, and only those
  days are refused; `None` leaves the whole year open. A rule whose shape the
  year can compute gives the days it would place, widened by seven days where
  the table has a substitution or a bridge policy; a table of announced
  dates that has run out gives the span of dates the day took in the years it
  lists; a rule read in no year, a calendar whose range ended and a
  subdivision not read have no window. `hc_holidays_in_year` writes the
  window in two columns after the bridge flag, and `hc_holidays_on` writes
  a gap on the days it could fall on and on no other.
- **A table says which years it answers for.** `RuleSet::coverage` gives,
  for the country and for each subdivision, the first year any rule is read
  for, the first year from which no rule is a gap for want of reading, the
  last year no announced list has run out in, whether some rule is read in no
  year, and the first year the weekend is read; `hc_holiday_coverage` writes
  them, with the rule that sets the later start.

## Consequences

- No module writes a gap helper of its own; a table states its years in
  the rule.
- More answers are gaps. A table whose earliest source is recent reports
  its earlier years as unknown — Switzerland's nationwide days before
  2026, whose Jura, Schwyz and Zurich laws were read only as they stand —
  and not the days of today for every year. A caller who
  wants the modern list for an old year can still read it from a later
  year; the library does not choose that reading for them.
- A region the table's sources were not read for is a gap, which is why
  the tables list their subdivisions read without days: Japan's prefectures
  with no ordinance of their own, Wyoming, Honiara.
- The unscoped Canadian answer is the federal one, and a province's answer
  may have fewer federal days than it.
- The establishment of most older rules is the table's own reading, not a
  separately cited instrument. A rule that is absent before a year no source
  sets is mended by one call, `read_from`, on that rule.
- A table whose sources are one recent text or list does not answer for the
  years before it. The tables of the Americas, Europe, Africa, the Middle East and Oceania each carry the first
  year their sources support, set on every rule that has none of its own by
  `countries::read_all`, and `tests/first_years.rs` and
  `tests/first_years_mea_oceania.rs` check that no table
  answers a year before it; `docs/systems/holiday-first-years.md` says which
  instrument or list each first year is. A rule whose own text is read from a
  later year than the table's sets its own `read_from`, as Venezuela's Ley de
  Fiestas Nacionales and San Marino's civil days do.
