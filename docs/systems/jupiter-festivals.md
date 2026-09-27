# The Kumbh Mela and Pushkaram: festivals set by Jupiter's sign

Backs `hc-calendars-indic::kumbh` and `hc-calendars-indic::pushkaram`. No
calendar identifier is registered. The conditions of the Kumbh Mela are
`KumbhYoga` entries, one identifier each, and the rivers of Pushkaram are
`PushkaramRiver` entries ([policy.md](../policy.md) §5 and §10).

## What it is

Both festivals come round with Jupiter, which takes about twelve years to
pass through the twelve sidereal signs, about a year in each.

The **Kumbh Mela** is a pilgrimage held at four sites: Haridwar on the
Ganga, Prayag (Prayagraj) at the meeting of the Ganga and the Yamuna,
Nashik on the Godavari, and Ujjain on the Shipra. Each site holds it about
every twelve years, when Jupiter, the Sun and at some sites the Moon stand
in the signs set for that site [wikipedia-kumbh-mela,
kumbh-allahabad-astrology]. The government of the state that holds it
fixes and announces the bathing days: the Maha Kumbh of 2025 at Prayagraj
ran from 13 January to 26 February [wikipedia-kumbh-mela].

**Pushkaram** (Telugu *Pushkaralu*) is a festival of twelve rivers, one
for each sign. When Jupiter enters a sign, that river's festival is kept,
above all on the first twelve days, the *Ādi Pushkaram*, and the last
twelve before Jupiter leaves, the *Antya Pushkaram* [wikipedia-pushkaram].
The state governments along the river hold it: Andhra Pradesh's Kurnool
district held the Tungabhadra festival from 20 November to 1 December
2020 [kurnool-tungabhadra-2020].

## How it works

### The Kumbh Mela

The Mela Adhikari of the 2013 Kumbh at Allahabad gives each site's
condition in English with its Sanskrit verse, and alternative conditions
for three sites [kumbh-allahabad-astrology]:

| Identifier | Site | Jupiter in | The Sun in | At the new moon |
| --- | --- | --- | --- | --- |
| `kumbh-haridwar` | Haridwar | Kumbha | Meṣa | no |
| `kumbh-prayag-vrishabha` | Prayag | Vṛṣabha | Makara | no |
| `kumbh-prayag-mesha` | Prayag | Meṣa | Makara, with the Moon | yes |
| `kumbh-nashik-simha` | Nashik | Siṃha | Siṃha | no |
| `kumbh-nashik-karka` | Nashik | Karka | Karka, with the Moon | yes |
| `kumbh-ujjain-mesha` | Ujjain | Siṃha | Meṣa | no |
| `kumbh-ujjain-tula` | Ujjain | Tulā | the new moon of Kārttika | yes |

The page says the Ujjain alternative holds "when Jupiter enters in Libra
and Sun & Moon remain together on Kartik Amavasya". The new moon that
ends Āśvina and begins Kārttika in the amānta reckoning, the new moon of
Diwali, falls with the Sun in Tulā, and that is the reading here.

A condition holds in a year when Jupiter is in its sign at the moment the
Sun enters the condition's sign, or, for a new-moon condition, at the new
moon that falls in the Sun's stay there. The first moment is the one that
counts, because a stay of the Sun can hold a change of Jupiter's sign.

**Worked example.** Drik Panchang puts Jupiter's entry into Vṛṣabha at
13:50 IST on 1 May 2024 and its entry into Mithuna at 23:20 on 14 May 2025
[drik-guru-gochar]. The Sun entered Makara on 14 January 2025, with
Jupiter in Vṛṣabha: Prayag's first condition held, and the Maha Kumbh
opened on 13 January, its first great bath on Makara Saṅkrānti
[wikipedia-kumbh-mela]. A year earlier Jupiter was in Meṣa, which it had
entered on 22 April 2023, and the new moon of the night of 9 to 10
February 2024 fell in the Sun's stay in Makara: Prayag's second condition held in
2024, when no festival was held. Haridwar shows why the first moment is
read. Jupiter entered Kumbha on 1 May 2009, during the Sun's stay in Meṣa
that began on 14 April, and was in Kumbha when the Sun entered Meṣa on
14 April 2010. The festival was held in 2010.

### Pushkaram

| Sign | River | Where, if the source says |
| --- | --- | --- |
| Meṣa | Ganga | |
| Vṛṣabha | Narmada | |
| Mithuna | Sarasvati | |
| Karka | Yamuna | |
| Siṃha | Godavari | |
| Kanyā | Krishna | |
| Tulā | Kaveri | |
| Vṛścika | Bhima | Maharashtra, Karnataka, Telangana |
| Vṛścika | Tamraparni | Tamil Nadu |
| Dhanus | Tapti | |
| Dhanus | Brahmaputra | Assam |
| Makara | Tungabhadra | |
| Kumbha | Sindhu | |
| Mīna | Pranahita | |

The table is Wikipedia's [wikipedia-pushkaram], which cites Dalal (2014)
and notes that the rivers vary by region. When Jupiter enters a sign,
turns back out of it and enters it again, the second entry is reckoned for
the first part of the festival [wikipedia-pushkaram, citing Pillai 1996,
not read].

The twelve days begin on the civil day of Jupiter's entry, or on the next
day when the entry falls after that day's sunset. No source read states
this. It is the reading that gives every festival whose dates were read,
from Drik Panchang's entry times [drik-guru-gochar]:

| River | Jupiter's entry (IST) | The festival | Source |
| --- | --- | --- | --- |
| Godavari | 14 July 2015, 07:07 | 14–25 July 2015 | [wikipedia-godavari-pushkaram] |
| Kaveri | 12 September 2017, 08:00 | 12–23 September 2017 | [wikipedia-kaveri-pushkaram] |
| Tungabhadra | 20 November 2020, 14:55, the second entry | 20 November – 1 December 2020 | [kurnool-tungabhadra-2020] |
| Pranahita | 13 April 2022, 16:57 | 13–24 April 2022 | [hansindia-pranahita-2022] |
| Ganga | 22 April 2023, 06:12 | from 22 April 2023 | [wikipedia-pushkaram] |
| Sarasvati | 14 May 2025, 23:20, after sunset | 15–26 May 2025 | [wikipedia-sarasvati-pushkaram] |

**Worked example.** Jupiter entered Makara on 30 March 2020, went back
into Dhanus on 30 June and entered Makara again at 14:55 IST on
20 November [drik-guru-gochar]. The second entry is reckoned. It fell
before sunset, so the twelve days ran from 20 November to 1 December, as
Kurnool district held them [kurnool-tungabhadra-2020].

## What is carried

- `KumbhYoga`, the seven conditions, with `occasion`, the Sun's stay in
  the condition's sign that begins in a Gregorian year or the new moon in
  it, and `in_year`, that occasion when Jupiter is in the condition's sign
  at its start.
- `PushkaramRiver`, the fourteen rivers, with `rivers_of`, and
  `adi_pushkaram`, the twelve days for an entry of Jupiter.

**Jupiter's position is the caller's.** This library has no ephemeris of
Jupiter, so `in_year` takes Jupiter's sign as a function of the moment and
`adi_pushkaram` takes the moment of the entry. The tests supply Drik
Panchang's entry times.

Not carried:

- The bathing days of a Kumbh Mela, which the state government announces
  each time.
- The Ardh Kumbh, held between two Kumbh festivals at Haridwar and Prayag,
  whose condition the sources read do not give.
- The *Antya Pushkaram*, the last twelve days: no dated example was found.
- The dates Wikipedia's Pushkaram table gives for years to come, which do
  not agree with Drik Panchang's entries: Godavari "July 27 – August 03,
  2027" against Jupiter's entry into Siṃha on 26 June 2027, Krishna 12–23
  August 2028 against its entry into Kanyā on 24 July 2028.

## Accuracy

The Sun's entries and the new moons are this library's, from `hc-seasons`
and `hc-astro`. They stand about 8 minutes before Drik Panchang's for the
saṅkrāntis, from the difference in the Lahiri ayanamsa (see
[hindu-calendars.md](hindu-calendars.md)). No condition tested falls near
enough to Jupiter's change of sign for that to matter.

With Drik Panchang's Jupiter, the conditions give every Kumbh year that
Wikipedia's table lists from 2001 to 2028: Prayag 2001, 2013 and 2025,
Nashik 2003, 2015 and 2027, Ujjain 2004, 2016 and 2028, Haridwar 2010 and
2021 [wikipedia-kumbh-mela]. Nashik's condition also holds in 2004, when the
Sun entered Siṃha twelve days before Jupiter left it; the table lists 2003
alone. The three alternative conditions hold in other years, and in no
year a festival was held: Prayag's in 2012 and 2024, Nashik's in 2002,
2003, 2014 and 2026, Ujjain's in 2005, 2006, 2017 and 2029.

The Pushkaram rule gives all six festivals in the table above. Wikipedia's
table gives two other festivals that the rule does not give. Its Dhanus
festival of 2019, of the Tapti and the Brahmaputra, opens on 29 March, where Jupiter entered
Dhanus at 03:09 on 30 March. Its Sindhu festival of 2021 opens on
6 April, from the first entry into Kumbha, where Jupiter went back into
Makara in September and entered Kumbha again on 21 November. Neither date
is sourced in the table, and the rule is fitted to the festivals above,
not quoted.

A low-precision model of Jupiter, the Keplerian elements of Standish and
Williams on JPL's "Approximate Positions of the Planets" page, was tried
against Drik Panchang's entries from 2001 to 2030 and put them from
under an hour to 31 hours out, the worst near a station, where Jupiter moves slowly. That is
too coarse for a day, so no such model is carried. A full theory, such as
VSOP87's series for Jupiter, is the missing piece.

## Sources

- [kumbh-allahabad-astrology]: the conditions of all four sites in English
  with their verses. The Mela Adhikari's page for the 2013 Kumbh, read in
  the Internet Archive's copy of 29 October 2019, 2026-09-28.
- [wikipedia-kumbh-mela]: the four sites and rivers, the table of Kumbh
  years from 1974 to 2028, and the dates of 2025. Read 2026-09-28.
- [wikipedia-pushkaram]: the rivers by sign, the twelve days, the second
  entry. Read 2026-09-28. It cites Dalal (2014), Warrier (2014) and
  Pillai (1996), none read.
- [drik-guru-gochar]: Jupiter's sidereal entries for New Delhi, each year
  from 2001 to 2030. Read 2026-09-28.
- [kurnool-tungabhadra-2020]: the Tungabhadra festival of 20 November to
  1 December 2020. Read 2026-09-28.
- [hansindia-pranahita-2022]: the Pranahita festival of 13 to 24 April
  2022. Read 2026-09-28.
- [wikipedia-godavari-pushkaram], [wikipedia-kaveri-pushkaram] and
  [wikipedia-sarasvati-pushkaram]: the festivals of 2015, 2017 and 2025.
  Read 2026-09-28.
- [jpl-approximate-positions]: the Keplerian elements of Standish and
  Williams, tried and not used. Read 2026-09-28.

## Code

`crates/hc-calendars-indic/src/kumbh.rs` and `pushkaram.rs`. The tests
that anchor them:
`the_festivals_held_from_2001_to_2028_meet_their_sites_conditions`,
`the_second_conditions_fall_in_other_years`,
`the_maha_kumbh_of_2025_opened_in_the_suns_stay_in_makara`,
`the_festivals_whose_dates_were_read_fall_on_the_twelve_days`,
`the_ganga_festival_of_2023_began_on_the_day_of_the_entry`,
`two_signs_have_two_rivers` and
`there_is_no_pushkaram_day_where_the_sun_does_not_set`.
