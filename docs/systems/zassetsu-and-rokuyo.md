# The 雑節 and 六曜, and the 旧暦 they read

Backs `hc-seasons`' module `zassetsu`, and `hc-almanac`'s modules
`rokuyo`, `moon_viewing` (十五夜 and 十三夜) and `lunisolar`, which reads
the 旧暦 date all three and 不成就日 and 二十七宿 are keyed to from
`hc-calendars-lunar`.

The solar terms these days are counted from, the meridian a day is read
at, and how the instants compare with the 暦要項 are in
[solar-terms-and-pentads.md](solar-terms-and-pentads.md); this document
does not repeat them.

## What it is

**The 雑節.** Besides the 24 solar terms, a Japanese almanac prints a set
of seasonal markers that it has called 雑節 since the 明治20年暦 (1887):
節分, 彼岸, 社日, 八十八夜, 入梅, 半夏生, 土用, 二百十日, and by some
counts 二百二十日 [nao-rekiwiki-zassetsu, wikipedia-ja-zassetsu]. They are
of mixed origin. 土用 is five-phase theory: wood, fire, metal and water
take the four seasons, and earth is given the last fifth of each
[nao-rekiwiki-zassetsu]. 社日 is Chinese in origin, the day of the god of
the soil, a prayer for the harvest in spring and thanks for it in autumn
[wikipedia-ja-shanichi]. 八十八夜 and 二百十日 are Japanese 暦注, the late
frost and the typhoons [nao-rekiwiki-zassetsu, nao-rekiwiki-rekichu]. 入梅
marks the rainy season and 半夏生 the end of rice planting; 半夏生 is one of
the 72 pentads, and the one that kept a date in the almanac after the
others lost theirs [nao-rekiwiki-72ko].

The National Astronomical Observatory of Japan publishes eleven of these
days each year in the 暦要項, in its table 「二十四節気および雑節」: the four
土用の入り, 節分, the two 彼岸入り, 八十八夜, 入梅, 半夏生 and 二百十日, with the
longitude and the instant to the minute for the six that are fixed by the
Sun [nao-rekiyoko-2024, nao-rekiyoko-2025, nao-rekiyoko-2026,
nao-rekiyoko-2027-sekki]. 社日 was printed in the 本暦 and has not been among
the 雑節 of the Observatory's 暦象年表 since it began after the war, nor of
the 暦要項 [nao-rekiwiki-zassetsu].

**六曜.** A six-day cycle of lucky and unlucky days — 先勝, 友引, 先負, 仏滅,
大安, 赤口 — printed on most Japanese calendars and consulted for weddings
and funerals: weddings are most often held on 大安, and funerals are
avoided on 友引, when some crematoria and funeral businesses close
[wikipedia-ja-rokuyo]. Its six names in their modern form are first found
in the 『万暦両面鑑』 of about 1747; in the Edo period it was one 暦注 among
many; the Meiji reform banned the notes of lucky and unlucky days from the
official almanac, and 六曜 survived it [wikipedia-ja-rokuyo,
nao-rekiwiki-rekichu]. The Observatory does not print it.

**The lunisolar date.** 六曜 is a function of the lunisolar month and day,
and so are 十五夜 (the fifteenth of the eighth month) and 十三夜 (the
thirteenth of the ninth). Japan has made no official lunisolar calculation
since the 天保暦 was abolished in 1872 [nao-faq-kyureki]; what almanacs
print as 旧暦 is built from modern new moons and solar terms with the
天保暦 rules applied by analogy [nao-rekiwiki-2033, nao-topics-2014-2033].

## How it works

**The 雑節** take four shapes [nao-rekiwiki-zassetsu]:

| Shape | Days | Rule |
| --- | --- | --- |
| A solar longitude | 土用の入り at 297°, 27°, 117°, 207°; 入梅 at 80°; 半夏生 at 100° | The day the Sun reaches it |
| An offset from a term | 節分; 彼岸入り, 中日, 明け | The eve of 立春, 立夏, 立秋, 立冬; the equinox, three days before and three after |
| A count from 立春 | 八十八夜, 二百十日, 二百二十日 | The 88th, 210th and 220th day, 立春 being the first |
| A stem day | 社日 | The 戊 day nearest the equinox |

The four 土用の入り are 18° before the 立 terms at 315°, 45°, 135° and 225°,
a fifth of each 90° season; 土用 runs to the eve of the 立 term and so lasts
17, 18 or 19 days. Only the 節分 before 立春 is printed today.

社日 needs a tie rule. The ten stems repeat every ten days, so there is
always a 戊 day within five days of the equinox, and when the equinox is a
癸 day there are two, five days either side. The almanacs took the earlier
from the 貞享暦 to the 明治7年暦; from the 明治14年暦 they took the earlier
if the equinox fell before noon and the later if after. When the rule
changed in between is not known [nao-rekiwiki-zassetsu,
nao-rekiwiki-zassetsu-shanichi]; Japanese Wikipedia gives the noon rule
first and the earlier-day rule as another [wikipedia-ja-shanichi]. The
rules differ about twice in twenty years.

Three days had other rules before the modern ones [nao-rekiwiki-zassetsu,
nao-rekiwiki-72ko]. 入梅 was the first 壬 day after entering 芒種 from the
貞享三年暦; when 芒種 itself was a 壬 day the next was taken until the
元文五年暦 and 芒種 itself after it; 80° from the 明治9年暦. 半夏生 was placed
by 平気 until the 天保暦, as the 夏至 instant plus two seventy-seconds of the
year in the almanac of 1844, and from 1848 at 100° or at the second third
of the interval from 夏至 to 小暑, which the almanacs do not tell apart; Japanese
Wikipedia gives the older rule as the eleventh day counting 夏至 as the
first [wikipedia-ja-hangesho]. 土用 has been fixed by longitude since the
明治二年暦.

**六曜.** Add the lunisolar month number to the day of the month and take the
remainder by six: 0 is 大安, 1 赤口, 2 先勝, 3 友引, 4 先負, 5 仏滅
[wikipedia-ja-rokuyo]. The first of the first month is therefore 先勝, of
the second 友引, and so on, and a leap month runs as the month before it.
Within a month the cycle advances a day at a time; at each new moon it
jumps.

**The 旧暦** is `hc-calendars-lunar`'s Japanese Tenpō calendar with its
1872 bound removed, at `Meridian::JAPAN` and at every offset but the
Chinese ones; at `Meridian::CHINA` and `CHINA_BEFORE_1929` the annotations
read the `chinese` calendar instead, the same 時憲暦 rule at Beijing's
meridian to 1928 and 120°E after [hko-conversion]. `hc_almanac::lunisolar`
names the two as a `Reckoning`. A month runs from the day of one new moon to the day
before the next, read at Kyoto's meridian to 1872, Tokyo time for
1873–1887 and Japan Standard Time from 1888; the months are numbered from
the one holding the winter solstice, the eleventh, and when thirteen months
fall between two such months the first with no 中気 is the leap month and
repeats the number before it.

That is the 時憲暦 rule, not the whole of the 天保暦's. Under 定気 the 中気
are closest together near perihelion, and a month can contain two. The
天保暦 then requires the months containing 冬至, 春分, 夏至 and 秋分 to be the
eleventh, second, fifth and eighth; the Chinese 時憲暦 rule counts from one
winter solstice to the next and, when thirteen months fall between, makes
the first 中気-less month the leap month [nao-topics-2014-2033]. In
2033–34 the months holding 秋分 and 冬至 are only one month apart, so no
numbering satisfies the 天保暦 rule: this is the 旧暦2033年問題, the first
such case since the 天保暦 took effect in 1844, and the Observatory
tabulates three resolutions — 閏11月 (案1, which the 時憲暦 rule also gives),
閏7月 (案2) and 閏1月 (案3) — and notes that no public body will choose among
them [nao-rekiwiki-2033, nao-topics-2014-2033]. The 日本カレンダー暦文化振興協会
recommended 閏11月 in 2015 [wikipedia-ja-kyureki-2033]; the calendar here
gives it.

六曜 and the moon-viewing nights read that calendar, not the plain 中気
rule [nao-rekiwiki-chijun] applied month by month, taking the later 中気
when a month holds two. The plain rule is cheaper — 3,653 consecutive days
of the full calculation take 21 to 26 ms inside one memo scope against
about 10 ms for it, in a release build — and wrong where two 中気 fall in
one month; Accuracy gives the days.

**Worked example: 八十八夜 of 2024.** 立春 fell at 17:27 JST on 4 February
2024 [nao-rekiyoko-2024]. Counting 4 February as day 1, 29 February is day
26, 31 March day 57 and 30 April day 87, so day 88 is 1 May; the 暦要項
prints 八十八夜 5月01日. In 2023, with no 29 February and 立春 on 4 February
by the crate's computation, the same count lands on 2 May: the count is
from 立春, not from 1 January, and a leap day moves it.

**Worked example: 春社 of 2024.** The spring equinox fell at 12:06 JST on
20 March 2024 [nao-rekiyoko-2024]. By the sexagenary day count (see
[sexagenary-cycle.md](sexagenary-cycle.md)) 20 March 2024 is 癸未, so the
戊 days either side are 戊寅 on 15 March and 戊子 on 25 March, five days
each way: a tie. The equinox is six minutes after noon, so the rule of the
明治14年暦 takes the later day, 25 March, and the older rule takes 15 March.
Six minutes is far more than the half-minute to which the instant is
known, so the day each rule gives is not in doubt.

**Worked example: the 六曜 of 17 September 2024.** The 暦要項's 朔弦望 gives
new moons at 10:56 JST on 3 September and 03:49 JST on 3 October 2024
[nao-rekiyoko-2024-sakugenbo], so a lunisolar month runs from 3 September
to 2 October. 秋分, the 中気 of the eighth month, falls in it at 21:44 JST
on 22 September [nao-rekiyoko-2024], so it is the eighth month, and 17
September is its day 17 − 3 + 1 = 15: 中秋の名月. 8 + 15 = 23, and 23 mod 6
= 5, so the day is 仏滅 — as it is every year for the fifteenth of the
eighth month [wikipedia-ja-rokuyo].

## What is carried

- **`zassetsu`**: `Zassetsu`, a catalogue of 21 entries — the four
  `*_DOYO_ENTRY`, the four `*_SETSUBUN`, the six `*_HIGAN_ENTRY`,
  `*_HIGAN_MIDDLE` and `*_HIGAN_EXIT`, `SPRING_SHANICHI` and
  `AUTUMN_SHANICHI`, `HACHIJUHACHIYA`, `NYUBAI`, `HANGESHO`, `NIHYAKUTOKA`
  and `NIHYAKUHATSUKA` — each with a Japanese name, Hepburn romaji and an
  English gloss; `ZassetsuRule`, the four shapes above as data; `day_of`
  and `zassetsu_in_year`; `higan` (`HiganPeriod`), `doyo` (`DoyoPeriod`
  with its one or two 丑の日), `setsubun`; `shanichi`, the rule of the
  明治14年暦, which is `day_of` for the two 社日 entries, and
  `classical_shanichi`, the rule of the 貞享暦 to the 明治7年暦, both named
  and neither a setting of the other; `classical_nyubai`, the 壬 rule as
  printed from the 元文五年暦; `classical_hangesho`, the eleventh day from
  夏至. Every function takes a `Meridian`.
- **`hc_almanac::rokuyo`**: `Rokuyo`, the six in cycle order, with
  Japanese names, romaji and glosses; `Rokuyo::from_lunisolar`, the rule;
  `rokuyo` for a day at a meridian and `rokuyo_of` for a lunisolar date
  in hand.
- **`hc_almanac::lunisolar`**: `Reckoning`, `JapaneseTenpo` or `Chinese`,
  with `Reckoning::at` for the one a meridian names, `date` for a day's
  lunisolar date and `day_in_gregorian_year` for the day of a month and
  day in a Gregorian year, both inside a memo scope; `lunisolar_date` for
  a day at a meridian.
- **`hc_almanac::moon_viewing`**: `mid_autumn_moon` and
  `thirteenth_night`, each for a year at a meridian.
- **Not carried, deliberately.** The 貞享 reading of 入梅, which took the next
  壬 day when 芒種 was a 壬 day, and the 1844 reading of 半夏生, the 夏至
  instant plus two seventy-seconds of the year: each is superseded, and no
  almanac date from the years they were printed was read to anchor a test.
  平気, and so every pre-1844 placing of these days. 出梅, which the almanacs
  did not print. The Chinese and Korean 雜節; China's 三伏 and 數九 are in
  `san_fu`. The 天保暦's solstice-and-equinox rule, which no numbering
  satisfies in 2033–34. A 六曜 before 1873 that any almanac printed: the
  answers there are the modern rule over the 天保暦's rules, continued
  before 1844.
  The other 暦注, which are `hc-almanac`'s.

## Accuracy

**Against the 暦要項.** The eleven 雑節 the 暦要項 prints for each of 2024,
2025, 2026 and 2027 — forty-four days — are all on the published day. The
six longitude days of each year are printed with a JST minute, and the
twenty-four computed instants are −33 s to +32 s from it, mean +0.2 s;
the eighteen of 2024–2026 are all on the published minute and three of
2027's six fall just outside it, by 30 to 33 seconds. Three of the
twenty-four fall within thirty minutes of midnight, among them 入梅 2025 at
0:24 and 秋土用 2027 at 0:10, and each is on the published day. The
instants are the solar terms' search at other longitudes, and share its
ΔT: observed to 2026-04-01 and the USNO's predictions after it, as the
solar-terms document describes.

**社日 against the almanacs.** The 暦Wiki's table 「社日の選び方」 gives the
equinoxes and the 社日 printed in every almanac from 1842 to 1946
[nao-rekiwiki-zassetsu-shanichi]. For the 74 Gregorian years 1873–1946,
all 148 equinox days in it are the crate's, and all 148 社日 are the day of
the rule of their year: `classical_shanichi` for 1873 and 1874,
`shanichi` from 1881, and both, which agree, for 1875–1880. Fifteen of
the 148 are ties. The one that separates the rules is the autumn of 1874:
the equinox on a 癸 day at 14:41 Tokyo time, and the 明治7年暦 printed the
earlier 戊, 18 September, where the later rule takes the 28th. Over
1800–2199 the equinox is a 癸 day 78 times, and the two rules differ on
the 40 of those that fall in the afternoon: two in twenty years, as the
暦Wiki says. Before 1888 the table's times are Tokyo local time, about
nineteen minutes ahead of 135°E; no tie of 1873–1887 is within nineteen
minutes of noon, so the difference moves nothing here.

**節分.** 3 February in every year 1985–2020, 4 February in 1984, 2 February
in 2021 and in 1897 and in no year between, as the Observatory's note on
the 2021 節分 gives [nao-topics-2021-setsubun].

**The 旧暦 against the Observatory's tables.** The months of 2014, with
the leap ninth month [nao-rekiwiki-chijun, nao-topics-2014-2033]; of
1984–85, where the month from 22 December holds both 冬至 and 大寒 and is
the eleventh [nao-topics-2014-2033]; and of 2033–34 under 案1
[nao-rekiwiki-2033] are all reproduced.

**Agreement with the published reference, against the plain 中気 rule.**
Over 1900–2100 at the Japanese meridian, the plain 中気 rule applied month
by month gives another 六曜, 二十七宿 and 不成就日 than the calendar on 268
days, in five runs, each at a month holding two 中気 and covering the
months the two number differently: 25 September to 23 October 1965 (29
days); 22 December 1984 to 19 February 1985 (60); 25 August to 21 November
2033 (89, all of the 3,653 days of 2024–2033 on which they differ); 20
January to 18 February 2034 (30); and 21 December 2052 to 18 February 2053
(60). The combinations built on them differ with them; the plain rule has
no ninth month in 1965, so no 十三夜, which the calendar puts on 7 October,
and it puts 十五夜 of 2033 on 7 October, 案2's, where the calendar puts it
on 8 September, 案1's [nao-topics-2014-2033].

At the Chinese meridian the two differ on 329 days of 1900–2100, in seven
runs: 17 November to 16 December 1914, 3 February to 3 March 1916 and 10
November to 9 December 1920 (30 each), whose new moons fell a few minutes
before midnight at Beijing's meridian, which the calendar keeps to 1928,
and after it at 120°E, where the plain rule reads them; and the four runs
of 1984–85, 2033, 2034 and 2052–53 above, the same months as at the
Japanese meridian. The Hong Kong Observatory's conversion tables begin
every one of those months on the day the calendar does
[hko-conversion-tables, hko-conversion-tables-moon-keyed].

At an offset with no lunisolar calendar — `Meridian::UNIVERSAL`, `INDIA`,
a longitude — the annotations read the Japanese calendar: every meridian
but the Chinese gives Japan's answers. The plain rule would read the Moon
at that offset, a calendar nobody kept, and differs from them on 28,300
days of 1900–2100 at `UNIVERSAL`, the facade's default meridian, and on
10,759 at `INDIA`.

**Against a published calendar.** Arachne's online calendar
[arachne-onlinecalendar] prints the 六曜 of October 1965, January 1985,
September, October and November 2033, February 2034 and January 2053, a
month of each run; all 212 days are this calendar's. 便利コム's 2033
calendar prints 案2's instead [benri-com-calendar-2033]: publishers differ
in 2033, and this follows the one the 暦文化振興協会 recommended.

**六曜** is exact given the lunisolar date. The lunar new years of
2015–2026, the 2023 閏二月 and the fixed 六曜 of the first of each month
[wikipedia-ja-rokuyo] are its tests, with the days above.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [nao-rekiwiki-zassetsu] | The 雑節 since the 明治20年暦; the four 節分 and the one printed; 土用 by five-phase theory, at 297°, 27°, 117°, 207°, 17 to 19 days, by longitude since the 明治二年暦; 彼岸; 八十八夜 and 二百十日 as the 88th and 210th day and Japanese; 二百二十日 as counted by some; 入梅's 壬 rule, the 元文 change and 80° from the 明治9年暦; 半夏生 at 100°; 社日's two tie rules and its absence from the 暦象年表 and 暦要項 | Yes, 2026-09-26 |
| [nao-rekiwiki-zassetsu-shanichi] | The equinoxes and 社日 printed 1842–1946, of which 1873–1946 are the test's table; the 1874 row; about two differences in twenty years | Yes, 2026-09-26 |
| [nao-rekiwiki-72ko] | 半夏生 as the pentad that kept its date; its placing by 平気, in 1844 and from 1848 | Yes, 2026-09-26 |
| [nao-rekiwiki-rekichu] | 暦注; 八十八夜 and 二百十日 as Japanese 暦注; the Meiji abolition of the middle and lower registers | Yes, 2026-09-26 |
| [nao-rekiwiki-chijun] | The plain 中気 rule, compared by date; the table of 2014 | Yes, 2026-09-26 |
| [nao-rekiwiki-2033] | The 旧暦2033年問題, its three resolutions, first since 1844, no public body to decide; the 天保暦 rules applied by analogy | Yes, 2026-09-26 |
| [nao-topics-2014-2033] | The 天保暦 solstice-and-equinox rule and the 時憲暦 rule; the tables of 2014, 1984–85 and 2033–34; 中秋の名月 of 2033 under 案1 and 案2 | Yes, 2026-09-26 |
| [nao-faq-kyureki] | No official lunisolar calculation in Japan today | Yes, 2026-09-26 |
| [wikipedia-ja-kyureki-2033] | The 暦文化振興協会's 2015 recommendation of 閏11月 and its reasons. Secondary: the association's page was opened for its date, its PDFs not read | Yes, 2026-09-28 |
| [arachne-onlinecalendar] | The 六曜 of the seven months above | Yes, 2026-09-28 |
| [hko-conversion], [hko-conversion-tables], [hko-conversion-tables-moon-keyed] | The Chinese calendar's meridians; the first days of the months that changed at the Chinese meridian | Yes, 2026-09-25 to 2026-09-28 |
| [benri-com-calendar-2033] | 案2's 六曜 on four days of 2033 | Yes, 2026-09-28 |
| [nao-topics-2021-setsubun] | 節分 2021 on 2 February, the first not on the 3rd since 1984 and on the 2nd since 1897 | Yes, 2026-09-26 |
| [nao-rekiyoko-2024] | The 2024 雑節 and their instants; 立春 17:27, 春分 12:06 and 秋分 21:44 JST | Yes, 2026-09-26 for the 雑節; the terms 2026-09-25 |
| [nao-rekiyoko-2025] | The 2025 雑節 and their instants | Yes, 2026-09-26 for the 雑節 |
| [nao-rekiyoko-2026] | The 2026 雑節 and their instants | Yes, 2026-09-26 for the 雑節 |
| [nao-rekiyoko-2027-sekki] | The 2027 雑節 and their instants | Yes, 2026-09-26 |
| [nao-rekiyoko-2024-sakugenbo] | The new moons of 3 September and 3 October 2024 | Yes, 2026-09-26 |
| [wikipedia-ja-rokuyo] | The rule, the first-of-month table, the leap month, the readings, 友引 and funerals, 大安 and weddings, the history. Secondary: 『頭書長暦』, 『六曜私』, 『万暦両面鑑』 and Kanda Shigeru's study, which it cites, were not read | Yes, 2026-09-26 |
| [wikipedia-ja-shanichi] | 社日's origin and meaning; the noon rule and the earlier-day rule | Yes, 2026-09-26 |
| [wikipedia-ja-hangesho] | The eleventh day from 夏至 as the older 半夏生 | Yes, 2026-09-26 |
| [wikipedia-ja-nyubai] | 入梅 before and after 貞享暦; its dating of 80° to 天保暦, 1844, where the 暦Wiki has the 明治9年暦, which is followed | Yes, 2026-09-26 |
| [wikipedia-ja-zassetsu] | The list of nine | Yes, 2026-09-26 |
| [nao-rekiwiki-kanshi] | The sexagenary day count of the 社日 example | Not re-read here; the sexagenary document cites it |

The works the 暦Wiki cites for these rules — 渋川春海's 『貞享暦法通書』, the
新法暦書, 『星学須知』, 『永暦雑書天文大成綱目』 and the almanacs themselves in
the National Diet Library and National Archives — were not read.

## Code

`crates/hc-seasons/src/zassetsu.rs`; `crates/hc-almanac/src/rokuyo.rs`,
`moon_viewing.rs` and `lunisolar.rs`. The solar terms and the meridian are
`hc-seasons`' `solar_terms.rs` and `meridian.rs`; the day stems are
`hc_calendar::cycle::sexagenary_day`; the 旧暦 is `hc-calendars-lunar`'s
`japanese_tenpo::UNBOUNDED_PARAMETERS`, read through
`LunisolarParameters::from_fixed_unbounded`.

Anchors in `zassetsu`:
`the_zassetsu_of_2024_to_2027_fall_where_the_rekiyoko_puts_them`,
`the_longitude_zassetsu_of_2024_to_2027_are_on_the_published_minute`,
`the_shanichi_of_1873_to_1946_are_the_days_the_almanacs_printed`,
`the_almanac_of_1874_took_the_earlier_day_on_an_afternoon_tie`,
`the_spring_shanichi_of_2024_is_decided_by_six_minutes`,
`the_two_shanichi_rules_differ_only_on_an_afternoon_tie`,
`setsubun_was_the_second_of_february_in_2021`,
`the_counted_days_shift_with_the_leap_year`,
`the_doyo_entries_are_eighteen_degrees_before_their_terms`,
`the_classical_and_modern_rainy_season_rules_disagree`,
`the_two_hangesho_rules_never_differ_by_more_than_a_day`.

In `hc_almanac::rokuyo`:
`the_first_of_each_month_has_the_rokuyo_the_almanacs_print`,
`published_rokuyo_dates_come_out_right`,
`the_changed_months_are_the_ones_a_published_calendar_prints`,
`the_cycle_advances_daily_and_resets_at_every_new_moon`,
`a_leap_month_repeats_the_previous_months_rokuyo_sequence`.

In `hc_almanac::lunisolar`:
`the_lunar_new_year_is_the_first_of_the_first_month`,
`the_leap_second_month_of_2023_is_found`,
`the_months_of_2014_are_the_ones_the_observatory_tabulates`,
`the_chinese_meridian_reads_the_months_the_observatory_tabulates`; in
`hc_almanac::moon_viewing`,
`the_mid_autumn_moon_falls_where_the_almanacs_put_it`,
`the_mid_autumn_moon_of_2033_is_the_first_resolutions`; in
`hc-calendars-lunar`'s `japanese_tenpo`,
`the_continued_calendar_follows_the_observatorys_tables`.
