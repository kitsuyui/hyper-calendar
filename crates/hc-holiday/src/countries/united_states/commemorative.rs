//! The states' commemorative days: the days of observance their codes
//! designate beside the legal holidays, which close nothing.
//!
//! Each is an observance, [`Kind::Observance`](crate::rule::Kind::Observance),
//! scoped to its state, cited to the section it was read in, from the
//! session law that set it where the section's history names one (the years
//! before absent), and otherwise from the year of the version read (the
//! years before a gap). A day whose section was read only through a
//! summarising fetch, and not verified against its text, is marked
//! [`Confidence::Approximate`](crate::rule::Confidence::Approximate), and its
//! citation says so. The chapters read, and the days they designate that
//! no calendar rule can date, are in `docs/systems/us-state-holidays.md`.

use hc_calendar::Weekday;

use super::state_observance;
use crate::rule::{HolidayRule, Rule};

// The sections read, one citation per state and site.
const US_AL_DAYS: &str = "Code of Alabama 1975 § 1-3-8(g) to Code of Alabama 1975 § 1-3-9(b), the official site, alison.legislature.state.al.us (https://alison.legislature.state.al.us/code-of-alabama?section=1-3-8 and the sections after it), retrieved 2026-09-29";
const US_AK_DAYS: &str = "Alaska Stat. § 44.12.030 to Alaska Stat. § 44.12.195, a secondary copy on codes.findlaw.com (https://codes.findlaw.com/ak/title-44-state-government/ak-st-sect-44-12-030/ and the sections after it), retrieved 2026-09-29";
const US_AZ_DAYS: &str = "A.R.S. § 1-304 to A.R.S. § 1-321, the official site, azleg.gov (https://www.azleg.gov/ars/1/00304.htm and the sections after it), retrieved 2026-09-29";
const US_AR_DAYS: &str = "Ark. Code Ann. § 1-5-106 to Ark. Code Ann. § 1-5-122, a secondary copy on codes.findlaw.com (https://codes.findlaw.com/ar/title-1-general-provisions/ar-code-sect-1-5-106/ and the sections after it), retrieved 2026-09-29";
const US_CA_DAYS: &str = "Cal. Gov. Code § 6708 to Cal. Gov. Code § 6736, the official site, leginfo.legislature.ca.gov (https://leginfo.legislature.ca.gov/faces/codes_displayText.xhtml?lawCode=GOV&division=7.&title=1.&part=&chapter=7.&article=), retrieved 2026-09-29";
const US_CO_DAYS: &str = "C.R.S. § 24-11-104 to C.R.S. § 24-11-116, a secondary copy on colorado.public.law (https://colorado.public.law/statutes/crs_24-11-104 and the sections after it), retrieved 2026-09-29";
const US_CO_DAYS_2: &str = "C.R.S. § 24-11-117 to C.R.S. § 24-11-118, a secondary copy on codes.findlaw.com (https://codes.findlaw.com/co/title-24-government-state/co-rev-st-sect-24-11-117/ and the sections after it), retrieved 2026-09-29";
const US_CT_DAYS: &str = "Conn. Gen. Stat. § 10-29a(a)(2) to Conn. Gen. Stat. § 10-29a(a)(117), the official site, cga.ct.gov, in the Internet Archive's capture (https://www.cga.ct.gov/current/pub/chap_164.htm), retrieved 2026-09-29";
const US_CT_DAYS_2: &str = "Conn. Gen. Stat. § 10-29a(a)(118) (2026 Supp.) to Conn. Gen. Stat. § 10-29a(a)(138) (2026 Supp.), the official site, cga.ct.gov (https://www.cga.ct.gov/2026/sup/chap_164.htm), retrieved 2026-09-29";
const US_DE_DAYS: &str = "1 Del. C. § 601 to 1 Del. C. § 611, the official site, delcode.delaware.gov (https://delcode.delaware.gov/title1/c006/index.html), retrieved 2026-09-29";
const US_FL_DAYS: &str = "Fla. Stat. § 683.04 to Fla. Stat. § 683.335, the official site, leg.state.fl.us (https://www.leg.state.fl.us/statutes/index.cfm?App_mode=Display_Statute&URL=0600-0699/0683/0683.html), retrieved 2026-09-29";
const US_GA_DAYS: &str = "O.C.G.A. § 1-4-5 to O.C.G.A. § 1-4-26, a secondary copy on codes.findlaw.com (https://codes.findlaw.com/ga/title-1-general-provisions/ga-code-sect-1-4-5/ and the sections after it), retrieved 2026-09-29";
const US_HI_DAYS: &str = "HRS § 8-3.4 to HRS § 8-42, the official site, data.capitol.hawaii.gov (https://data.capitol.hawaii.gov/hrscurrent/Vol01_Ch0001-0042F/HRS0008/HRS_0008-0003_0004.htm and the sections after it), retrieved 2026-09-29";
const US_IL_DAYS: &str = "5 ILCS 490/2 to 5 ILCS 490/196, a secondary copy on web.archive.org, in the Internet Archive's capture (https://web.archive.org/web/20260306143627/https://www.ilga.gov/Legislation/ILCS/Articles?ActID=134&ChapterID=2&Chapter=GENERAL%20PROVISIONS&MajorTopic=GOVERNMENT&Print=True), retrieved 2026-09-29";
const US_IN_DAYS: &str = "IC 1-1-10-1 to IC 1-1-14-1, a secondary copy on codes.findlaw.com, read through a summarising fetch, not verified against the text (https://codes.findlaw.com/in/title-1-general-provisions/in-code-sect-1-1-10-1/ and the sections after it), retrieved 2026-09-29";
const US_IA_DAYS: &str = "Iowa Code § 1C.3 to Iowa Code § 1C.17, the official site, legis.iowa.gov (https://www.legis.iowa.gov/docs/code/2026/1C.html), retrieved 2026-09-29";
const US_KY_DAYS: &str = "KRS 2.112 to KRS 2.235, a secondary copy on codes.findlaw.com, first read through a summarising fetch, then verified against the text of the Internet Archive's capture (https://codes.findlaw.com/ky/title-i-sovereignty-and-jurisdiction-of-the-commonwealth/ky-rev-st-sect-2-112/ and the sections after it), retrieved 2026-09-29";
const US_KY_DAYS_2: &str = "KRS 2.120 to KRS 2.236, a secondary copy on codes.findlaw.com, read through a summarising fetch, not verified against the text (https://codes.findlaw.com/ky/title-i-sovereignty-and-jurisdiction-of-the-commonwealth/ky-rev-st-sect-2-120/ and the sections after it), retrieved 2026-09-29";
const US_LA_DAYS: &str = "La. R.S. 1:56 to La. R.S. 1:58.10, the official site, legis.la.gov (https://legis.la.gov/Legis/Law.aspx?d=74098 and the sections after it), retrieved 2026-09-29";
const US_ME_DAYS: &str = "1 M.R.S. § 112 to 1 M.R.S. § 150-W, the official site, legislature.maine.gov (https://legislature.maine.gov/statutes/1/title1sec112.html and the sections after it), retrieved 2026-09-29";
const US_MD_DAYS: &str = "Md. Code, Gen. Prov. § 7-402 to Md. Code, Gen. Prov. § 7-421, the official site, mgaleg.maryland.gov (https://mgaleg.maryland.gov/mgawebsite/Laws/StatuteText?article=ggp&section=7-402&enactments=false and the sections after it), retrieved 2026-09-29";
const US_MA_DAYS: &str = "M.G.L. c. 6, § 12B to M.G.L. c. 6, § 15AAAAAAA, a secondary copy on web.archive.org, in the Internet Archive's capture (https://web.archive.org/web/2026/https://malegislature.gov/Laws/GeneralLaws/PartI/TitleII/Chapter6/Section12B and the sections after it), retrieved 2026-09-29";
const US_MA_DAYS_2: &str = "M.G.L. c. 6, § 13 to M.G.L. c. 6, § 15BBBBBBB, a secondary copy on codes.findlaw.com, in the Internet Archive's capture, read through a summarising fetch, not verified against the text (https://codes.findlaw.com/ma/part-i-administration-of-the-government-ch-1-182/ma-gen-laws-ch-6-sect-13/ and the sections after it), retrieved 2026-09-29";
const US_MA_DAYS_3: &str = "M.G.L. c. 6, § 14, a secondary copy on codes.findlaw.com, in the Internet Archive's capture, first read through a summarising fetch, then verified against the text of the Internet Archive's capture (https://codes.findlaw.com/ma/part-i-administration-of-the-government-ch-1-182/ma-gen-laws-ch-6-sect-14/), retrieved 2026-09-29";
const US_MI_DAYS: &str = "MCL 435.111 to MCL 435.401, a secondary copy on codes.findlaw.com, read through a summarising fetch, not verified against the text (https://codes.findlaw.com/mi/chapter-435-sundays-and-holidays/mi-comp-laws-435-111/ and the sections after it), retrieved 2026-09-29";
const US_MI_DAYS_2: &str = "MCL 435.302 to MCL 435.361, a secondary copy on codes.findlaw.com, first read through a summarising fetch, then verified against the text of the Internet Archive's capture (https://codes.findlaw.com/mi/chapter-435-sundays-and-holidays/mi-comp-laws-435-302/ and the sections after it), retrieved 2026-09-29";
const US_MN_DAYS: &str = "Minn. Stat. § 10.50 to Minn. Stat. § 10.597, the official site, revisor.mn.gov (https://www.revisor.mn.gov/statutes/cite/10/full), retrieved 2026-09-29";
const US_MS_DAYS: &str = "Miss. Code § 3-3-7(3) to Miss. Code § 3-3-7(7), a secondary copy on codes.findlaw.com, read through a summarising fetch, not verified against the text (https://codes.findlaw.com/ms/title-3-state-sovereignty-jurisdiction-and-holidays/ms-code-sect-3-3-7/), retrieved 2026-09-29";
const US_MO_DAYS: &str = "RSMo § 9.005 to RSMo § 9.516, the official site, revisor.mo.gov (https://revisor.mo.gov/main/ViewChapter.aspx?chapter=9), retrieved 2026-09-29";
const US_MT_DAYS: &str = "MCA 1-1-225 to MCA 1-1-233, the official site, mca.legmt.gov (https://mca.legmt.gov/bills/mca/title_0010/chapter_0010/part_0020/section_0250/0010-0010-0020-0250.html and the sections after it), retrieved 2026-09-29";
const US_NE_DAYS: &str = "Neb. Rev. Stat. § 84-104.04 to Neb. Rev. Stat. § 84-108, a secondary copy on codes.findlaw.com, read through a summarising fetch, not verified against the text (https://codes.findlaw.com/ne/chapter-84-state-officers/ne-rev-st-sect-84-104-04/ and the sections after it), retrieved 2026-09-29";
const US_NV_DAYS: &str = "NRS 236.018 to NRS 236.074, a secondary copy on codes.findlaw.com, read through a summarising fetch, not verified against the text (https://codes.findlaw.com/nv/title-19-miscellaneous-matters-related-to-government-and-public-affairs/nv-rev-st-236-018/ and the sections after it), retrieved 2026-09-29";
const US_NJ_DAYS: &str = "N.J.S.A. 36:2-1 to N.J.S.A. 36:2-457, a secondary copy on codes.findlaw.com, read through a summarising fetch, not verified against the text (https://codes.findlaw.com/nj/title-36-legal-holidays/nj-st-sect-36-2-1/ and the sections after it), retrieved 2026-09-29";
const US_NJ_DAYS_2: &str = "N.J.S.A. 36:2-130 to N.J.S.A. 36:2-255, a secondary copy on codes.findlaw.com, first read through a summarising fetch, then verified against the text of the Internet Archive's capture (https://codes.findlaw.com/nj/title-36-legal-holidays/nj-st-sect-36-2-130/ and the sections after it), retrieved 2026-09-29";
const US_NY_DAYS: &str = "N.Y. Exec. Law § 168-a(3), the official site, nysenate.gov (https://www.nysenate.gov/legislation/laws/EXC/168-A), retrieved 2026-09-29";
const US_NC_DAYS: &str = "N.C. Gen. Stat. § 103-7 to N.C. Gen. Stat. § 103-18, a secondary copy on ncleg.gov, in the Internet Archive's capture (https://www.ncleg.gov/EnactedLegislation/Statutes/HTML/ByChapter/Chapter_103.html), retrieved 2026-09-29";
const US_ND_DAYS: &str = "N.D. Cent. Code § 1-03-06 to N.D. Cent. Code § 1-03-23, a secondary copy on codes.findlaw.com, read through a summarising fetch, not verified against the text (https://codes.findlaw.com/nd/title-1-general-provisions/nd-cent-code-sect-1-03-06/ and the sections after it), retrieved 2026-09-29";
const US_ND_DAYS_2: &str = "N.D. Cent. Code § 1-03-08 to N.D. Cent. Code § 1-03-20, a secondary copy on codes.findlaw.com, first read through a summarising fetch, then verified against the text of the Internet Archive's capture (https://codes.findlaw.com/nd/title-1-general-provisions/nd-cent-code-sect-1-03-08/ and the sections after it), retrieved 2026-09-29";
const US_OH_DAYS: &str = "Ohio Rev. Code § 5.22 to Ohio Rev. Code § 5.61, a secondary copy on codes.ohio.gov, in the Internet Archive's capture (https://codes.ohio.gov/ohio-revised-code/chapter-5), retrieved 2026-09-29";
const US_OR_DAYS: &str = "ORS 187.206 to ORS 187.323, a secondary copy on oregonlegislature.gov, in the Internet Archive's capture (https://www.oregonlegislature.gov/bills_laws/ors/ors187.html), retrieved 2026-09-29";
const US_PA_DAYS: &str = "44 P.S. § 22 to 44 P.S. § 40.11, a secondary copy on codes.findlaw.com, read through a summarising fetch, not verified against the text (https://codes.findlaw.com/pa/title-44-ps-legal-holidays-and-observances/pa-st-sect-44-22/ and the sections after it), retrieved 2026-09-29";
const US_RI_DAYS: &str = "R.I. Gen. Laws § 25-2-4 to R.I. Gen. Laws § 25-2-14, a secondary copy on webserver.rilegislature.gov, in the Internet Archive's capture (https://webserver.rilegislature.gov/Statutes/TITLE25/25-2/25-2-4.htm and the sections after it), retrieved 2026-09-29";
const US_SC_DAYS: &str = "S.C. Code § 53-3-10 to S.C. Code § 53-3-330, the official site, scstatehouse.gov (https://www.scstatehouse.gov/code/t53c003.php), retrieved 2026-09-29";
const US_SD_DAYS: &str = "SDCL 1-5-10 to SDCL 1-5-13, the official site, sdlegislature.gov (https://sdlegislature.gov/api/Statutes/1-5.html?all=true), retrieved 2026-09-29";
const US_TN_DAYS: &str = "Tenn. Code Ann. § 15-2-101 to Tenn. Code Ann. § 15-2-159, a secondary copy on codes.findlaw.com, read through a summarising fetch, not verified against the text (https://codes.findlaw.com/tn/title-15-holidays-and-days-of-special-observance/tn-code-sect-15-2-101/ and the sections after it), retrieved 2026-09-29";
const US_TX_DAYS: &str = "Tex. Gov't Code § 662.041 to Tex. Gov't Code § 662.089, the official site, tcss.legis.texas.gov (https://tcss.legis.texas.gov/resources/GV/htm/GV.662.htm), retrieved 2026-09-29";
const US_UT_DAYS: &str = "Utah Code § 63G-1-401(4)(a) to Utah Code § 63G-1-401(4)(p), the official site, le.utah.gov, read through a summarising fetch, not verified against the text (https://le.utah.gov/xcode/Title63G/Chapter1/C63G-1-S401_2026050620260506.html), retrieved 2026-09-29";
const US_VT_DAYS: &str = "1 V.S.A. § 372 to 1 V.S.A. § 377(a), a secondary copy on legislature.vermont.gov, in the Internet Archive's capture (https://legislature.vermont.gov/statutes/fullchapter/01/007), retrieved 2026-09-29";
const US_VA_DAYS: &str = "Va. Code § 2.2-3302 to Va. Code § 2.2-3319, the official site, law.lis.virginia.gov (https://law.lis.virginia.gov/vacode/title2.2/chapter33/section2.2-3302/ and the sections after it), retrieved 2026-09-29";
const US_WA_DAYS: &str = "RCW 1.16.050(7)(a) to RCW 1.16.050(7)(t), the official site, app.leg.wa.gov (https://app.leg.wa.gov/RCW/default.aspx?cite=1.16.050), retrieved 2026-09-29";
const US_WV_DAYS: &str = "W. Va. Code § 2-2-1a(b) to W. Va. Code § 2-2-1a(h), the official site, code.wvlegislature.gov (https://code.wvlegislature.gov/2-2-1a/), retrieved 2026-09-29";
const US_WI_DAYS: &str = "Wis. Stat. § 995.22 to Wis. Stat. § 995.30, a secondary copy on docs.legis.wisconsin.gov, in the Internet Archive's capture (https://docs.legis.wisconsin.gov/document/statutes/995.22 and the sections after it), retrieved 2026-09-29";
const US_WY_DAYS: &str = "Wyo. Stat. § 8-4-102 to Wyo. Stat. § 8-4-116, a secondary copy on codes.findlaw.com, read through a summarising fetch, not verified against the text (https://codes.findlaw.com/wy/title-8-general-provisions/wy-st-sect-8-4-102/ and the sections after it), retrieved 2026-09-29";

/// The commemorative days, state by state.
pub(super) const DAYS: &[HolidayRule] = &[
    // Alabama.
    state_observance(
        "Mrs. Rosa L. Parks Day",
        Rule::gregorian(12, 1),
        &["US-AL"],
        US_AL_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Alabama Peace Officers’ Memorial Day",
        Rule::nth(5, 1, Weekday::Friday),
        &["US-AL"],
        US_AL_DAYS,
    )
    .years(Some(2023), None),
    // Alaska.
    state_observance(
        "Wickersham Day",
        Rule::gregorian(8, 24),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Anthony J. Dimond Day",
        Rule::gregorian(11, 30),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Ernest Gruening Day",
        Rule::gregorian(2, 6),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Bob Bartlett Day",
        Rule::gregorian(4, 20),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "William A. Egan Day",
        Rule::gregorian(10, 8),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Walter Harper Day",
        Rule::gregorian(6, 7),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Katie John Day",
        Rule::gregorian(5, 31),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Elizabeth Peratrovich Day",
        Rule::gregorian(2, 16),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Susan Butcher Day",
        Rule::nth(3, 1, Weekday::Saturday),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Ted Stevens Day",
        Rule::nth(7, 4, Weekday::Saturday),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Vietnam Veterans Day",
        Rule::gregorian(3, 29),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Alaska Flag Day",
        Rule::gregorian(7, 9),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Alaska Day of Prayer",
        Rule::nth(5, 1, Weekday::Thursday),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Fetal Alcohol Spectrum Disorders Awareness Day",
        Rule::gregorian(9, 9),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance("Family Day", Rule::gregorian(5, 1), &["US-AK"], US_AK_DAYS).read_from(2025),
    state_observance(
        "Former Prisoners of War Recognition Day",
        Rule::gregorian(4, 9),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Prisoners of War and Missing in Action Recognition Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Military Family Day",
        Rule::gregorian(11, 1),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Women Veterans Day",
        Rule::gregorian(11, 9),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Pearl Harbor Remembrance Day",
        Rule::gregorian(12, 7),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Alaska Territorial Guard Day",
        Rule::gregorian(10, 18),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Dutch Harbor Remembrance Day",
        Rule::gregorian(6, 3),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Great Alaska Good Friday Earthquake Remembrance Day",
        Rule::gregorian(3, 27),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Alaska Mining Day",
        Rule::gregorian(5, 10),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Drunk Driving Victims Remembrance Day",
        Rule::gregorian(7, 3),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Older Alaskans' Day",
        Rule::nth(9, 2, Weekday::Wednesday),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Alaska Agriculture Day",
        Rule::nth(5, 1, Weekday::Tuesday),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "4-H Day",
        Rule::nth(10, 1, Weekday::Wednesday),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Children's Day",
        Rule::nth(6, 2, Weekday::Sunday),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance("Marmot Day", Rule::gregorian(2, 2), &["US-AK"], US_AK_DAYS).read_from(2025),
    state_observance(
        "Purple Heart Day",
        Rule::gregorian(8, 7),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Alaska Public Gardens Day",
        Rule::WeekdayOnOrBefore {
            month: 5,
            day: 29,
            weekday: Weekday::Saturday,
        },
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Alaska Wild Salmon Day",
        Rule::gregorian(8, 10),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Alaska National Guard Day",
        Rule::gregorian(7, 30),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Jay Hammond Day",
        Rule::gregorian(7, 21),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Dr. Walter Soboleff Day",
        Rule::gregorian(11, 14),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Vic Fischer and Jack Coghill Constitution of the State of Alaska Day",
        Rule::gregorian(4, 24),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Alaska Law Enforcement Officers' Day",
        Rule::gregorian(1, 9),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Alaska Community Health Aide Appreciation Day",
        Rule::gregorian(9, 10),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Alaska Firefighters' Day",
        Rule::WeekdayOnOrBefore {
            month: 10,
            day: 9,
            weekday: Weekday::Sunday,
        },
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "African American Soldiers' Contribution to Building the Alaska Highway Day",
        Rule::gregorian(10, 25),
        &["US-AK"],
        US_AK_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "Hmong-American Veterans Day",
        Rule::gregorian(5, 15),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Ashley Johnson-Barr Day",
        Rule::gregorian(3, 12),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Myalgic Encephalomyelitis/Chronic Fatigue Syndrome Day of Recognition",
        Rule::gregorian(5, 12),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Patriot Day",
        Rule::gregorian(9, 11),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Post-Traumatic Stress Injury Awareness Day",
        Rule::gregorian(6, 27),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Don Young Day",
        Rule::gregorian(6, 9),
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Alaska Veterans' Poppy Day",
        Rule::WeekdayOnOrBefore {
            month: 5,
            day: 28,
            weekday: Weekday::Friday,
        },
        &["US-AK"],
        US_AK_DAYS,
    )
    .read_from(2025),
    // Arizona.
    state_observance(
        "Arbor Day",
        Rule::last(4, Weekday::Friday),
        &["US-AZ"],
        US_AZ_DAYS,
    )
    .read_from(2026),
    state_observance(
        "National day of the cowboy",
        Rule::nth(7, 4, Weekday::Saturday),
        &["US-AZ"],
        US_AZ_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Korean war veterans' day",
        Rule::gregorian(7, 27),
        &["US-AZ"],
        US_AZ_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Prisoners of war remembrance day",
        Rule::gregorian(4, 9),
        &["US-AZ"],
        US_AZ_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Boy scouts of America day",
        Rule::gregorian(2, 8),
        &["US-AZ"],
        US_AZ_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Girl scouts of the United States of America day",
        Rule::gregorian(3, 12),
        &["US-AZ"],
        US_AZ_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Tuskegee airmen commemoration day",
        Rule::nth(3, 4, Weekday::Thursday),
        &["US-AZ"],
        US_AZ_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Vietnam veterans' day",
        Rule::gregorian(3, 29),
        &["US-AZ"],
        US_AZ_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Arizona first responders' day of gratitude and remembrance",
        Rule::gregorian(9, 27),
        &["US-AZ"],
        US_AZ_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Purple heart day",
        Rule::gregorian(8, 7),
        &["US-AZ"],
        US_AZ_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Beirut marine barracks bombing remembrance day",
        Rule::gregorian(10, 23),
        &["US-AZ"],
        US_AZ_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Public lands day",
        Rule::nth(4, 1, Weekday::Saturday),
        &["US-AZ"],
        US_AZ_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Sandra Day O'Connor civics celebration day",
        Rule::gregorian(9, 25),
        &["US-AZ"],
        US_AZ_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Fred Korematsu day of civil liberties and the constitution",
        Rule::gregorian(1, 30),
        &["US-AZ"],
        US_AZ_DAYS,
    )
    .read_from(2026),
    state_observance(
        "9/11 education day",
        Rule::gregorian(9, 11),
        &["US-AZ"],
        US_AZ_DAYS,
    )
    .read_from(2026),
    // Arkansas.
    state_observance(
        "General Douglas MacArthur Day",
        Rule::gregorian(1, 26),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Silas Hunt Day",
        Rule::gregorian(2, 2),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Abraham Lincoln's Birthday",
        Rule::gregorian(2, 12),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Arkansas Teachers' Day",
        Rule::nth(3, 1, Weekday::Tuesday),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Arbor Day",
        Rule::nth(3, 3, Weekday::Monday),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Patriots' Day",
        Rule::gregorian(4, 19),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Arkansas Bird Day",
        Rule::gregorian(4, 26),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance("Good Friday", Rule::easter(-2), &["US-AR"], US_AR_DAYS).read_from(2024),
    state_observance(
        "Jefferson Davis' Birthday",
        Rule::gregorian(6, 3),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Columbus Day",
        Rule::gregorian(10, 12),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Senator Hattie W. Caraway Day",
        Rule::gregorian(12, 19),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Robert E. Lee Day",
        Rule::nth(10, 2, Weekday::Saturday),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "John H. Johnson Day",
        Rule::gregorian(11, 1),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Confederate Flag Day",
        Rule::easter(-1),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "White Cane Safety Day",
        Rule::gregorian(10, 15),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Prisoners of War Remembrance Day",
        Rule::gregorian(4, 9),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Arkansas Agriculture Recognition Day",
        Rule::nth(3, 1, Weekday::Friday),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "POW/MIA Recognition Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Juneteenth Independence Day",
        Rule::nth(6, 3, Weekday::Saturday),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Firefighter Recognition Day",
        Rule::gregorian(1, 27),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Hemophilia Awareness Day",
        Rule::nth(5, 1, Weekday::Monday),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Arkansas Music Appreciation Day",
        Rule::gregorian(9, 1),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "National Day of the Cowboy",
        Rule::nth(7, 4, Weekday::Saturday),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Sultana Disaster Remembrance Day",
        Rule::gregorian(4, 27),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "John R. “Johnny” Cash Day",
        Rule::gregorian(2, 26),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Lupus Awareness Day",
        Rule::gregorian(4, 23),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Women Veterans Day",
        Rule::gregorian(6, 12),
        &["US-AR"],
        US_AR_DAYS,
    )
    .read_from(2024),
    // California.
    state_observance(
        "Cabrillo Day",
        Rule::gregorian(9, 28),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(1963), None),
    state_observance(
        "Dr. Martin Luther King, Jr., Day",
        Rule::gregorian(1, 15),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(1978), None),
    state_observance("Arbor Day", Rule::gregorian(3, 7), &["US-CA"], US_CA_DAYS)
        .years(Some(1974), None),
    state_observance(
        "A Day of Remembrance: Japanese American Evacuation",
        Rule::gregorian(2, 19),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(1979), None),
    state_observance(
        "Stepparents Day",
        Rule::nth(10, 1, Weekday::Sunday),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(1985), None),
    state_observance(
        "John Muir Day",
        Rule::gregorian(4, 21),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(1989), None),
    state_observance(
        "Welcome Home Vietnam Veterans Day",
        Rule::gregorian(3, 30),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Pearl Harbor Day",
        Rule::gregorian(12, 7),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(1989), None),
    state_observance(
        "Juneteenth National Freedom Day: A day of observance",
        Rule::nth(6, 3, Weekday::Saturday),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(2003), None),
    state_observance(
        "Harvey Milk Day",
        Rule::gregorian(5, 22),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Fred Korematsu Day of Civil Liberties and the Constitution",
        Rule::gregorian(1, 30),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(2010), None),
    state_observance(
        "Ronald Reagan Day",
        Rule::gregorian(2, 6),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(2010), None),
    state_observance(
        "Ed Roberts Day",
        Rule::gregorian(1, 23),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(2010), None),
    state_observance(
        "Larry Itliong Day",
        Rule::gregorian(10, 25),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Space Day",
        Rule::nth(5, 1, Weekday::Friday),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(2016), None),
    state_observance(
        "Dolores Huerta Day",
        Rule::gregorian(4, 10),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(2018), None),
    state_observance(
        "California Farmworker Day",
        Rule::gregorian(8, 26),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(2021), None),
    state_observance(
        "Transgender Day of Remembrance",
        Rule::gregorian(11, 20),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "World AIDS Day",
        Rule::gregorian(12, 1),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(2024), None),
    state_observance(
        "Dolly Parton Day",
        Rule::gregorian(9, 25),
        &["US-CA"],
        US_CA_DAYS,
    )
    .years(Some(2026), None),
    // Colorado.
    state_observance(
        "Arbor Day",
        Rule::nth(4, 3, Weekday::Friday),
        &["US-CO"],
        US_CO_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Susan B. Anthony Day",
        Rule::gregorian(2, 15),
        &["US-CO"],
        US_CO_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Leif Erikson Day",
        Rule::gregorian(10, 9),
        &["US-CO"],
        US_CO_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Colorado Day",
        Rule::nth(8, 1, Weekday::Monday),
        &["US-CO"],
        US_CO_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Cesar Chavez Day",
        Rule::gregorian(3, 31),
        &["US-CO"],
        US_CO_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Public Lands Day",
        Rule::nth(5, 3, Weekday::Saturday),
        &["US-CO"],
        US_CO_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Welcome Home Vietnam Veterans Day",
        Rule::gregorian(3, 30),
        &["US-CO"],
        US_CO_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Lunar New Year Day",
        Rule::nth(2, 1, Weekday::Friday),
        &["US-CO"],
        US_CO_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Jury Appreciation Day",
        Rule::gregorian(9, 5),
        &["US-CO"],
        US_CO_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Women Veterans Appreciation Day",
        Rule::gregorian(6, 12),
        &["US-CO"],
        US_CO_DAYS_2,
    )
    .read_from(2025),
    state_observance(
        "Living Organ Donor Recognition Day",
        Rule::gregorian(4, 11),
        &["US-CO"],
        US_CO_DAYS_2,
    )
    .read_from(2025),
    // Connecticut.
    state_observance(
        "Pan American Day",
        Rule::gregorian(4, 14),
        &["US-CT"],
        US_CT_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Arbor Day",
        Rule::last(4, Weekday::Friday),
        &["US-CT"],
        US_CT_DAYS,
    )
    .read_from(2025),
    state_observance("Loyalty Day", Rule::gregorian(5, 1), &["US-CT"], US_CT_DAYS).read_from(2025),
    state_observance(
        "Senior Citizens Day",
        Rule::nth(5, 1, Weekday::Sunday),
        &["US-CT"],
        US_CT_DAYS,
    )
    .read_from(2025),
    state_observance("Flag Day", Rule::gregorian(6, 14), &["US-CT"], US_CT_DAYS).read_from(2025),
    state_observance(
        "School Safety Patrol Day",
        Rule::nth(9, 2, Weekday::Monday),
        &["US-CT"],
        US_CT_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Nathan Hale Day",
        Rule::gregorian(9, 22),
        &["US-CT"],
        US_CT_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Indian Day",
        Rule::last(9, Weekday::Friday),
        &["US-CT"],
        US_CT_DAYS,
    )
    .read_from(2025),
    state_observance(
        "Puerto Rico Day",
        Rule::nth(9, 4, Weekday::Sunday),
        &["US-CT"],
        US_CT_DAYS,
    )
    .read_from(2025),
    state_observance(
        "St. Patrick's Day",
        Rule::gregorian(3, 17),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1983), None),
    state_observance(
        "German-American Day",
        Rule::gregorian(10, 6),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1984), None),
    state_observance(
        "Friends Day",
        Rule::nth(4, 4, Weekday::Sunday),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1984), None),
    state_observance(
        "Ukrainian-American Day",
        Rule::gregorian(8, 24),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1987), None),
    state_observance(
        "Retired Teachers Day",
        Rule::nth(2, 3, Weekday::Wednesday),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1989), None),
    state_observance(
        "End of World War II Day",
        Rule::gregorian(8, 14),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1989), None),
    state_observance(
        "Honor Our Heroes and Remembrance Day",
        Rule::gregorian(9, 11),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2002), None),
    state_observance(
        "Workers' Memorial Day",
        Rule::gregorian(4, 28),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1990), None),
    state_observance(
        "Disability Awareness Day",
        Rule::gregorian(7, 26),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1991), None),
    state_observance(
        "Volunteer Firefighter and Volunteer Emergency Medical Services Personnel Day",
        Rule::nth(8, 1, Weekday::Saturday),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1991), None),
    state_observance(
        "Women's Independence Day",
        Rule::gregorian(8, 26),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1991), None),
    state_observance(
        "Destroyer Escort Day",
        Rule::nth(6, 3, Weekday::Saturday),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1993), None),
    state_observance(
        "Iwo Jima Day",
        Rule::gregorian(2, 23),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1993), None),
    state_observance(
        "Korean Armistice Day",
        Rule::gregorian(7, 27),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1994), None),
    state_observance(
        "Prudence Crandall Day",
        Rule::gregorian(9, 3),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1994), None),
    state_observance(
        "Polish-American Day",
        Rule::gregorian(5, 3),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1995), None),
    state_observance(
        "Green Up Day",
        Rule::last(4, Weekday::Saturday),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1995), None),
    state_observance(
        "Romanian-American Day",
        Rule::gregorian(12, 1),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1996), None),
    state_observance(
        "Republic of China on Taiwan-American Day",
        Rule::gregorian(10, 10),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1996), None),
    state_observance(
        "Austrian-American Day",
        Rule::gregorian(5, 15),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1996), None),
    state_observance(
        "Greek-American Day",
        Rule::gregorian(3, 25),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1996), None),
    state_observance(
        "Hungarian Freedom Fighters Day",
        Rule::gregorian(10, 23),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1996), None),
    state_observance(
        "National Children's Day",
        Rule::nth(10, 2, Weekday::Sunday),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1996), None),
    state_observance(
        "Youth to Work Day",
        Rule::nth(2, 2, Weekday::Wednesday),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1996), None),
    state_observance(
        "Christa Corrigan McAuliffe Day",
        Rule::gregorian(5, 24),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1996), None),
    state_observance(
        "Gulf War Veterans Day",
        Rule::gregorian(2, 28),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1997), None),
    state_observance(
        "Long Island Sound Day",
        Rule::WeekdayOnOrBefore {
            month: 5,
            day: 28,
            weekday: Weekday::Friday,
        },
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1997), None),
    state_observance(
        "Family Day",
        Rule::nth(9, 2, Weekday::Sunday),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(1997), None),
    state_observance(
        "Connecticut Aviation Pioneer Day",
        Rule::gregorian(5, 25),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2003), None),
    state_observance(
        "Juneteenth Independence Day",
        Rule::WeekdayOnOrAfter {
            month: 6,
            day: 16,
            weekday: Weekday::Saturday,
        },
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2003), None),
    state_observance(
        "Corsair Day",
        Rule::gregorian(5, 29),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2005), None),
    state_observance(
        "Frederick Law Olmsted Day",
        Rule::gregorian(4, 26),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2005), None),
    state_observance(
        "Missing Persons Day",
        Rule::gregorian(8, 23),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Fibromyalgia Awareness Day",
        Rule::gregorian(5, 12),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Fragile X Awareness Day",
        Rule::gregorian(9, 13),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Self Injury Awareness Day",
        Rule::gregorian(3, 1),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Thomas Paine Day",
        Rule::gregorian(1, 29),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Canada Appreciation Day",
        Rule::gregorian(7, 1),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Welcome Home Vietnam Veterans Day",
        Rule::gregorian(3, 30),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2010), None),
    state_observance(
        "French Canadian-American Day",
        Rule::gregorian(6, 24),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2013), None),
    state_observance(
        "First Responder Day",
        Rule::gregorian(9, 27),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2014), None),
    state_observance(
        "Are You Dense? Breast Cancer Awareness Day",
        Rule::gregorian(10, 30),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2014), None),
    state_observance(
        "Neurological Disorders Awareness Day",
        Rule::gregorian(10, 9),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2014), None),
    state_observance(
        "Spinal Muscular Atrophy with Respiratory Distress Awareness Day",
        Rule::gregorian(2, 10),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Safe Haven Day",
        Rule::gregorian(4, 4),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Trigeminal Neuralgia Awareness Day",
        Rule::gregorian(10, 7),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "Bob Hope Day",
        Rule::gregorian(5, 29),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "Purebred Dog Day",
        Rule::gregorian(5, 1),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "PJ Day",
        Rule::nth(12, 2, Weekday::Friday),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "Patriots' Day",
        Rule::nth(4, 3, Weekday::Monday),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "National K9 Veterans' Day",
        Rule::gregorian(3, 13),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "Fibrodysplasia Ossificans Progressiva Awareness Day",
        Rule::gregorian(11, 26),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "Cable Technician Recognition Day",
        Rule::gregorian(9, 8),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2018), None),
    state_observance(
        "Military Spouses' Day",
        Rule::gregorian(11, 12),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2018), None),
    state_observance(
        "Sikh Genocide Remembrance Day",
        Rule::gregorian(11, 1),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2018), None),
    state_observance(
        "Cadet Nurse Corps Day",
        Rule::gregorian(6, 15),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "22q11.2 Deletion Syndrome Day",
        Rule::gregorian(11, 22),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Diffuse Intrinsic Pontine Glioma Day",
        Rule::gregorian(5, 17),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Thirteenth Amendment Day",
        Rule::gregorian(12, 6),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Clubfoot Day",
        Rule::gregorian(6, 3),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Encephalitis Day",
        Rule::gregorian(2, 22),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "U.S.S. Indianapolis CA-35 Day",
        Rule::gregorian(7, 30),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Moyamoya Awareness Day",
        Rule::gregorian(5, 6),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Connecticut Race Amity Day",
        Rule::nth(6, 2, Weekday::Sunday),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2021), None),
    state_observance(
        "Xeroderma Pigmentosum Awareness Day",
        Rule::gregorian(5, 13),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2021), None),
    state_observance(
        "Maternal Mental Health Day",
        Rule::gregorian(5, 5),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Get Outside and Play For Children's Mental Health Day",
        Rule::gregorian(5, 26),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Ann Petry Day",
        Rule::gregorian(5, 10),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Trinity College Day",
        Rule::gregorian(5, 16),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Bosnian Genocide Remembrance Day",
        Rule::gregorian(7, 11),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Free Enterprise Day",
        Rule::gregorian(9, 14),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Constitution Day",
        Rule::gregorian(9, 17),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "PANS and PANDAS Awareness Day",
        Rule::gregorian(10, 9),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Intellectual and Developmental Disabilities Awareness and Advocacy Day",
        Rule::gregorian(5, 23),
        &["US-CT"],
        US_CT_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Scouting America Day",
        Rule::gregorian(2, 8),
        &["US-CT"],
        US_CT_DAYS_2,
    )
    .years(Some(2025), None),
    state_observance(
        "Neurofibromatosis Awareness Day",
        Rule::gregorian(2, 14),
        &["US-CT"],
        US_CT_DAYS_2,
    )
    .years(Some(2025), None),
    state_observance(
        "Parkinson's Awareness Day",
        Rule::gregorian(4, 11),
        &["US-CT"],
        US_CT_DAYS_2,
    )
    .years(Some(2025), None),
    state_observance(
        "Tuskegee Airmen Day",
        Rule::gregorian(4, 26),
        &["US-CT"],
        US_CT_DAYS_2,
    )
    .years(Some(2025), None),
    state_observance(
        "Local Journalism Appreciation Day",
        Rule::nth(5, 1, Weekday::Wednesday),
        &["US-CT"],
        US_CT_DAYS_2,
    )
    .years(Some(2025), None),
    state_observance(
        "Red Dress Day",
        Rule::gregorian(5, 5),
        &["US-CT"],
        US_CT_DAYS_2,
    )
    .years(Some(2025), None),
    state_observance(
        "Dystonia Awareness Day",
        Rule::gregorian(5, 10),
        &["US-CT"],
        US_CT_DAYS_2,
    )
    .years(Some(2025), None),
    state_observance(
        "Face Equity Day",
        Rule::gregorian(5, 19),
        &["US-CT"],
        US_CT_DAYS_2,
    )
    .years(Some(2025), None),
    state_observance(
        "Barber Recognition Day",
        Rule::gregorian(6, 1),
        &["US-CT"],
        US_CT_DAYS_2,
    )
    .years(Some(2025), None),
    state_observance(
        "National Women Veterans' Recognition Day",
        Rule::gregorian(6, 12),
        &["US-CT"],
        US_CT_DAYS_2,
    )
    .years(Some(2025), None),
    state_observance(
        "Connecticut Microbiome Day",
        Rule::gregorian(6, 27),
        &["US-CT"],
        US_CT_DAYS_2,
    )
    .years(Some(2025), None),
    state_observance(
        "Connecticut Recipients of the Medal of Honor Day",
        Rule::gregorian(7, 12),
        &["US-CT"],
        US_CT_DAYS_2,
    )
    .years(Some(2025), None),
    state_observance(
        "Lobster Roll Day",
        Rule::nth(9, 3, Weekday::Saturday),
        &["US-CT"],
        US_CT_DAYS_2,
    )
    .years(Some(2025), None),
    state_observance(
        "Varian Fry Day",
        Rule::gregorian(10, 15),
        &["US-CT"],
        US_CT_DAYS_2,
    )
    .years(Some(2025), None),
    state_observance(
        "Connecticut Liver Health Day",
        Rule::gregorian(4, 19),
        &["US-CT"],
        US_CT_DAYS_2,
    )
    .years(Some(2025), None),
    // Delaware.
    state_observance(
        "Arbor Day",
        Rule::last(4, Weekday::Friday),
        &["US-DE"],
        US_DE_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Delaware Day",
        Rule::gregorian(12, 7),
        &["US-DE"],
        US_DE_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Native American Day",
        Rule::WeekdayOnOrAfter {
            month: 9,
            day: 6,
            weekday: Weekday::Saturday,
        },
        &["US-DE"],
        US_DE_DAYS,
    )
    .years(Some(1997), None),
    state_observance(
        "Juneteenth National Freedom Day",
        Rule::nth(6, 3, Weekday::Saturday),
        &["US-DE"],
        US_DE_DAYS,
    )
    .years(Some(2000), None),
    state_observance(
        "Military Spouse Appreciation Day",
        Rule::WeekdayOnOrAfter {
            month: 5,
            day: 6,
            weekday: Weekday::Friday,
        },
        &["US-DE"],
        US_DE_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Vietnam Veterans Day",
        Rule::gregorian(3, 30),
        &["US-DE"],
        US_DE_DAYS,
    )
    .years(Some(2010), None),
    state_observance(
        "6888th Central Postal Directory Battalion Day",
        Rule::gregorian(3, 9),
        &["US-DE"],
        US_DE_DAYS,
    )
    .years(Some(2025), None),
    state_observance(
        "Puerto Rico Day",
        Rule::gregorian(6, 11),
        &["US-DE"],
        US_DE_DAYS,
    )
    .years(Some(2026), None),
    // Florida.
    state_observance(
        "Arbor Day",
        Rule::nth(1, 3, Weekday::Friday),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(1945), None),
    state_observance(
        "Pan-American Day",
        Rule::gregorian(4, 14),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(1945), None),
    state_observance(
        "Grandparents’ and Family Caregivers’ Day",
        Rule::WeekdayOnOrAfter {
            month: 9,
            day: 7,
            weekday: Weekday::Sunday,
        },
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(1971), None),
    state_observance(
        "Law Enforcement Appreciation Day",
        Rule::gregorian(5, 1),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(1972), None),
    state_observance(
        "Law Enforcement Memorial Day",
        Rule::gregorian(5, 15),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(1978), None),
    state_observance(
        "Patriots’ Day",
        Rule::gregorian(4, 19),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(1976), None),
    state_observance(
        "I Am An American Day",
        Rule::nth(10, 3, Weekday::Sunday),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(1987), None),
    state_observance(
        "Purple Heart Day",
        Rule::gregorian(8, 7),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(2012), None),
    state_observance(
        "Medal of Honor Day",
        Rule::gregorian(3, 25),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(2018), None),
    state_observance(
        "Teacher’s Day",
        Rule::nth(5, 3, Weekday::Friday),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(1978), None),
    state_observance(
        "Parents’ and Children’s Day",
        Rule::nth(4, 1, Weekday::Sunday),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(1988), None),
    state_observance(
        "Save the Florida Panther Day",
        Rule::nth(3, 3, Weekday::Saturday),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(1990), None),
    state_observance(
        "Everglades Day",
        Rule::gregorian(4, 7),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(2012), None),
    state_observance(
        "Holocaust Remembrance Day",
        Rule::gregorian(1, 27),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(2025), None),
    state_observance("Law Day", Rule::gregorian(5, 1), &["US-FL"], US_FL_DAYS)
        .years(Some(1998), None),
    state_observance(
        "Florida Missing Children’s Day",
        Rule::nth(9, 2, Weekday::Monday),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(2000), None),
    state_observance(
        "Florida Alzheimer’s Disease Day",
        Rule::gregorian(2, 6),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(2000), None),
    state_observance(
        "Bill of Rights Day",
        Rule::gregorian(12, 15),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(2001), None),
    state_observance(
        "Ronald Reagan Day",
        Rule::gregorian(2, 6),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(2007), None),
    state_observance(
        "Homeless Persons’ Memorial Day",
        Rule::gregorian(12, 21),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(2001), None),
    state_observance(
        "Three Kings Day",
        Rule::gregorian(1, 6),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(2007), None),
    state_observance(
        "Child Welfare Professionals Recognition Day",
        Rule::nth(5, 2, Weekday::Monday),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(2008), None),
    state_observance(
        "Victims of Communism Day",
        Rule::gregorian(11, 7),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Revive Awareness Day",
        Rule::gregorian(6, 6),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(2024), None),
    state_observance(
        "Fentanyl Awareness and Education Day",
        Rule::gregorian(8, 21),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(2025), None),
    state_observance(
        "9/11 Heroes’ Day",
        Rule::gregorian(9, 11),
        &["US-FL"],
        US_FL_DAYS,
    )
    .years(Some(2023), None),
    // Georgia.
    state_observance(
        "Bird Day",
        Rule::nth(10, 2, Weekday::Thursday),
        &["US-GA"],
        US_GA_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Law Enforcement Officer Appreciation Day",
        Rule::nth(2, 2, Weekday::Monday),
        &["US-GA"],
        US_GA_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Peace Officer Memorial Day",
        Rule::gregorian(5, 15),
        &["US-GA"],
        US_GA_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Children's Day",
        Rule::nth(10, 1, Weekday::Sunday),
        &["US-GA"],
        US_GA_DAYS,
    )
    .years(Some(1990), None),
    state_observance(
        "Former Prisoners of War Recognition Day",
        Rule::gregorian(4, 9),
        &["US-GA"],
        US_GA_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Girls and Women in Sports Day",
        Rule::nth(2, 1, Weekday::Thursday),
        &["US-GA"],
        US_GA_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Firefighter Appreciation Day",
        Rule::nth(2, 1, Weekday::Tuesday),
        &["US-GA"],
        US_GA_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Bill of Rights Day",
        Rule::gregorian(12, 15),
        &["US-GA"],
        US_GA_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Bill Elliott Day",
        Rule::gregorian(10, 8),
        &["US-GA"],
        US_GA_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Ronald Reagan Day",
        Rule::gregorian(2, 6),
        &["US-GA"],
        US_GA_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Prison Chaplains Appreciation Day",
        Rule::nth(3, 4, Weekday::Monday),
        &["US-GA"],
        US_GA_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Georgia Day",
        Rule::gregorian(2, 12),
        &["US-GA"],
        US_GA_DAYS,
    )
    .read_from(2024),
    state_observance(
        "School Bus Drivers Appreciation Day",
        Rule::nth(10, 3, Weekday::Monday),
        &["US-GA"],
        US_GA_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Purple Heart Day",
        Rule::gregorian(8, 7),
        &["US-GA"],
        US_GA_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Water Professionals Appreciation Day",
        Rule::nth(5, 1, Weekday::Monday),
        &["US-GA"],
        US_GA_DAYS,
    )
    .read_from(2024),
    state_observance(
        "Childhood Cancer Awareness Day",
        Rule::gregorian(9, 1),
        &["US-GA"],
        US_GA_DAYS,
    )
    .read_from(2024),
    state_observance(
        "National Swearing-in Commitment Day",
        Rule::nth(2, 2, Weekday::Wednesday),
        &["US-GA"],
        US_GA_DAYS,
    )
    .read_from(2024),
    state_observance(
        "First Responders Appreciation Day",
        Rule::gregorian(9, 11),
        &["US-GA"],
        US_GA_DAYS,
    )
    .read_from(2024),
    // Hawaii.
    state_observance(
        "Civil Liberties and the Constitution Day",
        Rule::gregorian(1, 30),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(2013), None),
    state_observance(
        "Patriot Day",
        Rule::gregorian(9, 11),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Gold Star Family Day",
        Rule::last(9, Weekday::Sunday),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(2014), None),
    state_observance("Buddha Day", Rule::gregorian(4, 8), &["US-HI"], US_HI_DAYS)
        .years(Some(1963), None),
    state_observance(
        "Baha'i New Year's Day",
        Rule::gregorian(3, 21),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(1971), None),
    state_observance(
        "Queen Lili‘uokalani Day",
        Rule::gregorian(9, 2),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(1993), None),
    state_observance(
        "Arbor Day",
        Rule::nth(11, 1, Weekday::Friday),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(1979), None),
    state_observance(
        "Saint Damien de Veuster Day",
        Rule::gregorian(5, 10),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(1982), None),
    state_observance(
        "Saint Marianne Cope Day",
        Rule::gregorian(1, 23),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(2014), None),
    state_observance(
        "Respect for Our Elders Day",
        Rule::nth(10, 3, Weekday::Sunday),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(1986), None),
    state_observance("Bodhi Day", Rule::gregorian(12, 8), &["US-HI"], US_HI_DAYS)
        .years(Some(1990), None),
    state_observance(
        "Children and Youth Day",
        Rule::nth(10, 1, Weekday::Sunday),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(1994), None),
    state_observance(
        "World Ocean Day",
        Rule::gregorian(6, 8),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(1998), None),
    state_observance(
        "Water Safety Day",
        Rule::gregorian(5, 15),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(2024), None),
    state_observance("Lei Day", Rule::gregorian(5, 1), &["US-HI"], US_HI_DAYS)
        .years(Some(2001), None),
    state_observance(
        "Kupuna Recognition Day",
        Rule::nth(7, 4, Weekday::Saturday),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(2006), None),
    state_observance("Peace Day", Rule::gregorian(9, 21), &["US-HI"], US_HI_DAYS)
        .years(Some(2007), None),
    state_observance(
        "Mohandas Karamchand Gandhi Day",
        Rule::gregorian(10, 2),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Caregiver Recognition Day",
        Rule::nth(11, 1, Weekday::Saturday),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(2008), None),
    state_observance(
        "Sakada Day",
        Rule::gregorian(12, 20),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "International Yoga Day",
        Rule::gregorian(6, 21),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "La Hoihoi Ea",
        Rule::gregorian(7, 31),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "La Kuokoa, Hawaiian Independence Day",
        Rule::gregorian(11, 28),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Kimchi Day",
        Rule::gregorian(11, 22),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(2024), None),
    state_observance(
        "Laulau Day",
        Rule::nth(5, 1, Weekday::Friday),
        &["US-HI"],
        US_HI_DAYS,
    )
    .years(Some(2025), None),
    // Illinois.
    state_observance(
        "Ronald Reagan Day",
        Rule::gregorian(2, 6),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2002), None),
    state_observance(
        "Barack Obama Day",
        Rule::gregorian(8, 4),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2018), None),
    state_observance(
        "Indigenous Peoples Day",
        Rule::last(9, Weekday::Monday),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "Republic of Ireland Day",
        Rule::gregorian(4, 18),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Arbor and Bird Day",
        Rule::last(4, Weekday::Friday),
        &["US-IL"],
        US_IL_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Chaplains Day",
        Rule::nth(5, 1, Weekday::Sunday),
        &["US-IL"],
        US_IL_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Citizenship Day",
        Rule::nth(5, 3, Weekday::Sunday),
        &["US-IL"],
        US_IL_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Coal Miners Memorial Day",
        Rule::gregorian(11, 13),
        &["US-IL"],
        US_IL_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Great Grandparents Day",
        Rule::WeekdayOnOrAfter {
            month: 9,
            day: 7,
            weekday: Weekday::Sunday,
        },
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2005), None),
    state_observance(
        "D.A.R.E. Day",
        Rule::nth(4, 2, Weekday::Thursday),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2011), None),
    state_observance(
        "Fathers Day",
        Rule::nth(6, 3, Weekday::Sunday),
        &["US-IL"],
        US_IL_DAYS,
    )
    .read_from(2026),
    state_observance("Flag Day", Rule::gregorian(6, 14), &["US-IL"], US_IL_DAYS).read_from(2026),
    state_observance(
        "Gold Star Mothers' Day",
        Rule::last(9, Weekday::Sunday),
        &["US-IL"],
        US_IL_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Gold Star Family Day",
        Rule::WeekdayOnOrBefore {
            month: 9,
            day: 29,
            weekday: Weekday::Saturday,
        },
        &["US-IL"],
        US_IL_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Grandmothers Day",
        Rule::nth(10, 2, Weekday::Sunday),
        &["US-IL"],
        US_IL_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Esther Golar Day",
        Rule::gregorian(4, 16),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2018), None),
    state_observance(
        "Mothers Day",
        Rule::nth(5, 2, Weekday::Sunday),
        &["US-IL"],
        US_IL_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Pearl Harbor Remembrance Day",
        Rule::gregorian(12, 7),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(1995), None),
    state_observance(
        "Senior Citizens Day",
        Rule::nth(5, 3, Weekday::Sunday),
        &["US-IL"],
        US_IL_DAYS,
    )
    .read_from(2026),
    state_observance(
        "September 11th Day of Remembrance",
        Rule::gregorian(9, 11),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2002), None),
    state_observance(
        "G.I. Bill of Rights Day",
        Rule::gregorian(11, 4),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Viet Nam War Veterans Day",
        Rule::gregorian(3, 29),
        &["US-IL"],
        US_IL_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Korean War Armistice Day",
        Rule::gregorian(7, 27),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(1998), None),
    state_observance(
        "POW/MIA Recognition Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(1998), None),
    state_observance(
        "Veterans Gardening Day",
        Rule::nth(5, 1, Weekday::Saturday),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Day of Prayer in Illinois",
        Rule::nth(5, 1, Weekday::Thursday),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(1999), None),
    state_observance(
        "Jane Addams Day",
        Rule::gregorian(12, 10),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2007), None),
    state_observance(
        "Volunteer Emergency Responder Appreciation Day",
        Rule::nth(5, 3, Weekday::Thursday),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2012), None),
    state_observance(
        "Scott's Law Day",
        Rule::gregorian(12, 23),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2016), None),
    state_observance(
        "Diffuse Intrinsic Pontine Glioma (DIPG) Awareness Day",
        Rule::gregorian(5, 17),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Children's Day (El Dia de los Ninos)",
        Rule::nth(6, 2, Weekday::Sunday),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2010), None),
    state_observance(
        "Preventing Lost Potential Day",
        Rule::gregorian(9, 19),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Peace Officers Memorial Day",
        Rule::nth(5, 1, Weekday::Thursday),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2010), None),
    state_observance(
        "National Peace Officers Memorial Day",
        Rule::gregorian(5, 15),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2010), None),
    state_observance(
        "Illinois State Trooper Day",
        Rule::gregorian(4, 1),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "First Responder Mental Health Awareness Day",
        Rule::nth(5, 3, Weekday::Friday),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Adlai Stevenson Day",
        Rule::gregorian(2, 5),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2010), None),
    state_observance(
        "Day of Remembrance of the Victims of Slavery and the Transatlantic Slave Trade",
        Rule::gregorian(3, 25),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2010), None),
    state_observance(
        "Purple Heart Day",
        Rule::gregorian(8, 7),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2012), None),
    state_observance(
        "Diabetes Awareness Day",
        Rule::gregorian(11, 14),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2012), None),
    state_observance(
        "Mother Mary Ann Bickerdyke Day",
        Rule::nth(5, 2, Weekday::Wednesday),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2013), None),
    state_observance(
        "Sweet Corn Appreciation Day",
        Rule::gregorian(8, 1),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Illinois Constitution Day",
        Rule::gregorian(8, 26),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Illinois Statehood Day",
        Rule::gregorian(12, 3),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Day of the Horse",
        Rule::gregorian(3, 5),
        &["US-IL"],
        US_IL_DAYS,
    )
    .years(Some(2018), None),
    // Indiana.
    state_observance(
        "Indiana Day",
        Rule::gregorian(12, 11),
        &["US-IN"],
        US_IN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance("Flag Day", Rule::gregorian(6, 14), &["US-IN"], US_IN_DAYS)
        .read_from(2026)
        .approximate(),
    state_observance(
        "Casimir Pulaski Day",
        Rule::nth(3, 1, Weekday::Monday),
        &["US-IN"],
        US_IN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "George Rogers Clark Day",
        Rule::gregorian(2, 25),
        &["US-IN"],
        US_IN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Northwest Ordinance Day",
        Rule::gregorian(7, 13),
        &["US-IN"],
        US_IN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    // Iowa.
    state_observance(
        "Mother's Day",
        Rule::nth(5, 2, Weekday::Sunday),
        &["US-IA"],
        US_IA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Father's Day",
        Rule::nth(6, 3, Weekday::Sunday),
        &["US-IA"],
        US_IA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Independence Sunday",
        Rule::WeekdayOnOrBefore {
            month: 7,
            day: 4,
            weekday: Weekday::Sunday,
        },
        &["US-IA"],
        US_IA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Columbus Day (October 12)",
        Rule::gregorian(10, 12),
        &["US-IA"],
        US_IA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Youth Honor Day",
        Rule::gregorian(10, 31),
        &["US-IA"],
        US_IA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Herbert Hoover Day",
        Rule::WeekdayOnOrAfter {
            month: 8,
            day: 7,
            weekday: Weekday::Sunday,
        },
        &["US-IA"],
        US_IA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Arbor Day",
        Rule::last(4, Weekday::Friday),
        &["US-IA"],
        US_IA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Iowa State Flag Day",
        Rule::gregorian(3, 29),
        &["US-IA"],
        US_IA_DAYS,
    )
    .years(Some(1998), None),
    state_observance(
        "Dr. Norman E. Borlaug World Food Prize Day",
        Rule::gregorian(10, 16),
        &["US-IA"],
        US_IA_DAYS,
    )
    .years(Some(2002), None),
    state_observance(
        "Bill of Rights Day",
        Rule::gregorian(12, 15),
        &["US-IA"],
        US_IA_DAYS,
    )
    .years(Some(2002), None),
    state_observance(
        "Juneteenth National Freedom Day",
        Rule::nth(6, 3, Weekday::Saturday),
        &["US-IA"],
        US_IA_DAYS,
    )
    .years(Some(2002), None),
    state_observance(
        "Gift to Iowa's Future Recognition Day",
        Rule::nth(4, 1, Weekday::Monday),
        &["US-IA"],
        US_IA_DAYS,
    )
    .years(Some(2008), None),
    state_observance(
        "Purple Heart Day",
        Rule::gregorian(8, 7),
        &["US-IA"],
        US_IA_DAYS,
    )
    .years(Some(2011), None),
    state_observance(
        "George Washington Carver Day",
        Rule::gregorian(2, 1),
        &["US-IA"],
        US_IA_DAYS,
    )
    .years(Some(2022), None),
    // Kentucky.
    state_observance(
        "National Agriculture (Ag) Day",
        Rule::gregorian(3, 22),
        &["US-KY"],
        US_KY_DAYS,
    )
    .read_from(2026),
    state_observance("Flag Day", Rule::gregorian(6, 14), &["US-KY"], US_KY_DAYS_2)
        .read_from(2026)
        .approximate(),
    state_observance(
        "Mother's Day",
        Rule::nth(5, 2, Weekday::Sunday),
        &["US-KY"],
        US_KY_DAYS_2,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Grandmother's Day",
        Rule::nth(10, 2, Weekday::Sunday),
        &["US-KY"],
        US_KY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Barrier Awareness Day",
        Rule::gregorian(5, 7),
        &["US-KY"],
        US_KY_DAYS_2,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Disability Day",
        Rule::gregorian(8, 2),
        &["US-KY"],
        US_KY_DAYS_2,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "General Pulaski's Day",
        Rule::gregorian(10, 11),
        &["US-KY"],
        US_KY_DAYS_2,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Kentucky Harvest Day",
        Rule::gregorian(11, 15),
        &["US-KY"],
        US_KY_DAYS_2,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "A Day of Prayer for Kentucky's Students",
        Rule::last(9, Weekday::Wednesday),
        &["US-KY"],
        US_KY_DAYS_2,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "9/11 First Responders Day",
        Rule::gregorian(9, 11),
        &["US-KY"],
        US_KY_DAYS_2,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Tuskegee Airmen Commemoration Day",
        Rule::nth(3, 4, Weekday::Thursday),
        &["US-KY"],
        US_KY_DAYS_2,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Kentucky National Guard Day",
        Rule::gregorian(6, 24),
        &["US-KY"],
        US_KY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Kentucky Retired Veterans Day",
        Rule::gregorian(7, 1),
        &["US-KY"],
        US_KY_DAYS_2,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Korean War Armistice Day",
        Rule::gregorian(7, 27),
        &["US-KY"],
        US_KY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Welcome Home Vietnam Veterans Day",
        Rule::gregorian(3, 30),
        &["US-KY"],
        US_KY_DAYS_2,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Diffuse Intrinsic Pontine Glioma Awareness Day",
        Rule::gregorian(5, 17),
        &["US-KY"],
        US_KY_DAYS_2,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "ALS Awareness Day",
        Rule::gregorian(2, 21),
        &["US-KY"],
        US_KY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Mesothelioma Awareness Day",
        Rule::gregorian(9, 26),
        &["US-KY"],
        US_KY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Arthrogryposis Multiplex Congenita Awareness Day",
        Rule::gregorian(6, 30),
        &["US-KY"],
        US_KY_DAYS_2,
    )
    .read_from(2026)
    .approximate(),
    // Louisiana.
    state_observance(
        "Doctors' Day",
        Rule::gregorian(3, 30),
        &["US-LA"],
        US_LA_DAYS,
    )
    .years(Some(1952), None),
    state_observance(
        "Arbor Day",
        Rule::nth(1, 3, Weekday::Friday),
        &["US-LA"],
        US_LA_DAYS,
    )
    .years(Some(1968), None),
    state_observance(
        "My Nationality American Day",
        Rule::gregorian(12, 7),
        &["US-LA"],
        US_LA_DAYS,
    )
    .years(Some(1983), None),
    state_observance(
        "National Airborne Day",
        Rule::gregorian(8, 16),
        &["US-LA"],
        US_LA_DAYS,
    )
    .years(Some(2001), None),
    state_observance(
        "Juneteenth Day",
        Rule::nth(6, 3, Weekday::Saturday),
        &["US-LA"],
        US_LA_DAYS,
    )
    .years(Some(2003), None),
    state_observance(
        "Ronald Reagan Day",
        Rule::gregorian(2, 6),
        &["US-LA"],
        US_LA_DAYS,
    )
    .years(Some(2004), None),
    state_observance(
        "Purple Heart Recognition Day",
        Rule::gregorian(8, 7),
        &["US-LA"],
        US_LA_DAYS,
    )
    .years(Some(2014), None),
    state_observance(
        "Diffuse Intrinsic Pontine Glioma Awareness Day",
        Rule::gregorian(5, 17),
        &["US-LA"],
        US_LA_DAYS,
    )
    .years(Some(2018), None),
    state_observance("SCN2A Day", Rule::gregorian(2, 24), &["US-LA"], US_LA_DAYS)
        .years(Some(2026), None),
    // Maine.
    state_observance(
        "Poetry Day",
        Rule::gregorian(10, 15),
        &["US-ME"],
        US_ME_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Statehood Day",
        Rule::gregorian(3, 15),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(1975), None),
    state_observance(
        "Chester Greenwood Day",
        Rule::gregorian(12, 21),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(1977), None),
    state_observance(
        "R. B. Hall Day",
        Rule::last(6, Weekday::Saturday),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(1981), None),
    state_observance(
        "Saint Jean-Baptiste Day",
        Rule::gregorian(6, 24),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(1983), None),
    state_observance(
        "Sailors' Memorial Day",
        Rule::nth(6, 2, Weekday::Sunday),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(1985), None),
    state_observance(
        "Samantha Smith Day",
        Rule::nth(6, 1, Weekday::Monday),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(1987), None),
    state_observance(
        "Maine Merchant Marine Day",
        Rule::gregorian(5, 22),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(1987), None),
    state_observance(
        "Margaret Chase Smith Day",
        Rule::gregorian(12, 14),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(1987), None),
    state_observance(
        "Edmund S. Muskie Day",
        Rule::gregorian(3, 28),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(1987), None),
    state_observance(
        "Former Prisoner of War Recognition Day",
        Rule::gregorian(4, 9),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(1989), None),
    state_observance(
        "Landowner Recognition Day",
        Rule::nth(9, 3, Weekday::Saturday),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(1995), None),
    state_observance(
        "Children's Day",
        Rule::last(9, Weekday::Friday),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(1997), None),
    state_observance(
        "Firefighter's Recognition Day",
        Rule::nth(10, 1, Weekday::Saturday),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(1999), None),
    state_observance(
        "Prisoner of War - Missing in Action Recognition Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(1999), None),
    state_observance(
        "Organ Donor Awareness Day",
        Rule::gregorian(12, 3),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(1999), None),
    state_observance(
        "Major-General Henry Knox Day",
        Rule::gregorian(7, 25),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(1999), None),
    state_observance(
        "Maine Youth Field and Stream Day",
        Rule::nth(9, 2, Weekday::Saturday),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2001), None),
    state_observance(
        "Colonel Freeman McGilvery Day",
        Rule::nth(9, 1, Weekday::Saturday),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2001), None),
    state_observance(
        "Destroyer Escort Day",
        Rule::nth(6, 3, Weekday::Saturday),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2001), None),
    state_observance(
        "Equal Pay Day",
        Rule::nth(4, 1, Weekday::Tuesday),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2001), None),
    state_observance(
        "Family Reunion Day",
        Rule::WeekdayOnOrAfter {
            month: 8,
            day: 3,
            weekday: Weekday::Monday,
        },
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2005), None),
    state_observance(
        "Lung Cancer Awareness Day",
        Rule::gregorian(11, 1),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2007), None),
    state_observance(
        "Cold War Victory Day",
        Rule::gregorian(5, 1),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2007), None),
    state_observance(
        "Missing Persons Day",
        Rule::gregorian(5, 25),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2007), None),
    state_observance(
        "Native American Veterans Day",
        Rule::gregorian(6, 21),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2009), None),
    state_observance("Wyeth Day", Rule::gregorian(7, 12), &["US-ME"], US_ME_DAYS)
        .years(Some(2009), None),
    state_observance(
        "Governor William King Day",
        Rule::gregorian(3, 16),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2011), None),
    state_observance(
        "Vietnam War Veterans Day",
        Rule::gregorian(3, 29),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2011), None),
    state_observance(
        "Juneteenth Independence Day",
        Rule::nth(6, 3, Weekday::Saturday),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2011), None),
    state_observance(
        "Maine Korean War Veteran Recognition Day",
        Rule::gregorian(7, 27),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2013), None),
    state_observance(
        "Maine Seniors Day",
        Rule::nth(9, 2, Weekday::Saturday),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2013), None),
    state_observance(
        "Native American Heritage and Culture Day",
        Rule::gregorian(3, 20),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Veterans in the Arts and Humanities Day",
        Rule::gregorian(11, 1),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Maine Community Litter Cleanup Day",
        Rule::nth(5, 1, Weekday::Saturday),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "First Responders Day",
        Rule::gregorian(9, 11),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "James Weldon Johnson Day",
        Rule::gregorian(6, 17),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2021), None),
    state_observance(
        "Maine Needham Day",
        Rule::last(9, Weekday::Saturday),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Maine Alewife Day",
        Rule::WeekdayOnOrBefore {
            month: 5,
            day: 29,
            weekday: Weekday::Saturday,
        },
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Maine Irish Heritage Day",
        Rule::gregorian(3, 17),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "A Day to Remember",
        Rule::gregorian(1, 6),
        &["US-ME"],
        US_ME_DAYS,
    )
    .years(Some(2025), None),
    // Maryland.
    state_observance(
        "Maryland Holocaust Remembrance Day",
        Rule::gregorian(1, 27),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    state_observance(
        "6888th Central Postal Directory Battalion Day",
        Rule::gregorian(3, 9),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Tuskegee Airmen Commemoration Day",
        Rule::nth(3, 4, Weekday::Thursday),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Welcome Home Vietnam Veterans Day",
        Rule::gregorian(3, 30),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Crime Victim and Advocate Commemorative Day",
        Rule::gregorian(4, 3),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    state_observance(
        "John Hanson's Birthday",
        Rule::gregorian(4, 13),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    state_observance(
        "National Healthcare Decisions Day",
        Rule::gregorian(4, 16),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Law Day U.S.A.",
        Rule::gregorian(5, 1),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Maryland Centenarians Day",
        Rule::nth(5, 2, Weekday::Thursday),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Negro Baseball League Day",
        Rule::nth(5, 2, Weekday::Saturday),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Fire, Rescue, and Emergency Services Workers Memorial Day",
        Rule::nth(6, 1, Weekday::Sunday),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Maryland Charter Day",
        Rule::gregorian(6, 20),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Thurgood Marshall Day",
        Rule::gregorian(7, 2),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Maryland Veterans Suicide Prevention Day",
        Rule::gregorian(9, 30),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    state_observance(
        "South Asian American Heritage Day",
        Rule::gregorian(10, 2),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Dashain Day",
        Rule::gregorian(10, 5),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Poetry Day",
        Rule::gregorian(10, 15),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Maryland Emancipation Day",
        Rule::gregorian(11, 1),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Annapolis Charter Day",
        Rule::gregorian(12, 17),
        &["US-MD"],
        US_MD_DAYS,
    )
    .read_from(2026),
    // Massachusetts.
    state_observance(
        "Anniversary of the death of General Pulaski",
        Rule::gregorian(10, 11),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Bunker Hill Day (anniversary of the battle of Bunker Hill)",
        Rule::gregorian(6, 17),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Boston Massacre anniversary",
        Rule::gregorian(3, 5),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Commodore John Barry Day",
        Rule::gregorian(9, 13),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "New Orleans Day",
        Rule::gregorian(1, 8),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Anniversary of the death of General Lafayette",
        Rule::gregorian(5, 20),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Evacuation Day",
        Rule::gregorian(3, 17),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Student Government Day",
        Rule::nth(4, 1, Weekday::Friday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "United Nations Day",
        Rule::gregorian(10, 24),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance("Loyalty Day", Rule::gregorian(5, 1), &["US-MA"], US_MA_DAYS).read_from(2026),
    state_observance(
        "Polish Constitution Day",
        Rule::gregorian(5, 3),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Peter Francisco Day",
        Rule::gregorian(3, 15),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Albert Schweitzer's Reverence for Life Day",
        Rule::gregorian(1, 14),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Veterans of World War I Hospital Day",
        Rule::nth(4, 1, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Aunt's and Uncle's Day",
        Rule::nth(4, 2, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Horace Mann Day",
        Rule::gregorian(5, 4),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Mother's Day",
        Rule::nth(5, 2, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Father's Day",
        Rule::nth(6, 3, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Purple Heart Day",
        Rule::gregorian(8, 7),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Senior Citizens' Day",
        Rule::nth(10, 1, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Disabled American Veteran's Hospital Day",
        Rule::nth(12, 1, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Army and Navy Union Day",
        Rule::nth(12, 2, Weekday::Saturday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Children's Day",
        Rule::nth(6, 2, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Teachers' Day",
        Rule::nth(6, 1, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Maritime Day",
        Rule::gregorian(5, 22),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Jamaican Independence Day",
        Rule::nth(8, 1, Weekday::Monday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Iwo Jima Day",
        Rule::gregorian(2, 19),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Tadeusz Kosciuszko Day",
        Rule::nth(2, 1, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Pearl Harbor Day",
        Rule::gregorian(12, 7),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Grandparents' Day",
        Rule::WeekdayOnOrAfter {
            month: 9,
            day: 7,
            weekday: Weekday::Sunday,
        },
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Anniversary of the enlistment of Deborah Samson",
        Rule::gregorian(5, 23),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Lithuanian Independence Day",
        Rule::gregorian(2, 16),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Statue of Liberty Awareness Day",
        Rule::gregorian(10, 26),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Slovak Independence Day",
        Rule::gregorian(3, 14),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Social Security Day",
        Rule::gregorian(8, 14),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Korean War Veterans Day",
        Rule::gregorian(6, 25),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Human Rights Day",
        Rule::gregorian(12, 10),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Exercise Tiger Day",
        Rule::gregorian(4, 28),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Former Prisoner of War Recognition Day",
        Rule::gregorian(4, 9),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Homeless Unity Day",
        Rule::gregorian(2, 20),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "USO Appreciation Day",
        Rule::gregorian(2, 4),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Rose Fitzgerald Kennedy Day",
        Rule::gregorian(7, 22),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Destroyer Escort Day",
        Rule::nth(6, 3, Weekday::Saturday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "School Principals' Recognition Day",
        Rule::gregorian(4, 27),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Native American Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Candle Safety Day",
        Rule::nth(12, 2, Weekday::Monday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Massachusetts Biomedical Research Day",
        Rule::gregorian(10, 21),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Spanish War Memorial Day and Maine Memorial Day",
        Rule::gregorian(2, 15),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "State Constitution Day",
        Rule::gregorian(10, 25),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Constitution Day",
        Rule::gregorian(9, 17),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Susan B. Anthony Day",
        Rule::gregorian(8, 26),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Youth Honor Day",
        Rule::gregorian(10, 31),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Liberty Tree Day",
        Rule::gregorian(8, 14),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Italian American War Veterans of the United States, Inc., Day",
        Rule::gregorian(3, 27),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "John F. Kennedy Day",
        Rule::last(11, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Battleship Massachusetts Memorial Day",
        Rule::last(6, Weekday::Saturday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "United States Marine Corps Day",
        Rule::gregorian(11, 10),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Martin Luther King, Jr. Day (January 15 proclamation)",
        Rule::gregorian(1, 15),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Kalevala Day",
        Rule::gregorian(2, 28),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Social Justice for Ireland Day",
        Rule::nth(10, 1, Weekday::Saturday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "White Cane Safety Day",
        Rule::gregorian(10, 15),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "National Hunting and Fishing Day",
        Rule::nth(9, 4, Weekday::Saturday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Bataan-Corregidor Day",
        Rule::gregorian(4, 9),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Retired Members of the Armed Forces Day",
        Rule::nth(6, 1, Weekday::Monday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Endangered Species Day",
        Rule::nth(9, 2, Weekday::Saturday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "John Carver Day",
        Rule::nth(6, 4, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Armenian Martyrs' Day",
        Rule::gregorian(4, 24),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Fire Fighters Memorial Sunday",
        Rule::nth(6, 2, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Vietnam Veterans Day",
        Rule::gregorian(3, 29),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "State Walking Sunday",
        Rule::nth(6, 2, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Saint Jean de Baptiste Day",
        Rule::nth(6, 4, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Town Meeting Day",
        Rule::gregorian(10, 8),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Greek Independence Day",
        Rule::gregorian(3, 25),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Public Employees Appreciation Day",
        Rule::nth(6, 1, Weekday::Wednesday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Presidents Day (29 May)",
        Rule::gregorian(5, 29),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Youth in Government Day",
        Rule::nth(8, 1, Weekday::Friday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Joshua James Day",
        Rule::nth(5, 3, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Leif Ericson Day",
        Rule::gregorian(10, 9),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Massachusetts Whale Awareness Day",
        Rule::nth(5, 1, Weekday::Thursday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "POW/MIA Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Independent Living Center Day",
        Rule::nth(10, 1, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Massachusetts Police Memorial Day",
        Rule::gregorian(5, 15),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Workers' Memorial Day",
        Rule::nth(4, 4, Weekday::Friday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Samuel Slater Day",
        Rule::gregorian(12, 20),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Massachusetts Emergency Responders Memorial Day",
        Rule::nth(5, 2, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Robert Goddard Day",
        Rule::gregorian(3, 16),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Special Needs Awareness Day",
        Rule::gregorian(5, 23),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Robert Frost Day",
        Rule::nth(10, 4, Weekday::Saturday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Lucy Stone Day",
        Rule::gregorian(3, 8),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Congenital Heart Defect Awareness Day",
        Rule::gregorian(2, 14),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Arthritis Awareness Day",
        Rule::nth(10, 3, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Luther Burbank Day",
        Rule::gregorian(3, 7),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Civilian Conservation Corps Day",
        Rule::gregorian(3, 31),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Thomas Paine Day",
        Rule::gregorian(1, 29),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance("Tartan Day", Rule::gregorian(4, 6), &["US-MA"], US_MA_DAYS).read_from(2026),
    state_observance(
        "Missing Children's Day",
        Rule::gregorian(5, 25),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance("Unity Day", Rule::gregorian(9, 11), &["US-MA"], US_MA_DAYS).read_from(2026),
    state_observance(
        "Myositis Awareness Day",
        Rule::gregorian(9, 21),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "9th Regiment Massachusetts Volunteer Infantry of the Civil War Day",
        Rule::gregorian(6, 27),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "54th Regiment Massachusetts Volunteer Infantry of the Civil War Day",
        Rule::gregorian(7, 18),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Guardians' Day",
        Rule::nth(4, 4, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "World War II Commemoration Day",
        Rule::gregorian(9, 2),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Bill of Rights Day",
        Rule::gregorian(12, 15),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Leopoldville Disaster Remembrance Day",
        Rule::gregorian(12, 24),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Phenylketonuria Awareness Day",
        Rule::gregorian(5, 24),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Philanthropy Day",
        Rule::gregorian(11, 15),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Jack Kerouac Day",
        Rule::gregorian(3, 12),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Massachusetts Nonprofit Awareness Day",
        Rule::nth(6, 2, Weekday::Monday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Polish American Congress Day",
        Rule::gregorian(10, 30),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "No Name Calling Day",
        Rule::nth(1, 4, Weekday::Wednesday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Eunice Kennedy Shriver Day",
        Rule::nth(9, 4, Weekday::Saturday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Fragile X Awareness Day",
        Rule::gregorian(7, 22),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Massachusetts Service and Volunteerism Day",
        Rule::nth(4, 2, Weekday::Thursday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .years(Some(2014), None),
    state_observance(
        "United States Army Birthday Day",
        Rule::gregorian(6, 14),
        &["US-MA"],
        US_MA_DAYS,
    )
    .years(Some(2014), None),
    state_observance(
        "General Sylvanus Thayer Day",
        Rule::gregorian(6, 9),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "PANDAS/PANS Awareness Day",
        Rule::gregorian(10, 9),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ataxia Awareness Day",
        Rule::gregorian(9, 25),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Southbridge Lions Club Bow Ties for Esophageal Cancer Awareness Day",
        Rule::last(5, Weekday::Tuesday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Massachusetts Race Amity Day",
        Rule::nth(6, 2, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "First Responder Day",
        Rule::WeekdayOnOrAfter {
            month: 4,
            day: 14,
            weekday: Weekday::Sunday,
        },
        &["US-MA"],
        US_MA_DAYS,
    )
    .years(Some(2016), None),
    state_observance(
        "Facioscapulohumeral Muscular Dystrophy Day",
        Rule::gregorian(6, 20),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance("Peace Day", Rule::gregorian(9, 21), &["US-MA"], US_MA_DAYS)
        .years(Some(2016), None),
    state_observance(
        "Massachusetts Women's Defense Corps Remembrance Day",
        Rule::gregorian(4, 2),
        &["US-MA"],
        US_MA_DAYS,
    )
    .years(Some(2018), None),
    state_observance(
        "Gold Star Wives Day",
        Rule::gregorian(4, 5),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Gold Star Mothers and Families Day",
        Rule::last(9, Weekday::Sunday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Frederick Douglass Day",
        Rule::gregorian(2, 14),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "United States Navy Day",
        Rule::gregorian(10, 13),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "United States Cadet Nurse Corps Day",
        Rule::gregorian(7, 1),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Overdose Awareness Day",
        Rule::gregorian(8, 31),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Negro Election Day",
        Rule::nth(7, 3, Weekday::Saturday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Founding of the United States Army",
        Rule::gregorian(6, 14),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Founding of the United States Air Force",
        Rule::gregorian(9, 18),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Founding of the National Guard",
        Rule::gregorian(12, 13),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Founding of the United States Coast Guard",
        Rule::gregorian(8, 4),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Massachusetts Emancipation Day (Quock Walker Day)",
        Rule::gregorian(7, 8),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Moyamoya Awareness Day",
        Rule::gregorian(5, 6),
        &["US-MA"],
        US_MA_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Inflammatory Breast Cancer Awareness Day",
        Rule::nth(10, 2, Weekday::Tuesday),
        &["US-MA"],
        US_MA_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "School Custodian Day",
        Rule::gregorian(10, 2),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "United States Navy Seabees Day",
        Rule::gregorian(3, 5),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Right Whale Day",
        Rule::gregorian(4, 24),
        &["US-MA"],
        US_MA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "United States Merchant Marine Day",
        Rule::gregorian(5, 22),
        &["US-MA"],
        US_MA_DAYS,
    )
    .years(Some(2024), None),
    state_observance(
        "Rosa Parks Day",
        Rule::gregorian(2, 4),
        &["US-MA"],
        US_MA_DAYS,
    )
    .years(Some(2024), None),
    state_observance(
        "Lincoln Day",
        Rule::gregorian(2, 12),
        &["US-MA"],
        US_MA_DAYS_2,
    )
    .read_from(2026)
    .approximate(),
    state_observance("Flag Day", Rule::gregorian(6, 14), &["US-MA"], US_MA_DAYS_3).read_from(2026),
    state_observance(
        "Arbor and Bird Day",
        Rule::last(4, Weekday::Friday),
        &["US-MA"],
        US_MA_DAYS_2,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "United States Space Force Day",
        Rule::gregorian(12, 20),
        &["US-MA"],
        US_MA_DAYS_2,
    )
    .read_from(2026)
    .approximate(),
    // Michigan.
    state_observance(
        "Mrs. Rosa L. Parks Day",
        Rule::WeekdayOnOrAfter {
            month: 2,
            day: 5,
            weekday: Weekday::Monday,
        },
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Grandparents' and Grandchildren's Day",
        Rule::gregorian(3, 18),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Casimir Pulaski Day",
        Rule::gregorian(10, 11),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Michigan Indian Day",
        Rule::nth(9, 4, Weekday::Friday),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "John Fitzgerald Kennedy Day",
        Rule::gregorian(5, 29),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "American Family Day",
        Rule::nth(8, 1, Weekday::Sunday),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Log Cabin Day",
        Rule::last(6, Weekday::Sunday),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Pearl Harbor Day",
        Rule::gregorian(12, 7),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Michigan Day of Remembrance of the Armenian Genocide",
        Rule::gregorian(4, 24),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Children's Memorial Day",
        Rule::nth(4, 4, Weekday::Friday),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Cesar E. Chavez Day",
        Rule::gregorian(3, 31),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "President Gerald R. Ford Day",
        Rule::gregorian(7, 14),
        &["US-MI"],
        US_MI_DAYS_2,
    )
    .read_from(2026),
    state_observance(
        "Henry Ford Day",
        Rule::gregorian(7, 30),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Fred Korematsu Day",
        Rule::gregorian(1, 30),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Willie Horton Day",
        Rule::gregorian(10, 18),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "United States Army commemoration",
        Rule::gregorian(6, 14),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "United States Coast Guard commemoration",
        Rule::gregorian(8, 4),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "United States Air Force commemoration",
        Rule::gregorian(9, 18),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "United States Navy commemoration",
        Rule::gregorian(10, 13),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "United States Marine Corps commemoration",
        Rule::gregorian(11, 10),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Police Officers Memorial Day",
        Rule::gregorian(5, 15),
        &["US-MI"],
        US_MI_DAYS_2,
    )
    .read_from(2026),
    state_observance(
        "Firefighters Memorial Day",
        Rule::gregorian(5, 4),
        &["US-MI"],
        US_MI_DAYS_2,
    )
    .read_from(2026),
    state_observance(
        "Sojourner Truth Day",
        Rule::gregorian(11, 26),
        &["US-MI"],
        US_MI_DAYS_2,
    )
    .read_from(2026),
    state_observance(
        "Women Veterans Recognition Day",
        Rule::gregorian(6, 12),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Negro Leagues Day",
        Rule::gregorian(5, 2),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Blue Star Mothers Day",
        Rule::gregorian(2, 1),
        &["US-MI"],
        US_MI_DAYS,
    )
    .read_from(2026)
    .approximate(),
    // Minnesota.
    state_observance(
        "Ethnic American Day",
        Rule::nth(6, 1, Weekday::Sunday),
        &["US-MN"],
        US_MN_DAYS,
    )
    .years(Some(1990), None),
    state_observance("India Day", Rule::gregorian(8, 15), &["US-MN"], US_MN_DAYS)
        .years(Some(2021), None),
    state_observance(
        "Four Chaplains Day",
        Rule::gregorian(2, 3),
        &["US-MN"],
        US_MN_DAYS,
    )
    .years(Some(1998), None),
    state_observance(
        "Military Spouses and Families Day",
        Rule::WeekdayOnOrBefore {
            month: 5,
            day: 30,
            weekday: Weekday::Sunday,
        },
        &["US-MN"],
        US_MN_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Medal of Honor Day",
        Rule::gregorian(3, 25),
        &["US-MN"],
        US_MN_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "POW and MIA Recognition Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-MN"],
        US_MN_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Veterans Suicide Prevention and Awareness Day",
        Rule::nth(10, 1, Weekday::Saturday),
        &["US-MN"],
        US_MN_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Combat Wounded Veterans Purple Heart Day",
        Rule::gregorian(8, 7),
        &["US-MN"],
        US_MN_DAYS,
    )
    .years(Some(2001), None),
    state_observance(
        "Hmong Special Guerrilla Units Remembrance Day",
        Rule::gregorian(5, 14),
        &["US-MN"],
        US_MN_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Atomic Veterans Day",
        Rule::gregorian(7, 16),
        &["US-MN"],
        US_MN_DAYS,
    )
    .years(Some(2018), None),
    state_observance(
        "Fallen Firefighters Memorial Day",
        Rule::last(9, Weekday::Sunday),
        &["US-MN"],
        US_MN_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Dr. Norman E. Borlaug World Food Prize Day",
        Rule::gregorian(10, 16),
        &["US-MN"],
        US_MN_DAYS,
    )
    .years(Some(2004), None),
    state_observance(
        "General John Vessey Day",
        Rule::gregorian(6, 29),
        &["US-MN"],
        US_MN_DAYS,
    )
    .years(Some(2018), None),
    state_observance(
        "American Allies Day",
        Rule::gregorian(6, 30),
        &["US-MN"],
        US_MN_DAYS,
    )
    .years(Some(2019), None),
    // Mississippi.
    state_observance(
        "Elvis Aaron Presley Day",
        Rule::gregorian(8, 16),
        &["US-MS"],
        US_MS_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Hernando de Soto Day",
        Rule::gregorian(5, 8),
        &["US-MS"],
        US_MS_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Tuskegee Airmen Day",
        Rule::nth(3, 4, Weekday::Thursday),
        &["US-MS"],
        US_MS_DAYS,
    )
    .read_from(2026)
    .approximate(),
    // Missouri.
    state_observance(
        "John Donaldson Day",
        Rule::gregorian(2, 20),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2025), None),
    state_observance(
        "Jefferson Day",
        Rule::gregorian(4, 13),
        &["US-MO"],
        US_MO_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Missouri Day",
        Rule::nth(10, 3, Weekday::Wednesday),
        &["US-MO"],
        US_MO_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Jackie Robinson Day",
        Rule::gregorian(4, 15),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Lucile Bluford Day",
        Rule::gregorian(7, 1),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2016), None),
    state_observance(
        "Law Day U.S.A.",
        Rule::gregorian(5, 1),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(1961), None),
    state_observance(
        "Law Enforcement Appreciation Day",
        Rule::nth(5, 1, Weekday::Friday),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2021), None),
    state_observance("Flag Day", Rule::gregorian(6, 14), &["US-MO"], US_MO_DAYS)
        .years(Some(1986), None),
    state_observance(
        "Prisoners of War Remembrance Day",
        Rule::gregorian(4, 9),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(1987), None),
    state_observance(
        "POW/MIA Recognition Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(1999), None),
    state_observance(
        "Silver Star Families of America Day",
        Rule::gregorian(5, 1),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Korean War Veterans Day",
        Rule::gregorian(7, 27),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(1989), None),
    state_observance(
        "Vietnam Veterans Day",
        Rule::gregorian(3, 30),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2012), None),
    state_observance(
        "Veterans of Operation Iraq/Enduring Freedom Day",
        Rule::gregorian(3, 26),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2012), None),
    state_observance(
        "Stars and Stripes Day",
        Rule::gregorian(11, 9),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Arbor Day",
        Rule::nth(4, 1, Weekday::Friday),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(1990), None),
    state_observance(
        "Bird Appreciation Day",
        Rule::gregorian(3, 21),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2001), None),
    state_observance(
        "Pearl Harbor Remembrance Day",
        Rule::gregorian(12, 7),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(1996), None),
    state_observance(
        "Patriots Day",
        Rule::gregorian(4, 19),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2002), None),
    state_observance(
        "Battle of St. Louis Memorial Day",
        Rule::gregorian(5, 26),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Missouri's Peace Officers Memorial Day",
        Rule::gregorian(5, 15),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(1996), None),
    state_observance(
        "Emergency Services Day",
        Rule::gregorian(9, 11),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(1999), None),
    state_observance(
        "Emergency Personnel Appreciation Day",
        Rule::gregorian(9, 11),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2002), None),
    state_observance(
        "Missouri School Read-In Day",
        Rule::nth(3, 2, Weekday::Friday),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2011), None),
    state_observance(
        "Bill of Rights Day",
        Rule::gregorian(12, 15),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2002), None),
    state_observance(
        "Constitution Day",
        Rule::gregorian(1, 31),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2022), None),
    state_observance("PKS Day", Rule::gregorian(12, 4), &["US-MO"], US_MO_DAYS)
        .years(Some(2013), None),
    state_observance(
        "ROHHAD Awareness Day",
        Rule::gregorian(5, 7),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Dress in Blue for Colon Cancer Awareness Day",
        Rule::nth(3, 1, Weekday::Friday),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2011), None),
    state_observance(
        "Organ Donor Recognition Day",
        Rule::gregorian(7, 3),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Alzheimer's Awareness Day",
        Rule::nth(3, 2, Weekday::Tuesday),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2001), None),
    state_observance(
        "Alpha Phi Alpha Day",
        Rule::gregorian(12, 4),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Rosa Parks Day",
        Rule::gregorian(2, 4),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2006), None),
    state_observance(
        "Walk & Bike to School Day",
        Rule::nth(10, 1, Weekday::Wednesday),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2011), None),
    state_observance(
        "Bike to Work Day",
        Rule::nth(5, 3, Weekday::Friday),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2011), None),
    state_observance(
        "Girl Scout Day",
        Rule::gregorian(3, 12),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2010), None),
    state_observance(
        "Random Acts of Kindness Day",
        Rule::gregorian(8, 31),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2021), None),
    state_observance(
        "Links, Incorporated Day",
        Rule::gregorian(11, 9),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2025), None),
    state_observance(
        "Dangers of Inflation Awareness Day",
        Rule::gregorian(4, 15),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Epilepsy Awareness Day",
        Rule::gregorian(3, 26),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Medical Radiation Safety Awareness Day",
        Rule::gregorian(3, 27),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2014), None),
    state_observance(
        "Missouri Lineworker Appreciation Day",
        Rule::nth(4, 2, Weekday::Monday),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Alpha Kappa Alpha Sorority Day",
        Rule::gregorian(1, 15),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Ethel Hedgeman Lyle Day",
        Rule::gregorian(2, 10),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Pinhook Remembrance Day",
        Rule::gregorian(5, 2),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Walt Disney - 'A Day to Dream' Day",
        Rule::gregorian(10, 16),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "George Jones Day",
        Rule::gregorian(9, 12),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2026), None),
    state_observance(
        "Mark Twain Day",
        Rule::gregorian(11, 30),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2021), None),
    state_observance(
        "Iron Curtain Speech Day",
        Rule::gregorian(3, 5),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2021), None),
    state_observance(
        "Celiac Awareness Day",
        Rule::nth(5, 2, Weekday::Wednesday),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Missouri Sliced Bread Day",
        Rule::gregorian(7, 7),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Posttraumatic Stress Awareness Day",
        Rule::gregorian(6, 27),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2018), None),
    state_observance(
        "Mormon War Remembrance Day",
        Rule::gregorian(7, 2),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Diffuse Intrinsic Pontine Glioma Awareness Day",
        Rule::gregorian(9, 9),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Hypoplastic Left Heart Syndrome Awareness Day",
        Rule::gregorian(4, 18),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "John Jordan 'Buck' O'Neil Day",
        Rule::gregorian(11, 13),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2021), None),
    state_observance(
        "Eddie Gaedel Day",
        Rule::gregorian(8, 19),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2025), None),
    state_observance(
        "Honor Guard Appreciation Day",
        Rule::gregorian(8, 19),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2020), None),
    state_observance(
        "Ghost Army Recognition Day",
        Rule::gregorian(6, 6),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2020), None),
    state_observance(
        "Walthall Moore Day",
        Rule::gregorian(5, 1),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2021), None),
    state_observance(
        "Farmers and Ranchers Day",
        Rule::gregorian(7, 20),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Pioneering Black Women's Day",
        Rule::gregorian(3, 26),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2021), None),
    state_observance(
        "Hazel Erby Day",
        Rule::gregorian(9, 22),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2021), None),
    state_observance(
        "Betty L. Thompson Day",
        Rule::gregorian(12, 3),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "School Bus Drivers' Appreciation Day",
        Rule::gregorian(5, 10),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "National Good Neighbor Day",
        Rule::gregorian(9, 28),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Caregiver Appreciation Day",
        Rule::gregorian(9, 15),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Biliary Atresia Awareness Day",
        Rule::gregorian(10, 1),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Missouri Donate Life Day",
        Rule::gregorian(4, 16),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Lupus Awareness Day",
        Rule::gregorian(5, 10),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Missouri Black Bear Awareness Day",
        Rule::gregorian(4, 22),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2026), None),
    state_observance(
        "Sexual Assault Prevention and Awareness Day",
        Rule::gregorian(6, 1),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Chris Sifford Day",
        Rule::gregorian(8, 6),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2025), None),
    state_observance(
        "Women Veterans Appreciation Day",
        Rule::gregorian(6, 12),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Breast Cancer Awareness Day",
        Rule::nth(10, 1, Weekday::Saturday),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Domestic Violence Awareness Day",
        Rule::nth(10, 3, Weekday::Saturday),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Albert Pujols Day",
        Rule::gregorian(1, 16),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Shelley v. Kraemer Day",
        Rule::gregorian(5, 3),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "K.C. Wolf Day",
        Rule::gregorian(11, 23),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Lloyd Gaines Day",
        Rule::gregorian(3, 19),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Premenstrual Dysphoric Disorder (PMDD) Awareness Day",
        Rule::gregorian(10, 2),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Michael Collins Day",
        Rule::gregorian(10, 16),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2025), None),
    state_observance(
        "Emmett Kelly Day",
        Rule::last(4, Weekday::Saturday),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2025), None),
    state_observance(
        "Baker Service Appreciation Day",
        Rule::gregorian(4, 16),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Celia Day",
        Rule::nth(4, 2, Weekday::Tuesday),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2025), None),
    state_observance(
        "Freeman Bosley, Sr. Day",
        Rule::gregorian(12, 1),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2026), None),
    state_observance(
        "Believe in Gianna Day",
        Rule::gregorian(11, 13),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2025), None),
    state_observance(
        "Amyloidosis Awareness Day",
        Rule::gregorian(5, 8),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2025), None),
    state_observance(
        "Ulysses S. Grant Day",
        Rule::gregorian(4, 27),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2026), None),
    state_observance(
        "Kappa Alpha Psi Day",
        Rule::gregorian(1, 5),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2025), None),
    state_observance(
        "Frankie Muse Freeman Day",
        Rule::gregorian(11, 24),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2026), None),
    state_observance(
        "End Neighborhood Gun Violence Day",
        Rule::gregorian(6, 17),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2025), None),
    state_observance(
        "PANS/PANDAS Awareness Day",
        Rule::gregorian(3, 26),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2026), None),
    state_observance(
        "Racquetball Day",
        Rule::gregorian(4, 5),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2026), None),
    state_observance(
        "William Lacy Clay Sr. Day",
        Rule::gregorian(4, 30),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2026), None),
    state_observance(
        "Missouri River Runner Day",
        Rule::gregorian(5, 10),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2026), None),
    state_observance(
        "Charlie Parker Day",
        Rule::gregorian(3, 12),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2026), None),
    state_observance(
        "Leon Jordan Day",
        Rule::gregorian(7, 15),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2026), None),
    state_observance(
        "Election Worker Appreciation Day",
        Rule::gregorian(8, 12),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2026), None),
    state_observance(
        "Eliot and Muriel Battle Day",
        Rule::gregorian(6, 28),
        &["US-MO"],
        US_MO_DAYS,
    )
    .years(Some(2026), None),
    // Montana.
    state_observance(
        "Arbor Day",
        Rule::last(4, Weekday::Friday),
        &["US-MT"],
        US_MT_DAYS,
    )
    .years(Some(1989), None),
    state_observance(
        "Bill of Rights Day",
        Rule::gregorian(12, 15),
        &["US-MT"],
        US_MT_DAYS,
    )
    .years(Some(2007), None),
    state_observance(
        "American Indian Heritage Day",
        Rule::last(9, Weekday::Friday),
        &["US-MT"],
        US_MT_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Teen Driver Safety Day",
        Rule::nth(10, 3, Weekday::Tuesday),
        &["US-MT"],
        US_MT_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Welcome Home Vietnam Veterans Day",
        Rule::gregorian(3, 30),
        &["US-MT"],
        US_MT_DAYS,
    )
    .years(Some(2011), None),
    state_observance(
        "Juneteenth National Freedom Day",
        Rule::nth(6, 3, Weekday::Saturday),
        &["US-MT"],
        US_MT_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "Montana Mining Day",
        Rule::gregorian(2, 9),
        &["US-MT"],
        US_MT_DAYS,
    )
    .years(Some(2025), None),
    // Nebraska.
    state_observance(
        "George W. Norris Day",
        Rule::gregorian(1, 5),
        &["US-NE"],
        US_NE_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Martin Luther King, Jr. Day (January 15 school observance)",
        Rule::gregorian(1, 15),
        &["US-NE"],
        US_NE_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "American Indian Day",
        Rule::nth(9, 4, Weekday::Monday),
        &["US-NE"],
        US_NE_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Workers Memorial Day",
        Rule::gregorian(4, 28),
        &["US-NE"],
        US_NE_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance("State Day", Rule::gregorian(3, 1), &["US-NE"], US_NE_DAYS)
        .read_from(2026)
        .approximate(),
    state_observance(
        "Pulaski's Memorial Day",
        Rule::gregorian(10, 11),
        &["US-NE"],
        US_NE_DAYS,
    )
    .read_from(2026)
    .approximate(),
    // Nevada.
    state_observance(
        "Arbor Day",
        Rule::last(4, Weekday::Friday),
        &["US-NV"],
        US_NV_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Mother's Day",
        Rule::nth(5, 2, Weekday::Sunday),
        &["US-NV"],
        US_NV_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Cesar Chavez Day",
        Rule::gregorian(3, 31),
        &["US-NV"],
        US_NV_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Law Day U.S.A.",
        Rule::gregorian(5, 1),
        &["US-NV"],
        US_NV_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Asian Culture Day",
        Rule::gregorian(5, 18),
        &["US-NV"],
        US_NV_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Constitution Day",
        Rule::gregorian(9, 17),
        &["US-NV"],
        US_NV_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Sarah Winnemucca Day",
        Rule::gregorian(10, 16),
        &["US-NV"],
        US_NV_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Indigenous Peoples Day",
        Rule::gregorian(8, 9),
        &["US-NV"],
        US_NV_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Nevada Tribes Legislative Day",
        Rule::nth(2, 2, Weekday::Tuesday),
        &["US-NV"],
        US_NV_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Native American Day",
        Rule::nth(9, 4, Weekday::Friday),
        &["US-NV"],
        US_NV_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Healthcare Decisions Day",
        Rule::gregorian(4, 16),
        &["US-NV"],
        US_NV_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Buffalo Soldiers Day",
        Rule::gregorian(7, 28),
        &["US-NV"],
        US_NV_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Pearl Harbor Remembrance Day",
        Rule::gregorian(12, 7),
        &["US-NV"],
        US_NV_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Veterans Day at the Legislature",
        Rule::nth(3, 3, Weekday::Wednesday),
        &["US-NV"],
        US_NV_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Public Lands Day",
        Rule::last(9, Weekday::Saturday),
        &["US-NV"],
        US_NV_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance("Tartan Day", Rule::gregorian(4, 6), &["US-NV"], US_NV_DAYS)
        .read_from(2026)
        .approximate(),
    state_observance(
        "World Esports Day",
        Rule::WeekdayOnOrBefore {
            month: 10,
            day: 24,
            weekday: Weekday::Saturday,
        },
        &["US-NV"],
        US_NV_DAYS,
    )
    .read_from(2026)
    .approximate(),
    // New Jersey.
    state_observance(
        "Crispus Attucks Day",
        Rule::gregorian(3, 5),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "New Jersey Day",
        Rule::gregorian(4, 17),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance("Law Day", Rule::gregorian(5, 1), &["US-NJ"], US_NJ_DAYS)
        .read_from(2026)
        .approximate(),
    state_observance(
        "New Jersey P.O.W.-M.I.A. Recognition Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "New Jersey Retired Teachers Day",
        Rule::nth(11, 1, Weekday::Sunday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Police, Firemen and First Aid Recognition Day",
        Rule::nth(5, 3, Weekday::Sunday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Grandparents' Day",
        Rule::WeekdayOnOrAfter {
            month: 9,
            day: 7,
            weekday: Weekday::Sunday,
        },
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "New Jersey Shore Celebration Day",
        Rule::WeekdayOnOrAfter {
            month: 5,
            day: 16,
            weekday: Weekday::Saturday,
        },
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Vietnam Veterans' Remembrance Day",
        Rule::gregorian(5, 7),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Korean War Veterans' Day",
        Rule::gregorian(7, 27),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Alzheimer's Disease Awareness Day",
        Rule::nth(11, 1, Weekday::Monday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Women's Equality Day",
        Rule::gregorian(8, 26),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Senior Citizen's Day",
        Rule::gregorian(5, 15),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "A Day of Prayer in New Jersey",
        Rule::nth(5, 1, Weekday::Thursday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Take our Daughters to Work Day",
        Rule::last(4, Weekday::Thursday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Delaware Bay Day",
        Rule::nth(6, 2, Weekday::Saturday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Children's Memorial Day",
        Rule::gregorian(5, 25),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Native American Day",
        Rule::nth(9, 4, Weekday::Friday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Thomas Mundy Peterson Day",
        Rule::gregorian(3, 31),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Toms River East Little League World Champions Day",
        Rule::gregorian(8, 29),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Bill of Rights Day in New Jersey",
        Rule::gregorian(11, 20),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Peace Officers Memorial Day",
        Rule::gregorian(5, 15),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Loyal Heart Award Day",
        Rule::nth(5, 1, Weekday::Sunday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Pearl Harbor Remembrance Day",
        Rule::gregorian(12, 7),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "National Airborne Day",
        Rule::gregorian(8, 16),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Parents of Fallen Military Sons and Daughters Day",
        Rule::last(9, Weekday::Sunday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Foster Children's Day",
        Rule::gregorian(12, 12),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "New Jersey Credit Union Day",
        Rule::nth(10, 3, Weekday::Thursday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Patriots Day",
        Rule::nth(4, 3, Weekday::Monday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "UNICEF Day",
        Rule::gregorian(10, 31),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Liberty Day",
        Rule::gregorian(3, 16),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Juneteenth Independence Day",
        Rule::nth(6, 3, Weekday::Saturday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Rett Syndrome Awareness Day",
        Rule::nth(10, 3, Weekday::Tuesday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Lymphedema Awareness Day",
        Rule::gregorian(3, 6),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Freedom Walk Day",
        Rule::WeekdayOnOrBefore {
            month: 9,
            day: 10,
            weekday: Weekday::Sunday,
        },
        &["US-NJ"],
        US_NJ_DAYS_2,
    )
    .read_from(2026),
    state_observance(
        "Firefighter Recognition Day",
        Rule::gregorian(10, 2),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Aviation Maintenance Technician Day",
        Rule::gregorian(5, 24),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Postpartum Depression Awareness Day",
        Rule::gregorian(10, 20),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Gold Star Mother's Day",
        Rule::last(9, Weekday::Sunday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance("Denim Day", Rule::gregorian(4, 28), &["US-NJ"], US_NJ_DAYS)
        .read_from(2026)
        .approximate(),
    state_observance(
        "Mesothelioma Awareness Day",
        Rule::gregorian(9, 26),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "K9 Veterans Day",
        Rule::gregorian(3, 13),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Corrections Officer Day",
        Rule::gregorian(7, 30),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Neuroendocrine Tumor Cancer Awareness Day",
        Rule::gregorian(11, 10),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Human Trafficking Awareness Day",
        Rule::gregorian(1, 11),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Neonatal Alloimmune Thrombocytopenia Awareness Day",
        Rule::gregorian(6, 7),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Clarence Clemons Day",
        Rule::gregorian(1, 11),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Patient Advocate Day",
        Rule::gregorian(11, 15),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Attention Deficit/Hyperactivity Disorder Awareness Day",
        Rule::gregorian(1, 21),
        &["US-NJ"],
        US_NJ_DAYS_2,
    )
    .read_from(2026),
    state_observance(
        "Doctors' Day",
        Rule::gregorian(3, 30),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Dominican Pride Day",
        Rule::nth(7, 2, Weekday::Saturday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Children's Grief Awareness Day",
        Rule::nth(11, 3, Weekday::Thursday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "ALS Awareness Day",
        Rule::nth(5, 3, Weekday::Wednesday),
        &["US-NJ"],
        US_NJ_DAYS_2,
    )
    .read_from(2026),
    state_observance(
        "Dominican Restoration Day",
        Rule::gregorian(8, 16),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Hannah G. Solomon Day",
        Rule::gregorian(1, 14),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Night of Conversation",
        Rule::gregorian(11, 19),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Senator Joseph Palaia Day",
        Rule::gregorian(2, 3),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Maternal Health Awareness Day",
        Rule::gregorian(1, 23),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Post-Traumatic Stress Disorder Awareness Day",
        Rule::gregorian(6, 27),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "ASK (Asking Saves Kids) Day",
        Rule::gregorian(6, 21),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "World Autism Awareness Day",
        Rule::gregorian(4, 2),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Concussion Awareness Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Evans Syndrome Awareness Day",
        Rule::gregorian(9, 21),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Transgender Day of Remembrance",
        Rule::gregorian(11, 20),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Fibrodysplasia Ossificans Progressiva Awareness Day",
        Rule::gregorian(4, 23),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Socks for the Homeless Day",
        Rule::gregorian(2, 14),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Tourette Syndrome Awareness Day",
        Rule::gregorian(6, 4),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "World Refugee Day",
        Rule::gregorian(6, 20),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Hispanic Journalist Pride Day",
        Rule::gregorian(9, 20),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Sergeant Dominick Pilla and Corporal Jamie Smith Day",
        Rule::gregorian(10, 3),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "A Day in the Life -- Type 1 Diabetes Day",
        Rule::WeekdayOnOrAfter {
            month: 11,
            day: 19,
            weekday: Weekday::Monday,
        },
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance("Sikh Day", Rule::gregorian(4, 14), &["US-NJ"], US_NJ_DAYS)
        .read_from(2026)
        .approximate(),
    state_observance(
        "Peter Francisco Day",
        Rule::gregorian(3, 15),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "New Jersey Economic Development Day",
        Rule::nth(5, 2, Weekday::Monday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Portugal Day",
        Rule::gregorian(6, 10),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Women Veterans Appreciation Day",
        Rule::gregorian(6, 12),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Knock Out Opioid Abuse Day",
        Rule::gregorian(10, 6),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Alice Paul Day",
        Rule::gregorian(1, 11),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Puerto Rico Day",
        Rule::nth(9, 3, Weekday::Sunday),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Hydrogen and Fuel Cell Day",
        Rule::gregorian(10, 8),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Women in Public Office Day",
        Rule::gregorian(3, 19),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Overdose Awareness Day",
        Rule::gregorian(8, 31),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "COVID-19 Heroes Day",
        Rule::gregorian(3, 9),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Frederick Douglass Day",
        Rule::gregorian(2, 14),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Gun Violence Awareness Day",
        Rule::gregorian(6, 2),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Alpha Phi Alpha Day",
        Rule::gregorian(12, 4),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Personal Carbon Footprint Awareness Day",
        Rule::gregorian(4, 22),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "International Women's Day",
        Rule::gregorian(3, 8),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Military Child Appreciation Day",
        Rule::gregorian(4, 14),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Borinqueneers Day",
        Rule::gregorian(4, 13),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "New Jersey Railroad Workers Day",
        Rule::gregorian(10, 15),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Fred Korematsu Day of Civil Liberties and the Constitution",
        Rule::gregorian(1, 30),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Upcycling Day",
        Rule::gregorian(4, 16),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Day of the Girl",
        Rule::gregorian(10, 11),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Larry Doby Day",
        Rule::gregorian(7, 5),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Fentanyl Poisoning Awareness Day",
        Rule::gregorian(7, 14),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Kimchi Day",
        Rule::gregorian(11, 22),
        &["US-NJ"],
        US_NJ_DAYS,
    )
    .read_from(2026)
    .approximate(),
    // New York.
    state_observance(
        "Haym Salomon Day",
        Rule::gregorian(1, 6),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Holocaust Remembrance Day",
        Rule::gregorian(1, 27),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Rosa Parks Day",
        Rule::gregorian(2, 4),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Susan B. Anthony Day",
        Rule::gregorian(2, 15),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Lithuanian Independence Day",
        Rule::gregorian(2, 16),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Gulf War Veterans' Day",
        Rule::gregorian(2, 28),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance("Pulaski Day", Rule::gregorian(3, 4), &["US-NY"], US_NY_DAYS).read_from(2026),
    state_observance(
        "International Women's Day",
        Rule::gregorian(3, 8),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Harriet Tubman Day",
        Rule::gregorian(3, 10),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Vietnam Veterans' Day",
        Rule::gregorian(3, 29),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "POW Recognition Day",
        Rule::gregorian(4, 9),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Coretta Scott King Day",
        Rule::gregorian(4, 27),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Workers' Memorial Day",
        Rule::gregorian(4, 28),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "New York State Teacher Day",
        Rule::nth(5, 1, Weekday::Tuesday),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Thurgood Marshall Day",
        Rule::gregorian(5, 17),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Children's Day",
        Rule::nth(6, 1, Weekday::Sunday),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Italian Independence Day",
        Rule::gregorian(6, 2),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Women Veterans Recognition Day",
        Rule::gregorian(6, 12),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Korean War Veterans' Day",
        Rule::gregorian(6, 25),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Abolition Commemoration Day",
        Rule::nth(7, 2, Weekday::Monday),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ukrainian Independence Day",
        Rule::gregorian(8, 24),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Women's Equality Day",
        Rule::gregorian(8, 26),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Battle of Plattsburgh Day / September 11th Remembrance Day",
        Rule::gregorian(9, 11),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "John Barry Day / Uncle Sam Day in the State of New York",
        Rule::gregorian(9, 13),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Friedrich Wilhelm von Steuben Memorial Day",
        Rule::gregorian(9, 17),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "New York State POW/MIA Recognition Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "War of 1812 Day",
        Rule::last(9, Weekday::Saturday),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Native-American Day",
        Rule::nth(9, 4, Weekday::Saturday),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Gold Star Mothers' Day",
        Rule::last(9, Weekday::Sunday),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Raoul Wallenberg Day",
        Rule::gregorian(10, 5),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "New Netherland Day in the State of New York",
        Rule::gregorian(10, 11),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Disabilities History Day",
        Rule::gregorian(10, 18),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Theodore Roosevelt Day",
        Rule::gregorian(10, 27),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Witness for Tolerance Day",
        Rule::gregorian(11, 9),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Elizabeth Cady Stanton Day",
        Rule::gregorian(11, 12),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "New York State School-Related Professionals Recognition Day",
        Rule::nth(11, 3, Weekday::Tuesday),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Sojourner Truth Day",
        Rule::gregorian(11, 26),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Shirley Chisholm Day",
        Rule::gregorian(11, 30),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "International Day of Persons with Disabilities",
        Rule::gregorian(12, 3),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Pearl Harbor Day",
        Rule::gregorian(12, 7),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Bastogne Day",
        Rule::gregorian(12, 16),
        &["US-NY"],
        US_NY_DAYS,
    )
    .read_from(2026),
    // North Carolina.
    state_observance(
        "American Family Day",
        Rule::nth(8, 1, Weekday::Sunday),
        &["US-NC"],
        US_NC_DAYS,
    )
    .years(Some(1979), None),
    state_observance(
        "Prisoner of War Recognition Day",
        Rule::gregorian(4, 9),
        &["US-NC"],
        US_NC_DAYS,
    )
    .years(Some(1989), None),
    state_observance(
        "Pearl Harbor Remembrance Day",
        Rule::gregorian(12, 7),
        &["US-NC"],
        US_NC_DAYS,
    )
    .years(Some(1991), None),
    state_observance(
        "North Carolina Fragile X Awareness Day",
        Rule::gregorian(7, 22),
        &["US-NC"],
        US_NC_DAYS,
    )
    .years(Some(2013), None),
    state_observance(
        "Lineman Appreciation Day",
        Rule::nth(4, 2, Weekday::Monday),
        &["US-NC"],
        US_NC_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Posttraumatic Stress Injury Awareness Day",
        Rule::gregorian(6, 27),
        &["US-NC"],
        US_NC_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Wounded Heroes Day",
        Rule::gregorian(4, 24),
        &["US-NC"],
        US_NC_DAYS,
    )
    .years(Some(2021), None),
    state_observance(
        "North Carolina Farmers Appreciation Day",
        Rule::nth(11, 2, Weekday::Wednesday),
        &["US-NC"],
        US_NC_DAYS,
    )
    .years(Some(2024), None),
    state_observance(
        "North Carolina Great Trails State Day",
        Rule::nth(10, 3, Weekday::Saturday),
        &["US-NC"],
        US_NC_DAYS,
    )
    .years(Some(2024), None),
    // North Dakota.
    state_observance(
        "Mothers' Day",
        Rule::nth(5, 2, Weekday::Sunday),
        &["US-ND"],
        US_ND_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Arbor Day",
        Rule::nth(5, 1, Weekday::Friday),
        &["US-ND"],
        US_ND_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance("Bird Day", Rule::gregorian(4, 26), &["US-ND"], US_ND_DAYS_2).read_from(2026),
    state_observance(
        "Workers' Memorial Day",
        Rule::gregorian(4, 28),
        &["US-ND"],
        US_ND_DAYS_2,
    )
    .read_from(2026),
    state_observance(
        "Gold Star Mothers' Day",
        Rule::last(9, Weekday::Sunday),
        &["US-ND"],
        US_ND_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Four Chaplains Sunday",
        Rule::nth(2, 1, Weekday::Sunday),
        &["US-ND"],
        US_ND_DAYS_2,
    )
    .read_from(2026),
    state_observance(
        "Indigenous Peoples Day",
        Rule::WeekdayOnOrAfter {
            month: 10,
            day: 5,
            weekday: Weekday::Friday,
        },
        &["US-ND"],
        US_ND_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Vietnam Veterans' Day",
        Rule::gregorian(3, 29),
        &["US-ND"],
        US_ND_DAYS_2,
    )
    .read_from(2026),
    state_observance(
        "Patriots' Day",
        Rule::nth(4, 3, Weekday::Monday),
        &["US-ND"],
        US_ND_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Right-To-Life Day",
        Rule::gregorian(1, 22),
        &["US-ND"],
        US_ND_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Fetal Alcohol Spectrum Disorder Awareness Day",
        Rule::gregorian(9, 9),
        &["US-ND"],
        US_ND_DAYS_2,
    )
    .read_from(2026),
    state_observance(
        "Prisoner of War and Missing in Action Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-ND"],
        US_ND_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "North Dakota Constitution Day",
        Rule::gregorian(10, 1),
        &["US-ND"],
        US_ND_DAYS,
    )
    .read_from(2026)
    .approximate(),
    // Ohio.
    state_observance(
        "Arbor Day",
        Rule::last(4, Weekday::Friday),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "World War I day",
        Rule::gregorian(4, 6),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "General Pulaski Memorial Day",
        Rule::gregorian(10, 11),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Native American Indian Day",
        Rule::nth(9, 4, Weekday::Saturday),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ohio statehood day",
        Rule::gregorian(3, 1),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Workers memorial day",
        Rule::gregorian(4, 28),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Destroyer Escort Day in Ohio",
        Rule::nth(6, 3, Weekday::Saturday),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Korean War Veterans' Day",
        Rule::gregorian(7, 27),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Gold Star Mothers Day",
        Rule::last(9, Weekday::Sunday),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "School Energy Conservation Day in Ohio",
        Rule::nth(3, 3, Weekday::Friday),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ohio Mammography Day",
        Rule::nth(10, 3, Weekday::Thursday),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ohio Township Day",
        Rule::gregorian(2, 1),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ohio National Guard Day",
        Rule::gregorian(7, 25),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Exemplary Adult Care Provider Day",
        Rule::gregorian(4, 18),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Dean Martin Day",
        Rule::gregorian(6, 7),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance("Tartan Day", Rule::gregorian(4, 6), &["US-OH"], US_OH_DAYS).read_from(2026),
    state_observance(
        "School Bus Drivers Appreciation Day",
        Rule::nth(5, 1, Weekday::Monday),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "George Rogers Clark Day",
        Rule::gregorian(11, 19),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Rosa Parks Day",
        Rule::gregorian(12, 1),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ohio Public Safety Employee Day",
        Rule::gregorian(9, 11),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Emancipation Day",
        Rule::gregorian(9, 22),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Heritage and Freedom Flag of the Former Republic of Vietnam Day",
        Rule::gregorian(4, 29),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Crohn's and Colitis Awareness Day",
        Rule::gregorian(5, 23),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ronald Reagan Day",
        Rule::gregorian(2, 6),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Brothers and Sisters' Day",
        Rule::nth(8, 3, Weekday::Saturday),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Lithopolis Honeyfest Day",
        Rule::WeekdayOnOrAfter {
            month: 9,
            day: 6,
            weekday: Weekday::Saturday,
        },
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Internet Safety Day",
        Rule::nth(9, 4, Weekday::Sunday),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "U.S.S. Hocking Day",
        Rule::gregorian(10, 22),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Adoption Day",
        Rule::WeekdayOnOrAfter {
            month: 11,
            day: 17,
            weekday: Weekday::Saturday,
        },
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Homeless Persons' Memorial Day",
        Rule::gregorian(12, 21),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Prescription Drug Abuse Awareness and Education Day",
        Rule::nth(5, 1, Weekday::Friday),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ohio Overdose Awareness Day",
        Rule::gregorian(8, 31),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "John Glenn Friendship 7 Day",
        Rule::gregorian(2, 20),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Elder Abuse Awareness Day",
        Rule::gregorian(6, 15),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Traumatic Brain Injury Awareness Day",
        Rule::gregorian(7, 9),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Vietnam Veterans' Day",
        Rule::gregorian(3, 30),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Pregnancy and Infant Loss Remembrance Day",
        Rule::gregorian(10, 15),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Emma \"Grandma\" Gatewood Day",
        Rule::gregorian(4, 27),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ohio Suicide Prevention Day",
        Rule::gregorian(9, 10),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Metastatic Breast Cancer Awareness Day",
        Rule::gregorian(10, 13),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Meningitis Awareness Day",
        Rule::gregorian(3, 9),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Lincoln's birthday (school commemoration)",
        Rule::gregorian(2, 12),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Washington's birthday (school commemoration)",
        Rule::gregorian(2, 22),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Memorial day (school commemoration)",
        Rule::gregorian(5, 30),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Veterans educate today's students day (V.E.T.S. day)",
        Rule::gregorian(11, 10),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Blue Star Mothers and Families Day",
        Rule::nth(7, 4, Weekday::Sunday),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Harrison Dillard Day",
        Rule::gregorian(7, 8),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "All for the Kids Awareness Day",
        Rule::gregorian(5, 15),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Fanconi Anemia Awareness Day",
        Rule::gregorian(5, 1),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Hypoparathyroidism Awareness Day",
        Rule::gregorian(6, 1),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "James A. Garfield Day",
        Rule::gregorian(11, 19),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Human Trafficking Awareness Day",
        Rule::gregorian(1, 11),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Sudden Unexpected Death in Epilepsy Awareness Day",
        Rule::gregorian(10, 26),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Rutherford B. Hayes Day",
        Rule::gregorian(10, 4),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Moses Fleetwood Walker Day",
        Rule::gregorian(10, 7),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "National Atomic Veterans Day",
        Rule::gregorian(7, 16),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ohio Tuskegee Airmen Day",
        Rule::gregorian(3, 29),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Farmer's Day",
        Rule::gregorian(10, 12),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Agriculture Day",
        Rule::gregorian(3, 21),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ohio Stillbirth Prevention Day",
        Rule::gregorian(9, 19),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Dravet Syndrome Awareness Day",
        Rule::gregorian(10, 16),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ohio Internship and Co-Op Appreciation Day",
        Rule::nth(4, 2, Weekday::Tuesday),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "International Ataxia Awareness Day",
        Rule::gregorian(9, 25),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Childhood Asthma Awareness Day",
        Rule::gregorian(5, 5),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Omphalocele Awareness Day",
        Rule::gregorian(1, 31),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Chronic Traumatic Encephalopathy Awareness Day",
        Rule::gregorian(1, 30),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Bartter Syndrome Awareness Day",
        Rule::gregorian(5, 30),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Aortic Aneurysm Awareness Day",
        Rule::gregorian(2, 13),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Sanfilippo Syndrome Awareness Day",
        Rule::gregorian(11, 16),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "S.M.A.R.T. Parent Day",
        Rule::gregorian(10, 6),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Diabetic Ketoacidosis (DKA) Awareness Day",
        Rule::gregorian(4, 26),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Cholangiocarcinoma Awareness Day",
        Rule::gregorian(2, 12),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ulysses S. Grant Day",
        Rule::gregorian(4, 27),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ohio Public Lands Day",
        Rule::last(9, Weekday::Saturday),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Hypertrophic Cardiomyopathy Awareness Day",
        Rule::nth(2, 4, Weekday::Wednesday),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Aromatic L-Amino Acid Decarboxylase Deficiency Awareness Day",
        Rule::gregorian(10, 23),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ameloblastoma Awareness Day",
        Rule::gregorian(3, 20),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Rascal Flatts Day",
        Rule::gregorian(2, 21),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "First Responders' Appreciation Day",
        Rule::gregorian(5, 24),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Jesse Owens Day",
        Rule::gregorian(9, 12),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Charles Follis Day",
        Rule::gregorian(2, 3),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Armed Services, Peace Officer, First Responder, and Dual Service Recognition Day",
        Rule::gregorian(11, 10),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Annie Glenn Communication Disorders Awareness Day",
        Rule::gregorian(2, 17),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Diffuse Intrinsic Pontine Glioma Awareness Day",
        Rule::gregorian(5, 17),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Toni Morrison Day",
        Rule::gregorian(2, 18),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Eugene \"Gene\" F. Kranz Day",
        Rule::gregorian(8, 17),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ohio Purple Heart Day",
        Rule::gregorian(8, 7),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ohio Survivors of Suicide Loss Day",
        Rule::WeekdayOnOrAfter {
            month: 11,
            day: 17,
            weekday: Weekday::Saturday,
        },
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Veterans Suicide Awareness and Prevention Day",
        Rule::gregorian(9, 22),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Superman Day",
        Rule::gregorian(6, 12),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ohio National Missing Children's Day",
        Rule::gregorian(5, 25),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Women Veterans' Day",
        Rule::gregorian(6, 12),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Buffalo Soldiers Day",
        Rule::gregorian(7, 28),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Speaker Jo Ann Davidson Day",
        Rule::gregorian(9, 28),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Ukraine Independence Day",
        Rule::gregorian(8, 24),
        &["US-OH"],
        US_OH_DAYS,
    )
    .read_from(2026),
    // Oregon.
    state_observance(
        "Minoru Yasui Day",
        Rule::gregorian(3, 28),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2016), None),
    state_observance(
        "Cherry Blossom Day",
        Rule::nth(3, 3, Weekday::Saturday),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "Korean American Day",
        Rule::gregorian(1, 13),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2007), None),
    state_observance(
        "Oregon Outdoor Recreation Day",
        Rule::nth(6, 1, Weekday::Saturday),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "Armed Forces Day (POW/MIA flag display)",
        Rule::nth(5, 3, Weekday::Saturday),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(1999), None),
    state_observance(
        "Flag Day (POW/MIA flag display)",
        Rule::gregorian(6, 14),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(1999), None),
    state_observance(
        "Oregon POW/MIA Recognition Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "Thomas Paine Day",
        Rule::gregorian(1, 29),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Ewing Young Day",
        Rule::gregorian(2, 9),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Welcome Home Vietnam Veterans Day",
        Rule::gregorian(3, 30),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2011), None),
    state_observance(
        "Holodomor Remembrance Day",
        Rule::nth(11, 4, Weekday::Saturday),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Korean War Veterans Honor Day",
        Rule::gregorian(6, 25),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2011), None),
    state_observance(
        "Edward Dickinson Baker Day",
        Rule::gregorian(2, 24),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2011), None),
    state_observance(
        "Oregon Purple Heart Recognition Day",
        Rule::gregorian(8, 7),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "First Responder Appreciation Day",
        Rule::gregorian(9, 27),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2013), None),
    state_observance(
        "Boring and Dull Day",
        Rule::gregorian(8, 9),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2013), None),
    state_observance(
        "Spirit of '45 Day",
        Rule::nth(8, 2, Weekday::Sunday),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2013), None),
    state_observance(
        "Tom McCall Day",
        Rule::gregorian(3, 22),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Honorary Artists of Oregon Day",
        Rule::gregorian(4, 14),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Bracero Program Day",
        Rule::gregorian(8, 4),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "\"Mighty Oregon\" Day",
        Rule::gregorian(3, 10),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Oregon Post-Traumatic Stress Injury Awareness Day",
        Rule::gregorian(6, 27),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2018), None),
    state_observance(
        "PANDAS/PANS Awareness Day",
        Rule::gregorian(10, 9),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Crater Lake National Park Day",
        Rule::gregorian(5, 22),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2021), None),
    state_observance(
        "Hypertrophic Cardiomyopathy Awareness Day",
        Rule::nth(2, 4, Weekday::Wednesday),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2024), None),
    state_observance(
        "Oregon Farmer and Rancher Day",
        Rule::nth(3, 3, Weekday::Tuesday),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2025), None),
    state_observance(
        "Oregon Adoption Day",
        Rule::gregorian(8, 25),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2025), None),
    state_observance(
        "Oregon Youth Suicide Awareness Day",
        Rule::gregorian(10, 9),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2025), None),
    state_observance(
        "Ruby Bridges Walk to School Day",
        Rule::gregorian(11, 14),
        &["US-OR"],
        US_OR_DAYS,
    )
    .years(Some(2025), None),
    state_observance("Nowruz Day", Rule::gregorian(3, 21), &["US-OR"], US_OR_DAYS)
        .years(Some(2025), None),
    // Pennsylvania.
    state_observance(
        "Birthday of William Penn",
        Rule::gregorian(10, 24),
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Commodore John Barry Day",
        Rule::gregorian(9, 13),
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "National Anthem Day",
        Rule::gregorian(9, 14),
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Charter Day",
        Rule::gregorian(3, 14),
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "American Loyalty Day",
        Rule::gregorian(5, 1),
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Columbus Day (12 October)",
        Rule::gregorian(10, 12),
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Local Government Day",
        Rule::gregorian(4, 15),
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance("Bird Day", Rule::gregorian(3, 21), &["US-PA"], US_PA_DAYS)
        .read_from(2026)
        .approximate(),
    state_observance(
        "Shut-in Day",
        Rule::nth(10, 3, Weekday::Sunday),
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Pennsylvania German Day",
        Rule::gregorian(6, 28),
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Hubert H. Humphrey Day",
        Rule::gregorian(5, 27),
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Pennsylvanians with Disabilities Day",
        Rule::gregorian(1, 30),
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Lithuanian Independence Day",
        Rule::gregorian(2, 16),
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Rachel Carson Day",
        Rule::gregorian(5, 27),
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Pennsylvania POW/MIA Recognition Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Peace Officers Memorial Day",
        Rule::gregorian(5, 15),
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance("Earth Day", Rule::gregorian(4, 22), &["US-PA"], US_PA_DAYS)
        .read_from(2026)
        .approximate(),
    state_observance(
        "Arbor Day",
        Rule::last(4, Weekday::Friday),
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Commonwealth Day of Prayer and Celebration of Religious Freedom",
        Rule::nth(5, 1, Weekday::Thursday),
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Rosa Parks Remembrance Day",
        Rule::WeekdayOnOrAfter {
            month: 2,
            day: 5,
            weekday: Weekday::Monday,
        },
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Bill of Rights Day",
        Rule::gregorian(12, 15),
        &["US-PA"],
        US_PA_DAYS,
    )
    .read_from(2026)
    .approximate(),
    // Rhode Island.
    state_observance(
        "Rhode Island Indian Day of the Narragansett tribe of Indians",
        Rule::WeekdayOnOrAfter {
            month: 8,
            day: 7,
            weekday: Weekday::Saturday,
        },
        &["US-RI"],
        US_RI_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Disabled American Veterans Day",
        Rule::gregorian(7, 31),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1960), None),
    state_observance(
        "V.F.W. Loyalty Day",
        Rule::gregorian(5, 1),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1960), None),
    state_observance(
        "ITAM-Vets Daisy Day",
        Rule::nth(6, 1, Weekday::Saturday),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1962), None),
    state_observance(
        "Founders Day of the Italian-American War Veterans of the United States, Incorporated",
        Rule::gregorian(2, 15),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1963), None),
    state_observance(
        "National Police Memorial Day",
        Rule::gregorian(5, 15),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1966), None),
    state_observance(
        "General Casimir Pulaski Day",
        Rule::gregorian(10, 11),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1973), None),
    state_observance(
        "Dauphine Day",
        Rule::gregorian(4, 21),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1975), None),
    state_observance(
        "Martin Luther King, Jr. Day (15 January)",
        Rule::gregorian(1, 15),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1975), None),
    state_observance(
        "Viet Nam Veterans' Day",
        Rule::gregorian(3, 29),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1981), None),
    state_observance(
        "Nurses' Day",
        Rule::nth(5, 1, Weekday::Monday),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1985), None),
    state_observance(
        "Retired Teachers' Day",
        Rule::nth(4, 1, Weekday::Wednesday),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1986), None),
    state_observance(
        "Friendship Day",
        Rule::nth(5, 2, Weekday::Friday),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1986), None),
    state_observance(
        "Lithuanian Independence Day",
        Rule::gregorian(2, 16),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1987), None),
    state_observance(
        "Saint Jean-Baptiste Day",
        Rule::gregorian(6, 24),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1988), None),
    state_observance(
        "Arbor Day",
        Rule::last(4, Weekday::Friday),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1990), None),
    state_observance(
        "Workers' Memorial Day",
        Rule::nth(4, 4, Weekday::Friday),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1990), None),
    state_observance(
        "Destroyer Escort Day",
        Rule::nth(6, 3, Weekday::Saturday),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1991), None),
    state_observance(
        "Peter Francisco Day",
        Rule::gregorian(3, 15),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1991), None),
    state_observance(
        "Social Workers' Day",
        Rule::nth(3, 2, Weekday::Wednesday),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1991), None),
    state_observance(
        "POW-MIA's Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1992), None),
    state_observance(
        "American Indian Heritage Day",
        Rule::gregorian(9, 24),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1995), None),
    state_observance(
        "Gaspee Days (Saturday)",
        Rule::nth(6, 2, Weekday::Saturday),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1995), None),
    state_observance(
        "Gaspee Days (Sunday)",
        Rule::WeekdayOnOrAfter {
            month: 6,
            day: 9,
            weekday: Weekday::Sunday,
        },
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1995), None),
    state_observance(
        "Dr. George Washington Carver Recognition Day",
        Rule::gregorian(1, 5),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1998), None),
    state_observance(
        "Korean War Veterans Memorial Day",
        Rule::gregorian(7, 27),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1998), None),
    state_observance(
        "Rhode Island Hero's Day",
        Rule::nth(5, 2, Weekday::Thursday),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1999), None),
    state_observance(
        "Combat Veterans' Day",
        Rule::nth(4, 3, Weekday::Saturday),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(2000), None),
    state_observance(
        "Cesar Chavez Day",
        Rule::gregorian(3, 31),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(2000), None),
    state_observance(
        "Rhode Island Gold Star Family Day",
        Rule::nth(10, 3, Weekday::Sunday),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(2006), None),
    state_observance(
        "Rhode Island Blue Star Parents Day",
        Rule::nth(5, 3, Weekday::Sunday),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(2014), None),
    state_observance(
        "Firefighters' and Police Officers' Appreciation Day",
        Rule::gregorian(9, 11),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(2007), None),
    state_observance(
        "Dominican Republic Independence Day",
        Rule::gregorian(2, 27),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(2007), None),
    state_observance(
        "John Clarke Day",
        Rule::nth(10, 1, Weekday::Monday),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(2008), None),
    state_observance(
        "Rhode Island Seabees Day",
        Rule::gregorian(3, 5),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Military nurses day",
        Rule::nth(6, 1, Weekday::Monday),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(2012), None),
    state_observance(
        "RI Patriot Guard Riders Day",
        Rule::gregorian(4, 12),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(2014), None),
    state_observance(
        "Ataxia Awareness Day",
        Rule::gregorian(9, 25),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "\"The Rhode Island Nine Beirut Marines\" Observance Day",
        Rule::gregorian(10, 23),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "Historical Cemetery Restoration/Awareness Day",
        Rule::nth(4, 2, Weekday::Saturday),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(2018), None),
    state_observance(
        "Rhode Island Gold Star Spouses Day",
        Rule::gregorian(4, 5),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(2024), None),
    state_observance(
        "First responders flag half-mast day (unnamed)",
        Rule::gregorian(10, 28),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(2024), None),
    state_observance(
        "White Cane Safety Day",
        Rule::gregorian(10, 15),
        &["US-RI"],
        US_RI_DAYS,
    )
    .years(Some(1971), None),
    // South Carolina.
    state_observance(
        "Arbor Day",
        Rule::nth(12, 1, Weekday::Friday),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(1940), None),
    state_observance(
        "Frances Willard Day",
        Rule::nth(10, 4, Weekday::Friday),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(1918), None),
    state_observance(
        "General Pulaski's Memorial Day",
        Rule::gregorian(10, 11),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(1932), None),
    state_observance(
        "General Francis Marion Memorial Day",
        Rule::gregorian(2, 27),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2007), None),
    state_observance(
        "Mother's Day",
        Rule::nth(5, 2, Weekday::Sunday),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(1923), None),
    state_observance(
        "Family Respect Day",
        Rule::WeekdayOnOrAfter {
            month: 5,
            day: 6,
            weekday: Weekday::Friday,
        },
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2001), None),
    state_observance(
        "Grandmother's Day",
        Rule::nth(10, 2, Weekday::Sunday),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(1970), None),
    state_observance(
        "South Carolina Day",
        Rule::gregorian(3, 18),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(1906), None),
    state_observance(
        "Spirit of '45 Day",
        Rule::nth(8, 2, Weekday::Sunday),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2014), None),
    state_observance("Loyalty Day", Rule::gregorian(5, 1), &["US-SC"], US_SC_DAYS)
        .years(Some(1957), None),
    state_observance(
        "Eartha Kitt Day",
        Rule::gregorian(1, 17),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2016), None),
    state_observance(
        "Sickle Cell Day in South Carolina",
        Rule::gregorian(6, 19),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2018), None),
    state_observance(
        "Fibromyalgia Awareness Day",
        Rule::gregorian(5, 12),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2008), None),
    state_observance(
        "Purple Heart Day",
        Rule::gregorian(8, 7),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2013), None),
    state_observance(
        "Aynor Harvest Hoe-Down Festival Weekend",
        Rule::nth(9, 3, Weekday::Saturday),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2014), None),
    state_observance(
        "Carolina Day",
        Rule::gregorian(6, 28),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(1996), None),
    state_observance(
        "State Day of Remembrance (September eleventh)",
        Rule::gregorian(9, 11),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2002), None),
    state_observance(
        "POW/MIA Recognition Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2004), None),
    state_observance(
        "Bill of Rights Day",
        Rule::gregorian(12, 15),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2006), None),
    state_observance(
        "Vietnam Veterans Survivors' and Remembrance Day",
        Rule::nth(5, 1, Weekday::Friday),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2006), None),
    state_observance(
        "A Day of Recognition for Veterans' Spouses and Families",
        Rule::WeekdayOnOrAfter {
            month: 11,
            day: 23,
            weekday: Weekday::Friday,
        },
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2014), None),
    state_observance(
        "Post-Traumatic Stress Injury (PTSI) Awareness Day",
        Rule::gregorian(6, 27),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2016), None),
    state_observance(
        "South Carolina Day of Service",
        Rule::nth(5, 3, Weekday::Saturday),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2016), None),
    state_observance(
        "Barbers' Day",
        Rule::nth(2, 3, Weekday::Wednesday),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Penn Center Heritage Day",
        Rule::nth(11, 2, Weekday::Saturday),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2018), None),
    state_observance(
        "Dr. Ronald McNair Day",
        Rule::gregorian(10, 21),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2018), None),
    state_observance(
        "Atomic Veterans Day",
        Rule::gregorian(7, 16),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2020), None),
    state_observance(
        "Historically Black Colleges and Universities Day",
        Rule::nth(2, 3, Weekday::Tuesday),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2022), None),
    state_observance(
        "Robert Smalls Day",
        Rule::gregorian(5, 13),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Women in Hunting and Fishing Awareness Day",
        Rule::nth(11, 3, Weekday::Saturday),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Clog Dancing Day",
        Rule::gregorian(8, 8),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Water Professionals Day",
        Rule::nth(3, 1, Weekday::Monday),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2024), None),
    state_observance(
        "Mayflower Compact Day",
        Rule::gregorian(11, 21),
        &["US-SC"],
        US_SC_DAYS,
    )
    .years(Some(2025), None),
    // South Dakota.
    state_observance(
        "Arbor Day",
        Rule::last(4, Weekday::Friday),
        &["US-SD"],
        US_SD_DAYS,
    )
    .years(Some(1998), None),
    state_observance(
        "POW/MIA Recognition Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-SD"],
        US_SD_DAYS,
    )
    .years(Some(2013), None),
    // Tennessee.
    state_observance(
        "Robert E. Lee Day",
        Rule::gregorian(1, 19),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Abraham Lincoln Day",
        Rule::gregorian(2, 12),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Andrew Jackson Day",
        Rule::gregorian(3, 15),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Memorial Day or Confederate Decoration Day",
        Rule::gregorian(6, 3),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Nathan Bedford Forrest Day",
        Rule::gregorian(7, 13),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Mothers' Day",
        Rule::nth(5, 2, Weekday::Sunday),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Statehood Day",
        Rule::gregorian(6, 1),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Family Day",
        Rule::last(8, Weekday::Sunday),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Franklin D. Roosevelt Day",
        Rule::gregorian(1, 30),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "American Indian Day",
        Rule::nth(9, 4, Weekday::Monday),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Scottish, Scots-Irish Heritage Day",
        Rule::gregorian(6, 24),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Workers' Memorial Day",
        Rule::gregorian(4, 28),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "John Sevier Day",
        Rule::gregorian(6, 23),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Harriet Tubman Day",
        Rule::gregorian(3, 10),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Bluegrass Day",
        Rule::nth(5, 4, Weekday::Saturday),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Emancipation Day",
        Rule::gregorian(8, 8),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Patriots' Day",
        Rule::gregorian(4, 19),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Wilma Rudolph Day",
        Rule::gregorian(6, 23),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Vietnam Veterans Day",
        Rule::gregorian(3, 29),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Tennessee and United States Constitutions Day",
        Rule::gregorian(9, 17),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Firefighters' Memorial Day",
        Rule::gregorian(10, 9),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Gold Star Mother's Day",
        Rule::last(9, Weekday::Sunday),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Tennessee National Guard Day",
        Rule::gregorian(3, 3),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Transverse Myelitis Awareness Day",
        Rule::gregorian(6, 6),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Police Memorial Day",
        Rule::gregorian(5, 11),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Tennessee Missing Children's Day",
        Rule::gregorian(3, 4),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Tennessee Rural Mayor's Day",
        Rule::nth(10, 1, Weekday::Monday),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Febb Burn Day",
        Rule::gregorian(8, 18),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Mrs. Rosa L. Parks Day",
        Rule::gregorian(12, 1),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Post-Traumatic Stress Injury Awareness Day",
        Rule::gregorian(6, 27),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Women's Suffrage Day",
        Rule::gregorian(8, 18),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Women's Veterans Day",
        Rule::gregorian(6, 12),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "D-Day Remembrance Day in Tennessee",
        Rule::gregorian(6, 6),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Tennessee Manufacturing Day",
        Rule::nth(10, 1, Weekday::Friday),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Star-Spangled Banner Day",
        Rule::gregorian(9, 14),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Historically Black Colleges and Universities Day",
        Rule::gregorian(11, 8),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Alexander Disease Day",
        Rule::gregorian(4, 7),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Ida B. Wells Day",
        Rule::gregorian(7, 16),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Tennessee Lineworker Appreciation Day",
        Rule::nth(4, 2, Weekday::Monday),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "PANDAS Awareness Day",
        Rule::gregorian(10, 9),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance("APDS Day", Rule::gregorian(10, 1), &["US-TN"], US_TN_DAYS)
        .read_from(2026)
        .approximate(),
    state_observance(
        "James K. Polk Day",
        Rule::gregorian(11, 2),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Fathers' Day",
        Rule::nth(6, 3, Weekday::Sunday),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Clog Dancing Day in the Volunteer State",
        Rule::gregorian(8, 8),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "General Pulaski Memorial Day",
        Rule::gregorian(10, 11),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance("Law Day", Rule::gregorian(5, 1), &["US-TN"], US_TN_DAYS)
        .read_from(2026)
        .approximate(),
    state_observance(
        "Gold Star Father's Day",
        Rule::gregorian(11, 9),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Sanctity of Life Day",
        Rule::gregorian(1, 22),
        &["US-TN"],
        US_TN_DAYS,
    )
    .read_from(2026)
    .approximate(),
    // Texas.
    state_observance(
        "Sam Rayburn Day",
        Rule::gregorian(1, 6),
        &["US-TX"],
        US_TX_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Former Prisoners of War Recognition Day",
        Rule::gregorian(4, 9),
        &["US-TX"],
        US_TX_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Father of Texas Day",
        Rule::gregorian(11, 3),
        &["US-TX"],
        US_TX_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Texas Parents Day",
        Rule::nth(8, 2, Weekday::Sunday),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(1999), None),
    state_observance(
        "State of Texas Anniversary Remembrance Day (STAR Day)",
        Rule::gregorian(2, 19),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(1999), None),
    state_observance(
        "Public School Paraprofessional Day",
        Rule::nth(5, 2, Weekday::Wednesday),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2001), None),
    state_observance(
        "Texas First Responders Day",
        Rule::gregorian(9, 11),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2003), None),
    state_observance(
        "Women's Independence Day",
        Rule::gregorian(8, 26),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2005), None),
    state_observance(
        "Texian Navy Day",
        Rule::nth(9, 3, Weekday::Saturday),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2005), None),
    state_observance(
        "Volunteers For Democracy Day",
        Rule::nth(1, 2, Weekday::Tuesday),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2007), None),
    state_observance(
        "Texas Adoption Day",
        Rule::WeekdayOnOrAfter {
            month: 11,
            day: 17,
            weekday: Weekday::Saturday,
        },
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2007), None),
    state_observance(
        "Dr. Hector P. Garcia Day",
        Rule::nth(9, 3, Weekday::Wednesday),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "American Indian Heritage Day",
        Rule::last(9, Weekday::Friday),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2013), None),
    state_observance(
        "Texas Arbor Day",
        Rule::nth(11, 1, Weekday::Friday),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2013), None),
    state_observance(
        "Vietnam Veterans Day",
        Rule::gregorian(3, 29),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Influenza Awareness Day",
        Rule::gregorian(10, 1),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2013), None),
    state_observance(
        "Willie Velasquez Day",
        Rule::gregorian(5, 9),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2013), None),
    state_observance(
        "National Day of The Cowboy",
        Rule::nth(7, 4, Weekday::Saturday),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Iwo Jima Day",
        Rule::gregorian(2, 19),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Lung Cancer Awareness Day",
        Rule::gregorian(5, 24),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Gold Star Mother's Day",
        Rule::last(9, Weekday::Sunday),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2015), None),
    state_observance(
        "Women Veterans Day",
        Rule::gregorian(6, 12),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "Fallen Law Enforcement Officer Day",
        Rule::gregorian(7, 7),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "Law Enforcement Appreciation Day",
        Rule::gregorian(1, 9),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "Breast Reconstruction Awareness Day",
        Rule::nth(10, 3, Weekday::Wednesday),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2017), None),
    state_observance("BRAVE Day", Rule::gregorian(3, 21), &["US-TX"], US_TX_DAYS)
        .years(Some(2017), None),
    state_observance(
        "Waxahachie Chautauqua Day",
        Rule::gregorian(7, 26),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2017), None),
    state_observance(
        "Military Spouse Appreciation Day",
        Rule::gregorian(5, 8),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Space Exploration Day",
        Rule::gregorian(7, 20),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Texas Girls in STEM Day",
        Rule::gregorian(3, 1),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Texas Firefighters Day",
        Rule::gregorian(5, 4),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Sexual Assault Survivors Day",
        Rule::gregorian(1, 28),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Diffuse Intrinsic Pontine Glioma Awareness Day",
        Rule::gregorian(5, 17),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Master Sergeant Jonathan J. Dunbar Day",
        Rule::gregorian(3, 30),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Blue Tie Day",
        Rule::gregorian(6, 13),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "International Holocaust Remembrance Day",
        Rule::gregorian(1, 27),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2021), None),
    state_observance(
        "Victims of Communism Day",
        Rule::gregorian(11, 7),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2021), None),
    state_observance(
        "Supportive Palliative Care Awareness Day",
        Rule::gregorian(10, 10),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Vanessa Guillén Day",
        Rule::gregorian(9, 30),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Rosa Parks Day",
        Rule::gregorian(12, 1),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2021), None),
    state_observance(
        "COVID-19 Heroes and Memorial Day",
        Rule::gregorian(3, 4),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Entrepreneurs with Disabilities Day",
        Rule::gregorian(10, 17),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Special Forces Day",
        Rule::gregorian(6, 28),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "Profound Autism Awareness Day",
        Rule::gregorian(3, 17),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2025), None),
    state_observance(
        "Unplug Texas Day",
        Rule::gregorian(10, 21),
        &["US-TX"],
        US_TX_DAYS,
    )
    .years(Some(2025), None),
    // Utah.
    state_observance(
        "Utah History Day at the Capitol",
        Rule::WeekdayOnOrAfter {
            month: 1,
            day: 26,
            weekday: Weekday::Friday,
        },
        &["US-UT"],
        US_UT_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Day of Remembrance for Incarceration of Japanese Americans",
        Rule::gregorian(2, 19),
        &["US-UT"],
        US_UT_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Utah State Flag Day",
        Rule::gregorian(3, 9),
        &["US-UT"],
        US_UT_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Vietnam Veterans Recognition Day",
        Rule::gregorian(3, 29),
        &["US-UT"],
        US_UT_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Utah Railroad Workers Day",
        Rule::gregorian(5, 10),
        &["US-UT"],
        US_UT_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Dandy-Walker Syndrome Awareness Day",
        Rule::gregorian(5, 11),
        &["US-UT"],
        US_UT_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Armed Forces Day",
        Rule::nth(5, 3, Weekday::Saturday),
        &["US-UT"],
        US_UT_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Arthrogryposis Multiplex Congenita Awareness Day",
        Rule::gregorian(6, 30),
        &["US-UT"],
        US_UT_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Navajo Code Talker Day",
        Rule::gregorian(8, 14),
        &["US-UT"],
        US_UT_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Rachael Runyan/Missing and Exploited Children's Day",
        Rule::gregorian(8, 26),
        &["US-UT"],
        US_UT_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "September 11th Day of Remembrance",
        Rule::gregorian(9, 11),
        &["US-UT"],
        US_UT_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Constitution Day",
        Rule::gregorian(9, 17),
        &["US-UT"],
        US_UT_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "POW/MIA Recognition Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-UT"],
        US_UT_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Victims of Communism Memorial Day",
        Rule::gregorian(11, 7),
        &["US-UT"],
        US_UT_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Indigenous People Day",
        Rule::WeekdayOnOrAfter {
            month: 11,
            day: 19,
            weekday: Weekday::Monday,
        },
        &["US-UT"],
        US_UT_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Bill of Rights Day",
        Rule::gregorian(12, 15),
        &["US-UT"],
        US_UT_DAYS,
    )
    .read_from(2026)
    .approximate(),
    // Vermont.
    state_observance(
        "Arbor Day",
        Rule::nth(5, 1, Weekday::Friday),
        &["US-VT"],
        US_VT_DAYS,
    )
    .read_from(2026),
    state_observance(
        "POW-MIA Recognition Day",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-VT"],
        US_VT_DAYS,
    )
    .years(Some(1998), None),
    state_observance(
        "Green Up Day",
        Rule::nth(5, 1, Weekday::Saturday),
        &["US-VT"],
        US_VT_DAYS,
    )
    .years(Some(2014), None),
    // Virginia.
    state_observance(
        "Yorktown Day",
        Rule::gregorian(10, 19),
        &["US-VA"],
        US_VA_DAYS,
    )
    .years(Some(1983), None),
    state_observance(
        "Motherhood and Apple Pie Day",
        Rule::gregorian(1, 26),
        &["US-VA"],
        US_VA_DAYS,
    )
    .years(Some(1989), None),
    state_observance(
        "Mother's Day",
        Rule::nth(5, 2, Weekday::Sunday),
        &["US-VA"],
        US_VA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Commonwealth Day of Prayer",
        Rule::nth(5, 1, Weekday::Thursday),
        &["US-VA"],
        US_VA_DAYS,
    )
    .years(Some(1997), None),
    state_observance(
        "Arbor Day",
        Rule::last(4, Weekday::Friday),
        &["US-VA"],
        US_VA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Dogwood Day",
        Rule::nth(4, 3, Weekday::Saturday),
        &["US-VA"],
        US_VA_DAYS,
    )
    .years(Some(1952), None),
    state_observance(
        "First Lady's Day in Virginia",
        Rule::gregorian(6, 2),
        &["US-VA"],
        US_VA_DAYS,
    )
    .years(Some(1960), None),
    state_observance(
        "Pearl Harbor Remembrance Day",
        Rule::gregorian(12, 7),
        &["US-VA"],
        US_VA_DAYS,
    )
    .years(Some(1983), None),
    state_observance(
        "Armed Forces Day (POW/MIA flag display)",
        Rule::nth(5, 3, Weekday::Saturday),
        &["US-VA"],
        US_VA_DAYS,
    )
    .years(Some(2001), None),
    state_observance(
        "Flag Day (POW/MIA flag display)",
        Rule::gregorian(6, 14),
        &["US-VA"],
        US_VA_DAYS,
    )
    .years(Some(2001), None),
    state_observance(
        "National POW/MIA Recognition Day (POW/MIA flag display)",
        Rule::nth(9, 3, Weekday::Friday),
        &["US-VA"],
        US_VA_DAYS,
    )
    .years(Some(2001), None),
    state_observance(
        "Vietnam Human Rights Day",
        Rule::gregorian(5, 11),
        &["US-VA"],
        US_VA_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Day of recognition for early childhood and day-care providers and professionals",
        Rule::WeekdayOnOrAfter {
            month: 5,
            day: 6,
            weekday: Weekday::Friday,
        },
        &["US-VA"],
        US_VA_DAYS,
    )
    .years(Some(1989), None),
    state_observance(
        "Day of recognition for direct care staffs and other long-term care professionals",
        Rule::nth(6, 2, Weekday::Wednesday),
        &["US-VA"],
        US_VA_DAYS,
    )
    .years(Some(2005), None),
    state_observance(
        "Day of recognition for bone marrow donor programs",
        Rule::gregorian(4, 8),
        &["US-VA"],
        US_VA_DAYS,
    )
    .years(Some(1992), None),
    state_observance(
        "Bill of Rights Day",
        Rule::gregorian(12, 15),
        &["US-VA"],
        US_VA_DAYS,
    )
    .years(Some(1998), None),
    state_observance(
        "Citizenship Day",
        Rule::gregorian(9, 17),
        &["US-VA"],
        US_VA_DAYS,
    )
    .years(Some(1974), None),
    state_observance(
        "White Cane Safety Day",
        Rule::gregorian(10, 15),
        &["US-VA"],
        US_VA_DAYS,
    )
    .years(Some(1972), None),
    state_observance(
        "Day of Appreciation for American Indians",
        Rule::WeekdayOnOrAfter {
            month: 11,
            day: 21,
            weekday: Weekday::Wednesday,
        },
        &["US-VA"],
        US_VA_DAYS,
    )
    .read_from(2026),
    // Washington.
    state_observance(
        "Human trafficking awareness day",
        Rule::gregorian(1, 11),
        &["US-WA"],
        US_WA_DAYS,
    )
    .years(Some(2016), None),
    state_observance(
        "Korean American day",
        Rule::gregorian(1, 13),
        &["US-WA"],
        US_WA_DAYS,
    )
    .years(Some(2007), None),
    state_observance(
        "Washington army and air national guard day",
        Rule::gregorian(1, 26),
        &["US-WA"],
        US_WA_DAYS,
    )
    .years(Some(1991), None),
    state_observance(
        "Civil liberties day of remembrance",
        Rule::gregorian(2, 19),
        &["US-WA"],
        US_WA_DAYS,
    )
    .years(Some(2003), None),
    state_observance(
        "Billy Frank Jr. day",
        Rule::gregorian(3, 9),
        &["US-WA"],
        US_WA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Welcome home Vietnam veterans day",
        Rule::gregorian(3, 30),
        &["US-WA"],
        US_WA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Cesar Chavez day",
        Rule::gregorian(3, 31),
        &["US-WA"],
        US_WA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Former prisoner of war recognition day",
        Rule::gregorian(4, 9),
        &["US-WA"],
        US_WA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Dolores Huerta day",
        Rule::gregorian(4, 10),
        &["US-WA"],
        US_WA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Mother Joseph day",
        Rule::gregorian(4, 16),
        &["US-WA"],
        US_WA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Water safety day",
        Rule::gregorian(5, 15),
        &["US-WA"],
        US_WA_DAYS,
    )
    .years(Some(2023), None),
    state_observance(
        "National Korean war veterans armistice day",
        Rule::gregorian(7, 27),
        &["US-WA"],
        US_WA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Purple heart recipient recognition day",
        Rule::gregorian(8, 7),
        &["US-WA"],
        US_WA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Marcus Whitman day",
        Rule::gregorian(9, 4),
        &["US-WA"],
        US_WA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Public lands day",
        Rule::nth(9, 4, Weekday::Saturday),
        &["US-WA"],
        US_WA_DAYS,
    )
    .years(Some(2019), None),
    state_observance(
        "Washington state children's day",
        Rule::nth(10, 2, Weekday::Sunday),
        &["US-WA"],
        US_WA_DAYS,
    )
    .years(Some(1993), None),
    state_observance(
        "Columbus day",
        Rule::gregorian(10, 12),
        &["US-WA"],
        US_WA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Pearl Harbor remembrance day",
        Rule::gregorian(12, 7),
        &["US-WA"],
        US_WA_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Blood donor day",
        Rule::gregorian(12, 18),
        &["US-WA"],
        US_WA_DAYS,
    )
    .years(Some(2020), None),
    // West Virginia.
    state_observance(
        "Susan B. Anthony Day",
        Rule::WeekdayOnOrAfter {
            month: 11,
            day: 2,
            weekday: Weekday::Tuesday,
        },
        &["US-WV"],
        US_WV_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Vietnam Veteran Recognition Day",
        Rule::gregorian(3, 30),
        &["US-WV"],
        US_WV_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Purple Heart Recognition Day",
        Rule::gregorian(8, 7),
        &["US-WV"],
        US_WV_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Korean War Veteran Recognition Day",
        Rule::gregorian(7, 27),
        &["US-WV"],
        US_WV_DAYS,
    )
    .read_from(2026),
    state_observance(
        "West Virginia Day of Prayer",
        Rule::nth(5, 1, Weekday::Thursday),
        &["US-WV"],
        US_WV_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Marshall University 75 Memorial Day",
        Rule::gregorian(11, 14),
        &["US-WV"],
        US_WV_DAYS,
    )
    .read_from(2026),
    // Wisconsin.
    state_observance(
        "Family Sunday",
        Rule::nth(11, 1, Weekday::Sunday),
        &["US-WI"],
        US_WI_DAYS,
    )
    .years(Some(1973), None),
    state_observance(
        "Wisconsin Firefighters Memorial Day",
        Rule::WeekdayOnOrAfter {
            month: 10,
            day: 9,
            weekday: Weekday::Saturday,
        },
        &["US-WI"],
        US_WI_DAYS,
    )
    .read_from(2026),
    state_observance(
        "Indian Rights Day",
        Rule::gregorian(7, 4),
        &["US-WI"],
        US_WI_DAYS,
    )
    .read_from(2026),
    state_observance(
        "William D. Hoard Day",
        Rule::gregorian(10, 10),
        &["US-WI"],
        US_WI_DAYS,
    )
    .years(Some(2009), None),
    state_observance(
        "Ronald W. Reagan Day",
        Rule::gregorian(2, 6),
        &["US-WI"],
        US_WI_DAYS,
    )
    .years(Some(2011), None),
    // Wyoming.
    state_observance(
        "Arbor Day",
        Rule::last(4, Weekday::Monday),
        &["US-WY"],
        US_WY_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Nellie Tayloe Ross's birthday",
        Rule::gregorian(11, 29),
        &["US-WY"],
        US_WY_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Pearl Harbor Remembrance Day",
        Rule::gregorian(12, 7),
        &["US-WY"],
        US_WY_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Juneteenth holiday",
        Rule::nth(6, 3, Weekday::Saturday),
        &["US-WY"],
        US_WY_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Wyoming Veterans Welcome Home Day",
        Rule::gregorian(3, 30),
        &["US-WY"],
        US_WY_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Purple Heart Day",
        Rule::gregorian(8, 7),
        &["US-WY"],
        US_WY_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Estelle Reel Day",
        Rule::gregorian(1, 7),
        &["US-WY"],
        US_WY_DAYS,
    )
    .read_from(2026)
    .approximate(),
    state_observance(
        "Moon Landing Day",
        Rule::gregorian(7, 20),
        &["US-WY"],
        US_WY_DAYS,
    )
    .read_from(2026)
    .approximate(),
];
