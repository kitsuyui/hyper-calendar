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
#[cfg(feature = "tz")]
use hc_format::radio::dcf77::summer_time;
use hc_format::radio::dcf77::{Dcf77Frame, Zone};
use hc_format::radio::jjy::{JjyContent, JjyFrame};
use hc_format::radio::wwvb::{AmFrame, DstNext, DstState, PmFrame};
use hc_format::radio::{FrameError, LeapNotice, Symbol};

use crate::boundary::{Answer, Refusal, names, push_cell};
use crate::time_lines::{tai_instant, tai_parts, unix_instant};

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
    let (pairs, _) = digits.as_chunks::<2>();
    for (octet, [high, low]) in octets.iter_mut().zip(pairs) {
        let (Some(high), Some(low)) = (hex_digit(*high), hex_digit(*low)) else {
            return Err(Refusal::Malformed);
        };
        *octet = high << 4 | low;
    }
    Ok((octets, digits.len() / 2))
}

const fn hex_digit(digit: u8) -> Option<u8> {
    match digit {
        b'0'..=b'9' => Some(digit - b'0'),
        b'a'..=b'f' => Some(digit - b'a' + 10),
        b'A'..=b'F' => Some(digit - b'A' + 10),
        _ => None,
    }
}

/// The five cells of an instant: its TAI seconds and attoseconds, and its
/// UTC label's POSIX second, leap-second flag and attoseconds.
fn push_instant(out: &mut String, tai: Instant<Tai>, utc: UtcInstant) -> Answer<()> {
    let (seconds, attoseconds) = tai_parts(tai)?;
    let _ = write!(
        out,
        "{seconds}\t{attoseconds}\t{}\t{}\t{}",
        utc.unix_seconds,
        u8::from(utc.leap_second),
        utc.subsec_attos
    );
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
    let mut out = String::from(name);
    out.push('\t');
    push_instant(&mut out, tai, utc)?;
    out.push('\n');
    Ok(out)
}

/// Octets as lower-case hexadecimal.
fn push_hex(out: &mut String, octets: &[u8]) {
    for octet in octets {
        let _ = write!(out, "{octet:02x}");
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
    let mut out = String::new();
    push_hex(&mut out, code.as_slice());
    out.push('\n');
    Ok(out)
}

/// How many columns [`ccsds_ascii_parse_line`] writes.
pub const CCSDS_ASCII_COLUMNS: usize = 9;

/// The name a variation is written as.
const fn variation_name(variation: AsciiVariation) -> &'static str {
    match variation {
        AsciiVariation::A => "a",
        AsciiVariation::B => "b",
    }
}

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
    let mut out = String::from(variation_name(code.variation));
    out.push('\t');
    push_instant(&mut out, tai, utc)?;
    match code.precision {
        AsciiPrecision::Hour => out.push_str("\thour\t"),
        AsciiPrecision::Minute => out.push_str("\tminute\t"),
        AsciiPrecision::Second => out.push_str("\tsecond\t"),
        AsciiPrecision::Fraction(digits) => {
            let _ = write!(out, "\tfraction\t{digits}");
        }
    }
    let _ = writeln!(out, "\t{}", u8::from(code.terminator));
    Ok(out)
}

/// A precision as a line names it: `hour`, `minute`, `second`, or the
/// digits of the fraction, `1` to `18`.
fn ascii_precision(name: &str) -> Answer<AsciiPrecision> {
    let name = name.trim();
    if names(name, "hour") {
        return Ok(AsciiPrecision::Hour);
    }
    if names(name, "minute") {
        return Ok(AsciiPrecision::Minute);
    }
    if names(name, "second") {
        return Ok(AsciiPrecision::Second);
    }
    let digits = match name.as_bytes() {
        [digit @ b'1'..=b'9'] => digit - b'0',
        [b'1', digit @ b'0'..=b'8'] => 10 + digit - b'0',
        _ => return Err(Refusal::Unknown),
    };
    Ok(AsciiPrecision::Fraction(digits))
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
    let variation = if names(variation, "a") {
        AsciiVariation::A
    } else if names(variation, "b") {
        AsciiVariation::B
    } else {
        return Err(Refusal::Unknown);
    };
    let precision = ascii_precision(precision)?;
    let utc = unix::utc_from_tai(instant, policy(strict))?;
    let code = AsciiTime::from_utc_instant(utc, variation, precision, terminator)
        .map_err(|_| Refusal::OutOfRange)?;
    let mut out = String::new();
    code.write(&mut out).map_err(|_| Refusal::OutOfRange)?;
    out.push('\n');
    Ok(out)
}

/// The radio codes `hc_radio_decode` and `hc_radio_encode` read and write,
/// by name.
pub const RADIO_CODES: [&str; 4] = ["jjy", "dcf77", "wwvb-am", "wwvb-pm"];

/// Which radio code.
#[derive(Clone, Copy, PartialEq, Eq)]
enum RadioCode {
    Jjy,
    Dcf77,
    WwvbAm,
    WwvbPm,
}

fn radio_code(name: &str) -> Answer<RadioCode> {
    [
        RadioCode::Jjy,
        RadioCode::Dcf77,
        RadioCode::WwvbAm,
        RadioCode::WwvbPm,
    ]
    .into_iter()
    .zip(RADIO_CODES)
    .find(|(_, id)| names(name, id))
    .map(|(code, _)| code)
    .ok_or(Refusal::Unknown)
}

/// The longest frame, a minute with an inserted leap second.
const MAX_FRAME: usize = 61;

/// A frame written one character a second: `0`, `1`, and for the codes
/// with a marker `M`, in either case; white space around it allowed.
///
/// # Errors
///
/// [`Refusal::Malformed`] for any other character, a marker in a code of
/// bits, or more than 61 seconds.
fn frame_symbols(text: &str, markers: bool) -> Answer<([Symbol; MAX_FRAME], usize)> {
    let text = text.trim().as_bytes();
    if text.len() > MAX_FRAME {
        return Err(Refusal::Malformed);
    }
    let mut symbols = [Symbol::Zero; MAX_FRAME];
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
    let (symbols, len) = frame_symbols(text, false)?;
    Ok((symbols.map(|symbol| symbol == Symbol::One), len))
}

/// The refusal a frame error is: a frame that is not the code's.
const fn frame_refusal(_: FrameError) -> Refusal {
    Refusal::Malformed
}

const fn leap_name(leap: LeapNotice) -> &'static str {
    match leap {
        LeapNotice::None => "none",
        LeapNotice::Positive => "positive",
        LeapNotice::Negative => "negative",
    }
}

const fn dst_name(dst: DstState) -> &'static str {
    match dst {
        DstState::Standard => "standard",
        DstState::BeginsToday => "begins-today",
        DstState::InEffect => "in-effect",
        DstState::EndsToday => "ends-today",
    }
}

fn dst_state(name: &str) -> Answer<DstState> {
    [
        DstState::Standard,
        DstState::BeginsToday,
        DstState::InEffect,
        DstState::EndsToday,
    ]
    .into_iter()
    .find(|state| names(name, dst_name(*state)))
    .ok_or(Refusal::Unknown)
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
/// [`RADIO_CODES`], a string of `0`, `1` and, for `jjy` and `wwvb-am`,
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
            let (symbols, len) = frame_symbols(frame, true)?;
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
                summer: match frame.zone {
                    Zone::Cet => "cet",
                    Zone::Cest => "cest",
                },
                zone_change: Some(frame.zone_change),
                dut1_tenths: None,
                dst_next: None,
            }
        }
        RadioCode::WwvbAm => {
            let (symbols, len) = frame_symbols(frame, true)?;
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
                summer: dst_name(frame.dst),
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
                summer: dst_name(frame.dst),
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
    let mut out = String::new();
    let _ = write!(
        out,
        "{unix}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t",
        reading.day.0,
        reading.time.hour(),
        reading.time.minute(),
        decoded.offset_hours,
        decoded.seconds,
        leap_name(decoded.leap),
        decoded.summer,
    );
    if let Some(change) = decoded.zone_change {
        let _ = write!(out, "{}", u8::from(change));
    }
    out.push('\t');
    if let Some(tenths) = decoded.dut1_tenths {
        let _ = write!(out, "{tenths}");
    }
    out.push('\t');
    if let Some(word) = decoded.dst_next {
        let _ = write!(out, "{word}");
    }
    out.push('\n');
    Ok(out)
}

/// The first and last POSIX seconds of the years 1 to 9999, the minutes
/// `hc_radio_encode` writes a frame for.
const RADIO_UNIX_RANGE: core::ops::RangeInclusive<i64> = -62_135_596_800..=253_402_300_799;

/// The line of `hc_radio_encode`: the frame of a radio code of
/// [`RADIO_CODES`] for the minute that begins at a POSIX second, as a
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
/// zone's rules instead of the caller, in a build with the `tz` feature: for `dcf77` the zone and A1 from
/// [`dcf77::summer_time`](hc_format::radio::dcf77::summer_time), for both
/// WWVB codes the state of the minute's UTC day from
/// [`DstState::of_day`]. The other arguments are
/// [`radio_encode_line`]'s; the phase code's `dst_next` stays the caller's
/// and must be one Table 8 lists for the direction the rules give.
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
    dst_next: u32,
) -> Answer<String> {
    encode_radio(
        code,
        unix_seconds,
        leap,
        Summer::Rules(zone),
        dut1_tenths,
        dst_next,
    )
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
                if names(summer, "cet") {
                    Ok((Zone::Cet, zone_change))
                } else if names(summer, "cest") {
                    Ok((Zone::Cest, zone_change))
                } else {
                    Err(Refusal::Unknown)
                }
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
            Self::Named { summer, .. } => dst_state(summer),
            #[cfg(feature = "tz")]
            Self::Rules(zone) => Ok(DstState::of_day(zone, UnixTime::from_seconds(unix_seconds))),
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
    let mut out = String::new();
    let mut push_symbols = |symbols: &[Symbol]| {
        for symbol in symbols {
            out.push(match symbol {
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
            let word = u8::try_from(dst_next)
                .ok()
                .filter(|word| *word < 64)
                .ok_or(Refusal::OutOfRange)?;
            let next = DstNext::from_word(word, dst.bits().0).ok_or(Refusal::OutOfRange)?;
            let reading = minute_at(0)?;
            let year = gregorian::year_from_fixed(reading.day);
            let frame = PmFrame::for_minute(reading, year - year.rem_euclid(100), dst, leap, next)
                .map_err(range)?;
            push_symbols(&bits(frame.encode().map_err(range)?.as_slice()));
        }
    }
    out.push('\n');
    Ok(out)
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
    Ok(alloc::format!(
        "{}\t{}\n",
        reading.seconds(),
        reading.subsec_attos()
    ))
}

/// The six-hour reckoning an identifier names, `ethiopian-hours` or
/// `swahili-hours`, in any ASCII case.
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other.
pub fn six_hour_reckoning(id: &str) -> Answer<&'static Reckoning> {
    east_african_hours::ALL
        .iter()
        .find(|reckoning| names(id, reckoning.id))
        .ok_or(Refusal::Unknown)
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
    let mut out = String::new();
    let _ = write!(
        out,
        "{}\t{}\t{}\t{}\t",
        reading.hour,
        reading.minute,
        reading.second,
        match reading.half {
            Half::Day => "day",
            Half::Night => "night",
        }
    );
    if let Some(period) = reckoning.period(civil) {
        push_cell(&mut out, period.name);
        out.push('\t');
        push_cell(&mut out, period.english);
    } else {
        out.push('\t');
    }
    out.push('\n');
    Ok(out)
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
) -> Answer<i64> {
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
        i64::from(civil.hour()) * 3_600
            + i64::from(civil.minute()) * 60
            + i64::from(civil.second()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    fn cells(line: &str) -> Vec<&str> {
        line.trim_end_matches('\n').split('\t').collect()
    }

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

    /// `DateTime.MaxValue`, 23:59:59.9999999 on 9999-12-31, is 3 155 378 975
    /// 999 999 999 ticks (`ms-datetime-maxvalue`), and 1970-01-01 is
    /// 621 355 968 000 000 000.
    /// The stations' examples again with the summer-time state read from
    /// the zones' rules — Europe/Berlin's for DCF77, America/New_York's,
    /// on the United States' rule as Denver, for WWVB — and the frames
    /// announcing 2026's European changes, which carry A1.
    #[cfg(feature = "tz")]
    #[test]
    fn a_zones_rules_give_the_frames_the_caller_would() {
        let berlin = hc_format::hc_tz::builtin::zone("Europe/Berlin").expect("built in");
        let new_york = hc_format::hc_tz::builtin::zone("America/New_York").expect("built in");
        let named = |code, unix, summer, change, dut1, next| {
            radio_encode_line(code, unix, 0, summer, change, dut1, next).expect("a minute")
        };
        let ruled = |code, unix, zone: &dyn TimeZone, dut1, next| {
            radio_encode_line_by_zone(code, unix, 0, zone, dut1, next).expect("a minute")
        };
        assert_eq!(
            ruled("dcf77", 1_790_512_200, &berlin, 0, 0).trim_end(),
            DCF77
        );
        assert_eq!(
            ruled("wwvb-am", 1_341_423_000, &new_york, 4, 0).trim_end(),
            WWVB_AM
        );
        // Table 10's frame but for its notice and reserved bits, which
        // `encode` leaves 0.
        assert_eq!(
            ruled("wwvb-pm", 1_341_423_000, &new_york, 0, 27),
            named("wwvb-pm", 1_341_423_000, "in-effect", false, 0, 27)
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
                    ruled("dcf77", minute, &berlin, 0, 0),
                    named("dcf77", minute, summer, a1, 0, 0),
                    "{minute}"
                );
            }
        }
        // 2026-03-08, the United States' change: bit 57 from 00:00 UTC.
        assert_eq!(
            ruled("wwvb-am", 1_772_928_000, &new_york, 0, 0),
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
            radio_encode_line_by_zone("dcf77", 1_790_512_200, 0, &london, 0, 0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            radio_encode_line_by_zone("jjy", 1_080_807_900, 0, &berlin, 0, 0),
            Err(Refusal::Unknown)
        );
        // Summer time in effect in July; 27 is a word for leaving it.
        assert_eq!(
            radio_encode_line_by_zone("wwvb-pm", 1_341_423_000, 0, &new_york, 0, 0),
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
}
