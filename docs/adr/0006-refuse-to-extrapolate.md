# 0006 — Refuse to extrapolate observational data

**Status:** Accepted

## Context

Several inputs this library needs are *measured and announced*, not derived:

- Leap seconds, announced by the IERS about six months ahead.
- `DUT1` (`UT1 − UTC`), published in IERS Bulletins A and B.
- ΔT before the atomic era, reconstructed from eclipse records.
- Holiday dates in countries where they are set by annual decree.
- Hijri month starts, which depend on crescent visibility decided per country.

A library can either invent an answer past the data, or say it does not know.

## Decision

The default is to say it does not know.

- `LeapPolicy::Strict` returns `AfterModelEnd` past the announced validity of
  the leap-second table. `LeapPolicy::Extrapolate` exists, but the caller has
  to ask for it.
- `Ut1Offsets` interpolates inside the supplied series and returns
  `BeforeModelStart` or `AfterModelEnd` outside it. UT1 without a series is
  the `Ut1` marker's ΔT model, a different type with its accuracy stated.
- Calendars whose rules are astronomical carry `is_astronomical: true`, and
  those with bounded data carry `earliest`/`latest` in their metadata.
- Holidays that are announced rather than computed are flagged `Approximate`.

## Consequences

**Good.** A caller who gets a value can trust it. A caller who gets an error
learns something true about the world, which is more useful than a plausible
number. Scheduling across the boundary of announced data becomes a visible
decision rather than a silent assumption.

**Costs.** More `Result` in the API than a library that always answers. Code
that schedules into the future has to choose a policy explicitly. That is the
intended cost: the choice exists whether or not the API admits it.
