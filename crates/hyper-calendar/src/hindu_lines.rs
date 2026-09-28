//! The tab-separated lines the WebAssembly module and the C library write
//! about the Hindu lunisolar date and the *Sūrya Siddhānta*'s sky, written
//! once.
//!
//! * The amānta lunisolar date of a day at a place the caller gives, on
//!   one of two skies: the true Sun and Moon in the zodiac of a named
//!   ayanāṃśa, as `hindu-lunar` reads them at the Central Station, or the
//!   *Sūrya Siddhānta*'s, as `hindu-lunar-surya-siddhanta` reads them at
//!   Ujjain. The place is a parameter and not a calendar of its own
//!   (`docs/policy.md` §5); `docs/systems/hindu-calendars.md` says how
//!   often it moves a date.
//! * The *Sūrya Siddhānta*'s Sun and Moon at an instant, with the tithi and
//!   the sign, and its sunrise on a day at a place
//!   ([`hc_calendars_indic::surya_siddhanta`]).
//!
//! The date's line ends with its labels in a locale, from the vocabulary
//! `describe_day` uses for `hindu-lunar` and `hindu-lunar-surya-siddhanta`
//! ([`crate::lines`]): the month, the word the locale writes before an
//! intercalary month, the Śaka and Vikrama eras, and the tag of the data
//! that answered.
//!
//! The true sky answers for the years the true calendars convert, Śaka
//! 1622 through 2221, from Chaitra śukla 1 in March 1700 to the eve of
//! the one in March 2300; the Siddhānta's, which is arithmetic, for the
//! days of `hindu-lunar-surya-siddhanta`, Kali Yuga 1 to 10 000, which
//! [`SIDDHANTA_FIRST_DAY`] and [`SIDDHANTA_LAST_DAY`] name.

use alloc::string::String;

use hc_astro::riseset::{Location, sunrise};
use hc_calendar::fixed::Moment;
use hc_calendar::{Calendar, CalendarId, DynAdapter, DynCalendar, Rd};
use hc_calendars_indic::barhaspatya::{self, MeanSignRule};
use hc_calendars_indic::surya_siddhanta::{self, MAX_SUNRISE_LATITUDE};
use hc_calendars_indic::{
    HinduLunarCalendar, HinduLunarDate, HinduPurnimantaCalendar, SiddhantaLunarCalendar,
};
use hc_core::math::floor;
use hc_i18n::Locale;
use hc_i18n::names::{self as vocabulary, NameContext, NameWidth};

use crate::astro_lines::{moment_on_day, unix_from_moment};
use crate::boundary::{Answer, Line, Refusal};
use crate::lines::{era_label_or_empty, locale_for, locale_used, month_label_or_empty};
use crate::panchanga_lines::ayanamsa;
use hc_core::catalogue::matches;

/// The name of the *Sūrya Siddhānta*'s sky, beside the ayanāṃśa names of
/// the true one.
pub use hc_calendars_indic::surya_siddhanta::SKY as SURYA_SIDDHANTA;

/// The first fixed day the Siddhānta's exports answer for: Chaitra śukla 1
/// of Kali Yuga 1 on `hindu-lunar-surya-siddhanta`, 13 January 3101 BCE.
pub const SIDDHANTA_FIRST_DAY: i64 = -1_132_604;

/// The last fixed day the Siddhānta's exports answer for, the last of Kali
/// Yuga 10 000 on the same calendar, 15 June 6900.
pub const SIDDHANTA_LAST_DAY: i64 = 2_519_974;

/// How many columns [`hindu_lunar_date_line`] writes.
pub const HINDU_LUNAR_DATE_COLUMNS: usize = 12;

/// The era code of the Vikrama Saṃvat in `hc-i18n`'s vocabulary.
const VIKRAMA: &str = "vs";

/// The calendars whose vocabulary names the Vikrama Saṃvat, `vs`, in the
/// order they are asked: the Kārttikādi lunisolar year's, then the Vikrami
/// solar year's. The amānta calendars count the same era from Chaitra and
/// have no entry for it of their own.
const VIKRAMA_VOCABULARY: [CalendarId; 2] = [
    CalendarId("vikram-samvat-kartikadi"),
    CalendarId("hindu-solar-vikrami"),
];

/// A fixed day the Siddhānta's exports answer for.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] outside [`SIDDHANTA_FIRST_DAY`] to
/// [`SIDDHANTA_LAST_DAY`].
fn siddhanta_day(fixed: i64) -> Answer<Rd> {
    if (SIDDHANTA_FIRST_DAY..=SIDDHANTA_LAST_DAY).contains(&fixed) {
        Ok(Rd(fixed))
    } else {
        Err(Refusal::OutOfRange)
    }
}

/// A Universal Time instant on a day the Siddhānta's exports answer for.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] on any other day.
fn siddhanta_moment(universal_unix: i64) -> Answer<Moment> {
    moment_on_day(universal_unix, siddhanta_day)
}

/// A place where the Sun rises every day, on either sky: within
/// [`MAX_SUNRISE_LATITUDE`] of the equator. A lunisolar month begins at the
/// first sunrise after a conjunction, so a place with even one day of the
/// year without a sunrise has months that cannot be read there.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] beyond it.
fn sunrise_place(place: Location) -> Answer<Location> {
    if place.latitude_degrees.abs() <= MAX_SUNRISE_LATITUDE {
        Ok(place)
    } else {
        Err(Refusal::OutOfRange)
    }
}

/// The line of a date and the sunrise it was read at, with its labels in
/// the locale a tag asks for, from the vocabulary of `calendar`, the
/// registered calendar of the sky that read it.
fn date_line(
    date: HinduLunarDate,
    sunrise: Moment,
    calendar: &dyn DynCalendar,
    fields: &hc_calendar::DateFields,
    tag: &str,
) -> String {
    let locale = locale_for(calendar, tag);
    let id = calendar.meta().id;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(date.year)
        .value(date.vikrama_year())
        .value(date.month)
        .flag(date.leap_month)
        .value(date.day)
        .flag(date.leap_day)
        .value(unix_from_moment(sunrise))
        .cell(&month_label_or_empty(&locale, calendar, fields))
        .cell_or_empty(
            date.leap_month
                .then(|| vocabulary::leap_month_prefix(&locale, id).trim_end()),
        )
        .cell(era_label_or_empty(
            &locale,
            calendar,
            hc_calendars_indic::hindu_lunar::ERA,
        ));
    // The locale's name for the era in either calendar that names it, else
    // English's: the fallback `hc_i18n::names::era_label` ends at for
    // every era, so that the cell is a name or empty and never a code.
    let named_in = |locale: &hc_i18n::Locale| {
        VIKRAMA_VOCABULARY.iter().find_map(|&vocabulary_of| {
            vocabulary::era_name_by_code(locale, vocabulary_of, VIKRAMA, NameWidth::Wide)
        })
    };
    let vikrama = named_in(&locale).or_else(|| named_in(&vocabulary::english()));
    line.cell_or_empty(vikrama).cell(locale_used(&locale));
    line.end();
    out
}

/// The line of `hc_hindu_lunar_date`: the amānta lunisolar date of a fixed
/// day read at the sunrise of a place, as the Śaka year, the Vikrama year,
/// the month (1 for Chaitra through 12 for Phālguna), whether it is the
/// intercalary month, 1 or 0, the tithi (1 through 30), whether the day is
/// the second to carry it, 1 or 0, and the sunrise the day was read at, as
/// whole POSIX seconds of Universal Time, rounded down; then, in the
/// locale `locale` names, the month's name, with the locale's word for an
/// intercalary month before it where the month is one (`Bhadra`,
/// `Adhika Sravana`; भाद्रपद under `hi`); that word alone for an
/// intercalary month (`Adhika`, अधिक), else empty; the Śaka era's name
/// (`Saka`); the Vikrama Saṃvat's (`Vikrama Samvat`); and the tag of the
/// data that answered.
///
/// The names are the vocabulary `describe_day` uses for `hindu-lunar`, or
/// for `hindu-lunar-surya-siddhanta` on the Siddhānta's sky, resolved as
/// [`crate::lines`] resolves every locale: the one asked for where it
/// names the calendar, else English; `native` asks for the calendar's own
/// languages first, Sanskrit then Hindi. The amānta calendars have no
/// name for the Vikrama Saṃvat of their own, so it is the one the
/// Kārttikādi and Vikrami calendars' vocabulary gives the same era, `vs`.
/// The Śaka era's name is the era cell `describe_day` writes for the
/// calendar, by `hc-i18n`'s one rule for an era's name,
/// [`hc_i18n::names::era_label`]: the locale's; else, for an era several
/// calendars count ([`hc_i18n::data::SHARED_ERAS`]), the national
/// calendar's, whose years are Śaka years too — Hindi's शक; else the
/// calendar's own; else English's. The Vikrama Saṃvat's falls back to
/// English's the same way. So an era cell is a name, never a code:
/// Sanskrit names neither era and writes `Saka` and `Vikrama Samvat`, and
/// Hindi, which names no Vikrama Saṃvat, `Vikrama Samvat`.
///
/// `sky` is an ayanāṃśa [`ayanamsa`] names, for the true Sun and Moon in
/// its zodiac read at the place's sunrise, as `hindu-lunar` is with Lahiri's
/// at the Central Station; or [`SURYA_SIDDHANTA`], for the Siddhānta's Sun
/// and Moon read at its own sunrise there, as
/// `hindu-lunar-surya-siddhanta` is at Ujjain.
///
/// `sky` comes first because it chooses the reckoning that reads the day,
/// the true calendar or the Siddhānta's, as a calendar's name comes first
/// in [`crate::planetary_lines::circad_date_line`].
/// [`crate::panchanga_lines::panchanga_of_day_lines`] has one reckoning,
/// and its ayanāṃśa, last, sets only the zodiac it reads the day in.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a sky not named. [`Refusal::OutOfRange`] on
/// either sky for a place beyond [`MAX_SUNRISE_LATITUDE`], where some day
/// of the year has no sunrise; on the true sky for a day outside Śaka 1622
/// through 2221, Chaitra śukla 1 in March 1700 to the eve of the one in
/// March 2300, and on the Siddhānta's for a day outside
/// [`SIDDHANTA_FIRST_DAY`] to [`SIDDHANTA_LAST_DAY`]. [`Refusal::NoData`]
/// on the true sky for a day whose sunrise at the place, or a search that
/// reads it, the model does not find.
pub fn hindu_lunar_date_line(
    sky: &str,
    fixed: i64,
    place: Location,
    locale: &str,
) -> Answer<String> {
    if matches(sky, SURYA_SIDDHANTA) {
        let day = siddhanta_day(fixed)?;
        let place = sunrise_place(place)?;
        let date = SiddhantaLunarCalendar::new(place)
            .from_fixed(day)
            .map_err(Refusal::from)?;
        let registered = SiddhantaLunarCalendar::UJJAIN;
        let fields = registered.to_fields(date).map_err(Refusal::from)?;
        return Ok(date_line(
            date,
            surya_siddhanta::sunrise(day, place),
            &DynAdapter::new(registered),
            &fields,
            locale,
        ));
    }
    let place = sunrise_place(place)?;
    let calendar = HinduLunarCalendar::new(place, ayanamsa(sky)?);
    let day = Rd(fixed);
    let date = calendar.from_fixed(day).map_err(Refusal::from)?;
    let rise = sunrise(day, place).ok_or(Refusal::NoData)?;
    let registered = HinduLunarCalendar::RASHTRIYA;
    let fields = registered.to_fields(date).map_err(Refusal::from)?;
    Ok(date_line(
        date,
        rise,
        &DynAdapter::new(registered),
        &fields,
        locale,
    ))
}

/// The line of `hc_surya_siddhanta_at`: the Siddhānta's Sun and Moon at a
/// Universal Time instant, as the Sun's and the Moon's sidereal longitudes
/// in degrees, the Moon's elongation from the Sun in degrees, 0 to 360, the
/// tithi in progress (1 through 30) and the sign the Sun is in (1 for Meṣa
/// through 12 for Mīna).
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for an instant outside the days
/// [`SIDDHANTA_FIRST_DAY`] to [`SIDDHANTA_LAST_DAY`].
pub fn surya_siddhanta_line(universal_unix: i64) -> Answer<String> {
    let moment = siddhanta_moment(universal_unix)?;
    let sun = surya_siddhanta::solar_longitude(moment);
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(sun)
        .value(surya_siddhanta::lunar_longitude(moment))
        .value(surya_siddhanta::lunar_phase(moment))
        .value(surya_siddhanta::tithi_at(moment))
        .value(floor(sun / 30.0) as u8 % 12 + 1);
    line.end();
    Ok(out)
}

/// The line of `hc_surya_siddhanta_sunrise`: the Siddhānta's sunrise on a
/// day at a place, as whole POSIX seconds of Universal Time, rounded down.
/// The Siddhānta reads the place's latitude and its longitude and nothing
/// else.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a day outside [`SIDDHANTA_FIRST_DAY`] to
/// [`SIDDHANTA_LAST_DAY`] or a place beyond [`MAX_SUNRISE_LATITUDE`].
pub fn surya_siddhanta_sunrise_line(fixed: i64, place: Location) -> Answer<String> {
    let day = siddhanta_day(fixed)?;
    let place = sunrise_place(place)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(unix_from_moment(surya_siddhanta::sunrise(day, place)));
    line.end();
    Ok(out)
}

/// The rule of [`barhaspatya::RULES`] a name names, by
/// [`barhaspatya::by_id`]:
/// `surya-siddhanta-bija`, the *Sūrya Siddhānta* with the *bīja*, by which
/// the registered pūrṇimānta calendar names its years;
/// `surya-siddhanta`, the same without it; or `arya-siddhanta`.
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other name.
pub fn barhaspatya_rule(name: &str) -> Answer<MeanSignRule> {
    barhaspatya::by_id(name).ok_or(Refusal::Unknown)
}

/// The first and last Śaka years, expired, the Bārhaspatya exports answer
/// for: Kali Yuga 1 to 10 000 expired, the years of
/// [`SIDDHANTA_FIRST_DAY`] to [`SIDDHANTA_LAST_DAY`].
pub const BARHASPATYA_SAKA_YEARS: core::ops::RangeInclusive<i64> =
    1 - barhaspatya::SAKA_TO_KALI..=10_000 - barhaspatya::SAKA_TO_KALI;

/// The name of a position of the northern cycle in the locale of the
/// registered pūrṇimānta calendar's vocabulary, as a cell, as
/// `hc_day_extras` writes its `barhaspatya-samvatsara` field.
fn samvatsara_name(line: &mut Line<'_>, calendar: &dyn DynCalendar, locale: &Locale, position: u8) {
    line.cell_with(|cell| {
        // A cell never refuses a write.
        let _ = hc_i18n::fields::write_value_name(
            locale,
            calendar.meta().id,
            calendar.cycles(),
            barhaspatya::FIELD,
            i64::from(position),
            NameWidth::Wide,
            NameContext::Format,
            cell,
        );
    });
}

/// How many columns [`barhaspatya_year_line`] writes.
pub const BARHASPATYA_YEAR_COLUMNS: usize = 5;

/// The line of `hc_barhaspatya_year`: the name of the northern sixty-year
/// cycle a rule couples with the year that begins in an *expired* Śaka
/// year, the year the *Rashtriya Panchang* prints — the name current at
/// the apparent Meṣa saṅkrānti of its solar year
/// ([`MeanSignRule::of_saka`]) — and the name the rule expunges in that
/// solar year, if any ([`MeanSignRule::expunged_in`]).
///
/// The cells: the name's position, 1 for Prabhava through 60 for Kṣaya;
/// its name in the locale; the expunged name's position and name, both
/// empty in a year that expunges none; and the locale used, the tag of the
/// locale data that answered, as `hc_day_extras` writes it — the names are
/// English's where that locale names no year, so `hi` is written beside
/// Pingala. The names are the pūrṇimānta calendar's, whose
/// `barhaspatya-samvatsara` field `hc_day_extras` writes, resolved as
/// [`crate::lines`] resolves every locale. `rule` is
/// [`barhaspatya_rule`]'s: the pūrṇimānta calendar's own is
/// `surya-siddhanta-bija`, and some of the Hindi press's announcements of
/// 2021–26 name the years by `surya-siddhanta`, one name on, and others by
/// the pūrṇimānta calendar's rule.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a rule not named, and [`Refusal::OutOfRange`]
/// for a Śaka year outside [`BARHASPATYA_SAKA_YEARS`].
pub fn barhaspatya_year_line(rule: &str, saka: i64, locale: &str) -> Answer<String> {
    let rule = barhaspatya_rule(rule)?;
    if !BARHASPATYA_SAKA_YEARS.contains(&saka) {
        return Err(Refusal::OutOfRange);
    }
    let calendar = DynAdapter::new(HinduPurnimantaCalendar::RASHTRIYA);
    let locale = locale_for(&calendar, locale);
    let position = rule.of_saka(saka);
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(position);
    samvatsara_name(&mut line, &calendar, &locale, position);
    if let Some(expunged) = rule.expunged_in(saka + barhaspatya::SAKA_TO_KALI) {
        line.value(expunged);
        samvatsara_name(&mut line, &calendar, &locale, expunged);
    } else {
        line.empties(2);
    }
    line.cell(locale_used(&locale));
    line.end();
    Ok(out)
}

/// How many columns [`barhaspatya_year_at_line`] writes.
pub const BARHASPATYA_AT_COLUMNS: usize = 3;

/// The line of `hc_barhaspatya_year_at`: the name of the northern cycle in
/// progress at a Universal Time instant by a rule
/// ([`barhaspatya::in_progress_at`]): the name current at the last
/// apparent Meṣa saṅkrānti of the *Sūrya Siddhānta* until the day the rule
/// ends it, and the names after it from then on, which Sewell and Dikshit
/// give as correct within two ghaṭikās where the saṅkrānti is known.
///
/// The cells: the position, 1 to 60; its name in the locale, as
/// [`barhaspatya_year_line`] names it; and the locale used, as there.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a rule not named, and [`Refusal::OutOfRange`]
/// for an instant outside the days [`SIDDHANTA_FIRST_DAY`] to
/// [`SIDDHANTA_LAST_DAY`].
pub fn barhaspatya_year_at_line(rule: &str, universal_unix: i64, locale: &str) -> Answer<String> {
    let rule = barhaspatya_rule(rule)?;
    let moment = siddhanta_moment(universal_unix)?;
    let calendar = DynAdapter::new(HinduPurnimantaCalendar::RASHTRIYA);
    let locale = locale_for(&calendar, locale);
    let position = barhaspatya::in_progress_at(rule, moment);
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(position);
    samvatsara_name(&mut line, &calendar, &locale, position);
    line.cell(locale_used(&locale));
    line.end();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    use hc_calendar::fixed::RD_OF_UNIX_EPOCH;

    use hc_calendars_indic::places::{CENTRAL_STATION, UJJAIN};

    use crate::boundary::cells;

    fn ymd(year: i64, month: u8, day: u8) -> i64 {
        hc_calendars_solar::gregorian::to_fixed(year, month, day)
            .expect("a date")
            .0
    }

    /// Chaitra śukla 1 of Śaka 1947, Vikrama 2082, 30 March 2025: the
    /// *Rashtriya Panchang*'s new year at the Central Station on the true
    /// sky, and the Siddhānta's at Ujjain, as
    /// `docs/systems/hindu-calendars.md` works it, its sunrise there at
    /// 01:01 UT.
    #[test]
    fn the_new_year_of_saka_1947_on_both_skies() {
        let day = ymd(2025, 3, 30);
        let line = hindu_lunar_date_line("Lahiri", day, CENTRAL_STATION, "en").expect("a date");
        assert_eq!(cells(&line)[..6], ["1947", "2082", "1", "0", "1", "0"]);
        let line = hindu_lunar_date_line("SURYA-SIDDHANTA", day, UJJAIN, "en").expect("a date");
        let cells = cells(&line);
        assert_eq!(cells[..6], ["1947", "2082", "1", "0", "1", "0"]);
        let sunrise: i64 = cells[6].parse().expect("an instant");
        // 2025-03-30 01:01 UTC is POSIX 1 743 296 460.
        assert!((sunrise - 1_743_296_460).abs() < 60, "{sunrise}");
    }

    /// 27 September 2026 at Tokyo, `zone1970.tab`'s principal location of
    /// `Asia/Tokyo`, on every ayanāṃśa and on the Siddhānta's sky: Śaka
    /// 1948, Vikrama 2083, the sixth month, Bhādrapada, and the sixteenth
    /// tithi, labelled as `describe_day` labels `hindu-lunar`: Bhadra in
    /// English, भाद्रपद in Hindi and Sanskrit, whose data name no era for
    /// the calendar but for the Śaka era Hindi's शक, and English's names for
    /// every era they do not name. `native` asks for
    /// Sanskrit, and Japanese, which does not name the calendar, is
    /// English.
    #[test]
    fn the_labels_of_27_september_2026_at_tokyo() {
        let day = ymd(2026, 9, 27);
        let tokyo = Location::new(35.654_444, 139.744_722, 0.0);
        for sky in [
            "Lahiri",
            "Raman",
            "Krishnamurti",
            "Fagan-Bradley",
            SURYA_SIDDHANTA,
        ] {
            let labels = |tag: &str| {
                let line = hindu_lunar_date_line(sky, day, tokyo, tag).expect("a date");
                let cells: Vec<String> = cells(&line).iter().map(|cell| (*cell).into()).collect();
                assert_eq!(cells.len(), HINDU_LUNAR_DATE_COLUMNS, "{line}");
                assert_eq!(cells[..6], ["1948", "2083", "6", "0", "16", "0"], "{sky}");
                cells[7..].to_vec()
            };
            assert_eq!(
                labels("en"),
                ["Bhadra", "", "Saka", "Vikrama Samvat", "en"],
                "{sky}"
            );
            assert_eq!(labels("ja"), labels("en"), "{sky}");
            assert_eq!(
                labels("hi-IN"),
                ["भाद्रपद", "", "शक", "Vikrama Samvat", "hi"],
                "{sky}"
            );
            assert_eq!(
                labels("sa"),
                ["भाद्रपद", "", "Saka", "Vikrama Samvat", "sa"],
                "{sky}"
            );
            assert_eq!(labels("native"), labels("sa"), "{sky}");
        }
        // The month label is `describe_day`'s for the registered calendar,
        // and so is the Śaka era's, by the one rule for a shared era: in
        // the era cell, in the formatted date, and in this line, for the
        // true sky's calendar and the Siddhānta's alike.
        for tag in ["hi", "en", "sa"] {
            let described = crate::lines::describe_day(&crate::registry(), Rd(day), tag);
            for (calendar, sky) in [
                ("hindu-lunar", "Lahiri"),
                ("hindu-lunar-surya-siddhanta", SURYA_SIDDHANTA),
            ] {
                let row: Vec<&str> = described
                    .lines()
                    .find(|line| line.starts_with(&alloc::format!("{calendar}\t")))
                    .expect("registered")
                    .split('\t')
                    .collect();
                let line = hindu_lunar_date_line(sky, day, tokyo, tag).expect("a date");
                let cells = cells(&line);
                assert_eq!(row[2], "saka", "{calendar}");
                assert_eq!(row[3], cells[9], "{calendar} {tag}");
                assert_eq!(row[7], cells[7], "{calendar} {tag}");
                assert_eq!(row[16], cells[11], "{calendar} {tag}");
                if !row[3].is_empty() {
                    assert!(row[15].contains(row[3]), "{calendar} {tag}: {}", row[15]);
                }
            }
        }
        let described = crate::lines::describe_day(&crate::registry(), Rd(day), "hi");
        let row = described
            .lines()
            .find(|line| line.starts_with("hindu-lunar\t"))
            .expect("hindu-lunar");
        assert_eq!(row.split('\t').nth(3), Some("शक"));
        assert_eq!(row.split('\t').nth(15), Some("16 भाद्रपद 1948 शक"));
    }

    /// The adhika Śrāvaṇa of Śaka 1945, 18 July to 16 August 2023, as the
    /// *Rashtriya Panchang* has it at the Central Station
    /// (`docs/systems/hindu-calendars.md`): the month label carries the
    /// locale's word for it, and the word has a cell of its own.
    #[test]
    fn an_intercalary_month_is_written_with_the_locales_word() {
        let day = ymd(2023, 8, 1);
        let labels = |tag: &str| {
            let line = hindu_lunar_date_line("Lahiri", day, CENTRAL_STATION, tag).expect("a date");
            let cells: Vec<String> = cells(&line).iter().map(|cell| (*cell).into()).collect();
            assert_eq!(cells[..4], ["1945", "2080", "5", "1"], "{line}");
            cells[7..9].to_vec()
        };
        assert_eq!(labels("en"), ["Adhika Sravana", "Adhika"]);
        assert_eq!(labels("hi"), ["अधिक श्रावण", "अधिक"]);
    }

    /// At Ujjain the true sky's line is `HinduLunarCalendar::UJJAIN`'s date,
    /// which the book's astronomical calendar is.
    #[test]
    fn a_place_given_is_the_calendar_rebuilt_there() {
        for day in ymd(2024, 1, 1)..ymd(2024, 1, 20) {
            let line = hindu_lunar_date_line("lahiri", day, UJJAIN, "en").expect("a date");
            let date = HinduLunarCalendar::UJJAIN
                .from_fixed(Rd(day))
                .expect("in range");
            let registered = HinduLunarCalendar::RASHTRIYA;
            let fields = registered.to_fields(date).expect("fields");
            let expected = date_line(
                date,
                sunrise(Rd(day), UJJAIN).expect("a sunrise"),
                &DynAdapter::new(registered),
                &fields,
                "en",
            );
            assert_eq!(line, expected);
        }
    }

    #[test]
    fn the_refusals_are_named() {
        let day = ymd(2025, 3, 30);
        assert_eq!(
            hindu_lunar_date_line("", day, UJJAIN, "en"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            hindu_lunar_date_line("lahiri", ymd(1699, 12, 31), UJJAIN, "en"),
            Err(Refusal::OutOfRange)
        );
        let north = Location::new(80.0, 20.0, 0.0);
        assert_eq!(
            hindu_lunar_date_line("lahiri", ymd(2024, 12, 21), north, "en"),
            Err(Refusal::OutOfRange)
        );
        // Midsummer at 70° N has a sunrise, but the months there cannot be
        // read: December has days without one.
        assert_eq!(
            hindu_lunar_date_line(
                "lahiri",
                ymd(2024, 6, 21),
                Location::new(70.0, 20.0, 0.0),
                "en"
            ),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            hindu_lunar_date_line(SURYA_SIDDHANTA, day, north, "en"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            hindu_lunar_date_line(SURYA_SIDDHANTA, SIDDHANTA_FIRST_DAY - 1, UJJAIN, "en"),
            Err(Refusal::OutOfRange)
        );
        assert!(hindu_lunar_date_line(SURYA_SIDDHANTA, SIDDHANTA_FIRST_DAY, UJJAIN, "en").is_ok());
        assert!(hindu_lunar_date_line(SURYA_SIDDHANTA, SIDDHANTA_LAST_DAY, UJJAIN, "en").is_ok());
        assert_eq!(
            surya_siddhanta_sunrise_line(day, north),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(surya_siddhanta_line(i64::MAX), Err(Refusal::OutOfRange));
    }

    /// The range the constants name is the registered calendar's.
    #[test]
    fn the_siddhantas_days_are_the_registered_calendars() {
        let calendar = SiddhantaLunarCalendar::UJJAIN;
        assert_eq!(calendar.earliest(), Ok(Rd(SIDDHANTA_FIRST_DAY)));
        assert_eq!(calendar.latest(), Ok(Rd(SIDDHANTA_LAST_DAY)));
    }

    /// At the Siddhānta's sunrise at Ujjain on 30 March 2025 its Moon stands
    /// at 352.87° and its Sun at 345.28°, in Mīna, an elongation of 7.58°:
    /// the first tithi (`docs/systems/hindu-calendars.md`).
    #[test]
    fn the_siddhantas_sky_at_its_sunrise_of_30_march_2025() {
        let line = surya_siddhanta_sunrise_line(ymd(2025, 3, 30), UJJAIN).expect("a sunrise");
        let sunrise: i64 = cells(&line)[0].parse().expect("an instant");
        let line = surya_siddhanta_line(sunrise).expect("in range");
        let cells = cells(&line);
        let number = |index: usize| -> f64 { cells[index].parse().expect("a number") };
        assert!((number(0) - 345.28).abs() < 0.01, "{}", cells[0]);
        assert!((number(1) - 352.87).abs() < 0.01, "{}", cells[1]);
        assert!((number(2) - 7.58).abs() < 0.01, "{}", cells[2]);
        assert_eq!(cells[3..], ["1", "12"]);
    }

    /// The POSIX second of a reading in Indian Standard Time.
    fn ist_unix(year: i64, month: u8, day: u8, minutes: i64) -> i64 {
        (ymd(year, month, day) - RD_OF_UNIX_EPOCH) * 86_400 + minutes * 60 - 19_800
    }

    /// Drik Panchang heads Vikrama 2081, 2082 and 2083, Śaka 1946 to 1948,
    /// Pingala, Kalayukta and Siddharthi, by the rule with the *bīja*; some
    /// of the Hindi press names them Kalayukta, Siddharthi and Raudra, by
    /// the rule without it (`drikpanchang-day-2024-2026`, `webdunia-samvat-2081`,
    /// `dainiktribune-samvat-2082`, `aajtak-samvat-2083`, as
    /// `hc-calendars-indic`'s tests read them). The rule with the *bīja*
    /// expunges Manmatha in Śaka 1864 and Durmati in 1949.
    #[test]
    fn the_northern_years_of_2024_to_2026_are_named_by_their_rules() {
        let year = |rule: &str, saka: i64| {
            let line = barhaspatya_year_line(rule, saka, "en").expect("in range");
            let cells: Vec<String> = cells(&line).into_iter().map(String::from).collect();
            assert_eq!(cells.len(), BARHASPATYA_YEAR_COLUMNS);
            cells
        };
        for (saka, bija, plain) in [
            (1_946, ("51", "Pingala"), ("52", "Kalayukta")),
            (1_947, ("52", "Kalayukta"), ("53", "Siddharthin")),
            (1_948, ("53", "Siddharthin"), ("54", "Raudra")),
        ] {
            assert_eq!(year("surya-siddhanta-bija", saka)[..2], [bija.0, bija.1]);
            assert_eq!(year("Surya-Siddhanta", saka)[..2], [plain.0, plain.1]);
        }
        assert_eq!(year("surya-siddhanta-bija", 1_948)[2..], ["", "", "en"]);
        let expunged = year("surya-siddhanta-bija", 1_949);
        assert_eq!(expunged[2], "55");
        assert_eq!(
            Some(expunged[3].as_str()),
            hc_calendars_indic::samvatsara::name(55)
        );
        assert_eq!(year("surya-siddhanta-bija", 1_864)[2], "29");
        assert_eq!(
            barhaspatya_year_line("bija", 1_946, "en"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            barhaspatya_year_line("arya-siddhanta", 6_822, "en"),
            Err(Refusal::OutOfRange)
        );
        assert!(barhaspatya_year_line("arya-siddhanta", -3_178, "en").is_ok());
    }

    /// Drik Panchang ends Pingala at 14:14 IST on 29 April 2024, and the
    /// rule with the *bīja* about two hours later; the rule without it has
    /// Siddharthi from 15 March 2025, in progress at Chaitra śukla 1, 30
    /// March 2025 (`drikpanchang-day-2024-2026`, `shivshakti-samvat-2082`).
    #[test]
    fn the_name_in_progress_follows_the_rule_within_the_year() {
        let at = |rule: &str, unix: i64| {
            let line = barhaspatya_year_at_line(rule, unix, "en").expect("in range");
            let cells: Vec<String> = cells(&line).into_iter().map(String::from).collect();
            assert_eq!(cells.len(), BARHASPATYA_AT_COLUMNS);
            cells
        };
        let pingala_ends = ist_unix(2024, 4, 29, 14 * 60 + 14);
        assert_eq!(
            at("surya-siddhanta-bija", pingala_ends + 115 * 60),
            ["51", "Pingala", "en"]
        );
        assert_eq!(at("surya-siddhanta-bija", pingala_ends + 144 * 60)[0], "52");
        assert_eq!(at("surya-siddhanta", ist_unix(2025, 3, 30, 360))[0], "53");
        assert_eq!(at("surya-siddhanta", ist_unix(2025, 3, 15, 0))[0], "52");
        assert_eq!(
            barhaspatya_year_at_line("surya-siddhanta", i64::MIN, "en"),
            Err(Refusal::OutOfRange)
        );
    }
}
