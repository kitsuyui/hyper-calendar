//! The per-day facts that every 暦注 rule is a statement about.
//!
//! Almost every annotation in this crate is a predicate over three things:
//! the day's sexagenary position, the 節月 (the month that begins at a
//! sectional solar term) it falls in, and — for a handful — the lunisolar
//! month and day. Computing the last two means running the solar-longitude
//! series and a new-moon search, which is far and away the most expensive
//! thing this crate does. A printed almanac page carries twenty-odd
//! annotations for one day, so computing that context once and handing it to
//! every rule is the difference between one astronomical solve and forty.
//!
//! [`DayContext`] is that shared context. Every evaluator in the crate takes
//! one by reference.

use hc_calendar::Rd;
use hc_calendar::cycle::{Sexagenary, sexagenary_day};
use hc_core::math::{floor, normalize_degrees};
use hc_seasons::lunisolar::{LunisolarDay, lunisolar_day};
use hc_seasons::solar_terms::SolarTerm;
use hc_seasons::{Meridian, Moment};

/// Degrees of apparent solar longitude between two sectional terms.
const DEGREES_PER_SOLAR_MONTH: f64 = 30.0;

/// The apparent solar longitude of 立春, which opens the first 節月.
const LICHUN_LONGITUDE: f64 = 315.0;

/// How far back the sectional-term search starts, in days.
///
/// The Sun needs at most about 31.5 days to cover 30°, near aphelion. Any
/// start earlier than the previous crossing of the same longitude — a full
/// year back — and later than 31.5 days before the instant wanted brackets
/// exactly one crossing, so 35 is comfortably inside both bounds.
const SEARCH_LOOKBACK_DAYS: f64 = 35.0;

/// The earthly branch index of 寅, which names the first 節月.
const BRANCH_OF_FIRST_SOLAR_MONTH: u8 = 2;

/// A 節月: one of the twelve months that begin at a sectional solar term.
///
/// This is the month almost every 暦注 table means by "正月", "二月" and so
/// on, and it is *not* the lunisolar month. The first 節月 opens at 立春 and
/// is 寅月; the twelfth opens at 小寒 and is 丑月. A 節月 is a solar quantity
/// — it has nothing to do with the Moon — so its boundaries move by at most
/// a day from year to year, where a lunisolar month's move by a fortnight.
///
/// The distinction matters because a few 暦注 genuinely are keyed to the
/// lunisolar month instead (不成就日 is the clearest case), and conflating
/// the two is the single commonest error in published rule tables.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SolarMonth {
    /// One-based, 1 for 寅月 beginning at 立春 through 12 for 丑月 beginning
    /// at 小寒.
    number: u8,
    /// The sectional term that opened it.
    term: SolarTerm,
    /// The day that term fell on, at the meridian asked for.
    start: Rd,
}

impl SolarMonth {
    /// The one-based month number, 1 for 寅月 through 12 for 丑月.
    #[must_use]
    pub const fn number(self) -> u8 {
        self.number
    }

    /// The sectional term that opens this month.
    #[must_use]
    pub const fn term(self) -> SolarTerm {
        self.term
    }

    /// The day the opening term fell on.
    #[must_use]
    pub const fn start(self) -> Rd {
        self.start
    }

    /// The zero-based earthly branch that names this month: 2 (寅) for the
    /// first, wrapping to 1 (丑) for the twelfth.
    #[must_use]
    pub const fn branch_index(self) -> u8 {
        (self.number + BRANCH_OF_FIRST_SOLAR_MONTH - 1) % 12
    }

    /// The zero-based index into a twelve-entry table written in almanac
    /// order, i.e. `number - 1`.
    #[must_use]
    pub const fn table_index(self) -> usize {
        (self.number - 1) as usize
    }

    /// The sectional term that opens 節月 `number`, for `number` in 1..=12.
    ///
    /// Returns `None` outside that range.
    #[must_use]
    pub const fn opening_term(number: u8) -> Option<SolarTerm> {
        if number == 0 || number > 12 {
            return None;
        }
        Some(sectional_term(number - 1))
    }
}

/// The sectional term that opens 節月 number `offset + 1`.
///
/// 立春 opens the first and sits at 315°; each following 節月 opens 30°
/// further along the ecliptic, which is two [`SolarTerm`] steps. Stepping
/// rather than indexing keeps this a `const fn` without reaching into
/// `hc-seasons`' private numbering.
const fn sectional_term(offset: u8) -> SolarTerm {
    let mut term = SolarTerm::BEGINNING_OF_SPRING;
    let mut stepped = 0;
    while stepped < offset {
        term = term.next().next();
        stepped += 1;
    }
    term
}

/// The sectional term in effect on a day, and the 節月 it opens.
///
/// The term "begins" on the day its instant falls on at the given meridian,
/// even when that instant is at a minute to midnight — that is how an
/// almanac prints it, and it is why this function needs a [`Meridian`].
///
/// The astronomy is `hc-astro`'s VSOP87 solar series, good to about 1″. An
/// instant within about a minute of local midnight can therefore still be
/// assigned the wrong day, which moves the 節月 boundary by one
/// day and, through it, every annotation keyed to the 節月. See the crate
/// README for the measured rate.
#[must_use]
pub fn solar_month_of(day: Rd, meridian: Meridian) -> SolarMonth {
    let end_of_day = meridian.midnight(Rd(day.0 + 1));
    let longitude = hc_astro::solar_longitude(end_of_day);
    let elapsed = normalize_degrees(longitude - LICHUN_LONGITUDE);
    let offset = floor(elapsed / DEGREES_PER_SOLAR_MONTH) as u8;
    let number = offset + 1;
    let target = normalize_degrees(LICHUN_LONGITUDE + f64::from(offset) * DEGREES_PER_SOLAR_MONTH);
    let moment =
        hc_astro::solar_longitude_after(target, Moment(end_of_day.0 - SEARCH_LOOKBACK_DAYS));
    SolarMonth {
        number,
        term: sectional_term(offset),
        start: meridian.day_of(moment),
    }
}

/// Everything a 暦注 rule can ask about a day.
///
/// Build one with [`DayContext::new`] and pass it to every evaluator. The
/// astronomy runs once, in the constructor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DayContext {
    day: Rd,
    meridian: Meridian,
    sexagenary: Sexagenary,
    solar_month: SolarMonth,
    lunisolar: LunisolarDay,
}

impl DayContext {
    /// Compute the context for a day at a meridian.
    ///
    /// This runs one solar-longitude solve and two new-moon searches. It is
    /// the expensive call in this crate; everything downstream is table
    /// lookup and modular arithmetic.
    #[must_use]
    pub fn new(day: Rd, meridian: Meridian) -> Self {
        Self {
            day,
            meridian,
            sexagenary: sexagenary_day(day),
            solar_month: solar_month_of(day, meridian),
            lunisolar: lunisolar_day(day, meridian),
        }
    }

    /// The day this context describes.
    #[must_use]
    pub const fn day(self) -> Rd {
        self.day
    }

    /// The meridian the day boundaries were read at.
    #[must_use]
    pub const fn meridian(self) -> Meridian {
        self.meridian
    }

    /// The day's position in the sexagenary cycle (日の干支).
    #[must_use]
    pub const fn sexagenary(self) -> Sexagenary {
        self.sexagenary
    }

    /// The day's zero-based heavenly stem, 0 for 甲 through 9 for 癸.
    #[must_use]
    pub const fn stem_index(self) -> u8 {
        self.sexagenary.stem_index()
    }

    /// The day's zero-based earthly branch, 0 for 子 through 11 for 亥.
    #[must_use]
    pub const fn branch_index(self) -> u8 {
        self.sexagenary.branch_index()
    }

    /// The 節月 the day falls in.
    #[must_use]
    pub const fn solar_month(self) -> SolarMonth {
        self.solar_month
    }

    /// How many days the day is past the sectional term that opened its 節月,
    /// counting that term's own day as zero.
    #[must_use]
    pub const fn days_into_solar_month(self) -> i64 {
        self.day.0 - self.solar_month.start.0
    }

    /// The day's lunisolar month and day.
    ///
    /// This is `hc-seasons`' minimal 定気 derivation, which is what 六曜 is
    /// built on. It decides month by month rather than looking at the whole
    /// year between two solstices, so its leap months can differ from the
    /// 天保暦 rule, and when they do a whole month is numbered differently:
    /// 89 of the 3,653 days of 2024–2033, measured in
    /// `hc_seasons::lunisolar`. Of the rules here only 不成就日 reads it; the
    /// 二十七宿 and 六曜 use the same derivation.
    #[must_use]
    pub const fn lunisolar(self) -> LunisolarDay {
        self.lunisolar
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// RD 738886 is 2024-01-01; the National Astronomical Observatory of
    /// Japan's 暦要項 for 2024 puts 小寒 on 6 January, so New Year's Day
    /// still belongs to the twelfth 節月, 子月, opened by 大雪 on
    /// 2023-12-07.
    #[test]
    fn new_years_day_2024_falls_in_the_solar_month_opened_by_the_december_terms() {
        let month = solar_month_of(Rd(738_886), Meridian::JAPAN);
        assert_eq!(month.number(), 11);
        assert_eq!(month.term(), SolarTerm::from_degrees(255).expect("大雪"));
        assert_eq!(month.start(), Rd(738_861));
    }

    /// 立春 2024 fell on 4 February (RD 738920) in Japan, so that day opens
    /// 節月 1 and the day before still belongs to 節月 12.
    #[test]
    fn the_solar_month_turns_on_the_day_of_the_sectional_term() {
        let lichun = Rd(738_920);
        assert_eq!(solar_month_of(lichun, Meridian::JAPAN).number(), 1);
        assert_eq!(solar_month_of(lichun, Meridian::JAPAN).start(), lichun);
        assert_eq!(
            solar_month_of(Rd(lichun.0 - 1), Meridian::JAPAN).number(),
            12
        );
    }

    #[test]
    fn the_twelve_solar_months_carry_the_branches_from_the_tiger() {
        for number in 1..=12u8 {
            let term = SolarMonth::opening_term(number).expect("in range");
            let month = SolarMonth {
                number,
                term,
                start: Rd(0),
            };
            assert_eq!(month.branch_index(), (number + 1) % 12);
            assert!(term.is_sectional(), "節月 {number} must open at a 節気");
        }
        assert_eq!(SolarMonth::opening_term(0), None);
        assert_eq!(SolarMonth::opening_term(13), None);
    }

    #[test]
    fn the_first_solar_month_opens_at_the_beginning_of_spring() {
        assert_eq!(
            SolarMonth::opening_term(1),
            Some(SolarTerm::BEGINNING_OF_SPRING)
        );
    }

    /// A year of consecutive days must visit the twelve 節月 in order and
    /// never skip or repeat one.
    #[test]
    fn solar_months_advance_monotonically_through_a_year() {
        let mut previous = solar_month_of(Rd(738_886), Meridian::JAPAN).number();
        let mut changes = 0;
        for offset in 1..366 {
            let number = solar_month_of(Rd(738_886 + offset), Meridian::JAPAN).number();
            if number != previous {
                assert_eq!(number, previous % 12 + 1, "at offset {offset}");
                changes += 1;
                previous = number;
            }
        }
        assert_eq!(changes, 12);
    }

    #[test]
    fn the_context_agrees_with_the_sexagenary_day_cycle() {
        // RD 738886 is 2024-01-01, which is a 甲子 day: (738886 + 14) is an
        // exact multiple of 60.
        let context = DayContext::new(Rd(738_886), Meridian::JAPAN);
        assert_eq!(context.sexagenary().index(), 0);
        assert_eq!(context.stem_index(), 0);
        assert_eq!(context.branch_index(), 0);
        assert_eq!(context.day(), Rd(738_886));
        assert_eq!(context.meridian(), Meridian::JAPAN);
    }

    /// The 旧暦-keyed annotations use `hc-seasons`' minimal derivation rather
    /// than `hc-calendars-lunar`'s full Chinese calendar, so the crate owes
    /// the reader a *measurement* of the difference rather than an
    /// assurance.
    ///
    /// Measured over the 3,653 days of 2024–2033 at the Chinese meridian,
    /// the two disagree on **89 days, 2.4%**, in one contiguous run — the
    /// minimal rule decides a leap month one month at a time where the full
    /// one looks across a whole solstice-to-solstice year, so when they
    /// differ they differ about a whole month's numbering and not about
    /// single days. These are the days on which 不成就日, 二十七宿 and 六曜
    /// can come out differently under the two.
    ///
    /// The bound asserted here is five per cent. It is a regression guard,
    /// not a claim: if the figure moves, the README figure must move too.
    #[test]
    fn the_minimal_lunisolar_derivation_tracks_the_full_chinese_calendar() {
        use hc_calendar::Calendar;
        use hc_calendars_lunar::ChineseCalendar;

        let mut disagreements = 0;
        let mut runs = 0;
        let mut previous_disagreed = false;
        let total = 3_653;
        for offset in 0..total {
            let day = Rd(738_886 + offset);
            let minimal = lunisolar_day(day, Meridian::CHINA);
            let full = ChineseCalendar
                .from_fixed(day)
                .expect("the Chinese calendar covers the twenty-first century");
            let differs = minimal.month != full.month.ordinal
                || minimal.day != full.day
                || minimal.leap_month != full.month.leap;
            if differs {
                disagreements += 1;
                if !previous_disagreed {
                    runs += 1;
                }
            }
            previous_disagreed = differs;
        }
        assert!(
            disagreements * 20 < total,
            "the two lunisolar derivations diverged on {disagreements} of {total} days"
        );
        assert!(
            runs <= 5,
            "divergence should come in whole-month runs, not scattered days; saw {runs} runs"
        );
    }

    #[test]
    fn days_into_the_solar_month_is_zero_on_the_terms_own_day() {
        let lichun = DayContext::new(Rd(738_920), Meridian::JAPAN);
        assert_eq!(lichun.days_into_solar_month(), 0);
        let next = DayContext::new(Rd(738_921), Meridian::JAPAN);
        assert_eq!(next.days_into_solar_month(), 1);
    }
}
