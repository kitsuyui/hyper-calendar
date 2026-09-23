# hc-relativity

Relativistic time for the `hyper-calendar` workspace: Lorentz transforms,
special and gravitational time dilation, and worldline integration.

Clocks do not agree. One in orbit runs fast, one on a fast ship runs slow, and
the difference is large enough to matter — a GPS satellite gains 38 µs a day,
which is ten kilometres of positioning error. This crate computes those
differences well enough to build a timeline with: a satellite constellation's,
or a science-fiction novel's.

## What it covers

| Module | What it does |
| --- | --- |
| `constants` | `c`, `G`, standard gravity, the Julian year, the light-year, the AU, and the standard gravitational parameters of the Sun, Earth, Moon, Mars, Jupiter and Sagittarius A\* — each with its source and a note on how well it is known. |
| `special` | Lorentz factor from β, from m/s and from rapidity; rapidity and its inverse; collinear velocity composition; proper ↔ coordinate time; relativistic Doppler (general, longitudinal, transverse); aberration. |
| `gravitational` | Schwarzschild radius; static dilation factor; gravitational frequency ratio and redshift; circular-orbit speed; the exact circular-geodesic factor `√(1 − 3GM/rc²)`; and the combined orbit-versus-ground rate offset, exact and weak-field. |
| `worldline` | Piecewise worldlines of segments (constant velocity or constant proper acceleration, with an optional static potential), integrated to proper time; plus the closed-form relativistic-rocket relations and the flip-and-burn profile. |
| `dilated` | A `ClockComparison` attached to real `hc_core::Instant<Tai>` values, so a ship's clock and an Earth clock can be printed side by side; and an `hc_uncertainty::Uncertain` result when the cruise velocity carries an error bar. |

## Anchors

Each is a published figure and each is a test:

| Quantity | Computed | Source |
| --- | --- | --- |
| Lorentz factor at β = 0.6 | 1.25 exactly | any textbook |
| GPS gravitational gain | +45.65 µs/day | the standard worked example |
| GPS kinematic loss | −7.21 µs/day | " |
| GPS net gain | +38.44 µs/day | " |
| Schwarzschild radius of the Sun | 2 953.25 m | 2GM☉/c² |
| 1 g flip-and-burn to Andromeda (2.5 Mly) | 28.60 years aboard, 2 500 002 at home | the relativistic-rocket relations |
| 1 g for one year of ship time | β = 0.7748, 0.564 ly covered | " |
| ISS-altitude circular orbit | −24.5 µs/day | this model; see below |

The GPS figures come out right because the ground clock is placed at the WGS 84
equatorial radius and the satellite at the nominal 26 561 750 m semi-major
axis, and because `GM_EARTH` is the value the GPS control segment itself uses.
The exact Schwarzschild computation and the weak-field expansion are both
implemented and agree to 8·10⁻⁹ of themselves — 3·10⁻⁷ µs/day — which is one
of the tests.

## What it deliberately does not do

- **The metric is Schwarzschild**: non-rotating, uncharged, spherically
  symmetric. The Earth's quadrupole moment `J₂`, the Kerr frame-dragging term,
  the rotation of a ground station, orbital eccentricity and the Sagnac effect
  are all absent. Each moves the GPS numbers by nanoseconds per day, not
  microseconds — but a real time-transfer system needs all of them, and this
  crate should not be used as though it had them. The ISS figure above is
  −24.5 µs/day where the usually quoted number is nearer −28; the test
  attributes the difference to the rotating geoid and the station's
  non-circular orbit, neither of which is modelled.
- **`circular_orbit_speed` is Newtonian.** The relativistic correction is of
  order `r_s/r`, which is 5·10⁻¹⁰ at GPS altitude. It would matter near a
  black hole and the function says so.
- **Gravity and motion are combined multiplicatively.** `√(1−r_s/r)·√(1−β²)`
  is the weak-field composition, exact only when the velocity is the one a
  local static observer measures. The error is the product of the two small
  terms — parts in 10¹⁸ in Earth orbit.
- **No numerical integration.** Every worldline segment is closed-form, so
  there is no step size and no accumulated error; the price is that only two
  velocity profiles exist.
- **No `f32`, and no `NaN` ever.** `|β| ≥ 1`, a radius inside the horizon, a
  γ below 1 and a backwards worldline segment are all named errors caught
  where they occur, rather than silent `NaN`s surfacing as a nonsensical
  arrival date.

## Accuracy claimed

- The hyperbolic functions are built from `hc_core::math`'s exponentials with
  Taylor branches near zero and large-argument shortcuts; round-trip tests
  hold them to better than 1e-11 relative over rapidities up to 5. Two
  identities are used specifically to avoid cancellation: `cosh b − cosh a =
  2 sinh((b+a)/2) sinh((b−a)/2)`, and `arcosh(1+y) = 2 arsinh √(y/2)`. Without
  them a one-second 1 g burn gets its distance wrong by tens of per cent.
- `atanh(tanh(w))` loses about one digit per unit of rapidity above 5, because
  a double cannot hold how close `tanh` gets to 1. Use
  `lorentz_factor_from_rapidity` rather than round-tripping through β at
  extreme speed; that is stated in the docs and tested as a known limit.
- Proper-time results are `hc_core::Duration`, exact to the attosecond within
  the precision of the `f64` factor that produced them.
- Uncertain propagation is analytic, not composed from the generic
  `Uncertain` operators: β appears twice in `√(1 − β²)` and those operators
  assume independence.

## Where the reference data came from

- `c`, standard gravity, the Julian year, the light-year, the astronomical
  unit and the WGS 84 equatorial radius are **defined** values (2019 SI, 3rd
  CGPM 1901, IAU 2012 Resolution B2, WGS 84).
- `G` = 6.674 30(15)·10⁻¹¹, CODATA 2018 — relative uncertainty 2.2·10⁻⁵, by
  far the worst number here, which is why every formula takes a `GM`.
- `GM☉` — IAU 2015 Resolution B3 nominal value. `GM⊕` — IERS Conventions
  (2010) and WGS 84. Moon — JPL DE430. Mars and Jupiter systems — JPL DE440.
- Sagittarius A\* — (4.297 ± 0.013)·10⁶ M☉, GRAVITY Collaboration, A&A 625,
  L10 (2019). Good to three figures, and `GravitatingBody::source` says so.
- The transverse Doppler shift as a confirmation of time dilation: Ives and
  Stilwell (1938).

## Feature flags

`default = ["std"]`, `std = ["alloc", ...]`, `alloc = [...]`. Nothing here
needs a heap. A build without `std` also enables `libm`, which passes through
to `hc-core` for floating-point math; `--no-default-features` alone stops at
`hc-core`'s compile-time guard.
