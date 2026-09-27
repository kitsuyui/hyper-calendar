# Panchak

Backs `hc-calendars-indic::panchak`. No calendar identifier is registered:
a Panchak is a window of time, not a calendar. The two naming tables are
`PanchakNaming` entries, `panchak-five-kinds` and `panchak-raj-midweek`
([policy.md](../policy.md) §5).

## What it is

Panchak, "the five", is the time the Moon takes to pass through the last
five nakṣatras: the second half of Dhaniṣṭhā, then Śatabhiṣā, Pūrva
Bhādrapadā, Uttara Bhādrapadā and Revatī. Almanacs mark it as a time to
avoid some undertakings, and name each window by the weekday it begins
on [prokerala-panchak, nakshatrica-panchak, indiatv-panchak-2025].

## How it works

A nakṣatra is 13°20′ of the sidereal ecliptic, and a quarter of one is
3°20′. Dhaniṣṭhā, the 23rd, runs from 293°20′ to 306°40′, so its third
quarter begins at 300°, which is also the start of the sign Kumbha. The
window opens when the Moon's sidereal longitude reaches 300° and closes
when it reaches 360°, the end of Revatī and of the sign Mīna: the Moon's
passage through Kumbha and Mīna [nakshatrica-panchak]. The Moon covers
the 60° in four to five days, once a sidereal month, so a year has
thirteen or fourteen windows. The longitude depends on the ayanamsa;
Nakshatrica uses Lahiri's [nakshatrica-panchak].

The kind of a window is set by the weekday it begins on. The sources agree
on five weekdays and part on two:

| Weekday | `panchak-five-kinds` | `panchak-raj-midweek` |
| --- | --- | --- |
| Sunday | Rog | Rog |
| Monday | Raj | Raj |
| Tuesday | Agni | Agni |
| Wednesday | none | Raj |
| Thursday | none | Raj |
| Friday | Chor | Chor |
| Saturday | Mrityu | Mrityu |

Prokerala names the five kinds and no others, and spells Monday's
"Rajya" [prokerala-panchak]. Nakshatrica calls the Wednesday and
Thursday windows "Neutral" [nakshatrica-panchak]. India TV calls a window
beginning on Monday, Wednesday or Thursday Raj Panchak
[indiatv-panchak-2025].

**Worked example.** Drik Panchang's first window of 2025 for New Delhi
opens on Friday 3 January at 10:47 IST and closes on Tuesday 7 January at
17:50 [drik-panchak]: the Moon travels 60° in 4 days 7 hours, about
14° a day. It opens on a Friday, so both tables call it Chor Panchak.
The last window of 2025 opens on Wednesday 24 December at 19:46: India TV
calls it Raj Panchak [indiatv-panchak-2025], and the five-kind table names
no kind.

## What is carried

- `is_panchak` and `window`: whether the Moon is in the arc at a moment,
  and the window in progress or the next one, as the moments the Moon
  reaches 300° and 360°, in Universal Time, with the ayanamsa as a
  parameter.
- `PanchakNaming::FIVE_KINDS` and `PanchakNaming::RAJ_MIDWEEK`, with
  `kind`, the kind for a weekday.

The weekday is the caller's to take. The sources say "the weekday it
begins on" and do not say whether the day is counted from midnight or from
sunrise, which matters for a window that opens between the two, as the
window of 23 April 2025 does at 00:31 [drik-panchak].

Not carried: which activities each kind forbids, which is advice rather
than a rule of time.

## Accuracy

The arc is exact, and the times are as good as the Moon's longitude and
the ayanamsa. Against Drik Panchang's fourteen windows of 2025 for New
Delhi, every opening and closing is from 6 seconds before the printed
minute to 52 seconds after it, and its window of 21 to 25 October 2026 is
within a minute at both ends [drik-panchak].

Prokerala's windows of September to November 2026 close within a minute of
this module's and open 4.9 to 5.2 minutes before them, nearly 3′ of the
Moon's travel short of 300° [prokerala-panchak]. The page does not say
why, and Drik Panchang's October window opens at 07:00, with this module,
where Prokerala's opens at 06:55.

## Sources

- [nakshatrica-panchak]: the arc from 300° to 360° with the Lahiri
  ayanamsa, the weekday table with Wednesday and Thursday "Neutral". Read
  2026-09-28.
- [prokerala-panchak]: the five kinds by weekday, and three windows of
  2026 for Ujjain. Read 2026-09-28.
- [indiatv-panchak-2025]: Raj Panchak for a window beginning on Monday,
  Wednesday or Thursday, and the window of 24 to 29 December 2025. Read
  2026-09-28.
- [drik-panchak]: the windows of 2025 and 2026 for New Delhi. Read
  2026-09-28. The page does not state the rule.

## Code

`crates/hc-calendars-indic/src/panchak.rs`. The tests that anchor it:
`the_windows_of_2025_open_and_close_when_drik_panchang_says`,
`prokerala_closes_the_autumn_windows_of_2026_with_drik_panchang_but_opens_them_earlier`,
`the_two_tables_part_on_wednesday_and_thursday_only` and
`a_window_holds_the_moon_in_the_last_sixty_degrees`.
