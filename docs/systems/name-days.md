# Name days

The system behind `hc-name-days`: what a name day is in the countries that
keep one, how the crate models a list, what it carries, how the carried data
was checked, and where every statement comes from. Sources are cited by key
into [`../references.bib`](../references.bib).

## What it is

A name day (Latvian *vārda diena*, Finnish *nimipäivä*, Swedish *namnsdag*,
Hungarian *névnap*, Greek *ονομαστική εορτή*) is a day of the year on which
people bearing a given name are celebrated. The custom descends from the
church calendar of saints, and in most of the countries that keep it the
list has since been taken over by a secular body, a publisher or nobody:

- **Latvia.** The Kalendārvārdu ekspertu komisija of the Valsts valodas
  centrs keeps a traditional list of about a thousand names and an extended
  list of about five and a half thousand, revises them not more often than
  once in three years, and publishes both as open data [lva-vardadienas]
  [lvportals-2014]. 29 February carries no names; 22 May is the day for
  names not in the calendar.
- **Finland.** The University of Helsinki's Almanac Office keeps four
  lists — Finnish, Finland-Swedish, Orthodox and Sámi — revised every five
  years, and holds the copyright to them [helsinki-copyright].
- **Sweden.** The Namnlängdskommittén of the Swedish Academy, the two royal
  academies and the Institute for Language and Folklore keeps the list
  [isof-namnsdagar] [svenska-akademien-namnlangden]; it has had no official
  status since 1972 [riksdagen-kru4].
- **Norway.** A private publisher, Almanakkforlaget, owns the list
  [almanakkforlaget-navnedager].
- **Slovakia.** The Ministry of Culture's Kalendárová komisia publishes an
  official but recommendatory list [culture-sk-kalendarium].
- **Czechia, Poland, Denmark, Lithuania.** Publishers' lists with no body to
  choose between them [ptejteseknihovny-kalendarium] [plwiki-imieniny]
  [dawiki-navnedag] [ltwiki-vardadienis].
- **Hungary, Estonia.** The printed lists rest on copyrighted books, and the
  claims of an official keeper are unconfirmed [huwiki-nevnap]
  [stat-ee-nimepaevad].
- **Greece, Bulgaria, Russia.** The Orthodox church calendar names saints by
  day; which given names belong to which saint is custom, and some name days
  move with Pascha [elwiki-eortologio] [bg-patriarshia-calendar]
  [ruwiki-imeniny].
- **France, Croatia, Germany and Austria, Spain.** Liturgical calendars, or
  publishers' selections from them, with no civil list [frwiki-fleuristes]
  [ktabkbih-imendanski] [dewiki-namenstage].

## How it works

**A list is 366 slots.** One per day of a leap year, 29 February included,
each holding the names its authority prints on that day, in the authority's
order. A slot with no names is a statement, not a hole, and every vendored
list's empty slots are exactly the ones its leap-day rule accounts for.

**The leap day is an enum.** `LeapDayRule` has four variants, one per
convention the survey found, and a fifth would be a modelling decision
(ADR 0007), not a discovery about a country:

| Variant | Convention | Where |
| --- | --- | --- |
| `NoNames` | 29 February carries no names | Latvia (an en dash), Finland, Norway |
| `OwnNames` | 29 February carries names of its own, kept in leap years | Czechia (Horymír), Estonia |
| `ShiftAfter24February` | 24 February is the leap day; in a leap year it is empty and the names of 24–28 February move to 25–29 | Hungary, Denmark |
| `LeapYearsOnly` | 29 February carries a name the authority states is kept only in leap years | France (Auguste Chapdelaine) |

`slot_index(rule, year, month, day)` is the whole of the arithmetic: a
valid date reads its own slot, except that under `ShiftAfter24February` a
common year's 24–28 February read the slots of 25–29 February, because the
layout is the leap year's and the leap year is the one that has moved. A
date that does not exist — 29 February of a common year included — is an
error, not a substitution.

*Worked example: Hungary's leap day.* Hungary is a gap in this crate, not a
vendored list, but its calendar is the textbook case of
`ShiftAfter24February`, and Hungarian Wikipedia's day pages state it
[huwiki-februar-24]: 24 February is Mátyás, 25 February Géza and 28 February
Elemér, and in a leap year 24 February is the *szökőnap*, the leap day,
which carries no names, while the names of 24–28 February move one day on,
Mátyás to the 25th and Elemér to the 29th ("Mátyás ugrása"). Laid out as a
leap year's 366 slots, the list therefore reads:

| Slot | Names |
| --- | --- |
| 24 February | — |
| 25 February | Mátyás |
| 26 February | Géza |
| 29 February | Elemér |

and the arithmetic, by hand:

1. **2024, a leap year, 25 February.** The date exists and the year is
   leap, so it reads its own slot, 25 February: Mátyás.
2. **2024, 24 February.** Its own slot, which is empty: nobody's name day.
3. **2025, a common year, 24 February.** Under `ShiftAfter24February` a
   common year's 24–28 February read the slot one day later, so this reads
   the 25 February slot: Mátyás, on the 24th, as in every common year.
4. **2025, 28 February.** Reads the 29 February slot: Elemér.
5. **2025, 29 February.** Does not exist: an error.

**One evaluator, two kinds of list.** `names_on(list, year, month, day)` and
`days_of(list, name, year)` read anything implementing `NameDays`: the
vendored `NameDayList` tables and the `OwnedNameDayList` a caller loads from
text. Both refuse a year the edition does not cover (ADR 0006).

**Editions are separately named.** An authority's revision is data, not a
correction (policy §10): the Latvian list decided in 2022 and the one decided
in 2025 are `lv-traditional-2023` and `lv-traditional-2026`, each with its
validity span, and `in_force(country, year)` returns every list in force in a
year — several at once, since Latvia keeps two.

**The movable Orthodox name days are not yet carried.** They are rules on the
Julian computus — Thomas Sunday (Pascha + 7) for Θωμάς and Θωμαΐς, All Saints
(Pascha + 56) for names with no saint of their own, St George moved to Easter
Monday when 23 April falls before Pascha; Bulgaria's Цветница, Великден,
Тодоровден and Спасовден — and belong beside `hc_holiday::computus` rather than
duplicated here [elwiki-eortologio] [bg-patriarshia-calendar]. `gaps::GREECE`
and `gaps::BULGARIA` record the rules until they arrive.

## What is carried

**Latvia**, four lists, all CC0-1.0 [lva-vardadienas]:

| Identifier | Names | Decided | In force | File |
| --- | --- | --- | --- | --- |
| `lv-traditional-2023` | 1,023 | 16 March 2022 [vvc-2022] | 2023–2025 | the XLSX published 2023-04-26, as captured 2024-08-12 [lva-vardadienas-2023] |
| `lv-extended-2023` | 5,589 | 16 March 2022 [vvc-2022] | 2023–2025 | the CSV published 2023-04-26, as captured 2024-08-13 [lva-vardadienas-2023] |
| `lv-traditional-2026` | 1,032 | 30 April 2025 [vvc-2025] | from 2026 | the CSV published 2025-05-16, retrieved 2026-09-25 |
| `lv-extended-2026` | 5,652 | 30 April 2025 [vvc-2025] | from 2026 | the CSV published 2026-03-16, retrieved 2026-09-25 |

What the source prints that is not a name is kept as a `Note`: the sentence
on 22 May, *Visu neparasto un kalendāros neierakstīto vārdu diena*, with
`unlisted_names_day` set to 22 May; and the six Latgalian forms the 2026
extended list prints in parentheses (`Mora (LTG: Muora)`, `Borbala (LTG:
Buorbala)`, `Borbola (LTG: Buorbola)`, `Borbula (LTG: Buorbula)`, `Joņs (LTG:
Juoņs)`, `Povuls (LTG: Puovuls)`), each carried as a name of its own with a
note naming the Latvian form it belongs to. The en dash on 29 February is the
`NoNames` rule and an empty slot.

**The loader**, `hc_name_days::load`, behind the `alloc` feature: a text
format of `key = value` lines — header fields, then one line per day of a
leap year with names separated by `;` — read into an `OwnedNameDayList` the
evaluator treats exactly like a vendored table. It exists for the lists this
crate may not ship. The crate distributes no such list and no fixture of one;
its tests use a synthetic list and the Latvian tables rendered to text and
read back.

**Not carried**, each a `gaps::Gap` with its reason as a value:

| Country | Reason | Because |
| --- | --- | --- |
| Finland | `LicensedForAFee` | The University holds the copyright, confirmed by the Supreme Court in 2000; free publication stops at two weeks or 15 names; a whole year is charged per copy [helsinki-copyright] [helsinki-pricing] [kko-2000-56] |
| Norway | `LicensedForAFee` | Almanakkforlaget owns the list; editorial use is free with credit, commercial use on its terms [almanakkforlaget-navnedager] |
| Sweden | `LicenceUnknown` | No terms stated by the committee's institutions; catalogue protection can apply [isof-namnsdagar] [svenska-akademien-namnlangden] |
| Slovakia | `LicenceUnknown` | The ministry states no terms; whether the list is an official work under Zákon č. 185/2015 Z. z. § 5 is unchecked [culture-sk-kalendarium] |
| Croatia | `LicenceUnknown` | Only Bosnia and Herzegovina's episcopal list is compiled, with no terms stated [ktabkbih-imendanski] |
| France | `LicenceUnknown` | The postal calendar's terms are not stated and Nominis's were not retrieved [frwiki-fleuristes] |
| Czechia, Poland, Denmark, Lithuania | `SourcesDisagreeWithNoAuthority` | Publishers' lists with no body to choose between them [ptejteseknihovny-kalendarium] [plwiki-imieniny] [dawiki-navnedag] [ltwiki-vardadienis] |
| Hungary, Estonia | `MethodUnpublished` | The printed lists rest on copyrighted books; the official keepers claimed are unconfirmed [huwiki-nevnap] [stat-ee-nimepaevad] |
| Greece, Bulgaria, Russia, Germany and Austria, Spain | `SaintsNotNames` | The church calendar names saints, not given names; the mapping has no authority [elwiki-eortologio] [bg-patriarshia-calendar] [ruwiki-imeniny] [dewiki-namenstage] |

## Accuracy

The Latvian tables are transcriptions, generated by a script from the
centre's files, and checked three ways:

1. **Against the centre's own announcements.** Every one of the nine names
   the 2025 decision added is asserted to be on its announced day in the
   2026 edition and absent from the 2023 edition [vvc-2025]; every one of the
   ten names the 2022 decision added is asserted on its day in both editions
   [vvc-2022]; the eight names of the 2014 revision likewise [lvportals-2014].
   A sample of the fifty-six extended additions of 2025 and of the fourteen
   of 2022 is asserted the same way. One discrepancy: the 2022 announcement
   as read gives Dafne 18 February, and both files give 8 February; the file
   is the record and the test follows it.
2. **Against the files' own structure.** Each edition has 366 rows and
   exactly one empty one, 29 February; 22 May begins with Emīlija in every
   edition; no slot repeats a name; no name contains whitespace; the
   traditional list is contained in the extended list day by day; the totals
   equal the counts taken from the files (1,023 and 1,032; 5,589 and 5,652),
   so the traditional list grew by exactly nine.
3. **Against each other.** The two editions' validity spans abut with no gap
   and no overlap, and the loader round-trips every slot of every edition.

The centre's extended file changed once within the 2026 edition: the file of
2026-03-16 has Enia on 4 March and the file of 2026-01-10 did not. The crate
carries the later file.

## Sources

| Key | Used for | Read directly |
| --- | --- | --- |
| [lva-vardadienas] | the 2026 editions, the licence, the publication dates | yes |
| [lva-vardadienas-2023] | the 2023 editions | yes, from the Wayback Machine |
| [vvc-2025] | the 2025 decision, its nine and fifty-six additions, the in-force date | yes |
| [vvc-2022] | the 2022 decision and its additions | yes |
| [lvportals-2014] | the 2014 additions, the commission's standing | no, via the research report |
| [helsinki-copyright] | Finland's terms and the free-use limit | yes |
| [helsinki-pricing] | Finland's royalty | no, via the research report |
| [kko-2000-56] | the legal basis of Finland's terms | no, via the research report |
| [almanakkforlaget-navnedager] | Norway's terms | no, via the research report |
| [isof-namnsdagar], [svenska-akademien-namnlangden], [riksdagen-kru4] | Sweden's committee, revisions and status | no, via the research report |
| [culture-sk-kalendarium] | Slovakia's list and commission | no, via the research report |
| [ptejteseknihovny-kalendarium] | Czechia | no, via the research report |
| [huwiki-nevnap], [dawiki-navnedag], [stat-ee-nimepaevad], [ltwiki-vardadienis], [plwiki-imieniny] | Hungary, Denmark, Estonia, Lithuania, Poland | no, via the research report |
| [huwiki-februar-24] | the worked example: the Hungarian names of 24, 25, 28 and 29 February and the leap-year shift | yes, 2026-09-26 |
| [bg-patriarshia-calendar], [elwiki-eortologio], [ruwiki-imeniny] | the Orthodox countries and the movable rules | no, via the research report |
| [frwiki-fleuristes], [ktabkbih-imendanski], [dewiki-namenstage] | France, Croatia, Germany and Austria | no, via the research report |

"Via the research report" means the page was read during the survey of
2026-09-25 that preceded the crate and quoted there; the crate's text
repeats the quotation and the URL.

## Code

| Module | Holds |
| --- | --- |
| `hc_name_days::list` | `NameDayList`, `LeapDayRule`, `Licence`, `Provenance`, `Validity`, `Note`, `NameDays`, `slot_index`, `names_on`, `days_of` |
| `hc_name_days::latvia` | the four editions, as a `catalogue!`, with `latvia/data.rs` generated from the files |
| `hc_name_days::load` | `OwnedNameDayList::parse`, `NameDayList::to_text`, `ParseError` |
| `hc_name_days::gaps` | seventeen `Gap`s, as a `catalogue!` |

Anchoring tests, beside the data:

- `latvia::tests::the_nine_names_of_2025_are_in_the_2026_edition_and_not_in_the_2023_one`
- `latvia::tests::the_ten_names_of_2022_and_the_eight_of_2014_are_in_both_editions`
- `latvia::tests::the_leap_day_carries_no_names_because_the_source_prints_a_dash`
- `latvia::tests::the_twenty_second_of_may_is_emilija_and_the_day_for_unlisted_names`
- `latvia::tests::the_latgalian_forms_of_2026_are_names_with_a_note_saying_whose_they_are`
- `latvia::tests::the_traditional_list_is_contained_in_the_extended_list_day_by_day`
- `latvia::tests::the_sizes_are_as_counted_from_the_files`
- `list::tests::the_bissextus_rule_moves_late_february_one_day_later_in_a_leap_year`
- `list::tests::an_edition_refuses_a_year_it_does_not_cover`
- `load::tests::every_vendored_list_survives_a_round_trip_through_the_text_format`
- `tests::every_shipped_list_may_be_redistributed`
- `tests::every_empty_slot_is_the_one_the_leap_day_rule_names`
