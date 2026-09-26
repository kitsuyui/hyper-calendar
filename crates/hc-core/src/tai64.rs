//! TAI64, TAI64N and TAI64NA labels.
//!
//! D. J. Bernstein, "TAI64, TAI64N, and TAI64NA", <https://cr.yp.to/libtai/tai64.html>,
//! read 2026-09-26 (`bernstein-tai64`). A TAI64 label is an integer below
//! 2⁶³ naming one TAI second: `2⁶² + s` is the second beginning `s` seconds
//! after the beginning of 1970 TAI, and `2⁶² − s` the one beginning `s`
//! seconds before it. Labels from 2⁶³ are reserved for future extensions.
//! The external format is the label in eight big-endian bytes; TAI64N adds
//! four big-endian bytes counting the nanoseconds of that second, and
//! TAI64NA four more counting the attoseconds of that nanosecond, each
//! from 0 to 999 999 999.
//!
//! The origin is 1970-01-01 00:00:00 *TAI*, the origin of
//! [`Instant<Tai>`], not the POSIX epoch: 1970 UTC began 8.000 082 s of TAI
//! later ([`crate::epoch::UNIX`]). TAI64NA resolves the attosecond, which is
//! [`Duration`]'s resolution, so it round-trips exactly; TAI64 and TAI64N
//! name the second and the nanosecond that contain an instant, which is
//! the floor of its reading.
//!
//! # A second convention: POSIX seconds plus ten
//!
//! Labels in the wild are not all true TAI. daemontools' `tai64n`, which
//! stamps the lines `multilog` writes, takes the system clock "as the
//! number of TAI seconds since 1970-01-01 00:00:10 TAI" (Bernstein, "The
//! tai64n program", <https://cr.yp.to/daemontools/tai64n.html>, read
//! 2026-09-27, `bernstein-daemontools-tai64n`). That holds on a clock kept
//! in the Olson library's `right/` mode. On an ordinary clock, which keeps
//! POSIX time, the label it writes is `2⁶² + 10 + POSIX seconds`, and its
//! own `tai64nlocal` page warns that localtime then uses "a broken time
//! scale that does not account for leap seconds"
//! (`bernstein-daemontools-tai64nlocal`). Such a label read as true TAI
//! falls `TAI − UTC − 10` seconds early: 27 s since 2017.
//!
//! The two readings are separate named conventions, as `docs/policy.md` §5
//! asks, not a parameter: `tai64` is the functions above, true TAI through
//! [`Instant<Tai>`], and `tai64-posix-plus-10` is
//! [`label_posix_plus_10`], [`from_label_posix_plus_10`] and their
//! external-format pairs, which read and write [`UnixTime`] with no
//! leap-second table at all. Neither can tell from the bytes which one
//! wrote them; the caller knows what clock the log came from. See
//! `docs/time-scales.md`.

use crate::duration::Duration;
use crate::error::{TimeError, TimeResult};
use crate::scale::{Instant, Tai};
use crate::unix::UnixTime;

/// `2⁶²`, the label of the second that began 1970 TAI.
pub const TAI64_ORIGIN_LABEL: u64 = 1 << 62;

/// The first label reserved for future extensions, `2⁶³`.
pub const TAI64_RESERVED: u64 = 1 << 63;

const PER_BILLION: u64 = 1_000_000_000;

/// The TAI64 label of the second containing a TAI instant.
///
/// # Errors
///
/// [`TimeError::OutOfRange`] for a second that no label can hold: the
/// labels run below 2⁶³, about 146 billion years either side of 1970.
pub fn label(instant: Instant<Tai>) -> TimeResult<u64> {
    let seconds = instant.since_epoch().whole_seconds();
    let label = seconds
        .checked_add(i128::from(TAI64_ORIGIN_LABEL))
        .ok_or(TimeError::OutOfRange)?;
    match u64::try_from(label) {
        Ok(label) if label < TAI64_RESERVED => Ok(label),
        _ => Err(TimeError::OutOfRange),
    }
}

/// The TAI instant at the start of the second a TAI64 label names.
///
/// # Errors
///
/// [`TimeError::OutOfRange`] for a label from 2⁶³, which is reserved.
pub fn from_label(label: u64) -> TimeResult<Instant<Tai>> {
    if label >= TAI64_RESERVED {
        return Err(TimeError::OutOfRange);
    }
    let seconds = i128::from(label) - i128::from(TAI64_ORIGIN_LABEL);
    Ok(Instant::from_epoch(Duration::from_secs(seconds)))
}

/// The nanoseconds and the attoseconds within them of an instant's
/// sub-second part.
fn split_subsec(instant: Instant<Tai>) -> (u32, u32) {
    let attos = instant.since_epoch().subsec_attos();
    // Both parts are below 10⁹ because `attos` is below 10¹⁸.
    let nanos = u32::try_from(attos / PER_BILLION).unwrap_or(u32::MAX);
    let rest = u32::try_from(attos % PER_BILLION).unwrap_or(u32::MAX);
    (nanos, rest)
}

fn counter(bytes: [u8; 4]) -> TimeResult<u64> {
    let value = u64::from(u32::from_be_bytes(bytes));
    if value >= PER_BILLION {
        return Err(TimeError::OutOfRange);
    }
    Ok(value)
}

fn join(label: u64, nanos: u64, attos: u64) -> TimeResult<Instant<Tai>> {
    let start = from_label(label)?;
    // Below 10¹⁸ by the counters' ranges.
    let subsec = Duration::from_attos(i128::from(nanos * PER_BILLION + attos));
    start.checked_add(subsec)
}

/// The external TAI64 format: the label in eight big-endian bytes.
///
/// # Errors
///
/// As [`label`].
pub fn encode_tai64(instant: Instant<Tai>) -> TimeResult<[u8; 8]> {
    Ok(label(instant)?.to_be_bytes())
}

/// Read the external TAI64 format.
///
/// # Errors
///
/// As [`from_label`].
pub fn decode_tai64(bytes: [u8; 8]) -> TimeResult<Instant<Tai>> {
    from_label(u64::from_be_bytes(bytes))
}

/// The external TAI64N format: the TAI64 label and four big-endian bytes
/// of nanoseconds.
///
/// # Errors
///
/// As [`label`].
pub fn encode_tai64n(instant: Instant<Tai>) -> TimeResult<[u8; 12]> {
    let mut out = [0; 12];
    out[..8].copy_from_slice(&encode_tai64(instant)?);
    out[8..].copy_from_slice(&split_subsec(instant).0.to_be_bytes());
    Ok(out)
}

/// Read the external TAI64N format.
///
/// # Errors
///
/// [`TimeError::OutOfRange`] for a reserved label or a nanosecond count
/// above 999 999 999.
pub fn decode_tai64n(bytes: [u8; 12]) -> TimeResult<Instant<Tai>> {
    let [l0, l1, l2, l3, l4, l5, l6, l7, n0, n1, n2, n3] = bytes;
    let label = u64::from_be_bytes([l0, l1, l2, l3, l4, l5, l6, l7]);
    join(label, counter([n0, n1, n2, n3])?, 0)
}

/// The external TAI64NA format: the TAI64N label and four big-endian bytes
/// of attoseconds. Exact for every representable instant.
///
/// # Errors
///
/// As [`label`].
pub fn encode_tai64na(instant: Instant<Tai>) -> TimeResult<[u8; 16]> {
    let mut out = [0; 16];
    out[..12].copy_from_slice(&encode_tai64n(instant)?);
    out[12..].copy_from_slice(&split_subsec(instant).1.to_be_bytes());
    Ok(out)
}

/// Read the external TAI64NA format.
///
/// # Errors
///
/// [`TimeError::OutOfRange`] for a reserved label or a counter above
/// 999 999 999.
pub fn decode_tai64na(bytes: [u8; 16]) -> TimeResult<Instant<Tai>> {
    let [
        l0,
        l1,
        l2,
        l3,
        l4,
        l5,
        l6,
        l7,
        n0,
        n1,
        n2,
        n3,
        a0,
        a1,
        a2,
        a3,
    ] = bytes;
    let label = u64::from_be_bytes([l0, l1, l2, l3, l4, l5, l6, l7]);
    join(
        label,
        counter([n0, n1, n2, n3])?,
        counter([a0, a1, a2, a3])?,
    )
}

/// The label of POSIX second 0 in the `tai64-posix-plus-10` convention,
/// `2⁶² + 10`.
pub const POSIX_PLUS_10_ORIGIN_LABEL: u64 = TAI64_ORIGIN_LABEL + 10;

/// The `tai64-posix-plus-10` label of the POSIX second containing a POSIX
/// timestamp: `2⁶² + 10 + seconds`, as daemontools' `tai64n` writes it on
/// a clock that keeps POSIX time. See the [module documentation](self).
///
/// # Errors
///
/// [`TimeError::OutOfRange`] for a second before −(2⁶² + 10) or from
/// 2⁶² − 10, whose label would fall outside 0 to 2⁶³ − 1.
pub fn label_posix_plus_10(unix: UnixTime) -> TimeResult<u64> {
    let label = i128::from(unix.seconds()) + i128::from(POSIX_PLUS_10_ORIGIN_LABEL);
    match u64::try_from(label) {
        Ok(label) if label < TAI64_RESERVED => Ok(label),
        _ => Err(TimeError::OutOfRange),
    }
}

/// The POSIX second a `tai64-posix-plus-10` label names: the label less
/// 2⁶² + 10.
///
/// # Errors
///
/// [`TimeError::OutOfRange`] for a label from 2⁶³, which is reserved.
pub fn from_label_posix_plus_10(label: u64) -> TimeResult<UnixTime> {
    if label >= TAI64_RESERVED {
        return Err(TimeError::OutOfRange);
    }
    let seconds = i128::from(label) - i128::from(POSIX_PLUS_10_ORIGIN_LABEL);
    let seconds = i64::try_from(seconds).map_err(|_| TimeError::OutOfRange)?;
    Ok(UnixTime::from_seconds(seconds))
}

/// The external TAI64 format of a POSIX timestamp in the
/// `tai64-posix-plus-10` convention.
///
/// # Errors
///
/// As [`label_posix_plus_10`].
pub fn encode_tai64_posix_plus_10(unix: UnixTime) -> TimeResult<[u8; 8]> {
    Ok(label_posix_plus_10(unix)?.to_be_bytes())
}

/// Read the external TAI64 format in the `tai64-posix-plus-10` convention.
///
/// # Errors
///
/// As [`from_label_posix_plus_10`].
pub fn decode_tai64_posix_plus_10(bytes: [u8; 8]) -> TimeResult<UnixTime> {
    from_label_posix_plus_10(u64::from_be_bytes(bytes))
}

/// The external TAI64N format of a POSIX timestamp in the
/// `tai64-posix-plus-10` convention: what `tai64n` prints, as bytes.
///
/// # Errors
///
/// As [`label_posix_plus_10`].
pub fn encode_tai64n_posix_plus_10(unix: UnixTime) -> TimeResult<[u8; 12]> {
    let mut out = [0; 12];
    out[..8].copy_from_slice(&encode_tai64_posix_plus_10(unix)?);
    // Below 10⁹ because the sub-second part is below 10¹⁸.
    let nanos = u32::try_from(unix.subsec_attos() / PER_BILLION).unwrap_or(u32::MAX);
    out[8..].copy_from_slice(&nanos.to_be_bytes());
    Ok(out)
}

/// Read the external TAI64N format in the `tai64-posix-plus-10`
/// convention.
///
/// # Errors
///
/// [`TimeError::OutOfRange`] for a reserved label or a nanosecond count
/// above 999 999 999.
pub fn decode_tai64n_posix_plus_10(bytes: [u8; 12]) -> TimeResult<UnixTime> {
    let [l0, l1, l2, l3, l4, l5, l6, l7, n0, n1, n2, n3] = bytes;
    let label = u64::from_be_bytes([l0, l1, l2, l3, l4, l5, l6, l7]);
    let nanos = counter([n0, n1, n2, n3])?;
    let seconds = from_label_posix_plus_10(label)?.seconds();
    UnixTime::new(seconds, nanos * PER_BILLION)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unix::{LeapPolicy, tai_from_unix, utc_from_tai};

    fn tai(seconds: i128) -> Instant<Tai> {
        Instant::from_epoch(Duration::from_secs(seconds))
    }

    /// Bernstein's examples: 3fffffffffffffff is the second that ended
    /// 1969 TAI, 4000000000000000 the one that began 1970, and
    /// 400000002a2b2c2d is 1992-06-02 08:07:09 TAI.
    #[test]
    fn bernsteins_examples() {
        assert_eq!(
            decode_tai64([0x3f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]),
            Ok(tai(-1))
        );
        assert_eq!(decode_tai64([0x40, 0, 0, 0, 0, 0, 0, 0]), Ok(tai(0)));
        assert_eq!(decode_tai64([0x40, 0, 0, 0, 0, 0, 0, 1]), Ok(tai(1)));

        // 1992-06-02 is day 8 188 from 1970-01-01, and 08:07:09 is 29 229 s.
        let example = tai(8_188 * 86_400 + 29_229);
        let bytes = [0x40, 0, 0, 0, 0x2a, 0x2b, 0x2c, 0x2d];
        assert_eq!(encode_tai64(example), Ok(bytes));
        assert_eq!(decode_tai64(bytes), Ok(example));
    }

    /// The same example through the leap-second table: Bernstein gives it
    /// as 1992-06-02 08:06:43 UTC, when TAI − UTC was 26 s.
    #[test]
    fn bernsteins_example_agrees_with_the_leap_table() {
        // 1992-06-02 08:06:43 UTC is POSIX 707 472 403.
        let from_utc =
            tai_from_unix(UnixTime::from_seconds(707_472_403), LeapPolicy::Strict).expect("table");
        assert_eq!(
            encode_tai64(from_utc),
            Ok([0x40, 0, 0, 0, 0x2a, 0x2b, 0x2c, 0x2d])
        );
    }

    /// The `tai64n` page's example, `4000000037c219bf2ef02e94`, which its
    /// `tai64nlocal` page prints as 1999-08-23 21:03:43.787492500 in
    /// US/Pacific (UTC − 7 h that day) on a clock that keeps TAI, the
    /// `right/` zones. Read as true TAI it is exactly that instant.
    #[test]
    fn the_daemontools_example_is_true_tai() {
        let bytes = [
            0x40, 0, 0, 0, 0x37, 0xc2, 0x19, 0xbf, 0x2e, 0xf0, 0x2e, 0x94,
        ];
        let instant = decode_tai64n(bytes).expect("in range");
        assert_eq!(
            instant,
            Instant::from_epoch(
                Duration::from_secs(935_467_455) + Duration::from_nanos(787_492_500)
            )
        );
        // 1999-08-24 04:03:43 UTC is POSIX 935 467 423; TAI − UTC was 32 s.
        let utc = utc_from_tai(instant, LeapPolicy::Strict).expect("table");
        assert_eq!(utc.unix_seconds, 935_467_423);
        assert!(!utc.leap_second);
        assert_eq!(utc.subsec_attos, 787_492_500_000_000_000);
    }

    /// `tai64-posix-plus-10`: POSIX 0 is `@400000000000000a`, derived from
    /// the `tai64n` page's "TAI seconds since 1970-01-01 00:00:10 TAI" with
    /// a POSIX clock. The same daemontools bytes read this way are 22 s
    /// later than read as TAI, `TAI − UTC − 10` in 1999, and 27 s since
    /// 2017.
    #[test]
    fn the_posix_plus_10_convention() {
        assert_eq!(
            encode_tai64_posix_plus_10(UnixTime::EPOCH),
            Ok([0x40, 0, 0, 0, 0, 0, 0, 0x0a])
        );
        assert_eq!(
            decode_tai64_posix_plus_10([0x40, 0, 0, 0, 0, 0, 0, 0x0a]),
            Ok(UnixTime::EPOCH)
        );
        assert_eq!(
            label_posix_plus_10(UnixTime::EPOCH),
            Ok(POSIX_PLUS_10_ORIGIN_LABEL)
        );

        let bytes = [
            0x40, 0, 0, 0, 0x37, 0xc2, 0x19, 0xbf, 0x2e, 0xf0, 0x2e, 0x94,
        ];
        let posix = decode_tai64n_posix_plus_10(bytes).expect("in range");
        assert_eq!(
            posix,
            UnixTime::new(935_467_445, 787_492_500_000_000_000).expect("ok")
        );
        assert_eq!(encode_tai64n_posix_plus_10(posix), Ok(bytes));
        let as_tai =
            utc_from_tai(decode_tai64n(bytes).expect("ok"), LeapPolicy::Strict).expect("table");
        assert_eq!(posix.seconds() - as_tai.unix_seconds, 22);

        // 2024-01-01 00:00:00 UTC, POSIX 1 704 067 200: the two labels of
        // the same second differ by TAI − UTC − 10 = 27.
        let unix = UnixTime::from_seconds(1_704_067_200);
        let tai = tai_from_unix(unix, LeapPolicy::Strict).expect("table");
        assert_eq!(
            label(tai).expect("ok") - label_posix_plus_10(unix).expect("ok"),
            27
        );

        // Sub-nanosecond parts floor, and the reserved labels are refused.
        let fine = UnixTime::new(0, 1_999_999_999).expect("ok");
        assert_eq!(
            decode_tai64n_posix_plus_10(encode_tai64n_posix_plus_10(fine).expect("ok")),
            UnixTime::new(0, 1_000_000_000)
        );
        assert_eq!(
            from_label_posix_plus_10(TAI64_RESERVED),
            Err(TimeError::OutOfRange)
        );
        assert_eq!(
            decode_tai64n_posix_plus_10([0x40, 0, 0, 0, 0, 0, 0, 0x0a, 0x3b, 0x9a, 0xca, 0x00]),
            Err(TimeError::OutOfRange)
        );
        // The label range reaches every i64 second: i64::MIN is label
        // 2⁶² + 10 − 2⁶³, below 0.
        assert_eq!(
            label_posix_plus_10(UnixTime::from_seconds(i64::MIN)),
            Err(TimeError::OutOfRange)
        );
        assert_eq!(
            label_posix_plus_10(UnixTime::from_seconds(-(1 << 62) - 10)),
            Ok(0)
        );
        assert_eq!(
            from_label_posix_plus_10(0),
            Ok(UnixTime::from_seconds(-(1 << 62) - 10))
        );
        assert_eq!(
            label_posix_plus_10(UnixTime::from_seconds((1 << 62) - 11)),
            Ok(TAI64_RESERVED - 1)
        );
        assert_eq!(
            label_posix_plus_10(UnixTime::from_seconds((1 << 62) - 10)),
            Err(TimeError::OutOfRange)
        );
    }

    #[test]
    fn tai64na_round_trips_exactly_and_the_shorter_forms_floor() {
        for attos in [
            0i128,
            1,
            999_999_999,
            1_000_000_000,
            123_456_789_987_654_321,
            -1,
            -1_500_000_000_000_000_000,
            707_472_429_000_000_000_000_000_001,
        ] {
            let instant = Instant::<Tai>::from_epoch(Duration::from_attos(attos));
            let na = encode_tai64na(instant).expect("in range");
            assert_eq!(decode_tai64na(na), Ok(instant), "{attos}");

            let floor_nanos = Duration::from_attos(attos.div_euclid(1_000_000_000) * 1_000_000_000);
            let n = encode_tai64n(instant).expect("in range");
            assert_eq!(n[..], na[..12]);
            assert_eq!(
                decode_tai64n(n),
                Ok(Instant::from_epoch(floor_nanos)),
                "{attos}"
            );

            let floor_secs = Duration::from_secs(attos.div_euclid(1_000_000_000_000_000_000));
            let s = encode_tai64(instant).expect("in range");
            assert_eq!(
                decode_tai64(s),
                Ok(Instant::from_epoch(floor_secs)),
                "{attos}"
            );
        }
        // 1.5 s before 1970 TAI is in the second labelled 2⁶² − 2, half-way.
        let bytes = encode_tai64n(Instant::from_epoch(Duration::from_millis(-1_500))).expect("ok");
        assert_eq!(
            bytes,
            [
                0x3f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0x1d, 0xcd, 0x65, 0
            ]
        );
    }

    #[test]
    fn reserved_labels_and_overlong_counters_are_refused() {
        assert_eq!(from_label(TAI64_RESERVED), Err(TimeError::OutOfRange));
        assert_eq!(
            decode_tai64([0x80, 0, 0, 0, 0, 0, 0, 0]),
            Err(TimeError::OutOfRange)
        );
        assert_eq!(
            decode_tai64n([0x40, 0, 0, 0, 0, 0, 0, 0, 0x3b, 0x9a, 0xca, 0x00]),
            Err(TimeError::OutOfRange),
            "10⁹ nanoseconds"
        );
        assert_eq!(
            decode_tai64na([
                0x40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x3b, 0x9a, 0xca, 0x00
            ]),
            Err(TimeError::OutOfRange),
            "10⁹ attoseconds"
        );
        assert_eq!(
            decode_tai64n([0x40, 0, 0, 0, 0, 0, 0, 0, 0x3b, 0x9a, 0xc9, 0xff]),
            Ok(Instant::from_epoch(Duration::from_nanos(999_999_999)))
        );
        // The ends of the label range.
        assert_eq!(from_label(0), Ok(tai(-(1 << 62))));
        assert_eq!(label(tai(-(1 << 62))), Ok(0));
        assert_eq!(label(tai((1 << 62) - 1)), Ok(TAI64_RESERVED - 1));
        assert_eq!(label(tai(1 << 62)), Err(TimeError::OutOfRange));
        assert_eq!(label(tai(-(1 << 62) - 1)), Err(TimeError::OutOfRange));
    }
}
