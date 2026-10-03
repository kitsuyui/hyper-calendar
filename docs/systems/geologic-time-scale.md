# The geological time scale: the ICS chart, its ranks and its editions

Backs the `geologic` module of `hc-deep-time`: `GeologicInterval`,
`GeologicRank`, the arrays `EONS`, `ERAS`, `PERIODS`, `EPOCHS` and `AGES`,
`intervals`, `interval_at`, `chain_at`, `by_id`, `children`, the edition type
`ChartEdition` with `BoundaryAmendment`, `ICS_CHART_2026_06`,
`ICS_CHART_2024_12`, `CHART_EDITIONS` and `edition_by_id`, `CHART_VERSION`,
`CHART_CITATION` and `OLDEST_BOUNDARY_MA`; the interval names of
`hc-deep-time::names`; the geologic chain of `timeline::place_megayears_ago`;
and the `deep-time` export `hc_geologic_intervals` (WebAssembly and C). The
identifiers are the 175 interval ids (`maastrichtian`, `cambrian-stage-10`)
and the two edition ids `ics-chart-2026-06` and `ics-chart-2024-12`. No
calendar identifier is registered: the scale counts megayears back from the
present, not days. Policy §10 puts it in scope because the International
Commission on Stratigraphy defines the set.

## What it is

**The scale.** The geological time scale is what the International Commission
on Stratigraphy (ICS) calls the Standard Global Chronostratigraphic
(Geochronologic) Scale, and says is to be found in its International
Chronostratigraphic Chart [ics-guide-abridged]. Bodies of rock are
chronostratigraphic units; the spans of time in which they formed are
geochronologic units, and the hierarchy pairs the two names: eonothem and eon,
erathem and era, system and period, series and epoch, stage and age
[ics-guide-abridged] [ics-chart-page]. The Guide calls the stage the lowest
ranking unit that can be recognised on a global scale, with subseries, series,
systems and erathems defined by the stage boundaries [ics-guide-abridged]. The
International Union of Geological Sciences (IUGS) ratifies the ICS's
elections, its stratigraphic standards, its GSSPs and its formal stage names,
and the chart is the ICS's product [ics-statutes].

**How a unit is defined.** The lower boundary of a stage is defined by a
Global Boundary Stratotype Section and Point (GSSP): a level in a section of
rock, marked by a spike, that coincides with a biological or other marker
[ics-gssp-table]. A candidate is approved by a vote of more than 60 % of a
subcommission's voting members, then by the ICS's voting members, then by the
IUGS Executive Committee, and the ICS updates the chart and the GSSP table, in
its words typically within days; a ratified unit or GSSP cannot be proposed
for change in its first ten years [ics-gssp-table]. The Precambrian has no
GSSPs, apart from the Ediacaran's: it was subdivided into arbitrary
geochronometric units called Global Standard Stratigraphic Ages (GSSAs)
[ics-guide-abridged]. The chart itself says that its numerical ages do not
define units in the Phanerozoic and the Ediacaran, and that only GSSPs do
[ics-chart-page]. So for a Phanerozoic stage the age is an estimate of where a
fixed point lies, and it moves when the dating improves; for a GSSA the age is
the definition. The GSSP table has 98 rows with a numerical age, and marks
several as still "Proposed", the Anisian and the Olenekian among them
[ics-gssp-table].

**The chart and its versions.** The ICS numbers each release by year and
month. It revises the chart "when definition status of geological time units
changes and/or when insight in numeric age of unit boundaries is improved"
[ics-news-156]. The supplementary page lists the current chart, 2026/06, and
25 earlier ones in PDF and JPG, from 2008 to 2024/12; its change log runs from
2012 to 2026/06 and names two further releases, 2013/Episodes and 2016/12
[ics-chart-supplementary] [ics-chart-changelog]. The ages follow *A Geologic
Time Scale 2012* until 2024/12 and *A Geologic Time Scale 2020* from then on,
"with documented exceptions" where an ICS subcommission decided otherwise
[ics-chart-changelog] [gradstein2020]. The 2024/12 release re-synchronised 44
numerical ages in one step [ics-chart-changelog]. The 2026/06 release,
distributed at STRATI 2026, changed three, and the ICS expects its next
release, 2026/12, to follow any GSSP ratified before then [ics-news-156]. The
chart is now generated from an RDF data file, a SKOS vocabulary, that the ICS
publishes [ics-chart-supplementary]. The chart asks to be cited as Cohen,
Harper, Gibbard and Car (2025) [cohen2025] from 2026/06; the 2024/12 chart
printed Cohen, Finney, Gibbard and Fan (2013, updated) [cohen2013]
[ics-chart-changelog] [ics-chart-page]. The ICS's own pages are not consistent
about the current number: the "Chart Updating Status" paragraph of the
supplementary page still says 2024/12 beside a link to the 2026/06 chart
[ics-chart-supplementary].

**What the chart leaves out.** It does not carry an Anthropocene: the ICS and
the IUGS approved the rejection of that proposal as a formal unit
[ics-news-152]. Regional chronostratigraphic scales are separate: the Guide
expects them probably always to be needed beside the global one
[ics-guide-abridged].

## How it works

**Ranks and boundaries.** The crate holds five ranks, `Eon`, `Era`, `Period`,
`Epoch` and `Age`, each a flat array ordered youngest first, each interval
naming its parent one rank up by identifier. An interval has a top and a base,
in megayears before the present. Each boundary carries an uncertainty (zero
where the chart prints none), a flag for the chart's tilde and the number of
significant figures the chart prints, so that 66.00 Ma has four and 0.0117 Ma
three. A boundary is shared by the interval below it and the one above, at
every rank that has it.

**The age convention.** The unit is `Ma`, which the ICS Guide defines with
`ka` and `Ga` as 10³, 10⁶ and 10⁹ years [ics-guide-abridged], and which the
chart's vocabulary glosses as millions of years ago [ics-chart-page]. No ICS
page read here names the year the count starts from, nor the length of the
year. The crate takes the Julian year of 365.25 days, so one megayear is 31
557 600 000 000 s (`DeepUnit::Megayear`), and calls the datum the chart's
present. That is exact enough everywhere but the Holocene. The Holocene's
base, 11 700 years in the Greenland ice core, is counted back from 2000 CE
("b2k"), which makes it 11 650 years before 1950 [walker2009]
[wikipedia-greenlandian]; the chart prints 0.0117 Ma for the same boundary,
and 0.0082 and 0.0042 Ma for the Northgrippian and Meghalayan bases
[ics-gssp-table], which a search summary of Walker et al. (2018) gave as 8236
and 4250 b2k [walker2018-holocene]. `archaeology` keeps the b2k and BP scales
apart with `b2k_to_bp`; this module keeps one number per boundary, the
chart's, and `timeline` ignores the 76 years between 1950 and the present. The
two tables therefore put the Holocene's base 50 years apart (0.0117 Ma, 11 650
BP), which is below the three figures the chart prints.

**The uncertainty and the tilde.** The vocabulary's property for the "±" is
`schema:marginOfError`, and none of the ICS pages read says whether it is one
standard deviation or two [ics-chart-page]. The crate stores it as a standard
deviation. A chart boundary with no "±" is stored with a zero: the chart
prints no "±" for a GSSA that is a round number, nor for several GSSP ages,
such as the Danian's 66.00 Ma [ics-gssp-table]. The tilde, in the chart's
words, marks a Phanerozoic boundary "without ratified GSSPs or without
constrained numerical ages", and the crate keeps it as an `approximate` flag
on the boundary [ics-chart-page].

**The half-open rule.** `GeologicInterval::contains_ma` is true for `top ≤ ma
< base`, so a boundary age belongs to the older interval: 66.00 Ma is
Maastrichtian and Cretaceous, 65.99 Ma is Danian and Paleogene.
`interval_at(ma, rank)` is `None` where the rank does not reach: before the
Hadean's base, below the present, anywhere in the Precambrian for epochs and
ages, and in the one gap the age rank has, from 419.62 to 422.7 Ma, which is
the Pridoli, an epoch the chart leaves without stages. `chain_at` gives the
five ranks at once, with `None` where a rank does not reach, so 3000 Ma is the
Archean and the Mesoarchean and three `None`s. `children` lists the intervals
one rank down.

**A duration.** `GeologicInterval::duration` subtracts the two boundaries as
`DeepTime` values and combines their uncertainties in quadrature, which treats
them as independent. The ICS change log says the Ordovician and Silurian ages
were "age-modelled" together, so for those the independence is an assumption
and not a fact [ics-chart-changelog].

**An edition.** The arrays are the 2026/06 chart [ics-chart-2026-06]. A
`ChartEdition` holds an identifier, the version as the ICS writes it, a source
string and a list of `BoundaryAmendment`s: for each boundary the edition
prints differently, the age 2026/06 gives it (`current_ma`), and the age,
uncertainty and figures the edition gives it. An amendment applies wherever
its `current_ma` appears, as a top or a base, at any rank, by the bit pattern
of the `f64`. The edition's `intervals`, `interval_at` and `by_id` apply the
amendments to the arrays on the way out. `CHART_EDITIONS` lists the editions
newest first and `edition_by_id` finds one by the matching rule of policy §5
(white space around the identifier is ignored and ASCII letters match in
either case), so an edition is chosen by its name and cannot be chosen by
accident.

| Edition | Identifier | Differs from the arrays |
| --- | --- | --- |
| v2026/06 | `ics-chart-2026-06` | not at all: the arrays are this edition |
| v2024/12 | `ics-chart-2024-12` | three ages, which move nine intervals [ics-chart-2024-12] |

The three ages, which are all that the 2026/06 release changed [ics-news-156]
[ics-chart-changelog]:

| Boundary | v2026/06 | v2024/12 | Intervals it bounds |
| --- | --- | --- | --- |
| Base of the Anisian | 247.0 Ma | 246.7 Ma | Anisian, Middle Triassic, Olenekian, Lower Triassic |
| Base of the Olenekian | 250.8 Ma | 249.9 Ma | Olenekian, Induan |
| Base of the Wuchiapingian | 259.857 ± 0.084 Ma | 259.51 ± 0.21 Ma | Wuchiapingian, Lopingian, Capitanian, Guadalupian |

The editions are the policy's reading of an authority's revision (§10: the old
version stays and the new one is added under its own name) and, where two
charts disagree on a boundary, of §5: the choice is a name, not a parameter.

**The names.** `hc-deep-time::names` carries the chart's interval names in 14
languages: `cs`, `de`, `es`, `fr`, `id`, `it`, `ja`, `ko`, `nl`, `pl`, `pt`,
`ru`, `tr` and `zh-Hans`, from the ICS's own SKOS vocabulary [ics-chart-ttl].
A tag is matched as `hc-i18n` matches one, by dropping subtags from the right,
so `ja-JP` finds `ja` and `zh-Hans-CN` finds `zh-Hans`; plain `zh` and
`zh-Hant` find nothing. The Japanese names are the Geological Society of
Japan's [gsj-chart-2024-12], in its notation: `第四系／紀` is the Quaternary as a
system (rock) and as a period (time) at once, and a stage is written in
katakana without the suffix (`オレネキアン`). The Chinese names are
chronostratigraphic, as the ICS's Chinese chart writes them
[ics-chart-2023-09-zh]: 显生宇, 第四系, 全新统, 奥伦尼克阶. They name the rock unit, not the
span of time, and are in simplified script only. In 13 of the 14 languages 21
intervals have no name, because the chart's vocabulary has none: the upper,
middle and lower series of the Cretaceous, Jurassic, Triassic, Pennsylvanian,
Mississippian, Devonian and Ordovician, and the Upper Pleistocene. Russian
lacks four more: three the vocabulary gives in English words, and the Hadean,
in mixed scripts. The names do not depend on the edition.

**Worked example: 250 Ma, and two editions.** Take 250 Ma before the present.

1. Eon: Phanerozoic runs from 0 to 538.8 Ma, so it contains 250. Era: the
   Cenozoic ends at 66 and the Mesozoic runs from 66 to 251.902, so Mesozoic.
   Period: the Triassic runs from 201.4 to 251.902. Epoch: in v2026/06 the
   Lower Triassic runs from 247.0 to 251.902.
2. Age, v2026/06: the Olenekian runs from 247.0 to 250.8, and 247.0 ≤ 250 <
   250.8, so the Olenekian. The chain is Phanerozoic, Mesozoic, Triassic,
   Lower Triassic, Olenekian.
3. Age, v2024/12: the Olenekian runs from 246.7 to 249.9, and 250 is not below
   249.9; the Induan runs from 249.9 to 251.902, which contains 250. The same
   age is the Olenekian in one edition and the Induan in the other.
4. The boundary itself: 250.8 Ma is not below the Olenekian's base, so it is
   in the older interval, the Induan, in v2026/06.
5. A duration with its uncertainty. The Induan's top has no "±" in either
   edition and its base is 251.902 ± 0.024: in v2026/06 it lasted 251.902 −
   250.8 = 1.102 ± 0.024 Ma, and in v2024/12 251.902 − 249.9 = 2.002 ± 0.024
   Ma. The Wuchiapingian has two uncertain ends: 259.857 − 254.14 = 5.717 Ma
   with √(0.084² + 0.070²) = √0.011956 = 0.109 Ma in v2026/06, and 259.51 −
   254.14 = 5.370 Ma with √(0.210² + 0.070²) = √0.049 = 0.221 Ma in v2024/12.
6. In seconds, 250.8 Ma is 250.8 × 10⁶ × 31 557 600 s = 7.914 646 08 × 10¹⁵ s.
7. At the boundary, `hc_geologic_intervals(4, "ja")` writes for the Olenekian
   a line whose cells are `age`, `olenekian`, `Olenekian`, `lower-triassic`,
   then the older bound 250.8, its uncertainty 0, its figures 4 and its
   approximate flag 0, the younger bound 247, 0, 4 and 0,
   `megayears-before-present`, an empty description, the chart citation and
   `オレネキアン`. It always writes the 2026/06 values.

Every number above was checked by running the crate: the edition's
`interval_at` and `duration`, `as_megayears`, and the export's line builder.

## What is carried

The 2026/06 chart in five ranks, the chart's own ages, uncertainties, tildes
and printed figures, and its hierarchy:

| Rank | Entries | Spans |
| --- | --- | --- |
| `Eon` | 4 | 0 to 4567 Ma: Phanerozoic, Proterozoic, Archean, Hadean |
| `Era` | 10 | 0 to 4031 Ma; the Hadean has none |
| `Period` | 22 | 0 to 2500 Ma; the Archean has none |
| `Epoch` | 38 | 0 to 538.8 Ma |
| `Age` | 101 | 0 to 538.8 Ma, except the Pridoli |

- **The intervals,** 175 in all, each with its id (the chart's name in
  lower-case kebab: `cambrian-series-2`), name, rank, parent, boundaries and
  flags. The ids are matched by `hc_core::catalogue::matches`; a name such as
  `Cambrian Stage 10` is not an id, and a superseded name, `vendian`, is not
  here.
- **The two editions,** as above, and `CHART_VERSION` and `CHART_CITATION` for
  the arrays, so that a chronology quoted from the crate says which chart it
  came from.
- **The rank names in both vocabularies:** `english_name` gives eon to age,
  `chronostratigraphic_name` gives eonothem to stage.
- **24 intervals with a tilde,** 15 distinct ages in all: 205.7, 227.3 and
  237.0 Ma in the Triassic, 491.0 to 529.0 Ma in the Cambrian, 635 and 720 Ma
  and the Hadean's base, 4567 Ma.
- **The exports.** `hc_geologic_intervals(rank, locale)` writes one line per
  interval of a rank, youngest first, in the sixteen columns every deep-time
  line has, in `megayears-before-present`, with the parent's id as the scope
  and the chart's name in the locale last; it takes `rank` as 0 for the eons
  to 4 for the ages. `hc_place_years_ago` writes the geologic chain of a
  moment, and a moment up to a century ahead is still placed in the intervals
  that end at the present. The exports carry no edition: the editions are
  Rust-only.
- **The names** of the 14 languages above, 154 of the 175 intervals in each,
  150 in Russian.

**Not carried**, with the reason for each; none of them is a decision the
policy makes:

- *The charts before 2024/12.* The ICS lists 24 of them back to 2008, and its
  change log names the change each made since 2012. Not yet done, and not only
  for want of the charts: the PDFs were not read for this document, and the
  amendment type changes the numbers of intervals the arrays already have. It
  cannot say that a unit was added, renamed or moved between ranks, as the
  Holocene's three stages were in 2018/07 or the Miaolingian replaced Series 3
  [ics-chart-changelog]. The 2024/12 re-synchronisation of 44 ages would also
  need 44 amendments, and one amendment moves every boundary that has its age.
  The base of the Pleistocene is the clearest case of an earlier convention
  with no name here: the Calabrian's GSSP defined it in 1985, at 1.80 Ma, and
  the Gelasian's has defined it at 2.58 Ma since 2009 [ics-gssp-table].
- *Whether a boundary is a GSSP, a GSSA or neither,* its ratification year,
  its section and its coordinates. The GSSP table has them for 98 rows and the
  vocabulary flags a unit as having a ratified GSSP or GSSA [ics-gssp-table]
  [ics-chart-page]. Not yet done: the crate stores only the tilde, so a zero
  uncertainty cannot be read as "defined" or "unmeasured".
- *The italic type,* by which the chart marks informal units and placeholders
  for unnamed ones [ics-chart-page]. The crate has no flag for it; the
  vocabulary does not carry it, so a source for each unit would be the printed
  chart, which was not read.
- *The ratified subseries,* Lower, Middle and Upper of the Holocene and the
  Pleistocene, and of the Miocene and the Pliocene [ics-chart-changelog]. The
  Upper Pleistocene is held as an age, as the vocabulary has it, and the
  Holocene's three stages are held; the subseries as such are not. The
  supplementary page still lists a subseries indication among the planned
  additions to the ICS's data file [ics-chart-supplementary]. Not yet done.
- *The Carboniferous subsystems.* The Mississippian and the Pennsylvanian are
  not a rank here; their series have the Carboniferous as parent. Not yet
  done: the ICS vocabulary calls the sub-period an informal unit while its
  GSSP table and change log treat the two as sub-periods and subsystems
  [ics-chart-page] [ics-gssp-table] [ics-chart-changelog], and the crate has
  taken neither view in a source it names.
- *The Precambrian super-eon,* which the vocabulary defines as an informal
  unit containing the eons before any evidence of life [ics-chart-page].
- *Regional and national stages,* of any country. No source for any of them
  was read; the ICS leaves them to regional scales [ics-guide-abridged].
- *Any other scale than the chart's:* marine isotope stages, magnetic chrons,
  biozones, the Blytt-Sernander stages. No source read.
- *The Geologic Time Scale 2020* itself, from which most ages come
  [gradstein2020]: not read, so no age is taken from it.
- *The ICS's other languages.* The vocabulary labels units in 26 language
  tags, English included, and the crate carries 14 of them, the languages
  `names.rs` says `hc-i18n` also carries. The 11 others are `az`, `ca`,
  `es-a`, `eu`, `fi`, `hu`, `lt`, `nl-be`, `no`, `pt-br` and `sk`
  [ics-chart-page]. Not yet done. Traditional-script Chinese has no table
  either.
- *The chart's colours and notation codes,* which the vocabulary carries.

## Accuracy

The values are transcribed from the printed charts, and the checks are of
three kinds.

**The crate's own tests,** in `geologic.rs`: every rank is ordered, never
overlaps and has positive durations; each rank tiles its span, the Pridoli
being the one gap; each interval names an existing coarser parent and lies
inside it; the counts are 4, 10, 22, 38 and 101; the half-open rule, the chain
at 250 Ma, 0 Ma and 3000 Ma, the Pridoli gap and the identifiers are tested;
and the 2024/12 edition is tested to differ from 2026/06 in exactly the nine
intervals the 2026/06 file's change notes name. Those are checks of internal
consistency and of the three moved ages. They do not check a transcribed age
against the ICS, and no test does.

**A comparison made for this document,** on 2026-10-03, of the arrays with the
ICS's vocabulary as the chart page serves it (version 2026-06, modified
2026-06-20) [ics-chart-page], interval by interval, 175 intervals and 350
boundaries counting a boundary at each interval it bounds:

- Age, uncertainty and tilde agree on all but nine boundaries, which are four
  departures.
- The 443.1 Ma boundary has "± 1.0" in the vocabulary and the crate has ± 0.9,
  which it takes from the printed chart. Six intervals bound it: the Silurian,
  the Llandovery, the Rhuddanian, the Ordovician, the Upper Ordovician and the
  Hirnantian.
- The Aquitanian's base is 23.03 Ma in the vocabulary and 23.04 Ma in the
  crate. The change log gives 23.04 Ma from 2024/12 [ics-chart-changelog], and
  the vocabulary's own Miocene base is 23.04 Ma.
- The Ludlow's top is 419.62 ± 1.36 Ma in the vocabulary, which gives the
  Pridoli two ranks, age and epoch, and 422.7 ± 1.6 Ma in the crate, where the
  Pridoli is an epoch and the Ludfordian is the youngest age of the Silurian.
- The Hadean's base carries a tilde in the crate and none in the vocabulary.
  The ICS's change log says that in 2022/10 the Hadean's GSSA was set at
  4,567.30 ± 0.16 Ma, "displayed in rounded form" as 4,567, replacing a
  provisional age of about 4,600 Ma [ics-chart-changelog].

The first three are departures the module documentation records. The Hadean's
tilde is not recorded there.

**The GSSP table,** read the same day: 98 rows have a numerical age and 96
agree with the crate's base age and, where the table prints one, its
uncertainty. The two that do not are the Aquitanian (23.03 against 23.04) and
the Rhuddanian (± 1.0 against ± 0.9) [ics-gssp-table]. The table has no row
for the Valanginian, whose GSSP the change log says was ratified on 18
December 2024.

**The three moved ages** agree with the ICS's news item and its change log
number for number, and the change log's 2026/06 entries name the same three
boundaries and no others [ics-news-156] [ics-chart-changelog]. The 2024/12
values are the "was" values of the news item and the change log (246.7, 249.9
and 259.51 ± 0.21); they were not checked against the 2024/12 PDF.

**What the stated precision means.** The error bars are the chart's, and
nothing narrower. The `figures` fields are the digits the chart prints: the
GSSP table prints 66.00, 56.00 and 1.80, which the crate holds as four, four
and three figures. The printed charts were not read. Whether the "±" is one or
two standard deviations is not stated by any ICS page read, so the `std_dev`
fields and the quadrature of `duration` rest on that reading. A duration's
uncertainty is that of two independent boundaries and is smaller than the
truth where they are not. `timeline::place_megayears_ago` places a moment by
its central value alone, so its uncertainty does not decide which interval it
is in; an age of 250.8 ± 0.5 Ma is placed in the Induan, the older interval,
though its ± straddles the Olenekian.

**Where the crate's documentation disagrees with the ICS.**

- The module documentation calls a GSSA "a round number *defined by decree*",
  and the README "a GSSA defined by decree". The Hadean's and the Eoarchean's
  GSSAs are 4,567.30 ± 0.16 Ma and 4,031 ± 3 Ma [ics-chart-changelog], and the
  Guide calls GSSAs "arbitrary geochronometric units" [ics-guide-abridged].
- They also say the chart does not carry the GSSA-or-GSSP distinction in
  machine-readable form. The vocabulary carries `gts:ratifiedGSSP` on 130 of
  its 178 concepts and `gts:ratifiedGSSA` on 19 [ics-chart-page].
- `OLDEST_BOUNDARY_MA` says the chart prints the Hadean's base without an
  uncertainty. The ICS's stated GSSA has one, and the chart displays it
  rounded.
- The module documentation calls the Mississippian and Pennsylvanian "formally
  subsystems". The ICS vocabulary defines a sub-period as an informal
  geochronologic unit [ics-chart-page].
- The module documentation says the tilde means "no ratified GSSP and no
  well-constrained age"; the chart says "or" and limits it to the Phanerozoic
  [ics-chart-page], though the vocabulary, and so the crate, marks 635 and 720
  Ma as well.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [ics-chart-page] | The current version (v2026-06); the chart's three paragraphs of notes, including the tilde and the sources of ages; the vocabulary's definitions of ranks, its `ratifiedGSSP` and `ratifiedGSSA` flags, `marginOfError`, the sub-period and super-eon definitions; every boundary compared | Yes, 2026-10-03. The page is rendered by script, so the text and the vocabulary were read from the script it loads |
| [ics-chart-supplementary] | The list of 25 earlier charts and the translations; the change log's link; the data file; the planned subseries indication; the stale "2024/12" statement | Yes, 2026-10-03 |
| [ics-chart-changelog] | Every change since 2012: the 2026/06, 2024/12 and 2023/09 changes, the Hadean and Eoarchean GSSAs, the sources of the ages, the notes on citation | Yes, 2026-10-03 (a plain-text file) |
| [ics-news-156] | The 2026/06 release: the three ages, their sources, the 2026/12 expectation | Yes, 2026-10-03 |
| [ics-news-152] | The rejection of an Anthropocene unit | Yes, 2026-10-03 |
| [ics-gssp-table] | The GSSP definition and process; the 98 ages compared; the "Proposed" rows; the Pleistocene's two bases | Yes, 2026-10-03 |
| [ics-guide-abridged] | The hierarchy, the GSSA definition, `ka`, `Ma`, `Ga`, regional scales | Yes, chapter 9 and the definitions chapter, 2026-10-03; the Guide itself (Salvador 1994) not read |
| [ics-statutes] | That the IUGS ratifies the ICS's elections, standards, GSSPs and formal stage names | Yes, 2026-10-03 |
| [wikipedia-greenlandian] | The Holocene's base against 2000 and 1950 | Yes, 2026-10-03; secondary |
| [walker2009] | The Holocene GSSP at 11 700 b2k | Not read; cited through [wikipedia-greenlandian] |
| [walker2018-holocene] | The Northgrippian and Meghalayan GSSPs at 8236 and 4250 b2k | Not read; the figures are as a search summary of the paper's page gave them |
| [cohen2025] | The citation the chart asks for | Not read; the citation as the ICS prints it |
| [cohen2013] | The citation the 2024/12 chart prints | Not read; the citation as the ICS prints it |
| [gradstein2020] | The source of most ages | Not read |
| [ics-chart-2026-06] | The ages the arrays transcribe | Not read: a PDF, not opened for this document, though the supplementary page links it at the address the crate cites; its ages were compared with the vocabulary, the change log and the GSSP table instead |
| [ics-chart-2024-12] | The three ages of the second edition | Not read: a PDF; checked against the change log only |
| [ics-chart-ttl] | The names in 14 languages | Not read from its repository; the same vocabulary was read as the chart page serves it. The commits the crate names were not checked |
| [gsj-chart-2024-12] | The Japanese names | Not read: a PDF |
| [ics-chart-2023-09-zh] | The Chinese names | Not read: a PDF |

## Code

`crates/hc-deep-time/src/geologic.rs` (the arrays, the editions, the queries)
and `crates/hc-deep-time/src/names.rs` (the translations);
`crates/hc-deep-time/src/timeline.rs` for the placement of a moment;
`crates/hyper-calendar/src/deep_time_lines.rs` for the export's lines and
`crates/hyper-calendar-wasm/src/deep_time.rs` for `hc_geologic_intervals`. The
tests that anchor it, in `geologic.rs`:
`every_rank_is_ordered_youngest_to_oldest_and_never_overlaps`,
`every_rank_tiles_its_span_without_gaps_except_the_pridoli`,
`the_chart_has_the_expected_number_of_intervals`,
`every_interval_lies_inside_its_parent`,
`the_cretaceous_paleogene_boundary_belongs_to_the_older_interval`,
`the_pridoli_gap_leaves_the_age_rank_with_no_answer`,
`the_revised_triassic_and_permian_ages_are_the_current_ones`,
`chart_v2024_12_keeps_its_own_three_boundaries` and
`the_two_editions_differ_in_exactly_the_intervals_the_2026_06_file_notes`; in
`names.rs`: `the_japanese_names_are_the_geological_society_of_japans`,
`the_chinese_names_are_the_ics_chinese_charts_without_layout_spaces`,
`tags_are_matched_as_hc_i18n_matches_them` and
`what_the_chart_does_not_name_is_left_unnamed`; and for the export,
`the_geologic_ranks_are_numbered_coarsest_first` in
`crates/hyper-calendar-wasm/src/tests/deep_time.rs`.
