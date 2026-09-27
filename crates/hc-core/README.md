# hc-core

Exact instants and durations on the uniform time scales (TAI, GPS time
and the rest), UTC with its leap seconds, the well-known epochs, and the
timestamp formats software writes from them. Every other crate in the
workspace uses these primitives, and none redefines them.

It depends on no other workspace crate. Its one external dependency is
`libm`, which is optional (see [Feature flags](#feature-flags)).

```rust
use hc_core::unix::{LeapPolicy, UnixTime, tai_from_unix};

// 2017-01-01 00:00:00 UTC, the second after the last leap second.
let tai = tai_from_unix(UnixTime::from_seconds(1_483_228_800), LeapPolicy::Strict)?;
assert_eq!(tai.since_epoch().whole_seconds(), 1_483_228_800 + 37);
# Ok::<(), hc_core::TimeError>(())
```

## What it owns

| Module | What it is |
| --- | --- |
| `duration` | `Duration`: an exact span of SI seconds, `i128` whole seconds and `u64` attoseconds, of either sign. No floating point. It also carries what Python's `timedelta` needs: weeks, floor division and a remainder with the divisor's sign (`checked_div_floor`, `checked_rem`, `checked_div_rem`), the normalised `(days, seconds, attoseconds)` split, and `days_and_clock`, the `-1 day, 19:00:00` string form. |
| `scale` | `Instant<S>`: a reading on a uniform time scale, the scale being a zero-sized type parameter, so a TAI reading cannot be passed where a TT reading is expected. The scales are TAI (International Atomic Time), TT (Terrestrial Time), TCG (Geocentric Coordinate Time), TCB (Barycentric Coordinate Time), TDB (Barycentric Dynamical Time), and the GNSS times GPS, Galileo, BeiDou and NavIC; UT1, the Earth's rotation read as a time, is measured rather than defined, so it is in `hc-astro` beside the ΔT (TT − UT1) model it is computed from. |
| `leap` | The UTC leap-second table, as data, and `end_of_day_step`, the leap second that ends a given UTC day. |
| `tt_bipm` | TT(BIPM), the BIPM's yearly realisations of Terrestrial Time, from the table of TT(BIPMxx) − TAI − 32.184 s the caller supplies, interpolated inside it and refused outside it. |
| `unix` | POSIX time (`UnixTime`), UTC with the leap second made explicit (`UtcInstant`), and the conversions between them and TAI under a `LeapPolicy`. |
| `gnss` | The GNSS week numbers — GPS legacy and CNAV, Galileo, BeiDou, NavIC — with the time of week and rollover resolution against a reference the caller supplies, and GLONASS time, UTC(SU) + 3 h with its leap seconds and its four-year intervals. See `docs/systems/gnss-time.md`. |
| `epoch` | Well-known epochs as TAI readings: Unix, GPS, Galileo, BeiDou, NavIC, GLONASS, J2000, MJD, the Julian Day, Rata Die, .NET's ticks, the CCSDS Unsegmented Code's 1958 TAI, Windows FILETIME, NTP, Core Foundation, the TCG/TCB origin, the UUID's 1582 and SAS's and Stata's 1960. |
| `tai64` | Bernstein's TAI64, TAI64N and TAI64NA labels: 2⁶² + TAI seconds since 1970 TAI in 8, 12 or 16 big-endian bytes, encoded from and decoded to `Instant<Tai>`; TAI64NA is exact to the attosecond. Also `tai64-posix-plus-10`, the 2⁶² + 10 + POSIX seconds that daemontools' `tai64n` writes on an ordinary clock, to and from `UnixTime`. See `docs/time-scales.md`. |
| `ntp` | NTP's 128-bit date with its era number and era offset, and the 64-bit timestamp resolved into the era within 2³¹ s of a reference time, across the 2036 wrap. See `docs/systems/binary-timestamps.md`. |
| `uuid` | The 60-bit timestamp of UUID versions 1 and 6, 100 ns from 1582-10-15, to and from `UnixTime`, and both octet layouts. See `docs/systems/binary-timestamps.md`. |
| `dotnet` | .NET's `DateTime.Ticks`, 100 ns from 0001-01-01 without leap seconds, with the `DateTimeKind` that says which zone they count in; only a `Utc` value converts to `UnixTime`. See `docs/systems/binary-timestamps.md`. |
| `ccsds` | The CCSDS time codes' P-field, and the Unsegmented Code (TAI seconds and binary fractions from 1958, or from an agency's epoch) and the Day Segmented Code (UTC days from 1958 and the millisecond of the day, to 86 400 999 on a leap-second day), both ways. The calendar codes are `hc-format::ccsds`. See `docs/systems/ccsds-time-codes.md`. |
| `sas_stata` | SAS datetimes and Stata's `%tc` and `%tC` from 1960, the last counting leap seconds as UTC does. See `docs/systems/statistical-software-dates.md`. |
| `epoch_notation` | Julian and Besselian epochs, J2000.0 and B1950.0, from and to `Instant<Tt>`. |
| `internet_time` | Swatch Internet Time, @000 to @999 from POSIX time on Biel Mean Time, UTC+1. |
| `math` | The floating-point functions a `no_std` build has to route somewhere. |
| `memo` | A memo of pure functions for the length of one call: inside a `scope`, `cached` computes each keyed question once and hands the same value back. It is emptied when the outermost scope ends, and computes every time outside a scope, without `std` or without the `memo` feature, so it changes how long a call takes and never what it returns. |
| `catalogue` | The `catalogue!` macro, which declares a table of named entries together with the tests every such table needs, and `catalogue_tests!`, which adds those tests to a table assembled by hand. |

## The lines it draws

- **No calendar lives here.** A calendar turns a day number into fields; that
  is `hc-calendar`'s job, and the day number itself is defined there.
- **UTC is not a time scale.** Converting between the scales here never moves
  an event; it only re-reads the clock, by an offset that is a fixed constant
  or a smooth model. UTC's labels skip or repeat a second at each leap
  second, so no such offset re-reads it, and it lives in `leap` and `unix`.
- **Everything that can be exact is exact.** Floating point appears only
  where the physics is itself a fitted model.

## Accuracy

| Claim | |
| --- | --- |
| `Duration` | exact to the attosecond over a range of about 5 × 10³⁰ years; arithmetic reports overflow rather than wrapping. The operators `+ - * / %` panic on overflow or a zero divisor, and each has a `checked_*` twin ([policy §8](../../docs/policy.md)) |
| TAI − TT | exactly 32.184 s, for TT(TAI); TT(BIPMxx) is as good as the series supplied, linearly interpolated between its ten-day samples |
| TAI − GPS, GST, NavIC; TAI − BDT | 19 s and 33 s, exact by convention; the systems' realisations are steered to national UTCs and differ from these by sub-microsecond residuals that are not carried |
| TAI − UTC, 1972 onward | exact, a whole number of seconds from the IANA `leap-seconds.list`, which mirrors IERS Bulletin C; the last entry is the leap second of 2017-01-01 |
| TAI − UTC, 1961 to 1971 | the official rate-offset coefficients of that era, in `RATE_ERA` |
| before 1961 | refused: UTC did not exist, and a conversion returns `BeforeModelStart` unless the caller opts into treating UTC as TAI |
| Julian epoch | exact in definition; an `f64` year, about 10⁻¹³ of a year (3 µs) near the present |
| Besselian epoch | ERFA's `eraEpb` constants, on TT for TDB; within 10⁻¹⁰ of a year of SOFA's example |
| NTP, UUID, .NET, SAS and `%tc` counts | exact labels of 86 400-second days; `%tC` exact from the leap-second table, refused past it under `LeapPolicy::Strict` |
| CCSDS CUC and CDS | exact; a CUC fraction finer than the attosecond, eight octets or more, is rounded up to it; a CDS leap second is checked against the table and refused past it |
| after the table's announced validity | `LeapPolicy::Strict` refuses with `AfterModelEnd`; `LeapPolicy::Extrapolate` holds the last published offset, and the caller has named the forecast by choosing it |
| `23:59:60` | representable: `UtcInstant` carries the leap-second flag that POSIX time cannot |
| TDB | a model, not a constant: the truncated Fairhead–Bretagnon series in `scale`, to about 30 µs over 1980–2100 |

The table's validity horizon is `leap::table_valid_until_unix`, the expiry
declared by the `leap-seconds.list` it was built from, and it is updated
together with the table and never past what the file itself claims. The
CGPM resolved in 2022 to stop inserting leap seconds by 2035. That changes
this table's future, not its past.

## Feature flags

| Feature | Effect |
| --- | --- |
| `std` (default) | platform floating-point math; implies `alloc` |
| `alloc` | the parts that need an allocator |
| `libm` | portable software floating-point math for `no_std` targets |
| `memo` | with `std`, the per-call memo of the `memo` module: a caller opens a `memo::scope`, and the astronomy the calendar crates memoize — a sunrise of a day at a place, the conjunction after a moment — is computed once inside it; without it every call computes. The facade turns it on with its astronomical calendars, and `hyper_calendar::lines` opens a scope for each description of a day |

A build with neither `std` nor `libm` is a compile error, deliberately: a
missing feature is a fact about the build, not about the input, and belongs
where the person choosing the features will see it.

## What this crate is not

Not a date library. It has no notion of a year, a month or a weekday, and
it has no clock. Every number in it is a count of SI seconds from an epoch,
or an entry in the table of leap seconds.
