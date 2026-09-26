# System documents

One document per system that a maintainer cannot be expected to know already:
what it is in the world, how it works, what this library carries of it and how
well, and where every statement comes from. The rule is
[policy.md §12](../policy.md); the sources they cite are in
[`references.bib`](../references.bib).

Each document has the same sections, so that a reader knows where to look:

| Section | What it holds |
| --- | --- |
| **What it is** | The system as the world keeps it: who used it, when, for what |
| **How it works** | The rules, from the sources, with a worked example that can be followed by hand |
| **What is carried** | The identifiers this library registers for it, their range, what is computed and what is tabulated, what is deliberately left out and why |
| **Accuracy** | How the implementation was checked against the published reference, the measured agreement, and the known disagreements |
| **Sources** | Every source, keyed to `references.bib`, with what each was used for and whether it was read directly |
| **Code** | The module that implements it and the tests that anchor it |

The document explains; the module documentation summarises it in a paragraph
and names it; the `sources` strings and test comments cite the same sources.
None of the three repeats another.

## Documents

| System | Document | Backs |
| --- | --- | --- |
| Babylonian calendar of the Seleucid era | [babylonian.md](babylonian.md) | `babylonian` |
| Name-day lists | [name-days.md](name-days.md) | `hc-name-days` |
| The Japanese lunisolar calendars, Senmyō to Tenpō | [japanese-lunisolar.md](japanese-lunisolar.md) | `japanese-senmyo`, `japanese-jokyo`, `japanese-horyaku`, `japanese-kansei`, `japanese-tenpo` |
| The Thai lunar calendar's year types, and the Buddhist year as printed | [thai-lunar.md](thai-lunar.md) | `thai-lunar`; `buddhist::printed_year` |
| The Khmer *Chhankitek*: the *suryayatra* rule for the leap month and the leap day as Cambodia applies it, the Buddhist Era changing at Pisakh, and why the Lao, Sinhalese and Tai calendars are not carried | [khmer-chhankitek.md](khmer-chhankitek.md) | `khmer`; `southeast_asian` |
| China's annual holiday arrangements and working weekends | [china-holiday-arrangements.md](china-holiday-arrangements.md) | `hc-holiday`'s `CHINA`; `XSHG`, `XSHE` |
| The Hijri calendars: the tabular schemes, the Umm al-Qura table and the observational prediction | [hijri.md](hijri.md) | `islamic-civil`, `islamic-tbla`, `islamic-fatimid`, `islamic-umalqura`, `islamic-rgsa`; `tabular` |
| The Hindu calendars: amānta and pūrṇimānta months, the solar months, the nakṣatras, ayanāṃśa, the sixty year names | [hindu-calendars.md](hindu-calendars.md) | `hindu-lunar`, `hindu-lunar-purnimanta`, `hindu-solar-tamil`, `hindu-solar-malayalam`, `hindu-solar-bengali`, `hindu-solar-vikrami`, `hindu-old-solar`, `hindu-old-lunar`; `tithi`, `nakshatra`, `surya_siddhanta`, `samvatsara` |
| The Maya and Aztec counts: the Long Count under two correlations, the Tzolkʼin, Haabʼ and Calendar Round, the 819-day count's stations and colour-directions over twenty stations, the tonalpohualli and xiuhpohualli | [mesoamerican-counts.md](mesoamerican-counts.md) | `maya-longcount`, `maya-longcount-gmt2`, `maya-tzolkin`, `maya-haab`, `maya-round`, `maya-tzolkin-gmt2`, `maya-haab-gmt2`, `maya-round-gmt2`, `maya-819`, `maya-819-gmt2`, `aztec-tonalpohualli`, `aztec-xiuhpohualli` |
| The Zapotec *yza* of Villa Alta: the months of Manuscript 85, the years named by their first day, the correlation of 1695; and why the Purépecha, Zoque and Mixtec years are not carried | [mesoamerican-years.md](mesoamerican-years.md) | `zapotec-yza` |
| The East Asian lunisolar calendars: China, Korea and Vietnam on their meridians | [east-asian-lunisolar.md](east-asian-lunisolar.md) | `chinese`, `dangi`, `vietnamese`; `lunisolar` |
| South Korea's public holidays and the substitute holiday, with the collision rule | [korea-holidays.md](korea-holidays.md) | `hc-holiday`'s `SOUTH_KOREA` and `engine::collisions`; `XKRX` |
| Russia's public holidays and the transfers of days off | [russia-transfers.md](russia-transfers.md) | `hc-holiday`'s `RUSSIA`; `MISX` |
| Afghanistan's holidays under the Islamic Emirate: the official lunar calendar, the Ministry of Labour's notices, the solar days on `persian-afghan` | [afghanistan-holidays.md](afghanistan-holidays.md) | `hc-holiday`'s `AFGHANISTAN`; `CalendarSystem::SOLAR_HIJRI_AFGHAN` |
| Japan's holiday law and its amendments | [japan-holidays.md](japan-holidays.md) | `hc-holiday`'s `JAPAN`; `XJPX` |
| The 24 solar terms and 72 pentads, and the zodiac conventions: 定気, the meridian, the ayanāṃśa | [solar-terms-and-pentads.md](solar-terms-and-pentads.md) | `hc-seasons`: `solar_terms`, `pentads`, `meridian`, `zodiac` |
| The Berber agrarian calendar and the Amazigh era | [berber.md](berber.md) | `berber` |
| The Mandaean calendar: the Parwanaia and the years after Adam | [mandaean.md](mandaean.md) | `mandaean` |
| The modern Assyrian calendar and its 4750 BC epoch | [assyrian.md](assyrian.md) | `assyrian` |
| The Yazidi year: Serêsal and the Eastern calendar | [yazidi.md](yazidi.md) | `yazidi` |
| The Hanke–Henry Permanent Calendar: the ISO year cut into months, and Xtr | [hanke-henry.md](hanke-henry.md) | `hanke-henry` |
| The Old Icelandic calendar: the misseri, the sumarauki and the Julian and Gregorian rules | [icelandic.md](icelandic.md) | `icelandic`, `icelandic-julian` |
| The Qumran and Jubilees 364-day year and the mishmarot, on an epoch of this library's | [qumran.md](qumran.md) | `qumran` |
| The Soviet revolutionary weeks of 1929–1940, from the decrees | [soviet-week.md](soviet-week.md) | `soviet-week` |
| The Meyer–Palmen Solilunar Calendar: two remainders over a 6840-year era, and Meton | [meyer-palmen.md](meyer-palmen.md) | `meyer-palmen` |
| Palmen's Yerm lunar calendar: yerms of 17 and 15 months, 52 to a cycle, the night from noon | [yerm.md](yerm.md) | `yerm` |
| Nepal's calendars: the Bikram Sambat as gazetted, and Nepal Sambat | [nepal-calendars.md](nepal-calendars.md) | `bikram-sambat`, `nepal-sambat` |
| The Hebrew calendar: the molad, the nineteen-year cycle and the four dehiyyot | [hebrew.md](hebrew.md) | `hebrew` |
| The Samaritan calendar: the conjunction at Mount Gerizim, the first month after Julian 11 March, the Entry Era changing at the sixth month | [samaritan.md](samaritan.md) | `samaritan` |
| The Odia Anka: the Gajapati's regnal years from Suniā, the numbers they drop, the reign of Dibyasingha Deb | [odia-anka.md](odia-anka.md) | `odia-anka` |
| The Vira Nirvana Samvat: the Jain era of 527 BCE on the amānta months, from Kārtika śukla 1, and why there is one era and not two | [vira-nirvana-samvat.md](vira-nirvana-samvat.md) | `vira-nirvana-samvat` |
| The Gregorian reform, country by country, and the Swedish exception | [gregorian-reform.md](gregorian-reform.md) | `julian-gregorian-<polity>`, fourteen of them; `swedish-1700`; the adoption table by country, `adoption` |
| The Burmese calendar: the eras of the Myanmar Era, watat and yat-ngyin, the record's exceptions as data | [burmese.md](burmese.md) | `burmese`; `hc-holiday`'s Thingyan days |
| The Tibetan calendar: the Phugpa arithmetic, the lunar day with its skipped and extra days, the leap-month rule, the sixty-year names | [tibetan-phugpa.md](tibetan-phugpa.md) | `tibetan` |
| The Tibetan calendar's other versions: the Tsurphu, the Bhutanese with its leap month after the month it repeats, the Mongolian New Genden and Tsagaan Sar; why the Kālacakra *karaṇa* and the yellow calculation are not carried | [tibetan-variants.md](tibetan-variants.md) | `tibetan-tsurphu`, `tibetan-bhutan`, `mongolian` |
| Holidays on the Tibetan calendar: Mongolia's lunar days of its holidays law and Bhutan's Bhutanese-dated list days, the month a holiday is kept in, and the skipped and repeated days reported as gaps | [tibetan-calendar-holidays.md](tibetan-calendar-holidays.md) | `hc-holiday`'s `MONGOLIA` and `BHUTAN`, `Rule::TibetanDay`; `CalendarSystem::MONGOLIAN`, `CalendarSystem::TIBETAN_BHUTAN` |
| The Milankovitch orbital elements and daily insolation, from Berger's 1978 series | [orbital-elements.md](orbital-elements.md) | `hc-orbital` |
| The Javanese calendar: Sultan Agung's lunar year, the windu and the kurup, and the Yogyakarta and Aboge reckonings | [javanese.md](javanese.md) | `javanese`, `javanese-yogyakarta`, `javanese-aboge` |
| The Balinese Pawukon's ten concurrent weeks, and the Javanese pasaran and wetonan | [pawukon-and-pasaran.md](pawukon-and-pasaran.md) | `balinese-pawukon`, `javanese-pasaran` |
| The Badíʿ calendar from 2015 and the French Republican decree, against their arithmetic siblings | [equinox-calendars.md](equinox-calendars.md) | `bahai-astronomical`, `french-republican-equinox`; `bahai`, `bahai-arithmetic`, `french-republican-arithmetic` |
| The sexagenary cycle for year, month, day and hour, its three year boundaries and its readings | [sexagenary-cycle.md](sexagenary-cycle.md) | `sexagenary`; `hc-calendar::cycle`, `cycle::readings` |
| The Swedish runestaff: the day-letter runes, and the golden-number runes on the new moons of the Julian ecclesiastical lunar calendar | [runic-calendar.md](runic-calendar.md) | `hc-calendars-solar::cycles::runic` |

## Systems that need a document

These systems are implemented and need a document they do not yet have. A
system not yet coded is written up from its sources before it is coded, with
the sections above and a row in the table above; one found to need a document
after it is coded is listed here until it has one
([policy.md §12](../policy.md)).

| System | Why it needs a document | Backs |
| --- | --- | --- |
| The Japanese era names: the unified stream, the two courts of 1331–1392, and eras read from the start of the year or from the day proclaimed | A court split, eras backdated to the start of the year against the reckoning as proclaimed, and the gaps of 655–686 and 687–701 | `japanese`, `japanese-northern`, `japanese-southern`, `japanese-proclaimed`; `hc-calendars-regional::nengo` |
| The Chinese and Korean regnal eras | Eras from the year after accession (踰年改元), restored and withdrawn eras, and regimes proclaiming at the same time | `chinese-regnal`, `korean-regnal` |
| The Solar Hijri calendar in Iran and Afghanistan | The new year by the equinox against noon at a named place, the laws of 1925, 1957 and 2022, and competing arithmetic schemes | `persian`, `persian-afghan`, `persian-arithmetic` |
| The Zoroastrian calendars | The intercalations, the split of 1745, the Fasli reform, and the era's epoch | `zoroastrian-qadimi`, `zoroastrian-shahanshahi`, `zoroastrian-fasli` |
| The Armenian calendar | The wandering year, and Sarkawag's fixed year and its extension backwards | `armenian`, `armenian-fixed` |
| The Bangladeshi calendar | Decrees that change the rule inside one identifier | `bangladeshi` |
| The Nanakshahi calendar | Versions that change the rule inside one identifier | `nanakshahi` |
| The Japanese almanac notes, 暦注 and 選日 | Publishers' conventions, the reversal of the nine stars, the leap month, and the anchor of the twelve directs | `hc-almanac` |
| The 雑節 and 六曜, with the lunisolar months they rest on | Days defined from the solar terms and from a minimal lunisolar calendar | `hc-seasons`: `zassetsu`, `rokuyo`, `lunisolar` |
| Mars timekeeping | The sol, Coordinated Mars Time, local mean and true solar time, the Mars year, mission sol counts and the Darian calendar | `hc-planetary`: `mars`, `mars::missions`, `mars::darian` |
| Taiwan's, Vietnam's and Hong Kong's holiday regimes | Annual notices, make-up working days and swapped Saturdays, and Hong Kong's collision and eve rules | `hc-holiday`'s `TAIWAN`, `VIETNAM`, `HONG_KONG` |
