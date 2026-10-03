//! The first year each table of the Americas and Europe answers for (ADR
//! 0013): before it the engine reports a gap or, where the day
//! was not yet established, leaves it out, and never answers a year no
//! source read covers.
//!
//! Each row gives the table's first supported year and what that year is:
//! the first whole year of the earliest instrument read, or the year of the
//! only list read. The reasons are in each table's doc comment and `sources`.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::countries;
use hc_holiday::engine::HolidayCalendar;
use hc_holiday::rule::RuleSet;

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

fn table(code: &str) -> &'static RuleSet {
    match countries::by_code(code) {
        Some(country) => country,
        None => panic!("{code} is not a registered country"),
    }
}

/// `(code, first supported year)` of the tables whose nationwide days are
/// read from one year.
const FIRST_YEARS: &[(&str, i64)] = &[
    // The Americas.
    ("AD", 2024), // the work calendars of 2024 to 2026
    ("AG", 2006), // the Amendment Act 2005, from its first full year
    ("AR", 2011), // Decreto 1584/2010
    ("BB", 1998), // Cap. 352 (L.R.O. 1998)
    ("BO", 2017), // Decreto Supremo 2750 of 1 May 2016, first whole year
    ("BR", 2003), // Lei 10.607 of 19 December 2002
    ("BS", 1973), // the Schedule, whose earliest dated entry is Independence Day
    ("BZ", 2020), // the Government's notices from 2020
    ("CL", 1981), // the first change the laws read date
    ("CO", 1984), // Ley 51 de 1983, first full year
    ("CR", 2020), // Ley 9803 (2020)
    ("CU", 2015), // Law 116, in force from 17 June 2014
    ("DM", 2021), // the Government's lists from 2021
    ("DO", 1998), // Ley 139-97, in force from 27 June 1997
    ("EC", 2017), // the law of Registro Oficial 906 of 20 December 2016
    ("GD", 2017), // the Amendment Act No. 2 of 2017
    ("GT", 2018), // Decreto 19-2018
    ("GY", 2012), // L.R.O. 1/2012
    ("HN", 1959), // Decreto 189 of 1959
    ("HT", 1985), // the Code du travail of 24 February 1984
    ("JM", 1961), // Labour Day replaced Empire Day
    ("KN", 2003), // the revised edition to 31 December 2002
    ("LC", 2005), // amended to Act 5 of 2004
    ("NI", 1997), // Ley 185 of 30 October 1996
    ("PA", 2008), // Ley 70 of 28 December 2007
    ("PE", 2026), // the only list read
    ("PY", 1990), // Ley 8/90
    ("SR", 2007), // S.B. 2007 no. 98
    ("SV", 1995), // Decreto Legislativo 408 of 1995
    ("TT", 1996), // the latest dated change of the Schedule
    ("UY", 1997), // Ley 16.805 of 24 December 1996
    ("VC", 2019), // the earliest list read
    ("VE", 1972), // the Ley de Fiestas Nacionales of 1971; the LOTTT days from 2013
    ("MX", 2006), // Ley Federal del Trabajo art. 74 as reformed in 2006
    ("ES", 1990), // article 45 as read, in force from 8 November 1989
    ("CH", 2000), // the federal day (Arbeitsgesetz art. 20a); the other three days nationwide from 2023
    // Europe.
    ("AL", 1993), // Law 7651 of 21 December 1992
    ("AT", 1968), // Feiertagsruhegesetz as amended in 1967, first whole year
    ("BE", 1975), // the royal decree of 18 April 1974
    ("BG", 2017), // Labour Code art. 154 as amended by SG 105/2016
    ("BY", 1991), // Independence Day of 1991, the earliest dated year
    ("CY", 2026), // the only list read
    ("CZ", 2001), // Act 245/2000
    ("DE", 1990), // the Einigungsvertrag
    ("DK", 2024), // Lov nr. 214 of 2023, in force 1 January 2024
    ("EE", 1994), // the act of 8 February 1994
    ("FI", 2026), // the only list read
    ("FR", 2017), // L3133-1 in force from 10 August 2016
    ("GB", 1971), // the Banking and Financial Dealings Act 1971
    ("GR", 2022), // Law 4808/2021
    ("HR", 2002), // NN 136/2002
    ("HU", 2013), // the Labour Code of 2012, in force from 1 July 2012
    ("IE", 1998), // the Organisation of Working Time Act 1997
    ("IS", 1998), // lög nr. 32/1997
    ("IT", 1949), // legge 260/1949
    ("LI", 1986), // LGBl. 1986 Nr. 85
    ("LT", 1990), // the Law on Holidays of 1990
    ("LU", 2019), // the law of 25 April 2019
    ("LV", 1995), // the earliest year the amending laws date
    ("MC", 1966), // law 798 of 18 February 1966
    ("MD", 2009), // the earliest dated change
    ("ME", 2007), // the Law on State and Other Holidays of 2007
    ("MK", 2007), // the amendment of 2007
    ("MT", 2026), // the Act as read, undated
    ("NL", 2011), // the Algemene termijnenwet, in force from 10 October 2010
    ("NO", 1995), // LOV-1995-02-24-12
    ("PL", 2011), // the amendment of 2010, from 2011
    ("PT", 2016), // Lei 8/2016
    ("RO", 2012), // Legea 147/2012
    ("RS", 2002), // the Law of 2001, first whole year
    ("RU", 1991), // the earliest year Wikipedia dates a change
    ("SE", 1989), // lag (1989:253)
    ("SI", 1992), // ZPDPD, Uradni list RS 26/91
    ("SK", 2021), // the earliest dated change
    ("UA", 2015), // law 238-VIII
    ("VA", 2011), // the Governorate's Regulation of 21 November 2010
    ("SM", 2014), // law 152/2013 for the civil days; the religious days from 2025
];

/// The tables with no nationwide set: their days are each entity's, so they
/// are asked for one region.
const REGION_FIRST_YEARS: &[(&str, &str, i64)] = &[
    ("BA", "BA-BIH", 1995), // the laws of 1995 for 1 March and 25 November
    ("BA", "BA-SRP", 2007), // the Law on Holidays, 43/07
    ("BA", "BA-BRC", 2002), // the Law on Holidays of the District, 19/02
];

fn entries_and_gaps(country: &RuleSet, region: Option<&str>, year: i64) -> (usize, usize) {
    let calendar = HolidayCalendar::for_year(country, region, year);
    let entries = calendar.in_year(year).len();
    let gaps = calendar
        .gaps()
        .iter()
        .filter(|gap| gap.year == year)
        .count();
    (entries, gaps)
}

#[test]
fn every_table_answers_from_its_first_year_and_not_before() {
    for &(code, first) in FIRST_YEARS {
        let country = table(code);
        let (entries, _) = entries_and_gaps(country, None, first);
        assert!(
            entries > 0,
            "{code} {first}: the first supported year is empty"
        );
        // The year before: nothing is answered, and the sources not
        // having been read is said as a gap, unless every rule that
        // applies in it began later, which is an answer.
        for before in [first - 1, first - 10, first - 100, 1500] {
            let (entries, gaps) = entries_and_gaps(country, None, before);
            assert_eq!(entries, 0, "{code} {before}: answered before {first}");
            if before == first - 1 && !matches!(code, "BO" | "SM") {
                assert!(gaps > 0 || first <= 1500, "{code} {before}: not a gap");
            }
        }
    }
}

#[test]
fn a_country_with_no_nationwide_days_is_read_from_each_entity_s_first_year() {
    for &(code, region, first) in REGION_FIRST_YEARS {
        let country = table(code);
        let (entries, _) = entries_and_gaps(country, Some(region), first);
        assert!(entries > 0, "{code} {region} {first}: empty");
        for before in [first - 1, first - 20, 1500] {
            let (entries, gaps) = entries_and_gaps(country, Some(region), before);
            assert_eq!(
                entries, 0,
                "{code} {region} {before}: answered before {first}"
            );
            if before == first - 1 {
                assert!(gaps > 0, "{code} {region} {before}: not a gap");
            }
        }
    }
}

#[test]
fn san_marino_reads_its_civil_days_from_2014_and_its_religious_days_from_2025() {
    let sm = table("SM");
    let on = |year: i64, month: u8, day: u8| {
        HolidayCalendar::for_year(sm, None, year)
            .on(ymd(year, month, day))
            .len()
    };
    // 1 May and 25 March are law 152 of 2013's; Epiphany is the bank's
    // calendar of 2025's.
    assert_eq!(on(2014, 5, 1), 1);
    assert_eq!(on(2014, 3, 25), 1);
    assert_eq!(on(2014, 1, 6), 0);
    assert_eq!(on(2013, 5, 1), 0);
    assert_eq!(on(2025, 1, 6), 1);
    let gaps = HolidayCalendar::for_year(sm, None, 2014)
        .gaps()
        .iter()
        .filter(|gap| gap.year == 2014)
        .map(|gap| gap.name)
        .collect::<Vec<_>>();
    assert!(gaps.contains(&"Epiphany"));
    assert!(!gaps.contains(&"Labour Day"));
}

#[test]
fn bolivia_reads_decreto_supremo_5521_for_2026() {
    let bo = table("BO");
    let calendar = HolidayCalendar::for_year(bo, None, 2026);
    let names_on = |month: u8, day: u8| {
        calendar
            .on(ymd(2026, month, day))
            .iter()
            .map(|holiday| holiday.name)
            .collect::<Vec<_>>()
    };
    // Article 4: Thursday 22 January moves to Friday 23 January and Sunday
    // 21 June to Monday 22 June, so neither original is a holiday.
    assert!(names_on(1, 22).is_empty());
    assert_eq!(names_on(1, 23), ["Plurinational State Foundation Day"]);
    assert!(names_on(6, 21).is_empty());
    assert_eq!(names_on(6, 22), ["Aymara Amazonian New Year"]);
    // Article 3: Friday 5 June after Corpus Christi and Friday 7 August
    // after Independence Day.
    assert_eq!(names_on(6, 4), ["Corpus Christi"]);
    assert_eq!(names_on(6, 5), ["Additional holiday after Corpus Christi"]);
    assert_eq!(names_on(8, 6), ["Independence Day"]);
    assert_eq!(
        names_on(8, 7),
        ["Additional holiday after Independence Day"]
    );
    // The decree is cited, and 2026's own decree leaves no gap.
    assert!(calendar.gaps().iter().all(|gap| gap.year != 2026));
    // Every other year's decree was not read: a gap, 2025 and 2027 among them.
    for year in [2018, 2025, 2027, 2030] {
        let gaps = HolidayCalendar::for_year(bo, None, year);
        assert!(
            gaps.gaps()
                .iter()
                .any(|gap| gap.year == year && gap.source.contains("only the decree for 2026")),
            "BO {year}: no gap for the year's decree"
        );
    }
    // 22 January and 21 June of 2027 are the plain dates again.
    let c2027 = HolidayCalendar::for_year(bo, None, 2027);
    assert!(c2027.is_holiday(ymd(2027, 1, 22)));
    assert!(c2027.is_holiday(ymd(2027, 6, 21)) || c2027.is_holiday(ymd(2027, 6, 22)));
}

#[test]
fn argentina_reports_2017_and_guemes_before_2018_as_gaps() {
    let ar = table("AR");
    for year in [2016, 2017] {
        let gaps = HolidayCalendar::for_year(ar, None, year);
        assert!(
            gaps.gaps().iter().any(|gap| gap.local_name
                == "Paso a la Inmortalidad del General Martín Miguel de Güemes"),
            "AR {year}: Güemes's day should be a gap"
        );
    }
    let gaps = HolidayCalendar::for_year(ar, None, 2017);
    assert!(
        gaps.gaps()
            .iter()
            .any(|gap| gap.local_name == "Feriados trasladables de 2017")
    );
    // Decreto 1584/2010 answers 2011 to 2016, and Ley 27.399 2018 on.
    let c2016 = HolidayCalendar::for_year(ar, None, 2016);
    assert!(c2016.is_holiday(ymd(2016, 8, 15)));
    let c2018 = HolidayCalendar::for_year(ar, None, 2018);
    assert!(c2018.is_holiday(ymd(2018, 8, 20)));
}

#[test]
fn saint_vincent_reports_2020_as_a_gap() {
    let vc = table("VC");
    let gaps = HolidayCalendar::for_year(vc, None, 2020);
    assert!(
        gaps.gaps()
            .iter()
            .any(|gap| gap.year == 2020 && gap.name == "The days of 2020")
    );
}

#[test]
fn venezuela_reads_the_ley_de_fiestas_nacionales_from_1972() {
    let ve = table("VE");
    // The Ley de Fiestas Nacionales, Gaceta Oficial 29.541 of 22 June 1971,
    // article 1: 19 April, 24 June, 5 July, 24 July and 12 October.
    let c1990 = HolidayCalendar::for_year(ve, None, 1990);
    for (month, day) in [(4, 19), (6, 24), (7, 5), (7, 24), (10, 12)] {
        assert!(
            c1990.is_holiday(ymd(1990, month, day)),
            "VE 1990-{month}-{day}"
        );
    }
    // The labour law's days are the LOTTT's, from 2013.
    assert!(!c1990.is_holiday(ymd(1990, 12, 24)));
    assert!(
        c1990
            .gaps()
            .iter()
            .any(|gap| gap.year == 1990 && gap.name == "Christmas Eve")
    );
    let c1971 = HolidayCalendar::for_year(ve, None, 1971);
    assert!(!c1971.is_holiday(ymd(1971, 7, 5)));
}

#[test]
fn a_bilingual_canton_gives_its_own_language_and_never_german_where_not_read() {
    let ch = table("CH");
    let local = |region: &str, month: u8, day: u8, name: &str| {
        HolidayCalendar::for_year(ch, Some(region), 2026)
            .all()
            .iter()
            .filter(|holiday| holiday.name == name && holiday.date == ymd(2026, month, day))
            .map(|holiday| holiday.local_name)
            .collect::<Vec<_>>()
    };
    // Geneva (art. 1 of the Loi sur les jours fériés): "1er Janvier",
    // "Ascension", "Noël"; Neuchâtel's, Jura's and Ticino's likewise.
    assert_eq!(local("CH-GE", 1, 1, "New Year's Day"), ["1er Janvier"]);
    assert_eq!(local("CH-GE", 12, 25, "Christmas Day"), ["Noël"]);
    assert_eq!(local("CH-JU", 1, 1, "New Year's Day"), ["Nouvel-An"]);
    assert_eq!(local("CH-NE", 5, 14, "Ascension"), ["Ascension"]);
    assert_eq!(local("CH-TI", 1, 1, "New Year's Day"), ["Capodanno"]);
    assert_eq!(local("CH-TI", 5, 14, "Ascension"), ["Ascensione"]);
    assert_eq!(local("CH-TI", 12, 25, "Christmas Day"), ["Natale"]);
    // Fribourg, Vaud and Valais: the French text of these three days was not
    // read, so the English name stands and not the German.
    for region in ["CH-FR", "CH-VD", "CH-VS"] {
        assert_eq!(local(region, 1, 1, "New Year's Day"), [""], "{region}");
        assert_eq!(local(region, 12, 25, "Christmas Day"), [""], "{region}");
    }
    // The German cantons keep theirs.
    assert_eq!(local("CH-ZH", 1, 1, "New Year's Day"), ["Neujahrstag"]);
}
