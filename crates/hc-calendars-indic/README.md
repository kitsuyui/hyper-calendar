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
| | `hindu-solar-vikrami` | the same months from Vaiśākha; the month begins on the sunrise-to-sunrise day of the saṅkrānti; Vikrama Saṃvat — Punjab, Odisha, and the Nepali reckoning | |
| `tithi` | — | the lunar day: which tithi is in progress at a moment, and which a civil day carries | |
| `places` | — | the Central Station of the national calendar (82°30′ E), Ujjain, New Delhi | |

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

## What is not here yet

The Nepali Bikram Sambat as its committee publishes it, which follows the
Vikrami rule and has not yet been compared against it; the Odia year counts
and the Tamil sixty-year names. `docs/calendars.md` tracks each.

## Features

`std` (default) and `alloc`; with neither, the calendar still converts and
only the registry is absent.

[`hyper-calendar`]: https://github.com/kitsuyui/hyper-calendar
