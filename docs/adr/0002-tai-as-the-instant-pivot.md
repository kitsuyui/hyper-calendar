# 0002 — TAI as the instant pivot, and UTC is not a scale marker

**Status:** Accepted

## Context

TT, TCG, TDB, TCB and GPS are all uniform scales that differ from each other by
computable amounts. UTC is not: its seconds are SI seconds, but its labelling
occasionally repeats one. Most date libraries paper over this by pretending UTC
is uniform, which is why `23:59:60` is unrepresentable in most of them.

## Decision

`Instant<S>` is parameterised by a zero-sized `TimeScale` marker and converts
through TAI. UTC is deliberately **not** one of those markers. It is modelled
separately by `UtcInstant`, which carries an explicit `leap_second` flag, and by
`UnixTime`, which is named so that nobody mistakes POSIX time for elapsed time.

Offsets between coordinate scales are computed in `f64` and then *applied to*
the exact `Duration`, rather than converting the whole reading through `f64`.

## Consequences

**Good.** A TAI value cannot be silently used where a TT value is expected —
the type system stops it. `23:59:60` is nameable, so leap-second handling is a
feature rather than a workaround. The attosecond precision of a reading
survives a scale conversion, because only the small offset goes through
floating point.

**Costs.** Callers meet more types than in a library with one `DateTime`.
`Instant<S>` needs manual `Clone`/`Copy`/`PartialEq` impls because deriving
them would add spurious bounds on the marker. `AnyInstant` exists as the
scale-erased escape hatch for FFI and heterogeneous collections.

## Alternatives rejected

- **A runtime `scale: TimeScaleId` field.** Cheaper to write, but moves every
  mistake from compile time to run time, which is the opposite of what a
  correctness-focused library should do. Kept as `AnyInstant` for the cases
  that genuinely need it.
- **UTC as the canonical internal scale.** Makes every arithmetic operation
  depend on the leap-second table, including arithmetic on far-future or
  far-past values where the table says nothing.
