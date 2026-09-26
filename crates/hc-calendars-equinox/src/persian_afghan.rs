//! The Solar Hijri calendar as Afghanistan kept it — `persian-afghan`.
//!
//! The same calendar as [`crate::persian`], under other month names. The
//! year begins at the March equinox, the first six months have thirty-one
//! days, the next five thirty and the last twenty-nine or thirty, and every
//! day falls where the Iranian calendar puts it; the months are named for
//! the signs of the zodiac in their Arabic forms — *Hamal*, *Sawr*,
//! *Jawzā* … *Hūt* in Dari — where Iran's are the Zoroastrian *Farvardin* …
//! *Esfand*. The date type, the equinox rule and every function of the
//! arithmetic are `persian`'s; this module adds the names and the period of
//! use, and nothing else.
//!
//! # The period of use
//!
//! The solar Hijri calendar has been official in Afghanistan since 1301 SH
//! (1922), but the month lengths of this calendar are later: before
//! 1336 SH (1957) "the number of days in most months ranged from 29 to 32
//! according to the year", and only in 1336 SH were they fixed at six of
//! thirty-one, five of thirty and a last of twenty-nine or thirty (Daniel
//! Balland, "Calendars iii. Afghan calendars", *Encyclopaedia Iranica*;
//! it calls the Afghan calendar "basically the same as the Persian one").
//! So [`usage`](Calendar::usage) begins on 1 Hamal 1336, 21 March 1957, and
//! the years of 1301–1335 are [`hc_calendar::Standing::Proleptic`]: their
//! months had other lengths, which this calendar does not reproduce.
//!
//! The Constitution of 2004 made it "the basis for state offices" (art.
//! 18). Civil use ends with 7 Asad 1401, 29 July 2022. The Islamic Emirate's
//! Administrative Office ordered in March 2022 that all administrative
//! letters and correspondence be dated by the lunar Hijri year, their
//! registers to run to the end of the lunar year and to start afresh with
//! the new one (Hasht-e Subh, 26 March 2022; Afghanistan International,
//! 29 March 2022), and the official calendar of 1444 AH that the Ministry
//! of Information and Culture then issued runs from 1 Muharram, 8 Asad
//! 1401, to 27 Saratan 1402 (Rukhshana Media, 2 August 2022). What the
//! switch did not do is retire the solar calendar: that official calendar
//! still gives each lunar date its solar equivalent and keeps the holidays
//! of 28 Asad, the recovery of independence, and 26 Dalw, while it drops
//! the holidays of the first and second days of the solar new year, the
//! Nowruz holidays. Two months before the switch was ordered, the cabinet
//! had moved the fiscal year onto the solar Hijri year, from 1 Hamal
//! (Ariana News, 11 January 2022), and no source read says the
//! fiscal year moved again. So the calendar is [`Usage::civil_until`] the
//! day before 1 Muharram 1444 and in use without an end after it.
//!
//! # Which noon
//!
//! Iran's rule decides Nowruz by noon, Iran Standard Time; none of the
//! sources read states a separate rule for Afghanistan, whose clocks keep
//! UTC+04:30, and Balland calls the Afghan calendar "basically the same as
//! the Persian one". This module keeps Iran's rule. The two noons are an
//! hour apart, and they name different days only in a year whose equinox
//! falls between 07:30 and 08:30 Universal Time. Four years of the civil
//! period are such years: 1342, 1346, 1375 and 1379, whose equinoxes fell
//! at about 08:20, 07:37, 08:03 and 07:35 UT, so that 1 Hamal is 21 March
//! 1963, 21 March 1967, 20 March 1996 and 20 March 2000 here and would be a
//! day later by a noon at Kabul. The one Afghan calendar read that states
//! its rule, the volunteer Afghan Calendar project's, puts 1 Hamal on
//! 20 March in Gregorian leap years and on 21 March otherwise, which agrees
//! with Iran's rule in all four; it is not an authority, and a test names
//! the four years rather than claims them.
//!
//! The Taliban's first government also imposed the lunar Hijri year in the
//! areas it held: Afghanistan International dates Mullah Mohammad Omar's
//! decree to 1378 (1999), and Wikipedia's "Solar Hijri calendar" has the
//! year change "overnight from 1375 to 1417", in 1996, citing NBC News
//! (not read). A period of use has one beginning and two ends, not a gap,
//! so those years are inside the civil period here, and this paragraph is
//! where the gap is recorded.
//!
//! # Sources
//!
//! * Daniel Balland, "Calendars iii. Afghan calendars", *Encyclopaedia
//!   Iranica* IV/6 (1990), `iranicaonline.org/articles/calendars`, read
//!   2026-09-26 in the Wayback Machine's copy of 5 September 2026 (the live
//!   page is behind a browser check): 1301 SH, the month lengths fixed in
//!   1336 SH, the Dari names as the Arabic names of the signs, the Pashto
//!   translations "rarely used". Its Table 39 of month names is an image
//!   and was not read. (`balland1990`)
//! * Wikipedia, "Solar Hijri calendar", retrieved 2026-09-26: the month
//!   table with the Dari and Pashto names in their own script and
//!   romanised, which gives the romanisations `hc-i18n` carries.
//!   (`wikipedia-solar-hijri-calendar`)
//! * Unicode CLDR 48, `common/main/fa_AF.xml` and `ps.xml`, `calendar
//!   type="persian"`, read 2026-09-26: the same Dari names, with the ezafe
//!   on the format form *سنبلهٔ*, and the Pashto names `hc-i18n`'s `ps`
//!   carries. (`cldr48-persian-months`)
//! * Mohammad Shaker Rasa, "Taliban Changes Solar Year to Hijri Lunar
//!   Calendar", *Hasht-e Subh*, 26 March 2022, `8am.media/eng/`, retrieved
//!   2026-09-26; its dateline gives 26 March 2022 as 1401/01/06.
//!   (`rasa2022`)
//! * Ayoub Arwin, "قمری به جای خورشیدی؛ گروه طالبان با تغییر تقویم به دنبال
//!   چیست؟", *Afghanistan International*, 9 Hamal 1401 (29 March 2022),
//!   `afintl.com/202203299928`, retrieved 2026-09-26: the letter signed by
//!   the head of the Administrative Office and its scope, "تمام مکاتیب و
//!   مراسلات اداری". (`arwin2022`)
//! * "تقویم جدید طالبان…", *Rukhshana Media*, 2 August 2022,
//!   `rukhshana.com`, retrieved 2026-09-26: the official calendar of
//!   1444 AH from 1 Muharram (8 Asad 1401), its solar equivalents and its
//!   holidays. (`rukhshana2022`)
//! * "IEA changes its fiscal from Gregorian calendar to Hijri Shamsi",
//!   *Ariana News*, 11 January 2022, `ariananews.af`, retrieved 2026-09-26.
//!   (`ariana2022`)
//! * The Constitution of Afghanistan of 2004, art. 18, in the Constitute
//!   Project's English text, `constituteproject.org`, retrieved
//!   2026-09-26: "The basis for state offices shall be the solar
//!   calendar." (`afghanistan-constitution-2004`)
//! * Afghan Calendar project, "Calendar algorithm",
//!   `nongnu.org/afghancalendar`, retrieved 2026-09-26: a volunteer
//!   calendar's arithmetic rule for 1 Hamal, read as a check and not as an
//!   authority. (`afghan-calendar-project`)
//!
//! The system document is `docs/systems/solar-hijri.md`.
//!
//! # Exactness
//!
//! Exactly `persian`'s, which is astronomical: see that module for the
//! tolerance and [`crate::persian::new_year_margin`].

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Usage,
    YearKind,
};
use hc_calendars_solar::persian::PersianDate;

use crate::persian;

/// The calendar identifier.
pub const ID: CalendarId = CalendarId("persian-afghan");

/// The twelve months in Dari, the Arabic names of the signs of the zodiac,
/// in the stand-alone form Wikipedia's month table prints; CLDR 48's
/// `fa_AF` has the same names and writes the sixth, in the format form,
/// with the ezafe, *سنبلهٔ*.
pub const MONTHS: [&str; 12] = [
    "حمل",
    "ثور",
    "جوزا",
    "سرطان",
    "اسد",
    "سنبله",
    "میزان",
    "عقرب",
    "قوس",
    "جدی",
    "دلو",
    "حوت",
];

/// The year from which the month lengths are the ones this calendar has:
/// 1336 SH, when Afghanistan fixed them (Balland, *Encyclopaedia
/// Iranica*).
pub const FIXED_MONTHS_YEAR: i64 = 1_336;

/// The year in which official business moved to the lunar Hijri year.
pub const LUNAR_SWITCH_YEAR: i64 = 1_401;

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Daniel Balland, \"Calendars iii. Afghan calendars\", \
    *Encyclopaedia Iranica* IV/6 (1990), retrieved 2026-09-26: official since 1301 SH, the \
    month lengths fixed in 1336 SH (1957); Hasht-e Subh, 26 March 2022, and Afghanistan \
    International, 29 March 2022: administrative correspondence by the lunar Hijri year from the \
    new lunar year; Rukhshana Media, 2 August 2022: the official calendar of 1444 AH from \
    1 Muharram, 8 Asad 1401, with the solar dates beside the lunar ones";

/// The first day of civil use: 1 Hamal 1336, when the month lengths were
/// fixed.
///
/// # Errors
///
/// None in practice: the year is inside `persian`'s range.
pub fn civil_from() -> CalendarResult<Rd> {
    persian::new_year(FIXED_MONTHS_YEAR)
}

/// The last day of civil use: 7 Asad 1401, the day before 1 Muharram 1444.
///
/// # Errors
///
/// None in practice: the date is inside `persian`'s range.
pub fn civil_until() -> CalendarResult<Rd> {
    persian::to_fixed(LUNAR_SWITCH_YEAR, 5, 7)
}

/// The Solar Hijri calendar as Afghanistan kept it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AfghanPersianCalendar;

/// Twelve months under their Dari names and the seven-day week.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for AfghanPersianCalendar {
    type Date = PersianDate;

    /// Civil from 1 Hamal 1336, when the month lengths were fixed, to
    /// 7 Asad 1401, the day before official business moved to the lunar
    /// Hijri year; in use after it beside the lunar dates, with no end.
    fn usage(&self) -> Usage {
        match (civil_from(), civil_until()) {
            (Ok(from), Ok(until)) => Usage::since(from, USAGE_SOURCE).civil_until(until),
            _ => Usage::UNRECORDED,
        }
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    /// `persian`'s rule: a year whose Hut has thirty days.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        persian::is_leap_year(year).ok_or(CalendarError::YearOutOfRange)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Solar Hijri (Afghanistan)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: true,
            earliest: Some(persian::earliest()),
            latest: Some(persian::latest()),
            native_locales: &["fa-AF", "ps"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        persian::PersianCalendar.to_fixed(date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        persian::PersianCalendar.from_fixed(rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        persian::PersianCalendar.to_fields(date)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        persian::PersianCalendar.from_fields(fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::Standing;
    use hc_calendars_solar::gregorian;
    use hc_calendars_solar::persian::ERA;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    fn to_gregorian(year: i64, month: u8, day: u8) -> CalendarResult<(i64, u8, u8)> {
        gregorian::from_fixed(persian::to_fixed(year, month, day)?)
    }

    #[test]
    fn every_day_is_the_iranian_day_under_another_name() {
        let calendar = AfghanPersianCalendar;
        let start = persian::new_year(1_300).unwrap().0;
        let end = persian::new_year(1_500).unwrap().0;
        for rd in (start..end).step_by(89) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(date, persian::PersianCalendar.from_fixed(Rd(rd)).unwrap());
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "rd {rd}");
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(date));
        }
        assert_eq!(calendar.is_leap_year(1_403), Ok(true));
        assert_eq!(calendar.is_leap_year(1_404), Ok(false));
    }

    #[test]
    fn the_dates_the_afghan_sources_print_are_where_the_calendar_puts_them() {
        // Hasht-e Subh's dateline of 26 March 2022: "Solar Hijri Date:
        // 1401/01/06".
        assert_eq!(to_gregorian(1_401, 1, 6), Ok((2022, 3, 26)));
        // Rukhshana Media, 2 August 2022: the lunar year 1444 from
        // 1 Muharram, 8 Asad 1401.
        assert_eq!(to_gregorian(1_401, 5, 8), Ok((2022, 7, 30)));
        // 1 Hamal 1336, the first day with the fixed month lengths.
        assert_eq!(civil_from(), Ok(ymd(1957, 3, 21)));
    }

    #[test]
    fn civil_from_1957_to_the_eve_of_1_muharram_1444_and_in_use_after() {
        let calendar = AfghanPersianCalendar;
        let usage = calendar.usage();
        assert_eq!(usage.from, Some(ymd(1957, 3, 21)));
        assert_eq!(usage.civil_until, Some(ymd(2022, 7, 29)));
        assert_eq!(usage.until, None);
        assert_eq!(calendar.standing(ymd(1957, 3, 20)), Standing::Proleptic);
        assert_eq!(calendar.standing(ymd(1957, 3, 21)), Standing::InUse);
        assert_eq!(calendar.standing(ymd(2026, 9, 26)), Standing::InUse);
        assert!(usage.is_civil(ymd(2022, 7, 29)));
        assert!(!usage.is_civil(ymd(2022, 7, 30)));
    }

    /// Iran decides Nowruz by noon at UTC+03:30; Afghanistan's clocks keep
    /// UTC+04:30, an hour earlier. The two rules part only when the
    /// equinox falls between 07:30 and 08:30 UT — a margin, before Iran's
    /// noon, of more than nothing and at most sixty minutes — and in the
    /// civil period that is four years, which this calendar decides by
    /// Iran's rule and which no authority read settles.
    #[test]
    fn four_years_of_the_civil_period_turn_on_which_noon_is_meant() {
        let parted: Vec<i64> = (FIXED_MONTHS_YEAR..=LUNAR_SWITCH_YEAR)
            .filter(|&year| (0.0..=60.0).contains(&persian::new_year_margin(year).unwrap()))
            .collect();
        assert_eq!(parted, [1_342, 1_346, 1_375, 1_379]);
        // Iran's rule; the Afghan Calendar project's rule — 20 March in a
        // Gregorian leap year, 21 March otherwise — gives the same four.
        assert_eq!(to_gregorian(1_342, 1, 1), Ok((1963, 3, 21)));
        assert_eq!(to_gregorian(1_346, 1, 1), Ok((1967, 3, 21)));
        assert_eq!(to_gregorian(1_375, 1, 1), Ok((1996, 3, 20)));
        assert_eq!(to_gregorian(1_379, 1, 1), Ok((2000, 3, 20)));
    }

    #[test]
    fn the_months_are_the_signs_in_dari() {
        assert_eq!(MONTHS[0], "حمل");
        assert_eq!(MONTHS[1], "ثور");
        assert_eq!(MONTHS[11], "حوت");
        assert_eq!(SHAPE[0].name(0), Some("حمل"));
        assert_eq!(SHAPE[0].name(11), Some("حوت"));
        let meta = AfghanPersianCalendar.meta();
        assert_eq!(meta.id, ID);
        assert!(meta.is_astronomical);
        assert_eq!(meta.native_locales, &["fa-AF", "ps"]);
    }
}
