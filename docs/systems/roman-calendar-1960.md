# The General Roman Calendar of 1960: the 1962 Missal's calendar and its four classes

Backs `hc-holiday`'s `roman_calendar_1960`, the rule set
`roman-general-1960`.

## What it is

On 25 July 1960 John XXIII's motu proprio *Rubricarum instructum* approved
a new Code of Rubrics for the Roman Breviary and Missal, and with it a
revised General Roman Calendar [rubrics-1960]. The 1962 edition of the
Roman Missal prints that calendar. The Missal is still in use, under
*Summorum Pontificum* (2007) as limited by *Traditionis custodes* (2021)
[wikipedia-grc-1960], so the calendar is still kept where that Mass is
said.

The calendar of the 1969 reform, which `roman-general` carries in its 2002
form, replaced it everywhere else. The two differ in their ranks, in the
season of Septuagesima before Lent, and in the dates of many feasts.

## How it works

The Code of Rubrics ranks every liturgical day in one of four classes
[rubrics-1960]:

- **Sundays** are of the I class (Advent I–IV, Lent I–IV, Passiontide I–II,
  Easter, Low Sunday and Pentecost) or of the II class (all the others)
  (nos. 10–12).
- **Ferias** are of the I class (Ash Wednesday and the ferias of Holy
  Week), the II class (17 to 23 December and the Ember Days), the III class
  (the other ferias of Advent and Lent) or the IV class (all others)
  (nos. 22–25).
- **Vigils** are of the I class (Christmas and Pentecost), the II class
  (the Ascension, the Assumption, St John the Baptist, Sts Peter and Paul)
  or the III class (St Laurence) (nos. 29–32).
- **Feasts** are of the I, II or III class (no. 36). The feasts that the
  older calendar ranked as simples stayed **commemorations**
  [wikipedia-grc-1960]: a saint remembered by a prayer in the office and
  Mass of the day, with no day of its own.

The calendar prints each day of the year with its feast and class, and any
commemorations made on it. The Table of Liturgical Days lists every feast
of the I and II class of the universal calendar: nineteen feasts and two
other days (the octave-day of Christmas and All Souls' Day) of the I class,
and thirty-one feasts of the II class [rubrics-1960, pp. 111–114].

Four feasts in the calendar move with the week or with Easter:

| Feast | Class | Day |
| --- | --- | --- |
| The Holy Name of Jesus | II | the Sunday between the octave-day of Christmas and the Epiphany, or 2 January if there is none |
| The Holy Family | II | the first Sunday after the Epiphany |
| The Seven Sorrows of our Lady | Comm. | the Friday after the I Sunday of Passiontide |
| Christ the King | I | the last Sunday of October |

In a leap year St Matthias moves from 24 to 25 February and St Gabriel of
the Sorrowing Virgin from 27 to 28 February, as the note on February says.
All Souls' Day is moved to Monday 3 November when 2 November is a Sunday
(no. 96).

The Proper of Time follows Easter as in any Roman calendar. The rubrics
read give the feast of the Blessed Trinity as "the first Sunday after
Pentecost". They name the Ascension, Corpus Christi and the Sacred Heart
without their days. The offsets carried are those of the rest of this
crate's computus: Ascension 39 days after Easter, Corpus Christi 60, the
Sacred Heart 68.

**Worked example.** In 1962, the year of the Missal, Easter was on 22 April.
Septuagesima Sunday was 63 days earlier, 18 February, and Ash Wednesday was
7 March. The Seven Sorrows of our Lady fell on Friday 13 April, the Friday
after the I Sunday of Passiontide (8 April). New Year's Day was a Monday,
so no Sunday fell between 2 and 5 January, and the Holy Name was kept on
2 January. The Holy Family was on Sunday 7 January, Christ the King on
Sunday 28 October, and the First Sunday of Advent on 2 December. On
14 January the calendar gives St Hilary, a feast of the III class, with a
commemoration of St Felix.

### Where it differs from the calendar of 1969

A few examples, each of which `roman_calendar` carries in its 2002 form:

| Celebration | 1960 | 1969 onwards |
| --- | --- | --- |
| The Visitation | 2 July, II class | 31 May, feast |
| St Matthias | 24 February, II class | 14 May, feast |
| St Thomas, Apostle | 21 December, II class | 3 July, feast |
| St Thomas Aquinas | 7 March, III class | 28 January, memorial |
| The Queenship of our Lady | 31 May, II class | 22 August, memorial |
| The Immaculate Heart | 22 August, II class | the Saturday after the Sacred Heart, memorial |
| Christ the King | the last Sunday of October, I class | the last Sunday before Advent, solemnity |
| The Baptism of our Lord | 13 January, II class | the Sunday after 6 January, feast |
| The Precious Blood | 1 July, I class | not in the calendar |
| St Gabriel and St Raphael | 24 March and 24 October, III class | with St Michael on 29 September |

The season of Septuagesima, the three Sundays before Lent, is kept in
1960 and was dropped in 1969.

## What is carried

`roman_calendar_1960::CELEBRATIONS`, and the same days as the rule set
`GENERAL_ROMAN_CALENDAR_1960` (`roman-general-1960`), each a
`Kind::Religious` observance:

- every day the calendar prints, month by month, with its class: 53 of the
  I class, 41 of the II, 181 of the III and 106 commemorations, counting
  the movable feasts and the Proper of Time below. A commemoration made on
  a feast's day is an entry of its own, after the feast. The titles are
  the English translation's, with its abbreviations: Bp. (bishop),
  Cf. (confessor), Doct. (doctor), M. (martyr), V. (virgin), App. (apostles);
- the four movable feasts above, and the leap-year and All Souls' rules;
- from the Proper of Time: the Sundays, ferias, vigils and days within the
  octaves of Easter and Pentecost that are of the I class; the Vigil of the
  Ascension, of the II class; and Septuagesima, Sexagesima and
  Quinquagesima Sundays, of the II class, because they mark the season the
  1969 calendar dropped.

Not carried:

- **Precedence.** When two days fall together, the Table of Occurrence
  decides which is kept, and whether the other is commemorated or moved.
  That is not applied, so a day can list a feast that is not kept that
  year, as `roman-general` does. The transfer of an impeded feast of the
  I class (no. 96), and so of the Annunciation to the Monday after Low
  Sunday when it must be moved until after Easter, is not applied either.
- **The Ember Days.** The text read names them but does not state how the
  September week is dated, as the `rogation-roman-1960` table says. The
  II class ferias of 17 to 23 December and the II class Sundays after
  Epiphany and after Pentecost are also not listed.
- **The Litanies.** They are the `rogation-roman-1960` table.
- **Particular calendars** of nations, dioceses, orders and churches, and
  the options that *Cum sanctissima* (2020) allows for saints canonised
  after 1960 [wikipedia-grc-1960].

## Accuracy

The calendar was transcribed from the text layer of a scan of the English
translation [rubrics-1960, pp. 98–110]. Where the scan's text lost a class
(on some days of March to June, where the class stands in a column of its
own) or a day number (17 April, 18 September), the
class or day was taken from Wikipedia's transcription of the calendar
[wikipedia-grc-1960]. Where their spellings disagree (Modestus, Daria), the
spelling is Wikipedia's. The whole table was then compared day by day with
Wikipedia's: the class of each day's principal entry and the number of
commemorations agree on all the days either lists. The I and II class
feasts agree in number with the Table of Liturgical Days.

Wikipedia's titles are not used, since they are not always the 1960
wording: for example, Wikipedia calls St Irenaeus a Doctor of the Church,
and the translation calls him "S. Irenaeus, Bp., M.". The translation's
title is the one carried.

## Sources

- [rubrics-1960]: the Code of Rubrics, nos. 10–36 and 96; the calendar of
  the Roman Breviary and Missal, pp. 98–110; the Table of Liturgical Days,
  pp. 111–114. Read 2026-09-27 in the scan's text layer. The Latin in *Acta
  Apostolicae Sedis* 52 (1960) was not read.
- [wikipedia-grc-1960]: the check on every day's class and commemorations,
  the classes and days the scan's text lost, the simples that stayed
  commemorations, the calendar's present use, and *Cum sanctissima*. Read 2026-09-27. It cites *Acta Apostolicae Sedis*
  52 (1960), not read.
- [liturgyoffice-calendar] and the decrees `roman_calendar` cites, for
  the 1969 calendar in the comparison above.

## Code

`crates/hc-holiday/src/roman_calendar_1960.rs`. The tests that anchor it:
`the_classes_are_counted_as_the_module_states`,
`the_first_and_second_class_feasts_are_the_tables`,
`the_movable_days_of_1962_fall_where_the_rules_put_them`,
`a_leap_year_moves_st_matthias_and_st_gabriel`,
`all_souls_day_leaves_a_sunday_for_the_monday`,
`a_day_carries_its_feast_and_its_commemoration` and
`the_feasts_that_moved_in_1969_are_on_their_1960_days`.
