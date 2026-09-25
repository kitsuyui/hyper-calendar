# `hc-calendars-lunar`

Lunar and lunisolar calendars for [`hyper-calendar`]: the Hijri family, the
Hebrew calendar, the Babylonian calendar of the Seleucid era, the Tibetan
Phugpa calendar, the East Asian lunisolar calendars of China, Korea and
Vietnam, and the five successive lunisolar calendars Japan used between 862
and 1872.

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
| `babylonian` | `babylonian` | astronomical | SE −71 to 386 (383 BCE to 76 CE) |
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

## The Hijri family

The five Hijri identifiers are written up in
[`docs/systems/hijri.md`](../../docs/systems/hijri.md): the calendar as
kept by sighting and the schemes beside it, the thirty-year cycle with its
four leap-year patterns and two epochs, the Umm al-Qura rules by period and
the table's provenance, the visibility criterion, a month worked by hand
and a named evening on which the three disagree, and what each measurement
below means. This section keeps the summary and the figures.

The tabular calendars are counting rules and are exact as such: `tabular`
is checked against the closed form of Reingold and Dershowitz for every
month of 1–3000 AH, and `islamic-fatimid` against the Bohra community's own
published Mawlid of 1439. The Umm al-Qura table is exact where it reaches,
1300–1600 AH, and refuses everywhere else. The observational prediction is
a forecast under one criterion at one place: against the Umm al-Qura table
over 1400–1445 AH it starts the month a day later for 322 of 552 months —
58% — and never earlier, which is the signature of a sighting criterion
against a computation criterion and is asserted in a test.

## Japan's historical calendars

The five Japanese systems are written up in
[`docs/systems/japanese-lunisolar.md`](../../docs/systems/japanese-lunisolar.md):
what each was and who computed it, how 恒気, 定朔, 進朔 and the 里差 work,
with two worked examples that can be followed by hand, every constant with
its source, and how the measurement below was made. This section keeps the
summary and the figures.

A calendar that ran for eight centuries on ninth-century constants is not the
sky, and must not be computed as if it were. Senmyō-reki's tropical year is
3.4 minutes too long; over the 823 years Japan used it, its solar terms slid
about two days away from the Sun, and *that drift is why the Jōkyō reform
happened*. So each of the four pre-Tenpō systems carries its own 歳実, 朔実
and 近点月, its own 恒気 major solar terms and, for Senmyō-reki, its own 進朔;
Tenpō-reki, which defined its terms as the true Sun, is computed from
`hc-astro`. Every constant in the code carries its source in a comment.

### Measured agreement

`tests/data/japanese_month_lengths.txt` holds the first day and the length of
every month of every year from 862 to 1843 — **982 years, 12 146 months,
300 592 days** — from the 西暦との対照表 of the Japanese Wikipedia era
articles, which transcribe 内田正男『日本暦日原典』 (雄山閣, 1975). It is one
source family, cross-checked as the document describes. Against it, with
`cargo test -p hc-calendars-lunar -- --nocapture`:

| System | New years | Intercalary months | Month starts | Individual days |
|---|---|---|---|---|
| 宣明暦 Senmyō | 94.65% | 93.68% | 96.43% | **96.39%** |
| 貞享暦 Jōkyō | 97.14% | 100.00% | 98.85% | **98.83%** |
| 宝暦暦 Hōryaku | 97.67% | 90.70% | 97.74% | **97.73%** |
| 寛政暦 Kansei | 97.83% | 97.83% | 99.12% | **99.11%** |

The four dates usually asked for all come out right: 本能寺の変 天正10年6月2日 =
1582-06-21 Julian, 関ヶ原 慶長5年9月15日 = 1600-10-21, 赤穂事件討ち入り
元禄15年12月14日 = 1703-01-30, and 貞享2年1月1日 = 1685-02-04.

Almost every disagreement is a month boundary one day off; the document gives
the three causes and the two things this crate measured rather than assumed —
that a system's own tropical year is worth twenty-five points of
intercalary-month agreement over modern solar theory, and that the true
conjunction reproduces the published month starts better than the system's
own tables do, which is why each module also exports a `PARAMETERS_TABULATED`
and both are measured. Two scalars per calendar — the 暦元 solstice phase,
and for Senmyō-reki the 進朔 limit — are fitted, and the code labels them as
such wherever they appear.

**元嘉暦, 儀鳳暦, 大衍暦 and 五紀暦 (604–862) are not implemented**; the
document says why.

## Accuracy

**Arithmetic calendars — exact.** The tabular Hijri and Hebrew calendars are
counting rules, and this is those rules. The tabular Hijri checks are in
the section above. The Hebrew implementation reproduces 1 Tishrei 5784 =
2023-09-16 and 15 Nisan 5784 = 2024-04-23, keeps Rosh Hashanah off Sunday,
Wednesday and Friday for all 9 999 years, and gives every year one of the
six permitted lengths.

Exact is not the same as astronomically right. The Hebrew molad is 0.4 seconds
longer than the true mean synodic month, so it drifts about a day later every
216 years; the tabular Hijri month is 2.9 seconds short, so it drifts a day in
about 2 400 years. Both figures are in the module documentation.

**The Umm al-Qura table — exact where it reaches, and nowhere else.** See
the section above and the system document.

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

**The Babylonian calendar — measured against the standard table.** The
month begins on the evening that passes the moonlag criterion at Babylon,
and the thirteenth month falls where the nineteen-year rule puts it, both as
*Calendrical Calculations* states them. Against Parker and Dubberstein's
*Babylonian Chronology* (1971 edition, in R. H. van Gent's transcription)
over the 5 664 months from SE −71, where their table follows the rule
without exception, every intercalary month is in its place and the first
day of the month is theirs for 82.9%, a day later for 16.6% and a day
earlier for 0.5%, never further off; the rate is much the same in every
fifty-year stretch, so it is the two visibility criteria that differ. The
criterion is applied to each evening on its own, so 38 of those months run
31 days here where the table, which does the same, has 29 or 30; the module
documentation says why the thirty-day rule is not imposed. The range stops
where the table stops following the rule at one end and where
the table ends at the other. The figure is asserted in an ignored test that
runs against a copy of the table, and the module documentation names the
rows it cites.

**Historical calendars — measured, and the measurement is above.** The four
Japanese systems in `japanese_historical` do not use modern solar theory at
all, and the whole of what they claim is the agreement rate in that section:
96.4% to 99.1% of individual days against 982 years of published table. They
are the only calendars in this crate that aim at what a bureau published
rather than at what a rule gives.

## Meridian conventions

The three modern calendars are written up in
[`docs/systems/east-asian-lunisolar.md`](../../docs/systems/east-asian-lunisolar.md):
who promulgates each, the 1645 reform that bounds them, the rules with the
suì and the zhōngqì, Seollal 1988 and Tết 1985 worked by hand, and what each
published new year in the Accuracy section above was checked against. This
section keeps the tables and the cases.

The day boundary is read in local time, which is why these calendars disagree
with each other at all. The tables implemented are those of *Calendrical
Calculations*, keyed by year here; the document gives the days within the
years for Korea and measures that the difference moves nothing:

| Calendar | Offsets |
|---|---|
| Chinese | Beijing local mean time (116°25′E) before 1929; UT+8 from 1929 |
| Dangi | Seoul local mean time (126°58′E) before 1908; UT+8:30 1908–1911; UT+9 1912–1953; UT+8:30 1954–1960; UT+9 from 1961 |
| Vietnamese | UT+8 before 1968; UT+7 from 1968 |
| Japanese (all five) | Kyoto local mean time (135°46′E) before 1888; UT+9 from 1888 |

One caveat on that last row. Kyoto is right from Jōkyō-reki onward, whose
里差 was the reform's headline change; Senmyō-reki was applied with no
correction at all, and the system document says how that offset is absorbed
and what the other arrangement measures.

These are not decoration. Over 1900–2049 the Korean and Chinese new years fall
on different days nine times, 1988 among them (Seollal 18 February, Chinese
New Year 17 February); the Vietnamese calendar kept Tết 1968 on 29 January in
the North and 30 January in the South; and in 1985 the same hour moved the
winter solstice, the leap month and Tết itself a whole lunation before Chinese
New Year. All three are tested, and the document works the last two by hand.

## Reference data

**The Umm al-Qura table** is 301 `u16` values, one per Hijri year from 1300 to
1600, each carrying twelve bits for which months have 30 days: ICU's
`UMALQURA` table, bit for bit, and the system document says how it was
read, what rules it embodies and which Saudi announcements it was checked
against. The covered range is **1300 AH to 1600 AH inclusive — Gregorian
1882-11-12 to 2174-11-25 — and nothing outside it.** Outside that span this
calendar returns `BeforeEpoch` or `AfterSupportedRange`. It does not fall
back to an arithmetic rule, because a computed month presented as an Umm
al-Qurā month would be a fabrication. (ICU itself does fall back, so a
caller comparing the two outside the range is comparing a refusal against a
guess.)

Everything else — the epochs, the intercalation schemes, the dehiyyot, the
lunisolar rules — comes from Reingold and Dershowitz, *Calendrical
Calculations*, and is cited in the module documentation where it is used;
the Hijri sources are keyed in `docs/references.bib`.

## What this crate refuses to claim

**It does not know what anyone announced.**

The Hijri months of religious practice are proclaimed after a sighting is
reported and accepted. `islamic_observational` predicts whether the crescent
*should* have been visible from a given place in a clear sky under one
published criterion. That is a forecast of an observation, not a record of a
decision; the module says so, and the measured gap is in the Hijri section
above and explained in the system document.

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
named; the decree that ended it is in the system document. There is no Tenpō
date after that, and this crate returns an error rather than inventing one.
(It will, if asked through an unbounded parameter set, tell you that Meiji 6
was due a leap sixth month.)

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

`std` (default) → `alloc` → core, and `libm`, which passes through to
`hc-core` for floating-point math where there is no `std`. The crate builds
with `--no-default-features --features alloc,libm`; nothing here needs an
allocator except through `hc-calendar`'s registry.

[`hyper-calendar`]: https://github.com/kitsuyui/hyper-calendar
