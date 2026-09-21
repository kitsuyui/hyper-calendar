# Architecture decision records

One file per decision that could reasonably have gone the other way. Each
records the context, the choice and what it costs — so that a future
contributor can tell whether the reasoning still holds rather than guessing at
intent.

| # | Decision | Status |
| --- | --- | --- |
| [0001](0001-rata-die-as-the-calendar-pivot.md) | Rata Die as the calendar pivot | Accepted |
| [0002](0002-tai-as-the-instant-pivot.md) | TAI as the instant pivot, and UTC is not a scale marker | Accepted |
| [0003](0003-exact-duration-and-separate-deep-time.md) | Exact `Duration`, with deep time in a separate type | Accepted |
| [0004](0004-one-crate-per-capability.md) | One crate per capability, behind a feature-gated facade | Accepted |
| [0005](0005-no-external-dependencies.md) | No external dependencies | Accepted |
| [0006](0006-refuse-to-extrapolate.md) | Refuse to extrapolate observational data | Accepted |
