# The Gregorian reform, country by country

Backs the twelve identifiers `julian-gregorian-<polity>` in
`hc-calendars-solar`, and the Swedish exception `swedish-1700` beside them.

## What it is

The Julian calendar, in force in Europe since 45 BCE, makes every fourth
year a leap year and so runs a mean year of 365.25 days, about eleven
minutes longer than the tropical year; by the sixteenth century the
equinox had moved ten days from where the Council of Nicaea had known it.
The bull *Inter gravissimas* of Gregory XIII, dated 24 February 1582,
corrected both the accumulated error and the rule [inter-gravissimas]. Ten
days were removed from October 1582, "from the third day before the Nones
up to the day before the Ides, inclusive" — 5 to 14 October — so that the
day after the feast of St Francis, Thursday 4 October, was called Friday
15 October; and from then on the centennial years would be leap years
only when divisible by 400, so that 1700, 1800 and 1900 were common and
2000 was leap. The mean year became 365.2425 days
[wikipedia-gregorian-calendar]. The bull kept the cycle of dominical
letters, which is to say the week ran on unbroken: no weekday was dropped
with the dates. The reform was the work of Aloysius Lilius, presented by
his brother Antonio; its purpose was Easter, and the bull's new table of epacts is the
part that mattered most to the Church and least to a civil calendar.

A papal bull binds Catholic states and nobody else, and so the reform
spread by the decision of each polity over the next three and a half
centuries: the Catholic monarchies in 1582 and 1583, the Protestant
states of the Empire and of Scandinavia in 1700, Britain in 1752, Sweden
in 1753, and the Orthodox states of eastern Europe only after 1916. Each
change dropped the days the Julian calendar had by then accumulated — ten
in the sixteenth century, eleven from 1 March 1700, twelve from 1 March
1800, thirteen from 1 March 1900 — and each left a gap of dates that never
existed in that place. For as long as the two calendars ran side by side
a date needed a country: the same day was 25 October 1917 in Petrograd
and 7 November 1917 in London, and historians mark the difference with
*Old Style* (O.S.) and *New Style* (N.S.). That is why this library has no
one "historical" calendar but one calendar per cut-over, as
[policy.md §5](../policy.md) requires.

## How it works

### The model

A reform calendar is one number: the fixed day *c* on which the Gregorian
reckoning took effect in a polity. Three things follow, and all three are
what the module does:

1. a date label is read as Julian if the Julian day it names is before
   *c*, and as Gregorian if the Gregorian day it names is on or after *c*;
2. a label that satisfies neither — a Julian label on or after *c*, or a
   Gregorian label before it — names a day that was skipped, and is
   refused rather than converted;
3. the fixed day counts straight through, so the weekday cycle is
   unbroken across the gap: the day after Wednesday 2 September 1752 in
   Britain was Thursday 14 September.

The number of dates dropped is the Julian calendar's accumulated excess at
the moment of the reform, and the module computes it as the gap between
the last Julian date and the first Gregorian one read as a Julian label.
The polity's year-start convention — Lady Day in England until the same
Act that changed the calendar — is a separate axis, carried by
`year_style`, and the reform calendar always counts the year from
1 January.

### The twelve cut-overs

Each row is one identifier. "Dropped" is the number of dates that never
existed; the weekdays are the module's, and the day of the week is
continuous in every row. The instrument column names the decree or act
behind the date and the Sources section says which were read.

| Identifier | Polity, as the module names it | Last Julian date | First Gregorian date | Dropped | Instrument |
| --- | --- | --- | --- | --- | --- |
| `julian-gregorian-catholic` | Papal States, Spain, Portugal, Poland-Lithuania | Thursday 4 October 1582 | Friday 15 October 1582 | 10 | *Inter gravissimas*, 24 February 1582 [inter-gravissimas]; that Spain, Portugal and Poland-Lithuania changed on the bull's own day rests on secondary sources [wikipedia-adoption-list], which note local resistance in Poland |
| `julian-gregorian-fr` | France | Sunday 9 December 1582 | Monday 20 December 1582 | 10 | An edict of Henri III, not read; the dates are from secondary sources [frwiki-passage-gregorien, wikipedia-adoption-gregorian], which exclude Alsace and Lorraine (Lorraine changed in 1760) |
| `julian-gregorian-nl` | Holland and Zeeland | Friday 14 December 1582 | Saturday 25 December 1582 | 10 | No instrument read. The date is Zeeland's and the southern provinces' in every source; for Holland the secondary sources disagree, see below |
| `julian-gregorian-de-catholic` | Catholic Germany (Bavaria) | Saturday 5 October 1583 | Sunday 16 October 1583 | 10 | A ducal order, not read; the dates from a secondary source [dewiki-gregorianischer-kalender]. Bavaria only: the other Catholic territories of the Empire changed on dates of their own |
| `julian-gregorian-hu` | Hungary | Saturday 21 October 1587 | Sunday 1 November 1587 | 10 | A law of the diet of 1587/88, not read; the dates from secondary sources [wikipedia-adoption-list, frwiki-passage-gregorien]; the Hungarian encyclopaedia's own dating is less definite [huwiki-gergely-naptar] |
| `julian-gregorian-de-protestant` | Protestant Germany, Denmark and Norway | Sunday 18 February 1700 | Monday 1 March 1700 | 11 | For the Empire, the resolution of the *Corpus Evangelicorum* at the Regensburg diet of 1699 introducing the *Verbesserter Reichskalender*; for Denmark–Norway, a royal ordinance prepared by Ole Rømer. Neither read; the dates from secondary sources [dewiki-gregorianischer-kalender, dawiki-gregorianske-kalender] |
| `julian-gregorian-gb` | Great Britain and its colonies | Wednesday 2 September 1752 | Thursday 14 September 1752 | 11 | Calendar (New Style) Act 1750, 24 Geo. II c. 23 [uk-calendar-act-1750], read: the day after 2 September 1752 "shall be called, reckoned, and accounted to be the fourteenth day of September", the centennial rule, the year from 1 January 1752, and the Act's reach over all the Crown's dominions |
| `julian-gregorian-se` | Sweden and Finland | Wednesday 17 February 1753 | Thursday 1 March 1753 | 11 | The Swedish decision of 1753, not read; the dates from secondary sources [hogman-tiderakning, wikipedia-swedish-calendar]. Finland was then part of Sweden and changed with it |
| `julian-gregorian-bg` | Bulgaria | Thursday 31 March 1916 | Friday 14 April 1916 | 13 | Decree No. 8 of Tsar Ferdinand, State Gazette no. 65 of 21 March 1916, not read; the dates from a secondary source [bgwiki-grigorianski-kalendar] |
| `julian-gregorian-ru` | Soviet Russia | Wednesday 31 January 1918 | Thursday 14 February 1918 | 13 | The Sovnarkom decree of 24 January (6 February) 1918 on the introduction of the Western European calendar, read in transcription [sovnarkom-calendar-decree-1918]: the day following 31 January Old Style "shall be counted as 14 February" New Style, with both dates to be written on documents until the end of June 1918 |
| `julian-gregorian-ro` | Romania and Serbia | Sunday 31 March 1919 | Monday 14 April 1919 | 13 | For Romania, the decree-law of 5/18 March 1919, not read; the dates from a secondary source [rowiki-calendarul-gregorian], for the Old Kingdom only. **For Serbia the row is wrong**, see below |
| `julian-gregorian-gr` | Greece | Wednesday 15 February 1923 | Thursday 1 March 1923 | 13 | A royal decree under which 16 February 1923 was reckoned as 1 March 1923, not read; the dates from secondary sources [elwiki-gregoriano-imerologio, wikipedia-adoption-list], which except Mount Athos. The Church of Greece went instead to the Revised Julian calendar in 1924 |

Two rows carry a name the sources do not support:

- **Holland.** The Dutch encyclopaedia and the English list of adoption
  dates put Holland's change at 1 January followed by 12 January 1583,
  and only Zeeland and the southern provinces at 14 followed by
  25 December 1582 [nlwiki-gregoriaanse-kalender,
  wikipedia-adoption-list]; the French encyclopaedia and the English
  narrative article put Holland and Zeeland together in December
  [frwiki-passage-gregorien, wikipedia-adoption-gregorian]. No primary
  source was read. The identifier's date is right for Zeeland, and for
  Holland it rests on the weaker pair of secondary sources.
- **Serbia.** The Kingdom of Serbs, Croats and Slovenes changed by a law
  of 10 January 1919, published in the first number of its official
  gazette, under which 15 January 1919 Old Style was reckoned as
  28 January 1919: the last Julian date was Monday 14 January and the
  first Gregorian one Tuesday 28 January [srwiki-gregorijanski-kalendar,
  wikipedia-adoption-list]. That is ten weeks before Romania's change, and
  the identifier `julian-gregorian-ro` gives every Serbian date between
  28 January and 13 April 1919 a label nobody in Belgrade wrote. The
  region string "Romania and Serbia" is unsupported by any source read,
  and a `julian-gregorian-rs` of its own is the fix; until then a caller
  can build it with `ReformCalendar::with_cutover` and the fixed day of
  28 January 1919, RD 700 562.

### Worked example: a British date across 1752

The Act of 1750 says the natural day after 2 September 1752 is to be
called 14 September [uk-calendar-act-1750]. The fixed day of a Julian date
*y*-*m*-*d* is

    −2 + 365 (y − 1) + ⌊(y − 1) / 4⌋ + ⌊(367 m − 362) / 12⌋ + a + d,

where *a* is 0 in January and February, −1 after February in a leap year
and −2 otherwise, and the Julian epoch, 1 January AD 1, is RD −1
[reingold2018code, `fixed-from-julian`]. For 2 September 1752: 365 × 1751
= 639 115; ⌊1751 / 4⌋ = 437; ⌊(367 × 9 − 362) / 12⌋ = ⌊2941 / 12⌋ = 245;
1752 is a Julian leap year, so *a* = −1; and −2 + 639 115 + 437 + 245 − 1
+ 2 = **639 796**. 639 796 = 7 × 91 399 + 3, and RD 0 is a Sunday, so it
is a Wednesday.

The fixed day of a Gregorian date is the same sum with the century terms
added and no −2:

    365 (y − 1) + ⌊(y − 1) / 4⌋ − ⌊(y − 1) / 100⌋ + ⌊(y − 1) / 400⌋ + ⌊(367 m − 362) / 12⌋ + a + d.

For 14 September 1752: 639 115 + 437 − 17 + 4 + 245 − 1 + 14 = **639 797**,
a Thursday, and one day after 639 796. So the module's `julian-gregorian-gb`,
whose cut-over *c* is 639 797, reads the label 1752-09-02 as Julian (its
Julian day 639 796 is before *c*) and the label 1752-09-14 as Gregorian,
and the two days are consecutive, Wednesday then Thursday, as the test
`the_weekday_cycle_is_unbroken_across_every_gap` holds. The label
1752-09-10 names Julian day 639 804, which is not before *c*, and
Gregorian day 639 793, which is not on or after it, so it is refused: one
of the eleven dates that Britain never had. The year 1752 had 355 days
and its September 19, which `the_reform_year_is_short_by_the_days_it_skipped`
asserts.

The same day carried a different label elsewhere: 639 796 was
13 September 1752 in France, which had been Gregorian for 170 years, and
the Act's other change, the year beginning on 1 January rather than
25 March, is the one that makes "12 February 1721" in an English source
mean the day this library calls 1722-02-12; that is `year_style`'s
business, not this calendar's.

### The Swedish exception

Sweden did not drop its days at once. In November 1699 it resolved to omit
the eleven Julian leap days from 1700 to 1740 one by one, and took the
first step: 29 February 1700 was left out, so that from 1 March 1700 the
Swedish date was one day ahead of the Julian and ten behind the Gregorian.
No further step was taken — the leap days of 1704 and 1708 were kept, in
the middle of the Great Northern War — and in January 1711 Charles XII
ordered a return to the Julian calendar, which was done by giving February
1712 thirty days: Friday 30 February 1712, the *tillökningsdag*, was
Julian 29 February and Gregorian 11 March, and the next day was Julian
1 March 1712 [hogman-tiderakning, wikipedia-swedish-calendar]. Sweden then
went Gregorian in one step in 1753, Wednesday 17 February being followed
by Thursday 1 March.

So Sweden had three calendars in sixty years, and the library carries two
identifiers for them. `swedish-1700` is the twelve years from 1 March 1700
to 30 February 1712, the Julian calendar with every label a day early plus
the one date no other calendar has, bounded to exactly those days.
`julian-gregorian-se` is the 1753 cut-over, and is Julian before it, which
is right for every Swedish day outside those twelve years and wrong by one
day for every day inside them; the test
`every_day_is_one_ahead_of_julian_and_ten_behind_gregorian` in `swedish`
asserts the disagreement day by day. The two are separate because a
calendar that was kept for twelve years and abandoned is not a phase of
the reform but a calendar of its own, and because a `julian-gregorian-se`
that knew about 1700–1712 would be a calendar with two cut-overs and a
gap in the middle, which is not what the identifier claims to be.

## What is carried

- **Identifiers**, all in `hc-calendars-solar`, all with the date as
  `ReformDate { year, month, day }` and the era `OS` before the cut-over
  and `NS` from it: the twelve of the table, as `ADOPTIONS`, each an
  `Adoption { id, region, last_julian, first_gregorian }` from which the
  cut-over and the dropped days are computed rather than stored. The
  default `ReformCalendar` is the Catholic one, the reform itself.
- **Any other cut-over** through `ReformCalendar::with_cutover`, which
  takes an identifier, a region and the fixed day of the first Gregorian
  date. That is how a polity not in the table is reached, and the test
  `a_custom_cutover_works_for_polities_not_in_the_table` does it for
  Alaska.
- **Range** the whole of `julian`'s and `gregorian`'s: years −9 999 999
  to 9 999 999, Julian before the cut-over and Gregorian after, both
  proleptic. Before 45 BCE the Julian calendar did not exist and before
  1582 the Gregorian did not; the identifiers make no claim about what
  was kept, only about which arithmetic a label belongs to.
- **A leap year** is one in which 29 February was written under whichever
  calendar was in force that day: Julian 1700 is leap in Russia and not in
  Bavaria, and a polity that dropped the last days of February 1700 —
  Protestant Germany — has no 29 February that year at all.
- **Not carried, and why.**
  - Serbia and Holland at their own dates, for the reason given above:
    the table's rows for them are wrong or doubtful, and a fix is a
    change to the code.
  - The other Dutch provinces, which changed one by one — Gelderland on
    12 July 1700, Utrecht and Overijssel on 12 December 1700, Friesland
    and Groningen on 12 January 1701, Drenthe on 12 May 1701
    [nlwiki-gregoriaanse-kalender] — and the other territories of the
    Empire, Catholic and Protestant, each on its own day; Lorraine in
    1760 [frwiki-passage-gregorien]; Transylvania in 1590 and Bukovina in
    1773 [rowiki-calendarul-gregorian]; Mount Athos, which never changed.
    The table is deliberately twelve rows, the ones a reader of European
    and Russian sources meets; every other polity is a fixed day away
    through `with_cutover`, and a row is added when a source for it has
    been read.
  - **Alaska**, which on its transfer to the United States changed
    calendar and side of the date line in one act, so that Friday
    6 October 1867 was followed by Friday 18 October and a weekday
    repeated [wikipedia-alaska-purchase]. The eleven-day calendar half is
    what `with_cutover` models; the date-line half is a time-zone matter,
    and a reform calendar cannot say that two consecutive days were both
    Fridays.
  - **Turkey**, whose change was not from the Julian calendar: the
    Ottoman Rumi calendar took the Gregorian month lengths on 1 March
    1917 and the Republic replaced the Rumi year by the Gregorian era
    under Law No. 698 of 26 December 1925, from 1 January 1926
    [trwiki-miladi-takvim]. That is the `rumi` calendar's story and it is
    carried there.
  - **Japan**, which went to the Gregorian calendar on 1 January 1873
    (Meiji 6) from a lunisolar calendar, not a Julian one
    [wikipedia-ja-gregorio-reki]; the last lunisolar day is where
    `japanese-tenpo` ends. The proclamation's number and date were not
    confirmed from any page reachable for this document. **China** in
    1912 and **Egypt** in 1875 likewise changed from a lunisolar and a
    Coptic fiscal calendar respectively [wikipedia-adoption-list], and
    are not Julian cut-overs.
  - The Orthodox churches' Revised Julian calendar of 1923, which is
    `revised-julian`; the year-start conventions, which are
    `year_style`; and the Swedish years 1700–1712, which are
    `swedish-1700`.

## Accuracy

Exact where the table is right: both calendars are integer arithmetic and
the cut-over is a single comparison. What the tests check is that the
table is internally consistent and that the published facts about the
cut-overs are reproduced.

| Measure | Result | Test |
| --- | --- | --- |
| In every row, the first Gregorian date is the day after the last Julian one | 12 of 12 | `every_tabulated_adoption_is_internally_consistent` |
| Dates dropped: 10 in 1582, 11 in 1752, 13 in 1918 and 1923 | As stated | `the_reform_skipped_ten_days_and_later_adopters_skipped_more` |
| Wednesday 2 September 1752 followed by Thursday 14 September 1752 [uk-calendar-act-1750] | Reproduced | `the_weekday_cycle_is_unbroken_across_every_gap` |
| 3 to 13 September 1752 refused in Britain; the 2nd and the 14th accepted | As stated | `dates_in_the_gap_never_existed` |
| 25 October 1917 O.S. in Russia is 7 November 1917 in Britain | Reproduced | `the_same_label_means_different_days_in_different_countries` |
| Thursday 4 October 1582 followed by Friday 15 October 1582 [inter-gravissimas] | Reproduced | `each_side_of_the_cutover_uses_the_right_calendar`, `the_default_calendar_is_the_reform_itself` |
| 1582 of 355 days with an October of 21 in Catholic Europe; 1752 of 355 days with a September of 19 in Britain | As stated | `the_reform_year_is_short_by_the_days_it_skipped` |
| 1700 leap in Britain, common in Catholic Europe, without a 29 February in Protestant Germany; 1900 leap and 2100 common in Russia | As stated | `a_leap_year_is_one_in_which_29_february_was_written` |
| Round trips over 2 400 days around each of the twelve cut-overs, and over a million days for Britain | All | `every_day_around_every_cutover_round_trips`, `a_wide_range_round_trips_for_the_british_calendar` |
| Alaska, 6 October followed by 18 October 1867, through `with_cutover` | Reproduced | `a_custom_cutover_works_for_polities_not_in_the_table` |
| The Swedish calendar a day ahead of `julian-gregorian-se` on every one of its 4 384 days | 4 384 of 4 384 | `every_day_is_one_ahead_of_julian_and_ten_behind_gregorian`, in `swedish` |

What the tests do not check is the table itself against its sources: no
test names a decree. Of the twelve rows, three rest on a document read
for this document — the bull, the British Act and the Soviet decree — and
nine on secondary sources, as the table above says row by row; one of
those nine, Serbia, is contradicted by them. The module's own statement
of its sources, that the dates are "as summarised by Reingold and
Dershowitz, *Calendrical Calculations* (4th ed.), appendix, and by the
*Explanatory Supplement to the Astronomical Almanac* (3rd ed., 2013),
§15.3", could not be confirmed: neither book was read, and the published
code of *Calendrical Calculations* carries no table of adoption dates
[reingold2018code].

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [inter-gravissimas] | The bull's date, the ten days removed, 4 October followed by 15 October, the centennial rule, the dominical letters kept, Lilius | Yes, 2026-09-25, in the Wikisource translation |
| [uk-calendar-act-1750] | The British cut-over, the year from 1 January 1752, the centennial rule, the Act's reach | Yes, 2026-09-25 |
| [sovnarkom-calendar-decree-1918] | The Russian cut-over and the double dating until 1 July 1918 | Yes, 2026-09-25, in the transcription of the 1957 edition of the decrees |
| [hogman-tiderakning] | Sweden's decision of 1699, the leap day of 1700 omitted, the reversal of 1711, 30 February 1712, and 17 February followed by 1 March 1753; Finland with Sweden | Yes, 2026-09-25 |
| [wikipedia-swedish-calendar] | The leap days of 1704 and 1708 kept; 30 February 1712 = 29 February Julian = 11 March Gregorian; the 1753 change; the sources it cites, Hildebrand 1882 and Lamont 1920, not read | Yes, 2026-09-25 |
| [wikipedia-gregorian-calendar] | The mean year of 365.2425 days; the adoption dates in summary | Yes, 2026-09-25 |
| [wikipedia-adoption-gregorian] | The narrative of adoption: France, Holland and Zeeland, Britain, Sweden | Yes, 2026-09-25 |
| [wikipedia-adoption-list] | The per-country table: Spain, Portugal and Poland; Holland and Zeeland separately; Hungary; Denmark–Norway; Serbia; Greece; Turkey 1917; Japan; China; Alaska; Egypt | Yes, 2026-09-25 |
| [wikipedia-alaska-purchase] | Alaska's transfer on 18 October 1867 with the change of calendar | Yes, 2026-09-25 |
| [dewiki-gregorianischer-kalender] | Bavaria 1583 by ducal order; the *Corpus Evangelicorum* and the *Verbesserter Reichskalender* of 1700; Denmark with them | Yes, 2026-09-25 |
| [dawiki-gregorianske-kalender] | Denmark: 18 February followed by 1 March 1700, prepared by Rømer | Yes, 2026-09-25 |
| [frwiki-passage-gregorien] | France under Henri III; Holland and Zeeland; Hungary; Lorraine 1760 | Yes, 2026-09-25 |
| [nlwiki-gregoriaanse-kalender] | Holland on 12 January 1583 and Zeeland on 25 December 1582; the other provinces 1700–1701 | Yes, 2026-09-25 |
| [huwiki-gergely-naptar] | The diet of 1587/88 | Yes, 2026-09-25 |
| [bgwiki-grigorianski-kalendar] | Decree No. 8 of 1916 and the gazette; 31 March followed by 14 April 1916 | Yes, 2026-09-25 |
| [rowiki-calendarul-gregorian] | The decree-law of 5/18 March 1919; 31 March followed by 14 April 1919 in the Old Kingdom; Transylvania and Bukovina | Yes, 2026-09-25 |
| [srwiki-gregorijanski-kalendar] | The law of 10 January 1919 and its gazette; 15 January O.S. reckoned as 28 January 1919 | Yes, 2026-09-25 |
| [elwiki-gregoriano-imerologio] | The royal decree; 16 February 1923 reckoned as 1 March; the Church's Revised Julian calendar of 1924 | Yes, 2026-09-25 |
| [trwiki-miladi-takvim] | Law No. 698 of 26 December 1925, in force 1 January 1926 | Yes, 2026-09-25 |
| [wikipedia-ja-gregorio-reki] | Japan's change on 1 January 1873 | Yes, 2026-09-25; the proclamation's number was not on the page |
| [reingold2018code] | `fixed-from-julian`, `fixed-from-gregorian`, `julian-epoch`; that the code carries no adoption table | Yes, 2026-09-25 |
| [reingold2018] | Cited by the module for an appendix of adoption dates | Not read |
| [explanatory-supplement-2013] | Cited by the module for §15.3 | Not read |

The pages that could not be reached on 2026-09-25: the Russian decree on
Wikisource and the Japanese proclamation on Wikipedia and Wikisource, for
which the transcription at Moscow State University and the Japanese
article on the Gregorian calendar were used instead.

## Code

`crates/hc-calendars-solar/src/julian_gregorian.rs`: `Adoption` and
`ADOPTIONS`, `Adoption::cutover` and `Adoption::skipped_days`,
`ReformCalendar` with `new`, `with_cutover`, `cutover`, `is_new_style`,
and the `Calendar` implementation whose `to_fixed` does the two-sided
test above; `adoption_by_id` finds a row. Anchors:
`every_tabulated_adoption_is_internally_consistent`,
`the_weekday_cycle_is_unbroken_across_every_gap`,
`dates_in_the_gap_never_existed`,
`the_reform_year_is_short_by_the_days_it_skipped`,
`a_custom_cutover_works_for_polities_not_in_the_table`. The Swedish
calendar is `swedish.rs`, with `DOUBLE_LEAP_DAY`, `EARLIEST` and
`LATEST`, and its anchors
`the_calendar_begins_by_leaving_out_29_february_1700`,
`thirty_february_1712_returns_it_to_the_julian_calendar` and
`every_day_is_one_ahead_of_julian_and_ten_behind_gregorian`. The year
starts are `year_style.rs`; the Rumi calendar is `rumi.rs`.
