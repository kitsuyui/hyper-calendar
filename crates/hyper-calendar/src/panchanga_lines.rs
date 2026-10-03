//! The tab-separated lines the WebAssembly module and the C library write
//! about the pañcāṅga's yoga and karaṇa, written once.
//!
//! Each answer is two lines of the same nine columns, the yoga's first
//! and the karaṇa's second: the limb (`yoga` or `karana`), its number
//! (the yoga 1 for Viṣkambha through 27 for Vaidhṛti; the karaṇa the
//! half-tithi, 1 for the first half of śukla 1 through 60 for the second
//! half of amāvāsyā), its name as Drik Panchang spells it in English and
//! in Devanagari, the moments it began and ends and the moment it was read
//! at, each as whole POSIX seconds of Universal Time, rounded down, and
//! the ayanāṃśa the yoga was reckoned with, by the identifier [`ayanamsa`]
//! reads back and by its full name, both empty for the karaṇa, which needs
//! none. The arithmetic and the names are
//! [`hc_calendars_indic::panchanga`]'s; the instants are held to the sky
//! layer's era, [`crate::astro_lines`].
//!
//! Rāhu kālam, Yamaganda and Gulika kālam, [`hc_calendars_indic::kalam`],
//! are three more lines of their own, one a period, each named in a
//! locale, so in a build with `i18n` too: `kalam_lines`.

use alloc::string::String;

use hc_astro::riseset::{Location, sunrise};
use hc_calendar::Rd;
use hc_calendar::fixed::Moment;
use hc_calendars_indic::HinduLunarCalendar;
use hc_calendars_indic::nakshatra::{nakshatra_at, nakshatra_id, nakshatra_name, nakshatra_span};
use hc_calendars_indic::panchanga::{
    KARANA_NAMES, KARANA_NAMES_DEVANAGARI, YOGA_NAMES, YOGA_NAMES_DEVANAGARI, karana_at,
    karana_name, karana_name_surya_siddhanta, karana_span, yoga_at, yoga_span,
};
use hc_calendars_indic::surya_siddhanta::{self, MAX_SUNRISE_LATITUDE};
use hc_calendars_indic::tithi::{self, Paksha, paksha_of, tithi_name};
use hc_calendars_indic::{amrita_siddhi, muhurta, panchanga, vaishnava};
use hc_core::catalogue::matches;
use hc_core::math::floor;
use hc_seasons::zodiac::{Ayanamsa, AyanamsaKind, SiderealSign, node};

#[cfg(feature = "i18n")]
use hc_calendars_indic::kalam::{Kalam, KalamConvention, SpanClock};

use crate::astro_lines::{MISSING_COLUMNS, missing_cells};
use crate::astro_lines::{day_in_era, moment_in_era, moment_on_day, unix_from_moment};
#[cfg(feature = "i18n")]
use crate::boundary::reckoning_name;
use crate::boundary::{Answer, Line, Refusal};

/// The name of the *Sūrya Siddhānta*'s sky, beside the ayanāṃśa names of
/// the true one.
pub use hc_calendars_indic::surya_siddhanta::SKY as SURYA_SIDDHANTA;

/// The sky's full name, as the ayanāṃśa's full name is written beside its
/// identifier.
pub const SURYA_SIDDHANTA_NAME: &str = "Sūrya Siddhānta";

/// The first fixed day the Siddhānta's exports answer for: Chaitra śukla 1
/// of Kali Yuga 1 on `hindu-lunar-surya-siddhanta`, 13 January 3101 BCE.
pub const SIDDHANTA_FIRST_DAY: i64 = -1_132_604;

/// The last fixed day the Siddhānta's exports answer for, the last of Kali
/// Yuga 10 000 on the same calendar, 15 June 6900.
pub const SIDDHANTA_LAST_DAY: i64 = 2_519_974;

/// A fixed day the Siddhānta's exports answer for.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] outside [`SIDDHANTA_FIRST_DAY`] to
/// [`SIDDHANTA_LAST_DAY`].
pub(crate) fn siddhanta_day(fixed: i64) -> Answer<Rd> {
    if (SIDDHANTA_FIRST_DAY..=SIDDHANTA_LAST_DAY).contains(&fixed) {
        Ok(Rd(fixed))
    } else {
        Err(Refusal::OutOfRange)
    }
}

/// A place where the Sun rises every day, on either sky: within
/// [`MAX_SUNRISE_LATITUDE`] of the equator. A lunisolar month begins at the
/// first sunrise after a conjunction, so a place with even one day of the
/// year without a sunrise has months that cannot be read there.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] beyond it.
pub(crate) fn sunrise_place(place: Location) -> Answer<Location> {
    if place.latitude_degrees.abs() <= MAX_SUNRISE_LATITUDE {
        Ok(place)
    } else {
        Err(Refusal::OutOfRange)
    }
}

/// The ayanāṃśa an identifier names: one of [`Ayanamsa::ALL`] by
/// [`Ayanamsa::by_id`], `lahiri`, `lahiri-rashtriya`, `lahiri-crc-1955`, `lahiri-drik`, `raman`, `krishnamurti`,
/// `reingold-dershowitz` or `fagan-bradley`.
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other text, the empty one and a full name
/// such as `Lahiri (Chitrapaksha)` included: the yoga moves with the
/// ayanāṃśa, so none is assumed.
pub fn ayanamsa(id: &str) -> Answer<Ayanamsa> {
    Ayanamsa::by_id(id).ok_or(Refusal::Unknown)
}

/// The two lines at a moment, read at the POSIX second `read_at`: the
/// caller's own instant, which a round trip through a day count could
/// move by a rounding.
fn lines_at(moment: Moment, read_at: i64, ayanamsa: Ayanamsa) -> String {
    let mut out = String::new();
    let yoga = yoga_at(moment, ayanamsa);
    let (began, ends) = yoga_span(moment, ayanamsa);
    let index = usize::from(yoga - 1);
    let mut line = Line::new(&mut out);
    line.cell("yoga")
        .value(yoga)
        .cell(YOGA_NAMES[index])
        .cell(YOGA_NAMES_DEVANAGARI[index])
        .value(unix_from_moment(began))
        .value(unix_from_moment(ends))
        .value(read_at)
        .cell(ayanamsa.id())
        .cell(ayanamsa.name());
    line.end();
    let half = karana_at(moment);
    let (began, ends) = karana_span(moment);
    // `karana_at` gives a half of the month, 1 to 60, which always has a
    // name.
    if let Some(name) = karana_name(half) {
        let name = usize::from(name);
        let mut line = Line::new(&mut out);
        line.cell("karana")
            .value(half)
            .cell(KARANA_NAMES[name])
            .cell(KARANA_NAMES_DEVANAGARI[name])
            .value(unix_from_moment(began))
            .value(unix_from_moment(ends))
            .value(read_at)
            .empties(2);
        line.end();
    }
    out
}

/// The two lines at a moment by the *Sūrya Siddhānta*'s Sun and Moon,
/// read at the POSIX second `read_at`: [`lines_at`]'s columns, the sky
/// named on both lines, since the Siddhānta's karaṇa is its own as its
/// yoga is.
fn siddhanta_lines_at(moment: Moment, read_at: i64) -> String {
    let mut out = String::new();
    let yoga = surya_siddhanta::yoga_at(moment);
    let (began, ends) = surya_siddhanta::yoga_span(moment);
    let index = usize::from(yoga - 1);
    let mut line = Line::new(&mut out);
    line.cell("yoga")
        .value(yoga)
        .cell(YOGA_NAMES[index])
        .cell(YOGA_NAMES_DEVANAGARI[index])
        .value(unix_from_moment(began))
        .value(unix_from_moment(ends))
        .value(read_at)
        .cell(SURYA_SIDDHANTA)
        .cell(SURYA_SIDDHANTA_NAME);
    line.end();
    let half = surya_siddhanta::karana_at(moment);
    let (began, ends) = surya_siddhanta::karana_span(moment);
    // The book names its four fixed karaṇas in its own order, Nāga before
    // Catuṣpada; `karana_at` gives a half, 1 to 60, which always has one.
    if let Some(name) = karana_name_surya_siddhanta(half) {
        let name = usize::from(name);
        let mut line = Line::new(&mut out);
        line.cell("karana")
            .value(half)
            .cell(KARANA_NAMES[name])
            .cell(KARANA_NAMES_DEVANAGARI[name])
            .value(unix_from_moment(began))
            .value(unix_from_moment(ends))
            .value(read_at)
            .cell(SURYA_SIDDHANTA)
            .cell(SURYA_SIDDHANTA_NAME);
        line.end();
    }
    out
}

/// The lines of `hc_panchanga_at`: the yoga and the karaṇa in progress at
/// a Universal Time instant, on the true sky in the zodiac of an ayanāṃśa
/// or on [`SURYA_SIDDHANTA`]'s.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a sky that is neither [`SURYA_SIDDHANTA`] nor
/// an ayanāṃśa [`ayanamsa`] names, and [`Refusal::OutOfRange`] for an
/// instant outside the sky layer's era on the true sky, or outside
/// [`SIDDHANTA_FIRST_DAY`] to [`SIDDHANTA_LAST_DAY`] on the Siddhānta's.
pub fn panchanga_at_lines(universal_unix: i64, ayanamsa_name: &str) -> Answer<String> {
    if matches(ayanamsa_name, SURYA_SIDDHANTA) {
        let moment = moment_on_day(universal_unix, siddhanta_day)?;
        return Ok(siddhanta_lines_at(moment, universal_unix));
    }
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    Ok(lines_at(
        moment_in_era(universal_unix)?,
        universal_unix,
        ayanamsa,
    ))
}

/// The lines of `hc_panchanga_of_day`: the yoga and the karaṇa a day
/// carries at a place, the ones in progress at its sunrise.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a sky [`panchanga_at_lines`] does not read,
/// [`Refusal::OutOfRange`] for a day outside the sky layer's era, and
/// [`Refusal::NoData`] for a day on which the Sun does not rise at the
/// place: a pañcāṅga reads its day at sunrise, and no other moment is put
/// in its place. On [`SURYA_SIDDHANTA`]'s sky the day is read at the
/// Siddhānta's own sunrise, which every day has within
/// [`MAX_SUNRISE_LATITUDE`]; a place beyond it, or a day outside
/// [`SIDDHANTA_FIRST_DAY`] to [`SIDDHANTA_LAST_DAY`], is
/// [`Refusal::OutOfRange`].
pub fn panchanga_of_day_lines(fixed: i64, place: Location, ayanamsa_name: &str) -> Answer<String> {
    if matches(ayanamsa_name, SURYA_SIDDHANTA) {
        let day = siddhanta_day(fixed)?;
        let place = sunrise_place(place)?;
        let rise = surya_siddhanta::sunrise(day, place);
        return Ok(siddhanta_lines_at(rise, unix_from_moment(rise)));
    }
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let day = day_in_era(fixed)?;
    let rise = sunrise(day, place).ok_or(Refusal::NoData)?;
    Ok(lines_at(rise, unix_from_moment(rise), ayanamsa))
}

/// Which sky a tithi is read on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sky {
    /// The true Sun and Moon of `hc-astro`.
    True,
    /// The *Sūrya Siddhānta*'s.
    Siddhanta,
}

/// The sky a name asks for: [`SURYA_SIDDHANTA`], or `true` or the
/// identifier of any ayanāṃśa [`ayanamsa`] reads for the true one, whose
/// tithi does not move with the ayanāṃśa, so that the same text a caller
/// gives `hc_panchanga_at` serves.
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other text.
fn sky(name: &str) -> Answer<Sky> {
    if matches(name, SURYA_SIDDHANTA) {
        Ok(Sky::Siddhanta)
    } else if matches(name, "true") || Ayanamsa::by_id(name).is_some() {
        Ok(Sky::True)
    } else {
        Err(Refusal::Unknown)
    }
}

/// How many columns each line of [`tithi_at_lines`] writes.
pub const TITHI_COLUMNS: usize = 8;

/// How many columns each line of [`tithis_of_day_lines`] writes.
pub const TITHIS_OF_DAY_COLUMNS: usize = TITHI_COLUMNS + 3;

/// The first eight cells of a tithi's line: its number, 1 to 30, the
/// fortnight (`shukla` or `krishna`), its day within the fortnight, 1 to 15,
/// its name in IAST ([`tithi_name`]), the moments it began and ends as whole
/// POSIX seconds of Universal Time, rounded down, the instant read, and the
/// sky, `true` or [`SURYA_SIDDHANTA`].
fn tithi_cells(line: &mut Line<'_>, number: u8, span: (Moment, Moment), read_at: i64, sky: Sky) {
    let (paksha, day) = paksha_of(number);
    line.value(number)
        .cell(match paksha {
            Paksha::Shukla => "shukla",
            Paksha::Krishna => "krishna",
        })
        .value(day)
        .cell(tithi_name(number).unwrap_or(""))
        .value(unix_from_moment(span.0))
        .value(unix_from_moment(span.1))
        .value(read_at)
        .cell(match sky {
            Sky::True => "true",
            Sky::Siddhanta => SURYA_SIDDHANTA,
        });
}

/// The tithi in progress at a moment on a sky, and its span.
fn tithi_now(moment: Moment, sky: Sky) -> (u8, (Moment, Moment)) {
    match sky {
        Sky::True => (tithi::tithi_number_at(moment), tithi::tithi_span(moment)),
        Sky::Siddhanta => (
            surya_siddhanta::tithi_at(moment),
            surya_siddhanta::tithi_span(moment),
        ),
    }
}

/// The line of `hc_tithi_at`: the tithi in progress at a Universal Time
/// instant, with the moments it began and ends — its number, 1 for śukla
/// pratipadā through 30 for amāvasyā, the fortnight, its day in it, its
/// name in IAST, the instants it began and ends and the one read, as whole
/// POSIX seconds, and the sky.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a sky that is not `true`, an ayanāṃśa or the Siddhānta, and
/// [`Refusal::OutOfRange`] for an instant outside the sky layer's era on
/// the true sky, or outside [`SIDDHANTA_FIRST_DAY`] to
/// [`SIDDHANTA_LAST_DAY`] on the Siddhānta's.
pub fn tithi_at_lines(universal_unix: i64, sky_name: &str) -> Answer<String> {
    let sky = sky(sky_name)?;
    let moment = match sky {
        Sky::True => moment_in_era(universal_unix)?,
        Sky::Siddhanta => moment_on_day(universal_unix, siddhanta_day)?,
    };
    let (number, span) = tithi_now(moment, sky);
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    tithi_cells(&mut line, number, span, universal_unix, sky);
    line.end();
    Ok(out)
}

/// The lines of `hc_tithis_of_day`: every tithi in progress between a
/// day's sunrise at a place and the next, one a line in order, each the
/// columns of [`tithi_at_lines`], with the instant read the day's sunrise,
/// and three flags. `1` for the first when the tithi holds the day's
/// sunrise, the one the day carries; `1` for the second when it holds the
/// next sunrise as well, a tithi that is repeated (*adhika*, *vṛddhi*) and
/// is the day's again tomorrow; `1` for the third when it holds neither, a
/// tithi that begins after one sunrise and ends before the next and is
/// skipped (*kṣaya*), which no civil day carries.
///
/// # Errors
///
/// As [`panchanga_of_day_lines`].
pub fn tithis_of_day_lines(fixed: i64, place: Location, sky_name: &str) -> Answer<String> {
    let sky = sky(sky_name)?;
    let (rise, next_rise) = match sky {
        Sky::True => {
            let day = day_in_era(fixed)?;
            (
                sunrise(day, place).ok_or(Refusal::NoData)?,
                sunrise(Rd(day.0 + 1), place).ok_or(Refusal::NoData)?,
            )
        }
        Sky::Siddhanta => {
            let day = siddhanta_day(fixed)?;
            let place = sunrise_place(place)?;
            (
                surya_siddhanta::sunrise(day, place),
                surya_siddhanta::sunrise(siddhanta_day(fixed + 1)?, place),
            )
        }
    };
    let mut out = String::new();
    let mut at = rise;
    // A tithi runs 0.8 to 1.2 days and a day from sunrise to sunrise is
    // about one, so two or three are in progress.
    for _ in 0..4 {
        let (number, span) = tithi_now(at, sky);
        let holds = |moment: Moment| span.0.0 <= moment.0 && moment.0 < span.1.0;
        let (first, second) = (holds(rise), holds(next_rise));
        let mut line = Line::new(&mut out);
        tithi_cells(&mut line, number, span, unix_from_moment(rise), sky);
        line.flag(first)
            .flag(first && second)
            .flag(!first && !second);
        line.end();
        if span.1.0 >= next_rise.0 {
            break;
        }
        at = Moment(span.1.0 + 1e-4);
    }
    Ok(out)
}

/// How many columns each line of [`ayanamsas_lines`] writes.
pub const AYANAMSA_COLUMNS: usize = 6;

/// The lines of `hc_ayanamsas`: every named ayanāṃśa, [`Ayanamsa::ALL`],
/// one a line in the table's order — the identifier [`ayanamsa`] reads back,
/// the full name, the Julian date its anchor is quoted for, the anchor in
/// degrees, where the anchor is from, and what the anchor stands for: `mean`
/// or `true` (the mean value plus the nutation in longitude of the day, as
/// [`hc_seasons::zodiac::AyanamsaKind`] has it). The value at another moment
/// is the anchor carried by the IAU 2006 general precession, and for a `true`
/// one by the nutation of each day besides.
#[must_use]
pub fn ayanamsas_lines() -> String {
    let mut out = String::new();
    for ayanamsa in Ayanamsa::ALL {
        let mut line = Line::new(&mut out);
        line.cell(ayanamsa.id())
            .cell(ayanamsa.name())
            .value(ayanamsa.anchor_julian_date())
            .value(ayanamsa.degrees_at_anchor())
            .cell(ayanamsa.source())
            .cell(kind_cell(*ayanamsa));
        line.end();
    }
    out
}

/// The cell naming what an ayanāṃśa's anchor stands for.
fn kind_cell(ayanamsa: Ayanamsa) -> &'static str {
    match ayanamsa.kind() {
        AyanamsaKind::Mean => "mean",
        AyanamsaKind::True => "true",
    }
}

/// The line of one ayanāṃśa's value: the degrees, the identifier and name
/// (the empty identifier for an anchor a caller gave), the anchor's Julian
/// date and degrees, and the instant read.
fn ayanamsa_value_line(ayanamsa: Ayanamsa, moment: Moment, read_at: i64) -> String {
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(ayanamsa.degrees_at(moment))
        .cell(ayanamsa.id())
        .cell(ayanamsa.name())
        .value(ayanamsa.anchor_julian_date())
        .value(ayanamsa.degrees_at_anchor())
        .value(read_at)
        .cell(kind_cell(ayanamsa));
    line.end();
    out
}

/// How many columns each line of [`ayanamsa_at_line`] writes.
pub const AYANAMSA_VALUE_COLUMNS: usize = 7;

/// The line of `hc_ayanamsa_at`: a named ayanāṃśa's value in degrees at a
/// Universal Time instant, then its identifier, its name, its anchor's Julian
/// date and degrees, the instant read, and `mean` or `true` for what the
/// anchor stands for (see [`ayanamsas_lines`]).
///
/// # Errors
///
/// [`Refusal::Unknown`] for an ayanāṃśa [`ayanamsa`] does not name and
/// [`Refusal::OutOfRange`] for an instant outside the sky layer's era.
pub fn ayanamsa_at_line(universal_unix: i64, ayanamsa_name: &str) -> Answer<String> {
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    Ok(ayanamsa_value_line(
        ayanamsa,
        moment_in_era(universal_unix)?,
        universal_unix,
    ))
}

/// The line of `hc_ayanamsa_from_anchor`: the value in degrees at a
/// Universal Time instant of an ayanāṃśa a caller anchors, `degrees` at the
/// Julian date `anchor_julian_date`, carried by the IAU 2006 general
/// precession as the named ones are; the columns of [`ayanamsa_at_line`],
/// with `custom` as the identifier and name and `mean` as the kind.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for an instant outside the sky layer's era, an
/// anchor that is not finite, a Julian date outside the sky layer's era
/// (from −1000 to 3000) or degrees outside −360° to 360°.
pub fn ayanamsa_from_anchor_line(
    universal_unix: i64,
    anchor_julian_date: f64,
    degrees_at_anchor: f64,
) -> Answer<String> {
    crate::astro_lines::julian_date_in_era(anchor_julian_date)?;
    if !(-360.0..=360.0).contains(&degrees_at_anchor) {
        return Err(Refusal::OutOfRange);
    }
    let ayanamsa = Ayanamsa::new("custom", "custom", anchor_julian_date, degrees_at_anchor);
    Ok(ayanamsa_value_line(
        ayanamsa,
        moment_in_era(universal_unix)?,
        universal_unix,
    ))
}

/// How many columns each line of [`nakshatra_at_lines`] writes./// How many columns each line of [`nakshatra_at_lines`] writes.
pub const NAKSHATRA_COLUMNS: usize = 8;

/// The line of the nakṣatra in progress at a moment, read at `read_at`:
/// its number, identifier and name, when the Moon entered it and when it
/// leaves, the instant read, and the ayanāṃśa by identifier and full name.
fn nakshatra_line(moment: Moment, read_at: i64, ayanamsa: Ayanamsa) -> String {
    let number = nakshatra_at(moment, ayanamsa);
    let (entered, leaves) = nakshatra_span(number, moment, ayanamsa);
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(number);
    nakshatra_cells(&mut line, number);
    line.value(unix_from_moment(entered))
        .value(unix_from_moment(leaves))
        .value(read_at)
        .cell(ayanamsa.id())
        .cell(ayanamsa.name());
    line.end();
    out
}

/// A nakṣatra's identifier and name, [`nakshatra_id`] and
/// [`nakshatra_name`], the two cells every line that names one writes
/// after its number: `pushya` and `Puṣya`.
pub(crate) fn nakshatra_cells(line: &mut Line<'_>, number: u8) {
    line.cell(nakshatra_id(number).unwrap_or(""))
        .cell(nakshatra_name(number).unwrap_or(""));
}

/// The line of `hc_nakshatra_at`: the nakṣatra the Moon is in at a
/// Universal Time instant, 1 for Aśvinī through 27 for Revatī, in the
/// zodiac of an ayanāṃśa — its number, its identifier and name (`ashvini`,
/// `Aśvinī`), the instants the Moon entered it
/// and leaves it and the instant read, as whole POSIX seconds rounded
/// down, and the ayanāṃśa by identifier and full name.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an ayanāṃśa [`ayanamsa`] does not name, and
/// [`Refusal::OutOfRange`] for an instant outside the sky layer's era.
pub fn nakshatra_at_lines(universal_unix: i64, ayanamsa_name: &str) -> Answer<String> {
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    Ok(nakshatra_line(
        moment_in_era(universal_unix)?,
        universal_unix,
        ayanamsa,
    ))
}

/// The line of `hc_nakshatra_of_day`: [`nakshatra_at_lines`]' line read
/// at the day's sunrise at the place, the nakṣatra a pañcāṅga prints for
/// the day.
///
/// # Errors
///
/// As [`panchanga_of_day_lines`] on the true sky.
pub fn nakshatra_of_day_lines(fixed: i64, place: Location, ayanamsa_name: &str) -> Answer<String> {
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let day = day_in_era(fixed)?;
    let rise = sunrise(day, place).ok_or(Refusal::NoData)?;
    Ok(nakshatra_line(rise, unix_from_moment(rise), ayanamsa))
}

/// How many columns [`amrita_siddhi_line`] writes.
pub const AMRITA_SIDDHI_COLUMNS: usize = 9;

/// The line of `hc_amrita_siddhi`: the *amṛta siddhi yoga* of a day at a
/// place, [`amrita_siddhi::amrita_siddhi`] — its name as Drik Panchang
/// prints it in English and in Devanagari, the nakṣatra the weekday pairs
/// with, 1 to 27, its identifier and name, the start and the end of the part of the day, sunrise to
/// the next sunrise, that the Moon spends in it, as whole POSIX seconds
/// rounded down, both empty on a day it spends none, `1` if the yoga falls
/// on the day and `0` if not, and the ayanāṃśa's identifier.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an ayanāṃśa [`ayanamsa`] does not name,
/// [`Refusal::OutOfRange`] for a day outside the sky layer's era, and
/// [`Refusal::NoData`] for a day on which, or on whose morrow, the Sun
/// does not rise at the place, which leaves the day without bounds.
pub fn amrita_siddhi_line(fixed: i64, place: Location, ayanamsa_name: &str) -> Answer<String> {
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let day = day_in_era(fixed)?;
    let span = amrita_siddhi::amrita_siddhi(day, place, ayanamsa).map_err(|_| Refusal::NoData)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    let nakshatra = amrita_siddhi::nakshatra_of(hc_calendar::Weekday::from_rd(day));
    line.cell(amrita_siddhi::NAME)
        .cell(amrita_siddhi::NAME_DEVANAGARI)
        .value(nakshatra);
    nakshatra_cells(&mut line, nakshatra);
    line.value_or_empty(span.map(|span| unix_from_moment(span.start)))
        .value_or_empty(span.map(|span| unix_from_moment(span.end)))
        .flag(span.is_some())
        .cell(ayanamsa.id());
    line.end();
    Ok(out)
}

/// How many columns each line of [`muhurtas_lines`] writes.
pub const MUHURTA_COLUMNS: usize = 6 + MISSING_COLUMNS;

/// The lines of `hc_muhurtas`: the thirty muhūrtas of a day at a place,
/// the fifteen of the daylight and the fifteen of the night after it, in
/// order, [`muhurta::muhurta`] — the half (`day` or `night`), the
/// muhūrta's number in it, 1 to 15, its name in
/// [`muhurta::WIKIPEDIA_NAMES`], its start and its end as whole POSIX
/// seconds rounded down, what the pañcāṅga prints it as (`abhijit` for the
/// eighth of the daylight on a day but a Wednesday, `dur-muhurtam` for the
/// weekday's ones in [`muhurta::DUR_MUHURTAM`], else empty), and the four
/// cells of a missing solar event, as `hc_kalam` writes them, where the
/// Sun does not rise or set and the start and end are empty.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a day outside the sky layer's era.
pub fn muhurtas_lines(fixed: i64, place: Location) -> Answer<String> {
    use muhurta::{ABHIJIT, DUR_MUHURTAM, Half, MUHURTAS_PER_HALF, Muhurta};
    let day = day_in_era(fixed)?;
    let weekday = hc_calendar::Weekday::from_rd(day);
    let dur = DUR_MUHURTAM[weekday.sunday_first_number() as usize];
    let mut out = String::new();
    for (half, name) in [(Half::Day, "day"), (Half::Night, "night")] {
        for number in 1..=MUHURTAS_PER_HALF {
            // Every number of 1 to 15 is a muhūrta of either half.
            let Some(which) = (match half {
                Half::Day => Muhurta::day(number),
                Half::Night => Muhurta::night(number),
            }) else {
                continue;
            };
            let mark = if half == Half::Day
                && number == ABHIJIT
                && weekday != hc_calendar::Weekday::Wednesday
            {
                "abhijit"
            } else if dur.contains(&Some(which)) {
                "dur-muhurtam"
            } else {
                ""
            };
            let mut line = Line::new(&mut out);
            line.cell(name).value(number).cell(which.wikipedia_name());
            match muhurta::muhurta(which, day, place) {
                Ok(span) => {
                    line.value(unix_from_moment(span.start))
                        .value(unix_from_moment(span.end))
                        .cell(mark);
                    missing_cells(&mut line, None);
                }
                Err(missing) => {
                    line.empties(2).cell(mark);
                    missing_cells(&mut line, Some(missing));
                }
            }
            line.end();
        }
    }
    Ok(out)
}

/// How many columns each line of [`kalam_lines`] writes.
#[cfg(feature = "i18n")]
pub const KALAM_COLUMNS: usize = 8 + MISSING_COLUMNS;

/// The lines of `hc_kalam`: Rāhu kālam, Yamaganda and Gulika kālam on a
/// day by a convention of [`KalamConvention::ALL`], selected by its
/// identifier in any case, one line each in the order a pañcāṅga prints
/// them, as the period's identifier, its English name as Drik Panchang
/// prints it, its name in a locale and the tag of the data that named it,
/// by [`hc_i18n::reckonings::name_or_fallback`] — राहुकाल under `hi` —
/// the eighth of the day it takes (1 to 8), the clock its start and end
/// are read on, and the start and the end, then the four cells of a
/// missing solar event.
///
/// On `rahu-kalam-sunrise` the clock is `universal` and the start and end
/// are whole POSIX seconds, rounded down, of the eighth of the daylight at
/// the place; where the Sun does not rise or set they are empty and the
/// last cells name the missing event. On `rahu-kalam-fixed` the clock is
/// `local` and they are seconds after midnight of the day's own clock, in
/// whatever zone the caller keeps: Rāhu kālam on a Monday is 27 000 to
/// 32 400, 07:30 to 09:00, wherever it is read.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a convention not named, and
/// [`Refusal::OutOfRange`] for a day outside the sky layer's era.
#[cfg(feature = "i18n")]
pub fn kalam_lines(convention: &str, fixed: i64, place: Location, locale: &str) -> Answer<String> {
    let convention = KalamConvention::by_id(convention).ok_or(Refusal::Unknown)?;
    let day = day_in_era(fixed)?;
    let (clock, seconds): (&str, &dyn Fn(Moment) -> i64) = match convention.clock {
        SpanClock::Universal => ("universal", &unix_from_moment),
        SpanClock::Local => ("local", &|moment: Moment| {
            hc_core::math::round((moment.0 - day.0 as f64) * 86_400.0) as i64
        }),
    };
    let mut out = String::new();
    for period in Kalam::ALL {
        let mut line = Line::new(&mut out);
        line.cell(period.id).cell(period.english_name);
        reckoning_name(&mut line, locale, hc_i18n::reckonings::KALAM, period.id);
        line.value(period.part(hc_calendar::Weekday::from_rd(day)))
            .cell(clock);
        match (convention.span)(period, day, place) {
            Ok(span) => {
                line.value(seconds(span.start)).value(seconds(span.end));
                missing_cells(&mut line, None);
            }
            Err(missing) => {
                line.empties(2);
                missing_cells(&mut line, Some(missing));
            }
        }
        line.end();
    }
    Ok(out)
}

/// How many columns each line of [`festival_readings_lines`] writes.
pub const FESTIVAL_READING_COLUMNS: usize = 4;

/// The lines of `hc_festival_readings`: the readings of a festival's day
/// where the sects part, [`vaishnava::FestivalReading::ALL`], one a line in
/// the table's order — the identifier `hc_janmashtami` reads back, the
/// English name, how the day is taken in words, and where the reading is
/// from.
#[must_use]
pub fn festival_readings_lines() -> String {
    let mut out = String::new();
    for reading in vaishnava::FestivalReading::ALL {
        let mut line = Line::new(&mut out);
        line.cell(reading.id)
            .cell(reading.english_name)
            .cell(reading.rule)
            .cell(reading.source);
        line.end();
    }
    out
}

/// The reading an identifier names: `smarta` or `vaishnava`.
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other text.
fn festival_reading(id: &str) -> Answer<vaishnava::FestivalReading> {
    vaishnava::FestivalReading::by_id(id).ok_or(Refusal::Unknown)
}

/// The refusal of a festival day's calendar error: a year or month the
/// calendar does not cover is out of its range, and a tithi that does not
/// exist is an invalid date.
fn festival_refusal(error: hc_calendar::CalendarError) -> Refusal {
    use hc_calendar::CalendarError;
    match error {
        CalendarError::YearOutOfRange | CalendarError::MonthOutOfRange => Refusal::OutOfRange,
        other => Refusal::from(other),
    }
}

/// How many columns each line of [`janmashtami_line`] writes.
pub const JANMASHTAMI_COLUMNS: usize = 8;

/// How many columns each line of [`vaishnava_day_line`] writes.
pub const VAISHNAVA_DAY_COLUMNS: usize = 9;

/// The cells of a festival's day: the fixed day, its Gregorian date, the
/// tithi the day carries at sunrise there, and the ayanāṃśa.
fn festival_day_cells(line: &mut Line<'_>, day: Rd, calendar: &HinduLunarCalendar, id: &str) {
    let (year, month, date) = hc_calendars_solar::gregorian::from_fixed(day).unwrap_or((0, 0, 0));
    let sunrise_tithi = calendar.from_fixed(day).map_or(0, |date| date.day);
    line.value(day.0)
        .value(year)
        .value(month)
        .value(date)
        .value(sunrise_tithi)
        .cell(id);
}

/// The line of `hc_janmashtami`: the day of Kṛṣṇa Janmāṣṭamī in a Gregorian
/// year, at a place, by a reading — the reading's identifier, the year, the
/// fixed day, its Gregorian year, month and day, the tithi the day carries
/// at sunrise there (the eighth, 23 or 24 for a Vaiṣṇava day that is a
/// Navamī), and the ayanāṃśa. Śrāvaṇa falls in August or September, in the
/// Śaka year the Gregorian one less 78.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a reading or an ayanāṃśa not named;
/// [`Refusal::OutOfRange`] for a place beyond the latitude where the Sun
/// rises every day or a year outside the calendar's, 1700 to 2299; a day
/// on which the Sun does not rise there is [`Refusal::NoData`].
pub fn janmashtami_line(
    year: i64,
    reading: &str,
    place: Location,
    ayanamsa_name: &str,
) -> Answer<String> {
    let reading = festival_reading(reading)?;
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let place = sunrise_place(place)?;
    let calendar = HinduLunarCalendar::new(place, ayanamsa);
    let day = vaishnava::janmashtami_by(reading, &calendar, year).map_err(festival_refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(reading.id).value(year);
    festival_day_cells(&mut line, day, &calendar, ayanamsa_name);
    line.end();
    Ok(out)
}

/// The line of `hc_vaishnava_day`: the Vaiṣṇava day of a tithi of an amānta
/// month of a Śaka year at a place — the first day whose sunrise carries the
/// tithi or a later one — as `vaishnava` and the Śaka year, then the cells
/// of [`janmashtami_line`]'s, the month and the tithi in the place of the
/// reading and the year.
///
/// The month is 1 for Chaitra through 12 for Phālguna and the tithi 1
/// through 30, counted through the month from śukla pratipadā.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an ayanāṃśa not named; [`Refusal::OutOfRange`]
/// for a place beyond the latitude where the Sun rises every day, a Śaka
/// year outside 1622 to 2221 or a month outside 1 to 12;
/// [`Refusal::InvalidDate`] for a tithi outside 1 to 30.
pub fn vaishnava_day_line(
    saka_year: i64,
    month: u8,
    tithi: u8,
    place: Location,
    ayanamsa_name: &str,
) -> Answer<String> {
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let place = sunrise_place(place)?;
    if !(1..=12).contains(&month) {
        return Err(Refusal::OutOfRange);
    }
    let calendar = HinduLunarCalendar::new(place, ayanamsa);
    let day = vaishnava::day_of(&calendar, saka_year, month, tithi).map_err(festival_refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(saka_year).value(month).value(tithi);
    festival_day_cells(&mut line, day, &calendar, ayanamsa_name);
    line.end();
    Ok(out)
}

/// How many columns each line of [`vishti_free_span_line`] writes.
pub const VISHTI_FREE_SPAN_COLUMNS: usize = 7;

/// The line of `hc_vishti_free_span`: the part of a tithi that Bhadra, the
/// karaṇa Viṣṭi, does not cover, for the tithi of an amānta month of a Śaka
/// year at a place — the Śaka year, the month, the tithi, the first day of
/// the month (the fixed day), the moments the span begins and ends as whole
/// POSIX seconds of Universal Time, rounded down, and the ayanāṃśa. It is
/// the second half of the full moon's tithi, the part Rakṣā Bandhana waits
/// for, and of the fourth tithi of each fortnight the first half.
///
/// A tithi on which Viṣṭi never falls is an answer with both moments empty.
///
/// # Errors
///
/// As [`vaishnava_day_line`].
pub fn vishti_free_span_line(
    saka_year: i64,
    month: u8,
    tithi: u8,
    place: Location,
    ayanamsa_name: &str,
) -> Answer<String> {
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let place = sunrise_place(place)?;
    if !(1..=12).contains(&month) {
        return Err(Refusal::OutOfRange);
    }
    if !(1..=30).contains(&tithi) {
        return Err(Refusal::InvalidDate);
    }
    let calendar = HinduLunarCalendar::new(place, ayanamsa);
    let (first, _) = calendar
        .month_span(saka_year, month, false)
        .map_err(festival_refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(saka_year)
        .value(month)
        .value(tithi)
        .value(first.0);
    match panchanga::vishti_free_span(tithi, first) {
        Some((from, to)) => {
            line.value(unix_from_moment(from))
                .value(unix_from_moment(to));
        }
        None => {
            line.empties(2);
        }
    }
    line.cell(ayanamsa_name);
    line.end();
    Ok(out)
}

/// The fixed day on which year `year` of a historical Indian era over the
/// lunisolar months begins, for `hc_era_new_year`: one of
/// [`hc_calendars_indic::lunar_era::ALL`] by its identifier (`vikram-samvat-kartikadi`,
/// `rajyabhisheka-saka`, `saptarshi`, `gupta`, `valabhi`, `kalachuri` or
/// `lakshmana-sena`).
///
/// An era's year opens at the first day of a month or in the middle of one
/// (Kārttika śukla 1 for the Kārttikādi years, Āśvina śukla 1 for the
/// Chedi, Jyeṣṭha śukla 13 for Śivājī's), so the day is the era's own and
/// not the first of a month.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an identifier that is not one of these
/// eras; [`Refusal::OutOfRange`] for a year outside the months the era is
/// read over.
pub fn era_new_year(calendar: &str, year: i64) -> Answer<i64> {
    let era = hc_calendars_indic::lunar_era::ALL
        .iter()
        .find(|era| era.id.0 == calendar)
        .ok_or(Refusal::Unknown)?;
    era.new_year(year)
        .map(|day| day.0)
        .map_err(festival_refusal)
}

/// How many columns each line of [`rahu_at_line`] writes.
pub const RAHU_AT_COLUMNS: usize = 11;

/// The longest span [`rahu_ingresses_lines`] accepts, in seconds: a hundred
/// Julian years.
pub const MAX_NODE_SPAN_SECONDS: i64 = 100 * 31_557_600;

/// The cells of a node's place: its sidereal longitude in degrees, the
/// sign by number from 1 for Meṣa, identifier and Sanskrit name.
fn node_place_cells(line: &mut Line<'_>, longitude: f64) {
    let index = floor(longitude / 30.0) as u8 % 12;
    let sign = SiderealSign::from_index(index).unwrap_or(SiderealSign::MESHA);
    line.value(longitude)
        .value(index + 1)
        .cell(sign.id())
        .cell(sign.sanskrit_name());
}

/// The line of `hc_rahu_at`: Rāhu and Ketu at a Universal Time instant —
/// `mean`, the kind of node (the true node is not carried), Rāhu's
/// sidereal longitude in degrees and the sign it is in as a number from 1,
/// identifier and Sanskrit name, then Ketu's longitude and sign the same way
/// (opposite, 180° on), the ayanāṃśa and the instant read.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an ayanāṃśa not named; [`Refusal::OutOfRange`]
/// for an instant outside the sky layer's era.
pub fn rahu_at_line(universal_unix: i64, ayanamsa_name: &str) -> Answer<String> {
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let moment = moment_in_era(universal_unix)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell("mean");
    node_place_cells(&mut line, node::rahu_longitude(moment, ayanamsa));
    node_place_cells(&mut line, node::ketu_longitude(moment, ayanamsa));
    line.cell(ayanamsa_name).value(universal_unix);
    line.end();
    Ok(out)
}

/// How many columns each line of [`rahu_ingresses_lines`] writes.
pub const RAHU_INGRESS_COLUMNS: usize = 7;

/// The lines of `hc_rahu_ingresses`: the entries of the mean node into the
/// sidereal signs in the half-open span `[from, to)` of POSIX seconds, in
/// time order — the moment as whole POSIX seconds of Universal Time,
/// rounded down, the sign Rāhu leaves and the one it enters, each by
/// identifier and Sanskrit name, and the identifier of the sign Ketu
/// enters (the one opposite Rāhu's). Rāhu moves backward, a sign in about 566
/// days.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an ayanāṃśa not named; [`Refusal::OutOfRange`]
/// for a span with an end outside the sky layer's era or longer than
/// [`MAX_NODE_SPAN_SECONDS`].
pub fn rahu_ingresses_lines(from: i64, to: i64, ayanamsa_name: &str) -> Answer<String> {
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let start = moment_in_era(from)?;
    if to <= from {
        return Ok(String::new());
    }
    let end = moment_in_era(to - 1)?;
    if to - from > MAX_NODE_SPAN_SECONDS {
        return Err(Refusal::OutOfRange);
    }
    let mut out = String::new();
    for ingress in node::ingresses(start, Moment(end.0 + 1.0 / 86_400.0), ayanamsa) {
        let unix = unix_from_moment(ingress.moment);
        if unix >= to {
            break;
        }
        let ketu = SiderealSign::from_index((ingress.rahu_into.index() + 6) % 12)
            .unwrap_or(SiderealSign::MESHA);
        let mut line = Line::new(&mut out);
        line.value(unix)
            .cell(ingress.rahu_from.id())
            .cell(ingress.rahu_from.sanskrit_name())
            .cell(ingress.rahu_into.id())
            .cell(ingress.rahu_into.sanskrit_name())
            .cell(ketu.id())
            .cell(ketu.sanskrit_name());
        line.end();
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows_of(text: &str) -> alloc::vec::Vec<alloc::vec::Vec<String>> {
        text.lines()
            .map(|line| line.split('\t').map(String::from).collect())
            .collect()
    }

    /// Drik Panchang's day page for Tokyo (35°41′22″N 139°41′30″E,
    /// `drik-day-panchang-tokyo-2025`, read 2026-10-03) of 13 January 2025
    /// prints "Chaturdashi upto 08:33 AM" with sunrise at 06:51 JST: the
    /// bright fortnight's fourteenth, ending at 23:33 UT on the 12th, the
    /// same moment as the New Delhi page's 05:03 IST. Purnima follows.
    #[test]
    fn a_tithi_ends_when_drik_panchang_says_and_a_day_lists_the_tithis_it_holds() {
        let tokyo = Location::new(
            35.0 + 41.0 / 60.0 + 22.0 / 3_600.0,
            139.0 + 41.0 / 60.0 + 30.0 / 3_600.0,
            0.0,
        );
        let day = hc_calendars_solar::gregorian::to_fixed(2025, 1, 13)
            .expect("a date")
            .0;
        let unix = |fixed: i64, hours: f64| {
            (fixed - hc_calendar::fixed::RD_OF_UNIX_EPOCH) * 86_400 + (hours * 3_600.0) as i64
        };
        let ends_at = unix(day - 1, 23.0 + 33.0 / 60.0);
        let noon = unix(day - 1, 12.0);
        let rows = rows_of(&tithi_at_lines(noon, "lahiri").expect("in the era"));
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].len(), TITHI_COLUMNS);
        assert_eq!(rows[0][..4], ["14", "shukla", "14", "Caturdaśī"]);
        assert_eq!(rows[0][7], "true");
        let ends: i64 = rows[0][5].parse().expect("an instant");
        assert!((ends - ends_at).abs() <= 120, "{} s", ends - ends_at);
        let began: i64 = rows[0][4].parse().expect("an instant");
        assert!((ends - began) > 20 * 3_600 && (ends - began) < 28 * 3_600);
        assert_eq!(rows[0][6], noon.to_string());
        // The day's tithis: Caturdaśī at sunrise and Pūrṇimā, which holds
        // the next sunrise too.
        let rows = rows_of(&tithis_of_day_lines(day, tokyo, "true").expect("in the era"));
        assert!(rows.iter().all(|row| row.len() == TITHIS_OF_DAY_COLUMNS));
        assert_eq!(rows[0][..4], ["14", "shukla", "14", "Caturdaśī"]);
        assert_eq!(rows[0][8..], ["1", "0", "0"]);
        assert_eq!(rows[1][..4], ["15", "shukla", "15", "Pūrṇimā"]);
        assert_eq!(rows[1][8..], ["0", "0", "0"]);
        let sunrise_read: i64 = rows[0][6].parse().expect("an instant");
        assert!((sunrise_read - unix(day - 1, 21.0 + 51.0 / 60.0)).abs() <= 120);
        assert_eq!(tithi_at_lines(noon, "mars"), Err(Refusal::Unknown));
        assert_eq!(
            tithi_at_lines(
                unix(
                    hc_calendars_solar::gregorian::to_fixed(3001, 1, 1)
                        .expect("a date")
                        .0,
                    0.0
                ),
                "true"
            ),
            Err(Refusal::OutOfRange)
        );
    }

    /// Over a year at Delhi the day's tithis agree with `tithi_of_day`,
    /// which the Drik Panchang comparisons hold: the flagged tithi is the
    /// day's, a tithi flagged repeated is the next day's too, and each skip
    /// of two between the numbers of consecutive days is one row flagged
    /// skipped.
    #[test]
    fn a_days_flags_agree_with_the_tithi_the_day_carries() {
        let delhi = Location::new(28.6139, 77.2090, 0.0);
        let first = hc_calendars_solar::gregorian::to_fixed(2025, 1, 1)
            .expect("a date")
            .0;
        let (mut skipped_rows, mut skipped_days, mut repeated_rows, mut repeated_days) =
            (0, 0, 0, 0);
        for fixed in first..first + 365 {
            let rows = rows_of(&tithis_of_day_lines(fixed, delhi, "lahiri").expect("in the era"));
            let carried = hc_calendars_indic::tithi::tithi_of_day(Rd(fixed), delhi);
            let next = hc_calendars_indic::tithi::tithi_of_day(Rd(fixed + 1), delhi);
            let at_sunrise: alloc::vec::Vec<_> = rows.iter().filter(|row| row[8] == "1").collect();
            assert_eq!(at_sunrise.len(), 1, "{fixed}");
            assert_eq!(at_sunrise[0][0], carried.to_string(), "{fixed}");
            // A day's own tithi is the first row, in order.
            assert_eq!(rows[0][0], carried.to_string());
            repeated_rows += rows.iter().filter(|row| row[9] == "1").count();
            skipped_rows += rows.iter().filter(|row| row[10] == "1").count();
            if next == carried {
                repeated_days += 1;
                assert_eq!(at_sunrise[0][9], "1", "{fixed}");
            }
            if (next + 30 - carried) % 30 == 2 {
                skipped_days += 1;
                let skipped: alloc::vec::Vec<_> =
                    rows.iter().filter(|row| row[10] == "1").collect();
                assert_eq!(skipped.len(), 1, "{fixed}");
                assert_eq!(skipped[0][0], ((carried % 30) + 1).to_string(), "{fixed}");
            }
        }
        assert_eq!((repeated_rows, skipped_rows), (repeated_days, skipped_days));
        assert!(
            skipped_rows >= 1 && repeated_rows >= 1,
            "{skipped_rows} {repeated_rows}"
        );
    }

    /// Drik Panchang's page prints its Lahiri ayanāṃśa as 24.213067 on 1
    /// January 2025 (`drik-day-panchang-ayanamsha`, as `Ayanamsa::LAHIRI_DRIK`
    /// is anchored), and the Calendar Reform Committee fixed Lahiri's at
    /// 23°15′00″ on 21 March 1956 (`crc1955`, p. 8).
    #[test]
    fn the_ayanamsas_are_listed_and_valued_at_an_instant() {
        let table = rows_of(&ayanamsas_lines());
        assert_eq!(table.len(), Ayanamsa::ALL.len());
        assert!(table.iter().all(|row| row.len() == AYANAMSA_COLUMNS));
        let ids: alloc::vec::Vec<&str> = table.iter().map(|row| row[0].as_str()).collect();
        assert!(ids.contains(&"lahiri-drik") && ids.contains(&"lahiri-crc-1955"));
        let crc = table
            .iter()
            .find(|row| row[0] == "lahiri-crc-1955")
            .expect("a row");
        assert_eq!((crc[2].as_str(), crc[3].as_str()), ("2435553.5", "23.25"));
        // The Committee's and the Rashtriya Panchang's printed values are
        // true ones, the Swiss Ephemeris's and Drik Panchang's mean.
        let kind = |id: &str| {
            table
                .iter()
                .find(|row| row[0] == id)
                .map(|row| row[5].clone())
        };
        assert_eq!(kind("lahiri-crc-1955").as_deref(), Some("true"));
        assert_eq!(kind("lahiri-rashtriya").as_deref(), Some("true"));
        assert_eq!(kind("lahiri").as_deref(), Some("mean"));
        assert_eq!(kind("lahiri-drik").as_deref(), Some("mean"));
        let day = |year| {
            hc_calendars_solar::gregorian::to_fixed(year, 1, 1)
                .expect("a date")
                .0
        };
        let unix = (day(2025) - hc_calendar::fixed::RD_OF_UNIX_EPOCH) * 86_400;
        let row = rows_of(&ayanamsa_at_line(unix, "lahiri-drik").expect("in the era"));
        assert_eq!(row[0].len(), AYANAMSA_VALUE_COLUMNS);
        let degrees: f64 = row[0][0].parse().expect("degrees");
        assert!((degrees - 24.213_067).abs() < 3e-4, "{degrees}");
        assert_eq!(row[0][1..3], ["lahiri-drik", "Lahiri (Drik Panchang)"]);
        assert_eq!(row[0][6], "mean");
        // An anchor a caller gives is read back at its own Julian date.
        let march_1956 = (hc_calendars_solar::gregorian::to_fixed(1956, 3, 21)
            .expect("a date")
            .0
            - hc_calendar::fixed::RD_OF_UNIX_EPOCH)
            * 86_400;
        let custom = rows_of(
            &ayanamsa_from_anchor_line(march_1956, 2_435_553.5, 23.25).expect("in the era"),
        );
        let degrees: f64 = custom[0][0].parse().expect("degrees");
        assert!((degrees - 23.25).abs() < 1e-6, "{degrees}");
        assert_eq!(custom[0][1..3], ["custom", "custom"]);
        assert_eq!(custom[0][6], "mean");
        // The Committee's own reading is its printed 23°15′0″ on that day
        // and a true value; the named Lahiri sits 17.3 arcseconds below it.
        let committee =
            rows_of(&ayanamsa_at_line(march_1956, "lahiri-crc-1955").expect("in the era"));
        let degrees: f64 = committee[0][0].parse().expect("degrees");
        assert!((degrees - 23.25).abs() < 1e-6, "{degrees}");
        assert_eq!(committee[0][6], "true");
        let lahiri: f64 = rows_of(&ayanamsa_at_line(march_1956, "lahiri").expect("in the era"))[0]
            [0]
        .parse()
        .expect("degrees");
        assert!(((23.25 - lahiri) * 3_600.0 - 17.3).abs() < 0.5, "{lahiri}");
        assert_eq!(ayanamsa_at_line(unix, "mars"), Err(Refusal::Unknown));
        assert_eq!(
            ayanamsa_from_anchor_line(unix, f64::NAN, 1.0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            ayanamsa_from_anchor_line(unix, 2_435_553.5, 400.0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            ayanamsa_from_anchor_line(unix, 1.0e9, 1.0),
            Err(Refusal::OutOfRange)
        );
    }

    /// Drik Panchang's New Delhi page of 1 January 2025, a Wednesday
    /// (`drik-day-panchang-2025`, as `hc-calendars-indic`'s test reads it):
    /// Rāhu kālam from 12:25, Yamaganda from 08:32 and Gulikai kālam from
    /// 11:07 IST, each within a minute; on the fixed day Wednesday's Rāhu
    /// kālam is the fifth eighth, 12:00 to 13:30, as the temple table has
    /// it (`tirumala-kalam-table`).
    #[cfg(feature = "i18n")]
    #[test]
    fn the_first_of_january_2025_has_drik_panchangs_kalams() {
        let delhi = Location::new(
            28.0 + 38.0 / 60.0 + 8.0 / 3_600.0,
            77.0 + 13.0 / 60.0 + 28.0 / 3_600.0,
            0.0,
        );
        let day = hc_calendars_solar::gregorian::to_fixed(2025, 1, 1)
            .expect("a date")
            .0;
        let text = kalam_lines("rahu-kalam-sunrise", day, delhi, "en").expect("in range");
        let rows: alloc::vec::Vec<alloc::vec::Vec<&str>> = text
            .lines()
            .map(|line| line.split('\t').collect())
            .collect();
        assert_eq!(rows.len(), 3);
        let ist_midnight = (day - hc_calendar::fixed::RD_OF_UNIX_EPOCH) * 86_400 - 19_800;
        for (row, (id, part, start)) in rows.iter().zip([
            ("rahu-kalam", "5", 12 * 60 + 25),
            ("yamaganda", "2", 8 * 60 + 32),
            ("gulika-kalam", "4", 11 * 60 + 7),
        ]) {
            assert_eq!(row.len(), KALAM_COLUMNS);
            assert_eq!((row[0], row[4], row[5]), (id, part, "universal"));
            assert_eq!(row[1], row[2]);
            assert_eq!(row[3], "en");
            let at: i64 = row[6].parse().expect("an instant");
            let minutes = (at - ist_midnight) as f64 / 60.0 - f64::from(start);
            assert!(minutes.abs() < 1.0, "{id}: {minutes} min");
            assert_eq!(row[8..], ["", "", "", ""]);
        }
        let fixed = kalam_lines("RAHU-KALAM-FIXED", day, delhi, "hi").expect("in range");
        let first: alloc::vec::Vec<&str> =
            fixed.lines().next().expect("a line").split('\t').collect();
        // Drik Panchang's Hindi day pañcāṅga labels it राहुकाल.
        assert_eq!(
            first[2..8],
            ["राहुकाल", "hi", "5", "local", "43200", "48600"]
        );
        let tromso = Location::new(69.6496, 18.9560, 0.0);
        let midwinter = hc_calendars_solar::gregorian::to_fixed(2024, 12, 21)
            .expect("a date")
            .0;
        let polar = kalam_lines("rahu-kalam-sunrise", midwinter, tromso, "en").expect("in range");
        let cells: alloc::vec::Vec<&str> =
            polar.lines().next().expect("a line").split('\t').collect();
        assert_eq!(cells[6..10], ["", "", "sunrise", &midwinter.to_string()]);
        assert_eq!(
            kalam_lines("yamardha", day, delhi, "en"),
            Err(Refusal::Unknown)
        );
        // The names are `hc-calendars-indic`'s table's, each with its clock.
        for convention in KalamConvention::ALL {
            let text = kalam_lines(convention.id, day, delhi, "en").expect("in range");
            let clock = match convention.clock {
                SpanClock::Universal => "universal",
                SpanClock::Local => "local",
            };
            assert!(
                text.lines()
                    .all(|line| line.split('\t').nth(5) == Some(clock)),
                "{}",
                convention.id
            );
        }
    }

    /// Drik Panchang's page for 1 January 2025, as
    /// `hc_calendars_indic::panchanga`'s test reads it: "Yoga Vyaghata upto
    /// 05:07 PM" and "Karana Balava upto 02:55 PM", IST, read at sunrise:
    /// 11:37 and 09:25 UTC, POSIX 1 735 731 420 and 1 735 723 500.
    #[test]
    fn the_first_of_january_2025_carries_vyaghata_and_balava() {
        let place = Location::new(23.183_333, 82.5, 0.0);
        let day = hc_calendars_solar::gregorian::to_fixed(2025, 1, 1)
            .expect("a date")
            .0;
        let text = panchanga_of_day_lines(day, place, "lahiri").expect("a sunrise");
        let rows: alloc::vec::Vec<alloc::vec::Vec<&str>> = text
            .lines()
            .map(|line| line.split('\t').collect())
            .collect();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0][..4], ["yoga", "13", "Vyaghata", "व्याघात"]);
        assert!(rows.iter().all(|row| row.len() == 9), "{rows:?}");
        assert_eq!(rows[0][7..], ["lahiri", "Lahiri (Chitrapaksha)"]);
        // The identifier the line writes is the one the lookup reads.
        assert_eq!(ayanamsa(rows[0][7]), Ok(Ayanamsa::LAHIRI));
        let ends: i64 = rows[0][5].parse().expect("an instant");
        assert!((ends - 1_735_731_420).abs() < 90, "{ends}");
        assert_eq!(rows[1][0], "karana");
        assert_eq!(rows[1][2..4], ["Balava", "बालव"]);
        let ends: i64 = rows[1][5].parse().expect("an instant");
        assert!((0..120).contains(&(ends - 1_735_723_500)), "{ends}");
        assert_eq!(rows[1][7..], ["", ""]);
        assert_eq!(rows[0][6], rows[1][6]);
        assert_eq!(
            panchanga_of_day_lines(day, place, ""),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            panchanga_of_day_lines(day, place, "Lahiri (Chitrapaksha)"),
            Err(Refusal::Unknown)
        );
        let polar = Location::new(89.0, 0.0, 0.0);
        assert_eq!(
            panchanga_of_day_lines(day, polar, "lahiri"),
            Err(Refusal::NoData)
        );
    }

    const NEW_DELHI: Location = Location::new(
        28.0 + 38.0 / 60.0 + 8.0 / 3_600.0,
        77.0 + 13.0 / 60.0 + 28.0 / 3_600.0,
        0.0,
    );

    fn day(year: i64, month: u8, day: u8) -> i64 {
        hc_calendars_solar::gregorian::to_fixed(year, month, day)
            .expect("a date")
            .0
    }

    /// Drik Panchang's New Delhi page of 1 January 2025, a Wednesday
    /// (`drik-day-panchang-2025`): no Abhijit, and the eighth muhūrta of
    /// the day as Dur Muhurtam; the muhūrtas tile the day and the night.
    #[test]
    fn the_muhurtas_tile_the_day_and_mark_what_the_panchanga_prints() {
        let text = muhurtas_lines(day(2025, 1, 1), NEW_DELHI).expect("in range");
        let rows: Vec<Vec<&str>> = text
            .lines()
            .map(|line| line.split('\t').collect())
            .collect();
        assert_eq!(rows.len(), 30);
        assert!(rows.iter().all(|row| row.len() == MUHURTA_COLUMNS));
        let marked: Vec<(&str, &str, &str)> = rows
            .iter()
            .filter(|row| !row[5].is_empty())
            .map(|row| (row[0], row[1], row[5]))
            .collect();
        assert_eq!(marked, [("day", "8", "dur-muhurtam")]);
        // Wikipedia's names: Rudra first, Vidhi the eighth of the day,
        // Girīśa at sunset and Samudra last (`wikipedia-muhurta`).
        let names: Vec<(&str, &str, &str)> = [0, 7, 15, 29]
            .iter()
            .map(|&index| (rows[index][0], rows[index][1], rows[index][2]))
            .collect();
        assert_eq!(
            names,
            [
                ("day", "1", "Rudra"),
                ("day", "8", "Vidhi"),
                ("night", "1", "Girīśa"),
                ("night", "15", "Samudra")
            ]
        );
        for pair in rows.windows(2) {
            assert_eq!(pair[0][4], pair[1][3]);
        }
        // Thursday 2 January: Abhijit is the eighth, and Dur Muhurtam the
        // sixth and the twelfth.
        let text = muhurtas_lines(day(2025, 1, 2), NEW_DELHI).expect("in range");
        let marked: Vec<String> = text
            .lines()
            .filter_map(|line| {
                let cells: Vec<&str> = line.split('\t').collect();
                (!cells[5].is_empty()).then(|| alloc::format!("{} {}", cells[1], cells[5]))
            })
            .collect();
        assert_eq!(marked, ["6 dur-muhurtam", "8 abhijit", "12 dur-muhurtam"]);
        let polar = Location::new(78.0, 15.0, 0.0);
        let text = muhurtas_lines(day(2025, 1, 1), polar).expect("in range");
        assert!(
            text.lines()
                .all(|line| line.split('\t').nth(6) == Some("sunrise"))
        );
    }

    /// Drik Panchang prints the amṛta siddhi yoga on Tuesday 7 January
    /// 2025, from the Moon's entry into Aśvinī, and on no day between it
    /// and Saturday 11 January.
    #[test]
    fn amrita_siddhi_is_on_drik_panchangs_days() {
        let line = amrita_siddhi_line(day(2025, 1, 7), NEW_DELHI, "lahiri").expect("in range");
        let cells = crate::boundary::cells(&line);
        assert_eq!(cells.len(), AMRITA_SIDDHI_COLUMNS);
        assert_eq!(
            cells[..5],
            [
                "Amrita Siddhi Yoga",
                "अमृत सिद्धि योग",
                "1",
                "ashvini",
                "Aśvinī"
            ]
        );
        assert_eq!(cells[7..], ["1", "lahiri"]);
        let none = amrita_siddhi_line(day(2025, 1, 8), NEW_DELHI, "lahiri").expect("in range");
        // Wednesday's nakṣatra is Anurādhā.
        assert_eq!(
            crate::boundary::cells(&none)[2..8],
            ["17", "anuradha", "Anurādhā", "", "", "0"]
        );
        assert_eq!(
            amrita_siddhi_line(day(2025, 1, 7), NEW_DELHI, "tropical"),
            Err(Refusal::Unknown)
        );
    }

    /// The nakṣatra the amṛta siddhi yoga of 7 January 2025 takes is the
    /// one the Moon is in when it begins.
    #[test]
    fn the_nakshatra_is_the_moons() {
        let line = amrita_siddhi_line(day(2025, 1, 7), NEW_DELHI, "lahiri").expect("in range");
        let start: i64 = crate::boundary::cells(&line)[5].parse().expect("a start");
        let at = nakshatra_at_lines(start + 60, "lahiri").expect("in range");
        let cells = crate::boundary::cells(&at);
        assert_eq!(cells.len(), NAKSHATRA_COLUMNS);
        assert_eq!(cells[..3], ["1", "ashvini", "Aśvinī"]);
        assert!((cells[3].parse::<i64>().expect("an entry") - start).abs() <= 1);
        assert_eq!(cells[6], "lahiri");
        assert!(nakshatra_of_day_lines(day(2025, 1, 7), NEW_DELHI, "lahiri").is_ok());
    }

    /// On the Siddhānta's sky the yoga and the karaṇa are the book's, and
    /// both lines name the sky; Sewell and Dikshit's Poona pañcāṅga of
    /// September 1894 is the anchor `hc-calendars-indic` holds them to.
    #[test]
    fn the_siddhantas_sky_is_named_on_both_lines() {
        let unix = 1_700_000_000;
        let text = panchanga_at_lines(unix, "Surya-Siddhanta").expect("in range");
        let rows: Vec<Vec<&str>> = text
            .lines()
            .map(|line| line.split('\t').collect())
            .collect();
        assert_eq!(rows[0][7..], ["surya-siddhanta", "Sūrya Siddhānta"]);
        assert_eq!(rows[1][7..], ["surya-siddhanta", "Sūrya Siddhānta"]);
        let moment = moment_in_era(unix).expect("in era");
        assert_eq!(
            rows[0][1],
            alloc::format!("{}", surya_siddhanta::yoga_at(moment))
        );
        assert_eq!(
            rows[1][1],
            alloc::format!("{}", surya_siddhanta::karana_at(moment))
        );
        let text = panchanga_of_day_lines(
            day(1894, 9, 2),
            Location::new(18.52, 73.87, 0.0),
            "surya-siddhanta",
        )
        .expect("in range");
        assert_eq!(text.lines().count(), 2);
        assert_eq!(
            panchanga_of_day_lines(
                day(1894, 9, 2),
                Location::new(70.0, 0.0, 0.0),
                "surya-siddhanta"
            ),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            panchanga_at_lines(-200_000_000_000, "surya-siddhanta"),
            Err(Refusal::OutOfRange)
        );
    }
    /// The POSIX second of a Universal Time date and time.
    fn unix(year: i64, month: u8, date: u8, hour: i64, minute: i64) -> i64 {
        (day(year, month, date) - hc_calendar::fixed::RD_OF_UNIX_EPOCH) * 86_400
            + hour * 3_600
            + minute * 60
    }

    const TOKYO: Location = Location::new(35.6894, 139.6917, 0.0);

    /// Drik Panchang's ISKCON Janmashtami dates for Tokyo
    /// (`drik-iskcon-janmashtami`, read 2026-10-03) and the days the
    /// *Rashtriya Panchang* lists, 6 September 2023 and 26 August 2024
    /// (`rashtriya-panchang-1946`), through the boundary's lines.
    #[test]
    fn the_two_readings_of_janmashtami_are_listed_and_read() {
        let table = rows_of(&festival_readings_lines());
        assert_eq!(table.len(), vaishnava::FestivalReading::ALL.len());
        assert!(
            table
                .iter()
                .all(|row| row.len() == FESTIVAL_READING_COLUMNS)
        );
        assert_eq!(table[0][0], "smarta");
        assert_eq!(table[1][0], "vaishnava");
        let read = |year, reading, place| {
            let text = janmashtami_line(year, reading, place, "lahiri").expect("a day");
            let row = rows_of(&text).remove(0);
            assert_eq!(row.len(), JANMASHTAMI_COLUMNS);
            (row[2].parse::<i64>().expect("a day"), row)
        };
        // Tokyo's list: 27 August 2024, 16 August 2025, 5 September 2026.
        for (year, month, date) in [(2024, 8, 27), (2025, 8, 16), (2026, 9, 5), (2034, 9, 6)] {
            let (fixed, row) = read(year, "vaishnava", TOKYO);
            assert_eq!(fixed, day(year, month, date), "{year}");
            assert_eq!(row[0], "vaishnava");
            assert_eq!(
                [&*row[3], &*row[4], &*row[5]],
                [&*year.to_string(), &*month.to_string(), &*date.to_string()]
            );
        }
        // The national almanac's Smārta days, at the Central Station.
        let station = HinduLunarCalendar::RASHTRIYA.location;
        for (year, month, date) in [(2023, 9, 6), (2024, 8, 26), (2025, 8, 15)] {
            let (fixed, row) = read(year, "smarta", station);
            assert_eq!(fixed, day(year, month, date), "{year}");
            assert_eq!(row[0], "smarta");
        }
        assert_eq!(
            janmashtami_line(2025, "iskcon", station, "lahiri"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            janmashtami_line(2025, "smarta", station, "nope"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            janmashtami_line(1000, "smarta", station, "lahiri"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            janmashtami_line(2025, "smarta", Location::new(70.0, 0.0, 0.0), "lahiri"),
            Err(Refusal::OutOfRange)
        );
    }

    /// Śrāvaṇa kṛṣṇa 8 of Śaka 1947 at Tokyo is Drik Panchang's ISKCON day,
    /// 16 August 2025, and any tithi has its Vaiṣṇava day.
    #[test]
    fn the_vaishnava_day_of_a_tithi_is_the_first_sunrise_that_carries_it() {
        let text = vaishnava_day_line(1947, 5, 23, TOKYO, "lahiri").expect("a day");
        let row = rows_of(&text).remove(0);
        assert_eq!(row.len(), VAISHNAVA_DAY_COLUMNS);
        assert_eq!(row[..3], ["1947", "5", "23"]);
        assert_eq!(row[3], day(2025, 8, 16).to_string());
        assert!(row[7] == "23" || row[7] == "24", "{row:?}");
        assert_eq!(
            vaishnava_day_line(1947, 5, 31, TOKYO, "lahiri"),
            Err(Refusal::InvalidDate)
        );
        assert_eq!(
            vaishnava_day_line(1947, 13, 1, TOKYO, "lahiri"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            vaishnava_day_line(1947, 5, 23, TOKYO, "nope"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            vaishnava_day_line(1000, 5, 23, TOKYO, "lahiri"),
            Err(Refusal::OutOfRange)
        );
    }

    /// Bhadra of Rakṣā Bandhana 2024 ended at 13:33 IST on 19 August, as
    /// the page the library follows prints it (`onlinejyotish-rakhi-2024`,
    /// secondary), 08:03 UT, and the free half runs to the end of the
    /// tithi.
    #[test]
    fn the_free_span_of_the_full_moon_begins_where_bhadra_ends() {
        let station = HinduLunarCalendar::RASHTRIYA.location;
        let text = vishti_free_span_line(1946, 5, 15, station, "lahiri").expect("a span");
        let row = rows_of(&text).remove(0);
        assert_eq!(row.len(), VISHTI_FREE_SPAN_COLUMNS);
        let from: i64 = row[4].parse().expect("an instant");
        let to: i64 = row[5].parse().expect("an instant");
        assert!((from - unix(2024, 8, 19, 8, 3)).abs() <= 600, "{from}");
        assert!(to > from && to - from < 86_400);
        assert_eq!(row[6], "lahiri");
        // The fourth tithi has a free half too; the first never meets Viṣṭi.
        let first = vishti_free_span_line(1946, 5, 1, station, "lahiri").expect("a span");
        assert!(rows_of(&first)[0][4].is_empty() || !rows_of(&first)[0][5].is_empty());
        assert_eq!(
            vishti_free_span_line(1946, 5, 0, station, "lahiri"),
            Err(Refusal::InvalidDate)
        );
        assert_eq!(
            vishti_free_span_line(1946, 0, 15, station, "lahiri"),
            Err(Refusal::OutOfRange)
        );
    }

    /// Drik Panchang's mean Rāhu transits for New Delhi
    /// (`drik-rahu-transit`, read 2026-10-03): Rāhu enters Kumbha on 18 May
    /// 2025 at 16:30 IST, Mīna on 30 October 2023, Makara on 5 December
    /// 2026, with Drik's ayanāṃśa; Ketu stands opposite.
    #[test]
    fn rahu_and_ketu_stand_where_drik_panchang_says() {
        let after = unix(2025, 5, 19, 12, 0);
        let row = rows_of(&rahu_at_line(after, "lahiri-drik").expect("a node")).remove(0);
        assert_eq!(row.len(), RAHU_AT_COLUMNS);
        assert_eq!(row[0], "mean");
        assert_eq!(row[2..5], ["11", "kumbha", "Kumbha"]);
        assert_eq!(row[6..9], ["5", "simha", "Siṃha"]);
        assert_eq!(row[9], "lahiri-drik");
        let rahu: f64 = row[1].parse().expect("degrees");
        let ketu: f64 = row[5].parse().expect("degrees");
        assert!(((ketu - rahu).rem_euclid(360.0) - 180.0).abs() < 1e-6);
        let text = rahu_ingresses_lines(
            unix(2023, 1, 1, 0, 0),
            unix(2027, 1, 1, 0, 0),
            "lahiri-drik",
        )
        .expect("ingresses");
        let rows = rows_of(&text);
        assert_eq!(rows.len(), 3);
        assert!(rows.iter().all(|row| row.len() == RAHU_INGRESS_COLUMNS));
        // 16:30 IST is 11:00 UT; within three minutes.
        for (row, (y, m, d, h, mi), into) in [
            (&rows[0], (2023, 10, 30, 8, 3), "mina"),
            (&rows[1], (2025, 5, 18, 11, 0), "kumbha"),
            (&rows[2], (2026, 12, 5, 13, 58), "makara"),
        ] {
            let at: i64 = row[0].parse().expect("an instant");
            assert!((at - unix(y, m, d, h, mi)).abs() <= 180, "{row:?}");
            assert_eq!(row[3], into);
        }
        assert_eq!(rows[1][5], "simha");
        assert_eq!(rahu_ingresses_lines(0, 1, "nope"), Err(Refusal::Unknown));
        assert_eq!(
            rahu_ingresses_lines(0, 4_000_000_000, "lahiri"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(rahu_ingresses_lines(10, 10, "lahiri"), Ok(String::new()));
        assert_eq!(
            rahu_at_line(-200_000_000_000, "lahiri"),
            Err(Refusal::OutOfRange)
        );
    }
    /// Chedi year 1 opens on Kielhorn's 5 September 248 (Julian), Gupta year
    /// 1 on 26 February 320 and the Mithila Panchang's Lakṣmaṇa Sena year
    /// 907 at the Kārttika of 2025 (`sewell1896`, `hinducalculator-mithila-panchang`).
    #[test]
    fn an_eras_new_year_is_the_day_its_year_opens() {
        let julian = |year, month, date| {
            hc_calendars_solar::julian::to_fixed(year, month, date)
                .expect("a date")
                .0
        };
        assert_eq!(era_new_year("kalachuri", 1), Ok(julian(248, 9, 5)));
        assert_eq!(era_new_year("gupta", 1), Ok(julian(320, 2, 26)));
        let opening = era_new_year("lakshmana-sena", 907).expect("a day");
        // Kārttika śukla 1 of Śaka 1947, 22 October 2025.
        assert_eq!(opening, day(2025, 10, 22));
        assert_eq!(era_new_year("hindu-lunar", 1), Err(Refusal::Unknown));
        assert_eq!(era_new_year("gupta", 100_000), Err(Refusal::OutOfRange));
    }

    /// A year can lack an ordinary month: the Central Station's Śaka 1885
    /// has no Mārgaśīrṣa (month 9) of its own, and the line says the month is
    /// out of the calendar's range, as the calendar does.
    #[test]
    fn a_month_the_year_lacks_is_out_of_range() {
        let station = HinduLunarCalendar::RASHTRIYA.location;
        assert_eq!(
            vaishnava_day_line(1885, 9, 15, station, "lahiri"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            vishti_free_span_line(1885, 9, 15, station, "lahiri"),
            Err(Refusal::OutOfRange)
        );
        assert!(vaishnava_day_line(1884, 9, 15, station, "lahiri").is_ok());
    }
}
