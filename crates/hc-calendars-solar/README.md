# hc-calendars-solar

Solar and purely arithmetic calendars for [`hyper-calendar`]. Every calendar
here implements `hc_calendar::Calendar`, so each one is two functions against
the Rata Die fixed day — `to_fixed` and `from_fixed` — and everything else
(the dynamic interface, the registry entry, conversion between any pair)
follows from those.

## What is in it

The calendars below, plus twelve national variants of the Julian/Gregorian
reform; [`docs/supported.md`](../../docs/supported.md) counts them, so this
file does not:

| Module | Calendar | Identifier |
| --- | --- | --- |
| `gregorian` | Proleptic Gregorian | `gregory` |
| `julian` | Proleptic Julian, with BC/AD eras | `julian` |
| `julian_gregorian` | Julian before a country's reform, Gregorian after | `julian-gregorian-*` |
| `year_style` | Where the year began: Lady Day, Annunciation (Florentine, Pisan), Nativity, more veneto, Greek | *(not calendars; a year-number conversion)* |
| `iso_week` | ISO 8601 week date | `iso8601-week` |
| `ordinal` | ISO 8601 ordinal date | `iso8601-ordinal` |
| `julian_day` | Julian Day Number, Modified Julian Date | `julian-day`, `modified-julian-day` |
| `day_counts` | Lilian, ANSI, Dublin, Reduced, Truncated, CNES and CCSDS day counts | `lilian`, `ansi-date`, `dublin-julian-day`, `reduced-julian-day`, `truncated-julian-day`, `cnes-julian-day`, `ccsds-day` |
| `coptic` | Coptic (Era of the Martyrs) | `coptic` |
| `ethiopic` | Ethiopian (Incarnation and World eras) | `ethiopic` |
| `egyptian` | Ancient Egyptian wandering year | `egyptian` |
| `armenian` | Ancient Armenian | `armenian` |
| `armenian_fixed` | Armenian (fixed, Sarkawag 1084) | `armenian-fixed` |
| `persian` | Solar Hijri, **arithmetic** variant | `persian-arithmetic` |
| `indian` | Indian national civil (Śaka) | `indian` |
| `buddhist` | Thai solar | `buddhist` |
| `minguo` | Minguo, with 民國前 | `roc` |
| `juche` | Juche | `juche` |
| `holocene` | Human Era | `holocene` |
| `byzantine` | Byzantine *Anno Mundi*, September new year | `byzantine` |
| `roman` | *Ab urbe condita* | `roman-auc` |
| `french_republican` | French Republican, **arithmetic (Romme)** variant | `french-republican-arithmetic` |
| `bahai` | Badíʿ, **arithmetic Western** variant | `bahai-arithmetic` |
| `symmetry454` | Symmetry454 | `symmetry454` |
| `symmetry010` | Symmetry010 | `symmetry010` |
| `revised_julian` | Revised Julian (Milanković) | `revised-julian` |
| `koki` | Japanese imperial year (皇紀, kōki) | `japanese-imperial` |
| `world_calendar` | The World Calendar | `world-calendar` |

`register_all(&mut CalendarRegistry)`, behind the `alloc` feature, inserts every
calendar in the table.

## What it deliberately does not do

The crate's boundary is arithmetic. A calendar whose rule is "every fourth
year" belongs here; one whose rule is "the day the equinox falls at Tehran"
does not, because the answer would depend on an ephemeris and would move when
the model behind it improved. Three calendars sit on that line and are
implemented in their arithmetic form only, under names that say so:

* **Solar Hijri.** The official Iranian calendar begins the year at the
  observed March equinox. Implemented here is the 2 820-year cyclic rule
  associated with Birashk, which disagrees with the observation in a handful
  of years even inside the range where it is at its best. The CLDR identifier
  `persian` is left free for `hc-astro`.
* **French Republican.** The decree of 1793 used the true autumn equinox at
  Paris. Implemented here is Romme's proposed arithmetic rule, which puts the
  sextile day at the end of An IV where France put it at the end of An III —
  so the two disagree from 1795, inside the twelve years the calendar was
  actually in force. Anyone converting a dated document needs the equinox
  variant.
* **Badíʿ.** Since 2015 the Bahá'í calendar is unified on astronomical rules
  keyed to the Tehran equinox and sunset. Implemented here is the pre-2015
  Western form with Naw-Rúz pinned to 21 March, which is exact for what it is
  and an approximation after B.E. 171.

Smaller omissions, each documented in its module: the Roman republican
calendar before 45 BC and the kalends/nones/ides counting (`roman`); the
1 April year start Thailand used before 1941 (`buddhist`); the 25 March year
start England used before 1752 (`julian_gregorian`); the regional lunisolar
Hindu calendars (`indian`); the Alexandrian and Antiochene world eras
(`byzantine`); month and weekday names in each *language*, which are locale
data and live in `hc-i18n`. A calendar whose months have one orthography that
every language borrows — Coptic, Ethiopic, Egyptian, the two Armenian,
Persian, Indian, French Republican — declares those names itself, with its
shape, and `hc-i18n` consults them after the locale.

## Accuracy

Every conversion in this crate is integer arithmetic. There is no floating
point in any conversion path, so "accuracy" means exactness, not error bars:
within the range each calendar advertises in its `CalendarMeta`, `to_fixed`
and `from_fixed` are exact inverses and are tested to be.

The Gregorian and Julian modules support years −9 999 999 to 9 999 999, which
comfortably contains the −9999..=9999 range the rest of the workspace assumes.
Calendars with an epoch start at it and refuse earlier days with
`CalendarError::BeforeEpoch` rather than extending proleptically into a period
where the year number would be meaningless.

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

The Gregorian reform adoption dates come from the national legislation and
gazettes as summarised in the same work and in the *Explanatory Supplement to
the Astronomical Almanac* (3rd ed., 2013), §15.3. Each table entry is checked
for internal consistency: the day after the last Julian date must be the first
Gregorian one.

Anchors used in tests, each cross-checked against a second derivation (a known
weekday or a Julian Day Number): 1970-01-01 is RD 719 163, JDN 2 440 588 and a
Thursday; 2000-01-01 is RD 730 120; 1582-10-04 Julian is followed by
1582-10-15 Gregorian; 2021-01-01 is ISO 2020-W53-5; 22 March 1957 is 1 Chaitra
1879 Śaka; 11 February 1979 is 22 Bahman 1357; Ethiopian new year dates for
2022 to 2025; Symmetry454's year 2005 opening on Monday 3 January 2005.

## Features

`default = ["std"]`; `std` implies `alloc`. The crate builds with
`--no-default-features` and with `--no-default-features --features alloc`;
only `register_all` and the registry need an allocator.

[`hyper-calendar`]: https://github.com/kitsuyui/hyper-calendar
