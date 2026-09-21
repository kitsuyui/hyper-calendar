# hc-units

Units of time that somebody **defined**, held as exact rationals.

## The line this crate draws

A time library gets asked two questions that look like one:

- *How long is a minute?* — sixty seconds, because the BIPM says so. Exact,
  forever, no error bar.
- *How long is a year?* — several answers, and the honest ones have error
  bars. The tropical year is a measurement that drifts. The sidereal day is a
  measurement. The synodic month is a measurement. The galactic year is a
  measurement with a ten-per-cent spread that popular writing prints as a
  single round number.

This crate answers only the first kind. Every unit in it is a fixed multiple
of the SI second fixed by some authority, so every one is an exact
[`Ratio`](src/ratio.rs) and conversions compose without losing anything.

Measured periods live with the model that measured them — `hc-astro` for the
Sun and Moon, `hc-planetary` for other bodies, `hc-deep-time` for the galactic
year and the precession of the equinoxes — with their uncertainties attached.

## Why exact rationals and not `Duration`

`hc_core::Duration` is exact to the attosecond, which covers everything a
clock produces and not everything a *unit* produces. Attoseconds are a decimal
grid, and the units here are the ones chosen for their divisibility:

| Unit | Seconds | Attoseconds |
| --- | --- | --- |
| flick | 1/705 600 000 | 1 417 233 560.0907… |
| helek | 10/3 | 3 333 333 333 333 333 333.33… |
| NTSC frame | 1001/30 000 | 33 366 666 666 666 666.6… |

Storing a flick as 1 417 233 560 attoseconds throws away the one property it
was invented to have. So `Ratio::to_duration` **refuses** rather than rounds,
and `to_duration_rounded` rounds where a caller asked for it.

## What is in the table

| Family | Examples |
| --- | --- |
| SI | quectosecond through gigasecond |
| Civil | minute, hour, day, week, fortnight, Julian year and century, mean Gregorian year/quarter/month |
| Horological | helek and rega (Hebrew), 刻 in both of its lengths and 時辰 (East Asian), ghati, vighati, prana, muhurta (Indian), the medieval moment |
| Decimal | the French Republican decimal hour, minute and second; Swatch `.beat` |
| Hexadecimal | Nystrom's hexadecimal hour, maxime, minute and second |
| Media | flick, FILETIME tick, microfortnight, the 50 Hz and 60 Hz jiffies |
| Scientific | shake, svedberg, the light-centimetre jiffy |

Plus [`media`](src/media.rs) — frame and sample rates as exact periods,
including the NTSC 1000/1001 pull-down — and [`tempo`](src/tempo.rs) — BPM,
note values with dots and tuplets, time signatures, and MIDI ticks.

## Competing conventions get names

Policy §5. The Chinese 刻 is 1/100 of a day in the 百刻 system and 1/96 of a
day after the 時憲曆 reform of 1645. Both are in the table, as `KE_HUNDRED`
and `KE_NINETY_SIX`, rather than one entry with a flag. Likewise the jiffy,
which is 1/60 s, 1/50 s or the light-centimetre depending on who is speaking.

## The table is open

`Unit` is a struct, not an enum. An enum would make this crate the sole
authority on which units exist, and the set is open: somebody with a unit from
a tradition this table has never heard of should be able to write it down and
convert with it, not file a pull request first.

## Anchors

Each is a test:

| Claim | |
| --- | --- |
| A flick divides every frame and sample rate in `media` | exactly |
| A helek | 3⅓ s, 1080 to the hour |
| Swatch `.beat` and the French decimal minute | both 86.4 s |
| 120 BPM | a beat of exactly 0.5 s |
| MIDI's default 500 000 µs per quarter note | 120 BPM exactly |
| 140 BPM in MIDI | *not* exact, and the API says so |
| A flick as a `Duration` | refused, not rounded |

## What this is not

Not a dimensional-analysis library — there is one dimension here and it is
time. And not a way to add three months to a date: that depends on which month
you start in, which makes it `hc-calendar`'s question. `MEAN_GREGORIAN_MONTH`
is named for the average it is so that nobody reaches for it by accident.
