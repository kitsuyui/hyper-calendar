//! The amānta month engine the Hindu lunisolar calendars share.
//!
//! A month runs from conjunction to conjunction and begins with the first
//! day whose sunrise follows the conjunction; it is named for the
//! saṅkrānti it holds, intercalary when it holds none and kṣaya when it
//! holds two; a day carries the tithi in progress at its sunrise. The rules
//! are [`crate::hindu_lunar`]'s, written up in
//! `docs/systems/hindu-calendars.md`, and they do not depend on whose Sun
//! and Moon are read: [`crate::hindu_lunar::HinduLunarCalendar`] reads the
//! true Sun and Moon of modern astronomy in an ayanāṃśa's zodiac, and
//! [`crate::hindu_lunar_siddhanta::SiddhantaLunarCalendar`] the *Sūrya
//! Siddhānta*'s. Each supplies a [`Sky`], and [`Amanta`] is the one
//! implementation of the months over either.

use hc_calendar::fixed::Moment;
use hc_calendar::{CalendarError, CalendarResult, Rd};
use hc_seasons::zodiac::SiderealSign;

use crate::hindu_lunar::{HinduLunarDate, MONTHS_IN_YEAR};
use crate::tithi::TITHIS_PER_MONTH;

/// What an amānta calendar reads from the sky: where its conjunctions and
/// saṅkrāntis fall, whose sunrise names the day, and how its years are
/// counted and bounded.
pub(crate) trait Sky: Copy {
    /// The first conjunction at or after a moment: the instant the Moon's
    /// elongation from the Sun, the quantity the tithi is counted in,
    /// returns to zero.
    fn conjunction_at_or_after(&self, moment: Moment) -> Moment;

    /// The sidereal sign the Sun stands in at a moment.
    fn sign_at(&self, moment: Moment) -> SiderealSign;

    /// The first moment at or after `moment` at which the Sun enters
    /// `sign`.
    fn ingress_after(&self, sign: SiderealSign, moment: Moment) -> Moment;

    /// Sunrise on a day, in Universal Time.
    fn sunrise(&self, day: Rd) -> Moment;

    /// The tithi a day carries: the one in progress at its sunrise.
    fn tithi_of_day(&self, day: Rd) -> u8;

    /// The Śaka year of a month numbered `month` that begins after the
    /// conjunction `start`.
    fn year_of(&self, start: Moment, month: u8) -> i64;

    /// A moment less than a month before the saṅkrānti that names `month`
    /// of the Śaka `year` — the Meṣa saṅkrānti for Chaitra — from which
    /// [`Sky::ingress_after`] finds it.
    fn sankranti_search_start(&self, year: i64, month: u8) -> Moment;

    /// The Śaka years the calendar converts.
    fn years(&self) -> (i64, i64);

    /// The first and last days the calendar converts, where they are
    /// written down rather than searched for.
    fn named_range(&self) -> Option<(Rd, Rd)>;

    /// The zodiac as words of a [`hc_core::memo`] key: the part of the sky
    /// the months depend on and the place does not.
    fn zodiac_key(&self) -> [u64; 2];

    /// The place as words of a [`hc_core::memo`] key.
    fn place_key(&self) -> [u64; 3];
}

/// The last conjunction strictly before a moment.
fn conjunction_before<S: Sky>(sky: &S, moment: Moment) -> Moment {
    // A synodic month is under thirty days, so a search from 31 days back
    // finds the conjunction before or the one before that.
    let mut found = sky.conjunction_at_or_after(Moment(moment.0 - 31.0));
    loop {
        let next = sky.conjunction_at_or_after(Moment(found.0 + 1.0));
        if next.0 >= moment.0 {
            return found;
        }
        found = next;
    }
}

/// One lunar month, as the calendar sees it: the two conjunctions that
/// bound it and the name the saṅkrāntis between them give it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct LunarMonth {
    /// The conjunction the month begins after.
    pub(crate) start: Moment,
    /// The conjunction the month ends with.
    pub(crate) end: Moment,
    /// The month's number, 1 to 12.
    pub(crate) month: u8,
    /// Whether the month is intercalary.
    pub(crate) leap: bool,
    /// Whether the month holds two saṅkrāntis and has lost a name.
    pub(crate) kshaya: bool,
    /// The Śaka year the month belongs to.
    pub(crate) year: i64,
}

/// The amānta months over a [`Sky`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Amanta<S>(pub(crate) S);

impl<S: Sky> Amanta<S> {
    /// The month number a saṅkrānti into `sign` gives: Meṣa's is Chaitra.
    const fn month_of_sign(sign: SiderealSign) -> u8 {
        sign.index() + 1
    }

    /// Whether a Śaka year is one the calendar converts.
    fn converts(&self, year: i64) -> bool {
        let (first, last) = self.0.years();
        (first..=last).contains(&year)
    }

    /// The lunar month bounded by the conjunction before `moment` and the
    /// one after, named and placed in its year.
    pub(crate) fn month_containing(&self, moment: Moment) -> LunarMonth {
        let start = conjunction_before(&self.0, moment);
        self.month_from(start)
    }

    /// The lunar month beginning after the conjunction `start`.
    ///
    /// The place plays no part: the conjunctions and saṅkrāntis are the
    /// same instants everywhere. So inside a [`hc_core::memo::scope`] a
    /// month is found once for every calendar with the same zodiac,
    /// wherever its sunrise is read.
    pub(crate) fn month_from(&self, start: Moment) -> LunarMonth {
        enum MonthFrom {}
        let [anchor, degrees] = self.0.zodiac_key();
        hc_core::memo::cached::<MonthFrom, _, 3>([anchor, degrees, start.0.to_bits()], || {
            self.computed_month_from(start)
        })
    }

    /// [`Amanta::month_from`], found.
    fn computed_month_from(&self, start: Moment) -> LunarMonth {
        let end = self.0.conjunction_at_or_after(Moment(start.0 + 1.0));
        let sign_at_start = self.0.sign_at(start);
        let sign_at_end = self.0.sign_at(end);
        let crossings =
            (i16::from(sign_at_end.index()) - i16::from(sign_at_start.index())).rem_euclid(12);
        let (month, leap, kshaya) = match crossings {
            0 => (Self::month_of_sign(sign_at_start.next()), true, false),
            1 => (Self::month_of_sign(sign_at_end), false, false),
            _ => (Self::month_of_sign(sign_at_start.next()), false, true),
        };
        let year = self.0.year_of(start, month);
        LunarMonth {
            start,
            end,
            month,
            leap,
            kshaya,
            year,
        }
    }

    /// The calendar and one more word as a [`hc_core::memo`] key: its place
    /// and its zodiac, bit for bit, then `word`.
    fn key_with(&self, word: u64) -> [u64; 6] {
        let [latitude, longitude, elevation] = self.0.place_key();
        let [anchor, degrees] = self.0.zodiac_key();
        [latitude, longitude, elevation, anchor, degrees, word]
    }

    /// The label — month number and intercalary flag — of the month after
    /// the one containing `day`, for the pūrṇimānta renaming; inside a
    /// [`hc_core::memo::scope`], once a day.
    pub(crate) fn next_month_label(&self, day: Rd) -> (u8, bool) {
        enum NextMonthLabel {}
        hc_core::memo::cached::<NextMonthLabel, _, 6>(self.key_with(day.0 as u64), || {
            self.computed_next_month_label(day)
        })
    }

    /// [`Amanta::next_month_label`], computed.
    fn computed_next_month_label(&self, day: Rd) -> (u8, bool) {
        let current = self.month_containing(self.0.sunrise(day));
        let next = self.month_from(current.end);
        (next.month, next.leap)
    }

    /// The first day of a month: the first day whose sunrise follows the
    /// conjunction the month begins after.
    ///
    /// The search starts the day before the conjunction's date in Universal
    /// Time and walks forward. It cannot stop at the day after: east of
    /// Greenwich the local date runs ahead, and a conjunction late in the
    /// Universal day can fall after the next local sunrise too. At the
    /// Central Station on 11 June 2002 the new moon came at 05:17 Indian
    /// Standard Time and the sun had risen at 05:13, so the month began on
    /// the 12th.
    pub(crate) fn first_day_of(&self, month: LunarMonth) -> Rd {
        let mut day = Rd(month.start.day().0 - 1);
        while self.0.sunrise(day).0 <= month.start.0 {
            day = Rd(day.0 + 1);
        }
        day
    }

    /// The months of a Śaka year, in order: twelve or thirteen of them.
    ///
    /// The year opens with its first month named Chaitra — the intercalary
    /// one if there is one — and runs to the month before the next.
    fn months_of_year(&self, year: i64) -> MonthsOfYear<S> {
        // The ordinary Chaitra is the month holding the year's Meṣa
        // saṅkrānti; an intercalary Chaitra would be the month before it.
        let mesha = self
            .0
            .ingress_after(SiderealSign::MESHA, self.0.sankranti_search_start(year, 1));
        let chaitra = self.month_containing(mesha);
        let before = self.month_from(conjunction_before(&self.0, Moment(chaitra.start.0 - 1.0)));
        let first = if before.month == 1 && before.leap {
            before
        } else {
            chaitra
        };
        MonthsOfYear {
            calendar: *self,
            current: Some(first),
            year,
        }
    }

    /// The month of `year` numbered `month`, intercalary or not.
    ///
    /// The ordinary month is the one holding the saṅkrānti that names it —
    /// Meṣa's for Chaitra; the intercalary month of the same name is the
    /// one before it, when that one has no saṅkrānti. A kṣaya month is the
    /// one case the saṅkrānti does not land in a month of its own name, and
    /// it is reported as the name not existing.
    fn find_month(&self, year: i64, month: u8, leap: bool) -> CalendarResult<LunarMonth> {
        if !self.converts(year) {
            return Err(CalendarError::YearOutOfRange);
        }
        if month == 0 || month > MONTHS_IN_YEAR {
            return Err(CalendarError::MonthOutOfRange);
        }
        let Some(sign) = SiderealSign::from_index(month - 1) else {
            return Err(CalendarError::MonthOutOfRange);
        };
        let sankranti = self
            .0
            .ingress_after(sign, self.0.sankranti_search_start(year, month));
        let ordinary = self.month_containing(sankranti);
        if ordinary.month != month || ordinary.year != year {
            return Err(CalendarError::MonthOutOfRange);
        }
        if !leap {
            return Ok(ordinary);
        }
        let before = self.month_from(conjunction_before(&self.0, Moment(ordinary.start.0 - 1.0)));
        if before.month == month && before.leap && before.year == year {
            Ok(before)
        } else {
            Err(CalendarError::MonthOutOfRange)
        }
    }

    /// The day of a month carrying tithi `day` at sunrise — the second such
    /// day when `leap_day` — or `None` when the tithi holds no sunrise.
    fn day_in_month(&self, month: LunarMonth, day: u8, leap_day: bool) -> Option<Rd> {
        let first = self.first_day_of(month);
        let next_first = self.first_day_of(self.month_from(month.end));
        // Tithis average a little under a day, so tithi `day` falls on or a
        // day or two before the day numbered `day`; start three days early
        // and walk forward until the tithi is passed.
        let mut candidate = Rd(first.0 + i64::from(day) - 4);
        if candidate < first {
            candidate = first;
        }
        let mut found = None;
        while candidate < next_first {
            let tithi = self.0.tithi_of_day(candidate);
            if tithi == day {
                if !leap_day {
                    return Some(candidate);
                }
                if found.is_some() {
                    return Some(candidate);
                }
                found = Some(candidate);
            } else if tithi > day {
                // Passed it: a skipped tithi, or no second day for a
                // repeated one.
                return None;
            }
            candidate = Rd(candidate.0 + 1);
        }
        None
    }

    /// The fixed day of a date; see the calendars' `to_fixed`.
    pub(crate) fn fixed_of(&self, date: HinduLunarDate) -> CalendarResult<Rd> {
        enum ToFixed {}
        let mut key = [0; 8];
        key[..6].copy_from_slice(&self.key_with(date.year as u64));
        key[6] = u64::from(date.month) << 8 | u64::from(date.day);
        key[7] = u64::from(date.leap_month) << 1 | u64::from(date.leap_day);
        hc_core::memo::cached::<ToFixed, _, 8>(key, || self.computed_to_fixed(date))
    }

    /// [`Amanta::fixed_of`], computed.
    fn computed_to_fixed(&self, date: HinduLunarDate) -> CalendarResult<Rd> {
        if date.day == 0 || date.day > TITHIS_PER_MONTH {
            return Err(CalendarError::DayOutOfRange);
        }
        let month = self.find_month(date.year, date.month, date.leap_month)?;
        self.day_in_month(month, date.day, date.leap_day)
            .ok_or(CalendarError::DayOutOfRange)
    }

    /// The date of a fixed day; see the calendars' `from_fixed`.
    pub(crate) fn date_of(&self, rd: Rd) -> CalendarResult<HinduLunarDate> {
        enum FromFixed {}
        hc_core::memo::cached::<FromFixed, _, 6>(self.key_with(rd.0 as u64), || {
            self.computed_from_fixed(rd)
        })
    }

    /// [`Amanta::date_of`], computed.
    fn computed_from_fixed(&self, rd: Rd) -> CalendarResult<HinduLunarDate> {
        let sunrise = self.0.sunrise(rd);
        let month = self.month_containing(sunrise);
        let (first_year, last_year) = self.0.years();
        if month.year < first_year {
            return Err(CalendarError::BeforeEpoch);
        }
        if month.year > last_year {
            return Err(CalendarError::AfterSupportedRange);
        }
        let day = self.0.tithi_of_day(rd);
        let first = self.first_day_of(month);
        let leap_day = rd > first && self.0.tithi_of_day(Rd(rd.0 - 1)) == day;
        Ok(HinduLunarDate {
            year: month.year,
            month: month.month,
            leap_month: month.leap,
            day,
            leap_day,
        })
    }

    /// The days of a month: its first day, and the day after its last.
    pub(crate) fn month_span(&self, year: i64, month: u8, leap: bool) -> CalendarResult<(Rd, Rd)> {
        let found = self.find_month(year, month, leap)?;
        let first = self.first_day_of(found);
        let next = self.first_day_of(self.month_from(found.end));
        Ok((first, next))
    }

    /// The first day of Chaitra of a Śaka year.
    pub(crate) fn new_year(&self, year: i64) -> CalendarResult<Rd> {
        if !self.converts(year) {
            return Err(CalendarError::YearOutOfRange);
        }
        let first = self
            .months_of_year(year)
            .next()
            .ok_or(CalendarError::YearOutOfRange)?;
        Ok(self.first_day_of(first))
    }

    /// The intercalary month of a Śaka year, if it has one, as its number
    /// and its first and last days; inside a [`hc_core::memo::scope`] each
    /// year is searched once.
    pub(crate) fn leap_month_of(&self, year: i64) -> CalendarResult<Option<(u8, Rd, Rd)>> {
        enum LeapMonthOf {}
        hc_core::memo::cached::<LeapMonthOf, _, 6>(self.key_with(year as u64), || {
            self.computed_leap_month_of(year)
        })
    }

    /// [`Amanta::leap_month_of`], searched for.
    fn computed_leap_month_of(&self, year: i64) -> CalendarResult<Option<(u8, Rd, Rd)>> {
        if !self.converts(year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(self
            .months_of_year(year)
            .find(|month| month.leap)
            .map(|month| {
                let first = self.first_day_of(month);
                let last = Rd(self.first_day_of(self.month_from(month.end)).0 - 1);
                (month.month, first, last)
            }))
    }

    /// Whether a Śaka year has a kṣaya month.
    pub(crate) fn has_kshaya_month(&self, year: i64) -> CalendarResult<bool> {
        if !self.converts(year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(self.months_of_year(year).any(|month| month.kshaya))
    }

    /// The number of days in a Śaka year.
    pub(crate) fn days_in_year(&self, year: i64) -> CalendarResult<u16> {
        let this = self.new_year(year)?;
        let next = self.new_year(year + 1)?;
        Ok((next.0 - this.0) as u16)
    }

    /// The earliest fixed day the calendar converts.
    pub(crate) fn earliest(&self) -> CalendarResult<Rd> {
        match self.0.named_range() {
            Some((earliest, _)) => Ok(earliest),
            None => self.computed_earliest(),
        }
    }

    /// [`Amanta::earliest`] as the sky finds it: Chaitra śukla 1 of the
    /// first year.
    pub(crate) fn computed_earliest(&self) -> CalendarResult<Rd> {
        self.new_year(self.0.years().0)
    }

    /// The latest fixed day the calendar converts.
    pub(crate) fn latest(&self) -> CalendarResult<Rd> {
        match self.0.named_range() {
            Some((_, latest)) => Ok(latest),
            None => self.computed_latest(),
        }
    }

    /// [`Amanta::latest`] as the sky finds it: the day before Chaitra
    /// śukla 1 of the year after the last.
    pub(crate) fn computed_latest(&self) -> CalendarResult<Rd> {
        let first = self
            .months_of_year(self.0.years().1 + 1)
            .next()
            .ok_or(CalendarError::YearOutOfRange)?;
        Ok(Rd(self.first_day_of(first).0 - 1))
    }
}

/// The months of one Śaka year, in order.
struct MonthsOfYear<S> {
    calendar: Amanta<S>,
    current: Option<LunarMonth>,
    year: i64,
}

impl<S: Sky> Iterator for MonthsOfYear<S> {
    type Item = LunarMonth;

    fn next(&mut self) -> Option<LunarMonth> {
        let month = self.current?;
        let following = self.calendar.month_from(month.end);
        // The year ends where the next Chaitra begins.
        self.current = if following.month == 1 {
            None
        } else {
            Some(following)
        };
        if month.year == self.year {
            Some(month)
        } else {
            None
        }
    }
}

/// The public methods an amānta calendar offers, each the engine's over
/// the calendar's own [`Sky`], written once for every calendar type that
/// has one.
macro_rules! amanta_methods {
    () => {
        /// The fixed day of a date.
        ///
        /// # Errors
        ///
        /// Returns [`CalendarError::YearOutOfRange`](hc_calendar::CalendarError::YearOutOfRange) outside the years the
        /// calendar converts; [`CalendarError::MonthOutOfRange`](hc_calendar::CalendarError::MonthOutOfRange) for a
        /// month number outside 1–12, for an intercalary month the year
        /// does not have, or for a name a kṣaya month lost; and
        /// [`CalendarError::DayOutOfRange`](hc_calendar::CalendarError::DayOutOfRange) for a tithi outside 1–30, a
        /// tithi the month skips, or a repeated tithi the month does not
        /// repeat.
        ///
        /// Inside a [`hc_core::memo::scope`] each date is converted once:
        /// the calendars built on this one convert the same dates.
        pub fn to_fixed(&self, date: HinduLunarDate) -> CalendarResult<Rd> {
            crate::amanta::Amanta(*self).fixed_of(date)
        }

        /// The date of a fixed day.
        ///
        /// # Errors
        ///
        /// Returns [`CalendarError::BeforeEpoch`](hc_calendar::CalendarError::BeforeEpoch) or
        /// [`CalendarError::AfterSupportedRange`](hc_calendar::CalendarError::AfterSupportedRange) outside the years this
        /// calendar converts.
        ///
        /// Inside a [`hc_core::memo::scope`] each day is converted once:
        /// the calendars built on this one convert the same days.
        pub fn from_fixed(&self, rd: Rd) -> CalendarResult<HinduLunarDate> {
            crate::amanta::Amanta(*self).date_of(rd)
        }

        /// The days of a month: its first day, and the day after its last.
        ///
        /// # Errors
        ///
        /// As `to_fixed`: the year out of range, the month number outside
        /// 1–12, an intercalary month the year does not have, or a name a
        /// kṣaya month lost.
        pub fn month_span(&self, year: i64, month: u8, leap: bool) -> CalendarResult<(Rd, Rd)> {
            crate::amanta::Amanta(*self).month_span(year, month, leap)
        }

        /// The first day of Chaitra — Chaitra śukla pratipadā, the new
        /// year — of a Śaka year. When the year opens with an intercalary
        /// Chaitra, this is that month's first day.
        ///
        /// # Errors
        ///
        /// Returns [`CalendarError::YearOutOfRange`](hc_calendar::CalendarError::YearOutOfRange) outside the years the
        /// calendar converts.
        pub fn new_year(&self, year: i64) -> CalendarResult<Rd> {
            crate::amanta::Amanta(*self).new_year(year)
        }

        /// The intercalary month of a Śaka year, if it has one, as its
        /// number and its first and last days.
        ///
        /// # Errors
        ///
        /// Returns [`CalendarError::YearOutOfRange`](hc_calendar::CalendarError::YearOutOfRange) outside the years the
        /// calendar converts.
        ///
        /// Inside a [`hc_core::memo::scope`] each year is searched once.
        pub fn leap_month_of(&self, year: i64) -> CalendarResult<Option<(u8, Rd, Rd)>> {
            crate::amanta::Amanta(*self).leap_month_of(year)
        }

        /// Whether a Śaka year has a kṣaya month — one that lost its name
        /// to a second saṅkrānti.
        ///
        /// # Errors
        ///
        /// Returns [`CalendarError::YearOutOfRange`](hc_calendar::CalendarError::YearOutOfRange) outside the years the
        /// calendar converts.
        pub fn has_kshaya_month(&self, year: i64) -> CalendarResult<bool> {
            crate::amanta::Amanta(*self).has_kshaya_month(year)
        }

        /// The number of days in a Śaka year.
        ///
        /// # Errors
        ///
        /// Returns [`CalendarError::YearOutOfRange`](hc_calendar::CalendarError::YearOutOfRange) outside the years the
        /// calendar converts.
        pub fn days_in_year(&self, year: i64) -> CalendarResult<u16> {
            crate::amanta::Amanta(*self).days_in_year(year)
        }

        /// The earliest fixed day this calendar converts: Chaitra śukla 1
        /// of its first year.
        ///
        /// # Errors
        ///
        /// Returns an error only if the sky cannot place the year, which it
        /// can.
        pub fn earliest(&self) -> CalendarResult<Rd> {
            crate::amanta::Amanta(*self).earliest()
        }

        /// The latest fixed day this calendar converts: the day before
        /// Chaitra śukla 1 of the year after its last.
        ///
        /// # Errors
        ///
        /// Returns an error only if the sky cannot place the year, which it
        /// can.
        pub fn latest(&self) -> CalendarResult<Rd> {
            crate::amanta::Amanta(*self).latest()
        }
    };
}

pub(crate) use amanta_methods;
