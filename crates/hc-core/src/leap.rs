//! The UTC leap-second table.
//!
//! This module is *data*. The algorithm that uses it lives in
//! [`crate::unix`]. Keeping the two apart is the same separation the
//! calendar crates apply: a leap second is an announcement by the IERS, not
//! something a formula can derive, so it must be replaceable without touching
//! any conversion code.
//!
//! # Scope
//!
//! * From 1972-01-01 onwards, `TAI - UTC` is a whole number of seconds and
//!   this table is exact.
//! * From 1961-01-01 to 1971-12-31, UTC ran at a deliberately offset rate;
//!   [`RATE_ERA`] carries the official piecewise-linear coefficients.
//! * Before 1961-01-01 UTC did not exist. Conversions there return
//!   [`crate::TimeError::BeforeModelStart`] unless the caller opts into
//!   treating UTC as TAI-like.
//!
//! Leap seconds are announced roughly six months ahead, so any date past
//! [`table_valid_until_unix`] is unknowable, not merely unlisted. Callers who
//! need a prediction must say so explicitly.

/// One step of the modern integer leap-second table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeapEntry {
    /// The first Unix second at which `tai_minus_utc` applies.
    pub start_unix: i64,
    /// `TAI - UTC` in whole seconds from `start_unix` onwards.
    pub tai_minus_utc: i64,
    /// The UTC date on which the step took effect, for diagnostics.
    pub label: &'static str,
}

/// One segment of the 1961-1971 rate-offset era.
///
/// `TAI - UTC = offset + (MJD - origin_mjd) * drift`, in seconds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RateEntry {
    /// The first Unix second at which this segment applies.
    pub start_unix: i64,
    /// The constant term, in seconds.
    pub offset: f64,
    /// The Modified Julian Date the drift is measured from.
    pub origin_mjd: f64,
    /// Seconds of drift per day of MJD.
    pub drift: f64,
    /// The UTC date on which the segment took effect, for diagnostics.
    pub label: &'static str,
}

/// The integer leap-second table, ascending by `start_unix`.
///
/// Source: the IANA `leap-seconds.list` distributed with the time zone
/// database, <https://data.iana.org/time-zones/tzdb/leap-seconds.list>,
/// which is updated through IERS Bulletin C (`iana-leap-seconds-list`).
/// Checked 2026-09-26 against the copy whose `#$` line, its last update,
/// is NTP 3 992 312 697 (2026-07-06) and whose `#@` expiry is NTP
/// 4 023 129 600, 28 June 2027 (see [`table_valid_until_unix`]). The last
/// entry is 2017-01-01; no leap second has been announced since.
///
/// The 27th CGPM (2022), Resolution 4, decided that the maximum value of
/// UT1 − UTC will be increased in, or before, 2035 (`cgpm2022-res4`, read
/// 2026-09-26 at <https://www.bipm.org/en/cgpm-2022/resolution-4>). That
/// will change this table's future, not its past: the data changes and the
/// conversion code does not — which is the whole reason this is a table.
pub const TABLE: &[LeapEntry] = &[
    LeapEntry {
        start_unix: 63_072_000,
        tai_minus_utc: 10,
        label: "1972-01-01",
    },
    LeapEntry {
        start_unix: 78_796_800,
        tai_minus_utc: 11,
        label: "1972-07-01",
    },
    LeapEntry {
        start_unix: 94_694_400,
        tai_minus_utc: 12,
        label: "1973-01-01",
    },
    LeapEntry {
        start_unix: 126_230_400,
        tai_minus_utc: 13,
        label: "1974-01-01",
    },
    LeapEntry {
        start_unix: 157_766_400,
        tai_minus_utc: 14,
        label: "1975-01-01",
    },
    LeapEntry {
        start_unix: 189_302_400,
        tai_minus_utc: 15,
        label: "1976-01-01",
    },
    LeapEntry {
        start_unix: 220_924_800,
        tai_minus_utc: 16,
        label: "1977-01-01",
    },
    LeapEntry {
        start_unix: 252_460_800,
        tai_minus_utc: 17,
        label: "1978-01-01",
    },
    LeapEntry {
        start_unix: 283_996_800,
        tai_minus_utc: 18,
        label: "1979-01-01",
    },
    LeapEntry {
        start_unix: 315_532_800,
        tai_minus_utc: 19,
        label: "1980-01-01",
    },
    LeapEntry {
        start_unix: 362_793_600,
        tai_minus_utc: 20,
        label: "1981-07-01",
    },
    LeapEntry {
        start_unix: 394_329_600,
        tai_minus_utc: 21,
        label: "1982-07-01",
    },
    LeapEntry {
        start_unix: 425_865_600,
        tai_minus_utc: 22,
        label: "1983-07-01",
    },
    LeapEntry {
        start_unix: 489_024_000,
        tai_minus_utc: 23,
        label: "1985-07-01",
    },
    LeapEntry {
        start_unix: 567_993_600,
        tai_minus_utc: 24,
        label: "1988-01-01",
    },
    LeapEntry {
        start_unix: 631_152_000,
        tai_minus_utc: 25,
        label: "1990-01-01",
    },
    LeapEntry {
        start_unix: 662_688_000,
        tai_minus_utc: 26,
        label: "1991-01-01",
    },
    LeapEntry {
        start_unix: 709_948_800,
        tai_minus_utc: 27,
        label: "1992-07-01",
    },
    LeapEntry {
        start_unix: 741_484_800,
        tai_minus_utc: 28,
        label: "1993-07-01",
    },
    LeapEntry {
        start_unix: 773_020_800,
        tai_minus_utc: 29,
        label: "1994-07-01",
    },
    LeapEntry {
        start_unix: 820_454_400,
        tai_minus_utc: 30,
        label: "1996-01-01",
    },
    LeapEntry {
        start_unix: 867_715_200,
        tai_minus_utc: 31,
        label: "1997-07-01",
    },
    LeapEntry {
        start_unix: 915_148_800,
        tai_minus_utc: 32,
        label: "1999-01-01",
    },
    LeapEntry {
        start_unix: 1_136_073_600,
        tai_minus_utc: 33,
        label: "2006-01-01",
    },
    LeapEntry {
        start_unix: 1_230_768_000,
        tai_minus_utc: 34,
        label: "2009-01-01",
    },
    LeapEntry {
        start_unix: 1_341_100_800,
        tai_minus_utc: 35,
        label: "2012-07-01",
    },
    LeapEntry {
        start_unix: 1_435_708_800,
        tai_minus_utc: 36,
        label: "2015-07-01",
    },
    LeapEntry {
        start_unix: 1_483_228_800,
        tai_minus_utc: 37,
        label: "2017-01-01",
    },
];

/// The 1961-1971 rate-offset era, ascending by `start_unix`.
///
/// Before 1972, UTC was kept close to UT1 by running its seconds at a
/// deliberately different *rate* and applying occasional fractional steps, so
/// `TAI - UTC` is piecewise linear rather than an integer. The coefficients
/// are the `tai-utc.dat` series published by the USNO,
/// <https://maia.usno.navy.mil/ser7/tai-utc.dat> (`usno-tai-utc`), and the
/// same thirteen segments, with the same coefficients, in the IERS Bulletin C
/// history, <https://hpiers.obspm.fr/iers/bul/bulc/UTC-TAI.history>
/// (`iers-utc-tai-history`); both were read on 2026-10-03, and every
/// segment's start date below is the date they give, with the Julian Date
/// the USNO file gives for it:
///
/// | Start | JD | `TAI − UTC` at the start |
/// | --- | --- | --- |
/// | 1961-01-01 | 2 437 300.5 | 1.422 818 0 s + (MJD − 37 300) × 0.001 296 s |
/// | 1961-08-01 | 2 437 512.5 | 1.372 818 0 s, same line |
/// | 1962-01-01 | 2 437 665.5 | 1.845 858 0 s + (MJD − 37 665) × 0.001 123 2 s |
/// | 1963-11-01 | 2 438 334.5 | 1.945 858 0 s, same line |
/// | 1964-01-01 | 2 438 395.5 | 3.240 130 0 s + (MJD − 38 761) × 0.001 296 s |
/// | 1964-04-01 | 2 438 486.5 | 3.340 130 0 s, same line |
/// | 1964-09-01 | 2 438 639.5 | 3.440 130 0 s, same line |
/// | 1965-01-01 | 2 438 761.5 | 3.540 130 0 s, same line |
/// | 1965-03-01 | 2 438 820.5 | 3.640 130 0 s, same line |
/// | 1965-07-01 | 2 438 942.5 | 3.740 130 0 s, same line |
/// | 1965-09-01 | 2 439 004.5 | 3.840 130 0 s, same line |
/// | 1966-01-01 | 2 439 126.5 | 4.313 170 0 s + (MJD − 39 126) × 0.002 592 s |
/// | 1968-02-01 | 2 439 887.5 | 4.213 170 0 s, same line |
///
/// which gives the 8.000 082 s of 1970-01-01 and 9.892 242 s at the end of
/// 1971-12-31. Each `start_unix` is `(MJD − 40 587) × 86 400` of the JD
/// above, and a test recomputes all thirteen from that column. The steps
/// between segments are −0.05 s (1961-08-01), +0.1 s (1963-11-01, 1964-04-01,
/// 1964-09-01, 1965-01-01, 1965-03-01, 1965-07-01, 1965-09-01), −0.1 s
/// (1968-02-01), none at 1962-01-01, 1964-01-01 and 1966-01-01, where the
/// new line continues the old one, and +0.107 758 s at 1972-01-01 where the
/// integer era begins. None of them is a leap second: no second was inserted
/// before 1972-06-30.
///
/// A UTC second in this era was not an SI second, so conversions here are
/// exact in the sense that they reproduce the published relation, not in the
/// sense that elapsed-time arithmetic across the boundary is unsurprising.
pub const RATE_ERA: &[RateEntry] = &[
    RateEntry {
        start_unix: -283_996_800,
        offset: 1.422_818_0,
        origin_mjd: 37_300.0,
        drift: 0.001_296,
        label: "1961-01-01",
    },
    RateEntry {
        start_unix: -265_680_000,
        offset: 1.372_818_0,
        origin_mjd: 37_300.0,
        drift: 0.001_296,
        label: "1961-08-01",
    },
    RateEntry {
        start_unix: -252_460_800,
        offset: 1.845_858_0,
        origin_mjd: 37_665.0,
        drift: 0.001_123_2,
        label: "1962-01-01",
    },
    RateEntry {
        start_unix: -194_659_200,
        offset: 1.945_858_0,
        origin_mjd: 37_665.0,
        drift: 0.001_123_2,
        label: "1963-11-01",
    },
    RateEntry {
        start_unix: -189_388_800,
        offset: 3.240_130_0,
        origin_mjd: 38_761.0,
        drift: 0.001_296,
        label: "1964-01-01",
    },
    RateEntry {
        start_unix: -181_526_400,
        offset: 3.340_130_0,
        origin_mjd: 38_761.0,
        drift: 0.001_296,
        label: "1964-04-01",
    },
    RateEntry {
        start_unix: -168_307_200,
        offset: 3.440_130_0,
        origin_mjd: 38_761.0,
        drift: 0.001_296,
        label: "1964-09-01",
    },
    RateEntry {
        start_unix: -157_766_400,
        offset: 3.540_130_0,
        origin_mjd: 38_761.0,
        drift: 0.001_296,
        label: "1965-01-01",
    },
    RateEntry {
        start_unix: -152_668_800,
        offset: 3.640_130_0,
        origin_mjd: 38_761.0,
        drift: 0.001_296,
        label: "1965-03-01",
    },
    RateEntry {
        start_unix: -142_128_000,
        offset: 3.740_130_0,
        origin_mjd: 38_761.0,
        drift: 0.001_296,
        label: "1965-07-01",
    },
    RateEntry {
        start_unix: -136_771_200,
        offset: 3.840_130_0,
        origin_mjd: 38_761.0,
        drift: 0.001_296,
        label: "1965-09-01",
    },
    RateEntry {
        start_unix: -126_230_400,
        offset: 4.313_170_0,
        origin_mjd: 39_126.0,
        drift: 0.002_592,
        label: "1966-01-01",
    },
    RateEntry {
        start_unix: -60_480_000,
        offset: 4.213_170_0,
        origin_mjd: 39_126.0,
        drift: 0.002_592,
        label: "1968-02-01",
    },
];

/// The last Unix second for which the integer table is authoritative.
///
/// The IERS announces leap seconds about six months ahead, so a conversion
/// past this point is a forecast. The value is the announced validity of the
/// current Bulletin C rather than the last table entry.
#[must_use]
pub const fn table_valid_until_unix() -> i64 {
    // 2027-06-28T00:00:00Z, the expiry declared by the `#@` line of the IANA
    // `leap-seconds.list` this table was built from. Update it together with
    // `TABLE`, and never past what the current file actually claims.
    1_814_140_800
}

/// The first Unix second at which the integer leap-second regime applies.
#[must_use]
pub const fn integer_era_start_unix() -> i64 {
    63_072_000
}

/// The first Unix second at which UTC is defined at all.
#[must_use]
pub const fn utc_start_unix() -> i64 {
    -283_996_800
}

/// Every Unix second at which a leap second was inserted or removed.
///
/// A positive value means a second was inserted (a `23:59:60` exists just
/// before it); a negative value would mean one was removed. No negative leap
/// second has ever been scheduled.
pub fn steps() -> impl Iterator<Item = (i64, i64)> {
    TABLE.windows(2).map(|pair| {
        let [before, after] = [pair[0], pair[1]];
        (after.start_unix, after.tai_minus_utc - before.tai_minus_utc)
    })
}

/// The leap second that ends the UTC day `unix_day`, counted in days from
/// 1970-01-01: +1 when a second 23:59:60 was inserted before the next
/// midnight, −1 when 23:59:59 was omitted, and 0 otherwise.
///
/// Every code that writes UTC with its leap second needs this check, since
/// the code itself cannot say whether a day ended in one. Before 1972 no
/// whole second was ever inserted, and the answer is 0.
///
/// # Errors
///
/// [`crate::TimeError::AfterModelEnd`] when the day ends after
/// [`table_valid_until_unix`], where whether it ends in a leap second has
/// not been announced, and [`crate::TimeError::Overflow`] for a day whose
/// end is not a representable POSIX second.
pub fn end_of_day_step(unix_day: i64) -> crate::TimeResult<i64> {
    let end = unix_day
        .checked_add(1)
        .and_then(crate::duration::seconds_in_days)
        .ok_or(crate::TimeError::Overflow)?;
    if end > table_valid_until_unix() {
        return Err(crate::TimeError::AfterModelEnd);
    }
    Ok(steps()
        .find(|(at, _)| *at == end)
        .map_or(0, |(_, delta)| delta))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_is_sorted_and_monotonic() {
        for pair in TABLE.windows(2) {
            assert!(pair[0].start_unix < pair[1].start_unix, "{:?}", pair);
            assert!(pair[0].tai_minus_utc < pair[1].tai_minus_utc, "{:?}", pair);
        }
    }

    /// The USNO `tai-utc.dat` rows of 1961-1972, read 2026-10-03 at
    /// <https://maia.usno.navy.mil/ser7/tai-utc.dat>, and the IERS
    /// `UTC-TAI.history`, read the same day, which gives the same dates
    /// and coefficients: the start date, its Julian Date, `TAI − UTC` at
    /// the start of the line in seconds, the MJD it is measured from and
    /// its drift in seconds per day. The 1972 row ends the era.
    const USNO: [(&str, f64, f64, f64, f64); 14] = [
        ("1961-01-01", 2_437_300.5, 1.422_818_0, 37_300.0, 0.001_296),
        ("1961-08-01", 2_437_512.5, 1.372_818_0, 37_300.0, 0.001_296),
        (
            "1962-01-01",
            2_437_665.5,
            1.845_858_0,
            37_665.0,
            0.001_123_2,
        ),
        (
            "1963-11-01",
            2_438_334.5,
            1.945_858_0,
            37_665.0,
            0.001_123_2,
        ),
        ("1964-01-01", 2_438_395.5, 3.240_130_0, 38_761.0, 0.001_296),
        ("1964-04-01", 2_438_486.5, 3.340_130_0, 38_761.0, 0.001_296),
        ("1964-09-01", 2_438_639.5, 3.440_130_0, 38_761.0, 0.001_296),
        ("1965-01-01", 2_438_761.5, 3.540_130_0, 38_761.0, 0.001_296),
        ("1965-03-01", 2_438_820.5, 3.640_130_0, 38_761.0, 0.001_296),
        ("1965-07-01", 2_438_942.5, 3.740_130_0, 38_761.0, 0.001_296),
        ("1965-09-01", 2_439_004.5, 3.840_130_0, 38_761.0, 0.001_296),
        ("1966-01-01", 2_439_126.5, 4.313_170_0, 39_126.0, 0.002_592),
        ("1968-02-01", 2_439_887.5, 4.213_170_0, 39_126.0, 0.002_592),
        ("1972-01-01", 2_441_317.5, 10.0, 41_317.0, 0.0),
    ];

    /// Audit 10, a11: eight of the thirteen start dates were another day.
    /// Every segment starts on its label's day, the day the two published
    /// tables give, and carries their coefficients.
    #[test]
    fn every_rate_segment_starts_on_the_day_the_published_table_gives() {
        assert_eq!(RATE_ERA.len(), USNO.len() - 1);
        for (entry, (label, jd, offset, origin, drift)) in RATE_ERA.iter().zip(USNO) {
            assert_eq!(entry.label, label);
            // JD 2 437 300.5 is MJD 37 300, and the epoch is MJD 40 587.
            let mjd = jd - 2_400_000.5;
            assert_eq!(mjd.fract(), 0.0, "{label}");
            assert_eq!(
                entry.start_unix,
                (mjd as i64 - 40_587) * 86_400,
                "{label}: starts on another day"
            );
            assert_eq!(entry.offset, offset, "{label}");
            assert_eq!(entry.origin_mjd, origin, "{label}");
            assert_eq!(entry.drift, drift, "{label}");
        }
        assert_eq!(integer_era_start_unix(), (41_317 - 40_587) * 86_400);
        assert_eq!(utc_start_unix(), RATE_ERA[0].start_unix);
    }

    /// The start days counted by hand from the length of the months and
    /// years, not from the Julian Dates: the check the table above cannot
    /// give if a Julian Date were mistyped with its date.
    #[test]
    fn a_few_segment_starts_are_the_calendar_days_they_are_named_for() {
        let day = |label: &str| {
            RATE_ERA
                .iter()
                .find(|entry| entry.label == label)
                .unwrap()
                .start_unix
                / 86_400
        };
        // 1970-01-01 is day 0; 1965-01-01 is the five years 1965 to 1969
        // before it, of which 1968 is a leap year.
        assert_eq!(day("1965-01-01"), -(365 + 365 + 366 + 365 + 365));
        assert_eq!(day("1964-01-01"), day("1965-01-01") - 366);
        assert_eq!(day("1964-04-01"), day("1964-01-01") + 31 + 29 + 31);
        assert_eq!(
            day("1964-09-01"),
            day("1964-04-01") + 30 + 31 + 30 + 31 + 31
        );
        assert_eq!(day("1965-03-01"), day("1965-01-01") + 31 + 28);
        assert_eq!(day("1965-07-01"), day("1965-03-01") + 31 + 30 + 31 + 30);
        assert_eq!(day("1965-09-01"), day("1965-07-01") + 31 + 31);
        assert_eq!(
            day("1961-08-01"),
            day("1961-01-01") + 31 + 28 + 31 + 30 + 31 + 30 + 31
        );
    }

    #[test]
    fn rate_era_is_sorted() {
        for pair in RATE_ERA.windows(2) {
            assert!(pair[0].start_unix < pair[1].start_unix, "{:?}", pair);
        }
    }

    #[test]
    fn every_modern_step_is_a_single_inserted_second() {
        for (_, delta) in steps() {
            assert_eq!(delta, 1);
        }
    }

    #[test]
    fn the_table_starts_at_ten_seconds() {
        assert_eq!(TABLE[0].tai_minus_utc, 10);
        assert_eq!(TABLE[0].start_unix, integer_era_start_unix());
    }

    #[test]
    fn the_table_ends_at_thirty_seven_seconds() {
        let last = TABLE[TABLE.len() - 1];
        assert_eq!(last.tai_minus_utc, 37);
        assert_eq!(last.label, "2017-01-01");
    }

    /// 2016-12-31 ended in the last inserted second; the day before did
    /// not, a day of 1970 cannot have, and a day past the table's validity
    /// is not yet known.
    #[test]
    fn the_end_of_a_day_is_read_from_the_table() {
        assert_eq!(end_of_day_step(17_166), Ok(1), "2016-12-31");
        assert_eq!(end_of_day_step(17_165), Ok(0), "2016-12-30");
        assert_eq!(end_of_day_step(364), Ok(0), "1970-12-31");
        let last_known = crate::duration::days_and_seconds(table_valid_until_unix()).0 - 1;
        assert_eq!(end_of_day_step(last_known), Ok(0));
        assert_eq!(
            end_of_day_step(last_known + 1),
            Err(crate::TimeError::AfterModelEnd)
        );
        assert_eq!(end_of_day_step(i64::MAX), Err(crate::TimeError::Overflow));
    }
}
