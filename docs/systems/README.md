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
| China's annual holiday arrangements and working weekends | [china-holiday-arrangements.md](china-holiday-arrangements.md) | `hc-holiday`'s `CHINA`; `XSHG`, `XSHE` |
| The Hijri calendars: the tabular schemes, the Umm al-Qura table and the observational prediction | [hijri.md](hijri.md) | `islamic-civil`, `islamic-tbla`, `islamic-fatimid`, `islamic-umalqura`, `islamic-rgsa`; `tabular` |
| The Hindu calendars: amānta and pūrṇimānta months, the solar months, the nakṣatras, ayanāṃśa | [hindu-calendars.md](hindu-calendars.md) | `hindu-lunar`, `hindu-lunar-purnimanta`, `hindu-solar-tamil`, `hindu-solar-malayalam`, `hindu-solar-bengali`, `hindu-solar-vikrami`, `hindu-old-solar`, `hindu-old-lunar`; `tithi`, `nakshatra`, `surya_siddhanta` |
| The Maya and Aztec counts: the Long Count under two correlations, the Tzolkʼin, Haabʼ and Calendar Round, the tonalpohualli and xiuhpohualli | [mesoamerican-counts.md](mesoamerican-counts.md) | `maya-longcount`, `maya-longcount-gmt2`, `maya-tzolkin`, `maya-haab`, `maya-round`, `maya-tzolkin-gmt2`, `maya-haab-gmt2`, `maya-round-gmt2`, `aztec-tonalpohualli`, `aztec-xiuhpohualli` |
| The East Asian lunisolar calendars: China, Korea and Vietnam on their meridians | [east-asian-lunisolar.md](east-asian-lunisolar.md) | `chinese`, `dangi`, `vietnamese`; `lunisolar` |
| South Korea's public holidays and the substitute holiday, with the collision rule | [korea-holidays.md](korea-holidays.md) | `hc-holiday`'s `SOUTH_KOREA` and `engine::collisions`; `XKRX` |
| Russia's public holidays and the transfers of days off | [russia-transfers.md](russia-transfers.md) | `hc-holiday`'s `RUSSIA`; `MISX` |
| Japan's holiday law and its amendments | [japan-holidays.md](japan-holidays.md) | `hc-holiday`'s `JAPAN`; `XJPX` |
| The 24 solar terms and 72 pentads, and the zodiac conventions: 定気, the meridian, the ayanāṃśa | [solar-terms-and-pentads.md](solar-terms-and-pentads.md) | `hc-seasons`: `solar_terms`, `pentads`, `meridian`, `zodiac` |
| The Berber agrarian calendar and the Amazigh era | [berber.md](berber.md) | `berber` |
| The Mandaean calendar: the Parwanaia and the years after Adam | [mandaean.md](mandaean.md) | `mandaean` |
| The modern Assyrian calendar and its 4750 BC epoch | [assyrian.md](assyrian.md) | `assyrian` |
| The Yazidi year: Serêsal and the Eastern calendar | [yazidi.md](yazidi.md) | `yazidi` |
| Nepal's calendars: the Bikram Sambat as gazetted, and Nepal Sambat | [nepal-calendars.md](nepal-calendars.md) | `bikram-sambat`, `nepal-sambat` |
| The Hebrew calendar: the molad, the nineteen-year cycle and the four dehiyyot | [hebrew.md](hebrew.md) | `hebrew` |
| The Gregorian reform, country by country, and the Swedish exception | [gregorian-reform.md](gregorian-reform.md) | `julian-gregorian-<polity>`, twelve of them; `swedish-1700` |
| The Burmese calendar: the eras of the Myanmar Era, watat and yat-ngyin, the record's exceptions as data | [burmese.md](burmese.md) | `burmese`; `hc-holiday`'s Thingyan days |
| The Tibetan calendar: the Phugpa arithmetic, the lunar day with its skipped and extra days, the leap-month rule, the sixty-year names | [tibetan-phugpa.md](tibetan-phugpa.md) | `tibetan` |
| The Milankovitch orbital elements and daily insolation, from Berger's 1978 series | [orbital-elements.md](orbital-elements.md) | `hc-orbital` |

## Systems that need a document

These are implemented and their explanation is still only in module
documentation and crate READMEs, which is where the document will be drawn
from. Each is a task of its own; the order is by how far the module's
explanation is from what a reader needs.

| System | Where the explanation is now |
| --- | --- |
| The Burmese calendar's eras and exceptions | `burmese` |
| The Tibetan Phugpa arithmetic | `tibetan` |
| Bikram Sambat as gazetted, and Nepal Sambat | `bikram_sambat`, `nepal_sambat` |
| The Hebrew calendar's dehiyyot | `hebrew` |
| The Balinese Pawukon and the Javanese pasaran | `balinese_pawukon`, `javanese_pasaran` |
| The Badíʿ calendar and the French Republican equinox rule | `hc-calendars-equinox` |
| The sexagenary cycle and its year boundaries | `sexagenary` |
