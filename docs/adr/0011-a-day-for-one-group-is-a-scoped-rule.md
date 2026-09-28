# 0011 — A day for one group of people is a rule scoped to the group

**Status:** Accepted

## Context

Some statutes give a day to part of the population and to no one else.
China's 全国年节及纪念日放假办法, Article 3, lists 部分公民放假的节日及纪念日:
women have half of 8 March off, youth of fourteen and over half of 4 May,
children under fourteen the whole of 1 June, and active servicemen half of
1 August. Article 6 says such a day is not made up when it falls on a
weekend. Taiwan's 紀念日及節日實施條例, Article 6, leaves Police Day, Fire
Fighters' Day, Armed Forces Day and Coast Guard Day to each service's
authority, and lets each indigenous person choose three days for their
people's ceremonies. Laos gives the Lao Women's Union's day to women
employees; Nepal's Home Ministry gives Teej to women employees, days to the
Kirat, Muslim and Sikh faithful and to the Newar community, and a day to
employees with disabilities; Bangladesh's notifications list optional
holidays for each faith.

None of these is a day off for the country, and none is a mere
observance: for the people it names, the instrument stops work, or half of
it. The table had two ways to say who a day is for, and neither fits.

- **`Kind`** says what a day does, not for whom. A `Kind::Women` would be
  one variant per group, would multiply against the kinds a group's day can
  be — a whole day, a half day, a gap — and could not name a group and a
  region together.
- **A region** is a place. A group is not one, and a day may be for a group
  in one place: Nepal's Jitiya is for the women employees who keep it, in
  the districts that keep it.
- **Leaving the days out**, as the tables did, answers "no day" for a
  woman in China on 8 March, which is wrong.

## Decision

A day for one group is a rule of its country's table given to that group
alone, as a subdivision's day is a rule scoped to its ISO 3166-2 code, and
the two scopes are independent.

- `HolidayRule::groups` lists the groups a rule is given to; empty means
  everyone. `HolidayRule::for_groups` sets it, as `in_regions` sets
  `regions`.
- A group is a `hc_holiday::group::Group`, an identifier and an English
  name, in the table `GROUPS` that `hc_core::catalogue!` declares. The
  identifiers are lower-case kebab, like every other identifier, and
  matched by `hc_core::catalogue::matches`. The set is data, since the
  world adds a group whenever a statute names one
  ([ADR 0007](0007-sets-the-world-can-extend-are-data.md)). A group names
  who the day is for and nothing more: China's youth are those of fourteen
  and over, and the rule's name and source say so. Its name in a language
  other than English is `hc-i18n`'s `holiday_groups`, carried only from an
  instrument written in that language.
- A calendar is evaluated in a `Scope`, a region and a group, either of
  which may be absent. `HolidayCalendar::scoped` and its companions take
  one, and every constructor that takes a region is the scope of that
  region for everyone. A scope with no group answers for everyone's days
  alone, never the union of every group's, as a scope with no region
  answers for the nationwide days alone.
- At the boundary, `hc_holidays_in_year` and `hc_holiday_is_day_off` take
  a `group` argument after the region, null or empty for everyone.
  `hc_holidays_in_year` and `hc_holidays_on` write the group whose own
  entry a line is in a column of its own, after the region, and
  `hc_holidays_on` writes each group's own lines after the subdivisions'.
  `hc_holiday_tables` lists a table's groups and their names in the
  locale.
- A day whose group is named but whose effect is not known — a Taiwanese
  service's day, which its authority's rule sets — is a rule of the group
  that reports a gap, not a day left out.

## Consequences

- The C and WebAssembly signatures of `hc_holidays_in_year` and
  `hc_holiday_is_day_off` gain the group; the JavaScript methods take it
  as an optional last argument, so a caller that passes none gets what it
  got before. The lines gain a column.
- A caller who asks for a group gets everyone's days and the group's own;
  one who wants a person's calendar asks for each group the person
  belongs to, one at a time, as for a region. The library does not
  combine groups.
- A group's day is kept by the table's own policies. China makes up no day
  for some citizens under Article 6, and its table has no substitution
  rule in any case; a table whose substitution law reaches a group's day
  would move it as it moves any other.
- `Holiday` gains `groups`, as it has `regions`.
