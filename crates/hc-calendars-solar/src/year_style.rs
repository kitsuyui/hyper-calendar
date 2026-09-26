//! Where the year began, before 1 January won.
//!
//! [`crate::julian_gregorian`] models the day a country moved from the
//! Julian calendar to the Gregorian one, and says in its own doc comment
//! that it does not model the *start of the year*: "recovering that
//! needs the scribe's convention, not just the country". That is
//! true, and it is precisely the case policy §5 answers — a finite set of
//! named conventions the caller selects, rather than one silent default.
//!
//! The stakes are a whole year. A charter dated "12 February 1721" in
//! England means 1722 by modern reckoning. Pisa and Florence are forty
//! miles apart and their year numbers differed by one, in opposite
//! directions from the modern year, for the same day. Getting this wrong is
//! the commonest error in medieval dating, and it is silent: the date looks
//! perfectly ordinary.
//!
//! # What this is and is not
//!
//! It is not a calendar. The month and day are whatever the underlying
//! calendar says — usually [`crate::julian`] — and only the *year number*
//! changes. So this is a conversion between a document's year and a
//! historian's, and the API is shaped that way: give it a day and it tells
//! you what a scribe would have written; give it a dateline and it tells
//! you which day is meant.
//!
//! # The Easter style is not here
//!
//! *Mos gallicanus* began the year at Easter, which moves. That makes the
//! year between 330 and 400 days long, so some dates occur **twice** in one
//! year and some not at all — 30 March 1201 in Paris is genuinely
//! ambiguous, not merely unknown. A conversion that returned one answer
//! would be wrong half the time. It needs the computus, an explicit
//! occurrence index, and an API that can say "twice"; until it has all
//! three it is better absent than guessed. Policy §4.
//!
//! **Sources:** C. R. Cheney, *A Handbook of Dates for Students of British
//! History*, revised by Michael Jones (Cambridge, 2000; `cheney2000`);
//! Adriano Cappelli, *Cronologia, Cronografia e Calendario Perpetuo*
//! (7th ed., Milan, 1998; `cappelli1998`); H. Grotefend, *Zeitrechnung des
//! deutschen Mittelalters und der Neuzeit* (Hanover, 1891–1898;
//! `grotefend1891`); V. Grumel, *La chronologie* (Paris, 1958;
//! `grumel1958`), none of them read here: they are named as the authority
//! for each style below. The last English year to begin on 25 March is
//! fixed by the Calendar (New Style) Act 1750 (`uk-calendar-act-1750`).

use hc_calendar::{CalendarError, CalendarResult, Rd};

use crate::julian;

/// A convention for where the year begins and how it is numbered.
///
/// A plain struct rather than an enum: the set of local styles is open —
/// Cappelli alone tabulates dozens of Italian cities — and a caller with a
/// convention this table has never heard of should be able to write it down
/// and convert with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct YearStyle {
    /// A stable identifier.
    pub id: &'static str,
    /// The name the literature uses.
    pub english_name: &'static str,
    /// The month the year begins in.
    pub start_month: u8,
    /// The day of the month the year begins on.
    pub start_day: u8,
    /// What to add to the modern year for days on or after the start.
    pub offset_from_start: i64,
    /// What to add to the modern year for days before the start.
    pub offset_before_start: i64,
    /// Who used it, and where it is tabulated.
    pub authority: &'static str,
}

impl YearStyle {
    /// The year number a scribe using this style would write for `rd`.
    ///
    /// # Errors
    ///
    /// Propagates whatever [`crate::julian`] says about the day.
    pub fn year_of(&self, rd: Rd) -> CalendarResult<i64> {
        let (year, month, day) = julian::from_fixed(rd)?;
        Ok(year + self.offset_for(month, day))
    }

    /// The offset this style applies to a month and day of the modern year.
    const fn offset_for(&self, month: u8, day: u8) -> i64 {
        if month > self.start_month || (month == self.start_month && day >= self.start_day) {
            self.offset_from_start
        } else {
            self.offset_before_start
        }
    }

    /// The fixed day meant by a dateline written in this style.
    ///
    /// `year` is the number on the document; `month` and `day` are the
    /// Julian month and day, which no style alters.
    ///
    /// # Errors
    ///
    /// Propagates [`crate::julian`]'s errors for a date that does not exist.
    pub fn to_fixed(&self, year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
        if month == 0 || month > 12 {
            return Err(CalendarError::MonthOutOfRange);
        }
        julian::to_fixed(year - self.offset_for(month, day), month, day)
    }

    /// The first and last day of the style's year `year`.
    ///
    /// # Errors
    ///
    /// Propagates [`crate::julian`]'s errors.
    pub fn span(&self, year: i64) -> CalendarResult<(Rd, Rd)> {
        let first = self.to_fixed(year, self.start_month, self.start_day)?;
        let next = self.to_fixed(year + 1, self.start_month, self.start_day)?;
        Ok((first, Rd(next.0 - 1)))
    }

    /// Whether this style agrees with modern 1 January reckoning for `rd`.
    ///
    /// # Errors
    ///
    /// Propagates [`crate::julian`]'s errors.
    pub fn agrees_with_modern(&self, rd: Rd) -> CalendarResult<bool> {
        let (year, _, _) = julian::from_fixed(rd)?;
        Ok(self.year_of(rd)? == year)
    }
}

hc_core::catalogue! {
    type: YearStyle,
    id: |entry| entry.id,
    provenance: |entry| entry.authority,
    tests: year_style_catalogue,

    /// Every style in this module.
    pub const ALL;

    /// The style with this id, if this module has one.
    pub fn by_id;

    entries: {
    /// The modern style: the year begins on 1 January and is numbered as we
    /// number it.
    ///
    /// Present so that "no style" is a style with a name, and so that a caller
    /// switching between conventions has something to switch *from*.
    pub const CIRCUMCISION = YearStyle {
        id: "circumcision",
        english_name: "Circumcision style (1 January)",
        start_month: 1,
        start_day: 1,
        offset_from_start: 0,
        offset_before_start: 0,
        authority: "Modern reckoning; the Feast of the Circumcision",
    };

    /// The Annunciation style as Florence used it: the year begins on 25 March
    /// and runs a year *behind* the modern one until then.
    ///
    /// So 1 February 1200 Florentine is 1 February 1201 modern.
    pub const ANNUNCIATION_FLORENTINE = YearStyle {
        id: "annunciation-florentine",
        english_name: "Annunciation style, Florentine (25 March, behind)",
        start_month: 3,
        start_day: 25,
        offset_from_start: 0,
        offset_before_start: -1,
        authority: "Cappelli, Cronologia [cappelli1998]; the Florentine stile dell'Incarnazione",
    };

    /// The Annunciation style as Pisa used it: the year begins on 25 March and
    /// runs a year *ahead* of the modern one from then.
    ///
    /// So 1 April 1200 modern is 1 April 1201 Pisan. Pisa and Florence are
    /// forty miles apart and their datelines differ by a year for most of it.
    pub const ANNUNCIATION_PISAN = YearStyle {
        id: "annunciation-pisan",
        english_name: "Annunciation style, Pisan (25 March, ahead)",
        start_month: 3,
        start_day: 25,
        offset_from_start: 1,
        offset_before_start: 0,
        authority: "Cappelli, Cronologia [cappelli1998]; the Pisan stile dell'Incarnazione",
    };

    /// The English legal year, which began on 25 March to 1751.
    ///
    /// The same arithmetic as the Florentine style, under the name English
    /// records use. A document dated 12 February 1721 means 1722. The
    /// Calendar (New Style) Act 1750 began the year 1752 on 1 January, so
    /// 1751, which began on 25 March and ended on 31 December, is the last
    /// year of the style (`uk-calendar-act-1750`).
    pub const LADY_DAY = YearStyle {
        id: "lady-day",
        english_name: "Lady Day style (25 March, behind)",
        start_month: 3,
        start_day: 25,
        offset_from_start: 0,
        offset_before_start: -1,
        authority: "Cheney, Handbook of Dates [cheney2000]; the English legal year to 1751, \
            ended by the Calendar (New Style) Act 1750 [uk-calendar-act-1750]",
    };

    /// The Nativity style: the year begins on 25 December, a week early.
    ///
    /// Used by the papal chancery for long stretches and widely in Germany. So
    /// 28 December 1200 modern is 28 December 1201 in this style.
    pub const NATIVITY = YearStyle {
        id: "nativity",
        english_name: "Nativity style (25 December)",
        start_month: 12,
        start_day: 25,
        offset_from_start: 1,
        offset_before_start: 0,
        authority: "Grotefend, Zeitrechnung [grotefend1891]; the stilus nativitatis",
    };

    /// The Venetian *more veneto*: the year begins on 1 March.
    ///
    /// January and February belong to the year before, so 1 February 1500
    /// *more veneto* is 1 February 1501 modern.
    pub const MORE_VENETO = YearStyle {
        id: "more-veneto",
        english_name: "More veneto (1 March)",
        start_month: 3,
        start_day: 1,
        offset_from_start: 0,
        offset_before_start: -1,
        authority: "Cappelli, Cronologia [cappelli1998]; the Venetian chancery",
    };

    /// The Greek or Constantinopolitan style: the year begins on 1 September.
    ///
    /// The Byzantine civil year, and the one the indiction turns on. September
    /// to December carry the next year's number.
    pub const SEPTEMBER = YearStyle {
        id: "september",
        english_name: "Greek style (1 September)",
        start_month: 9,
        start_day: 1,
        offset_from_start: 1,
        offset_before_start: 0,
        authority: "Grumel, La chronologie [grumel1958]; the Byzantine civil year",
    };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The example `julian_gregorian` gives in its own doc comment, which
    /// is why this module exists.
    #[test]
    fn an_english_document_dated_february_seventeen_twenty_one_means_seventeen_twenty_two() {
        let day = LADY_DAY.to_fixed(1721, 2, 12).expect("a real date");
        assert_eq!(julian::from_fixed(day), Ok((1722, 2, 12)));
        assert_eq!(LADY_DAY.year_of(day), Ok(1721));
        assert_eq!(CIRCUMCISION.year_of(day), Ok(1722));
    }

    /// Forty miles apart, a year apart, in opposite directions.
    #[test]
    fn pisa_and_florence_disagree_with_each_other_and_with_us() {
        // A day after 25 March, when both styles are in their "from start"
        // half.
        let day = julian::to_fixed(1200, 6, 1).expect("a real date");
        assert_eq!(CIRCUMCISION.year_of(day), Ok(1200));
        assert_eq!(ANNUNCIATION_FLORENTINE.year_of(day), Ok(1200));
        assert_eq!(ANNUNCIATION_PISAN.year_of(day), Ok(1201));

        // A day before it, when Florence is behind and Pisa has caught up.
        let day = julian::to_fixed(1200, 2, 1).expect("a real date");
        assert_eq!(CIRCUMCISION.year_of(day), Ok(1200));
        assert_eq!(ANNUNCIATION_FLORENTINE.year_of(day), Ok(1199));
        assert_eq!(ANNUNCIATION_PISAN.year_of(day), Ok(1200));

        // So for most of the year the two cities' datelines differ by one,
        // and for the rest they agree — which is the trap.
        let summer = julian::to_fixed(1200, 8, 1).expect("a real date");
        assert_ne!(
            ANNUNCIATION_FLORENTINE.year_of(summer),
            ANNUNCIATION_PISAN.year_of(summer)
        );
        let winter = julian::to_fixed(1200, 1, 5).expect("a real date");
        assert_eq!(
            ANNUNCIATION_FLORENTINE.year_of(winter).expect("in range") + 1,
            ANNUNCIATION_PISAN.year_of(winter).expect("in range")
        );
    }

    #[test]
    fn the_nativity_style_turns_the_year_a_week_early() {
        let christmas = julian::to_fixed(1200, 12, 25).expect("a real date");
        assert_eq!(NATIVITY.year_of(christmas), Ok(1201));
        assert_eq!(CIRCUMCISION.year_of(christmas), Ok(1200));

        let eve = Rd(christmas.0 - 1);
        assert_eq!(NATIVITY.year_of(eve), Ok(1200));
    }

    #[test]
    fn more_veneto_puts_january_in_the_year_before() {
        let day = julian::to_fixed(1501, 2, 1).expect("a real date");
        assert_eq!(MORE_VENETO.year_of(day), Ok(1500));
        let day = julian::to_fixed(1501, 3, 1).expect("a real date");
        assert_eq!(MORE_VENETO.year_of(day), Ok(1501));
    }

    #[test]
    fn the_greek_style_turns_on_the_first_of_september() {
        let august = julian::to_fixed(1000, 8, 31).expect("a real date");
        let september = julian::to_fixed(1000, 9, 1).expect("a real date");
        assert_eq!(SEPTEMBER.year_of(august), Ok(1000));
        assert_eq!(SEPTEMBER.year_of(september), Ok(1001));
    }

    /// Every style must be its own inverse: what it writes for a day, it
    /// reads back as that day.
    #[test]
    fn every_style_round_trips_every_day_of_four_centuries() {
        let start = julian::to_fixed(1200, 1, 1).expect("a real date");
        let end = julian::to_fixed(1600, 1, 1).expect("a real date");
        for style in ALL {
            for rd in (start.0..end.0).step_by(7) {
                let rd = Rd(rd);
                let (_, month, day) = julian::from_fixed(rd).expect("in range");
                let written = style.year_of(rd).expect("in range");
                assert_eq!(
                    style.to_fixed(written, month, day),
                    Ok(rd),
                    "{} on {rd}",
                    style.english_name
                );
            }
        }
    }

    /// A style's year is 365 or 366 days long, like any other — the styles
    /// here move the boundary, they do not change the length. (The Easter
    /// style does, which is why it is not here.)
    #[test]
    fn every_style_year_is_an_ordinary_length() {
        for style in ALL {
            for year in 1200..1400 {
                let (first, last) = style.span(year).expect("in range");
                let length = last.0 - first.0 + 1;
                assert!(
                    length == 365 || length == 366,
                    "{} year {year} was {length} days",
                    style.english_name
                );
            }
        }
    }

    #[test]
    fn a_style_says_when_it_agrees_with_modern_reckoning() {
        // The modern style always agrees; the others agree for part of the
        // year and not the rest.
        for year in 1200..1300 {
            let day = julian::to_fixed(year, 7, 1).expect("a real date");
            assert_eq!(CIRCUMCISION.agrees_with_modern(day), Ok(true));
        }
        let summer = julian::to_fixed(1250, 7, 1).expect("a real date");
        assert_eq!(ANNUNCIATION_PISAN.agrees_with_modern(summer), Ok(false));
        assert_eq!(ANNUNCIATION_FLORENTINE.agrees_with_modern(summer), Ok(true));
    }

    #[test]
    fn a_month_outside_the_year_is_refused() {
        assert_eq!(
            LADY_DAY.to_fixed(1721, 13, 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            LADY_DAY.to_fixed(1721, 0, 1),
            Err(CalendarError::MonthOutOfRange)
        );
    }
}
