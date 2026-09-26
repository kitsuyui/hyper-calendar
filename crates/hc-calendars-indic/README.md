# `hc-calendars-indic`

The calendars of the Indian subcontinent for [`hyper-calendar`], beginning
with the one most of India dates its festivals in: the Hindu lunisolar
calendar in its *amānta* form, computed from the true Sun and Moon the way
the Government of India's *Rashtriya Panchang* computes it.

The Hindu calendars — the amānta and pūrṇimānta months, the four solar
reckonings, the Old Hindu arithmetic, the nakṣatras and the ayanamsa —
are written up in [`docs/systems/hindu-calendars.md`](../../docs/systems/hindu-calendars.md):
what each is, how it works with a worked example, what is carried, how
it was checked against the almanac, and where every statement comes from.
This README summarises the crate; the module documentation summarises
each module.

## What is here

| Module | Identifier | What it is | Range |
|---|---|---|---|
| `hindu_lunar` | `hindu-lunar` | months new moon to new moon, named for the saṅkrānti they contain; the day is the tithi at sunrise; Śaka years | Gregorian 1700–2299 |
| `hindu_purnimanta` | `hindu-lunar-purnimanta` | the same tithis under the north's names: the dark fortnight first, named for the bright one that follows; the intercalary month inserted whole | Gregorian 1700–2299 |
| `hindu_solar` | `hindu-solar-tamil` | the Sun's stay in each sidereal sign; the month begins on the saṅkrānti's day unless it fell after sunset; Śaka years from Chithirai, each named in the southern sixty-year cycle as the extra `samvatsara`, with the Tiruvaḷḷuvar year, which turns at Thai 1, as the extra `tiruvalluvar-year` | Gregorian 1700–2299 |
| | `hindu-solar-malayalam` | the same months from Chingam; the month begins on the saṅkrānti's day unless it fell after three fifths of the daylight; Kollam era | |
| | `hindu-solar-bengali` | the same months from Boishakh; the month begins the day after the saṅkrānti's; Bengali San | |
| | `hindu-solar-vikrami` | the same months from Vaiśākha; the month begins on the sunrise-to-sunrise day of the saṅkrānti; Vikrama Saṃvat — Punjab, Haryana, Odisha | |
| `tithi` | — | the lunar day: which tithi is in progress at a moment, and which a civil day carries | |
| `nakshatra` | — | the Moon's station among the twenty-seven: which is in progress at a moment, and when the Moon enters and leaves one; and the Sun's, the almanacs' Sūrya nakṣatra transits, by which Kerala's ñāṭṭuvēla are counted | |
| `hindu_old` | `hindu-old-solar` | the *Ārya Siddhānta*'s mean Sun: twelve months of a twelfth of a 365.258 68-day year, named for the signs; Kali Yuga years | Kali Yuga 0–10000 |
| | `hindu-old-lunar` | its mean Moon: 29.530 58-day months named for the solar month that begins within them, the intercalary one being the month no solar month begins in, thirty mean tithis a month; Kali Yuga years | Kali Yuga 0–10000 |
| `nepal_sambat` | `nepal-sambat` | the Newar lunisolar calendar: the amānta months under their Newar names from Kachhalā (Kārtika), the year opening at Mha Puja, the day read at Kathmandu's sunrise; Nepal Sambat years | Gregorian 1700–2299 |
| `vira_nirvana` | `vira-nirvana-samvat` | the Jain era of Mahāvīra's nirvāṇa over the amānta months, the year opening at Kārtika śukla 1, the day after Dīpāvalī, 605 years after the Śaka year's Kārtika; the day read at the Central Station's sunrise | Gregorian 1700–2299 |
| `odia_anka` | `odia-anka` | the regnal years of the Gajapati of Puri, Dibyasingha Deb: the *aṅka* turns at Suniā, nija Bhādrapada śukla 12, over the pūrṇimānta months, and never takes 1, a number ending in 6, or one ending in 0 but 10; an integer mapping from the full year of the reign, with the Amli year beside it — see [`docs/systems/odia-anka.md`](../../docs/systems/odia-anka.md) | Suniā 1970 to 2299 |
| `samvatsara` | — | the southern sixty-year cycle of year names, Prabhava to Kṣaya, by Sewell and Dikshit's rule on the Śaka year | |
| `bikram_sambat` | `bikram-sambat` | the solar calendar of Nepal, Baisakh to Chait: the months the Government of Nepal gazettes for 2080–2083 BS, and elsewhere the *Sūrya Siddhānta*'s saṅkrāntis on their civil day at Kathmandu; Bikram Sambat years | Gregorian 1700–2299 |
| `surya_siddhanta` | — | the Sun of the *Sūrya Siddhānta*: its sidereal longitude, the sign it stands in, and its saṅkrāntis | |
| `places` | — | the Central Station of the national calendar (82°30′ E), Ujjain, New Delhi, Kathmandu | |

The calendar is judged at a place — a tithi that ends within an hour of
sunrise belongs to different days in Delhi and Chennai — and with an
ayanamsa. `HinduLunarCalendar::RASHTRIYA` is the registered one: sunrise at
the Central Station, the Lahiri ayanamsa, as the national almanac has it.
`HinduLunarCalendar::UJJAIN` is the classical reference, and
`HinduLunarCalendar::new` takes any place and any ayanamsa `hc-seasons`
knows.

## What the tests are

For the Hindu calendars, the *Rashtriya Panchang* itself, Śaka 1945 and
1946 (2023–2025): the first day of every lunar fortnight, the intercalary
Śrāvaṇa of 1945, the printed ayanamsa, the festival dates that are the
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

The Tamil year names are held to Sewell and Dikshit's worked examples of
1752, 1803–04 and 1822, to the Tamil New Year of 2024 (Krodhi) and 2025
(Visvavasu), and in Tamil script to the *Tamil Lexicon*'s list. The Odia
Anka is held to Sewell and Dikshit's rule and their four reigns' first
days of the 2nd Anka, 1797 to 1859, and to the Anka the Gajapati declared
at Suniā in 2011 and 2024–2026.

The two Old Hindu calendars have no such table — the *Panchang* tabulates
the true calendars, not the mean ones they replaced — so they are held to
what an arithmetic calendar must do instead, and to the true months of
2024–2025, which they run within two days of.

## What is not here yet

The Odia Amli and Vilayati years with their solar months; the sixty-year
names on the southern lunisolar year, and the northern Bārhaspatya cycle
with its expunged years; the Bikram Sambat's gazetted months before 2080
and after 2083, which would replace the reckoning in those years.
`docs/calendars.md` tracks each.

## Features

`std` (default) and `alloc`; with neither, the calendar still converts and
only the registry is absent.

[`hyper-calendar`]: https://github.com/kitsuyui/hyper-calendar
