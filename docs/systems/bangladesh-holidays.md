# Bangladesh's holiday notifications: the general, executive-order and optional holidays

Bangladesh's holidays are not a statute's list. Each year the Ministry of
Public Administration (জনপ্রশাসন মন্ত্রণালয়) issues a notification
(প্রজ্ঞাপন) of the year's holidays for every government, semi-government,
autonomous and statutory office, a few months before the year begins. It
gives three kinds of day: general holidays (সাধারণ ছুটি), holidays by
executive order (নির্বাহী আদেশে সরকারি ছুটি), and optional holidays
(ঐচ্ছিক ছুটি), which an employee takes on application and only for their
own religion or community. This document covers the notifications for 2025
(21 October 2024) and 2026 (9 November 2025), and what the table
`BANGLADESH` carries of them. The calendar the civil days are dated in is
`bangladeshi`'s.

## What it is

A notification has, in order, the general holidays, the executive-order
holidays and the optional holidays, and says which of them fall on a
weekly holiday, Friday or Saturday [bd-mopa-public-holiday,
bdnews24-holidays-2025, ekhon-holidays-2026].

| Kind | Days | Who keeps it |
| --- | --- | --- |
| General holiday (সাধারণ ছুটি) | 12 in 2025, 14 in 2026 | every office |
| Executive-order holiday (নির্বাহী আদেশে সরকারি ছুটি) | 14 in 2025, 14 in 2026 | every office |
| Optional holiday (ঐচ্ছিক ছুটি) | Muslim section 5, Hindu 9, Christian 8, Buddhist 7, for the small ethnic groups (ক্ষুদ্র নৃগোষ্ঠী) 2, the same in both years | the employee who applies, of that religion or community |

For the optional holidays the notification says that an employee may be
allowed at most three days of the optional holidays of their own religion in
a year ("নিজ ধর্ম অনুযায়ী অনধিক তিন দিনের ঐচ্ছিক ছুটি"), that each employee
must have the competent authority's approval for the three at the start of
the year, and that the optional holidays may be joined to the general
holidays, the executive-order holidays and the weekly holidays
[shikshabarta-holidays-2025, ajkerpatrika-holidays-2026,
nayadiganta-holidays-2026]. Offices whose hours and holidays are set by
their own law, and those whose service the Government has declared
essential, declare the holidays themselves.

## How it works

A general or executive-order holiday is a rule of `BANGLADESH` for
everyone: the civil days on their Gregorian dates, Pohela Boishakh and
Chaitra Sankranti on the Bangladeshi calendar, the Hijri days on the
tabular calendar as predictions (the notifications themselves star them as
depending on the moon), and the Hindu and Buddhist days as the notification
dates them for the two years.

An optional holiday is the same kind of rule given to the group of its
section, as Nepal's days for a faith are
([ADR 0011](../adr/0011-a-day-for-one-group-is-a-scoped-rule.md)). The
Muslim, Hindu, Christian and Buddhist sections are the groups `muslims`,
`hindus`, `christians` and `buddhists`; the section for the employees of the
small ethnic groups in the Chittagong Hill Tracts and outside them is the
group `small-ethnic-groups`. The notification's own word is ক্ষুদ্র নৃগোষ্ঠী,
"small ethnic group", not "indigenous", so the group is not
`indigenous-peoples`, which is Taiwan's. A caller who asks for the calendar
for a group gets everyone's days and the group's own; one who asks for no
group gets everyone's alone, and the library does not combine groups.

The day is [`Kind::Religious`], and the section for the small ethnic groups,
whose festival the notification calls a social one (সামাজিক উৎসব), is
[`Kind::Observance`]. Neither is a day off in the library's sense: the
employee takes the day on application, and until they do, and for every
employee who does not, it is a working day, which business-day arithmetic
counts as one. The cap of three is not computed. A group's calendar lists
every day of its section, five to nine, and which three an employee chose,
and whether their authority approved them, are theirs and the authority's.

Worked example: the Hindu section of the 2026 notification. Ajker Patrika
and Ekhon TV reproduce it; Ekhon TV's table prints the weekday beside each
day. The section has nine days: Saraswati Puja on Friday 23 January,
Shivaratri Brata on Sunday 15 February, Dolyatra on Tuesday 3 March, the
appearance of Harichand Thakur on Tuesday 17 March, Mahalaya on Saturday 10
October, the two days of Durga Puja, Saptami and Ashtami, on Sunday 18 and
Monday 19 October, Lakshmi Puja on Sunday 25 October and Shyama Puja on
Sunday 8 November [ajkerpatrika-holidays-2026, ekhon-holidays-2026]. The
section's total, nine, is the notification's. The Hindu rules are given to
`hindus`:

```rust
use hc_holiday::countries::BANGLADESH;
use hc_holiday::engine::HolidayCalendar;
use hc_holiday::rule::Scope;

let hindus = HolidayCalendar::for_year_scoped(&BANGLADESH, Scope::group("hindus"), 2026);
let everyone = HolidayCalendar::for_year(&BANGLADESH, None, 2026);
// 23 January 2026: Saraswati Puja for the Hindu group, an ordinary day,
// and the Friday weekly holiday, for everyone.
```

The Buddhist section's Chaitra Sankranti, 13 April, is listed in 2025
without an exception and in 2026 "for all districts but the three hill
ones" (তিন পার্বত্য জেলা ছাড়া অন্য সব জেলার জন্য), where the 2026
notification makes it a general holiday. The 2025 rule is for the group
everywhere; the 2026 rule is `except_in` Bandarban, Khagrachhari and
Rangamati (`BD-01`, `BD-29`, `BD-56`), whose general holiday is the table's
region-scoped rule.

## What is carried

Every optional holiday of the two notifications, for 2025 and 2026, as the
newspapers reproduce them, with two exceptions that are gaps in 2025:

| Section | Group | Days carried |
| --- | --- | --- |
| Muslim | `muslims` | Shab-e-Meraj (2026), the third day after Eid-ul-Fitr, the fourth day after Eid-ul-Azha, Akheri Chahar Shomba (2026), Fateha-e-Yazdahum |
| Hindu | `hindus` | Saraswati Puja, Shivaratri Brata, Dolyatra, the appearance of Harichand Thakur, Mahalaya, Durga Puja (Saptami and Ashtami), Lakshmi Puja, Shyama Puja |
| Christian | `christians` | English New Year's Day, Ash Wednesday, Holy Thursday, Good Friday, Holy Saturday, Easter Sunday, Christmas (the day before and the day after) |
| Buddhist | `buddhists` | Magha Purnima, Chaitra Sankranti, Buddha Purnima (the day before and the day after), Asalhi Purnima, Madhu Purnima, Probarana Purnima |
| Small ethnic groups | `small-ethnic-groups` | Boisabi and the like, 12 and 15 April |

The days are the notifications' own dates, a listing of the two years, and
a gap in any other year: no rule was fitted to them, and the notification
of 2027 is the next. The days the notifications date by a relation to
another day — the third day after Eid-ul-Fitr, the fourth after
Eid-ul-Azha, the day before and the day after Buddha Purnima, Christmas —
are carried as dates all the same, since the notifications give dates, and
the Eid days were not computed from the tabular calendar, which puts
Eid-ul-Fitr 2026 a day before the notification's.

Not carried:

* **Shab-e-Meraj and Akheri Chahar Shomba of 2025** are gaps. The lists'
  reproductions disagree on the first: 28 February in bdnews24.com, Prothom
  Alo, Shikshabarta and The Daily Campus, 28 January in Dainik Bangla and
  BVNews24. 15 February 2025 is Shab-e-Barat, 15 Sha'ban, so a 27 Rajab,
  Shab-e-Meraj, cannot be 28 February. Every reproduction prints the second
  as 20 September 2025, a Saturday, while the last Wednesday of Safar 1447
  fell in August. Either the notification has misprints the newspapers
  copied, or the newspapers do; no copy of the notification was read to
  tell. The two days are reported as gaps, not guessed. The same
  newspaper copy puts Buddha Purnima's general holiday on 21 May where its
  optional days are 10 and 12 May, and the table keeps 11 May, which the
  optional days bracket.
* **The Bengali names of the groups.** `hc_i18n::holiday_groups` carries a
  language's name for a group only from an instrument written in that
  language. The notification is in Bengali and would give them, but it was
  not read (its PDF was not opened), and the newspapers spell the Christian
  section's name two ways, খ্রিস্টান and খ্রিষ্টান. The groups are named in
  English until a reading of the notification settles the spellings.
* **Other years' notifications**, before 2025 and after 2026.
* **The sections' effect on a weekly holiday.** The notification counts the
  optional days that fall on a Friday or a Saturday (one Muslim day in 2026
  is on a Saturday), and gives nothing in their place; the table moves
  nothing.
* **Pakistan's** optional holidays for its religious minorities, the
  neighbouring table's, are not carried either; its Cabinet Division list is
  a scanned PDF.

## Accuracy

Every row of the 2026 sections is in two reproductions, Ajker Patrika and
Ekhon TV, which agree, and Ekhon TV's weekdays agree with the dates;
`crates/hc-holiday/tests/bangladesh_optional.rs` checks every weekday. The
2025 rows are in at least four reproductions that agree, except the two
above. The totals of each section agree with the notification's, five, nine,
eight, seven and two, which the tests count.

No row was read in the notification itself: the Ministry's page lists the
notifications as PDF files, which the project does not open, and prints none
of their text as HTML. The reproductions are newspapers' text, and so a
secondary source for the dates, and the table's `sources` string says so.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [bd-mopa-public-holiday] | The Ministry's page listing the notifications | Yes, 2026-10-03, as HTML; the PDFs were not opened |
| [bdnews24-holidays-2025] | The 2025 lists, the three-day sentence | Yes, 2026-10-03 |
| [prothomalo-holidays-2025], [shikshabarta-holidays-2025], [dailycampus-holidays-2025] | The 2025 lists, a check | Yes, 2026-10-03 |
| [dainikbangla-holidays-2025], [bvnews24-holidays-2025] | The 2025 lists, with Shab-e-Meraj on 28 January | Yes, 2026-10-03 |
| [ajkerpatrika-holidays-2026] | The 2026 lists and the sentences on the three days | Yes, 2026-10-03 |
| [ekhon-holidays-2026] | The 2026 lists with weekdays | Yes, 2026-10-03 |
| [nayadiganta-holidays-2026] | The 2026 optional totals and the three days | Yes, 2026-10-03 |
| The notifications of 2025 and 2026 themselves, on mopa.gov.bd | The general and executive-order days, as the table's `sources` string cites them, retrieved 2026-09-23 | Not read for the optional holidays |
| The Cabinet Division's notification of 2 July 2025 and Prothom Alo's report of it | July Mass Uprising Day | As the table's `sources` string records; not a reference of this document |

## Code

`crates/hc-holiday/src/countries/asia.rs`: `BANGLADESH`, its weekend
(Friday and Saturday), `BD_RULES` for the general and executive-order
holidays, and `BD_NOTIFIED`, the Hindu and Buddhist days of the general
holidays. `crates/hc-holiday/src/countries/bangladesh_optional.rs`: the
listing `BD_OPTIONAL`, the rules of the five sections in
`BD_OPTIONAL_RULES`, and `BD_ALL_RULES`, which joins them to `BD_RULES` and
is what `BANGLADESH` carries. The groups are `hc_holiday::group`'s
`MUSLIMS`, `HINDUS`, `CHRISTIANS`, `BUDDHISTS` and `SMALL_ETHNIC_GROUPS`.
Anchors: `crates/hc-holiday/tests/bangladesh_optional.rs` for the optional
holidays and `crates/hc-holiday/tests/countries.rs` for the rest.
