# IRIG serial time codes A, B, D, E, G and H

Backs `hc-format::irig`.

## What it is

IRIG Standard 200 defines six serial time codes that carry the time of
year from a timing system to the equipment that records data against it:
test ranges, tracking stations and data-handling systems
[rcc-200-16]. The United States ranges keep "UTC referenced to the United
States Naval Observatory (USNO) Master Clock" (chapter 1), but the code
itself names no time scale: a generator sends the clock it is set to.

The six formats are one design at six rates (Tables 3-1 and 3-2):

| Format | Pulses | Index count | Frame |
| --- | --- | --- | --- |
| A | 1 000 a second | 1 ms | 0.1 s |
| B | 100 a second | 10 ms | 1 s |
| D | 1 a minute | 1 min | 1 h |
| E | 10 a second | 0.1 s | 10 s |
| G | 10 000 a second | 0.1 ms | 10 ms |
| H | 1 a second | 1 s | 1 min |

## How it works

A frame is one pulse per index count: 100 counts in A, B, E and G, 60 in
D and H. The width of a pulse is its meaning (§3.6, §3.9):

| Width, of the index count | Symbol |
| --- | --- |
| 0.2 | a binary 0, or an index marker |
| 0.5 | a binary 1 |
| 0.8 | the reference bit Pr, or a position identifier |

Pr is count 0, and its leading edge is the instant the frame names. The
position identifiers P1, P2 … fall at counts 9, 19, 29 …, and P0 is the
last count of the frame, so that P0 and the next Pr make two long pulses
in a row. Every count that no field uses is an index marker.

The fields, by index count (Tables 5-1, 5-4, 5-7, 5-9, 5-12 and 5-15).
Each BCD digit is sent least significant bit first, weights 1 2 4 8:

| Field | A | B | D | E | G | H |
| --- | --- | --- | --- | --- | --- | --- |
| units of seconds | 1–4 | 1–4 | — | — | 1–4 | — |
| tens of seconds | 6–8 | 6–8 | — | 6–8 | 6–8 | — |
| minutes | 10–13, 15–17 | 10–13, 15–17 | — | 10–13, 15–17 | 10–13, 15–17 | 10–13, 15–17 |
| hours | 20–23, 25–26 | 20–23, 25–26 | 20–23, 25–26 | 20–23, 25–26 | 20–23, 25–26 | 20–23, 25–26 |
| day of the year | 30–33, 35–38, 40–41 | the same | the same | the same | the same | the same |
| tenths of seconds | 45–48 | — | — | — | 45–48 | — |
| hundredths of seconds | — | — | — | — | 50–53 | — |
| year, last two digits | 50–53, 55–58 | 50–53, 55–58 | — | 50–53, 55–58 | 60–63, 65–68 | — |
| control bits | 60–68, 70–78 | 60–68, 70–78 | 50–58 | 60–68, 70–78 | 70–78, 80–88, 90–98 | 50–58 |
| straight binary seconds of the day | 80–88, 90–97 | 80–88, 90–97 | — | — | — | — |

The straight binary seconds (SBS) are the seconds since midnight in 17
bits, 2⁰ at count 80 and 2¹⁶ at count 97. The year "cycles to the next
year on January 1st of each year and will count to year 2099" (§3.6). The
control bits have no standard meaning: their use "is left to the end user
of the time codes" (§4.1).

Which fields a signal carries is the last digit of its designation, a
format letter and three digits (Figure 4-1): the modulation (0 pulse
width, 1 amplitude-modulated sine wave, 2 Manchester), the carrier
frequency and resolution, and the *coded expression*:

| Expression | Fields |
| --- | --- |
| 0 | BCD time of year, control bits, SBS |
| 1 | BCD time of year, control bits |
| 2 | BCD time of year |
| 3 | BCD time of year, SBS |
| 4 | BCD time of year, year, control bits, SBS |
| 5 | BCD time of year, year, control bits |
| 6 | BCD time of year, year |
| 7 | BCD time of year, year, SBS |

Table 4-1 limits each format's digits: A and B take every expression, E
and G take 1, 2, 5 and 6, and D and H take 1 and 2. "No other combinations
are standard." A field the expression leaves out is sent as index
markers.

*Worked example.* Figure 5-2 draws the IRIG B frame of day 173, 21:18:42,
year 03. Counts 1–4 are `0100`, 2 units of seconds; 6–8 are `001`, 4
tens; minutes 18 are `0001` and `100` at 10–13 and 15–17; hours 21 are
`1000` and `01`; day 173 is `1100`, `1110` and `10`. The year 03 is `1100`
at 50–53. The SBS are 21 × 3 600 + 18 × 60 + 42 = 76 722, which is
2¹ + 2⁴ + 2⁵ + 2⁷ + 2⁸ + 2⁹ + 2¹¹ + 2¹³ + 2¹⁶: counts 80–88 read `010011011`
and 90–97 `10101001`. The control bits are all 0. Written with `M` for a
long pulse, Pr first:

```text
M01000001M 000101000M 100000100M 110001110M 100000000M
110000000M 000000000M 000000000M 010011011M 101010010M
```

Day 173 of 2003 is 22 June.

## What is carried

`hc_format::irig`:

- `IrigFormat`, the six formats with their frame length, index count,
  frame time and number of control bits.
- `IrigCode`, a format and a coded expression that Table 4-1 permits, from
  the two or from a signal designation such as `B122`, whose modulation
  and frequency digits are checked against Table 4-1 and then dropped.
- `IrigFrame`: `decode` from a slice of `Symbol`s, `encode` to one, and
  `for_reading`, the frame whose Pr falls at a date and time. Decoding
  checks every marker, every index marker, every BCD digit, and that the
  SBS agree with the BCD time. `reading` reads the two-digit year in the
  century the caller names; `reading_in_year` takes the year for a code
  that carries none.

Deliberate choices:

- **Format E has no SBS.** §3.7 and Table 5-9 lay SBS out for E, but Table
  4-1 permits E only expressions 1, 2, 5 and 6, none with SBS. The module
  follows Table 4-1, and E's counts 80–98 are index markers.
- **The leap second.** On a leap-second day the SBS do not return to 0 at
  24:00 (§3.6), so at 23:59:60 they read 86 400. The standard lays out no
  leap-second frame; the module writes second 60 in BCD and reads it only
  at the end of a month, as `hc-format::radio` does.
- **No time scale.** A reading is a date and a time of day. The module
  gives no POSIX time, because the code does not say which clock it
  carries.

Not carried: the pulse widths and the carrier (modulation, Manchester
coding, mark-to-space ratio), which are how the symbols are sent; and the
parallel codes of IRIG Standard 205, which the RCC publishes only to its
members.

## Accuracy

The codes are exact. The checks are the standard's figures, read pulse by
pulse, and the round trip of every permitted code.

| Check | Test | Result |
| --- | --- | --- |
| Figures 5-1 (A), 5-2 (B), 5-3 (D), 5-5 (G) and 5-6 (H): day 173 of 2003, 21:18:42.8, 21:18:42, hour 21, 21:18:42.80 and 21:24, both ways | `the_standards_figures` | exact |
| Figure 5-4 (E) draws Figure 5-2's pulses, with units of seconds and SBS that E does not carry; its note puts the frame at 21:18:40. The drawn frame is refused, and the frame of 21:18:40 is written | `figure_5_4_is_not_an_e_frame` | refused; written |
| Signal designations checked against Table 4-1, and Figure 4-1's A137 | `signal_designations_follow_table_4_1` | exact |
| Control bits at their counts, and refused where the code has none | `control_bits` | exact |
| 23:59:60 on 31 December 2016, SBS 86 400, and refused on the 30th | `a_leap_second` | exact |
| Marker, index-marker, digit, SBS, length, year and day errors | `broken_frames_are_refused` | each refused |
| Every permitted code at four times of every day of 2000–2099 | `every_code_round_trips` | exact; every day in a release build, every 89th and every month's first and last in a debug one |

The figures' pulses were measured from the standard's PDF, drawn at six
times its size, as the width of each pulse against the index count.

Two errors in the standard were met. Table 5-5 gives the weights of IRIG
B's counts 57 and 58 as 20 and 20; Table 5-4 and Figure 5-2 give 40 and
80, which the module uses. Figure 5-4 is noted above.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [rcc-200-16] | Everything: the formats, the pulse widths, the field layouts, Table 4-1, the SBS, the year, and Figures 4-1 and 5-1 to 5-6 | Yes, the PDF from irig106.org, 2026-09-28 |

The survey's source, Wikipedia's "IRIG timecode", was not used for any
rule.

## Code

`crates/hc-format/src/irig.rs`, with `Symbol`, `Frame` and the BCD
helpers shared from `crates/hc-format/src/radio/mod.rs`. Anchors:
`the_standards_figures`, `figure_5_4_is_not_an_e_frame`.
