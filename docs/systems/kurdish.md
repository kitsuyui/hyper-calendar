# The Kurdish calendar: the Solar Hijri year under Kurdish names, 1321 ahead

## What it is

The Kurdish calendar is a solar calendar that Kurds use to mark years,
months and seasons. English Wikipedia's article "Kurdish calendar" says it
begins each year on Newroz, 21 March, at the spring equinox; names twelve
months, the first six of 31 days, the next five of 30 and the last, Reşeme,
of 29 or 30; and says it is "formally recognized for cultural and official
use in the Kurdistan Region of Iraq", with no date for that recognition
[wikipedia-kurdish-calendar]. The article's table of the months is headed
"Approximate Gregorian Span", and it states no leap rule and no rule for
the year number. Its history section puts the start of the count at the
Battle of Nineveh in 612 BC, which gives 2638 for the year from 21 March
2026, not the 2726 the same page displays.

That displayed date is the only exact statement of the reckoning the page
carries. The page's box "Today's Kurdish Date" is produced by
`Template:Kurdish calendar date today`, whose wikitext writes
`{{#time:xij}}`, the month name switched on `{{#time:xin}}`, and
`{{#expr: {{#time:xiY}} + 1321}}` [wikipedia-template-kurdish-calendar-date-today].
MediaWiki's `xi` codes are the Iranian (Jalaly) calendar
[mediawiki-parserfunctions-time]. So the page's reckoning is the Solar Hijri
date, with the month renamed and the year 1321 ahead of the Solar Hijri
year. It is not a calendar on fixed Gregorian dates.

## How it works

MediaWiki computes the `xi` codes with `Language::tsToIranian`, "by Roozbeh
Pournader and Mohammad Toossi" [mediawiki-language-tstoiranian]. It counts
the days since 1 January 1600, less 79, so that day 0 is 20 March 1600, 1
Farvardin 979. It divides them into cycles of 12 053 days, each of 33
years, the first four-year block of a cycle opening with a 366-day year,
and then into the months of 31, 30 and 29 days. That is the 33-year rule
this library carries as `persian-arithmetic-33` ([solar-hijri.md](solar-hijri.md)): a Solar Hijri year is leap when it leaves a
remainder of 1, 5, 9, 13, 17, 22, 26 or 30 on division by 33.

Worked example. 4 October 2026 is 12 Mehr 1405. Mehr is the seventh month, whose
Kurdish name is Rezber, and 1405 + 1321 is 2726, so the date is 12 Rezber
2726, the date the page displayed on 4 October 2026. Newroz 2726 is 1
Farvardin 1405, 21 March 2026. A year on fixed Gregorian dates would put Newroz on 21 March
every year; the template does not. 1403 is a leap year of the 33-year rule,
so 1 Farvardin 1403 is 20 March 2024 and 1 Xakelêwe 2724 is that day, and
30 Reşeme 2724 is 20 March 2025; 1 Xakelêwe 2728 is 20 March 2028.

| Month | Kurdish name | Solar Hijri month | Days |
| --- | --- | --- | --- |
| 1 | Xakelêwe | Farvardin | 31 |
| 2 | Gulan | Ordibehesht | 31 |
| 3 | Cozerdan | Khordad | 31 |
| 4 | Pûşper | Tir | 31 |
| 5 | Gelawêj | Mordad | 31 |
| 6 | Xermanan | Shahrivar | 31 |
| 7 | Rezber | Mehr | 30 |
| 8 | Gelarêzan | Aban | 30 |
| 9 | Sermawez | Azar | 30 |
| 10 | Befranbar | Dey | 30 |
| 11 | Rêbendan | Bahman | 30 |
| 12 | Reşeme | Esfand | 29, or 30 in a leap year |

## What is carried

- **`kurdish`** is the reckoning the page displays: `persian-arithmetic-33`'s
  date, the months under their Kurdish names and the year 1321 ahead
  (`YEAR_OFFSET`). Policy §5 gives each reckoning its own id; this id names
  the template's, and says so, so that a reckoning from a Kurdish source gets
  another. It converts Kurdish 2300 (Solar Hijri 979, the year that begins on 20 March 1600,
  the template's first day) to 9999. Days before 20 March 1600 and years before 2300 are
  refused, not answered: the sources read display no date earlier, and a year they do not
  reach is a gap (ADR 0013).
- Not carried, each for want of a source that states it:
  - A year on fixed Gregorian dates, every month beginning on the same Gregorian day each year
    and Reşeme taking its 30th day from a Gregorian February. The article's spans are headed
    approximate and no source read states it. Where the Solar Hijri Nowruz falls on 20 March,
    in 2024 and 2028 among others, such a reading and the page's would disagree for a whole
    year.
  - A Kurdish year over the astronomical Solar Hijri calendar, the Iranian civil rule
    (`persian`, in `hc-calendars-equinox`): it needs a source stating that the Kurdistan Region
    reckons the year by the equinox. The two agree on every day from 1 Farvardin 1178 to
    29 Esfand 1634, Kurdish 2499 to 2955, and part company on the 33-year rule's leap day,
    30 Esfand 1634, which the equinox calendar has as 1 Farvardin 1635. The span is the one
    Borkowski gives, not read here, as Heydari-Malayeri reports it; the day-by-day agreement
    is the test `the_33_year_rule_agrees_day_by_day_until_its_leap_day_in_1634`.
  - The article's epoch of 612 BC: it gives neither 2726 for 2026 nor any other year the article
    writes.
  - A period of use. The article names the Kurdistan Region of Iraq and no date, so the usage is unrecorded.
  - The Sorani spellings of the months, خاکەلێوە to ڕەشەمە, which the template also writes,
    for want of a Kurdish locale; `hc_describe_day` answers with the romanised
    names under the language `und`.
  - The Kurdistan Region's own instrument, which was not found and would replace the article.

## Accuracy

Exact to the date the page's template displays, from 20 March 1600, the first
day the template's arithmetic is defined for, to the year 9999. Before 1600
nothing is answered: the 33-year rule's arithmetic carried back is a date no
page displays, so `from_fixed` refuses a day before 20 March 1600 with
`BeforeEpoch` and `to_fixed` refuses a year before 2300 with `YearOutOfRange`
(ADR 0013; policy §4). The
test `the_template_is_the_pournader_toossi_algorithm` holds this module to a
Rust transcription of `tsToIranian` on every day from 20 March 1600 to the
end of 2600, the month under its Kurdish name and the year 1321 ahead;
the transcription was written from the PHP source read on 2026-10-05, and MediaWiki
itself was not run. The page's one displayed date, 12 Rezber 2726 for 4
October 2026, is `the_source_wrote_12_rezber_2726_for_4_october_2026`.

Whether the Kurdistan Region reckons the year the same way is not
established by any source read, so a day carries the template's reckoning,
not the Region's: a user needing the Region's calendar needs the Region's instrument.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [wikipedia-kurdish-calendar] | The months, their lengths and approximate spans, Newroz on 21 March, the recognition in the Kurdistan Region, the 612 BC statement, the date displayed on 4 October 2026 | Yes, 2026-10-04; as wikitext 2026-10-05; secondary, its own citations (Kirmanj 2014, Hirschler 2001, Rafaat 2016, Elis 2004, O'Leary, McGarry and Ṣāliḥ 2005) not read |
| [wikipedia-template-kurdish-calendar-date-today] | The `#time:xi` codes and `+ 1321` that produce the displayed date | Yes, as wikitext, 2026-10-05 |
| [mediawiki-language-tstoiranian] | The algorithm the `xi` codes compute | Yes, 2026-10-05 |
| [mediawiki-parserfunctions-time] | That the `xi` codes are the Iranian calendar | Yes, as wikitext, 2026-10-05 |
| [heydari-malayeri2004] | The 33-year rule's remainders, through [solar-hijri.md](solar-hijri.md) | Yes, per that document |

## Code

`crates/hc-calendars-solar/src/kurdish.rs` (`MONTHS`, `YEAR_OFFSET`,
`TEMPLATE_FROM`, `KurdishCalendar`), over `persian_33` and the shared
`common::six_thirty_ones_*` month arithmetic. Anchors:
`the_template_is_the_pournader_toossi_algorithm`,
`the_source_wrote_12_rezber_2726_for_4_october_2026`,
`resheme_has_thirty_days_in_the_rules_leap_years`, `every_day_round_trips`.
