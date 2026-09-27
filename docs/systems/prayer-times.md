# Islamic prayer times by named method

Backs `hc-astro::solar_time`'s `PrayerMethod`, `PRAYER_METHODS`,
`prayer_method`, `fajr`, `maghrib`, `isha` and `islamic_midnight`. The
afternoon prayer, *ʿaṣr*, and the other times of day the module carries are
in [hours-of-the-day.md](hours-of-the-day.md).

## What it is

The five daily prayers of Islam are fixed by the Sun: *fajr* at dawn,
*ẓuhr* after noon, *ʿaṣr* in the afternoon, *maghrib* at sunset and *ʿishāʾ*
at nightfall. Dawn and nightfall are when the sky begins to lighten and has
finished darkening, and the authorities that publish timetables fix each by
a depression of the Sun below the horizon; they differ on the angles, and
some fix *ʿishāʾ* as an interval after sunset instead. A timetable for a
place is therefore one authority's method applied there. The methods
compiled by Pray Times are those of the Muslim World League, the Islamic
Society of North America, the Egyptian General Authority of Survey, Umm
al-Qura University in Makkah, the University of Islamic Sciences in Karachi,
the Institute of Geophysics of the University of Tehran, the Leva Research
Institute in Qom (the Jafari method), the Muslims of France, the Spiritual
Administration of Muslims of Russia and the Islamic Religious Council of
Singapore (MUIS) [praytimes-methods].

## How it works

Each method gives:

| Method | *Fajr* | *Maghrib* | *ʿIshāʾ* | Middle of the night |
| --- | --- | --- | --- | --- |
| Muslim World League (`mwl`) | 18° | sunset | 17° | sunset to sunrise |
| ISNA (`isna`) | 15° | sunset | 15° | sunset to sunrise |
| Egypt (`egypt`) | 19.5° | sunset | 17.5° | sunset to sunrise |
| Umm al-Qura (`umm-al-qura`) | 18.5° | sunset | 90 min after *maghrib*, 120 in Ramaḍān | sunset to sunrise |
| Umm al-Qura before Muḥarram 1430 (`umm-al-qura-before-1430`) | 19° | sunset | as above | sunset to sunrise |
| Karachi (`karachi`) | 18° | sunset | 18° | sunset to sunrise |
| Tehran (`tehran`) | 17.7° | 4.5° | 14° | sunset to *fajr* |
| Jafari (`jafari`) | 16° | 4° | 14° | sunset to *fajr* |
| France (`france`) | 12° | sunset | 12° | sunset to sunrise |
| Russia (`russia`) | 16° | sunset | 15° | sunset to sunrise |
| Singapore (`singapore`) | 20° | sunset | 18° | sunset to sunrise |

An angle is the depression of the Sun's centre below the geometric horizon,
without refraction. Sunset and sunrise are `hc-astro`'s, the upper limb on
the horizon with the centre 50′ below. *Ẓuhr* is solar noon, the Sun's
transit. Pray Times notes that the Tehran method does not state its
*ʿishāʾ* angle explicitly, and that Umm al-Qura's *fajr* was at 19° before
Muḥarram 1430, December 2008 [praytimes-methods]; the earlier method keeps
its own identifier, as an authority's revision is data
(`docs/policy.md` §10).

Where the Sun does not reach a method's angle — in summer at high
latitudes, or where it does not set — the time does not exist by that
method. The compilation read gives no method's own rule for those
latitudes, the authorities' own texts were not read, and no such rule is
carried, so the functions return `MissingSolarEvent::DawnDepression` for *fajr*,
`MissingSolarEvent::Depression` for an evening angle, or
`MissingSolarEvent::Sunset`, and compute nothing in its place.

**Worked example: Singapore, 1 January 2026, by the Singapore method.** At
1°17′ N, 103°50′ E the Sun's centre rises through 20° below the horizon at
21:43.0 UT on 31 December, 05:43.0 local time (UTC+8); MUIS prints Subuh at
5:44. Sunset is at 19:09.7 local and MUIS's Maghrib 7:11; the Sun reaches
18° below the horizon at 20:24.4, and Isyak is 8:25; the transit is at
13:08.1, and Zohor 1:10 [muis-prayer-timetable-2026]. The published times
are the computed ones rounded up, and up to a minute or so later. That is
not a margin of caution alone: Syuruk, the sunrise that ends the time of
Subuh, is also printed later than the computed sunrise, where caution
would put it earlier. MUIS does not state how it rounds or adjusts.

## What is carried

- `PrayerMethod`, one per method above, with `id`, English name, the
  *fajr* depression, `MaghribRule`, `IshaRule`, `MidnightRule` and its
  source; `PRAYER_METHODS` lists them and `prayer_method` finds one by
  identifier. The list is data (ADR 0007).
- `fajr`, `maghrib`, `isha` and `islamic_midnight` for a local day, a place
  and a method, in Universal Time. `isha` takes whether the day is in
  Ramaḍān, which only Umm al-Qura's methods read; `hc-astro` has no
  calendar, so the caller says.
- **Not carried.** The rules applications use at high latitudes — the
  middle of the night, a seventh of the night, the angle's share of the
  night — which Pray Times offers as general adjustments rather than as
  any method's own. The Muslim World League's "Local Relative Estimation"
  for latitudes 48.6° to 66.6°, which the International Astronomical
  Center reports its Fiqh Council approved at a meeting in Mecca on
  1 August 2009 [iac-high-latitudes]: that account is secondary, and the
  League's own text was not read. The authorities' margins of caution and their rounding.
  The choice between the Shafiʿi and Hanafi *ʿaṣr*, which is `asr_shafii`
  and `asr_hanafi`, since it is separate from the method.

## Accuracy

| Measure | Result | Test |
| --- | --- | --- |
| MUIS's timetable for 2026, twelve days through the year, by the Singapore method at Singapore | Every Subuh, Syuruk, Maghrib and Isyak the computed time or up to 1.5 min later; every Zohor 0.5 to 2.5 min after the transit | `the_singapore_method_reproduces_muis_timetable_for_2026` |
| The same timetable with *fajr* at 19° or 21° | Every Subuh 2 min or more outside that | the same |
| Each method's times at its angles, in the order of the day, Umm al-Qura's intervals | All eleven | `every_method_puts_its_times_at_its_angles` |
| The middle of the night by each rule | Both | `the_middle_of_the_night_follows_the_methods_rule` |
| Refusal where the Sun does not reach the angle: Tromsø at midsummer, London at 18° and 17° | Refused; 12° at London computed | `a_method_refuses_the_times_the_sun_does_not_reach` |

Only the Singapore method is checked against its authority's own
timetable. The other ten methods are computed from the compiled angles and
checked against nothing their authorities published: no timetable of
theirs was read. MUIS does not say which point of Singapore its timetable
is computed for; the test takes the city's coordinates from Wikipedia, and
a point elsewhere on the island moves the times by up to about a minute.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [praytimes-methods] | Every method's parameters, Umm al-Qura's earlier *fajr*, Tehran's unstated *ʿishāʾ* angle | Yes, 2026-09-27; a compilation. The authorities' own documents were not read |
| [muis-prayer-timetable-2026] | The anchor | Yes, 2026-09-27 |
| [iac-high-latitudes] | The Muslim World League's high-latitude method, not carried | Yes, 2026-09-27; secondary. The League's own text was not read |

## Code

`crates/hc-astro/src/solar_time.rs`: `PrayerMethod`, `IshaRule`,
`MaghribRule`, `MidnightRule`, the catalogue and the four functions, with
the tests named above.
