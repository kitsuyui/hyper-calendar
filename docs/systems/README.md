# System documents

This directory holds one document for each *system* that a maintainer
cannot be expected to know already. A system is a calendar, a holiday
regime, a time code or another reckoning with rules of its own. Each
document explains what the system is in the world, how it works, what this
library carries of it and how well, and where every statement comes from.
Read it before changing the code it backs.

[policy.md §12](../policy.md) states the rule. The sources the documents
cite are in [`references.bib`](../references.bib).

Each document has the same sections, so that a reader knows where to look:

| Section | What it holds |
| --- | --- |
| **What it is** | The system as the world keeps it: who used it, when, for what |
| **How it works** | The rules, from the sources, with a worked example that can be followed by hand |
| **What is carried** | The identifiers this library registers for it, their range, what is computed and what is tabulated, what is deliberately left out and why |
| **Accuracy** | How the implementation was checked against the published reference, the measured agreement, and the known disagreements |
| **Sources** | Every source, keyed to `references.bib`, with what each was used for and whether it was read directly |
| **Code** | The module that implements it and the tests that anchor it |

Three places describe each system, and none of them repeats another:

- the document explains it;
- the module documentation summarises it in a paragraph and names the
  document;
- the `sources` strings and the test comments cite the same sources.

## Documents

The Backs column names what each document backs: calendar identifiers,
crates, modules, functions or holiday tables.

| System | Document | Backs |
| --- | --- | --- |
| Babylonian calendar of the Seleucid era | [babylonian.md](babylonian.md) | `babylonian` |
| Name-day lists, and the movable Orthodox name days | [name-days.md](name-days.md) | `hc-name-days`; `hc-holiday`'s `name-days-greek-movable` and `name-days-bulgarian-movable` |
| The Japanese lunisolar calendars, Senmyō to Tenpō | [japanese-lunisolar.md](japanese-lunisolar.md) | `japanese-senmyo`, `japanese-jokyo`, `japanese-horyaku`, `japanese-kansei`, `japanese-tenpo`; the seasonal 進朔 and the 修正宝暦暦 as parameter sets |
| The Thai lunar calendar's year types, and the Buddhist year as printed | [thai-lunar.md](thai-lunar.md) | `thai-lunar`; `buddhist::printed_year` |
| The Khmer *Chhankitek* and the Lao calendar: the *suryayatra* rule for the leap month and the leap day as Cambodia and Laos apply it, where Dupertuis's statement for Laos leaves it open, the Buddhist Era changing at Pisakh, and why the Sinhalese and Tai calendars are not carried | [khmer-chhankitek.md](khmer-chhankitek.md) | `khmer`; `lao`; `southeast_asian` |
| China's annual holiday arrangements and working weekends, the days for some citizens, and the autonomous regions' festivals | [china-holiday-arrangements.md](china-holiday-arrangements.md) | `hc-holiday`'s `CHINA`, with its groups `women`, `youth`, `children`, `military` and its regions `CN-GX`, `CN-NX`, `CN-XJ`; `XSHG`, `XSHE` |
| The Hijri calendars: the tabular schemes, the Umm al-Qura table and the observational prediction | [hijri.md](hijri.md) | `islamic-civil`, `islamic-tbla`, `islamic-fatimid`, `islamic-umalqura`, `islamic-rgsa`, `islamic-observational-cairo-rd`, `islamic-saudi-rule-rd`; `tabular`; `islamic_observational`'s Yallop items: `VisibilityCriterion::YALLOP`, `QTestCriterion`, `YallopVisibility`, `bruin_best_time`, `arc_of_vision`, `crescent_width_arcminutes`, `yallop_q` |
| Global and regional Hijri calendars: the Unified Hijri Calendar as Muhammadiyah and Diyanet apply it, the FCNA calendar, and the Neo-MABIMS and Odeh criteria | [unified-hijri.md](unified-hijri.md) | `islamic-khgt`, `islamic-istanbul-2016`, `islamic-fcna`; `islamic_global`; `islamic_observational`'s `SunsetCriterion`, `Frame`, `VTestCriterion`, `OdehZone` and the criteria `istanbul-2016`, `khgt`, `mabims-2021-topocentric`, `mabims-2021-geocentric-elongation`, `odeh` |
| The Hindu calendars: amānta and pūrṇimānta months, the solar months, the nakṣatras, the yoga and the karaṇa, ayanāṃśa, the sixty year names in the south and the Bārhaspatya cycle of the north with its expunged names and Jupiter's twelve-year cycle, the amṛta siddhi yoga | [hindu-calendars.md](hindu-calendars.md) | `hindu-lunar`, `hindu-lunar-reingold-dershowitz`, `hindu-lunar-surya-siddhanta`, `hindu-lunar-purnimanta`, `hindu-solar-tamil`, `hindu-solar-reingold-dershowitz`, `hindu-solar-malayalam`, `hindu-solar-bengali`, `hindu-solar-vikrami`, `hindu-solar-surya-siddhanta`, `hindu-old-solar`, `hindu-old-lunar`; `tithi`, `nakshatra`, `panchanga`, `amrita_siddhi`, `surya_siddhanta`, `samvatsara`, `barhaspatya` |
| The Maya and Aztec counts: the Long Count under three correlations, the Tzolkʼin, Haabʼ and Calendar Round, the 819-day count's stations and colour-directions over twenty stations, the tonalpohualli and xiuhpohualli | [mesoamerican-counts.md](mesoamerican-counts.md) | `maya-longcount`, `maya-longcount-gmt2`, `maya-longcount-584286`, `maya-tzolkin`, `maya-haab`, `maya-round`, `maya-tzolkin-gmt2`, `maya-haab-gmt2`, `maya-round-gmt2`, `maya-tzolkin-584286`, `maya-haab-584286`, `maya-round-584286`, `maya-819`, `maya-819-gmt2`, `maya-819-584286`, `aztec-tonalpohualli`, `aztec-xiuhpohualli` |
| The Zapotec *yza* of Villa Alta: the months of Manuscript 85, the years named by their first day, the correlation of 1695; the Mixtec year of Caso's reconstruction; and why the Purépecha and Zoque years are not carried | [mesoamerican-years.md](mesoamerican-years.md) | `zapotec-yza`, `mixtec-year` |
| The East Asian lunisolar calendars: China, Korea and Vietnam on their meridians | [east-asian-lunisolar.md](east-asian-lunisolar.md) | `chinese`, `dangi`, `vietnamese`; `lunisolar`; `dangi-kasi`; `chinese::almanac_solar_term_days`, the four age counts, `chinese::marriage_augury`; the Vietnamese zodiac, `cycle::VIETNAMESE_ZODIAC_ANIMALS` |
| The Japanese era system: the backdated and the proclaimed reckonings, the two courts of 1331–1392 and the reunion, the gaps of 655–686 and 687–701 | [japanese-eras.md](japanese-eras.md) | `japanese`, `japanese-northern`, `japanese-southern`, `japanese-proclaimed`, `japanese-northern-proclaimed`, `japanese-southern-proclaimed`, `japanese-kaigen-toji`; `nengo` |
| The Chinese and Korean regnal eras: 踰年改元 and its exceptions, restored and withdrawn eras, the concurrent regimes of 1644–1683, 宣統 at the court to 1924, 洪憲 and Manchukuo's eras, the Korean Empire's three eras in two readings | [east-asian-eras.md](east-asian-eras.md) | `chinese-regnal`, `chinese-regnal-qing-court`, `hongxian`, `manchukuo`, `korean-regnal`, `korean-regnal-backdated` |
| South Korea's public holidays and the substitute holiday, with the collision rule | [korea-holidays.md](korea-holidays.md) | `hc-holiday`'s `SOUTH_KOREA` and `engine::collisions`; `XKRX` |
| Russia's public holidays and the transfers of days off | [russia-transfers.md](russia-transfers.md) | `hc-holiday`'s `RUSSIA`; `MISX` |
| Taiwan's holidays: the 條例 of 2025, the make-up day before a Saturday and after a Sunday, the swapped Saturdays of the calendars to 2025, and the services' days | [taiwan-holidays.md](taiwan-holidays.md) | `hc-holiday`'s `TAIWAN`, with its groups `police`, `firefighters`, `military`, `coast-guard`, `indigenous-peoples` |
| Nepal's holiday notices: the days for everyone, for a community, a faith or a group of employees, and for a place | [nepal-holidays.md](nepal-holidays.md) | `hc-holiday`'s `NEPAL`, with its groups `newar`, `dura`, `women`, `persons-with-disabilities`, `kirat`, `muslims`, `sikhs` |
| Bangladesh's holiday notifications: the general and executive-order holidays, and the optional holidays given to each faith's group | [bangladesh-holidays.md](bangladesh-holidays.md) | `hc-holiday`'s `BANGLADESH`, with its groups `muslims`, `hindus`, `christians`, `buddhists` and `small-ethnic-groups` |
| Vietnam's holidays: the Labour Code's list, the make-up day, and each year's notices of Tết, National Day and the swapped Saturdays | [vietnam-holidays.md](vietnam-holidays.md) | `hc-holiday`'s `VIETNAM` |
| Hong Kong's general and statutory holidays: the Sunday and coincidence rules, the eve rule of 1983 to 2011, and the phasing of 2021 | [hong-kong-holidays.md](hong-kong-holidays.md) | `hc-holiday`'s `HONG_KONG`; `XHKG` |
| Holidays announced year by year: the thirteen tables whose days a yearly list or a declaration per holiday fixes, the years each carries, its gaps, its weekend and the class of its sources | [announced-holidays.md](announced-holidays.md) | `hc-holiday`'s `FIJI`, `KIRIBATI`, `LIBERIA`, `GAMBIA`, `SUDAN`, `TOGO`, `NIGER`, `GABON`, `SIERRA_LEONE`, `ESWATINI`, `GUINEA_BISSAU`, `SOUTH_SUDAN`, `NORTH_KOREA` |
| Afghanistan's holidays under the Islamic Emirate: the official lunar calendar, the Ministry of Labour's notices, the solar days on `persian-afghan` | [afghanistan-holidays.md](afghanistan-holidays.md) | `hc-holiday`'s `AFGHANISTAN`; `CalendarSystem::SOLAR_HIJRI_AFGHAN` |
| Japan's holiday law and its amendments, the prefectures' days and the designated cities' days | [japan-holidays.md](japan-holidays.md) | `hc-holiday`'s `JAPAN`, with its regions `JP-01` to `JP-47` and cities such as `JP-14-100`; `XJPX` |
| Regional weekends: Malaysia's states that keep Friday, Johor's changes of 2014 and 2025, and the Government of Sharjah's Friday–Sunday weekend, with the replacement days and the gaps | [regional-weekends.md](regional-weekends.md) | `hc-holiday`'s `WeekendPolicy::regions`, `SubstitutionPolicy::regions`, `RuleSet::weekend_in`; `MALAYSIA`, `UNITED_ARAB_EMIRATES`; column 12 of `hc_holiday_tables` |
| New Zealand's provincial anniversary days: the Monday nearest, the exceptions, the provinces that are not regions | [new-zealand-anniversary-days.md](new-zealand-anniversary-days.md) | `hc-holiday`'s `NEW_ZEALAND`, with its regions; `countries::new_zealand` |
| India's state holidays under the Negotiable Instruments Act, from the Reserve Bank's lists of 2019–2026, for the twenty-eight states and union territories with an office of their own but Chandigarh | [india-state-holidays.md](india-state-holidays.md) | `hc-holiday`'s `INDIA`, with its regions; `countries::india` |
| The provincial days of Solomon Islands and Vanuatu | [pacific-provincial-days.md](pacific-provincial-days.md) | `hc-holiday`'s `SOLOMON_ISLANDS` and `VANUATU`, with their regions; `countries::solomon_islands`, `countries::vanuatu` |
| Andorra's national holidays and the parishes' own, fixed by each comú each year | [andorra-holidays.md](andorra-holidays.md) | `hc-holiday`'s `ANDORRA`, with its regions `AD-02` to `AD-08` |
| Bolivia's national holidays and the departments' efemérides | [bolivia-holidays.md](bolivia-holidays.md) | `hc-holiday`'s `BOLIVIA`, with its regions `BO-L`, `BO-N`, `BO-O`, `BO-S` and `BO-T` |
| Spain's national holidays and the autonomous communities' days, from the yearly resolutions | [spain-holidays.md](spain-holidays.md) | `hc-holiday`'s `SPAIN`, with its regions `ES-AN` to `ES-VC` |
| Switzerland's federal day and the cantons' holidays, equal to Sunday or rest days | [switzerland-holidays.md](switzerland-holidays.md) | `hc-holiday`'s `SWITZERLAND`, with its regions `CH-AG` to `CH-ZH` |
| Canada's federal holidays and the provinces' and territories' general holidays | [canada-holidays.md](canada-holidays.md) | `hc-holiday`'s `CANADA`, with its regions `CA-AB` to `CA-YT` |
| Mexico's federal days of rest and the states' own for their public servants | [mexico-holidays.md](mexico-holidays.md) | `hc-holiday`'s `MEXICO`, with its region `MX-JAL` |
| The US states' own holidays beyond the federal list, and what each code makes of them | [us-state-holidays.md](us-state-holidays.md) | `hc-holiday`'s `UNITED_STATES`, with its regions `US-AK` to `US-WV` |
| The 24 solar terms and 72 pentads, and the zodiac conventions: 定気, the meridian, the ayanāṃśa, the decans and the *drekkāṇa* | [solar-terms-and-pentads.md](solar-terms-and-pentads.md) | `hc-seasons`: `solar_terms`, `pentads`, `meridian`, `zodiac` |
| The Berber agrarian calendar and the Amazigh era | [berber.md](berber.md) | `berber` |
| The Mandaean calendar: the Parwanaia and the years after Adam | [mandaean.md](mandaean.md) | `mandaean` |
| The modern Assyrian calendar and its 4750 BC epoch | [assyrian.md](assyrian.md) | `assyrian` |
| The Yazidi year: Serêsal and the Eastern calendar | [yazidi.md](yazidi.md) | `yazidi`; `hc-holiday`'s `yazidi` table |
| The Hanke–Henry Permanent Calendar: the ISO year cut into months, and Xtr | [hanke-henry.md](hanke-henry.md) | `hanke-henry` |
| The Old Icelandic calendar: the misseri, the sumarauki, the Julian and Gregorian rules, the Almanac's leap week, the Friday winter and the medieval day | [icelandic.md](icelandic.md) | `icelandic`, `icelandic-julian`, `icelandic-almanac`, `icelandic-friday`, `icelandic-julian-friday`, `icelandic-medieval` |
| The Qumran and Jubilees 364-day year and the mishmarot, on an epoch of this library's, and its festivals | [qumran.md](qumran.md) | `qumran`; `hc-holiday`'s `qumran-festivals` |
| The Soviet revolutionary weeks of 1929–1940, from the decrees | [soviet-week.md](soviet-week.md) | `soviet-week` |
| The Meyer–Palmen Solilunar Calendar: two remainders over a 6840-year era, and Meton | [meyer-palmen.md](meyer-palmen.md) | `meyer-palmen` |
| Palmen's Yerm lunar calendar: yerms of 17 and 15 months, 52 to a cycle, the night from noon | [yerm.md](yerm.md) | `yerm` |
| Four reforms from Hermetic Systems: Dee's 33-year rule in the Dee–Cecil and Dee correlations, and why Cassidy's is the same calendar; the Hermetic Leap Week's hexades; Palmen's Week and Month; the Tabot calendar | [hermetic-reforms.md](hermetic-reforms.md) | `dee-cecil`, `dee`, `hermetic-leap-week`, `week-and-month`, `tabot`; `leap_week` |
| The Liberalia Triday Calendar: a solar calendar of quarters and a lunar one of months, both of three-day tridays, from 17 March 1904 | [liberalia-triday.md](liberalia-triday.md) | `liberalia-triday-solar`, `liberalia-triday-lunar` |
| The Archetypes Calendar: two remainders over an 1 803-year period, Persephone and the long Sophia, the ten-day tweek | [archetypes.md](archetypes.md) | `archetypes` |
| The Terran Computational Calendar: elapsed time since 0TC over TAI, the minimonth of leap days and leap seconds, year bases; why it is a function pair and not a calendar | [terran-computational.md](terran-computational.md) | `hc-calendars-solar::terran` |
| The Indian eras over the Hindu months and the Faṣlī years: the Kārttikādi Vikrama Saṃvat, the Rājyābhiṣeka Śaka, the Saptarṣi era and its dropped hundreds, the Magi San, the Faṣlī years of Madras and Bombay and the Sūr-san; the Gupta, Valabhī, Kalachuri and Lakṣmaṇa Sena eras over the *Sūrya Siddhānta*'s months, and the eras not carried and why | [indian-eras.md](indian-eras.md) | `vikram-samvat-kartikadi`, `rajyabhisheka-saka`, `saptarshi`, `gupta`, `valabhi`, `kalachuri`, `lakshmana-sena`, `magi-san`, `fasli-madras`, `fasli-bombay`, `sur-san`; `lunar_era`, `year_start`, `fasli` |
| Nepal's calendars: the Bikram Sambat as gazetted, and Nepal Sambat | [nepal-calendars.md](nepal-calendars.md) | `bikram-sambat`, `nepal-sambat`, `nepal-sambat-fortnight` |
| The Hebrew calendar: the molad, the nineteen-year cycle and the four dehiyyot, and the sabbatical year | [hebrew.md](hebrew.md) | `hebrew`; `hebrew::yahrzeit`, `hebrew::birthday`, `hebrew::is_sabbatical_year` |
| Astronomical Easter at the meridian of Jerusalem: the Aleppo proposal of 1997, the full moon on a Sunday, apparent time | [astronomical-easter.md](astronomical-easter.md) | `hc-holiday`'s `Computus::ASTRONOMICAL_JERUSALEM` |
| The Ember and Rogation Days: the Prayer Book's weeks after Lent 1, Pentecost, 14 September and 13 December, the last two read as one week each, *Common Worship*'s traditional weeks and its uncomputable week before an ordination, the Roman Greater and Lesser Litanies of 1960 | [ember-and-rogation-days.md](ember-and-rogation-days.md) | `hc-holiday`'s `ember-bcp1662`, `ember-common-worship`, `rogation-roman-1960` |
| The General Roman Calendar of 1960: the 1962 Missal's calendar, its four classes and commemorations, the movable feasts, and where it differs from the calendar of 1969 | [roman-calendar-1960.md](roman-calendar-1960.md) | `hc-holiday`'s `roman_calendar_1960`, `roman-general-1960` |
| Holiday identifiers: the stable name of a holiday within its table, by which a line of one export is joined to another's | [holiday-ids.md](holiday-ids.md) | `hc-holiday`'s `id` module, `HolidayRule::id`; the identifier column of `hc_holidays_in_year`, `hc_holidays_on`, `hc_holidays_on_in` and `hc_common_worship_on` |
| The *Common Worship* calendar: the Principal Feasts, Principal Holy Days and Festivals, the transfers the Rules require, and the years they leave open; and the Book of Common Prayer's of 1662, its red-letter and black-letter days | [common-worship-calendar.md](common-worship-calendar.md) | `hc-holiday`'s `common_worship`, `common-worship`; `book_of_common_prayer`, `bcp-1662`, `bcp-1662-1871` |
| The liturgical year of the Assyrian Church of the East: nine periods of seven weeks or four, anchored to Easter and to the Cross on 13 September, its Fridays and saints; and the Chaldean and Syro-Malabar forms, with the Cross on 14 September | [church-of-the-east-year.md](church-of-the-east-year.md) | `hc-holiday`'s `church-of-the-east`, `chaldean`, `syro-malabar`, `east_syriac` |
| The Eastern Orthodox fasts: the four seasons and the Meatfast, the weekly fasts, the one-day fasts and the fast-free weeks, on the Julian and the Revised Julian fixed dates, and the Apostles' Fast that vanishes on the second | [orthodox-fasts.md](orthodox-fasts.md) | `hc-holiday`'s `orthodox_fasts`: `orthodox-fasts`, `orthodox-fasts-revised-julian` |
| The fasts of the Armenian, Coptic and Ethiopian churches: each church's seasons, weeks and one-day fasts, its fast-free days and its Wednesdays and Fridays | [oriental-fasts.md](oriental-fasts.md) | `hc-holiday`'s `oriental_fasts`: `armenian-fasts`, `armenian-fasts-jerusalem`, `armenian-fasts-fifty-days`, `coptic-fasts`, `ethiopian-fasts` |
| The lectionary cycles: Years A, B and C, the Roman weekday Years I and II, and the Revised Common Lectionary's Propers | [lectionary-cycles.md](lectionary-cycles.md) | `hc-holiday`'s `lectionary` |
| The observational Hebrew calendar: Reingold and Dershowitz's prediction of the Second Temple months at Haifa | [hebrew-observational.md](hebrew-observational.md) | `hebrew-observational` |
| The Samaritan calendar: the conjunction at Mount Gerizim, the first month after Julian 11 March, the Entry Era changing at the sixth month | [samaritan.md](samaritan.md) | `samaritan` |
| The Odia Anka: the Gajapati's regnal years from Suniā, the numbers they drop, the reign of Dibyasingha Deb | [odia-anka.md](odia-anka.md) | `odia-anka` |
| The Vira Nirvana Samvat: the Jain era of 527 BCE on the amānta months, from Kārtika śukla 1, and why there is one era and not two | [vira-nirvana-samvat.md](vira-nirvana-samvat.md) | `vira-nirvana-samvat` |
| The Gregorian reform, country by country, and the Swedish exception | [gregorian-reform.md](gregorian-reform.md) | `julian-gregorian-<polity>`, fourteen of them; `swedish-1700`; the adoption table by country, `adoption` |
| The Burmese calendar: the eras of the Myanmar Era, watat and yat-ngyin, the record's exceptions as data | [burmese.md](burmese.md) | `burmese`; `hc-holiday`'s Thingyan days |
| The Tibetan calendar: the Phugpa arithmetic, the lunar day with its skipped and extra days, the leap-month rule, the sixty-year names | [tibetan-phugpa.md](tibetan-phugpa.md) | `tibetan` |
| The Tibetan calendar's other versions: the Tsurphu, the Bhutanese with its leap month after the month it repeats, the Mongolian New Genden and Tsagaan Sar; Lochen's anomaly and the *karaṇa* Sun, the conventions of Henning's almanacs; why the Kālacakra *karaṇa* and the yellow calculation are not carried | [tibetan-variants.md](tibetan-variants.md) | `tibetan-tsurphu`, `tibetan-bhutan`, `mongolian`, `tibetan-lochen`, `tibetan-tsurphu-karana`, `tibetan-bhutan-lochen` |
| The Tibetan almanac: the five components — lunar mansion, *yoga*, *karaṇa* — and the true Sun and Moon, the planets and Rāhu, the *rab byung* names and the count from 127 BCE, the Bhutanese weekday and winter solstice, the Mongolian months and colours, and where a festival on a skipped or repeated date falls | [tibetan-almanac.md](tibetan-almanac.md) | `hc-calendars-regional`'s `tibetan_almanac` |
| Holidays on the Tibetan calendar: Mongolia's lunar days of its holidays law, Bhutan's Bhutanese-dated list days, Winter Solstice and Thimphu's festivals, and the Tibetan *düchen*, the month a holiday is kept in, and the skipped and repeated days and months: gaps, Berzin's rule or Henning's almanacs, each a named convention | [tibetan-calendar-holidays.md](tibetan-calendar-holidays.md) | `hc-holiday`'s `MONGOLIA`, `BHUTAN`, `buddhist-tibetan`, `buddhist-tibetan-berzin` and `buddhist-tibetan-henning`, `Rule::TibetanDay` and `TibetanDayRule`; `CalendarSystem::MONGOLIAN`, `CalendarSystem::TIBETAN_BHUTAN` |
| The Milankovitch orbital elements and daily insolation, from Berger's 1978 series | [orbital-elements.md](orbital-elements.md) | `hc-orbital` |
| The Javanese calendar: Sultan Agung's lunar year, the windu and the kurup, and the Yogyakarta and Aboge reckonings | [javanese.md](javanese.md) | `javanese`, `javanese-yogyakarta`, `javanese-aboge` |
| The Balinese Pawukon's ten concurrent weeks and its holy days, and the Javanese pasaran and wetonan | [pawukon-and-pasaran.md](pawukon-and-pasaran.md) | `balinese-pawukon`, `javanese-pasaran`; `hc-holiday`'s `balinese-pawukon-days` |
| The Badíʿ calendar from 2015 and the French Republican decree, against their arithmetic siblings, Romme's and Richards's | [equinox-calendars.md](equinox-calendars.md) | `bahai-astronomical`, `french-republican-equinox`; `bahai`, `bahai-arithmetic`, `french-republican-arithmetic`, `french-republican-arithmetic-richards` |
| The Jalālī calendar: Nowrūz before the Sun's noon at Isfahan, Ṭūsī's table of 295 years with its quinquennia and the rule of 161, the extra days after Esfandārmoḏ or after Bahman at Naṭanz | [jalali.md](jalali.md) | `jalali`, `jalali-natanz`, `jalali-tusi` |
| The Olympiads: the ancient count from 776 BC over the Julian year, and the IOC's Olympiads from 1896 with the Games not celebrated | [olympiads.md](olympiads.md) | `olympiad`; `olympiad::ioc_olympiad` |
| The Seleucid era and its neighbours: the calendar of Antioch, the Seleucid era from 1 October, Antioch's two year starts, the era of Gaza, the Arsacid era at Babylon, and why its Iranian form is not carried | [seleucid-eras.md](seleucid-eras.md) | `seleucid-syrian`, `antioch-caesarean-era`, `antioch-caesarean-era-september`, `gaza-era`, `arsacid-era` |
| Year counts over another calendar's year: the Spanish era and its kingdoms, the Masonic years, ADA, the Capitoline count *ab urbe condita*, the Cheondogyo 포덕 year, the Iranian imperial year, the four epochs of the years of the Yellow Emperor, the Era of Philip, the Bostran era, the Era Fascista | [era-counts.md](era-counts.md) | `spanish-era`, `masonic-anno-lucis`, `masonic-anno-inventionis`, `masonic-anno-depositionis`, `masonic-anno-ordinis`, `ada`, `roman-auc-capitoline`, `cheondogyo-podeok`, `persian-imperial`, `huangdi-era`, `huangdi-era-tongmenghui`, `huangdi-era-liu-shipei`, `huangdi-era-jiangsu`, `philip-era`, `bostran-era`, `era-fascista` |
| The Taiping Heavenly Calendar: the 366-day year of 1852–1869, and its day names a day ahead | [taiping-tianli.md](taiping-tianli.md) | `taiping-tianli` |
| The calendar of the Roman province of Asia: the decree of 9/8 BC, the months from the ninth day before the Kalends, Sebaste and the leap Xandikos, and why the range starts in AD 4 | [asian-calendar.md](asian-calendar.md) | `asian` |
| The Solar Hijri calendar: Nowruz by the clock's noon and by the Sun's at Tehran, the 33-year rule and the 2 820-year cycle | [solar-hijri.md](solar-hijri.md) | `persian`, `persian-apparent-noon`, `persian-afghan`, `persian-arithmetic`, `persian-arithmetic-33` |
| The Zoroastrian calendars: the Parsi intercalation of the 1120s, the split of 1745, the Fasli of 1906, the ZRE, and the Iranian community's thirty-day months | [zoroastrian.md](zoroastrian.md) | `zoroastrian-qadimi`, `zoroastrian-shahanshahi`, `zoroastrian-fasli`; `hc-holiday`'s `zoroastrian-iranian` |
| The Armenian calendar: the Great Era's wandering year and Sarkawag's fixed year of 1084 | [armenian.md](armenian.md) | `armenian`, `armenian-fixed` |
| The Bangladeshi and Nanakshahi calendars: fixed month lengths by decree, and their revisions | [fixed-solar-namings.md](fixed-solar-namings.md) | `bangladeshi`, `nanakshahi` |
| The sexagenary cycle for year, month, day and hour, its three year boundaries and its readings | [sexagenary-cycle.md](sexagenary-cycle.md) | `sexagenary`; `hc-calendar::cycle`, `cycle::readings` |
| The Swedish runestaff: the day-letter runes, and the golden-number runes on the new moons of the Julian ecclesiastical lunar calendar | [runic-calendar.md](runic-calendar.md) | `hc-calendars-solar::cycles::runic` |
| The Japanese almanac notes: 暦注下段, 選日, 十二直, 二十八宿 and each mansion's undertakings, 九星, 六曜, 七曜, 納音 and 臘日 | [japanese-almanac-notes.md](japanese-almanac-notes.md) | `hc-almanac`: `lower_register`, `selected_days`, `twelve_directs`, `mansions`, `mansion_undertakings`, `nine_stars`, `rokuyo`, `seven_luminaries`, `nayin`, `rounichi` |
| The 雑節 and 六曜, and the 旧暦 六曜 and the moon-viewing nights read: the 社日 tie rules, the 旧暦2033年問題, the days that changed when the simplified derivation went | [zassetsu-and-rokuyo.md](zassetsu-and-rokuyo.md) | `hc-seasons`: `zassetsu`; `hc-almanac`: `rokuyo`, `moon_viewing`, `lunisolar` |
| East Asian folk days and almanac cycles: お盆 in three reckonings, 酉の市, 初午, 亥の子 and 十日夜 on the Gregorian calendar and the 旧暦, 恵方, the 八将神 and 金神, 三元九運, 손 없는 날, the Vietnamese Tam Nương and Nguyệt Kỵ, the four counts from 正月, and 入梅 and 出梅 of the Chinese almanac | [east-asian-folk-days.md](east-asian-folk-days.md) | `hc-holiday`'s `obon-july`, `obon-august`, `obon-lunar`, `tori-no-ichi`, `hatsuuma`, `hatsuuma-lunar`, `inoko`, `inoko-november`, `tokanya`, `tokanya-november`; `hc-almanac`: `lucky_direction`, `direction_deities`, `nine_periods`, `days_without_son`, `vietnamese_days`, `first_month_counts`; `hc-seasons`: `meiyu` |
| Days counted from a solar term: 寒食 under its three reckonings, the 105 days of 한식 settled against the Korean almanac, and the Observatory's 伝統的七夕 | [solar-term-counts.md](solar-term-counts.md) | `hc-seasons`: `cold_food`, `moon_calendar::traditional_tanabata`; `hc-holiday`'s `chinese-folk` and `korean-folk` |
| Spreadsheet serial dates: the Excel 1900 system and its phantom 29 February 1900, the Excel 1904 system, the OLE Automation date and its negative fractions | [spreadsheet-dates.md](spreadsheet-dates.md) | `excel-1900`, `excel-1904`, `ole-automation-date`; `spreadsheet` |
| The GNSS system times: GPS, Galileo, BeiDou and NavIC time and their epochs, the broadcast week numbers and their rollovers, GLONASS time with its leap seconds and four-year intervals | [gnss-time.md](gnss-time.md) | `hc-core`: `scale`, `epoch`, `gnss` |
| Statistical software dates: SAS dates and datetimes, Stata's `%td`, its two datetime encodings with and without leap seconds and its 52-week year, and MATLAB's `datenum` | [statistical-software-dates.md](statistical-software-dates.md) | `sas-date`, `stata-date`, `stata-week`, `matlab-datenum`; `hc-core`: `sas_stata` |
| Local mean and local apparent (sundial) time on the Earth, temporal, planetary and Italian hours, the Edo 不定時法 with the 寛政暦's and the Observatory's dawn and dusk, the ʿaṣr and the Jewish evening times, the Jewish times in temporal hours by the GRA and the Magen Avraham, as named conventions, the Ethiopian and Swahili hours, and the Chinese night watches | [hours-of-the-day.md](hours-of-the-day.md) | `hc-astro::solar_time`; `hc-seasons::planetary_hours`; `hc-format::east_african_hours`: `ethiopian-hours`, `swahili-hours`; `hc-format::night_watches` |
| Rāhu kālam, Yamaganda and Gulika kālam: the eighth of the day each takes by the weekday, from sunrise to sunset or from 06:00 to 18:00; the fifteen muhūrtas of the day and the night, Abhijit and Dur Muhurtam | [rahu-kalam.md](rahu-kalam.md) | `hc-calendars-indic::kalam`, `hc-calendars-indic::muhurta` |
| Choghadiya: the eighths of the daylight and of the night, each named for one of seven kinds by the weekday | [choghadiya.md](choghadiya.md) | `hc-calendars-indic::choghadiya` |
| Panchak: the Moon's passage through the last five nakṣatras, 300° to 360°, and the two tables that name a window by its weekday | [panchak.md](panchak.md) | `hc-calendars-indic::panchak` |
| The Kumbh Mela's conditions at its four sites and the rivers of Pushkaram: festivals set by Jupiter's sidereal sign | [jupiter-festivals.md](jupiter-festivals.md) | `hc-calendars-indic`: `kumbh`, `pushkaram` |
| Jupiter's position from the VSOP87B series: its sidereal sign, the moments it enters a sign and turns, its heliacal risings and the years of Jupiter named by them | [jupiter-ephemeris.md](jupiter-ephemeris.md) | `hc-astro`: `jupiter`, `vsop87_jupiter`; `hc-seasons`: `zodiac::jupiter`, `zodiac::node` |
| Binary timestamps: NTP's eras and the 2036 wrap, the 60-bit timestamp of UUID versions 1 and 6, the FAT date and time words, and .NET's `DateTime.Ticks` with its Kind | [binary-timestamps.md](binary-timestamps.md) | `hc-core`: `ntp`, `uuid`, `dotnet`, `epoch`; `hc-format`: `fat` |
| The CCSDS time codes: the P-field, the Unsegmented Code at Level 1 and 2, the Day Segmented and Calendar Segmented Codes and the ASCII codes A and B, with UTC's leap second | [ccsds-time-codes.md](ccsds-time-codes.md) | `hc-core`: `ccsds`, `leap`, `epoch`; `hc-format`: `ccsds` |
| The radio time codes: JJY with its call-sign frames, DCF77 for the following minute, and WWVB's amplitude code and its phase code of 2012, with their leap-second frames | [radio-time-codes.md](radio-time-codes.md) | `hc-format`: `radio` |
| The IRIG serial time codes A, B, D, E, G and H: the time of year in BCD, the year, the control bits and the straight binary seconds, and the coded expressions of Table 4-1 | [irig-time-codes.md](irig-time-codes.md) | `hc-format`: `irig` |
| Islamic prayer times by named method: the Muslim World League, ISNA, Egypt, Umm al-Qura, Karachi, Tehran, Jafari, France, Russia and Singapore | [prayer-times.md](prayer-times.md) | `hc-astro::solar_time`'s `PrayerMethod`, `PRAYER_METHODS`, `fajr`, `maghrib`, `isha`, `islamic_midnight` |
| Rising and setting: sunrise, sunset, twilight, moonrise and moonset, and the named horizons they are measured against — the default geometric dip, the USNO's sea level and *Calendrical Calculations*' 19″·√h | [rise-and-set.md](rise-and-set.md) | `hc-astro`: `riseset`, `horizon` |
| Zone locations: the principal location of each zone in the IANA database's `zone1970.tab`, the links `zone.tab` places and the rest followed through `backward` and `backzone`, and CLDR's exemplar city of each zone in a locale | [zone-locations.md](zone-locations.md) | `hc-tz`: `location`; `hc-i18n`: `exemplar_cities`; `zone_lines` |
| Zone names and day periods: CLDR's metazones, the specific, generic and location names of a zone and the localized GMT format, UTS #35's fallbacks, and the day periods beyond am and pm with each language's rules | [zone-names.md](zone-names.md) | `hc-i18n`: `zone_names`, `day_periods`; `hc-format`: the CLDR fields `z`, `O`, `v`, `V`, `b`, `B` |
| Place names: what CLDR 48 calls every territory and every ISO 3166-2 subdivision in each carried locale, CLDR's subdivision ids and the ISO form, the draft levels, the inheritance through parents to English, and how the names stand beside the holiday tables' subdivisions and the zones' countries | [place-names.md](place-names.md) | `hc-i18n`: `place_names`; `hyper_calendar::place_lines`; `hc_territories`, `hc_subdivisions`, `hc_place_name` |
| Earth rotation: UT0, UT1 and the smoothed UT2, UT1R and UT1S with the IERS 2010 zonal tide table, the Earth Rotation Angle, and the IAU 1982 and IAU 2006 Greenwich sidereal times | [earth-rotation.md](earth-rotation.md) | `hc-astro`: `ut_variants`, `earth` |
| Mars timekeeping: the Mars Sol Date, Coordinated Mars Time, local mean and true solar time, the Mars year from 1955, the mission sol conventions, the Darian calendar and its Martiana variant | [mars-timekeeping.md](mars-timekeeping.md) | `hc-planetary`: `mars`, `mars::missions`, `mars::darian`, `mars::martiana` (`martiana`) |
| Circad calendars: Gangale's Darian calendar for Titan and the Gregorian-based calendars of the Galilean moons, and why the Darian-based Galilean family is not carried | [circad-calendars.md](circad-calendars.md) | `darian-titan`, `gregorian-io`, `gregorian-europa`, `gregorian-ganymede`, `gregorian-callisto`; `hc-planetary`: `circad`, `titan`, `galilean` |
| The earliest evidence of life, of *Homo sapiens* and of writing: the published claims, each dated in the shape its source gives — an age, a minimum, a range — with the disputes named, and the Japanese names of the deep-time tables by identifier | [earliest-evidence.md](earliest-evidence.md) | `hc-deep-time`: `evidence`, `names::entry_name` |
| Written dates read back: the templates walked in reverse, every name at every width, native digits, Han and Hebrew numerals and 元年, an era or a year left out, and the refusals — an ambiguous text, a two-digit year, a year named only by a cycle or not at all, a weekday or a field that is not the day's | [written-dates.md](written-dates.md) | `hc-format`: `label::parse_date`; `hc_parse_date`; `DynCalendar::era_code` |
| Hebrew numerals: CLDR's rules for the letters, 15 and 16, the thousands and the millennium a date leaves out, and the marks as typed | [hebrew-numerals.md](hebrew-numerals.md) | `hc-i18n`: `numbering`'s `hebr`, `numbering::same_typed_mark`, `names::DateTemplates::omitted_thousands` |
| Greek numerals: CLDR's rules for the letters, the thousands and the myriads, and the three spellings of 6 read back, the digamma, the stigma and στ | [greek-numerals.md](greek-numerals.md) | `hc-i18n`: `numbering`'s `grek` and `greklow` |
| Locale fallback: UTS #35's truncation, CLDR's `parentLocales` and likely scripts, the lookups keyed by language instead, and each regional file's default numbering system | [locale-fallback.md](locale-fallback.md) | `hc-i18n`: `Locale::parent`, `Locale::fallback`, `data::PARENT_LOCALES`, `data::DEFAULT_NUMBERING`, `NumberingSystem::for_locale` |
| Where each national table of the Americas, Europe, Africa, the Middle East and Oceania begins: the first year its sources support, the gap before it, and why for each table | [holiday-first-years.md](holiday-first-years.md) | `hc-holiday`'s `countries::read_all` and the tables of `americas.rs`, `europe.rs`, `andorra.rs`, `bolivia.rs`, `mexico.rs` |

## Systems that need a document

A system is written up from its sources before it is coded, with the
sections above and a row in the table above. A system found to need a
document only after it is coded is listed here until it has one
([policy.md §12](../policy.md)). One is listed:

- **The Hindu festival days**, `hc-holiday`'s `hindu` rules and the
  `hindu` table. The document would cover:
  - the part of the day each festival's tithi must hold, and the Calendar
    Reform Committee's rules for it;
  - the two Deepavalis, Lakṣmī Pūjā and Naraka Caturdaśī, and which of the
    governments' tables keeps which;
  - the central government of India's lists, with their Vaiṣṇava
    Janmāṣṭamī.

  Until the document is written, the module documentation of
  `hc_holiday::hindu` states it. [hindu-calendars.md](hindu-calendars.md)
  covers the calendar and not the festivals.

## Systems judged not to need one

Some implemented systems are one rule each. The rule is stated in full
where it is coded and in [time-scales.md](../time-scales.md), and a worked
example would add nothing. These are the TAI64 labels and their two
conventions, the Julian and Besselian epochs, and Swatch Internet Time.
They stay there unless a competing reading or a table of exceptions turns
up.

Three more are judged not to need one, each for its own reason:

- **The Heliocentric Julian Date**, `hc-astro::hjd`. It is one
  correction, the Rømer delay. Its competing readings are time scales and
  frames, which [time-scales.md](../time-scales.md) names:
  - HJD_TT and HJD_UTC, both computed;
  - HJD′_UTC, which drifts with the leap seconds and is not computed;
  - the mixed frame of SLALIB's rows far from 2000, measured there against
    IDL's table.

  The barycentric BJD_TDB would need the solar-system barycentre. It would
  need a document if it were carried.
- **TT(BIPM)**, `hc-core::tt_bipm`. The library carries no realisation of
  it, only the interpolation of a series the caller supplies by name. So
  the realisations that revise each other are the caller's data, as a DUT1
  series is. [time-scales.md](../time-scales.md) states the rule, and
  `hc_core::tt_bipm` states the revision, up to 0.2 ns, between TT(BIPM24)
  and TT(BIPM25).
- **The Turkmen month and weekday names of 2002–2008**,
  `hc-i18n::dated::TURKMEN_2002`. It is a list of names and two dates. The
  one open question, the day in 2002 the law took effect, is answered
  `Undecided` rather than resolved. [i18n.md](../i18n.md) states the
  period and its sources. A second naming period, or a source that dates
  the law's effect, would not change the rule.
