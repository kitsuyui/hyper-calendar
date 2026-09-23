# hc-holiday

Holidays and observances, as data.

**A holiday is data, never code.** This crate is one evaluator and one rule
vocabulary; every country and every religious tradition in it is a table of
rule values. There is no function named after a country anywhere in the
source, and adding a country adds no branch to the engine. A caller who wants
a company calendar, a school year or a fictional setting supplies their own
`RuleSet` and gets the same machinery.

| Module | What it holds |
| --- | --- |
| `rule` | the rule vocabulary and the observance modifiers |
| `computus` | Easter, Gregorian and Julian, and the offsets keyed to it |
| `engine` | evaluation, and business-day arithmetic |
| `traditions` | the cross-cutting religious cycles |
| `roman_calendar` | the General Roman Calendar: every celebration with its rank, and the decrees since 2002 |
| `international` | the United Nations international days, each citing its resolution |
| `exchanges` | 21 exchange calendars: New York, Nasdaq, Toronto, São Paulo, London, Frankfurt, Zurich, Euronext's seven markets, Nasdaq's four Nordic markets, Tokyo, Hong Kong, Sydney |
| `countries` | 124 national tables |

## The vocabulary

```text
FixedGregorian { month, day }         New Year's Day, 1 January
NthWeekday { month, n, weekday }      US Thanksgiving, 4th Thursday of November
LastWeekday { month, weekday }        UK Spring Bank Holiday, last Monday of May
WeekdayOnOrAfter { month, day, wd }   Midsommardagen, the Saturday on or after 20 June
WeekdayOnOrBefore { month, day, wd }  Buß- und Bettag, the Wednesday before 23 November
FixedInCalendar { system, month, day} Eid al-Fiṭr, 1 Shawwāl; Rosh Hashanah, 1 Tishrei
SolarTerm { term, meridian }          春分の日, the equinox at UTC+9
EasterRelative { computus, offset }   Good Friday (−2), Corpus Christi (+60)
LunarPhase { phase, month, day, mer } the first full moon on or after a date
Tithi { month, tithi, prevails, wt }  Rāma Navamī, Chaitra śukla 9 at midday
Sankranti { sign, ayanamsa, mer }     Makar Sankranti, the Sun's entry into Makara
Nakshatra { n, sign, tithi, ay, mer } Thaipusam, Puṣya in Thai
Offset { base, days }                 Seollal's eve, 除夕, Tết's first days
Span { from, to }                     Dashain, Phūlpātī to Āśvina śukla 12, six or seven days
MovedByWeekday { base, moves }        Argentina's feriados trasladables, Colombia's Ley Emiliani
Tabulated { function, first, last }   Matariki, gazetted through a stated last year
Computed(fn)                          the handful that really are bespoke
```

Modifiers are data too: `SubstitutionPolicy` (which weekdays move a holiday,
which way, whether the search steps past a day already taken, and whether two
holidays colliding count), `BridgePolicy` (Japan's 国民の休日),
`WeekendPolicy` (which days are the weekend, over stated years),
`valid_from` / `valid_until` on every rule, `regions` for subdivision scoping,
and `Kind` for public / bank / religious / observance. A set can also
`include` other sets — an exchange on its country's calendar — each evaluated
under its own policies, with its days off merged in and its observances left
behind.

Only **five** statutes in the whole crate are `Computed`, each written as a
sentence rather than a pattern: Ireland's St Brigid's Day, the Dutch royal
day (under two monarchs), US Inauguration Day, Mexico's presidential handover
and Israel's Yom HaAtzmaut. New Zealand's Matariki is `Tabulated`, a
published schedule with its last year stated, so that running past it is a
reported gap. Japan needs none.

## Japan is complete and exact

Every Japanese public holiday from the 祝日法 (昭和23年法律第178号, in force
20 July 1948) to today, with every amendment:

| In force | Change |
| --- | --- |
| 1948 | the nine original holidays — and only the three that follow 20 July are holidays in 1948 |
| 1966 / 1967 | 敬老の日, 体育の日, and 建国記念の日 once its 政令 fixed the date |
| 1973 | 振替休日, from 12 April — so 1973-02-12 is *not* a holiday and 1973-04-30 is the first one ever |
| 1986 | 国民の休日; the first actual occurrence is 1988-05-04 |
| 1989 | 天皇誕生日 4/29 → 12/23, 4/29 becomes みどりの日 |
| 1996 | 海の日 |
| 2000, 2003 | ハッピーマンデー, in two waves |
| 2007 | 昭和の日, みどりの日 → 5/4, and 振替休日 becomes "the nearest following non-holiday" |
| 2016 | 山の日 |
| 2019 | 天皇の即位の日 and 即位礼正殿の儀, which bridge 4/30 and 5/2 into a ten-day Golden Week; no 天皇誕生日 at all that year |
| 2020, 2021 | the Tokyo Olympics moved 海の日, スポーツの日 and 山の日, twice |

The 1959, 1989, 1990 and 1993 imperial one-offs are there too.

**春分の日 and 秋分の日 are computed, not tabulated.** The statute defines them
as the day of the equinox; the National Astronomical Observatory of Japan
computes the instant in JST and the Cabinet Office prints the resulting date
in the 官報 a year ahead. So this crate carries `Rule::SolarTerm` at
`Meridian::JAPAN` and lets `hc-seasons` answer. Over 1980–2099 that agrees
with all 240 dates the Observatory has published. The underlying solar
longitude is VSOP87, good to about 1″, so an equinox lands within the minute
the almanacs round to; only an equinox within about a minute of JST midnight
could still be given the wrong *day*, and the tightest case in the modern
record, the autumn equinox of 2012 at 23:49 JST, is eleven minutes clear.

## Coverage

**124 countries.** Albania, Algeria, Andorra, Argentina, Armenia, Australia
(eight states and territories), Austria, Azerbaijan, the Bahamas, Bahrain,
Barbados, Belarus, Belgium, Belize, Bolivia, Botswana, Brazil, Bulgaria, Canada
(federal plus the provincial days fixed by statute), Chile, China, Colombia,
Costa Rica, Côte d'Ivoire, Croatia, Cuba, Cyprus, Czechia, Denmark, the
Dominican Republic, Ecuador, Egypt, Estonia, Ethiopia, Finland, France
(métropole plus Alsace-Moselle), Georgia, Germany (all 16 *Länder*), Ghana,
Greece, Guatemala, Guyana, Haiti, Hong Kong, Hungary, Iceland, India, Indonesia,
Iran, Iraq, Ireland, Israel, Italy, Jamaica, Japan, Jordan, Kazakhstan, Kenya,
Kuwait, Kyrgyzstan, Latvia, Lebanon, Liechtenstein, Lithuania, Luxembourg,
Macau, Malawi, Malaysia, Malta, Mauritius, Mexico, Moldova, Monaco, Montenegro,
Morocco, Myanmar, Namibia, Nepal, the Netherlands, New Zealand, Nigeria, North
Macedonia, Norway, Oman, Pakistan, Panama, Peru, the Philippines, Poland,
Portugal, Qatar, Romania, Russia, San Marino, Saudi Arabia, Senegal, Serbia,
Singapore, Slovakia, Slovenia, South Africa, South Korea, Spain, Sri Lanka, Sweden,
Switzerland, Taiwan, Tajikistan, Tanzania, Thailand, Trinidad and Tobago,
Tunisia, Türkiye, Turkmenistan, Uganda, Ukraine, the United Arab Emirates, the
United Kingdom (three bank-holiday jurisdictions), the United States, Uruguay,
Uzbekistan, Vietnam, Zambia, Zimbabwe.

**Twenty traditions.** Christianity under both computations, the General
Roman Calendar with the rank of every celebration (`roman_calendar`), the
Ethiopian Orthodox Tewahedo and the Coptic Orthodox Churches, Islam,
Judaism, the Bahá'í Faith, Hinduism, Jainism (Paryuṣaṇa and Daśa Lakṣaṇa
counted back from their last days), Sikhism on the Nanakshahi calendar of
2003, Buddhism (partial), Chinese folk tradition, Shinto with the imperial
court rites beside it, the Wheel of the Year in both hemispheres, and the
Zoroastrian schedule of feasts on each of its three reckonings.

The Ethiopian entry is worth a word. Its fixed feasts are ordinary dates —
29 Tahsas, 11 Tirr — in the Ethiopic calendar, and until `CalendarSystem`
stopped being a closed enum none of them could be written down at all. Its
movable cycle follows *Bahire Hasab*, whose arithmetic is its own but whose
rule is the Alexandrian computus: Tinsae is the Orthodox Pascha every year,
so the cycle is written as the *tewsak* offsets from the Julian-computus
Easter, from the Fast of Nineveh to Paraclete, and three years of anchors
check it.

Japan is complete back to 1948. Every other country is the **present-day
national list**, with historical `valid_from` / `valid_until` years wherever a
change is named in the cited source — the US Uniform Monday Holiday Act, the
French suspension of 8 May, Italy's 1977–2000 Republic Day, Portugal's
2013–2015 austerity suspensions, Denmark's abolition of Store bededag in 2024,
Korea's successive extensions of the 대체공휴일, and so on. None of them
claims to be complete back to its own founding.

## Exchange calendars

`exchanges` carries the trading calendars of stock exchanges as rule sets
keyed by ISO 10383 Market Identifier Code: the days an exchange is closed, as
public-kind entries that stop business-day arithmetic, and the days it closes
early or opens late, as observances that do not. Nineteen so far, each from
the exchange's own published calendar: the New York Stock Exchange (`XNYS`)
and Nasdaq (`XNAS`) on one calendar, which closes on
Good Friday, which no statute makes a holiday, trades on Columbus Day and
Veterans Day, moves a Saturday holiday to the Friday before except a New
Year's Day, whose Friday is the last day of the year, and closes at 1:00 p.m.
the day after Thanksgiving and on 3 July and Christmas Eve when those fall on
a Monday to Thursday; the Toronto Stock Exchange (`XTSE`); the Frankfurt
Stock Exchange on Xetra (`XETR`), which closes on Christmas Eve and New Year's
Eve, trades on Ascension Day and Corpus Christi and moves nothing off a
weekend; SIX in Zurich (`XSWX`), which closes on Berchtoldstag, Ascension Day
and Whit Monday besides and also moves nothing; the Australian Securities Exchange (`XASX`), which trades on the
states' Monday for a Sunday Anzac Day; and Euronext's seven markets, four of
them — Amsterdam, Brussels, Lisbon, Paris — on one calendar of six closed days
and two half days, Dublin moving a weekend holiday where the others leave it,
Milan closing on Ferragosto and both eves, Oslo keeping the Norwegian days and
halving the Wednesday before Easter; and B3 in São Paulo (`BVMF`), closed
on Carnival Monday and Tuesday, Christmas Eve and the last weekday of the
year, and open from 1:00 p.m. on Ash Wednesday; and Tokyo (`XJPX`) and Hong
Kong (`XHKG`), which close on every national or general holiday and add days
of their own — Tokyo 2 and 3 January and 31 December, Hong Kong three half
days — and so include their countries' tables through `includes` rather than
copy them; London (`XLON`), which includes the United Kingdom's table for
England and Wales, the region the inclusion names, and halves the last
weekdays before Christmas and of the year; Seoul (`XKRX`), which includes
South Korea's table and adds 1 May, a day off for employees before it was a
public holiday, and the last weekday of the year, and which matches the
Korea Exchange's own closure lists for every year from 2009 to 2029;
Shanghai (`XSHG`), which includes China's table — its annual arrangements,
without the weekend days they make working days — and adds the eve of the
Spring Festival 2024, and which matches the exchange's closure notices for
2014 to 2026; and Nasdaq's Nordic markets — Copenhagen (`XCSE`), Stockholm
(`XSTO`), Helsinki (`XHEL`) and Iceland (`XICE`) — each on its country's
days as Nasdaq's calendar lists them, Stockholm with five half days a year.
The unscheduled closures a read source
records — the September 2001 attacks, Hurricane Sandy, the day of mourning
for President George H. W. Bush — are data in the New York table; those no
source read here gives are not, and the table says which.

## What this crate will not do

* **It will not claim a Hijri holiday is exact.** Eid depends on a crescent
  sighting decided per country, sometimes on the night before. Every
  Hijri-dated entry is flagged `Confidence::Approximate` — including Saudi
  Arabia's, which uses its own Umm al-Qurā calendar, and Türkiye's, which
  follows the Diyanet's precomputed table. They are good predictions, not
  announcements. A test asserts that no Hijri-dated rule anywhere in the crate
  claims to be exact.
* **It will not invent a substitution rule it cannot cite.** France, Germany,
  Italy, Spain, Portugal, the Netherlands, Belgium, Switzerland, Austria, the
  Nordic countries, Poland, Czechia, Greece, China, India, Indonesia, Mexico,
  Brazil, Israel, Saudi Arabia, the Emirates, Türkiye, Egypt and the
  Philippines carry **no** substitution policy, because they have none in
  calendar-expressible form. Their holidays fall on the weekend and stay
  there. That is a deliberate refusal, not an oversight.
* **It will not guess an annual administrative act.** China's 调休, Taiwan's
  Lunar New Year makeup days, Vietnam's Tết span, Thailand's Songkran makeup
  days and Indonesia's *cuti bersama* are decided year by year by a ministry,
  not by a rule. Where the acts have been read they are data for their
  years: China's arrangements from 2008 to 2026 and Russia's transfer
  decrees from 2013 to 2027, with the weekend days they put to work as
  `Kind::Workday` entries that business-day arithmetic
  counts. A year past them is a gap. Elsewhere the statutory days are listed
  and the bridging days are not.
* **It will not pretend a list is current.** Every table carries a
  `sources_checked` date and names its statute or gazette in a comment.
* **It will not guess outside the span it evaluated.** Business-day arithmetic
  that walks off the end of a `HolidayCalendar` returns `None`.
* **It does not model evenings.** A Jewish holiday begins at sunset on the
  preceding day and a Hijri one likewise; this crate names days, not evenings.

## The years it can answer

Most rules are Gregorian arithmetic and have no end. Some are dated in a
calendar that does:

| Rules dated in | Answerable over |
| --- | --- |
| Chinese, Korean (Dangi), Vietnamese | 1645–2150 |
| Umm al-Qurā | the published table, 1300–1600 AH |
| Easter | 1583–4099 Gregorian, 326–4099 Julian |
| New Zealand's Matariki | 2022–2035, the years this crate's sources publish |

Outside those the holiday has no date, which is **not** the same as not
occurring — and an evaluated calendar used to express both by leaving it
out. `holidays_in_year(&countries::CHINA, None, 2151)` returned seven
entries instead of thirteen, with the Spring Festival, the Dragon Boat
Festival and the Mid-Autumn Festival missing and everything that survived
marked `Exact`.

Now the calendar says so:

```rust
let calendar = HolidayCalendar::for_year(&countries::CHINA, None, 2151);
assert!(!calendar.is_complete());
for gap in calendar.gaps() {
    println!("{} could not be computed for {}", gap.name, gap.year);
}
```

`gaps()` is empty for every year inside every referenced calendar's range,
which is every year a caller is likely to ask about. The holidays that *are*
returned outside it remain correct; the list is incomplete, not wrong.

One residue, stated because it is small and real: a rule shifted from
another day — 除夕 is 春節 minus one — can still be missed when its base
falls in an out-of-range year *and* within the shift of a year boundary.

## What is approximate, and why

| Entry | Why |
| --- | --- |
| Every Hijri-dated holiday, in forty-six countries and the Islamic tradition table | the observed date is a sighting decision, per country |
| Vesak, Makha Bucha, Asalha Bucha, Khao Phansa (Thailand, Indonesia, Singapore, Malaysia, the Buddhist table) | dated by the Thai lunar calendar, which this crate does not have; approximated as the full moon of Chinese lunar month *n* − 2. Right in 2022, 2024 and 2025; a day out in 2023; a month out in a Thai intercalary year |
| Nepal's festivals — Buddha Jayanti, Dashain, Tihar and the rest | each is a tithi read at Kathmandu, and the part of the day it must hold is fitted to the notices of 2080–2083 BS, which it reproduces, rather than quoted from the almanac |

## Deliberate gaps

* **India** carries the three national holidays and the gazetted list, with
  Holi, Ram Navami, Mahavir Jayanti, Buddha Purnima, Janmashtami, Dussehra,
  Diwali and Guru Nanak's Birthday computed on `hindu-lunar`; the states' own
  days are not carried. Singapore's and Malaysia's Deepavali is the same
  Dīpāvalī rule. Nyepi is still missing from Indonesia: the Balinese Saka
  calendar that dates it is not in the crate, and the crate will not tabulate
  what it cannot compute.
* **Nepal** carries the holidays of its Home Ministry's notices for every
  office in the country, except Chhath, which no one tithi rule puts where all
  four notices do. The holidays for one community, region or group — the
  Kathmandu Valley's *jātrā*s, Fagu Purnima's two dates, the days for women
  or for the Newar community — are not carried.
* **Subdivisions** are modelled only where a statute names them. German
  *Länder*, US federal-versus-state, Australian states, Canadian provinces, UK
  jurisdictions and French Alsace-Moselle are in; Swiss cantons, Spanish
  autonomous communities, Italian patron-saint days, Malaysian states and New
  Zealand anniversary days are not.

## Business days

The weekend is data. Saturday–Sunday is the common case; Friday–Saturday holds
in Egypt, Israel and Saudi Arabia today; Thursday–Friday held in Saudi Arabia
until its June 2013 royal order; the Emirates moved to Saturday–Sunday on
1 January 2022; and Nepal kept a one-day Saturday weekend until April 2026.
All five are exercised by the test suite.

```rust
use hc_calendars_solar::gregorian::to_fixed;
use hc_holiday::countries::JAPAN;
use hc_holiday::engine::HolidayCalendar;

let calendar = HolidayCalendar::new(&JAPAN, None, 2024, 2024);
// Golden Week: one business day after Friday 26 April is Tuesday 30 April,
// because 29 April is 昭和の日.
assert_eq!(
    calendar.add_business_days(to_fixed(2024, 4, 26)?, 1),
    Some(to_fixed(2024, 4, 30)?)
);
```

`business_days_between` counts the half-open interval `[start, end)`, so the
count from Monday to Wednesday plus the count from Wednesday to Friday is the
count from Monday to Friday, and a reversed interval gives a negative answer.

## Where the data came from

Every table names its source in a comment on the `RuleSet` and carries the
date the sources were checked. The statutes cited include the Japanese 祝日法
and every amending act through 令和2年法律第68号; 5 U.S.C. § 6103 and the
executive orders behind the in-lieu-of rule; the UK Banking and Financial
Dealings Act 1971 and the proclamations under section 1(3); the sixteen German
*Feiertagsgesetze*; the French Code du travail and the Code du travail local
d'Alsace-Moselle; 《全国年节及纪念日放假办法》; Taiwan's 紀念日及節日實施條例;
Korea's 관공서의 공휴일에 관한 규정; the Ley Federal del Trabajo; the Canada
Labour Code; the Te Kāhui o Matariki Public Holiday Act 2022; and the Public
Holidays Act 36 of 1994. The two Easter algorithms are Butcher's and
Delambre's arrangements as Meeus gives them in *Astronomical Algorithms*,
chapter 8.

## Accuracy claimed

* **Easter, both computations: exact.** Verified against fifteen published
  Western dates and twelve Orthodox ones, and checked structurally over
  1583–2500 (always a Sunday, always between 22 March and 25 April).
* **Hebrew-dated holidays: exact**, because the Hebrew calendar is arithmetic.
* **Chinese, Korean and Vietnamese lunisolar dates: exact to the astronomical
  model** of `hc-calendars-lunar`, whose lunar conjunctions land within about a
  minute.
* **Solar-term holidays: exact to the model**, with the day-boundary caveat
  above. Zero disagreements against Japan's published equinox days, 1980–2099.
* **Everything else: exact as stated in the cited statute**, subject to the
  refusals above.

Every table is anchored by tests — Japan amendment by amendment, every other
country on its cited dates, every tradition on a few years of its cycle — and
`cargo test -p hc-holiday` lists them; a count written here would only drift.
