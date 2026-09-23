# `hc-calendars-lunar`

Lunar and lunisolar calendars for [`hyper-calendar`]: the Hijri family, the
Hebrew calendar, the Tibetan Phugpa calendar, the East Asian lunisolar
calendars of China, Korea and Vietnam, and the five successive lunisolar
calendars Japan used between 862 and 1872.

Every calendar implements `hc_calendar::Calendar`, so every one of them
converts through `Rd`, the Rata Die fixed day, and none of them knows the
others exist.

## What is here

| Module | Identifier | Kind | Range |
|---|---|---|---|
| `islamic_civil` | `islamic-civil` | arithmetic | 1–9999 AH |
| `islamic_astronomical` | `islamic-tbla` | arithmetic | 1–9999 AH |
| `tabular::FATIMID` | `islamic-fatimid` | arithmetic | 1–9999 AH |
| `islamic_umalqura` | `islamic-umalqura` | published table | **1300–1600 AH only** |
| `islamic_observational` | `islamic-rgsa` | prediction | 1900–2100 CE |
| `hebrew` | `hebrew` | arithmetic | AM 1–9999 |
| `tibetan` | `tibetan` | arithmetic (Phugpa) | 1000–3000 |
| `chinese` | `chinese` | astronomical | 1645–2150 CE |
| `dangi` | `dangi` | astronomical | 1645–2150 CE |
| `vietnamese` | `vietnamese` | astronomical | 1645–2150 CE |
| `japanese_historical::senmyo` | `japanese-senmyo` | historical | 862-02-03 (Julian) to 1685-02-03 |
| `japanese_historical::jokyo` | `japanese-jokyo` | historical | 1685-02-04 to 1755-02-10 |
| `japanese_historical::horyaku` | `japanese-horyaku` | historical | 1755-02-11 to 1798-02-15 |
| `japanese_historical::kansei` | `japanese-kansei` | historical | 1798-02-16 to 1844-02-17 |
| `japanese_tenpo` | `japanese-tenpo` | astronomical | 1844-02-18 to **1872-12-31** |

The five Japanese ranges abut exactly. Between them they name every Japanese
day from the adoption of Senmyō-reki to the abolition of the lunisolar
calendar, with no gap and no overlap, and each refuses every day outside its
own period of use.

`tabular` and `lunisolar` are the two engines those modules configure.
`tabular` takes an epoch and one of four intercalation schemes, so eight
tabular Hijri calendars are reachable, not two. `lunisolar` takes a meridian
history, an epoch, a year numbering, a choice of true or mean solar terms
and — the whole of what makes the Japanese historical calendars possible —
an optional `MeanMotionModel` holding one system's own period
constants. Nine lunisolar calendars, one algorithm.

## Japan's historical calendars

A calendar that ran for eight centuries on ninth-century constants is not the
sky, and must not be computed as if it were. Senmyō-reki's tropical year is
3.4 minutes too long; over the 823 years Japan used it, its solar terms slid
about two days away from the Sun, and *that drift is why the Jōkyō reform
happened*. A Senmyō-reki driven by a modern solar series would have no error
to demonstrate.

So each of these four carries its own 歳実 and 朔実, its own 恒気 (mean, equal)
major solar terms, and — for Senmyō-reki — its own 進朔, the rule that holds a
late conjunction over to the following day.

| System | In force | 歳実 (days) | 朔実 (days) | 近点月 | Source for the constants |
|---|---|---|---|---|---|
| 宣明暦 Senmyō | 862–1685 | 3068055/8400 = 365.244643 | 248057/8400 = 29.530595 | 231458.19/8400 = 27.554546 | NAO 暦計算室 暦Wiki「宣明暦」; 新唐書 卷030上 長慶宣明曆 |
| 貞享暦 Jōkyō | 1685–1755 | 365.241696 | 29.530590 | 27.554600 | NAO 暦計算室 暦Wiki「貞享暦」 |
| 宝暦暦 Hōryaku | 1755–1798 | 365.241556 | 29.530590 | 27.554600 | NAO 暦計算室 暦Wiki「宝暦暦」 |
| 寛政暦 Kansei | 1798–1844 | 365.242347 | 29.530584 | *modern 27.554550* | NAO 暦計算室 暦Wiki「寛政暦」 |

Senmyō-reki additionally uses its 日躔 and 月離 tables' peaks, 1526/8400 and
3225/8400 of a day, and its 進朔限 of 6300/8400. The 進朔限 is sourced; **the
two 朓朒 peaks are a reading of the tables in 新唐書 and are quoted literally
by no secondary source reachable from here**, though NAO's 暦Wiki models the
lunar correction as (6.29° − 1.27°)·sin l over the Moon's daily motion, which
peaks at the same 0.38 days.

Every constant in the code carries its source in a comment.

### Measured agreement

`tests/data/japanese_month_lengths.txt` holds the first day and the length of
every month of every year from 862 to 1843 — **982 years, 12 146 months,
300 592 days** — taken from the 西暦との対照表 published in the Japanese
Wikipedia article for each 元号, which are transcriptions of 内田正男
『日本暦日原典』 (雄山閣, 1975). The file's header records how it was gathered
and the three cross-checks it passed: every interval between consecutive month
starts is 29 or 30 days, the tables' own 小の月 marks agree with those
intervals in every case, and the tables' independently given Julian and
Gregorian columns agree in all 3 959 rows that carry both.

It is one source family. A separate scrape of 2 096 dated events from
unrelated Japanese Wikipedia articles agrees with it on 96.9% of cases, the
disagreements being scattered single-article errors, but that is corroboration
and not independence. This README says so rather than implying otherwise.

Against that table, with `cargo test -p hc-calendars-lunar --
--nocapture`:

| System | New years | Intercalary months | Month starts | Individual days |
|---|---|---|---|---|
| 宣明暦 Senmyō | 94.65% | 93.68% | 96.43% | **96.39%** |
| 貞享暦 Jōkyō | 97.14% | 100.00% | 98.85% | **98.83%** |
| 宝暦暦 Hōryaku | 97.67% | 90.70% | 97.74% | **97.73%** |
| 寛政暦 Kansei | 97.83% | 97.83% | 99.12% | **99.11%** |

The four dates usually asked for all come out right: 本能寺の変 天正10年6月2日 =
1582-06-21 Julian, 関ヶ原 慶長5年9月15日 = 1600-10-21, 赤穂事件討ち入り
元禄15年12月14日 = 1703-01-30, and 貞享2年1月1日 = 1685-02-04.

### The pattern of the disagreement

Not random. Almost every failure is a month boundary landing one day early or
late, which then moves the day-of-month for that month and, if it falls near a
中気, can move the intercalary month and so the following new year. Three
causes, in order of size:

1. **The conjunction.** A pre-modern 定朔 is a table lookup — fourteen
   tabulated daily increments per half anomalistic month — and this crate has
   those tables for Senmyō-reki alone. The residual is a few tenths of a day,
   which is exactly the width of a day boundary.
2. **The 暦元 solstice.** A system's winter solstice came from its 上元積年,
   an arithmetic chain reaching back millions of years, not from an
   observation in the adoption year. That chain was not recoverable, so the
   phase is **fitted**: one scalar per calendar, +0.20 days for Senmyō-reki
   and −0.30 to −0.50 for the three Edo systems. That three independently
   derived Edo systems all want a solstice a third to half a day early is
   itself a finding — they determined it by gnomon shadow, and this is the
   size of that method's known bias.
3. **Decree.** The Japanese court occasionally adjusted a promulgated month by
   hand, and Senmyō-reki's later centuries accumulated errors the bureau
   sometimes corrected. No computation reproduces those.

Two fitted scalars per calendar against 300 592 days of independent data is
calibration rather than curve-fitting, but it is fitting and the code labels
it as such everywhere it appears.

### Two things this crate measured rather than assumed

**The historical tropical year is worth twenty-five points.** Over every fifth
year of the table — 165 years, enough to be decisive and cheap enough to run
in a debug build — Senmyō-reki's own 歳実 places the intercalary month
correctly 92.1% of the time. The same machinery with modern apparent (定気)
solar terms manages 66.7%, and the same 恒気 rule on the *modern* tropical
year manages 64.2%. A reconstruction on modern solar theory would not be a
worse Senmyō-reki — it would be a different calendar.
`the_systems_own_tropical_year_is_what_places_the_intercalary_month` asserts
this, so the claim cannot quietly stop being true.

**Modern astronomy reproduces these calendars' *conjunctions* better than
their own tables do**, which was not the expected result. Approximating a
system's 日躔 and 月離 tables by a single sine each places the published month
starts at 89.9% for Senmyō-reki and 91.0% for Kansei-reki, against 96.7% and
99.1% for the true conjunction over the same sample. Those bureaux computed conjunctions
to within an hour or two — it was their solar theory that was two days out. So
the default parameter set takes the conjunction from `hc-astro` and everything
else from the system, and each module also exports a `PARAMETERS_TABULATED`
that uses the system's own amplitudes. Both are measured, and both numbers are
above.

### What is not here

**元嘉暦 (604–697), 儀鳳暦 (697–764), 大衍暦 (764–862) and 五紀暦 (858–862)
are not implemented.** Their period constants are recoverable — 元嘉暦 is 日法
752 with 222070/608 and 22207/752; 儀鳳暦 and 五紀暦 share 総法 1340 with
489428/1340 and 39571/1340; 大衍暦 is 通法 3040 with 1110343/3040 and
89773/3040 — but the validation table begins in 862, the adoption dates before
then are contested by years, 元嘉暦 used 平朔 where the others used 定朔, and
their 進朔限 differ from system to system. Four more calendars that nothing
could check would be worse than none.

## Accuracy

**Arithmetic calendars — exact.** The tabular Hijri and Hebrew calendars are
counting rules, and this is those rules. The tabular Hijri implementation is
checked against the closed form in Reingold and Dershowitz, *Calendrical
Calculations*, for every month of the first three thousand years. The Hebrew
implementation reproduces 1 Tishrei 5784 = 2023-09-16 and 15 Nisan 5784 =
2024-04-23, keeps Rosh Hashanah off Sunday, Wednesday and Friday for all
9 999 years, and gives every year one of the six permitted lengths.

Exact is not the same as astronomically right. The Hebrew molad is 0.4 seconds
longer than the true mean synodic month, so it drifts about a day later every
216 years; the tabular Hijri month is 2.9 seconds short, so it drifts a day in
about 2 500 years. Both figures are in the module documentation.

**The Umm al-Qura table — exact where it reaches, and nowhere else.** See
below.

**Astronomical calendars — good to a day, usually.** Conjunctions come from
`hc-astro` and land within about a minute. Solar longitude comes from the
VSOP87 series, good to about 1″, whose solstice instants land within the
minute the almanacs round to. When a conjunction or a solstice falls within
about a minute of local midnight, the day assigned
can be wrong by one — and a wrong day for a zhōngqì can move a leap month by a
whole month. The crate tests the published new years it can check (Chinese New
Year 1900, 2000, 2020–2026; Seollal 1988 and 2024; Tết 1968, 1985 and 2024;
the Tenpō dates of 1844 and 1872) and they all come out right, but that is
evidence, not a guarantee.

**Historical calendars — measured, and the measurement is above.** The four
Japanese systems in `japanese_historical` do not use modern solar theory at
all, and the whole of what they claim is the agreement rate in that section:
96.4% to 99.1% of individual days against 982 years of published table. They
are the only calendars in this crate that aim at what a bureau published
rather than at what a rule gives.

## Meridian conventions

The day boundary is read in local time, which is why these calendars disagree
with each other at all. The tables implemented are those of *Calendrical
Calculations*:

| Calendar | Offsets |
|---|---|
| Chinese | Beijing local mean time (116°25′E) before 1929; UT+8 from 1929 |
| Dangi | Seoul local mean time (126°58′E) before 1908; UT+8:30 1908–1911; UT+9 1912–1953; UT+8:30 1954–1960; UT+9 from 1961 |
| Vietnamese | UT+8 before 1968; UT+7 from 1968 |
| Japanese (all five) | Kyoto local mean time (135°46′E) before 1888; UT+9 from 1888 |

One caveat on that last row. Kyoto is right from Jōkyō-reki onward, because
Shibukawa's 里差 — the correction from the Chinese capital's meridian to
Kyoto's — was one of the 1685 reform's headline changes. Senmyō-reki had no
such correction: Japan applied the Chinese tables unadjusted for eight
centuries, so its true reckoning was Chang'an's, 0.075 days west. That offset
is not modelled as a meridian here; it is absorbed into Senmyō-reki's fitted
進朔限, and the module documentation says so and gives the agreement rate for
the other arrangement.

These are not decoration. Over 1900–2049 the Korean and Chinese new years fall
on different days nine times, including 1988, when Seollal was 18 February and
Chinese New Year 17 February. The Vietnamese table reproduces the best-known
case of all: the DRV moved from UT+8 to UT+7 on 1 January 1968, so the North
kept Tết on 29 January 1968 and the South, still on UT+8, on 30 January. Both
are tested. So is Tết 1985, where the same hour moved a zhōngqì, moved the leap
month, and put Tết a *whole lunation* before Chinese New Year.

## Reference data

**The Umm al-Qura table** is 301 `u16` values, one per Hijri year from 1300 to
1600, each carrying twelve bits for which months have 30 days. It was
extracted from the platform's own `islamic-umalqura` implementation
(Foundation's `Calendar(identifier: .islamicUmmAlQura)`, which is ICU's
`UMALQURA` data) by asking for the first day of every month in that span and
differencing. It was then spot-checked against dates published by the Saudi
authorities: 1 Muḥarram 1300 = 1882-11-12, 1 Muḥarram 1445 = 2023-07-19,
1 Ramaḍān 1445 = 2024-03-11, 1 Shawwāl 1445 = 2024-04-10, 1 Ramaḍān 1446 =
2025-03-01, 1 Muḥarram 1447 = 2025-06-26.

The covered range is **1300 AH to 1600 AH inclusive — Gregorian 1882-11-12 to
2174-11-25 — and nothing outside it.** Outside that span this calendar returns
`BeforeEpoch` or `AfterSupportedRange`. It does not fall back to an arithmetic
rule, because a computed month presented as an Umm al-Qurā month would be a
fabrication. (ICU itself does fall back, so a caller comparing the two outside
the range is comparing a refusal against a guess.)

Everything else — the epochs, the intercalation schemes, the dehiyyot, the
lunisolar rules — comes from Reingold and Dershowitz, *Calendrical
Calculations*, and is cited in the module documentation where it is used.

## What this crate refuses to claim

**It does not know what anyone announced.**

The Hijri months of religious practice are proclaimed after a sighting is
reported and accepted. `islamic_observational` predicts whether the crescent
*should* have been visible from a given place in a clear sky under one
published criterion. That is a forecast of an observation, not a record of a
decision, and the crate says so in the module documentation and measures the
gap: against the Umm al-Qura table over 1400–1445 AH it starts the month one
day later for 322 of 552 months — 58% — and never earlier. That is the
expected signature of a sighting criterion against a computation criterion,
and the number is asserted in a test so it cannot drift unnoticed.

The Chinese, Korean and Vietnamese calendars were promulgated by bureaux using
their own tables and their own solar theories. A date `chinese` gives for 1700
is what the modern rules say, not what the almanac of 1700 said. The lower
bound of 1645 is the Shíxiàn calendar, which introduced the true-solar-term
rule implemented there; before that the terms were mean, the month numbering
could differ, and the crate refuses those years rather than answering wrongly.
The Korean and Vietnamese courts adopted the same rules some years later
still, so dates in that gap are what the rules give, not what was proclaimed
in Hanseong or Huế.

Japan is the exception, and deliberately so: `japanese_historical` implements
the pre-1844 Japanese systems *on their own constants* precisely in order to
say what the almanac said, and the section above gives the rate at which it
succeeds. Even there the crate does not claim to know what was announced —
the court adjusted a promulgated month by decree from time to time, and no
computation reproduces a decree.

The Tenpō calendar stops on 1872-12-31 because that is the last day it ever
named: the Dajōkan decree of 9 November 1872 made Meiji 5, twelfth month,
third day into 1 January 1873. There is no Tenpō date after that, and this
crate returns an error rather than inventing one. (It will, if asked through
an unbounded parameter set, tell you that Meiji 6 was due a leap sixth month —
the thirteen months of salary usually given as the reason for the reform's
haste.)

Finally, year numbering. The Chinese count of 4661 for the year that began in
2024 is the one *Calendrical Calculations* uses and the one for which
`hc_calendar::cycle::sexagenary_year` is directly correct; other conventions
number the same year 4721 or 4722, and none of them is official, because the
calendar has no official continuous era. Dangi 4357 is the standard Korean
number. The Vietnamese and Japanese year numbers — the Gregorian year in which
the lunisolar year begins — are this crate's own convention and are labelled
as such. Historical Japanese dates were written with a nengō, and nengō belong
to `hc-calendars-regional`, not here.

## Feature flags

`std` (default) → `alloc` → core. The crate builds with
`--no-default-features --features alloc`; nothing here needs an allocator
except through `hc-calendar`'s registry.

[`hyper-calendar`]: https://github.com/kitsuyui/hyper-calendar
