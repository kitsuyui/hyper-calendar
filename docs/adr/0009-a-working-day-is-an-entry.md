# 0009 — A weekend day made a working day is an entry, not a weekend rule

**Status:** Accepted

## Context

China's State Council publishes an arrangement for each coming year. It
extends the holidays with weekdays off, and it pays for them by making
weekend days working days. In 2024 the Spring Festival ran from Saturday
10 to Saturday 17 February, and Sunday 4 and Sunday 18 February were worked.
Taiwan's 調整上班日, Vietnam's *làm bù* and Russia's transfers of days off
work the same way.

Until then the engine had one way for a day to be worked: it wasn't on the
weekend and no day-off entry fell on it. A worked Sunday had no way to say
so, and business-day arithmetic across a Chinese holiday was wrong in both
directions: it missed the weekdays off, and it skipped the Sundays worked.

There were two ways to carry the worked day:

- **As a weekend policy.** `WeekendPolicy` already varies by year, so a
  year's policy could list its exceptions. But the exceptions belong to one
  festival's arrangement, not to the weekend law. A weekend policy is a rule
  over years; this is a list of dates.
- **As an entry.** It has a date, a name and a source, like a holiday, and
  it can be evaluated, included, bounded by years and tabulated like one.

## Decision

A weekend day made a working day is a `Kind::Workday` entry, built with
`HolidayRule::workday`. `Kind::is_day_off` is false for it.
`HolidayCalendar::is_business_day` treats it as a working day even on the
weekend, and `HolidayCalendar::is_designated_workday` says whether a day is
one.

A table that includes another lends only its days off, so a working day
stays in its own table: an exchange that closes on China's holidays stays
closed on the Sunday China works.

China's arrangements are data, per [ADR 0007](0007-sets-the-world-can-extend-are-data.md).
The notices for 2008 to 2026 are tabulated, together with the three that
changed a year after its arrangement. A year outside the table is a gap, per
[ADR 0006](0006-refuse-to-extrapolate.md): the arrangement for 2027 has not
been published, and guessing it would be wrong for certain.

## Consequences

- Business-day arithmetic across a Chinese holiday is right for the
  tabulated years, and a year past them is reported rather than guessed.
- A calendar's entries now include days that are not days off. Any caller
  that treated "has an entry" as "is a day off" was already wrong for
  observances, and `is_day_off` and `is_holiday` remain the questions to
  ask.
- The FFI and WebAssembly kind strings gain `workday`.
- Russia's transfer decrees for 2013 to 2027 use it for their working
  Saturdays. Taiwan and Vietnam can carry their worked days the same way,
  once their acts are read.
