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
            2026,
            pointer,
            capacity,
        )
    };
    assert_eq!(written, needed);
    let text = unsafe { core::str::from_utf8(core::slice::from_raw_parts(pointer, capacity)) }
        .expect("UTF-8");
    assert!(
        text.starts_with("2026-01-01\tNew Year's Day\t元日\tpublic\texact\t0\t\t\t\n"),
        "{text}"
    );
    assert!(text.contains("\t1\t2026-05-03\t\t\n"), "{text}");
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
    assert!(rows.iter().all(|row| row.len() == 11), "{rows:?}");
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
    let gap = rows.iter().find(|row| row[4] == "gap").expect("a gap");
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
            ""
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
    // whole years give, each evaluated on its own.
    for (month, day) in [(1, 1), (2, 17), (3, 8), (6, 15), (9, 25), (11, 8)] {
        let fixed = hc_gregorian_to_fixed(2026, month, day);
        let text =
            read_lines(|buffer, capacity| unsafe { hc_holidays_on(fixed, buffer, capacity) });
        let mut expected = String::new();
        let day = hc::hc_holiday::Rd(fixed);
        for table in hc::holiday_lines::tables() {
            let year = hc::hc_holiday::HolidayCalendar::for_year(table, None, 2026);
            hc::holiday_lines::push_day_lines(&mut expected, table, &year, day);
            for region in table.regions() {
                let regional = hc::hc_holiday::HolidayCalendar::for_year(table, Some(region), 2026);
                hc::holiday_lines::push_region_day_lines(
                    &mut expected,
                    table,
                    region,
                    &regional,
                    &year,
                    day,
                );
            }
            for group in table.groups() {
                let scope = hc::hc_holiday::Scope::group(group.id);
                let grouped = hc::hc_holiday::HolidayCalendar::for_year_scoped(table, scope, 2026);
                hc::holiday_lines::push_scoped_day_lines(
                    &mut expected,
                    table,
                    scope,
                    &grouped,
                    &[&year],
                    day,
                );
            }
            for (region, group) in table.region_groups() {
                let scope = hc::hc_holiday::Scope::new(Some(region), Some(group.id));
                let both = hc::hc_holiday::HolidayCalendar::for_year_scoped(table, scope, 2026);
                let regional = hc::hc_holiday::HolidayCalendar::for_year_scoped(
                    table,
                    scope.for_everyone(),
                    2026,
                );
                let grouped = hc::hc_holiday::HolidayCalendar::for_year_scoped(
                    table,
                    scope.nationwide(),
                    2026,
                );
                hc::holiday_lines::push_scoped_day_lines(
                    &mut expected,
                    table,
                    scope,
                    &both,
                    &[&regional, &grouped],
                    day,
                );
            }
        }
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
            2026,
            buffer,
            capacity,
        )
    });
    let own: Vec<&str> = year
        .lines()
        .filter(|line| line.ends_with("\tJP-13\t"))
        .collect();
    assert_eq!(
        own,
        ["2026-10-01\tTokyo Citizens' Day\t都民の日\tschool\texact\t0\t\tJP-13\t"]
    );
    assert!(year.contains("2026-01-01\tNew Year's Day\t元日\tpublic\texact\t0\t\t\t\n"));
    let nationwide = read_lines(|buffer, capacity| unsafe {
        hc_holidays_in_year(
            jp.as_ptr(),
            2,
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
    assert!(tokyo_lines[0].ends_with("\t0\t\tJP-13\t"));
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
            2026,
            buffer,
            capacity,
        )
    });
    let own: Vec<&str> = year
        .lines()
        .filter(|line| line.ends_with("\twomen"))
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
    assert!(women_lines[0].ends_with("\t0\t\t\twomen"));
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
