//! The tab-separated lines the WebAssembly module and the C library write
//! about exactly defined units of time, written once.
//!
//! Everything here reads [`hc_units`]: every unit in it is a fixed multiple
//! of the SI second by some authority, so every length is an exact rational
//! and conversions compose without losing anything. **No cell here is a
//! float.** A ratio crosses the boundary as two cells, a numerator and a
//! denominator, in lowest terms and written in decimal, because they are
//! 128-bit integers and a quectosecond is 10⁻³⁰ of a second: a double would
//! hold neither the 30 decades nor the 705 600 000th that a flick is. The
//! count a caller gives is a pair of `i64`s for the same reason.

use alloc::string::String;

use hc_units::media::{FRAME_RATES, SAMPLE_RATES};
use hc_units::tempo::{NoteValue, Tempo};
use hc_units::unit::{ALL, FLICK, Family, Quantity, Unit, by_id};
use hc_units::{Rate, Ratio, UnitError};

use crate::boundary::{Answer, Line, Refusal};

/// How many columns a line of [`units_lines`] has.
pub const UNIT_COLUMNS: usize = 7;

/// How many columns a line of [`unit_convert_line`] has.
pub const UNIT_CONVERT_COLUMNS: usize = 10;

/// How many columns a line of [`rates_lines`] has.
pub const RATE_COLUMNS: usize = 4;

/// How many columns a line of [`frame_period_line`] has.
pub const FRAME_PERIOD_COLUMNS: usize = 11;

/// How many columns a line of [`tempo_line`] has.
pub const TEMPO_COLUMNS: usize = 10;

/// The refusal an error of the library is: a number that left the 128 bits
/// is [`Refusal::Overflow`], and every other a value out of range.
fn refusal(error: UnitError) -> Refusal {
    match error {
        UnitError::Overflow => Refusal::Overflow,
        _ => Refusal::OutOfRange,
    }
}

/// A ratio as two cells, its numerator and its denominator in lowest terms.
fn ratio_cells(line: &mut Line<'_>, ratio: Ratio) {
    line.value(ratio.numerator()).value(ratio.denominator());
}

/// The identifier of a family of units.
const fn family_id(family: Family) -> &'static str {
    match family {
        Family::Si => "si",
        Family::Civil => "civil",
        Family::Horological => "horological",
        Family::Decimal => "decimal",
        Family::Hexadecimal => "hexadecimal",
        Family::Media => "media",
        Family::Scientific => "scientific",
        _ => "other",
    }
}

/// The unit with an identifier, in any ASCII case.
fn unit_of(id: &str) -> Answer<Unit> {
    by_id(id).ok_or(Refusal::Unknown)
}

/// Every unit of time with an exactly defined length, shortest first, one
/// line each: the identifier, the English name, the symbol (empty where
/// there is none), the length in seconds as a numerator and a denominator in
/// lowest terms, the family (`si`, `civil`, `horological`, `decimal`,
/// `hexadecimal`, `media` or `scientific`), and the authority that defines
/// it.
///
/// A unit that is measured rather than defined, such as the sidereal day or
/// the tropical year, is not here: it lives with the model that measured it.
#[must_use]
pub fn units_lines() -> String {
    let mut out = String::new();
    for unit in ALL {
        let mut line = Line::new(&mut out);
        line.cell(unit.id)
            .cell(unit.name)
            .cell_or_empty(unit.symbol);
        ratio_cells(&mut line, unit.seconds);
        line.cell(family_id(unit.family)).cell(unit.authority);
        line.end();
    }
    out
}

/// A count of one unit written in another, exactly, as one line: the two
/// unit identifiers, the count given as a numerator and a denominator, the
/// count in the other unit as a numerator and a denominator, `1` where that
/// count is a whole number, the length in seconds as a numerator and a
/// denominator, and `1` where an attosecond count holds that length exactly
/// (a flick, a third of a second and an NTSC frame are lengths no
/// `Duration` holds).
///
/// The count is `count_numerator / count_denominator`; it may be negative.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a unit that [`units_lines`] does not list,
/// [`Refusal::OutOfRange`] for a denominator of 0, and [`Refusal::Overflow`]
/// for a count or a length whose numerator or denominator leaves 128 bits.
pub fn unit_convert_line(
    count_numerator: i64,
    count_denominator: i64,
    from: &str,
    to: &str,
) -> Answer<String> {
    let (from, to) = (unit_of(from)?, unit_of(to)?);
    let count =
        Ratio::new(i128::from(count_numerator), i128::from(count_denominator)).map_err(refusal)?;
    let given = Quantity::new(count, from);
    let converted = given.to(to).map_err(refusal)?;
    let seconds = given.seconds().map_err(refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(from.id).cell(to.id);
    ratio_cells(&mut line, count);
    ratio_cells(&mut line, converted.count);
    line.flag(converted.count.is_integer());
    ratio_cells(&mut line, seconds);
    line.flag(seconds.to_duration().is_ok());
    line.end();
    Ok(out)
}

/// What kind of rate a table holds it in.
const FRAME: &str = "frame";
/// The kind of a sample rate.
const SAMPLE: &str = "sample";

/// Every frame rate and sample rate the crate carries as an exact period,
/// one line each: the identifier, the kind (`frame` or `sample`), and the
/// rate in events per second as a numerator and a denominator.
///
/// The NTSC rates are exact: 29.97 is 30 000 / 1 001. Each identifier is
/// what [`frame_period_line`] reads for its rate.
#[must_use]
pub fn rates_lines() -> String {
    let mut out = String::new();
    for (kind, table) in [(FRAME, FRAME_RATES), (SAMPLE, SAMPLE_RATES)] {
        for rate in table {
            let mut line = Line::new(&mut out);
            line.cell(rate.id).cell(kind);
            ratio_cells(&mut line, rate.hertz);
            line.end();
        }
    }
    out
}

/// A rate named by a table's identifier, or given as an exact `n` or `n/d`
/// events per second, with the kind that says which.
fn rate_of(given: &str) -> Answer<(Rate, &'static str)> {
    let given = given.trim();
    for (kind, table) in [(FRAME, FRAME_RATES), (SAMPLE, SAMPLE_RATES)] {
        if let Some(rate) = table
            .iter()
            .find(|rate| hc_core::catalogue::matches(given, rate.id))
        {
            return Ok((*rate, kind));
        }
    }
    let (numerator, denominator) = match given.split_once('/') {
        Some((numerator, denominator)) => (numerator, denominator),
        None => (given, "1"),
    };
    let read = |text: &str| text.trim().parse::<i64>().map_err(|_| Refusal::Unknown);
    let hertz = Ratio::new(i128::from(read(numerator)?), i128::from(read(denominator)?))
        .map_err(refusal)?;
    if hertz.is_zero() || hertz.is_negative() {
        return Err(Refusal::OutOfRange);
    }
    Ok((
        Rate {
            id: "custom",
            hertz,
        },
        "custom",
    ))
}

/// The length of one frame or one sample, exactly, as one line: the rate's
/// identifier (`custom` for one given as a fraction) and kind (`frame`,
/// `sample` or `custom`), the rate in events per second as a numerator and a
/// denominator, the length of one event in seconds as a numerator and a
/// denominator, the length counted in a unit as a numerator and a
/// denominator, the unit's identifier, `1` where it is a whole number of
/// that unit, and `1` where it is a whole number of flicks (the unit chosen so
/// that every common frame and sample rate is).
///
/// `rate` is an identifier of [`rates_lines`], such as `29.97` or `48000`, or
/// an exact rate written `n` or `n/d` events per second, such as `30000/1001`.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a rate that is neither an identifier nor such a
/// fraction, and for a unit [`units_lines`] does not list;
/// [`Refusal::OutOfRange`] for a rate that is not positive; and
/// [`Refusal::Overflow`] for a length that leaves 128 bits.
pub fn frame_period_line(rate: &str, in_unit: &str) -> Answer<String> {
    let (rate, kind) = rate_of(rate)?;
    let unit = unit_of(in_unit)?;
    let period = rate.period().map_err(refusal)?;
    let counted = rate.one_in(unit).map_err(refusal)?;
    let flicks = rate.one_in(FLICK).map_err(refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(rate.id).cell(kind);
    ratio_cells(&mut line, rate.hertz);
    ratio_cells(&mut line, period);
    ratio_cells(&mut line, counted.count);
    line.cell(unit.id)
        .flag(counted.count.is_integer())
        .flag(flicks.count.is_integer());
    line.end();
    Ok(out)
}

/// A note at a tempo, exactly, as one line: the tempo in beats per minute as
/// a numerator and a denominator, the length of one beat in seconds as a
/// numerator and a denominator, the note's length as a fraction of a whole
/// note as a numerator and a denominator, the note's length in seconds as a
/// numerator and a denominator, the microseconds per quarter note a MIDI
/// `Set Tempo` event stores (empty where the tempo does not fit its 24 bits),
/// and `1` where that integer holds the tempo exactly.
///
/// The tempo is `bpm_numerator / bpm_denominator` beats per minute, where a
/// beat is the note of `beat_halvings` halvings of a whole note: 2 is a
/// quarter note, 3 an eighth. The note is `note_halvings` halvings, with
/// `dots` augmentation dots, each adding half of what came before (one dot
/// makes it 3/2 as long, two 7/4), and, when `tuplet_count` and `tuplet_space`
/// are both above 0, `tuplet_count` of it in the time of `tuplet_space`: a
/// triplet is 3 in the time of 2. Both 0 is no tuplet.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a tempo that is not positive, a denominator of
/// 0, a tuplet with only one of its two numbers 0, and halvings or dots past
/// what a ratio holds; and [`Refusal::Overflow`] for a length that leaves 128
/// bits.
pub fn tempo_line(
    bpm_numerator: i64,
    bpm_denominator: i64,
    note_halvings: u32,
    dots: u32,
    tuplet_space: u32,
    tuplet_count: u32,
    beat_halvings: u32,
) -> Answer<String> {
    let halvings = |count: u32| u8::try_from(count).map_err(|_| Refusal::OutOfRange);
    let bpm =
        Ratio::new(i128::from(bpm_numerator), i128::from(bpm_denominator)).map_err(refusal)?;
    let tempo = Tempo::from_bpm_ratio(bpm).map_err(refusal)?;
    let mut note = NoteValue::new(halvings(note_halvings)?).dotted(halvings(dots)?);
    match (tuplet_space, tuplet_count) {
        (0, 0) => {}
        (space, count) if space > 0 && count > 0 => note = note.tuplet(space, count),
        _ => return Err(Refusal::OutOfRange),
    }
    let beat = NoteValue::new(halvings(beat_halvings)?);
    let fraction = note.fraction_of_whole().map_err(refusal)?;
    let length = note.duration_at(tempo, beat).map_err(refusal)?;
    let beat_seconds = tempo.beat().map_err(refusal)?;
    // MIDI stores microseconds per quarter note, whatever the beat is.
    let quarter = NoteValue::QUARTER
        .duration_at(tempo, beat)
        .map_err(refusal)?;
    let quarter_tempo = Ratio::from_secs(60)
        .checked_div(quarter)
        .and_then(Tempo::from_bpm_ratio)
        .map_err(refusal)?;
    let midi = quarter_tempo.to_midi_micros_per_beat().ok();
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    ratio_cells(&mut line, bpm);
    ratio_cells(&mut line, beat_seconds);
    ratio_cells(&mut line, fraction);
    ratio_cells(&mut line, length);
    match midi {
        Some((micros, exact)) => line.value(micros).flag(exact),
        None => line.empties(2),
    };
    line.end();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::cells;

    fn row(text: &str) -> alloc::vec::Vec<String> {
        cells(text).iter().map(|cell| (*cell).to_owned()).collect()
    }

    #[test]
    fn the_catalogue_has_all_fifty_three_units_with_their_lengths() {
        let text = units_lines();
        assert_eq!(text.lines().count(), ALL.len());
        assert_eq!(ALL.len(), 53);
        assert!(
            text.lines()
                .all(|line| line.split('\t').count() == UNIT_COLUMNS)
        );
        let find = |id: &str| {
            row(text
                .lines()
                .find(|line| line.split('\t').next() == Some(id))
                .unwrap_or_default())
        };
        // A helek is 3 1/3 s (a 1080th of an hour), the Hebrew calendar's
        // unit; a flick is a 705 600 000th of a second (Facebook's Flicks);
        // a Swatch beat and a French decimal minute are both 86.4 s.
        assert_eq!(&find("helek")[3..5], ["10", "3"]);
        assert_eq!(&find("flick")[3..5], ["1", "705600000"]);
        assert_eq!(&find("beat")[3..5], ["432", "5"]);
        assert_eq!(&find("decimal-minute")[3..5], ["432", "5"]);
        assert_eq!(&find("day")[3..5], ["86400", "1"]);
        assert_eq!(&find("julian-year")[3..5], ["31557600", "1"]);
        // The smallest SI prefix has all thirty decades.
        let smallest = find("quectosecond");
        assert_eq!(smallest[2], "qs");
        assert_eq!(smallest[4], "1000000000000000000000000000000");
        assert_eq!(find("flick")[5], "media");
        assert_eq!(find("second")[2], "s");
        // A unit with no symbol has an empty cell.
        assert_eq!(find("helek")[2], "");
        assert_eq!(find("helek")[5], "horological");
    }

    #[test]
    fn a_conversion_is_exact_and_says_so() {
        // Two hours are 7 200 000 milliseconds, and a whole number.
        let line = unit_convert_line(2, 1, "hour", "millisecond").unwrap_or_default();
        assert_eq!(
            row(&line),
            [
                "hour",
                "millisecond",
                "2",
                "1",
                "7200000",
                "1",
                "1",
                "7200",
                "1",
                "1"
            ]
        );
        // An hour is 1080 chalakim; three hours and a third of one is not
        // whole in seconds either, and a helek is no attosecond count.
        let line = unit_convert_line(1, 1, "hour", "helek").unwrap_or_default();
        assert_eq!(&row(&line)[4..7], ["1080", "1", "1"]);
        let line = unit_convert_line(1, 1, "helek", "second").unwrap_or_default();
        assert_eq!(&row(&line)[4..], ["10", "3", "0", "10", "3", "0"]);
        // A flick is not a whole attosecond count: 1e18 / 705 600 000, reduced,
        // is 625 000 000 000 / 441, which is 1 417 233 560.09 as.
        let line = unit_convert_line(1, 1, "flick", "attosecond").unwrap_or_default();
        let cells = row(&line);
        assert_eq!(&cells[4..6], ["625000000000", "441"]);
        assert_eq!(cells[6], "0");
        assert_eq!(cells[9], "0");
        // A Julian year is 1461/4 days.
        let line = unit_convert_line(1, 1, "julian-year", "day").unwrap_or_default();
        assert_eq!(&row(&line)[4..7], ["1461", "4", "0"]);
        // A negative fraction of a unit.
        let line = unit_convert_line(-3, 2, "minute", "second").unwrap_or_default();
        assert_eq!(&row(&line)[2..7], ["-3", "2", "-90", "1", "1"]);
        // The ratio is reduced: 6/4 is 3/2.
        assert_eq!(
            &row(&unit_convert_line(6, 4, "second", "second").unwrap_or_default())[2..4],
            ["3", "2"]
        );
        assert_eq!(
            unit_convert_line(1, 0, "second", "day"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            unit_convert_line(1, 1, "second", "furlong"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            unit_convert_line(1, 1, "parsec", "second"),
            Err(Refusal::Unknown)
        );
        // A quectosecond to a Julian millennium overflows 128 bits.
        assert_eq!(
            unit_convert_line(i64::MAX, 1, "julian-millennium", "quectosecond"),
            Err(Refusal::Overflow)
        );
    }

    #[test]
    fn every_frame_and_sample_rate_is_listed_with_its_period() {
        let text = rates_lines();
        assert_eq!(text.lines().count(), FRAME_RATES.len() + SAMPLE_RATES.len());
        assert!(
            text.lines()
                .all(|line| line.split('\t').count() == RATE_COLUMNS)
        );
        assert_eq!(
            row(text
                .lines()
                .find(|line| line.starts_with("29.97\t"))
                .unwrap_or_default()),
            ["29.97", "frame", "30000", "1001"]
        );
        // Every listed rate is a whole number of flicks, which is why the
        // flick exists, and has an exact period.
        for line in text.lines() {
            let id = line.split('\t').next().unwrap_or_default();
            let period = frame_period_line(id, "flick").unwrap_or_default();
            assert_eq!(
                row(&period)[8..],
                ["flick", "1", "1"].map(String::from),
                "{id}"
            );
        }
        // 24 fps is 1/24 s and 29 400 000 flicks.
        let line = frame_period_line("24", "flick").unwrap_or_default();
        assert_eq!(
            row(&line),
            [
                "24", "frame", "24", "1", "1", "24", "29400000", "1", "flick", "1", "1"
            ]
        );
        // The NTSC frame is 1001/30000 s, which is no whole number of
        // milliseconds but is of flicks: 23 543 520.
        let line = frame_period_line("29.97", "millisecond").unwrap_or_default();
        let cells = row(&line);
        assert_eq!(&cells[4..6], ["1001", "30000"]);
        assert_eq!(&cells[6..8], ["1001", "30"]);
        assert_eq!(cells[9], "0");
        assert_eq!(cells[10], "1");
        let line = frame_period_line("29.97", "flick").unwrap_or_default();
        assert_eq!(&row(&line)[6..8], ["23543520", "1"]);
        // CD audio: 44 100 samples a second.
        assert_eq!(
            &row(&frame_period_line("44100", "second").unwrap_or_default())[..2],
            ["44100", "sample"]
        );
        // A rate given as a fraction.
        let line = frame_period_line("30000/1001", "second").unwrap_or_default();
        assert_eq!(&row(&line)[..4], ["custom", "custom", "30000", "1001"]);
        assert_eq!(
            &row(&frame_period_line("25", "second").unwrap_or_default())[..2],
            ["25", "frame"]
        );
        assert_eq!(frame_period_line("0", "second"), Err(Refusal::OutOfRange));
        assert_eq!(frame_period_line("-24", "second"), Err(Refusal::OutOfRange));
        assert_eq!(frame_period_line("1/0", "second"), Err(Refusal::OutOfRange));
        assert_eq!(frame_period_line("fast", "second"), Err(Refusal::Unknown));
        assert_eq!(frame_period_line("24", "furlong"), Err(Refusal::Unknown));
    }

    #[test]
    fn a_dotted_triplet_at_a_tempo_is_a_fraction_of_a_second() {
        // 120 BPM is a beat of exactly a half second, and the MIDI default
        // 500 000 microseconds a quarter, exactly.
        let line = tempo_line(120, 1, 2, 0, 0, 0, 2).unwrap_or_default();
        assert_eq!(
            row(&line),
            ["120", "1", "1", "2", "1", "4", "1", "2", "500000", "1"]
        );
        // 138 BPM, a dotted quarter in a triplet (3/8 times 2/3 = 1/4 of a
        // whole note, one beat) is 10/23 s.
        let line = tempo_line(138, 1, 2, 1, 2, 3, 2).unwrap_or_default();
        assert_eq!(&row(&line)[4..8], ["1", "4", "10", "23"]);
        // A dotted quarter is 3/8 of a whole note; double-dotted 7/16; an
        // eighth-note triplet 1/12.
        assert_eq!(
            &row(&tempo_line(120, 1, 2, 1, 0, 0, 2).unwrap_or_default())[4..6],
            ["3", "8"]
        );
        assert_eq!(
            &row(&tempo_line(120, 1, 2, 2, 0, 0, 2).unwrap_or_default())[4..6],
            ["7", "16"]
        );
        assert_eq!(
            &row(&tempo_line(120, 1, 3, 0, 2, 3, 2).unwrap_or_default())[4..6],
            ["1", "12"]
        );
        // 140 BPM is 428 571.428... microseconds a quarter: the MIDI
        // integer is short and says so.
        let line = tempo_line(140, 1, 2, 0, 0, 0, 2).unwrap_or_default();
        assert_eq!(&row(&line)[8..], ["428571", "0"]);
        // MIDI stores the quarter whatever the beat is: at 60 beats of eighth
        // notes a minute a quarter is two seconds.
        assert_eq!(
            row(&tempo_line(60, 1, 2, 0, 0, 0, 3).unwrap_or_default())[8..],
            ["2000000", "1"].map(String::from)
        );
        // 3 BPM does not fit MIDI's 24 bits.
        let slow = tempo_line(3, 1, 2, 0, 0, 0, 2).unwrap_or_default();
        assert_eq!(&row(&slow)[8..], ["", ""]);
        // A fractional tempo is exact: 120.5 BPM.
        let line = tempo_line(241, 2, 2, 0, 0, 0, 2).unwrap_or_default();
        assert_eq!(&row(&line)[..4], ["241", "2", "120", "241"]);
        assert_eq!(tempo_line(0, 1, 2, 0, 0, 0, 2), Err(Refusal::OutOfRange));
        assert_eq!(tempo_line(-120, 1, 2, 0, 0, 0, 2), Err(Refusal::OutOfRange));
        assert_eq!(tempo_line(120, 0, 2, 0, 0, 0, 2), Err(Refusal::OutOfRange));
        assert_eq!(tempo_line(120, 1, 2, 0, 0, 3, 2), Err(Refusal::OutOfRange));
        assert_eq!(
            tempo_line(120, 1, 2, 300, 0, 0, 2),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            tempo_line(120, 1, 300, 0, 0, 0, 2),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(tempo_line(120, 1, 200, 0, 0, 0, 2), Err(Refusal::Overflow));
        assert_eq!(tempo_line(120, 1, 2, 130, 0, 0, 2), Err(Refusal::Overflow));
    }
}
