//! The Kārtikādi year over the amānta months: the arithmetic shared by the
//! eras that open their year at Kārtika śukla pratipadā, the day after
//! Dīpāvalī — Nepal Sambat ([`crate::nepal_sambat`]) and the Vira Nirvana
//! Samvat ([`crate::vira_nirvana`]).
//!
//! Such an era renames nothing but the year and the order of the months:
//! its month 1 is Kārtika, amānta month 8, and its month 12 Āśvina, amānta
//! month 7, and a date keeps its tithi, its fortnight and its intercalary
//! or repeated day exactly as [`crate::hindu_lunar`] has them. What differs
//! between the eras is one number, how far the Śaka year of Kārtika to
//! Phālguna is from the era's year, and the place whose sunrise reads the
//! day, which is the amānta calendar's.

use hc_calendar::{CalendarResult, Rd};

use crate::hindu_lunar::HinduLunarCalendar;

/// The amānta month, 1 for Chaitra, that opens a Kārtikādi year: Kārtika.
pub(crate) const KARTIKA: u8 = 8;

/// The amānta month, 1 for Chaitra, of a Kārtikādi month, 1 for Kārtika.
pub(crate) const fn amanta_month(month: u8) -> u8 {
    (month + 6) % 12 + 1
}

/// The Kārtikādi month, 1 for Kārtika, of an amānta month, 1 for Chaitra.
pub(crate) const fn kartikadi_month(amanta: u8) -> u8 {
    (amanta + 4) % 12 + 1
}

/// The Śaka year of an amānta month in a Kārtikādi year, where `offset` is
/// the Śaka year less the era's year for Kārtika to Phālguna; Chaitra to
/// Āśvina are a Śaka year later.
pub(crate) const fn saka_year(year: i64, amanta: u8, offset: i64) -> i64 {
    if amanta >= KARTIKA {
        year + offset
    } else {
        year + offset + 1
    }
}

/// The era's year of an amānta month in a Śaka year: the inverse of
/// [`saka_year`].
pub(crate) const fn era_year(saka: i64, amanta: u8, offset: i64) -> i64 {
    if amanta >= KARTIKA {
        saka - offset
    } else {
        saka - offset - 1
    }
}

/// New Year's Day of an era's year: the first day of its first Kārtika —
/// the intercalary one, in a year that has it.
///
/// # Errors
///
/// [`hc_calendar::CalendarError::YearOutOfRange`] outside the amānta
/// engine's range.
pub(crate) fn new_year(lunar: &HinduLunarCalendar, year: i64, offset: i64) -> CalendarResult<Rd> {
    let saka = year + offset;
    lunar
        .month_span(saka, KARTIKA, true)
        .or_else(|_| lunar.month_span(saka, KARTIKA, false))
        .map(|(first, _)| first)
}

/// Whether an era's year has an adhika māsa. The year runs from Kārtika of
/// one Śaka year into the next, so the month may fall in either.
///
/// # Errors
///
/// [`hc_calendar::CalendarError::YearOutOfRange`] outside the amānta
/// engine's range.
pub(crate) fn is_leap_year(
    lunar: &HinduLunarCalendar,
    year: i64,
    offset: i64,
) -> CalendarResult<bool> {
    let saka = year + offset;
    let autumn = lunar
        .leap_month_of(saka)?
        .is_some_and(|(month, _, _)| month >= KARTIKA);
    let spring = lunar
        .leap_month_of(saka + 1)?
        .is_some_and(|(month, _, _)| month < KARTIKA);
    Ok(autumn || spring)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_months_turn_at_kartika_and_back() {
        assert_eq!(amanta_month(1), KARTIKA);
        assert_eq!(amanta_month(5), 12);
        assert_eq!(amanta_month(6), 1);
        assert_eq!(amanta_month(12), 7);
        for month in 1..=12 {
            assert_eq!(kartikadi_month(amanta_month(month)), month);
        }
    }

    #[test]
    fn the_year_steps_at_chaitra_against_the_saka_year() {
        for amanta in 1..=12 {
            let saka = saka_year(2_551, amanta, -605);
            assert_eq!(era_year(saka, amanta, -605), 2_551);
        }
        assert_eq!(saka_year(2_551, KARTIKA, -605), 1_946);
        assert_eq!(saka_year(2_551, 1, -605), 1_947);
    }
}
