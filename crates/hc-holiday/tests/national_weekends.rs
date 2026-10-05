//! The weekend each national table and each exchange reads, and the first
//! date it reads one from (ADR 0015).
//!
//! Before that date the weekend law is a gap: `weekend_in` answers `None`,
//! not Saturday and Sunday. Each row lists the regimes of the table's own
//! policy as the day each begins and the days it gives, `None` for the
//! years not read, in the order `hc_holiday::countries::weekends` states
//! them; the sources and the basis of each are in
//! `docs/systems/national-weekends.md`.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;
use hc_holiday::rule::RuleSet;
use hc_holiday::{countries, exchanges, international, traditions};

const SAT_SUN: &[Weekday] = &[Weekday::Saturday, Weekday::Sunday];
const FRI_SAT: &[Weekday] = &[Weekday::Friday, Weekday::Saturday];
const THU_FRI: &[Weekday] = &[Weekday::Thursday, Weekday::Friday];
const FRI_SUN: &[Weekday] = &[Weekday::Friday, Weekday::Sunday];
const FRIDAY: &[Weekday] = &[Weekday::Friday];
const SATURDAY: &[Weekday] = &[Weekday::Saturday];
const SUNDAY: &[Weekday] = &[Weekday::Sunday];

type Regimes = &'static [((i64, u8, u8), Option<&'static [Weekday]>)];

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

/// The national tables: the regimes of each, from the year 1900 (not read) on.
const COUNTRIES: &[(&str, Regimes)] = &[
    ("AD", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "AE",
        &[
            ((1900, 1, 1), None),
            ((1999, 1, 1), Some(THU_FRI)),
            ((2006, 9, 1), Some(FRI_SAT)),
            ((2022, 1, 1), Some(SAT_SUN)),
        ],
    ),
    (
        "AF",
        &[
            ((1900, 1, 1), None),
            ((2010, 1, 1), Some(FRIDAY)),
            ((2010, 12, 2), Some(THU_FRI)),
            ((2019, 1, 1), None),
            ((2026, 1, 1), Some(FRIDAY)),
        ],
    ),
    ("AG", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("AL", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "AM",
        &[((1900, 1, 1), None), ((2005, 6, 21), Some(SAT_SUN))],
    ),
    ("AO", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("AR", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("AT", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "AU",
        &[((1900, 1, 1), None), ((2015, 5, 14), Some(SAT_SUN))],
    ),
    ("AZ", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("BA", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("BB", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "BD",
        &[
            ((1900, 1, 1), None),
            ((1982, 4, 1), Some(FRIDAY)),
            ((2005, 9, 9), Some(FRI_SAT)),
        ],
    ),
    ("BE", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("BF", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("BG", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "BH",
        &[
            ((1900, 1, 1), None),
            ((1990, 3, 1), Some(THU_FRI)),
            ((2006, 9, 1), Some(FRI_SAT)),
        ],
    ),
    ("BI", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("BJ", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("BN", &[((1900, 1, 1), None), ((2019, 1, 1), Some(FRI_SUN))]),
    (
        "BO",
        &[((1900, 1, 1), None), ((2010, 12, 26), Some(SAT_SUN))],
    ),
    ("BR", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("BS", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("BT", &[((1900, 1, 1), None), ((2016, 1, 1), Some(SAT_SUN))]),
    ("BW", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("BY", &[((1900, 1, 1), None), ((2000, 1, 1), Some(SAT_SUN))]),
    ("BZ", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("CA", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "CD",
        &[
            ((1900, 1, 1), None),
            ((2024, 1, 1), Some(SUNDAY)),
            ((2024, 8, 1), Some(SAT_SUN)),
        ],
    ),
    ("CG", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("CH", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("CI", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "CL",
        &[((1900, 1, 1), None), ((2005, 3, 16), Some(SAT_SUN))],
    ),
    ("CM", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "CN",
        &[
            ((1900, 1, 1), None),
            ((1995, 3, 25), Some(SUNDAY)),
            ((1995, 5, 1), Some(SAT_SUN)),
        ],
    ),
    ("CO", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("CR", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("CU", &[((1900, 1, 1), None), ((2014, 1, 1), Some(SUNDAY))]),
    ("CV", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("CY", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "CZ",
        &[((1900, 1, 1), None), ((1968, 6, 10), Some(SAT_SUN))],
    ),
    ("DE", &[((1900, 1, 1), None), ((2006, 3, 1), Some(SAT_SUN))]),
    ("DJ", &[((1900, 1, 1), None), ((2017, 1, 1), Some(FRI_SAT))]),
    ("DK", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("DM", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("DO", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "DZ",
        &[
            ((1900, 1, 1), None),
            ((1975, 1, 1), Some(SAT_SUN)),
            ((1976, 1, 1), Some(THU_FRI)),
            ((2009, 8, 14), Some(FRI_SAT)),
        ],
    ),
    (
        "EC",
        &[((1900, 1, 1), None), ((2010, 10, 6), Some(SAT_SUN))],
    ),
    ("EE", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "EG",
        &[
            ((1900, 1, 1), None),
            ((2006, 1, 1), Some(FRIDAY)),
            ((2006, 1, 21), Some(FRI_SAT)),
        ],
    ),
    ("ES", &[((1900, 1, 1), None), ((2019, 3, 1), Some(SAT_SUN))]),
    ("ET", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("FI", &[((1900, 1, 1), None), ((1969, 4, 1), Some(SAT_SUN))]),
    ("FJ", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("FM", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("FR", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("GA", &[((1900, 1, 1), None), ((2022, 1, 1), Some(SUNDAY))]),
    ("GB", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("GD", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("GE", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("GH", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("GM", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("GN", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("GQ", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("GR", &[((1900, 1, 1), None), ((1981, 1, 1), Some(SAT_SUN))]),
    ("GT", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("GW", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SUNDAY))]),
    ("GY", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("HK", &[((1900, 1, 1), None), ((2006, 7, 3), Some(SAT_SUN))]),
    ("HN", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("HR", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("HT", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("HU", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "ID",
        &[((1900, 1, 1), None), ((1995, 10, 1), Some(SAT_SUN))],
    ),
    ("IE", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "IL",
        &[((1900, 1, 1), None), ((1952, 1, 1), Some(SATURDAY))],
    ),
    ("IN", &[((1900, 1, 1), None), ((1985, 6, 3), Some(SAT_SUN))]),
    ("IQ", &[((1900, 1, 1), None), ((2005, 3, 1), Some(FRI_SAT))]),
    ("IR", &[((1900, 1, 1), None), ((1991, 1, 1), Some(FRIDAY))]),
    ("IS", &[((1900, 1, 1), None), ((1983, 1, 1), Some(SAT_SUN))]),
    ("IT", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("JM", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "JO",
        &[
            ((1900, 1, 1), None),
            ((2000, 1, 1), Some(THU_FRI)),
            ((2000, 1, 8), Some(FRI_SAT)),
        ],
    ),
    ("JP", &[((1900, 1, 1), None), ((1992, 5, 1), Some(SAT_SUN))]),
    (
        "KE",
        &[((1900, 1, 1), None), ((2023, 9, 25), Some(SAT_SUN))],
    ),
    ("KG", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("KH", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SUNDAY))]),
    ("KI", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("KM", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("KN", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("KP", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SUNDAY))]),
    ("KR", &[((1900, 1, 1), None), ((2005, 7, 1), Some(SAT_SUN))]),
    (
        "KW",
        &[
            ((1900, 1, 1), None),
            ((2007, 1, 1), Some(THU_FRI)),
            ((2007, 9, 1), Some(FRI_SAT)),
        ],
    ),
    ("KZ", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "LA",
        &[((1900, 1, 1), None), ((2017, 12, 19), Some(SAT_SUN))],
    ),
    ("LB", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("LC", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("LI", &[((1900, 1, 1), None), ((2009, 1, 1), Some(SAT_SUN))]),
    ("LK", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("LR", &[((1900, 1, 1), None), ((2016, 1, 1), Some(SUNDAY))]),
    ("LS", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("LT", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("LU", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("LV", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("LY", &[((1900, 1, 1), None), ((2006, 1, 1), Some(FRI_SAT))]),
    (
        "MA",
        &[((1900, 1, 1), None), ((2005, 7, 20), Some(SAT_SUN))],
    ),
    ("MC", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("MD", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("ME", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("MG", &[((1900, 1, 1), None), ((2009, 8, 1), Some(SAT_SUN))]),
    ("MH", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("MK", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("ML", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("MM", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("MN", &[((1900, 1, 1), None), ((1999, 7, 1), Some(SAT_SUN))]),
    ("MO", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "MR",
        &[
            ((1900, 1, 1), None),
            ((2005, 4, 6), Some(SAT_SUN)),
            ((2007, 12, 23), Some(FRI_SAT)),
            ((2014, 10, 1), Some(SAT_SUN)),
        ],
    ),
    ("MT", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("MU", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("MV", &[((1900, 1, 1), None), ((2013, 1, 1), Some(FRI_SAT))]),
    ("MW", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("MX", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "MY",
        &[((1900, 1, 1), None), ((2013, 11, 25), Some(SAT_SUN))],
    ),
    ("MZ", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("NA", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("NE", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("NG", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("NI", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("NL", &[((1900, 1, 1), None), ((1965, 4, 1), Some(SAT_SUN))]),
    ("NO", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "NP",
        &[
            ((1900, 1, 1), None),
            ((2022, 1, 1), Some(SATURDAY)),
            ((2022, 5, 15), Some(SAT_SUN)),
            ((2022, 6, 15), Some(SATURDAY)),
            ((2026, 4, 6), Some(SAT_SUN)),
        ],
    ),
    ("NR", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("NZ", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "OM",
        &[
            ((1900, 1, 1), None),
            ((2013, 1, 1), Some(THU_FRI)),
            ((2013, 5, 1), Some(FRI_SAT)),
        ],
    ),
    ("PA", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("PE", &[((1900, 1, 1), None), ((1996, 1, 4), Some(SAT_SUN))]),
    ("PG", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "PH",
        &[((1900, 1, 1), None), ((1991, 12, 27), Some(SAT_SUN))],
    ),
    ("PK", &[((1900, 1, 1), None), ((2022, 6, 9), Some(SAT_SUN))]),
    (
        "PL",
        &[
            ((1900, 1, 1), None),
            ((1951, 1, 1), Some(SUNDAY)),
            ((1973, 1, 1), None),
            ((2001, 1, 1), Some(SAT_SUN)),
        ],
    ),
    (
        "PS",
        &[
            ((1900, 1, 1), None),
            ((2007, 1, 1), Some(THU_FRI)),
            ((2007, 7, 1), Some(FRI_SAT)),
        ],
    ),
    ("PT", &[((1900, 1, 1), None), ((2014, 8, 1), Some(SAT_SUN))]),
    ("PW", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("PY", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "QA",
        &[
            ((1900, 1, 1), None),
            ((2003, 1, 1), Some(THU_FRI)),
            ((2003, 8, 1), Some(FRI_SAT)),
        ],
    ),
    ("RO", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("RS", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "RU",
        &[
            ((1900, 1, 1), None),
            ((1940, 1, 1), Some(SUNDAY)),
            ((1967, 1, 1), None),
            ((1968, 1, 1), Some(SAT_SUN)),
        ],
    ),
    ("RW", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "SA",
        &[
            ((1900, 1, 1), None),
            ((2013, 1, 1), Some(THU_FRI)),
            ((2013, 6, 29), Some(FRI_SAT)),
        ],
    ),
    ("SB", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("SC", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "SD",
        &[
            ((1900, 1, 1), None),
            ((1998, 1, 21), Some(FRIDAY)),
            ((2008, 1, 26), Some(FRI_SAT)),
        ],
    ),
    ("SE", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("SG", &[((1900, 1, 1), None), ((2004, 9, 1), Some(SAT_SUN))]),
    ("SI", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "SK",
        &[((1900, 1, 1), None), ((1968, 6, 10), Some(SAT_SUN))],
    ),
    ("SL", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("SM", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("SN", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("SO", &[((1900, 1, 1), None), ((2025, 1, 1), Some(FRIDAY))]),
    ("SR", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("SS", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("SV", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("SY", &[((1900, 1, 1), None), ((2004, 2, 1), Some(FRI_SAT))]),
    ("SZ", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SUNDAY))]),
    ("TD", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("TG", &[((1900, 1, 1), None), ((2022, 1, 1), Some(SUNDAY))]),
    ("TH", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("TJ", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("TL", &[((1900, 1, 1), None), ((2013, 1, 1), Some(SUNDAY))]),
    ("TM", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "TN",
        &[((1900, 1, 1), None), ((2012, 9, 17), Some(SAT_SUN))],
    ),
    ("TO", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "TR",
        &[
            ((1900, 1, 1), None),
            ((1924, 1, 2), Some(FRIDAY)),
            ((1935, 6, 2), Some(SUNDAY)),
            ((1974, 7, 1), Some(SAT_SUN)),
        ],
    ),
    ("TT", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("TV", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "TW",
        &[
            ((1900, 1, 1), None),
            ((1998, 1, 1), None),
            ((2001, 1, 1), Some(SAT_SUN)),
        ],
    ),
    ("TZ", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("UA", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("UG", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("US", &[((1900, 1, 1), None), ((1966, 9, 6), Some(SAT_SUN))]),
    ("UY", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("UZ", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("VA", &[((1900, 1, 1), None), ((2011, 1, 1), Some(SUNDAY))]),
    ("VC", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("VE", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("VN", &[((1900, 1, 1), None), ((2021, 1, 1), Some(SAT_SUN))]),
    ("VU", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("WS", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    (
        "YE",
        &[
            ((1900, 1, 1), None),
            ((2013, 1, 1), Some(THU_FRI)),
            ((2013, 8, 15), Some(FRI_SAT)),
        ],
    ),
    ("ZA", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("ZM", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
    ("ZW", &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))]),
];

/// The exchanges.
const EXCHANGES: &[(&str, Regimes)] = &[
    (
        "BVMF",
        &[((1900, 1, 1), None), ((2021, 1, 1), Some(SAT_SUN))],
    ),
    (
        "MISX",
        &[((1900, 1, 1), None), ((2023, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XAMS",
        &[((1900, 1, 1), None), ((2021, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XASX",
        &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XBKK",
        &[((1900, 1, 1), None), ((2022, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XBOM",
        &[((1900, 1, 1), None), ((2020, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XBRU",
        &[((1900, 1, 1), None), ((2021, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XCSE",
        &[((1900, 1, 1), None), ((2025, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XDUB",
        &[((1900, 1, 1), None), ((2021, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XETR",
        &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XHEL",
        &[((1900, 1, 1), None), ((2025, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XHKG",
        &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XICE",
        &[((1900, 1, 1), None), ((2025, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XIDX",
        &[((1900, 1, 1), None), ((2020, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XIST",
        &[((1900, 1, 1), None), ((2019, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XJPX",
        &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XJSE",
        &[((1900, 1, 1), None), ((2024, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XKLS",
        &[((1900, 1, 1), None), ((2020, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XKRX",
        &[((1900, 1, 1), None), ((2009, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XLIS",
        &[((1900, 1, 1), None), ((2021, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XLON",
        &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XMAD",
        &[((1900, 1, 1), None), ((2023, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XMEX",
        &[((1900, 1, 1), None), ((2019, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XMIL",
        &[((1900, 1, 1), None), ((2021, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XNAS",
        &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XNSE",
        &[((1900, 1, 1), None), ((2020, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XNYS",
        &[((1900, 1, 1), None), ((1953, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XNZE",
        &[((1900, 1, 1), None), ((2021, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XOSL",
        &[((1900, 1, 1), None), ((2021, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XPAR",
        &[((1900, 1, 1), None), ((2021, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XPHS",
        &[((1900, 1, 1), None), ((2020, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XSAU",
        &[
            ((1900, 1, 1), None),
            ((2006, 2, 25), Some(THU_FRI)),
            ((2013, 6, 29), Some(FRI_SAT)),
        ],
    ),
    (
        "XSES",
        &[((1900, 1, 1), None), ((2020, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XSHE",
        &[((1900, 1, 1), None), ((2015, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XSHG",
        &[((1900, 1, 1), None), ((2014, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XSTO",
        &[((1900, 1, 1), None), ((2025, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XSWX",
        &[((1900, 1, 1), None), ((2026, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XTAE",
        &[
            ((1900, 1, 1), None),
            ((2024, 1, 1), Some(FRI_SAT)),
            ((2026, 1, 4), Some(SAT_SUN)),
        ],
    ),
    (
        "XTAI",
        &[((1900, 1, 1), None), ((2023, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XTSE",
        &[((1900, 1, 1), None), ((2025, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XWAR",
        &[((1900, 1, 1), None), ((2019, 1, 1), Some(SAT_SUN))],
    ),
    (
        "XWBO",
        &[((1900, 1, 1), None), ((2019, 1, 1), Some(SAT_SUN))],
    ),
];

fn check(set: &RuleSet, regimes: Regimes) {
    let code = set.code;
    let mut before: Option<Option<&[Weekday]>> = None;
    for &((year, month, day), days) in regimes {
        let start = ymd(year, month, day);
        assert_eq!(
            set.weekend_on(start),
            days,
            "{code} on {year}-{month}-{day}"
        );
        if let Some(previous) = before {
            assert_eq!(
                set.weekend_on(Rd(start.0 - 1)),
                previous,
                "{code} the day before {year}-{month}-{day}"
            );
        }
        before = Some(days);
    }
    // Long before, and long after, the same as the first and the last.
    assert_eq!(set.weekend_on(ymd(1500, 1, 1)), None, "{code} in 1500");
    assert_eq!(
        set.weekend_on(ymd(2100, 1, 1)),
        regimes.last().and_then(|row| row.1),
        "{code} in 2100"
    );
}

#[test]
fn every_national_table_reads_its_weekend_from_a_first_date() {
    assert_eq!(COUNTRIES.len(), countries::ALL.len());
    for table in countries::ALL {
        let Some(&(_, regimes)) = COUNTRIES.iter().find(|(code, _)| *code == table.code) else {
            panic!("{} has no row", table.code);
        };
        assert!(table.states_a_weekend(), "{}", table.code);
        check(table, regimes);
    }
}

#[test]
fn every_exchange_reads_its_trading_week_from_a_first_date() {
    assert_eq!(EXCHANGES.len(), exchanges::ALL.len());
    for table in exchanges::ALL {
        let Some(&(_, regimes)) = EXCHANGES.iter().find(|(code, _)| *code == table.code) else {
            panic!("{} has no row", table.code);
        };
        assert!(table.states_a_weekend(), "{}", table.code);
        check(table, regimes);
    }
}

#[test]
fn a_tradition_or_a_list_of_days_states_no_weekend_and_keeps_saturday_and_sunday() {
    for table in traditions::ALL.iter().chain(international::ALL) {
        assert!(!table.states_a_weekend(), "{}", table.code);
        assert!(table.weekend.is_empty(), "{}", table.code);
        assert_eq!(
            table.weekend_on(ymd(1500, 1, 1)),
            Some(SAT_SUN),
            "{}",
            table.code
        );
        assert!(!table.weekend_unread_in(None, 1500), "{}", table.code);
    }
}

/// The system document's text, which lists every national table either with
/// the date its weekend is first read and the keys of its sources, or among
/// those read from 2026 only.
const DOCUMENT: &str = include_str!("../../../docs/systems/national-weekends.md");

fn between<'a>(text: &'a str, from: &str, to: &str) -> &'a str {
    let Some(start) = text.find(from) else {
        panic!("the document has no {from:?}");
    };
    let rest = &text[start..];
    let end = rest[from.len()..]
        .find(to)
        .map_or(rest.len(), |at| at + from.len());
    &rest[..end]
}

#[test]
fn the_document_gives_every_table_its_first_year_and_its_sources() {
    let dated = between(
        DOCUMENT,
        "#### Tables with a dated weekend",
        "#### Tables read from 2026 only",
    );
    let undated = between(
        DOCUMENT,
        "#### Tables read from 2026 only",
        "#### Examined and not carried",
    );
    let mut listed = 0;
    for table in countries::ALL {
        let code = table.code;
        let first = table.weekend_first_year(None);
        let row = dated
            .lines()
            .find(|line| line.starts_with(&format!("| {code} | ")));
        match row {
            Some(row) => {
                listed += 1;
                let cells: Vec<&str> = row.split('|').map(str::trim).collect();
                // "", code, country, first read, weekend, sources, ""
                assert_eq!(cells.len(), 7, "{code}: {row}");
                let year: i64 = cells[3]
                    .rsplit(' ')
                    .next()
                    .and_then(|year| year.parse().ok())
                    .unwrap_or_else(|| panic!("{code}: first read {:?}", cells[3]));
                assert_eq!(first, Some(year), "{code}: first read in the document");
                assert!(
                    year < 2026 || cells[3] != "2026",
                    "{code} is dated, but is first read in 2026"
                );
                assert!(
                    cells[5].contains('`') || cells[5].starts_with("not re-read"),
                    "{code}: the sources column"
                );
            }
            None => {
                assert_eq!(first, Some(2026), "{code} is read from 2026 only");
                let there = undated.contains(&format!(": {code} "))
                    || undated.contains(&format!(", {code} "));
                assert!(there, "{code} is in neither list of the document");
            }
        }
    }
    let rows = dated
        .lines()
        .filter(|line| {
            line.starts_with("| ") && !line.starts_with("| Code") && !line.starts_with("| ---")
        })
        .count();
    assert_eq!(
        rows, listed,
        "a row of the dated table is no national table"
    );
}
