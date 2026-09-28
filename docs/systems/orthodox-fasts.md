# The Eastern Orthodox fasts: the four seasons, the weekly fasts and the fast-free weeks

Backs `hc-holiday`'s `orthodox_fasts`, with the two reckonings
`orthodox-fasts` (the Julian calendar) and `orthodox-fasts-revised-julian`
(the Revised Julian calendar).

## What it is

The Eastern Orthodox churches keep many fast days. A fast day means
abstaining from some foods; which foods depends on the day and on the rule
a parish follows. The days themselves are set by one scheme:

- four **fasting seasons**: Great Lent with Holy Week before Pascha, the
  Apostles' Fast before the feast of Peter and Paul, the Dormition Fast
  before 15 August, and the Nativity Fast before Christmas;
- the week before Great Lent, when meat is not eaten but the Wednesday and
  Friday fasts are lifted (the OCA calls it the *Meatfast* and lists it
  among its fasting seasons; the Moscow Patriarchate's calendar read here
  calls it "Cheesefare week (Maslenitsa) - fast-free" and adds "Meat is
  excluded");
- **Wednesdays and Fridays** all year round;
- three **one-day fasts**: the eve of Theophany (5 January), the Beheading
  of John the Baptist (29 August) and the Exaltation of the Cross
  (14 September);
- four **fast-free periods**, when even Wednesday and Friday are not
  fasted: Christmas to the eve of Theophany, the week after the Sunday of
  the Publican and the Pharisee, Bright Week after Pascha, and the week
  after Pentecost.

The churches do not agree on the calendar under the fixed dates. Some date
them in the Julian calendar. Others have used the Revised Julian calendar,
which gives the Gregorian dates until 2800, since they adopted it; which
church did so when is in the sources of the
`christian-orthodox-revised-julian` table. Both keep Pascha by the Julian
computus. So the two reckonings share every date counted from Pascha and
differ, by thirteen days until 2100, on every fixed date. The table
[`christian-orthodox`](../observances.md) and its Revised Julian twin make
the same split for the feasts.

## How it works

The OCA's outline "Fasting & Fast-Free Seasons of the Church" gives the
whole scheme [oca-fasting-seasons]. With *P* for Pascha:

| Period | Kind | First day | Last day |
| --- | --- | --- | --- |
| Christmastide | fast-free | 25 December | 4 January |
| Week after the Publican and the Pharisee | fast-free | *P* − 69 | *P* − 64 |
| Bright Week | fast-free | *P* + 1 | *P* + 6 |
| Trinity Week | fast-free | *P* + 50 | *P* + 55 |
| Meatfast (Cheesefare week) | meat excluded | *P* − 55 | *P* − 49 |
| Great Lent and Holy Week | fast | *P* − 48 | *P* − 1 |
| Apostles' Fast | fast | *P* + 57 | 28 June |
| Dormition Fast | fast | 1 August | 14 August |
| Nativity Fast | fast | 15 November | 24 December |
| Eve of Theophany | fast | 5 January | 5 January |
| Beheading of John the Baptist | fast | 29 August | 29 August |
| Exaltation of the Cross | fast | 14 September | 14 September |

The Meatfast is the one period of its kind. The two sources name it
differently and state one rule. The OCA's outline lists "Meatfast - Monday
after the Sunday of Last Judgment through Cheesefare Sunday" under
"Fasting Seasons", and says of its own calendar that where no fast is
marked "all foods may be eaten (except during Cheesefare Week, when meat is
forbidden for every day)". The passage of *The Lenten Triodion* (Mother
Mary and Kallistos Ware, 1978, pp. 35–37) that the outline quotes adds
that "in the week before Lent, meat is forbidden, but eggs, cheese and
other dairy products (as well as fish) may be eaten on all days, including
Wednesday and Friday" [oca-fasting-seasons]; the book itself was not read.
The Holy Trinity calendar marks Wednesday 26 February 2025 "Cheesefare
week (Maslenitsa) - fast-free. Tone two. Maslenitsa. Meat is excluded",
and every other day of the week the same way, all seven read
[holy-trinity-calendar]. So
the week is not a fast in the sense the weekly fasts are, which neither
source keeps in it, and it is not free of abstinence either: meat is
excluded on all seven days. The library carries it as that third kind.

The outline states the periods in the Church's own terms, and the offsets
are what those terms count to. The Sunday of the Publican and the Pharisee
is the tenth Sunday before Pascha, *P* − 70, so its week is the Monday to
Saturday after; the Holy Trinity calendar marks Wednesday 12 February 2025,
*P* − 67, as "A fast-free week" [holy-trinity-calendar]. The Meatfast runs from the Monday
after the Sunday of the Last Judgment, *P* − 56, "through Cheesefare
Sunday", *P* − 49. Great Lent begins on the "1st Monday of Great Lent",
*P* − 48, and runs "through Great and Holy Saturday". Bright Week is "the
week after Pascha until St Thomas Sunday". Trinity Week is "the week after
Pentecost until the Saturday before All Saints Sunday". The Wednesdays and
Fridays are fast days "except for Fast-Free Weeks" [oca-fasting-seasons].

The Apostles' Fast begins on the Monday after All Saints' Sunday, the
second Monday after Pentecost, and ends on 28 June, the eve of Peter and
Paul [wikipedia-apostles-fast]. Its start is counted from Pascha and its
end is a fixed date, so its length changes every year. On the Julian
reckoning it lasts from 8 to 42 days [wikipedia-apostles-fast]. On the
Revised Julian reckoning the same end falls thirteen days earlier. When
Pascha is late enough, the fast shrinks to one day, and in some years it
does not happen at all. The Patriarchate of Antioch, which keeps the
Revised Julian calendar, says so outright: "If this fast coincides with the
feast directly, it is eliminated" [antioch-apostles-fast]. In 2024 it
did not happen: Pascha was on 5 May, the Monday after All Saints' was
1 July, and 29 June came first [trueorthodox-apostles-2024]. The same
happened in 2002 [orthochristian-apostles-fast].

A day is in the first period of the table it falls in, with the fast-free
periods first. A Wednesday or a Friday in no period is a fast day. Any
other day is not. A fast day abstains from meat and more; a day of the
Meatfast from meat alone; any other day from nothing.

**Worked example.** Pascha in 2025 was on 20 April (Gregorian). Great Lent
began 48 days earlier, on Monday 3 March, and Bright Week ran from
21 to 26 April, so Bright Wednesday, 23 April, was fast-free. Pentecost was
on 8 June and All Saints' on 15 June, so the Apostles' Fast began on
Monday 16 June on both reckonings. On the Julian reckoning it ended on
28 June Julian, which is 11 July Gregorian: 26 days. On the Revised Julian
reckoning it ended on 28 June: 13 days. In 2027 Pascha is on 2 May, so
All Saints' falls on 27 June. On the Revised Julian reckoning the fast is
the single day of Monday 28 June.

## What is carried

In `hc_holiday::orthodox_fasts`:

- `Reckoning`, the two reckonings as a table: `Reckoning::JULIAN`
  (`orthodox-fasts`) and `Reckoning::REVISED_JULIAN`
  (`orthodox-fasts-revised-julian`). They differ only in the calendar their
  fixed dates are read in.
- `Period`, the twelve periods of the table above, with their identifiers,
  kinds and bounds, in the order a day is tested against them. A kind is
  `PeriodKind::Fast`, `PeriodKind::FastFree` or, for the Meatfast alone,
  `PeriodKind::MeatExcluded`.
- `status`: whether a day is in a period, is a weekly fast, or is neither.
  `abstinence` answers what it abstains from, `Abstinence::Nothing`,
  `Abstinence::Meat` or `Abstinence::Fast`, and `is_fast_day` whether it is
  a fast day, which a day of the Meatfast is not.
- `span`: the first and last day of a period in a year of the reckoning, or
  nothing when the period does not happen that year, which only the
  Apostles' Fast does.

The years are those whose Pascha the Julian computus gives, 326 to 4099,
as `computus` computes it. Outside them there is no answer.

Not carried:

- **The food rules of a fast day.** Which foods are allowed on which fast
  day (the OCA's "wine and oil", "fish allowed", the monastic "strict
  fast") differs between churches and parishes and changes when a feast
  falls within a fast. Only the three degrees above are carried.
- **Feasts that lift a weekly fast.** Some churches do not fast on a
  Wednesday or Friday that is a great feast, such as the Transfiguration or
  the Meeting of the Lord. The OCA outline does not say so, and no rule
  that says which feasts was read.
- **Which church keeps which reckoning in a given year.** The dates of
  adoption are as in the `christian-orthodox-revised-julian` table's
  sources. The two reckonings here are conventions with no years.
- The Armenian, Coptic and Ethiopian fasts, which follow schemes of
  their own: they are four more reckonings of the same module, with
  their own periods, in [oriental-fasts.md](oriental-fasts.md).

## Accuracy

The scheme is arithmetic, so it is exact to the sources. It is checked
against:

- the Holy Trinity Russian Orthodox Church's calendar (Moscow Patriarchate,
  Julian calendar) [holy-trinity-calendar], on 34 days of 2025 and 2026: the
  beginning of Great Lent and Holy Saturday, Bright Wednesday, the
  fast-free Wednesday and Friday of Trinity Week, the first and last days of
  the Apostles' and Dormition Fasts and the days after each, the Beheading
  and the Exaltation, the eve and first day of the Nativity Fast and its
  last day, Christmastide, the eve of Theophany, the fast-free Wednesday of
  the week after the Publican and the Pharisee, the fasting Wednesday of the
  week after it, and nine days of Cheesefare week, not fast days and
  excluding meat: all seven of 2025, Monday to Cheesefare Sunday, and the
  Wednesday and Friday of 2026. Every day agrees.
- the OCA's daily pages (Revised Julian calendar) [oca-daily-readings]: the
  "Beginning of the Great Fast" on 3 March 2025 and the "Beginning of the
  Apostles Fast" on 16 June 2025 and 28 June 2027.
- the account of 2024 [trueorthodox-apostles-2024]: no Apostles' Fast on
  the Revised Julian reckoning.

## Sources

- [oca-fasting-seasons]: the whole scheme, with the dates of each season,
  fast-free week and one-day fast, and the rule of Cheesefare week, in its
  own words and in those of *The Lenten Triodion* it quotes. Read
  2026-09-27. It gives no author or date.
- [wikipedia-apostles-fast]: the start of the Apostles' Fast, its end,
  its length of 8 to 42 days on the Julian reckoning, and that it may
  not happen on the Revised Julian one. Read 2026-09-27. It cites no source
  for the last statement.
- [antioch-apostles-fast]: the Patriarchate of Antioch on the fast's
  length, "between one day and 6 weeks", and its elimination. Read
  2026-09-27.
- [orthochristian-apostles-fast]: the fast ending before it begins in 2002.
  Read 2026-09-27.
- [trueorthodox-apostles-2024]: no Apostles' Fast on the Revised Julian
  reckoning in 2024. Read 2026-09-27. It is written by a Julian-calendar
  church against the Revised Julian calendar and is cited only for the
  date.
- [holy-trinity-calendar]: the day pages of the Julian check. Read
  2026-09-27.
- [oca-daily-readings]: the day pages of the Revised Julian check. Read
  2026-09-27.

## Code

`crates/hc-holiday/src/orthodox_fasts.rs`. The tests that anchor it:
`the_julian_reckoning_agrees_with_a_moscow_patriarchate_calendar`,
`cheesefare_week_excludes_meat_and_lifts_the_weekly_fasts`,
`the_revised_julian_fasts_begin_when_the_oca_says`,
`the_revised_julian_apostles_fast_vanished_in_2024`,
`the_apostles_fast_is_eight_to_forty_two_days_on_the_julian_reckoning`,
`the_two_reckonings_share_the_moveable_days` and
`no_answer_outside_the_computus`.

The WebAssembly and C exports `hc_orthodox_fast_on` and
`hc_orthodox_fast_seasons` write what a day is and the periods of a year
under a reckoning, from `hyper_calendar::holiday_lines`.
