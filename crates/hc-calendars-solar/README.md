# hc-calendars-solar

Solar and purely arithmetic calendars for [`hyper-calendar`]. Every calendar
here implements `hc_calendar::Calendar`, so each one is two functions against
the Rata Die fixed day — `to_fixed` and `from_fixed` — and everything else
(the dynamic interface, the registry entry, conversion between any pair)
follows from those.

## What is in it

The calendars below, plus fourteen national and provincial variants of the Julian/Gregorian
reform; [`docs/supported.md`](../../docs/supported.md) counts them, so this
file does not:

| Module | Calendar | Identifier |
| --- | --- | --- |
| `gregorian` | Proleptic Gregorian | `gregory` |
| `julian` | Proleptic Julian, with BC/AD eras | `julian` |
| `julian_gregorian` | Julian before a country's reform, Gregorian after | `julian-gregorian-*` |
| `swedish` | Swedish, 1700–1712: the Julian calendar a day early, and 30 February 1712 | `swedish-1700` |
| `year_style` | Where the year began: Lady Day, Annunciation (Florentine, Pisan), Nativity, more veneto, Greek | *(not calendars; a year-number conversion)* |
| `adoption` | When each country took the Gregorian calendar, by ISO 3166-1 alpha-2 code, one row per step, with its instrument | *(not a calendar; a table of cut-overs)* |
| `cycles` | The computus cycles: golden number, dominical letter, epact, solar cycle, indiction, Julian Period | *(not calendars; year numbers)* |
| `cycles::runic` | The Swedish runestaff read against the Julian date: each day's letter rune and the golden-number rune of the new moons, old series ([docs/systems/runic-calendar.md](../../docs/systems/runic-calendar.md)) | *(a reading, not a calendar)* |
| `iso_week` | ISO 8601 week date | `iso8601-week` |
| `ordinal` | ISO 8601 ordinal date | `iso8601-ordinal` |
| `julian_day` | Julian Day Number, Modified Julian Date | `julian-day`, `modified-julian-day` |
| `day_counts` | Lilian, ANSI, Dublin, Reduced, Truncated, CNES and CCSDS day counts | `lilian`, `ansi-date`, `dublin-julian-day`, `reduced-julian-day`, `truncated-julian-day`, `cnes-julian-day`, `ccsds-day` |
| `coptic` | Coptic (Era of the Martyrs) | `coptic` |
| `ethiopic` | Ethiopian (Incarnation and World eras) | `ethiopic` |
| `egyptian` | Ancient Egyptian wandering year | `egyptian` |
| `armenian` | Ancient Armenian | `armenian` |
| `armenian_fixed` | Armenian (fixed, Sarkawag 1084) | `armenian-fixed` |
| `zoroastrian` | Zoroastrian: the Qadimi and Shahanshahi wandering years, and the Fasli with its leap day | `zoroastrian-qadimi`, `zoroastrian-shahanshahi`, `zoroastrian-fasli` |
| `persian` | Solar Hijri, **arithmetic** variant | `persian-arithmetic` |
| `indian` | Indian national civil (Śaka) | `indian` |
| `nanakshahi` | Nanakshahi, the Sikh solar calendar of 2003 | `nanakshahi` |
| `bangladeshi` | the Bangladeshi national calendar: the Bengali months from 14 April, the 1987 lengths to 1425 and the 2019 revision's from 1426 | `bangladeshi` |
| `discordian` | Discordian: five seasons of 73 days, the five-day week, St. Tib's Day | `discordian` |
| `buddhist` | Thai solar | `buddhist` |
| `minguo` | Minguo, with 民國前 | `roc` |
| `juche` | Juche | `juche` |
| `holocene` | Human Era | `holocene` |
| `byzantine` | Byzantine *Anno Mundi*, September new year | `byzantine` |
| `roman` | *Ab urbe condita* | `roman-auc` |
| `rumi` | Rumi, the Ottoman civil calendar of 1840–1925: Julian days to 1917, Gregorian after, the year less 584 | `rumi` |
| `french_republican` | French Republican, **arithmetic (Romme)** variant | `french-republican-arithmetic` |
| `bahai` | Badíʿ, **arithmetic Western** variant | `bahai-arithmetic` |
| `bahai_kept` | Badíʿ as kept: the arithmetic rule to 171 BE, the Bahá'í World Centre's table for 172–221 BE, nothing after | `bahai` |
| `symmetry454` | Symmetry454 | `symmetry454` |
| `symmetry010` | Symmetry010 | `symmetry010` |
| `symmetry` | The leap rule the two Symmetry calendars share | *(not a calendar)* |
| `revised_julian` | Revised Julian (Milanković) | `revised-julian` |
| `koki` | Japanese imperial year (皇紀, kōki) | `japanese-imperial` |
| `world_calendar` | The World Calendar | `world-calendar` |
| `international_fixed` | International Fixed (Cotsworth): thirteen months of 28 days with Sol, Year Day and Leap Day outside the week | `international-fixed` |
| `positivist` | Positivist (Comte, 1849): thirteen months of 28 days from Moïse to Bichat, year 1 in 1789, the complementary days outside the week | `positivist` |
| `berber` | Berber (Amazigh) agrarian: the Julian year under Latin-derived month names, Yennayer on 14 January, the Amazigh era of 950 BC that the Académie berbère constructed in the 1960s | `berber` |
| `mandaean` | Mandaean: twelve thirty-day zodiacal months with the five Parwanaia after the eighth, no leap day, years after the creation of Adam | `mandaean` |
| `assyrian` | Modern Assyrian: the Gregorian months from 1 Neesan = 1 April under Syriac names, year 1 in 4750 BC | `assyrian` |
| `yazidi` | Yazidi: the year from Serêsal, the first Wednesday of Eastern (Julian) Nisan, in the 4750 count; the day of the year over the Julian date | `yazidi` |
| `hanke_henry` | Hanke–Henry Permanent: quarters of 30, 30 and 31 days from Monday 1 January, the week *Xtr* after December in the years with 53 ISO weeks | `hanke-henry` |
| `icelandic` | Old Icelandic *misseristal*: 52 weeks in two *misseri*, summer from the first Thursday on or after 19 April Gregorian, or 9 April Julian before 1700, and the leap week *sumarauki* | `icelandic`, `icelandic-julian` |
| `qumran` | The Qumran and *Jubilees* 364-day year: quarters of 91 days from a Wednesday, no intercalation, the twenty-four priestly courses by week; the epoch a convention of this library | `qumran` |
| `soviet_week` | The Soviet revolutionary weeks of 1929–1940: Gregorian dates under the continuous five-day week and then the six-day week of the decrees | `soviet-week` |

`register_all(&mut CalendarRegistry)`, behind the `alloc` feature, inserts every
calendar in the table.

## What it deliberately does not do

The crate's boundary is arithmetic. A calendar whose rule is "every fourth
year" belongs here; one whose rule is "the day the equinox falls at Tehran"
does not, because the answer would depend on an ephemeris and would move when
the model behind it improved. Three calendars sit on that line and are
implemented here in their arithmetic form, under names that say so; the
astronomical forms are in `hc-calendars-equinox`:

* **Solar Hijri.** The official Iranian calendar begins the year at the
  observed March equinox. Implemented here is the 2 820-year cyclic rule
  associated with Birashk, which disagrees with the observation in a handful
  of years even inside the range where it is at its best. The CLDR identifier
  `persian` belongs to the astronomical calendar in `hc-calendars-equinox`.
* **French Republican.** The decree of 1793 used the true autumn equinox at
  Paris. Implemented here is Romme's proposed arithmetic rule, which puts the
  sextile day at the end of An IV where France put it at the end of An III —
  so the two disagree from 1795, inside the twelve years the calendar was
  actually in force. Anyone converting a dated document needs the equinox
  variant.
* **Badíʿ.** Since 2015 the Bahá'í calendar is unified on astronomical rules
  keyed to the Tehran equinox and sunset. Implemented here is the pre-2015
  Western form with Naw-Rúz pinned to 21 March, which is exact for what it is
  and an approximation after B.E. 171. `bahai` is the calendar as kept: this
  rule to 171 BE and the Bahá'í World Centre's published table for 172–221
  BE. It refuses later days rather than compute them.

From 1889 to 1940 Thailand's year began on 1 April, and it was counted in
the Rattanakosin era until March 1913. `buddhist` keeps the modern year, and
`buddhist::printed_year` and `buddhist::printed_to_fixed` convert between a
day and the year a Thai document of the time printed for it.

Smaller omissions, each documented in its module: the Roman republican
calendar before 45 BC and the kalends/nones/ides counting, which is in
`hc_format::roman` (`roman`); the year starts other than 1 January, which are
`year_style` (`julian_gregorian`); the regional lunisolar Hindu calendars,
which are in `hc-calendars-indic` (`indian`); the Alexandrian and Antiochene world eras
(`byzantine`); month and weekday names in each *language*, which are locale
data and live in `hc-i18n`. A calendar whose months have one orthography that
every language borrows — Coptic, Ethiopic, Egyptian, the two Armenian,
Zoroastrian, Persian, Indian, Nanakshahi, Bangladeshi, Rumi, French
Republican, Discordian — declares those names itself, with its shape, and
`hc-i18n` consults them after the locale.

## Accuracy

Every conversion in this crate is integer arithmetic. There is no floating
point in any conversion path, so "accuracy" means exactness, not error bars:
within the range each calendar advertises in its `CalendarMeta`, `to_fixed`
and `from_fixed` are exact inverses and are tested to be.

The Gregorian and Julian modules support years −9 999 999 to 9 999 999, which
comfortably contains the −9999..=9999 range the rest of the workspace assumes.
Calendars with an epoch start at it and refuse earlier days with
`CalendarError::BeforeEpoch` rather than extending proleptically into a period
where the year number would be meaningless. Minguo is the exception: it
counts the years before 1912 as 民國前, so it covers the Gregorian range.

Where a calendar's rule *approximates* an astronomical year, the mean year is
stated and tested: 365.242 198 58 days for the Persian 2 820-year cycle,
365.242 25 for the Romme rule, 365.2423 for Symmetry454's 293-year cycle,
against the Gregorian 365.2425.

## Reference data

Formulae are those of Reingold and Dershowitz, *Calendrical Calculations*
(4th ed., Cambridge, 2018), which is also the source of the epochs used by the
Coptic, Ethiopic, Egyptian, Armenian, Persian, Indian and French Republican
modules. Every one of those epochs is cross-checked in a test against an
independent statement of the same day — a Julian calendar date, a Julian Day
Number, or both — rather than trusted as a constant.

The Gregorian reform's fourteen cut-overs, and the Swedish calendar of
1700–1712 beside them, are written up in
[`docs/systems/gregorian-reform.md`](../../docs/systems/gregorian-reform.md):
the decree or act behind each date and which of them were read — the bull,
the British Act of 1750 and the Soviet decree of 1918 directly, the Serbian
law of 1919 as a newspaper quotes it, the rest through secondary sources —
the dropped days and the unbroken week, a British date of 1752 worked
across the gap, and the polities deliberately not carried.
Each table entry is checked for internal consistency: the day after the
last Julian date must be the first Gregorian one.

Anchors used in tests, each cross-checked against a second derivation (a known
weekday or a Julian Day Number): 1970-01-01 is RD 719 163, JDN 2 440 588 and a
Thursday; 2000-01-01 is RD 730 120; 1582-10-04 Julian is followed by
1582-10-15 Gregorian; 2021-01-01 is ISO 2020-W53-5; 22 March 1957 is 1 Chaitra
1879 Śaka; 11 February 1979 is 22 Bahman 1357; Ethiopian new year dates for
2022 to 2025; Symmetry454's year 2005 opening on Monday 3 January 2005.

## Features

`default = ["std"]`; `std` implies `alloc`. Without `std`, `libm` passes
through to `hc-core`, which refuses to compile with neither. The crate builds
with `--no-default-features --features libm` and with
`--no-default-features --features alloc,libm`; only `register_all` and the
registry need an allocator.

[`hyper-calendar`]: https://github.com/kitsuyui/hyper-calendar
