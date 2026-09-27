# CCSDS time code formats: CUC, CDS, CCS and ASCII A and B

Backs `hc-core::ccsds` with the epoch `ccsds-cuc`, `hc-core::leap::end_of_day_step`,
and `hc-format::ccsds`. The day count `ccsds-day` in `hc-calendars-solar` is
the CDS day segment read as a calendar.

## What it is

The Consultative Committee for Space Data Systems recommends five ways of
writing a time for exchange between space agencies [ccsds-301-0-b-4]. Four
are binary: a code is a preamble field, the *P-field*, that says which
format follows, and a time field, the *T-field*. The fifth is a text form.

| Code | T-field | Time scale | Epoch |
| --- | --- | --- | --- |
| **CUC**, CCSDS Unsegmented Code | a binary count of seconds and binary fractions of a second | TAI; "leap-second corrections do not apply" (§3.2.1) | 1958 January 1 TAI (Level 1) or one the agency defines (Level 2) |
| **CDS**, CCSDS Day Segmented Code | a count of days, the millisecond of the day, and optionally the microsecond or picosecond of the millisecond | UTC; "the leap second correction must be made" (§3.3.1) | 1958 January 1 (Level 1) or one the agency defines (Level 2) |
| **CCS**, CCSDS Calendar Segmented Code | the year, month and day or the day of the year, hour, minute, second and up to twelve decimal places, in binary-coded decimal | UTC, with the leap second (§3.4.1) | the calendar's own |
| **ASCII A** | `YYYY-MM-DDThh:mm:ss.d→dZ` | UTC, with the leap second (§3.5.1) | the Gregorian calendar's |
| **ASCII B** | `YYYY-DDDThh:mm:ss.d→dZ` | UTC, with the leap second (§3.5.1) | the Gregorian calendar's |

A P-field is optional. Without it the code "is not self-identified, and
identification must be obtained by other means" (§3.1.1), so a decoder is
told the format by its caller. The ASCII codes never have one (§3.5.2).

The standard sorts codes into four levels by how much of them a reader can
interpret (§1.3). Level 1 codes are fully self-defined and carry absolute
time. A Level 2 code has a self-defined structure but an epoch that "it is
necessary to obtain … from an external source", so it gives time only
relative to that epoch. Level 3 and Level 4 codes are agency-defined
octets whose P-field states only their length.

## How it works

### The P-field

The first octet is always present when a P-field is sent (§3.1.1):

| Bits | Meaning |
| --- | --- |
| 0 | extension flag: 1 if another P-field octet follows |
| 1–3 | time code identification: `001` CUC Level 1, `010` CUC Level 2, `100` CDS, `101` CCS, `110` agency-defined Level 3 or 4; `000`, `011` and `111` reserved |
| 4–7 | details of the code |

Bit 0 is the first bit transmitted and the most significant (§1.5).

**CUC** (§3.2.2): bits 4–5 are the number of octets of whole seconds less
one, bits 6–7 the number of octets of binary fraction. A second octet,
signalled by bit 0, adds up to 3 octets of seconds (its bits 1–2) and up to
7 of fraction (bits 3–5); its bits 6–7 are "reserved for mission
definition". A CUC T-field therefore holds 1 to 7 octets of seconds and 0 to
10 octets of fraction.

**CDS** (§3.3.2): bit 4 is the epoch, 0 for 1958 January 1 (Level 1) and 1
for an agency-defined one (Level 2); bit 5 the day segment, 16 or 24 bits;
bits 6–7 the submillisecond segment: none, 16 bits of microseconds, 32 bits
of picoseconds, or `11`, reserved.

**CCS** (§3.4.2): bit 4 is the variation, 0 for month and day of month, 1
for day of year; bits 5–7 the number of subsecond octets, 0 to 6, each two
decimal digits; `111` is "not used".

**Agency-defined** (§3.6.2): bits 4–7 are the T-field's length less one, 1
to 16 octets.

### The T-fields

**CUC** is one binary counter, most significant octet first. With *n*
octets of fraction the count is in units of 2⁻⁸ⁿ s, and the time is the
count times that unit after the epoch.

**CDS** is three right-adjusted binary counters: the day (16 or 24 bits),
the millisecond of the day (32 bits), and the submillisecond (16 or 32
bits). Annex A gives their ranges: the millisecond of the day runs to
86 399 999, and "0 to 86,400,999 or 86,398,999 when leap second adjustments
are introduced"; the microsecond of the millisecond runs to 999.

**CCS** is binary-coded decimal: two decimal digits to an octet, the year
in 16 bits, then month and day of month (8 bits each), or the day of the
year in 16 bits "the four most significant bits of [which] are not used and
are set to zero" (§3.4.1.2); then hour, minute and second, 8 bits each, and
the subsecond octets for 10⁻² s down to 10⁻¹² s. Annex A: the second runs
to 59, "0 to 60 or 58 when leap second adjustments are introduced"; the
year from 1 to 9999; the leap year is "every year divisible by 4, except
for the years divisible by 100 and not divisible by 400".

**ASCII A and B** (§3.5.1) are one octet per character, all subfields with
leading zeros, the second 00–59 "(-58 or -60 during leap seconds)", the
fraction "as many 'd' characters … as required", and "an optional
terminator consisting of the ASCII character 'Z'". Both are subsets of
ISO 8601 (annex B3.4). §3.5.1.3 allows subsets: the calendar part or the
time part alone, without the `T`, and either truncated; joined with the `T`,
"the CALENDAR subset may NOT have been truncated from the RIGHT, and the
TIME subset may NOT have been truncated from the LEFT".

### Leap seconds

CUC counts TAI seconds and never sees a leap second. The three UTC codes
must represent one: a day that ends in an inserted second has a second 60,
a millisecond of the day up to 86 400 999, and one that ends in an omitted
second stops at second 58 and millisecond 86 398 999. Whether a given day
ends in a leap second is not in the code; it is in the IERS's announcements,
which this library keeps as `hc-core::leap`.

### Worked example

The standard's own example of the ASCII codes is 1988-01-18T17:20:43.123456
UTC, written `1988-01-18T17:20:43.123456Z` in code A and
`1988-018T17:20:43.123456Z` in code B (§§3.5.1.1–3.5.1.2): 18 January is
the 18th day of the year.

The same instant in CCS, month-and-day variation with microsecond
resolution (three subsecond octets): the P-field is `0` (no extension),
`101` (CCS), `0` (month and day), `011` (three octets), `0101 0011`, 0x53.
The T-field is the decimal digits two to an octet:

```
19 88 01 18 17 20 43 12 34 56
```

In the day-of-year variation the P-field is `0101 1011`, 0x5B, and the
month and day become the 16-bit day of year 018, `00 18`:

```
19 88 00 18 17 20 43 12 34 56
```

In CDS the day is the count from 1958 January 1: 1958-01-01 to 1988-01-18
is 10 974 days, 0x2ADE, and 17:20:43.123 is millisecond 62 443 123,
0x03B8 CE73, with 456 µs of the millisecond left over, 0x01C8. With a
16-bit day and a 16-bit microsecond segment the P-field is `0100 0001`,
0x41, and the code is

```
41  2A DE  03 B8 CE 73  01 C8
```

A leap second, in CDS: 2016-12-31 ended in an inserted second. That day is
day 21 549 from 1958, and 23:59:60.5 is millisecond 86 400 500. On any
other day that millisecond does not exist.

CUC counts TAI. At 2000-01-01T00:00:00 UTC, TAI − UTC was 32 s, and
1958-01-01 to 2000-01-01 is 15 340 days, so the Level 1 count is
15 340 × 86 400 + 32 = 1 325 376 032 s, 0x4EFF A220. With four octets of
seconds and none of fraction the P-field is `0001 1100`, 0x1C, and the code
is `1C 4E FF A2 20`; half a second later, with one octet of fraction
(P-field `0001 1101`, 0x1D), it is `1D 4E FF A2 20 80`.

## What is carried

- **`hc-core::ccsds`**: the P-field of every binary code, decoded and
  encoded, refusing the reserved identifications, the reserved CDS and CCS
  resolutions, and extension octets the standard does not define; the CUC
  and CDS T-fields, decoded and encoded, with the P-field or without it.
  - CUC reads as a `Duration` since its epoch, rounded up to the
    attosecond so that writing it again gives the same count, and a Level 1 code as an `Instant<Tai>` from 1958-01-01T00:00:00 TAI,
    the epoch `ccsds-cuc`. A Level 2 code's epoch is the caller's, on the
    scale the caller names: `CucTime::since_epoch` gives the span, and a
    Level 1 conversion of a Level 2 code is refused.
  - CDS reads as a `UtcInstant`. A Level 1 code counts from 1958-01-01; a
    Level 2 code's epoch day is the caller's, given as a day of the POSIX
    day count, so that 1950 January 1, the agency epoch the standard names
    as an example, is `-7305`. A millisecond of the day at or past
    86 400 000 is accepted only on a day that ends in an inserted leap
    second; past the leap-second table's validity it is refused with
    `AfterModelEnd`, since whether that day ends in one is not yet known.
  - The unit of a CUC count is the second. §3.2.1 lets the metadata name
    another; a code on another unit is outside this module.
- **`hc-core::leap::end_of_day_step`**: the leap second, +1, −1 or 0, that
  ends a UTC day, from the table; the check every UTC code needs.
- **`hc-format::ccsds`**: the CCS T-field in both variations, to and from a
  UTC `CivilDateTime` with the leap second checked against the table; the
  ASCII codes A and B, parsed and written, with the fraction to 18 digits
  and the `Z` optional, including the time part truncated from the right as
  §3.5.1.3(e) allows (`1988-018T17:20`); and `decode`, which reads a
  P-field and dispatches to the code it names.
- **Not carried**, with the reason:
  - *The ASCII calendar and time subsets used alone* (`1988-01-18`,
    `17:20:43`, and the left-truncated `-01-18`): they name a day or a time
    of day, not an instant, and §3.5.1.3(a) makes their reading depend on a
    context the code does not state.
  - *The contents of Level 3 and Level 4 codes*: the standard defines only
    their length. `decode` returns their octets.
  - *NASA's PB-5J*, annex E's example of an agency-defined code: the annex
    is informative and the code is "presently being considered".
  - *UTC's fractional steps before 1972*, which no whole-second code can
    express.

## Accuracy

Everything is exact but the CUC fraction's attosecond. Its unit is
2⁻⁸ⁿ s for *n* octets: a whole number of attoseconds for one or two, and
finer than that from three. The span a count reads as is rounded up to the
attosecond, and a span is written as the count at or before it, so a count
read and written again is the same count for up to seven octets, whose
unit, 2⁻⁵⁶ s, is still about 14 attoseconds. From eight octets the unit is
finer than the attosecond and neighbouring counts read as one span.

| Check | Test | Result |
| --- | --- | --- |
| The standard's ASCII A and B example, both ways | `the_standards_ascii_examples` | exact |
| The same instant in CCS, both variations | `the_ascii_example_in_ccs` | exact |
| The same instant in CDS, with a microsecond segment | `the_ascii_example_in_cds` | exact |
| CUC Level 1 count 0 is 1958-01-01T00:00:00 TAI; 2000-01-01 UTC is 1 325 376 032 s | `the_cuc_epoch_is_1958_tai`, `the_first_second_of_2000` | exact |
| 1958 January 1 is 2 922 days after 1950 January 1, annex B3.2 | `the_1950_agency_epoch` | exact |
| The leap second of 2016-12-31 in CDS, CCS and ASCII; the same second refused on 2016-12-30 | `the_2016_leap_second_in_every_utc_code` | exact |
| Every P-field octet decodes or is refused, and re-encodes to itself | `every_first_octet` | all 256 |
| CUC, CDS and CCS round-trip over a sample of instants and every format | `round_trips` | exact |

Annex B3.2 gives 1958 January 1 as "Julian date 2436203.5", and the
glossary repeats it. That Julian date is the start of 1957 December 31; the
start of 1958 January 1 is JD 2 436 204.5, as `ccsds-day` has it. The code
follows the date the text names.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [ccsds-301-0-b-4] | Every rule above: §§1.3, 1.5, 3.1–3.6, annexes A, B and D | Yes, the PDF, 2026-09-27 |

Annex D's table of TAI − UTC ends in 2009; the table used is the IANA
`leap-seconds.list` of `hc-core::leap` [iana-leap-seconds-list]. ISO 8601,
which annex B3.4 says the ASCII codes are subsets of, was not read for
this document.

## Code

`crates/hc-core/src/ccsds.rs`, `crates/hc-core/src/leap.rs`
(`end_of_day_step`), the epoch in `crates/hc-core/src/epoch.rs`, and
`crates/hc-format/src/ccsds.rs`. Anchors: `the_standards_ascii_examples`,
`the_ascii_example_in_ccs`, `the_ascii_example_in_cds`,
`the_cuc_epoch_is_1958_tai`, `the_2016_leap_second_in_every_utc_code`.
