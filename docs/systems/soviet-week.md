# The Soviet revolutionary weeks, 1929–1940

Backs the identifier `soviet-week` in `hc-calendars-solar`.

## What it is

For eleven years the Soviet Union worked to a week that was not seven days
long. The calendar itself was untouched: the Gregorian months kept their
lengths, Pravda dated issues 31 January and 31 March and never 30 February,
and a thirty-day "Soviet revolutionary calendar" proposed in February 1930
was rejected [wikipedia-soviet-calendar]. What the decrees replaced was the
week by which work and rest were scheduled — first with a continuous
five-day week, the *nepreryvka*, then with a six-day week, the
*shestidnevka* — until the seven-day week returned in June 1940. The
seven-day week and its day names went on in newspaper mastheads and in the
countryside throughout [wikipedia-soviet-calendar].

## How it works

**The five-day week, from 1 October 1929.** Sovnarkom's decree of
26 August 1929 ordered the transition to continuous production "уже с
1929-1930 хозяйственного года", the economic year beginning 1 October
[sovnarkom1929a, art. 1]. Its rules came on 24 September 1929: a five-day
week, "четыре дня работы и один день отдыха", with each worker's rest day
"поочередно в различные дни" by schedule, not fewer than 72 rest days a
year, and work forbidden on the revolutionary days — 22 January, 1 and
2 May, 7 and 8 November — with the note "Революционные праздники не
включаются в счет рабочих недель": the holidays are not counted in the
working weeks [sovnarkom1929b, arts. 1, 2, 4, 6]. So the five days of the
week run continuously except across the five holidays, which stand outside
it, and a common year is 72 five-day weeks and five holidays — which is
why a worker has exactly 72 rest days. Each day of the week was the rest
day of one fifth of the workforce, named by a colour or a Roman numeral;
the colours differ between sources, and the 1931 pocket calendar lays the
360 non-holiday days out in five rows I to V [wikipedia-soviet-calendar].

**The six-day week, from 1 December 1931.** The decree of 21 November 1931
let institutions move to an interrupted week with "твердые выходные дни
по следующим числам месяца: 6, 12, 18, 24 и 30", and "вместо выходного
дня в конце февраля … 1 марта", from 1 December 1931 [sovnarkom1931,
arts. 4, 8]. The week is now read off the day of the month: days 1–6 are
the first week, and so on to 25–30; the 31st belongs to none and is a
working day; 1 March is a rest day for February's missing 30th. A 1935
calendar prints "22 октября — четвёртый день шестидневки"
[wikipedia-ru-shestidnevka].

**The seven-day week, from 27 June 1940.** The Presidium's decree of
26 June 1940 moved all state, cooperative and public enterprises "с
шестидневки на семидневную неделю, считая седьмой день недели —
воскресенье — днем отдыха" [presidium1940, art. 2].

**Worked example.** Which day of the five-day week was 8 March 1930? The
week began with day I on 1 October 1929. From then to 31 December 1929 are
92 days, of which 7 and 8 November are holidays: 90 counted days, eighteen
weeks, so 1 January 1930 is day I again. From 1 January to 7 March 1930 are
66 days, one of them the holiday of 22 January: 65 counted days, thirteen
weeks, so 8 March 1930 is day I. And 22 October 1935, under the six-day
week, is day 22 of its month: 22 − 18 = 4, the fourth day, as the 1935
calendar says.

## What is carried

- **Identifier** `soviet-week`, 1 October 1929 to 26 June 1940, refusing
  the days outside; the date is the Gregorian one. The periods are the
  table `PERIODS`, one row per decree, and the holidays `HOLIDAYS`.
- **Fields** `to_fields` adds `week-length` (5 or 6), `soviet-week-day`
  (1–5 or 1–6, and 0 outside the week: a holiday of the five-day week, the
  31st of the six-day), and `rest-day` (1 on the six-day week's common rest
  days, 1 March included).
- **Shape** the Gregorian twelve months, `five-day-week` named I to V,
  `six-day-week` numbered, and the seven-day `weekday`. The two Soviet
  weeks are cycles of their own kinds, as the Javanese *pasaran* is, since
  `weekday` is the seven-day week that stayed in use beside them.
- **Usage** between the two ends, sourced to the decrees.
- **Not carried**, with the reason:
  - *The colours*: sources disagree on them and on their order.
  - *The workplaces' own schedules*: some fifty continuous weeks of other
    lengths were in use by December 1929, the six-day week spread from the
    summer of 1931 ahead of the decree, a quarter of industry was still
    continuous in 1935, and some offices rested on the 31st or worked on
    1 March [wikipedia-soviet-calendar]. The calendar is the decrees'.
  - *The six-day week's holidays*: under it the revolutionary days are
    holidays inside the week, observances for `hc-holiday`, not a change
    to the week.
  - *Before 1 October 1929 and from 27 June 1940*, which are the plain
    Gregorian calendar and its seven-day week.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| Day I on 1 October 1929, 1 January 1930 and 1 January 1931; the November holidays outside the week | `the_five_day_week_runs_from_1_october_1929_and_skips_the_holidays` | all |
| Each of the five days is a rest day 72 times in 1930 (art. 2 of the decree of 24 September 1929) | `each_group_rests_72_days_in_1930` | all |
| 22 October 1935 the fourth day; rest on the 6th … 30th and 1 March; the 31st outside; 18 August the third rest day of August 1933 (Aviation Day) | `the_six_day_week_is_read_off_the_day_of_the_month` | all |
| Every day of the range round-trips and is its Gregorian date; the days either side refused | `the_calendar_is_the_gregorian_one_within_the_decrees` | 3 922 days |

That 1 October 1929 is day I is an inference, not a statement of any
source read: it is the first day of the economic year the decree names,
and it is the only start consistent with the 1930 colour calendar's order
"beginning 1 January" and the 1931 pocket calendar's grid, since both
1929's remaining 90 working days and 1930's 360 are whole weeks.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [sovnarkom1929a] | The transition from the economic year 1929–1930 | Yes, museumreforms.ru, 2026-09-26 |
| [sovnarkom1929b] | The five-day week, the 72 rest days, the holidays outside the weeks | Yes, museumreforms.ru, 2026-09-26 |
| [sovnarkom1931] | The six-day week's rest days, 1 March, 1 December 1931 | Yes, museumreforms.ru, 2026-09-26 |
| [presidium1940] | The seven-day week from 27 June 1940 | Yes, 1000dokumente.de, 2026-09-26 |
| [wikipedia-soviet-calendar] | The unchanged Gregorian months, the 1930 and 1931 calendars, the uneven adoption | Yes, 2026-09-26 |
| [wikipedia-ru-shestidnevka] | The 1935 calendar's fourth day; Aviation Day | Yes, 2026-09-26 |

## Code

`crates/hc-calendars-solar/src/soviet_week.rs`. Anchors:
`the_five_day_week_runs_from_1_october_1929_and_skips_the_holidays`,
`each_group_rests_72_days_in_1930`,
`the_six_day_week_is_read_off_the_day_of_the_month`. The periods are
`PERIODS`; the reckoning `week_day`.
