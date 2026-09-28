//! Every registered calendar converts its own first and last day: the day
//! reads as fields, and those fields convert back to the day. A calendar
//! whose conversion looks past its declared range to name a day inside it
//! — the next month's start, to name a dark fortnight; the next year's
//! era, to name the last day of this one — fails here rather than in a
//! reader of its dates.
//!
//! Every calendar has both ends but the cycles that count no years, which
//! have neither: `OPEN_ENDED` names them, so that a calendar that loses
//! its range fails here by name rather than dropping out of the sweep.

use std::collections::BTreeSet;

/// The calendars with no first or last day: cycles of named days or years
/// that repeat without an era, so that no day is outside them.
const OPEN_ENDED: &[&str] = &[
    "akan",
    "aztec-tonalpohualli",
    "aztec-xiuhpohualli",
    "balinese-pawukon",
    "javanese-pasaran",
    "maya-819",
    "maya-819-584286",
    "maya-819-gmt2",
    "maya-haab",
    "maya-haab-584286",
    "maya-haab-gmt2",
    "maya-tzolkin",
    "maya-tzolkin-584286",
    "maya-tzolkin-gmt2",
    "mixtec-year",
    "sexagenary",
    "zapotec-yza",
];

#[test]
fn every_calendar_round_trips_its_first_and_last_day_through_fields() {
    let registry = hyper_calendar::registry();
    let mut open: BTreeSet<&str> = BTreeSet::new();
    let mut faults: BTreeSet<String> = BTreeSet::new();
    for meta in registry.metas() {
        let calendar = registry.get(meta.id).expect("registered");
        for (end, day) in [("first", meta.earliest), ("last", meta.latest)] {
            let Some(day) = day else {
                open.insert(meta.id.0);
                if !OPEN_ENDED.contains(&meta.id.0) {
                    faults.insert(format!("{} has no {end} day", meta.id.0));
                }
                continue;
            };
            if OPEN_ENDED.contains(&meta.id.0) {
                faults.insert(format!("{} has a {end} day, {}", meta.id.0, day.0));
            }
            match calendar.fixed_to_fields(day) {
                Err(error) => {
                    faults.insert(format!("{} {end} day {}: {error:?}", meta.id.0, day.0));
                }
                Ok(fields) => match calendar.fields_to_fixed(&fields) {
                    Ok(back) if back == day => {}
                    back => {
                        faults.insert(format!(
                            "{} {end} day {}: {fields:?} gives {back:?}",
                            meta.id.0, day.0
                        ));
                    }
                },
            }
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
    // Each open-ended calendar is registered in the build that has them
    // all, so that one renamed or removed is not silently skipped.
    #[cfg(feature = "full")]
    assert_eq!(
        open,
        OPEN_ENDED.iter().copied().collect::<BTreeSet<_>>(),
        "the calendars without a first or last day"
    );
    assert!(open.len() < registry.metas().count(), "{open:?}");
}
