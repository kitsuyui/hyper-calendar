use super::super::*;
use super::read_lines;

#[test]
fn holiday_tables_answer_by_identifier() {
    let jp = b"JP";
    let day = hc_gregorian_to_fixed(2026, 1, 1);
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                jp.as_ptr(),
                2,
                core::ptr::null(),
                0,
                core::ptr::null(),
                0,
                day,
            )
        },
        1
    );
    let xnys = b"XNYS";
    let good_friday = hc_gregorian_to_fixed(2026, 4, 3);
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                xnys.as_ptr(),
                4,
                core::ptr::null(),
                0,
                core::ptr::null(),
                0,
                good_friday,
            )
        },
        1
    );
    let us = b"US";
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                us.as_ptr(),
                2,
                core::ptr::null(),
                0,
                core::ptr::null(),
                0,
                good_friday,
            )
        },
        0
    );
    let zz = b"ZZ";
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                zz.as_ptr(),
                2,
                core::ptr::null(),
                0,
                core::ptr::null(),
                0,
                day,
            )
        },
        HC_ERR_UNKNOWN
    );
    let not_utf8 = [0xffu8];
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                not_utf8.as_ptr(),
                1,
                core::ptr::null(),
                0,
                core::ptr::null(),
                0,
                day,
            )
        },
        HC_ERR_NOT_UTF8
    );
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                jp.as_ptr(),
                2,
                not_utf8.as_ptr(),
                1,
                core::ptr::null(),
                0,
                day,
            )
        },
        HC_ERR_NOT_UTF8
    );
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                core::ptr::null(),
                2,
                core::ptr::null(),
                0,
                core::ptr::null(),
                0,
                day,
            )
        },
        HC_ERR_NULL_POINTER
    );
    assert_eq!(
        unsafe {
            hc_holidays_in_year(
                zz.as_ptr(),
                2,
                core::ptr::null(),
                0,
                core::ptr::null(),
                0,
                core::ptr::null(),
                0,
                2026,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
    // A null buffer measures; a sized one receives the lines.
    let needed = unsafe {
        hc_holidays_in_year(
            jp.as_ptr(),
            2,
            core::ptr::null(),
            0,
            core::ptr::null(),
            0,
            core::ptr::null(),
            0,
            2026,
            core::ptr::null_mut(),
            0,
        )
    };
    assert!(needed > 0);
    let capacity = needed as usize;
    let pointer = hc_alloc(capacity);
    let written = unsafe {
        hc_holidays_in_year(
            jp.as_ptr(),
            2,
            core::ptr::null(),
            0,
            core::ptr::null(),
            0,
            core::ptr::null(),
            0,
            2026,
            pointer,
            capacity,
        )
    };
    assert_eq!(written, needed);
    let text = unsafe { core::str::from_utf8(core::slice::from_raw_parts(pointer, capacity)) }
        .expect("UTF-8");
    assert!(
        text.starts_with(
            "2026-01-01\tNew Year's Day\t元日\tpublic\texact\t0\t\t\t\tnew-years-day\t\t0\t\t\n"
        ),
        "{text}"
    );
    assert!(
        text.contains("\t1\t2026-05-03\t\t\tconstitution-memorial-day\t\t0\t\t\n"),
        "{text}"
    );
    unsafe { hc_free(pointer, capacity) };
    let codes_len = unsafe { hc_holiday_codes(core::ptr::null_mut(), 0) };
    assert!(codes_len > 0);
    let capacity = codes_len as usize;
    let pointer = hc_alloc(capacity);
    assert_eq!(unsafe { hc_holiday_codes(pointer, capacity) }, codes_len);
    let text = unsafe { core::str::from_utf8(core::slice::from_raw_parts(pointer, capacity)) }
        .expect("UTF-8");
    assert!(text.contains("\nJP\n") && text.contains("\nXNYS\n") && text.contains("un-days\n"));
    unsafe { hc_free(pointer, capacity) };
}

#[test]
fn one_day_across_every_table_decodes_column_by_column() {
    // 2026-05-06 is Japan's substitute for Constitution Memorial Day
    // (3 May, a Sunday), and Greenery Day is 4 May.
    let day = hc_gregorian_to_fixed(2026, 5, 6);
    let text = read_lines(|buffer, capacity| unsafe { hc_holidays_on(day, buffer, capacity) });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert!(rows.iter().all(|row| row.len() == 13), "{rows:?}");
    let japan: Vec<&Vec<&str>> = rows.iter().filter(|row| row[0] == "JP").collect();
    let substitute = japan
        .iter()
        .find(|row| row[7] == "1")
        .unwrap_or_else(|| panic!("no substitute in {japan:?}"));
    assert_eq!(substitute[1], "Japan");
    assert_eq!(substitute[2], "Constitution Memorial Day");
    assert_eq!(substitute[3], "憲法記念日");
    assert_eq!(substitute[4], "public");
    assert_eq!(substitute[5], "exact");
    assert_eq!(substitute[8], hc_gregorian_to_fixed(2026, 5, 3).to_string());
    // The tables come in the order `hc_holiday_codes` lists them:
    // an exchange's rows follow every country's.
    let codes: Vec<&str> = rows.iter().map(|row| row[0]).collect();
    let first_exchange = codes
        .iter()
        .position(|code| code.starts_with('X'))
        .expect("x");
    assert!(
        codes[..first_exchange].iter().all(|code| code.len() == 2),
        "{codes:?}"
    );
    // A gap on an ordinary day is a table whose announcement for
    // the year has not been read, and it is reported as such.
    let gap = rows
        .iter()
        .find(|row| row[4] == "gap" && row[6].is_empty())
        .expect("a gap");
    assert_eq!(gap[5..11], ["", "", "0", "", "", ""], "{gap:?}");
}

#[test]
fn a_rule_that_cites_its_instrument_carries_it() {
    // World Braille Day, set by General Assembly resolution 73/161.
    let day = hc_gregorian_to_fixed(2026, 1, 4);
    let text = read_lines(|buffer, capacity| unsafe { hc_holidays_on(day, buffer, capacity) });
    let braille = text
        .lines()
        .map(|line| line.split('\t').collect::<Vec<&str>>())
        .find(|row| row[2] == "World Braille Day")
        .expect("the UN days are a table");
    assert_eq!(
        braille,
        [
            "un-days",
            "United Nations international days",
            "World Braille Day",
            "",
            "observance",
            "exact",
            "A/RES/73/161",
            "0",
            "",
            "",
            "",
            "world-braille-day",
            "0"
        ]
    );
}

#[test]
fn a_year_a_table_cannot_answer_is_reported_as_gaps() {
    // 2150 is past the Chinese calendar's range, so the tables dated
    // in it report the lunisolar holidays as gaps rather than
    // showing an empty day.
    let day = hc_gregorian_to_fixed(2150, 2, 1);
    let text = read_lines(|buffer, capacity| unsafe { hc_holidays_on(day, buffer, capacity) });
    let gaps: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect::<Vec<&str>>())
        .filter(|row| row[4] == "gap")
        .collect();
    let chinese = gaps
        .iter()
        .find(|row| row[0] == "CN" && row[2] == "Spring Festival")
        .unwrap_or_else(|| panic!("no Chinese gap in {gaps:?}"));
    assert_eq!(
        chinese[..9],
        [
            "CN",
            "China",
            "Spring Festival",
            "春节",
            "gap",
            "",
            "",
            "0",
            ""
        ]
    );
}

#[test]
fn a_day_with_no_year_is_refused() {
    assert_eq!(
        unsafe { hc_holidays_on(i64::MAX, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

#[test]
fn one_day_across_every_table_is_what_the_years_say() {
    // The call evaluates each table for the day alone, through one
    // memo for all of them; the lines must be the ones the tables'
    // whole years give, each evaluated on its own. A year's table does not
    // depend on the day, so each is evaluated once for all the days.
    let days = [(1, 1), (2, 17), (3, 8), (6, 15), (9, 25), (11, 8)];
    let days: Vec<(u32, u32, i64)> = days
        .into_iter()
        .map(|(month, day)| (month, day, hc_gregorian_to_fixed(2026, month, day)))
        .collect();
    let mut expected = vec![String::new(); days.len()];
    for table in hc::holiday_lines::tables() {
        let year = hc::hc_holiday::HolidayCalendar::for_year(table, None, 2026);
        for (text, (_, _, fixed)) in expected.iter_mut().zip(&days) {
            let day = hc::hc_holiday::Rd(*fixed);
            hc::holiday_lines::push_day_lines(text, table, &year, day);
        }
        for region in table.answered_regions() {
            let regional = hc::hc_holiday::HolidayCalendar::for_year(table, Some(region), 2026);
            for (text, (_, _, fixed)) in expected.iter_mut().zip(&days) {
                let day = hc::hc_holiday::Rd(*fixed);
                hc::holiday_lines::push_region_day_lines(
                    text, table, region, &regional, &year, day,
                );
            }
        }
        for group in table.groups() {
            let scope = hc::hc_holiday::Scope::group(group.id);
            let grouped = hc::hc_holiday::HolidayCalendar::for_year_scoped(table, scope, 2026);
            for (text, (_, _, fixed)) in expected.iter_mut().zip(&days) {
                let day = hc::hc_holiday::Rd(*fixed);
                hc::holiday_lines::push_scoped_day_lines(
                    text,
                    table,
                    scope,
                    &grouped,
                    &[&year],
                    day,
                );
            }
        }
        for (region, group) in table.region_groups() {
            let scope = hc::hc_holiday::Scope::new(Some(region), Some(group.id));
            let both = hc::hc_holiday::HolidayCalendar::for_year_scoped(table, scope, 2026);
            let regional =
                hc::hc_holiday::HolidayCalendar::for_year_scoped(table, scope.for_everyone(), 2026);
            let grouped =
                hc::hc_holiday::HolidayCalendar::for_year_scoped(table, scope.nationwide(), 2026);
            for (text, (_, _, fixed)) in expected.iter_mut().zip(&days) {
                let day = hc::hc_holiday::Rd(*fixed);
                hc::holiday_lines::push_scoped_day_lines(
                    text,
                    table,
                    scope,
                    &both,
                    &[&regional, &grouped],
                    day,
                );
            }
        }
    }
    for (expected, (month, day, fixed)) in expected.into_iter().zip(days) {
        let text =
            read_lines(|buffer, capacity| unsafe { hc_holidays_on(fixed, buffer, capacity) });
        assert_eq!(text, expected, "2026-{month:02}-{day:02}");
    }
}

/// A measurement, not a gate: one day across every table, as a page
/// asks for it. Run with `cargo test --release -p hyper-calendar-wasm
/// --features holiday -- --nocapture` to read the times; the
/// README's figures are the `release-compact` profile's.
#[test]
fn one_day_across_every_table_is_timed() {
    let tables = hc::holiday_lines::holiday_codes().lines().count();
    for (month, day) in [(1, 1), (2, 17), (9, 25)] {
        let fixed = hc_gregorian_to_fixed(2026, month, day);
        let start = std::time::Instant::now();
        let needed = unsafe { hc_holidays_on(fixed, core::ptr::null_mut(), 0) };
        let elapsed = start.elapsed();
        assert!(needed > 0);
        eprintln!(
            "hc_holidays_on for 2026-{month:02}-{day:02} across {tables} tables: {elapsed:?}"
        );
    }
}

#[test]
fn a_subdivision_is_a_region_of_its_country_s_table() {
    // Tokyo's 都民の日 on 1 October: a line of `JP` in the region
    // `JP-13`, in the year and on the day, and nowhere in the nationwide
    // year. `JP-13` is not a table.
    let jp = b"JP";
    let tokyo = b"jp-13";
    let year = read_lines(|buffer, capacity| unsafe {
        hc_holidays_in_year(
            jp.as_ptr(),
            2,
            tokyo.as_ptr(),
            5,
            core::ptr::null(),
            0,
            core::ptr::null(),
            0,
            2026,
            buffer,
            capacity,
        )
    });
    // The nine cells every line had before the identifier and the source.
    let own: Vec<String> = year
        .lines()
        .filter(|line| line.split('\t').nth(7) == Some("JP-13"))
        .map(|line| line.split('\t').take(9).collect::<Vec<_>>().join("\t"))
        .collect();
    assert_eq!(
        own,
        [
            "2026-03-10\tTokyo Peace Day\t東京都平和の日\tobservance\texact\t0\t\tJP-13\t",
            "2026-10-01\tTokyo Citizens' Day\t都民の日\tschool\texact\t0\t\tJP-13\t",
            "2026-11-07\tTokyo Education Day\t東京都教育の日\tobservance\texact\t0\t\tJP-13\t",
        ]
    );
    assert!(year.contains(
        "2026-01-01\tNew Year's Day\t元日\tpublic\texact\t0\t\t\t\tnew-years-day\t\t0\t\t\n"
    ));
    let nationwide = read_lines(|buffer, capacity| unsafe {
        hc_holidays_in_year(
            jp.as_ptr(),
            2,
            core::ptr::null(),
            0,
            core::ptr::null(),
            0,
            core::ptr::null(),
            0,
            2026,
            buffer,
            capacity,
        )
    });
    assert!(!nationwide.contains("都民の日"), "{nationwide}");
    let day = hc_gregorian_to_fixed(2026, 10, 1);
    let lines = read_lines(|buffer, capacity| unsafe { hc_holidays_on(day, buffer, capacity) });
    let tokyo_lines: Vec<&str> = lines
        .lines()
        .filter(|line| line.contains("都民の日"))
        .collect();
    assert_eq!(tokyo_lines.len(), 1, "{lines}");
    assert!(tokyo_lines[0].starts_with("JP\tJapan\tTokyo Citizens' Day\t"));
    assert!(tokyo_lines[0].ends_with("\t0\t\tJP-13\t\ttokyo-citizens-day\t0"));
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(jp.as_ptr(), 2, tokyo.as_ptr(), 5, core::ptr::null(), 0, day)
        },
        0
    );
    let codes = read_lines(|buffer, capacity| unsafe { hc_holiday_codes(buffer, capacity) });
    assert!(!codes.lines().any(|code| code == "JP-13"));
    let tables = read_lines(|buffer, capacity| unsafe {
        hc_holiday_tables("en".as_ptr(), 2, buffer, capacity)
    });
    let japan: Vec<&str> = tables
        .lines()
        .find(|line| line.starts_with("JP\t"))
        .expect("Japan")
        .split('\t')
        .collect();
    assert!(japan[8].split(';').any(|code| code == "JP-13"), "{japan:?}");
    assert!(
        !japan[8].split(';').any(|code| code == "JP-14"),
        "{japan:?}"
    );
}

#[test]
fn a_group_is_a_scope_of_its_country_s_table() {
    // China's half day for women on 8 March: a line of `CN` with the group
    // `women`, in the year asked for the group and on the day, and nowhere
    // in everyone's year.
    let cn = b"CN";
    let women = b" Women ";
    let year = read_lines(|buffer, capacity| unsafe {
        hc_holidays_in_year(
            cn.as_ptr(),
            2,
            core::ptr::null(),
            0,
            women.as_ptr(),
            women.len(),
            core::ptr::null(),
            0,
            2026,
            buffer,
            capacity,
        )
    });
    let own: Vec<String> = year
        .lines()
        .filter(|line| line.split('\t').nth(8) == Some("women"))
        .map(|line| line.split('\t').take(9).collect::<Vec<_>>().join("\t"))
        .collect();
    assert_eq!(
        own,
        ["2026-03-08\tWomen's Day\t妇女节\thalf-day\texact\t0\t\t\twomen"]
    );
    let everyone = read_lines(|buffer, capacity| unsafe {
        hc_holidays_in_year(
            cn.as_ptr(),
            2,
            core::ptr::null(),
            0,
            core::ptr::null(),
            0,
            core::ptr::null(),
            0,
            2026,
            buffer,
            capacity,
        )
    });
    assert!(!everyone.contains("妇女节"), "{everyone}");
    let day = hc_gregorian_to_fixed(2026, 3, 8);
    let lines = read_lines(|buffer, capacity| unsafe { hc_holidays_on(day, buffer, capacity) });
    let women_lines: Vec<&str> = lines
        .lines()
        .filter(|line| line.contains("妇女节"))
        .collect();
    assert_eq!(women_lines.len(), 1, "{lines}");
    assert!(women_lines[0].starts_with("CN\tChina\tWomen's Day\t"));
    assert!(women_lines[0].contains("\t0\t\t\twomen"));
    // Children's Day is a whole day off for children alone.
    let children = b"children";
    let june_1 = hc_gregorian_to_fixed(2026, 6, 1);
    let off = |group: &[u8]| unsafe {
        hc_holiday_is_day_off(
            cn.as_ptr(),
            2,
            core::ptr::null(),
            0,
            group.as_ptr(),
            group.len(),
            june_1,
        )
    };
    assert_eq!(off(children), 1);
    assert_eq!(off(b""), 0);
    // A known group with no day of its own in the table has everyone's
    // days; a name that is no group is refused.
    assert_eq!(off(b"police"), 0);
    assert_eq!(off(b"childrens"), HC_ERR_UNKNOWN);
    let unknown = b"childrens";
    assert_eq!(
        unsafe {
            hc_holidays_in_year(
                cn.as_ptr(),
                2,
                core::ptr::null(),
                0,
                unknown.as_ptr(),
                unknown.len(),
                core::ptr::null(),
                0,
                2026,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
    let tables = read_lines(|buffer, capacity| unsafe {
        hc_holiday_tables("zh-CN".as_ptr(), 5, buffer, capacity)
    });
    let china: Vec<&str> = tables
        .lines()
        .find(|line| line.starts_with("CN\t"))
        .expect("China")
        .split('\t')
        .collect();
    assert_eq!(china[9], "children;military;women;youth");
    assert_eq!(china[10], "少年儿童;现役军人;妇女;青年");
}

/// A day the table cannot answer is refused, not answered "no": a year past
/// China's lunisolar range is a gap, Canada's weekend of 2025 is read from
/// 2026, and Kedah's weekend law of 2012 was not read.
#[test]
fn a_day_a_gap_leaves_open_is_refused_and_a_known_one_is_answered() {
    let day_off = |code: &str, region: &str, fixed: i64| unsafe {
        hc_holiday_is_day_off(
            code.as_ptr(),
            code.len(),
            region.as_ptr(),
            region.len(),
            core::ptr::null(),
            0,
            fixed,
        )
    };
    let victoria = hc_gregorian_to_fixed(2025, 5, 19);
    assert_eq!(day_off("CA", "CA-NL", victoria), HC_ERR_OUT_OF_RANGE);
    // China's festivals are dated in the lunisolar calendar to 2150: a day
    // of 2151 that is no fixed holiday might be one of them.
    assert_eq!(
        day_off("CN", "", hc_gregorian_to_fixed(2151, 3, 4)),
        HC_ERR_NO_DATA
    );
    assert_eq!(
        day_off("CA", "CA-NL", hc_gregorian_to_fixed(2025, 12, 25)),
        1
    );
    assert_eq!(day_off("JP", "", hc_gregorian_to_fixed(2026, 3, 4)), 0);
    assert_eq!(
        day_off("MY", "MY-02", hc_gregorian_to_fixed(2012, 5, 11)),
        HC_ERR_OUT_OF_RANGE
    );
    // Kedah's own days were not read, so the day is open too, though its
    // weekend law of 2026 was.
    assert_eq!(
        day_off("MY", "MY-02", hc_gregorian_to_fixed(2026, 3, 4)),
        HC_ERR_NO_DATA
    );
}

/// Kedah keeps Friday and Saturday from 25 November 2013 (ADR 0015); the
/// law before it was not read.
#[test]
fn the_weekend_of_a_region_is_one_or_zero_and_refused_where_unread() {
    let weekend = |code: &str, region: &str, fixed: i64| unsafe {
        hc_holiday_is_weekend(
            code.as_ptr(),
            code.len(),
            region.as_ptr(),
            region.len(),
            fixed,
        )
    };
    let friday = hc_gregorian_to_fixed(2026, 3, 6);
    assert_eq!(weekend("MY", "MY-02", friday), 1);
    assert_eq!(weekend("MY", "", friday), 0);
    assert_eq!(weekend("MY", "", friday + 1), 1);
    assert_eq!(weekend("MY", "MY-02", friday + 2), 0);
    assert_eq!(weekend("AE", "AE-SH", friday), 1);
    assert_eq!(weekend("JP", "", friday + 1), 1);
    assert_eq!(
        weekend("MY", "MY-02", hc_gregorian_to_fixed(2013, 11, 24)),
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        weekend("MY", "MY-02", hc_gregorian_to_fixed(2013, 11, 29)),
        1
    );
    assert_eq!(weekend("ZZ", "", friday), HC_ERR_UNKNOWN);
    assert_eq!(weekend("MY", "MY-99", friday), HC_ERR_UNKNOWN);
    // Column 9 of `hc_holiday_tables` lists the regions with only a weekend
    // law of their own, and the export accepts each of them.
    let tables = read_lines(|buffer, capacity| unsafe {
        hc_holiday_tables("en".as_ptr(), 2, buffer, capacity)
    });
    let regions = |code: &str| -> Vec<String> {
        let row: Vec<&str> = tables
            .lines()
            .find(|line| line.split('\t').next() == Some(code))
            .expect("a table")
            .split('\t')
            .collect();
        row[8]
            .split(';')
            .filter(|region| !region.is_empty())
            .map(String::from)
            .collect()
    };
    assert_eq!(regions("MY"), ["MY-01", "MY-02", "MY-03", "MY-09", "MY-11"]);
    assert_eq!(regions("AE"), ["AE-SH"]);
    for code in ["MY", "AE"] {
        for region in regions(code) {
            assert_ne!(weekend(code, &region, friday), HC_ERR_UNKNOWN, "{region}");
        }
    }
    assert_eq!(weekend("MY", "", 1 << 62), HC_ERR_OUT_OF_RANGE);
    let not_utf8 = [0xffu8];
    assert_eq!(
        unsafe { hc_holiday_is_weekend(not_utf8.as_ptr(), 1, core::ptr::null(), 0, friday) },
        HC_ERR_NOT_UTF8
    );
    assert_eq!(
        unsafe { hc_holiday_is_weekend(core::ptr::null(), 2, core::ptr::null(), 0, friday) },
        HC_ERR_NULL_POINTER
    );
}

/// `hc_holiday_next` or `hc_holiday_previous` for a table, a region and a
/// kind, with no group.
fn beyond(
    forward: bool,
    (code, region, kind): (&str, &str, &str),
    fixed: i64,
    buffer: *mut u8,
    capacity: usize,
) -> i64 {
    unsafe {
        if forward {
            hc_holiday_next(
                code.as_ptr(),
                code.len(),
                region.as_ptr(),
                region.len(),
                core::ptr::null(),
                0,
                kind.as_ptr(),
                kind.len(),
                fixed,
                buffer,
                capacity,
            )
        } else {
            hc_holiday_previous(
                code.as_ptr(),
                code.len(),
                region.as_ptr(),
                region.len(),
                core::ptr::null(),
                0,
                kind.as_ptr(),
                kind.len(),
                fixed,
                buffer,
                capacity,
            )
        }
    }
}

/// The next holiday of Japan after Tuesday 28 April 2026 is Shōwa Day, the
/// 29th; the last before 7 May is the substitute for 3 May; and a gap that
/// could hide a nearer one is refused.
#[test]
fn the_next_and_the_previous_holiday_are_a_line_of_the_year() {
    let jp = ("JP", "", "");
    let line = |forward: bool, scope: (&str, &str, &str), fixed: i64| {
        read_lines(|buffer, capacity| beyond(forward, scope, fixed, buffer, capacity))
    };
    let next = line(true, jp, hc_gregorian_to_fixed(2026, 4, 28));
    let row: Vec<&str> = next.trim_end().split('\t').collect();
    assert_eq!(row.len(), 12, "{next}");
    assert_eq!(
        row[..5],
        ["2026-04-29", "Shōwa Day", "昭和の日", "public", "exact"]
    );
    assert_eq!(row[9], "showa-day");
    let previous = line(false, jp, hc_gregorian_to_fixed(2026, 5, 7));
    let row: Vec<&str> = previous.trim_end().split('\t').collect();
    assert_eq!(row[0], "2026-05-06");
    assert_eq!(row[5], "1", "a substitute day");
    assert_eq!(row[6], "2026-05-03");
    // The bridge of 22 September 2009 says so in the last cell.
    let bridge = line(true, jp, hc_gregorian_to_fixed(2009, 9, 21));
    let row: Vec<&str> = bridge.trim_end().split('\t').collect();
    assert_eq!(row[..2], ["2009-09-22", "Citizens' Holiday"]);
    assert_eq!(row[11], "1");
    // A gap could hide a nearer entry, and a table with none of the kind
    // has none to give.
    let refuse = |forward: bool, scope: (&str, &str, &str), fixed: i64| {
        beyond(forward, scope, fixed, core::ptr::null_mut(), 0)
    };
    let nl = ("CA", "CA-NL", "");
    assert_eq!(
        refuse(true, nl, hc_gregorian_to_fixed(2025, 5, 1)),
        HC_ERR_NO_DATA
    );
    assert_eq!(
        refuse(false, nl, hc_gregorian_to_fixed(2025, 7, 15)),
        HC_ERR_NO_DATA
    );
    let new_year = hc_gregorian_to_fixed(2026, 1, 1);
    assert_eq!(refuse(true, ("un-days", "", ""), new_year), HC_ERR_NO_DATA);
    assert_eq!(refuse(true, ("ZZ", "", ""), new_year), HC_ERR_UNKNOWN);
    assert_eq!(
        refuse(true, ("JP", "", "festival"), new_year),
        HC_ERR_UNKNOWN
    );
    assert_eq!(refuse(true, jp, 1 << 62), HC_ERR_OUT_OF_RANGE);
}

/// A region that has only a weekend law has substitute days of its own:
/// Awal Muharram 2025 was Friday 27 June, which Kedah moves to Sunday the
/// 29th.
#[test]
fn a_regions_substitute_day_is_a_line_of_the_day_it_falls_on() {
    let sunday = hc_gregorian_to_fixed(2025, 6, 29);
    let text = read_lines(|buffer, capacity| unsafe { hc_holidays_on(sunday, buffer, capacity) });
    let kedah: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect::<Vec<_>>())
        .filter(|row| row[0] == "MY" && row[9] == "MY-02")
        .collect();
    assert!(
        kedah.iter().any(|row| row[7] == "1"
            && row[8] == hc_gregorian_to_fixed(2025, 6, 27).to_string()
            && row[4] == "public"),
        "{kedah:?}"
    );
    // The bridge flag is the last of the thirteen cells.
    assert!(text.lines().all(|line| line.split('\t').count() == 13));
}
