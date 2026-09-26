# Binary timestamps: NTP eras, UUID versions 1 and 6, and FAT date words

Backs `hc-core::ntp`, `hc-core::uuid` with the epoch `uuid-gregorian`, and
`hc-format::fat`.

## What it is

Three fixed-width fields that protocols and file systems write a time
into. Each is a count from an epoch in a word too narrow to last, and the
interesting part of each is what happens at the edge of the word.

- The **NTP timestamp** of every NTP packet is 32 bits of seconds from
  1900 and 32 bits of fraction; the seconds wrap in February 2036. NTP's
  128-bit *date* format carries the era that the timestamp leaves out
  [rfc5905].
- **UUIDs of versions 1 and 6** carry a 60-bit count of 100 ns intervals
  from the Gregorian reform, 15 October 1582, split across the first eight
  octets in two different orders [rfc9562].
- The **MS-DOS date and time words** of the FAT file system pack a local
  wall-clock reading into two 16-bit values, years from 1980 and seconds at
  two-second resolution [ms-dosdatetimetofiletime; ms-file-times].

## How it works

**NTP.** A date is a signed 64-bit count *s* of seconds from the prime
epoch, "0 h 1 January 1900 UTC", divided into a 32-bit era number and a
32-bit era offset: "era = s / 2^(32) and timestamp = s - era * 2^(32),
which works for positive and negative dates" [rfc5905, §6] — the division
floors, so 31 December 1899 is era −1. The 64-bit timestamp is the era
offset without the era. "Eras cannot be produced by NTP directly"; they
come from outside, and §6 names the window in which they come out right:
"if the client is set within 68 years of the server", that is within 2³¹ s.
The seconds count whole 86 400-second days, as §6's table shows (1 January
1970 is 2 208 988 800), so a date is a POSIX-style label.

*Worked example.* NTP's own table gives 8 February 2036, MJD 64 731, as
era 1, offset 63 104. From MJD 15 020, 1 January 1900, that is 49 711 days,
4 295 030 400 s; less 2³² = 4 294 967 296 it is 63 104 into era 1. The
wrap itself is 2³² s after 1900, 06:28:16 UTC on 7 February 2036. The
64-bit timestamp 63 104 read against a clock in 2030 is that 2036 date;
against a clock in 1920 it is 17:31:44 on 1 January 1900.

**UUID.** Version 1 writes the 60-bit timestamp low bits first: octets 0–3
the low 32 bits, 4–5 the middle 16, and 6–7 the version nibble followed by
the high 12. Version 6 writes it high bits first: octets 0–3 the high 32,
4–5 the middle 16, 6–7 the version and the low 12 [rfc9562, §§5.1, 5.6].
Octet 8's top two bits are the variant, `0b10`. The count converts from
POSIX time as the RFC's own pseudocode does it: nanoseconds / 100 +
0x01B21DD213814000 (Appendix A).

*Worked example.* RFC 9562's test vectors both carry 0x1EC9414C232AB00,
Tuesday 22 February 2022 at 2:22:22 PM GMT−5, POSIX 1 645 557 742:
1 645 557 742 × 10⁷ + 122 192 928 000 000 000 = 138 648 505 420 000 000.
Version 1 is `C232AB00-9414-11EC-…`, the low word first and the high 12
bits `1EC` after the version `1`; version 6 is `1EC9414C-232A-6B00-…`,
the high 32 bits first and the low 12 bits `B00` after the version `6`.

**FAT.** The date word is the day of the month in bits 0–4, the month in
5–8 and the year less 1980 in 9–15; the time word is the second divided by
2 in bits 0–4, the minute in 5–10 and the hour in 11–15
[ms-dosdatetimetofiletime]. "The FAT file system records times on disk in
local time", and its write time "has a resolution of 2 seconds"
[ms-file-times].

*Worked example.* 26 September 2026 at 23:59:58, local time: year offset
46, month 9 and day 26 give the date word 46 × 512 + 9 × 32 + 26 =
23 866; hour 23, minute 59 and 58 / 2 = 29 give the time word
23 × 2048 + 59 × 32 + 29 = 49 021. Microsoft's pages give no dated
example, so this one is worked from the layout alone.

## What is carried

- **`hc-core::ntp`**: `NtpDate` (era, offset, 64-bit fraction) to and from
  `UnixTime`, and in its 16-byte layout; `NtpTimestamp` (seconds, 32-bit
  fraction) in its 8-byte layout, and `NtpTimestamp::resolve`, which takes
  a reference time and returns the date within 2³¹ s of it, the lower end
  included. The zero timestamp, which RFC 5905 reserves for "unknown or
  unsynchronized time", is refused by `resolve`.
- **`hc-core::uuid`** and the epoch **`uuid-gregorian`**: the timestamp to
  and from `UnixTime`, floored to 100 ns, refused before 1582-10-15 and
  past 2⁶⁰ − 1; `encode` writes a version 1 or 6 UUID from a timestamp and
  the caller's clock sequence and node, and `decode` reads the version and
  timestamp, refusing any other version or variant.
- **`hc-format::fat`**: `decode_date`, `decode_time` and `decode` to a
  `CivilDateTime` in no zone; `encode_date`, `encode_time` and `encode`
  from one, 1980 to 2107, keeping the even second at or before the
  reading.
- **Not carried**, with the reason:
  - *NTP's 32-bit short format*, a span of 16 bits of seconds used for
    delay and dispersion, which is not a time.
  - *A UUID's clock sequence and node*, which are not time; `encode` takes
    them as given.
  - *FAT's 10 ms creation-time byte and its access date*, and *exFAT's
    offset byte*, which Microsoft's page for these words does not
    describe. The zone of a FAT reading is not in the words and is not
    guessed.
  - *Leap seconds* in all three: each is a label counting 86 400 s a day,
    and RFC 9562 §6.1 allows a generator to smear them.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| All twelve rows of RFC 5905 Figure 4, with errata 4025 and 6524 applied | `figure_4_of_rfc_5905` | exact |
| The wrap at 2036-02-07T06:28:16Z and the first day of era 1 | `the_timestamp_wraps_on_7_february_2036` | exact |
| One timestamp resolved against 2030, 1990 and 1920 | `a_timestamp_is_resolved_against_a_reference` | exact |
| The ±2³¹ s window at both ends | `the_window_is_half_an_era_each_side` | exact |
| RFC 9562's timestamp and its POSIX second | `the_rfc_vector_timestamp_is_its_posix_second` | exact |
| The version 1 and version 6 test vectors, both ways | `the_version_1_vector`, `the_version_6_vector` | exact |
| The 60-bit field ends on 5236-03-31 21:21:00.6846975 UTC | `the_count_starts_on_15_october_1582_and_ends_in_60_bits` | exact |
| FAT's fields in their bits, 1980 to 2107, invalid fields refused | `each_field_is_in_its_bits`, `the_years_run_from_1980_to_2107`, `fields_outside_their_range_are_refused` | exact |

Two of the sources contradict themselves, and their errata decide:

- RFC 5905's Figure 4 prints the MJD of 1 January 0 as −678 491 and the
  era offset of 1 January 1 as 202 939 144. Errata 4025 and 6524, held for
  the next revision, give −678 941 and 202 934 144, which the rest of the
  row and the table agree with [rfc5905-errata].
- RFC 9562 §5.1 says version 1's `time_high` holds "the least significant
  12 bits", and §6.1 that the field lasts "until 5623 AD". Its own test
  vector puts the most significant bits there, and the count ends in 5236;
  verified errata 7955 and 8288 correct both [rfc9562-errata].

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [rfc5905] | The date and timestamp formats, eras, the 68-year window, Figure 4 | Yes, rfc-editor.org, 2026-09-26 |
| [rfc5905-errata] | Errata 4025 and 6524 to Figure 4 | Yes, 2026-09-26 |
| [rfc9562] | The UUID timestamp, both layouts, the test vectors | Yes, rfc-editor.org, 2026-09-26 |
| [rfc9562-errata] | Errata 7955 and 8288 | Yes, 2026-09-26 |
| [ms-dosdatetimetofiletime] | The FAT date and time layouts | Yes, Microsoft Learn, 2026-09-26 |
| [ms-file-times] | FAT's local time and two-second resolution | Yes, Microsoft Learn, 2026-09-26 |

The FAT32 specification (`fatgen103`) was not read.

## Code

`crates/hc-core/src/ntp.rs`, `crates/hc-core/src/uuid.rs` and the epoch in
`crates/hc-core/src/epoch.rs`, and `crates/hc-format/src/fat.rs`. Anchors:
`figure_4_of_rfc_5905`, `the_version_1_vector`, `the_version_6_vector`,
`each_field_is_in_its_bits`.
