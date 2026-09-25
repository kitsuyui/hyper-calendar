# The Berber agrarian calendar and the Amazigh era

Backs the identifier `berber` in `hc-calendars-solar`.

## What it is

The calendar the countryside of the Maghreb keeps for the farming year is
the Julian calendar, inherited from the Roman province of Africa and never
reformed: the twelve months under names that come straight from the Latin
ones — *Yennayer* (Ennayer) from *Januarius*, *Furar* from *Februarius*,
*Ɣuct* from *Augustus* — and the leap day every fourth year without
exception, so that it now runs thirteen days behind the Gregorian calendar
[laporte2019, gast1992, elbriga1996]. The Encyclopédie berbère's entry on
the calendar finds it in use, alongside the Hijri calendar for religion and
the Gregorian for administration, wherever Berber farmers work, and notes
that "comme la plupart des paysans sont peu lettrés, ils calculent le plus
souvent les dates du calendrier julien en décalant simplement de treize
jours celles du calendrier grégorien" [gast1992, p. 1713]. Its first day,
1 Yennayer, "la porte de l'année", is the Berber new year, kept with a meal
and a night of ceremony across North Africa [elbriga1996].

The year had no number. The traditional calendar was not tied to an era,
and where a way of naming years survives, among the Tuareg, each year has a
name rather than a count [wikipedia-berber-calendar]. The "Amazigh era"
that counts from 950 BC — the accession of Shoshenq I, the Libyan prince
who founded Egypt's twenty-second dynasty and is the earliest Berber known
by name — is, in Laporte's words, "une création très récente, initiée à
Paris dans le milieu de l'Académie berbère, sans doute par Ammar Negadi
(dit « Amar Achaoui ») qui a affirmé en être le créateur": the first Berber
calendar bearing a year, 2930, appeared in 1980, and the year is now "un
acquis récent, mais désormais largement diffusé" [laporte2019]. Wikipedia
dates the initiative to the 1960s and gives the formula, the Gregorian year
plus 950 [wikipedia-berber-calendar]. The Amazigh year 2976 began on
14 January 2026.

## How it works

**The date.** Every Berber date is the Julian date of the same day, with the
year increased by 950. 1 Yennayer of year *Y* is Julian 1 January of year
*Y* − 950, and Julian 1 January is Gregorian 14 January from 1901 to 2100,
13 January from 1800 to 1900 and 15 January from 2101, the difference being
the Julian calendar's own drift, which grows by a day at each leap day the
Gregorian calendar skips.

**Worked example.** What is the Berber date of Gregorian 25 September 2026?
The Julian date is thirteen days earlier, 12 September 2026; the Berber
year is 2026 + 950 = 2976; and the ninth month is *Ctembeṛ*. So 12 Ctembeṛ
2976. In the other direction, 29 Furar 2974 is Julian 29 February 2024,
which exists because 2024 is divisible by four, and is Gregorian 13 March
2024.

**Which January.** Two conventions for Yennayer are current, and they are
not the same day:

| Day | Who keeps it | What it is |
| --- | --- | --- |
| 14 January (Julian 1 January) | The agrarian calendar wherever it is kept; a Tunisian calendar page prints 1 Yennayer *ʿajmi* against 14 January; Genevois's "un retard de 13 jours" [wikipedia-berber-calendar, genevois1975] | The calendar's own new year |
| 12 January | Much of Algeria and the diaspora since the cultural associations of the 1960s, and Algeria's public holiday since 2018; noted at Oran in 1950 as two days before the common agrarian date [wikipedia-berber-calendar] | An observance two days early, which Wikipedia attributes to a mistake of those associations |

Nothing in the sources gives the 12 January observance a calendar of its
own — it is a date on the Gregorian calendar — so this library carries the
calendar's own day and leaves the holiday to `hc-holiday`, where Algeria's
table already has Yennayer on 12 January from law 18-12 of 2 July 2018.

**The leap day.** Wikipedia states, without a source, that "the extra day
in leap years is not usually added at the end of February, but at the end
of the year" [wikipedia-berber-calendar]. Neither Encyclopédie berbère
entry read says so, and Laporte says the months are "directement ceux du
calendrier julien" [laporte2019]; the Julian 29 February is kept.

## What is carried

- **Identifier** `berber`, with the year, the month 1–12 and the day, and
  the extra field `julian-year`. The month names are declared with the
  shape in the Kabyle forms Wikipedia tabulates: Yennayer, Furar, Meɣres,
  Yebrir, Mayyu, Yunyu, Yulyu, Ɣuct, Ctembeṛ, Tubeṛ, Wambeṛ, Duǧembeṛ. The
  Riffian, Shilha, Shawiya, Mozabite and Maghrebi Arabic spellings are a
  locale's and are not carried.
- **Year number** the Amazigh era, the Julian year plus 950, `EpochForward`
  from year 1 in 950 BC; the module documentation and this document say
  what it is. `usage` begins on 1 Yennayer 2930, 14 January 1980, the first
  calendar to print the number, so that every earlier year reports
  `Standing::Proleptic`.
- **Range** year 1 to 999 999, 1 Yennayer 1 (Julian 1 January 950 BC) to
  31 Duǧembeṛ 999 999.
- **Not carried:**
  - *The 12 January observance*, as above.
  - *The agrarian sub-seasons* — the black and white nights, the *llyali*
    and *ssmaym*, the Ahaggar and Ouargla divisions that the Encyclopédie
    berbère describes [gast1992] — which are observances on this calendar
    rather than its structure, and belong in `hc-holiday` if anywhere.
  - *The medieval Berber month names* van den Boogert reconstructs, which
    are a different and undated calendar [wikipedia-berber-calendar].
  - *The Tunisian and Libyan form* of the calendar, which has the same
    days under other spellings.

## Accuracy

The reference is the Julian calendar, so the measure is agreement with
`julian` and with the published equivalences:

| Check | Test | Result |
| --- | --- | --- |
| 1 Yennayer of every year 1901–2100 is Gregorian 14 January; 13 January in 1900 and 15 January in 2101 | `yennayer_is_julian_new_year_and_gregorian_14_january_this_century` | 200 years |
| 1980 was 2930 [laporte2019]; 14 January 2018 opened 2968 (The National, via [wikipedia-berber-calendar]); year 1 is 950 BC | `the_era_counts_from_950_bc_and_was_first_printed_as_2930_in_1980` | all |
| Every sampled day is the Julian date under the year plus 950 | `every_date_is_the_julian_date_under_another_year_number` | 5 688 days |
| Every day of 2972–2979 round-trips | `every_single_day_of_a_leap_cycle_round_trips` | 2 922 days |

There is no published table to disagree with; the one known disagreement
is the 12 January convention, which is not this calendar.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [laporte2019] | The era's origin, Negadi, the 1980 calendar dated 2930, the month names as Julian | Yes, 2026-09-25 |
| [gast1992] | The Julian calendar as the farmers' calendar, the thirteen-day shift, the sub-seasons | Yes, 2026-09-25 |
| [elbriga1996] | Ennayer from *Januarius*, the observance | Yes, 2026-09-25 |
| [wikipedia-berber-calendar] | The month-name table, the era formula, 14 and 12 January, the unsourced leap-day claim | Yes, 2026-09-25 |
| [genevois1975] | The thirteen-day lag, as quoted by Wikipedia | Not read |
| [servier1962] | The month names, as cited by Laporte | Not read |

Genevois and Servier are the primary accounts; reading either would let the
month names be cited from the ethnography rather than from Wikipedia's
table of it.

## Code

`crates/hc-calendars-solar/src/berber.rs`. Anchors:
`yennayer_is_julian_new_year_and_gregorian_14_january_this_century`,
`the_era_counts_from_950_bc_and_was_first_printed_as_2930_in_1980`. The
month names are the module's `MONTHS`; the era offset is `YEAR_OFFSET`;
Algeria's 12 January holiday is `hc-holiday`'s `ALGERIA`.
