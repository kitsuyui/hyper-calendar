# 0010 — A day off for a government's own offices is its own kind

**Status:** Accepted

## Context

Japan's prefectures set days of their own by ordinance, and the ordinances
do different things with them. Most make a 県民の日 a day for the people
to think about their prefecture and for the prefecture to hold events,
and the prefectural schools close on it under a rule of the board of
education. Okinawa's 慰霊の日 is more: its 休日条例 makes 23 June a
県の休日, a day off for the prefecture's own offices, as 地方自治法
Article 4-2 paragraph 3 allows for a day of "特別な歴史的、社会的意義".
On that day the prefecture's offices are closed and a deadline for an
application to them that falls on it moves to the next day (paragraph 4).
The national government is open, a bank's holidays are the national ones of
銀行法施行令 Article 5, and no private employer is bound.

Of the engine's six kinds, none says that:

- `Public` and `Bank` stop work, and `is_day_off` is true for both.
  Business-day arithmetic in Okinawa would then skip 23 June, and a
  settlement date computed over it would be a day late.
- `Observance`, `Religious` and `School` do not stop work, and `Workday`
  is a weekend day worked. `School` says
  who closes, and a 県民の日 on which the prefectural schools close is one
  (Bulgaria's 1 November already is). `Observance` would say that nothing
  closes, which is untrue of 慰霊の日.

## Decision

A day off for the offices of the government that sets it, and for no one
else, is `Kind::Government`, written `government` at the boundary.
`Kind::is_day_off` is false for it, so business-day arithmetic works
through it, as the banks and the national government do. The kind, the
entry's name and its `source` say what the day is; the name stays the
day's own name.

A prefectural day is a rule of `JAPAN`'s table scoped to its ISO 3166-2
code (`JP-47`), like every other subdivision's day, and its kind is what its
instrument makes it: `Government` for a 県の休日, `School` where the
prefectural schools close, `Observance` where the ordinance sets a day and
nothing closes.

## Consequences

- The FFI and WebAssembly kind strings gain `government`.
- A caller who wants the calendar of a prefecture's offices, rather than
  of its businesses, can treat `government` entries of that region as days
  off; the library does not choose that reading for them.
- Another country's regional or municipal administrative closure can use
  the kind on the same terms, once its instrument is read.
