# The equinox calendars: the Badíʿ calendar from 2015 and the French Republican decree

Backs the identifiers `bahai-astronomical` and `french-republican-equinox`
in `hc-calendars-equinox`, and explains their siblings `bahai`,
`bahai-arithmetic` and `french-republican-arithmetic` in
`hc-calendars-solar`. The crate's Solar Hijri calendars, `persian` among
them, are written up in [solar-hijri.md](solar-hijri.md).

## What it is

Two calendars whose year begins on the day an equinox falls, judged by a
clock at a named place. The equinox is one instant for the whole Earth;
which *day* it falls on depends on where the day is reckoned from and when
it begins there, and each calendar names both.

**The Badíʿ calendar** of the Bahá'í Faith: nineteen months of nineteen
days, 361 days, and four or five intercalary days, Ayyám-i-Há, between the
eighteenth month and the nineteenth, ʿAláʾ, the month of the fast. The day
runs from sunset to sunset, and the era begins with the year of the Báb's
declaration, Naw-Rúz 1844. Until 171 BE the communities of the West kept
Naw-Rúz on 21 March. In a letter of 10 July 2014 the Universal House of
Justice unified the calendar from Naw-Rúz 172 BE, 21 March 2015 — "the
setting of the sun on 20 March 2015 will signalize the end of the year
171": Tehran is the point from which the moment of the March equinox, and
so Naw-Rúz, is determined by astronomical computation; Ayyám-i-Há has as
many days as it takes to reach the next Naw-Rúz, four in 172 BE; and the
Twin Holy Birthdays are the first and second day after the eighth new moon
following Naw-Rúz [uhj-2014-07-10]. A table for half a century was
promised with the letter, and the Bahá'í World Centre issued it as
*Badíʿ dates 172 to 221 BE*, prepared by an ad hoc committee from data
supplied by Her Majesty's Nautical Almanac Office, with Tehran's position
taken from WGS 84 [bwc-badi-dates].

**The French Republican calendar**: twelve months of thirty days, each of
three *décades*, and five complementary days, six in a *sextile* year. The
Convention adopted it by the decree of 14 vendémiaire an II (5 October
1793), in force from the next day, 15 vendémiaire (6 October)
[frwiki-calendrier-republicain, decret-14-vendemiaire-an-ii], gave it its
final form by the decree of 4 frimaire an II (24 November 1793), which
fixes no date of entry into force of its own, and dated its era from
22 September 1792, the day of the autumn equinox that followed the
abolition of the monarchy. It was used until 10 nivôse an XIV
(31 December 1805); a sénatus-consulte of 22 fructidor an XIII
(9 September 1805) restored the Gregorian calendar from 1 January 1806
[frwiki-calendrier-republicain, decret-4-frimaire-an-ii].

The decree's article III fixes the year: each year begins at midnight
with the day on which the true autumn equinox falls for the Paris
Observatory [decret-4-frimaire-an-ii]. Its article X calls the four-year
period after which a day is "ordinarily" added the *Franciade*, and its
fourth year the *sextile* — which the equinox does not honour, since about
three times a century the sextiles are five years apart. The astronomers
Delambre, Lalande and Laplace saw the conflict, and on 19 floréal an III
(8 May 1795) Gilbert Romme, who had presented the calendar in 1793, read
to the Committee of Public Instruction a report proposing a fixed rule in
its place: a sextile every fourth year from An IV, except the century
years not divisible by four hundred, and except every four-thousandth
year. The report went no further: Romme was dead within weeks, after the
insurrection of prairial, and his successor did not put it to the
Convention [frwiki-calendrier-republicain,
wikipedia-french-republican-calendar].

## How it works

**Naw-Rúz.** Bahá'u'lláh's rule, quoted in the letter, is that Naw-Rúz
falls on the day the Sun enters Aries even should this occur a minute
before sunset [uhj-2014-07-10]. With a day that begins at sunset that
is the Badíʿ day, sunset to sunset at Tehran, in which the March equinox
falls, named, as the World Centre's table names it, by the Gregorian day it
ends in [bwc-badi-dates]. Reingold and Dershowitz state the same rule as
the first day whose sunset at Tehran finds the Sun past 0° of longitude,
with Tehran at 35.696111° N, 51.423056° E and a sea-level horizon
[reingold2018code, `bahai-location`, `bahai-sunset`,
`astro-bahai-new-year-on-or-before`].

**The year.** Ayyám-i-Há is as long as it takes to reach the next
Naw-Rúz: a year is 361 + 4 or 361 + 5 days, and which follows from where
two equinoxes fall, not from a cycle [uhj-2014-07-10].

**The Twin Holy Birthdays.** Count eight new moons from Naw-Rúz; the Birth
of the Báb is the first day after the Badíʿ day containing the eighth, and
the Birth of Bahá'u'lláh the day after that [uhj-2014-07-10]. Reingold and
Dershowitz count from the sunset that *ends* Naw-Rúz [reingold2018code,
`birth-of-the-bab`]; the module counts from the one that begins it. The two
differ only when a new moon falls within the day of Naw-Rúz itself.

**1 vendémiaire.** The day, midnight to midnight in true (apparent solar)
time at the Paris Observatory, in which the September equinox falls
[decret-4-frimaire-an-ii]. Reingold and Dershowitz state it as the first
day at whose closing apparent midnight the Sun is past 180°, with the
Observatory at 48°50′11″ N, 2°20′15″ E, 27 m [reingold2018code, `paris`,
`midnight-in-paris`, `french-new-year-on-or-before`]. The year is 365 or
366 days according to where the next equinox falls, and the sixth
complementary day, the *jour de la Révolution*, exists in the long ones
[decret-4-frimaire-an-ii].

**Romme's arithmetic.** Year *y* is sextile when `y mod 4 = 0`, `y mod 400`
is not 100, 200 or 300, and `y mod 4000 ≠ 0`; the mean year is
365.24225 days [frwiki-calendrier-republicain; reingold2018code,
`arithmetic-french-leap-year?`]. The equinox made An III, VII and XI
sextile; the rule makes An IV, VIII and XII sextile, so An IV, VIII and XII
begin a day earlier by the rule than they did in fact, and the two agree
again a year later [wikipedia-french-republican-calendar].

**The arithmetic Badíʿ calendar.** Before 172 BE the West kept Naw-Rúz on
21 March, and Ayyám-i-Há had five days when the following Gregorian year
was a leap year, so that the Badíʿ year had the length of the Gregorian
year it ended in [reingold2018code, `bahai-new-year`].

**Worked example: Naw-Rúz 183 BE.** The March equinox of 2026 falls at
14:45:55 UT on 20 March, 18:15:55 in Tehran's standard time of UTC+3:30.
Tehran's sunset that evening, against a sea-level horizon, is at 14:45:49
UT. The equinox comes six seconds *after* the sunset, so it falls in the
Badíʿ day that began at that sunset and ends at the next: Naw-Rúz 183 BE is
21 March 2026, as the World Centre's table has it [bwc-badi-dates], and the
Birth of the Báb is 10 November, the table's date too. It is the sharpest
row in the table, and two modelling choices decide it:

- *ΔT.* The equinox is computed in dynamical time and converted to
  Universal Time by ΔT. With the published ΔT of 2026, about 69.1 seconds
  [usno-deltat, usno-deltat-preds], the equinox falls six seconds after
  the sunset. With the Espenak–Meeus polynomial [espenak-meeus-2006],
  five seconds larger, it would fall under a second before, and the model
  would say 20 March; `hc-astro` takes ΔT from the measurements over this
  span, and the polynomial only outside it.
- *The horizon.* Tehran stands about 1 100 m high, the elevation Reingold
  and Dershowitz give the city in their `tehran`, used for the Persian
  calendar; a horizon dipped by that height puts the sunset about five
  minutes later, the equinox before it, and Naw-Rúz on the 20th, against
  the table. Their `bahai-location` has the Bahá'í coordinates with an
  elevation of 0 m [reingold2018code], and so does the module.

Even so the model's call is six seconds against a sunset good to a minute
or two, so the test names 183 BE as a row the model does not claim. The
other such row is 216 BE: the equinox of 2059 at 14:43:59 UT, 1.8 minutes
before sunset, gives 20 March 2059, which is also the table's date.

**Worked example: 1 vendémiaire An IV.** The September equinox of 1795
falls at 02:27:59 UT on 23 September. The apparent midnight at the Paris
Observatory that opens 23 September is at 23:42:55 UT on the 22nd — Paris
is 9 minutes 21 seconds east of Greenwich, and the equation of time in late
September puts the Sun's midnight about eight minutes earlier still. The
equinox is 2 hours 45 minutes into that day, so 1 vendémiaire An IV is
23 September 1795, which makes An III a year of 366 days, sextile, as it
was kept. By Romme's rule An III is common and An IV begins on
22 September 1795.

**The undecidable year.** A rule that depends on which side of midnight an
instant falls is only as good as the instant. Romme's report of 1795 made
the case with An CXLIV, whose equinox the astronomers put twenty seconds
before midnight, well within the three or four minutes their tables were
good to [wikipedia-french-republican-calendar,
frwiki-calendrier-republicain]. The model today puts that equinox, in
1935, at 23:38:07 UT on 23 September, 4.8 minutes before the apparent
midnight that ends the day, so it decides 1 vendémiaire CXLIV as
23 September 1935 with a margin larger than its tolerance. Within the
range the module converts, to Gregorian 3000, three years fall closer than
its one-minute tolerance — An CCCXXX (2121), CDLXXXVII (2278) and MXXXVII
(2828) — and those are the model's to decide and nobody else's, since no
authority computes the calendar any more. For the Badíʿ calendar the
authority exists, and eight years between 1 and 1157 BE fall within its
three-minute tolerance of sunset; two of them, 183 and 216, are in the
table, which settles them.

## What is carried

| Identifier | Crate | Rule | Range |
| --- | --- | --- | --- |
| `bahai-astronomical` | `hc-calendars-equinox` | The 2015 rule, at Tehran's sea-level sunset, for any year | 1 BE to the Naw-Rúz of Gregorian 3000 |
| `bahai` | `hc-calendars-solar` | The calendar as kept: the arithmetic rule to 171 BE, the World Centre's table for 172–221 BE | To 19 March 2065; refuses after |
| `bahai-arithmetic` | `hc-calendars-solar` | 21 March, the Gregorian year's length, for any year | 1 to 9 999 BE |
| `french-republican-equinox` | `hc-calendars-equinox` | Article III: the equinox at the Paris Observatory in true time | An I to the new year of Gregorian 3000 |
| `french-republican-arithmetic` | `hc-calendars-solar` | Romme's rule of 1795 | An I to 9 999 |

**Why separate names.** Each pair is one calendar under two rules that give
different days — Naw-Rúz 173 BE is 20 March 2016 by the equinox and
21 March by the Western rule; An IV begins on 23 September 1795 by the
equinox and on the 22nd by Romme — so, under policy §5, they are
calendars with names, not one calendar with a switch. The name says which
rule, and the arithmetic ones say *arithmetic*.

**Why a different crate.** `hc-calendars-solar` draws its line at
counting: nothing there asks where the Sun is, so nothing there pays for
an ephemeris. The equinox calendars ask, through `hc-astro`'s solar
longitude, sunset and apparent midnight, and they are solar, so they are
the equinox crate's, selected by the `equinox` feature of
`hyper-calendar`. The date types, `BahaiDate` and `FrenchRepublicanDate`,
and the calendars' own month and *décade*-day names are the solar
crate's, shared, so a date converts between the variants of one calendar
without translation.

**What each carries.** The Badíʿ calendars carry the nineteen months and
Ayyám-i-Há as month 0, the Váḥid and Kull-i-Shayʼ cycles, the fast, and a
day boundary at sunset, `DayBoundary::Sunset(DayNaming::ByEnd)`; a date
names the Gregorian day the Badíʿ day ends in. The astronomical one also carries the Twin Holy Birthdays and
`new_year_margin`, the minutes between the equinox and the deciding
sunset. The French calendars carry the twelve months with the
complementary days as month 13, the *décade* and its day names, and the
French one `new_year_margin` to the nearer apparent midnight.

**What neither carries.** The Badíʿ calendar as kept in Iran and the East
before 172 BE, where Naw-Rúz followed the Iranian equinox day; the Bahá'í
holy days as observances, which are `hc-holiday`'s `BAHAI` tradition,
dated on the calendar as kept; the French decimal time of article XI; the saints' and
plants' names of each day; the Paris Commune's brief revival of 1871.
`bahai-astronomical` before 172 BE and `french-republican-equinox` after
An XIV are proleptic: 1 BE begins on 20 March 1844 by the equinox, the day
before the Naw-Rúz the calendar was founded on.

## Accuracy

The equinox comes from `hc-astro`'s VSOP87 solar series and so is good to
seconds; the deciding clock is good to what it can be placed to. Each
module states that as a tolerance: three minutes for a Tehran sunset,
which depends on refraction and the horizon an almanac assumes, and one
minute for a Paris apparent midnight, which does not. A year whose
equinox falls within the tolerance is decided by the model, and the tests
say so rather than claim it.

| Check | Test | Result |
| --- | --- | --- |
| The fifty Naw-Rúzes of the World Centre's table, 172–221 BE, and the length of Ayyám-i-Há | `every_naw_ruz_of_the_world_centres_table_the_model_can_decide_is_reproduced` | 48 of 50 claimed and reproduced; 183 and 216 BE within tolerance, named, and reproduced |
| The fifty Twin Holy Birthdays of the table | `every_twin_birthday_of_the_world_centres_table_is_reproduced` | 50 of 50 |
| The margin's sign and size in a clear year each way, 172 and 181 BE | `the_margin_is_measured_from_sunset` | Holds |
| The fourteen new years France kept, An I–XIV, each by more than the tolerance | `the_fourteen_years_france_kept_begin_where_the_record_says` | 14 of 14 |
| The decree's two observed equinoxes, 9:18:30 on 22 September 1792 and 3:11:38 in the afternoon of 22 September 1793, in true time at the Observatory | `the_decrees_two_observed_equinoxes_are_the_models_in_true_time` | Within 52 and 48 seconds |
| The sextile years among them are III, VII and XI | `the_sextile_years_are_the_third_seventh_and_eleventh` | Holds |
| Romme's rule begins An IV, VIII and XII a day earlier, and agrees otherwise | `romme_and_the_equinox_part_company_in_the_fourth_eighth_and_twelfth_years` | Holds |
| The arithmetic Western calendar and the equinox part company in 173 BE | `before_172_be_the_rule_is_proleptic_and_says_so_by_its_name` | Holds |

The kept calendar's own tests check that the table's two columns, the
Naw-Rúz and the Ayyám-i-Há, agree row by row, which is the check on the
transcription.

The decree gives the two equinoxes the astronomers had observed, "pour
l'observatoire de Paris" [decret-4-frimaire-an-ii]. The model puts them at
9:17:38 and 3:12:26 in true time at the Observatory, within a minute of
both. Checked for this document and not by a test: in Paris mean time the
model would put them at about 9:10 and 3:05, so the decree's times are
true times, as article III's rule is.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [uhj-2014-07-10] | The 2015 rule: Tehran, the equinox, Ayyám-i-Há, the Twin Holy Birthdays, the start at the sunset of 20 March 2015, the sunset rule quoted from Bahá'u'lláh, the day at sunset | Yes, 2026-09-26 |
| [bwc-badi-dates] | The table of 172–221 BE: the Naw-Rúzes, the Twin Holy Birthdays, Ayyám-i-Há; who prepared it and from what data | Yes, 2026-09-26, as a rendered image, the text layer's digits being unreadable |
| [decret-4-frimaire-an-ii] | Articles I, III, IV, V, VII, IX, X and XI: the era, the new-year rule, the two observed equinoxes, the months, the complementary days, the *Franciade*, the *jour de la Révolution*, decimal time | Yes, 2026-09-26, in Wikisource's transcription of the Imprimerie nationale print of 1793 |
| [romme-an-iii-sextiles] | The proposed fixed rule and the An CXLIV example | Not read directly; read through [frwiki-calendrier-republicain], which quotes it, and [wikipedia-french-republican-calendar] |
| [romme-1793-rapport] | The calendar's first presentation, 20 September 1793 | Not read |
| [decret-14-vendemiaire-an-ii] | The calendar's adoption, 5 October 1793 | Not read: Gallica answered with a bot check on 2026-09-26; read through [frwiki-calendrier-republicain] |
| [frwiki-calendrier-republicain] | The decrees of 14 vendémiaire, 4 frimaire and 22 fructidor, the entry into force on 15 vendémiaire an II, Romme's 1795 report and its rule, the mean year, Delambre's objections, the dates of use | Yes, 2026-09-26 |
| [wikipedia-french-republican-calendar] | The rule's proposal date, the sextiles of III, VII and XI, An 144's predicted equinox, the fourteen new years | Yes, 2026-09-26 |
| [reingold2018code] | `bahai-location`, `bahai-sunset`, `astro-bahai-new-year-on-or-before`, `birth-of-the-bab`, `bahai-new-year`, `tehran`, `paris`, `midnight-in-paris`, `french-new-year-on-or-before`, `arithmetic-french-leap-year?` | Yes, 2026-09-26 |
| [reingold2018] | The book those functions come from | Not read directly |
| [usno-deltat], [usno-deltat-preds] | ΔT in 2026 | Read for `hc-astro` on 2026-09-25; not re-read here |
| [espenak-meeus-2006] | The ΔT polynomial against which 2026's measured ΔT is compared | Read for `hc-astro` on 2026-09-26; not re-read here |

The solar module's statement that the complementary days were the
*sansculottides* in the decree of 1793 and the *jours complémentaires*
from An III was not checked against a source here.

## Code

`crates/hc-calendars-equinox/src/bahai.rs`,
`crates/hc-calendars-equinox/src/french_republican.rs` and
`crates/hc-calendars-equinox/src/places.rs`; the siblings
`crates/hc-calendars-solar/src/bahai.rs`,
`crates/hc-calendars-solar/src/bahai_kept.rs` and
`crates/hc-calendars-solar/src/french_republican.rs`. Anchors in the
equinox modules:
`every_naw_ruz_of_the_world_centres_table_the_model_can_decide_is_reproduced`,
`every_twin_birthday_of_the_world_centres_table_is_reproduced`,
`the_margin_is_measured_from_sunset`,
`before_172_be_the_rule_is_proleptic_and_says_so_by_its_name`,
`the_fourteen_years_france_kept_begin_where_the_record_says`,
`the_decrees_two_observed_equinoxes_are_the_models_in_true_time`,
`the_sextile_years_are_the_third_seventh_and_eleventh`,
`romme_and_the_equinox_part_company_in_the_fourth_eighth_and_twelfth_years`;
in the crate root, `the_dynamic_leap_year_is_the_module_rule`. In the
siblings: `the_two_columns_of_the_table_agree_row_by_row`,
`nothing_is_answered_past_the_table`,
`naw_ruz_is_pinned_to_the_twenty_first_of_march`,
`the_fifth_intercalary_day_appears_before_a_gregorian_leap_year`,
`the_arithmetic_rule_diverges_from_the_equinox_at_an_iv`,
`the_century_and_millennium_exceptions_are_both_exercised`.
