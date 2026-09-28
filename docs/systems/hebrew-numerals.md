# Hebrew numerals

Backs `hc-i18n`'s numbering system `hebr`, the Hebrew calendar's dates in
Hebrew (`he`), and the reading of those dates by `label::parse_date`.

## What it is

Hebrew writes numbers with the letters of its alphabet, each letter
standing for a value that is added to the others' [wikipedia-hebrew-numerals].
Arabic numerals are the rule in modern print, and the letters are kept for
a few purposes. The first of these that the source names is "writing the
days and years of the Hebrew calendar". A Hebrew calendar printed in Hebrew
writes 15 Adar 5764 as ט״ו באדר ה׳תשס״ד, or, more often, without the
thousands, as ט״ו באדר תשס״ד.

CLDR carries the system as the numbering system `hebr`, of the algorithmic
type, whose rules are the `hebrew` rule set [cldr48-supplemental]. The
Hebrew locale writes the four patterns of its Hebrew-calendar date with
it: in `he.xml`, the full pattern "EEEE, d בMMMM y" and the long, medium
and short "d בMMMM y" carry `numbers="hebr"`. Its `availableFormats`
items, the year alone and the month and day among them, carry no
`numbers` attribute [cldr48-calendar-dates].

## How it works

The letters of 1 to 9 are א ב ג ד ה ו ז ח ט, of 10 to 90 י כ ל מ נ ס ע פ צ,
and of 100 to 400 ק ר ש ת. The larger hundreds are sums: ת״ק is 500 and תתק
is 900. The values add, largest first, so קעז is 100 + 70 + 7 = 177
[wikipedia-hebrew-numerals].

Two marks show that the letters are a number and not a word:

- a **geresh** ׳ (U+05F3) after a single letter, ק׳;
- a **gershayim** ״ (U+05F4) before the last letter of two or more, כ״ח.

The rules have exceptions, which CLDR's rule-based number format spells
out in `common/rbnf/root.xml`, in its `%hebrew`, `%%hebrew-0-99`,
`%%hebrew-thousands` and `%hebrew-item` rules [cldr48-rbnf]:

- 15 and 16 are ט״ו (9 + 6) and ט״ז (9 + 7). Ten and five, and ten and six,
  are forms of the Name of God, a convention of the Middle Ages; before it
  they were written י״ה and י״ו [wikipedia-hebrew-numerals].
- A few numbers change the order of their letters so as not to spell a
  word: 298 is רח״צ, 304 ד״ש, 344 שד״מ, 698 תרח״צ and 744 תשד״מ. The source
  gives 744's reason: תשמ״ד would mean "you will be destroyed".
- A hundred with nothing after it takes the geresh, ק׳. A round ten after
  the hundreds takes the gershayim before it, תש״ע for 770. Eighty is then
  the final form of its letter, תש״ף.
- The thousands are their number, marked with the geresh, before the rest:
  ה׳תשפ״ז is 5 × 1000 + 787. A thousand, two thousand and three thousand
  are the words אלף, אלפיים and ג׳ אלפים. Nothing marks where the thousands
  end, "which can theoretically lead to ambiguity"
  [wikipedia-hebrew-numerals]. The rules write 5000 as ה׳, which is also
  how they write 5.

**The millennium.** "When specifying years of the Hebrew calendar in the
present millennium, writers usually omit the thousands (which is presently
5 [ה])", and the sentence goes on: "but if they do not, this is accepted
to mean 5,000, with no ambiguity" [wikipedia-hebrew-numerals]. "This" can
be read two ways: the thousands, when a writer does write them, are taken
as 5 000 and not as the letter's 5; or a year the writer leaves them off
is taken to be of the millennium of 5 000. The library reads a year
written with its thousands as written, and one written without them by
the first half of the sentence, as of the present millennium. The source's own example of the
common usage is יום חמישי ג׳ בניסן תשס״ז: Thursday, 3 Nisan (5)767. A
tombstone of 1935 it shows writes the year 695 with לפ״ק after it,
"without the thousands". "Presently 5" holds until AM 6000, which begins
in 2239 CE; the source says nothing of another millennium.

**Typed marks.** Most keyboards have neither mark. A reader types an
apostrophe ' (U+0027) for the geresh and a quotation mark " (U+0022) for
the gershayim [wikipedia-geresh] [wikipedia-gershayim]. UTS #35's loose
matching names the first substitution: "U+05F3 HEBREW PUNCTUATION GERESH
… might be typed instead as U+0027 APOSTROPHE" [uts35-v48].

**Worked example.** 28 September 2026 is 17 Tishri 5787.

- The day, 17, is ten and seven: י and ז, with the gershayim before the
  last letter, י״ז.
- The year, 5787, is five thousands and 787.
  - The five is ה with the geresh, ה׳.
  - 787 is 400 + 300 + 80 + 7: the hundreds תש, then eighty-seven as פ, the
    gershayim, and ז.
  - Together, תשפ״ז.
- CLDR's pattern puts ב before the month: י״ז בתשרי ה׳תשפ״ז.
- Written in common usage, the date is י״ז בתשרי תשפ״ז. Typed, it is
  י"ז בתשרי תשפ"ז.

## What is carried

- `hebr` in `hc_i18n::numbering`. It writes 1 to 9 999 by CLDR's rules,
  which covers every day of a month and every year the Hebrew calendars
  convert (`hebrew` stops at AM 9 999). It refuses zero, a negative number
  and anything from 10 000 up. The system has no zero, and CLDR's rules for
  larger numbers are not needed by a date.
- It reads a number back only as the rules write it, with a typed mark in
  place of either mark: י״ז and י"ז are 17, while the older י״ה and a
  reordered ז״י are refused. It reads ה׳ as 5, the smaller of the two
  numbers the rules write so.
- `hc_i18n::numbering::same_typed_mark`, the rule that a typed apostrophe
  or quotation mark matches a geresh or gershayim. The date reader uses it
  for names too: אדר א' is אדר א׳.
- The Hebrew calendar in `he` (`hebrew` and `hebrew-observational`). The
  day and the year are written in Hebrew numerals, as CLDR's pattern does:
  י״ז בתשרי ה׳תשפ״ז. The year keeps its thousands, since CLDR's rules write
  them. The year is written in the locale's digits instead where its
  numerals would not read back as itself, or where the whole date would
  not read back as its day:
  - a year before AM 1000, whose letters are those of a year of the sixth
    millennium without its thousands: the first day, 1 Tishri AM 1, is
    written א׳ בתשרי 1, not א׳ בתשרי א׳, which reads as AM 5001. The
    years 1 to 99 so written are then refused as two-digit years
    ([written-dates.md](written-dates.md)), and the years from 100 read
    back;
  - a round thousand from AM 4000, whose numerals ה׳ are also 5;
  - any date whose text, read back, names another day. The renderer reads
    each date it writes so before writing it. With the reading rules
    below, no date of AM 1000 to 9 998 is written in digits on this
    account: of every seventh day, about 470 000 dates, counted on
    2026-09-28, only the 311 of the round thousands were.

  This fallback is the library's choice, made to keep the reading exact;
  no source writes such years.
- Reading the millennium. `DateTemplates::omitted_thousands` states it for
  the Hebrew date in `he`: 5 000. A year read in Hebrew numerals with fewer
  than four places is taken to be of that millennium, so תשפ״ז is 5787. A
  year written with its thousands, ה׳תשפ״ז, or in digits, 5787, is read as
  written. The rule is the present millennium's, and it is applied to
  every year the calendar converts: תשפ״ז is never read as 6787 or 4787.
- A run of letters each with its geresh is one numeral, and so is a
  letter with its geresh followed at once by another letter: the
  thousands and the rest. The reader ends neither a number nor a name
  inside one. 1 Tishri 5002 is א׳ בתשרי ה׳ב׳, 5002, and not the year 5005
  followed by the narrow weekday ב׳ (Monday), which Hebrew writes with the
  same letter and geresh; 1 Adar 1011 is א׳ באדר א׳י״א, Adar and the year
  1011, and not אדר א׳, Adar I, and the year 11. A weekday after the
  year needs a space: א׳ בתשרי ה׳ ב׳ is Monday, 1 Tishri 5005.

Not carried:

- Dots over the letters in place of the marks, as on the tombstone.
- The final forms of the letters for 500 to 900, which the source says
  "was not widely accepted and soon abandoned".
- A date with לפ״ק after its year.
- Hebrew numerals in any other locale or calendar. A caller can still ask
  for them with `-u-nu-hebr`.

## Accuracy

`hc_i18n`'s tests write the source's numbers and read them back:

- ט״ו and ט״ז;
- תשד״מ and תש״ף;
- the years ה׳תש״ס, ה׳תשס״ד, ה׳תש״ע and ה׳תשפ״ה from the source's table of
  recent years, with ה׳תשפ״ז.

They also write every number from 1 to 9 999 and read it back, with the
marks as written and as typed. The only numbers that do not come back as
themselves are the round thousands from 4 000, which come back as their
number of thousands.

The date reader's tests read the source's two dated examples, in full and
in common usage: יום שני ט״ו באדר ה׳תשס״ד and תשס״ד (Monday, 8 March
2004), and יום חמישי ג׳ בניסן ה׳תשס״ז and תשס״ז (Thursday, 22 March 2007).
Each weekday is checked against the day. The sweep of every calendar
([written-dates.md](written-dates.md)) formats the Hebrew calendars in
`he` on every sample day and reads them back, and `hebrew` also on 1
Tishri of AM 1003, 5002, 5300 and 9001, whose years end in a letter with
its geresh.

Every day of the 63 years AM k × 1000 + 1 to 6 and k × 1000 + 300, for k
from 1 to 9, 23 007 days in all, whose years end as a narrow weekday is
written, was written in `he` and read back as itself on 2026-09-28; and 1
Tishri of every year from AM 100 to 9 998.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [cldr48-rbnf] | The `%hebrew` rules: the marks, 15 and 16, the reordered hundreds, the thousands, the words for 1 000 to 3 000 | Yes, 2026-09-28, `common/rbnf/root.xml` of the release-48 tag |
| [cldr48-supplemental] | `numberingSystems.xml`: `hebr` is algorithmic, with the rules `hebrew` | Yes, 2026-09-28, that element |
| [cldr48-calendar-dates] | `he.xml`, `calendar type="hebrew"`: the full pattern "EEEE, d בMMMM y" and the long, medium and short "d בMMMM y" with `numbers="hebr"`; no `numbers` on the `availableFormats` items | Yes, 2026-09-28, that calendar's `dateFormats` and `availableFormats` |
| [wikipedia-hebrew-numerals] | The letters' values; 15 and 16; 744; the thousands omitted in the present millennium, "presently 5"; the dated examples and the table of recent years | Yes, 2026-09-28, the article's wikitext |
| [wikipedia-geresh], [wikipedia-gershayim] | An apostrophe typed for the geresh and a quotation mark for the gershayim | Yes, 2026-09-28 |
| [uts35-v48] | Loose matching: the geresh "might be typed instead as" an apostrophe | Yes, 2026-09-28, "Lenient Parsing" in the release-48 source, `docs/ldml/tr35.md` |

## Code

- `crates/hc-i18n/src/numbering/hebrew.rs`: the writer, the reader and the
  typed marks. The system is `hebr` in `numbering::ALL`.
- `hc_i18n::names::DateTemplates::omitted_thousands`, and the template
  `{day:hebr} ב{month} {year:hebr}` of `he`'s Hebrew calendar in
  `crates/hc-i18n/src/data.rs`.
- `hc-format`'s renderer: `write_in` falls back to digits for a year that
  would not read back, and `write_date` for a date that would not
  (`Renderer::reads_back`). Its reader, `label/read.rs`, reads the
  numbering systems a template names, applies the millennium, keeps a
  numeral's run whole (`NumberingSystem::continues_into`), and matches
  typed marks.
- Tests:
  - `numbering`'s `hebrew_numerals_are_written_as_cldr_spells_them` and
    `hebrew_numerals_round_trip_and_read_typed_marks`;
  - `crates/hyper-calendar/tests/written_dates.rs`'s
    `hebrew_dates_read_in_hebrew_numerals`.
