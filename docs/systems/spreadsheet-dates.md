# Spreadsheet serial dates: Excel 1900, Excel 1904 and the OLE Automation date

Backs the identifiers `excel-1900`, `excel-1904` and `ole-automation-date`
in `hc-calendars-solar`.

## What it is

Spreadsheets store a date as a number of days, a *serial*, and a time as
its fraction. Lotus 1-2-3 counted 1900 as a leap year, which it was not;
Multiplan and Excel kept the same serials so that worksheets would move
between the programs, and Microsoft has kept them since, because
correcting the count would move almost every stored date by a day and
change what WEEKDAY returns [ms-excel-1900-leap]. The Macintosh Excel
counted from 1904 instead, which avoids the missing day; both systems are
still selectable per workbook [ms-excel-date-systems]. The OLE Automation
date, the `DATE` type of COM and `DateTime.ToOADate` in .NET, is the 1900
serial from March 1900 on, extended backwards through 1899 by a count that
has no phantom day in it [ms-tooadate].

## How it works

**Excel 1900.** Serial 1 is 1 January 1900. Serials 1 to 59 are
1 January to 28 February 1900. Serial 60 is displayed as 29 February 1900,
a day the Gregorian calendar does not have. Serial 61 is 1 March 1900, and
from there each serial is one more than the count of days from 31 December
1899 [ms-excel-1900-leap]. Microsoft notes that the WEEKDAY function
"returns incorrect values for dates before March 1, 1900": the serial
steps through a day that did not happen, so the weekday cycle that holds
from March, read backwards, is a day early before it.

**Excel 1904.** Serial 0 is 1 January 1904, and "the serial number of a
date in the 1900 date system is always 1,462 days greater than the serial
number of the same date in the 1904 date system" [ms-excel-date-systems].
The 1 462 is four years and a day, 1 January 1900 to 1 January 1904 being
1 460 days, plus serial 1's offset from zero and the phantom day.

**OLE Automation.** A floating-point number whose integral part is "the
number of days before or after midnight, 30 December 1899, and whose
fractional component represents the time on that day divided by 24"
[ms-tooadate]. The fraction of a negative value is a time of day, not a
subtraction: −1.25 is 06:00 on 29 December 1899, the day −1 plus a quarter
day. The supported range is midnight on 1 January 100 to the end of
31 December 9999.

**Worked example.** 5 July 2011. From 30 December 1899 to 5 July 2011 is
40 729 days, so its OLE day and its Excel 1900 serial are both 40 729, and
its Excel 1904 serial is 40 729 − 1 462 = 39 267, the numbers Microsoft
Support gives [ms-excel-date-systems]. For 1 January 1900 the OLE day is 2,
the Excel 1900 serial 1: the two counts part at 1 March 1900, where Excel
has counted one day more than elapsed.

## What is carried

- **`excel-1900`**, `spreadsheet::Excel1900Calendar`: serials 1 to
  2 958 465, 1 January 1900 to 31 December 9999. Serial 60 is refused with
  `DayOutOfRange`; `spreadsheet::excel_1900_day` names it instead, as
  `Phantom29February1900`, for a caller reading a worksheet that holds one.
  No day maps to serial 60. Serial 0, which Excel shows as "January 0,
  1900", is outside the range.
- **`excel-1904`**, `day_counts::EXCEL_1904`: serial 0 to 2 957 003,
  1 January 1904 to 31 December 9999.
- **`ole-automation-date`**, `day_counts::OLE_AUTOMATION`: the whole-day
  count, 1 January 100 to 31 December 9999, and
  `spreadsheet::ole_automation` and `to_ole_automation`, which read and
  write the fractional value with the time of day as a `Duration`. The
  values in (−1, 0) repeat the times of 30 December 1899 that [0, 1) gives;
  writing gives the non-negative one.
- **Not carried**, with the reason:
  - *Excel's WEEKDAY*, which is Excel's function, not the date. The
    calendar's day is the true one.
  - *The fractional time of the Excel serials*, which is the OLE reading
    from March 1900 and has no negative values; the day is what the
    calendars carry.
  - *Rounding of the fraction.* The time of day is the `f64` as it stands;
    how Excel or .NET round it to milliseconds is not stated by the pages
    read.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| Serials 1, 59 and 61 are 1 January, 28 February and 1 March 1900; 60 refused and named; no day maps to 60 | `serial_60_is_refused_and_named` | exact |
| Serial 0 and 2 958 466 refused; serials round-trip over the range | `the_range_is_serial_1_to_9999` | all sampled |
| 5 July 2011 is 40 729; the 1904 system is 1 462 behind from 1904 to 9999 | `the_1904_system_is_1462_behind` | exact |
| 5 July 2011 is 39 267 in the 1904 system; its ends | `the_vendor_counts_refuse_what_the_vendor_does_not_support` | exact |
| The weekday read off the serial is a day early before March 1900 | `a_weekday_read_off_the_serial_is_a_day_early_before_march_1900` | serials 1–59 |
| Microsoft Learn's 1.0, 2.25, −1.0 and −1.25 | `ole_automation_reads_a_negative_fraction_as_a_time_of_day` | exact, both ways |
| The OLE Automation day is JDN − 2 415 019 and the 1904 serial JDN − 2 416 481 | `the_offsets_from_the_julian_day_number_are_the_published_ones` | exact |

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [ms-excel-1900-leap] | The 29 February 1900 of Lotus 1-2-3 and Excel; WEEKDAY before March 1900 | Yes, Microsoft Learn, 2026-09-26 |
| [ms-excel-date-systems] | The 1900 and 1904 systems, the 1 462 days, the 5 July 2011 example | Yes, Microsoft Support, 2026-09-26 |
| [ms-tooadate] | The OLE Automation date, its examples and range | Yes, Microsoft Learn, 2026-09-26 |

Weir, "Leap Back" (2006), which quotes ECMA-376 on the 1900 system, was
not read, and neither was ECMA-376.

## Code

`crates/hc-calendars-solar/src/spreadsheet.rs` for `excel-1900` and the
OLE fraction; `crates/hc-calendars-solar/src/day_counts.rs` for
`excel-1904` and `ole-automation-date`. Anchors:
`serial_60_is_refused_and_named`, `the_1904_system_is_1462_behind`,
`ole_automation_reads_a_negative_fraction_as_a_time_of_day`.
