use super::super::*;
use super::read_lines;

/// A written date reads back as the module's line; a null text is the
/// empty one, and a null calendar is refused.
#[test]
fn a_written_date_reads_back_as_the_modules_line() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_parse_date(
            c"japanese".as_ptr(),
            c"ja".as_ptr(),
            c"令和8年9月28日".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(
        text,
        hc::lines::parse_date("japanese", "ja", "令和8年9月28日").expect("known")
    );
    assert!(text.ends_with("\t739887\n"), "{text}");
    let empty = read_lines(|buffer, capacity, written| unsafe {
        hc_parse_date(
            c"gregory".as_ptr(),
            c"en".as_ptr(),
            core::ptr::null(),
            buffer,
            capacity,
            written,
        )
    });
    assert!(empty.contains("\t101\tempty\t"), "{empty}");
    let mut written = 0usize;
    assert_eq!(
        unsafe {
            hc_parse_date(
                core::ptr::null(),
                c"en".as_ptr(),
                c"1".as_ptr(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe {
            hc_parse_date(
                c"no-such-calendar".as_ptr(),
                c"en".as_ptr(),
                c"1".as_ptr(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_UNKNOWN
    );
}

/// 21 March 2005 in Turkmen, the module's line: Başgün of Nowruz.
#[test]
fn the_naming_period_line_is_the_modules() {
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2005, 3, 21, &mut day) },
        HC_OK
    );
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_naming_period_on(
            c"gregory".as_ptr(),
            day,
            c"tk".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(
        text,
        hc::lines::naming_period_line(&hc::registry(), "tk", "gregory", day).expect("known")
    );
    assert!(
        text.starts_with("in-force\tturkmen-2002\tNowruz\tBaşgün\t"),
        "{text}"
    );
    let mut written = 0usize;
    assert_eq!(
        unsafe {
            hc_naming_period_on(
                core::ptr::null(),
                day,
                c"tk".as_ptr(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe {
            hc_naming_period_on(
                c"no-such-calendar".as_ptr(),
                day,
                c"tk".as_ptr(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_UNKNOWN
    );
    let not_utf8 = c"\xff";
    for (calendar, locale) in [(not_utf8, c"tk"), (c"gregory", not_utf8)] {
        assert_eq!(
            unsafe {
                hc_naming_period_on(
                    calendar.as_ptr(),
                    day,
                    locale.as_ptr(),
                    core::ptr::null_mut(),
                    0,
                    &mut written,
                )
            },
            HC_ERROR_NOT_UTF8
        );
    }
}

#[test]
fn the_first_day_of_the_week_follows_the_locale() {
    let first_day = |locale: *const core::ffi::c_char| {
        let mut day = 0_u8;
        let status = unsafe { hc_first_day_of_week(locale, &raw mut day) };
        assert_eq!(status, HC_OK);
        day
    };
    assert_eq!(first_day(c"en-US".as_ptr()), 7);
    assert_eq!(first_day(c"en-GB".as_ptr()), 1);
    assert_eq!(first_day(c"ja".as_ptr()), 7);
    assert_eq!(first_day(c"zh-Hans".as_ptr()), 1);
    assert_eq!(first_day(c"ar-SA".as_ptr()), 7);
    assert_eq!(first_day(c"fr".as_ptr()), 1);
    assert_eq!(first_day(c"und".as_ptr()), 1);
    assert_eq!(first_day(core::ptr::null()), 1);
    let mut day = 0_u8;
    assert_eq!(
        unsafe { hc_first_day_of_week(c"\xff".as_ptr(), &raw mut day) },
        HC_ERROR_NOT_UTF8
    );
    assert_eq!(
        unsafe { hc_first_day_of_week(c"en".as_ptr(), core::ptr::null_mut()) },
        HC_ERROR_NULL_POINTER
    );
}

#[test]
fn the_calendar_list_is_the_calendars_names_without_the_day() {
    let rows = |text: &str| -> Vec<Vec<String>> {
        text.lines()
            .map(|line| line.split('\t').map(str::to_owned).collect())
            .collect()
    };
    let list = rows(&read_lines(|buffer, capacity, written| unsafe {
        hc_calendar_list(c"ja-JP".as_ptr(), buffer, capacity, written)
    }));
    let calendars = rows(&read_lines(|buffer, capacity, written| unsafe {
        hc_calendars(739_880, c"ja-JP".as_ptr(), buffer, capacity, written)
    }));
    assert_eq!(list.len(), hc::registry().len());
    assert!(list.iter().all(|row| row.len() == 6), "{list:?}");
    for (row, full) in list.iter().zip(&calendars) {
        assert_eq!(row[..3], full[..3]);
        assert_eq!(row[5], full[9], "the native locales, {row:?}");
    }
    let japanese = list
        .iter()
        .find(|row| row[0] == "japanese")
        .expect("japanese");
    assert_eq!(japanese[3..], ["ja", "hc-calendars-regional", "ja"]);
    assert_eq!(
        unsafe {
            hc_calendar_list(
                c"\xff".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_NOT_UTF8
    );
}

#[test]
fn one_day_in_every_calendar_decodes_column_by_column() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_describe_day(739_880, c"ja-JP".as_ptr(), buffer, capacity, written)
    });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), hc::registry().len());
    assert!(rows.iter().all(|row| row.len() == 18), "{rows:?}");
    let japanese = rows
        .iter()
        .find(|row| row[0] == "japanese")
        .expect("japanese");
    assert_eq!(
        japanese[2..10],
        ["reiwa", "令和", "8", "9", "0", "9月", "21", "0"]
    );
    assert_eq!(japanese[11..14], ["", "", "in-use"]);
    assert_eq!(japanese[15..18], ["令和8年9月21日", "ja", ""]);
    let rumi = rows.iter().find(|row| row[0] == "rumi").expect("rumi");
    assert_eq!(rumi[11..14], ["7", "after-supported-range", ""]);
    // Neither Japanese nor Turkish names the Rumi calendar, so it
    // is rendered in English, and says so.
    assert_eq!(rumi[15..17], ["", "en"]);
    // Column 15, where the day begins, is one of the words the
    // documentation lists, and column 18 names the civil day by it:
    // `daybreak` is the medieval Icelandic day's, named by its start.
    for row in &rows {
        let boundary = row[14];
        assert!(
            ["midnight", "noon", "sunset", "sunrise", "daybreak"].contains(&boundary)
                || boundary.starts_with("local-time "),
            "{row:?}"
        );
        assert_eq!(row[17].is_empty(), boundary == "midnight", "{row:?}");
    }
    let icelandic = rows
        .iter()
        .find(|row| row[0] == "icelandic-medieval")
        .expect("icelandic-medieval");
    assert_eq!(icelandic[14], "daybreak");
    assert_eq!(icelandic[17], "start");
    // A null locale is `und`.
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_describe_day(739_880, core::ptr::null(), buffer, capacity, written)
    });
    assert!(text.contains("\tM09\t"), "{text}");
    assert_eq!(
        unsafe {
            hc_describe_day(
                739_880,
                c"\xff".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_NOT_UTF8
    );
}

#[test]
fn the_gregorian_adoption_is_one_line_per_step() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_gregorian_adoption(c"CN".as_ptr(), buffer, capacity, written)
    });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), 2, "{rows:?}");
    assert!(rows.iter().all(|row| row.len() == 7), "{rows:?}");
    assert_eq!(rows[0][..4], ["697977", "697978", "chinese", "partial"]);
    assert_eq!(rows[1][3], "civil");
    let mut written = 0usize;
    let mut buffer = [0 as c_char; 4];
    assert_eq!(
        unsafe { hc_gregorian_adoption(c"ZZ".as_ptr(), buffer.as_mut_ptr(), 4, &mut written) },
        HC_OK
    );
    assert_eq!(written, 1, "an unknown region is the empty string");
    assert_eq!(
        unsafe { hc_gregorian_adoption(core::ptr::null(), buffer.as_mut_ptr(), 4, &mut written) },
        HC_ERROR_NULL_POINTER
    );
}

#[test]
fn the_units_the_calendars_and_the_locales_are_lines_too() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_calendar_units(
            c"japanese".as_ptr(),
            0,
            // 1989-01-01 to 2019-12-31.
            726_103,
            737_424,
            c"ja".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    let labels: Vec<&str> = rows.iter().map(|row| row[2]).collect();
    assert_eq!(labels, ["昭和", "平成", "令和"]);
    assert!(rows.iter().all(|row| row.len() == 8), "{rows:?}");
    assert_eq!(rows[2][7], "ja");
    assert_eq!(
        unsafe {
            hc_calendar_units(
                c"no-such-calendar".as_ptr(),
                0,
                0,
                10,
                core::ptr::null(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_calendar_units(
                c"gregory".as_ptr(),
                4,
                0,
                10,
                core::ptr::null(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_calendar_units(
                core::ptr::null(),
                0,
                0,
                10,
                core::ptr::null(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_NULL_POINTER
    );

    // At the cap, 100 000 days as days, the text is measured; one
    // day more, or a trillion, is refused without being built, and
    // `written` is left alone.
    let units_of = |unit: u32, from: i64, to: i64, written: &mut usize| unsafe {
        hc_calendar_units(
            c"gregory".as_ptr(),
            unit,
            from,
            to,
            c"en".as_ptr(),
            core::ptr::null_mut(),
            0,
            written,
        )
    };
    let cap = hc::lines::MAX_CALENDAR_UNITS as i64;
    assert_eq!(cap, 100_000);
    let mut written = 0usize;
    assert_eq!(
        units_of(3, 700_000, 700_000 + cap, &mut written),
        HC_ERROR_BUFFER_TOO_SMALL
    );
    assert!(written > 100_000, "{written}");
    let mut untouched = 7usize;
    assert_eq!(
        units_of(3, 700_000, 700_000 + cap + 1, &mut untouched),
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        units_of(3, 0, 1_000_000_000_000, &mut untouched),
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(untouched, 7);
    assert_eq!(
        units_of(1, 700_000, 700_000 + 365_243, &mut written),
        HC_ERROR_BUFFER_TOO_SMALL
    );

    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_calendars(739_880, c"native".as_ptr(), buffer, capacity, written)
    });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), hc::registry().len());
    assert!(rows.iter().all(|row| row.len() == 11), "{rows:?}");
    let hebrew = rows.iter().find(|row| row[0] == "hebrew").expect("hebrew");
    assert_eq!(hebrew[2], "Hebrew");
    assert_eq!(hebrew[9], "he");
    assert!(!hebrew[1].is_empty(), "named in its own language");

    let text =
        read_lines(|buffer, capacity, written| unsafe { hc_locales(buffer, capacity, written) });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), hc::hc_i18n::data::LOCALES.len());
    assert!(rows.iter().all(|row| row.len() == 10), "{rows:?}");
    assert_eq!(rows[0][0], "aeb-Latn");
    // The parent, the default numbering system and the direction:
    // `parentLocales` send `en-GB` to `en-001`, `ar-EG` writes `arab`
    // (`docs/systems/locale-fallback.md`, from CLDR 48).
    let row = |tag: &str| rows.iter().find(|row| row[0] == tag).expect(tag).clone();
    assert_eq!(row("en-GB")[7..], ["en-001", "latn", "ltr"]);
    assert_eq!(row("ar-EG")[7..], ["ar", "arab", "rtl"]);
    assert_eq!(row("ja")[7..], ["und", "latn", "ltr"]);
}

/// The Khmer and Lao year types, the Maya counts under a named
/// correlation, the Akan day, the weton and Sri Lanka's Buddhist year
/// cross the C boundary as the module's lines.
#[test]
fn the_regional_cycles_and_counts_cross_the_c_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_southeast_asian_year_type(c"khmer".as_ptr(), 2568, buffer, capacity, written)
    });
    assert_eq!(
        Ok(text.clone()),
        hc::calendar_values::southeast_asian_year_type_line("khmer", 2568)
    );
    assert!(text.starts_with("khmer\t2568\tnormal\t354\t0\t"), "{text}");
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_southeast_asian_year_type(c"lao".as_ptr(), 1342, buffer, capacity, written)
    });
    assert!(
        text.starts_with("lao\t1342\textra-month\t384\t1\t"),
        "{text}"
    );
    let mut written = 0usize;
    for (calendar, year, status) in [
        (c"khmer".as_ptr(), 2443, HC_ERROR_OUT_OF_RANGE),
        (c"thai-lunar".as_ptr(), 2568, HC_ERROR_UNKNOWN),
        (core::ptr::null(), 2568, HC_ERROR_NULL_POINTER),
    ] {
        assert_eq!(
            unsafe {
                hc_southeast_asian_year_type(calendar, year, core::ptr::null_mut(), 0, &mut written)
            },
            status
        );
    }
    let epoch = 584_283 - 1_721_425;
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_maya_long_count(epoch, c"gmt".as_ptr(), buffer, capacity, written)
    });
    assert_eq!(
        text,
        "maya-longcount\t584283\t0.0.0.0.0\t0\t0\t0\t0\t0\t4\tAhau\t8\tCumku\n"
    );
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_maya_long_count(
            2_009_802 - 1_721_425,
            c"584286".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(
        Ok(text.clone()),
        hc::calendar_values::maya_long_count_line(2_009_802 - 1_721_425, "584286")
    );
    assert!(
        text.starts_with("maya-longcount-584286\t584286\t9.17.19.13.16\t"),
        "{text}"
    );
    for (fixed, correlation, status) in [
        (epoch - 1, c"gmt".as_ptr(), HC_ERROR_OUT_OF_RANGE),
        (epoch, c"lounsbury".as_ptr(), HC_ERROR_UNKNOWN),
        (epoch, core::ptr::null(), HC_ERROR_NULL_POINTER),
    ] {
        assert_eq!(
            unsafe {
                hc_maya_long_count(fixed, correlation, core::ptr::null_mut(), 0, &mut written)
            },
            status
        );
    }
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(1978, 1, 23, &mut day) },
        HC_OK
    );
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_akan_day(day, buffer, capacity, written)
    });
    assert_eq!(text, "0\t1\t1\tFo\t2\tƐdwoada\tDwo\tFo-Dwo\tFɔdwo\n");
    assert_eq!(
        unsafe { hc_akan_day(i64::MIN, core::ptr::null_mut(), 0, &mut written) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(1945, 8, 17, &mut day) },
        HC_OK
    );
    let text =
        read_lines(|buffer, capacity, written| unsafe { hc_weton(day, buffer, capacity, written) });
    assert_eq!(Ok(text.clone()), hc::calendar_values::weton_line(day));
    assert!(
        text.ends_with("\t6\tJemuwah\t1\tLegi\t6\t5\t11\tJemuwah Legi\n"),
        "{text}"
    );
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2025, 5, 12, &mut day) },
        HC_OK
    );
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_buddhist_lk_year(day, buffer, capacity, written)
    });
    assert_eq!(
        Ok(text.clone()),
        hc::calendar_values::buddhist_lk_year_line(day)
    );
    assert!(
        text.starts_with(&format!("2569\t2025\t{day}\t{day}\t")),
        "{text}"
    );
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2022, 12, 31, &mut day) },
        HC_OK
    );
    assert_eq!(
        unsafe { hc_buddhist_lk_year(day, core::ptr::null_mut(), 0, &mut written) },
        HC_ERROR_OUT_OF_RANGE
    );
}
