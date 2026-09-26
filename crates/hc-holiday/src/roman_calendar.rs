//! The General Roman Calendar: the celebrations the Roman Rite keeps
//! everywhere, each with its rank.
//!
//! The calendar of the Roman Missal of 2002, as the Liturgy Office of the
//! Bishops' Conference of England and Wales publishes it, with the
//! celebrations the Holy See has inscribed or raised since, each from its
//! decree. A celebration is a day of the year — St Anthony on 17 January —
//! or one of the handful the calendar dates by the Sunday or by Easter,
//! and its rank is one of the four the Universal Norms give:
//!
//! | Rank | Count | Examples |
//! | --- | --- | --- |
//! | Solemnity | 14 | the Nativity of the Lord, the Assumption, St Joseph |
//! | Feast | 26 | the Presentation of the Lord, the apostles, St Mary Magdalene |
//! | Memorial | 69 | St Agnes, St Thomas Aquinas, Mary, Mother of the Church |
//! | Optional memorial | 120 | St Blaise, Our Lady of Lourdes, St Hildegard |
//!
//! with the Commemoration of All the Faithful Departed, which the calendar
//! prints without a rank and the Norms place with the solemnities, as a
//! fifth. The counts are for the calendar as it stands; [`Celebration::since`]
//! says from which year a later decree's entry applies. The Proper of Time
//! — Easter, the Ascension, Pentecost and the seasons — is not a
//! celebration of this calendar and is not here; the solemnities and
//! feasts of the Lord that the calendar prints with a rule, the Most Holy
//! Trinity or the Holy Family, are.
//!
//! # What this is not
//!
//! It is the calendar, not the *ordo*. Two celebrations fall on one day —
//! a memorial on a Sunday, a solemnity in Holy Week — and the Table of
//! Liturgical Days decides which is kept and whether the other moves;
//! that precedence is not applied here, so a day can list a memorial the
//! Church does not celebrate that year. Nor are the proper calendars of a
//! country, diocese or religious order, nor the transfers a conference of
//! bishops may make: England and Wales keep the Body and Blood of Christ
//! on the Sunday after the Most Holy Trinity, and this calendar on the
//! Thursday, as the General Roman Calendar has it.
//! The Lectionary's Sunday and weekday cycles, which an ordo also prints,
//! are rules and not editorial choices, and are [`crate::lectionary`]'s.
//!
//! Sources: the General Roman Calendar and the Universal Norms on the
//! Liturgical Year and the General Roman Calendar (`roman-calendar-norms`),
//! which Paul VI approved by the motu proprio *Mysterii Paschalis* of
//! 14 February 1969 (`mysterii-paschalis-1969`), as the Liturgy Office of
//! the Bishops' Conference of England and Wales publishes them
//! (liturgyoffice.org.uk/Calendar/Info/, `liturgyoffice-calendar`),
//! retrieved 2026-09-23: a secondary copy, since the primary, the calendar
//! printed in the *Missale Romanum*, editio typica tertia (2002), was not
//! read. The decrees of the Congregation, now Dicastery, for Divine Worship
//! and the Discipline of the Sacraments cited at each later entry with its
//! Prot. N., from vatican.va, retrieved 2026-09-23; the Prot. N. of the
//! decree of 11 February 2018 is from a copy of the Italian decree, the
//! vatican.va page printing none, and that of 9 November 2025 from the
//! Dicastery's notice of 3 February 2026 on cultodivino.va, both read
//! 2026-09-26.

use alloc::vec::Vec;

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;

use crate::rule::{Days, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate};

/// The rank of a celebration, as the Universal Norms on the Liturgical
/// Year give them (nos. 10–14).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rank {
    /// A solemnity, the highest rank: the principal days.
    Solemnity,
    /// The Commemoration of All the Faithful Departed, printed without a
    /// rank and placed with the solemnities in the Table of Liturgical
    /// Days.
    Commemoration,
    /// A feast, kept within the limits of the natural day.
    Feast,
    /// An obligatory memorial.
    Memorial,
    /// An optional memorial: a celebration the calendar prints without a
    /// rank, as its note says.
    OptionalMemorial,
}

impl Rank {
    /// The rank's English name.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Solemnity => "Solemnity",
            Self::Commemoration => "Commemoration",
            Self::Feast => "Feast",
            Self::Memorial => "Memorial",
            Self::OptionalMemorial => "Optional memorial",
        }
    }
}

/// A celebration of the General Roman Calendar.
#[derive(Debug, Clone, Copy)]
pub struct Celebration {
    /// Its title, as the calendar prints it.
    pub title: &'static str,
    /// Its rank.
    pub rank: Rank,
    /// The day it falls on.
    pub rule: Rule,
    /// The first year it applies in, when a decree inscribed it or gave it
    /// this rank after 2002.
    pub since: Option<i32>,
    /// The last year it applied in, when a decree replaced it.
    pub until: Option<i32>,
}

impl Celebration {
    /// Whether the celebration is in the calendar in `year`.
    #[must_use]
    pub const fn applies_in(&self, year: i64) -> bool {
        let after = match self.since {
            Some(since) => year >= since as i64,
            None => true,
        };
        let before = match self.until {
            Some(until) => year <= until as i64,
            None => true,
        };
        after && before
    }
}

/// The Sunday within the octave of the Nativity, or 30 December when there
/// is none: the Holy Family.
fn holy_family(year: i64) -> Days {
    for day in 26..=31 {
        if let Ok(rd) = gregorian::to_fixed(year, 12, day)
            && Weekday::from_rd(rd) == Weekday::Sunday
        {
            return Days::one(rd);
        }
    }
    gregorian::to_fixed(year, 12, 30).map_or_else(|_| Days::new(), Days::one)
}

/// The first Sunday after 6 January: the Baptism of the Lord.
const BAPTISM_OF_THE_LORD: Rule = Rule::WeekdayOnOrAfter {
    month: 1,
    day: 7,
    weekday: Weekday::Sunday,
};

/// The last Sunday in Ordinary Time, the one before the first of Advent:
/// Christ the King.
const CHRIST_THE_KING: Rule = Rule::WeekdayOnOrAfter {
    month: 11,
    day: 20,
    weekday: Weekday::Sunday,
};

/// `Some` of a year, or `None` when there is none.
macro_rules! optional_year {
    () => {
        None
    };
    ($year:literal) => {
        Some($year)
    };
}

/// The calendar, written once and read twice: as [`CELEBRATIONS`], with
/// the ranks, and as the rule set the engine evaluates.
macro_rules! general_roman_calendar {
    ($(
        $title:literal, $rank:ident, $rule:expr
        $(, since $since:literal)?
        $(, until $until:literal)?
    );* $(;)?) => {
        /// Every celebration of the General Roman Calendar, in calendar
        /// order, the movable ones where the calendar prints them.
        pub static CELEBRATIONS: &[Celebration] = &[$(
            Celebration {
                title: $title,
                rank: Rank::$rank,
                rule: $rule,
                since: optional_year!($($since)?),
                until: optional_year!($($until)?),
            }
        ),*];

        static RULES: &[HolidayRule] = &[$(
            HolidayRule::observance($title, "", $rule)
                .of_kind(Kind::Religious)
                .years(optional_year!($($since)?), optional_year!($($until)?))
        ),*];
    };
}

general_roman_calendar! {
    // January
    "Solemnity of Mary, the Holy Mother of God", Solemnity, Rule::gregorian(1, 1);
    "Sts Basil the Great and Gregory Nazianzen, Bishops and Doctors of the Church", Memorial, Rule::gregorian(1, 2);
    "The Most Holy Name of Jesus", OptionalMemorial, Rule::gregorian(1, 3);
    "The Epiphany of the Lord", Solemnity, Rule::gregorian(1, 6);
    "St Raymond of Penyafort, Priest", OptionalMemorial, Rule::gregorian(1, 7);
    "The Baptism of the Lord", Feast, BAPTISM_OF_THE_LORD;
    "St Hilary, Bishop and Doctor of the Church", OptionalMemorial, Rule::gregorian(1, 13);
    "St Anthony, Abbot", Memorial, Rule::gregorian(1, 17);
    "St Fabian, Pope and Martyr", OptionalMemorial, Rule::gregorian(1, 20);
    "St Sebastian, Martyr", OptionalMemorial, Rule::gregorian(1, 20);
    "St Agnes, Virgin and Martyr", Memorial, Rule::gregorian(1, 21);
    "St Vincent, Deacon and Martyr", OptionalMemorial, Rule::gregorian(1, 22);
    "St Francis de Sales, Bishop and Doctor of the Church", Memorial, Rule::gregorian(1, 24);
    "The Conversion of St Paul, the Apostle", Feast, Rule::gregorian(1, 25);
    "Sts Timothy and Titus, Bishops", Memorial, Rule::gregorian(1, 26);
    "St Angela Merici, Virgin", OptionalMemorial, Rule::gregorian(1, 27);
    "St Thomas Aquinas, Priest and Doctor of the Church", Memorial, Rule::gregorian(1, 28);
    "St John Bosco, Priest", Memorial, Rule::gregorian(1, 31);
    // February
    "The Presentation of the Lord", Feast, Rule::gregorian(2, 2);
    "St Blaise, Bishop and Martyr", OptionalMemorial, Rule::gregorian(2, 3);
    "St Ansgar, Bishop", OptionalMemorial, Rule::gregorian(2, 3);
    "St Agatha, Virgin and Martyr", Memorial, Rule::gregorian(2, 5);
    "St Paul Miki and Companions, Martyrs", Memorial, Rule::gregorian(2, 6);
    "St Jerome Emiliani", OptionalMemorial, Rule::gregorian(2, 8);
    "St Josephine Bakhita, Virgin", OptionalMemorial, Rule::gregorian(2, 8);
    "St Scholastica, Virgin", Memorial, Rule::gregorian(2, 10);
    "Our Lady of Lourdes", OptionalMemorial, Rule::gregorian(2, 11);
    "Sts Cyril, Monk, and Methodius, Bishop", Memorial, Rule::gregorian(2, 14);
    "The Seven Holy Founders of the Servite Order", OptionalMemorial, Rule::gregorian(2, 17);
    "St Peter Damian, Bishop and Doctor of the Church", OptionalMemorial, Rule::gregorian(2, 21);
    "The Chair of St Peter the Apostle", Feast, Rule::gregorian(2, 22);
    "St Polycarp, Bishop and Martyr", Memorial, Rule::gregorian(2, 23);
    // Decree of 25 January 2021, Prot. N. 40/21.
    "St Gregory of Narek, Abbot and Doctor of the Church", OptionalMemorial, Rule::gregorian(2, 27), since 2021;
    // March
    "St Casimir", OptionalMemorial, Rule::gregorian(3, 4);
    "Sts Perpetua and Felicity, Martyrs", Memorial, Rule::gregorian(3, 7);
    "St John of God, Religious", OptionalMemorial, Rule::gregorian(3, 8);
    "St Frances of Rome, Religious", OptionalMemorial, Rule::gregorian(3, 9);
    "St Patrick, Bishop", OptionalMemorial, Rule::gregorian(3, 17);
    "St Cyril of Jerusalem, Bishop and Doctor of the Church", OptionalMemorial, Rule::gregorian(3, 18);
    "St Joseph, Spouse of the Blessed Virgin Mary", Solemnity, Rule::gregorian(3, 19);
    "St Turibius of Mogrovejo, Bishop", OptionalMemorial, Rule::gregorian(3, 23);
    "The Annunciation of the Lord", Solemnity, Rule::gregorian(3, 25);
    // April
    "St Francis of Paola, Hermit", OptionalMemorial, Rule::gregorian(4, 2);
    "St Isidore, Bishop and Doctor of the Church", OptionalMemorial, Rule::gregorian(4, 4);
    "St Vincent Ferrer, Priest", OptionalMemorial, Rule::gregorian(4, 5);
    "St John Baptist de la Salle, Priest", Memorial, Rule::gregorian(4, 7);
    "St Stanislaus, Bishop and Martyr", Memorial, Rule::gregorian(4, 11);
    "St Martin I, Pope and Martyr", OptionalMemorial, Rule::gregorian(4, 13);
    "St Anselm, Bishop and Doctor of the Church", OptionalMemorial, Rule::gregorian(4, 21);
    "St George, Martyr", OptionalMemorial, Rule::gregorian(4, 23);
    "St Adalbert, Bishop and Martyr", OptionalMemorial, Rule::gregorian(4, 23);
    "St Fidelis of Sigmaringen, Priest and Martyr", OptionalMemorial, Rule::gregorian(4, 24);
    "St Mark, Evangelist", Feast, Rule::gregorian(4, 25);
    "St Peter Chanel, Priest and Martyr", OptionalMemorial, Rule::gregorian(4, 28);
    "St Louis Grignion de Montfort, Priest", OptionalMemorial, Rule::gregorian(4, 28);
    "St Catherine of Siena, Virgin and Doctor of the Church", Memorial, Rule::gregorian(4, 29);
    "St Pius V, Pope", OptionalMemorial, Rule::gregorian(4, 30);
    // May
    "St Joseph the Worker", OptionalMemorial, Rule::gregorian(5, 1);
    "St Athanasius, Bishop and Doctor of the Church", Memorial, Rule::gregorian(5, 2);
    "Sts Philip and James, Apostles", Feast, Rule::gregorian(5, 3);
    // Decree of 25 January 2021, Prot. N. 40/21.
    "St John De Avila, Priest and Doctor of the Church", OptionalMemorial, Rule::gregorian(5, 10), since 2021;
    "Sts Nereus and Achilleus, Martyrs", OptionalMemorial, Rule::gregorian(5, 12);
    "St Pancras, Martyr", OptionalMemorial, Rule::gregorian(5, 12);
    "Our Lady of Fatima", OptionalMemorial, Rule::gregorian(5, 13);
    "St Matthias, Apostle", Feast, Rule::gregorian(5, 14);
    "St John I, Pope and Martyr", OptionalMemorial, Rule::gregorian(5, 18);
    "St Bernardine of Siena, Priest", OptionalMemorial, Rule::gregorian(5, 20);
    "St Christopher Magallanes, Priest, and Companions, Martyrs", OptionalMemorial, Rule::gregorian(5, 21);
    "St Rita of Cascia, Religious", OptionalMemorial, Rule::gregorian(5, 22);
    "St Bede the Venerable, Priest and Doctor of the Church", OptionalMemorial, Rule::gregorian(5, 25);
    "St Gregory VII, Pope", OptionalMemorial, Rule::gregorian(5, 25);
    "St Mary Magdalene de’ Pazzi, Virgin", OptionalMemorial, Rule::gregorian(5, 25);
    "St Philip Neri, Priest", Memorial, Rule::gregorian(5, 26);
    "St Augustine of Canterbury, Bishop", OptionalMemorial, Rule::gregorian(5, 27);
    // Decree of 25 January 2019, Prot. N. 29/19.
    "St Paul VI, Pope", OptionalMemorial, Rule::gregorian(5, 29), since 2019;
    "The Visitation of the Blessed Virgin Mary", Feast, Rule::gregorian(5, 31);
    // Decree of 11 February 2018, Prot. N. 10/18: the Monday after
    // Pentecost.
    "The Blessed Virgin Mary, Mother of the Church", Memorial, Rule::easter(50), since 2018;
    "The Most Holy Trinity", Solemnity, Rule::easter(56);
    "The Most Holy Body and Blood of Christ", Solemnity, Rule::easter(60);
    "The Most Sacred Heart of Jesus", Solemnity, Rule::easter(68);
    "The Immaculate Heart of the Blessed Virgin Mary", Memorial, Rule::easter(69);
    // June
    "St Justin, Martyr", Memorial, Rule::gregorian(6, 1);
    "Sts Marcellinus and Peter, Martyrs", OptionalMemorial, Rule::gregorian(6, 2);
    "St Charles Lwanga and Companions, Martyrs", Memorial, Rule::gregorian(6, 3);
    "St Boniface, Bishop and Martyr", Memorial, Rule::gregorian(6, 5);
    "St Norbert, Bishop", OptionalMemorial, Rule::gregorian(6, 6);
    "St Ephrem, Deacon and Doctor of the Church", OptionalMemorial, Rule::gregorian(6, 9);
    "St Barnabas, Apostle", Memorial, Rule::gregorian(6, 11);
    "St Anthony of Padua, Priest and Doctor of the Church", Memorial, Rule::gregorian(6, 13);
    "St Romuald, Abbot", OptionalMemorial, Rule::gregorian(6, 19);
    "St Aloysius Gonzaga, Religious", Memorial, Rule::gregorian(6, 21);
    "St Paulinus of Nola, Bishop", OptionalMemorial, Rule::gregorian(6, 22);
    "Sts John Fisher, Bishop, and Thomas More, Martyrs", OptionalMemorial, Rule::gregorian(6, 22);
    "The Nativity of St John the Baptist", Solemnity, Rule::gregorian(6, 24);
    "St Cyril of Alexandria, Bishop and Doctor of the Church", OptionalMemorial, Rule::gregorian(6, 27);
    "St Irenaeus, Bishop and Martyr", Memorial, Rule::gregorian(6, 28);
    "Sts Peter and Paul, Apostles", Solemnity, Rule::gregorian(6, 29);
    "The First Martyrs of the Holy Roman Church", OptionalMemorial, Rule::gregorian(6, 30);
    // July
    "St Thomas, Apostle", Feast, Rule::gregorian(7, 3);
    "St Elizabeth of Portugal", OptionalMemorial, Rule::gregorian(7, 4);
    "St Anthony Zaccaria, Priest", OptionalMemorial, Rule::gregorian(7, 5);
    "St Maria Goretti, Virgin and Martyr", OptionalMemorial, Rule::gregorian(7, 6);
    "St Augustine Zhao Rong, Priest, and Companions, Martyrs", OptionalMemorial, Rule::gregorian(7, 9);
    "St Benedict, Abbot", Memorial, Rule::gregorian(7, 11);
    "St Henry", OptionalMemorial, Rule::gregorian(7, 13);
    "St Camillus de Lellis, Priest", OptionalMemorial, Rule::gregorian(7, 14);
    "St Bonaventure, Bishop and Doctor of the Church", Memorial, Rule::gregorian(7, 15);
    "Our Lady of Mount Carmel", OptionalMemorial, Rule::gregorian(7, 16);
    "St Apollinaris, Bishop and Martyr", OptionalMemorial, Rule::gregorian(7, 20);
    "St Lawrence of Brindisi, Priest and Doctor of the Church", OptionalMemorial, Rule::gregorian(7, 21);
    "St Mary Magdalene", Memorial, Rule::gregorian(7, 22), until 2015;
    // Decree of 3 June 2016, Prot. N. 257/16: "the rank of Feast rather
    // than Memorial".
    "St Mary Magdalene", Feast, Rule::gregorian(7, 22), since 2016;
    "St Bridget, Religious", OptionalMemorial, Rule::gregorian(7, 23);
    "St Sharbel Makhlūf, Priest", OptionalMemorial, Rule::gregorian(7, 24);
    "St James, Apostle", Feast, Rule::gregorian(7, 25);
    "Sts Joachim and Anne, Parents of the Blessed Virgin Mary", Memorial, Rule::gregorian(7, 26);
    "St Martha", Memorial, Rule::gregorian(7, 29), until 2020;
    // Decree of 26 January 2021, Prot. N. 35/21.
    "Sts Martha, Mary and Lazarus", Memorial, Rule::gregorian(7, 29), since 2021;
    "St Peter Chrysologus, Bishop and Doctor of the Church", OptionalMemorial, Rule::gregorian(7, 30);
    "St Ignatius of Loyola, Priest", Memorial, Rule::gregorian(7, 31);
    // August
    "St Alphonsus Mary Liguori, Bishop and Doctor of the Church", Memorial, Rule::gregorian(8, 1);
    "St Eusebius of Vercelli, Bishop", OptionalMemorial, Rule::gregorian(8, 2);
    "St Peter Julian Eymard, Priest", OptionalMemorial, Rule::gregorian(8, 2);
    "St John Mary Vianney, Priest", Memorial, Rule::gregorian(8, 4);
    "The Dedication of the Basilica of St Mary Major", OptionalMemorial, Rule::gregorian(8, 5);
    "The Transfiguration of the Lord", Feast, Rule::gregorian(8, 6);
    "St Sixtus II, Pope, and Companions, Martyrs", OptionalMemorial, Rule::gregorian(8, 7);
    "St Cajetan, Priest", OptionalMemorial, Rule::gregorian(8, 7);
    "St Dominic, Priest", Memorial, Rule::gregorian(8, 8);
    "St Teresa Benedicta of the Cross, Virgin and Martyr", OptionalMemorial, Rule::gregorian(8, 9);
    "St Lawrence, Deacon and Martyr", Feast, Rule::gregorian(8, 10);
    "St Clare, Virgin", Memorial, Rule::gregorian(8, 11);
    "St Jane Frances de Chantal, Religious", OptionalMemorial, Rule::gregorian(8, 12);
    "Sts Pontian, Pope, and Hippolytus, Priest, Martyrs", OptionalMemorial, Rule::gregorian(8, 13);
    "St Maximilian Mary Kolbe, Priest and Martyr", Memorial, Rule::gregorian(8, 14);
    "The Assumption of the Blessed Virgin Mary", Solemnity, Rule::gregorian(8, 15);
    "St Stephen of Hungary", OptionalMemorial, Rule::gregorian(8, 16);
    "St John Eudes, Priest", OptionalMemorial, Rule::gregorian(8, 19);
    "St Bernard, Abbot and Doctor of the Church", Memorial, Rule::gregorian(8, 20);
    "St Pius X, Pope", Memorial, Rule::gregorian(8, 21);
    "The Queenship of the Blessed Virgin Mary", Memorial, Rule::gregorian(8, 22);
    "St Rose of Lima, Virgin", OptionalMemorial, Rule::gregorian(8, 23);
    "St Bartholomew, Apostle", Feast, Rule::gregorian(8, 24);
    "St Louis", OptionalMemorial, Rule::gregorian(8, 25);
    "St Joseph Calasanz, Priest", OptionalMemorial, Rule::gregorian(8, 25);
    "St Monica", Memorial, Rule::gregorian(8, 27);
    "St Augustine, Bishop and Doctor of the Church", Memorial, Rule::gregorian(8, 28);
    "The Passion of St John the Baptist", Memorial, Rule::gregorian(8, 29);
    // September
    "St Gregory the Great, Pope and Doctor of the Church", Memorial, Rule::gregorian(9, 3);
    // Decree of 24 December 2024, Prot. N. 703/24.
    "St Teresa of Calcutta, Virgin", OptionalMemorial, Rule::gregorian(9, 5), since 2025;
    "The Nativity of the Blessed Virgin Mary", Feast, Rule::gregorian(9, 8);
    "St Peter Claver, Priest", OptionalMemorial, Rule::gregorian(9, 9);
    "The Most Holy Name of Mary", OptionalMemorial, Rule::gregorian(9, 12);
    "St John Chrysostom, Bishop and Doctor of the Church", Memorial, Rule::gregorian(9, 13);
    "The Exaltation of the Holy Cross", Feast, Rule::gregorian(9, 14);
    "Our Lady of Sorrows", Memorial, Rule::gregorian(9, 15);
    "Sts Cornelius, Pope, and Cyprian, Bishop, Martyrs", Memorial, Rule::gregorian(9, 16);
    "St Robert Bellarmine, Bishop and Doctor of the Church", OptionalMemorial, Rule::gregorian(9, 17);
    // Decree of 25 January 2021, Prot. N. 40/21.
    "St Hildegard of Bingen, Virgin and Doctor of the Church", OptionalMemorial, Rule::gregorian(9, 17), since 2021;
    "St Januarius, Bishop and Martyr", OptionalMemorial, Rule::gregorian(9, 19);
    "Sts Andrew Kim Tae-gŏn, Priest, Paul Chŏng Ha-sang, and Companions, Martyrs", Memorial, Rule::gregorian(9, 20);
    "St Matthew, Apostle and Evangelist", Feast, Rule::gregorian(9, 21);
    "St Pius of Pietrelcina, Priest", Memorial, Rule::gregorian(9, 23);
    "Sts Cosmas and Damian, Martyrs", OptionalMemorial, Rule::gregorian(9, 26);
    "St Vincent de Paul, Priest", Memorial, Rule::gregorian(9, 27);
    "St Wenceslaus, Martyr", OptionalMemorial, Rule::gregorian(9, 28);
    "St Lawrence Ruiz and Companions, Martyrs", OptionalMemorial, Rule::gregorian(9, 28);
    "Sts Michael, Gabriel and Raphael, Archangels", Feast, Rule::gregorian(9, 29);
    "St Jerome, Priest and Doctor of the Church", Memorial, Rule::gregorian(9, 30);
    // October
    "St Thérèse of the Child Jesus, Virgin and Doctor of the Church", Memorial, Rule::gregorian(10, 1);
    "The Holy Guardian Angels", Memorial, Rule::gregorian(10, 2);
    "St Francis of Assisi", Memorial, Rule::gregorian(10, 4);
    // Decree of 18 May 2020, Prot. N. 229/20.
    "St Maria Faustina Kowalska, Virgin", OptionalMemorial, Rule::gregorian(10, 5), since 2020;
    "St Bruno, Priest", OptionalMemorial, Rule::gregorian(10, 6);
    "Our Lady of the Rosary", Memorial, Rule::gregorian(10, 7);
    "St Denis, Bishop, and Companions, Martyrs", OptionalMemorial, Rule::gregorian(10, 9);
    "St John Leonardi, Priest", OptionalMemorial, Rule::gregorian(10, 9);
    // Decree of 9 November 2025, Prot. N. 760/25.
    "St John Henry Newman, Priest and Doctor of the Church", OptionalMemorial, Rule::gregorian(10, 9), since 2026;
    // Decree of 29 May 2014, Prot. N. 309/14.
    "St John XXIII, Pope", OptionalMemorial, Rule::gregorian(10, 11), since 2014;
    "St Callistus I, Pope and Martyr", OptionalMemorial, Rule::gregorian(10, 14);
    "St Teresa of Jesus, Virgin and Doctor of the Church", Memorial, Rule::gregorian(10, 15);
    "St Hedwig, Religious", OptionalMemorial, Rule::gregorian(10, 16);
    "St Margaret Mary Alacoque, Virgin", OptionalMemorial, Rule::gregorian(10, 16);
    "St Ignatius of Antioch, Bishop and Martyr", Memorial, Rule::gregorian(10, 17);
    "St Luke, Evangelist", Feast, Rule::gregorian(10, 18);
    "Sts John de Brébeuf and Isaac Jogues, Priests, and Companions, Martyrs", OptionalMemorial, Rule::gregorian(10, 19);
    "St Paul of the Cross, Priest", OptionalMemorial, Rule::gregorian(10, 19);
    // Decree of 29 May 2014, Prot. N. 309/14.
    "St John Paul II, Pope", OptionalMemorial, Rule::gregorian(10, 22), since 2014;
    "St John of Capestrano, Priest", OptionalMemorial, Rule::gregorian(10, 23);
    "St Anthony Mary Claret, Bishop", OptionalMemorial, Rule::gregorian(10, 24);
    "Sts Simon and Jude, Apostles", Feast, Rule::gregorian(10, 28);
    // November
    "All Saints", Solemnity, Rule::gregorian(11, 1);
    "The Commemoration of All the Faithful Departed (All Souls’ Day)", Commemoration, Rule::gregorian(11, 2);
    "St Martin de Porres, Religious", OptionalMemorial, Rule::gregorian(11, 3);
    "St Charles Borromeo, Bishop", Memorial, Rule::gregorian(11, 4);
    "The Dedication of the Lateran Basilica", Feast, Rule::gregorian(11, 9);
    "St Leo the Great, Pope and Doctor of the Church", Memorial, Rule::gregorian(11, 10);
    "St Martin of Tours, Bishop", Memorial, Rule::gregorian(11, 11);
    "St Josaphat, Bishop and Martyr", Memorial, Rule::gregorian(11, 12);
    "St Albert the Great, Bishop and Doctor of the Church", OptionalMemorial, Rule::gregorian(11, 15);
    "St Margaret of Scotland", OptionalMemorial, Rule::gregorian(11, 16);
    "St Gertrude, Virgin", OptionalMemorial, Rule::gregorian(11, 16);
    "St Elizabeth of Hungary, Religious", Memorial, Rule::gregorian(11, 17);
    "The Dedication of the Basilicas of Sts Peter and Paul, Apostles", OptionalMemorial, Rule::gregorian(11, 18);
    "The Presentation of the Blessed Virgin Mary", Memorial, Rule::gregorian(11, 21);
    "St Cecilia, Virgin and Martyr", Memorial, Rule::gregorian(11, 22);
    "St Clement I, Pope and Martyr", OptionalMemorial, Rule::gregorian(11, 23);
    "St Columban, Abbot", OptionalMemorial, Rule::gregorian(11, 23);
    "Our Lord Jesus Christ, King of the Universe", Solemnity, CHRIST_THE_KING;
    "St Andrew Dũng-Lạc, Priest, and Companions, Martyrs", Memorial, Rule::gregorian(11, 24);
    "St Catherine of Alexandria, Virgin and Martyr", OptionalMemorial, Rule::gregorian(11, 25);
    "St Andrew, Apostle", Feast, Rule::gregorian(11, 30);
    // December
    "St Francis Xavier, Priest", Memorial, Rule::gregorian(12, 3);
    "St John Damascene, Priest and Doctor of the Church", OptionalMemorial, Rule::gregorian(12, 4);
    "St Nicholas, Bishop", OptionalMemorial, Rule::gregorian(12, 6);
    "St Ambrose, Bishop and Doctor of the Church", Memorial, Rule::gregorian(12, 7);
    "The Immaculate Conception of the Blessed Virgin Mary", Solemnity, Rule::gregorian(12, 8);
    "St Juan Diego Cuauhtlatoatzin", OptionalMemorial, Rule::gregorian(12, 9);
    // Decree of 7 October 2019, Prot. N. 404/19.
    "The Blessed Virgin Mary of Loreto", OptionalMemorial, Rule::gregorian(12, 10), since 2019;
    "St Damasus I, Pope", OptionalMemorial, Rule::gregorian(12, 11);
    "Our Lady of Guadalupe", OptionalMemorial, Rule::gregorian(12, 12);
    "St Lucy, Virgin and Martyr", Memorial, Rule::gregorian(12, 13);
    "St John of the Cross, Priest and Doctor of the Church", Memorial, Rule::gregorian(12, 14);
    "St Peter Canisius, Priest and Doctor of the Church", OptionalMemorial, Rule::gregorian(12, 21);
    "St John of Kanty, Priest", OptionalMemorial, Rule::gregorian(12, 23);
    "The Nativity of the Lord (Christmas)", Solemnity, Rule::gregorian(12, 25);
    "St Stephen, the First Martyr", Feast, Rule::gregorian(12, 26);
    "St John, Apostle and Evangelist", Feast, Rule::gregorian(12, 27);
    "The Holy Innocents, Martyrs", Feast, Rule::gregorian(12, 28);
    "St Thomas Becket, Bishop and Martyr", OptionalMemorial, Rule::gregorian(12, 29);
    "The Holy Family of Jesus, Mary and Joseph", Feast, Rule::Computed(holy_family);
    "St Sylvester I, Pope", OptionalMemorial, Rule::gregorian(12, 31);
}

/// The General Roman Calendar as a rule set: every celebration a
/// [`Kind::Religious`] observance, in the years it applies, with no
/// precedence applied.
pub static GENERAL_ROMAN_CALENDAR: RuleSet = RuleSet {
    code: "roman-general",
    english_name: "General Roman Calendar",
    rules: RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "General Roman Calendar and Universal Norms on the Liturgical Year and the General \
              Roman Calendar (approved by Mysterii Paschalis, 14 February 1969), as the Liturgy \
              Office of the Bishops' Conference of England and Wales publishes them \
              (liturgyoffice.org.uk/Calendar/Info/, secondary; the Missale Romanum, editio \
              typica tertia, 2002, not read), retrieved 2026-09-23; decrees of the Congregation \
              (Dicastery) for Divine Worship and the Discipline of the Sacraments of 29 May 2014 \
              (Prot. N. 309/14), 3 June 2016 (257/16), 11 February 2018 (10/18), 25 January 2019 \
              (29/19), 7 October 2019 (404/19), 18 May 2020 (229/20), 25 January 2021 (40/21), \
              26 January 2021 (35/21), 24 December 2024 (703/24) and 9 November 2025 (760/25), \
              from vatican.va and cultodivino.va, retrieved 2026-09-23 and 2026-09-26",
};

/// The celebrations the calendar lists on a day, in its order: every one
/// that applies in the day's year and falls on it, before precedence.
#[must_use]
pub fn celebrations_on(day: Rd) -> Vec<&'static Celebration> {
    let Ok((year, _, _)) = gregorian::from_fixed(day) else {
        return Vec::new();
    };
    CELEBRATIONS
        .iter()
        .filter(|celebration| {
            celebration.applies_in(year)
                && celebration
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

    #[test]
    fn the_ranks_are_counted_as_the_module_states() {
        let count = |rank: Rank| {
            CELEBRATIONS
                .iter()
                .filter(|celebration| celebration.rank == rank && celebration.until.is_none())
                .count()
        };
        assert_eq!(count(Rank::Solemnity), 14);
        assert_eq!(count(Rank::Commemoration), 1);
        assert_eq!(count(Rank::Feast), 26);
        assert_eq!(count(Rank::Memorial), 69);
        assert_eq!(count(Rank::OptionalMemorial), 120);
    }

    #[test]
    fn the_movable_celebrations_fall_where_the_2026_ordo_puts_them() {
        // The Liturgy Office's summary of 2026: the Baptism of the Lord on
        // 11 January, Mary, Mother of the Church on 25 May, the Most Holy
        // Trinity on 31 May, the Sacred Heart on 12 June, Christ the King on
        // 22 November and the Holy Family on 27 December — and on 28
        // December in 2025.
        for (year, month, day, title) in [
            (2026, 1, 11, "The Baptism of the Lord"),
            (2026, 5, 25, "The Blessed Virgin Mary, Mother of the Church"),
            (2026, 5, 31, "The Most Holy Trinity"),
            (2026, 6, 12, "The Most Sacred Heart of Jesus"),
            (2026, 11, 22, "Our Lord Jesus Christ, King of the Universe"),
            (2026, 12, 27, "The Holy Family of Jesus, Mary and Joseph"),
            (2025, 12, 28, "The Holy Family of Jesus, Mary and Joseph"),
        ] {
            assert!(
                titles_on(year, month, day).contains(&title),
                "{year}-{month}-{day}"
            );
        }
        // The general calendar keeps the Body and Blood of Christ on the
        // Thursday; England and Wales move it to Sunday 7 June.
        assert!(titles_on(2026, 6, 4).contains(&"The Most Holy Body and Blood of Christ"));
        assert!(
            titles_on(2026, 6, 13).contains(&"The Immaculate Heart of the Blessed Virgin Mary")
        );
        // Christmas on a Sunday leaves the octave no Sunday: 30 December.
        assert!(titles_on(2022, 12, 30).contains(&"The Holy Family of Jesus, Mary and Joseph"));
    }

    #[test]
    fn a_decree_applies_from_its_year() {
        assert_eq!(titles_on(2013, 10, 22), Vec::<&str>::new());
        assert_eq!(titles_on(2014, 10, 22), ["St John Paul II, Pope"]);
        assert_eq!(titles_on(2020, 7, 29), ["St Martha"]);
        assert_eq!(titles_on(2021, 7, 29), ["Sts Martha, Mary and Lazarus"]);
        assert_eq!(
            titles_on(2025, 10, 9),
            [
                "St Denis, Bishop, and Companions, Martyrs",
                "St John Leonardi, Priest"
            ]
        );
        assert_eq!(titles_on(2026, 10, 9).len(), 3);
        let magdalene = |year| celebrations_on(ymd(year, 7, 22))[0].rank;
        assert_eq!(magdalene(2015), Rank::Memorial);
        assert_eq!(magdalene(2016), Rank::Feast);
        // Whit Monday: 5 June 2017, before the decree, and 21 May 2018.
        let mother = "The Blessed Virgin Mary, Mother of the Church";
        assert!(!titles_on(2017, 6, 5).contains(&mother));
        assert!(titles_on(2018, 5, 21).contains(&mother));
        assert_eq!(
            titles_on(2020, 12, 10),
            ["The Blessed Virgin Mary of Loreto"]
        );
    }

    #[test]
    fn every_title_is_distinct_but_the_one_a_decree_re_ranked() {
        for (index, celebration) in CELEBRATIONS.iter().enumerate() {
            for other in &CELEBRATIONS[index + 1..] {
                if celebration.title == other.title {
                    assert_eq!(celebration.title, "St Mary Magdalene");
                }
            }
        }
    }

    #[test]
    fn the_rule_set_carries_every_celebration_as_a_religious_day() {
        let calendar =
            crate::engine::HolidayCalendar::for_year(&GENERAL_ROMAN_CALENDAR, None, 2026);
        let christmas = calendar.on(ymd(2026, 12, 25));
        assert_eq!(christmas.len(), 1);
        assert_eq!(christmas[0].kind, Kind::Religious);
        assert!(
            !calendar.is_holiday(ymd(2026, 12, 25)),
            "a tradition gives no day off"
        );
    }
}
