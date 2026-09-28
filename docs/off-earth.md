# Time off Earth, and time at speed

This document describes two crates. `hc-planetary` keeps time on other
bodies: Mars sols, mission sol counts, and calendars for Mars, Titan and the
Galilean moons. `hc-relativity` computes the time dilation of special and
general relativity, and integrates a journey's proper time, the time a clock
carried along it reads.

## `hc-planetary`

### Mars

Mars is the only other body with operational timekeeping standards, so it
has the most detail. Each row names a quantity `hc_planetary::mars`
computes.

| Quantity | Meaning |
| --- | --- |
| **Sol** | A Martian solar day: 88 775.244 s, about 2.7% longer than Earth's |
| **MSD** | Mars Sol Date — the sol count from a 1873-12-29 epoch, the Martian analogue of the Julian Day |
| **MTC** | Coordinated Mars Time — mean solar time at the Martian prime meridian, MSD's fractional part in 24 Martian hours |
| **LMST** | Local Mean Solar Time at a given west longitude |
| **LTST** | Local True Solar Time, LMST plus the Martian equation of time (which runs from about −51 to +40 minutes, far more than Earth's −14 to +16) |
| **Mission sol** | The landing-relative sol count each mission uses. All ten surface missions are tabulated: Viking 1 and 2, Mars Pathfinder, Spirit, Opportunity, Phoenix, Curiosity, InSight, Perseverance and Zhurong, as `viking-1`, `viking-2`, `mars-pathfinder`, `spirit`, `opportunity`, `phoenix`, `curiosity`, `insight`, `perseverance` and `zhurong`; each counts its sols on the clock its team kept, `local-mean-solar-time` or `local-true-solar-time-at-landing` |
| **Darian calendar** | Gangale's 24-month calendar for the Martian year, a proposal for Martian civil use |
| **Martiana calendar** | Gangale's variant of it with Aitken's week: the week never shortened, every month of a quarter beginning on the same sol, a two-year cycle, and a decennial sol outside the week |
| **Mars year** | The Clancy convention, counting from the 1955 northern spring equinox, used throughout Mars atmospheric science |

### Other bodies

Rotation periods, orbital periods, axial tilts and semi-major axes for 22
bodies — the Sun, the eight planets, the Moon, Phobos and Deimos, Ceres, the
Galilean moons, Enceladus, Titan, Triton, Pluto and Charon — with the solar
day and the year in local days derived from them, enough to define a local
solar day and a year on each, and the source for every constant.

### Calendars for the moons of Jupiter and Saturn

Gangale extended the Darian calendar to the moons that have no day a person
could live by: each is tidally locked, so its solar day is its orbit, 1.8 to
16.8 Earth days. His calendars divide the solar day into *circads* of about
21 to 24 hours and build eight-circad weeks, months and a borrowed year out
of them. `hc-planetary` carries the **Darian calendar for Titan**, 24 months
in the Martian year with a circad of one sixteenth of Titan's day, and the
**Gregorian-based calendars of Io, Europa, Ganymede and Callisto**, thirteen
months in the Earth year under the Gregorian year number; the Darian-based
Galilean family is read and not yet carried, because its source contradicts
itself where the dates depend on it. All of them run through one engine
whose fixed day is a circad, not an Earth day. The rules, their sources and
their limits are in [systems/circad-calendars.md](systems/circad-calendars.md).

Coordinated Lunar Time (LTC) is not carried. The US Office of Science and
Technology Policy asked for one in April 2024. As of 2026-09-26 no
definition had been published, and the library does not invent one. The
roadmap row in [calendars.md](calendars.md#stage-6--non-terrestrial) is
Researching.

The WebAssembly module and the C library carry Mars time, the mission sols,
the body table, each body's local mean solar time, and the dates of the
Titan, Galilean and Martiana calendars at an instant (`hc_circad_date`) as
their `planetary` layer; `crates/hyper-calendar-wasm/README.md` gives the
lines.

## `hc-relativity`

### Special relativity

- Lorentz factor from velocity, in m/s or as a fraction of c
- Rapidity, and velocity addition that stays under c by construction
- Proper time along a constant-velocity segment, and its inverse
- Relativistic Doppler factor and aberration
- `beta >= 1` is an error, not a `NaN`

### Gravitation

- Schwarzschild time dilation at a radius from a mass
- Gravitational redshift between two radii
- Schwarzschild radius
- The combined kinematic and gravitational factor for a circular orbit

The reference check is GPS. A satellite clock runs about 45.7 µs a day fast
from the weaker gravitational potential, and about 7.2 µs a day slow from its
orbital speed, for a net 38.4 µs a day fast. The test suite checks all three
figures.

The two boundary crates carry a constant-velocity clock and a clock held
still at a radius as their `relativity` layer, a layer of its own because it
shares no crate with `planetary`; each line names the constants it used.

### Worldlines

A `Worldline` is a sequence of segments, each with a coordinate duration, a
velocity profile (constant, or constant proper acceleration) and an optional
gravitational potential. Integrating it gives the proper time elapsed along the
path.

A worldline answers questions such as these:

- A twin paradox with a real turnaround, not an idealised instantaneous one
- A 1 g relativistic rocket: the standard closed forms for distance,
  coordinate time, proper time and final velocity. A 1 g flip-and-burn to
  Andromeda, 2.5 million light-years, takes about 28.6 years of ship time
  and 2.5 million years of Earth time. The test suite checks both.
- A ship's clock and a planetary clock compared as `Instant`s, with
  `hc-uncertainty` carrying the error through when the inputs are uncertain

### What it is not

It has no general-relativistic field solver, no numerical spacetime, and no
rotating or charged black holes. The geometry is Schwarzschild's, for a
non-rotating mass, plus flat-space special relativity. Anything beyond that
belongs in a physics package.
