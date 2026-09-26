//! Easter, both ways, and the feasts that hang off it.
//!
//! Easter is the one date in the Western calendar that is genuinely
//! *computed* rather than tabulated, and it is computed twice: the Gregorian
//! computus that the Roman Catholic and Protestant churches adopted with the
//! calendar reform of 1582, and the Julian computus that most Orthodox
//! churches still use. The two coincide in some years — 2025 and 2028, for
//! instance — and are five weeks apart in others.
//!
//! Once Easter is known, roughly twenty feasts are a fixed offset from it,
//! and an offset is data. Nothing in this module knows the name of a single
//! holiday.
//!
//! # Where the algorithms come from
//!
//! The Gregorian computus is the "anonymous Gregorian" or Butcher algorithm
//! as Meeus gives it in *Astronomical Algorithms*, 2nd edition, chapter 8,
//! "Date of Easter" (`meeus1998`), which is itself Butcher's 1876
//! arrangement of Gauss's rule. The Julian computus is Meeus's Julian
//! variant in the same chapter, due to Delambre. Both are exact
//! arithmetic, not astronomy: the ecclesiastical moon of the computus is a
//! table, and how far it sits from the real one differs between the two.
//!
//! - The Gregorian tables were built to stop "any progressive departure"
//!   from the Moon (Richards in the *Explanatory Supplement*, 3rd edition,
//!   §15.4.3, `richards2013`). Their paschal full moon is usually within
//!   a day of the astronomical one and never more than three days from it,
//!   as the Astronomical Society of South Australia's Easter Dating Method
//!   states (`assa-easter`).
//! - The Julian computus's nineteen-year cycle has no such correction. By
//!   the sixteenth century its new moons fell four days before the dates
//!   its tables assumed (Richards §15.4.1), and the error grows by about a
//!   day in three centuries (Wikipedia, "Computus", secondary); and its
//!   equinox, 21 March in the Julian calendar, falls thirteen days after
//!   the astronomical one, as the World Council of Churches' Aleppo
//!   statement of 1997 says (`wcc-aleppo-1997`).
//!
//! Easter is therefore *exactly* what these functions say, and only
//! approximately the Sunday after a full moon.

use hc_calendar::Rd;
use hc_calendars_solar::{gregorian, julian};

/// A paschal reckoning: a rule that gives Easter Sunday for a year.
///
/// Two ship, and they are the two that govern nearly every church. A third
/// — the astronomical Easter of the 1997 Aleppo proposal, say — is an entry
/// with its own function, not a variant this crate has to be taught
/// (ADR 0007). Two reckonings are equal when they have the same identifier.
#[derive(Debug, Clone, Copy)]
pub struct Computus {
    /// A short identifier: `gregorian`, `julian`.
    pub id: &'static str,
    /// The name in English.
    pub english_name: &'static str,
    /// The first Gregorian year the reckoning is defined for.
    pub first_year: i64,
    easter: fn(i64) -> Option<Rd>,
}

impl PartialEq for Computus {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Computus {}

impl core::hash::Hash for Computus {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Computus {
    /// A reckoning from its identifier, its English name, its first year
    /// and the function that computes its Easter.
    #[must_use]
    pub const fn new(
        id: &'static str,
        english_name: &'static str,
        first_year: i64,
        easter: fn(i64) -> Option<Rd>,
    ) -> Self {
        Self {
            id,
            english_name,
            first_year,
            easter,
        }
    }

    /// Easter Sunday of a Gregorian year under this reckoning, or `None`
    /// outside the years it is defined for.
    #[must_use]
    pub fn easter(self, year: i64) -> Option<Rd> {
        (self.easter)(year)
    }
}

hc_core::catalogue! {
    type: Computus,
    id: |computus| computus.id,
    tests: computus_tests,
    associated;

    /// Every reckoning this crate ships.
    pub const ALL;
    /// The reckoning with this identifier.
    pub fn by_id;

    entries: {
        /// The Gregorian computus: Western Christianity from 1583 onwards.
        pub const GREGORIAN = Self::new(
            "gregorian",
            "Gregorian computus",
            GREGORIAN_COMPUTUS_FIRST_YEAR,
            gregorian_easter,
        );
        /// The Julian computus: most Orthodox churches, and all of
        /// Christendom before the reform.
        pub const JULIAN = Self::new(
            "julian",
            "Julian computus",
            JULIAN_COMPUTUS_FIRST_YEAR,
            orthodox_easter,
        );
    }
}

/// The first Gregorian year the Gregorian computus is defined for.
///
/// The reform bull *Inter gravissimas* took effect in October 1582, so the
/// first Easter computed this way was that of 1583. Asking for an earlier
/// year gets `None` rather than a proleptic answer nobody ever observed.
pub const GREGORIAN_COMPUTUS_FIRST_YEAR: i64 = 1583;

/// The first Gregorian year the Julian computus is defined for.
///
/// 326 is the first year after the Council of Nicaea, the year the
/// Astronomical Society of South Australia's method starts from
/// (`assa-easter`). Nicaea laid down no algorithm: it endorsed the rule of
/// the equinox and published no computation, as Mosshammer argues
/// (`mosshammer2008`, through its review in the *Bryn Mawr Classical
/// Review*) and the Aleppo statement says (`wcc-aleppo-1997`). The
/// Alexandrian nineteen-year cycle this computus is was authoritative in
/// the East from the late fourth century, and was taken up in the West over
/// the sixth to eighth — Dionysius Exiguus's table of 525, Whitby in 664.
/// So the years from 326 to the late fourth century are the rule applied
/// backwards, and not the Easters every church kept.
pub const JULIAN_COMPUTUS_FIRST_YEAR: i64 = 326;

/// The last year either computus is computed for here.
///
/// Both algorithms are pure integer arithmetic and would keep answering
/// forever, and the Gregorian rules themselves have no last year. 4099 is
/// the end of the range the Astronomical Society of South Australia states
/// for its method, "all years 326 to 4099 A.D.", because "one additional
/// February 29 date will need to be removed in about 4140 A.D." — its
/// assumption about a calendar amendment no church has made
/// (`assa-easter`). Refusing past it is this library's choice, taken from
/// that range.
pub const COMPUTUS_LAST_YEAR: i64 = 4099;

/// Easter Sunday, as a fixed day, under the chosen computus.
///
/// The Julian computus returns the fixed day of a date that the Orthodox
/// churches state in the Julian calendar; because [`Rd`] is calendar-neutral,
/// the same value read through the Gregorian calendar is the civil date the
/// feast is kept on. Use [`orthodox_easter_julian_date`] when the
/// Julian-calendar month and day are what is wanted.
///
/// Returns `None` outside the years the computus is defined for.
#[must_use]
pub fn easter(computus: Computus, year: i64) -> Option<Rd> {
    computus.easter(year)
}

/// Western Easter Sunday, as a Gregorian month and day.
///
/// Returns `None` before 1583 or after [`COMPUTUS_LAST_YEAR`].
#[must_use]
pub fn gregorian_easter_date(year: i64) -> Option<(u8, u8)> {
    if !(GREGORIAN_COMPUTUS_FIRST_YEAR..=COMPUTUS_LAST_YEAR).contains(&year) {
        return None;
    }
    // Butcher's arrangement, as Meeus gives it. Every quantity is a named
    // step of the computus: `h` is the epact-derived age of the
    // ecclesiastical moon, `l` the days from the paschal full moon to the
    // following Sunday.
    let a = year.rem_euclid(19);
    let b = year.div_euclid(100);
    let c = year.rem_euclid(100);
    let d = b.div_euclid(4);
    let e = b.rem_euclid(4);
    let f = (b + 8).div_euclid(25);
    let g = (b - f + 1).div_euclid(3);
    let h = (19 * a + b - d - g + 15).rem_euclid(30);
    let i = c.div_euclid(4);
    let k = c.rem_euclid(4);
    let l = (32 + 2 * e + 2 * i - h - k).rem_euclid(7);
    let m = (a + 11 * h + 22 * l).div_euclid(451);
    let n = h + l - 7 * m + 114;
    let month = n.div_euclid(31);
    let day = n.rem_euclid(31) + 1;
    Some((u8::try_from(month).ok()?, u8::try_from(day).ok()?))
}

/// Western Easter Sunday as a fixed day.
///
/// Returns `None` before 1583 or after [`COMPUTUS_LAST_YEAR`].
#[must_use]
pub fn gregorian_easter(year: i64) -> Option<Rd> {
    let (month, day) = gregorian_easter_date(year)?;
    gregorian::to_fixed(year, month, day).ok()
}

/// Orthodox Easter Sunday, as a **Julian** calendar month and day.
///
/// This is the form the Orthodox churches state the feast in. In the
/// twentieth and twenty-first centuries the Julian calendar runs thirteen
/// days behind the Gregorian one, so 22 April Julian is 5 May Gregorian.
///
/// Returns `None` before 326 or after [`COMPUTUS_LAST_YEAR`].
#[must_use]
pub fn orthodox_easter_julian_date(year: i64) -> Option<(u8, u8)> {
    if !(JULIAN_COMPUTUS_FIRST_YEAR..=COMPUTUS_LAST_YEAR).contains(&year) {
        return None;
    }
    // Delambre's rule for the Julian computus, from Meeus chapter 8. It is
    // shorter than the Gregorian one because the Julian calendar has no
    // century exceptions to correct for.
    let a = year.rem_euclid(4);
    let b = year.rem_euclid(7);
    let c = year.rem_euclid(19);
    let d = (19 * c + 15).rem_euclid(30);
    let e = (2 * a + 4 * b - d + 34).rem_euclid(7);
    let n = d + e + 114;
    let month = n.div_euclid(31);
    let day = n.rem_euclid(31) + 1;
    Some((u8::try_from(month).ok()?, u8::try_from(day).ok()?))
}

/// Orthodox Easter Sunday as a fixed day.
///
/// Returns `None` before 326 or after [`COMPUTUS_LAST_YEAR`].
#[must_use]
pub fn orthodox_easter(year: i64) -> Option<Rd> {
    let (month, day) = orthodox_easter_julian_date(year)?;
    julian::to_fixed(year, month, day).ok()
}

/// Orthodox Easter Sunday as a **Gregorian** calendar year, month and day —
/// the date it appears on a civil calendar.
///
/// Returns `None` outside the supported years.
#[must_use]
pub fn orthodox_easter_gregorian_date(year: i64) -> Option<(i64, u8, u8)> {
    gregorian::from_fixed(orthodox_easter(year)?).ok()
}

/// The named feasts that are a fixed offset from Easter.
///
/// These are the offsets, not the holidays: a country table says which of
/// them it keeps and under which computus. Corpus Christi is the Thursday
/// after Trinity Sunday, which is the Thursday sixty days after Easter;
/// Whit Monday is the day after Pentecost.
pub mod offsets {
    /// Septuagesima Sunday.
    pub const SEPTUAGESIMA: i16 = -63;
    /// Shrove Monday, *Rosenmontag*.
    pub const SHROVE_MONDAY: i16 = -48;
    /// Shrove Tuesday, *Mardi Gras*, Carnival Tuesday.
    pub const SHROVE_TUESDAY: i16 = -47;
    /// Ash Wednesday, the first day of Lent in the Western reckoning.
    pub const ASH_WEDNESDAY: i16 = -46;
    /// Laetare Sunday, the fourth Sunday of Lent.
    pub const LAETARE_SUNDAY: i16 = -21;
    /// Palm Sunday.
    pub const PALM_SUNDAY: i16 = -7;
    /// Holy Monday.
    pub const HOLY_MONDAY: i16 = -6;
    /// Holy Tuesday.
    pub const HOLY_TUESDAY: i16 = -5;
    /// Spy Wednesday.
    pub const HOLY_WEDNESDAY: i16 = -4;
    /// Maundy Thursday.
    pub const MAUNDY_THURSDAY: i16 = -3;
    /// Good Friday.
    pub const GOOD_FRIDAY: i16 = -2;
    /// Holy Saturday.
    pub const HOLY_SATURDAY: i16 = -1;
    /// Easter Sunday itself.
    pub const EASTER_SUNDAY: i16 = 0;
    /// Easter Monday.
    pub const EASTER_MONDAY: i16 = 1;
    /// Easter Tuesday, still a holiday in a few places.
    pub const EASTER_TUESDAY: i16 = 2;
    /// Divine Mercy Sunday, the octave of Easter.
    pub const DIVINE_MERCY_SUNDAY: i16 = 7;
    /// The Ascension of the Lord, forty days after Easter counting Easter
    /// itself as the first.
    pub const ASCENSION: i16 = 39;
    /// Pentecost, Whit Sunday.
    pub const PENTECOST: i16 = 49;
    /// Whit Monday, *Pfingstmontag*, *lundi de Pentecôte*.
    pub const WHIT_MONDAY: i16 = 50;
    /// Trinity Sunday.
    pub const TRINITY_SUNDAY: i16 = 56;
    /// Corpus Christi, *Fronleichnam*.
    pub const CORPUS_CHRISTI: i16 = 60;
    /// The Sacred Heart of Jesus.
    pub const SACRED_HEART: i16 = 68;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    #[test]
    fn the_gregorian_computus_matches_published_easter_dates() {
        // 1818 and 1886, the earliest and latest possible dates, as Meeus
        // gives them in chapter 8 (`meeus1998`), and 5 April 2026, as the
        // Liturgy Office's summary of 2026 gives it (`roman_calendar`). The
        // other years are not from a printed calendar read for this test.
        let published = [
            (1818, 3, 22), // the earliest possible Easter, and Meeus's example
            (1886, 4, 25), // the latest possible Easter
            (1954, 4, 18),
            (1961, 4, 2),
            (1991, 3, 31),
            (2000, 4, 23),
            (2008, 3, 23),
            (2011, 4, 24),
            (2018, 4, 1),
            (2024, 3, 31),
            (2025, 4, 20),
            (2026, 4, 5),
            (2027, 3, 28),
            (2030, 4, 21),
            (2038, 4, 25),
        ];
        for (year, month, day) in published {
            assert_eq!(
                gregorian_easter(year),
                Some(greg(year, month, day)),
                "Western Easter {year}"
            );
        }
    }

    #[test]
    fn the_julian_computus_matches_published_orthodox_easter_dates() {
        // Stated in the Julian calendar, as the churches state them; not
        // from a printed paschalion read for this test.
        let published = [
            (2008, 4, 14),
            (2010, 3, 22),
            (2016, 4, 18),
            (2021, 4, 19),
            (2024, 4, 22),
            (2025, 4, 7),
        ];
        for (year, month, day) in published {
            assert_eq!(
                orthodox_easter_julian_date(year),
                Some((month, day)),
                "Orthodox Easter {year}, Julian reckoning"
            );
        }
    }

    #[test]
    fn orthodox_easter_lands_on_the_right_civil_date() {
        // The civil dates of the same Sundays; not from a printed church
        // calendar read for this test.
        let civil = [
            (2021, 5, 2),
            (2022, 4, 24),
            (2023, 4, 16),
            (2024, 5, 5),
            (2025, 4, 20),
            (2026, 4, 12),
        ];
        for (year, month, day) in civil {
            assert_eq!(
                orthodox_easter_gregorian_date(year),
                Some((year, month, day)),
                "Orthodox Easter {year}, civil date"
            );
        }
    }

    #[test]
    fn the_two_computations_coincide_in_2025_and_differ_in_2024() {
        assert_eq!(
            easter(Computus::GREGORIAN, 2025),
            easter(Computus::JULIAN, 2025)
        );
        assert_ne!(
            easter(Computus::GREGORIAN, 2024),
            easter(Computus::JULIAN, 2024)
        );
    }

    #[test]
    fn easter_is_always_a_sunday() {
        use hc_calendar::Weekday;
        for year in 1583..=2500 {
            let western = gregorian_easter(year).expect("in range");
            assert_eq!(Weekday::from_rd(western), Weekday::Sunday, "Western {year}");
            let orthodox = orthodox_easter(year).expect("in range");
            assert_eq!(
                Weekday::from_rd(orthodox),
                Weekday::Sunday,
                "Orthodox {year}"
            );
        }
    }

    #[test]
    fn western_easter_stays_within_its_legal_window() {
        // By construction Easter falls between 22 March and 25 April.
        for year in 1583..=2500 {
            let (month, day) = gregorian_easter_date(year).expect("in range");
            let ordinal = if month == 3 { day } else { day + 31 };
            assert!((22..=56).contains(&ordinal), "{year}: {month}/{day}");
        }
    }

    #[test]
    fn the_computus_refuses_years_it_was_never_defined_for() {
        assert_eq!(gregorian_easter(1582), None);
        assert_eq!(gregorian_easter(4100), None);
        assert_eq!(orthodox_easter(325), None);
        assert_eq!(orthodox_easter(4100), None);
    }

    #[test]
    fn the_keyed_feasts_are_offsets_and_nothing_more() {
        // Good Friday 2024 was 29 March, Ascension 9 May, Pentecost 19 May,
        // Corpus Christi 30 May.
        let easter_2024 = gregorian_easter(2024).expect("in range");
        assert_eq!(
            Rd(easter_2024.0 + i64::from(offsets::GOOD_FRIDAY)),
            greg(2024, 3, 29)
        );
        assert_eq!(
            Rd(easter_2024.0 + i64::from(offsets::ASCENSION)),
            greg(2024, 5, 9)
        );
        assert_eq!(
            Rd(easter_2024.0 + i64::from(offsets::PENTECOST)),
            greg(2024, 5, 19)
        );
        assert_eq!(
            Rd(easter_2024.0 + i64::from(offsets::CORPUS_CHRISTI)),
            greg(2024, 5, 30)
        );
    }
}
