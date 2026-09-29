# 0014 — A municipality is a region within its subdivision

**Status:** Accepted

## Context

A subdivision's days are rules of its country's table scoped to its
ISO 3166-2 code ([ADR 0011](0011-a-day-for-one-group-is-a-scoped-rule.md)
set the same shape for groups of people), and the Japanese prefectures'
days were carried that way, `JP-01` to `JP-47`. The level below has days
of its own too. A city sets a 市民の日 or a 市制記念日 by ordinance, as a
prefecture sets a 県民の日, and some go further. Under 地方自治法 Article
4-2, paragraph 3, a local government's 休日条例 may make a day of special
historical meaning a day its offices close. The city's board of education
may list the day among its schools' 休業日, as the prefectures' boards
list theirs. Other countries have the same level. El Salvador's Código de
Trabajo gives 3 and 5 August "en la ciudad de San Salvador". Italy's D.P.R.
792/1985 names 29 June for the comune di Roma. Both tables had to scope
these days to the wider unit, or leave them out.

ISO 3166-2 stops at the first or second level of a country, and no
international standard codes municipalities. The table therefore needs
three things: a code for a municipality, a way to ask for one, and a way
to say whether its days were read. The options were these:

- **A third independent scope beside the region and the group.** Its code
  would be the municipality's alone. But a city is a place, and it lies in
  exactly one prefecture. An independent field would let a caller ask for
  Kawasaki in Tokyo. It would also make the caller name the prefecture a
  second time, and every boundary signature would gain an argument.
- **Scoping to the subdivision**, as El Salvador's and Italy's days are.
  That is wider than the instrument. It would say that 開港記念日 is kept in
  Kawasaki because Yokohama keeps it.
- **A foreign coding such as UN/LOCODE.** That standard codes places of
  trade and transport, not local governments. A LOCODE names a port or a
  station, not the area an ordinance governs, and it does not nest under
  ISO 3166-2.

## Decision

A municipality is a region: its code is its subdivision's ISO 3166-2 code,
a hyphen, and its code within that subdivision in the country's own
standard. A region scoped to a subdivision includes each municipality
within it.

- For Japan the local part is the three-digit 市区町村コード of JIS X 0402,
  the 全国地方公共団体コード without the prefecture's two digits (which
  are the ISO 3166-2:JP number) and without the check digit. 川崎市 is
  14130 with check digit 5, so its code is `JP-14-130`. 札幌市 is
  `JP-01-100`. Another country's local part is decided when its first
  municipality is carried, by the same rule: the national standard's code
  of the municipality within the subdivision.
- An ISO 3166-2 code has no second hyphen, so the parent of a code is
  everything before its last hyphen when that part still has a hyphen
  (`hc_holiday::rule::region_parent`). `region_within` tells whether a
  code is a region or lies within it, matched in either case as every
  identifier is.
- A rule's `regions` and `except_regions` may name municipalities. Asked
  for `JP-14-130`, a table applies the rules scoped to `JP-14-130` and the
  ones scoped to `JP-14`, and it drops the ones excepted from either.
  Asked for `JP-14`, it has none of Kawasaki's days, just as a table
  asked for no region has none of a prefecture's.
- A municipality is read when the table's rules name it, or when
  `Subdivisions::Read` lists it, and when its subdivision is read too
  ([ADR 0013](0013-a-year-the-sources-do-not-reach-is-a-gap.md)). Reading
  a prefecture's instruments says nothing of its cities. So a city that
  no rule names and no list gives keeps its prefecture's days, and its
  own days are the gap `UNREAD_SUBDIVISION` in each year asked for.
- At the boundary nothing new is added. The `region` argument takes a
  municipality's code. `hc_holiday_tables` lists the municipalities among
  a table's regions. `hc_holidays_on` writes a municipality's own lines
  with its code after its subdivision's lines, less what the subdivision
  already has. A year's line of `hc_holidays_in_year` asked for a
  municipality carries the widest region whose own entry it is: `JP-11`
  for 県民の日 asked in さいたま市, and `JP-11-100` for the city's own day.

## Consequences

- A caller who wants a city asks for one code and gets the country's,
  the prefecture's and the city's days. There is no new argument, and no
  signature changes.
- Every city not yet read is a gap, not an answer. A caller who asks for
  a Japanese town now gets its prefecture's days and a gap for its own.
  Before, the town's code was an unread region like any other, with the
  nationwide days alone and a gap.
- The local part is not self-describing: `JP-14-130` needs JIS X 0402 to
  be read. The code is checkable against the national list, which is the
  reason to take it from there rather than invent one.
- A ward of a designated city (横浜市鶴見区, 14101) has a code of its own
  in JIS X 0402 that does not nest under the city's, so this scheme
  cannot scope a day to a ward within a city. No instrument read sets a
  ward's day, and the question is left open until one does.
