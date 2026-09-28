//! The United States — the federal holidays of 5 U.S.C. § 6103, and the
//! states' own days from their codes.
//!
//! The states' days are written up in `docs/systems/us-state-holidays.md`
//! in the repository, with every state and the District of Columbia, what
//! was read for each and what each code makes of its days; this comment
//! keeps the summary.
//!
//! A state's code lists the state's legal holidays, and most of them are
//! the federal days, which the table carries nationwide. The days a state
//! lists beyond them are rules scoped to its ISO 3166-2 code. None binds a
//! private employer. Where the code, or the state's personnel law read
//! beside it, closes the state's offices or makes the day a paid holiday
//! for its employees, the day is [`Kind::Government`]; where it only makes
//! the day a legal holiday for courts, deadlines and instruments, says the
//! offices may stay open, or designates a day of observance, it is
//! [`Kind::Observance`]. Neither counts as a day off for business days.
//! A day is carried from the session law that set it where the section's
//! history names one, and absent before; otherwise from the year of the text
//! read, the years before it a gap (`earlier_years_unread`), since the day
//! is usually older than the copy.
//! The codes' own weekend moves are not carried: the engine substitutes
//! only days off, and a state's day is not one. Days whose date the code leaves
//! to a governor, an election law not read or a local body, days for part
//! of a state, and half days are not carried.

use hc_calendar::Weekday;
use hc_calendars_solar::gregorian;

use crate::computus::offsets::{GOOD_FRIDAY, SHROVE_TUESDAY};
use crate::rule::{
    CalendarSystem, Days, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate,
    SubstituteDirection, SubstitutionPolicy,
};

/// The District of Columbia and the counties around it, the only place
/// Inauguration Day is a holiday.
const US_CAPITAL_REGION: &[&str] = &["US-DC"];

/// Inauguration Day, 5 U.S.C. § 6103(c).
///
/// "January 20 of each fourth year after 1965" — and 21 January when the
/// 20th is a Sunday, because the oath is not administered publicly on a
/// Sunday. The section explicitly denies the in-lieu-of day that every other
/// federal holiday gets, so a Saturday inauguration simply is not a holiday
/// for anyone. Nothing in the rule vocabulary says "every fourth year", so
/// this is one of the genuine handful.
fn inauguration_day(year: i64) -> Days {
    if year < 1965 || (year - 1965).rem_euclid(4) != 0 {
        return Days::new();
    }
    let Ok(twentieth) = gregorian::to_fixed(year, 1, 20) else {
        return Days::new();
    };
    if Weekday::from_rd(twentieth) == Weekday::Sunday {
        return Days::one(hc_calendar::Rd(twentieth.0 + 1));
    }
    Days::one(twentieth)
}

/// The Friday after Thanksgiving, the fourth Thursday of November: the
/// first Friday on or after 23 November.
const FRIDAY_AFTER_THANKSGIVING: Rule = Rule::WeekdayOnOrAfter {
    month: 11,
    day: 23,
    weekday: Weekday::Friday,
};

/// "The Tuesday next after the first Monday in November": the first
/// Tuesday on or after 2 November.
const ELECTION_DAY: Rule = Rule::WeekdayOnOrAfter {
    month: 11,
    day: 2,
    weekday: Weekday::Tuesday,
};

/// The general election day in the even years, when the states the table
/// carries hold their general elections, and in no other.
fn even_year_election_day(year: i64) -> Days {
    if year.rem_euclid(2) != 0 {
        return Days::new();
    }
    ELECTION_DAY.days_in_year(year)
}

/// The general election day in the years of a presidential election, those
/// divisible by four.
fn presidential_election_day(year: i64) -> Days {
    if year.rem_euclid(4) != 0 {
        return Days::new();
    }
    ELECTION_DAY.days_in_year(year)
}

/// Utah's Juneteenth, 19 June kept on a Monday: Utah Code § 63G-1-301(1)(f)
/// keeps it on 19 June when that is a Monday, on the Monday before when it
/// is a Tuesday to a Friday, and on the Monday after when it is a Saturday
/// or a Sunday.
static JUNE_19: Rule = Rule::gregorian(6, 19);
static TO_PRECEDING_OR_NEXT_MONDAY: &[(Weekday, i16)] = &[
    (Weekday::Tuesday, -1),
    (Weekday::Wednesday, -2),
    (Weekday::Thursday, -3),
    (Weekday::Friday, -4),
    (Weekday::Saturday, 2),
    (Weekday::Sunday, 1),
];

/// A state's day that closes its offices or is a paid holiday for its
/// employees, from `first`, in the state `region`.
const fn state(
    name: &'static str,
    rule: Rule,
    first: i32,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(name, "", rule)
        .of_kind(Kind::Government)
        .years(Some(first), None)
        .in_regions(region)
        .cited(source)
}

/// A state's legal holiday that closes nothing, or its designated day of
/// observance, from `first`, in the state `region`.
const fn state_observance(
    name: &'static str,
    rule: Rule,
    first: i32,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::observance(name, "", rule)
        .years(Some(first), None)
        .in_regions(region)
        .cited(source)
}

/// The years before a state's day's first year, from `from` (all of them
/// when `None`), as a gap: the text read is the version in force and not
/// the act that set the day, so whether the day was kept in those years is
/// not known, and saying it was not would be a guess.
const fn earlier_years_unread(
    name: &'static str,
    from: Option<i32>,
    first: i32,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::observance(name, "", Rule::UNREAD)
        .years(from, Some(first - 1))
        .in_regions(region)
        .cited(source)
}

// The states with a day of their own, by ISO 3166-2 code, and the code
// section each was read in.
const US_AK: &[&str] = &["US-AK"];
const US_AK_LAW: &str = "Alaska Stat. § 44.12.010, FindLaw's copy current as of 1 January 2025 (secondary) (https://codes.findlaw.com/ak/title-44-state-government/ak-st-sect-44-12-010/), retrieved 2026-09-29";
const US_AL: &[&str] = &["US-AL"];
const US_AL_LAW: &str = "Code of Alabama 1975 § 1-3-8, as amended by Act 2025-303, the Legislature's site (https://alison.legislature.state.al.us/code-of-alabama?section=1-3-8), retrieved 2026-09-29";
const US_AR: &[&str] = &["US-AR"];
const US_AR_LAW: &str = "Ark. Code Ann. § 1-5-101, FindLaw's copy current as of 28 March 2024 (secondary) (https://codes.findlaw.com/ar/title-1-general-provisions/ar-code-sect-1-5-101/), retrieved 2026-09-29";
const US_AZ: &[&str] = &["US-AZ"];
const US_AZ_LAW: &str = "A.R.S. §§ 1-301 and 1-302, the Legislature's site, which shows no history (https://www.azleg.gov/ars/1/00301.htm), retrieved 2026-09-29";
const US_CA: &[&str] = &["US-CA"];
const US_CA_LAW: &str = "Cal. Gov. Code §§ 6700, 6712 and 19853, California Legislative Information (https://leginfo.legislature.ca.gov/faces/codes_displaySection.xhtml?lawCode=GOV&sectionNum=6700), retrieved 2026-09-29";
const US_CO: &[&str] = &["US-CO"];
const US_CO_LAW: &str = "C.R.S. § 24-11-101, as amended by HB20-1031, Colorado.Public.Law's copy of the 2024 edition (secondary) and the bill's page (https://leg.colorado.gov/bills/hb20-1031), retrieved 2026-09-29";
const US_CT: &[&str] = &["US-CT"];
const US_CT_LAW: &str = "Conn. Gen. Stat. § 1-4, the Internet Archive's capture of 10 September 2026 of the General Assembly's page (https://www.cga.ct.gov/current/pub/chap_002.htm), retrieved 2026-09-29";
const US_DC: &[&str] = &["US-DC"];
const US_DC_LAW: &str = "D.C. Code §§ 1-612.02 and 28-2701, the Council's code site (https://code.dccouncil.gov/us/dc/council/code/sections/1-612.02), retrieved 2026-09-29";
const US_DE: &[&str] = &["US-DE"];
const US_DE_LAW: &str = "1 Del. C. § 501, the General Assembly's site (https://delcode.delaware.gov/title1/c005/index.html), retrieved 2026-09-29";
const US_FL: &[&str] = &["US-FL"];
const US_FL_LAW: &str = "Fla. Stat. §§ 683.01, 683.06 and 110.117 (2026), the Legislature's site (https://www.leg.state.fl.us/statutes/index.cfm?App_mode=Display_Statute&URL=0600-0699/0683/0683.html), retrieved 2026-09-29";
const US_HI: &[&str] = &["US-HI"];
const US_HI_LAW: &str = "Haw. Rev. Stat. §§ 8-1 and 8-2, the Legislature's data site (https://data.capitol.hawaii.gov/hrscurrent/Vol01_Ch0001-0042F/HRS0008/), retrieved 2026-09-29";
const US_IA: &[&str] = &["US-IA"];
const US_IA_LAW: &str = "Iowa Code §§ 1C.1 and 1C.2 (2026), the Legislature's site (https://www.legis.iowa.gov/docs/code/2026/1C.html), retrieved 2026-09-29";
const US_ID: &[&str] = &["US-ID"];
const US_ID_LAW: &str = "Idaho Code §§ 73-108A to 73-108C, the Internet Archive's 2026 captures of the Legislature's pages (https://legislature.idaho.gov/statutesrules/idstat/title73/t73ch1/), retrieved 2026-09-29";
const US_IL: &[&str] = &["US-IL"];
const US_IL_LAW: &str = "205 ILCS 630/17, the Internet Archive's capture of 3 April 2025 of the General Assembly's page (https://www.ilga.gov/legislation/ilcs/fulltext.asp?DocName=020506300K17), retrieved 2026-09-29";
const US_IN: &[&str] = &["US-IN"];
const US_IN_LAW: &str = "Ind. Code §§ 1-1-9-1 and 1-1-9-2, FindLaw's copy current as of 1 January 2026 (secondary) (https://codes.findlaw.com/in/title-1-general-provisions/in-code-sect-1-1-9-1/), retrieved 2026-09-29";
const US_KS: &[&str] = &["US-KS"];
const US_KS_LAW: &str = "K.S.A. 35-201 to 35-208, the Legislature's site (https://www.kslegislature.gov/li/b2025_26/statute/035_000_0000_chapter/035_002_0000_article/), retrieved 2026-09-29";
const US_KY: &[&str] = &["US-KY"];
const US_KY_LAW: &str = "KRS 2.110, 2.190 and 18A.190, FindLaw's copies current as of 1 January 2025 (secondary) (https://codes.findlaw.com/ky/title-i-sovereignty-and-jurisdiction-of-the-commonwealth/ky-rev-st-sect-2-110/), retrieved 2026-09-29";
const US_LA: &[&str] = &["US-LA"];
const US_LA_LAW: &str = "La. R.S. 1:55, the Legislature's site (https://legis.la.gov/Legis/Law.aspx?d=74097), retrieved 2026-09-29";
const US_MA: &[&str] = &["US-MA"];
const US_MA_LAW: &str = "Mass. Gen. Laws c. 4, § 7, cl. Eighteenth, FindLaw's copy current as of 1 January 2025 (secondary) (https://codes.findlaw.com/ma/part-i-administration-of-the-government-ch-1-182/ma-gen-laws-ch-4-sect-7/), retrieved 2026-09-29";
const US_MD: &[&str] = &["US-MD"];
const US_MD_LAW: &str = "Md. Code, General Provisions § 1-111 and State Personnel and Pensions § 9-201, the General Assembly's site (https://mgaleg.maryland.gov/mgawebsite/Laws/StatuteText?article=ggp&section=1-111&enactments=false), retrieved 2026-09-29";
const US_ME: &[&str] = &["US-ME"];
const US_ME_LAW: &str = "4 M.R.S. § 1051, the Legislature's site (https://legislature.maine.gov/statutes/4/title4sec1051-1.html), retrieved 2026-09-29";
const US_MI: &[&str] = &["US-MI"];
const US_MI_LAW: &str = "MCL 435.101, FindLaw's copy current as of 1 January 2025 (secondary) (https://codes.findlaw.com/mi/chapter-435-sundays-and-holidays/mi-comp-laws-435-101/), retrieved 2026-09-29";
const US_MN: &[&str] = &["US-MN"];
const US_MN_LAW: &str = "Minn. Stat. § 645.44, subd. 5 (2025), the Revisor's site (https://www.revisor.mn.gov/statutes/cite/645.44), retrieved 2026-09-29";
const US_MO: &[&str] = &["US-MO"];
const US_MO_LAW: &str = "RSMo 9.010 and 9.035, the Revisor's site, 9.010 as in force from 28 August 2022 (https://revisor.mo.gov/main/OneSection.aspx?section=9.010), retrieved 2026-09-29";
const US_MS: &[&str] = &["US-MS"];
const US_MS_LAW: &str = "Miss. Code § 3-3-7, FindLaw's copy current as of 1 January 2025 (secondary) (https://codes.findlaw.com/ms/title-3-state-sovereignty-jurisdiction-and-holidays/ms-code-sect-3-3-7/), retrieved 2026-09-29";
const US_MT: &[&str] = &["US-MT"];
const US_MT_LAW: &str = "MCA 1-1-216 (2025), the Legislature's site (https://mca.legmt.gov/bills/mca/title_0010/chapter_0010/part_0020/section_0160/0010-0010-0020-0160.html), retrieved 2026-09-29";
const US_NC: &[&str] = &["US-NC"];
const US_NC_LAW: &str = "N.C. Gen. Stat. § 103-4, FindLaw's copy current as of 1 January 2023 (secondary) (https://codes.findlaw.com/nc/chapter-103-sundays-holidays-and-special-days/nc-gen-st-sect-103-4.html), retrieved 2026-09-29";
const US_ND: &[&str] = &["US-ND"];
const US_ND_LAW: &str = "N.D. Cent. Code § 1-03-01, FindLaw's copy current as of 1 January 2024 (secondary) (https://codes.findlaw.com/nd/title-1-general-provisions/nd-cent-code-sect-1-03-01/), retrieved 2026-09-29";
const US_NE: &[&str] = &["US-NE"];
const US_NE_LAW: &str = "Neb. Rev. Stat. §§ 25-2221 and 84-1001, FindLaw's copies current as of 1 January 2024 (secondary) (https://codes.findlaw.com/ne/chapter-25-courts-civil-procedure/ne-rev-st-sect-25-2221/), retrieved 2026-09-29";
const US_NJ: &[&str] = &["US-NJ"];
const US_NJ_LAW: &str = "N.J.S.A. 36:1-1, FindLaw's copy current as of 1 January 2024 (secondary) (https://codes.findlaw.com/nj/title-36-legal-holidays/nj-st-sect-36-1-1/), retrieved 2026-09-29";
const US_NM: &[&str] = &["US-NM"];
const US_NM_LAW: &str = "NMSA 12-5-1, 12-5-4, 12-5-7, 12-5-9, 12-5-10 and 12-5-12, FindLaw's copies current as of 1 January 2024 (secondary) (https://codes.findlaw.com/nm/chapter-12-miscellaneous-public-affairs-matters/nm-st-sect-12-5-2/), retrieved 2026-09-29";
const US_NV: &[&str] = &["US-NV"];
const US_NV_LAW: &str = "NRS 236.015, FindLaw's copy current as of 1 January 2025 (secondary) (https://codes.findlaw.com/nv/title-19-miscellaneous-matters-related-to-government-and-public-affairs/nv-rev-st-236-015/), retrieved 2026-09-29";
const US_NY: &[&str] = &["US-NY"];
const US_NY_LAW: &str = "N.Y. Gen. Constr. Law § 24, the Senate's site, the section as revised 16 October 2020 (https://www.nysenate.gov/legislation/laws/GCN/24), retrieved 2026-09-29";
const US_OR: &[&str] = &["US-OR"];
const US_OR_LAW: &str = "ORS 187.278, Oregon.Public.Law's copy (secondary) (https://oregon.public.law/statutes/ors_187.278), retrieved 2026-09-29";
const US_PA: &[&str] = &["US-PA"];
const US_PA_LAW: &str = "44 P.S. § 11, FindLaw's copy current as of 1 January 2026 (secondary) (https://codes.findlaw.com/pa/title-44-ps-legal-holidays-and-observances/pa-st-sect-44-11.html), retrieved 2026-09-29";
const US_RI: &[&str] = &["US-RI"];
const US_RI_LAW: &str = "R.I. Gen. Laws § 25-1-1, FindLaw's copy current as of 1 January 2026 (secondary) (https://codes.findlaw.com/ri/title-25-holidays-and-days-of-special-observance/ri-gen-laws-sect-25-1-1/), retrieved 2026-09-29";
const US_SC: &[&str] = &["US-SC"];
const US_SC_LAW: &str = "S.C. Code Ann. §§ 53-5-10 and 53-5-30, as amended by 2009 Act No. 33, the General Assembly's site (https://www.scstatehouse.gov/code/t53c005.php), retrieved 2026-09-29";
const US_SD: &[&str] = &["US-SD"];
const US_SD_LAW: &str = "SDCL 1-5-1.3 and 1-5-8 to 1-5-18, the Legislature's site (https://sdlegislature.gov/api/Statutes/1-5.html?all=true), retrieved 2026-09-29";
const US_TN: &[&str] = &["US-TN"];
const US_TN_LAW: &str = "Tenn. Code Ann. § 15-1-101, FindLaw's copy current as of 2 January 2024 (secondary) (https://codes.findlaw.com/tn/title-15-holidays-and-days-of-special-observance/tn-code-sect-15-1-101/), retrieved 2026-09-29";
const US_TX: &[&str] = &["US-TX"];
const US_TX_LAW: &str = "Tex. Gov't Code §§ 662.003 to 662.005, Texas.Public.Law's copies verified 26 May 2025 (secondary) (https://texas.public.law/statutes/tex._gov't_code_section_662.003), retrieved 2026-09-29";
const US_UT: &[&str] = &["US-UT"];
const US_UT_LAW: &str = "Utah Code § 63G-1-301, FindLaw's copy current as of 1 January 2025 (secondary) (https://codes.findlaw.com/ut/title-63g-general-government/ut-code-sect-63g-1-301/), retrieved 2026-09-29";
const US_VA: &[&str] = &["US-VA"];
const US_VA_LAW: &str = "Va. Code § 2.2-3300, as amended by Acts 2020, cc. 417 and 418, the General Assembly's site (https://law.lis.virginia.gov/vacode/title2.2/chapter33/section2.2-3300/), retrieved 2026-09-29";
const US_VT: &[&str] = &["US-VT"];
const US_VT_LAW: &str = "Vt. Stat. Ann. tit. 1, § 371, FindLaw's copy current as of 1 January 2024 (secondary) (https://codes.findlaw.com/vt/title-1-general-provisions/vt-st-tit-1-sect-371/), retrieved 2026-09-29";
const US_WA: &[&str] = &["US-WA"];
const US_WA_LAW: &str = "RCW 1.16.050, the Legislature's site (https://app.leg.wa.gov/RCW/default.aspx?cite=1.16.050), retrieved 2026-09-29";
const US_WI: &[&str] = &["US-WI"];
const US_WI_LAW: &str = "Wis. Stat. § 995.20, FindLaw's copy current as of 1 January 2025 (secondary) (https://codes.findlaw.com/wi/miscellaneous-statutes-ch-995/wi-st-995-20/), retrieved 2026-09-29";
const US_WV: &[&str] = &["US-WV"];
const US_WV_LAW: &str = "W. Va. Code § 2-2-1, the Legislature's site, as amended in 2026 (https://code.wvlegislature.gov/2-2-1/), retrieved 2026-09-29";

static US_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)).years(Some(1871), None),
    HolidayRule::public(
        "Birthday of Martin Luther King, Jr.",
        "",
        Rule::nth(1, 3, Weekday::Monday),
    )
    .years(Some(1986), None),
    HolidayRule::fixed_public("Inauguration Day", "", Rule::Computed(inauguration_day))
        .in_regions(US_CAPITAL_REGION),
    // The Uniform Monday Holiday Act of 1968 took effect on 1 January 1971.
    HolidayRule::public("Washington's Birthday", "", Rule::gregorian(2, 22))
        .years(Some(1879), Some(1970)),
    HolidayRule::public(
        "Washington's Birthday",
        "",
        Rule::nth(2, 3, Weekday::Monday),
    )
    .years(Some(1971), None),
    HolidayRule::public("Memorial Day", "", Rule::gregorian(5, 30)).years(Some(1888), Some(1970)),
    HolidayRule::public("Memorial Day", "", Rule::last(5, Weekday::Monday)).years(Some(1971), None),
    HolidayRule::public(
        "Juneteenth National Independence Day",
        "",
        Rule::gregorian(6, 19),
    )
    .years(Some(2021), None),
    HolidayRule::public("Independence Day", "", Rule::gregorian(7, 4)).years(Some(1871), None),
    HolidayRule::public("Labor Day", "", Rule::nth(9, 1, Weekday::Monday)).years(Some(1894), None),
    HolidayRule::public("Columbus Day", "", Rule::nth(10, 2, Weekday::Monday))
        .years(Some(1971), None),
    // Veterans Day spent the Uniform Monday years on the fourth Monday of
    // October; Public Law 94-97 put it back on 11 November from 1978.
    HolidayRule::public("Veterans Day", "", Rule::gregorian(11, 11)).years(Some(1954), Some(1970)),
    HolidayRule::public("Veterans Day", "", Rule::nth(10, 4, Weekday::Monday))
        .years(Some(1971), Some(1977)),
    HolidayRule::public("Veterans Day", "", Rule::gregorian(11, 11)).years(Some(1978), None),
    HolidayRule::public("Thanksgiving Day", "", Rule::nth(11, 4, Weekday::Thursday))
        .years(Some(1942), None),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)).years(Some(1871), None),
    // Every full day an executive order closed the executive departments
    // from 2018: two state funerals and the Christmas closures.
    HolidayRule::fixed_public("National Day of Mourning", "", Rule::gregorian(12, 5))
        .years(Some(2018), Some(2018))
        .cited("Executive Order 13852 of 1 December 2018, 83 FR 62687"),
    us_closure(
        12,
        24,
        2018,
        "Executive Order 13854 of 18 December 2018, 83 FR 65481",
    ),
    us_closure(
        12,
        24,
        2019,
        "Executive Order 13900 of 17 December 2019, 84 FR 69983",
    ),
    us_closure(
        12,
        24,
        2020,
        "Executive Order 13965 of 11 December 2020, 85 FR 81337",
    ),
    us_closure(
        12,
        24,
        2024,
        "Executive Order 14129 of 18 December 2024, 89 FR 104857",
    ),
    HolidayRule::fixed_public("National Day of Mourning", "", Rule::gregorian(1, 9))
        .years(Some(2025), Some(2025))
        .cited("Executive Order 14133 of 30 December 2024, 90 FR 187"),
    us_closure(
        12,
        24,
        2025,
        "Executive Order 14371 of 18 December 2025, 90 FR 60545",
    ),
    us_closure(
        12,
        26,
        2025,
        "Executive Order 14371 of 18 December 2025, 90 FR 60545",
    ),
    // ── The states' own days ──────────────────────────────────────────
    // Alaska.
    state(
        "Seward's Day",
        Rule::last(3, Weekday::Monday),
        2025,
        US_AK,
        US_AK_LAW,
    ),
    earlier_years_unread("Seward's Day", None, 2025, US_AK, US_AK_LAW),
    state(
        "Alaska Day",
        Rule::gregorian(10, 18),
        2025,
        US_AK,
        US_AK_LAW,
    ),
    earlier_years_unread("Alaska Day", None, 2025, US_AK, US_AK_LAW),
    // Alabama.
    state(
        "Confederate Memorial Day",
        Rule::nth(4, 4, Weekday::Monday),
        2025,
        US_AL,
        US_AL_LAW,
    ),
    earlier_years_unread("Confederate Memorial Day", None, 2025, US_AL, US_AL_LAW),
    state(
        "Jefferson Davis' Birthday",
        Rule::nth(6, 1, Weekday::Monday),
        2025,
        US_AL,
        US_AL_LAW,
    ),
    earlier_years_unread("Jefferson Davis' Birthday", None, 2025, US_AL, US_AL_LAW),
    // Arkansas.
    state(
        "Christmas Eve",
        Rule::gregorian(12, 24),
        2024,
        US_AR,
        US_AR_LAW,
    ),
    earlier_years_unread("Christmas Eve", None, 2024, US_AR, US_AR_LAW),
    // Arizona.
    state(
        "Mothers' Day",
        Rule::nth(5, 2, Weekday::Sunday),
        2026,
        US_AZ,
        US_AZ_LAW,
    ),
    earlier_years_unread("Mothers' Day", None, 2026, US_AZ, US_AZ_LAW),
    state(
        "Native American Day",
        Rule::WeekdayOnOrAfter {
            month: 6,
            day: 2,
            weekday: Weekday::Sunday,
        },
        2026,
        US_AZ,
        US_AZ_LAW,
    ),
    earlier_years_unread("Native American Day", None, 2026, US_AZ, US_AZ_LAW),
    state(
        "Fathers' Day",
        Rule::nth(6, 3, Weekday::Sunday),
        2026,
        US_AZ,
        US_AZ_LAW,
    ),
    earlier_years_unread("Fathers' Day", None, 2026, US_AZ, US_AZ_LAW),
    state(
        "American Family Day",
        Rule::nth(8, 1, Weekday::Sunday),
        2026,
        US_AZ,
        US_AZ_LAW,
    ),
    earlier_years_unread("American Family Day", None, 2026, US_AZ, US_AZ_LAW),
    state(
        "Constitution Commemoration Day",
        Rule::WeekdayOnOrBefore {
            month: 9,
            day: 17,
            weekday: Weekday::Sunday,
        },
        2026,
        US_AZ,
        US_AZ_LAW,
    ),
    earlier_years_unread(
        "Constitution Commemoration Day",
        None,
        2026,
        US_AZ,
        US_AZ_LAW,
    ),
    // California.
    state(
        "Farmworkers Day",
        Rule::gregorian(3, 31),
        2026,
        US_CA,
        US_CA_LAW,
    ),
    earlier_years_unread("Farmworkers Day", None, 2026, US_CA, US_CA_LAW),
    state_observance(
        "Lincoln Day",
        Rule::gregorian(2, 12),
        2026,
        US_CA,
        US_CA_LAW,
    ),
    earlier_years_unread("Lincoln Day", None, 2026, US_CA, US_CA_LAW),
    state_observance(
        "Genocide Remembrance Day",
        Rule::gregorian(4, 24),
        2023,
        US_CA,
        US_CA_LAW,
    ),
    state_observance(
        "Admission Day",
        Rule::gregorian(9, 9),
        2026,
        US_CA,
        US_CA_LAW,
    ),
    earlier_years_unread("Admission Day", None, 2026, US_CA, US_CA_LAW),
    state_observance(
        "Native American Day",
        Rule::nth(9, 4, Weekday::Friday),
        1999,
        US_CA,
        US_CA_LAW,
    ),
    earlier_years_unread("Native American Day", None, 1999, US_CA, US_CA_LAW),
    state(
        "Day after Thanksgiving",
        FRIDAY_AFTER_THANKSGIVING,
        2026,
        US_CA,
        US_CA_LAW,
    ),
    earlier_years_unread("Day after Thanksgiving", None, 2026, US_CA, US_CA_LAW),
    // Colorado.
    state(
        "Frances Xavier Cabrini Day",
        Rule::nth(10, 1, Weekday::Monday),
        2020,
        US_CO,
        US_CO_LAW,
    ),
    // Connecticut.
    state(
        "Lincoln Day",
        Rule::gregorian(2, 12),
        2026,
        US_CT,
        US_CT_LAW,
    ),
    earlier_years_unread("Lincoln Day", None, 2026, US_CT, US_CT_LAW),
    // The District of Columbia.
    state(
        "District of Columbia Emancipation Day",
        Rule::gregorian(4, 16),
        2005,
        US_DC,
        US_DC_LAW,
    ),
    // Delaware.
    state(
        "Good Friday",
        Rule::easter(GOOD_FRIDAY),
        2026,
        US_DE,
        US_DE_LAW,
    ),
    earlier_years_unread("Good Friday", None, 2026, US_DE, US_DE_LAW),
    state(
        "Friday after Thanksgiving",
        FRIDAY_AFTER_THANKSGIVING,
        2026,
        US_DE,
        US_DE_LAW,
    ),
    earlier_years_unread("Friday after Thanksgiving", None, 2026, US_DE, US_DE_LAW),
    state(
        "General Election Day",
        Rule::Computed(even_year_election_day),
        2026,
        US_DE,
        US_DE_LAW,
    ),
    earlier_years_unread("General Election Day", None, 2026, US_DE, US_DE_LAW),
    // Florida.
    state_observance(
        "Birthday of Martin Luther King, Jr.",
        Rule::gregorian(1, 15),
        2026,
        US_FL,
        US_FL_LAW,
    ),
    earlier_years_unread(
        "Birthday of Martin Luther King, Jr.",
        None,
        2026,
        US_FL,
        US_FL_LAW,
    ),
    state_observance(
        "Birthday of Robert E. Lee",
        Rule::gregorian(1, 19),
        2026,
        US_FL,
        US_FL_LAW,
    ),
    earlier_years_unread("Birthday of Robert E. Lee", None, 2026, US_FL, US_FL_LAW),
    state_observance(
        "Lincoln's Birthday",
        Rule::gregorian(2, 12),
        2026,
        US_FL,
        US_FL_LAW,
    ),
    earlier_years_unread("Lincoln's Birthday", None, 2026, US_FL, US_FL_LAW),
    state_observance(
        "Susan B. Anthony's Birthday",
        Rule::gregorian(2, 15),
        2026,
        US_FL,
        US_FL_LAW,
    ),
    earlier_years_unread("Susan B. Anthony's Birthday", None, 2026, US_FL, US_FL_LAW),
    state_observance(
        "Tuskegee Airmen Commemoration Day",
        Rule::nth(3, 4, Weekday::Thursday),
        2026,
        US_FL,
        US_FL_LAW,
    ),
    earlier_years_unread(
        "Tuskegee Airmen Commemoration Day",
        None,
        2026,
        US_FL,
        US_FL_LAW,
    ),
    state_observance(
        "Good Friday",
        Rule::easter(GOOD_FRIDAY),
        2026,
        US_FL,
        US_FL_LAW,
    ),
    earlier_years_unread("Good Friday", None, 2026, US_FL, US_FL_LAW),
    state_observance(
        "Pascua Florida Day",
        Rule::gregorian(4, 2),
        1953,
        US_FL,
        US_FL_LAW,
    ),
    state_observance(
        "Confederate Memorial Day",
        Rule::gregorian(4, 26),
        2026,
        US_FL,
        US_FL_LAW,
    ),
    earlier_years_unread("Confederate Memorial Day", None, 2026, US_FL, US_FL_LAW),
    state_observance(
        "Birthday of Jefferson Davis",
        Rule::gregorian(6, 3),
        2026,
        US_FL,
        US_FL_LAW,
    ),
    earlier_years_unread("Birthday of Jefferson Davis", None, 2026, US_FL, US_FL_LAW),
    state_observance("Flag Day", Rule::gregorian(6, 14), 2026, US_FL, US_FL_LAW),
    earlier_years_unread("Flag Day", None, 2026, US_FL, US_FL_LAW),
    state_observance(
        "General Election Day",
        Rule::Computed(even_year_election_day),
        2026,
        US_FL,
        US_FL_LAW,
    ),
    earlier_years_unread("General Election Day", None, 2026, US_FL, US_FL_LAW),
    state(
        "Friday after Thanksgiving",
        FRIDAY_AFTER_THANKSGIVING,
        2022,
        US_FL,
        US_FL_LAW,
    ),
    earlier_years_unread("Friday after Thanksgiving", None, 2022, US_FL, US_FL_LAW),
    // Hawaii.
    state(
        "Prince Jonah Kuhio Kalanianaole Day",
        Rule::gregorian(3, 26),
        2001,
        US_HI,
        US_HI_LAW,
    ),
    earlier_years_unread(
        "Prince Jonah Kuhio Kalanianaole Day",
        None,
        2001,
        US_HI,
        US_HI_LAW,
    ),
    state(
        "Good Friday",
        Rule::easter(GOOD_FRIDAY),
        2001,
        US_HI,
        US_HI_LAW,
    ),
    earlier_years_unread("Good Friday", None, 2001, US_HI, US_HI_LAW),
    state(
        "King Kamehameha I Day",
        Rule::gregorian(6, 11),
        2001,
        US_HI,
        US_HI_LAW,
    ),
    earlier_years_unread("King Kamehameha I Day", None, 2001, US_HI, US_HI_LAW),
    state(
        "Statehood Day",
        Rule::nth(8, 3, Weekday::Friday),
        2001,
        US_HI,
        US_HI_LAW,
    ),
    earlier_years_unread("Statehood Day", None, 2001, US_HI, US_HI_LAW),
    state(
        "General Election Day",
        Rule::Computed(even_year_election_day),
        2001,
        US_HI,
        US_HI_LAW,
    ),
    earlier_years_unread("General Election Day", None, 2001, US_HI, US_HI_LAW),
    // Iowa.
    state_observance(
        "Lincoln's Birthday",
        Rule::gregorian(2, 12),
        1993,
        US_IA,
        US_IA_LAW,
    ),
    earlier_years_unread("Lincoln's Birthday", None, 1993, US_IA, US_IA_LAW),
    state(
        "Friday after Thanksgiving",
        FRIDAY_AFTER_THANKSGIVING,
        2008,
        US_IA,
        US_IA_LAW,
    ),
    earlier_years_unread("Friday after Thanksgiving", None, 2008, US_IA, US_IA_LAW),
    // Idaho.
    state_observance(
        "Constitutional Commemorative Day",
        Rule::gregorian(9, 17),
        1989,
        US_ID,
        US_ID_LAW,
    ),
    state_observance(
        "Children's Day",
        Rule::gregorian(4, 30),
        2003,
        US_ID,
        US_ID_LAW,
    ),
    state_observance("Idaho Day", Rule::gregorian(3, 4), 2014, US_ID, US_ID_LAW),
    // Illinois.
    state_observance(
        "Lincoln's Birthday",
        Rule::gregorian(2, 12),
        2022,
        US_IL,
        US_IL_LAW,
    ),
    earlier_years_unread("Lincoln's Birthday", None, 2022, US_IL, US_IL_LAW),
    state_observance(
        "Casimir Pulaski's Birthday",
        Rule::nth(3, 1, Weekday::Monday),
        2022,
        US_IL,
        US_IL_LAW,
    ),
    earlier_years_unread("Casimir Pulaski's Birthday", None, 2022, US_IL, US_IL_LAW),
    state_observance(
        "Good Friday",
        Rule::easter(GOOD_FRIDAY),
        2022,
        US_IL,
        US_IL_LAW,
    ),
    earlier_years_unread("Good Friday", None, 2022, US_IL, US_IL_LAW),
    state_observance(
        "General Election Day",
        Rule::Computed(even_year_election_day),
        2022,
        US_IL,
        US_IL_LAW,
    ),
    earlier_years_unread("General Election Day", None, 2022, US_IL, US_IL_LAW),
    // Indiana.
    state(
        "Lincoln's Birthday",
        Rule::gregorian(2, 12),
        2026,
        US_IN,
        US_IN_LAW,
    ),
    earlier_years_unread("Lincoln's Birthday", None, 2026, US_IN, US_IN_LAW),
    state(
        "Good Friday",
        Rule::easter(GOOD_FRIDAY),
        2026,
        US_IN,
        US_IN_LAW,
    ),
    earlier_years_unread("Good Friday", None, 2026, US_IN, US_IN_LAW),
    state("Election Day", ELECTION_DAY, 2026, US_IN, US_IN_LAW),
    earlier_years_unread("Election Day", None, 2026, US_IN, US_IN_LAW),
    // Kansas.
    state_observance(
        "General Pulaski's Memorial Day",
        Rule::gregorian(10, 11),
        1935,
        US_KS,
        US_KS_LAW,
    ),
    state_observance(
        "Family Day",
        Rule::WeekdayOnOrAfter {
            month: 11,
            day: 25,
            weekday: Weekday::Sunday,
        },
        1971,
        US_KS,
        US_KS_LAW,
    ),
    state_observance(
        "Pearl Harbor Remembrance Day",
        Rule::gregorian(12, 7),
        1988,
        US_KS,
        US_KS_LAW,
    ),
    state_observance(
        "Dwight D. Eisenhower Day",
        Rule::gregorian(10, 14),
        1999,
        US_KS,
        US_KS_LAW,
    ),
    state_observance(
        "Native American Day",
        Rule::nth(9, 4, Weekday::Saturday),
        2013,
        US_KS,
        US_KS_LAW,
    ),
    earlier_years_unread("Native American Day", Some(1945), 2013, US_KS, US_KS_LAW),
    state_observance(
        "National Day of the Cowboy",
        Rule::nth(7, 4, Weekday::Saturday),
        2014,
        US_KS,
        US_KS_LAW,
    ),
    // Kentucky.
    state_observance(
        "Robert E. Lee Day",
        Rule::gregorian(1, 19),
        2025,
        US_KY,
        US_KY_LAW,
    ),
    earlier_years_unread("Robert E. Lee Day", None, 2025, US_KY, US_KY_LAW),
    state_observance(
        "Franklin D. Roosevelt Day",
        Rule::gregorian(1, 30),
        2025,
        US_KY,
        US_KY_LAW,
    ),
    earlier_years_unread("Franklin D. Roosevelt Day", None, 2025, US_KY, US_KY_LAW),
    state_observance(
        "Lincoln's Birthday",
        Rule::gregorian(2, 12),
        2025,
        US_KY,
        US_KY_LAW,
    ),
    earlier_years_unread("Lincoln's Birthday", None, 2025, US_KY, US_KY_LAW),
    state_observance(
        "Confederate Memorial Day and Jefferson Davis Day",
        Rule::gregorian(6, 3),
        2025,
        US_KY,
        US_KY_LAW,
    ),
    earlier_years_unread(
        "Confederate Memorial Day and Jefferson Davis Day",
        None,
        2025,
        US_KY,
        US_KY_LAW,
    ),
    state(
        "Presidential Election Day",
        Rule::Computed(presidential_election_day),
        2025,
        US_KY,
        US_KY_LAW,
    ),
    earlier_years_unread("Presidential Election Day", None, 2025, US_KY, US_KY_LAW),
    // Louisiana.
    state_observance(
        "Battle of New Orleans",
        Rule::gregorian(1, 8),
        2026,
        US_LA,
        US_LA_LAW,
    ),
    earlier_years_unread("Battle of New Orleans", None, 2026, US_LA, US_LA_LAW),
    state(
        "Mardi Gras",
        Rule::easter(SHROVE_TUESDAY),
        2026,
        US_LA,
        US_LA_LAW,
    ),
    earlier_years_unread("Mardi Gras", None, 2026, US_LA, US_LA_LAW),
    state(
        "Good Friday",
        Rule::easter(GOOD_FRIDAY),
        2026,
        US_LA,
        US_LA_LAW,
    ),
    earlier_years_unread("Good Friday", None, 2026, US_LA, US_LA_LAW),
    state_observance(
        "Huey P. Long Day",
        Rule::gregorian(8, 30),
        2026,
        US_LA,
        US_LA_LAW,
    ),
    earlier_years_unread("Huey P. Long Day", None, 2026, US_LA, US_LA_LAW),
    state_observance(
        "All Saints' Day",
        Rule::gregorian(11, 1),
        2026,
        US_LA,
        US_LA_LAW,
    ),
    earlier_years_unread("All Saints' Day", None, 2026, US_LA, US_LA_LAW),
    // Massachusetts.
    state(
        "Patriots' Day",
        Rule::nth(4, 3, Weekday::Monday),
        2025,
        US_MA,
        US_MA_LAW,
    ),
    earlier_years_unread("Patriots' Day", None, 2025, US_MA, US_MA_LAW),
    // Maryland.
    state_observance(
        "Lincoln's Birthday",
        Rule::gregorian(2, 12),
        2026,
        US_MD,
        US_MD_LAW,
    ),
    earlier_years_unread("Lincoln's Birthday", None, 2026, US_MD, US_MD_LAW),
    state_observance(
        "Maryland Day",
        Rule::gregorian(3, 25),
        2026,
        US_MD,
        US_MD_LAW,
    ),
    earlier_years_unread("Maryland Day", None, 2026, US_MD, US_MD_LAW),
    state_observance(
        "Good Friday",
        Rule::easter(GOOD_FRIDAY),
        2026,
        US_MD,
        US_MD_LAW,
    ),
    earlier_years_unread("Good Friday", None, 2026, US_MD, US_MD_LAW),
    state_observance(
        "Defenders' Day",
        Rule::gregorian(9, 12),
        2026,
        US_MD,
        US_MD_LAW,
    ),
    earlier_years_unread("Defenders' Day", None, 2026, US_MD, US_MD_LAW),
    state(
        "American Indian Heritage Day",
        FRIDAY_AFTER_THANKSGIVING,
        2026,
        US_MD,
        US_MD_LAW,
    ),
    earlier_years_unread("American Indian Heritage Day", None, 2026, US_MD, US_MD_LAW),
    state(
        "General Election Day",
        Rule::Computed(even_year_election_day),
        2026,
        US_MD,
        US_MD_LAW,
    ),
    earlier_years_unread("General Election Day", None, 2026, US_MD, US_MD_LAW),
    // Maine.
    state(
        "Patriot's Day",
        Rule::nth(4, 3, Weekday::Monday),
        2026,
        US_ME,
        US_ME_LAW,
    ),
    earlier_years_unread("Patriot's Day", None, 2026, US_ME, US_ME_LAW),
    // Michigan.
    state_observance(
        "Lincoln's Birthday",
        Rule::gregorian(2, 12),
        2025,
        US_MI,
        US_MI_LAW,
    ),
    earlier_years_unread("Lincoln's Birthday", None, 2025, US_MI, US_MI_LAW),
    // Minnesota.
    state(
        "Friday after Thanksgiving",
        FRIDAY_AFTER_THANKSGIVING,
        2025,
        US_MN,
        US_MN_LAW,
    ),
    earlier_years_unread("Friday after Thanksgiving", None, 2025, US_MN, US_MN_LAW),
    // Missouri.
    state(
        "Lincoln Day",
        Rule::gregorian(2, 12),
        2022,
        US_MO,
        US_MO_LAW,
    ),
    earlier_years_unread("Lincoln Day", None, 2022, US_MO, US_MO_LAW),
    state("Truman Day", Rule::gregorian(5, 8), 2022, US_MO, US_MO_LAW),
    earlier_years_unread("Truman Day", None, 2022, US_MO, US_MO_LAW),
    // Mississippi.
    state(
        "Confederate Memorial Day",
        Rule::last(4, Weekday::Monday),
        2025,
        US_MS,
        US_MS_LAW,
    ),
    earlier_years_unread("Confederate Memorial Day", None, 2025, US_MS, US_MS_LAW),
    // Montana.
    state(
        "General Election Day",
        Rule::Computed(even_year_election_day),
        2025,
        US_MT,
        US_MT_LAW,
    ),
    earlier_years_unread("General Election Day", None, 2025, US_MT, US_MT_LAW),
    // North Carolina.
    state_observance(
        "Robert E. Lee's Birthday",
        Rule::gregorian(1, 19),
        2023,
        US_NC,
        US_NC_LAW,
    ),
    earlier_years_unread("Robert E. Lee's Birthday", None, 2023, US_NC, US_NC_LAW),
    state_observance(
        "Greek Independence Day",
        Rule::gregorian(3, 25),
        2023,
        US_NC,
        US_NC_LAW,
    ),
    earlier_years_unread("Greek Independence Day", None, 2023, US_NC, US_NC_LAW),
    state_observance(
        "Anniversary of the Halifax Resolves",
        Rule::gregorian(4, 12),
        2023,
        US_NC,
        US_NC_LAW,
    ),
    earlier_years_unread(
        "Anniversary of the Halifax Resolves",
        None,
        2023,
        US_NC,
        US_NC_LAW,
    ),
    state_observance(
        "Confederate Memorial Day",
        Rule::gregorian(5, 10),
        2023,
        US_NC,
        US_NC_LAW,
    ),
    earlier_years_unread("Confederate Memorial Day", None, 2023, US_NC, US_NC_LAW),
    state_observance(
        "Anniversary of the Mecklenburg Declaration of Independence",
        Rule::gregorian(5, 20),
        2023,
        US_NC,
        US_NC_LAW,
    ),
    earlier_years_unread(
        "Anniversary of the Mecklenburg Declaration of Independence",
        None,
        2023,
        US_NC,
        US_NC_LAW,
    ),
    state_observance(
        "Good Friday",
        Rule::easter(GOOD_FRIDAY),
        2023,
        US_NC,
        US_NC_LAW,
    ),
    earlier_years_unread("Good Friday", None, 2023, US_NC, US_NC_LAW),
    state_observance(
        "First Responders Day",
        Rule::gregorian(9, 11),
        2023,
        US_NC,
        US_NC_LAW,
    ),
    earlier_years_unread("First Responders Day", None, 2023, US_NC, US_NC_LAW),
    state_observance(
        "Yom Kippur",
        Rule::in_calendar(CalendarSystem::HEBREW, 1, 10),
        2023,
        US_NC,
        US_NC_LAW,
    ),
    earlier_years_unread("Yom Kippur", None, 2023, US_NC, US_NC_LAW),
    state_observance(
        "Election Day",
        Rule::Computed(even_year_election_day),
        2023,
        US_NC,
        US_NC_LAW,
    ),
    earlier_years_unread("Election Day", None, 2023, US_NC, US_NC_LAW),
    // North Dakota.
    state(
        "Good Friday",
        Rule::easter(GOOD_FRIDAY),
        2024,
        US_ND,
        US_ND_LAW,
    ),
    earlier_years_unread("Good Friday", None, 2024, US_ND, US_ND_LAW),
    // Nebraska.
    state(
        "Arbor Day",
        Rule::last(4, Weekday::Friday),
        2024,
        US_NE,
        US_NE_LAW,
    ),
    earlier_years_unread("Arbor Day", None, 2024, US_NE, US_NE_LAW),
    state(
        "Day after Thanksgiving",
        FRIDAY_AFTER_THANKSGIVING,
        2024,
        US_NE,
        US_NE_LAW,
    ),
    earlier_years_unread("Day after Thanksgiving", None, 2024, US_NE, US_NE_LAW),
    // New Jersey.
    state_observance(
        "Lincoln's Birthday",
        Rule::gregorian(2, 12),
        2024,
        US_NJ,
        US_NJ_LAW,
    ),
    earlier_years_unread("Lincoln's Birthday", None, 2024, US_NJ, US_NJ_LAW),
    state(
        "Good Friday",
        Rule::easter(GOOD_FRIDAY),
        2024,
        US_NJ,
        US_NJ_LAW,
    ),
    earlier_years_unread("Good Friday", None, 2024, US_NJ, US_NJ_LAW),
    state(
        "Juneteenth Day",
        Rule::nth(6, 3, Weekday::Friday),
        2024,
        US_NJ,
        US_NJ_LAW,
    ),
    earlier_years_unread("Juneteenth Day", None, 2024, US_NJ, US_NJ_LAW),
    state("General Election Day", ELECTION_DAY, 2024, US_NJ, US_NJ_LAW),
    earlier_years_unread("General Election Day", None, 2024, US_NJ, US_NJ_LAW),
    // New Mexico.
    state_observance(
        "American Indian Day",
        Rule::nth(2, 1, Weekday::Friday),
        2024,
        US_NM,
        US_NM_LAW,
    ),
    earlier_years_unread("American Indian Day", None, 2024, US_NM, US_NM_LAW),
    state_observance(
        "Guadalupe Hidalgo Treaty Day",
        Rule::gregorian(2, 2),
        2024,
        US_NM,
        US_NM_LAW,
    ),
    earlier_years_unread("Guadalupe Hidalgo Treaty Day", None, 2024, US_NM, US_NM_LAW),
    state_observance(
        "African-American Day",
        Rule::nth(2, 2, Weekday::Friday),
        2024,
        US_NM,
        US_NM_LAW,
    ),
    earlier_years_unread("African-American Day", None, 2024, US_NM, US_NM_LAW),
    state_observance(
        "Arbor Day",
        Rule::nth(3, 2, Weekday::Friday),
        2024,
        US_NM,
        US_NM_LAW,
    ),
    earlier_years_unread("Arbor Day", None, 2024, US_NM, US_NM_LAW),
    state_observance("Bataan Day", Rule::gregorian(4, 9), 2024, US_NM, US_NM_LAW),
    earlier_years_unread("Bataan Day", None, 2024, US_NM, US_NM_LAW),
    state_observance(
        "Ernie Pyle Day",
        Rule::gregorian(8, 3),
        2024,
        US_NM,
        US_NM_LAW,
    ),
    earlier_years_unread("Ernie Pyle Day", None, 2024, US_NM, US_NM_LAW),
    // Nevada.
    state(
        "Nevada Day",
        Rule::last(10, Weekday::Friday),
        2025,
        US_NV,
        US_NV_LAW,
    ),
    earlier_years_unread("Nevada Day", None, 2025, US_NV, US_NV_LAW),
    state(
        "Family Day",
        FRIDAY_AFTER_THANKSGIVING,
        2025,
        US_NV,
        US_NV_LAW,
    ),
    earlier_years_unread("Family Day", None, 2025, US_NV, US_NV_LAW),
    // New York.
    state(
        "Lincoln's Birthday",
        Rule::gregorian(2, 12),
        2020,
        US_NY,
        US_NY_LAW,
    ),
    earlier_years_unread("Lincoln's Birthday", None, 2020, US_NY, US_NY_LAW),
    state(
        "Flag Day",
        Rule::nth(6, 2, Weekday::Sunday),
        2020,
        US_NY,
        US_NY_LAW,
    ),
    earlier_years_unread("Flag Day", None, 2020, US_NY, US_NY_LAW),
    state("General Election Day", ELECTION_DAY, 2020, US_NY, US_NY_LAW),
    earlier_years_unread("General Election Day", None, 2020, US_NY, US_NY_LAW),
    // Oregon.
    state_observance(
        "Oregon Statehood Day",
        Rule::gregorian(2, 14),
        2015,
        US_OR,
        US_OR_LAW,
    ),
    // Pennsylvania.
    state_observance(
        "Good Friday",
        Rule::easter(GOOD_FRIDAY),
        2026,
        US_PA,
        US_PA_LAW,
    ),
    earlier_years_unread("Good Friday", None, 2026, US_PA, US_PA_LAW),
    state_observance("Flag Day", Rule::gregorian(6, 14), 2026, US_PA, US_PA_LAW),
    earlier_years_unread("Flag Day", None, 2026, US_PA, US_PA_LAW),
    state_observance("Election Day", ELECTION_DAY, 2026, US_PA, US_PA_LAW),
    earlier_years_unread("Election Day", None, 2026, US_PA, US_PA_LAW),
    // Rhode Island.
    state(
        "Rhode Island Independence Day",
        Rule::gregorian(5, 4),
        2026,
        US_RI,
        US_RI_LAW,
    ),
    earlier_years_unread(
        "Rhode Island Independence Day",
        None,
        2026,
        US_RI,
        US_RI_LAW,
    ),
    state(
        "Victory Day",
        Rule::nth(8, 2, Weekday::Monday),
        2026,
        US_RI,
        US_RI_LAW,
    ),
    earlier_years_unread("Victory Day", None, 2026, US_RI, US_RI_LAW),
    state(
        "Election Day",
        Rule::Computed(even_year_election_day),
        2026,
        US_RI,
        US_RI_LAW,
    ),
    earlier_years_unread("Election Day", None, 2026, US_RI, US_RI_LAW),
    // South Carolina.
    state(
        "Confederate Memorial Day",
        Rule::gregorian(5, 10),
        2010,
        US_SC,
        US_SC_LAW,
    ),
    earlier_years_unread("Confederate Memorial Day", None, 2010, US_SC, US_SC_LAW),
    state(
        "Day after Thanksgiving",
        FRIDAY_AFTER_THANKSGIVING,
        2009,
        US_SC,
        US_SC_LAW,
    ),
    earlier_years_unread("Day after Thanksgiving", None, 2009, US_SC, US_SC_LAW),
    state(
        "Christmas Eve",
        Rule::gregorian(12, 24),
        2009,
        US_SC,
        US_SC_LAW,
    ),
    earlier_years_unread("Christmas Eve", None, 2009, US_SC, US_SC_LAW),
    state(
        "26 December",
        Rule::gregorian(12, 26),
        2009,
        US_SC,
        US_SC_LAW,
    ),
    earlier_years_unread("26 December", None, 2009, US_SC, US_SC_LAW),
    // South Dakota.
    state_observance(
        "South Dakota Statehood Day",
        Rule::gregorian(11, 2),
        2001,
        US_SD,
        US_SD_LAW,
    ),
    state_observance(
        "Little Big Horn Recognition Day",
        Rule::gregorian(6, 25),
        1994,
        US_SD,
        US_SD_LAW,
    ),
    state_observance(
        "Wounded Knee Day",
        Rule::gregorian(12, 29),
        1994,
        US_SD,
        US_SD_LAW,
    ),
    state_observance(
        "Bill of Rights Day",
        Rule::gregorian(12, 15),
        1998,
        US_SD,
        US_SD_LAW,
    ),
    state_observance(
        "Joe Foss Day",
        Rule::gregorian(4, 17),
        2004,
        US_SD,
        US_SD_LAW,
    ),
    state_observance(
        "Purple Heart Recognition Day",
        Rule::gregorian(8, 7),
        2013,
        US_SD,
        US_SD_LAW,
    ),
    state_observance(
        "Welcome Home Vietnam Veterans Day",
        Rule::gregorian(3, 30),
        2013,
        US_SD,
        US_SD_LAW,
    ),
    state_observance(
        "Day of the American Cowboy",
        Rule::nth(7, 4, Weekday::Saturday),
        2014,
        US_SD,
        US_SD_LAW,
    ),
    state_observance(
        "Peter Norbeck Day",
        Rule::gregorian(8, 27),
        2018,
        US_SD,
        US_SD_LAW,
    ),
    state_observance(
        "Medal of Honor Recognition Day",
        Rule::gregorian(3, 25),
        2024,
        US_SD,
        US_SD_LAW,
    ),
    // Tennessee.
    state(
        "Good Friday",
        Rule::easter(GOOD_FRIDAY),
        2024,
        US_TN,
        US_TN_LAW,
    ),
    earlier_years_unread("Good Friday", None, 2024, US_TN, US_TN_LAW),
    // Texas.
    state(
        "Confederate Heroes Day",
        Rule::gregorian(1, 19),
        2025,
        US_TX,
        US_TX_LAW,
    ),
    earlier_years_unread("Confederate Heroes Day", None, 2025, US_TX, US_TX_LAW),
    state(
        "Texas Independence Day",
        Rule::gregorian(3, 2),
        2025,
        US_TX,
        US_TX_LAW,
    ),
    earlier_years_unread("Texas Independence Day", None, 2025, US_TX, US_TX_LAW),
    state(
        "San Jacinto Day",
        Rule::gregorian(4, 21),
        2025,
        US_TX,
        US_TX_LAW,
    ),
    earlier_years_unread("San Jacinto Day", None, 2025, US_TX, US_TX_LAW),
    state(
        "Lyndon Baines Johnson Day",
        Rule::gregorian(8, 27),
        2025,
        US_TX,
        US_TX_LAW,
    ),
    earlier_years_unread("Lyndon Baines Johnson Day", None, 2025, US_TX, US_TX_LAW),
    state(
        "Friday after Thanksgiving",
        FRIDAY_AFTER_THANKSGIVING,
        2025,
        US_TX,
        US_TX_LAW,
    ),
    earlier_years_unread("Friday after Thanksgiving", None, 2025, US_TX, US_TX_LAW),
    state(
        "24 December",
        Rule::gregorian(12, 24),
        2025,
        US_TX,
        US_TX_LAW,
    ),
    earlier_years_unread("24 December", None, 2025, US_TX, US_TX_LAW),
    state(
        "26 December",
        Rule::gregorian(12, 26),
        2025,
        US_TX,
        US_TX_LAW,
    ),
    earlier_years_unread("26 December", None, 2025, US_TX, US_TX_LAW),
    // Utah.
    state(
        "Pioneer Day",
        Rule::gregorian(7, 24),
        2025,
        US_UT,
        US_UT_LAW,
    ),
    earlier_years_unread("Pioneer Day", None, 2025, US_UT, US_UT_LAW),
    state(
        "Juneteenth National Freedom Day",
        Rule::moved_by_weekday(&JUNE_19, TO_PRECEDING_OR_NEXT_MONDAY),
        2025,
        US_UT,
        US_UT_LAW,
    ),
    earlier_years_unread(
        "Juneteenth National Freedom Day",
        None,
        2025,
        US_UT,
        US_UT_LAW,
    ),
    // Virginia.
    state("Election Day", ELECTION_DAY, 2020, US_VA, US_VA_LAW),
    state(
        "Day after Thanksgiving",
        FRIDAY_AFTER_THANKSGIVING,
        2020,
        US_VA,
        US_VA_LAW,
    ),
    earlier_years_unread("Day after Thanksgiving", None, 2020, US_VA, US_VA_LAW),
    // Vermont.
    state(
        "Town Meeting Day",
        Rule::nth(3, 1, Weekday::Tuesday),
        2024,
        US_VT,
        US_VT_LAW,
    ),
    earlier_years_unread("Town Meeting Day", None, 2024, US_VT, US_VT_LAW),
    state(
        "Bennington Battle Day",
        Rule::gregorian(8, 16),
        2024,
        US_VT,
        US_VT_LAW,
    ),
    earlier_years_unread("Bennington Battle Day", None, 2024, US_VT, US_VT_LAW),
    // Washington.
    state(
        "Native American Heritage Day",
        FRIDAY_AFTER_THANKSGIVING,
        2014,
        US_WA,
        US_WA_LAW,
    ),
    earlier_years_unread("Native American Heritage Day", None, 2014, US_WA, US_WA_LAW),
    // Wisconsin.
    state(
        "General Election Day",
        Rule::Computed(even_year_election_day),
        2025,
        US_WI,
        US_WI_LAW,
    ),
    earlier_years_unread("General Election Day", None, 2025, US_WI, US_WI_LAW),
    // West Virginia.
    state(
        "West Virginia Day",
        Rule::gregorian(6, 20),
        2026,
        US_WV,
        US_WV_LAW,
    ),
    earlier_years_unread("West Virginia Day", None, 2026, US_WV, US_WV_LAW),
    state(
        "Lincoln's Day",
        FRIDAY_AFTER_THANKSGIVING,
        2026,
        US_WV,
        US_WV_LAW,
    ),
    earlier_years_unread("Lincoln's Day", None, 2026, US_WV, US_WV_LAW),
];

/// A day an executive order closed the executive departments and excused
/// their employees, in the one year it names.
const fn us_closure(month: u8, day: u8, year: i32, order: &'static str) -> HolidayRule {
    HolidayRule::fixed_public(
        "Closing of Executive Departments",
        "",
        Rule::gregorian(month, day),
    )
    .years(Some(year), Some(year))
    .cited(order)
}

/// The federal "in lieu of" rule: Executive Order 11582 of 1971 codified
/// what Executive Order 10358 had begun in 1959 — a Saturday holiday is kept
/// the preceding Friday, a Sunday holiday the following Monday.
static US_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Nearest,
    skip_occupied: false,
    on_collision: false,
    valid_from: Some(1959),
    valid_until: None,
}];

/// The United States: the federal holidays of 5 U.S.C. § 6103.
pub static UNITED_STATES: RuleSet = RuleSet {
    code: "US",
    english_name: "United States",
    rules: US_RULES,
    substitution: US_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "5 U.S.C. § 6103(a) to (c), on the Legal Information Institute \
              (law.cornell.edu/uscode/text/5/6103), retrieved 2026-09-26, whose subsection (b) \
              states the in-lieu-of rule; Pub. L. 90-363 (1968, the Uniform Monday Holiday \
              Act), Pub. L. 94-97 (1975) restoring Veterans Day, Pub. L. 98-144 (1983) for \
              Martin Luther King Jr. Day and Pub. L. 117-17 (2021) for Juneteenth, not read; \
              Executive Orders 10358 and 11582 on the in-lieu-of rule before it was in the \
              statute, not read. The full-day closures by executive order from 2018 — \
              Executive Orders 13852, 13854, 13900, 13965, 14129, 14133 and 14371 — from the \
              Federal Register's documents API (federalregister.gov/api/v1), retrieved \
              2026-09-26, each cited on its entry; the closures before 2018, the Christmas \
              ones listed there from 1997 and the funeral days alike, and the half-day \
              closures, are not carried. The states' own days from their codes, each cited on \
              its entries, read 2026-09-29, as docs/systems/us-state-holidays.md lists them; \
              New Hampshire's, Oklahoma's and Georgia's not carried",
};
