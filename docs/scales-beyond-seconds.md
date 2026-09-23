# Scales beyond seconds, and time that is not known exactly

Requirement 6 of the brief: support times that an ordinary calendar cannot
reach — the chronology of the universe, Planck time — and, because such values
are never exactly known, carry significant figures, error bars and vague ranges
alongside them.

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

So the library splits the problem. Exact things are exact; inexact things say
how inexact they are.

## `hc-uncertainty`

| Type | Answers |
| --- | --- |
| `Significant` | "13.8 billion years" has three significant figures, and arithmetic on it must not manufacture more |
| `Uncertain { value, std_dev }` | A Gaussian, with first-order propagation through `+ − × ÷`, `ln`, `exp`, `powf`, and weighted combination |
| `DurationInterval` | A closed `[lo, hi]` of `Duration`, with interval arithmetic |
| `FuzzyInstant` | An instant that is exact, known to a resolution, bounded, Gaussian, open-ended, or simply unknown |

`FuzzyInstant` implements **Allen's interval algebra**: given two vague
instants, `before`, `meets`, `overlaps`, `starts`, `during`, `finishes`,
`equals` and their inverses are computed as the *set* of relations that remain
possible. Asking "did A happen before B" when both are vague gets an honest
answer — sometimes `{before, meets, overlaps}` — instead of a coin flip.

### EDTF

`hc-uncertainty::edtf` parses and renders **ISO 8601-2 / Extended Date/Time
Format**, the standard vocabulary libraries and archives already use for this:

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

Supporting the standard rather than inventing a notation means the library can
round-trip data from archival catalogues without loss.

## `hc-deep-time`

Above and below the range where seconds are a comfortable unit, the useful
representation is a magnitude with an exponent, not a count.

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
- **Long astronomical periods**: the precession of the equinoxes and the
  galactic year, each with its spread and whether it drifts.
- **Future chronology**: the Sun's remaining stages, the end of star
  formation, the lower bound on proton decay, black hole evaporation times,
  and the four cosmological eras out to the Dark Era — the values that make
  the logarithmic scale necessary.

Every entry in those tables carries its uncertainty. A timeline that says the
Hadean began 4.567 Ga with no error bar is not a timeline, it is a decoration.

## What this is not

It is not a physics engine and not a cosmology solver. `hc-deep-time` carries
published values and lets you do arithmetic on them with the error bars intact.
Computing a value from a cosmological model is the caller's job; the library's
job is to stop the error bars from being dropped on the way.
