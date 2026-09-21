# 0003 — Exact `Duration`, with deep time in a separate type

**Status:** Accepted

## Context

The brief asks for both attosecond-scale civil precision and cosmological
spans, including Planck time at 5.39×10⁻⁴⁴ s. No single exact integer
representation covers both without absurd width, and the Planck-scale value is
not exact anyway — it is derived from measured constants.

## Decision

`hc-core::Duration` is `i128` seconds plus `u64` attoseconds, stored as a floor
decomposition. It is exact, and it is the only duration type used anywhere in
the civil, astronomical and relativistic paths.

Anything below an attosecond or above the range where "a count of seconds" is a
sensible answer lives in `hc-deep-time`, as a magnitude with explicit
significant figures and uncertainty.

## Consequences

**Good.** Civil arithmetic is exact and has no floating-point drift. The `i128`
second count spans about 4×10²⁰ times the age of the universe, so overflow is
theoretical. Values that are not exactly known are *typed* as not exactly
known, so precision cannot be silently manufactured.

**Costs.** Two duration-like concepts instead of one, and a conversion between
them that is lossy in the deep-time direction by construction. The floor
decomposition is slightly surprising (`−0.5 s` is `(−1, 5×10¹⁷)`) but makes
`Ord` a plain tuple comparison that agrees with arithmetic.

## Alternatives rejected

- **`f64` seconds throughout.** Loses microsecond resolution around the present
  day at the scale of a billion seconds, which is unacceptable for a library
  whose point is precision.
- **A decimal mantissa-and-exponent type throughout.** Covers the whole range,
  but makes ordinary civil arithmetic inexact and slow for no benefit to the
  99% case.
