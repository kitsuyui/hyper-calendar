# hc-core

The time primitives every other crate uses and none may redefine.

## What it owns

| Module | What it is |
| --- | --- |
| `duration` | `Duration`: an exact span of SI seconds, `i128` whole seconds and `u64` attoseconds, of either sign. No floating point. |
| `scale` | `Instant<S>`: a reading on a uniform time scale, the scale being a zero-sized type parameter, so a TAI reading cannot be passed where a TT reading is expected. The scales are TAI, TT, TCG, TCB, TDB, GPS and UT1. |
| `leap` | The UTC leap-second table, as data. |
| `unix` | POSIX time (`UnixTime`), UTC with the leap second made explicit (`UtcInstant`), and the conversions between them and TAI under a `LeapPolicy`. |
| `epoch` | Well-known epochs as TAI readings: Unix, GPS, J2000, MJD, the Julian Day, Rata Die, Windows FILETIME, NTP, Core Foundation, and the TCG/TCB origin. |
| `math` | The floating-point functions a `no_std` build has to route somewhere. |
| `catalogue` | The `catalogue!` macro, which declares a table of named entries together with the tests every such table needs, and `catalogue_tests!`, which adds those tests to a table assembled by hand. |

## The lines it draws

- **No calendar lives here.** A calendar turns a day number into fields; that
  is `hc-calendar`'s job, and the day number itself is defined there.
- **UTC is not a time scale.** Converting between the scales here never moves
  an event; it only re-reads the clock, by an offset that is a fixed constant
  or a smooth model. UTC's labelling repeats or skips a second now and then,
  so it cannot be that, and it lives in `leap` and `unix` instead.
- **Everything that can be exact is exact.** Floating point appears only
  where the physics is itself a fitted model.

## Accuracy

| Claim | |
| --- | --- |
| `Duration` | exact to the attosecond over a range of about 5 × 10³⁰ years; arithmetic reports overflow rather than wrapping |
| TAI − TT | exactly 32.184 s |
| TAI − UTC, 1972 onward | exact, a whole number of seconds from the IANA `leap-seconds.list`, which mirrors IERS Bulletin C; the last entry is the leap second of 2017-01-01 |
| TAI − UTC, 1961 to 1971 | the official rate-offset coefficients of that era, in `RATE_ERA` |
| before 1961 | refused: UTC did not exist, and a conversion returns `BeforeModelStart` unless the caller opts into treating UTC as TAI |
| after the table's announced validity | `LeapPolicy::Strict` refuses with `AfterModelEnd`; `LeapPolicy::Extrapolate` holds the last published offset, and the caller has named the forecast by choosing it |
| `23:59:60` | representable: `UtcInstant` carries the leap-second flag that POSIX time cannot |
| TDB, UT1 | models, not constants. TDB follows a periodic series in `scale`. Real UT1 needs a caller-supplied `Ut1Offsets` series, which refuses to extrapolate outside its samples; the `Ut1` marker alone is a placeholder that returns the TAI reading unchanged |

The table's validity horizon is `leap::table_valid_until_unix`, the expiry
declared by the `leap-seconds.list` it was built from, and it is updated
together with the table and never past what the file itself claims. The 2022
CGPM resolution to retire the leap second by 2035 will change this table's
future, not its past.

## Feature flags

| Feature | Effect |
| --- | --- |
| `std` (default) | platform floating-point math; implies `alloc` |
| `alloc` | the parts that need an allocator |
| `libm` | portable software floating-point math for `no_std` targets |

A build with neither `std` nor `libm` is a compile error, deliberately: a
missing feature is a fact about the build, not about the input, and belongs
where the person choosing the features will see it.

## What this crate is not

Not a date library. It has no notion of a year, a month or a weekday, and
nothing in it knows what day it is. Every number in it is a count of SI
seconds from an epoch, or a table of when UTC decided to have one more.
