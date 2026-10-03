# Uncertain time: significant figures, error bars, fuzzy instants and EDTF

Backs the crate `hc-uncertainty` and its five modules: `sig_figs`
(`Significant`, `MAX_FIGURES`), `quantity` (`Uncertain`), `interval`
(`DurationInterval`), `fuzzy` (`FuzzyInstant`, `Support`, `AllenRelation`,
`RelationSet`) and `edtf` (`EdtfValue`, `EdtfDate`, `EdtfEndpoint`,
`EdtfSetMember`, `EdtfQualifier`, `EdtfPrecision`). It is reached as
`hyper_calendar::hc_uncertainty` under the facade's `uncertainty` feature,
and `FuzzyInstant` and `Uncertain` are in the facade's prelude. No calendar
identifier is registered, and no WebAssembly or C export calls `Significant`
or the EDTF parser directly. The crate reaches those boundaries through
`hc-deep-time`, `hc-orbital` and `hc-relativity`, whose `deep-time`,
`orbital` and `relativity` bundles carry it.

## What it is

Almost nothing outside a laboratory has an exact timestamp. A charter is
dated "in the third year of the king", a sample is `3200 ± 50` years before
present, a catalogue says `1667 or 1668`, and the age of the universe is
three digits and not eleven. This document covers the five vocabularies the
crate uses to hold such a value without inventing the missing digits.

**Significant figures.** A number written to *n* figures claims *n* digits
and no more. Reducing a number to fewer digits follows the NIST rules: a
discarded part beginning below 5 leaves the last kept digit, one beginning
with 5 and followed by a nonzero digit raises it, and one that is exactly 5
raises it only if it is odd [nist-sp811-b7]. For arithmetic on such numbers
the textbook rules are that a product keeps the smaller figure count and a
sum ends at the coarser last place. They are guidelines, and they do not
ensure that the implied uncertainty matches the true one
[wikipedia-significant-figures].

**An error bar.** A measurement is a value and a standard deviation. Through
a function the standard deviation is carried to first order, valid only
where the linear form of the function fits
[wikipedia-propagation-of-uncertainty].
Two independent measurements of one quantity are merged by weighting each by
the reciprocal of its variance [wikipedia-inverse-variance-weighting]. How
many digits of the bar to print is a convention, and the conventions differ:
the Particle Data Group's depends on the leading digits of the bar
[pdg-rounding-quoted-errors-45, npae-kyiv-error-bars], and the GUM's, as a
search result quotes it and as the page itself was not read, is at most two
figures [jcgm100-gum-7-2-6].

**An interval.** "Somewhere in `[lo, hi]`", with no preference inside. The
arithmetic of such ranges encloses every possible result and is not tight
when a variable occurs twice [wikipedia-interval-arithmetic].

**A vague instant, and Allen's relations.** A date such as "in 1066", "between
1180 and 1185" or "before 1500" is a set of instants. Allen's algebra names
the thirteen ways two proper intervals can stand to each other: before,
meets, overlaps, starts, during, finishes, equals and the converses of the
first six [wikipedia-allen-interval-algebra, allen1983].

**EDTF.** The Library of Congress's Extended Date/Time Format is the
interchange syntax for a date that is not fully known. Its specification of
4 February 2019 defines three levels: Level 0 (dates, dates with a time of
day, intervals of two dates), Level 1 (qualifiers, unspecified digits, open
and unknown interval ends, negative years, seasons) and Level 2 (exponential
years, significant digits, sub-year groupings, sets, finer qualification)
[loc-edtf-2019]. ISO 8601-2:2019 specifies an EDTF profile, "virtually
identical" to the Library of Congress's [wikipedia-iso-8601]. The
specification requires the extended format, with hyphens and colons, and
excludes ISO 8601's basic format [loc-edtf-2019].

## How it works

### Significant figures

`Significant` is a value and a figure count from 1 to 17. 17 is
`MAX_FIGURES`: the value is exact, and prints as the shortest numeral that
reads back as the same `f64`.

- **Rounding** works on the shortest decimal numeral that identifies the
  `f64` (`{:e}`), not on its binary expansion, because "exactly one half" is
  a statement about decimal digits. The three NIST rules apply to that
  numeral, so `2.675` to three figures is `2.68` (the 7 is odd, and the
  tie raises it) although the double `2.675` lies just below the tie
  [nist-sp811-b7].
- **A decade crossing** keeps the claim: `9.96` to two figures is `10`, with
  decimal exponent 1 and last place 10⁰, and prints `10`, not `10.0`.
- **Printing** pads with zeros to the claimed count (`1.0` to four figures is
  `1.000`). Plain decimal is used when it shows only claimed digits or a
  zero after the point. Scientific notation is used when the decimal
  exponent is at least the figure count or below −4, so `13.8e9` to three
  figures is `1.38e10`, never `13800000000`, and `1000` to three is `1.00e3`.
  Scientific notation is the form in which a trailing zero is unambiguous
  [wikipedia-significant-figures].
- **Arithmetic.** A product or quotient takes the smaller figure count. A sum
  or difference takes the coarser of the two last significant places and
  recomputes the count from that place and the magnitude of the answer, so
  `100.0 + 0.001` stays at the tenths and `1.0000 − 0.9999` keeps one
  figure. An integer power keeps the count. These are the rules of
  [wikipedia-significant-figures]: `1.234 × 2` is `2` and `1.234 + 2` is `3`,
  both as that page writes them.

### Error bars

`Uncertain { value, std_dev }` holds a Gaussian. For independent inputs
the variance of `f(x, y)` is `(∂f/∂x)² σx² + (∂f/∂y)² σy²`
[wikipedia-propagation-of-uncertainty], applied to:

| Operation | σ of the result |
| --- | --- |
| `x ± y` | `√(σx² + σy²)` |
| `x · y` | `√((y σx)² + (x σy)²)` |
| `x / y` | `√((σx / y)² + (x σy / y²)²)`; a zero divisor is refused |
| `xⁿ` | `\|n xⁿ⁻¹\| σx` |
| `ln x` | `σx / x`, for `x > 0` |
| `eˣ` | `eˣ σx` |

There is no covariance term: `x.checked_add(x)` gives `σ√2`, which is right
only for independent copies, and `scaled(2.0)` gives `2σ`, which is right
for one variable [wikipedia-propagation-of-uncertainty]. Where the
relative error is large the first-order formula is not trustworthy. The
crate's rule of thumb is σ/|x| below about 0.1; no source read gives that
figure.

`combine` and `weighted_mean` take weights `1/σ²`; the result's variance is
`1/Σw` [wikipedia-inverse-variance-weighting]. An exact input wins outright,
and two exact inputs that differ are an error.

`to_significant` quotes the bar to two figures, by the `Significant`
rounding, and lets the last place of that rounded bar be the last place of the
value. The rounded bar is used, so `5.43 ± 0.996` has the bar `1.0` and the
value `5.4`. Two digits of the bar is CODATA's practice: in the 2022
adjustment's complete listing 273 of the 274 inexact uncertainties are
printed with two significant digits, and the other, the Fermi coupling
constant, with one (read by counting the digits in the listing, 2026-10-03)
[codata2022]. The Particle Data Group's rule varies the digits and is not
implemented: two when the bar's three highest digits are 100 to 354, one for
355 to 949, and 950 to 999 rounded up to 1000 with two kept
[pdg-rounding-quoted-errors-45, npae-kyiv-error-bars].

### Intervals and fuzzy instants

`DurationInterval` is a closed `[lo, hi]` of `hc_core::Duration`, which is
exact (integer seconds and attoseconds). The empty interval is its own
value, so an intersection is total. Addition is `[lo₁ + lo₂, hi₁ + hi₂]` and
subtraction crosses the bounds, `[lo₁ − hi₂, hi₁ − lo₂]`
[wikipedia-interval-arithmetic]. `A − A` is therefore not zero: for
`[1, 2]` it is `[−1, 1]`. This is the dependency problem, and there is no
operation that hides it. The midpoint halves the width componentwise, so it
stays exact past the 5.4 × 10¹² years at which an attosecond total of the
width overflows.

`FuzzyInstant` is one of seven kinds, each reduced to a **support**, a pair
of bounds on the TAI scale:

| Kind | Support |
| --- | --- |
| `Exact` | one instant |
| `Resolved { start, resolution }` | `[start, start + resolution]`, closed, so consecutive years meet |
| `Bounded { earliest, latest }` | the two bounds |
| `Gaussian { centre, std_dev }` | `centre ± 3σ` by default (`DEFAULT_SIGMA_ENVELOPE`) |
| `Before(t)` | upper bound known, lower bound unknown |
| `After(t)` | lower bound known, upper bound unknown |
| `Unknown` | neither |

A bound that is not known is not infinite: "before 1500" is some definite
instant the record does not give. That is why `relations` returns a **set**.
Allen's relations depend only on the order of the four endpoints, so the code
fixes the known bounds, lets each unknown bound take every order-distinct
position (below everything, equal to a known bound, or in a gap), and keeps
every relation that holds in some placement. A flat range is quoted as a
Gaussian with `σ = w/√12`, the standard deviation of a uniform distribution
of width `w` [wikipedia-continuous-uniform-distribution]. Allen's relations
assume a start strictly before an end [wikipedia-allen-interval-algebra]; an
exact instant has a degenerate support, several relations can hold at once,
and the set is reported as it is.

### EDTF

`EdtfValue::parse` reads the forms below and `Display` writes the same text
back. Each is a form of the Library of Congress text [loc-edtf-2019] unless a
note says otherwise.

| Form | Meaning | Support |
| --- | --- | --- |
| `1984`, `1984-01`, `1984-01-01` | year, month, day precision | the unit, as `Resolved` |
| `1984?` | uncertain | not widened |
| `1984~`, `1984%` | approximate; both | widened by its own span on each side |
| `198X`, `19XX`, `1984-01-XX`, `1984-XX-XX` | unspecified trailing digits | the decade, century, month, year |
| `1984-XX-01` | Level 2 style: unspecified month, known day | the hull of the twelve days |
| `-1985` | negative year | the year; year 0000 is 1 BC [wikipedia-iso-8601] |
| `Y-170000002` | year of more than four digits | the year |
| `1985/..`, `../1985` | open end, open start | `After`, `Before` |
| `1985/`, `/1985` | unknown end, unknown start | the same `After`, `Before` |
| `1964/2008` | interval of two dates | `Bounded`, from the start of the first to the end of the last |
| `[1667,1668,1670..1672]` | one of | the hull of the members |
| `{1960,1961-12}` | all of | the hull of the members |
| `..1760-12-03`, `1760-12..` | no later, no earlier | `Before`, `After`; see Accuracy |

The widening for `~` and `%` is a convention of the crate: the text says a
value is approximate and not by how much, and the crate adds one span of the
stated precision on each side. A date is placed by counting 86 400-second
days from 1970-01-01 on the TAI scale, with the proleptic Gregorian day
number from `hc_calendar::gregorian`. A year beyond that function's range is
reduced by whole 400-year cycles. An end of a range is the start of the next
day, so `1984` runs from the start of 1984-01-01 to the start of 1985-01-01.

**Worked example.** The numbers below were checked by running the crate
(2026-10-03).

*A bar and its value.* `y = 10.057 62 Ω` with `u = 27 mΩ = 0.027 Ω`.
The bar `0.027` already has two figures, so its last place is 10⁻³. Rounding
`y` at that place discards `62`, which begins with 6, so the digit before it
is raised: `10.058`, five figures. This is the GUM's example as
[npae-kyiv-error-bars] quotes it; the GUM itself was not read.
A tie: `1234.5 ± 12`. The bar's last place is 10⁰. The discard is exactly
`5` and the digit before it, 4, is even, so the value is `1234`, four figures
[nist-sp811-b7]. The Particle Data Group would write `0.827 ± 0.367`
as `0.8 ± 0.4`, because 367 lies in 355 to 949. The crate writes the bar as
`0.37` and the value as `0.83`.

*Combining.* `10 ± 0.3` plus `20 ± 0.4` is `30`, with
`σ² = 0.09 + 0.16 = 0.25`, so `σ = 0.5`. For two measurements of one
quantity, `5 ± 1` and `6 ± 2`, the weights are 1 and 0.25 (the sum is
1.25), the mean is `(5 + 1.5)/1.25 = 5.2`, and `σ = √(1/1.25) = 0.894`.
Quoted, the bar is `0.89`, its last place is 10⁻², and the value prints
`5.20`.

*Relations.* Take the spans `[1180, 1185]`, `[1183, 1190]` and
`[1190, 1195]`. The first starts first, shares 1183 to 1185 with the second
and ends first: `{o}`. It ends before the third begins: `{<}`. Now
`Before(1500)` against a span `[1400, 1450]`. The span's bounds are known
and the first instant's start is not, only that it is at or before 1500.
Placing that start below 1400 gives `contains`; at 1400, `started by`; between
1400 and 1450, `overlapped by`; at 1450, `met by`; above 1450, `after`. The
crate returns `{di si oi mi >}`, five relations.

*EDTF.* The day numbers count from 1970-01-01. 1984-01-01 is day 5113
(fourteen years of 365 days, 5110, and the leap days of 1972, 1976 and
1980). 1984 has 366 days, so `1984` is `[5113, 5479]`. For `1984~` the
span, 366 days, is taken off the start and added to the end: `[4747, 5845]`,
from 1982-12-31 to 1986-01-02, which is 410 140 800 s to 505 008 000 s. `198X`
runs from 1980-01-01 (day 3652) to 1990-01-01 (day 7305), 3653 days.
`[1667,1668,1670..1672]` is the hull from the start of 1667 to the start of
1673, 2192 days, and the gap of 1669 is lost.

## What is carried

- **`Significant`**, with rounding by the three NIST rules on the shortest
  decimal numeral, the figure count 1 to 17, `exact`, `with_figures`, the
  four arithmetic rules and an integer power, and a `Display` that pads and
  chooses the notation. It needs no allocator: the numeral is written into a
  40-byte buffer.
- **`Uncertain`**: the table above, `from_relative`, `within`, `contains`,
  `overlaps`, `z_score` (the separation over `√(σ₁² + σ₂²)`), `combine`,
  `weighted_mean` and `to_significant`.
- **`DurationInterval`**: construction, `centred`, `hull`, `intersect`,
  `midpoint`, addition, subtraction, negation, and scaling by an integer or a
  real factor.
- **`FuzzyInstant`** with its seven kinds, the support at a stated number of
  σ, `best_estimate` (the midpoint of a bounded or resolved support, and
  `None` for an open one, not an invented value), `as_uncertain_seconds`,
  and the thirteen relations as a `RelationSet` with `definitely_before`,
  `possibly_before`, `definitely_after`, `possibly_after` and
  `possibly_concurrent`.
- **EDTF** as in the table above, with `EdtfPrecision` from `Unknown`
  (`XXXX`) to `Day`. The set forms need the `alloc` feature and, without it,
  return `Unsupported` rather than parsing less. The module needs the
  `edtf` feature, which brings `hc-calendar`; `libm` is for `no_std` builds.

Not carried, with the reason for each:

- Not carried: EDTF's date and time of day, `1985-04-12T23:20:30Z` and its
  zone shifts. The Library of Congress text makes them Level 0 features and
  requires all of Level 0 [loc-edtf-2019], so the crate cannot claim any
  level. The module documentation gives as its reason that a second time
  parser beside `hc_core::Instant` is not wanted. Not yet done.
- Not carried: seasons `2001-21` to `2001-24` (Level 1) and the sub-year
  groupings 25 to 41, hemisphere seasons, quarters, quadrimesters and
  semestrals (Level 2). The text read gives only the codes and labels:
  "Spring (independent of location)", "Quarter 1 (3 months in duration)".
  It gives no month boundaries, so the crate has none to use
  [loc-edtf-2019]. Not yet done. What could be carried without a boundary is
  the code as a label with the year as the support. For a winter even that
  is a choice, which the text does not make: whether `2001-24` is the winter
  that begins or the one that ends in 2001 (this last point is this
  document's observation, not the text's).
- Not carried: qualifiers on part of a date. The text has two forms: a mark
  to the right of a component qualifies it and every component to its left
  (`2004-06~-11` is year and month approximate), and a mark to its left
  qualifies that component alone (`?2004-06-~11`). It also allows portions
  of an interval to be qualified (`2004-06-~01/2004-06-~20`)
  [loc-edtf-2019]. The module documentation says the implied support is not
  an interval. Not yet done.
- Not carried: exponential years and significant digits, `Y-17E7`, `1950S2`
  ("some year between 1900 and 1999, estimated to be 1950") and `Y3388E2S3`
  [loc-edtf-2019]. This is the same idea as a figure count and could map to
  `Significant`. Not yet done.
- Not carried: `X` anywhere but at the end of a component. Level 2 lets it
  occur anywhere within a component (`1984-1X`, "October, November, or
  December 1984") [loc-edtf-2019]; the parser rejects `1984-1X` and `1X84`.
  Not yet done.
- Not carried: week dates, ordinal dates, durations and ISO 8601's basic
  format. The Library of Congress text says they are outside EDTF: the extended
  format is required and the ISO 8601-1 features outside Level 0 are to be
  suppressed [loc-edtf-2019]. ISO 8601-2:2019 as a document was not read, so
  anything it has beyond the EDTF profile is not carried and not known here.
- Not carried: a disjunctive fuzzy instant, so a set is its hull and the gaps
  are lost (not yet done).
- Not carried: correlated inputs, and any distribution other than a
  Gaussian, a uniform one and a cut-off Gaussian (not yet done; the first
  order formula fails for large relative errors, and a distribution would
  serve there).
- Not carried: the Particle Data Group's variable digits, additional digits
  for rounding in later calculations (the GUM's remark, seen only as a search
  result quoted it [jcgm100-gum-7-2-6]), and any rounding convention other
  than ties to even (not yet done).
- Not carried: placing a vague date of another calendar, such as the third
  century BC or a regnal year (not yet done: the crate takes instants, and
  the calendar crates give them; its one use of `hc-calendar` is the
  Gregorian days of EDTF).

## Accuracy

The 177 unit tests and 4 documentation tests of the crate pass
(`cargo test -p hc-uncertainty`, 2026-10-03: 40 in `quantity`, 38 in
`edtf`, 36 in `fuzzy`, 32 in `interval`, 29 in `sig_figs`, 2 in `error`).
The values in the worked example were also printed by a probe linked to the
crate, which is where the figures not in a test come from.

- **Rounding.** All of NIST's examples are reproduced: `6.974 951 5` to 3
  digits is `6.97`, to 5 `6.9750`, to 7 `6.974 952`, and `6.974 950 5` to 7
  is `6.974 950` (tests); to 2 digits it is `7.0` (probe) [nist-sp811-b7].
  The tie is read on the decimal numeral: `2.675` to 3 is `2.68`, `1.15` to
  2 is `1.2`, `0.285` to 2 is `0.28`. Rounding is stable under repetition,
  and the printed digits equal `rounded()` at every tie tested. A numeral of
  16 or 17 digits, or a power of ten above 10²², can come back one unit in
  the last place from the nearest double; the function says so.
- **A different outcome from NIST's second principle.** B.7.2 writes
  `36 ft × 0.3048 m/ft = 10.9728 m = 11.0 m`, three figures, because "11"
  would lose the information in 36's rounding error of ±1.4 % [nist-sp811-b7].
  `Significant` of 36 to two figures times the exact `0.3048` gives `11`,
  two figures (probe). That is the guideline of [wikipedia-significant-figures]
  applied as written, and it is not what B.7.2 prints.
- **Error-bar digits.** `to_significant` reproduces the GUM's `10.057 62 Ω`
  with `27 mΩ` as `10.058 Ω` (test), but only the quotation of it in
  [npae-kyiv-error-bars] was read, not the GUM. It agrees with 273 of the
  274 CODATA 2022 inexact uncertainties in using two digits, and differs
  from the Particle Data Group wherever the bar's leading three digits are
  355 or above: `0.827 ± 0.367` is `0.83` with a bar of `0.37`, against the
  group's `0.8 ± 0.4`. At 950 to 999 the group rounds the bar up to 1000 with
  two digits kept, so `0.5 ± 0.0951` is the group's `0.50 ± 0.10` and the
  crate's `0.500` with `0.095` (probe).
- **Propagation** is exact for linear functions. For others the README says
  "correct to O(σ²)"; the page read for the formula says only that the linear
  form must be close to the function near the point
  [wikipedia-propagation-of-uncertainty], so no figure for the error is
  claimed here.
- **Allen's relations** are exhaustive and every one is reachable (tests
  `relations_are_exhaustive_for_every_pair_of_definite_spans` and
  `every_allen_relation_is_reachable_from_some_pair_of_spans`). Allen's own
  paper was not read, so the names, the symbols and the thirteen are as the
  Wikipedia article gives them. The 3σ cut of a Gaussian (99.73 %) is a
  convention the code states, not measured here.
- **EDTF placement** is integer arithmetic of whole days, exact. The Gregorian
  step is `hc-calendar`'s, and a test compares the 400-year-cycle adapter with
  it for every 1 March and 28 February of four cycles either side of year zero
  and at the range's edges. The anchors `1970-01-01 = RD 719163` and
  `1945-11-12 = RD 710347` are tested; their source, Reingold and Dershowitz
  [reingold2018], was not read here. The days are counted as
  86 400 seconds on TAI, which ignores leap seconds; the README says the
  displacement is below 40 seconds since 1961 and undefined before it. That
  statement was not re-measured here, and is irrelevant at one day.

Disagreements between the code, its documentation and the sources, found on
2026-10-03:

1. **The EDTF level.** `edtf.rs` line 1 and `lib.rs` line 24 say "levels 0 to
   2". The text requires all of Level 0, which includes the time of day
   [loc-edtf-2019]. The crate parses a subset of each level.
2. **What `2004-06~-11` means.** `edtf.rs` lines 31 to 33 and the README
   describe it as "June is approximate but the year and day are not". By the
   text, a mark to the right of a component applies to it and everything to
   its left: the year and the month are approximate, the day is not
   [loc-edtf-2019]. The form is rejected either way.
3. **`Y` with a short year.** `Y5` parses and prints as `Y5`; the text allows
   the prefix "when (and only when) the year exceeds four digits"
   [loc-edtf-2019]. `EdtfDate::long_year` accepts any year.
4. **Bare earlier and later forms.** `..1760-12-03` and `1760-12..` parse
   and round-trip as values of their own. The text writes them inside the set
   brackets, `[..1760-12-03]`, `[1760-12..]`, and writes open ends of
   intervals as `1985/..` and `../1985` [loc-edtf-2019]. I found no
   standalone form in the text.
5. **Open and unknown ends.** `EdtfEndpoint::Open` is documented as "genuinely
   continues". The text says `..` is used "either because there is none or
   for any other reason", and an empty end for an unknown one
   [loc-edtf-2019]. Both end up as the same unknown bound of a
   `FuzzyInstant`.
6. **Unlisted gaps.** The README lists the rejected forms but not `1984-1X`
   or `1X84`, and gives `Y17E7S3` as the exponential form. The text's
   examples are `Y-17E7`, `1950S2` and `Y3388E2S3`.
7. **The width of `~`.** `to_fuzzy_instant` says `1984~` "covers 1983 to
   1985". It adds one span of the date's own length, 366 days in the leap
   year 1984, so the support is 1982-12-31 to 1986-01-02. The span follows
   the year's length (365 days for `1983~`).
8. **Where `Display` turns scientific.** The comment says "more than four
   leading zeros". The code switches at a decimal exponent below −4, and the
   test renders `0.000 012 3`, which has four, as `1.23e-5`.
9. **Notation sources.** The comment on `Display` calls the notation the
   convention of the SI Brochure and of the *Physical Review* style guide.
   Neither was read for this document; [wikipedia-significant-figures] says
   scientific notation removes the ambiguity, which is the part this document
   cites.
10. **The 0.1 rule.** The README and `quantity.rs` give σ/|x| below about 0.1
    as the range of trust. No source read gives that figure.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [loc-edtf-2019] | Every EDTF form, level and compliance statement above, and the examples `Y-17E7`, `1950S2`, `2004-06~-11`, `?2004-06-~11`, `1984-1X` | Yes, the HTML page, 2026-10-03 |
| [wikipedia-iso-8601] | ISO 8601-2:2019 defines an EDTF profile "virtually identical" to the Library of Congress's | Yes, 2026-10-03; secondary, the ISO text was not read |
| [nist-sp811-b7] | The rounding rules 1 to 3, their examples, and B.7.2's `36 ft` example | Yes, the HTML page, 2026-10-03 |
| [jcgm100-gum-7-2-6] | At most two figures of an uncertainty, the estimate rounded to agree, and the `10.057 62 Ω` example | **Not read**: the HTML page answers with a bot challenge (HTTP 403) and the PDF was not opened. The example is read only in [npae-kyiv-error-bars]; "at most two figures" only as a search result quoted it |
| [npae-kyiv-error-bars] | The GUM example `10.057 62 Ω`, `27 mΩ`, `10.058 Ω`; the Particle Data Group's rule and its two examples | Yes, 2026-10-03; secondary |
| [pdg-rounding-quoted-errors-45] | The Particle Data Group's rule, as the 2011 review's section 5.3 quoted in a GitHub issue | Yes, 2026-10-03; secondary, the PDG text is a PDF and was not opened |
| [codata2022] | The 2022 adjustment's complete listing: 273 of 274 inexact uncertainties with two digits | Yes, the plain-text table, 2026-10-03; the digits counted by script |
| [wikipedia-significant-figures] | The product and sum rules, the two examples, the caveat, scientific notation, one or two figures of a bar | Yes, 2026-10-03; secondary |
| [wikipedia-propagation-of-uncertainty] | The first-order variance, the rules, the validity, the covariance and `A − A` | Yes, 2026-10-03; secondary |
| [wikipedia-inverse-variance-weighting] | The weights, `1/Σw` and maximum likelihood | Yes, 2026-10-03; secondary |
| [wikipedia-interval-arithmetic] | Sum and difference of intervals, the dependency problem | Yes, 2026-10-03; secondary |
| [wikipedia-allen-interval-algebra] | The thirteen relations, the strict start, exhaustiveness | Yes, 2026-10-03; secondary |
| [allen1983] | The original definition of the thirteen relations | **Not read**; cited through the Wikipedia article |
| [wikipedia-continuous-uniform-distribution] | The variance `(b − a)²/12` behind `w/√12` | Yes, 2026-10-03; secondary. The code cites the GUM's section 4.3.7 for the same factor, which was not read |
| [reingold2018] | The Rata Die anchors of the Gregorian arithmetic | **Not read** for this document; the tests carry them |

## Code

`crates/hc-uncertainty/src/`: `sig_figs.rs`, `quantity.rs`, `interval.rs`,
`fuzzy.rs`, `edtf.rs` and `error.rs`. There is no `tests` directory: the
tests are in each module. The ones that anchor it:

- `sig_figs.rs`: `nist_sp_811_b71_rounds_below_one_half_down_and_above_up`,
  `nist_sp_811_b71_sends_exactly_one_half_to_the_even_digit`,
  `a_tie_is_a_property_of_the_decimal_numeral_not_of_the_binary_value`,
  `display_and_rounded_agree_at_a_tie`,
  `a_decade_crossing_keeps_the_claimed_figures`,
  `addition_is_limited_by_the_coarser_decimal_place` and
  `subtraction_of_near_equals_destroys_figures`.
- `quantity.rs`: `the_gum_example_rounds_the_estimate_to_the_uncertainty`,
  `the_place_is_read_from_the_rounded_error_bar` and
  `adding_a_quantity_to_itself_shows_the_independence_assumption`.
- `interval.rs`: `subtraction_crosses_the_bounds`,
  `subtracting_an_interval_from_itself_is_not_zero` and
  `the_midpoint_survives_spans_far_longer_than_an_attosecond_count`.
- `fuzzy.rs`: `an_open_ended_bound_leaves_several_relations_possible`,
  `a_flat_range_becomes_a_gaussian_by_the_root_twelve_rule`,
  `relations_are_exhaustive_for_every_pair_of_definite_spans` and
  `every_allen_relation_is_reachable_from_some_pair_of_spans`.
- `edtf.rs`: `every_supported_form_round_trips_through_parse_and_render`,
  `the_adapter_agrees_with_hc_calendar_wherever_both_convert`,
  `the_unix_epoch_sits_at_the_published_rata_die`,
  `a_set_collapses_to_the_hull_of_its_members` and
  `an_empty_interval_endpoint_is_unknown_rather_than_open`.

`hc-deep-time`, `hc-orbital` and `hc-relativity` use `Uncertain`, and
`hc-deep-time` also `Significant`; the facade turns on the crate's `edtf`
feature with its `civil` feature.
