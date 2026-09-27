# The earliest evidence of life, of *Homo sapiens* and of writing

Backs `hc_deep_time::evidence`, the `hc_earliest_evidence` export and its
`earliestEvidence` binding, and the names by identifier in
`hc_deep_time::names`.

## What it is

A timeline of deep time wants three landmarks that none of the chronologies
this library carries can give: when life began, when our species appeared,
and when people began to write. Each is asked as a single date and none has
one. There is no ratified boundary for any of them, as there is for a
geological stage, and no authority keeps a list. What exists instead is a
literature of claims: a paper reports a find — graphite in a zircon, a
layered structure in a metamorphosed carbonate, a skull, an inscribed label
— with a date for it, and later papers accept, qualify, push back or
dispute it.

So the useful answer to "when is the earliest evidence of life" is not a
number but the set of claims, each with its own date in the form its
source gives it, and a statement of which are disputed. That is what this
library carries. The set is **not** closed in the sense of
[policy.md §10](../policy.md): no authority defines it. The choice of the
three landmarks is the library's, made because a timeline asked for them,
and within a landmark the claims carried are those the literature read
cites as the earliest; other published claims exist, and the ones known
and not carried are named below. What can be checked is each entry: it is
a published claim that names itself the earliest of its kind, or is named
so by a later paper, and its date is that paper's. Coverage is stated
entry by entry, not as a complete list.

## How it works

### The shape of a date

A source dates a find in one of three shapes, and the shape is part of the
claim:

| Shape | Written in the source as | Carried as |
| --- | --- | --- |
| An age | "315 ± 34 thousand years ago", "3,700-Myr-old", "ca. 3.48 Ga" | `Dating::Age` |
| A minimum age | "a new minimum age … of 233 ± 22 kyr", "by at least 3,800 Myr", "more than 3700 million years ago" | `Dating::AtLeast` |
| A range | "at least 3,770 million and possibly 4,280 million years old" | `Dating::Between` |

A minimum age has no older limit, and inventing one — or turning "at least
233 kyr" into "233 kyr" — would be a different claim. A range is not a mean
and a spread. Every entry keeps the shape its source uses.

The uncertainty is the second half of the shape. A `±` means nothing until
the source says whether it is one standard deviation, two, or a range, so a
standard uncertainty is carried only where the source says so:

- Vidal et al. 2022 give 233 ± 22 kyr "at 2 σ" [vidal2022]. The standard
  uncertainty carried is 11 kyr.
- Richter et al. 2017 give 315 ± 34 ka in the abstract [richter2017]; the
  full text, which would say what the `±` is, was not reached. The value is
  carried with no standard uncertainty and the `±` stays in the
  description.
- Bell et al. 2015 give 4.10 ± 0.01 Ga, likewise in an abstract that does
  not say [bell2015]; carried the same way.
- Every other date read is written without a `±` and is carried with no
  standard uncertainty. Those written "about", "approximately", "ca.", "~"
  or "-Myr-old" — "3,700-Myr-old", "ca. 3.48 Ga", "ca. 3300 BC" — are
  carried as approximate; those written as plain figures — Dodd et al.'s
  "at least 3,770 million and possibly 4,280 million years", Rosing's
  "more than 3700 million years ago" — are not.

### The datum

Every age is in years before 1950, the BP datum `hc_deep_time::archaeology`
uses, so that a landmark sorts among the archaeological periods. The
geological and palaeoanthropological sources count from the year they
measured in; the difference is under a century and at least two decades
below the precision of any date here, the same approximation
`hc_deep_time::timeline` makes. The two writing claims are dated in
calendar years BC, and there the conversion is exact: *n* BC is
1949 + *n* years before 1950, because the Christian era has no year zero.

### A worked example

Vidal et al. matched the KHS Tuff, which "conclusively overlies" the member
of the Kibish Formation that holds the Omo I fossils, to the Qi2 eruption
of Shala volcano, and dated the eruption by ⁴⁰Ar/³⁹Ar to a weighted mean of
233 ± 22 kyr at 2σ [vidal2022]. A tuff above a fossil is younger than the
fossil, so the tuff's age is a minimum for it. The entry is therefore:

- the shape `AtLeast`, with no older limit;
- the minimum 233 000 years, three figures as printed;
- the standard uncertainty 22 000 / 2 = 11 000 years.

`hc_earliest_evidence` writes it as

```text
earliest-evidence  earliest-homo-sapiens-omo-kibish  Omo I, Omo-Kibish  earliest-homo-sapiens
  (start: empty × 4)  233000  11000  3  0  years-before-1950  …  オモの化石
```

— the older bound empty, the younger bound the minimum. A page that draws
it draws an arrow into the past from 233 kyr, not a point.

The Uruk IV entry is the calendar case. Englund dates "the emergence of
proto-cuneiform ca. 3300 BC" [englund2004]; 1949 + 3300 = 5249 years before
1950, approximate, two figures. Placed with `place_years_ago(5249.0, 0.0)`
it falls in this library's Bronze Age, whose start at 5250 BP is one of the
module's round numbers — a coincidence of two round numbers, not a
finding; tomb U-j's 5299 falls fifty years earlier, in the Chalcolithic
(`the_claims_are_placed_against_the_round_numbered_periods`).

## What is carried

Twelve claims under three landmarks, oldest first within each, in
`hc_deep_time::evidence::EVIDENCE`:

| Identifier | Claim | Date as carried | Disputed |
| --- | --- | --- | --- |
| `earliest-life-jack-hills` | Graphite inclusions in a Jack Hills zircon, "potentially biogenic" [bell2015] | at least 4.10 Ga (± 0.01 as printed, no σ) | — |
| `earliest-life-akilia` | Light carbon in apatite, Akilia and Isua [mojzsis1996] | at least about 3.8 Ga | Fedo & Whitehouse 2002 [fedo2002] |
| `earliest-life-nuvvuagittuq` | Haematite filaments, Nuvvuagittuq belt, "putative" [dodd2017] | between 4.28 and 3.77 Ga | — |
| `earliest-life-isua-graphite` | Graphite globules in Isua sediments [rosing1999] | more than 3.7 Ga | — |
| `earliest-life-isua-stromatolites` | Stromatolites in Isua metacarbonates [nutman2016] | about 3.7 Ga | Allwood et al. 2018 [allwood2018] |
| `earliest-life-dresser` | Stromatolites, microfossils and hot-spring deposits, Dresser Formation [djokic2017; nutman2016] | about 3.48 Ga | — |
| `earliest-life-apex-chert` | Filamentous microfossils, Apex chert [schopf1993] | at least about 3.465 Ga | Brasier et al. 2002 [brasier2002] |
| `earliest-life-strelley-pool` | Stromatolite reef, Strelley Pool Chert [allwood2006] | about 3.43 Ga | — |
| `earliest-homo-sapiens-jebel-irhoud` | Jebel Irhoud hominins [hublin2017; richter2017] | 315 ka (± 34 as printed, no σ) | — |
| `earliest-homo-sapiens-omo-kibish` | Omo I, Omo-Kibish [vidal2022] | at least 233 ka, σ 11 ka | — |
| `earliest-writing-abydos-u-j` | Inscribed labels and pots of tomb U-j, Abydos [gorsdorf1998] | about 3350 BC, 5299 before 1950 | — |
| `earliest-writing-uruk-iv` | Proto-cuneiform tablets of Uruk IV [englund2004] | about 3300 BC, 5249 before 1950 | — |

"Disputed" lists only a rebuttal read for this library, so a claim without
one is not therefore undisputed: a rebuttal not read here is not listed.
Nature's Japanese highlight of the Isua paper expected it to be argued
about ("議論を呼びそうだ") two years before Allwood et al. were
[natureasia-2016-09-22]. A dispute is written into the
entry's description and its `disputed_by`; the claim stays, under its own
identifier, and no entry merges two claims into one number
([policy.md §5](../policy.md)). The two writing entries are two readings of
one landmark in the same way: an Egyptian and a Mesopotamian claim, dated by
different methods, that the sources read do not compare.

Every entry of every deep-time table also has an `id`, lower-case kebab like
a calendar's, written as the second column of every deep-time line; the
geologic rows' scope is the parent's id. The cosmic epochs and events, the
archaeological periods and three of the claims have Japanese names,
`hc_deep_time::names::entry_name`, each read in the source below:

| Identifier | Name | Read in |
| --- | --- | --- |
| `planck-epoch`, `grand-unification-epoch`, `inflationary-epoch`, `electroweak-epoch`, `quark-epoch`, `hadron-epoch`, `lepton-epoch`, `photon-epoch` | プランク時代, 大統一時代, インフレーション時代, 電弱時代, クォーク時代, ハドロン時代, レプトン時代, 光子時代 | the Japanese Wikipedia's article of that title, each giving the English term, and its 宇宙の年表 [wikipedia-ja-cosmic-epochs] |
| `dark-ages`, `reionisation`, `big-bang-nucleosynthesis`, `matter-radiation-equality`, `recombination`, `first-stars`, `present-day` | 宇宙の暗黒時代, 宇宙の再電離, ビッグバン元素合成, 物質と放射の等密度時, 宇宙の晴れ上がり, 初代星, 現在 | the Astronomical Society of Japan's 天文学辞典 [astro-dic]: its entries of those names; 等密度時 in 物質優勢期（宇宙の）; 宇宙の晴れ上がり, which it gives as the term for what English calls the recombination epoch or photon decoupling; 現在 in 宇宙年齢 |
| `first-galaxies` | 初代銀河 | the National Astronomical Observatory of Japan's ALMA feature of 2018 [alma-6000-vol2] |
| `milky-way-formation`, `solar-system-formation` | 銀河系の形成, 太陽系の形成 | the Japanese Wikipedia, 宇宙カレンダー [wikipedia-ja-cosmic-calendar] |
| `modern-period`, `middle-ages`, `classical-antiquity`, `iron-age`, `bronze-age`, `chalcolithic`, `neolithic`, `upper-palaeolithic`, `middle-palaeolithic`, `lower-palaeolithic` | 近代, 中世, 古典古代, 鉄器時代, 青銅器時代, 銅器時代, 新石器時代, 後期旧石器時代, 中期旧石器時代, 前期旧石器時代 | the Japanese Wikipedia article the English article of the period's name links to [wikipedia-ja-archaeological-periods] |
| `earliest-homo-sapiens-jebel-irhoud`, `earliest-homo-sapiens-omo-kibish` | ジェベル・イルードのヒト族化石, オモの化石 | Nature's Japanese highlights of the two papers [natureasia-2017-06-08; natureasia-2022-01-27] |
| `earliest-writing-uruk-iv` | 原楔形文字 | the Japanese Wikipedia, 原楔形文字 [wikipedia-ja-proto-cuneiform] |

Left unnamed, because no established term was found: `era-of-galaxies`;
`neutrino-decoupling`, which the 天文学辞典 describes ("ニュートリノが脱結合した")
without naming as a noun; `epipalaeolithic`, whose Japanese article is
titled 亜旧石器時代 and opens with 終末期旧石器時代, two terms from one source;
and nine claims, whose sites were not found named in a Japanese source —
Nature's Japanese highlights of the two Isua papers describe the find
without naming it [natureasia-2016-09-22; natureasia-2018-11-08]. The
future eras and events have no Japanese table.

Not carried:

- **Other claims to the earliest life**: Ohtomo et al., *Nature Geoscience*
  7, 25 (2014), on biogenic graphite at Isua, was found and not read; Buick's
  criticisms of the Isua stromatolites were seen only as quoted in a news
  summary. Nothing is carried from either.
- **Other claims to the earliest *Homo sapiens***: Herto, Florisbad and
  the others are not carried, and neither is the older Omo date of about
  197 kyr, which Vidal et al. replace.
- **Proto-writing**: the Jiahu signs, the Vinča symbols and the tokens and
  numerical tablets before Uruk IV are not writing in the sources read;
  Englund's account puts the numerical tablets and clay bullae before
  proto-cuneiform.

## Accuracy

The entries are transcriptions, so the check is that each number is the
source's. Every date was read in the source named for it on 2026-09-27 and
is tested against it in `the_published_anchors_are_carried`:
Vidal's minimum and its σ, Richter's age with no σ, Dodd's range, Nutman's
220 Myr between Isua and the Dresser, and the BC conversions for Englund
and Görsdorf et al.

Three things limit what the entries mean:

- **Most dates were read in abstracts.** Only Vidal et al., Djokic et al.,
  Görsdorf et al. and Englund were read in full. An abstract's round
  number is what the paper stands behind, but its error model is in the
  text, which is why three `±` are carried without a σ.
- **The U-j date is the phase's, not the tomb's.** Görsdorf et al. date
  Naqada IIIa2 "to the middle of the 34th century BC" by wiggle-matching the
  tombs of that phase, U-j among them; the two samples from U-j itself
  calibrate to ranges between 3490 and 3030 cal BC as the paper lists them,
  on the 1993 calibration curve. The entry carries the phase's date as
  approximate, and the description names both.
- **The archaeological periods beside them are round numbers.** Jebel
  Irhoud's 315 ka falls in this library's Lower Palaeolithic, whose end at
  300 ka is a round number, although Richter et al. place the site's
  artefacts in the Middle Stone Age. The period table's own documentation
  says what its boundaries are worth; a claim is placed against it, not
  corrected by it.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [bell2015] | Jack Hills graphite, 4.10 ± 0.01 Ga | Abstract, from Europe PMC and PubMed Central, 2026-09-27; the full text was not served |
| [mojzsis1996] | Akilia and Isua apatite carbon, by at least 3,800 Myr | Abstract |
| [fedo2002] | The rebuttal of Akilia | Abstract |
| [dodd2017] | Nuvvuagittuq, at least 3,770 and possibly 4,280 Myr | Abstract |
| [rosing1999] | Isua graphite, more than 3700 Ma | Abstract |
| [nutman2016] | Isua stromatolites, 3,700 Myr; the Dresser Formation at 3,480 Myr as the previous accepted evidence | Abstract |
| [allwood2018] | The rebuttal of the Isua stromatolites | Abstract |
| [djokic2017] | The Dresser Formation, ca. 3.48 Ga | Yes, open access, 2026-09-27 |
| [schopf1993] | Apex chert microfossils, at least approximately 3465 Ma | Abstract |
| [brasier2002] | The rebuttal of the Apex chert | Abstract |
| [allwood2006] | Strelley Pool stromatolites, 3,430 Myr | Abstract |
| [hublin2017] | The Jebel Irhoud fossils and their reading as early *H. sapiens* | Abstract |
| [richter2017] | The Jebel Irhoud date, 315 ± 34 ka, and the tooth's 286 ± 32 ka | Abstract; the full text, which would give the confidence level, was not reached |
| [vidal2022] | Omo I, a minimum of 233 ± 22 kyr at 2σ | Yes, open access, 2026-09-27 |
| [gorsdorf1998] | Tomb U-j, Naqada IIIa2 in the middle of the 34th century BC; U-j as the earliest hieroglyphic writing from Egypt | Yes, from the University of Arizona repository, 2026-09-27 |
| [englund2004] | Proto-cuneiform, emerging ca. 3300 BC (p. 26); Uruk IV and Uruk III, the Uruk III period ca. 3100–3000 BC (p. 40) | Yes, the whole chapter, pp. 23–46, in the CDLI's PDF, 2026-09-27; it cites Englund 1998 (OBO 160/1) and Nissen, Damerow & Englund 1993, not read here |
| [astro-dic] | Japanese names of cosmic epochs and events | Yes, 2026-09-27 |
| [alma-6000-vol2] | 初代銀河 | Yes, 2026-09-27 |
| [wikipedia-ja-cosmic-epochs] | Japanese names of the cosmic epochs | Yes, 2026-09-27 |
| [wikipedia-ja-cosmic-calendar] | 銀河系の形成, 太陽系の形成 | Yes, 2026-09-27 |
| [wikipedia-ja-archaeological-periods] | Japanese names of the archaeological periods | Yes, 2026-09-27, reached through the English articles' interlanguage links |
| [wikipedia-ja-proto-cuneiform] | 原楔形文字 | Yes, 2026-09-27 |
| [natureasia-2017-06-08] | ジェベル・イルードのヒト族化石 | Yes, 2026-09-27 |
| [natureasia-2022-01-27] | オモの化石 | Yes, 2026-09-27 |
| [natureasia-2016-09-22] | That the Isua stromatolites were expected to be controversial; no name taken | Yes, 2026-09-27 |
| [natureasia-2018-11-08] | The Japanese account of Allwood et al.; no name taken | Yes, 2026-09-27 |

## Code

`crates/hc-deep-time/src/evidence.rs` holds the claims, `names.rs` the
Japanese names by identifier, and `crates/hyper-calendar/src/deep_time_lines.rs`
writes them. Anchors: `the_published_anchors_are_carried`,
`no_uncertainty_is_invented`, `a_disputed_claim_says_so_in_its_description`,
`a_year_bc_is_counted_across_the_missing_year_zero`,
`the_table_is_grouped_by_landmark_and_oldest_first` in `evidence`;
`every_entry_name_is_keyed_to_an_entry_that_exists` and
`the_japanese_entry_names_are_the_ones_read` in `names`;
`every_identifier_is_lower_case_kebab` and
`no_two_entries_of_any_table_share_an_identifier` in the crate root;
`an_earliest_evidence_row_keeps_the_shape_of_its_date` in the lines, and
the JavaScript binding's "the earliest evidence keeps the shape of each
source's date". The lines of every other deep-time export carry a
standard uncertainty on every bound
(`only_an_earliest_evidence_row_may_lack_a_standard_uncertainty`).
