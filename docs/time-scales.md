# Time scales

This document describes the time scales, epochs and timestamp formats the
library carries: what each one is, how they relate, and where each lives in
the code. Calendars are in [calendars.md](calendars.md).

## Two different questions

"What time is it" hides two questions that behave differently:

1. **How much time has elapsed?** Answered by a uniform scale whose seconds are
   all the same length. TAI, TT, GPS.
2. **What should the clock on the wall say?** Answered by a scale tied to
   something human or planetary — the Earth's rotation, a civil convention.
   UT1, UTC, local time.

`hyper-calendar` keeps the two in different types: `Instant<S>` for the
first, `UtcInstant` and `CivilDateTime` for the second.

## The uniform scales

All of these but UT1 are in `hc-core::scale`. Each is a zero-sized marker
type, so `Instant<Tai>` and `Instant<Tt>` are different types. The Relation
column gives each scale's definition in terms of TAI or TT.

| Scale | What it is | Relation |
| --- | --- | --- |
| **TAI** | International Atomic Time. The weighted average of some 450 atomic clocks, realising proper time on the rotating geoid. The canonical scale of this library. | — |
| **TT** | Terrestrial Time. The theoretical ideal that TAI approximates; the independent variable of geocentric ephemerides. | `TT = TAI + 32.184 s`, exactly and by definition |
| **TCG** | Geocentric Coordinate Time. TT without the gravitational rescaling, so it runs slightly fast. | `TCG − TT = L_G/(1−L_G) · (TT − T₀)`, `L_G = 6.969290134×10⁻¹⁰` |
| **TDB** | Barycentric Dynamical Time. TT plus the periodic terms from the Earth's orbital motion; the independent variable of solar-system ephemerides. | `TDB − TT` is a truncated Fairhead–Bretagnon series, at most about 1.7 ms |
| **TCB** | Barycentric Coordinate Time. | `TCB − TDB` is linear in `L_B = 1.550519768×10⁻⁸` plus `TDB₀` |
| **GPS** | GPS system time, set to UTC at its 1980 epoch and without leap seconds since. | `GPS = TAI − 19 s`, exact by convention. GPS time is steered to UTC(USNO), not to TAI [is-gps-200g, §3.3.4], so what a receiver recovers differs from TAI − 19 s by a steering residual SOFA puts at "sub-microsecond" [sofa-ts]; the residual is observational and not carried |
| **GST** | Galileo System Time. Not Greenwich sidereal time. | `GST = TAI − 19 s`, by convention, as GPS |
| **BDT** | BeiDou Time, from 2006-01-01 00:00:00 UTC. | `BDT = TAI − 33 s`, by convention; 14 s behind GPS |
| **NavIC** | NavIC (IRNSS) system time. | `NavIC = TAI − 19 s`, by convention, as GPS |
| **UT1** | Universal Time — the Earth's actual rotation angle. In `hc-astro`, not `hc-core`. | Observational; modelled as `TT − ΔT`, or read from a DUT1 series. See below. |

`T₀` throughout is `1977-01-01T00:00:00 TAI`, the defining origin of TCG and
TCB.

**TT(BIPM).** The `Tt` marker is TT(TAI). The BIPM also publishes better
realisations of TT, one a year, named for the year: TT(BIPM25) is computed
from the frequency standards' data to December 2025 and tabulated as
TT(BIPM25) − TAI − 32.184 s every ten days from MJD 42 589: 27.67 µs at
MJD 60 669, 25 December 2024 [bipm-ttbipm-2025]. A later realisation revises the recent part of an
earlier one, so they are data. `hc_core::tt_bipm::TtBipmSeries` takes the
realisation the caller trusts, with its name, interpolates it linearly in
TAI, and refuses outside it, as `Ut1Offsets` does for DUT1. Its reading is
a `Duration`, not an `Instant<Tt>`, because converting an `Instant<Tt>` back
to TAI uses the exact 32.184 s. "When accuracies of better than 30 µs are
required, TT(BIPM) must be used" [eastman2010, §2.2].

GLONASS time is UTC(SU) + 3 h and takes leap seconds with UTC, so it is
not a marker here but `hc_core::gnss::GlonassTime`, a `UtcInstant` read three
hours ahead. The GNSS week numbers, their rollovers and GLONASS's
four-year intervals are in `hc_core::gnss`; the systems, their epochs and
their sources are in [systems/gnss-time.md](systems/gnss-time.md).

### Why the offsets are computed the way they are

TCG, TDB and TCB differ from TT by a *small* amount against the reading
itself: TDB by at most about 1.7 ms, TCG by about a second and TCB by about
twenty-four seconds since 1977. So the library computes that small difference
in `f64` and then applies it to the exact `Duration`, rather than converting
the whole reading to `f64` and back. An `f64` near 1.7 billion seconds
resolves only about 240 ns. An offset of a few seconds converts with an
error of about a femtosecond.

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

Each row gives the span of years and how `TAI − UTC` is found in it.

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

The CGPM resolved in 2022 to stop inserting leap seconds by 2035. That
changes the table's future, not its past. The table is data, so a change in
policy changes the data and not the code.

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
  ΔT below. Against the IERS EOP 20 C04 series [iers-eopc04] it is within
  0.1 s from 1974-01-01 through 2026-04-01, where ΔT is the observed value;
  0.06 s late on 2026-07-01, where ΔT is the USNO's prediction; and falls behind
  after October 2033 — 8.9 s at the hand-over — because there the
  Espenak–Meeus polynomial answers, and ΔT has grown more slowly since
  2006 than its forecast segment assumed.

Since 1972 the IERS has kept `|UT1 − UTC| < 0.9 s` (ITU-R TF.460-6), so past
the observed table UTC itself is a closer reading of UT1 than the model;
only a DUT1 series does better there.

### UT2, UT1R and UT1S

Before VLBI, the time services published UT1 with some of its known
variation smoothed out, and each smoothing has its own name.
`hc-astro::ut_variants` takes the caller's UT1 and returns each of them,
so that a historical reading labelled with one can be put back on UT1;
none is disseminated now, and the IERS recommends exchanging UT1 and the
length of day only [iers-tn36, ch. 8].

| Reading | Function | What is removed | Source |
| --- | --- | --- | --- |
| **UT2** | `ut2`, `ut2_minus_ut1` | The conventional seasonal variation: UT2 − UT1 = 0.022 sin 2π*T* − 0.012 cos 2π*T* − 0.006 sin 4π*T* + 0.007 cos 4π*T* s, *T* = 2000.000 + (MJD − 51 544.03)/365.2422, at most ±0.031 s | [usno-eo-values]; SOFA calls it "no longer used" [sofa-ts] |
| **UT1R** | `ut1r_iers2010`, `ut1r_minus_ut1_iers2010` | The zonal tides with periods under 35 days, as the IAU adopted in 1982: the 41 short terms of Table 8.1, at most 2.75 ms | [iers-tn36], ch. 8, Table 8.1 and footnote 1 |
| **UT1S** | `ut1s_iers2010`, `ut1s_minus_ut1_iers2010` | All the zonal tides, to the 18.6-year nodal term: the table's 62 terms, at most 0.173 s | [iers-tn36], ch. 8, Table 8.1 |

The tidal model is part of the name. Table 8.1 is the IERS 2010 model,
Yoder, Williams and Parke's 1981 elastic tide with Wahr and Bergen's 1986
inelastic body tide and Kantha et al.'s 1998 ocean tide added, and it
differs from the Conventions 2003 model by about 6 µs at the fortnightly
term [iers-tn36, ch. 8, §8.1]. It is not Yoder et al.'s own 1981 tables,
which the IAU's 1982 definition of UT1R names; a UT1R from those tables
would be a separate function. The three tide papers and the IAU
resolution are cited through [iers-tn36], ch. 8, and were not read. The
whole-table sum reproduces the test case printed in the IERS routine
`RG_ZONT2.F` to 10⁻¹² s [iers-rg-zont2]; the routine's header was read
for the test case, and the table and the Delaunay arguments (ch. 5,
eq. 5.43) were taken from the Conventions' text. The system, with worked
examples of UT2 and the Earth Rotation Angle, is written up in
[systems/earth-rotation.md](systems/earth-rotation.md).

### GMAT, the astronomical day

The *Nautical Almanac* counted Greenwich mean time from noon up to its
volume for 1924 [nautical-almanac-1924]. The reading is the same mean
time, twelve hours behind, and the astronomical date is the civil date of
the noon the day begins at. The almanac calls both reckonings "G.M.T.";
the name GMAT was introduced later to tell the noon-based one apart
[wikipedia-greenwich-mean-time, citing the *Astronomical Supplement to the
Astronomical Almanac*, 1992, p. 76, not read].
`hc-astro::gmat` converts a reading either way, exactly, and the
WebAssembly and C exports `hc_gmat_from_gmt` and `hc_gmt_from_gmat` write
it as a fixed day, seconds of the day and attoseconds.

| Reading | Function | Rule | Source |
| --- | --- | --- | --- |
| **GMAT**, Greenwich Mean Astronomical Time | `gmat_from_gmt`, `gmt_from_gmat` | GMAT = GMT − 12 h; the day begins at noon and is named by the civil day it begins on, `DayBoundary::Noon(DayNaming::ByStart)`. The *Nautical Almanac* reckons its G.M.T. so up to its volume for 1924 and from midnight from 1925: 1924 December 31, 12ʰ GMAT is 1925 January 1, 0ʰ GMT | [nautical-almanac-1924], the notice before the title page; its lunar eclipses of 20 February and 14 August 1924, at 4ʰ 12ᵐ 25ˢ.7 and 8ʰ 22ᵐ 59ˢ.1 from noon, fall where NASA's catalogue puts them, in the afternoon and evening [espenak-lunar-eclipses-1901] |

## Sidereal time and the Earth Rotation Angle

UT1 is defined by the Earth's rotation, and the rotation is measured by an
angle. Two conventions for that angle are in use, and `hc-astro::earth`
carries both under their own names, as [policy.md](policy.md) §5 asks.
[systems/earth-rotation.md](systems/earth-rotation.md) works the angle by
hand.

| Quantity | Function | Convention | Argument |
| --- | --- | --- | --- |
| **ERA**, the Earth Rotation Angle | `earth_rotation_angle` | IAU 2000: ERA = 2π(0.779 057 273 264 0 + 1.002 737 811 911 354 48 *T*u), *T*u = JD(UT1) − 2 451 545.0 (IERS Conventions 2010 [iers-tn36], ch. 5, eqs 5.14–5.15) | UT1 |
| **GMST**, IAU 2006 | `mean_sidereal_time_iau2006`, `mean_sidereal_time_iau2006_at` | ERA plus the precession in right ascension, 0.014 506″ + 4612.156 534″ *t* + … (ch. 5, eq. 5.32) | UT1 for the ERA, TT centuries *t* for the polynomial |
| **GMST**, IAU 1982 | `mean_sidereal_time_iau1982` (also `mean_sidereal_time`) | Meeus's (12.4), a polynomial in UT1 alone [meeus1998] | UT1 |
| **GAST**, from IAU 1982 | `apparent_sidereal_time_iau1982` (also `apparent_sidereal_time`) | IAU 1982 GMST plus the equation of the equinoxes Δψ cos ε from Meeus's abridged nutation | UT1 |

The ERA is the angle between the Celestial and Terrestrial Intermediate
Origins, and is linear in UT1 by definition: it is what UT1 *is* in the
IAU 2000 framework. Mean sidereal time is measured from the equinox instead,
and the equinox moves with precession, so the IAU 2006 GMST is the ERA plus
a polynomial in TT. The IAU 1982 GMST folds both into one polynomial in UT1.
The two GMSTs differ by 0.14 ms of time on 2006-01-01 and part slowly over
the centuries. The rise, set and transit code uses the IAU 1982 pair.

The apparent sidereal time here is not the IAU 2006/2000A GAST, which needs
the full nutation series; Meeus's four-term nutation holds it to about
0.5″, or 0.03 s of time, against ERFA's `eraGst94`.

The IAU 2006 functions reproduce ERFA's test values for `eraEra00` and
`eraGmst06` to 10⁻⁹ degree, and the IAU 1982 GMST ERFA's `eraGmst82` to
0.7 µs of time [erfa]. ERFA is the BSD-licensed derivative of the IAU's
SOFA library; its test file was read, not its routines.

## ΔT

`hc-astro` needs ΔT (TT − UT1) to place astronomical events on a civil
calendar. It answers from three sources, and `time::delta_t_regime` says
which one answered a given year. Neither table is extrapolated.

| Span | Source | How well it is known |
| --- | --- | --- |
| 1974-01-01 to 2026-04-01 (`time::TABULATED_DELTA_T_FIRST`, `TABULATED_DELTA_T_LAST`) | The USNO's observed values, one a year, interpolated between samples | The observation itself at each sample. In the atomic era ΔT = 32.184 s + (TAI − UTC) − DUT1, and the tests check the table against the leap-second table and the IERS DUT1 through that identity |
| From there to 2033-10-01 (`PREDICTED_DELTA_T_LAST`) | The USNO's quarterly predictions, with the error the USNO states for each (`time::delta_t_predicted`) | Where the predictions overlap the observations, the observation wins. There the predictions ran up to 0.12 s low, more than their stated error, so the stated error is an estimate and not a bound |
| Outside both | The Espenak–Meeus polynomial fits over −500 to +2150, and a parabola beyond | Before atomic clocks the fits rest on eclipse records: a few seconds in 1900, minutes in 1000 CE, hours in 1000 BCE. After 2033 the fit is a forecast made in 2006 that runs 8.9 s high, so an instant computed there is about nine seconds early until the tables are extended |

`hc-astro` does not attach an uncertainty to each value. [Its
README](../crates/hc-astro/README.md) gives
the sources, the second model it carries, and the measurements behind the
figures above.

This is one reason a Chinese lunisolar date computed for 500 CE can differ
by a day from what was proclaimed. The Sun and Moon can be computed for
that year, but ΔT is not known well enough to say on which side of
midnight an event fell.

## Epochs

`hc-core::epoch` collects the origins that other systems chose, each as a
TAI reading. The first column is the identifier `hc_core::epoch::by_id`
accepts:

| Epoch | Origin |
| --- | --- |
| `unix` | 1970-01-01T00:00:00Z (`TAI − UTC` was 8.000082 s) |
| `gps` | 1980-01-06T00:00:00Z |
| `galileo-system-time` | 1999-08-22T00:00:00 GST, 1999-08-21T23:59:47Z |
| `beidou-time` | 2006-01-01T00:00:00Z |
| `navic-time` | 1999-08-22T00:00:00 NavIC, 1999-08-21T23:59:47Z |
| `glonass-time` | 1996-01-01T00:00:00 UTC(SU) + 3 h, 1995-12-31T21:00:00Z, the start of *N*4 = 1 |
| `j2000` | 2000-01-01T12:00:00 TT |
| `tcg-tcb-origin` | 1977-01-01T00:00:00 TAI |
| `mjd` | 1858-11-17T00:00:00 UT |
| `julian-day` | −4712-01-01T12:00:00 UT, proleptic Julian |
| `rata-die` | 0001-01-01, proleptic Gregorian |
| `dotnet-ticks` | 0001-01-01T00:00:00, proleptic Gregorian, the origin of .NET's `DateTime.Ticks`; the same instant as `rata-die` |
| `ccsds-cuc` | 1958-01-01T00:00:00 TAI, the Level 1 epoch of the CCSDS Unsegmented Code |
| `windows-filetime` | 1601-01-01T00:00:00Z |
| `ntp` | 1900-01-01T00:00:00Z |
| `core-foundation` | 2001-01-01T00:00:00Z |
| `uuid-gregorian` | 1582-10-15T00:00:00Z, proleptic, the timestamp of UUID versions 1 and 6 |
| `sas-stata` | 1960-01-01T00:00:00Z, proleptic, SAS dates and datetimes and Stata's `%td`, `%tc` and `%tC` |

## TAI64 labels

`hc-core::tai64` reads and writes Bernstein's TAI64 family [bernstein-tai64]:
the label `2⁶² + s` for the TAI second beginning `s` seconds after
1970-01-01 00:00:00 TAI, in eight big-endian bytes, followed in TAI64N by
four bytes of nanoseconds and in TAI64NA by four more of attoseconds. The
origin is 1970 *TAI*, the origin of `Instant<Tai>`, not the POSIX epoch.
Labels from 2⁶³ are reserved and refused. TAI64NA resolves the attosecond,
as `Duration` does, so it round-trips exactly; TAI64 and TAI64N name the
second and nanosecond that contain an instant.

Not every label in the wild is true TAI, so the module carries two named
conventions (policy §5):

| Convention | Label of a second | Reads and writes | Source |
| --- | --- | --- | --- |
| `tai64` | `2⁶² + s`, `s` the TAI seconds since 1970-01-01 00:00:00 TAI | `Instant<Tai>` | [bernstein-tai64] |
| `tai64-posix-plus-10` | `2⁶² + 10 + p`, `p` the POSIX seconds | `UnixTime`, with no leap-second table | [bernstein-daemontools-tai64n], [bernstein-daemontools-tai64nlocal] |

daemontools' `tai64n`, which timestamps `multilog`'s logs, takes the
system clock as "TAI seconds since 1970-01-01 00:00:10 TAI". On a clock
kept in the Olson `right/` mode that is true, and the label is `tai64`:
the page's example `4000000037c219bf2ef02e94`, which `tai64nlocal`'s page
prints as 1999-08-23 21:03:43.787492500 US/Pacific, decodes as `tai64` to
exactly that instant, 1999-08-24 04:03:43.787492500 UTC. On an ordinary
clock, which keeps POSIX time, the label is `tai64-posix-plus-10`; POSIX 0
is `@400000000000000a`. Read as `tai64`, such a label is `TAI − UTC − 10`
seconds early, 22 s in 1999 and 27 s since 2017. The bytes do not say which
clock wrote them, so the caller chooses by name. libtai's `tai_now`
makes the same assumption, "that the time_t returned from the time
function represents the number of TAI seconds since 1970-01-01 00:00:10
TAI", which it says matches the Olson library's "right" mode
[bernstein-libtai-tai].

## Computing timestamps

Counts that software writes from one of those epochs, each a label of
86 400-second days unless it says otherwise:

- **NTP eras**, `hc-core::ntp`: the 128-bit date's era and era offset, and
  the 64-bit timestamp, which wraps at 2036-02-07T06:28:16Z, resolved
  against a reference time within 2³¹ s of it [rfc5905].
- **UUID versions 1 and 6**, `hc-core::uuid`: the 60-bit count of 100 ns
  from `uuid-gregorian`, in both octet orders [rfc9562].
- **SAS and Stata datetimes**, `hc-core::sas_stata`: SAS seconds and
  Stata's `%tc` milliseconds from 1960, and Stata's `%tC`, which counts the
  leap seconds inserted since 1972 as UTC does and so reads the leap-second
  table [sas-lrcon-dates; stata-help-datetime-conversion].
- **.NET `DateTime.Ticks`**, `hc-core::dotnet`: 100 ns from `dotnet-ticks`,
  without leap seconds, in the zone the value's `Kind` names; only a `Utc`
  value is an instant [ms-datetime-ticks; ms-datetimekind].
- **CCSDS time codes**, `hc-core::ccsds` and `hc-format::ccsds`
  [ccsds-301-0-b-4]. The Unsegmented Code counts TAI seconds and binary
  fractions from `ccsds-cuc` and is not a label: it has no leap seconds.
  The Day Segmented Code counts UTC days from 1958 and the millisecond of
  the day, which reaches 86 400 999 on a day that ends in an inserted leap
  second. The Calendar Segmented Code and the ASCII codes A and B are UTC
  readings, with second 60. All three UTC codes check the second against
  the leap-second table, `hc-core::leap::end_of_day_step`, and refuse
  23:59:60 past its end.
- **Radio time codes**, `hc-format::radio`: a minute's frame of JJY (JST),
  DCF77 (CET or CEST, for the following minute) and WWVB's amplitude and
  phase codes (UTC). JJY and WWVB send a frame of 61 seconds for an
  inserted leap second and of 59 for an omitted one; DCF77 sends 60 marks
  for an inserted one, and its omitted one, which PTB describes no frame
  for, is not carried [nict-jjy-timecode; ptb-dcf77-timecode;
  nist-wwvb-enhanced-2013].
- **IRIG time codes**, `hc-format::irig`: a frame of IRIG A, B, D, E, G or
  H, the day of the year and the time of day in BCD, the year's last two
  digits and the seconds of the day in binary where the code carries
  them. The code names no time scale, so a frame is read as a date and a
  time of day. The standard leaves the leap second's frame open
  [rcc-200-16]; at 23:59:60 the module lets the seconds of the day reach
  86 400.

[systems/binary-timestamps.md](systems/binary-timestamps.md),
[systems/statistical-software-dates.md](systems/statistical-software-dates.md),
[systems/ccsds-time-codes.md](systems/ccsds-time-codes.md),
[systems/radio-time-codes.md](systems/radio-time-codes.md) and
[systems/irig-time-codes.md](systems/irig-time-codes.md) work examples
through; the FAT date and time words, which are local time, are in
`hc-format::fat`.

## Julian and Besselian epochs

`hc-core::epoch_notation` writes a TT instant as a year and a fraction
[sofa-ts, §2.4]. The Julian epoch is 2000.0 + (JD − 2 451 545.0) / 365.25
on TT, exactly as defined. The Besselian epoch is
1900.0 + (JD − 2 415 020.313 52) / 365.242 198 781, the constants of ERFA's
`eraEpb`, which credits them to Lieske (1979, not read) [erfa-epb]; its
scale is TDB, taken as TT, which `eraEpb` calls indistinguishable for this
purpose. An epoch written without its letter is Besselian before 1984.0 and
Julian from it. SOFA's example, JD 2 457 073.056 31 as B2015.136 594 102 1
and J2015.134 993 319 6, is a test.

## The Heliocentric Julian Date

`hc-astro::hjd` corrects a Julian Date to the Sun: the light from a distant
object reaches the Earth up to 8.3 minutes before or after it passes the
Sun, and the correction is the Rømer delay, the Earth's heliocentric
position projected on the object's direction and divided by the speed of
light, both "in the same coordinate system" [eastman2010, §2.1]. Here that
system is the mean equator and equinox of J2000: the Earth is VSOP87's,
turned back from the equinox of date by the general precession, and the
object's right ascension and declination are J2000's.

The time scale is part of the name, because a Julian Date "can be
specified in many time standards" [eastman2010, §2.2]. `hjd_tt` is HJD_TT,
JD_TT plus the delay. `hjd_utc` is HJD_UTC, JD_UTC plus the delay with the
Earth still placed at the TT instant, from the TT − UTC the caller gives.
Placing it at the UTC reading instead gives what the paper calls HJD′_UTC,
which drifts with the leap seconds, and nothing here computes it.

The HJD is good only to 8 s as a time on an inertial clock, because the
Sun is pulled about the barycentre by Jupiter and Saturn, and the IAU
deprecated it in 1991 [eastman2010, §2.1]. The barycentric BJD_TDB needs
the barycentre, which the workspace does not compute. As a computation of
the HJD, the delay agrees with the IDL Astronomy Library's `helio_jd` to
the 0.1 s it prints for six objects from 1940 to 2100 [idl-helio-jd]. The
same table's SLALIB column agrees within ten years of 2000; its 1940 and
2100 rows, 3.9 s and 4.8 s off, are reproduced to 0.1 s by the Earth on the
equinox of date against the J2000 direction, a mixed frame.

## Swatch Internet Time

`hc-core::internet_time::Beat` reads @000 to @999 from POSIX time:
⌊seconds since midnight BMT / 86.4⌋, BMT being UTC+1 all year, not the
mean solar time of Biel [swatch-internet-time;
wikipedia-swatch-internet-time]. @000 is 23:00 UTC and @248 is
04:57:07.2 UTC. A leap second reads as the second after it, as POSIX time
does. The 86.4-second `beat` itself is a unit in `hc-units`.

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
