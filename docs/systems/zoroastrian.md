# The Zoroastrian calendars: Qadimi, Shahanshahi and Fasli

Backs the identifiers `zoroastrian-qadimi`, `zoroastrian-shahanshahi` and
`zoroastrian-fasli` in `hc-calendars-solar`.

## What it is

The calendar of the Zoroastrian communities of Iran and of the Parsis of
India: a year of twelve months of thirty days, each day of the month named
for a divinity, and five epagomenal days, the *Gatha* days, named for the
five hymns of Zoroaster. Years are counted in the Yazdegerdi era, from the
accession of the last Sasanian king, Yazdegerd III, and written Y.Z. or
A.Y. [wikipedia-zoroastrian-calendar]; Panaino dates the era from his
coronation in A.D. 631 and gives 1358 Y.Z. as 1989 [panaino1990].

The year of 365 days drifts against the seasons by a day every four years,
and the communities have answered that differently, which is why there are
three calendars:

- **The Parsis' month.** Between 1125 and 1129 the Parsis of India inserted
  one embolismic month, *Aspandarmad vahizak*, and inserted none after it;
  the Iranian communities did not insert it. The two wandering years have
  been thirty days apart since [wikipedia-zoroastrian-calendar].
- **The split of 1745.** In 1720 Jāmāsb Welāyatī of Kermān, a priest,
  came to Surat and found the Parsi calendar a month behind Iran's. On
  17 June 1745 — Boyce and Hinnells say 1746 — one party of the Parsis
  adopted the Iranian reckoning and called it *qadīm*, "old"; the majority
  kept their own, *rasmī*, "traditional", also called *Shenshai* or
  *Shahanshahi*, "royalist" [panaino1990]. The two calendars are
  `zoroastrian-qadimi` and `zoroastrian-shahanshahi`.
- **Fasli.** In 1906 the Zarthosti Fasili Sal Mandal of Bombay proposed a
  seasonal calendar with a leap day, which found little support in India
  and more in Iran, where it is kept on the civil Solar Hijri year as the
  *Bastani* calendar [wikipedia-zoroastrian-calendar; panaino1990]. That
  is `zoroastrian-fasli`, as the Parsi Fasli community prints it
  [kadva2009].

The Zarathushtrian Religious Era, ZRE, adopted in 1990 by the
Zarathushtrian Assembly of California, counts from the March equinox of
1738 BCE, so that 3738 ZRE began in 2000 CE [wikipedia-zoroastrian-calendar].

## How it works

**The wandering years.** Nowruz, 1 Fravardin, of year *Y* Y.Z. falls on
Julian Day Number 1 952 063 + (*Y* − 1) × 365 by the Qadimi reckoning, and
1 952 093 + (*Y* − 1) × 365 by the Shahanshahi, the second holding for every
year after the intercalation; in the Julian year 1129, 498 Y.Z. began on
12 February by the Qadimi and 14 March by the Shahanshahi
[wikipedia-zoroastrian-calendar, citing Parise, not read].

**The Fasli year.** Year *Y* begins on 21 March of Gregorian year
*Y* + 630, and has a sixth epagomenal day, *Avardad Sal Gah*, on 20 March
when the Gregorian year it ends in is a leap year [kadva2009].

**Worked example: Nowruz 1370 Y.Z.** By the Qadimi formula,
1 952 063 + 1 369 × 365 = 1 952 063 + 499 685 = 2 451 748. Julian Day
2 451 545 is 1 January 2000; 203 days later is 22 July 2000, which is
Nowruz 1370 Qadimi. The Shahanshahi Nowruz is thirty days later,
21 August 2000. The Fasli 1370 begins on 21 March of 1370 + 630 = 2000.
All three are the dates Wikipedia gives, and 1370 Y.Z. is 3738 ZRE in each
[wikipedia-zoroastrian-calendar].

## What is carried

- **`zoroastrian-qadimi`**: the wandering year from 16 June 632 Julian,
  JDN 1 952 063, years 1 to 99 999; in use from the epoch, the calendar as
  it continued in Iran.
- **`zoroastrian-shahanshahi`**: the same year thirty days later, from
  498 Y.Z., the first year the source states it for; before the
  intercalation the Parsis kept the Qadimi count, and that is the calendar
  to ask.
- **`zoroastrian-fasli`**: the year from 21 March with the Gregorian leap
  day; in use from 21 March 1906, the Nowruz of the year of the proposal,
  since the source gives only the year.
- The day names, Gatha names and month names in the present-day Parsi
  forms of [kadva2009]; the era code `yz`; `ZoroastrianDate::zre_year`,
  the year plus 2 368, the source's pairing of 1370 Y.Z. with 3738 ZRE in
  all three reckonings. For the Fasli year, whose 1 Fravardin is 21 March,
  within a day of the equinox the ZRE turns at, the pairing is nearly
  exact; for the wandering years it pairs the year numbers and nothing
  more.
- **Not carried:**
  - *The Bastani calendar* of Iran, which is the civil Solar Hijri year
    (`persian`, [solar-hijri.md](solar-hijri.md)) under Zoroastrian names.
  - *The ZRE as a calendar*: its year turns at the astronomical equinox,
    which none of these reckonings follows.
  - *The Denkard's intercalation of a month every 120 years*, which no
    community has practised since the 1120s.
  - *Local variants*: Panaino reports villages near Naṭanz that put the
    epagomenal days after Bahman rather than Esfandārmoḏ [panaino1990].

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| The Qadimi epoch is 16 June 632 Julian, JDN 1 952 063 | `the_qadimi_epoch_is_the_accession_of_yazdegerd` | as stated |
| Nowruz 1370 in each reckoning: 22 July, 21 August and 21 March 2000; 3738 ZRE | `nowruz_of_1370_is_where_the_source_puts_it_in_each_reckoning` | 3 of 3 |
| 498 Y.Z. began on 12 February and 14 March 1129 Julian | `the_intercalation_set_the_two_wandering_years_a_month_apart` | as stated |
| The Fasli tables of 1379–1400 Y.Z. [kadva2009] | `the_fasli_year_is_the_compendium_table`, `the_fasli_gahambars_fall_on_the_seasonal_dates_the_source_gives` | all |
| Every calendar round-trips a wide range | `every_calendar_round_trips_a_wide_range` | all |

The one disagreement between the sources read is the year of the split,
1745 or 1746 [panaino1990]; the calendars do not depend on it, because
each is its reckoning back to its first year and the split is a fact about
who kept which.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [wikipedia-zoroastrian-calendar] | The epoch, the two Julian Day formulas, 1129 and 2000, the intercalation of the 1120s, the Fasli proposal of 1906 and the Bastani calendar, the ZRE | Yes, 2026-09-26 |
| [panaino1990] | The split of 1745 or 1746, *qadīm* and *rasmī*, the Shenshai name, the Fasli of 1906, the era from 631, the Naṭanz variant | Yes, 2026-09-26, in the Wayback Machine's copy of 5 September 2026 |
| [kadva2009] | The Fasli tables and the present-day name forms | Read by the module author, 2026-09-22; not re-read |
| [taqizadeh1938] | The history of the Iranian calendars, which Panaino and Heydari-Malayeri cite | Not read |

Parise, *The Book of Calendars* (2nd ed., Gorgias Press, 2002), which
Wikipedia cites for the Julian Day formulas and the 1129 and 2000 dates,
was not read either.

## Code

`crates/hc-calendars-solar/src/zoroastrian.rs`: `Reckoning`,
`ZoroastrianCalendar`, `QADIMI_EPOCH`, `SHAHANSHAHI_EPOCH`,
`SHAHANSHAHI_MIN_YEAR`, `FASLI_GREGORIAN_OFFSET`, `ZRE_OFFSET`, `MONTHS`,
`DAY_NAMES`, `GATHA_DAYS`. Anchors:
`nowruz_of_1370_is_where_the_source_puts_it_in_each_reckoning`,
`the_intercalation_set_the_two_wandering_years_a_month_apart`,
`the_fasli_year_is_the_compendium_table`.
