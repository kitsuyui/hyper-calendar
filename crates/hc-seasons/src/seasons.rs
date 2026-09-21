//! The four seasons, under the three definitions that disagree.
//!
//! "When does summer start" has no single answer, and the three common
//! answers are a month apart from one another:
//!
//! | Definition | Northern summer begins | Because |
//! | --- | --- | --- |
//! | [`SeasonDefinition::Astronomical`] | ~21 June | the solstice |
//! | [`SeasonDefinition::Meteorological`] | 1 June | whole months group best for statistics |
//! | [`SeasonDefinition::EastAsian`] | ~6 May | 立夏, the solstice is *mid*-summer |
//!
//! The East Asian one is not a shifted version of the astronomical one: it
//! treats the solstices and equinoxes as the *middles* of their seasons,
//! which is what 夏至 ("summer's extreme") and 中秋 ("mid-autumn") actually
//! mean. That is why the Mid-Autumn Festival is in September.
//!
//! None of the three is the default here. A caller who does not say which
//! they mean has not decided yet.
//!
//! # Hemispheres
//!
//! The astronomical and meteorological definitions flip across the equator,
//! and [`Hemisphere`] does that. The East Asian definition does not really
//! have a southern form — it is a description of the Chinese agricultural
//! year — so flipping it is a mechanical courtesy, offered because refusing
//! would be more annoying than useful, and documented as such.
//!
//! The tropics have two seasons or six, not four, and nothing here describes
//! them.

use hc_calendar::Rd;

use crate::gregorian::month_from_rd;
use crate::meridian::Meridian;
use crate::solar_terms::{SolarTerm, term_day};

/// One of the four seasons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Season {
    /// Spring: 春, the season of growth.
    Spring,
    /// Summer: 夏.
    Summer,
    /// Autumn: 秋.
    Autumn,
    /// Winter: 冬.
    Winter,
}

impl Season {
    /// All four, in order from spring.
    pub const ALL: [Self; 4] = [Self::Spring, Self::Summer, Self::Autumn, Self::Winter];

    /// The next season, wrapping from winter to spring.
    #[must_use]
    pub const fn next(self) -> Self {
        match self {
            Self::Spring => Self::Summer,
            Self::Summer => Self::Autumn,
            Self::Autumn => Self::Winter,
            Self::Winter => Self::Spring,
        }
    }

    /// The previous season.
    #[must_use]
    pub const fn previous(self) -> Self {
        self.next().next().next()
    }

    /// The opposite season, which is the one the other hemisphere is in.
    #[must_use]
    pub const fn opposite(self) -> Self {
        self.next().next()
    }

    /// The 立 term that opens this season in the East Asian reckoning.
    ///
    /// 立春, 立夏, 立秋, 立冬 — the four "establishments", at 315°, 45°, 135°
    /// and 225°.
    #[must_use]
    pub const fn east_asian_opening_term(self) -> SolarTerm {
        match self {
            Self::Spring => SolarTerm::BEGINNING_OF_SPRING,
            Self::Summer => SolarTerm::BEGINNING_OF_SUMMER,
            Self::Autumn => SolarTerm::BEGINNING_OF_AUTUMN,
            Self::Winter => SolarTerm::BEGINNING_OF_WINTER,
        }
    }

    /// The equinox or solstice at the middle of this season in the East
    /// Asian reckoning, and at its start in the astronomical one.
    #[must_use]
    pub const fn cardinal_term(self) -> SolarTerm {
        match self {
            Self::Spring => SolarTerm::SPRING_EQUINOX,
            Self::Summer => SolarTerm::SUMMER_SOLSTICE,
            Self::Autumn => SolarTerm::AUTUMN_EQUINOX,
            Self::Winter => SolarTerm::WINTER_SOLSTICE,
        }
    }

    /// The first Gregorian month of this season in the northern
    /// meteorological reckoning: March, June, September, December.
    #[must_use]
    pub const fn meteorological_first_month(self) -> u8 {
        match self {
            Self::Spring => 3,
            Self::Summer => 6,
            Self::Autumn => 9,
            Self::Winter => 12,
        }
    }

    /// The name in English.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Spring => "spring",
            Self::Summer => "summer",
            Self::Autumn => "autumn",
            Self::Winter => "winter",
        }
    }

    /// The name in Japanese characters: 春, 夏, 秋, 冬.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        match self {
            Self::Spring => "春",
            Self::Summer => "夏",
            Self::Autumn => "秋",
            Self::Winter => "冬",
        }
    }

    /// The Japanese reading in Hepburn romaji.
    #[must_use]
    pub const fn romaji(self) -> &'static str {
        match self {
            Self::Spring => "haru",
            Self::Summer => "natsu",
            Self::Autumn => "aki",
            Self::Winter => "fuyu",
        }
    }
}

/// Which of the three season definitions to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeasonDefinition {
    /// Solstitial: each season runs from an equinox or solstice to the next.
    ///
    /// This is the one almanacs and school textbooks in Europe and North
    /// America use. Its seasons are unequal — northern spring is four days
    /// longer than northern autumn — because the Earth's orbit is an ellipse.
    Astronomical,
    /// Whole calendar months: spring is March to May, and so on.
    ///
    /// This is the one meteorological services use, because a climate record
    /// wants equal, whole-month buckets that do not drift against the
    /// calendar from year to year.
    Meteorological,
    /// The 立 terms: each season runs from 立春, 立夏, 立秋 or 立冬 to the
    /// next.
    ///
    /// The solstices and equinoxes are the *middles* of these seasons, which
    /// is why 夏至 is midsummer and not the first day of summer.
    EastAsian,
}

/// Which side of the equator the question is being asked from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Hemisphere {
    /// North of the equator.
    Northern,
    /// South of the equator: every season is the opposite one.
    Southern,
}

impl Hemisphere {
    /// The season a northern-hemisphere rule names, read from this
    /// hemisphere.
    #[must_use]
    const fn read(self, northern: Season) -> Season {
        match self {
            Self::Northern => northern,
            Self::Southern => northern.opposite(),
        }
    }
}

/// The first day of a season in a Gregorian year, under a given definition.
///
/// The year is the Gregorian year the season's *northern* opening falls in,
/// so northern winter of 2024 starts in December 2024 and runs into 2025.
/// Southern winter of 2024 starts in June 2024.
#[must_use]
pub fn season_start(
    year: i64,
    season: Season,
    definition: SeasonDefinition,
    hemisphere: Hemisphere,
    meridian: Meridian,
) -> Rd {
    // Internally everything is computed on the northern rule; the southern
    // hemisphere simply asks about the opposite season.
    let northern = hemisphere.read(season);
    match definition {
        SeasonDefinition::Astronomical => term_day(year, northern.cardinal_term(), meridian),
        SeasonDefinition::EastAsian => term_day(year, northern.east_asian_opening_term(), meridian),
        SeasonDefinition::Meteorological => {
            crate::gregorian::from_year_month_day(year, northern.meteorological_first_month(), 1)
        }
    }
}

/// The season a day falls in, under a given definition.
///
/// ```
/// use hc_seasons::{Meridian, seasons::{Hemisphere, Season, SeasonDefinition, season_of}};
///
/// // 10 May 2024 is spring meteorologically and astronomically, but summer
/// // in the East Asian reckoning: 立夏 fell on 5 May, and the almanac had
/// // already left spring behind.
/// let day = hc_calendar::Rd(739_016);
/// let north = Hemisphere::Northern;
/// let japan = Meridian::JAPAN;
/// assert_eq!(
///     season_of(day, SeasonDefinition::Meteorological, north, japan),
///     Season::Spring
/// );
/// assert_eq!(
///     season_of(day, SeasonDefinition::Astronomical, north, japan),
///     Season::Spring
/// );
/// assert_eq!(
///     season_of(day, SeasonDefinition::EastAsian, north, japan),
///     Season::Summer
/// );
/// ```
#[must_use]
pub fn season_of(
    day: Rd,
    definition: SeasonDefinition,
    hemisphere: Hemisphere,
    meridian: Meridian,
) -> Season {
    let northern = match definition {
        SeasonDefinition::Meteorological => match month_from_rd(day) {
            3..=5 => Season::Spring,
            6..=8 => Season::Summer,
            9..=11 => Season::Autumn,
            _ => Season::Winter,
        },
        SeasonDefinition::Astronomical | SeasonDefinition::EastAsian => {
            season_from_solar_term(day, definition, meridian)
        }
    };
    hemisphere.read(northern)
}

/// The northern season of a day, for the two definitions that are keyed to
/// solar longitude.
///
/// Both reduce to "which quadrant of the ecliptic is the Sun in", offset by
/// nothing for the astronomical rule and by 45° for the East Asian one. The
/// work goes through [`crate::solar_terms::term_on_day`] rather than the raw
/// longitude so that the answer agrees, day for day, with the term the same
/// meridian would print.
fn season_from_solar_term(day: Rd, definition: SeasonDefinition, meridian: Meridian) -> Season {
    let term = crate::solar_terms::term_on_day(day, meridian);
    let degrees = term.solar_longitude_degrees() as i64;
    let offset = match definition {
        // Astronomical: spring starts at 0°, so the quadrants are
        // 0–90, 90–180, 180–270, 270–360.
        SeasonDefinition::Astronomical => 0,
        // East Asian: spring starts at 315°, so the quadrants are shifted
        // back by 45°.
        _ => 45,
    };
    match ((degrees + offset) % 360) / 90 {
        0 => Season::Spring,
        1 => Season::Summer,
        2 => Season::Autumn,
        _ => Season::Winter,
    }
}

/// The interval a season occupies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeasonPeriod {
    /// Which season.
    pub season: Season,
    /// The first day.
    pub start: Rd,
    /// The last day, which is the day before the next season's start.
    pub end: Rd,
}

impl SeasonPeriod {
    /// How many days the season runs, endpoints included.
    #[must_use]
    pub const fn length_days(self) -> i64 {
        self.end.0 - self.start.0 + 1
    }

    /// Whether a day falls inside it.
    #[must_use]
    pub const fn contains(self, day: Rd) -> bool {
        day.0 >= self.start.0 && day.0 <= self.end.0
    }
}

/// The interval a season occupies in a Gregorian year.
///
/// Winter spans the new year, so its `end` is in the following Gregorian
/// year.
#[must_use]
pub fn season_period(
    year: i64,
    season: Season,
    definition: SeasonDefinition,
    hemisphere: Hemisphere,
    meridian: Meridian,
) -> SeasonPeriod {
    let start = season_start(year, season, definition, hemisphere, meridian);
    let next = season.next();
    // The following season opens later in the same Gregorian year unless
    // this one is the last of the year, in which case it opens in the next.
    let candidate = season_start(year, next, definition, hemisphere, meridian);
    let end = if candidate.0 > start.0 {
        Rd(candidate.0 - 1)
    } else {
        Rd(season_start(year + 1, next, definition, hemisphere, meridian).0 - 1)
    };
    SeasonPeriod { season, start, end }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::from_year_month_day;

    const JAPAN: Meridian = Meridian::JAPAN;
    const NORTH: Hemisphere = Hemisphere::Northern;
    const SOUTH: Hemisphere = Hemisphere::Southern;

    #[test]
    fn the_seasons_form_a_cycle_of_four() {
        for season in Season::ALL {
            assert_eq!(season.next().next().next().next(), season);
            assert_eq!(season.next().previous(), season);
            assert_eq!(season.opposite().opposite(), season);
            assert_ne!(season.opposite(), season);
            assert!(!season.english_name().is_empty());
            assert!(!season.japanese_name().is_empty());
            assert!(!season.romaji().is_empty());
        }
        assert_eq!(Season::Spring.opposite(), Season::Autumn);
        assert_eq!(Season::Summer.opposite(), Season::Winter);
    }

    /// The three definitions put the start of northern summer 2024 a month
    /// apart from one another, which is the whole reason the caller has to
    /// choose.
    #[test]
    fn the_three_definitions_start_summer_on_three_different_days() {
        let astronomical = season_start(
            2024,
            Season::Summer,
            SeasonDefinition::Astronomical,
            NORTH,
            JAPAN,
        );
        let meteorological = season_start(
            2024,
            Season::Summer,
            SeasonDefinition::Meteorological,
            NORTH,
            JAPAN,
        );
        let east_asian = season_start(
            2024,
            Season::Summer,
            SeasonDefinition::EastAsian,
            NORTH,
            JAPAN,
        );
        assert_eq!(astronomical, from_year_month_day(2024, 6, 21));
        assert_eq!(meteorological, from_year_month_day(2024, 6, 1));
        assert_eq!(east_asian, from_year_month_day(2024, 5, 5));
        assert!(east_asian < meteorological && meteorological < astronomical);
        // Six weeks separate the earliest and the latest answer.
        assert_eq!(astronomical.0 - east_asian.0, 47);
    }

    #[test]
    fn the_east_asian_seasons_are_centred_on_the_cardinal_points() {
        for year in 1990..2030 {
            for season in Season::ALL {
                let period = season_period(year, season, SeasonDefinition::EastAsian, NORTH, JAPAN);
                let cardinal = term_day(year, season.cardinal_term(), JAPAN);
                // The solstice or equinox sits within a few days of the
                // middle of the East Asian season, never at its edge.
                let from_start = cardinal.0 - period.start.0;
                assert!(
                    (40..=52).contains(&from_start),
                    "{} of {year}: cardinal point {from_start} days in",
                    season.english_name()
                );
            }
        }
    }

    #[test]
    fn the_astronomical_seasons_begin_at_the_cardinal_points() {
        for year in 1990..2030 {
            for season in Season::ALL {
                assert_eq!(
                    season_start(year, season, SeasonDefinition::Astronomical, NORTH, JAPAN),
                    term_day(year, season.cardinal_term(), JAPAN)
                );
            }
        }
    }

    #[test]
    fn the_meteorological_seasons_begin_on_the_first_of_a_month() {
        for year in 1900..2100 {
            for season in Season::ALL {
                let start =
                    season_start(year, season, SeasonDefinition::Meteorological, NORTH, JAPAN);
                assert_eq!(
                    crate::gregorian::year_month_day_from_rd(start).2,
                    1,
                    "{} of {year} did not start on the first",
                    season.english_name()
                );
                assert_eq!(
                    crate::gregorian::year_month_day_from_rd(start).1,
                    season.meteorological_first_month()
                );
            }
        }
    }

    #[test]
    fn every_day_of_a_year_has_a_season_under_every_definition() {
        let start = from_year_month_day(2024, 1, 1);
        for definition in [
            SeasonDefinition::Astronomical,
            SeasonDefinition::Meteorological,
            SeasonDefinition::EastAsian,
        ] {
            let mut counts = [0i64; 4];
            for offset in 0..366 {
                let day = Rd(start.0 + offset);
                let season = season_of(day, definition, NORTH, JAPAN);
                counts[Season::ALL.iter().position(|s| *s == season).unwrap()] += 1;
            }
            assert_eq!(counts.iter().sum::<i64>(), 366);
            for count in counts {
                assert!(
                    (85..=100).contains(&count),
                    "a season got {count} days under {definition:?}"
                );
            }
        }
    }

    #[test]
    fn the_southern_hemisphere_is_always_in_the_opposite_season() {
        let start = from_year_month_day(2024, 1, 1);
        for definition in [
            SeasonDefinition::Astronomical,
            SeasonDefinition::Meteorological,
            SeasonDefinition::EastAsian,
        ] {
            for offset in (0..366).step_by(7) {
                let day = Rd(start.0 + offset);
                assert_eq!(
                    season_of(day, definition, SOUTH, JAPAN),
                    season_of(day, definition, NORTH, JAPAN).opposite()
                );
            }
        }
    }

    #[test]
    fn a_season_contains_its_own_first_and_last_day_and_nothing_beyond() {
        for definition in [
            SeasonDefinition::Astronomical,
            SeasonDefinition::Meteorological,
            SeasonDefinition::EastAsian,
        ] {
            for season in Season::ALL {
                let period = season_period(2024, season, definition, NORTH, JAPAN);
                assert!(period.contains(period.start));
                assert!(period.contains(period.end));
                assert!(!period.contains(Rd(period.start.0 - 1)));
                assert!(!period.contains(Rd(period.end.0 + 1)));
                assert!(
                    (85..=100).contains(&period.length_days()),
                    "{} of 2024 ran {} days under {definition:?}",
                    season.english_name(),
                    period.length_days()
                );
                assert_eq!(
                    season_of(period.start, definition, NORTH, JAPAN),
                    season,
                    "{} of 2024 did not contain its own start under {definition:?}",
                    season.english_name()
                );
                assert_eq!(season_of(period.end, definition, NORTH, JAPAN), season);
            }
        }
    }

    #[test]
    fn the_four_seasons_of_a_year_tile_it_without_a_gap() {
        for definition in [
            SeasonDefinition::Astronomical,
            SeasonDefinition::Meteorological,
            SeasonDefinition::EastAsian,
        ] {
            for year in 2000..2030 {
                for season in Season::ALL {
                    let period = season_period(year, season, definition, NORTH, JAPAN);
                    // The day after a season ends is the first day of the
                    // next one: no gap, no overlap, whichever rule is used.
                    assert_eq!(
                        season_of(Rd(period.end.0 + 1), definition, NORTH, JAPAN),
                        season.next(),
                        "{} of {year} under {definition:?} did not hand over to {}",
                        season.english_name(),
                        season.next().english_name()
                    );
                    assert_eq!(
                        season_of(period.end, definition, NORTH, JAPAN),
                        season,
                        "{} of {year} under {definition:?} ended in the wrong season",
                        season.english_name()
                    );
                }
            }
        }
    }

    /// Northern summer is the longest astronomical season and northern winter
    /// the shortest, because the Earth is near aphelion in July and moving
    /// slowly, and near perihelion in January and moving fast. The
    /// meteorological definition flattens that out on purpose, which is
    /// exactly what a climate statistic wants and exactly what an almanac
    /// does not.
    #[test]
    fn the_astronomical_seasons_are_not_of_equal_length() {
        let lengths = Season::ALL.map(|season| {
            season_period(2024, season, SeasonDefinition::Astronomical, NORTH, JAPAN).length_days()
        });
        let [spring, summer, autumn, winter] = lengths;
        // Summer and spring differ by less than a day, so in whole days they
        // can tie; the strict ordering only shows up in the instants.
        assert!(
            summer >= spring,
            "summer {summer} was shorter than spring {spring}"
        );
        assert!(
            spring > autumn,
            "spring {spring} was not longer than autumn {autumn}"
        );
        assert!(
            autumn > winter,
            "autumn {autumn} was not longer than winter {winter}"
        );
        assert!(
            summer - winter >= 4,
            "the spread was only {} days",
            summer - winter
        );
        // In the instants the ordering is strict: summer is the longest.
        let instant_length = |season: Season| {
            let start = crate::solar_terms::term_moment(2024, season.cardinal_term());
            let next = season.next();
            let end = if next == Season::Spring {
                crate::solar_terms::term_moment(2025, next.cardinal_term())
            } else {
                crate::solar_terms::term_moment(2024, next.cardinal_term())
            };
            end.0 - start.0
        };
        assert!(instant_length(Season::Summer) > instant_length(Season::Spring));
        assert!(instant_length(Season::Winter) < instant_length(Season::Autumn));

        let meteorological = Season::ALL.map(|season| {
            season_period(2024, season, SeasonDefinition::Meteorological, NORTH, JAPAN)
                .length_days()
        });
        let spread = meteorological.iter().copied().max().unwrap_or(0)
            - meteorological.iter().copied().min().unwrap_or(0);
        assert!(
            spread <= 2,
            "the whole-month seasons varied by {spread} days"
        );
    }

    /// The Mid-Autumn Festival is in September because the autumn equinox is
    /// the *middle* of the East Asian autumn, not its start. The same day is
    /// early autumn astronomically and mid-autumn in the East Asian
    /// reckoning — a disagreement of a whole season for part of the year.
    #[test]
    fn the_two_astronomical_definitions_disagree_for_half_the_year() {
        let start = from_year_month_day(2024, 1, 1);
        let mut disagreements = 0;
        for offset in 0..366 {
            let day = Rd(start.0 + offset);
            if season_of(day, SeasonDefinition::Astronomical, NORTH, JAPAN)
                != season_of(day, SeasonDefinition::EastAsian, NORTH, JAPAN)
            {
                disagreements += 1;
            }
        }
        // 45° of the ecliptic is an eighth of the year, and the two rules are
        // offset by exactly that, so they disagree about half the time.
        assert!(
            (170..=200).contains(&disagreements),
            "{disagreements} days of disagreement"
        );
    }

    #[test]
    fn the_season_of_a_day_agrees_with_the_period_that_contains_it() {
        for definition in [
            SeasonDefinition::Astronomical,
            SeasonDefinition::Meteorological,
            SeasonDefinition::EastAsian,
        ] {
            let start = from_year_month_day(2024, 4, 1);
            for offset in 0..200 {
                let day = Rd(start.0 + offset);
                let season = season_of(day, definition, NORTH, JAPAN);
                let period = season_period(2024, season, definition, NORTH, JAPAN);
                assert!(
                    period.contains(day),
                    "{day} said {} under {definition:?} but is outside that period",
                    season.english_name()
                );
            }
        }
    }

    #[test]
    fn the_opening_terms_are_the_four_establishments() {
        assert_eq!(
            Season::Spring.east_asian_opening_term(),
            SolarTerm::BEGINNING_OF_SPRING
        );
        assert_eq!(
            Season::Summer.east_asian_opening_term(),
            SolarTerm::BEGINNING_OF_SUMMER
        );
        assert_eq!(
            Season::Autumn.east_asian_opening_term(),
            SolarTerm::BEGINNING_OF_AUTUMN
        );
        assert_eq!(
            Season::Winter.east_asian_opening_term(),
            SolarTerm::BEGINNING_OF_WINTER
        );
        for season in Season::ALL {
            let opening = season.east_asian_opening_term().solar_longitude_degrees();
            let cardinal = season.cardinal_term().solar_longitude_degrees();
            assert!(
                ((cardinal - opening + 360.0) % 360.0 - 45.0).abs() < 1e-9,
                "{} opens more than 45 degrees before its cardinal point",
                season.english_name()
            );
        }
    }
}
