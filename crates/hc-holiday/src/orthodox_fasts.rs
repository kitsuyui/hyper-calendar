//! The Eastern Orthodox fasts: the four fasting seasons, the Wednesday and
//! Friday fasts, the three one-day fasts and the fast-free weeks.
//!
//! `docs/systems/orthodox-fasts.md` in the repository describes the
//! scheme, works 2025 by hand and states the sources; this page states the
//! code's own facts.
//!
//! The scheme is the OCA's outline "Fasting & Fast-Free Seasons of the
//! Church" (`oca-fasting-seasons`): twelve [`Period`]s, each bounded by an
//! offset from Pascha or by a fixed date, tested in order, the fast-free
//! ones first, and every other Wednesday and Friday a fast day. The
//! Apostles' Fast runs from the Monday after All Saints' Sunday to 28 June
//! (`wikipedia-apostles-fast`), so it is the one period whose length
//! changes, and on the Revised Julian reckoning it can vanish: "If this
//! fast coincides with the feast directly, it is eliminated"
//! (`antioch-apostles-fast`). [`span`] then returns nothing.
//!
//! The two [`Reckoning`]s differ only in the calendar the fixed dates are
//! read in, and both keep Pascha by the Julian computus
//! (`docs/policy.md` §5):
//!
//! | Identifier | Fixed dates in |
//! | --- | --- |
//! | `orthodox-fasts` | the Julian calendar |
//! | `orthodox-fasts-revised-julian` | the Revised Julian calendar |
//!
//! Only whether a day is a fast day is carried, not what may be eaten on
//! it, which differs from church to church and from feast to feast.

use hc_calendar::{Month, Rd, Weekday};

use crate::computus::{COMPUTUS_LAST_YEAR, JULIAN_COMPUTUS_FIRST_YEAR, orthodox_easter};
use crate::rule::CalendarSystem;

/// A reckoning of the fasts: the calendar its fixed dates are read in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reckoning {
    /// The identifier.
    pub id: &'static str,
    /// Its name in English.
    pub english_name: &'static str,
    /// The calendar the fixed dates of the [`Period`]s are dates of.
    pub fixed: CalendarSystem,
    /// Where it comes from.
    pub sources: &'static str,
}

/// The sources every reckoning shares.
const SCHEME: &str = "OCA, \"Fasting & Fast-Free Seasons of the Church\" \
    (oca.org/liturgics/outlines/fasting-fast-free-seasons-of-the-church, oca-fasting-seasons); \
    Wikipedia, \"Apostles' Fast\" (wikipedia-apostles-fast); Patriarchate of Antioch, \
    \"The Apostles' Fast\" (antiochpatriarchate.org, antioch-apostles-fast); all retrieved \
    2026-09-27. Pascha by the Julian computus, as `computus` computes it";

hc_core::catalogue! {
    type: Reckoning,
    id: |reckoning| reckoning.id,
    provenance: |reckoning| reckoning.sources,
    tests: reckoning_catalogue_tests,
    associated;

    /// The two reckonings.
    pub const ALL;
    /// The reckoning with this identifier.
    pub fn by_id;

    entries: {
        /// The fixed dates in the Julian calendar, as the churches that did
        /// not take up the Revised Julian calendar keep them: thirteen days
        /// after the Gregorian dates until 2100.
        pub const JULIAN = Self {
            id: "orthodox-fasts",
            english_name: "Eastern Orthodox fasts (Julian calendar)",
            fixed: CalendarSystem::JULIAN,
            sources: SCHEME,
        };
        /// The fixed dates in the Revised Julian calendar, which are the
        /// Gregorian dates from 1600 to 2800.
        pub const REVISED_JULIAN = Self {
            id: "orthodox-fasts-revised-julian",
            english_name: "Eastern Orthodox fasts (Revised Julian calendar)",
            fixed: CalendarSystem::REVISED_JULIAN,
            sources: SCHEME,
        };
    }
}

/// Whether a period is a fast or lifts the weekly fasts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PeriodKind {
    /// Every day of the period is a fast day.
    Fast,
    /// No day of the period is a fast day, Wednesday and Friday included.
    FastFree,
}

/// Where a period starts or ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bound {
    /// A number of days from Pascha, negative before it.
    FromPascha(i16),
    /// A fixed month and day in the reckoning's calendar.
    Fixed {
        /// The month, 1 to 12.
        month: u8,
        /// The day of the month.
        day: u8,
    },
}

/// A period of the scheme: a fasting season, a one-day fast or a
/// fast-free week.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Period {
    /// The identifier.
    pub id: &'static str,
    /// Its name in English, as the OCA's outline gives it.
    pub english_name: &'static str,
    /// Whether it is a fast or lifts the fasts.
    pub kind: PeriodKind,
    /// The first day.
    pub first: Bound,
    /// The last day, included.
    pub last: Bound,
    /// Where it comes from.
    pub source: &'static str,
}

/// The outline, as `source` strings cite it.
const OUTLINE: &str = "OCA, \"Fasting & Fast-Free Seasons of the Church\" (oca-fasting-seasons)";

const fn fixed(month: u8, day: u8) -> Bound {
    Bound::Fixed { month, day }
}

hc_core::catalogue! {
    type: Period,
    id: |period| period.id,
    provenance: |period| period.source,
    tests: period_catalogue_tests,
    associated;

    /// The twelve periods, in the order a day is tested against them: the
    /// fast-free ones first, since they lift the weekly fasts, then the
    /// seasons and the one-day fasts.
    pub const ALL;
    /// The period with this identifier.
    pub fn by_id;

    entries: {
        /// From the Nativity to the eve of Theophany.
        pub const CHRISTMASTIDE = Self {
            id: "christmastide",
            english_name: "Afterfeast of Nativity to Theophany Eve",
            kind: PeriodKind::FastFree,
            first: fixed(12, 25),
            last: fixed(1, 4),
            source: OUTLINE,
        };
        /// The week after the Sunday of the Publican and the Pharisee, the
        /// tenth before Pascha.
        pub const PUBLICAN_AND_PHARISEE = Self {
            id: "publican-and-pharisee-week",
            english_name: "Week following Sunday of Publican & Pharisee",
            kind: PeriodKind::FastFree,
            first: Bound::FromPascha(-69),
            last: Bound::FromPascha(-64),
            source: OUTLINE,
        };
        /// The week after Pascha, to St Thomas Sunday.
        pub const BRIGHT_WEEK = Self {
            id: "bright-week",
            english_name: "Bright Week",
            kind: PeriodKind::FastFree,
            first: Bound::FromPascha(1),
            last: Bound::FromPascha(6),
            source: OUTLINE,
        };
        /// The week after Pentecost, to the Saturday before All Saints'.
        pub const TRINITY_WEEK = Self {
            id: "trinity-week",
            english_name: "Trinity Week",
            kind: PeriodKind::FastFree,
            first: Bound::FromPascha(50),
            last: Bound::FromPascha(55),
            source: OUTLINE,
        };
        /// The week before Great Lent, from the Monday after the Sunday of
        /// the Last Judgment through Cheesefare Sunday, when meat is not
        /// eaten.
        pub const MEATFAST = Self {
            id: "meatfast",
            english_name: "Meatfast",
            kind: PeriodKind::Fast,
            first: Bound::FromPascha(-55),
            last: Bound::FromPascha(-49),
            source: OUTLINE,
        };
        /// From the first Monday of Great Lent through Holy Saturday.
        pub const GREAT_LENT = Self {
            id: "great-lent",
            english_name: "Great Lent & Holy Week",
            kind: PeriodKind::Fast,
            first: Bound::FromPascha(-48),
            last: Bound::FromPascha(-1),
            source: OUTLINE,
        };
        /// From the Monday after All Saints' Sunday to the eve of Peter and
        /// Paul, when that Monday comes first.
        pub const APOSTLES = Self {
            id: "apostles-fast",
            english_name: "Apostles' (Peter & Paul) Fast",
            kind: PeriodKind::Fast,
            first: Bound::FromPascha(57),
            last: fixed(6, 28),
            source: "Wikipedia, \"Apostles' Fast\" (wikipedia-apostles-fast), for the first \
                     and last day; Patriarchate of Antioch (antioch-apostles-fast), for the years \
                     it vanishes",
        };
        /// Before the Dormition.
        pub const DORMITION = Self {
            id: "dormition-fast",
            english_name: "Dormition (Theotokos) Fast",
            kind: PeriodKind::Fast,
            first: fixed(8, 1),
            last: fixed(8, 14),
            source: OUTLINE,
        };
        /// Before the Nativity: St Philip's Fast.
        pub const NATIVITY = Self {
            id: "nativity-fast",
            english_name: "Nativity (St. Philip's Fast)",
            kind: PeriodKind::Fast,
            first: fixed(11, 15),
            last: fixed(12, 24),
            source: OUTLINE,
        };
        /// The eve of Theophany.
        pub const THEOPHANY_EVE = Self {
            id: "theophany-eve",
            english_name: "Theophany Eve",
            kind: PeriodKind::Fast,
            first: fixed(1, 5),
            last: fixed(1, 5),
            source: OUTLINE,
        };
        /// The Beheading of St John the Baptist.
        pub const BEHEADING = Self {
            id: "beheading-of-john-the-baptist",
            english_name: "Beheading of St. John the Baptist",
            kind: PeriodKind::Fast,
            first: fixed(8, 29),
            last: fixed(8, 29),
            source: OUTLINE,
        };
        /// The Elevation (Exaltation) of the Cross.
        pub const ELEVATION_OF_THE_CROSS = Self {
            id: "elevation-of-the-cross",
            english_name: "Elevation of the Cross",
            kind: PeriodKind::Fast,
            first: fixed(9, 14),
            last: fixed(9, 14),
            source: OUTLINE,
        };
    }
}

/// What a day is in the scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The day is in a period: a fast day if the period is a
    /// [`PeriodKind::Fast`], not one if it is [`PeriodKind::FastFree`].
    InPeriod(&'static Period),
    /// A Wednesday or Friday in no period: a fast day.
    WeeklyFast(Weekday),
    /// Any other day: not a fast day.
    NotFasting,
}

impl Status {
    /// Whether the day is a fast day.
    #[must_use]
    pub const fn is_fast_day(self) -> bool {
        match self {
            Self::InPeriod(period) => matches!(period.kind, PeriodKind::Fast),
            Self::WeeklyFast(_) => true,
            Self::NotFasting => false,
        }
    }
}

/// The day a bound falls on in a year of the reckoning.
fn resolve(bound: Bound, reckoning: &Reckoning, year: i64) -> Option<Rd> {
    match bound {
        Bound::FromPascha(offset) => {
            orthodox_easter(year).map(|pascha| Rd(pascha.0 + i64::from(offset)))
        }
        Bound::Fixed { month, day } => reckoning.fixed.to_fixed(year, Month::regular(month), day),
    }
}

/// The first and last day of a period that begins in a year of the
/// reckoning, both included.
///
/// A period of two fixed dates whose last is earlier in the year than its
/// first, as Christmastide's is, ends in the next year. A period with a
/// bound counted from Pascha whose last day comes before its first does
/// not happen that year, which is the Apostles' Fast on the Revised Julian
/// reckoning when Pascha is late: `None`. So is a year outside 326 to
/// 4099, whose Pascha the Julian computus does not give.
#[must_use]
pub fn span(reckoning: &Reckoning, period: &Period, year: i64) -> Option<(Rd, Rd)> {
    if !(JULIAN_COMPUTUS_FIRST_YEAR..=COMPUTUS_LAST_YEAR).contains(&year) {
        return None;
    }
    let first = resolve(period.first, reckoning, year)?;
    let last = resolve(period.last, reckoning, year)?;
    if last >= first {
        return Some((first, last));
    }
    match (period.first, period.last) {
        (Bound::Fixed { .. }, Bound::Fixed { .. }) => {
            Some((first, resolve(period.last, reckoning, year + 1)?))
        }
        _ => None,
    }
}

/// What a day is in the scheme of a reckoning: in a period, a weekly
/// fast, or neither.
///
/// Returns `None` outside the years 326 to 4099 of the reckoning's
/// calendar, whose Pascha the Julian computus gives.
#[must_use]
pub fn status(reckoning: &Reckoning, day: Rd) -> Option<Status> {
    let year = reckoning.fixed.year_containing(day)?;
    if !(JULIAN_COMPUTUS_FIRST_YEAR..=COMPUTUS_LAST_YEAR).contains(&year) {
        return None;
    }
    for period in Period::ALL {
        for begun in [year - 1, year] {
            if let Some((first, last)) = span(reckoning, period, begun)
                && first <= day
                && day <= last
            {
                return Some(Status::InPeriod(period));
            }
        }
    }
    let weekday = Weekday::from_rd(day);
    Some(match weekday {
        Weekday::Wednesday | Weekday::Friday => Status::WeeklyFast(weekday),
        _ => Status::NotFasting,
    })
}

/// Whether a day is a fast day on a reckoning, or `None` outside its
/// years.
#[must_use]
pub fn is_fast_day(reckoning: &Reckoning, day: Rd) -> Option<bool> {
    status(reckoning, day).map(Status::is_fast_day)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendars_solar::gregorian;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    fn period_on(reckoning: &Reckoning, day: Rd) -> Option<&'static str> {
        match status(reckoning, day) {
            Some(Status::InPeriod(period)) => Some(period.id),
            _ => None,
        }
    }

    /// The Holy Trinity Russian Orthodox Church's calendar, a parish of
    /// the Moscow Patriarchate on the Julian calendar
    /// (`holy-trinity-calendar`, retrieved 2026-09-27): each Gregorian day,
    /// the period the page names, if any, and whether it marks a fast.
    #[test]
    fn the_julian_reckoning_agrees_with_a_moscow_patriarchate_calendar() {
        let julian = &Reckoning::JULIAN;
        for (year, month, day, period, fast) in [
            // "A fast-free week. ... Fast-free"
            (2025, 2, 12, Some("publican-and-pharisee-week"), false),
            // "Meatfare week. ... Fast. Fish Allowed"
            (2025, 2, 19, None, true),
            // "Cheesefare week (Maslenitsa) ... Meat is excluded"
            (2025, 2, 26, Some("meatfast"), true),
            (2025, 3, 2, Some("meatfast"), true),
            // "Beginning of the Great Lent"
            (2025, 3, 3, Some("great-lent"), true),
            (2025, 4, 19, Some("great-lent"), true),
            // Pascha: "The End of the Great Lent"
            (2025, 4, 20, None, false),
            // "Bright Wednesday. Bright Week. Fast-free"
            (2025, 4, 23, Some("bright-week"), false),
            // "Afterfeast of Pentecost. ... Fast-free Week"
            (2025, 6, 11, Some("trinity-week"), false),
            (2025, 6, 13, Some("trinity-week"), false),
            // All Saints: "Eve of Apostles' (Peter & Paul) Fast"
            (2025, 6, 15, None, false),
            (2025, 6, 16, Some("apostles-fast"), true),
            // 28 June Julian, the last day.
            (2025, 7, 11, Some("apostles-fast"), true),
            (2025, 7, 12, None, false),
            // A Wednesday: "Fast. Food with Oil"
            (2025, 8, 13, None, true),
            // 1 and 14 August Julian: "Dormition (Theotokos) Fast"
            (2025, 8, 14, Some("dormition-fast"), true),
            (2025, 8, 27, Some("dormition-fast"), true),
            (2025, 8, 28, None, false),
            // 29 August and 14 September Julian: "Fast"
            (2025, 9, 11, Some("beheading-of-john-the-baptist"), true),
            (2025, 9, 27, Some("elevation-of-the-cross"), true),
            // "Eve of the Nativity Fast", then "Nativity (St. Philip's Fast)"
            (2025, 11, 27, None, false),
            (2025, 11, 28, Some("nativity-fast"), true),
            (2026, 1, 6, Some("nativity-fast"), true),
            // 25 December and 1 January Julian: "Sviatki. Fast-free"
            (2026, 1, 7, Some("christmastide"), false),
            (2026, 1, 14, Some("christmastide"), false),
            // 5 January Julian, a Sunday: "Fast"
            (2026, 1, 18, Some("theophany-eve"), true),
            // "Fast. Fish Allowed"
            (2026, 1, 21, None, true),
        ] {
            let rd = ymd(year, month, day);
            assert_eq!(period_on(julian, rd), period, "{year}-{month}-{day}");
            assert_eq!(is_fast_day(julian, rd), Some(fast), "{year}-{month}-{day}");
        }
    }

    /// The OCA's daily pages, on the Revised Julian calendar
    /// (`oca-daily-readings`, retrieved 2026-09-27): "Beginning of the Great
    /// Fast" on 3 March 2025 and "BEGINNING OF THE APOSTLES FAST" on
    /// 16 June 2025 and 28 June 2027, the last a fast of one day.
    #[test]
    fn the_revised_julian_fasts_begin_when_the_oca_says() {
        let revised = &Reckoning::REVISED_JULIAN;
        assert_eq!(
            span(revised, &Period::GREAT_LENT, 2025).map(|s| s.0),
            Some(ymd(2025, 3, 3))
        );
        assert_eq!(
            span(revised, &Period::APOSTLES, 2025),
            Some((ymd(2025, 6, 16), ymd(2025, 6, 28)))
        );
        assert_eq!(
            span(revised, &Period::APOSTLES, 2027),
            Some((ymd(2027, 6, 28), ymd(2027, 6, 28)))
        );
        // The fixed fasts on the Gregorian dates.
        assert_eq!(
            span(revised, &Period::NATIVITY, 2025),
            Some((ymd(2025, 11, 15), ymd(2025, 12, 24)))
        );
        assert_eq!(
            span(revised, &Period::CHRISTMASTIDE, 2025),
            Some((ymd(2025, 12, 25), ymd(2026, 1, 4)))
        );
        assert_eq!(period_on(revised, ymd(2026, 1, 5)), Some("theophany-eve"));
    }

    /// Pascha on 5 May 2024 put the Monday after All Saints' on 1 July,
    /// after the feast of 29 June: "completely absent this year"
    /// (`trueorthodox-apostles-2024`). The same happened in 2002
    /// (`orthochristian-apostles-fast`).
    #[test]
    fn the_revised_julian_apostles_fast_vanished_in_2024() {
        let revised = &Reckoning::REVISED_JULIAN;
        assert_eq!(span(revised, &Period::APOSTLES, 2024), None);
        assert_eq!(span(revised, &Period::APOSTLES, 2002), None);
        // Trinity Week ran to 29 June, and the Monday after All Saints' is
        // an ordinary day, the Friday after it a weekly fast.
        assert_eq!(period_on(revised, ymd(2024, 6, 28)), Some("trinity-week"));
        assert_eq!(status(revised, ymd(2024, 7, 1)), Some(Status::NotFasting));
        assert_eq!(
            status(revised, ymd(2024, 7, 5)),
            Some(Status::WeeklyFast(Weekday::Friday))
        );
        assert_eq!(
            period_on(&Reckoning::JULIAN, ymd(2024, 7, 1)),
            Some("apostles-fast")
        );
        // The Julian reckoning kept it, to 28 June Julian, 11 July.
        assert_eq!(
            span(&Reckoning::JULIAN, &Period::APOSTLES, 2024),
            Some((ymd(2024, 7, 1), ymd(2024, 7, 11)))
        );
    }

    /// "It may be as short as eight days or as long as 42 days"
    /// (`wikipedia-apostles-fast`), over the whole 532-year Paschal cycle.
    #[test]
    fn the_apostles_fast_is_eight_to_forty_two_days_on_the_julian_reckoning() {
        let (shortest, longest) = (1900..2432).fold((i64::MAX, 0), |(low, high), year| {
            let (first, last) =
                span(&Reckoning::JULIAN, &Period::APOSTLES, year).expect("always kept");
            let length = last.0 - first.0 + 1;
            (low.min(length), high.max(length))
        });
        assert_eq!((shortest, longest), (8, 42));
    }

    #[test]
    fn the_two_reckonings_share_the_moveable_days() {
        for year in [1990, 2025, 2026, 2031] {
            for period in [
                &Period::PUBLICAN_AND_PHARISEE,
                &Period::MEATFAST,
                &Period::GREAT_LENT,
                &Period::BRIGHT_WEEK,
                &Period::TRINITY_WEEK,
            ] {
                assert_eq!(
                    span(&Reckoning::JULIAN, period, year),
                    span(&Reckoning::REVISED_JULIAN, period, year)
                );
            }
            // And the fixed ones thirteen days apart.
            let (julian, _) = span(&Reckoning::JULIAN, &Period::DORMITION, year).unwrap();
            let (revised, _) = span(&Reckoning::REVISED_JULIAN, &Period::DORMITION, year).unwrap();
            assert_eq!(julian.0 - revised.0, 13);
        }
        assert_eq!(Reckoning::by_id("orthodox-fasts"), Some(Reckoning::JULIAN));
        assert_eq!(Period::by_id("bright-week"), Some(Period::BRIGHT_WEEK));
    }

    #[test]
    fn no_answer_outside_the_computus() {
        let late = ymd(4200, 3, 1);
        assert_eq!(status(&Reckoning::JULIAN, late), None);
        assert_eq!(span(&Reckoning::JULIAN, &Period::GREAT_LENT, 4100), None);
        assert_eq!(span(&Reckoning::JULIAN, &Period::GREAT_LENT, 325), None);
        assert!(span(&Reckoning::JULIAN, &Period::GREAT_LENT, 4099).is_some());
    }
}
