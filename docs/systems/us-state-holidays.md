# The US states' own holidays

The United States has federal holidays and fifty-one sets of state ones.
This document covers the days the states' codes list beyond the federal
list, which `hc-holiday`'s `UNITED_STATES` table carries as rules scoped
to each state's ISO 3166-2 code, and the District of Columbia's.

## What it is

5 U.S.C. § 6103 lists the eleven federal holidays. They bind the federal
government's employees and, by the custom of banks and markets, set much
of the country's calendar, but they bind no private employer and no
state. Each state's code has its own list of legal holidays. Most repeat
the federal days, some under other names (Washington-Lincoln Day,
Indigenous Peoples' Day, Frances Xavier Cabrini Day), some leave one out
(Columbus Day and Juneteenth most often), and many add days of their own:
a state's founding (Alaska Day, Nevada Day, West Virginia Day, Statehood
Day in Hawaii), a battle (San Jacinto, Bennington), a person (Seward,
Kuhio, Kamehameha, Truman), the Friday after Thanksgiving, Good Friday,
the Confederate days of the South, election days.

What a state's legal holiday does is the state's own affair, and it
varies. In some codes it closes the state's offices and gives its
employees a paid day (Alabama's "official state holidays", Nevada Day,
Texas's state holidays). In others it is a legal holiday for courts,
deadlines and negotiable instruments while the offices stay open, and a
separate personnel law says which days the employees have (Florida's
§ 683.01 against § 110.117, Maryland's General Provisions against its
State Personnel and Pensions article, Kentucky's "may be closed").
Besides the holidays, most codes designate commemorative days, often
dozens, which close nothing.

## How it works

Nevada's NRS 236.015 makes "the last Friday in October (Nevada Day)" a
legal holiday and says that all state, county and city offices, courts,
public schools and the Nevada System of Higher Education must close on
it [us-nv-code]. In 2026 the last Friday of October is the 30th. So
`UNITED_STATES` asked for `US-NV` in 2026 gives Nevada Day on Friday
30 October 2026 as `Kind::Government`; asked for `US-CA` or nationwide,
it does not. The day remains a business day: a private employer in Reno
owes nothing on it, and the table's business-day arithmetic counts only
the federal days off.

Florida's § 683.01 makes Lincoln's Birthday, 12 February, a "legal
holiday" and a "public holiday", but § 110.117, which lists the days the
state's offices close, does not have it [us-fl-code]. So Lincoln's
Birthday in `US-FL` is `Kind::Observance`, and the Friday after
Thanksgiving, which § 110.117 has, is `Kind::Government`.

## What is carried

- **The federal holidays**, nationwide, as the table carried them before
  this document, with Inauguration Day in `US-DC`.
- **The days each state's code lists beyond them**, scoped to its code,
  in the table below: 147 rules in 45 states and the District, and
  California's Good Friday afternoon, a half day.
  - `Kind::Government` where the code, or the personnel law read beside
    it, closes the state's offices or makes the day a paid holiday for its
    employees, or where the code makes the day a legal holiday and says
    nothing to the contrary;
  - `Kind::Observance` where it is a legal holiday that closes nothing,
    or a day the code designates for observance. For the states whose
    legal-holiday list adds nothing, Idaho, Kansas, New Mexico, Oregon
    and South Dakota, the commemorative days of the holiday chapter read
    are carried, so that each state's own days are there; for the others
    they are not.
- **The first year** of each day is the year a source read gives as the
  day's establishment, the session law that created it, where the
  section's history names one (South Dakota, Kansas, Idaho, Colorado,
  Oregon, California's Genocide Remembrance Day, Virginia's Election Day,
  Florida's Pascua Florida Day, the District); the years before are
  absent. Otherwise it is the year of the version read, the last
  amendment's where the history gives one and the copy's date where it does
  not, and the years before are a gap, which the engine reports for the
  rule read from that year (`HolidayRule::read_from`): the day is usually older (Texas's days were recodified in
  1993, Hawaii's are of the 1890s), and the texts that would show it were
  not read. The table's "First year" column says which.
- **Not yet carried, and why:**
  - the days the table's last column names: days whose date a governor
    chooses or an election law not read sets, days for a county or a
    parish, optional days, and days on the federal date under a state
    name;
  - the Saturday afternoon half holidays of Michigan, New York,
    Pennsylvania and Tennessee: a rule gives at most sixteen days a year,
    and these are every Saturday's afternoon. California's "Good Friday
    from 12 noon until 3 p.m." (§ 6700(a)(19)) is carried, as
    `Kind::HalfDay`, from 2026, a gap before;
  - the states' weekend moves: many codes keep a Saturday day on the
    Friday and a Sunday day on the Monday, but the engine substitutes only
    days off, and a state's day is not one; the code's rule is noted;
  - New Hampshire and Oklahoma, whose codes could not be read, and Georgia,
    whose extra state holiday the Governor chooses each year: asked for,
    each keeps the federal days and reports its own as a gap. Wyoming,
    whose code adds no day, is listed among the subdivisions read and is
    complete;
  - the commemorative days of the states whose holidays are carried, some
    two hundred in the chapters met.

### The fifty-one

Read on 2026-09-29, each in the legislature's own code site where it
could be reached, and otherwise in a copy, marked secondary in the
instrument's entry: FindLaw, Oregon.Public.Law, Colorado.Public.Law and
Texas.Public.Law, or the Internet Archive's capture of the official page
where the live site refused the connection. "Observance" marks a day
carried as `Kind::Observance`; the others are `Kind::Government`.

| Code | State | Days carried | Instrument | What the code makes of them | First year, and the years before | Not carried |
| --- | --- | --- | --- | --- | --- | --- |
| US-AL | Alabama | Confederate Memorial Day, the fourth Monday of April; Jefferson Davis' Birthday, the first Monday of June | Code of Alabama 1975 § 1-3-8, as amended by Act 2025-303 [us-al-code] | official state holidays, all state offices closed | 2025; earlier years: gap | Lee's and Jefferson's birthdays, named with the third Mondays of January and February, the federal days; Thanksgiving "as designated by the Governor"; Mardi Gras, a holiday in Baldwin and Mobile counties only; Rosa Parks Day, a local option; Peace Officers' Memorial Day (§ 1-3-9) |
| US-AK | Alaska | Seward's Day, the last Monday of March; Alaska Day, 18 October | Alaska Stat. § 44.12.010 [us-ak-code] | legal holidays | 2025; earlier years: gap | the designated days of article 2 of chapter 44.12, of which Wickersham Day and one more were read, the rest not enumerated |
| US-AZ | Arizona | Mothers' Day, the second Sunday of May; Native American Day, the Sunday on or after 2 June; Fathers' Day, the third Sunday of June; American Family Day, the first Sunday of August; Constitution Commemoration Day, the Sunday on or before 17 September | A.R.S. §§ 1-301 and 1-302 [us-az-code] | holidays; § 1-302 closes public offices and courts | 2026; earlier years: gap | National Navajo Code Talkers Day, which § 1-301 makes a holiday and § 1-313 says is not one; the days of observance of §§ 1-304 to 1-321 |
| US-AR | Arkansas | Christmas Eve, 24 December | Ark. Code Ann. § 1-5-101 [us-ar-code] | a paid holiday for state employees | 2024; earlier years: gap | the memorial days of § 1-5-106, which the section says are not legal holidays, Good Friday and Lee's day among them, and the designated days of §§ 1-5-107 to 1-5-122; the employee's birthday |
| US-CA | California | Farmworkers Day, 31 March; Lincoln Day, 12 February (observance); Genocide Remembrance Day, 24 April (observance); Admission Day, 9 September (observance); Native American Day, the fourth Friday of September (observance); Day after Thanksgiving, the Friday after Thanksgiving; Good Friday from 12 noon until 3 p.m. (half day) | Cal. Gov. Code §§ 6700, 6712 and 19853 [us-ca-code] | § 6700 holidays; the Government days are paid holidays of § 19853, the observances elective or unpaid | Genocide Remembrance Day: established 2023 (AB 1801, Stats. 2022, ch. 761); the others: 1999, 2026; earlier years: gap | Lunar New Year, dated by the new moons after the solstice; Diwali, "the 15th day of the month of Kartik", whose reckoning the section does not name; the proclamation days of §§ 6708 to 6736 |
| US-CO | Colorado | Frances Xavier Cabrini Day, the first Monday of October | C.R.S. § 24-11-101, as amended by HB20-1031 [us-co-code] | a legal holiday | Frances Xavier Cabrini Day: established 2020 (HB20-1031) | the days of observance of §§ 24-11-104 to 24-11-118, Colorado Day among them |
| US-CT | Connecticut | Lincoln Day, 12 February | Conn. Gen. Stat. § 1-4 [us-ct-code] | a legal holiday | 2026; earlier years: gap | days the Governor or the President appoints |
| US-DE | Delaware | Good Friday, Good Friday; Friday after Thanksgiving, the Friday after Thanksgiving; General Election Day, the Tuesday after the first Monday of November, even years | 1 Del. C. § 501 [us-de-code] | legal holidays | 2026; earlier years: gap | Return Day, the afternoon of the second day after the general election, in Sussex County only; the two floating holidays |
| US-DC | District of Columbia | District of Columbia Emancipation Day, 16 April | D.C. Code §§ 1-612.02 and 28-2701 [us-dc-code] | a legal public holiday for the District's employees | District of Columbia Emancipation Day: established 2005 (D.C. Law 15-288) |  |
| US-FL | Florida | Birthday of Martin Luther King, Jr., 15 January (observance); Birthday of Robert E. Lee, 19 January (observance); Lincoln's Birthday, 12 February (observance); Susan B. Anthony's Birthday, 15 February (observance); Tuskegee Airmen Commemoration Day, the fourth Thursday of March (observance); Good Friday, Good Friday (observance); Pascua Florida Day, 2 April (observance); Confederate Memorial Day, 26 April (observance); Birthday of Jefferson Davis, 3 June (observance); Flag Day, 14 June (observance); General Election Day, the Tuesday after the first Monday of November, even years (observance); Friday after Thanksgiving, the Friday after Thanksgiving | Fla. Stat. §§ 683.01, 683.06 and 110.117 (2026) [us-fl-code] | legal and public holidays that close nothing (observance), and the § 110.117 paid holiday | Pascua Florida Day: established 1953 (ch. 28063, 1953, § 683.06); the others: 2022, 2026; earlier years: gap | Shrove Tuesday, in counties where carnival associations are organised; Gasparilla Day, DeSoto Day and Parade Day, county days the statutes do not date; Rosh Hashanah and Yom Kippur, court holidays at a chief judge's option; the designated days of §§ 683.04 to 683.335 |
| US-GA | Georgia | none carried | [us-ga-code] |  |  | the state holiday the Governor chooses each year from 19 January, 26 April, 3 June or a date in lieu; the designated days of §§ 1-4-5 to 1-4-26 |
| US-HI | Hawaii | Prince Jonah Kuhio Kalanianaole Day, 26 March; Good Friday, Good Friday; King Kamehameha I Day, 11 June; Statehood Day, the third Friday of August; General Election Day, the Tuesday after the first Monday of November, even years | Haw. Rev. Stat. §§ 8-1 and 8-2 [us-hi-code] | state holidays | 2001; earlier years: gap | the days of §§ 8-1.5 to 8-42 that the chapter says are not state holidays |
| US-ID | Idaho | Constitutional Commemorative Day, 17 September (observance); Children's Day, 30 April (observance); Idaho Day, 4 March (observance) | Idaho Code §§ 73-108A to 73-108C [us-id-code] | commemorative days | Constitutional Commemorative Day: established 1989 (1989, ch. 77); Children's Day: established 2003 (2003, ch. 110); Idaho Day: established 2014 (2014, ch. 31) |  |
| US-IL | Illinois | Lincoln's Birthday, 12 February (observance); Casimir Pulaski's Birthday, the first Monday of March (observance); Good Friday, Good Friday (observance); General Election Day, the Tuesday after the first Monday of November, even years (observance) | 205 ILCS 630/17 [us-il-code] | legal holidays on which a bank may close | 2022; earlier years: gap |  |
| US-IN | Indiana | Lincoln's Birthday, 12 February; Good Friday, Good Friday; Election Day, the Tuesday after the first Monday of November | Ind. Code §§ 1-1-9-1 and 1-1-9-2 [us-in-code] | legal holidays and paid holidays for state employees | 2026; earlier years: gap | the primary election days, whose dates are in the election law, not read |
| US-IA | Iowa | Lincoln's Birthday, 12 February (observance); Friday after Thanksgiving, the Friday after Thanksgiving | Iowa Code §§ 1C.1 and 1C.2 (2026) [us-ia-code] | a legal public holiday without closure, and a paid holiday | 1993, 2008; earlier years: gap | the recognition days of §§ 1C.3 to 1C.17 |
| US-KS | Kansas | General Pulaski's Memorial Day, 11 October (observance); Family Day, the Sunday on or after 25 November (observance); Pearl Harbor Remembrance Day, 7 December (observance); Dwight D. Eisenhower Day, 14 October (observance); Native American Day, the fourth Saturday of September (observance); National Day of the Cowboy, the fourth Saturday of July (observance) | K.S.A. 35-201 to 35-208 [us-ks-code] | days of commemoration | General Pulaski's Memorial Day: established 1935 (L. 1935, ch. 276); Family Day: established 1971 (L. 1971, ch. 148); Pearl Harbor Remembrance Day: established 1988 (L. 1988, ch. 287); Dwight D. Eisenhower Day: established 1999 (L. 1999, ch. 144); National Day of the Cowboy: established 2014 (L. 2014, ch. 133); Native American Day: set in 1945 (L. 1945, ch. 347), carried under its name from 2013 (L. 2013, ch. 79); 1945 to 2012: gap | K.S.A. 35-107 adds no legal holiday; Arbor Day (first year not stated), Native American Legislative Day and the Mother's Day flag |
| US-KY | Kentucky | Robert E. Lee Day, 19 January (observance); Franklin D. Roosevelt Day, 30 January (observance); Lincoln's Birthday, 12 February (observance); Confederate Memorial Day and Jefferson Davis Day, 3 June (observance); Presidential Election Day, the Tuesday after the first Monday of November, presidential years | KRS 2.110, 2.190 and 18A.190 [us-ky-code] | days on which public offices "may be closed", and a state holiday that closes offices and schools | 2025; earlier years: gap | Good Friday's half day; the extra days with New Year's Day, Thanksgiving and Christmas that the Governor designates |
| US-LA | Louisiana | Battle of New Orleans, 8 January (observance); Mardi Gras, Shrove Tuesday; Good Friday, Good Friday; Huey P. Long Day, 30 August (observance); All Saints' Day, 1 November (observance) | La. R.S. 1:55 [us-la-code] | legal holidays; the Government days are ones state employees do not work | 2026; earlier years: gap | Inauguration Day in Baton Rouge; the even-year election day of subsection B, read only in a summary; the court clerks' days and festival Fridays; Acadian Day, by proclamation; Mardi Gras's parish and bank rules |
| US-ME | Maine | Patriot's Day, the third Monday of April | 4 M.R.S. § 1051 [us-me-code] | a legal holiday | 2026; earlier years: gap |  |
| US-MD | Maryland | Lincoln's Birthday, 12 February (observance); Maryland Day, 25 March (observance); Good Friday, Good Friday (observance); Defenders' Day, 12 September (observance); American Indian Heritage Day, the Friday after Thanksgiving; General Election Day, the Tuesday after the first Monday of November, even years | Md. Code, General Provisions § 1-111 and State Personnel and Pensions § 9-201 [us-md-code] | legal holidays, and employee holidays of SPP § 9-201 | 2026; earlier years: gap | days the President or the Governor designates for a general cessation of business |
| US-MA | Massachusetts | Patriots' Day, the third Monday of April | Mass. Gen. Laws c. 4, § 7, cl. Eighteenth [us-ma-code] | a legal holiday | 2025; earlier years: gap | Evacuation Day and Bunker Hill Day, legal holidays in Suffolk County only |
| US-MI | Michigan | Lincoln's Birthday, 12 February (observance) | MCL 435.101 [us-mi-code] | a public holiday for instruments and courts | 2025; earlier years: gap | the Saturday afternoon half holiday |
| US-MN | Minnesota | Friday after Thanksgiving, the Friday after Thanksgiving | Minn. Stat. § 645.44, subd. 5 (2025) [us-mn-code] | a holiday for the executive branch | 2025; earlier years: gap |  |
| US-MS | Mississippi | Confederate Memorial Day, the last Monday of April | Miss. Code § 3-3-7 [us-ms-code] | a legal holiday | 2025; earlier years: gap | Lee's and Jefferson Davis's birthdays, named with the federal Mondays in January and May; Mardi Gras, a local substitute; the recognition days of § 3-3-7(3) to (7) |
| US-MO | Missouri | Lincoln Day, 12 February; Truman Day, 8 May | RSMo 9.010 and 9.035 [us-mo-code] | public holidays | 2022; earlier years: gap | the commemorative days of chapter 9 beyond §§ 9.010 to 9.035, not read |
| US-MT | Montana | General Election Day, the Tuesday after the first Monday of November, even years | MCA 1-1-216 (2025) [us-mt-code] | a legal holiday | 2025; earlier years: gap | the observances of §§ 1-1-224 to 1-1-233, Juneteenth on the third Saturday of June and American Indian Heritage Day among them |
| US-NE | Nebraska | Arbor Day, the last Friday of April; Day after Thanksgiving, the Friday after Thanksgiving | Neb. Rev. Stat. §§ 25-2221 and 84-1001 [us-ne-code] | court holidays and paid state holidays (§ 84-1001) | 2024; earlier years: gap |  |
| US-NV | Nevada | Nevada Day, the last Friday of October; Family Day, the Friday after Thanksgiving | NRS 236.015 [us-nv-code] | legal holidays that close state, county and city offices, courts and schools | 2025; earlier years: gap |  |
| US-NH | New Hampshire | none carried |  |  |  | nothing read: RSA 288:1 refused the connection on the Court's site and was not found in a copy |
| US-NJ | New Jersey | Lincoln's Birthday, 12 February (observance); Good Friday, Good Friday; Juneteenth Day, the third Friday of June; General Election Day, the Tuesday after the first Monday of November | N.J.S.A. 36:1-1 [us-nj-code] | public holidays; Lincoln's Birthday not one for state business since 2008 | 2024; earlier years: gap | every Saturday, a public holiday for instruments |
| US-NM | New Mexico | American Indian Day, the first Friday of February (observance); Guadalupe Hidalgo Treaty Day, 2 February (observance); African-American Day, the second Friday of February (observance); Arbor Day, the second Friday of March (observance); Bataan Day, 9 April (observance); Ernie Pyle Day, 3 August (observance) | NMSA 12-5-1, 12-5-4, 12-5-7, 12-5-9, 12-5-10 and 12-5-12 [us-nm-code] | days set apart, without closure | 2024; earlier years: gap | the other days of article 12-5 |
| US-NY | New York | Lincoln's Birthday, 12 February; Flag Day, the second Sunday of June; General Election Day, the Tuesday after the first Monday of November | N.Y. Gen. Constr. Law § 24 [us-ny-code] | public holidays | 2020; earlier years: gap | days appointed for thanksgiving, fasting or prayer; the Saturday afternoon half holiday |
| US-NC | North Carolina | Robert E. Lee's Birthday, 19 January (observance); Greek Independence Day, 25 March (observance); Anniversary of the Halifax Resolves, 12 April (observance); Confederate Memorial Day, 10 May (observance); Anniversary of the Mecklenburg Declaration of Independence, 20 May (observance); Good Friday, Good Friday (observance); First Responders Day, 11 September (observance); Yom Kippur, 10 Tishri (observance); Election Day, the Tuesday after the first Monday of November, even years (observance) | N.C. Gen. Stat. § 103-4 [us-nc-code] | legal public holidays, whose closures the State Human Resources Commission sets | 2023; earlier years: gap |  |
| US-ND | North Dakota | Good Friday, Good Friday | N.D. Cent. Code § 1-03-01 [us-nd-code] | a holiday | 2024; earlier years: gap |  |
| US-OH | Ohio | none carried | [us-oh-code] |  |  | none: Ohio Rev. Code § 1.14, in FindLaw's copy, lists only the federal days and days the Governor or the President appoints |
| US-OK | Oklahoma | none carried |  |  |  | nothing read: 25 O.S. §§ 82.1 and 82.2 refused the connection on the official site and were not found in a copy |
| US-OR | Oregon | Oregon Statehood Day, 14 February (observance) | ORS 187.278 [us-or-code] | a commemorative day | Oregon Statehood Day: established 2015 (2015 c.578) | Indigenous Peoples' Day (ORS 187.296), a designation on the federal Columbus Day |
| US-PA | Pennsylvania | Good Friday, Good Friday (observance); Flag Day, 14 June (observance); Election Day, the Tuesday after the first Monday of November (observance) | 44 P.S. § 11 [us-pa-code] | legal holidays for instruments | 2026; earlier years: gap | the Saturday afternoon half holiday; a bank holiday a governor or president declares |
| US-RI | Rhode Island | Rhode Island Independence Day, 4 May; Victory Day, the second Monday of August; Election Day, the Tuesday after the first Monday of November, even years | R.I. Gen. Laws § 25-1-1 [us-ri-code] | holidays | 2026; earlier years: gap |  |
| US-SC | South Carolina | Confederate Memorial Day, 10 May; Day after Thanksgiving, the Friday after Thanksgiving; Christmas Eve, 24 December; 26 December, 26 December | S.C. Code Ann. §§ 53-5-10 and 53-5-30, as amended by 2009 Act No. 33 [us-sc-code] | legal holidays | 2009, 2010; earlier years: gap |  |
| US-SD | South Dakota | South Dakota Statehood Day, 2 November (observance); Little Big Horn Recognition Day, 25 June (observance); Wounded Knee Day, 29 December (observance); Bill of Rights Day, 15 December (observance); Joe Foss Day, 17 April (observance); Purple Heart Recognition Day, 7 August (observance); Welcome Home Vietnam Veterans Day, 30 March (observance); Day of the American Cowboy, the fourth Saturday of July (observance); Peter Norbeck Day, 27 August (observance); Medal of Honor Recognition Day, 25 March (observance) | SDCL 1-5-1.3 and 1-5-8 to 1-5-18 [us-sd-code] | working holidays | South Dakota Statehood Day: established 2001 (SL 2001, ch 6); Little Big Horn Recognition Day: established 1994 (SL 1994, ch 2); Wounded Knee Day: established 1994 (SL 1994, ch 2); Bill of Rights Day: established 1998 (SL 1998, ch 5); Joe Foss Day: established 2004 (SL 2004, ch 5); Purple Heart Recognition Day: established 2013 (SL 2013, ch 2); Welcome Home Vietnam Veterans Day: established 2013 (SL 2013, ch 3); Day of the American Cowboy: established 2014 (SL 2014, ch 3); Peter Norbeck Day: established 2018 (SL 2018, ch 5); Medal of Honor Recognition Day: established 2024 (SL 2024, ch 4) | Native Americans' Day, the legal holiday on the federal Columbus Day; Arbor Day and POW/MIA Recognition Day, whose wording was read only in a summary |
| US-TN | Tennessee | Good Friday, Good Friday | Tenn. Code Ann. § 15-1-101 [us-tn-code] | a legal holiday | 2024; earlier years: gap | election days; the Saturday afternoon half holiday |
| US-TX | Texas | Confederate Heroes Day, 19 January; Texas Independence Day, 2 March; San Jacinto Day, 21 April; Lyndon Baines Johnson Day, 27 August; Friday after Thanksgiving, the Friday after Thanksgiving; 24 December, 24 December; 26 December, 26 December | Tex. Gov't Code §§ 662.003 to 662.005 [us-tx-code] | state holidays, paid days off for state employees | 2025; earlier years: gap | the optional holidays, Rosh Hashanah, Yom Kippur, Good Friday and Cesar Chavez Day; Emancipation Day in Texas, on the federal 19 June |
| US-UT | Utah | Pioneer Day, 24 July; Juneteenth National Freedom Day, 19 June, kept on a Monday | Utah Code § 63G-1-301 [us-ut-code] | legal holidays | 2025; earlier years: gap | the personal preference day |
| US-VT | Vermont | Town Meeting Day, the first Tuesday of March; Bennington Battle Day, 16 August | Vt. Stat. Ann. tit. 1, § 371 [us-vt-code] | legal holidays | 2024; earlier years: gap | Indigenous Peoples' Day, on the federal Columbus Day |
| US-VA | Virginia | Election Day, the Tuesday after the first Monday of November; Day after Thanksgiving, the Friday after Thanksgiving | Va. Code § 2.2-3300, as amended by Acts 2020, cc. 417 and 418 [us-va-code] | legal holidays | Election Day: established 2020 (Acts 2020, cc. 417 and 418); the others: 2020; earlier years: gap | Lee-Jackson Day, which the section no longer lists; the chapter's days of observance |
| US-WA | Washington | Native American Heritage Day, the Friday after Thanksgiving | RCW 1.16.050 [us-wa-code] | a legal holiday | 2014; earlier years: gap | the legislatively recognized days of subsection (7), which close nothing |
| US-WV | West Virginia | West Virginia Day, 20 June; Lincoln's Day, the Friday after Thanksgiving | W. Va. Code § 2-2-1 [us-wv-code] | legal holidays | 2026; earlier years: gap | election days (items 13 and 14) |
| US-WI | Wisconsin | General Election Day, the Tuesday after the first Monday of November, even years | Wis. Stat. § 995.20 [us-wi-code] | a legal holiday | 2025; earlier years: gap | the partisan primary and municipal election days; Good Friday from eleven to three |
| US-WY | Wyoming | none carried | [us-wy-code] |  |  | none: Wyo. Stat. § 8-4-101, in FindLaw's copy, adds no day of its own |

## Accuracy

`crates/hc-holiday/tests/us_states.rs` holds each of the 147 rules to
its code: the day's date in its first year and in 2026 (the election days
in their years), worked out from the code's rule by a script apart from
the crate; its kind and its state; nothing the year before, and a gap
there unless the first year is the day's establishment (Colorado's 2019
absent, Texas's 2024 a gap, Kansas's Native American Day absent in 1944
and a gap in 1945 to 2012); Texas
Independence Day in Texas alone and still a business day there; the
even-year and presidential election days; and Utah's Monday Juneteenth.
What a reader should know:

- Most copies read were secondary, and several official sites refused
  the connection or render by script; the instrument's entry says which.
  A secondary copy may lag the law: FindLaw's copies are of 2023 to 2026.
- The first years are mostly those of the text read, not of the day, so
  a calendar for 2020 in `US-TX` reports the Texas days as gaps rather
  than claiming there were none.
- The line between `Kind::Government` and `Kind::Observance` follows
  what the code read says the day does, and a personnel rule not read can
  close offices on a day the code calls a mere legal holiday, as North
  Carolina's State Human Resources Commission does.
- Arizona's Sunday holidays are carried because § 1-301 lists them; they
  close nothing that is open.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [us-al-code] … [us-wy-code] | Each state's holiday sections: the days, their rules, their effect, the history notes | Yes, 2026-09-29; each entry says where, and whether the copy is secondary |

## Code

`crates/hc-holiday/src/countries/united_states.rs`: the federal rules
first in `US_RULES`, then the states', built by `state` and
`state_observance`, each with `.years` from its session law or
`.read_from` the year of its text, with `FRIDAY_AFTER_THANKSGIVING`, `ELECTION_DAY`,
`even_year_election_day`, `presidential_election_day` and Utah's
`TO_PRECEDING_OR_NEXT_MONDAY`; one region and one citation constant per
state, `US_AK` and `US_AK_LAW` to `US_WV` and `US_WV_LAW`. The
exchanges `XNYS` and `XNAS` keep their own lists. Anchors: the tests in
`crates/hc-holiday/tests/us_states.rs`, and the federal tests in
`crates/hc-holiday/tests/countries.rs`.
