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
| Julian→Gregorian reform (per country) | `julian-gregorian-<polity>`, 14 of them | `hc-calendars-solar` | Done — fourteen cut-overs, three of them on a document read, Serbia's on its law as a newspaper quotes it and ten on secondary sources, with a regional table of adoptions by country beside them; see [systems/gregorian-reform.md](systems/gregorian-reform.md) |
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
| French Republican, arithmetic (Romme) | `french-republican-arithmetic` | `hc-calendars-solar` | Done — Romme's proposed rule of 1795, never adopted; see [systems/equinox-calendars.md](systems/equinox-calendars.md) |
| Bahá'í (Badíʿ), arithmetic Western rule | `bahai-arithmetic` | `hc-calendars-solar` | Done — the Western rule kept until 171 BE, continued; see [systems/equinox-calendars.md](systems/equinox-calendars.md) |
| Bahá'í (Badíʿ), as kept | `bahai` | `hc-calendars-solar` | Done through 221 BE — the Western rule to 171 BE and the World Centre's table after; see [systems/equinox-calendars.md](systems/equinox-calendars.md) |
| Symmetry454 | `symmetry454` | `hc-calendars-solar` | Done |
| Symmetry010 | `symmetry010` | `hc-calendars-solar` | Done |
| Revised Julian (Milanković, 1923) | `revised-julian` | `hc-calendars-solar` | Done |
| World Calendar (1930 proposal) | `world-calendar` | `hc-calendars-solar` | Done |
| Swedish calendar, 1700–1712 | `swedish-1700` | `hc-calendars-solar` | Done — the third calendar Sweden kept from 1 March 1700 to 30 February 1712, neither Julian nor Gregorian: the leap day of 1700 dropped, those of 1704 and 1708 kept by error, so a day ahead of Julian and ten behind Gregorian, and abandoned by giving February 1712 thirty days; bounded to those twelve years, since `julian` is the answer before and after them and `julian-gregorian-se` carries only the 1753 cut-over |
| Berber / Amazigh agrarian year (*fellāḥī*) | `berber` | `hc-calendars-solar` | Done — the Julian year under the Kabyle forms of the Latin-derived month names, 1 Yennayer on Julian 1 January (14 January Gregorian this century), in the "Amazigh era" of 950 BC that the Paris Académie berbère constructed in the 1960s and first printed in 1980, which the module and [systems/berber.md](systems/berber.md) label as such; Algeria's 12 January holiday is `hc-holiday`'s, not the calendar's. The agrarian sub-seasons are not carried |
| Mandaean | `mandaean` | `hc-calendars-solar` | Done — 365 days with no leap rule, twelve thirty-day months named for the zodiac constellations, and the five *Parwanaia* after the eighth month as a thirteenth named position; anchored to Drower's 1930s dates and Häberl's 18 July 2019, and numbered in Häberl's era after the creation of Adam, 481,343 AA from that day, which is a scholar's reckoning and not a number Mandaeans write ([systems/mandaean.md](systems/mandaean.md)) |
| Modern Assyrian | `assyrian` | `hc-calendars-solar` | Done — the Gregorian months from 1 Neesan = 1 April under the Syriac names AINA prints, the year the Gregorian one plus 4750 from April and one less before it, the epoch fixed by Jean Alkhas in *Gilgamesh* in 1955 ([systems/assyrian.md](systems/assyrian.md)). The ancient *limmu* eponym years are a different and much harder object and are not this row |
| Yazidi (*Serê Sal*) | `yazidi` | `hc-calendars-solar` | Done — the year from Serêsal, the first Wednesday of Nisan in the Eastern (Julian) calendar, which is the first Wednesday on or after 14 April Gregorian this century, in the community's count of the Gregorian year plus 4750; carried as the year and the day of the year over the Julian date, since a weekday new year makes a 364- or 371-day year that a month-and-day shape cannot hold, and no month table was found in the sources. The "Seleucid calendar" of the sources is the Julian month scheme, not a year count ([systems/yazidi.md](systems/yazidi.md)) |
| Old Icelandic *misseri* | `icelandic`, `icelandic-julian` | `hc-calendars-solar` | Done — 52 weeks in two *misseri*, twelve thirty-day months always beginning on the same weekday with the four *aukanætur* after the third, and the leap week *sumarauki* whenever the next First Day of Summer is 53 weeks away; summer on the first Thursday on or after 19 April Gregorian since 1700 (`icelandic`), on or after 9 April Julian before (`icelandic-julian`), the two parting at Midsummer 1702. Janson, "The Icelandic Calendar", *Scripta Islandica* 62 (2011); the Almanac's end-of-summer leap week until 1928 and the Friday winter are not carried ([systems/icelandic.md](systems/icelandic.md)) |
| Qumran / Jubilees 364-day year, with the *mishmarot* | `qumran` | `hc-calendars-solar` | Done — four quarters of 30, 30 and 31 days from a Wednesday, no intercalation, so it drifts through the seasons and says so; the twenty-four priestly courses a week at a time from Gamul, closing after six years, as Talmon restores 4Q319–330. The epoch and year count are this library's convention, the Wednesday on or after 21 March AD 1 (Julian), since nothing read correlates the year to a Julian day; the *Otot* cycle of 4Q319 is not carried ([systems/qumran.md](systems/qumran.md)) |
| Soviet revolutionary weeks (*nepreryvka*, 1929–1940) | `soviet-week` | `hc-calendars-solar` | Done — the Gregorian date with the week of the decrees: the continuous five-day week from 1 October 1929, the revolutionary holidays outside it, then from 1 December 1931 the six-day week resting on the 6th, 12th, 18th, 24th and 30th and on 1 March, the 31st in no week, to 26 June 1940; the five-day rest day belonged to a group of people, so the day of the week is carried and the group's colour is not ([systems/soviet-week.md](systems/soviet-week.md)) |
| Runic calendar / *primstav* | — | `hc-calendars-solar::cycles::runic` | Done — a reading of the Julian date, not a calendar: the day-letter rune of every day, ᚠ ᚢ ᚦ ᚬ ᚱ ᚴ ᚼ for A–G, and the golden-number rune of the new moons, the sixteen younger-futhark runes and ᛮ ᛯ ᛰ for 1–19, in the old series the Swedish staffs carry before 1690, from the Julian ecclesiastical lunar calendar of the *Explanatory Supplement* (1961), table 14.4; Easter read off it equals the Julian computus for 326–1582 ([systems/runic-calendar.md](systems/runic-calendar.md)). The corrected golden numbers of the staffs after 1690 are Planned, waiting on a source that tabulates them; the Norwegian *primstav*'s feast marks vary by staff and no source read tabulates one, so they are not carried |
| International Fixed (Cotsworth, 1902) | `international-fixed` | `hc-calendars-solar` | Done — thirteen months of 28 days with *Sol* between June and July, *Year Day* and *Leap Day* both outside the week, every month opening on the calendar's own Sunday, the Gregorian leap rule; Kodak's calendar from 1928 to 1989. Cotsworth, *The Rational Almanac*, on archive.org |
| Hanke–Henry Permanent | `hanke-henry` | `hc-calendars-solar` | Done — quarters of 30, 30 and 31 days, a 364-day year always beginning Monday 1 January, and the seven-day *Xtr*, month 13, at the end of December in the years whose Gregorian year begins or ends on a Thursday, the ISO long years, as the authors' own site states it and lists the years to 3001; the 2004 form with the leap week between June and July is superseded and not carried ([systems/hanke-henry.md](systems/hanke-henry.md)) |
| Positivist (Comte, 1849) | `positivist` | `hc-calendars-solar` | Done — thirteen months of 28 days named Moïse to Bichat, each opening on the calendar's own Monday, the 365th day the *Fête universelle des Morts* outside the week and the *Fête générale des Saintes Femmes* after it in Gregorian leap years, year 1 in 1789; from the *Calendrier positiviste*, 4th ed. (1852), on archive.org |
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
tropical year (Palmen, "Leap Week Calendars"). Hanke–Henry takes the ISO
rule. The Icelandic calendar has a third, older than both: the leap week
falls wherever it keeps the First Day of Summer in a Thursday window of the
Julian or Gregorian calendar, which under the Gregorian window also gives 71
in 400 years, in different years from ISO's. The Qumran 364-day year, the
same shape with no leap rule at all, drifts. The durable sources for the
reform family are the archive.org copy of Cotsworth and the Hermetic Systems
pages: Bromberg's University of Toronto pages are gone, `worldcalendar.org`
is a parked domain, and `theworldcalendar.org` is no longer the World
Calendar Association's site and is not cited as it.

## Stage 2 — Lunar and lunisolar calendars

Their crates need the astronomical engine, so they live behind features:
`hc-calendars-lunar` behind `lunar`, `hc-calendars-indic` behind `indic`. The
tabular Hijri, Hebrew, Tibetan and Old Hindu calendars are arithmetic, and
live with their families; so are the two lunar proposals, Meyer–Palmen and
Yerm, which live with the lunar calendars because their months are lunations.

| Calendar | Id | Crate | Status |
| --- | --- | --- | --- |
| Tabular Islamic, civil epoch | `islamic-civil` | `hc-calendars-lunar` | Done — the common thirty-year scheme on the Friday epoch, checked against the published closed form; the family is in [systems/hijri.md](systems/hijri.md) |
| Tabular Islamic, astronomical epoch | `islamic-tbla` | `hc-calendars-lunar` | Done — the same scheme on the Thursday epoch, one day earlier; see [systems/hijri.md](systems/hijri.md) |
| Fatimid / Ṭayyibī Bohra *Misri* | `islamic-fatimid` | `hc-calendars-lunar` | Done — the Bohra community's scheme on the Thursday epoch, anchored to its published Mawlid of 1439; see [systems/hijri.md](systems/hijri.md) |
| Umm al-Qura (Saudi official) | `islamic-umalqura` | `hc-calendars-lunar` | Done — the published table for 1300–1600 AH, refused outside it, with the rules it embodies and the announcements it was checked against in [systems/hijri.md](systems/hijri.md) |
| Observational Hijri | `islamic-rgsa` | `hc-calendars-lunar` | Partial — a prediction under one visibility criterion at Mecca, a day after the Umm al-Qura table for 58% of months and never a record of an announcement; see [systems/hijri.md](systems/hijri.md) |
| Hebrew | `hebrew` | `hc-calendars-lunar` | Done — the fixed calendar as Maimonides states it, checked against two published dates of 5784 and its own structural rules over 9 999 years; see [systems/hebrew.md](systems/hebrew.md) |
| Chinese lunisolar | `chinese` | `hc-calendars-lunar` | Done — the modern rule at Beijing's meridian from the Shíxiàn reform of 1645, checked against the published new years; see [systems/east-asian-lunisolar.md](systems/east-asian-lunisolar.md) |
| Korean (Dangi) | `dangi` | `hc-calendars-lunar` | Done — the same rule read at Seoul through five changes of meridian, which is why Seollal 1988 fell a day after Chinese New Year; see [systems/east-asian-lunisolar.md](systems/east-asian-lunisolar.md) |
| Vietnamese | `vietnamese` | `hc-calendars-lunar` | Done — the same rule at UT+7 from 1968, which put Tết 1985 a lunation before Chinese New Year; see [systems/east-asian-lunisolar.md](systems/east-asian-lunisolar.md) |
| Japanese lunisolar (Tenpō, 1844–1872) | `japanese-tenpo` | `hc-calendars-lunar` | Done |
| Japanese lunisolar (Kansei, 1798–1844) | `japanese-kansei` | `hc-calendars-lunar` | Done |
| Japanese lunisolar (Hōryaku, 1755–1798) | `japanese-horyaku` | `hc-calendars-lunar` | Done |
| Japanese lunisolar (Jōkyō, 1685–1755) | `japanese-jokyo` | `hc-calendars-lunar` | Done |
| Japanese lunisolar (Senmyō, 862–1685) | `japanese-senmyo` | `hc-calendars-lunar` | Done |
| Tibetan (Phugpa) | `tibetan` | `hc-calendars-lunar` | Done — Janson's Phugpa arithmetic in exact rationals, checked against his tables and the published Losars; see [systems/tibetan-phugpa.md](systems/tibetan-phugpa.md) |
| Meyer–Palmen Solilunar (1999) | `meyer-palmen` | `hc-calendars-lunar` | Done — lunisolar, so beside the Hebrew calendar rather than the solar ones: cycle-year-month-day over sixty-year cycles, odd months of 29 days and even of 30, a thirteenth month *Meton* of 30 or 31 in a long year; year n = 60·cycle + year is long when n·2519 mod 6840 < 2519, and the kth long year's Meton has 31 days when k·1328 mod 2519 < 1328; 000-01-01-01 on JDN 207 227, Sunday 8 April 4146 BC. Meyer's and Palmen's Hermetic Systems pages, with every correspondence row, Palmen's table for 102-25 to 102-44 and the frequency table of 4001 New Year's Days reproduced ([systems/meyer-palmen.md](systems/meyer-palmen.md)) |
| Yerm lunar (Palmen) | `yerm` | `hc-calendars-lunar` | Done — purely lunar: months alternating 30 and 29 nights in a *yerm* of seventeen months, fifteen when its number in the cycle is divisible by three, 52 yerms, 850 months and 25 101 nights to a cycle, numbered from cycle 1 on JDN 1 948 379; the night begins at noon and is placed on the civil day whose noon begins it, as Palmen's tables and the Julian Day do. Palmen's Hermetic Systems page, with its tables of new yerms, cycles and months reproduced; it claims nothing about the seasons, so it is not tested against them ([systems/yerm.md](systems/yerm.md)) |
| Hindu lunisolar, amānta | `hindu-lunar` | `hc-calendars-indic` | Done — the *Rashtriya Panchang*'s reckoning, true Sun and Moon at the Central Station's sunrise with the Lahiri ayanamsa, tested against two years of its tables; the rules, the checks and the sources are in [systems/hindu-calendars.md](systems/hindu-calendars.md) |
| Hindu lunisolar, pūrṇimānta | `hindu-lunar-purnimanta` | `hc-calendars-indic` | Done — the amānta tithis under the north's month names; see [systems/hindu-calendars.md](systems/hindu-calendars.md) |
| Hindu solar, Tamil | `hindu-solar-tamil` | `hc-calendars-indic` | Done — the Tamil rule for the month's first day, read off the almanac's tables; see [systems/hindu-calendars.md](systems/hindu-calendars.md) |
| Hindu solar, Malayalam (Kollam era) | `hindu-solar-malayalam` | `hc-calendars-indic` | Done — the Malabar rule; see [systems/hindu-calendars.md](systems/hindu-calendars.md), which names the two Medam rows the almanac prints differently |
| Hindu solar, Bengali (Bangabda) | `hindu-solar-bengali` | `hc-calendars-indic` | Done — the Bengal rule; see [systems/hindu-calendars.md](systems/hindu-calendars.md) |
| Hindu solar, Vikrami (Punjab, Haryana, Odisha) | `hindu-solar-vikrami` | `hc-calendars-indic` | Done — the Orissa rule; see [systems/hindu-calendars.md](systems/hindu-calendars.md) |
| Old Hindu (mean) solar and lunisolar | `hindu-old-solar`, `hindu-old-lunar` | `hc-calendars-indic` | Done — the *Ārya Siddhānta*'s mean Sun and Moon as Reingold and Dershowitz give the arithmetic, held to what a mean calendar must do since no almanac prints one; see [systems/hindu-calendars.md](systems/hindu-calendars.md) |
| Odia Anka (Gajapati of Puri) | `odia-anka` | `hc-calendars-indic` | Done — the regnal count Sewell and Dikshit describe (Art. 64): the year turns at Suniā, Bhādrapada śukla 12, over the pūrṇimānta months, and drops every number ending in 6 and every one ending in 0 but 10; a reign's first full year is its 2nd Anka, so 1 never names a year either. An integer mapping from the full year of the reign to the number printed, and its inverse, which refuses 1, 6, 16, 20 and the rest; the Anka is never computed from another by arithmetic. Pinned to Dibyasingha Deb, Gajapati from July 1970, whose 2nd Anka opened at the Suniā of 1970, and checked against the Anka he declared at Suniā in 2011 (52), 2024 (68), 2025 (69) and 2026 (71), and against Sewell and Dikshit's first days of the 2nd Anka of four reigns, 1797 to 1859; the Amli year of the same Suniā is an extra field. An intercalary Bhādrapada and a śukla 12 without a sunrise are the library's rule, not a source's; see [systems/odia-anka.md](systems/odia-anka.md) |
| Odia Amli and Vilayati years | — | `hc-calendars-indic` | Planned — the Amli year begins on Bhādrapada śukla 12 with its epoch in A.D. 592–93, but its months are solar, each from the saṅkrānti (Sewell and Dikshit, Arts. 28, 52 and 71); the Vilayati year has the same number and begins at the Kanyā saṅkrānti. Each is the Orissa-rule solar month under a year of its own. The Amli number is already `odia-anka`'s `amli-year` |
| Tamil sixty-year names | `hindu-solar-tamil` | `hc-calendars-indic` | Done — the southern sixty-year cycle, Prabhava to Kṣaya, by Sewell and Dikshit's rule (Art. 62: the current Śaka year plus 11, modulo 60), written as the extra field `samvatsara` and declared as a sixty-name cycle; the names in Tamil script are the *Tamil Lexicon*'s, in `hc-i18n`'s `ta`, and in English Sewell and Dikshit's Sanskrit forms. Checked against their worked examples of 1752, 1803–04 and 1822 and the Tamil New Year of 2024 (Krodhi) and 2025 (Visvavasu); see [systems/hindu-calendars.md](systems/hindu-calendars.md) |
| Southern sixty-year names on the lunisolar year | — | `hc-calendars-indic` | Planned — the same cycle and the same rule name the lunisolar year south of the Narmada from Chaitra śukla 1, the Telugu and Kannada Ugādi (Sewell and Dikshit, Art. 62, and their Telugu example of 1822, Chitrabhanu); an extra field on an amānta calendar read as the south's almanacs read it, which is not the registered `hindu-lunar` |
| Northern Bārhaspatya sixty-year cycle | — | `hc-calendars-indic` | Planned — the same sixty names reckoned by Jupiter's mean motion through the signs, 361.027 days a year by the *Sūrya Siddhānta*, so that one name is expunged about every 85 years; "the northern samvatsara has advanced by 12 on the southern" by 1896 (Sewell and Dikshit, Arts. 53–62; 1822 was Vijaya in the north and Chitrabhanu in the south). A cycle of its own, not the southern one under a parameter, because the literature keeps the two apart |
| Vira Nirvana Samvat (Jain) | `vira-nirvana-samvat` | `hc-calendars-indic` | Done — the amānta months of `hindu-lunar` under the era of 527 BCE, 605 years and 5 months before the Śaka era, the year opening at Kārtika śukla 1, the day after Dīpāvalī, checked against the Oshwal Association's calendar of 2025 (2551, and 2552 from 22 October). One identifier, not the two planned: no source read gives the Digambara epoch of 662 BCE, and the ones read have Digambara scholars uphold 527 BCE too, so a second name would mark no disagreement; see [systems/vira-nirvana-samvat.md](systems/vira-nirvana-samvat.md) |
| Assamese Bhāskarābda, Tripuri and Tulu | — | `hc-calendars-indic` | Researching — Assamese is the Bengali reckoning read sunrise to sunrise, with a contested epoch of 593 or 594 CE; Tripuri is the Bengali calendar three years apart, a pure offset; Tulu rests on a stub and community pages. Mostly name tables and a flag over the solar engine, each waiting on an authority |
| Sinhalese solar year (*Aluth Avurudu*) | — | `hc-calendars-indic` | Researching — the sidereal solar months as the Tamil calendar has them, but the year does not turn at a day boundary: the *nonagathe* of about 12 h 48 min spans the Meena to Mesha transit and the new year dawns at its midpoint, a clock instant the Ministry of Buddhasasana publishes each year as the *Avurudu Nekath Seettuwa*. A year beginning at a time is a `CivilTime` beside an `Rd`, and the published times are data; `hc-holiday` carries Sri Lanka's gazetted dates |
| Samaritan | `samaritan` | `hc-calendars-lunar` | Done — Reingold and Dershowitz's modern calculation: the month from the true conjunction at or before apparent noon at Mount Gerizim, the first month after Julian 11 March, the Entry Era of 1639 BCE turning at the sixth month, 1900–2100 by this library's choice. Agrees with the community's Passover sacrifices of 2017–2020 and its months of autumn 2026, puts 2016's a day late, and allows a 7 April Passover the community says it never keeps; the priesthood's own calculation was not read. See [systems/samaritan.md](systems/samaritan.md) |
| Tibetan — Tsurphu, Kālacakra, Geden, Bhutanese | `tibetan-tsurphu`, `tibetan-geden`, `tibetan-bhutan` | `hc-calendars-lunar` | Planned — parameter sets over the Phugpa engine, each with its own set of skipped and doubled dates: Tsurphu mixes the full-tenet *grub rtsis* and the précis *byed rtsis* and differs from Phugpa by a whole day or a whole month; Kālacakra is the parent system, whose two sub-methods give different answers; Geden starts elsewhere in the sixty-year cycle; the Bhutanese changes core constants and the weekday computation after Pema Karpo, and is only approximately published in English. Janson, *Tibetan Calendar Mathematics*; Henning, *Kālacakra and the Tibetan Calendar*; Gantumur, *Possible Reforms of the Tibetan Lunisolar Calendar* |
| Mongolian (*Bilgiin toolol*) | `mongolian` | `hc-calendars-lunar` | Planned — the Tegüs Buyantu system of 1747, a Phugpa re-epoched to another start in the sixty-year cycle and so with a different set of doubled and omitted dates; Tsagaan Sar is the second new moon after the winter solstice, not the Chinese rule, and can differ from it by a day or a month. The Inner Mongolian "yellow calculation" keeps Chinese-style months with no doubled or omitted dates and is a calendar of its own. `hc-holiday`'s Mongolia table waits on this row |
| Javanese (*Pananggalan Jawa*) | `javanese`, `javanese-yogyakarta`, `javanese-aboge` | `hc-calendars-lunar` | Done — Sultan Agung's graft of 1633, the Hijri months under the Śaka year number, 1 Sura 1555 on Friday 8 July 1633; days from sunset; the eight-year *windu* with Ehe, Dal and Jimakir of 355 days, and the *kurup* that drops the last long year's day every 120 years to stay on the tabular Hijri cycle. The kurup correction is a list of kurup with their first and last years, from R. Tanaya's *Kabudayan Paugêraning Taun Jawa* (1971): Pakubuwana V's change of 1748 and Surakarta's order of 1935 that began Asapon on 24 March 1936, projected to Jimakir 2346. Three reckonings under their own names, because the literature has three: Surakarta's, the standard one; Yogyakarta's, which ran the old kurup to 1794 (Karjanto and Beauducel, arXiv:2012.10064); and the Aboge communities', who never took the change of 1936, checked against their fasts and Idul Fitri of 2017, 2024 and 2025. Every 1 Sura of Tanaya's tables and 1440 of his month openings agree with the pasaran and *wuku* of the regional crate, save four misprints; the Surakarta court's old adjustments of the Dal months, which Karjanto and Beauducel's tables carry, are not; see [systems/javanese.md](systems/javanese.md) |
| Balinese Śaka (*sasih*) | `balinese-saka` | `hc-calendars-lunar` | Researching — twelve *sasih* of fifteen waxing (*penanggal*) and fifteen waning (*panglong*) days, reconciled by *ngunaratri*, two lunar days on one solar day about every 63 days so that a date number is skipped, and an intercalary *mala masa* after the eleventh or twelfth month placed so that Tilem Kapitu never falls in December. `DateFields::leap_day` can carry the skip; where the ngunaratri and the intercalation fall rests with almanac authority, so the calendar is table-driven until a published table is read. Nyepi in `hc-holiday` waits on it |
| Ancient Egyptian lunar | `egyptian-lunar` | `hc-calendars-lunar` | Researching — the religious calendar beside the civil wandering year, whose month begins on the morning the old crescent can no longer be seen, the opposite trigger from the Islamic first visibility. Parker, *The Calendars of Ancient Egypt* (SAOC 26), is the one monograph, and the trigger makes any computed form approximate |

## Stage 3 — Astronomical variants of stage 1 calendars

Same calendars, computed from observation rather than from a cycle. They can
disagree with the arithmetic form by a day, which is exactly why both exist.

| Calendar | Id | Status |
| --- | --- | --- |
| Solar Hijri, astronomical (noon, Iran Standard Time) | `persian` | Done, in `hc-calendars-equinox` — Nowruz 1404 on 21 March 2025, where Birashk's cycle says the 20th |
| French Republican, autumn equinox at Paris | `french-republican-equinox` | Done, in `hc-calendars-equinox` — the decree's rule, reproducing the fourteen new years France kept; see [systems/equinox-calendars.md](systems/equinox-calendars.md) |
| Bahá'í, Naw-Rúz from the Tehran equinox for any year | `bahai-astronomical` | Done, in `hc-calendars-equinox` — the 2015 rule, reproducing the World Centre's table for 172–221 BE; see [systems/equinox-calendars.md](systems/equinox-calendars.md) |
| Solar Hijri as kept in Afghanistan | `persian-afghan` | Done, in `hc-calendars-equinox` — `persian`'s days under the Arabic names of the zodiac signs, حمل … حوت in Dari, with Pashto and English names in `hc-i18n`; civil from 1 Hamal 1336 (21 March 1957), when the month lengths were fixed (Balland, *Encyclopaedia Iranica*), to 7 Asad 1401 (29 July 2022), the eve of 1 Muharram 1444, from which official correspondence is dated by the lunar Hijri year, and in use after it beside the lunar dates. Iran's noon decides Nowruz; a noon at Kabul would move 1 Hamal in 1342, 1346, 1375 and 1379, which no authority read settles. `hc-holiday`'s Afghanistan table dates 24 and 28 Asad and 26 Dalw in it, as `CalendarSystem::SOLAR_HIJRI_AFGHAN` |
| Ethiopian Easter-linked movable cycle (Bahire Hasab) | — | Done, in `hc-holiday`; see [observances.md](observances.md) |
| Coptic Easter-linked movable cycle | — | Done, in `hc-holiday`; see [observances.md](observances.md) |

## Stage 4 — Regional, cyclic and era calendars

| Calendar | Id | Crate | Status |
| --- | --- | --- | --- |
| Japanese imperial eras (和暦, 大化 → 令和) | `japanese` | `hc-calendars-regional` | Done — 248 nengō from 大化 (645); days converted from 862-02-07, the first day of Senmyō-reki |
| Japanese eras, Northern Court (北朝) | `japanese-northern` | `hc-calendars-regional` | Done |
| Japanese eras, Southern Court (南朝) | `japanese-southern` | `hc-calendars-regional` | Done |
| Japanese eras, as proclaimed (改元当時) | `japanese-proclaimed` | `hc-calendars-regional` | Done |
| Chinese sexagenary cycle (干支), incl. the four pillars (四柱/八字) | `sexagenary` | `hc-calendars-regional` (arithmetic in `hc-calendar::cycle`) | Done — days, years under three named boundaries, months and hours, with nine readings; see [systems/sexagenary-cycle.md](systems/sexagenary-cycle.md) |
| Chinese regnal eras (年号) | `chinese-regnal` | `hc-calendars-regional` | Partial — the Qing eras day by day over `chinese` from 1645 to the abdication of 1912, and the Ming, Southern Ming, Shun and Qing eras as year data with the backdated reading; the eras before the Ming wait on a source that dates them |
| Korean regnal eras | `korean-regnal` | `hc-calendars-regional` | Partial — the Korean Empire's 建陽, 光武 and 隆熙 on their proclamation days, 1896–1910; Joseon's use of the Chinese eras waits on `chinese-regnal`, and the 開國 count is a helper |
| Maya long count (GMT 584283) | `maya-longcount` | `hc-calendars-regional` | Done — twenty baktun under 584 283, reading Chiapa de Corzo Stela 2's 7.16.3.2.13 through to 13.0.0.0.0; the constants, the worked readings and the sources are in [systems/mesoamerican-counts.md](systems/mesoamerican-counts.md) |
| Maya long count (GMT+2, 584285) | `maya-longcount-gmt2` | `hc-calendars-regional` | Done — the same count two days later, a calendar of its own; see [systems/mesoamerican-counts.md](systems/mesoamerican-counts.md) |
| Maya Tzolkʼin (260 days) | `maya-tzolkin` | `hc-calendars-regional` | Done — the 260-day cycle with its round number, anchored to 584 283; see [systems/mesoamerican-counts.md](systems/mesoamerican-counts.md) |
| Maya Haabʼ (365 days) | `maya-haab` | `hc-calendars-regional` | Done — the 365-day vague year, days counted from 0 and no leap day; see [systems/mesoamerican-counts.md](systems/mesoamerican-counts.md) |
| Maya calendar round | `maya-round` | `hc-calendars-regional` | Done — the 18 980-day pairing, one combination in five accepted; see [systems/mesoamerican-counts.md](systems/mesoamerican-counts.md) |
| Maya Tzolkʼin, Haabʼ and calendar round under GMT+2 | `maya-tzolkin-gmt2`, `maya-haab-gmt2`, `maya-round-gmt2` | `hc-calendars-regional` | Done — the same three cycles anchored to 584 285, so that they read beside `maya-longcount-gmt2` as the 584 283 ones read beside `maya-longcount`; see [systems/mesoamerican-counts.md](systems/mesoamerican-counts.md) |
| Aztec Tonalpohualli | `aztec-tonalpohualli` | `hc-calendars-regional` | Done — the 260-day count under Caso's anchor of 13 August 1521; see [systems/mesoamerican-counts.md](systems/mesoamerican-counts.md) |
| Aztec Xiuhpohualli | `aztec-xiuhpohualli` | `hc-calendars-regional` | Done — the uncorrected 365-day year under the same anchor; see [systems/mesoamerican-counts.md](systems/mesoamerican-counts.md) |
| Maya 819-day count | `maya-819`, `maya-819-gmt2` | `hc-calendars-regional` | Done — the stations 819 days apart from 1 Caban 5 Cumku, three days before 0.0.0.0.0, under the four colour-directions, carried over Linden and Bricker's twenty stations of 16 380 days, the least common multiple of 819 and 260 (their paper's abstract only read), under both correlations; anchored to Lounsbury's reading of the Temple of the Cross and of Pakal's birth, 20 days after a south station; see [systems/mesoamerican-counts.md](systems/mesoamerican-counts.md) |
| Zapotec *yza* (365 days) | `zapotec-yza` | `hc-calendars-regional` | Done — the colonial Northern Zapotec year of Villa Alta: eighteen months of twenty days and a *quicholla* of five, never intercalated, named *toohua* to *gohui* in the order of Manuscript 85, the one Zapotec month list known (Alcina Franch 1966, as Urcid, *Zapotec Hieroglyphic Writing*, 2001, Table 3.5, prints it), and each year named by the 260-day day it begins on, Earthquake, Wind, Deer or Soaproot; anchored to Justeson and Tavárez's correlation as Tavárez and Justeson (*Ancient Mesoamerica* 19, 2008) report it — the year 11 Earthquake began on 23 February 1695 (Gregorian), the 183rd day was 23 August from 1696 to 1703, and the 260-day count is Caso's Mexica one, 63 days out of step with `aztec-xiuhpohualli`'s year. The manuscript's own month lengths, which total 364, are not carried, and Cline's Julian reading of 1696 as 7 Motion is recorded, not registered: no start day for it was read. See [systems/mesoamerican-years.md](systems/mesoamerican-years.md) |
| Purépecha year, *huriyata miucua* | `purepecha-year` | `hc-calendars-regional` | Planned — missing a list of all eighteen month names and an anchor. The *Relación de Michoacán* names thirteen months and dates four, and one more, not named, on 14 November (Caso, "The Calendar of the Tarascans", *American Antiquity* 9, 1943, first page only read); the rest of Caso's article is behind the publisher's login, Paso y Troncoso's "Calendario de los tarascos" (1887) could not be fetched, and no year bearer was found in a source read. See [systems/mesoamerican-years.md](systems/mesoamerican-years.md) |
| Zoque year, *hame* | `zoque-hame` | `hc-calendars-regional` | Planned — missing everything but the name: the sources read give only *may* and *hame* for the two counts (Wikipedia, "Mesoamerican calendars", citing Edmonson in *Arqueología Mexicana*, not read), and no month list or anchor from Lowe, Lee or Campbell and Kaufman was found in a readable form. See [systems/mesoamerican-years.md](systems/mesoamerican-years.md) |
| Mixtec year bearer | `mixtec-year` | `hc-calendars-regional` | Planned — the rule is sourced: the Mixtec year ended 40 days before the Aztec, so the same year carries a number one lower, 1519 being Aztec 1 Reed and Mixtec 13 Reed (Milbrath, *Trace* 81, 2022, citing Jiménez Moreno 1940 and Caso 1967, neither read). Missing: any list of Mixtec month names, and the day of the year the bearer falls on — Caso puts the Aztec bearer on the 360th day, Milbrath on the first of Izcalli, the 341st, and where the Mixtec year begins depends on which is followed. Reading a Mixtec codex through `aztec-xiuhpohualli` gives a silently wrong year name, which is the reason for a row of its own. See [systems/mesoamerican-years.md](systems/mesoamerican-years.md) |
| Living 260-day counts — Kʼicheʼ, Kaqchikel and Ixil *Cholqʼij*, Mixe *xëë tun* | `cholqij` | `hc-calendars-regional` | Researching — a mod-260 count kept unbroken through five centuries where the 365-day frame around it was not, so `maya-tzolkin` is a living calendar as well as an archaeological one; the modern anchor differs slightly between communities, and the anchor is the whole question. Tedlock, *Maya Daykeeping*; Ríos Cortés (Zenodo 5541316) |
| Muisca (*zocam*) | `muisca` | `hc-calendars-regional` | Researching — reconstruction, sources conflict: three concurrent year kinds on a vigesimal count, a rural year of twelve or thirteen lunations with a "deaf month", a common year of twenty months of thirty days, 600 days, and a priestly year of thirty-seven months, from Duquesne's account of 1795, long dismissed as invention and partly rehabilitated. The chroniclers disagree even on the week, and the two published versions of Duquesne differ (Izquierdo Peña, arXiv:0812.0574). A 600-day year is the kind of claim a library gets asked about, which is why the row exists |
| Mapuche *We Tripantu* and Aymara *Willkakuti* | — | `hc-calendars-regional` | Researching — thirteen *küyen* of 28 days, reset by observation at the June solstice and the Pleiades' heliacal reappearance, Willkakuti's boundary the solstice sunrise seen at Tiwanaku's Gate of the Sun and a Bolivian holiday since 2009; the year counts attached to both are modern reconstructions, and the sources are thin |
| Borana (Oromo) | `borana` | `hc-calendars-regional` | Researching — lunar-stellar: twelve months fixed by the conjunction of a named moon phase with one of seven stars, no week, and twenty-seven named days cycling through a 29- or 30-day month so the first names recur at its end, a day-name cycle shorter than the month, which nothing here has. It needs a star-conjunction ephemeris and carries a day or two of observational slack. Legesse, *Gada* (1973); Doyle, "The Borana Calendar Reinterpreted", *Current Anthropology* (1986); Ruggles' archaeoastronomical critique; and a great deal of unreliable repetition on the web |
| Swahili nautical year (*Nairuzi*, Mwaka Kogwa) | `swahili` | `hc-calendars-regional` | Researching — a 365-day solar year kept beside the Hijri year on the Swahili coast, whose new year has drifted to late July, which is the evidence for a non-leap year of the Persian type; the rule is inferred from the drift, not attested (*Encyclopaedia Iranica*, "East Africa") |
| East Polynesian lunar calendars — Hawaiian *Kaulana Mahina*, Marquesan, Tahitian, Tuamotuan | `hawaiian` and the rest as name tables | `hc-calendars-regional` | Researching — about thirty named nights against a 29.53-day month, so a name is periodically dropped; the Hawaiian nights in three ten-night *anahulu*, the month from the first crescent visible after sunset, a thirteenth month to hold the *makahiki* to the sidereal year. One mechanism and many name tables, which is the split [policy.md](policy.md) §2 asks for; which night is dropped is observational. West Polynesia numbers its nights instead of naming them, a different data model for the same sky. Valério, Tamburini and Corazza, "Computational analysis reveals historical trajectory of East Polynesian lunar calendars", *PLoS ONE* 21(7); Langlas, "Nā Pō o ka Malama" |
| Rapa Nui | `rapa-nui` | `hc-calendars-regional` | Researching — twelve or thirteen months of named nights with one or two intercalary nights, *hotu* and *hiro*, intercalation at the night rather than the month; Thomson recorded thirteen months and others twelve, and the *Mamari* rongorongo tablet thought to encode it is undeciphered |
| Carolinian sidereal months | — | `hc-calendars-regional` | Researching — twelve or thirteen "moons" that are independent of the moon, each named for a star and of unequal length, tied to the star compass of Satawal, Puluwat, Woleai and Ulithi (Gladwin, *East Is a Big Bird*, 1970); a non-lunar "lunar" calendar with unequal months is a shape nothing here can hold. The heliacal risings compute; the month-length conventions are island-specific and largely unpublished |
| Lao lunisolar | `lao` | `hc-calendars-regional` | Planned — the same *suryayatra* arithmetic as `khmer` by Gislén and Eade, "The Calendars of Southeast Asia. 2", *JAHH* 22(3), 2019, who treat Thailand, Laos and Cambodia as one system. Missing: the rule Laos uses when the leap month and the leap day fall in one year (Cambodia moves the day to the next year; Faraut's weekday rule, which Gislén and Eade set out, also shifts whole lunar years by a day), for which Gislén and Eade name Phetsarath, "Le calendrier lao" (1956), and Dupertuis, "Le calcul du calendrier laotien" (1981), neither read; a published Lao date in a year where the choice shows; and the month names in Lao script. See [systems/khmer-chhankitek.md](systems/khmer-chhankitek.md) |
| Sinhalese lunar (the Poya days) | `sinhalese-lunar` | `hc-calendars-regional` | Planned — Sri Lanka's Poya days are fixed each year by the Holidays Act orders, and no source read gives a rule that reproduces them; `hc-holiday`'s Sri Lanka table already tabulates the orders for 2023–2027, *adhi* Poya days included. Missing: a sourced rule, or the published almanac's own method |
| Tai, Shan and Dai lunisolar | `shan` | `hc-calendars-regional` | Planned — the Tai calendars number their months. Gislén and Eade's Table 5 gives Caitra as month 5 in the central Thai numbering, 6 at Keng Tung and 7 at Chiang Mai, one and two ahead of the Thai count rather than the Burmese. Missing: a Shan or Dai numbering from a source, and which year rule, the Burmese or the Thai, the Tai calendars follow; Eade, *The Calendrical Systems of Mainland South-East Asia* (Brill, 1995), which treats the family, was not read |
| Cham *Sakawi* and Sundanese *Kala Sunda* | — | `hc-calendars-regional` | Researching — the Cham keep two calendars for one people, the Ahier lunisolar and the Awal of Islamic months on an eight-year intercalation cycle, reconciled by priestly decision; the Sundanese keep a lunar and a solar system side by side, with no English account to carry them from |
| Turkic twelve-year animal cycle | — | `hc-calendar::cycle` | Researching — a bare twelve-year naming cycle without the sixty-year stem product, riding on whatever local reckoning applied, from the Old Turkic khaganates to twentieth-century Azerbaijan, and attached to different year boundaries, Nowruz in Azerbaijani use; the animal substitutions are regionally diagnostic, the dragon a fish or a crocodile (Kāshgarī, *Dīwān Lughāt al-Turk*). A labelling cycle beside `sexagenary`, not a calendar; the anchoring is the open question |
| Balinese Pawukon (thirty *wuku*, ten concurrent weeks) | `balinese-pawukon` | `hc-calendars-regional` | Done — Reingold and Dershowitz's arithmetic; see [systems/pawukon-and-pasaran.md](systems/pawukon-and-pasaran.md) |
| Javanese Pasaran (five-day market week) | `javanese-pasaran` | `hc-calendars-regional` | Done — the pasaran, the 35-day wetonan and the neptu; see [systems/pawukon-and-pasaran.md](systems/pawukon-and-pasaran.md) |
| Javanese Pranata Mangsa (twelve agricultural seasons) | `pranata-mangsa` | `hc-seasons` | Researching — the two tables read disagree by a day throughout, and neither adds up. The Indonesian Wikipedia starts Kasa on 22 June; the English one starts Kaso on 23 June. The Indonesian table leaves 22 December in no season, between Kanem's end on the 21st and Kapitu's start on the 23rd. It also gives Kawolu 26 days in a common year, while its dates, 4 to 28 February, span 25. The English table starts Kanem on 11 November, where Kalima's 27 days end on the 9th. Neither names the text it follows, so there is no source to carry the table from |
| The Sun's nakṣatra (Kerala's ñāṭṭuvēla, the Deccan's rain nakṣatras) | — | `hc-calendars-indic` | Done — `nakshatra::solar_nakshatra_at` and its ingress and span functions, eight to ten minutes from Drik Panchang's 2025 transits; the periods' regional names and lore are not carried; see [systems/hindu-calendars.md](systems/hindu-calendars.md) |
| Igbo four-day week (Izu) | `igbo` | `hc-calendars-regional` | Researching — the source says the calendar is "neither universal nor synchronized" between communities and dates no market day, so there is no anchor to carry |
| Yoruba four-day week | `yoruba` | `hc-calendars-regional` | Researching — regional variants differ |
| Akan Adaduanan (42-day cycle) | `akan` | `hc-calendars-regional` | Done — the six-day and seven-day weeks against each other, anchored on the Fɔdwo of 23 January 1978 the source dates; the four dabɔne named |
| Nepali Bikram Sambat | `bikram-sambat` | `hc-calendars-indic` | Done — the gazetted months of 2080–2083 BS and the *Sūrya Siddhānta* reckoning elsewhere; more gazetted years would replace the reckoning in them; see [systems/nepal-calendars.md](systems/nepal-calendars.md) |
| Nepal Sambat (lunar) | `nepal-sambat` | `hc-calendars-indic` | Done — the amānta months under their Newar names from Kachhalā, read at Kathmandu's sunrise; Lalitpur's solar Nepal Sambat is not carried; see [systems/nepal-calendars.md](systems/nepal-calendars.md) |
| Burmese | `burmese` | `hc-calendars-regional` | Done — Yan Naing Aye's arithmetic of the Myanmar Era with the record's exceptions as data, 1 to 3000 ME, checked against the published holidays; see [systems/burmese.md](systems/burmese.md) |
| Thai lunar | `thai-lunar` | `hc-calendars-regional` | Done — the calendar as Thailand publishes it, its year types carried as data for 2535–2570 BE (1992–2027) and the first six months of 2571 and refused outside; the year types, how they were read off the published holy days, and why Eade's *suriyayatra* rule is not used are in [systems/thai-lunar.md](systems/thai-lunar.md) |
| Khmer (*Chhankitek*) | `khmer` | `hc-calendars-regional` | Done — the leap month and the leap day by the *suryayatra* rule as Phylypo Tum gives it from Roath Kim Soeun, over the layout it shares with `thai-lunar` in `southeast_asian`, 1900–2200, reproducing the lunar dates of the New Years of 2022–2026, the sub-decreed holidays of 2024–2027 and Tum's checked dates from 1913; see [systems/khmer-chhankitek.md](systems/khmer-chhankitek.md) |
| Attic (Athenian) | `attic` | `hc-calendars-regional` | Researching — reconstruction, sources conflict |
| Babylonian (Seleucid era) | `babylonian` | `hc-calendars-lunar` | Done — the nineteen-year rule and the moonlag criterion at Babylon as *Calendrical Calculations* states them, SE −71 to 386 (383 BCE to 76 CE), where Parker and Dubberstein's table follows the rule without exception; their first days of the month are matched for 82.9% of months and missed by a day for the rest. Years are the Seleucid count, continued backwards before SE 1; the regnal labels the earlier tablets carry are not, and the centuries before 383 BCE, intercalated by decree, would need their table rather than a rule |
| Ancient Roman pre-Julian | `roman-republican` | `hc-calendars-regional` | Researching — intercalation was discretionary |
| Celtic Coligny | `coligny` | `hc-calendars-regional` | Researching — reconstruction |
| Seleucid era with the Macedonian months | `seleucid` | `hc-calendars-regional` | Researching — Macedonian month names on the Babylonian lunisolar mechanism, and the first continuously numbered era; the same Seleucid year begins in autumn by the court reckoning and in spring by the Babylonian, so one year number is two calendars. Parker and Dubberstein, *Babylonian Chronology 626 B.C.–A.D. 75*, give the tables; the row rides on `babylonian` |
| Shang oracle-bone reckoning | — | `hc-calendars-regional` | Researching — days counted in the sexagenary cycle, the ancestor of `sexagenary`, continuously and reliably; the year intercalated by appending a thirteenth month, and in attested cases a fourteenth and a fifteenth, with its own number rather than a repeat of another's, which `Month.leap` does not describe. The day cycle is sound and the year structure is contested |
| Pentecontad | — | `hc-calendars-regional` | Researching — seven periods of fifty days, seven weeks and an *atzeret* each, 350 days with a fifteen- or sixteen-day supplement, identified by Julius and Hildegard Lewy in the 1940s; later work on the Old Assyrian and Babylonian material finds no evidence for the Amorite origin, so reconstruction, sources conflict |
| Inca | `inca` | `hc-calendars-regional` | Researching — two incompatible reconstructions rather than none: a state lunisolar year of twelve thirty-day months and five days, and Zuidema's reading of the *ceque* system, 328 *huacas* on 41 radial lines as a 328-day sidereal-lunar year, twelve months of 27⅓ days, plus 37 days of the Pleiades' invisibility, a calendar laid out in space (Zuidema in Ruggles, ed., *Handbook of Archaeoastronomy and Ethnoastronomy*, 2015). The literature is substantial and disagrees with itself |

The sexagenary cycle's pillars, its three year boundaries and its readings
are written up in [systems/sexagenary-cycle.md](systems/sexagenary-cycle.md).

## Stage 5 — Seasonal subdivisions

Not calendars in their own right, but named subdivisions layered onto one.

| System | Crate | Status |
| --- | --- | --- |
| 二十四節気 — the 24 solar terms | `hc-seasons` | Done — 定気 at 15° of apparent longitude, read at a named meridian; the rules, the 暦要項 comparison and the sources are in [systems/solar-terms-and-pentads.md](systems/solar-terms-and-pentads.md) |
| 七十二候 — the 72 pentads (Chinese and Japanese sets; a further set is one entry) | `hc-seasons` | Done — the 時訓解 list and the 1874 略本暦 list at 5° each; see [systems/solar-terms-and-pentads.md](systems/solar-terms-and-pentads.md) |
| 雑節 — zassetsu (節分, 彼岸, 社日, 八十八夜, 入梅, 半夏生, 土用, 二百十日, 二百二十日) | `hc-seasons` | Done |
| 三伏 and 數九 — the Chinese dog days and the nine nines | `hc-seasons` | Done — 入伏 the third 庚 day from the summer solstice, 中伏 the fourth, 末伏 the first from 立秋, the solstice counted when it is itself a 庚 day, as the published dates of 2017–2030 require; the nines from the winter solstice, as the Hong Kong Observatory counts them |
| Moon phases as a calendar layer | `hc-seasons` | Done |
| 六曜 — rokuyō (先勝, 友引, 先負, 仏滅, 大安, 赤口) | `hc-seasons` | Done |
| 十二直 and 二十八宿 (and 二十七宿) | `hc-almanac` | Done |
| Computus cycles — golden number, dominical letter, epact, solar cycle, indiction, Julian Period | `hc-calendars-solar::cycles` | Done |
| Medieval year-start styles — Lady Day, Annunciation (Florentine and Pisan), Nativity, *more veneto*, Greek | `hc-calendars-solar::year_style` | Done |
| Roman day notation — kalends, nones, ides, *pridie*, the doubled bissextile day | `hc-format::roman` | Done |
| 九星, 七曜, 暦注下段, 選日 | `hc-almanac` | Done |
| 黄道十二宮 — Western zodiac signs (tropical), with periods | `hc-seasons` | Done — the twelve 中気 as sign boundaries; see [systems/solar-terms-and-pentads.md](systems/solar-terms-and-pentads.md) |
| Sidereal signs / rāśi, with the Lahiri and other ayanamsas | `hc-seasons` | Done — the tropical longitude less an ayanāṃśa carried by IAU 2006 precession from a Swiss Ephemeris anchor; see [systems/solar-terms-and-pentads.md](systems/solar-terms-and-pentads.md) |
| Indian solar months over the rāśi — Sanskrit, Tamil, Bengali, Malayalam, and any tradition added as an entry | `hc-seasons` | Done — month names over the sidereal boundaries; see [systems/solar-terms-and-pentads.md](systems/solar-terms-and-pentads.md) |
| 十二次 — the Chinese twelvefold ecliptic division | `hc-seasons` | Done — 星紀 from 大雪, each 次 opening at a 節気; see [systems/solar-terms-and-pentads.md](systems/solar-terms-and-pentads.md) |
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
