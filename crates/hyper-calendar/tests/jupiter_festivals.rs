//! Jupiter's position, its entries into the sidereal signs, and the
//! festivals those set, computed from the VSOP87B series for Jupiter and
//! held to what was published: Drik Panchang's table of Jupiter's entries
//! from 2001 to 2030, the Kumbh Mela years Wikipedia lists from 1974 to
//! 2028 and the dates of the Pushkaram festivals that were held.
//!
//! `docs/systems/jupiter-festivals.md` writes up what these find.

#![cfg(feature = "jupiter")]
#![expect(
    clippy::expect_used,
    reason = "a date or a line that does not read is a failed test, and the message says which"
)]

use hyper_calendar::hc_astro::riseset::Location;
use hyper_calendar::hc_calendar::Rd;
use hyper_calendar::hc_calendar::fixed::Moment;
use hyper_calendar::hc_calendars_indic::kumbh::{KumbhYoga, in_year};
use hyper_calendar::hc_calendars_solar::gregorian;
use hyper_calendar::hc_seasons::zodiac::jupiter;
use hyper_calendar::hc_seasons::zodiac::{Ayanamsa, SiderealSign};

/// Jupiter's sidereal ingresses, Lahiri, as Drik Panchang's "Guru Gochar"
/// pages for New Delhi print them for 2001 to 2030 (`drik-guru-gochar`,
/// retrieved 2026-09-28): the year, month, day, hour and minute IST, and
/// the sign entered, as its index from Meṣa. Jupiter was in Vṛṣabha before
/// the first row.
const DRIK_JUPITER: [(i64, u8, u8, u8, u8, u8); 49] = [
    (2001, 6, 16, 8, 38, 2),
    (2002, 7, 5, 13, 33, 3),
    (2003, 7, 30, 13, 4, 4),
    (2004, 8, 28, 0, 40, 5),
    (2005, 9, 28, 6, 27, 6),
    (2006, 10, 27, 23, 2, 7),
    (2007, 11, 22, 5, 36, 8),
    (2008, 12, 10, 0, 6, 9),
    (2009, 5, 1, 19, 13, 10),
    (2009, 7, 30, 19, 16, 9),
    (2009, 12, 20, 0, 35, 10),
    (2010, 5, 2, 8, 25, 11),
    (2010, 11, 1, 11, 56, 10),
    (2010, 12, 6, 10, 3, 11),
    (2011, 5, 8, 14, 26, 0),
    (2012, 5, 17, 9, 49, 1),
    (2013, 5, 31, 7, 10, 2),
    (2014, 6, 19, 9, 16, 3),
    (2015, 7, 14, 7, 7, 4),
    (2016, 8, 11, 22, 24, 5),
    (2017, 9, 12, 8, 0, 6),
    (2018, 10, 11, 20, 39, 7),
    (2019, 3, 30, 3, 9, 8),
    (2019, 4, 22, 17, 53, 7),
    (2019, 11, 5, 6, 41, 8),
    (2020, 3, 30, 5, 59, 9),
    (2020, 6, 30, 3, 6, 8),
    (2020, 11, 20, 14, 55, 9),
    (2021, 4, 6, 1, 50, 10),
    (2021, 9, 14, 11, 42, 9),
    (2021, 11, 21, 2, 5, 10),
    (2022, 4, 13, 16, 57, 11),
    (2023, 4, 22, 6, 12, 0),
    (2024, 5, 1, 13, 50, 1),
    (2025, 5, 14, 23, 20, 2),
    (2025, 10, 18, 21, 39, 3),
    (2025, 12, 5, 15, 38, 2),
    (2026, 6, 2, 2, 25, 3),
    (2026, 10, 31, 12, 50, 4),
    (2027, 1, 25, 0, 52, 3),
    (2027, 6, 26, 5, 43, 4),
    (2027, 11, 26, 19, 17, 5),
    (2028, 2, 28, 18, 50, 4),
    (2028, 7, 24, 15, 51, 5),
    (2028, 12, 26, 14, 1, 6),
    (2029, 3, 29, 14, 6, 5),
    (2029, 8, 25, 1, 13, 6),
    (2030, 1, 25, 2, 10, 7),
    (2030, 5, 1, 13, 48, 6),
];

/// A moment given in Indian Standard Time.
fn ist(year: i64, month: u8, day: u8, hour: u8, minute: u8) -> Moment {
    let day = gregorian::to_fixed(year, month, day).expect("a date");
    Moment(day.0 as f64 + (f64::from(hour) + f64::from(minute) / 60.0 - 5.5) / 24.0)
}

fn year_start(year: i64) -> Moment {
    Moment(hyper_calendar::hc_calendar::gregorian::new_year(year).0 as f64)
}

/// New Delhi, the place Drik Panchang's times are for.
const NEW_DELHI: Location = Location::new(28.6356, 77.2244, 0.0);

/// Drik Panchang's ayanāṃśa is this crate's Lahiri plus 25 arcseconds: the
/// sidereal longitude of Jupiter at each of the 49 entries is on the boundary
/// to within 23.9″ to 27.1″ of it, over thirty years, forward and in
/// retrograde, which a different nutation or precession would not give.
const DRIK_LAHIRI_SHIFT_ARCSECONDS: f64 = 25.0;

fn drik_lahiri() -> Ayanamsa {
    let lahiri = Ayanamsa::LAHIRI;
    Ayanamsa::new(
        "lahiri-drik",
        "Lahiri, shifted to Drik Panchang's",
        lahiri.anchor_julian_date(),
        lahiri.degrees_at_anchor() + DRIK_LAHIRI_SHIFT_ARCSECONDS / 3600.0,
    )
}

#[test]
fn the_sidereal_longitude_stands_a_constant_25_arcseconds_off_drik_panchangs() {
    let mut previous = SiderealSign::VRISHABHA;
    for row in DRIK_JUPITER {
        let (year, month, day, hour, minute, entered) = row;
        let sign = SiderealSign::from_index(entered).expect("a sign");
        // The boundary entered: the sign's start going forward, the end of
        // the sign going back into it.
        let boundary = if sign == previous.next() {
            sign.start_longitude_degrees()
        } else {
            sign.next().start_longitude_degrees()
        };
        let longitude =
            jupiter::sidereal_longitude(ist(year, month, day, hour, minute), Ayanamsa::LAHIRI);
        let off = ((longitude - boundary + 180.0).rem_euclid(360.0) - 180.0) * 3600.0;
        assert!((23.0..28.0).contains(&off), "{row:?}: {off}″");
        previous = sign;
    }
}

#[test]
fn the_ingresses_agree_with_drik_panchangs_within_seven_minutes_given_the_shift() {
    let found: Vec<_> =
        jupiter::ingresses(year_start(2001), year_start(2031), drik_lahiri()).collect();
    // Drik's table ends at 1 May 2030; one more entry follows that year.
    assert_eq!(found.len(), DRIK_JUPITER.len() + 1);
    let mut worst: f64 = 0.0;
    for (row, mine) in DRIK_JUPITER.iter().zip(&found) {
        let drik = ist(row.0, row.1, row.2, row.3, row.4);
        assert_eq!(mine.to.index(), row.5, "{row:?}");
        worst = worst.max((mine.moment.0 - drik.0).abs() * 1440.0);
    }
    assert!(worst < 7.5, "{worst} minutes");
    // Without the shift: forward entries 40 to 135 minutes before Drik's,
    // returns up to four and a half hours after, as the station is slow.
    let lahiri: Vec<_> =
        jupiter::ingresses(year_start(2001), year_start(2031), Ayanamsa::LAHIRI).collect();
    for (row, mine) in DRIK_JUPITER.iter().zip(&lahiri) {
        let drik = ist(row.0, row.1, row.2, row.3, row.4);
        let minutes = (mine.moment.0 - drik.0) * 1440.0;
        assert!(
            if mine.is_forward() {
                (-270.0..-40.0).contains(&minutes)
            } else {
                (40.0..270.0).contains(&minutes)
            },
            "{row:?}: {minutes} minutes"
        );
    }
}

/// Wikipedia's table of Kumbh Mela years (`wikipedia-kumbh-mela`, read
/// 2026-10-03) by site, the Kumbh and Maha Kumbh years of 1974 to 2028; the
/// Ardh Kumbh years are not carried.
const HARIDWAR: [i64; 5] = [1974, 1986, 1998, 2010, 2021];
const PRAYAG: [i64; 5] = [1977, 1989, 2001, 2013, 2025];
const NASHIK: [i64; 5] = [1980, 1992, 2003, 2015, 2027];
const UJJAIN: [i64; 5] = [1980, 1992, 2004, 2016, 2028];

/// The years from 1974 to 2030 in which a condition holds, Lahiri, with
/// Jupiter's sign computed.
fn held(id: &str) -> Vec<i64> {
    let yoga = KumbhYoga::by_id(id).expect("a condition");
    let lahiri = Ayanamsa::LAHIRI;
    (1974..=2030)
        .filter(|&year| {
            in_year(&yoga, year, lahiri, |moment| {
                jupiter::sign_at(moment, lahiri)
            })
            .is_some()
        })
        .collect()
}

#[test]
fn every_kumbh_year_of_the_table_meets_a_condition_of_its_site() {
    assert_eq!(held("kumbh-haridwar"), [1974, 1986, 1998, 2010, 2021]);
    // Prayag's first condition gives four of the five; 1977, which the table
    // lists, meets the second, in which Jupiter is in Meṣa; and that holds
    // in 2000, 2012 and 2024, when there was no festival.
    assert_eq!(held("kumbh-prayag-vrishabha"), [1989, 2001, 2013, 2025]);
    assert_eq!(held("kumbh-prayag-mesha"), [1977, 2000, 2012, 2024]);
    // Nashik's holds a year before the table's 1992 and a year after its
    // 2003, where the Sun's stay in Siṃha and Jupiter's overlap in two
    // Augusts running.
    assert_eq!(
        held("kumbh-nashik-simha"),
        [1980, 1991, 1992, 2003, 2004, 2015, 2027]
    );
    assert_eq!(held("kumbh-ujjain-simha"), UJJAIN);
    for (years, held) in [
        (HARIDWAR, held("kumbh-haridwar")),
        (NASHIK, held("kumbh-nashik-simha")),
        (UJJAIN, held("kumbh-ujjain-simha")),
    ] {
        assert!(
            years.iter().all(|year| held.contains(year)),
            "{years:?} {held:?}"
        );
    }
    let prayag: Vec<i64> = held("kumbh-prayag-vrishabha")
        .into_iter()
        .chain(held("kumbh-prayag-mesha"))
        .collect();
    assert!(PRAYAG.iter().all(|year| prayag.contains(year)));
    // The conditions that no festival followed.
    assert_eq!(
        held("kumbh-nashik-karka"),
        [1979, 1990, 1991, 2002, 2003, 2014, 2026]
    );
    assert_eq!(
        held("kumbh-ujjain-tula"),
        [1981, 1982, 1993, 1994, 2005, 2006, 2017, 2029]
    );
}

fn day(year: i64, month: u8, day: u8) -> Moment {
    Moment(gregorian::to_fixed(year, month, day).expect("a date").0 as f64)
}

#[test]
fn the_recent_and_announced_festivals_meet_their_conditions() {
    let lahiri = Ayanamsa::LAHIRI;
    let held = |id: &str, year: i64| {
        let yoga = KumbhYoga::by_id(id).expect("a condition");
        in_year(&yoga, year, lahiri, |moment| {
            jupiter::sign_at(moment, lahiri)
        })
    };
    // The Maha Kumbh of 13 January to 26 February 2025, `wikipedia-2025-prayag-maha-kumbh`:
    // the Sun enters Makara on 14 January with Jupiter in Vṛṣabha.
    let prayag = held("kumbh-prayag-vrishabha", 2025).expect("held in 2025");
    assert!(prayag.from.0 > day(2025, 1, 13).0 && prayag.from.0 < day(2025, 1, 15).0);
    assert_eq!(
        jupiter::sign_at(day(2025, 2, 12), lahiri),
        SiderealSign::VRISHABHA
    );
    // The Simhastha at Ujjain, "between April 9 and May 8, 2028"
    // (`theweek-simhastha-2028`): the Sun enters Meṣa inside it, with Jupiter
    // in Siṃha, where it stays from 28 February to 24 July 2028.
    let ujjain = held("kumbh-ujjain-simha", 2028).expect("held in 2028");
    assert!(ujjain.from.0 > day(2028, 4, 9).0 && ujjain.from.0 < day(2028, 5, 8).0);
    for (month, date) in [(4, 9), (5, 8)] {
        assert_eq!(
            jupiter::sign_at(day(2028, month, date), lahiri),
            SiderealSign::SIMHA
        );
    }
    // The Nashik Kumbh of 2027, whose baths the Chief Minister announced for
    // 29 July, 2 August, 31 August and 11 and 12 September 2027
    // (`indiatv-nashik-kumbh-2027`): Jupiter is in Siṃha from 26 June to
    // 26 November and the Sun enters Siṃha in the middle of August.
    let nashik = held("kumbh-nashik-simha", 2027).expect("held in 2027");
    assert!(nashik.from.0 > day(2027, 8, 15).0 && nashik.from.0 < day(2027, 8, 19).0);
    for (month, date) in [(7, 29), (8, 2), (8, 31), (9, 11), (9, 12)] {
        assert_eq!(
            jupiter::sign_at(day(2027, month, date), lahiri),
            SiderealSign::SIMHA
        );
    }
    // The condition is sidereal: the tropical sign of Jupiter at each of
    // these is another.
    for moment in [prayag.from, ujjain.from, nashik.from] {
        let tropical = hyper_calendar::hc_astro::jupiter::longitude(moment);
        let sidereal = jupiter::sidereal_longitude(moment, lahiri);
        assert_ne!((tropical / 30.0).floor(), (sidereal / 30.0).floor());
    }
}

/// The lines of `pushkaram_by_sky_lines` for a sign, as (river, first day,
/// last day, the entry's POSIX second), one per river of the sign.
fn festival(sign: &str, year: i64, rule: &str) -> Vec<(String, Rd, Rd, i64)> {
    use hyper_calendar::jupiter_lines::{PUSHKARAM_BY_SKY_COLUMNS, pushkaram_by_sky_lines};
    let text = pushkaram_by_sky_lines(sign, year, "lahiri", rule, NEW_DELHI, "india", "en")
        .expect("a line");
    text.lines()
        .map(|line| {
            let cells: Vec<&str> = line.split('\t').collect();
            assert_eq!(cells.len(), PUSHKARAM_BY_SKY_COLUMNS, "{line}");
            assert_eq!(cells[PUSHKARAM_BY_SKY_COLUMNS - 1], rule);
            (
                cells[0].to_owned(),
                Rd(cells[6].parse().expect("a day")),
                Rd(cells[7].parse().expect("a day")),
                cells[PUSHKARAM_BY_SKY_COLUMNS - 2]
                    .parse()
                    .expect("a second"),
            )
        })
        .collect()
}

fn rd(year: i64, month: u8, date: u8) -> Rd {
    gregorian::to_fixed(year, month, date).expect("a date")
}

/// The Pushkaram festivals whose dates were read, with Drik Panchang's entry
/// of Jupiter that began each: the river, the sign, the entry (year, month,
/// day, hour, minute IST) and the first and last day of the festival as
/// (year, month, day). The sources are in the system document.
const FESTIVALS: [(&str, &str, [i64; 5], [i64; 6]); 9] = [
    (
        "pushkaram-godavari",
        "simha",
        [2015, 7, 14, 7, 7],
        [2015, 7, 14, 2015, 7, 25],
    ),
    (
        "pushkaram-krishna",
        "kanya",
        [2016, 8, 11, 22, 24],
        [2016, 8, 12, 2016, 8, 23],
    ),
    (
        "pushkaram-kaveri",
        "tula",
        [2017, 9, 12, 8, 0],
        [2017, 9, 12, 2017, 9, 23],
    ),
    (
        "pushkaram-brahmaputra",
        "dhanus",
        [2019, 11, 5, 6, 41],
        [2019, 11, 5, 2019, 11, 16],
    ),
    (
        "pushkaram-tungabhadra",
        "makara",
        [2020, 11, 20, 14, 55],
        [2020, 11, 20, 2020, 12, 1],
    ),
    (
        "pushkaram-pranahita",
        "mina",
        [2022, 4, 13, 16, 57],
        [2022, 4, 13, 2022, 4, 24],
    ),
    (
        "pushkaram-ganga",
        "mesha",
        [2023, 4, 22, 6, 12],
        [2023, 4, 22, 2023, 5, 3],
    ),
    (
        "pushkaram-sarasvati",
        "mithuna",
        [2025, 5, 14, 23, 20],
        [2025, 5, 15, 2025, 5, 26],
    ),
    (
        "pushkaram-godavari",
        "simha",
        [2027, 6, 26, 5, 43],
        [2027, 6, 26, 2027, 7, 7],
    ),
];

#[test]
fn the_festivals_whose_dates_were_read_follow_the_final_entry_found_from_the_sky() {
    for (river, sign, entry, days) in FESTIVALS {
        let lines = festival(sign, entry[0], "pushkaram-final-entry");
        let (_, first, last, found) = lines
            .iter()
            .find(|(id, ..)| id == river)
            .unwrap_or_else(|| panic!("{river} {entry:?}: {lines:?}"));
        let (day, month, date) = (days[0], days[1], days[2]);
        assert_eq!(*first, rd(day, month as u8, date as u8), "{river}");
        assert_eq!(*last, rd(days[3], days[4] as u8, days[5] as u8), "{river}");
        // The entry is Drik Panchang's within the 25″ of the ayanāṃśa:
        // 40 to 70 minutes before it for a forward entry.
        let drik = ist(
            entry[0],
            entry[1] as u8,
            entry[2] as u8,
            entry[3] as u8,
            entry[4] as u8,
        );
        let minutes = ((found - hyper_calendar::astro_lines::unix_from_moment(drik)) as f64) / 60.0;
        assert!(
            (-80.0..-30.0).contains(&minutes),
            "{river} {minutes} minutes"
        );
    }
}

#[test]
fn the_other_reading_of_the_second_entry_opens_the_festivals_wikipedia_dates() {
    // Wikipedia's table opens the Tapti and Brahmaputra festival of 2019 on
    // 29 March and the Sindhu festival of 2021 on 6 April, at the first entry
    // into the sign; no festival read followed it.
    let first = festival("dhanus", 2019, "pushkaram-first-entry");
    assert_eq!(first[0].1, rd(2019, 3, 30));
    let first = festival("kumbha", 2021, "pushkaram-first-entry");
    assert_eq!(first[0].1, rd(2021, 4, 6));
    // The final entries are those of November.
    assert_eq!(
        festival("dhanus", 2019, "pushkaram-final-entry")[0].1,
        rd(2019, 11, 5)
    );
    assert_eq!(
        festival("kumbha", 2021, "pushkaram-final-entry")[0].1,
        rd(2021, 11, 21)
    );
    // Where Jupiter enters once, the rules agree.
    assert_eq!(
        festival("simha", 2015, "pushkaram-first-entry")[0].1,
        festival("simha", 2015, "pushkaram-final-entry")[0].1
    );
}

/// The lines of `pushkarams_in_year_lines` for a year, as `festival` reads a
/// sign's: (river, first day, last day, the entry's POSIX second).
fn festivals_of(year: i64, rule: &str) -> Vec<(String, Rd, Rd, i64)> {
    use hyper_calendar::jupiter_lines::{PUSHKARAMS_IN_YEAR_COLUMNS, pushkarams_in_year_lines};
    let text =
        pushkarams_in_year_lines(year, "lahiri", rule, NEW_DELHI, "india", "en").expect("lines");
    text.lines()
        .map(|line| {
            let cells: Vec<&str> = line.split('\t').collect();
            assert_eq!(cells.len(), PUSHKARAMS_IN_YEAR_COLUMNS, "{line}");
            (
                cells[0].to_owned(),
                Rd(cells[6].parse().expect("a day")),
                Rd(cells[7].parse().expect("a day")),
                cells[PUSHKARAMS_IN_YEAR_COLUMNS - 2]
                    .parse()
                    .expect("a second"),
            )
        })
        .collect()
}

/// The nine festivals whose dates were read, and the two Wikipedia opens at
/// first entries, are each in the one call that gives their year's lines,
/// on the same days and at the same second as in the call that gives their
/// sign's.
#[test]
fn the_year_export_gives_the_festivals_the_sign_export_does() {
    for (river, sign, entry, days) in FESTIVALS {
        let year = festivals_of(entry[0], "pushkaram-final-entry");
        let line = year
            .iter()
            .find(|(id, ..)| id == river)
            .unwrap_or_else(|| panic!("{river} {entry:?}: {year:?}"));
        let (first, last) = (
            rd(days[0], days[1] as u8, days[2] as u8),
            rd(days[3], days[4] as u8, days[5] as u8),
        );
        assert_eq!((line.1, line.2), (first, last), "{river}");
        let by_sign = festival(sign, entry[0], "pushkaram-final-entry");
        assert!(
            by_sign.iter().all(|line| year.contains(line)),
            "{river}: {by_sign:?} in {year:?}"
        );
    }
    // The first entry rule's: the Tapti and Brahmaputra festival of 2019 and
    // the Sindhu festival of 2021, as Wikipedia dates them.
    let wikipedia = festivals_of(2019, "pushkaram-first-entry");
    assert!(
        wikipedia
            .iter()
            .any(|(id, first, ..)| id == "pushkaram-tapti" && *first == rd(2019, 3, 30))
    );
    let sindhu = festivals_of(2021, "pushkaram-first-entry");
    assert!(
        sindhu
            .iter()
            .any(|(id, first, ..)| id == "pushkaram-sindhu" && *first == rd(2021, 4, 6))
    );
}

/// An ingress is one instant, to the last bit, however the search that finds it
/// is begun: 1 100 day windows from 1900 to 2100, each asked from its start and
/// from 3.7 days before it, give 317 ingresses that both find, and they are the
/// same ingresses to the bit. (Before the search was laid on a grid they
/// differed in every one, by up to 0.064 s, and by a whole second in eight.)
#[test]
fn an_ingress_is_the_same_instant_from_windows_that_start_apart() {
    let lahiri = Ayanamsa::LAHIRI;
    let (mut at, mut matched) = (693_596.0, 0);
    while at < 693_596.0 + 73_000.0 {
        let until = Moment(at + 1_100.0);
        let from_here: Vec<_> = jupiter::ingresses(Moment(at), until, lahiri).collect();
        let from_before: Vec<_> = jupiter::ingresses(Moment(at - 3.7), until, lahiri).collect();
        for ingress in &from_here {
            let other = from_before
                .iter()
                .find(|other| (other.moment.0 - ingress.moment.0).abs() < 1.0)
                .unwrap_or_else(|| panic!("{ingress:?} not found from before"));
            assert_eq!(other, ingress);
            matched += 1;
        }
        at += 1_100.0;
    }
    assert!(matched >= 300, "{matched}");
}

/// The ingress lines of a span, as (the moment's POSIX second, the sign
/// entered).
fn ingress_seconds(from: i64, to: i64) -> Vec<(i64, String)> {
    use hyper_calendar::jupiter_lines::{INGRESS_COLUMNS, ingress_lines};
    ingress_lines(from, to, "lahiri")
        .expect("lines")
        .lines()
        .map(|line| {
            let cells: Vec<&str> = line.split('\t').collect();
            assert_eq!(cells.len(), INGRESS_COLUMNS);
            (cells[0].parse().expect("a second"), cells[3].to_owned())
        })
        .collect()
}

/// `hc_jupiter_ingresses`, `hc_pushkaram_by_sky` and `hc_pushkarams_in_year`
/// give the same second for the same entry, from the spans that contain it
/// and from the years that do: the nine festivals' entries, each asked from
/// spans that begin days and months before it and end days and months after.
#[test]
fn the_ingress_and_pushkaram_exports_agree_on_an_entry() {
    let unix = |year: i64, month: u8, date: u8| (rd(year, month, date).0 - 719_163) * 86_400;
    for (river, sign, entry, _) in FESTIVALS {
        let by_sign = festival(sign, entry[0], "pushkaram-final-entry");
        let (_, _, _, second) = by_sign
            .iter()
            .find(|(id, ..)| id == river)
            .unwrap_or_else(|| panic!("{river}"));
        let in_year = festivals_of(entry[0], "pushkaram-final-entry");
        assert!(in_year.iter().any(|line| line.3 == *second), "{river}");
        let centre = unix(entry[0], entry[1] as u8, entry[2] as u8);
        for (before, after) in [
            (86_400 * 2, 86_400),
            (86_400 * 37 + 12_345, 86_400 * 41),
            (86_400 * 300, 86_400 * 500 + 7),
            (3_000, 86_400 * 20),
        ] {
            let found = ingress_seconds(centre - before, centre + after);
            let it = found
                .iter()
                .find(|(s, entered)| entered.as_str() == sign && (s - second).abs() < 86_400 * 2);
            assert_eq!(
                it.map(|(s, _)| *s),
                Some(*second),
                "{river}: {before} s before and {after} after"
            );
        }
    }
}

/// A rising and its setting are the same two instants from every span that
/// holds the rising: Drik Panchang's 2026 asta, asked from spans that begin
/// in the months before it.
#[test]
fn the_risings_are_the_same_instants_from_spans_that_start_apart() {
    use hyper_calendar::jupiter_lines::rising_lines;
    let unix = |month: u8, date: u8| (rd(2026, month, date).0 - 719_163) * 86_400;
    let mut found = Vec::new();
    for (from, to) in [
        (unix(1, 1), unix(12, 31)),
        (unix(1, 1) + 1, unix(9, 1)),
        (unix(4, 3) + 40_000, unix(8, 20)),
        (unix(6, 21), unix(8, 14) + 500),
        (unix(7, 20) + 7, unix(12, 1)),
    ] {
        let text = rising_lines(from, to, "lahiri").expect("lines");
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 1, "{from} {to}: {text}");
        let cells: Vec<&str> = lines[0].split('\t').collect();
        found.push((
            cells[0].to_owned(),
            cells[1].to_owned(),
            cells[2].to_owned(),
        ));
    }
    assert!(found.windows(2).all(|pair| pair[0] == pair[1]), "{found:?}");
}

#[test]
fn most_years_hold_no_entry_into_a_sign() {
    // Jupiter enters Siṃha once in twelve years, or three times in two. Each
    // year is a search of the whole year's sky: a build instrumented for
    // coverage takes every third (docs/policy.md §7).
    let step = if hyper_calendar::hc_core::sweep::INSTRUMENTED {
        3
    } else {
        1
    };
    for year in (2016..=2026).step_by(step) {
        assert!(
            festival("simha", year, "pushkaram-final-entry").is_empty(),
            "{year}"
        );
    }
}
