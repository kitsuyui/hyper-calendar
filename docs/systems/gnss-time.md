# GNSS system times and week numbers

Backs `hc-core`'s `Gps`, `GalileoTime`, `BeidouTime` and `NavicTime`
scales, the `gps`, `galileo-system-time`, `beidou-time`, `navic-time` and
`glonass-time` epochs, the week numberings in `hc_core::gnss`, and
`hc_core::gnss::GlonassTime`.

## What it is

Every satellite navigation system broadcasts its own clock. A receiver
solves for its position against that clock, so the system time is what the
signal carries and UTC is what the receiver derives from it with the offset
the navigation message also broadcasts. Four of the five global and
regional systems keep an atomic scale without leap seconds, continuous
since an epoch of its own choosing at which it stood a fixed whole number
of seconds from UTC: GPS time and BeiDou Time equal to UTC, Galileo System
Time and NavIC time equal to GPS time, 13 s ahead of UTC. GLONASS keeps
Moscow's civil time, leap seconds included.

TAI (International Atomic Time) is the continuous atomic count; UTC
(Coordinated Universal Time) is TAI with leap seconds inserted to follow
the Earth's rotation, and UTC(USNO) and UTC(SU) are the realisations of it
maintained by the US Naval Observatory and Russia's national reference.
An ICD (Interface Control Document) is each system's published signal
specification.

Each is realised by a control segment steering the constellation to a
national or regional UTC: GPS to UTC(USNO) [is-gps-200g, §3.3.4], GLONASS
to UTC(SU) [glonass-icd-5-1, §3.3.3]. The realisation differs from the
nominal relation to TAI by nanoseconds to tens of nanoseconds; SOFA (the
IAU's Standards of Fundamental Astronomy) calls the GPS relation good to
"sub-microsecond" accuracy [sofa-ts]. The offsets below are the
*nominal* relations, exact by convention, and the library does not carry
the steering residuals, which are observational.

## How it works

**The scales.** A scale set equal to UTC at its epoch and given no leap
seconds after it, as GPS time and BeiDou Time were, is TAI less the
`TAI − UTC` that held at the epoch. Galileo System Time and NavIC time
were set to GPS time instead, so they inherit its offset, TAI − 19 s, and
not the 32 s that `TAI − UTC` was on their epoch day:

| Scale | Epoch | `TAI − UTC` on the epoch day (for reference) | Relation |
| --- | --- | --- | --- |
| GPS time | 1980-01-06 00:00:00 UTC [is-gps-200g, §3.3.4] | 19 s | `GPS = TAI − 19 s` |
| Galileo System Time | 1999-08-22 00:00:00 UTC less 13 s, that is 1999-08-21 23:59:47 UTC [galileo-os-sis-icd-2-1, §5.1.2] | 32 s | `GST = TAI − 19 s` |
| BeiDou Time | 2006-01-01 00:00:00 UTC [bds-sis-icd-b1i-1-0, §3.3] | 33 s | `BDT = TAI − 33 s` |
| NavIC system time | 00:00 on 1999-08-22 of its own reckoning, 1999-08-21 23:59:47 UTC [irnss-sps-icd-1-1, §5.7] | 32 s | `NavIC = TAI − 19 s` |

GST and NavIC time were set thirteen seconds ahead of UTC at their epoch,
the GPS − UTC of that day [galileo-os-sis-icd-2-1, §5.1.2;
irnss-sps-icd-1-1, §5.7], so both read GPS time; their week zero is GPS
week 1024. BDT was set equal to UTC in 2006, when GPS − UTC was 14 s, so
BDT = GPS − 14 s. The `TAI − UTC` column is the leap-second table's, not
the ICDs', which state the epochs in UTC.

**GLONASS time** is UTC(SU) + 3 h [glonass-icd-5-1, §3.3.3]. It inserts
a leap second when UTC does, at 02:59:60 Moscow time rather than 23:59:60,
so it is UTC shifted by three hours and not a scale with a fixed offset
from TAI. Its dates are broadcast as a four-year interval number *N*4,
counted from 1996 [glonass-icd-5-1, §4.5], and a day number *N*T within
the interval, 1 on 1 January of the interval's leap year
[glonass-icd-5-1, §4.4]. The ICD's own example (Attachment A.3.1.3) reads
*N*T = 839 as the interval's third year, since days 732 to 1 096 are
year 3, and with *N*4 = 2 gets 1996 + 4 × (2 − 1) + (3 − 1) = 2002.

**Week numbers.** The systems broadcast time as a week number and the
seconds of the week, 0 to 604 799, counted from the week zero epoch. The
week field is short, so the broadcast number is the full week modulo a
power of two:

| Field | Bits | Modulus | Period | Source |
| --- | --- | --- | --- | --- |
| GPS legacy navigation message (LNAV) | 10 | 1 024 | 19.6 years | [is-gps-200g, §20.3.3.3.1.1] |
| GPS civil navigation message (CNAV) | 13 | 8 192 | 157 years | [is-gps-200g, §30.3.3.1.1.1] |
| Galileo | 12 | 4 096 | 78.5 years | [galileo-os-sis-icd-2-1, Table 67] |
| BeiDou | 13 | 8 192 | 157 years | [bds-sis-icd-b1i-1-0, §5.2.4.4] |
| NavIC | 10 | 1 024 | 19.6 years | [irnss-sps-icd-1-1, §5.7, §6.2.1.1] |

The GPS legacy week has rolled over twice, in August 1999 and at
23:59:42 UTC on 6 April 2019, the ends of GPS weeks 1023 and 2047
[gps-gov-rollover]. A broadcast week number therefore names a
family of weeks 1 024 apart, and only something outside the signal — a
date the receiver already knows to be close, or a date it knows it is not
before — picks one.

**Worked example.** GPS week 2048 begins at 2019-04-07 00:00:00 GPS. From
1980-01-06 to 2019-04-07 is 14 336 days, 2 048 weeks. In April 2019
`TAI − UTC` was 37 s, so `GPS − UTC` was 18 s and the week began at
2019-04-06 23:59:42 UTC. The legacy message broadcast week 2048 mod 1024,
which is 0, again. A receiver that knows it is after 1 January 2019,
GPS week 2034, takes the first week at or after 2034 that is 0 modulo
1 024: 2048. A receiver that instead assumes it is still in the first
1 024-week period reads broadcast week 0 as 6 January 1980: the rollover
bug of April 2019.

## What is carried

- **Scales** `Gps`, `GalileoTime`, `BeidouTime` and `NavicTime` in
  `hc_core::scale`, each an exact offset from TAI, with `TimeScaleId`
  variants. The abbreviation of `GalileoTime` is the ICD's GST, which is
  not Greenwich sidereal time; the identifiers spell the name out.
- **Epochs** `gps`, `galileo-system-time`, `beidou-time`, `navic-time`
  and `glonass-time` (the start of *N*4 = 1) in `hc_core::epoch`.
- **Week numberings** in `hc_core::gnss`: `gps-lnav-week`,
  `gps-cnav-week`, `galileo-week`, `beidou-week` and `navic-week`, each an
  epoch and a bit width. `week_time` splits a TAI instant into the full week
  and the time of week, `to_tai` joins them, `broadcast` truncates a full
  week to the field, and the two rollover rules take a reference instant
  the caller supplies: `resolve_not_before` returns the first full week at
  or after the reference's week, and `resolve_nearest` the full week within
  half a period of it, the half-open window `[reference − m/2,
  reference + m/2)`.
- **GLONASS** `GlonassTime`, a `UtcInstant` read three hours ahead, with
  conversions from and to UTC and, through the leap-second table, TAI.
  `GlonassTime::date` gives a `GlonassDate` of *N*4
  (`four_year_interval`) and *N*T (`day`); the time of day is the
  reading's own.
- **Not carried**, with the reason:
  - *The steering residuals* of each realisation, and the broadcast
    `GPS − UTC` and GGTO (the GPS–Galileo time offset) parameters: they
    are observations, supplied in the navigation message, not rules.
  - *QZSS time*: not carried, because IS-QZSS was not read.
  - *GLONASS dates from 2100.* The interval arithmetic is a 1 461-day
    cycle, which the Gregorian calendar breaks in 2100, a common year;
    `GlonassTime::date` refuses instants outside 1996–2099 rather than
    count a 29 February that does not exist.
  - *The ICDs' Z-count, subframe and message structure*, which are about
    the signal, not the time.

## Accuracy

The relations are definitions, so the checks are that the epochs and
offsets agree when reached two ways: through the leap-seconds table from
the UTC label the ICD gives, and through the stated `TAI − system`
constant.

| Check | Test | Result |
| --- | --- | --- |
| Each epoch's TAI reading is its UTC label plus the table's `TAI − UTC` less the ICD's stated shift; each scale reads its epoch as the label | `every_gnss_epoch_is_its_utc_label_read_through_the_leap_table` | all five |
| GST and NavIC week 0 is GPS week 1024; BDT week 0 is GPS week 1356, second 14 | `the_week_zeros_line_up_with_gps_weeks` | exact |
| GPS week 2048 begins at 2019-04-06 23:59:42 UTC, broadcast as legacy week 0 and CNAV week 2048, and resolves to 2048 against 2019-01-01 by both rules | `the_april_2019_rollover` | exact |
| Week and time of week round-trip; a time of week of 604 800 s or a week before zero is refused | `week_time_round_trips_and_refuses_bad_fields` | all |
| GLONASS reads 03:00 for 00:00 UTC, names 2017-01-01 02:59:60 for the leap second, and round-trips | `glonass_is_utc_three_hours_ahead_with_its_leap_seconds` | exact |
| *N*4 = 8, *N*T = 1 at 2024-01-01 00:00 Moscow; *N*T = 1461 on 2027-12-31; 2100 refused | `glonass_four_year_intervals` | exact |
| The ICD's example: *N*4 = 2 and *N*T = 839 fall in 2002, whose days are *N*T 732 to 1 096 | `the_icd_example_of_nt_839_is_2002` | exact |

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [is-gps-200g] | GPS time and its epoch, steering to UTC(USNO), the 10- and 13-bit week numbers (§3.3.4, §20.3.3.3.1.1, §30.3.3.1.1.1) | Yes, the PDF at its entry's URL, 2026-09-27 |
| [arl-gps-time-2019] | The GPS epoch, quoting IS-GPS-200 §3.3.4 | Yes, 2026-09-27 |
| [gps-gov-rollover] | The rollovers of August 1999 and 23:59:42 UTC on 6 April 2019 | Yes, 2026-09-27 |
| [galileo-os-sis-icd-2-1] | The GST epoch and shift, the 12-bit week (§5.1.2, Table 67) | Yes, the PDF at its entry's URL, 2026-09-27 |
| [bds-sis-icd-b1i-1-0] | The BDT epoch and no leap seconds (§3.3), the 13-bit week (§5.2.4.4) | Yes, the PDF at its entry's URL, 2026-09-27 |
| [glonass-icd-5-1] | UTC(SU) + 3 h and the leap seconds (§3.3.3), *N*T (§4.4), *N*4 (§4.5), the worked example (Attachment A.3.1.3) | Yes, the PDF at its entry's URL, 2026-09-27 |
| [irnss-sps-icd-1-1] | The NavIC epoch and the 10-bit week (§5.7, §6.2.1.1) | Yes, the PDF at its entry's URL, 2026-09-27 |
| [sofa-ts] | GPS time's sub-microsecond relation to TAI | Yes, 2026-09-27 |
| [iana-leap-seconds-list] | `TAI − UTC` at each epoch | Yes, as `hc_core::leap` records |

## Code

`crates/hc-core/src/gnss.rs`, with the scales in `scale.rs` and the epochs
in `epoch.rs`. Anchors: `every_gnss_epoch_is_its_utc_label_read_through_the_leap_table`,
`the_week_zeros_line_up_with_gps_weeks`, `the_april_2019_rollover`,
`glonass_is_utc_three_hours_ahead_with_its_leap_seconds`,
`glonass_four_year_intervals`, `the_icd_example_of_nt_839_is_2002`.
