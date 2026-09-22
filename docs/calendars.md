> **This file is the roadmap, not the inventory.** What exists is listed in
> [`supported.md`](supported.md), which is generated from the code and cannot
> drift from it. What is here is the part a generator cannot produce: what is
> planned, what is being researched, what is out of scope and why, and how to
> add a calendar. Where a row below is marked Done, `supported.md` is the
> authority on its identifier, range and day boundary.

# Calendar coverage

Requirement 3 of the project brief asks for every calendar we can know about,
covered in stages and without quietly dropping any. This file is that list. It
is the roadmap *and* the honest status report: a calendar is only marked
**Done** when it is implemented, round-trip tested and anchored to a published
reference date.

Status values:

| Value | Meaning |
| --- | --- |
| **Done** | Implemented, tested, anchored to a citable reference |
| **Partial** | Implemented with a documented restriction (arithmetic variant only, bounded range) |
| **Planned** | Accepted into scope with a known algorithm; not yet written |
| **Researching** | In scope, but the rules are contested or the sources disagree |
| **Out of scope** | Deliberately excluded, with a reason |

Identifiers follow Unicode CLDR where CLDR has one, so that values interoperate
with `Intl.DateTimeFormat` and ICU without a translation table.

---

## Stage 1 — Solar and arithmetic calendars

The base layer. These need no astronomy, so they carry no ephemeris cost.

| Calendar | Id | Crate | Status |
| --- | --- | --- | --- |
| Proleptic Gregorian | `gregory` | `hc-calendars-solar` | Done |
| Proleptic Julian | `julian` | `hc-calendars-solar` | Done |
| Julian→Gregorian reform (per country) | `julian-gregorian-<polity>`, 12 of them | `hc-calendars-solar` | Done |
| ISO 8601 week date | `iso8601-week` | `hc-calendars-solar` | Done |
| ISO 8601 ordinal date | `iso8601-ordinal` | `hc-calendars-solar` | Done |
| Julian Day Number | `julian-day` | `hc-calendars-solar` | Done |
| Modified Julian Day | `modified-julian-day` | `hc-calendars-solar` | Done |
| Lilian, ANSI, Dublin, Reduced, Truncated, CNES and CCSDS day counts | `lilian`, `ansi-date`, `dublin-julian-day`, `reduced-julian-day`, `truncated-julian-day`, `cnes-julian-day`, `ccsds-day` | `hc-calendars-solar` | Done |
| Coptic | `coptic` | `hc-calendars-solar` | Done |
| Ethiopic (Amete Mihret) | `ethiopic` | `hc-calendars-solar` | Done |
| Ethiopic (Amete Alem) | — | `hc-calendars-solar` | Partial — an `amete-alem-year` field on `ethiopic`, not a calendar of its own |
| Ancient Egyptian wandering year | `egyptian` | `hc-calendars-solar` | Done |
| Armenian (wandering) | `armenian` | `hc-calendars-solar` | Done |
| Armenian (fixed, Sarkawag 1084) | `armenian-fixed` | `hc-calendars-solar` | Done |
| Zoroastrian (Qadimi / Shahanshahi / Fasli) | `zoroastrian-qadimi`, `zoroastrian-shahanshahi`, `zoroastrian-fasli` | `hc-calendars-solar` | Done — the two wandering years from the Yazdegerdi epoch, thirty days apart since the Parsi intercalation of the 1120s, and the Fasli on 21 March with the Gregorian leap day; the Iranian *Bastani* observance is `persian` |
| Solar Hijri (Persian), arithmetic | `persian-arithmetic` | `hc-calendars-solar` | Done — Birashk's 2 820-year cycle; the astronomical calendar is `persian`, stage 3 |
| Indian national civil (Śaka) | `indian` | `hc-calendars-solar` | Done |
| Thai solar (Buddhist Era) | `buddhist` | `hc-calendars-solar` | Partial — the 1889–1940 April year start is not modelled |
| Minguo (Republic of China) | `roc` | `hc-calendars-solar` | Done |
| Juche (DPRK) | `juche` | `hc-calendars-solar` | Done |
| Holocene / Human Era (人類紀元) | `holocene` | `hc-calendars-solar` | Done |
| Japanese imperial year (皇紀) | `japanese-imperial` | `hc-calendars-solar` | Done — proleptic before the 1873 adoption |
| Byzantine / Anno Mundi world era | `byzantine` | `hc-calendars-solar` | Done |
| Roman *ab urbe condita* | `roman-auc` | `hc-calendars-solar` | Done |
| French Republican, arithmetic (Romme) | `french-republican-arithmetic` | `hc-calendars-solar` | Done — Romme's proposal; the decree's rule is `french-republican-equinox`, stage 3 |
| Bahá'í (Badíʿ), arithmetic Western rule | `bahai-arithmetic` | `hc-calendars-solar` | Done — the rule kept until 171 BE, continued proleptically |
| Bahá'í (Badíʿ), as kept | `bahai` | `hc-calendars-solar` | Done through 221 BE (19 March 2065) — the arithmetic rule to 171 BE, the Bahá'í World Centre's published table for 172–221 BE; refuses after, until `hc-astro` extends it |
| Symmetry454 | `symmetry454` | `hc-calendars-solar` | Done |
| Symmetry010 | `symmetry010` | `hc-calendars-solar` | Done |
| Revised Julian (Milanković, 1923) | `revised-julian` | `hc-calendars-solar` | Done |
| World Calendar (1930 proposal) | `world-calendar` | `hc-calendars-solar` | Done |

## Stage 2 — Lunar and lunisolar calendars

These need the astronomical engine, so they live behind the `lunar` feature.

| Calendar | Id | Crate | Status |
| --- | --- | --- | --- |
| Tabular Islamic, civil epoch | `islamic-civil` | `hc-calendars-lunar` | Done |
| Tabular Islamic, astronomical epoch | `islamic-tbla` | `hc-calendars-lunar` | Done |
| Fatimid / Ṭayyibī Bohra *Misri* | `islamic-fatimid` | `hc-calendars-lunar` | Done |
| Umm al-Qura (Saudi official) | `islamic-umalqura` | `hc-calendars-lunar` | Done — table-driven, 1300–1600 AH |
| Observational Hijri | `islamic-rgsa` | `hc-calendars-lunar` | Partial — visibility model is a simplification |
| Hebrew | `hebrew` | `hc-calendars-lunar` | Done |
| Chinese lunisolar | `chinese` | `hc-calendars-lunar` | Done |
| Korean (Dangi) | `dangi` | `hc-calendars-lunar` | Done |
| Vietnamese | `vietnamese` | `hc-calendars-lunar` | Done |
| Japanese lunisolar (Tenpō, 1844–1872) | `japanese-tenpo` | `hc-calendars-lunar` | Done |
| Japanese lunisolar (Kansei, 1798–1844) | `japanese-kansei` | `hc-calendars-lunar` | Done |
| Japanese lunisolar (Hōryaku, 1755–1798) | `japanese-horyaku` | `hc-calendars-lunar` | Done |
| Japanese lunisolar (Jōkyō, 1685–1755) | `japanese-jokyo` | `hc-calendars-lunar` | Done |
| Japanese lunisolar (Senmyō, 862–1685) | `japanese-senmyo` | `hc-calendars-lunar` | Done |
| Tibetan (Phugpa) | `tibetan` | `hc-calendars-lunar` | Planned |
| Hindu lunisolar, amānta | `hindu-lunar` | `hc-calendars-indic` | Done — true Sun and Moon, Lahiri ayanamsa, the day read at the Central Station's sunrise as the *Rashtriya Panchang* does; tested against two years of its month tables |
| Hindu lunisolar, pūrṇimānta | `hindu-lunar-purnimanta` | `hc-calendars-indic` | Done — the amānta tithis under the north's month names, the intercalary month inserted whole, as the *Rashtriya Panchang*'s vadi column labels them |
| Hindu solar, Tamil | `hindu-solar-tamil` | `hc-calendars-indic` | Done — the month begins on the saṅkrānti's day unless it fell after sunset; the *Rashtriya Panchang*'s regional tables for 2023–2025 |
| Hindu solar, Malayalam (Kollam era) | `hindu-solar-malayalam` | `hc-calendars-indic` | Done — unless it fell after three fifths of the daylight |
| Hindu solar, Bengali (Bangabda) | `hindu-solar-bengali` | `hc-calendars-indic` | Done — the day after the saṅkrānti's |
| Hindu solar, Vikrami (Punjab, Odisha; the Nepali reckoning) | `hindu-solar-vikrami` | `hc-calendars-indic` | Done — the sunrise-to-sunrise day of the saṅkrānti |
| Old Hindu (mean) lunisolar and solar | `hindu-old` | `hc-calendars-lunar` | Planned |

## Stage 3 — Astronomical variants of stage 1 calendars

Same calendars, computed from observation rather than from a cycle. They can
disagree with the arithmetic form by a day, which is exactly why both exist.

| Calendar | Id | Status |
| --- | --- | --- |
| Solar Hijri, astronomical (noon, Iran Standard Time) | `persian` | Done, in `hc-calendars-equinox` — Nowruz 1404 on 21 March 2025, where Birashk's cycle says the 20th |
| French Republican, autumn equinox at Paris | `french-republican-equinox` | Done, in `hc-calendars-equinox` — the fourteen new years France kept |
| Bahá'í, Naw-Rúz from the Tehran equinox for any year | `bahai-astronomical` | Done, in `hc-calendars-equinox` — reproduces every row of the 172–221 BE table `bahai` carries, Twin Holy Birthdays included |
| Ethiopian Easter-linked movable cycle (Bahire Hasab) | — | Done, in `hc-holiday`; see [observances.md](observances.md) |
| Coptic Easter-linked movable cycle | — | Done, in `hc-holiday`; see [observances.md](observances.md) |

## Stage 4 — Regional, cyclic and era calendars

| Calendar | Id | Crate | Status |
| --- | --- | --- | --- |
| Japanese imperial eras (和暦, 大化 → 令和) | `japanese` | `hc-calendars-regional` | Done — 248 nengō |
| Japanese eras, Northern Court (北朝) | `japanese-northern` | `hc-calendars-regional` | Done |
| Japanese eras, Southern Court (南朝) | `japanese-southern` | `hc-calendars-regional` | Done |
| Japanese eras, as proclaimed (改元当時) | `japanese-proclaimed` | `hc-calendars-regional` | Done |
| Chinese sexagenary cycle (干支), incl. the four pillars (四柱/八字) | `sexagenary` | `hc-calendars-regional` (arithmetic in `hc-calendar::cycle`) | Done |
| Chinese regnal eras (年号) | `chinese-regnal` | `hc-calendars-regional` | Planned |
| Korean regnal eras | `korean-regnal` | `hc-calendars-regional` | Planned |
| Maya long count (GMT 584283) | `maya-longcount` | `hc-calendars-regional` | Done |
| Maya long count (GMT+2, 584285) | `maya-longcount-gmt2` | `hc-calendars-regional` | Done |
| Maya Tzolkʼin (260 days) | `maya-tzolkin` | `hc-calendars-regional` | Done |
| Maya Haabʼ (365 days) | `maya-haab` | `hc-calendars-regional` | Done |
| Maya calendar round | `maya-round` | `hc-calendars-regional` | Done |
| Aztec Tonalpohualli | `aztec-tonalpohualli` | `hc-calendars-regional` | Done |
| Aztec Xiuhpohualli | `aztec-xiuhpohualli` | `hc-calendars-regional` | Done |
| Balinese Pawukon (thirty *wuku*, ten concurrent weeks) | `balinese-pawukon` | `hc-calendars-regional` | Done |
| Javanese Pasaran (five-day market week) | `javanese-pasaran` | `hc-calendars-regional` | Done |
| Igbo four-day week (Izu) | `igbo` | `hc-calendars-regional` | Planned |
| Yoruba four-day week | `yoruba` | `hc-calendars-regional` | Researching — regional variants differ |
| Akan Adaduanan (42-day cycle) | `akan` | `hc-calendars-regional` | Planned |
| Nepali Bikram Sambat | `bikram-sambat` | `hc-calendars-regional` | Planned — the Vikrami solar rule and era are `hindu-solar-vikrami`; what remains is comparing the committee's published calendar against it |
| Nepal Sambat | `nepal-sambat` | `hc-calendars-regional` | Planned |
| Burmese | `burmese` | `hc-calendars-regional` | Planned |
| Thai lunar (Chulasakarat) | `thai-lunar` | `hc-calendars-regional` | Planned |
| Rumi (late Ottoman fiscal) | `rumi` | `hc-calendars-regional` | Planned |
| Attic (Athenian) | `attic` | `hc-calendars-regional` | Researching — reconstruction, sources conflict |
| Babylonian | `babylonian` | `hc-calendars-regional` | Researching — regnal anchoring is uncertain |
| Ancient Roman pre-Julian | `roman-republican` | `hc-calendars-regional` | Researching — intercalation was discretionary |
| Celtic Coligny | `coligny` | `hc-calendars-regional` | Researching — reconstruction |
| Inca | `inca` | `hc-calendars-regional` | Researching — no surviving written record |
| Discordian | `discordian` | `hc-calendars-regional` | Planned |

The sexagenary cycle covers the year, month, day and hour pillars, the twelve
double-hours (十二時辰) beginning at 23:00, and the 五虎遁 and 五鼠遁 rules
that derive the month and hour stems. Its three rival year boundaries — 立春,
the lunisolar new year, and 1 January — each have their own separately named
function, because they share their arithmetic and differ only in which days
they cover. The solar terms that fix the month pillar and the 立春 boundary
are taken as arguments, from `hc-seasons`: `hc-calendar` carries no ephemeris.

## Stage 5 — Seasonal subdivisions

Not calendars in their own right, but named subdivisions layered onto one.

| System | Crate | Status |
| --- | --- | --- |
| 二十四節気 — the 24 solar terms | `hc-seasons` | Done |
| 七十二候 — the 72 pentads (Chinese and Japanese sets; a further set is one entry) | `hc-seasons` | Done |
| 雑節 — zassetsu (節分, 彼岸, 社日, 八十八夜, 入梅, 半夏生, 土用, 二百十日, 二百二十日) | `hc-seasons` | Done |
| Moon phases as a calendar layer | `hc-seasons` | Done |
| 六曜 — rokuyō (先勝, 友引, 先負, 仏滅, 大安, 赤口) | `hc-seasons` | Done |
| 十二直 and 二十八宿 (and 二十七宿) | `hc-almanac` | Done |
| Computus cycles — golden number, dominical letter, epact, solar cycle, indiction, Julian Period | `hc-calendars-solar::cycles` | Done |
| Medieval year-start styles — Lady Day, Annunciation (Florentine and Pisan), Nativity, *more veneto*, Greek | `hc-calendars-solar::year_style` | Done |
| Roman day notation — kalends, nones, ides, *pridie*, the doubled bissextile day | `hc-format::roman` | Done |
| 九星, 七曜, 暦注下段, 選日 | `hc-almanac` | Done |
| 黄道十二宮 — Western zodiac signs (tropical), with periods | `hc-seasons` | Done |
| Sidereal signs / rāśi, with the Lahiri and other ayanamsas | `hc-seasons` | Done |
| Indian solar months over the rāśi — Sanskrit, Tamil, Bengali, Malayalam, and any tradition added as an entry | `hc-seasons` | Done |
| 十二次 — the Chinese twelvefold ecliptic division | `hc-seasons` | Done |
| Traditional Irish/Gaelic quarter days | `hc-seasons` | Planned |

## Stage 6 — Non-terrestrial

| System | Crate | Status |
| --- | --- | --- |
| Mars Sol Date (MSD) and Coordinated Mars Time (MTC) | `hc-planetary` | Done |
| Mars local mean and true solar time at a longitude | `hc-planetary` | Done |
| Darian Martian calendar | `hc-planetary` | Done |
| Mission sol counts (landing-relative) | `hc-planetary` | Done |
| Lunar day / lunation clocks | `hc-planetary` | Done |
| Rotation and orbit periods of the major bodies | `hc-planetary` | Done |
| Coordinated Lunar Time (LTC, per 2024 US policy directive) | `hc-planetary` | Researching — the standard is still being defined |

## Out of scope, with reasons

| Calendar | Why not |
| --- | --- |
| Fictional calendars from specific works (Shire Reckoning, Stardates, Imperial Dating) | Copyrighted settings. The `hc-relativity` and `hc-planetary` primitives are there so a downstream crate can build one. |
| Liturgical *ordo* for a specific denomination and year | An editorial product, not an algorithm. The movable-feast computus that underlies it is in `hc-holiday`. |
| A general timeline of historical events | No authority defines the set, so its coverage could never be stated honestly — see [policy.md](policy.md) §10. Periodic events whose set *is* externally defined, such as the Olympiads the IOC counts, are in scope. |

## Adding a calendar

1. Implement `hc_calendar::Calendar` in its own module in the right crate,
   declaring its shape in `cycles` — the compiler insists — with a `month`
   cycle exactly when its dates carry a month.
2. Add it to `register_all` so the registry picks it up.
3. Round-trip test it across its full supported range in a loop.
4. Anchor it to at least one published reference date, cited in a comment.
5. Name its positions. If its sources use one orthography that other
   languages borrow, declare the names with the shape (`CycleShape::named`)
   and cite them; where a language has its own word, add that word to
   `hc-i18n`. Era names go to `hc-i18n`.
6. Regenerate `docs/supported.md` and move its row in this table to
   **Done**.

If step 1 makes you want to add a branch to shared logic, stop — see
[policy.md](policy.md) §2.

If two authorities disagree about what the calendar does, register both under
their own identifiers rather than taking a parameter or picking a default —
see [policy.md](policy.md) §5.
