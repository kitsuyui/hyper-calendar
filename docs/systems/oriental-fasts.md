# The fasts of the Armenian, Coptic and Ethiopian churches

Backs `hc-holiday`'s `oriental_fasts`, the periods of four reckonings of
`orthodox_fasts`: `armenian-fasts` and `armenian-fasts-jerusalem`,
`coptic-fasts` and `ethiopian-fasts`.

## What it is

The Oriental Orthodox churches fast more than the Eastern Orthodox do,
and each on a scheme of its own. All three keep Wednesdays and Fridays as
fast days, as the Eastern churches do, and lift them in seasons of joy;
beyond that they differ.

- **The Armenian Apostolic Church** fasts in weeks: a week of fasting,
  Monday to Friday, before each of its great feasts, its eve the Sunday
  before. Great Lent is 48 days, from the seventh Monday before Easter to
  Holy Saturday. About 160 days of the year are fast days, one church
  site counts [arak29-fasts]. Etchmiadzin keeps the Gregorian calendar and
  computus it took up in 1923; the Armenian Patriarchate of Jerusalem the
  Julian [armenian-church-sydney].
- **The Coptic Orthodox Church** fasts in seasons, each dated in the
  Coptic calendar: the Nativity Fast of 43 days, Jonah's Fast, Great Lent
  of 55, the Apostles' Fast, the Fast of the Virgin; and the Paramoun, the
  eve of the Nativity and of Theophany [st-takla-fasts].
- **The Ethiopian Orthodox Tewahedo Church** keeps seven fasts, dated in
  the Ethiopic calendar: Tsome Nebiyat before Genna, the Gahad, Tsome
  Nenewe, Abiy Tsom, Tsome Hawaryat, Tsome Filseta and the Wednesdays and
  Fridays, Tsome Dihnet [ethiopianorthodox-org-calendar].

The Coptic and Ethiopian churches keep Easter by the Alexandrian computus,
which is the Julian Pascha.

## How it works

A scheme is a list of periods, each bounded by a number of days from
Easter, a date of a calendar, or a number of days from the Sunday nearest
a date. A day is tested against the periods in order, the fast-free ones
first; a Wednesday or Friday in no period is a fast day, and any other day
is not. This is the Eastern Orthodox scheme's machinery
([orthodox-fasts.md](orthodox-fasts.md)), with each church's periods.

**Armenian.** The Armenian Diocese of Georgia gives each weekly fast with
its eve, "the Sunday preceding the week prior to the feast", the fast
"lasting from Monday to Friday", and lists the eves of 2020 to 2030
[armenian-church-georgia-fasts]. The eves of those lists fit, in every
year, a day counted from Easter or from the Sunday nearest a date, and
the fasts are the five days after:

| Fast | Eve | The fast |
| --- | --- | --- |
| The Catechumens (Aradjavorats) | Easter − 70 | Easter − 69 to − 65 |
| Great Lent and Holy Week | Easter − 49 | Easter − 48 to − 1, 48 days |
| Elijah | Pentecost, Easter + 49 | Easter + 50 to + 54 |
| St Gregory the Illuminator | Easter + 70 | Easter + 71 to + 75 |
| The Transfiguration (Vardavar, Easter + 98) | Easter + 91 | Easter + 92 to + 96 |
| The Assumption, the Sunday nearest 15 August | the Sunday before it | the week before it |
| The Exaltation, the Sunday nearest 14 September | the Sunday before it | the week before it |
| The Holy Cross of Varak | the Sunday after the Exaltation | the week after it |
| Advent (Hisnag) | the Sunday nearest 18 November | the week after it |
| St James of Nisibis | three weeks after the Advent eve | the week after it |

The Christmas fast, "The week before Theophany", is "a six (6) day fast"
[armenian-church-sydney-fasts], 30 December to 4 January as the Western
Prelacy dates it [westernprelacy-fasts]; the Diocese of Georgia's eve of
it "is always celebrated on December 29" [armenian-church-georgia-fasts]. The Wednesdays and Fridays are fasts
"except for the forty days after Easter (until Ascension)" and "the
octave (eight day) celebration of Theophany from January 6-13"
[armenian-church-sydney-fasts].

**Coptic.** St-Takla's answer on the fasts gives each with its length and
days [st-takla-fasts]:

- the Nativity Fast, "43 يوما", "من 16 هاتور حتى 29 كيهك", from 16 Hatour
  to the Nativity; its rite adds that the Nativity is 29 Koiak, "7 يناير",
  and falls on 28 Koiak in a Gregorian leap year [st-takla-nativity-fast],
  so the fast ends on 6 January;
- Jonah's Fast, three days, two weeks before Great Lent;
- Great Lent, 55 days, with Easter;
- the Apostles' Fast "من اليوم التالي لعيد العنصرة حتى 5 أبيب", from the
  day after Pentecost to the feast of Peter and Paul on 5 Epip, so to
  4 Epip;
- the Fast of the Virgin, 15 days, to the Assumption on 16 Mesori;
- the Wednesdays and Fridays, "عدا الخمسين المقدسة و عيدي الميلاد و
  الغطاس", except in the Holy Fifty and on the Nativity and Theophany.

The Paramoun, the fast on the eve of a feast, is one day, but "إذا جاء
العيد يوم الأحد يكون البرامون يومي السبت والجمعة" and on a Monday it is the
Friday, Saturday and Sunday [st-takla-paramoun]. The Paramoun of the
Nativity falls within the Nativity Fast; that of Theophany, before
11 Tobi, is a fast of its own.

**Ethiopian.** The church's calendar page gives Tsome Nebiyat "with Sibket
on 15th Hedar" to "the 28th of Tahsas", the Gahad "on the eve of
Epiphany", Tsome Nenewe "on Monday, Tuesday and Wednesday of the third
week before Lent", and the Wednesdays and Fridays, with "no fast if the
Christmas and Epiphany fall on a Wednesday or Friday" and none from
Easter to Pentecost [ethiopianorthodox-org-calendar]. Mahibere Kidusan's
reckoning of the fasts of 2011 E.C. counts Abiy Tsom 14 days after Nenewe
and Tsome Hawaryat 119, the day after Paraclete, and Tsome Filseta from
1 Nehase [eotcmk-fasts-2011]; Tsome Hawaryat "lasts until July 12 (Hamele
5) with the observance of the Feast day of Saint Peter and Saint Paul"
[eotcmk-hawaryat-2023], and Filseta is "the 1st to the 15th of Nehasie"
[eotcmk-filseta-2016].

**Worked example: 2026.** Gregorian Easter is 5 April and the Julian
Pascha 12 April.

1. *Armenian.* The Catechumens: Easter − 69 to − 65, 26 to 30 January.
   Great Lent: 16 February to 4 April. Pentecost is 24 May, so Elijah's
   fast is 25 to 29 May. The Sunday nearest 15 August (a Saturday) is
   16 August; the Assumption's fast is 10 to 14 August. The Sunday nearest
   18 November (a Wednesday) is 15 November, so Advent's fast is 16 to
   20 November and St James's, three weeks on, 7 to 11 December.
2. *Coptic.* Jonah: Pascha − 69 to − 67, 2 to 4 February. Great Lent:
   16 February to 11 April. The Holy Fifty: 12 April to 31 May, no
   Wednesday or Friday fasted. The Apostles: 1 June to 4 Epip, 11 July.
   The Virgin: 1 to 15 Mesori, 7 to 21 August. The Nativity Fast:
   16 Hatour, 25 November, to 6 January 2027. Theophany, 11 Tobi, is
   Tuesday 19 January 2027, so its Paramoun is 18 January alone.
3. *Ethiopian.* Tsome Nebiyat of 2018 E.C.: 15 Hidar, 24 November 2025, to
   28 Tahsas, 6 January 2026. The Gahad: 10 Tirr, 18 January. Tsome
   Nenewe: 2 to 4 February. Abiy Tsom: 16 February to 11 April. Tsome
   Hawaryat: 1 June to 4 Hamle, 11 July. Tsome Filseta: 7 to 21 August.

## What is carried

- **`armenian-fasts`**, on the Gregorian calendar and computus, and
  **`armenian-fasts-jerusalem`**, the same periods on the Julian calendar
  and computus (`docs/policy.md` §5, as `christian-armenian` and
  `christian-armenian-jerusalem` split the feasts): the forty days after
  Easter and the octave of Theophany, fast-free; the Christmas fast; the
  Catechumens; Great Lent; and the eight weeks before or after the feasts
  above.
- **`coptic-fasts`**: the Holy Fifty, the Nativity on 7 January and
  Theophany, fast-free; the Nativity Fast, Jonah's Fast, Great Lent, the
  Apostles' Fast, the Virgin's Fast and the Paramoun of Theophany.
- **`ethiopian-fasts`**: the fifty days, Genna and Timkat, fast-free; Tsome
  Nebiyat, the Gahad of Timkat, Tsome Nenewe, Abiy Tsom, Tsome Hawaryat
  and Tsome Filseta.
- **Years**: 1583 to 4099 on the three Gregorian reckonings, as the
  Gregorian computus and the year numbers of the Coptic and Ethiopic
  dates' Gregorian years run; 326 to 4099 on the Julian. The periods are
  the churches' present ones; when each was adopted is not a thing the
  sources read say.

Not carried:

- **What may be eaten**, which differs by day and by degree of fast, and
  the Coptic Nativity Fast's three added days as a separate fast.
- **The Armenian days of no fast within the five tabernacle feasts.** A
  Yerevan parish's article says there are no Wednesday or Friday fasts
  during the days those feasts are kept [surb-zoravor-oraphaq], without
  saying which days.
- **The conventions the sources disagree on**, below; each is named, and
  one reading is carried.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| The Armenian eves of 2025 and 2026 of eight weekly fasts and Great Lent [armenian-church-georgia-fasts]; the Catechumens "January 25-30" and Great Lent "February 16 - April 04" 2026 [armenian-mother-see-calendar-2026] | `the_armenian_fasts_follow_the_eves_the_diocese_prints` | 18 of 18, and both 2026 spans |
| The Jerusalem reckoning's eves of 2026, the "old calendar" column of a Yerevan church's Tonatsuyts [surb-zoravor-tonatsuyts-2026] | `the_jerusalem_reckoning_is_the_old_calendar_column` | 11 of 11 |
| The Coptic fasts of 2023–2028: Jonah's, the Great Fast's first day, the Apostles', the Virgin's and the Nativity's first and last days [suscopts-fasts] | `the_coptic_fasts_are_the_metropolis_calendar_of_2023_to_2028` | 6 of 6 years, every day |
| The Paramoun before a Sunday, a Monday and a Saturday Theophany | `the_coptic_paramoun_runs_back_from_a_sunday_or_monday_theophany` | from the rule |
| Abiy Tsom of 2016 and 2017, Tsome Nenewe of 2017 and 2019, Tsome Nebiyat of 2022–23, Tsome Hawaryat of 2019 [eotcmk-great-lent-2016; eotcmk-great-lent-2017; eotcmk-nineveh-2017; eotcmk-nebiyat-2022; eotcmk-fasts-2011] | `the_ethiopian_fasts_are_the_dates_mahibere_kidusan_prints` | 7 of 7 |

The Diocese of Georgia's lists have slips: Pentecost of 2023 and 2026 on a
Saturday and a Wednesday, and the Transfiguration's eve of 2024 and 2028
a day or a month off the rule every other year keeps. They are the
lists', and the rule is carried.

**Where the sources disagree.**

- *The Armenian fast-free weeks after Easter.* Forty days, to the
  Ascension, in the Sydney parish's page and the Western Prelacy's
  [armenian-church-sydney-fasts; westernprelacy-fasts]; "the fifty days
  following Easter" on arak29's [arak29-fasts]. Forty is carried.
- *The Armenian Christmas fast.* Six days, "December 30 – January 4", in
  Sydney's and the Western Prelacy's pages; the Diocese of Georgia's ends
  it "on January 5, the eve of the feast". The six days are carried.
- *The Armenian Fast of Elijah of 2026.* The Diocese of Georgia's rule and
  the Yerevan Tonatsuyts put it in the week after Pentecost, 25–29 May; a
  Mother See calendar page dates its eve 31 May
  [armenian-mother-see-calendar-2026]. The week after Pentecost is carried.
- *Great Lent's length.* The Mother See's text says forty days and Holy
  Week; the Diocese of Georgia 48, the span the Mother See's dates give.
  Ethiopian Abiy Tsom is "56 days" on the church's calendar page and 55,
  Easter − 55 to Holy Saturday, in Mahibere Kidusan's reckoning; 55 is
  carried.
- *Tsome Nebiyat's length*: 40 days on the calendar page, 43 and 44 in
  Mahibere Kidusan's articles; 15 Hidar to 28 Tahsas, 44 days counted
  inclusively, is carried.
- *Tsome Filseta*: 16 days on the calendar page, 1–15 Nehase in Mahibere
  Kidusan's article; the fifteen days are carried.
- *The Ethiopian Gahad.* The calendar page lists the eves of Genna and
  Timkat as fasts; a parish page says the Gahad is kept on a Tuesday or
  Thursday when the feast falls on a Wednesday or Friday
  [eotc-ma-fasts]. The eve of Timkat is carried as a fast every year; the
  eve of Genna is within Tsome Nebiyat.
- *The Coptic Nativity in a leap year.* The Metropolis's calendar keeps
  the Nativity on 7 January and ends the fast on 6 January every year,
  so the fast is 42 days when 16 Hatour is 26 November; St-Takla's rite
  says the Nativity is 28 Koiak in those years. Both give the days
  carried. The `coptic-orthodox` feast table dates the Nativity 29 Koiak,
  8 January in those years.
- *The Coptic Paramoun before a Friday feast*, two days in Wikipedia's
  account, is in no primary source read, and is not carried.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [armenian-church-georgia-fasts] | Each Armenian weekly fast, its eve and its days, Great Lent of 48 days, the Christmas fast's eve on 29 December; the eves of 2020–2030 | Yes, 2026-09-29 |
| [armenian-church-sydney-fasts] | The Wednesdays and Fridays and their exemptions; the six-day Christmas fast; Elijah after Pentecost | Yes, 2026-09-29 |
| [armenian-mother-see-calendar-2026] | The Catechumens and Great Lent of 2026 | Yes, 2026-09-29 |
| [surb-zoravor-tonatsuyts-2026] | The eves of 2026 in the new and old calendars | Yes, 2026-09-29 |
| [surb-zoravor-oraphaq] | No weekly fasts in the tabernacle feasts | Yes, 2026-09-29 |
| [westernprelacy-fasts] | The forty days and the Christmas fast, in the Catholicosate of Cilicia's Western Prelacy | Yes, 2026-09-29; its dates carry no year |
| [arak29-fasts] | The fifty days, 160 fast days a year | Yes, 2026-09-29 |
| [armenian-church-sydney] | The Gregorian and Julian calendars of Etchmiadzin and Jerusalem | Yes, 2026-09-26 |
| [st-takla-fasts] | Every Coptic fast's length and days, the Wednesdays and Fridays | Yes, 2026-09-29 (Arabic) |
| [st-takla-nativity-fast] | The Nativity on 7 January, 28 Koiak in a leap year | Yes, 2026-09-29 (Arabic) |
| [st-takla-paramoun] | The Paramoun's days | Yes, 2026-09-29 (Arabic) |
| [copticchurch-net-fasts] | The Apostles' Fast to 5 Epip; the 43 days of the Nativity Fast | Yes, 2026-09-29 |
| [suscopts-fasts] | The Coptic fasts of 2023–2028 | Yes, 2026-09-29, the rows the page's own GraphQL endpoint returns |
| [ethiopianorthodox-org-calendar] | The Ethiopian fasts, Tsome Nebiyat's days, the Gahad, the Wednesdays and Fridays | Yes, 2026-09-29 |
| [eotcmk-fasts-2011] | The Bahire Hasab of the fasts: their offsets and the dates of 2011 E.C. | Yes, 2026-09-29 (Amharic) |
| [eotcmk-hawaryat-2023], [eotcmk-filseta-2016] | Tsome Hawaryat to 5 Hamle; Tsome Filseta of 1–15 Nehase | Yes, 2026-09-29 |
| [eotcmk-great-lent-2016], [eotcmk-great-lent-2017], [eotcmk-nineveh-2017], [eotcmk-nebiyat-2022] | Dated fasts | Yes, 2026-09-29 |
| [eotc-ma-fasts] | Tsome Hawaryat from the day after Paraclete; the Gahad as a substitute fast | Yes, 2026-09-29 |

The research was done through web pages only; the Western Prelacy's
daily readings, the Eastern Prelacy's calendars, Mahibere Kidusan's "The
Orders of the Fast" and the Claremont Coptic Encyclopedia's entries on
the fasts are PDFs or scripted pages and were not read.

## Code

`crates/hc-holiday/src/oriental_fasts.rs`, the periods, and
`crates/hc-holiday/src/orthodox_fasts.rs`, the reckonings and the scheme's
`status` and `span`. Anchors: `the_armenian_fasts_follow_the_eves_the_diocese_prints`,
`the_jerusalem_reckoning_is_the_old_calendar_column`,
`the_armenian_weekly_fasts_are_lifted_after_easter_and_after_theophany`,
`the_coptic_fasts_are_the_metropolis_calendar_of_2023_to_2028`,
`the_coptic_paramoun_runs_back_from_a_sunday_or_monday_theophany`,
`the_ethiopian_fasts_are_the_dates_mahibere_kidusan_prints`. The
WebAssembly and C exports `hc_orthodox_fast_on` and
`hc_orthodox_fast_seasons` take the four identifiers as they take the
Eastern Orthodox two.
