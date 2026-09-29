//! The Church of England's *Common Worship* calendar: its Principal
//! Feasts, Principal Holy Days and Festivals, each with its rank, on the
//! day it is kept once the transfers its Rules require have been made.
//!
//! "Rules to Order the Christian Year" (`cw-rules`) ranks the
//! celebrations and says when one gives way to another. This module
//! carries the three ranks the Rules list by name —
//!
//! | Rank | Count | Examples |
//! | --- | --- | --- |
//! | Principal Feast | 9 | Christmas Day, Easter Day, All Saints' Day |
//! | Principal Holy Day | 3 | Ash Wednesday, Maundy Thursday, Good Friday |
//! | Festival | 28 | the apostles and evangelists, Holy Cross Day, Christ the King |
//!
//! — and the transfers the Rules make mandatory. The system is written up
//! in `docs/systems/common-worship-calendar.md`: in short, the Annunciation
//! moves off a Sunday to the Monday, and out of the fortnight from Palm
//! Sunday to the Second Sunday of Easter to the Monday after it; St Joseph,
//! St George and St Mark leave that fortnight for the same Monday, St Mark
//! for the Tuesday when St George is moved too; a Festival on a Sunday of
//! Advent, Lent or Eastertide moves to the Monday; and a Festival on a
//! Principal Feast or Principal Holy Day moves to "the first available
//! day", which is always the next day for the Festivals the calendar can
//! bring into that position.
//!
//! What the Rules permit and do not require is not applied: the Epiphany
//! on the Sunday between 2 and 8 January, the Presentation on the Sunday
//! between 28 January and 3 February, All Saints on the Sunday between
//! 30 October and 5 November, the Blessed Virgin Mary on 8 September, a
//! Festival moved off an ordinary Sunday, and a Festival moved for Corpus
//! Christi, which a church may keep as a Festival or not. The Rules move
//! St George and St Mark to fixed days after Easter without saying what
//! happens when another Festival is already there, and forbid Easter Week
//! to saints' days without naming a day for one that falls in it. When
//! Easter is 17 April St George's Monday is St Mark's Day; the Rules' note
//! on the two, that "George will have been transferred to the first
//! available free day", and the Church's Daily Prayer for Tuesday 26 April
//! 2022, which keeps "George, Martyr, Patron of England" that day, put St
//! George on the Tuesday and leave St Mark on the Monday. The other years
//! are reported as gaps ([`Rule::Unsettled`]): an Easter from 22 to 25
//! April, when St George, St Mark or Philip and James is left without a
//! day — St Mark and Philip and James on 1 May when Easter is 22 April,
//! St George and Philip and James when it is 23 April, and Philip and
//! James alone when it is 24 April, the Second Sunday of Easter with St
//! George on the Monday after, or 25 April, Easter Week.
//!
//! Not carried: the Lesser Festivals and Commemorations of the calendar,
//! which the minister may keep or not; a church's Patronal and Dedication
//! Festivals; and the Book of Common Prayer's own calendar. The Rules are
//! applied to any year; when they were authorized was not read.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;

use crate::computus::gregorian_easter;
use crate::computus::offsets::{
    ASCENSION, ASH_WEDNESDAY, EASTER_SUNDAY, GOOD_FRIDAY, MAUNDY_THURSDAY, PENTECOST,
    TRINITY_SUNDAY,
};
use crate::rule::{
    Days, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate, Subdivisions,
};

/// The rank of a celebration, as the Rules list them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rank {
    /// A Principal Feast, which no other celebration may displace.
    PrincipalFeast,
    /// A Principal Holy Day: Ash Wednesday, Maundy Thursday, Good Friday.
    PrincipalHolyDay,
    /// A Festival, "not usually displaced".
    Festival,
}

impl Rank {
    /// The rank's English name.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::PrincipalFeast => "Principal Feast",
            Self::PrincipalHolyDay => "Principal Holy Day",
            Self::Festival => "Festival",
        }
    }
}

/// A celebration of the calendar.
#[derive(Debug, Clone, Copy)]
pub struct Celebration {
    /// Its title, as the Rules print it.
    pub title: &'static str,
    /// Its rank.
    pub rank: Rank,
    /// The day it is kept, after the transfers the Rules require.
    pub rule: Rule,
}

/// A Gregorian date, or `None` outside the calendar's range.
fn date(year: i64, month: u8, day: u8) -> Option<Rd> {
    gregorian::to_fixed(year, month, day).ok()
}

/// Whether a day is a Sunday.
fn is_sunday(day: Rd) -> bool {
    Weekday::from_rd(day) == Weekday::Sunday
}

/// Whether `day` lies between Palm Sunday and the Second Sunday of Easter
/// inclusive, the fortnight the Annunciation, St Joseph, St George and
/// St Mark leave.
const fn in_paschal_fortnight(day: Rd, easter: Rd) -> bool {
    day.0 >= easter.0 - 7 && day.0 <= easter.0 + 7
}

/// The Monday after the Second Sunday of Easter, where the four go.
const fn monday_after_easter_two(easter: Rd) -> Rd {
    Rd(easter.0 + 8)
}

/// Easter of `year` and the date `month`/`day` in it.
fn easter_and(year: i64, month: u8, day: u8) -> Option<(Rd, Rd)> {
    Some((gregorian_easter(year)?, date(year, month, day)?))
}

/// The Annunciation, on 25 March: to the Monday after the Second Sunday of
/// Easter from the paschal fortnight, and to the Monday from a Sunday.
fn annunciation_day(year: i64) -> Option<Rd> {
    let (easter, day) = easter_and(year, 3, 25)?;
    Some(if in_paschal_fortnight(day, easter) {
        monday_after_easter_two(easter)
    } else if is_sunday(day) {
        Rd(day.0 + 1)
    } else {
        day
    })
}

fn annunciation(year: i64) -> Days {
    annunciation_day(year).map_or_else(Days::new, Days::one)
}

/// St Joseph, on 19 March: from the paschal fortnight to the Monday after
/// the Second Sunday of Easter, or the next day if the Annunciation is
/// there; from a Sunday, always one of Lent, to the Monday.
fn joseph(year: i64) -> Days {
    let Some((easter, day)) = easter_and(year, 3, 19) else {
        return Days::new();
    };
    Days::one(if in_paschal_fortnight(day, easter) {
        let monday = monday_after_easter_two(easter);
        if annunciation_day(year) == Some(monday) {
            Rd(monday.0 + 1)
        } else {
            monday
        }
    } else if is_sunday(day) {
        Rd(day.0 + 1)
    } else {
        day
    })
}

/// Whether `day` is 25 April or 1 May, the days of St Mark and of Philip
/// and James, onto which the Rules can move St George or St Mark without
/// saying which of the two Festivals is kept there.
fn holds_another_festival(year: i64, day: Rd) -> bool {
    date(year, 4, 25) == Some(day) || date(year, 5, 1) == Some(day)
}

/// St George, on 23 April: from the paschal fortnight to the Monday after
/// the Second Sunday of Easter; from a Sunday, one of Eastertide, to the
/// Monday. When that Monday is St Mark's Day, as when Easter is 17 April,
/// to the Tuesday, "the first available free day", as the Church kept it
/// on 26 April 2022. Unsettled when the Monday is the day of Philip and
/// James, which happens when Easter is 23 April.
fn george(year: i64) -> Option<Days> {
    let Some((easter, day)) = easter_and(year, 4, 23) else {
        return Some(Days::new());
    };
    if in_paschal_fortnight(day, easter) {
        let monday = monday_after_easter_two(easter);
        if date(year, 4, 25) == Some(monday) {
            return Some(Days::one(Rd(monday.0 + 1)));
        }
        return (!holds_another_festival(year, monday)).then(|| Days::one(monday));
    }
    Some(Days::one(if is_sunday(day) { Rd(day.0 + 1) } else { day }))
}

/// St Mark, on 25 April: from the paschal fortnight to the Monday after
/// the Second Sunday of Easter, or the Tuesday when St George is moved
/// there too; from a Sunday, one of Eastertide, to the Monday. It keeps
/// its day when St George's Monday is 25 April, St George going on to the
/// Tuesday. Unsettled when St Mark's Tuesday is 1 May, as when Easter is
/// 22 April.
fn mark(year: i64) -> Option<Days> {
    let Some((easter, day)) = easter_and(year, 4, 25) else {
        return Some(Days::new());
    };
    if in_paschal_fortnight(day, easter) {
        let monday = monday_after_easter_two(easter);
        let george_moves = in_paschal_fortnight(Rd(day.0 - 2), easter);
        let moved = Rd(monday.0 + i64::from(george_moves));
        return (!holds_another_festival(year, moved)).then(|| Days::one(moved));
    }
    Some(Days::one(if is_sunday(day) { Rd(day.0 + 1) } else { day }))
}

/// A Festival of Eastertide or just after it, which moves to the next day
/// when it falls on Ascension Day, on a Sunday of Eastertide — Pentecost
/// among them — or on Trinity Sunday. The next day is free in every case
/// the calendar produces.
fn after_easter(year: i64, month: u8, day: u8) -> Option<Rd> {
    let (easter, day) = easter_and(year, month, day)?;
    let offset = day.0 - easter.0;
    let displaced = offset == i64::from(ASCENSION)
        || offset == i64::from(TRINITY_SUNDAY)
        || (is_sunday(day) && (0..=i64::from(PENTECOST)).contains(&offset));
    Some(if displaced { Rd(day.0 + 1) } else { day })
}

/// Philip and James, on 1 May, as [`after_easter`]. Unsettled when Easter
/// is 22 to 25 April: 1 May is then the Tuesday St Mark is moved to (22),
/// the Monday St George is moved to (23), the Second Sunday of Easter
/// with St George moved to the Monday after (24), or a day of Easter Week
/// (25).
fn philip_and_james(year: i64) -> Option<Days> {
    let Some((easter, day)) = easter_and(year, 5, 1) else {
        return Some(Days::new());
    };
    let offset = day.0 - easter.0;
    if (6..=9).contains(&offset) {
        return None;
    }
    Some(after_easter(year, 5, 1).map_or_else(Days::new, Days::one))
}

fn matthias(year: i64) -> Days {
    after_easter(year, 5, 14).map_or_else(Days::new, Days::one)
}

fn visitation(year: i64) -> Days {
    after_easter(year, 5, 31).map_or_else(Days::new, Days::one)
}

fn barnabas(year: i64) -> Days {
    after_easter(year, 6, 11).map_or_else(Days::new, Days::one)
}

/// St Andrew, on 30 November: to the Monday when it is the First Sunday of
/// Advent.
fn andrew(year: i64) -> Days {
    date(year, 11, 30).map_or_else(Days::new, |day| {
        Days::one(if is_sunday(day) { Rd(day.0 + 1) } else { day })
    })
}

/// The Baptism of Christ: the First Sunday of Epiphany, or the Second,
/// 13 January, when 6 January is a Sunday — the Sunday from 7 to 13 January.
const BAPTISM_OF_CHRIST: Rule = Rule::WeekdayOnOrAfter {
    month: 1,
    day: 7,
    weekday: Weekday::Sunday,
};

/// Christ the King: the Sunday next before Advent, from 20 to 26 November.
const CHRIST_THE_KING: Rule = Rule::WeekdayOnOrAfter {
    month: 11,
    day: 20,
    weekday: Weekday::Sunday,
};

/// The calendar, written once and read twice: as [`CELEBRATIONS`], with
/// the ranks, and as the rule set the engine evaluates.
macro_rules! common_worship_calendar {
    ($($title:literal, $rank:ident, $rule:expr);* $(;)?) => {
        /// Every Principal Feast, Principal Holy Day and Festival, in the
        /// Rules' order.
        pub static CELEBRATIONS: &[Celebration] = &[$(
            Celebration { title: $title, rank: Rank::$rank, rule: $rule }
        ),*];

        static RULES: &[HolidayRule] = &[$(
            HolidayRule::observance($title, "", $rule).of_kind(Kind::Religious)
        ),*];
    };
}

common_worship_calendar! {
    "Christmas Day", PrincipalFeast, Rule::gregorian(12, 25);
    "The Epiphany", PrincipalFeast, Rule::gregorian(1, 6);
    "The Presentation of Christ in the Temple", PrincipalFeast, Rule::gregorian(2, 2);
    "The Annunciation of Our Lord to the Blessed Virgin Mary", PrincipalFeast,
        Rule::Computed(annunciation);
    "Easter Day", PrincipalFeast, Rule::easter(EASTER_SUNDAY);
    "Ascension Day", PrincipalFeast, Rule::easter(ASCENSION);
    "Pentecost (Whit Sunday)", PrincipalFeast, Rule::easter(PENTECOST);
    "Trinity Sunday", PrincipalFeast, Rule::easter(TRINITY_SUNDAY);
    "All Saints' Day", PrincipalFeast, Rule::gregorian(11, 1);
    "Ash Wednesday", PrincipalHolyDay, Rule::easter(ASH_WEDNESDAY);
    "Maundy Thursday", PrincipalHolyDay, Rule::easter(MAUNDY_THURSDAY);
    "Good Friday", PrincipalHolyDay, Rule::easter(GOOD_FRIDAY);
    "The Naming and Circumcision of Jesus", Festival, Rule::gregorian(1, 1);
    "The Baptism of Christ", Festival, BAPTISM_OF_CHRIST;
    "The Conversion of Paul", Festival, Rule::gregorian(1, 25);
    "Joseph of Nazareth", Festival, Rule::Computed(joseph);
    "George, Martyr, Patron of England", Festival, Rule::Unsettled(george);
    "Mark the Evangelist", Festival, Rule::Unsettled(mark);
    "Philip and James, Apostles", Festival, Rule::Unsettled(philip_and_james);
    "Matthias the Apostle", Festival, Rule::Computed(matthias);
    "The Visit of the Blessed Virgin Mary to Elizabeth", Festival, Rule::Computed(visitation);
    "Barnabas the Apostle", Festival, Rule::Computed(barnabas);
    "The Birth of John the Baptist", Festival, Rule::gregorian(6, 24);
    "Peter and Paul, Apostles", Festival, Rule::gregorian(6, 29);
    "Thomas the Apostle", Festival, Rule::gregorian(7, 3);
    "Mary Magdalene", Festival, Rule::gregorian(7, 22);
    "James the Apostle", Festival, Rule::gregorian(7, 25);
    "The Transfiguration of Our Lord", Festival, Rule::gregorian(8, 6);
    "The Blessed Virgin Mary", Festival, Rule::gregorian(8, 15);
    "Bartholomew the Apostle", Festival, Rule::gregorian(8, 24);
    "Holy Cross Day", Festival, Rule::gregorian(9, 14);
    "Matthew, Apostle and Evangelist", Festival, Rule::gregorian(9, 21);
    "Michael and All Angels", Festival, Rule::gregorian(9, 29);
    "Luke the Evangelist", Festival, Rule::gregorian(10, 18);
    "Simon and Jude, Apostles", Festival, Rule::gregorian(10, 28);
    "Christ the King", Festival, CHRIST_THE_KING;
    "Andrew the Apostle", Festival, Rule::Computed(andrew);
    "Stephen, Deacon, First Martyr", Festival, Rule::gregorian(12, 26);
    "John, Apostle and Evangelist", Festival, Rule::gregorian(12, 27);
    "The Holy Innocents", Festival, Rule::gregorian(12, 28);
}

/// The *Common Worship* calendar of the Church of England: its Principal
/// Feasts, Principal Holy Days and Festivals, each on the day it is kept
/// after the transfers the Rules require. See the module documentation for
/// what is applied and what is not.
pub static COMMON_WORSHIP: RuleSet = RuleSet {
    code: "common-worship",
    english_name: "Church of England (Common Worship calendar)",
    rules: RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: "Church of England, Common Worship, \"Rules to Order the Christian Year\" \
              and \"A Table of Transferences\" (churchofengland.org, `cw-rules`), \
              retrieved 2026-09-27, for the ranks, the celebrations of each and the \
              transfers; the Church of England's Morning Prayer pages of Daily Prayer \
              (`cofe-daily-prayer-2026`), for the Visit of the Blessed Virgin Mary on \
              Monday 1 June 2026 after Trinity Sunday and Barnabas, Holy Cross Day, \
              Matthew and Michael and All Angels on their days in 2026, as checks, \
              retrieved 2026-09-27; Charlotte Green, \"When is St George's Day?\", \
              Full Fact, 23 April 2025 (`fullfact-st-george-2025`), for St George's Day \
              on Monday 28 April 2025, as a check",
    subdivisions: Subdivisions::Undivided,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_ranks_are_counted_as_the_module_states() {
        let count = |rank: Rank| {
            CELEBRATIONS
                .iter()
                .filter(|celebration| celebration.rank == rank)
                .count()
        };
        assert_eq!(count(Rank::PrincipalFeast), 9);
        assert_eq!(count(Rank::PrincipalHolyDay), 3);
        assert_eq!(count(Rank::Festival), 28);
        assert_eq!(RULES.len(), CELEBRATIONS.len());
    }

    #[test]
    fn every_title_is_distinct() {
        for (index, celebration) in CELEBRATIONS.iter().enumerate() {
            for other in &CELEBRATIONS[index + 1..] {
                assert_ne!(celebration.title, other.title);
            }
        }
    }
}
