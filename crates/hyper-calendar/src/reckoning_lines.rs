//! The tab-separated lines the WebAssembly module and the C library write
//! about the other reckonings of a day that the calendars mark, written
//! once: each line's terms named in a locale by
//! [`hc_i18n::reckonings`], with the tag of the data that named them.
//!
//! * The choghadiya, the eighths of a day's daylight and of the night after
//!   it, each of a kind the weekday sets
//!   ([`hc_calendars_indic::choghadiya`]): [`choghadiya_lines`].
//! * The Panchak window in progress at an instant, or the next, and the
//!   kind a naming table gives it by the weekday it opens on
//!   ([`hc_calendars_indic::panchak`]): [`panchak_line`].
//! * Whether a year's sky meets a condition of the Kumbh Mela, and the
//!   twelve days of the *Ādi Pushkaram* of each river of a sign
//!   ([`hc_calendars_indic::kumbh`], [`hc_calendars_indic::pushkaram`]).
//!   The library has no ephemeris of Jupiter, so Jupiter's sidereal sign
//!   and the moment it enters one are the caller's: [`kumbh_line`],
//!   [`pushkaram_lines`].
//! * The folk days of a day — the counts of the Chinese first month
//!   ([`mod@hc_almanac::first_month_counts`]), 入梅 and 出梅
//!   ([`hc_seasons::meiyu`]), Tam Nương and Nguyệt Kỵ
//!   ([`hc_almanac::vietnamese_days`]) and the Turkish folk year of Hızır
//!   and Kasım ([`hc_seasons::hizir_kasim`]): [`folk_day_lines`].
//! * The Chinese night watch of a time of the civil clock, by the fixed
//!   reckoning ([`hc_format::night_watches`]): [`night_watch_line`].
//!
//! The days and instants answer for the sky layer's era,
//! [`crate::astro_lines`], as [`crate::panchanga_lines::kalam_lines`]'s do.
//! A sidereal sign is named by the lower-case ASCII form of its Sanskrit
//! name, [`SIGN_IDS`], as the Kumbh conditions' identifiers spell it.

use alloc::string::String;
use core::fmt::Write;

use hc_almanac::first_month_counts::first_month_counts;
use hc_almanac::vietnamese_days::{is_nguyet_ky_day_number, is_tam_nuong_day_number};
use hc_astro::riseset::Location;
use hc_calendar::gregorian::year_from_fixed;
use hc_calendar::{CivilTime, Rd, Weekday};
use hc_calendars_indic::choghadiya::{self, Choghadiya, Half as ChoghadiyaHalf, Quality};
use hc_calendars_indic::kumbh::KumbhYoga;
use hc_calendars_indic::panchak::{self, PanchakNaming};
use hc_calendars_indic::pushkaram::{adi_pushkaram, rivers_of};
use hc_calendars_lunar::{chinese, vietnamese};
use hc_format::night_watches::fixed_night_watch;
use hc_i18n::reckonings::{
    CHOGHADIYA, FIRST_MONTH_COUNT, FOLK_HALF, FOLK_NAMED_DAY, KUMBH_SITE, NIGHT_WATCH, PANCHAK,
    PLUM_RAINS, PUSHKARAM_RIVER, VIETNAMESE_DAY,
};
use hc_seasons::hizir_kasim::{self, Half as FolkHalf, NamedDay};
use hc_seasons::zodiac::{RulingPlanet, SiderealSign};

use crate::astro_lines::{
    EARLIEST_YEAR, LATEST_YEAR, MISSING_COLUMNS, day_in_era, moment_in_era, push_missing_cells,
    unix_from_moment,
};
use crate::boundary::{Answer, Refusal, names, push_cell, push_reckoning_name};
use crate::panchanga_lines::ayanamsa;
use crate::season_lines::{PLUM_RAIN_RULES, meridian};

/// The identifier of each sidereal sign, Meṣa first: the lower-case ASCII
/// form of its Sanskrit name.
pub const SIGN_IDS: [&str; 12] = [
    "mesha",
    "vrishabha",
    "mithuna",
    "karka",
    "simha",
    "kanya",
    "tula",
    "vrishchika",
    "dhanus",
    "makara",
    "kumbha",
    "mina",
];

/// The identifier of a sidereal sign.
#[must_use]
pub fn sign_id(sign: SiderealSign) -> &'static str {
    SIGN_IDS[usize::from(sign.index())]
}

/// The sidereal sign an identifier of [`SIGN_IDS`] names, in any case.
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other name.
pub fn sign(name: &str) -> Answer<SiderealSign> {
    SiderealSign::ALL
        .into_iter()
        .find(|sign| names(name, sign_id(*sign)))
        .ok_or(Refusal::Unknown)
}

/// A quality of the choghadiya as the word a line carries.
const fn quality_name(quality: Quality) -> &'static str {
    match quality {
        Quality::Auspicious => "auspicious",
        Quality::Neutral => "neutral",
        Quality::Inauspicious => "inauspicious",
    }
}

/// The identifier of the planet a choghadiya kind is ruled by, which the
/// kind names in English.
fn ruler_id(kind: &Choghadiya) -> &'static str {
    RulingPlanet::CHALDEAN_ORDER
        .iter()
        .find(|planet| planet.english_name() == kind.ruler)
        .map_or("", |planet| planet.id)
}

/// How many columns each line of [`choghadiya_lines`] writes.
pub const CHOGHADIYA_COLUMNS: usize = 9 + MISSING_COLUMNS;

/// The lines of `hc_choghadiya`: the sixteen choghadiya of a day at a
/// place, the eight parts of its daylight, sunrise to sunset, and the
/// eight of the night after it, sunset to the next sunrise, in order.
///
/// Each line: the half, `day` or `night`; the part, 1 to 8; the kind's
/// identifier, `udvega` to `roga`; its name in the locale and the tag of
/// the data that named it, by [`hc_i18n::reckonings::name_or_fallback`];
/// its quality, `auspicious`, `neutral` or `inauspicious`; the planet that
/// rules it, `sun` to `saturn`; the start and the end, as whole POSIX
/// seconds of Universal Time, rounded down; then the four cells of a
/// missing solar event, as [`crate::panchanga_lines::kalam_lines`] writes
/// them. Where the Sun does not rise or set, the half's eight lines keep
/// their kinds, which the weekday alone sets, leave the start and end
/// empty and name the missing event. A night takes the weekday of the
/// sunset that begins it.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a day outside the sky layer's era.
pub fn choghadiya_lines(fixed: i64, place: Location, locale: &str) -> Answer<String> {
    let day = day_in_era(fixed)?;
    let weekday = Weekday::from_rd(day);
    let mut out = String::new();
    for (half, name, periods) in [
        (ChoghadiyaHalf::Day, "day", choghadiya::day(day, place)),
        (
            ChoghadiyaHalf::Night,
            "night",
            choghadiya::night(day, place),
        ),
    ] {
        for part in 1..=8u8 {
            let kind = choghadiya::kind_of(weekday, half, part);
            let _ = write!(out, "{name}\t{part}\t{}\t", kind.id);
            push_reckoning_name(&mut out, locale, CHOGHADIYA, kind.id);
            let _ = write!(
                out,
                "\t{}\t{}\t",
                quality_name(kind.quality),
                ruler_id(&kind)
            );
            match &periods {
                Ok(parts) => {
                    let span = parts[usize::from(part - 1)].span;
                    let _ = write!(
                        out,
                        "{}\t{}\t",
                        unix_from_moment(span.start),
                        unix_from_moment(span.end)
                    );
                    push_missing_cells(&mut out, None);
                }
                Err(missing) => {
                    out.push_str("\t\t");
                    push_missing_cells(&mut out, Some(*missing));
                }
            }
            out.push('\n');
        }
    }
    Ok(out)
}

/// Seconds in a day.
const SECONDS_PER_DAY: i64 = 86_400;

/// How many columns [`panchak_line`] writes.
pub const PANCHAK_COLUMNS: usize = 7;

/// The line of `hc_panchak`: the Panchak window in progress at a Universal
/// Time instant, or the next one when the Moon is outside the arc then,
/// and its kind under a naming table of
/// [`PanchakNaming::ALL`] selected by its identifier in any case.
///
/// The cells: `1` when the instant is within the window, else `0`; the
/// moments the Moon's sidereal longitude, in the zodiac of the ayanāṃśa
/// [`ayanamsa`] names, reaches 300° and 360°, as whole POSIX seconds of
/// Universal Time, rounded down; the weekday the window opens on, Monday
/// 1 to Sunday 7, on a clock `offset_seconds` ahead of Universal Time,
/// midnight to midnight; the kind the table gives that weekday, as the
/// lower case of its name (`rog`, `raj`, `agni`, `chor` or `mrityu`), and
/// its name in the locale and the tag that named it, all three empty on a
/// weekday the table names no kind for.
///
/// The sources name a window by "the weekday it begins on" and do not
/// say whether that day runs from midnight or from sunrise, so the clock
/// is the caller's: a reckoning from sunrise gives an opening between
/// midnight and sunrise the weekday before, and the caller asks for it by
/// that weekday's kind.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a table or an ayanāṃśa not named;
/// [`Refusal::OutOfRange`] for an offset of a day or more either way, and
/// for an instant outside the sky layer's era.
pub fn panchak_line(
    naming: &str,
    universal_unix: i64,
    ayanamsa_name: &str,
    offset_seconds: i32,
    locale: &str,
) -> Answer<String> {
    let naming = PanchakNaming::ALL
        .iter()
        .find(|known| names(naming, known.id))
        .ok_or(Refusal::Unknown)?;
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let offset = i64::from(offset_seconds);
    if offset.abs() >= SECONDS_PER_DAY {
        return Err(Refusal::OutOfRange);
    }
    let moment = moment_in_era(universal_unix)?;
    let window = panchak::window(moment, ayanamsa);
    let (opens, closes) = (
        unix_from_moment(window.opens),
        unix_from_moment(window.closes),
    );
    let local_day = Rd::from_unix_days((opens + offset).div_euclid(SECONDS_PER_DAY));
    let weekday = Weekday::from_rd(local_day);
    let mut out = String::new();
    let _ = write!(
        out,
        "{}\t{opens}\t{closes}\t{}\t",
        u8::from(panchak::is_panchak(moment, ayanamsa)),
        weekday.iso_number()
    );
    if let Some(kind) = naming.kind(weekday) {
        push_cell(&mut out, &kind.to_ascii_lowercase());
        out.push('\t');
        push_reckoning_name(&mut out, locale, PANCHAK, kind);
    } else {
        out.push_str("\t\t");
    }
    out.push('\n');
    Ok(out)
}

/// How many columns [`kumbh_line`] writes.
pub const KUMBH_COLUMNS: usize = 11;

/// The line of `hc_kumbh`: when in a Gregorian year the Sun, and the Moon
/// where the condition asks for it, stand as a condition of the Kumbh
/// Mela of [`KumbhYoga::ALL`] requires, selected by its identifier in any
/// case, and whether Jupiter's sign, which the caller gives, meets it.
///
/// The library has no ephemeris of Jupiter, so `jupiter` is the caller's:
/// the sidereal sign, by an identifier of [`SIGN_IDS`], that Jupiter is in
/// at the occasion's first moment, in the zodiac of the same ayanāṃśa. It
/// is read at that moment because the Sun's stay can hold a change of
/// Jupiter's sign; the first moment is in this same line whatever `jupiter`
/// is, so a caller asks once with it empty and again with the sign it
/// reads there.
///
/// The cells: the condition's identifier; its site's identifier, the lower
/// case of the site's English name (`haridwar`, `prayag`, `nashik`,
/// `ujjain`), and the site's name in the locale and the tag that named it;
/// the river the site stands on, in English as the source gives it; the
/// signs Jupiter and the Sun must be in; `1` when the Moon must be with the
/// Sun at the new moon as well, else `0`; the occasion's first and last
/// moments — the Sun's entry into its sign and into the next, or the new
/// moon twice — as whole POSIX seconds of Universal Time, rounded down,
/// empty for a new-moon condition whose stay holds no new moon that year;
/// and `1` when there is an occasion and `jupiter` is the condition's
/// sign, `0` when not, empty when `jupiter` is empty.
///
/// The dates of each festival are fixed and announced by the government of
/// the state that holds it: the line says whether a year's sky meets a
/// site's condition, not when the bathing days are.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a condition, an ayanāṃśa or a sign not named;
/// [`Refusal::OutOfRange`] for a year outside the sky layer's era.
pub fn kumbh_line(
    yoga: &str,
    year: i64,
    ayanamsa_name: &str,
    jupiter: &str,
    locale: &str,
) -> Answer<String> {
    let yoga = KumbhYoga::ALL
        .iter()
        .find(|known| names(yoga, known.id))
        .ok_or(Refusal::Unknown)?;
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let jupiter = if jupiter.trim().is_empty() {
        None
    } else {
        Some(sign(jupiter)?)
    };
    if !(EARLIEST_YEAR..=LATEST_YEAR).contains(&year) {
        return Err(Refusal::OutOfRange);
    }
    let site = yoga.site.to_ascii_lowercase();
    let mut out = String::new();
    push_cell(&mut out, yoga.id);
    out.push('\t');
    push_cell(&mut out, &site);
    out.push('\t');
    push_reckoning_name(&mut out, locale, KUMBH_SITE, &site);
    out.push('\t');
    push_cell(&mut out, yoga.river);
    let _ = write!(
        out,
        "\t{}\t{}\t{}\t",
        sign_id(yoga.jupiter),
        sign_id(yoga.sun),
        u8::from(yoga.at_new_moon)
    );
    let occasion = yoga.occasion(year, ayanamsa);
    if let Some(occasion) = occasion {
        let _ = write!(
            out,
            "{}\t{}\t",
            unix_from_moment(occasion.from),
            unix_from_moment(occasion.to)
        );
    } else {
        out.push_str("\t\t");
    }
    if let Some(jupiter) = jupiter {
        out.push(if occasion.is_some() && jupiter == yoga.jupiter {
            '1'
        } else {
            '0'
        });
    }
    out.push('\n');
    Ok(out)
}

/// How many columns each line of [`pushkaram_lines`] writes.
pub const PUSHKARAM_COLUMNS: usize = 7 + MISSING_COLUMNS;

/// The lines of `hc_pushkaram`: the twelve days of the *Ādi Pushkaram* of
/// each river of a sidereal sign, for Jupiter's entry into the sign at a
/// Universal Time instant, one line a river in `hc-calendars-indic`'s
/// order.
///
/// The library has no ephemeris of Jupiter, so the sign, by an identifier
/// of [`SIGN_IDS`], and the moment of the entry are the caller's; where
/// Jupiter enters, turns back and enters again, the second entry is the
/// one the festival follows. The first day is the civil day of the entry
/// at `meridian`, read as for `hc_term_in_effect`, or the next day when
/// the entry falls after that day's sunset at the place: a reading fitted
/// to the festivals whose dates were read, which no source read states.
///
/// Each line: the river's identifier, `pushkaram-ganga` to
/// `pushkaram-pranahita`; its name in the locale and the tag that named
/// it; the region the source keeps it in for the sign, in English, empty
/// where it names none; the sign's identifier; the first and last days, as
/// fixed days; then the four cells of a missing solar event, which name
/// the sunset where the Sun does not set on the day of the entry and the
/// two days are empty.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a sign or a meridian not named;
/// [`Refusal::OutOfRange`] for an instant outside the sky layer's era.
pub fn pushkaram_lines(
    sign_name: &str,
    entry_unix: i64,
    place: Location,
    meridian_name: &str,
    locale: &str,
) -> Answer<String> {
    let sign = sign(sign_name)?;
    let meridian = meridian(meridian_name)?;
    let entry = moment_in_era(entry_unix)?;
    let span = adi_pushkaram(entry, place, meridian);
    let mut out = String::new();
    for river in rivers_of(sign) {
        push_cell(&mut out, river.id);
        out.push('\t');
        push_reckoning_name(&mut out, locale, PUSHKARAM_RIVER, river.id);
        out.push('\t');
        push_cell(&mut out, river.region);
        let _ = write!(out, "\t{}\t", sign_id(sign));
        match span {
            Ok(days) => {
                let _ = write!(out, "{}\t{}\t", days.first.0, days.last.0);
                push_missing_cells(&mut out, None);
            }
            Err(missing) => {
                out.push_str("\t\t");
                push_missing_cells(&mut out, Some(missing));
            }
        }
        out.push('\n');
    }
    Ok(out)
}

/// The identifier of a half of the Turkish folk year.
const fn folk_half_id(half: FolkHalf) -> &'static str {
    match half {
        FolkHalf::Hizir => "hizir",
        FolkHalf::Kasim => "kasim",
    }
}

/// The identifier of a named day of the Turkish folk year.
#[must_use]
pub const fn folk_named_day_id(day: NamedDay) -> &'static str {
    match day {
        NamedDay::Hidirellez => "hidirellez",
        NamedDay::Kasim => "kasim",
        NamedDay::Erbain => "erbain",
        NamedDay::Hamsin => "hamsin",
        NamedDay::CemreAir => "cemre-air",
        NamedDay::CemreWater => "cemre-water",
        NamedDay::CemreEarth => "cemre-earth",
    }
}

/// The Chinese year beginning in 2024 is 4661, so the Chinese year that
/// begins in a Gregorian year is that year plus this.
const CHINESE_YEAR_OFFSET: i64 = 2_637;

/// The Gregorian year in which the Chinese year a day is in began, or
/// `None` outside the years `chinese` converts.
fn chinese_year_begun(day: Rd) -> Option<i64> {
    let year = year_from_fixed(day);
    let new_year = chinese::new_year(year + CHINESE_YEAR_OFFSET).ok()?;
    Some(if day.0 < new_year.0 { year - 1 } else { year })
}

/// The identifiers of the four counts of the first month, in the order
/// [`folk_day_lines`] writes them: 几龙治水, 几牛耕田, 几日得辛, 几人分饼.
pub const FIRST_MONTH_COUNT_IDS: [&str; 4] = ["dragons", "oxen", "xin", "cakes"];

/// How many columns each line of [`folk_day_lines`] writes.
pub const FOLK_DAY_COLUMNS: usize = 5;

/// One line of [`folk_day_lines`].
fn push_folk(out: &mut String, kind: &str, id: &str, locale: &str, count: Option<i64>) {
    push_cell(out, kind);
    out.push('\t');
    push_cell(out, id);
    out.push('\t');
    push_reckoning_name(out, locale, kind, id);
    out.push('\t');
    if let Some(count) = count {
        let _ = write!(out, "{count}");
    }
    out.push('\n');
}

/// The lines of `hc_folk_day`: the folk reckonings of a day outside the
/// Japanese almanac, one a line, each named in a locale, in this order:
///
/// * `first-month-count`, four lines, `dragons`, `oxen`, `xin` and
///   `cakes` — 几龙治水, 几牛耕田, 几日得辛 and 几人分饼 — with the count of
///   the Chinese year the day is in: the day of 正月 on which the first 辰,
///   丑, 辛 and 丙 day falls, 1 to 12 or 1 to 10; none outside the years
///   `chinese` converts, 1645 to 2150.
/// * `plum-rains`, a line for each rule by which the day is 入梅 or 出梅
///   with its solar term at the meridian: `ru-mei-bing`, `ru-mei-ren`,
///   `chu-mei-wei`; no count.
/// * `vietnamese-day`, `tam-nuong` or `nguyet-ky` when the day's number in
///   the Vietnamese lunar calendar is one of those days, with that number;
///   none outside the years `vietnamese` converts.
/// * `folk-half`, the half of the Turkish folk year the day is in, `hizir`
///   from 6 May or `kasim` from 8 November, with the day's count in it
///   from 1; always.
/// * `folk-named-day`, when the day is one of the named days of the folk
///   year, `hidirellez`, `kasim`, `erbain`, `hamsin`, `cemre-air`,
///   `cemre-water` or `cemre-earth`, with its count in its half.
///
/// Each line: the kind; the identifier; the name in the locale and the tag
/// of the data that named it, by
/// [`hc_i18n::reckonings::name_or_fallback`] — the locale's, else
/// English's, else the kind's own language's, which is first under
/// `native`; and the count, empty where the kind has none.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a meridian [`meridian`] does not read, and
/// [`Refusal::OutOfRange`] for a day outside the sky layer's era.
pub fn folk_day_lines(fixed: i64, meridian_name: &str, locale: &str) -> Answer<String> {
    let meridian = meridian(meridian_name)?;
    let day = day_in_era(fixed)?;
    let mut out = String::new();
    if let Some(counts) = chinese_year_begun(day).and_then(first_month_counts) {
        let values = [counts.dragons, counts.oxen, counts.xin, counts.cakes];
        for (id, count) in FIRST_MONTH_COUNT_IDS.into_iter().zip(values) {
            push_folk(
                &mut out,
                FIRST_MONTH_COUNT,
                id,
                locale,
                Some(i64::from(count)),
            );
        }
    }
    let year = year_from_fixed(day);
    for (id, day_of) in PLUM_RAIN_RULES {
        if day_of(year, meridian) == day {
            push_folk(&mut out, PLUM_RAINS, id, locale, None);
        }
    }
    if let Ok((_, _, lunar_day)) = vietnamese::PARAMETERS.from_fixed(day) {
        for (id, falls) in [
            ("tam-nuong", is_tam_nuong_day_number(lunar_day)),
            ("nguyet-ky", is_nguyet_ky_day_number(lunar_day)),
        ] {
            if falls {
                push_folk(
                    &mut out,
                    VIETNAMESE_DAY,
                    id,
                    locale,
                    Some(i64::from(lunar_day)),
                );
            }
        }
    }
    let folk = hizir_kasim::folk_day(day);
    let count = Some(i64::from(folk.day));
    push_folk(&mut out, FOLK_HALF, folk_half_id(folk.half), locale, count);
    if let Some(named) = NamedDay::ALL
        .into_iter()
        .find(|named| named.folk_day() == folk)
    {
        push_folk(
            &mut out,
            FOLK_NAMED_DAY,
            folk_named_day_id(named),
            locale,
            count,
        );
    }
    Ok(out)
}

/// The identifiers of the five watches in [`hc_i18n::reckonings`].
const WATCH_IDS: [&str; 5] = ["1", "2", "3", "4", "5"];

/// How many columns [`night_watch_line`] writes.
pub const NIGHT_WATCH_COLUMNS: usize = 6;

/// The line of `hc_night_watch`: the Chinese night watch, 更, and its
/// points, 點, of a time of the civil clock, as whole seconds after
/// midnight, by the fixed reckoning, 19:00 to 05:00 in five watches of two
/// hours and each watch in five points of 24 minutes — or no line at all
/// from 05:00 to 18:59, which is not night.
///
/// The cells: the watch, 1 (一更) to 5 (五更); the points struck since it
/// began, 0 to 4; the watch's name in the locale and the tag that named
/// it, by [`hc_i18n::reckonings::name_or_fallback`]; and its Han name,
/// 黃昏 to 平旦, and its double hour, 戌 to 寅, in Chinese as the source
/// writes them.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a time of day from 86 400 s.
pub fn night_watch_line(seconds_of_day: u32, locale: &str) -> Answer<String> {
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
    let mut out = String::new();
    let Some(watch) = fixed_night_watch(civil) else {
        return Ok(out);
    };
    let _ = write!(out, "{}\t{}\t", watch.watch, watch.points);
    let id = WATCH_IDS[usize::from(watch.watch - 1)];
    push_reckoning_name(&mut out, locale, NIGHT_WATCH, id);
    out.push('\t');
    push_cell(&mut out, watch.han_name());
    out.push('\t');
    push_cell(&mut out, watch.branch());
    out.push('\n');
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;
    use hc_calendar::fixed::RD_OF_UNIX_EPOCH;

    fn ymd(year: i64, month: u8, day: u8) -> i64 {
        hc_calendars_solar::gregorian::to_fixed(year, month, day)
            .expect("a date")
            .0
    }

    /// The POSIX second of a reading in Indian Standard Time.
    fn ist(year: i64, month: u8, day: u8, hour: i64, minute: i64) -> i64 {
        (ymd(year, month, day) - RD_OF_UNIX_EPOCH) * 86_400 + hour * 3_600 + minute * 60 - 19_800
    }

    fn rows(text: &str, columns: usize) -> Vec<Vec<&str>> {
        text.lines()
            .map(|line| {
                let cells: Vec<&str> = line.split('\t').collect();
                assert_eq!(cells.len(), columns, "{line}");
                cells
            })
            .collect()
    }

    /// New Delhi, as Drik Panchang's pages place it.
    const NEW_DELHI: Location = Location::new(
        28.0 + 38.0 / 60.0 + 8.0 / 3_600.0,
        77.0 + 13.0 / 60.0 + 28.0 / 3_600.0,
        0.0,
    );

    /// Drik Panchang's New Delhi page of Wednesday 1 January 2025
    /// (`drik-choghadiya-2025`, as `docs/systems/choghadiya.md` works it):
    /// the day opens with Labha at 07:14 IST, ruled by Mercury, and its
    /// fifth part is Roga from 12:25; the night opens with Udvega at 17:36
    /// and its second part is Shubha from 19:18. Within a minute each.
    #[test]
    fn the_first_of_january_2025_has_drik_panchangs_choghadiya() {
        let text = choghadiya_lines(ymd(2025, 1, 1), NEW_DELHI, "en").expect("in range");
        let day = rows(&text, CHOGHADIYA_COLUMNS);
        assert_eq!(day.len(), 16);
        for (row, (half, part, id, quality, ruler, (hour, minute))) in [
            (0, ("day", "1", "labha", "auspicious", "mercury", (7, 14))),
            (4, ("day", "5", "roga", "inauspicious", "mars", (12, 25))),
            (8, ("night", "1", "udvega", "inauspicious", "sun", (17, 36))),
            (
                9,
                ("night", "2", "shubha", "auspicious", "jupiter", (19, 18)),
            ),
        ] {
            let cells = &day[row];
            assert_eq!(
                cells[..3]
                    .iter()
                    .chain(&cells[5..7])
                    .copied()
                    .collect::<Vec<_>>(),
                [half, part, id, quality, ruler]
            );
            assert_eq!(cells[4], "en");
            let start: i64 = cells[7].parse().expect("an instant");
            let off = (start - ist(2025, 1, 1, hour, minute)) as f64 / 60.0;
            assert!(off.abs() < 1.0, "{id}: {off} min");
            assert_eq!(cells[9..], ["", "", "", ""]);
        }
        assert_eq!(day[0][3], "Labha");
        // The eighth part repeats the first, and the night ends at the next
        // sunrise, where the next day's first part begins.
        assert_eq!(day[7][2], "labha");
        let next = choghadiya_lines(ymd(2025, 1, 2), NEW_DELHI, "native").expect("in range");
        assert_eq!(day[15][8], rows(&next, CHOGHADIYA_COLUMNS)[0][7]);
        // Above the Arctic Circle at midwinter the day's parts have no
        // times, and their kinds are still the weekday's: a Saturday's day
        // opens with Kala.
        let tromso = Location::new(69.6496, 18.9560, 0.0);
        let polar = choghadiya_lines(ymd(2024, 12, 21), tromso, "en").expect("in range");
        let polar = rows(&polar, CHOGHADIYA_COLUMNS);
        assert_eq!(polar[0][7..10], ["", "", "sunrise"]);
        assert_eq!(polar[0][2], "kala");
        assert_eq!(
            choghadiya_lines(i64::MAX, NEW_DELHI, "en"),
            Err(Refusal::OutOfRange)
        );
    }

    /// Drik Panchang's windows of 2025 for New Delhi (`drik-panchak`, as
    /// `docs/systems/panchak.md` reads them): the first opens on Friday
    /// 3 January at 10:47 IST and closes on Tuesday 7 January at 17:50,
    /// Chor Panchak under both tables; the last opens on Wednesday
    /// 24 December at 19:46, Raj Panchak to India TV and no kind in the
    /// five-kind table. The window of 23 April 2025 opens at 00:31 IST,
    /// a Wednesday by the Indian clock and a Tuesday, Agni, by UTC's.
    #[test]
    fn the_windows_of_2025_take_their_kinds_by_the_weekday_they_open_on() {
        let within = ist(2025, 1, 5, 12, 0);
        let line =
            panchak_line("panchak-five-kinds", within, "lahiri", 19_800, "en").expect("in range");
        let cells = rows(&line, PANCHAK_COLUMNS).remove(0);
        assert_eq!(cells[0], "1");
        let opens: i64 = cells[1].parse().expect("an instant");
        let closes: i64 = cells[2].parse().expect("an instant");
        assert!((opens - ist(2025, 1, 3, 10, 47)).abs() < 90, "{opens}");
        assert!((closes - ist(2025, 1, 7, 17, 50)).abs() < 90, "{closes}");
        assert_eq!(cells[3..], ["5", "chor", "Chor", "en"]);
        // Before it, the same window is the next one.
        let before = panchak_line(
            "panchak-five-kinds",
            ist(2025, 1, 1, 0, 0),
            "Lahiri",
            19_800,
            "en",
        )
        .expect("in range");
        assert_eq!(
            rows(&before, PANCHAK_COLUMNS)[0][..3],
            ["0", cells[1], cells[2]]
        );

        let december = ist(2025, 12, 24, 12, 0);
        let raj = panchak_line("PANCHAK-RAJ-MIDWEEK", december, "lahiri", 19_800, "native")
            .expect("in range");
        assert_eq!(
            rows(&raj, PANCHAK_COLUMNS)[0][3..],
            ["3", "raj", "Raj", "en"]
        );
        let none =
            panchak_line("panchak-five-kinds", december, "lahiri", 19_800, "en").expect("in range");
        assert_eq!(rows(&none, PANCHAK_COLUMNS)[0][3..], ["3", "", "", ""]);

        let april = ist(2025, 4, 22, 12, 0);
        let by_ist =
            panchak_line("panchak-five-kinds", april, "lahiri", 19_800, "en").expect("in range");
        assert_eq!(rows(&by_ist, PANCHAK_COLUMNS)[0][3..5], ["3", ""]);
        let by_utc =
            panchak_line("panchak-five-kinds", april, "lahiri", 0, "en").expect("in range");
        assert_eq!(rows(&by_utc, PANCHAK_COLUMNS)[0][3..5], ["2", "agni"]);

        assert_eq!(
            panchak_line("panchak", within, "lahiri", 0, "en"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            panchak_line("panchak-five-kinds", within, "", 0, "en"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            panchak_line("panchak-five-kinds", within, "lahiri", 86_400, "en"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            panchak_line("panchak-five-kinds", i64::MIN, "lahiri", 0, "en"),
            Err(Refusal::OutOfRange)
        );
    }

    /// The Maha Kumbh of 2025 (`wikipedia-kumbh-mela`, as
    /// `hc-calendars-indic`'s test reads it): Prayag's Vṛṣabha condition,
    /// the Sun's stay in Makara from the morning of 14 January IST to
    /// 12 February, with Jupiter in Vṛṣabha by Drik Panchang's ingresses.
    #[test]
    fn the_maha_kumbh_of_2025_meets_prayags_condition() {
        let line = kumbh_line("kumbh-prayag-vrishabha", 2025, "lahiri", "vrishabha", "en")
            .expect("in range");
        let cells = rows(&line, KUMBH_COLUMNS).remove(0);
        assert_eq!(
            cells[..8],
            [
                "kumbh-prayag-vrishabha",
                "prayag",
                "Prayag",
                "en",
                "Ganga and Yamuna",
                "vrishabha",
                "makara",
                "0"
            ]
        );
        let from: i64 = cells[8].parse().expect("an instant");
        let to: i64 = cells[9].parse().expect("an instant");
        assert!(ist(2025, 1, 14, 6, 0) < from && from < ist(2025, 1, 14, 12, 0));
        assert!(ist(2025, 2, 12, 0, 0) < to && to < ist(2025, 2, 13, 0, 0));
        assert_eq!(cells[10], "1");
        let other =
            kumbh_line("kumbh-prayag-vrishabha", 2025, "lahiri", "MESHA", "en").expect("in range");
        assert_eq!(rows(&other, KUMBH_COLUMNS)[0][10], "0");
        let unknown =
            kumbh_line("kumbh-prayag-vrishabha", 2025, "lahiri", "", "en").expect("in range");
        assert_eq!(
            rows(&unknown, KUMBH_COLUMNS)[0][8..],
            [cells[8], cells[9], ""]
        );
        // A new-moon condition's first and last moments are the new moon.
        let tula = kumbh_line("kumbh-ujjain-tula", 2017, "lahiri", "tula", "en").expect("in range");
        let tula = rows(&tula, KUMBH_COLUMNS).remove(0);
        assert_eq!((tula[7], tula[10]), ("1", "1"));
        assert_eq!(tula[8], tula[9]);
        assert_eq!(
            kumbh_line("kumbh", 2025, "lahiri", "", "en"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            kumbh_line("kumbh-haridwar", 2025, "lahiri", "aries", "en"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            kumbh_line("kumbh-haridwar", 3001, "lahiri", "", "en"),
            Err(Refusal::OutOfRange)
        );
    }

    /// The Godavari Pushkaram of 2015, 14 to 25 July, for Jupiter's entry
    /// into Siṃha at 07:07 IST on 14 July (`wikipedia-godavari-pushkaram`
    /// and `drik-guru-gochar`, as `hc-calendars-indic`'s test reads them);
    /// and Vṛścika's two rivers, each with its region.
    #[test]
    fn the_godavari_pushkaram_of_2015_is_its_twelve_days() {
        let entry = ist(2015, 7, 14, 7, 7);
        let text = pushkaram_lines("simha", entry, NEW_DELHI, "india", "en").expect("in range");
        let row = rows(&text, PUSHKARAM_COLUMNS).remove(0);
        let (first, last) = (ymd(2015, 7, 14).to_string(), ymd(2015, 7, 25).to_string());
        assert_eq!(
            row[..7],
            [
                "pushkaram-godavari",
                "Godavari",
                "en",
                "",
                "simha",
                &first,
                &last
            ]
        );
        assert_eq!(row[7..], ["", "", "", ""]);
        let scorpion =
            pushkaram_lines("vrishchika", entry, NEW_DELHI, "india", "en").expect("in range");
        let ids: Vec<(&str, &str)> = rows(&scorpion, PUSHKARAM_COLUMNS)
            .iter()
            .map(|row| (row[0], row[3]))
            .collect();
        assert_eq!(
            ids,
            [
                ("pushkaram-bhima", "Maharashtra, Karnataka, Telangana"),
                ("pushkaram-tamraparni", "Tamil Nadu")
            ]
        );
        let polar = Location::new(89.0, 0.0, 0.0);
        let dark = pushkaram_lines("simha", ist(2015, 12, 21, 12, 0), polar, "india", "en")
            .expect("in range");
        assert_eq!(rows(&dark, PUSHKARAM_COLUMNS)[0][5..8], ["", "", "sunset"]);
        assert_eq!(
            pushkaram_lines("leo", entry, NEW_DELHI, "india", "en"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            pushkaram_lines("simha", entry, NEW_DELHI, "mars", "en"),
            Err(Refusal::Unknown)
        );
    }

    fn folk(fixed: i64, meridian: &str, locale: &str) -> Vec<Vec<String>> {
        let text = folk_day_lines(fixed, meridian, locale).expect("in range");
        rows(&text, FOLK_DAY_COLUMNS)
            .into_iter()
            .map(|row| row.into_iter().map(String::from).collect())
            .collect()
    }

    fn kinds(rows: &[Vec<String>], kind: &str) -> Vec<(String, String, String, String)> {
        rows.iter()
            .filter(|row| row[0] == kind)
            .map(|row| {
                (
                    row[1].clone(),
                    row[2].clone(),
                    row[3].clone(),
                    row[4].clone(),
                )
            })
            .collect()
    }

    fn named(id: &str, name: &str, tag: &str, count: &str) -> (String, String, String, String) {
        (id.into(), name.into(), tag.into(), count.into())
    }

    /// 11 June 2026: 入梅 by South China's rule, 丙辰 (`qq-meiyu-2026`); in
    /// 2026 七龙治水, 四牛耕田, 十日得辛, 五人分饼 (`netease-2026-longzhishui`);
    /// and day 37 of the Hızır half, which began on 6 May.
    #[test]
    fn a_june_day_of_2026_carries_its_folk_reckonings() {
        let rows = folk(ymd(2026, 6, 11), "china", "en");
        assert_eq!(
            kinds(&rows, FIRST_MONTH_COUNT),
            [
                named("dragons", "几龙治水", "zh-Hans", "7"),
                named("oxen", "几牛耕田", "zh-Hans", "4"),
                named("xin", "几日得辛", "zh-Hans", "10"),
                named("cakes", "几人分饼", "zh-Hans", "5"),
            ]
        );
        assert_eq!(
            kinds(&rows, PLUM_RAINS),
            [named("ru-mei-bing", "入梅", "zh-Hans", "")]
        );
        assert_eq!(
            kinds(&rows, FOLK_HALF),
            [named("hizir", "Hızır günleri", "tr", "37")]
        );
        assert!(kinds(&rows, FOLK_NAMED_DAY).is_empty());
        // 出梅 of 2026 on 8 July, 癸未; and 6 June 2025, 丙午, and 12 June,
        // 壬子, the two rules of 2025 (`netease-meiyu-2025`, `qq-meiyu-2025`).
        let chu = folk(ymd(2026, 7, 8), "china", "en");
        assert_eq!(kinds(&chu, PLUM_RAINS)[0].0, "chu-mei-wei");
        assert_eq!(
            kinds(&folk(ymd(2025, 6, 6), "china", "en"), PLUM_RAINS)[0].0,
            "ru-mei-bing"
        );
        assert_eq!(
            kinds(&folk(ymd(2025, 6, 12), "china", "en"), PLUM_RAINS)[0].0,
            "ru-mei-ren"
        );
        // Before 17 February 2026, 正月初一, the counts are the year before's.
        let january = folk(ymd(2026, 1, 20), "china", "en");
        let of_2025 = first_month_counts(2025).expect("in range");
        assert_eq!(
            kinds(&january, FIRST_MONTH_COUNT)[0].3,
            of_2025.dragons.to_string()
        );
    }

    /// Monday 23 March 2026 is the 5th of the second lunar month, Nguyệt Kỵ
    /// (`baonghean-2026-03-23`); 1 January 2026 is on Lịch Ngày Tốt's list
    /// of Tam Nương days (`lichngaytot-tam-nuong-2026`); 20 February 2026
    /// is Kasım 105, the first *cemre* (`bilkent-cemre`); and 6 May 2026
    /// is Hıdırellez, Hızır 1 (`wikipedia-tr-hidirellez`).
    #[test]
    fn the_vietnamese_and_turkish_days_are_their_sources_days() {
        let march = folk(ymd(2026, 3, 23), "", "vi");
        assert_eq!(
            kinds(&march, VIETNAMESE_DAY),
            [named("nguyet-ky", "Nguyệt Kỵ", "vi", "5")]
        );
        let new_year = folk(ymd(2026, 1, 1), "", "en");
        assert_eq!(kinds(&new_year, VIETNAMESE_DAY)[0].0, "tam-nuong");
        let cemre = folk(ymd(2026, 2, 20), "", "tr-CY");
        assert_eq!(
            kinds(&cemre, FOLK_HALF),
            [named("kasim", "Kasım günleri", "tr", "105")]
        );
        assert_eq!(
            kinds(&cemre, FOLK_NAMED_DAY),
            [named("cemre-air", "birinci cemre", "tr", "105")]
        );
        let hidirellez = folk(ymd(2026, 5, 6), "", "native");
        assert_eq!(
            kinds(&hidirellez, FOLK_NAMED_DAY),
            [named("hidirellez", "Hıdırellez", "tr", "1")]
        );
        // Every day writes its half, and names every line.
        for offset in (0..366).step_by(5) {
            let rows = folk(ymd(2026, 1, 1) + offset, "china", "de");
            assert_eq!(kinds(&rows, FOLK_HALF).len(), 1);
            assert!(
                rows.iter()
                    .all(|row| !row[2].is_empty() && !row[3].is_empty())
            );
        }
        // Outside the Chinese and Vietnamese calendars' years only the
        // Turkish year is left.
        let old = folk(ymd(1500, 3, 1), "china", "en");
        assert!(old.iter().all(|row| row[0] == FOLK_HALF));
        assert_eq!(folk_day_lines(0, "mars", "en"), Err(Refusal::Unknown));
        assert_eq!(folk_day_lines(i64::MAX, "", "en"), Err(Refusal::OutOfRange));
    }

    /// "三更两点就是指子時两點，即夜间11点48分" (`wikipedia-zh-dian`); 三更
    /// is 子, 夜半 (`wikipedia-zh-geng`); noon is no watch.
    #[test]
    fn the_second_point_of_the_third_watch_is_struck_at_23_48() {
        let line = night_watch_line(23 * 3_600 + 48 * 60, "zh-TW").expect("a time");
        assert_eq!(
            rows(&line, NIGHT_WATCH_COLUMNS)[0],
            ["3", "2", "三更", "zh-Hant", "夜半", "子"]
        );
        let first = night_watch_line(19 * 3_600, "en").expect("a time");
        assert_eq!(
            rows(&first, NIGHT_WATCH_COLUMNS)[0][..4],
            ["1", "0", "一更", "zh-Hant"]
        );
        let last = night_watch_line(5 * 3_600 - 1, "en").expect("a time");
        assert_eq!(rows(&last, NIGHT_WATCH_COLUMNS)[0][..2], ["5", "4"]);
        assert_eq!(night_watch_line(12 * 3_600, "en"), Ok(String::new()));
        assert_eq!(night_watch_line(5 * 3_600, "en"), Ok(String::new()));
        assert_eq!(night_watch_line(86_400, "en"), Err(Refusal::OutOfRange));
    }

    #[test]
    fn a_sign_is_named_by_its_sanskrit_name() {
        for sign in SiderealSign::ALL {
            assert_eq!(super::sign(sign_id(sign)), Ok(sign));
        }
        assert_eq!(super::sign(" Kumbha "), Ok(SiderealSign::KUMBHA));
        assert_eq!(super::sign("aquarius"), Err(Refusal::Unknown));
    }
}
