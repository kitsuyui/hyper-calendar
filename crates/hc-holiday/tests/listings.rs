//! The tables of announced dates are data, so a typo in one is a silent
//! miss: an entry whose name no row carries yields nothing, and a row no
//! entry reads is never seen. These tests hold every [`Listing`] and every
//! [`Rule::Listed`] that reads one to each other.

#![cfg(feature = "alloc")]

use std::collections::BTreeMap;

use hc_holiday::rule::{ListedEntry, Listing, ListingKey, Rule, RuleSet};

/// Every rule set the crate carries.
fn rule_sets() -> Vec<&'static RuleSet> {
    let mut sets = Vec::new();
    sets.extend(hc_holiday::countries::ALL.iter().copied());
    sets.extend(hc_holiday::exchanges::ALL.iter().copied());
    sets.extend(hc_holiday::traditions::ALL.iter().copied());
    sets.extend(hc_holiday::international::ALL.iter().copied());
    sets
}

/// The listed entries a rule reads, through the rules it is built on.
fn listed_in(rule: &Rule, out: &mut Vec<(ListedEntry, i64, i64)>) {
    match rule {
        Rule::Listed {
            entry,
            first_year,
            last_year,
        } => out.push((*entry, i64::from(*first_year), i64::from(*last_year))),
        Rule::Offset { base, .. } | Rule::MovedByWeekday { base, .. } => listed_in(base, out),
        Rule::Span { from, to } => {
            listed_in(from, out);
            listed_in(to, out);
        }
        _ => {}
    }
}

/// An entry reading a table: its key, and the first and last year.
type Reader = (ListingKey, i64, i64);

/// A table's identity: its address.
fn table_id(listing: &'static Listing) -> usize {
    core::ptr::from_ref(listing) as usize
}

/// A row's key and year.
fn rows(listing: &Listing) -> Vec<(ListingKey, i64)> {
    match listing {
        Listing::Dates(rows) => rows.iter().map(|r| (ListingKey::Every, r.0)).collect(),
        Listing::Named(rows) => rows.iter().map(|r| (ListingKey::Name(r.3), r.0)).collect(),
        Listing::Numbered(rows) => rows
            .iter()
            .map(|r| (ListingKey::Number(r.3), r.0))
            .collect(),
        Listing::Spans(rows) => rows.iter().map(|r| (ListingKey::Name(r.5), r.0)).collect(),
    }
}

fn key_text(key: ListingKey) -> String {
    match key {
        ListingKey::Every => "every row".to_owned(),
        ListingKey::Name(name) => format!("{name:?}"),
        ListingKey::Number(number) => format!("#{number}"),
    }
}

/// See [`every_listed_entry_reads_rows_its_table_has_and_every_row_is_read`].
const EMPTY_ON_PURPOSE: &[&str] = &["VN \"tet\""];

#[test]
fn every_listed_entry_reads_rows_its_table_has_and_every_row_is_read() {
    // table -> (the table, key -> the years the entries reading it cover)
    let mut read: BTreeMap<usize, (&'static Listing, Vec<Reader>)> = BTreeMap::new();
    let mut entries = 0;
    let mut without_rows = Vec::new();
    for set in rule_sets() {
        for holiday in set.rules {
            let mut found = Vec::new();
            listed_in(&holiday.rule, &mut found);
            for (entry, first, last) in found {
                entries += 1;
                assert!(
                    entry.listing.accepts(entry.key),
                    "{}: {} reads its table with {}, a key of the wrong kind",
                    set.code,
                    holiday.name,
                    key_text(entry.key)
                );
                if !matches!(entry.listing, Listing::Dates(rows) if rows.is_empty())
                    && !rows(entry.listing).iter().any(|(key, _)| *key == entry.key)
                {
                    without_rows.push(format!("{} {}", set.code, key_text(entry.key)));
                }
                read.entry(table_id(entry.listing))
                    .or_insert((entry.listing, Vec::new()))
                    .1
                    .push((entry.key, first, last));
            }
        }
    }
    assert!(entries > 200, "only {entries} listed entries");
    // The entries whose tables carry no row for them: the sources were
    // read and gave the day nothing in those years, as Vietnam's notices
    // worked no Saturday for Tết. Any other is a misspelt key.
    without_rows.sort();
    assert_eq!(without_rows, EMPTY_ON_PURPOSE, "{without_rows:#?}");
    for (listing, readers) in read.values() {
        for (key, year) in rows(listing) {
            assert!(
                readers
                    .iter()
                    .any(|&(k, first, last)| k == key && (first..=last).contains(&year)),
                "a row of {} in {year} is read by no entry that covers its year",
                key_text(key)
            );
        }
    }
}

#[test]
fn a_row_of_each_shape_is_read_as_its_date() {
    static SPANS: Listing = Listing::Spans(&[(2024, 2, 28, 3, 2, "a"), (2024, 5, 1, 5, 1, "b")]);
    static NAMED: Listing =
        Listing::Named(&[(2024, 2, 30, "a"), (2024, 3, 1, "a"), (2025, 3, 1, "a")]);
    let day = |month, day| hc_calendar::gregorian::to_fixed_saturating(2024, month, day);
    assert_eq!(
        SPANS.days_in(2024, ListingKey::Name("a")).as_slice(),
        [day(2, 28), day(2, 29), day(3, 1), day(3, 2)]
    );
    // A row whose date does not exist is skipped, and a row of another
    // year is not read.
    assert_eq!(
        NAMED.days_in(2024, ListingKey::Name("a")).as_slice(),
        [day(3, 1)]
    );
    // A key of the wrong kind reads nothing.
    assert!(NAMED.days_in(2024, ListingKey::Number(0)).is_empty());
    assert!(!NAMED.accepts(ListingKey::Every));
    // An entry is a gap outside its years.
    let rule = Rule::listed(NAMED.named("a"), 2024, 2024);
    assert!(rule.is_resolvable_in(2024));
    assert!(!rule.is_resolvable_in(2025));
    assert!(!Rule::UNREAD.is_resolvable_in(2024));
}
