# The liturgical year of the Assyrian Church of the East

Backs `hc-holiday`'s tradition table `church-of-the-east`.

## What it is

The Church of the East divides its year into periods of seven weeks,
*shawu'e*, rather than building it from a list of feasts. The division is
attributed to the Catholicos Išoʿyahb III, who codified the year in the
prayer book called the *Ḥudra*; its last revision was made at the Upper
Monastery near Mosul in 1250 [andrious-liturgical-year]. The Assyrian
Church of the East keeps the year on the Gregorian calendar: its
Patriarch decided in 1964 to leave the Julian calendar, and part of the
Church, now the Ancient Church of the East, separated over the decision
[wikipedia-ancient-church-of-the-east].

The Chaldean Catholic Church and the Syro-Malabar Church keep the same
East Syriac year in forms of their own. They are not this table.

## How it works

The Church's own account, by Rev. Tower Andrious, counts nine periods.
"Ideally, each counts seven weeks": seven periods of seven weeks, and two
of four, "in order for the liturgical year to begin and end in the form of
the Holy Cross" [andrious-liturgical-year]. The year runs:

| Period | Its weeks | Where it starts |
| --- | --- | --- |
| Annunciation (*Subara*) and Nativity | "four Annunciation Sundays and two Nativity Sundays" | the fourth Sunday before Christmas |
| Epiphany (*Denha*) | "four to nine weeks", by the date of Easter | 6 January |
| The Great Fast (*Sawma Rabba*) | "seven weeks prior to Easter" | Easter − 49 |
| The Resurrection (*Qyamta*) | seven weeks; it "begins on Easter Sunday, and ends at Pentecost" | Easter |
| The Apostles (*Shlihe*) | seven weeks | Pentecost, Easter + 49 |
| Summer (*Qaita*) | seven weeks; it "may lose one Sunday in exceptional cases" | Easter + 98 |
| Elijah and the Cross | seven Sundays, as few as five | Easter + 147, or a week earlier |
| Moses | "did not exceed four weeks" | after Elijah and the Cross |
| The Dedication of the Church (*Qudash Idta*) | four Sundays | four weeks before the Annunciation |

Three fixed points hold the year in place. The Feast of the Cross "happens
in The Church of the East precisely on the 13th of September", a day
before the Byzantine and Latin 14th; the Transfiguration is on 6 August;
and the Epiphany on 6 January [andrious-liturgical-year]. The Rogation of
the Ninevites, a three-day fast, "takes place on Monday, Tuesday and
Wednesday of the third week preceding the Great Lent", which is 69, 68
and 67 days before Easter.

**Elijah and the Cross.** Elijah must begin before the Cross: its first
Sunday "is the beginning of preparatory fasting for the feast of the
cross". "If the Feast of the Cross falls earlier or even on the first
Sunday of Elijah then ... the 6th and the 7th Sunday of the Summer merge
together in order to have the first Sunday of Elijah prior to the Feast of
the Cross" [andrious-liturgical-year]. Seven weeks of Summer after
Easter + 98 end at Easter + 147, and that is on or after 13 September when
Easter is 19 April or later; Elijah then begins at Easter + 140, which is
never later than 12 September. The Elijah passage names the Cross "on
September 14", against the same document's 13th; the 13th is the one the
Church's calendar keeps, and the one carried.

**Worked example: 2026.** Easter 2026 is 5 April on the Gregorian
computus.

1. The Rogation of the Ninevites: Easter − 69 = Monday 26 January, to
   Wednesday 28 January.
2. The Great Fast: Easter − 49 = Sunday 15 February.
3. The Resurrection: Sunday 5 April. The Apostles: Pentecost, Easter + 49
   = Sunday 24 May.
4. Summer: Easter + 98 = Sunday 12 July, the Feast of Nusardel.
5. Elijah: Easter + 147 = Sunday 30 August, before 13 September, so no
   Sunday of Summer is lost.
6. The Dedication of the Church: the fourth Sunday before the
   Annunciation's first. The first Sunday on or after 27 November is
   29 November, so the Annunciation begins then and the Dedication on
   1 November.

The Church's calendar for 2026 has every one of these days
[ace-liturgical-calendar].

## What is carried

- **`church-of-the-east`**, from 1965: the Epiphany, the three days of the
  Rogation of the Ninevites, the first Sunday of the Great Fast, Palm
  Sunday, Easter, the Ascension, Pentecost, the first Sunday of Summer,
  the Transfiguration, the first Sunday of Elijah, the Feast of the Cross,
  the first Sundays of the Dedication of the Church and of the
  Annunciation, and the Nativity. Every entry is religious and none is a
  day off. English names as the Church's calendar gives them, with its
  Syriac names beside them.
- **From 1965**, the year after the decision of 1964; the day the change
  took effect was not read, so 1964 is left out.

Not carried:

- **The Sundays of Moses.** They follow Elijah and the Cross, and the
  statement read does not fix where Elijah and the Cross end: it gives
  seven Sundays, and five when the Cross is early. The Church's calendar
  for 2026–2029 starts Moses seven weeks after Elijah in each year.
- **The order of the Sundays of Elijah and of the Cross.** The statement
  says that the fourth Sunday of Elijah always follows the Cross and that
  the second and third merge when only one Sunday of Elijah precedes it.
  The Church's calendar does something else in 2027 and 2028: it keeps
  the Sunday named "3rd Holy Cross, 6th Elijah" before the Feast of the
  Cross in 2027, and the second and third Sundays of Elijah after it in
  2028. Neither is modelled.
- **The numbering of the Sundays of Epiphany**, which the calendar fits to
  the weeks by keeping two on one day, and the Fridays of commemoration.
- **The saints' days.**
- **The Chaldean, Syro-Malabar and Ancient Church of the East
  calendars.**

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| The Rogation of the Ninevites, the Great Fast, the Resurrection, the Apostles, Summer, Elijah, the Dedication and the Annunciation of 2026–2029, and the Epiphany, the Transfiguration and the Cross in each year, against the Church's calendar [ace-liturgical-calendar] | `the_church_of_the_east_year_is_its_own_calendar_of_2026_to_2029` | 44 of 44 |
| Elijah before the Cross, Summer of six or seven weeks, the Dedication four weeks before the Annunciation, 1965–2100 | `elijah_begins_before_the_cross_in_every_year` | yes |
| Easter on 24 April 2011: Elijah on 11 September, Summer's last two Sundays merged | `elijah_begins_before_the_cross_in_every_year` | from the rule; no calendar of 2011 was read |

The calendar's Easters of 2026–2029 are all before 19 April, so no year it
covers exercises the merging of Summer's Sundays; that rule rests on the
statement alone. The calendar lists the Rogation of the Ninevites of 2028
on 7 January, a Friday; the rule and the calendar's own Great Fast of
27 February put the Monday on 7 February, and the test takes 7 February.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [andrious-liturgical-year] | The nine periods and their weeks, the Rogation of the Ninevites, the Feast of the Cross on 13 September, Elijah before the Cross, the Transfiguration on 6 August | Yes, the PDF in the Internet Archive's copy of 13 February 2026, 2026-09-27; the live file returned an error |
| [ace-liturgical-calendar] | Every Sunday and feast of 2026–2029, and the Syriac names | Yes, the English and Assyrian feeds, 2026-09-27 |
| [wikipedia-ancient-church-of-the-east] | The Gregorian calendar from 1964 and the schism over it | Yes, 2026-09-27 (secondary) |

## Code

`crates/hc-holiday/src/traditions.rs`: `CHURCH_OF_THE_EAST` and
`first_sunday_of_elijah`. Anchors in `crates/hc-holiday/tests/traditions.rs`:
`the_church_of_the_east_year_is_its_own_calendar_of_2026_to_2029`,
`elijah_begins_before_the_cross_in_every_year`.
