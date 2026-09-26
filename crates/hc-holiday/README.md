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
| `computus` | Easter, Gregorian and Julian, the astronomical reckoning at Jerusalem, and the offsets keyed to it |
| `engine` | evaluation, and business-day arithmetic |
| `hindu` | the Hindu festival rules the traditions and the national tables share |
| `traditions` | the cross-cutting religious cycles |
| `lectionary` | the lectionary cycles: the Sunday and weekday years and the RCL's Propers |
| `roman_calendar` | the General Roman Calendar: every celebration with its rank, and the decrees since 2002 |
| `international` | the United Nations international days, each citing its resolution |
| `exchanges` | 42 exchange calendars: New York, Nasdaq, Toronto, Mexico City, São Paulo, London, Frankfurt, Zurich, Vienna, Madrid, Warsaw, Moscow, Istanbul, Euronext's seven markets, Nasdaq's four Nordic markets, Johannesburg, Tel Aviv, Riyadh, Tokyo, Seoul, Shanghai, Shenzhen, Taipei, Hong Kong, Mumbai's NSE and BSE, Bangkok, Singapore, Kuala Lumpur, Jakarta, Manila, Sydney, NZX |
| `countries` | 195 national tables |

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
TibetanDay { calendar, month, day }   Tsagaan Sar, days 1–3 of the first month in mongolian
Offset { base, days }                 Seollal's eve, 除夕
Span { from, to }                     Dashain, Phūlpātī to Āśvina śukla 12, six or seven days
MovedByWeekday { base, moves }        Argentina's feriados trasladables, Colombia's Ley Emiliani
Tabulated { function, first, last }   Matariki, gazetted through a stated last year
Computed(fn)                          the handful that really are bespoke
```

Modifiers are data too: `SubstitutionPolicy` (which weekdays move a holiday,
which way, whether the search steps past a day already taken, and whether two
holidays colliding count), `BridgePolicy` (Japan's 国民の休日),
`WeekendPolicy` (which days are the weekend, over stated years and, where
the law changed mid-year, from the day it took effect),
`valid_from` / `valid_until` on every rule, `regions` for subdivision scoping,
and `Kind` for public / bank / religious / observance. A set can also
`include` other sets — an exchange on its country's calendar — each evaluated
under its own policies, with its days off merged in and its observances left
behind.

A rule is `Computed` only where its statute is written as a sentence rather
than a pattern: Ireland's St Brigid's Day, the Dutch royal day (under two
monarchs), US Inauguration Day, Mexico's presidential handover, Israel's Yom
HaAtzmaut and some thirty more in the national tables, most of them a move
that depends on the weekday or on another holiday — Chile's September days,
Costa Rica's tourism Mondays of 2020–2024, Hong Kong's make-up days of
1983–2011, Myanmar's Thingyan. New Zealand's Matariki is `Tabulated`, a
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
`Meridian::JAPAN` and lets `hc-seasons` answer. Against the 102 equinox days
the Observatory published for 1980 to 2030, transcribed into `hc-seasons`'s
test, that disagrees nowhere; the Observatory publishes one year ahead, so
for 2031–2099 the same test compares the computation with a floor formula
that reproduces the published table — a prediction, not a publication, and
the formula is the test's own. The underlying solar
longitude is VSOP87, good to about 1″, so an equinox lands within the minute
the almanacs round to; only an equinox within about a minute of JST midnight
could still be given the wrong *day*, and the tightest case in the modern
record, the autumn equinox of 2012 at 23:49 JST, is eleven minutes clear.

## Coverage

**195 countries.** Afghanistan, Albania, Algeria, Andorra, Angola, Antigua and Barbuda,
Argentina, Armenia, Australia (eight states and territories), Austria,
Azerbaijan, the Bahamas, Bahrain, Bangladesh, Barbados, Belarus, Belgium,
Belize, Benin, Bhutan, Bolivia, Bosnia and Herzegovina, Botswana, Brazil,
Brunei, Bulgaria, Burkina Faso, Burundi, Cabo Verde, Cambodia, Cameroon, Canada
(federal plus the provincial days fixed by statute), Chad, Chile, China,
Colombia, Comoros, Costa Rica, Côte d'Ivoire, Croatia, Cuba, Cyprus, Czechia, Democratic
Republic of the Congo, Denmark, Djibouti, Dominica, the Dominican Republic,
Ecuador, Egypt, El Salvador, Equatorial Guinea, Estonia, Eswatini, Ethiopia, Fiji, Finland, France (métropole
plus Alsace-Moselle), Gabon, the Gambia, Georgia, Germany (all 16 *Länder*), Ghana, Greece,
Grenada, Guatemala, Guinea, Guinea-Bissau, Guyana, Haiti, Honduras, Hong Kong, Hungary,
Iceland, India, Indonesia, Iran, Iraq, Ireland, Israel, Italy, Jamaica, Japan,
Jordan, Kazakhstan, Kenya, Kiribati, Kuwait, Kyrgyzstan, Laos, Latvia, Lebanon,
Lesotho, Liberia, Libya, Liechtenstein, Lithuania, Luxembourg, Macau, Madagascar, Malawi,
Malaysia, the Maldives, Mali, Malta, the Marshall Islands, Mauritania,
Mauritius, Mexico, Micronesia, Moldova, Monaco, Mongolia, Montenegro, Morocco,
Mozambique, Myanmar, Namibia, Nauru, Nepal, the Netherlands, New Zealand,
Nicaragua, Niger, Nigeria, North Korea, North Macedonia, Norway, Oman, Pakistan, Palau, Palestine,
Panama, Papua New Guinea, Paraguay, Peru, the Philippines, Poland, Portugal,
Qatar, Republic of the Congo, Romania, Russia, Rwanda, Saint Kitts and Nevis,
Saint Lucia, Saint Vincent and the Grenadines, Samoa, San Marino, Saudi Arabia,
Senegal, Serbia, Seychelles, Sierra Leone, Singapore, Slovakia, Slovenia, the Solomon Islands,
Somalia, South Africa, South Korea, South Sudan, Spain, Sri Lanka, Sudan, Suriname, Sweden, Switzerland,
Syria, Taiwan, Tajikistan, Tanzania, Thailand, Timor-Leste, Togo, Tonga, Trinidad and
Tobago, Tunisia, Türkiye, Turkmenistan, Tuvalu, Uganda, Ukraine, the United Arab
Emirates, the United Kingdom (three bank-holiday jurisdictions), the United
States, Uruguay, Uzbekistan, Vanuatu, Vatican City, Venezuela, Vietnam, Yemen,
Zambia, Zimbabwe.

**Forty-three traditions.** Western Christianity on the Gregorian computus,
the General Roman Calendar with the rank of every celebration
(`roman_calendar`), Orthodox Christianity with its fixed feasts on the
Julian calendar and, as a second table, on the Revised Julian, the
Ethiopian Orthodox Tewahedo and the Coptic Orthodox Churches, the Armenian
Apostolic Church on the Gregorian calendar of Etchmiadzin
(`christian-armenian`) and the Julian of the Patriarchate of Jerusalem
(`christian-armenian-jerusalem`), the Ember and Rogation Days of the 1662
Prayer Book (`ember-bcp1662`) and of *Common Worship*'s traditional weeks
(`ember-common-worship`) and the Rogation Days of the Roman rubrics of 1960
(`rogation-roman-1960`), Islam, Judaism with Ta'anit Esther and Sh'ela, the
Samaritan festivals on `samaritan`, the Mandaean feasts and *mbattal* days
on `mandaean`, the Yazidi feasts on the Eastern calendar, the Bahá'í Faith,
Hinduism, Jainism (Paryuṣaṇa and Daśa Lakṣaṇa counted back from their last
days), Sikhism on the Nanakshahi calendar of 2003 (`sikh-nanakshahi-2003`),
Buddhism as Thailand dates its four holy days on `thai-lunar`
(`buddhist-thai`) and its uposatha days, แรม 14 or 15 ค่ำ by the month's
length (`buddhist-uposatha-thai`), as Japan and the Chinese calendar date the
East Asian days (`buddhist-east-asian`) and as the Tibetan calendar dates the
*düchen* (`buddhist-tibetan`), a skipped or doubled day or month reported as
a gap, Chinese folk tradition with 人日, 上巳 and 寒食, the Little New Year in
five regional tables (`chinese-xiaonian-north`, `-south`, `-jiangnan`,
`-nanjing`, `-southwest`), Taoism's three Yuan and Mazu's days (`taoist`),
the Korean folk days on `dangi` with 한식 105 days after 동지
(`korean-folk`), the Vietnamese on `vietnamese` (`vietnamese-folk`), Japan's
五節句 on their Gregorian dates from 1873 (`gosekku`), Shinto with the
imperial court rites beside it, the Wheel of the Year in both hemispheres,
the Zoroastrian schedule of feasts on each of its three reckonings, Plough
Monday, Plough Sunday and Distaff Day (`plough-days`), and Chaharshanbe Suri
on `persian`, the eve of the year's last Wednesday (`chaharshanbe-suri`).

Beside the tables, `computus` carries a third reckoning of Easter, the
astronomical one at the meridian of Jerusalem that the World Council of
Churches proposed at Aleppo in 1997 (`astronomical-jerusalem`), which no
church keeps; and `lectionary` gives the year of the Sunday cycle, A, B or
C, the Roman weekday cycle, I or II, and the Revised Common Lectionary's
Proper of a Sunday after Trinity — the rules, not the copyrighted
readings.

The Ethiopian entry is worth a word. Its fixed feasts are ordinary dates —
29 Tahsas, 11 Tirr — in the Ethiopic calendar, which a rule can name because
`CalendarSystem` is open to any calendar the registry holds rather than a
closed list. Its movable cycle follows *Bahire Hasab*, whose arithmetic is
its own but whose rule is the Alexandrian computus: Tinsae is the Orthodox Pascha every year,
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
early or opens late, as observances that do not. Forty-two so far, each
from the exchange's own published calendar: the New York Stock Exchange (`XNYS`)
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
2014 to 2026; Taipei (`XTAI`), which includes Taiwan's table without the
Saturdays it works, and adds Labour Day before it was a government holiday
and the two settlement-only days before the Lunar New Year break, and which
matches the exchange's schedules for 2023 to 2026; and Nasdaq's Nordic markets — Copenhagen (`XCSE`), Stockholm
(`XSTO`), Helsinki (`XHEL`) and Iceland (`XICE`) — each on its country's
days as Nasdaq's calendar lists them, Stockholm with five half days a year.
The unscheduled closures a read source
records — the September 2001 attacks, Hurricane Sandy, the day of mourning
for President George H. W. Bush — are data in the New York table; those no
source read here gives are not, and the table says which.

Ten more follow their exchanges' own lists. Johannesburg (`XJSE`), Mexico
City (`XMEX`), Warsaw (`XWAR`) and NZX (`XNZE`) include their
countries' tables: Johannesburg adds the days the President declared and
closes at noon on the December days its schedules name, which are carried for
2023 to 2025 and are a gap in other years; Mexico City adds Holy Thursday, Good Friday,
2 November and 12 December, the CNBV's closing days, and matches the lists for
2019 to 2026; Warsaw adds Good Friday, Christmas Eve before it was a public
holiday and New Year's Eve, and matches its lists for 2019 to 2027; NZX adds
the Queen Elizabeth II holiday of 2022 and its two abbreviated days, and
trades on the regional anniversary days. Vienna (`XWBO`) closes on 26 October
and five Christmas and New Year days, trades on six Austrian holidays and, from
2023, on Whit Monday; Madrid (`XMAD`) closes on six days and trades to 14:00 on
24 and 31 December. Moscow (`MISX`) does not close on Russia's days off: it
closes on the holiday dates themselves, trades on the rest and on the working
Saturdays, and in 2026 holds only its weekend session on four holidays; Tel
Aviv (`XTAE`), which moved from a Sunday–Thursday to a Monday–Friday week in
January 2026, Riyadh (`XSAU`), on a Sunday–Thursday week with its Eid holidays
as announced, and Istanbul (`XIST`), with the Bayram days and the half-day eves
of its tables, are carried as their lists give each year — 2023 to 2026 for
Moscow and Riyadh, 2024 to 2027 for Tel Aviv, 2019 to 2026 for Istanbul — and
a year outside them is a gap.

Eight more are in Asia. Shenzhen (`XSHE`) includes China's table on the
Shanghai rules, its notices for 2015 to 2026 closing the same days. The other
seven set their days each year and are carried as their lists give them, a
year outside them a gap: Bangkok (`XBKK`, from its holiday pages for 2022 to
2027, which substitute a weekday for a Saturday holiday as well as a Sunday
one and add a special holiday most years); the National Stock Exchange of
India (`XNSE`) and BSE (`XBOM`), on one list from their circulars and notices
for 2020 to 2026 — Maharashtra's holidays, the days added for elections, the
Diwali Laxmi Pujan holiday on which a Muhurat session is held, carried as a
closure whose name says so, and the Union Budget and other weekend sessions,
carried as working days; Singapore (`XSES`), which prints no closure list but
states that it follows the Ministry of Manpower's calendar, so the Ministry's
holidays for 2020 to 2026, Polling Days included, are carried under that rule
with the half days SGX prints; Kuala Lumpur (`XKLS`, from its calendar pages
for 2020 to 2026, with the Mondays its own note gives for a Sunday holiday and
the half-day eves it printed to 2023); Jakarta (`XIDX`, from its calendars for
2020 to 2022 and 2024 to 2026, with the joint-leave and election days and its
own 31 December, 2023 a gap since its calendar could not be read); and Manila
(`XPHS`, from its memoranda for 2020 to 2026, with the suspensions for the Taal
ash, the quarantine, a typhoon, a technical failure and floods).

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
  swaps of a working day for a Saturday, Vietnam's Tết span and *làm bù*,
  Thailand's Songkran makeup days and Indonesia's *cuti bersama* are decided
  year by year by a ministry, not by a rule. Where the acts have been read
  they are data for their years: China's arrangements from 2008 to 2026,
  Russia's transfer decrees from 2013 to 2027, Taiwan's swaps from 2017
  until they ended in 2025 and Vietnam's notices from 2021 to 2026, with the
  weekend days they put to work as `Kind::Workday` entries that
  business-day arithmetic counts. A year past them is a gap. Elsewhere the
  statutory days are listed and the bridging days are not.
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
| Hindu lunisolar, Bikram Sambat | 1700–2300 |
| Thai lunar | 1992–2027, the years Thailand has published |
| Badíʿ, as kept | to 19 March 2065, where the Bahá'í World Centre's table ends |
| Easter | 1583–4099 Gregorian, 326–4099 Julian |
| New Zealand's Matariki | 2022–2035, the years this crate's sources publish |
| Nauru's declared days | 2023, 2024 and 2026, the years whose gazettes this crate read |

The same holds for every day a table takes from a government's yearly
notices rather than from a rule — Sri Lanka's Poya days, Cambodia's Khmer
New Year, Fiji's lists and the rest: each table's documentation names the
years read.

Outside those the holiday has no date, which is **not** the same as not
occurring, and a holiday list alone cannot tell the two apart:
`holidays_in_year(&countries::CHINA, None, 2151)` gives seven entries
instead of thirteen, with the Spring Festival, the Dragon Boat Festival and
the Mid-Autumn Festival missing and everything that remains marked `Exact`.

So the calendar also says what it could not compute:

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
| Every Hijri-dated holiday, in seventy-one countries and the Islamic tradition table | the observed date is a sighting decision, per country. Indonesia's, Singapore's and Malaysia's are the published lists' for 2020–2026 and exact, and for 2027 the lists' announcements; the Philippines' the proclamations' for 2020–2026 |
| Vesak in Indonesia, Singapore and Malaysia outside 2020–2027 | the full moon of the fourth Chinese month, which is Singapore's date in every year of 2020–2027 and misses the others' in some: Malaysia's Wesak Day 2023 was 4 May, a month before it, and Indonesia's Waisak a day or two after it in 2022–2024. Inside those years it is the lists' date. Thailand's four Buddhist days, and the `buddhist-thai` table's, are exact on `thai-lunar` for 1992–2027 and gaps outside |
| Nepal's festivals — Buddha Jayanti, Dashain, Tihar and the rest | each is a tithi read at Kathmandu, and the part of the day it must hold is fitted to the notices of 2080–2083 BS, which it reproduces, rather than quoted from the almanac |
| Bangladesh's Buddha Purnima | the notifications' own dates for 2025 and 2026, which they star as depending on the moon |
| Mongolia's Tsagaan Sar, Buddha's Birthday and Chinggis Khaan Day; Bhutan's Losar, Buddhist days and Traditional Day of Offering | the day the law or the Ministry's list states on the Tibetan calendar, `mongolian` or `tibetan-bhutan`, outside the years read (Tsagaan Sar 2025–2026, Bhutan's lists 2025–2026); the Government or the Ministry settles each year, and Bhutan's calendar of 2003 had Losar a day before the arithmetic |

## Deliberate gaps

* **India** carries the three national holidays and the gazetted list, with
  Holi, Ram Navami, Mahavir Jayanti, Buddha Purnima, Janmashtami, Dussehra,
  Diwali and Guru Nanak's Birthday computed on `hindu-lunar`; the states' own
  days are not carried. Singapore's and Malaysia's Deepavali is the same
  Dīpāvalī rule. Indonesia's Nyepi is dated by the Balinese Śaka lunisolar
  calendar, which is not in the crate (`balinese-pawukon` is the 210-day
  wuku cycle, a different reckoning): it is carried from the joint decrees
  for 2020–2027 and is a gap in any other year.
* **Nepal** carries the holidays of its Home Ministry's notices for every
  office in the country, except Chhath, which no one tithi rule puts where all
  four notices do. The holidays for one community, region or group — the
  Kathmandu Valley's *jātrā*s, Fagu Purnima's two dates, the days for women
  or for the Newar community — are not carried.
* **Bangladesh** carries the general and executive-order holidays of the
  Ministry of Public Administration's notifications. Janmashtami, the Durga
  Puja's Navami and Bijoya Dashami, and Buddha Purnima are those
  notifications' dates for 2025 and 2026 — the Indian rules miss each by a
  day in one of the two years — and a later year reports them as gaps. The
  optional holidays are not carried.
* **Mongolia** and **Bhutan** date their lunar days on the Tibetan
  calendar, `mongolian` and `tibetan-bhutan`, with `TibetanDay`. Where the
  calendar skips or repeats the day's number, the sources read do not say
  where the holiday goes — three days off were reported for Mongolia's 2022,
  its third number skipped, and two were counted in 2025, its first
  skipped — so that year is a gap. Bhutan's Winter Solstice, Blessed Rainy Day
  and Dassain are its Ministry's lists for 2025 and 2026, and gaps in other
  years.
* **Cambodia** carries the days off of the Royal Government's sub-decrees
  for 2025, 2026 and 2027. Khmer New Year, Visak Bochea, the Royal Ploughing
  Ceremony, Pchum Ben and the Water Festival are dated on the Khmer calendar,
  which the crate does not have, or by the palace; they are the sub-decrees'
  dates, and another year reports them as gaps.
* **Laos** carries the official holidays of its Decree on Holidays. Lao New
  Year is the dates of the Prime Minister's Office's notices for 2024 to
  2026, and another year reports it as a gap; the Lao Women's Union's day,
  off for women alone, is not carried.
* **Papua New Guinea** carries only the days its Public Holidays Act fixes.
  Independence Day, the Sovereign's Birthday and any other day are appointed
  by notice in the National Gazette, and no gazette was read.
* **Subdivisions** are modelled only where a statute names them. German
  *Länder*, Australian states and territories, Canadian provinces, UK
  jurisdictions, French Alsace-Moselle, the three units of Bosnia and
  Herzegovina and Bangladesh's hill districts are in, and so are a few
  single places: Inauguration Day in the District of Columbia, Chișinău,
  Guatemala City, San Salvador, Managua and Chile's Arica and Parinacota.
  US states, Swiss cantons, Spanish autonomous communities, Italian
  patron-saint days, Malaysian states and New Zealand anniversary days are
  not.

## Business days

The weekend is data. Saturday–Sunday is the common case; Friday–Saturday holds
in Bangladesh, the Maldives and most of the Arab states today;
Thursday–Friday held in Saudi Arabia until its royal order took effect on
29 June 2013; the Emirates moved to Saturday–Sunday on 1 January 2022; Nepal
kept a one-day Saturday weekend until 6 April 2026; and Brunei rests on
Friday and Sunday and works the Saturday between. A weekend rule carries the
day it took effect where the source gives one, so a change in the middle of
a year changes the arithmetic on that day and not on 1 January. All six are exercised by the test suite. A
few tables rest on one day: Iran and Djibouti the Friday, Israel the
Saturday, which is its only statutory day of rest, and Cuba, Cambodia,
Timor-Leste and the Vatican the Sunday.

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
  above. Zero disagreements against Japan's published equinox days, 1980–2030,
  and against the floor formula that extends them to 2099.
* **Everything else: exact as stated in the cited statute**, subject to the
  refusals above.

Every table is anchored by tests — Japan amendment by amendment, every other
country on its cited dates, every tradition on a few years of its cycle — and
`cargo test -p hc-holiday` lists them; a count written here would only drift.
