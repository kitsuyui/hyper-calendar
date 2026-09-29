# Architecture decision records

One file per decision that could reasonably have gone the other way. Each
records the context, the choice and what it costs — so that a future
contributor can tell whether the reasoning still holds rather than guessing at
intent.

An ADR keeps its original text. When the code moves away from part of one,
a dated **Later changes** section at its end says what changed, and the
status says so. An ADR replaced outright would be marked Superseded, with a
link to the record that replaced it; none is, as of 2026-09-27. Each
record's status was last checked against the code on that date.

| # | Decision | Status |
| --- | --- | --- |
| [0001](0001-rata-die-as-the-calendar-pivot.md) | Rata Die as the calendar pivot | Accepted |
| [0002](0002-tai-as-the-instant-pivot.md) | TAI as the instant pivot, and UTC is not a scale marker | Accepted |
| [0003](0003-exact-duration-and-separate-deep-time.md) | Exact `Duration`, with deep time in a separate type | Accepted |
| [0004](0004-one-crate-per-capability.md) | One crate per capability, behind a feature-gated facade | Accepted |
| [0005](0005-no-external-dependencies.md) | No external dependencies | Accepted |
| [0006](0006-refuse-to-extrapolate.md) | Refuse to extrapolate observational data | Accepted; amended 2026-09-27 |
| [0007](0007-sets-the-world-can-extend-are-data.md) | Sets the world can extend are data; sets we define are enums | Accepted |
| [0008](0008-exchange-calendars-are-rule-sets.md) | Exchange calendars are rule sets keyed by Market Identifier Code | Accepted |
| [0009](0009-a-working-day-is-an-entry.md) | A weekend day made a working day is an entry, not a weekend rule | Accepted; updated 2026-09-27 |
| [0010](0010-a-government-office-day-off-is-its-own-kind.md) | A day off for a government's own offices is its own kind | Accepted |
| [0011](0011-a-day-for-one-group-is-a-scoped-rule.md) | A day for one group of people is a rule scoped to the group | Accepted |
| [0012](0012-a-half-day-off-is-its-own-kind.md) | A half day off is its own kind | Accepted |
| [0013](0013-a-year-the-sources-do-not-reach-is-a-gap.md) | A year the sources do not reach is a gap, and so is a subdivision they were not read for | Accepted |
| [0014](0014-a-municipality-is-a-region-within-its-subdivision.md) | A municipality is a region within its subdivision | Accepted |
