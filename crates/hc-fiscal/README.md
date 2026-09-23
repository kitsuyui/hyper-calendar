# hc-fiscal

Years that do not begin on 1 January: fiscal years, tax years, academic years,
and the 52/53-week reporting calendars that retailers and broadcasters run on.

This crate adds no calendar. It observes that a great many institutions agree
about what *day* it is and disagree about what *year* it is, and that the
disagreement is regular enough to be data.

## The two things it exists to get right

**The label.** Japan's 2024年度 runs 1 April 2024 to 31 March 2025. The United
States' FY 2024 ran 1 October **2023** to 30 September 2024, because the United
States names a fiscal year after the calendar year it *ends* in. Both are
written "FY2024"; on 1 November 2023 Tokyo and Washington disagree by a whole
year about which one it is.

So `LabelConvention` is an explicit enum — `LabelledByStartYear` and
`LabelledByEndYear` — and it deliberately has **no `Default`**. A default here
would take a position on the caller's behalf silently, which is the failure
mode `docs/policy.md` §5 exists to prevent. New Zealand makes the point on its
own: its Crown financial year is labelled by its start and its tax year by its
end, in the same country.

**The calendar.** "The year starts on 1 April" is not a complete rule until the
calendar is named. So a `YearStart` carries a `StartCalendar`, and the
non-Gregorian entries go through the real calendars in `hc-calendars-solar` and
`hc-calendars-indic` rather than through a hard-coded approximate Gregorian
date:

| Country | Start | Calendar | Labelled by |
| --- | --- | --- | --- |
| Iran | 1 Farvardin | `persian-arithmetic` | start year (Solar Hijri) |
| Ethiopia | Hamle 1 | `ethiopic` | **end** year (Ethiopic) |
| Thailand | 1 October | `buddhist` | **end** year (Buddhist Era) |
| Nepal | 1 Shrawan | `bikram-sambat` | start year (Bikram Sambat) |

Thailand is there because the label 2568 is a Buddhist Era year. Expressing
that as a start *in the Thai calendar* rather than as a Gregorian date with 543
added afterwards is exactly the data/algorithm split of `docs/policy.md` §2:
one piece of arithmetic then serves Bangkok, Tokyo and Tehran.

## What is in it

| Module | Subject |
| --- | --- |
| `year_system` | the core type — a start in a named calendar, a labelling convention, a validity range. Given an `Rd` it yields the label and the position in the year; given a label it yields the span. |
| `quarters` | quarters, halves and months *of the fiscal year*. Japan's Q1 is April–June; the United States federal Q1 is October–December. |
| `countries` | 22 national tables, each with a source and a `sources_checked` date. |
| `academic` | 8 school and university years, each carrying how firmly it is fixed. |
| `retail` | 4-4-5, 4-5-4, 5-4-4 and the 52/53-week year, as named conventions. |

Fiscal months generalise rather than special-case: fiscal month *n* runs from
the start day-of-month to the day before the same day-of-month a month later.
Where the start is the first of a month that reduces to calendar months; where
it is not — the United Kingdom's 6 April — it produces HMRC's tax months, 6th
to 5th, without a branch.

## Why the retail calendars live here

`docs/observances.md` lists "4-4-5, 13-period retail" as out of scope for the
holiday engine. That judgement is right and it is about holidays: a retail
period is not an observance and names no day. But a 52/53-week year *is* a year
that does not begin on 1 January, and it wants the same four things this crate
already has — a start rule, a labelling convention, a validity range, and
sub-periods counted from the year's own start. So it is here, and the
`retail::ISO_8601_WEEK_YEAR` value is checked day for day against
`hc_calendars_solar::iso_week`, which implements ISO 8601's own definition from
the other side.

The two anchor rules are named, not parameterised, because they are the two
alternatives of Treasury Regulation § 1.441-2(a)(1)(iii): "the last Saturday in
January" and "the Saturday nearest 31 January". The regulation's own worked
example shows why the distinction matters — a Saturday year end in November
2001 is 24 November under one rule and 1 December under the other.

## What it claims, and how precisely

* **Gregorian-anchored years are exact.** Integer arithmetic throughout; no
  floating point except `FiscalPosition::fraction_elapsed`, which is a
  convenience and claims nothing finer than a day.
* **Ethiopia is exact** to the Ethiopic calendar. Its Gregorian start happens
  to be 8 July every year across the modern era, because Pagumen falls in
  September and never lands inside the Sene-to-Hamle boundary — but that is a
  *consequence*, so the crate computes it through the calendar rather than
  hard-coding it.
* **Iran is approximate, by a stated amount.** The official rule is
  astronomical: 1 Farvardin is the day the March equinox falls before noon at
  52.5°E. `hc-calendars-solar` has no ephemeris, so this crate uses the 2 820-
  year Birashk cycle under the identifier `persian-arithmetic`. Measured
  against the published Solar Hijri years it agrees for 1400, 1401, 1402, 1403
  and 1405, and is **one day early for 1404**. Tøndering names AP 1404 and
  AP 1437 as the only two disagreements between AP 1244 and AP 1531
  (AD 1865–2152). There is a test that asserts the wrong answer on purpose, so
  that the day an astronomical Solar Hijri calendar lands, it fails and says so.
* **Nepal is exact where the months are published, and approximate elsewhere,
  by a stated amount.** The year begins on 1 Shrawan in the Bikram Sambat, and
  a Bikram Sambat month is as long as the Government of Nepal says. The months
  of 2080–2083 BS are the gazette's, which puts 1 Shrawan on 16 July 2024 and
  17 July 2025 and 2026. Other years go through `hc-calendars-indic`'s
  reckoning, which missed one of those 48 months by a day.

## What it deliberately does not do

* **It leaves holes where history left them.** The United States' transition
  quarter — 1 July to 30 September 1976, after FY1976 ended and before FY1977
  began — belongs to no fiscal year, and `FiscalProfile::at` returns `None` for
  every day in it. Sweden's eighteen-month 1995/96 budget year is likewise
  outside both Swedish budget-year systems, so 1996 has no answer. England's 1751 and 1752
  are outside both English ones.
* **It refuses quarters it cannot define.** The Ethiopic year is twelve
  thirty-day months plus Pagumen. No source says which quarter Pagumen falls
  in, so every period method returns `PeriodsNotDefined` for an
  Ethiopic-anchored system. The span and the day numbering still work.
* **It does not assert a national academic year where there is none.** Every
  academic entry carries an `Authority`; only Japan, France and New Zealand
  claim `is_national_rule()`. The United States, Germany, Australia, India and
  the United Kingdom record the modal choice with a note saying so.
* **It knows no company's fiscal year.** Walmart reports to a fixed 31 January
  and Apple to the last Saturday of September; a filer's own year is a
  `WeekYearSystem` the caller writes out.
* **It computes no business days or holidays.** That is `hc-holiday`. 31 March
  ends Japan's 年度 whether or not it is a Sunday.

## The United Kingdom, and what this crate will not repeat

The UK tax year starts on 6 April because of the calendar reform
`hc_calendars_solar::julian_gregorian` already implements as the `gb` adoption.
The English legal and fiscal year began on 25 March, Lady Day. The Calendar
(New Style) Act 1750 made 2 September 1752 be followed by 14 September 1752,
and its **section 6** provided that the times of payment of rents and
annuities, the running of leases and the attaining of full age were *not*
altered by the renaming — so obligations moved eleven natural days later and
the Lady Day boundary landed on 5 April. That is the statute, and it is a
better account than the usual "the Treasury did not want to lose eleven days of
revenue", which is a fair gloss on the motive but not the mechanism.

The usual telling then adds a twelfth day in 1800, on the grounds that the
Julian calendar would have had a leap day that year and the Gregorian did not.
**This crate does not assert that.** No statute or contemporary record for such
an adjustment turned up, and Pitt's first income tax under the Income Tax Act
1799 already ran to 5 April **1800** — so the boundary existed before the
adjustment is supposed to have happened. A better-evidenced explanation, argued
by Alan O'Brien and summarised by Paul Lewis, is that a year running "from
25 March" legally began on 26 March, and 26 March plus eleven days is 6 April
with no 1800 step required. That is not yet established scholarship, so the
crate states the 1752 chain as fact, records the 1800 story as unsupported, and
names the alternative without endorsing it.

## Where the data came from

Every `FiscalProfile` and `AcademicProfile` carries a `sources` string and a
`sources_checked` date, as the holiday tables do. The primary sources are
statutes and ministries: 財政法第11条 and 学校教育法施行規則第59条 for Japan;
31 U.S.C. § 1102 and Pub. L. 93-344 for the United States; the Interpretation
Act 1978, the Income Tax Act 2007 and the Calendar (New Style) Act 1750 for the
United Kingdom; the Financial Administration Act for Canada; the General
Clauses Act 1897 for India; พระราชบัญญัติวิธีการงบประมาณ พ.ศ. 2561 for
Thailand; قانون محاسبات عمومی کشور art. 6 for Iran; Financial Administration
Proclamation No. 648/2009 for Ethiopia; the Financial Procedures and Fiscal
Responsibility Act, 2076, section 2(e), for Nepal; the Budget Law art. 18 for
China;
Bundeshaushaltsordnung § 4, the LOLF art. 1, the Russian Budget Code art. 12
and Lei nº 4.320 art. 34 for the calendar-year countries; Treasury Regulation
§ 1.441-2 and the National Retail Federation for the retail calendars.

Where a primary source could not be read first-hand — Hong Kong's Public
Finance Ordinance, Singapore's Financial Procedure Act, South Africa's PFMA,
Ethiopia's proclamation text — the entry's `note` says so rather than implying
a citation it does not have. Where a commonly repeated claim could not be
sourced at all — Pakistan's supposed 1959 change, the UK's 1800 leap day — the
crate declines to encode it and says why.

## Testing

134 unit tests and 1 doc test. Among them: Japan's 年度 boundary from both sides; the United States
and Japan labelling the same day a year apart; the 92 days of the 1976
transition quarter belonging to no year; the UK tax year's 6 April start and
its five-day offset from the government year; the published Solar Hijri years
against Iran's entry, including the one the approximation gets wrong; the
Ethiopian fiscal year against three published EFY spans and its stability
across 85 years; a 53-week NRF retail year against the federation's own dates;
and the ISO 8601 week year cross-checked day for day against
`hc_calendars_solar::iso_week` over 14 000 days.

```sh
cargo test -p hc-fiscal
cargo clippy -p hc-fiscal --all-targets --all-features -- -D warnings
cargo build -p hc-fiscal --no-default-features --features alloc,libm
```
