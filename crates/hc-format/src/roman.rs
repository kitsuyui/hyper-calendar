//! Dates the way a Roman wrote them: *a.d. III Kal. Apr.*
//!
//! `hc_calendars_solar::roman` implements years *ab urbe condita* and
//! leaves the counting of days by kalends, nones and ides to this module,
//! as a formatting question rather than a calendar one.
//!
//! The rules are those of Reingold and Dershowitz, *Calendrical
//! Calculations* (`reingold2018`), chapter 3 (the Julian calendar), as
//! their `roman-from-fixed`, `fixed-from-roman`, `ides-of-month` and
//! `nones-of-month` state them (`reingold2018code`, `calendar.l`, read
//! 2026-09-26): the Ides on the 15th of March, May, July and October and
//! the 13th otherwise, the Nones eight days earlier, and in a Julian leap
//! year the 25th of February as the repeated day. The *bis sextum* reading
//! of the leap day is the one the jurist Celsus gives in the *Digest*,
//! 50.16.98 (not read here).
//!
//! # The system
//!
//! Three named days anchor each month, and every other day is counted
//! *backwards* to the next one, inclusively — which is why the numbers are
//! one larger than a modern reader expects.
//!
//! | Day | Ordinary month | March, May, July, October |
//! | --- | --- | --- |
//! | Kalendae | the 1st | the 1st |
//! | Nonae | the 5th | the 7th |
//! | Idus | the 13th | the 15th |
//!
//! Days after the Ides count back to the Kalends of the *next* month. The
//! day immediately before a named day is not counted but named: *pridie*.
//!
//! # The leap day, and why there are two styles
//!
//! The Julian leap day was not a 29 February. It was a *doubled* 24
//! February — *bis sextum Kalendas Martias*, "a second sixth day before the
//! Kalends of March", which is where the word bissextile comes from. So in
//! a leap year two consecutive days both bear *a.d. VI Kal. Mart.*, and the
//! days after them keep the numbers they have in a common year.
//!
//! Modern editors more often just count straight back through a 29-day
//! February, which gives *a.d. VII Kal. Mart.* for 24 February and shifts
//! everything after it.
//!
//! Both are in use, and they disagree on exactly two days of a leap year —
//! the 24th and the 25th of February. Everything from the fifth day before
//! the Kalends onward keeps its number either way, because the intercalary
//! day went in between the sixth and the seventh. Two days is a small
//! disagreement and a silent one, which is the kind policy §5 exists for:
//! each gets a name rather than one being the default.
//!
//! # What this does not do
//!
//! It does not touch the *republican* calendar before 45 BC, whose
//! intercalations were political and are still disputed. That limit belongs
//! to the calendar and `hc_calendars_solar::roman` states it.

use core::fmt;

use hc_calendar::{CalendarError, CalendarResult, Rd};
use hc_calendars_solar::julian;

/// Which of the three named days a date counts back to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Anchor {
    /// *Kalendae*, the first of the month.
    Kalends,
    /// *Nonae*, the fifth or seventh.
    Nones,
    /// *Idus*, the thirteenth or fifteenth.
    Ides,
}

impl Anchor {
    /// The abbreviation, in the accusative the counting formula takes.
    #[must_use]
    pub const fn abbreviation(self) -> &'static str {
        match self {
            Self::Kalends => "Kal.",
            Self::Nones => "Non.",
            Self::Ides => "Id.",
        }
    }

    /// The full Latin name, in the accusative.
    #[must_use]
    pub const fn latin(self) -> &'static str {
        match self {
            Self::Kalends => "Kalendas",
            Self::Nones => "Nonas",
            Self::Ides => "Idus",
        }
    }
}

/// How to write the Julian leap day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BissextileStyle {
    /// The Roman way: 24 and 25 February both count as the sixth day before
    /// the Kalends of March, the second marked *bis*.
    ///
    /// This is what the reform actually did and what the word bissextile
    /// records, so it is the default.
    #[default]
    Doubled,
    /// The modern editorial way: count straight back through a 29-day
    /// February, so 24 February is the seventh day before the Kalends.
    Counted,
}

/// The months in the accusative, as they appear after the anchor.
const MONTHS: [&str; 12] = [
    "Ian.", "Feb.", "Mart.", "Apr.", "Mai.", "Iun.", "Iul.", "Aug.", "Sept.", "Oct.", "Nov.",
    "Dec.",
];

/// The months whose Nones fall on the 7th and Ides on the 15th.
const LONG_ANCHOR_MONTHS: [u8; 4] = [3, 5, 7, 10];

/// Whether `month` takes the later Nones and Ides.
const fn has_long_anchors(month: u8) -> bool {
    let mut index = 0;
    while index < LONG_ANCHOR_MONTHS.len() {
        if LONG_ANCHOR_MONTHS[index] == month {
            return true;
        }
        index += 1;
    }
    false
}

/// The day of the month the Nones fall on.
#[must_use]
pub const fn nones_of(month: u8) -> u8 {
    if has_long_anchors(month) { 7 } else { 5 }
}

/// The day of the month the Ides fall on.
#[must_use]
pub const fn ides_of(month: u8) -> u8 {
    if has_long_anchors(month) { 15 } else { 13 }
}

/// A date written the Roman way.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RomanDayName {
    /// The named day counted back to.
    pub anchor: Anchor,
    /// The month the anchor belongs to, 1 through 12. For days after the
    /// Ides this is the *following* month.
    pub anchor_month: u8,
    /// How many days back, counted inclusively. 1 means the anchor itself,
    /// 2 means *pridie*.
    pub count: u8,
    /// Whether this is the intercalated second sixth day, written *bis*.
    pub bissextile: bool,
}

impl RomanDayName {
    /// The Roman name of a Julian date.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::MonthOutOfRange`] or
    /// [`CalendarError::DayOutOfRange`] for a date that does not exist.
    pub const fn of_julian(
        year: i64,
        month: u8,
        day: u8,
        style: BissextileStyle,
    ) -> CalendarResult<Self> {
        let Some(length) = julian::days_in_month(year, month) else {
            return Err(CalendarError::MonthOutOfRange);
        };
        if day == 0 || day > length {
            return Err(CalendarError::DayOutOfRange);
        }

        let leap_february = month == 2 && length == 29;
        if leap_february && matches!(style, BissextileStyle::Doubled) {
            // 24 February is a.d. VI Kal. Mart.; 25 February is the
            // intercalated repeat of it; from 26 February the count is the
            // common-year one, which runs back from a 28-day month.
            if day == 24 {
                return Ok(Self {
                    anchor: Anchor::Kalends,
                    anchor_month: 3,
                    count: 6,
                    bissextile: false,
                });
            }
            if day == 25 {
                return Ok(Self {
                    anchor: Anchor::Kalends,
                    anchor_month: 3,
                    count: 6,
                    bissextile: true,
                });
            }
            if day > 25 {
                return Ok(Self {
                    anchor: Anchor::Kalends,
                    anchor_month: 3,
                    // 26 February behaves as the 25th of a 28-day February.
                    count: 28 - (day - 1) + 2,
                    bissextile: false,
                });
            }
        }

        let nones = nones_of(month);
        let ides = ides_of(month);
        if day == 1 {
            return Ok(Self {
                anchor: Anchor::Kalends,
                anchor_month: month,
                count: 1,
                bissextile: false,
            });
        }
        if day <= nones {
            return Ok(Self {
                anchor: Anchor::Nones,
                anchor_month: month,
                count: nones - day + 1,
                bissextile: false,
            });
        }
        if day <= ides {
            return Ok(Self {
                anchor: Anchor::Ides,
                anchor_month: month,
                count: ides - day + 1,
                bissextile: false,
            });
        }
        Ok(Self {
            anchor: Anchor::Kalends,
            anchor_month: if month == 12 { 1 } else { month + 1 },
            count: length - day + 2,
            bissextile: false,
        })
    }

    /// The Roman name of a fixed day, read through the Julian calendar.
    ///
    /// # Errors
    ///
    /// Propagates [`hc_calendars_solar::julian`]'s errors.
    pub fn of_fixed(rd: Rd, style: BissextileStyle) -> CalendarResult<Self> {
        let (year, month, day) = julian::from_fixed(rd)?;
        Self::of_julian(year, month, day, style)
    }

    /// Whether this names one of the three anchor days itself.
    #[must_use]
    pub const fn is_anchor_day(self) -> bool {
        self.count == 1
    }

    /// Whether this is the day before an anchor, written *pridie*.
    #[must_use]
    pub const fn is_pridie(self) -> bool {
        self.count == 2 && !self.bissextile
    }

    /// The month name the formula ends with.
    #[must_use]
    pub const fn month_name(self) -> &'static str {
        MONTHS[self.anchor_month as usize - 1]
    }
}

impl fmt::Display for RomanDayName {
    /// Writes `Kal. Apr.`, `prid. Id. Mart.`, `a.d. III Kal. Apr.` or
    /// `a.d. bis VI Kal. Mart.`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_anchor_day() {
            return write!(f, "{} {}", self.anchor.abbreviation(), self.month_name());
        }
        if self.is_pridie() {
            return write!(
                f,
                "prid. {} {}",
                self.anchor.abbreviation(),
                self.month_name()
            );
        }
        f.write_str("a.d. ")?;
        if self.bissextile {
            f.write_str("bis ")?;
        }
        write_roman_numeral(f, self.count)?;
        write!(f, " {} {}", self.anchor.abbreviation(), self.month_name())
    }
}

/// Writes a small Roman numeral, which is all this notation ever needs.
///
/// The largest count the system produces is XIX — the nineteenth day before
/// the Kalends of January, which is 14 December.
fn write_roman_numeral(f: &mut fmt::Formatter<'_>, mut value: u8) -> fmt::Result {
    const PARTS: [(u8, &str); 7] = [
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
    ];
    for (amount, numeral) in PARTS {
        while value >= amount {
            f.write_str(numeral)?;
            value -= amount;
        }
    }
    if value == 4 {
        return f.write_str("IV");
    }
    for _ in 0..value {
        f.write_str("I")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use alloc::string::ToString;

    use super::*;

    fn name(year: i64, month: u8, day: u8) -> alloc::string::String {
        RomanDayName::of_julian(year, month, day, BissextileStyle::Doubled)
            .expect("a real date")
            .to_string()
    }

    /// The example `roman.rs` gives in its own doc comment.
    #[test]
    fn the_thirtieth_of_march_is_the_third_day_before_the_kalends_of_april() {
        assert_eq!(name(100, 3, 30), "a.d. III Kal. Apr.");
    }

    #[test]
    fn the_three_anchors_are_named_not_counted() {
        assert_eq!(name(100, 4, 1), "Kal. Apr.");
        assert_eq!(name(100, 4, 5), "Non. Apr.");
        assert_eq!(name(100, 4, 13), "Id. Apr.");
        // March, May, July and October take the later ones.
        assert_eq!(name(100, 3, 7), "Non. Mart.");
        assert_eq!(name(100, 3, 15), "Id. Mart.");
        assert_eq!(name(100, 10, 7), "Non. Oct.");
        assert_eq!(name(100, 10, 15), "Id. Oct.");
    }

    /// The most famous date in the calendar.
    #[test]
    fn the_ides_of_march_are_the_fifteenth() {
        assert_eq!(ides_of(3), 15);
        assert_eq!(name(-43, 3, 15), "Id. Mart.");
    }

    #[test]
    fn the_day_before_an_anchor_is_pridie() {
        assert_eq!(name(100, 3, 31), "prid. Kal. Apr.");
        assert_eq!(name(100, 3, 14), "prid. Id. Mart.");
        assert_eq!(name(100, 3, 6), "prid. Non. Mart.");
    }

    #[test]
    fn days_after_the_ides_count_to_the_next_months_kalends() {
        // 14 December is the largest count the system produces.
        assert_eq!(name(100, 12, 14), "a.d. XIX Kal. Ian.");
        assert_eq!(name(100, 12, 31), "prid. Kal. Ian.");
        assert_eq!(name(100, 1, 14), "a.d. XIX Kal. Feb.");
    }

    /// The doubled day, which is where the word bissextile comes from.
    #[test]
    fn a_leap_year_has_two_sixth_days_before_the_kalends_of_march() {
        assert!(julian::is_leap_year(100));
        assert_eq!(name(100, 2, 24), "a.d. VI Kal. Mart.");
        assert_eq!(name(100, 2, 25), "a.d. bis VI Kal. Mart.");
        // And the days after keep their common-year numbers.
        assert_eq!(name(100, 2, 26), "a.d. V Kal. Mart.");
        assert_eq!(name(100, 2, 27), "a.d. IV Kal. Mart.");
        assert_eq!(name(100, 2, 28), "a.d. III Kal. Mart.");
        assert_eq!(name(100, 2, 29), "prid. Kal. Mart.");

        // A common year has one sixth day and nothing before the 24th moves.
        assert!(!julian::is_leap_year(101));
        assert_eq!(name(101, 2, 24), "a.d. VI Kal. Mart.");
        assert_eq!(name(101, 2, 28), "prid. Kal. Mart.");
    }

    /// The other style, and exactly where the two disagree — which is two
    /// days a leap year and not, as I first wrote, five. The intercalary
    /// day was inserted between the sixth and seventh days before the
    /// Kalends, so everything from the fifth onward keeps its number in
    /// both styles.
    #[test]
    fn the_counted_style_shifts_two_days_and_no_others() {
        let counted = |day: u8| {
            RomanDayName::of_julian(100, 2, day, BissextileStyle::Counted)
                .expect("a real date")
                .to_string()
        };
        assert_eq!(counted(24), "a.d. VII Kal. Mart.");
        assert_eq!(counted(25), "a.d. VI Kal. Mart.");
        assert_eq!(counted(29), "prid. Kal. Mart.");

        // The two styles agree on every day of a leap February except the
        // 24th and the 25th.
        for day in 1..=29u8 {
            let doubled = name(100, 2, day);
            let counted = counted(day);
            if day == 24 || day == 25 {
                assert_ne!(doubled, counted, "{day} February should differ");
            } else {
                assert_eq!(doubled, counted, "{day} February should agree");
            }
        }
        // And on every day of a common year, where there is nothing to
        // double.
        for day in 1..=28u8 {
            assert_eq!(
                name(101, 2, day),
                RomanDayName::of_julian(101, 2, day, BissextileStyle::Counted)
                    .expect("a real date")
                    .to_string()
            );
        }
    }

    /// Every day of a leap year and a common year must produce a name, and
    /// the counts must stay inside the range the system can express.
    #[test]
    fn every_day_of_a_year_has_a_name() {
        for year in [100, 101, 1582, 1900] {
            for month in 1..=12u8 {
                let length = julian::days_in_month(year, month).expect("a real month");
                for day in 1..=length {
                    for style in [BissextileStyle::Doubled, BissextileStyle::Counted] {
                        let name =
                            RomanDayName::of_julian(year, month, day, style).expect("a real date");
                        assert!(
                            (1..=19).contains(&name.count),
                            "{year}-{month}-{day} gave {}",
                            name.count
                        );
                        assert!(!name.to_string().is_empty());
                    }
                }
            }
        }
    }

    /// Distinct days must get distinct names, or the notation would be
    /// ambiguous — except for the bissextile pair, which is ambiguous on
    /// purpose and is disambiguated by *bis*.
    #[test]
    fn distinct_days_get_distinct_names_within_a_year() {
        extern crate alloc;
        use alloc::collections::BTreeSet;
        use alloc::string::String;
        for year in [100, 101] {
            let mut seen: BTreeSet<String> = BTreeSet::new();
            for month in 1..=12u8 {
                let length = julian::days_in_month(year, month).expect("a real month");
                for day in 1..=length {
                    let rendered = name(year, month, day);
                    assert!(
                        seen.insert(rendered.clone()),
                        "{rendered} repeated in {year}"
                    );
                }
            }
        }
    }

    #[test]
    fn a_fixed_day_can_be_named_directly() {
        let rd = julian::to_fixed(-43, 3, 15).expect("a real date");
        let name = RomanDayName::of_fixed(rd, BissextileStyle::Doubled).expect("in range");
        assert_eq!(name.to_string(), "Id. Mart.");
        assert_eq!(name.anchor, Anchor::Ides);
        assert!(name.is_anchor_day());
    }

    #[test]
    fn an_impossible_date_is_refused() {
        assert_eq!(
            RomanDayName::of_julian(101, 2, 29, BissextileStyle::Doubled),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            RomanDayName::of_julian(100, 13, 1, BissextileStyle::Doubled),
            Err(CalendarError::MonthOutOfRange)
        );
    }

    #[test]
    fn the_numerals_are_the_ones_the_notation_uses() {
        // Exercised through the dates that produce each count.
        assert_eq!(name(100, 4, 2), "a.d. IV Non. Apr.");
        assert_eq!(name(100, 1, 20), "a.d. XIII Kal. Feb.");
        assert_eq!(name(100, 1, 16), "a.d. XVII Kal. Feb.");
        assert_eq!(name(100, 1, 15), "a.d. XVIII Kal. Feb.");
    }
}
