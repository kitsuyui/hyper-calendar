# Coverage audit

An outside reading of what this repository actually covers, against what
[`calendars.md`](calendars.md) and [`observances.md`](observances.md) claim,
and against the world's calendars as the literature describes them.

**Snapshot: 2026-09-21, 01:14–01:40 JST.** The workspace was being edited
while this was written, so parts of it are already stale by design. Three
changes landed mid-audit and are noted where they matter: `observances.md` was
corrected, the zodiac rows in `calendars.md` were promoted to Done, and
`hc-calendars-lunar::japanese_historical` (Japan's seven pre-Tenpō systems)
appeared half-written. Nothing here is a reason to revert any of that.

This file is read-only output. It proposes no edits to any other document; it
records what a reader who trusted the tables would be surprised by, and what
the tables do not mention at all.

---

## 1. What was audited

- Every row of `calendars.md` and `observances.md`, checked against the
  `CalendarId` values the code registers, the `register_all` functions, the
  `hc-holiday` country and tradition tables, and the module lists of the
  calendar crates.
- `hc_calendar::Calendar`, `DynCalendar`, `CalendarMeta`, `DateFields` and the
  Rata Die pivot, read as an abstraction rather than as code, to judge which
  calendars they could and could not represent.
- The world's calendar systems by region, religion, era, kind and mechanism.
  Every claim below about how a calendar works carries a citation.

The verdict in one line: **the code is in better shape than the coverage
tables, the tables understate more often than they overstate, and the largest
gaps are no longer names on a list but four or five mechanisms the abstraction
cannot yet express.**

A note on the code's own honesty, since an audit that only lists faults
misleads: several things the research flagged as classic library mistakes are
already handled correctly and explicitly. `maya.rs` documents both the
Goodman–Martínez–Thompson correlation (584 283) and the 584 285 alternative,
and states in prose why it refuses to offer a switch. The Haabʼ implementation
numbers the day from **0**, the seating of Pop, which is the Classic-period
convention and the contested one. `buddhist.rs` says in its module doc that it
does not model the pre-1941 Thai year beginning on 1 April. `islamic_rgsa` is
marked Partial because its visibility model is a simplification. That standard
is why the findings below are worth acting on rather than arguing with.

---

## 2. Doc/code reconciliation

### 2a. Rows that claim more than the code delivers

| Claim | Where | What the code has | Severity |
| --- | --- | --- | --- |
| `ethioaa` — "Ethiopic (Amete Alem), Done" | `calendars.md` stage 1 | No `CalendarId("ethioaa")` exists. Amete Alem is an **extra field** on the Ethiopic calendar (`ethiopic.rs:170`, `"amete-alem-year"`). `registry.get(CalendarId("ethioaa"))` returns `None` | **High** — the row names a CLDR identifier the library does not answer to, which is exactly the interop the table's header promises |
| `persian`, `french-republican`, `bahai` | `calendars.md` stage 1 | Registered as `persian-arithmetic`, `french-republican-arithmetic`, `bahai-arithmetic` | **High** — and it contradicts the table's own header: CLDR's identifier *is* `persian`. The astronomical variants are stage-3 Planned, so the bare CLDR name is currently unclaimed by anything |
| `iso8601`, `ordinal`, `world` | `calendars.md` stage 1 | Registered as `iso8601-week`, `iso8601-ordinal`, `world-calendar` | Medium |
| `julian-gregorian` | `calendars.md` stage 1 | Not an identifier. Twelve per-polity calendars, `julian-gregorian-catholic` … `julian-gregorian-gr`, plus a constructor for arbitrary cut-overs (`julian_gregorian.rs:111`) | Medium — the row understates the feature *and* misnames it |
| "Julian Day Number / MJD" as one row, id `julian-day` | `calendars.md` stage 1 | Two registered calendars: `julian-day` and `modified-julian-day` | Low |
| "Thai solar (Buddhist Era) — Done" | `calendars.md` stage 1 | `buddhist.rs` documents that it **does not model 1889–1940**, when the Thai year began on 1 April, nor the Rattanakosin Sok era that preceded BE. The restriction is stated in the code and nowhere in the table | Medium — by the table's own vocabulary this is Partial, not Done |
| "十二直 and 二十八宿 — `hc-seasons` — Planned" | `calendars.md` stage 5 | Written, but in `hc-almanac`, whose `lib.rs` is `//! Placeholder.` at this snapshot, so `mansions.rs`, `twelve_directs.rs`, `seven_luminaries.rs`, `rules.rs` and `context.rs` are not compiled. The crate is a workspace member and the facade already has an `almanac` feature | Medium — the status is right by accident; the crate is wrong |
| "Add its vocabulary to `hc-i18n`" (step 5 of *Adding a calendar*) | `calendars.md` | `hc-i18n` has month/era tables for `gregory`, `japanese`, `buddhist`, `islamic`, `hebrew`, `chinese` only. `islamic` and `mayan` are not identifiers any calendar reports (`names.rs:913`, `names.rs:1080`), so a lookup keyed on a real `CalendarId` misses | Medium — most Done rows have not had step 5 done, and two i18n keys can never match |
| "The 19-crate workspace" | `README.md` §8 | Twenty members; `hc-almanac` is the twentieth | Low |
| `hc-almanac` | Absent from `calendars.md`, `architecture.md`'s crate graph and the README | Exists, is a workspace member, has a description and a facade feature | Low, but it is the only crate nothing documents |

`observances.md` **was** the worst offender when this audit began — it listed
Russia, Ukraine, Hungary, Romania and Argentina under **Done** with no country
table for any of them, marked Shinto **Partial** with no Shinto rule set
anywhere, and omitted the AE and NP tables that do exist. All five country
rows, the Shinto row and the two missing tables were corrected by a concurrent
change during the audit window. As of 01:22 JST the Done + Partial country list
and the 43 `CountryRules` codes in `hc-holiday/src/countries/` **match
exactly**, and the six `RuleSet` traditions match their rows. No action.

### 2b. Things the code does that the docs do not mention

| In the code | Documented as | Note |
| --- | --- | --- |
| `japanese-imperial` — 皇紀 / Kōki, `hc-calendars-solar/src/koki.rs`, registered and round-trip tested | **Nothing**. No row in any table | The only wholly undocumented calendar in the workspace |
| `aztec-tonalpohualli` and `aztec-xiuhpohualli`, registered, 631 lines, tested | "Aztec Tonalpohualli and Xiuhpohualli — **Planned**" | Two calendars, not one; the row also gives a single id `aztec` that nothing answers to |
| Byzantine **indiction** — the fifteen-year fiscal cycle, `byzantine.rs:85` | "Byzantine / Anno Mundi world era — Done" | A separate cycle riding on the calendar, and a genuinely non-astronomical one |
| `vietnamese-south-1968` — the 1968 Saigon/Hanoi meridian split | Not mentioned | The North moved to UTC+7 on 8 August 1967 while the South stayed on UTC+8, so Tết Mậu Thân fell on 29 January in Hanoi and 30 January in Saigon ([Vietnamese calendar](https://en.wikipedia.org/wiki/Vietnamese_calendar)). This is one of the sharpest single-calendar bifurcations anywhere and the table does not claim it |
| `japanese-tenpo-unbounded` | Not mentioned | The unbounded companion to the range-limited Tenpō calendar |
| Nengō table: 248 eras, including 17 Northern and 9 Southern Court entries and a `Certainty::Disputed` marker | "大化 → 令和, Done" | The Nanbokuchō split is the hard part and the row does not claim it |
| `ZiHourConvention` — the 23:00 day boundary for the four pillars, `cycle.rs:628` | Not mentioned | See §4.2; this is the only day-boundary machinery in the library |
| `GMT_CORRELATION` / `GMT_PLUS_TWO_CORRELATION`, with a written rationale for refusing a switch | Not mentioned | A deliberate, defended decision that the table does not record |
| `hc-seasons::zodiac` — tropical, sidereal with selectable ayanamsa, rāśi with Tamil/Bengali/Malayalam solar months, 十二次 | Was "Planned"; **promoted to Done during the audit** | Resolved |
| `hc-calendars-lunar::japanese_historical` — Genka, Gihō, Taien, Goki, Senmyō, Jōkyō, Hōryaku, Kansei | Not mentioned | **In flight at this snapshot**: four of eight sub-modules written. Not a gap |

---

## 3. Missing calendars

Only systems absent from *both* the code and the tables — not even as a
Planned or Researching row — are listed. Priority weighs value against work,
favouring calendars in current use and calendars whose *mechanism* is not
already represented. Modellability uses the project's own vocabulary:
**Yes**, **Approximate**, **Table-driven** (no closed form exists), and
**Reconstruction, sources conflict**.

### 3a. Africa

| Calendar | Region / era | What makes it distinct | Modellable | Sources | Priority |
| --- | --- | --- | --- | --- | --- |
| **Borana (Oromo)** | S. Ethiopia, N. Kenya; living | Lunar-**stellar**: the twelve months are fixed by the conjunction of a named moon phase with one of seven stars, not by the sun; no week at all; **27 named days** cycling through a 29/30-day month, so the first two or three names recur at month's end ([Wikipedia](https://en.wikipedia.org/wiki/Borana_calendar); Legesse, *Gada*, 1973; Doyle, "The Borana Calendar Reinterpreted", *Current Anthropology*, 1986) | Approximate — needs a star-conjunction ephemeris, ±1–2 days of observational slack | One strong ethnography, one archaeoastronomical critique (Ruggles), and a great deal of unreliable web repetition | **High** — a day-name cycle *shorter than the month* is a mechanism nothing in the library has |
| **Bahire Hasab (ባሕረ ሃሳብ)** | Ethiopian Orthodox Tewahedo; living | A computistical layer *above* the Ethiopic civil calendar: a 19-year Metonic cycle plus *Abektē* (the solar–lunar epact difference), *Metqē*, *Wenber* and *Amete Alem*, which together place Fasika and the movable fast cycle ([Wikipedia](https://en.wikipedia.org/wiki/Bahre_Hasab); [bahirehasab.org](https://bahirehasab.org/)) | Yes | Good in the Ge'ez tradition, thin in English scholarship | **High** — `ethiopic` is Done and Ethiopian holidays are not; this is the missing piece between them |
| **Berber / Amazigh agrarian (*fellāḥī*)** | Maghreb; living, rural | A surviving Julian calendar with Latin-derived month names, thirteen days off Gregorian, overlaid with agrarian sub-seasons ([Wikipedia](https://en.wikipedia.org/wiki/Berber_calendar)) | Yes for the arithmetic; the "Amazigh era" epoch of 950 BC is a 1960s construction and should be labelled as such | Good for structure, politicised for the era | Medium |
| **Swahili / Nairuzi (Mwaka Kogwa)** | Swahili coast; living festival | A 365-day nautical solar year kept in parallel with the Hijri year; the new year has drifted to late July, which is the evidence for a non-leap Persian-type year ([Encyclopaedia Iranica](https://www.iranicaonline.org/articles/east-africa-i-economic-political-and-cultural-relations-through-1900/)) | Approximate — the rule is inferred from the drift, not attested | Weak | Low |
| **Nuer / Dinka ecological reckoning** | South Sudan; living | Not a counted calendar: time is structured by transhumance phase, with month-names that are activity-names and no fixed boundaries (Evans-Pritchard, *The Nuer*, 1940) | **No** | One canonical ethnography, no formalisation | Out of scope — worth an explicit row saying why |

### 3b. The Americas

The Maya and Aztec machinery is implemented; what is missing is everything
*else* the same machinery would serve, plus one cycle the Maya rows do not
cover.

| Calendar | Region / era | What makes it distinct | Modellable | Sources | Priority |
| --- | --- | --- | --- | --- | --- |
| **Maya 819-day count** | Classic Maya | A four-station 819-day cycle whose closure was an open problem until 2023, when it was shown to complete over **twenty stations — 16 380 days, the LCM of 819 and 260 (63 Tzolkʼin)** — and to commensurate the synodic periods of all five naked-eye planets | Yes; it is pure arithmetic | **Excellent and recent**: Linden & Bricker, "The Maya 819-Day Count and Planetary Astronomy", *Ancient Mesoamerica*, 2023 ([Cambridge Core](https://www.cambridge.org/core/journals/ancient-mesoamerica/article/abs/maya-819day-count-and-planetary-astronomy/9839C2633BECD1356C94D4079E2580FE)) | **High** — four Maya calendars are Done and this fifth cycle is missing; it is also the cleanest example in this document of a multi-cycle product structure |
| **Mesoamerican 365-day vague year beyond Maya and Aztec** — Zapotec *piye*/*yza*, Mixtec, Purépecha *huriyata miucua*, Zoque *hame* | Mesoamerica | 18 × 20 + 5 unlucky days with **no intercalation at all**, so it slips a day every four years. The Zapotec *piye* is arguably the oldest 260-day count ([Mesoamerican calendars](https://en.wikipedia.org/wiki/Mesoamerican_calendars)) | Yes — the arithmetic is the implemented `maya-haab`/`maya-tzolkin` with different name tables | Excellent for the counts, adequate for the names | **High per unit of work** — this is a data exercise on an engine that already exists |
| **Mixtec year-bearer offset** | Oaxaca/Puebla, codices ~1000–1600 | Same 260/365 machinery, but **the Mixtec year-bearer day sits 40 days before the Aztec one**, so the same solar year carries a year-name one number lower (Aztec 2 Reed = Mixtec 1 Reed) ([Aztec calendar](https://en.wikipedia.org/wiki/Aztec_calendar); Jansen & Pérez Jiménez) | Yes given the offset | Good | **High** — a pure offset, and therefore a *silent wrong answer* for anyone who reuses the now-implemented Aztec calendars for Mixtec sources |
| **Living 260-day counts** — K'iche'/Kaqchikel/Ixil *Cholqʼij*, Mixe *xëë tun* | Highland Guatemala, Oaxaca; **current** | The day-count has run unbroken through five centuries of suppression even where the 365-day frame around it did not; the modern anchor differs slightly between communities (Tedlock, *Maya Daykeeping*; Ríos Cortés, [Zenodo 5541316](https://zenodo.org/records/5541316)) | Yes — a mod-260 count; the anchor is the question | Good academic | Medium — and a reminder that `maya-tzolkin` is a living calendar, not only an archaeological one |
| **Epi-Olmec Long Count** | Isthmian Gulf coast, from ≥36 BCE | The oldest attested Long Count is not Maya: Chiapa de Corzo Stela 2 reads 7.16.3.2.13 (36 BCE) in Epi-Olmec style, with the two highest digits reconstructed and a minority reading of 8.7.3.2.13 = 182 CE ([Mesoamerican Long Count](https://en.wikipedia.org/wiki/Mesoamerican_Long_Count_calendar)) | Yes — it is the implemented `maya-longcount` | Excellent | Low as code, worth a sentence in the Maya rows |
| **Inca / Quilla** | Cusco, ~1400–1533 | Two incompatible models: a state lunisolar 12×30+5, and Zuidema's reading of the **ceque system — 328 huacas on 41 radial lines as a 328-day sidereal-lunar year (12 × 27⅓)** plus 37 days of Pleiades invisibility, i.e. a calendar laid out in *space* ([Zuidema in Ruggles (ed.), *Handbook of Archaeoastronomy and Ethnoastronomy*, Springer 2015](https://link.springer.com/referenceworkentry/10.1007/978-1-4614-6141-8_79)) | Reconstruction, sources conflict | Good but contested | *Already a Researching row* — but the existing note ("no surviving written record") understates it: there is a substantial scholarly literature and it disagrees with itself |
| **Muisca (*zocam*)** | Altiplano Cundiboyacense; pre-1537 | Three concurrent year kinds of different lengths — rural (12–13 lunations with a "deaf month"), common (**20 months × 30 days = 600 days**, about 1.64 Gregorian years) and priestly (37 months) — on a vigesimal count ([Wikipedia](https://en.wikipedia.org/wiki/Muisca_calendar); Izquierdo Peña, [arXiv:0812.0574](https://arxiv.org/pdf/0812.0574)) | **Reconstruction, sources conflict.** Duquesne's 1795 account was long dismissed as invention and partly rehabilitated; chroniclers disagree on the week (10-day, 15-day, 3-day, or none at all), and two published versions of Duquesne differ | Poor | Low as code — but it deserves a Researching row, because a 600-day year is the sort of claim a library gets asked about |
| **Mapuche We Tripantu**, **Aymara Willkakuti** | Chile/Argentina, Bolivia/Peru; living, and a Bolivian national holiday since 2009 | 13 *küyen* of 28 days = 364, with an **observational** reset at the June solstice and the heliacal reappearance of the Pleiades; Willkakuti's boundary is the solstice sunrise observed at Tiwanaku's Gate of the Sun ([We Tripantu](https://en.wikipedia.org/wiki/We_Tripantu); [Willkakuti](https://en.wikipedia.org/wiki/Willkakuti)) | Approximate; the year counts attached to both are modern reconstructions | Thin | Low |
| **Lakota winter counts (*waníyetu wówapi*)**, **Anishinaabe thirteen moons**, **Hopi/Zuni horizon calendar**, **Barasana star calendar**, **Inuit and Nisgaʼa moon-months** | The Americas; living or recent | Four different reasons not to model: a winter count identifies each year by a **name chosen retrospectively** and has no day resolution (Greene & Thornton, *The Year the Stars Fell*); Anishinaabe moon-names **vary by community for the same lunation**; the Hopi/Zuni calendar is a function of **the observer's exact standing position** against named horizon landmarks, so villages legitimately differ ([Soyal](https://en.wikipedia.org/wiki/Soyal)); Barasana seasons are of unequal, variable length set by heliacal events (Hugh-Jones, *JSA* 1.1, 2015) | **No, as arithmetic** | Excellent ethnographically, absent computationally | Out of scope, explicitly — saying so is better than silence, because all of these appear on general "list of calendars" pages |

### 3c. Oceania

| Calendar | Region / era | What makes it distinct | Modellable | Sources | Priority |
| --- | --- | --- | --- | --- | --- |
| **East Polynesian lunar calendars as a family** — Hawaiian *Kaulana Mahina*, Māori *maramataka*, Marquesan, Tahitian, Tuamotuan, Rapa Nui | East Polynesia; living and revitalised | **~30 named nights per lunation against a 29.53-day month, so a name must periodically be dropped.** Hawaiian nights run in three ten-night *anahulu*, the month begins at first visible crescent after sunset (**sunset day boundary**), and a 13th month is intercalated to hold the *makahiki* to the sidereal year. Note that **West Polynesia numbers its nights instead of naming them** — a different data model for the same sky | Approximate | **Now unusually good**: Valério, Tamburini & Corazza, "Computational analysis reveals historical trajectory of East Polynesian lunar calendars", *PLoS ONE* 21(7), [doi:10.1371/journal.pone.0353287](https://doi.org/10.1371/journal.pone.0353287), analysing 48 night-name lists; Langlas, ["Nā Pō o ka Malama"](https://scholarspace.manoa.hawaii.edu/server/api/core/bitstreams/ce779376-821a-4847-a0ef-8688575e2fbd/content) | **Medium–high** — one shared mechanism and many name tables is precisely the data/algorithm split [policy.md](policy.md) §2 asks for |
| **Māori maramataka specifically** | Aotearoa; living | Year begins at **the first new moon after the heliacal rise of Matariki** — a double observational trigger. **Over forty iwi versions exist and they disagree on the month boundary itself**: most begin at new moon (Whiro), some at full moon (Rākaunui) ([Te Ara](https://teara.govt.nz/en/maramataka-the-lunar-calendar/print); [Te Papa](https://www.tepapa.govt.nz/digital-museum/explore-digital-museum/maori/matariki-maori-new-year/what-maramataka-maori-lunar)) | **Approximate at best — there is no single maramataka** | Excellent institutional sources for the concept, no canonical arithmetic | Low as a calendar. `hc-holiday` already computes New Zealand's *statutory* Matariki, which is a Crown schedule, not the maramataka; the honest table entry is "no canonical form to implement" |
| **Tongan calendar** | Tonga; pre-contact | Twelve named lunar months with a thirteenth, *Ooa-ki-fangongo*, **intercalated on a biological trigger**: if at the next new moon the yams and fish do not show the appearances proper to the first month, another month is inserted (Collocott, "Tongan Astronomy and Calendar", *Bishop Museum Occasional Papers* VIII(4), 1922, [archive.org](https://archive.org/stream/tonganastronomyc00collrich/tonganastronomyc00collrich_djvu.txt)) | **No** — the trigger is the state of organisms | One excellent primary source | Out of scope, but worth naming: it is the clearest case in this document of a rule that is perfectly well specified and still not computable |
| **Rapa Nui calendar** | Easter Island | 12 or 13 months of ~28–29 named nights with **one to two intercalary *nights* (*hotu*, *hiro*)** — intercalation at the night level rather than the month level. The *Mamari* rongorongo tablet is thought to encode it and is undeciphered ([Wikipedia](https://en.wikipedia.org/wiki/Rapa_Nui_calendar); [a note on the Rapanui lunar calendar](https://arxiv.org/pdf/1405.6092)) | Reconstruction, sources conflict — Thomson recorded 13 months, others 12 | Conflicting | Low |
| **Carolinian / Micronesian sidereal calendar** | Satawal, Puluwat, Woleai, Ulithi; living navigational tradition | Twelve or thirteen **"moons" that are independent of the moon**: each month is named for a star and they are of **unequal length**, tied to the 32-point star compass ([Penn Museum](https://www.penn.museum/sites/Navigation/predicting/predicting.html); Gladwin, *East Is a Big Bird*, 1970) | Approximate — heliacal risings compute, the month-length conventions are island-specific and largely unpublished | Thin for the calendar, good for the navigation | Low as code, **high as a category**: a non-lunar "lunar" calendar with unequal months is a shape nothing here can hold |
| **Australian Aboriginal seasonal calendars** | Continent-wide; living, ~20 published by the Bureau of Meteorology | Two to six or more seasons of **unequal and non-fixed length**, each triggered by ecological indicators — Nyoongar has six, D'harawal keys seasons to quoll calls and lilly-pilly ripening ([Bureau of Meteorology](https://www.bom.gov.au/resources/indigenous-weather-knowledge/indigenous-seasonal-calendars)) | **No** — they carry no date arithmetic | **Excellent** — government-published, which is unusual for this category | Out of scope, explicitly. They are well sourced and still not calendars in this library's sense, which makes them the right example to cite in the exclusion row |

### 3d. Southeast Asia

The Theravada cluster is the highest-value regional block in this document,
because `hc-holiday`'s `BUDDHIST` rule set says in its own `sources` string
that the Theravada full moons are "approximated from the Chinese lunisolar
calendar because no Thai, Burmese or Sinhalese lunar calendar exists in
`hc-calendars-lunar` yet". An already-shipped Partial is waiting on these rows.

| Calendar | Region / era | What makes it distinct | Modellable | Sources | Priority |
| --- | --- | --- | --- | --- | --- |
| **Javanese calendar (*Pananggalan Jawa*)** | Java; 1633–present, living | Sultan Agung's deliberate graft: the **Śaka year number continued onto an Islamic lunar year**, so the era has no founding event (1633 CE = 1555 AJ); **days begin at sunset**; an 8-year *windu* of named years with fixed 354/355 lengths; and a 15-*windu* **kurup** of 120 lunar years = 42 524 days, locking it to the tabular Islamic 30-year cycle ([Wikipedia](https://en.wikipedia.org/wiki/Javanese_calendar); ["An ethnoarithmetic excursion into the Javanese calendar", arXiv:2012.10064](https://arxiv.org/abs/2012.10064)) | Yes, with the kurup correction table | Good | **High.** The table lists only the Pasaran five-day week; the calendar the Pasaran belongs to is missing, and a 120-year correction cycle is longer than anything implemented |
| **Balinese Śaka (*sasih*)** | Bali; living | 12 *sasih* of a nominal 30 days (15 waxing *penanggal*, 15 waning *panglong*), reconciled by **ngunaratri — "minus one night": two lunar days are assigned to one solar day roughly every 63 days, so a date number is simply skipped.** Intercalary *mala masa* after month 11 or 12, constrained so Tilem Kapitu does not fall in December ([Wikipedia](https://en.wikipedia.org/wiki/Balinese_saka_calendar)) | Approximate — the ngunaratri placement and the mala masa decision rest with almanac authority | Good | **High** — the Pawukon is Done and the calendar beside it is unlisted; and see §4.1 |
| **Pranata Mangsa** | Java, Sunda, Bali; living farmer's calendar | A **solar** year of twelve *mangsa* of **deliberately unequal length, symmetric about the solstices**: 41, 23, 24, 25, 27, 43 \| 43, 26/27, 25, 24, 23, 41 days; year begins ~23 June; **Kawolu is the only leap-bearing season** ([Javanese calendar §Pranata mangsa](https://en.wikipedia.org/wiki/Javanese_calendar#Pranata_mangsa); ["Revisiting Javanese pranata mangsa", arXiv:2204.13893](https://arxiv.org/abs/2204.13893)) | **Yes, exactly** — fixed Gregorian anchors since 19th-century standardisation | Good | **High per unit of work.** It is arithmetic, it is in daily agricultural use, and twelve months of unequal length is a shape the library has not met |
| **Burmese / Myanmar (Kawza Thekkarit)** | Myanmar; epoch 22 March 638 CE, living | **Two intercalation objects that can co-occur**: a "little watat" inserts a second Waso (384 days); a "big watat" adds that month *and* an intercalary day appended to Nayon (385 days) — and the extra day is **never** inserted outside a watat year, which is the exact opposite of the Thai and Khmer rule. **Makaranta** (mean reckoning) and **Thandeikta** (1853, Mindon) are two live rule sets; constants change at ME 1217 and ME 1312 ([Wikipedia](https://en.wikipedia.org/wiki/Burmese_calendar); Yan Naing Aye, ["Algorithm, Program and Calculation of Myanmar Calendar"](http://cool-emerald.blogspot.com/2013/06/algorithm-program-and-calculation-of.html); Irwin, *The Burmese & Arakanese Calendars*, 1909) | Yes, with the published exception tables | **Excellent** | *Already Planned* — but it blocks a shipped Partial, so it should move up |
| **Thai lunar (Chulasakarat)** | Thailand; living (wan phra, royal ceremony) | Three mutually exclusive year types: *pakatimat* 354 d, *athikawan* 355 d (extra day on Month 7), *athikamat* 384 d (a second Month 8) — and **the Thai/Lao/Khmer prohibition on combining the extra day and the extra month** is the main reason Thai and Burmese dates diverge. Days are recorded as **(waxing 1–15) or (waning 1–14/15)**, a two-field value rather than 1..30 ([Wikipedia](https://en.wikipedia.org/wiki/Thai_lunar_calendar)) | Yes | Good | *Already Planned* — same reason |
| **Khmer Chhankitek, Lao, Sinhalese, Tai/Shan/Dai** | Mainland and island SE Asia; living | Siblings of the above with their own rules: Khmer always doubles Ashadha and always adds the leap day in Jyestha; Tai and Shan systems **number months rather than naming them, running about two ordinals ahead of the Burmese sequence** — a silent cross-conversion trap | Yes as variants | **Thin on Wikipedia** (there is no English article for the Lao, Shan or Dai calendars), **good** in Gislén & Eade, "The Calendars of Southeast Asia 2", *JAHH* 22(3), 2019 ([Lund](https://portal.research.lu.se/en/publications/the-calendars-of-southeast-asia-2-burma-thailand-laos-and-cambodi/)) and Eade, *The Calendrical Systems of Mainland South-East Asia*, Brill 1995 | Medium |
| **Thai solar before 1941 — Rattanakosin Sok and the April new year** | Thailand, 1888–1940 | R.S. 1 = 6 April 1782 with a 1 April new year; the switch to Buddhist Era in 1912 kept that new year; the 1941 move to 1 January made **BE 2483 a nine-month year (April–December 1940)**, and the conversion offset is **BE − 542, not − 543, for 1 January to 31 March in pre-1941 years** ([Wikipedia](https://en.wikipedia.org/wiki/Thai_solar_calendar)) | Yes, exactly | Excellent | **High** — the code already names this as its own gap, and a nine-month year is the best regression test in this section |
| **Cham Sakawi**, **Sundanese Kala Sunda**, **pre-colonial Filipino reckoning** | Vietnam/Cambodia, West Java, the Philippines | Cham runs **two parallel calendars for one people**, Ahier (lunisolar) and Awal (Islamic months on an eight-year intercalation cycle), reconciled by priestly decision; Kala Sunda runs a lunar and a solar system simultaneously; pre-colonial Filipino reckoning has **no year number at all**, only named unequal daylight intervals (W. H. Scott, *Barangay*, 1994) | Reconstruction / not modellable | Thin; no English Wikipedia article exists for the Sundanese calendar | Low — Researching or Out-of-scope rows |

### 3e. South Asia

`hindu-lunar`, `hindu-solar`, `hindu-old`, `bikram-sambat` and `nepal-sambat`
are already Planned rows, so most of this section is *scoping* those rows
rather than adding new ones — but the scoping matters, because they are not
one calendar each.

| Calendar | Region / era | What makes it distinct | Modellable | Sources | Priority |
| --- | --- | --- | --- | --- | --- |
| **The sankranti→civil-day assignment rules** | Pan-Indian | This is the axis, not a calendar. **The same astronomical instant produces four different civil dates**: Orissa assigns it to the same day unconditionally; Tamil to the same day if before **sunset**; Kerala if before **aparāhṇa, three-fifths of the sunrise-to-sunset interval**; Bengal takes the *following* day if the transit falls between sunrise and **midnight**, and the *third* day if between midnight and sunrise. South Indian systems treat sankranti as the month's **start**, Bengali and Assamese as its **end** ([Nirayana system](https://en.wikipedia.org/wiki/Nirayana_system); [Sankranti](https://en.wikipedia.org/wiki/Sankranti)) | Approximate — three of the four need sunrise and sunset at a reference location | Good | **Highest in this section.** Any `hindu-solar` implementation must choose one of these, and the choice is the calendar |
| **Bangladeshi national calendar (Bangabda)** | Bangladesh; official | **Purely arithmetic and Gregorian-locked, with no astronomy at all.** The 1966 Shahidullah scheme (months 1–5 of 31 days, 6–12 of 30, Falgun 31 in leap years) was adopted in 1987; the **2018 decision, effective 16 October 2019**, moved Ashshin to 31 days and made Falgun 29/30 so the calendar tracks Gregorian 29 February exactly. Fixed anchors: Pohela Boishakh = 14 April, 8 Falgun = 21 February. Era offset is **−594 before Pohela Boishakh and −593 after**, inside one Gregorian year ([Wikipedia](https://en.wikipedia.org/wiki/Bangladeshi_national_calendar)) | **Yes** — but you need both closed forms, 1987–2019 and post-2019 | Good, with a datable statute | **High** — a currently official state calendar, fully arithmetic, absent from every table |
| **West Bengal Bangabda** | India; living | **Same era number, different calendar**: sidereal solar with astronomically determined month lengths under the Bengal sankranti rule, plus a purnimanta lunar layer. It routinely differs from the Bangladeshi calendar **by a day** ([Bengali calendars](https://en.wikipedia.org/wiki/Bengali_calendars)) | Approximate — panjika houses disagree (Surya Siddhanta vs Drik) | Good on structure, weak on authoritative arithmetic | Medium, and it must not share an identifier with the Bangladeshi one |
| **Tamil**, **Malayalam Kollavarsham**, **Odia**, **Assamese Bhāskarābda**, **Tripuri**, **Tulu** | South and East India; living | Regional sidereal solar calendars differing chiefly in the sankranti rule above and in their epochs. Kerala's **aparāhṇa** cutoff is the only three-fifths-of-daylight rule in the world; Malayalam has **two competing new years inside one system** (official Chingam 1 vs astronomical Vishu); Assamese is explicitly **sunrise-to-sunrise** with a contested epoch (593 or 594 CE); Tripuri is Bengal's calendar shifted three years | Approximate; Tulu is weakly sourced | Good for Tamil, Malayalam and Odia; moderate for Assamese; **weak for Tulu** (a stub plus community sources — low confidence) | Medium — mostly name tables and one rule flag over a shared engine. **`hc-seasons` already ships Tamil, Bengali and Malayalam solar months over the rāśi**, so the naming half exists |
| **Odia Anka year** | Odisha regnal reckoning; still printed for the Gajapati of Puri | **A year sequence with deliberate gaps**: every year ending in 6 is skipped, as is every year ending in 0 except 10, and year 1 does not exist. So 1, 6, 16, 20, 26, 30, 36… are absent ([Wikipedia](https://en.wikipedia.org/wiki/Anka_year)) | **Yes** — a pure integer mapping | Good | Medium — and an excellent unit test, because it breaks `days_in_year(y) - days_in_year(y-1)` reasoning outright (§4.5) |
| **Vikram Samvat, Indian** | North and West India | **Two incompatible month-start conventions — amānta (new-moon, southern) and pūrṇimānta (full-moon, northern) — which change both the month boundaries and the era offset** (56 vs 57 BCE) ([Wikipedia](https://en.wikipedia.org/wiki/Vikram_Samvat)) | Approximate; both must be supported | Good | Medium — note that `calendars.md` lists "Hindu lunisolar (Amanta and Purnimanta)" as one row, which is right |
| **Bikram Sambat, Nepal** | Nepal; official since 1901 | **Not the Indian calendar of the same name.** It is sidereal *solar*, with month lengths varying from 29 to 32 days set by actual rāśi-transit durations against UTC+05:45, and **there is no published closed form**: the authoritative month-length table is fixed annually by the Nepal Panchanga Nirnayak Samiti, whose ruling is final and **can differ by a day from independent computation** ([Vikram Samvat](https://en.wikipedia.org/wiki/Vikram_Samvat)) | **Table-driven only** | The official body exists; third-party descriptions are numerous and mutually inconsistent | **High as a scoping correction.** `bikram-sambat` is Planned as though it were an algorithm. It is a table with a supported range, exactly like `islamic-umalqura` — which this library already models correctly, so the pattern exists |
| **Nepal Sambat** | Kathmandu Valley; epoch 20 October 879 CE | Lunisolar and **tithi-based**, with the new year at Mha Puja during Swanti — tied to the **Tihar/Diwali lunar position** rather than any solar transit, so it is the one South Asian new year falling in October–November. A **separate solar version was created in 2020** for administrative use ([Wikipedia](https://en.wikipedia.org/wiki/Nepal_Sambat)) | Approximate for the lunar form; the 2020 solar variant is poorly documented | Moderate, conflicting | Medium |
| **Sinhalese Aluth Avurudu** | Sri Lanka | **The year does not begin at a day boundary at all.** The *nonagathe* ("neutral period") of about **12 hours 48 minutes** spans the Meena→Mesha transit, and the new year dawns at its **midpoint** — a published clock instant. The operative times are issued annually by the Ministry of Buddhasasana as the *Avurudu Nekath Seettuwa* ([Wikipedia](https://en.wikipedia.org/wiki/Sinhalese_New_Year)) | **Table-driven**, and see §4.10 | Good for the published times, weak for the algorithm | Medium — Sri Lanka also has a separate **poya** full-moon public-holiday cycle that `hc-holiday` would want |
| **Nanakshahi (Sikh)** | Punjab; epoch 1469 CE | Purewal's design is **tropical** — the only tropical solar calendar in South Asia, where everything else is sidereal — and fully arithmetic: five months of 31 days then seven of 30, leap day in Phagun, 1 Chet fixed at 14 March. Adopted by the SGPC in **2003**; the **2010 amendment made month starts movable and reverted three major festivals to lunar dates**, and 2014 modified it again. "Mool Nanakshahi" (all-fixed, 2003) remains in use by other bodies ([Wikipedia](https://en.wikipedia.org/wiki/Nanakshahi_calendar)) | Yes for the frame; **you must pick a variant and name it** | Good | **High** — `observances.md` has a Planned Sikh row that cannot be filled without choosing here, and the "sources conflict" vocabulary already exists for exactly this |
| **Vira Nirvana Samvat (Jain)** | India; the oldest era still in use | Epoch 7 October 527 BCE in the Śvetāmbara reckoning and **662 BCE in the Digambara** ([Wikipedia](https://en.wikipedia.org/wiki/Vira_Nirvana_Samvat)) | Yes once a Hindu lunisolar base exists; two epochs, not one | Good | Low — blocked, but `observances.md` has a Planned Jain row that needs it |
| **Kali Yuga *ahargana*** | The substrate, not a civil calendar | Epoch **midnight at the Ujjain meridian, 17/18 February 3102 BCE**, supplying the elapsed-day count on which all Surya Siddhanta computation rests ([Wikipedia](https://en.wikipedia.org/wiki/Kali_Yuga)) | Yes — a fixed JD constant | Excellent | **This is the natural internal day-number origin for the whole Hindu family**, and the note that it is an Ujjain-meridian midnight rather than a UT one belongs in the implementation before it starts |

### 3f. Central and North Asia

| Calendar | Region / era | What makes it distinct | Modellable | Sources | Priority |
| --- | --- | --- | --- | --- | --- |
| **Tibetan — Tsurphu, Kālacakra, Geden, Bhutanese** | Tibet, Bhutan, Mongolia, Buryatia | `calendars.md` lists **`tibetan` (Phugpa)** as though it were one calendar. It is a family: **Tsurphu** mixes rule sets (full-tenet *grub rtsis* for weekdays, précis *byed rtsis* for intercalation) and **differs from Phugpa by a whole day or a whole month**; **Kālacakra** is the parent system whose two sub-methods give different answers; **Geden** is Phugpa with a different starting point in the 60-year cycle, so the *set of skipped and doubled dates* differs; **Bhutanese** changes several core variables and the weekday computation after Pema Karpo | Yes for Phugpa, Tsurphu and Geden; approximate for Bhutanese (constants not well published in English) | **Excellent** — Janson, *Tibetan Calendar Mathematics*, [arXiv:1401.6285](https://arxiv.org/abs/1401.6285); Henning, *Kālacakra and the Tibetan Calendar*; Gantumur, *Possible Reforms of the Tibetan Lunisolar Calendar*, [arXiv:2604.01233](https://arxiv.org/abs/2604.01233) | **High as a scoping correction** — one row is being asked to carry four parameter sets. But see §4.1: none of them fits `DateFields` yet |
| **Mongolian (*Bilgiin toolol*)** | Mongolia, Inner Mongolia, Buryatia; Tegüs Buyantu system, 1747 | A **re-epoched Phugpa**: same skip/double machinery, different start in the 60-year cycle, so a different set of doubled and omitted dates. Tsagaan Sar is the **second new moon after the winter solstice**, which is not the Chinese rule and can differ by a day or a month. The Inner Mongolian "yellow calculation" uses Chinese-style months with **no doubled or omitted dates at all** ([Mongolian calendar](https://en.wikipedia.org/wiki/Mongolian_calendar); [Tsagaan Sar](https://en.wikipedia.org/wiki/Tsagaan_Sar)) | Yes — a parameter set over the Phugpa engine | Moderate | Medium — nearly free once Tibetan exists, which is the argument for building Tibetan as a parameterised engine rather than a calendar |
| **Turkic twelve-year animal cycle** | Old Turkic khaganates from the 8th c.; Uyghur, Qypchaq, Chagatai, Ottoman, Azerbaijani into the 20th c. | **A bare 12-year naming cycle with no 60-year stem/element product** in most attestations — a *label* riding on whatever local reckoning applies, and attaching to different year boundaries in different regions (Azerbaijani usage starts at Nowruz, not at a new moon). Animal substitutions are regionally diagnostic: dragon becomes a fish or crocodile in Old Turkic and Azerbaijani ([Azerbaijani calendar beliefs](https://en.wikipedia.org/wiki/Azerbaijani_calendar_beliefs); Kāshgarī, *Dīwān Lughāt al-Turk*) | Yes **as a labelling cycle**; not as a calendar | Good for the naming, poor for the anchoring | Medium — and it belongs with `sexagenary` in `hc-calendar::cycle`, not in a calendars crate |
| **Yakut / Sakha (Саха ыйдара)** | Sakha; largely displaced by Gregorian | Observationally triggered lunisolar: **the year begins in late May at the first cuckoo call**, the Pleiades' position against the moon's age is used to re-sync lunar to solar, and an extra month is inserted into summer every three years ([Yhyakh](https://en.wikipedia.org/wiki/Yhyakh); [Open University](https://www.open.ac.uk/blogs/religious-studies/?p=1747)) | **No** — the triggers are a bird call and an asterism | Weak; 18th–19th-century expedition ethnography | Out of scope, with a reason |
| **Bulgar (Proto-Bulgarian)** | Bulgar khanates, 7th–9th c.; extinct | A 12-year animal cycle paired with an ordinal month number, giving dates of the form "year-animal + month-ordinal" with **no day resolution** ([Wikipedia](https://en.wikipedia.org/wiki/Bulgar_calendar)) | **Reconstruction, sources conflict — do not implement.** The cyclic reading is Mikkola's 1913 hypothesis; Pritsak and Moskov modify it; Dobrev rejects the Turkic basis for an Iranian one. Several year-names are unattested and there is no agreed absolute anchor | Poor: one 15th-century transcript with ten date pairs, a marginal note and an inscription | Out of scope — a good worked example of the project's "sources conflict" verdict |
| **Solar Hijri in Afghanistan** | Afghanistan, official 1957– | The same astronomical calendar as Iran's, but with **Arabic-Islamic month names where Iran uses Zoroastrian ones** — a locale flag over shared arithmetic. **Status note: in 2022 the Taliban administration moved official business to the lunar Hijri calendar**, effective 1 Muharram 1444 AH, so "Afghanistan's official calendar" is time-dependent ([Wikipedia](https://en.wikipedia.org/wiki/Solar_Hijri_calendar)) | Yes — this is the already-Planned `persian-astronomical` with a different name table | Excellent | Medium — cheap once stage 3 lands, and the 2022 change is a `ValidFrom` case for `hc-holiday` |

### 3g. Europe and the Near East

| Calendar | Region / era | What makes it distinct | Modellable | Sources | Priority |
| --- | --- | --- | --- | --- | --- |
| **Revised Julian (Milanković)** | Orthodox churches, 1923– | Leap rule **mod 900**: a century year is leap only when it leaves 200 or 600 on division by 900, giving 218 leap days per 900 years and a mean year of 365.242222 days. It **first diverges from Gregorian in 2800**, and several churches use it for fixed feasts while keeping the *Julian* Paschalion — a single calendar with two rules inside it ([Wikipedia](https://en.wikipedia.org/wiki/Revised_Julian_calendar)) | Yes, trivially | Excellent | **Highest of any single calendar in this document.** In current official use by eight autocephalous churches, an arithmetic one-liner given `gregorian.rs`, and an excellent far-future regression vector |
| **Old Icelandic *misseri*** | Iceland, c. 930– | **The week is the unit and the month is derived from it**: 52 weeks = 364 days, two *misseri* of 26 weeks, every month always starting on the same weekday, the year beginning the first Thursday after 18 April, and a **leap week (*sumarauki*)** rather than a leap day. Two rule regimes, pre- and post-1700 ([Janson, "The Icelandic Calendar"](https://www2.math.uu.se/~svantejs/papers/calendars/iceland.pdf); Reingold & Dershowitz, *Calendrical Calculations: The Ultimate Edition*, ch. 6) | Yes | **Excellent** — a rigorous standalone paper and a textbook chapter | **High** — the best available stress test for an API that assumes year → month → day |
| **Swedish calendar, 1 Mar 1700 – 30 Feb 1712** | Sweden-Finland | A third calendar, neither Julian nor Gregorian: leap days were to be dropped from 1700 to 1740, 1700's was dropped, then 1704's and 1708's were kept **by error**, leaving Sweden a day ahead of Julian and ten behind Gregorian; the scheme was abandoned by giving 1712 *two* leap days, producing **30 February 1712** ([Wikipedia](https://en.wikipedia.org/wiki/Swedish_calendar)) | Yes, exactly — a finite twelve-year table | Excellent and unambiguous | **High.** `julian-gregorian-se` currently models Sweden as a single 1753 cut-over, which is right for 1753 and wrong for those twelve years. See §4.4 |
| **Soviet revolutionary week (*nepreryvka*)** | USSR, 1929–1940 | The Gregorian *date* was kept and the **week** was replaced: from 1929 a five-day continuous week with the workforce split into five colour groups on staggered rest days, then from 1931 a six-day week resting on the 6th, 12th, 18th, 24th and 30th, so the 31st was an extra working day ([Wikipedia](https://en.wikipedia.org/wiki/Soviet_calendar)) | Yes for the dates; **the rest day is a property of a person, not of a date** | Excellent | Medium as coverage, **high as a test**: it is the sharpest available challenge to "weekday is derivable from `Rd` alone" (ADR 0001) |
| **Qumran / Jubilees 364-day calendar with the *mishmarot*** | Judaea, 3rd c. BCE – 1st c. CE | 364 days exactly, **four quarters of 91 days = 13 whole weeks**, so every festival falls on a fixed weekday forever and **no intercalation is specified at all**; the 4Q320–330 *mishmarot* roster cycles the 24 priestly courses against it over a six-year period inside a 294-year *otot* cycle ([Wikipedia](https://en.wikipedia.org/wiki/Qumran_calendrical_texts)) | Yes — the most purely arithmetic calendar on this page | Excellent; the scrolls are published | **High** — a fixed-weekday year and a 24-name cycle riding on a six-year period is a structure nothing here has |
| **Samaritan** | Nablus and Holon; living, ~800 adherents | Lunisolar and Hebrew-like, but on an independent conjunction calculation kept by the High Priest's house, a different epoch, and **no postponement rules (*dehiyyot*)**, so festival dates routinely differ from Rabbinic ones | Approximate — Reingold & Dershowitz give an algorithm, but the operative calendar is still issued by the priesthood | Moderate | Medium — the *absence* of dehiyyot makes it a useful sibling to the implemented `hebrew` |
| **Mandaean** | Iraq, Iran, diaspora; living | 365 days with **no leap rule at all**, twelve 30-day months named for zodiac constellations, and the five epagomenal *Parwanaya* days inserted **mid-year, after the eighth month** rather than at the end ([Wikipedia](https://en.wikipedia.org/wiki/Mandaean_calendar)) | Yes, trivially, once the epoch is fixed | Moderate | Medium — cheap, and a genuine variation on the Egyptian/Coptic/Armenian pattern already implemented |
| **Yazidi (Serê Sal)** | Iraq, Syria, Caucasus; living | Julian/Seleucid-based, with a new year defined by a **weekday rule rather than a date**: the first Wednesday on or after 14 April Gregorian ([Wikipedia](https://en.wikipedia.org/wiki/Yazidi_New_Year)) | Yes | Moderate | Medium — and `hc-holiday` now has `WeekdayOnOrAfter`, so the rule vocabulary already fits |
| **Modern Assyrian** | Assyrian diaspora, 1950s– | Gregorian-aligned with Akkadian month names and a constructed 4750 BC epoch | Yes — a pure Gregorian offset | Good for the modern form; the ancient *limmu* eponym-year system is a different and much harder object | Low, but near-free |
| **Syriac / Chaldean / Church of the East liturgical year** | Iraq, Syria, Kerala; living | The year is divided into **seven-week seasons (*shawue*)** anchored to Easter — a 7×7 superstructure, not a feast list ([PDF](https://bethkokheh.assyrianchurch.org/wp-content/uploads/2020/08/The-Liturgical-Year.pdf)) | Yes, given a computus | Good, ecclesiastical | Medium — it belongs in `observances.md`, which has no row for it |
| **Seleucid era and the Macedonian calendar** | Near East, 312 BC– | Macedonian month names on the Babylonian lunisolar mechanism, and **the same Seleucid year number begins on two different dates** depending on whether the court (autumn) or Babylonian (spring) reckoning is used. The first continuous numbered era in history ([Seleucid era](https://en.wikipedia.org/wiki/Seleucid_era)) | Yes given Parker & Dubberstein's tables; the dual new year is the trap | Excellent (Parker & Dubberstein, *Babylonian Chronology 626 B.C.–A.D. 75*) | Medium — `babylonian` is already Researching and this rides on it |
| **Ancient Egyptian lunar** | Egypt, 3rd millennium BC | A second, religious calendar beside the implemented civil wandering year, whose month begins **on the morning when the waning crescent can no longer be seen** — the opposite trigger from the Islamic first-visibility rule (Parker, *The Calendars of Ancient Egypt*, SAOC 26, [free from ISAC](https://isac.uchicago.edu/research/publications/saoc/saoc-26-calendars-ancient-egypt)) | Approximate | Good; one canonical monograph | Medium — a new observational trigger is worth more than a new offset |
| **Runic calendar / *primstav*** | Scandinavia, 13th–18th c. | A perpetual calendar carved on a stave: 19 golden-number runes against 7 dominical-letter runes, so one object serves every year ([Wikipedia](https://en.wikipedia.org/wiki/Runic_calendar); Ole Worm, *Computus Runicus*, 1643) | Yes | Good; a published primary source | Medium |
| **Pentecontad** | Western Mesopotamia and the Levant | Seven periods of fifty days — seven weeks of seven plus an *atzeret* — totalling 350, with a 15- or 16-day supplement ([Wikipedia](https://en.wikipedia.org/wiki/Pentecontad_calendar)) | **Reconstruction, sources conflict.** Identified by Julius and Hildegard Lewy in the 1940s; later work on Old Assyrian and Babylonian material finds no evidence for the Amorite origin | Poor | Low — a Researching row |
| **Shang oracle-bone reckoning** | China, 14th–11th c. BC | Days counted in the sexagenary cycle (the ancestor of the implemented `sexagenary`), with intercalation by appending a **13th month — and, in attested cases, a 14th and a 15th** ([Sexagenary cycle](https://en.wikipedia.org/wiki/Sexagenary_cycle)) | Reconstruction; the day count is continuous and reliable, the year structure is not | Good for the day cycle, contested for the rest | Low — and note it breaks "an intercalary month repeats the previous one" |
| **Slavic folk, Baltic, Basque, Druze** | Europe and the Levant | **Not distinct calendar systems.** Slavic and Baltic folk calendars are seasonal overlays on the Julian or Gregorian year with nature-derived month names; Basque month names are a naming layer; Druze observance runs on the Hijri calendar plus fixed Gregorian dates | N/A | The Slavic case additionally attracts **fabricated claims** (nine-day weeks, 41-day months) with no chronicle or archaeological support | Out of scope. Worth naming, because the pseudo-history is easy to find and hard to unsee |

### 3h. Reform calendars

The World Calendar and Symmetry454 are both Done, so this category is opened
but not filled. The whole family shares one mechanism the library does not yet
have: a **leap week**. A leap-week calendar's year is 364 days (52 weeks) or
371 (53), never 365 or 366, so every date keeps a fixed weekday at the cost of
±6–7 days of seasonal drift. Two leap rules are in circulation — the ISO one
(71 leap weeks per 400 years, the year has 53 Thursdays), which requires
carrying a Gregorian implementation alongside, and Bromberg's standalone
**52/293**, whose mean year of 365 + 71/293 ≈ 365.242321 days targets the mean
**March-equinox** year rather than the mean tropical year, and is therefore
more accurate than Gregorian's 365.2425 for that purpose
([Palmen, "Leap Week Calendars"](https://www.hermetic.ch/cal_stud/palmen/lweek1.htm)).
This is the same mechanism the Old Icelandic *sumarauki* needs, which is an
argument for building it once.

| Calendar | Rule | Modellable | Priority |
| --- | --- | --- | --- |
| **Symmetry010** | Bromberg's other calendar: the **same 52/293 leap rule as the implemented Symmetry454** — leap when `(52·Y + 146) mod 293 < 52` — but quarters of 30+31+30 days instead of 4+5+4 weeks ([Wikipedia](https://en.wikipedia.org/wiki/Symmetry454); [Bromberg's site, which has moved](https://kalendis.free.nf/symmetry.htm)) | Yes; the leap rule is **already written**, at `symmetry454.rs:62`. ThreeTen-Extra's `Symmetry010Date` is an independent oracle to test against | **Highest of any calendar in this document per unit of work** |
| **International Fixed (Cotsworth, 1902)** | 13 × 28 days with *Sol* between June and July; *Year Day* and *Leap Day* both **outside the week**; **leap rule is plain Gregorian**. The canonical "13-month calendar"; Kodak ran it from 1928 to 1982 | Yes. Genuine free primary source: Cotsworth, [*The Rational Almanac* (1905)](https://archive.org/details/rationalalmanact00cotsuoft) | Medium |
| **Hanke–Henry Permanent** | Quarters of 30+30+31 = 91 days, a 364-day year always starting Monday 1 January, and a 7-day month *Xtr* after December **on exactly the ISO 53-week rule** ([Henry, JHU](https://henry.pha.jhu.edu/calendar.html)) | Yes — and note the JHU page is **internally inconsistent**: one answer still places *Xtr* between June and July, a fossil of its CCC&T predecessor. Implementing from that page alone puts the leap week in the wrong place | Medium |
| **Positivist (Comte, 1849)** | 13 × 28 = 364, day 365 the *Fête générale des Morts* outside the week, leap adds a second such day; **leap rule is plain Gregorian**; era year 1 = 1789 | Yes on structure. The primary is the *Catéchisme Positiviste* (1852) on Gallica; positivists.org fails its TLS handshake | Low |
| **Pax (Colligan, 1930)** | 13 × 28 = 364 with *Columbus* between November and December; leap is a **7-day month *Pax*** when the last two digits are divisible by 6 (including 00) or equal 99, except years divisible by 400 → 71 leap weeks per 400 years | Arithmetic is implementable, but **the leap rule as circulated may be a later reconstruction rather than Colligan's wording** — the primary is offline and the usual secondary link is dead | Low, and say so if implemented |
| **Meyer–Palmen Solilunar (1999)** | `cycle-year-month-day` over 60-year cycles; odd months 29 days, even 30, a 13th month *Meton* of 30–31. With Y = 6840, L = 2519, M = 1328: year *y* of cycle *c* is long iff `((60c+y)·L) mod Y < L`, and if long *Meton* has 31 days iff `(⌊((60c+y)·L)/Y⌋·M) mod L < M` ([Hermetic Systems](https://www.hermetic.ch/cal_stud/nlsc/nlsc.htm)) | **Yes — fully specified, purely arithmetic, with an explicit epoch anchor** (000-01-01-01 MP = JDN 207 227) and stated invariants that convert directly into property tests | Medium. Single self-published source, but it is the most implementation-ready specification in this whole table |
| **Yerm lunar (Palmen)** | Abandons the solar year entirely: months alternate 30/29 days within a *yerm*; yerms have 17 months except every third, which has 15; the cycle is **52 yerms = 850 months = 25 101 nights**. Dates begin at 12:00 local so a night is not split ([Hermetic Systems](https://www.hermetic.ch/cal_stud/palmen/yerm1.htm)) | Yes | Low — but it is the only purely lunar reform proposal here, and it makes **no** seasonal claim, so it must not be tested against equinoxes |
| **Tranquility (Siggins, 1989)** | 13 × 28 days named for scientists, weeks running Friday→Thursday; *Armstrong Day* outside the week; *Aldrin Day* inside Hippocrates on the 4/100/400 rule; epoch 20 July 1969, eras AT/BT | Partial — the primary is *Omni*, July 1989, print only, and the main fan reference has an expired certificate | Low |
| **Invariable Calendar** | **The usual attribution is muddled.** The "Invariable Calendar" is **Grosclaude's (Geneva, 1900)**: four 91-day quarters of 30+30+31 with a blank New Year's Day. **Mastrofini (1834) is a separate, earlier proposal** — a 364-day year always starting Sunday with the 365th day extra-calendrical | Grosclaude partial; **Mastrofini: no usable source at all** (no title, no publisher, and its only external reference is a dead link) | Low — and do not present the Mastrofini structure as established |
| **Raventós Symmetrical Perpetual** | 13 × 28 days with "Vacational" between July and August, each month starting Monday | **No — the author cannot be identified.** The only artifact is a JavaScript converter whose own header comment reads `// based on the algorithms of ?` | Do not implement, or implement strictly as "the behaviour of that converter", with the caveat in the doc comment. It fails the project's own step 4, "anchor it to at least one published reference" |
| **Dreamspell / Thirteen Moon** | Argüelles's 13×28+1 calendar, **widely mistaken for the Maya Tzolkʼin**, from which it deliberately differs by skipping 29 February, so the two counts drift apart ([Wikipedia](https://en.wikipedia.org/wiki/Dreamspell)) | Yes | **Add as an explicit Out-of-scope row.** `maya-tzolkin` is Done, users will ask why it disagrees with their Dreamspell app, and the answer belongs in the table rather than in an issue |

One practical note for whoever writes these rows: several of the canonical URLs
for reform calendars are dead or, worse, hijacked.
`individual.utoronto.ca/kalendis/*` now 404s (Bromberg retired in 2018);
`worldcalendar.org` is a parked domain for sale; **`theworldcalendar.org` has
been repurposed into an SEO content farm and must not be cited as the World
Calendar Association**; `myweb.ecu.edu/mccartyr/*` is gone, which breaks the
usual primary for both Pax and Mastrofini. The archive.org copy of Cotsworth
and the Hermetic Systems pages are the durable ones.

### 3i. On the fictional-calendar exclusion

`calendars.md` excludes "fictional calendars from specific works (Shire
Reckoning, Stardates, Imperial Dating)" because they are "copyrighted
settings". **The exclusion is drawn on the wrong axis, and the project does not
apply it to itself.** What follows is a sourced engineering summary, not legal
advice.

**A calendar system is not copyrightable subject matter, and the US Copyright
Office says so in as many words.** 17 USC 102(b) excludes "any idea,
procedure, process, system, method of operation, concept, principle, or
discovery, *regardless of the form in which it is described, explained,
illustrated, or embodied*" — the codification of *Baker v. Selden*, 101 U.S. 99
(1879), which held that copyright in a book explaining a system reaches only
the author's explanation and not the system.
[37 CFR 202.1](https://www.copyright.gov/title37/202/37cfr202-1.html) then
names the case outright: subsection (b) excludes "ideas, plans, methods,
systems, or devices", and subsection (d) excludes "works consisting entirely of
information that is common property containing no original authorship, such as,
for example: **Standard calendars**, height and weight charts, tape measures
and rulers, schedules of sporting events". *Feist v. Rural*, 499 U.S. 340
(1991), forecloses any "sweat of the brow" claim over a conversion table. The
nearest thing to an on-point fight is the Klingon one: in *Paramount v.
Axanar*, Paramount asserted copyright in the Klingon language, the Language
Creation Society filed an [amicus brief](https://conlang.org/axanar/) arguing a
language is a system rather than expression, and **Paramount withdrew the claim
before the court reached it**. No case law on calendar or date-conversion
software appears to exist at all.

What *is* protected is a different list: the prose, tables and diagrams of the
source work (Tolkien's Appendix D text, a rulebook's calendar page); fictional
facts; and — weakly, on a Feist selection-and-arrangement theory — a curated
set of invented month and day names. Trademarks ("Star Trek", "Middle-earth",
"Warhammer 40,000") are a separate and, in practice, more operative
consideration, and they govern what a crate may be *called* rather than what it
may compute.

**The project already contradicts its own row.** `calendars.md` lists the
**Darian** Martian calendar as **Done** — a system whose author asserts
"Copyright © 1986–2005 by Thomas Gangale" — and lists the **Discordian**
calendar as **Planned**, a system from *Principia Discordia*, which carries
"Ⓚ ALL RIGHTS REVERSED — Reprint what you like" and shipped inside `util-linux`
as `ddate` for roughly eighteen years without challenge. Both are constructed
calendars from identifiable authors. Under §102(b) both are implementable, and
so is Shire Reckoning; under the exclusion row's stated rationale, Darian
should have been excluded too. One of those two rows is wrong, and it is the
exclusion row.

**The three named examples fail for three different reasons**, which one row
cannot express:

- **Shire Reckoning** — a clean, fully specified algorithm. The exposure is the
  name list and the trademarks, not the arithmetic. (In passing: the commonly
  cited 2044 date is the UK/EU life+70 expiry. The binding date for a crates.io
  publisher is the US term — 95 years from the 1955 publication of Appendix D
  in *The Return of the King*, so **circa 2051**.)
- **Stardates** — should be excluded for **"no unambiguous published rule"**,
  the same criterion that correctly sinks Raventós in §3h. There is no official
  conversion formula; the TNG writers' guide gives a convention that the
  programmes themselves violate. Copyright is the wrong reason.
- **Imperial Dating** — fully specified and trivially implementable
  (`0.123.456.M41`, the year split into 1000 parts). The real obstacle is
  **Games Workshop's fan policy, which permits non-commercial use only** and is
  therefore incompatible with a permissively licensed crate. That is a
  licensing-posture question, not copyright subject matter.

**The exclusion also sweeps in things that are unambiguously free**, at a cost
in coverage and no gain in safety: the **Discordian** calendar (above) and the
**Terran Computational Calendar**, whose site carries an explicit Creative
Commons Public Domain Mark. Both are fully specified.

Recommendation: replace the single row with the four criteria the project can
actually apply.

| Criterion | Effect | Examples |
| --- | --- | --- |
| No unambiguous published rule | Exclude | Stardates, the Dune Universal Standard Calendar, Raventós, the Klingon calendar |
| Rule is free; names and marks are not | **Implement the arithmetic** with generic month indices; put names behind an optional feature or a downstream crate; do not reproduce prose; do not use the mark in the crate name | Shire Reckoning, the Calendar of Harptos (**not** covered by WotC's CC-BY SRD 5.1 — Forgotten Realms names were Product Identity), the Tamrielic calendar (community documentation is CC-BY-SA, which is copyleft and would infect reused prose) |
| Publisher policy forbids commercial redistribution | Exclude **on policy**, stating the policy | Warhammer 40,000 |
| Explicitly free | Implement | Discordian, Terran Computational, Darian (already Done) |

### 3j. Computing and standards epochs

`calendars.md` stage 1 covers Julian Day, MJD, ISO week and ISO ordinal;
[`time-scales.md`](time-scales.md) covers TAI, GPS and the uniform scales; and
`hc-uncertainty::edtf` already implements ISO 8601-2 / EDTF, including the
part that matters structurally — an EDTF value such as `1984?` is **not a
point in time**, has no instant, no duration and no total order, so it needs
its own type rather than a `Date`. Most of this axis is therefore outside
`calendars.md` by design and is not counted as a gap. Four remarks:

- **4-4-5 / NRF 4-5-4 retail calendars** are an Out-of-scope row, reasoned as
  "organisation-specific rather than cultural". That reasoning does not hold
  as stated. The **NRF 4-5-4 calendar is a published, normative, citable
  standard** with a deterministic rule — a 53rd week is added when four or
  more days remain in January after the 52nd
  ([NRF](https://nrf.com/resources/4-5-4-calendar)) — and
  **[IRS Publication 538](https://www.irs.gov/publications/p538)** gives the
  exact parameterisation of the general 52/53-week family: the year "must
  always end on the same day of the week", either the last occurrence of that
  weekday in a month *or* the occurrence "nearest to the last day of the
  calendar month", which are two different calendars from one weekday. The
  accurate framing is "**a parameterised rule set, not a calendar**" — which
  is exactly `hc-holiday`'s own design, and arguably puts it *in* scope there
  rather than out of scope here. What a library must not do is hardcode one
  variant: the parameters are the period-end weekday, the anchor month,
  last-versus-nearest, and the 4-4-5 / 4-5-4 / 5-4-4 grouping.
- **Lilian date, Dublin JD, Truncated JD and CNES JD** are one-line offsets
  from the two Julian Day calendars already implemented, and `julian_day.rs`
  is already structured to build a calendar from an offset (`day_count_meta`).
  Near-free. Note that **Dublin JD is noon-based** like JD itself, while the
  rest are midnight-based — the `.5` in `MJD = JD − 2400000.5` is the whole
  point of MJD.
- **"Julian date" in lot codes is a naming trap, and it is the most common one
  in this whole survey.** In manufacturing, aviation and food packaging a
  "Julian date" is a **day-of-year** code — `DDD`, `YDDD` or `YYDDD` — with no
  relation to the astronomical Julian Day, no time component, and a genuinely
  ambiguous year (`0032` could be 1990, 2000, 2010 or 2020). No single
  normative standard defines it; the closest is
  [USDA FSIS](https://www.fsis.usda.gov/food-safety/safe-food-handling-and-preparation/food-safety-basics/food-product-dating)
  on egg pack dates. If such an API is ever added it should be named
  `ordinal_lot_code`, never `julian_date`.
- **Leap smearing cannot be represented as UTC, and should not be.** Smeared
  time is neither UTC nor TAI and is undetectable to the client;
  [RFC 8633](https://www.rfc-editor.org/rfc/rfc8633.html) (the NTP BCP) says
  clients **must not** mix smeared and non-smeared servers and that public
  NTP servers **must not** smear. Since this library's whole UTC story is an
  explicit leap-second table and a type that can say `23:59:60`, a smeared
  input silently mislabelled as UTC would defeat it. That is worth one line in
  [`time-scales.md`](time-scales.md) — as a thing the library refuses to
  represent, in the same spirit as [ADR 0006](adr/0006-refuse-to-extrapolate.md).

---

## 4. Structural findings

Where a gap is not a missing row but a missing capacity. Ordered by how
expensive each becomes if left until after the calendars that need it are
written.

### 4.1 `DateFields` can describe a leap **month** but not a leap **day**

`Month` carries `{ ordinal, leap }` and the doc comment explains exactly why:
the Chinese leap fourth month is 閏四月, not month 13. `day` is a bare
`Option<u8>` with no such flag.

A large family of calendars **omits and repeats day numbers**:

- **Tibetan and its variants.** The date is whichever of the 30 tithis is
  current at daybreak, so a tithi that begins and ends between two daybreaks
  yields **no calendar day** (*chad*, omitted) and a tithi spanning two
  daybreaks yields **two days with the same number** (*lhag*, duplicated).
  Weekdays never skip or repeat — only dates do
  ([Janson, *Tibetan Calendar Mathematics*, arXiv:1401.6285](https://arxiv.org/abs/1401.6285)).
- **Mongolian, Geden and Bhutanese**, which inherit the mechanism with
  different parameters, so the *set* of affected dates differs per variant.
- **Hindu lunisolar.** The same rule, at sunrise: a short tithi gives a
  *kshaya* day, a long one an *adhika/vriddhi* day
  ([Astronomical basis of the Hindu calendar](https://en.wikipedia.org/wiki/Astronomical_basis_of_the_Hindu_calendar);
  [Dershowitz & Reingold, *Indian Calendrical Calculations*](https://www.cs.tau.ac.il/~nachum/calendar-book/papers/hindu-paper.pdf)).
- **Balinese Śaka.** *Ngunaratri* assigns two lunar days to one solar day
  about every 63 days, skipping a date number.

For a repeated day, two consecutive `Rd` values produce identical
`(era, year, month, day)`. The trait's stated contract —
`from_fixed(to_fixed(date)) == date` for every valid date, and
`to_fixed(from_fixed(rd)) == rd` for every `rd` in range — **cannot hold**,
and `hc-calendars-solar`'s `round_trip_every_calendar!` macro would reject
such a calendar on its first duplicated day.

`tibetan`, `hindu-lunar`, `hindu-solar` and `hindu-old` are all already
**Planned** rows. The fix — a `Day { ordinal, leap }` mirroring `Month`, or a
reserved extra field — is cheap now and is a breaking change to `DateFields`,
the FFI struct and the wasm surface later. **This is the single most expensive
finding in this document to defer.**

### 4.2 `CalendarMeta` has no day-boundary field, though ADR 0001 says it should

[ADR 0001](adr/0001-rata-die-as-the-calendar-pivot.md) names this cost
honestly and says the library "handles it by pairing `Rd` with a `CivilTime`
and documenting the day-start convention per calendar". The documenting has
happened; the *modelling* has not. `CalendarMeta` carries `year_kind`,
`has_leap_months`, `is_astronomical`, `earliest` and `latest`, and nothing
about when the day begins. `Moment` is defined as an offset "from local
midnight of `rd`" (`fixed.rs:134`).

The research turned up more distinct day boundaries than the ADR anticipates:

| Boundary | Calendars |
| --- | --- |
| Sunset | Hebrew, Islamic, Bahá'í, **Javanese**, **Hawaiian**, Babylonian, Attic, Coligny |
| Sunrise | Hindu tithi assignment, **Bengali**, **Assamese** (explicitly), Balinese |
| Dawn, ~06:00 | Ethiopic |
| **Daybreak = 05:00 local mean solar time, and therefore longitude-dependent** | Tibetan family (Janson's constant) |
| **Three-fifths of the sunrise-to-sunset interval (*aparāhṇa*)** | Malayalam sankranti rule |
| **Sunrise *and* midnight in one test** | Bengali sankranti rule |
| 23:00 | Chinese traditional, four pillars |
| Sunrise *and* sunset (two halves), with a separate midnight astronomical day | **Burmese** |
| Ujjain-meridian midnight | Kali Yuga *ahargana*, the substrate of every Surya Siddhanta computation |

A generic consumer — the registry, `hyper-calendar-ffi`,
`hyper-calendar-wasm`, the holiday engine — can learn none of this. The
library has solved the problem exactly once, locally: `ZiHourConvention`
(`cycle.rs:628`) lets the four-pillars code choose between 23:00 and midnight.
That is the right idea in the wrong place — an enum in `hc-calendar::cycle`
serving one calendar, rather than a property of `CalendarMeta` that every
calendar can state and every consumer can read.

It matters most at stage 3, where the astronomical variants land, and for
`observances.md`, where "is today a holiday" for a sunset-reckoned holiday has
a different answer at 18:00 than at 10:00.

### 4.3 The holiday engine's calendar set is a closed enum

`hc_holiday::rule::CalendarSystem` is a closed enum of eight calendars —
Gregorian, Julian, IslamicCivil, IslamicUmmAlQura, Hebrew, Chinese, Dangi,
Vietnamese — with a sound documented justification: a holiday table is a
`static` value and a trait object cannot be one without an allocator.

The justification is right and the consequence is still a wall. Every Planned
tradition row in `observances.md` needs a new variant **and** a new match arm:
Hindu, Sikh, Jain, Bahá'í and Zoroastrian all date their festivals in
calendars this enum cannot name. So do Coptic and Ethiopic feasts, which is
why there is no Ethiopia table. The crate says so about itself in the
`BUDDHIST` rule set's own `sources` string, and `observances.md` now repeats
it: India's, Singapore's, Malaysia's and Indonesia's missing holidays are
"absent because the calendars they are dated in do not exist in
`hc-calendars-lunar` yet".

[policy.md](policy.md) §2 states the test precisely — "adding Bolivia's
holidays, or the Tibetan calendar, or Welsh plural rules, should mean adding a
data entry, not editing control flow" — and this is the one place in the
workspace where adding a *calendar* requires editing control flow in a
*different crate*. The `no_std`-without-alloc constraint is real; a registry of
`&'static dyn DynCalendar`, or a `CalendarSystem(&'static str)` resolved
against a caller-supplied slice, would keep it while opening the set.

### 4.4 `Adoption` is a single cut-over pair, and Sweden needed a list

`julian_gregorian::Adoption` is `{ id, region, last_julian, first_gregorian }`.
The module's own comment already notes that "several did so twice". Sweden
breaks the shape outright: between 1 March 1700 and 30 February 1712 it kept a
third calendar that was neither Julian nor Gregorian, and `julian-gregorian-se`
models only the 1753 cut-over. The entry is not wrong for 1753 and is wrong
for every date in those twelve years. A `&'static [Cutover]` instead of a pair
fixes Sweden and whatever comes after it.

### 4.5 `DynCalendar::is_leap_year`'s default is a heuristic

```rust
let this = self.days_in_year(year)?;
let previous = self.days_in_year(year - 1)?;
Ok(this > previous)
```

"Longer than last year" is not "leap". Hebrew common years are 353, 354 or 355
days and leap years 383, 384 or 385; a 355-day common year following a 353-day
common year — and the 19-year cycle does contain consecutive common years — is
reported as leap. No registered calendar overrides the `DynCalendar` method
(the per-calendar `is_leap_year` functions are free functions on the concrete
modules, which `DynAdapter` never calls). The same default will misreport the
**Burmese** year, which has three lengths for the same reason, and the
**Thai lunar** year, which has three mutually exclusive types. The **Odia Anka
year**, whose numbering skips whole years, makes `year - 1` itself meaningless.

### 4.6 `DynCalendar::days_in_year` assumes month 1, day 1 exists and starts the year

It computes `ymd(year, 1, 1)` to `ymd(year + 1, 1, 1)`. For `maya-longcount`,
`from_fields` requires the five long-count extras and rejects a bare y-m-d, so
the method can only fail. For `balinese-pawukon` and `sexagenary`, "year" is a
cycle index and the answer is a category error rather than a number. Cyclic
calendars sit in the same registry as calendars where this means something,
and `CalendarMeta` offers no "has months" or "has years" predicate to tell
them apart.

### 4.7 `ExtraFields` has eight slots and the Pawukon uses all eight

`MAX_EXTRA_FIELDS = 8`, and the constant's own comment cites the Pawukon as
the reason. The Pawukon then fills all eight (`dwiwara` … `dasawara`), putting
the seven-day week in `day` and the *wuku* in `month`, and dropping the
one-day week as derivable. The module documents this and it is defensible.

The point is that there is **zero headroom**. A combined Maya view — long
count (5) plus Tzolkʼin (2) plus Haabʼ (2) — needs nine, and the 819-day count
of §3b would make ten. Borana needs a star-month, a 27-day name and a lunar
phase alongside a month and day. `set` returns `CalendarError::Overflow`,
which is a runtime failure for a shape known at compile time.

### 4.8 `DateFields::era` is `Option<&'static str>`

Era codes must be compile-time constants. The 248-entry nengō table gets away
with this because it is compiled in. `chinese-regnal` — a **Planned** row —
would not: there are over a thousand 年号, and, more to the point, a caller who
wants to supply an era table across the FFI or wasm boundary cannot produce a
`&'static str`. Any runtime-supplied era vocabulary is out of reach by
construction. The same applies to the Lakota winter counts of §3b, whose
"year" simply *is* a name.

### 4.9 There is no representation for a table-driven calendar's authority

`islamic-umalqura` is modelled correctly: table-driven, bounded to 1300–1600
AH, refusing to extrapolate. That pattern needs to generalise, because at
least three Planned or near-Planned calendars are **not algorithms at all**:

- **Nepali Bikram Sambat**, whose month lengths are fixed annually by decree
  of the Panchanga Nirnayak Samiti and **can differ by a day from independent
  computation**.
- **Sri Lankan Avurudu nekath**, issued annually by a ministry panel.
- Any Gulf holiday table announced by decree — which `observances.md` already
  handles well, via `Confidence::Approximate` and an invitation to supply your
  own `RuleSet`.

The gap is that `CalendarMeta` can say *bounded* (`earliest`/`latest`) but not
*table-driven, and the table is someone else's ruling*. `is_astronomical`
distinguishes arithmetic from computed; nothing distinguishes computed from
**decreed**. That is a third category, and it is the one where a wrong answer
is most confidently wrong.

### 4.10 Two boundaries that `Rd` genuinely cannot hold

ADR 0001 is right that a day is the largest unit every calendar agrees on.
Two cases in the research are not day-aligned at all:

- **Sinhalese Aluth Avurudu.** The year turns at the *midpoint of the
  nonagathe*, a published clock instant about 12 h 48 m wide. "The year begins
  on date D" is not expressible; the year begins at a time.
- **Tibetan daybreak at 05:00 local mean solar time**, which makes the
  date-to-`Rd` mapping depend on longitude, not only on the calendar.

Neither breaks the pivot — both are handled by pairing `Rd` with a
`CivilTime`, exactly as the ADR says — but neither can be stated in
`CalendarMeta` today, so a caller has no way to learn that it must.

### 4.11 The provenance of the formulae is undocumented, and this audit would deepen the dependency

Not a defect — a gap in the record, raised because §5 recommends leaning
further on the same source. *Calendrical Calculations* is cited by chapter and
section throughout the workspace: `gregorian.rs`, `common.rs`, `persian.rs`,
`french_republican.rs`, `julian_gregorian.rs`, `maya.rs` (ch. 11), `aztec.rs`
(ch. 9), `balinese_pawukon.rs` (§10.6), `hc-astro`'s `time.rs`, `solar.rs` and
`search.rs` (§14.4), `hc-tz`'s `gregorian.rs` (§2.2), `hc-holiday`'s Hebrew
rule set (ch. 8), and four crate READMEs. [ADR 0001](adr/0001-rata-die-as-the-calendar-pivot.md)
names it as the source of the design.

The **formulae and the design are not copyrightable subject matter** — they are
procedures and systems under 17 USC 102(b), per §3i above — and citing them is
exactly right. The **accompanying Lisp code is separately licensed and is not
free**: the authors retain all rights except as granted, and downstream ports
state plainly that commercial use is not permitted (for example
[ferd/calcalc](https://github.com/ferd/calcalc)). This workspace is
BSD-3-Clause and [ADR 0005](adr/0005-no-external-dependencies.md) explicitly
values that "no dependency can break a build, change a licence or introduce a
breaking change underneath us".

Everything visible in the code is consistent with the safe practice — Rust
written against the published formulae, with its own tests and its own cited
reference dates, which is the correct way to use the book. The finding is that
**the practice is nowhere stated**. A sentence in [policy.md](policy.md) or a
short ADR — "we implement from the published formulae and the cited reference
dates, never from the book's accompanying source" — costs nothing now and is
worth having before the next round of work adds Icelandic (ch. 6), Generic
Cyclical (ch. 13), Hindu, Tibetan and Samaritan calendars, all of which the
book also covers.

### 4.12 Smaller notes

- **`Month.leap` means "the intercalary repetition of `ordinal`."** Shang
  oracle-bone intercalation appended a 13th — and, in attested cases, a 14th
  and 15th — month with its own number rather than a repeat; the Hindu
  *kshaya* month leaves ordinals non-contiguous. Both are representable
  numerically; both make `has_leap_months` and any generic month iteration
  mislead. So do the **four different meanings of "leap"** in Southeast Asia
  alone: add a month (most), add a day (Thai and Khmer), add both in one year
  (Burmese only), and *skip* a day (Balinese *ngunaratri*).
- **Day-of-month is not always an integer.** Thai, Khmer, Lao and Balinese
  dates are a *(phase, 1–15)* pair — waxing ขึ้น always to 15, waning แรม to 14
  or 15. Flattening that into 1..30 loses the hollow/full-month distinction on
  the way back.
- **Weekday is derivable from `Rd` alone**, per ADR 0001, "because the
  seven-day cycle survived every calendar reform". It very nearly did. The
  Soviet Union ran a five- and then a six-day week from 1929 to 1940, and the
  French Republican *décade* ran a ten-day one — the latter is implemented, so
  the ADR's claim is already narrower than it reads.

---

## 5. Recommended ordering

The reasoning throughout is the same: **change the shape before adding rows
that depend on the shape**, then take the cheapest well-sourced wins, then the
ones that teach the abstraction something.

**First — the free corrections.** Fix the wrong identifiers in `calendars.md`
stage 1, or register the identifiers the table promises (for `ethioaa` the
second is probably right, since CLDR has it and the header promises CLDR
interop). Promote the Aztec row. Demote the Thai solar row to Partial, since
the code already documents the restriction. Add rows for `japanese-imperial`,
`modified-julian-day`, the Byzantine indiction and `hc-almanac`. This costs
nothing and removes every case where a reader who trusts the table writes code
that returns `None`.

**Second — `Day { ordinal, leap }`, before `tibetan` or `hindu-lunar`.**
§4.1. Four Planned rows need it, the Balinese Śaka calendar and the whole
Mongolian and Bhutanese family need it, it is a breaking change to three
public surfaces later, and the round-trip macro will reject those calendars
without it.

**Third — a day-start field on `CalendarMeta`, and fold `ZiHourConvention`
into it.** §4.2. Stage 3 is the astronomical variants, and the day boundary is
what makes them disagree with the arithmetic forms. Doing this after stage 3
means revisiting stage 3. The field needs more than an enum of three values:
the table in §4.2 has nine distinct boundaries, two of them functions of
sunrise and sunset rather than constants.

**Fourth — open `CalendarSystem`.** §4.3. Five Planned tradition rows, the
Ethiopia table, and an already-shipped Partial that the crate itself flags as
approximated through the wrong calendar all wait behind one enum.

**Fifth — the cheap, well-sourced calendars**, in roughly this order because
each is nearly free given what exists: **Symmetry010** (the leap rule is
already in the file), **Revised Julian** (in current official use; a mod-900
variant of `gregorian.rs`), the **Julian Day offsets** (Lilian, Dublin,
Truncated, CNES), the **Mesoamerican name tables** (Zapotec, Mixtec with its
40-day year-bearer offset, Purépecha — all riding the implemented Maya
engine), the **Maya 819-day count**, **Pranata Mangsa**, the **Bangladeshi
national calendar**, **Mandaean**, **modern Assyrian**, **Yazidi**, and the
**reform proposals whose rules are unambiguous** — International Fixed,
Hanke–Henry, Positivist and Meyer–Palmen Solilunar, which between them need
one new mechanism (the leap week) that the Old Icelandic calendar wants
anyway. Leave Pax, Mastrofini and Raventós until someone produces a primary
source; by the project's own step 4 they are not yet implementable.

**Sixth — the Theravada block.** Burmese, Thai lunar, Khmer, Lao and
Sinhalese, plus the pre-1941 Thai solar year. This is the highest-value
regional cluster because it un-blocks a Partial that has already shipped, and
because Gislén & Eade's *JAHH* series treats the four as one system, so they
are cheaper together than separately.

**Seventh — the calendars that stress the abstraction**, which is where the
value is now that the base layer is dense: the **Qumran 364-day calendar with
the *mishmarot*** (a fixed-weekday year and a named-course cycle), the **Old
Icelandic misseri calendar** (week-primary, leap week), the **Javanese** and
**Balinese Śaka** calendars (the two calendars whose subsidiary cycles are
already Done without them), and **Borana** (a day-name cycle shorter than the
month).

**Eighth — Sweden 1700–1712**, by turning `Adoption` into a cut-over list.
§4.4. Small, finite, fully documented, and the cheapest correctness test in
this document: any library claiming historical Julian/Gregorian conversion for
Sweden is probably wrong here.

**Ninth — the scoping corrections that are not new rows.** Split `tibetan`
into Phugpa, Tsurphu, Kālacakra, Geden and Bhutanese parameter sets, and build
it as a parameterised engine so Mongolian is nearly free. Re-scope
`bikram-sambat` from *Planned algorithm* to *table-driven, bounded*, following
`islamic-umalqura`. Split `hindu-solar` by sankranti rule, because Orissa,
Tamil, Kerala and Bengal are four different calendars sharing one astronomy.
Name a Nanakshahi variant (2003 Mool, 2010 or 2014) rather than a calendar.

**Tenth — the honest negatives.** Add Out-of-scope or Researching rows, with
reasons, for: **Dreamspell** (because `maya-tzolkin` is Done and users will
ask), **Muisca**, **Pentecontad**, **Bulgar**, **Rapa Nui** and **Cham**
("reconstruction, sources conflict"), **Taiping tianli** ("sources too thin"),
**Tongan** ("the intercalary trigger is the state of yams and fish"),
**Sakha** ("the year begins at the first cuckoo call"), **Hopi/Zuni** ("the
calendar is a function of the observer's standing position"), **Lakota winter
counts** ("the year is a name chosen afterwards"), **Australian Aboriginal
seasonal calendars** ("well sourced, and not date arithmetic"), and the
**Slavic, Baltic, Basque and Druze** naming layers.

A table that says *why not* is worth more than a table that is silent, and
this project already has the vocabulary for it.

**Eleventh — three wording fixes that are not about coverage at all.** Replace
the single fictional-calendar exclusion with the four criteria in §3i, since
the current row contradicts the project's own Darian and Discordian rows.
Re-reason the 4-4-5 row per §3j — "a parameterised rule set, not a calendar"
rather than "organisation-specific" — and decide whether that puts it in scope
for `hc-holiday`. And write down the provenance practice of §4.11 before the
next round of work leans further on the same book.

---

## Sources

Claims are cited inline. Four works carry most of the weight and are worth
acquiring if they are not already to hand:

- **Dershowitz & Reingold, *Calendrical Calculations: The Ultimate Edition***
  (Cambridge University Press, 2018) —
  [reingold.co/calendars.shtml](https://reingold.co/calendars.shtml). The
  architecture already follows it. It carries tested algorithms for the
  Icelandic (ch. 6), Balinese Pawukon (ch. 12), **Generic Cyclical Calendars**
  (ch. 13 — the right abstraction for §4.7), Babylonian, Samaritan,
  Coptic/Ethiopic, Persian in both forms, French Revolutionary in both forms,
  Tibetan, and Old and Modern Hindu calendars.
- **Gislén & Eade, "The Calendars of Southeast Asia", *JAHH* 22(3), 2019** — a
  six-part series; the best modern technical treatment of Burma, Thailand,
  Laos and Cambodia (part 2), Vietnam (part 3) and Malaysia and Indonesia
  (part 4). Also J. C. Eade, *The Calendrical Systems of Mainland South-East
  Asia*, Brill, 1995.
- **Svante Janson**, [*Tibetan Calendar Mathematics*](https://arxiv.org/abs/1401.6285)
  and [*The Icelandic Calendar*](https://www2.math.uu.se/~svantejs/papers/calendars/iceland.pdf).
- **Saha & Lahiri (eds.), *Report of the Calendar Reform Committee*, CSIR,
  1955** — surveyed thirty Indian calendars and is the source of the Lahiri
  ayanāṁśa that `hc-seasons::zodiac::sidereal` already offers.

Two research caveats, recorded rather than hidden. The session exhausted its
web-search budget, so a handful of entries — Tulu, Sundanese, Lao, Shan and
Dai — rest on thinner sourcing than the rest and are marked as such in place.
And three cited hosts were unreachable at the time of writing and should be
re-checked before being relied on: `npns.gov.np` (Nepal Panchanga Nirnayak
Samiti), `en.banglapedia.org`, and `kalacakra.org` (mismatched TLS
certificate).
