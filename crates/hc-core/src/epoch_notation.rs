//! Julian and Besselian epochs: an instant written as a year and a
//! fraction, "J2000.0", "B1950.0".
//!
//! IAU SOFA Board, *SOFA Time Scale and Calendar Tools*, document revision
//! 1.4, 2016, §2.4, read 2026-09-26 (`sofa-ts`):
//!
//! - **The Julian epoch**, `julian-epoch`, "uses the Julian year of
//!   exactly 365.25 days, and the TT time scale; Julian epoch 2000 is
//!   defined to be 2000 January 1.5, which is JD 2451545.0". So
//!   J = 2000.0 + (JD(TT) − 2 451 545.0) / 365.25, exactly, and
//!   [`julian_epoch`] computes it from an [`Instant<Tt>`].
//! - **The Besselian epoch**, `besselian-epoch`: "The unit is tropical
//!   years (about 365.2422 days), the time scale is ephemeris time … and
//!   the Besselian year begins when the ecliptic longitude of the mean Sun
//!   is 280°". The document gives no constants. ERFA's `eraEpb`, whose
//!   library "is intended to retain identical functionality to the SOFA
//!   library", read 2026-09-26 (`erfa-epb`, BSD-3-Clause), gives the
//!   conventional ones,
//!   which it credits to Lieske (1979, not read): "Besselian Epoch B1900.0
//!   is JD 2415020.31352 and the length of the year is 365.242198781
//!   days", on TDB, "which for all practical purposes in the present
//!   context is indistinguishable from TT". So
//!   B = 1900.0 + (JD − 2 415 020.313 52) / 365.242 198 781, and
//!   [`besselian_epoch`] takes TT for the JD, as `eraEpb` allows.
//! - **Without a letter**, "it can be assumed that epochs before 1984.0 are
//!   Besselian and, from 1984.0 on, Julian": [`EpochKind::unprefixed`].
//!
//! The epoch is an `f64`, as SOFA's is. Near the present it resolves about
//! 10⁻¹³ of a year, some 3 µs; the conversions do their arithmetic on the
//! exact span from J2000.0 and round only at the end.

use crate::duration::Duration;
use crate::error::TimeResult;
use crate::scale::{Instant, Tt};

/// J2000.0, 2000-01-01T12:00:00 TT, as a TT reading from 1970 TT.
const J2000_TT: Duration = Duration::from_secs(946_728_000);

/// Days in a Julian year.
pub const JULIAN_YEAR_DAYS: f64 = 365.25;

/// Days in the Besselian (tropical) year of `eraEpb`.
pub const BESSELIAN_YEAR_DAYS: f64 = 365.242_198_781;

/// Days from B1900.0, JD 2 415 020.313 52, to J2000.0, `eraEpb`'s
/// `D1900`.
pub const B1900_BEFORE_J2000_DAYS: f64 = 36_524.686_48;

/// Which of the two notations an epoch is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EpochKind {
    /// Julian years of 365.25 days of TT, written `J`.
    Julian,
    /// Besselian years of 365.242 198 781 days, written `B`.
    Besselian,
}

impl EpochKind {
    /// The kind SOFA says an epoch written without a letter is: Besselian
    /// before 1984.0, Julian from it.
    #[must_use]
    pub fn unprefixed(year: f64) -> Self {
        if year < 1984.0 {
            Self::Besselian
        } else {
            Self::Julian
        }
    }

    /// The letter that writes this kind.
    #[must_use]
    pub const fn letter(self) -> char {
        match self {
            Self::Julian => 'J',
            Self::Besselian => 'B',
        }
    }

    /// The epoch of an instant in this notation.
    #[must_use]
    pub fn epoch(self, tt: Instant<Tt>) -> f64 {
        match self {
            Self::Julian => julian_epoch(tt),
            Self::Besselian => besselian_epoch(tt),
        }
    }

    /// The instant of an epoch in this notation.
    ///
    /// # Errors
    ///
    /// As [`from_julian_epoch`].
    pub fn instant(self, year: f64) -> TimeResult<Instant<Tt>> {
        match self {
            Self::Julian => from_julian_epoch(year),
            Self::Besselian => from_besselian_epoch(year),
        }
    }
}

/// Days of TT from J2000.0.
fn days_from_j2000(tt: Instant<Tt>) -> f64 {
    tt.since_epoch()
        .checked_sub(J2000_TT)
        .map_or(f64::NAN, Duration::as_days_f64)
}

fn from_days_from_j2000(days: f64) -> TimeResult<Instant<Tt>> {
    let span = Duration::from_days_f64(days)?;
    Ok(Instant::from_epoch(J2000_TT.checked_add(span)?))
}

/// The Julian epoch of a TT instant: 2000.0 + days from J2000.0 / 365.25.
#[must_use]
pub fn julian_epoch(tt: Instant<Tt>) -> f64 {
    2000.0 + days_from_j2000(tt) / JULIAN_YEAR_DAYS
}

/// The TT instant of a Julian epoch.
///
/// # Errors
///
/// [`crate::TimeError::NotFinite`] for a year that is not finite, and
/// [`crate::TimeError::Overflow`] for one too far away to represent.
pub fn from_julian_epoch(year: f64) -> TimeResult<Instant<Tt>> {
    from_days_from_j2000((year - 2000.0) * JULIAN_YEAR_DAYS)
}

/// The Besselian epoch of an instant, TT taken for TDB:
/// 1900.0 + days from B1900.0 / 365.242 198 781.
#[must_use]
pub fn besselian_epoch(tt: Instant<Tt>) -> f64 {
    1900.0 + (days_from_j2000(tt) + B1900_BEFORE_J2000_DAYS) / BESSELIAN_YEAR_DAYS
}

/// The instant of a Besselian epoch, on TT.
///
/// # Errors
///
/// As [`from_julian_epoch`].
pub fn from_besselian_epoch(year: f64) -> TimeResult<Instant<Tt>> {
    from_days_from_j2000((year - 1900.0) * BESSELIAN_YEAR_DAYS - B1900_BEFORE_J2000_DAYS)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The instant of a Julian Date of TT.
    fn tt_of_jd(jd: f64) -> Instant<Tt> {
        from_days_from_j2000(jd - 2_451_545.0).expect("representable")
    }

    /// SOFA, §2.4: JD 2457073.05631 is B2015.1365941021 and
    /// J2015.1349933196, and each converts back to 2457073.056310000.
    #[test]
    fn sofa_s_example() {
        let tt = tt_of_jd(2_457_073.056_31);
        let julian = julian_epoch(tt);
        let besselian = besselian_epoch(tt);
        assert!((julian - 2_015.134_993_319_6).abs() < 1e-10, "{julian}");
        assert!(
            (besselian - 2_015.136_594_102_1).abs() < 1e-10,
            "{besselian}"
        );
        for (kind, year) in [
            (EpochKind::Julian, julian),
            (EpochKind::Besselian, besselian),
        ] {
            let back = kind.instant(year).expect("representable");
            let days = back
                .since_epoch()
                .checked_sub(tt.since_epoch())
                .expect("fits")
                .as_days_f64();
            assert!(days.abs() < 1e-9, "{kind:?} {days}");
        }
    }

    /// J2000.0 is 2000 January 1.5 TT, and B1900.0 is JD 2415020.31352.
    #[test]
    fn the_defining_epochs() {
        let j2000: Instant<Tt> = crate::epoch::J2000.instant().convert();
        assert_eq!(julian_epoch(j2000), 2000.0);
        assert_eq!(from_julian_epoch(2000.0), Ok(j2000));
        let b1900 = tt_of_jd(2_415_020.313_52);
        assert!((besselian_epoch(b1900) - 1900.0).abs() < 1e-12);
        assert!((2_451_545.0 - B1900_BEFORE_J2000_DAYS - 2_415_020.313_52).abs() < 1e-9);
    }

    #[test]
    fn an_epoch_without_a_letter_is_besselian_before_1984() {
        assert_eq!(EpochKind::unprefixed(1950.0), EpochKind::Besselian);
        assert_eq!(EpochKind::unprefixed(1983.999), EpochKind::Besselian);
        assert_eq!(EpochKind::unprefixed(1984.0), EpochKind::Julian);
        assert_eq!(EpochKind::unprefixed(2000.0).letter(), 'J');
    }

    #[test]
    fn a_non_finite_year_is_refused() {
        assert_eq!(
            from_julian_epoch(f64::NAN),
            Err(crate::TimeError::NotFinite)
        );
        assert!(from_besselian_epoch(f64::INFINITY).is_err());
    }
}
