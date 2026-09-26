# Astronomical Easter at the meridian of Jerusalem

Backs the `Computus` entry `astronomical-jerusalem` in `hc-holiday`.

## What it is

In March 1997 a consultation of the World Council of Churches and the
Middle East Council of Churches at Aleppo proposed that the churches keep
one Easter by returning to the norm of Nicaea and computing it from the
sky: "to maintain the Nicene norms (that Easter should fall on the Sunday
following the first vernal full moon)", "to calculate the astronomical
data (the vernal equinox and the full moon) by the most accurate possible
scientific means", "using as the basis for reckoning the meridian of
Jerusalem, the place of Christ's death and resurrection"
[wcc-aleppo-1997, § 11]. The statement appends a table of the Easters
the rule gives for 2001–2025, beside the Gregorian and Julian ones, the
astronomical vernal full moon and the Jewish Passover [§ 15]. It hoped the
churches would study the proposal before 2001, the year in which their two
reckonings coincided [§§ 16–18]. None adopted it, and no church keeps
Easter by it. It is carried because it is the reckoning both
ecclesiastical ones approximate, so the two can be measured against it.

## How it works

1. **The equinox.** The instant the Sun reaches ecliptic longitude 0° in
   the Gregorian year.
2. **The paschal full moon.** The first instant at or after the equinox
   at which the Moon's elongation from the Sun is 180°. The two *instants*
   are compared, not their dates.
3. **The day.** That instant's date in apparent solar time at Jerusalem:
   Universal Time plus the longitude, 35.24° east, as a fraction of a day,
   plus the equation of time.
4. **Easter.** The first Sunday strictly after that day.

The statement gives the norm, the meridian and nothing finer. Two
questions it leaves open are settled here as follows.

- **A full moon on a Sunday.** "The Sunday following the first vernal full
  moon" could be read to include that Sunday. The table settles it: the
  full moon of 2001 is 8 April, a Sunday, and Easter is 15 April. Reingold
  and Dershowitz's `astronomical-easter` takes the Sunday after too
  (`kday-after`) [reingold2018code]. It is the only year of the 25 in the
  table in which the full moon falls on a Sunday.
- **Which time at Jerusalem.** The statement says "meridian", which is a
  longitude, not a clock. Reingold and Dershowitz date the full moon in
  apparent (sundial) time at their `jerusalem` location, 31.78° N,
  35.24° E [reingold2018code], and this library follows them. Local mean
  time at the same longitude gives the same Easter in every year from 1583
  to 2150; Israel Standard Time, UTC+2, differs in 1653, 1775 and 1873.

**Worked example: 2019.** `hc-seasons` puts the March equinox on 21 March
in apparent time at Jerusalem, and `hc-astro` the next full moon about
four hours later, still 21 March, a Thursday. The first Sunday after is
24 March. The Gregorian computus gives 21 April, a month later, because
its own paschal moon for that year is the one after its fixed equinox of
21 March. The table has 24 March and 21 April.

## What is carried

- **Identifier** `astronomical-jerusalem`, as a `Computus` entry beside
  `gregorian` and `julian`, so any `Rule::EasterRelative` can be keyed to
  it. No tradition table uses it, since no church keeps it.
- **Functions** `astronomical_paschal_full_moon`, the day of the full moon
  at Jerusalem, and `astronomical_easter`.
- **Range** Gregorian years 1583 to 2150. The statement puts no bound on
  the rule. 1583 is where this library's Gregorian computus starts, so
  that the two can be compared in every year either answers; an
  astronomical Easter before 1997 is the rule applied backwards. 2150 is
  where `hc-astro`'s ΔT leaves Espenak and Meeus's fitted segments for a
  parabola whose error grows by hours, enough to move a full moon near
  midnight to the other day.
- **Not carried.** The statement's Passover column, which is the Hebrew
  calendar's 15 Nisan and is `hebrew::passover`; a fixed Sunday, which
  the statement sets aside [§ 12 (v)].

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| The astronomical Easter of every year 2001–2025 in the table [wcc-aleppo-1997] | `the_astronomical_reckoning_reproduces_the_aleppo_table` | 25 of 25 |
| The vernal full moon of every year 2001–2025 in the table | the same | 25 of 25 |
| 8 April 2001 is a Sunday and Easter a week later | `a_full_moon_on_a_sunday_puts_easter_a_week_later` | yes |
| The table's Gregorian column equals its astronomical one except in 2019 | `the_table_agrees_with_the_gregorian_computus_except_in_2019` | yes |
| The equinox and the full moon of 2019 fall on 21 March at Jerusalem, a few hours apart | `the_equinox_and_the_full_moon_of_2019_share_a_day_at_jerusalem` | yes |

Over 1583–2150 the astronomical Easter equals the Gregorian in 516 of the
568 years and the Julian in 202, measured with this implementation.

The table's Julian column gives 20 May for 2025; the Julian computus gives
20 April, the date the Orthodox churches kept, so the table's entry is a
misprint. It is not used.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [wcc-aleppo-1997] | The proposal, the meridian, the table of 2001–2025 | Yes, oikoumene.org, 2026-09-26 |
| [reingold2018code] | `astronomical-easter`, `apparent-from-universal`, the `jerusalem` location | Yes, `calendar.l`, 2026-09-26 |

## Code

`crates/hc-holiday/src/computus.rs`: `Computus::ASTRONOMICAL_JERUSALEM`,
`astronomical_paschal_full_moon`, `astronomical_easter`,
`ASTRONOMICAL_EASTER_FIRST_YEAR`, `ASTRONOMICAL_EASTER_LAST_YEAR`,
`JERUSALEM_LONGITUDE_DEGREES`. Anchors:
`the_astronomical_reckoning_reproduces_the_aleppo_table`,
`a_full_moon_on_a_sunday_puts_easter_a_week_later`.
