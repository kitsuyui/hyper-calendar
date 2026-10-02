# `hc-calendars-indic`

The calendars of the Indian subcontinent for [`hyper-calendar`], beginning
with the one most of India dates its festivals in: the Hindu lunisolar
calendar in its *amānta* form, computed from the true Sun and Moon the way
the Government of India's *Rashtriya Panchang* computes it.

The Hindu calendars — the amānta and pūrṇimānta months, the four regional
solar reckonings, the Old Hindu arithmetic, the nakṣatras, the yoga and
karaṇa, and the ayanāṃśa — are written up in
[`docs/systems/hindu-calendars.md`](../../docs/systems/hindu-calendars.md):
what each is, how it works with a worked example, what is carried, how
it was checked against the almanac, and where every statement comes from.
The eras of Sewell and Dikshit's Art. 71 over those months and the Faṣlī
years are written up the same way in
[`docs/systems/indian-eras.md`](../../docs/systems/indian-eras.md). This
README summarises the crate; the module documentation summarises each
module.

## What is here

| Module | Identifier | What it is | Range |
|---|---|---|---|
| `hindu_lunar` | `hindu-lunar` | months new moon to new moon, named for the saṅkrānti they contain; the day is the tithi at sunrise; Śaka years from Chaitra śukla 1, each named in the southern sixty-year cycle as the extra `samvatsara`, as at the Telugu and Kannada Ugādi | Śaka 1622–2221: Chaitra śukla 1 in March 1700 to the eve of the one in March 2300 |
| | `hindu-lunar-reingold-dershowitz` | the amānta months and tithis read at Ujjain's sunrise with the book's own ayanāṃśa: Reingold and Dershowitz's astronomical Hindu lunisolar calendar | as `hindu-lunar` |
| `hindu_lunar_siddhanta` | `hindu-lunar-surya-siddhanta` | the same months and tithis on the *Sūrya Siddhānta*'s Sun and Moon, the day read at its own sunrise at Ujjain: Reingold and Dershowitz's modern Hindu lunisolar calendar, and the reckoning of the almanacs that compute by the Siddhānta | Kali Yuga 1–10000 (3101 BCE to 6899 CE) |
| `hindu_purnimanta` | `hindu-lunar-purnimanta` | the same tithis under the north's names: the dark fortnight first, named for the bright one that follows; the intercalary month inserted whole; each year named in the northern Bārhaspatya cycle, by the *Sūrya Siddhānta* with the *bīja*, as the extra `barhaspatya-samvatsara`, a key apart from the southern `samvatsara` | Śaka 1622–2221: Chaitra śukla 1 in March 1700 to the eve of the one in March 2300 |
| `hindu_solar` | `hindu-solar-tamil` | the Sun's stay in each sidereal sign; the month begins on the saṅkrānti's day unless it fell after sunset; Śaka years from Chithirai, each named in the southern sixty-year cycle as the extra `samvatsara`, with the Tiruvaḷḷuvar year, which turns at Thai 1, as the extra `tiruvalluvar-year` | roughly Gregorian 1700–2299: the years opening in 1700 to 2299 |
| | `hindu-solar-malayalam` | the same months from Chingam; the month begins on the saṅkrānti's day unless it fell after three fifths of the daylight; Kollam era | |
| | `hindu-solar-bengali` | the same months from Boishakh; the month begins the day after the saṅkrānti's; Bengali San | |
| | `hindu-solar-vikrami` | the same months from Vaiśākha; the month begins on the sunrise-to-sunrise day of the saṅkrānti; Vikrama Saṃvat — the months of Punjab and Haryana, which Odisha keeps under the years of `odia-anka` | |
| | `hindu-solar-reingold-dershowitz` | the Tamil rule on the true Sun read at Ujjain with the book's ayanāṃśa and sunset, months named by the signs, Śaka years: their astronomical Hindu solar calendar | as `hindu-solar-tamil` |
| | `magi-san` | the Bengali months and days under the Magi San of Chittagong, the Bengali San less 45 | |
| `hindu_solar_siddhanta` | `hindu-solar-surya-siddhanta` | the months on the *Sūrya Siddhānta*'s Sun, named by their signs; the month begins on the day whose closing sunrise at Ujjain, by the Siddhānta, is the first in the new sign; the Siddhānta's Śaka years: Reingold and Dershowitz's modern Hindu solar calendar | Kali Yuga 1–10000 (3101 BCE to 6899 CE) |
| `tithi` | — | the lunar day: which tithi is in progress at a moment, and which a civil day carries | |
| `nakshatra` | — | the Moon's station among the twenty-seven: which is in progress at a moment, and when the Moon enters and leaves one; and the Sun's, the almanacs' Sūrya nakṣatra transits, by which Kerala's ñāṭṭuvēla are counted, with the twenty-seven's Malayalam names, which the ñāṭṭuvēla take | |
| `panchanga` | — | the yoga, from the sum of the Sun's and Moon's sidereal longitudes, and the karaṇa, the half-tithi: which is in progress at a moment or a sunrise, and when it ends, and the part of a tithi Viṣṭi (Bhadra) does not cover, `vishti_free_span`, which Raksha Bandhan waits for; on the *Sūrya Siddhānta*'s Sun and Moon, `surya_siddhanta`'s `yoga_at` and `karana_at` | |
| `amrita_siddhi` | — | the *amṛta siddhi yoga* of Sewell and Dikshit's Art. 39, a weekday with its nakṣatra, Hasta on Sunday to Rohiṇī on Saturday: the part of the day, sunrise to sunrise, the Moon spends in it (Drik Panchang's January 2025 to 1.5 min, and Art. 30's three days of September 1894) | |
| `kalam` | — | Rāhu kālam, Yamaganda and Gulika kālam: the eighth of the day each takes by the weekday, from local sunrise to sunset (`kalam::by_sunrise`, within 0.55 min of Drik Panchang's New Delhi times for 1–7 January 2025) or from 06:00 to 18:00 of the clock (`kalam::by_fixed_day`) — see [`docs/systems/rahu-kalam.md`](../../docs/systems/rahu-kalam.md) | |
| `muhurta` | — | the fifteen muhūrtas of the daylight and of the night; Abhijit, the eighth of the day, none on a Wednesday, and Dur Muhurtam, one or two by the weekday, as Drik Panchang prints them (within a minute of its New Delhi times for January 2025) — see [`docs/systems/rahu-kalam.md`](../../docs/systems/rahu-kalam.md) | |
| `choghadiya` | — | the eighths of the daylight and of the night at a place, each named for one of seven kinds by the weekday, as Drik Panchang prints them (every start within 0.59 min of its New Delhi times for 1–7 January 2025) | |
| `panchak` | — | the Moon's passage from 300° to 360° sidereal, the third quarter of Dhaniṣṭhā to the end of Revatī, and its kind by the weekday it opens on, in two tables, `panchak-five-kinds` and `panchak-raj-midweek` (within a minute of Drik Panchang's windows of 2025) | |
| `kumbh` | — | the seven conditions of Jupiter, the Sun and the Moon under which the Kumbh Mela is held at Haridwar, Prayag, Nashik and Ujjain, and the occasion in a year on which each holds; Jupiter's sign is the caller's | |
| `pushkaram` | — | the rivers of the twelve signs, and the twelve days of the *Ādi Pushkaram* from Jupiter's entry into a sign, which the caller supplies | |
| `vaishnava` | — | the Vaiṣṇava reading of a festival's day: Krṣṇa Janmāṣṭamī on the first sunrise of Śrāvaṇa kṛṣṇa at or after Ashtami (Drik Panchang's ISKCON dates, Tokyo, 2024–2034); the Vaiṣṇava Ekadashi measured and not given as a rule | |
| `hindu_old` | `hindu-old-solar` | the *Ārya Siddhānta*'s mean Sun: twelve months of a twelfth of a 365.258 68-day year, named for the signs; Kali Yuga years | Kali Yuga 0–10000 |
| | `hindu-old-lunar` | its mean Moon: 29.530 58-day months named for the solar month that begins within them, the intercalary one being the month no solar month begins in, thirty mean tithis a month; Kali Yuga years | Kali Yuga 0–10000 |
| `nepal_sambat` | `nepal-sambat` | the Newar lunisolar calendar: the amānta months under their Newar names from Kachhalā (Kārtika), the year opening at Mha Puja, the day read at Kathmandu's sunrise; Nepal Sambat years; the fortnights *thwa* and *gā* and the seven Newar tithi names Wikipedia's table gives | the amānta engine's, Chaitra śukla 1 in March 1700 to the eve of the one in March 2300 |
| | `nepal-sambat-fortnight` | the same days as `nepal-sambat` written as the Nepal Panchang Nirnayak Bikas Samiti's notice of 2024 writes them: the fortnight, a month's name with *thwa* or *gā* joined to it (कछलाथ्व, कछलागा, …; अनलाथ्व, अनलागा), and the tithi within it, 1–15 and 30 for the new moon; the notice gives no form for the weekday it requires beside a doubled tithi, so none is written | as `nepal-sambat` |
| `vira_nirvana` | `vira-nirvana-samvat` | the Jain era of Mahāvīra's nirvāṇa over the amānta months, the year opening at Kārtika śukla 1, the day after Dīpāvalī, 605 years after the Śaka year's Kārtika; the day read at the Central Station's sunrise | the amānta engine's, Chaitra śukla 1 in March 1700 to the eve of the one in March 2300 |
| `odia_anka` | `odia-anka` | the regnal years of the Gajapati of Puri, Dibyasingha Deb: the *aṅka* turns at Suniā, nija Bhādrapada śukla 12, over the pūrṇimānta months, and never takes 1, a number ending in 6, or one ending in 0 but 10; an integer mapping from the full year of the reign, with the Amli year beside it — see [`docs/systems/odia-anka.md`](../../docs/systems/odia-anka.md) | Suniā 1970 to 2299 |
| `lunar_era` | `vikram-samvat-kartikadi` | the Gujarati Vikrama year from Kārttika śukla 1 over the amānta months, numbered from Kārttika — see [`docs/systems/indian-eras.md`](../../docs/systems/indian-eras.md) | the amānta engine's, Chaitra śukla 1 in March 1700 to the eve of the one in March 2300 |
| | `rajyabhisheka-saka` | Śivājī's era from Jyeṣṭha śukla 13 over the amānta months, year 1 from the coronation of 1674 | |
| | `saptarshi` | the Saptarṣi era of Kashmir from Chaitra śukla 1 over the pūrṇimānta months, counted in full from Kali 27 current, with the Laukika year of the dropped hundreds as the extra `laukika-year` | |
| | `gupta` | the Gupta era from Chaitra śukla 1 over the *Sūrya Siddhānta*'s pūrṇimānta months, current years, Gupta 1 opening on 26 February 320 (Julian) | Kali Yuga 1–10000 |
| | `valabhi` | the Valabhī era, the Gupta count thrown back to Kārttika śukla 1, over the Siddhānta's amānta months | Kali Yuga 1–10000 |
| | `kalachuri` | the Chedi or Kalachuri era from Āśvina śukla 1 over the Siddhānta's pūrṇimānta months, Chedi 1 opening on 5 September 248 (Julian) | Kali Yuga 1–10000 |
| | `lakshmana-sena` | Kielhorn's reading of the Lakṣmaṇa Sena era of Mithila, Kārttikādi over the Siddhānta's amānta months, in current years (907 in October 2026) | Kali Yuga 1–10000 |
| `fasli` | `fasli-madras` | the Faṣlī revenue year of Madras from 1 July, over the Gregorian months and days | 13 July 1855 to the year from 1 July 2299 |
| | `fasli-bombay` | the Faṣlī year of Bombay from the Sun's entry into Mṛgaśira, the sunrise-to-sunrise day at Ujjain of the Lahiri ingress, over the Gregorian months and days, a later opening repeating its day | roughly Gregorian 1700–2299: the years opening in 1700 to 2299 |
| | `sur-san` | the Maratha Sūr-san: the Bombay days, nine years behind | roughly Gregorian 1700–2299: the years opening in 1700 to 2299 |
| `year_start` | — | where an era's year opens among the amānta months, the arithmetic Nepal Sambat, the Vira Nirvana Samvat and the eras above share | |
| `samvatsara` | — | the southern sixty-year cycle of year names, Prabhava to Kṣaya, by Sewell and Dikshit's rule on the Śaka year | |
| `barhaspatya` | — | the northern sixty-year cycle by Jupiter's mean motion: Sewell and Dikshit's rule for the *Sūrya Siddhānta*, with and without the *bīja*, and the *Ārya Siddhānta*; the name at a Meṣa saṅkrānti, the name a rule couples with a Śaka year (`of_saka`: with the *bīja* as Drik Panchang names Vikrama 2081–83, without it as the Hindi press's New Year announcements do), the name a year expunges, the name in progress at a moment; the twelve-year cycle of the mean-sign system (Art. 63), Chaitra to Phālguna, coupled with the sixty as Table XII couples them, with Jupiter's mean sign | |
| `bikram_sambat` | `bikram-sambat` | the solar calendar of Nepal, Baisakh to Chait: the months the Government of Nepal gazettes for 2080–2083 BS, and elsewhere the *Sūrya Siddhānta*'s saṅkrāntis on their civil day at Kathmandu; Bikram Sambat years | roughly Gregorian 1700–2299: the years opening in 1700 to 2299 |
| `surya_siddhanta` | — | the Sun and Moon of the *Sūrya Siddhānta*: their sidereal longitudes, the sign the Sun stands in and its saṅkrāntis, the elongation, the tithi and the conjunction, and the Siddhānta's sunrise | |
| `places` | — | the Central Station of the national calendar (82°30′ E), Ujjain, New Delhi, Kathmandu | |

The calendar is judged at a place — a tithi that ends within an hour of
sunrise belongs to different days in Delhi and Chennai — and with an
ayanāṃśa. `HinduLunarCalendar::RASHTRIYA` is the registered one: sunrise at
the Central Station, the Lahiri ayanāṃśa, as the national almanac has it.
`HinduLunarCalendar::UJJAIN` is the classical reference, and
`HinduLunarCalendar::new` takes any place and any ayanāṃśa `hc-seasons`
knows, and gives it the identifier of the ayanāṃśa's convention, not the
place's: `hindu-lunar` for Lahiri's, `hindu-lunar-raman` for Raman's, and
`hindu-lunar-other-ayanamsa` for an anchor `hc-seasons` does not name (§5;
the calendars over it carry the same suffix, and only the Lahiri ones are
registered). Ujjain is where Reingold and Dershowitz read their astronomical
calendars. Ujjain with the Lahiri ayanāṃśa is a constant and not a
registered calendar, because the place is its only difference from
`hindu-lunar` and a place is a parameter: over 2000–2030 the two give
different dates on 302 of 11 323 days, and the Tamil rule read there moves
5 of 372 month starts. The book's own ayanāṃśa, zero at the *Sūrya
Siddhānta*'s Meṣa saṅkrānti of 285 CE, is `hc-seasons`'s
`Ayanamsa::REINGOLD_DERSHOWITZ`, and its Tamil sunset, the Sun's centre on
the geometric horizon, is `SankrantiRule::BeforeCentreSets`. Those are
conventions of the book's, so its two astronomical calendars are
registered under its name, `hindu-lunar-reingold-dershowitz`
(`HinduLunarCalendar::REINGOLD_DERSHOWITZ`) and
`hindu-solar-reingold-dershowitz` (`hindu_solar::REINGOLD_DERSHOWITZ`):
they give the book's value on every sample date but the four its errata
explain, the lunisolar one agrees with `UJJAIN` on every day of 2000–2030,
and the solar one moves one more Tamil month start of the 372.
`hindu-lunar-surya-siddhanta` is a registered calendar, because its Sun,
Moon and sunrise are another convention: it gives Reingold and
Dershowitz's value for all 33 of their sample dates, 586 BCE to 2094, and
parts from `HinduLunarCalendar::UJJAIN` on 1 389 days of 2000–2030. So is
`hindu-solar-surya-siddhanta`, the solar months on the same Sun and
sunrise, which gives the book's value for all 33 dates too.

## What the tests are

For the Hindu calendars, the *Rashtriya Panchang* itself, Śaka 1945 and
1946 (2023–2025): the first day of every lunar fortnight, the intercalary
Śrāvaṇa of 1945, the printed ayanāṃśa, the festival dates that are the
tithi at sunrise, and the ninety-six first days of the solar months in
its "Regional Calendars" tables; for the nakṣatras, Drik Panchang's
transit times. The system document lists every check and its result.

Nepal's two calendars are written up in
[`docs/systems/nepal-calendars.md`](../../docs/systems/nepal-calendars.md).
The Bikram Sambat is held to the Government of Nepal's holiday notices for
2080–2083 BS, every Saturday they list and every weekday they name, and
the Nepal Sambat to the published Mha Puja dates of 2013 to 2017; the
system document lists every check and its result, and the one month the
*Sūrya Siddhānta* reckoning misses. The *Sūrya Siddhānta* model itself
reproduces the classical table of sines and is checked against the modern
Sun for how far its saṅkrāntis fall from the Lahiri ones.

The *Sūrya Siddhānta*'s yoga and karaṇa are held to the pañcāṅga extract
Sewell and Dikshit print for Poona in September 1894, the amṛta siddhi
yoga to that extract's three days and to Drik Panchang's New Delhi pages
of January 2025, and Abhijit and Dur Muhurtam to the same pages; the
twelve-year cycle of Jupiter to Table XII.

The Tamil year names are held to Sewell and Dikshit's worked examples of
1752, 1803–04 and 1822, to the Tamil New Year of 2024 (Krodhi) and 2025
(Visvavasu), and in Tamil script to the *Tamil Lexicon*'s list; the same
names on the lunisolar year to the Telugu almanac's Ugādi years of 2024
to 2026. The northern cycle is held to Sewell and Dikshit's three worked
examples of the rule, to their list of expunged names, to Table I's
saṅkrānti of 1514, and to Pingala for Vikrama 2081 as a Varanasi almanac
prints it. The Odia
Anka is held to Sewell and Dikshit's rule and their four reigns' first
days of the 2nd Anka, 1797 to 1859, and to the Anka the Gajapati declared
at Suniā in 2011 and 2024–2026.

The eras over the Hindu months are held to Sewell and Dikshit's
equations and epochs, to the Gujarati Samvat Drik Panchang prints for
2024–2025, to the Rājyābhiṣeka years Raigad keeps by the tithi in 2023
and 2026, and to Saptarṣi 5100 from Navreh 2024; the Faṣlī years to
Sewell and Dikshit's Faṣlī 1302 of 1892 in both places, Swamikannu
Pillai's Faṣlī 1320 of 1910–11 and his and their Bombay openings, and
Faṣlī 1410 of 2000–01. Every year boundary of each era is checked over
the whole range, and every Faṣlī day round-trips in a release build.

The two Old Hindu calendars have no such table — the *Panchang* tabulates
the true calendars, not the mean ones they replaced — so they are held to
what an arithmetic calendar must do instead, and to the true months of
2024–2025, which they run within two days of.

## What is not here yet

The Odia Amli and Vilayati years with their solar months, which Sewell and
Dikshit do not give enough to reproduce; the Bikram Sambat's gazetted
months before 2080 and after 2083, which would replace the reckoning in
those years.
`docs/calendars.md` tracks each.

## Features

`std` (default) and `alloc`; with neither, the calendar still converts and
only the registry is absent.

[`hyper-calendar`]: https://github.com/kitsuyui/hyper-calendar
