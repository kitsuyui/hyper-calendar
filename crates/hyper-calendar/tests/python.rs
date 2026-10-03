//! The `civil` layer against the examples of Python's `datetime`
//! documentation (<https://docs.python.org/3/library/datetime.html>,
//! retrieved 2026-09-26). Each test quotes the example it reproduces; the
//! expected values are the documentation's, not the output of running
//! Python, except in the tests that say they are an interpreter's answer
//! (CPython 3.14.7) to a case the documentation states a rule for.
//! `docs/python-parity.md` is the full correspondence.

use hyper_calendar::civil::{
    Date, DateTime, Replace, Resolution, StructTime, Time, TimeDelta, TimeDeltaParts, calendar,
};

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

/// The documentation's rule: a result of `timedelta * float`, `/ float` and
/// `/ int` is "rounded to the nearest multiple of timedelta.resolution using
/// round-half-to-even", where the float is multiplied as the exact ratio
/// `float.as_integer_ratio()` gives. The expected values are CPython 3.14.7's
/// answers; the audit's case (a22) is the first.
#[test]
fn scaling_by_a_float_rounds_as_python_does() {
    let micro = |count: i64| TimeDelta::from_micros(count);
    let times =
        |span: TimeDelta, factor: f64| span.checked_scale_at(factor, Resolution::Microsecond);
    // `timedelta(microseconds=25508964008398) * 0.5`:
    // `datetime.timedelta(days=147, seconds=53682, microseconds=4199)`.
    let long = micro(25_508_964_008_398);
    assert_eq!(times(long, 0.5), Ok(micro(12_754_482_004_199)));
    // The same product at the attosecond is exact: no `.00419900007545948`.
    assert_eq!(long.checked_scale(0.5), Ok(micro(12_754_482_004_199)));
    assert_eq!(times(micro(1), 0.5), Ok(micro(0)));
    assert_eq!(times(micro(3), 0.5), Ok(micro(2)));
    assert_eq!(times(micro(5), 0.5), Ok(micro(2)));
    assert_eq!(times(micro(-5), 0.5), Ok(micro(-2)));
    assert_eq!(times(micro(10), 0.1), Ok(micro(1)));
    assert_eq!(times(micro(1_000_000), 1.1), Ok(micro(1_100_000)));
    // `td / int` and `td / float`.
    let by_int =
        |span: TimeDelta, divisor| span.checked_div_nearest_at(divisor, Resolution::Microsecond);
    assert_eq!(by_int(micro(7), 2), Ok(micro(4)));
    assert_eq!(by_int(micro(5), 2), Ok(micro(2)));
    assert_eq!(by_int(micro(5), -2), Ok(micro(-2)));
    assert_eq!(
        micro(10).checked_div_f64_at(0.3, Resolution::Microsecond),
        Ok(micro(33))
    );
    // `td // int` floors, and at the attosecond this library keeps the rest.
    assert_eq!(micro(7) / 2, TimeDelta::from_nanos(3_500));
}

/// `timedelta / timedelta` is the integer true division of the two spans in
/// microseconds, which Python rounds correctly. CPython 3.14.7's answers.
#[test]
fn the_ratio_of_two_timedeltas_is_correctly_rounded() {
    let micro = |count: i64| TimeDelta::from_micros(count);
    assert_eq!(micro(1).ratio(micro(3)), Ok(0.333_333_333_333_333_3));
    assert_eq!(micro(2).ratio(micro(3)), Ok(0.666_666_666_666_666_6));
    assert_eq!(
        TimeDelta::from_days(1).ratio(micro(3)),
        Ok(28_800_000_000.0)
    );
    assert_eq!(
        micro(86_400_000_000_000).ratio(micro(7)),
        Ok(12_342_857_142_857.143)
    );
    assert_eq!(
        TimeDelta::from_days(365).ratio(TimeDelta::from_days(73)),
        Ok(5.0)
    );
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

/// Audit 10, a14: the 362 days of the last Gregorian year from 4 January
/// were given week 1, by the branch that "cannot be reached". The week dates
/// are from the algorithm of Wikipedia, "ISO week date", read 2026-10-03
/// (worked in `scripts/iso-week-pins.py`).
#[test]
fn isocalendar_is_right_at_both_ends_of_the_range() {
    let last = Date::MAX.iso_calendar();
    assert_eq!((last.year, last.week, last.weekday), (9_999_999, 52, 5));
    let january = date(9_999_999, 1, 4).iso_calendar();
    assert_eq!(
        (january.year, january.week, january.weekday),
        (9_999_999, 1, 1)
    );
    let before = date(9_999_999, 1, 3).iso_calendar();
    assert_eq!(
        (before.year, before.week, before.weekday),
        (9_999_998, 53, 7)
    );
    let first = Date::MIN.iso_calendar();
    assert_eq!((first.year, first.week, first.weekday), (-9_999_999, 1, 1));
    assert_eq!(Date::from_iso_calendar(9_999_999, 52, 5), Ok(Date::MAX));
    assert_eq!(Date::from_iso_calendar(-9_999_999, 1, 1), Ok(Date::MIN));
    // Every day of the last year has a week date that gives the day back.
    let mut day = date(9_999_999, 1, 1);
    for _ in 0..365 {
        let week = day.iso_calendar();
        assert_eq!(
            Date::from_iso_calendar(week.year, week.week, week.weekday),
            Ok(day)
        );
        if day == Date::MAX {
            break;
        }
        day = day.add_days(1).unwrap();
    }
    assert_eq!(day, Date::MAX);
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

// --- time.struct_time and the calendar module -------------------------------

/// The `time` module's documentation:
///
/// ```text
/// >>> time.gmtime(0)
/// time.struct_time(tm_year=1970, tm_mon=1, tm_mday=1, tm_hour=0, tm_min=0,
///                  tm_sec=0, tm_wday=3, tm_yday=1, tm_isdst=0)
/// >>> from time import gmtime, strftime
/// >>> strftime("%a, %d %b %Y %H:%M:%S +0000", gmtime())
/// 'Thu, 28 Jun 2001 14:17:15 +0000'
/// ```
///
/// and "`time.gmtime()` and `calendar.timegm()` are each other's inverse".
/// The documentation gives the second example at a moment it does not name;
/// 993737835 is that moment (CPython 3.14.7 reads it back as the string).
#[test]
fn gmtime_and_timegm_are_each_others_inverse() {
    let epoch = StructTime::gmtime(0).unwrap();
    assert_eq!(
        epoch,
        StructTime {
            tm_year: 1970,
            tm_mon: 1,
            tm_mday: 1,
            tm_hour: 0,
            tm_min: 0,
            tm_sec: 0,
            tm_wday: 3,
            tm_yday: 1,
            tm_isdst: 0
        }
    );
    let moment = StructTime::gmtime(993_737_835).unwrap();
    assert_eq!(
        moment.strftime("%a, %d %b %Y %H:%M:%S +0000").unwrap(),
        "Thu, 28 Jun 2001 14:17:15 +0000"
    );
    for seconds in [
        -1,
        0,
        1,
        993_737_835,
        -62_135_596_800,
        253_402_300_799,
        1_791_030_896,
    ] {
        let tuple = StructTime::gmtime(seconds).unwrap();
        assert_eq!(tuple.timegm(), Ok(seconds), "{seconds}");
    }
    let before = StructTime::gmtime(-1).unwrap();
    assert_eq!(
        (before.tm_year, before.tm_mon, before.tm_mday),
        (1969, 12, 31)
    );
    assert_eq!((before.tm_wday, before.tm_yday), (2, 365));
}

/// `calendar.timegm` adds the fields up without checking the day, hour,
/// minute or second (CPython 3.14.7: `timegm((2026, 12, 31, 24, 60, 60))`
/// is 1798765260, `timegm((2000, 2, 30, 0, 0, 0))` is 951868800) and refuses
/// a month outside 1 to 12.
#[test]
fn timegm_does_not_check_the_fields_below_the_month() {
    let tuple = |year, month, day, hour, minute, second| StructTime {
        tm_year: year,
        tm_mon: month,
        tm_mday: day,
        tm_hour: hour,
        tm_min: minute,
        tm_sec: second,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
    };
    assert_eq!(tuple(2026, 10, 3, 12, 34, 56).timegm(), Ok(1_791_030_896));
    assert_eq!(tuple(2026, 12, 31, 24, 60, 60).timegm(), Ok(1_798_765_260));
    assert_eq!(tuple(2000, 2, 30, 0, 0, 0).timegm(), Ok(951_868_800));
    assert_eq!(tuple(1969, 12, 31, 23, 59, 59).timegm(), Ok(-1));
    assert!(tuple(2026, 13, 1, 0, 0, 0).timegm().is_err());
}

/// ```text
/// >>> time.strptime("30 Nov 00", "%d %b %y")
/// time.struct_time(tm_year=2000, tm_mon=11, tm_mday=30, tm_hour=0, tm_min=0,
///                  tm_sec=0, tm_wday=3, tm_yday=335, tm_isdst=-1)
/// >>> time.asctime(...)   # 'Sun Jun 20 23:21:05 1993'
/// ```
///
/// and "the day field is two characters long and is space padded if the day
/// is a single digit, e.g.: `'Wed Jun  9 04:26:40 1993'`".
#[test]
fn strptime_and_asctime_match_the_time_module_documentation() {
    let parsed = StructTime::strptime("30 Nov 00", "%d %b %y").unwrap();
    assert_eq!(
        (
            parsed.tm_year,
            parsed.tm_mon,
            parsed.tm_mday,
            parsed.tm_hour,
            parsed.tm_wday,
            parsed.tm_yday,
            parsed.tm_isdst
        ),
        (2000, 11, 30, 0, 3, 335, -1)
    );
    let june = |day, hour, minute, second| {
        StructTime::from_date_time(
            DateTime::from_parts(1993, 6, day, hour, minute, second, 0).unwrap(),
            -1,
        )
        .asctime()
        .unwrap()
    };
    assert_eq!(june(20, 23, 21, 5), "Sun Jun 20 23:21:05 1993");
    assert_eq!(june(9, 4, 26, 40), "Wed Jun  9 04:26:40 1993");
    let timetuple = StructTime::from_date_time(
        DateTime::from_parts(2026, 10, 3, 12, 34, 56, 0).unwrap(),
        -1,
    );
    // `date.timetuple()` is midnight with an unknown flag.
    let date = StructTime::from_date(date(2026, 10, 3));
    assert_eq!(
        (date.tm_hour, date.tm_wday, date.tm_yday, date.tm_isdst),
        (0, 5, 276, -1)
    );
    assert_eq!(
        (timetuple.tm_sec, timetuple.tm_wday, timetuple.tm_yday),
        (56, 5, 276)
    );
    assert_eq!(
        timetuple.to_date_time(),
        DateTime::from_parts(2026, 10, 3, 12, 34, 56, 0)
    );
}

/// The `calendar` module: "`monthrange` returns weekday of first day of the
/// month and number of days in month", "`monthcalendar` returns a matrix
/// representing a month's calendar. Each row represents a week; days outside
/// of the month are represented by zeros", and "`leapdays` returns number of
/// leap years in the range from y1 to y2 (exclusive)". The values are
/// CPython 3.14.7's; the documentation gives the rules, not these numbers.
#[test]
fn the_calendar_module_functions_answer_as_python_does() {
    assert_eq!(calendar::monthrange(2026, 10), Ok((3, 31)));
    assert_eq!(calendar::monthrange(2024, 2), Ok((3, 29)));
    assert_eq!(calendar::monthrange(2023, 2), Ok((2, 28)));
    assert_eq!(calendar::monthrange(1900, 2), Ok((3, 28)));
    assert!(calendar::monthrange(2026, 13).is_err());
    assert_eq!(
        calendar::monthcalendar(2026, 10, 0).unwrap(),
        vec![
            [0, 0, 0, 1, 2, 3, 4],
            [5, 6, 7, 8, 9, 10, 11],
            [12, 13, 14, 15, 16, 17, 18],
            [19, 20, 21, 22, 23, 24, 25],
            [26, 27, 28, 29, 30, 31, 0],
        ]
    );
    // With Sunday first (`calendar.setfirstweekday(calendar.SUNDAY)`).
    assert_eq!(
        calendar::monthcalendar(2026, 10, 6).unwrap().first(),
        Some(&[0, 0, 0, 0, 1, 2, 3])
    );
    assert_eq!(calendar::monthcalendar(2026, 2, 0).unwrap().len(), 5);
    assert_eq!(calendar::leapdays(2000, 2101), 25);
    assert_eq!(calendar::leapdays(1900, 1901), 0);
    assert_eq!(calendar::leapdays(1, 5), 1);
    assert_eq!(calendar::leapdays(2000, 2000), 0);
    assert_eq!(calendar::leapdays(2020, 2000), -5);
    assert_eq!(calendar::leapdays(-4, 5), 3);
    assert!(calendar::is_leap_year(2000) && !calendar::is_leap_year(1900));
    assert_eq!(calendar::weekday(2026, 10, 3), Ok(5));
    assert_eq!(calendar::weekday(1, 1, 1), Ok(0));
    assert_eq!(calendar::weekday(1970, 1, 1), Ok(3));
}
