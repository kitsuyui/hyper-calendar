//! The calendar of the Book of Common Prayer of 1662: its Table of the
//! Feasts and the saints' days of its Kalendar.
//!
//! `docs/systems/common-worship-calendar.md` in the repository describes
//! it beside the calendar of *Common Worship* and states the sources; this
//! page states the code's own facts.
//!
//! The Prayer Book's "Table of all the Feasts that are to be observed in
//! the Church of England through the year" lists "All Sundaies in the
//! year" and twenty-nine other days: twenty-four fixed feasts, the
//! Ascension, and the Monday and Tuesday in Easter week and in Whitsun
//! week. Its Kalendar prints those days in red, and other saints' days, the
//! "black-letter days", beside them. [`CELEBRATIONS`] carries both, each
//! with its [`Letter`]; the rule set `bcp-1662` carries them as religious
//! days. The moveable feasts follow the Prayer Book's rules: Easter Day
//! "the first Sunday after the Full Moon, which happens upon, or next after
//! the Twenty-first Day of March", by the Gregorian computus the Calendar
//! (New Style) Act 1750 gave it from 1752, Ascension Day forty days after,
//! Whitsunday seven weeks and Trinity Sunday eight, and Advent Sunday "the
//! nearest Sunday to the Feast of S. Andrew, whether before or after".
//!
//! The English names are those of the Kalendar as Lynda Howell transcribes
//! the modern Prayer Book's, and the local names the 1662 book's own, as
//! she transcribes the 1892 facsimile of the Annexed Book (secondary; the
//! Church of England's own Kalendar is a PDF, not read). The red-letter
//! days are named as the Table of Feasts names them. Where the 1662 book's
//! Kalendar has a day the modern one does not — King Charles the Martyr
//! (30 January), Charles II's Nativity and Return (29 May), the Papists'
//! Conspiracy (5 November) and St Blasius (3 February) — its own wording
//! is both names. St Mary Magdalen, on 21 July in the 1662 transcription
//! and 22 July in the modern one, is a gap: the two disagree.
//!
//! The Prayer Book gives no rule for a holy day that falls on a Sunday; the
//! "Rules to Order the Service" that a later Prayer Book prints are not its
//! 1662 text and are not carried, and nor is a leap-year day for St
//! Matthias, which the Kalendar keeps on 24 February. The vigils, fasts and
//! Ember Days are the `ember-bcp1662` table's and the Kalendar's lessons are
//! not carried. The table begins in 1753, the first year wholly on the
//! Gregorian calendar; the Julian years from 1662 are not carried.

use hc_calendar::Weekday;
use hc_calendars_solar::gregorian;

use crate::computus::offsets::{ASCENSION, EASTER_SUNDAY, PENTECOST, TRINITY_SUNDAY};
use crate::rule::{Days, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate};

/// How the Kalendar prints a day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Letter {
    /// A red-letter day: a feast of the Table of Feasts, with its collect,
    /// epistle and gospel.
    Red,
    /// A black-letter day: a saint the Kalendar names, with no service of
    /// its own.
    Black,
}

/// A day of the Prayer Book's calendar.
#[derive(Debug, Clone, Copy)]
pub struct Celebration {
    /// Its name, as the modern Kalendar prints it or the Table of Feasts.
    pub title: &'static str,
    /// Its name in the 1662 book's Kalendar.
    pub title_1662: &'static str,
    /// Red letter or black.
    pub letter: Letter,
    /// The day it falls on.
    pub rule: Rule,
}

/// St Mary Magdalen, whose day the two transcriptions put on 21 and
/// 22 July: a gap every year.
const fn mary_magdalen(_: i64) -> Option<Days> {
    None
}

/// Advent Sunday: the Sunday nearest St Andrew, 27 November to 3 December.
fn advent_sunday(year: i64) -> Days {
    gregorian::to_fixed(year, 11, 27).map_or_else(
        |_| Days::new(),
        |day| Days::one(Weekday::Sunday.on_or_after(day)),
    )
}

/// The calendar, written once and read twice.
macro_rules! prayer_book {
    ($(($title:expr, $old:expr, $letter:ident, $rule:expr));* $(;)?) => {
        /// Every day of the calendar: the red-letter days, the moveable
        /// feasts, then the black-letter days in the Kalendar's order.
        pub static CELEBRATIONS: &[Celebration] = &[$(
            Celebration { title: $title, title_1662: $old, letter: Letter::$letter, rule: $rule }
        ),*];

        static RULES: &[HolidayRule] = &[$(
            HolidayRule::observance($title, $old, $rule)
                .of_kind(Kind::Religious)
                .years(Some(1753), None)
        ),*];
    };
}

prayer_book! {
    ("The Circumcision of our Lord Jesus Christ", "Circumc. of our Ld.", Red, Rule::gregorian(1, 1));
    ("The Epiphany", "Epiphanie of our Ld.", Red, Rule::gregorian(1, 6));
    ("The Conversion of S. Paul", "Convers: of S. Paul", Red, Rule::gregorian(1, 25));
    ("The Purification of the blessed Virgin", "Purif. of Marie ye. B. Virg.", Red, Rule::gregorian(2, 2));
    ("Saint Matthias the Apostle", "Matthias Apost. & M.", Red, Rule::gregorian(2, 24));
    ("The Annunciation of the Blessed Virgin", "Annunc. of Mary", Red, Rule::gregorian(3, 25));
    ("S. Mark the Evangelist", "S. Mark Evang. & Mart.", Red, Rule::gregorian(4, 25));
    ("S. Philip, and S. Jacob the Apostles", "S. Phil & S. Jac Ap. & M.", Red, Rule::gregorian(5, 1));
    ("S. Barnabas", "S. Barnab, Apost. & M.", Red, Rule::gregorian(6, 11));
    ("The Nativity of S. John Baptist", "Nativ. of S. John Bapt:", Red, Rule::gregorian(6, 24));
    ("S. Peter the Apostle", "S. Pet. Apost. & M.", Red, Rule::gregorian(6, 29));
    ("S. James the Apostle", "S. Iames Apost. & M.", Red, Rule::gregorian(7, 25));
    ("S. Bartholmew the Apostle", "S. Barth. Apost. & M.", Red, Rule::gregorian(8, 24));
    ("S. Matthew the Apostle", "S. Mat. Ap. Evang. & M.", Red, Rule::gregorian(9, 21));
    ("S. Michael and all Angels", "S. Mich. & all Angels", Red, Rule::gregorian(9, 29));
    ("S. Luke the Evangelist", "S. Luke Evang.", Red, Rule::gregorian(10, 18));
    ("S. Simon, and S. Jude the Apostles", "S. Sim. & S. Jud. Ap. & M.", Red, Rule::gregorian(10, 28));
    ("All Saints", "All Saints day.", Red, Rule::gregorian(11, 1));
    ("S. Andrew the Apostle", "S. Andr. Apost. & M.", Red, Rule::gregorian(11, 30));
    ("S. Thomas the Apostle", "S Thomas Apost. & M.", Red, Rule::gregorian(12, 21));
    ("The Nativity of our Lord", "Christmas day.", Red, Rule::gregorian(12, 25));
    ("S. Stephen the Martyr", "S. Steph. ye. first Mart.", Red, Rule::gregorian(12, 26));
    ("S. John the Evangelist", "S. John Apost. & Evang.", Red, Rule::gregorian(12, 27));
    ("The holy Innocents", "Innocents day.", Red, Rule::gregorian(12, 28));
    ("Easter Day", "Easter-Day", Red, Rule::easter(EASTER_SUNDAY));
    ("Monday in Easter week", "Monday in Easter week", Red, Rule::easter(1));
    ("Tuesday in Easter week", "Tuesday in Easter week", Red, Rule::easter(2));
    ("The Ascension of our Lord Jesus Christ", "Ascension-Day", Red, Rule::easter(ASCENSION));
    ("Whitsunday", "Whitsunday", Red, Rule::easter(PENTECOST));
    ("Monday in Whitsun week", "Monday in Whitsun week", Red, Rule::easter(PENTECOST + 1));
    ("Tuesday in Whitsun week", "Tuesday in Whitsun week", Red, Rule::easter(PENTECOST + 2));
    ("Trinity Sunday", "Trinity-Sunday", Red, Rule::easter(TRINITY_SUNDAY));
    ("Advent Sunday", "Advent-Sunday", Red, Rule::Computed(advent_sunday));
    ("Lucian, P. & M.", "Lucian Pr. & Mart.", Black, Rule::gregorian(1, 8));
    ("Hilary, Bp. & C.", "Hilarie B. &, Conf.", Black, Rule::gregorian(1, 13));
    ("Prisca, V. & M.", "Prisca Rom. Virg. & Mart.", Black, Rule::gregorian(1, 18));
    ("Fabian, Bp. & M.", "Fabian B. of Rome & Mart.", Black, Rule::gregorian(1, 20));
    ("Agnes, V. & M.", "Agnes Rom. Virg. & Mart.", Black, Rule::gregorian(1, 21));
    ("Vincent, Mart.", "Vincent Span. Deac. & M.", Black, Rule::gregorian(1, 22));
    ("K. Charles Mart.", "K. Charles Mart.", Black, Rule::gregorian(1, 30));
    ("Blasius an Armen. B. & M.", "Blasius an Armen. B. & M.", Black, Rule::gregorian(2, 3));
    ("Agatha, V. & M.", "Agatha, a Sicilian V. & M.", Black, Rule::gregorian(2, 5));
    ("Valentine, Bishop", "Valentine Bish. & M.", Black, Rule::gregorian(2, 14));
    ("David, Archbp.", "David Arch-B. of Menevia", Black, Rule::gregorian(3, 1));
    ("Chad, Bishop", "Cedde, or Chad B. of Litchf.", Black, Rule::gregorian(3, 2));
    ("Perpetua, M.", "Perpetua Mauritan. M.", Black, Rule::gregorian(3, 7));
    ("Gregory, M. B.", "Gregorio, M. B. of Rome, & C.", Black, Rule::gregorian(3, 12));
    ("Edward, King of the West-Sax.", "Edward K. of ye. West Saxons", Black, Rule::gregorian(3, 18));
    ("Benedict, Abbot.", "Benedict, Abbot.", Black, Rule::gregorian(3, 21));
    ("Richard, Bp.", "Richard B. of Chichester", Black, Rule::gregorian(4, 3));
    ("S. Ambrose, Bp.", "Ambrose B. of Millan.", Black, Rule::gregorian(4, 4));
    ("Alphege, Abp.", "Alphege Arch-B. of Cant.", Black, Rule::gregorian(4, 19));
    ("St. George, M.", "S. George M.", Black, Rule::gregorian(4, 23));
    ("Invent. of Cross", "Invention of the Cross.", Black, Rule::gregorian(5, 3));
    ("St. John, E. ante Port. Lat.", "S. John Evang. ante port. latin.", Black, Rule::gregorian(5, 6));
    ("Dunstan, Archbp.", "Dunstan Arch.B. of Cant.", Black, Rule::gregorian(5, 19));
    ("Augustine, Archbp.", "Aug: ye. first Arch B. of Cant.", Black, Rule::gregorian(5, 26));
    ("Ven. Bede, Presb.", "Ven. Bede pr.", Black, Rule::gregorian(5, 27));
    ("CH. II. Nat. et Ret.", "CH. II. Nat. et Ret.", Black, Rule::gregorian(5, 29));
    ("Nicomede, M.", "Nicomede Rom. Pr. & M.", Black, Rule::gregorian(6, 1));
    ("Boniface, Bishop.", "Boniface B. of Mentz & M.", Black, Rule::gregorian(6, 5));
    ("St. Alban, Mart.", "S. Alban M.", Black, Rule::gregorian(6, 17));
    ("Tr. of King Edw.", "Transl: of Edwd. K. of ye. W. Sax.", Black, Rule::gregorian(6, 20));
    ("Visitation of the Blessed Virgin Mary", "Visit: of ye. Bl. Virg. Marie.", Black, Rule::gregorian(7, 2));
    ("Tr. of St. Martin", "Transl: of S. Martin B. & C.", Black, Rule::gregorian(7, 4));
    ("Swithun, Bishop", "Swithun B. of Winch. Transl.", Black, Rule::gregorian(7, 15));
    ("Margaret V. & M.", "Margaret V. & M. at Antioch.", Black, Rule::gregorian(7, 20));
    ("St. Anne", "S. Anne, mother to ye. Bl. V. M.", Black, Rule::gregorian(7, 26));
    ("Lammas Day", "Lammas Day", Black, Rule::gregorian(8, 1));
    ("Transfiguration", "Transfigur. of our Lord.", Black, Rule::gregorian(8, 6));
    ("Name of Jesus", "Name of Jesus", Black, Rule::gregorian(8, 7));
    ("St. Lawrence, M.", "S. Laur. Arch D. of Rome & M.", Black, Rule::gregorian(8, 10));
    ("St. Augustine, B.", "S. Aug. B. of Hippo. C. D.", Black, Rule::gregorian(8, 28));
    ("Beheading of St. John Baptist", "Behead: of S. John Bapt.", Black, Rule::gregorian(8, 29));
    ("Giles, Abbot.", "Giles Abbot & Conf.", Black, Rule::gregorian(9, 1));
    ("Enurchus, Bishop.", "Enurchus B. of Orleans", Black, Rule::gregorian(9, 7));
    ("Nat. of Vir. Mary.", "Nativ. of ye. Bl. Virg. Mary", Black, Rule::gregorian(9, 8));
    ("Holy-Cross Day", "Holy crosse day", Black, Rule::gregorian(9, 14));
    ("Lambert, Bishop", "Lambert B. & M.", Black, Rule::gregorian(9, 17));
    ("St. Cyprian, Abp.", "S. Cypr: Ar. B. of Carth & M.", Black, Rule::gregorian(9, 26));
    ("St. Jerome", "S. Hierome Pr. Conf. & Doct.", Black, Rule::gregorian(9, 30));
    ("Remigius, Bp.", "Remigius B. of Rhemes", Black, Rule::gregorian(10, 1));
    ("Faith, V. & M.", "Faith Virg. & M.", Black, Rule::gregorian(10, 6));
    ("St. Denys, Bishop", "S. Denys Areop. B. & M.", Black, Rule::gregorian(10, 9));
    ("Trans. K. Edw.", "Transl. of K. Edward Conf.", Black, Rule::gregorian(10, 13));
    ("Etheldreda, V.", "Ethelrede Virg.", Black, Rule::gregorian(10, 17));
    ("Crispin, Martyr", "Crispine Mart.", Black, Rule::gregorian(10, 25));
    ("Papists Conspiracy", "Papists Conspiracy", Black, Rule::gregorian(11, 5));
    ("Leonard, Conf.", "Leonard Confess.", Black, Rule::gregorian(11, 6));
    ("St. Martin, Bp.", "S. Martin B. & Conf:", Black, Rule::gregorian(11, 11));
    ("Britius, Bishop", "Britius Bishop", Black, Rule::gregorian(11, 13));
    ("Machutus, Bp.", "Machutus B.", Black, Rule::gregorian(11, 15));
    ("Hugh, Bishop", "Hugh B. of Lincoln.", Black, Rule::gregorian(11, 17));
    ("Edmund, King", "Edmund K. & M.", Black, Rule::gregorian(11, 20));
    ("Cecilia, V. & M.", "Cecilia Virg. & M.", Black, Rule::gregorian(11, 22));
    ("St. Clement, Bp.", "S.Clemt. 1. B of R. & M.", Black, Rule::gregorian(11, 23));
    ("Catherine, V. & M.", "Catherine Virg. & M.", Black, Rule::gregorian(11, 25));
    ("Nicolas, Bishop", "Nicholas B. of Myra in Lycia", Black, Rule::gregorian(12, 6));
    ("Conception of Vir. Mary", "Concep: of ye. Bl. V. Mary.", Black, Rule::gregorian(12, 8));
    ("Lucy, Vir. & M.", "Lucie Virg. & M.", Black, Rule::gregorian(12, 13));
    ("O Sapientia", "O Sapientia", Black, Rule::gregorian(12, 16));
    ("Silvester, Bishop", "Silvester B. of Rome.", Black, Rule::gregorian(12, 31));
    ("St. Mary Magdalen", "S. Marie Magdalen.", Black, Rule::Unsettled(mary_magdalen));
}

/// The calendar of the Book of Common Prayer of 1662 as a rule set: every
/// red-letter and black-letter day a [`Kind::Religious`] observance, from
/// 1753.
pub static BOOK_OF_COMMON_PRAYER_1662: RuleSet = RuleSet {
    code: "bcp-1662",
    english_name: "Book of Common Prayer (1662)",
    rules: RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "The Book of Common Prayer (1662): \"A Table of all the Feasts that are to be \
              observed in the Church of England through the year\" and the Tables and Rules for \
              the Moveable Feasts, in Wikisource's transcription of the 1892 facsimile of the \
              Annexed Book (en.wikisource.org, Book of Common Prayer (1892), pp. 39-44, \
              wikisource-bcp-1892), secondary; the Kalendar of the 1662 book and of the modern \
              Prayer Book in Lynda Howell's transcriptions \
              (eskimo.com/~lhowell/bcp1662/info/cal_1662/ and cal_1871/, howell-bcp-kalendar), \
              secondary; the Calendar (New Style) Act 1750's tables for the Gregorian computus \
              (calendar-new-style-act-1750); all retrieved 2026-09-29. The Church of England's \
              Kalendar and Tables, PDFs, were not read",
};

#[cfg(test)]
mod tests {
    use super::*;

    fn titles_on(year: i64, month: u8, day: u8) -> alloc::vec::Vec<&'static str> {
        let day = gregorian::to_fixed(year, month, day).unwrap();
        CELEBRATIONS
            .iter()
            .filter(|c| c.rule.days_in_year(year).as_slice().contains(&day))
            .map(|c| c.title)
            .collect()
    }

    /// The Table of Feasts: "All Sundaies in the year" and twenty-nine
    /// other days, twenty-four of them fixed.
    #[test]
    fn the_table_of_feasts_is_carried_in_red() {
        let red: alloc::vec::Vec<_> = CELEBRATIONS
            .iter()
            .filter(|c| c.letter == Letter::Red)
            .collect();
        let weekdays = red
            .iter()
            .filter(|c| {
                !matches!(
                    c.title,
                    "Easter Day" | "Whitsunday" | "Trinity Sunday" | "Advent Sunday"
                )
            })
            .count();
        assert_eq!(weekdays, 29);
        assert_eq!(
            red.iter()
                .filter(|c| matches!(c.rule, Rule::FixedGregorian { .. }))
                .count(),
            24
        );
        // Seventy black-letter days, with the four only the 1662 Kalendar
        // has and St Mary Magdalen, a gap.
        assert_eq!(
            CELEBRATIONS
                .iter()
                .filter(|c| c.letter == Letter::Black)
                .count(),
            70
        );
    }

    /// 2026: Easter 5 April, Ascension Day 14 May, Whitsunday 24 May,
    /// Trinity Sunday 31 May; St Andrew on Monday 30 November, the nearest
    /// Sunday Advent Sunday, 29 November.
    #[test]
    fn the_moveable_feasts_of_2026() {
        assert_eq!(titles_on(2026, 4, 5), ["Easter Day"]);
        assert_eq!(titles_on(2026, 4, 7), ["Tuesday in Easter week"]);
        assert_eq!(
            titles_on(2026, 5, 14),
            ["The Ascension of our Lord Jesus Christ"]
        );
        assert_eq!(titles_on(2026, 5, 25), ["Monday in Whitsun week"]);
        assert_eq!(titles_on(2026, 5, 31), ["Trinity Sunday"]);
        assert_eq!(titles_on(2026, 11, 29), ["Advent Sunday"]);
        assert_eq!(titles_on(2026, 11, 30), ["S. Andrew the Apostle"]);
        assert_eq!(titles_on(2026, 1, 13), ["Hilary, Bp. & C."]);
        assert_eq!(titles_on(2026, 12, 16), ["O Sapientia"]);
        // St Matthias stays on 24 February in a leap year.
        assert_eq!(titles_on(2028, 2, 24), ["Saint Matthias the Apostle"]);
        let calendar =
            crate::engine::HolidayCalendar::for_year(&BOOK_OF_COMMON_PRAYER_1662, None, 2026);
        assert!(
            calendar
                .gaps()
                .iter()
                .any(|gap| gap.name == "St. Mary Magdalen")
        );
        assert!(
            crate::engine::HolidayCalendar::for_year(&BOOK_OF_COMMON_PRAYER_1662, None, 1752)
                .on(gregorian::to_fixed(1752, 12, 25).unwrap())
                .is_empty()
        );
    }
}
