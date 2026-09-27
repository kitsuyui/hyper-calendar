# Global and regional Hijri calendars: the Unified Hijri Calendar, the FCNA calendar and Neo-MABIMS

Backs the identifiers `islamic-khgt` and `islamic-istanbul-2016` in
`hc-calendars-lunar`'s `islamic_global`, `islamic-fcna` in `islamic_fcna`,
and the criteria `istanbul-2016`, `khgt`, `mabims-2021-topocentric`,
`mabims-2021-geocentric-elongation` and `odeh` of `islamic_observational`'s
`NamedCriterion::ALL`. The Hijri calendar in general, the tabular schemes,
the Umm al-Qura table and the single-place prediction are in
[hijri.md](hijri.md).

## What it is

Three bodies publish Hijri calendars computed in advance by a rule that
treats the whole Earth, or a region, as one place of sighting, so that a
month begins on the same civil day for everyone who follows them.

- **The Unified Hijri Calendar** of the congress Diyanet convened in
  Istanbul on 28–30 May 2016, which adopted a single calendar for the whole
  world based on the possibility of sighting the crescent anywhere on Earth
  [diyanet-kongre-2016]. Two bodies apply it and publish its dates:
  - **Türkiye's Presidency of Religious Affairs (Diyanet)**, whose lists of
    religious days give the first day of each month [diyanet-dini-gunler];
  - **Muhammadiyah**, the Indonesian organisation, as its *Kalender
    Hijriah Global Tunggal* (KHGT), in force from 1 Muḥarram 1447, 26 June
    2025 [muhammadiyah-ughc-2025], published year by year to 1492 AH
    [khgt-kalendar-hijriah].
- **The Fiqh Council of North America (FCNA)**, which publishes the first
  day of every month from Muḥarram 1440 to Dhū al-Ḥijja 1467, 2018–2045,
  computed by the criterion of the European Council for Fatwa and Research
  [fcna-calendar].
- **The Neo-MABIMS criterion** of the religious-affairs ministers of Brunei,
  Indonesia, Malaysia and Singapore, ratified on 8 December 2021 and applied
  in Malaysia from Muḥarram 1443 and in Indonesia from 2022
  [djamaluddin-mabims-2022, mufid-djamaluddin-2023]. It is a regional
  criterion of possible sighting, and the member states still fix the
  month at a sighting session; it is carried as a criterion and not as a
  calendar.

In Ramaḍān 1447 the two Unified Hijri Calendars began the month a day
apart: Muhammadiyah on Wednesday 18 February 2026 [muhammadiyah-ramadan-1447]
and Diyanet on Thursday 19 February [diyanet-ramazan-1447]. FCNA also began
it on the 18th [fcna-ramadan-1447].

## How it works

### The Unified Hijri Calendar

The two bodies state the same two thresholds, and the rest differently.

**Muhammadiyah** [muhammadiyah-ughc-2025, §C.3; muhammadiyah-khgt-site]:

1. The whole world is one place of sighting; the new month begins on the
   same day everywhere, the day counted from the International Date Line.
2. The month begins on the next day if, **before 24:00 UT**, somewhere on
   Earth at sunset the Moon's elongation is at least 8° and its altitude
   at least 5°, both **geocentric** ("elongasi dan ketinggian hilal
   geosentris").
3. If the thresholds are met only after 24:00 UT, the month still begins
   if they are met somewhere with the conjunction before dawn in New
   Zealand, and if they are met on the mainland of the Americas.

**Diyanet** [diyanet-ramazan-1447]:

1. No place of sighting is privileged: "Yeryüzünün herhangi bir
   bölgesinde hilalin ru'yeti mümkün olursa buna dayanılarak ayın
   başladığına hükmedilir", if the crescent can be sighted in any region
   of the Earth, the month is held to have begun. Its assessment of time
   zones says it again: "Dünyanın herhangi bir yerinde", anywhere in the
   world, together with the conjunction before *imsāk* in the easternmost
   region, and states that the computations are made in UTC.
2. The crescent is visible when, after the conjunction, the Moon is at
   least 8° from the Sun, and at sunset at least 5° above the horizon.
3. For the month to begin, the sighting must be possible on the mainland
   of South or North America, and the conjunction must fall before
   *imsāk* at Wellington.

Diyanet's statement gives no frame for the two angles. T. Djamaluddin, of
Indonesia's government hisab-rukyat team, gives the Turkish criterion as a
geocentric elongation and a **topocentric** altitude, and shows that
Diyanet's map of the crescent's visibility for Ramaḍān 1447 is drawn with
the topocentric altitude and Muhammadiyah's with the geocentric one
[djamaluddin-khgt-turki-2025]. The topocentric altitude is the geocentric
one less the Moon's parallax in altitude, about 0.95° at the horizon.

The two texts also differ in shape. Muhammadiyah bounds its "anywhere"
condition at 24:00 UT; Diyanet's statement gives it no hour, and lists the
Americas and Wellington conditions as required for the month to begin. Read
literally, the Americas condition would make "anywhere" idle. This library
reads Diyanet's condition with Muhammadiyah's bound: the statement computes
in UTC, and in Ramaḍān 1447 it set aside a crescent visible over the
Pacific from 03:42 UT on the 18th, after midnight and not on land. The
bound is an inference, not the statement's words. Against Diyanet's lists
it dates 172 of 174 months as published. Requiring the Americas and
Wellington of every month dates 171, with Rajab 1453 a day late. That
evening, 16 October 2031, both thresholds are met before 24:00 UT over the
south-eastern Pacific, west of South America, and not on the mainland as
the library follows it. Muhammadiyah's site, summarising the Turkish
model, also joins the two after-midnight conditions with "atau", or, where
its parameters say "and" [muhammadiyah-khgt-site]. `docs/policy.md` §5
therefore gives each body's calendar its own identifier.

**This library's reading** of both:

- **Sunset** is the Sun's upper limb on the sea-level horizon, its centre
  50′ below the geometric horizon.
- **Elongation** is the arc of light, the true angular distance between
  the centres of the Sun and the Moon seen from the centre of the Earth.
- **Before 24:00 UT, anywhere** is judged on the evening terminator at
  24:00 UT, at every quarter degree of latitude where the Sun sets. At a
  given latitude a later sunset sees an older Moon, farther from the Sun
  and higher, so the sunsets that fall exactly at midnight are the best
  that fall before it.
- **The mainland of the Americas** is judged along its western edge, for
  the same reason: a line through 67 coastal places, from Point Barrow to
  Cold Bay near the end of the Alaska Peninsula, and from Prince Rupert to
  Cape Froward, each at the coordinates of its Wikipedia article
  [wikipedia-coordinates-americas], followed in steps of at most a
  quarter of a degree. The sunset judged is that of the local date of the
  evening, so the line reaches 168° W at Cape Prince of Wales.
- **Dawn in New Zealand** is astronomical dawn, the Sun 18° below the
  horizon, at Wellington on the civil day after the evening. Neither text
  read gives the angle; Muhammadiyah names no place and Wellington stands
  for it too.
- The evening counts only after the conjunction and within the week that
  follows it.

For the searches, which judge several hundred sunsets an evening, the
Sun's and the Moon's apparent places are sampled at seven instants over
the day and a half in which the sunsets of one local date fall and
interpolated; the interpolation follows `hc-astro`'s series to 10⁻⁴ of a
degree.

**Worked example: Ramaḍān 1447.** The conjunction fell at 12:01 UT on
17 February 2026, as both bodies give it. At 24:00 UT that day the Moon was
6.11° from the Sun: nothing on the terminator reached 8°, so the condition
of anywhere before midnight failed for both rules, as Muhammadiyah says
[muhammadiyah-ramadan-1447]. Astronomical
dawn at Wellington on 18 February was at 16:06 UT on the 17th, four hours
after the conjunction, so the New Zealand condition held. Along the
Alaskan coast the evening of the 17th, local date, came after midnight UT:

| Place | Sunset (UT, 18 Feb) | Elongation | Geocentric altitude | Topocentric altitude |
| --- | --- | --- | --- | --- |
| Cape Prince of Wales | 03:47 | 8.04° | 4.50° | 3.54° |
| Bethel | 03:42 | 8.00° | 5.00° | 4.04° |
| Port Heiden | 03:41 | 7.99° | 5.37° | 4.42° |
| Nelson Lagoon | 03:54 | 8.10° | 5.55° | 4.60° |
| Cold Bay | 04:02 | 8.17° | 5.68° | 4.73° |

With the geocentric altitude the thresholds are met on the Alaska
Peninsula, at Nelson Lagoon and Cold Bay, and Ramaḍān begins on the 18th:
Muhammadiyah's figures, 8°00′06″ and 5°23′01″ "in Alaska", are those of a
point near Port Heiden, and its account of the case names Bethel, where
both figures stand at the thresholds [muhammadiyah-ramadan-1447]. With the
topocentric altitude no point of the mainland reaches 5°; the crescent is
visible that evening only over the Pacific, "not on land", as Diyanet says,
and Ramaḍān begins on the 19th [diyanet-ramazan-1447]. On the evening of
the 18th the thresholds are met at every point of the Alaskan coast, by
more than 10° of topocentric altitude. On the Pacific coast the
topocentric altitude falls below 5° at 44 of the 534 points, from 43.4° S
southward in Chilean Patagonia.

### The FCNA calendar

The Council's criterion is "that after conjunction somewhere on the globe,
at the local sunset, the angle between the sun and moon at local sunset
should be at least 8°, and the moon should be positioned at least 5° above
the horizon", the month beginning the next day if so and the day after
otherwise [fcna-calendar]. It names no frame and no limit on where
"somewhere" may be; for Ramaḍān 1447 it counted the thresholds met in
Polynesia or the Fiji region [fcna-ramadan-1447]. One day is set
outside the rule: "Eid al-Aḍḥā will be the day after Yawm 'Arafah as
determined by the Supreme Court of Saudi Arabia" [fcna-calendar], so the
Council's Eid al-Aḍḥā can differ from the 10th of its own Dhū al-Ḥijja.
The Council's earlier rule, from its conference of 10 June 2006, began the month at sunset of the
day on which the conjunction occurred before 12:00 noon GMT [fcna-2006].

### Neo-MABIMS

The criterion is a Moon at least 3° above the horizon and at least 6.4°
from the Sun, "the angular distance from the centre of the moon to the
sun", at sunset [mufid-djamaluddin-2023]. Mufid and Djamaluddin note that
the formula does not fix whether the two quantities are topocentric or
geocentric, and that practice differs; Djamaluddin's own analysis of 1447
uses a geocentric elongation, as a stated modification
[djamaluddin-kalender-1447]. Where the crescent is judged is not fixed by
the criterion either: Djamaluddin's analysis asks whether it is met in
Southeast Asia.

### Odeh's criterion

Odeh's criterion judges the crescent at Bruin's best time,
T<sub>b</sub> = T<sub>s</sub> + (4/9) Lag, by two quantities: the airless
topocentric arc of vision ARCV, the Moon's altitude less the Sun's, and the
topocentric width of the crescent W in minutes of arc. His equation 2 is
V = ARCV − (−0.1018 W³ + 0.7319 W² − 6.3226 W + 7.1651), and the zones are
A, visible by naked eyes, for V ≥ 5.65; B, visible by optical aid and
possibly by naked eyes, for 2 ≤ V < 5.65; C, visible by optical aid only,
for −0.96 ≤ V < 2; and D below [odeh2004]. The cubic is Yallop's
([hijri.md](hijri.md)) with a different constant and without Yallop's
division by ten. Odeh gives no formula for the width; the library takes
Yallop's topocentric semi-diameter times 1 − cos ARCL with the topocentric
arc of light.

## What is carried

- **`islamic-khgt`**, `IslamicGlobalCalendar::KHGT`: Muhammadiyah's rule
  as read above, 1900–2100, in use from 26 June 2025.
- **`islamic-istanbul-2016`**, `IslamicGlobalCalendar::ISTANBUL_2016`:
  Diyanet's rule as read above, 1900–2100. The statement dates the
  criteria to the Ru'yet-i Hilâl Conference of Istanbul in 1978 and their
  confirmation to the congress of 2016 [diyanet-ramazan-1447]. No source
  read says from when Diyanet's lists follow the 2016 rule, so its period
  of use is unrecorded.
- **`islamic-fcna`**, `IslamicFcnaCalendar`: the Council's published
  table, 1 Muḥarram 1440 to the end of Jumādā II 1465, 11 September 2018 to
  7 June 2043. The page lists every month to Dhū al-Ḥijja 1467 but the first
  of Shaʿbān 1465, so the length of Rajab 1465 is unknown, and that month and
  every later one are refused. No rule is computed for it: none this
  library tried reproduces it (below). The table dates months, not the
  Council's Eid al-Aḍḥā, which follows the Saudi Supreme Court's day of
  ʿArafah [fcna-calendar] and is not carried. The 2006 rule is not carried, since
  no month the Council dated by it was read.
- **The criteria at a place**, each a `NamedCriterion` that any
  `ObservationSite`, and the WebAssembly and C exports' `hc_crescent_visible`,
  can judge an evening by:
  `istanbul-2016` (geocentric elongation 8°, topocentric altitude 5°),
  `khgt` (both geocentric), `mabims-2021-topocentric` (both topocentric,
  6.4° and 3°), `mabims-2021-geocentric-elongation` (geocentric elongation,
  topocentric altitude) and `odeh` (visible from V ≥ 2, with `OdehZone` for
  the four zones). The two Neo-MABIMS readings are the ones the sources
  support; no official text read fixes either, and each has its own name.
- **Not carried, and why.**
  - A Neo-MABIMS calendar: the criterion names no place, the member states
    set their months after a sighting session, and no body publishes a
    calendar under it that could check one.
  - What any body announced on the day. The published lists are used to
    check the rules, not carried as data.
  - A land model beyond the western edge of the Americas. The interior
    and the eastern coast never see a later sunset than the western edge
    at the same latitude, so they cannot decide a month.

## Accuracy

| Measure | Result | Test |
| --- | --- | --- |
| Ramaḍān 1447: `islamic-khgt` on 18 February 2026 and `islamic-istanbul-2016` on 19 February, each for the reason its body gives | Both | `ramadan_1447_begins_a_day_apart_and_for_the_stated_reasons` |
| 1 Muḥarram 1447 = 26 June 2025 under `islamic-khgt` | Reproduced | `the_calendar_began_on_the_first_of_muharram_1447` |
| Muhammadiyah's page, 1447–1449 AH, the years in force and just ahead | 36 of 36 | `khgt_s_published_months_are_reproduced_for_1447_to_1449` |
| Muhammadiyah's page, all 551 months of 1447–1492 carried | 493 on the same day, 52 a day later here, 6 a day earlier, every difference from 1450 on | the same, in a release build |
| Diyanet's lists, 174 month starts of 1443–1457 AH | 172 of 174: Dhū al-Qaʿda 1444 and Dhū al-Ḥijja 1453 a day earlier here | `diyanet_s_published_months_are_reproduced_but_two` |
| The FCNA table against the Council's page | 306 of 306 carried, row for row | `the_fcna_table_is_the_councils_page` |
| The two Unified Hijri rules against the FCNA page, 335 months | KHGT's 305, Diyanet's 300 | `neither_unified_rule_reproduces_the_fcna_table`, in a release build |
| Every day of 1900–2100 converts both ways under both rules | All in a release build, several minutes of one core for the two; every 101st day and every 1 Muḥarram with the day before it in a debug one | `every_day_of_1900_to_2100_round_trips_under_both_rules` |
| Every month both rules begin in 1900–2100 has 29 or 30 days, as Muhammadiyah requires of its calendar [muhammadiyah-ughc-2025, §C.2] | All in a release build; every seventh year in a debug one | `every_month_has_twenty_nine_or_thirty_days` |
| The interpolated sky against `hc-astro` | Sun and Moon under 10⁻⁴ degree, sidereal time under 10⁻³ | `the_interpolated_sky_follows_the_ephemeris` |
| The searches' sunsets against `hc-astro`'s | Within 3 s | `the_searched_sunsets_are_the_crates_sunsets` |
| Odeh's Table V from his equation 2 | Every one of the 27 cells to the printed tenth | `odehs_equation_reproduces_his_table_five` |
| Djamaluddin's twelve evenings of 1447 by Neo-MABIMS, judged at Banda Aceh and Jakarta | 11 of 12 under either reading; Rajab's evening of 20 December 2025, met "di wilayah Indonesia", is met at neither place | `the_two_mabims_readings_against_djamaluddins_1447` |

**What the disagreements say.** Diyanet's two: the evenings of Dhū
al-Qaʿda 1444 and Dhū al-Ḥijja 1453 meet the thresholds here and not in
Diyanet's computation. Muhammadiyah's page agrees exactly for the years in force; from
1450 on it begins 52 months a day earlier than the rule as its texts state
it, and 6 a day later. A reading that follows the site's "atau" — the
thresholds met anywhere after midnight with the conjunction before dawn in
New Zealand, or met on the Americas — dates some of those months as the
page does and dates months of 1447 wrongly, so it is not the page's rule
either. The FCNA page is not reproduced by any reading tried here,
geocentric or topocentric, anywhere or on land, before midnight or on the
local date, the closest missing 30 of its 335 months; that is why its
calendar is carried as its table.

**What is not measured.** The dawn angle in New Zealand: the tests do not
vary it. The coastline between the places carried, which the line cuts
across bays and fjords.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [muhammadiyah-ughc-2025] | KHGT's principles and parameters, note 31, the start on 1 Muḥarram 1447 | Yes, the PDF, 2026-09-27 |
| [muhammadiyah-khgt-site] | The parameters as the site states them, geocentric, and the "atau" of its summary | Yes, 2026-09-27 |
| [khgt-kalendar-hijriah] | The first day of every month of 1447–1492 | Yes, 2026-09-27 |
| [muhammadiyah-ramadan-1447] | 1 Ramaḍān 1447 = 18 February 2026; the conjunction, the New Zealand condition, Bethel, the figures in Alaska | Yes, 2026-09-27 |
| [diyanet-ramazan-1447] | Diyanet's criteria, "anywhere on Earth", the Wellington condition, the conferences of 1978 and 2016, 1 Ramaḍān 1447 = 19 February 2026, visibility over the Pacific | Yes, 2026-09-27 |
| [diyanet-dini-gunler] | The first days of months of 1443–1457 | Yes, 2026-09-27 |
| [diyanet-kongre-2016] | The congress and its final declaration | The declaration only, 2026-09-27; the Scientific Committee's working paper, which Muhammadiyah cites for the rule, not read |
| [djamaluddin-khgt-turki-2025] | The frames of the Turkish criterion | Yes, 2026-09-27; secondary |
| [djamaluddin-kalender-1447] | Neo-MABIMS with a geocentric elongation; the twelve evenings of 1447 | Yes, 2026-09-27 |
| [djamaluddin-mabims-2022] | Neo-MABIMS's ratification and adoption | Yes, 2026-09-27 |
| [mufid-djamaluddin-2023] | The Neo-MABIMS criterion and its open frames | Yes, 2026-09-27 |
| [fcna-calendar] | The Council's criterion, its table, and Eid al-Aḍḥā after the Saudi Supreme Court's day of ʿArafah | Yes, 2026-09-27 |
| [fcna-ramadan-1447] | 1 Ramaḍān 1447 = 18 February 2026, 1 Shawwāl = 20 March | Yes, 2026-09-27 |
| [fcna-2006] | The 2006 rule | Yes, 2026-09-27 |
| [odeh2004] | Odeh's criterion, Table V and equation 2 | Yes, the PDF, 2026-09-27; Table VI not extracted |
| [wikipedia-coordinates-americas] | The places on the western edge of the Americas, and Wellington | Yes, 2026-09-27 |

The ratified Neo-MABIMS text and the Kemenag and JAKIM statements of how
they apply it were not read; they are what would decide between the two
readings.

## Code

`crates/hc-calendars-lunar/src/islamic_global.rs` holds `GlobalRule`, its
two constants, the coast, the interpolated sky and
`IslamicGlobalCalendar`; `islamic_fcna.rs` the FCNA table;
`islamic_observational.rs` `SunsetCriterion`, `Frame`, `VTestCriterion`,
`OdehZone`, the topocentric quantities, the named criteria, and the month
search and month count shared through `NewMonthRule`, `decompose_with` and
`compose_with`. The tests are in those modules and in
`crates/hc-calendars-lunar/tests/unified_hijri.rs`, against
`tests/data/khgt_month_starts.txt`, `diyanet_month_starts.txt` and
`fcna_month_starts.txt`.
