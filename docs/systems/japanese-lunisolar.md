# The Japanese lunisolar calendars, Senmyō to Tenpō

Backs the identifiers `japanese-senmyo`, `japanese-jokyo`, `japanese-horyaku`,
`japanese-kansei` and `japanese-tenpo` in `hc-calendars-lunar`.

## What it is

Japan kept a Chinese-style lunisolar calendar from the seventh century until
the end of 1872. The court adopted it, the bureau of the day computed it, and
what the bureau published is what the documents are dated by. Eight systems
ran in succession; the five from 862 on are the ones this library carries.

| System | In force | Computed by | Adopted |
| --- | --- | --- | --- |
| 宣明暦 Senmyō-reki | 862–1685 | Xu Ang (徐昂), Tang China, 822; the Japanese court's 陰陽寮 applied its tables unchanged | 貞観4年1月1日 = 862-02-03 Julian, after the Balhae embassy of 859 brought it [wikipedia-ja-senmyo, nao-rekiwiki-senmyo] |
| 貞享暦 Jōkyō-reki | 1685–1755 | Shibukawa Harumi (渋川春海); the shogunate created the post of 天文方 for him | 貞享2年1月1日 = 1685-02-04, by imperial proclamation [wikipedia-ja-jokyo, nao-rekiwiki-jokyo] |
| 宝暦暦 Hōryaku-reki | 1755–1798 | 土御門泰邦 (安倍泰邦), the court astrologer, with collaborators, finished 1754; the work was begun at the shogun Yoshimune's instigation | 宝暦5年1月1日 = 1755-02-11 [nao-rekiwiki-horyaku, wikipedia-ja-horyaku] |
| 寛政暦 Kansei-reki | 1798–1844 | 高橋至時 and 間重富, pupils of 麻田剛立, appointed to the shogunate's 天文方 | 寛政10年1月1日 = 1798-02-16 [nao-rekiwiki-kansei, wikipedia-ja-kansei] |
| 天保暦 Tenpō-reki | 1844–1872 | 渋川景佑, shogunate 天文方 | 天保15年1月1日 = 1844-02-18 [nao-rekiwiki-tenpo, wikipedia-ja-tenpo] |

Senmyō-reki was Chinese for seventy years and Japanese for 823. China
replaced it in 892; Japan, cut off from the newer Chinese systems and with a
court that had lost the competence to compute one, kept it until Shibukawa
Harumi showed that its solar terms stood two days from the Sun — in his
words, 「宣明暦、天に後る二日なる」 [nao-rekiwiki-senmyo, wikipedia-ja-senmyo].
His Jōkyō-reki was the first calendar computed in Japan, a reworking of the
Yuan 授時暦 with a correction for the longitude of Kyoto [nao-rekiwiki-jokyo].
Hōryaku-reki, which the court's astrologer took over from the shogunate's
astronomers, is generally judged a step backwards: the almanac for 1763 did
not carry the solar eclipse of 宝暦13年9月1日, which 西村遠里, 麻田剛立 and
others had predicted, and the system was patched in 1771 as 修正宝暦暦
[wikipedia-ja-horyaku, nao-rekiwiki-horyaku]. Kansei-reki, built on
暦象考成後編, was the first Japanese calendar to move the Sun and the Moon on
ellipses rather than on tabulated increments [wikipedia-ja-kansei].
Tenpō-reki, the last, replaced the equal division of the year by the true
solar longitude — 定気 for 平気 — and expressed its instants in apparent solar
time [nao-rekiwiki-tenpo].

The lunisolar calendar was abolished by 明治5年太政官布告第337号, issued on
明治5年11月9日 (1872-12-09), which declared that the third day of the twelfth
month of Meiji 5 would be 1 January 1873 in the solar calendar
[wikipedia-ja-meiji-kaireki, nao-rekiwiki-meiji]. The usual explanation is
Ōkuma Shigenobu's: the government had just moved its officials to monthly
pay, Meiji 6 was due an intercalary month, and thirteen salaries were one too
many; a further notice, 布告第374号, declined to pay for the two-day twelfth
month at all [nao-rekiwiki-meiji, wikipedia-ja-meiji-kaireki]. Almanacs went
on printing the old calendar by continuing Tenpō-reki's rules; the official
ones stopped in 1910 [wikipedia-ja-meiji-kaireki].

## How it works

The five systems share one structure and differ in their constants and in
three rules. The structure is the East Asian one that *Calendrical
Calculations* states for the Chinese calendar [reingold2018], read at Kyoto:

1. A month runs from conjunction to conjunction. Its first day is the day,
   in Kyoto local mean time, that contains the conjunction.
2. Month 11 contains the winter solstice. From one winter solstice to the
   next is a *suì* of twelve or thirteen new moons.
3. A suì of thirteen new moons takes an intercalary month: the first month
   in it that contains no 中気, no major solar term. The intercalary month
   repeats the number of the month before it, 閏2月 after 2月.

**Solar terms: 恒気 and 定気.** The twelve 中気 are the points where the Sun's
longitude is a multiple of 30°, 冬至 being one of them. Every system before
Tenpō-reki placed them by 恒気 (also 平気, *píngqì*): twelve equal twelfths of
the system's own tropical year, counted from its own winter solstice, so that
the 中気 are 30.437 days apart whatever the Sun does [nao-rekiwiki-jokyo,
nao-rekiwiki-kansei, whose 定数 lines read 定朔、平気]. Tenpō-reki placed them
by 定気 (*dìngqì*), the true longitude, so that the intervals are unequal and
the summer ones longest [nao-rekiwiki-tenpo]. The choice changes which month
is the intercalary one, which is why it is a parameter of the engine and not
decoration: under 恒気 a 閏正月 is ordinary, under 定気 it is nearly
impossible.

**Conjunctions: 定朔.** From 儀鳳暦 in 697 every Japanese system began the
month at the true conjunction rather than the mean one
[nao-rekiwiki-history1]. A pre-modern true conjunction is the mean
conjunction displaced by two tabulated corrections, the 朓朒 of the 日躔
table for the Sun's unequal motion and of the 月離 table for the Moon's, each
given directly as a time. Senmyō-reki's tables are in 新唐書 [xintangshu]: the
solar correction runs from zero at the solstices to a peak of 1526 parts of
8400 at the equinoxes; the lunar table's cumulative correction reaches 3172
at the seventh day and gains a further 53 in that day before turning, so its
peak is 3225 parts — the module's 3225 is that reading, and no secondary
source quotes it. The National Astronomical Observatory's 暦Wiki models the
same lunar correction as (6.29° − 1.27°)·sin *l* over the Moon's mean daily
motion, which peaks at 0.38 days, the same figure [nao-rekiwiki-getsuri].

This library reproduces 定朔 in two ways, and the difference is measured
rather than assumed (see Accuracy). The default takes the instant of the
conjunction from `hc-astro`, converted to Kyoto local mean time. The
alternative, `PARAMETERS_TABULATED`, takes the system's own mean conjunction
and displaces it by

```text
Δt = solar_equation_days · sin(anomaly from the winter solstice)
   − lunar_equation_days · sin(anomaly from perigee)
```

with the Sun's anomaly counted from the winter solstice, because that is
where the East Asian systems put the Sun's perigee (盈初縮末). The
amplitudes are Senmyō-reki's own 1526/8400 and 3225/8400 days for
Senmyō-reki, and for the three Edo systems, whose tables this library does
not have, the modern first-order equation of centre, 1.9148°, and the Moon's
equation of centre less the evection, 6.2886° − 1.2740° [meeus1998], each
divided by the mean elongation rate. Nothing else of the Moon's motion is
modelled, because no East Asian system before the Western tables modelled
it.

**進朔.** Senmyō-reki holds a late conjunction over: when the conjunction
falls after a limit in the day, the month begins on the *next* day, so that
no old moon can be seen on the first. 新唐書 states the rule as 「凡定朔小餘，
秋分後，四分之三已上，進一日」 — after the autumn equinox, three quarters of
the day — with a smaller limit after the spring equinox, reduced by a fifth
of the change in the twilight interval since the equinox, and no advance
when an eclipse would be seen at first contact [xintangshu]. The 暦Wiki gives
the same limit as 6300 of 8400 parts and says the rule was dropped in China
with 授時暦 [nao-rekiwiki-shinsaku]; Japan, still on Senmyō-reki, kept it
until Jōkyō-reki, which follows 授時暦 and has none. This library carries a
single limit for Senmyō-reki and none for the later systems; the seasonal
variant and the eclipse exception are not modelled. The rule moves about a
quarter of Senmyō-reki's month boundaries, so a reconstruction without it is
not the published calendar.

**里差, the meridian.** Chinese tables give instants for the Chinese capital.
Shibukawa introduced a 里差, a longitude correction, of 5刻 (five hundredths
of a day) from 大都, the Yuan capital at Beijing, to Kyoto [nao-rekiwiki-jokyo,
nao-rekiwiki-meridian]; he also recorded that Senmyō-reki's tables were 7刻
from Chang'an and had been applied in Japan with no correction at all
[nao-rekiwiki-meridian, citing Shibukawa's 貞享暦 p. 76; nao-rekiwiki-senmyo].
Every system from Jōkyō-reki on is therefore Kyoto's, and the library
computes all five at Kyoto local mean time, 135°46′E. For Senmyō-reki that
is 0.075 days east of the meridian its instants were really for; the
library does not model the Chang'an meridian but absorbs the offset into the
進朔 limit, which is 0.80 rather than the sourced 0.75, and the module
documentation gives the agreement rate for the other arrangement.

**The constants.** Each system's tropical year (歳実 or 歳周), synodic month
(朔実 or 朔策) and anomalistic month (近点月), in days, as this library
carries them and as the sources re-read on 2026-09-25 give them:

| System | 歳実 | 朔実 | 近点月 | Source |
| --- | --- | --- | --- | --- |
| 宣明暦 | 3068055/8400 = 365.244643 | 248057/8400 = 29.530595 | 231458.19/8400 = 27.554546 | 統法 8400, 章歳, 章月 and 曆周 in [xintangshu]; the decimals in [nao-rekiwiki-senmyo] |
| 貞享暦 | 365.241696 | 29.530590 | 27.554600 | [nao-rekiwiki-jokyo]; the year is 授時暦's with 消長法 applied, and the sidereal year 365.256696 is that plus 0.015 |
| 宝暦暦 | 365.241556 | 29.530590 | 27.554600 | [nao-rekiwiki-horyaku], the value as promulgated; 修正宝暦暦 of 1771 made it 365.241626, six seconds a year, which the library does not carry |
| 寛政暦 | 365.242347 | 29.530584 | 27.554570 | [nao-rekiwiki-kansei] gives 365.242347071 and derives both months from the daily mean motions: 29.530584 = 360 / (13.1763981114 − 0.9856469352) and 27.554570 = 360 / (13.1763981114 − 0.1114147178). The modern 27.554550 differs by 2 × 10⁻⁵ days, which moves no figure below at two decimals, because the anomalistic month enters only as the phase of a sine |
| 天保暦 | — | — | — | Not used: the library computes Tenpō-reki from the true Sun and Moon. The page gives 365.242233952291 and 29.530588 [nao-rekiwiki-tenpo] |

**Worked example: 貞享2年1月1日.** Jōkyō-reki took effect on its own new
year, and the sources give the day: the Japanese Wikipedia table for 貞享
prints 1685/2/4 in the Gregorian row and 1685/1/25 in the Julian one
[wikipedia-ja-era-tables]. To follow the library by hand, use fixed days
(RD 1 = 0001-01-01 proleptic Gregorian) in Kyoto local mean time.

1. *The solstice.* The true December solstice of 1684 at Kyoto is
   RD 615059.229, 1684-12-21 at 05:30. Jōkyō-reki's solstice epoch is that
   less the fitted 0.40 days, RD 615058.829, so the system's 冬至 is on
   1684-12-20. The 中気 then follow at intervals of 365.241696/12 =
   30.436808 days: 大寒 at RD 615089.27 (1685-01-20), 雨水 at 615119.70
   (1685-02-19), 春分 at 615150.14 (1685-03-22).
2. *The suì.* The next solstice is at RD 615424.07, 1685-12-21. The
   conjunctions from `hc-astro`, in Kyoto time, fall on 1685-01-05 (RD
   615074), 1685-02-04 (RD 615104.048, at 01:09), 1685-03-05, … and
   1685-11-26 (RD 615399), whose month contains the solstice of 1685 and is
   therefore month 11. From the month after the 1684 solstice's month to
   that one is twelve months, so the suì has no intercalary month.
3. *The new year.* With no intercalary month, the year begins at the second
   new moon after the solstice: 1685-01-05 begins the twelfth month of the
   old year and 1685-02-04 the first month of the new. Check by the 中気:
   the month from 1685-02-04 to 1685-03-04 contains 雨水 and no other major
   term, and 雨水 is the 中気 of the first month.
4. *The day.* There is no 進朔 after 1684, so a conjunction at 01:09 begins
   its month that day. RD 615104 is 1685-02-04 Gregorian, 1685-01-25 Julian,
   and the library returns year 1685, month 1, day 1 — the nengō 貞享 is
   `hc-calendars-regional`'s business. The test
   `jokyo_took_effect_on_the_fourth_of_february_1685` holds the date and
   `senmyo_ends_on_the_day_before_the_jokyo_reform` holds the day before
   it, 貞享元年12月30日, as Senmyō-reki's last.

**Worked example: the intercalary month of 正治2年 (1200), with 進朔.** The
table this library is measured against gives 1200 thirteen months with
閏2月, beginning RD 437950 = 1200-01-25 Gregorian (1200-01-18 Julian), and a
dated event in the Wikipedia articles is 正治2年閏2月11日 = 1200-03-27 Julian.

1. *The solstice.* Senmyō-reki's solstice epoch is RD 314464.559, the true
   solstice of 861 at Kyoto plus the fitted 0.20 days, and its year is
   365.244643 days, so the 338th solstice after it is at RD 437917.248,
   1199-12-23 Gregorian. The 中気 follow every 30.437054 days: 春分 at
   RD 438008.56 (1200-03-23), 穀雨 at 438038.996 (23:54 on 1200-04-22).
2. *The suì.* The next solstice is at RD 438282.49, 1200-12-22. The first
   conjunction after the 1199 solstice is at RD 437920.912, 1199-12-26 at
   21:53 — past the 進朔 limit, so the twelfth month of 1199 begins on
   1199-12-27. The month containing the 1200 solstice begins on 1200-12-15.
   That is thirteen months, so the suì takes an intercalary month.
3. *The intercalary month.* The month beginning 1200-03-24 (conjunction
   RD 438009.225, 05:24) runs to 1200-04-21. 春分 fell the day before it
   began and 穀雨 falls on the day after it ends, so it contains no 中気; it
   is the first such month in the suì, and it is 閏2月. Its eleventh day is
   1200-04-03 Gregorian, 1200-03-27 Julian, which is the dated event.
4. *進朔 elsewhere in the year.* The conjunction of RD 438156.974 falls at
   23:22 on 1200-08-18, so 7月 begins on 1200-08-19, as the table has it.
   The conjunction of RD 437979.801 falls at 19:13 on 1200-02-23, a fraction
   of 0.801, just past the library's limit of 0.80, so the library begins
   2月 on 1200-02-24 — and the table begins it on 1200-02-23. That is one of
   the disagreements the next sections count: a conjunction within minutes
   of the limit, where the bureau's own tables evidently put it earlier in
   the day than the modern instant does. From 閏2月 on, the two agree again.

## What is carried

- **Identifiers**, all in `hc-calendars-lunar` and all with the month as
  `Month { ordinal, leap }`: `japanese-senmyo` (module
  `japanese_historical::senmyo`), `japanese-jokyo` (`::jokyo`),
  `japanese-horyaku` (`::horyaku`), `japanese-kansei` (`::kansei`) and
  `japanese-tenpo` (`japanese_tenpo`). CLDR names none of them, and none is
  `japanese`, which is the Gregorian calendar with nengō years in
  `hc-calendars-regional`.
- **Ranges** that abut exactly, each system refusing every day outside its
  period of use: Senmyō 862-02-03 Julian (RD 314512, proleptic Gregorian
  862-02-07) to 1685-02-03; Jōkyō 1685-02-04 to 1755-02-10; Hōryaku
  1755-02-11 to 1798-02-15; Kansei 1798-02-16 to 1844-02-17; Tenpō
  1844-02-18 to 1872-12-31, the last day the calendar ever named. Between
  them they name every Japanese day from the adoption of Senmyō-reki to the
  abolition, with no gap and no overlap.
- **Year numbering**: the Gregorian year in which the lunisolar year begins,
  a convention of this library and labelled as one; the sexagenary year is
  the shared East Asian cycle. Nengō are not carried here.
- **The meridian**: Kyoto local mean time, 135°46′E, for all five; a second
  row for Japan Standard Time from 1888 exists only so that the table states
  the whole history.
- **Two parameter sets per pre-Tenpō system.** `PARAMETERS`, the default,
  takes the conjunction from `hc-astro` and everything else — the solstice,
  the 恒気 terms, the intercalation, 進朔 — from the system's own constants.
  `PARAMETERS_TABULATED` takes the conjunction from the system's own
  amplitudes as well. Both are exported and both are measured, because the
  gap between them is the most informative number the module produces.
- **A solstice phase per system, fitted.** A system's winter solstice came
  from its 上元積年, an arithmetic chain reaching back millions of years, not
  from an observation in the adoption year, and that chain was not
  recovered. So each model's solstice epoch is the true solstice of the year
  before adoption, at Kyoto, plus one fitted scalar: +0.20 days for
  Senmyō-reki, −0.40 for Jōkyō-reki, −0.50 for Hōryaku-reki, −0.30 for
  Kansei-reki, each chosen to maximise agreement with the published table
  over the system's whole life. Senmyō-reki's 進朔 limit of 0.80 is the
  second fitted scalar. Everything else is sourced. The explanation the
  module offers for the three Edo phases — that a solstice found by gnomon
  shadow comes out a third to half a day early — has no source named here.
- **Tenpō-reki** carries no constants of its own: it is the engine's default
  rule, true solar longitude and true conjunction at Kyoto, bounded at the
  abolition. `UNBOUNDED_PARAMETERS` continues the same rule past 1872, which
  is what modern almanacs do and what the 旧暦 of 六曜 and 十五夜 is keyed to;
  it is not a historical calendar and is named so it cannot be mistaken for
  one.
- **Not carried:** 元嘉暦 (604–697), 儀鳳暦 (697–764), 大衍暦 (764–862) and
  五紀暦 (858–862, used alongside 大衍暦). The validation table begins in
  862, the adoption dates before it are uncertain by years
  [nao-rekiwiki-history1], 元嘉暦 used 平朔 where the others used 定朔, and
  the 進朔 limits differ from system to system. Four calendars nothing could
  check would be worse than none. The module records period constants for
  them (元嘉暦 with 日法 752, 儀鳳暦 and 五紀暦 with 総法 1340, 大衍暦 with
  通法 3040) without naming a source; this document does not repeat them as
  fact. Also not carried: 修正宝暦暦's revised year; the seasonal 進朔 limit
  and its eclipse exception; the courts' adjustments by decree.

## Accuracy

**The reference.** `crates/hc-calendars-lunar/tests/data/japanese_month_lengths.txt`
holds the first day and the length of every month of every lunisolar year
from 862 to 1843: 982 years, 12 146 months, 300 592 days. It was harvested
from the 西暦との対照表 in the Japanese Wikipedia article for each of the 242
元号 of the period, which tabulate the Julian and Gregorian date of the first
day of every month and mark the 29-day months [wikipedia-ja-era-tables]. The
articles do not name their source; the tables are transcriptions of 内田正男
『日本暦日原典』 [uchida1975], the standard computed reconstruction of the
Japanese calendar from 445 to 1872 [wikipedia-ja-nihon-rekijitsu-genten],
and that attribution is this repository's, not the articles'. The harvest
was de-duplicated across the Northern and Southern Court overlap and
cross-checked three ways before use, and the test
`the_published_table_is_internally_consistent` repeats what can be repeated:
every one of the 12 145 intervals between consecutive month starts is 29 or
30 days; the tables' own 小の月 marks agree with those intervals in every
case; and the Julian and Gregorian columns, given independently, agree in all
3 959 rows that carry both. It is one source family. A separate scrape of
2 096 dated events from unrelated Wikipedia articles agrees with it on 96.9%
of cases, the disagreements being scattered single-article errors; eighteen
of those events, one or two a century, are held in
`dated_events_scattered_over_nine_centuries_come_out_right`. That is
corroboration, not independence.

**The measurement**, by `every_system_reproduces_the_published_table_at_the_stated_rate`,
which prints these figures under `cargo test -p hc-calendars-lunar --
--nocapture` and asserts floors a little below them:

| System | New years | Intercalary months | Month starts | Individual days |
| --- | --- | --- | --- | --- |
| 宣明暦 Senmyō | 94.65% | 93.68% | 96.43% | 96.39% |
| 貞享暦 Jōkyō | 97.14% | 100.00% | 98.85% | 98.83% |
| 宝暦暦 Hōryaku | 97.67% | 90.70% | 97.74% | 97.73% |
| 寛政暦 Kansei | 97.83% | 97.83% | 99.12% | 99.11% |

Tenpō-reki is not in the table, which ends in 1843, and is anchored rather
than measured: its first day, 1844-02-18, the first day of its last month,
1872-12-30, and its last day, 1872-12-31, come out right, and the year after
the abolition comes out with thirteen months and 閏6月.

**The pattern of disagreement.** Not random. Almost every failure is a month
boundary landing one day early or late, which moves the day-of-month for that
month and, if the boundary falls near a 中気, can move the intercalary month
and with it the following new year. Three causes, in order of size:

1. *The conjunction.* A pre-modern 定朔 is a table lookup, fourteen tabulated
   daily increments per half anomalistic month, and this library has the
   tables for Senmyō-reki alone. The residual between the modern instant and
   the bureau's is a few tenths of a day, which is exactly the width of a
   day boundary; `the_omitted_lunar_inequalities_are_the_size_the_documentation_claims`
   keeps the residual of the tabulated model under a quarter of a day on
   average and under 0.7 at worst.
2. *The 暦元 solstice.* The fitted phase above. That three independently
   derived Edo systems all want a solstice a third to half a day early is
   itself a finding, whatever its explanation.
3. *Decree.* The court occasionally adjusted a promulgated month by hand,
   and Senmyō-reki's later centuries accumulated errors the bureau sometimes
   corrected. No computation reproduces those.

Two scalars fitted per calendar against 300 592 days of independent data is
calibration rather than curve-fitting, but it is fitting, and the code
labels it as such wherever it appears.

**Two things measured rather than assumed.**

*The historical tropical year is worth twenty-five points.* Over every fifth
year of Senmyō-reki's 823 — 165 years, enough to be decisive and cheap enough
for a debug build — the system's own 歳実 under 恒気 places the intercalary
month correctly 92.1% of the time. The same machinery with modern apparent
solar terms (定気) manages 66.7%, and the same 恒気 rule on the modern
tropical year manages 64.2%. A reconstruction on modern solar theory would
not be a worse Senmyō-reki; it would be a different calendar.
`the_systems_own_tropical_year_is_what_places_the_intercalary_month` asserts
the gap. The drift itself is measured in
`senmyos_solar_terms_drift_a_day_and_a_half_away_from_the_sun`: 0.0018 days
a year for 823 years is a day and a half, on top of whatever the Chinese
original already carried in 862.

*Modern astronomy reproduces these calendars' conjunctions better than their
own tables do*, which was not the expected result. Approximating a system's
日躔 and 月離 tables by a single sine each places the published month starts
at 89.9% for Senmyō-reki and 91.0% for Kansei-reki, against 96.7% and 99.1%
for the true conjunction over the same sample
(`the_systems_own_conjunction_tables_do_worse_than_the_true_conjunction`).
The inference — that the bureaux computed conjunctions to within an hour or
two and it was their solar theory that was two days out — is this
repository's reading of that measurement and not a statement any source
makes. It is why the default parameter set takes the conjunction from
`hc-astro` and everything else from the system, and why both sets are kept.
The gap is largest for Kansei-reki, for a reason worth recording: it solved
Kepler's equation, so a single sine reproduces it worse than it reproduces
the ninth-century system.

**The meridian for Senmyō-reki.** With the sourced limit of 0.75 and a
Chang'an meridian, Senmyō-reki agrees with the table on 95.6% of days; with
Kyoto and the fitted 0.80, on 96.4%. Both are measured in the integration
tests, so the trade is visible rather than asserted.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [xintangshu] | Senmyō-reki's 統法 8400, 章歳 3068055, 章月 248057, 曆周 231458 秒 19; the 日躔 table's peak 1526 at 春分 and 秋分; the 月離 table's 3172 at the seventh day plus 53, which is the 3225 the module carries; the 進朔 rule 「秋分後，四分之三已上，進一日」 and its spring-half and eclipse conditions | Yes, 2026-09-25, on Wikisource |
| [nao-rekiwiki-senmyo] | The decimal constants, 27.554546 for 曆周; 822 and 892 in China, 862 to 1684 in Japan, 823 years; Shibukawa's 「天に後る二日」; the remark that a 里差 of 7刻 would bring it near the modern calendar | Yes, 2026-09-25 |
| [nao-rekiwiki-jokyo] | 365.241696, 29.530590, 27.554600, the sidereal year 365.256696; 定朔、平気; the 里差 of 5刻 from 大都 to Kyoto; 授時暦 with 消長法 | Yes, 2026-09-25 |
| [nao-rekiwiki-horyaku] | 365.241556 and 365.256556 as promulgated, 365.241626 for 修正宝暦暦 from 1771; 選者 安倍泰邦 (土御門泰邦); 1755 to 1797 | Yes, 2026-09-25 |
| [nao-rekiwiki-kansei] | 365.242347071, 29.530584 and the derived 27.554570; 選者 高橋至時、間重富; 定朔、平気 | Yes, 2026-09-25 |
| [nao-rekiwiki-tenpo] | 定朔、定気 and apparent solar time; 1844 to 1872; 選者 渋川景佑; the constants this library does not use | Yes, 2026-09-25 |
| [nao-rekiwiki-shinsaku] | What 進朔 is for; the limit 6300 of 8400; its abolition in 授時暦 | Yes, 2026-09-25 |
| [nao-rekiwiki-getsuri] | The lunar correction as (A − B)·sin *l* with A = 6.29°, B = 1.27°, peaking at 0.38 days | Yes, 2026-09-25 |
| [nao-rekiwiki-meridian] | Kyoto as the reference meridian of the Edo calendars; Shibukawa's figures of 5刻 to 大都 and 7刻 to Chang'an | Yes, 2026-09-25 |
| [nao-rekiwiki-history1] | The sequence 元嘉暦, 儀鳳暦, 大衍暦, 五紀暦; that their adoption dates are uncertain; 儀鳳暦's introduction of 定朔 | Yes, 2026-09-25 |
| [nao-rekiwiki-meiji] | The decree of 明治5年11月9日; 布告第374号 and the unpaid twelfth month | Yes, 2026-09-25 |
| [wikipedia-ja-era-tables] | The 西暦との対照表 of every era article from 貞観 to 天保, the origin of the validation table; 貞享's row for 1685 | Yes; harvested for the table, and 貞享 re-read 2026-09-25 |
| [uchida1975] | The reconstruction the era tables transcribe | Not read directly |
| [wikipedia-ja-nihon-rekijitsu-genten] | Uchida's coverage, 445 to 1872, and editions | Yes, 2026-09-25 |
| [wikipedia-ja-senmyo] | Xu Ang, 822; the Balhae embassy of 859; adoption 862-02-03; why it was not replaced; the two-day error | Yes, 2026-09-25 |
| [wikipedia-ja-jokyo] | Shibukawa; the proclamation and the creation of the 天文方; 授時暦 against the court's 大統暦 | Yes, 2026-09-25 |
| [wikipedia-ja-horyaku] | 土御門泰邦 and his collaborators; Yoshimune; the eclipse of 1763 and its predictors; 修正宝暦暦 | Yes, 2026-09-25 |
| [wikipedia-ja-kansei] | 高橋至時 and 間重富, 麻田剛立's pupils; 暦象考成後編 and the ellipse for Sun and Moon | Yes, 2026-09-25 |
| [wikipedia-ja-tenpo] | 渋川景佑; adoption 1844-02-18; 定気; the abolition | Yes, 2026-09-25 |
| [wikipedia-ja-meiji-kaireki] | 太政官布告第337号; Ōkuma's account; the old calendar in almanacs to 1910 | Yes, 2026-09-25 |
| [reingold2018] | The lunisolar structure: the suì, month 11, the no-中気 rule, the sexagenary cycle | Not re-read for this document; the engine cites it |
| [meeus1998] | The equation-of-centre amplitudes 1.9148°, 6.2886° and 1.2740° substituted for the Edo systems' tables | Not read for this document; the module cites it |

Statements in the module documentation that no source read here supports,
recorded so that they are not mistaken for sourced: that Shibukawa spent
twenty years on the reform and that his was the third proposal; that he
called 進朔 groundless; that the Edo solstice phases are the bias of the
gnomon-shadow method; that 暦象考成後編 derives from Lalande (it predates
Lalande's tables; the sources read say only Kepler's ellipse); the period
constants of the four pre-862 systems; and the quotation 「天行二日を違う」,
where the 暦Wiki has 「宣明暦、天に後る二日なる」.

## Code

`crates/hc-calendars-lunar/src/japanese_historical.rs` (the four pre-Tenpō
systems, as submodules `senmyo`, `jokyo`, `horyaku`, `kansei`),
`crates/hc-calendars-lunar/src/japanese_tenpo.rs`, and the engine they
configure, `crates/hc-calendars-lunar/src/lunisolar.rs`, whose
`MeanMotionModel` holds a system's constants and whose `LunisolarParameters`
holds the meridian, the term mode and the range.

Anchors in the module: `senmyo_states_its_constants_as_the_eight_thousand_four_hundredths_it_used`,
`senmyo_took_effect_on_the_third_of_february_862_in_the_julian_calendar`,
`jokyo_took_effect_on_the_fourth_of_february_1685`,
`horyaku_took_effect_on_the_eleventh_of_february_1755`,
`kansei_took_effect_on_the_sixteenth_of_february_1798`,
`kansei_ends_on_the_day_before_tenpo_takes_over`,
`the_four_systems_tile_the_years_862_to_1844_without_a_gap`,
`the_epochs_are_the_sky_of_the_adoption_year_and_nothing_later`,
`shinsaku_holds_a_late_conjunction_over_to_the_next_day`,
`senmyos_solar_terms_drift_a_day_and_a_half_away_from_the_sun`; in
`japanese_tenpo`: `the_calendar_took_effect_on_the_eighteenth_of_february_1844`,
`the_last_day_is_the_second_of_the_twelfth_month_of_meiji_five`,
`the_year_after_the_abolition_would_have_had_thirteen_months`.

The measurement, in `crates/hc-calendars-lunar/tests/japanese_historical.rs`:
`the_published_table_is_internally_consistent`,
`every_system_reproduces_the_published_table_at_the_stated_rate`,
`the_systems_own_tropical_year_is_what_places_the_intercalary_month`,
`the_systems_own_conjunction_tables_do_worse_than_the_true_conjunction`,
`the_four_named_anchors_from_the_documents_come_out_right`,
`dated_events_scattered_over_nine_centuries_come_out_right`,
`the_four_systems_and_tenpo_tile_the_millennium_without_a_gap`.
