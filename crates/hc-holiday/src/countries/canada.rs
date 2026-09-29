//! Canada.

use hc_calendar::Weekday;

use crate::computus::offsets::{EASTER_MONDAY, GOOD_FRIDAY};
use crate::rule::{
    HolidayRule, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate, Subdivisions, SubstituteDirection,
    SubstitutionPolicy,
};

// The provinces and territories with a day of their own, by ISO 3166-2
// code, and the statute or page each was read in.
const CA_AB: &[&str] = &["CA-AB"];
const CA_BC: &[&str] = &["CA-BC"];
const CA_MB: &[&str] = &["CA-MB"];
const CA_NB: &[&str] = &["CA-NB"];
const CA_NL: &[&str] = &["CA-NL"];
const CA_NS: &[&str] = &["CA-NS"];
const CA_NT: &[&str] = &["CA-NT"];
const CA_NU: &[&str] = &["CA-NU"];
const CA_ON: &[&str] = &["CA-ON"];
const CA_PE: &[&str] = &["CA-PE"];
const CA_QC: &[&str] = &["CA-QC"];
const CA_SK: &[&str] = &["CA-SK"];
const CA_YT: &[&str] = &["CA-YT"];

const AB_SOURCE: &str = "Government of Alberta, \"Employment standards – Alberta general holidays\" \
     (alberta.ca/alberta-general-holidays, secondary: the Employment Standards Code, RSA 2000, \
     c E-9, is PDF only), retrieved 2026-09-29; the first year from Wikipedia, \"Family Day \
     (Canada)\" (secondary)";
const BC_FAMILY_DAY: &str = "Family Day Act, SBC 2012, c 24, and Family Day Regulation, B.C. Reg. \
     149/2012, s 1, before and after B.C. Reg. 75/2018 (bclaws.gov.bc.ca, point in time), \
     retrieved 2026-09-29";
const BC_DAY: &str = "British Columbia Day Act, RSBC 1996, c 34, s 1, and Employment Standards Act, \
     RSBC 1996, c 113, s 1, current to 2026-09-22 (bclaws.gov.bc.ca), retrieved 2026-09-29";
const BC_LIST: &str = "Employment Standards Act, RSBC 1996, c 113, s 1 \"statutory holiday\": \
     New Year's Day, Family Day, Good Friday, Victoria Day, Canada Day, British Columbia Day, \
     Labour Day, National Day for Truth and Reconciliation, Thanksgiving Day, Remembrance Day and \
     Christmas Day, current to 2026-09-22 (bclaws.gov.bc.ca), retrieved 2026-09-29";
const MB_LIST: &str = "The Employment Standards Code, C.C.S.M. c. E110, s 21(1) \"general holiday\": \
     New Year's Day, Louis Riel Day, Good Friday, Victoria Day, July 1, Labour Day, Orange Shirt \
     Day (National Day for Truth and Reconciliation), Thanksgiving Day and Christmas Day, current \
     to 2026-09-25 (web2.gov.mb.ca), retrieved 2026-09-29";
const MB_SOURCE: &str = "The Employment Standards Code, C.C.S.M. c. E110, s 21(1)(a.1), added by The \
     Statutory Holidays Act (Various Acts Amended), S.M. 2007, c. 18, in force 8 November 2007 \
     (web2.gov.mb.ca), retrieved 2026-09-29";
const NB_FAMILY_DAY: &str = "An Act Respecting Family Day (Bill 67, 58th Legislature), amending the \
     Employment Standards Act, SNB 1982, c E-7.2, s 1, in force 1 January 2018 (legnb.ca), \
     retrieved 2026-09-29";
const NB_DAY: &str = "Employment Standards Act, SNB 1982, c E-7.2, s 1 \"public holiday\": New \
     Year's Day, Family Day, Good Friday, Canada Day, New Brunswick Day, Labour Day, Remembrance \
     Day and Christmas Day, consolidated to 12 June 2026, the Internet Archive's capture of \
     laws.gnb.ca (the site refused the connection), retrieved 2026-09-29; the date from \
     canada-holidays.ca (secondary)";
const NL_SOURCE: &str = "Labour Standards Act, RSNL 1990, c L-2, s 14(1) \"public holiday\": New \
     Year's Day, Good Friday, Remembrance Day, Memorial Day, Labour Day and Christmas Day, s 14 as \
     1977 c52 and 2001 c33 left it (assembly.nl.ca, amended to 2024 c35), retrieved 2026-09-29";
const NS_SOURCE: &str = "Labour Standards Code, RSNS 1989, c 246, s 2(ga) \"general holiday\": New \
     Year's Day, Good Friday, Canada Day, Labour Day, Christmas Day and the third Monday in \
     February, CanLII's copy current to 1 February 2022 in the Internet Archive's capture \
     (secondary), and Nova Scotia's \"Holiday and Retail Closing Day Charts\" for 2025 and 2026 \
     (novascotia.ca/lae/employmentrights), retrieved 2026-09-29; Heritage Day in force 1 January \
     2015, from \"Changes to the Labour Standards Code\" (novascotia.ca)";
const NT_SOURCE: &str = "Employment Standards Act, SNWT 2007, c 13, s 22, not read (PDF only); the \
     Department of Education, Culture and Employment's \"Frequently Asked Questions\" \
     (ece.gov.nt.ca), whose 11 statutory holidays are the federal days but Boxing Day, \"National \
     Aboriginal Day\" and the \"First Monday in August\", and the Government's news release of \
     13 July 2022 for the National Day for Truth and Reconciliation from 2022, retrieved \
     2026-09-29; National Indigenous Peoples Day's date and first year from Wikipedia \
     (secondary)";
const NU_SOURCE: &str = "Nunavut Labour Standards Compliance Office, Fact Sheet 10, \"General \
     Holidays\" (nu-lsco.ca, secondary: the Labour Standards Act was not reachable), retrieved \
     2026-09-29; Nunavut Day's date and first year from Wikipedia, \"Nunavut Day\" (secondary)";
const ON_SOURCE: &str = "Employment Standards Act, 2000, S.O. 2000, c. 41, s 1(1) \"public holiday\", \
     Family Day added by 2007, c. 16, Sched. A, s 1, in force 3 December 2007 (ontario.ca/laws), \
     retrieved 2026-09-29";
const PE_SOURCE: &str = "Employment Standards Act, RSPEI 1988, c E-6.2, s 6(1) \"paid holiday\": New \
     Year's Day, Islander Day, Good Friday, Canada Day, Labour Day, National Day for Truth and \
     Reconciliation, Remembrance Day and Christmas Day, CanLII's copy current to 1 June 2022 in \
     the Internet Archive's capture (secondary), and the Government's page \"Paid Holidays\" of \
     6 June 2024, retrieved 2026-09-29; Islander Day's first year from Wikipedia (secondary)";
const QC_SOURCE: &str = "Loi sur la fête nationale, RLRQ c F-1.1, arts. 1 and 2 (1978, c. 5), \
     \"À jour au 11 décembre 2025\", the Internet Archive's capture of LégisQuébec (the site \
     refused the connection), retrieved 2026-09-29";
const SK_SOURCE: &str = "Government of Saskatchewan, \"List of Saskatchewan Public Holidays\" \
     (saskatchewan.ca, secondary: The Saskatchewan Employment Act, SS 2013, c S-15.1, is PDF \
     only), retrieved 2026-09-29; Family Day's first year from Wikipedia, \"Family Day \
     (Canada)\" (secondary)";
const QC_LNT: &str = "Loi sur les normes du travail, RLRQ c N-1.1, art. 60, \"À jour au 11 \
     décembre 2025\": 1 January, \"le Vendredi saint ou le lundi de Pâques, au choix de \
     l'employeur\", \"le lundi qui précède le 25 mai\", 1 July, the first Monday of September, the \
     second Monday of October and 25 December, the Internet Archive's capture of LégisQuébec, \
     retrieved 2026-09-29";
const QC_PATRIOTES: &str = "Loi sur les normes du travail, RLRQ c N-1.1, art. 60(3), and the \
     Premier's communiqué of 24 November 2002, \"Congé férié à l'occasion de la Journée nationale \
     des Patriotes\", for Décret 1322-2002 (PDF, not read): the day \"sera dorénavant célébrée le \
     lundi précédant immédiatement le 25 mai\", the Monday already off as the Fête de Dollard, the \
     Internet Archive's capture of premier-ministre.gouv.qc.ca, retrieved 2026-09-29";
const YT_SOURCE: &str = "Employment Standards Act, RSY 2002, c 72, s 1 \"general holiday\", in force \
     since 20 April 2023, CanLII's copy in the Internet Archive's capture (secondary), National \
     Indigenous Peoples Day added by SY 2017, c 1 and the National Day for Truth and \
     Reconciliation by SY 2022, c 18, assented 24 November 2022; and the Government's page \"Find \
     a Yukon statutory holiday\" of 29 January 2026, for the dates and the days that are not \
     holidays, Boxing Day among them, retrieved 2026-09-29";

/// A province's general holiday in `region`: a day off under its
/// employment-standards law, moved off a weekend as the federal ones are.
/// Its years are the call's: `.years` from the year a source gives as its
/// first, or `.read_from` the year read where none does.
const fn provincial(
    name: &'static str,
    local_name: &'static str,
    rule: Rule,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::public(name, local_name, rule)
        .in_regions(region)
        .cited(source)
}

/// A province's general holiday on a fixed date, in `region`, left where
/// it falls: no provincial weekend rule for it was read.
const fn provincial_fixed(
    name: &'static str,
    local_name: &'static str,
    month: u8,
    day: u8,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(name, local_name, Rule::gregorian(month, day))
        .in_regions(region)
        .cited(source)
}

/// A federal day that `region`'s law, in the text read in 2026, does not
/// keep: the federal rule excepts the region, and this says what the
/// earlier texts, not read, did — a gap in each year before 2026.
const fn not_kept(
    name: &'static str,
    local_name: &'static str,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::observance(name, local_name, Rule::NO_DAY)
        .in_regions(region)
        .cited(source)
        .read_from(READ)
}

/// The year the provinces' laws were read, and the first year answered
/// for a day no source read dates.
const READ: i32 = 2026;

/// The third Monday of February, the provinces' February day.
const THIRD_MONDAY_OF_FEBRUARY: Rule = Rule::nth(2, 3, Weekday::Monday);
/// The first Monday of August, the provinces' August day.
const FIRST_MONDAY_OF_AUGUST: Rule = Rule::nth(8, 1, Weekday::Monday);

// The provinces whose lists, in the texts read, leave out a federal day
// (docs/systems/canada-holidays.md). Newfoundland and Labrador's 1 July is
// its own Memorial Day.
const NO_GOOD_FRIDAY: &[&str] = &["CA-QC"];
const NO_VICTORIA_DAY: &[&str] = &["CA-NB", "CA-NL", "CA-NS", "CA-PE", "CA-QC"];
const NO_CANADA_DAY: &[&str] = &["CA-NL"];
const NO_TRUTH_AND_RECONCILIATION: &[&str] = &[
    "CA-AB", "CA-BC", "CA-MB", "CA-NB", "CA-NL", "CA-NS", "CA-NT", "CA-NU", "CA-ON", "CA-PE",
    "CA-QC", "CA-SK", "CA-YT",
];
const NO_THANKSGIVING: &[&str] = &["CA-NB", "CA-NL", "CA-NS", "CA-PE"];
const NO_REMEMBRANCE_DAY: &[&str] = &["CA-MB", "CA-NS", "CA-ON", "CA-QC"];
const NO_BOXING_DAY: &[&str] = &[
    "CA-AB", "CA-BC", "CA-MB", "CA-NB", "CA-NL", "CA-NS", "CA-NT", "CA-NU", "CA-PE", "CA-QC",
    "CA-SK", "CA-YT",
];

static CA_RULES: &[HolidayRule] = &[
    // ── The federal days, of federally regulated employers ──────────────
    HolidayRule::public("New Year's Day", "Jour de l'An", Rule::gregorian(1, 1)),
    HolidayRule::public("Good Friday", "Vendredi saint", Rule::easter(GOOD_FRIDAY))
        .except_in(NO_GOOD_FRIDAY),
    // Victoria Day is the Monday preceding 25 May.
    HolidayRule::public(
        "Victoria Day",
        "Fête de la Reine",
        Rule::WeekdayOnOrBefore {
            month: 5,
            day: 24,
            weekday: Weekday::Monday,
        },
    )
    .except_in(NO_VICTORIA_DAY),
    HolidayRule::public("Canada Day", "Fête du Canada", Rule::gregorian(7, 1))
        .except_in(NO_CANADA_DAY),
    HolidayRule::public(
        "Labour Day",
        "Fête du Travail",
        Rule::nth(9, 1, Weekday::Monday),
    ),
    HolidayRule::public(
        "National Day for Truth and Reconciliation",
        "Journée nationale de la vérité et de la réconciliation",
        Rule::gregorian(9, 30),
    )
    .years(Some(2021), None)
    .except_in(NO_TRUTH_AND_RECONCILIATION),
    HolidayRule::public(
        "Thanksgiving",
        "Action de grâce",
        Rule::nth(10, 2, Weekday::Monday),
    )
    .except_in(NO_THANKSGIVING),
    HolidayRule::public(
        "Remembrance Day",
        "Jour du Souvenir",
        Rule::gregorian(11, 11),
    )
    .except_in(NO_REMEMBRANCE_DAY),
    HolidayRule::public("Christmas Day", "Noël", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "Lendemain de Noël", Rule::gregorian(12, 26))
        .except_in(NO_BOXING_DAY),
    // ── The federal days a province's text read leaves out ─────────────
    not_kept("Victoria Day", "Fête de la Reine", CA_NB, NB_DAY),
    not_kept("Victoria Day", "", CA_NL, NL_SOURCE),
    not_kept("Victoria Day", "", CA_NS, NS_SOURCE),
    not_kept("Victoria Day", "", CA_PE, PE_SOURCE),
    // Quebec keeps Victoria Day's Monday as "le lundi qui précède le 25 mai",
    // named the Journée nationale des patriotes from 2003; since when
    // article 60 has had the Monday was not read.
    not_kept("Victoria Day", "Fête de la Reine", CA_QC, QC_LNT).read_from(2003),
    not_kept(
        "National Day for Truth and Reconciliation",
        "",
        CA_AB,
        AB_SOURCE,
    )
    .years(Some(2021), None),
    not_kept(
        "National Day for Truth and Reconciliation",
        "",
        CA_NL,
        NL_SOURCE,
    )
    .years(Some(2021), None),
    not_kept(
        "National Day for Truth and Reconciliation",
        "",
        CA_NU,
        NU_SOURCE,
    )
    .years(Some(2021), None),
    not_kept(
        "National Day for Truth and Reconciliation",
        "",
        CA_ON,
        ON_SOURCE,
    )
    .years(Some(2021), None),
    not_kept(
        "National Day for Truth and Reconciliation",
        "",
        CA_SK,
        SK_SOURCE,
    )
    .years(Some(2021), None),
    not_kept(
        "National Day for Truth and Reconciliation",
        "",
        CA_NB,
        NB_DAY,
    )
    .years(Some(2021), None),
    not_kept(
        "National Day for Truth and Reconciliation",
        "",
        CA_NS,
        NS_SOURCE,
    )
    .years(Some(2021), None),
    not_kept(
        "National Day for Truth and Reconciliation",
        "Journée nationale de la vérité et de la réconciliation",
        CA_QC,
        QC_LNT,
    )
    .years(Some(2021), None),
    not_kept("Thanksgiving", "Action de grâce", CA_NB, NB_DAY),
    not_kept("Thanksgiving", "", CA_NL, NL_SOURCE),
    not_kept("Thanksgiving", "", CA_NS, NS_SOURCE),
    not_kept("Thanksgiving", "", CA_PE, PE_SOURCE),
    not_kept("Remembrance Day", "jour du Souvenir", CA_MB, MB_LIST),
    not_kept("Remembrance Day", "", CA_ON, ON_SOURCE),
    not_kept("Remembrance Day", "Jour du Souvenir", CA_QC, QC_LNT),
    // Nova Scotia's Code does not keep it; its Remembrance Day Act, not
    // read, may give some employees the day: every year a gap.
    HolidayRule::public("Remembrance Day", "", Rule::UNREAD)
        .in_regions(CA_NS)
        .cited(NS_SOURCE),
    not_kept("Boxing Day", "", CA_AB, AB_SOURCE),
    not_kept("Boxing Day", "", CA_BC, BC_LIST),
    not_kept("Boxing Day", "", CA_MB, MB_LIST),
    not_kept("Boxing Day", "", CA_NL, NL_SOURCE),
    not_kept("Boxing Day", "", CA_NU, NU_SOURCE),
    not_kept("Boxing Day", "", CA_SK, SK_SOURCE),
    not_kept("Boxing Day", "", CA_NB, NB_DAY),
    not_kept("Boxing Day", "", CA_NS, NS_SOURCE),
    not_kept("Boxing Day", "", CA_NT, NT_SOURCE),
    not_kept("Boxing Day", "", CA_PE, PE_SOURCE),
    not_kept("Boxing Day", "Lendemain de Noël", CA_QC, QC_LNT),
    // Yukon's list read is the text in force from 20 April 2023.
    not_kept("Boxing Day", "", CA_YT, YT_SOURCE).read_from(2023),
    // British Columbia's and Manitoba's lists keep the federal day of
    // 2021; the texts read do not say from which year.
    provincial(
        "National Day for Truth and Reconciliation",
        "",
        Rule::gregorian(9, 30),
        CA_BC,
        BC_LIST,
    )
    .years(Some(2021), None)
    .read_from(READ),
    provincial(
        "Orange Shirt Day (National Day for Truth and Reconciliation)",
        "Journée du chandail orange (Journée nationale de la vérité et de la réconciliation)",
        Rule::gregorian(9, 30),
        CA_MB,
        MB_LIST,
    )
    .years(Some(2021), None)
    .read_from(READ),
    // The territories and Prince Edward Island added the federal day by
    // their own acts: Prince Edward Island by SPEI 2021, c 33, assented
    // 17 November 2021 and in the text current to 1 June 2022; the
    // Northwest Territories from 2022, the public service alone having it in
    // 2021; Yukon by SY 2022, c 18, assented 24 November 2022.
    provincial(
        "National Day for Truth and Reconciliation",
        "",
        Rule::gregorian(9, 30),
        CA_PE,
        PE_SOURCE,
    )
    .years(Some(2022), None),
    provincial(
        "National Day for Truth and Reconciliation",
        "",
        Rule::gregorian(9, 30),
        CA_NT,
        NT_SOURCE,
    )
    .years(Some(2022), None),
    provincial(
        "National Day for Truth and Reconciliation",
        "",
        Rule::gregorian(9, 30),
        CA_YT,
        YT_SOURCE,
    )
    .years(Some(2023), None),
    // Quebec's article 60 gives Good Friday or Easter Monday, "au choix de
    // l'employeur": each is a day off only where the employer chose it, so
    // both are observances.
    HolidayRule::observance(
        "Good Friday or Easter Monday, at the employer's choice",
        "Vendredi saint ou lundi de Pâques, au choix de l'employeur",
        Rule::easter(GOOD_FRIDAY),
    )
    .in_regions(CA_QC)
    .cited(QC_LNT)
    .read_from(READ),
    HolidayRule::observance(
        "Good Friday or Easter Monday, at the employer's choice",
        "Vendredi saint ou lundi de Pâques, au choix de l'employeur",
        Rule::easter(EASTER_MONDAY),
    )
    .in_regions(CA_QC)
    .cited(QC_LNT)
    .read_from(READ),
    provincial(
        "National Patriots' Day",
        "Journée nationale des patriotes",
        Rule::WeekdayOnOrBefore {
            month: 5,
            day: 24,
            weekday: Weekday::Monday,
        },
        CA_QC,
        QC_PATRIOTES,
    )
    .years(Some(2003), None),
    // ── The provinces' and territories' own days ─────────────────────────
    // The February day.
    provincial("Family Day", "", THIRD_MONDAY_OF_FEBRUARY, CA_AB, AB_SOURCE)
        .years(Some(1990), None),
    provincial(
        "Family Day",
        "",
        Rule::nth(2, 2, Weekday::Monday),
        CA_BC,
        BC_FAMILY_DAY,
    )
    .years(Some(2013), Some(2018)),
    provincial(
        "Family Day",
        "",
        THIRD_MONDAY_OF_FEBRUARY,
        CA_BC,
        BC_FAMILY_DAY,
    )
    .years(Some(2019), None),
    provincial(
        "Louis Riel Day",
        "jour de Louis Riel",
        THIRD_MONDAY_OF_FEBRUARY,
        CA_MB,
        MB_SOURCE,
    )
    .years(Some(2008), None),
    provincial(
        "Family Day",
        "jour de la Famille",
        THIRD_MONDAY_OF_FEBRUARY,
        CA_NB,
        NB_FAMILY_DAY,
    )
    .years(Some(2018), None),
    provincial(
        "Nova Scotia Heritage Day",
        "",
        THIRD_MONDAY_OF_FEBRUARY,
        CA_NS,
        NS_SOURCE,
    )
    .years(Some(2015), None),
    provincial("Family Day", "", THIRD_MONDAY_OF_FEBRUARY, CA_ON, ON_SOURCE)
        .years(Some(2008), None),
    // Islander Day was first kept on the second Monday of February 2009,
    // and moved to the third from 2010.
    provincial(
        "Islander Day",
        "",
        Rule::nth(2, 2, Weekday::Monday),
        CA_PE,
        PE_SOURCE,
    )
    .years(Some(2009), Some(2009)),
    provincial(
        "Islander Day",
        "",
        THIRD_MONDAY_OF_FEBRUARY,
        CA_PE,
        PE_SOURCE,
    )
    .years(Some(2010), None),
    provincial("Family Day", "", THIRD_MONDAY_OF_FEBRUARY, CA_SK, SK_SOURCE)
        .years(Some(2007), None),
    // The June and July days.
    provincial_fixed(
        "National Indigenous Peoples Day",
        "",
        6,
        21,
        CA_NT,
        NT_SOURCE,
    )
    .years(Some(2001), None),
    provincial_fixed(
        "National Indigenous Peoples Day",
        "",
        6,
        21,
        CA_YT,
        YT_SOURCE,
    )
    .years(Some(2017), None),
    provincial(
        "Saint-Jean-Baptiste Day",
        "Fête nationale du Québec",
        Rule::gregorian(6, 24),
        CA_QC,
        QC_SOURCE,
    )
    .read_from(1979),
    provincial_fixed("Memorial Day", "", 7, 1, CA_NL, NL_SOURCE).read_from(READ),
    provincial_fixed("Nunavut Day", "", 7, 9, CA_NU, NU_SOURCE).years(Some(2001), None),
    // The August days.
    provincial(
        "British Columbia Day",
        "",
        FIRST_MONDAY_OF_AUGUST,
        CA_BC,
        BC_DAY,
    )
    .read_from(READ),
    provincial(
        "New Brunswick Day",
        "",
        FIRST_MONDAY_OF_AUGUST,
        CA_NB,
        NB_DAY,
    )
    .read_from(READ),
    provincial(
        "Civic Holiday",
        "",
        FIRST_MONDAY_OF_AUGUST,
        CA_NT,
        NT_SOURCE,
    )
    .read_from(READ),
    provincial(
        "Civic Holiday",
        "",
        FIRST_MONDAY_OF_AUGUST,
        CA_NU,
        NU_SOURCE,
    )
    .read_from(READ),
    provincial(
        "Saskatchewan Day",
        "",
        FIRST_MONDAY_OF_AUGUST,
        CA_SK,
        SK_SOURCE,
    )
    .read_from(READ),
    provincial(
        "Discovery Day",
        "",
        Rule::nth(8, 3, Weekday::Monday),
        CA_YT,
        YT_SOURCE,
    )
    .read_from(READ),
];

static CA_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// Canada: the federally regulated holidays of the Canada Labour Code, and
/// the provinces' and territories' own general holidays.
///
/// The provinces' days are written up in `docs/systems/canada-holidays.md`
/// in the repository, with every province and territory and what was read
/// for each. Asked for no region, the table answers for the federally
/// regulated employers the Code covers. Asked for a province, it answers
/// for the employers its employment-standards law covers: the federal days
/// less the ones the province's text read leaves out, and the province's
/// own, each a day off, [`Kind::Public`](crate::rule::Kind::Public). A day
/// is carried from the year a source read gives as its first, and absent
/// before; where no source gives one, from 2026, the year read, and the
/// years before are a gap, as a federal day a province leaves out is before
/// 2026. Several sources are secondary, the provinces' statute sites being
/// closed to the reading or publishing PDF only.
pub static CANADA: RuleSet = RuleSet {
    code: "CA",
    english_name: "Canada",
    rules: CA_RULES,
    substitution: CA_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Canada Labour Code (R.S.C. 1985, c. L-2), s. 166, and the Holidays Act (R.S.C. \
              1985, c. H-5), both current to 2026-09-03, on the Justice Laws Website \
              (laws-lois.justice.gc.ca), retrieved 2026-09-26; the Holidays Act moves only a \
              Sunday Canada Day, and the Code's s. 195 on a holiday falling on a non-working \
              day was not read; the provinces' and territories' days each cited on its entries, \
              read 2026-09-29, as docs/systems/canada-holidays.md lists them. The days asked \
              for no region are the federal ones, and a province's are the federal ones its \
              text read keeps, as Ontario's keeps no Remembrance Day, and its own",
    subdivisions: Subdivisions::Read(&[]),
};
