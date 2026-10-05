# Scales beyond seconds, and time that is not known exactly

This document covers times that an ordinary calendar cannot reach, from the
Planck time to the chronology of the universe. Such values are never known
exactly, so it also covers how the library carries significant figures,
error bars and vague ranges. Three crates are involved: `hc-uncertainty`,
`hc-deep-time` and `hc-orbital`.

## Why `Duration` is not enough

`hc-core::Duration` is exact: `i128` seconds plus attoseconds. That is the
right representation for everything a clock can measure, and the wrong one for
everything it cannot.

- Planck time is about 5.39×10⁻⁴⁴ s. An attosecond is 10⁻¹⁸ s. An exact integer
  type would need 26 more orders of magnitude, and the value it would store is
  not exact anyway — the Planck time is derived from measured constants and is
  known to about 1 part in 10⁵.
- The age of the universe is about 13.787 billion years ± 0.020. Storing it as
  435 084 600 000 000 000 s implies eighteen significant figures where there
  are five.

So the library keeps two kinds of value. An exact value is exact. An inexact
value carries how inexact it is.

## `hc-uncertainty`

Each row names a type and the question it answers. The system is written up in
[systems/uncertainty.md](systems/uncertainty.md).

| Type | Answers |
| --- | --- |
| `Significant` | "13.8 billion years" has three significant figures, and arithmetic on it must not manufacture more |
| `Uncertain { value, std_dev }` | A Gaussian, with first-order propagation through `+ − × ÷`, `ln`, `exp`, `powf`, and weighted combination |
| `DurationInterval` | A closed `[lo, hi]` of `Duration`, with interval arithmetic |
| `FuzzyInstant` | An instant that is exact, known to a resolution, bounded, Gaussian, open-ended, or unknown |

`FuzzyInstant` implements **Allen's interval algebra**: given two vague
instants, `before`, `meets`, `overlaps`, `starts`, `during`, `finishes`,
`equals` and their inverses are computed as the *set* of relations that remain
possible. Asking "did A happen before B" when both are vague can return a
set such as `{before, meets, overlaps}` rather than one relation.

### EDTF

`hc-uncertainty::edtf` parses and renders **ISO 8601-2 / Extended Date/Time
Format**, the notation libraries and archives use for uncertain dates. Each
row is a form the parser accepts:

| Form | Meaning |
| --- | --- |
| `1984?` | uncertain |
| `1984~` | approximate |
| `1984%` | both |
| `1984-01-XX` | day unspecified |
| `Y-170000002` | a year far outside the four-digit range |
| `1984/1985` | an interval |
| `..1760-12-03` | open start |
| `1760-12..` | open end |
| `[1667,1668,1670..1672]` | one of these |
| `{1960,1961-12}` | all of these |

Every accepted form renders back to the same text. The forms the parser
rejects — times of day, seasons, qualification of single components and
exponential years — are listed in the
[`hc-uncertainty` README](../crates/hc-uncertainty/README.md).

## `hc-deep-time`

Above and below the range where seconds are a comfortable unit, the useful
representation is a magnitude with an exponent, not a count. The geological
time scale, whose authority is the International Commission on Stratigraphy,
is written up in [systems/geologic-time-scale.md](systems/geologic-time-scale.md).

- `DeepTime` — a value in seconds as an `Uncertain` plus a count of
  significant figures, with convenience constructors for the units the
  literature uses: Planck times, yoctoseconds through days, Julian years,
  kiloyears, megayears and gigayears. Ages before present (BP, counted from
  1950 by convention in radiocarbon work) are `archaeology::Bp`.
- Logarithmic comparison and formatting, because the interesting question about
  10⁻⁴³ s and 10¹⁷ s is the ratio, not the difference.
- **The chronology of the universe** as data: eleven epochs, from the Planck
  epoch through grand unification, inflation, the electroweak, quark, hadron,
  lepton and photon epochs, the dark ages and reionisation to the era of
  galaxies; and nine dated events, from neutrino decoupling, nucleosynthesis,
  matter–radiation equality and recombination to the first stars and
  galaxies, the formation of the Milky Way and of the Sun and Solar System,
  and the present — each with its stated uncertainty and its source.
- **The geological time scale** as data: eons, eras, periods, epochs and ages
  with ICS boundary ages and their published uncertainties.
- **The earliest evidence** of life, of *Homo sapiens* and of writing: the
  published claims, each dated as an age, a minimum age or a range as its
  source gives it, and the disputed ones named with their rebuttals
  ([`systems/earliest-evidence.md`](systems/earliest-evidence.md)).
- **Long astronomical periods**: the precession of the equinoxes and the
  galactic year, each with its spread and whether it drifts.
- **Future chronology**: the Sun's remaining stages, the end of star
  formation, the lower bound on proton decay, black hole evaporation times,
  and the four cosmological eras out to the Dark Era — the values that make
  the logarithmic scale necessary.

Every entry in those tables carries its uncertainty and its source, and a
stable lower-case identifier that the WebAssembly and C lines write beside
its English name.

## `hc-orbital`

Between the calendar and deep time lies the span the ice ages happened in:
tens to hundreds of thousands of years, where the Earth's orbit is no
longer the fixed ellipse a calendar assumes. `hc-orbital` evaluates
Berger's 1978 trigonometric solution for the **Milankovitch elements** —
eccentricity, obliquity, the longitude of perihelion from the moving
equinox and the climatic precession *e* sin ϖ — and the daily insolation
that follows from them, for a million years either side of 1950.

- "Present" is 1950, the same datum as `archaeology::Bp`, so a BP age is
  the argument.
- Every element is an `Uncertain` whose error bar is the measured
  disagreement with the author's later solution, in tiers that widen with
  distance.
- Beyond ±1 Myr the crate refuses ([ADR 0006](adr/0006-refuse-to-extrapolate.md)); a longer solution — Laskar
  et al. 2004 — is named and not carried.

The explanation and the worked example are in
[`systems/orbital-elements.md`](systems/orbital-elements.md).

## At the boundary

The WebAssembly module and the C library write these crates as lines, one
layer each; every cell is text and each README lists the columns. **A value
the library holds exactly crosses as an integer or a pair of integers, never
as a float**: an EDTF date is a fixed day, an interval's bounds are whole
seconds and attoseconds, a unit's length is a numerator and a denominator in
lowest terms, and a figure count is an integer. A measurement crosses with its
standard deviation, and each line says to how many figures it may be read.

| Layer | Exports | Reads |
| --- | --- | --- |
| `uncertainty` | `hc_edtf_parse`, `hc_edtf_relations`, `hc_significant`, `hc_significant_op`, `hc_uncertain`, `hc_uncertain_op`, `hc_interval` | `hc_uncertainty::{edtf, fuzzy, sig_figs, quantity, interval}`: an EDTF value placed on the timeline with its support and Allen's relations to another, a number's figures and the laboratory rules of arithmetic on them, a `value ± σ` with the first-order rules and the weighted mean, intervals of whole seconds |
| `units` | `hc_units`, `hc_unit_convert`, `hc_rates`, `hc_frame_period`, `hc_tempo` | `hc-units`: the 53 exactly defined units with their lengths as ratios, an exact conversion, the period of a frame or a sample, a note at a tempo and its MIDI value |
| `deep-time` | `hc_place_years_ago`, `hc_cosmic_events`, `hc_earliest_evidence`, `hc_archaeological_periods`, `hc_future_events`, `hc_geologic_intervals`, `hc_planck_units`, `hc_bp_convert`, `hc_deep_convert`, `hc_deep_compare` | `hc_deep_time::{constants, archaeology, magnitude, universe, evidence, periods, future, geologic, timeline}`: a moment some years ago placed in every chronology, the cosmic epochs and events, the earliest-evidence claims, the archaeological periods, the dated events of the far future and the geologic intervals of a rank, the CODATA 2022 Planck units with their bars, the BP, b2k and calendar-year datums (a radiocarbon age is refused: it needs a calibration curve), a magnitude in any unit, and two magnitudes compared across decades |
| `orbital` | `hc_orbit_at`, `hc_orbit_series`, `hc_daily_insolation` | `hc-orbital`: Earth's orbital elements and the June insolation at 65° N at an epoch, the same over a series of epochs, and `hc_orbital::daily_insolation` at any latitude and solar longitude |
| `relativity` | `hc_proper_time`, `hc_gravitational_dilation`, `hc_gravitating_bodies`, `hc_orbit_rate_offset`, `hc_rocket`, `hc_flip_and_burn`, `hc_doppler`, `hc_velocity_add`, `hc_schwarzschild_radius`, `hc_proper_time_uncertain` | `hc-relativity`: a clock at a constant speed, a clock held still at a radius from a body and the bodies carried, the GPS orbit offset, hyperbolic motion, the Doppler factor, velocity composition with `1 − β` held without cancellation, the Schwarzschild radius, and a proper time with the standard deviation of an uncertain speed |

The line makers are `hyper_calendar::{uncertainty_lines, units_lines,
deep_time_lines, orbital_lines, relativity_lines}`; each is tested against
values recomputed in 50- or 60-digit decimal arithmetic (Python's `decimal`).
Sources read: [wikipedia-constant-acceleration] for the rocket's formulas and
[wikipedia-before-present] for BP and b2k (11 650 BP is 9701 BC); the
11 700 b2k of the Holocene's base is the crate's own and is not on that page.

## What this is not

It is not a physics engine and not a cosmology solver. `hc-deep-time`
carries published values and does arithmetic on them with the error bars
kept. Computing a value from a cosmological model is the caller's job.
