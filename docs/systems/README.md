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

## Systems that need a document

These are implemented and their explanation is still only in module
documentation and crate READMEs, which is where the document will be drawn
from. Each is a task of its own; the order is by how far the module's
explanation is from what a reader needs.

| System | Where the explanation is now |
| --- | --- |
| The Thai lunar calendar's year types and the printed Buddhist year | `thai_lunar` and `buddhist` |
| The Burmese calendar's eras and exceptions | `burmese` |
| The Tibetan Phugpa arithmetic | `tibetan` |
| The Hindu calendars: amānta and pūrṇimānta months, the solar months, the nakṣatras, ayanāṃśa | `hc-calendars-indic` |
| Bikram Sambat as gazetted, and Nepal Sambat | `bikram_sambat`, `nepal_sambat` |
| The East Asian lunisolar calendars and their meridian conventions | `lunisolar`, `chinese`, `dangi`, `vietnamese` |
| The Hijri family: tabular schemes, the Umm al-Qura table, the observational prediction | `tabular`, `islamic_umalqura`, `islamic_observational` |
| The Hebrew calendar's dehiyyot | `hebrew` |
| The Maya and Aztec counts | `maya`, `aztec` |
| The Balinese Pawukon and the Javanese pasaran | `balinese_pawukon`, `javanese_pasaran` |
| The Badíʿ calendar and the French Republican equinox rule | `hc-calendars-equinox` |
| The 24 solar terms and 72 pentads, and the zodiac conventions | `hc-seasons` |
| The sexagenary cycle and its year boundaries | `sexagenary` |
| China's annual holiday arrangements and working weekends | `hc-holiday`, China |
| Russia's transferred days off | `hc-holiday`, Russia |
| Korea's substitute-holiday rules and collisions | `hc-holiday`, South Korea, and `engine` |
| Japan's holiday law and its amendments | `hc-holiday`, Japan |
| The Gregorian reform, country by country | `julian_gregorian` |
