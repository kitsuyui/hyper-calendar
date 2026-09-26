# The Hijri calendars: the tabular schemes, the Umm al-Qura table and the observational prediction

Backs the identifiers `islamic-civil`, `islamic-tbla`, `islamic-fatimid`,
`islamic-umalqura` and `islamic-rgsa` in `hc-calendars-lunar`, and the
Kūshyār ibn Labbān and Ḥabash al-Ḥāsib schemes that `tabular` can build but
does not register.

## What it is

The Hijri (*hijrī*) calendar is the lunar calendar of Islam: twelve months
of 29 or 30 days, a year of 354 or 355 days that runs through the seasons
in about 33 years, counted from the year of the Prophet's migration to
Medina in 622 CE. It fixes the days of religious observance — the fasting
month of Ramaḍān, ʿĪd al-Fiṭr on 1 Shawwāl, ʿĪd al-Aḍḥā on 10 Dhū al-Ḥijja
during the pilgrimage — for more than a billion people, most of whom keep
the Gregorian calendar for everything else [vangent-tabcal]. The day begins
at sunset.

**The calendar as kept is announced, not computed.** A month begins on the
evening the new crescent (*hilāl*) is seen after sunset and a competent
authority accepts the report. In Saudi Arabia the Supreme Court calls on
the public to look for the crescent on the evening of the 29th and
announces the result the same night [spa-ramadan-1445, spa-eid-alfitr-1445,
spa-ramadan-1446]; other countries and communities have their own
authorities and their own criteria, and on the same evening they reach
different answers, so that the major festivals are kept one, two or even
three days apart from one place to another [vangent-ummalqura]. No
computation reproduces an announcement, and nothing in this library claims
to.

Beside the announced calendar stand three kinds of scheme, and this
document covers one implementation of each:

- **The tabular (arithmetic, *ḥisābī*) calendar**, introduced by Muslim
  astronomers in the eighth century to predict the approximate beginning of
  the months: months alternately 30 and 29 days, and an extra day at the
  end of the year in eleven years of every thirty [vangent-tabcal]. It is
  the calendar of the medieval astronomical tables (*zīj*es), of most
  published conversion tables and of most software — CLDR's `islamic-civil`
  and `islamic-tbla` are two of its variants [cldr-bcp47-calendar] — and it
  is the operative religious calendar of one community: the Ṭayyibī
  Ismāʿīlīs, of whom the Dawoodi Bohras are the largest, keep every
  religious date on a calculated calendar they call *Misri* (Egyptian)
  after the Fatimid Imams whose method it follows, with the rule set out in
  the community's *Sahifa* [dawoodibohras-misri].
- **The Umm al-Qura calendar**, the computed lunar calendar Saudi Arabia
  has used for civil purposes for several decades, followed also by
  Bahrain, Qatar and the United Arab Emirates and, through Saudi-funded
  mosques and through software, by many communities elsewhere. Its dates
  are determined at the Institute of Astronomical and Geophysical Research
  of the King Abdulaziz City for Science and Technology (KACST) in Riyadh
  from modern theories of the Sun and Moon, for the latitude and longitude
  of the Kaʿba [vangent-ummalqura]. It is an administrative calendar; the
  Court's sighting announcements are what fix Ramaḍān and the ʿĪds, and in
  the years checked below they fell on the table's day.
- **A predicted observational calendar**, which applies a published
  visibility criterion to a named place and takes the first evening the
  crescent *should* have been visible as the start of the month. Reingold
  and Dershowitz give one such calendar in *Calendrical Calculations*, with
  Cairo as its sample location [reingold2018code]; this library applies it
  at Mecca under CLDR's identifier `islamic-rgsa`, "Hijri calendar, Saudi
  Arabia sighting" [cldr-bcp47-calendar]. It is a forecast of an
  observation, not a record of a decision.

Who uses which, in short: administrations and software the tabular
calendar; Saudi Arabia and its neighbours the Umm al-Qura table for civil
dates; the Bohras the Fatimid tabular scheme for religious dates; everyone
else's religious dates come from an announcement, which a prediction can
anticipate and cannot replace.

## How it works

### The thirty-year cycle

Odd months have 30 days and even months 29, so a common year has 354 days.
In eleven years of every thirty the twelfth month, Dhū al-Ḥijja, takes a
thirtieth day: 30 years run 30 × 354 + 11 = 10 631 days, a mean month of
10 631 / 360 = 29.530 556 days [vangent-tabcal]. The mean synodic month is
29.530 588 861 days [reingold2018code, `mean-synodic-month`], so the
tabular month is 2.9 seconds short and the scheme drifts a day from the
Moon in about 2 400 years (30 000 months); that figure is derived here, not
quoted.

Which eleven years are long is the one thing the tabular calendars differ
on. Van Gent tabulates four schemes from the literature [vangent-tabcal]:

| Scheme | Long years of the cycle | Attributed to | In this crate |
| --- | --- | --- | --- |
| I | 2, 5, 7, 10, 13, 15, 18, 21, 24, 26, 29 | Kūshyār ibn Labbān, Ulugh Beg, ʿAlī al-Qūshjī, Taqī al-Dīn | `LeapYearRule::KUSHYAR_IBN_LABBAN` |
| II | 2, 5, 7, 10, 13, 16, 18, 21, 24, 26, 29 | al-Fazārī, al-Khwārizmī, al-Battānī, the Toledan and Alfonsine tables, Microsoft's HijriCalendar | `LeapYearRule::CIVIL` |
| III | 2, 5, 8, 10, 13, 16, 19, 21, 24, 27, 29 | The Fatimid calendar, also called the Ismāʿīlī, Ṭayyibī or Bohra calendar; Ibn al-Ajdābī | `LeapYearRule::FATIMID` |
| IV | 2, 5, 8, 11, 13, 16, 19, 21, 24, 27, 30 | Ḥabash al-Ḥāsib, al-Bīrūnī, Elias of Nisibis | `LeapYearRule::HABASH_AL_HASIB` |

Scheme II is the common one: it is what "the tabular Islamic calendar"
means unless a source says otherwise, and CLDR's two identifiers both
carry it [cldr-bcp47-calendar]. The Bohra community states scheme III in
its own words: divide the year by 30, and a remainder of 2, 5, 8, 10, 13,
16, 19, 21, 24, 27 or 29 makes a *kabisa* year [dawoodibohras-misri]. All
four agree on years 2, 5, 13, 21 and 24, and since each adds eleven days a
cycle they coincide again at every cycle boundary. A fifth pattern, 2, 5,
8, 10, 13, 16, 18, 21, 24, 26, 29, is known only from a lost astrolabe of
1212/13, and an eight-year cycle with long years 2, 5 and 8 was used in the
Ottoman Empire and South-East Asia [vangent-tabcal]; neither is carried.

Each scheme comes in two variants, by **epoch**: 1 Muḥarram 1 AH is either
Thursday 15 July 622 (Julian), the "astronomical" epoch, or Friday 16 July
622, the "civil" epoch [vangent-tabcal]. The difference is the old question
of whether the day is counted from the preceding sunset or from the
morning. In Julian Day Numbers the two are 1 948 439 and 1 948 440
[icu-islamcal]; in this library's fixed days, RD 227 014 and RD 227 015.
Reingold and Dershowitz use the Friday epoch [reingold2018code,
`islamic-epoch`], and CLDR names the two variants of scheme II
`islamic-tbla` (Thursday) and `islamic-civil` (Friday)
[cldr-bcp47-calendar, cldr-islamic-calendar-types].

For scheme II the closed forms are Reingold and Dershowitz's
[reingold2018code, `islamic-leap-year?`, `fixed-from-islamic`]: year *y*
is long when (14 + 11*y*) mod 30 < 11, and the fixed day of *y*-*m*-*d* is

    epoch − 1 + 354 (y − 1) + ⌊(3 + 11y) / 30⌋ + 29 (m − 1) + ⌊m / 2⌋ + d.

The crate implements every scheme the same way — count the whole cycles,
then the years within the cycle, then the long years among them, then the
months — and checks that against the closed form.

**Worked example: 1 Ramaḍān 1445 by hand, scheme II on the Friday epoch.**
1445 − 1 = 1444 = 48 × 30 + 4, so 48 whole cycles precede the year and it
stands fifth in the 49th. Of the years before position 5, only year 2 is
long. Ramaḍān is month 9, and the eight months before it hold
29 × 8 + ⌊9 / 2⌋ = 236 days. So

    227 015 + 48 × 10 631 + 4 × 354 + 1 + 236 + (1 − 1)
    = 227 015 + 510 288 + 1 416 + 1 + 236 = 738 956,

which is Monday 11 March 2024. The closed form agrees: 227 014 +
354 × 1444 + ⌊(3 + 11 × 1445) / 30⌋ + 29 × 8 + 4 + 1 = 227 014 + 511 176 +
529 + 237 = 738 956. On the Thursday epoch (`islamic-tbla`) the same date
is Sunday 10 March, and so it is on the Fatimid scheme, whose long years
before position 5 are the same one.

### The Umm al-Qura rules

The Umm al-Qura calendar is computed, but by rules that have changed, and
the rules are what van Gent reconstructs from the published dates
[vangent-ummalqura]:

| Years | Rule for beginning a month |
| --- | --- |
| Before 1392 AH | Uncertain. A claim that the computed lunar altitude at sunset had to be at least 9° is not borne out by a conversion table published by the King Fahd University of Petroleum and Minerals in the early 1990s; between 1356 and 1392 AH about 17.5% of months began before the astronomical new moon |
| 1392–1419 AH (16 March 1972 to 16 April 1999) | If the new moon falls less than three hours after Saudi midnight (0h UT), the month begins at the previous sunset; otherwise at the following one. About a third of months began before the new moon |
| 1420–1422 AH (from 17 April 1999) | On the 29th, the next day is the first of the new month if moonset is after sunset at Mecca; otherwise the month runs 30 days. A month could still begin before the new moon when the Moon stood far north of the ecliptic, as in Rajab 1421 and Shaʿbān 1422 |
| From 1423 AH (15 March 2002) | On the 29th, the next day is the first of the new month if the geocentric conjunction occurs before sunset **and** the Moon sets after the Sun at Mecca; otherwise the month runs 30 days |

Under the current rule a month always begins after the new moon with the
crescent above the horizon at sunset, often only just; in about three
quarters of months it would not be visible to the naked eye on that first
evening [vangent-ummalqura]. The calendar is computed for the Kaʿba in the
Great Mosque of Mecca.

**The table's provenance and range.** What this library carries is not the
rules but the result: the length of every month from 1300 to 1600 AH, one
bit per month. The 301 masks are those of ICU's `UMALQURA_MONTHLENGTH`
table, which covers `UMALQURA_YEAR_START = 1300` to `UMALQURA_YEAR_END =
1600` and names no source for its data; ICU's epochs are the same two
Julian Day Numbers as above, and outside the table ICU falls back to the
civil arithmetic [icu-islamcal]. The module says its copy was read off the
platform's Foundation implementation, which wraps ICU's; the copy was
compared bit for bit with ICU's own source on 2026-09-25, with every one of
the 301 years agreeing (ICU numbers the bits from Dhū al-Ḥijja, this crate
from Muḥarram). The official site of the calendar is KACST's
[kacst-ummulqura], which on 2026-09-25 served only its title, so how the
1300–1419 years of the table were produced — from records of the calendar
as issued, or from a reconstruction under the rules above — is not stated
by any source read here. Van Gent's converter computes its Umm al-Qurā
dates from the rules rather than tabulating them [vangent-ummalqura], so
it could not serve as a check on the rows either. The table is therefore
another library's, cross-checked against five announcements; the rows for
1300–1391 AH, before any rule is known, are a computation whose author is
not named.

### The visibility criterion

The predicted calendar is Reingold and Dershowitz's observational Islamic
calendar with their `visible-crescent` test, which in the published code is
S. K. Shaukat's criterion [reingold2018code, `shaukat-criterion`,
`simple-best-view`, `arc-of-light`]. On the evening before a candidate
first day, at the moment the Sun is 4.5° below the horizon, the crescent
counts as visible when all three hold:

1. the Moon is past conjunction and short of first quarter: its elongation
   from the Sun in longitude lies in 0° ≤ φ < 90°;
2. its *arc of light*, the true angular separation from the Sun,
   arccos(cos β · cos φ) for lunar latitude β, is at least 10.6° and at most
   90°;
3. its altitude above the horizon is more than 4.1°.

The first day of the month containing a day is the most recent day whose
eve passed the test, found by stepping forward from two days before the
last new moon, or from a month earlier when the day is within three days
of the new moon and its own eve fails [reingold2018code,
`phasis-on-or-before`]. The year and month are then the elapsed lunations
from the Friday epoch, rounded over the mean synodic month, split into
twelves [reingold2018code, `observational-islamic-from-fixed`]. The
observing place is a parameter: the book's sample location is Cairo, and
`islamic-rgsa` observes from Mecca at the book's own `mecca` constant,
21°25′24″N, 39°49′24″E, 298 m [reingold2018code, `mecca`]. No type in the
module has a default site or criterion; each is named where it is
chosen.

**The 4.5° interpolation.** `hc-astro` gives dusk only at the three standard
twilight depressions, so the module does not solve for the 4.5° instant.
It takes the depression the Sun has at sunset — not zero, since the upper
limb is on the horizon, refraction has lifted it, and the horizon dips
with the observer's height — and interpolates linearly between sunset and
civil dusk (6°) to the fraction where 4.5° falls. At Mecca that puts the
judging moment about thirteen minutes after sunset. The module states the
error of the linearisation as well under a minute; no test measures it,
and a minute would move the Moon by half a degree of altitude at most.

**Worked example: why the prediction lands a day after the table.** The
new moon of March 2024 was at 09:00 UT on Sunday 10 March. At Mecca that
evening the Sun set at 15:31 UT (18:31 local) and the Moon at 15:44 UT,
thirteen minutes later. Both Umm al-Qura conditions held on the 29th of
Shaʿbān — conjunction before sunset, moonset after sunset — so the table
begins Ramaḍān 1445 on Monday 11 March, and so does the tabular civil
calendar, as worked above. The Supreme Court, having received reports of a
sighting that evening, announced the same day [spa-ramadan-1445].

The criterion, judged at 15:44 UT with the Sun 4.5° down, sees a Moon
6¾ hours past conjunction: elongation 4.0°, latitude −1.9°, arc of light
4.4°, altitude −0.4°. It fails the second test by a wide margin and the
third because the Moon has just set. On Monday evening 11 March the Moon
is 30 hours old, with elongation 18.2°, arc of light 18.2° and altitude
13.7°, and passes. So `islamic-rgsa` puts 1 Ramaḍān 1445 on Tuesday
12 March 2024, one day after the table and after the announcement. The
same happens at 1 Ramaḍān 1446: the table and the Court have Saturday
1 March 2025 [spa-ramadan-1446], the evening of 28 February shows an arc of
light of 8.5° and an altitude of 4.0°, and the prediction says Sunday
2 March. It does not always happen: for 1 Shawwāl 1445 the evening of
9 April 2024 shows an arc of 12.6° at 8.0° altitude, the criterion passes,
and table, prediction and announcement all give Wednesday 10 April
[spa-eid-alfitr-1445]. The figures are from `hc-astro` through the
module's own functions and are the ones the tests rely on.

## What is carried

- **Identifiers**, all in `hc-calendars-lunar`, all with the date as
  `IslamicDate { year, month, day }`, the era code `ah`, and the day beginning at
  sunset and named by the civil day it ends on,
  `DayBoundary::Sunset(DayNaming::ByEnd)`: the crescent that opens a month
  is judged "on eve of" its first day, at the sunset of the civil day
  before [reingold2018code, `phasis-on-or-before`, `saudi-criterion`]. The
  tabular calendars have no observed evening and state the same boundary,
  so every variant agrees:

  | Identifier | What it is | Parameters |
  | --- | --- | --- |
  | `islamic-civil` | Tabular, scheme II, Friday epoch | `LeapYearRule::CIVIL`, `CIVIL_EPOCH` = RD 227 015 |
  | `islamic-tbla` | Tabular, scheme II, Thursday epoch | `LeapYearRule::CIVIL`, `ASTRONOMICAL_EPOCH` = RD 227 014 |
  | `islamic-fatimid` | Tabular, scheme III, Thursday epoch: the Bohra *Misri* calendar | `LeapYearRule::FATIMID`, `ASTRONOMICAL_EPOCH` |
  | `islamic-umalqura` | The Umm al-Qura table | `MONTH_LENGTH_MASKS`, 301 × `u16` |
  | `islamic-rgsa` | The observational prediction at Mecca | `ObservationSite::MECCA`: the location and `VisibilityCriterion::SHAUKAT` (4.5°, 10.6°, 4.1°), named after the published code's `shaukat-criterion` |

  The Fatimid calendar's epoch is fixed by the community's own anchor: its
  page dates the Mawlid, 12 Rabīʿ al-Awwal 1439, to 30 November 2017
  [dawoodibohras-misri], which scheme III gives on the Thursday epoch and
  misses by a day on the Friday one. The name is this library's own, minted
  because CLDR has no identifier for the scheme.
- **Constructible but not registered:** scheme I and scheme IV, on either
  epoch, through `TabularIslamicCalendar::new` with
  `LeapYearRule::KUSHYAR_IBN_LABBAN` or `LeapYearRule::HABASH_AL_HASIB`. They
  are medieval *zīj* variants that no community keeps and no authority
  publishes today, so there is nobody who could say a registry entry for
  them was right or wrong, and the crate keeps them as parameters. A site
  other than Mecca, or other thresholds, are reached the same way through
  `ObservationSite::new` and `IslamicObservationalCalendar::new`.
- **Ranges.** The tabular calendars convert 1 to 9 999 AH, a bound chosen
  to match the rest of the library rather than anything in the sources. The
  table converts 1300 to 1600 AH, RD 687 337 (12 November 1882) to
  25 November 2174, and refuses every day outside it rather than falling
  back to arithmetic: a computed month presented as an Umm al-Qura month
  would be a fabrication, and a caller comparing this crate with ICU outside
  the table is comparing a refusal with a guess. The prediction converts
  1 January 1900 to 31 December 2100, a window the module bounds not by the
  astronomy, which is good well beyond it, but by how far it is honest to
  carry a forecast of a human decision.
- **What is tabulated and what is computed.** The table is the calendar:
  each year's mask gives twelve month lengths, the year starts are
  accumulated from them once, and no astronomy is done, which is why the
  calendar is exact and why its metadata says it is not astronomical. The
  tabular calendars are pure counting. The prediction computes sunset,
  civil dusk, the Moon's elongation, latitude and altitude from `hc-astro`
  for every evening it judges, and the month count from the mean synodic
  month.
- **Not carried, and why.**
  - What any authority announced. The announcements cited below are used
    to check the table, not carried as data; a record of proclamations is a
    different thing from a calendar and would be a table of its own.
  - The Umm al-Qura calendar before 1300 AH or after 1600, and Reingold and
    Dershowitz's computed `saudi-criterion` [reingold2018code], which
    reproduces the post-1423 rule from the astronomy. Inside the table's
    range the table is the published thing and a computation would only
    disagree with it in the years the rules were different; outside it a
    computation would be presenting a guess as the Saudi calendar.
  - The astrolabe pattern and the eight-year cycle, for which no user and no
    anchor were found.
  - Any claim about the observational calendar beyond what the 322-of-552
    figure below says: it is the rate at which one criterion at one place
    lands a day after one table, and it says nothing about how often the
    prediction matches any announcement.

## Accuracy

| Measure | Result | Test |
| --- | --- | --- |
| Scheme II on the Friday epoch against the published closed form, every month of 1–3000 AH | 36 000 of 36 000 | `the_table_driven_civil_rule_agrees_with_the_published_closed_form` |
| Every scheme has eleven long years, runs 10 631 days a cycle, is long in 2, 5, 13, 21 and 24, and realigns at each cycle boundary | All four | `every_rule_has_eleven_long_years_in_thirty`, `every_rule_gives_a_cycle_of_ten_thousand_six_hundred_and_thirty_one_days`, `all_four_rules_agree_on_the_years_the_sources_agree_on`, `the_variants_realign_at_every_cycle_boundary` |
| The two epochs are JDN 1 948 440 (Friday) and 1 948 439 (Thursday) | Both | `the_civil_epoch_is_the_sixteenth_of_july_622`, `the_astronomical_epoch_is_one_day_earlier`, `the_civil_epoch_is_a_friday_and_the_astronomical_one_a_thursday` |
| The Bohra anchor, 12 Rabīʿ al-Awwal 1439 = 30 November 2017, and the *kabisa* remainders, with 1431 long and 1432 not | Reproduced | `the_bohra_misri_calendar_matches_its_own_published_mawlid`, `the_kabisa_remainders_are_the_ones_the_community_publishes` |
| The Umm al-Qura masks against ICU's `islamcal.cpp` | 301 of 301, checked 2026-09-25 by hand, not by a test | — |
| Five Saudi announcements against the table: 1 Muḥarram 1445 = 19 July 2023, 1 Ramaḍān 1445 = 11 March 2024, 1 Shawwāl 1445 = 10 April 2024, 1 Ramaḍān 1446 = 1 March 2025, 1 Muḥarram 1447 = 26 June 2025 | 5 of 5 on the table's day | `the_published_dates_are_reproduced` |
| The tabular civil calendar against the table over all 3 612 months of 1300–1600 AH | 1 421 month starts differ (39.3%), never by more than three days | `the_table_stays_close_to_the_arithmetic_calendar_without_matching_it` |
| The prediction against the table over the 552 months of 1400–1445 AH | 322 begin a day later (58.3%), 230 on the same day, none earlier, none further off | `the_prediction_disagrees_with_the_saudi_table_and_the_crate_says_by_how_much` |

**What the five announcements check.** They are the Supreme Court's
sighting-based decisions [spa-ramadan-1445, spa-eid-alfitr-1445,
spa-ramadan-1446, gulfnews-muharram-1445, gulfnews-muharram-1447], and each
fell on the day the table gives. That confirms five rows of the table; it
does not make the table a record of announcements, and the module's sixth
row, 1 Muḥarram 1300 = 12 November 1882, is the table's own first entry,
for which no Saudi publication was found — its only check here is that the
civil arithmetic gives the same day.

**What the 322-of-552 figure measures.** For each of the 552 month starts
the table gives for 1400–1445 AH, whether Shaukat's criterion at Mecca
passes on the evening before the table's first day. When it does, the two
agree; when it does not, the prediction begins the month the next evening,
which is why the difference is always one day in the same direction: the
table's conditions — conjunction before sunset, Moon above the horizon at
sunset — are the beginning of what the criterion asks, not the end of it. So
58% is the share of Umm al-Qura first evenings on which the crescent is,
by this criterion, too young or too low to be seen. It is not an error rate
of either calendar, since neither is trying to be the other. Two caveats on
reading it: the 552 months span three regimes of the Umm al-Qura rules
(1400–1419, 1420–1422 and 1423–1445), so the figure averages over rules
that were not the same; and van Gent's "about 75% not visible to the naked
eye" is for the post-1423 rule under his own visibility model, a different
measure that should not be expected to match. The number is asserted
exactly in the test so that a change to `hc-astro` or to the criterion
cannot move it unnoticed.

**What the module documentation states on its own authority** is listed
at the end of the next section.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [reingold2018] | The scheme II closed forms, the observational calendar, the criterion | Not read directly; the published code was |
| [reingold2018code] | `islamic-epoch`, `islamic-leap-year?`, `fixed-from-islamic`, `islamic-from-fixed`, `mean-synodic-month`, `mecca`, `islamic-location`, `shaukat-criterion`, `simple-best-view`, `arc-of-light`, `visible-crescent`, `phasis-on-or-before`, `fixed-from-observational-islamic`, `observational-islamic-from-fixed`, `saudi-criterion` | Yes, 2026-09-25 |
| [vangent-tabcal] | The origin of the tabular calendar, the four schemes and their attributions, the astrolabe pattern, the eight-year cycle, the two epochs | Yes, 2026-09-25 |
| [vangent-ummalqura] | Who uses the Umm al-Qura calendar and who computes it, the Kaʿba as its place, the rules by period, the 75% figure | Yes, 2026-09-25 |
| [kacst-ummulqura] | The official site | Reached 2026-09-25; the page served only its title, and nothing was read from it |
| [cldr-bcp47-calendar] | The identifiers and their descriptions | Yes, 2026-09-25 |
| [cldr-islamic-calendar-types] | The Friday and Thursday epochs of the two tabular identifiers; Umm al-Qura as Saudi Arabia's administrative calendar | 2026-09-25, as an extracted summary only; nothing is quoted from it |
| [icu-islamcal] | The table, its range, the two epochs, the fallback outside the range | Yes, 2026-09-25 |
| [dawoodibohras-misri] | The Bohra calendar: its name, its rule, the *kabisa* remainders, the worked years 1431 and 1432, the Mawlid of 1439 on 30 November 2017 | Yes, 2026-09-25 |
| [spa-ramadan-1445] | 1 Ramaḍān 1445 = Monday 11 March 2024 | 2026-09-25; the page served its title, which carries the date |
| [spa-eid-alfitr-1445] | 1 Shawwāl 1445 = Wednesday 10 April 2024, Ramaḍān of 30 days | Yes, 2026-09-25 |
| [spa-ramadan-1446] | 1 Ramaḍān 1446 = Saturday 1 March 2025 | 2026-09-25; the page served its title, which carries the date |
| [gulfnews-muharram-1445] | 1 Muḥarram 1445 = Wednesday 19 July 2023, after a sighting on 18 July | Yes, 2026-09-25 |
| [gulfnews-muharram-1447] | The sighting on the evening of 25 June 2025 and Dhū al-Ḥijja 1446 of 29 days, so 1 Muḥarram 1447 = Thursday 26 June 2025 | Yes, 2026-09-25 |

Statements in the module documentation that no source in this table
supports, and that stand as the module's own:

- The extraction of the table from Apple's Foundation cannot be checked
  from what is carried; that the result equals ICU's table can be, and was.
- The interpolation error "well under a minute" is the module's estimate
  and is not measured by a test.


## Code

`crates/hc-calendars-lunar/src/tabular.rs` holds the cycle arithmetic,
`LeapYearRule` with the four schemes, the two epochs and the `FATIMID`
calendar; `islamic_civil.rs` and `islamic_astronomical.rs` are the two CLDR
parameter sets on it; `islamic_umalqura.rs` holds `MONTH_LENGTH_MASKS` and
the year-start accumulation; `islamic_observational.rs` holds
`VisibilityCriterion`, `ObservationSite`, `MECCA` and the month search.
Anchors: in `tabular`,
`the_table_driven_civil_rule_agrees_with_the_published_closed_form`,
`the_variants_realign_at_every_cycle_boundary`,
`kushyars_rule_differs_from_the_civil_one_in_exactly_one_year`,
`the_bohra_misri_calendar_matches_its_own_published_mawlid`; in
`islamic_umalqura`, `the_published_dates_are_reproduced`,
`the_table_covers_exactly_thirteen_hundred_to_sixteen_hundred`,
`the_range_is_refused_rather_than_extrapolated`,
`the_table_stays_close_to_the_arithmetic_calendar_without_matching_it`; in
`islamic_observational`,
`the_evaluation_moment_falls_between_sunset_and_civil_dusk`,
`the_prediction_disagrees_with_the_saudi_table_and_the_crate_says_by_how_much`,
`the_site_is_configurable_and_the_answer_depends_on_it`. The registry entry
for the prediction is Mecca's, and `register_all` in `lib.rs` says why the
Kūshyār and Ḥabash schemes are not registered. English month names are in
`hc-i18n`.
