# The Hindu festival days: the part of the day a tithi must hold, the two Deepavalis and the central government's lists

Backs `hc-holiday`'s `hindu` module (`hc_holiday::hindu`: the rules `UGADI`,
`RAMA_NAVAMI`, `MAHAVIR_JAYANTI`, `AKSHAYA_TRITIYA`, `BUDDHA_PURNIMA`,
`GURU_PURNIMA`, `RAKSHA_BANDHAN`, `JANMASHTAMI`, `GANESH_CHATURTHI`,
`NAVARATRI`, `DURGA_ASHTAMI`, `VIJAYA_DASHAMI`, `DIWALI`,
`NARAKA_CHATURDASHI`, `GURU_NANAK_JAYANTI`, `MAHA_SHIVARATRI`,
`HOLIKA_DAHAN`, `HOLI`, `MAKAR_SANKRANTI`, `MESHA_SANKRANTI`, and the
Sikh, Jain and Tamil ones `GURU_GOBIND_SINGH_PARKASH`, `HOLA_MOHALLA`,
`SAMVATSARI`, `ANANT_CHATURDASHI` and `THAIPUSAM`), the rule kinds `Rule::Tithi`
and `Rule::TithiAfterBhadra` and `WhenTwice` of `hc-holiday`'s `rule` module,
`hc-calendars-indic`'s `tithi::Prevalence` and `vaishnava`, the table `hindu`
(`traditions::HINDU`), and the Hindu days of the tables `IN`, `SG`, `MY`, `LK`,
`TT`, `GY`, `SR`, `MU`, `MM` and `KE`. The rules reach a caller through the
WebAssembly and C exports `hc_holidays_in_year`, `hc_holidays_on` and
`hc_holidays_on_in`, by a table's code. No calendar identifier is registered:
the days are dated in `hindu-lunar`, which
[hindu-calendars.md](hindu-calendars.md) covers, and Nepal's and
Bangladesh's own lists of them are [nepal-holidays.md](nepal-holidays.md)'s
and [bangladesh-holidays.md](bangladesh-holidays.md)'s.

## What it is

**A festival is a tithi, kept on a part of a day.** A tithi, a thirtieth
of the lunar month, lasts from about 0.9 to 1.1 civil days, so it seldom
begins and ends with a day. A calendar gives a civil day the tithi in
progress at its sunrise, and most Hindu festivals are not kept on that
day. Each rite belongs to a part of the day: the noon, the afternoon, the
hour after sunset, the midnight, the dawn before sunrise. The festival is
the day on which its tithi holds that part. Krishna Janmashtami of 2025
is the eighth tithi of the dark half of Śrāvaṇa, kept on Friday 15 August,
whose sunrise carried the seventh: the eighth began at 23:50 that night
and held its midnight.

**The Calendar Reform Committee's list.** The Committee appointed by the
Council of Scientific and Industrial Research in 1952 printed in its
report of 1955 "General Rules for Religious Festivals", with a list of the
lunar festivals by amānta month, tithi and part of the day, and the solar
festivals by the Sun's entry into a sign [crc1955]. It says it followed
the *dharmaśāstra*, naming the *Nirṇayasindhu* and *Dharmasindhu*
among others, and that the rules differ so much from state to state and
sect to sect that a common rule for all India is difficult to formulate;
where it could, it laid one down for uniformity. Its general rule is that a
rite is for the noon or the forenoon unless the list says another time; if
the tithi holds the time on two successive days the festival is on the first
day where the list marks it *pūrvaviddha* and on the second where it marks
it *paraviddha*, the tithi meeting the one before it or the one after it
[crc1955, pp. 101--102 and the explanation of terms, p. 108]. The
module documentation of `hc_holiday::hindu` says the *Rashtriya Panchang*'s
"Principal Festivals and Anniversaries" follows these conventions, and its
rules reproduce that list [rashtriya-panchang-1945,
rashtriya-panchang-1946]; the almanac was not read again here.

**The two Deepavalis.** Dīpāvalī is five days around the new moon of
Āśvina, and two of them are called Deepavali. In most of India the festival
is Lakṣmī Pūjā, the evening of the new moon (*amāvāsyā*), the thirtieth
tithi. In Tamil Nadu it is Naraka Caturdaśī, the fourteenth tithi of the
dark half, marked by a bath before sunrise [drik-tamil-deepavali,
drik-naraka-chaturdashi-info, wikipedia-naraka-chaturdashi]. By the rules
below the two are one day in 45 of the 76 years 1995 to 2070 and consecutive
days, the Tamil one first, in the other 31. A government's holiday is one or
the other, and which one is the question the tables below answer for each
government.

**The central government's lists.** The Department of Personnel and
Training, DoPT, publishes each year the holidays of the central
government's offices in Delhi and New Delhi: a closed list, Annexure I,
and a restricted list, Annexure II, from which an employee may take two
days. Both name the Hindu days with their Śaka dates. The closed list's
Janmashtami is, in every year read, the day of a Vaiṣṇava reading, which
takes the first sunrise at or after the eighth tithi. The lists name it so
in several years, spelling it Vaishnavi, Vaishnava, Vaishnva and Vaishnav,
and the restricted list of 2022 has the Smārta day beside it as
"Janmashtami (Smarta)" [dopt-holidays-2017-2018-2027,
dopt-holidays-2019-2024, dopt-holidays-2026-pages].

## How it works

**The month and the days that can qualify.** A `Rule::Tithi` names an amānta
month, 1 for Chaitra through 12 for Phālguna, and a tithi, 1 through 30,
the dark half's tithis numbered on from 16:
Janmashtami is month 5, tithi 23, and
Dīpāvalī month 7, tithi 30. A Gregorian year lies in two Śaka years, and in
each the engine takes the ordinary month of that name, never the repeated
one. The month is the one that holds the Sun's entry into the sign of the
same number, Meṣa for Chaitra, as [hindu-calendars.md](hindu-calendars.md)
places it, and its first day is the first whose sunrise follows the new
moon. The engine reads the tithi at the rule's part of a day on each day
from three before to three after the day numbered `tithi` from the month's
first day, and on the day before the first, since a first tithi can begin
the afternoon before. A month the year does not have, a kṣaya one, has no
festival (Accuracy, below).

**The part of the day.** `Prevalence` is the instant a tithi is read at, on
a day at the calendar's place, which is the Central Station of the national
calendar, 23°11′ N 82°30′ E, so that its local mean time is Indian Standard
Time [crc1955, p. 4]. Sunrise and sunset are the upper limb of the Sun at a
sea-level horizon. The instants are the library's choice of one point in a
part of the day that is an interval:

| `Prevalence` | The instant probed | The Committee's part of the day | Festivals |
| --- | --- | --- | --- |
| `Dawn` | 96 minutes before sunrise | *aruṇodaya*, two *muhūrta*s, about 4 *ghaṭikā*s or 1 h 36 min, before sunrise; Naraka Caturdaśī "covering a period of 4 ghatikas before sunrise" | Naraka Caturdaśī |
| `Sunrise` | sunrise | the tithi of the civil day | Ugādi, Mahāvīra Jayantī, Navarātri, Durgā Aṣṭamī |
| `Midday` | halfway from sunrise to sunset | *madhyāhna*, from one *ghaṭikā* (24 min) before midday to as long after; or the 6th to 10th, or 7th to 9th, of the fifteen *muhūrta*s of the daytime | Rāma Navamī, Akṣaya Tṛtīyā, Buddha Pūrṇimā, Guru Pūrṇimā, Gaṇeśa Caturthī, Guru Nānak Jayantī |
| `Afternoon` | seven tenths of the way from sunrise to sunset | *aparāhṇa*, the last third of the daytime, or the 10th to 12th *muhūrta*s | Vijayā Daśamī |
| `Evening` | one hour after sunset | *pradoṣa*, two *muhūrta*s, about 1 h 36 min, after sunset, in some opinion three | Dīpāvalī, Holikā Dahana |
| `Midnight` | halfway from sunset to the next sunrise | *niśīthā*, two *ghaṭikā*s covering midnight | Janmāṣṭamī, Mahā Śivarātri |

A *muhūrta* is a fifteenth of the daytime, about 48 minutes [crc1955,
p. 108]. The Committee's parts are intervals, and a tithi can hold some of
one: the rule asks only whether the tithi is in progress at the instant, and
the Committee's and the dharmaśāstra's rules for a tithi that holds only part
of the interval are not modelled. The length of *pradoṣa* is disputed, two
*muhūrta*s by the Committee and, by Wikipedia, an hour and a half each side
of sunset [crc1955, p. 108; wikipedia-pradosha]; the probe an hour after
sunset is inside both. The Committee's definitions are what this document
has: the *Nirṇayasindhu* and *Dharmasindhu* it names, and P. V. Kane's
survey of the parts of the day, were not read [kane1958]. The night a day's
midnight belongs to is the one after its sunset, so a day's evening and its
midnight fall on the same night.

**Which day, when two qualify.** `WhenTwice::Earlier` takes the first day on
which the tithi holds the part, the *pūrvaviddha* reading, and
`WhenTwice::Later` the second, the *paraviddha* one. When the tithi holds
the part on no day, the rule takes the day that carries the tithi at
sunrise or, for a tithi no sunrise carries, the day it begins and ends
within. The Committee's Janmāṣṭamī rule sends a tithi that holds no midnight
to the second day, which this fallback does not do, and Janmāṣṭamī used the
fallback in none of the 76 years 1995 to 2070 measured below.

**The rules.** Each rule below is a `Rule::Tithi` unless it says otherwise.
The last column is what the Committee's list says of the same festival
[crc1955, pp. 102--105]; where the rule parts from it the Accuracy section
says how often.

| Rule | Month, tithi | Part, and which day if two | The Committee's list |
| --- | --- | --- | --- |
| `UGADI` | Chaitra śukla 1 | sunrise, earlier | Chaitra śukla 1 is Navarātrārambha, *paraviddha*; Ugādi is not named |
| `RAMA_NAVAMI` | Chaitra śukla 9 | midday, earlier | *madhyāhna-vyāpinī*; no marking |
| `MAHAVIR_JAYANTI` | Chaitra śukla 13 | sunrise, earlier | a Jain day beside Ananga Trayodaśī (*pūrvaviddha*); no part of the day for it |
| `AKSHAYA_TRITIYA` | Vaiśākha śukla 3 | midday, earlier | *pūrvāhṇa-vyāpinī*, *paraviddha* |
| `BUDDHA_PURNIMA` | Vaiśākha śukla 15 | midday, earlier | named, no marking |
| `GURU_PURNIMA` | Āṣāḍha śukla 15 | midday, earlier | *paraviddha* |
| `RAKSHA_BANDHAN` (`TithiAfterBhadra`) | Śrāvaṇa śukla 15 | the half of the tithi that Bhadra does not cover | "in the second half of purnima" |
| `JANMASHTAMI` | Śrāvaṇa kṛṣṇa 8 | midnight, later | *madhyarātra-vyāpinī*; the second day if midnight is covered on two days or none; for Vaiṣṇavas the day after the Saptamī day |
| `GANESH_CHATURTHI` | Bhādrapada śukla 4 | midday, earlier | *madhyāhna-vyāpinī* and *pūrvaviddha* |
| `NAVARATRI` | Āśvina śukla 1 | sunrise, earlier | Navarātrārambha, *paraviddha* |
| `DURGA_ASHTAMI` | Āśvina śukla 8 | sunrise, earlier | Mahāṣṭamī, *paraviddha* |
| `VIJAYA_DASHAMI` | Āśvina śukla 10 | afternoon, earlier | no part of the day; the day it touches Śravaṇa nakṣatra by daylight, elsewhere than Bengal |
| `NARAKA_CHATURDASHI` | Āśvina kṛṣṇa 14 | dawn, earlier | 4 *ghaṭikā*s before sunrise, the first day if two |
| `DIWALI` | Āśvina kṛṣṇa 15, the thirtieth tithi | evening, later | Dīpāvalī and Mahālakṣmī Pūjā *pradoṣa-vyāpinī*; Kālī Pūjā *niśīthā-vyāpinī* |
| `GURU_NANAK_JAYANTI` | Kārtika śukla 15 | midday, earlier | not listed |
| `MAHA_SHIVARATRI` | Māgha kṛṣṇa 14 | midnight, earlier | *niśīthā-vyāpinī*; if two, Hemādri the first day and Mādhava the second |
| `HOLIKA_DAHAN` | Phālguna śukla 15 | evening, earlier | *sāyāhna-vyāpinī*, the second half of the tithi at night |
| `HOLI` (`Offset`) | the day after Holikā Dahana | — | "on the day after Holikadahana" |
| `MAKAR_SANKRANTI`, `MESHA_SANKRANTI` (`Sankranti`) | the Sun's entry into Makara and Meṣa | the civil day at the Indian meridian, Lahiri ayanāṃśa | the solar festivals by the Sun's transit, the day taken from midnight |

The Sikh and Jain rules of the same module are sunrise tithis:
`GURU_GOBIND_SINGH_PARKASH` (Pauṣa śukla 7), `HOLA_MOHALLA` (Phālguna
kṛṣṇa 1), `SAMVATSARI` (Bhādrapada śukla 4) and `ANANT_CHATURDASHI`
(Bhādrapada śukla 14), and `THAIPUSAM` is a `Rule::Nakshatra`, Puṣya in the
month of Thai at the Indian meridian. Their sources are the Sikh and Jain
tables'; they are in the module because the Hindu table's rules are.

**Rakṣā Bandhana and Holikā Dahana, kept out of Bhadra.** Bhadra, the
karaṇa Viṣṭi, covers the first half of the full-moon tithi, and the rite
waits for the second half [drik-raksha-bandhan]. `Rule::TithiAfterBhadra`
takes the civil day on which that half begins, or the next day if the tithi
still holds six *ghaṭikā*s, 2 hours 24 minutes, after the next sunrise, the
six from a jyotiṣī's page [onlinejyotish-rakhi-2024]. The karaṇas and the
rule's measured agreement are in [hindu-calendars.md](hindu-calendars.md).
Holikā Dahana is the other day the almanacs keep out of Bhadra, and
`HOLIKA_DAHAN` does not: it takes the evening of the full-moon tithi
[drik-holika-dahan].

**The two Deepavalis, by rule.** `DIWALI` is Lakṣmī Pūjā: the new-moon tithi
at the probe an hour after sunset, the later day if two. `NARAKA_CHATURDASHI`
is the fourteenth tithi at the dawn probe, the earlier day if two. When the
fourteenth tithi holds the dawn and the new moon holds the next evening, the
days are a day apart; when the new moon begins between the dawn and the
evening, as in 2019 to 2026, one day holds both. In 1995 to 2070 the two
rules give days a day apart in 31 of the 76 years, and never more.

**Smārta and Vaiṣṇava Janmāṣṭamī.** `JANMASHTAMI` is the Smārta day: the
night on which the eighth tithi holds midnight. The Vaiṣṇava day is the
first sunrise at or after the eighth tithi, which `vaishnava::janmashtami`
computes and `hc-holiday` does not register
[hindu-calendars.md](hindu-calendars.md). That function was fitted to Drik
Panchang's ISKCON days for Tokyo, 2024 to 2034, which it gives in all eleven
years [drik-iskcon-janmashtami]. At the Central Station the Smārta and the
Vaiṣṇava day are one in 2021, 2024 and 2026 and a day apart in the other
eight years of 2017 to 2027.

**Worked example.** Krishna Janmashtami 2021, a tithi that holds the midnight
of two nights. The rule is month 5, tithi 23, at midnight, the later day.
Times are Indian Standard Time at the Central Station.

1. The eighth tithi begins when the Moon is 22 × 12° = 264° ahead of the
   Sun and ends at 276°. The library puts it from 23:25:39 on 29 August to
   02:00:03 on 31 August, 26 hours 34 minutes. New Delhi's Drik Panchang
   prints 11:25 PM and 01:59 AM [drik-krishna-janmashtami-delhi].
2. The midnight probe of a day is halfway from its sunset to the next
   sunrise. For 29 August, sunset is 18:20:22 and the sunrise after is
   05:41:28, so the probe is 00:00:55 on 30 August; the tithi is the 23rd.
   For 30 August the probe is 00:00:37 on 31 August, and the 23rd still
   holds it, 1 hour 59 minutes before its end. The probe of 28 August falls
   on the 29th, before the tithi begins, and that of 31 August, 00:00:18 on
   1 September, after it ends, in the 24th.
3. Two days qualify, 29 and 30 August. `Later` takes the 30th, a Monday.
4. The page for New Delhi gives Monday 30 August, with Nishita from 11:59 PM
   to 12:44 AM [drik-krishna-janmashtami-delhi]; the DoPT list for 2021 has
   Janmashtami on Monday 30 August [dopt-holidays-2019-2024]. The Committee's
   rule is the same: the second day when midnight is covered on two days.
   `Earlier` would have been Sunday 29 August. Here the Smārta and the
   Vaiṣṇava day are one, since the eighth tithi holds the sunrise of the
   30th.

**Worked example: when the readings part, 2025.** The eighth tithi runs
from 23:50:20 on 15 August to 21:35:03 on 16 August (the page prints 11:49 PM
and 09:34 PM). The midnight probe of the 15th is 00:04:32 on the 16th, inside
the tithi; that of the 16th is 00:04:19 on the 17th, in the ninth. Only the
15th qualifies, though the sunrise of the 15th, 05:36:13, still carries the
seventh. The first sunrise at or after the eighth tithi's beginning is the
16th, at 05:36:35. Drik Panchang for New Delhi gives Friday 15 August as the
Smārta day and Saturday 16 August for ISKCON [drik-krishna-janmashtami-delhi];
the DoPT list for 2025 has Saturday 16 August [dopt-holidays-2025-2027];
`hindu` gives the 15th and `vaishnava::janmashtami` the 16th.

**Worked example: the two Deepavalis, 2018.** The fourteenth tithi runs
from 23:47:06 on 5 November to 22:27:21 on 6 November, and the new-moon tithi
from then to 21:32:07 on the 7th (the pages print 11:47 PM, 10:27 PM and
9:32 PM) [drik-abhyang-snan-2018, drik-lakshmi-puja]. The dawn probe of the
6th, 04:31:50, is in the fourteenth tithi, and that of the 5th, 04:31:15, in
the thirteenth: Naraka Caturdaśī is Tuesday 6 November. The evening probe of
the 6th, 18:19:08, is in the fourteenth, and that of the 7th, 18:18:38, in the
new-moon tithi: Lakṣmī Pūjā is Wednesday 7 November. Drik Panchang gives
Tamil Deepavali on Tuesday 6 November and "In North India" Wednesday 7
November [drik-tamil-deepavali].

## What is carried

**The `hindu` table.** Twenty rules, in this order: Makara Saṅkrānti,
Mahā Śivarātri, Holikā Dahana, Holī, Ugādi, Rāma Navamī, Mahāvīra Jayantī,
Meṣa Saṅkrānti, Akṣaya Tṛtīyā, Buddha Pūrṇimā, Guru Pūrṇimā, Rakṣā
Bandhana, Kṛṣṇa Janmāṣṭamī, Gaṇeśa Caturthī, Navarātri, Durgā Aṣṭamī,
Vijayā Daśamī, Naraka Caturdaśī, Dīpāvalī and Guru Nānak Jayantī, each of
`Kind::Religious` and `Confidence::Exact`, in the Gregorian years 1701 to
2298. The calendar the tithis are read on converts the Śaka years 1622 to
2221, that is 1700 to 2299, and the first and last year, whose months can
lie partly outside, are left out: 1700, 2299 and every year beyond are gaps,
all twenty reported, not days. The ayanāṃśa and the sunrise are the
national calendar's, so a regional almanac that follows another place or
another ayanāṃśa can build the same rules with its own `HinduLunarCalendar`.
Every day of the table is computed. The days tabulated are in the tables of
governments that publish them: India's closed list for 2025 to 2027, Sri
Lanka's orders for 2023 to 2027, Myanmar's notices for 2020 to 2025, Fiji's
lists for 2019 to 2026, and Bangladesh's and Nepal's notices.

**Which table keeps which Deepavali.** Where two governments' lists of a
year part, a table follows the lists that were read.

| Table | Rule | The lists read |
| --- | --- | --- |
| `SG` | Naraka Caturdaśī | The Ministry of Manpower: 18 October 2017 and 6 November 2018 [mom-public-holidays-2017-2018], 8 November 2026 and 28 October 2027 [mom-public-holidays-page, mom-public-holidays-consolidated] |
| `MY` | Naraka Caturdaśī | The federal schedules, 28 October 2027 among them [bkpp-hari-kelepasan-am, publicholidays-my-deepavali] |
| `LK` | tabulated | The Holidays Act orders for 2023 to 2027, which the Tamil rule gives in every year |
| `GY` | Naraka Caturdaśī | The Ministry of Public Security: 18 October 2017, listed as tentative, and 6 November 2018 [mops-national-holidays-2017-2018] |
| `TT` | Naraka Caturdaśī | The President's appointment of 18 October 2017 [newsday-divali-2017] and Legal Notice No. 135 of 2018 for 6 November, known by its title only [tt-legal-notice-135-2018] |
| `IN` | Lakṣmī Pūjā | DoPT: 19 October 2017, 7 November 2018 and 29 October 2027 [dopt-holidays-2017-2018-2027], each list letting a state that keeps Naraka Caturdaśī alone close central offices on that day instead |
| `MU` | Lakṣmī Pūjā | The Prime Minister's Office: 19 October 2017 and 7 November 2018 [pmo-mu-public-holidays-2017-2018] |
| `SR` | Lakṣmī Pūjā | The Minister of Home Affairs: 19 October 2017 and 7 November 2018 [waterkant-divali-2017-2018] |
| `MM`, `KE` | Lakṣmī Pūjā | None found for a year the two rules part in: Myanmar's notices of 2020 to 2025 are listed, and agree with either rule but 2024's, which neither gives; Kenya's Act dates Diwali "depending upon the appearance of the moon" |

Where no list of a parting year was found the table keeps Lakṣmī Pūjā, the
day of India's national list. `MU` also keeps Mahā Śivarātri, Ugādi and Gaṇeśa
Caturthī on the rules, approximate, and Cavadee on the Pusam rule of
`THAIPUSAM` at the island's own meridian; `GY` and `SR` keep Phagwah on
`HOLI`. Fiji's Diwali is tabulated from its Ministry's lists
for 2019 to 2026 and is not a rule: it is a day after both rules in six of
those eight years, two days after in 2020 and the same day in 2021, and why
was not looked into. Nepal's festivals are `Rule::Tithi` read at Kathmandu
with parts of the day fitted to its notices, not these rules
[nepal-holidays.md](nepal-holidays.md), and Bangladesh's Janmashtami is its
notifications' day, 16 August 2025 and 4 September 2026.

**India.** The table `IN` carries the DoPT's closed list as it was read, for
2025 to 2027, a listing with the days of those lists: Holi, Rāma Navamī,
Mahāvīra Jayantī, Buddha Pūrṇimā, Janmāṣṭamī, Dussehra, Dīpāvalī and Guru
Nānak's Birthday, and three of Delhi's optional days. After 2027 each is the
`hindu` rule, approximate, and before 2025 each is a gap. Its Diwali is Lakṣmī
Pūjā. Its Janmashtami in 2025 is 16 August and in 2027 25 August, the Vaiṣṇava
day, and after 2027 the Smārta rule's. The states' own Janmashtami days are
in [india-state-holidays.md](india-state-holidays.md): in 2022, 2023 and
2025 the table built from the Reserve Bank's lists gives both days to
different states, which was not read again here.

**Not carried:**

- The Vaiṣṇava Janmāṣṭamī as a registered rule (not yet done:
  `vaishnava::janmashtami` computes it, and policy §5 asks for an identifier
  of its own for each convention). Eleven DoPT lists agree with it, as
  Accuracy measures, and no source read states the rule.
- A rule for the Holī of the lists (no source read): the DoPT keeps Holī a
  day after `HOLI` in 2023, 2026 and 2027, and `HOLA_MOHALLA` gives those
  three days and not those of 2022, 2024 and 2025.
- The Śravaṇa nakṣatra's part in Vijayā Daśamī (not yet done): the Committee
  names it, and the page for New Delhi in 2019 follows it (Accuracy).
- The Committee's rules for a tithi that holds only part of the interval of a
  part of the day, and its regional variants, such as Bengal's (not yet done:
  no almanac read gives a case to hold such a rule to).
- The Vaiṣṇava Ekadashi and the other tithis' Vaiṣṇava days (GCal's rules were
  not read; [hindu-calendars.md](hindu-calendars.md) says so).
- The rest of the Committee's list of lunar festivals, among them the
  Ekādaśīs, Nṛsiṃha Caturdaśī and Rāsayātrā (not yet done): the table carries
  eighteen lunar days and two solar ones.
- A gap for a kṣaya month (a defect of the code, below).

## Accuracy

Measured on 2026-10-03 with a scratch program that links the worktree's
crates and evaluates the rules, and with the tests named under Code. A tithi
begins and ends at one instant for the whole Earth; only the day that
carries it depends on the place.

**The Rashtriya Panchang's list.** The test
`the_hindu_festivals_fall_where_the_rashtriya_panchang_lists_them` holds 32
days of the lists for Śaka 1945 and 1946, from 22 March 2023 to 10 April
2025, and the rules give all of them. The list itself was not read again
here. Of the 20 rules, Guru Pūrṇimā,
Navarātri and Durgā Aṣṭamī have no day in that test; Drik Panchang's pages
for New Delhi give 10 July, 22 September and 30 September 2025, which the
rules give [drik-festival-pages-2025].

**Drik Panchang, 27 tithi times.** Against the beginning and end of the tithi
on its pages for New Delhi and Chennai in 2014, 2018, 2019, 2021, 2025,
2026 and 2027 (Janmāṣṭamī, the two Deepavalis, Vijayā Daśamī, Rāma Navamī,
Śivarātri, Gaṇeśa Caturthī, Guru Pūrṇimā, Akṣaya Tṛtīyā, Rakṣā Bandhana and
Holikā Dahana), the library's moment is from 15 seconds before the printed
minute to 1 minute 37 seconds after it. The days agree in every case but
two:

- Vijayā Daśamī 2019. The tenth tithi runs from 12:38 on 7 October to
  14:50 on 8 October, so the Afternoon probe, 14:09 on the 7th and 14:09 on
  the 8th, is inside it on both days, and `Earlier` gives Monday 7 October.
  Drik Panchang for New Delhi gives Tuesday 8 October, with Śravaṇa nakṣatra
  beginning at 17:26 on the 7th, after that afternoon, and the DoPT list
  gives the 8th [drik-vijayadashami-2019, dopt-holidays-2019-2024]. The
  Committee names the Śravaṇa nakṣatra as the test where the tithi touches
  it. In 2026 the DoPT list's 20 October is `Earlier`'s, and `Later` would
  give the 21st. Neither choice fits both years, and whether Śravaṇa decides
  2026 too was not checked.
- Holikā Dahana 2026, which the page gives on 3 March and the rule on the
  2nd. The module documentation has the reason, and the pages for 2016 and
  2023 part from the rule in the same way [drik-holika-dahan].

Of the pages' choices of day, Janmāṣṭamī of 2021 and 2025 are in the worked
examples, and Rakṣā Bandhana's 75 of 76 years of 1995 to 2070 are in
`raksha_bandhan_is_the_day_drik_panchang_gives_for_seventy_five_years`: the
76th, 2036, parts from the page by the place.

**The two Deepavalis.** Drik Panchang's pages for Tamil Deepavali at Chennai
for 2013 to 2019, 2022 and 2027 give the Tamil day and the North Indian one
for nine years, 18 days [drik-tamil-deepavali]: `NARAKA_CHATURDASHI` gives
the first and `DIWALI` the second in every one, with the days a day apart in
2013, 2014, 2015, 2016, 2017, 2018 and 2027 and the same in 2019 and 2022.
The two rules give days a day apart in 31 of the 76 years 1995 to 2070,
and never two days or more; the years of 2019 to 2026 are all one day,
which `the_two_deepavalis_part_where_the_lists_say_they_do` holds for 2017
to 2027. The dawn probe is the start of the dawn and not the whole of it: in
8 of the 76 years the fourteenth tithi has ended by the sunrise of the
day chosen, 2021 among them, whose 4 November is also the day of Singapore's
and Malaysia's lists [mom-public-holidays-consolidated,
bkpp-hari-kelepasan-am], and in three years, 2031, 2054 and 2063, no dawn
holds the fourteenth and the fallback chooses.

**The central government's lists, 2017 to 2027.** Of 82 days in the closed
lists read, for Holi, Rāma Navamī, Mahāvīra Jayantī, Buddha Pūrṇimā,
Janmashtami, Dussehra, Diwali and Guru Nānak's Birthday, the rules give
69, and the 13 that part are:

| Festival | Years | The lists | The rule |
| --- | --- | --- | --- |
| Janmashtami | 2017, 2018, 2019, 2020, 2022, 2023, 2025, 2027 | the day after | `JANMASHTAMI` |
| Holi | 2023, 2026, 2027 | the day after | `HOLI` |
| Guru Nānak's Birthday | 2027 | 14 November | 13 November |
| Dussehra | 2019 | 8 October | 7 October |

The first row is the Vaiṣṇava Janmāṣṭamī, and `vaishnava::janmashtami` at the
Central Station gives the lists' day in all eleven years, 2017 to 2027,
those of 2021, 2024 and 2026 among them, where it is also the Smārta day.
That is eleven lists and eleven days. The module documentation counts the
parting years of 2025 and 2027 and says they are no anchor to fit a rule
to; the lists part from `JANMASHTAMI` in eight of the eleven years. The
reading is still fitted to days, not quoted: the Committee says Vaiṣṇavas
keep the day after the Saptamī day, which is a different rule.

The restricted lists carry the Smārta days where they were read. Janmashtami
(Smarta) on 18 August 2022 is `JANMASHTAMI`'s, and so are Holikā Dahana 2022
and 2024, Rakṣā Bandhana and Gaṇeśa Caturthī 2022, Gaṇeśa Caturthī and Mahā
Śivarātri 2026, and Naraka Caturdaśī in 2017, 2018, 2022, 2024, 2026 and 2027
[dopt-holidays-2017-2018-2027, dopt-holidays-2019-2024,
dopt-holidays-2026-pages, dopt-holidays-2025-2027]. Holikā Dahana on 3 March
2026, against the rule's 2nd, is the one that parts.

Guru Nānak's Birthday on the sunrise tithi gives all eleven of the lists'
days where the midday gives ten: 2027's tithi is the fifteenth from 09:56 on
13 November to 08:56 on the 14th, so only the 13th holds midday and only the
14th holds sunrise. That is a fit, not a source.

The days of the lists were read through the fetch tool as StaffNews and
GConnect reproduce them, and the figures above rest on those readings, not on
dopt.gov.in, which did not connect.

**Where the two days qualify, and where no day does.** The earlier and the
later day give different days in 2 to 5 per cent of the years 1701 to 2298
for each rule, in 11 to 29 of the 598 years. Over 2017 to 2036 the choice
changes a day for Rāma Navamī (2032), Akṣaya Tṛtīyā (2020), Janmāṣṭamī
(2021), Gaṇeśa Caturthī (2033), Durgā Aṣṭamī (2027, 2034), Vijayā Daśamī
(2019, 2026) and Mahā Śivarātri (2018), and for none of the others, `DIWALI`
among them. Where the Committee marks *paraviddha* and the rule takes the
earlier day (Akṣaya Tṛtīyā, Guru Pūrṇimā, Navarātri, Mahāṣṭamī) the later
would change 67 festival-years of the 2,392 in 1701 to 2298 for those four.
In 1995 to 2070 the rule falls back to the day the tithi holds at sunrise,
or the day it begins and ends within, for Ugādi in 10 years, Navarātri in 9,
Dīpāvalī in 8, Guru Nānak's Birthday in 8, Buddha Pūrṇimā, Gaṇeśa Caturthī
and Durgā Aṣṭamī in 5 each, and in fewer for the rest.

**A kṣaya month.** The calendar has no Māgha in the Śaka year 1904, 1983, a
lunar month holding two saṅkrāntis, and `MAHA_SHIVARATRI` has no day in 1983
or in 2284: the table has 19 days in both years, reports no gap, and
`is_complete` is true. That is a day silently missing, where policy §4 has
the engine say what it cannot answer. No source read says where Mahā
Śivarātri is kept in such a year, and the cause in 2284 was not looked into.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [crc1955] | The general rules for religious festivals, the list of lunar festivals with the part of the day and the *viddha* mark of each, the definitions of the parts of the day, the Central Station | Yes, 2026-10-03, the Internet Archive's OCR text; pp. 101 to 108, the rules and the list; the report's own date tables were not compared |
| [rashtriya-panchang-1945], [rashtriya-panchang-1946] | The festival days the rules reproduce | No: read by the module's author; the Centre served no copy |
| [kane1958] | The standard survey of the parts of the day | No |
| [wikipedia-pradosha] | The window of *pradoṣa* an hour and a half either side of sunset | Yes, 2026-10-03 |
| [drik-krishna-janmashtami-delhi] | Janmāṣṭamī 2021 and 2025, the tithi's times, the Smārta and ISKCON days | Yes, 2026-10-03 |
| [drik-lakshmi-puja], [drik-abhyang-snan-2018] | The new-moon and fourteenth tithis of 2018 and the days | Yes, 2026-10-03 |
| [drik-tamil-deepavali], [drik-naraka-chaturdashi-info] | The Tamil and the North Indian day for nine years and the rule for Naraka Caturdaśī | Yes, 2026-10-03 |
| [drik-vijayadashami-2019], [drik-festival-pages-2025] | Vijayā Daśamī 2019 and the six pages of 2025: five festivals' days and tithis, and Śāradīya Navarātri | Yes, 2026-10-03 |
| [drik-raksha-bandhan], [drik-holika-dahan] | Bhadra and Rakṣā Bandhana, the rules for Holikā Dahana | Yes, 2026-10-03 |
| [onlinejyotish-rakhi-2024] | The six *ghaṭikā*s | Yes, 2026-10-03; secondary, no text named |
| [drik-iskcon-janmashtami] | The ISKCON days at Tokyo for `vaishnava` | Yes, 2026-10-03 |
| [dopt-holidays-2017-2018-2027] | The lists of 2017, 2018 and 2027, the Naraka Caturdaśī note | Yes, 2026-10-03, the three pages; secondary |
| [dopt-holidays-2019-2024], [dopt-holidays-2026-pages] | The lists of 2019 to 2024 and 2026 | Yes, 2026-10-03, secondary, through the fetch tool's summary; dopt.gov.in and documents.doptcirculars.nic.in did not connect |
| [dopt-holidays-2025-2027] | The list of 2025, the restricted days of 2026 and 2027 | The 2025 page only; the 2026 and 2027 PDFs were not opened |
| [mom-public-holidays-page] | Deepavali 8 November 2026 and 28 October 2027 | Yes, 2026-10-03 |
| [mom-public-holidays-2017-2018], [mom-public-holidays-consolidated] | Singapore's days of 2017, 2018 and 2020 to 2027 | No: the press releases are PDFs, and the dataset was not re-read |
| [bkpp-hari-kelepasan-am] | Malaysia's days | No: PDFs. [publicholidays-my-deepavali], a commercial calendar, was read for 28 October 2027 |
| [mops-national-holidays-2017-2018], [pmo-mu-public-holidays-2017-2018] | Guyana's and Mauritius's days | No: PDFs |
| [newsday-divali-2017], [tt-legal-notice-135-2018] | Trinidad and Tobago's days | No: the page refused the connection; the notice was never read |
| [waterkant-divali-2017-2018] | Suriname's day of 2018 | The 2018 report only, 2026-10-03; secondary |
| [wikipedia-naraka-chaturdashi] | Tamil Nadu's Deepavali on Naraka Caturdaśī | Yes, 2026-10-03; secondary |

The statements of the module documentation for which this document names no
source of its own are the Myanmar and Kenya rows of the table above, from
the notices and the Act named in `traditions.rs` and `africa_middle_east.rs`.

## Code

`crates/hc-holiday/src/hindu.rs`, the rules; `rule.rs`, `Rule::Tithi`,
`Rule::TithiAfterBhadra`, `WhenTwice`, `tithi_days`,
`tithi_after_bhadra_days`; `traditions.rs`, `HINDU`;
`crates/hc-calendars-indic/src/tithi.rs`, `Prevalence`, `sunrise_of`,
`sunset_of`, and `vaishnava.rs`. The tables' own days are in
`crates/hc-holiday/src/countries/asia.rs`, `americas.rs`,
`africa_middle_east.rs` and `oceania.rs`. Anchors, in
`crates/hc-holiday/tests/traditions.rs`:
`the_hindu_festivals_fall_where_the_rashtriya_panchang_lists_them`,
`naraka_chaturdashi_is_the_central_governments_day`,
`a_first_tithi_that_no_sunrise_carries_still_opens_the_year` and
`every_hindu_date_is_exact_and_religious`; in `countries.rs`:
`the_two_deepavalis_part_where_the_lists_say_they_do`,
`singapore_and_malaysia_keep_the_days_their_lists_date`,
`india_keeps_the_days_of_the_central_government_lists` and
`myanmar_keeps_the_notified_deepavali`; in `raksha_bandhan.rs`:
`raksha_bandhan_is_the_day_drik_panchang_gives_for_seventy_five_years`; and
in `hc-calendars-indic`'s `tithi.rs`
`the_prevalences_run_through_the_day_in_order`, and `vaishnava.rs`
`the_first_sunrise_at_or_after_ashtami_is_the_iskcon_day_in_all_eleven_years`.
The comparisons of this document with the lists of 2017 to 2027, with
Drik Panchang's pages and with the earlier and later day have no test of
their own; they were made on 2026-10-03 with a scratch program.
