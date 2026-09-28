//! Canada.

use hc_calendar::Weekday;

use crate::computus::offsets::GOOD_FRIDAY;
use crate::rule::{
    HolidayRule, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate, SubstituteDirection,
    SubstitutionPolicy,
};

// The provinces and territories with a day of their own, by ISO 3166-2
// code, and the statute or page each was read in.
const CA_AB: &[&str] = &["CA-AB"];
const CA_BC: &[&str] = &["CA-BC"];
const CA_MB: &[&str] = &["CA-MB"];
const CA_NB: &[&str] = &["CA-NB"];
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
const MB_SOURCE: &str = "The Employment Standards Code, C.C.S.M. c. E110, s 21(1)(a.1), added by The \
     Statutory Holidays Act (Various Acts Amended), S.M. 2007, c. 18, in force 8 November 2007 \
     (web2.gov.mb.ca), retrieved 2026-09-29";
const NB_FAMILY_DAY: &str = "An Act Respecting Family Day (Bill 67, 58th Legislature), amending the \
     Employment Standards Act, SNB 1982, c E-7.2, s 1, in force 1 January 2018 (legnb.ca), \
     retrieved 2026-09-29";
const NB_DAY: &str = "Employment Standards Act, SNB 1982, c E-7.2, s 1, known from a search excerpt of \
     CanLII's copy and the date from canada-holidays.ca (secondary; the Act's sites refused the \
     connection), retrieved 2026-09-29";
const NS_SOURCE: &str = "Nova Scotia, \"Changes to the Labour Standards Code\" \
     (novascotia.ca/lae/employmentrights, secondary: the Code and the 2013 Act were not \
     reachable), retrieved 2026-09-29: in force 1 January 2015";
const NT_SOURCE: &str = "Employment Standards Act, SNWT 2007, c 13, s 1, not read (PDF only); the days \
     from canada-holidays.ca and Wikipedia, \"National Indigenous Peoples Day\" (secondary), \
     retrieved 2026-09-29";
const NU_SOURCE: &str = "Nunavut Labour Standards Compliance Office, Fact Sheet 10, \"General \
     Holidays\" (nu-lsco.ca, secondary: the Labour Standards Act was not reachable), retrieved \
     2026-09-29; Nunavut Day's date and first year from Wikipedia, \"Nunavut Day\" (secondary)";
const ON_SOURCE: &str = "Employment Standards Act, 2000, S.O. 2000, c. 41, s 1(1) \"public holiday\", \
     Family Day added by 2007, c. 16, Sched. A, s 1, in force 3 December 2007 (ontario.ca/laws), \
     retrieved 2026-09-29";
const PE_SOURCE: &str = "Employment Standards Act, RSPEI 1988, c E-6.2, not read (PDF, and the site's \
     browser check); the day from Wikipedia, \"Family Day (Canada)\", and canada-holidays.ca \
     (secondary), retrieved 2026-09-29";
const QC_SOURCE: &str = "Loi sur la fête nationale, RLRQ c F-1.1, not read (LégisQuébec did not \
     answer); the day from canada-holidays.ca (secondary), retrieved 2026-09-29";
const SK_SOURCE: &str = "Government of Saskatchewan, \"List of Saskatchewan Public Holidays\" \
     (saskatchewan.ca, secondary: The Saskatchewan Employment Act, SS 2013, c S-15.1, is PDF \
     only), retrieved 2026-09-29; Family Day's first year from Wikipedia, \"Family Day \
     (Canada)\" (secondary)";
const YT_SOURCE: &str = "Employment Standards Act, RSY 2002, c 72, not read (the Territory's sites \
     refused the connection); the days from canada-holidays.ca and Wikipedia (secondary), \
     retrieved 2026-09-29";

/// A province's general holiday, from `first`, in `region`: a day off
/// under its employment-standards law, moved off a weekend as the federal
/// ones are.
const fn provincial(
    name: &'static str,
    local_name: &'static str,
    rule: Rule,
    first: i32,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::public(name, local_name, rule)
        .years(Some(first), None)
        .in_regions(region)
        .cited(source)
}

/// A province's general holiday on a fixed date, from `first`, left where
/// it falls: no provincial weekend rule for it was read.
const fn provincial_fixed(
    name: &'static str,
    local_name: &'static str,
    month: u8,
    day: u8,
    first: i32,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(name, local_name, Rule::gregorian(month, day))
        .years(Some(first), None)
        .in_regions(region)
        .cited(source)
}

/// The years before a day's first year, as a gap: the source read gives the
/// day as it stands and not the year it was set, so whether it was kept
/// before is not known.
const fn earlier_years_unread(
    name: &'static str,
    local_name: &'static str,
    first: i32,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::observance(name, local_name, Rule::UNREAD)
        .years(None, Some(first - 1))
        .in_regions(region)
        .cited(source)
}

/// The third Monday of February, the provinces' February day.
const THIRD_MONDAY_OF_FEBRUARY: Rule = Rule::nth(2, 3, Weekday::Monday);
/// The first Monday of August, the provinces' August day.
const FIRST_MONDAY_OF_AUGUST: Rule = Rule::nth(8, 1, Weekday::Monday);

static CA_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Jour de l'An", Rule::gregorian(1, 1)),
    HolidayRule::public("Good Friday", "Vendredi saint", Rule::easter(GOOD_FRIDAY)),
    // Victoria Day is the Monday preceding 25 May.
    HolidayRule::public(
        "Victoria Day",
        "Fête de la Reine",
        Rule::WeekdayOnOrBefore {
            month: 5,
            day: 24,
            weekday: Weekday::Monday,
        },
    ),
    HolidayRule::public("Canada Day", "Fête du Canada", Rule::gregorian(7, 1)),
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
    .years(Some(2021), None),
    HolidayRule::public(
        "Thanksgiving",
        "Action de grâce",
        Rule::nth(10, 2, Weekday::Monday),
    ),
    HolidayRule::public(
        "Remembrance Day",
        "Jour du Souvenir",
        Rule::gregorian(11, 11),
    ),
    HolidayRule::public("Christmas Day", "Noël", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "Lendemain de Noël", Rule::gregorian(12, 26)),
    // ── The provinces' and territories' own days ─────────────────────────
    // The February day.
    provincial(
        "Family Day",
        "",
        THIRD_MONDAY_OF_FEBRUARY,
        1990,
        CA_AB,
        AB_SOURCE,
    ),
    HolidayRule::public("Family Day", "", Rule::nth(2, 2, Weekday::Monday))
        .years(Some(2013), Some(2018))
        .in_regions(CA_BC)
        .cited(BC_FAMILY_DAY),
    provincial(
        "Family Day",
        "",
        THIRD_MONDAY_OF_FEBRUARY,
        2019,
        CA_BC,
        BC_FAMILY_DAY,
    ),
    provincial(
        "Louis Riel Day",
        "jour de Louis Riel",
        THIRD_MONDAY_OF_FEBRUARY,
        2008,
        CA_MB,
        MB_SOURCE,
    ),
    provincial(
        "Family Day",
        "jour de la Famille",
        THIRD_MONDAY_OF_FEBRUARY,
        2018,
        CA_NB,
        NB_FAMILY_DAY,
    ),
    provincial(
        "Nova Scotia Heritage Day",
        "",
        THIRD_MONDAY_OF_FEBRUARY,
        2015,
        CA_NS,
        NS_SOURCE,
    ),
    provincial(
        "Family Day",
        "",
        THIRD_MONDAY_OF_FEBRUARY,
        2008,
        CA_ON,
        ON_SOURCE,
    ),
    provincial(
        "Islander Day",
        "",
        THIRD_MONDAY_OF_FEBRUARY,
        2009,
        CA_PE,
        PE_SOURCE,
    ),
    provincial(
        "Family Day",
        "",
        THIRD_MONDAY_OF_FEBRUARY,
        2007,
        CA_SK,
        SK_SOURCE,
    ),
    // The June and July days.
    provincial_fixed(
        "National Indigenous Peoples Day",
        "",
        6,
        21,
        2001,
        CA_NT,
        NT_SOURCE,
    ),
    provincial_fixed(
        "National Indigenous Peoples Day",
        "",
        6,
        21,
        2017,
        CA_YT,
        YT_SOURCE,
    ),
    provincial(
        "Saint-Jean-Baptiste Day",
        "Fête nationale du Québec",
        Rule::gregorian(6, 24),
        2026,
        CA_QC,
        QC_SOURCE,
    ),
    earlier_years_unread(
        "Saint-Jean-Baptiste Day",
        "Fête nationale du Québec",
        2026,
        CA_QC,
        QC_SOURCE,
    ),
    provincial_fixed("Nunavut Day", "", 7, 9, 2001, CA_NU, NU_SOURCE),
    // The August days.
    provincial(
        "British Columbia Day",
        "",
        FIRST_MONDAY_OF_AUGUST,
        2026,
        CA_BC,
        BC_DAY,
    ),
    earlier_years_unread("British Columbia Day", "", 2026, CA_BC, BC_DAY),
    provincial(
        "New Brunswick Day",
        "",
        FIRST_MONDAY_OF_AUGUST,
        2026,
        CA_NB,
        NB_DAY,
    ),
    earlier_years_unread("New Brunswick Day", "", 2026, CA_NB, NB_DAY),
    provincial(
        "Civic Holiday",
        "",
        FIRST_MONDAY_OF_AUGUST,
        2026,
        CA_NT,
        NT_SOURCE,
    ),
    earlier_years_unread("Civic Holiday", "", 2026, CA_NT, NT_SOURCE),
    provincial(
        "Civic Holiday",
        "",
        FIRST_MONDAY_OF_AUGUST,
        2026,
        CA_NU,
        NU_SOURCE,
    ),
    earlier_years_unread("Civic Holiday", "", 2026, CA_NU, NU_SOURCE),
    provincial(
        "Saskatchewan Day",
        "",
        FIRST_MONDAY_OF_AUGUST,
        2026,
        CA_SK,
        SK_SOURCE,
    ),
    earlier_years_unread("Saskatchewan Day", "", 2026, CA_SK, SK_SOURCE),
    provincial(
        "Discovery Day",
        "",
        Rule::nth(8, 3, Weekday::Monday),
        2026,
        CA_YT,
        YT_SOURCE,
    ),
    earlier_years_unread("Discovery Day", "", 2026, CA_YT, YT_SOURCE),
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
/// for each. A province's general holiday binds the employers its
/// employment-standards law covers, so it is a day off,
/// [`Kind::Public`](crate::rule::Kind::Public), in its region. A day is
/// carried from the year a source read gives as its first, and absent
/// before; where no source gives one, from 2026, the year read, and the
/// years before are a gap; several sources are secondary, the
/// provinces' statute sites being closed to the reading or publishing PDF
/// only.
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
              read 2026-09-29, as docs/systems/canada-holidays.md lists them. The national days \
              are the federal ones, and a province that does not keep one, as Ontario does not \
              keep Remembrance Day, is not modelled",
};
