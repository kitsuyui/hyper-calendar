# Jupiter's position from VSOP87B, its signs and its risings

Backs `hc_astro::vsop87_jupiter` and `hc_astro::jupiter` (behind the
`jupiter` feature of `hc-astro`), `hc_seasons::zodiac::jupiter`, the
facade's `jupiter` feature with `hyper_calendar::jupiter_lines`, and the
`jupiter` layer of the WebAssembly module and the C library. No calendar
identifier is registered. Two readings of the sources that disagree are
named as identifiers, `EntryRule`'s `pushkaram-final-entry` and
`pushkaram-first-entry` ([policy.md](../policy.md) §5).

## What it is

Jupiter takes about twelve years round the zodiac, and the twelve-year
festivals of India are set by which sidereal sign it is in: the Kumbh Mela
and Pushkaram ([jupiter-festivals.md](jupiter-festivals.md)), and the
years of Jupiter by its risings. Each is a question of Jupiter's
**longitude**, to a fraction of a degree, on the day. Jupiter moves
0.08° a day on average, 0.25° at its fastest, and stands still at its
stations, so a sign boundary near a station is crossed over days, and the
longitude needs to be good to an arcsecond or two for the moment of a sign
change to be good to hours.

The theory is VSOP87, the Bureau des longitudes' solution for the planets
(Bretagnon and Francou, *Astronomy and Astrophysics* 202, 309, 1988
[bretagnon1988], paper not read). The workspace already takes the Sun from
its series for the Earth ([earth-rotation.md](earth-rotation.md) and
`hc_astro::vsop87`, VSOP87D). This document adds the series for Jupiter,
VSOP87B, from the IMCCE's file `VSOP87B.jup` [vsop87b-jup]: Jupiter's
heliocentric longitude, latitude and radius in the dynamical ecliptic and
equinox of J2000.0, as 18 blocks of terms *A* cos(*B* + *C* τ), τ in Julian
millennia from J2000.0 TDB, for the three variables and the powers τ⁰ to
τ⁵: 3 625 terms. It is one file of 484 519 bytes, and the one file the user
approved for download.

## How it works

### The data, and what it costs

`scripts/vsop87-jupiter.py PATH/VSOP87B.jup` reads the file and writes
`crates/hc-astro/src/vsop87_jupiter/data.rs`; with `--check` it exits 1 if
the committed file is not what the script makes from it. The script refuses
a file that is not the 484 519 bytes of sha256
`408b5938ab39e661940b4253579f4a7a65f206926d98bec20e46aebb06cdd159`, that
has a block of a different length, or a term whose amplitude is not the
hypotenuse of its two printed coefficients (S and C: the check holds for all
3 625 to the file's last digit). The generated file records the source, its
version (B2), its size and its sha256.

Each number of the file has 11 decimals, so the data keeps them exactly: an
amplitude and a phase are stored as integers of 10⁻¹¹ (each divided by 10¹¹
is the nearest `f64` to the decimal the file prints, so nothing is rounded),
and the frequency as an index into the 953 distinct frequencies the file
uses, which many terms share. A term is 13 bytes:

| Part | Bytes | |
| --- | ---: | --- |
| amplitude *A* | 6 | the largest, 529.69, is the mean motion in the τ¹ block of the longitude |
| phase *B* | 5 | under 2π |
| frequency index | 2 | into 953 |
| 3 625 terms | 47 125 | |
| the 953 frequencies, `f64` | 7 624 | |
| **the tables** | **54 749** | against 484 519 bytes of text |

The tables are behind the `jupiter` feature of `hc-astro` and nothing else
turns it on: `hc-seasons` has a `jupiter` feature that passes it through, the
facade's `jupiter` feature turns on both with the crates its lines need, and
the WebAssembly module and the C library have a `jupiter` layer of their own.
A build without the feature carries none of it. In `wasm32` with the
`release-compact` profile the `jupiter` layer is
**216 294 bytes (211 KiB)**, of which the 54.7 kB of tables are about a
quarter and the rest is the festival lines' share of the sky layer's code, the
zodiac and the locale names; the other sixteen layers are unchanged in size, and
`full` grows by 66 kB, 0.9%, to 7.40 MiB. The layer is the one a front
end names to have Jupiter; see the README's table.

### The position

`hc_astro::vsop87_jupiter::jupiter_heliocentric(τ, truncation)` sums the
series. `Truncation::Full` sums all of them; `Truncation::Amplitude(limit)`
drops each term whose largest size over the date, *A* |τ|ᵏ, is below the
limit, in radians and astronomical units. The terms are not in the file in
order of size, so this is a test of each, not a stop. At J2000.0:

| Limit | Terms kept | The largest difference from the full series in longitude, 4000 years |
| --- | ---: | ---: |
| 10⁻⁴ | 29 | 130″ |
| 10⁻⁵ | 68 | 19″ |
| 10⁻⁶ | 168 | 2.5″ |
| 3 × 10⁻⁷ | 258 | 1.1″ |
| 10⁻⁷ | 398 | 0.45″ |
| **10⁻⁸, the default** | **985** | **0.093″** (latitude 0.046″, distance 5 × 10⁻⁷ AU) |
| none | 3 625 | 0 |

The default, `jupiter::DEFAULT_TRUNCATION`, was measured against the full
series at 60 000 dates from 1000 BCE to 3000 CE; a position is some 15 µs
against 37 µs in a release build. Everything below that takes no argument
uses it.

`hc_astro::jupiter::apparent` turns that into what an almanac prints for the
Earth's observer, in six steps (Meeus's chapters 21 and 32 and 33 give the
rotation and the corrections, [meeus1998], not re-read here: each step is
held by the agreement with Horizons below):

1. Jupiter's heliocentric position at the moment the light left it, rotated
   from the ecliptic of J2000.0 to the ecliptic of the date by the angles η,
   Π and *p* of IAU 1976, the precession VSOP87D's Earth is stated in.
2. The Earth's heliocentric position at the moment, from `hc_astro::vsop87`:
   the series the Sun is taken from, VSOP87D, the ecliptic and equinox of
   date.
3. The geocentric vector is their difference; the light-time is its length
   over 499.004 783 8 s an astronomical unit; Jupiter is found again at the
   earlier moment, twice, which brings the light-time to under 3 × 10⁻⁶ day.
4. Annual aberration, by the classical sum of the unit vector and the Earth's
   velocity over the speed of light. The velocity is the Earth's position
   0.0005 day either side of the moment, taken in the J2000.0 frame so that the
   precession of the frame of date is not mistaken for the Earth's motion.
5. The step from the dynamical ecliptic to FK5, a constant 0.090 33″ and a
   periodic 0.039 16″ in latitude, as `solar` applies to the Sun.
6. Nutation in longitude from `hc_astro::earth`, which carries four terms and
   is good to about half an arcsecond, to pass from the mean equinox of the
   date to the true.

The gravitational deflection of light, under a milliarcsecond at Jupiter's
distance from the Sun except within a few degrees of it, is not applied.

**Worked example.** Jupiter at its opposition, 0 h UT on 7 December 2024.
ΔT is 69.14 s, so τ is 0.024 932 240 4 centuries of TT. The series gives
Jupiter at 5.074 452 AU from the Sun, heliocentric longitude 75.830 432° and
latitude −0.544 526° in the ecliptic of J2000.0, which the precession turns to
76.176 696° and −0.541 358° of date. The Earth is at 75.374 972° and
0.985 204 AU, so Jupiter is 4.089 415 AU away, light takes 0.023 618 day
(34 min), and the geocentric longitude is 76.375 605° in the mean equinox of
the date; nutation of −0.000 34° makes it 76.375 262° in the true. JPL
Horizons, DE441, gives 76.3751533°, latitude −0.6718041° and 4.0894152 AU
[jpl-horizons-jupiter]: this is 0.39″, 0.03″ and 90 km from them.

### The sidereal sign

`hc_seasons::zodiac::jupiter::sidereal_longitude` is the apparent
longitude in the true equinox of the date less the ayanāṃśa of the date,
which is how `hc_seasons`' Sun is read. That Jupiter's longitude is measured
from the true equinox and the ayanāṃśa from the mean one is a convention
this was tried against rather than assumed: for the 49 entries of Jupiter
into a sign that Drik Panchang prints from 2001 to 2030, forward and in
retrograde, the sidereal longitude at its time is on the boundary to within
23.9″ to 27.1″, a constant 25.0″, which is this library's Lahiri against
Drik's; without the nutation the difference wanders from 6″ to 43″ with the
node's 18.6-year cycle. With Lahiri raised by that 25″, all 49 entries come
within 6.9 minutes of Drik Panchang's, 1.7 minutes on average. Without it,
this library's entries come 40 to 135 minutes before Drik's going forward and
up to four and a half hours after it coming back, which is the same 25″ at a
slower speed. The tropical zodiac is not an alternative reading of these
festivals: Jupiter's tropical sign is another from the sidereal at every
festival tested, by an ayanāṃśa of 24°.

`ingresses(from, until, ayanamsa)` finds Jupiter's crossings of the sign
boundaries in order. From the distance to the nearest boundary and
Jupiter's greatest speed, 0.3° a day, it steps as far as cannot cross one,
at least 0.1 day, and bisects the step that did to 10⁻⁶ day. Five years take
some 55 ms in a release build. An `Ingress` says whether Jupiter moves
`is_forward` into the next sign or turns back into the one before, which it
does about two years in three, so a year's ingresses come in runs: 2019 has
Dhanus on 29 March (UT), Vṛścika on 22 April and Dhanus again on 5 November.

### Which entry the Pushkaram follows

Where Jupiter enters a sign, turns back out and enters again, the sources
disagree on which entry opens the festival, so each reading is its own
`EntryRule` and its own identifier:

| Identifier | The entry | Read in |
| --- | --- | --- |
| `pushkaram-final-entry` | the forward entry after which Jupiter stays until it moves on to the next sign: the second, where there are two | the Brahmaputra festival of 5 to 16 November 2019 and the Tungabhadra festival of 20 November to 1 December 2020, both of which began at the November entries, the second [sentinel-brahmaputra-2019, kurnool-tungabhadra-2020] |
| `pushkaram-first-entry` | the first forward entry into the sign of a run of entries and returns | Wikipedia's table of the festivals, which opens the 2019 Dhanus festival on 29 March and the 2021 Sindhu festival on 6 April [wikipedia-pushkaram] |

`entry_into(sign, from, until, rule, ayanamsa)` looks 800 days past either end
of the span, because whether an entry is the final one depends on where Jupiter
goes next and whether it is the first on what came before it.

### The risings, and the years of Jupiter by them

Jupiter is lost in the Sun's light when its longitude comes within a
fixed arc of the Sun's, and returns when it has left it. The Indian
astronomers' arc for Jupiter is 11°: Varāhamihira's *Pañcasiddhāntikā* and
Bhāskara I's *Laghubhāskarīya* 7.1–2, as Subbarayappa and Sarma's source book
gives them [subbarayappa1985, the chapter read online], and the *Sūrya
Siddhānta*'s, as Drik Panchang quotes it [drik-guru-asta]; Bhāskara II's is
14° of the *śīghra* anomaly, a different measure. `next_heliacal_rising(from,
arc)` finds the first moment after `from` when Jupiter's longitude east of the
Sun's, which falls as the Sun overtakes it, passes −arc, and the setting before
it, when it passed +arc. Their separation is the *asta*, 26 to 34 days.

The twelve-year cycle of Jupiter by its risings is one of the two kinds Sewell
and Dikshit describe, "a year beginning with Jupiter's heliacal rising" of
about 400 days [sewell1896, Art. 63, as [hindu-calendars.md](hindu-calendars.md)
reads it]. Their source for the names is the *Bṛhatsaṃhitā*, ch. 8, verses 1
and 2: "the years of Jupiter take their names from the several Nakṣatras in
which he reappears after his conjunction with the Sun", the names those of
the lunar months beginning with Kārttika, each following two nakṣatras
beginning from Kṛttikā, "but the fifth, the eleventh and the twelfth years
follow, each, three" [varahamihira-brihat-samhita, Iyer's translation]. That
fixes the table: Kārttika Kṛttikā, Rohiṇī; Mārgaśīrṣa Mṛgaśīrṣa, Ārdrā;
Pauṣa Punarvasu, Puṣya; Māgha Āśleṣā, Maghā; Phālguna the two Phalgunīs and
Hasta; Caitra Citrā, Svātī; Vaiśākha Viśākhā, Anurādhā; Jyaiṣṭha Jyeṣṭhā,
Mūla; Āṣāḍha the two Āṣāḍhās; Śrāvaṇa Śravaṇa, Dhaniṣṭhā; Bhādrapada
Śatabhiṣaj and the two Bhādrapadās; Āśvayuja Revatī, Aśvinī, Bharaṇī:
27 nakṣatras in twelve names. It agrees with Sewell and Dikshit's Table XII,
as carried in `hc-calendars-indic::barhaspatya`: verse 27 begins the cycle of
sixty when Jupiter "reappears at the beginning of the constellation of
Dhaniṣṭhā", which is Śrāvaṇa, and Table XII couples the first of the sixty,
Prabhava, with Śrāvaṇa. A test holds the two together.

**Worked example.** Jupiter rose from the Sun's light on 13 August 2026 at
105.48° sidereal (Lahiri), in Puṣya, which is Pauṣa's; the year of Jupiter
that began is Pauṣa. Drik Panchang puts the asta from 15 July 19:59 to
12 August 05:03 IST for New Delhi [drik-guru-asta]; the arc gives 14 July to
13 August (UT).

## What is carried

- `vsop87_jupiter`: `jupiter_heliocentric`, `Truncation`, `terms_kept`,
  `SOURCE_SHA256` and `SOURCE_TERMS`.
- `jupiter`: `apparent` and `apparent_at_centuries` (a TT argument),
  `Apparent` (tropical longitude, the same in the mean equinox, latitude,
  distance, light-time, and the heliocentric position), `longitude`,
  `daily_motion_degrees`, `DEFAULT_TRUNCATION`.
- `hc_seasons::zodiac::jupiter`: `sidereal_longitude`, `sign_at`, `position`,
  `Ingress`, `ingresses`, `EntryRule`, `entry_into`, `HeliacalRising`,
  `next_heliacal_rising`, `VISIBILITY_ARC_DEGREES`.
- The facade's `jupiter_lines` and the five exports of the `jupiter` layer:
  `hc_jupiter_at` (the position, tropical and sidereal), `hc_jupiter_ingresses`
  (a span's crossings), `hc_jupiter_risings` (a span's risings, with the year's
  name), `hc_kumbh_by_sky` and `hc_pushkaram_by_sky` (the festivals with Jupiter
  found), whose columns are in the WebAssembly module's README. The caller-given
  `hc_kumbh` and `hc_pushkaram` stay in `calendars`.

Not carried:

- **The gravitational deflection of light**, and the nutation beyond the four
  terms `hc_astro::earth` has, which put 0.5″ on Jupiter's longitude, 8 hours
  at a station.
- **A Siddhāntic Jupiter.** A rising is by the true sky and a fixed arc of
  11°; no *Sūrya Siddhānta* almanac's Jupiter, whose longitude differs by
  degrees, is computed, so no almanac's printed year of Jupiter is reproduced.
  The visibility Drik Panchang prints is local and not a fixed arc: its
  windows for New Delhi stand up to 3.9 days from this one in 2024 to 2027, so
  a rising that falls near a nakṣatra boundary may be named differently.
- **The expunction of a year of Jupiter.** The rule names each rising by its
  nakṣatra, which is what verses 1 and 2 say: a year of Jupiter is 399 days and
  Jupiter moves 2.8 nakṣatras between risings while the runs are two or three
  wide, so names are sometimes skipped and sometimes repeated (Phālguna twice
  in 2027 and 2028), where Sewell and Dikshit say one name is expunged every 12
  years or so. The later verses of the chapter that may settle it, and S. B.
  Dikshit's account in the *Indian Antiquary*, vol. XVII, were not read.
- **The arcsecond outside 1500 to 2500.** The series are good to 1″ for
  2 000 years either side of J2000.0 and the layer answers for −1000 to 3000,
  where Jupiter is 2″ off in the year 1 and 11.5″ in 1001 BCE (a day and a
  half at the average speed, weeks at a station).

## Accuracy

Everything is held to JPL Horizons' DE441 [jpl-horizons-jupiter], whose
Jupiter system barycenter (5) is the body that VSOP87 gives.

**The series** (`the_heliocentric_position_agrees_with_horizons_as_vsop87_says`),
heliocentric longitude, latitude and radius in the ecliptic of J2000.0, the
0.09″ of the FK5 frame not removed:

| Date (TDB) | Longitude | Latitude | Radius |
| --- | ---: | ---: | ---: |
| 12 Jan 1001 BCE | −8.3″ | −1.5″ | −3 033 km |
| 1 Jan 1 | −2.1″ | 0.2″ | 333 km |
| 27 Dec 999 | −0.9″ | 0.0″ | 332 km |
| 1 Jan 1500 | −0.3″ | 0.0″ | 87 km |
| 1 Jan 1900 | 0.1″ | −0.1″ | 41 km |
| 1 Jan 1950 | 0.1″ | 0.0″ | 129 km |
| 1 Jan 2000 | 0.2″ | 0.1″ | 42 km |
| 1 Jan 2025 | 0.2″ | 0.1″ | −87 km |
| 1 Jan 2050 | 0.4″ | 0.0″ | 57 km |
| 1 Jan 2100 | 0.4″ | −0.0″ | 16 km |
| 1 Jan 2500 | 0.5″ | 0.1″ | 68 km |
| 1 Jan 3000 | 0.6″ | 0.0″ | 86 km |

That is VSOP87's own figure, "1″ over 2000 years" for Jupiter (as the
documentation is quoted [vsop87-doc-copy]; its check values are in a file not
read: the readme gives none), and a little better in the middle.

**The apparent position** (`the_apparent_position_agrees_with_horizons`), the
longitude in the true equinox of date, at the twelve dates above in TT and
eleven in UT, full series: within 0.45″ from 1900 to 2500 (0.1″ in 1900 and
1950, 0.1″ at J2000.0, 0.3″ in 2025 and 0.8″ in 3000); 0.5″ in 1500; 1.0″ in
999; 2.0″ in the year 1; 11.5″ in 1001 BCE. The latitude is within 0.15″ and
the distance within 400 km from 1500. The default cut adds under 0.1″. At the
average speed 1″ is about 7 minutes; at a station, days.

**The ingresses** (`the_ingresses_agree_with_drik_panchangs_within_seven_minutes_given_the_shift`):
49 entries from 2001 to 2030 within 6.9 minutes of Drik Panchang's, with the
25″ above; the same 25″, to 23.9″ to 27.1″, on each.

**The asta** (`the_risings_of_2024_to_2027_are_within_four_days_of_drik_panchangs`):
the arc's windows for 2024 to 2027 stand 0.8 to 3.9 days from Drik
Panchang's; theirs correspond to an arc of about 10°.

## Sources

- [bretagnon1988]: the theory; the paper not read.
- [vsop87b-jup]: the data, read in the saved file; its sha256, size and
  checks above.
- [vsop87-doc-copy]: what the documentation says, as two copies quote it; the
  IMCCE's `VSOP87.doc` and the CDS ReadMe could not be read.
- [jpl-horizons-jupiter]: DE441 through Horizons: vectors and apparent
  positions at 23 dates. Read 2026-10-03.
- [drik-guru-gochar]: Jupiter's sidereal entries for New Delhi, 2001 to 2030;
  [drik-guru-asta]: its asta, 2024 to 2027.
- [varahamihira-brihat-samhita]: ch. 8 verses 1, 2 and 27 in Iyer's
  translation on wisdomlib; the table of names is read from the verses, the
  page's own footnote of it being cut short; the later verses not read.
- [subbarayappa1985]: the arcs of visibility, read on wisdomlib; the book not
  read.
- [sewell1896]: the twelve-year cycle by risings and the mean-sign kind, Art.
  63; [hindu-calendars.md](hindu-calendars.md) reads it.
- [meeus1998]: the precession of the ecliptic and the corrections to a planet's
  position, chapters 21, 32 and 33: the coefficients are Meeus's as this crate's
  `solar` already carries them, not re-read here.

## Code

`scripts/vsop87-jupiter.py`; `crates/hc-astro/src/vsop87_jupiter.rs` and
`vsop87_jupiter/data.rs` (generated), `crates/hc-astro/src/jupiter.rs`;
`crates/hc-seasons/src/zodiac/jupiter.rs`; `crates/hyper-calendar/src/jupiter_lines.rs`
and the `("jupiter", …)` rows of `exports.rs`. The tests that anchor them:
`the_heliocentric_position_agrees_with_horizons_as_vsop87_says`,
`the_apparent_position_agrees_with_horizons`,
`the_default_cut_is_close_to_the_full_series`,
`the_first_and_the_last_term_are_the_files` (`hc-astro`);
`jupiter_at_its_opposition_of_2024_is_in_vrishabha_in_retrograde`,
`the_ingresses_are_in_order_and_chain`,
`a_rising_asked_for_in_the_asta_is_the_one_that_ends_it`
(`hc-seasons`); `the_sidereal_longitude_stands_a_constant_25_arcseconds_off_drik_panchangs`,
`the_ingresses_agree_with_drik_panchangs_within_seven_minutes_given_the_shift`,
`every_kumbh_year_of_the_table_meets_a_condition_of_its_site`,
`the_recent_and_announced_festivals_meet_their_conditions`,
`the_festivals_whose_dates_were_read_follow_the_final_entry_found_from_the_sky`,
`the_other_reading_of_the_second_entry_opens_the_festivals_wikipedia_dates`
(`crates/hyper-calendar/tests/jupiter_festivals.rs`);
`the_year_of_jupiter_is_named_by_the_nakshatra_of_its_rising`,
`the_first_year_of_the_sixty_is_named_for_dhanishtha_as_sewells_table_has_it`,
`the_risings_of_2024_to_2027_are_within_four_days_of_drik_panchangs`
(`jupiter_lines`).
