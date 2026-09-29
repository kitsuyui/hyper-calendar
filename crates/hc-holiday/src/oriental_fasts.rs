//! The fasts of the Armenian, Coptic and Ethiopian churches, each on its
//! own scheme: the periods of [`crate::orthodox_fasts`]'s reckonings
//! `armenian-fasts`, `armenian-fasts-jerusalem`, `coptic-fasts` and
//! `ethiopian-fasts`, which that module's `status`, `span` and
//! `abstinence` read.
//!
//! `docs/systems/oriental-fasts.md` in the repository describes the three
//! schemes, works a year of each and states the sources; this page states
//! the code's own facts.
//!
//! The three churches keep Wednesdays and Fridays as fast days, as the
//! Eastern Orthodox do, and each lifts them in its own fast-free days. Their
//! seasons are their own:
//!
//! | Reckoning | Its fasts | Its fast-free days |
//! | --- | --- | --- |
//! | `armenian-fasts`, `armenian-fasts-jerusalem` | Great Lent, 48 days; the Fast of the Catechumens; eight weeks of fasting, Monday to Friday, before the feasts; the six days before Theophany | the forty days after Easter; the octave of Theophany |
//! | `coptic-fasts` | the Nativity Fast from 16 Hatour; Jonah's Fast; Great Lent, 55 days; the Apostles' Fast to 4 Epip; the Virgin's Fast, 1–15 Mesori; the Paramoun of Theophany | the Holy Fifty; the Nativity and Theophany |
//! | `ethiopian-fasts` | Tsome Nebiyat from 15 Hidar; Tsome Nenewe; Abiy Tsom, 55 days; Tsome Hawaryat to 4 Hamle; Tsome Filseta, 1–15 Nehase; the Gahad of Timkat | the fifty days; Genna and Timkat |
//!
//! The two Armenian reckonings differ only in the calendar and the
//! computus: the Gregorian ones Etchmiadzin adopted in 1923, and the Julian
//! the Patriarchate of Jerusalem keeps (`docs/policy.md` §5), as the tables
//! `christian-armenian` and `christian-armenian-jerusalem` do for the
//! feasts. The Coptic and Ethiopian fasts are dated in their own calendars,
//! and Easter in both is the Alexandrian computus's, the Julian Pascha.
//!
//! What the days may be eaten is not carried: only whether a day is a fast
//! day, as in `orthodox_fasts`.

use crate::orthodox_fasts::{Bound, Period, PeriodKind};
use crate::rule::CalendarSystem;

const fn from_pascha(days: i16) -> Bound {
    Bound::FromPascha(days)
}

const fn fixed(month: u8, day: u8) -> Bound {
    Bound::Fixed { month, day }
}

const fn nearest(month: u8, day: u8, days: i16) -> Bound {
    Bound::FromSundayNearest { month, day, days }
}

const fn coptic(month: u8, day: u8) -> Bound {
    Bound::InCalendar {
        calendar: CalendarSystem::COPTIC,
        month,
        day,
    }
}

const fn ethiopic(month: u8, day: u8) -> Bound {
    Bound::InCalendar {
        calendar: CalendarSystem::ETHIOPIC,
        month,
        day,
    }
}

const fn period(
    id: &'static str,
    english_name: &'static str,
    kind: PeriodKind,
    first: Bound,
    last: Bound,
    source: &'static str,
) -> Period {
    Period {
        id,
        english_name,
        kind,
        first,
        last,
        source,
    }
}

// ─────────────────────────────────────────────────────────────────────────
// The Armenian Apostolic Church
// ─────────────────────────────────────────────────────────────────────────

/// The Armenian Diocese of Georgia's pages of the fasts, with their lists of
/// the eves of 2020–2030, as `source` strings cite them.
const GEORGIA: &str =
    "Armenian Diocese of Georgia (armenianchurch.ge, armenian-church-georgia-fasts)";
/// The Sydney parish's page of the fasting days.
const SYDNEY: &str = "Armenian Apostolic Church of Holy Resurrection, Sydney, \"Days of \
    Abstinence / Fasting Days\" (armenian-church-sydney-fasts)";

/// Where the Armenian reckonings take their periods from.
pub const ARMENIAN_SOURCES: &str = "Armenian Diocese of Georgia, the pages of the fasts \
    (armenianchurch.ge/en/kalendar-prazdnikov/, armenian-church-georgia-fasts), with the eves of \
    each fast of 2020-2030, for the weeks of fasting from Monday to Friday after the Sunday \
    eve, Great Lent of 48 days, the Christmas fast from 29 December and the Fast of Elijah \
    after Pentecost; Armenian Apostolic Church of Holy Resurrection, Sydney, \"Days of \
    Abstinence / Fasting Days\" (armenian-church-sydney-fasts), for the Wednesdays and Fridays \
    and their exemptions, the forty days after Easter and the octave of Theophany, and the six \
    days before Theophany; the Western Prelacy of the Armenian Apostolic Church of America, \
    \"Fasting or Abstinence Days\" (westernprelacy-fasts), for those days, 30 December to \
    4 January; the Mother See of Holy Etchmiadzin's liturgical calendar of 2026 \
    (armenianchurch.org, armenian-mother-see-calendar-2026), for the Fast of the Catechumens and \
    Great Lent of 2026; all retrieved 2026-09-29";

/// The Armenian fasts, for both reckonings: the fast-free days first.
pub static ARMENIAN: &[Period] = &[
    period(
        "easter-to-ascension",
        "The forty days after Easter",
        PeriodKind::FastFree,
        from_pascha(0),
        from_pascha(39),
        SYDNEY,
    ),
    period(
        "theophany-octave",
        "The octave of Theophany",
        PeriodKind::FastFree,
        fixed(1, 6),
        fixed(1, 13),
        SYDNEY,
    ),
    period(
        "theophany-fast",
        "Fast of Theophany (Christmas)",
        PeriodKind::Fast,
        fixed(12, 30),
        fixed(1, 4),
        "Armenian Apostolic Church of Holy Resurrection, Sydney (armenian-church-sydney-fasts), \
         for its six days; the Western Prelacy (westernprelacy-fasts), for 30 December to 4 January",
    ),
    period(
        "catechumens-fast",
        "Fast of the Catechumens (Aradjavorats)",
        PeriodKind::Fast,
        from_pascha(-69),
        from_pascha(-65),
        GEORGIA,
    ),
    period(
        "great-lent",
        "Great Lent and Holy Week",
        PeriodKind::Fast,
        from_pascha(-48),
        from_pascha(-1),
        GEORGIA,
    ),
    period(
        "elijah-fast",
        "Fast of Elijah",
        PeriodKind::Fast,
        from_pascha(50),
        from_pascha(54),
        GEORGIA,
    ),
    period(
        "st-gregory-fast",
        "Fast of St Gregory the Illuminator",
        PeriodKind::Fast,
        from_pascha(71),
        from_pascha(75),
        GEORGIA,
    ),
    period(
        "transfiguration-fast",
        "Fast of the Transfiguration (Vardavar)",
        PeriodKind::Fast,
        from_pascha(92),
        from_pascha(96),
        GEORGIA,
    ),
    period(
        "assumption-fast",
        "Fast of the Assumption",
        PeriodKind::Fast,
        nearest(8, 15, -6),
        nearest(8, 15, -2),
        GEORGIA,
    ),
    period(
        "exaltation-fast",
        "Fast of the Exaltation of the Cross",
        PeriodKind::Fast,
        nearest(9, 14, -6),
        nearest(9, 14, -2),
        GEORGIA,
    ),
    period(
        "varak-fast",
        "Fast of the Holy Cross of Varak",
        PeriodKind::Fast,
        nearest(9, 14, 8),
        nearest(9, 14, 12),
        GEORGIA,
    ),
    period(
        "advent-fast",
        "Fast of the first week of Advent (Hisnag)",
        PeriodKind::Fast,
        nearest(11, 18, 1),
        nearest(11, 18, 5),
        GEORGIA,
    ),
    period(
        "st-james-fast",
        "Fast of St James of Nisibis",
        PeriodKind::Fast,
        nearest(11, 18, 22),
        nearest(11, 18, 26),
        GEORGIA,
    ),
];

// ─────────────────────────────────────────────────────────────────────────
// The Coptic Orthodox Church
// ─────────────────────────────────────────────────────────────────────────

/// St-Takla's answer on the fasts, as `source` strings cite it.
const ST_TAKLA: &str = "St-Takla.org, the fasts of the first and second degree (st-takla-fasts)";

/// Where `coptic-fasts` takes its periods from.
pub const COPTIC_SOURCES: &str = "St-Takla.org (the Church of St Takla Haymanot, Alexandria), \
    the fasts of the first and second degree (st-takla-fasts), for each fast's length and days, \
    the Wednesdays and Fridays and their exemptions; its rites of the Nativity Fast, for the \
    Nativity on 7 January, 28 Koiak in a Gregorian leap year (st-takla-nativity-fast), and of \
    the Paramoun, for its days before a Sunday or Monday feast (st-takla-paramoun); \
    CopticChurch.net, \"Introduction to the Coptic Church\" (copticchurch-net-fasts), for the \
    Apostles' Fast from the day after Pentecost to 5 Epip; checked against the Coptic Orthodox \
    Metropolis of the Southern United States's fasts and feasts of 2023-2028 (suscopts-fasts); \
    all retrieved 2026-09-29";

/// The Coptic fasts: the fast-free days first.
pub static COPTIC: &[Period] = &[
    period(
        "holy-fifty",
        "The Holy Fifty days",
        PeriodKind::FastFree,
        from_pascha(0),
        from_pascha(49),
        ST_TAKLA,
    ),
    period(
        "nativity-feast",
        "The Nativity",
        PeriodKind::FastFree,
        Bound::InCalendar {
            calendar: CalendarSystem::GREGORIAN,
            month: 1,
            day: 7,
        },
        Bound::InCalendar {
            calendar: CalendarSystem::GREGORIAN,
            month: 1,
            day: 7,
        },
        "St-Takla.org, the rite of the Nativity Fast (st-takla-nativity-fast)",
    ),
    period(
        "theophany-feast",
        "Theophany",
        PeriodKind::FastFree,
        coptic(5, 11),
        coptic(5, 11),
        ST_TAKLA,
    ),
    period(
        "nativity-fast",
        "The Nativity Fast",
        PeriodKind::Fast,
        coptic(3, 16),
        Bound::InCalendar {
            calendar: CalendarSystem::GREGORIAN,
            month: 1,
            day: 6,
        },
        ST_TAKLA,
    ),
    period(
        "jonah-fast",
        "The Fast of Jonah (Nineveh)",
        PeriodKind::Fast,
        from_pascha(-69),
        from_pascha(-67),
        ST_TAKLA,
    ),
    period(
        "great-lent",
        "Great Lent and Holy Week",
        PeriodKind::Fast,
        from_pascha(-55),
        from_pascha(-1),
        ST_TAKLA,
    ),
    period(
        "apostles-fast",
        "The Apostles' Fast",
        PeriodKind::Fast,
        from_pascha(50),
        coptic(11, 4),
        ST_TAKLA,
    ),
    period(
        "virgin-fast",
        "The Fast of the Virgin",
        PeriodKind::Fast,
        coptic(12, 1),
        coptic(12, 15),
        ST_TAKLA,
    ),
    period(
        "theophany-paramoun",
        "The Paramoun of Theophany",
        PeriodKind::Fast,
        Bound::Paramoun {
            calendar: CalendarSystem::COPTIC,
            month: 5,
            day: 11,
        },
        coptic(5, 10),
        "St-Takla.org, the rite of the Paramoun (st-takla-paramoun)",
    ),
];

// ─────────────────────────────────────────────────────────────────────────
// The Ethiopian Orthodox Tewahedo Church
// ─────────────────────────────────────────────────────────────────────────

/// The church's calendar page, as `source` strings cite it.
/// Where Genna's kept date comes from.
const GENNA_KEPT: &str = "Wikipedia, \"Public holidays in Ethiopia\", Genna on 7 January, where \
     Meskel has 28 September \"(leap year)\" beside its 27th, retrieved 2026-09-29 (secondary)";

/// A day of the Gregorian calendar, as the kept Genna is.
const fn gregorian(month: u8, day: u8) -> Bound {
    Bound::InCalendar {
        calendar: CalendarSystem::GREGORIAN,
        month,
        day,
    }
}

const ETHIOPIAN_ORTHODOX_ORG: &str =
    "ethiopianorthodox.org, the calendar page (ethiopianorthodox-org-calendar)";
/// Mahibere Kidusan's reckoning of the fasts of 2011 E.C.
const MAHIBERE_KIDUSAN: &str =
    "Mahibere Kidusan, the Bahire Hasab of the fasts of 2011 E.C. (eotcmk-fasts-2011)";

/// Where `ethiopian-fasts` takes its periods from.
pub const ETHIOPIAN_SOURCES: &str = "ethiopianorthodox.org, the calendar page \
    (ethiopianorthodox.org/english/calendar.html, ethiopianorthodox-org-calendar), for the \
    fasts, Tsome Nebiyat from 15 Hidar to 28 Tahsas, the Gahad of Timkat, Tsome Nenewe, the \
    Wednesdays and Fridays and the fifty days after Easter; Mahibere Kidusan, the Sunday School \
    Department of the Ethiopian Orthodox Tewahedo Church (eotcmk.org), its Bahire Hasab of the \
    fasts of 2011 E.C. (eotcmk-fasts-2011), for the offsets of Abiy Tsom and Tsome Hawaryat from \
    Nenewe and Tsome Filseta from 1 Nehase, and its articles on Tsome Hawaryat to 5 Hamle and \
    Tsome Filseta of 1-15 Nehase (eotcmk-hawaryat-2023, eotcmk-filseta-2016); Keraneyo Medhane \
    Alem, \"The Order of Fasts\" (eotc-ma-fasts), for Tsome Hawaryat from the day after \
    Paraclete and the Wednesdays and Fridays lifted on Genna and Timkat; all retrieved \
    2026-09-29";

/// The Ethiopian fasts: the fast-free days first.
pub static ETHIOPIAN: &[Period] = &[
    period(
        "fifty-days",
        "The fifty days from Tinsae to Paraclete",
        PeriodKind::FastFree,
        from_pascha(0),
        from_pascha(49),
        ETHIOPIAN_ORTHODOX_ORG,
    ),
    // Genna as it is kept, on 7 January every year, as the Coptic Nativity
    // is: 29 Tahsas, the calendrical date, is 8 January after an Ethiopic
    // leap year.
    period(
        "genna",
        "Genna",
        PeriodKind::FastFree,
        gregorian(1, 7),
        gregorian(1, 7),
        GENNA_KEPT,
    ),
    period(
        "timkat",
        "Timkat",
        PeriodKind::FastFree,
        ethiopic(5, 11),
        ethiopic(5, 11),
        ETHIOPIAN_ORTHODOX_ORG,
    ),
    period(
        "tsome-nebiyat",
        "Tsome Nebiyat (the Fast of the Prophets)",
        PeriodKind::Fast,
        ethiopic(3, 15),
        gregorian(1, 6),
        ETHIOPIAN_ORTHODOX_ORG,
    ),
    period(
        "gahad-of-timkat",
        "Gahad of Timkat",
        PeriodKind::Fast,
        ethiopic(5, 10),
        ethiopic(5, 10),
        ETHIOPIAN_ORTHODOX_ORG,
    ),
    period(
        "tsome-nenewe",
        "Tsome Nenewe (the Fast of Nineveh)",
        PeriodKind::Fast,
        from_pascha(-69),
        from_pascha(-67),
        ETHIOPIAN_ORTHODOX_ORG,
    ),
    period(
        "abiy-tsom",
        "Abiy Tsom (Great Lent)",
        PeriodKind::Fast,
        from_pascha(-55),
        from_pascha(-1),
        MAHIBERE_KIDUSAN,
    ),
    period(
        "tsome-hawaryat",
        "Tsome Hawaryat (the Fast of the Apostles)",
        PeriodKind::Fast,
        from_pascha(50),
        ethiopic(11, 4),
        MAHIBERE_KIDUSAN,
    ),
    period(
        "tsome-filseta",
        "Tsome Filseta (the Fast of the Assumption)",
        PeriodKind::Fast,
        ethiopic(12, 1),
        ethiopic(12, 15),
        MAHIBERE_KIDUSAN,
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orthodox_fasts::{Reckoning, Status, span, status};
    use hc_calendar::Rd;
    use hc_calendars_solar::gregorian;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    fn by_id(reckoning: &Reckoning, id: &str) -> &'static Period {
        reckoning
            .periods
            .iter()
            .find(|period| period.id == id)
            .unwrap_or_else(|| panic!("{id}"))
    }

    fn span_of(reckoning: &Reckoning, id: &str, year: i64) -> (Rd, Rd) {
        span(reckoning, by_id(reckoning, id), year).unwrap_or_else(|| panic!("{id} {year}"))
    }

    fn in_period(reckoning: &Reckoning, day: Rd) -> Option<&'static str> {
        match status(reckoning, day)? {
            Status::InPeriod(period) => Some(period.id),
            _ => None,
        }
    }

    #[test]
    fn every_scheme_names_its_periods_once() {
        for reckoning in Reckoning::ALL {
            for (i, period) in reckoning.periods.iter().enumerate() {
                assert!(
                    reckoning.periods[..i].iter().all(|p| p.id != period.id),
                    "{} {}",
                    reckoning.id,
                    period.id
                );
            }
        }
    }

    /// The Armenian Diocese of Georgia's eves of 2025 and 2026, each fast
    /// the Monday to Friday after (`armenian-church-georgia-fasts`); the
    /// Mother See's Fast of the Catechumens, "January 25-30" with its eve,
    /// and Great Lent, "February 16 - April 04", of 2026.
    #[test]
    fn the_armenian_fasts_follow_the_eves_the_diocese_prints() {
        let armenian = &Reckoning::ARMENIAN;
        for (id, eves) in [
            ("catechumens-fast", [(2025, 2, 9), (2026, 1, 25)]),
            ("great-lent", [(2025, 3, 2), (2026, 2, 15)]),
            ("st-gregory-fast", [(2025, 6, 29), (2026, 6, 14)]),
            ("transfiguration-fast", [(2025, 7, 20), (2026, 7, 5)]),
            ("assumption-fast", [(2025, 8, 10), (2026, 8, 9)]),
            ("varak-fast", [(2025, 9, 21), (2026, 9, 20)]),
            ("advent-fast", [(2025, 11, 16), (2026, 11, 15)]),
            ("st-james-fast", [(2025, 12, 7), (2026, 12, 6)]),
        ] {
            for (year, month, day) in eves {
                let (first, _) = span_of(armenian, id, year);
                assert_eq!(first, Rd(ymd(year, month, day).0 + 1), "{id} {year}");
            }
        }
        assert_eq!(
            span_of(armenian, "catechumens-fast", 2026),
            (ymd(2026, 1, 26), ymd(2026, 1, 30))
        );
        assert_eq!(
            span_of(armenian, "great-lent", 2026),
            (ymd(2026, 2, 16), ymd(2026, 4, 4))
        );
        // The Exaltation's eve of 2026, 6 September; Elijah's fast the
        // week after Pentecost, 24 May 2026.
        assert_eq!(
            span_of(armenian, "exaltation-fast", 2026).0,
            ymd(2026, 9, 7)
        );
        assert_eq!(
            span_of(armenian, "elijah-fast", 2026),
            (ymd(2026, 5, 25), ymd(2026, 5, 29))
        );
        assert_eq!(
            span_of(armenian, "theophany-fast", 2026),
            (ymd(2026, 12, 30), ymd(2027, 1, 4))
        );
    }

    /// The Yerevan church of St Zoravor's Tonatsuyts of 2026, whose "old
    /// calendar" column dates each eve by the Julian calendar and
    /// computus (`surb-zoravor-tonatsuyts-2026`): the Jerusalem reckoning's
    /// fasts begin the day after.
    #[test]
    fn the_jerusalem_reckoning_is_the_old_calendar_column() {
        let jerusalem = &Reckoning::ARMENIAN_JERUSALEM;
        for (id, (month, day)) in [
            ("catechumens-fast", (2, 1)),
            ("great-lent", (2, 22)),
            ("elijah-fast", (5, 31)),
            ("st-gregory-fast", (6, 21)),
            ("transfiguration-fast", (7, 12)),
            ("assumption-fast", (8, 23)),
            ("exaltation-fast", (9, 20)),
            ("varak-fast", (10, 4)),
            ("advent-fast", (11, 29)),
            ("st-james-fast", (12, 20)),
        ] {
            assert_eq!(
                span_of(jerusalem, id, 2026).0,
                Rd(ymd(2026, month, day).0 + 1),
                "{id}"
            );
        }
        // Its Christmas eve, 29 December Julian, is 11 January 2027.
        assert_eq!(
            span_of(jerusalem, "theophany-fast", 2026).0,
            ymd(2027, 1, 12)
        );
    }

    #[test]
    fn the_armenian_weekly_fasts_are_lifted_after_easter_and_after_theophany() {
        let armenian = &Reckoning::ARMENIAN;
        // Easter 5 April 2026: Wednesday 8 April is free, and Wednesday
        // 20 May, after the Ascension on 14 May, a fast.
        assert_eq!(
            in_period(armenian, ymd(2026, 4, 8)),
            Some("easter-to-ascension")
        );
        assert_eq!(
            status(armenian, ymd(2026, 5, 20)),
            Some(Status::WeeklyFast(hc_calendar::Weekday::Wednesday))
        );
        assert_eq!(
            in_period(armenian, ymd(2027, 1, 8)),
            Some("theophany-octave")
        );
        assert!(!status(armenian, ymd(2027, 1, 8)).unwrap().is_fast_day());
    }

    /// The Coptic Metropolis of the Southern United States's fasts of
    /// 2023–2028 (`suscopts-fasts`): Jonah's, the Great Fast's first day,
    /// the Apostles', the Virgin's and the Nativity's, first and last day.
    #[test]
    fn the_coptic_fasts_are_the_metropolis_calendar_of_2023_to_2028() {
        let coptic = &Reckoning::COPTIC;
        type Year = (i64, (u8, u8), (u8, u8), (u8, u8), (u8, u8), (u8, u8));
        let years: [Year; 6] = [
            (2023, (2, 6), (2, 20), (6, 5), (8, 7), (11, 26)),
            (2024, (2, 26), (3, 11), (6, 24), (8, 7), (11, 25)),
            (2025, (2, 10), (2, 24), (6, 9), (8, 7), (11, 25)),
            (2026, (2, 2), (2, 16), (6, 1), (8, 7), (11, 25)),
            (2027, (2, 22), (3, 8), (6, 21), (8, 7), (11, 26)),
            (2028, (2, 7), (2, 21), (6, 5), (8, 7), (11, 25)),
        ];
        for (year, jonah, lent, apostles, virgin, nativity) in years {
            let day = |(month, date): (u8, u8)| ymd(year, month, date);
            assert_eq!(
                span_of(coptic, "jonah-fast", year),
                (day(jonah), Rd(day(jonah).0 + 2)),
                "{year}"
            );
            assert_eq!(span_of(coptic, "great-lent", year).0, day(lent), "{year}");
            assert_eq!(
                span_of(coptic, "apostles-fast", year),
                (day(apostles), ymd(year, 7, 11)),
                "{year}"
            );
            assert_eq!(
                span_of(coptic, "virgin-fast", year),
                (day(virgin), ymd(year, 8, 21)),
                "{year}"
            );
            assert_eq!(
                span_of(coptic, "nativity-fast", year),
                (day(nativity), ymd(year + 1, 1, 6)),
                "{year}"
            );
        }
    }

    #[test]
    fn the_coptic_paramoun_runs_back_from_a_sunday_or_monday_theophany() {
        let coptic = &Reckoning::COPTIC;
        // 11 Tobi: Sunday 19 January 2025, the Paramoun Friday and
        // Saturday; Monday 19 January 2026, Friday to Sunday; Saturday
        // 20 January 2024, the eve alone.
        assert_eq!(
            span_of(coptic, "theophany-paramoun", 2025),
            (ymd(2025, 1, 17), ymd(2025, 1, 18))
        );
        assert_eq!(
            span_of(coptic, "theophany-paramoun", 2026),
            (ymd(2026, 1, 16), ymd(2026, 1, 18))
        );
        assert_eq!(
            span_of(coptic, "theophany-paramoun", 2024),
            (ymd(2024, 1, 19), ymd(2024, 1, 19))
        );
        // The Holy Fifty lift the Wednesday after Easter; Theophany on a
        // Wednesday lifts it too: 19 January 2022.
        assert!(!status(coptic, ymd(2026, 4, 15)).unwrap().is_fast_day());
        assert_eq!(in_period(coptic, ymd(2022, 1, 19)), Some("theophany-feast"));
    }

    /// Mahibere Kidusan's dates (`eotcmk-great-lent-2016`,
    /// `eotcmk-great-lent-2017`, `eotcmk-nineveh-2017`,
    /// `eotcmk-nebiyat-2022`, `eotcmk-fasts-2011`).
    #[test]
    fn the_ethiopian_fasts_are_the_dates_mahibere_kidusan_prints() {
        let ethiopian = &Reckoning::ETHIOPIAN;
        // Abiy Tsom "on Monday Yekatit 28, 2008 E.C.( March 7, 2016)" and
        // "Monday Yekatit 13, 2009 E.C.( February 20, 2017)".
        assert_eq!(span_of(ethiopian, "abiy-tsom", 2016).0, ymd(2016, 3, 7));
        assert_eq!(span_of(ethiopian, "abiy-tsom", 2017).0, ymd(2017, 2, 20));
        // Nenewe "begins on Monday, February 6 and ends February 8" 2017.
        assert_eq!(
            span_of(ethiopian, "tsome-nenewe", 2017),
            (ymd(2017, 2, 6), ymd(2017, 2, 8))
        );
        // Nebiyat from "November 24, (Hidar 15) 2022" to "January 6,
        // (Tahisas 28), 2023".
        assert_eq!(
            span_of(ethiopian, "tsome-nebiyat", 2022),
            (ymd(2022, 11, 24), ymd(2023, 1, 6))
        );
        // 2011 E.C.: Nenewe from 11 Yekatit and Tsome Hawaryat from
        // 10 Sene, 18 February and 17 June 2019.
        assert_eq!(span_of(ethiopian, "tsome-nenewe", 2019).0, ymd(2019, 2, 18));
        assert_eq!(
            span_of(ethiopian, "tsome-hawaryat", 2019).0,
            ymd(2019, 6, 17)
        );
        // Tsome Filseta, 1–15 Nehase: 7–21 August 2026.
        assert_eq!(
            span_of(ethiopian, "tsome-filseta", 2026),
            (ymd(2026, 8, 7), ymd(2026, 8, 21))
        );
        // Timkat's eve is a fast; Timkat on a Wednesday lifts the weekly
        // fast: 19 January 2022.
        assert_eq!(
            in_period(ethiopian, ymd(2026, 1, 18)),
            Some("gahad-of-timkat")
        );
        assert_eq!(in_period(ethiopian, ymd(2022, 1, 19)), Some("timkat"));
    }

    /// After the Ethiopic leap year 2015 E.C., 29 Tahsas, Genna's
    /// calendrical date, is 8 January 2024; Genna is kept on the 7th, and
    /// the fast ends on the 6th, as the Coptic Nativity Fast does: the
    /// fast does not reach the day Genna is kept.
    #[test]
    fn the_ethiopian_nativity_fast_ends_the_day_before_genna_is_kept() {
        let ethiopian = &Reckoning::ETHIOPIAN;
        for (year, first) in [
            (2023, ymd(2023, 11, 25)),
            (2027, ymd(2027, 11, 25)),
            (2024, ymd(2024, 11, 24)),
        ] {
            assert_eq!(
                span_of(ethiopian, "tsome-nebiyat", year),
                (first, ymd(year + 1, 1, 6)),
                "{year}"
            );
            assert_eq!(in_period(ethiopian, ymd(year + 1, 1, 7)), Some("genna"));
            assert!(
                !status(ethiopian, ymd(year + 1, 1, 7))
                    .unwrap()
                    .is_fast_day()
            );
        }
        // Monday 8 January 2024 is in no period and no fast day.
        assert_eq!(in_period(ethiopian, ymd(2024, 1, 8)), None);
        assert!(!status(ethiopian, ymd(2024, 1, 8)).unwrap().is_fast_day());
    }
}
