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
| **Partial** | Implemented with a documented restriction: a simplified model, or part of the calendar not carried |
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
| Discordian | `discordian` | `hc-calendars-solar` | Done — the five seasons, the Erisian week and St. Tib's Day of the *Principia Discordia*, on the Gregorian leap rule; the eleven named holydays |
| Nanakshahi (Sikh, 2003) | `nanakshahi` | `hc-calendars-solar` | Done — the 2003 calendar of fixed Gregorian month starts, year 1 in 1469; the SGPC's 2010 and 2014 revisions are the Bikrami calendar under the same name and are `hindu-solar-vikrami` and `hindu-lunar` |
| Bangladeshi national calendar | `bangladeshi` | `hc-calendars-solar` | Done — the Bengali months from 14 April in the Bengali era, every month on a fixed Gregorian date; the 1966 committee's lengths, adopted in 1987, through 1425, and the 2019 revision's from 1426, when Ashvin gained a day and Falgun lost one. The Falgun leap day is taken to be the Gregorian one of the February Falgun spans, the source saying only "every leap year" |
| Thai solar (Buddhist Era) | `buddhist` | `hc-calendars-solar` | Done — the modern year, from 1 January. The year as printed from 1889 to 1940, beginning on 1 April and in the Rattanakosin era until March 1913, through `buddhist::printed_year` and `printed_to_fixed`; the lunar reckoning before 1 April 1889 is refused |
| Minguo (Republic of China) | `roc` | `hc-calendars-solar` | Done |
| Juche (DPRK) | `juche` | `hc-calendars-solar` | Done |
| Holocene / Human Era (人類紀元) | `holocene` | `hc-calendars-solar` | Done |
| Japanese imperial year (皇紀) | `japanese-imperial` | `hc-calendars-solar` | Done — proleptic before the 1873 adoption |
| Rumi (Ottoman civil, 1840–1925) | `rumi` | `hc-calendars-solar` | Done — Julian days with the year on 1 Mart to 15 Şubat 1332, Gregorian days from 1 Mart 1333 (1 March 1917), the year less 584 throughout; bounded to the years it was kept |
| Byzantine / Anno Mundi world era | `byzantine` | `hc-calendars-solar` | Done |
| Roman *ab urbe condita* | `roman-auc` | `hc-calendars-solar` | Done |
| French Republican, arithmetic (Romme) | `french-republican-arithmetic` | `hc-calendars-solar` | Done — Romme's proposal; the decree's rule is `french-republican-equinox`, stage 3 |
| Bahá'í (Badíʿ), arithmetic Western rule | `bahai-arithmetic` | `hc-calendars-solar` | Done — the rule kept until 171 BE, continued proleptically |
| Bahá'í (Badíʿ), as kept | `bahai` | `hc-calendars-solar` | Done through 221 BE (19 March 2065) — the arithmetic rule to 171 BE, the Bahá'í World Centre's published table for 172–221 BE; refuses after, until `hc-astro` extends it |
| Symmetry454 | `symmetry454` | `hc-calendars-solar` | Done |
| Symmetry010 | `symmetry010` | `hc-calendars-solar` | Done |
| Revised Julian (Milanković, 1923) | `revised-julian` | `hc-calendars-solar` | Done |
| World Calendar (1930 proposal) | `world-calendar` | `hc-calendars-solar` | Done |
| Swedish calendar, 1700–1712 | `swedish-1700` | `hc-calendars-solar` | Done — the third calendar Sweden kept from 1 March 1700 to 30 February 1712, neither Julian nor Gregorian: the leap day of 1700 dropped, those of 1704 and 1708 kept by error, so a day ahead of Julian and ten behind Gregorian, and abandoned by giving February 1712 thirty days; bounded to those twelve years, since `julian` is the answer before and after them and `julian-gregorian-se` carries only the 1753 cut-over |
| Berber / Amazigh agrarian year (*fellāḥī*) | `berber` | `hc-calendars-solar` | Planned — the Julian year under Latin-derived month names, thirteen days behind the Gregorian, still kept in the rural Maghreb with its agrarian sub-seasons; the "Amazigh era" from 950 BC is a construction of the 1960s and is labelled as one |
| Mandaean | `mandaean` | `hc-calendars-solar` | Planned — 365 days with no leap rule, twelve thirty-day months named for the zodiac constellations, and the five *Parwanaya* days after the eighth month rather than at the year's end; a variation on the Egyptian pattern, once a source fixes the epoch |
| Modern Assyrian | `assyrian` | `hc-calendars-solar` | Planned — the Gregorian year under Akkadian month names from a constructed 4750 BC epoch, in diaspora use since the 1950s. The ancient *limmu* eponym years are a different and much harder object and are not this row |
| Yazidi (*Serê Sal*) | `yazidi` | `hc-calendars-solar` | Planned — the Julian year on the Seleucid era, whose new year is a weekday rule rather than a date: the first Wednesday on or after 14 April Gregorian. `hc-holiday`'s `WeekdayOnOrAfter` already says it |
| Old Icelandic *misseri* | `icelandic` | `hc-calendars-solar` | Planned — the week is the unit and the month is derived from it: 52 weeks of 364 days in two *misseri* of 26, every month always beginning on the same weekday, the year on the first Thursday after 18 April, and a leap week (*sumarauki*) rather than a leap day, under one rule before 1700 and another after. Janson, "The Icelandic Calendar"; Reingold and Dershowitz ch. 6 |
| Qumran / Jubilees 364-day year, with the *mishmarot* | `qumran` | `hc-calendars-solar` | Planned — four quarters of 91 days, thirteen whole weeks each, so every festival falls on a fixed weekday and no intercalation is specified at all; the 4Q320–330 *mishmarot* cycle the twenty-four priestly courses against it over six years inside a 294-year *otot* cycle. The scrolls are published, and a fixed-weekday year carrying a named-course cycle is a structure nothing here has |
| Soviet revolutionary weeks (*nepreryvka*, 1929–1940) | `soviet-week` | `hc-calendars-solar` | Planned — the Gregorian date kept and the week replaced: a continuous five-day week from 1929, the workforce in five colour groups on staggered rest days, then from 1931 a six-day week resting on the 6th, 12th, 18th, 24th and 30th, the 31st a working day. The five-day rest day belonged to a group of people rather than to a date, so only the six-day week's rest day follows from the day number |
| Runic calendar / *primstav* | — | `hc-calendars-solar::cycles` | Planned — a perpetual calendar carved on a stave, nineteen golden-number runes against seven dominical-letter runes, one object serving every Julian year (Ole Worm, *Computus Runicus*, 1643). The cycles it reads are Done; the reading is not |
| International Fixed (Cotsworth, 1902) | `international-fixed` | `hc-calendars-solar` | Done — thirteen months of 28 days with *Sol* between June and July, *Year Day* and *Leap Day* both outside the week, every month opening on the calendar's own Sunday, the Gregorian leap rule; Kodak's calendar from 1928 to 1989. Cotsworth, *The Rational Almanac*, on archive.org |
| Hanke–Henry Permanent | `hanke-henry` | `hc-calendars-solar` | Planned — quarters of 30, 30 and 31 days, a 364-day year always beginning Monday 1 January, and a seven-day month *Xtr* after December in the years the ISO week rule gives 53 weeks. Henry's own page still places *Xtr* between June and July in one answer, a fossil of the earlier proposal, so the leap week is placed from the rule and not from that page |
| Positivist (Comte, 1849) | `positivist` | `hc-calendars-solar` | Done — thirteen months of 28 days named Moïse to Bichat, each opening on the calendar's own Monday, the 365th day the *Fête universelle des Morts* outside the week and the *Fête générale des Saintes Femmes* after it in Gregorian leap years, year 1 in 1789; from the *Calendrier positiviste*, 4th ed. (1852), on archive.org |
| Meyer–Palmen Solilunar (1999) | `meyer-palmen` | `hc-calendars-solar` | Planned — a cycle-year-month-day count over sixty-year cycles, odd months of 29 days and even of 30, a thirteenth month *Meton* of 30 or 31; wholly arithmetic, with the two leap rules, the epoch (JDN 207 227) and the invariants stated on the Hermetic Systems page, so its property tests write themselves. One self-published source, and the most implementation-ready specification in this table |
| Yerm lunar (Palmen) | `yerm` | `hc-calendars-solar` | Planned — purely lunar: months alternating 30 and 29 days within a *yerm* of seventeen months, every third of fifteen, in a cycle of 52 yerms, 850 months and 25 101 nights, the day beginning at noon so that a night is not split. It claims nothing about the seasons, so it is not tested against them |
| Pax (Colligan, 1930) | `pax` | `hc-calendars-solar` | Researching — thirteen months of 28 days with *Columbus* between November and December, and a seven-day month *Pax* in the years whose last two digits divide by 6 or are 99, except the years divisible by 400. The leap rule as circulated may be a later reconstruction rather than Colligan's own wording, and the primary is offline, so it is not carried until read |
| Tranquility (Siggins, 1989) | `tranquility` | `hc-calendars-solar` | Researching — thirteen months named for scientists, weeks Friday to Thursday, *Armstrong Day* outside the week and *Aldrin Day* on the Gregorian leap rule, epoch 20 July 1969. The primary is *Omni*, July 1989, in print only |
| Invariable Calendar (Grosclaude, 1900) and Mastrofini's (1834) | — | `hc-calendars-solar` | Researching — two proposals usually run together: Grosclaude's four 91-day quarters of 30, 30 and 31 days with a blank New Year's Day, and Mastrofini's earlier 364-day year always beginning on a Sunday with the 365th day outside the calendar. Grosclaude's is partly sourced; Mastrofini's has no usable source at all, and its structure is not to be presented as established |

The leap-week proposals — Symmetry454 and Symmetry010, Hanke–Henry, the
Icelandic *sumarauki* — share one mechanism: a year of 364 or 371 days, never
365 or 366, so that every date keeps its weekday at the cost of six or seven
days of seasonal drift. Two leap rules are in circulation, the ISO one (71
leap weeks in 400 years, the years with 53 Thursdays), which needs a Gregorian
calendar beside it, and Bromberg's standalone 52/293, whose mean year of
365 + 71/293 days targets the mean March-equinox year rather than the mean
tropical year (Palmen, "Leap Week Calendars"). The durable sources for the
reform family are the archive.org copy of Cotsworth and the Hermetic Systems
pages: Bromberg's University of Toronto pages are gone, `worldcalendar.org`
is a parked domain, and `theworldcalendar.org` is no longer the World
Calendar Association's site and is not cited as it.

## Stage 2 — Lunar and lunisolar calendars

Their crates need the astronomical engine, so they live behind features:
`hc-calendars-lunar` behind `lunar`, `hc-calendars-indic` behind `indic`. The
tabular Hijri, Hebrew, Tibetan and Old Hindu calendars are arithmetic, and
live with their families.

| Calendar | Id | Crate | Status |
| --- | --- | --- | --- |
| Tabular Islamic, civil epoch | `islamic-civil` | `hc-calendars-lunar` | Done — the common thirty-year scheme on the Friday epoch, checked against the published closed form; the family is in [systems/hijri.md](systems/hijri.md) |
| Tabular Islamic, astronomical epoch | `islamic-tbla` | `hc-calendars-lunar` | Done — the same scheme on the Thursday epoch, one day earlier; see [systems/hijri.md](systems/hijri.md) |
| Fatimid / Ṭayyibī Bohra *Misri* | `islamic-fatimid` | `hc-calendars-lunar` | Done — the Bohra community's scheme on the Thursday epoch, anchored to its published Mawlid of 1439; see [systems/hijri.md](systems/hijri.md) |
| Umm al-Qura (Saudi official) | `islamic-umalqura` | `hc-calendars-lunar` | Done — the published table for 1300–1600 AH, refused outside it, with the rules it embodies and the announcements it was checked against in [systems/hijri.md](systems/hijri.md) |
| Observational Hijri | `islamic-rgsa` | `hc-calendars-lunar` | Partial — a prediction under one visibility criterion at Mecca, a day after the Umm al-Qura table for 58% of months and never a record of an announcement; see [systems/hijri.md](systems/hijri.md) |
| Hebrew | `hebrew` | `hc-calendars-lunar` | Done |
| Chinese lunisolar | `chinese` | `hc-calendars-lunar` | Done |
| Korean (Dangi) | `dangi` | `hc-calendars-lunar` | Done |
| Vietnamese | `vietnamese` | `hc-calendars-lunar` | Done |
| Japanese lunisolar (Tenpō, 1844–1872) | `japanese-tenpo` | `hc-calendars-lunar` | Done |
| Japanese lunisolar (Kansei, 1798–1844) | `japanese-kansei` | `hc-calendars-lunar` | Done |
| Japanese lunisolar (Hōryaku, 1755–1798) | `japanese-horyaku` | `hc-calendars-lunar` | Done |
| Japanese lunisolar (Jōkyō, 1685–1755) | `japanese-jokyo` | `hc-calendars-lunar` | Done |
| Japanese lunisolar (Senmyō, 862–1685) | `japanese-senmyo` | `hc-calendars-lunar` | Done |
| Tibetan (Phugpa) | `tibetan` | `hc-calendars-lunar` | Done — Janson's statement of the Phugpa arithmetic with exact rational constants, skipped and extra days, the sixty-year names; the Tsurphu and other traditions not carried |
| Hindu lunisolar, amānta | `hindu-lunar` | `hc-calendars-indic` | Done — the *Rashtriya Panchang*'s reckoning, true Sun and Moon at the Central Station's sunrise with the Lahiri ayanamsa, tested against two years of its tables; the rules, the checks and the sources are in [systems/hindu-calendars.md](systems/hindu-calendars.md) |
| Hindu lunisolar, pūrṇimānta | `hindu-lunar-purnimanta` | `hc-calendars-indic` | Done — the amānta tithis under the north's month names; see [systems/hindu-calendars.md](systems/hindu-calendars.md) |
| Hindu solar, Tamil | `hindu-solar-tamil` | `hc-calendars-indic` | Done — the Tamil rule for the month's first day, read off the almanac's tables; see [systems/hindu-calendars.md](systems/hindu-calendars.md) |
| Hindu solar, Malayalam (Kollam era) | `hindu-solar-malayalam` | `hc-calendars-indic` | Done — the Malabar rule; see [systems/hindu-calendars.md](systems/hindu-calendars.md), which names the two Medam rows the almanac prints differently |
| Hindu solar, Bengali (Bangabda) | `hindu-solar-bengali` | `hc-calendars-indic` | Done — the Bengal rule; see [systems/hindu-calendars.md](systems/hindu-calendars.md) |
| Hindu solar, Vikrami (Punjab, Haryana, Odisha) | `hindu-solar-vikrami` | `hc-calendars-indic` | Done — the Orissa rule; see [systems/hindu-calendars.md](systems/hindu-calendars.md) |
| Old Hindu (mean) solar and lunisolar | `hindu-old-solar`, `hindu-old-lunar` | `hc-calendars-indic` | Done — the *Ārya Siddhānta*'s mean Sun and Moon as Reingold and Dershowitz give the arithmetic, held to what a mean calendar must do since no almanac prints one; see [systems/hindu-calendars.md](systems/hindu-calendars.md) |
| Odia year counts, incl. the Anka year | — | `hc-calendars-indic` | Planned — the Anka is a regnal reckoning still printed for the Gajapati of Puri whose sequence skips every year ending in 6 and every year ending in 0 except 10, and has no year 1, so 1, 6, 16, 20, 26, 30 and 36 are absent: a pure integer mapping, and a calendar for which "the year before" means nothing |
| Tamil sixty-year names | — | `hc-calendars-indic` | Planned |
| Vira Nirvana Samvat (Jain) | `jain-svetambara`, `jain-digambara` | `hc-calendars-indic` | Planned — the oldest era still in use, on the amānta months of `hindu-lunar`, with two epochs rather than one: 7 October 527 BCE in the Śvetāmbara reckoning and 662 BCE in the Digambara, each under its own identifier |
| Assamese Bhāskarābda, Tripuri and Tulu | — | `hc-calendars-indic` | Researching — Assamese is the Bengali reckoning read sunrise to sunrise, with a contested epoch of 593 or 594 CE; Tripuri is the Bengali calendar three years apart, a pure offset; Tulu rests on a stub and community pages. Mostly name tables and a flag over the solar engine, each waiting on an authority |
| Sinhalese solar year (*Aluth Avurudu*) | — | `hc-calendars-indic` | Researching — the sidereal solar months as the Tamil calendar has them, but the year does not turn at a day boundary: the *nonagathe* of about 12 h 48 min spans the Meena to Mesha transit and the new year dawns at its midpoint, a clock instant the Ministry of Buddhasasana publishes each year as the *Avurudu Nekath Seettuwa*. A year beginning at a time is a `CivilTime` beside an `Rd`, and the published times are data; `hc-holiday` carries Sri Lanka's gazetted dates |
| Samaritan | `samaritan` | `hc-calendars-lunar` | Planned — Hebrew-like, on the High Priest's house's own conjunction calculation, a different epoch, and no postponements (*dehiyyot*), so its festivals routinely differ from the Rabbinic dates. Reingold and Dershowitz give an algorithm; the calendar as kept is still issued by the priesthood, so it is a sibling of `hebrew` with a stated range |
| Tibetan — Tsurphu, Kālacakra, Geden, Bhutanese | `tibetan-tsurphu`, `tibetan-geden`, `tibetan-bhutan` | `hc-calendars-lunar` | Planned — parameter sets over the Phugpa engine, each with its own set of skipped and doubled dates: Tsurphu mixes the full-tenet *grub rtsis* and the précis *byed rtsis* and differs from Phugpa by a whole day or a whole month; Kālacakra is the parent system, whose two sub-methods give different answers; Geden starts elsewhere in the sixty-year cycle; the Bhutanese changes core constants and the weekday computation after Pema Karpo, and is only approximately published in English. Janson, *Tibetan Calendar Mathematics*; Henning, *Kālacakra and the Tibetan Calendar*; Gantumur, *Possible Reforms of the Tibetan Lunisolar Calendar* |
| Mongolian (*Bilgiin toolol*) | `mongolian` | `hc-calendars-lunar` | Planned — the Tegüs Buyantu system of 1747, a Phugpa re-epoched to another start in the sixty-year cycle and so with a different set of doubled and omitted dates; Tsagaan Sar is the second new moon after the winter solstice, not the Chinese rule, and can differ from it by a day or a month. The Inner Mongolian "yellow calculation" keeps Chinese-style months with no doubled or omitted dates and is a calendar of its own. `hc-holiday`'s Mongolia table waits on this row |
| Javanese (*Pananggalan Jawa*) | `javanese` | `hc-calendars-lunar` | Planned — Sultan Agung's graft of 1633: the Śaka year number continued onto Islamic lunar months, so 1633 CE is 1555 AJ and the era has no founding event; days from sunset; an eight-year *windu* of named years of fixed 354 or 355 days, and a *kurup* of fifteen *windu*, 120 lunar years of 42 524 days, that locks it to the tabular Islamic thirty-year cycle, so the kurup correction is a table longer than any cycle implemented. The Pasaran week is Done, and this is the calendar it belongs to ("An ethnoarithmetic excursion into the Javanese calendar", arXiv:2012.10064) |
| Balinese Śaka (*sasih*) | `balinese-saka` | `hc-calendars-lunar` | Researching — twelve *sasih* of fifteen waxing (*penanggal*) and fifteen waning (*panglong*) days, reconciled by *ngunaratri*, two lunar days on one solar day about every 63 days so that a date number is skipped, and an intercalary *mala masa* after the eleventh or twelfth month placed so that Tilem Kapitu never falls in December. `DateFields::leap_day` can carry the skip; where the ngunaratri and the intercalation fall rests with almanac authority, so the calendar is table-driven until a published table is read. Nyepi in `hc-holiday` waits on it |
| Ancient Egyptian lunar | `egyptian-lunar` | `hc-calendars-lunar` | Researching — the religious calendar beside the civil wandering year, whose month begins on the morning the old crescent can no longer be seen, the opposite trigger from the Islamic first visibility. Parker, *The Calendars of Ancient Egypt* (SAOC 26), is the one monograph, and the trigger makes any computed form approximate |

## Stage 3 — Astronomical variants of stage 1 calendars

Same calendars, computed from observation rather than from a cycle. They can
disagree with the arithmetic form by a day, which is exactly why both exist.

| Calendar | Id | Status |
| --- | --- | --- |
| Solar Hijri, astronomical (noon, Iran Standard Time) | `persian` | Done, in `hc-calendars-equinox` — Nowruz 1404 on 21 March 2025, where Birashk's cycle says the 20th |
| French Republican, autumn equinox at Paris | `french-republican-equinox` | Done, in `hc-calendars-equinox` — the fourteen new years France kept |
| Bahá'í, Naw-Rúz from the Tehran equinox for any year | `bahai-astronomical` | Done, in `hc-calendars-equinox` — reproduces every row of the 172–221 BE table `bahai` carries, Twin Holy Birthdays included |
| Solar Hijri as kept in Afghanistan | `persian-afghan` | Planned, in `hc-calendars-equinox` — `persian` under Arabic month names where Iran's are Zoroastrian, official from 1957. In 2022 the administration moved official business to the lunar Hijri from 1 Muharram 1444, so which calendar is Afghanistan's official one is a dated fact, a `ValidFrom` for `hc-holiday` |
| Ethiopian Easter-linked movable cycle (Bahire Hasab) | — | Done, in `hc-holiday`; see [observances.md](observances.md) |
| Coptic Easter-linked movable cycle | — | Done, in `hc-holiday`; see [observances.md](observances.md) |

## Stage 4 — Regional, cyclic and era calendars

| Calendar | Id | Crate | Status |
| --- | --- | --- | --- |
| Japanese imperial eras (和暦, 大化 → 令和) | `japanese` | `hc-calendars-regional` | Done — 248 nengō from 大化 (645); days converted from 862-02-07, the first day of Senmyō-reki |
| Japanese eras, Northern Court (北朝) | `japanese-northern` | `hc-calendars-regional` | Done |
| Japanese eras, Southern Court (南朝) | `japanese-southern` | `hc-calendars-regional` | Done |
| Japanese eras, as proclaimed (改元当時) | `japanese-proclaimed` | `hc-calendars-regional` | Done |
| Chinese sexagenary cycle (干支), incl. the four pillars (四柱/八字) | `sexagenary` | `hc-calendars-regional` (arithmetic in `hc-calendar::cycle`) | Done |
| Chinese regnal eras (年号) | `chinese-regnal` | `hc-calendars-regional` | Partial — the Qing eras day by day over `chinese` from 1645 to the abdication of 1912, and the Ming, Southern Ming, Shun and Qing eras as year data with the backdated reading; the eras before the Ming wait on a source that dates them |
| Korean regnal eras | `korean-regnal` | `hc-calendars-regional` | Partial — the Korean Empire's 建陽, 光武 and 隆熙 on their proclamation days, 1896–1910; Joseon's use of the Chinese eras waits on `chinese-regnal`, and the 開國 count is a helper |
| Maya long count (GMT 584283) | `maya-longcount` | `hc-calendars-regional` | Done — the count is older than the Maya: the earliest attested Long Count, Chiapa de Corzo Stela 2, reads 7.16.3.2.13 (36 BCE) in Epi-Olmec style, its two highest digits reconstructed, and this calendar reads it |
| Maya long count (GMT+2, 584285) | `maya-longcount-gmt2` | `hc-calendars-regional` | Done |
| Maya Tzolkʼin (260 days) | `maya-tzolkin` | `hc-calendars-regional` | Done |
| Maya Haabʼ (365 days) | `maya-haab` | `hc-calendars-regional` | Done |
| Maya calendar round | `maya-round` | `hc-calendars-regional` | Done |
| Aztec Tonalpohualli | `aztec-tonalpohualli` | `hc-calendars-regional` | Done |
| Aztec Xiuhpohualli | `aztec-xiuhpohualli` | `hc-calendars-regional` | Done |
| Maya 819-day count | `maya-819` | `hc-calendars-regional` | Planned — a four-station cycle of 819 days that closes over twenty stations, 16 380 days, the least common multiple of 819 and 260, and commensurates the synodic periods of all five naked-eye planets; pure arithmetic beside the four Maya calendars that are Done. Linden and Bricker, "The Maya 819-Day Count and Planetary Astronomy", *Ancient Mesoamerica* (2023) |
| Mesoamerican 365-day years beyond the Maya and Aztec — Zapotec *yza*, Purépecha *huriyata miucua*, Zoque *hame* | `zapotec-yza`, `purepecha-year`, `zoque-hame` | `hc-calendars-regional` | Planned — eighteen months of twenty days and five unlucky days with no intercalation at all, the arithmetic of `maya-haab` and `maya-tzolkin` under other name tables; the Zapotec *piye* is arguably the oldest 260-day count. A data exercise on an engine that exists |
| Mixtec year bearer | `mixtec-year` | `hc-calendars-regional` | Planned — the Aztec 260/365 machinery with the year-bearer day forty days before the Aztec one, so the same solar year carries a name one number lower, Aztec 2 Reed being Mixtec 1 Reed; reading a Mixtec codex through `aztec-xiuhpohualli` gives a silent wrong answer, which is the reason for a row of its own (Jansen and Pérez Jiménez) |
| Living 260-day counts — Kʼicheʼ, Kaqchikel and Ixil *Cholqʼij*, Mixe *xëë tun* | `cholqij` | `hc-calendars-regional` | Researching — a mod-260 count kept unbroken through five centuries where the 365-day frame around it was not, so `maya-tzolkin` is a living calendar as well as an archaeological one; the modern anchor differs slightly between communities, and the anchor is the whole question. Tedlock, *Maya Daykeeping*; Ríos Cortés (Zenodo 5541316) |
| Muisca (*zocam*) | `muisca` | `hc-calendars-regional` | Researching — reconstruction, sources conflict: three concurrent year kinds on a vigesimal count, a rural year of twelve or thirteen lunations with a "deaf month", a common year of twenty months of thirty days, 600 days, and a priestly year of thirty-seven months, from Duquesne's account of 1795, long dismissed as invention and partly rehabilitated. The chroniclers disagree even on the week, and the two published versions of Duquesne differ (Izquierdo Peña, arXiv:0812.0574). A 600-day year is the kind of claim a library gets asked about, which is why the row exists |
| Mapuche *We Tripantu* and Aymara *Willkakuti* | — | `hc-calendars-regional` | Researching — thirteen *küyen* of 28 days, reset by observation at the June solstice and the Pleiades' heliacal reappearance, Willkakuti's boundary the solstice sunrise seen at Tiwanaku's Gate of the Sun and a Bolivian holiday since 2009; the year counts attached to both are modern reconstructions, and the sources are thin |
| Borana (Oromo) | `borana` | `hc-calendars-regional` | Researching — lunar-stellar: twelve months fixed by the conjunction of a named moon phase with one of seven stars, no week, and twenty-seven named days cycling through a 29- or 30-day month so the first names recur at its end, a day-name cycle shorter than the month, which nothing here has. It needs a star-conjunction ephemeris and carries a day or two of observational slack. Legesse, *Gada* (1973); Doyle, "The Borana Calendar Reinterpreted", *Current Anthropology* (1986); Ruggles' archaeoastronomical critique; and a great deal of unreliable repetition on the web |
| Swahili nautical year (*Nairuzi*, Mwaka Kogwa) | `swahili` | `hc-calendars-regional` | Researching — a 365-day solar year kept beside the Hijri year on the Swahili coast, whose new year has drifted to late July, which is the evidence for a non-leap year of the Persian type; the rule is inferred from the drift, not attested (*Encyclopaedia Iranica*, "East Africa") |
| East Polynesian lunar calendars — Hawaiian *Kaulana Mahina*, Marquesan, Tahitian, Tuamotuan | `hawaiian` and the rest as name tables | `hc-calendars-regional` | Researching — about thirty named nights against a 29.53-day month, so a name is periodically dropped; the Hawaiian nights in three ten-night *anahulu*, the month from the first crescent visible after sunset, a thirteenth month to hold the *makahiki* to the sidereal year. One mechanism and many name tables, which is the split [policy.md](policy.md) §2 asks for; which night is dropped is observational. West Polynesia numbers its nights instead of naming them, a different data model for the same sky. Valério, Tamburini and Corazza, "Computational analysis reveals historical trajectory of East Polynesian lunar calendars", *PLoS ONE* 21(7); Langlas, "Nā Pō o ka Malama" |
| Rapa Nui | `rapa-nui` | `hc-calendars-regional` | Researching — twelve or thirteen months of named nights with one or two intercalary nights, *hotu* and *hiro*, intercalation at the night rather than the month; Thomson recorded thirteen months and others twelve, and the *Mamari* rongorongo tablet thought to encode it is undeciphered |
| Carolinian sidereal months | — | `hc-calendars-regional` | Researching — twelve or thirteen "moons" that are independent of the moon, each named for a star and of unequal length, tied to the star compass of Satawal, Puluwat, Woleai and Ulithi (Gladwin, *East Is a Big Bird*, 1970); a non-lunar "lunar" calendar with unequal months is a shape nothing here can hold. The heliacal risings compute; the month-length conventions are island-specific and largely unpublished |
| Khmer (*Chhankitek*), Lao, Sinhalese and Tai/Shan/Dai lunisolar | `khmer`, `lao`, `sinhalese-lunar`, `shan` | `hc-calendars-regional` | Planned — siblings of `burmese` and `thai-lunar`, each with its own rule: Khmer always doubles Ashadha and always adds the leap day in Jyestha; the Tai and Shan systems number their months rather than naming them, about two ordinals ahead of the Burmese sequence, a silent cross-conversion trap. Gislén and Eade, "The Calendars of Southeast Asia 2", *JAHH* 22(3), 2019, and Eade, *The Calendrical Systems of Mainland South-East Asia* (Brill, 1995), treat the family as one system; the English Wikipedia has no article for the Lao, Shan or Dai calendars |
| Cham *Sakawi* and Sundanese *Kala Sunda* | — | `hc-calendars-regional` | Researching — the Cham keep two calendars for one people, the Ahier lunisolar and the Awal of Islamic months on an eight-year intercalation cycle, reconciled by priestly decision; the Sundanese keep a lunar and a solar system side by side, with no English account to carry them from |
| Turkic twelve-year animal cycle | — | `hc-calendar::cycle` | Researching — a bare twelve-year naming cycle without the sixty-year stem product, riding on whatever local reckoning applied, from the Old Turkic khaganates to twentieth-century Azerbaijan, and attached to different year boundaries, Nowruz in Azerbaijani use; the animal substitutions are regionally diagnostic, the dragon a fish or a crocodile (Kāshgarī, *Dīwān Lughāt al-Turk*). A labelling cycle beside `sexagenary`, not a calendar; the anchoring is the open question |
| Balinese Pawukon (thirty *wuku*, ten concurrent weeks) | `balinese-pawukon` | `hc-calendars-regional` | Done |
| Javanese Pasaran (five-day market week) | `javanese-pasaran` | `hc-calendars-regional` | Done |
| Javanese Pranata Mangsa (twelve agricultural seasons) | `pranata-mangsa` | `hc-seasons` | Researching — the two tables read disagree by a day throughout, and neither adds up. The Indonesian Wikipedia starts Kasa on 22 June; the English one starts Kaso on 23 June. The Indonesian table leaves 22 December in no season, between Kanem's end on the 21st and Kapitu's start on the 23rd. It also gives Kawolu 26 days in a common year, while its dates, 4 to 28 February, span 25. The English table starts Kanem on 11 November, where Kalima's 27 days end on the 9th. Neither names the text it follows, so there is no source to carry the table from |
| The Sun's nakṣatra (Kerala's ñāṭṭuvēla, the Deccan's rain nakṣatras) | — | `hc-calendars-indic` | Done — `nakshatra::solar_nakshatra_at` and its ingress and span functions, eight to ten minutes from Drik Panchang's 2025 transits; the periods' regional names and lore are not carried; see [systems/hindu-calendars.md](systems/hindu-calendars.md) |
| Igbo four-day week (Izu) | `igbo` | `hc-calendars-regional` | Researching — the source says the calendar is "neither universal nor synchronized" between communities and dates no market day, so there is no anchor to carry |
| Yoruba four-day week | `yoruba` | `hc-calendars-regional` | Researching — regional variants differ |
| Akan Adaduanan (42-day cycle) | `akan` | `hc-calendars-regional` | Done — the six-day and seven-day weeks against each other, anchored on the Fɔdwo of 23 January 1978 the source dates; the four dabɔne named |
| Nepali Bikram Sambat | `bikram-sambat` | `hc-calendars-indic` | Done — the months the Government of Nepal gazettes for 2080–2083 BS, read from the Saturdays its holiday notices list; elsewhere the *Sūrya Siddhānta*'s saṅkrāntis on their civil day at Kathmandu, which matches 47 of those 48 months (Magh 2082 is the one it misses). Not the Vikrami rule: with the modern Sun and Lahiri ayanamsa no hour-of-day rule fits the gazette. More gazetted years would replace the reckoning in them |
| Nepal Sambat (lunar) | `nepal-sambat` | `hc-calendars-indic` | Done — the amānta months under their Newar names, Kachhalā (Kārtika) first, the year opening at Mha Puja, the day read at Kathmandu's sunrise; tested against the Mha Puja dates of 2013, 2014, 2016 and 2017. The solar Nepal Sambat of Lalitpur is not carried: its source does not say where its leap day falls |
| Burmese | `burmese` | `hc-calendars-regional` | Done — Yan Naing Aye's arithmetic of the Myanmar Era, era by era, with the record's exceptions as data; 1 to 3000 ME |
| Thai lunar | `thai-lunar` | `hc-calendars-regional` | Done — the calendar as Thailand publishes it, its year types carried as data for 2535–2570 BE (1992–2027) and the first six months of 2571 and refused outside; the year types, how they were read off the published holy days, and why Eade's *suriyayatra* rule is not used are in [systems/thai-lunar.md](systems/thai-lunar.md) |
| Attic (Athenian) | `attic` | `hc-calendars-regional` | Researching — reconstruction, sources conflict |
| Babylonian (Seleucid era) | `babylonian` | `hc-calendars-lunar` | Done — the nineteen-year rule and the moonlag criterion at Babylon as *Calendrical Calculations* states them, SE −71 to 386 (383 BCE to 76 CE), where Parker and Dubberstein's table follows the rule without exception; their first days of the month are matched for 82.9% of months and missed by a day for the rest. Years are the Seleucid count, continued backwards before SE 1; the regnal labels the earlier tablets carry are not, and the centuries before 383 BCE, intercalated by decree, would need their table rather than a rule |
| Ancient Roman pre-Julian | `roman-republican` | `hc-calendars-regional` | Researching — intercalation was discretionary |
| Celtic Coligny | `coligny` | `hc-calendars-regional` | Researching — reconstruction |
| Seleucid era with the Macedonian months | `seleucid` | `hc-calendars-regional` | Researching — Macedonian month names on the Babylonian lunisolar mechanism, and the first continuously numbered era; the same Seleucid year begins in autumn by the court reckoning and in spring by the Babylonian, so one year number is two calendars. Parker and Dubberstein, *Babylonian Chronology 626 B.C.–A.D. 75*, give the tables; the row rides on `babylonian` |
| Shang oracle-bone reckoning | — | `hc-calendars-regional` | Researching — days counted in the sexagenary cycle, the ancestor of `sexagenary`, continuously and reliably; the year intercalated by appending a thirteenth month, and in attested cases a fourteenth and a fifteenth, with its own number rather than a repeat of another's, which `Month.leap` does not describe. The day cycle is sound and the year structure is contested |
| Pentecontad | — | `hc-calendars-regional` | Researching — seven periods of fifty days, seven weeks and an *atzeret* each, 350 days with a fifteen- or sixteen-day supplement, identified by Julius and Hildegard Lewy in the 1940s; later work on the Old Assyrian and Babylonian material finds no evidence for the Amorite origin, so reconstruction, sources conflict |
| Inca | `inca` | `hc-calendars-regional` | Researching — two incompatible reconstructions rather than none: a state lunisolar year of twelve thirty-day months and five days, and Zuidema's reading of the *ceque* system, 328 *huacas* on 41 radial lines as a 328-day sidereal-lunar year, twelve months of 27⅓ days, plus 37 days of the Pleiades' invisibility, a calendar laid out in space (Zuidema in Ruggles, ed., *Handbook of Archaeoastronomy and Ethnoastronomy*, 2015). The literature is substantial and disagrees with itself |

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
| 三伏 and 數九 — the Chinese dog days and the nine nines | `hc-seasons` | Done — 入伏 the third 庚 day from the summer solstice, 中伏 the fourth, 末伏 the first from 立秋, the solstice counted when it is itself a 庚 day, as the published dates of 2017–2030 require; the nines from the winter solstice, as the Hong Kong Observatory counts them |
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
| Quarter days and term days — England and Wales, the English cross-quarter days, Ireland, Scotland traditional and under the 1990 Act | `hc-seasons` | Done |

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
| Calendars with no unambiguous published rule — Stardates, the Dune Universal Standard Calendar, the Klingon calendar | Nothing to implement. Stardates have no official conversion, and the convention in the *TNG* writers' guide is broken by the programmes themselves. Being fictional is not the reason: `discordian` and the Darian calendar are constructed calendars from identifiable authors and are Done. |
| Calendars whose publisher permits non-commercial use only — Warhammer 40,000's Imperial Dating | Fully specified and trivial, `0.123.456.M41` being the year in a thousand parts, and excluded on policy: Games Workshop's fan policy is incompatible with a permissively licensed crate. |
| Calendars whose rule is free but whose names are not — Shire Reckoning, the Calendar of Harptos | A calendar system is a system, not expression: 37 CFR 202.1 lists "standard calendars" among the things copyright does not reach, and the arithmetic of Tolkien's Appendix D could be carried under numbered months. What the source work protects is its prose and tables, its curated month and day names, and its mark, which governs what a crate may be called; Harptos's names are Product Identity outside the open licence. A calendar without its names is not the one anyone asks for, so these wait, and the `hc-relativity` and `hc-planetary` primitives are there so a downstream crate can build one. A constructed calendar that is explicitly free, such as the Terran Computational Calendar with its public-domain mark, is in scope on the same terms as any other. |
| Raventós Symmetrical Perpetual | No identifiable author: the only artifact is a converter whose own header reads "based on the algorithms of ?", which fails step 4 of *Adding a calendar*. |
| Dreamspell / Thirteen Moon | Argüelles's calendar of thirteen months of 28 days and a day out of time, widely mistaken for the Maya Tzolkʼin, from which it deliberately departs by skipping 29 February, so the two counts drift apart. `maya-tzolkin` will not agree with a Dreamspell app, and this is why. |
| "Julian dates" on lot codes and packaging | A day-of-year code, `DDD`, `YDDD` or `YYDDD`, with an ambiguous year and no relation to the astronomical Julian Day; no normative standard defines it, USDA FSIS on egg pack dates being the nearest. The day of the year is `iso8601-ordinal`, and anything added for the code would be named for the ordinal, never `julian`. |
| Nuer and Dinka ecological reckoning | Not a counted calendar: time is structured by the phase of transhumance, the month names are activity names and no boundary is fixed (Evans-Pritchard, *The Nuer*, 1940). |
| Lakota winter counts (*waníyetu wówapi*) | The year is a name chosen afterwards for what happened in it, with no day resolution (Greene and Thornton, *The Year the Stars Fell*). |
| Anishinaabe thirteen moons; Inuit and Nisgaʼa moon-months | The moon names vary by community for the same lunation, so there is no one table to carry, and no year arithmetic rides on them. |
| Hopi and Zuni horizon calendars | The date is a function of the observer's exact standing position against named horizon landmarks, so villages legitimately differ. |
| Barasana star calendar | Seasons of unequal and variable length set by heliacal events (Hugh-Jones, *Journal of Skyscape Archaeology* 1.1, 2015); excellent ethnography, no arithmetic. |
| Māori *maramataka* | Over forty iwi versions that disagree on the month boundary itself, most beginning at the new moon (Whiro) and some at the full (Rākaunui), and a year opening at the first new moon after Matariki's heliacal rise: no canonical form to implement. The statutory Matariki holiday is a Crown schedule, not the maramataka, and is in `hc-holiday`. |
| Tongan | Twelve named lunar months with a thirteenth, *Ooa-ki-fangongo*, inserted when at the next new moon the yams and the fish do not show the appearances proper to the first month (Collocott, "Tongan Astronomy and Calendar", *Bishop Museum Occasional Papers* VIII(4), 1922): a rule perfectly well specified and still not computable, because its trigger is the state of organisms. |
| Australian Aboriginal seasonal calendars | Two to six or more seasons of unequal and unfixed length, each begun by an ecological indicator — Nyoongar's six, D'harawal's quoll calls and lilly-pilly ripening — some twenty of them published by the Bureau of Meteorology. Well sourced, and carrying no date arithmetic. |
| Pre-colonial Filipino reckoning | No year number at all, only named daylight intervals of unequal length (W. H. Scott, *Barangay*, 1994). |
| Yakut / Sakha (*Саха ыйдара*) | The year begins in late May at the first cuckoo call, the Pleiades against the moon's age re-syncs the months, and a month is added to summer every third year: the triggers are a bird and an asterism, and the sources are expedition ethnography of the eighteenth and nineteenth centuries. |
| Bulgar (Proto-Bulgarian) | Reconstruction, sources conflict: a twelve-year animal cycle with an ordinal month and no day resolution, read that way by Mikkola in 1913 and revised by Pritsak, Moskov and Dobrev, who rejects the Turkic basis for an Iranian one, from one fifteenth-century transcript with ten date pairs, a marginal note and an inscription. Several year names are unattested and there is no agreed anchor. |
| Slavic, Baltic and Basque folk calendars; Druze observance | Naming layers on the Julian or Gregorian year, or on the Hijri, not calendar systems. The Slavic case also attracts fabricated nine-day weeks and 41-day months with no chronicle or archaeological support. |
| Liturgical *ordo* for a specific denomination and year | An editorial product, not an algorithm. The movable-feast computus that underlies it is in `hc-holiday`. |
| A general timeline of historical events | No authority defines the set, so its coverage could never be stated honestly — see [policy.md](policy.md) §10. Periodic events whose set *is* externally defined, such as the Olympiads the IOC counts, are in scope. |

## Adding a calendar

0. If the calendar is one a maintainer cannot be expected to know — year
   types, a reconstruction, a rule that depends on a place or an authority —
   write it up first under [`systems/`](systems/README.md), from its sources,
   and add those sources to [`references.bib`](references.bib). The module
   documentation will summarise the document and name it
   ([policy.md §12](policy.md)).
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
