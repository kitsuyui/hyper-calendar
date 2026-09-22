//! The measurement: how far the printed zodiac dates have drifted from the
//! Sun.
//!
//! Every newspaper astrology column prints the same fixed dates — "Aries:
//! March 21 – April 19", "Leo: July 23 – August 22" — and none of them
//! recomputes anything. The dates were a good fit when they were settled, in
//! the English-language press of the early twentieth century. They are a
//! worse fit now, because the March equinox has moved: it fell on 21 March in
//! most years around 1900 and falls on 20 March in most years now, and the
//! whole zodiac is pinned to it.
//!
//! This test compares the computed ingresses against those fixed dates and
//! **reports the disagreement rate and its size** rather than asserting that
//! either is right. The drift is real and it is the interesting part.
//!
//! # Why this is not simply "the fixed dates are wrong"
//!
//! Three separate things move the answer, and only one of them is drift:
//!
//! 1. **The calendar drifting against the Sun.** Between the 1900 and the
//!    2100 century rules there is no skipped leap year — 2000 was a leap year
//!    — so for those two hundred years the Gregorian calendar keeps a mean
//!    year of exactly 365.25 days and behaves like the Julian one. That is
//!    0.0078 days longer than the tropical year, so every ingress creeps
//!    about **three quarters of a day earlier per century** until 2100 resets
//!    it. Over 1900–2099 that is a day and a half, which is why the March
//!    equinox was usually 21 March in 1900 and is usually 20 March now.
//! 2. **The leap-year cycle.** Within any four years an ingress moves back
//!    three quarters of a day and then forward a day, so a sign's date takes
//!    two adjacent values even in a single decade. A fixed date can only
//!    match one of them, and a mean that sits near a midnight can swing the
//!    match rate from a quarter to nine tenths without much having moved.
//! 3. **The meridian.** A fixed date has no meridian attached. The same
//!    ingress instant is one date in London and another in Tokyo about
//!    three times in eight, so a list of fixed dates cannot be right
//!    everywhere at once. The test reports three meridians, and the result is
//!    worth reading: the printed dates fit **New York best around 1900** —
//!    which is where and when they were settled — and fit **Tokyo best in the
//!    late twentieth century**, purely because a nine-hour offset happens to
//!    catch the drift at a different point.
//!
//! # The accuracy caveat that applies to every number below
//!
//! `hc-astro`'s solar longitude is the VSOP87 series, good to about 1″. An
//! ingress falling within about a minute of local midnight can still be
//! assigned the wrong day, so one or two of the disagreements counted here
//! may be the model's and not the almanac's. The test therefore checks the
//! *shape* of the disagreement — its size in days and how it changes across
//! a century — rather than an exact count, and prints the counts for a human
//! to read.

use hc_seasons::Meridian;
use hc_seasons::zodiac::TropicalSign;
use hc_seasons::zodiac::tropical::{ingress_day, ingress_moment};

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

/// The day a sign is conventionally said to start in a given year.
///
/// Every conventional start date falls in the same Gregorian year as the
/// computed ingress, because the year's twelve ingresses run from Aquarius in
/// January to Capricorn in December and the printed dates do the same.
fn conventional_start(year: i64, sign: TropicalSign) -> hc_calendar::Rd {
    let period = sign.conventional_period();
    fixed_from_gregorian(
        year,
        i64::from(period.start_month),
        i64::from(period.start_day),
    )
}

/// The three meridians the comparison is run at.
///
/// Greenwich because the conventional dates are an English-language
/// convention; New York because that is where most of the columns that print
/// them are written; Tokyo because it is the furthest from Greenwich that
/// this crate has a named meridian for, and so shows the largest effect.
const MERIDIANS: [(&str, Meridian); 3] = [
    ("Greenwich", Meridian::UNIVERSAL),
    ("New York (UTC-5)", Meridian::from_seconds(-5 * 3_600)),
    ("Tokyo (UTC+9)", Meridian::JAPAN),
];

/// How many sign-years each span below compares.
fn comparisons(first_year: i64, last_year: i64) -> i64 {
    (last_year - first_year + 1) * 12
}

/// The count of disagreements, the extreme offsets, and the mean offset
/// measured on the instants rather than the days.
///
/// The day count is what a reader wants; the mean instant offset is what
/// actually moves, because a day count is a day count of something that has
/// been rounded to a midnight.
struct Drift {
    disagreements: i64,
    total: i64,
    earliest: i64,
    latest: i64,
    mean_offset_days: f64,
}

impl Drift {
    fn measure(first_year: i64, last_year: i64, meridian: Meridian) -> Self {
        let mut drift = Self {
            disagreements: 0,
            total: 0,
            earliest: 0,
            latest: 0,
            mean_offset_days: 0.0,
        };
        let mut offset_total = 0.0;
        for year in first_year..=last_year {
            for sign in TropicalSign::ALL {
                let printed = conventional_start(year, sign);
                let offset = ingress_day(year, sign, meridian).0 - printed.0;
                drift.total += 1;
                if offset != 0 {
                    drift.disagreements += 1;
                }
                drift.earliest = drift.earliest.min(offset);
                drift.latest = drift.latest.max(offset);
                // The same comparison on the instant: how long after the
                // printed date's local midnight the Sun actually arrived.
                let local = meridian.local(ingress_moment(year, sign)).0;
                offset_total += local - printed.0 as f64;
            }
        }
        drift.mean_offset_days = offset_total / drift.total as f64;
        drift
    }

    fn percent(&self) -> f64 {
        self.disagreements as f64 * 100.0 / self.total as f64
    }
}

/// The headline measurement, printed as a table.
///
/// Run with `cargo test -p hc-seasons --test zodiac_conventional_dates -- --nocapture`
/// to read it.
#[test]
fn the_printed_zodiac_dates_disagree_with_the_sun_and_the_rate_is_reported() {
    let spans = [(1900i64, 1929i64), (1970, 1999), (2000, 2029), (2070, 2099)];

    println!();
    println!("Computed sign ingresses against the conventional printed dates");
    println!("(12 signs a year; offset is computed day minus printed day)");
    println!();
    println!(
        "{:<18} {:>10} {:>9} {:>7} {:>8} {:>10} {:>10}",
        "meridian", "span", "compared", "differ", "rate", "offsets", "mean"
    );

    for (name, meridian) in MERIDIANS {
        for (first, last) in spans {
            let drift = Drift::measure(first, last, meridian);
            assert_eq!(drift.total, comparisons(first, last));
            println!(
                "{:<18} {:>10} {:>9} {:>7} {:>7.1}% {:>4} to {:>2} {:>+9.2}d",
                name,
                format!("{first}-{last}"),
                drift.total,
                drift.disagreements,
                drift.percent(),
                drift.earliest,
                drift.latest,
                drift.mean_offset_days,
            );
        }
    }
    println!();

    // The *day* counts are not monotone, because a day count is a rounded
    // number and the rounding depends on the meridian: the printed dates fit
    // New York best around 1900 and Tokyo best in the late twentieth century.
    // What is monotone is the thing underneath — the mean arrival time of
    // the Sun against the printed date — and it falls at every meridian and
    // in every span, because the calendar is running slow against the Sun.
    for (name, meridian) in MERIDIANS {
        let mut previous = f64::MAX;
        for (first, last) in spans {
            let drift = Drift::measure(first, last, meridian);
            assert!(
                drift.mean_offset_days < previous,
                "at {name} the {first}-{last} mean offset {:+.2}d did not fall \
                 below the previous span's {previous:+.2}d",
                drift.mean_offset_days
            );
            previous = drift.mean_offset_days;
        }
    }

    // And the size of that fall is the calendar's: about three quarters of a
    // day per century while no century rule intervenes.
    let first_span = Drift::measure(1900, 1929, Meridian::UNIVERSAL);
    let last_span = Drift::measure(2070, 2099, Meridian::UNIVERSAL);
    let fall = first_span.mean_offset_days - last_span.mean_offset_days;
    println!("Mean arrival fell {fall:.2} days between 1900-1929 and 2070-2099");
    println!();
    assert!(
        (1.1..=1.7).contains(&fall),
        "the mean arrival fell {fall} days over 170 years, not the \
         three-quarters-of-a-day-a-century the leap rule implies"
    );

    // And the disagreement is never large: across two centuries and three
    // meridians the printed dates are a day or two out, never a season out.
    // Anything beyond two days would mean this crate had the astronomy wrong
    // rather than the almanac being stale.
    for (name, meridian) in MERIDIANS {
        let drift = Drift::measure(1900, 2099, meridian);
        assert!(
            drift.earliest >= -2 && drift.latest <= 2,
            "at {name} the offsets ranged {} to {} days",
            drift.earliest,
            drift.latest
        );
    }
}

/// The offsets are almost all 0 or −1: the printed dates are the ones the
/// equinox used to fall on, and it now falls a day earlier most years.
#[test]
fn the_modern_disagreement_is_almost_always_exactly_one_day_early() {
    let mut histogram = [0i64; 5];
    for year in 2000..=2029 {
        for sign in TropicalSign::ALL {
            let offset =
                ingress_day(year, sign, Meridian::UNIVERSAL).0 - conventional_start(year, sign).0;
            let bucket = (offset + 2).clamp(0, 4) as usize;
            histogram[bucket] += 1;
        }
    }
    println!();
    println!("Offset distribution, 2000-2029, Greenwich:");
    for (bucket, count) in histogram.into_iter().enumerate() {
        println!("  {:>+3} days: {count:>4}", bucket as i64 - 2);
    }
    println!();

    let early_by_one = histogram[1];
    let exact = histogram[2];
    // Nothing is ever late and nothing is ever two days early: the drift is
    // entirely one-sided and entirely small. A late ingress would mean the
    // astronomy was wrong rather than the almanac stale.
    assert_eq!(
        histogram[0], 0,
        "no ingress should be two whole days before its printed date"
    );
    assert_eq!(histogram[3] + histogram[4], 0, "no ingress should be late");
    // In this window the split is close to even — 185 exact against 175 a day
    // early at the time of writing — which is exactly what a drifting mean
    // sitting near a midnight produces. It will not stay even.
    assert!(
        early_by_one > 100 && exact > 100,
        "the offsets should be split between 0 and -1 days, not {exact} to \
         {early_by_one}"
    );
    assert_eq!(early_by_one + exact, 360);
}

/// A fixed date has no meridian, and the meridian changes the answer. So even
/// a perfectly recomputed list of fixed dates would be wrong somewhere.
#[test]
fn the_printed_dates_cannot_be_right_at_every_meridian_at_once() {
    let mut disagreements = 0;
    for year in 1950..=2049 {
        for sign in TropicalSign::ALL {
            if ingress_day(year, sign, Meridian::UNIVERSAL)
                != ingress_day(year, sign, Meridian::JAPAN)
            {
                disagreements += 1;
            }
        }
    }
    println!();
    println!(
        "Greenwich and Tokyo put a sign boundary on different dates \
         {disagreements} times in 1200 sign-years ({:.1}%)",
        disagreements as f64 * 100.0 / 1200.0
    );
    println!();
    // Nine hours in twenty-four is three eighths, so about 450 of 1200.
    assert!(
        (350..=550).contains(&disagreements),
        "{disagreements} is not the rate nine hours of longitude implies"
    );
}

/// The conventional dates were not invented at random: they are close to the
/// ingresses of the era they were printed in. This checks that the printed
/// list is a plausible early-twentieth-century almanac rather than an
/// arbitrary twelve dates, by requiring every one of them to be within a day
/// of the real 1910 ingress.
#[test]
fn the_printed_dates_match_an_early_twentieth_century_almanac_to_within_a_day() {
    for sign in TropicalSign::ALL {
        let offset =
            ingress_day(1910, sign, Meridian::UNIVERSAL).0 - conventional_start(1910, sign).0;
        assert!(
            (-1..=1).contains(&offset),
            "{} was {offset} days from its printed date in 1910",
            sign.english_name()
        );
    }
}

/// A sanity check on the instants behind the days: consecutive ingresses of
/// the same sign are one tropical year apart, so nothing above is comparing
/// the wrong crossing.
#[test]
fn consecutive_ingresses_of_a_sign_are_one_tropical_year_apart() {
    for sign in TropicalSign::ALL {
        for year in 1900..2100 {
            let gap = ingress_moment(year + 1, sign).0 - ingress_moment(year, sign).0;
            assert!(
                (365.0..=366.0).contains(&gap),
                "{} of {year} to {} of {} was {gap} days",
                sign.english_name(),
                sign.english_name(),
                year + 1
            );
        }
    }
}
