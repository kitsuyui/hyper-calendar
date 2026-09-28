//! Every registered calendar converts its own first and last day: the day
//! reads as fields, and those fields convert back to the day. A calendar
//! whose conversion looks past its declared range to name a day inside it
//! — the next month's start, to name a dark fortnight; the next year's
//! era, to name the last day of this one — fails here rather than in a
//! reader of its dates.

use std::collections::BTreeSet;

#[test]
fn every_calendar_round_trips_its_first_and_last_day_through_fields() {
    let registry = hyper_calendar::registry();
    let mut ends = 0_usize;
    let mut faults: BTreeSet<String> = BTreeSet::new();
    for meta in registry.metas() {
        let calendar = registry.get(meta.id).expect("registered");
        for (end, day) in [("first", meta.earliest), ("last", meta.latest)] {
            let Some(day) = day else { continue };
            ends += 1;
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
    assert!(ends > 100, "{ends}");
}
