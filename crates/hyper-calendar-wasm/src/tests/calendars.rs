use super::super::*;
use super::read_lines;

/// 21 March 2005, a Monday of March, is Başgün of Nowruz in Turkmen
/// (Wikipedia's names of 2002 to 2008); in Russian it is ordinary.
#[test]
fn a_turkmen_day_of_2005_is_named_by_the_period() {
    let day = hc_gregorian_to_fixed(2005, 3, 21);
    let read = |calendar: &str, locale: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_naming_period_on(
                calendar.as_ptr(),
                calendar.len(),
                day,
                locale.as_ptr(),
                locale.len(),
                buffer,
                capacity,
            )
        })
    };
    let text = read("gregory", "tk-TM");
    assert!(
        text.starts_with("in-force\tturkmen-2002\tNowruz\tBaşgün\tFirst day\t"),
        "{text}"
    );
    assert_eq!(text.trim_end_matches('\n').split('\t').count(), 9);
    assert_eq!(read("gregory", "ru"), "ordinary\t\t\t\t\t\t\t\t\n");
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_naming_period_on("maya".as_ptr(), 4, day, "tk".as_ptr(), 2, null, 0) },
        HC_ERR_UNKNOWN
    );
    let nothing = core::ptr::null();
    assert_eq!(
        unsafe { hc_naming_period_on(nothing, 7, day, "tk".as_ptr(), 2, null, 0) },
        HC_ERR_NULL_POINTER
    );
    assert_eq!(
        unsafe { hc_naming_period_on("gregory".as_ptr(), 7, day, nothing, 2, null, 0) },
        HC_ERR_NULL_POINTER
    );
    let not_utf8 = [0xff_u8];
    assert_eq!(
        unsafe { hc_naming_period_on(not_utf8.as_ptr(), 1, day, "tk".as_ptr(), 2, null, 0) },
        HC_ERR_NOT_UTF8
    );
    assert_eq!(
        unsafe { hc_naming_period_on("gregory".as_ptr(), 7, day, not_utf8.as_ptr(), 1, null, 0) },
        HC_ERR_NOT_UTF8
    );
}

/// 令和8年9月28日 reads back as 28 September 2026, the Japanese calendar's
/// line of `hc_describe_day` and the fixed day; a text with a year of two
/// digits is a line that says so; the strings fail as every export's do.
#[test]
fn a_written_date_reads_back_as_its_line() {
    let read = |calendar: &str, locale: &str, text: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_parse_date(
                calendar.as_ptr(),
                calendar.len(),
                locale.as_ptr(),
                locale.len(),
                text.as_ptr(),
                text.len(),
                buffer,
                capacity,
            )
        })
    };
    let line = read("japanese", "ja", "令和8年9月28日");
    let cells: Vec<&str> = line.trim_end_matches('\n').split('\t').collect();
    assert_eq!(cells.len(), hc::lines::PARSE_DATE_COLUMNS);
    assert_eq!(
        cells[..5],
        ["japanese", "Japanese (imperial eras)", "reiwa", "令和", "8"]
    );
    assert_eq!(cells[15], "令和8年9月28日");
    assert_eq!(cells[18], "739887");
    let refused = read("gregory", "en", "September 28, 26");
    assert!(refused.contains("\t104\ttwo-digit-year\t"), "{refused}");
    assert!(refused.ends_with("\ten\t\t\n"), "{refused}");
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe {
            hc_parse_date(
                "maya".as_ptr(),
                4,
                "en".as_ptr(),
                2,
                "1".as_ptr(),
                1,
                null,
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
    let nothing = core::ptr::null();
    assert_eq!(
        unsafe { hc_parse_date(nothing, 7, "en".as_ptr(), 2, "1".as_ptr(), 1, null, 0) },
        HC_ERR_NULL_POINTER
    );
    let not_utf8 = [0xff_u8];
    assert_eq!(
        unsafe {
            hc_parse_date(
                "gregory".as_ptr(),
                7,
                "en".as_ptr(),
                2,
                not_utf8.as_ptr(),
                1,
                null,
                0,
            )
        },
        HC_ERR_NOT_UTF8
    );
}

/// 2026-09-21, described for a locale.
fn describe(locale: &str) -> Vec<Vec<String>> {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_describe_day(739_880, locale.as_ptr(), locale.len(), buffer, capacity)
    });
    text.lines()
        .map(|line| line.split('\t').map(str::to_owned).collect())
        .collect()
}

fn row<'a>(rows: &'a [Vec<String>], id: &str) -> &'a [String] {
    rows.iter()
        .find(|row| row[0] == id)
        .unwrap_or_else(|| panic!("no row for {id}"))
}

fn first_day(locale: &str) -> i64 {
    unsafe { hc_first_day_of_week(locale.as_ptr(), locale.len()) }
}

#[test]
fn the_first_day_of_the_week_follows_the_locale() {
    // By region, by the language's likely region, and by `-u-fw-`.
    for (tag, day) in [
        ("en-US", 7),
        ("en-GB", 1),
        ("en", 7),
        ("ja", 7),
        ("zh-Hans", 1),
        ("ar-SA", 7),
        ("ar-EG", 6),
        ("fr", 1),
        ("pt", 7),
        ("pt-PT", 7),
        ("de-u-fw-sun", 7),
        ("und", 1),
        ("not a tag", 1),
        ("", 1),
    ] {
        assert_eq!(first_day(tag), day, "{tag}");
    }
    assert_eq!(
        unsafe { hc_first_day_of_week(core::ptr::null(), 0) },
        1,
        "a null empty tag is the root locale"
    );
    assert_eq!(
        unsafe { hc_first_day_of_week(core::ptr::null(), 2) },
        HC_ERR_NULL_POINTER
    );
    let bad = [0xff_u8, 0xfe];
    assert_eq!(
        unsafe { hc_first_day_of_week(bad.as_ptr(), bad.len()) },
        HC_ERR_NOT_UTF8
    );
}

#[test]
fn every_registered_calendar_is_a_line_with_eighteen_columns() {
    let rows = describe("en");
    assert_eq!(rows.len(), hc::registry().len());
    let ids: Vec<&str> = hc::registry().metas().map(|meta| meta.id.0).collect();
    let listed: Vec<&str> = rows.iter().map(|row| row[0].as_str()).collect();
    assert_eq!(listed, ids, "registry order");
    for row in &rows {
        assert_eq!(row.len(), 18, "{row:?}");
        // A midnight start needs no naming; every other one has it.
        assert_eq!(row[17].is_empty(), row[14] == "midnight", "{row:?}");
        assert!(["", "start", "end"].contains(&row[17].as_str()), "{row:?}");
        // Every calendar states where its day begins and which
        // locale answered.
        assert!(!row[14].is_empty(), "{row:?}");
        assert!(!row[16].is_empty(), "{row:?}");
        // A converted day is formatted; a refused one is not.
        assert_eq!(row[15].is_empty(), !row[11].is_empty(), "{row:?}");
    }
}

#[test]
fn a_converted_day_decodes_column_by_column() {
    let rows = describe("ja-JP");
    let japanese = row(&rows, "japanese");
    assert_eq!(
        japanese,
        [
            "japanese",
            "Japanese (imperial eras)",
            "reiwa",
            "令和",
            "8",
            "9",
            "0",
            "9月",
            "21",
            "0",
            "",
            "",
            "",
            // The era calendar has been in use since 862, and never
            // abandoned.
            "in-use",
            "midnight",
            "令和8年9月21日",
            "ja",
            ""
        ]
    );
    let gregorian = row(&rows, "gregory");
    assert_eq!(gregorian[0..2], ["gregory", "Gregorian"]);
    assert_eq!(gregorian[4..10], ["2026", "9", "0", "9月", "21", "0"]);
    assert_eq!(gregorian[11..14], ["", "", "in-use"]);
    assert_eq!(gregorian[15..17], ["2026年9月21日", "ja"]);
    // Japanese has no words for the Hebrew months, so the Hebrew
    // calendar answers in English, not Hebrew, and says so.
    let hebrew = row(&rows, "hebrew");
    assert_eq!(hebrew[16], "en");
    assert!(hebrew[7].is_ascii(), "{hebrew:?}");
    // The last column names the civil day a day is named after: the
    // Hebrew day that begins at sunset by the one it ends on, the
    // Julian Day that begins at noon by the one it begins on.
    assert_eq!(
        (hebrew[14].as_str(), hebrew[17].as_str()),
        ("sunset", "end")
    );
    let julian_day = row(&rows, "julian-day");
    assert_eq!(
        (julian_day[14].as_str(), julian_day[17].as_str()),
        ("noon", "start")
    );
    let tibetan = row(&rows, "tibetan");
    assert_eq!(
        (tibetan[14].as_str(), tibetan[17].as_str()),
        ("local-time 05:00:00", "start")
    );
    let rows = describe("en");
    assert_eq!(row(&rows, "gregory")[7], "September");
    assert_eq!(row(&rows, "gregory")[15..17], ["September 21, 2026", "en"]);
    assert_eq!(row(&rows, "hebrew")[16], "en");
    // A tag with no data falls back to the root locale, whose month
    // names are CLDR's `M01`..`M12` rather than English.
    let rows = describe("tlh");
    assert_eq!(row(&rows, "gregory")[7], "M09");
    assert_eq!(row(&rows, "gregory")[16], "und");
    // A tag that does not parse at all falls back the same way.
    let rows = describe("!!");
    assert_eq!(row(&rows, "gregory")[7], "M09");
    // And `native` renders each calendar in its own language.
    let rows = describe("native");
    assert_eq!(row(&rows, "gregory")[16], "en");
    assert_eq!(row(&rows, "japanese")[15..17], ["令和8年9月21日", "ja"]);
    assert_eq!(row(&rows, "hebrew")[16], "he");
    assert_eq!(row(&rows, "chinese")[16], "zh-Hans");
}

#[test]
fn a_leap_month_and_the_extra_fields_are_carried() {
    // 2023-03-22 was 閏二月初一 in the Chinese calendar.
    let day = hc_gregorian_to_fixed(2023, 3, 22);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_describe_day(day, "zh-Hans".as_ptr(), 7, buffer, capacity)
    });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    let chinese = rows
        .iter()
        .find(|row| row[0] == "chinese")
        .expect("chinese");
    assert_eq!(chinese[5..10], ["2", "1", "闰二月", "1", "0"]);
    assert!(chinese[10].contains("cycle="), "{chinese:?}");
    assert!(chinese[10].contains(';'), "{chinese:?}");
    assert_eq!(chinese[15..17], ["2023癸卯年闰二月初一", "zh-Hans"]);
}

#[test]
fn a_refusal_is_a_line_with_its_code_and_name() {
    let rows = describe("en");
    // The Rumi calendar was kept only from 1840 to 1925.
    let rumi = row(&rows, "rumi");
    assert_eq!(rumi[0..2], ["rumi", "Rumi"]);
    assert!(rumi[2..11].iter().all(String::is_empty), "{rumi:?}");
    assert_eq!(rumi[11..14], ["7", "after-supported-range", ""]);
    assert_eq!(rumi[14..17], ["midnight", "", "en"]);
}

/// The units of one calendar over a range, decoded.
fn walk(id: &str, unit: u32, from: i64, to: i64, locale: &str) -> Vec<Vec<String>> {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_calendar_units(
            id.as_ptr(),
            id.len(),
            unit,
            from,
            to,
            locale.as_ptr(),
            locale.len(),
            buffer,
            capacity,
        )
    });
    text.lines()
        .map(|line| line.split('\t').map(str::to_owned).collect())
        .collect()
}

#[test]
fn the_units_of_a_calendar_are_labelled_spans() {
    // The Japanese eras from the Meiji Restoration to Reiwa.
    let eras = walk(
        "japanese",
        0,
        hc_gregorian_to_fixed(1868, 1, 1),
        hc_gregorian_to_fixed(2019, 12, 31),
        "ja",
    );
    let labels: Vec<&str> = eras.iter().map(|row| row[2].as_str()).collect();
    assert!(
        labels.ends_with(&["明治", "大正", "昭和", "平成", "令和"]),
        "{labels:?}"
    );
    let reiwa = eras.last().expect("reiwa");
    assert_eq!(reiwa.len(), 8);
    assert_eq!(reiwa[0], hc_gregorian_to_fixed(2019, 5, 1).to_string());
    // The imperial-era calendar records no period of use.
    assert_eq!(reiwa[3..8], ["0", "in-use", "", "", "ja"]);
    for pair in eras.windows(2) {
        assert_eq!(pair[0][1], pair[1][0], "spans touch");
    }
    // The same eras in English, as CLDR 48's `root.xml` names them, with
    // the years of every era before Meiji.
    let eras = walk(
        "japanese",
        0,
        hc_gregorian_to_fixed(1850, 1, 1),
        hc_gregorian_to_fixed(1870, 1, 1),
        "en",
    );
    let labels: Vec<&str> = eras.iter().map(|row| row[2].as_str()).collect();
    assert!(
        labels.contains(&"Kaei (1848–1854)") && labels.contains(&"Meiji"),
        "{labels:?}"
    );
    // A first year is 元年 and the years after it are numbered.
    let years = walk(
        "japanese",
        1,
        hc_gregorian_to_fixed(2019, 5, 1),
        hc_gregorian_to_fixed(2020, 1, 2),
        "ja",
    );
    let labels: Vec<&str> = years.iter().map(|row| row[2].as_str()).collect();
    assert_eq!(labels, ["令和元年", "令和2年"]);
    // The Chinese months of 2023 carry the leap second month.
    let months = walk(
        "chinese",
        2,
        hc_gregorian_to_fixed(2023, 1, 22),
        hc_gregorian_to_fixed(2024, 2, 10),
        "zh-Hans",
    );
    assert_eq!(months.len(), 13, "{months:?}");
    let leap = months
        .iter()
        .find(|row| row[3] == "1")
        .expect("a leap month");
    assert_eq!(leap[2], "闰二月");
    assert_eq!(leap[0], hc_gregorian_to_fixed(2023, 3, 22).to_string());
    assert_eq!(months[0][2], "正月");
    // A calendar's edges are refusals with the calendar's own error.
    let years = walk(
        "rumi",
        1,
        hc_gregorian_to_fixed(1925, 1, 1),
        hc_gregorian_to_fixed(1927, 1, 1),
        "en",
    );
    let last = years.last().expect("a span");
    assert_eq!(last[2..7], ["", "", "", "7", "after-supported-range"]);
    assert_eq!(last[7], "en");
    // A unit the calendar does not have is one refusal.
    let months = walk("maya-longcount", 2, 0, 1_000, "en");
    assert_eq!(months.len(), 1);
    assert_eq!(months[0][5..7], ["5", "unsupported-field"]);
    let days = walk("maya-longcount", 3, 0, 3, "en");
    assert_eq!(days.len(), 3);
    assert!(!days[0][2].is_empty(), "{days:?}");
}

#[test]
fn thirty_years_of_chinese_months_are_walked_by_month_not_by_day() {
    let started = std::time::Instant::now();
    let months = walk(
        "chinese",
        2,
        hc_gregorian_to_fixed(1996, 1, 1),
        hc_gregorian_to_fixed(2026, 1, 1),
        "zh-Hans",
    );
    let elapsed = started.elapsed();
    assert!(months.len() >= 370, "{} months", months.len());
    // About a third of a second in a release build on a developer
    // machine; a shared CI runner, with the suite running in
    // parallel, has taken one and a half. The bound is a regression
    // guard against a day-by-day walk, which takes minutes, not a
    // benchmark. A debug build is too slow and too variable for any
    // bound, so the time is only reported there.
    if !cfg!(debug_assertions) {
        assert!(
            elapsed.as_secs_f64() < 5.0,
            "{} months took {elapsed:?}",
            months.len()
        );
    }
    eprintln!("{} Chinese months walked in {elapsed:?}", months.len());
}

#[test]
fn units_refuse_what_they_do_not_know() {
    let id = "no-such-calendar";
    assert_eq!(
        unsafe {
            hc_calendar_units(
                id.as_ptr(),
                id.len(),
                1,
                0,
                10,
                "en".as_ptr(),
                2,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
    let id = "gregory";
    assert_eq!(
        unsafe {
            hc_calendar_units(
                id.as_ptr(),
                id.len(),
                4,
                0,
                10,
                "en".as_ptr(),
                2,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
    // An empty range is no text at all.
    assert_eq!(
        unsafe {
            hc_calendar_units(
                id.as_ptr(),
                id.len(),
                1,
                10,
                10,
                "en".as_ptr(),
                2,
                core::ptr::null_mut(),
                0,
            )
        },
        0
    );
    // At the cap, 100 000 days as days, the text is measured; one
    // day more, or a trillion, is refused without being built.
    let units_of = |unit: u32, from: i64, to: i64| unsafe {
        hc_calendar_units(
            id.as_ptr(),
            id.len(),
            unit,
            from,
            to,
            "en".as_ptr(),
            2,
            core::ptr::null_mut(),
            0,
        )
    };
    let cap = hc::lines::MAX_CALENDAR_UNITS as i64;
    assert_eq!(cap, 100_000);
    assert!(units_of(3, 700_000, 700_000 + cap) > 0);
    assert_eq!(units_of(3, 700_000, 700_000 + cap + 1), HC_ERR_OUT_OF_RANGE);
    assert_eq!(units_of(3, 0, 1_000_000_000_000), HC_ERR_OUT_OF_RANGE);
    // The cap counts lines, not days: a millennium of years is a
    // thousand lines.
    assert!(units_of(1, 700_000, 700_000 + 365_243) > 0);
    let not_utf8 = [0xffu8];
    assert_eq!(
        unsafe {
            hc_calendar_units(
                not_utf8.as_ptr(),
                1,
                1,
                0,
                10,
                "en".as_ptr(),
                2,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_NOT_UTF8
    );
}

#[test]
fn the_gregorian_adoption_is_one_line_per_step() {
    let region = |code: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_gregorian_adoption(code.as_ptr(), code.len(), buffer, capacity)
        })
    };
    let japan = region("JP");
    let rows: Vec<Vec<&str>> = japan
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].len(), 7);
    // 1872-12-31 was the Tenpō calendar's last day.
    assert_eq!(
        rows[0][..4],
        ["683734", "683735", "japanese-tenpo", "civil"]
    );
    assert!(rows[0][4].contains("337"), "{rows:?}");
    assert_eq!(rows[0][5..], ["gregory", "Japan"]);
    assert_eq!(region("se").lines().count(), 3);
    assert_eq!(
        unsafe { hc_gregorian_adoption("ZZ".as_ptr(), 2, core::ptr::null_mut(), 0) },
        0,
        "an unknown region writes nothing"
    );
}

#[test]
fn every_calendar_and_every_locale_is_listed() {
    let today = 739_880;
    let text = read_lines(|buffer, capacity| unsafe {
        hc_calendars(today, "ja".as_ptr(), 2, buffer, capacity)
    });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), hc::registry().len());
    assert!(rows.iter().all(|row| row.len() == 11), "{rows:?}");
    let gregorian = rows
        .iter()
        .find(|row| row[0] == "gregory")
        .expect("gregory");
    assert_eq!(gregorian[1..3], ["西暦(グレゴリオ暦)", "Gregorian"]);
    assert!(
        !gregorian[3].is_empty() && !gregorian[4].is_empty(),
        "{gregorian:?}"
    );
    assert_eq!(gregorian[5..11], ["0", "1", "1", "1", "", "in-use"]);
    let japanese = rows
        .iter()
        .find(|row| row[0] == "japanese")
        .expect("japanese");
    assert_eq!(japanese[1], "和暦");
    assert_eq!(japanese[5..10], ["1", "1", "1", "1", "ja"]);
    let chinese = rows
        .iter()
        .find(|row| row[0] == "chinese")
        .expect("chinese");
    assert_eq!(chinese[9], "zh-Hans;zh-Hant");
    let long_count = rows
        .iter()
        .find(|row| row[0] == "maya-longcount")
        .expect("long count");
    assert_eq!(long_count[5..10], ["0", "0", "0", "0", "yua"]);
    let rumi = rows.iter().find(|row| row[0] == "rumi").expect("rumi");
    assert!(
        ["in-use", "proleptic", "extended", "unrecorded"].contains(&rumi[10]),
        "{rumi:?}"
    );

    let text = read_lines(|buffer, capacity| unsafe { hc_locales(buffer, capacity) });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), hc::hc_i18n::data::LOCALES.len());
    assert!(rows.iter().all(|row| row.len() == 7), "{rows:?}");
    let ja = rows.iter().find(|row| row[0] == "ja").expect("ja");
    assert_eq!(ja[1..6], ["Japanese", "日本語", "1", "1", "1"]);
    let named: Vec<&str> = ja[6].split(';').collect();
    assert!(
        named.contains(&"japanese") && named.contains(&"chinese"),
        "{named:?}"
    );
    assert!(!named.contains(&"gregory"), "{named:?}");
    let coptic = rows.iter().find(|row| row[0] == "cop").expect("cop");
    assert_eq!(coptic[3..6], ["0", "0", "0"]);
    assert_eq!(coptic[6], "coptic");
}

#[test]
fn the_calendar_list_is_the_calendars_names_without_the_day() {
    let split = |text: &str| -> Vec<Vec<String>> {
        text.lines()
            .map(|line| line.split('\t').map(str::to_owned).collect())
            .collect()
    };
    for locale in ["ja", "en", "he", "und", "native"] {
        let list = split(&read_lines(|buffer, capacity| unsafe {
            hc_calendar_list(locale.as_ptr(), locale.len(), buffer, capacity)
        }));
        let calendars = split(&read_lines(|buffer, capacity| unsafe {
            hc_calendars(739_880, locale.as_ptr(), locale.len(), buffer, capacity)
        }));
        assert_eq!(list.len(), hc::registry().len(), "{locale}");
        assert!(list.iter().all(|row| row.len() == 6), "{list:?}");
        for (row, full) in list.iter().zip(&calendars) {
            assert_eq!(row[..3], full[..3], "{locale}");
            assert_eq!(row[5], full[9], "the native locales, {row:?}");
            assert_eq!(row[1].is_empty(), row[3].is_empty(), "{row:?}");
        }
    }
    let list = split(&read_lines(|buffer, capacity| unsafe {
        hc_calendar_list("ja".as_ptr(), 2, buffer, capacity)
    }));
    let row = |id: &str| list.iter().find(|row| row[0] == id).expect(id).clone();
    assert_eq!(
        row("japanese")[1..],
        [
            "和暦",
            "Japanese (imperial eras)",
            "ja",
            "hc-calendars-regional",
            "ja"
        ]
    );
    assert_eq!(row("gregory")[3..], ["ja", "hc-calendars-solar", ""]);
    assert_eq!(row("chinese")[5], "zh-Hans;zh-Hant");
    assert_eq!(row("chinese")[4], "hc-calendars-lunar");
    assert_eq!(row("persian")[4], "hc-calendars-equinox");
    assert_eq!(row("hindu-lunar")[4], "hc-calendars-indic");
    let native = split(&read_lines(|buffer, capacity| unsafe {
        hc_calendar_list("native".as_ptr(), 6, buffer, capacity)
    }));
    let hebrew = native
        .iter()
        .find(|row| row[0] == "hebrew")
        .expect("hebrew");
    assert_eq!(hebrew[1], "לוח השנה העברי");
    assert_eq!(hebrew[3], "he");
    let not_utf8 = [0xffu8];
    assert_eq!(
        unsafe { hc_calendar_list(not_utf8.as_ptr(), 1, core::ptr::null_mut(), 0) },
        HC_ERR_NOT_UTF8
    );
}

#[test]
fn the_locale_argument_fails_as_text_does() {
    let not_utf8 = [0xffu8];
    assert_eq!(
        unsafe { hc_describe_day(739_880, not_utf8.as_ptr(), 1, core::ptr::null_mut(), 0) },
        HC_ERR_NOT_UTF8
    );
    assert_eq!(
        unsafe { hc_describe_day(739_880, core::ptr::null(), 3, core::ptr::null_mut(), 0) },
        HC_ERR_NULL_POINTER
    );
    // A null locale with no length is `und`.
    let needed =
        unsafe { hc_describe_day(739_880, core::ptr::null(), 0, core::ptr::null_mut(), 0) };
    assert!(needed > 0);
}
