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
/// database, which mirrors IERS Bulletin C. The last entry is 2017-01-01; no
/// leap second has been announced since, and the file checked when this table
/// was written declared an expiry of 2027-06-28 (see
/// [`table_valid_until_unix`]).
///
/// The 2022 CGPM resolution to retire the leap second by 2035 will change
/// this table's future, not its past. When it lands, the data changes and the
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
/// are the `tai-utc.dat` series published by the USNO and mirrored by the
/// IERS Earth Orientation Centre.
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
        start_unix: -272_505_600,
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
        start_unix: -189_302_400,
        offset: 3.240_130_0,
        origin_mjd: 38_761.0,
        drift: 0.001_296,
        label: "1964-01-01",
    },
    RateEntry {
        start_unix: -181_353_600,
        offset: 3.340_130_0,
        origin_mjd: 38_761.0,
        drift: 0.001_296,
        label: "1964-04-01",
    },
    RateEntry {
        start_unix: -173_404_800,
        offset: 3.440_130_0,
        origin_mjd: 38_761.0,
        drift: 0.001_296,
        label: "1964-09-01",
    },
    RateEntry {
        start_unix: -165_542_400,
        offset: 3.540_130_0,
        origin_mjd: 38_761.0,
        drift: 0.001_296,
        label: "1965-01-01",
    },
    RateEntry {
        start_unix: -157_680_000,
        offset: 3.640_130_0,
        origin_mjd: 38_761.0,
        drift: 0.001_296,
        label: "1965-03-01",
    },
    RateEntry {
        start_unix: -152_409_600,
        offset: 3.740_130_0,
        origin_mjd: 38_761.0,
        drift: 0.001_296,
        label: "1965-07-01",
    },
    RateEntry {
        start_unix: -142_128_000,
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
}
