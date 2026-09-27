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
//! | II class | 41 | the Holy Family, the Purification, the apostles, the vigils of the Ascension and of the Assumption |
//! | III class | 181 | St Hilary, St Agnes, the Holy Name of Mary |
//! | Commemoration | 106 | St Telesphorus, St George, Our Lady of Mount Carmel |
//!
//! A fourth class holds the ferias of the year that are none of these; they
//! are not listed. The table carries the calendar as the English
//! translation of the rubrics prints it, month by month, with the titles in
//! its abbreviations ("Bp." bishop, "Cf." confessor, "Doct." doctor, "M."
//! martyr, "V." virgin), a commemoration on the same day as a feast as an
//! entry of its own; the movable feasts the calendar and the Table of
//! Liturgical Days name; and the Sundays, ferias, vigils and days within an
//! octave of the Proper of Time that the Table ranks in the first class,
//! with the Vigil of the Ascension and the three Sundays of Septuagesima,
//! which are of the second.
//!
//! # What this is not
//!
//! As with [`crate::roman_calendar`], this is the calendar, not the *ordo*:
//! when two days fall together the Table of Occurrence decides which is
//! kept and whether the other is commemorated or moved, and that is not
//! applied. The rules the calendar and the rubrics give for a feast's own
//! date are: St Matthias on 25 February and St Gabriel of the Sorrowing
//! Virgin on 28 February in a leap year, and All Souls' Day on 3 November
//! when 2 November is a Sunday (no. 96). The transfer of an impeded feast
//! of the I class, the Annunciation's among them (no. 96), is not
//! applied. The Ember Days, the ferias of Advent from 17 to
//! 23 December, the other Sundays of the second class — after Epiphany,
//! the second to the fifth after Easter, after the Ascension and after
//! Pentecost — and the particular calendars of nations, dioceses and
//! orders are not carried; the Greater and Lesser Litanies are the
//! `rogation-roman-1960` table's.
//!
//! Sources: the Code of Rubrics and its calendar in the English
//! translation *The New Rubrics of the Roman Breviary and Missal* (1960),
//! pp. 98–114 (`rubrics-1960`), read in a scanned copy's text layer,
//! retrieved 2026-09-27; checked day by day against Wikipedia, "General
//! Roman Calendar of 1960" (`wikipedia-grc-1960`), retrieved 2026-09-27,
//! whose class and number of commemorations agree on every day. The Latin
//! text in *Acta Apostolicae Sedis* 52 (1960) was not read.

use alloc::vec::Vec;

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;

use crate::computus::offsets::{
    ASCENSION, ASH_WEDNESDAY, CORPUS_CHRISTI, EASTER_SUNDAY, PALM_SUNDAY, PENTECOST, SACRED_HEART,
    SEPTUAGESIMA, TRINITY_SUNDAY,
};
use crate::rule::{Days, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate};

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
            HolidayRule::observance($title, "", $rule).of_kind(Kind::Religious)
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
    "Septuagesima Sunday", Second, Rule::easter(SEPTUAGESIMA);
    "Sexagesima Sunday", Second, Rule::easter(-56);
    "Quinquagesima Sunday", Second, Rule::easter(-49);
    "Ash Wednesday", First, Rule::easter(ASH_WEDNESDAY);
    "First Sunday of Lent", First, Rule::easter(-42);
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
    "Vigil of the Ascension of our Lord", Second, Rule::easter(ASCENSION - 1);
    "Ascension of our Lord", First, Rule::easter(ASCENSION);
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
}

/// The calendar of 1960 as a rule set: every day a [`Kind::Religious`]
/// observance, with no precedence applied.
pub static GENERAL_ROMAN_CALENDAR_1960: RuleSet = RuleSet {
    code: "roman-general-1960",
    english_name: "General Roman Calendar of 1960",
    rules: RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: "Code of Rubrics approved by John XXIII's motu proprio Rubricarum instructum of \
              25 July 1960, General Rubrics nos. 10-36 and 96, the Calendar of the Roman \
              Breviary and Missal and the Table of Liturgical Days, in the English translation \
              The New Rubrics of the Roman Breviary and Missal (1960), pp. 98-114, read in the \
              copy at cdn.restorethe54.com/media/pdf/the-new-rubrics-of-the-roman-missal-and-\
              breviary-1960.pdf (rubrics-1960), retrieved 2026-09-27; checked against \
              Wikipedia, \"General Roman Calendar of 1960\" (wikipedia-grc-1960), retrieved \
              2026-09-27. The Latin in Acta Apostolicae Sedis 52 (1960) was not read",
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

    #[test]
    fn the_classes_are_counted_as_the_module_states() {
        assert_eq!(count(Class::First), 53);
        assert_eq!(count(Class::Second), 41);
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
            titles_on(1962, 1, 14),
            ["S. Hilary, Bp., Cf., Doct.", "S. Felix, priest, M."]
        );
        let hilary = celebrations_on(ymd(1962, 1, 14));
        assert_eq!(hilary[0].class, Class::Third);
        assert_eq!(hilary[1].class, Class::Commemoration);
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
