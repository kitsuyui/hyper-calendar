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
//! the sources were read in, and every earlier year is a gap: the live page of Wikipedia's "Workweek and weekend", retrieved
//! 2026-10-04 and secondary, and the table's own sources for its holidays say
//! no more. The sources and the per-country reading are in
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
pub static AE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(1998),
    WeekendPolicy::of(THU_FRI).from(1999).until_day(2006, 8, 31),
    WeekendPolicy::of(FRI_SAT).from_day(2006, 9, 1).until(2021),
    WeekendPolicy::of(SAT_SUN).from(2022),
    WeekendPolicy::of(FRI_SAT_SUN)
        .from(2022)
        .in_regions(&["AE-SH"]),
];

/// Afghanistan: Friday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. The law read: Friday. The Ministry of Labour and
/// Social Affairs' notice for Arafah and Eid al-Adha 1447 counts "four working
/// days" from Tuesday 9 Dhu al-Hijjah to a return on Sunday the 14th, passing
/// over the Friday and counting the Thursday and the Saturday.
pub static AF: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
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

/// Armenia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static AM: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
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

/// Australia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static AU: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
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
pub static BN: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2018),
    WeekendPolicy::of(FRI_SUN).from(2019),
];

/// Bolivia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static BO: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
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

/// Belarus: Saturday and Sunday: the Labour Code of 26 July 1999, articles 124
/// and 136, gives a five-day week's days off as Saturday and Sunday (the Code's
/// text through a summarising tool, its in-force date not read, so the policy
/// begins with 2000, the year after). The Soviet-era weekend is not read.
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

/// Democratic Republic of the Congo: Saturday and Sunday, as the table carried
/// it, from 2026, the year the sources were read in; no dated source for an
/// earlier weekend was found, so the years before are a gap. Wikipedia's
/// "Workweek and weekend", retrieved 2026-10-04 (secondary), is the page read.
pub static CD: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
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

/// Chile: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static CL: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
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

/// Cuba: Article 82 of the Labour Code rests the week on Sunday, the "descanso
/// dominical" that article 97 moves. The Code is Ley 116 of 20 December 2013,
/// and the policy begins with 2014, the first whole year after it.
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

/// Czechia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static CZ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Germany: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static DE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Djibouti: Friday, from 2026, the year the sources
/// were read in; no dated source for an earlier weekend was found, so the years
/// before are a gap. The law read: Article 97 of the Labour Code: the weekly
/// rest "takes place in principle on Friday"; article 2 of arrêté 2019-193
/// gives it to all employees at once on the Friday.
pub static DJ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(FRIDAY).from(2026),
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
/// decision of 21 July 2009. The date of the first regime is the year before
/// the change, the news report's own reference.
pub static DZ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(1974),
    WeekendPolicy::of(SAT_SUN).from(1975).until(1975),
    WeekendPolicy::of(THU_FRI).from(1976).until_day(2009, 8, 13),
    WeekendPolicy::of(FRI_SAT).from_day(2009, 8, 14),
];

/// Ecuador: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static EC: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
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
pub static EG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2005),
    WeekendPolicy::of(FRIDAY).from(2006).until_day(2006, 1, 20),
    WeekendPolicy::of(FRI_SAT).from_day(2006, 1, 21),
];

/// Spain: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static ES: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Ethiopia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static ET: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Finland: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static FI: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
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

/// Greece: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static GR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
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

/// Indonesia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static ID: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
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
pub static IL: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(1951),
    WeekendPolicy::of(SATURDAY).from(1952),
];

/// India: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static IN: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Iraq: Friday and Saturday for government offices from the last week of
/// February 2005, when the Government added Saturday to Friday (Al Jazeera, 28
/// February 2005); the day is not given, so the policy begins in March. A blog
/// of March 2005 said the Government might revert to Thursday and Friday; no
/// reversal was found, and The National of 7 December 2021 says Iraq's weekend
/// is Friday and Saturday.
pub static IQ: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2005, 2, 28),
    WeekendPolicy::of(FRI_SAT).from_day(2005, 3, 1),
];

/// Iran: Friday, from 2026, the year the sources were
/// read in; no dated source for an earlier weekend was found, so the years
/// before are a gap. The law read: Iran's weekly holiday is Friday, under
/// article 17 of the Constitution; Thursday is a half-day in most offices and
/// is not a weekend day.
pub static IR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(FRIDAY).from(2026),
];

/// Iceland: Saturday and Sunday: Law 88/1971 as amended by L. 25/1979 and L.
/// 94/1982, in force on 31 December 1982, sets 40 working hours a week at eight
/// hours a day from Monday to Friday (Althingi's consolidated text); the
/// Saturday and the Sunday are the days outside that week.
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
/// from the week of 8 January 2000, as Al Wakeel News and Wikipedia's page on
/// the workweek report; the first regime is read from 1999, the year of the
/// report that gives it.
pub static JO: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(1998),
    WeekendPolicy::of(THU_FRI).from(1999).until_day(2000, 1, 7),
    WeekendPolicy::of(FRI_SAT).from_day(2000, 1, 8),
];

/// Japan: Saturday and Sunday for national civil servants from 1 May 1992, when
/// the complete two-day weekend began, as the Japanese Wikipedia's article on
/// the 週休二日制 has it (secondary); the 一般職の職員の勤務時間、休暇等に関する法律 (平成6年法律第33号),
/// article 6(1), gives them Saturday and Sunday as 週休日. Before 1992 Saturday
/// was worked in part, and the weekend is not read.
pub static JP: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(1992, 4, 30),
    WeekendPolicy::of(SAT_SUN).from_day(1992, 5, 1),
];

/// Kenya: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static KE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
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

/// South Korea: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static KR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Kuwait: Thursday and Friday until the Cabinet's decision of 27 May 2007
/// moved the Government's offices to Friday and Saturday from Saturday 1
/// September 2007, as Arab News reported it from KUNA; the first regime is read
/// from 1 January 2007.
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

/// Laos: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static LA: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
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

/// Liechtenstein: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static LI: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
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
pub static LY: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2005),
    WeekendPolicy::of(FRI_SAT).from(2006),
];

/// Morocco: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static MA: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
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

/// Madagascar: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static MG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
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

/// Mongolia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static MN: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Macau: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static MO: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Mauritania: Friday and Saturday until the decree the Council of Ministers
/// adopted on 11 September 2014, and Saturday and Sunday from Wednesday 1
/// October 2014, for the public sector. The Friday-and-Saturday regime is read
/// from 2007, the year BBC News reported Mauritania's change from a Monday-to-
/// Friday week; before it the weekend is not read.
pub static MR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2006),
    WeekendPolicy::of(FRI_SAT).from(2007).until_day(2014, 9, 30),
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
/// workweek gives for the Maldives' change; the Employment Act's section 97
/// makes every Friday a public holiday and the government works Sunday to
/// Thursday (the Ministry of Economic Development and Trade's gazette notice of
/// 27 March 2024; trade portals say the same). The regime before 2013 is not
/// read.
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
/// (<https://www.spotlightnepal.com/2026/04/05/nepal-government-decides-grant-
/// two-day-holiday-saturday-and-sunday/>), and OnlineKhabar English reported it
/// the same day from the government spokesperson as starting the next day. The
/// first Sunday off was 12 April 2026. So the Saturday-only weekend runs to
/// Sunday 5 April 2026, a working day, and Saturday and Sunday from Monday 6
/// April. The Saturday-only weekend is read from 2026, the first year the
/// sources dated it: every earlier regime is undated in all sources read.
pub static NP: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SATURDAY).from(2026).until_day(2026, 4, 5),
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
/// May 2013, for the public and private sectors alike, as Gulf News and Al
/// Riyadh reported the Council of Ministers' statement of April 2013; the
/// decree's number and text were not read. The first regime is read from 2013,
/// the year of the report.
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

/// Peru: Saturday and Sunday, from 2026, the year the
/// sources were read in; no dated source for an earlier weekend was found, so
/// the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static PE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Papua New Guinea: Saturday and Sunday, from 2026,
/// the year the sources were read in; no dated source for an earlier weekend
/// was found, so the years before are a gap. Wikipedia's "Workweek and
/// weekend", retrieved 2026-10-04 (secondary), is the page read.
pub static PG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Philippines: Saturday and Sunday, from 2026, the
/// year the sources were read in; no dated source for an earlier weekend was
/// found, so the years before are a gap. Wikipedia's "Workweek and weekend",
/// retrieved 2026-10-04 (secondary), is the page read.
pub static PH: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Pakistan: Saturday and Sunday for the federal government from 9 June 2022,
/// as news reports of the notification say. Earlier regimes the reports give
/// (Sunday from 1947, Friday from 1977, Sunday from 1997, Saturday and Sunday
/// from February 2019 and Sunday alone from 16 April to 8 June 2022) disagree
/// among themselves and are not carried. A report of March 2026 on a four-day
/// federal week was not confirmed in force.
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
pub static PL: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(1950),
    WeekendPolicy::of(SUNDAY).from(1951).until(1972),
    WeekendPolicy::unread().from(1973).until(2000),
    WeekendPolicy::of(SAT_SUN).from(2001),
];

/// Palestine: Friday and Saturday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static PS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(FRI_SAT).from(2026),
];

/// Portugal: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static PT: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
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
/// back on the Sunday after a Saturday. The Friday regime is read from 21
/// January 1998, the first date a source read gives for it (news reports of the
/// public sector).
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

/// Singapore: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static SG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Slovenia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static SI: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Slovakia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static SK: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
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

/// Somalia: Friday: article 64(1) of the Labour Code, one day's rest a week,
/// which "should normally fall on Friday" ("maalinta Jimcaha"); a 2025 news
/// article says Friday is the designated weekly day off. The code's year was
/// not read, and the policy begins with 2025.
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

/// Syria: Friday and Saturday from February 2004, when the Council of Ministers
/// made the State's offices keep a Sunday-to-Thursday week; a travel-
/// information page, not an official source, gives 1 February 2004, and the
/// instrument was not found. The weekend before it is not read.
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

/// Togo: Sunday: article 198 of the Code du travail of 2021, the weekly rest "a
/// lieu en principe le dimanche"; the Code does not govern permanent civil
/// servants. The policy begins with 2022, the first whole year after the Code.
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

/// Tunisia: Saturday and Sunday, from 2026, the year
/// the sources were read in; no dated source for an earlier weekend was found,
/// so the years before are a gap. Wikipedia's "Workweek and weekend", retrieved
/// 2026-10-04 (secondary), is the page read.
pub static TN: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
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
/// 6101 as enacted by Pub. L. 89-554 of 6 September 1966 sets the
/// administrative workweek Monday through Friday. The Government's earlier
/// workweeks, a Saturday half day among them, are not read.
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

/// Vatican City: Sunday alone: the Governorate's regulations' "day of weekly
/// rest, which coincides with Sunday", the Regulation of 21 November 2010; the
/// policy begins with 2011, the table's first year.
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

/// Vietnam: Saturday and Sunday: a travel agency's page, as the Internet
/// Archive kept it on 3 May 2017, says government agencies are closed on
/// Saturday and Sunday (a weak source; the Labour Code gives the employer's day
/// off as Sunday or another day). Before it the weekend is not read.
pub static VN: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until_day(2017, 5, 2),
    WeekendPolicy::of(SAT_SUN).from_day(2017, 5, 3),
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
/// August, which is carried as a working day. The first regime is read from
/// 2013, the year of the reports.
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
/// gap.
pub static BVMF: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// Moscow Exchange: Saturday and Sunday from 2023, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap.
pub static MISX: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2022),
    WeekendPolicy::of(SAT_SUN).from(2023),
];

/// Euronext Amsterdam: Saturday and Sunday from 2021, the first year of the
/// lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap.
pub static XAMS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// Australian Securities Exchange: Saturday and Sunday from 2026, the first
/// year of the lists the table's closed days are read from, which name weekday
/// closures only; the exchange's trading week before it is not read, and the
/// years before are a gap.
pub static XASX: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Stock Exchange of Thailand: Saturday and Sunday from 2022, the first year of
/// the lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap.
pub static XBKK: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2021),
    WeekendPolicy::of(SAT_SUN).from(2022),
];

/// BSE (Bombay Stock Exchange): Saturday and Sunday from 2020, the first year
/// of the lists the table's closed days are read from, which name weekday
/// closures only; the exchange's trading week before it is not read, and the
/// years before are a gap.
pub static XBOM: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2019),
    WeekendPolicy::of(SAT_SUN).from(2020),
];

/// Euronext Brussels: Saturday and Sunday from 2021, the first year of the
/// lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap.
pub static XBRU: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// Nasdaq Copenhagen: Saturday and Sunday from 2025, the first year of the
/// lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap.
pub static XCSE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2024),
    WeekendPolicy::of(SAT_SUN).from(2025),
];

/// Euronext Dublin: Saturday and Sunday from 2021, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap.
pub static XDUB: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// Frankfurt Stock Exchange (Xetra): Saturday and Sunday from 2026, the first
/// year of the lists the table's closed days are read from, which name weekday
/// closures only; the exchange's trading week before it is not read, and the
/// years before are a gap.
pub static XETR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Nasdaq Helsinki: Saturday and Sunday from 2025, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap.
pub static XHEL: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2024),
    WeekendPolicy::of(SAT_SUN).from(2025),
];

/// Stock Exchange of Hong Kong (HKEX): Saturday and Sunday from 2026, the first
/// year of the lists the table's closed days are read from, which name weekday
/// closures only; the exchange's trading week before it is not read, and the
/// years before are a gap.
pub static XHKG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Nasdaq Iceland: Saturday and Sunday from 2025, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap.
pub static XICE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2024),
    WeekendPolicy::of(SAT_SUN).from(2025),
];

/// Indonesia Stock Exchange: Saturday and Sunday from 2020, the first year of
/// the lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap.
pub static XIDX: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2019),
    WeekendPolicy::of(SAT_SUN).from(2020),
];

/// Borsa İstanbul: Saturday and Sunday from 2019, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap.
pub static XIST: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2018),
    WeekendPolicy::of(SAT_SUN).from(2019),
];

/// Tokyo Stock Exchange (JPX): Saturday and Sunday from 2026, the first year of
/// the lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap.
pub static XJPX: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Johannesburg Stock Exchange: Saturday and Sunday from 2024, the first year
/// of the lists the table's closed days are read from, which name weekday
/// closures only; the exchange's trading week before it is not read, and the
/// years before are a gap.
pub static XJSE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2023),
    WeekendPolicy::of(SAT_SUN).from(2024),
];

/// Bursa Malaysia: Saturday and Sunday from 2020, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap.
pub static XKLS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2019),
    WeekendPolicy::of(SAT_SUN).from(2020),
];

/// Korea Exchange: Saturday and Sunday from 2009, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap.
pub static XKRX: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2008),
    WeekendPolicy::of(SAT_SUN).from(2009),
];

/// Euronext Lisbon: Saturday and Sunday from 2021, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap.
pub static XLIS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// London Stock Exchange: Saturday and Sunday from 2026, the first year of the
/// lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap.
pub static XLON: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Bolsa de Madrid (BME): Saturday and Sunday from 2023, the first year of the
/// lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap.
pub static XMAD: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2022),
    WeekendPolicy::of(SAT_SUN).from(2023),
];

/// Bolsa Mexicana de Valores: Saturday and Sunday from 2019, the first year of
/// the lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap.
pub static XMEX: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2018),
    WeekendPolicy::of(SAT_SUN).from(2019),
];

/// Euronext Milan (Borsa Italiana): Saturday and Sunday from 2021, the first
/// year of the lists the table's closed days are read from, which name weekday
/// closures only; the exchange's trading week before it is not read, and the
/// years before are a gap.
pub static XMIL: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// Nasdaq: Saturday and Sunday from 2026, the first year of the lists the
/// table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap.
pub static XNAS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// National Stock Exchange of India: Saturday and Sunday from 2020, the first
/// year of the lists the table's closed days are read from, which name weekday
/// closures only; the exchange's trading week before it is not read, and the
/// years before are a gap.
pub static XNSE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2019),
    WeekendPolicy::of(SAT_SUN).from(2020),
];

/// New York Stock Exchange: Saturday and Sunday from 1953: the exchange stopped
/// trading on Saturdays in 1952 (Wikipedia, "New York Stock Exchange",
/// secondary; the day was not verified), and 1953 is the first whole year
/// after. The lists the table's closed days come from are read from 2026.
pub static XNYS: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(1952),
    WeekendPolicy::of(SAT_SUN).from(1953),
];

/// NZX: Saturday and Sunday from 2021, the first year of the lists the table's
/// closed days are read from, which name weekday closures only; the exchange's
/// trading week before it is not read, and the years before are a gap.
pub static XNZE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// Euronext Oslo (Oslo Børs): Saturday and Sunday from 2021, the first year of
/// the lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap.
pub static XOSL: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// Euronext Paris: Saturday and Sunday from 2021, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap.
pub static XPAR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2020),
    WeekendPolicy::of(SAT_SUN).from(2021),
];

/// Philippine Stock Exchange: Saturday and Sunday from 2020, the first year of
/// the lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap.
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
/// before are a gap.
pub static XSES: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2019),
    WeekendPolicy::of(SAT_SUN).from(2020),
];

/// Shenzhen Stock Exchange: Saturday and Sunday from 2015, the first year of
/// the lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap.
pub static XSHE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2014),
    WeekendPolicy::of(SAT_SUN).from(2015),
];

/// Shanghai Stock Exchange: Saturday and Sunday from 2014, the first year of
/// the lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap.
pub static XSHG: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2013),
    WeekendPolicy::of(SAT_SUN).from(2014),
];

/// Nasdaq Stockholm: Saturday and Sunday from 2025, the first year of the lists
/// the table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap.
pub static XSTO: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2024),
    WeekendPolicy::of(SAT_SUN).from(2025),
];

/// SIX Swiss Exchange: Saturday and Sunday from 2026, the first year of the
/// lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap.
pub static XSWX: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2025),
    WeekendPolicy::of(SAT_SUN).from(2026),
];

/// Tel Aviv Stock Exchange: Sunday to Thursday to Thursday 1 January 2026,
/// Monday to Friday from Monday 5 January 2026: Friday 2 January was a weekend
/// day of the old week and Sunday 4 January one of the new, with no session, as
/// Solactive's notice of 8 January 2026 records. The schedules of the
/// exchange's vacation days are read from 2024.
pub static XTAE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2023),
    WeekendPolicy::of(FRI_SAT).from(2024).until_day(2026, 1, 3),
    WeekendPolicy::of(SAT_SUN).from_day(2026, 1, 4),
];

/// Taiwan Stock Exchange: Saturday and Sunday from 2023, the first year of the
/// lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap.
pub static XTAI: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2022),
    WeekendPolicy::of(SAT_SUN).from(2023),
];

/// Toronto Stock Exchange: Saturday and Sunday from 2025, the first year of the
/// lists the table's closed days are read from, which name weekday closures
/// only; the exchange's trading week before it is not read, and the years
/// before are a gap.
pub static XTSE: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2024),
    WeekendPolicy::of(SAT_SUN).from(2025),
];

/// Warsaw Stock Exchange (GPW): Saturday and Sunday from 2019, the first year
/// of the lists the table's closed days are read from, which name weekday
/// closures only; the exchange's trading week before it is not read, and the
/// years before are a gap.
pub static XWAR: &[WeekendPolicy] = &[
    WeekendPolicy::unread().until(2018),
    WeekendPolicy::of(SAT_SUN).from(2019),
];

/// Wiener Börse: Saturday and Sunday from 2019, the first year of the lists the
/// table's closed days are read from, which name weekday closures only; the
/// exchange's trading week before it is not read, and the years before are a
/// gap.
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
