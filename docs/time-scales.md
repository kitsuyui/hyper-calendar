# Time scales

Requirement 1 of the brief: support the international specifications for time
and dates completely. That starts with being precise about what a "time" is.

## Two different questions

"What time is it" hides two questions that behave differently:

1. **How much time has elapsed?** Answered by a uniform scale whose seconds are
   all the same length. TAI, TT, GPS.
2. **What should the clock on the wall say?** Answered by a scale tied to
   something human or planetary — the Earth's rotation, a civil convention.
   UT1, UTC, local time.

Mixing them is the single most common source of date bugs. `hyper-calendar`
keeps them in different types: `Instant<S>` for the first, `UtcInstant` and
`CivilDateTime` for the second.

## The uniform scales

All of these are implemented in `hc-core::scale`. Each is a zero-sized marker
type, so `Instant<Tai>` and `Instant<Tt>` are different types and cannot be
confused.

| Scale | What it is | Relation |
| --- | --- | --- |
| **TAI** | International Atomic Time. The weighted average of some 450 atomic clocks, realising proper time on the rotating geoid. The canonical scale of this library. | — |
| **TT** | Terrestrial Time. The theoretical ideal that TAI approximates; the independent variable of geocentric ephemerides. | `TT = TAI + 32.184 s`, exactly and by definition |
| **TCG** | Geocentric Coordinate Time. TT without the gravitational rescaling, so it runs slightly fast. | `TCG − TT = L_G/(1−L_G) · (TT − T₀)`, `L_G = 6.969290134×10⁻¹⁰` |
| **TDB** | Barycentric Dynamical Time. TT plus the periodic terms from the Earth's orbital motion; the independent variable of solar-system ephemerides. | `TDB − TT` is a truncated Fairhead–Bretagnon series, at most about 1.7 ms |
| **TCB** | Barycentric Coordinate Time. | `TCB − TDB` is linear in `L_B = 1.550519768×10⁻⁸` plus `TDB₀` |
| **GPS** | GPS system time. | `GPS = TAI − 19 s`, exactly, frozen at the 1980 epoch |
| **UT1** | Universal Time — the Earth's actual rotation angle. In `hc-astro`, not `hc-core`. | Observational; modelled as `TT − ΔT`, or read from a DUT1 series. See below. |

`T₀` throughout is `1977-01-01T00:00:00 TAI`, the defining origin of TCG and
TCB.

### Why the offsets are computed the way they are

TCG, TDB and TCB differ from TT by a *small* amount against the reading
itself: TDB by at most about 1.7 ms, TCG by about a second and TCB by about
twenty-four seconds since 1977. So the library computes that small difference
in `f64` and then applies it to the exact `Duration`, rather than converting
the whole reading to `f64` and back. An `f64` near 1.7 billion seconds
resolves only about 240 ns; an offset of a few seconds converts with
femtosecond error, which is nothing that matters.

## UTC is not one of them

UTC's seconds are SI seconds, but its *labelling* occasionally repeats one. It
is therefore not a constant offset from TAI and cannot be a `TimeScale` marker
without lying.

Instead, `hc-core::unix` models it explicitly:

- `UtcInstant { unix_seconds, leap_second, subsec_attos }` — the `leap_second`
  flag names the inserted `23:59:60`, so it is representable rather than being
  something callers work around.
- `UnixTime` — POSIX `time_t`. It counts *nominal* 86 400-second days, so a
  leap second and the second after it share a timestamp. That ambiguity is what
  POSIX time is, and the type is named so nobody mistakes it for elapsed time.

### Smeared time is not UTC

Some operators spread a leap second over the hours around it, so that their
clocks never read `23:59:60`. What such a clock reads is neither UTC nor
TAI, and a client cannot tell from the reading alone; RFC 8633, the NTP best
current practice, says clients must not mix smeared and unsmeared servers and
that public servers must not smear. The library has no type for smeared
time and does not label it UTC, for the same reason it refuses to
extrapolate a leap second ([ADR 0006](adr/0006-refuse-to-extrapolate.md)):
a smeared reading passed off as UTC would defeat the leap-second table and
the flag that make `23:59:60` nameable. A caller with a smeared clock
converts it to true UTC on their side, where the smear's shape is known.

### Three eras of UTC

| Era | `TAI − UTC` |
| --- | --- |
| Before 1961-01-01 | UTC did not exist. `LeapPolicy::Strict` refuses; `Extrapolate` returns 0 by convention. |
| 1961-01-01 to 1971-12-31 | A piecewise-linear rate offset, `offset + (MJD − origin) × drift`. Fractional, and the second itself was a different length. |
| 1972-01-01 onward | A whole number of seconds, stepped by announced leap seconds. 10 s at the start, 37 s since 2017-01-01. |

### The end of the table is a real boundary

Leap seconds are *announced* by the IERS about six months ahead, not predicted.
Past the announced validity, `LeapPolicy::Strict` returns `AfterModelEnd`.
`LeapPolicy::Extrapolate` will hold the last value, but the caller has to ask
for it — because that answer is a forecast, and the difference matters to
anyone scheduling across the boundary.

(The 2022 CGPM resolution to abandon the leap second by 2035 will change this
table's future, not its past. The table is data; when the decision lands, the
data changes and the code does not.)

## UT1 and DUT1

UT1 tracks the Earth's rotation, which is not predictable: tidal friction,
core-mantle coupling, glacial rebound and atmospheric angular momentum all move
it. `UT1 − UTC` (DUT1) is measured by VLBI and published in IERS Bulletins A
and B.

The library therefore does **not** ship a DUT1 table baked in. Both readings
of UT1 live in `hc-astro::ut1`:

- `Ut1Offsets` takes the series the caller trusts and reads
  `UT1 = UTC + DUT1`. Between samples it interpolates `UT1 − TAI`, which is
  continuous, rather than DUT1, which steps a second at every leap second.
  Outside the series it returns `BeforeModelStart` or `AfterModelEnd` rather
  than extrapolating.
- The `Ut1` scale marker needs no series. It reads `UT1 = TT − ΔT` from the
  Espenak–Meeus model below. Against the IERS EOP 20 C04 series it is within
  about 0.1 s from 1972 through 2005, where the model was fitted, and falls
  behind after: 1.4 s on 2016-01-01, 6.0 s on 2026-01-01, because ΔT has
  grown more slowly since 2006 than the model's forecast segment assumed.

Since 1972 the IERS has kept `|UT1 − UTC| < 0.9 s` (ITU-R TF.460-6), so
inside the leap-second table UTC itself is a closer reading of UT1 than the
model has been since 2011; only a DUT1 series does better.

## ΔT

`hc-astro` needs `TT − UT1` (ΔT) to place historical astronomical events on a
civil calendar. Before atomic clocks this is reconstructed from eclipse
records, and the uncertainty grows fast: a few seconds in 1900, minutes in
1000 CE, hours in 1000 BCE. `hc-astro` uses the Espenak–Meeus polynomial fits.
It does not attach an uncertainty to each value, but `time::is_fitted_year`
reports whether a year lies inside the span the fits cover (−500 to +2150) or
in the parabolic extrapolation beyond it.

This is why a Chinese lunisolar date computed for 500 CE can differ by a day
from what was actually proclaimed: the calculation is right and the Earth's
rotation is the unknown.

## Epochs

`hc-core::epoch` collects the origins that other systems chose, all expressed
as TAI readings so nothing has to rediscover a magic number:

| Epoch | Origin |
| --- | --- |
| `unix` | 1970-01-01T00:00:00Z (`TAI − UTC` was 8.000082 s) |
| `gps` | 1980-01-06T00:00:00Z |
| `j2000` | 2000-01-01T12:00:00 TT |
| `tcg-tcb-origin` | 1977-01-01T00:00:00 TAI |
| `mjd` | 1858-11-17T00:00:00 UT |
| `julian-day` | −4712-01-01T12:00:00 UT, proleptic Julian |
| `rata-die` | 0001-01-01, proleptic Gregorian |
| `windows-filetime` | 1601-01-01T00:00:00Z |
| `ntp` | 1900-01-01T00:00:00Z |
| `core-foundation` | 2001-01-01T00:00:00Z |

## Duration

`Duration` is `i128` seconds plus `u64` attoseconds, stored as a floor
decomposition so that `−0.5 s` is `(−1, 5×10¹⁷)` and `Ord` agrees with
arithmetic.

- The `i128` second count spans about 4×10²⁰ times the age of the universe, so
  nothing overflows in practice.
- 10⁻¹⁸ s resolves anything an optical clock can measure.
- Spans shorter than an attosecond — Planck time — are not exact quantities in
  any physical sense, so they live in `hc-deep-time` as magnitudes with stated
  uncertainty rather than being forced into an integer type. See
  [scales-beyond-seconds.md](scales-beyond-seconds.md).
