# `hc-calendars-indic`

The calendars of the Indian subcontinent for [`hyper-calendar`], beginning
with the one most of India dates its festivals in: the Hindu lunisolar
calendar in its *amānta* form, computed from the true Sun and Moon the way
the Government of India's *Rashtriya Panchang* computes it.

## What is here

| Module | Identifier | What it is | Range |
|---|---|---|---|
| `hindu_lunar` | `hindu-lunar` | months new moon to new moon, named for the saṅkrānti they contain; the day is the tithi at sunrise; Śaka years | Gregorian 1700–2299 |
| `hindu_purnimanta` | `hindu-lunar-purnimanta` | the same tithis under the north's names: the dark fortnight first, named for the bright one that follows; the intercalary month inserted whole | Gregorian 1700–2299 |
| `hindu_solar` | `hindu-solar-tamil` | the Sun's stay in each sidereal sign; the month begins on the saṅkrānti's day unless it fell after sunset; Tiruvaḷḷuvar years | Gregorian 1700–2299 |
| | `hindu-solar-malayalam` | the same months from Chingam; the month begins on the saṅkrānti's day unless it fell after three fifths of the daylight; Kollam era | |
| | `hindu-solar-bengali` | the same months from Boishakh; the month begins the day after the saṅkrānti's; Bengali San | |
| | `hindu-solar-vikrami` | the same months from Vaiśākha; the month begins on the sunrise-to-sunrise day of the saṅkrānti; Vikrama Saṃvat — Punjab, Haryana, Odisha | |
| `tithi` | — | the lunar day: which tithi is in progress at a moment, and which a civil day carries | |
| `nakshatra` | — | the Moon's station among the twenty-seven: which is in progress at a moment, and when the Moon enters and leaves one; and the Sun's, the almanacs' Sūrya nakṣatra transits by which Kerala's ñāṭṭuvēla and the Deccan's rain nakṣatras are counted | |
| `hindu_old` | `hindu-old-solar` | the *Ārya Siddhānta*'s mean Sun: twelve months of a twelfth of a 365.258 68-day year, named for the signs; Kali Yuga years | Kali Yuga 0–10000 |
| | `hindu-old-lunar` | its mean Moon: 29.530 58-day months named for the solar month that begins within them, the intercalary one being the month no solar month begins in, thirty mean tithis a month; Kali Yuga years | Kali Yuga 0–10000 |
| `nepal_sambat` | `nepal-sambat` | the Newar lunisolar calendar: the amānta months under their Newar names from Kachhalā (Kārtika), the year opening at Mha Puja, the day read at Kathmandu's sunrise; Nepal Sambat years | Gregorian 1700–2299 |
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

The *Rashtriya Panchang* itself, Śaka 1945 and 1946 (2023–2025), published
by the Positional Astronomy Centre of the India Meteorological Department:
the first day of every lunar fortnight it tabulates, the intercalary
Śrāvaṇa of 1945, the ayanamsa it prints at the head of each month, the
festival dates it lists where those are the tithi at sunrise — and, for
the solar reckonings, its "Regional Calendars" tables: the first day of
every month of every reckoning for both years, ninety-six dates, from
which the four rules were read rather than assumed.

The Bikram Sambat is held to the Government of Nepal's gazette: the
notices fixing the public holidays of 2080–2083 BS, which list every
Saturday of the year by its Bikram Sambat date. The calendar reproduces
every listed Saturday, the weekday of each New Year's Day, and the one
Gregorian date the notices name, Christmas 2025 as Pus 10. The
*Sūrya Siddhānta* reckoning it falls back on outside those years begins 47
of the 48 gazetted months on the gazette's day; the test states the one it
misses, Magh 2082, a day early. The *Sūrya Siddhānta* model reproduces the
classical table of sines and is checked against the modern Sun for how far
its saṅkrāntis fall from the Lahiri ones.

The two Old Hindu calendars have no such table — the *Panchang* tabulates
the true calendars, not the mean ones they replaced — so they are held to
what an arithmetic calendar must do instead: every one of the 3 652 952
days in their range converts and converts back, months and years have the
lengths the mean motions allow, an intercalary month falls seven years in
nineteen and precedes the month it is named for, both calendars place the
Kali Yuga epoch on the same day as the sources, and the mean months of
2024–2025 are checked against the true ones above, which they run within
two days of.

## What is not here yet

The Odia year counts and the Tamil sixty-year names; the Bikram Sambat's
gazetted months before 2080 and after 2083, which would replace the
reckoning in those years. `docs/calendars.md` tracks each.

## Features

`std` (default) and `alloc`; with neither, the calendar still converts and
only the registry is absent.

[`hyper-calendar`]: https://github.com/kitsuyui/hyper-calendar
