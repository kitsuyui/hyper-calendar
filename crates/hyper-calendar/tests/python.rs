//! The `civil` layer against the examples of Python's `datetime`
//! documentation (<https://docs.python.org/3/library/datetime.html>,
//! retrieved 2026-09-26). Each test quotes the example it reproduces; the
//! expected values are the documentation's, not the output of running
//! Python. `docs/python-parity.md` is the full correspondence.

use hyper_calendar::civil::{Date, DateTime, Replace, Time, TimeDelta, TimeDeltaParts};

fn date(year: i64, month: u8, day: u8) -> Date {
    match Date::new(year, month, day) {
        Ok(date) => date,
        Err(error) => panic!("{year}-{month}-{day}: {error}"),
    }
}

// --- timedelta -------------------------------------------------------------

/// ```text
/// >>> delta = dt.timedelta(days=50, seconds=27, microseconds=10,
/// ...     milliseconds=29000, minutes=5, hours=8, weeks=2)
/// >>> delta
/// datetime.timedelta(days=64, seconds=29156, microseconds=10)
/// ```
#[test]
fn timedelta_normalises_its_keyword_arguments() {
    let delta = TimeDelta::from_parts(TimeDeltaParts {
        days: 50,
        seconds: 27,
        microseconds: 10,
        milliseconds: 29_000,
        minutes: 5,
        hours: 8,
        weeks: 2,
    });
    assert_eq!(
        (delta.days(), delta.seconds(), delta.microseconds()),
        (64, 29_156, 10)
    );
}

/// ```text
/// >>> d = dt.timedelta(microseconds=-1)
/// >>> (d.days, d.seconds, d.microseconds)
/// (-1, 86399, 999999)
/// ```
#[test]
fn a_negative_timedelta_normalises_into_the_day_before() {
    let d = TimeDelta::from_micros(-1);
    assert_eq!(
        (d.days(), d.seconds(), d.microseconds()),
        (-1, 86_399, 999_999)
    );
}

/// ```text
/// >>> year = dt.timedelta(days=365)
/// >>> another_year = dt.timedelta(weeks=40, days=84, hours=23,
/// ...                             minutes=50, seconds=600)
/// >>> year == another_year
/// True
/// >>> year.total_seconds()
/// 31536000.0
/// ```
#[test]
fn equal_spans_built_differently_are_equal() {
    let year = TimeDelta::from_days(365);
    let another_year = TimeDelta::from_parts(TimeDeltaParts {
        weeks: 40,
        days: 84,
        hours: 23,
        minutes: 50,
        seconds: 600,
        ..TimeDeltaParts::default()
    });
    assert_eq!(year, another_year);
    assert_eq!(year.total_seconds(), 31_536_000.0);
}

/// ```text
/// >>> ten_years = 10 * year
/// >>> ten_years
/// datetime.timedelta(days=3650)
/// >>> ten_years.days // 365
/// 10
/// >>> nine_years = ten_years - year
/// >>> nine_years
/// datetime.timedelta(days=3285)
/// >>> three_years = nine_years // 3
/// >>> three_years, three_years.days // 365
/// (datetime.timedelta(days=1095), 3)
/// ```
#[test]
fn timedelta_arithmetic_follows_the_documented_example() {
    let year = TimeDelta::from_days(365);
    let ten_years = 10 * year;
    assert_eq!(ten_years, TimeDelta::from_days(3_650));
    assert_eq!(ten_years.days() / 365, 10);
    let nine_years = ten_years - year;
    assert_eq!(nine_years, TimeDelta::from_days(3_285));
    let three_years = nine_years / 3;
    assert_eq!(
        (three_years, three_years.days() / 365),
        (TimeDelta::from_days(1_095), 3)
    );
    assert_eq!(three_years.checked_div_floor(year), Ok(3));
    assert_eq!(
        nine_years % TimeDelta::from_days(1_000),
        TimeDelta::from_days(285)
    );
    assert_eq!(
        TimeDelta::from_days(1).checked_div_rem(TimeDelta::from_hours(1)),
        Ok((24, TimeDelta::ZERO))
    );
    assert_eq!(year.ratio(TimeDelta::from_days(73)), Ok(5.0));
    assert_eq!(-year, TimeDelta::from_days(-365));
    assert_eq!((-year).abs(), year);
    let mut running = TimeDelta::ZERO;
    running += year;
    running -= TimeDelta::from_days(1);
    assert_eq!(running, TimeDelta::from_days(364));
}

/// ```text
/// >>> timedelta(hours=-5)
/// datetime.timedelta(days=-1, seconds=68400)
/// >>> print(_)
/// -1 day, 19:00:00
/// ```
#[test]
fn a_timedelta_prints_as_python_prints_it() {
    let delta = TimeDelta::from_hours(-5);
    assert_eq!((delta.days(), delta.seconds()), (-1, 68_400));
    assert_eq!(delta.to_string(), "-1 day, 19:00:00");
    assert_eq!(
        TimeDelta::from_days(3_650).to_string(),
        "3650 days, 0:00:00"
    );
    assert_eq!(TimeDelta::from_micros(10).to_string(), "0:00:00.000010");
}

/// ```text
/// >>> duration = dt.timedelta(seconds=11235813)
/// >>> duration.days, duration.seconds
/// (130, 3813)
/// >>> duration.total_seconds()
/// 11235813.0
/// ```
#[test]
fn seconds_is_the_remainder_and_total_seconds_is_the_whole() {
    let duration = TimeDelta::from_seconds(11_235_813);
    assert_eq!((duration.days(), duration.seconds()), (130, 3_813));
    assert_eq!(duration.total_seconds(), 11_235_813.0);
}

// --- date ------------------------------------------------------------------

/// ```text
/// >>> dt.date(2002, 12, 4).weekday()
/// 2
/// >>> dt.date(2002, 12, 4).isoweekday()
/// 3
/// ```
#[test]
fn weekday_numbers_match_python() {
    let wednesday = date(2002, 12, 4);
    assert_eq!(wednesday.weekday().monday_first_number(), 2);
    assert_eq!(wednesday.iso_weekday(), 3);
}

/// ```text
/// >>> dt.date(2003, 12, 29).isocalendar()
/// datetime.IsoCalendarDate(year=2004, week=1, weekday=1)
/// >>> dt.date(2004, 1, 4).isocalendar()
/// datetime.IsoCalendarDate(year=2004, week=1, weekday=7)
/// ```
#[test]
fn isocalendar_matches_the_documented_examples() {
    let first = date(2003, 12, 29).iso_calendar();
    assert_eq!((first.year, first.week, first.weekday), (2004, 1, 1));
    let seventh = date(2004, 1, 4).iso_calendar();
    assert_eq!((seventh.year, seventh.week, seventh.weekday), (2004, 1, 7));
    assert_eq!(
        Date::from_iso_calendar(2004, 1, 1).unwrap(),
        date(2003, 12, 29)
    );
    assert!(Date::from_iso_calendar(2004, 54, 1).is_err());
    assert_eq!(
        DateTime::from_iso_calendar(2004, 1, 1).unwrap(),
        DateTime::midnight(date(2003, 12, 29))
    );
}

/// ```text
/// >>> d = dt.date(2002, 12, 31)
/// >>> d.replace(day=26)
/// datetime.date(2002, 12, 26)
/// ```
#[test]
fn date_replace_matches_the_documented_example() {
    assert_eq!(
        date(2002, 12, 31).replace(None, None, Some(26)).unwrap(),
        date(2002, 12, 26)
    );
}

/// "`date2 = date1 + timedelta`: date2 is moved forward in time if
/// `timedelta.days > 0`, or backward if `timedelta.days < 0` …
/// `timedelta.seconds` and `timedelta.microseconds` are ignored."
#[test]
fn date_arithmetic_uses_only_the_whole_days_of_a_span() {
    let new_year_eve = date(2002, 12, 31);
    assert_eq!(new_year_eve + TimeDelta::from_days(1), date(2003, 1, 1));
    assert_eq!(new_year_eve - TimeDelta::from_days(365), date(2001, 12, 31));
    // A second forward is zero whole days; a second back is minus one.
    assert_eq!(new_year_eve + TimeDelta::from_seconds(1), new_year_eve);
    assert_eq!(
        new_year_eve + TimeDelta::from_seconds(-1),
        date(2002, 12, 30)
    );
    assert_eq!(date(2003, 1, 1) - new_year_eve, TimeDelta::from_days(1));
    assert!(
        Date::MAX
            .checked_add_delta(TimeDelta::from_days(1))
            .is_err()
    );
    assert!(
        Date::MIN
            .checked_add_delta(TimeDelta::from_days(-1))
            .is_err()
    );
}

#[test]
fn the_ranges_are_stated_as_constants() {
    assert!(Date::MIN < Date::new(1, 1, 1).unwrap());
    assert!(Date::MAX > Date::new(9_999, 12, 31).unwrap());
    assert_eq!(Date::RESOLUTION, TimeDelta::from_days(1));
    assert!(Time::MAX.is_leap_second());
    assert_eq!(DateTime::MIN, DateTime::midnight(Date::MIN));
    assert!(TimeDelta::MIN < TimeDelta::ZERO && TimeDelta::ZERO < TimeDelta::RESOLUTION);
    assert!(TimeDelta::MIN.checked_abs().is_err());
}

// --- time and datetime -------------------------------------------------------

/// ```text
/// >>> d = dt.date(2005, 7, 14)
/// >>> t = dt.time(12, 30)
/// >>> dt.datetime.combine(d, t)
/// datetime.datetime(2005, 7, 14, 12, 30)
/// ```
#[test]
fn combine_matches_the_documented_example() {
    let combined = DateTime::combine(date(2005, 7, 14), Time::hms(12, 30, 0).unwrap());
    assert_eq!(
        combined,
        DateTime::from_parts(2005, 7, 14, 12, 30, 0, 0).unwrap()
    );
}

#[test]
fn replace_takes_python_s_keywords() {
    let reading = DateTime::from_parts(2002, 12, 31, 23, 59, 59, 0).unwrap();
    let replaced = reading
        .replace(Replace {
            day: Some(26),
            hour: Some(1),
            microsecond: Some(500),
            ..Replace::default()
        })
        .unwrap();
    assert_eq!(replaced.date, date(2002, 12, 26));
    assert_eq!(replaced.time, Time::from_hms_micro(1, 59, 59, 500).unwrap());
    assert!(
        reading
            .replace(Replace {
                month: Some(2),
                ..Replace::default()
            })
            .is_err()
    );
    assert_eq!(
        Time::NOON.replace(None, Some(30), None, None).unwrap(),
        Time::hms(12, 30, 0).unwrap()
    );
}

/// `datetime2 = datetime1 + timedelta` moves across midnight and across
/// years as nominal 86 400-second days.
#[test]
fn datetime_arithmetic_crosses_midnight() {
    let reading = DateTime::from_parts(2002, 12, 31, 23, 30, 0, 0).unwrap();
    let later = reading + TimeDelta::from_hours(1);
    assert_eq!(
        later,
        DateTime::from_parts(2003, 1, 1, 0, 30, 0, 0).unwrap()
    );
    assert_eq!(later - reading, TimeDelta::from_hours(1));
    assert_eq!(later - TimeDelta::from_hours(1), reading);
    assert_eq!(
        reading + TimeDelta::from_micros(-1),
        DateTime::new(
            date(2002, 12, 31),
            Time::from_hms_micro(23, 29, 59, 999_999).unwrap()
        )
    );
    assert_eq!(
        DateTime::from_ordinal(731_215).unwrap(),
        DateTime::midnight(date(2002, 12, 31))
    );
}

#[cfg(feature = "format")]
mod with_format {
    use super::*;
    use hyper_calendar::civil::TimeSpec;
    use hyper_calendar::hc_format::ZoneInfo;

    /// ```text
    /// >>> dt = datetime.strptime("21/11/06 16:30", "%d/%m/%y %H:%M")
    /// >>> dt
    /// datetime.datetime(2006, 11, 21, 16, 30)
    /// >>> ic = dt.isocalendar()
    /// >>> for it in ic:
    /// ...     print(it)
    /// 2006    # ISO year
    /// 47      # ISO week
    /// 2       # ISO weekday
    /// >>> dt.strftime("%A, %d. %B %Y %I:%M%p")
    /// 'Tuesday, 21. November 2006 04:30PM'
    /// ```
    #[test]
    fn the_datetime_usage_example_round_trips() {
        let (reading, zone) = DateTime::strptime("21/11/06 16:30", "%d/%m/%y %H:%M").unwrap();
        assert_eq!(
            reading,
            DateTime::from_parts(2006, 11, 21, 16, 30, 0, 0).unwrap()
        );
        assert_eq!(zone, ZoneInfo::Unspecified);
        let week = reading.date.iso_calendar();
        assert_eq!((week.year, week.week, week.weekday), (2006, 47, 2));
        assert_eq!(
            reading.strftime("%A, %d. %B %Y %I:%M%p").unwrap(),
            "Tuesday, 21. November 2006 04:30PM"
        );
    }

    /// ```text
    /// >>> dt.date.fromisoformat('2021-W01-1')
    /// datetime.date(2021, 1, 4)
    /// >>> dt.datetime.fromisoformat('2011-11-04T00:05:23+04:00')
    /// datetime.datetime(2011, 11, 4, 0, 5, 23,
    ///     tzinfo=datetime.timezone(datetime.timedelta(seconds=14400)))
    /// >>> dt.time.fromisoformat('04:23:01.000384')
    /// datetime.time(4, 23, 1, 384)
    /// ```
    #[test]
    fn fromisoformat_reaches_the_civil_types() {
        assert_eq!(
            Date::from_iso_format("2021-W01-1").unwrap(),
            date(2021, 1, 4)
        );
        let (reading, zone) = DateTime::from_iso_format("2011-11-04T00:05:23+04:00").unwrap();
        assert_eq!(
            reading,
            DateTime::from_parts(2011, 11, 4, 0, 5, 23, 0).unwrap()
        );
        assert_eq!(zone.offset().unwrap().seconds(), 14_400);
        let (time, _) = Time::from_iso_format("04:23:01.000384").unwrap();
        assert_eq!(time, Time::from_hms_micro(4, 23, 1, 384).unwrap());
    }

    /// ```text
    /// >>> dt.datetime(2019, 5, 18, 15, 17, 8, 132263).isoformat()
    /// '2019-05-18T15:17:08.132263'
    /// >>> dt.time(hour=12, minute=34, second=56, microsecond=123456).isoformat(timespec='minutes')
    /// '12:34'
    /// >>> dt.date(2002, 12, 4).ctime()
    /// 'Wed Dec  4 00:00:00 2002'
    /// ```
    #[test]
    fn isoformat_and_ctime_match_the_documented_examples() {
        let reading = DateTime::new(
            date(2019, 5, 18),
            Time::from_hms_micro(15, 17, 8, 132_263).unwrap(),
        );
        assert_eq!(
            reading.iso_format('T', TimeSpec::Auto).unwrap(),
            "2019-05-18T15:17:08.132263"
        );
        assert_eq!(
            Time::from_hms_micro(12, 34, 56, 123_456)
                .unwrap()
                .iso_format(TimeSpec::Minutes)
                .unwrap(),
            "12:34"
        );
        assert_eq!(
            date(2002, 12, 4).ctime().unwrap(),
            "Wed Dec  4 00:00:00 2002"
        );
        assert_eq!(
            Time::hms(21, 30, 0).unwrap().strftime("%H:%M %Y").unwrap(),
            "21:30 1900"
        );
    }
}

#[cfg(feature = "tz")]
mod with_tz {
    use super::*;
    use hyper_calendar::UnixTime;
    use hyper_calendar::hc_tz::{Disambiguation, FixedTimeZone, UtcOffset, builtin};

    /// `datetime.fromtimestamp(0, timezone.utc)` is 1970-01-01 00:00, and a
    /// reading's `timestamp()` in a zone inverts it.
    #[test]
    fn timestamps_go_through_a_named_zone() {
        assert_eq!(
            DateTime::from_timestamp_utc(UnixTime::EPOCH).unwrap(),
            DateTime::midnight(Date::UNIX_EPOCH)
        );
        let tokyo = FixedTimeZone::new("Asia/Tokyo", UtcOffset::from_hms(9, 0, 0).unwrap());
        let reading = DateTime::from_parts(2026, 9, 21, 12, 0, 0, 0).unwrap();
        let unix = reading.timestamp(&tokyo, Disambiguation::Reject).unwrap();
        assert_eq!(
            unix.seconds(),
            reading.timestamp_utc().unwrap().seconds() - 9 * 3_600
        );
        assert_eq!(DateTime::from_timestamp(unix, &tokyo).unwrap(), reading);
        assert_eq!(Date::from_timestamp(unix, &tokyo).unwrap(), reading.date);
    }

    /// `astimezone` converts through the instant, and a reading New York
    /// shows twice is refused unless the caller says which one — Python's
    /// `fold`.
    #[test]
    fn astimezone_converts_through_the_instant() {
        let tokyo = FixedTimeZone::new("Asia/Tokyo", UtcOffset::from_hms(9, 0, 0).unwrap());
        let reading = DateTime::from_parts(2026, 9, 21, 12, 0, 0, 0).unwrap();
        let utc = reading
            .astimezone(&tokyo, &hyper_calendar::hc_tz::Utc, Disambiguation::Reject)
            .unwrap();
        assert_eq!(utc, DateTime::from_parts(2026, 9, 21, 3, 0, 0, 0).unwrap());
        assert_eq!(
            reading
                .utc_offset(&tokyo, Disambiguation::Reject)
                .unwrap()
                .seconds(),
            9 * 3_600
        );
        let new_york = builtin::zone("America/New_York").unwrap();
        let twice = DateTime::from_parts(2024, 11, 3, 1, 30, 0, 0).unwrap();
        assert!(twice.timestamp(&new_york, Disambiguation::Reject).is_err());
        let first = twice
            .timestamp(&new_york, Disambiguation::Earliest)
            .unwrap();
        let second = twice.timestamp(&new_york, Disambiguation::Latest).unwrap();
        assert_eq!(second.seconds() - first.seconds(), 3_600);
    }
}

#[cfg(feature = "humanize")]
#[test]
fn humanize_is_reachable_from_the_facade() {
    use hyper_calendar::hc_humanize::natural::{DeltaOptions, Natural};
    let natural = Natural::english();
    assert_eq!(
        natural
            .naturaldelta(TimeDelta::from_minutes(30).inner(), DeltaOptions::default())
            .unwrap(),
        "30 minutes"
    );
}
