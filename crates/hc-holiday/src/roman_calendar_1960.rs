//! The General Roman Calendar of 1960: the calendar of the 1962 Missal,
//! each day with its class.
//!
//! John XXIII's motu proprio *Rubricarum instructum* of 25 July 1960
//! approved a new Code of Rubrics, and with it a calendar, which the 1962
//! edition of the Roman Missal prints and which the Missal's present use
//! keeps. `docs/systems/roman-calendar-1960.md` in the repository describes
//! it, works an example and states the sources; this page states the code's
//! own facts.
//!
//! The rubrics rank every liturgical day in one of four classes (General
//! Rubrics nos. 10–36, `rubrics-1960`), and the simple feasts of the older
//! calendar stayed *commemorations*, made in the office of another day
//! (`wikipedia-grc-1960`):
//!
//! | Class | Count | Examples |
//! | --- | --- | --- |
//! | I class | 53 | Christmas, Easter, the Sundays of Advent and Lent, St Joseph, All Souls' Day |
//! | II class | 95 | the Holy Family, the Purification, the apostles, the vigils of the Ascension and of the Assumption, the Sundays after Epiphany, Easter and Pentecost, the ferias of 17 to 23 December, the Ember Days of Advent, Lent and September |
//! | III class | 181 | St Hilary, St Agnes, the Holy Name of Mary |
//! | Commemoration | 106 | St Telesphorus, St George, Our Lady of Mount Carmel |
//!
//! A fourth class holds the ferias of the year that are none of these; they
//! are not listed, and nor are the ferias of Advent to 16 December and of
//! Lent, of the III class, which are every weekday of their seasons:
//! [`SEASON`] holds them for [`ordo`]. The table carries the calendar as the English
//! translation of the rubrics prints it, month by month, with the titles in
//! its abbreviations ("Bp." bishop, "Cf." confessor, "Doct." doctor, "M."
//! martyr, "V." virgin), a commemoration on the same day as a feast as an
//! entry of its own; the movable feasts the calendar and the Table of
//! Liturgical Days name; and the Sundays, ferias, vigils and days within an
//! octave of the Proper of Time that the Table ranks in the first class;
//! and of the second class, the Vigil of the Ascension, the ferias of
//! Advent from 17 to 23 December (no. 24a) and every other Sunday, "All
//! other Sundays not mentioned above": within the octave of Christmas,
//! after Epiphany, of Septuagesima, after Easter, after the Ascension and
//! after Pentecost, with the Sundays after Epiphany that Septuagesima
//! impedes resumed after the Twenty-third after Pentecost as no. 18 orders.
//!
//! # The ordo
//!
//! [`CELEBRATIONS`] and the rule set list every day the calendar gives a
//! date, before precedence. [`ordo`] and [`office_on`] apply it: which
//! liturgical day is kept by the Table of Liturgical Days arranged
//! according to order of precedence (no. 91, [`precedence`]), with its
//! exceptions (nos. 15, 16, 30); the transfer of an impeded feast of the I
//! class to the nearest following day not of the I or II class, the
//! Annunciation's after Easter to the Monday after Low Sunday (nos. 95–99);
//! the vigils omitted on a Sunday or a feast of the I class, or when their
//! feast is not kept (no. 33); and the commemorations the office allows,
//! privileged or ordinary, in their order (nos. 106–114). The Table of
//! Occurrence itself (p. 115), a grid, is not legible in the text read; its
//! outcomes are those the numbered rubrics state. The rules for a feast's
//! own date are the calendar's: St Matthias on 25 February and St Gabriel
//! of the Sorrowing Virgin on 28 February in a leap year, and All Souls' Day
//! on 3 November when 2 November is a Sunday (no. 96b).
//!
//! The Ember Days of Advent, Lent and September are ferias of the II class,
//! place 18 of the Table with the greater ferias of Advent: the Wednesday,
//! Friday and Saturday after the Third Sunday of Advent, the First Sunday
//! of Lent and the third Sunday of September within the month (Wikipedia,
//! "General Roman Calendar of 1960" and "Ember days", retrieved
//! 2026-09-29; divinumofficium.com's "Rubrics 1960" kalendar for February,
//! May, September and December 2027). An Ember Day of Advent from 17 to
//! 23 December is that day's feria under the Ember title. Those of
//! Pentecost are days within its octave, of the I class, and keep the
//! octave's titles here.
//!
//! The calendar is carried from [`FIRST_YEAR`], 1961, the year the Code of
//! Rubrics came into force; before it the table has no day and [`ordo`]
//! and [`office_on`] answer `None`.
//!
//! # What this is not
//!
//! The concurrence
//! of vespers (nos. 103–105); the commemorations excluded by the identity
//! of a saint or a mystery (no. 112a, d) beyond the feasts of the Lord; and
//! the particular calendars of nations, dioceses and orders, whose places
//! in the table (nos. 12, 13, 19, 20, 23) are empty here. The Greater and
//! Lesser Litanies are the `rogation-roman-1960` table's.
//!
//! Sources: the Code of Rubrics and its calendar in the English
//! translation *The New Rubrics of the Roman Breviary and Missal* (1960),
//! nos. 6–114 and pp. 98–116 (`rubrics-1960`), read in a scanned copy's
//! text layer, retrieved 2026-09-27 and re-read 2026-09-29; checked day by
//! day against Wikipedia, "General Roman Calendar of 1960"
//! (`wikipedia-grc-1960`), retrieved 2026-09-27, whose class and number of
//! commemorations agree on every day; the ordo checked against propria.org's
//! "Catholic Ordo" for 2019 to 2026 (`propria-ordo`), retrieved 2026-09-29.
//! The Latin text in *Acta Apostolicae Sedis* 52 (1960) was not read.

use alloc::vec::Vec;

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;

use crate::computus::{
    self,
    offsets::{
        ASCENSION, ASH_WEDNESDAY, CORPUS_CHRISTI, EASTER_SUNDAY, PALM_SUNDAY, PENTECOST,
        SACRED_HEART, SEPTUAGESIMA, TRINITY_SUNDAY,
    },
};
use crate::rule::{Days, HolidayRule, Kind, NO_WEEKEND, Rule, RuleSet, SourceDate, Subdivisions};

/// The first year the rubrics of 1960 were in force: the Code of Rubrics
/// promulgated by *Rubricarum instructum* was "effective January 1, 1961"
/// (Wikipedia, "Rubricarum instructum", retrieved 2026-09-29). Before it
/// the calendar and its ordo are absent.
pub const FIRST_YEAR: i64 = 1961;

/// The instrument that established the calendar, cited by its every rule.
const RUBRICARUM_INSTRUCTUM: &str = "John XXIII, motu proprio Rubricarum instructum, 25 July 1960, \
     the Code of Rubrics effective 1 January 1961 (Wikipedia, \"Rubricarum instructum\", \
     retrieved 2026-09-29)";

/// The class of a liturgical day under the rubrics of 1960 (nos. 10–35).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Class {
    /// The first class: the principal feasts, the Sundays of Advent, Lent
    /// and Passiontide, Ash Wednesday and Holy Week.
    First,
    /// The second class.
    Second,
    /// The third class.
    Third,
    /// The fourth class: the ferias of the year that are none of the
    /// others, and the Saturday Office of our Lady (nos. 26, 78). No entry
    /// of [`CELEBRATIONS`] is of it; the days of [`SEASON`] are.
    Fourth,
    /// A commemoration: a saint remembered in the office of the day, not a
    /// day of its own rank.
    Commemoration,
}

impl Class {
    /// The class's English name, as the translation prints it.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::First => "I class",
            Self::Second => "II class",
            Self::Third => "III class",
            Self::Fourth => "IV class",
            Self::Commemoration => "Commemoration",
        }
    }
}

/// A day of the calendar of 1960.
#[derive(Debug, Clone, Copy)]
pub struct Celebration {
    /// Its title, as the translation prints it.
    pub title: &'static str,
    /// Its class.
    pub class: Class,
    /// The day it falls on.
    pub rule: Rule,
}

/// St Matthias: 24 February, or 25 February in a leap year.
fn matthias(year: i64) -> Days {
    leap_year_february(year, 24)
}

/// St Gabriel of the Sorrowing Virgin: 27 February, or 28 February in a
/// leap year.
fn gabriel_of_the_sorrowing_virgin(year: i64) -> Days {
    leap_year_february(year, 27)
}

/// A day of February that moves one day later in a leap year, as the
/// calendar's note on February says.
fn leap_year_february(year: i64, day: u8) -> Days {
    let day = if gregorian::is_leap_year(year) {
        day + 1
    } else {
        day
    };
    gregorian::to_fixed(year, 2, day).map_or_else(|_| Days::new(), Days::one)
}

/// The Holy Name of Jesus: the Sunday between the octave-day of Christmas
/// and the Epiphany, or 2 January when there is none.
fn holy_name(year: i64) -> Days {
    for day in 2..=5 {
        if let Ok(rd) = gregorian::to_fixed(year, 1, day)
            && Weekday::from_rd(rd) == Weekday::Sunday
        {
            return Days::one(rd);
        }
    }
    gregorian::to_fixed(year, 1, 2).map_or_else(|_| Days::new(), Days::one)
}

/// All Souls' Day: 2 November, or the Monday after when that is a Sunday
/// (no. 96).
fn all_souls(year: i64) -> Days {
    let Ok(day) = gregorian::to_fixed(year, 11, 2) else {
        return Days::new();
    };
    if Weekday::from_rd(day) == Weekday::Sunday {
        Days::one(day + 1)
    } else {
        Days::one(day)
    }
}

/// The first Sunday after the Epiphany: the Holy Family.
const HOLY_FAMILY: Rule = Rule::WeekdayOnOrAfter {
    month: 1,
    day: 7,
    weekday: Weekday::Sunday,
};

/// The last Sunday of October: Christ the King.
const CHRIST_THE_KING: Rule = Rule::LastWeekday {
    month: 10,
    weekday: Weekday::Sunday,
};

/// The `n`-th Sunday of Advent, the first being the Sunday between
/// 27 November and 3 December.
const fn advent(n: u8) -> Rule {
    Rule::WeekdayOnOrAfter {
        month: if n == 1 { 11 } else { 12 },
        day: match n {
            1 => 27,
            2 => 4,
            3 => 11,
            _ => 18,
        },
        weekday: Weekday::Sunday,
    }
}

/// The Sunday within the octave of Christmas, between 26 and 31 December;
/// none when Christmas is a Sunday, whose next Sunday is the octave-day.
fn sunday_within_the_octave_of_christmas(year: i64) -> Days {
    let Ok(stephen) = gregorian::to_fixed(year, 12, 26) else {
        return Days::new();
    };
    let sunday = Weekday::Sunday.on_or_after(stephen);
    if sunday.0 - stephen.0 <= 5 {
        Days::one(sunday)
    } else {
        Days::new()
    }
}

/// The Holy Family, the first Sunday after the Epiphany, from which the
/// other Sundays after the Epiphany are counted.
fn first_sunday_after_epiphany(year: i64) -> Option<Rd> {
    Some(Weekday::Sunday.after(gregorian::to_fixed(year, 1, 6).ok()?))
}

/// The `n`-th Sunday after the Epiphany, 2 to 6, when it comes before
/// Septuagesima; the ones it would meet are resumed after Pentecost
/// (no. 18).
fn sunday_after_epiphany(year: i64, n: i64) -> Option<Days> {
    let first = first_sunday_after_epiphany(year)?;
    let easter = computus::gregorian_easter(year)?;
    let sunday = Rd(first.0 + 7 * (n - 1));
    Some(if sunday.0 < easter.0 + i64::from(SEPTUAGESIMA) {
        Days::one(sunday)
    } else {
        Days::new()
    })
}

/// How many Sundays there are after Pentecost, Trinity Sunday the first:
/// 23 to 28, as no. 18 counts them.
fn sundays_after_pentecost(year: i64) -> Option<(Rd, i64)> {
    let pentecost = Rd(computus::gregorian_easter(year)?.0 + i64::from(PENTECOST));
    let advent = Weekday::Sunday.on_or_after(gregorian::to_fixed(year, 11, 27).ok()?);
    Some((pentecost, (advent.0 - pentecost.0) / 7 - 1))
}

/// The `k`-th Sunday after Pentecost, 2 to 23, in its own place; with 23
/// Sundays the Twenty-third is not kept, since "The Sunday which is set
/// down as XXIV after Pentecost is always put in the last place, omitting,
/// if need be, any others for which there happens to be no place" (no. 18).
fn sunday_after_pentecost(year: i64, k: i64) -> Option<Days> {
    let (pentecost, count) = sundays_after_pentecost(year)?;
    Some(if k < count {
        Days::one(Rd(pentecost.0 + 7 * k))
    } else {
        Days::new()
    })
}

/// The last Sunday after Pentecost, which is always the Twenty-fourth's
/// (no. 18): the Sunday before Advent.
fn last_sunday_after_pentecost(year: i64) -> Option<Days> {
    sundays_after_pentecost(year).map(|(pentecost, count)| Days::one(Rd(pentecost.0 + 7 * count)))
}

/// The `m`-th Sunday after the Epiphany, 3 to 6, resumed after the
/// Twenty-third Sunday after Pentecost (no. 18): with 25 Sundays after
/// Pentecost the Sixth is the twenty-fourth, with 26 the Fifth and Sixth,
/// with 27 the Fourth to Sixth, with 28 the Third to Sixth, and the last
/// is always the Twenty-fourth after Pentecost.
fn resumed_sunday_after_epiphany(year: i64, m: i64) -> Option<Days> {
    let (pentecost, count) = sundays_after_pentecost(year)?;
    let first_resumed = 31 - count;
    if count < 25 || m < first_resumed {
        return Some(Days::new());
    }
    let place = 24 + (m - first_resumed);
    Some(Days::one(Rd(pentecost.0 + 7 * place)))
}

/// A feria of Advent of the II class, `day` December, 17 to 23, when it is
/// not a Sunday (no. 24) nor one of the Ember Days of Advent, which are
/// ferias of the same class under their own title.
fn greater_feria_of_advent(year: i64, day: u8) -> Option<Days> {
    match gregorian::to_fixed(year, 12, day) {
        Ok(rd)
            if Weekday::from_rd(rd) != Weekday::Sunday
                && !EMBER_DAYS_OF_ADVENT
                    .iter()
                    .any(|ember| ember.days_in_year(year).as_slice().contains(&rd)) =>
        {
            Some(Days::one(rd))
        }
        _ => Some(Days::new()),
    }
}

/// The Third Sunday of Advent, the Sunday the Ember Days of Advent follow.
static THIRD_SUNDAY_OF_ADVENT: Rule = advent(3);
/// The Third Sunday of September "actually within the calendar month",
/// the Sunday John XXIII's rubrics make the September Ember Days follow
/// (Wikipedia, "Ember days", retrieved 2026-09-29).
static THIRD_SUNDAY_OF_SEPTEMBER: Rule = Rule::nth(9, 3, Weekday::Sunday);

/// The Wednesday, Friday and Saturday after `sunday`.
macro_rules! ember_week {
    ($sunday:expr) => {
        [
            Rule::Offset {
                base: &$sunday,
                days: 3,
            },
            Rule::Offset {
                base: &$sunday,
                days: 5,
            },
            Rule::Offset {
                base: &$sunday,
                days: 6,
            },
        ]
    };
}

/// The Ember Days of Advent: the Wednesday, Friday and Saturday after the
/// Third Sunday of Advent.
static EMBER_DAYS_OF_ADVENT: [Rule; 3] = ember_week!(THIRD_SUNDAY_OF_ADVENT);
/// The Ember Days of September: the Wednesday, Friday and Saturday after
/// the third Sunday of September.
static EMBER_DAYS_OF_SEPTEMBER: [Rule; 3] = ember_week!(THIRD_SUNDAY_OF_SEPTEMBER);

/// The rules of the Sundays and ferias the Proper of Time adds, each a
/// `fn(i64) -> Option<Days>` for [`Rule::Unsettled`]: none where there is
/// no Easter to count from.
macro_rules! computed_days {
    ($($name:ident = $function:ident($($argument:expr),*);)*) => {
        $(fn $name(year: i64) -> Option<Days> { $function(year, $($argument),*) })*
    };
}

computed_days! {
    second_sunday_after_epiphany = sunday_after_epiphany(2);
    third_sunday_after_epiphany = sunday_after_epiphany(3);
    fourth_sunday_after_epiphany = sunday_after_epiphany(4);
    fifth_sunday_after_epiphany = sunday_after_epiphany(5);
    sixth_sunday_after_epiphany = sunday_after_epiphany(6);
    third_resumed = resumed_sunday_after_epiphany(3);
    fourth_resumed = resumed_sunday_after_epiphany(4);
    fifth_resumed = resumed_sunday_after_epiphany(5);
    sixth_resumed = resumed_sunday_after_epiphany(6);
    feria_17 = greater_feria_of_advent(17);
    feria_18 = greater_feria_of_advent(18);
    feria_19 = greater_feria_of_advent(19);
    feria_20 = greater_feria_of_advent(20);
    feria_21 = greater_feria_of_advent(21);
    feria_22 = greater_feria_of_advent(22);
    feria_23 = greater_feria_of_advent(23);
    pentecost_2 = sunday_after_pentecost(2);
    pentecost_3 = sunday_after_pentecost(3);
    pentecost_4 = sunday_after_pentecost(4);
    pentecost_5 = sunday_after_pentecost(5);
    pentecost_6 = sunday_after_pentecost(6);
    pentecost_7 = sunday_after_pentecost(7);
    pentecost_8 = sunday_after_pentecost(8);
    pentecost_9 = sunday_after_pentecost(9);
    pentecost_10 = sunday_after_pentecost(10);
    pentecost_11 = sunday_after_pentecost(11);
    pentecost_12 = sunday_after_pentecost(12);
    pentecost_13 = sunday_after_pentecost(13);
    pentecost_14 = sunday_after_pentecost(14);
    pentecost_15 = sunday_after_pentecost(15);
    pentecost_16 = sunday_after_pentecost(16);
    pentecost_17 = sunday_after_pentecost(17);
    pentecost_18 = sunday_after_pentecost(18);
    pentecost_19 = sunday_after_pentecost(19);
    pentecost_20 = sunday_after_pentecost(20);
    pentecost_21 = sunday_after_pentecost(21);
    pentecost_22 = sunday_after_pentecost(22);
    pentecost_23 = sunday_after_pentecost(23);
}

const MATTHIAS: Rule = Rule::Computed(matthias);
const GABRIEL_OF_THE_SORROWING_VIRGIN: Rule = Rule::Computed(gabriel_of_the_sorrowing_virgin);

/// The calendar, written once and read twice: as [`CELEBRATIONS`], with
/// the classes, and as the rule set the engine evaluates.
macro_rules! calendar_1960 {
    ($($title:literal, $class:ident, $rule:expr);* $(;)?) => {
        /// Every day of the calendar of 1960 this table carries: the
        /// calendar's months in order, with the movable feasts where the
        /// calendar prints them, then the Proper of Time.
        pub static CELEBRATIONS: &[Celebration] = &[$(
            Celebration { title: $title, class: Class::$class, rule: $rule }
        ),*];

        static RULES: &[HolidayRule] = &[$(
            HolidayRule::observance($title, "", $rule)
                .of_kind(Kind::Religious)
                .years(Some(FIRST_YEAR as i32), None)
                .cited(RUBRICARUM_INSTRUCTUM)
        ),*];
    };
}

calendar_1960! {
    // January
    "Octave-day of Christmas", First, Rule::gregorian(1, 1);
    "S. Telesphorus, Pope, M.", Commemoration, Rule::gregorian(1, 5);
    "The Epiphany of our Lord", First, Rule::gregorian(1, 6);
    "S. Hyginus, Pope, M.", Commemoration, Rule::gregorian(1, 11);
    "Commemoration of the Baptism of our Lord", Second, Rule::gregorian(1, 13);
    "S. Hilary, Bp., Cf., Doct.", Third, Rule::gregorian(1, 14);
    "S. Felix, priest, M.", Commemoration, Rule::gregorian(1, 14);
    "S. Paul the First Hermit, Cf.", Third, Rule::gregorian(1, 15);
    "S. Maurus, Abb.", Commemoration, Rule::gregorian(1, 15);
    "S. Marcellus I, Pope, M.", Third, Rule::gregorian(1, 16);
    "S. Antony, Abb.", Third, Rule::gregorian(1, 17);
    "S. Prisca, V.M.", Commemoration, Rule::gregorian(1, 18);
    "SS. Marius, Martha, Audifax and Abachum, MM.", Commemoration, Rule::gregorian(1, 19);
    "S. Canute, king, M.", Commemoration, Rule::gregorian(1, 19);
    "SS. Fabian, Pope, and Sebastian, MM.", Third, Rule::gregorian(1, 20);
    "S. Agnes, V.M.", Third, Rule::gregorian(1, 21);
    "SS. Vincent and Anastasius, MM.", Third, Rule::gregorian(1, 22);
    "S. Raymond of Penafort, Cf.", Third, Rule::gregorian(1, 23);
    "S. Emerentiana, V.M.", Commemoration, Rule::gregorian(1, 23);
    "S. Timothy, Bp., M.", Third, Rule::gregorian(1, 24);
    "The Conversion of S. Paul, Ap.", Third, Rule::gregorian(1, 25);
    "S. Peter, Ap.", Commemoration, Rule::gregorian(1, 25);
    "S. Polycarp, Bp., M.", Third, Rule::gregorian(1, 26);
    "S. John Chrysostom, Bp., Cf., Doct.", Third, Rule::gregorian(1, 27);
    "S. Peter Nolasco, Cf.", Third, Rule::gregorian(1, 28);
    "S. Agnes, V.M., secundo", Commemoration, Rule::gregorian(1, 28);
    "S. Francis de Sales, Bp., Cf., Doct.", Third, Rule::gregorian(1, 29);
    "S. Martina, V.M.", Third, Rule::gregorian(1, 30);
    "S. John Bosco, Cf.", Third, Rule::gregorian(1, 31);
    "The Holy Name of Jesus", Second, Rule::Computed(holy_name);
    "The Holy Family of Jesus, Mary and Joseph", Second, HOLY_FAMILY;
    // February
    "S. Ignatius, Bp., M.", Third, Rule::gregorian(2, 1);
    "The Purification of our Lady", Second, Rule::gregorian(2, 2);
    "S. Blaise, Bp., M.", Commemoration, Rule::gregorian(2, 3);
    "S. Andrew Corsini, Bp., Cf.", Third, Rule::gregorian(2, 4);
    "S. Agatha, V.M.", Third, Rule::gregorian(2, 5);
    "S. Titus, Bp., Cf.", Third, Rule::gregorian(2, 6);
    "S. Dorothy, V.M.", Commemoration, Rule::gregorian(2, 6);
    "S. Romuald, Abb.", Third, Rule::gregorian(2, 7);
    "S. John of Matha, Cf.", Third, Rule::gregorian(2, 8);
    "S. Cyril, Bishop of Alexandria, Cf., Doct.", Third, Rule::gregorian(2, 9);
    "S. Apollonia, V.M.", Commemoration, Rule::gregorian(2, 9);
    "S. Scholastica, V.", Third, Rule::gregorian(2, 10);
    "The Apparition of the Immaculate Virgin Mary", Third, Rule::gregorian(2, 11);
    "Seven Holy Founders of the Servite Order, Cff.", Third, Rule::gregorian(2, 12);
    "S. Valentine, M.", Commemoration, Rule::gregorian(2, 14);
    "SS. Faustinus and Jovita, MM.", Commemoration, Rule::gregorian(2, 15);
    "S. Simeon, Bp., M.", Commemoration, Rule::gregorian(2, 18);
    "S. Peter's Chair", Second, Rule::gregorian(2, 22);
    "S. Paul, Ap.", Commemoration, Rule::gregorian(2, 22);
    "S. Peter Damian, Bp., Cf., Doct.", Third, Rule::gregorian(2, 23);
    "S. Matthias, Ap.", Second, MATTHIAS;
    "S. Gabriel of the Sorrowing Virgin, Cf.", Third, GABRIEL_OF_THE_SORROWING_VIRGIN;
    // March
    "S. Casimir, Cf.", Third, Rule::gregorian(3, 4);
    "S. Lucius I, Pope, M.", Commemoration, Rule::gregorian(3, 4);
    "SS. Perpetua and Felicitas, MM.", Third, Rule::gregorian(3, 6);
    "S. Thomas Aquinas, Cf., Doct.", Third, Rule::gregorian(3, 7);
    "S. John of God, Cf.", Third, Rule::gregorian(3, 8);
    "S. Frances of Rome, Widow", Third, Rule::gregorian(3, 9);
    "The Forty Martyrs", Third, Rule::gregorian(3, 10);
    "S. Gregory I, Pope, Cf., Doct.", Third, Rule::gregorian(3, 12);
    "S. Patrick, Bp., Cf.", Third, Rule::gregorian(3, 17);
    "S. Cyril, Bishop of Jerusalem, Cf., Doct.", Third, Rule::gregorian(3, 18);
    "S. Joseph, Husband of our Lady, Cf., Patron of Universal Church", First, Rule::gregorian(3, 19);
    "S. Benedict, Abb.", Third, Rule::gregorian(3, 21);
    "S. Gabriel, Archangel", Third, Rule::gregorian(3, 24);
    "The Annunciation of the Blessed Virgin Mary", First, Rule::gregorian(3, 25);
    "S. John Damascene, Cf., Doct.", Third, Rule::gregorian(3, 27);
    "S. John Capistran, Cf.", Third, Rule::gregorian(3, 28);
    "The Seven Sorrows of our Lady", Commemoration, Rule::easter(-9);
    // April
    "S. Francis of Paola, Cf.", Third, Rule::gregorian(4, 2);
    "S. Isidore, Bp., Cf., Doct.", Third, Rule::gregorian(4, 4);
    "S. Vincent Ferrer, Cf.", Third, Rule::gregorian(4, 5);
    "S. Leo I, Pope, Cf., Doct.", Third, Rule::gregorian(4, 11);
    "S. Hermenegild, M.", Third, Rule::gregorian(4, 13);
    "S. Justin, M.", Third, Rule::gregorian(4, 14);
    "SS. Tiburtius, Valerian and Maximus, MM.", Commemoration, Rule::gregorian(4, 14);
    "S. Anicetus I, Pope, M.", Commemoration, Rule::gregorian(4, 17);
    "S. Anselm, Bp., Cf., Doct.", Third, Rule::gregorian(4, 21);
    "SS. Soter and Caius, Popes, MM.", Third, Rule::gregorian(4, 22);
    "S. George, M.", Commemoration, Rule::gregorian(4, 23);
    "S. Fidelis of Sigmaringen, M.", Third, Rule::gregorian(4, 24);
    "S. Mark, Evangelist", Second, Rule::gregorian(4, 25);
    "SS. Cletus and Marcellinus, Popes, MM.", Third, Rule::gregorian(4, 26);
    "S. Peter Canisius, Cf., Doct.", Third, Rule::gregorian(4, 27);
    "S. Paul of the Cross, Cf.", Third, Rule::gregorian(4, 28);
    "S. Peter, M.", Third, Rule::gregorian(4, 29);
    "S. Catherine of Siena, V.", Third, Rule::gregorian(4, 30);
    // May
    "S. Joseph the Workman, Husband of our Lady, Cf.", First, Rule::gregorian(5, 1);
    "S. Athanasius, Bp., Cf., Doct.", Third, Rule::gregorian(5, 2);
    "SS. Alexander, Eventius and Theodulus, MM., and Juvenal, Bp., Cf.", Commemoration, Rule::gregorian(5, 3);
    "S. Monica, Widow", Third, Rule::gregorian(5, 4);
    "S. Pius V, Pope, Cf.", Third, Rule::gregorian(5, 5);
    "S. Stanislas, Bp., M.", Third, Rule::gregorian(5, 7);
    "S. Gregory Nazianzen, Bp., Cf., Doct.", Third, Rule::gregorian(5, 9);
    "S. Antonine, Bp., Cf.", Third, Rule::gregorian(5, 10);
    "SS. Gordian and Epimachus, MM.", Commemoration, Rule::gregorian(5, 10);
    "SS. Philip and James, App.", Second, Rule::gregorian(5, 11);
    "SS. Nereus, Achilleus, Domitilla, V., and Pancras, MM.", Third, Rule::gregorian(5, 12);
    "S. Robert Bellarmine, Bp., Cf., Doct.", Third, Rule::gregorian(5, 13);
    "S. Boniface, M.", Commemoration, Rule::gregorian(5, 14);
    "S. John Baptist de la Salle, Cf.", Third, Rule::gregorian(5, 15);
    "S. Ubald, Bp., Cf.", Third, Rule::gregorian(5, 16);
    "S. Pascal Baylon, Cf.", Third, Rule::gregorian(5, 17);
    "S. Venantius, M.", Third, Rule::gregorian(5, 18);
    "S. Peter Celestine, Pope, Cf.", Third, Rule::gregorian(5, 19);
    "S. Pudentiana, V.", Commemoration, Rule::gregorian(5, 19);
    "S. Bernardine of Siena, Cf.", Third, Rule::gregorian(5, 20);
    "S. Gregory VII, Pope, Cf.", Third, Rule::gregorian(5, 25);
    "S. Urban I, Pope, M.", Commemoration, Rule::gregorian(5, 25);
    "S. Philip Neri, Cf.", Third, Rule::gregorian(5, 26);
    "S. Eleutherius, Pope, M.", Commemoration, Rule::gregorian(5, 26);
    "S. Bede the Venerable, Cf., Doct.", Third, Rule::gregorian(5, 27);
    "S. John I, Pope, M.", Commemoration, Rule::gregorian(5, 27);
    "S. Augustine, Bp., Cf.", Third, Rule::gregorian(5, 28);
    "S. Mary Magdalen dei Pazzi, V.", Third, Rule::gregorian(5, 29);
    "S. Felix I, Pope, M.", Commemoration, Rule::gregorian(5, 30);
    "The Queenship of our Lady", Second, Rule::gregorian(5, 31);
    "S. Petronilla, V.", Commemoration, Rule::gregorian(5, 31);
    // June
    "S. Angela dei Merici, V.", Third, Rule::gregorian(6, 1);
    "SS. Marcellinus, Peter and Erasmus, Bp., MM.", Commemoration, Rule::gregorian(6, 2);
    "S. Francis Caracciolo, Cf.", Third, Rule::gregorian(6, 4);
    "S. Boniface, Bp., M.", Third, Rule::gregorian(6, 5);
    "S. Norbert, Bp., Cf.", Third, Rule::gregorian(6, 6);
    "SS. Primus and Felician, MM.", Commemoration, Rule::gregorian(6, 9);
    "S. Margaret, Queen, Widow", Third, Rule::gregorian(6, 10);
    "S. Barnabas, Ap.", Third, Rule::gregorian(6, 11);
    "S. John of Sahagun, Cf.", Third, Rule::gregorian(6, 12);
    "SS. Basilides, Cyrinus, Nabor and Nazarius, MM.", Commemoration, Rule::gregorian(6, 12);
    "S. Antony of Padua, Cf., Doct.", Third, Rule::gregorian(6, 13);
    "S. Basil the Great, Bp., Cf., Doct.", Third, Rule::gregorian(6, 14);
    "SS. Vitus, Modestus and Crescentia, MM.", Commemoration, Rule::gregorian(6, 15);
    "S. Gregory Barbarigo, Bp., Cf.", Third, Rule::gregorian(6, 17);
    "S. Ephraem the Syrian, deacon, Cf., Doct.", Third, Rule::gregorian(6, 18);
    "SS. Mark and Marcellian, MM.", Commemoration, Rule::gregorian(6, 18);
    "S. Juliana Falconieri, V.", Third, Rule::gregorian(6, 19);
    "SS. Gervase and Protase, MM.", Commemoration, Rule::gregorian(6, 19);
    "S. Silverius, Pope, M.", Commemoration, Rule::gregorian(6, 20);
    "S. Aloysius Gonzaga, Cf.", Third, Rule::gregorian(6, 21);
    "S. Paulinus, Bp., Cf.", Third, Rule::gregorian(6, 22);
    "Vigil of the Birthday of S. John the Baptist", Second, Rule::gregorian(6, 23);
    "The Birthday of S. John the Baptist", First, Rule::gregorian(6, 24);
    "S. William, Abb.", Third, Rule::gregorian(6, 25);
    "SS. John and Paul, MM.", Third, Rule::gregorian(6, 26);
    "Vigil of SS. Peter and Paul, Apostles", Second, Rule::gregorian(6, 28);
    "SS. Peter and Paul, App.", First, Rule::gregorian(6, 29);
    "Commemoration of S. Paul, Ap.", Third, Rule::gregorian(6, 30);
    "S. Peter, Ap.", Commemoration, Rule::gregorian(6, 30);
    // July
    "The Precious Blood of our Lord", First, Rule::gregorian(7, 1);
    "The Visitation of our Lady", Second, Rule::gregorian(7, 2);
    "SS. Processus and Martinian, MM.", Commemoration, Rule::gregorian(7, 2);
    "S. Irenaeus, Bp., M.", Third, Rule::gregorian(7, 3);
    "S. Antony Mary Zaccaria, Cf.", Third, Rule::gregorian(7, 5);
    "SS. Cyril and Methodius, Bpp., Cff.", Third, Rule::gregorian(7, 7);
    "S. Elizabeth, Queen, Widow", Third, Rule::gregorian(7, 8);
    "The Seven Holy Brothers, MM., and SS. Rufina and Secunda, VV., MM.", Third, Rule::gregorian(7, 10);
    "S. Pius I, Pope, M.", Commemoration, Rule::gregorian(7, 11);
    "S. John Gualbert, Abb.", Third, Rule::gregorian(7, 12);
    "SS. Nabor and Felix, MM.", Commemoration, Rule::gregorian(7, 12);
    "S. Bonaventure, Bp., Cf., Doct.", Third, Rule::gregorian(7, 14);
    "S. Henry, Emperor, Cf.", Third, Rule::gregorian(7, 15);
    "Our Lady of Mount Carmel", Commemoration, Rule::gregorian(7, 16);
    "S. Alexis, Cf.", Commemoration, Rule::gregorian(7, 17);
    "S. Camillus de Lellis, Cf.", Third, Rule::gregorian(7, 18);
    "SS. Symphorosa and her Seven Sons, MM.", Commemoration, Rule::gregorian(7, 18);
    "S. Vincent de Paul, Cf.", Third, Rule::gregorian(7, 19);
    "S. Jerome Emiliani, Cf.", Third, Rule::gregorian(7, 20);
    "S. Margaret, V.M.", Commemoration, Rule::gregorian(7, 20);
    "S. Laurence of Brindisi, Cf., Doct.", Third, Rule::gregorian(7, 21);
    "S. Praxede, V.", Commemoration, Rule::gregorian(7, 21);
    "S. Mary Magdalen, Penitent", Third, Rule::gregorian(7, 22);
    "S. Apollinaris, Bp., M.", Third, Rule::gregorian(7, 23);
    "S. Liborius, Bp., Cf.", Commemoration, Rule::gregorian(7, 23);
    "S. Christina, V.M.", Commemoration, Rule::gregorian(7, 24);
    "S. James, Ap.", Second, Rule::gregorian(7, 25);
    "S. Christopher, M.", Commemoration, Rule::gregorian(7, 25);
    "S. Anne, Mother of our Lady", Second, Rule::gregorian(7, 26);
    "S. Pantaleon, M.", Commemoration, Rule::gregorian(7, 27);
    "SS. Nazarius and Celsus, MM., Victor I, Pope, M., and Innocent I, Pope, Cf.", Third, Rule::gregorian(7, 28);
    "S. Martha, V.", Third, Rule::gregorian(7, 29);
    "SS. Felix, Simplicius, Faustinus and Beatrice, MM.", Commemoration, Rule::gregorian(7, 29);
    "SS. Abdon and Sennen, MM.", Commemoration, Rule::gregorian(7, 30);
    "S. Ignatius, Cf.", Third, Rule::gregorian(7, 31);
    // August
    "The Holy Machabees, MM.", Commemoration, Rule::gregorian(8, 1);
    "S. Alphonsus Mary de' Liguori, Bp., Cf., Doct.", Third, Rule::gregorian(8, 2);
    "S. Stephen I, Pope, M.", Commemoration, Rule::gregorian(8, 2);
    "S. Dominic, Cf.", Third, Rule::gregorian(8, 4);
    "Dedication of the Basilica of our Lady of the Snows", Third, Rule::gregorian(8, 5);
    "The Transfiguration of our Lord", Second, Rule::gregorian(8, 6);
    "SS. Sixtus II, Pope, Felicissimus and Agapitus, MM.", Commemoration, Rule::gregorian(8, 6);
    "S. Cajetan, Cf.", Third, Rule::gregorian(8, 7);
    "S. Donatus, Bp., M.", Commemoration, Rule::gregorian(8, 7);
    "S. John Mary Vianney, Cf.", Third, Rule::gregorian(8, 8);
    "SS. Cyriack, Largus and Smaragdus, MM.", Commemoration, Rule::gregorian(8, 8);
    "Vigil of S. Laurence, M.", Third, Rule::gregorian(8, 9);
    "S. Romanus, M.", Commemoration, Rule::gregorian(8, 9);
    "S. Laurence, M.", Second, Rule::gregorian(8, 10);
    "SS. Tiburtius and Susanna, V., MM.", Commemoration, Rule::gregorian(8, 11);
    "S. Clare, V.", Third, Rule::gregorian(8, 12);
    "SS. Hippolytus and Cassian, MM.", Commemoration, Rule::gregorian(8, 13);
    "Vigil of the Assumption of our Lady", Second, Rule::gregorian(8, 14);
    "S. Eusebius, Cf.", Commemoration, Rule::gregorian(8, 14);
    "The Assumption of our Lady", First, Rule::gregorian(8, 15);
    "S. Joachim, Father of our Lady, Cf.", Second, Rule::gregorian(8, 16);
    "S. Hyacinth, Cf.", Third, Rule::gregorian(8, 17);
    "S. Agapitus, M.", Commemoration, Rule::gregorian(8, 18);
    "S. John Eudes, Cf.", Third, Rule::gregorian(8, 19);
    "S. Bernard, Abb., Cf., Doct.", Third, Rule::gregorian(8, 20);
    "S. Jane Frances Fremiot de Chantal, Widow", Third, Rule::gregorian(8, 21);
    "The Immaculate Heart of our Lady", Second, Rule::gregorian(8, 22);
    "SS. Timothy and his Companions, MM.", Commemoration, Rule::gregorian(8, 22);
    "S. Philip Benizi, Cf.", Third, Rule::gregorian(8, 23);
    "S. Bartholomew, Ap.", Second, Rule::gregorian(8, 24);
    "S. Louis, king, Cf.", Third, Rule::gregorian(8, 25);
    "S. Zephyrinus, Pope, M.", Commemoration, Rule::gregorian(8, 26);
    "S. Joseph Calasanza, Cf.", Third, Rule::gregorian(8, 27);
    "S. Augustine, Bp., Cf., Doct.", Third, Rule::gregorian(8, 28);
    "S. Hermes, M.", Commemoration, Rule::gregorian(8, 28);
    "The Beheading of S. John the Baptist", Third, Rule::gregorian(8, 29);
    "S. Sabina, M.", Commemoration, Rule::gregorian(8, 29);
    "S. Rose of Lima, V.", Third, Rule::gregorian(8, 30);
    "SS. Felix and Adauctus, MM.", Commemoration, Rule::gregorian(8, 30);
    "S. Raymond Nonnatus, Cf.", Third, Rule::gregorian(8, 31);
    // September
    "S. Giles, Abb.", Commemoration, Rule::gregorian(9, 1);
    "The Twelve Brothers, MM.", Commemoration, Rule::gregorian(9, 1);
    "S. Stephen, king, Cf.", Third, Rule::gregorian(9, 2);
    "S. Pius X, Pope, Cf.", Third, Rule::gregorian(9, 3);
    "S. Laurence Giustiniani, Bp., Cf.", Third, Rule::gregorian(9, 5);
    "The Birthday of our Lady", Second, Rule::gregorian(9, 8);
    "S. Adrian, M.", Commemoration, Rule::gregorian(9, 8);
    "S. Gorgonius, M.", Commemoration, Rule::gregorian(9, 9);
    "S. Nicholas of Tolentino, Cf.", Third, Rule::gregorian(9, 10);
    "SS. Protus and Hyacinth, MM.", Commemoration, Rule::gregorian(9, 11);
    "The Holy Name of Mary", Third, Rule::gregorian(9, 12);
    "The Exaltation of the Holy Cross", Second, Rule::gregorian(9, 14);
    "The Seven Sorrows of our Lady", Second, Rule::gregorian(9, 15);
    "S. Nicomedes, M.", Commemoration, Rule::gregorian(9, 15);
    "SS. Cornelius, Pope, and Cyprian, Bp., MM.", Third, Rule::gregorian(9, 16);
    "SS. Euphemia, V., Lucy and Geminianus, MM.", Commemoration, Rule::gregorian(9, 16);
    "The Stigmata of S. Francis, Cf.", Commemoration, Rule::gregorian(9, 17);
    "S. Joseph of Cupertino, Cf.", Third, Rule::gregorian(9, 18);
    "SS. Januarius, Bp., and his Companions, MM.", Third, Rule::gregorian(9, 19);
    "SS. Eustace and his Companions, MM.", Commemoration, Rule::gregorian(9, 20);
    "S. Matthew, Ap. and Evang.", Second, Rule::gregorian(9, 21);
    "S. Thomas of Villanova, Bp., Cf.", Third, Rule::gregorian(9, 22);
    "SS. Maurice and his Companions, MM.", Commemoration, Rule::gregorian(9, 22);
    "S. Linus, Pope, M.", Third, Rule::gregorian(9, 23);
    "S. Thecla, V.M.", Commemoration, Rule::gregorian(9, 23);
    "Our Lady of Ransom", Commemoration, Rule::gregorian(9, 24);
    "SS. Cyprian and Justina, V., MM.", Commemoration, Rule::gregorian(9, 26);
    "SS. Cosmas and Damian, MM.", Third, Rule::gregorian(9, 27);
    "S. Wenceslaus, duke, M.", Third, Rule::gregorian(9, 28);
    "Dedication of S. Michael, Archangel", First, Rule::gregorian(9, 29);
    "S. Jerome, priest, Cf., Doct.", Third, Rule::gregorian(9, 30);
    // October
    "S. Remigius, Bp., Cf.", Commemoration, Rule::gregorian(10, 1);
    "The Guardian Angels", Third, Rule::gregorian(10, 2);
    "S. Teresa of the Child Jesus, V.", Third, Rule::gregorian(10, 3);
    "S. Francis, Cf.", Third, Rule::gregorian(10, 4);
    "SS. Placid and his Companions, MM.", Commemoration, Rule::gregorian(10, 5);
    "S. Bruno, Cf.", Third, Rule::gregorian(10, 6);
    "Our Lady of the Rosary", Second, Rule::gregorian(10, 7);
    "S. Mark I, Pope, Cf.", Commemoration, Rule::gregorian(10, 7);
    "S. Bridget, Widow", Third, Rule::gregorian(10, 8);
    "SS. Sergius, Bacchus, Marcellus and Apuleius, MM.", Commemoration, Rule::gregorian(10, 8);
    "S. John Leonardi, Cf.", Third, Rule::gregorian(10, 9);
    "SS. Denis, Bp., Rusticus and Eleutherius, MM.", Commemoration, Rule::gregorian(10, 9);
    "S. Francis Borgia, Cf.", Third, Rule::gregorian(10, 10);
    "The Motherhood of our Lady", Second, Rule::gregorian(10, 11);
    "S. Edward, king, Cf.", Third, Rule::gregorian(10, 13);
    "S. Callistus I, Pope, M.", Third, Rule::gregorian(10, 14);
    "S. Teresa, V.", Third, Rule::gregorian(10, 15);
    "S. Hedwig, Widow", Third, Rule::gregorian(10, 16);
    "S. Margaret Mary Alacoque, V.", Third, Rule::gregorian(10, 17);
    "S. Luke, Evang.", Second, Rule::gregorian(10, 18);
    "S. Peter of Alcantara, Cf.", Third, Rule::gregorian(10, 19);
    "S. John of Kanti, Cf.", Third, Rule::gregorian(10, 20);
    "S. Hilarion, Abb.", Commemoration, Rule::gregorian(10, 21);
    "S. Ursula and her Companions, VV., MM.", Commemoration, Rule::gregorian(10, 21);
    "S. Antony Mary Claret, Bp., Cf.", Third, Rule::gregorian(10, 23);
    "S. Raphael, Archangel", Third, Rule::gregorian(10, 24);
    "SS. Chrysanthus and Daria, MM.", Commemoration, Rule::gregorian(10, 25);
    "S. Evaristus, Pope, M.", Commemoration, Rule::gregorian(10, 26);
    "SS. Simon and Jude, App.", Second, Rule::gregorian(10, 28);
    "Christ, the King", First, CHRIST_THE_KING;
    // November
    "All Saints", First, Rule::gregorian(11, 1);
    "All Souls' Day", First, Rule::Computed(all_souls);
    "S. Charles, Bp., Cf.", Third, Rule::gregorian(11, 4);
    "SS. Vitalis and Agricola, MM.", Commemoration, Rule::gregorian(11, 4);
    "The Four Crowned Martyrs", Commemoration, Rule::gregorian(11, 8);
    "Dedication of the Archbasilica of our Saviour", Second, Rule::gregorian(11, 9);
    "S. Theodore, M.", Commemoration, Rule::gregorian(11, 9);
    "S. Andrew Avellino, Cf.", Third, Rule::gregorian(11, 10);
    "SS. Trypho, Respicius and Nympha, V., MM.", Commemoration, Rule::gregorian(11, 10);
    "S. Martin, Bp., Cf.", Third, Rule::gregorian(11, 11);
    "S. Mennas, M.", Commemoration, Rule::gregorian(11, 11);
    "S. Martin I, Pope, M.", Third, Rule::gregorian(11, 12);
    "S. Didacus, Cf.", Third, Rule::gregorian(11, 13);
    "S. Josaphat, Bp., M.", Third, Rule::gregorian(11, 14);
    "S. Albert the Great, Bp., Cf., Doct.", Third, Rule::gregorian(11, 15);
    "S. Gertrude, V.", Third, Rule::gregorian(11, 16);
    "S. Gregory the Wonder-worker, Bp., Cf.", Third, Rule::gregorian(11, 17);
    "Dedication of the Basilicas of SS. Peter and Paul, App.", Third, Rule::gregorian(11, 18);
    "S. Elizabeth, Widow", Third, Rule::gregorian(11, 19);
    "S. Pontianus, Pope, M.", Commemoration, Rule::gregorian(11, 19);
    "S. Felix of Valois, Cf.", Third, Rule::gregorian(11, 20);
    "The Presentation of our Lady", Third, Rule::gregorian(11, 21);
    "S. Cecilia, V.M.", Third, Rule::gregorian(11, 22);
    "S. Clement I, Pope, M.", Third, Rule::gregorian(11, 23);
    "S. Felicity, M.", Commemoration, Rule::gregorian(11, 23);
    "S. John of the Cross, Cf., Doct.", Third, Rule::gregorian(11, 24);
    "S. Chrysogonus, M.", Commemoration, Rule::gregorian(11, 24);
    "S. Catherine, V.M.", Third, Rule::gregorian(11, 25);
    "S. Silvester, Abb.", Third, Rule::gregorian(11, 26);
    "S. Peter of Alexandria, Bp., M.", Commemoration, Rule::gregorian(11, 26);
    "S. Saturninus, M.", Commemoration, Rule::gregorian(11, 29);
    "S. Andrew, Ap.", Second, Rule::gregorian(11, 30);
    // December
    "S. Bibiana, V.M.", Third, Rule::gregorian(12, 2);
    "S. Francis Xavier, Cf.", Third, Rule::gregorian(12, 3);
    "S. Peter Chrysologus, Bp., Cf., Doct.", Third, Rule::gregorian(12, 4);
    "S. Barbara, V.M.", Commemoration, Rule::gregorian(12, 4);
    "S. Sabbas, Abb.", Commemoration, Rule::gregorian(12, 5);
    "S. Nicholas, Bp., Cf.", Third, Rule::gregorian(12, 6);
    "S. Ambrose, Bp., Cf., Doct.", Third, Rule::gregorian(12, 7);
    "The Immaculate Conception of our Lady", First, Rule::gregorian(12, 8);
    "S. Melchiades, Pope, M.", Commemoration, Rule::gregorian(12, 10);
    "S. Damasus I, Pope, Cf.", Third, Rule::gregorian(12, 11);
    "S. Lucy, V.M.", Third, Rule::gregorian(12, 13);
    "S. Eusebius, Bp., M.", Third, Rule::gregorian(12, 16);
    "S. Thomas, Ap.", Second, Rule::gregorian(12, 21);
    "Vigil of Christmas", First, Rule::gregorian(12, 24);
    "Christmas Day", First, Rule::gregorian(12, 25);
    "S. Anastasia, M.", Commemoration, Rule::gregorian(12, 25);
    "S. Stephen, Protomartyr", Second, Rule::gregorian(12, 26);
    "S. John, Ap. and Evang.", Second, Rule::gregorian(12, 27);
    "The Holy Innocents, MM.", Second, Rule::gregorian(12, 28);
    "V day within the octave of Christmas", Second, Rule::gregorian(12, 29);
    "S. Thomas, Bp., M.", Commemoration, Rule::gregorian(12, 29);
    "VI day within the octave of Christmas", Second, Rule::gregorian(12, 30);
    "VII day within the octave of Christmas", Second, Rule::gregorian(12, 31);
    "S. Silvester I, Pope, Cf.", Commemoration, Rule::gregorian(12, 31);
    // The Proper of Time, as the Table of Liturgical Days ranks it.
    "First Sunday of Advent", First, advent(1);
    "Second Sunday of Advent", First, advent(2);
    "Third Sunday of Advent", First, advent(3);
    "Fourth Sunday of Advent", First, advent(4);
    "Feria of Advent, 17 December", Second, Rule::Unsettled(feria_17);
    "Feria of Advent, 18 December", Second, Rule::Unsettled(feria_18);
    "Feria of Advent, 19 December", Second, Rule::Unsettled(feria_19);
    "Feria of Advent, 20 December", Second, Rule::Unsettled(feria_20);
    "Feria of Advent, 21 December", Second, Rule::Unsettled(feria_21);
    "Feria of Advent, 22 December", Second, Rule::Unsettled(feria_22);
    "Feria of Advent, 23 December", Second, Rule::Unsettled(feria_23);
    // The Ember Days of Advent, Lent and September, ferias of the II class;
    // those of Pentecost are days within its octave, of the I class.
    "Ember Wednesday of Advent", Second, EMBER_DAYS_OF_ADVENT[0];
    "Ember Friday of Advent", Second, EMBER_DAYS_OF_ADVENT[1];
    "Ember Saturday of Advent", Second, EMBER_DAYS_OF_ADVENT[2];
    "Sunday within the octave of Christmas", Second, Rule::Computed(sunday_within_the_octave_of_christmas);
    "Second Sunday after Epiphany", Second, Rule::Unsettled(second_sunday_after_epiphany);
    "Third Sunday after Epiphany", Second, Rule::Unsettled(third_sunday_after_epiphany);
    "Fourth Sunday after Epiphany", Second, Rule::Unsettled(fourth_sunday_after_epiphany);
    "Fifth Sunday after Epiphany", Second, Rule::Unsettled(fifth_sunday_after_epiphany);
    "Sixth Sunday after Epiphany", Second, Rule::Unsettled(sixth_sunday_after_epiphany);
    "Septuagesima Sunday", Second, Rule::easter(SEPTUAGESIMA);
    "Sexagesima Sunday", Second, Rule::easter(-56);
    "Quinquagesima Sunday", Second, Rule::easter(-49);
    "Ash Wednesday", First, Rule::easter(ASH_WEDNESDAY);
    "First Sunday of Lent", First, Rule::easter(-42);
    "Ember Wednesday of Lent", Second, Rule::easter(-39);
    "Ember Friday of Lent", Second, Rule::easter(-37);
    "Ember Saturday of Lent", Second, Rule::easter(-36);
    "Second Sunday of Lent", First, Rule::easter(-35);
    "Third Sunday of Lent", First, Rule::easter(-28);
    "Fourth Sunday of Lent", First, Rule::easter(-21);
    "First Sunday of Passiontide", First, Rule::easter(-14);
    "Second Sunday of Passiontide or Palm Sunday", First, Rule::easter(PALM_SUNDAY);
    "Monday of Holy Week", First, Rule::easter(-6);
    "Tuesday of Holy Week", First, Rule::easter(-5);
    "Wednesday of Holy Week", First, Rule::easter(-4);
    "Thursday of Holy Week", First, Rule::easter(-3);
    "Friday of Holy Week", First, Rule::easter(-2);
    "Saturday of Holy Week", First, Rule::easter(-1);
    "Easter Sunday", First, Rule::easter(EASTER_SUNDAY);
    "Monday within the octave of Easter", First, Rule::easter(1);
    "Tuesday within the octave of Easter", First, Rule::easter(2);
    "Wednesday within the octave of Easter", First, Rule::easter(3);
    "Thursday within the octave of Easter", First, Rule::easter(4);
    "Friday within the octave of Easter", First, Rule::easter(5);
    "Saturday within the octave of Easter", First, Rule::easter(6);
    "Low Sunday", First, Rule::easter(7);
    "Second Sunday after Easter", Second, Rule::easter(14);
    "Third Sunday after Easter", Second, Rule::easter(21);
    "Fourth Sunday after Easter", Second, Rule::easter(28);
    "Fifth Sunday after Easter", Second, Rule::easter(35);
    "Vigil of the Ascension of our Lord", Second, Rule::easter(ASCENSION - 1);
    "Ascension of our Lord", First, Rule::easter(ASCENSION);
    "Sunday after the Ascension", Second, Rule::easter(ASCENSION + 3);
    "Vigil of Pentecost", First, Rule::easter(PENTECOST - 1);
    "Pentecost or Whit Sunday", First, Rule::easter(PENTECOST);
    "Monday within the octave of Pentecost", First, Rule::easter(PENTECOST + 1);
    "Tuesday within the octave of Pentecost", First, Rule::easter(PENTECOST + 2);
    "Wednesday within the octave of Pentecost", First, Rule::easter(PENTECOST + 3);
    "Thursday within the octave of Pentecost", First, Rule::easter(PENTECOST + 4);
    "Friday within the octave of Pentecost", First, Rule::easter(PENTECOST + 5);
    "Saturday within the octave of Pentecost", First, Rule::easter(PENTECOST + 6);
    "Feast of Blessed Trinity", First, Rule::easter(TRINITY_SUNDAY);
    "Feast of Corpus Christi", First, Rule::easter(CORPUS_CHRISTI);
    "Feast of the Sacred Heart", First, Rule::easter(SACRED_HEART);
    "Ember Wednesday of September", Second, EMBER_DAYS_OF_SEPTEMBER[0];
    "Ember Friday of September", Second, EMBER_DAYS_OF_SEPTEMBER[1];
    "Ember Saturday of September", Second, EMBER_DAYS_OF_SEPTEMBER[2];
    "Second Sunday after Pentecost", Second, Rule::Unsettled(pentecost_2);
    "Third Sunday after Pentecost", Second, Rule::Unsettled(pentecost_3);
    "Fourth Sunday after Pentecost", Second, Rule::Unsettled(pentecost_4);
    "Fifth Sunday after Pentecost", Second, Rule::Unsettled(pentecost_5);
    "Sixth Sunday after Pentecost", Second, Rule::Unsettled(pentecost_6);
    "Seventh Sunday after Pentecost", Second, Rule::Unsettled(pentecost_7);
    "Eighth Sunday after Pentecost", Second, Rule::Unsettled(pentecost_8);
    "Ninth Sunday after Pentecost", Second, Rule::Unsettled(pentecost_9);
    "Tenth Sunday after Pentecost", Second, Rule::Unsettled(pentecost_10);
    "Eleventh Sunday after Pentecost", Second, Rule::Unsettled(pentecost_11);
    "Twelfth Sunday after Pentecost", Second, Rule::Unsettled(pentecost_12);
    "Thirteenth Sunday after Pentecost", Second, Rule::Unsettled(pentecost_13);
    "Fourteenth Sunday after Pentecost", Second, Rule::Unsettled(pentecost_14);
    "Fifteenth Sunday after Pentecost", Second, Rule::Unsettled(pentecost_15);
    "Sixteenth Sunday after Pentecost", Second, Rule::Unsettled(pentecost_16);
    "Seventeenth Sunday after Pentecost", Second, Rule::Unsettled(pentecost_17);
    "Eighteenth Sunday after Pentecost", Second, Rule::Unsettled(pentecost_18);
    "Nineteenth Sunday after Pentecost", Second, Rule::Unsettled(pentecost_19);
    "Twentieth Sunday after Pentecost", Second, Rule::Unsettled(pentecost_20);
    "Twenty-first Sunday after Pentecost", Second, Rule::Unsettled(pentecost_21);
    "Twenty-second Sunday after Pentecost", Second, Rule::Unsettled(pentecost_22);
    "Twenty-third Sunday after Pentecost", Second, Rule::Unsettled(pentecost_23);
    "Third Sunday after Epiphany, resumed after Pentecost", Second, Rule::Unsettled(third_resumed);
    "Fourth Sunday after Epiphany, resumed after Pentecost", Second, Rule::Unsettled(fourth_resumed);
    "Fifth Sunday after Epiphany, resumed after Pentecost", Second, Rule::Unsettled(fifth_resumed);
    "Sixth Sunday after Epiphany, resumed after Pentecost", Second, Rule::Unsettled(sixth_resumed);
    "Twenty-fourth and last Sunday after Pentecost", Second, Rule::Unsettled(last_sunday_after_pentecost);
}

/// The calendar of 1960 as a rule set: every day a [`Kind::Religious`]
/// observance, with no precedence applied; [`ordo`] applies it.
pub static GENERAL_ROMAN_CALENDAR_1960: RuleSet = RuleSet {
    code: "roman-general-1960",
    english_name: "General Roman Calendar of 1960",
    rules: RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Code of Rubrics approved by John XXIII's motu proprio Rubricarum instructum of \
              25 July 1960, General Rubrics nos. 10-36 (the Sundays after Epiphany resumed \
              after Pentecost, no. 18; the ferias of 17-23 December, no. 24) and 96, the \
              Calendar of the Roman Breviary and Missal and the Table of Liturgical Days, in the \
              English translation The New Rubrics of the Roman Breviary and Missal (1960), \
              pp. 98-114, read in the copy at cdn.restorethe54.com/media/pdf/the-new-rubrics-of-\
              the-roman-missal-and-breviary-1960.pdf (rubrics-1960), retrieved 2026-09-27 and \
              re-read in the same text layer 2026-09-29; checked against \
              Wikipedia, \"General Roman Calendar of 1960\" (wikipedia-grc-1960), retrieved \
              2026-09-27. The Latin in Acta Apostolicae Sedis 52 (1960) was not read",
    subdivisions: Subdivisions::Undivided,
};

/// The days the calendar lists on a day, in its order, before precedence.
#[must_use]
pub fn celebrations_on(day: Rd) -> Vec<&'static Celebration> {
    let Ok((year, _, _)) = gregorian::from_fixed(day) else {
        return Vec::new();
    };
    CELEBRATIONS
        .iter()
        .filter(|celebration| {
            celebration
                .rule
                .days_in_year(year)
                .as_slice()
                .contains(&day)
        })
        .collect()
}

// ─────────────────────────────────────────────────────────────────────────
// Precedence: the office of a day
// ─────────────────────────────────────────────────────────────────────────

/// A day that no rule of its own lists: the office of the season on a
/// weekday that has no other, for [`Office`].
const fn season_day(title: &'static str, class: Class) -> Celebration {
    Celebration {
        title,
        class,
        rule: Rule::Computed(no_days),
    }
}

/// The rule of a [`SEASON`] day, which [`ordo`] places itself.
const fn no_days(_: i64) -> Days {
    Days::new()
}

/// The days of the season an [`Office`] can be of that are not in
/// [`CELEBRATIONS`], because they are every weekday of a season rather
/// than a day the calendar prints: the ferias of Lent and Passiontide
/// (III class, no. 25a), of Advent to 16 December (III class, no. 25b),
/// the second to fourth days within the octave of Christmas, on which
/// the feasts of 26–28 December fall (II class, no. 17 of the table), the
/// Saturday Office of our Lady on a Saturday that is a feria of the IV
/// class (no. 78), and the other ferias (IV class, no. 26).
pub static SEASON: [Celebration; 8] = [
    season_day("Feria of Lent", Class::Third),
    season_day("Feria of Passiontide", Class::Third),
    season_day("Feria of Advent", Class::Third),
    season_day("II day within the octave of Christmas", Class::Second),
    season_day("III day within the octave of Christmas", Class::Second),
    season_day("IV day within the octave of Christmas", Class::Second),
    season_day("Saturday Office of our Lady", Class::Fourth),
    season_day("Feria", Class::Fourth),
];

const FERIA_OF_LENT: &Celebration = &SEASON[0];
const FERIA_OF_PASSIONTIDE: &Celebration = &SEASON[1];
const FERIA_OF_ADVENT: &Celebration = &SEASON[2];
const OCTAVE_OF_CHRISTMAS: [&Celebration; 3] = [&SEASON[3], &SEASON[4], &SEASON[5]];
const SATURDAY_OF_OUR_LADY: &Celebration = &SEASON[6];
const FERIA: &Celebration = &SEASON[7];

/// The places of the Table of Liturgical Days arranged according to order
/// of precedence (no. 91) that are not their class's default: each place
/// and the titles in it. A title of the I class not here is no. 11, "Feasts
/// I class of the universal Church not mentioned above"; of the II class,
/// no. 16, a Sunday no. 15 and a vigil no. 21; of the III class no. 24.
const PLACES: &[(u8, &[&str])] = &[
    (
        1,
        &["Christmas Day", "Easter Sunday", "Pentecost or Whit Sunday"],
    ),
    (
        2,
        &[
            "Thursday of Holy Week",
            "Friday of Holy Week",
            "Saturday of Holy Week",
        ],
    ),
    (
        3,
        &[
            "The Epiphany of our Lord",
            "Ascension of our Lord",
            "Feast of Blessed Trinity",
            "Feast of Corpus Christi",
            "Feast of the Sacred Heart",
            "Christ, the King",
        ],
    ),
    (
        4,
        &[
            "The Immaculate Conception of our Lady",
            "The Assumption of our Lady",
        ],
    ),
    (5, &["Vigil of Christmas", "Octave-day of Christmas"]),
    (
        6,
        &[
            "First Sunday of Advent",
            "Second Sunday of Advent",
            "Third Sunday of Advent",
            "Fourth Sunday of Advent",
            "First Sunday of Lent",
            "Second Sunday of Lent",
            "Third Sunday of Lent",
            "Fourth Sunday of Lent",
            "First Sunday of Passiontide",
            "Second Sunday of Passiontide or Palm Sunday",
            "Low Sunday",
        ],
    ),
    (
        7,
        &[
            "Ash Wednesday",
            "Monday of Holy Week",
            "Tuesday of Holy Week",
            "Wednesday of Holy Week",
        ],
    ),
    (8, &["All Souls' Day"]),
    (9, &["Vigil of Pentecost"]),
    (
        10,
        &[
            "Monday within the octave of Easter",
            "Tuesday within the octave of Easter",
            "Wednesday within the octave of Easter",
            "Thursday within the octave of Easter",
            "Friday within the octave of Easter",
            "Saturday within the octave of Easter",
            "Monday within the octave of Pentecost",
            "Tuesday within the octave of Pentecost",
            "Wednesday within the octave of Pentecost",
            "Thursday within the octave of Pentecost",
            "Friday within the octave of Pentecost",
            "Saturday within the octave of Pentecost",
        ],
    ),
    (14, FEASTS_OF_THE_LORD_OF_THE_SECOND_CLASS),
    (
        17,
        &[
            "II day within the octave of Christmas",
            "III day within the octave of Christmas",
            "IV day within the octave of Christmas",
            "V day within the octave of Christmas",
            "VI day within the octave of Christmas",
            "VII day within the octave of Christmas",
        ],
    ),
    (
        18,
        &[
            "Feria of Advent, 17 December",
            "Feria of Advent, 18 December",
            "Feria of Advent, 19 December",
            "Feria of Advent, 20 December",
            "Feria of Advent, 21 December",
            "Feria of Advent, 22 December",
            "Feria of Advent, 23 December",
            "Ember Wednesday of Advent",
            "Ember Friday of Advent",
            "Ember Saturday of Advent",
            "Ember Wednesday of Lent",
            "Ember Friday of Lent",
            "Ember Saturday of Lent",
            "Ember Wednesday of September",
            "Ember Friday of September",
            "Ember Saturday of September",
        ],
    ),
    (22, &["Feria of Lent", "Feria of Passiontide"]),
    (25, &["Feria of Advent"]),
    (26, &["Vigil of S. Laurence, M."]),
    (27, &["Saturday Office of our Lady"]),
    (28, &["Feria"]),
];

/// The feasts of the Lord of the II class, the movable ones first, as the
/// Table of Liturgical Days lists them before the feasts of our Lady, with
/// the Purification, which the Changes in the Breviary say "is considered
/// a feast of the Lord" (`rubrics-1960`, pp. 112, 118–).
const FEASTS_OF_THE_LORD_OF_THE_SECOND_CLASS: &[&str] = &[
    "The Holy Name of Jesus",
    "The Holy Family of Jesus, Mary and Joseph",
    "Commemoration of the Baptism of our Lord",
    "The Purification of our Lady",
    "The Transfiguration of our Lord",
    "The Exaltation of the Holy Cross",
    "Dedication of the Archbasilica of our Saviour",
];

/// The movable feasts of the Lord of the II class, which go before the
/// fixed ones of the same place (no. 91, 14).
const MOVABLE_FEASTS_OF_THE_LORD: &[&str] = &[
    "The Holy Name of Jesus",
    "The Holy Family of Jesus, Mary and Joseph",
];

/// The feasts of the Lord of the I class: each takes the place of a
/// Sunday it falls on, which is then not commemorated (nos. 16a, 17), and
/// excludes the commemoration of another feast of the Lord (no. 112a).
const FEASTS_OF_THE_LORD_OF_THE_FIRST_CLASS: &[&str] = &[
    "Christmas Day",
    "Octave-day of Christmas",
    "The Epiphany of our Lord",
    "Easter Sunday",
    "Ascension of our Lord",
    "Feast of Blessed Trinity",
    "Feast of Corpus Christi",
    "Feast of the Sacred Heart",
    "The Precious Blood of our Lord",
    "Christ, the King",
];

/// Whether a celebration is a feast of the Lord, of the I or II class.
fn is_feast_of_the_lord(celebration: &Celebration) -> bool {
    FEASTS_OF_THE_LORD_OF_THE_FIRST_CLASS.contains(&celebration.title)
        || FEASTS_OF_THE_LORD_OF_THE_SECOND_CLASS.contains(&celebration.title)
}

/// The place of a liturgical day in the Table of Liturgical Days arranged
/// according to order of precedence of no. 91, 1 to 28, for the universal
/// calendar: the places 12, 13, 19, 20 and 23 are the particular calendars'
/// and hold nothing here. A commemoration is not a liturgical day and has
/// none.
#[must_use]
pub fn precedence(celebration: &Celebration) -> Option<u8> {
    if let Some((place, _)) = PLACES
        .iter()
        .find(|(_, titles)| titles.contains(&celebration.title))
    {
        return Some(*place);
    }
    let title = celebration.title;
    match celebration.class {
        Class::First => Some(11),
        Class::Second if title.contains("Sunday") => Some(15),
        Class::Second if title.starts_with("Vigil") => Some(21),
        Class::Second => Some(16),
        Class::Third => Some(24),
        Class::Fourth => Some(28),
        Class::Commemoration => None,
    }
}

/// Whether a liturgical day is of the season — a Sunday, a feria, a vigil
/// or a day within an octave — rather than a feast.
fn is_of_the_season(celebration: &Celebration) -> bool {
    let title = celebration.title;
    title.contains("Sunday")
        || title.starts_with("Vigil")
        || title.contains("within the octave")
        || title.contains("of Holy Week")
        || title.starts_with("Feria")
        || title.starts_with("Ember")
        || title == "Ash Wednesday"
        || title == "Saturday Office of our Lady"
}

/// Whether an impeded day is transferred rather than commemorated or
/// omitted: a feast of the I class (no. 95). All Souls' Day has its own
/// rule (no. 96b), which its [`Rule`] applies.
fn is_transferable(celebration: &Celebration) -> bool {
    celebration.class == Class::First && !is_of_the_season(celebration)
}

/// Whether the commemoration of an impeded day is privileged (no. 109): a
/// Sunday, a day of the I class, a day within the octave of Christmas,
/// the ferias of Advent, Lent and Passiontide, and the Ember Days, place 18
/// with the greater ferias of Advent.
fn is_privileged(celebration: &Celebration) -> bool {
    celebration.title.contains("Sunday")
        || celebration.class == Class::First
        || precedence(celebration) == Some(17)
        || matches!(precedence(celebration), Some(18 | 22 | 25))
}

/// The inseparable commemorations of no. 110: St Peter in the office of
/// St Paul and St Paul in St Peter's, which the calendar prints beside
/// the feast, as their titles there.
const INSEPARABLE: &[&str] = &["S. Peter, Ap.", "S. Paul, Ap."];

/// The feasts whose first vigil precedes them, for no. 33: a vigil is
/// omitted when its feast is transferred or reduced to a commemoration.
const VIGILS: &[(&str, &str)] = &[
    (
        "Vigil of the Ascension of our Lord",
        "Ascension of our Lord",
    ),
    (
        "Vigil of the Birthday of S. John the Baptist",
        "The Birthday of S. John the Baptist",
    ),
    (
        "Vigil of SS. Peter and Paul, Apostles",
        "SS. Peter and Paul, App.",
    ),
    ("Vigil of S. Laurence, M.", "S. Laurence, M."),
    (
        "Vigil of the Assumption of our Lady",
        "The Assumption of our Lady",
    ),
];

/// What the rubrics of 1960 make of a day: the liturgical day whose
/// office and Mass are said, the days commemorated in it, and the days
/// that fall on it and are not kept there.
#[derive(Debug, Clone)]
pub struct Office {
    /// The day.
    pub day: Rd,
    /// The liturgical day kept.
    pub office: &'static Celebration,
    /// For a feast of the I class transferred here (no. 96), the day it was
    /// impeded on.
    pub transferred_from: Option<Rd>,
    /// The commemorations made, in their order (no. 113): the season first,
    /// then the order of the table of precedence, then the commemorations
    /// the calendar prints, with an inseparable one (no. 110) at once after
    /// its apostle and not counted.
    pub commemorations: Vec<&'static Celebration>,
    /// The feasts of the I class impeded here and transferred (no. 96).
    pub transferred: Vec<&'static Celebration>,
    /// The days the calendar lists here that are neither kept,
    /// commemorated nor transferred: impeded, and "omitted completely in
    /// that year" (no. 95), or beyond the number of commemorations the day
    /// allows (no. 114). The ferias of the IV class, which "are never
    /// commemorated" (no. 26), are not listed.
    pub omitted: Vec<&'static Celebration>,
}

/// One liturgical day competing for a day's office.
#[derive(Clone, Copy)]
struct Candidate {
    celebration: &'static Celebration,
    /// Where it stands in [`CELEBRATIONS`], or past it for a [`SEASON`] day.
    index: usize,
    transferred_from: Option<Rd>,
}

impl Candidate {
    fn key(&self) -> (u8, u8, usize) {
        let movable_first = u8::from(!MOVABLE_FEASTS_OF_THE_LORD.contains(&self.celebration.title));
        (
            precedence(self.celebration).unwrap_or(u8::MAX),
            movable_first,
            self.index,
        )
    }
}

/// Where a [`SEASON`] day sorts after the entries of [`CELEBRATIONS`].
const SEASON_INDEX: usize = usize::MAX / 2;

/// The office of every day of a Gregorian year under the rubrics of 1960,
/// with precedence applied: the Table of Liturgical Days (no. 91), the
/// transfer of an impeded feast of the I class to the nearest following
/// day not of the I or II class, the Annunciation's to the Monday after
/// Low Sunday (nos. 95–99), the vigils omitted on a Sunday or a feast of
/// the I class or when their feast is not kept (no. 33), and the
/// commemorations the day allows (nos. 106–114). `docs/systems/roman-
/// calendar-1960.md` says what is and is not applied.
///
/// Returns `None` before [`FIRST_YEAR`], when the rubrics were not yet in
/// force, and after 4099, the last year whose Easter the Gregorian computus
/// gives.
#[must_use]
pub fn ordo(year: i64) -> Option<Vec<Office>> {
    if year < FIRST_YEAR {
        return None;
    }
    let easter = computus::gregorian_easter(year)?;
    let first = gregorian::to_fixed(year, 1, 1).ok()?;
    let last = gregorian::to_fixed(year, 12, 31).ok()?;
    let length = usize::try_from(last.0 - first.0 + 1).ok()?;
    let mut listed: Vec<Vec<usize>> = (0..length).map(|_| Vec::new()).collect();
    for (index, celebration) in CELEBRATIONS.iter().enumerate() {
        for day in celebration.rule.days_in_year(year).as_slice() {
            if let Some(slot) = usize::try_from(day.0 - first.0)
                .ok()
                .and_then(|offset| listed.get_mut(offset))
            {
                slot.push(index);
            }
        }
    }
    let year_days = YearDays {
        first,
        easter,
        listed,
    };
    // No. 33 omits a vigil whose feast is not kept, which is known only
    // once the feast's day is settled: run until the omitted vigils stop
    // changing.
    let mut omitted_vigils: Vec<Rd> = Vec::new();
    loop {
        let offices = year_days.offices(&omitted_vigils);
        let mut now: Vec<Rd> = Vec::new();
        for office in &offices {
            for index in &year_days.listed[office.day.0.abs_diff(first.0) as usize] {
                let title = CELEBRATIONS[*index].title;
                if let Some((_, feast)) = VIGILS.iter().find(|(vigil, _)| *vigil == title)
                    && let Some(next) = offices.iter().find(|o| o.day.0 == office.day.0 + 1)
                    && next.office.title != *feast
                {
                    now.push(office.day);
                }
            }
        }
        if now == omitted_vigils {
            return Some(offices);
        }
        omitted_vigils = now;
    }
}

/// The office of a day under the rubrics of 1960; see [`ordo`].
///
/// ```
/// use hc_holiday::roman_calendar_1960::office_on;
/// use hc_calendars_solar::gregorian;
///
/// // 25 March 1962 was the Third Sunday of Lent: the Annunciation, a
/// // feast of the I class, went to Monday 26 March.
/// let sunday = office_on(gregorian::to_fixed(1962, 3, 25)?).expect("in range");
/// assert_eq!(sunday.office.title, "Third Sunday of Lent");
/// let monday = office_on(gregorian::to_fixed(1962, 3, 26)?).expect("in range");
/// assert_eq!(monday.office.title, "The Annunciation of the Blessed Virgin Mary");
/// # Ok::<(), hc_calendar::CalendarError>(())
/// ```
#[must_use]
pub fn office_on(day: Rd) -> Option<Office> {
    let year = gregorian::year_from_fixed(day).ok()?;
    ordo(year)?.into_iter().find(|office| office.day == day)
}

/// The days of a year and what the calendar lists on each.
struct YearDays {
    first: Rd,
    easter: Rd,
    /// For each day from 1 January, the indices of [`CELEBRATIONS`] on it.
    listed: Vec<Vec<usize>>,
}

impl YearDays {
    /// The day of the season on a weekday that no entry of the season
    /// holds.
    fn season(&self, day: Rd) -> Option<&'static Celebration> {
        let weekday = Weekday::from_rd(day);
        if weekday == Weekday::Sunday {
            return None;
        }
        let from_easter = day.0 - self.easter.0;
        if (-45..=-15).contains(&from_easter) {
            return Some(FERIA_OF_LENT);
        }
        if (-13..=-8).contains(&from_easter) {
            return Some(FERIA_OF_PASSIONTIDE);
        }
        let (year, month, date) = gregorian::from_fixed(day).ok()?;
        let advent = Weekday::Sunday.on_or_after(gregorian::to_fixed(year, 11, 27).ok()?);
        if day > advent && (month == 11 || date <= 16) {
            return Some(FERIA_OF_ADVENT);
        }
        if month == 12 && (26..=28).contains(&date) {
            return OCTAVE_OF_CHRISTMAS.get(usize::from(date - 26)).copied();
        }
        Some(if weekday == Weekday::Saturday {
            SATURDAY_OF_OUR_LADY
        } else {
            FERIA
        })
    }

    fn offices(&self, omitted_vigils: &[Rd]) -> Vec<Office> {
        let mut offices = Vec::with_capacity(self.listed.len());
        // The impeded feasts of the I class not yet placed, with the day
        // each was impeded on.
        let mut pending: Vec<Candidate> = Vec::new();
        // The Annunciation's day when it "must be transferred until after
        // Easter": the Monday after Low Sunday, "as to its proper place"
        // (no. 96a).
        let mut annunciation_after_easter: Option<(Rd, Candidate)> = None;
        for (offset, indices) in self.listed.iter().enumerate() {
            let day = Rd(self.first.0 + offset as i64);
            let is_sunday = Weekday::from_rd(day) == Weekday::Sunday;
            let listed: Vec<Candidate> = indices
                .iter()
                .map(|&index| Candidate {
                    celebration: &CELEBRATIONS[index],
                    index,
                    transferred_from: None,
                })
                .collect();
            let has_first_class_feast = listed.iter().any(|c| is_transferable(c.celebration));
            let mut candidates: Vec<Candidate> = Vec::new();
            let mut commemorated: Vec<Candidate> = Vec::new();
            let mut omitted: Vec<&'static Celebration> = Vec::new();
            for candidate in &listed {
                let celebration = candidate.celebration;
                if celebration.class == Class::Commemoration {
                    commemorated.push(*candidate);
                } else if celebration.title.starts_with("Vigil")
                    && celebration.class != Class::First
                    && (is_sunday || has_first_class_feast || omitted_vigils.contains(&day))
                {
                    // No. 33: "omitted entirely".
                    omitted.push(celebration);
                } else {
                    candidates.push(*candidate);
                }
            }
            if !candidates.iter().any(|c| is_of_the_season(c.celebration))
                && let Some(season) = self.season(day)
            {
                candidates.push(Candidate {
                    celebration: season,
                    index: SEASON_INDEX,
                    transferred_from: None,
                });
            }
            if let Some((pinned, candidate)) = annunciation_after_easter
                && pinned == day
            {
                candidates.push(candidate);
                annunciation_after_easter = None;
            }
            candidates.sort_by_key(Candidate::key);
            // A transferred feast goes to the nearest day "which is not I or
            // II class" (no. 96), in the order of the table, the first
            // impeded first among equals (nos. 97–98).
            let is_free = candidates
                .first()
                .is_none_or(|c| matches!(c.celebration.class, Class::Third | Class::Fourth));
            if is_free && !pending.is_empty() {
                let best = (0..pending.len())
                    .min_by_key(|&i| (pending[i].key().0, pending[i].transferred_from, i))
                    .unwrap_or(0);
                let arriving = pending.remove(best);
                candidates.insert(0, arriving);
            }
            let Some((office, losers)) = candidates.split_first() else {
                continue;
            };
            let mut transferred = Vec::new();
            for loser in losers {
                if is_transferable(loser.celebration) && loser.transferred_from.is_none() {
                    transferred.push(loser.celebration);
                    let moved = Candidate {
                        transferred_from: Some(day),
                        ..*loser
                    };
                    let from_easter = day.0 - self.easter.0;
                    if loser.celebration.title == ANNUNCIATION && (-7..=7).contains(&from_easter) {
                        annunciation_after_easter = Some((Rd(self.easter.0 + 8), moved));
                    } else {
                        pending.push(moved);
                    }
                } else if loser.celebration.class == Class::Fourth {
                    // No. 26: never commemorated.
                } else {
                    commemorated.push(*loser);
                }
            }
            let (commemorations, dropped) =
                commemorations(office.celebration, is_sunday, &commemorated);
            omitted.extend(dropped);
            offices.push(Office {
                day,
                office: office.celebration,
                transferred_from: office.transferred_from,
                commemorations,
                transferred,
                omitted,
            });
        }
        offices
    }
}

/// The title of the Annunciation, whose transfer after Easter no. 96a
/// gives a day of its own.
const ANNUNCIATION: &str = "The Annunciation of the Blessed Virgin Mary";

/// The commemorations an office allows of the days impeded on its day,
/// and the ones it omits (nos. 30, 106–114, 16a).
fn commemorations(
    office: &'static Celebration,
    day_is_sunday: bool,
    impeded: &[Candidate],
) -> (Vec<&'static Celebration>, Vec<&'static Celebration>) {
    let mut omitted = Vec::new();
    // No. 30: the vigils of the I class "do not admit any commemoration".
    if matches!(office.title, "Vigil of Christmas" | "Vigil of Pentecost") {
        omitted.extend(impeded.iter().map(|c| c.celebration));
        return (Vec::new(), omitted);
    }
    let office_is_of_the_lord = is_feast_of_the_lord(office);
    // A feast of the Lord on a Sunday "takes the place of the Sunday with
    // all rights and privileges" (no. 16a), and so allows what a Sunday of
    // the II class allows.
    let replaces_a_sunday = office_is_of_the_lord && day_is_sunday;
    let office_is_sunday = office.title.contains("Sunday") || replaces_a_sunday;
    // The candidates in the order no. 113 sets: the season first, then the
    // table of precedence, then the calendar's commemorations as printed.
    let mut ordered: Vec<&Candidate> = impeded
        .iter()
        .filter(|c| !INSEPARABLE.contains(&c.celebration.title))
        .collect();
    ordered.sort_by_key(|c| {
        (
            u8::from(!is_of_the_season(c.celebration)),
            precedence(c.celebration).unwrap_or(u8::MAX),
            c.index,
        )
    });
    let privileged_first = ordered.iter().any(|c| is_privileged(c.celebration));
    let place = match precedence(office).unwrap_or(u8::MAX) {
        14 if replaces_a_sunday => 15,
        place => place,
    };
    let mut kept: Vec<&'static Celebration> = Vec::new();
    let mut counted = 0usize;
    // No. 112c: "the Office, Mass or commemoration of the season excludes
    // another commemoration of the season".
    let mut season_taken = is_of_the_season(office);
    for candidate in ordered {
        let celebration = candidate.celebration;
        let seasonal = is_of_the_season(celebration);
        let excluded = (office_is_of_the_lord
            && (celebration.title.contains("Sunday") || is_feast_of_the_lord(celebration)))
            || (office_is_sunday && is_feast_of_the_lord(celebration))
            || (seasonal && season_taken);
        let allowed = !excluded
            && match place {
                // No. 111a, and no. 23 for the ferias of the I class.
                ..=13 => is_privileged(celebration) && counted < 1,
                // No. 111b: one, of a feast of the II class, and none if a
                // privileged commemoration must be made.
                15 => {
                    counted < 1
                        && if privileged_first {
                            is_privileged(celebration)
                        } else {
                            celebration.class == Class::Second
                        }
                }
                // No. 111c.
                14..=21 => counted < 1,
                // No. 111d.
                _ => counted < 2,
            };
        if allowed {
            kept.push(celebration);
            counted += 1;
            season_taken |= seasonal;
            // No. 110: the other apostle at once after, not counted.
            if let Some(partner) = inseparable_partner(candidate, impeded) {
                kept.push(partner);
            }
        } else {
            omitted.push(celebration);
            if let Some(partner) = inseparable_partner(candidate, impeded) {
                omitted.push(partner);
            }
        }
    }
    // The inseparable commemoration of the office's own apostle comes
    // first and is not counted (no. 110a, b).
    if let Some(partner) = impeded.iter().find(|c| {
        INSEPARABLE.contains(&c.celebration.title)
            && c.index > 0
            && CELEBRATIONS
                .get(c.index - 1)
                .is_some_and(|f| core::ptr::eq(f, office))
    }) {
        kept.insert(0, partner.celebration);
    }
    (kept, omitted)
}

/// The inseparable commemoration printed at once after a feast, if the
/// day lists it.
fn inseparable_partner(feast: &Candidate, impeded: &[Candidate]) -> Option<&'static Celebration> {
    impeded
        .iter()
        .find(|c| INSEPARABLE.contains(&c.celebration.title) && c.index == feast.index + 1)
        .map(|c| c.celebration)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    fn titles_on(year: i64, month: u8, day: u8) -> Vec<&'static str> {
        celebrations_on(ymd(year, month, day))
            .iter()
            .map(|celebration| celebration.title)
            .collect()
    }

    fn count(class: Class) -> usize {
        CELEBRATIONS
            .iter()
            .filter(|celebration| celebration.class == class)
            .count()
    }

    fn office(year: i64, month: u8, day: u8) -> Office {
        office_on(ymd(year, month, day)).expect("in range")
    }

    fn commemorated(office: &Office) -> Vec<&'static str> {
        office.commemorations.iter().map(|c| c.title).collect()
    }

    #[test]
    fn every_title_the_precedence_tables_name_is_a_day() {
        let exists = |title: &str| {
            CELEBRATIONS
                .iter()
                .chain(SEASON.iter())
                .any(|c| c.title == title)
        };
        for (_, titles) in PLACES {
            for title in *titles {
                assert!(exists(title), "{title}");
            }
        }
        for title in FEASTS_OF_THE_LORD_OF_THE_FIRST_CLASS
            .iter()
            .chain(MOVABLE_FEASTS_OF_THE_LORD)
            .chain(INSEPARABLE)
            .chain(VIGILS.iter().flat_map(|(vigil, feast)| [vigil, feast]))
        {
            assert!(exists(title), "{title}");
        }
        // Every liturgical day has a place; the particular calendars'
        // places hold nothing.
        for celebration in CELEBRATIONS.iter().chain(SEASON.iter()) {
            let place = precedence(celebration);
            assert_eq!(place.is_none(), celebration.class == Class::Commemoration);
            assert!(!matches!(place, Some(12 | 13 | 19 | 20 | 23)));
        }
    }

    /// The Sundays of the II class after Epiphany, Easter and Pentecost, as
    /// no. 18 orders them: 2026 has 26 Sundays after Pentecost, so the
    /// Fifth and Sixth after Epiphany are resumed as the 24th and 25th and
    /// the 26th is the Twenty-fourth and last; 2038 has 23, and the
    /// Twenty-third is not kept.
    #[test]
    fn the_sundays_of_the_second_class_are_ordered_as_no_18_says() {
        assert_eq!(
            office(2026, 1, 18).office.title,
            "Second Sunday after Epiphany"
        );
        assert_eq!(
            office(2026, 1, 25).office.title,
            "Third Sunday after Epiphany"
        );
        assert_eq!(
            office(2026, 4, 19).office.title,
            "Second Sunday after Easter"
        );
        assert_eq!(
            office(2026, 5, 17).office.title,
            "Sunday after the Ascension"
        );
        assert_eq!(
            office(2026, 6, 7).office.title,
            "Second Sunday after Pentecost"
        );
        assert_eq!(
            office(2026, 11, 8).office.title,
            "Fifth Sunday after Epiphany, resumed after Pentecost"
        );
        assert_eq!(
            office(2026, 11, 15).office.title,
            "Sixth Sunday after Epiphany, resumed after Pentecost"
        );
        assert_eq!(
            office(2026, 11, 22).office.title,
            "Twenty-fourth and last Sunday after Pentecost"
        );
        // 2038: Pentecost 13 June, Advent 28 November.
        assert_eq!(
            office(2038, 11, 14).office.title,
            "Twenty-second Sunday after Pentecost"
        );
        assert_eq!(
            office(2038, 11, 21).office.title,
            "Twenty-fourth and last Sunday after Pentecost"
        );
        // Every Sunday of every year has one office.
        for year in [1962, 2000, 2011, 2026, 2038, 2100] {
            for o in ordo(year).unwrap() {
                if Weekday::from_rd(o.day) == Weekday::Sunday {
                    assert!(
                        o.office.title.contains("Sunday")
                            || is_feast_of_the_lord(o.office)
                            || o.office.class == Class::First,
                        "{year}: {:?} {}",
                        gregorian::from_fixed(o.day),
                        o.office.title
                    );
                }
            }
        }
    }

    /// The ordo of propria.org for 2019–2026 (`propria-ordo`), which prints
    /// each day's Mass, class and commemorations: days that exercise the
    /// rules, as it prints them.
    #[test]
    fn the_ordo_is_the_published_ordo_on_the_days_that_test_the_rules() {
        // (date, the office, its commemorations)
        type Day = ((i64, u8, u8), &'static str, &'static [&'static str]);
        let days: &[Day] = &[
            // St Joseph on the Fourth Sunday of Lent, transferred (no. 96).
            ((2023, 3, 19), "Fourth Sunday of Lent", &[]),
            (
                (2023, 3, 20),
                "S. Joseph, Husband of our Lady, Cf., Patron of Universal Church",
                &["Feria of Lent"],
            ),
            // The Annunciation in Holy Week, to the Monday after Low Sunday.
            ((2024, 3, 25), "Monday of Holy Week", &[]),
            (
                (2024, 4, 8),
                "The Annunciation of the Blessed Virgin Mary",
                &[],
            ),
            // St John the Baptist after the Sacred Heart; his vigil omitted.
            ((2022, 6, 23), "Feria", &[]),
            ((2022, 6, 24), "Feast of the Sacred Heart", &[]),
            ((2022, 6, 25), "The Birthday of S. John the Baptist", &[]),
            // The Immaculate Conception on a feria of Advent.
            (
                (2025, 12, 8),
                "The Immaculate Conception of our Lady",
                &["Feria of Advent"],
            ),
            // The feasts within the octave of Christmas, the octave day
            // commemorated; the Sunday, with the Holy Innocents.
            (
                (2025, 12, 26),
                "S. Stephen, Protomartyr",
                &["II day within the octave of Christmas"],
            ),
            (
                (2025, 12, 28),
                "Sunday within the octave of Christmas",
                &["The Holy Innocents, MM."],
            ),
            (
                (2025, 12, 29),
                "V day within the octave of Christmas",
                &["S. Thomas, Bp., M."],
            ),
            // The Holy Family on its Sunday, as a Sunday, allowing only a
            // feast of the II class: St Hyginus omitted.
            (
                (2026, 1, 11),
                "The Holy Family of Jesus, Mary and Joseph",
                &[],
            ),
            // A feast of the III class on a Sunday of the II class: omitted.
            ((2026, 1, 25), "Third Sunday after Epiphany", &[]),
            // A commemoration on a feria and on a Saturday of our Lady.
            ((2026, 2, 3), "Feria", &["S. Blaise, Bp., M."]),
            (
                (2026, 2, 14),
                "Saturday Office of our Lady",
                &["S. Valentine, M."],
            ),
            // A feast of the II class in Lent, the feria commemorated; a
            // feast of the III class in Lent, reduced to a commemoration.
            ((2026, 2, 24), "S. Matthias, Ap.", &["Feria of Lent"]),
            (
                (2026, 3, 6),
                "Feria of Lent",
                &["SS. Perpetua and Felicitas, MM."],
            ),
            // Christ the King and All Saints on Sundays after Pentecost.
            ((2026, 10, 25), "Christ, the King", &[]),
            (
                (2026, 11, 1),
                "All Saints",
                &["Twenty-third Sunday after Pentecost"],
            ),
            // St Andrew on the Monday after the First Sunday of Advent.
            ((2026, 11, 30), "S. Andrew, Ap.", &["Feria of Advent"]),
        ];
        for &((year, month, day), title, commemorations) in days {
            let o = office(year, month, day);
            assert_eq!(o.office.title, title, "{year}-{month}-{day}");
            assert_eq!(commemorated(&o), commemorations, "{year}-{month}-{day}");
        }
    }

    /// The transfers of no. 96–98 on the years that meet them.
    #[test]
    fn an_impeded_feast_of_the_first_class_goes_to_the_nearest_free_day() {
        // 2035: St Joseph on Monday of Holy Week and the Annunciation on
        // Easter Sunday. The Annunciation goes to the Monday after Low
        // Sunday "as to its proper place" and St Joseph, impeded as far,
        // to the Tuesday.
        let annunciation = office(2035, 4, 2);
        assert_eq!(annunciation.office.title, ANNUNCIATION);
        assert_eq!(annunciation.transferred_from, Some(ymd(2035, 3, 25)));
        let joseph = office(2035, 4, 3);
        assert!(joseph.office.title.starts_with("S. Joseph, Husband"));
        assert_eq!(joseph.transferred_from, Some(ymd(2035, 3, 19)));
        assert_eq!(
            office(2035, 3, 19).transferred[0].title,
            joseph.office.title
        );
        // 2011: the Precious Blood on the Sacred Heart, 1 July, passes the
        // Visitation (II class) and a Sunday (II class) to Monday 4 July.
        assert_eq!(office(2011, 7, 1).office.title, "Feast of the Sacred Heart");
        assert_eq!(
            office(2011, 7, 2).office.title,
            "The Visitation of our Lady"
        );
        assert_eq!(
            office(2011, 7, 4).office.title,
            "The Precious Blood of our Lord"
        );
        // 1962: the Annunciation on the Third Sunday of Lent, to Monday
        // 26 March, with the feria commemorated.
        let monday = office(1962, 3, 26);
        assert_eq!(monday.office.title, ANNUNCIATION);
        assert_eq!(commemorated(&monday), ["Feria of Lent"]);
    }

    #[test]
    fn the_sunday_and_the_vigils_give_way_as_the_rubrics_say() {
        // No. 16a: the Transfiguration on a Sunday takes its place, and the
        // Sunday is not commemorated.
        let transfiguration = office(2028, 8, 6);
        assert_eq!(
            transfiguration.office.title,
            "The Transfiguration of our Lord"
        );
        assert!(transfiguration.commemorations.is_empty());
        assert!(
            transfiguration
                .omitted
                .iter()
                .any(|c| c.title.contains("Sunday"))
        );
        // No. 111b: a feast of the II class on a Sunday is commemorated.
        let james = office(2027, 7, 25);
        assert!(james.office.title.contains("Sunday after Pentecost"));
        assert_eq!(commemorated(&james), ["S. James, Ap."]);
        // No. 33: the vigil of St Laurence on a Sunday is omitted.
        let vigil = office(2026, 8, 9);
        assert_eq!(vigil.office.title, "Eleventh Sunday after Pentecost");
        assert!(
            vigil
                .omitted
                .iter()
                .any(|c| c.title == "Vigil of S. Laurence, M.")
        );
        // No. 30: the Vigil of Christmas on the Fourth Sunday of Advent,
        // which is not commemorated.
        let eve = office(2023, 12, 24);
        assert_eq!(eve.office.title, "Vigil of Christmas");
        assert!(eve.commemorations.is_empty());
        // No. 15: the Immaculate Conception on the Second Sunday of Advent,
        // which is commemorated.
        let conception = office(2024, 12, 8);
        assert_eq!(commemorated(&conception), ["Second Sunday of Advent"]);
        // No. 110: St Paul under St Peter's Chair, not counted, then the
        // one commemoration a day of the II class allows; on the First
        // Sunday of Lent both are omitted.
        assert_eq!(
            commemorated(&office(2027, 2, 22)),
            ["S. Paul, Ap.", "Feria of Lent"]
        );
        let lent = office(2026, 2, 22);
        assert_eq!(lent.office.title, "First Sunday of Lent");
        assert!(lent.commemorations.is_empty());
        assert_eq!(lent.omitted.len(), 2);
        // Before the rubrics came into force, and outside the computus's
        // years, there is no ordo.
        assert!(ordo(1582).is_none());
        assert!(ordo(1583).is_none());
        assert!(ordo(1960).is_none());
        assert!(ordo(1961).is_some());
        assert!(office_on(ymd(4100, 1, 1)).is_none());
    }

    /// divinumofficium.com's "Rubrics 1960" kalendar: "Feria Quarta
    /// Quattuor Temporum Septembris", II classis, on 22 September 2027, and
    /// the Friday and Saturday on the 24th and 25th; those of Lent on 17, 19
    /// and 20 February 2027 and of Advent on 15, 17 and 18 December 2027.
    #[test]
    fn the_ember_days_of_1960_are_ferias_of_the_second_class() {
        for (month, day, title) in [
            (2, 17, "Ember Wednesday of Lent"),
            (2, 19, "Ember Friday of Lent"),
            (2, 20, "Ember Saturday of Lent"),
            (9, 22, "Ember Wednesday of September"),
            (9, 24, "Ember Friday of September"),
            (9, 25, "Ember Saturday of September"),
            (12, 15, "Ember Wednesday of Advent"),
            (12, 17, "Ember Friday of Advent"),
            (12, 18, "Ember Saturday of Advent"),
        ] {
            let kept = office(2027, month, day);
            assert_eq!(kept.office.title, title, "2027-{month}-{day}");
            assert_eq!(kept.office.class, Class::Second);
            assert_eq!(precedence(kept.office), Some(18));
        }
        // St Thomas of Villanova, of the III class, is commemorated on the
        // Wednesday; the greater feria of 17 December is the Ember Friday.
        let wednesday = office(2027, 9, 22);
        assert!(
            wednesday
                .commemorations
                .iter()
                .any(|c| c.title.starts_with("S. Thomas of Villanova"))
        );
        assert!(
            celebrations_on(ymd(2027, 12, 17))
                .iter()
                .all(|c| c.title != "Feria of Advent, 17 December")
        );
        // propria.org's ordo for 2026 (`propria-ordo`): "Ember Wednesday of
        // Advent", class 2, on 17 December 2025, and the Friday and Saturday
        // on the 19th and 20th; "Ember Wednesday in September" with St
        // Linus commemorated on 23 September 2026, and the Saturday with Sts
        // Cyprian and Justina on the 26th.
        for (year, month, day, title) in [
            (2025, 12, 17, "Ember Wednesday of Advent"),
            (2025, 12, 19, "Ember Friday of Advent"),
            (2025, 12, 20, "Ember Saturday of Advent"),
            (2026, 9, 25, "Ember Friday of September"),
        ] {
            assert_eq!(office(year, month, day).office.title, title);
        }
        let linus = office(2026, 9, 23);
        assert_eq!(linus.office.title, "Ember Wednesday of September");
        assert_eq!(
            linus
                .commemorations
                .iter()
                .map(|c| c.title)
                .collect::<Vec<_>>(),
            ["S. Linus, Pope, M."]
        );
        let saturday = office(2026, 9, 26);
        assert_eq!(saturday.office.title, "Ember Saturday of September");
        assert!(
            saturday
                .commemorations
                .iter()
                .any(|c| c.title == "SS. Cyprian and Justina, V., MM.")
        );
        // The earliest September week, 14 September a Saturday (2024): the
        // 18th, 20th and 21st.
        assert_eq!(
            office(2024, 9, 18).office.title,
            "Ember Wednesday of September"
        );
        // St Matthew, of the II class, goes before the Ember Saturday on
        // 21 September 2024 (no. 91: place 16 before 18), which is
        // commemorated, a privileged commemoration.
        let matthew = office(2024, 9, 21);
        assert_eq!(matthew.office.title, "S. Matthew, Ap. and Evang.");
        assert!(
            matthew
                .commemorations
                .iter()
                .any(|c| c.title == "Ember Saturday of September")
        );
        // The latest, 14 September a Sunday (2025): the 24th, 26th and 27th.
        assert_eq!(
            office(2025, 9, 24).office.title,
            "Ember Wednesday of September"
        );
        assert_eq!(
            office(2025, 9, 27).office.title,
            "Ember Saturday of September"
        );
    }

    #[test]
    fn the_calendar_of_1960_begins_in_1961() {
        let days = |year| crate::engine::holidays_in_year(&GENERAL_ROMAN_CALENDAR_1960, None, year);
        assert!(days(1960).is_empty());
        assert!(!days(1961).is_empty());
        assert!(
            crate::engine::HolidayCalendar::for_year(&GENERAL_ROMAN_CALENDAR_1960, None, 1900)
                .is_complete()
        );
        assert!(office_on(ymd(1960, 12, 31)).is_none());
        assert!(office_on(ymd(1961, 1, 1)).is_some());
    }

    #[test]
    fn the_classes_are_counted_as_the_module_states() {
        assert_eq!(count(Class::First), 53);
        assert_eq!(count(Class::Second), 95);
        assert_eq!(count(Class::Third), 181);
        assert_eq!(count(Class::Commemoration), 106);
    }

    /// The Table of Liturgical Days lists the feasts of the I and II class
    /// of the universal calendar (`rubrics-1960`, pp. 111–114): nineteen
    /// feasts and two other days of the first class, and thirty-one feasts
    /// of the second.
    #[test]
    fn the_first_and_second_class_feasts_are_the_tables() {
        let is_feast = |title: &str| {
            !title.starts_with("Vigil")
                && !title.contains("Sunday")
                && !title.contains("of Holy Week")
                && !title.contains("within the octave")
                && !title.contains("day within")
                && !title.starts_with("Feria")
                && !title.starts_with("Ember")
                && title != "Ash Wednesday"
        };
        let first: Vec<&str> = CELEBRATIONS
            .iter()
            .filter(|c| c.class == Class::First && is_feast(c.title))
            .map(|c| c.title)
            .collect();
        // Nineteen feasts, with the octave-day of Christmas and All Souls'
        // Day; Easter and Pentecost are counted among the Sundays.
        assert_eq!(first.len(), 17 + 2, "{first:?}");
        let second = CELEBRATIONS
            .iter()
            .filter(|c| c.class == Class::Second && is_feast(c.title))
            .count();
        assert_eq!(second, 31);
    }

    /// Worked in `docs/systems/roman-calendar-1960.md`: 1962, the year of
    /// the Missal, with Easter on 22 April.
    #[test]
    fn the_movable_days_of_1962_fall_where_the_rules_put_them() {
        for (month, day, title) in [
            (1, 2, "The Holy Name of Jesus"),
            (1, 7, "The Holy Family of Jesus, Mary and Joseph"),
            (2, 18, "Septuagesima Sunday"),
            (3, 7, "Ash Wednesday"),
            (4, 13, "The Seven Sorrows of our Lady"),
            (4, 22, "Easter Sunday"),
            (4, 29, "Low Sunday"),
            (5, 31, "Ascension of our Lord"),
            (6, 10, "Pentecost or Whit Sunday"),
            (6, 17, "Feast of Blessed Trinity"),
            (6, 21, "Feast of Corpus Christi"),
            (6, 29, "Feast of the Sacred Heart"),
            (10, 28, "Christ, the King"),
            (12, 2, "First Sunday of Advent"),
        ] {
            assert!(
                titles_on(1962, month, day).contains(&title),
                "{month}-{day}: {:?}",
                titles_on(1962, month, day)
            );
        }
        // 1962 has no Sunday between 2 and 5 January, so the Holy Name is
        // on 2 January; 1964's is on Sunday 5 January.
        assert!(titles_on(1964, 1, 5).contains(&"The Holy Name of Jesus"));
        assert!(!titles_on(1964, 1, 2).contains(&"The Holy Name of Jesus"));
    }

    #[test]
    fn a_leap_year_moves_st_matthias_and_st_gabriel() {
        assert_eq!(titles_on(1962, 2, 24), ["S. Matthias, Ap."]);
        assert_eq!(titles_on(1964, 2, 25), ["S. Matthias, Ap."]);
        assert!(titles_on(1964, 2, 24).is_empty());
        assert_eq!(
            titles_on(1964, 2, 28),
            ["S. Gabriel of the Sorrowing Virgin, Cf."]
        );
    }

    #[test]
    fn all_souls_day_leaves_a_sunday_for_the_monday() {
        // 2 November 1958 was a Sunday; 1962's was a Friday.
        assert!(titles_on(1958, 11, 3).contains(&"All Souls' Day"));
        assert!(!titles_on(1958, 11, 2).contains(&"All Souls' Day"));
        assert!(titles_on(1962, 11, 2).contains(&"All Souls' Day"));
    }

    /// The calendar's first days of January as the translation prints
    /// them, a feast and its commemoration on 14 January.
    #[test]
    fn a_day_carries_its_feast_and_its_commemoration() {
        assert_eq!(
            titles_on(1963, 1, 14),
            ["S. Hilary, Bp., Cf., Doct.", "S. Felix, priest, M."]
        );
        let hilary = celebrations_on(ymd(1963, 1, 14));
        assert_eq!(hilary[0].class, Class::Third);
        assert_eq!(hilary[1].class, Class::Commemoration);
        // In 1962 14 January was the Second Sunday after Epiphany, which
        // the calendar lists after them and whose office it is.
        assert_eq!(
            titles_on(1962, 1, 14),
            [
                "S. Hilary, Bp., Cf., Doct.",
                "S. Felix, priest, M.",
                "Second Sunday after Epiphany"
            ]
        );
        let sunday = office(1962, 1, 14);
        assert_eq!(sunday.office.title, "Second Sunday after Epiphany");
        assert!(sunday.commemorations.is_empty());
        // The feasts the rubrics of 1960 took out are not here: the Finding
        // of the Cross on 3 May and St Peter's Chains on 1 August.
        assert!(!titles_on(1962, 5, 3).iter().any(|t| t.contains("Cross")));
        assert_eq!(titles_on(1962, 8, 1), ["The Holy Machabees, MM."]);
        let calendar =
            crate::engine::HolidayCalendar::for_year(&GENERAL_ROMAN_CALENDAR_1960, None, 1962);
        assert!(!calendar.is_holiday(ymd(1962, 12, 25)));
        assert_eq!(calendar.on(ymd(1962, 12, 25))[0].kind, Kind::Religious);
    }

    /// Five feasts on their 1960 days and, in [`crate::roman_calendar`],
    /// on the days the 1969 reform gave them.
    #[test]
    fn the_feasts_that_moved_in_1969_are_on_their_1960_days() {
        for (month, day, title, later_month, later_day, later) in [
            (
                7,
                2,
                "The Visitation of our Lady",
                5,
                31,
                "The Visitation of the Blessed Virgin Mary",
            ),
            (2, 24, "S. Matthias, Ap.", 5, 14, "St Matthias, Apostle"),
            (12, 21, "S. Thomas, Ap.", 7, 3, "St Thomas, Apostle"),
            (
                3,
                7,
                "S. Thomas Aquinas, Cf., Doct.",
                1,
                28,
                "St Thomas Aquinas, Priest and Doctor of the Church",
            ),
            (
                5,
                31,
                "The Queenship of our Lady",
                8,
                22,
                "The Queenship of the Blessed Virgin Mary",
            ),
        ] {
            assert!(titles_on(2025, month, day).contains(&title), "{title}");
            let after = crate::roman_calendar::celebrations_on(ymd(2025, later_month, later_day));
            assert!(after.iter().any(|c| c.title == later), "{later}");
        }
        // Christ the King: 26 October 2025 here, 23 November in 1969's.
        assert!(titles_on(2025, 10, 26).contains(&"Christ, the King"));
        let king = crate::roman_calendar::celebrations_on(ymd(2025, 11, 23));
        assert!(
            king.iter()
                .any(|c| c.title.contains("King of the Universe"))
        );
    }
}
