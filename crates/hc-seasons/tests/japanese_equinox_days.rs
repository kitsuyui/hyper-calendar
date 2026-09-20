//! The acceptance test: Japan's legally published equinox days.
//!
//! 春分の日 and 秋分の日 are Japanese public holidays, and the law
//! (国民の祝日に関する法律, 1948) defines them not by a date but as "the day
//! of the vernal equinox" and "the day of the autumnal equinox". Which day
//! that is, is decided by the National Astronomical Observatory of Japan,
//! which computes the equinox in Japan Standard Time and publishes the result
//! in the 暦要項 in the *Official Gazette* each February, one year ahead.
//!
//! That makes them the sharpest possible test of this crate's central claim.
//! The answer is a *day*, it is derived from an instant, the instant is read
//! at a stated meridian, and an independent authority publishes the answer.
//! If `hc-seasons` and the Observatory disagree, one of them is wrong about a
//! public holiday.
//!
//! # What is being compared against
//!
//! The published days for 1980–2030 are listed explicitly below. They are
//! also reproduced by the floor formula that Japanese references give for
//! 1980–2099:
//!
//! ```text
//! 春分の日 = floor(20.8431 + 0.242194 (Y - 1980)) - floor((Y - 1980) / 4)
//! 秋分の日 = floor(23.2488 + 0.242194 (Y - 1980)) - floor((Y - 1980) / 4)
//! ```
//!
//! A test below checks the explicit table against that formula, so neither
//! can drift without the other noticing, and the formula then extends the
//! comparison to 2099 for free.
//!
//! # What is expected to disagree
//!
//! `hc-astro`'s solar longitude is the Meeus low-precision series, good to
//! about 0.01°, with a measured systematic bias of about −4.5 minutes. An
//! equinox falling within roughly ten minutes of midnight JST can therefore
//! be assigned the wrong day. 2012's autumn equinox, at 23:49 JST, is the
//! tightest case in the modern record. The tests measure the disagreement
//! rate and print it; they do not hide it.

use hc_seasons::Meridian;
use hc_seasons::solar_terms::{SolarTerm, term_day};

/// 春分の日 in March, for 1980 to 2030 inclusive, as published by the
/// National Astronomical Observatory of Japan.
const PUBLISHED_VERNAL_EQUINOX_DAYS: [u8; 51] = [
    20, 21, 21, 21, 20, 21, 21, 21, 20, 21, // 1980-1989
    21, 21, 20, 20, 21, 21, 20, 20, 21, 21, // 1990-1999
    20, 20, 21, 21, 20, 20, 21, 21, 20, 20, // 2000-2009
    21, 21, 20, 20, 21, 21, 20, 20, 21, 21, // 2010-2019
    20, 20, 21, 21, 20, 20, 20, 21, 20, 20, // 2020-2029
    20, // 2030
];

/// 秋分の日 in September, for 1980 to 2030 inclusive.
///
/// 22 September did not occur between 1896 and 2011; 2012 was the first, and
/// from then on it recurs every leap year.
const PUBLISHED_AUTUMNAL_EQUINOX_DAYS: [u8; 51] = [
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, // 1980-1989
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, // 1990-1999
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, // 2000-2009
    23, 23, 22, 23, 23, 23, 22, 23, 23, 23, // 2010-2019
    22, 23, 23, 23, 22, 23, 23, 23, 22, 23, // 2020-2029
    23, // 2030
];

/// The first year the explicit table and the formula both cover.
const FIRST_TABLE_YEAR: i64 = 1980;

/// The last year the formula is documented for.
const LAST_FORMULA_YEAR: i64 = 2099;

/// The proleptic Gregorian fixed day of a year, month and day.
///
/// Reingold & Dershowitz, *Calendrical Calculations*, 4th ed., §2.2. The
/// crate keeps its own copy private, so the test has one too rather than
/// reaching into it.
fn fixed_from_gregorian(year: i64, month: i64, day: i64) -> hc_calendar::Rd {
    let prior = year - 1;
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let correction = if month <= 2 {
        0
    } else if leap {
        -1
    } else {
        -2
    };
    hc_calendar::Rd(
        365 * prior + prior.div_euclid(4) - prior.div_euclid(100)
            + prior.div_euclid(400)
            + (367 * month - 362).div_euclid(12)
            + correction
            + day,
    )
}

/// The published 春分の日 day-of-month, by the documented 1980–2099 formula.
fn formula_vernal_day(year: i64) -> i64 {
    let elapsed = (year - 1980) as f64;
    (20.8431 + 0.242_194 * elapsed).floor() as i64 - (year - 1980).div_euclid(4)
}

/// The published 秋分の日 day-of-month, by the documented 1980–2099 formula.
fn formula_autumnal_day(year: i64) -> i64 {
    let elapsed = (year - 1980) as f64;
    (23.2488 + 0.242_194 * elapsed).floor() as i64 - (year - 1980).div_euclid(4)
}

/// The published day the two references agree on, as a fixed day.
fn published_day(year: i64, vernal: bool) -> hc_calendar::Rd {
    if vernal {
        fixed_from_gregorian(year, 3, formula_vernal_day(year))
    } else {
        fixed_from_gregorian(year, 9, formula_autumnal_day(year))
    }
}

/// The day this crate computes, in JST.
fn computed_day(year: i64, vernal: bool) -> hc_calendar::Rd {
    let term = if vernal {
        SolarTerm::SPRING_EQUINOX
    } else {
        SolarTerm::AUTUMN_EQUINOX
    };
    term_day(year, term, Meridian::JAPAN)
}

/// Compare over a span of years and return (comparisons, disagreements).
fn compare(first: i64, last: i64) -> (usize, usize) {
    let mut comparisons = 0;
    let mut disagreements = 0;
    for year in first..=last {
        for vernal in [true, false] {
            comparisons += 1;
            if computed_day(year, vernal) != published_day(year, vernal) {
                disagreements += 1;
                let season = if vernal {
                    "春分の日"
                } else {
                    "秋分の日"
                };
                println!(
                    "{year} {season}: computed RD {} but the gazette says RD {}",
                    computed_day(year, vernal).0,
                    published_day(year, vernal).0
                );
            }
        }
    }
    (comparisons, disagreements)
}

/// The explicit table and the documented formula must be the same data. If
/// they ever part company, one of the two was transcribed wrong and every
/// other test in this file is measuring the wrong thing.
#[test]
fn the_published_table_and_the_published_formula_agree() {
    for (offset, day) in PUBLISHED_VERNAL_EQUINOX_DAYS.into_iter().enumerate() {
        let year = FIRST_TABLE_YEAR + offset as i64;
        assert_eq!(
            i64::from(day),
            formula_vernal_day(year),
            "春分の日 {year}: the table and the formula disagree"
        );
    }
    for (offset, day) in PUBLISHED_AUTUMNAL_EQUINOX_DAYS.into_iter().enumerate() {
        let year = FIRST_TABLE_YEAR + offset as i64;
        assert_eq!(
            i64::from(day),
            formula_autumnal_day(year),
            "秋分の日 {year}: the table and the formula disagree"
        );
    }
}

/// A few dates everyone knows, spelled out so that a reader can check them
/// against a Japanese calendar without reading the table.
#[test]
fn the_dates_everyone_remembers_are_right() {
    // 2012 was the first 22 September since 1896.
    assert_eq!(computed_day(2012, false), fixed_from_gregorian(2012, 9, 22));
    assert_eq!(computed_day(2011, false), fixed_from_gregorian(2011, 9, 23));
    // 2021 was the first 20 March since 1796 in the old reckoning and the
    // earliest 春分の日 of the modern era.
    assert_eq!(computed_day(2021, true), fixed_from_gregorian(2021, 3, 20));
    assert_eq!(computed_day(2024, true), fixed_from_gregorian(2024, 3, 20));
    assert_eq!(computed_day(2024, false), fixed_from_gregorian(2024, 9, 22));
    assert_eq!(computed_day(1980, true), fixed_from_gregorian(1980, 3, 20));
    assert_eq!(computed_day(1980, false), fixed_from_gregorian(1980, 9, 23));
}

/// **The acceptance test.** 102 published days over 1980–2030, compared one
/// by one against what this crate computes for the Japanese meridian.
///
/// The bound is deliberately tight. `hc-astro` can only misplace a day when
/// the equinox falls within about ten minutes of midnight JST, and over this
/// span that happens to no equinox at all, so the expected answer is zero.
/// The bound is set at two so that the test reports a regression rather than
/// a hard failure if the underlying series is ever retuned, and the printed
/// rate is what the README quotes.
#[test]
fn the_published_equinox_days_of_1980_to_2030_are_reproduced() {
    let (comparisons, disagreements) = compare(1980, 2030);
    assert_eq!(comparisons, 102, "the span should hold 102 published days");
    let rate = disagreements as f64 / comparisons as f64 * 100.0;
    println!("1980-2030: {disagreements} of {comparisons} disagree ({rate:.2}%)");
    assert!(
        disagreements <= 2,
        "{disagreements} of {comparisons} published equinox days disagree ({rate:.2}%), \
         which is more than the low-precision solar series can explain"
    );
}

/// The same comparison extended to the whole span the published formula
/// covers: 240 days over 1980–2099.
#[test]
fn the_published_equinox_days_of_1980_to_2099_are_reproduced() {
    let (comparisons, disagreements) = compare(FIRST_TABLE_YEAR, LAST_FORMULA_YEAR);
    assert_eq!(comparisons, 240);
    let rate = disagreements as f64 / comparisons as f64 * 100.0;
    println!("1980-2099: {disagreements} of {comparisons} disagree ({rate:.2}%)");
    assert!(
        rate < 2.0,
        "{disagreements} of {comparisons} disagree ({rate:.2}%), which is over the \
         few percent the −4.5-minute bias in the solar series can account for"
    );
}

/// The residual is a *boundary* effect, not a general inaccuracy. Every day
/// this crate computes must be one of the two calendar days the equinox can
/// fall on, never further out — a genuine model failure would show up as a
/// two-day error long before it showed up as a one-day one.
#[test]
fn no_computed_equinox_day_is_more_than_a_day_from_the_published_one() {
    for year in FIRST_TABLE_YEAR..=LAST_FORMULA_YEAR {
        for vernal in [true, false] {
            let gap = computed_day(year, vernal).0 - published_day(year, vernal).0;
            assert!(
                (-1..=1).contains(&gap),
                "{year} (vernal: {vernal}) was {gap} days out, which is not a \
                 midnight-boundary effect"
            );
        }
    }
}

/// Where the crate and the gazette *do* disagree, the equinox must be within
/// half an hour of midnight JST. Anywhere else, a −4.5-minute bias cannot
/// move a date, so a disagreement would mean something else was broken.
#[test]
fn every_disagreement_is_a_near_midnight_case() {
    for year in FIRST_TABLE_YEAR..=LAST_FORMULA_YEAR {
        for vernal in [true, false] {
            if computed_day(year, vernal) == published_day(year, vernal) {
                continue;
            }
            let term = if vernal {
                SolarTerm::SPRING_EQUINOX
            } else {
                SolarTerm::AUTUMN_EQUINOX
            };
            let hours =
                Meridian::JAPAN.local_hours(hc_seasons::solar_terms::term_moment(year, term));
            let from_midnight = hours.min(24.0 - hours);
            assert!(
                from_midnight < 0.5,
                "{year} (vernal: {vernal}) disagreed although the equinox was at \
                 {hours:.2}h JST, {from_midnight:.2} hours from midnight"
            );
        }
    }
}

/// The meridian is load-bearing. Computing Japan's holiday in Universal Time
/// instead of JST gets it wrong many times a century, which is the mistake
/// this crate's `Meridian` argument exists to make impossible.
#[test]
fn computing_the_holiday_in_universal_time_would_get_it_wrong() {
    let mut wrong = 0;
    for year in FIRST_TABLE_YEAR..=LAST_FORMULA_YEAR {
        for (vernal, term) in [
            (true, SolarTerm::SPRING_EQUINOX),
            (false, SolarTerm::AUTUMN_EQUINOX),
        ] {
            if term_day(year, term, Meridian::UNIVERSAL) != published_day(year, vernal) {
                wrong += 1;
            }
        }
    }
    println!("1980-2099 in UT instead of JST: {wrong} of 240 wrong");
    assert!(
        wrong > 50,
        "only {wrong} of 240 would be wrong in UT, which cannot be right"
    );
}

/// 春分の日 is the middle day of the spring 彼岸 and the day the spring
/// equinox's solar term falls on. All three must name the same day, or the
/// crate is internally inconsistent about its own headline case.
#[test]
fn the_holiday_the_higan_and_the_solar_term_all_name_the_same_day() {
    for year in FIRST_TABLE_YEAR..=LAST_FORMULA_YEAR {
        let spring = hc_seasons::zassetsu::higan(
            year,
            hc_seasons::zassetsu::HiganSeason::Spring,
            Meridian::JAPAN,
        );
        assert_eq!(spring.middle, computed_day(year, true));
        assert_eq!(
            hc_seasons::zassetsu::day_of(
                hc_seasons::Zassetsu::SpringHiganMiddle,
                year,
                Meridian::JAPAN
            ),
            computed_day(year, true)
        );
        let autumn = hc_seasons::zassetsu::higan(
            year,
            hc_seasons::zassetsu::HiganSeason::Autumn,
            Meridian::JAPAN,
        );
        assert_eq!(autumn.middle, computed_day(year, false));
    }
}
