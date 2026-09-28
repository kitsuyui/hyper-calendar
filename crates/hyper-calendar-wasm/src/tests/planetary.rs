use super::super::*;
use super::read_lines;

fn cells(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .map(|line| line.split('\t').map(str::to_owned).collect())
        .collect()
}

/// Gangale's Titan calibration: 2002-12-18 10:42 UTC, POSIX
/// 1 040 208 120, was 209 Aries 13, Julian Circad 144 096, Solis.
#[test]
fn a_circad_date_crosses_the_boundary() {
    let id = "darian-titan";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_circad_date(id.as_ptr(), id.len(), 1_040_208_120.0, buffer, capacity)
    });
    let rows = cells(&text);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].len(), hc::planetary_lines::CIRCAD_DATE_COLUMNS);
    assert_eq!(
        rows[0][..7],
        ["darian-titan", "209", "9", "13", "Aries", "Solis", "144096"]
    );
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_circad_date("titan".as_ptr(), 5, 0.0, null, 0) },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_circad_date("martiana".as_ptr(), 8, 1e10, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

fn number(cell: &str) -> f64 {
    cell.parse()
        .unwrap_or_else(|_| panic!("not a number: {cell}"))
}

/// Mars24's worked example A: 2000-01-06T00:00:00Z at the prime
/// meridian is MSD 44795.99976, MTC 23:59:39, Ls 277.18758°, EOT
/// −5.18774° and LTST 23:38:54.
#[test]
fn the_first_mars24_worked_example_decodes_column_by_column() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_mars_time(947_116_800.0, 0.0, buffer, capacity)
    });
    let rows = cells(&text);
    assert_eq!(rows.len(), 1);
    let row = &rows[0];
    assert_eq!(row.len(), 16, "{row:?}");
    assert!((number(&row[0]) - 44_795.999_760_4).abs() < 1e-6, "{row:?}");
    assert_eq!(row[1], "23:59:39");
    assert_eq!(row[3], "23:59:39");
    assert_eq!(row[5], "23:38:54");
    assert!((number(&row[7]) - -5.187_74 * 4.0).abs() < 1e-3, "{row:?}");
    assert!((number(&row[8]) - 277.187_58).abs() < 1e-5, "{row:?}");
    assert_eq!(row[9], "24");
    assert_eq!(row[10], "207");
    assert!(row[15].contains("Mars24"), "{}", row[15]);
    assert!(
        row.iter()
            .all(|cell| !cell.contains('e') || cell.contains(' '))
    );
}

#[test]
fn a_mission_sol_is_the_missions_own_and_an_unpublished_one_is_refused() {
    let curiosity = "curiosity";
    let sol = |name: &str, unix: f64| unsafe { hc_mission_sol(name.as_ptr(), name.len(), unix) };
    assert_eq!(sol(curiosity, 1_344_230_277.0), 0);
    assert_eq!(sol("mars-pathfinder", 868_035_415.0), 1);
    assert_eq!(sol("Mars Pathfinder", 868_035_415.0), HC_ERR_UNKNOWN);
    assert_eq!(sol("zhurong", 1_700_000_000.0), HC_ERR_NO_DATA);
    assert_eq!(sol("beagle-2", 1_700_000_000.0), HC_ERR_UNKNOWN);
    assert_eq!(
        sol(curiosity, 1_344_230_277.0 - 86_400.0),
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(sol(curiosity, 5e9), HC_ERR_OUT_OF_RANGE);
    assert_eq!(
        unsafe { hc_mission_sol(core::ptr::null(), 3, 0.0) },
        HC_ERR_NULL_POINTER
    );
    let listed = cells(&read_lines(|buffer, capacity| unsafe {
        hc_missions(buffer, capacity)
    }));
    assert_eq!(listed.len(), 10);
    assert!(listed.iter().all(|row| row.len() == 11), "{listed:?}");
    let zhurong = listed.iter().find(|row| row[0] == "zhurong");
    assert_eq!(
        zhurong.map(|row| &row[4..7]),
        Some(&[String::new(), String::new(), String::new()][..])
    );
}

#[test]
fn every_body_lists_and_titans_clock_reads() {
    let bodies = cells(&read_lines(|buffer, capacity| unsafe {
        hc_bodies(buffer, capacity)
    }));
    assert_eq!(bodies.len(), 22);
    assert!(bodies.iter().all(|row| row.len() == 12), "{bodies:?}");
    let titan = "titan";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_body_time(
            titan.as_ptr(),
            titan.len(),
            947_116_800.0,
            0.0,
            buffer,
            capacity,
        )
    });
    let row = &cells(&text)[0];
    assert_eq!(row.len(), 8, "{row:?}");
    // A Titan hour is 15.97 Earth hours.
    assert!((number(&row[5]) / 3_600.0 - 15.97).abs() < 0.01, "{row:?}");
    assert_eq!(row[6], "convention");
    let sun = "Sun";
    assert_eq!(
        unsafe { hc_body_time(sun.as_ptr(), sun.len(), 0.0, 0.0, core::ptr::null_mut(), 0) },
        HC_ERR_NO_DATA
    );
}
