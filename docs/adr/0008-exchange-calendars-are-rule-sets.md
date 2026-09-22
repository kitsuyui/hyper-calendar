# 0008 — Exchange calendars are rule sets keyed by Market Identifier Code

**Status:** Accepted

## Context

A trading-day calendar answers the same question a national holiday table
does — is this day a working day, and if not, why — for a narrower
institution. The New York Stock Exchange closes on Good Friday, which no
United States statute makes a holiday, trades on Columbus Day and Veterans
Day, which the federal government does not, moves a Saturday holiday to the
Friday before except a New Year's Day, and closes on days no statute foresaw:
a hurricane, a day of mourning. It also closes early on three kinds of day,
which are trading days for every purpose that counts them.

The first draft was a type of its own, `TradingCalendar`, with a weekend, a
list of closed days and a list of early closes. Everything in it already
existed in `RuleSet`, and everything `RuleSet` can express — a rule valid
between two years, a substitution policy, a bridge — an exchange has needed at
some time.

## Decision

An exchange calendar is a `RuleSet`, in `hc_holiday::exchanges`, keyed by the
exchange's ISO 10383 Market Identifier Code and found by `by_code`. A closed
day is a `Kind::Public` entry, so `is_holiday` and business-day arithmetic
count it. An early close is a `Kind::Observance` whose name says so, so they
do not: the day is noted and traded on. Unscheduled closures a source records
are rules valid in their one year, as [ADR 0007](0007-sets-the-world-can-extend-are-data.md)
would have any set the world extends.

The exchange's own published calendar is the source and the test: a table is
checked against every year the exchange has published, and carries a
closure only where a read source records it.

## Consequences

- No new engine. An exchange calendar drives the same `HolidayCalendar` and
  the same business-day functions as a country, and gains every rule kind the
  engine grows.
- A partial day has no kind of its own, and a caller that wants the hours
  reads the name. This record first said that a second kind of partial day
  — a late open, a lunch-hour close — would make `Kind` grow; the second
  kind arrived with B3's Ash Wednesday, which opens at 1:00 p.m., and it
  did not need to. An early close and a late open are alike a day the
  exchange trades on and business-day arithmetic must count, and the name
  already says which; a kind would say nothing the name does not. Every
  partial day is an observance whose name begins "Early close", "Half
  trading day" or "Late open", and the tests check that convention.
- The code is the MIC and not a country: an exchange is not its country's
  calendar, and a country has more than one exchange.
