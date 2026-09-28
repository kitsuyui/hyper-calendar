# 0012 — A half day off is its own kind

**Status:** Accepted

## Context

China's Article 3 gives three of its four days for some citizens as half
days: 妇女放假半天, 14周岁以上的青年放假半天, 现役军人放假半天. The engine's
kinds could not say so.

- `Public` and `Bank` stop work for the day, and `is_day_off` is true for
  both. Business-day arithmetic for women in China would skip 8 March, and
  a deadline counted over it would be a day late, though the afternoon or
  the morning is worked.
- `Observance` says that nothing stops, which is untrue of a half day off.
- The exchanges carry an early close as an `Observance` named "Early
  close, …", because the exchange's own calendar calls it a trading day.
  A statute that gives half a day off is saying more than that the day is
  noted.

## Decision

Half a day off is `Kind::HalfDay`, written `half-day` at the boundary.
`Kind::is_day_off` is false for it, so business-day arithmetic counts the
day as a working day, as it counts an early close. The kind says that work
stops for part of the day; which part, where a statute says, is in the
rule's source.

## Consequences

- The FFI and WebAssembly kind strings gain `half-day`, and the `.d.ts`
  `HolidayKind` with them.
- A caller who wants to treat a half day as a day off, or to schedule
  around its morning, can select the kind; the library does not choose
  that reading for them.
- The exchanges' early closes stay observances, as their calendars call
  them trading days. Another statute's half days, Nepal's "आधा दिन" among
  them, can use the kind once an item is read that gives one.
