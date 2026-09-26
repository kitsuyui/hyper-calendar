//! Where and when the Gregorian calendar was adopted, by country.
//!
//! [`crate::julian_gregorian`] carries fourteen cut-overs as calendars, one
//! identifier each, because a date written in one of those polities needs
//! its own arithmetic. This module asks the other question — *when did this
//! country start writing Gregorian dates, and from what?* — and answers it
//! by ISO 3166-1 alpha-2 code, one row per step, because many countries got
//! there in more than one: China declared the solar calendar in 1912 and
//! made it the only one in 1929, Sweden took one step in 1700, stepped back
//! in 1712 and changed at once in 1753, and the Dutch provinces changed one
//! by one.
//!
//! The system is written up in `docs/systems/gregorian-reform.md` in the
//! repository, whose regional table is this one; the rows that are Julian
//! cut-overs take their dates and their instruments from
//! [`crate::julian_gregorian::ADOPTIONS`] rather than restating them, so that
//! each of those dates has one source, and the rest name their own. Every
//! cut-over of that table is the step of at least one row here.
//!
//! # A row
//!
//! Every step is a pair of consecutive fixed days: the last day of the old
//! reckoning and the first of the new, with the week unbroken between them.
//! The old calendar is named by its registry identifier — `julian`,
//! `japanese-tenpo`, `dangi`, `chinese`, `islamic-umalqura`, `rumi`,
//! `swedish-1700` — and so is the new one, which is `gregory` on every row
//! but two: Sweden's step of 1700 was to `swedish-1700` and its step of 1712
//! back to `julian`.
//!
//! The [`Scope`] says how far the step reached. A **civil** step changed
//! the calendar of civil life for the whole of the polity as it then was.
//! A **partial** one changed it for part of the country (a Dutch province,
//! the Catholic territories of the Empire), for some purposes only (Saudi
//! Arabia's public-sector pay in 2016), or in part of the calendar (the
//! Rumi calendar's move to the Gregorian days in 1917, which kept its own
//! year). An **ecclesiastical** step changed a church's calendar and not the
//! state's; the vocabulary has the word because such steps exist, and no
//! row carries it yet, because no source for one has been read.
//!
//! # What a country code means here
//!
//! The code is today's country, and a row is a step taken by whatever polity
//! then governed the land, which is said in [`RegionalAdoption::polity`]:
//! Finland's rows are Sweden's, Norway's are Denmark–Norway's, and both
//! Koreas' row is the Joseon court's. A code with no row is a country this
//! table does not know, not one that never adopted the calendar. Provinces
//! and territories with no alpha-2 code of their own — the Dutch provinces,
//! the German states — are rows of their country with a partial scope, the
//! polity naming which part.

use hc_calendar::{CalendarError, CalendarResult, Rd};

use crate::julian_gregorian::{self, Adoption};
use crate::{gregorian, rumi, swedish};

/// How far one step of an adoption reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scope {
    /// The calendar of civil life, throughout the polity as it then was.
    Civil,
    /// A church's calendar, and not the state's.
    Ecclesiastical,
    /// Part of the country, some purposes only, or part of the calendar.
    Partial,
}

impl Scope {
    /// The word a line carries: `civil`, `ecclesiastical` or `partial`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Civil => "civil",
            Self::Ecclesiastical => "ecclesiastical",
            Self::Partial => "partial",
        }
    }
}

/// When a step took effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// The cut-over of the row of [`julian_gregorian::ADOPTIONS`] with this
    /// identifier, whose dates and instrument the step shares.
    Reform(&'static str),
    /// The first day of the new reckoning.
    From(Rd),
}

/// One step of one country's adoption of the Gregorian calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionalAdoption {
    /// The country, as an ISO 3166-1 alpha-2 code.
    pub region: &'static str,
    /// Who took the step, in English: the polity then governing, and the
    /// part of the country where the scope is partial.
    pub polity: &'static str,
    /// When the step took effect.
    pub step: Step,
    /// The registry identifier of the calendar kept until the step.
    pub old_calendar: &'static str,
    /// The registry identifier of the calendar kept from it.
    pub new_calendar: &'static str,
    /// How far the step reached.
    pub scope: Scope,
    /// The instrument behind the step, with its date, and whether it was
    /// read; empty on a [`Step::Reform`] row, whose adoption names it.
    own_source: &'static str,
}

impl RegionalAdoption {
    /// The tabulated cut-over a [`Step::Reform`] row shares.
    fn reform(&self) -> Option<Adoption> {
        match self.step {
            Step::Reform(id) => julian_gregorian::adoption_by_id(id),
            Step::From(_) => None,
        }
    }

    /// The first day of the new reckoning.
    ///
    /// # Errors
    ///
    /// [`CalendarError::UnsupportedField`] for a [`Step::Reform`] row whose
    /// identifier is not tabulated, which a unit test rules out, and what
    /// [`Adoption::cutover`] returns.
    pub fn first_day(&self) -> CalendarResult<Rd> {
        match self.step {
            Step::From(rd) => Ok(rd),
            Step::Reform(_) => self
                .reform()
                .ok_or(CalendarError::UnsupportedField("adoption"))?
                .cutover(),
        }
    }

    /// The last day of the old reckoning: the day before
    /// [`RegionalAdoption::first_day`], since no step broke the week.
    ///
    /// # Errors
    ///
    /// See [`RegionalAdoption::first_day`].
    pub fn last_old_day(&self) -> CalendarResult<Rd> {
        self.first_day().map(|rd| Rd(rd.0 - 1))
    }

    /// The instrument behind the step, with its date, and whether it was
    /// read.
    #[must_use]
    pub fn source(&self) -> &'static str {
        if !self.own_source.is_empty() {
            return self.own_source;
        }
        self.reform().map_or("", |adoption| adoption.source)
    }
}

/// A Gregorian date as a fixed day, for the table below; a malformed date
/// is day 0, which the tests rule out.
const fn day(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(rd) => rd,
        Err(_) => Rd(0),
    }
}

const fn reform(
    region: &'static str,
    polity: &'static str,
    id: &'static str,
    scope: Scope,
) -> RegionalAdoption {
    RegionalAdoption {
        region,
        polity,
        step: Step::Reform(id),
        old_calendar: "julian",
        new_calendar: "gregory",
        scope,
        own_source: "",
    }
}

const fn from(
    region: &'static str,
    polity: &'static str,
    first_day: Rd,
    old_calendar: &'static str,
    scope: Scope,
    source: &'static str,
) -> RegionalAdoption {
    RegionalAdoption {
        region,
        polity,
        step: Step::From(first_day),
        old_calendar,
        new_calendar: "gregory",
        scope,
        own_source: source,
    }
}

const SWEDEN_1699: &str = "Sweden's resolution of November 1699 to omit the eleven Julian leap days of 1700–1740 one by one, of which only the first, 29 February 1700, was omitted; not read, the dates from secondary sources [hogman-tiderakning, wikipedia-swedish-calendar]";
const SWEDEN_1711: &str = "Charles XII's order of January 1711 returning to the Julian calendar by a 30 February 1712; not read, the dates from secondary sources [hogman-tiderakning, wikipedia-swedish-calendar]";
const NETHERLANDS_1700: &str = "No instrument read; the provinces' dates from a secondary source [nlwiki-gregoriaanse-kalender]";
const KOREA_1895: &str = "King Gojong's edict in the Official Gazette (관보) of 개국 504년 9월 9일 (lunar, 1895), making lunar 개국 504년 11월 17일 the first day of 1896, not read; the dates from Korean Wikipedia, 태양력 and 건양, retrieved 2026-09-26 [kowiki-taeyangryeok, kowiki-geonyang]";

/// The steps, grouped by country in code order and in date order within a
/// country.
pub const REGIONAL_ADOPTIONS: &[RegionalAdoption] = &[
    reform("BG", "Bulgaria", "julian-gregorian-bg", Scope::Civil),
    from(
        "CN",
        "Republic of China, the provisional government at Nanjing",
        day(1912, 1, 1),
        "chinese",
        Scope::Partial,
        "Sun Yat-sen's order to use the solar calendar (改用阳历令) of 1 January 1912 and his circular telegram of 2 January making 黄帝纪元4609年11月13日 the first day of the Republic's first year, not read; the dates from Chinese Wikipedia, 民国纪年 and 公历, retrieved 2026-09-26 [zhwiki-minguo-jinian, zhwiki-gongli]. Partial: the Qing still governed the north until 12 February 1912, and the lunisolar calendar stayed in popular and much official use",
    ),
    from(
        "CN",
        "Republic of China, the Nationalist Government",
        day(1929, 1, 1),
        "chinese",
        Scope::Civil,
        "The Nationalist Government's resolution of 10 October 1928 that the Gregorian calendar be used throughout the country from 1 January 1929, not read; the dates from Chinese Wikipedia, 公历, retrieved 2026-09-26 [zhwiki-gongli]",
    ),
    reform(
        "DE",
        "Catholic Germany (Bavaria)",
        "julian-gregorian-de-catholic",
        Scope::Partial,
    ),
    reform(
        "DE",
        "Protestant Germany",
        "julian-gregorian-de-protestant",
        Scope::Partial,
    ),
    reform(
        "DK",
        "Denmark–Norway",
        "julian-gregorian-de-protestant",
        Scope::Civil,
    ),
    reform("ES", "Spain", "julian-gregorian-catholic", Scope::Civil),
    RegionalAdoption {
        region: "FI",
        polity: "Sweden, of which Finland was part",
        step: Step::From(swedish::EARLIEST),
        old_calendar: "julian",
        new_calendar: swedish::ID,
        scope: Scope::Civil,
        own_source: SWEDEN_1699,
    },
    RegionalAdoption {
        region: "FI",
        polity: "Sweden, of which Finland was part",
        step: Step::From(Rd(swedish::LATEST.0 + 1)),
        old_calendar: swedish::ID,
        new_calendar: "julian",
        scope: Scope::Civil,
        own_source: SWEDEN_1711,
    },
    reform(
        "FI",
        "Sweden, of which Finland was part",
        "julian-gregorian-se",
        Scope::Civil,
    ),
    reform("FR", "France", "julian-gregorian-fr", Scope::Civil),
    reform(
        "GB",
        "Great Britain and its colonies",
        "julian-gregorian-gb",
        Scope::Civil,
    ),
    reform("GR", "Greece", "julian-gregorian-gr", Scope::Civil),
    reform("HU", "Hungary", "julian-gregorian-hu", Scope::Civil),
    from(
        "JP",
        "Japan",
        day(1873, 1, 1),
        "japanese-tenpo",
        Scope::Civil,
        "Dajōkan Proclamation No. 337 of Meiji 5 (明治5年太政官布告第337号), 明治5年11月9日 (9 December 1872), making Meiji 5, twelfth month, third day 1 January 1873; the proclamation not read, its terms from the National Astronomical Observatory of Japan's 暦Wiki and Japanese Wikipedia, 明治改暦 [nao-rekiwiki-meiji, wikipedia-ja-meiji-kaireki]",
    ),
    from(
        "KP",
        "Joseon",
        day(1896, 1, 1),
        "dangi",
        Scope::Civil,
        KOREA_1895,
    ),
    from(
        "KR",
        "Joseon",
        day(1896, 1, 1),
        "dangi",
        Scope::Civil,
        KOREA_1895,
    ),
    reform(
        "NL",
        "Zeeland and the southern Netherlands",
        "julian-gregorian-nl",
        Scope::Partial,
    ),
    reform(
        "NL",
        "Holland",
        "julian-gregorian-nl-holland",
        Scope::Partial,
    ),
    RegionalAdoption {
        polity: "Gelderland",
        step: Step::From(day(1700, 7, 12)),
        own_source: NETHERLANDS_1700,
        ..reform("NL", "", "", Scope::Partial)
    },
    RegionalAdoption {
        polity: "Utrecht and Overijssel",
        step: Step::From(day(1700, 12, 12)),
        own_source: NETHERLANDS_1700,
        ..reform("NL", "", "", Scope::Partial)
    },
    RegionalAdoption {
        polity: "Friesland and Groningen",
        step: Step::From(day(1701, 1, 12)),
        own_source: NETHERLANDS_1700,
        ..reform("NL", "", "", Scope::Partial)
    },
    RegionalAdoption {
        polity: "Drenthe",
        step: Step::From(day(1701, 5, 12)),
        own_source: NETHERLANDS_1700,
        ..reform("NL", "", "", Scope::Partial)
    },
    reform(
        "NO",
        "Denmark–Norway",
        "julian-gregorian-de-protestant",
        Scope::Civil,
    ),
    reform("PL", "Poland", "julian-gregorian-catholic", Scope::Civil),
    reform("PT", "Portugal", "julian-gregorian-catholic", Scope::Civil),
    reform(
        "RO",
        "Romania (the Old Kingdom)",
        "julian-gregorian-ro",
        Scope::Civil,
    ),
    reform(
        "RS",
        "Kingdom of Serbs, Croats and Slovenes",
        "julian-gregorian-rs",
        Scope::Civil,
    ),
    reform("RU", "Soviet Russia", "julian-gregorian-ru", Scope::Civil),
    from(
        "SA",
        "Saudi Arabia, the pay of the public sector",
        day(2016, 10, 1),
        "islamic-umalqura",
        Scope::Partial,
        "A decision of the Council of Ministers of September 2016 paying public-sector staff by the Gregorian calendar from 1 October 2016, not read; the date from English Wikipedia, \"Adoption of the Gregorian calendar\", retrieved 2026-09-26, citing The Economist of 17 December 2016 and The Independent of 3 October 2016, neither read [wikipedia-adoption-gregorian]. English Wikipedia's \"Islamic calendar\" gives 14 February 2016 instead [wikipedia-islamic-calendar]. The Umm al-Qura calendar remains in use for every other purpose",
    ),
    RegionalAdoption {
        region: "SE",
        polity: "Sweden",
        step: Step::From(swedish::EARLIEST),
        old_calendar: "julian",
        new_calendar: swedish::ID,
        scope: Scope::Civil,
        own_source: SWEDEN_1699,
    },
    RegionalAdoption {
        region: "SE",
        polity: "Sweden",
        step: Step::From(Rd(swedish::LATEST.0 + 1)),
        old_calendar: swedish::ID,
        new_calendar: "julian",
        scope: Scope::Civil,
        own_source: SWEDEN_1711,
    },
    reform("SE", "Sweden", "julian-gregorian-se", Scope::Civil),
    from(
        "TR",
        "Ottoman Empire",
        rumi::CUTOVER,
        "rumi",
        Scope::Partial,
        "An Ottoman law of 1917 moving the Rumi calendar to the Gregorian days, whose number and date were not found: 15 Şubat 1332 (28 February 1917) was followed by 1 Mart 1333 (1 March 1917) and the Rumi year kept; the dates from English Wikipedia, \"Rumi calendar\", retrieved 2026-09-26, citing Revue du monde musulman 43 (1921), p. 47, not read [wikipedia-rumi-calendar]. Partial: the days changed and the year did not",
    ),
    from(
        "TR",
        "Republic of Turkey",
        Rd(rumi::LATEST.0 + 1),
        "rumi",
        Scope::Civil,
        "Law No. 698 of 26 December 1925, in force from 1 January 1926, replacing the Rumi and Hijri years by the Gregorian, not read; the dates from a secondary source [trwiki-miladi-takvim]",
    ),
];

/// Whether two ISO 3166-1 alpha-2 codes are the same, ignoring ASCII case.
fn same_region(one: &str, other: &str) -> bool {
    one.eq_ignore_ascii_case(other)
}

/// The steps of one country's adoption, oldest first: none for a code the
/// table does not know.
pub fn gregorian_adoption(region: &str) -> impl Iterator<Item = &'static RegionalAdoption> + '_ {
    REGIONAL_ADOPTIONS
        .iter()
        .filter(move |row| same_region(row.region, region))
}

#[cfg(all(test, feature = "alloc"))]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    fn fixed(year: i64, month: u8, day: u8) -> i64 {
        gregorian::to_fixed(year, month, day).map_or(i64::MIN, |rd| rd.0)
    }

    fn steps(region: &str) -> impl Iterator<Item = (i64, i64)> + '_ {
        gregorian_adoption(region).map(|row| {
            (
                row.last_old_day().map_or(i64::MIN, |rd| rd.0),
                row.first_day().map_or(i64::MIN, |rd| rd.0),
            )
        })
    }

    #[test]
    fn every_row_resolves_and_names_its_source() {
        for row in REGIONAL_ADOPTIONS {
            let first = row.first_day().unwrap();
            assert!(first.0 > fixed(1500, 1, 1), "{row:?}");
            assert_eq!(row.last_old_day().unwrap().0, first.0 - 1);
            assert!(!row.source().is_empty(), "{row:?}");
            assert!(!row.polity.is_empty(), "{row:?}");
            assert_eq!(row.region.len(), 2);
            assert!(row.region.bytes().all(|b| b.is_ascii_uppercase()));
            assert_ne!(row.old_calendar, row.new_calendar);
        }
    }

    #[test]
    fn the_table_is_grouped_by_country_and_in_date_order() {
        for pair in REGIONAL_ADOPTIONS.windows(2) {
            let (one, other) = (&pair[0], &pair[1]);
            assert!(one.region <= other.region, "{one:?} {other:?}");
            if one.region == other.region {
                assert!(one.first_day().unwrap() < other.first_day().unwrap());
            }
        }
    }

    #[test]
    fn the_julian_rows_are_the_tabulated_cutovers() {
        let britain: Vec<_> = steps("GB").collect();
        assert_eq!(britain, [(fixed(1752, 9, 13), fixed(1752, 9, 14))]);
        // Wednesday 2 September 1752 Old Style is the day before.
        assert_eq!(britain[0].0, crate::julian::to_fixed(1752, 9, 2).unwrap().0);
        let russia: Vec<_> = steps("ru").collect();
        assert_eq!(russia, [(fixed(1918, 2, 13), fixed(1918, 2, 14))]);
        let greece: Vec<_> = steps("GR").collect();
        assert_eq!(greece, [(fixed(1923, 2, 28), fixed(1923, 3, 1))]);
    }

    #[test]
    fn every_cutover_is_the_step_of_a_row() {
        for adoption in julian_gregorian::ADOPTIONS {
            assert!(
                REGIONAL_ADOPTIONS
                    .iter()
                    .any(|row| row.step == Step::Reform(adoption.id)),
                "{}",
                adoption.id
            );
        }
    }

    #[test]
    fn serbia_has_its_own_date() {
        let serbia: Vec<_> = steps("RS").collect();
        assert_eq!(serbia, [(fixed(1919, 1, 27), fixed(1919, 1, 28))]);
        assert_eq!(serbia[0].1, 700_562);
        assert_eq!(serbia[0].0, crate::julian::to_fixed(1919, 1, 14).unwrap().0);
        let row = gregorian_adoption("RS").next().unwrap();
        assert_eq!(row.step, Step::Reform("julian-gregorian-rs"));
        assert_eq!(
            row.source(),
            julian_gregorian::adoption_by_id("julian-gregorian-rs")
                .unwrap()
                .source
        );
        let romania: Vec<_> = steps("RO").collect();
        assert_eq!(romania, [(fixed(1919, 4, 13), fixed(1919, 4, 14))]);
    }

    #[test]
    fn sweden_went_there_and_back_before_it_went_for_good() {
        let rows: Vec<_> = gregorian_adoption("SE").collect();
        let calendars: Vec<_> = rows
            .iter()
            .map(|row| (row.old_calendar, row.new_calendar))
            .collect();
        assert_eq!(
            calendars,
            [
                ("julian", "swedish-1700"),
                ("swedish-1700", "julian"),
                ("julian", "gregory")
            ]
        );
        // 28 February 1700 Julian, then 1 March 1700 Swedish.
        assert_eq!(
            rows[0].last_old_day().unwrap(),
            crate::julian::to_fixed(1700, 2, 28).unwrap()
        );
        assert_eq!(
            rows[0].first_day().unwrap(),
            swedish::to_fixed(1700, 3, 1).unwrap()
        );
        // 30 February 1712 Swedish, then 1 March 1712 Julian.
        assert_eq!(
            rows[1].last_old_day().unwrap(),
            swedish::to_fixed(1712, 2, 30).unwrap()
        );
        assert_eq!(
            rows[1].first_day().unwrap(),
            crate::julian::to_fixed(1712, 3, 1).unwrap()
        );
        assert_eq!(
            rows[2].first_day().unwrap(),
            gregorian::to_fixed(1753, 3, 1).unwrap()
        );
        let finland: Vec<_> = steps("FI").collect();
        assert_eq!(finland, steps("SE").collect::<Vec<_>>());
    }

    #[test]
    fn the_dutch_provinces_are_partial_steps_of_one_country() {
        let rows: Vec<_> = gregorian_adoption("NL").collect();
        assert_eq!(rows.len(), 6);
        assert!(rows.iter().all(|row| row.scope == Scope::Partial));
        // Zeeland on 25 December 1582, Holland on 12 January 1583, both
        // the reform table's.
        assert_eq!(rows[0].step, Step::Reform("julian-gregorian-nl"));
        assert_eq!(rows[0].first_day().unwrap().0, fixed(1582, 12, 25));
        assert_eq!(rows[1].polity, "Holland");
        assert_eq!(rows[1].step, Step::Reform("julian-gregorian-nl-holland"));
        assert_eq!(
            rows[1].last_old_day().unwrap(),
            crate::julian::to_fixed(1583, 1, 1).unwrap()
        );
        assert_eq!(rows[2].polity, "Gelderland");
        assert_eq!(
            rows[2].last_old_day().unwrap(),
            crate::julian::to_fixed(1700, 6, 30).unwrap()
        );
    }

    #[test]
    fn turkey_moved_the_days_first_and_the_year_later() {
        let rows: Vec<_> = gregorian_adoption("TR").collect();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].first_day().unwrap().0, fixed(1917, 3, 1));
        assert_eq!(rows[0].scope, Scope::Partial);
        assert_eq!(rows[1].first_day().unwrap().0, fixed(1926, 1, 1));
        assert_eq!(rows[1].scope, Scope::Civil);
        assert!(rows.iter().all(|row| row.old_calendar == "rumi"));
    }

    #[test]
    fn a_region_the_table_does_not_know_has_no_rows() {
        assert_eq!(gregorian_adoption("ZZ").count(), 0);
        assert_eq!(gregorian_adoption("").count(), 0);
        assert_eq!(gregorian_adoption("GBR").count(), 0);
    }

    #[test]
    fn the_scopes_are_named() {
        assert_eq!(Scope::Civil.as_str(), "civil");
        assert_eq!(Scope::Ecclesiastical.as_str(), "ecclesiastical");
        assert_eq!(Scope::Partial.as_str(), "partial");
    }
}
