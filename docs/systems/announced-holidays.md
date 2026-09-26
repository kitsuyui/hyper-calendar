# Holidays announced year by year

Backs thirteen tables in `hc-holiday`: `FIJI` (`FJ`) and `KIRIBATI` (`KI`)
in `countries/oceania.rs`; `LIBERIA` (`LR`), `GAMBIA` (`GM`), `SUDAN`
(`SD`), `TOGO` (`TG`), `NIGER` (`NE`), `GABON` (`GA`), `SIERRA_LEONE`
(`SL`), `ESWATINI` (`SZ`), `GUINEA_BISSAU` (`GW`) and `SOUTH_SUDAN` (`SS`)
in `countries/africa_middle_east.rs`; and `NORTH_KOREA` (`KP`) in
`countries/asia.rs`.

## What it is

In most countries a statute lists the public holidays and a reader can
compute them. In these thirteen, what a statute says is not the whole
answer, or not the answer at all. The days actually kept are fixed by an
instrument issued for the year, or for the one holiday, shortly before it
falls:

- **A list for the year.** Fiji's Cabinet approves the holidays each year
  and the Ministry of Information publishes them. Kiribati's Beretitenti
  orders them under the Public Holidays Ordinance. South Sudan's Ministry
  of Labour publishes a Public Holidays Calendar, and North Korea's
  Foreign Languages Publishing House a wall calendar.
- **A declaration per holiday.** Liberia's President proclaims each
  holiday, citing its Act. The Gambia's Office of the President declares
  each one under section 76 of the Constitution. Sudan's Council of
  Ministers announces each "in all parts of the country". The ministries
  of labour of Togo, Niger and Gabon issue a communiqué for each, and
  Guinea-Bissau's Government decrees each Eid. Sierra Leone's Office of
  the President declares its holidays in the Gazette, and Eswatini's
  Minister appoints Umhlanga, Incwala and Labour Day by notice.

In several of them a statute sits behind the instrument and does not
settle it. Fiji's Public Holidays Act (Cap. 101) keeps a Schedule that the
yearly lists depart from. Sierra Leone's Public Holidays Act (Cap. 58)
names days no notice read declares, and the notices declare days its
Schedule does not name. Togo's law of 1987 lists twelve *fêtes légales*,
and the communiqués of 2024 to 2026 declare days it does not name, so it
is not the current list. Sudan's Labour Act pays "holidays and official
holidays" and names none. The sources behind each statement here are the
ones each table's `sources` string names, with the date each was read.

## How it works

Each table carries three kinds of day.

1. **A day the instrument always gives on the same date, and a statute
   also names**, is carried by rule. Liberia's Acts, Eswatini's Act of
   1938, Sierra Leone's Christian days, Kiribati's and Fiji's Easter days,
   and the fixed days of South Sudan, Guinea-Bissau and Niger are of this
   kind: a caller gets them for any year from the table's first.
2. **A day whose date, length or existence the instrument decides** is
   tabulated for the years whose instrument was read. Each such holiday is
   a `Rule::Tabulated` whose `first_year..=last_year` covers only those
   years, or several, one per run of years read, with the years in between
   named as unread. The `announced` and `unread_in` helpers in
   `africa_middle_east.rs` build them.
3. **A year outside what was read is a gap, never a guess.** The engine
   reports the holiday in `HolidayCalendar::gaps` for that year rather
   than placing it on the day it fell the year before, and a day whose
   instrument was never read in any year, such as Sierra Leone's
   Independence Day, is a gap in every year.

No table in this document predicts a Hijri day on the tabular calendar
where an instrument decides it, with one exception: Niger's Eid al-Fitr
from 2026, which ordonnance 2026-12 makes a *fête légale* of two days, is
carried on the tabular calendar and marked approximate, as every other
country's Hijri days are. Somalia's Eids are the same kind of rule and are
in its own table's documentation, not here, because its statute names the
days.

### Worked example: Sudan's Eid al-Fitr in 2026 and 2025

The General Secretariat of the Council of Ministers announced on
13 March 2026 the holiday for Eid al-Fitr 1447. The table carries its days,
Thursday 19 to Tuesday 24 March 2026, as `sd_fitr` for 2026 alone.

1. Asked for 2026, the table gives the six days. Friday 20 and Saturday 21
   March are also the Friday–Saturday weekend, which the table keeps from
   26 January 2008; the Eid is not lengthened for them, because the
   announcement did not lengthen it.
2. Wednesday 25 March is a working day: the announcement ends the Eid on
   the 24th.
3. Asked for 2025, the table gives no Eid al-Fitr and reports it as a gap.
   The tabular Hijri calendar would put 1 Shawwal 1446 at the end of March
   2025, but how many days the Council gave that year, and from which, was
   not read, so no day is shown.

The test `sudan_carries_the_council_of_ministers_announcements_it_read`
checks each step.

## What is carried

| Table | By rule | Tabulated, for the years read | A gap | Weekend |
| --- | --- | --- | --- | --- |
| Fiji | Good Friday, Easter Saturday, Easter Monday (Cap. 101) | Every other day of the Ministry's lists, 2019–2026 | Those days in any other year | Saturday–Sunday, the days the lists move a holiday off when they move one |
| Kiribati | Good Friday, Easter Monday (Cap. 81) | Every other day of the orders for 2025 (revised) and 2026, the "in honour of" days included | Those days in any other year | Saturday–Sunday |
| Liberia | The eleven days of the Acts, Flag Day from 1916, Decoration Day from 1917; a Sunday holiday on the Monday from 2014 | — | Fast and Prayer Day and Thanksgiving in 1883 and Unification Day in 1960, the years of their Acts | Sunday, the Decent Work Act |
| The Gambia | — | Every holiday as declared, 2021–2026: New Year 2022–2026 but 2023; Independence Day 2024–2025; Good Friday and Easter Monday 2022–2026 but 2023; Labour Day 2021, 2024, 2026; Africa Day 2022 and 2025; Koriteh 2021–2026 but 2023; Tobaski 2021–2022 and 2025; Tamharit, Gamo 2021–2022 and 2025–2026; Assumption 2021–2026 but 2024; Christmas and Boxing Day 2021 and 2023–2025; the election days of 2021 and 2022 | Each holiday in every year not listed | Saturday–Sunday, the 2021 Media Advisory's |
| Sudan | — | Christmas 2025; Independence Day, both Eids, the Islamic New Year and the Prophet's Birthday 2026 | Each in every other year | Friday to 25 January 2008, Friday–Saturday from 26 January 2008 |
| Togo | — | Eid al-Fitr and Labour Day 2026; Tabaski 2025–2026; Independence Day 2025–2026; Whit Monday 2025; the days off of 2024–2026 | The other *fêtes légales* of 1987 in every year from 1987 | Sunday, the Code du travail of 2021 |
| Niger | From 2023: 1 January, 18 December, Christmas, 3 August; 26 July from 2024; 26 March and two days of Eid al-Fitr, approximate, from 2026 | Easter Monday and Tabaski 2026 | Labour Day every year; Eid al-Fitr before 2026 | Saturday–Sunday, décret 2017-682 |
| Gabon | Liberation Day from 2024 | New Year and the Ascension 2026; Pentecost, Whit Monday, Eid al-Adha and Christmas 2025; the Assumption and the Fête nationale 2024; the days off of 2025–2026 | Easter Monday, Labour Day, Eid al-Fitr, All Saints' Day every year | Sunday, the Code du travail of 2021 |
| Sierra Leone | New Year, Good Friday, Easter Monday, Christmas, Boxing Day; a Sunday holiday on the Monday | Armed Forces Day 2020 and 2023; Women's Day 2020; Eid al-Fitr 2020; Eid al-Adha 2022 | Armed Forces Day 2021–2022; Independence Day, Labour Day and the Moulid every year | Saturday–Sunday |
| Eswatini | The Schedule's days with section 2's Sunday proviso; the King's Birthday but in 2001 and 2026; 22 July to 2024 | Labour Day 2005; Umhlanga 2006 and 2025; Incwala 2007; the King's Birthday 2026, on the Friday; Lutsango Day 2025–2026 | The King's Birthday 2001; the notice days in every other year | Sunday |
| Guinea-Bissau | From 2023: New Year, 20 January, 1 May, 24 September, Christmas | Tabaski 2025; Eid al-Fitr 2026 | Easter every year; each Eid in any other year | Sunday, the Lei Geral do Trabalho |
| South Sudan | From 2022: the five single days on fixed dates | Easter 2022 and 2025–2026; Eid al-Fitr 2022 and 2025; Eid al-Adha and Christmas 2022 | Each span in every other year | Saturday–Sunday, not sourced |
| North Korea | From 2020: the wall calendar's thirteen holidays, the lunar days on `dangi` | — | — | Sunday, the Socialist Labour Law |

**North Korea** is the one table here with no tabulated day. Its
instrument, the wall calendar, was read only for 2020, in a transcription,
and it is carried by rule from 2020 on the assumption that the list has not
changed; KCNA's reports of 2026 confirm the names and the dates, not which
days are days off.

**The weekends.** Five are from a labour law read: Liberia's Decent Work
Act, Togo's and Gabon's Codes du travail ("en principe le dimanche",
article 198 of 2021 in Togo and 220 in Gabon [gabon-code-travail-2021]),
North Korea's Socialist Labour Law, and Niger's décret 2017-682, whose
article 135 requires the Saturday and Sunday off from May to September and
leaves the rest of the year to the arrangement an employer chooses
[niger-decret-2017-682]. Guinea-Bissau's Sunday is article 123 of the Lei
Geral do Trabalho, read in a summary [guinea-bissau-lgt-dcjri].
Eswatini's is the Ministry of Home Affairs' statement, as the press reports
it, that Saturday is "a normal working day". Fiji's and The Gambia's are
the days their instruments move a holiday off. Sudan's is from the press.
Sierra Leone's is inferred from notices that moved Saturday holidays.
South Sudan's is an assumption: section 59 of its Labour Act gives a weekly
rest "on such day as is customary" and section 54 of its Civil Service Act
a "standard 40 hour working week" [south-sudan-civil-service-act-2011],
and neither names a day.

**Not carried.** The one-off days of an instrument — an election, a day of
mourning, a "working holiday" — except where the table says otherwise;
the afternoons and half days; the days declared for one community only;
and every year of every table beyond the instruments read.

## Accuracy

A tabulated day is exactly the instrument's, and the tests named below
check days of each table against its instruments. What cannot be exact is the
coverage: a table is as current as its last instrument read, which for
most is of 2026 and for Sierra Leone of March 2023. A day carried by rule
is right as long as the instrument keeps giving it; where a rule stands on
a statute that the instruments depart from — Fiji's and Sierra Leone's
Schedules — only the days that every instrument read keeps are carried by
rule.

Two kinds of source fall short of the instrument. For Gabon, the
communiqués were read only as the Gabonese press reproduces them; for
Guinea-Bissau, the decree of 2023 only as the press quotes it; for
Niger's 2026 Easter Monday and Tabaski, the press alone. And North
Korea's days off rest on a transcription of a calendar not seen. Each
table's `sources` string says which of its statements rest on such a
source.

## Sources

Each table's `sources` string cites its sources in full, with the date
each was read. The instrument each table rests on, and its class:

| Table | Key | The instrument | Class |
| --- | --- | --- | --- |
| Fiji | [fiji-moi-public-holidays] | The Ministry of Information's yearly lists, 2019–2026 | Official lists, read in the Internet Archive's captures, the Ministry's site refusing the session |
| Kiribati | [kiribati-ph-orders] | The Beretitenti's orders for 2025 and 2026 | Official instruments, from president.gov.ki |
| Liberia | [liberia-mofa-proclamations] | The President's proclamations, 2012–2026 | Official, as the Ministry of Foreign Affairs' press releases give them |
| The Gambia | [gambia-op-declarations] | The Office of the President's declarations, 2021–2026 | Official, from op.gov.gm |
| Sudan | [sudan-cabinet-announcements] | The Council of Ministers' announcements, December 2025 to August 2026 | Official, from sudan.gov.sd; the weekend from the press |
| Togo | [togo-communiques] | The ministry's communiqués and the Government portal's notices, 2024–2026 | Official; the law of 1987 from the Journal officiel |
| Niger | [niger-communiques] | The ministry's communiqués and the Council of Ministers' texts, 2023–2026 | Official, as the state newspaper and news agency print them; the 2026 Easter Monday and Tabaski from the press |
| Gabon | [gabon-communiques-press] | The Ministry of Labour's communiqués, 2024–2026 | Secondary: the press's reproductions; the decree they cite not read |
| Sierra Leone | [sierra-leone-gazette-notices] | The Gazette notices of 2020–2023 | Official gazette, from gazettes.africa and the Internet Archive |
| Eswatini | [eswatini-gazette-notices] | The Gazette notices of 2001–2013 and the Government's notice of 2026 | Official gazette; Umhlanga 2025 and Lutsango Day from the press |
| Guinea-Bissau | [guinea-bissau-decree-1-2023-press] | Decree 1/2023 and the Eid decrees of 2025 and 2026 | Secondary for the decree, quoted by the press; the state news agency for 2026 |
| South Sudan | [south-sudan-mol-calendar-2022] | The Ministry of Labour's calendar of 2022 and notices of 2024–2026 | Official, from the NGO Forum's copies and mol.gov.ss |
| North Korea | [kp-wall-calendar-2020-snu] | The 2020 wall calendar | Secondary: Seoul National University's transcription; KCNA's reports of 2026 for names and dates |

The weekend sources read for this document are
[gabon-code-travail-2021], article 220; [niger-code-travail-2012],
article 114, and [niger-decret-2017-682], articles 135 and 184;
[guinea-bissau-lgt-dcjri], articles 123 and 124 as summarised, the law's
own scan not read; and [south-sudan-civil-service-act-2011], section 54.
All were read on 2026-09-26; the Niger decree in a copy whose text layer
is incomplete in places.

## Code

- `crates/hc-holiday/src/countries/oceania.rs`: `FIJI`, `KIRIBATI`.
- `crates/hc-holiday/src/countries/africa_middle_east.rs`: `LIBERIA`,
  `GAMBIA`, `SUDAN`, `TOGO`, `NIGER`, `GABON`, `SIERRA_LEONE`, `ESWATINI`,
  `GUINEA_BISSAU`, `SOUTH_SUDAN`, and the helpers `announced`,
  `unread_in`, `announced_days!` and `NOT_READ`.
- `crates/hc-holiday/src/countries/asia.rs`: `NORTH_KOREA`.
- `crates/hc-holiday/tests/countries.rs`:
  `fiji_keeps_the_days_of_the_governments_yearly_lists`,
  `fiji_reports_the_years_its_lists_do_not_cover_as_gaps`,
  `kiribati_keeps_the_days_of_the_beretitentis_orders`,
  `kiribati_reports_the_years_its_orders_were_not_read_as_gaps`,
  `liberia_keeps_its_acts_and_moves_a_sunday_holiday_to_the_monday`,
  `the_gambia_is_its_presidents_declarations_year_by_year`,
  `sudan_carries_the_council_of_ministers_announcements_it_read`,
  `togo_reports_the_laws_days_and_carries_the_communiques_read`,
  `niger_follows_the_amendments_of_2023_to_2026`,
  `gabon_carries_the_communiques_as_reproduced`,
  `sierra_leone_moves_a_sunday_holiday_to_the_monday_as_the_notices_do`,
  `eswatini_keeps_the_act_and_the_notices_read`,
  `guinea_bissau_keeps_decree_1_2023_and_the_eid_days_read`,
  `south_sudan_keeps_the_ministrys_days_and_reports_the_unread_spans`,
  `north_korea_keeps_the_2020_calendars_days_off_on_kcnas_dates`.
