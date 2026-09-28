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

**Precedence.** When two liturgical days fall together, "Precedence in
liturgical days is governed exclusively by the following table" (no. 91):
twenty-eight places, from Christmas, Easter and Pentecost (1) to the
ferias of the IV class (28), with the particular calendars' patrons and
titulars at 12, 13, 19, 20 and 23. A few rules qualify it. The
Immaculate Conception takes precedence over the Sunday of Advent it
falls on (no. 15). A feast of the Lord of the I or II class on a Sunday of
the II class "takes the place of the Sunday with all rights and
privileges; accordingly the Sunday is not commemorated" (no. 16a); the
Purification "is considered a feast of the Lord", and so is the
Dedication of a church (Changes in the Breviary). The Vigil of Christmas
takes the place of the Fourth Sunday of Advent, which is not
commemorated (no. 30). A vigil of the II or III class is "omitted
entirely" on a Sunday, on a feast of the I class, or when its feast is
transferred or reduced to a commemoration (no. 33).

What happens to the day that gives way depends on what it is (nos.
95–99):

- a feast of the I class "is transferred to the nearest day following
  which is not I or II class". When "the feast of the Annunciation of our
  Lady must be transferred until after Easter, it is transferred, as to
  its proper place, to Monday after Low Sunday" (no. 96a). Several feasts
  transferred keep the order of the table, the first impeded first among
  equals (nos. 97–98);
- any other day is commemorated, or "omitted completely in that year".

A commemoration is privileged — a Sunday, a day of the I class, a day
within the octave of Christmas, the Ember Days of September, the ferias
of Advent, Lent and Passiontide (no. 109) — or ordinary. How many a day
allows is fixed by its class (no. 111): on a day of the I class none
"except a privileged one"; on a Sunday of the II class one, "namely of a
feast II class", and none if a privileged commemoration must be made; on
another day of the II class one; on a day of the III or IV class two.
The season is commemorated first and the rest in the order of the table
(no. 113). "The Office, Mass or commemoration of the season excludes
another commemoration of the season", and a Sunday excludes a feast of
the Lord and the reverse (no. 112b, c). St Paul is always commemorated in
the office of St Peter and the reverse, and the two prayers count as one
(no. 110). The ferias of the IV class "are never commemorated" (no. 26).

The rubrics end with a Table of Occurrence (p. 115) that sets these
outcomes out as a grid, "Office of 1, commemoration of 2 at Ld. and Vp."
and so on. In the text layer read its cells are not legible; the outcomes
carried are the numbered rubrics' above.

**The Sundays of the II class.** "All other Sundays not mentioned above"
are of the II class: the Sunday within the octave of Christmas, those
after the Epiphany, of Septuagesima, after Easter, after the Ascension
and after Pentecost. "The Sundays after Epiphany which are impeded by
Septuagesima are transferred until after XXIII Sunday after Pentecost":
with 25 Sundays after Pentecost the Sixth after Epiphany is the
twenty-fourth, with 26 the Fifth and Sixth, with 27 the Fourth to Sixth,
with 28 the Third to Sixth; "The Sunday which is set down as XXIV after
Pentecost is always put in the last place, omitting, if need be, any
others for which there happens to be no place" (no. 18). The ferias of
Advent from 17 to 23 December are of the II class, the other ferias of
Advent and those of Lent and Passiontide of the III (nos. 24–25).

**Worked example.** In 1962, the year of the Missal, Easter was on 22 April.
Septuagesima Sunday was 63 days earlier, 18 February, and Ash Wednesday was
7 March. The Seven Sorrows of our Lady fell on Friday 13 April, the Friday
after the I Sunday of Passiontide (8 April). New Year's Day was a Monday,
so no Sunday fell between 2 and 5 January, and the Holy Name was kept on
2 January. The Holy Family was on Sunday 7 January, Christ the King on
Sunday 28 October, and the First Sunday of Advent on 2 December. On
14 January the calendar gives St Hilary, a feast of the III class, with a
commemoration of St Felix; but 14 January 1962 was the Second Sunday
after Epiphany, of the II class, whose office it was, and which allows
only the commemoration of a feast of the II class, so both were omitted.
25 March was the Third Sunday of Lent, of the I class and place 6, which
impeded the Annunciation, place 11: a feast of the I class, it was
transferred to Monday 26 March, a feria of Lent of the III class, which
was commemorated in it as a privileged commemoration. On 29 June the
Sacred Heart (place 3) impeded Sts Peter and Paul (place 11), who went to
30 June, the Commemoration of St Paul, a feast of the III class; their
vigil on 28 June was omitted, as no. 33 omits the vigil of a feast
transferred.

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
  I class, 86 of the II, 181 of the III and 106 commemorations, counting
  the movable feasts and the Proper of Time below. A commemoration made on
  a feast's day is an entry of its own, after the feast. The titles are
  the English translation's, with its abbreviations: Bp. (bishop),
  Cf. (confessor), Doct. (doctor), M. (martyr), V. (virgin), App. (apostles);
- the four movable feasts above, and the leap-year and All Souls' rules;
- from the Proper of Time: the Sundays, ferias, vigils and days within the
  octaves of Easter and Pentecost that are of the I class; and of the II
  class, the Vigil of the Ascension, the ferias of Advent from 17 to
  23 December that are not Sundays, and every Sunday of the II class: the
  Sunday within the octave of Christmas, the Second to Sixth after
  Epiphany when they come before Septuagesima, Septuagesima, Sexagesima
  and Quinquagesima, the Second to Fifth after Easter, the Sunday after
  the Ascension, the Second to Twenty-third after Pentecost, the Third to
  Sixth after Epiphany resumed, and the Twenty-fourth and last, in the
  places no. 18 gives them;
- `SEASON`, the days that are every weekday of a season: the ferias of
  Lent and Passiontide and of Advent to 16 December (III class), the
  second to fourth days within the octave of Christmas (II class), and the
  ferias and the Saturday Office of our Lady (IV class);
- **precedence**: `precedence`, each liturgical day's place in the table
  of no. 91; and `ordo` and `office_on`, the office of every day of a year,
  with the commemorations made, the feasts transferred and the days
  omitted, by the rules above.

Not carried:

- **The Ember Days.** The text read names them but does not state how the
  weeks are dated, as the `rogation-roman-1960` table says. So an Ember
  Day is the feria of its season here, of the III or IV class instead of
  the II, and the privileged commemoration of the Ember Days of
  September is not made.
- **The Table of Occurrence** as a grid, which is not legible in the text
  read; the numbered rubrics' outcomes are carried instead.
- **Concurrence** (nos. 103–105), which arranges vespers between two
  days, and the exclusions of no. 112a and d, by the identity of a saint
  or a mystery, beyond those between the feasts of the Lord and a
  Sunday.
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

**The ordo** was checked against propria.org's "Catholic Ordo" for the
liturgical years 2019 to 2026 [propria-ordo], which prints each day's Mass,
its class and its commemorations (read 2026-09-29): 2,936 days of Advent
2018 to November 2026, the Ember Days, which are not carried, set aside.
On 2,856 the class of the office and the number of commemorations agree,
and so do the transfers of those years: St Joseph from the Fourth
Sunday of Lent to Monday 20 March 2023, the Annunciation from Monday of
Holy Week to Monday 8 April 2024, St John the Baptist from the Sacred
Heart to Saturday 25 June 2022 with no vigil on 23 June. Of the other 80:

- about 40 are how the ordo writes the day, not what it is: prayers that
  are not commemorations ("Pro Papa", the Propagation of the Faith, the
  Greater Litanies), a commemoration of several saints written with "and",
  an optional Mass printed in place of a commemoration (Our Lady of Mount
  Carmel, the Seven Sorrows), and five days whose class is a slip (a
  feria "3", the Fifth Sunday after Epiphany "3" in 2022);
- eight are Christmas, whose commemoration of St Anastasia the ordo makes
  "at dawn", in the second Mass only, which no rule read covers;
- the rest are commemorations the ordo does not make on a day of the II,
  III or IV class that allows them: St Matthias on Sexagesima Sunday 2019,
  St Peter of Alexandria, St Liborius, St Silvester and Sts Symphorosa and
  her Sons in some years and not in others, Sts Processus and Martinian
  with the Visitation in 2024–2026. The ordo is not consistent between
  years here, and the rubrics are followed;
- three follow the ordo's practice against a rubric: the Twenty-second
  Sunday after Pentecost commemorated in the Dedication of the Lateran on
  9 November 2025, which no. 16a forbids; St Andrew commemorated on the
  First Sunday of Advent 2025, a day of the I class, on which no. 111a
  allows only a privileged commemoration; and St Saturninus commemorated
  on 28 November 2026, the day before his own, which no. 94 does not
  allow.

Wikipedia's titles are not used, since they are not always the 1960
wording: for example, Wikipedia calls St Irenaeus a Doctor of the Church,
and the translation calls him "S. Irenaeus, Bp., M.". The translation's
title is the one carried.

## Sources

- [rubrics-1960]: the Code of Rubrics, nos. 6–36 and 91–114; the calendar
  of the Roman Breviary and Missal, pp. 98–110; the Table of Liturgical
  Days, pp. 111–114; the Table of Occurrence and its notes, pp. 115–117,
  the grid not legible; the Changes in the Breviary, on the Purification
  and the Dedication as feasts of the Lord. Read 2026-09-27 in the scan's
  text layer, and re-read in the same text 2026-09-29; the PDF itself was
  not opened again. The Latin in *Acta Apostolicae Sedis* 52 (1960) was
  not read.
- [propria-ordo]: the ordo of 2019–2026, the check on the office of every
  day. Read 2026-09-29, the HTML pages. A publisher's ordo, secondary to
  the rubrics.
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
`a_day_carries_its_feast_and_its_commemoration`,
`the_feasts_that_moved_in_1969_are_on_their_1960_days`,
`every_title_the_precedence_tables_name_is_a_day`,
`the_sundays_of_the_second_class_are_ordered_as_no_18_says`,
`the_ordo_is_the_published_ordo_on_the_days_that_test_the_rules`,
`an_impeded_feast_of_the_first_class_goes_to_the_nearest_free_day` and
`the_sunday_and_the_vigils_give_way_as_the_rubrics_say`.
