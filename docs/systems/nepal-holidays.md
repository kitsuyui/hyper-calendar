# Nepal's holiday notices: the days for everyone, for a community, a faith or a group, and for a place

Nepal's public holidays are not a statute's list. Each year the Ministry
of Home Affairs publishes a notice of the year's government and public
holidays in the *Nepal Rajpatra*, Part 5, dated in the Bikram Sambat,
and the notice gives some days to every office in the country and others
to a community, a faith, a group of employees or a place alone. This
document covers the notices for 2080 to 2083 BS, the days of each
section, and what the table `NEPAL` carries of them. The calendars the
days are dated in are [nepal-calendars.md](nepal-calendars.md)'s.

## What it is

A notice has numbered sections, each with a heading that says whom its
days are for. The four read have the same sections, with small changes of
wording [np-moha-holidays-2080, np-moha-holidays-2081,
np-moha-holidays-2082, np-moha-holidays-2083]:

| Section | Heading | For |
| --- | --- | --- |
| 2.1 | the public festival holidays for every office in the country | everyone |
| 2.2 | सम्बन्धित धर्म, संस्कृति, भौगोलिक क्षेत्र र स्थान विशेषका लागि मात्र हुने सार्वजनिक पर्व बिदा | one religion, culture, region or place |
| 3 | महिला कर्मचारीका लागि मात्र | women employees |
| 4 | शिक्षण संस्थाका लागि मात्र | educational institutions |
| 5 | जात्रा बिदा (काठमाडौं उपत्यकालाई मात्र) | the Kathmandu Valley |
| 6.1 | मुलुकभरि सबैलाई हुने सार्वजनिक दिवस बिदा | everyone |
| 6.2 | अपाङ्गता भएका कर्मचारीको लागि मात्र | employees with disabilities |
| 7.1 | the days of the faiths for everyone | everyone |
| 7.2 | सम्बन्धित धर्मावलम्बीहरूका लागि मात्र | the faithful of one religion |
| 8 | राष्ट्रियरूपमा मनाइने तर कार्यालय खुल्ने प्रकृतिका दिवस | observed, the offices open |
| 9 | the missions abroad | up to 18 days each mission arranges |
| 10, 11 | the services that work on holidays, and office hours | — |

The notes (द्रष्टव्य) let a province declare up to six days a year, which
bind the federal offices in it as well, and define a half day (आधा दिन)
as until 1:30 pm, or 1:00 pm from Kārtika 16 to Māgha 15; no item of the
four notices is given as a half day. International Women's Day is in
section 6.1, for everyone, in all four.

## How it works

A day of section 2.1, 6.1 or 7.1 is a rule of `NEPAL` for everyone. A day
of another section is the same kind of rule scoped to whom the section
names: a group of people of `hc_holiday::group`
([ADR 0011](../adr/0011-a-day-for-one-group-is-a-scoped-rule.md)), or a
kind. So Teej is a rule given to `women`, and a caller who asks for
Nepal's calendar for women gets everyone's days and Teej, and one who
asks for no group gets everyone's alone.

Worked example: Haritalika Teej in 2082 BS. Section 3 (क) of the notice
for 2082 prints "हरितालिका (तीज) व्रत — भदौ १० गते मङ्गलबार". Bhadra 10,
2082 BS is 26 August 2025 in the gazetted Bikram Sambat, a Tuesday, as the
notice prints. The table's listing has the row `(2025, 8, 26, "teej")`,
and the rule reading it is given to `women`:

```rust
use hc_holiday::countries::NEPAL;
use hc_holiday::engine::HolidayCalendar;
use hc_holiday::rule::Scope;

let women = HolidayCalendar::for_year_scoped(&NEPAL, Scope::group("women"), 2025);
let everyone = HolidayCalendar::for_year(&NEPAL, None, 2025);
// 26 August 2025: Teej for women, an ordinary day for everyone else.
```

A day each notice dates the same way — Falgunanda Jayanti on Kārtika 25,
the International Day of Persons with Disabilities on 3 December, the
section 8 days on their Bikram Sambat dates — is a rule from 2023, the
first year of the notice for 2080 BS, and a gap in every year before,
whose notices were not read. A day on a tithi — Teej, Jitiya, Gai Jatra,
Basanta Panchami — is the notices' own dates, and a gap in a year before
or after them, because no rule was fitted to the four years and the days
of 2084 BS are the next notice's. Dura Mhaipru Nakuma, first listed for
2083 BS, is absent from 2023 to 2025, whose notices were read and do not
list it, and a gap before.

## What is carried

Every item of every section outside 2.1, 6.1 and 7.1, with the dates the
notices print (BS, then Gregorian; each printed weekday agrees):

| Item | Day | For | 2080 | 2081 | 2082 | 2083 | Carried as |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 2.2 | गाईजात्रा, Gai Jatra | देशभरका नेवार समुदायका लागि मात्र | भदौ १४, 2023-08-31 | भदौ ४, 2024-08-20 | साउन २५, 2025-08-10 | भदौ १३, 2026-08-29 | `newar`, 2023–2026, a gap before and after |
| 2.2 | गौरा पर्व, Gaura Parva | no one printed | भदौ ७ | भदौ १० | भदौ १५ | भदौ १९ | not carried: whom the day is for is not printed |
| 2.2 | दुरा म्हैप्रु नकुमा | देशभरका दुरा समुदायका लागि मात्र | — | — | — | पुस १५, 2026-12-30 | `dura`, 2026, a gap after and before 2023 |
| 2.2 | सिरुवा पावनीको दिन | झापा, मोरङ, सुनसरी, सिराहा र सप्तरी जिल्लामा | no date | no date | no date | no date | not carried: five districts, no date |
| 3 (क) | हरितालिका (तीज) व्रत | women employees | असोज १, 2023-09-18 | भदौ २१, 2024-09-06 | भदौ १०, 2025-08-26 | भदौ २९, 2026-09-14 | `women`, 2023–2026, a gap before and after |
| 3 (ख) | जितिया पर्व | जितिया पर्व मनाउने महिला कर्मचारीको लागि | असोज २०, 2023-10-07 | असोज ९, 2024-09-25 | भदौ ३०, 2025-09-15 | असोज १८, 2026-10-04 | `women`, 2023–2026, a gap before and after |
| 4 | वसन्त पञ्चमी, Basanta Panchami | the educational institutions | फागुन २, 2024-02-14 | माघ २१, 2025-02-03 | माघ ९, 2026-01-23 | माघ २८, 2027-02-11 | everyone, `school`, 2024–2027, a gap before and after |
| 5 | गाईजात्रा, इन्द्रजात्रा, भोटो जात्रा, घोडेजात्रा | the Kathmandu Valley | printed | printed | printed | printed | not carried: see below |
| 6.2 | अन्तर्राष्ट्रिय अपाङ्गता दिवस (डिसेम्बर ३) | employees with disabilities | 2023-12-03 | 2024-12-03 | 2025-12-03 | 2026-12-03 | `persons-with-disabilities`, 3 December from 2023, a gap before |
| 7.2 (क) | फाल्गुनन्द जयन्ती | किराँत धर्मावलम्बी | कात्तिक २५, 2023-11-11 | कात्तिक २५, 2024-11-10 | कात्तिक २५, 2025-11-11 | कात्तिक २५, 2026-11-11 | `kirat`, Kārtika 25 from 2023, a gap before |
| 7.2 (ख) | मोहम्मद जयन्तीका दिन | नेपाली मुस्लिम धर्मावलम्बी | no date | no date | no date | no date | `muslims`, 12 Rabīʿ al-Awwal on the tabular calendar from 2023, approximate, a gap before |
| 7.2 (ग) | गुरु नानक जयन्ती | नेपाली सिख धर्मावलम्बी | मङ्सिर ११, 2023-11-27 | कात्तिक ३०, 2024-11-15 | no date | no date | `sikhs`, the full moon of Kārtika from 2023, approximate, a gap before |
| 8 | जातीय भेदभाव तथा छुवाछुत उन्मूलन राष्ट्रिय दिवस | offices open | जेठ २१ | जेठ २१ | जेठ २१ | जेठ २१ | everyone, `observance`, Jyeṣṭha 21 from 2023, a gap before |
| 8 | निजामती सेवा दिवस, Civil Service Day | offices open | भदौ २२ | भदौ २२ | भदौ २२ | भदौ २२ | everyone, `observance`, Bhadra 22 from 2023, a gap before |
| 8 | जेनजी सहिद दिवस, Gen Z Martyrs' Day | offices open | — | — | — | भदौ २३, 2026-09-08 | everyone, `observance`, Bhadra 23 from 2026 |
| 8, then 6.1 | सहिद दिवस, Martyrs' Day | offices open in 2080; a day off for everyone in 2081–2083 | माघ १६, 2024-01-30, offices open | माघ १६, 2025-01-29 | माघ १६, 2026-01-30 | माघ १६, 2027-01-30 | everyone: `observance` in 2024, a day off in the other years |

Martyrs' Day is the one day whose section changed. The notice for 2080
lists it in section 8, kept with the offices open, and those for 2081 to
2083 in section 6.1, a day off for everyone, which the press's copies of
the lists bear out: Nagarik News prints Māgha 16 of 2080 under "राष्ट्रिय
रूपमा मनाइने तर कार्यालय खुला रहने" [nagarik-bida-2080], and Nepal Press
the 2081 day under "मुलुकभरि सबैलाई हुने सार्वजनिक दिवस बिदा"
[nepalpress-bida-2081]. So 30 January 2024 is an observance and a business
day, and Māgha 16 of every other year a day off; the years before 2080,
whose notices were not read, are carried as a day off as the other days
of section 6.1 are.

Guru Nanak Jayanti is `hindu::GURU_NANAK_JAYANTI`, Kārtika pūrṇimā at
midday, which gives both printed dates and 5 November 2025 and
24 November 2026 for the notices that give "the day of" it. The Prophet's
birthday is on no date in any notice and is a prediction, as Nepal's two
Eids are.

**Not carried, and why:**

- **The places.** Section 5's four jātrās are for the Kathmandu Valley,
  Siruwa Pawani for five named districts, and section 2.1's Fagu Purnima
  is two days, the fifty-six hill and mountain districts one day and the
  twenty-one Terai districts the next. A subdivision is an ISO 3166-2
  code, and ISO 3166-2:NP codes the seven provinces (`NP-P1` to `NP-P7`
  are the codes CLDR 48 counts as regular) and no district. The Valley's
  three districts are part of Bagmati, `NP-P3`, not the whole of it;
  Siruwa Pawani's are in two provinces, and the Terai's in five. No code
  says any of them, and no invented code is carried.
- **Gaura Parva**, which section 2.2 lists with no community or place
  beside it, so that whom the notice gives it to is not printed.
- **Bhoto Jatra and Siruwa Pawani's dates**: each notice gives "the day
  the jātrā is held" and no date.
- **The provinces' days** under the notes, which are each province's own
  instrument, and the missions' days of section 9: none was read.

## Accuracy

The dates are the notices' own, turned into the Gregorian calendar by
`bikram-sambat` as gazetted; each weekday the notice prints beside a
date agrees with the one `bikram-sambat` gives, which is a check on the
turning and on the reading. `crates/hc-holiday/tests/nepal_sections.rs`
holds every dated item of the table above, with its printed weekday, and
checks that the calendar for the item's group has it and the calendar
for everyone does not.

The notices are scanned pages in Nepali, read in the Ministry's viewer.
Where a character was unclear at the normal zoom it was read again at
200 %; the notice for 2080 prints Saptari as "ससरी" in the district list
of Siruwa Pawani and of Fagu Purnima, which is kept as a misprint of
सप्तरी.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [np-moha-holidays] | The listing of the notices | Yes, 2026-09-25 |
| [np-moha-holidays-2080] | The notice for 2080 BS, sections 2.2 to 11 and the notes | Yes, 2026-09-29, the scanned PDF in the Ministry's viewer |
| [np-moha-holidays-2081] | The notice for 2081 BS, the same, and its सूचना २ | Yes, 2026-09-29, the same |
| [np-moha-holidays-2082] | The notice for 2082 BS, the same | Yes, 2026-09-29, the same |
| [np-moha-holidays-2083] | The notice for 2083 BS, the same | Yes, 2026-09-29, the same |
| [nagarik-bida-2080] | Martyrs' Day of 2080 BS kept with the offices open, a secondary check on the notice | Yes, 2026-09-29 |
| [nepalpress-bida-2081] | Martyrs' Day of 2081 BS a day off for everyone, the same | Yes, 2026-09-29 |

## Code

`crates/hc-holiday/src/countries/nepal_sections.rs`: the listing
`NP_NOTICE_DAYS`, the rules of the sections beyond 2.1 in
`NP_SECTION_RULES`, and `NP_ALL_RULES`, which joins them to `NP_RULES` in
`asia.rs` and is what `NEPAL` carries. The groups are
`hc_holiday::group`'s `NEWAR`, `DURA`, `WOMEN`, `PERSONS_WITH_DISABILITIES`,
`KIRAT`, `MUSLIMS` and `SIKHS`, and their Nepali names, as the notices
write them, are `hc_i18n::holiday_groups`'s. Anchors:
`crates/hc-holiday/tests/nepal_sections.rs`.
