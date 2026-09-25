# South Korea's public holidays and the substitute holiday

Backs the `SOUTH_KOREA` table (`KR`) in `hc-holiday`, the engine's
collision rule, and through the table the Korea Exchange (`XKRX`), which
includes it.

## What it is

South Korea's public holidays are set by a presidential decree, the
관공서의 공휴일에 관한 규정 (Regulations on public holidays of government
offices), first promulgated in 1949 and amended some twenty times since
[kr-holiday-regulation-history]. Its Article 2 lists the days on which
government offices close; the list is what the rest of the country
follows, and since 2022 the 공휴일에 관한 법률 (Public Holidays Act,
법률 제18291호 of 7 July 2021) has stated it as an Act and left the
substitute-holiday rule to the decree [kr-holiday-act-2021]. The decree
has three kinds of entry, and the table carries each differently:

- **The named days** of Article 2, items 2 to 10: the four 국경일
  (삼일절 1 March, 광복절 15 August, 개천절 3 October, 한글날 9 October —
  since 11 May 2026 all five, 제헌절 17 July restored), 1 January, the
  three days of 설날 (the eve, the day and the day after the first of the
  first month), 부처님 오신 날 (the eighth of the fourth month), 노동절
  (1 May, from 2026), 어린이날 (5 May), 현충일 (6 June), the three days of
  추석 (the fourteenth to sixteenth of the eighth month) and 기독탄신일
  (25 December). Item 1 is Sunday itself, which matters below
  [kr-holiday-regulation-2026].
- **Election days**, item 10-2 since 대통령령 제19674호 of 6 September
  2006: the day of every election held under Article 34 of the 공직선거법
  because a term has run out — presidential every five years, National
  Assembly every four, local every four — is a public holiday without a
  further decision. An election held because of a vacancy is not, and is
  designated on its own if at all [kr-holiday-regulation-history].
- **Designated days**, item 11, "그 밖에 정부에서 수시 지정하는 날":
  a 임시공휴일 declared by Cabinet decision for one occasion, since the
  2021 amendment only after deliberation by the 국무회의. The 70th
  anniversary of liberation on 14 August 2015, a bridge day on 6 May 2016,
  the Chuseok bridge on 2 October 2017, 17 August 2020, the Chuseok bridge
  on 2 October 2023, 국군의 날 on 1 October 2024 and the Seollal bridge on
  27 January 2025 were all of this kind, and so were the two presidential
  elections that followed a vacancy, 9 May 2017 and 3 June 2025
  [wikipedia-ko-public-holidays, krx-market-closing].

**The 대체공휴일.** A holiday that falls on a weekend is lost unless the
law gives a day in its place. Korea has tried a next-day rule twice
before the present one. 대통령령 제1461호 of 27 March 1959 added to the
1949 decree the clause "일요일과 일요일 이외의 공휴일이 중복되는 때에는
그 익일도 공휴일로 한다" — when a Sunday and another holiday coincide, the
following day is a holiday too — which the text of 16 March 1960 still
carries and the text in force from 1 January 1961 (국무원령 제152호 of
30 December 1960) no longer does [kr-holiday-regulation-1959,
kr-holiday-regulation-1960, kr-holiday-regulation-1960-12]. Then
대통령령 제12616호 of 1 February 1989 brought in the 익일휴무제, which its
stated purpose describes as "연휴외의 공휴일이 겹칠 때에는 그 다음날도
공휴일로 함" — a holiday outside the 연휴, the runs of 신정, 설날 and
추석, that coincides with another earns the next day — and 제13155호 of
5 November 1990 abolished it from 1991, together with 국군의 날 and
한글날, "10월에 편중된 공휴일을 완화하고" [kr-holiday-regulation-history].
The decree's own text for 1989 was not read; a secondary account says the
rule produced one day, Monday 2 October 1989 after 국군의 날 on the Sunday,
and did not reach the 신정, 설날 and 추석 runs
[wikipedia-ko-public-holidays]. From 1991 to 2013 nothing replaced it. The
substitute holiday then came in three steps, each an amendment to Article
3 of the decree:

| Amendment | Decree | In force | Holidays that gained a substitute | Trigger |
| --- | --- | --- | --- | --- |
| 5 November 2013 | 대통령령 제24828호 | 5 November 2013, so first in 2014 | 설날, 추석 (items 4 and 9); 어린이날 (item 7) | Seollal and Chuseok: "다른 공휴일과 겹칠 경우", overlapping another public holiday — which includes Sunday, item 1, and not Saturday; Children's Day: "토요일이나 다른 공휴일", a Saturday or another holiday [kr-holiday-regulation-2017] |
| 4 August 2021 | 대통령령 제31930호 | 4 August 2021, first applied to 광복절 on Sunday 15 August 2021, kept on Monday the 16th | 삼일절, 광복절, 개천절, 한글날 (item 2) | A Saturday, a Sunday or another holiday; the Act's own transitional provision applied it "이 법 시행일 전이라도" to the three national days and Christmas of 2021 [kr-holiday-act-2021, korea-kr-substitute-holidays-2021] |
| 4 May 2023 | 대통령령 제33448호 | 4 May 2023 | 부처님 오신 날, 기독탄신일 (items 6 and 10 as then numbered) | A Saturday, a Sunday or another holiday [kr-holiday-regulation-history] |
| 30 April 2026 | 대통령령 제36290호 | 1 May 2026 for 노동절; 11 May 2026 for 제헌절 | 노동절, 제헌절 | The same, from the start [kr-holiday-regulation-2026] |

Two holidays never gained one: 신정 (1 January) and 현충일 (6 June). Nor
did the election days or the designated days, which are holidays for one
year and one occasion.

As in force from 11 May 2026 the rule reads, in Article 3 paragraph 1,
"제2조제2호부터 제10호까지의 공휴일이 다음 각 호의 어느 하나에 해당하는
경우에는 그 공휴일 다음의 첫 번째 비공휴일을 대체공휴일로 한다": the first
non-holiday after the holiday becomes the substitute. Its three cases are
the ones in the table — a Saturday or a Sunday for the national days,
Buddha's Birthday, Labour Day, Children's Day and Christmas; a Sunday
only for the Seollal and Chuseok days; and, for all of them, overlapping
another public holiday. Paragraph 2 adds that when two substitutes fall
on the same day the run continues to the next non-holiday after it, and
paragraph 3 that a substitute falling on a Saturday moves on again
[kr-holiday-regulation-2026].

## How it works

**The table.** Every named day is a rule with the years it was a holiday:
설날 from 1985 (the day alone, then called 민속의 날) and its eve and
following day from 1989; 추석's eve and following day from 1989; 한글날
to 1990 and again from 2013; 제헌절 to 2007 and again from 2026; 노동절
from 2026; and the days the decree dropped — 2 January to 1998 and
3 January to 1989 (the 1949 decree listed all three days of 신정,
제12616호 of 1989 kept two, 제15939호 of 18 December 1998 one), 식목일 to
2005 (with 사방의 날, 21 March, in its place in 1960 alone, by 제1568호's
부칙, and 식목일 back by 국무원령 제210호 of 27 February 1961; dropped from
2006 by 제18893호 of 30 June 2005) and 국군의 날 from 1976 to 1990 (by
제8235호 of 3 September 1976, whose text was not read; dropped by 제13155호)
[kr-holiday-regulation-1949, kr-holiday-regulation-1960,
kr-holiday-regulation-1961, kr-holiday-regulation-history]. 어린이날 and
부처님 오신 날 date from 제7538호 of 27 January 1975
[kr-holiday-regulation-1975] and 현충일 from 제1145호 of 19 April 1956, in
force 6 June 1956 [kr-holiday-regulation-1959]. Each rule that the
대체공휴일 reaches carries `substituted_from` with the first year of its
step — 2014, 2021, 2023 or 2026 — and the two that it never reaches, 신정
and 현충일, are `fixed_public`, which opts them out. The Seollal and
Chuseok days carry `substitute_on(SUNDAY_ONLY)`, their own trigger set,
because a Saturday never counted for them. Election days and designated
days are `kr_one_off` rules — `fixed_public`, bounded to one year — so
they are never moved and never trigger a substitute on their own. The
holidays the 익일휴무제 of 1989–90 could reach are split at 1990: the row
to 1990 substitutes whenever a policy is in force, which before 2014 is
those two years only, and the row from 1991 carries its step's year.

**The policies.** `KR_SUBSTITUTION` has two entries. The 익일휴무제 of
1989–1990: trigger Sunday, direction `Forward`, `skip_occupied` false and
`on_collision` false, which reproduces the one day the secondary account
gives and produces nothing for 개천절 falling on the day of Chuseok on
Wednesday 3 October 1990 — whether that day owed a substitute the sources
read do not say, and the table does not claim it. The 1959–60 clause is
not carried (below). The 대체공휴일 from 2014: trigger Saturday and Sunday,
direction `Forward`, `skip_occupied` true, `on_collision` true. The engine
(`evaluate` in `engine.rs`) takes the
year's occurrences in date order and, for each that is a day off, that
`substitutes_in` the year and whose trigger — the rule's own if it has one,
else the policy's — contains the day's weekday, or that the collision rule
marks, walks forward from the day to the first date that is neither in the
trigger set nor already occupied by a day off or by a substitute assigned
earlier in the pass. That single walk is paragraphs 1 to 3 of Article 3 at
once: "the first non-holiday after", the run past another substitute, and
the step over a Saturday.

**The collision rule.** "또는 다른 공휴일과 겹칠 경우" — two holidays on
one day owe a third. `collisions()` in `engine.rs` looks at each date's
occurrences: a date with *k* days off owes *k − 1* substitutes, and they
go to the occurrences that can be substituted in that year, the later in
the engine's (date, name) order first. Both halves matter:

- On 5 May 2025 Children's Day and Buddha's Birthday fell together, both
  substitutable, and Tuesday 6 May was the 대체공휴일.
- On 3 October 2017 the eve of Chuseok fell on 개천절. Chuseok's days had
  been substitutable since 2014; the national days were not until 2021.
  One substitute is owed; National Foundation Day cannot take it, so it is
  Chuseok's, and the walk from Tuesday 3 October passes Wednesday and
  Thursday, Chuseok's own days, to land on Friday 6 October. The Korea
  Exchange's closure list for 2017 has 2 October (a designated day), 3 to
  6 October and 9 October, which is what the engine gives
  [krx-market-closing].

**The exchange.** `KOREA_EXCHANGE` includes `SOUTH_KOREA` and receives its
days off — substitutes, election days and designated days with the rest —
and adds two of its own: 근로자의 날, 1 May, which was a paid day off for
employees under its own Act and not a public holiday until it became
노동절 in 2026, carried to 2025 and never moved off a weekend; and the last
weekday of the year. From 2026 the 1 May comes through the country's
table, once, and is substituted like it.

**Worked example: Chuseok 2025.** 추석, the fifteenth of the eighth month
of the `dangi` calendar, is Monday 6 October 2025, so the three days are
Sunday 5, Monday 6 and Tuesday 7 October. Friday 3 October is 개천절.
The eve falls on a Sunday; its trigger set is Sunday only, so it is
triggered. The walk starts at Monday 6 October, which is a day off,
Tuesday 7 October, which is one too, and stops at Wednesday 8 October,
which is neither a weekend day nor occupied. Monday and Tuesday trigger
nothing. Ask a calendar built for `SOUTH_KOREA` and 2025: `on` for
8 October returns one entry named "Chuseok" with `observed_for` Sunday
5 October and `is_substitute` true; `is_business_day` is false from
Friday 3 to Thursday 9 October, when 한글날 follows, and
`add_business_days` from Thursday 2 October by one lands on Friday
10 October. The Korea Exchange's list for 2025 closes 3, 6, 7, 8 and
9 October [krx-market-closing]. In the same year Seollal is Wednesday
29 January, the three days are Tuesday to Thursday, nothing is triggered,
and the Cabinet's designated Monday 27 January is a `kr_one_off` row
beside them.

For comparison, Seollal 2024 is Saturday 10 February: Friday 9, Saturday
10 and Sunday 11 February. Only the Sunday is triggered — the Saturday
day, 설날 itself, is not, since Saturday is not in its trigger set — and
the substitute is Monday 12 February, for the day after Seollal. One
substitute, not two, which is what the decree says and what the exchange
closed.

## What is carried

- **The named days**, as rules bounded to the years each was a holiday,
  Seollal and Chuseok dated in the `dangi` calendar at the Seoul meridian
  (which puts them a day from the Chinese dates a few times a century),
  the dropped days among them. The rules are not bounded below except
  where a text read here gives a start — Seollal from 1985 and 1989,
  2 and 3 January, 식목일, Constitution Day and Hangul Day from 1949,
  현충일 from 1956, Children's Day and Buddha's Birthday from 1975, 국군의
  날 from 1976 — so for years before a rule's start the table is silent
  rather than wrong, and for 1949 onward it answers with the days it has.
- **The 익일휴무제 of 1989–1990**, as a Sunday-only policy, and **the
  대체공휴일** in its three steps and the collision rule, from 2014.
- **Election days** for the thirteen elections from December 2007 to the
  local elections of 3 June 2026, each a one-year rule.
- **Designated days** from 2009, the first year the Korea Exchange's
  closure lists reach: nine of them, listed above. A day designated after
  the table was read is not carried, and a year is not reported as a gap
  for that: the table has no `Tabulated` rule, so `is_complete` is true
  for every year and a future designation is simply absent until it is
  entered.
- **Not carried, and why:**
  - 국제연합일, 24 October, in the texts of 1959 to 1975 and gone by 1985:
    the decree that dropped it (제8235호 of 1976, presumably, which added
    국군의 날) was not read, so its last year is not established.
  - The next-day clause of 1959–1960, "그 익일도 공휴일로 한다": it would
    have moved 식목일 on Sunday 5 April 1959, 3 January, 제헌절, 한글날 and
    Christmas on Sundays in 1960, and its scope — every holiday, the 신정
    run included — differs from the 1989 rule's; it is stated here and
    left for a reader of those two years, since the table's other days of
    that time (국제연합일, 사방의 날's regular 15 March) are not settled
    either.
  - Designated days before 2009: nothing to check them against.
  - The proviso to Article 2 that lets the head of an office set its own
    rest days in consultation with the Prime Minister, which is not
    nationwide.
  - By-elections and re-elections, which are not holidays.
  - 근로자의 날 before 2026, a day off for employees under the
    근로자의 날 제정에 관한 법률 and not a public holiday; the exchange's
    table carries it because the exchange closed.
  - Saturday as a rest day of government offices, which the decree does
    not list; the table's weekend policy is Saturday and Sunday, which is
    what business-day arithmetic needs.

## Accuracy

The regime is checked twice. `south_korea_holidays` and
`the_korean_substitute_holiday_covers_sundays_and_collisions` in
`crates/hc-holiday/tests/countries.rs` pin the decree's rules: the
Seollal and Chuseok days of 2024 and 2025, Hangul Day absent in 2000 and
Constitution Day absent in 2010, the 2026 additions and 1 May 2025 as a
working day, three election and designated days, the dropped days in
their first and last years and absent the year after (2 January 1998 and
not 1999, 3 January 1989 and not 1990, 식목일 in 1959, 1961 and 2005 and
not in 1960 or 2006, 사방의 날 on 21 March 1960, 국군의 날 in 1976 and 1990
and not in 1975 or 1991, 현충일 in 1956 and not 1955), and the substitutes
of Monday 12 February 2024 (a Sunday day of Seollal), Monday 3 March 2025
(a Saturday 삼일절, under the 2021 step), 6 May 2025 and 6 October 2017
(the two collisions above), 19 July and 3 May 2027 (the 2026 additions on
a weekend), Monday 2 October 1989 under the 익일휴무제, and no substitute
for Sunday 3 October 2010, for Sunday 28 January 1990 (the Seollal run),
for Wednesday 3 October 1990 or for Sunday 9 October 1988.

`the_krx_closes_on_the_days_it_lists_from_2009_to_2029` in
`crates/hc-holiday/tests/exchanges.rs` then compares every weekday the
Korea Exchange lists as closed for 2009 to 2029 — 21 years — with the
exchange's table, which is the country's table plus its two days. Since
the exchange closes on every public holiday, that list is an independent
reading of the whole regime for those years: every substitute, election
day and designated day, and the absence of a substitute where the rule
did not yet reach. The lists for the years ahead carry only the days
already set; the list for 2030 stops at October and is left out.

Known points a reader may stumble on:

- The 2021 step was enacted twice: the Act of 7 July 2021, in force
  1 January 2022, with a transitional provision for the 2021 national
  days and Christmas, and the decree amendment of 4 August 2021, in force
  at once. The table dates the step 2021 and its comments the decree.
- The 2026 decree is in force from 1 May 2026 for 노동절 and from 11 May
  2026 for the 국경일 item that restores 제헌절. Both fall in 2026 either
  way; the table bounds both at 2026.
- Seollal 1985 to 1988 was one day under the name 민속의 날; the table
  carries it under "Seollal" and the local name 설날.
- The 2013 decree's wording "다른 공휴일과 겹칠 경우" for Seollal and
  Chuseok reaches Sunday only because the decree lists Sunday as a public
  holiday. The 2021 text says "일요일" for them in words. The table's
  `SUNDAY_ONLY` trigger is the same rule under both.
- The years before 1985 are under-reported by 국제연합일 and by whatever
  else the texts between 1961 and 1985 that were not read may hold; the
  amendment list read here begins at 1985.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [kr-holiday-regulation-2026] | The decree as in force from 11 May 2026: Article 2's list, Article 3's three paragraphs, the 부칙 with its two dates | The consolidated text, 2026-09-25, from a mirror; law.go.kr's own page renders by script and could not be read as text |
| [kr-holiday-regulation-history] | The amendment list: every decree number, promulgation date, commencement and stated purpose from 1985 to 2026 | Yes, 2026-09-25, law.go.kr's 제정·개정이유 list |
| [kr-holiday-regulation-2017] | The 2013 wording of Article 3 (unchanged to 2021) and Article 2 as then numbered; the 1949 origin | Yes, 2026-09-25, the Wikisource copy of 대통령령 제28394호 |
| [kr-holiday-act-2021] | The Act: 법률 제18291호, its Article 2 list, Article 3 leaving the substitute to the decree, 부칙 제2조 for 2021 | Yes, 2026-09-25, the Wikisource copy; law.go.kr's page not readable as text |
| [korea-kr-substitute-holidays-2021] | The 2021 step in the government's own words: the four national days, 16 August, 4 and 11 October 2021, Sunday only for Seollal and Chuseok | Yes, 2026-09-25 |
| [wikipedia-ko-public-holidays] | The reasons for the designated days and the vacancy elections; the Hangul Day and Constitution Day gaps; the 익일휴무제 as applied once, on 2 October 1989, and not to the 신정, 설날 and 추석 runs | Yes, 2026-09-25; secondary, for reasons only — the dates themselves are checked against the exchange |
| [kr-holiday-regulation-1949] | The 1949 list: 1, 2 and 3 January, 식목일, 한글날, Christmas | Yes, 2026-09-25, the Wikisource copy |
| [kr-holiday-regulation-1959] | The 1959 next-day clause; 현충일 from 6 June 1956; 국제연합일 | Yes, 2026-09-25, the Wikisource copy |
| [kr-holiday-regulation-1960] | 사방의 날 in place of 식목일, and 21 March for 1960 | Yes, 2026-09-25, the Wikisource copy |
| [kr-holiday-regulation-1960-12] | The next-day clause gone from 1 January 1961 | Yes, 2026-09-25, the Wikisource copy |
| [kr-holiday-regulation-1961] | 식목일 restored | Yes, 2026-09-25, the Wikisource copy |
| [kr-holiday-regulation-1975] | 어린이날 and 석가탄신일 from 27 January 1975 | Yes, 2026-09-25, the Wikisource copy |
| [krx-market-closing] | The exchange's closed days 2009 to 2030, the check for every substitute, election day and designated day | As the exchange table cites it, retrieved 2026-09-23; not re-read for this document |

Not read here: the texts of 대통령령 제24828호, 제31930호 and 제33448호
themselves, beyond what the 2017 consolidated text, the amendment list and
the government's summary show of them; the Cabinet decisions designating
each 임시공휴일, known through the exchange's lists and the secondary
source; the texts of 제8235호 (1976), 제12616호 (1989), 제13155호 (1990),
제15939호 (1998) and 제18893호 (2005), which Wikisource's index lists but
did not serve, and which are known here through the amendment list's
stated purposes; and the texts between 1961 and 1975 other than 제7538호.

## Code

`crates/hc-holiday/src/countries/asia.rs`: the rules in `KR_RULES`, the
one-year rows through `kr_one_off`, the trigger set `SUNDAY_ONLY`, the
policy `KR_SUBSTITUTION` and the table `SOUTH_KOREA`. The engine's part
is `SubstitutionPolicy` with `on_collision`, `HolidayRule::substituted_from`,
`substitute_on` and `fixed_public` in `rule.rs`, and `collisions()`,
`substitute_day` and the substitution pass of `evaluate` in `engine.rs`.
The exchange is `KOREA_EXCHANGE` in `exchanges.rs`, with `XKRX_RULES`.

Anchors: `south_korea_holidays` and
`the_korean_substitute_holiday_covers_sundays_and_collisions` in
`crates/hc-holiday/tests/countries.rs`;
`the_krx_closes_on_the_days_it_lists_from_2009_to_2029` and
`the_krx_keeps_labour_day_and_the_last_weekday_of_the_year` in
`crates/hc-holiday/tests/exchanges.rs`.
