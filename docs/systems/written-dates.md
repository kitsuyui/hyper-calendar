# Written dates, read back

Backs `hc-format`'s `label::parse_date`, the facade's
`lines::parse_date` and the export `hc_parse_date`: a date as a locale
writes it in one calendar, read back as the calendar's fields and the
fixed day, or refused with the reason.

## What it is

A date a reader types, pastes or copies from a document is text:
令和8年9月28日, 康熙五十二年十一月初一, *28 Eylül 2026*, ٢٨ سبتمبر ٢٠٢٦. The
library writes such text for every registered calendar in every carried
locale (`label::write_date`, which `hc_describe_day`'s formatted column
uses), from the vocabulary and date templates of `hc-i18n`. Reading it
back is the inverse, and it is harder in one way that matters here: the
text often does not say everything the calendar needs, and a guess would
return a day with confidence the text does not support.

UTS #35, Part 4, "Parsing Dates and Times" [uts35-dates-48], describes the
problem for CLDR's patterns. It splits the fields into numeric and symbolic
ones, and advises four things:

- try the locale's own pattern first;
- check symbols first, so that the Japanese 3月 is read as a month and not
  as a number and a literal;
- accept a name at every width, standalone as well as in a date, with or
  without its abbreviation period, in either case, where it is unique;
- warn that a date written with narrow names may read back as another
  date.

It also notes that `yy` keeps "just the two low-order digits of the
year". The advice for text that does not fit the pattern is heuristic
resynchronisation. This reader follows the first three points and not
the heuristics: §4 of the [policy](../policy.md) says the library refuses
to guess.

## How it works

The reader walks the same templates the renderer fills. For each placeholder
it tries everything the renderer could have written there, then keeps a
match only if the calendar agrees with all of it.

1. **Templates.** A locale states how it writes a calendar's dates as a
   [`TemplateChain`](../../crates/hc-i18n/src/names.rs):
   - the entry's templates, then the calendar's notation, then the
     locale's, then the default;
   - for each, a date template, and within it the year's (with the
     first-year template, `{era}元年`) and the day's.

   The reader tries each distinct date template, and inside it each year
   and day template.
2. **Placeholders.** At a placeholder the reader tries every candidate.
   - **Eras:** every era name in the locale's fallback chain, English's and
     the calendar's own. The calendar's own come through the new
     `DynCalendar::era_code`, which lists the codes `era_name` answers
     for, such as the 248 nengō.
   - **Months:** every month label at every width and in both contexts,
     in a year with and without the intercalary month, as the renderer
     writes it: the name, the calendar's own name, or the number, with the
     leap prefix. A label with Latin digits, 9月, is also tried with its
     number in the reader's other numbering systems, 九月.
   - **Days and extra fields:** the locale's day names, named cycle values
     (the Pawukon's days, a Tamil year name) and the sexagenary pairs.
   - **Numbers:** read in the locale's digits and in Latin digits, in a
     locale written in Han characters also as Han numerals and positional
     Han digits, and in any system a template names, `{year:hebr}`. A
     numeral counts only if the system writes that value the same way, so
     二〇二六 is not read as six. A year written in numerals with fewer than
     four places takes the thousands a template lets it leave out: תשפ״ז is
     5787 ([hebrew-numerals.md](hebrew-numerals.md)). Neither a number nor
     a name ends inside a Hebrew numeral's run, a letter with its geresh
     followed at once by another letter: ה׳ב׳ is the year 5002, and אדר
     א׳י״א is Adar and the year 1011.
   - **An absent field:** the renderer may write nothing, so the reader
     also tries matching nothing. A date's year is tried as absent too,
     so 9月28日 is read, and refused as not saying which year.

   Literal text matches as the renderer's space-collapsing sink leaves it.
   White space covers any run of it, and a separator (`,` `、` `،`) may be
   missing. A weekday name may come before or after the date. Beside a
   number it needs white space or a separator, since a narrow Hebrew
   weekday, ב׳, is written as a numeral is; beside a word or a template's
   text it may follow directly, 2026年9月28日月曜日.
3. **Resolution.** A complete match gives fields. The reader converts them
   to a fixed day and back, and keeps the day only if all of these hold:
   - the calendar's own fields for that day agree with everything the text
     wrote, including the era, the year, the month and its leap flag, the
     day, every extra field, and the kind of year a month name implies
     (*Adar II* only in a leap year);
   - the renderer, given those fields, would have taken the same template
     levels;
   - a weekday the text names is the day's.

   Where the text leaves something out, the reader fills it in and
   checks each option:
   - a leap day the locale names, St. Tib's Day, which stands for its
     month and day; where the locale names one, a day written by its
     number is not the leap day;
   - an implied era, `ad` or `am`, or the calendar's only era, where the
     locale's and English's era codes for it and its own name one and no
     other: ۶ مهر ۱۴۰۵ is in `ap`, the Solar Hijri era;
   - a leap day;
   - a day that the extra fields determine (the Burmese and Thai lunar
     half and day);
   - a year written only through extra fields: an era's year that
     shifts by a constant, a cycle and a year of the cycle, as Meyer–Palmen
     and the Liberalia Triday write it, measured on three probe days
     centuries apart.
4. **Answer.** One day is the answer. Two or more days are
   `ambiguous`, and the two earliest are named. The reader also tries the
   day before and after when the text writes no day, and flips a flag the
   text does not write.

**Worked example.** 令和元年5月1日 in `ja`, calendar `japanese`:

- The entry states the date template `{year}{month}{day}`, the year
  template `{era}{year}年`, the first-year template `{era}元年` and the day
  template `{day}日`.
- The first-year template matches 令和 as the era `reiwa`, from `ja`'s
  era names, and 元年 as its literal, so the year is 1.
- The month label 5月, `ja`'s wide name for month 5, matches next.
- The day template reads the number 1 and the literal 日.
- The fields reiwa 1-5-1 convert to fixed day 737 180 and back to the
  same fields.
- The renderer, given them, writes the year with the first-year template,
  so the reading is kept: 1 May 2019.

令和1年5月1日 reads as the same day through the year template, and so do
平成1年1月8日 and 明治1年9月8日: the renderer would not write them, but they
say nothing the calendar contradicts.

### The refusals

| Refusal | Code | When | Example |
| --- | ---: | --- | --- |
| `empty` | 101 | the text is white space | |
| `not-recognised` | 102 | no template matches; the offset is where the furthest match stopped | *September 28, 2026 at noon* |
| `ambiguous` | 103 | two days read | 2026w39, a Stata week; *3. März 1 Man'en*, since 1860 has a leap third month that German, which has no word for it, writes as the ordinary one |
| `two-digit-year` | 104 | the year has one or two digits, no sign and no era, the calendar's years have three digits or more on its sample day, and it has the same month and day a hundred or four hundred years on | *September 28, 26*; *September 28, 0026* reads as the year 26 |
| `year-not-written` | 105 | the year is not written, or only by a cycle that recurs, and two probe years give different days | 癸卯年闰二月初一; a Tzolkʼin day; 9月28日, *September 28* |
| `weekday-mismatch` | 106 | the weekday is not the day's | *Tuesday, September 28, 2026* |
| `field-mismatch` | 107 | the text writes a year, a month and a day, and another field that the day with them does not have | 2025丙午年八月十八, since 2025 is 乙巳; 2026년(을사년) 8월 18일 |
| the calendar's | 1–11 | the fields read are not a day of the calendar | *February 30, 2026*, `day-out-of-range` |

A text that leaves out its year is refused as `year-not-written` only
where no reading with a year came nearer. *September 28* is also September
of the year 28 without a day, which the calendar refuses as
`missing-field`. That refusal yields to the reading without the year,
since the text left the day empty and not the year. A refusal for fields
the renderer would not write through the template the text matched does
not count at all. ۶ مهر read by the default template as the year 6 without
a day is such a case.

The field mismatch is reported only where the text writes the year, or
fields that count it. Two fields that contradict each other name no day,
and the reader does not choose which one to believe. A year the text does
not write is instead tried on two probe years, and those years' fields do
not match what the text wrote. That is `year-not-written`, not a
contradiction.

An era name that two of a calendar's eras share is tried as each, and
refused as `ambiguous` only if both read as a day. `chinese-regnal`'s
table names 洪武 twice, `hongwu` and `hongwu-1402`, but both are before
the calendar's range of 1645–1912, so no date it converts meets the case.
The calendar is named by the caller, so a name another calendar also uses
(*AD* in `julian` and `gregory`, 民國 only in `roc`) is not ambiguous at
all.

## What is carried

- Every registered calendar, in every carried locale and under `native`.
  The locale is resolved as a rendered cell resolves it
  (`label::locale_for`), so what `hc_describe_day` writes, `hc_parse_date`
  reads.
- The lenient readings UTS #35 lists:
  - every width and both contexts;
  - either case;
  - an abbreviation with its period where the data has none (*Sep.*);
  - extra or missing white space;
  - apostrophes as UTS #35 Part 1's "Loose Matching" lists them
    [uts35-v48]: U+2019 ’, U+02BC ʼ and U+0027 ' as one, U+02BB ʻ and
    U+2018 ‘ as another, so *Man’en*, as CLDR's root writes the era, reads
    as typed, *Man'en*. CLDR 48's `supplemental/characters.xml` has no
    fallback for these, only the geresh's.
- Latin digits in a locale with its own. In a locale written in Han
  characters, Han numerals, positional Han digits, 元 for the first year
  of an era, and the Chinese day names 初一 … 三十 for a lunisolar
  calendar whose data names none.
- The Hebrew calendar in Hebrew, in Hebrew numerals as CLDR 48 `he.xml`
  writes it, `numbers="hebr"` [cldr48-calendar-dates]. The formatter
  writes the thousands, י״ז בתשרי ה׳תשפ״ז. The reader also reads them left
  out, י״ז בתשרי תשפ״ז, in the millennium Wikipedia gives as the present
  one [wikipedia-hebrew-numerals]; with an apostrophe and a quotation mark
  for the geresh and gershayim, י"ז; and in digits, 17 בתשרי 5787. Names
  match typed marks too, אדר א'. The rules, their exceptions and the
  years written in digits instead are in
  [hebrew-numerals.md](hebrew-numerals.md).
- An era left out of a calendar that has only one: ۶ مهر ۱۴۰۵. An era left
  out of a calendar with two is refused. *Mäskäräm 18, 2019* names no era
  of the Ethiopic calendar's two.
- The Chinese and Dangi years by the related Gregorian year and the stem
  and branch. These are the `y` items of CLDR 48 `zh.xml`, `zh_Hant.xml`,
  `yue.xml` and `yue_Hans.xml`, "rU年", and of `ko.xml`, "r년(U년)", and
  the long date of the first two, "rU年MMMd" [cldr48-calendar-dates].
  They read 2026丙午年八月十八 and 2026년(병오년) 8월 18일. The two must
  agree, or the text is refused as `field-mismatch`. English writes the
  year so already, *Eighth Month 18, 2026(bing-wu)*. The Dangi date in
  `zh_Hant.xml`, `yue.xml` and `yue_Hans.xml` is those files' own
  `calendar type="dangi"`, whose long date "U年MMMd日" writes the stem and
  branch alone and the day in digits: 丙午年八月18日, `year-not-written`.
  CLDR 48 has no `vietnamese` calendar in any of these files, so the
  Vietnamese date takes the Chinese calendar's templates by this library's
  choice: the calendar shares the Chinese months and sexagenary count.
- A weekday before or after the date, which must be the day's.
- A leap month as the locale writes it, with the word the renderer puts
  before or after it (`hc_i18n::names::leap_month_prefix` and
  `leap_month_suffix`). The Japanese era calendars take their months from
  the Gregorian entry, which has no leap month, and the word from an entry
  of their own, with each locale's CLDR name for the era
  ([japanese-eras.md](japanese-eras.md)):
  - 万延元年閏3月3日 in `ja`, as Wikipedia (ja) dates the reunion of the
    courts, 元中9年閏10月5日 [wikipedia-ja-genchu], and tabulates 万延元年's
    閏三月 [wikipedia-ja-manen];
  - 萬延1年閏3月3日 in `zh-Hant` and 万延 (1860–1861)1年闰3月3日 in `zh-Hans`,
    as Wikipedia (zh) dates it, 明德3年閏10月5日 [wikipedia-zh-go-komatsu].
    The era name with its years is CLDR's: `zh.xml` states era 228 of
    `calendar type="japanese"` as "万延 (1860–1861)" in `eraAbbr`, the
    width its pattern's G asks for, and states no other form without the
    years, in `eraNames`, `eraNarrow` or an `alt` variant. Its pattern,
    "Gy年M月d日", puts no space between the era and the year, so neither
    does the date. `en`, `ko`, `hi`, `ru`, `th`, `id`, `nl`, `ar` and `cs`
    likewise state their older eras with their years, and their patterns'
    spaces are kept: *intercalary March 3, 1 Man’en (1860–1861)*. The
    reader also reads the era without its years, by the calendar's own
    name: 万延1年闰3月3日. A romanisation the calendar's own table gives two
    eras, 延慶 and 延享 both *Enkyo*, is not written in a Latin-script
    locale with no name of its own, since the text would name both: German
    writes English's names, which CLDR's years tell apart, *Enkyō
    (1308–1311)*, and where English has none, the native name, 貞和 of the
    Northern court beside 承和, both *Jowa* (`label::era_label`);
  - the same in `yue-Hant` and `yue-Hans`, 萬延1年閏3月3日 and
    万延1年闰3月3日, and in `ko` and `vi`, 만엔 (1860 ~ 1861) 1년 윤3월 3일 and
    *3 tháng 3 Nhuận, 1 Man'en*, each with the leap pattern its CLDR file
    gives the Chinese calendar, 閏{0}, 闰{0}, 윤{0} and {0} Nhuận
    [cldr48-japanese-eras]. CLDR has no leap pattern for the Japanese
    calendar, so the rule is this library's: every locale whose file has
    one for the Chinese calendar borrows it;
  - *intercalary March 3, 1 Man’en (1860–1861)* in `en`, the word of the
    National Diet Library's "Calendar History", "Uru-zuki," or
    intercalary month" [ndl-koyomi-history], for the month "called by the
    name of the preceding month" [bramsen1910].

  The Chinese family's numbered months take the same English word,
  *intercalary Sixth Month*, as Henning writes "an extra, or intercalary,
  month" for the Tibetan [kalacakra-org]; no source read writes a Chinese
  leap month in an English date, so its place before the month is this
  library's. Tibetan writes ཟླ་ཤོལ་ before the month, ཟླ་ཤོལ་ཟླ་བ་དགུ་པ,
  Henning's *zla shol*; where an almanac puts the word was not read, so its
  place before the month is this library's too. The other locales write
  the Japanese leap month as the ordinary one, and a date in it is
  `ambiguous` there.
- A leap unit a source names, written by that name:
  - St. Tib's Day, the Discordian day "inserted between the 59th and 60th
    days of the Season of Chaos" [principia-discordia-p34], in place of
    its season and day: *St. Tib's Day, 3190 YOLD*, 29 February 2024. The
    source writes no date with it, so the rest of the date is this
    library's; a locale with no name of its own takes the English one, as
    it takes the era's;
  - Nepal Sambat's intercalary month, Analā in English and अनला in `ne`,
    "The intercalary month" of Wikipedia's table of months, whichever
    month it doubles [wikipedia-nepal-sambat]: *Analā 1, 1111 NS*;
  - the Lao later eighth month, ເດືອນແປດຫລັງ, the regular month 8 of a
    year with two, as Lao Wikipedia writes a date in it, "ວັນແຮມ 1 ຄ່ຳ
    ເດືອນແປດຫລັງ" [wikipedia-lo-khao-phansa]. No source read names the
    extra month before it, which keeps ເດືອນແປດ.
- The Chinese regnal calendar in Chinese in Han numerals, the year with
  元年 for the first: 康熙五十二年十一月一日, 康熙元年正月一日. GB/T
  15835-2011 §4.2.1 prescribes Han numerals for 历史朝代纪年 and 农历月日,
  "清咸丰十年九月二十日" [gb-t-15835-2011], and Wikipedia (zh) writes the
  reign's dates so in both scripts, 康熙六十一年十一月十三日, with 康熙元年
  in its table [wikipedia-zh-kangxi]. The template names the system,
  `{year:hans}`, since the locale writes its other dates in Latin digits.
  The reader still reads 康熙52年十一月1日 and 康熙五十二年十一月初一.
  The library applies GB/T 15835-2011, a standard of the People's
  Republic, to this calendar in `zh-Hans` only; the `zh-Hant` template
  rests on Wikipedia's traditional rendering. The same clause lists
  日本庆应三年, a Japanese era's year in Han numerals, yet `zh-Hans`
  writes the Japanese era calendars by CLDR's "Gy年M月d日", in digits,
  万延 (1860–1861)1年: the locale's CLDR pattern is kept for them, by this
  library's choice.

Not carried:

- Unicode normalisation: the reader compares code points, so a
  decomposed *ü* does not match a composed one. This needs tables the
  workspace does not have (policy §9).
- Prefix matches of names (*Sept*, *Se*).
- Dates in a pattern other than the locale's own, such as *09/05/02*.
  UTS #35 resolves these by heuristics, and the reader does not guess.
- Times of day and zones.
- The Chinese calendar's year in Japanese, which CLDR 48 `ja.xml` writes
  by its stem and branch alone, "U年", and the Korean long date, "U년 MMM
  d일", which does the same: `year-not-written`. The Cantonese long date,
  "U (r) 年MMMd", which puts the related year after the cyclic one, is not
  written or read.
- Hebrew numerals with dots over the letters, or with לפ״ק after the year.

## Accuracy

`crates/hyper-calendar/tests/written_dates.rs` formats and reads back every
registered calendar. Each is formatted in every carried locale, the root
and `native`, on its sample days, where the calendar converts them: 1
January 1 CE, 15 October 1582, 15 June 1900, 1 January 1970, 1 January and
28 September 2026 and 22 December 2100; its first and last days; and the
days the test's `MORE_DAYS` lists for it, the doubled days below and four
Hebrew New Years whose years end in a letter with its geresh.

In a release build the sweep reads 211 calendars × 55 locale settings, on
1 729 calendar-days, which is 95 095 texts:

| Outcome | Texts |
| --- | ---: |
| Read back as the day written | 83 725 |
| `year-not-written`: cycles that recur, see below | 6 538 |
| `missing-field`: the 819-day count's station is not written | 1 155 |
| `two-digit-year`: years 0–99, on the calendars' first days | 1 092 |
| `ambiguous`: see below | 2 585 |
| Read as a wrong day | 0 |

Counted 2026-09-29 by a program that repeats the release sweep.

A second count, of 2026-09-29, writes and reads the first day of every
era of every calendar in every locale setting, found where the era of a
day changes, and the first day of every month over four years from 2026:
1 456 era days and 8 771 month days, 562 375 texts. Every text that does
not read back is one of the refusals listed below, or a Japanese era
calendar's leap month in a locale with no word for it, which that locale
writes as the ordinary month (the ambiguity above). A romanisation two eras
share would be a third kind, which is why it is not written.

The `year-not-written` cases are all cycles that recur:

- the Akan, tonalpohualli, Pawukon, pasaran, Haabʼ, Tzolkʼin and Calendar
  Round counts;
- the year bearers of `mixtec-year` and `zapotec-yza`;
- the sexagenary day;
- the sexagenary year of `chinese`, `dangi` and `vietnamese` in `ja`,
  which writes the year by it alone, and of `dangi` in `zh-Hant`,
  `yue-Hant` and `yue-Hans`, whose CLDR files write it so.

The `ambiguous` cases are all texts that name more than one day. Apart
from `stata-week`, whose date is a week, they are calendars that give two
days one date and whose sources write no mark that tells them apart:

- `fasli-bombay` and `sur-san`, a doubled day in early June, 7 June in
  1993 and 1994, both 7 June of Faṣlī 1403. The doubled day is this
  library's: the year keeps Hijri months that are not carried, and the
  Gregorian day under the year is the library's choice
  ([indian-eras.md](indian-eras.md)). No source writes a date in it, so
  none marks the second day, and the text is refused;
- `tibetan`, `tibetan-tsurphu`, `tibetan-bhutan`, `mongolian`,
  `tibetan-lochen` and `tibetan-tsurphu-karana`, a doubled lunar day.
  Janson says the first of two days with the same number "is regarded as a leap day, and denoted 'Extra' in the almanacs"
  [janson2014, §6], and gives no date written with it. Henning's archive
  of the Bhutanese calendar writes the two days as two entries of one
  number, each with its Western date, and no mark [kalacakra-org]. No
  source read writes the mark in a date, so the formatter writes none;
- `hindu-lunar`, `hindu-lunar-surya-siddhanta`, `hindu-lunar-purnimanta`,
  `vira-nirvana-samvat`, `vikram-samvat-kartikadi`, `rajyabhisheka-saka`,
  `saptarshi` and `odia-anka`, a tithi that spans two sunrises and names
  both days. Wikipedia names such a tithi "an *adhika* or *ahorātra*
  tithi" [wikipedia-tithi], and *Calendrical Calculations*, whose
  arithmetic the calendars follow, carries the second day as a leap day
  [reingold2018], but no source read writes a mark in a date;
- `nepal-sambat`, a doubled tithi as in the Hindu calendars. Nepal's
  almanac committee writes the tithi on both days and makes the weekday
  compulsory to tell them apart [nepal-panchang-committee-2081]. The
  formatter does not write the weekday, since the rule gives no written
  form to place it in; a text with the weekday, *Silā 21, 1110 NS,
  Friday*, reads as the one day.

The test lists each of these calendars with its refusal and reason, and
reads each on a pair of its doubled days: in `tibetan` 15 and 16 January
1990, and in the others 15 and 16 February 1990. It fails on any other
refusal, on a wrong day, and in a release build on a listed refusal the
sweep no longer meets. `crates/hyper-calendar/tests/distinct_dates.rs`
holds the calendars whose leap units are named, the Lao and Discordian
among them, to a text of their own for every day of 1950–2050.

Every calendar converts its own first and last day back from its fields
(`crates/hyper-calendar/tests/range_ends.rs`), which names the 17 that
have neither. The last day of `hindu-lunar-purnimanta`, `odia-anka` and
`saptarshi` is a dark fortnight named for the Chaitra after it, which
begins past the range, and its dark half is found from the Phālguna it
belongs to. The last ISO year of `iso8601`, `iso8601-week` and
`week-and-month` takes the next year's week one.

A debug build reads every sample day in the calendar's own language, and
every other locale setting on one of the calendar's days, staggered over
the pairings, so that every calendar is read in every locale (policy §7):
about 12 700 texts, some thirty seconds. The anchors are the dates of
this document and of the calendars' own system documents, all read in
either build:

- 令和元年5月1日, and 令和1年5月1日, 平成1年1月8日 and 明治1年9月8日; 嘉永三年一月一日,
  in an era `ja`'s CLDR file names; 明治元年9月8日, 23 October 1868
  ([japanese-eras.md](japanese-eras.md)); 万延元年3月3日 and 万延元年閏3月3日,
  a month apart, and the leap day as every locale that carries the
  calendar writes it;
- 광무 1년 8월 14일 ([east-asian-eras.md](east-asian-eras.md));
- 康熙五十二年十一月初一 and 康熙52年十一月1日, and 康熙五十二年十一月一日 as
  the formatter writes it in both Chinese scripts;
- 民國115年9月28日, *28 сентября 115 г. Minguo*, *28 сентября 8 г. Рэйва*,
  28 กันยายน ปีไต้หวัน 115,
  *28 Eylül 2026*, ٢٨ سبتمبر ٢٠٢٦ and २८ सप्टेंबर, २०२६;
- *1 Adar II 5784*, with *1 Adar II 5785* refused, since 5785 is a common
  year;
- Wikipedia's Hebrew dates [wikipedia-hebrew-numerals], in full and in
  common usage: יום שני ט״ו באדר ה׳תשס״ד and תשס״ד, Monday 8 March 2004;
  יום חמישי ג׳ בניסן ה׳תשס״ז and תשס״ז, Thursday 22 March 2007; and
  א׳ בתשרי ה׳ב׳, 1 Tishri 5002, not the year 5005 and a weekday;
- ۶ مهر ۱۴۰۵ without its era; 9月28日 and *September 28*, refused;
- 2026丙午年八月十八 and 2026년(병오년) 8월 18일 in each of the three calendars,
  丙午年八月18日 for the Dangi in Traditional Chinese and Cantonese,
  2023癸卯年闰二月初一 for 22 March 2023, and 2025丙午年八月十八 refused;
- *St. Tib's Day, 3190 YOLD*, *Analā 1, 1111 NS* and ເດືອນແປດຫລັງ ຂຶ້ນ 15
  ຄ່ຳ ປີ 1388, each read as its own day.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [uts35-v48] | Part 1, "Loose Matching": the apostrophe-like characters read as one | Yes, 2026-09-29, the release's `tr35.html` |
| [uts35-dates-48] | "Parsing Dates and Times": numeric and symbolic fields, symbols first, names at every width with and without an abbreviation marker, case and normalisation variants, the warning about narrow names; `yy` as the two low-order digits of the year | Yes, 2026-09-28, the section in the release-48 source of the specification (`docs/ldml/tr35-dates.md` of `unicode-org/cldr`) |
| The system documents of the calendars | The dated examples used as anchors: [japanese-eras.md](japanese-eras.md) for 万延元年3月3日 and 明治元年9月8日, [east-asian-eras.md](east-asian-eras.md) for 光武元年8月14日 | Their own sources, as those documents record |
| [wikipedia-ja-genchu], [wikipedia-ja-manen] | 閏 before a Japanese era's month number: 元中9年閏10月5日; 万延元年's 閏三月 | Yes, 2026-09-28 |
| [wikipedia-zh-go-komatsu] | 明德3年閏10月5日 and 明德3年闰10月5日, a Japanese era's leap month in Chinese | Yes, 2026-09-28, both renderings |
| [cldr48-most-spoken] | `yue.xml` and `yue_Hans.xml`: the Chinese calendar's leap pattern 閏{0} and 闰{0}; no leap pattern in their `japanese` calendar | Yes, 2026-09-28, those two elements |
| [bramsen1910] | The intercalary month named "by the name of the preceding month" | Yes, 2026-09-28, the introductory essay in the Internet Archive's text; the tables, which mark the month "Int.", not read |
| [ndl-koyomi-history] | "intercalary", the English word for the Japanese calendar's extra month, "Uru-zuki" | Yes, 2026-09-28 |
| [cldr48-japanese-eras] | Each locale's CLDR names for the Japanese eras; the leap patterns 윤{0} of `ko.xml` and {0} Nhuận of `vi.xml` | Yes, 2026-09-28, by `scripts/japanese-eras-cldr.py` |
| [kalacakra-org] | *zla shol*, *zla ba lhag pa* for the intercalary month; the Bhutanese archive's doubled days, written without a mark | Yes, 2026-09-28, "On intercalary months" and the year beginning in 2025 |
| [janson2014] | The first of a doubled date "denoted 'Extra' in the almanacs", §6; no Tibetan word for the leap month in dates | Yes, 2026-09-28, the ar5iv rendering of the arXiv text |
| [gb-t-15835-2011] | §4.2.1: Han numerals for 历史朝代纪年 and 农历月日, 清咸丰十年九月二十日 | Yes, 2026-09-28, a reproduction of the standard's text |
| [wikipedia-zh-kangxi] | 康熙六十一年十一月十三日 and 康熙元年, in both renderings | Yes, 2026-09-28 |
| [cldr48-calendar-dates] | `he.xml`'s four Hebrew-calendar date patterns with `numbers="hebr"`; the `chinese` calendar's `y` items and long dates in `zh.xml`, `zh_Hant.xml`, `yue.xml`, `yue_Hans.xml`, `ja.xml` and `ko.xml`, which `dangi` inherits in `zh.xml`, `ja.xml` and `ko.xml`; the `dangi` calendar of `zh_Hant.xml`, `yue.xml` and `yue_Hans.xml`, "U年MMMd日" | Yes, 2026-09-28, those elements |
| [principia-discordia-p34] | St. Tib's Day, "inserted between the 59th and 60th days of the Season of Chaos" | Yes, 2026-09-28, a transcription of the page |
| [wikipedia-nepal-sambat] | Analā, अनला, "The intercalary month" of the table of months | Yes, 2026-09-28, the wikitext's table |
| [nepal-panchang-committee-2081] | A doubled tithi written on both days, and the weekday compulsory beside it | Yes, 2026-09-28, a reproduction of the notice's text |
| [wikipedia-lo-khao-phansa] | ເດືອນແປດຫລັງ, the later eighth month, in a date | Yes, 2026-09-28, revision 54400 |
| [wikipedia-tithi] | A tithi at two sunrises, "an adhika or ahorātra tithi" | Yes, 2026-09-28, the wikitext |
| [reingold2018] | The second day of a doubled tithi carried as a leap day | As the Hindu calendars' system documents record |
| [wikipedia-hebrew-numerals] | The thousands left out in the present millennium, "presently 5"; the dated examples | Yes, 2026-09-28; the numerals' other sources are in [hebrew-numerals.md](hebrew-numerals.md) |

The templates and names the reader walks are `hc-i18n`'s, sourced where
they are stated. The leap-month words, the named leap units and the
regnal numerals above are this document's additions; the rest of the
vocabulary is the locales'.

## Code

- `hc-format`: `label::parse_date`, `label::DateRefusal` and
  `label::ParsedDate`, in `crates/hc-format/src/label/read.rs`, beside the
  renderer in `label.rs` whose templates and name writers it reuses.
- `hc-calendar`: `Calendar::era_code` and `DynCalendar::era_code`,
  implemented by `japanese` and its variants, `chinese-regnal`,
  `korean-regnal`, `meyer-palmen` and the Javanese calendars.
- `hc-calendar`: `DynCalendar::has_intercalary_month_of`, the kind of
  year a month's name goes by, *Adar II* or ເດືອນແປດຫລັງ.
- `hc-i18n`: `names::for_each_era_name`; `names::leap_month_prefix`,
  `names::leap_month_suffix` and `names::month_label_in`, which take a
  calendar's leap-month word from the entry that names its months, or
  else from another of the locale's entries for the calendar, as the
  Japanese era calendars state theirs; `names::LeapMonthNames` and
  `names::leap_day_name`, for a leap unit written by a name of its own.
- `hc-format`: `label::era_label`, the era's name as a date writes it,
  which the facade's lines share.
- `hc-format`'s renderer: a placeholder naming a numbering system,
  `{year:hans}`, writes the number in it. `{year:hebr}` falls back to
  digits for a year whose numerals would not read back as itself, and
  for a date that as a whole would not (`Renderer::reads_back`).
- `hc-i18n`: the numbering system `hebr`, `numbering::same_typed_mark`,
  `NumberingSystem::continues_into` and `writes_char`, and
  `names::DateTemplates::omitted_thousands`.
- The facade: `lines::parse_date`, the export `hc_parse_date` in the
  `calendars` layer, and the binding's `parseDate`.
- Tests:
  - `crates/hyper-calendar/tests/written_dates.rs`, the sweep, the anchors
    and one test for each refusal;
  - `crates/hyper-calendar/tests/range_ends.rs`, which converts every
    calendar's first and last day to fields and back;
  - `crates/hyper-calendar/tests/date_refusals.rs`, which holds the
    refusal tables of this document, the READMEs and the bindings to
    `DateRefusal`'s codes and names;
  - `crates/hyper-calendar/tests/distinct_dates.rs`, which holds the
    calendars whose leap units are named to a text for each day;
  - `crates/hyper-calendar/tests/vocabulary.rs`, which holds `era_code` to
    every era a calendar names itself;
  - `lines`' `a_described_date_reads_back_as_its_line`;
  - the boundary tests in `hyper-calendar-wasm`, `hyper-calendar-ffi` and
    `js/hyper-calendar.test.js`.
