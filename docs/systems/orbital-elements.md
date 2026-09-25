# The Milankovitch orbital elements and daily insolation, from Berger's 1978 series

Backs the crate `hc-orbital`.

## What it is

Earth's orbit is not the fixed ellipse a calendar assumes. Pulled by the
other planets, its eccentricity swings between nearly zero and about 0.06
with periods near 100 000 and 400 000 years; the axis nods between about
22.0° and 24.5° of obliquity every 41 000 years; and the direction of
perihelion turns relative to the equinoxes, so that the season in which
Earth is nearest the Sun comes round every 19 000 to 23 000 years. These
three cycles set how much sunlight each latitude receives in each season,
and they are the pacemaker of the ice ages: Milankovitch's hypothesis of
the 1920s, confirmed in the deep-sea record by Hays, Imbrie and Shackleton
in 1976 [berger1978, as summarised in its title and its users].

Palaeoclimatology works with four numbers at an epoch:

| Symbol | Name | Now |
| --- | --- | --- |
| *e* | eccentricity | 0.0167 |
| ε | obliquity | 23.45° |
| ϖ | longitude of perihelion from the moving vernal equinox | 102° |
| *e* sin ϖ | climatic precession | 0.0164 |

and with the daily mean insolation at 65° N at the June solstice, the curve
the theory is usually told with, because that is the latitude and season
whose sunlight decides whether the winter's snow survives.

The solution the field standardised on is André Berger's 1978 trigonometric
expansion [berger1978], built from Bretagnon's planetary theory: three
tables of terms whose sums give *e*, ε and ϖ at any time within a million
years of the present. PMIP, the palaeoclimate model intercomparison, has
prescribed it for its mid-Holocene and Last Glacial Maximum experiments
through every phase since [braconnot2012]; the NCAR and NASA GISS climate
models carry it verbatim [cesm-shr-orb, giss-orbpar]. It was superseded
for longer spans by Berger and Loutre's 1991 solution [bergerloutre1991]
and then by Laskar's numerical integrations, La2004 [laskar2004], which
reach fifty million years; but for the last few hundred thousand years the
1978 series remains the reference the experiments are defined against, and
it is small — 144 terms — where La2004 is a table by the megabyte.

## How it works

**Time.** *t* is in years from 1950 CE, negative in the past. "Present"
in this document and in the crate is 1950, the series' own epoch, which is
also the radiocarbon BP datum; 21 000 years before present is *t* =
−21 000 and the year −19 050 in astronomical numbering.

**The three sums.** Each table lists terms of amplitude *A*ᵢ, mean rate
*B*ᵢ in arcseconds per year and phase *C*ᵢ in degrees, and every term is
*A*ᵢ × trig(*B*ᵢ *t* + *C*ᵢ). The author's program combines them as follows
[berger1978deposit]:

```text
e sin π = Σ Aᵢ sin(Bᵢ t + Cᵢ)      e cos π = Σ Aᵢ cos(Bᵢ t + Cᵢ)     (19 terms)
e       = √((e sin π)² + (e cos π)²)
π       = atan2(e sin π, e cos π)
ψ       = ζ + ψ̄ t + Σ Aᵢ sin(Bᵢ t + Cᵢ) / 3600″                     (78 terms)
ϖ       = π + ψ, reduced to [0°, 360°)
ε       = ε* + Σ Aᵢ cos(Bᵢ t + Cᵢ) / 3600″                          (47 terms)
```

with ε* = 23.320556°, ζ = 3.392506° and ψ̄ = 50.439273″ per year. The
19-term table gives the *eccentricity vector*, whose length is *e* and whose
angle π is the longitude of perihelion from the fixed equinox of the
reference frame; the 78-term table is the general precession in longitude
ψ, the motion of the equinox itself; ϖ = π + ψ is the longitude of
perihelion from the *moving* equinox, the one that matters for the seasons.
The climatic precession is *e* sin ϖ.

**The 180°.** ϖ is heliocentric: the direction from the Sun to Earth's
perihelion, about 102° at present. The Sun's *geocentric* longitude when
Earth is at perihelion is ϖ + 180°, about 282°, in early January; that is
what the insolation formula needs, and what some tables print as "the
longitude of perihelion" — PMIP's "perihelion" and GISS's `PERI` are ϖ +
180°, so both publish "perihelion − 180" to give Berger's ϖ. The confusion
is old enough that NCAR's code cites an appendix explaining it
[cesm-shr-orb].

**Daily insolation.** The mean irradiance at the top of the atmosphere over
one day at latitude φ, when the Sun's true longitude is λ, for a solar
constant *S*₀ [berger1978deposit, subroutine `DAYINS`]:

```text
ρ      = (1 − e²) / (1 + e cos(λ − (ϖ + 180°)))    the Earth–Sun distance, AU
S      = S₀ / (π ρ²)
sin δ  = sin ε sin λ                                the Sun's declination
H₀     = arccos(−tan φ tan δ)                       the half day arc
W      = S (H₀ sin φ sin δ + cos φ cos δ sin H₀)    W m⁻²
```

with *W* = *S* π sin φ sin δ in polar day and 0 in polar night. Berger's
"mid-month" values are taken at λ = 0°, 30°, …, 330° for March, April, …,
February, so mid-June is the solstice, λ = 90°, and mid-July is 120°.

**Worked example: 21 000 years before present**, the Last Glacial Maximum,
*t* = −21 000. The first term of each table, to show the shape, then the
sums (which the reader can reproduce from `INSOL.IN`):

- Eccentricity, term 5 (the largest): *A* = 0.01860798, *B* = 4.2072050″/yr,
  *C* = 28.620089°. The argument is 4.2072050 × (−21 000) / 3600 + 28.620089
  = −24.542 + 28.620 = 4.078°; the term contributes 0.01860798 × sin 4.078°
  = 0.001323 to *e* sin π. Over all 19 terms, *e* sin π = 0.0126773 and
  *e* cos π = 0.0141439, so *e* = 0.0189938 and π = atan2(0.0126773,
  0.0141439) = 41.870°.
- Precession, term 1: *A* = 7391.02″, *B* = 31.609974″/yr, *C* = 251.9025°.
  The argument is −184.392 + 251.903 = 67.511°; the term is 7391.02 ×
  sin 67.511° = 6829″ = 1.897°. The secular part is ψ̄ *t* = 50.439273 ×
  (−21 000)/3600 = −294.229°; the 78 periodic terms sum to +3.391°; so ψ =
  3.393 − 294.229 + 3.391 = −287.445°, and ϖ = 41.870 − 287.445 = −245.575°
  = **114.425°** reduced.
- Obliquity, term 1: *A* = −2462.22″, same *B* and *C* as the precession's
  first term (it is the same 41 000-year nutation of the ecliptic), so the
  same 67.511° and −2462.22 × cos 67.511° = −941.8″ = −0.2616°. All 47 terms
  sum to −0.3715°, so ε = 23.320556 − 0.3715 = **22.949°**.
- Climatic precession: *e* sin ϖ = 0.0189938 × sin 114.425° = **0.01729**.

PMIP's Last Glacial Maximum is defined by exactly these: *e* = 0.018994,
ε = 22.949°, perihelion − 180 = 114.42° [braconnot2012]. Then the June
solstice at 65° N with *S*₀ = 1360 W m⁻²: ϖ + 180 = 294.425°, so λ − (ϖ +
180°) = −204.425°, cos = −0.91050, ρ = (1 − 0.0189938²)/(1 − 0.0189938 ×
0.91050) = 1.01723, *S* = 1360/(π × 1.01723²) = 418.36; sin δ = sin ε =
0.38993 (λ = 90°), δ = 22.949°; sin φ sin δ = 0.35338, cos φ cos δ =
0.38917; H₀ = arccos(−0.35338/0.38917) = 155.236° = 2.70937 rad; *W* =
418.36 × (2.70937 × 0.35338 + 0.38917 × sin 155.236°) = **468.75 W m⁻²**,
against 477.63 at 1950 and 526.53 at the early-Holocene peak, 11 000 BP.

## What is carried

- **`elements_at(years_before_present)`**, returning *e*, ε in degrees
  (radians on request), ϖ in degrees in [0, 360) and *e* sin ϖ, each an
  `Uncertain` whose `std_dev` is a *spread* — the measured disagreement
  with the 1991 solution described under Accuracy, in three tiers by
  distance from 1950 — and not a Gaussian width. ϖ's spread is
  asin(spread of *e* sin ϖ / *e*), or 180° when *e* is smaller than that
  spread, since the direction of a vector shorter than its error bar is
  unknown; *e* falls to 0.0006 within the span.
- **`EPOCH_YEAR`** = 1950, **`VALID_SPAN`** = ±1 000 000 years, and
  `years_before_present_from_year` from a calendar year in astronomical
  numbering. Outside the span the crate returns `OutsideValidSpan` (ADR
  0006): a trigonometric fit past its span does not fade, it lies. The
  bound is the one the author's program, GISS and NCAR all state; NOAA's
  readme prefers the 1991 solution beyond 800 kyr and the tiers say so.
- **`daily_insolation(elements, latitude, solar longitude, solar
  constant)`** and **`insolation_65n_june`**, with the solar constant a
  parameter and the three values the reference tables used as named
  constants: 1.95 cal cm⁻² min⁻¹ = 1359.8 W m⁻² for the 1978 tables,
  1360 for the 1991 tables [noaa-insolation], 1368 in the author's 2014
  program [berger1978deposit].
- **The three tables**, generated from the author's `INSOL.IN` rather than
  typed, with each term's number in the paper kept beside it.
- **Not carried:**
  - *The paper's own accuracy statement.* The paper could not be read (the
    publisher's site refused the request); the spreads are measured here
    instead, as the next section says. Reading it would let the claimed
    precision be quoted beside the measured one.
  - *Calendar dates.* Berger's program turns a month and day into a solar
    longitude through the mean anomaly and a 365-day year, which is not a
    calendar and can differ from one by a day. Insolation is asked for at
    a longitude; `hc-astro` gives the present-day date of one.
  - *Berger & Loutre 1991 and La2004.* Each would be a second named
    solution beside this one (policy §5), not a parameter or a replacement.
    La2004 is the modern reference and is distributed as tables (nominal
    −50 to +20 Myr) [laskar2004]; carrying it means vendoring megabytes,
    with the download named and dated, and a `VALID_SPAN` of its own.
  - *The short-term obliquity.* `hc-astro` carries Laskar's 1986 polynomial
    (Meeus 22.3), good to 0.01″ over a thousand years and diverging beyond
    ten thousand; this series is coarse now and meaningful for a million
    years. They agree at 1950 to 0.001°. For a date, use `hc-astro`.

## Accuracy

Three references, three kinds of check.

**Transcription.** NASA GISS evaluates the same series with its program
`ORBPAR` and publishes the result for every year 0–2100 CE to seven figures
[giss-orbpar]. Agreement to the last printed digit — *e* to 10⁻⁷, ε to
10⁻⁶°, ϖ to 10⁻⁵° at 1, 1000, 1950, 2000 and 2100 CE — shows the tables
were transcribed correctly and the sums combined as the author combines
them. Test: `the_giss_transcription_of_the_same_series_is_reproduced_to_seven_figures`.

**The author's own tables of this solution.** NOAA's `bein1.dat`
[noaa-insolation] prints the 1978 solution at every thousand years from 0
to 100 kyr BP, *e* to six decimals, ϖ to 0.01°, ε to 0.001°, *e* sin ϖ to
five decimals, and mid-month insolation at every 10° of latitude in whole
langleys per day. Over all 101 epochs the largest differences are 5 × 10⁻⁷
in *e*, 0.005° in ϖ, 0.0005° in ε and 5 × 10⁻⁶ in *e* sin ϖ — the tables'
own rounding — and the June insolation at 60° N and 70° N agrees to
±0.2 W m⁻², the rounding of a whole langley (0.4843 W m⁻²). Tests:
`the_authors_own_1978_tables_are_reproduced_to_their_printed_digits`,
`the_authors_own_1978_insolation_tables_are_reproduced_within_their_rounding`.
PMIP's mid-Holocene and Last Glacial Maximum values are the same solution
at 6 and 21 ka and match to their printed digits:
`the_pmip_experiment_epochs_match`.

**Drift from the later solution.** The 1978 series against Berger and
Loutre's 1991 solution, read as NOAA's `orbit91` at 1 kyr steps
[noaa-insolation], gives the spreads the crate carries. Largest absolute
difference over the span from 1950 back to:

| Back to | *e* | ε | *e* sin ϖ | Carried as |
| --- | --- | --- | --- | --- |
| 100 kyr | 0.00178 | 0.0415° | 0.00237 | 0.002, 0.05°, 0.0025 |
| 800 kyr | 0.00804 | 0.195° | 0.00762 | 0.009, 0.20°, 0.008 |
| 1 000 kyr | 0.00904 | 0.262° | 0.0141 | 0.010, 0.27°, 0.015 |

At 1950 itself the two solutions differ by 0.0005 in *e* and 0.67° in ϖ,
and NOAA's readme prefers the 1978 one for that epoch; at 21 ka they differ
by 0.0004, 0.040° and 0.44°. ϖ is not tabulated because it is
ill-conditioned when *e* is small: the largest raw difference in ϖ over
500 kyr is 77°, at an epoch where *e* ≈ 0.002. Beyond 1 Myr the 1978
series breaks down as expected of a truncated expansion — by 1.5 Myr ϖ is
54° off and ε 0.45°, by 2 Myr ϖ is 149° off — which is why the crate stops
at 1 Myr and the doc comments say the 1991 solution is preferred past 800
kyr. The future half of the span is unmeasured, since `orbit91` stops at
1950, and is given the past's figures. The 65° N mid-July insolation of the
1991 table agrees with this series to under 1 W m⁻² over the last 100 kyr:
`the_1991_tables_65n_july_agree_to_under_one_watt_over_the_last_100_kyr`.

**Present day against the short-term models.** ε at 1950 is 23.4463°
against 23.4457° from Meeus's polynomial; *e* at 2000 is 0.016704 against
Meeus's J2000 0.016709; ϖ at 2000 is 102.90° against the J2000 102.94°.
Test: `the_present_day_obliquity_and_eccentricity_match_the_short_term_models`.

**The early-Holocene peak.** The maximum of 65° N June insolation over the
last 30 kyr is 526.5 W m⁻² at 11 100 BP (*S*₀ = 1360), 49 W m⁻² above
1950; the literature's "about 11 000 years ago, some 8–10 % above today"
is this. Test: `the_65n_june_maximum_of_the_last_30_kyr_is_in_the_early_holocene`.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [berger1978] | The solution and its formulation; cited as the source of every value | No: the publisher's site refused the request on 2026-09-25 |
| [berger1978deposit] | The three coefficient tables (`INSOL.IN`), the constants, the sign of time, the combination of the sums, the 180° and the insolation subroutine (`insol14.f`) | Yes, 2026-09-25, from Zenodo, CC BY 4.0 |
| [noaa-insolation] | The 1978 solution's own tabulated elements and insolations (`bein1.dat`), the solar constants, the 1991 solution used to measure drift (`orbit91`), the 800 kyr preference | Yes, 2026-09-25 |
| [giss-orbpar] | The transcription check to seven figures; the "approximately 1 million years" statement | Yes, 2026-09-25 |
| [cesm-shr-orb] | The conventions (time sign, the 180°, the ±1 Myr bound) in an independent transcription; no code taken | Yes, 2026-09-25 |
| [braconnot2012] | The PMIP 6 ka and 21 ka values | Not read; the values were read on the PMIP design pages |
| [bergerloutre1991] | The later solution, as the reference for the spreads | Not read; read as `orbit91` |
| [laskar2004] | Named as the solution that would replace this one for longer spans | Not read |
| [meeus1998] | The present-day obliquity, eccentricity and longitude of perihelion (22.3, 31.A) | Yes |

The primary text is the one gap. Its tables are the author's and were read
from the author; its accuracy claim is the one statement this document
cannot quote.

## Code

`crates/hc-orbital/src/series.rs` holds the tables, `elements.rs` the
sums, the epoch, the span and the spreads, `insolation.rs` the daily
formula. Anchors:
`the_giss_transcription_of_the_same_series_is_reproduced_to_seven_figures`,
`the_authors_own_1978_tables_are_reproduced_to_their_printed_digits`,
`the_pmip_experiment_epochs_match`,
`a_million_years_is_answered_and_a_year_more_is_refused`,
`the_authors_own_1978_insolation_tables_are_reproduced_within_their_rounding`,
`the_65n_june_maximum_of_the_last_30_kyr_is_in_the_early_holocene`. The
facade test `the_orbital_epoch_is_the_radiocarbon_bp_datum` ties
`EPOCH_YEAR` to `hc-deep-time`'s `BP_DATUM_YEAR`.
