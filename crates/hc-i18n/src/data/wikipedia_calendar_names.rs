//! What each locale's Wikipedia calls the calendars CLDR has no key for,
//! generated.
//!
//! **Do not edit.** `scripts/calendar-names-wikipedia.py` writes this file
//! from the interlanguage links of Wikipedia's articles on the registry's
//! calendars (the MediaWiki API, `action=query&prop=langlinks`, and for the
//! Chinese edition's two scripts `action=parse&prop=displaytitle` in the
//! `zh-hans` and `zh-hant` variants; read 2026-10-05,
//! `wikipedia-calendar-names`), as the script's documentation says: for
//! each calendar the script names an article about it, and each carried
//! locale whose edition the links reach takes the title of that edition's
//! article, exactly as the edition writes it; nothing is translated, and a
//! calendar no edition of a language has an article on keeps no name in
//! it, so that a page shows its English name. A calendar CLDR or
//! `crate::calendar_names` names in a locale is not here, since those
//! answer first; a title that would give two calendars one name in a locale
//! is left out, as is one written in a script other than the locale's.
//! `crate::calendar_names::sourced_calendar_name` reads the table.
//!
//! The articles read, with the revision each stood at:
//!
//! - `akan`: "Akan calendar" (en), revision 1371379884
//! - `armenian`: "Armenian calendar" (en), revision 1370859595
//! - `assyrian`: "Assyrian calendar" (en), revision 1365583819
//! - `aztec-tonalpohualli`: "Tōnalpōhualli" (en), revision 1370457609
//! - `aztec-xiuhpohualli`: "Xiuhpōhualli" (en), revision 1370457507
//! - `babylonian`: "Babylonian calendar" (en), revision 1370333938
//! - `bahai`: "Baháʼí calendar" (en), revision 1373546365
//! - `balinese-pawukon`: "Pawukon calendar" (en), revision 1370333736
//! - `berber`: "Berber calendar" (en), revision 1370824600
//! - `bikram-sambat`: "Vikram Samvat" (en), revision 1373671607
//! - `bostran-era`: "Bostran era" (en), revision 1357204822
//! - `buddhist`: "Buddhist calendar" (en), revision 1375579327
//! - `burmese`: "Burmese calendar" (en), revision 1367001745
//! - `byzantine`: "Byzantine calendar" (en), revision 1371380732
//! - `chinese`: "Chinese calendar" (en), revision 1377501044
//! - `chinese-daming`: "大明暦" (ja), revision 80372342
//! - `chinese-jingchu`: "景初暦" (ja), revision 109946243
//! - `chinese-kaihuang`: "開皇曆" (zh), revision 86433347
//! - `chinese-qianxiang`: "乾象暦" (ja), revision 80374715
//! - `chinese-sanji`: "三紀曆" (zh), revision 86433291
//! - `chinese-sifen`: "四分暦" (ja), revision 100804614
//! - `chinese-taichu`: "太初暦" (ja), revision 83241478
//! - `chinese-tianhe`: "天和曆" (zh), revision 86433370
//! - `chinese-xinghe`: "興和曆" (zh), revision 86433390
//! - `chinese-yuanjia`: "Genka calendar" (en), revision 1260001476
//! - `chinese-zhengguang`: "正光曆" (zh), revision 86433344
//! - `coptic`: "Coptic calendar" (en), revision 1377921223
//! - `dangi`: "Korean calendar" (en), revision 1374953509
//! - `discordian`: "Discordian calendar" (en), revision 1343742284
//! - `egyptian`: "Ancient Egyptian calendar" (en), revision 1373218552
//! - `era-fascista`: "Era Fascista" (en), revision 1367662039
//! - `ethiopic`: "Ethiopian calendar" (en), revision 1377716959
//! - `french-republican-equinox`: "French Republican calendar" (en), revision 1376080890
//! - `gregory`: "Gregorian calendar" (en), revision 1374418208
//! - `gupta`: "Gupta era" (en), revision 1276107364
//! - `hanke-henry`: "Hanke–Henry Permanent Calendar" (en), revision 1376032615
//! - `hebrew`: "Hebrew calendar" (en), revision 1376602290
//! - `hindu-lunar`: "Hindu calendar" (en), revision 1378288612
//! - `hindu-solar-bengali`: "Bengali calendar" (en), revision 1376728119
//! - `hindu-solar-malayalam`: "Malayalam calendar" (en), revision 1357695146
//! - `hindu-solar-tamil`: "Tamil calendar" (en), revision 1364470828
//! - `holocene`: "Holocene calendar" (en), revision 1373097779
//! - `hongxian`: "洪憲" (ja), revision 110512376
//! - `huangdi-era`: "黄帝紀元" (ja), revision 103487873
//! - `indian`: "Indian national calendar" (en), revision 1316860262
//! - `international-fixed`: "International Fixed Calendar" (en), revision 1361359641
//! - `islamic-civil`: "Tabular Islamic calendar" (en), revision 1350610030
//! - `iso8601`: "ISO 8601" (en), revision 1377416850
//! - `iso8601-ordinal`: "Ordinal date" (en), revision 1377488361
//! - `iso8601-week`: "ISO week date" (en), revision 1353454244
//! - `jalali`: "Jalali calendar" (en), revision 1340217433
//! - `japanese`: "Japanese calendar" (en), revision 1375479676
//! - `japanese-horyaku`: "Hōryaku calendar" (en), revision 1360158189
//! - `japanese-imperial`: "Japanese imperial year" (en), revision 1369143713
//! - `japanese-jokyo`: "Jōkyō calendar" (en), revision 1359009091
//! - `japanese-kansei`: "Kansei calendar" (en), revision 1325404362
//! - `japanese-senmyo`: "Xuanming calendar" (en), revision 1280891312
//! - `japanese-tenpo`: "Tenpō calendar" (en), revision 1360988747
//! - `javanese`: "Javanese calendar" (en), revision 1372036231
//! - `juche`: "Juche calendar" (en), revision 1371612834
//! - `julian`: "Julian calendar" (en), revision 1370601235
//! - `julian-day`: "Julian day" (en), revision 1373709161
//! - `kalachuri`: "Kalachuri Era" (en), revision 1348200274
//! - `kurdish`: "Kurdish calendar" (en), revision 1370838922
//! - `lilian`: "Lilian date" (en), revision 1364502863
//! - `mandaean`: "Mandaean calendar" (en), revision 1355297349
//! - `masonic-anno-lucis`: "Anno Lucis" (en), revision 1374487489
//! - `maya-haab`: "Haabʼ" (en), revision 1378122072
//! - `maya-longcount`: "Mesoamerican Long Count calendar" (en), revision 1378363988
//! - `maya-tzolkin`: "Tzolkʼin" (en), revision 1378122483
//! - `mongolian`: "Mongolian calendar" (en), revision 1360710355
//! - `nanakshahi`: "Nanakshahi calendar" (en), revision 1375294638
//! - `nepal-sambat`: "Nepal Sambat" (en), revision 1371755317
//! - `olympiad`: "Olympiad" (en), revision 1371958101
//! - `pax`: "Pax Calendar" (en), revision 1374009220
//! - `persian`: "Solar Hijri calendar" (en), revision 1376296373
//! - `positivist`: "Positivist calendar" (en), revision 1373025736
//! - `revised-julian`: "Revised Julian calendar" (en), revision 1375522663
//! - `roc`: "Republic of China calendar" (en), revision 1377423666
//! - `roman-auc`: "Ab urbe condita" (en), revision 1373977213
//! - `rumi`: "Rumi calendar" (en), revision 1360941523
//! - `sexagenary`: "Sexagenary cycle" (en), revision 1373859084
//! - `soviet-week`: "Soviet calendar" (en), revision 1373709209
//! - `spanish-era`: "Spanish era" (en), revision 1338000003
//! - `swedish-1700`: "Swedish calendar" (en), revision 1365289269
//! - `symmetry454`: "Symmetry454" (en), revision 1361199146
//! - `thai-lunar`: "Thai lunar calendar" (en), revision 1332096508
//! - `tibetan`: "Tibetan calendar" (en), revision 1371374864
//! - `vietnamese`: "Vietnamese calendar" (en), revision 1372830742
//! - `vira-nirvana-samvat`: "Vira Nirvana Samvat" (en), revision 1323057038
//! - `world-calendar`: "World Calendar" (en), revision 1362559950
//!
//! Not named, no article naming the calendar:
//!
//! - `ada`, `ansi-date`, `antioch-caesarean-era`, `antioch-caesarean-era-september`, `archetypes`, `arsacid-era`, `asian`, `ccsds-day`, `cheondogyo-podeok`, `chinese-regnal`, `chinese-regnal-qing-court`, `chronological-julian-day`, `cnes-julian-day`, `dee`, `dee-cecil`, `dublin-julian-day`, `excel-1900`, `excel-1904`, `gaza-era`, `hermetic-leap-week`, `hindu-old-lunar`, `hindu-old-solar`, `hindu-solar-reingold-dershowitz`, `hindu-solar-surya-siddhanta`, `hindu-solar-vikrami`, `julian-gregorian-bg`, `julian-gregorian-catholic`, `julian-gregorian-de-catholic`, `julian-gregorian-de-protestant`, `julian-gregorian-fr`, `julian-gregorian-gb`, `julian-gregorian-gr`, `julian-gregorian-hu`, `julian-gregorian-nl-holland`, `julian-gregorian-nl-states-general`, `julian-gregorian-ro`, `julian-gregorian-rs`, `julian-gregorian-ru`, `julian-gregorian-se`, `korean-regnal`, `korean-regnal-backdated`, `lakshmana-sena`, `lao`, `liberalia-triday-lunar`, `liberalia-triday-solar`, `magi-san`, `manchukuo`, `masonic-anno-depositionis`, `masonic-anno-inventionis`, `masonic-anno-ordinis`, `matlab-datenum`, `maya-819`, `maya-819-584286`, `maya-819-gmt2`, `meyer-palmen`, `mixtec-year`, `modified-julian-day-2000`, `ole-automation-date`, `persian-imperial`, `philip-era`, `philip-era-ptolemy`, `qumran`, `rajyabhisheka-saka`, `saptarshi`, `sas-date`, `stata-date`, `stata-week`, `sur-san`, `symmetry010`, `taiping-tianli`, `tranquility`, `truncated-julian-day`, `vietnamese-regnal-nguyen`, `week-and-month`, `yazidi`, `yerm`, `zapotec-yza`: no article on it in the English, Japanese or Chinese Wikipedia (searched 2026-10-05); a day count, a proposal or a reckoning no edition has written up.
//! - `armenian-fixed`, `bahai-arithmetic`, `bahai-astronomical`, `buddhist-lk`, `dangi-kasi`, `egyptian-ptolemy`, `french-republican-arithmetic`, `french-republican-arithmetic-richards`, `hebrew-observational`, `hindu-lunar-purnimanta`, `hindu-lunar-reingold-dershowitz`, `hindu-lunar-surya-siddhanta`, `huangdi-era-jiangsu`, `huangdi-era-liu-shipei`, `huangdi-era-tongmenghui`, `islamic-tbla`, `jalali-natanz`, `jalali-tusi`, `japanese-kaigen-toji`, `japanese-northern`, `japanese-northern-proclaimed`, `japanese-proclaimed`, `japanese-southern`, `japanese-southern-proclaimed`, `javanese-aboge`, `javanese-yogyakarta`, `masonic-anno-lucis-march`, `maya-haab-584286`, `maya-haab-gmt2`, `maya-longcount-584286`, `maya-longcount-gmt2`, `maya-tzolkin-584286`, `maya-tzolkin-gmt2`, `nepal-sambat-fortnight`, `persian-afghan`, `persian-apparent-noon`, `persian-arithmetic`, `persian-arithmetic-33`, `roman-auc-capitoline`, `tibetan-bhutan`, `tibetan-bhutan-lochen`, `tibetan-lochen`, `tibetan-tsurphu`, `tibetan-tsurphu-karana`, `vietnamese-south-1968`, `vikram-samvat-kartikadi`: a convention of a calendar whose article names the calendar, not the convention, and whose own rule carries the title.
//! - `bangladeshi`, `fasli-bombay`, `fasli-madras`, `islamic-fatimid`, `islamic-fcna`, `islamic-istanbul-2016`, `islamic-khgt`, `islamic-observational-cairo-rd`, `islamic-rgsa`, `islamic-saudi-rule-rd`, `seleucid-syrian`, `zoroastrian-fasli`, `zoroastrian-qadimi`, `zoroastrian-shahanshahi`: one article would serve several identifiers, none of them the calendar's own rule.
//! - `icelandic`, `icelandic-almanac`, `icelandic-friday`, `icelandic-julian`, `icelandic-julian-friday`, `icelandic-medieval`, `islamic-umalqura`, `khmer`, `maya-round`, `maya-round-584286`, `maya-round-gmt2`, `modified-julian-day`, `reduced-julian-day`, `samaritan`, `valabhi`: the English title redirects to a section of an article on another subject, whose links name that subject.
//! - `javanese-pasaran`, `odia-anka`, `tabot`: the article of that title is about another subject.
//!
//! 155 calendars are not named: 77 with no article, 46 conventions, 14 sharing
//! an article, 15 under a section redirect and 3 with a title about another
//! subject.
//!
//! Locales with no edition to read:
//!
//! - `aeb-Latn`: no Wikipedia edition writes the language.
//! - `ar-EG`: a regional entry, which the fallback takes to its language's names.
//! - `ayl-Latn`: no Wikipedia edition writes the language.
//! - `cop`: no Coptic Wikipedia, only an Incubator test project.
//! - `en`: the calendars' own English names (`CalendarMeta::english_name`) serve English; the English articles are the ones the links are read from.
//! - `en-001`: a regional entry, which the fallback takes to its language's names.
//! - `en-GB`: a regional entry, which the fallback takes to its language's names.
//! - `es-419`: a regional entry, which the fallback takes to its language's names.
//! - `mid`: no Wikipedia edition writes the language.
//! - `mix`: no Wikipedia edition writes the language.
//! - `pt-PT`: a regional entry, which the fallback takes to its language's names.
//! - `rif`: no Wikipedia edition writes the language.
//! - `syr`: no edition keyed `syr`; the Classical Syriac edition is keyed `arc`, Aramaic, and is not read for it.
//! - `und`: root names nothing.
//! - `ur-IN`: a regional entry, which the fallback takes to its language's names.
//! - `yua`: no Wikipedia edition writes the language.
//! - `yue-Hans`: the Cantonese edition (`zh-yue`) writes Traditional characters only.
//! - `zap`: no Wikipedia edition writes the language.
//! - `zh-Hant-HK`: a regional entry, which the fallback takes to its language's names.

use crate::names::CalendarDisplayName;

/// Each carried locale's Wikipedia titles for the calendars CLDR and the
/// hand-written entries do not name in it, sorted by tag so that a lookup can
/// search it; the identifiers sorted within a tag.
pub static WIKIPEDIA_CALENDAR_NAMES: &[(&str, &[CalendarDisplayName])] = &[
    (
        "ar",
        &[
            CalendarDisplayName::new("armenian", "تقويم أرمني"),
            CalendarDisplayName::new("assyrian", "شهور سريانية"),
            CalendarDisplayName::new("babylonian", "تقويم بابلي"),
            CalendarDisplayName::new("bahai", "تقويم بهائي"),
            CalendarDisplayName::new("berber", "تقويم أمازيغي"),
            CalendarDisplayName::new("bikram-sambat", "فيكرم سامفات"),
            CalendarDisplayName::new("bostran-era", "تقويم بصرى"),
            CalendarDisplayName::new("burmese", "تقويم بورمي"),
            CalendarDisplayName::new("byzantine", "تقويم بيزنطي"),
            CalendarDisplayName::new("discordian", "تقويم ديسكوردي"),
            CalendarDisplayName::new("egyptian", "تقويم مصري"),
            CalendarDisplayName::new("french-republican-equinox", "تقويم جمهوري فرنسي"),
            CalendarDisplayName::new("hindu-lunar", "تقويم هندي"),
            CalendarDisplayName::new("hindu-solar-bengali", "تقويم بنغالي"),
            CalendarDisplayName::new("holocene", "تقويم هولوسين"),
            CalendarDisplayName::new("jalali", "تقويم جلالي"),
            CalendarDisplayName::new("javanese", "تقويم جاوي"),
            CalendarDisplayName::new("juche", "تقويم كوري شمالي"),
            CalendarDisplayName::new("julian", "تقويم يوليوسي"),
            CalendarDisplayName::new("julian-day", "تاريخ يولياني"),
            CalendarDisplayName::new("kurdish", "تقويم كردي"),
            CalendarDisplayName::new("masonic-anno-lucis", "سنة النور"),
            CalendarDisplayName::new("maya-longcount", "تقويم العد الطويل في أمريكا الوسطى"),
            CalendarDisplayName::new("nanakshahi", "تقويم سيخي"),
            CalendarDisplayName::new("olympiad", "أولمبياد"),
            CalendarDisplayName::new("roman-auc", "تقويم بداية روما"),
            CalendarDisplayName::new("rumi", "تقويم رومي"),
            CalendarDisplayName::new("sexagenary", "دورة ستينية"),
            CalendarDisplayName::new("soviet-week", "تقويم ثوري سوفييتي"),
        ],
    ),
    (
        "ban",
        &[
            CalendarDisplayName::new("assyrian", "Kalénder Assyria"),
            CalendarDisplayName::new("berber", "Kalénder Berber"),
            CalendarDisplayName::new("buddhist", "Kalénder Budha"),
            CalendarDisplayName::new("byzantine", "Kalénder Byzantium"),
            CalendarDisplayName::new("chinese", "Imlék"),
            CalendarDisplayName::new("gregory", "Kalénder Grégorian"),
            CalendarDisplayName::new("hindu-lunar", "Kalénder Hindu"),
            CalendarDisplayName::new("hindu-solar-bengali", "Kalénder Bengali"),
            CalendarDisplayName::new("javanese", "Kalénder Jawa"),
            CalendarDisplayName::new("julian", "Kalénder Julian"),
        ],
    ),
    (
        "bn",
        &[
            CalendarDisplayName::new("armenian", "আর্মেনীয় বর্ষপঞ্জি"),
            CalendarDisplayName::new("assyrian", "অ্যাসিরীয় বর্ষপঞ্জি"),
            CalendarDisplayName::new("bahai", "বাহাই বর্ষপঞ্জি"),
            CalendarDisplayName::new("bikram-sambat", "বিক্রম সংবৎ"),
            CalendarDisplayName::new("burmese", "বর্মী বর্ষপঞ্জি"),
            CalendarDisplayName::new("byzantine", "বাইজেন্টাইন বর্ষপঞ্জি"),
            CalendarDisplayName::new("discordian", "ডিস্কর্ডীয় বর্ষপঞ্জি"),
            CalendarDisplayName::new("era-fascista", "ফ্যাসিস্ট সাল"),
            CalendarDisplayName::new("hindu-lunar", "হিন্দু পঞ্জিকা"),
            CalendarDisplayName::new("hindu-solar-bengali", "বঙ্গাব্দ"),
            CalendarDisplayName::new("holocene", "হলোসিন বর্ষপঞ্জী"),
            CalendarDisplayName::new("jalali", "জালালি বর্ষপঞ্জি"),
            CalendarDisplayName::new("julian", "জুলীয় বর্ষপঞ্জি"),
            CalendarDisplayName::new("nanakshahi", "নানকশাহী বর্ষপঞ্জি"),
            CalendarDisplayName::new("nepal-sambat", "নেপাল সংবৎ"),
            CalendarDisplayName::new("roman-auc", "নগরাব্দ"),
            CalendarDisplayName::new("rumi", "রুমি বর্ষপঞ্জি"),
            CalendarDisplayName::new("sexagenary", "ষাটবার্ষিক চক্র"),
            CalendarDisplayName::new("spanish-era", "স্পেনীয় সাল"),
            CalendarDisplayName::new("swedish-1700", "সুইডীয় বর্ষপঞ্জি"),
        ],
    ),
    (
        "cs",
        &[
            CalendarDisplayName::new("armenian", "Arménský kalendář"),
            CalendarDisplayName::new("aztec-tonalpohualli", "Tonalpohualli"),
            CalendarDisplayName::new("bahai", "Bahá’í kalendář"),
            CalendarDisplayName::new("egyptian", "Egyptský kalendář"),
            CalendarDisplayName::new(
                "french-republican-equinox",
                "Francouzský revoluční kalendář",
            ),
            CalendarDisplayName::new("hindu-lunar", "Hinduistický kalendář"),
            CalendarDisplayName::new("holocene", "Holocénový letopočet"),
            CalendarDisplayName::new("iso8601-ordinal", "Pořadové datum"),
            CalendarDisplayName::new("julian", "Juliánský kalendář"),
            CalendarDisplayName::new("julian-day", "Juliánské datum"),
            CalendarDisplayName::new("masonic-anno-lucis", "Anno Lucis"),
            CalendarDisplayName::new("maya-tzolkin", "Mayská denní znamení"),
            CalendarDisplayName::new("positivist", "Pozitivistický kalendář"),
            CalendarDisplayName::new("sexagenary", "Kan-č’"),
            CalendarDisplayName::new("soviet-week", "Sovětský revoluční kalendář"),
            CalendarDisplayName::new("spanish-era", "Španělská éra"),
            CalendarDisplayName::new("tibetan", "Tibetský kalendář"),
        ],
    ),
    (
        "de",
        &[
            CalendarDisplayName::new("armenian", "Armenischer Kalender"),
            CalendarDisplayName::new("assyrian", "Assyrischer Kalender"),
            CalendarDisplayName::new("babylonian", "Babylonischer Kalender"),
            CalendarDisplayName::new("bahai", "Badi-Kalender"),
            CalendarDisplayName::new("bikram-sambat", "Vikram Sambat"),
            CalendarDisplayName::new("byzantine", "Byzantinischer Kalender"),
            CalendarDisplayName::new("discordian", "Diskordianischer Kalender"),
            CalendarDisplayName::new("egyptian", "Ägyptischer Kalender"),
            CalendarDisplayName::new("era-fascista", "Era Fascista"),
            CalendarDisplayName::new(
                "french-republican-equinox",
                "Französischer Revolutionskalender",
            ),
            CalendarDisplayName::new("hindu-lunar", "Hinduistischer Lunisolarkalender"),
            CalendarDisplayName::new("hindu-solar-bengali", "Bengalischer Solarkalender"),
            CalendarDisplayName::new("hindu-solar-malayalam", "Malayalam-Kalender"),
            CalendarDisplayName::new("hindu-solar-tamil", "Tamilischer Kalender"),
            CalendarDisplayName::new("holocene", "Holozän-Kalender"),
            CalendarDisplayName::new("international-fixed", "Internationaler Ewiger Kalender"),
            CalendarDisplayName::new("javanese", "Javanischer Kalender"),
            CalendarDisplayName::new("julian", "Julianischer Kalender"),
            CalendarDisplayName::new("julian-day", "Julianisches Datum"),
            CalendarDisplayName::new("maya-haab", "Haab"),
            CalendarDisplayName::new("maya-longcount", "Lange Zählung"),
            CalendarDisplayName::new("maya-tzolkin", "Tzolkin"),
            CalendarDisplayName::new("mongolian", "Mongolischer Kalender"),
            CalendarDisplayName::new("olympiad", "Olympiade"),
            CalendarDisplayName::new("positivist", "Positivisten-Kalender"),
            CalendarDisplayName::new("revised-julian", "Neujulianischer Kalender"),
            CalendarDisplayName::new("roman-auc", "Ab urbe condita"),
            CalendarDisplayName::new("rumi", "Rumi-Kalender"),
            CalendarDisplayName::new("soviet-week", "Sowjetischer Revolutionskalender"),
            CalendarDisplayName::new("spanish-era", "Era"),
            CalendarDisplayName::new("swedish-1700", "Schwedischer Kalender"),
            CalendarDisplayName::new("thai-lunar", "Thailändischer Mondkalender"),
            CalendarDisplayName::new("tibetan", "Tibetischer Kalender"),
            CalendarDisplayName::new("world-calendar", "Weltkalender"),
        ],
    ),
    (
        "es",
        &[
            CalendarDisplayName::new("armenian", "Calendario pagano armenio"),
            CalendarDisplayName::new("aztec-tonalpohualli", "Tonalpohualli"),
            CalendarDisplayName::new("babylonian", "Calendario babilónico"),
            CalendarDisplayName::new("bahai", "Calendario bahaí"),
            CalendarDisplayName::new("balinese-pawukon", "Calendario pawukon"),
            CalendarDisplayName::new("berber", "Calendario bereber"),
            CalendarDisplayName::new("bikram-sambat", "Vikram Samvat"),
            CalendarDisplayName::new("byzantine", "Calendario bizantino"),
            CalendarDisplayName::new("egyptian", "Calendario egipcio"),
            CalendarDisplayName::new("era-fascista", "Calendario fascista"),
            CalendarDisplayName::new(
                "french-republican-equinox",
                "Calendario republicano francés",
            ),
            CalendarDisplayName::new("hanke-henry", "Calendario permanente Hanke-Henry"),
            CalendarDisplayName::new("hindu-lunar", "Calendario hindú"),
            CalendarDisplayName::new("hindu-solar-bengali", "Calendario bengalí"),
            CalendarDisplayName::new("holocene", "Calendario holoceno"),
            CalendarDisplayName::new("international-fixed", "Calendario fijo internacional"),
            CalendarDisplayName::new("javanese", "Calendario javanés"),
            CalendarDisplayName::new("juche", "Calendario norcoreano"),
            CalendarDisplayName::new("julian", "Calendario juliano"),
            CalendarDisplayName::new("julian-day", "Fecha juliana"),
            CalendarDisplayName::new("masonic-anno-lucis", "Anno Lucis"),
            CalendarDisplayName::new("maya-haab", "Haab"),
            CalendarDisplayName::new("maya-longcount", "Cuenta larga"),
            CalendarDisplayName::new("maya-tzolkin", "Tzolkin"),
            CalendarDisplayName::new("mongolian", "Calendario mongol"),
            CalendarDisplayName::new("nepal-sambat", "Nepal Sambat"),
            CalendarDisplayName::new("olympiad", "Olimpiada"),
            CalendarDisplayName::new("revised-julian", "Calendario juliano revisado"),
            CalendarDisplayName::new("roman-auc", "Ab Urbe condita"),
            CalendarDisplayName::new("rumi", "Calendario rumi"),
            CalendarDisplayName::new("sexagenary", "Ciclo sexagesimal chino"),
            CalendarDisplayName::new("soviet-week", "Calendario revolucionario soviético"),
            CalendarDisplayName::new("spanish-era", "Era hispánica"),
            CalendarDisplayName::new("swedish-1700", "Calendario sueco"),
            CalendarDisplayName::new("tibetan", "Calendario tibetano"),
            CalendarDisplayName::new("world-calendar", "Calendario Mundial"),
        ],
    ),
    (
        "fa",
        &[
            CalendarDisplayName::new("armenian", "گاه‌شماری ارمنی"),
            CalendarDisplayName::new("assyrian", "گاه‌شماری آسوری"),
            CalendarDisplayName::new("aztec-tonalpohualli", "توناپوالی"),
            CalendarDisplayName::new("babylonian", "گاه‌شماری بابلی"),
            CalendarDisplayName::new("bahai", "گاه‌شماری بهائی"),
            CalendarDisplayName::new("berber", "گاه‌شماری بربری"),
            CalendarDisplayName::new("bikram-sambat", "ویکرم سموت"),
            CalendarDisplayName::new("burmese", "گاه‌شماری سنتی برمه‌ای"),
            CalendarDisplayName::new("byzantine", "گاه‌شماری بیزانسی"),
            CalendarDisplayName::new("discordian", "گاه‌شماری دیسکوردیان"),
            CalendarDisplayName::new("egyptian", "گاه‌شماری مصری"),
            CalendarDisplayName::new("french-republican-equinox", "تقویم جمهوری فرانسه"),
            CalendarDisplayName::new("hanke-henry", "گاه‌شماری همیشگی هنری–هانکه"),
            CalendarDisplayName::new("hindu-lunar", "گاه‌شماری هندو"),
            CalendarDisplayName::new("hindu-solar-bengali", "گاه‌شماری بنگالی"),
            CalendarDisplayName::new("hindu-solar-tamil", "گاه‌شماری تامیل"),
            CalendarDisplayName::new("holocene", "گاه‌شماری هولوسن"),
            CalendarDisplayName::new("international-fixed", "تقویم بین‌المللی ثابت"),
            CalendarDisplayName::new("jalali", "گاه‌شماری جلالی"),
            CalendarDisplayName::new("japanese-imperial", "سال امپراتوری ژاپنی"),
            CalendarDisplayName::new("japanese-jokyo", "گاه‌شماری جوکیو"),
            CalendarDisplayName::new("japanese-senmyo", "سنمیو-رکی"),
            CalendarDisplayName::new("japanese-tenpo", "گاه‌شماری تنپو"),
            CalendarDisplayName::new("javanese", "گاه‌شماری جاوه‌ای"),
            CalendarDisplayName::new("juche", "گاه‌شماری کره شمالی"),
            CalendarDisplayName::new("julian", "گاه‌شماری ژولینی"),
            CalendarDisplayName::new("julian-day", "روز ژولیوسی"),
            CalendarDisplayName::new("kurdish", "گاه‌شماری کردی"),
            CalendarDisplayName::new("nanakshahi", "گاه‌شماری نانک‌شاهی"),
            CalendarDisplayName::new("olympiad", "المپیاد"),
            CalendarDisplayName::new("roman-auc", "از تاریخ پیدایش رم"),
            CalendarDisplayName::new("sexagenary", "چرخه شصت‌تایی"),
            CalendarDisplayName::new("soviet-week", "تقویم شوروی"),
            CalendarDisplayName::new("world-calendar", "تقویم جهانی"),
        ],
    ),
    (
        "fil",
        &[
            CalendarDisplayName::new("babylonian", "Kalendaryong Babilonyo"),
            CalendarDisplayName::new("hindu-solar-bengali", "Kalendaryong Bengali"),
            CalendarDisplayName::new("julian", "Kalendaryong Juliyano"),
        ],
    ),
    (
        "fr",
        &[
            CalendarDisplayName::new("armenian", "Calendrier arménien"),
            CalendarDisplayName::new("aztec-tonalpohualli", "Tonalpohualli"),
            CalendarDisplayName::new("bahai", "Calendrier badīʿ"),
            CalendarDisplayName::new("balinese-pawukon", "Calendrier pawukon"),
            CalendarDisplayName::new("berber", "Calendrier berbère"),
            CalendarDisplayName::new("bikram-sambat", "Vikram Samvat"),
            CalendarDisplayName::new("byzantine", "Calendrier byzantin"),
            CalendarDisplayName::new("chinese-yuanjia", "Calendrier Genka"),
            CalendarDisplayName::new("egyptian", "Calendrier de l'Égypte antique"),
            CalendarDisplayName::new("era-fascista", "Calendrier fasciste"),
            CalendarDisplayName::new("french-republican-equinox", "Calendrier républicain"),
            CalendarDisplayName::new("gupta", "Ère Gupta"),
            CalendarDisplayName::new("hanke-henry", "Calendrier permanent Hanke-Henry"),
            CalendarDisplayName::new("hindu-lunar", "Calendrier hindou"),
            CalendarDisplayName::new("hindu-solar-bengali", "Calendrier bengali"),
            CalendarDisplayName::new("hindu-solar-malayalam", "Calendrier malayalam"),
            CalendarDisplayName::new("hindu-solar-tamil", "Calendrier tamoul"),
            CalendarDisplayName::new("holocene", "Calendrier holocène"),
            CalendarDisplayName::new("international-fixed", "Calendrier fixe"),
            CalendarDisplayName::new("japanese-horyaku", "Calendrier Hōryaku"),
            CalendarDisplayName::new("japanese-imperial", "Année impériale japonaise"),
            CalendarDisplayName::new("japanese-jokyo", "Calendrier Jōkyō"),
            CalendarDisplayName::new("japanese-kansei", "Calendrier Kansei"),
            CalendarDisplayName::new("japanese-senmyo", "Calendrier Xuanming"),
            CalendarDisplayName::new("japanese-tenpo", "Calendrier Tenpō"),
            CalendarDisplayName::new("juche", "Calendrier juche"),
            CalendarDisplayName::new("julian", "Calendrier julien"),
            CalendarDisplayName::new("julian-day", "Jour julien"),
            CalendarDisplayName::new("kurdish", "Calendrier kurde"),
            CalendarDisplayName::new("masonic-anno-lucis", "Calendrier maçonnique"),
            CalendarDisplayName::new("maya-haab", "Calendrier haab"),
            CalendarDisplayName::new("maya-longcount", "Compte long"),
            CalendarDisplayName::new("maya-tzolkin", "Calendrier Tzolk'in"),
            CalendarDisplayName::new("nanakshahi", "Calendrier Nanakshahi"),
            CalendarDisplayName::new("olympiad", "Olympiade"),
            CalendarDisplayName::new("pax", "Calendrier Pax"),
            CalendarDisplayName::new("revised-julian", "Calendrier julien révisé"),
            CalendarDisplayName::new("roman-auc", "Ab Urbe condita"),
            CalendarDisplayName::new("rumi", "Calendrier rumi"),
            CalendarDisplayName::new("sexagenary", "Cycle sexagésimal chinois"),
            CalendarDisplayName::new("soviet-week", "Calendrier révolutionnaire soviétique"),
            CalendarDisplayName::new("spanish-era", "Ère d'Espagne"),
            CalendarDisplayName::new("swedish-1700", "Calendrier suédois"),
            CalendarDisplayName::new("symmetry454", "Symmetry454"),
            CalendarDisplayName::new("thai-lunar", "Calendrier lunaire thaïlandais"),
            CalendarDisplayName::new("tibetan", "Calendrier tibétain"),
            CalendarDisplayName::new("world-calendar", "Calendrier universel"),
        ],
    ),
    (
        "ha",
        &[CalendarDisplayName::new("masonic-anno-lucis", "Anno Lucis")],
    ),
    (
        "he",
        &[
            CalendarDisplayName::new("aztec-tonalpohualli", "טונאלפוואלי"),
            CalendarDisplayName::new("babylonian", "הלוח הבבלי"),
            CalendarDisplayName::new("bahai", "לוח השנה הבהאי"),
            CalendarDisplayName::new("berber", "לוח השנה האמזיע'י"),
            CalendarDisplayName::new("byzantine", "הלוח הביזנטי"),
            CalendarDisplayName::new("french-republican-equinox", "לוח השנה הרפובליקני הצרפתי"),
            CalendarDisplayName::new("hindu-lunar", "לוח השנה ההינדואי"),
            CalendarDisplayName::new("holocene", "לוח השנה ההולוקני"),
            CalendarDisplayName::new("juche", "לוח השנה הצפון קוריאני"),
            CalendarDisplayName::new("julian", "הלוח היוליאני"),
            CalendarDisplayName::new("julian-day", "יום יוליאני"),
            CalendarDisplayName::new("olympiad", "אולימפיאדה"),
            CalendarDisplayName::new("roman-auc", "Ab urbe condita"),
            CalendarDisplayName::new("soviet-week", "הלוח המהפכני הסובייטי"),
            CalendarDisplayName::new("swedish-1700", "לוח השנה השוודי"),
        ],
    ),
    (
        "hi",
        &[
            CalendarDisplayName::new("balinese-pawukon", "पावुकों पञ्चाङ्ग"),
            CalendarDisplayName::new("bikram-sambat", "विक्रम संवत"),
            CalendarDisplayName::new("burmese", "म्यान्मार का पंचांग"),
            CalendarDisplayName::new("gupta", "गुप्ताब्द"),
            CalendarDisplayName::new("hindu-lunar", "हिन्दू पंचांग"),
            CalendarDisplayName::new("hindu-solar-bengali", "बंगाली पंचांग"),
            CalendarDisplayName::new("hindu-solar-tamil", "तमिल कैलेंडर"),
            CalendarDisplayName::new("julian", "जूलियन कैलेंडर"),
            CalendarDisplayName::new("julian-day", "जूलियन दिन"),
            CalendarDisplayName::new("nanakshahi", "नानकशाही जंतरी"),
            CalendarDisplayName::new("nepal-sambat", "नेपाल सम्वत्"),
            CalendarDisplayName::new("olympiad", "ओलम्पियाड"),
            CalendarDisplayName::new("tibetan", "तिब्बती पंचांग"),
            CalendarDisplayName::new("vira-nirvana-samvat", "वीर निर्वाण संवत"),
        ],
    ),
    (
        "id",
        &[
            CalendarDisplayName::new("armenian", "Kalender Armenia"),
            CalendarDisplayName::new("assyrian", "Kalender Asiria"),
            CalendarDisplayName::new("aztec-tonalpohualli", "Tōnalpōhualli"),
            CalendarDisplayName::new("aztec-xiuhpohualli", "Xiuhpōhualli"),
            CalendarDisplayName::new("babylonian", "Kalender Babel"),
            CalendarDisplayName::new("bahai", "Kalender Baha'i"),
            CalendarDisplayName::new("berber", "Kalender Berber"),
            CalendarDisplayName::new("bikram-sambat", "Vikram Samvat"),
            CalendarDisplayName::new("byzantine", "Kalender Romawi Timur"),
            CalendarDisplayName::new("egyptian", "Kalender Mesir"),
            CalendarDisplayName::new("french-republican-equinox", "Kalender Revolusi Prancis"),
            CalendarDisplayName::new("hindu-lunar", "Kalender Hindu"),
            CalendarDisplayName::new("hindu-solar-bengali", "Kalender Bengali"),
            CalendarDisplayName::new("holocene", "Kalender Holosen"),
            CalendarDisplayName::new("japanese-imperial", "Tahun Jepang"),
            CalendarDisplayName::new("japanese-tenpo", "Kalender Tempō"),
            CalendarDisplayName::new("javanese", "Kalender Jawa"),
            CalendarDisplayName::new("juche", "Kalender Juche"),
            CalendarDisplayName::new("julian", "Kalender Julius"),
            CalendarDisplayName::new("julian-day", "Hari Julian"),
            CalendarDisplayName::new("masonic-anno-lucis", "Anno Lucis"),
            CalendarDisplayName::new("maya-longcount", "Kalender Hitung Panjang"),
            CalendarDisplayName::new("maya-tzolkin", "Kalender Tzolkin"),
            CalendarDisplayName::new("nanakshahi", "Kalender Nanakshahi"),
            CalendarDisplayName::new("olympiad", "Olimpiade"),
            CalendarDisplayName::new("revised-julian", "Kalender Julius terevisi"),
            CalendarDisplayName::new("roman-auc", "Ab urbe condita"),
            CalendarDisplayName::new("sexagenary", "Ganzhi"),
            CalendarDisplayName::new("swedish-1700", "Kalender Swedia"),
        ],
    ),
    (
        "it",
        &[
            CalendarDisplayName::new("armenian", "Calendario armeno"),
            CalendarDisplayName::new("assyrian", "Calendario assiro"),
            CalendarDisplayName::new("aztec-tonalpohualli", "Tonalpohualli"),
            CalendarDisplayName::new("aztec-xiuhpohualli", "Xiuhpohualli"),
            CalendarDisplayName::new("babylonian", "Calendario babilonese"),
            CalendarDisplayName::new("bahai", "Calendario bahá'í"),
            CalendarDisplayName::new("balinese-pawukon", "Calendario balinese Pawukon"),
            CalendarDisplayName::new("berber", "Calendario berbero"),
            CalendarDisplayName::new("bikram-sambat", "Vikram Samvat"),
            CalendarDisplayName::new("burmese", "Calendario birmano"),
            CalendarDisplayName::new("byzantine", "Calendario bizantino"),
            CalendarDisplayName::new("egyptian", "Calendario egizio"),
            CalendarDisplayName::new("era-fascista", "Era fascista"),
            CalendarDisplayName::new(
                "french-republican-equinox",
                "Calendario rivoluzionario francese",
            ),
            CalendarDisplayName::new("hindu-lunar", "Calendario induista"),
            CalendarDisplayName::new("hindu-solar-bengali", "Calendario bengalese"),
            CalendarDisplayName::new("holocene", "Era olocenica"),
            CalendarDisplayName::new("international-fixed", "Calendario Cotsworth"),
            CalendarDisplayName::new("juche", "Calendario nordcoreano"),
            CalendarDisplayName::new("julian", "Calendario giuliano"),
            CalendarDisplayName::new("julian-day", "Giorno giuliano"),
            CalendarDisplayName::new("maya-haab", "Haab'"),
            CalendarDisplayName::new("maya-tzolkin", "Tzolkin"),
            CalendarDisplayName::new("nepal-sambat", "Nepal Sambat"),
            CalendarDisplayName::new("olympiad", "Olimpiade"),
            CalendarDisplayName::new("roman-auc", "Ab Urbe condita"),
            CalendarDisplayName::new("rumi", "Calendario Rumi"),
            CalendarDisplayName::new("sexagenary", "Ganzhi"),
            CalendarDisplayName::new("soviet-week", "Calendario rivoluzionario sovietico"),
            CalendarDisplayName::new("spanish-era", "Era ispanica"),
            CalendarDisplayName::new("swedish-1700", "Calendario svedese"),
            CalendarDisplayName::new("symmetry454", "Symmetry454"),
            CalendarDisplayName::new("world-calendar", "Calendario mondiale"),
        ],
    ),
    (
        "ja",
        &[
            CalendarDisplayName::new("armenian", "アルメニア暦"),
            CalendarDisplayName::new("aztec-tonalpohualli", "トナルポワリ"),
            CalendarDisplayName::new("aztec-xiuhpohualli", "シウポワリ"),
            CalendarDisplayName::new("babylonian", "バビロニア暦"),
            CalendarDisplayName::new("bahai", "バハイ暦"),
            CalendarDisplayName::new("bikram-sambat", "ヴィクラマ暦"),
            CalendarDisplayName::new("burmese", "ビルマ暦"),
            CalendarDisplayName::new("chinese-daming", "大明暦"),
            CalendarDisplayName::new("chinese-jingchu", "景初暦"),
            CalendarDisplayName::new("chinese-qianxiang", "乾象暦"),
            CalendarDisplayName::new("chinese-sifen", "四分暦"),
            CalendarDisplayName::new("chinese-taichu", "太初暦"),
            CalendarDisplayName::new("chinese-yuanjia", "元嘉暦"),
            CalendarDisplayName::new("discordian", "ディスコーディアン暦"),
            CalendarDisplayName::new("era-fascista", "ファシスト暦"),
            CalendarDisplayName::new("french-republican-equinox", "フランス革命暦"),
            CalendarDisplayName::new("hanke-henry", "ハンキ＝ヘンリー・パーマネント・カレンダー"),
            CalendarDisplayName::new("hindu-lunar", "ヒンドゥー暦"),
            CalendarDisplayName::new("holocene", "人類紀元"),
            CalendarDisplayName::new("hongxian", "洪憲"),
            CalendarDisplayName::new("huangdi-era", "黄帝紀元"),
            CalendarDisplayName::new("international-fixed", "国際固定暦"),
            CalendarDisplayName::new("jalali", "ジャラーリー暦"),
            CalendarDisplayName::new("japanese-horyaku", "宝暦暦"),
            CalendarDisplayName::new("japanese-imperial", "神武天皇即位紀元"),
            CalendarDisplayName::new("japanese-jokyo", "貞享暦"),
            CalendarDisplayName::new("japanese-kansei", "寛政暦"),
            CalendarDisplayName::new("japanese-senmyo", "宣明暦"),
            CalendarDisplayName::new("japanese-tenpo", "天保暦"),
            CalendarDisplayName::new("juche", "主体年号"),
            CalendarDisplayName::new("julian", "ユリウス暦"),
            CalendarDisplayName::new("julian-day", "ユリウス通日"),
            CalendarDisplayName::new("lilian", "リリウス日"),
            CalendarDisplayName::new("maya-haab", "ハアブ"),
            CalendarDisplayName::new("maya-longcount", "長期暦"),
            CalendarDisplayName::new("maya-tzolkin", "ツォルキン"),
            CalendarDisplayName::new("nepal-sambat", "ネパール暦"),
            CalendarDisplayName::new("olympiad", "オリンピアード"),
            CalendarDisplayName::new("pax", "13の月の暦"),
            CalendarDisplayName::new("revised-julian", "修正ユリウス暦"),
            CalendarDisplayName::new("roman-auc", "ローマ建国紀元"),
            CalendarDisplayName::new("sexagenary", "干支"),
            CalendarDisplayName::new("soviet-week", "ソビエト連邦暦"),
            CalendarDisplayName::new("swedish-1700", "スウェーデン暦"),
            CalendarDisplayName::new("thai-lunar", "チャントラカティ"),
            CalendarDisplayName::new("world-calendar", "世界暦"),
        ],
    ),
    (
        "jv",
        &[
            CalendarDisplayName::new("french-republican-equinox", "Pananggalan Révolusi Prancis"),
            CalendarDisplayName::new("indian", "Pananggalan Saka"),
            CalendarDisplayName::new("japanese-imperial", "Pananggalan Jepang"),
            CalendarDisplayName::new("javanese", "Pananggalan Jawa"),
            CalendarDisplayName::new("julian", "Pananggalan Julius"),
            CalendarDisplayName::new("olympiad", "Olimpiadhe"),
        ],
    ),
    (
        "kab",
        &[CalendarDisplayName::new("berber", "Taswast tamaziɣt")],
    ),
    (
        "ko",
        &[
            CalendarDisplayName::new("armenian", "아르메니아력"),
            CalendarDisplayName::new("assyrian", "아시리아력"),
            CalendarDisplayName::new("babylonian", "바빌로니아력"),
            CalendarDisplayName::new("bahai", "바하이력"),
            CalendarDisplayName::new("berber", "베르베르력"),
            CalendarDisplayName::new("bikram-sambat", "비크람 삼바트"),
            CalendarDisplayName::new("chinese-daming", "대명력"),
            CalendarDisplayName::new("chinese-taichu", "태초력"),
            CalendarDisplayName::new("chinese-yuanjia", "원가력"),
            CalendarDisplayName::new("egyptian", "이집트력"),
            CalendarDisplayName::new("french-republican-equinox", "프랑스 혁명력"),
            CalendarDisplayName::new("gupta", "굽타기원"),
            CalendarDisplayName::new("hindu-lunar", "힌두력"),
            CalendarDisplayName::new("holocene", "인류력"),
            CalendarDisplayName::new("international-fixed", "국제고정력"),
            CalendarDisplayName::new("japanese-imperial", "진무 천황 즉위기원"),
            CalendarDisplayName::new("japanese-jokyo", "조쿄력"),
            CalendarDisplayName::new("japanese-senmyo", "선명력"),
            CalendarDisplayName::new("japanese-tenpo", "덴포력"),
            CalendarDisplayName::new("juche", "주체연호"),
            CalendarDisplayName::new("julian", "율리우스력"),
            CalendarDisplayName::new("julian-day", "율리우스일"),
            CalendarDisplayName::new("maya-longcount", "메소아메리카 장주기 달력"),
            CalendarDisplayName::new("mongolian", "몽골력"),
            CalendarDisplayName::new("nepal-sambat", "네팔 삼바트"),
            CalendarDisplayName::new("olympiad", "올림피아드"),
            CalendarDisplayName::new("revised-julian", "개정 율리우스력"),
            CalendarDisplayName::new("roman-auc", "로마 건국 원년"),
            CalendarDisplayName::new("rumi", "루미력"),
            CalendarDisplayName::new("sexagenary", "간지"),
            CalendarDisplayName::new("thai-lunar", "태국 태음력"),
            CalendarDisplayName::new("tibetan", "티베트력"),
            CalendarDisplayName::new("world-calendar", "세계력"),
        ],
    ),
    (
        "ml",
        &[
            CalendarDisplayName::new("egyptian", "ഈജിപ്ഷ്യൻ കലണ്ടർ"),
            CalendarDisplayName::new("french-republican-equinox", "റിപ്പബ്ലിക്കൻ കലണ്ടർ"),
            CalendarDisplayName::new("hindu-solar-malayalam", "കൊല്ലവർഷ കാലഗണനാരീതി"),
            CalendarDisplayName::new("julian-day", "ജൂലിയൻ ദിനസംഖ്യ"),
            CalendarDisplayName::new("nanakshahi", "നാനക്ഷി കലണ്ടർ"),
        ],
    ),
    (
        "mr",
        &[
            CalendarDisplayName::new("bikram-sambat", "विक्रम संवत्सर"),
            CalendarDisplayName::new("hindu-lunar", "हिंदू दिनदर्शिका"),
            CalendarDisplayName::new("julian", "ज्युलियन दिनदर्शिका"),
            CalendarDisplayName::new("nanakshahi", "नानकशाही"),
        ],
    ),
    (
        "my",
        &[
            CalendarDisplayName::new("burmese", "မြန်မာ သက္ကရာဇ်"),
            CalendarDisplayName::new("julian-day", "ဂျူလီယန်နေ့စွဲ"),
            CalendarDisplayName::new("world-calendar", "ကမ္ဘာ့ပြက္ခဒိန်"),
        ],
    ),
    (
        "nah",
        &[
            CalendarDisplayName::new("aztec-tonalpohualli", "Tonalpohualli"),
            CalendarDisplayName::new("aztec-xiuhpohualli", "Xiuhpohualli"),
        ],
    ),
    (
        "ne",
        &[
            CalendarDisplayName::new("bikram-sambat", "विक्रम सम्वत्"),
            CalendarDisplayName::new("hindu-lunar", "हिन्दु पञ्चाङ्ग"),
            CalendarDisplayName::new("hindu-solar-bengali", "बङ्गाली पात्रो"),
            CalendarDisplayName::new("nepal-sambat", "नेपाल सम्वत्"),
        ],
    ),
    (
        "nl",
        &[
            CalendarDisplayName::new("armenian", "Armeense kalender"),
            CalendarDisplayName::new("assyrian", "Assyrische kalender"),
            CalendarDisplayName::new("aztec-tonalpohualli", "Tonalpohualli"),
            CalendarDisplayName::new("aztec-xiuhpohualli", "Xiuhpohualli"),
            CalendarDisplayName::new("bahai", "Bahai-kalender"),
            CalendarDisplayName::new("bikram-sambat", "Vikram Samvat"),
            CalendarDisplayName::new("egyptian", "Egyptische kalender"),
            CalendarDisplayName::new("era-fascista", "Fascistische jaartelling"),
            CalendarDisplayName::new("french-republican-equinox", "Franse republikeinse kalender"),
            CalendarDisplayName::new("hindu-lunar", "Hindoekalender"),
            CalendarDisplayName::new("holocene", "Holocene kalender"),
            CalendarDisplayName::new("huangdi-era", "Chinese jaartelling"),
            CalendarDisplayName::new("international-fixed", "Wereldwijde vaste kalender"),
            CalendarDisplayName::new("javanese", "Javaanse kalender"),
            CalendarDisplayName::new("julian", "Juliaanse kalender"),
            CalendarDisplayName::new("julian-day", "Juliaanse dag"),
            CalendarDisplayName::new("masonic-anno-lucis", "Maçonnieke kalender"),
            CalendarDisplayName::new("maya-haab", "Haab"),
            CalendarDisplayName::new("maya-longcount", "Lange telling"),
            CalendarDisplayName::new("maya-tzolkin", "Tzolkin"),
            CalendarDisplayName::new("olympiad", "Olympiade"),
            CalendarDisplayName::new("roman-auc", "Ab urbe condita"),
            CalendarDisplayName::new("tibetan", "Tibetaanse kalender"),
            CalendarDisplayName::new("world-calendar", "Wereldkalender"),
        ],
    ),
    (
        "pa-Arab",
        &[
            CalendarDisplayName::new("assyrian", "آشوری تقویم"),
            CalendarDisplayName::new("bikram-sambat", "بکرمی تقویم"),
            CalendarDisplayName::new("chinese", "چینی تقویم"),
            CalendarDisplayName::new("gregory", "گریگوری کیلنڈر"),
            CalendarDisplayName::new("hebrew", "عبرانی کیلنڈر"),
            CalendarDisplayName::new("hindu-lunar", "ہندو تقویم"),
            CalendarDisplayName::new("hindu-solar-bengali", "بنگالی تقویم"),
            CalendarDisplayName::new("holocene", "انسانی دور کیلنڈر"),
            CalendarDisplayName::new("nanakshahi", "نانک شاہی جنتری"),
            CalendarDisplayName::new("persian", "شمسی ہجری تقویم"),
        ],
    ),
    (
        "pa-Guru",
        &[
            CalendarDisplayName::new("bikram-sambat", "ਬਿਕਰਮੀ ਸੰਮਤ"),
            CalendarDisplayName::new("hindu-lunar", "ਹਿੰਦੂ ਕੈਲੰਡਰ"),
            CalendarDisplayName::new("julian", "ਜੂਲੀਅਨ ਕੈਲੰਡਰ"),
            CalendarDisplayName::new("nanakshahi", "ਨਾਨਕਸ਼ਾਹੀ ਕੈਲੰਡਰ"),
        ],
    ),
    (
        "pl",
        &[
            CalendarDisplayName::new("babylonian", "Kalendarz babiloński"),
            CalendarDisplayName::new("bikram-sambat", "Kalendarz Wikrama"),
            CalendarDisplayName::new("burmese", "Kalendarz birmański"),
            CalendarDisplayName::new("byzantine", "Kalendarz bizantyński"),
            CalendarDisplayName::new("egyptian", "Kalendarz egipski"),
            CalendarDisplayName::new(
                "french-republican-equinox",
                "Francuski kalendarz rewolucyjny",
            ),
            CalendarDisplayName::new("holocene", "Kalendarz holoceński"),
            CalendarDisplayName::new("juche", "Kalendarz dżucze"),
            CalendarDisplayName::new("julian", "Kalendarz juliański"),
            CalendarDisplayName::new("julian-day", "Data juliańska"),
            CalendarDisplayName::new("maya-haab", "Haab"),
            CalendarDisplayName::new("maya-longcount", "Długa rachuba"),
            CalendarDisplayName::new("maya-tzolkin", "Tzolkin"),
            CalendarDisplayName::new("olympiad", "Olimpiada"),
            CalendarDisplayName::new("revised-julian", "Kalendarz nowojuliański"),
            CalendarDisplayName::new("roman-auc", "Ab urbe condita"),
            CalendarDisplayName::new("soviet-week", "Kalendarz radziecki"),
            CalendarDisplayName::new("swedish-1700", "Kalendarz szwedzki"),
            CalendarDisplayName::new("symmetry454", "Symmetry454"),
            CalendarDisplayName::new("world-calendar", "World Calendar"),
        ],
    ),
    (
        "ps",
        &[
            CalendarDisplayName::new("gregory", "ګرېګوري کليز"),
            CalendarDisplayName::new("hindu-lunar", "هندي تقويم"),
            CalendarDisplayName::new("julian", "جولين کليز"),
            CalendarDisplayName::new("persian", "لمريز لېږديز کليز"),
        ],
    ),
    (
        "pt",
        &[
            CalendarDisplayName::new("armenian", "Calendário arménio"),
            CalendarDisplayName::new("aztec-tonalpohualli", "Tonalpohualli"),
            CalendarDisplayName::new("babylonian", "Calendário babilônico"),
            CalendarDisplayName::new("bahai", "Calendário bahá'í"),
            CalendarDisplayName::new("bikram-sambat", "Vikram Samvat"),
            CalendarDisplayName::new("bostran-era", "Era bostrena"),
            CalendarDisplayName::new("byzantine", "Calendário bizantino"),
            CalendarDisplayName::new("egyptian", "Calendário egípcio"),
            CalendarDisplayName::new("era-fascista", "Calendário fascista"),
            CalendarDisplayName::new(
                "french-republican-equinox",
                "Calendário revolucionário francês",
            ),
            CalendarDisplayName::new("hindu-lunar", "Calendário hindu"),
            CalendarDisplayName::new("holocene", "Calendário Holoceno"),
            CalendarDisplayName::new("international-fixed", "Calendário Fixo Internacional"),
            CalendarDisplayName::new("juche", "Calendário norte-coreano"),
            CalendarDisplayName::new("julian", "Calendário juliano"),
            CalendarDisplayName::new("julian-day", "Data juliana"),
            CalendarDisplayName::new("mandaean", "Calendário mandeu"),
            CalendarDisplayName::new("masonic-anno-lucis", "Calendário maçónico"),
            CalendarDisplayName::new("maya-haab", "Haabʼ"),
            CalendarDisplayName::new("maya-longcount", "Contagem longa"),
            CalendarDisplayName::new("maya-tzolkin", "Tzolkʼin"),
            CalendarDisplayName::new("nepal-sambat", "Nepal Sambat"),
            CalendarDisplayName::new("olympiad", "Olimpíada"),
            CalendarDisplayName::new("positivist", "Calendário positivista"),
            CalendarDisplayName::new("roman-auc", "Ab urbe condita"),
            CalendarDisplayName::new("soviet-week", "Calendário soviético"),
            CalendarDisplayName::new("spanish-era", "Era hispânica"),
            CalendarDisplayName::new("swedish-1700", "Calendário sueco"),
            CalendarDisplayName::new("symmetry454", "Symmetry454"),
        ],
    ),
    (
        "ru",
        &[
            CalendarDisplayName::new("armenian", "Армянский церковный календарь"),
            CalendarDisplayName::new("assyrian", "Ассирийский календарь"),
            CalendarDisplayName::new("aztec-tonalpohualli", "Тональпоуалли"),
            CalendarDisplayName::new("babylonian", "Вавилонский календарь"),
            CalendarDisplayName::new("bahai", "Календарь бахаи"),
            CalendarDisplayName::new("bikram-sambat", "Викрам-самват"),
            CalendarDisplayName::new("byzantine", "Византийский календарь"),
            CalendarDisplayName::new("egyptian", "Древнеегипетский календарь"),
            CalendarDisplayName::new("era-fascista", "Era Fascista"),
            CalendarDisplayName::new(
                "french-republican-equinox",
                "Французский республиканский календарь",
            ),
            CalendarDisplayName::new("hanke-henry", "Постоянный календарь Ханке — Генри"),
            CalendarDisplayName::new("hindu-lunar", "Древнеиндийский календарь"),
            CalendarDisplayName::new("hindu-solar-bengali", "Бенгальский календарь"),
            CalendarDisplayName::new("hindu-solar-tamil", "Тамильский календарь"),
            CalendarDisplayName::new("holocene", "Голоценовая эра"),
            CalendarDisplayName::new(
                "international-fixed",
                "Международный фиксированный календарь",
            ),
            CalendarDisplayName::new("javanese", "Яванский календарь"),
            CalendarDisplayName::new("juche", "Календарь чучхе"),
            CalendarDisplayName::new("julian", "Юлианский календарь"),
            CalendarDisplayName::new("julian-day", "Юлианская дата"),
            CalendarDisplayName::new("lilian", "Лилианская дата"),
            CalendarDisplayName::new("masonic-anno-lucis", "Масонский календарь"),
            CalendarDisplayName::new(
                "maya-longcount",
                "Мезоамериканский календарь длинного счёта",
            ),
            CalendarDisplayName::new("maya-tzolkin", "Цолькин"),
            CalendarDisplayName::new("olympiad", "Олимпиада"),
            CalendarDisplayName::new("positivist", "Календарь Конта"),
            CalendarDisplayName::new("revised-julian", "Новоюлианский календарь"),
            CalendarDisplayName::new("roman-auc", "Ab Urbe condita"),
            CalendarDisplayName::new("rumi", "Румийский календарь"),
            CalendarDisplayName::new("sexagenary", "Система гань-чжи"),
            CalendarDisplayName::new("soviet-week", "Непрерывная рабочая неделя"),
            CalendarDisplayName::new("spanish-era", "Испанская эра"),
            CalendarDisplayName::new("swedish-1700", "Шведский календарь"),
            CalendarDisplayName::new("symmetry454", "Symmetry454"),
            CalendarDisplayName::new("tibetan", "Тибетский календарь"),
            CalendarDisplayName::new("vietnamese", "Вьетнамский календарь"),
        ],
    ),
    (
        "sa",
        &[
            CalendarDisplayName::new("bikram-sambat", "विक्रमसंवत्"),
            CalendarDisplayName::new("hindu-solar-bengali", "वङ्गाब्दः"),
            CalendarDisplayName::new("indian", "भारतस्य राष्ट्रियपञ्चाङ्गम्"),
        ],
    ),
    (
        "shi-Latn",
        &[CalendarDisplayName::new("berber", "Asggʷas amaziɣ")],
    ),
    (
        "sw",
        &[
            CalendarDisplayName::new("armenian", "Kalenda ya Kiarmenia"),
            CalendarDisplayName::new("bikram-sambat", "Vikram Samvat"),
            CalendarDisplayName::new("egyptian", "Kalenda ya Misri ya Kale"),
            CalendarDisplayName::new("julian", "Kalenda ya Juliasi"),
            CalendarDisplayName::new("lilian", "Tarehe ya Lilius"),
            CalendarDisplayName::new("roman-auc", "Ab urbe condita"),
        ],
    ),
    (
        "ta",
        &[
            CalendarDisplayName::new("bikram-sambat", "விக்ரம் நாட்காட்டி"),
            CalendarDisplayName::new("hindu-lunar", "இந்து நாட்காட்டி"),
            CalendarDisplayName::new("hindu-solar-malayalam", "கொல்ல ஆண்டு"),
            CalendarDisplayName::new("hindu-solar-tamil", "தமிழ் நாட்காட்டி"),
            CalendarDisplayName::new("holocene", "ஓலோசீன் நாட்காட்டி"),
            CalendarDisplayName::new("julian", "யூலியன் நாட்காட்டி"),
            CalendarDisplayName::new("nanakshahi", "நானக்சாகி நாட்காட்டி"),
            CalendarDisplayName::new("olympiad", "ஒலிம்பியாடு"),
            CalendarDisplayName::new("roman-auc", "அப் ஊர்பி கொண்டிட்டா"),
        ],
    ),
    (
        "te",
        &[
            CalendarDisplayName::new("bikram-sambat", "విక్రమాదిత్య శకం"),
            CalendarDisplayName::new("gupta", "గుప్త యుగం"),
            CalendarDisplayName::new("hindu-lunar", "హిందూ కాలగణన"),
            CalendarDisplayName::new("hindu-solar-bengali", "బెంగాలీ క్యాలెండర్"),
            CalendarDisplayName::new("hindu-solar-malayalam", "మలయాళ క్యాలెండర్"),
            CalendarDisplayName::new("hindu-solar-tamil", "తమిళ క్యాలెండర్"),
            CalendarDisplayName::new("nanakshahi", "నానక్‌షాహి కేలండర్"),
        ],
    ),
    (
        "th",
        &[
            CalendarDisplayName::new("armenian", "ปฏิทินอาร์มีเนีย"),
            CalendarDisplayName::new("assyrian", "ปฏิทินอัสซีเรีย"),
            CalendarDisplayName::new("bikram-sambat", "วิกรมสัมวัต"),
            CalendarDisplayName::new("burmese", "ปฏิทินพม่า"),
            CalendarDisplayName::new("discordian", "ปฏิทินดิสคอร์เดียน"),
            CalendarDisplayName::new("french-republican-equinox", "ปฏิทินสาธารณรัฐฝรั่งเศส"),
            CalendarDisplayName::new("hindu-lunar", "ปฏิทินฮินดู"),
            CalendarDisplayName::new("hindu-solar-bengali", "ปฏิทินเบงกอล"),
            CalendarDisplayName::new("holocene", "ปฏิทินโฮโลซีน"),
            CalendarDisplayName::new("international-fixed", "ปฏิทินคงที่สากล"),
            CalendarDisplayName::new("japanese-horyaku", "ปฏิทินโฮเรียกุ"),
            CalendarDisplayName::new("japanese-jokyo", "ปฏิทินโจเกียว"),
            CalendarDisplayName::new("japanese-kansei", "ปฏิทินคันเซ"),
            CalendarDisplayName::new("japanese-senmyo", "ปฏิทินเซฺวียนหมิง"),
            CalendarDisplayName::new("japanese-tenpo", "ปฏิทินเท็มโป"),
            CalendarDisplayName::new("javanese", "ปฏิทินชวา"),
            CalendarDisplayName::new("juche", "ปฏิทินเกาหลีเหนือ"),
            CalendarDisplayName::new("julian", "ปฏิทินจูเลียส"),
            CalendarDisplayName::new("julian-day", "วันจูเลียส"),
            CalendarDisplayName::new("nanakshahi", "ปฏิทินนานักชาฮี"),
            CalendarDisplayName::new("olympiad", "โอลิมเปียด"),
            CalendarDisplayName::new("roman-auc", "อับอูร์เบกอนดิตา"),
            CalendarDisplayName::new("sexagenary", "แผนภูมิสวรรค์"),
            CalendarDisplayName::new("thai-lunar", "ปฏิทินจันทรคติไทย"),
            CalendarDisplayName::new("tibetan", "ปฏิทินทิเบต"),
        ],
    ),
    (
        "tr",
        &[
            CalendarDisplayName::new("armenian", "Ermeni takvimi"),
            CalendarDisplayName::new("assyrian", "Asur Takvimi"),
            CalendarDisplayName::new("babylonian", "Babil takvimi"),
            CalendarDisplayName::new("bahai", "Bahâî takvimi"),
            CalendarDisplayName::new("berber", "Berberi takvimi"),
            CalendarDisplayName::new("bikram-sambat", "Vikram Samvat"),
            CalendarDisplayName::new("burmese", "Geleneksel Birmanya takvimi"),
            CalendarDisplayName::new("byzantine", "Bizans takvimi"),
            CalendarDisplayName::new("discordian", "Discordian takvimi"),
            CalendarDisplayName::new("french-republican-equinox", "Fransız cumhuriyetçi takvimi"),
            CalendarDisplayName::new("hindu-lunar", "Hindu takvimi"),
            CalendarDisplayName::new("hindu-solar-bengali", "Bengal takvimi"),
            CalendarDisplayName::new("holocene", "Holosen takvimi"),
            CalendarDisplayName::new("jalali", "Celali takvimi"),
            CalendarDisplayName::new("japanese-jokyo", "Jōkyō takvimi"),
            CalendarDisplayName::new("japanese-tenpo", "Tenpō takvimi"),
            CalendarDisplayName::new("juche", "Kuzey Kore takvimi"),
            CalendarDisplayName::new("julian", "Jülyen takvimi"),
            CalendarDisplayName::new("julian-day", "Jülyen günü"),
            CalendarDisplayName::new("kurdish", "Kürt takvimi"),
            CalendarDisplayName::new("masonic-anno-lucis", "Anno Lucis"),
            CalendarDisplayName::new("positivist", "Pozitivist takvim"),
            CalendarDisplayName::new("revised-julian", "Yeniden düzenlenmiş Jülyen takvimi"),
            CalendarDisplayName::new("roman-auc", "Ab urbe condita"),
            CalendarDisplayName::new("rumi", "Rumi takvim"),
            CalendarDisplayName::new("sexagenary", "Altmışlık döngü"),
            CalendarDisplayName::new("swedish-1700", "İsveç takvimi"),
            CalendarDisplayName::new("tibetan", "Tibet takvimi"),
        ],
    ),
    (
        "ur",
        &[
            CalendarDisplayName::new("assyrian", "آشوری تقویم"),
            CalendarDisplayName::new("hindu-lunar", "ہندو تقویم"),
            CalendarDisplayName::new("hindu-solar-bengali", "بنگالی تقویم"),
            CalendarDisplayName::new("julian", "جولین کیلنڈر"),
            CalendarDisplayName::new("nanakshahi", "نانک شاہی تقویم"),
        ],
    ),
    (
        "vi",
        &[
            CalendarDisplayName::new("armenian", "Lịch Armenia"),
            CalendarDisplayName::new("assyrian", "Lịch Assyria"),
            CalendarDisplayName::new("berber", "Lịch Berber"),
            CalendarDisplayName::new("french-republican-equinox", "Lịch Cộng hòa Pháp"),
            CalendarDisplayName::new("hindu-solar-bengali", "Lịch Bengal"),
            CalendarDisplayName::new("holocene", "Lịch Holocen"),
            CalendarDisplayName::new("jalali", "Lịch Jalali"),
            CalendarDisplayName::new("juche", "Lịch Chủ thể"),
            CalendarDisplayName::new("julian", "Lịch Julius"),
            CalendarDisplayName::new("julian-day", "Ngày Julius"),
            CalendarDisplayName::new("kurdish", "Lịch Kurd"),
            CalendarDisplayName::new("olympiad", "Olympiad"),
            CalendarDisplayName::new("roman-auc", "Ab urbe condita"),
            CalendarDisplayName::new("sexagenary", "Can Chi"),
        ],
    ),
    (
        "zgh",
        &[CalendarDisplayName::new("berber", "ⴰⵙⵎⵍⵓⵙⵙⴰⵏ ⴰⵎⴰⵣⵉⵖ")],
    ),
    (
        "zh-Hans",
        &[
            CalendarDisplayName::new("armenian", "亚美尼亚历法"),
            CalendarDisplayName::new("assyrian", "亚述历"),
            CalendarDisplayName::new("aztec-tonalpohualli", "阿兹特克神圣历"),
            CalendarDisplayName::new("aztec-xiuhpohualli", "阿兹特克太阳历"),
            CalendarDisplayName::new("babylonian", "巴比伦历"),
            CalendarDisplayName::new("bahai", "巴哈伊历法"),
            CalendarDisplayName::new("bikram-sambat", "维克拉姆历"),
            CalendarDisplayName::new("burmese", "缅历"),
            CalendarDisplayName::new("byzantine", "拜占庭历"),
            CalendarDisplayName::new("chinese-daming", "大明历"),
            CalendarDisplayName::new("chinese-jingchu", "景初历"),
            CalendarDisplayName::new("chinese-kaihuang", "开皇历"),
            CalendarDisplayName::new("chinese-qianxiang", "乾象历"),
            CalendarDisplayName::new("chinese-sanji", "三纪历"),
            CalendarDisplayName::new("chinese-sifen", "后汉四分历"),
            CalendarDisplayName::new("chinese-taichu", "太初历"),
            CalendarDisplayName::new("chinese-tianhe", "天和历"),
            CalendarDisplayName::new("chinese-xinghe", "兴和历"),
            CalendarDisplayName::new("chinese-yuanjia", "元嘉历"),
            CalendarDisplayName::new("chinese-zhengguang", "正光历"),
            CalendarDisplayName::new("egyptian", "埃及历法"),
            CalendarDisplayName::new("french-republican-equinox", "法国共和历"),
            CalendarDisplayName::new("hanke-henry", "汉克亨利万年历"),
            CalendarDisplayName::new("hindu-lunar", "印度历法"),
            CalendarDisplayName::new("hindu-solar-bengali", "孟加拉历"),
            CalendarDisplayName::new("holocene", "全新世纪年"),
            CalendarDisplayName::new("hongxian", "洪宪"),
            CalendarDisplayName::new("huangdi-era", "黄帝纪元"),
            CalendarDisplayName::new("international-fixed", "国际固定历"),
            CalendarDisplayName::new("japanese-horyaku", "宝历历"),
            CalendarDisplayName::new("japanese-imperial", "皇纪"),
            CalendarDisplayName::new("japanese-jokyo", "贞享历"),
            CalendarDisplayName::new("japanese-senmyo", "宣明历"),
            CalendarDisplayName::new("japanese-tenpo", "天保历"),
            CalendarDisplayName::new("javanese", "爪哇历"),
            CalendarDisplayName::new("juche", "主体年号"),
            CalendarDisplayName::new("julian", "儒略历"),
            CalendarDisplayName::new("julian-day", "儒略日"),
            CalendarDisplayName::new("masonic-anno-lucis", "光明之年"),
            CalendarDisplayName::new("maya-longcount", "长纪历"),
            CalendarDisplayName::new("nepal-sambat", "尼瓦历"),
            CalendarDisplayName::new("olympiad", "奥林匹亚周期"),
            CalendarDisplayName::new("pax", "十三月历"),
            CalendarDisplayName::new("positivist", "实证主义历法"),
            CalendarDisplayName::new("revised-julian", "儒略改革历"),
            CalendarDisplayName::new("roman-auc", "罗马建城纪年"),
            CalendarDisplayName::new("sexagenary", "干支"),
            CalendarDisplayName::new("soviet-week", "苏维埃革命历法"),
            CalendarDisplayName::new("swedish-1700", "瑞典历"),
            CalendarDisplayName::new("thai-lunar", "泰国阴历"),
            CalendarDisplayName::new("vietnamese", "越南历"),
            CalendarDisplayName::new("world-calendar", "世界历"),
        ],
    ),
    (
        "zh-Hant",
        &[
            CalendarDisplayName::new("armenian", "亞美尼亞曆法"),
            CalendarDisplayName::new("assyrian", "亞述曆"),
            CalendarDisplayName::new("aztec-tonalpohualli", "阿茲特克神聖曆"),
            CalendarDisplayName::new("aztec-xiuhpohualli", "阿茲特克太陽曆"),
            CalendarDisplayName::new("babylonian", "巴比倫曆"),
            CalendarDisplayName::new("bahai", "巴哈伊曆法"),
            CalendarDisplayName::new("bikram-sambat", "維克拉姆歷"),
            CalendarDisplayName::new("burmese", "緬曆"),
            CalendarDisplayName::new("byzantine", "拜占庭歷"),
            CalendarDisplayName::new("chinese-daming", "大明曆"),
            CalendarDisplayName::new("chinese-jingchu", "景初曆"),
            CalendarDisplayName::new("chinese-kaihuang", "開皇曆"),
            CalendarDisplayName::new("chinese-qianxiang", "乾象曆"),
            CalendarDisplayName::new("chinese-sanji", "三紀曆"),
            CalendarDisplayName::new("chinese-sifen", "後漢四分曆"),
            CalendarDisplayName::new("chinese-taichu", "太初曆"),
            CalendarDisplayName::new("chinese-tianhe", "天和曆"),
            CalendarDisplayName::new("chinese-xinghe", "興和曆"),
            CalendarDisplayName::new("chinese-yuanjia", "元嘉曆"),
            CalendarDisplayName::new("chinese-zhengguang", "正光曆"),
            CalendarDisplayName::new("egyptian", "埃及曆法"),
            CalendarDisplayName::new("french-republican-equinox", "法國共和曆"),
            CalendarDisplayName::new("hanke-henry", "漢克亨利萬年曆"),
            CalendarDisplayName::new("hindu-lunar", "印度曆法"),
            CalendarDisplayName::new("hindu-solar-bengali", "孟加拉曆"),
            CalendarDisplayName::new("holocene", "全新世紀年"),
            CalendarDisplayName::new("hongxian", "洪憲"),
            CalendarDisplayName::new("huangdi-era", "黃帝紀元"),
            CalendarDisplayName::new("international-fixed", "國際固定曆"),
            CalendarDisplayName::new("japanese-horyaku", "寶曆曆"),
            CalendarDisplayName::new("japanese-imperial", "皇紀"),
            CalendarDisplayName::new("japanese-jokyo", "貞享曆"),
            CalendarDisplayName::new("japanese-senmyo", "宣明曆"),
            CalendarDisplayName::new("japanese-tenpo", "天保曆"),
            CalendarDisplayName::new("javanese", "爪哇曆"),
            CalendarDisplayName::new("juche", "主體年號"),
            CalendarDisplayName::new("julian", "儒略曆"),
            CalendarDisplayName::new("julian-day", "儒略日"),
            CalendarDisplayName::new("masonic-anno-lucis", "光明之年"),
            CalendarDisplayName::new("maya-longcount", "長紀曆"),
            CalendarDisplayName::new("nepal-sambat", "尼瓦曆"),
            CalendarDisplayName::new("olympiad", "奧林匹亞周期"),
            CalendarDisplayName::new("pax", "十三月曆"),
            CalendarDisplayName::new("positivist", "實證主義曆法"),
            CalendarDisplayName::new("revised-julian", "儒略改革曆"),
            CalendarDisplayName::new("roman-auc", "羅馬建城紀年"),
            CalendarDisplayName::new("sexagenary", "干支"),
            CalendarDisplayName::new("soviet-week", "蘇維埃革命曆法"),
            CalendarDisplayName::new("swedish-1700", "瑞典曆"),
            CalendarDisplayName::new("thai-lunar", "泰國陰曆"),
            CalendarDisplayName::new("vietnamese", "越南曆"),
            CalendarDisplayName::new("world-calendar", "世界曆"),
        ],
    ),
];
