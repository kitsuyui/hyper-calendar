# Choghadiya

Backs `hc-calendars-indic::choghadiya`. No calendar identifier is
registered: these are periods of a day and a night, not calendars.

## What it is

A choghadiya is one of the eight equal parts of the daylight or of the
night that almanacs of western India print for choosing a time to begin
something [wikipedia-choghadiya]. Each part is named for one of seven
kinds. Amrita, Shubha and Labha are auspicious, Chara is neutral, and
Udvega, Kala and Roga are inauspicious [wikipedia-choghadiya,
astromedha-choghadiya]. Drik Panchang prints the sixteen parts of every
day and place [drik-choghadiya-2025].

## How it works

The daylight, sunrise to sunset, is cut into eight equal parts, and the
night, sunset to the next sunrise, into eight more [wikipedia-choghadiya].
The parts of the day and of the night are therefore of different lengths,
except near the equinoxes.

Each kind is ruled by a planet [astromedha-choghadiya], and the kinds run
in this fixed cycle:

| Place | Kind | Drik Panchang's gloss | Ruler | Quality |
| --- | --- | --- | --- | --- |
| 0 | Udvega | Bad | Sun | inauspicious |
| 1 | Chara | Neutral | Venus | neutral |
| 2 | Labha | Gain | Mercury | auspicious |
| 3 | Amrita | Best | Moon | auspicious |
| 4 | Kala | Loss | Saturn | inauspicious |
| 5 | Shubha | Good | Jupiter | auspicious |
| 6 | Roga | Evil | Mars | inauspicious |

The day's first part takes the kind ruled by the weekday's own planet.
Each later part takes the next kind in the cycle, so the eighth part
repeats the first. The night's first part is set by the weekday too, and
each later part of the night is five places further round the cycle. A
night takes the weekday of the sunset that begins it.

| Weekday | The day's parts | The night's parts |
| --- | --- | --- |
| Sunday | Udvega, Chara, Labha, Amrita, Kala, Shubha, Roga, Udvega | Shubha, Amrita, Chara, Roga, Kala, Labha, Udvega, Shubha |
| Monday | Amrita, Kala, Shubha, Roga, Udvega, Chara, Labha, Amrita | Chara, Roga, Kala, Labha, Udvega, Shubha, Amrita, Chara |
| Tuesday | Roga, Udvega, Chara, Labha, Amrita, Kala, Shubha, Roga | Kala, Labha, Udvega, Shubha, Amrita, Chara, Roga, Kala |
| Wednesday | Labha, Amrita, Kala, Shubha, Roga, Udvega, Chara, Labha | Udvega, Shubha, Amrita, Chara, Roga, Kala, Labha, Udvega |
| Thursday | Shubha, Roga, Udvega, Chara, Labha, Amrita, Kala, Shubha | Amrita, Chara, Roga, Kala, Labha, Udvega, Shubha, Amrita |
| Friday | Chara, Labha, Amrita, Kala, Shubha, Roga, Udvega, Chara | Roga, Kala, Labha, Udvega, Shubha, Amrita, Chara, Roga |
| Saturday | Kala, Shubha, Roga, Udvega, Chara, Labha, Amrita, Kala | Labha, Udvega, Shubha, Amrita, Chara, Roga, Kala, Labha |

Both columns are Drik Panchang's pages for New Delhi of 1 to 7 January
2025, one page for each weekday [drik-choghadiya-2025]. Wikipedia gives
the division but not the sequences.

**Worked example.** At New Delhi on Wednesday 1 January 2025, Drik
Panchang gives sunrise at 07:14 IST and sunset at 17:36: 622 minutes of
daylight, so a part is 77.75 minutes. The day begins with Labha, ruled by
Mercury, Wednesday's planet: 07:14 to 08:31.75, printed as 07:14 to 08:32.
The fifth part is four places on, Roga, from 07:14 + 4 × 77.75 min =
12:25. The next sunrise is at 07:14 again, so the night runs 818 minutes
and a part is 102.25 minutes. The night begins with Udvega, 17:36 to
19:18.25, and its second part is five places on, Shubha, printed from
19:18 [drik-choghadiya-2025].

## What is carried

- `Choghadiya`, the seven kinds with Drik Panchang's English names and
  glosses, the quality and the ruler: `Choghadiya::CYCLE`, identifiers
  `udvega`, `chara`, `labha`, `amrita`, `kala`, `shubha` and `roga`.
- `DAY_FIRST` and `NIGHT_FIRST`, the first kind of the day and of the
  night by weekday, and `DAY_STEP` and `NIGHT_STEP`, one place and five.
  `kind_of` gives the kind of any part.
- `day` and `night`: the eight parts of a local day's daylight, and of the
  night after it, at a place, in Universal Time. Where the Sun does not
  rise or set they return `MissingSolarEvent`.

Not carried: a fixed-clock form of the day, like the 06:00 to 18:00 day
of Rāhu kālam, since no source read gives one for the choghadiya.

## Accuracy

The division is exact, and the times are as good as the sunrise and sunset
beneath them. Against Drik Panchang's New Delhi pages of 1 to 7 January
2025, all 112 kinds agree, and every one of the 112 starts is within 0.59
minutes of the printed minute [drik-choghadiya-2025].

## Sources

- [wikipedia-choghadiya]: the division of the day and the night into
  eight, and the three qualities. Read 2026-09-28. It cites Chitkara,
  *Encyclopaedia of Buddhism* (2005), not read.
- [astromedha-choghadiya]: the ruler of each kind and the qualities. Read
  2026-09-28. It does not give the sequences.
- [drik-choghadiya-2025]: the pages for New Delhi of 1 to 7 January 2025,
  with sunrise, sunset and every part's kind and times. The two sequence
  columns above are these pages. Read 2026-09-28.

## Code

`crates/hc-calendars-indic/src/choghadiya.rs`, which shares the division
into eighths with `kalam`. The tests that anchor it:
`a_january_week_takes_the_kinds_and_times_drik_panchang_prints`,
`the_eighth_part_repeats_the_first_and_each_half_holds_all_seven`,
`the_day_takes_the_kinds_in_the_order_of_their_rulers`,
`a_night_is_an_eighth_of_sunset_to_sunrise` and
`there_is_no_choghadiya_where_the_sun_does_not_set`.
