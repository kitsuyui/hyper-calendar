# Time off Earth, and time at speed

Requirement 7 of the brief: clocks on other bodies — Mars sols and the rest —
and the time dilation of special and general relativity, with enough
flexibility that a science-fiction timeline can actually be computed rather
than hand-waved.

## `hc-planetary`

### Mars

Mars is the case with real operational standards behind it, so it gets the most
detail.

| Quantity | Meaning |
| --- | --- |
| **Sol** | A Martian solar day: 88 775.244 s, about 2.7% longer than Earth's |
| **MSD** | Mars Sol Date — the sol count from a 1873-12-29 epoch, the Martian analogue of the Julian Day |
| **MTC** | Coordinated Mars Time — mean solar time at the Martian prime meridian, MSD's fractional part in 24 Martian hours |
| **LMST** | Local Mean Solar Time at a given west longitude |
| **LTST** | Local True Solar Time, LMST plus the Martian equation of time (which runs from about −51 to +40 minutes, far more than Earth's −14 to +16) |
| **Mission sol** | The landing-relative sol count each mission uses. All ten surface missions are tabulated: Viking 1 and 2, Mars Pathfinder, Spirit, Opportunity, Phoenix, Curiosity, InSight, Perseverance and Zhurong |
| **Darian calendar** | Gangale's 24-month Martian calendar, the most developed proposal for a Martian civil calendar |
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

Coordinated Lunar Time is tracked as *researching*: the 2024 US policy
directive asked for one, the standard is still being defined, and inventing a
definition would be worse than waiting.

The WebAssembly module and the C library carry Mars time, the mission sols,
the body table and each body's local mean solar time as their `planetary`
layer; `crates/hyper-calendar-wasm/README.md` gives the lines.

## `hc-relativity`

Time dilation is the part that makes an interstellar timeline computable.

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

The canonical check is GPS: a satellite clock runs about +45.7 µs/day fast from
the weaker gravitational potential and about −7.2 µs/day slow from its orbital
speed, for a net +38.4 µs/day. That number is in the test suite, because if the
library gets it wrong nothing else it says about relativity is trustworthy.

The two boundary crates carry a constant-velocity clock and a clock held
still at a radius as their `relativity` layer, a layer of its own because it
shares no crate with `planetary`; each line names the constants it used.

### Worldlines

A `Worldline` is a sequence of segments, each with a coordinate duration, a
velocity profile (constant, or constant proper acceleration) and an optional
gravitational potential. Integrating it gives the proper time elapsed along the
path.

That is what turns the library from a calculator into something you can plot a
story against:

- A twin paradox with a real turnaround, not an idealised instantaneous one
- A 1g relativistic rocket: the standard closed forms for distance, coordinate
  time, proper time and final velocity. (A 1g flip-and-burn to Andromeda,
  2.5 Mly, is about 28.6 years of ship time and 2.5 million years of Earth
  time — the test suite checks both.)
- A ship's clock and a planetary clock compared as `Instant`s, with
  `hc-uncertainty` carrying the error through when the inputs are uncertain

### What it is not

No general-relativistic field solver, no numerical spacetime, no rotating or
charged black holes. Schwarzschild geometry and flat-space special relativity
cover the cases a calendar library is asked about, and anything past that
belongs in a physics package rather than behind a date API.
