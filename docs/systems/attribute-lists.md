# Attributions to calendar units: birthstones, flowers, moon names, month names and weekdays

Backs the crate `hc-attributes`: its modules `authority`, `birthstones`,
`zodiac_stones`, `birth_flowers`, `moon_names`, `month_names`,
`weekday_attributions` and `gaps`; the twenty-five table identifiers listed
under "What is carried"; the six gap identifiers; and the computed
`moon_names::harvest_moon`, `moon_names::hunters_moon`,
`moon_names::harvest_moon_falls_in`, `moon_names::september_moon_name` and
`zodiac_stones::stones_on`. The crate is reached as
`hyper_calendar::hc_attributes` under the facade's `attributes` feature. The
WebAssembly and C surfaces export none of it. Policy §5 (competing
conventions get names) is why every list is its own table, and §10 says the
1912 and the 2016 American birthstone lists are both real.

## What it is

An attribution list says that a calendar unit is associated with a thing: a
month with a stone, a sign with a stone, a weekday with a god. None of it is
a fact about the unit. Each list is a fact about who issued it or who
recorded it, and the sources below show that most subjects have several lists.

**Birthstones.** George Kunz prints eight lists that predate 1912, one per
people or source (Jews, Romans, Isidore of Seville, Arabians, Poles, Russians,
Italians) and a last column headed "15th to 20th Century" (p. 315). He then
prints the list that the National Association of Jewellers "suggested and
adopted" at Kansas City in August 1912 (pp. 317 and 319 to 320)
[kunz1913]. Jewelers of America (JA) says the official American list dates
from 1912 and was established by the American National Retail Jewelers
Association, which is today's JA [jewelers-america-birthstones]; National
Jeweler gives the same name [nationaljeweler-spinel-2016]. Per JA, the list
was updated in 1952 (alexandrite for June, citrine for November, tourmaline
for October, zircon for December), and tanzanite became December's in 2002
[nationaljeweler-birthstone-evolution]. Spinel joined peridot for August in
2016, announced by JA and the American Gem Trade Association (AGTA)
[nationaljeweler-spinel-2016]. Wikipedia names the Jewelry Industry Council
of America for 1952 and adds that March's primary and alternative stones were
swapped and December's lapis lazuli replaced by zircon [wikipedia-birthstone].
The 2016 announcement came from JA and AGTA, not from the American Gem
Society.

The American trade bodies do not print one list today. National Jeweler
reported in 2016 that the American Gem Society left moonstone out of June and
that AGTA listed bloodstone for March, which JA and the Society did not
[nationaljeweler-birthstone-evolution]. On 2026-10-03 the pages read differ
in five months:

| Month | JA [jewelers-america-birthstones] | GIA [gia-birthstones] | American Gem Society [american-gem-society-birthstones] |
| --- | --- | --- | --- |
| March | aquamarine | aquamarine, bloodstone | aquamarine, bloodstone |
| June | pearl, moonstone, alexandrite | pearl, alexandrite, moonstone | pearl, alexandrite, moonstone |
| August | peridot, spinel | peridot, spinel, sardonyx | peridot, sardonyx, spinel |
| November | citrine, topaz | topaz, citrine | topaz, citrine |
| December | turquoise, tanzanite, blue zircon | tanzanite, turquoise, zircon | tanzanite, turquoise, zircon |

In the other seven months all three print garnet, amethyst, diamond, emerald,
ruby, sapphire, and opal with tourmaline. The Gemological Institute of America
(GIA) is not a body that issues a list; its pages give the stones and no
history of the list. Of AGTA's own page only the 403 answer was read.

Britain's National Association of Goldsmiths standardised a list in 1937,
per Wikipedia citing the *Oxford Companion to the Decorative Arts*
[wikipedia-birthstone]; no printing of that list was read. Japan's
association, 全国宝石卸商協同組合 (ZHO), lists a 1958 announcement of fifteen
stones on its history page [zho-history], and on 20 December 2021 revised the
list to twenty-nine stones by adding ten, so as to unify the lists in use
[kobe-np-birthstone-2021]. The committee's site says the 1958 Japanese list
was based on the American trade's list [irori-birthstone].
"Hindu", "Ayurvedic" and "Tibetan" month lists circulate on retail pages
that give no text for them [skyjems-eastern-birthstones].

**Zodiac and weekday stones.** Kunz prints a stone for each sign, with
dates, in verse (pp. 345 to 347), an "old Spanish list" that "probably
represents Arab tradition" (p. 347), and "gems of week days" (pp. 332 to 334),
two stones for each day, with a talismanic gem for each day beside them,
a list of phenomenal gems for the days (p. 335) and the colours and stones
of Siam for each day (p. 335) [kunz1913].

**Birth flowers.** The Old Farmer's Almanac prints one or two flowers a
month and says that cultures disagree and that availability varies
[ofa-birth-flowers]. A British retailer prints the same list with three
differences [bloomandwild-birth-flowers]. In Japan the practice of a flower
for each day of the year spread through NHK's radio programme
ラジオ深夜便, and Japanese Wikipedia cites several books whose flowers
for the same day differ [wikipedia-ja-birth-flower]. A magazine site of the
NHK Foundation says two experts, 柳宗民 and 鳥居恒夫, chose 366 plants for
seasonal fit and for Japanese events such as Setsubun
[steranet-birthday-flowers].

**Full-moon names.** Jonathan Carver (1778) gave a name to each month, from
March, the year starting at the first new moon after the vernal equinox, and
wrote only of "the Indians" [carver1778]. The Maine Farmers' Almanac for 1937
printed twelve other names, three to a season, and called them the names
given by "our early English ancestors" [projectpluto-bluemoon]. The Old
Farmer's Almanac first printed its twelve names in 1964, with June Moon,
Hot Moon for July and Travel Moon for November, which differ from today's
[history-com-full-moon-names]. Its page now says many names come from
Native American, Colonial American and other sources, and that Wolf Moon is
thought to be English [ofa-full-moon-names]. "Harvest moon" is recorded in
the *Oxford English Dictionary* for 1706 and "hunter's moon" in *The British
Apollo* of 1710, which gives it to "the country people"
[wikipedia-harvest-moon]. Richard Irving Dodge (1882) denied that the
Plains peoples he knew had fixed names for twelve moons [wikipedia-full-moon].

**Month names.** Bede (725) recorded the Old English months, a lunar list
with two Giuli and two Litha [wikipedia-germanic-calendar]. Einhard records
that Charlemagne gave the months names in his own tongue and lists them
[fordham-einhard]. The Finnish names are descriptive and the Czech names are
Slavic; Vojtěch Kebrle (1939) explains the Czech ones and says where the
explanations are open [kebrle1939].

**Weekday attributions.** The planetary week is attested at Pompeii by
a graffito of 60 CE, and Constantine made *dies Solis* a legal holiday in
321; the Germanic peoples substituted their gods, and the Old Norse Saturday
is "washing day" [wikipedia-names-of-days]. Thailand gives each day a
colour and a god [wikipedia-thai-solar-calendar].

## How it works

Every table is one `Authority` and `N` entries: twelve for a month or a sign,
seven for a weekday. An `Authority` holds an identifier, a name, the body
that issued the list (or `None`), a region, the dates established and
revised, the years it was current, a `Provenance` (`Promulgated`,
`Recorded`, `Vernacular`, `Contested` or `ModernInvention`), a source string
and a caveat. A contested authority must carry a caveat; so must one whose
validity has an end. An entry is a list of names in the order its source
printed them, never a single name, because many months carry several.

One function, `AttributionTable::at`, reads every table. A month is an
index 0 to 11, and a leap month answers `UnsupportedField("leap month")`
since no list assigns anything to one. A weekday table is indexed from Sunday,
and a key out of range answers `MonthOutOfRange`. `unanimous` and
`disagreement_count` compare entries as ordered lists, so two lists that name
the same stones in opposite orders disagree. There is no
`birthstone(month)`; `birthstones::all_for_month` returns one row for each
authority.

Two things are computed, not tabulated. The first is the Harvest Moon. The
Old Farmer's Almanac defines it as the full moon closest to the September
equinox, says that when it is October's the September moon is the Corn Moon,
and says the Hunter's Moon is the full moon that follows it
[ofa-full-moon-september-2026, ofa-full-moon-october-2026]. The code takes the
equinox as the autumn cardinal term from `hc-seasons`, takes the full moons of
August to November from `hc-seasons`' principal phases for the chosen
meridian, and picks the full moon whose instant is nearest the equinox,
earlier on a tie. The second is `zodiac_stones::stones_on`, which asks
`hc-seasons` for the tropical sign in effect on a day, the sign whose
ingress falls on or before the end of that day, and looks the stone up.

**Worked example.** December. `all_for_month` on a December returns six
rows: traditional [bloodstone, ruby]; 1912 [turquoise, lapis lazuli];
`birthstones-uk-2007` [tanzanite, turquoise]; `birthstones-jp-1958`
[turquoise, lapis lazuli]; `birthstones-us-2016` [turquoise, zircon,
tanzanite]; `birthstones-jp-2021` [turquoise, lapis lazuli, zircon,
tanzanite]. Five entries are distinct (the 1912 and 1958 rows are equal), so
`authorities_agree` is false, and only January (garnet in each) is
unanimous: `months_in_dispute` is 11. Kunz's 1912 table agrees with the
second row [kunz1913].

The Harvest Moon for 2025, run in Universal Time. The September equinox is
2025-09-22 18:19. The full moons are 2025-09-07 18:09, 15.007 days before it,
and 2025-10-07 03:48, 14.395 days after it. October's is nearer by 0.612 days,
so the Harvest Moon is 7 October, `harvest_moon_falls_in` answers 10,
`september_moon_name` answers "Corn Moon", and the Hunter's Moon is the next
full moon, 2025-11-05. For 2024 the equinox is 2024-09-22 12:44, the full
moons are 2024-09-18 02:35 (4.423 days before) and 2024-10-17 11:27 (24.946
days after), so the Harvest Moon is 18 September and the Hunter's Moon 17
October. EarthSky gives 03:48 UTC on 7 October 2025 for the full moon
[earthsky-harvest-moon-2025].

## What is carried

| Module | Identifiers | Keyed by |
| --- | --- | --- |
| `birthstones` | `birthstones-traditional`, `birthstones-us-1912`, `birthstones-uk-2007`, `birthstones-jp-1958`, `birthstones-us-2016`, `birthstones-jp-2021` | month |
| `zodiac_stones` | `zodiac-stones-kunz-1913` | tropical sign |
| `birth_flowers` | `birth-flowers-anglo-american`, `birth-flowers-uk-trade` | month |
| `moon_names` | `moon-names-ofa-current`, `moon-names-ofa-1964`, `moon-names-maine-1937`; `moon-names-carver-1778` | month; lunation from the March equinox |
| `month_names` | `month-names-old-english-bede`, `month-names-frankish-charlemagne`, `month-names-finnish`, `month-names-czech`, each with an English gloss | month |
| `weekday_attributions` | `weekday-planets-greco-roman`, `weekday-deities-germanic`, `weekday-names-old-english`, `weekday-luminaries-japanese`, `weekday-vasara-sanskrit`, `weekday-colours-thai`, `weekday-deities-thai`, `weekday-stones-kunz-1913` | weekday, Sunday first |
| `gaps` | `japanese-daily-birth-flowers`, `ayurvedic-birthstones`, `tibetan-birthstones`, `hindu-birthstones`, `hebrew-breastplate-stones`, `celtic-tree-calendar` | none |

Each table is looked up by its identifier (`by_id`), and a test holds every
identifier unique across the crate. The tables need neither `std` nor `alloc`;
`zodiac_stones` and the Harvest Moon functions need the `seasons` feature.
Names of stones are lowercase English so that two lists compare equal.
Everything is tabulated except the Harvest Moon and the sign a day falls in,
which are computed from `hc-seasons` and `hc-astro`; the tests run the Harvest
Moon over 1900 to 2099.
The `gaps` entries are values with a reason (`GapReason`) and the sources
consulted. Japan's 和風月名 are `hc-i18n`'s, the 七曜 annotation and the
moon-viewing dates are `hc-almanac`'s, and a test keeps the 和風月名 out of
this crate.

What the crate does not carry, with the reason that is true now:

- Not carried: the British list of 1937 (no printing of it was read). The
  table `birthstones-uk-2007` is the list as a 2007 archive copy printed it,
  which was not re-read here.
- Not carried: JA's, AGTA's and the American Gem Society's current lists as
  three tables (not yet done). `birthstones-us-2016` is attributed to JA and
  AGTA but its entries are the sets that GIA and the Society print, which
  JA's own page does not (see Accuracy); AGTA's page could not be read.
- Not carried: the Japanese list of 1958 as announced (not yet done). The
  history page names fifteen stones; `birthstones-jp-1958` holds nineteen,
  the stones the 2021 notice revised.
- Not carried: Kunz's seven other lists of p. 315, his Spanish zodiac list
  (p. 347) and his talismanic, phenomenal and Siamese weekday lists (pp. 332
  to 335) (not yet done; each is a published list that could be a table of
  its own).
- Not carried: "Hindu", "Ayurvedic" and "Tibetan" month lists (the retail
  pages read name no text for them, so no authority could be named under
  §10; the *navaratna* belong to the nine planets and to a person's chart,
  per the code's note on Johari, which was not read).
- Not carried: Japan's 366-day 誕生花 (the NHK list itself was not read). The
  gap's wording that NHK never published how it chose is not borne out in
  full: a page of the NHK Foundation names the two experts and their criteria
  [steranet-birthday-flowers], though it gives no rule for a flower to a day.
  A table that names the programme and its compilers is not yet done.
- Not carried: Aaron's breastplate as a list of months (Kunz prints the
  Authorized Version and a "later correction" side by side on p. 319, and the
  code records that the identifications differ; not yet done).
- Not carried: the "Celtic tree calendar" (recorded as a gap with Graves's
  *The White Goddess* of 1948, which was not read; Wikipedia calls it a modern
  creation extrapolated from the *Song of Amergin* [wikipedia-celtic-calendar],
  where the code says his reading of the Ogham tree names).
- Not carried: the names that the Old Farmer's Almanac's page gives for
  individual peoples, about one hundred, each marked with a people
  [ofa-full-moon-names]. The tables carry the Almanac's own twelve and
  Carver's and Maine's lists, none labelled with a nation (not yet done; no
  source for any per-nation list was read).

## Accuracy

The tables are transcriptions and the tests compare them with the published
lists; `cargo test -p hc-attributes` passed on 2026-10-03 with 156 unit tests
and 8 documentation tests. This document goes further and checks each table
against the source it names, where one could be read, on 2026-10-03:

| Table | Read | Result |
|---|---|---|
| `birthstones-traditional` | Kunz p. 315 [kunz1913] | Nine months agree. March, October and December differ: Kunz's 15th to 20th century column prints jasper then bloodstone, beryl then opal, ruby then bloodstone; the table has [bloodstone, jasper], [opal, aquamarine], [bloodstone, ruby]. Aquamarine is in Kunz's columns for the Jews, Romans, Isidore, Arabians and Poles, not in the column the table is named for. |
| `birthstones-us-1912` | Kunz pp. 319 to 320 | All twelve agree, in order. |
| `birthstones-uk-2007` | none | Not checked. |
| `birthstones-jp-1958` | [zho-history] | Does not agree: the history page names, for August, October, November and December, peridot, opal, topaz and turquoise alone; the table adds sardonyx, tourmaline, citrine and lapis lazuli. The caveat in the code says this. |
| `birthstones-us-2016` | [jewelers-america-birthstones], [gia-birthstones], [american-gem-society-birthstones] | Equal as sets to GIA's and the Society's twelve. JA's page differs in March (no bloodstone), August (no sardonyx) and December (blue zircon), and orders November as citrine, topaz. The order differs from GIA's pages in June and December, and from the Society's in June, August and December; in June it equals JA's. |
| `birthstones-jp-2021` | [wikipedia-ja-birthstone], [kobe-np-birthstone-2021] | Agrees as sets with Japanese Wikipedia's table of ZHO's revision; twenty-nine stones, ten added, as the newspaper says. ZHO's notice, a PDF, was not opened. |
| `zodiac-stones-kunz-1913` | Kunz pp. 345 to 347 | All twelve agree. Kunz's dates (Aries 21 March to 20 April, and so on) are not what `stones_on` uses: it uses the Sun's longitude, so 20 March 2025 is Aries in the code and Pisces by Kunz's dates. |
| `weekday-stones-kunz-1913` | Kunz pp. 332 to 334 | All seven agree; Kunz spells "loadstone". |
| `birth-flowers-anglo-american` | [ofa-birth-flowers] | All twelve agree, primary first. |
| `birth-flowers-uk-trade` | [bloomandwild-birth-flowers] | All twelve agree. The source is one retailer, not a trade body's list. |
| `moon-names-ofa-current` | [ofa-full-moon-names] | Ten months agree. The page prints September as Corn Moon with Harvest Moon in brackets, and the table has Harvest first; the page prints October as Hunter's Moon only, and the table adds Harvest Moon, which the Almanac's October page explains by the rule above. |
| `moon-names-ofa-1964` | [history-com-full-moon-names] | June, July and November agree; the other nine names are not given on the page and are taken to equal today's. |
| `moon-names-maine-1937` | [projectpluto-bluemoon] | All twelve agree with the transcription. The almanac itself was not read. |
| `moon-names-carver-1778` | [carver1778] | Ten agree. April and May are "month of Plants" and "Month of Flowers" in the text, "Plants Moon" and "Flowers Moon" in the table. The names are on pp. 251 to 252 of the edition read, not p. 250, where the chapter begins. |
| `month-names-old-english-bede` | [wikipedia-germanic-calendar] | The ten names agree with the translation as Wikipedia quotes it. The glosses agree in sense except where Wikipedia has "Paschal month" for Eosturmonath, "gentle" or "navigable" for Litha (the table: before and after midsummer), "month of sacred rites" for Hāligmōnaþ and "immolations" for Blōtmonath (the table: "holy month", "month of sacrifice"). The Ærra and Æfterra forms are not in what was read. |
| `month-names-frankish-charlemagne` | [fordham-einhard] | The twelve names agree, except that the text read prints July as Heuvimanoth; its translation gives no glosses, so the glosses are unchecked. |
| `month-names-finnish` | CLDR 48 `fi.xml` [cldr48-main] | The twelve names agree. The etymologies were not read. |
| `month-names-czech` | [kebrle1939] | The twelve names agree and ten glosses agree. Two do not: Kebrle's text derives červen from red fruit and his editor's note lists three rival readings (the dye insect, bee brood, bees), but the table glosses it as the dye insect with no caveat; he derives prosinec from millet (*proso*) and the note calls it unclear, but the table glosses it as "month the sun shines through". |
| `weekday-planets-greco-roman`, `weekday-luminaries-japanese`, `weekday-deities-thai` | CLDR 48 `la.xml`, `ja.xml`, `th.xml` [cldr48-main] | The day names agree. The Thai deities agree with [wikipedia-thai-solar-calendar]. |
| `weekday-colours-thai` | [wikipedia-thai-solar-calendar] | All seven agree with the table there; the sources the code cites for it were not read. Kunz p. 335 prints a different set of colours for Siam. |

The Harvest Moon agrees with the Old Farmer's Almanac's two instants for 2026
to the minute: 12:49 EDT on 26 September (16:49 UT) and 00:12 EDT on 26
October (04:12 UT) [ofa-full-moon-september-2026, ofa-full-moon-october-2026],
and with EarthSky's 03:48 UTC for 2025. The README's claim that the lunar
phases and the equinox are good to about a minute is `hc-seasons`'; it
was not re-measured here. Over 1900 to 2099 in Universal Time the Harvest Moon
is in October in 44 years (22 per cent; the README says about one year in
four, and the test asserts 40 to 70) and falls between 8 September and 8
October. The meridian matters more than the README
says. Between the Universal and the Japanese meridians the day differs by at
most one, in 71 years, but in seven (1917, 1936, 1993, 2031, 2050, 2069 and
2088) the day is 30 September in one and 1 October in the other, and so
`september_moon_name` answers Harvest Moon for one and Corn Moon for the other.
The test named `the_meridian_can_move_the_harvest_moon_a_day_but_not_a_month`
asserts only the one day. `stones_on` inherits the solar longitude of
`hc-seasons`; its documentation says a sign boundary within about ten minutes
of local midnight can move the answer by a day, which was not re-measured here.

The code and its documentation disagree in places. `months_in_dispute` is 11
(the test and the README say eleven), but the documentation of
`disagreement_count` says twelve. The documentation of `BIRTHSTONES_JP_2021`
says the 2021 and 2016 lists differ in seven months, and the test says eight:
they differ in seven as sets, and in eight as ordered lists, since August has
the same stones in two orders. The documentation of `BIRTHSTONES_US_2016`
says 1952 added pink tourmaline to October, while the table of 1912, which
agrees with Kunz, already has tourmaline in October. The table
`moon-names-ofa-current` claims 1964 as the year it was established and has
validity from 1964, which is the year of `moon-names-ofa-1964`; when the
Almanac changed June, July and November is not in any source read. The
README says the Maine almanac was read "at first hand"; the code's source
string says it was not read. The crate-level documentation says 中秋の名月
is `hc-seasons`'; the README and `moon_names` say `hc-almanac`'s.

## Sources

Read on 2026-10-03, as HTML pages (the Gutenberg texts are the editions
named). Where a page was read through a fetch tool's extraction and not in its
markup, the entry in `references.bib` says so.

- [kunz1913]: the eight lists (p. 315), the American list (pp. 317 to 320),
  the zodiacal stones (pp. 345 to 347), the weekday stones (pp. 332 to 335),
  from Project Gutenberg's HTML. The existing entry says "not re-read"; it
  was read here.
- [jewelers-america-birthstones]: JA's current list and the 1912 origin.
- [gia-birthstones]: GIA's twelve month pages; the overview names no stones.
- [american-gem-society-birthstones]: the Society's chart, read through a
  fetch tool's summary, because a direct request answered 403, so the markup
  was not inspected.
- [nationaljeweler-birthstone-evolution], [nationaljeweler-spinel-2016]: the
  revisions and the bodies' differences.
- [wikipedia-birthstone]: the 1952 body, the British date and Osborne's
  citation; secondary.
- [zho-history]: the fifteen stones of 1958 and the 2021 total.
- [kobe-np-birthstone-2021], [irori-birthstone], [wikipedia-ja-birthstone]:
  the 2021 revision, its motive, its table; secondary for the table.
- [ofa-birth-flowers], [bloomandwild-birth-flowers]: the two flower lists.
- [wikipedia-ja-birth-flower], [steranet-birthday-flowers]: the Japanese
  flowers; secondary and an NHK Foundation magazine site.
- [ofa-full-moon-names], [ofa-full-moon-september-2026],
  [ofa-full-moon-october-2026]: the Almanac's names and the Harvest Moon rule.
- [history-com-full-moon-names]: 1964's three names and nine of twelve names
  from Carver; [projectpluto-bluemoon]: the 1937 list as transcribed;
  [carver1778]: Carver's months, in the Gutenberg text of the 1778 edition;
  [wikipedia-full-moon]: Dodge, quoted, and the Red Men;
  [wikipedia-harvest-moon]: 1706 and 1710, and the definition;
  [earthsky-harvest-moon-2025]: the instant of 7 October 2025.
- [wikipedia-germanic-calendar]: Bede's months in Wallis's translation;
  [fordham-einhard]: Einhard's chapter 29 in Turner's translation;
  [kebrle1939]: Kebrle; [cldr48-main] (an existing key), the files `fi.xml`,
  `la.xml`, `ja.xml` and `th.xml` of release 48.
- [wikipedia-names-of-days], [wikipedia-thai-solar-calendar],
  [wikipedia-celtic-calendar], [skyjems-eastern-birthstones]: secondary, and a
  retailer's blog for the Eastern lists.

Not read, and what each is cited for in the code: the National Association of
Goldsmiths' page of 2007 (the British table; the archive page could not be
fetched); Osborne, *Oxford Companion to the Decorative Arts* (1985) p. 513
(1937); Grande and Augustyn, *Gems and Gemstones* (2009) p. 335 (tanzanite);
ZHO's notice of 2021, a PDF; 巽忠春, *あなたの宝石* (1962) (coral and jade);
NHK's reports (the market's fall); AGTA's page (403); the Maine Farmers'
Almanac of 1937 itself; Sky & Telescope (403); Dodge, *Our Wild Indians*;
Wallis's translation of Bede and the Latin; Köbler's Old English dictionary;
Braune's *Althochdeutsches Lesebuch* and De Rynck's Einhard (the glosses);
Uusi kielemme, Kotus and Wiktionary (the Finnish etymologies);
Monier-Williams and Fleet's *Corpus Inscriptionum Indicarum* III (the Sanskrit
names, and the Eran inscription's date, which is cited as year 165 and not
verified); Segaller, *Thai Ways*, and Lee (the colours); Grimm, *Teutonic
Mythology*, and the *Dictionary of Old Norse Prose* (the Germanic names);
Schaff (Constantine); Knuth (sign dates); Johari (the navaratna); Gleadow;
and Graves.

## Code

`crates/hc-attributes/src/`: `authority.rs` (the shape and the comparison
functions), `birthstones.rs`, `zodiac_stones.rs`, `birth_flowers.rs`,
`moon_names.rs`, `month_names.rs`, `weekday_attributions.rs`, `gaps.rs`.

The tests that anchor it, each in the module of its subject:
`january_is_the_only_month_all_six_authorities_agree_on`,
`the_american_and_japanese_lists_of_the_2010s_and_2020s_genuinely_differ`,
`the_japanese_revision_of_2021_added_exactly_ten_stones`,
`the_almanac_changed_three_of_its_own_names_between_1964_and_now`,
`seven_of_the_twelve_modern_names_appear_in_carvers_1778_list_verbatim`,
`the_harvest_moon_falls_in_october_in_the_years_it_does`,
`no_full_moon_is_nearer_the_september_equinox_than_the_harvest_moon`,
`a_day_in_mid_january_takes_capricorns_stone_not_januarys`,
`saturday_has_no_germanic_deity_and_the_table_says_so`, and, in `lib.rs`,
`every_contested_authority_carries_a_caveat` and
`every_authority_identifier_is_unique_across_the_whole_crate`. The facade test
`crates/hyper-calendar/tests/facade.rs` checks that the crate is reachable.
