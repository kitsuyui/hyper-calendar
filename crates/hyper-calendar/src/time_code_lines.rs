//! The tab-separated lines the WebAssembly module and the C library write
//! about time codes and clock readings, written once.
//!
//! * The binary CCSDS time codes, CUC, CDS and CCS, read from and written
//!   to their octets as hexadecimal, P-field first, from
//!   [`hc_format::ccsds`] and [`hc_core::ccsds`]; and the ASCII codes A
//!   and B parsed and written. Every instant is a TAI one, whole seconds
//!   from 1970-01-01 00:00:00 TAI and attoseconds, as in
//!   [`crate::time_lines`], and every line gives it again as a UTC label,
//!   the POSIX second, the leap-second flag and the attoseconds, from the
//!   leap-second table under the caller's `strict`.
//! * The frames of the long-wave time stations, JJY, DCF77 and both WWVB
//!   codes, read from and written as a string of symbols, from
//!   [`hc_format::radio`].
//! * The IRIG serial time codes A, B, D, E, G and H, a frame read and
//!   written in the radio codes' string form, from [`hc_format::irig`].
//! * .NET's `DateTime.Ticks` to and from POSIX time, from
//!   [`hc_core::dotnet`].
//! * The Ethiopian and Swahili six-hour readings of the civil clock, from
//!   [`hc_format::east_african_hours`].
//!
//! The numbers are read by hand, digit by digit, so that the layer carries
//! no general float parser.

use alloc::string::String;
use core::fmt::Write;

use hc_calendar::{CivilDateTime, CivilTime, Rd, gregorian};
#[cfg(feature = "tz")]
use hc_core::UnixTime;
use hc_core::ccsds::{CdsTime, CucTime, EpochLevel, Octets, Preamble};
use hc_core::dotnet::{DateTimeKind, DotnetDateTime};
use hc_core::unix::{self, LeapPolicy, UtcInstant};
use hc_core::{Instant, Tai, TimeError};
use hc_format::ValueError;
use hc_format::ccsds::{self, AsciiPrecision, AsciiTime, AsciiVariation, CcsTime, CcsdsCode};
use hc_format::east_african_hours::{self, Half, HourReading, Reckoning};
#[cfg(feature = "tz")]
use hc_format::hc_tz::TimeZone;
use hc_format::irig::{self, IrigCode, IrigFrame};
#[cfg(feature = "tz")]
use hc_format::radio::dcf77::summer_time;
use hc_format::radio::dcf77::{Dcf77Frame, Zone};
use hc_format::radio::jjy::{JjyContent, JjyFrame};
use hc_format::radio::wwvb::{AmFrame, DstNext, DstState, PmFrame};
use hc_format::radio::{Code as RadioCode, FrameError, LeapNotice, Symbol};

use crate::boundary::{Answer, Line, Refusal, line};
use crate::time_lines::{hex_into, tai_instant, tai_parts, unix_instant};

/// The leap-second policy a `strict` flag asks for.
const fn policy(strict: bool) -> LeapPolicy {
    if strict {
        LeapPolicy::Strict
    } else {
        LeapPolicy::Extrapolate
    }
}

/// The refusal a CCSDS value error is: past the leap-second table
/// [`Refusal::NoData`], otherwise octets or text that are not a code.
fn code_refusal(error: ValueError) -> Refusal {
    match error {
        ValueError::Time(TimeError::AfterModelEnd | TimeError::BeforeModelStart) => Refusal::NoData,
        _ => Refusal::Malformed,
    }
}

/// The most octets a binary CCSDS code has: a CUC code's two P-field
/// octets and seventeen of T-field.
const MAX_CODE_OCTETS: usize = Octets::CAPACITY;

/// Octets written as hexadecimal, two digits each in either case, white
/// space around them allowed, at most [`MAX_CODE_OCTETS`].
///
/// # Errors
///
/// [`Refusal::Malformed`] for no digits, an odd number, a character that
/// is not a hexadecimal digit, or too many.
fn code_octets(hex: &str) -> Answer<([u8; MAX_CODE_OCTETS], usize)> {
    let digits = hex.trim().as_bytes();
    if digits.is_empty() || !digits.len().is_multiple_of(2) || digits.len() > 2 * MAX_CODE_OCTETS {
        return Err(Refusal::Malformed);
    }
    let mut octets = [0u8; MAX_CODE_OCTETS];
    hex_into(digits, &mut octets)?;
    Ok((octets, digits.len() / 2))
}

/// The five cells of an instant: its TAI seconds and attoseconds, and its
/// UTC label's POSIX second, leap-second flag and attoseconds.
fn instant_cells(line: &mut Line<'_>, tai: Instant<Tai>, utc: UtcInstant) -> Answer<()> {
    let (seconds, attoseconds) = tai_parts(tai)?;
    line.value(seconds)
        .value(attoseconds)
        .value(utc.unix_seconds)
        .flag(utc.leap_second)
        .value(utc.subsec_attos);
    Ok(())
}

/// A UTC instant with its TAI instant, by the leap-second table.
fn with_tai(utc: UtcInstant, strict: bool) -> Answer<(Instant<Tai>, UtcInstant)> {
    Ok((unix::tai_from_utc(utc, policy(strict))?, utc))
}

/// How many columns [`ccsds_decode_line`] writes.
pub const CCSDS_DECODE_COLUMNS: usize = 6;

/// The line of `hc_ccsds_decode`: a binary CCSDS code, a P-field and
/// exactly the T-field it announces, as hexadecimal — the code's name,
/// `cuc`, `cds` or `ccs`, then the instant as TAI seconds and attoseconds
/// and as the UTC label's POSIX second, `1` for an inserted leap second or
/// `0`, and attoseconds. A CUC code counts TAI and gives its UTC label by
/// the leap-second table; CDS and CCS count UTC and give their TAI instant
/// by it.
///
/// # Errors
///
/// [`Refusal::Malformed`] for text that is not hexadecimal octets, a P-field
/// that does not decode, a T-field of another length, or fields out of
/// their range, a leap second on a day without one included;
/// [`Refusal::NoData`] for a Level 2 code, whose epoch only its agency
/// knows, a Level 3 or 4 code, which only its agency can read, 23:59:60
/// past the leap-second table, and under `strict` an instant outside it.
pub fn ccsds_decode_line(hex: &str, strict: bool) -> Answer<String> {
    let (octets, len) = code_octets(hex)?;
    let code = ccsds::decode(&octets[..len]).map_err(code_refusal)?;
    let (name, (tai, utc)) = match code {
        CcsdsCode::Cuc(time) => {
            if time.format().epoch != EpochLevel::Recommended {
                return Err(Refusal::NoData);
            }
            let tai = time.to_tai()?;
            ("cuc", (tai, unix::utc_from_tai(tai, policy(strict))?))
        }
        CcsdsCode::Cds(time) => {
            if time.format().epoch != EpochLevel::Recommended {
                return Err(Refusal::NoData);
            }
            let utc = time.to_utc().map_err(|error| code_refusal(error.into()))?;
            ("cds", with_tai(utc, strict)?)
        }
        CcsdsCode::Ccs(time) => (
            "ccs",
            with_tai(time.to_utc_instant().map_err(code_refusal)?, strict)?,
        ),
        CcsdsCode::AgencyDefined(_) => return Err(Refusal::NoData),
    };
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(name);
    instant_cells(&mut line, tai, utc)?;
    line.end();
    Ok(out)
}

/// Octets as lower-case hexadecimal.
fn hex(cell: &mut dyn Write, octets: &[u8]) {
    for octet in octets {
        let _ = write!(cell, "{octet:02x}");
    }
}

/// The line of `hc_ccsds_encode`: the binary CCSDS code of a TAI instant in
/// the format a P-field names, as lower-case hexadecimal, the P-field
/// first. A CUC code counts the TAI instant; CDS and CCS count its UTC
/// label, from the leap-second table. The finer part of the second is
/// floored to the format's resolution.
///
/// # Errors
///
/// [`Refusal::Malformed`] for a P-field that is not exactly one P-field;
/// [`Refusal::NoData`] for a Level 2, 3 or 4 format, and under `strict`
/// an instant outside the leap-second table; [`Refusal::OutOfRange`] for
/// attoseconds from 10¹⁸ and an instant the format cannot count: before
/// 1958, past its last count, or outside the years 1 to 9999.
pub fn ccsds_encode_line(
    tai_seconds: i64,
    attoseconds: u64,
    p_field: &str,
    strict: bool,
) -> Answer<String> {
    let instant = tai_instant(tai_seconds, attoseconds)?;
    let (octets, len) = code_octets(p_field)?;
    let (preamble, used) = Preamble::decode(&octets[..len]).map_err(|_| Refusal::Malformed)?;
    if used != len {
        return Err(Refusal::Malformed);
    }
    let range = |error: ValueError| match code_refusal(error) {
        Refusal::Malformed => Refusal::OutOfRange,
        other => other,
    };
    let code = match preamble {
        Preamble::Cuc(format) => {
            if format.epoch != EpochLevel::Recommended {
                return Err(Refusal::NoData);
            }
            CucTime::from_tai(format, instant)?.encode(true)?
        }
        Preamble::Cds(format) => {
            if format.epoch != EpochLevel::Recommended {
                return Err(Refusal::NoData);
            }
            let utc = unix::utc_from_tai(instant, policy(strict))?;
            CdsTime::from_utc(format, utc)
                .map_err(|error| range(error.into()))?
                .encode(true)?
        }
        Preamble::Ccs(format) => {
            let utc = unix::utc_from_tai(instant, policy(strict))?;
            CcsTime::from_utc_instant(format, utc)
                .map_err(range)?
                .encode(true)
                .map_err(range)?
        }
        Preamble::AgencyDefined { .. } => return Err(Refusal::NoData),
    };
    Ok(line(|line| {
        line.cell_with(|cell| hex(cell, code.as_slice()));
    }))
}

/// How many columns [`ccsds_ascii_parse_line`] writes.
pub const CCSDS_ASCII_COLUMNS: usize = 9;

/// The line of `hc_ccsds_ascii_parse`: an ASCII code A or B read — the
/// variation, `a` or `b`; the instant it names, the start of the span a
/// truncated code names, as [`ccsds_decode_line`] writes it; how far its
/// time part runs, `hour`, `minute`, `second` or `fraction`; the digits of
/// the fraction, 1 to 18, else empty; and `1` if the terminator `Z`
/// follows, else `0`.
///
/// # Errors
///
/// [`Refusal::Malformed`] for text that is not a code, a calendar or time
/// subset alone included, or 23:59:60 on a day the table does not end in
/// one; [`Refusal::NoData`] for 23:59:60 past the leap-second table, and
/// under `strict` an instant outside it.
pub fn ccsds_ascii_parse_line(text: &str, strict: bool) -> Answer<String> {
    let code = ccsds::parse(text).map_err(|_| Refusal::Malformed)?;
    let utc = code.to_utc_instant().map_err(code_refusal)?;
    let (tai, utc) = with_tai(utc, strict)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(code.variation.id());
    instant_cells(&mut line, tai, utc)?;
    match code.precision {
        AsciiPrecision::Hour => line.cell("hour").empty(),
        AsciiPrecision::Minute => line.cell("minute").empty(),
        AsciiPrecision::Second => line.cell("second").empty(),
        AsciiPrecision::Fraction(digits) => line.cell("fraction").value(digits),
    };
    line.flag(code.terminator);
    line.end();
    Ok(out)
}

/// The line of `hc_ccsds_ascii_format`: the ASCII code of a TAI instant's
/// UTC label, from the leap-second table, in a variation, `a` or `b`, to a
/// precision, `hour`, `minute`, `second` or the digits of the fraction,
/// `1` to `18`, with the terminator `Z` when `terminator` is set. The
/// reading is truncated to the precision, never rounded.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a variation or precision not named;
/// [`Refusal::OutOfRange`] for attoseconds from 10¹⁸ and an instant
/// outside the years 1 to 9999; [`Refusal::NoData`] under `strict` for an
/// instant outside the leap-second table.
pub fn ccsds_ascii_format_line(
    tai_seconds: i64,
    attoseconds: u64,
    variation: &str,
    precision: &str,
    terminator: bool,
    strict: bool,
) -> Answer<String> {
    let instant = tai_instant(tai_seconds, attoseconds)?;
    let variation = AsciiVariation::by_id(variation).ok_or(Refusal::Unknown)?;
    let precision = AsciiPrecision::by_name(precision).ok_or(Refusal::Unknown)?;
    let utc = unix::utc_from_tai(instant, policy(strict))?;
    let code = AsciiTime::from_utc_instant(utc, variation, precision, terminator)
        .map_err(|_| Refusal::OutOfRange)?;
    let mut text = String::new();
    code.write(&mut text).map_err(|_| Refusal::OutOfRange)?;
    Ok(line(|line| {
        line.cell(&text);
    }))
}

/// The radio code an identifier names, by [`RadioCode::by_id`].
fn radio_code(name: &str) -> Answer<RadioCode> {
    RadioCode::by_id(name).ok_or(Refusal::Unknown)
}

/// The longest frame, a minute with an inserted leap second.
const MAX_FRAME: usize = 61;

/// A frame written one character a symbol: `0`, `1`, and for the codes
/// with a marker `M`, in either case; white space around it allowed.
///
/// # Errors
///
/// [`Refusal::Malformed`] for any other character, a marker in a code of
/// bits, or more than `N` symbols: 61 seconds for a radio code.
fn frame_symbols<const N: usize>(text: &str, markers: bool) -> Answer<([Symbol; N], usize)> {
    let text = text.trim().as_bytes();
    if text.len() > N {
        return Err(Refusal::Malformed);
    }
    let mut symbols = [Symbol::Zero; N];
    for (symbol, character) in symbols.iter_mut().zip(text) {
        *symbol = match character {
            b'0' => Symbol::Zero,
            b'1' => Symbol::One,
            b'M' | b'm' if markers => Symbol::Marker,
            _ => return Err(Refusal::Malformed),
        };
    }
    Ok((symbols, text.len()))
}

/// The bits of a frame of a code without markers.
fn frame_bits(text: &str) -> Answer<([bool; MAX_FRAME], usize)> {
    let (symbols, len) = frame_symbols::<MAX_FRAME>(text, false)?;
    Ok((symbols.map(|symbol| symbol == Symbol::One), len))
}

/// The refusal a frame error is: a frame that is not the code's.
const fn frame_refusal(_: FrameError) -> Refusal {
    Refusal::Malformed
}

/// The century a caller names: a multiple of 100 from 0 to 9 900.
fn century(century: i64) -> Answer<i64> {
    if (0..=9_900).contains(&century) && century % 100 == 0 {
        Ok(century)
    } else {
        Err(Refusal::OutOfRange)
    }
}

/// How many columns [`radio_decode_line`] writes.
pub const RADIO_DECODE_COLUMNS: usize = 11;

/// What a decoded frame says, as [`radio_decode_line`] writes it.
struct Decoded {
    reading: CivilDateTime,
    offset_hours: i64,
    seconds: usize,
    leap: LeapNotice,
    summer: &'static str,
    zone_change: Option<bool>,
    dut1_tenths: Option<i8>,
    dst_next: Option<u8>,
}

/// The line of `hc_radio_decode`: one minute's frame of a radio code of
/// [`RadioCode::ALL`], a string of `0`, `1` and, for `jjy` and `wwvb-am`,
/// `M` for a marker, read in the century beginning `century` — the POSIX
/// second of the minute the frame names (its first marker, or for `dcf77`
/// the minute it announces); the fixed day, hour and minute of that
/// minute in the code's own time; the code's time's offset from UTC in
/// hours, 9 for JST, 1 or 2 for CET or CEST, 0 for WWVB's UTC; the frame's
/// seconds, 59 to 61; the leap second it announces, `none`, `positive` or
/// `negative`, DCF77's A2 and WWVB's amplitude bit 56 being read as a
/// second inserted, the only kind their frames make room for; the zone or
/// summer-time state, `cet` or `cest` for `dcf77`, `standard`,
/// `begins-today`, `in-effect` or `ends-today` for WWVB, empty for `jjy`;
/// DCF77's A1, `1` when the zone changes at the end of the hour, else
/// empty for the other codes; UT1 − UTC in tenths of a second for
/// `wwvb-am`, else empty; and the phase code's six-bit `dst_next` word for
/// `wwvb-pm`, else empty.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a code not named; [`Refusal::OutOfRange`] for
/// a century that is not a multiple of 100 from 0 to 9900;
/// [`Refusal::Malformed`] for a frame that is not the code's — a wrong
/// length, symbol, BCD digit or parity, or a date that does not exist;
/// [`Refusal::NoData`] for JJY's call-sign frame of minutes 15 and 45,
/// which carries no year.
pub fn radio_decode_line(code: &str, frame: &str, century_start: i64) -> Answer<String> {
    let code = radio_code(code)?;
    let century = century(century_start)?;
    let decoded = match code {
        RadioCode::Jjy => {
            let (symbols, len) = frame_symbols::<MAX_FRAME>(frame, true)?;
            let frame = JjyFrame::decode(&symbols[..len]).map_err(frame_refusal)?;
            let JjyContent::Standard { leap, .. } = frame.content else {
                return Err(Refusal::NoData);
            };
            Decoded {
                reading: frame.reading(century).map_err(frame_refusal)?,
                offset_hours: 9,
                seconds: len,
                leap,
                summer: "",
                zone_change: None,
                dut1_tenths: None,
                dst_next: None,
            }
        }
        RadioCode::Dcf77 => {
            let (bits, len) = frame_bits(frame)?;
            let frame = Dcf77Frame::decode(&bits[..len]).map_err(frame_refusal)?;
            Decoded {
                reading: frame.reading(century).map_err(frame_refusal)?,
                offset_hours: frame.zone.offset_hours(),
                seconds: len,
                leap: if frame.leap_second {
                    LeapNotice::Positive
                } else {
                    LeapNotice::None
                },
                summer: frame.zone.id(),
                zone_change: Some(frame.zone_change),
                dut1_tenths: None,
                dst_next: None,
            }
        }
        RadioCode::WwvbAm => {
            let (symbols, len) = frame_symbols::<MAX_FRAME>(frame, true)?;
            let frame = AmFrame::decode(&symbols[..len]).map_err(frame_refusal)?;
            Decoded {
                reading: frame.reading(century).map_err(frame_refusal)?,
                offset_hours: 0,
                seconds: len,
                leap: if frame.leap_second_warning {
                    LeapNotice::Positive
                } else {
                    LeapNotice::None
                },
                summer: frame.dst.id(),
                zone_change: None,
                dut1_tenths: Some(frame.dut1_tenths),
                dst_next: None,
            }
        }
        RadioCode::WwvbPm => {
            let (bits, len) = frame_bits(frame)?;
            let frame = PmFrame::decode(&bits[..len]).map_err(frame_refusal)?;
            Decoded {
                reading: frame.reading(century).map_err(frame_refusal)?,
                offset_hours: 0,
                seconds: len,
                leap: frame.leap,
                summer: frame.dst.id(),
                zone_change: None,
                dut1_tenths: None,
                dst_next: frame.next.word(),
            }
        }
    };
    let reading = decoded.reading;
    let unix = reading
        .day
        .to_unix_days()
        .checked_mul(86_400)
        .and_then(|start| {
            start.checked_add(
                i64::from(reading.time.hour()) * 3_600 + i64::from(reading.time.minute()) * 60
                    - decoded.offset_hours * 3_600,
            )
        })
        .ok_or(Refusal::Overflow)?;
    Ok(line(|line| {
        line.value(unix)
            .value(reading.day.0)
            .value(reading.time.hour())
            .value(reading.time.minute())
            .value(decoded.offset_hours)
            .value(decoded.seconds)
            .cell(decoded.leap.id())
            .cell(decoded.summer)
            .value_or_empty(decoded.zone_change.map(u8::from))
            .value_or_empty(decoded.dut1_tenths)
            .value_or_empty(decoded.dst_next);
    }))
}

/// The first and last POSIX seconds of the years 1 to 9999, the minutes
/// `hc_radio_encode` writes a frame for.
const RADIO_UNIX_RANGE: core::ops::RangeInclusive<i64> = -62_135_596_800..=253_402_300_799;

/// The line of `hc_radio_encode`: the frame of a radio code of
/// [`RadioCode::ALL`] for the minute that begins at a POSIX second, as a
/// string of `0`, `1` and, for `jjy` and `wwvb-am`, `M` — the frame that
/// begins with that minute, or for `dcf77` the one sent during the minute
/// before, which announces it. `leap` is the leap second announced for
/// the end of the month, or for `dcf77` of the hour: 1 for a second
/// inserted, −1 for one omitted, which only `jjy` and `wwvb-pm` can say,
/// and 0 for none; the frame of the minute before the leap second is a
/// second longer or shorter. `summer` is DCF77's zone, `cet` or `cest`,
/// which also sets the reading's offset, or WWVB's summer-time state,
/// `standard`, `begins-today`, `in-effect` or `ends-today`, and empty for
/// `jjy`. `zone_change` is DCF77's A1 and is read by that code alone,
/// `dut1_tenths` WWVB's amplitude UT1 − UTC in tenths, −9 to 9, and
/// `dst_next` the phase code's six-bit word, which must be one Table 8
/// lists for the direction `summer` gives. JJY's minutes 15 and 45 are its
/// call-sign frames, with no stop planned. The third party's bits, the call
/// bit and the reserved bits are 0.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a code, or a `summer` state, it does not name;
/// [`Refusal::OutOfRange`] for a second that does not begin a minute of
/// the years 1 to 9999, a `leap` other than −1, 0 or 1 or a negative one
/// the code cannot say, a UT1 − UTC outside ±0.9 s, and a `dst_next` word
/// Table 8 does not list for the direction.
pub fn radio_encode_line(
    code: &str,
    unix_seconds: i64,
    leap: i32,
    summer: &str,
    zone_change: bool,
    dut1_tenths: i32,
    dst_next: u32,
) -> Answer<String> {
    encode_radio(
        code,
        unix_seconds,
        leap,
        Summer::Named {
            summer,
            zone_change,
        },
        dut1_tenths,
        dst_next,
    )
}

/// The line of `hc_radio_encode` with the summer-time state read from a
/// zone's rules instead of the caller, in a build with the `tz` feature:
/// for `dcf77` the zone and A1 from
/// [`dcf77::summer_time`](hc_format::radio::dcf77::summer_time), for both
/// WWVB codes the state of the minute's UTC day from
/// [`DstState::of_day`], and for `wwvb-pm` its `dst_next` word, the next
/// change after that day, from [`DstNext::of_day`]. The other arguments
/// are [`radio_encode_line`]'s.
///
/// # Errors
///
/// As [`radio_encode_line`]'s, and [`Refusal::Unknown`] for `jjy`, which
/// has no summer time, and [`Refusal::OutOfRange`] for a `dcf77` minute at
/// which the zone keeps neither CET nor CEST, or changes to another within
/// the hour.
#[cfg(feature = "tz")]
pub fn radio_encode_line_by_zone(
    code: &str,
    unix_seconds: i64,
    leap: i32,
    zone: &dyn TimeZone,
    dut1_tenths: i32,
) -> Answer<String> {
    encode_radio(
        code,
        unix_seconds,
        leap,
        Summer::Rules(zone),
        dut1_tenths,
        0,
    )
}

/// The line of `hc_radio_encode`: [`radio_encode_line`]'s for a `summer`
/// state named outright; or, for one that names a zone after
/// [`RADIO_SUMMER_ZONE`], [`radio_encode_line_by_zone`]'s with the rules
/// [`crate::zone_lines::with_zone`] reads for the name, which leaves
/// `zone_change` and `dst_next` unread.
///
/// # Errors
///
/// As those two, and [`Refusal::Unknown`] for a name that selects no zone,
/// or for any name in a build without the `tz` feature.
pub fn radio_encode(
    code: &str,
    unix_seconds: i64,
    leap: i32,
    summer: &str,
    zone_change: bool,
    dut1_tenths: i32,
    dst_next: u32,
) -> Answer<String> {
    match radio_summer_zone(summer) {
        Some(zone) => radio_encode_in_zone(code, unix_seconds, leap, zone, dut1_tenths),
        None => radio_encode_line(
            code,
            unix_seconds,
            leap,
            summer,
            zone_change,
            dut1_tenths,
            dst_next,
        ),
    }
}

/// [`radio_encode_line_by_zone`] for the zone a name selects.
#[cfg(all(feature = "tz", feature = "std"))]
fn radio_encode_in_zone(
    code: &str,
    unix_seconds: i64,
    leap: i32,
    zone: &str,
    dut1_tenths: i32,
) -> Answer<String> {
    crate::zone_lines::with_zone(zone, |zone, _| {
        radio_encode_line_by_zone(code, unix_seconds, leap, zone, dut1_tenths)
    })?
}

/// A build without `tz` has no zone to read the state from.
#[cfg(not(all(feature = "tz", feature = "std")))]
const fn radio_encode_in_zone(
    _code: &str,
    _unix_seconds: i64,
    _leap: i32,
    _zone: &str,
    _dut1_tenths: i32,
) -> Answer<String> {
    Err(Refusal::Unknown)
}

/// What `hc_radio_encode`'s `summer` begins with to have the state read
/// from a zone's rules: `zone:Europe/Berlin`.
pub const RADIO_SUMMER_ZONE: &str = "zone:";

/// The zone a `summer` argument names after [`RADIO_SUMMER_ZONE`], in any
/// case, trimmed; `None` for a state named outright.
#[must_use]
pub fn radio_summer_zone(summer: &str) -> Option<&str> {
    let summer = summer.trim();
    // `get` rather than indexing: no slice can panic here.
    let prefix = summer.get(..RADIO_SUMMER_ZONE.len())?;
    let zone = summer.get(RADIO_SUMMER_ZONE.len()..)?;
    prefix
        .eq_ignore_ascii_case(RADIO_SUMMER_ZONE)
        .then(|| zone.trim())
}

/// Where a radio frame's summer-time state comes from.
#[derive(Clone, Copy)]
enum Summer<'a> {
    /// The caller's: `summer` named, and DCF77's A1.
    Named { summer: &'a str, zone_change: bool },
    /// A zone's rules.
    #[cfg(feature = "tz")]
    Rules(&'a dyn TimeZone),
}

impl Summer<'_> {
    /// DCF77's zone and A1 for the minute beginning at `unix_seconds`.
    fn dcf77(self, unix_seconds: i64) -> Answer<(Zone, bool)> {
        // Only a zone's rules read the minute.
        #[cfg(not(feature = "tz"))]
        let _ = unix_seconds;
        match self {
            Self::Named {
                summer,
                zone_change,
            } => {
                let zone = Zone::by_id(summer).ok_or(Refusal::Unknown)?;
                Ok((zone, zone_change))
            }
            #[cfg(feature = "tz")]
            Self::Rules(zone) => {
                summer_time(zone, UnixTime::from_seconds(unix_seconds)).ok_or(Refusal::OutOfRange)
            }
        }
    }

    /// WWVB's state for the minute beginning at `unix_seconds`.
    fn wwvb(self, unix_seconds: i64) -> Answer<DstState> {
        // Only a zone's rules read the minute.
        #[cfg(not(feature = "tz"))]
        let _ = unix_seconds;
        match self {
            Self::Named { summer, .. } => DstState::by_id(summer).ok_or(Refusal::Unknown),
            #[cfg(feature = "tz")]
            Self::Rules(zone) => Ok(DstState::of_day(zone, UnixTime::from_seconds(unix_seconds))),
        }
    }

    /// The phase code's `dst_next` for the minute beginning at
    /// `unix_seconds`: the caller's `word` when the state is named, which
    /// must be one Table 8 lists for the direction `dst_on` gives, or the
    /// zone's next change.
    fn dst_next(self, unix_seconds: i64, word: u32, dst_on: bool) -> Answer<DstNext> {
        // Only a zone's rules read the minute.
        #[cfg(not(feature = "tz"))]
        let _ = unix_seconds;
        match self {
            Self::Named { .. } => {
                let word = u8::try_from(word)
                    .ok()
                    .filter(|word| *word < 64)
                    .ok_or(Refusal::OutOfRange)?;
                DstNext::from_word(word, dst_on).ok_or(Refusal::OutOfRange)
            }
            #[cfg(feature = "tz")]
            Self::Rules(zone) => Ok(DstNext::of_day(zone, UnixTime::from_seconds(unix_seconds))),
        }
    }
}

/// [`radio_encode_line`] and [`radio_encode_line_by_zone`], from either
/// source of the summer-time state.
fn encode_radio(
    code: &str,
    unix_seconds: i64,
    leap: i32,
    summer: Summer<'_>,
    dut1_tenths: i32,
    dst_next: u32,
) -> Answer<String> {
    let code = radio_code(code)?;
    if !RADIO_UNIX_RANGE.contains(&unix_seconds) || unix_seconds.rem_euclid(60) != 0 {
        return Err(Refusal::OutOfRange);
    }
    let leap = match leap {
        0 => LeapNotice::None,
        1 => LeapNotice::Positive,
        -1 => LeapNotice::Negative,
        _ => return Err(Refusal::OutOfRange),
    };
    let signless = |leap: LeapNotice| match leap {
        LeapNotice::None => Ok(false),
        LeapNotice::Positive => Ok(true),
        LeapNotice::Negative => Err(Refusal::OutOfRange),
    };
    // A whole minute, and an offset of whole hours, give a whole minute of
    // the code's time.
    let minute_at = |offset_hours: i64| -> Answer<CivilDateTime> {
        let local = unix_seconds + offset_hours * 3_600;
        let minutes = local.rem_euclid(86_400) / 60;
        let time = CivilTime::hms((minutes / 60) as u8, (minutes % 60) as u8, 0)
            .map_err(|_| Refusal::OutOfRange)?;
        Ok(CivilDateTime::new(
            Rd::from_unix_days(local.div_euclid(86_400)),
            time,
        ))
    };
    let range = |_: FrameError| Refusal::OutOfRange;
    let mut frame_text = String::new();
    let mut push_symbols = |symbols: &[Symbol]| {
        for symbol in symbols {
            frame_text.push(match symbol {
                Symbol::Zero => '0',
                Symbol::One => '1',
                Symbol::Marker => 'M',
            });
        }
    };
    let bits = |bits: &[bool]| {
        bits.iter()
            .map(|&bit| Symbol::bit(bit))
            .collect::<alloc::vec::Vec<_>>()
    };
    match code {
        RadioCode::Jjy => {
            if !matches!(summer, Summer::Named { summer, .. } if summer.trim().is_empty()) {
                return Err(Refusal::Unknown);
            }
            let frame = JjyFrame::for_minute(minute_at(9)?, leap).map_err(range)?;
            push_symbols(frame.encode().map_err(range)?.as_slice());
        }
        RadioCode::Dcf77 => {
            let (zone, zone_change) = summer.dcf77(unix_seconds)?;
            let frame = Dcf77Frame::for_minute(
                minute_at(zone.offset_hours())?,
                zone,
                zone_change,
                signless(leap)?,
            )
            .map_err(range)?;
            push_symbols(&bits(frame.encode().map_err(range)?.as_slice()));
        }
        RadioCode::WwvbAm => {
            let dst = summer.wwvb(unix_seconds)?;
            let tenths = i8::try_from(dut1_tenths).map_err(|_| Refusal::OutOfRange)?;
            let frame =
                AmFrame::for_minute(minute_at(0)?, tenths, dst, signless(leap)?).map_err(range)?;
            push_symbols(frame.encode().map_err(range)?.as_slice());
        }
        RadioCode::WwvbPm => {
            let dst = summer.wwvb(unix_seconds)?;
            let next = summer.dst_next(unix_seconds, dst_next, dst.bits().0)?;
            let reading = minute_at(0)?;
            let year = gregorian::year_from_fixed(reading.day);
            let frame = PmFrame::for_minute(reading, year - year.rem_euclid(100), dst, leap, next)
                .map_err(range)?;
            push_symbols(&bits(frame.encode().map_err(range)?.as_slice()));
        }
    }
    Ok(line(|line| {
        line.cell(&frame_text);
    }))
}

/// The IRIG code a signal designation names, a format letter and three
/// digits as `B124`, with each digit one Table 4-1 of IRIG 200-16 permits
/// the format ([`IrigCode::from_signal`]).
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other text.
fn irig_code(signal: &str) -> Answer<IrigCode> {
    IrigCode::from_signal(signal).map_err(|_| Refusal::Unknown)
}

/// How many columns each line of [`irig_formats_lines`] writes.
pub const IRIG_FORMATS_COLUMNS: usize = 9;

/// A list of digits separated by spaces.
fn digit_list(cell: &mut dyn Write, digits: &[u8]) {
    for (index, digit) in digits.iter().enumerate() {
        let separator = if index > 0 { " " } else { "" };
        let _ = write!(cell, "{separator}{digit}");
    }
}

/// The lines of `hc_irig_formats`: every IRIG format of
/// [`IrigFormat::ALL`](hc_format::irig::IrigFormat::ALL), A first, one a
/// line, with what a caller needs to name a code and to find a frame.
///
/// The cells: the format's letter; the index count interval in
/// microseconds, 1 000 for A's 1 000 pulses a second (Table 3-1); the
/// index counts in a frame, 100 or 60 (Table 3-2); the frame's length in
/// microseconds, whose multiples from midnight are the readings
/// [`irig_encode_line`] writes a frame at; the fields of the BCD time of
/// year, most significant first and separated by spaces, the last of which
/// the frame's length is (`days hours minutes seconds tenths` for A); the
/// control bits the format has room for (Table 3-4); and the modulations,
/// the carriers and the coded expressions Table 4-1 permits it, the three
/// digits of a signal designation, each list separated by spaces.
#[must_use]
pub fn irig_formats_lines() -> String {
    let mut out = String::new();
    for format in irig::IrigFormat::ALL {
        let mut line = Line::new(&mut out);
        line.value(format.letter())
            .value(format.index_count_micros())
            .value(format.frame_len())
            .value(format.frame_micros())
            .cell(&format.time_fields().join(" "))
            .value(format.control_bits())
            .cell_with(|cell| digit_list(cell, format.modulations()))
            .cell_with(|cell| digit_list(cell, format.carriers()))
            .cell_with(|cell| digit_list(cell, format.expressions()));
        line.end();
    }
    out
}

/// The fixed days of the years 1 to 9999, the days `hc_irig_encode` writes
/// a frame for and `hc_irig_decode` reads one in.
const IRIG_DAYS: core::ops::RangeInclusive<i64> = 1..=3_652_059;

/// How many columns [`irig_decode_line`] writes.
pub const IRIG_DECODE_COLUMNS: usize = 9;

/// The line of `hc_irig_decode`: one frame of an IRIG serial time code,
/// A, B, D, E, G or H, named by its signal designation (`B124`, whose last
/// digit, the coded expression, says which fields the frame carries), as
/// a string of `0`, `1` and `M` for the index markers, position
/// identifiers and reference bit, Pr first, as the radio codes are
/// written — the reading at Pr and the fields that give it.
///
/// A code that carries the year's last two digits reads them in the
/// century `year` is in (2003 for `03` with `year` 2026); a code that does
/// not is read in `year` itself. The cells: the fixed day; the day of the
/// year, 1 January being 1; the hour, minute and second, 60 for a leap
/// second, and the hundredths of a second, each 0 where the format does
/// not send it; the year's two digits, empty for a code without them; the
/// control bits as a number, control bit 1 as its lowest bit, empty for a
/// code without them; and the straight binary seconds of the day, empty
/// for a code without them. The code carries no time scale, so the reading
/// is a date and a time of whatever clock the generator was set to.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a signal designation Table 4-1 does not
/// permit; [`Refusal::OutOfRange`] for a `year` outside 1 to 9999;
/// [`Refusal::Malformed`] for a frame that is not the code's — a wrong
/// length or symbol, a 1 where the code sends an index marker, a BCD digit
/// or a time of day out of range, straight binary seconds that are not
/// the BCD time's, a day the year does not have, a year's digits that are
/// not `year`'s, or a leap second anywhere but at the end of a month.
pub fn irig_decode_line(signal: &str, frame: &str, year: i64) -> Answer<String> {
    let code = irig_code(signal)?;
    if !(1..=9_999).contains(&year) {
        return Err(Refusal::OutOfRange);
    }
    let (symbols, len) = frame_symbols::<{ irig::MAX_FRAME }>(frame, true)?;
    let decoded = IrigFrame::decode(code, &symbols[..len]).map_err(frame_refusal)?;
    let reading = if code.has_year() {
        decoded.reading(year - year.rem_euclid(100))
    } else {
        decoded.reading_in_year(year)
    }
    .map_err(frame_refusal)?;
    Ok(line(|line| {
        line.value(reading.day.0)
            .value(decoded.day_of_year)
            .value(decoded.hour)
            .value(decoded.minute)
            .value(decoded.second)
            .value(decoded.hundredths)
            .value_or_empty(decoded.year)
            .value_or_empty(code.has_control().then_some(decoded.control))
            .value_or_empty(code.has_sbs().then(|| decoded.straight_binary_seconds()));
    }))
}

/// The line of `hc_irig_encode`: the frame of an IRIG code, named by its
/// signal designation as for [`irig_decode_line`], whose reference bit Pr
/// falls at a reading of the civil clock — a fixed day, whole seconds after
/// midnight, 86 400 being 23:59:60, and hundredths of a second — as a
/// string of `0`, `1` and `M`, Pr first. `control` is the control bits,
/// control bit 1 as its lowest bit, 0 for a code without them.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a signal designation Table 4-1 does not
/// permit; [`Refusal::OutOfRange`] for a day outside the years 1 to 9999,
/// seconds past 86 400, hundredths past 99, a reading the format has no
/// frame at — for B one off the second, for D one off the hour — and
/// control bits the code has no room for.
pub fn irig_encode_line(
    signal: &str,
    fixed: i64,
    seconds_of_day: u32,
    hundredths: u32,
    control: u32,
) -> Answer<String> {
    let code = irig_code(signal)?;
    if !IRIG_DAYS.contains(&fixed) || seconds_of_day > 86_400 || hundredths > 99 {
        return Err(Refusal::OutOfRange);
    }
    let (hour, minute, second) = if seconds_of_day == 86_400 {
        (23, 59, 60)
    } else {
        // Each below 24, 60 and 60 by the check above.
        (
            (seconds_of_day / 3_600) as u8,
            (seconds_of_day / 60 % 60) as u8,
            (seconds_of_day % 60) as u8,
        )
    };
    let time = CivilTime::new(
        hour,
        minute,
        second,
        u64::from(hundredths) * 10_000_000_000_000_000,
    )
    .map_err(|_| Refusal::OutOfRange)?;
    let range = |_: FrameError| Refusal::OutOfRange;
    let frame = IrigFrame::for_reading(code, CivilDateTime::new(Rd(fixed), time), control)
        .map_err(range)?;
    let symbols = frame.encode().map_err(range)?;
    Ok(line(|line| {
        line.cell_with(|cell| {
            for symbol in symbols.as_slice() {
                let _ = cell.write_char(match symbol {
                    Symbol::Zero => '0',
                    Symbol::One => '1',
                    Symbol::Marker => 'M',
                });
            }
        });
    }))
}

/// .NET's `DateTime.Ticks` of a POSIX instant as a `Utc` value: the
/// 100-nanosecond intervals since 0001-01-01 00:00, floored to the tick.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for attoseconds from 10¹⁸ and an instant before
/// 0001-01-01 or after 9999-12-31 23:59:59.9999999.
pub fn dotnet_ticks_from_unix(unix_seconds: i64, attoseconds: u64) -> Answer<i64> {
    Ok(DotnetDateTime::from_unix(unix_instant(unix_seconds, attoseconds)?)?.ticks())
}

/// The line of `hc_unix_from_dotnet_ticks`: the reading a count of .NET
/// ticks names in the POSIX shape, the whole seconds from
/// 1970-01-01 00:00 and the attoseconds: POSIX time for a `Utc` value, and
/// for a `Local` or `Unspecified` one the wall clock of a zone the value
/// does not name. The Kind is the caller's to know; the ticks are the same.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for ticks outside 0 to 3 155 378 975 999 999 999,
/// the range of `DateTime`.
pub fn unix_from_dotnet_ticks_line(ticks: i64) -> Answer<String> {
    let reading = DotnetDateTime::new(ticks, DateTimeKind::Utc)?.wall_clock();
    Ok(line(|line| {
        line.value(reading.seconds()).value(reading.subsec_attos());
    }))
}

/// The six-hour reckoning an identifier names, `ethiopian-hours` or
/// `swahili-hours`, by [`east_african_hours::by_id`].
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other.
pub fn six_hour_reckoning(id: &str) -> Answer<Reckoning> {
    east_african_hours::by_id(id).ok_or(Refusal::Unknown)
}

/// How many columns [`six_hour_clock_line`] writes.
pub const SIX_HOUR_CLOCK_COLUMNS: usize = 6;

/// The line of `hc_six_hour_clock`: a time of the civil day, as whole
/// seconds after midnight of the caller's wall clock, read on a six-hour
/// reckoning — the hour on the dial, 1 to 12, the minute and the second,
/// which are the civil clock's, the half, `day` or `night`, and the part
/// of the day the reckoning's source names for the hour, in its language
/// and in English, both empty for `ethiopian-hours`, which carries none.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a reckoning not named, and
/// [`Refusal::OutOfRange`] for a time of day from 86 400 s.
pub fn six_hour_clock_line(reckoning: &str, seconds_of_day: u32) -> Answer<String> {
    let reckoning = six_hour_reckoning(reckoning)?;
    if seconds_of_day >= 86_400 {
        return Err(Refusal::OutOfRange);
    }
    // Each below 24, 60 and 60 by the check above.
    let civil = CivilTime::hms(
        (seconds_of_day / 3_600) as u8,
        (seconds_of_day / 60 % 60) as u8,
        (seconds_of_day % 60) as u8,
    )
    .map_err(|_| Refusal::OutOfRange)?;
    let reading = reckoning.reading(civil);
    let period = reckoning.period(civil);
    Ok(line(|line| {
        line.value(reading.hour)
            .value(reading.minute)
            .value(reading.second)
            .cell(match reading.half {
                Half::Day => "day",
                Half::Night => "night",
            })
            .cell_or_empty(period.map(|period| period.name))
            .cell_or_empty(period.map(|period| period.english));
    }))
}

/// The civil time of day of a six-hour reading, as whole seconds after
/// midnight: the hour on the dial, 1 to 12, the minute and second, and
/// whether it is counted in the night half.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a reckoning not named, and
/// [`Refusal::OutOfRange`] for an hour outside 1 to 12, a minute or a
/// second above 59.
pub fn civil_from_six_hour_clock(
    reckoning: &str,
    hour: u32,
    minute: u32,
    second: u32,
    night: bool,
) -> Answer<u32> {
    let reckoning = six_hour_reckoning(reckoning)?;
    let field = |value: u32| u8::try_from(value).map_err(|_| Refusal::OutOfRange);
    if minute > 59 || second > 59 {
        return Err(Refusal::OutOfRange);
    }
    let reading = HourReading {
        hour: field(hour)?,
        minute: field(minute)?,
        second: field(second)?,
        subsec_attos: 0,
        half: if night { Half::Night } else { Half::Day },
    };
    let civil = reckoning.civil(reading).map_err(|_| Refusal::OutOfRange)?;
    Ok(
        u32::from(civil.hour()) * 3_600
            + u32::from(civil.minute()) * 60
            + u32::from(civil.second()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    use crate::boundary::cells;

    /// 1988-01-18T17:20:43.123456 UTC, the standard's example of the ASCII
    /// codes (CCSDS 301.0-B-4 §§3.5.1.1–3.5.1.2, `ccsds-301-0-b-4`): POSIX
    /// second 569 524 843, and TAI − UTC was 24 s.
    const EXAMPLE_UNIX: &str = "569524843";
    const EXAMPLE_TAI: i64 = 569_524_867;
    const EXAMPLE_ATTOS: u64 = 123_456_000_000_000_000;

    /// The standard's example worked through CCS and CDS in
    /// `docs/systems/ccsds-time-codes.md`: `53 19 88 01 18 17 20 43 12 34 56`
    /// and `5B 19 88 00 18 …` in CCS, and `41 2A DE 03 B8 CE 73 01 C8` in CDS.
    #[test]
    fn the_standards_example_decodes_in_ccs_and_cds() {
        for (hex, name) in [
            ("53198801181720431234 56", "ccs"),
            ("5B 19880018172043123456", "ccs"),
            ("412ADE03B8CE7301C8", "cds"),
        ] {
            let hex: String = hex.chars().filter(|c| !c.is_whitespace()).collect();
            let line = ccsds_decode_line(&hex, true).expect("the standard's example");
            assert_eq!(
                cells(&line),
                [
                    name,
                    &EXAMPLE_TAI.to_string(),
                    &EXAMPLE_ATTOS.to_string(),
                    EXAMPLE_UNIX,
                    "0",
                    &EXAMPLE_ATTOS.to_string()
                ],
                "{hex}"
            );
            assert_eq!(cells(&line).len(), CCSDS_DECODE_COLUMNS);
            let p_field = &hex[..2];
            let back = ccsds_encode_line(EXAMPLE_TAI, EXAMPLE_ATTOS, p_field, true)
                .expect("the same instant");
            assert_eq!(back.trim_end(), hex.to_ascii_lowercase());
        }
    }

    /// CUC counts TAI: at 2000-01-01T00:00:00 UTC, TAI − UTC 32 s, the
    /// code is `1C 4E FF A2 20`, and half a second later with one octet of
    /// fraction `1D 4E FF A2 20 80` (`docs/systems/ccsds-time-codes.md`).
    /// 23:59:60.5 on 2016-12-31 is CDS day 21 549, millisecond 86 400 500.
    #[test]
    fn cuc_counts_tai_and_cds_carries_the_leap_second() {
        let tai = 946_684_832;
        assert_eq!(
            ccsds_encode_line(tai, 0, "1c", true).as_deref(),
            Ok("1c4effa220\n")
        );
        assert_eq!(
            ccsds_encode_line(tai, 500_000_000_000_000_000, "1D", true).as_deref(),
            Ok("1d4effa22080\n")
        );
        let line = ccsds_decode_line("1c4effa220", true).expect("a CUC code");
        assert_eq!(
            cells(&line),
            ["cuc", "946684832", "0", "946684800", "0", "0"]
        );
        let leap = ccsds_decode_line("40542d05265df4", true).expect("a leap second");
        assert_eq!(
            cells(&leap),
            [
                "cds",
                "1483228836",
                "500000000000000000",
                "1483228800",
                "1",
                "500000000000000000"
            ]
        );
        assert_eq!(
            ccsds_encode_line(1_483_228_836, 500_000_000_000_000_000, "40", true).as_deref(),
            Ok("40542d05265df4\n")
        );
        // The same millisecond on the day before, which ends in no leap
        // second, does not exist.
        assert_eq!(
            ccsds_decode_line("40542c05265df4", true),
            Err(Refusal::Malformed)
        );
        // Level 2 counts from an epoch only the agency knows.
        assert_eq!(ccsds_decode_line("2c4effa220", true), Err(Refusal::NoData));
        assert_eq!(ccsds_encode_line(tai, 0, "2c", true), Err(Refusal::NoData));
        assert_eq!(ccsds_decode_line("1c4effa2", true), Err(Refusal::Malformed));
        assert_eq!(
            ccsds_decode_line("1c4effa22", true),
            Err(Refusal::Malformed)
        );
        assert_eq!(
            ccsds_encode_line(tai, 0, "1c4e", true),
            Err(Refusal::Malformed)
        );
        assert_eq!(
            ccsds_encode_line(-1_000_000_000, 0, "1c", false),
            Err(Refusal::OutOfRange)
        );
    }

    #[test]
    fn the_ascii_codes_are_the_standards_example() {
        let line = ccsds_ascii_parse_line("1988-01-18T17:20:43.123456Z", true).expect("code A");
        assert_eq!(
            cells(&line),
            [
                "a",
                &EXAMPLE_TAI.to_string(),
                &EXAMPLE_ATTOS.to_string(),
                EXAMPLE_UNIX,
                "0",
                &EXAMPLE_ATTOS.to_string(),
                "fraction",
                "6",
                "1"
            ]
        );
        assert_eq!(cells(&line).len(), CCSDS_ASCII_COLUMNS);
        let truncated = ccsds_ascii_parse_line("1988-018T17:20", true).expect("code B");
        assert_eq!(cells(&truncated)[0], "b");
        assert_eq!(cells(&truncated)[6..], ["minute", "", "0"]);
        assert_eq!(
            ccsds_ascii_format_line(EXAMPLE_TAI, EXAMPLE_ATTOS, "B", "6", true, true).as_deref(),
            Ok("1988-018T17:20:43.123456Z\n")
        );
        assert_eq!(
            ccsds_ascii_format_line(EXAMPLE_TAI, EXAMPLE_ATTOS, "a", "minute", false, true)
                .as_deref(),
            Ok("1988-01-18T17:20\n")
        );
        assert_eq!(
            ccsds_ascii_format_line(1_483_228_836, 0, "a", "second", true, true).as_deref(),
            Ok("2016-12-31T23:59:60Z\n")
        );
        assert_eq!(
            ccsds_ascii_format_line(0, 0, "c", "hour", true, true),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            ccsds_ascii_format_line(0, 0, "a", "19", true, true),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            ccsds_ascii_parse_line("1988-01-18", true),
            Err(Refusal::Malformed)
        );
    }

    /// NICT's figure (`nict-jjy-timecode`): 17:25 JST on 1 April 2004, day
    /// 92, a Thursday, 08:25 UTC.
    const NICT: &str = "M01000101M000100111M000001001M001000010M000000100M100000000M";

    /// Table 10 of NIST's "Enhanced WWVB Broadcast Format" (2013): 17:30
    /// UTC on 4 July 2012, in the amplitude code with UT1 − UTC +0.4 s and
    /// summer time in effect, and in the phase code, whose `dst_next` is
    /// out of summer time on the first Sunday of November at 2 AM, 011011.
    const WWVB_AM: &str = "M01100000M000100111M000101000M011000101M010000001M001001011M";
    const WWVB_PM: &str = "001110110100010010000011001000011000110100110100010110110110";

    /// The frame `docs/systems/radio-time-codes.md` works from PTB's layout
    /// (`ptb-dcf77-timecode`): sent from 14:29:00 CEST on Sunday
    /// 27 September 2026, it names 14:30 CEST.
    const DCF77: &str = "00000000000000000100100001100001010011100111110010011001000";

    #[test]
    fn the_stations_examples_decode_to_their_minutes() {
        let jjy = radio_decode_line("jjy", NICT, 2000).expect("NICT's frame");
        assert_eq!(
            cells(&jjy),
            [
                "1080807900",
                "731672",
                "17",
                "25",
                "9",
                "60",
                "none",
                "",
                "",
                "",
                ""
            ]
        );
        let dcf77 = radio_decode_line("DCF77", DCF77, 2000).expect("the worked frame");
        assert_eq!(
            cells(&dcf77),
            [
                "1790512200",
                "739886",
                "14",
                "30",
                "2",
                "59",
                "none",
                "cest",
                "0",
                "",
                ""
            ]
        );
        let am = radio_decode_line("wwvb-am", WWVB_AM, 2000).expect("Table 10");
        assert_eq!(
            cells(&am),
            [
                "1341423000",
                "734688",
                "17",
                "30",
                "0",
                "60",
                "none",
                "in-effect",
                "",
                "4",
                ""
            ]
        );
        let pm = radio_decode_line("wwvb-pm", WWVB_PM, 2000).expect("Table 10");
        assert_eq!(
            cells(&pm),
            [
                "1341423000",
                "734688",
                "17",
                "30",
                "0",
                "60",
                "none",
                "in-effect",
                "",
                "",
                "27"
            ]
        );
        assert_eq!(cells(&pm).len(), RADIO_DECODE_COLUMNS);
    }

    #[test]
    fn the_stations_examples_encode_from_their_minutes() {
        let encode = |code, unix, summer, dut1, next| {
            radio_encode_line(code, unix, 0, summer, false, dut1, next).expect("a minute")
        };
        assert_eq!(encode("jjy", 1_080_807_900, "", 0, 0).trim_end(), NICT);
        assert_eq!(
            encode("dcf77", 1_790_512_200, "cest", 0, 0).trim_end(),
            DCF77
        );
        assert_eq!(
            encode("wwvb-am", 1_341_423_000, "in-effect", 4, 0).trim_end(),
            WWVB_AM
        );
        // Table 10 sets the notice bit and one reserved bit, which the
        // encoder leaves 0: bits 29, 39 and 49.
        let pm = encode("wwvb-pm", 1_341_423_000, "in-effect", 0, 27);
        let differing: Vec<usize> = pm
            .trim_end()
            .bytes()
            .zip(WWVB_PM.bytes())
            .enumerate()
            .filter(|(_, (ours, theirs))| ours != theirs)
            .map(|(second, _)| second)
            .collect();
        assert_eq!(differing, [39, 49]);
        let back = radio_decode_line("wwvb-pm", pm.trim_end(), 2000).expect("our frame");
        assert_eq!(cells(&back)[10], "27");
        assert_eq!(
            radio_encode_line("jjy", 1_080_807_930, 0, "", false, 0, 0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            radio_encode_line("dcf77", 1_790_512_200, -1, "cest", false, 0, 0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            radio_encode_line("wwvb-am", 1_341_423_000, 0, "summer", false, 0, 0),
            Err(Refusal::Unknown)
        );
        assert_eq!(radio_decode_line("msf", NICT, 2000), Err(Refusal::Unknown));
        assert_eq!(
            radio_decode_line("jjy", NICT, 2001),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            radio_decode_line("jjy", &NICT[1..], 2000),
            Err(Refusal::Malformed)
        );
        assert_eq!(
            radio_decode_line("dcf77", NICT, 2000),
            Err(Refusal::Malformed)
        );
        // NICT's call-sign frame of 17:15 carries no year.
        let call_sign =
            radio_encode_line("jjy", 1_080_807_300, 0, "", false, 0, 0).expect("a call-sign frame");
        assert_eq!(
            radio_decode_line("jjy", call_sign.trim_end(), 2000),
            Err(Refusal::NoData)
        );
    }

    /// The stations' examples again with the summer-time state read from
    /// the zones' rules — Europe/Berlin's for DCF77, America/New_York's,
    /// on the United States' rule as Denver, for WWVB — the frames
    /// announcing 2026's European changes, which carry A1, and the phase
    /// code's `dst_next` around 2026's American changes.
    #[cfg(feature = "tz")]
    #[test]
    fn a_zones_rules_give_the_frames_the_caller_would() {
        let berlin = hc_format::hc_tz::builtin::zone("Europe/Berlin").expect("built in");
        let new_york = hc_format::hc_tz::builtin::zone("America/New_York").expect("built in");
        let named = |code, unix, summer, change, dut1, next| {
            radio_encode_line(code, unix, 0, summer, change, dut1, next).expect("a minute")
        };
        let ruled = |code, unix, zone: &dyn TimeZone, dut1| {
            radio_encode_line_by_zone(code, unix, 0, zone, dut1).expect("a minute")
        };
        assert_eq!(ruled("dcf77", 1_790_512_200, &berlin, 0).trim_end(), DCF77);
        assert_eq!(
            ruled("wwvb-am", 1_341_423_000, &new_york, 4).trim_end(),
            WWVB_AM
        );
        // Table 10's frame but for its notice and reserved bits, which
        // `encode` leaves 0: its `dst_next`, 011011, is the first Sunday of
        // November at 2 AM, which New York's rules give.
        assert_eq!(
            ruled("wwvb-pm", 1_341_423_000, &new_york, 0),
            named("wwvb-pm", 1_341_423_000, "in-effect", false, 0, 27)
        );
        // 2026's changes in New York: the word names 8 March until 00:00
        // UTC that day, then 1 November, then from 00:00 UTC on 1 November
        // the second Sunday of March 2027. The three are all 011011 and
        // read apart by `dst_on[1]`, so the frames are the named ones.
        for (minute, summer) in [
            (1_772_927_940, "standard"),
            (1_772_928_000, "begins-today"),
            (1_793_491_140, "in-effect"),
            (1_793_491_200, "ends-today"),
        ] {
            assert_eq!(
                ruled("wwvb-pm", minute, &new_york, 0),
                named("wwvb-pm", minute, summer, false, 0, 27),
                "{minute}"
            );
        }
        // Berlin's rules give schedules of their own: the last Sunday of
        // March at 2 AM before summer time, 000010, and of October at 3 AM
        // in it, 010000.
        assert_eq!(
            ruled("wwvb-pm", 1_768_435_200, &berlin, 0),
            named("wwvb-pm", 1_768_435_200, "standard", false, 0, 0b000010)
        );
        assert_eq!(
            ruled("wwvb-pm", 1_782_864_000, &berlin, 0),
            named("wwvb-pm", 1_782_864_000, "in-effect", false, 0, 0b010000)
        );
        for (change, before, after) in [
            (1_774_746_000, "cet", "cest"),
            (1_792_890_000, "cest", "cet"),
        ] {
            for (minute, summer, a1) in [
                (change - 3_600, before, false),
                (change - 3_540, before, true),
                (change, after, true),
                (change + 60, after, false),
            ] {
                assert_eq!(
                    ruled("dcf77", minute, &berlin, 0),
                    named("dcf77", minute, summer, a1, 0, 0),
                    "{minute}"
                );
            }
        }
        // 2026-03-08, the United States' change: bit 57 from 00:00 UTC.
        assert_eq!(
            ruled("wwvb-am", 1_772_928_000, &new_york, 0),
            named("wwvb-am", 1_772_928_000, "begins-today", false, 0, 0)
        );
        assert_eq!(
            radio_summer_zone(" Zone: Europe/Berlin"),
            Some("Europe/Berlin")
        );
        assert_eq!(radio_summer_zone("cest"), None);
        assert_eq!(radio_summer_zone("zon"), None);
        let london = hc_format::hc_tz::builtin::zone("Europe/London").expect("built in");
        assert_eq!(
            radio_encode_line_by_zone("dcf77", 1_790_512_200, 0, &london, 0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            radio_encode_line_by_zone("jjy", 1_080_807_900, 0, &berlin, 0),
            Err(Refusal::Unknown)
        );
    }

    /// `DateTime.MaxValue`, 23:59:59.9999999 on 9999-12-31, is 3 155 378 975
    /// 999 999 999 ticks (`ms-datetime-maxvalue`), and 1970-01-01 is
    /// 621 355 968 000 000 000.
    /// IRIG 200-16's Tables 3-1, 3-2, 3-4 and 4-1, one line a format.
    #[test]
    fn the_irig_formats_are_the_standards() {
        let text = irig_formats_lines();
        let rows: alloc::vec::Vec<alloc::vec::Vec<&str>> = text
            .lines()
            .map(|line| line.split('\t').collect())
            .collect();
        assert!(rows.iter().all(|row| row.len() == IRIG_FORMATS_COLUMNS));
        assert_eq!(
            rows.iter()
                .map(|row| row[0])
                .collect::<alloc::vec::Vec<_>>(),
            ["A", "B", "D", "E", "G", "H"]
        );
        assert_eq!(
            rows[1],
            [
                "B",
                "10000",
                "100",
                "1000000",
                "days hours minutes seconds",
                "18",
                "0 1 2",
                "0 2 3 4 5",
                "0 1 2 3 4 5 6 7"
            ]
        );
        assert_eq!(
            rows[2],
            [
                "D",
                "60000000",
                "60",
                "3600000000",
                "days hours",
                "9",
                "0 1",
                "0 1 2",
                "1 2"
            ]
        );
        assert_eq!(
            rows[4][3..6],
            [
                "10000",
                "days hours minutes seconds tenths hundredths",
                "27"
            ]
        );
        // Figure 5-2's B124 frame, 21:18:42, is at a whole second; half a
        // second on is no frame's reading.
        let day = gregorian::to_fixed(2003, 6, 22).expect("a date").0;
        assert!(irig_encode_line("B124", day, 76_722, 0, 0).is_ok());
        assert_eq!(
            irig_encode_line("B124", day, 76_722, 50, 0),
            Err(Refusal::OutOfRange)
        );
    }

    #[test]
    fn dotnet_ticks_are_microsofts() {
        assert_eq!(dotnet_ticks_from_unix(0, 0), Ok(621_355_968_000_000_000));
        let last = 253_402_300_799;
        assert_eq!(
            dotnet_ticks_from_unix(last, 999_999_900_000_000_000),
            Ok(3_155_378_975_999_999_999)
        );
        assert_eq!(
            dotnet_ticks_from_unix(last + 1, 0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            unix_from_dotnet_ticks_line(3_155_378_975_999_999_999).as_deref(),
            Ok("253402300799\t999999900000000000\n")
        );
        assert_eq!(
            unix_from_dotnet_ticks_line(0).as_deref(),
            Ok("-62135596800\t0\n")
        );
        assert_eq!(unix_from_dotnet_ticks_line(-1), Err(Refusal::OutOfRange));
    }

    /// The UNDP page's examples, "8 am is 2 o'clock" and "7 pm is 1
    /// o'clock" (`undp-eue-ethiopian-time`), and the Kansas lesson's "7:00
    /// am is referred to as saa moja asubuhi" and "7:00 pm is called saa
    /// moja usiku" (`ku-kiswahili-lesson-17`).
    #[test]
    fn the_six_hour_clocks_read_their_sources_examples() {
        let line = |reckoning, seconds| six_hour_clock_line(reckoning, seconds).expect("a time");
        assert_eq!(
            cells(&line("ethiopian-hours", 8 * 3_600)),
            ["2", "0", "0", "day", "", ""]
        );
        assert_eq!(
            cells(&line("ethiopian-hours", 19 * 3_600)),
            ["1", "0", "0", "night", "", ""]
        );
        assert_eq!(
            cells(&line("Swahili-Hours", 7 * 3_600)),
            ["1", "0", "0", "day", "asubuhi", "morning"]
        );
        assert_eq!(
            cells(&line("swahili-hours", 19 * 3_600 + 5)),
            ["1", "0", "5", "night", "usiku", "night"]
        );
        assert_eq!(
            cells(&line("swahili-hours", 0)).len(),
            SIX_HOUR_CLOCK_COLUMNS
        );
        assert_eq!(
            civil_from_six_hour_clock("ethiopian-hours", 2, 0, 0, false),
            Ok(8 * 3_600)
        );
        assert_eq!(
            civil_from_six_hour_clock("swahili-hours", 1, 30, 0, true),
            Ok(19 * 3_600 + 30 * 60)
        );
        assert_eq!(
            civil_from_six_hour_clock("ethiopian-hours", 13, 0, 0, false),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(six_hour_clock_line("amharic", 0), Err(Refusal::Unknown));
        assert_eq!(
            six_hour_clock_line("ethiopian-hours", 86_400),
            Err(Refusal::OutOfRange)
        );
    }

    /// Figure 5-2 of IRIG 200-16 (`rcc-200-16`, as `hc-format`'s test
    /// reads it): IRIG B, day 173 of 2003, 22 June, at 21:18:42, the year
    /// 03, the control bits 0 and the straight binary seconds 76 722, in a
    /// B124 frame; and Figure 5-6, IRIG H at 21:24 on the same day, which
    /// carries no year and is read in the year given.
    #[test]
    fn the_standards_figures_cross_both_ways() {
        const FIGURE_5_2: &str = "M01000001M000101000M100000100M110001110M100000000M\
                                  110000000M000000000M000000000M010011011M101010010M";
        const FIGURE_5_6: &str = "M00000000M001000100M100000100M110001110M100000000M000000000M";
        let day = gregorian::to_fixed(2003, 6, 22).expect("a date").0;
        let decoded = irig_decode_line("B124", FIGURE_5_2, 2026).expect("the figure");
        let cells: alloc::vec::Vec<&str> = decoded.trim_end().split('\t').collect();
        assert_eq!(cells.len(), IRIG_DECODE_COLUMNS);
        assert_eq!(
            cells,
            [
                &*day.to_string(),
                "173",
                "21",
                "18",
                "42",
                "0",
                "3",
                "0",
                "76722"
            ]
        );
        let seconds = 21 * 3_600 + 18 * 60 + 42;
        let encoded = irig_encode_line("b124", day, seconds, 0, 0).expect("a frame");
        assert_eq!(encoded.trim_end(), FIGURE_5_2);
        let h =
            irig_decode_line("H001", &FIGURE_5_6.to_ascii_lowercase(), 2003).expect("the figure");
        assert_eq!(h, alloc::format!("{day}\t173\t21\t24\t0\t0\t\t0\t\n"));
        assert_eq!(
            irig_encode_line("H001", day, 21 * 3_600 + 24 * 60, 0, 0)
                .expect("a frame")
                .trim_end(),
            FIGURE_5_6
        );
        // The century of `year` reads the two digits; a year that is not
        // the frame's is refused for a code without them.
        let in_2100s = irig_decode_line("B124", FIGURE_5_2, 2150).expect("the figure");
        assert!(
            in_2100s.starts_with(
                &gregorian::to_fixed(2103, 6, 22)
                    .expect("a date")
                    .0
                    .to_string()
            )
        );
        assert_eq!(
            irig_decode_line("B112", FIGURE_5_2, 2026),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            irig_decode_line("B124", &FIGURE_5_2[1..], 2026),
            Err(Refusal::Malformed)
        );
        assert_eq!(
            irig_decode_line("B120", FIGURE_5_2, 2026),
            Err(Refusal::Malformed)
        );
        assert_eq!(
            irig_decode_line("B124", FIGURE_5_2, 0),
            Err(Refusal::OutOfRange)
        );
        // Format H has no frame off the minute, B none off the second, and
        // B124 carries 18 control bits.
        assert_eq!(
            irig_encode_line("H001", day, seconds, 0, 0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            irig_encode_line("B124", day, seconds, 50, 0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            irig_encode_line("B124", day, seconds, 0, 1 << 18),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            irig_encode_line("B124", 0, 0, 0, 0),
            Err(Refusal::OutOfRange)
        );
        // 23:59:60 at the end of 2016, whose straight binary seconds reach
        // 86 400, round-trips.
        let leap_day = gregorian::to_fixed(2016, 12, 31).expect("a date").0;
        let leap = irig_encode_line("B127", leap_day, 86_400, 0, 0).expect("a frame");
        let back = irig_decode_line("B127", leap.trim_end(), 2016).expect("a frame");
        assert_eq!(
            back,
            alloc::format!("{leap_day}\t366\t23\t59\t60\t0\t16\t\t86400\n")
        );
    }
}
