//! The Orthodox fasts, called as a page calls them.

use super::super::*;
use super::read_lines;

/// Great Lent began on 3 March 2025 (`oca-fasting-seasons`, as
/// `docs/systems/orthodox-fasts.md` works it).
#[test]
fn great_lent_2025_crosses_the_boundary() {
    let reckoning = "orthodox-fasts";
    let day = hc_gregorian_to_fixed(2025, 3, 3);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_orthodox_fast_on(reckoning.as_ptr(), reckoning.len(), day, buffer, capacity)
    });
    assert_eq!(
        text,
        "1\tperiod\tgreat-lent\tGreat Lent & Holy Week\tfast\tfast\n"
    );
    // Wednesday 18 February 2026, in Cheesefare week.
    let cheesefare = hc_gregorian_to_fixed(2026, 2, 18);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_orthodox_fast_on(
            reckoning.as_ptr(),
            reckoning.len(),
            cheesefare,
            buffer,
            capacity,
        )
    });
    assert_eq!(text, "0\tperiod\tmeatfast\tMeatfast\tmeat-excluded\tmeat\n");
    let text = read_lines(|buffer, capacity| unsafe {
        hc_orthodox_fast_seasons(reckoning.as_ptr(), reckoning.len(), 2025, buffer, capacity)
    });
    assert!(
        text.contains(&format!("\tfast\t{day}\t{}\n", day + 47)),
        "{text}"
    );
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_orthodox_fast_seasons(reckoning.as_ptr(), reckoning.len(), 5000, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

/// The Coptic Apostles' Fast of 2026, 1 June to 11 July, as the Coptic
/// Metropolis of the Southern United States dates it (`suscopts-fasts`).
#[test]
fn the_oriental_fasts_cross_the_same_boundary() {
    let reckoning = "coptic-fasts";
    let day = hc_gregorian_to_fixed(2026, 6, 1);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_orthodox_fast_on(reckoning.as_ptr(), reckoning.len(), day, buffer, capacity)
    });
    assert_eq!(
        text,
        "1\tperiod\tapostles-fast\tThe Apostles' Fast\tfast\tfast\n"
    );
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_orthodox_fast_seasons(reckoning.as_ptr(), reckoning.len(), 1582, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

/// Each Oriental reckoning through the boundary: the Armenian Great Lent
/// of 2026 from 16 February (`armenian-mother-see-calendar-2026`); the
/// Jerusalem Fast of the Catechumens from 2 February 2026, the day after the
/// Tonatsuyts's old-calendar eve (`surb-zoravor-tonatsuyts-2026`); Wednesday
/// 20 May 2026 a fast on the forty days and free on the fifty
/// (`arak29-fasts`); and the Nativity and Genna on 7 January 2024, 25
/// December Julian, with 8 January in no period (`st-takla-nativity-fast`,
/// `eotc-ma-calendar`).
#[test]
fn each_oriental_reckoning_crosses_the_boundary() {
    let line = |reckoning: &str, day: i64| {
        read_lines(|buffer, capacity| unsafe {
            hc_orthodox_fast_on(reckoning.as_ptr(), reckoning.len(), day, buffer, capacity)
        })
    };
    assert_eq!(
        line("armenian-fasts", hc_gregorian_to_fixed(2026, 2, 16)),
        "1\tperiod\tgreat-lent\tGreat Lent and Holy Week\tfast\tfast\n"
    );
    assert_eq!(
        line(
            "armenian-fasts-jerusalem",
            hc_gregorian_to_fixed(2026, 2, 2)
        ),
        "1\tperiod\tcatechumens-fast\tFast of the Catechumens (Aradjavorats)\tfast\tfast\n"
    );
    let may_20 = hc_gregorian_to_fixed(2026, 5, 20);
    assert_eq!(
        line("armenian-fasts", may_20),
        "1\tweekly-fast\t\t\t\tfast\n"
    );
    assert_eq!(
        line("armenian-fasts-fifty-days", may_20),
        "0\tperiod\teaster-to-pentecost\tThe fifty days after Easter\tfast-free\tnothing\n"
    );
    let january_7 = hc_gregorian_to_fixed(2024, 1, 7);
    assert_eq!(
        line("coptic-fasts", january_7),
        "0\tperiod\tnativity-feast\tThe Nativity\tfast-free\tnothing\n"
    );
    assert_eq!(
        line("ethiopian-fasts", january_7),
        "0\tperiod\tgenna\tGenna\tfast-free\tnothing\n"
    );
    assert_eq!(
        line("ethiopian-fasts", january_7 + 1),
        "0\tnone\t\t\t\tnothing\n"
    );
    let text = read_lines(|buffer, capacity| unsafe {
        hc_orthodox_fast_seasons(
            "armenian-fasts-fifty-days".as_ptr(),
            25,
            2026,
            buffer,
            capacity,
        )
    });
    assert_eq!(text.lines().count(), 13, "{text}");
}
