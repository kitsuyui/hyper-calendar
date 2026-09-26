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
| The Hindu calendars: amānta and pūrṇimānta months, the solar months, the nakṣatras, ayanāṃśa, the sixty year names in the south and the Bārhaspatya cycle of the north with its expunged names | [hindu-calendars.md](hindu-calendars.md) | `hindu-lunar`, `hindu-lunar-purnimanta`, `hindu-solar-tamil`, `hindu-solar-malayalam`, `hindu-solar-bengali`, `hindu-solar-vikrami`, `hindu-old-solar`, `hindu-old-lunar`; `tithi`, `nakshatra`, `surya_siddhanta`, `samvatsara`, `barhaspatya` |
| The Maya and Aztec counts: the Long Count under three correlations, the Tzolkʼin, Haabʼ and Calendar Round, the 819-day count's stations and colour-directions over twenty stations, the tonalpohualli and xiuhpohualli | [mesoamerican-counts.md](mesoamerican-counts.md) | `maya-longcount`, `maya-longcount-gmt2`, `maya-longcount-584286`, `maya-tzolkin`, `maya-haab`, `maya-round`, `maya-tzolkin-gmt2`, `maya-haab-gmt2`, `maya-round-gmt2`, `maya-tzolkin-584286`, `maya-haab-584286`, `maya-round-584286`, `maya-819`, `maya-819-gmt2`, `maya-819-584286`, `aztec-tonalpohualli`, `aztec-xiuhpohualli` |
| The Zapotec *yza* of Villa Alta: the months of Manuscript 85, the years named by their first day, the correlation of 1695; and why the Purépecha, Zoque and Mixtec years are not carried | [mesoamerican-years.md](mesoamerican-years.md) | `zapotec-yza` |
| The East Asian lunisolar calendars: China, Korea and Vietnam on their meridians | [east-asian-lunisolar.md](east-asian-lunisolar.md) | `chinese`, `dangi`, `vietnamese`; `lunisolar` |
| The Japanese era system: the backdated and the proclaimed reckonings, the two courts of 1331–1392 and the reunion, the gaps of 655–686 and 687–701 | [japanese-eras.md](japanese-eras.md) | `japanese`, `japanese-northern`, `japanese-southern`, `japanese-proclaimed`, `japanese-northern-proclaimed`, `japanese-southern-proclaimed`; `nengo` |
| The Chinese and Korean regnal eras: 踰年改元 and its exceptions, restored and withdrawn eras, the concurrent regimes of 1644–1683, the Korean Empire's three eras | [east-asian-eras.md](east-asian-eras.md) | `chinese-regnal`, `korean-regnal` |
| South Korea's public holidays and the substitute holiday, with the collision rule | [korea-holidays.md](korea-holidays.md) | `hc-holiday`'s `SOUTH_KOREA` and `engine::collisions`; `XKRX` |
| Russia's public holidays and the transfers of days off | [russia-transfers.md](russia-transfers.md) | `hc-holiday`'s `RUSSIA`; `MISX` |
| Taiwan's holidays: the 條例 of 2025, the make-up day before a Saturday and after a Sunday, and the swapped Saturdays of the calendars to 2025 | [taiwan-holidays.md](taiwan-holidays.md) | `hc-holiday`'s `TAIWAN` |
| Vietnam's holidays: the Labour Code's list, the make-up day, and each year's notices of Tết, National Day and the swapped Saturdays | [vietnam-holidays.md](vietnam-holidays.md) | `hc-holiday`'s `VIETNAM` |
| Hong Kong's general and statutory holidays: the Sunday and coincidence rules, the eve rule of 1983 to 2011, and the phasing of 2021 | [hong-kong-holidays.md](hong-kong-holidays.md) | `hc-holiday`'s `HONG_KONG`; `XHKG` |
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
| The Solar Hijri calendar: Nowruz by the clock's noon and by the Sun's at Tehran, the 33-year rule and the 2 820-year cycle | [solar-hijri.md](solar-hijri.md) | `persian`, `persian-apparent-noon`, `persian-afghan`, `persian-arithmetic`, `persian-arithmetic-33` |
| The Zoroastrian calendars: the Parsi intercalation of the 1120s, the split of 1745, the Fasli of 1906, the ZRE | [zoroastrian.md](zoroastrian.md) | `zoroastrian-qadimi`, `zoroastrian-shahanshahi`, `zoroastrian-fasli` |
| The Armenian calendar: the Great Era's wandering year and Sarkawag's fixed year of 1084 | [armenian.md](armenian.md) | `armenian`, `armenian-fixed` |
| The Bangladeshi and Nanakshahi calendars: fixed month lengths by decree, and their revisions | [fixed-solar-namings.md](fixed-solar-namings.md) | `bangladeshi`, `nanakshahi` |
| The sexagenary cycle for year, month, day and hour, its three year boundaries and its readings | [sexagenary-cycle.md](sexagenary-cycle.md) | `sexagenary`; `hc-calendar::cycle`, `cycle::readings` |
| The Swedish runestaff: the day-letter runes, and the golden-number runes on the new moons of the Julian ecclesiastical lunar calendar | [runic-calendar.md](runic-calendar.md) | `hc-calendars-solar::cycles::runic` |
| The Japanese almanac notes: 暦注下段, 選日, 十二直, 二十八宿, 九星, 六曜 and 七曜 | [japanese-almanac-notes.md](japanese-almanac-notes.md) | `hc-almanac`: `lower_register`, `selected_days`, `twelve_directs`, `mansions`, `nine_stars`, `rokuyo`, `seven_luminaries` |
| The 雑節 and 六曜, and the minimal lunisolar derivation they rest on: the 社日 tie rules, the 旧暦2033年問題 | [zassetsu-and-rokuyo.md](zassetsu-and-rokuyo.md) | `hc-seasons`: `zassetsu`, `rokuyo`, `lunisolar` |
| Spreadsheet serial dates: the Excel 1900 system and its phantom 29 February 1900, the Excel 1904 system, the OLE Automation date and its negative fractions | [spreadsheet-dates.md](spreadsheet-dates.md) | `excel-1900`, `excel-1904`, `ole-automation-date`; `spreadsheet` |
| The GNSS system times: GPS, Galileo, BeiDou and NavIC time and their epochs, the broadcast week numbers and their rollovers, GLONASS time with its leap seconds and four-year intervals | [gnss-time.md](gnss-time.md) | `hc-core`: `scale`, `epoch`, `gnss` |
| Local mean and local apparent (sundial) time on the Earth, temporal and Italian hours, and the ʿaṣr and Jewish evening times as named conventions | [hours-of-the-day.md](hours-of-the-day.md) | `hc-astro::solar_time` |
| Mars timekeeping: the Mars Sol Date, Coordinated Mars Time, local mean and true solar time, the Mars year from 1955, the mission sol conventions, and the Darian calendar | [mars-timekeeping.md](mars-timekeeping.md) | `hc-planetary`: `mars`, `mars::missions`, `mars::darian` |

## Systems that need a document

Every implemented system that needs a document has one. The next is
written up from its sources before it is coded, with the sections above and
a row in the table above; one found to need a document after it is coded is
listed here until it has one ([policy.md §12](../policy.md)).
