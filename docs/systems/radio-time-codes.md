# Radio time codes: JJY, DCF77, and WWVB's amplitude and phase codes

Backs `hc-format::radio`: `jjy`, `dcf77` and `wwvb`.

## What it is

Three long-wave stations broadcast a time code that a radio-controlled
clock decodes, one frame of one symbol a second every minute:

| Station | Operator | Carrier | Time it carries | Source |
| --- | --- | --- | --- | --- |
| **JJY** | NICT, Japan | 40 kHz and 60 kHz | JST, "協定世界時(ＵＴＣ)を９時間進めたもの" (UTC nine hours ahead) | [nict-jjy-timecode] |
| **DCF77** | PTB, Germany | 77.5 kHz [ptb-dcf77-carrier] | CET or CEST, the frame saying which | [ptb-dcf77-timecode] |
| **WWVB**, amplitude code | NIST, United States | 60 kHz | UTC | [nist-sp432-2002; nist-sp250-67] |
| **WWVB**, phase code | NIST | the same carrier, phase-modulated | UTC | [nist-wwvb-enhanced-2013] |

Each station marks a symbol by how long it lowers the carrier, and the
codes differ in which minute a frame names, in the order of their bits, and
in how they announce a leap second or a change of summer time.

## How it works

### JJY

A frame is 60 symbols: a marker (0.2 s), a binary 1 (0.5 s) or a binary 0
(0.8 s) [nict-jjy-timecode]. The frame names "１周期の先頭マーカー（Ｍ）の時刻"
— the minute that begins at its first marker. The marker M is second 0,
and the position markers P1–P5 and P0 are seconds 9, 19, 29, 39, 49 and 59.

| Seconds | Field |
| --- | --- |
| 1–3, 5–8 | minute, BCD, weights 40 20 10 and 8 4 2 1 |
| 12–13, 15–18 | hour of JST, 20 10 and 8 4 2 1 |
| 22–23, 25–28, 30–33 | day of the year, 1 January being 1: 200 100, 80 40 20 10, 8 4 2 1 |
| 36, 37 | PA1 and PA2, even parity of the hour's and the minute's bits |
| 38, 40 | SU1 and SU2, spare: NICT sets both to 0 "利用方法が決まるまでは", until their use is decided |
| 41–48 | the last two digits of the year, 80 … 1 |
| 50–52 | the weekday, 0 Sunday to 6 Saturday, 4 2 1 |
| 53, 54 | LS1 and LS2: 00 no leap second within a month, 11 an inserted one, 10 an omitted one |
| 4, 10, 11, 14, 20, 21, 24, 34, 35, 55–58 | 0 |

At minutes 15 and 45 the frame carries the call sign instead: seconds 40–48
are the call sign in Morse, 50–52 announce a stop of the transmitter
(ST1–ST3) and 53–55 its length (ST4–ST6), and 38 and 56–58 are 0. Those
frames carry no year, weekday or leap-second bits.

A leap second is inserted or omitted before 09:00 JST on the first day of a
month. In an inserted one P0 falls at second 60 and second 59 is a binary 0;
in an omitted one P0 falls at second 58. LS1 and LS2 are shown from 09:00
JST on the 2nd of the month before until 08:59 on the 1st.

*Worked example.* NICT's figure is the frame of 17:25 JST on day 92 of
2004, 1 April, a Thursday, with no leap second within a month. The minute
25 is `010` `0101` in seconds 1–3 and 5–8; the hour 17 is `01` `0111` in
12–13 and 15–18; day 092 is `00`, `1001`, `0010`. The hour's bits hold four
ones, so PA1 is 0; the minute's hold three, so PA2 is 1. The year 04 is
`00000100` and the weekday 4, Thursday, is `100`.

### DCF77

A second mark of 0.1 s is a binary 0 and one of 0.2 s a binary 1. Second
59 has no mark: PTB's figure shows none, and its text calls the mark sent
there in a leap-second minute "different than before" [ptb-dcf77-timecode].

| Seconds | Field |
| --- | --- |
| 0 | M, "transmitted as binary zero" |
| 1–14 | "provided by a third party": civil warnings and Meteo Time's weather data, for which "PTB explicitly denies any responsibility" |
| 15 | R, the call bit, "used to signalize irregularities in the control facilities and to alarm PTB" |
| 16 | A1, a change between CET and CEST at the end of this hour |
| 17, 18 | Z1 and Z2: 01 for CET, 10 for CEST |
| 19 | A2, a leap second at the end of this hour |
| 20 | S, the start of the time, always 1 |
| 21–27, 28 | minute, BCD least significant bit first, weights 1 2 4 8 10 20 40; P1 |
| 29–34, 35 | hour, 1 2 4 8 10 20; P2 |
| 36–41 | day of the month, 1 2 4 8 10 20 |
| 42–44 | weekday, ISO 8601, Monday 1 to Sunday 7, 1 2 4 |
| 45–49 | month, 1 2 4 8 10 |
| 50–57, 58 | the last two digits of the year, 1 2 4 8 10 20 40 80; P3 |

"Each code emitted contains the information for the following minute": the
frame sent from 12:29:00 names 12:30. The parity bits "complement the
preceding information words (7 bits for the minute, 6 bits for the hour and
22 bits for the date including the number of the weekday) to an even number
of ones". A1 is sent for the hour before a change, "from 01:00:16 h CET
(02:00:16 h CEST) until 01:59:16 h CET (02:59:16 h CEST)", and A2 for the
hour before a leap second, "from 00:00:19 h CET (01:00:19 h CEST) until
00:59:19 h CET (01:59:19 h CEST)". In the minute of an inserted leap second
"the 59th second mark … is emitted … with a duration of 0.1 s. After that,
the inserted 60th second mark is emitted without carrier reduction": that
frame has 60 marks, the last a 0.

*Worked example.* PTB's page gives no dated frame, so this one is worked
from the layout. The frame sent from 14:29:00 CEST on Sunday 27 September
2026 names 14:30 CEST. Z1 Z2 are `10`. The minute 30 is 20 + 10, bits
`0000110` in seconds 21–27, two ones, so P1 is 0. The hour 14 is 10 + 4,
`001010`, two ones, P2 0. The day 27 is `111001`, the weekday 7 `111`, the
month 9 `10010` and the year 26 `01100100`: 4 + 3 + 2 + 3 = 12 ones, so P3
is 0.

### WWVB, amplitude code

The carrier is lowered at the start of each second and restored after
0.2 s for a 0, 0.5 s for a 1 and 0.8 s for a marker [nist-sp432-2002]. The
frame names the minute that begins at its first marker ("on time point A"),
UTC, most significant bit first [nist-sp250-67].

| Seconds | Field |
| --- | --- |
| 0, 9, 19, 29, 39, 49, 59 | markers: the frame reference marker at 0, P1–P5, and P0 |
| 1–3, 5–8 | minute, 40 20 10 and 8 4 2 1 |
| 12–13, 15–18 | hour, 20 10 and 8 4 2 1 |
| 22–23, 25–28, 30–33 | day of the year, 200 100, 80 40 20 10, 8 4 2 1 |
| 36–38 | the sign of UT1 − UTC: 1 0 1 positive, 0 1 0 negative |
| 40–43 | its magnitude in tenths of a second, 0.8 0.4 0.2 0.1 |
| 45–48, 50–53 | the last two digits of the year, 80 40 20 10 and 8 4 2 1 |
| 55 | the leap year indicator |
| 56 | the leap second warning: "a leap second will be added to UTC at the end of the current month" |
| 57, 58 | summer time: 00 standard time, 11 daylight saving time; bit 57 changes "at 0000 UTC on the day" of a change and bit 58 24 hours later |
| 4, 10, 11, 14, 20, 21, 24, 34, 35, 44, 54 | 0 |

SP 250-67 says the leap year bit is set "usually sometime in January but
before February 29", so a frame early in a leap year may still carry 0.
The leap second warning is set "near the beginning of the month" and reset
"immediately after the leap second occurs". For an inserted leap second,
"the time frame representing the extended 61-second minute, starting at
23:59:00UTC, will have bit 59 repeated (a marker in the legacy broadcast
and a '0' in PM)"; for an omitted one it "will have bit 59 removed"
[nist-wwvb-enhanced-2013, §4.4].

*Worked example.* SP 432's figure decodes as 2001, day 258, 18:42 UTC, with
UT1 − UTC = −0.7 s, so UT1 is 18:41:59.3. The NIST phase-code document's
Table 10 gives the amplitude bits of 17:30 UTC on 4 July 2012: minute
`011` `0000`, hour `01` `0111`, day `01` `1000` `0110` (186), UT1 sign
`101` and magnitude `0100` (+0.4 s), year `0001` `0010`, leap year 1, leap
second warning 0, summer time `11`.

### WWVB, phase code

The carrier's phase is also inverted for a 1 and left alone for a 0, one
bit a second, without disturbing the amplitude code
[nist-wwvb-enhanced-2013]. A time frame is:

| Seconds | Field |
| --- | --- |
| 0–12 | the synchronisation word `0011101101000` (sync_T) |
| 13–17 | five Hamming parity bits, time_par[4..0] |
| 18, 20–28, 30–38, 40–46 | the 26-bit minute count, time[25..0], most significant first |
| 19 | time[0] again |
| 29, 39 | reserved |
| 47–48, 50–52 | dst_ls[4..0], summer time and leap second (Table 4) |
| 49 | notice: NIST has posted a notice on its website |
| 53–58 | dst_next[5..0], the schedule of the next summer-time change (Table 8) |
| 59 | 0 |

The minute count is "the number of minutes that have elapsed since
00:00UTC on January 1st in the year 2000", "reset at the beginning of year
XX00", counting 60 minutes an hour and 24 hours a day. The parity bits are
sums modulo 2 of fifteen bits each of the count, as §4.3 lists them.
Table 4 maps twelve 5-bit words to a summer-time state, as bits 57–58 of
the amplitude code, and a leap-second notice: none, an omitted second, or
an inserted one "at the end of the current month", set at 00:00 UTC on the
first of the month. Table 8 maps 6-bit words, read with the summer-time
state, to the day and local hour of the next change: one of eight Sundays
counted from the first Sunday of March or of November, at 1, 2 or 3 AM,
or one of eight messages. A frame with the other synchronisation word,
sync_M, is a message and carries no time; at 10 and 40 minutes past each
hour a six-minute sequence replaces six frames.

*Worked example.* The document's Table 10 is 17:30 UTC on 4 July 2012.
From 2000-01-01 that is 4 568 days and 17 h 30 min: 4 568 × 1 440 + 1 050 =
6 578 970 minutes, binary `00011001000110001100011010`. The parity bits are
`10010`. Summer time has been in effect for over a day and no leap second
is due, `00011`. The next change is out of summer time on the first Sunday
of November at 2 AM, `011011` with the state's first bit 1.

## What is carried

- **`hc-format::radio`**: the symbols (`Symbol`: 0, 1, marker), a frame
  buffer long enough for a leap second, and for each code a decoded frame
  with `decode` and `encode`, and a reading in the code's own time:
  - `jjy::JjyFrame`: minute, hour, day of the year and, in an ordinary
    frame, the year, weekday, LS1 LS2 and SU1 SU2, or at 15 and 45 minutes
    the stop notice; `reading` gives the JST minute from the century the
    caller names, and `reading_in_year` from a year, which a call-sign
    frame needs; `to_unix` subtracts nine hours.
  - `dcf77::Dcf77Frame`: the fourteen third-party bits passed through as
    they came, the call bit, A1, the zone, A2, and the date and time of the
    minute the frame announces; `reading` and `to_unix`.
  - `wwvb::AmFrame`: the minute, hour, day of the year, year, UT1 − UTC in
    tenths, the leap year and leap second bits and the summer-time state;
    `reading` and `to_unix`.
  - `wwvb::PmFrame`: the minute count, the summer-time state, the leap
    second notice, the notice bit, the reserved bits and the next change,
    decoded through Tables 4 and 8; `reading` and `to_unix`, and
    `DstTransition::day`, the date a schedule names in a year.
- Decoding refuses a symbol of the wrong kind in a fixed place, a BCD digit
  above 9, a parity bit that does not match, a day the year does not have,
  a weekday that is not the date's, and a frame of 59 or 61 seconds except
  at the leap second's place with its announcement set. The two-digit
  years need a century, and the caller names it, since a frame does not
  say which century it is.
- **Not carried**, with the reason:
  - *The content of DCF77's bits 1–14*: PTB leaves them to a third party
    and publishes no layout; the weather code is Meteo Time's, which PTB's
    page links but does not describe.
  - *JJY's call sign*, a Morse signal in 40–48 that is not a bit.
  - *WWVB's message frames and six-minute sequences*, which carry no time
    frame; a sync_M frame is refused.
  - *Correction of a WWVB phase frame by its Hamming code*: a frame whose
    parity does not match is refused, not repaired.
  - *The station's own schedule for an announcement*: each `encode` writes
    the notices it is given, as a transmitter's operator sets them.
  - *The analogue side*: pulse widths, carrier phase and the detection of
    a second's start are the receiver's.
  - *DCF77's omitted leap second.* PTB calls one "negligible" and adds
    that "the technical facilities on the transmitter allow it", but does
    not say how the minute's marks would be sent, so a frame of 58 marks
    is refused as a length.
  - *MSF*, the British station, whose specification was not read.

## Accuracy

The codes are exact; the checks are the stations' own examples and the
round trip of every field.

| Check | Test | Result |
| --- | --- | --- |
| NICT's frame of 2004 day 92, 17:25 JST, both ways | `nicts_example_frame` | exact |
| A JJY call-sign frame at 17:15, with a stop notice | `a_call_sign_frame_needs_the_year` | exact |
| JJY's inserted and omitted leap second at 08:59 JST on the 1st | `jjy_leap_second_frames` | exact |
| DCF77's frame for 14:30 CEST on 27 September 2026, and the leap second of 1 January 2017 | `a_dcf77_frame_for_the_following_minute`, `the_dcf77_leap_second_frame` | exact |
| SP 432's figure 2.6: the decoded 2001, day 258, 18:42 UTC and UT1 − UTC = −0.7 s, and seconds 36–43, the UT1 sign and magnitude; the figure's other symbols are not transcribed | `sp_432s_frame` | exact |
| WWVB's amplitude frame of 4 July 2012, all 60 symbols of the phase-code document's Table 10 | `the_2012_example_in_both_codes` | exact |
| WWVB's phase frame of 4 July 2012, all 60 bits of Table 10 | `the_2012_example_in_both_codes` | exact |
| The minute count of 21:30 UTC on 28 July 2016, 8 717 610 | `the_minute_count_of_28_july_2016` | exact |
| Tables 4 and 8 read in both directions; their words distinct | `table_4_round_trips`, `table_8_round_trips` | all 12 and 56 |
| Every code round-trips at three or five minutes of every day, 2000–2099 | `every_code_round_trips` | exact; every day in a release build, in a debug one every 83rd to 97th and every month's first and last |
| Parity, BCD, marker and length errors are refused | `broken_frames_are_refused` | each refused |
| Field errors: a reading off the minute, a UT1 − UTC past ±0.9 s, a stop notice, call sign, minute count or `dst_next` the code cannot carry, a DCF77 weekday of 0, 60 marks without A2, and a 1 at WWVB's phase bits 59 and 60 | `field_refusals` in `jjy`, `dcf77` and `wwvb` | each refused |

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [nict-jjy-timecode] | JJY's symbols, layout, parity, leap second and stop notice | Yes, jjy.nict.go.jp, the page "標準電波の出し方" and its two figures, 2026-09-27 |
| [ptb-dcf77-timecode] | DCF77's layout, parity, A1, A2, Z1 Z2, the leap second, and the omitted one left out | Yes, ptb.de, the page "DCF77 time code" and its figure, 2026-09-27 |
| [ptb-dcf77-carrier] | DCF77's carrier, 77.5 kHz | Yes, ptb.de, the page "DCF77 carrier frequency", 2026-09-27 |
| [nist-sp432-2002] | WWVB's amplitude code: the bits, UT1, the leap year and leap second bits, summer time, the 2001 example | Yes, the PDF, 2026-09-27 |
| [nist-sp250-67] | The same, with the leap year bit's timing | Yes, the PDF, chapter 2 §2, 2026-09-27 |
| [nist-wwvb-enhanced-2013] | The phase code: layout, minute count, parity, Tables 3, 4, 8 and 10, leap seconds in both codes | Yes, revision 1.01, the PDF, 2026-09-27 |

NIST's page "WWVB Time Code Format" holds only SP 432's figure. The
survey's secondary sources, the Wikipedia articles on the three stations,
were not used for any rule.

## Code

`crates/hc-format/src/radio/mod.rs`, `jjy.rs`, `dcf77.rs` and `wwvb.rs`.
Anchors: `nicts_example_frame`, `sp_432s_frame`,
`the_2012_example_in_both_codes`, `the_minute_count_of_28_july_2016`,
`the_dcf77_leap_second_frame`.
