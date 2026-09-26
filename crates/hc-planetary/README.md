# hc-planetary

Time on bodies other than Earth: the Martian sol and everything built on it,
a data table of the solar system's major bodies, the Moon's lunation counts
and selenographic colongitude, and one generic clock over all of them.

Every entry point takes an `hc_core::Instant<Tai>`. TAI is the only scale with
no rotational content, which is the honest place to stand when the rotation you
care about is not the Earth's.

```rust
use hc_planetary::{BodyClock, mars};

let landing = mars::missions::mission("Curiosity").unwrap();
let moment = landing.landing_moment().unwrap();
assert_eq!(landing.clock().unwrap().sol(moment), 0);   // sol 0 at Gale Crater
assert_eq!(moment.mars_year(), 31);                    // Mars Year 31

let titan = BodyClock::for_name("Titan").unwrap();
assert!(!titan.is_standardised());                     // no standard exists
```

## What it covers

- **`mars`** — `MARS_SOL_SECONDS`, the Mars Sol Date, Coordinated Mars Time,
  local mean and true solar time with the full Martian equation of time,
  areocentric solar longitude `Ls`, Mars years under the Clancy convention,
  the sol counts of all ten surface missions, and the Darian calendar.
- **`bodies`** — 22 bodies with sidereal rotation, synodic day, orbital period,
  axial tilt and semi-major axis, and the solar day / year-in-local-days
  derived from them.
- **`moon`** — Meeus and Brown lunation numbers, the age of the Moon, the
  selenographic position of the Sun and the colongitude.
- **`clock`** — `BodyClock`, one interface from a TAI instant to a local time
  on any body in the table.

## What it deliberately does not do

- **No Coordinated Lunar Time.** As of 2026-09-26 no LTC scale was defined:
  the OSTP memorandum of 2 April 2024 asks NASA for a strategy by 31 December
  2026, and IAU 2024 Resolution III calls for one by international agreement
  (Resolution II defines Lunar Coordinate Time, TCL, a relativistic coordinate
  time, not a civil scale). The crate says so and provides no substitute. `moon::mean_solar_time`
  gives a *mean solar* clock for the Moon under a zero point this crate
  declares and labels as declared; it is not LTC.
- **No relativity.** Seen from the Moon, an Earth clock loses on average 58.7 µs
  per Earth day (the OSTP memorandum). That is `hc-relativity`'s subject.
- **No ephemeris.** Mars's orbit is the only planetary orbit modelled. The
  `bodies` table is data, not a propagator.
- **No invented standards.** Where no zero point has been agreed,
  `EpochBasis::Convention` marks the one this crate states, and only Earth and
  Mars are `EpochBasis::Standard`.
- **MER hybrid local solar time.** Spirit's and Opportunity's operational
  clocks were offset from site LMST by >41 and >37 minutes. The crate
  reproduces their sol *numbers*, not those offsets.

## Accuracy claimed

| quantity | claim |
|---|---|
| Mars `Ls`, MSD, MTC, LMST, LTST | max 0.008° of `Ls`, ≈3 s of true solar time, over ±100 yr of J2000 (Allison & McEwen's own figure). Both published GISS worked examples reproduce to the last published digit. |
| Mars years | boundaries solved from the same series; checked against eight published Mars-year start dates, 1998–2024, each landing inside the right Gregorian day. |
| Mission sol numbers | ten published sol↔Earth-date anchors, all correct. |
| Darian calendar | exact arithmetic; mean year 668.592 sols vs. the tropical 668.5921, ≈1 sol per 10 000 Mars years. |
| Moon | `hc-astro`'s series: ~10″ in longitude. Meeus example 53.a reproduces to 0.1° in colongitude. Conjunction instants can run ~20 min from published values. |
| `bodies` derivations | only as good as fact sheets quoted to 4–7 significant figures — parts in 10⁵ on a rotation period. Fine for "how long is a day on Titan", useless for propagating a day count over a century. |

## Sources, constant by constant

### Mars (all from NASA GISS, *Mars24 Sunclock — Algorithm and Worked Examples* and its *Technical Notes*, retrieved 2026-09-26, restating Allison 1997 *GRL* 24:1967 and Allison & McEwen 2000 *Planet. Space Sci.* 48:215, which were not read; the system is written up in [`docs/systems/mars-timekeeping.md`](../../docs/systems/mars-timekeeping.md))

| constant | value | note |
|---|---|---|
| `MARS_SOL_SECONDS` | 88 775.244 s | mean solar day, 24h39m35.244s |
| `MARS_SIDEREAL_DAY_SECONDS` | 88 642.663 s | 24h37m22.663s |
| `SOL_IN_DAYS` | 1.027 491 251 7 | Mars24 eq. C-2. `88775.244/86400` is 1.0274912500 exactly; the crate carries the published ratio and documents the 1.5 × 10⁻⁷ s/sol gap |
| `MSD_EPOCH_JULIAN_DATE_TT` / `MSD_AT_EPOCH` | JD 2 451 549.5 TT / 44 796.0 | MSD 0 falls on 1873-12-29 |
| `MSD_MIDNIGHT_ADJUSTMENT` | 0.000 962 6 | Mars24's revised value (its eq. C-2, revised 2015); carrying it rather than the 0.000 72 of 2000 is this library's choice, so that MSD and MTC match Mars24 |
| `MSD_MIDNIGHT_ADJUSTMENT_2000` | 0.000 72 | as first published in AM2000; the two differ by 21.5 Martian seconds |
| mean anomaly / fictitious mean Sun | 19.3871 + 0.52402073 Δt / 270.3871 + 0.524038496 Δt | eqs. B-1, B-2 |
| 7 perturbation terms | see `PERTURBERS` | eq. B-3 |
| equation of centre | 10.691 + 3.0e-7 Δt, 0.623, 0.050, 0.005, 0.0005 | eq. B-4 |
| equation of time | 2.861, −0.071, 0.002, −(ν−M) | eq. C-1; range −51.1 min at `Ls`≈329° to +39.9 min at `Ls`≈188°, against Earth's −14.2/+16.3 |
| `MARS_TROPICAL_YEAR_SOLS` / `_DAYS` | 668.5921 / 686.9725 | technical notes |
| `MARS_SIDEREAL_YEAR_SOLS` / `_DAYS` | 668.5991 / 686.9797 | technical notes |
| `MARS_YEAR_1_START_MSD` | 28 892.6593 | **a seed, not a citation.** Clancy et al. 2000 *JGR* 105(E4):9553 give the date 1955-04-11 and no time. Published times of day disagree; this model, a DE430 fit and Piqueux et al. 2015 *Icarus* 251:332 all cluster near 11:00 UTC, while the widely quoted 08:31 UTC does not reproduce. The constant is this crate's own `Ls = 0` solution and is re-solved at run time |
| mission landing times, clock meridians, sol-0/sol-1 conventions | Mars24 *Lander Mission Times*; Viking 1's site longitude via Kuchynka et al. 2014, which defines the prime meridian | landing instants are SCET where documented; several NASA-quoted times are Earth-received time, 8–13 min later |
| Darian months, weekdays, leap rule, epoch | Gangale, "The Darian Calendar for Mars", <https://ops-alaska.com/time/gangale_mst/darian.htm>, retrieved 2026-09-26; his SAE 2006-01-2249 was not read | leap rule `(Y−1)\2 + Y\10 − Y\100 + Y\500`, which the page calls the intercalation formula. The same page's "extended intercalation scheme", `\1000` (668.5910 sols) and a series of later formulas, is offered there as an example against the vernal-equinox year and is not carried |
| `DARIAN_EPOCH_MARS_SOL_DATE` | −94 129 | reproduces the published Darian dates of the Viking 1 and Perseverance landings (14 Mina 195, 13 Sagittarius 219). Gangale's continuous "Mars Julian Sol" is noon-based and reads 94 128.511 at the MSD epoch; this crate aligns the sol boundary with Airy midnight |

### Bodies (NASA NSSDC *Planetary Fact Sheets* unless noted)

Rotation, orbit, tilt and semi-major axis for the Sun, Mercury, Venus, Earth,
Moon, Mars, Phobos, Deimos, Ceres, Jupiter, Io, Europa, Ganymede, Callisto,
Saturn, Enceladus, Titan, Uranus, Neptune, Triton, Pluto and Charon. Satellite
rotation rates cross-checked against the IAU WGCCRE (Archinal et al. 2018,
*CeMDA* 130:22); Ceres from Konopliv et al. 2018, *Icarus* 299:411 and the JPL
Small-Body Database.

Contested values, carried as they stand with the conflict in `Body::source`:

- **Mercury** — the crate uses the IAU rate, 1407.5088 h. NSSDC prints 1407.6 h
  but derives its own 4222.6 h solar day from the IAU value; using 1407.6 would
  break the 3:2 resonance by half an hour.
- **Venus** — −5832.5 h from the NSSDC comparison table; the Venus sheet itself
  says −5832.6 h and the IAU rate gives −5832.444 h.
- **Neptune** — 16.11 h, the IAU 2009 System III value that NSSDC and JPL both
  publish. The IAU 2015 report adopts 15.9663 h after Karkoschka 2011. These
  genuinely conflict.
- **The Sun** — 609.12 h is the Carrington rate at 16° latitude, not the
  equator; the Sun rotates differentially and has no single period.
- **Pluto** — signed negative here because its 119.51° tilt makes the rotation
  retrograde in the ecliptic sense; the IAU calls the same rotation prograde
  about its own defined pole.

The solar day is derived as `1/P_solar = 1/P_sidereal − 1/P_orbit`, with the
orbital period taken **around the Sun** (a moon inherits its planet's year) and
the rotation period **signed**. Three bodies carry a measured solar day instead
of the derivation, because for them the measurement is the primary datum:
Earth (86 400 s by definition), Mars (the Mars24 sol) and the Moon (the mean
synodic month).

### Moon

- Meeus, *Astronomical Algorithms*, 2nd ed., ch. 47 (arguments), ch. 49
  (lunation 0 = the new moon of 2000-01-06) and ch. 53 (selenographic position
  of the Sun; `LUNAR_EQUATOR_INCLINATION` = 1.54242°).
- `BROWN_MINUS_MEEUS_LUNATION` = 953: Brown lunation 1 begins at the new moon of
  1923-01-17.
- Coordinated Lunar Time: US OSTP memorandum, April 2024, directing NASA to
  deliver a standard by the end of 2026. No definition was published at the
  time of writing.

## `hc_core::epoch::J2000`

This crate takes J2000 straight from `hc_core::epoch::J2000`. `src/util.rs`
binds it to a local name, and a test re-derives it from the leap-second table
and asserts that the two agree.

The value is easy to get wrong. Its TAI reading is 946 727 967.816 s;
applying the 32.184 s TT-to-TAI offset a second time gives 946 727 935.632 s.
That error puts MTC half a Martian minute out and breaks both published GISS
worked examples, which is why the agreement is a test.
