//! Uniform physical time scales and the instants read on them.
//!
//! A "time scale" here is a way of labelling events along the same physical
//! timeline. Converting between two of them never moves the event; it only
//! re-reads the clock. Keeping that distinction in the type system is why
//! [`Instant`] carries a scale marker.
//!
//! UTC is deliberately *not* one of these markers. UTC is a scale whose
//! seconds are SI seconds but whose labelling occasionally repeats or skips a
//! second, so it cannot be a simple offset from TAI. It lives in
//! [`crate::leap`] and [`crate::unix`] instead.

use core::cmp::Ordering;
use core::fmt;
use core::marker::PhantomData;

use crate::duration::Duration;
use crate::error::TimeResult;
use crate::math;

/// Runtime identifier for a time scale, for FFI and dynamic dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum TimeScaleId {
    /// International Atomic Time, the practical realisation of proper time on
    /// the rotating geoid.
    Tai,
    /// Terrestrial Time, TAI advanced by exactly 32.184 s.
    Tt,
    /// Geocentric Coordinate Time.
    Tcg,
    /// Barycentric Dynamical Time.
    Tdb,
    /// Barycentric Coordinate Time.
    Tcb,
    /// GPS time, TAI retarded by exactly 19 s.
    Gps,
    /// Universal Time, tied to the Earth's actual rotation angle. Its
    /// marker type lives in `hc-astro`, which has the ΔT model it needs.
    Ut1,
    /// Coordinated Universal Time — an atomic scale with leap seconds.
    Utc,
}

impl TimeScaleId {
    /// The conventional short name, as used in IERS and IAU documents.
    #[must_use]
    pub const fn abbreviation(self) -> &'static str {
        match self {
            Self::Tai => "TAI",
            Self::Tt => "TT",
            Self::Tcg => "TCG",
            Self::Tdb => "TDB",
            Self::Tcb => "TCB",
            Self::Gps => "GPS",
            Self::Ut1 => "UT1",
            Self::Utc => "UTC",
        }
    }
}

impl fmt::Display for TimeScaleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.abbreviation())
    }
}

/// A time scale whose reading is a function of the TAI reading of the same
/// event: an exact offset for TT and GPS, a smooth model for TCG, TDB and
/// TCB.
///
/// Implementors are zero-sized markers. The two required functions are
/// inverses of each other to within the precision of the underlying model.
///
/// UT1, the scale of the Earth's rotation, is measured rather than defined,
/// so it is not here: `hc-astro` implements it from its ΔT model, beside the
/// published `UT1 − UTC` series that give it exactly.
pub trait TimeScale: Copy + Clone + fmt::Debug + 'static {
    /// The runtime identifier for this scale.
    const ID: TimeScaleId;

    /// Read this scale at the event whose TAI reading is `tai`.
    ///
    /// Both values are measured from `1970-01-01T00:00:00` *as labelled in
    /// their own scale*, which is why a constant-offset scale such as TT has
    /// a constant offset here too.
    fn from_tai(tai: Duration) -> Duration;

    /// Read TAI at the event whose reading on this scale is `value`.
    fn to_tai(value: Duration) -> Duration;
}

/// TAI — International Atomic Time. The canonical scale of this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Tai;

/// TT — Terrestrial Time. `TT = TAI + 32.184 s`, exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Tt;

/// TCG — Geocentric Coordinate Time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Tcg;

/// TDB — Barycentric Dynamical Time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Tdb;

/// TCB — Barycentric Coordinate Time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Tcb;

/// GPS time. `GPS = TAI - 19 s`, exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Gps;

/// `TT - TAI`, an exact defined constant.
pub const TT_MINUS_TAI: Duration = Duration::from_attos(32_184_000_000_000_000_000);

/// `TAI - GPS`, an exact defined constant.
pub const TAI_MINUS_GPS: Duration = Duration::from_secs(19);

/// `L_G`, the defining constant relating TCG to TT (IAU 2000 Resolution B1.9).
pub const L_G: f64 = 6.969_290_134e-10;

/// `L_B`, the defining constant relating TCB to TDB (IAU 2006 Resolution B3).
pub const L_B: f64 = 1.550_519_768e-8;

/// `TDB_0`, the defining constant offset of TDB (IAU 2006 Resolution B3).
pub const TDB_0: f64 = -6.55e-5;

/// The TCG/TCB origin `1977-01-01T00:00:00 TAI`, as a TAI reading measured
/// from the 1970 TAI epoch.
const T0_TAI_SECS: f64 = 220_924_800.0;

impl TimeScale for Tai {
    const ID: TimeScaleId = TimeScaleId::Tai;

    fn from_tai(tai: Duration) -> Duration {
        tai
    }

    fn to_tai(value: Duration) -> Duration {
        value
    }
}

impl TimeScale for Tt {
    const ID: TimeScaleId = TimeScaleId::Tt;

    fn from_tai(tai: Duration) -> Duration {
        tai.checked_add(TT_MINUS_TAI).unwrap_or(Duration::MAX)
    }

    fn to_tai(value: Duration) -> Duration {
        value.checked_sub(TT_MINUS_TAI).unwrap_or(Duration::MIN)
    }
}

impl TimeScale for Gps {
    const ID: TimeScaleId = TimeScaleId::Gps;

    fn from_tai(tai: Duration) -> Duration {
        tai.checked_sub(TAI_MINUS_GPS).unwrap_or(Duration::MIN)
    }

    fn to_tai(value: Duration) -> Duration {
        value.checked_add(TAI_MINUS_GPS).unwrap_or(Duration::MAX)
    }
}

/// `TCG - TT` in seconds, given a TT reading in seconds from the 1970 epoch.
fn tcg_minus_tt_secs(tt_secs: f64) -> f64 {
    let elapsed = tt_secs - (T0_TAI_SECS + 32.184);
    L_G / (1.0 - L_G) * elapsed
}

impl TimeScale for Tcg {
    const ID: TimeScaleId = TimeScaleId::Tcg;

    fn from_tai(tai: Duration) -> Duration {
        let tt = Tt::from_tai(tai);
        let delta = tcg_minus_tt_secs(tt.as_secs_f64());
        add_small_offset(tt, delta)
    }

    fn to_tai(value: Duration) -> Duration {
        // TCG - TT is linear in TT, so the inverse is closed-form:
        // TCG - TT = L_G * (TCG - origin).
        let delta = L_G * (value.as_secs_f64() - (T0_TAI_SECS + 32.184));
        Tt::to_tai(add_small_offset(value, -delta))
    }
}

/// Apply a small, model-derived offset to an exact reading.
///
/// The offsets between coordinate time scales are small against the reading
/// itself — at most about 1.7 ms for TDB, and seconds to tens of seconds for
/// TCG and TCB over the modern era — so they can be computed in `f64`
/// without meaningful loss. Doing
/// the *addition* on [`Duration`] instead of on `f64` seconds keeps the full
/// attosecond precision of the original reading, which a round trip through
/// `f64` would destroy.
fn add_small_offset(base: Duration, offset_secs: f64) -> Duration {
    match Duration::from_secs_f64(offset_secs) {
        Ok(delta) => base.checked_add(delta).unwrap_or(base),
        Err(_) => base,
    }
}

/// `TDB - TT` in seconds, from the conventional Fairhead & Bretagnon
/// truncated series. Accurate to roughly 30 µs over 1980-2100.
///
/// `tt_secs` is a TT reading in seconds from the 1970 epoch.
#[must_use]
pub fn tdb_minus_tt_secs(tt_secs: f64) -> f64 {
    // Julian centuries of TT from J2000.0, whose TT reading on this epoch is
    // exactly 946_728_000 s. See `crate::epoch::J2000`.
    let centuries = (tt_secs - 946_728_000.0) / (36_525.0 * 86_400.0);
    0.001_657 * math::sin(628.307_6 * centuries + 6.240_1)
        + 0.000_022 * math::sin(575.338_5 * centuries + 4.297_0)
        + 0.000_014 * math::sin(1_256.615_2 * centuries + 6.196_9)
        + 0.000_005 * math::sin(606.977_7 * centuries + 4.021_2)
        + 0.000_005 * math::sin(52.969_1 * centuries + 0.444_4)
        + 0.000_002 * math::sin(21.329_9 * centuries + 5.543_1)
        + 0.000_010 * centuries * math::sin(628.307_6 * centuries + 4.249_0)
}

impl TimeScale for Tdb {
    const ID: TimeScaleId = TimeScaleId::Tdb;

    fn from_tai(tai: Duration) -> Duration {
        let tt = Tt::from_tai(tai);
        add_small_offset(tt, tdb_minus_tt_secs(tt.as_secs_f64()))
    }

    fn to_tai(value: Duration) -> Duration {
        // The periodic term is at most ~1.7 ms and its derivative is tiny, so
        // a fixed-point iteration on the offset converges immediately.
        let tdb_secs = value.as_secs_f64();
        let mut delta = 0.0;
        for _ in 0..3 {
            delta = tdb_minus_tt_secs(tdb_secs - delta);
        }
        Tt::to_tai(add_small_offset(value, -delta))
    }
}

impl TimeScale for Tcb {
    const ID: TimeScaleId = TimeScaleId::Tcb;

    fn from_tai(tai: Duration) -> Duration {
        let tt = Tt::from_tai(tai);
        let tt_secs = tt.as_secs_f64();
        let elapsed = tt_secs - (T0_TAI_SECS + 32.184);
        let delta = tdb_minus_tt_secs(tt_secs) + L_B * elapsed - TDB_0;
        add_small_offset(tt, delta)
    }

    fn to_tai(value: Duration) -> Duration {
        let tcb_secs = value.as_secs_f64();
        let origin = T0_TAI_SECS + 32.184;
        // TCB - TT is a linear secular term plus the small TDB periodic one;
        // iterate on the offset rather than on the absolute reading.
        let mut delta = 0.0;
        for _ in 0..4 {
            let tt_secs = tcb_secs - delta;
            delta = tdb_minus_tt_secs(tt_secs) + L_B * (tt_secs - origin) - TDB_0;
        }
        Tt::to_tai(add_small_offset(value, -delta))
    }
}

/// A reading on the uniform time scale `S`, measured from
/// `1970-01-01T00:00:00` as labelled in `S` itself.
///
/// ```
/// use hc_core::{Duration, Instant, Tai, Tt};
///
/// let launch = Instant::<Tai>::from_epoch(Duration::from_secs(0));
/// let same_event: Instant<Tt> = launch.convert();
/// assert_eq!(same_event.since_epoch(), Duration::from_millis(32_184));
/// ```
pub struct Instant<S: TimeScale> {
    since_epoch: Duration,
    scale: PhantomData<S>,
}

impl<S: TimeScale> Instant<S> {
    /// The instant labelled `1970-01-01T00:00:00` on this scale.
    pub const EPOCH: Self = Self {
        since_epoch: Duration::ZERO,
        scale: PhantomData,
    };

    /// Build an instant from its reading relative to the 1970 epoch.
    #[must_use]
    pub const fn from_epoch(since_epoch: Duration) -> Self {
        Self {
            since_epoch,
            scale: PhantomData,
        }
    }

    /// The reading relative to the 1970 epoch.
    #[must_use]
    pub const fn since_epoch(self) -> Duration {
        self.since_epoch
    }

    /// The runtime identifier of this instant's scale.
    #[must_use]
    pub const fn scale_id(self) -> TimeScaleId {
        S::ID
    }

    /// Re-read the same physical event on another scale.
    #[must_use]
    pub fn convert<T: TimeScale>(self) -> Instant<T> {
        Instant::from_epoch(T::from_tai(S::to_tai(self.since_epoch)))
    }

    /// The same event read on TAI.
    #[must_use]
    pub fn to_tai(self) -> Instant<Tai> {
        self.convert()
    }

    /// Advance the instant by a span.
    ///
    /// # Errors
    ///
    /// Returns [`crate::TimeError::Overflow`] when the result is
    /// unrepresentable.
    pub fn checked_add(self, span: Duration) -> TimeResult<Self> {
        self.since_epoch.checked_add(span).map(Self::from_epoch)
    }

    /// Move the instant backwards by a span.
    ///
    /// # Errors
    ///
    /// Returns [`crate::TimeError::Overflow`] when the result is
    /// unrepresentable.
    pub fn checked_sub(self, span: Duration) -> TimeResult<Self> {
        self.since_epoch.checked_sub(span).map(Self::from_epoch)
    }

    /// The span from `earlier` to `self`.
    ///
    /// # Errors
    ///
    /// Returns [`crate::TimeError::Overflow`] when the result is
    /// unrepresentable.
    pub fn duration_since(self, earlier: Self) -> TimeResult<Duration> {
        self.since_epoch.checked_sub(earlier.since_epoch)
    }
}

impl<S: TimeScale> Clone for Instant<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S: TimeScale> Copy for Instant<S> {}

impl<S: TimeScale> PartialEq for Instant<S> {
    fn eq(&self, other: &Self) -> bool {
        self.since_epoch == other.since_epoch
    }
}

impl<S: TimeScale> Eq for Instant<S> {}

impl<S: TimeScale> PartialOrd for Instant<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: TimeScale> Ord for Instant<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.since_epoch.cmp(&other.since_epoch)
    }
}

impl<S: TimeScale> fmt::Debug for Instant<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instant<{}>({})", S::ID, self.since_epoch)
    }
}

impl<S: TimeScale> Default for Instant<S> {
    fn default() -> Self {
        Self::EPOCH
    }
}

/// A scale-erased instant, for FFI and heterogeneous collections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnyInstant {
    /// Which scale the reading is on.
    pub scale: TimeScaleId,
    /// The reading relative to the 1970 epoch of that scale.
    pub since_epoch: Duration,
}

impl<S: TimeScale> From<Instant<S>> for AnyInstant {
    fn from(value: Instant<S>) -> Self {
        Self {
            scale: S::ID,
            since_epoch: value.since_epoch(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(feature = "std"))]
    use alloc::string::ToString as _;

    #[test]
    fn tt_is_tai_plus_the_defined_offset() {
        let tai = Instant::<Tai>::from_epoch(Duration::from_secs(1_000));
        let tt: Instant<Tt> = tai.convert();
        assert_eq!(tt.since_epoch(), Duration::from_secs(1_000) + TT_MINUS_TAI);
        let back: Instant<Tai> = tt.convert();
        assert_eq!(back, tai);
    }

    #[test]
    fn gps_is_tai_minus_nineteen_seconds() {
        let tai = Instant::<Tai>::from_epoch(Duration::from_secs(1_000));
        let gps: Instant<Gps> = tai.convert();
        assert_eq!(gps.since_epoch(), Duration::from_secs(981));
    }

    #[test]
    fn tcg_round_trips_within_a_nanosecond() {
        let tai = Instant::<Tai>::from_epoch(Duration::from_secs(1_700_000_000));
        let tcg: Instant<Tcg> = tai.convert();
        let back: Instant<Tai> = tcg.convert();
        let error = back
            .duration_since(tai)
            .unwrap()
            .checked_abs()
            .unwrap()
            .as_secs_f64();
        assert!(error < 1e-9, "round trip error {error}");
    }

    #[test]
    fn tcg_runs_ahead_of_tt_by_about_two_thirds_of_a_second_per_decade() {
        // L_G is ~6.97e-10, so a decade (3.156e8 s) accumulates ~0.22 s.
        let tai = Instant::<Tai>::from_epoch(Duration::from_secs(220_924_800 + 315_576_000));
        let tt: Instant<Tt> = tai.convert();
        let tcg: Instant<Tcg> = tai.convert();
        let delta = tcg.since_epoch().as_secs_f64() - tt.since_epoch().as_secs_f64();
        assert!((delta - 0.22).abs() < 0.01, "TCG - TT was {delta}");
    }

    #[test]
    fn tdb_stays_within_two_milliseconds_of_tt() {
        for year in 0..60 {
            let tai = Instant::<Tai>::from_epoch(Duration::from_secs(year * 31_557_600));
            let tt: Instant<Tt> = tai.convert();
            let tdb: Instant<Tdb> = tai.convert();
            let delta = tdb.since_epoch().as_secs_f64() - tt.since_epoch().as_secs_f64();
            assert!(delta.abs() < 0.002, "TDB - TT was {delta} in year {year}");
        }
    }

    #[test]
    fn tdb_round_trips_within_a_microsecond() {
        let tai = Instant::<Tai>::from_epoch(Duration::from_secs(1_700_000_000));
        let tdb: Instant<Tdb> = tai.convert();
        let back: Instant<Tai> = tdb.convert();
        let error = back
            .duration_since(tai)
            .unwrap()
            .checked_abs()
            .unwrap()
            .as_secs_f64();
        assert!(error < 1e-6, "round trip error {error}");
    }

    #[test]
    fn tcb_round_trips_within_a_microsecond() {
        let tai = Instant::<Tai>::from_epoch(Duration::from_secs(1_700_000_000));
        let tcb: Instant<Tcb> = tai.convert();
        let back: Instant<Tai> = tcb.convert();
        let error = back
            .duration_since(tai)
            .unwrap()
            .checked_abs()
            .unwrap()
            .as_secs_f64();
        assert!(error < 1e-6, "round trip error {error}");
    }

    #[test]
    fn scale_identifiers_render_conventionally() {
        assert_eq!(TimeScaleId::Tai.to_string(), "TAI");
        assert_eq!(TimeScaleId::Utc.abbreviation(), "UTC");
    }
}
