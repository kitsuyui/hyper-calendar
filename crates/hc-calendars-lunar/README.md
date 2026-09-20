# `hc-calendars-lunar`

Lunar and lunisolar calendars for [`hyper-calendar`]: the Hijri family, the
Hebrew calendar, and the East Asian lunisolar calendars of China, Korea,
Vietnam and pre-Meiji Japan.

Every calendar implements `hc_calendar::Calendar`, so every one of them
converts through `Rd`, the Rata Die fixed day, and none of them knows the
others exist.

## What is here

| Module | Identifier | Kind | Range |
|---|---|---|---|
| `islamic_civil` | `islamic-civil` | arithmetic | 1–9999 AH |
| `islamic_astronomical` | `islamic-tbla` | arithmetic | 1–9999 AH |
| `islamic_umalqura` | `islamic-umalqura` | published table | **1300–1600 AH only** |
| `islamic_observational` | `islamic-rgsa` | prediction | 1900–2100 CE |
| `hebrew` | `hebrew` | arithmetic | AM 1–9999 |
| `chinese` | `chinese` | astronomical | 1645–2150 CE |
| `dangi` | `dangi` | astronomical | 1645–2150 CE |
| `vietnamese` | `vietnamese` | astronomical | 1645–2150 CE |
| `japanese_tenpo` | `japanese-tenpo` | astronomical | 1844-02-18 to **1872-12-31** |

`tabular` and `lunisolar` are the two engines those modules configure.
`tabular` takes an epoch and one of four intercalation schemes, so eight
tabular Hijri calendars are reachable, not two. `lunisolar` takes a meridian
history, an epoch, a year numbering and a choice of true or mean solar terms,
so the four East Asian calendars are parameter sets containing no algorithm.

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
Meeus low-precision series, good to about 0.01°, whose solstice instants run
systematically some four and a half minutes early. When a conjunction or a
solstice falls within roughly ten minutes of local midnight, the day assigned
can be wrong by one — and a wrong day for a zhōngqì can move a leap month by a
whole month. The crate tests the published new years it can check (Chinese New
Year 1900, 2000, 2020–2026; Seollal 1988 and 2024; Tết 1968, 1985 and 2024;
the Tenpō dates of 1844 and 1872) and they all come out right, but that is
evidence, not a guarantee.

## Meridian conventions

The day boundary is read in local time, which is why these calendars disagree
with each other at all. The tables implemented are those of *Calendrical
Calculations*:

| Calendar | Offsets |
|---|---|
| Chinese | Beijing local mean time (116°25′E) before 1929; UT+8 from 1929 |
| Dangi | Seoul local mean time (126°58′E) before 1908; UT+8:30 1908–1911; UT+9 1912–1953; UT+8:30 1954–1960; UT+9 from 1961 |
| Vietnamese | UT+8 before 1968; UT+7 from 1968 |
| Japanese Tenpō | Kyoto local mean time (135°46′E) before 1888; UT+9 from 1888 |

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

The Chinese, Korean, Vietnamese and Japanese calendars were promulgated by
bureaux using their own tables and their own solar theories. A date this crate
gives for 1700 is what the modern rules say, not what the almanac of 1700 said.
The lower bound of 1645 is the Shíxiàn calendar, which introduced the
true-solar-term rule implemented here; before that the terms were mean, and the
crate refuses those years rather than answering wrongly. The Korean and
Vietnamese courts adopted the same rules some years later still, so dates in
that gap are what the rules give, not what was proclaimed in Hanseong or Huế.

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
number. The Vietnamese and Tenpō year numbers — the Gregorian year in which
the lunisolar year begins — are this crate's own convention and are labelled
as such.

## Feature flags

`std` (default) → `alloc` → core. The crate builds with
`--no-default-features --features alloc`; nothing here needs an allocator
except through `hc-calendar`'s registry.

[`hyper-calendar`]: https://github.com/kitsuyui/hyper-calendar
