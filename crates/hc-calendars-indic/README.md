# `hc-calendars-indic`

The calendars of the Indian subcontinent for [`hyper-calendar`], beginning
with the one most of India dates its festivals in: the Hindu lunisolar
calendar in its *amānta* form, computed from the true Sun and Moon the way
the Government of India's *Rashtriya Panchang* computes it.

## What is here

| Module | Identifier | What it is | Range |
|---|---|---|---|
| `hindu_lunar` | `hindu-lunar` | months new moon to new moon, named for the saṅkrānti they contain; the day is the tithi at sunrise; Śaka years | Gregorian 1700–2299 |
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

The *Rashtriya Panchang* itself, Śaka 1945 to 1947 (2023–2026), published
by the Positional Astronomy Centre of the India Meteorological Department:
the first day of every lunar month it tabulates, the intercalary Śrāvaṇa of
1945, the ayanamsa it prints at the head of each month, and the festival
dates it lists where those are the tithi at sunrise.

## What is not here yet

The *pūrṇimānta* reckoning of northern India, which counts the dark
fortnight first under the next month's name; the solar calendars of the
south and east (Tamil, Malayalam, Bengali, Odia), whose months
`hc-seasons` already names; and the festival rules that read a tithi at
midday or in the evening rather than at sunrise, which belong to
`hc-holiday`. `docs/calendars.md` tracks each.

## Features

`std` (default) and `alloc`; with neither, the calendar still converts and
only the registry is absent.

[`hyper-calendar`]: https://github.com/kitsuyui/hyper-calendar
