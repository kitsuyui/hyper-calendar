# Deep time: published magnitudes with error bars, the cosmic and future chronologies, the Planck units and the datums of "before present"

Backs the crate `hc-deep-time` (the facade feature `deep-time`), except the
two tables that have documents of their own: `geologic` is
[geologic-time-scale.md](geologic-time-scale.md) and `evidence` is
[earliest-evidence.md](earliest-evidence.md). It is what this document covers:
`magnitude` (`DeepTime`, `DeepUnit`), `constants` (`PhysicalConstant`, the
Planck units), `universe` (`CosmicEpoch`, `CosmicEvent`, `EPOCHS`, `EVENTS`,
`AGE_OF_UNIVERSE`), `future` (`FutureEvent`, `FutureEra`, `EVENTS`, `ERAS`,
`black_hole_lifetime`), `archaeology` (`Bp`, `Calibration`, `b2k_to_bp`,
`bp_to_b2k`, `PERIODS`), `periods` (`PRECESSION_OF_THE_EQUINOXES`,
`GALACTIC_YEAR`) and `timeline` (`place`, `span_between`, `Placement`); and the
exports that write them (`hyper_calendar::deep_time_lines`): `hc_place_years_ago`,
`hc_cosmic_events`, `hc_archaeological_periods`, `hc_future_events`,
`hc_planck_units`, `hc_bp_convert`, `hc_deep_convert` and `hc_deep_compare`
(with `hc_earliest_evidence` and `hc_geologic_intervals`, which the other two
documents describe). No calendar identifier is registered: everything here is
a span in SI seconds or a count of years before a stated epoch, and a calendar
year becomes a fixed day in `hc-calendar`.

## What it is

An ordinary calendar counts days exactly. Below an attosecond and above the
span where a count of seconds is a sensible answer, there is nothing to count:
the Planck time, the age of the universe, the time before the Sun leaves the
main sequence, and the time a black hole takes to evaporate are *published
magnitudes with error bars*. This crate stores them as such, so that the error
bar is never dropped on the way: a span is an uncertain number of seconds and
the number of figures the source prints.

**The Planck units.** The Planck time is `t_P = √(ħG/c⁵)`, the length `l_P =
√(ħG/c³)`, the mass `m_P = √(ħc/G)`. Since the 2019 SI redefinition `c` and `h`
are exact, so `G` is the only measured constant in them, and it is the
worst-measured constant of the CODATA table: 6.674 30(15)×10⁻¹¹ m³ kg⁻¹ s⁻²,
a relative standard uncertainty of 2.2×10⁻⁵. Every Planck unit inherits it,
halved by the square root, which gives the time and the length 1.1×10⁻⁵
[codata2022].

**The cosmic chronology.** The ages of the epochs of the universe come from the
Friedmann equation of a flat ΛCDM universe,
`t(a) = (1/H₀) ∫₀ᵃ a′ da′ / √(Ωr + Ωm a′ + ΩΛ a′⁴)`, where `a = 1/(1+z)` is the
scale factor, `Ωm` the matter density, `Ωr` the radiation density and `ΩΛ = 1 −
Ωm − Ωr`. The parameter set is one column of one table of the Planck 2018
results, the `TT,TE,EE+lowE+lensing+BAO` column: age 13.787 ± 0.020 Gyr,
`H₀` = 67.66 ± 0.42 km s⁻¹ Mpc⁻¹, `Ωm` = 0.3111 ± 0.0056, the redshift of last
scattering `z*` = 1089.80 ± 0.21 and of matter–radiation equality `z_eq` =
3387 ± 21 [planck2018-vi]. Using one column throughout is what keeps the
chronology consistent with a single likelihood. Before nucleosynthesis nothing
is measured: the boundaries are the conventional powers of ten of the textbook
chronology, and each carries an uncertainty equal to itself, "a decade, not a
measurement".

**The future chronology.** Two clocks are kept apart on purpose. A stellar or
experimental event is counted in years from the present, as it is published:
the Sun leaves the main sequence 5.42 ± 0.10 Gyr from now, the tip of the red
giant branch is 7.59 ± 0.05 Gyr away, the Sun becomes a white dwarf in 7.72 ±
0.05 Gyr, by the model of Schröder and Connon Smith. The eras of Adams and
Laughlin are counted in *cosmological decades* `η = log₁₀(t/yr)` from the Big
Bang, their own unit: Stelliferous 6 < η < 14, Degenerate 15 < η < 37, Black
Hole 38 < η < 100 and the Dark Era from 100, with the gaps between them theirs.
An event is `Modelled`, an `ExperimentalBound` (the proton lifetime of
2.4×10³⁴ years at 90 % confidence from Super-Kamiokande is a bound, not a
prediction) or an `OrderOfMagnitude`. The Hawking evaporation time of a
Schwarzschild black hole is `τ = 5120 π G² M³ / (ħ c⁴)`.

**"Before present" and its datums.** BP counts years back from 1950 CE, fixed
there because the atmospheric nuclear tests of the 1950s made later samples
useless as a radiocarbon baseline. Ice-core chronologies count from 2000 CE,
"b2k", fifty years ahead of it: the Holocene's base is 11 700 b2k, which is
11 650 BP. A *conventional radiocarbon age* is not a calendar age at all: it is
computed from the measured ¹⁴C activity on three conventions (the Libby
half-life of 5568 years, a constant atmospheric concentration, and a δ¹³C
normalisation), and turning it into a calendar age is *calibration* against a
curve (IntCal20 and its companions). A calendar year is counted in astronomical
numbering, which has a year 0, so 1950 BP is year 0 and 11 650 BP is year
−9700, which is 9701 BCE.

## How it works

*`DeepTime`.* An `Uncertain` count of seconds (a value and a standard
deviation, see [uncertainty.md](uncertainty.md)) with the figure count the
source prints. It has 17 units: the Planck time, the SI prefixes from yocto to
milli, the second, minute, hour and nominal day, the Julian year of 365.25 days
of 86 400 s, and kilo-, mega- and gigayears of it. Every unit but the Planck
time is exact by definition, its length read from `hc-units`'s catalogue
(`DeepUnit::defined_unit`; the three year multiples are `hc-units`'s Julian
year times a power of ten), and rescales the standard deviation exactly; the
Planck time is CODATA's measurement and brings its own 1.1×10⁻⁵. At these
ranges a difference carries no information (10¹⁷ s minus 10⁻⁴³ s is 10¹⁷ s), so
the comparisons that mean anything are the logarithm of a ratio and the number
of decades between two spans.

*The Friedmann integral.* The early ages in the tables were integrated once,
offline, with `Ωr h² = 4.1834×10⁻⁵`, the photons of `T = 2.7255 K` plus
`Neff = 3.046` neutrino species. The crate integrates nothing at run time:
`universe` carries the results, as published numbers with an uncertainty from
moving `H₀` and `Ωm` over their own 1σ ranges, and says so (a cosmology solver
is not carried, below).

*The conversions.* `bp_convert_line` moves a calendar age between `bp`, `b2k`
and `ce` by adding or subtracting a whole number of years (50 between BP and
b2k, and 1950 minus the year between BP and a calendar year), so the standard
deviation is unchanged. A `radiocarbon-bp` age is refused: `Bp::uncalibrated`
asked for a calendar year gives `CalibrationRequired`, and the line gives
`HC_ERR_NO_DATA`. `Bp::on_measured_half_life` multiplies by 5730/5568, which is
not calibration, and keeps the age uncalibrated.

*Placement.* `timeline::place` takes a span since the Big Bang and says which
cosmic epoch, which cosmic event, which geological interval, which
archaeological period and which future era it falls in, with the uncertainty
carried through. The chronologies count from three origins (the Big Bang, the
present, 1950 CE); the conversion goes through the age of the universe, so
every "years ago" inherits its 0.020 Gyr, 6.3×10¹⁴ s, which is larger than the
whole Quaternary. A moment up to a century ahead is still placed in the
intervals that end at the present, because the chart cannot name a future
boundary and a century is its resolution at its young end.

**A worked example: the Planck time.** With `ħ` = 1.054 571 817×10⁻³⁴ J s
(exact), `G` = 6.674 30×10⁻¹¹ and `c` = 299 792 458 m/s (exact): `ħG` =
7.038 5×10⁻⁴⁵, `c⁵` = 2.421 6×10⁴² and the quotient 2.906 6×10⁻⁸⁷, whose square
root is 5.391 246×10⁻⁴⁴ s. CODATA 2022 prints 5.391 247(60)×10⁻⁴⁴: the seventh
figure differs by 7×10⁻⁵⁰, a tenth of the standard uncertainty, as it should.
That uncertainty is half of `G`'s relative 2.247×10⁻⁵, 1.124×10⁻⁵, times the
value, 6×10⁻⁵⁰. `hc_deep_convert(1, 0, "planck-time", "second")` writes the
converted value 5.391247e-44 with the standard deviation 6e-50 and `log10` of
the seconds −43.2683 with a standard deviation of 4.8×10⁻⁶, and `0` in the cell
that says the conversion is exact.

**A worked example: the age of the universe at matter–radiation equality.**
`H₀` = 67.66 km s⁻¹ Mpc⁻¹ is 67.66×10³ m s⁻¹ over 3.085 677 6×10²² m per
megaparsec, 2.192 7×10⁻¹⁸ s⁻¹, so `1/H₀` = 4.560 6×10¹⁷ s. With `h` = 0.6766,
`Ωr` = 4.1834×10⁻⁵/`h²` = 9.138 3×10⁻⁵ and `ΩΛ` = 0.688 81. At `z_eq` = 3387 the
scale factor is `a` = 2.951 6×10⁻⁴, and there `ΩΛ a⁴` is below 10⁻¹⁴ against
`Ωr` of 10⁻⁴: the dark-energy term can be dropped. With `u = Ωr + Ωm a′` the
integral is `∫ a′ da′/√u = (1/Ωm²) ∫ (u^{1/2} − Ωr u^{−1/2}) du`, which has the
closed form

```text
t(a) = (2 / (3 H₀ Ωm²)) [ (u^{3/2} − Ωr^{3/2}) − 3 Ωr (u^{1/2} − Ωr^{1/2}) ],  u = Ωr + Ωm a.
```

At `a` = 2.951 6×10⁻⁴, `u` = 1.8321×10⁻⁴ and the bracket is 5.1621×10⁻⁷; the
prefactor is 2/(3 × 2.192 7×10⁻¹⁸ × 0.096 783) = 3.141 4×10¹⁸ s, so `t` =
1.6216×10¹² s, or 51 386 Julian years. The crate carries 1.62161×10¹² s with a
standard deviation of 2.4×10¹⁰ s (`matter-radiation-equality`, "5.14×10⁴ yr").
The same formula at `z*` = 1089.80 gives 1.17333×10¹³ s, 371 807 years; the
crate carries 1.17333×10¹³ (`recombination`, "3.72×10⁵ yr"). Matter and
radiation are equal where `Ωm a = Ωr`, `1 + z = Ωm/Ωr` = 3404, which agrees with
Planck's 3387 ± 21 within one standard deviation.

**A worked example: a Holocene date.** The base of the Holocene is 11 700 b2k.
`hc_bp_convert(11700, 0, "b2k", "bp")` gives 11 650 and the label `11650 cal BP`;
`hc_bp_convert(11650, 0, "bp", "ce")` gives the astronomical year −9700 and the
label `9701 BCE`; `hc_bp_convert(3200, 50, "radiocarbon-bp", "ce")` is
`HC_ERR_NO_DATA`.

**A worked example: the age of the universe in Planck times.** 13.787 Gyr is
4.350 846×10¹⁷ s; divided by 5.391 247×10⁻⁴⁴ s that is 8.070 2×10⁶⁰, and its
base-ten logarithm is 60.9069. `hc_deep_compare(13.787, 0.020, "gigayear", 1, 0,
"planck-time")` writes those, with the standard deviation of the logarithm,
6.3×10⁻⁴, from the age's.

**A worked example: a black hole.** `GM☉` = 1.327 124 4×10²⁰ m³ s⁻². `τ = 5120 π
(GM☉)³ / (G ħ c⁴)`: `(GM☉)³` = 2.337 4×10⁶⁰, times 5120 π = 16 085 gives
3.759 7×10⁶⁴; `G ħ c⁴` = 5.685 4×10⁻¹¹ (`c⁴` = 8.077 6×10³³); the quotient is
6.612 9×10⁷⁴ s, or 2.0955×10⁶⁷ Julian years. `black_hole_lifetime(1.0)` is 2.095×10⁶⁷ years, with
the relative uncertainty of `G` alone, because the mass is written `GM☉/G` and
`G` is the only uncertain constant left in it. The formula assumes a
non-rotating, uncharged hole radiating massless particles only; it ignores the
absorption of the cosmic microwave background, by which a hole colder than its
surroundings grows, so the number is the time an isolated hole takes once the
universe is colder than it, and not the time from now.

## What is carried

- **`constants`**: the speed of light and the reduced Planck constant (exact),
  `G` (measured), and the Planck time, length, mass, energy and temperature,
  each with value, standard uncertainty, the figures printed and its source, from
  CODATA 2022 [codata2022].
- **`universe`**: eleven epochs tiling the span from the Big Bang to the present
  without gap or overlap (Planck, grand unification, inflationary, electroweak,
  quark, hadron, lepton, photon, dark ages, reionisation, galaxies) and nine
  events (neutrino decoupling, nucleosynthesis, matter–radiation equality,
  recombination, first stars, first galaxies, the Milky Way, the Solar System,
  the present).
- **`future`**: eight events and four eras, from the Sun's end to 10¹⁰⁰ years,
  and `black_hole_lifetime`.
- **`archaeology`**: the `Bp` type with the calibration tag, `b2k_to_bp`,
  `bp_to_b2k`, and eleven conventional periods of the Southwest Asian and
  European sequence, each naming its region.
- **`periods`**: the precession of the equinoxes, 25 772 ± 2 Julian years, and
  the galactic year, 230 ± 15 Myr (half the literature's 225 to 250 Myr range,
  widened because Gaia-era parameters favour the low end); both are marked as
  drifting, so a count of cycles over millions of years is not the period times
  the span.
- **The boundary**: `hc_place_years_ago` (a moment placed in every chronology),
  `hc_cosmic_events`, `hc_archaeological_periods`, `hc_future_events` (all with
  the 16 columns of the deep-time line), `hc_planck_units` (10 columns),
  `hc_bp_convert` (8), `hc_deep_convert` (13) and `hc_deep_compare` (12).

Not carried:

- **A cosmology solver.** The Friedmann integral ran once, offline; the crate
  carries its results and their inputs. Computing an age from another set of
  parameters is not yet done.
- **Radiocarbon calibration.** The curves of IntCal20, SHCal20 and Marine20 are
  large datasets with their own release cadence, none is carried, and
  approximating them would give dates that look authoritative. The conversion
  is refused. Not yet done.
- **The archaeological periods of any region but Southwest Asia and Europe.**
  No other region's sequence has been added; no source for one was read.
- **Eras of the future beyond Adams and Laughlin's**, and any prediction not
  in the table. A decade in the gaps between their eras belongs to none.
- **A time scale for the span between 10⁻⁴³ s and the first measured epoch.**
  Those epochs are conventional decades and say so.
- **The proper conversion of a span to a count of calendar days.** A span here
  is in seconds; the conversion to a fixed day is `hc-calendar`'s.

## Accuracy

The Planck time, length, mass and the energy and temperature rows were compared
with the NIST table of the 2022 adjustment when the constants were entered
(2026-09-26); the Planck time and length and mass were recomputed by hand above
from `ħ`, `G` and `c`: t_P 5.391 246×10⁻⁴⁴ s, l_P 1.616 255×10⁻³⁵ m, m_P
2.176 434×10⁻⁸ kg against CODATA's 5.391 247, 1.616 255 and 2.176 434. The
Planck energy is derived as `m_P c²` and is 1.956 081×10⁹ J, which is the
published 1.220 890×10¹⁹ GeV.

The Friedmann integral was reproduced on 2026-10-04 by numerical integration
(Simpson's rule over 400 000 intervals) and by the closed form above, with the
crate's parameters (`Ωr` = 4.1834×10⁻⁵/`h²`, 1 pc = 3.085 677 581×10¹⁶ m):

| Age | This integral | The crate | Relative difference |
| --- | --- | --- | --- |
| Matter–radiation equality, `z` = 3387 | 1.621 626×10¹² s | 1.62161×10¹² s | 1.0×10⁻⁵ |
| Recombination, `z*` = 1089.80 | 1.173 332×10¹³ s | 1.17333×10¹³ s | 1.7×10⁻⁶ |
| First galaxies, `z` = 14.32 | 9.0335×10¹⁵ s | 9.0338×10¹⁵ s | 3.3×10⁻⁵ |
| The present, `a` = 1 | 4.3505×10¹⁷ s (13.786 Gyr) | 4.350 846×10¹⁷ s (13.787 Gyr) | 7.8×10⁻⁵ |

Every difference is far inside the standard uncertainty the crate states for
the entry (1.5×10⁻² relative at equality, 1.0×10⁻² at recombination and 1.5×10⁻² at
`z` = 14.32). The present is the published age and the integral is the check
that the integration is right, as the module says. Taking `Ωr h²` from the
constants (the photon density of `T` = 2.7255 K from `kB`, `ħ` and `c`, times
1 + (7/8)(4/11)^{4/3} `Neff` = 1.6918) gives 4.1837×10⁻⁵, which moves the
equality age by 2.2×10⁻⁵ and the others less.

The black hole formula was computed by hand above and agrees with the crate's
2.095×10⁶⁷ years. Wikipedia's page for Hawking radiation prints the same
formula with a coefficient of "≈ 2.140×10⁶⁷ years (M/M☉)³", 2.1 % higher, with
the solar mass it uses not stated; the crate's figure follows from `GM☉` and
`G`.

The conversions of datum are exact integer arithmetic on the year numbers. Not
checked against a second implementation: the period values, the stellar and
particle numbers of the future table, and the conventional decades. They were
taken from the sources named in the tables, and the sources marked "not read"
below are cited from the crate's documentation.

## Sources

- [codata2022]: CODATA 2022, Mohr, Newell, Taylor and Tiesinga, with the NIST
  table `allascii.txt`; read 2026-09-26 for the constants (see the entry).
- [planck2018-vi]: Planck Collaboration, *Planck 2018 results. VI*, A&A 641,
  A6 (2020). Not read for this document: Table 2's `+BAO` column is in the PDF.
  The abstract page (read 2026-10-04) gives the other combination of the
  data, H₀ = 67.4 ± 0.5 and Ωm = 0.315 ± 0.007.
- Kolb and Turner, *The Early Universe* (1990), for the conventional epochs;
  Cyburt et al., Rev. Mod. Phys. 88, 015004 (2016), for nucleosynthesis; Bromm
  and Larson, ARA&A 42, 79 (2004); Carniani et al., Nature 633, 318 (2024);
  Xiang and Rix, Nature 603, 599 (2022); Connelly et al., Science 338, 651
  (2012): cited by the tables, not read for this document.
- Schröder and Connon Smith, MNRAS 386, 155 (2008); Adams and Laughlin, Rev. Mod.
  Phys. 69, 337 (1997); Super-Kamiokande Collaboration, Phys. Rev. D 102,
  112011 (2020); Hawking, Nature 248, 30 (1974); Page, Phys. Rev. D 13, 198
  (1976): cited by the tables, not read for this document.
- [wikipedia-before-present]: the BP datum, b2k and the Holocene's base as 9701
  BC and 11 650 BP; read 2026-10-03; secondary. Stuiver and Polach, Radiocarbon 19,
  355 (1977), for the three conventions of a radiocarbon age, and Reimer et al.,
  Radiocarbon 62, 725 (2020), for IntCal20: cited, not read. [walker2009], for
  the Holocene GSSP at 11 700 b2k, as the geologic document says.
- [wikipedia-hawking-radiation]: the evaporation formula and its coefficient;
  read 2026-10-04 as the page's source text; secondary.
- [capitaine2003]: the IAU 2006 precession, from which the 25 772-year period
  of the table comes; not read directly (see the entry).
- The galactic year: the literature range 225 to 250 Myr and Bland-Hawthorn and
  Gerhard, ARA&A 54 (2016) 529, as the table cites them; not read.

## Code

`crates/hc-deep-time/src/`: `magnitude.rs`, `constants.rs`, `universe.rs`,
`future.rs`, `archaeology.rs`, `periods.rs` and `timeline.rs`; the lines are
`crates/hyper-calendar/src/deep_time_lines.rs` and the exports are in the
`deep_time` group of `crates/hyper-calendar/src/exports.rs`, with
`crates/hyper-calendar-wasm/src/deep_time.rs` and
`crates/hyper-calendar-ffi/src/deep_time.rs`.

The tests that anchor them are in the same files. In `constants.rs`:
`the_planck_time_matches_the_current_codata_value`,
`every_planck_unit_is_known_to_about_one_part_in_ninety_thousand`,
`gravitation_carries_twice_the_relative_uncertainty_of_the_planck_units`,
`the_tabulated_planck_time_agrees_with_its_own_definition` and
`the_planck_energy_agrees_with_the_mass_times_c_squared`; in `universe.rs`:
`the_epochs_tile_cosmic_history_without_gaps_or_overlaps`,
`the_age_of_the_universe_is_thirteen_point_seven_eight_seven_gigayears`,
`the_universe_is_about_eight_times_ten_to_the_sixty_planck_times_old`,
`recombination_is_about_three_hundred_and_seventy_thousand_years_in` and
`matter_radiation_equality_precedes_recombination`; in `future.rs`:
`a_solar_mass_black_hole_evaporates_in_two_times_ten_to_the_sixty_seven_years`,
`the_tabulated_evaporation_times_match_the_formula`,
`the_evaporation_time_inherits_only_the_uncertainty_of_gravitation`,
`the_adams_and_laughlin_gaps_are_preserved` and
`the_proton_decay_entry_is_labelled_a_bound_and_not_a_prediction`; in
`archaeology.rs`: `the_present_of_before_present_is_nineteen_fifty`,
`astronomical_numbering_puts_year_zero_at_one_bce`,
`an_uncalibrated_radiocarbon_age_refuses_to_become_a_calendar_year`,
`the_ice_core_datum_runs_fifty_years_ahead_of_the_radiocarbon_one` and
`the_neolithic_opens_at_the_holocene_gssp`; in `periods.rs`:
`a_galactic_year_is_about_seven_quadrillion_seconds`,
`every_period_states_a_spread_and_whether_it_drifts` and
`the_earth_is_about_twenty_galactic_years_old`; and in the facade's
`deep_time_lines.rs`: `every_line_has_every_column`,
`every_row_carries_its_identifier_and_no_two_share_one` and
`the_periods_and_the_future_events_are_listed`. No test integrates the
Friedmann equation: the tests hold the published values, and the integration
above is the measurement behind them.
