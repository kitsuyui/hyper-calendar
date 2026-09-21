//! 六曜 — the six-day cycle printed on Japanese calendars.
//!
//! The rule is one line: `(lunisolar month + lunisolar day) mod 6`, with 0
//! landing on 大安. The first day of the first month is therefore always
//! 先勝, the first of the second month always 友引, and so on: the cycle
//! restarts at every new moon, which is why a Japanese calendar's 六曜 column
//! jumps a step at apparently random intervals. It is not a weekday.
//!
//! The consequence people notice is 仏滅 and 大安: wedding halls charge more
//! on 大安 and funeral parlours close on 友引, so a library that gets this
//! wrong is wrong about something with money attached.
//!
//! # The dependency this module should not have
//!
//! 六曜 is a function of the lunisolar month and day, so it needs a lunisolar
//! calendar. `hc-calendars-lunar` is the crate that will own one; until it
//! exists, [`crate::lunisolar`] carries a minimal derivation and this module
//! calls it. When `hc-calendars-lunar` lands, re-point both.
//!
//! # Before 1873
//!
//! 六曜 as a daily cycle is a Meiji-era popularisation; earlier forms had
//! different names, a different order and, in the Muromachi period, a
//! different length. Nothing here is historical before the Gregorian
//! adoption in 1873, and the pre-1873 answers this module gives are
//! extrapolations of the modern rule, not what any surviving almanac says.

use hc_calendar::Rd;

use crate::lunisolar::{LunisolarDay, lunisolar_day};
use crate::meridian::Meridian;

/// One of the six days.
///
/// Ordering is the cycle order, 先勝 first, which is the order they succeed
/// one another within a lunisolar month.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rokuyo {
    /// 先勝: haste brings fortune. Lucky in the morning, unlucky after noon.
    Sensho,
    /// 友引: "pulling friends". Funerals are not held; crematoria close.
    Tomobiki,
    /// 先負: the opposite of 先勝. Act late, not early.
    Senbu,
    /// 仏滅: "the Buddha's death", the unluckiest of the six.
    Butsumetsu,
    /// 大安: "great peace", the luckiest. Weddings cost more.
    Taian,
    /// 赤口: "the red mouth". Unlucky except around noon.
    Shakko,
}

impl Rokuyo {
    /// All six, in cycle order.
    pub const ALL: [Self; 6] = [
        Self::Sensho,
        Self::Tomobiki,
        Self::Senbu,
        Self::Butsumetsu,
        Self::Taian,
        Self::Shakko,
    ];

    /// The six in the order `(month + day) mod 6` produces them.
    ///
    /// This is the table the rule is: remainder 0 is 大安, and since the
    /// first day of the first month gives 1 + 1 = 2, remainder 2 is 先勝.
    const BY_REMAINDER: [Self; 6] = [
        Self::Taian,
        Self::Shakko,
        Self::Sensho,
        Self::Tomobiki,
        Self::Senbu,
        Self::Butsumetsu,
    ];

    /// The 六曜 of a lunisolar month and day.
    ///
    /// A leap month carries the number of the month it follows, so a leap
    /// month's 六曜 run exactly as the preceding month's did.
    #[must_use]
    pub const fn from_lunisolar(month: u8, day: u8) -> Self {
        Self::BY_REMAINDER[((month as usize) + (day as usize)) % 6]
    }

    /// This day's position in the cycle, 0 for 先勝 through 5 for 赤口.
    #[must_use]
    pub const fn cycle_index(self) -> u8 {
        match self {
            Self::Sensho => 0,
            Self::Tomobiki => 1,
            Self::Senbu => 2,
            Self::Butsumetsu => 3,
            Self::Taian => 4,
            Self::Shakko => 5,
        }
    }

    /// The next in the cycle, which is the next day's 六曜 unless a new
    /// lunisolar month begins.
    #[must_use]
    pub const fn next(self) -> Self {
        Self::ALL[((self.cycle_index() as usize) + 1) % 6]
    }

    /// The name in Japanese characters, e.g. `"大安"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        match self {
            Self::Sensho => "先勝",
            Self::Tomobiki => "友引",
            Self::Senbu => "先負",
            Self::Butsumetsu => "仏滅",
            Self::Taian => "大安",
            Self::Shakko => "赤口",
        }
    }

    /// The name in Hepburn romaji.
    ///
    /// Several have two readings in ordinary use — 先勝 is *senshō* or
    /// *sakigachi*, 赤口 *shakkō* or *shakku* — and the form given here is the
    /// one the printed calendars use.
    #[must_use]
    pub const fn romaji(self) -> &'static str {
        match self {
            Self::Sensho => "senshō",
            Self::Tomobiki => "tomobiki",
            Self::Senbu => "senbu",
            Self::Butsumetsu => "butsumetsu",
            Self::Taian => "taian",
            Self::Shakko => "shakkō",
        }
    }

    /// A short English gloss of what the day is held to mean.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Sensho => "haste brings fortune; lucky before noon",
            Self::Tomobiki => "friends are drawn along; no funerals",
            Self::Senbu => "act late; unlucky before noon",
            Self::Butsumetsu => "the Buddha's death; the unluckiest day",
            Self::Taian => "great peace; the luckiest day",
            Self::Shakko => "the red mouth; unlucky except at noon",
        }
    }
}

/// The 六曜 of a day at a given meridian.
///
/// ```
/// use hc_seasons::{Meridian, rokuyo::{Rokuyo, rokuyo}};
///
/// // 10 February 2024 was the first of the first lunisolar month, and the
/// // first of the first month is always 先勝.
/// assert_eq!(rokuyo(hc_calendar::Rd(738_926), Meridian::JAPAN), Rokuyo::Sensho);
/// ```
#[must_use]
pub fn rokuyo(day: Rd, meridian: Meridian) -> Rokuyo {
    rokuyo_of(lunisolar_day(day, meridian))
}

/// The 六曜 of an already-derived lunisolar date.
///
/// Cheaper than [`rokuyo`] when the caller has the lunisolar date in hand,
/// because the conjunction search is the expensive part.
#[must_use]
pub const fn rokuyo_of(date: LunisolarDay) -> Rokuyo {
    Rokuyo::from_lunisolar(date.month, date.day)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::from_year_month_day;
    use crate::lunisolar::{month_number, month_start_containing, next_month_start};

    const JAPAN: Meridian = Meridian::JAPAN;

    /// The first day of each lunisolar month has a fixed 六曜, and the twelve
    /// of them are the six taken twice. This table is printed in every
    /// explanation of the cycle.
    const FIRST_OF_MONTH: [(u8, Rokuyo); 12] = [
        (1, Rokuyo::Sensho),
        (2, Rokuyo::Tomobiki),
        (3, Rokuyo::Senbu),
        (4, Rokuyo::Butsumetsu),
        (5, Rokuyo::Taian),
        (6, Rokuyo::Shakko),
        (7, Rokuyo::Sensho),
        (8, Rokuyo::Tomobiki),
        (9, Rokuyo::Senbu),
        (10, Rokuyo::Butsumetsu),
        (11, Rokuyo::Taian),
        (12, Rokuyo::Shakko),
    ];

    #[test]
    fn the_first_of_each_month_has_the_rokuyo_the_almanacs_print() {
        for (month, expected) in FIRST_OF_MONTH {
            assert_eq!(
                Rokuyo::from_lunisolar(month, 1),
                expected,
                "the first of month {month}"
            );
        }
    }

    #[test]
    fn the_cycle_runs_forward_within_a_month() {
        for month in 1..=12u8 {
            let mut expected = Rokuyo::from_lunisolar(month, 1);
            for day in 1..=29u8 {
                assert_eq!(
                    Rokuyo::from_lunisolar(month, day),
                    expected,
                    "month {month} day {day}"
                );
                expected = expected.next();
            }
        }
    }

    #[test]
    fn the_six_are_a_closed_cycle() {
        for rokuyo in Rokuyo::ALL {
            let mut walked = rokuyo;
            for _ in 0..6 {
                walked = walked.next();
            }
            assert_eq!(walked, rokuyo);
            assert!((0..6).contains(&rokuyo.cycle_index()));
            assert!(!rokuyo.japanese_name().is_empty());
            assert!(!rokuyo.romaji().is_empty());
            assert!(rokuyo.english_name().is_ascii());
        }
        let indices: [u8; 6] = Rokuyo::ALL.map(Rokuyo::cycle_index);
        assert_eq!(indices, [0, 1, 2, 3, 4, 5]);
    }

    /// Anchors that can be checked against a printed calendar. 1 January
    /// 2024 was 赤口; the lunar new years are the first of the first month
    /// and so are always 先勝; 中秋の名月 is the fifteenth of the eighth
    /// month, 8 + 15 = 23, remainder 5, so it is always 仏滅.
    #[test]
    fn published_rokuyo_dates_come_out_right() {
        let expected = [
            ((2024, 1, 1), Rokuyo::Shakko),
            ((2024, 2, 10), Rokuyo::Sensho),
            ((2025, 1, 29), Rokuyo::Sensho),
            ((2024, 9, 17), Rokuyo::Butsumetsu),
        ];
        for ((year, month, day), rokuyo_expected) in expected {
            assert_eq!(
                rokuyo(from_year_month_day(year, month, day), JAPAN),
                rokuyo_expected,
                "{year}-{month:02}-{day:02}"
            );
        }
    }

    /// The defining property: the cycle advances by one every day *except*
    /// at a new moon, where it jumps to whatever the new month number makes
    /// it. Over three years that must hold on every single day.
    #[test]
    fn the_cycle_advances_daily_and_resets_at_every_new_moon() {
        let start = from_year_month_day(2022, 1, 1);
        let mut previous_date = crate::lunisolar::lunisolar_day(start, JAPAN);
        let mut previous = rokuyo(start, JAPAN);
        let mut resets = 0;
        for offset in 1..1_100 {
            let day = Rd(start.0 + offset);
            let date = crate::lunisolar::lunisolar_day(day, JAPAN);
            let current = rokuyo(day, JAPAN);
            if date.month_start == previous_date.month_start {
                assert_eq!(current, previous.next(), "the cycle skipped at {day}");
            } else {
                resets += 1;
                assert_eq!(
                    current,
                    Rokuyo::from_lunisolar(date.month, 1),
                    "a month began at {day} with the wrong 六曜"
                );
            }
            previous = current;
            previous_date = date;
        }
        assert!(
            (34..=40).contains(&resets),
            "{resets} new moons in three years"
        );
    }

    /// 2023 had a leap second month. A leap month repeats the number of the
    /// month before it, so 閏二月 runs the same 六曜 sequence that 二月 did:
    /// both open on 先負. That is the case a naive "count the months" rule
    /// gets wrong, and it is the reason 六曜 needs a real leap flag.
    #[test]
    fn a_leap_month_repeats_the_previous_months_rokuyo_sequence() {
        let ordinary_start = from_year_month_day(2023, 2, 20);
        let leap_start = from_year_month_day(2023, 3, 22);
        assert_eq!(month_number(ordinary_start, JAPAN), (2, false));
        assert_eq!(month_number(leap_start, JAPAN), (2, true));
        assert_eq!(rokuyo(ordinary_start, JAPAN), Rokuyo::Tomobiki);
        assert_eq!(rokuyo(leap_start, JAPAN), Rokuyo::Tomobiki);
        // And day for day, all the way through both months.
        let ordinary_length = next_month_start(ordinary_start, JAPAN).0 - ordinary_start.0;
        let leap_length = next_month_start(leap_start, JAPAN).0 - leap_start.0;
        let shared = ordinary_length.min(leap_length);
        for offset in 0..shared {
            assert_eq!(
                rokuyo(Rd(ordinary_start.0 + offset), JAPAN),
                rokuyo(Rd(leap_start.0 + offset), JAPAN),
                "day {offset} of the two second months differed"
            );
        }
        // The month after the leap month is the third, so it opens on 先負.
        let third = next_month_start(leap_start, JAPAN);
        assert_eq!(month_number(third, JAPAN), (3, false));
        assert_eq!(rokuyo(third, JAPAN), Rokuyo::Senbu);
    }

    /// Every leap month in a century must behave that way, not just 2023's.
    #[test]
    fn every_leap_month_reruns_the_previous_months_rokuyo() {
        let mut start = month_start_containing(from_year_month_day(1950, 1, 1), JAPAN);
        let end = from_year_month_day(2050, 1, 1);
        let mut checked = 0;
        while start < end {
            let (number, leap) = month_number(start, JAPAN);
            if leap {
                let previous = month_start_containing(Rd(start.0 - 1), JAPAN);
                assert_eq!(month_number(previous, JAPAN), (number, false));
                assert_eq!(rokuyo(start, JAPAN), rokuyo(previous, JAPAN));
                assert_eq!(rokuyo(start, JAPAN), Rokuyo::from_lunisolar(number, 1));
                checked += 1;
            }
            start = next_month_start(start, JAPAN);
        }
        assert!(checked > 30, "only {checked} leap months examined");
    }

    /// Each of the six turns up about a sixth of the time. Months of 29 and
    /// 30 days and the 12-into-6 month numbering skew it slightly, which is
    /// why the bound is loose rather than exact.
    #[test]
    fn each_of_the_six_gets_roughly_a_sixth_of_the_days() {
        let start = from_year_month_day(2000, 1, 1);
        let mut counts = [0usize; 6];
        for offset in 0..3_653 {
            let rokuyo = rokuyo(Rd(start.0 + offset), JAPAN);
            counts[rokuyo.cycle_index() as usize] += 1;
        }
        assert_eq!(counts.iter().sum::<usize>(), 3_653);
        for count in counts {
            assert!(
                (540..=680).contains(&count),
                "one of the six got {count} days in a decade: {counts:?}"
            );
        }
    }

    #[test]
    fn the_cheap_and_the_expensive_entry_points_agree() {
        for offset in 0..400 {
            let day = Rd(739_000 + offset);
            let date = crate::lunisolar::lunisolar_day(day, JAPAN);
            assert_eq!(rokuyo(day, JAPAN), rokuyo_of(date));
        }
    }

    /// 大安 follows 仏滅 immediately in the cycle, always: the unluckiest day
    /// of the six is the eve of the luckiest. That is the property wedding
    /// halls and funeral parlours price off.
    #[test]
    fn the_luckiest_day_always_follows_the_unluckiest() {
        assert_eq!(
            (Rokuyo::Taian.cycle_index() + 6 - Rokuyo::Butsumetsu.cycle_index()) % 6,
            1
        );
        let start = from_year_month_day(2024, 1, 1);
        for offset in 0..366 {
            let day = Rd(start.0 + offset);
            if rokuyo(day, JAPAN) == Rokuyo::Butsumetsu {
                let date = crate::lunisolar::lunisolar_day(day, JAPAN);
                // 大安 follows 仏滅 the next day, unless a month turns over.
                let next = crate::lunisolar::lunisolar_day(Rd(day.0 + 1), JAPAN);
                if next.month_start == date.month_start {
                    assert_eq!(rokuyo(Rd(day.0 + 1), JAPAN), Rokuyo::Taian);
                }
            }
        }
    }
}
