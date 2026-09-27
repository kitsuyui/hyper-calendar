# Earth rotation: UT1 and its smoothed readings, the Earth Rotation Angle and Greenwich sidereal time

Backs `hc-astro::ut_variants` (UT2, UT1R and UT1S) and the rotation half
of `hc-astro::earth` (the Earth Rotation Angle and the mean and apparent
sidereal times). No calendar identifier is registered: these are readings
of the Earth's rotation, not calendars. UT1 itself, from a DUT1 series or
from the ΔT model, is `hc-astro::ut1`, and [time-scales.md](../time-scales.md)
places all of them beside TAI and UTC.

## What it is

Universal Time is the Earth's rotation read as a time of day. The
rotation is not uniform: tides, the atmosphere, the core and the melting
of ice all change it, by milliseconds over a day and by seconds over a
century. What a time service measures therefore depends on how much of
that variation it keeps, and each choice has its own name:

- **UT0** is the rotation as one observatory measured it from star
  transits. **UT1** corrects UT0 for polar motion, which moves the
  observer's meridian, and is the rotation itself; it is what the IERS
  publishes now, as `UT1 − UTC`. SOFA lists UT0 and UT2 as "specialist
  forms of universal time that take into account polar motion and known
  seasonal effects; no longer used" [sofa-ts].
- **UT2** is UT1 with a conventional seasonal variation, about ±30 ms over
  the year, taken out. The time services published it until atomic time
  replaced it; it survives in historical readings.
- **UT1R** and **UT1S** are UT1 with the zonal tides' effect taken out:
  the tides with periods under 35 days for UT1R, as the IAU's 18th General
  Assembly adopted it at Patras in 1982, and all of them, to the 18.6-year
  nodal tide, for UT1S. Past versions of the IERS Conventions defined
  both; the 2010 Conventions recommend exchanging only UT1 and the length
  of day [iers-tn36, ch. 8, §8.1].

The rotation is also an angle. Two conventions measure it:

- The **Earth Rotation Angle** (ERA) of the IAU 2000 framework is the
  angle between the Celestial and the Terrestrial Intermediate Origins
  along the equator. It is linear in UT1 by definition; in that framework
  it is what UT1 *is* [iers-tn36, ch. 5, §5.5.3].
- **Greenwich mean sidereal time** (GMST) measures the same rotation from
  the equinox, which precession moves. The IAU 1982 expression is one
  polynomial in UT1 [meeus1998, (12.4)]; the IAU 2006 expression is the
  ERA plus the accumulated precession in right ascension, a polynomial in
  TT [iers-tn36, ch. 5, §5.5.7]. *Apparent* sidereal time adds nutation,
  the equation of the equinoxes.

## How it works

**UT2.** UT2 − UT1 = 0.022 sin 2π*T* − 0.012 cos 2π*T* − 0.006 sin 4π*T*
+ 0.007 cos 4π*T* seconds, with *T* = 2000.000 + (MJD − 51 544.03) /
365.2422 the Besselian year [usno-eo-values]. Only the fraction of *T*
matters.

**UT1R and UT1S.** Table 8.1 of the IERS Conventions gives 62 zonal tide
terms. Each has five integer multipliers *a*ᵢⱼ of the Delaunay arguments
*l*, *l*′, *F*, *D* and Ω (ch. 5, eq. 5.43), and sine and cosine
coefficients *B*ᵢ and *C*ᵢ. The tides' effect is
δUT1 = Σ *B*ᵢ sin ξᵢ + *C*ᵢ cos ξᵢ with ξᵢ = Σⱼ *a*ᵢⱼ αⱼ, and
"these corrections should be subtracted from the observed UT1 − UTC".
UT1R sums
the 41 terms with periods under 35 days, UT1S all 62 [iers-tn36, ch. 8].
The table is a model, and the model is part of the name: it is the sum of
Yoder, Williams and Parke's 1981 elastic tide, Wahr and Bergen's 1986
inelastic body tide and Kantha et al.'s 1998 ocean tide, and differs from
the model of the Conventions 2003 by about 6 µs at the fortnightly term,
out of a fortnightly amplitude of about 785 µs [iers-tn36, ch. 8, §8.1].
The IAU's 1982 definition of UT1R names Yoder et al.'s own tables, which
the Conventions report to hold four known errors.

**The Earth Rotation Angle.** ERA = 2π(0.779 057 273 264 0 +
1.002 737 811 911 354 48 *T*u), *T*u = JD(UT1) − 2 451 545.0 [iers-tn36,
ch. 5, eq. 5.14]. Equation 5.15 writes the same thing as the fraction of
the UT1 day plus the constant plus the excess over one turn a day,
0.002 737 811 911 354 48 *T*u, which keeps the whole turns out of the
product.

**GMST, IAU 2006.** GMST = ERA(UT1) + 0.014 506″ + 4612.156 534″ *t* +
1.391 581 7″ *t*² − 0.000 000 44″ *t*³ − 0.000 029 956″ *t*⁴ −
0.000 000 036 8″ *t*⁵, *t* in Julian centuries of TT since J2000.0
[iers-tn36, ch. 5, eq. 5.32]. The two parts take different time scales.

**GMST, IAU 1982.** θ = 280.460 618 37° + 360.985 647 366 29° *d* +
0.000 387 933° *T*² − *T*³ / 38 710 000, *d* the days and *T* the Julian
centuries of UT1 since J2000.0. That is ERFA's `eraGmst82`, GMST − UT1 =
24 110.548 41 s + 8 640 184.812 866 s *T* + 0.093 104 s *T*² −
6.2 × 10⁻⁶ s *T*³ at 0 h, with the UT1 day added and the constant moved
twelve hours to the Julian Date's noon, put in degrees at 240 s to the
degree [erfa-gmst82]. Meeus prints the same polynomial as his (12.4)
[meeus1998], not read here.

**Worked example: the Earth Rotation Angle.** Take JD 2 454 388.5 UT1,
00:00 UT1 on 15 October 2007, the date of ERFA's test [erfa].

1. *T*u = 2 454 388.5 − 2 451 545.0 = 2 843.5 days.
2. The fraction of the UT1 day is 0.5; the constant is 0.779 057 273 264 0;
   the excess is 0.002 737 811 911 354 48 × 2 843.5 = 7.784 968 169 936.
3. Their sum is 9.064 025 443 200 turns. Dropping the whole turns leaves
   0.064 025 443 200, which is 23.049 159 55° or 0.402 283 724 0 rad:
   ERFA's value for `eraEra00` is 0.402 283 724 002 815 810 2 rad.

**Worked example: UT2.** A quarter of a Besselian year after the formula's
epoch, *T* = 2000.25, so 2π*T* is a right angle: sin 2π*T* = 1,
cos 2π*T* = 0, sin 4π*T* = 0 and cos 4π*T* = −1. UT2 − UT1 =
0.022 − 0.007 = +0.015 s. At *T* = 2000.000, MJD 51 544.03, only the
cosines are left: −0.012 + 0.007 = −0.005 s.

**Worked example: the two GMSTs.** ERFA's test values at JD 2 453 736.5,
with UT1 and TT taken equal, are 1.754 174 971 870 091 203 rad for
`eraGmst06` and 1.754 174 981 860 675 096 rad for `eraGmst82` [erfa]. The
difference, 9.99 × 10⁻⁹ rad, is 9.99 × 10⁻⁹ × 86 400 / 2π = 0.137 ms of
sidereal time: the two conventions agree that closely on 2006-01-01, and
part slowly as their precession models diverge.

## What is carried

In `hc-astro::ut_variants`:

- `ut2` and `ut2_minus_ut1`, and `ut2_besselian_year`, the USNO's *T*.
- `ut1r_iers2010`, `ut1r_minus_ut1_iers2010`, `ut1s_iers2010` and
  `ut1s_minus_ut1_iers2010`, with the model in the name (policy §5).
- `ZONAL_TIDES_IERS2010`, the 62 rows of Table 8.1 in their UT1 columns;
  `delaunay_arguments`; `zonal_tide_ut1_effect`; and
  `UT1R_PERIOD_LIMIT_DAYS`, the 35 days.

Each function takes the caller's UT1 and returns the smoothed reading, so
that a historical value labelled with one of these names can be put back on
UT1. The Delaunay arguments are in centuries of TDB, for which the
Conventions allow TT, and the TT is `UT1 + ΔT`; a minute's error in ΔT
moves the largest fortnightly term, 0.786 ms over 13.66 days, by about
0.25 µs, and the whole table by under 1 µs.

In `hc-astro::earth`:

- `earth_rotation_angle`, the IAU 2000 ERA, from UT1.
- `mean_sidereal_time_iau2006` and `mean_sidereal_time_iau2006_at`, the
  IAU 2006 GMST: the first takes TT as `UT1 + ΔT`, the second takes it
  from the caller.
- `mean_sidereal_time_iau1982`, the IAU 1982 GMST, and
  `apparent_sidereal_time_iau1982`, that plus Meeus's equation of the
  equinoxes Δψ cos ε from his abridged nutation. Every rise, set and
  transit in the crate uses this pair.

**Not carried**, with the reason:

- *UT0*, which needs the observatory's own position against the moving
  pole: an observation, not a rule.
- *A UT2 on other coefficients.* Schlyter gives the seasonal correction as
  +0.022 sin 2π*t* − 0.017 cos 2π*t* − 0.007 sin 4π*t* + 0.006 cos 4π*t*
  seconds, *t* the fraction of the year from 1 January
  [schlyter-time-scales]. That is a different formula from the USNO's,
  and it may be an earlier definition, but the page gives no date or
  authority and the BIH's own publications were not read. Under policy §5
  it would be a separately named function; it is not added until a
  primary source dates it.
- *UT1R from Yoder et al.'s 1981 tables*, which the IAU's 1982 definition
  names. Yoder et al. (1981), Wahr and Bergen (1986), Kantha et al. (1998)
  and the IAU resolution (Transactions of the IAU, vol. XVIIIB, 1983,
  pp. 238–240) are all cited through [iers-tn36], ch. 8, and none was read.
  A UT1R from the 1981 tables would be its own function.
- *The diurnal and semi-diurnal ocean-tide terms* of the Conventions'
  §8.2 (`ORTHO_EOP.F`), tens of microseconds in UT1, which are a
  correction to UT1 itself and not part of any named smoothing.
- *The IAU 2006/2000A apparent sidereal time*, which needs the full
  nutation series; the apparent sidereal time here is the IAU 1982 one
  with Meeus's four-term nutation.
- *A DUT1 series.* UT1 is observational, and `hc-astro::ut1` reads the
  series the caller supplies rather than shipping one.

## Accuracy

The ERA and the GMSTs are definitions, so the checks are ERFA's test
values; the UT variants are formulas and a table, checked by hand and
against the IERS routine's test case.

| Check | Test | Result |
| --- | --- | --- |
| ERA at JD 2 454 388.5 is ERFA's `eraEra00` value | `the_earth_rotation_angle_matches_erfa` | 10⁻⁹ degree |
| ERA at J2000.0 is the constant, and a day later it has turned once and the excess | `the_earth_rotation_angle_starts_at_its_constant_and_turns_at_its_rate` | 10⁻⁹ degree |
| IAU 2006 GMST at JD 2 453 736.5 is ERFA's `eraGmst06` value; with modelled TT it moves the predicted 95 µas | `the_iau2006_mean_sidereal_time_matches_erfa` | 10⁻⁹ degree |
| IAU 1982 GMST is ERFA's `eraGmst82` value | `the_meeus_mean_sidereal_time_is_the_iau1982_convention` | 0.7 µs of time |
| The two GMSTs differ by 0.14 ms on 2006-01-01 | `the_two_mean_sidereal_times_differ_by_a_fraction_of_a_millisecond` | as ERFA's values differ |
| Apparent sidereal time against ERFA's `eraGst94` | `the_apparent_sidereal_time_agrees_with_erfa_gst94_within_the_abridged_nutation` | 0.5″, the abridged nutation's bound |
| UT2 − UT1 at *T* = 2000.000 and 2000.25, by hand | `ut2_at_the_formulas_epoch_is_the_sum_of_its_cosine_terms`, `ut2_a_quarter_year_on_is_the_first_sine_less_the_second_cosine` | 10⁻⁹ s |
| The whole of Table 8.1 reproduces `RG_ZONT2.F`'s test case | `the_whole_table_reproduces_the_iers_test_case` | 10⁻¹² s |
| Each row's period follows from its multipliers | `each_period_follows_from_its_multipliers` | the table's rounding |

No published value of UT2 − UT1 was found to anchor UT2 against: the
USNO page gives the formula and no values, and the BIH's tabulations were
not read. UT2 is therefore checked against the formula by hand only. The
same holds for UT1R and UT1S beyond the IERS test case, which sums all 62
terms and so pins UT1S and not the 35-day split; the split is checked by
counting the table's 41 short rows.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [iers-tn36] | ch. 5: the ERA (eqs 5.14–5.15), the IAU 2006 GMST (eq. 5.32), the Delaunay arguments (eq. 5.43); ch. 8: Table 8.1, UT1R and UT1S, the tide models, the 6 µs against the 2003 model, the IAU 1982 footnote | Yes, 2026-09-26; ch. 8 again 2026-09-27 |
| [iers-rg-zont2] | The test case of the whole table | The header only, 2026-09-26 |
| [usno-eo-values] | The UT2 formula | Yes, 2026-09-26 and 2026-09-27 |
| [sofa-ts] | UT0 and UT2 as "no longer used" | Yes, 2026-09-27 |
| [erfa] | The test values of `eraEra00`, `eraGmst06`, `eraGmst82` and `eraGst94` | The test file only, 2026-09-26 |
| [erfa-gmst82] | The IAU 1982 GMST: 24 110.548 41 s + 8 640 184.812 866 s *T* + 0.093 104 s *T*² − 6.2 × 10⁻⁶ s *T*³ in UT1 centuries, the coefficients `mean_sidereal_time_iau1982` carries in degrees | Yes, `gmst82.c`, 2026-09-27 |
| [meeus1998] | Meeus's form (12.4) of the IAU 1982 GMST, the abridged nutation and the equation of the equinoxes | Not read for this document; `hc-astro::earth` cites it |
| [schlyter-time-scales] | The other UT2 coefficients | Yes, 2026-09-27; secondary |

Capitaine et al. (2000), for the ERA, and Capitaine, Wallace and Chapront
(2003), for the IAU 2006 GMST, are cited through [iers-tn36] and were not
read; nor was Aoki et al. (1982), for the IAU 1982 GMST, which Meeus
follows. Yoder et al. (1981), Wahr and Bergen (1986), Kantha et al. (1998),
Simon et al. (1994) for the Delaunay arguments, and the IAU's 1982
resolution were not read either.

## Code

`crates/hc-astro/src/ut_variants.rs` and `crates/hc-astro/src/earth.rs`.
The tests that anchor them: `the_earth_rotation_angle_matches_erfa`,
`the_iau2006_mean_sidereal_time_matches_erfa`,
`the_meeus_mean_sidereal_time_is_the_iau1982_convention`,
`the_two_mean_sidereal_times_differ_by_a_fraction_of_a_millisecond`,
`ut2_at_the_formulas_epoch_is_the_sum_of_its_cosine_terms`,
`ut2_a_quarter_year_on_is_the_first_sine_less_the_second_cosine` and
`the_whole_table_reproduces_the_iers_test_case`.
