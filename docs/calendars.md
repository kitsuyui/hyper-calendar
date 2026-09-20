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
| Julian→Gregorian reform (per country) | `julian-gregorian` | `hc-calendars-solar` | Done |
| ISO 8601 week date | `iso8601` | `hc-calendars-solar` | Done |
| ISO 8601 ordinal date | `ordinal` | `hc-calendars-solar` | Done |
| Julian Day Number / MJD | `julian-day` | `hc-calendars-solar` | Done |
| Coptic | `coptic` | `hc-calendars-solar` | Done |
| Ethiopic (Amete Mihret) | `ethiopic` | `hc-calendars-solar` | Done |
| Ethiopic (Amete Alem) | `ethioaa` | `hc-calendars-solar` | Done |
| Ancient Egyptian wandering year | `egyptian` | `hc-calendars-solar` | Done |
| Armenian | `armenian` | `hc-calendars-solar` | Done |
| Solar Hijri (Persian), arithmetic | `persian` | `hc-calendars-solar` | Partial — 33-year cycle; astronomical variant in stage 3 |
| Indian national civil (Śaka) | `indian` | `hc-calendars-solar` | Done |
| Thai solar (Buddhist Era) | `buddhist` | `hc-calendars-solar` | Done |
| Minguo (Republic of China) | `roc` | `hc-calendars-solar` | Done |
| Juche (DPRK) | `juche` | `hc-calendars-solar` | Done |
| Holocene / Human Era (人類紀元) | `holocene` | `hc-calendars-solar` | Done |
| Byzantine / Anno Mundi world era | `byzantine` | `hc-calendars-solar` | Done |
| Roman *ab urbe condita* | `roman-auc` | `hc-calendars-solar` | Done |
| French Republican, arithmetic (Romme) | `french-republican` | `hc-calendars-solar` | Partial — equinox variant in stage 3 |
| Bahá'í (Badíʿ), arithmetic portion | `bahai` | `hc-calendars-solar` | Partial — Naw-Rúz is astronomical, stage 3 |
| Symmetry454 | `symmetry454` | `hc-calendars-solar` | Done |
| World Calendar (1930 proposal) | `world` | `hc-calendars-solar` | Done |

## Stage 2 — Lunar and lunisolar calendars

These need the astronomical engine, so they live behind the `lunar` feature.

| Calendar | Id | Crate | Status |
| --- | --- | --- | --- |
| Tabular Islamic, civil epoch | `islamic-civil` | `hc-calendars-lunar` | Done |
| Tabular Islamic, astronomical epoch | `islamic-tbla` | `hc-calendars-lunar` | Done |
| Umm al-Qura (Saudi official) | `islamic-umalqura` | `hc-calendars-lunar` | Done — table-driven, 1300–1600 AH |
| Observational Hijri | `islamic-rgsa` | `hc-calendars-lunar` | Partial — visibility model is a simplification |
| Hebrew | `hebrew` | `hc-calendars-lunar` | Done |
| Chinese lunisolar | `chinese` | `hc-calendars-lunar` | Done |
| Korean (Dangi) | `dangi` | `hc-calendars-lunar` | Done |
| Vietnamese | `vietnamese` | `hc-calendars-lunar` | Done |
| Japanese lunisolar (Tenpō, pre-1873) | `japanese-tenpo` | `hc-calendars-lunar` | Done |
| Tibetan (Phugpa) | `tibetan` | `hc-calendars-lunar` | Planned |
| Hindu lunisolar (Amanta and Purnimanta) | `hindu-lunar` | `hc-calendars-lunar` | Planned |
| Hindu solar (Sūrya Siddhānta) | `hindu-solar` | `hc-calendars-lunar` | Planned |
| Old Hindu (mean) lunisolar and solar | `hindu-old` | `hc-calendars-lunar` | Planned |

## Stage 3 — Astronomical variants of stage 1 calendars

Same calendars, computed from observation rather than from a cycle. They can
disagree with the arithmetic form by a day, which is exactly why both exist.

| Calendar | Id | Status |
| --- | --- | --- |
| Solar Hijri, astronomical (Tehran meridian) | `persian-astronomical` | Planned |
| French Republican, autumn equinox at Paris | `french-republican-equinox` | Planned |
| Bahá'í, Naw-Rúz from the Tehran equinox | `bahai-astronomical` | Planned |
| Ethiopian/Coptic Easter-linked movable cycle | — | See [observances.md](observances.md) |

## Stage 4 — Regional, cyclic and era calendars

| Calendar | Id | Crate | Status |
| --- | --- | --- | --- |
| Japanese imperial eras (和暦, 大化 → 令和) | `japanese` | `hc-calendars-regional` | Done |
| Chinese sexagenary cycle (干支) | `sexagenary` | `hc-calendar::cycle` | Done |
| Chinese regnal eras (年号) | `chinese-regnal` | `hc-calendars-regional` | Planned |
| Korean regnal eras | `korean-regnal` | `hc-calendars-regional` | Planned |
| Maya long count | `maya-longcount` | `hc-calendars-regional` | Done |
| Maya Tzolkʼin (260 days) | `maya-tzolkin` | `hc-calendars-regional` | Done |
| Maya Haabʼ (365 days) | `maya-haab` | `hc-calendars-regional` | Done |
| Maya calendar round | `maya-round` | `hc-calendars-regional` | Done |
| Aztec Tonalpohualli and Xiuhpohualli | `aztec` | `hc-calendars-regional` | Planned |
| Balinese Pawukon (ten concurrent cycles) | `balinese-pawukon` | `hc-calendars-regional` | Done |
| Javanese Pasaran (five-day market week) | `javanese-pasaran` | `hc-calendars-regional` | Done |
| Igbo four-day week (Izu) | `igbo` | `hc-calendars-regional` | Planned |
| Yoruba four-day week | `yoruba` | `hc-calendars-regional` | Researching — regional variants differ |
| Akan Adaduanan (42-day cycle) | `akan` | `hc-calendars-regional` | Planned |
| Nepali Bikram Sambat | `bikram-sambat` | `hc-calendars-regional` | Planned |
| Nepal Sambat | `nepal-sambat` | `hc-calendars-regional` | Planned |
| Burmese | `burmese` | `hc-calendars-regional` | Planned |
| Thai lunar (Chulasakarat) | `thai-lunar` | `hc-calendars-regional` | Planned |
| Zoroastrian (Qadimi / Shahanshahi / Fasli) | `zoroastrian` | `hc-calendars-regional` | Planned |
| Rumi (late Ottoman fiscal) | `rumi` | `hc-calendars-regional` | Planned |
| Attic (Athenian) | `attic` | `hc-calendars-regional` | Researching — reconstruction, sources conflict |
| Babylonian | `babylonian` | `hc-calendars-regional` | Researching — regnal anchoring is uncertain |
| Ancient Roman pre-Julian | `roman-republican` | `hc-calendars-regional` | Researching — intercalation was discretionary |
| Celtic Coligny | `coligny` | `hc-calendars-regional` | Researching — reconstruction |
| Inca | `inca` | `hc-calendars-regional` | Researching — no surviving written record |
| Discordian | `discordian` | `hc-calendars-regional` | Planned |

## Stage 5 — Seasonal subdivisions

Not calendars in their own right, but named subdivisions layered onto one.

| System | Crate | Status |
| --- | --- | --- |
| 二十四節気 — the 24 solar terms | `hc-seasons` | Done |
| 七十二候 — the 72 pentads (Chinese and Japanese variants) | `hc-seasons` | Done |
| 雑節 — zassetsu (節分, 彼岸, 八十八夜, 入梅, 土用, 二百十日) | `hc-seasons` | Done |
| Moon phases as a calendar layer | `hc-seasons` | Done |
| 六曜 — rokuyō (先勝, 友引, 先負, 仏滅, 大安, 赤口) | `hc-seasons` | Done |
| 十二直 and 二十八宿 | `hc-seasons` | Planned |
| Western astrological signs (tropical) | `hc-seasons` | Planned |
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
| "Perpetual" business calendars (4-4-5, 13-period retail) | Organisation-specific rather than cultural; better expressed as a downstream rule than as a calendar. |
| Liturgical *ordo* for a specific denomination and year | An editorial product, not an algorithm. The movable-feast computus that underlies it is in `hc-holiday`. |

## Adding a calendar

1. Implement `hc_calendar::Calendar` in its own module in the right crate.
2. Add it to `register_all` so the registry picks it up.
3. Round-trip test it across its full supported range in a loop.
4. Anchor it to at least one published reference date, cited in a comment.
5. Add its vocabulary to `hc-i18n` if it has month or era names.
6. Move its row in this table to **Done**.

If step 1 makes you want to add a branch to shared logic, stop — see
[policy.md](policy.md) §2.
