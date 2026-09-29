# The Tibetan calendar's other versions: Tsurphu, Bhutanese and Mongolian

Backs the identifiers `tibetan-tsurphu`, `tibetan-bhutan` and `mongolian`,
and `tibetan-lochen`, `tibetan-tsurphu-karana` and `tibetan-bhutan-lochen`,
two conventions of the true date, in `hc-calendars-lunar`. The arithmetic they share with the Phugpa
calendar, `tibetan` — the lunar day, the rule that skips and repeats
calendar days, the mean motions, the tables and the day that begins at
dawn — is written up in [tibetan-phugpa.md](tibetan-phugpa.md) and not
repeated here. This document says what each version and each convention
changes, and why the *Kālacakra* *karaṇa* calculation and the Inner
Mongolian "yellow" calculation are not carried. What the almanacs print
beside the date is in [tibetan-almanac.md](tibetan-almanac.md).

## What it is

The Tibetan calendar is kept in several versions whose rules differ in the
details, and they "frequently differ by a day or a month" [janson2014, §1].
Janson describes four in full [janson2014, Appendix A]:

- **Phugpa** (*phug-lugs*), begun in 1447, the official Tibetan calendar
  and the one published at Dharamsala; carried as `tibetan`.
- **Tsurphu** (*mtshur-lugs*, Tsurluk), also introduced in 1447, by
  Jamyang Dondrub Wozer, from the fourteenth-century commentaries of the
  3rd Karmapa Rangjung Dorje of Tsurphu monastery; the calendar of the
  Karma Kagyu, published from Rumtek [janson2014, Appendix A.2;
  kalacakra-org, "Open source Tsurphu calendar software"]. The Karmapa's
  office says it "remains the official calendar of the Karma Kamtsang to
  this day", and reported its Losar of 2014, a month before the Phugpa
  one [kagyuoffice-losar-2014].
- **Mongolian**, the *New Genden* version (Mongolian *Tögs buyant*), made
  by Sumpa Khenpo Yeshe Paljor, whose text uses the epoch of 1747; Janson
  cites Berzin for its creation in 1786. It became Mongolia's official
  calendar in 1911, was replaced for civil use in the 1920s and officially
  in 1948, and has returned since the 1990s as the calendar of Tsagaan Sar
  and of the holidays the Mongolian law dates by it; Berzin also has the
  Buryats and Tuvinians following it, and the Kalmyks the Phugpa
  [janson2014, Appendix A.3, citing Berzin, not read here; kalacakra-org,
  "Epoch data"].
- **Bhutanese**, described by Lhawang Lodrö in the eighteenth century
  "but said to be older", with its epoch in 1754: Bhutan's official
  calendar, in which acts are dated beside the Gregorian date and some
  holidays fall [janson2014, Appendix A.4; kalacakra-org, "Bhutan
  calendars"]. The Ministry of Home Affairs prints it day by day in its
  annual calendar [moha-bt-calendar-2025; moha-bt-calendar-2026].

**Geden.** The New Genden (*dge ldan rtsis gsar*, "New Genden
Calculations") is the Mongolian version, not a fifth: Henning's epoch
data list one Genden set, Yeshe Paljor's, and Janson's account of the
Mongolian calendar is that set [kalacakra-org, "Epoch data"; janson2014,
Appendix A.3]. Berzin's remark that it starts at the fortieth year of the
sixty-year cycle — the source of this repository's earlier note that
"Geden starts elsewhere in the sixty-year cycle" — is, Janson points out,
without effect, "the choice of epoch in itself has no importance", and the
text's epoch is in any case 1747, the first year of a cycle. So the
convention has one name here, `mongolian`, and `tibetan-geden` is not a
second name for it (policy §5 names conventions, not their aliases).

**Kālacakra.** The calendar of the *Kālacakra Tantra* itself, the
*karaṇa* (*byed rtsis*) calculation, has other mean motions and epoch
values, and Janson and Henning's epoch data give them: *m*₁ =
10 631⁄360, *s*₁ = 1 277⁄15 795, *a*₁ as in the other versions, from JD
2 015 531 with *m*₀ = 2 015 531 + 1⁄2, *s*₀ = 809⁄810, *a*₀ = 53⁄252, and
the intercalation index 0 at the epoch [janson2014, Appendix A.5;
kalacakra-org, "Epoch data"]. Janson also gives the two ways of
reckoning within it that the roadmap called its sub-methods: the Tantra's
own rounded daily increments *m*₂ = 59⁄60 and *s*₂ = 13⁄4 860, with a
simplified monthly step for each month after the first, "not used today",
and the modern almanacs' *m*₁/30 and *s*₁/30. But the calculation is not
carried, for three reasons the sources give. It is "not used to produce
calendars today", and the almanacs that print its values "do not give
any names or numbers, or years, for the months"; its leap month "seems"
to have taken the preceding month's number, which is Janson's reading and
not a rule he states from a text; and Henning's worked example of the
Tantra's own procedure rounds its intermediate results ("to the nearest
integer") where the *siddhānta* versions carry exact fractions, so it is
not a parameter set over this engine [janson2014, Appendix A.5;
kalacakra-org, "Example Kālacakra karaṇa calculations"]. What would be
needed is a published *karaṇa* calendar with month and day labels to hold
it to, and a statement from a text of how its months are numbered.

**The yellow calculation.** Inner Mongolia follows the Chinese-style
"yellow" system, which has "no repeated or skipped days" and months of 29
or 30 days, adds leap months in a way "similar to, but not equivalent to,
the Chinese", and uses "the basic calculations from Kālacakra" — Janson
quoting Berzin, and adding that it "seems to be a version of the Chinese
calendar … rather than the Tibetan" [janson2014, Appendix A.11]. Berzin's
own page gives more: the doubled and omitted days are "calculated as in the
Pugpa system, so that it is determined how many days there are in a month",
the days are then "numbered consecutively", the months "always correspond
in length and initial date with the Pugpa full tenet system months", and the doubled month "is
calculated according to the Pugpa full tenet system rules", after which
the classical Chinese calendar's rule applies: "if the month to be doubled
works out to be the 11th, 12th or 1st month, the following 3rd month is
doubled instead" [berzin-tibetan-calendar]. The page gives the same
exception for the classical Chinese calendar itself. The rule is stated,
but no yellow almanac was found to hold a reading of it to (docs/policy.md
§7 asks for a published anchor), so it is not carried.

**Two conventions of the true date.** Every version above takes the
almanacs' rounded anomaly increment and its own *siddhānta* Sun. Two
published calendars change one or the other, and so, now and then, the
calendar day a lunar day ends in:

- *Minling Lochen's anomaly.* The almanacs' *a*₂ = 1⁄28 rounds (1 + *a*₁)⁄30
  = 3 781⁄105 840, which Minling Lochen Dharmashri (1654–1717) proposed and
  Henning uses, "also used in his computed calendars" [janson2014, (7.24)
  and Remark 14]. The two give different calendars on about one day in
  4 100. Henning's computed Phugpa, Tsurphu and Bhutanese almanacs all take
  it [kalacakra-org-archive].
- *The karaṇa Sun.* "In some Tsurphu almanacs, the solar equation from the
  karana solar longitude calculation has also been used to calculate the
  (siddhanta) true_date", which "will lead to a slightly different
  true_date, and occasionally a different repeated or skipped day", about
  five times a year, and a New Year a day apart "about 2 times per
  century" [janson2014, Appendix A.2]; the module finds 44 such New Years
  in 1000–3000. Henning's Tsurphu program
  takes the *karaṇa* Sun, and his page says this "makes no difference to
  the structure of the calendar – the same days are omitted or duplicated"
  [kalacakra-org-software]; his almanacs show otherwise: they are the
  Tsurphu under the *karaṇa* Sun, and skip the 13th of month 6 of 2013,
  which the *siddhānta* Sun puts on 21 July.

The Phugpa under Lochen's anomaly is `tibetan-lochen`, the calendar of
Henning's Phugpa almanacs; the Tsurphu under the *karaṇa* Sun, with
Lochen's anomaly as Henning's almanacs have it, is `tibetan-tsurphu-karana`.
The Bhutanese under Lochen's anomaly, the calendar of Henning's Bhutanese
almanacs, is `tibetan-bhutan-lochen`. It and `tibetan-bhutan` agree on every
day of 2025 and 2026, which the Ministry's calendars print, so no official
date read tells them apart; they differ on 10 December 2001, 10 April 2004
and 22–23 April 2020 in 1990–2030, and his almanac for 2020 prints
22 and 23 April both as the 30th of month 2, as the exact increment gives.

## How it works

Every version uses the Phugpa mean motions *m*₁ = 167 025⁄5 656,
*s*₁ = 65⁄804, *a*₁ = 253⁄3 528, and their daily parts *m*₁/30, *s*₁/30 and
the almanacs' *a*₂ = 1⁄28; the same two tables; and the same day rule
[janson2014, Appendix A.2–A.4]. What differs is data:

| | Phugpa | Tsurphu | Mongolian | Bhutanese |
| --- | --- | --- | --- | --- |
| Epoch, month 3 of | 806 | 1852 (Kongtrul) | 1747 (Yeshe Paljor) | 1754 (Lhawang Lodrö) |
| β\*, the index at the epoch | 61 | 14 | 10 | 2 |
| A leap month where the index is | 48 or 49 | 0 or 1 | 46 or 47 | 59 or 60 (the month after it) |
| The leap month takes the number of | the month after | the month after | the month after | the month before |
| *m*₀ − JD of the epoch | −29 + 4 783⁄5 656 | 1 197 103⁄7 635 600 | 2 603⁄2 828 | 52⁄707 |
| *s*₀ | 743⁄804 | 23⁄27 135 | 397⁄402 | 1⁄67 |
| *a*₀ | 475⁄3 528 | 1⁄49 | 1 523⁄1 764 | 17⁄147 |
| β of the inverse | 123 | 187 | 172 | 191 |
| γ\* of the leap-year rule | 33 | 20 | 20 | 28 |

The epoch values are Janson's rationals for the digits of Henning's epoch
data — for Kongtrul's Tsurphu epoch, mean weekday 2;9,24,2,5,417 in the
radices 60, 60, 6, 13, 707, mean sun 0;1,22,2,4,18 and anomaly 0;72
[janson2014, Appendix A.2; kalacakra-org, "Epoch data"]. The Tsurphu has
two older epochs, of which Janson prints the one of 1732 and says it is
"equivalent and giving the same calendar"; Kongtrul's of 1852 is the one
Henning's Tsurphu program uses, and is the one carried.

**The two conventions.** Under Lochen's anomaly the Moon's anomaly at
the end of lunar day *d* of month *n* is *n a*₁ + *d* · 3 781⁄105 840 + *a*₀
in place of *n a*₁ + *d*⁄28 + *a*₀; nothing else changes. Under the
*karaṇa* Sun the Sun's equation of the true date is read from the *karaṇa*
mean Sun, *s*₁ = 1 277⁄15 795 a month and *s*₁⁄30 a lunar day from
*s*₀ = 809⁄810 at month 3 of 806 [janson2014, Appendix A.5], counted in the
Tsurphu's own true months moved to that epoch: the Tsurphu, like the
*karaṇa* calculation, has intercalation index 0 at 806 and rounds its
true month down, so its months are the *karaṇa* ones, and the 12 552 solar
months from 806 to Kongtrul's epoch of 1852 hold ⌊67 × 12 552⁄65⌋ = 12 938
true months, the remainder 14 being β\* at 1852 [janson2014, Appendix A.2].
The Tsurphu almanac also prints that *karaṇa* Sun as the day's solar
longitude, under either Sun in the true date [janson2014, Appendix A.2].

**Worked example: the 13th of month 6 of 2013 in the Tsurphu.** Janson's
example of a day the *karaṇa* Sun moves: "day 13, month 6, 2013, for which
the two versions yield 20 and 21 July". Under the *siddhānta* Sun lunar
day 12 ends on 20 July and lunar day 13 on 21 July, so each has its day.
Under the *karaṇa* Sun, whose mean longitude is then some 7° further on,
94.2° against 87.2°, and whose equation has the other sign, lunar day 13
ends at true date 2 456 494.980, before the dawn of 21 July, where the
*siddhānta* Sun has it at 2 456 495.003, just after; it ends on 20 July
with the 12th: 20 July is the 12th, the 13th is skipped,
and 21 July is the 14th. Henning's Tsurphu almanac for 2013 has "13.
Omitted" between the 12th on 20 July and the 14th on 21 July.

**The month count.** A version's leap rule decides how the true month,
67 *M*′⁄65 + β\*⁄65, is rounded to a count *n*. With *L* the index at
which a leap month is inserted, a month whose index has reached *L* is
rounded up, which is to say

```text
n = ⌊(67 M' + β* + c) / 65⌋,   c = (65 − L) mod 65
```

for a regular month, *c* being 17 for the Phugpa, as in (5.10), 0 for the
Tsurphu, whose true month "is obtained … by rounding down", 19 for the
Mongolian, "with 48 replaced by 46", and 6 for the Bhutanese, "with 48
replaced by 59" [janson2014, Appendix A.2–A.4]. The leap month is the
month before the regular month whose index is *L* or *L* + 1, with count
one less: under the Phugpa rule that regular month has the leap month's
number, under the Bhutanese the next number, since there "a leap month
is given the number of the *preceding* month". Gantumur writes the same
count with the same γ = 65 − τ, and the Bhutanese as the same rule with the
trigger set shifted by two [gantumur2026, §2.1 and "Parameters of the
Principal Traditions"]. The inverse is (5.19) with β = 201 − β\* − *c* for
the three that number a leap month by the month after it and 199 − β\* − *c*
for the Bhutanese; these are the 187, 172 and 191 Janson states, and 123
for the Phugpa from 806. A month is leap when it shares its number with
the month on the side it takes the number from.

**Worked example: Tsagaan Sar 2025**, the Mongolian year whose first day
the Government found omitted.

1. *The months.* Month 12 of 2024 is *M*′ = 12 × (2024 − 1747) + 12 − 3 =
   3 333, with index (2 × 3 333 + 10) mod 65 = 46: a leap month 12 comes
   before it. Its count is ⌊(67 × 3 333 + 10 + 19)⁄65⌋ = ⌊223 340⁄65⌋ =
   3 436, and the leap month's 3 435. Month 1 of 2025 is *M*′ = 3 334, index
   48, count ⌊223 407⁄65⌋ = 3 437.
2. *Day 30 of month 3 436*: mean date 2 460 734.547 9; anomaly 85⁄252, so
   28 × 0.337 3 = 9.444, past 7, and moon_tab(14 − 9.444) = 19 + 0.556 × 3 =
   20.667; mean sun 229⁄268 = 0.854 5, so 12 × (0.854 5 − 0.25) = 7.254,
   past 6, and sun_equ = −sun_tab(1.254) = −(6 + 0.254 × 4) = −7.015. The
   true date is 2 460 734.547 9 + 20.667⁄60 + 7.015⁄60 = 2 460 735.009:
   lunar day 30 ends on JD 2 460 735, Friday 28 February 2025, the
   *bituun*.
3. *Days 1 and 2 of month 3 437*: lunar day 1 ends at 2 460 735.938, on
   the same 28 February, and lunar day 2 at 2 460 736.850, on 1 March. Two
   lunar days end on 28 February, so it takes the first one's number, 30,
   and day 1 is skipped: Saturday 1 March 2025 is day 2 of the first spring
   month, Tsagaan Sar, and 2 March is day 3.

That is what the Government's resolution 109 of 26 February 2025 says:
"шинийн 1 тасарч, шинийн 2, 3-ны өдөр Бямба, Ням гарагт тохиож" — day 1
cut out, days 2 and 3 on the Saturday and Sunday
[mn-resolution-2025-109].

**Worked example: a Bhutanese leap month.** Month 5 of 2008 is
*M*′ = 12 × (2008 − 1754) + 2 = 3 050, with index (6 100 + 2) mod 65 = 57, so
the regular month 6 after it has index 59 and a leap month comes between
them; it takes the number 5 of the month before it. The regular month 5
has count ⌊(67 × 3 050 + 2 + 6)⁄65⌋ = 3 143, the leap month 5 3 144 and
month 6 3 145, and 28 July 2008 is day 26 of the leap month — the
Election Act's "26th Day of the Second 5th Month of the Earth Male Rat
Year corresponding to the 28th Day of the 7th Month of the Year 2008"
[janson2014, appendix on leap months and the mean sun, citing the National
Assembly of Bhutan]. The module writes it `2008-5L-26`, and its leap
month sorts after the regular month of the same number.

## What is carried

- **Identifiers** `tibetan-tsurphu` ("Tibetan (Tsurphu)"),
  `tibetan-bhutan` ("Tibetan (Bhutanese)") and `mongolian` ("Mongolian
  (Tögs buyant)"), each a `TibetanCalendar` value over the one engine that
  also carries `tibetan`, with the date type, the fields, the extra day,
  the range 1000–3000 and the dawn day boundary of the Phugpa calendar.
  The Bhutanese leap month follows the regular month of its number.
- **Exact arithmetic** from each version's epoch, as for the Phugpa.
- **Usage**: the Tsurphu from Losar of 1447 [janson2014, Appendix A.2;
  kalacakra-org], the Mongolian from Tsagaan Sar of 1786, the year
  Janson gives through Berzin, and the Bhutanese undated, its source
  giving no beginning.
- **Native locales** `bo` for the Tsurphu, `mn` for the Mongolian, `dz`
  for the Bhutanese, whose almanac and acts are in Dzongkha; the Tsurphu
  months take the Tibetan locale's numbered names, as the Phugpa ones do,
  and a leap month takes ཟླ་ཤོལ་ before its name, Henning's *zla shol*
  for "an extra, or intercalary, month" [kalacakra-org]; the word's place
  in a date is this library's ([written-dates.md](written-dates.md)).
- **The two conventions**, as three calendars: `tibetan-lochen` ("Tibetan
  (Phugpa, Lochen's anomaly)"), `tibetan-bhutan-lochen` ("Tibetan
  (Bhutanese, Lochen's anomaly)") and `tibetan-tsurphu-karana` ("Tibetan
  (Tsurphu, karaṇa Sun)"), `TibetanCalendar` values beside the four
  versions, the Phugpa's, the Bhutanese and the Tsurphu's data under
  `AnomalyStep::Lochen` and, for the last, `Sun::Karana`. Same fields,
  extra day, range, dawn boundary and native locale as their version —
  `bo`, and `dz` for the Bhutanese — and their version's month names;
  undated usage, the sources dating none. The engine's quantities —
  `Ratio`, the mean motions, `table`, `true_sun`, and the methods
  `mean_date`, `mean_sun`, `karana_mean_sun`, `anomaly`, `true_date`,
  `end_day`, `lunar_day_span` and `locate` — are public for the almanac;
  every version under another increment is one of the registered
  calendars, so none is reached without a name.
- **The Tsurphu almanac's *karaṇa* Sun**, the solar longitude it prints,
  and its Moon on the first of two days with one number, as
  [tibetan-almanac.md](tibetan-almanac.md) describes.
- **The Bhutanese weekday**, one ahead of the world's —
  `bhutanese_weekday`, the Ministry's calendar heading the column of
  Sundays *zla ba* — and **the winter solstice** at mean solar longitude
  250°, `bhutanese_winter_solstice` ([tibetan-almanac.md](tibetan-almanac.md)).
- **The Mongolian months by season and by animal, and the colours**:
  `mongolian_month`, `month_symbol` with the Tsurphu's month cycle, which
  is the Mongolian one, `year_symbol` and `COLOURS` [janson2014, Appendix
  A.3], with the Mongolian words of Gantumur's calendar
  ([tibetan-almanac.md](tibetan-almanac.md)).
- **Not carried**: the *karaṇa* calculation and the yellow calculation, as
  above. The Mongolian leap month's word, илүү, which Gantumur's calendar
  writes in a month's heading, is data in `tibetan_almanac` but writes no
  date, as `hc-i18n` carries no `mn` locale; the months of `mongolian`
  are written in the English and Tibetan numbered names.

## Accuracy

**Janson's comparison, reproduced** (`losar_and_the_leap_months_are_those_of_jansons_comparison`,
`the_skipped_and_repeated_days_of_2012_are_jansons`,
`the_versions_meet_at_the_common_epoch_as_jansons_table_gives`). His
Appendix A.13 prints, for the four versions, Losar for every year
2000–2030, the leap months 2000–2019, every repeated and skipped day of
2012, and the epoch values recomputed to JD 2 015 531 to six decimals: the
module reproduces all of them. Those are Janson's computations from the
same constants, so this is agreement with the source's arithmetic.

**Henning's digits** (`the_epoch_values_are_hennings_digits`,
`the_two_tsurphu_epochs_give_one_calendar`). Every epoch value carried
equals the mixed-radix digits of Henning's epoch data, read directly, and
so do the shared mean motions. The Tsurphu epochs of 1732 and 1852 give the
same Losar for every year 1600–2300 and the same date for every day of
1800–2200 in a release build (every 11th day in a debug one).

**Published Tsurphu dates** (`the_tsurphu_calendar_is_hennings_and_the_karmapas`).
Henning's Tsurphu program prints month 1 of 2013 as "Month: 1989;39",
"Anomaly: 18;45" and "Mean Weekday: 1;29,39,2,574", with day 1 on
11 February 2013: the module has the same true month count and index, the
same anomaly and the same mean weekday to the printed place, and the same
day. The Karmapa's office celebrated the Tsurluk Losar of the Male Wood
Horse year on 31 January 2014 [kagyuoffice-losar-2014], which the module
gives, a month before the Phugpa Losar of 2 March. No Rumtek almanac was
read.

**Official Mongolian dates** (`tsagaan_sar_falls_on_the_published_days`).
The Government's resolution 109 of 2025 (the skipped first day, worked
above); MONTSAME, the state news agency, for Tsagaan Sar on 24–26 February
2020, whose "widespread celebrations" the President's decree asked the
public to cancel
[montsame-tsagaan-sar-2020], and on 18, 19 and 20 February 2026
[montsame-tsagaan-sar-2026]; the constitution of 1992, in force "from the
horse hour of the auspicious yellow horse day of the black tiger first
spring month of the water monkey year of the seventeenth 60-year cycle",
day 9 of month 1, 12 February 1992 [janson2014, Appendix A.3, citing
Sanders and Bat-Iredüi, not read here]; and Gantumur's worked Tsagaan Sar
2026, true month 3 449 and JD 2 461 090 [gantumur2026]. The module agrees
with every one. Janson reports that Terbish's list of Mongolian New Years
1896–2013 agrees with these rules throughout, and that other lists, on
Mongolian Wikipedia among them, contradict each other [janson2014,
Appendix A.3]; those lists were not read here.

**The Ministry of Home Affairs' calendars, day by day**
(`every_day_of_the_ministrys_calendars_for_2025_and_2026_is_reproduced`,
`the_bhutanese_calendar_gives_the_governments_dates`). The Ministry's
calendars for 2025 and 2026 print the Bhutanese day number over every
Gregorian day, and state for each month which numbers are omitted (*chad*)
and doubled (*lhag*); their Dzongkha holiday lists give each holiday's
Bhutanese month and day. Measured on 2026-09-26 by reading every page
against the module's output: all 730 days, every month's omitted and
doubled days, and every dated holiday of the two lists agree, the Losars of 28 February
2025 and 18 February 2026 among them. The transcription is the test's
table; it was checked against the pages by eye, and a misreading would
fail it.

**The two conventions against Henning's almanacs**
(`hennings_almanacs_of_2013_skip_and_repeat_these_days`,
`lochens_anomaly_moves_the_days_janson_names`,
`the_karana_sun_moves_the_day_janson_names`). Every skipped and repeated
number of 2013 in Henning's Phugpa and Tsurphu almanacs is
`tibetan-lochen`'s and `tibetan-tsurphu-karana`'s, and every other day of
the year bears its number once. Janson's search found 9 days that the two
anomaly increments put differently in 1900–1999 and 8 in 2000–2099, among
them 10 February 2001, 10 May 2006 and 19 November 2025: the module finds
the same counts in a release build and the same three days. Henning's
Phugpa almanac for 2025 prints 18 and 19 November as the 29th of month 9
and 20 November as the 30th, as `tibetan-lochen` has them, where `tibetan`
makes 19 November the first of two days numbered 30; his Bhutanese almanac for 2020
prints 22 and 23 April both as the 30th of month 2, as the exact increment
gives. Measured on 2026-09-29, every date of the 1 448 days of his Phugpa
almanacs read (2012–2014 and 2025) and the 1 093 of his Tsurphu ones
(2012–2014) is the convention's; see [tibetan-almanac.md](tibetan-almanac.md)
for the columns. No almanac other than Henning's was read for either
convention, and none of the official dates read — Losar, Tsagaan Sar, the
Ministry's 730 days — falls on a day the conventions move.

**Known disagreements.**

- *Bhutan, Losar 2003.* Henning reports that the Bhutanese government's
  calendar had day 1 of month 1 on 3 and 4 March 2003, where the
  arithmetic — Janson's and Henning's own software — makes 3 March a
  repeated day 30 of the leap month 12 of 2002 and Losar 4 March; Janson
  has "no explanation for this discrepancy" [janson2014, Appendix A.13].
  The module carries the arithmetic
  (`bhutans_losar_of_2003_is_the_arithmetics_not_the_governments`).
- *Bhutan, the definition points.* The value of β that gives the actual
  Bhutanese leap months, 191, is not the one Lhawang Lodrö's own
  definition points would imply, 189; Janson has "no explanation for
  this" either [janson2014, appendix on leap months and the mean sun]. The
  module carries 191, which the Election Act of 2008 and the Ministry's
  calendars confirm.
- *Mongolia, "the second new moon after the winter solstice".* This
  repository's roadmap described Tsagaan Sar so. It is not the rule of the
  calendar, and it is not what the calendar gives: counting the
  astronomical new moons after the December solstice in Ulaanbaatar's
  standard time, Tsagaan Sar falls on the day of or after the second in
  21 of the years 2000–2030 and the third in the other 10 — 2001, 2004,
  2009, 2012, 2015, 2017, 2020, 2023, 2025 and 2028 — and two of the
  official years read, 2020 and 2025, are third-new-moon years
  (`tsagaan_sar_follows_the_second_or_third_new_moon_after_the_solstice`).
  None of the official dates read disagrees with the calendar itself.
- *Tsurphu, the almanacs.* Gantumur reports, after Schuh, that some
  published Tsurphu almanacs put the leap month one month later, in
  1964/65 and 1970/71 [gantumur2026, §2.1]; no such almanac was read, and
  the module follows the rule.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [janson2014] | The versions' history, epoch values, leap rules, β and γ\*, the comparison tables of Appendix A.13, the Election Act, the constitution of 1992, the Bhutan 2003 discrepancy, the *karaṇa* constants and the yellow calculation | Yes, 2026-09-26, from the TeX source on arXiv |
| [berzin-tibetan-calendar] | The yellow calculation's months and its leap-month exception | Yes, 2026-09-27 |
| [kalacakra-org] | "Epoch data": the digits of every epoch; "Open source Tsurphu calendar software": the 2013 month header, the *karaṇa* Sun; "Bhutan calendars" and "Bhutanese Calendar": the leap-month numbering, the weekday, the holidays; "Example Kālacakra karaṇa calculations" | Yes, 2026-09-26, over plain HTTP, the HTTPS host still presenting another domain's certificate |
| [kalacakra-org-archive] | Henning's computed Phugpa, Tsurphu and Bhutanese almanacs, their skipped and repeated days and the days the conventions move | Yes, 2026-09-29, over plain HTTP |
| [kalacakra-org-software] | "Open source Tsurphu calendar software": the *karaṇa* Sun of Henning's Tsurphu program | Yes, 2026-09-29 |
| [gantumur2026] | The same parameters stated independently, the month count in γ, Tsagaan Sar 2026 worked, Schuh's Tsurphu almanacs | Yes, 2026-09-26, from the TeX source on arXiv |
| [moha-bt-calendar-2025], [moha-bt-calendar-2026] | Every day of 2025 and 2026, the monthly omitted and doubled days, the holiday lists with Bhutanese dates | Yes, 2026-09-26, the page images extracted from the PDFs |
| [mn-resolution-2025-109] | Tsagaan Sar 2025, its first day omitted | Yes, 2026-09-26, on legalinfo.mn |
| [montsame-tsagaan-sar-2020], [montsame-tsagaan-sar-2026] | Tsagaan Sar 2020 and 2026 | Yes, 2026-09-26 |
| [kagyuoffice-losar-2014] | The Tsurluk Losar of 2014 | Yes, 2026-09-26 |
| [henning2007] | The Phugpa and Tsurphu histories, pp. 337–342 on the Tsurphu epochs | Not read; cited through Janson |
| Berzin, *Tibetan Astro Science* (1986) | The New Genden's date, 1786, and its users | Not read; cited through Janson |
| Sanders and Bat-Iredüi, *Colloquial Mongolian* (1999) | The constitution's date | Not read; cited through Janson |
| National Assembly of Bhutan, the Election Act of 2008 | Its enactment date | Not read; cited through Janson |
| Schuh, on the Tsurphu almanacs of 1964/65 and 1970/71 | The later leap months | Not read; cited through Gantumur |

## Code

`crates/hc-calendars-lunar/src/tibetan.rs`: the constants
`TIBETAN_TSURPHU`, `TIBETAN_BHUTAN` and `MONGOLIAN` beside `TIBETAN`, the
enum `LeapNumbering`, and the methods `true_month_count`,
`month_of_count`, `inverse_constant`, `leap_month_of` and `new_year` that
read a version's data. Anchors: `the_epoch_values_are_hennings_digits`,
`the_versions_meet_at_the_common_epoch_as_jansons_table_gives`,
`the_inverse_constants_and_leap_year_rules_are_jansons`,
`losar_and_the_leap_months_are_those_of_jansons_comparison`,
`the_skipped_and_repeated_days_of_2012_are_jansons`,
`the_two_tsurphu_epochs_give_one_calendar`,
`the_tsurphu_calendar_is_hennings_and_the_karmapas`,
`tsagaan_sar_falls_on_the_published_days`,
`tsagaan_sar_follows_the_second_or_third_new_moon_after_the_solstice`,
`the_bhutanese_calendar_gives_the_governments_dates`,
`every_day_of_the_ministrys_calendars_for_2025_and_2026_is_reproduced`,
`bhutans_losar_of_2003_is_the_arithmetics_not_the_governments`,
`the_bhutanese_leap_month_follows_the_month_it_repeats`,
`the_versions_are_registered_under_their_own_names`.

The conventions are in the same module: `AnomalyStep`, `Sun`,
`TIBETAN_LOCHEN`, `TIBETAN_TSURPHU_KARANA`, and the public quantities
above. Anchors: `lochens_anomaly_moves_the_days_janson_names`,
`the_karana_sun_moves_the_day_janson_names`,
`hennings_almanacs_of_2013_skip_and_repeat_these_days`,
`the_conventions_are_registered_and_round_trip`,
`exact_numbers_print_as_the_almanacs_do`.
