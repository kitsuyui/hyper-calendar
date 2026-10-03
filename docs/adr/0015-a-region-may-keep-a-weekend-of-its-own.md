# 0015 — A region may keep a weekend of its own

**Status:** Accepted

## Context

A table had one weekend law, `RuleSet::weekend`, a list of
`WeekendPolicy` values that differ only in the years they are in force,
and one substitution law, `RuleSet::substitution`, whose `trigger` is the
weekdays a holiday is moved off. Both were the country's. Some countries
have a region whose law sets another weekend.

- **Malaysia.** The Holidays Act 1951 defines the weekly holiday as
  Sunday or, in the states where Friday is observed, Friday. Kedah,
  Kelantan and Terengganu keep Friday and Saturday. Johor kept Saturday
  and Sunday from 1994, Friday and Saturday from 1 January 2014 by Sultan
  Ibrahim's decree, and Saturday and Sunday again from 1 January 2025.
  The Act moves a holiday that falls on the weekly holiday to the day
  after, so that a Friday holiday in Kedah goes to the Sunday, past the
  Saturday that is off already, and a Saturday holiday in Kelantan and
  Terengganu goes to the Sunday. The table's own comment named Johor,
  Kedah, Kelantan and Terengganu as the Friday–Saturday states and said
  that a weekend policy, having no region, could not scope them. A table
  that gives the country's Saturday–Sunday weekend answers wrongly for
  four states: `is_weekend`, the business-day arithmetic, and where a
  weekend holiday is moved to.
- **The United Arab Emirates.** The federal weekend has been
  Saturday–Sunday since 1 January 2022. The Government of Sharjah's
  offices keep Friday, Saturday and Sunday from the same day, a
  four-day week.
- **Other countries.** Every other table's weekend is a country's. The
  sources read give no other region a weekend of its own that a law or an
  authority sets; see
  [docs/systems/regional-weekends.md](../systems/regional-weekends.md) for
  what was looked at and not found.

The options were these:

- **A table for each region.** Malaysia's Kedah would be a second
  `RuleSet`, with every federal holiday copied. That is the duplication
  `includes` exists to avoid, and it would put the region in the table
  code, where a caller passes a region as an argument of the same table.
- **A parameter of the engine**, a caller-supplied weekend. Policy §5
  says a convention that an authority has settled gets a name, not a
  parameter. A state's weekend is the state's law, and a caller who asks
  for Kedah should get Kedah's without knowing the question.
- **Scoping the policy to the region**, as a holiday rule is
  ([ADR 0011](0011-a-day-for-one-group-is-a-scoped-rule.md),
  [ADR 0014](0014-a-municipality-is-a-region-within-its-subdivision.md)).

## Decision

A weekend policy and a substitution policy are scoped to regions like a
holiday rule, with the same codes.

- `WeekendPolicy::regions` and `SubstitutionPolicy::regions` list the
  subdivisions, as ISO 3166-2 codes, whose law the policy is; empty means
  the whole table. A municipality's code lies within its subdivision's
  ([ADR 0014](0014-a-municipality-is-a-region-within-its-subdivision.md)).
- A region's policy is used in place of the table's, not added to it.
  `RuleSet::weekend_in(region, day)` takes the policies in force on the
  day whose regions the asked region is or lies within, and the nearest
  wins, the longest code, so a municipality's own law beats its
  subdivision's. A region with no policy of its own on that day is
  answered by the table's, and so is a request for no region.
  `RuleSet::weekend_on(day)` is the table's own, as it was.
  `RuleSet::substitution_in_region(year, region)` chooses a substitution
  policy in the same way, and `substitution_in(year)` is the table's.
- `HolidayCalendar::is_weekend`, `is_business_day`, `add_business_days`
  and `business_days_between` use the calendar's own region, and a
  holiday moved off a weekend is moved by the region's substitution policy.
  Nothing is added to the boundary's signatures: the `region` argument
  that already selects a subdivision's holidays selects its weekend.
- A substitute may be kept off days besides the trigger. A state whose
  weekend is Friday and Saturday moves a Friday holiday to the Sunday, so
  its policy has `trigger: [Friday]` and `avoid: [Saturday]`, the day the
  substitute may not land on although a holiday on it is not moved.
  `SubstitutionPolicy::avoid` is empty for every policy that existed.
- **A weekend whose law was not read is a gap.** A `WeekendPolicy` with
  no `days` says that the region's weekend law in those years was not
  read, and that is not a weekend of no days. The engine reports the gap,
  `UNREAD_WEEKEND`, in each year asked for in which such a day lies;
  `weekend_in` answers `None` for the day; `is_weekend` says no and
  `weekend_is_read` says that the question has no answer; and
  `add_business_days` and `business_days_between` answer `None` for a
  walk that reaches the day, as they do for a walk that leaves the years
  evaluated. The table's own policy for the whole country has none, so a
  request for no region is never a gap for it.
- **Reading, as for the holidays.** A region whose weekend is carried is
  not thereby a region whose holidays were read:
  `RuleSet::reads_region` is unchanged, and Kedah's own days are still the
  gap `UNREAD_SUBDIVISION` until a source is read for them.
- **At the boundary** the weekend is column 14 of `hc_holiday_tables`,
  which lists a table's weekend laws, each as the days, the first and last
  day in force and the regions, written out in
  [docs/systems/regional-weekends.md](../systems/regional-weekends.md).
  The JavaScript binding reads it into `HolidayTable.weekend`.

## Consequences

- A caller who asks for Kedah gets a Friday and Saturday weekend and a
  Friday holiday on the Sunday, with no new argument. A caller who asks
  for Malaysia without a region gets the country's, which is what
  `RuleSet::weekend_on` was.
- Every `WeekendPolicy` and `SubstitutionPolicy` literal names its
  `regions`, and a substitution policy its `avoid`; an empty list is the
  whole table and no extra day.
- A region's weekend is the law its source states for the offices it
  governs: Sharjah's is the emirate government's, not the private
  sector's, which the federal Labour Law governs. A table that wants the
  weekend of both has to say which it carries; this one carries the
  government's.
- The line shape changes: `hc_holiday_tables` has a weekend column, its last (column 14).
- Years before the earliest a source reaches are reported, not guessed;
  Kedah, Kelantan, Terengganu and Perlis before 25 November 2013, and
  Johor to 1994, are gaps.
- A region whose weekend shifts within a year is no problem: the
  policy carries the day it takes effect, as the country's do.
