# Hong Kong's general and statutory holidays, and the substitution rules

Backs the `HONG_KONG` table (`HK`) in `hc-holiday`, and through it the
Hong Kong Exchange's table (`XHKG`).

## What it is

Hong Kong has two lists of holidays, in two ordinances.

**General holidays.** The General Holidays Ordinance (Cap. 149), section 3:
"the days specified in the Schedule shall be general holidays" — the days
banks, schools, public offices and the Government close, seventeen a
year. Each item of the Schedule carries its own Sunday rule: "(b) the first
day of January (or if that day is a Sunday, then the following day)",
"(c) Lunar New Year's Day (or if that day is a Sunday, then the fourth day
of Lunar New Year)", the Mid-Autumn Festival's following day "(or if that
day is a Sunday, then the second day following that Festival)", "(q)
Christmas Day (or if that day is a Sunday, then the second weekday after
Christmas Day)". Section 6(2), from 1998, adds the coincidence rule: "if
in any year two general holidays fall on the same day, the next following
day that is not itself a general holiday is to be observed as an
additional general holiday" [hk-cap149].

The Lunar New Year and Mid-Autumn items were different until 2012. In the
version of the Schedule in force from 18 September 1998 a Sunday Lunar New
Year day gave "the day preceding Lunar New Year's Day", the eve, and a
Sunday day following the Mid-Autumn Festival gave "the day of that
Festival", the Saturday before [hk-cap149]. The General Holidays and
Employment Legislation (Substitution of Holidays) (Amendment) Ordinance
2011, No. 23 of 2011, in force from 24 February 2012, replaced both with
the forward rule quoted above [hk-substitution-amendment-2011]. The
backward rule dates from 1983, as a secondary account gives it; section 3
was replaced by Ordinance 70 of 1983, which was not read [zhwiki-hk-holidays].

**Statutory holidays.** The Employment Ordinance (Cap. 57), section 39,
lists the days every employer must give, fewer than the general holidays.
Among them "(h) the Chinese Winter Solstice Festival (冬節) or Christmas
Day, at the option of the employer". The Employment (Amendment) Ordinance
2021, No. 21 of 2021, phases the rest of the general holidays in: the
Birthday of the Buddha from 2022, the first weekday after Christmas Day
from 2024, Easter Monday from 2026, Good Friday from 2028 and the day
following Good Friday from 2030, when the two lists will be one but for
the Winter Solstice option [hk-cap57, hk-employment-amendment-2021].
Section 39(4) moves a statutory holiday that falls on a rest day, or on
another statutory holiday, to "the next day thereafter which is not a
statutory holiday or an alternative holiday or a substituted holiday or a
rest day" [hk-cap57].

## How it works

1. **The dates.** The fixed days are Gregorian; the Lunar New Year days,
   the Buddha's Birthday, Tuen Ng, the day following Mid-Autumn and Chung
   Yeung are Chinese-calendar dates; Ching Ming is the solar term at 15°
   on the China meridian; the Easter days are the Western computus.
2. **A Sunday or a coincidence** moves the holiday to the next day that is
   not already a general holiday. A Saturday moves nothing.
3. **From 1983 to 2011** the Lunar New Year days and the day following
   Mid-Autumn move backwards instead, to the eve and to the festival day.
4. **The first weekday after Christmas Day** is 26 December, or 27 December
   when the 26th is a Sunday: the Schedule states it that way, so it never
   needs a substitute.
5. **Each general holiday is statutory or not** in a given year, by the
   2021 amendment's phasing.

**Worked example: 2026.** Good Friday is 3 April 2026, the day following
it Saturday 4 April, Easter Monday 6 April — and Ching Ming falls on
Sunday 5 April, Easter Sunday. Ching Ming's Sunday gives the next day, but
Monday 6 April is already a general holiday, so it gives Tuesday 7 April.
The Buddha's Birthday, the eighth of the fourth month, is Sunday 24 May,
made up on Monday 25 May; Chung Yeung, the ninth of the ninth, is Sunday
18 October, made up on Monday 19 October. The day following Mid-Autumn is
Saturday 26 September and moves nothing. Of the Easter days, Easter Monday
is a statutory holiday from 2026; Good Friday and the day following are
general holidays only, for banks and offices, until 2028 and 2030. The
Government's list of general holidays for 2026 gives these days
[govhk-general-holidays].

And from before the amendment: Lunar New Year's Day 2007 was Sunday
18 February; the eve, Saturday 17 February, was the general holiday in its
place, where from 2012 it would have been the fourth day, Wednesday
21 February.

## What is carried

- **The seventeen general holidays** from 1 July 1997, with that year's
  2 July, the Sino-Japanese War Victory Day and the day following National
  Day of 1997 and 1998, and the one-off 3 September 2015, both a general
  and a statutory holiday.
- **The Sunday and coincidence rule**, and the backward rule of 1983 to
  2011 for the Lunar New Year days and the day following Mid-Autumn, as
  two computed rules.
- **The statutory list**, by making a general holiday that is not yet
  statutory a `Kind::Bank` day in its years.
- **The Winter Solstice** as an observance, since an employer may give it
  in place of Christmas Day.
- **Not carried, and why:**
  - The colonial holidays before 1997, the Queen's Birthday and Liberation
    Day among them; the holidays that predate 1997 carry their own years —
    the Ching Ming, Tuen Ng and Chung Yeung Festivals and the third Lunar
    New Year day from 1968 — so a year before 1997 is answered
    incompletely.
  - Section 39(4)'s rest-day rule for statutory holidays, which turns on
    an employee's own rest day, not on the calendar.
  - The Government's typhoon and rainstorm closures, which are not
    holidays.

## Accuracy

`hong_kong_makes_up_sundays_and_coincidences_on_the_next_free_day` in
`crates/hc-holiday/tests/countries.rs` checks the gazetted lists for 2026
and 2027 day by day, and the substitutes of 2022, 2026 and 2027, among
them Ching Ming 2026 past Easter Monday and Christmas 2022 past the first
weekday after it.
`hong_kong_made_up_lunar_new_year_and_mid_autumn_on_their_eves_from_1983_to_2011`
pins 17 February 2007, 13 February 2010 and 3 October 2009, and the first
forward case, 13 February 2013.
`hong_kong_phases_the_general_holidays_into_the_statutory_list` checks
the kind of each phased day on either side of its year. For the exchange,
`hong_kong_closes_on_the_general_holidays_and_halves_three_eves` in
`crates/hc-holiday/tests/exchanges.rs`.

The years 1983 to 1996 are answered for the holidays the table carries
and not for the colonial ones, and 1983 itself rests on the secondary
account.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [hk-cap149] | Section 3, section 6(2), the Schedule now and as in force from 18 September 1998 and 24 February 2012 | Yes, 2026-09-26, HKLII's copies; elegislation.gov.hk renders by script |
| [hk-cap57] | Section 39: the statutory list, the Winter Solstice option, section 39(4) | Yes, 2026-09-26, HKLII's versions from 1 January 2022, 2024 and 2026 |
| [hk-substitution-amendment-2011] | Ordinance No. 23 of 2011 and its commencement on 24 February 2012 | The Labour Department's notice, 2026-09-26; the gazette not read |
| [hk-employment-amendment-2021] | Ordinance No. 21 of 2021 and the years of its phasing | The Labour Department's notice, 2026-09-26 |
| [govhk-general-holidays] | The lists for 2022, 2026 and 2027, and the Government's reasoning on substitutes | Yes, 2026-09-22 |
| [zhwiki-hk-holidays] | The changes of 1968, 1983, 1997 and 1999 | Yes, 2026-09-22; secondary |

## Code

`crates/hc-holiday/src/countries/asia.rs`: `HK_RULES`, `HK_SUBSTITUTION`,
`hk_lunar_new_year_eve`, `hk_mid_autumn_day` and the table `HONG_KONG`;
`crates/hc-holiday/src/exchanges.rs`: `XHKG_RULES` and
`HONG_KONG_EXCHANGES`. The engine's part is `SubstitutionPolicy`'s
`on_collision` and `skip_occupied`, `Rule::moved_by_weekday` and
`Kind::Bank`.

Anchors, in `crates/hc-holiday/tests/countries.rs`:
`hong_kong_makes_up_sundays_and_coincidences_on_the_next_free_day`,
`hong_kong_made_up_lunar_new_year_and_mid_autumn_on_their_eves_from_1983_to_2011`
and `hong_kong_phases_the_general_holidays_into_the_statutory_list`; in
`crates/hc-holiday/tests/exchanges.rs`,
`hong_kong_closes_on_the_general_holidays_and_halves_three_eves`.
