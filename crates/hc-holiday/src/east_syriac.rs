//! The East Syriac churches' years beyond the Assyrian Church of the
//! East's seasons: its Fridays of commemoration and saints' days, and the
//! Chaldean Catholic and Syro-Malabar Catholic years as tables of their
//! own.
//!
//! `docs/systems/church-of-the-east-year.md` in the repository describes
//! the year and its sister churches' forms and states the sources; this
//! page states the code's own facts.
//!
//! - [`CHURCH_OF_THE_EAST_COMMEMORATIONS`], the days the table
//!   `church-of-the-east` adds to its seasons: the Fridays of the Epiphany
//!   period whose place Rev. Tower Andrious's account of the year fixes —
//!   the first after the Epiphany, St John the Baptist's, the penultimate
//!   before the Great Fast, the local patron's, and the last, the faithful
//!   departed's — with the Friday of Lazarus, the Friday of Gold, the last
//!   Friday of the Apostles, and the commemoration of Mary on the Friday
//!   after Christmas; and the saints' days whose date in the Church's
//!   calendar of 2026–2029 is the same day of the year, or the same number
//!   of days from Easter, in all four years.
//! - [`CHALDEAN`] (`chaldean`), the Chaldean Catholic Church's year as its
//!   Diocese of St Thomas the Apostle states it: the ACE's seasons with the
//!   Feast of the Cross on 14 September and the first Sunday of the Cross
//!   the Sunday on or after it.
//! - [`SYRO_MALABAR`] (`syro-malabar`), the Syro-Malabar Church's, whose
//!   season of Elijah, the Cross and Moses begins on the fourteenth Sunday
//!   after Pentecost, with the Cross on 14 September.
//!
//! The Ancient Church of the East, which kept the Julian calendar in 1964,
//! is not carried: no calendar of its own was read.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;

use crate::computus::offsets::{ASCENSION, EASTER_SUNDAY, PALM_SUNDAY, PENTECOST};
use crate::rule::{
    Days, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate, Subdivisions, joined,
};
use crate::traditions::CHURCH_OF_THE_EAST_RULES;

/// A day of the Church of the East, from 1965, as `church-of-the-east`'s.
const fn east(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::observance(name, local, rule)
        .of_kind(Kind::Religious)
        .years(Some(1965), None)
}

/// A saint's day on a fixed date. The Church's calendar of 2026–2029 moves
/// the commemoration of Mary of 15 August and the Fast of the Virgin of
/// 1 August to the Monday when they fall on a Sunday (2027); whether it
/// moves the others, none of its four years shows, so a Sunday is a gap.
macro_rules! fixed_days {
    ($($name:ident = ($month:literal, $day:literal);)*) => {
        $(fn $name(year: i64) -> Option<Days> {
            let day = gregorian::to_fixed(year, $month, $day).ok()?;
            (Weekday::from_rd(day) != Weekday::Sunday).then(|| Days::one(day))
        })*
    };
}

fixed_days! {
    april_24 = (4, 24);
    may_1 = (5, 1);
    june_1 = (6, 1);
    june_9 = (6, 9);
    july_3 = (7, 3);
    august_7 = (8, 7);
    september_1 = (9, 1);
    september_8 = (9, 8);
    september_14 = (9, 14);
    october_27 = (10, 27);
    november_3 = (11, 3);
    may_15 = (5, 15);
}

const fn fixed_day(
    name: &'static str,
    local: &'static str,
    rule: fn(i64) -> Option<Days>,
) -> HolidayRule {
    east(name, local, Rule::Unsettled(rule))
}

/// The moves that take a Sunday to the Monday.
const SUNDAY_TO_MONDAY: &[(Weekday, i16)] = &[(Weekday::Sunday, 1)];
const AUGUST_1: Rule = Rule::gregorian(8, 1);
const AUGUST_15: Rule = Rule::gregorian(8, 15);

/// The Friday after the Epiphany: the first Friday of the Epiphany period.
fn first_friday_after_epiphany(year: i64) -> Days {
    gregorian::to_fixed(year, 1, 6).map_or_else(
        |_| Days::new(),
        |epiphany| Days::one(Weekday::Friday.after(epiphany)),
    )
}

/// The commemoration of Mary after a Christmas: "the second Friday after
/// Christmas, or the first Friday if one Friday coincides with Nativity
/// and the day of Epiphany" (`andrious-liturgical-year`). Read as the
/// Church's calendar of 2026–2029 keeps it: the second Friday after
/// Christmas when that is before the Epiphany, else the first.
fn mary_after_christmas(christmas_year: i64) -> Option<Rd> {
    let christmas = gregorian::to_fixed(christmas_year, 12, 25).ok()?;
    let epiphany = gregorian::to_fixed(christmas_year + 1, 1, 6).ok()?;
    let first = Weekday::Friday.after(christmas);
    let second = Rd(first.0 + 7);
    Some(if second < epiphany { second } else { first })
}

/// The commemorations of Mary that fall in a Gregorian year: after the
/// Christmas before, or, as on 31 December 2027, after its own.
fn friday_of_mary(year: i64) -> Days {
    let mut days = Days::new();
    for christmas_year in [year - 1, year] {
        if let Some(day) = mary_after_christmas(christmas_year)
            && gregorian::year_from_fixed(day).ok() == Some(year)
        {
            days.push(day);
        }
    }
    days
}

/// The Fridays of the Epiphany period, the Fridays of the Great Fast and of
/// the seasons after Easter that Andrious's account fixes, and the saints'
/// days of the Church's calendar of 2026–2029 that keep one date or one
/// distance from Easter in all four years; `church-of-the-east` carries
/// them after its seasons.
pub static CHURCH_OF_THE_EAST_COMMEMORATIONS: &[HolidayRule] = &[
    east(
        "Commemoration of the Virgin Mary (after Christmas)",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܬܝ ܡܲܪܝܲܡ",
        Rule::Computed(friday_of_mary),
    ),
    east(
        "Commemoration of St. John the Baptist",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܝܘܿܚܲܢܵܢ ܡܲܥܡܕܵܢܵܐ",
        Rule::Computed(first_friday_after_epiphany),
    ),
    east(
        "Commemoration of the patron of the local church (St. Awa Catholicos)",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܐܲܒ݂ܵܐ ܩܵܬ̣ܘܿܠܝ݂ܩܵܐ",
        Rule::easter(-58),
    ),
    east("Friday of the Deceased", "ܥܪܘܼܒܼܬܵܐ ܕܥܲܢܝ݂̈ܕܹܐ", Rule::easter(-51)),
    east(
        "Commemoration of Mar Benyamin Shimun XXI Catholicos Patriarch the Martyr",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܒܸܢܝܵܡܹܝܢ ܫܸܡܥܘܿܢ ܩܵܬܼܘܿܠܝ݂ܩܵܐ ܣܵܗܕܵܐ",
        Rule::easter(-55),
    ),
    east(
        "Middle Week of the Mysteries",
        "ܦܲܠܓܘܼܬ̣ܵܐ ܕܨܵܘܡܵܐ ܪܲܒܵܐ",
        Rule::easter(-25),
    ),
    east("Friday of Lazarus", "ܥܪܘܼܒ݂ܬܵܐ ܕܠܵܥܵܙܲܪ", Rule::easter(-9)),
    east("Thursday of Passover", "ܥܹܐܕܵܐ ܕܦܸܨܚܵܐ", Rule::easter(-3)),
    east("Good Friday", "ܥܪܘܼܒ݂ܬܵܐ ܕܚܲܫܵܐ", Rule::easter(-2)),
    east("Great Saturday", "ܫܲܒܬ̣ܵܐ ܕܢܘܼܗܪܵܐ", Rule::easter(-1)),
    east("Monday of the Thief", "ܬܪܹܝܢܒܫܲܒܵܐ ܕܓܲܝܵܣܵܐ", Rule::easter(1)),
    east("Friday of Confessors", "ܥܪܘܼܒ݂ܬܵܐ ܕܡܵܘܕܝܵܢܹ̈ܐ", Rule::easter(5)),
    east(
        "New Sunday and Commemoration of St. Moses",
        "ܚܲܕܒܫܲܒܵܐ ܚܲܕܬܼܵܐ ܘܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܡܘܼܫܹܐ",
        Rule::easter(7),
    ),
    east(
        "Commemoration of St. Abraham Mede",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܐܲܒܼܪܵܗܵܡ ܡܵܕܵܝܵܐ",
        Rule::easter(8),
    ),
    east(
        "Commemoration of St. Qayoma",
        "ܕܘܼܟ̣ܪܵܢܵܐ ܕܡܵܪܝ ܩܲܝܘܼܡܵܐ",
        Rule::easter(8),
    ),
    east(
        "Commemoration of St. Narsai the Teacher",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܢܲܪܣܲܝ ܡܲܠܦܵܢܵܐ",
        Rule::easter(11),
    ),
    east(
        "Commemoration of St. Pinkhis",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܦܝ݂ܢܚܸܣ",
        Rule::easter(12),
    ),
    east(
        "Commemoration of St. Abdisho, St. Jonah the Ascetic and St. Khanania",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܥܲܒ݂ܕܝ݂ܫܘܿܥ ܘܲܕܡܵܪܝ ܝܵܘܢܵܢ ܢܘܼܟ݂ܪܹܝܛܵܐ ܘܲܕܡܵܪܝ ܚܲܢܲܢܝܵܐ",
        Rule::easter(15),
    ),
    east(
        "Commemoration of St. Hurmizd",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܪܲܒܲܢ ܗܘܿܪܡܝ݂ܙܕ",
        Rule::easter(15),
    ),
    east(
        "Commemoration of St Elijah III Catholicos (Abu Khalim)",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܐܹܠܝܼܵܐ ܬܠܝܼܬܵܝܵܐ (ܐܲܒܘܼ ܚܲܠܸܡ)",
        Rule::easter(19),
    ),
    east(
        "Commemoration of St. Awraham of Kashkar",
        "ܕܘܼܟ̣ܪܵܢܵܐ ܕܡܵܪܝ ܐܲܒ̣ܪܵܗܵܡ ܕܟܲܫܟܲܪ",
        Rule::easter(22),
    ),
    east(
        "Commemoration of St. Issac of Nineveh",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܐܝܼܣܚܵܩ ܕܢܝܼܢܘܹܐ",
        Rule::easter(26),
    ),
    east(
        "Commemoration of St. Addai the Apostle",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܐܲܕܲܝ ܫܠܝ݂ܚܵܐ",
        Rule::easter(28),
    ),
    east(
        "Commemoration of King Abgar Awkama",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܲܠܟܵܐ ܐܲܒ̣ܓܲܪ ܐܘܿܟܵܡܵܐ",
        Rule::easter(29),
    ),
    east(
        "Commemoration of St. Andrew the Apostle",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܐܲܢܕܪܹܐܘܿܣ ܫܠܝܼܚܵܐ",
        Rule::easter(36),
    ),
    east("Sunday after Ascension", "ܙ ܕܩܝܡܬܐ", Rule::easter(42)),
    east("Friday of Gold", "ܥܪܘܼܒ݂ܬܵܐ ܕܕܲܗܒ݂ܵܐ", Rule::easter(54)),
    east(
        "Commemoration of St. Meelis and St. Bar Qusry",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܡܝ݂ܠܸܣ ܘܲܕܡܵܪܝ ܒܲܪ ܩܘܼܣܖܹ̈ܐ",
        Rule::easter(71),
    ),
    east(
        "Commemoration of St. Fibronia the martyr",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܬܝ ܦ̮ܸܒܪܘܿܢܝܼܵܐ ܣܵܗܕܬܵܐ",
        Rule::easter(75),
    ),
    east(
        "Commemoration of St. Awa Srapion",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܐܲܒ̣ܵܐ ܣܪܵܦܝܼܘܿܢ",
        Rule::easter(78),
    ),
    east(
        "Commemoration of St. Rabban Pethyon",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܪܲܒܵܢ ܦܸܬ̣ܝܘܿܢ",
        Rule::easter(89),
    ),
    east(
        "Commemoration of St. Ezekiel of Daqoq",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܚܲܙܩܝ݂ܐܹܝܠ ܕܕܵܩܘܿܩ",
        Rule::easter(92),
    ),
    east(
        "Commemoration of the Seventy-Two Apostles",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܫܲܒ݂ܥܝ݂ܢ ܘܲܬ̣ܪܹܝܢ ܫܠܝ݂ܚܹ̈ܐ",
        Rule::easter(96),
    ),
    east(
        "Commemoration of St. Jacob of Nisibis",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܝܲܥܩܘܿܒ݂ ܕܲܢܨܝ݂ܒ݂ܝ݂ܢ",
        Rule::easter(103),
    ),
    east(
        "Commemoration of St. Mari the Apostle",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܡܵܐܪܝ݂ ܫܠܝ݂ܚܵܐ",
        Rule::easter(110),
    ),
    east(
        "Commemoration of St. Shimon Bar Sabbae",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܫܸܡܥܘܿܢ ܒܲܪ ܨܲܒܵܥܹ̈ܐ",
        Rule::easter(138),
    ),
    fixed_day(
        "Commemoration of St. George the martyr",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܓܝ݂ܘܲܪܓܝ݂ܣ",
        april_24,
    ),
    fixed_day(
        "Commemoration of St. Awimalk Timothy",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܐܲܒܼܝܼܡܲܠܟ ܛܝܼܡܵܬܹܐܘܿܣ",
        may_1,
    ),
    fixed_day(
        "Commemoration of all Catholicos Patriarchs of Seleucia-Ctesiphon",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܟܠܲܝܗܝ ܩܵܬܼܘܿܠܝܼܩܹ̈ܐ ܦܵܛܲܪܝܵܖ̈ܟܹܐ ܕܣܵܠܝܼܩ ܘܩܛܝܼܣܦ̮ܘܿܢ",
        june_1,
    ),
    fixed_day(
        "Commemoration of St. Ephraim the Teacher",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܐܲܦܪܹܝܡ ܡܲܠܦܵܢܵܐ",
        june_9,
    ),
    fixed_day(
        "Commemoration of St. Thomas the Apostle",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܬܐܘܿܡܵܐ ܫܠܝ݂ܚܵܐ",
        july_3,
    ),
    fixed_day("Assyrian Martyrs Day", "ܝܵܘܡܵܐ ܕܣܵܗܕܵܐ ܐܵܬܘܿܪܵܝܵܐ", august_7),
    fixed_day(
        "Commemoration of St. Hurmizd & St. Tawor",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܪܲܒܲܢ ܗܘܿܪܡܝ݂ܙܕ ܘܕܡܪܝ ܬܐܘܪ",
        september_1,
    ),
    fixed_day(
        "Virgin Mary's Birthday",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܘܠܵܕܵܐ ܕܡܵܪܬܝ ܡܲܪܝܲܡ ܒܬܘܼܠܬܵܐ",
        september_8,
    ),
    fixed_day(
        "Commemoration of St. Sawa the Doctor, St. Abdisho, St. Zaia, St. Bisho and all Saints",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܣܵܒ݂ܵܐ ܐܵܣܝܵܐ ܘܲܕܡܵܪܝ ܥܲܒ݂ܕܝ݂ܫܘܿܥ ܘܲܕܡܵܪܝ ܒܹܗܝ݂ܫܘܿܥ ܘܲܕܡܵܪܝ ܙܲܝܥܵܐ ܘܲܕܟ݂ܠܗܘܿܢ ܩܲܕܝ݂ܫܹ̈ܐ",
        september_14,
    ),
    fixed_day(
        "Commemoration of St Rabban Yareth",
        "ܕܘܼܟ̣ܪܵܢܵܐ ܕܪܲܒܵܢ ܡܵܪܝ ܝܵܐܪܹܬ̣",
        october_27,
    ),
    fixed_day("Commemoration of St. Mikha", "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܝ ܡܝܼܟ̣ܵܐ", november_3),
    east(
        "Commemoration of the Virgin Mary",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܬܝ ܡܲܪܝܲܡ",
        Rule::Unsettled(may_15),
    ),
    east(
        "The Fast of the Virgin Mary",
        "ܨܵܘܡܵܐ ܕܡܵܪܬܝ ܡܲܪܝܲܡ",
        Rule::moved_by_weekday(&AUGUST_1, SUNDAY_TO_MONDAY),
    ),
    east(
        "Commemoration of the Virgin Mary (Assumption)",
        "ܕܘܼܟ݂ܪܵܢܵܐ ܕܡܵܪܬܝ ܡܲܪܝܲܡ",
        Rule::moved_by_weekday(&AUGUST_15, SUNDAY_TO_MONDAY),
    ),
];

const CHURCH_OF_THE_EAST_LEN: usize =
    CHURCH_OF_THE_EAST_RULES.len() + CHURCH_OF_THE_EAST_COMMEMORATIONS.len();

/// Every rule of `church-of-the-east`: its seasons, then the
/// commemorations here.
pub(crate) static CHURCH_OF_THE_EAST_ALL_RULES: [HolidayRule; CHURCH_OF_THE_EAST_LEN] =
    joined(&[CHURCH_OF_THE_EAST_RULES, CHURCH_OF_THE_EAST_COMMEMORATIONS]);

/// The first Sunday of Elijah when the Feast of the Cross is on `cross`
/// September: the Sunday after seven weeks of Summer, or a week earlier
/// when that is not before the Cross.
fn first_sunday_of_elijah_before(year: i64, cross: u8) -> Days {
    let (Some(easter), Ok(cross)) = (
        crate::computus::gregorian_easter(year),
        gregorian::to_fixed(year, 9, cross),
    ) else {
        return Days::new();
    };
    let ideal = Rd(easter.0 + 147);
    Days::one(if ideal < cross {
        ideal
    } else {
        Rd(ideal.0 - 7)
    })
}

fn chaldean_first_sunday_of_elijah(year: i64) -> Days {
    first_sunday_of_elijah_before(year, 14)
}

/// A day of the Chaldean or the Syro-Malabar year.
const fn day(name: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::observance(name, "", rule).of_kind(Kind::Religious)
}

const fn first_sunday_on_or_after(month: u8, day: u8) -> Rule {
    Rule::WeekdayOnOrAfter {
        month,
        day,
        weekday: Weekday::Sunday,
    }
}

static CHALDEAN_RULES: &[HolidayRule] = &[
    day("Rogation of the Ninevites (Ba'utha)", Rule::easter(-69)),
    day("Rogation of the Ninevites (Ba'utha)", Rule::easter(-68)),
    day("Rogation of the Ninevites (Ba'utha)", Rule::easter(-67)),
    day(
        "First Sunday of the Great Fast (Sawma Raba)",
        Rule::easter(-49),
    ),
    day("Palm Sunday", Rule::easter(PALM_SUNDAY)),
    day(
        "Easter (the season of the Resurrection begins)",
        Rule::easter(EASTER_SUNDAY),
    ),
    day("Ascension", Rule::easter(ASCENSION)),
    day(
        "Pentecost (the season of the Apostles begins)",
        Rule::easter(PENTECOST),
    ),
    day("First Sunday of Summer (Qayta)", Rule::easter(98)),
    day(
        "First Sunday of Elijah",
        Rule::Computed(chaldean_first_sunday_of_elijah),
    ),
    day("Feast of the Holy Cross", Rule::gregorian(9, 14)),
    day("First Sunday of the Cross", first_sunday_on_or_after(9, 14)),
    day(
        "First Sunday of the Dedication of the Church (Qudesh Edta)",
        first_sunday_on_or_after(10, 30),
    ),
    day(
        "First Sunday of the Annunciation (Subara)",
        first_sunday_on_or_after(11, 27),
    ),
    day("Nativity (Yelda)", Rule::gregorian(12, 25)),
];

/// The Chaldean Catholic Church's year, as its Diocese of St Thomas the
/// Apostle in the United States states it: the East Syriac seasons of
/// seven weeks, the Great Fast from "midnight on Sunday, 50 days before
/// Easter", Easter on "the first Sunday after the first full moon after
/// the Spring Equinox", Subara "between 11/27 and 12/3", the Dedication
/// from the Sunday between 30 October and 5 November, and Elijah lasting
/// "as long as it needs to until September 14th", the Feast of the Holy
/// Cross, whose Sunday on or after it is the first of the Cross. Where it
/// differs from `church-of-the-east`: the Cross on 14 September, not 13,
/// and so the Sunday Elijah must begin before; and the Sundays of the
/// Cross, which the Chaldean Sundays count beside Elijah's.
///
/// Elijah's first Sunday follows the ACE's rule with the 14th, as a
/// parish's bulletins of 2025 and 2026 date it: seven Sundays of Summer
/// and then Elijah, or six when the seventh would be on or after the Cross,
/// as in 2025. The Epiphany, Transfiguration and the Sundays of Moses, which
/// the diocese's page does not date, are not carried; nor are the
/// saints' days. The Gregorian calendar and computus are the Chaldean
/// Catholic Church's with Rome's; when it took them up was not read.
pub static CHALDEAN: RuleSet = RuleSet {
    code: "chaldean",
    english_name: "Chaldean Catholic Church",
    rules: CHALDEAN_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Chaldean Diocese of St Thomas the Apostle, \"Liturgical Year of the Chaldean \
              Church\" (chaldeanchurch.org/liturgical-year-of-the-chaldean-church/, \
              chaldean-liturgical-year), for the seasons, the Great Fast, Easter, Subara, the \
              Dedication, Elijah and the Feast of the Holy Cross on 14 September; St George \
              Chaldean Catholic Church's weekly bulletins of 2025-2026 \
              (stgeorgechaldean.com/category/announcements/, st-george-chaldean-bulletins), for \
              the Sundays they name; Chaldean News, \"A Historic Easter\" (31 March 2025, \
              chaldean-news-easter-2025), secondary, for the calendar aligned with Rome's; all \
              retrieved 2026-09-29",
    subdivisions: Subdivisions::Undivided,
};

/// The season of Denha: "The Sunday between January 2 and 6; otherwise
/// January 6".
fn syro_malabar_denha(year: i64) -> Days {
    let Ok(second) = gregorian::to_fixed(year, 1, 2) else {
        return Days::new();
    };
    let sunday = Weekday::Sunday.on_or_after(second);
    Days::one(if sunday.0 - second.0 <= 4 {
        sunday
    } else {
        Rd(second.0 + 4)
    })
}

static SYRO_MALABAR_RULES: &[HolidayRule] = &[
    day("Epiphany (Denha)", Rule::gregorian(1, 6)),
    day(
        "The season of Denha begins",
        Rule::Computed(syro_malabar_denha),
    ),
    day("Rogation of the Ninevites", Rule::easter(-69)),
    day("Rogation of the Ninevites", Rule::easter(-68)),
    day("Rogation of the Ninevites", Rule::easter(-67)),
    day("First Sunday of the Great Fast", Rule::easter(-49)),
    day("Palm Sunday", Rule::easter(PALM_SUNDAY)),
    day(
        "Easter (the season of the Resurrection begins)",
        Rule::easter(EASTER_SUNDAY),
    ),
    day("Ascension", Rule::easter(ASCENSION)),
    day(
        "Pentecost (the season of the Apostles begins)",
        Rule::easter(PENTECOST),
    ),
    day("SS. Peter and Paul", Rule::gregorian(6, 29)),
    day("Dukrana of St Thomas", Rule::gregorian(7, 3)),
    day("First Sunday of Summer (Kaitha)", Rule::easter(98)),
    day("Transfiguration", Rule::gregorian(8, 6)),
    day("Assumption", Rule::gregorian(8, 15)),
    day(
        "First Sunday of the season of Elijah, the Cross and Moses",
        Rule::easter(147),
    ),
    day("Exaltation of the Holy Cross", Rule::gregorian(9, 14)),
    day("First Sunday of the Cross", first_sunday_on_or_after(9, 14)),
    day(
        "First Sunday of the Dedication of the Church",
        first_sunday_on_or_after(10, 30),
    ),
    day(
        "First Sunday of the Annunciation",
        first_sunday_on_or_after(11, 27),
    ),
    day("Nativity", Rule::gregorian(12, 25)),
];

/// The Syro-Malabar Catholic Church's year: the East Syriac seasons, the
/// season of Denha from "The Sunday between January 2 and 6; otherwise
/// January 6", the Great Fast from "The 7th Sunday before Easter", Summer
/// (Kaitha) from "The 7th Sunday after Pentecost", the season of Elijah,
/// the Cross and Moses from "The 14th Sunday after Pentecost", the
/// Dedication from "The Sunday between October 30 and November 5", and the
/// days of obligation of 6 January, 29 June, 3 July, 15 August and
/// 25 December, from Wikipedia's account of the calendar, which cites the
/// Church's calendar of 2020–21 (secondary); the Cross "celebrated on 14th
/// September" (a parish in Fremont, California) and its first Sunday the
/// Sunday on or after it (a homily site); the Transfiguration on 6 August
/// (secondary). Where it differs from `church-of-the-east`: Denha may begin
/// before the Epiphany, Elijah's season begins on the fourteenth Sunday
/// after Pentecost whatever the day of the Cross, and the Cross is on
/// 14 September, its Sunday the first of the Cross. The Church's own
/// yearly calendar, the Panchangam, is a PDF and was not read; the
/// Rogation and Annunciation are the East Syriac year's, which the account
/// gives as the third Monday before Lent and the Sunday between
/// 27 November and 3 December.
pub static SYRO_MALABAR: RuleSet = RuleSet {
    code: "syro-malabar",
    english_name: "Syro-Malabar Catholic Church",
    rules: SYRO_MALABAR_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Wikipedia, \"Liturgical calendar of the Syro-Malabar Catholic Church\" \
              (wikipedia-syro-malabar-calendar), secondary, citing the Church's calendar of \
              2020-21 (a PDF, not read), for the seasons and the days of obligation; St Thomas \
              Syro-Malabar Catholic Forane Church, Fremont, \"Liturgical Seasons\" \
              (stthomassfo.org/parish/liturgy/liturgical-seasons-new, st-thomas-sfo-seasons), for \
              the Cross on 14 September; christianhomily.com, \"Elijah-Cross-Moses\" \
              (christianhomily-elijah-cross-moses), for the first Sunday of the Cross; Father \
              Raymond de Souza, \"Syro-Malabar calendar elevates summer feasts\" \
              (fatherdesouza-syro-malabar-2020), secondary, for the Transfiguration on 6 August; \
              all retrieved 2026-09-29",
    subdivisions: Subdivisions::Undivided,
};

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    fn dates(
        set: &RuleSet,
        name: &str,
        years: core::ops::RangeInclusive<i64>,
    ) -> Vec<(i64, u8, u8)> {
        let mut out = Vec::new();
        for year in years {
            for rule in set.rules.iter().filter(|rule| rule.name == name) {
                for day in rule.rule.days_in_year(year).as_slice() {
                    out.push(gregorian::from_fixed(*day).unwrap());
                }
            }
        }
        out.sort_unstable();
        out
    }

    /// Every commemoration against the Assyrian Church of the East's
    /// calendar of 2026–2029 (`ace-liturgical-calendar`). The calendar's
    /// 2028 puts the penultimate and the last Fridays before the Great Fast
    /// a week early, 11 and 18 February against a Great Fast of 27 February,
    /// as it puts that year's Rogation of the Ninevites a month early; the
    /// statement's rule, the Fridays before the Great Fast, gives 18 and
    /// 25 February.
    #[test]
    fn the_commemorations_are_the_calendar_of_2026_to_2029() {
        type Row = (&'static str, &'static [(i64, u8, u8)]);
        let expected: &[Row] = &[
            (
                "Commemoration of the Virgin Mary (after Christmas)",
                &[(2026, 1, 2), (2027, 1, 1), (2027, 12, 31), (2029, 1, 5)],
            ),
            (
                "Commemoration of St. John the Baptist",
                &[(2026, 1, 9), (2027, 1, 8), (2028, 1, 7), (2029, 1, 12)],
            ),
            (
                "Commemoration of the patron of the local church (St. Awa Catholicos)",
                &[(2026, 2, 6), (2027, 1, 29), (2028, 2, 11), (2029, 2, 2)],
            ),
            (
                "Friday of the Deceased",
                &[(2026, 2, 13), (2027, 2, 5), (2028, 2, 18), (2029, 2, 9)],
            ),
            (
                "Commemoration of Mar Benyamin Shimun XXI Catholicos Patriarch the Martyr",
                &[(2026, 2, 9), (2027, 2, 1), (2028, 2, 21), (2029, 2, 5)],
            ),
            (
                "Middle Week of the Mysteries",
                &[(2026, 3, 11), (2027, 3, 3), (2028, 3, 22), (2029, 3, 7)],
            ),
            (
                "Friday of Lazarus",
                &[(2026, 3, 27), (2027, 3, 19), (2028, 4, 7), (2029, 3, 23)],
            ),
            (
                "Thursday of Passover",
                &[(2026, 4, 2), (2027, 3, 25), (2028, 4, 13), (2029, 3, 29)],
            ),
            (
                "Good Friday",
                &[(2026, 4, 3), (2027, 3, 26), (2028, 4, 14), (2029, 3, 30)],
            ),
            (
                "Great Saturday",
                &[(2026, 4, 4), (2027, 3, 27), (2028, 4, 15), (2029, 3, 31)],
            ),
            (
                "Monday of the Thief",
                &[(2026, 4, 6), (2027, 3, 29), (2028, 4, 17), (2029, 4, 2)],
            ),
            (
                "Friday of Confessors",
                &[(2026, 4, 10), (2027, 4, 2), (2028, 4, 21), (2029, 4, 6)],
            ),
            (
                "New Sunday and Commemoration of St. Moses",
                &[(2026, 4, 12), (2027, 4, 4), (2028, 4, 23), (2029, 4, 8)],
            ),
            (
                "Commemoration of St. Abraham Mede",
                &[(2026, 4, 13), (2027, 4, 5), (2028, 4, 24), (2029, 4, 9)],
            ),
            (
                "Commemoration of St. Qayoma",
                &[(2026, 4, 13), (2027, 4, 5), (2028, 4, 24), (2029, 4, 9)],
            ),
            (
                "Commemoration of St. Narsai the Teacher",
                &[(2026, 4, 16), (2027, 4, 8), (2028, 4, 27), (2029, 4, 12)],
            ),
            (
                "Commemoration of St. Pinkhis",
                &[(2026, 4, 17), (2027, 4, 9), (2028, 4, 28), (2029, 4, 13)],
            ),
            (
                "Commemoration of St. Abdisho, St. Jonah the Ascetic and St. Khanania",
                &[(2026, 4, 20), (2027, 4, 12), (2028, 5, 1), (2029, 4, 16)],
            ),
            (
                "Commemoration of St. Hurmizd",
                &[(2026, 4, 20), (2027, 4, 12), (2028, 5, 1), (2029, 4, 16)],
            ),
            (
                "Commemoration of St Elijah III Catholicos (Abu Khalim)",
                &[(2026, 4, 24), (2027, 4, 16), (2028, 5, 5), (2029, 4, 20)],
            ),
            (
                "Commemoration of St. Awraham of Kashkar",
                &[(2026, 4, 27), (2027, 4, 19), (2028, 5, 8), (2029, 4, 23)],
            ),
            (
                "Commemoration of St. Issac of Nineveh",
                &[(2026, 5, 1), (2027, 4, 23), (2028, 5, 12), (2029, 4, 27)],
            ),
            (
                "Commemoration of St. Addai the Apostle",
                &[(2026, 5, 3), (2027, 4, 25), (2028, 5, 14), (2029, 4, 29)],
            ),
            (
                "Commemoration of King Abgar Awkama",
                &[(2026, 5, 4), (2027, 4, 26), (2028, 5, 15), (2029, 4, 30)],
            ),
            (
                "Commemoration of St. Andrew the Apostle",
                &[(2026, 5, 11), (2027, 5, 3), (2028, 5, 22), (2029, 5, 7)],
            ),
            (
                "Sunday after Ascension",
                &[(2026, 5, 17), (2027, 5, 9), (2028, 5, 28), (2029, 5, 13)],
            ),
            (
                "Friday of Gold",
                &[(2026, 5, 29), (2027, 5, 21), (2028, 6, 9), (2029, 5, 25)],
            ),
            (
                "Commemoration of St. Meelis and St. Bar Qusry",
                &[(2026, 6, 15), (2027, 6, 7), (2028, 6, 26), (2029, 6, 11)],
            ),
            (
                "Commemoration of St. Fibronia the martyr",
                &[(2026, 6, 19), (2027, 6, 11), (2028, 6, 30), (2029, 6, 15)],
            ),
            (
                "Commemoration of St. Awa Srapion",
                &[(2026, 6, 22), (2027, 6, 14), (2028, 7, 3), (2029, 6, 18)],
            ),
            (
                "Commemoration of St. Rabban Pethyon",
                &[(2026, 7, 3), (2027, 6, 25), (2028, 7, 14), (2029, 6, 29)],
            ),
            (
                "Commemoration of St. Ezekiel of Daqoq",
                &[(2026, 7, 6), (2027, 6, 28), (2028, 7, 17), (2029, 7, 2)],
            ),
            (
                "Commemoration of the Seventy-Two Apostles",
                &[(2026, 7, 10), (2027, 7, 2), (2028, 7, 21), (2029, 7, 6)],
            ),
            (
                "Commemoration of St. Jacob of Nisibis",
                &[(2026, 7, 17), (2027, 7, 9), (2028, 7, 28), (2029, 7, 13)],
            ),
            (
                "Commemoration of St. Mari the Apostle",
                &[(2026, 7, 24), (2027, 7, 16), (2028, 8, 4), (2029, 7, 20)],
            ),
            (
                "Commemoration of St. Shimon Bar Sabbae",
                &[(2026, 8, 21), (2027, 8, 13), (2028, 9, 1), (2029, 8, 17)],
            ),
            (
                "Commemoration of St. George the martyr",
                &[(2026, 4, 24), (2027, 4, 24), (2028, 4, 24), (2029, 4, 24)],
            ),
            (
                "Commemoration of St. Awimalk Timothy",
                &[(2026, 5, 1), (2027, 5, 1), (2028, 5, 1), (2029, 5, 1)],
            ),
            (
                "Commemoration of all Catholicos Patriarchs of Seleucia-Ctesiphon",
                &[(2026, 6, 1), (2027, 6, 1), (2028, 6, 1), (2029, 6, 1)],
            ),
            (
                "Commemoration of St. Ephraim the Teacher",
                &[(2026, 6, 9), (2027, 6, 9), (2028, 6, 9), (2029, 6, 9)],
            ),
            (
                "Commemoration of St. Thomas the Apostle",
                &[(2026, 7, 3), (2027, 7, 3), (2028, 7, 3), (2029, 7, 3)],
            ),
            (
                "Assyrian Martyrs Day",
                &[(2026, 8, 7), (2027, 8, 7), (2028, 8, 7), (2029, 8, 7)],
            ),
            (
                "Commemoration of St. Hurmizd & St. Tawor",
                &[(2026, 9, 1), (2027, 9, 1), (2028, 9, 1), (2029, 9, 1)],
            ),
            (
                "Virgin Mary's Birthday",
                &[(2026, 9, 8), (2027, 9, 8), (2028, 9, 8), (2029, 9, 8)],
            ),
            (
                "Commemoration of St. Sawa the Doctor, St. Abdisho, St. Zaia, St. Bisho and all Saints",
                &[(2026, 9, 14), (2027, 9, 14), (2028, 9, 14), (2029, 9, 14)],
            ),
            (
                "Commemoration of St Rabban Yareth",
                &[
                    (2026, 10, 27),
                    (2027, 10, 27),
                    (2028, 10, 27),
                    (2029, 10, 27),
                ],
            ),
            (
                "Commemoration of St. Mikha",
                &[(2026, 11, 3), (2027, 11, 3), (2028, 11, 3), (2029, 11, 3)],
            ),
            (
                "Commemoration of the Virgin Mary",
                &[(2026, 5, 15), (2027, 5, 15), (2028, 5, 15), (2029, 5, 15)],
            ),
            (
                "The Fast of the Virgin Mary",
                &[(2026, 8, 1), (2027, 8, 2), (2028, 8, 1), (2029, 8, 1)],
            ),
            (
                "Commemoration of the Virgin Mary (Assumption)",
                &[(2026, 8, 15), (2027, 8, 16), (2028, 8, 15), (2029, 8, 15)],
            ),
        ];
        let set = &crate::traditions::CHURCH_OF_THE_EAST;
        for (name, feed) in expected {
            let mut feed: Vec<(i64, u8, u8)> = feed.to_vec();
            if *name == "Commemoration of the patron of the local church (St. Awa Catholicos)" {
                feed.retain(|d| d.0 != 2028);
                feed.push((2028, 2, 18));
            }
            if *name == "Friday of the Deceased" {
                feed.retain(|d| d.0 != 2028);
                feed.push((2028, 2, 25));
            }
            feed.sort_unstable();
            assert_eq!(dates(set, name, 2026..=2029), feed, "{name}");
        }
        assert_eq!(
            expected.len(),
            CHURCH_OF_THE_EAST_COMMEMORATIONS.len(),
            "every commemoration checked"
        );
    }

    #[test]
    fn a_fixed_saints_day_on_a_sunday_is_a_gap() {
        // St George, 24 April: a Sunday in 2022, a gap; the Virgin's
        // 15 August 2027, a Sunday, on the Monday as the calendar keeps it.
        let set = &crate::traditions::CHURCH_OF_THE_EAST;
        assert!(dates(set, "Commemoration of St. George the martyr", 2022..=2022).is_empty());
        assert_eq!(
            dates(
                set,
                "Commemoration of the Virgin Mary (Assumption)",
                2027..=2027
            ),
            [(2027, 8, 16)]
        );
        let calendar = crate::engine::HolidayCalendar::for_year(set, None, 2022);
        assert!(!calendar.gaps().is_empty());
    }

    /// The Chaldean parish bulletins of 2025 and 2026
    /// (`st-george-chaldean-bulletins`): Elijah's first Sunday a week early
    /// in 2025, after six Sundays of Summer, since the seventh would have
    /// been 14 September; the second Sunday of the Cross on 27 September
    /// 2026; Ba'utha from 26 January 2026; the Dedication on 2 November
    /// 2025 and Subara on 30 November 2025.
    #[test]
    fn the_chaldean_year_is_the_parish_bulletins() {
        let set = &CHALDEAN;
        assert_eq!(
            dates(set, "First Sunday of Summer (Qayta)", 2025..=2026),
            [(2025, 7, 27), (2026, 7, 12)]
        );
        assert_eq!(
            dates(set, "First Sunday of Elijah", 2025..=2026),
            [(2025, 9, 7), (2026, 8, 30)]
        );
        assert_eq!(
            dates(set, "First Sunday of the Cross", 2026..=2026),
            [(2026, 9, 20)]
        );
        assert_eq!(
            dates(set, "Rogation of the Ninevites (Ba'utha)", 2026..=2026),
            [(2026, 1, 26), (2026, 1, 27), (2026, 1, 28)]
        );
        assert_eq!(
            dates(
                set,
                "First Sunday of the Great Fast (Sawma Raba)",
                2026..=2026
            ),
            [(2026, 2, 15)]
        );
        assert_eq!(
            dates(
                set,
                "First Sunday of the Dedication of the Church (Qudesh Edta)",
                2025..=2025
            ),
            [(2025, 11, 2)]
        );
        assert_eq!(
            dates(
                set,
                "First Sunday of the Annunciation (Subara)",
                2025..=2025
            ),
            [(2025, 11, 30)]
        );
        assert_eq!(dates(set, "Palm Sunday", 2026..=2026), [(2026, 3, 29)]);
        assert_eq!(
            dates(
                set,
                "Pentecost (the season of the Apostles begins)",
                2026..=2026
            ),
            [(2026, 5, 24)]
        );
        // Elijah always begins before the Cross.
        for year in 1900..=2100 {
            let elijah = dates(set, "First Sunday of Elijah", year..=year);
            assert!(elijah[0] < (year, 9, 14), "{year}");
        }
    }

    /// The Syro-Malabar Church's daily page named 29 September 2026 the
    /// "Second Tuesday of Cross" (`syro-malabar-liturgy-daily`): the first
    /// Sunday of the Cross was 20 September, the Sunday after the 14th.
    #[test]
    fn the_syro_malabar_cross_begins_on_the_sunday_after_14_september() {
        let set = &SYRO_MALABAR;
        assert_eq!(
            dates(set, "First Sunday of the Cross", 2026..=2026),
            [(2026, 9, 20)]
        );
        // 14 September 2025 was a Sunday, itself the first.
        assert_eq!(
            dates(set, "First Sunday of the Cross", 2025..=2025),
            [(2025, 9, 14)]
        );
        assert_eq!(
            dates(
                set,
                "First Sunday of the season of Elijah, the Cross and Moses",
                2026..=2026
            ),
            [(2026, 8, 30)]
        );
        // Denha: Sunday 4 January 2026; 6 January 2023, whose 1 January
        // was a Sunday and 2–6 January held none.
        assert_eq!(
            dates(set, "The season of Denha begins", 2026..=2026),
            [(2026, 1, 4)]
        );
        assert_eq!(
            dates(set, "The season of Denha begins", 2023..=2023),
            [(2023, 1, 6)]
        );
    }
}
