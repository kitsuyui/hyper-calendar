# The Kumbh Mela and Pushkaram: festivals set by Jupiter's sign

Backs `hc-calendars-indic::kumbh` and `hc-calendars-indic::pushkaram`, and
their computed forms in the facade's `jupiter_lines` and the `jupiter` layer.
No calendar identifier is registered. The conditions of the Kumbh Mela are
`KumbhYoga` entries, one identifier each, the rivers of Pushkaram are
`PushkaramRiver` entries, and the two readings of which entry of Jupiter
opens a Pushkaram are `EntryRule`'s `pushkaram-final-entry` and
`pushkaram-first-entry` ([policy.md](../policy.md) §5 and §10). Jupiter's
position is [jupiter-ephemeris.md](jupiter-ephemeris.md)'s.

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
for three sites [kumbh-allahabad-astrology]. The identifier of a site with
two conditions names Jupiter's sign in each:

| Identifier | Site | Jupiter in | The Sun in | At the new moon |
| --- | --- | --- | --- | --- |
| `kumbh-haridwar` | Haridwar | Kumbha | Meṣa | no |
| `kumbh-prayag-vrishabha` | Prayag | Vṛṣabha | Makara | no |
| `kumbh-prayag-mesha` | Prayag | Meṣa | Makara, with the Moon | yes |
| `kumbh-nashik-simha` | Nashik | Siṃha | Siṃha | no |
| `kumbh-nashik-karka` | Nashik | Karka | Karka, with the Moon | yes |
| `kumbh-ujjain-simha` | Ujjain | Siṃha | Meṣa | no |
| `kumbh-ujjain-tula` | Ujjain | Tulā | the new moon of Kārttika | yes |

The page says the Ujjain alternative holds "when Jupiter enters in Libra
and Sun & Moon remain together on Kartik Amavasya". The new moon that
ends Āśvina and begins Kārttika in the amānta reckoning, the new moon of
Diwali, falls with the Sun in Tulā, and that is the reading here.

A condition holds in a year when Jupiter is in its sign at one moment:

- the moment the Sun enters the condition's sign; or,
- for a new-moon condition, the new moon that falls in the Sun's stay
  there.

The first moment is the one that counts, because a stay of the Sun can
hold a change of Jupiter's sign.

**Worked example.** Drik Panchang puts Jupiter's entry into Vṛṣabha at
13:50 IST on 1 May 2024, and its entry into Mithuna at 23:20 on 14 May 2025
[drik-guru-gochar].

- **Prayag, 2025.** The Sun entered Makara on 14 January 2025, with
  Jupiter in Vṛṣabha. Prayag's first condition held. The Maha Kumbh opened
  on 13 January, and its first great bath was on Makara Saṅkrānti
  [wikipedia-kumbh-mela].
- **Prayag, 2024.** A year earlier Jupiter was in Meṣa, which it had
  entered on 22 April 2023. The new moon of the night of 9 to 10 February
  2024 fell in the Sun's stay in Makara. So Prayag's second condition held
  in 2024, when no festival was held.
- **Haridwar, 2009 and 2010**, show why the first moment is read. Jupiter
  entered Kumbha on 1 May 2009, during the Sun's stay in Meṣa that began on
  14 April. It was in Kumbha when the Sun entered Meṣa on 14 April 2010.
  The festival was held in 2010.
- **Haridwar, 2021**, came eleven years later, not twelve. Jupiter entered
  Kumbha at 01:50 IST on 6 April 2021, eight days before the Sun entered
  Meṣa on 14 April, so the condition held. Jupiter went back into Makara
  on 14 September and entered Kumbha again on 21 November; by the Sun's
  entry into Meṣa in April 2022 it was in Mīna, which it had entered on
  13 April [drik-guru-gochar]. The code reads Jupiter's sign at each
  year's saṅkrānti rather than counting twelve years, and the sky gives
  eleven here.

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
| Krishna | 11 August 2016, 22:24, after sunset | 12–23 August 2016 | [vijayawadapolice-krishna-2016] |
| Kaveri | 12 September 2017, 08:00 | 12–23 September 2017 | [wikipedia-kaveri-pushkaram] |
| Brahmaputra | 5 November 2019, 06:41, the second entry | 5–16 November 2019 | [sentinel-brahmaputra-2019] |
| Tungabhadra | 20 November 2020, 14:55, the second entry | 20 November – 1 December 2020 | [kurnool-tungabhadra-2020] |
| Pranahita | 13 April 2022, 16:57 | 13–24 April 2022 | [hansindia-pranahita-2022] |
| Ganga | 22 April 2023, 06:12 | from 22 April 2023 | [wikipedia-pushkaram] |
| Sarasvati | 14 May 2025, 23:20, after sunset | 15–26 May 2025 | [wikipedia-sarasvati-pushkaram] |
| Godavari | 26 June 2027, 05:43 | 26 June – 7 July 2027, announced | [eastgodavari-pushkaralu-2027] |

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

**Jupiter's sign has two paths.** `hc-calendars-indic`'s `in_year` takes
Jupiter's sign as a function of the moment and `adi_pushkaram` the moment of
the entry, so a caller with its own ephemeris gives them, as the tests of that
crate do with Drik Panchang's entry times. With the `jupiter` feature the
facade computes them from the VSOP87B series, in its `jupiter_lines`:
`hc_kumbh_by_sky` is `hc_kumbh` with `jupiter` found at the occasion's first
moment, and `hc_pushkaram_by_sky` is `hc_pushkaram` with the entry found in a
Gregorian year, by `hc_seasons::zodiac::jupiter::entry_into`, under one of the
two `EntryRule`s. `hc_pushkarams_in_year` is `hc_pushkaram_by_sky` for every
sign Jupiter enters in the year, by `entries_in`, which walks the ingresses
once for all twelve signs: the same lines, one sign after another in the
order of the entries, from one search of the sky where twelve calls make
twelve ([jupiter-ephemeris.md](jupiter-ephemeris.md#the-search-and-what-it-costs)
has the timings). A year in which Jupiter enters two signs has both (1999:
Mīna on 12 January, Meṣa on 26 May); one in which it enters a sign, turns back
and enters it again has the one entry the rule names (2025: Karka's first
entry, on 18 October, is the first-entry rule's and the 2026 final entry
the other's); and a year with none has no line, 8 of the 200 years of 1900
to 2099 by the final entry and 13 by the first. Every ingress these find is the same instant whatever span or
export asks for it (jupiter-ephemeris.md says how and what it moved), so
`hc_jupiter_ingresses`, `hc_pushkaram_by_sky` and `hc_pushkarams_in_year` agree
on a festival's entry to the second. `hc_kumbh` and `hc_pushkaram`
themselves, in the `calendars` layer, still take the caller's.

Not carried:

- The bathing days of a Kumbh Mela, which the state government announces
  each time.
- The Ardh Kumbh, held between two Kumbh festivals at Haridwar and Prayag,
  whose condition the sources read do not give.
- The *Antya Pushkaram*, the last twelve days: no dated example was found.
- The dates Wikipedia's Pushkaram table gives for years to come, where they
  do not agree with Drik Panchang's entries: Godavari "July 27 – August 03,
  2027" against Jupiter's entry into Siṃha on 26 June 2027 and the East
  Godavari district's 26 June to 7 July, Krishna 12–23 August 2028 against
  its entry into Kanyā on 24 July 2028, and Kaveri "September 12–23, 2029"
  against its entry into Tulā at 01:13 on 25 August 2029, from which the
  rule gives 25 August to 5 September.

## Accuracy

The Sun's entries and the new moons are this library's, from `hc-seasons`
and `hc-astro`. They stand about 8 minutes before Drik Panchang's for the
saṅkrāntis, from the difference in the Lahiri ayanāṃśa (see
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

The Pushkaram rule gives all nine festivals in the table above, the
Godavari's of 2027 as the district announces it. Wikipedia's
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
too coarse for a day, so no such model is carried. VSOP87's series for
Jupiter is.

### With Jupiter computed

**The entries.** From 2001 to 2030, Jupiter's 49 entries into a sign that
Drik Panchang prints are 49 of the 50 this crate finds (the fiftieth is
later in 2030, past the table's end), each in the sign and the direction
Drik Panchang gives. This crate's sidereal longitude of Jupiter at each is on
the boundary to within 23.9″ to 27.1″, a constant 25″ that is the difference of
the two Lahiri ayanāṃśas ([jupiter-ephemeris.md](jupiter-ephemeris.md)):
this crate's Lahiri is 25″ smaller. With 25″ added, every entry is within
6.9 minutes of Drik Panchang's, 1.7 minutes on average. With this crate's
own, the forward entries come 40 to 135 minutes earlier and the returns up to
four and a half hours later, and the Pushkaram days below do not change: the
entries stand 30 to 80 minutes before Drik Panchang's, no nearer than 2 hours
to a sunset in any of the nine festivals.

**The Kumbh Mela, 1974 to 2030.** With Jupiter's sign computed, Lahiri:

| Condition | Years it holds |
| --- | --- |
| `kumbh-haridwar` | 1974, 1986, 1998, 2010, 2021 |
| `kumbh-prayag-vrishabha` | 1989, 2001, 2013, 2025 |
| `kumbh-prayag-mesha` | 1977, 2000, 2012, 2024 |
| `kumbh-nashik-simha` | 1980, 1991, 1992, 2003, 2004, 2015, 2027 |
| `kumbh-nashik-karka` | 1979, 1990, 1991, 2002, 2003, 2014, 2026 |
| `kumbh-ujjain-simha` | 1980, 1992, 2004, 2016, 2028 |
| `kumbh-ujjain-tula` | 1981, 1982, 1993, 1994, 2005, 2006, 2017, 2029 |

Every year of the table Wikipedia gives from 1974 to 2028 meets a condition
of its site: Haridwar's 1974, 1986, 1998, 2010 and 2021, Nashik's 1980, 1992,
2003, 2015 and 2027, Ujjain's 1980, 1992, 2004, 2016 and 2028, and Prayag's
1989, 2001, 2013 and 2025 by the first condition. **Prayag's 1977 is the
exception to the first condition**: Jupiter entered Vṛṣabha on 8 July 1976,
turned back into Meṣa on 8 December and entered Vṛṣabha again on 22 February
1977, so on 14 January 1977, when the Sun entered Makara, it was in Meṣa: the
second condition holds, at the new moon of 19 January. That condition holds in
2000, 2012 and 2024 as well, in none of which a festival was held, so the
table's 1977 is not a reading of the second condition as a rule, only the one
year in which the first misses. Nashik's also holds in 1991 and in 2004: Jupiter
entered Siṃha on 14 August 1991, three days before the Sun, and left on
11 September 1992, so both Augusts hold; and the Sun entered Siṃha within two
weeks before Jupiter left it, on 27 August 2004. Nothing in the sources says which of
two years a festival is held in, and the library does not guess.

**The festivals of 2025 to 2028.**

- *Prayagraj, 2025*, 13 January to 26 February [wikipedia-2025-prayag-maha-kumbh]:
  the Sun enters Makara on 14 January, with Jupiter in Vṛṣabha; held under the
  first condition. Jupiter is in Vṛṣabha on 12 February, Magha Purnima.
- *Nashik, 2027*: the dates the Maharashtra government announced are 29 July and
  2 August, the first two *Amrit Snan*, and 31 August and 11 and 12 September
  [indiatv-nashik-kumbh-2027, for the first two; the others as the aggregators
  print them, not read at a primary source]. Jupiter is in Siṃha from 26 June to
  26 November 2027 and the Sun enters Siṃha on 17 August: `kumbh-nashik-simha`
  holds in 2027. The second condition, Jupiter in Karka with the Sun and Moon,
  does not: at the new moon of 2 August Jupiter is in Siṃha.
- *Ujjain, 2028*, "between April 9 and May 8" [theweek-simhastha-2028]: the Sun
  enters Meṣa on 13 April, inside it, with Jupiter in Siṃha, where it stays from
  28 February to 24 July 2028: `kumbh-ujjain-simha` holds in 2028.
- *The Godavari Pushkaram of 2027*, 26 June to 7 July [eastgodavari-pushkaralu-2027]:
  Jupiter enters Siṃha at 04:48 IST on 26 June, 55 minutes before Drik Panchang's
  05:43, and the twelve days are those.

**Pushkaram, the entries found.** For each of the nine festivals above,
`hc_pushkaram_by_sky` with `pushkaram-final-entry` finds the entry in the
year, and the twelve days from it are the published ones: the Godavari 2015,
Krishna 2016, Kaveri 2017, Brahmaputra 2019, Tungabhadra 2020, Pranahita
2022, Ganga 2023, Sarasvati 2025 and Godavari 2027, all nine. No year but
those has an entry into Siṃha between 2016 and 2026. With
`pushkaram-first-entry` the 2019 Dhanus festival opens on 30 March, where
Wikipedia's table has 29 March: Jupiter enters at 22:42 IST on the 29th by
this crate, after that day's sunset, and at 03:09 on the 30th by Drik
Panchang, so the rule gives the 30th from either. The 2021 Sindhu festival
opens on 6 April, as the table has it. Neither date is sourced there, and no
festival read followed them.

**The tropical zodiac** is not a reading of these festivals: at the Sun's
entry for Prayag 2025, Ujjain 2028 and Nashik 2027 Jupiter's tropical sign is
not its sidereal one, by the ayanāṃśa of 24°, and no condition of the sources
holds in the tropical. A Kumbh by the tropical zodiac is not carried, and no
source read asks for one.

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
- [vijayawadapolice-krishna-2016]: the Krishna festival of 12 to
  23 August 2016. The Vijayawada police page, read in the Internet
  Archive's copy of 26 January 2025, 2026-09-28.
- [sentinel-brahmaputra-2019]: the Brahmaputra festival of 5 to
  16 November 2019. *The Sentinel*, Guwahati, 8 September 2019, read
  2026-09-28.
- [eastgodavari-pushkaralu-2027]: the Godavari festival of 26 June to
  7 July 2027, as the district announces it. Read 2026-09-28.
- [jpl-approximate-positions]: the Keplerian elements of Standish and
  Williams, tried and not used. Read 2026-09-28.
- [webdunia-kumbh-2027]: the four sites' Hindi names, हरिद्वार,
  प्रयागराज, नासिक and उज्जैन, which `hc_i18n::reckonings` carries under
  `hi`. Read 2026-09-28.
- [amarujala-pushkar-kumbh-2025]: the rivers of the Pushkar Kumbh by sign in
  Hindi, ten of the twelve, nine of which `hc_i18n::reckonings` carries
  under `hi` (its कोवरी for Kaveri is not carried). Read 2026-09-28.
- [wikipedia-kumbh-mela]'s table of years was read again on 2026-10-03, from 1974.
- [wikipedia-2025-prayag-maha-kumbh]: the Maha Kumbh of 13 January to
  26 February 2025 and its bathing days. Read 2026-10-03.
- [theweek-simhastha-2028]: the Ujjain Simhastha, "between April 9 and May 8,
  2028". Read 2026-10-03; the pages that give 27 March to 27 May, and the
  Shahi Snan dates of 9 and 23 April and 8 May, are aggregators and not cited.
- [indiatv-nashik-kumbh-2027]: the Nashik Kumbh's flag hoisting on
  31 October 2026 and its first two Amrit Snan, 29 July and 2 August 2027, as
  announced by Maharashtra's Chief Minister. Read 2026-10-03.
- [jpl-horizons-jupiter], [vsop87b-jup], [vsop87-doc-copy], [drik-guru-asta],
  [varahamihira-brihat-samhita], [subbarayappa1985]: Jupiter's position and
  its risings, in [jupiter-ephemeris.md](jupiter-ephemeris.md).

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

The tests that anchor the computed forms are in
`crates/hyper-calendar/tests/jupiter_festivals.rs`:
`the_sidereal_longitude_stands_a_constant_25_arcseconds_off_drik_panchangs`,
`the_ingresses_agree_with_drik_panchangs_within_seven_minutes_given_the_shift`,
`every_kumbh_year_of_the_table_meets_a_condition_of_its_site`,
`the_recent_and_announced_festivals_meet_their_conditions`,
`the_festivals_whose_dates_were_read_follow_the_final_entry_found_from_the_sky`,
`the_other_reading_of_the_second_entry_opens_the_festivals_wikipedia_dates`,
`the_year_export_gives_the_festivals_the_sign_export_does`,
`the_ingress_and_pushkaram_exports_agree_on_an_entry` and
`most_years_hold_no_entry_into_a_sign`; and in `hyper_calendar::jupiter_lines`,
`the_kumbh_of_2025_is_met_with_jupiter_computed`,
`the_pushkaram_of_2019_follows_the_final_entry_into_dhanus`,
`a_years_pushkarams_are_those_of_each_sign_by_sky` (for 1971, 1998, 2025 and
2029, and both rules: the year's lines are those of each sign's call) and
`a_year_with_two_entries_has_two_signs_in_order`.

The WebAssembly and C exports `hc_kumbh` and `hc_pushkaram` write a
condition's occasion in a year and a sign's rivers' twelve days, from
`hyper_calendar::reckoning_lines`, in the `calendars` layer. With no
ephemeris of Jupiter there, Jupiter's sidereal sign and the moment of its
entry are arguments the caller gives. `hc_kumbh_by_sky` and
`hc_pushkaram_by_sky`, in the `jupiter` layer, write the same lines with two
more cells each, from `hyper_calendar::jupiter_lines`, Jupiter found: the
WebAssembly module's README has their columns. `hc_pushkarams_in_year` writes
the lines of `hc_pushkaram_by_sky` for each sign of a year at once.
