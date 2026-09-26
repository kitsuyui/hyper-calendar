//! The Javanese calendar's days against the weeks that run beside it.
//!
//! `hc_calendars_lunar::javanese` counts the years, months and days; the
//! pasaran, the weton and the *wuku* are this crate's `javanese_pasaran`
//! and `balinese_pawukon`. Tanaya's tables of the Javanese year give every
//! first day as a weekday, a pasaran and a wuku, so they check the two
//! crates against each other and against the book at once
//! (docs/systems/javanese.md).
//!
//! The source throughout is R. Tanaya, *Kabudayan Paugêraning Taun Jawa*,
//! Surakarta, 1971, as the Yayasan Sastra Lestari transcribes it
//! (sastra.org, #616) [tanaya1971], unless a test names another.

use hc_calendars_lunar::hc_calendar::{Calendar, Rd, gregorian};
use hc_calendars_lunar::javanese::{
    JAVANESE, JAVANESE_ABOGE, JAVANESE_YOGYAKARTA, JavaneseCalendar, JavaneseDate,
};
use hc_calendars_regional::javanese_pasaran::{DINA_NEPTU, PASARAN_NEPTU, pasaran_of};
use hc_calendars_regional::{BalinesePawukonCalendar, JavanesePasaranCalendar};

#[cfg(test)]
fn day(calendar: &JavaneseCalendar, year: i64, month: u8, day: u8) -> Rd {
    calendar
        .to_fixed(JavaneseDate::new(year, month, day))
        .unwrap()
}

#[cfg(test)]
fn weton(rd: Rd) -> String {
    JavanesePasaranCalendar.from_fixed(rd).unwrap().to_string()
}

/// "Dina kawitaning windu": the weton on which 1 Sura of each year of the
/// windu fell under each kurup, Alip to Jimakir, in `javanese_pasaran`'s
/// spellings (Tanaya writes Jum'at, Sabtu, Paing).
const JAM_IYAH: [&str; 8] = [
    "Jemuwah Legi",
    "Selasa Kliwon",
    "Ahad Kliwon",
    "Kemis Wage",
    "Senen Pon",
    "Setu Pon",
    "Rebo Pahing",
    "Ahad Legi",
];
const KAMSIYAH: [&str; 8] = [
    "Kemis Kliwon",
    "Senen Wage",
    "Setu Wage",
    "Rebo Pon",
    "Ahad Pahing",
    "Jemuwah Pahing",
    "Selasa Legi",
    "Setu Kliwon",
];
const ARBA_IYAH: [&str; 8] = [
    "Rebo Wage",
    "Ahad Pon",
    "Jemuwah Pon",
    "Selasa Pahing",
    "Setu Legi",
    "Kemis Legi",
    "Senen Kliwon",
    "Jemuwah Wage",
];
const SALASIYAH: [&str; 8] = [
    "Selasa Pon",
    "Setu Pahing",
    "Kemis Pahing",
    "Senen Legi",
    "Jemuwah Kliwon",
    "Rebo Kliwon",
    "Ahad Wage",
    "Kemis Pon",
];

/// Every year of every kurup Tanaya tabulates, in each reckoning: Surakarta
/// through the four kurup to Jimakir 1986; Yogyakarta, which ran Kamsiyah
/// to 1794 [karjanto2020]; the Aboge, who kept Arba'iyah [tempo-aboge-2017].
#[test]
fn the_first_day_of_every_year_has_tanayas_weton() {
    let spans: [(&JavaneseCalendar, i64, i64, &[&str; 8]); 10] = [
        (&JAVANESE, 1555, 1674, &JAM_IYAH),
        (&JAVANESE, 1675, 1748, &KAMSIYAH),
        (&JAVANESE, 1749, 1866, &ARBA_IYAH),
        (&JAVANESE, 1867, 1986, &SALASIYAH),
        (&JAVANESE_YOGYAKARTA, 1675, 1794, &KAMSIYAH),
        (&JAVANESE_YOGYAKARTA, 1795, 1866, &ARBA_IYAH),
        (&JAVANESE_YOGYAKARTA, 1867, 1986, &SALASIYAH),
        (&JAVANESE_ABOGE, 1555, 1674, &JAM_IYAH),
        (&JAVANESE_ABOGE, 1675, 1748, &KAMSIYAH),
        (&JAVANESE_ABOGE, 1749, 2346, &ARBA_IYAH),
    ];
    let mut checked = 0;
    for (calendar, first, last, table) in spans {
        for year in first..=last {
            let place = usize::try_from((year - 1555) % 8).unwrap();
            assert_eq!(
                weton(day(calendar, year, 1, 1)),
                table[place],
                "{} {year}",
                calendar.meta().id
            );
            checked += 1;
        }
    }
    assert_eq!(checked, 432 + 312 + 792);
}

/// "Wiwit tumindake taun Jawa ... dina Jum'at Lêgi, tanggal sapisan sasi
/// Muharram ing taun kang kawitan (Alif), windu Kunthara, nuju wuku
/// Kulawu": the first day was a Jumat Legi in wuku Kulawu, the
/// twenty-eighth, which the Balinese spell Kelawu.
#[test]
fn the_first_day_was_jumat_legi_in_wuku_kulawu() {
    let first = day(&JAVANESE, 1555, 1, 1);
    assert_eq!(weton(first), "Jemuwah Legi");
    let pawukon = BalinesePawukonCalendar.from_fixed(first).unwrap();
    assert_eq!(pawukon.wuku(), 28);
    assert_eq!(pawukon.wuku_name(), Ok("Kelawu"));
}

/// "Pananggalan Jawa 120 Taun, Khuruf Alip Salasiyah Pon": the neptu of the
/// weekday, the neptu of the pasaran and the number of the wuku of the
/// first of every month, for the windus that open on wuku Kulawu
/// (Kunthara and Sancaya) and on wuku Langkir (Adi and Sangara), as
/// printed. Rows are months, Sura first; columns are years, Alip first.
const KULAWU_WINDUS: [[(u8, u8, u8); 8]; 12] = [
    // Sura
    [
        (3, 7, 28),
        (9, 9, 18),
        (8, 9, 9),
        (4, 5, 30),
        (6, 8, 20),
        (7, 8, 11),
        (5, 4, 2),
        (8, 7, 22),
    ],
    // Sapar
    [
        (8, 7, 2),
        (4, 9, 23),
        (9, 9, 13),
        (7, 3, 4),
        (5, 8, 25),
        (6, 8, 15),
        (3, 4, 6),
        (9, 7, 26),
    ],
    // Mulud
    [
        (6, 9, 6),
        (3, 5, 27),
        (5, 5, 18),
        (8, 8, 8),
        (4, 4, 29),
        (9, 4, 19),
        (7, 7, 10),
        (5, 9, 1),
    ],
    // Bakda Mulud
    [
        (5, 9, 11),
        (8, 5, 1),
        (3, 5, 22),
        (9, 8, 12),
        (7, 4, 3),
        (4, 4, 24),
        (6, 7, 14),
        (3, 9, 5),
    ],
    // Jumadilawal
    [
        (4, 5, 15),
        (6, 8, 5),
        (7, 8, 26),
        (6, 4, 17),
        (8, 7, 7),
        (3, 7, 28),
        (9, 9, 18),
        (7, 5, 9),
    ],
    // Jumadilakir
    [
        (7, 5, 19),
        (5, 8, 10),
        (6, 8, 30),
        (3, 4, 21),
        (9, 7, 11),
        (8, 7, 2),
        (4, 9, 23),
        (6, 5, 13),
    ],
    // Rejeb
    [
        (8, 3, 23),
        (4, 4, 14),
        (9, 4, 4),
        (7, 7, 25),
        (5, 9, 16),
        (6, 9, 6),
        (3, 5, 27),
        (9, 8, 17),
    ],
    // Ruwah
    [
        (9, 8, 17),
        (7, 4, 18),
        (4, 4, 9),
        (6, 7, 29),
        (3, 9, 20),
        (5, 9, 11),
        (8, 5, 1),
        (4, 8, 22),
    ],
    // Pasa
    [
        (5, 4, 2),
        (8, 7, 22),
        (3, 7, 13),
        (9, 9, 3),
        (7, 5, 24),
        (4, 5, 15),
        (6, 8, 5),
        (3, 4, 26),
    ],
    // Sawal
    [
        (3, 4, 6),
        (9, 7, 26),
        (8, 7, 17),
        (4, 9, 8),
        (6, 5, 28),
        (7, 5, 19),
        (5, 8, 10),
        (8, 4, 30),
    ],
    // Sela
    [
        (7, 7, 10),
        (5, 9, 1),
        (6, 9, 21),
        (3, 5, 12),
        (9, 8, 2),
        (8, 8, 23),
        (4, 4, 14),
        (6, 7, 4),
    ],
    // Besar
    [
        (6, 7, 14),
        (3, 9, 5),
        (5, 9, 26),
        (8, 5, 16),
        (4, 8, 7),
        (9, 8, 27),
        (7, 4, 18),
        (5, 7, 9),
    ],
];

/// The same for the windus that open on wuku Langkir.
const LANGKIR_WINDUS: [[(u8, u8, u8); 8]; 12] = [
    // Sura
    [
        (3, 7, 13),
        (9, 9, 3),
        (8, 9, 24),
        (4, 5, 15),
        (6, 8, 5),
        (7, 8, 26),
        (5, 4, 17),
        (8, 7, 7),
    ],
    // Sapar
    [
        (8, 7, 17),
        (4, 9, 8),
        (9, 9, 28),
        (7, 5, 19),
        (5, 8, 10),
        (6, 8, 30),
        (3, 4, 21),
        (9, 7, 11),
    ],
    // Mulud
    [
        (6, 9, 21),
        (3, 5, 12),
        (5, 5, 3),
        (8, 8, 23),
        (4, 4, 14),
        (9, 4, 4),
        (7, 7, 25),
        (5, 9, 16),
    ],
    // Bakda Mulud
    [
        (5, 9, 26),
        (8, 5, 16),
        (3, 5, 7),
        (9, 8, 27),
        (7, 4, 18),
        (4, 4, 9),
        (6, 7, 29),
        (3, 9, 20),
    ],
    // Jumadilawal
    [
        (4, 5, 30),
        (6, 8, 20),
        (7, 8, 11),
        (5, 4, 2),
        (8, 7, 22),
        (3, 7, 13),
        (9, 9, 3),
        (7, 5, 24),
    ],
    // Jumadilakir
    [
        (7, 5, 4),
        (5, 8, 25),
        (6, 8, 15),
        (3, 4, 6),
        (9, 7, 26),
        (8, 7, 17),
        (4, 9, 8),
        (6, 5, 28),
    ],
    // Rejeb
    [
        (8, 8, 8),
        (4, 4, 29),
        (9, 4, 19),
        (7, 7, 10),
        (5, 9, 1),
        (6, 9, 21),
        (3, 5, 12),
        (9, 8, 2),
    ],
    // Ruwah
    [
        (9, 8, 12),
        (7, 4, 3),
        (4, 4, 24),
        (6, 7, 14),
        (3, 9, 5),
        (5, 9, 26),
        (8, 5, 16),
        (4, 8, 7),
    ],
    // Pasa
    [
        (5, 4, 17),
        (8, 7, 7),
        (3, 7, 28),
        (9, 9, 18),
        (7, 5, 9),
        (4, 5, 30),
        (6, 8, 20),
        (3, 4, 11),
    ],
    // Sawal
    [
        (3, 4, 21),
        (9, 7, 11),
        (8, 7, 2),
        (4, 9, 23),
        (6, 5, 13),
        (7, 5, 4),
        (5, 8, 25),
        (8, 4, 15),
    ],
    // Sela
    [
        (7, 7, 25),
        (5, 9, 16),
        (6, 9, 6),
        (3, 5, 27),
        (9, 8, 17),
        (8, 8, 8),
        (4, 4, 29),
        (6, 7, 19),
    ],
    // Besar
    [
        (6, 7, 29),
        (3, 9, 20),
        (5, 9, 11),
        (8, 5, 1),
        (4, 8, 22),
        (9, 8, 12),
        (7, 4, 3),
        (5, 7, 24),
    ],
];

/// The first year of each windu Tanaya heads his two Salasiyah tables
/// with: Kunthara and Sancaya for the Kulawu table, Adi and Sangara for the
/// Langkir one.
const KULAWU_WINDU_YEARS: [i64; 7] = [1875, 1891, 1907, 1923, 1939, 1955, 1971];
const LANGKIR_WINDU_YEARS: [i64; 8] = [1867, 1883, 1899, 1915, 1931, 1947, 1963, 1979];

/// The misprints: month and year of the windu, each disagreeing with the
/// rule by its own evidence (docs/systems/javanese.md). Two print a pasaran
/// neptu of 3, which no pasaran has; one prints Jumat where the Langkir
/// table's same weton is Ahad; one prints wuku 17 where the Langkir table's
/// 12 plus fifteen is 27.
const MISPRINTS: [(u8, usize); 4] = [(2, 3), (5, 3), (7, 0), (8, 0)];

/// Every month of fifteen windus of Salasiyah, 1440 first days, against
/// the neptu `javanese_pasaran` carries and the wuku of `balinese_pawukon`.
/// Every entry agrees except the four misprints, in each of the seven
/// windus the Kulawu table serves.
#[test]
fn the_months_of_two_windu_open_as_tanaya_tabulates() {
    let mut disagreements = Vec::new();
    let mut checked = 0;
    for (table, years) in [
        (&KULAWU_WINDUS, &KULAWU_WINDU_YEARS[..]),
        (&LANGKIR_WINDUS, &LANGKIR_WINDU_YEARS[..]),
    ] {
        for &windu in years {
            for (month, row) in (1u8..).zip(table.iter()) {
                for (place, &printed) in row.iter().enumerate() {
                    let year = windu + i64::try_from(place).unwrap();
                    let rd = day(&JAVANESE, year, month, 1);
                    let weekday = hc_calendars_lunar::hc_calendar::Weekday::from_rd(rd);
                    let computed = (
                        u8::try_from(DINA_NEPTU[usize::from(weekday.sunday_first_number())])
                            .unwrap(),
                        u8::try_from(PASARAN_NEPTU[usize::from(pasaran_of(rd) - 1)]).unwrap(),
                        BalinesePawukonCalendar.from_fixed(rd).unwrap().wuku(),
                    );
                    checked += 1;
                    if computed != printed {
                        disagreements.push((windu, month, place));
                    }
                }
            }
        }
    }
    assert_eq!(checked, 15 * 96);
    let expected: Vec<(i64, u8, usize)> = KULAWU_WINDU_YEARS
        .iter()
        .flat_map(|&windu| {
            let mut cells: Vec<(i64, u8, usize)> = MISPRINTS
                .iter()
                .map(|&(month, place)| (windu, month, place))
                .collect();
            cells.sort_unstable();
            cells
        })
        .collect();
    let mut found = disagreements.clone();
    found.sort_unstable();
    assert_eq!(found, expected);
}

/// Dated days with the weton their sources give.
#[test]
fn the_published_days_carry_their_weton() {
    for (calendar, date, expected) in [
        // Sultan Agung's first Garebeg Mulud, 12 Mulud Dal 1559, Senen Pon.
        (&JAVANESE, JavaneseDate::new(1559, 3, 12), "Senen Pon"),
        // Garebeg Mulud of Je 1902, Senen Legi.
        (&JAVANESE, JavaneseDate::new(1902, 3, 12), "Senen Legi"),
        // Bagus Ngarfah's "Rebo Wage tanggal kaping 4 wulan Zu'lkaedah taun
        // Dal 1831", in the plain dates.
        (&JAVANESE, JavaneseDate::new(1831, 11, 4), "Rebo Wage"),
        // PB V's "dintên Kêmis tanggal kaping 29 wulan Bêsar taun Ehe 1748",
        // and the Jumat that followed.
        (&JAVANESE, JavaneseDate::new(1748, 12, 29), "Kemis Pahing"),
        (&JAVANESE, JavaneseDate::new(1749, 1, 1), "Jemuwah Pon"),
        // "24 Marêt 1936 (marêngi ing dintên Sêlasa Pon)".
        (&JAVANESE, JavaneseDate::new(1867, 1, 1), "Selasa Pon"),
        // "13 Sura 1682 AJ (7 Oktober 1756 CE), a Kemis Pahing"
        // [karjanto2020].
        (&JAVANESE, JavaneseDate::new(1682, 1, 13), "Kemis Pahing"),
        // "Jumat Kliwon, 1 Suro 1959 Dal" and "Sabtu Wage, 30 Suro 1959 Dal"
        // [kompas-suro-1959].
        (&JAVANESE, JavaneseDate::new(1959, 1, 1), "Jemuwah Kliwon"),
        (&JAVANESE, JavaneseDate::new(1959, 1, 30), "Setu Wage"),
        // Rebo Legi, 19 July 2023, 30 Besar 1956 [detik-besar-1956].
        (&JAVANESE, JavaneseDate::new(1956, 12, 30), "Rebo Legi"),
        // The Aboge year Je 1950, whose 1 Muharram "jatuh pada Selasa
        // dengan hari pasaran Pahing" [tempo-aboge-2017].
        (
            &JAVANESE_ABOGE,
            JavaneseDate::new(1950, 1, 1),
            "Selasa Pahing",
        ),
        // "Ramadan jatuh pada Rabu Wage 13 Maret 2024" and Idul Fitri "Jumat
        // Wage 12 April 2024" [detik-aboge-2024].
        (&JAVANESE_ABOGE, JavaneseDate::new(1957, 9, 1), "Rebo Wage"),
        (
            &JAVANESE_ABOGE,
            JavaneseDate::new(1957, 10, 1),
            "Jemuwah Wage",
        ),
        // The fast from "Minggu Pon", Idul Fitri "Selasa Pon"
        // [kompas-aboge-2025].
        (&JAVANESE_ABOGE, JavaneseDate::new(1958, 9, 1), "Ahad Pon"),
        (
            &JAVANESE_ABOGE,
            JavaneseDate::new(1958, 10, 1),
            "Selasa Pon",
        ),
    ] {
        let rd = calendar.to_fixed(date).unwrap();
        assert_eq!(weton(rd), expected, "{date:?}");
    }
    // Every Alip year of the Aboge reckoning opens on Rebo Wage, as its
    // name says, to the end of the range.
    for year in (1755..=2339).step_by(8) {
        assert_eq!(weton(day(&JAVANESE_ABOGE, year, 1, 1)), "Rebo Wage");
    }
    assert_eq!(
        day(&JAVANESE_ABOGE, 1957, 9, 1),
        gregorian::to_fixed(2024, 3, 13).unwrap()
    );
}
