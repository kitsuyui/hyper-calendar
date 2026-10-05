//! The weekend of each national table and of each exchange, with the first
//! year its sources state one (ADR 0015).
//!
//! A weekend is a statute's or an authority's: the days of the week on which
//! the offices of the country, or the exchange, do not work. A table carries
//! the days with the date they began, and the years before the first date a
//! source read gives are a gap, [`crate::rule::UNREAD_WEEKEND`], not a
//! Saturday and a Sunday assumed. A source that gives only a year dates a
//! regime from 1 January of it, and one that gives only a month from the
//! first of that month, unless the policy's note says otherwise.
//!
//! Where no dated source was found, the weekend is read from 2026, the year
//! the sources were read in, and every earlier year is a gap: the live page of
//! Wikipedia's "Workweek and weekend", retrieved 2026-10-04 and secondary, and
//! the table's own sources for its holidays say no more. The note of each
//! static whose weekend is dated names its sources with their URLs and the date
//! they were retrieved. The per-country reading, with the sources' keys in
//! `docs/references.bib` and the countries examined and not carried, is in
//! `docs/systems/national-weekends.md`.

use hc_calendar::Weekday;
use hc_calendar::Weekday::{Friday, Saturday, Sunday, Thursday};

use crate::rule::WeekendPolicy;

const SAT_SUN: &[Weekday] = &[Saturday, Sunday];
const FRI_SAT: &[Weekday] = &[Friday, Saturday];
const THU_FRI: &[Weekday] = &[Thursday, Friday];
const FRI_SUN: &[Weekday] = &[Friday, Sunday];
const FRI_SAT_SUN: &[Weekday] = &[Friday, Saturday, Sunday];
const FRIDAY: &[Weekday] = &[Friday];
const SATURDAY: &[Weekday] = &[Saturday];
const SUNDAY: &[Weekday] = &[Sunday];

/// Andorra: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static AD: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// United Arab Emirates: Thursday and Friday for the federal government's
/// offices and the schools from 1999, the year of the earliest report read;
/// Friday and Saturday from Friday 1 September 2006, by the Cabinet decision
/// WAM reported on 16 May 2006; Saturday and Sunday, with a half-day Friday,
/// from 1 January 2022, as the Government portal states.
///
/// The Government of Sharjah's own offices keep Friday, Saturday and Sunday
/// from 1 January 2022: the Sharjah Executive Council's decision of December
/// 2021 gave them a four-day week, Monday to Thursday, as Khaleej Times of 9
/// December 2021 and the circular Gulf News reported on 28 December 2021 say,
/// and the Government portal's page on the public sector's working hours, as
/// updated on 12 August 2026, still says that the employees of the Government
/// of Sharjah work four days a week. It is the weekend of the emirate's
/// government, not of its private sector, which the federal Labour Law governs.
/// Before 2022 Sharjah kept the federal weekend, which the policies above give
/// (ADR 0015).
///
/// Sources, retrieved 2026-10-05: Gulf News, “Friday-Saturday weekend in UAE
/// from September” (WAM's report of the Cabinet's decision of May 2006)
/// <http://gulfnews.com/news/uae/general/friday-saturday-weekend-in-uae-from-september-1.237326>;
/// The National, “When Saturday replaced Thursday: the UAE's last weekend
/// change”, 7 December 2021
/// <https://www.thenationalnews.com/uae/2021/12/07/uae-weekend-change-when-friday-was-the-only-day-off/>;
/// UAE Government portal, “Working hours in the public sector”
/// <https://u.ae/en/information-and-services/jobs/Sector-of-employment/working-in-uae-government-sector/working-hours-in-the-public-sector>;
/// Khaleej Times, “3-day weekend in Sharjah: Workweek timings announced”, 9
/// December 2021
/// <https://www.khaleejtimes.com/uae/government/3-day-weekend-in-sharjah-workweek-timings-announced>;
/// Gulf News, “New UAE weekend: Sharjah clarifies timings, shifts for
/// government departments”, 28 December 2021
/// <https://gulfnews.com/uae/government/new-uae-weekend-sharjah-clarifies-timings-shifts-for-government-departments-1.84638602>.
pub static AE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(1998),
    WeekendPolicy::of(THU_FRI).from(1999).until_day(2006, 8, 31),
    WeekendPolicy::of(FRI_SAT).from_day(2006, 9, 1).until(2021),
    WeekendPolicy::of(SAT_SUN).from(2022),
    WeekendPolicy::of(FRI_SAT_SUN)
        .from(2022)
        .in_regions(&["AE-SH"]),
];

/// Afghanistan: Friday alone for the state institutions, which worked six days
/// a week with only Friday off, as RFE/RL wrote on 1 December 2010 (the first
/// regime is read from 2010, the year of that report); Thursday and Friday from
/// Thursday 2 December 2010, when the cabinet under President Karzai made
/// Thursday a second day off as a smog measure, first until 20 March 2011,
/// extended on 15 March for three months and from 22 June 2011 for an unknown
/// period (the Afghanistan Analysts Network, RFE/RL and Pajhwok). The sources
/// say Kabul, and whether the provinces followed is not stated. A Pakistani
/// daily of 24 July 2018 still gives Thursday and Friday as Afghanistan's
/// weekend, so they run to the end of 2018.
///
/// The years 2019 to 2025 are not read: no dated source says what the Taliban's
/// government kept. The law read for 2026 is Friday: the Ministry of Labour and
/// Social Affairs' notice for Arafah and Eid al-Adha 1447 counts "four working
/// days" from Tuesday 9 Dhu al-Hijjah to a return on Sunday the 14th, passing
/// over the Friday and counting the Thursday and the Saturday. The sources of
/// 2010 to 2018 disagree with that reading for those years, and each is carried
/// for its own years.
///
/// Sources, retrieved 2026-10-05: Afghanistan Analysts Network, “Afghan
/// Government Declares Kabul Smog Holiday”, 30 November 2010
/// <https://www.afghanistan-analysts.org/en/reports/rights-freedom/afghan-government-declares-kabul-smog-holiday/>;
/// RFE/RL, “Afghans To Get Additional Day Off Each Week”, 1 December 2010
/// <https://www.rferl.org/a/2236205.html>; Pajhwok Afghan News, “Thursday
/// holiday extended for unknown period”, 22 June 2011
/// <https://pajhwok.com/2011/06/22/thursday-holiday-extended-unknown-period/>;
/// Daily Times (Pakistan), 24 July 2018
/// <https://dailytimes.com.pk/272080/afghanistan-continues-to-move-behind-the-times/>;
/// Wikipedia, “Workweek and weekend”
/// <https://en.wikipedia.org/wiki/Workweek_and_weekend>.
pub static AF: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2009),
    WeekendPolicy::of(FRIDAY).from(2010).until_day(2010, 12, 1),
    WeekendPolicy::of(THU_FRI).from_day(2010, 12, 2).until(2018),
    WeekendPolicy::unread().from(2019).until(2025),
    WeekendPolicy::of(FRIDAY).from(2026),
];

/// Antigua and Barbuda: Saturday and Sunday, from
/// 2026, the year the sources were read in; no dated source for an earlier
/// weekend was found, so the years before are a gap. Wikipedia's "Workweek and
/// weekend", retrieved 2026-10-04 (secondary), is the page read.
pub static AG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Albania: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static AL: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Armenia: Saturday and Sunday from 21 June 2005, the day the Labour Code
/// (HO-124-N, adopted on 9 November 2004 and signed on 14 December 2004) came
/// into force: article 155 gives the general rest day as Sunday and, "in the
/// case of a five-day working week, Saturday and Sunday", except in the cases
/// of parts 2 to 4 of the article and of other legal acts. It is the general
/// labour law, which covers the public sector. The redaction read is the
/// original, whose status ARLIS gives as in force from 21 June to 22 August
/// 2005; whether later amendments changed the article was not checked.
///
/// Sources, retrieved 2026-10-05: Labour Code of Armenia (HO-124-N), article
/// 155, original redaction (ARLIS)
/// <https://www.arlis.am/documentview.aspx?docid=51>.
pub static AM: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2005, 6, 20),
    WeekendPolicy::of(SAT_SUN).from_day(2005, 6, 21),
];

/// Angola: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static AO: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Argentina: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static AR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Austria: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static AT: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Australia: Saturday and Sunday from 14 May 2015, when the Australian Public
/// Service Enterprise Award 2015 (MA000124) commenced: clause 8.2(c)(i) puts
/// the ordinary hours of work between 8.00 am and 6.00 pm, Monday to Friday.
/// The award supersedes the Australian Public Service Award 1998, which was not
/// read, so the years before are a gap.
///
/// Sources, retrieved 2026-10-05: Australian Public Service Enterprise Award
/// 2015 [MA000124] (Fair Work Ombudsman)
/// <https://awards.fairwork.gov.au/MA000124.html>.
pub static AU: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2015, 5, 13),
    WeekendPolicy::of(SAT_SUN).from_day(2015, 5, 14),
];

/// Azerbaijan: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static AZ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Bosnia and Herzegovina: Saturday and Sunday, from
/// 2026, the year the sources were read in; no dated source for an earlier
/// weekend was found, so the years before are a gap. Wikipedia's "Workweek and
/// weekend", retrieved 2026-10-04 (secondary), is the page read.
pub static BA: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Barbados: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static BB: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Bangladesh: Friday from March 1982 and Friday and Saturday from 9 September
/// 2005 for government, semi-government and autonomous offices, as news reports
/// of the decisions give them (the month of 1982 is the report's; the policy
/// begins the month after); the notifications' own count of weekly holidays in
/// 2025 and 2026 agrees with Friday and Saturday.
///
/// Sources, retrieved 2026-10-05: Arab News, “Bangladesh Introduces Two-Day
/// Weekly Off”, 7 September 2005 <https://www.arabnews.com/node/272622>; The
/// Financial Express (Dhaka), “Weekend holiday schedule” (undated)
/// <https://today.thefinancialexpress.com.bd/print/weekend-holiday-schedule>.
pub static BD: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(1982, 3, 31),
    WeekendPolicy::of(FRIDAY)
        .from_day(1982, 4, 1)
        .until_day(2005, 9, 8),
    WeekendPolicy::of(FRI_SAT).from_day(2005, 9, 9),
];

/// Belgium: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static BE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Burkina Faso: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static BF: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Bulgaria: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static BG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Bahrain: Thursday and Friday for the public sector from February 1990 (the
/// policy begins in March), and Friday and Saturday from Saturday 2 September
/// 2006, as Gulf News reported the Government's decision the next day.
///
/// Sources, retrieved 2026-10-05: Gulf News, “Bahrain adopts new weekend for
/// public staff”
/// <https://gulfnews.com/world/gulf/bahrain/bahrain-adopts-new-weekend-for-public-staff-1.253616>;
/// Gulf News, “Friday-Saturday weekend in Bahrain sought”
/// <https://gulfnews.com/world/gulf/bahrain/friday-saturday-weekend-in-bahrain-sought-1.237823>.
pub static BH: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(1990, 2, 28),
    WeekendPolicy::of(THU_FRI)
        .from_day(1990, 3, 1)
        .until_day(2006, 8, 31),
    WeekendPolicy::of(FRI_SAT).from_day(2006, 9, 1),
];

/// Burundi: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static BI: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Benin: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static BJ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Brunei: Friday and Sunday. The public service works "Isnin hingga Sabtu
/// (kecuali Jumaat, Ahad serta hari-hari kelepasan awam)", and every circular
/// read gives a substitute for a holiday on a Friday or a Sunday and none for
/// one on a Saturday. A United States Department of Commerce page on Brunei's
/// business customs, which lists the 2019 holidays, says government offices are
/// closed on Fridays and Sundays; it is the earliest dated source and the
/// policy begins with 2019.
///
/// Sources, retrieved 2026-10-05: U.S. Department of Commerce, “Brunei -
/// Business Customs” (the 2019 holidays)
/// <https://www.privacyshield.gov/ps/article?id=Brunei-Business-Customs>.
pub static BN: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2018),
    WeekendPolicy::of(FRI_SUN).from(2019),
];

/// Bolivia: Saturday and Sunday from 26 December 2010, the date of Decreto
/// Supremo 751, which fixes the working hours of the Executive and the bodies
/// under it as 08:30 to 12:30 and 14:30 to 18:30 "de lunes a viernes" (the
/// decree's date; the day it came into force is not on the page read). An
/// earlier decree, 29000 of 2 January 2007, gives the same days for the
/// Presidency and the Ministry of the Presidency only, and is not carried as
/// the country's.
///
/// Sources, retrieved 2026-10-05: Bolivia, Decreto Supremo No. 751 of 26
/// December 2010 (Lexivox) <https://www.lexivox.org/norms/BO-DS-N751.html>;
/// Bolivia, Decreto Supremo No. 29000 of 2 January 2007 (Lexivox)
/// <https://www.lexivox.org/norms/BO-DS-29000.html>.
pub static BO: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2010, 12, 25),
    WeekendPolicy::of(SAT_SUN).from_day(2010, 12, 26),
];

/// Brazil: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static BR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Bahamas: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static BS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Bhutan: Saturday and Sunday for government offices and civil servants, as
/// The Bhutanese's article of 14 May 2016 says ("no school on Saturday":
/// schools keep a six-day week) and the Royal Bhutanese Embassy's hours; no
/// instrument was found. The policy begins with 2016.
///
/// Sources, retrieved 2026-10-05: The Bhutanese, “No School on Saturday”, 14
/// May 2016 <https://thebhutanese.bt/no-school-on-saturday/>.
pub static BT: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2015),
    WeekendPolicy::of(SAT_SUN).from(2016),
];

/// Botswana: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static BW: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Belarus: Saturday and Sunday from 2000, the first whole year after the
/// Labour Code of 26 July 1999: its article 136 gives a five-day week two days
/// off in each calendar week and names Sunday as the common day off, the second
/// being set by the internal rules or the schedule, and the state legal portal
/// pravo.by says that the classical five-day week has "two days off, Saturday
/// and Sunday". The Code's day of entry into force was not read, so the policy
/// begins with the year after, and Saturday as the second day is the custom the
/// portal states and not a text of the Code. The Soviet-era weekend is not
/// read.
///
/// Sources, retrieved 2026-10-05: Labour Code of Belarus of 26 July 1999,
/// article 136 (zakony-by.com's copy)
/// <https://zakony-by.com/trudovoj_kodeks_rb/136.htm>; pravo.by, «Нормы
/// рабочего времени и оплата труда», January 2022
/// <https://pravo.by/novosti/obshchestvenno-politicheskie-i-v-oblasti-prava/2022/january/68381/>.
pub static BY: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(1999),
    WeekendPolicy::of(SAT_SUN).from(2000),
];

/// Belize: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static BZ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Canada: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static CA: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Democratic Republic of the Congo: Sunday alone, the public service working
/// six days, until 31 July 2024, and Saturday and Sunday from 1 August 2024,
/// when the Government's decree no. 24/09 of 17 February 2024 took effect after
/// a postponement from 1 July: the Public Service portal's page of 31 July 2024
/// says that the working week goes from six to five days, Monday to Friday,
/// "avec le samedi désormais considéré comme jour non ouvrable". The
/// Sunday-alone regime is read from 2024, the year of the report that gives it.
///
/// Sources, retrieved 2026-10-05: Portail de la Fonction Publique (RDC),
/// «Réaménagement des horaires de service», 31 July 2024
/// <https://fonctionpublique.gouv.cd/reamenagement-des-horaires-de-service-au-sein-de-ladministration-publique/>.
pub static CD: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2023),
    WeekendPolicy::of(SUNDAY).from(2024).until_day(2024, 7, 31),
    WeekendPolicy::of(SAT_SUN).from_day(2024, 8, 1),
];

/// Republic of the Congo: Saturday and Sunday, from
/// 2026, the year the sources were read in; no dated source for an earlier
/// weekend was found, so the years before are a gap. Wikipedia's "Workweek and
/// weekend", retrieved 2026-10-04 (secondary), is the page read.
pub static CG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Switzerland: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static CH: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Côte d'Ivoire: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static CI: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Chile: Saturday and Sunday from 16 March 2005, the day the DFL 29 of 2004
/// (promulgated on 16 June 2004), the consolidated text of Ley 18.834 on the
/// Estatuto Administrativo, was published: article 65 gives the ordinary
/// working time of officials as forty-four hours a week "distribuidas de lunes
/// a viernes". The article's margin note names article 59 of Ley 18.834, a law
/// of 1989 whose original wording was not read, so the years before 2005 are a
/// gap.
///
/// Sources, retrieved 2026-10-05: Ley Chile, DFL 29 of 2004 (texto refundido
/// del Estatuto Administrativo), article 65, published 16 March 2005
/// <https://www.leychile.cl/Consulta/obtxml?opt=7&idNorma=236392>.
pub static CL: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2005, 3, 15),
    WeekendPolicy::of(SAT_SUN).from_day(2005, 3, 16),
];

/// Cameroon: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static CM: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// China: Sunday alone, Saturday being a half working day, as the news report
/// of 25 March 1995 says of the system before, and Saturday and Sunday from 1
/// May 1995 for state departments and institutions, by the Ministry of
/// Personnel's measures effective that day under the State Council's regulation
/// of 25 March 1995 (a law firm's copy of the measures, not the gazette's).
///
/// Sources, retrieved 2026-10-05: UPI, “China proclaims five-day workweek”, 25
/// March 1995
/// <https://www.upi.com/Archives/1995/03/25/China-proclaims-five-day-workweek/8281796107600/>;
/// Lehman, Lee and Xu, translation of the State Council's regulations on
/// working hours of 1995
/// <https://www.lehmanlaw.com/resource-centre/laws-and-regulations/labor/state-council-regulations-on-working-hours-of-employees-1995.html>.
pub static CN: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(1995, 3, 24),
    WeekendPolicy::of(SUNDAY)
        .from_day(1995, 3, 25)
        .until_day(1995, 4, 30),
    WeekendPolicy::of(SAT_SUN).from_day(1995, 5, 1),
];

/// Colombia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static CO: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Costa Rica: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static CR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Cuba: the Labour Code, Ley 116 of 20 December 2013, rests the week on
/// Sunday: its weekly rest is a minimum of twenty-four consecutive hours, in
/// general on Sunday, the "descanso dominical" that the Code moves when the
/// work cannot stop. The policy begins with 2014, the first whole year after
/// the Code. The Code's page refused the connection on 2026-10-05, and the
/// article number is not re-verified: an extract of a search puts the rule in
/// article 93.
///
/// Sources, retrieved 2026-10-05: Gaceta Oficial de la República de Cuba, Ley
/// No. 116, “Código de Trabajo”
/// <https://www.gacetaoficial.gob.cu/es/ley-no-116-codigo-de-trabajo> (not
/// reachable on the day).
pub static CU: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2013),
    WeekendPolicy::of(SUNDAY).from(2014),
];

/// Cabo Verde: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static CV: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Cyprus: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static CY: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Czechia: Saturday and Sunday from 10 June 1968, when the decree 63/1968 Sb.
/// of the Czechoslovak Ministry of Labour and Social Affairs (of 15 May 1968)
/// took effect: where the nature of the work, the operation and the general
/// interest allow, workers are to have two consecutive days of uninterrupted
/// weekly rest, "pokud možno" on Sunday and Saturday, "dny všeobecného volna",
/// or on Sunday and Monday. It is a decree of principles for the five-day week,
/// which organisations introduced one by one, so the day is the decree's and
/// not the last organisation's. The decree was repealed on 1 January 2001; the
/// Labour Code of 2006 (262/2006 Sb., in force 1 January 2007) says only that
/// the weekly rest is to include Sunday, so Saturday as the second day is the
/// custom after 2000 and not a text read.
///
/// Sources, retrieved 2026-10-05: Vyhláška č. 63/1968 Sb. of the Czechoslovak
/// Ministry of Labour and Social Affairs (zakonyprolidi.cz)
/// <https://www.zakonyprolidi.cz/cs/1968-63>.
pub static CZ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(1968, 6, 9),
    WeekendPolicy::of(SAT_SUN).from_day(1968, 6, 10),
];

/// Germany: Saturday and Sunday for the federal civil service from 1 March
/// 2006, when the Arbeitszeitverordnung (AZV) of 23 February 2006 came into
/// force: section 3(2) spreads the regular weekly working time "auf Montag bis
/// Freitag", and lets it be spread over six days for service reasons. It is the
/// Bund's own officials, not the Länder's, and the rules before the AZV were
/// not read, so the years before are a gap.
///
/// Sources, retrieved 2026-10-05: Arbeitszeitverordnung of 23 February 2006, §
/// 3(2) (gesetze-im-internet.de)
/// <https://www.gesetze-im-internet.de/azv/BJNR042710006.html>.
pub static DE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2006, 2, 28),
    WeekendPolicy::of(SAT_SUN).from_day(2006, 3, 1),
];

/// Djibouti: Friday and Saturday from 1 January 2017, when, as the headline of
/// the Embassy of Djibouti's page says, the administration's hours ran "Sunday
/// to Thursday" (the body of the page says eight to five, "pendant cinq jours
/// par semaine à partir de janvier 2017"; the page itself is undated). Décret
/// 2025-165/PR/MTFPS of 1 July 2025, in force the day after its publication in
/// the edition of 15 July 2025, counts as working days for the civil servants'
/// leave all the days of the calendar "à l'exception du vendredi, du samedi et
/// des jours fériés" (article 4), and La Nation of 4 May 2026 reports the
/// hours, 8h to 15h, "du dimanche au jeudi" from 5 May 2026. The Labour Code
/// (loi 133/AN/05/5ème L, 28 January 2006), article 97, puts the weekly rest of
/// employees under contract "en principe le vendredi". Wikipedia's page, the
/// source of the reading of 2026 for the tables not dated here, gives Friday
/// alone for Djibouti; these dated sources contradict it, so Friday and
/// Saturday are carried, and the years before 2017 are a gap.
///
/// Sources, retrieved 2026-10-05: Embassy of Djibouti in Washington, headline
/// of an undated page on the hours from 1 January 2017
/// <https://djiboutiembassyus.org/article/starting-january-1-2017-working-hours-of-djibouti-administration-will-be-from-800-am-to-500>;
/// Djibouti, Décret n° 2025-165/PR/MTFPS, article 4 (Journal Officiel, edition
/// of 15 July 2025)
/// <https://www.journalofficiel.dj/texte-juridique/decret-n2025-165-pr-mtfps-fixant-le-regime-des-conges-et-absences-des-fonctionnaires/>;
/// La Nation (Djibouti), 4 May 2026
/// <https://www.lanation.dj/amenagement-des-horaires-de-travail-au-sein-de-ladministration-publique-une-reforme-pour-economiser-lenergie-et-optimiser-le-service-public/>;
/// Djibouti, Loi n° 133/AN/05/5ème L portant Code du Travail, article 97
/// <https://www.journalofficiel.dj/texte-juridique/loi-n133-an-05-5eme-l-portant-code-du-travail/>.
pub static DJ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2016),
    WeekendPolicy::of(FRI_SAT).from(2017),
];

/// Denmark: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static DK: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Dominica: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static DM: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Dominican Republic: Saturday and Sunday, from 2026,
/// the year the sources were read in; no dated source for an earlier weekend
/// was found, so the years before are a gap. Wikipedia's "Workweek and
/// weekend", retrieved 2026-10-04 (secondary), is the page read.
pub static DO: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Algeria: Saturday and Sunday until the ordinances of 1976 made it Thursday
/// and Friday (ordonnance no. 76-77 of 11 August 1976 on the weekly rest, known
/// by its title only, so the change is carried from 1 January 1976), and Friday
/// and Saturday from Friday 14 August 2009, by the Council of Ministers'
/// decision of 21 July 2009, which Jeune Afrique (AFP) of that day reports as
/// "au lieu de jeudi et vendredi". The date of the first regime is the year
/// before the change, a news report's own reference; no page read states
/// Saturday and Sunday for 1975, so that regime is the one that no source read
/// confirms.
///
/// Sources, retrieved 2026-10-05: Jeune Afrique (AFP), 21 July 2009
/// <https://www.jeuneafrique.com/depeches/109375/politique/lalgerie-decide-de-faire-du-vendredi-et-samedi-jours-de-conge-hebdomadaire-2/>.
pub static DZ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(1974),
    WeekendPolicy::of(SAT_SUN).from(1975).until(1975),
    WeekendPolicy::of(THU_FRI).from(1976).until_day(2009, 8, 13),
    WeekendPolicy::of(FRI_SAT).from_day(2009, 8, 14),
];

/// Ecuador: Saturday and Sunday from 6 October 2010, the date of the Registro
/// Oficial Suplemento 294 that published the Ley Orgánica de Servicio Público
/// (LOSEP): article 25(a) gives the ordinary working day as eight effective
/// hours "de lunes a viernes y durante los cinco días de cada semana", forty
/// hours a week. The text read is the consolidated one (reformed to 2020), so
/// the wording of 2010 itself is not verified.
///
/// Sources, retrieved 2026-10-05: Ecuador, Ley Orgánica de Servicio Público
/// (Registro Oficial Suplemento 294 of 6 October 2010), article 25(a)
/// <https://derechoecuador.com/ley-organica-de-servicio-publico-losep/>.
pub static EC: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2010, 10, 5),
    WeekendPolicy::of(SAT_SUN).from_day(2010, 10, 6),
];

/// Estonia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static EE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Egypt: Friday alone for government offices until 20 January 2006, and Friday
/// and Saturday from 21 January 2006, as an American Chamber of Commerce in
/// Egypt article of February 2006 and Wikipedia's page on Egypt's public
/// holidays have it (the article read only through a search extract).
///
/// Sources, retrieved 2026-10-05: AmCham Egypt, Business Monthly, February
/// 2006, “Gov't revises work week”
/// <https://www.amcham.org.eg/publications/business-monthly/issues/74/February-2006/597/govt-revises-work-week>
/// (read through a search extract only); Wikipedia, “Workweek and weekend”
/// <https://en.wikipedia.org/wiki/Workweek_and_weekend>.
pub static EG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2005),
    WeekendPolicy::of(FRIDAY).from(2006).until_day(2006, 1, 20),
    WeekendPolicy::of(FRI_SAT).from_day(2006, 1, 21),
];

/// Spain: Saturday and Sunday from 1 March 2019, when the Secretaría de Estado
/// de Función Pública's resolution of 28 February 2019 on the working day and
/// hours of the Administración General del Estado came into force: its item 3.2
/// fixes the attendance "de lunes a viernes". The BOE marks the resolution
/// repealed (the later instruction was not read) and says that it replaces the
/// resolution of 28 December 2012, which was not read, so the years before are
/// a gap.
///
/// Sources, retrieved 2026-10-05: BOE no. 52 of 1 March 2019, Resolución de 28
/// de febrero de 2019 (BOE-A-2019-2861)
/// <https://www.boe.es/buscar/act.php?id=BOE-A-2019-2861>.
pub static ES: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2019, 2, 28),
    WeekendPolicy::of(SAT_SUN).from_day(2019, 3, 1),
];

/// Ethiopia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static ET: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Finland: Saturday and Sunday from 1 April 1969, when the state offices
/// followed the five-day working week all the year round, in the summer months
/// only from 1966, which is not a weekend. The Finnish Wikipedia's article on
/// working time says so and adds that besides Sunday it is usually Saturday
/// that is free; its footnote cites the decree on working time in the state
/// offices (294/60, amended 301/66 and 161/68), which was not read. The source
/// is secondary.
///
/// Sources, retrieved 2026-10-05: Finnish Wikipedia, “Työaika”
/// <https://fi.wikipedia.org/wiki/Ty%C3%B6aika>.
pub static FI: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(1969, 3, 31),
    WeekendPolicy::of(SAT_SUN).from_day(1969, 4, 1),
];

/// Fiji: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static FJ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Micronesia: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static FM: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// France: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static FR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Gabon: Sunday: article 220 of the Code du travail, loi n° 022/2021 of 19
/// November 2021, "Le repos hebdomadaire est obligatoire ... Il a lieu en
/// principe le dimanche"; article 223 counts as working days "tous les jours
/// autres que le dimanche" and the holidays. The policy begins with 2022, the
/// first whole year after the law. Wikipedia's page on the workweek says Monday
/// to Friday; the Code is what is carried.
///
/// Sources, retrieved 2026-10-05: UNEP LEAP, record of Gabon's Loi n° 022/2021
/// du 19 novembre 2021 portant Code du Travail
/// <https://leap.unep.org/en/countries/ga/national-legislation/loi-ndeg0222021-du-19-novembre-2021-portant-code-du-travail>
/// (read through a search extract only).
pub static GA: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2021),
    WeekendPolicy::of(SUNDAY).from(2022),
];

/// United Kingdom: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static GB: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Grenada: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static GD: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Georgia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static GE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Ghana: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static GH: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// The Gambia: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static GM: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Guinea: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static GN: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Equatorial Guinea: Saturday and Sunday, from 2026,
/// the year the sources were read in; no dated source for an earlier weekend
/// was found, so the years before are a gap. Wikipedia's "Workweek and
/// weekend", retrieved 2026-10-04 (secondary), is the page read.
pub static GQ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Greece: Saturday and Sunday from 1 January 1981, as Law 1157/1981 ratifying
/// the act of legislative content of 29 December 1980 on the five-day working
/// week of the public services says in its article 1 ("Καθιερούται από 1
/// Ιανουαρίου 1981 πενθήμερος εβδομάς εργασίας, αρχομένη από Δευτέρας μέχρι και
/// Παρασκευής"), as a legal advisers' database excerpts it. The Gazette (ΦΕΚ Α'
/// 126 of 12 May 1981) is a PDF and was not opened.
///
/// Sources, retrieved 2026-10-05: Karagilanis S.A., Law 1157/1981, article 1
/// (the five-day week from 1 January 1981)
/// <https://karagilanis.gr/category/vasiki-nomothesia/nomos-1157-81/>.
pub static GR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(1980),
    WeekendPolicy::of(SAT_SUN).from(1981),
];

/// Guatemala: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static GT: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Guinea-Bissau: Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. The law read: Sunday: article 123(1) of the Lei
/// Geral do Trabalho, as the Portuguese Public Prosecutor's cooperation
/// department summarises it, "um dia de descanso por semana que, em princípio,
/// é ao domingo"; article 124 lets a half or whole day of complementary rest be
/// added, which is not the weekly rest and is not carried.
pub static GW: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SUNDAY).from(2026),
];

/// Guyana: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static GY: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Hong Kong: Saturday and Sunday for the civil service from 3 July 2006, when
/// the first phase of the five-day week began (the Government's press release
/// of 2 July 2006); the scheme was phased, and by 2012 about 70 per cent of
/// civil servants were on it, so a department that still works Saturdays is not
/// given here. Saturday is no statutory holiday; Sunday is.
///
/// Sources, retrieved 2026-10-05: Hong Kong Government, press release
/// “Government offices switch to five-day week”, 2 July 2006
/// <https://www.info.gov.hk/gia/general/200607/02/P200607010239.htm>.
pub static HK: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2006, 7, 2),
    WeekendPolicy::of(SAT_SUN).from_day(2006, 7, 3),
];

/// Honduras: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static HN: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Croatia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static HR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Haiti: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static HT: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Hungary: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static HU: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Indonesia: Saturday and Sunday from 1 October 1995, when Keputusan Presiden
/// 68/1995 took effect (its article 7): article 1 sets the working days of the
/// central government institutions and of the Jakarta administration as five,
/// "mulai hari Senin sampai dengan hari Jumat", and article 5 lets a minister
/// or head of an institution assign Saturday standby duty. The page's status is
/// repealed (by a later decree, not read), and the copy is unreviewed.
///
/// Sources, retrieved 2026-10-05: Indonesia, Keputusan Presiden No. 68 of 1995,
/// Pasal 1 and 7 (pasal.id)
/// <https://pasal.id/peraturan/keppres/keppres-no-68-tahun-1995>.
pub static ID: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(1995, 9, 30),
    WeekendPolicy::of(SAT_SUN).from_day(1995, 10, 1),
];

/// Ireland: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static IE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Israel: Israel's weekly day of rest: the Sabbath, and in law nothing else.
/// The Hours of Work and Rest Law, 5711-1951, section 7(b)(1), puts the Sabbath
/// in every Jewish employee's weekly rest, and the Law and Administration
/// Ordinance, section 18A, makes the Sabbath and the festivals the State's
/// prescribed days of rest. Friday is a working day: section 2(b) shortens the
/// day before the weekly rest to seven hours, and the Sunday-to-Thursday week
/// many employers keep is agreement and custom, not statute. Read from 1952,
/// the first whole year after the Law of 1951.
///
/// Sources, retrieved 2026-10-05: neto.work, “Hours of Work and Rest Law, 1951”
/// (the Law's own text is a PDF, not opened)
/// <https://www.neto.work/en/hours-of-work-and-rest-law-israel/>.
pub static IL: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(1951),
    WeekendPolicy::of(SATURDAY).from(1952),
];

/// India: Saturday and Sunday from 3 June 1985, when the central government's
/// administrative offices began a five-day week, "from Monday to Friday, with
/// all Saturdays as closed", by the Department of Personnel and Training's
/// Office Memorandum No. 13/4/85-JCA of 21 May 1985 (a reproduction on a news
/// site and a bank news site's account; the order itself was not read). The
/// memorandum fixes the hours of the offices in Delhi and New Delhi; the states
/// and other offices follow their own orders.
///
/// Sources, retrieved 2026-10-05: 7th Pay Commission News, reproduction of DoPT
/// Office Memorandum No. 13/4/85-JCA of 21 May 1985
/// <https://7thpaycommissionnews.in/five-days-a-week-working-in-administrative-offices-of-the-central-government/>;
/// Hellobanker, “Central Government Employees got 5 day work week in 1985”
/// <https://hellobanker.in/central-government-employees-got-5-day-work-week-in-1985/>.
pub static IN: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(1985, 6, 2),
    WeekendPolicy::of(SAT_SUN).from_day(1985, 6, 3),
];

/// Iraq: Friday and Saturday for government offices from the last week of
/// February 2005, when the Government added Saturday to Friday (Al Jazeera, 28
/// February 2005); the day is not given, so the policy begins in March. A blog
/// of March 2005 said the Government might revert to Thursday and Friday; no
/// reversal was found, and The National of 7 December 2021 says Iraq's weekend
/// is Friday and Saturday.
///
/// Sources, retrieved 2026-10-05: Al Jazeera, “Iraqis protest having Saturday
/// off”, 28 February 2005
/// <https://www.aljazeera.com/news/2005/2/28/iraqis-protest-having-saturday-off>;
/// The National, “Which countries have a Friday-Saturday weekend?”, 7 December
/// 2021
/// <https://www.thenationalnews.com/mena/2021/12/07/when-is-the-weekend-in-the-arab-world/>.
pub static IQ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2005, 2, 28),
    WeekendPolicy::of(FRI_SAT).from_day(2005, 3, 1),
];

/// Iran: Friday from 1991, the first whole year after the Labour Law's final
/// confirmation by the Expediency Council on 12 November 1990 (21 Aban 1369):
/// article 62 makes Friday the weekly paid day off for workers, and its note 1
/// lets public services and workplaces that cannot stop keep another day. It is
/// the labour law and not the civil service's: government offices keep Thursday
/// as a half day or a day off, which no instrument read states. Bills to make
/// Saturday (voted on 15 May 2024, rejected by the Guardian Council) or
/// Thursday (voted on 5 March 2025, pending the Guardian Council) the second
/// day are reported by sources that were not re-read here, and none is read in
/// force, so Friday stands for the years to 2026.
///
/// Sources, retrieved 2026-10-05: Iran Data Portal, Labor Law of Iran, article
/// 62 (translation) <https://irandataportal.syr.edu/labor-conditions/>.
pub static IR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(1990),
    WeekendPolicy::of(FRIDAY).from(1991),
];

/// Iceland: Saturday and Sunday: Law 88/1971 as amended by L. 25/1979 and L.
/// 94/1982, in force on 31 December 1982, sets 40 working hours a week at eight
/// hours a day from Monday to Friday (Althingi's consolidated text); the
/// Saturday and the Sunday are the days outside that week.
///
/// Sources, retrieved 2026-10-05: Althingi, Lög um 40 stunda vinnuviku nr.
/// 88/1971, consolidated text
/// <https://www.althingi.is/lagas/nuna/1971088.html>.
pub static IS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(1982),
    WeekendPolicy::of(SAT_SUN).from(1983),
];

/// Italy: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static IT: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Jamaica: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static JM: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Jordan: Thursday and Friday until the Government made it Friday and Saturday
/// from the week of 8 January 2000, as the Adventist Press Service reported on
/// 5 January 2000 ("a new law implementing the Friday/Saturday Weekend instead
/// of Thursdays and Fridays"); The National of 7 December 2021 lists Jordan
/// among the Friday-Saturday countries. The first regime is read from 2000, the
/// year of the report that gives it, which does not say how long it had lasted.
///
/// Sources, retrieved 2026-10-05: Adventist Press Service, “Jordan Announces
/// new Friday/Saturday Weekend”, 5 January 2000
/// <https://archive.wfn.org/2000/01/msg00078.html>; The National, “Which
/// countries have a Friday-Saturday weekend?”, 7 December 2021
/// <https://www.thenationalnews.com/mena/2021/12/07/when-is-the-weekend-in-the-arab-world/>.
pub static JO: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(1999),
    WeekendPolicy::of(THU_FRI).from(2000).until_day(2000, 1, 7),
    WeekendPolicy::of(FRI_SAT).from_day(2000, 1, 8),
];

/// Japan: Saturday and Sunday for national civil servants from 1 May 1992, when
/// the complete two-day weekend began, as the Japanese Wikipedia's article on
/// the 週休二日制 has it (secondary); the 一般職の職員の勤務時間、休暇等に関する法律 (平成6年法律第33号),
/// article 6(1), gives them Saturday and Sunday as 週休日. Before 1992 Saturday
/// was worked in part, and the weekend is not read.
///
/// Sources, retrieved 2026-10-05: Japanese Wikipedia, “週休二日制”
/// <https://ja.wikipedia.org/wiki/%E9%80%B1%E4%BC%91%E4%BA%8C%E6%97%A5%E5%88%B6>.
pub static JP: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(1992, 4, 30),
    WeekendPolicy::of(SAT_SUN).from_day(1992, 5, 1),
];

/// Kenya: Saturday and Sunday from 25 September 2023, the day of the Salaries
/// and Remuneration Commission's article, which says, citing section J(2) of
/// the Public Service Commission's Human Resource Policy Manual, that "public
/// officers are required to work 40 hours spread over 5 days in a week, from
/// Monday to Friday". The manual's own date is not given, so the policy begins
/// with the article's day.
///
/// Sources, retrieved 2026-10-05: Salaries and Remuneration Commission (Kenya),
/// 25 September 2023
/// <https://src.go.ke/2023/09/25/all-in-a-days-work-part-time-versus-full-time/>.
pub static KE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2023, 9, 24),
    WeekendPolicy::of(SAT_SUN).from_day(2023, 9, 25),
];

/// Kyrgyzstan: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static KG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Cambodia: Sunday, from 2026, the year the sources
/// were read in; no dated source for an earlier weekend was found, so the years
/// before are a gap. The law read: Article 147 of the Labour Law: weekly time
/// off "shall, in principle, be given on Sunday".
pub static KH: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SUNDAY).from(2026),
];

/// Kiribati: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static KI: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Comoros: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static KM: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Saint Kitts and Nevis: Saturday and Sunday, from
/// 2026, the year the sources were read in; no dated source for an earlier
/// weekend was found, so the years before are a gap. Wikipedia's "Workweek and
/// weekend", retrieved 2026-10-04 (secondary), is the page read.
pub static KN: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// North Korea: Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. The law read: Sunday: article 64 of the
/// Socialist Labour Law, "Sundays shall be days of rest".
pub static KP: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SUNDAY).from(2026),
];

/// South Korea: Saturday and Sunday from 1 July 2005. Presidential Decree 18438
/// of 24 June 2004 amended article 9 of the regulation on the service of
/// national civil servants, which gives the week's working time as forty hours,
/// "Saturday being a day off in principle" (토요일은 휴무함을 원칙으로 한다), from 1 July
/// 2004, and let Saturday leave be given only twice a month until 30 June 2005.
/// The weekend is therefore read from the end of that transition, and every
/// year before it is a gap. The regulation is read as Wikisource transcribes
/// it.
///
/// Sources, retrieved 2026-10-05: Korea, 국가공무원 복무규정, article 9 and the
/// supplementary provisions of 대통령령 제18438호 (Wikisource)
/// <https://ko.wikisource.org/wiki/%EA%B5%AD%EA%B0%80%EA%B3%B5%EB%AC%B4%EC%9B%90_%EB%B3%B5%EB%AC%B4%EA%B7%9C%EC%A0%95>.
pub static KR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2005, 6, 30),
    WeekendPolicy::of(SAT_SUN).from_day(2005, 7, 1),
];

/// Kuwait: Thursday and Friday until the Cabinet's decision of 27 May 2007
/// moved the Government's offices to Friday and Saturday from Saturday 1
/// September 2007, as Arab News reported it from KUNA; the first regime is read
/// from 1 January 2007.
///
/// Sources, retrieved 2026-10-05: Arab News, “Kuwait Adopts Friday-Saturday
/// Weekend”, 28 May 2007 <https://www.arabnews.com/node/298933>.
pub static KW: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2006),
    WeekendPolicy::of(THU_FRI).from(2007).until_day(2007, 8, 31),
    WeekendPolicy::of(FRI_SAT).from_day(2007, 9, 1),
];

/// Kazakhstan: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static KZ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Laos: Saturday and Sunday from 19 December 2017, the day Laopost reported
/// the Government's decree on holidays no. 386 of 15 December 2017, which says
/// that the weekly holiday is Saturday and Sunday for civil servants, soldiers,
/// police and workers. The decree's own text is a PDF, which was not opened,
/// and the day it took effect is not given, so the policy begins with the
/// report's day. The Labour Law of 2013 gives one rest day a week without
/// naming it in the pages read.
///
/// Sources, retrieved 2026-10-05: Laopost, 19 December 2017, on Decree No. 386
/// on holidays <https://laopost.com/archives/97867>.
pub static LA: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2017, 12, 18),
    WeekendPolicy::of(SAT_SUN).from_day(2017, 12, 19),
];

/// Lebanon: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static LB: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Saint Lucia: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static LC: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Liechtenstein: Saturday and Sunday from 1 January 2009, when the
/// Staatspersonalverordnung of 2 December 2008 (LGBl. 2008 Nr. 303) came into
/// force (article 112): article 38 excludes Saturdays, Sundays and holidays
/// from the normal working time, 6.00 to 20.00, of the state's staff. The text
/// read is the consolidated one, amended since, and the version of article 38
/// of 2009 itself was not checked.
///
/// Sources, retrieved 2026-10-05: Liechtenstein, Staatspersonalverordnung of 2
/// December 2008 (LGBl. 2008 Nr. 303), art. 38 and 112
/// <https://www.gesetze.li/konso/html/2008303000>.
pub static LI: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2008),
    WeekendPolicy::of(SAT_SUN).from(2009),
];

/// Sri Lanka: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static LK: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Liberia: The weekly rest of section 17.10 of the Decent Work Act, 2015: "at
/// least 36 consecutive hours which, unless otherwise agreed in writing, shall
/// include Sunday". Sunday is the one day the Act names. The policy begins with
/// 2016, the first whole year after the Act.
///
/// Sources, retrieved 2026-10-05: WageIndicator, “Annual Leave, Holiday Pay,
/// Weekly Rest Days - Liberia”
/// <https://wageindicator.org/en-lr/work-in-liberia/labour-law/annual-leave-and-holidays>.
pub static LR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2015),
    WeekendPolicy::of(SUNDAY).from(2016),
];

/// Lesotho: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static LS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Lithuania: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static LT: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Luxembourg: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static LU: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Latvia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static LV: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Libya: Friday and Saturday for the Government's offices from 2006: The
/// National of 7 December 2021 and Wikipedia's page on the workweek say that
/// Libya changed its weekend then; the Government's statement of 2 January 2006
/// gives no day of effect, so the policy begins with 2006. The Friday-only
/// weekend before it has no date in the sources read, and its years are not
/// read.
///
/// Sources, retrieved 2026-10-05: The National, “Which countries have a
/// Friday-Saturday weekend?”, 7 December 2021
/// <https://www.thenationalnews.com/mena/2021/12/07/when-is-the-weekend-in-the-arab-world/>;
/// Wikipedia, “Workweek and weekend”
/// <https://en.wikipedia.org/wiki/Workweek_and_weekend>.
pub static LY: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2005),
    WeekendPolicy::of(FRI_SAT).from(2006),
];

/// Morocco: Saturday and Sunday from 20 July 2005, the date of Décret n°
/// 2-05-916 (13 joumada II 1426), which, as La Vie éco of 12 October 2018
/// quotes it, fixes the working days of the State's and the local authorities'
/// establishments as Monday to Friday, 8:30 to 16:30. The day it took effect is
/// not given, so the policy begins with its date; the decree itself was not
/// read, and the years before are a gap.
///
/// Sources, retrieved 2026-10-05: La Vie éco, 12 October 2018, on Décret n°
/// 2-05-916 of 20 July 2005
/// <https://www.lavieeco.com/pouvoirs/continuite-du-service-dans-ladministration/>.
pub static MA: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2005, 7, 19),
    WeekendPolicy::of(SAT_SUN).from_day(2005, 7, 20),
];

/// Monaco: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static MC: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Moldova: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static MD: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Montenegro: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static ME: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Madagascar: Saturday and Sunday from 1 August 2009, when Décret n° 2009-969
/// of 14 July 2009 took effect ("Il prend effet à compter du 1er août 2009"):
/// its article 5 keeps the weekly working time of the public services at forty
/// hours, eight a day "du lundi au vendredi". The decree repeals that of 21 May
/// 2007 on the same matter, which was not read, so the years before are a gap.
///
/// Sources, retrieved 2026-10-05: Madagascar, Décret n° 2009-969 of 14 July
/// 2009, articles 1 and 5 (Lexxika)
/// <https://textes.lexxika.com/lois-malagasy/decret-n2009-969-du-14-juillet-2009-fixant-les-horaires-de-travail-effectif-dans-les-services-publics/>.
pub static MG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2009, 7, 31),
    WeekendPolicy::of(SAT_SUN).from_day(2009, 8, 1),
];

/// Marshall Islands: Saturday and Sunday, from 2026,
/// the year the sources were read in; no dated source for an earlier weekend
/// was found, so the years before are a gap. Wikipedia's "Workweek and
/// weekend", retrieved 2026-10-04 (secondary), is the page read.
pub static MH: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// North Macedonia: Saturday and Sunday, from 2026,
/// the year the sources were read in; no dated source for an earlier weekend
/// was found, so the years before are a gap. Wikipedia's "Workweek and
/// weekend", retrieved 2026-10-04 (secondary), is the page read.
pub static MK: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Mali: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static ML: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Myanmar: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static MM: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Mongolia: Saturday and Sunday from 1 July 1999, when the Labour Law of 14
/// May 1999 came into force (article 142.1), whose article 77.1 makes them the
/// days all rest ("Бямба, Ням гаригт нийтээр амарна"); the revised Labour Law
/// of 2 July 2021, in force from 1 January 2022, says the same in article 96.1.
/// It is the general labour law, and the civil service law, which could differ,
/// was not read.
///
/// Sources, retrieved 2026-10-05: Mongolia, Labour Law of 14 May 1999, art.
/// 77.1 and 142.1 (legalinfo.mn) <https://legalinfo.mn/mn/detail/565>;
/// Mongolia, Labour Law (revised) of 2 July 2021, art. 96.1 and 166.1
/// (legalinfo.mn) <https://legalinfo.mn/mn/detail?lawId=16230709635751>.
pub static MN: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(1999, 6, 30),
    WeekendPolicy::of(SAT_SUN).from_day(1999, 7, 1),
];

/// Macau: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static MO: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Mauritania: Saturday and Sunday from Wednesday 6 April 2005, the day of the
/// IRIN report that "Mauritania's two-day weekend has been shifted to Saturday
/// and Sunday ... following a ministerial vote on Wednesday", to Saturday 22
/// December 2007; Friday and Saturday from Sunday 23 December 2007, when the
/// weekly rest returned to Friday "pour cause religieuse" (cath.ch, dateline 26
/// December 2007: "Depuis avril 2005, ce repos hebdomadaire avait lieu les
/// samedi et dimanche"); and Saturday and Sunday from Wednesday 1 October 2014,
/// by the decree the Council of Ministers adopted on 11 September 2014 (Cridem,
/// Jeune Afrique): the week starts on Monday and ends on Friday. The
/// Friday-and-Saturday regime before April 2005, which cath.ch dates to the
/// early 1980s, has no day in a source read, and its years are a gap.
///
/// Sources, retrieved 2026-10-05: IRIN, “Working week changed in line with
/// economic partners”, 6 April 2005 (archive copy)
/// <https://web.archive.org/web/20240219084643/https://www.thenewhumanitarian.org/news/2005/04/06/working-week-changed-line-economic-partners>;
/// cath.ch (APIC), 26 December 2007
/// <https://cath.ch/newsf/mauritanie-retour-au-vendredi-jour-de-repos-hebdomadaire-pour-cause-religieuse/>;
/// Cridem, 15 September 2014, on the Council of Ministers' decree of 11
/// September 2014 <https://cridem.org/imprimable.php?article=660826>; Jeune
/// Afrique (AFP), 11 September 2014
/// <https://www.jeuneafrique.com/depeches/11961/politique/mauritanie-le-debut-du-week-end-repousse-du-vendredi-au-samedi/>.
pub static MR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2005, 4, 5),
    WeekendPolicy::of(SAT_SUN)
        .from_day(2005, 4, 6)
        .until_day(2007, 12, 22),
    WeekendPolicy::of(FRI_SAT)
        .from_day(2007, 12, 23)
        .until_day(2014, 9, 30),
    WeekendPolicy::of(SAT_SUN).from_day(2014, 10, 1),
];

/// Malta: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static MT: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Mauritius: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static MU: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Maldives: Friday and Saturday from 2013, the year Wikipedia's page on the
/// workweek gives for the Maldives' change, in a list item that carries no
/// reference of its own (the reference after the next item, Oman, is a Times of
/// Oman article); no source read dates a Maldivian decision to 2013. The
/// Employment Act's section 97 makes every Friday a public holiday (the Act is
/// a PDF, not opened), and the government works Sunday to Thursday: the
/// Ministry of Economic Development and Trade's gazette notice of 27 March 2024
/// gives its office open "except for Fridays and Saturdays", and trade portals
/// say the same. The regime before 2013 is not read.
///
/// Sources, retrieved 2026-10-05: Ministry of Economic Development and Trade of
/// the Maldives, notice of 27 March 2024 (gazette.gov.mv)
/// <https://www.gazette.gov.mv/iulaan/284434>; Lloyds Bank International Trade
/// Portal, “Opening hours and bank holidays in the Maldives” (August 2026)
/// <https://www.lloydsbanktrade.com/en/market-potential/maldives/opening-hours>;
/// Wikipedia, “Workweek and weekend”
/// <https://en.wikipedia.org/wiki/Workweek_and_weekend>.
pub static MV: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2012),
    WeekendPolicy::of(FRI_SAT).from(2013),
];

/// Malawi: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static MW: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Mexico: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static MX: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Mozambique: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static MZ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Namibia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static NA: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Niger: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static NE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Nigeria: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static NG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Nicaragua: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static NI: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Netherlands: Saturday and Sunday: the Algemene termijnenwet (Act of 25 July
/// 1964, in force 1 April 1965) treats Saturday, Sunday and the recognised
/// holidays alike for a time limit. The free Saturday was approved nationally
/// on 23 December 1960 and phased in sector by sector, the government's own
/// date not found, so the policy begins with the Act.
///
/// Sources, retrieved 2026-10-05: Algemene termijnenwet (Overheid.nl)
/// <https://wetten.overheid.nl/BWBR0002448>; Dutch Wikipedia, “Vrije zaterdag”
/// <https://nl.wikipedia.org/wiki/Vrije_zaterdag>.
pub static NL: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(1965, 3, 31),
    WeekendPolicy::of(SAT_SUN).from_day(1965, 4, 1),
];

/// Norway: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static NO: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Nepal kept a one-day weekend — Saturday alone — until the government
/// extended it to Saturday and Sunday in April 2026. It is the reason the
/// engine takes weekend days as data rather than assuming Saturday and Sunday,
/// and the reason a weekend rule carries years like everything else. The
/// cabinet decided on 5 April 2026 (चैत्र 22, 2082 BS) to close government
/// offices and every educational institution on Sundays as well as Saturdays,
/// "effective from Chaitra 23, 2082", which is Monday 6 April 2026, as a saving
/// on fuel during the disruption of petroleum supply. The decision is quoted by
/// New Spotlight, 5 April 2026
/// (<https://www.spotlightnepal.com/2026/04/05/nepal-government-decides-grant-two-day-holiday-saturday-and-sunday/>), and OnlineKhabar English reported it
/// the same day from the government spokesperson as starting the next day. The
/// first Sunday off was 12 April 2026. So the Saturday-only weekend runs to
/// Sunday 5 April 2026, a working day, and Saturday and Sunday from Monday 6
/// April.
///
/// The same was tried once before. The cabinet of Tuesday 26 April 2022 closed
/// government offices on Saturdays and Sundays from 15 May 2022, "Until now,
/// Nepal closes its offices on Saturdays only" (OnlineKhabar English and the
/// Kathmandu Post, both 27 April 2022), and the cabinet of Monday 6 June 2022
/// revoked the Sunday "effective from June 15" (the Kathmandu Post and the
/// Himalayan Times, 6 June 2022), Saturday staying the weekly holiday. So the
/// Saturday-only weekend is read from 2022, the year of the report that states
/// it; Saturday and Sunday run from 15 May to 14 June 2022; Saturday alone
/// again from 15 June 2022 to 5 April 2026. The Rana-era origin of the Saturday
/// holiday is told without a date in the sources read, and every year before
/// 2022 is a gap.
///
/// Sources, retrieved 2026-10-05: OnlineKhabar English, 27 April 2022
/// <https://english.onlinekhabar.com/two-day-weekend-nepal.html>; The Kathmandu
/// Post, 27 April 2022
/// <https://kathmandupost.com/national/2022/04/27/government-decides-two-days-public-holiday-in-a-week>;
/// The Kathmandu Post, 6 June 2022
/// <https://kathmandupost.com/national/2022/06/06/government-rolls-back-sunday-holiday-rule>;
/// The Himalayan Times, 6 June 2022
/// <https://thehimalayantimes.com/nepal/govt-revokes-two-day-weekend-decision-effective-june-15>;
/// New Spotlight, 5 April 2026
/// <https://www.spotlightnepal.com/2026/04/05/nepal-government-decides-grant-two-day-holiday-saturday-and-sunday/>;
/// OnlineKhabar English, 5 April 2026
/// <https://english.onlinekhabar.com/saturday-sunday-holiday-introduced-for-government-offices-and-educational-institutions.html>.
pub static NP: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2021),
    WeekendPolicy::of(SATURDAY)
        .from(2022)
        .until_day(2022, 5, 14),
    WeekendPolicy::of(SAT_SUN)
        .from_day(2022, 5, 15)
        .until_day(2022, 6, 14),
    WeekendPolicy::of(SATURDAY)
        .from_day(2022, 6, 15)
        .until_day(2026, 4, 5),
    WeekendPolicy::of(SAT_SUN).from_day(2026, 4, 6),
];

/// Nauru: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static NR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// New Zealand: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static NZ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Oman: Thursday and Friday until 30 April 2013, Friday and Saturday from 1
/// May 2013, for the public and private sectors alike, as Al Riyadh (Reuters, 7
/// April 2013), The National (7 April 2013) and Gulf News reported the Council
/// of Ministers' statement of April 2013, which they attribute to a royal
/// decree of Sultan Qaboos; the decree's number and text were not read. The
/// Gulf News page shows the date 2018, the site's re-dating of an article of
/// 2013. The first regime is read from 2013, the year of the report.
///
/// Sources, retrieved 2026-10-05: Al Riyadh (Reuters), 7 April 2013
/// <https://www.alriyadh.com/824049>; The National, 7 April 2013
/// <https://www.thenationalnews.com/world/mena/oman-to-align-weekend-days-with-uae-qatar-1.404841>;
/// Gulf News, “Oman to follow uniform Friday-Saturday weekend” (April 2013)
/// <https://gulfnews.com/world/gulf/oman/oman-to-follow-uniform-friday-saturday-weekend-1.1167447>.
pub static OM: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2012),
    WeekendPolicy::of(THU_FRI).from(2013).until_day(2013, 4, 30),
    WeekendPolicy::of(FRI_SAT).from_day(2013, 5, 1),
];

/// Panama: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static PA: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Peru: Saturday and Sunday from 4 January 1996, the day after the publication
/// of Decreto Legislativo 800 (given on 30 December 1995, published on 3
/// January 1996), whose article 2 makes the public administration's continuous
/// day of seven hours forty-five minutes apply "de lunes a viernes", and whose
/// article 4 puts it in force the day after publication. The text is a law
/// bank's copy.
///
/// Sources, retrieved 2026-10-05: Peru, Decreto Legislativo 800 of 30 December
/// 1995, art. 2 and 4 (InfoPublic's copy)
/// <https://infopublic.bpaprocorp.com/banco-de-leyes/decreto-legislativo-800>.
pub static PE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(1996, 1, 3),
    WeekendPolicy::of(SAT_SUN).from_day(1996, 1, 4),
];

/// Papua New Guinea: Saturday and Sunday, from 2026,
/// the year the sources were read in; no dated source for an earlier weekend
/// was found, so the years before are a gap. Wikipedia's "Workweek and
/// weekend", retrieved 2026-10-04 (secondary), is the page read.
pub static PG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Philippines: Saturday and Sunday from 27 December 1991, the date of Civil
/// Service Commission Resolution 91-1631 adopting the Omnibus Rules
/// Implementing Book V of Executive Order 292, whose Rule XVII, section 5 gives
/// the hours of the five-day week "on all days except Saturdays, Sundays and
/// Holidays". CSC Memorandum Circular 21 of 24 June 1991 had already required
/// eight hours for "five working days a week" without naming the days. The
/// Omnibus Rules' own day of effect was not read, so the date of the resolution
/// is taken.
///
/// Sources, retrieved 2026-10-05: Philippines, Omnibus Rules Implementing Book
/// V of Executive Order 292, Rule XVII, section 5 (eCodal), adopted by CSC
/// Resolution 91-1631 of 27 December 1991
/// <https://sites.google.com/view/e-codal/political/administrative-code/book-v/omnibus-rules>;
/// Philippines, CSC Memorandum Circular No. 21, s. 1991, of 24 June 1991
/// (Supreme Court E-Library)
/// <https://elibrary.judiciary.gov.ph/thebookshelf/showdocs/11/49201>.
pub static PH: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(1991, 12, 26),
    WeekendPolicy::of(SAT_SUN).from_day(1991, 12, 27),
];

/// Pakistan: Saturday and Sunday for the federal government from 9 June 2022,
/// as news reports of the notification say (Geo News, 8 June 2022: "restore
/// Saturday as a weekly holiday with immediate effect", a notification
/// confirmed on Wednesday). Earlier regimes the reports give (Sunday from 1947,
/// Friday from 1977, Sunday from 1997, Saturday and Sunday from February 2019
/// and Sunday alone from 16 April to 8 June 2022) disagree among themselves and
/// are not carried. Radio Pakistan reported on 11 March 2026 that the Cabinet
/// Division notified a four-day week, "offices will operate from Monday to
/// Thursday", effective immediately; The Nation of 18 September 2026 calls a
/// circulating notice of a four-day week fake, so the end of the four-day week
/// is not read and the policy is left on Saturday and Sunday.
///
/// Sources, retrieved 2026-10-05: Geo News, “Federal govt restores Saturday as
/// weekly holiday”, 8 June 2022
/// <https://www.geo.tv/latest/421401-federal-govt-restores-saturday-as-weekly-holiday>;
/// Radio Pakistan, 11 March 2026
/// <https://www.radio.gov.pk/11-03-2026/govt-notifies-four-day-workweek-for-offices-effective-immediately>.
pub static PK: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2022, 6, 8),
    WeekendPolicy::of(SAT_SUN).from_day(2022, 6, 9),
];

/// Poland: Sunday alone from the Act of 18 January 1951 on days free from work
/// until 1972 (Polish Wikipedia's article on the days free from work); from
/// 1973 every Saturday was free in turn and by the agreement of 31 January 1981
/// three a month, so the weekend of 1973 to 2000 is not read; from 2001 the
/// working week is five days and the employer sets the free day besides Sunday,
/// in practice Saturday, which Polish Wikipedia notes.
///
/// Sources, retrieved 2026-10-05: Act of 18 January 1951 on days free from work
/// (INFOR's copy)
/// <https://www.infor.pl/akt-prawny/47317,ustawa-o-dniach-wolnych-od-pracy.html>;
/// Polish Wikipedia, “Dni wolne od pracy w Polsce”
/// <https://pl.wikipedia.org/wiki/Dni_wolne_od_pracy_w_Polsce>; Polish
/// Wikipedia, “Wolna sobota (PRL)”
/// <https://pl.wikipedia.org/wiki/Wolna_sobota_(PRL)>.
pub static PL: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(1950),
    WeekendPolicy::of(SUNDAY).from(1951).until(1972),
    WeekendPolicy::unread().from(1973).until(2000),
    WeekendPolicy::of(SAT_SUN).from(2001),
];

/// Palestine: Thursday and Friday until 30 June 2007 and Friday and Saturday
/// from 1 July 2007 for the public sector of the Palestinian Authority's
/// government, as the United Nations' chronological review of July 2007 reports
/// from AP under 1 July: the Authority "is switching the weekend for the public
/// sector from Thursday-Friday to Friday-Saturday", and in the Gaza Strip some
/// employees were told to keep the old weekend. The Gaza side's weekend stayed
/// Thursday and Friday, which is not carried as a region. The first regime is
/// read from 2007, the year of the report. The General Personnel Council's
/// guide for the public employee, undated, still says six working days and the
/// holiday on Friday "except as special laws or decisions provide", so it is
/// not carried as a regime.
///
/// Sources, retrieved 2026-10-05: United Nations, Chronological review of
/// events, July 2007, entry of 1 July
/// <https://www.un.org/unispal/document/auto-insert-209221/>; Palestine,
/// General Personnel Council, Guide of the public employee (undated)
/// <https://www.gpc.pna.ps/diwan/arabic/userGiude/d2Part6Chapter1.jsp>.
pub static PS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2006),
    WeekendPolicy::of(THU_FRI).from(2007).until_day(2007, 6, 30),
    WeekendPolicy::of(FRI_SAT).from_day(2007, 7, 1),
];

/// Portugal: Saturday and Sunday from 1 August 2014, when the Lei Geral do
/// Trabalho em Funções Públicas (Lei n.º 35/2014 of 20 June 2014) came into
/// force (article 44: the first day of the second month after publication):
/// article 124 makes the working week in rule five days and the compulsory and
/// the complementary weekly rest "coincidir com o domingo e o sábado,
/// respetivamente". The laws before it were not read, so the years before are a
/// gap.
///
/// Sources, retrieved 2026-10-05: Portugal, Lei n.º 35/2014 of 20 June 2014
/// (LTFP), articles 44 and 124 (PGDL's copy)
/// <https://pgdlisboa.pt/leis/lei_mostra_articulado.php?nid=2171&tabela=leis>.
pub static PT: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2014, 7, 31),
    WeekendPolicy::of(SAT_SUN).from_day(2014, 8, 1),
];

/// Palau: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static PW: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Paraguay: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static PY: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Qatar: Thursday and Friday until the Cabinet's decision announced on 20 July
/// 2003 moved the public sector to Friday and Saturday from 1 August 2003, as
/// Arab News reported it; the first regime is read from 2003.
///
/// Sources, retrieved 2026-10-05: Arab News, “Qatar Changes Weekend”, 21 July
/// 2003 <https://www.arabnews.com/node/234601>.
pub static QA: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2002),
    WeekendPolicy::of(THU_FRI).from(2003).until_day(2003, 7, 31),
    WeekendPolicy::of(FRI_SAT).from_day(2003, 8, 1),
];

/// Romania: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static RO: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Serbia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static RS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Russia: Sunday alone from the return to the seven-day week in 1940 (the
/// Russian Wikipedia's article on the workweek) until the decree of 7 March
/// 1967 of the Party's Central Committee, the Council of Ministers and the
/// trade unions introduced the five-day week, which was put in force stage by
/// stage in 1967: that year is not read, and Saturday and Sunday are carried
/// from 1968. Under the Labour Code article 111 the common day off is Sunday
/// and the second is set by the employer; Saturday is the custom.
///
/// Sources, retrieved 2026-10-05: Russian Wikipedia, «Рабочая неделя»
/// <https://ru.wikipedia.org/wiki/%D0%A0%D0%B0%D0%B1%D0%BE%D1%87%D0%B0%D1%8F_%D0%BD%D0%B5%D0%B4%D0%B5%D0%BB%D1%8F>;
/// Labour Code of the Russian Federation, article 111 (ConsultantPlus's copy)
/// <https://www.consultant.ru/document/cons_doc_LAW_34683/57d0380478e89e33fc2a3d64af96b7d7793be7c6/>.
pub static RU: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(1939),
    WeekendPolicy::of(SUNDAY).from(1940).until(1966),
    WeekendPolicy::unread().from(1967).until(1967),
    WeekendPolicy::of(SAT_SUN).from(1968),
];

/// Rwanda: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static RW: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Saudi Arabia: Saudi Arabia moved the Government's weekend from Thursday and
/// Friday to Friday and Saturday by a royal order of 23 June 2013, "starting
/// from Saturday, 20/08/1434 corresponding to 29/6/2013", as the Saudi Press
/// Agency published it; Friday 28 June was a weekend day under both. The
/// Ministry of Labour said at the time that the private sector's statutory
/// weekly rest is Friday alone, which is not carried. The first regime is read
/// from 2013.
///
/// Sources, retrieved 2026-10-05: Saudi Press Agency, “Royal Order: Weekly
/// Holiday on Friday and Saturday”, 23 June 2013
/// <https://www.spa.gov.sa/1122964>.
pub static SA: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2012),
    WeekendPolicy::of(THU_FRI).from(2013).until_day(2013, 6, 28),
    WeekendPolicy::of(FRI_SAT).from_day(2013, 6, 29),
];

/// Solomon Islands: Saturday and Sunday, from 2026,
/// the year the sources were read in; no dated source for an earlier weekend
/// was found, so the years before are a gap. Wikipedia's "Workweek and
/// weekend", retrieved 2026-10-04 (secondary), is the page read.
pub static SB: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Seychelles: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static SC: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Sudan: Friday until the Council of Ministers added Saturday from 26 January
/// 2008, as the Sudan Tribune reported on 6 January 2008, for "six months for
/// studying and assessment"; the Government's notices of 2026 still put work
/// back on the Sunday after a Saturday (Sudan Horizon, 19 May 2026). The Friday
/// regime is read from 21 January 1998, the date of a law firm's table of
/// official working hours (Saturday to Wednesday 8:00 to 4:30, Thursday 8:00 to
/// 1:00, Friday off), the first date a source read gives for it; the table does
/// not say that it is the public sector's.
///
/// Sources, retrieved 2026-10-05: Sudan Tribune, 6 January 2008 (archive copy)
/// <https://web.archive.org/web/20100215043118id_/http://sudantribune.com/spip.php?article25469>;
/// Mondaq, “Official Holidays and Working Hours in the Sudan”, 21 January 1998
/// (a law firm's table of hours)
/// <https://www.mondaq.com/intellectual-property/3630/official-holidays-and-working-hours-in-the-sudan>;
/// Sudan Horizon, 19 May 2026
/// <https://sudanhorizon.com/sudanese-cabinet-sets-eid-al-adha-holiday-dates/>.
pub static SD: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(1998, 1, 20),
    WeekendPolicy::of(FRIDAY)
        .from_day(1998, 1, 21)
        .until_day(2008, 1, 25),
    WeekendPolicy::of(FRI_SAT).from_day(2008, 1, 26),
];

/// Sweden: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static SE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Singapore: Saturday and Sunday from 1 September 2004, when the Civil Service
/// went to a five-day week: the Ministry of Foreign Affairs' press statement of
/// 31 August 2004, "in line with Government's policy to introduce a five-day
/// workweek in the Civil Service", gives its staff hours Monday to Friday with
/// effect from 1 September 2004, while the consular counter keeps its Saturday
/// hours. A department that serves the public may keep a Saturday counter,
/// which is not a weekend day of its staff. The earlier half-day Saturday of
/// the civil service is not dated in the sources read.
///
/// Sources, retrieved 2026-10-05: Ministry of Foreign Affairs of Singapore,
/// press statement of 31 August 2004
/// <https://www.mfa.gov.sg/Newsroom/Press-Statements-Transcripts-and-Photos/2004/08/MFA-Press-Statement---Introduction-of-Five-Day-Workweek>.
pub static SG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2004, 8, 31),
    WeekendPolicy::of(SAT_SUN).from_day(2004, 9, 1),
];

/// Slovenia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static SI: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Slovakia: Saturday and Sunday from 10 June 1968, by the same decree 63/1968
/// Sb. of the Czechoslovak Ministry of Labour and Social Affairs, which applied
/// in the Czechoslovak Socialist Republic and so in Slovakia (see Czechia: the
/// days are those of "general leisure" where the work allows, or Sunday and
/// Monday). The Slovak Labour Code (311/2001 Z. z.) was not read: slov-lex.sk
/// serves its text only to a script, so Saturday as the second day after 1992
/// is the custom and not a text read.
///
/// Sources, retrieved 2026-10-05: Vyhláška č. 63/1968 Sb. of the Czechoslovak
/// Ministry of Labour and Social Affairs (zakonyprolidi.cz)
/// <https://www.zakonyprolidi.cz/cs/1968-63>.
pub static SK: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(1968, 6, 9),
    WeekendPolicy::of(SAT_SUN).from_day(1968, 6, 10),
];

/// Sierra Leone: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static SL: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// San Marino: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static SM: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Senegal: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static SN: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Somalia: Friday: article 64(1) of the Labour Code of 2024 (Law 36 of 2024),
/// one day's rest a week, which "should normally fall on Friday" ("maalinta
/// Jimcaha"); a 2025 news article says Friday is the designated weekly day off.
/// The policy begins with 2025, the year the Code is read from. Neither the
/// Code's text nor the article of 2025 could be found again on 2026-10-05: a
/// search extract gives the same rule as article 96 of the earlier Code (Law 65
/// of 1972), so this entry is not re-verified.
pub static SO: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2024),
    WeekendPolicy::of(FRIDAY).from(2025),
];

/// Suriname: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static SR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// South Sudan: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static SS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// El Salvador: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static SV: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Syria: Friday and Saturday from 1 February 2004, when the Council of
/// Ministers made the State's offices keep a Sunday-to-Thursday week, as
/// Jordan's Ad-Dustour reported on Thursday 25 December 2003 ("Syria announced
/// that it decided to make Saturday an official holiday in addition to Friday's
/// from the beginning of February"). The instrument was not found. A law firm's
/// table of official working hours on Mondaq (Saturday to Wednesday 8:30 to
/// 16:30, Thursday 8:30 to 13:30, so Friday off) is dated 22 January 1998 on
/// the page and titled "Year 2000", and does not say that it is the public
/// sector's, so the Friday-only weekend before 2004 is not carried.
///
/// Sources, retrieved 2026-10-05: Ad-Dustour (Amman), 25 December 2003
/// <https://www.addustour.com/articles/379374-%D8%A7%D9%84%D8%B3%D8%A8%D8%AA-%D8%B9%D8%B7%D9%84%D8%A9-%D8%B1%D8%B3%D9%85%D9%8A%D8%A9-%D9%81%D9%8A-%D8%B3%D9%88%D8%B1%D9%8A%D8%A7-%D9%85%D9%86-%D8%A8%D8%AF%D8%A7%D9%8A%D8%A9-%D8%B4%D8%A8%D8%A7%D8%B7>;
/// Mondaq, “Official Holidays and Working Hours in Syria”, 22 January 1998 (a
/// law firm's table of hours)
/// <https://www.mondaq.com/Information-Technology-and-Telecoms/3631/Official-Holidays-and-Working-Hours-in-Syria>.
pub static SY: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2004, 1, 31),
    WeekendPolicy::of(FRI_SAT).from_day(2004, 2, 1),
];

/// Eswatini: Sunday, from 2026, the year the sources
/// were read in; no dated source for an earlier weekend was found, so the years
/// before are a gap. The law read: Sunday. Section 2 of the Act moves a Sunday
/// holiday, and the Ministry of Home Affairs said in September 2025 that
/// Somhlolo Day on a Saturday "would not be shifted ... since Saturday is a
/// normal working day".
pub static SZ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SUNDAY).from(2026),
];

/// Chad: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static TD: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Togo: Sunday: the Code du travail of 2021 (loi n° 2021-012 of 18 June 2021),
/// article 198, a weekly rest of at least twenty-four consecutive hours which,
/// as LQDD's page puts it, "en général ... tombe le dimanche", some trades
/// moving it to another day. The Code's text is a PDF and was not opened, so
/// the words of the article itself are not quoted; the Code does not govern
/// permanent civil servants. The policy begins with 2022, the first whole year
/// after the Code.
///
/// Sources, retrieved 2026-10-05: LQDD, “Droit du travail Togo” (2026), on the
/// Code du travail of 18 June 2021, article 198
/// <https://www.lqdd.org/droit-du-travail-togo-droits-salarie/>.
pub static TG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2021),
    WeekendPolicy::of(SUNDAY).from(2022),
];

/// Thailand: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static TH: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Tajikistan: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static TJ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Timor-Leste: Article 30 of the Labour Code, Law No. 4/2012: the weekly rest
/// day "só pode deixar de ser ao domingo" for work that cannot stop. The policy
/// begins with 2013, the first whole year after the Code.
///
/// Sources, retrieved 2026-10-05: Jornal da República, Lei n.º 4/2012 (Lei do
/// Trabalho), article 30 <https://www.mj.gov.tl/jornal/?q=node/789>.
pub static TL: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2012),
    WeekendPolicy::of(SUNDAY).from(2013),
];

/// Turkmenistan: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static TM: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Tunisia: Saturday and Sunday from 17 September 2012, when décret 2012-1710
/// of 14 September 2012 took effect (article 9): the working hours and days of
/// the State's, the local authorities' and the administrative public bodies'
/// staff are spread "du lundi au vendredi", forty hours a week in winter, with
/// Saturday standby only for the services that give direct service to the
/// public (article 6). The decree names no regime that it replaces, so the
/// years before are a gap.
///
/// Sources, retrieved 2026-10-05: Tunisia, Décret n° 2012-1710 of 14 September
/// 2012, articles 2 and 9 (legislation-securite.tn)
/// <https://legislation-securite.tn/latest-laws/decret-n-2012-1710-du-14-septembre-2012-relatif-a-la-repartition-des-horaires-et-jours-de-travail-des-agents-de-letat-des-collectivites-locales-et-des-etablissements-publics-a-caracte/>.
pub static TN: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2012, 9, 16),
    WeekendPolicy::of(SAT_SUN).from_day(2012, 9, 17),
];

/// Tonga: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static TO: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Türkiye: Friday for the public service from 2 January 1924, Sunday from 2
/// June 1935 (a half-day Saturday from 13:00) and Saturday and Sunday from 1
/// July 1974, as the TDV Islam Ansiklopedisi's article on the weekly holiday
/// gives the dates (secondary); the Labour Law's private-sector rest is Sunday
/// alone.
///
/// Sources, retrieved 2026-10-05: TDV İslam Ansiklopedisi, “Hafta tatili”
/// <https://islamansiklopedisi.org.tr/hafta-tatili>.
pub static TR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(1924, 1, 1),
    WeekendPolicy::of(FRIDAY)
        .from_day(1924, 1, 2)
        .until_day(1935, 6, 1),
    WeekendPolicy::of(SUNDAY)
        .from_day(1935, 6, 2)
        .until_day(1974, 6, 30),
    WeekendPolicy::of(SAT_SUN).from_day(1974, 7, 1),
];

/// Trinidad and Tobago: Saturday and Sunday, from
/// 2026, the year the sources were read in; no dated source for an earlier
/// weekend was found, so the years before are a gap. Wikipedia's "Workweek and
/// weekend", retrieved 2026-10-04 (secondary), is the page read.
pub static TT: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Tuvalu: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static TV: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Taiwan: Saturday and Sunday from 1 January 2001, when the five-day week
/// reached public offices; Wikipedia says every second Saturday was free from
/// 1998 to 2000, years in which the weekend is not read.
///
/// Sources, retrieved 2026-10-05: Chinese Wikipedia, “週休二日”
/// <https://zh.wikipedia.org/wiki/%E9%80%B1%E4%BC%91%E4%BA%8C%E6%97%A5>; Taiwan
/// Panorama, April 1998, “Weekend Shuffle”
/// <https://www.taiwan-panorama.com/en/Articles/Details?Guid=7d511640-22ec-4937-9cce-8ae08dc5f510&CatId=11>.
pub static TW: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(1997),
    WeekendPolicy::unread().from(1998).until(2000),
    WeekendPolicy::of(SAT_SUN).from(2001),
];

/// Tanzania: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static TZ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Ukraine: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static UA: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Uganda: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static UG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// United States: Saturday and Sunday for the executive agencies: 5 U.S.C. §
/// 6101 as enacted by Pub. L. 89-554 of 6 September 1966 schedules the basic
/// 40-hour workweek on 5 days, "Monday through Friday when possible", the 2
/// days outside it being consecutive. The statute says "when possible", so a
/// Saturday or Sunday worked by an agency is the exception it allows. The
/// Government's earlier workweeks, a Saturday half day among them, are not
/// read.
///
/// Sources, retrieved 2026-10-05: 5 U.S.C. § 6101 (Cornell LII's copy)
/// <https://www.law.cornell.edu/uscode/text/5/6101>.
pub static US: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(1966, 9, 5),
    WeekendPolicy::of(SAT_SUN).from_day(1966, 9, 6),
];

/// Uruguay: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static UY: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Uzbekistan: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static UZ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Vatican City: Sunday alone: the Governorate's Regulation of 21 November
/// 2010, in force from 1 January 2011, article 26 §1: "I dipendenti hanno
/// diritto ad un giorno di riposo settimanale, che coincide normalmente con la
/// domenica" (the day of weekly rest, which normally coincides with Sunday);
/// the policy begins with 2011, the table's first year.
///
/// Sources, retrieved 2026-10-05: Governorate of Vatican City State,
/// Regolamento generale per il personale of 21 November 2010, art. 26
/// <https://www.vatican.va/roman_curia/labour_office/docs/documents/ulsa_b18_7_it.html>.
pub static VA: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2010),
    WeekendPolicy::of(SUNDAY).from(2011),
];

/// Saint Vincent and the Grenadines: Saturday and Sunday, as the table carried
/// it, from 2026, the year the sources were read in; no dated source for an
/// earlier weekend was found, so the years before are a gap. Wikipedia's
/// "Workweek and weekend", retrieved 2026-10-04 (secondary), is the page read.
pub static VC: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Venezuela: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static VE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Vietnam: Saturday and Sunday from 1 January 2021, the first year of the
/// notices of the civil service's days off, which give days off "in lieu of the
/// Saturday and Sunday" and swap a weekday holiday for a Saturday (the Ministry
/// of Labour's notice 4875/TB-LĐTBXH of 10 December 2020: Tết 10 to 14 February
/// 2021, and 15 and 16 February in lieu of the Saturday and Sunday among them);
/// the Labour Code of 2019, in force from 1 January 2021, gives the employer's
/// weekly rest day as Sunday or another day, and a travel agency's guide of 13
/// March 2024 says that "Government offices operate from Monday to Friday". A
/// travel-agency page that was archived on 3 May 2017 and said that government
/// agencies are closed on Saturday and Sunday could not be identified again,
/// and is not carried. The years before 2021 are a gap.
///
/// Sources, retrieved 2026-10-05: the Ministry of Labour's, the Ministry of
/// Home Affairs' and the Government Office's notices of the civil service's
/// days off, 2021 to 2026 (4875/TB-LĐTBXH of 10 December 2020 and the others)
/// (docs/references.bib); Vietnam, Bộ luật Lao động, Luật số 45/2019/QH14,
/// articles 111 and 112 (an archive copy of 2024)
/// <https://thuvienphapluat.vn/van-ban/Lao-dong-Tien-luong/Bo-Luat-lao-dong-2019-333670.aspx>;
/// Eco Travel Vietnam, “Business hours” (13 March 2024)
/// <https://ecotravelvietnam.com/travel-guide/business-hours>.
pub static VN: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// Vanuatu: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static VU: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Samoa: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static WS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Yemen: Thursday and Friday until Council of Ministers resolution 179 of 2013
/// made Saturday the second day in Thursday's place: from 15 August 2013, as
/// Yemen Post reported it the next day. Al Khaleej reported the change as
/// applied from Saturday 17 August; the two readings differ only on Thursday 15
/// August, which is carried as a working day. An earlier decree, reported by Al
/// Bawaba on 9 January 2012 ("Saturday February 2012 will be the first Saturday
/// off in Yemen"), was not carried out, as the resolution of 2013 shows. The
/// first regime is read from 2013, the year of the reports.
///
/// Sources, retrieved 2026-10-05: Yemen Post, 16 August 2013 (archive copy)
/// <https://web.archive.org/web/20130819194204id_/http://yemenpost.net/Detail123456789.aspx?ID=3&SubID=7132&MainCat=3>;
/// Al Khaleej, 17 August 2013
/// <https://www.alkhaleej.ae/2013-08-17/%D8%B5%D9%86%D8%B9%D8%A7%D8%A1-%D8%AA%D8%B9%D8%AA%D9%85%D8%AF-%D8%A7%D9%84%D8%B3%D8%A8%D8%AA-%D8%B9%D8%B7%D9%84%D8%A9-%D8%B1%D8%B3%D9%85%D9%8A%D8%A9-%D8%A8%D8%AF%D9%84%D8%A7%D9%8B-%D9%85%D9%86-%D8%A7%D9%84%D8%AE%D9%85%D9%8A%D8%B3/%D8%A7%D9%84%D8%B9%D8%A7%D9%84%D9%85/%D8%B3%D9%8A%D8%A7%D8%B3%D8%A9>;
/// Al Bawaba, 9 January 2012 (archive copy)
/// <https://web.archive.org/web/20130702222626id_/http://www.albawaba.com/editorchoice/yemen-have-friday-saturday-weekend-408239>.
pub static YE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2012),
    WeekendPolicy::of(THU_FRI).from(2013).until_day(2013, 8, 14),
    WeekendPolicy::of(FRI_SAT).from_day(2013, 8, 15),
];

/// South Africa: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static ZA: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Zambia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static ZM: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Zimbabwe: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static ZW: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// B3 (São Paulo): Saturday and Sunday from 2021, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap. The lists are B3, "Trading calendar"
/// (<https://www.b3.com.br/en_us/solutions/platforms/puma-trading-system/for-members-and-traders/trading-calendar/holidays/>),
/// retrieved 2026-09-23, as the table's `sources` names them.
pub static BVMF: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// Moscow Exchange: Saturday and Sunday from 2023, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap. The lists are the Moscow Exchange's «Расписание торгов на Московской
/// бирже в праздничные дни» for 2023 (<https://www.moex.com/n51887>), retrieved
/// 2026-09-23, as the table's `sources` names them.
pub static MISX: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2022),
    WeekendPolicy::of(SAT_SUN).from(2023),
];

/// Euronext Amsterdam: Saturday and Sunday from 2021, the first year of the
/// lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap. The lists are Euronext, "Trading hours & holidays"
/// (<https://www.euronext.com/en/trade/trading-hours-holidays>), retrieved
/// 2026-09-23, as the table's `sources` names them.
pub static XAMS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// Australian Securities Exchange: Saturday and Sunday from 2026, the first
/// year of the lists the table's closed days are read from, which name weekday
/// closures only; the exchange's trading week before it is not read, and the
/// years before are a gap. The lists are ASX, "Trading calendar"
/// (<https://www.asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours/trading-calendar>),
/// retrieved 2026-09-23, as the table's `sources` names them.
pub static XASX: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Stock Exchange of Thailand: Saturday and Sunday from 2022, the first year of
/// the lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap. The lists are SET, "SET Holidays"
/// (<https://www.set.or.th/en/about/event-calendar/holiday>), retrieved
/// 2026-09-25, as the table's `sources` names them.
pub static XBKK: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2021),
    WeekendPolicy::of(SAT_SUN).from(2022),
];

/// BSE (Bombay Stock Exchange): Saturday and Sunday from 2020, the first year
/// of the lists the table's closed days are read from, which name weekday
/// closures only; the exchange's trading week before it is not read, and the
/// years before are a gap. The lists are BSE, "Trading Holidays" notices
/// (<https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars.aspx?page=NOTICE>),
/// retrieved 2026-09-23, as the table's `sources` names them.
pub static XBOM: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2019),
    WeekendPolicy::of(SAT_SUN).from(2020),
];

/// Euronext Brussels: Saturday and Sunday from 2021, the first year of the
/// lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap. The lists are Euronext, "Trading hours & holidays"
/// (<https://www.euronext.com/en/trade/trading-hours-holidays>), retrieved
/// 2026-09-23, as the table's `sources` names them.
pub static XBRU: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// Nasdaq Copenhagen: Saturday and Sunday from 2025, the first year of the
/// lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap. The lists are Nasdaq, "European Markets Trading Hours"
/// (<https://www.nasdaqomxnordic.com/tradinghours>), retrieved 2026-09-23, as
/// the table's `sources` names them.
pub static XCSE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2024),
    WeekendPolicy::of(SAT_SUN).from(2025),
];

/// Euronext Dublin: Saturday and Sunday from 2021, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap. The lists are Euronext, "Trading hours & holidays"
/// (<https://www.euronext.com/en/trade/trading-hours-holidays>), retrieved
/// 2026-09-23, as the table's `sources` names them.
pub static XDUB: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// Frankfurt Stock Exchange (Xetra): Saturday and Sunday from 2026, the first
/// year of the lists the table's closed days are read from, which name weekday
/// closures only; the exchange's trading week before it is not read, and the
/// years before are a gap. The lists are Deutsche Börse, "Trading calendar and
/// trading hours"
/// (<https://www.cashmarket.deutsche-boerse.com/cash-en/trading/trading-calendar-and-trading-hours>),
/// retrieved 2026-09-23, as the table's `sources` names them.
pub static XETR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Nasdaq Helsinki: Saturday and Sunday from 2025, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap. The lists are Nasdaq, "European Markets Trading Hours"
/// (<https://www.nasdaqomxnordic.com/tradinghours>), retrieved 2026-09-23, as
/// the table's `sources` names them.
pub static XHEL: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2024),
    WeekendPolicy::of(SAT_SUN).from(2025),
];

/// Stock Exchange of Hong Kong (HKEX): Saturday and Sunday from 2026, the first
/// year of the lists the table's closed days are read from, which name weekday
/// closures only; the exchange's trading week before it is not read, and the
/// years before are a gap. The lists are HKEX, "HKEX Calendar"
/// (<https://www.hkex.com.hk/News/HKEX-Calendar>), retrieved 2026-09-23, as the
/// table's `sources` names them.
pub static XHKG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Nasdaq Iceland: Saturday and Sunday from 2025, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap. The lists are Nasdaq, "European Markets Trading Hours"
/// (<https://www.nasdaqomxnordic.com/tradinghours>), retrieved 2026-09-23, as
/// the table's `sources` names them.
pub static XICE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2024),
    WeekendPolicy::of(SAT_SUN).from(2025),
];

/// Indonesia Stock Exchange: Saturday and Sunday from 2020, the first year of
/// the lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap. The lists are IDX, "Trading Holiday"
/// (<https://www.idx.co.id/en-us/news/trading-holiday/>), retrieved 2026-09-23,
/// as the table's `sources` names them.
pub static XIDX: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2019),
    WeekendPolicy::of(SAT_SUN).from(2020),
];

/// Borsa İstanbul: Saturday and Sunday from 2019, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap. The lists are Borsa İstanbul, "Official Holidays"
/// (<https://www.borsaistanbul.com/en/official-holidays>), retrieved
/// 2026-09-23, as the table's `sources` names them.
pub static XIST: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2018),
    WeekendPolicy::of(SAT_SUN).from(2019),
];

/// Tokyo Stock Exchange (JPX): Saturday and Sunday from 2026, the first year of
/// the lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap. The lists are JPX, "Trading calendar"
/// (<https://www.jpx.co.jp/english/corporate/about-jpx/calendar/index.html>),
/// retrieved 2026-09-23, as the table's `sources` names them.
pub static XJPX: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Johannesburg Stock Exchange: Saturday and Sunday from 2024, the first year
/// of the lists the table's closed days are read from, which name weekday
/// closures only; the exchange's trading week before it is not read, and the
/// years before are a gap. The lists are JSE, the market calendars
/// (<https://clientportal.jse.co.za/reports/trading-calendars>), retrieved
/// 2026-09-23, as the table's `sources` names them.
pub static XJSE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2023),
    WeekendPolicy::of(SAT_SUN).from(2024),
];

/// Bursa Malaysia: Saturday and Sunday from 2020, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap. The lists are Bursa Malaysia, "Calendar"
/// (<https://www.bursamalaysia.com/about_bursa/about_us/calendar>), retrieved
/// 2026-09-23, as the table's `sources` names them.
pub static XKLS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2019),
    WeekendPolicy::of(SAT_SUN).from(2020),
];

/// Korea Exchange: Saturday and Sunday from 2009, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap. The lists are KRX, "Market Closing(Holiday)"
/// (<https://global.krx.co.kr/contents/GLB/05/0501/0501110000/GLB0501110000.jsp>),
/// retrieved 2026-09-23, as the table's `sources` names them.
pub static XKRX: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2008),
    WeekendPolicy::of(SAT_SUN).from(2009),
];

/// Euronext Lisbon: Saturday and Sunday from 2021, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap. The lists are Euronext, "Trading hours & holidays"
/// (<https://www.euronext.com/en/trade/trading-hours-holidays>), retrieved
/// 2026-09-23, as the table's `sources` names them.
pub static XLIS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// London Stock Exchange: Saturday and Sunday from 2026, the first year of the
/// lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap. The lists are London Stock Exchange, "Business days"
/// (<https://www.londonstockexchange.com/equities-trading/business-days>),
/// retrieved 2026-09-23, as the table's `sources` names them.
pub static XLON: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Bolsa de Madrid (BME): Saturday and Sunday from 2023, the first year of the
/// lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap. The lists are BME, "Calendario del mercado"
/// (<https://www.bolsasymercados.es/es/bme-exchange/negociar/calendario-del-mercado.html>),
/// retrieved 2026-09-23, as the table's `sources` names them.
pub static XMAD: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2022),
    WeekendPolicy::of(SAT_SUN).from(2023),
];

/// Bolsa Mexicana de Valores: Saturday and Sunday from 2019, the first year of
/// the lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap. The lists are BMV, "Calendario de días festivos"
/// (<https://www.bmv.com.mx/es/grupo-bmv/calendario-de-dias-festivos>),
/// retrieved 2026-09-23, as the table's `sources` names them.
pub static XMEX: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2018),
    WeekendPolicy::of(SAT_SUN).from(2019),
];

/// Euronext Milan (Borsa Italiana): Saturday and Sunday from 2021, the first
/// year of the lists the table's closed days are read from, which name weekday
/// closures only; the exchange's trading week before it is not read, and the
/// years before are a gap. The lists are Euronext, "Trading hours & holidays"
/// (<https://www.euronext.com/en/trade/trading-hours-holidays>), retrieved
/// 2026-09-23, as the table's `sources` names them.
pub static XMIL: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// Nasdaq: Saturday and Sunday from 2026, the first year of the lists the
/// table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap. The lists are Nasdaq Trader, "Trading Calendar"
/// (<https://www.nasdaqtrader.com/trader.aspx?id=calendar>), retrieved
/// 2026-09-23, as the table's `sources` names them.
pub static XNAS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// National Stock Exchange of India: Saturday and Sunday from 2020, the first
/// year of the lists the table's closed days are read from, which name weekday
/// closures only; the exchange's trading week before it is not read, and the
/// years before are a gap. The lists are NSE, "Trading Holidays" circulars
/// (<https://www.nseindia.com/resources/exchange-communication-circulars>),
/// retrieved 2026-09-23, as the table's `sources` names them.
pub static XNSE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2019),
    WeekendPolicy::of(SAT_SUN).from(2020),
];

/// New York Stock Exchange: Saturday and Sunday from 1953: the exchange stopped
/// trading on Saturdays in 1952 — "In 1952, Saturday trading hours are
/// eliminated, establishing the five-day trading week" (Wikipedia, "New York
/// Stock Exchange", <https://en.wikipedia.org/wiki/New_York_Stock_Exchange>,
/// retrieved 2026-10-05, secondary; the day was not verified), and 1953 is
/// the first whole year after. The lists the table's closed days come from,
/// NYSE's "Holidays & Trading Hours"
/// (<https://www.nyse.com/markets/hours-calendars>, retrieved 2026-09-22), are
/// read from 2026.
pub static XNYS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(1952),
    WeekendPolicy::of(SAT_SUN).from(1953),
];

/// NZX: Saturday and Sunday from 2021, the first year of the lists the table's
/// closed days are read from, which name weekday closures only; the exchange's
/// trading week before it is not read, and the years before are a gap. The
/// lists are NZX, "NZX Market Holidays" memos
/// (<https://www.nzx.com/announcements/383874>), retrieved 2026-09-23, as the
/// table's `sources` names them.
pub static XNZE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// Euronext Oslo (Oslo Børs): Saturday and Sunday from 2021, the first year of
/// the lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap. The lists are Euronext, "Trading hours & holidays"
/// (<https://www.euronext.com/en/trade/trading-hours-holidays>), retrieved
/// 2026-09-23, as the table's `sources` names them.
pub static XOSL: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// Euronext Paris: Saturday and Sunday from 2021, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap. The lists are Euronext, "Trading hours & holidays"
/// (<https://www.euronext.com/en/trade/trading-hours-holidays>), retrieved
/// 2026-09-23, as the table's `sources` names them.
pub static XPAR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// Philippine Stock Exchange: Saturday and Sunday from 2020, the first year of
/// the lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap. The lists are PSE, "Trading Hours & Holidays"
/// (<https://www.pse.com.ph/trading-hours-and-holidays/>), retrieved
/// 2026-09-23, as the table's `sources` names them.
pub static XPHS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2019),
    WeekendPolicy::of(SAT_SUN).from(2020),
];

/// Saudi Exchange (Tadawul): Sunday to Thursday, as the exchange's "Trading
/// Days: Sunday to Thursday" has it, since the royal order of 23 June 2013
/// moved the kingdom's working week and named the exchange among those it
/// bound, from Saturday 29 June 2013; before it the exchange traded Saturday to
/// Wednesday (Saturday 25 and Sunday 26 February 2006 were trading days, the
/// earliest dated source read; Thursday as a closed day then is inferred from
/// the national weekend). Every year before 2023 is a gap for the days in any
/// case.
pub static XSAU: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2006, 2, 24),
    WeekendPolicy::of(THU_FRI)
        .from_day(2006, 2, 25)
        .until_day(2013, 6, 28),
    WeekendPolicy::of(FRI_SAT).from_day(2013, 6, 29),
];

/// Singapore Exchange: Saturday and Sunday from 2020, the first year of the
/// lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap. The lists are SGX, "Securities Trading"
/// (<https://www.sgx.com/stock-exchange/trading>), retrieved 2026-09-23, as the
/// table's `sources` names them.
pub static XSES: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2019),
    WeekendPolicy::of(SAT_SUN).from(2020),
];

/// Shenzhen Stock Exchange: Saturday and Sunday from 2015, the first year of
/// the lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap. The lists are 深圳证券交易所, the 休市安排 notices
/// (<https://www.szse.cn/disclosure/notice/general/>), retrieved 2026-09-25, as
/// the table's `sources` names them.
pub static XSHE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2014),
    WeekendPolicy::of(SAT_SUN).from(2015),
];

/// Shanghai Stock Exchange: Saturday and Sunday from 2014, the first year of
/// the lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap. The lists are 上海证券交易所, the 休市安排 notices
/// (<https://www.sse.com.cn/disclosure/dealinstruc/closed/list/>), retrieved
/// 2026-09-23, as the table's `sources` names them.
pub static XSHG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2013),
    WeekendPolicy::of(SAT_SUN).from(2014),
];

/// Nasdaq Stockholm: Saturday and Sunday from 2025, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap. The lists are Nasdaq, "European Markets Trading Hours"
/// (<https://www.nasdaqomxnordic.com/tradinghours>), retrieved 2026-09-23, as
/// the table's `sources` names them.
pub static XSTO: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2024),
    WeekendPolicy::of(SAT_SUN).from(2025),
];

/// SIX Swiss Exchange: Saturday and Sunday from 2026, the first year of the
/// lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap. The lists are SIX, "Trading & Currency Holiday Calendar"
/// (<https://www.six-group.com/en/market-data/news-tools/trading-currency-holiday-calendar.html>),
/// retrieved 2026-09-23, as the table's `sources` names them.
pub static XSWX: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Tel Aviv Stock Exchange: Sunday to Thursday to Thursday 1 January 2026,
/// Monday to Friday from Monday 5 January 2026: Friday 2 January was a weekend
/// day of the old week and Sunday 4 January one of the new, with no session, as
/// Solactive's notice of 8 January 2026 records (its page,
/// <https://www.solactive.com/transition-to-monday-friday-calculation-indices-with-tel-aviv-stock-exchange-listed-securities-effective-january-5-2026/>,
/// answered 404 on 2026-10-05 and is not re-read); MSCI's announcement
/// "Change of Israel Trading Schedule - Effective January 5th, 2026" of 12
/// December 2025
/// (<https://app2.msci.com/webapp/index_ann/DocGet?pub_key=5P%2FP7%2F0GDSk%3D&lang=en&format=html>,
/// retrieved 2026-10-05) says that "effective January 5th, 2026, its trading
/// week will move from Sunday-Thursday to Monday-Friday". The schedules of the
/// exchange's vacation days, TASE's "Trading Vacation Schedule"
/// (<https://www.tase.co.il/en/content/knowledge_center/trading_vacation_schedule>,
/// retrieved 2026-09-23), are read from 2024.
pub static XTAE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2023),
    WeekendPolicy::of(FRI_SAT).from(2024).until_day(2026, 1, 3),
    WeekendPolicy::of(SAT_SUN).from_day(2026, 1, 4),
];

/// Taiwan Stock Exchange: Saturday and Sunday from 2023, the first year of the
/// lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap. The lists are 臺灣證券交易所, 市場開休市日期
/// (<https://www.twse.com.tw/zh/trading/holiday.html>), retrieved 2026-09-23,
/// as the table's `sources` names them.
pub static XTAI: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2022),
    WeekendPolicy::of(SAT_SUN).from(2023),
];

/// Toronto Stock Exchange: Saturday and Sunday from 2025, the first year of the
/// lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap. The lists are TMX Group, "Calendar"
/// (<https://www.tsx.com/en/trading/calendars-and-trading-hours/calendar>),
/// retrieved 2026-09-23, as the table's `sources` names them.
pub static XTSE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2024),
    WeekendPolicy::of(SAT_SUN).from(2025),
];

/// Warsaw Stock Exchange (GPW): Saturday and Sunday from 2019, the first year
/// of the lists the table's closed days are read from, which name weekday
/// closures only; the exchange's trading week before it is not read, and the
/// years before are a gap. The lists are GPW, "Szczegóły sesji"
/// (<https://www.gpw.pl/szczegoly-sesji>), retrieved 2026-09-23, as the table's
/// `sources` names them.
pub static XWAR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2018),
    WeekendPolicy::of(SAT_SUN).from(2019),
];

/// Wiener Börse: Saturday and Sunday from 2019, the first year of the lists the
/// table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap. The lists are Wiener Börse, "Handelskalender"
/// (<https://www.wienerborse.at/handel/handelsinformationen/handelskalender/>),
/// retrieved 2026-09-23, as the table's `sources` names them.
pub static XWBO: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2018),
    WeekendPolicy::of(SAT_SUN).from(2019),
];

/// The weekend of Malaysia and of the states whose own law differs, over the
/// years the sources reach (ADR 0015).
///
/// Most of Malaysia keeps Saturday and Sunday, the table's own policy, read
/// from 25 November 2013, the day of the Jakarta Post's report that names the
/// states that keep Friday; the National Security Council's notice of 31
/// December 2024 says the same. Four states have kept Friday: the Holidays Act
/// 1951 defines the weekly holiday as Sunday or, in the States where Friday is
/// observed, Friday, and the Unfederated Malay States, Johor, Kedah, Kelantan,
/// Perlis and Terengganu, kept Friday before independence (Tun Dr Mahathir's
/// memoir, as a blog quotes it, secondary).
///
/// * **Kedah, Kelantan and Terengganu** keep Friday and Saturday. The sources
///   read reach back to Jakarta Post's report of 25 November 2013 that they are
///   the states that "now have Friday and Saturday as rest days", and the
///   National Security Council's (MKN) notice of 31 December 2024 says they
///   still are the only ones, the policy running from the report's day. Before
///   it the weekend law of each state was not read: those years are a gap,
///   which is also why Perlis, which the report does not name and which keeps
///   Saturday and Sunday in the sources read, has no policy of its own from
///   that day.
///
/// * **Johor** kept Saturday and Sunday from 1994 until 31 December 2013, as
///   the Jakarta Post says ("prior to 1994" Johor kept the rest days that
///   Sultan Ibrahim's decree of 2013 restored); the decree gave it Friday and
///   Saturday from 1 January 2014 (the Jakarta Post, RTM and MKN agree); and
///   from 1 January 2025 it keeps Saturday and Sunday again (the Regent's
///   announcement of 7 October 2024, RTM and MKN). The years to 1994, in which
///   the report says no more than "prior to 1994", are a gap, and 1995 is the
///   first year of Saturday and Sunday, the day in 1994 not being given.
///
/// * **Perlis** is a gap before the report's day too: the sources read give no
///   date for its move off Friday, and a blog's account of a Perlis fatwa of 30
///   July 2009 asking to keep Friday disagrees with the 1994 the memoir gives.
///
/// Sources, retrieved 2026-10-05: The Jakarta Post, “Malaysia's southern state
/// changes weekend to Friday-Saturday”, 25 November 2013
/// <https://www.thejakartapost.com/news/2013/11/25/malaysias-southern-state-changes-weekend-friday-saturday.html>;
/// Radio Televisyen Malaysia, “Johor to revert weekend to Saturday and Sunday
/// in 2025”
/// <https://berita.rtm.gov.my/highlights/senarai-berita-highlights/senarai-artikel/johor-to-revert-weekend-to-saturday-and-sunday-in-2025/>;
/// Majlis Keselamatan Negara, notice of 31 December 2024
/// <https://www.mkn.gov.my/web/ms/2024/12/31/2025-johor-cuti-hujung-minggu-sabtu-ahad-antara-perkara-baharu-berkuat-kuasa-esok/>;
/// Holidays Act 1951 (Act 369), as mylaw.my reproduces it
/// <https://mylaw.my/legislation/holidays-act-1951>; parlimen81 blog, 12
/// November 2013
/// <http://parlimen81.blogspot.com/2013/11/sejarah-mengapa-cuti-hujung-minggu-di.html>.
pub static MY: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2013, 11, 24),
    WeekendPolicy::of(SAT_SUN).from_day(2013, 11, 25),
    // Johor: the years to 1994 are unread.
    WeekendPolicy::unread().in_regions(&["MY-01"]).until(1994),
    WeekendPolicy::of(SAT_SUN)
        .in_regions(&["MY-01"])
        .from(1995)
        .until(2013),
    WeekendPolicy::of(FRI_SAT)
        .in_regions(&["MY-01"])
        .from(2014)
        .until(2024),
    // Kedah, Kelantan, Terengganu and Perlis: unread before the report's
    // day, the day after which the three keep Friday and Saturday.
    WeekendPolicy::unread()
        .in_regions(&["MY-02", "MY-03", "MY-09", "MY-11"])
        .until_day(2013, 11, 24),
    WeekendPolicy::of(FRI_SAT)
        .in_regions(&["MY-02", "MY-03", "MY-11"])
        .from_day(2013, 11, 25),
];
