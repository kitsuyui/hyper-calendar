//! Panchak: the Moon's passage through the last five nakṣatras, from the
//! third quarter of Dhaniṣṭhā to the end of Revatī, and the kind a window
//! takes from the weekday it begins on.
//!
//! `docs/systems/panchak.md` in the repository describes the system,
//! works an example and states the sources; this page states the code's
//! own facts.
//!
//! The window opens when the Moon's sidereal longitude reaches 300°, the
//! start of the third quarter of Dhaniṣṭhā and of the sign Kumbha, and
//! closes when it reaches 360°, the end of Revatī and of Mīna
//! (`nakshatrica-panchak`). It lasts four to five days and comes round once
//! a sidereal month, thirteen or fourteen times a year. The longitude is
//! [`crate::nakshatra`]'s, so the window moves with the ayanamsa, which is
//! a parameter.
//!
//! The almanacs name a window by the weekday it begins on. They agree on
//! five of the seven weekdays and disagree on Wednesday and Thursday, so the
//! two tables are two [`PanchakNaming`] entries (`docs/policy.md` §5):
//! [`PanchakNaming::FIVE_KINDS`] names no kind on those two days, and
//! [`PanchakNaming::RAJ_MIDWEEK`] calls them Raj Panchak, as it calls
//! Monday's.

use hc_calendar::Weekday;
use hc_calendar::fixed::Moment;
use hc_seasons::zodiac::Ayanamsa;

use crate::nakshatra::{sidereal_lunar_longitude, sidereal_lunar_longitude_at_or_after};

/// The Moon's sidereal longitude at which a window opens: the start of the
/// third quarter of Dhaniṣṭhā, 22 nakṣatras and two quarters from zero.
pub const OPENS_AT_DEGREES: f64 = 300.0;

/// The Moon's sidereal longitude at which a window closes: the end of
/// Revatī, the last nakṣatra.
pub const CLOSES_AT_DEGREES: f64 = 360.0;

/// Whether the Moon is in the Panchak arc at a moment.
#[must_use]
pub fn is_panchak(moment: Moment, ayanamsa: Ayanamsa) -> bool {
    sidereal_lunar_longitude(moment, ayanamsa) >= OPENS_AT_DEGREES
}

/// A window: when the Moon reaches 300° and when it reaches 360°.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Window {
    /// The moment it opens, in Universal Time.
    pub opens: Moment,
    /// The moment it closes, in Universal Time.
    pub closes: Moment,
}

/// The window in progress at `moment`, or the next one when the Moon is
/// outside the arc then.
#[must_use]
pub fn window(moment: Moment, ayanamsa: Ayanamsa) -> Window {
    // A window lasts under five and a half days, so one in progress
    // opened within that of `moment`.
    let from = if is_panchak(moment, ayanamsa) {
        Moment(moment.0 - 5.5)
    } else {
        moment
    };
    let opens = sidereal_lunar_longitude_at_or_after(OPENS_AT_DEGREES, from, ayanamsa);
    let closes = sidereal_lunar_longitude_at_or_after(0.0, opens, ayanamsa);
    Window { opens, closes }
}

/// One almanac tradition's names for a window by the weekday it begins on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PanchakNaming {
    /// The identifier.
    pub id: &'static str,
    /// The kind for each weekday, Sunday first, or `None` where the
    /// tradition names none.
    pub kind_by_weekday: [Option<&'static str>; 7],
    /// Where the names come from.
    pub source: &'static str,
}

impl PanchakNaming {
    /// The kind of a window that begins on `weekday`, or `None` where the
    /// tradition names none. Which clock the weekday is read on, midnight
    /// to midnight or sunrise to sunrise, the sources do not say: the
    /// caller decides.
    #[must_use]
    pub const fn kind(&self, weekday: Weekday) -> Option<&'static str> {
        self.kind_by_weekday[weekday.sunday_first_number() as usize]
    }
}

hc_core::catalogue! {
    type: PanchakNaming,
    id: |naming| naming.id,
    provenance: |naming| naming.source,
    tests: panchak_catalogue_tests,
    associated;

    /// The two tables.
    pub const ALL;
    /// The table with this identifier.
    pub fn by_id;

    entries: {
        /// Five kinds, none on Wednesday or Thursday: Rog on Sunday, Raj on
        /// Monday, Agni on Tuesday, Chor on Friday, Mrityu on Saturday.
        /// Prokerala names these five and no others, spelling Monday's
        /// "Rajya"; Nakshatrica calls the Wednesday and Thursday windows
        /// "Neutral".
        pub const FIVE_KINDS = Self {
            id: "panchak-five-kinds",
            kind_by_weekday: [
                Some("Rog"),
                Some("Raj"),
                Some("Agni"),
                None,
                None,
                Some("Chor"),
                Some("Mrityu"),
            ],
            source: "Prokerala, \"Panchak\" (prokerala-panchak), and Nakshatrica, \"Panchak \
                     Calendar\" (nakshatrica-panchak), retrieved 2026-09-28",
        };
        /// The same five, with Wednesday and Thursday also Raj Panchak, as
        /// India TV states it: "Monday, Wednesday or Thursday".
        pub const RAJ_MIDWEEK = Self {
            id: "panchak-raj-midweek",
            kind_by_weekday: [
                Some("Rog"),
                Some("Raj"),
                Some("Agni"),
                Some("Raj"),
                Some("Raj"),
                Some("Chor"),
                Some("Mrityu"),
            ],
            source: "India TV, \"Panchak December 2025\", 15 December 2025 \
                     (indiatv-panchak-2025), retrieved 2026-09-28",
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendars_solar::gregorian;

    /// A moment given in Indian Standard Time.
    fn ist(year: i64, month: u8, day: u8, hour: u8, minute: u8) -> Moment {
        let day = gregorian::to_fixed(year, month, day).expect("a date");
        Moment(day.0 as f64 + (f64::from(hour) + f64::from(minute) / 60.0 - 5.5) / 24.0)
    }

    /// A printed moment: month, day, hour and minute.
    type Printed = (u8, u8, u8, u8);

    /// Drik Panchang's Panchak days of 2025 for New Delhi
    /// (`drik-panchak`, retrieved 2026-09-28): the month, day, hour
    /// and minute IST each window begins, and the same for its end.
    const DRIK_2025: [(Printed, Printed); 14] = [
        ((1, 3, 10, 47), (1, 7, 17, 50)),
        ((1, 30, 18, 35), (2, 3, 23, 16)),
        ((2, 27, 4, 37), (3, 3, 6, 39)),
        ((3, 26, 15, 14), (3, 30, 16, 35)),
        ((4, 23, 0, 31), (4, 27, 3, 39)),
        ((5, 20, 7, 35), (5, 24, 13, 48)),
        ((6, 16, 13, 10), (6, 20, 21, 45)),
        ((7, 13, 18, 53), (7, 18, 3, 39)),
        ((8, 10, 2, 11), (8, 14, 9, 6)),
        ((9, 6, 11, 21), (9, 10, 16, 3)),
        ((10, 3, 21, 27), (10, 8, 1, 28)),
        ((10, 31, 6, 48), (11, 4, 12, 34)),
        ((11, 27, 14, 7), (12, 1, 23, 18)),
        ((12, 24, 19, 46), (12, 29, 7, 41)),
    ];

    #[test]
    fn the_windows_of_2025_open_and_close_when_drik_panchang_says() {
        let mut offsets = alloc::vec::Vec::new();
        let mut moment = ist(2025, 1, 1, 0, 0);
        for ((om, od, oh, omin), (cm, cd, ch, cmin)) in DRIK_2025 {
            let found = window(moment, Ayanamsa::LAHIRI);
            let opens = ist(2025, om, od, oh, omin);
            let closes = ist(2025, cm, cd, ch, cmin);
            offsets.push((found.opens.0 - opens.0) * 1_440.0);
            offsets.push((found.closes.0 - closes.0) * 1_440.0);
            moment = Moment(found.closes.0 + 1.0);
        }
        // Every opening and closing from 6 seconds before the printed
        // minute to 52 seconds after it, as the pages appear to truncate
        // to the minute.
        let (low, high) = offsets.iter().fold((f64::MAX, f64::MIN), |(low, high), m| {
            (low.min(*m), high.max(*m))
        });
        assert!(low > -0.5 && high < 1.0, "{offsets:?}");
    }

    /// Prokerala's windows for Ujjain in IST (`prokerala-panchak`,
    /// retrieved 2026-09-28): 23 September 2026 21:52 to 28 September
    /// 10:16, 21 October 06:55 to 25 October 19:22, 17 November 15:25 to
    /// 22 November 05:54.
    #[test]
    fn prokerala_closes_the_autumn_windows_of_2026_with_drik_panchang_but_opens_them_earlier() {
        for ((om, od, oh, omin), (cm, cd, ch, cmin)) in [
            ((9, 23, 21, 52), (9, 28, 10, 16)),
            ((10, 21, 6, 55), (10, 25, 19, 22)),
            ((11, 17, 15, 25), (11, 22, 5, 54)),
        ] {
            let opens = ist(2026, om, od, oh, omin);
            let found = window(Moment(opens.0 + 1.0), Ayanamsa::LAHIRI);
            let closes = ist(2026, cm, cd, ch, cmin);
            let closing = (found.closes.0 - closes.0) * 1_440.0;
            assert!((-0.5..1.0).contains(&closing), "{cm}-{cd}: {closing}");
            // Prokerala opens each window about five minutes before the
            // Moon reaches 300°; Drik Panchang opens the October one at
            // 07:00, with this module. Why is not stated on the page.
            let opening = (found.opens.0 - opens.0) * 1_440.0;
            assert!((4.5..5.5).contains(&opening), "{om}-{od}: {opening}");
        }
        // Drik Panchang's page for 2026 (`drik-panchak`): 21 October
        // 07:00 to 25 October 19:22.
        let october = window(ist(2026, 10, 20, 0, 0), Ayanamsa::LAHIRI);
        assert!(((october.opens.0 - ist(2026, 10, 21, 7, 0).0) * 1_440.0).abs() < 1.0);
        assert!(((october.closes.0 - ist(2026, 10, 25, 19, 22).0) * 1_440.0).abs() < 1.0);
    }

    #[test]
    fn the_two_tables_part_on_wednesday_and_thursday_only() {
        // Drik Panchang's last window of 2025 opens on Wednesday
        // 24 December, which India TV calls Raj Panchak.
        let december = gregorian::to_fixed(2025, 12, 24).expect("a date");
        let weekday = Weekday::from_rd(december);
        assert_eq!(weekday, Weekday::Wednesday);
        assert_eq!(PanchakNaming::RAJ_MIDWEEK.kind(weekday), Some("Raj"));
        assert_eq!(PanchakNaming::FIVE_KINDS.kind(weekday), None);
        // The first of 2025 opens on Friday 3 January: Chor in both.
        let january = gregorian::to_fixed(2025, 1, 3).expect("a date");
        for naming in PanchakNaming::ALL {
            assert_eq!(naming.kind(Weekday::from_rd(january)), Some("Chor"));
        }
        let parting: alloc::vec::Vec<Weekday> = [
            Weekday::Sunday,
            Weekday::Monday,
            Weekday::Tuesday,
            Weekday::Wednesday,
            Weekday::Thursday,
            Weekday::Friday,
            Weekday::Saturday,
        ]
        .into_iter()
        .filter(|day| PanchakNaming::FIVE_KINDS.kind(*day) != PanchakNaming::RAJ_MIDWEEK.kind(*day))
        .collect();
        assert_eq!(parting, [Weekday::Wednesday, Weekday::Thursday]);
    }

    #[test]
    fn a_window_holds_the_moon_in_the_last_sixty_degrees() {
        let found = window(ist(2025, 1, 1, 0, 0), Ayanamsa::LAHIRI);
        let middle = Moment(f64::midpoint(found.opens.0, found.closes.0));
        assert!(is_panchak(middle, Ayanamsa::LAHIRI));
        assert!(!is_panchak(Moment(found.opens.0 - 0.01), Ayanamsa::LAHIRI));
        assert!(!is_panchak(Moment(found.closes.0 + 0.01), Ayanamsa::LAHIRI));
        let days = found.closes.0 - found.opens.0;
        assert!((4.0..5.5).contains(&days), "{days}");
        // From inside the window the same window is found.
        assert_eq!(window(middle, Ayanamsa::LAHIRI), found);
        assert_eq!(
            PanchakNaming::by_id("panchak-raj-midweek"),
            Some(PanchakNaming::RAJ_MIDWEEK)
        );
    }
}
