//! The tab-separated lines the WebAssembly module and the C library write
//! about the religious and traditional hours of a day, written once.
//!
//! * The Islamic prayer times of a local day at a place by a named method
//!   of [`hc_astro::solar_time::PRAYER_METHODS`], and the methods
//!   themselves with their parameters and sources.
//! * The Jewish times in temporal hours, [`Zman::ALL`], by one of the
//!   three reckonings of the day [`hc_astro::solar_time`] carries, with the
//!   dawns and nightfalls beside them.
//! * The Edo 不定時法 reading of an instant at a place, and its inverse, by
//!   the 寛政暦's 明け六つ and 暮れ六つ.
//!
//! Every time is whole POSIX seconds of Universal Time, rounded down, and
//! a time that does not happen that day — the Sun does not reach an angle,
//! or does not rise or set — is a line naming what is missing in the four
//! cells of [`crate::astro_lines::missing_cells`], never a number.
//! The days and instants answer for the sky layer's era,
//! [`crate::astro_lines`].

use alloc::string::String;

use hc_astro::riseset::{Location, solar_noon, sunrise};
use hc_astro::solar_time::{
    self, EdoHour, EdoTime, IshaRule, MaghribRule, MidnightRule, MissingSolarEvent, PRAYER_METHODS,
    Zman,
};
use hc_calendar::fixed::Moment;

use crate::astro_lines::{
    MISSING_COLUMNS, day_in_era, missing_cells, moment_in_era, moment_or_missing,
};
use crate::boundary::{Answer, Line, Refusal};

/// How many columns each line of [`prayer_times_lines`] writes: the time's
/// identifier, the instant and the four cells of a missing solar event.
pub const PRAYER_TIME_COLUMNS: usize = 2 + MISSING_COLUMNS;

/// The times [`prayer_times_lines`] writes, one line each, in this order.
pub const PRAYER_TIMES: [&str; 8] = [
    "fajr",
    "sunrise",
    "zuhr",
    "asr-shafii",
    "asr-hanafi",
    "maghrib",
    "isha",
    "midnight",
];

/// The lines of `hc_prayer_times`: the Islamic prayer times of a local day
/// at a place by a method, one line each of [`PRAYER_TIMES`] — *fajr* at
/// the method's angle, sunrise, *ẓuhr* at the Sun's transit, *ʿaṣr* by the
/// Shafiʿi and by the Hanafi shadow rule, since the method does not choose
/// between them, *maghrib*, *ʿishāʾ* and the middle of the night that
/// begins that evening — each the time's identifier, the instant, and the
/// four cells of a missing solar event. `ramadan` chooses the Ramaḍān
/// interval of a method that fixes *ʿishāʾ* after *maghrib*; `hc-astro` has
/// no calendar, so the caller says whether the day is in Ramaḍān.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a method [`solar_time::prayer_method`] does
/// not find, the empty string included: the times move with the method,
/// so none is assumed; and [`Refusal::OutOfRange`] for a day outside the
/// sky layer's era.
pub fn prayer_times_lines(
    method: &str,
    fixed: i64,
    place: Location,
    ramadan: bool,
) -> Answer<String> {
    let method = &solar_time::prayer_method(method).ok_or(Refusal::Unknown)?;
    let day = day_in_era(fixed)?;
    let times: [Result<Moment, MissingSolarEvent>; 8] = [
        solar_time::fajr(day, place, method),
        sunrise(day, place).ok_or(MissingSolarEvent::Sunrise(day)),
        Ok(solar_noon(day, place)),
        solar_time::asr_shafii(day, place),
        solar_time::asr_hanafi(day, place),
        solar_time::maghrib(day, place, method),
        solar_time::isha(day, place, method, ramadan),
        solar_time::islamic_midnight(day, place, method),
    ];
    let mut out = String::new();
    for (id, time) in PRAYER_TIMES.into_iter().zip(times) {
        let mut line = Line::new(&mut out);
        line.cell(id);
        moment_or_missing(&mut line, time);
        line.end();
    }
    Ok(out)
}

/// How many columns each line of [`prayer_methods_lines`] writes.
pub const PRAYER_METHOD_COLUMNS: usize = 9;

/// The lines of `hc_prayer_methods`: every method of [`PRAYER_METHODS`],
/// one a line, as its identifier, its English name, the depression of the
/// Sun at *fajr* in arcminutes, the depression at *maghrib* in arcminutes
/// (empty for a method that takes sunset), the depression at *ʿishāʾ* in
/// arcminutes (empty for a method of an interval), the interval after
/// *maghrib* in minutes outside and during Ramaḍān (empty for a method of
/// an angle), the middle of the night's rule, `sunset-to-sunrise` or
/// `sunset-to-fajr`, and its source.
#[must_use]
pub fn prayer_methods_lines() -> String {
    let mut out = String::new();
    for method in PRAYER_METHODS {
        let mut line = Line::new(&mut out);
        line.cell(method.id)
            .cell(method.english_name)
            .value(method.fajr_depression_arcminutes);
        match method.maghrib {
            MaghribRule::Depression(arcminutes) => line.value(arcminutes),
            MaghribRule::Sunset => line.empty(),
        };
        match method.isha {
            IshaRule::Depression(arcminutes) => line.value(arcminutes).empties(2),
            IshaRule::AfterMaghrib {
                minutes,
                ramadan_minutes,
            } => line.empty().value(minutes).value(ramadan_minutes),
        };
        line.cell(match method.midnight {
            MidnightRule::SunsetToSunrise => "sunset-to-sunrise",
            MidnightRule::SunsetToFajr => "sunset-to-fajr",
        })
        .cell(method.source);
        line.end();
    }
    out
}

/// The dawns and nightfalls `hc_zmanim` writes after the times in temporal
/// hours, in this order.
pub const JEWISH_TWILIGHTS: [&str; 4] = [
    "dawn-16-1-degrees",
    "dawn-72-minutes",
    "nightfall-8-5-degrees",
    "nightfall-72-minutes",
];

/// How many columns each line of [`zmanim_lines`] writes.
pub const ZMAN_COLUMNS: usize = 4 + MISSING_COLUMNS;

/// The lines of `hc_zmanim`: the Jewish times of a local day at a place by
/// a reckoning of [`solar_time::ZMANIM_RECKONINGS`], selected by its
/// identifier — one line for each of
/// [`Zman::ALL`], its identifier, its English name as Hebcal prints it,
/// its temporal hours from the start of the day, the instant and the four
/// cells of a missing solar event — then one line for each of
/// [`JEWISH_TWILIGHTS`], which no reckoning changes, with its identifier,
/// empty name and hours, the instant and the four cells.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a reckoning not named, and
/// [`Refusal::OutOfRange`] for a day outside the sky layer's era.
pub fn zmanim_lines(reckoning: &str, fixed: i64, place: Location) -> Answer<String> {
    let zman = solar_time::zmanim_reckoning(reckoning)
        .ok_or(Refusal::Unknown)?
        .zman;
    let day = day_in_era(fixed)?;
    let mut out = String::new();
    for time in Zman::ALL {
        let mut line = Line::new(&mut out);
        line.cell(time.id).cell(time.english_name).value(time.hours);
        moment_or_missing(&mut line, zman(time, day, place));
        line.end();
    }
    let twilights = [
        solar_time::jewish_dawn_16_1_degrees(day, place),
        solar_time::jewish_dawn_72_minutes(day, place),
        solar_time::jewish_nightfall_8_5_degrees(day, place),
        solar_time::jewish_nightfall_72_minutes(day, place),
    ];
    for (id, time) in JEWISH_TWILIGHTS.into_iter().zip(twilights) {
        let mut line = Line::new(&mut out);
        line.cell(id).empties(2);
        moment_or_missing(&mut line, time);
        line.end();
    }
    Ok(out)
}

/// How many columns [`edo_time_line`] writes.
pub const EDO_TIME_COLUMNS: usize = 8 + MISSING_COLUMNS;

/// The line of `hc_edo_time`: the Edo 不定時法 reading of a Universal Time
/// instant at a place by the 寛政暦's 明け六つ and 暮れ六つ — the fixed day
/// whose 明け六つ began the reading's day, the hour's place in the count
/// from 明け六つ (0 to 11), its name (明六つ … 暁七つ), its name in Hepburn
/// romaji, the strokes of the bell it is named by, its earthly branch, the
/// 天保暦's tenths of the hour gone (0 to 9) and the fraction of the hour
/// gone — then the four cells of a missing solar event. Where a dawn or a
/// dusk around the instant does not happen the first eight cells are
/// empty and the last four name it.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for an instant outside the sky layer's era.
pub fn edo_time_line(unix_seconds: i64, place: Location) -> Answer<String> {
    let universal = moment_in_era(unix_seconds)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    match solar_time::edo_time_kansei(universal, place) {
        Ok(reading) => {
            let hour = reading.hour;
            line.value(reading.day.0)
                .value(hour.index())
                .cell(hour.japanese_name())
                .cell(hour.romaji())
                .value(hour.strokes())
                .value(hour.branch())
                .value(reading.tenths())
                .value(reading.fraction);
            missing_cells(&mut line, None);
        }
        Err(missing) => {
            line.empties(EDO_TIME_COLUMNS - MISSING_COLUMNS);
            missing_cells(&mut line, Some(missing));
        }
    }
    line.end();
    Ok(out)
}

/// The line of `hc_unix_from_edo_time`: the Universal Time of a 不定時法
/// reading at a place — the day whose 明け六つ begins it, the hour's place
/// in the count from 明け六つ and the fraction of the hour gone — as whole
/// POSIX seconds, rounded down, and the four cells of a missing solar
/// event.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for an hour above 11, a fraction that is not
/// finite or is outside 0 to 1 (1 excluded), and a day outside the sky
/// layer's era.
pub fn unix_from_edo_time_line(
    fixed: i64,
    hour: u32,
    fraction: f64,
    place: Location,
) -> Answer<String> {
    let hour = u8::try_from(hour)
        .ok()
        .and_then(EdoHour::from_index)
        .ok_or(Refusal::OutOfRange)?;
    if !(0.0..1.0).contains(&fraction) {
        return Err(Refusal::OutOfRange);
    }
    let day = day_in_era(fixed)?;
    let reading = EdoTime {
        day,
        hour,
        fraction,
    };
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    moment_or_missing(
        &mut line,
        solar_time::universal_from_edo_time_kansei(reading, place),
    );
    line.end();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::astro_lines::{location, unix_from_moment};
    use alloc::vec::Vec;
    use hc_calendar::Rd;
    use hc_calendar::fixed::RD_OF_UNIX_EPOCH;
    use hc_calendars_solar::gregorian;

    fn rows(text: &str) -> Vec<Vec<&str>> {
        text.lines()
            .map(|line| line.split('\t').collect())
            .collect()
    }

    /// The minute of the local day a line's instant falls at, in a zone
    /// `zone` hours east of UTC.
    fn local_minutes(cell: &str, day: Rd, zone: f64) -> f64 {
        let unix: i64 = cell.parse().expect("an instant");
        (unix as f64 / 86_400.0 + RD_OF_UNIX_EPOCH as f64 - day.0 as f64) * 1_440.0 + zone * 60.0
    }

    /// MUIS's *Prayer Times for Singapore, Year 2026*
    /// (`muis-prayer-timetable-2026`): on 1 January, Subuh 5:44, Syuruk
    /// 7:08, Zohor 13:10, Maghrib 19:11 and Isyak 20:25, UTC+8. By the
    /// Singapore method each computed time is that or up to a minute and a
    /// half earlier, and the transit half a minute to two and a half
    /// minutes earlier, as `hc-astro`'s test of the whole timetable finds.
    #[test]
    fn the_singapore_method_gives_muis_times_for_new_years_day_2026() {
        let singapore = location(1.0 + 17.0 / 60.0, 103.0 + 50.0 / 60.0, 0.0).expect("a place");
        let day = gregorian::to_fixed(2026, 1, 1).expect("a date");
        let text = prayer_times_lines("Singapore", day.0, singapore, false).expect("in range");
        let rows = rows(&text);
        assert_eq!(rows.len(), PRAYER_TIMES.len());
        for (row, id) in rows.iter().zip(PRAYER_TIMES) {
            assert_eq!(row.len(), PRAYER_TIME_COLUMNS);
            assert_eq!(row[0], id);
            assert_eq!(row[2..], ["", "", "", ""], "{id} happens");
        }
        for (index, printed, earliest) in [
            (0, 5.0 * 60.0 + 44.0, -0.5),
            (1, 7.0 * 60.0 + 8.0, -0.5),
            (2, 13.0 * 60.0 + 10.0, 0.5),
            (5, 19.0 * 60.0 + 11.0, -0.5),
            (6, 20.0 * 60.0 + 25.0, -0.5),
        ] {
            let late = printed - local_minutes(rows[index][1], day, 8.0);
            assert!(
                (earliest..earliest + 2.0).contains(&late),
                "{}: {late} min",
                PRAYER_TIMES[index]
            );
        }
        // The Hanafi ʿaṣr comes after the Shafiʿi, and the night's middle
        // after ʿishāʾ.
        let at = |index: usize| rows[index][1].parse::<i64>().expect("an instant");
        assert!(at(3) < at(4) && at(4) < at(5) && at(6) < at(7));
        assert_eq!(
            prayer_times_lines("", day.0, singapore, false),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            prayer_times_lines("mwl", i64::MAX, singapore, false),
            Err(Refusal::OutOfRange)
        );
    }

    /// Umm al-Qura's *ʿishāʾ* is 90 minutes after *maghrib*, and 120 in
    /// Ramaḍān; Tromsø at midsummer has no *fajr* by any angle.
    #[test]
    fn ramadan_moves_umm_al_quras_isha_and_a_missing_dawn_is_named() {
        let makkah = location(21.4225, 39.8262, 0.0).expect("a place");
        let day = gregorian::to_fixed(2026, 2, 20).expect("a date");
        let instant = |ramadan: bool, index: usize| {
            let text = prayer_times_lines("umm-al-qura", day.0, makkah, ramadan).expect("a day");
            rows(&text)[index][1].parse::<i64>().expect("an instant")
        };
        assert!((instant(false, 6) - instant(false, 5) - 90 * 60).abs() <= 1);
        assert!((instant(true, 6) - instant(true, 5) - 120 * 60).abs() <= 1);
        let tromso = location(69.6496, 18.9560, 0.0).expect("a place");
        let midsummer = gregorian::to_fixed(2026, 6, 21).expect("a date");
        let text = prayer_times_lines("mwl", midsummer.0, tromso, false).expect("a day");
        let fajr = &rows(&text)[0];
        assert_eq!(
            fajr[1..],
            ["", "depression", &midsummer.0.to_string(), "1080", "64800"]
        );
    }

    #[test]
    fn every_method_is_a_line_of_its_parameters() {
        let text = prayer_methods_lines();
        let rows = rows(&text);
        assert_eq!(rows.len(), PRAYER_METHODS.len());
        for row in &rows {
            assert_eq!(row.len(), PRAYER_METHOD_COLUMNS, "{row:?}");
        }
        let singapore = rows.iter().find(|row| row[0] == "singapore").expect("MUIS");
        assert_eq!(
            singapore[2..8],
            ["1200", "", "1080", "", "", "sunset-to-sunrise"]
        );
        let tehran = rows.iter().find(|row| row[0] == "tehran").expect("Tehran");
        assert_eq!(
            tehran[2..8],
            ["1062", "270", "840", "", "", "sunset-to-fajr"]
        );
        let makkah = rows
            .iter()
            .find(|row| row[0] == "umm-al-qura")
            .expect("Makkah");
        assert_eq!(makkah[4..6], ["", "90"]);
        assert_eq!(makkah[6], "120");
    }

    /// Hebcal's zmanim for New York City on 1 January 2025, UTC−5
    /// (`hebcal-zmanim-api`): the latest Shema 9:04 by the MGA and 9:40 by
    /// the GRA, dawn at 16.1° 5:52, nightfall at 8.5° 17:25 and at 72
    /// minutes 17:52, each within a minute.
    #[test]
    fn the_new_york_zmanim_of_new_years_day_2025_are_hebcals() {
        let new_york = location(40.71427, -74.00597, 0.0).expect("a place");
        let day = gregorian::to_fixed(2025, 1, 1).expect("a date");
        let gra = zmanim_lines("ZMANIM-GRA", day.0, new_york).expect("in range");
        let mga = zmanim_lines("mga-72-minutes", day.0, new_york).expect("in range");
        let (gra, mga) = (rows(&gra), rows(&mga));
        assert_eq!(gra.len(), Zman::ALL.len() + JEWISH_TWILIGHTS.len());
        for row in gra.iter().chain(&mga) {
            assert_eq!(row.len(), ZMAN_COLUMNS);
        }
        assert_eq!(gra[0][..3], ["sof-zman-shma", "Latest Shema", "3"]);
        for (row, printed) in [
            (&mga[0], 9.0 * 60.0 + 4.0),
            (&gra[0], 9.0 * 60.0 + 40.0),
            (&gra[5], 5.0 * 60.0 + 52.0),
            (&gra[7], 17.0 * 60.0 + 25.0),
            (&gra[8], 17.0 * 60.0 + 52.0),
        ] {
            let off = local_minutes(row[3], day, -5.0) - printed;
            assert!(off.abs() < 1.0, "{}: {off} min", row[0]);
        }
        assert_eq!(gra[5][..3], ["dawn-16-1-degrees", "", ""]);
        assert_eq!(
            zmanim_lines("baal-hatanya", day.0, new_york),
            Err(Refusal::Unknown)
        );
        // The names are `hc-astro`'s table's, and each answers with its
        // own reckoning's times; the bare authority is not one of them.
        for reckoning in solar_time::ZMANIM_RECKONINGS {
            let text = zmanim_lines(reckoning.id, day.0, new_york).expect("in range");
            let expected = (reckoning.zman)(&Zman::SOF_ZMAN_SHMA, day, new_york).expect("a time");
            let cells = &rows(&text)[0];
            assert_eq!(
                cells[3],
                unix_from_moment(expected).to_string(),
                "{}",
                reckoning.id
            );
        }
        assert_eq!(zmanim_lines("gra", day.0, new_york), Err(Refusal::Unknown));
    }

    /// Hebcal prints no 16.1° dawn for London on 21 June 2025
    /// (`hebcal-zmanim-api`), and the MGA's 16.1° day has no times there.
    #[test]
    fn londons_midsummer_has_no_sixteen_degree_dawn() {
        let london = location(51.50853, -0.12574, 0.0).expect("a place");
        let day = gregorian::to_fixed(2025, 6, 21).expect("a date");
        let text = zmanim_lines("mga-16-1-degrees", day.0, london).expect("in range");
        let rows = rows(&text);
        assert_eq!(
            rows[0][3..],
            ["", "depression", &day.0.to_string(), "966", "57960"]
        );
        assert_eq!(rows[5][4], "depression");
        assert_ne!(rows[6][3], "", "the 72-minute dawn exists");
    }

    /// こよみのページ, 「理科年表の「夜明」と「日暮」の角度・補稿」: at Kyoto
    /// 夜明 by the Observatory's 7°21′40″ was at 5:28:47 JST on 20 March
    /// 2020, and 明け六つ by the 寛政暦's angle a tenth of a second earlier.
    /// That moment is 明六つ, hour 0, and 暮れ六つ is hour 6; the inverse
    /// of the reading is the instant again.
    #[test]
    fn kyoto_dawn_on_the_equinox_of_2020_begins_ake_mutsu() {
        let kyoto = location(35.0 + 36.0 / 3_600.0, 135.7417, 0.0).expect("a place");
        let day = gregorian::to_fixed(2020, 3, 20).expect("a date");
        let dawn = crate::astro_lines::solar_event_line("japanese-dawn-naoj", day.0, kyoto)
            .expect("in range");
        let dawn_cells: Vec<&str> = dawn.trim_end_matches('\n').split('\t').collect();
        assert_eq!(dawn_cells.len(), 1 + MISSING_COLUMNS);
        let jst = local_minutes(dawn_cells[0], day, 9.0) * 60.0;
        let printed = 5.0 * 3_600.0 + 28.0 * 60.0 + 47.0;
        assert!((jst - printed).abs() < 10.0, "{jst} s");
        let unix: i64 = dawn_cells[0].parse().expect("an instant");
        let reading = edo_time_line(unix + 2, kyoto).expect("in range");
        let cells: Vec<&str> = reading.trim_end_matches('\n').split('\t').collect();
        assert_eq!(cells.len(), EDO_TIME_COLUMNS);
        assert_eq!(
            cells[..7],
            [
                day.0.to_string().as_str(),
                "0",
                "明六つ",
                "ake mutsu",
                "6",
                "卯",
                "0"
            ]
        );
        let noon = edo_time_line(unix + 7 * 3_600, kyoto).expect("in range");
        assert_eq!(noon.split('\t').nth(2), Some("昼九つ"));
        let back = unix_from_edo_time_line(day.0, 0, 0.0, kyoto).expect("in range");
        let seconds: i64 = back
            .split('\t')
            .next()
            .expect("a cell")
            .parse()
            .expect("an instant");
        assert!((seconds - unix).abs() <= 1, "{seconds} against {unix}");
        let fraction: f64 = cells[7].parse().expect("a fraction");
        assert!((0.0..0.01).contains(&fraction));
        assert_eq!(
            unix_from_edo_time_line(day.0, 12, 0.0, kyoto),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            unix_from_edo_time_line(day.0, 0, 1.0, kyoto),
            Err(Refusal::OutOfRange)
        );
    }

    /// Helsinki at midsummer: the Sun stays within 6.4° of the horizon, so
    /// the 寛政暦's dusk does not happen and the depression is named to the
    /// arcsecond, 7°21′41″, with no whole number of arcminutes to give.
    #[test]
    fn a_missing_japanese_dusk_is_named_in_arcseconds() {
        let helsinki = location(60.1699, 24.9384, 0.0).expect("a place");
        let day = gregorian::to_fixed(2024, 6, 21).expect("a date");
        let line = crate::astro_lines::solar_event_line("japanese-dusk-kansei", day.0, helsinki)
            .expect("in range");
        let cells: Vec<&str> = line.trim_end_matches('\n').split('\t').collect();
        assert_eq!(cells, ["", "depression", &day.0.to_string(), "", "26501"]);
        let noon = crate::astro_lines::unix_from_moment(Moment(day.0 as f64 + 0.4));
        let reading = edo_time_line(noon, helsinki).expect("in range");
        let cells: Vec<&str> = reading.trim_end_matches('\n').split('\t').collect();
        assert_eq!(cells[..8], ["", "", "", "", "", "", "", ""]);
        assert_eq!(cells[8], "depression");
    }
}
