# Relativistic time: rate offsets, dilated clocks, worldlines and the relativistic rocket

Backs `hc-relativity`: its `constants`, `special`, `gravitational`,
`worldline` and `dilated` modules, and the private `hyperbolic` module they
share. Backs also the crate's `relativity` layer in the facade,
`hyper_calendar::relativity_lines` (`proper_time_line`,
`gravitational_dilation_line`, `gravitating_bodies_lines`), which is the
`hc_proper_time`, `hc_gravitational_dilation` and `hc_gravitating_bodies`
exports of the WebAssembly module and the C library and the JavaScript
`properTime`, `gravitationalDilation` and `gravitatingBodies`, behind the
`relativity` feature. No calendar identifier is registered: these are rates
of clocks, not calendars. The body identifiers the exports take are
`sun`, `earth`, `moon`, `mars`, `jupiter` and `sagittarius-a-star`.
[time-scales.md](../time-scales.md) places TT, TCG and TCB, which
`hc-core::scale` carries, and [off-earth.md](../off-earth.md) lists what the
crate does.

## What it is

Two clocks do not keep one rate. A clock's rate against a coordinate time
depends on how fast it moves and on the gravitational potential it sits in,
and the differences are large enough to engineer around. Ashby's review of
relativity in the Global Positioning System puts it so: velocities and
fields are small near the Earth, yet the first- and second-order Doppler
shifts, the gravitational shifts and the Sagnac effect "give rise to
significant relativistic effects" [ashby2003]. The satellite clocks are
therefore set low in frequency on the ground before launch, to
[1 − 4.4647 × 10⁻¹⁰] × 10.23 MHz = 10.229 999 995 43 MHz [ashby2003,
eqs 35 and 36].

The same physics is in the definitions of the relativistic time scales:

- **TT and TCG.** TT differs from Geocentric Coordinate Time by a constant
  rate, dTT/dTCG = 1 − *L*G, where *L*G = 6.969 290 134 × 10⁻¹⁰ is a
  defining constant (IAU 2000 Resolution B1.9). The IAU's 1991 Resolution A4
  had defined *L*G as *U*G/*c*², *U*G being the geopotential at the geoid;
  the 2000 resolution cut it loose from the geoid [soffel2003, App. A]
  [klioner2010].
- **TDB and TCB.** TDB = (1 − *L*B) TCB + TDB₀, with *L*B =
  1.550 519 768 × 10⁻⁸ and TDB₀ defining constants (IAU 2006 Resolution 3)
  [klioner2010]. Before that, *L*B was derived: the mean rate of TCG against
  TCB is 1 − *L*C, with *L*C = 1.480 826 867 41 × 10⁻⁸ ± 2 × 10⁻¹⁷ from
  Irwin and Fukushima (1999, not read), and 1 − *L*B = (1 − *L*C)(1 − *L*G),
  which gave 1.550 519 767 72 × 10⁻⁸ [soffel2003, Resolution B1.5, note 3].

A ship that holds a steady proper acceleration is a third case, and the
reason a calendar library meets relativity at all: a clock carried to
another star and one left at home disagree by years. Constant *proper*
acceleration, the acceleration the crew feel, makes rapidity grow linearly
in the ship's own time, so every quantity of the journey has a closed form
[baez-acceleration] [baez-rocket].

## How it works

**Special relativity.** The Lorentz factor is γ = 1/√(1 − β²) with
β = *v*/*c*, and collinear velocities compose as (*u* + *v*)/(1 + *uv*/*c*²),
so that 0.9*c* and 0.9*c* make 0.99448*c* and the sum never reaches *c*
[baez-velocities]. A clock moving at β records dτ = d*t*/γ, *t* being the
coordinate time of the frame it moves through. Rapidity *w* = artanh β turns
the composition law into addition; γ = cosh *w* and β = tanh *w*
[baez-acceleration] [baez-rocket]. The relativistic Doppler factor and
the aberration formula are textbook; no source for them was read for this
document, and the crate anchors them by identities (the transverse factor is
1/γ, the two longitudinal factors are reciprocal, a boost and its opposite
undo each other) rather than by a published value. The transverse Doppler
shift as a confirmation of time dilation is the experiment of Ives and
Stilwell (1938), whose paper was not read; its entry in a list of
experiments was [roberts-sr-experiments].

**Gravity.** A clock held still at radius *r* from a non-rotating spherical
mass runs at dτ/d*t* = √(1 − *r*s/*r*) against a clock at infinity,
*r*s = 2*GM*/*c*² the Schwarzschild radius. The crate takes the standard
gravitational parameter *GM*, never a mass, because *G* is known to only
2.2 × 10⁻⁵ [codata2022] while *GM* of the Earth is quoted to ten figures
[ashby2003]. The exact Schwarzschild expressions of this paragraph and the
next are textbook and no source for them was read. What was read is the
weak-field form Ashby uses, which keeps terms of first order in *V*/*c*² and
so adds a clock's gravitational and velocity terms: dτ = [1 + (*V* − Φ₀)/*c*²
− *v*²/2*c*²] d*t*, with *V* = −*GM*/*r* the Newtonian potential and Φ₀ the
reference potential that sets the unit of coordinate time, zero for a rate
against a clock at infinity and the geoid's value for TT [ashby2003,
eqs 12 and 27].

**An orbit against the ground.** For a circular orbit at radius *r* the
crate has two forms:

- Exact, in Schwarzschild: the orbiting clock runs at √(1 − 3*GM*/*rc*²), the
  extra *GM*/*rc*² of the 3 against the 2 of a static clock being the orbital
  motion; the ground clock at √(1 − *r*s/*r*g); the offset is their ratio
  minus 1.
- Weak-field: the sum of a gravitational term *GM*/*c*² (1/*r*g − 1/*r*) and
  a kinematic term −*v*²/2*c*² = −*GM*/2*rc*², with *v*² = *GM*/*r*. Ashby's
  eq. 35 writes the orbit's share as 3*GM*/2*ac*², which is *GM*/*ac*² +
  *GM*/2*ac*², the same two terms against infinity [ashby2003].

A fractional offset times 86 400 s times 10⁶ is microseconds a day.

**Worldlines.** A worldline is a list of segments, each with a coordinate
duration, a velocity profile and an optional static potential. A constant
segment gives dτ = d*t*/γ, and the integral is a multiplication. A segment
of constant proper acceleration *a* has *w* = *w*₀ + *aτ*/*c*, and the
relations of the relativistic rocket from rest are [baez-rocket]:

| Quantity | Relation |
| --- | --- |
| velocity | *v* = *c* tanh(*aT*/*c*) = *at* / √(1 + (*at*/*c*)²) |
| distance | *d* = (*c*²/*a*) (cosh(*aT*/*c*) − 1) = (*c*²/*a*) (√(1 + (*at*/*c*)²) − 1) |
| coordinate time | *t* = (*c*/*a*) sinh(*aT*/*c*) = √((*d*/*c*)² + 2*d*/*a*) |
| proper time | *T* = (*c*/*a*) arsinh(*at*/*c*) = (*c*/*a*) arcosh(*ad*/*c*² + 1) |
| Lorentz factor | γ = cosh(*aT*/*c*) = √(1 + (*at*/*c*)²) = *ad*/*c*² + 1 |

*T* is the crew's proper time, *t* the time in the frame they left, *d*
the distance in that frame. To arrive at rest the ship turns over at the
midpoint and decelerates; the same equations hold if one can "run the film
backwards", and *T* = (2*c*/*a*) arcosh(*ad*/2*c*² + 1) [baez-rocket]. A
segment's optional potential multiplies its kinematic factor by the static
factor √(1 − *r*s/*r*); that product is the weak-field composition of the two
effects. A `ClockComparison` attaches a worldline's two durations to a TAI
instant, so that the ship's clock, set to TAI at departure, and the home
clock can be printed side by side; their difference is the lag. When the
cruise speed carries an error bar σβ, the proper time of a constant leg
*t*√(1 − β²) has σ = *t* |β| σβ / √(1 − β²), propagated analytically
because β appears twice.

**Worked example.** *A rocket at 1 g for one year of ship time*, with the
crate's constants: *a* = 9.806 65 m s⁻², one year = 31 557 600 s (the Julian
year), *c* = 299 792 458 m s⁻¹.

1. *c*/*a* = 30 570 323 s = 0.968 715 yr, and the same number is *c*²/*a*
   in light years, 0.968 715 ly.
2. The rapidity is *w* = *aT*/*c* = 1/0.968 715 = 1.032 295.
3. *e*^*w* = 2.807 502 and *e*^−*w* = 0.356 188, so sinh *w* = 1.225 657,
   cosh *w* = 1.581 845 and tanh *w* = 0.774 827.
4. Coordinate time *t* = 0.968 715 × 1.225 657 = 1.187 312 yr; distance
   *d* = 0.968 715 × (1.581 845 − 1) = 0.563 642 ly; speed 0.7748*c*;
   γ = 1.5818. The crate returns the same to the digits shown.
5. The table on [baez-rocket]'s page gives 1.19 years, 0.56 ly, 0.77*c*
   and γ = 1.58 for *T* = 1 year.

The same page's flip-and-burn to the nearest star, 4.3 ly: each half is
2.15 ly, so *ad*/*c*² = 2.15/0.968 715 = 2.219 435, γ at the midpoint is
3.219 435 (β = 0.9505), arcosh(3.219 435) = ln(3.219 435 + 3.060 190) =
1.837 310, and *T* = 2 × 0.968 715 × 1.837 310 = 3.5597 yr on board, against
*t* = 2 √(2.15² + 2 × 2.15 × 0.968 715) = 5.9289 yr at home. The page prints
3.6 years.

**Worked example, the GPS clock.** *GM* = 3.986 004 418 × 10¹⁴ m³ s⁻²,
the ground clock at rest at the equatorial radius 6 378 137 m, the satellite
at 26 561 750 m, *c*² = 8.987 551 787 × 10¹⁶ m² s⁻².

1. *GM*/*c*² = 4.435 028 mm.
2. Ground term *GM*/(*c*² *r*g) = 4.435 028 × 10⁻³/6 378 137 =
   6.953 485 × 10⁻¹⁰.
3. Orbit term *GM*/(*c*² *r*) = 1.669 705 × 10⁻¹⁰; the kinematic term is
   half of it, −8.348 524 × 10⁻¹¹ (the speed is √(*GM*/*r*) = 3 873.83 m/s).
4. Gravitational gain 6.953 485 − 1.669 705 = 5.283 780 × 10⁻¹⁰, which is
   45.6519 µs a day. Kinematic loss −7.2131 µs a day. Net
   4.448 928 × 10⁻¹⁰, which is +38.4387 µs a day.
5. Ashby's eq. 35 uses a ground clock on the geoid, with the effective
   potential Φ₀/*c*² = −6.9693 × 10⁻¹⁰, which *L*G fixes at
   6.969 290 134 × 10⁻¹⁰, against the 6.953 485 × 10⁻¹⁰ of step 2. Its net is
   2.5046 × 10⁻¹⁰ − 6.9693 × 10⁻¹⁰ = −4.4647 × 10⁻¹⁰ in his sign, which is
   +38.575 µs a day; the crate's number is 0.137 µs a day lower, for reasons
   Accuracy gives.

## What is carried

The crate builds without `std` (with `libm`) and has no `f32`; a speed of
*c*, a radius inside the horizon, a negative mass or an overflow is a named
`RelativityError`, not a `NaN`.

In `hc_relativity::constants`:

| Constant | Value | Origin |
| --- | --- | --- |
| `SPEED_OF_LIGHT`, `SPEED_OF_LIGHT_SQUARED` | 299 792 458 m/s | the revised SI's defining constant [bipm-si-defining-constants] |
| `GRAVITATIONAL_CONSTANT` | 6.674 30 × 10⁻¹¹ m³ kg⁻¹ s⁻², relative uncertainty 2.2 × 10⁻⁵ | CODATA 2022 [codata2022] |
| `STANDARD_GRAVITY` | 9.806 65 m s⁻², the "1 g" of a rocket | exact [nist-codata-gn] |
| `LIGHT_YEAR` | 9 460 730 472 580 800 m | *c* × the Julian year, exact |
| `ASTRONOMICAL_UNIT` | 149 597 870 700 m | IAU 2012 Resolution B2 [iau-2012-b2] |
| `GM_SUN` | 1.327 124 400 412 794 2 × 10²⁰ m³ s⁻² | JPL DE440, `BODY10_GM` [naif-gm-de440] [park2021] |
| `GM_SUN_NOMINAL_IAU_2015` | 1.327 124 4 × 10²⁰ m³ s⁻², a conversion constant | IAU 2015 Resolution B3 [prsa2016] |
| `GM_EARTH` | 3.986 004 418 × 10¹⁴ m³ s⁻² | IERS Conventions (2010) and WGS 84 [iers-tn36]; the value Ashby uses [ashby2003] |
| `GM_MOON` | 4.902 800 118 457 55 × 10¹² m³ s⁻² | DE440, `BODY301_GM` [naif-gm-de440] |
| `GM_MARS` | 4.282 837 × 10¹³ m³ s⁻², the Mars system | DE440, `BODY4_GM` to seven figures [naif-gm-de440] |
| `GM_JUPITER` | 1.267 127 64 × 10¹⁷ m³ s⁻², the Jupiter system | DE440, `BODY5_GM` [naif-gm-de440] |
| `SAGITTARIUS_A_STAR_SOLAR_MASSES`, `GM_SAGITTARIUS_A_STAR` | 4.297 × 10⁶ and that times `GM_SUN` | GRAVITY Collaboration 2022 [gravity2022] |
| `EARTH_EQUATORIAL_RADIUS` | 6 378 137 m | the WGS 84 semi-major axis; Ashby's *a*₁ [ashby2003] |
| `GPS_ORBIT_RADIUS` | 26 561 750 m | an orbit of half a sidereal day about `GM_EARTH`, by the crate's test |

`GravitatingBody` and the table `GRAVITATING_BODIES` hold the six `GM`
values with an identifier, an English name, the name of the constant and a
source string; `by_id` looks one up as `hc_core::catalogue::matches`
does. The table is open: a caller with another `GM` writes one down.

In `hc_relativity::special`: `lorentz_factor`, `lorentz_factor_from_speed`,
`lorentz_factor_from_rapidity`, `beta_from_lorentz`, `rapidity`,
`beta_from_rapidity`, `add_velocities`, `proper_time_of`,
`coordinate_time_of`, `doppler_factor`, `longitudinal_doppler`,
`transverse_doppler`, `aberrated_cosine` and `check_beta`. Every function
takes β, not metres per second, except `lorentz_factor_from_speed`.

In `hc_relativity::gravitational`: `schwarzschild_radius`,
`schwarzschild_radius_of_mass`, `static_dilation_factor`,
`gravitational_frequency_ratio`, `gravitational_redshift`,
`circular_orbit_speed` (Newtonian), `circular_orbit_dilation_factor`,
`orbit_rate_offset` (exact), `gravitational_rate_offset`,
`kinematic_rate_offset`, `weak_field_orbit_rate_offset` and
`rate_offset_to_micros_per_day`.

In `hc_relativity::worldline`: `GravitationalPotential`, `VelocityProfile`
(`Constant`, `ConstantProperAcceleration`), `Segment` (`constant`,
`accelerating`, `in_potential`, `proper_time`, `displacement`, `final_beta`),
`Worldline` (`coordinate_time`, `proper_time`, `elapsed_difference`,
`displacement`, `final_beta`), the rocket relations `rocket_beta`,
`rocket_distance`, `rocket_coordinate_time`, `rocket_proper_time`,
`rocket_proper_time_for_distance`, `rocket_coordinate_time_for_distance`,
and the flip-and-burn pair `flip_and_burn_proper_time` and
`flip_and_burn_coordinate_time`. Every segment is closed form: no step size,
no integration error.

In `hc_relativity::dilated`: `ClockComparison`, `compare_clocks`,
`dilated_instant`, `proper_time_uncertain` and `ship_reading_uncertain`,
over `hc_core::Instant<Tai>`, `hc_core::Duration` and
`hc_uncertainty::Uncertain`.

At the boundary, three exports, each one line of tab-separated text.
`hc_proper_time` takes a speed in m/s and a coordinate time in seconds and
writes β, γ, the proper seconds, the rate 1/γ, the rate's offset from 1 in
µs a day, the constant used and the source. `hc_gravitational_dilation`
takes a body identifier and a radius and writes the body, its `GM`, the
constant, the Schwarzschild radius, the static factor, the offset in µs a
day, the constants used and the source. `hc_gravitating_bodies` lists the
table. A rate such as 1 − 3 × 10⁻¹⁰ keeps only six figures of its distance
from 1 as a double, so both exports compute the offset without cancellation,
by √(1 − *x*) − 1 = −*x*/(1 + √(1 − *x*)), *x* being β² or *r*s/*r*. The
rocket, the worldlines and the orbit offset are not exported; a caller of the
exports gets the GPS gain as the difference of two `hc_gravitational_dilation`
offsets, 45.65 µs a day, and the loss from `hc_proper_time` at the circular
speed, −7.21.

Not carried:

- **A ground clock on the geoid.** The ground clock is at rest on a sphere
  of the equatorial radius. The geoid's potential, with the quadrupole and
  the Earth's rotation, is what Ashby and the definition of TT use. Not yet
  done; Ashby gives the terms [ashby2003, eq. 18], and Accuracy sizes them.
- **Eccentricity and the quadrupole at the satellite.** Ashby's periodic
  term for an eccentric orbit, Δ*t*r = 4.4428 × 10⁻¹⁰ *e* √*a* sin *E* s/√m
  [ashby2003, eq. 39], and the quadrupole's effect on the orbit are not
  modelled. Not yet done.
- **The Sagnac effect and rotation (Kerr).** No model of a rotating mass or a
  rotating ground station. Not yet done; Ashby treats the Sagnac effect for
  receivers [ashby2003].
- **The TCB − TCG transformation.** Resolution B1.5 gives it as an integral
  over the planetary ephemerides [soffel2003]. `hc-core::scale` carries the
  linear *L*G and *L*B parts; this crate uses none of them except *L*G in a
  test. Not yet done here.
- **A lunar clock against an Earth clock.** The crate has the Moon's `GM`
  and the static factor, but not the combination of the two potentials and
  the Moon's orbital motion that the 58.7 µs a day of the OSTP memorandum
  names [ostp-2024-celestial-time]. Not yet done; `hc-planetary` points here
  for it.
- **A rocket's fuel.** [baez-rocket] works out the fuel of a photon drive,
  *M*/*m* = γ(1 + *v*/*c*) − 1 = *e*^(*aT*/*c*) − 1 for one burn; no function
  here does. Not yet done.
- **Other velocity profiles, and an expanding universe.** Only a constant
  speed and a constant proper acceleration exist. [baez-rocket] says the
  rocket relations fail beyond about a thousand million light years, where
  the expansion of the universe matters; the functions do not refuse a
  longer distance. Not yet done.
- **Uncertainty through a rocket or a potential.** Only a constant leg's
  proper time carries an error bar. Not yet done.

## Accuracy

**Against published values.** The closed forms are exact for their models, so
the checks are anchors and the size of the model's limits.

| Check | Test | Result |
| --- | --- | --- |
| γ at β = 0.6 is 5/4, by hand | `the_lorentz_factor_at_six_tenths_of_c_is_exactly_five_quarters` | 10⁻¹⁵ |
| 100 s at 0.6*c* is 80 s aboard | `a_moving_clock_runs_slow_by_exactly_the_lorentz_factor` | 10⁻⁹ s |
| Out and back at 0.6*c* for two years is 584 days aboard, 146 lost | `the_twin_paradox_comes_out_at_eighty_per_cent`, `the_travelling_twin_always_falls_behind` | 10⁻⁶ day |
| Net GPS offset is +38.4 µs a day, in `gravitational` and rebuilt from worldline segments | `gps_satellites_gain_thirty_eight_point_four_microseconds_a_day_net`, `the_gps_satellite_reconstructed_as_a_worldline_gains_its_canonical_offset` | 0.1 and 0.2 µs a day |
| The two parts, +45.7 and −7.2 µs a day | `gps_satellites_gain_forty_five_point_seven_microseconds_a_day_gravitationally`, `gps_satellites_lose_seven_point_two_microseconds_a_day_kinematically` | 0.1 and 0.05 µs a day |
| The exact and weak-field offsets agree | `the_exact_and_weak_field_gps_figures_agree` | 10⁻⁷ asserted, 8.1 × 10⁻⁹ measured |
| The break-even orbit is 1.5 Earth radii | `the_break_even_orbit_is_at_one_and_a_half_earth_radii` | 10⁻²⁰ |
| An ISS orbit is −24.5 µs a day: +3.712 and −28.183 | `a_low_enough_orbit_loses_time_instead_of_gaining_it` | 0.005 µs a day; −24.3 against the geoid |
| Sun's Schwarzschild radius is 2 953.25 m | `the_schwarzschild_radius_of_the_sun_is_about_three_kilometres` | 1 m |
| 1 g for a year of ship time: β = 0.7748, 0.564 ly | `a_one_g_burn_reaches_three_quarters_of_c_in_a_year_of_ship_time`, `a_one_g_burn_covers_half_a_light_year_in_the_first_year` | 10⁻⁴ and 10⁻³ |
| 1 g flip-and-burn over 2.5 Mly is 28.6 years aboard, 2 500 002 at home | `a_one_g_flip_and_burn_reaches_andromeda_in_under_twenty_nine_years`, `the_same_voyage_takes_two_and_a_half_million_years_at_home` | 0.2 year |
| A short burn reduces to Newton | `a_short_burn_reduces_to_the_newtonian_answer` | 10⁻¹⁴ |
| A constant-speed leg of 100 s at β = 0.6 ± 0.01 has an error bar of 0.75 s | `an_uncertain_velocity_makes_the_proper_time_uncertain` | 10⁻⁹ |
| The facade's lines at 0.6*c*, and the GPS offsets | `six_tenths_of_c_is_five_quarters`, `a_gps_clock_gains_forty_five_microseconds_a_day_over_the_ground` in `relativity_lines.rs` | 10⁻³ µs a day; 45.65 ± 0.01 |

The crate's tests agree with its README's anchors. What no test does is
compare the rocket with a published table; that was done for this document
by running the crate. With *a* = 1.03 ly/yr² and *c* = 1 ly/yr, the units
[baez-rocket] states its table in, `rocket_coordinate_time`,
`rocket_distance`, `rocket_beta` and `lorentz_factor_from_rapidity` give
every printed digit of the table for *T* = 1, 2, 5, 8 and 12 years (for
12 years, *t* = 113 243, *d* = 113 242, γ = 116 641), and
`flip_and_burn_proper_time` gives 3.6, 6.6, 20 and 28 years for 4.3, 27,
30 000 and 2 000 000 ly. With `STANDARD_GRAVITY` instead the values differ,
because 9.806 65 m s⁻² is 1.0323 ly/yr², not 1.03: 0.2 % in *a* is 0.2 %
in the rapidity, and the coordinate time, which grows as *e*^*w*, is then
off by about 1 % at *T* = 5 years (84.5 against 83.7) and 2.6 % at 12
(116 147 against 113 243). The two are the same function; the page rounds
its constant. The crate's Andromeda anchor uses 2.5 Mly, where the page uses
2 000 000 ly; at that distance the crate gives 28.2 years.

**The GPS figures against Ashby.** The crate's `README.md` (lines 48 and 56 to
62) says that +45.65, −7.21 and +38.44 µs a day are the GPS worked example
Ashby's review uses, and `constants.rs` (lines 80 and 81) that `GM_EARTH` is
the value he uses "for the satellite-clock figures"; `lib.rs` and
`gravitational.rs` list them as published figures. The review as read for this
document (Europe PMC's full text of *Living Reviews in Relativity* 6:1) states
neither the split nor the net in microseconds. It gives the combined
fractional offset, eq. 35, +2.5046 × 10⁻¹⁰ − 6.9693 × 10⁻¹⁰ = −4.4647 × 10⁻¹⁰
in his sign, which is 38.575 µs a day. The crate's 38.439 is 0.137 µs a day
(0.36 %) lower because its ground clock is at rest on a sphere of radius 6 378
137 m, where Ashby's is on the rotating geoid. The difference is Ashby's own
eq. 18: Φ₀/*c*² = −6.953 48 × 10⁻¹⁰ (the mass term, the crate's step 2) − 3.76
× 10⁻¹³ (the quadrupole) − 1.203 × 10⁻¹² (the centripetal term) = −6.969 27 ×
10⁻¹⁰, so the quadrupole is 0.033 µs a day and the rotation 0.104. The Europe
PMC copy of his equation prints the quadrupole as 10⁻¹⁰, which disagrees with
its own text and sum. The README says this for the ISS and not for GPS. The
satellite terms agree: 3*GM*/2*ac*² is 2.504 557 × 10⁻¹⁰ with the crate's
radius, his 2.5046. His orbit, where the effects cancel, is *a* ≈ 9 545 km
[ashby2003]; the crate's, with a ground clock on a sphere, is 1.5 × 6 378 137
m = 9 567 km, and with a geoid clock 3*GM*/(2*c*² *L*G) = 9 545.5 km.

**The orbit radius.** `GPS_ORBIT_RADIUS` is documented as the semi-major axis
of an orbit of half a sidereal day about `GM_EARTH`, which the test derives.
The test checks the period against half of 86 164.1 s to 2 s; the constant's
period is 43 082.015 s against 43 082.05 s, and a radius that fits half of 86
164.1 s exactly is 26 561 764 m, 14 m larger. The effect on the offset is of
order 10⁻⁶ µs a day. Ashby's text, as read, gives the radius only as "4.2
earth radii" and per-satellite values, such as 2.656 139 556 × 10⁷ m on 22
July 2000; the constant itself is the crate's.

**Constants.** The values are CODATA 2022 for *G*, the file `gm_de440.tpc`
dated 14 December 2022 for the DE440 bodies, and Ashby's for the Earth; they
change when those sources are revised. *G* is the worst number, which is why
every formula takes a `GM`. `GM_EARTH` (IERS, WGS 84) differs from DE440's
`BODY399_GM`, 3.986 004 355 07 × 10¹⁴ m³ s⁻² [naif-gm-de440], by 1.58 × 10⁻⁸
of itself, a change of 6 × 10⁻⁷ µs a day in the GPS net. That is of the size
of *L*B, 1.55 × 10⁻⁸, which is how far TDB-compatible units sit from SI ones,
and `constants.rs` calls the DE440 values TDB-compatible; whether that
accounts for the difference was not checked. The crate mixes the two sets,
which at this size changes no output. Sagittarius A\*'s mass is good to two
figures: the arXiv abstract of [gravity2022], as a summary of the page gave
it, has 4.30 × 10⁶ *M*☉ to about 0.25 %; the crate's note adds a systematic
error that brings it to about 1 %, which was not re-read.

**Model limits, as numbers.** The orbit is Keplerian and circular. Ashby's
eccentricity term is 4.4428 × 10⁻¹⁰ *e* √*a* s/√m in amplitude, which is 2.29
µs × *e* at the GPS radius; he gives 23 ns for *e* = 0.01 [ashby2003]. The
quadrupole and the rotation together are 0.137 µs a day, as above. The Sagnac
effect "can amount to hundreds of nanoseconds" in a time transfer [ashby2003]
and is not a rate of a clock. None is in the crate; each is nanoseconds, not
microseconds, on the 38 µs a day, as the README says. The weak-field
composition of gravity and speed, exact only for the velocity a local static
observer measures, leaves an error of the product of the two small terms; at
GPS it is 10⁻²⁰ of a rate, and the exact and weak-field offsets differ by 3.6
× 10⁻¹⁸ in rate (3 × 10⁻⁷ µs a day).

**Floating-point.** The hyperbolic functions are built from `hc_core::math`
with series near zero and two identities against cancellation, cosh *b* − cosh
*a* = 2 sinh((*b* + *a*)/2) sinh((*b* − *a*)/2) and arcosh(1 + *y*) = 2 arsinh
√(*y*/2); without the first, one second of 1 g gets its distance wrong by tens
of per cent. The round-trip tests hold them to 10⁻¹² for the inverse pairs,
10⁻¹¹ for artanh tanh *x* up to a rapidity of 5, 10⁻⁵ to 15, and 10⁻¹⁴ for the
fundamental identity scaled by cosh². The module's own comment claims better
than 10⁻¹³ over the whole range; the tests assert 10⁻¹² and the README's 10⁻¹¹
to a rapidity of 5. Past 5, a double cannot hold how close tanh comes to 1,
and the crate says to use `lorentz_factor_from_rapidity`. `Duration` results
are exact to the attosecond given the `f64` factor. `rocket_coordinate_time`
overflows at rapidity about 710, which at 1 g is about 690 years of ship time.
All 126 tests of `hc-relativity` pass.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [ashby2003] | The weak-field metric and proper time (eqs 12, 27), the geoid potential Φ₀ (eq. 18), the satellite offset (eqs 35, 36), the eccentricity term (eq. 39), `GM_EARTH` and the equatorial radius | Yes, 2026-10-03, Europe PMC's full text (PMC5253894) with the equations as LaTeX; eq. 18 prints the quadrupole as 10⁻¹⁰ where its text and sum give 10⁻¹³ |
| [baez-rocket] | The rocket relations, the table of *T* = 1 to 12 years, the flip-and-burn formula and its table, the limit of 10⁹ ly | Yes, 2026-10-03 |
| [baez-acceleration] | Constant proper acceleration as rapidity linear in proper time | Yes, 2026-10-03 |
| [baez-velocities] | γ and the composition law | Yes, 2026-10-03 |
| [roberts-sr-experiments] | The citation of Ives and Stilwell | The entry only, 2026-10-03; the paper was not read |
| [soffel2003] | The text of Resolution B1.9, the definition of *L*G, *L*C and *L*B | Yes, 2026-10-03, the arXiv HTML; the IAU's text was not opened |
| [klioner2010] | *L*G and *L*B as defining constants, IAU 2006 Resolution 3 | Yes, 2026-10-03, the ar5iv HTML; the resolution was not opened |
| [codata2022] | *G* and its uncertainty | The NIST page for *G*, 2026-10-03 |
| [nist-codata-gn] | `STANDARD_GRAVITY` | Yes, 2026-10-03; the CGPM of 1901 was not read |
| [bipm-si-defining-constants] | `SPEED_OF_LIGHT` | Yes, 2026-10-03 |
| [naif-gm-de440] | `GM_SUN`, `GM_MOON`, `GM_MARS`, `GM_JUPITER`, and `BODY399_GM` for the comparison | Yes, `gm_de440.tpc` itself, 2026-10-03 |
| [gravity2022] | `SAGITTARIUS_A_STAR_SOLAR_MASSES` | The abstract page on arXiv, 2026-10-03, through a summary of it; the paper's section 4 was not re-read |
| [park2021] | DE440 | Not read |
| [prsa2016] | `GM_SUN_NOMINAL_IAU_2015` | Not re-read |
| [iau-2012-b2] | `ASTRONOMICAL_UNIT` | Not re-read |
| [iers-tn36] | `GM_EARTH` as the IERS value | Not re-read; WGS 84 not read |
| [ostp-2024-celestial-time] | The 58.7 µs a day of the Moon | Not re-read |

Irwin and Fukushima (1999), for *L*C, is cited through [soffel2003] and was
not read. The IAU's own texts of Resolutions B1.9 (2000) and 3 (2006) were
not opened; their constants were read as the two papers quote them. Ives
and Stilwell (1938) was not read.

## Code

`crates/hc-relativity/src/constants.rs`, `special.rs`, `gravitational.rs`,
`worldline.rs`, `dilated.rs`, `hyperbolic.rs` and `error.rs`, with the
README of the crate. The facade layer is
`crates/hyper-calendar/src/relativity_lines.rs` and the `relativity` group of
`crates/hyper-calendar/src/exports.rs`, with
`crates/hyper-calendar-wasm/src/relativity.rs` and
`crates/hyper-calendar-ffi/src/relativity.rs`.

The tests that anchor them are those in the Accuracy table, and
`the_gps_orbit_is_a_half_sidereal_day`,
`the_solar_gm_matches_the_product_of_g_and_the_solar_mass` and
`every_body_names_the_constant_that_holds_its_gm` for the constants. The
crate has 126 tests, in the source files; it has no `tests` directory.
