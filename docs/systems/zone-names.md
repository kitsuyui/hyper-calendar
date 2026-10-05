# Zone names and day periods: what CLDR calls a time zone and a time of day

Backs `hc-i18n::zone_names` and `hc-i18n::day_periods`, and through them
the CLDR pattern fields `z`, `O`, `v`, `V`, `b` and `B` of
`hc-format::patterns::cldr`, with the offset fields `Z`, `X` and `x`
beside them. No calendar identifier is registered: these
are the words a date's time and zone are written in.

## What it is

UTS #35 Part 4, *Dates*, version 48.2 [uts35-dates-48], "Using Time Zone
Names", names a time zone three ways:

- by an offset, the *localized GMT format*: *GMT-7*, *UTC+01:00*, from a
  locale's `gmtFormat` ("GMT{0}"), `hourFormat` ("+HH:mm;-HH:mm") and
  `gmtZeroFormat` ("GMT");
- by the wall clock of a group of zones, the *generic* names, *Pacific
  Time*, which a meeting that recurs across a daylight change is written in;
- by one reading of that clock, the *specific* names, *Pacific Standard
  Time* and *Pacific Daylight Time*, each "equivalent to a particular
  offset from GMT".

The group is a *metazone*, "a collection of time zones that share the same
behavior and same name during some period". The names are kept by
metazone, so that the thirty-odd zones of the United States share four
sets of words; the supplemental `metaZones.xml` says which metazone a zone
is in at each instant, since "zones may join or leave a metazone over
time"; and a zone may have names of its own besides its metazone's, as
`Europe/London`'s daylight time is *British Summer Time* in English. Each
metazone has a *golden zone*, the zone of territory `001` in
`mapTimezones`, and may have a *preferred zone* in a territory:
`America/Vancouver` is the Pacific metazone's in Canada.

Where a zone has no name, a *location* names it: the locale's
`regionFormat` ("{0} Time") around its country's name, where the zone is
its country's only zone or its *primary zone* (`primaryZones`,
`Asia/Shanghai` for China), and around its exemplar city otherwise:
*Italy Time*, *Buenos Aires Time*.

The offset fields write the offset in ISO 8601's shapes instead of a
locale's: the Date Field Symbol Table makes `X` and `x` "the ISO8601 basic
format with hours field and optional minutes field" (`+05`, `+0530`), `XX`
and `xx` the basic form and `XXX` and `xxx` the extended one with hours and
minutes, `XXXX`, `xxxx` and `Z` to `ZZZ` the basic form "with hours,
minutes and optional seconds" (`-0800`, `-075258`), and `XXXXX`, `xxxxx`
and `ZZZZZ` the extended one (`-08:00`, `-07:52:58`); the capitals write
`Z` for a zero offset, and `ZZZZ` is the long localized GMT format. The
`Z` to `ZZZ` seconds are the table's own (`xxxx`'s example is `-075258`):
`-045602` for UTC−4:56:02, and not `-0456`.

A locale's data may also say that it has no value. UTS #35 Part 1,
"Empty Override" [uts35-v48], reserves `∅∅∅` "to indicate that a child
locale is to have no value for a path, even if the parent locale has a
value for that path". `en_001.xml` writes it for the short Pacific names,
which `en.xml` gives as *PT*, *PST* and *PDT*, and `ja.xml` for the short
generic name of `Japan` beside its *JST* and *JDT*.

The same Part's "Day Period Rule Sets" name the times of a day beyond am
and pm. Midnight and noon are fixed, 00:00 and 12:00, and "all locales must
support am/pm, but not all support noon or midnight": German has "no unique
term that means exactly 12:00 noon". A language may divide the day into
flexible periods, `morning1` to `night2`, which "completely cover the 24
hours" and may cross midnight; where it has none, "the computation of
dayPeriods falls back to AM/PM".

## How it works

### A zone's names

Take `America/Vancouver` at 2026-07-01 12:00 at −07:00, in English, for
each field.

1. **Canonicalize.** `bcp47/timezone.xml` gives the zone its short
   identifier `cavan`, CLDR's own identifier (the first of its aliases,
   here the same) and its region, `CA` (the first two letters of a
   five-letter identifier, unless a `region` attribute says otherwise).
   `Asia/Kolkata` canonicalizes to CLDR's `Asia/Calcutta`, by which its
   data is keyed.
2. **The metazone at the instant.** 2026-07-01 12:00 −07:00 is 19:00 UTC,
   29 715 540 minutes after 1970; `metaZones.xml` puts Vancouver in
   `America_Pacific` with no end.
3. **`zzzz`, the long specific name.** The reading is daylight time (the
   caller says so, from its zone's rules), so the zone's own long daylight
   name, which English has none of, else the metazone's: *Pacific Daylight
   Time*. `z` is the short one, *PDT*.
4. **`vvvv`, the long generic name.** The metazone's generic name is
   *Pacific Time*. English's likely region is `US`; the metazone has no
   preferred zone there, so its golden zone, `America/Los_Angeles`, stands
   for it, and that is not Vancouver. Vancouver is the metazone's preferred
   zone in its own region, Canada, so the name is qualified by the country,
   through the `fallbackFormat` "{1} ({0})": *Pacific Time (Canada)*. For
   `America/Los_Angeles` the same steps stop at *Pacific Time*.
5. **`VVVV`, the location.** Canada has many zones and Vancouver is not
   its primary one, so the exemplar city: *Vancouver Time*.
6. **`V`, `VV`, `VVV`.** `cavan`; the zone as given; *Vancouver*.
7. **`O`, `OOOO`.** *GMT-7*, *GMT-07:00*. French writes *UTC−7* and
   *UTC−07:00*, its `gmtFormat` being "UTC{0}" and its `hourFormat`
   "+HH:mm;−HH:mm", with a minus sign; a locale's digits are its own. At a
   zero offset both write the locale's `gmtZeroFormat`, *GMT* (Arabic's
   *غرينتش*): UTS #35 defines it as "how GMT/UTC with an offset of zero
   should be represented", ICU4J documents it for the long form and the
   short one [icu-zone-format-sources], and ICU 76.1 writes it for `OOOO`.
   The spec's list of long examples shows *GMT+00:00*, which that
   definition does not give; this follows the definition.

Where a name is missing, each field falls back as UTS #35's list says: `z`
to the short localized GMT format, `zzzz` to the long, `v` and `vvvv` to
the location and then the localized GMT format, `V` to `unk`, `VVV` to
the unknown zone's city (*Unknown Location*), `VVVV` to the long localized
GMT format. A name a locale's file writes as the empty override is
missing in that way: the lookup stops at that file, with no name, and
does not go on to the parent's. So `en-GB`'s `z` for Los Angeles in July
is *GMT-7*, not *PDT* and not `∅∅∅`.

*Type fallback* fills a type a metazone lacks: where it has no daylight
name, it "doesn't require daylight support", and every type takes its
generic name, else its standard one, so Greenwich's `vvvv` is *Greenwich
Mean Time*. The premise does not hold of a zone that is keeping daylight
time, and its standard name would state another offset, so the library
applies the fallback to a standard reading and not to a daylight one.
`Europe/London` at 2026-07-01 12:00 +01:00, in English:

1. The reading is daylight time. The zone's own long daylight name is
   *British Summer Time*, so `zzzz` is that.
2. `en.xml` gives London no short name of its own, and the zone's metazone
   since 1971, `GMT`, has only standard names, *Greenwich Mean Time* and
   *GMT*. There is no short daylight name.
3. So `z` falls to the short localized GMT format: *GMT+1*. The standard
   *GMT* would say UTC+0.

ICU4J's `TimeZoneFormat.formatSpecific` asks for the daylight name alone
for a daylight reading too [icu-zone-format-sources]; it is a comparison,
not the source of the rule.

The last type fallback reads the zone's rules: "Otherwise if the generic
type is needed, but not available, and the offset and daylight offset do
not change within 184 day +/- interval around the exact formatted time,
use the standard type", with the example "Mountain Standard Time" for
Phoenix [uts35-dates-48]. `America_Mountain` has a generic name, *Mountain Time*,
so the example applies the rule where the text's "not available" would
not; the library follows the example, and so covers both. `America/Phoenix`
at 2026-01-15 12:00 −07:00, in English, `vvvv`:

1. The zone has no generic name of its own; its metazone is
   `America_Mountain`.
2. The reading is standard time, and the caller's rules, `MST7`, change
   neither the offset nor the daylight flag from 2025-07-15 to 2026-07-18,
   184 days either side. The zone is steady.
3. So the standard name, the zone's own (none) else the metazone's:
   *Mountain Standard Time*, unqualified, where the generic path would
   write *Mountain Time (Phoenix)*, Phoenix not being the metazone's
   preferred zone. `v` is *MST*.

`America/Denver` the same day is not steady, since its rules change on
8 March, 52 days on, so it stays *Mountain Time*; `Asia/Tokyo`, `JST-9`,
is steady, and its `vvvv` is *Japan Standard Time*. The daylight flag
stands for the daylight offset, which a zone's rules do not state apart
from the offset. Where the standard name is the generic name's own text,
the generic path is kept, with its qualifier (`generic`'s rustdoc states the
rule). A context without the zone's rules keeps the generic name: `hc-format`'s `FormatContext` takes them with
`with_zone_rules`, and `hc_zone_name` and `hc_format_pattern` pass the
zone's. ICU4J's `TimeZoneGenericNames` writes the standard name for a zone
that keeps no daylight time within 184 days [icu4j-generic-names], a
comparison again. `en-GB` has *BST* from `en_GB.xml`, and
`Europe/Dublin` is the same case: its own long daylight name in English is
*Irish Standard Time*, for the summer, so CLDR's names read Irish summer
time as daylight time. The tz database's main format says the opposite in
its flag [tz-europe-eire-rules]: the `europe` file's "Rule Eire 1996 max - Oct lastSun 1:00u
-1:00 -" and "Zone Europe/Dublin ... 1:00 Eire IST/GMT" make UTC+1 `IST`
the standard time and winter's UTC+0 `GMT` a negative saving, flagged as
daylight (the footer is `IST-1GMT0,M10.5.0,M3.5.0/1`), where the rearguard
format, which the host's TZif is (macOS's tzdata 2026c, `isdst=1` from 29
March 2026), flags the summer. So the names read
`hc_tz::TimeZone::is_summer_time_at`, not `is_dst_at`: the reading with the
higher of the two offsets the zone keeps within a year either side of the
instant, and the flag where it keeps only one. `hc_zone_name` and
`hc_format_pattern` pass it, so both formats of the data give *Irish
Standard Time* in July and *Greenwich Mean Time* in January; `hc_zone_offset`
still gives the data's own flag, as its line says.

### A time of day

The rule sets are keyed by language, a few by language and script or
region (`hi_Latn`, `es_CO`), and a locale's is found by truncating its tag:
`zh-Hant-HK`, `zh-Hant`, `zh`. That is not the names' chain, since
`parentLocales` send `zh-Hant` and `yue-Hans` straight to root. ICU4C's
`DayPeriodRules::getInstance` truncates in the same way
[icu-zone-format-sources].

`B` at 15:30 in English: `dayPeriods.xml`'s English rules put 15:30 in
`afternoon1`, from 12:00 before 18:00, whose format name is *in the
afternoon*. At 12:00:00 exactly English has *noon*, so `b` and `B` both
write it; German's rules have no noon, so its `b` writes the pm name and
its `B` the period *mittags*, from 12:00 before 13:00. Japanese's 夜中,
`night2`, runs from 23:00 across midnight before 04:00, so `B` at 01:00 is
夜中. `B` at 20:00 in `zh-Hant`: `zh`'s rules put it in `evening1`, from
19:00, and `zh_Hant.xml` names that 晚上.

## What is carried

- **The formats** of every carried locale with a CLDR file — `hourFormat`,
  `gmtFormat`, `gmtZeroFormat`, `gmtUnknownFormat`, `regionFormat` in its
  three types, `fallbackFormat` and the exemplar city of `Etc/Unknown` —
  and `root.xml`'s as their floor, always: a few hundred bytes a locale.
- **With the `zone-names` feature:** the periods of each zone's metazones,
  the golden and preferred zones, the primary zones, every zone of
  `bcp47/timezone.xml` with its aliases, IANA name and region (a
  deprecated identifier's aliases join its preferred one's), the region
  each carried language's likely subtags give it, and English's metazone
  and zone names with root's (`Etc/UTC`'s *UTC*). It turns on the
  exemplar cities and the territory names, which a location is written
  with: every territory CLDR names, from `hc-i18n`'s `place_names`
  (`docs/systems/place-names.md`), its short form first, at the release
  levels, so that `America/Puerto_Rico` is *Puerto Rico Time*.
- **With `localized-zone-names`:** every other carried locale's names, 49
  tables with English's, some 600 kB of text. A language entry carries what its file
  states at the `approved` and `contributed` levels, a regional entry what
  its files resolve apart from its parent's, as the regional locales of
  `docs/i18n.md` are read, so that `en-GB` has *CET* and *CEST*, which
  `en_GB.xml` gives and `en.xml` does not. A name a file writes as the
  empty override is carried as `~`, "no name", where the entry would
  otherwise inherit its parent's: 41 names in `en-001`, `es-419` and
  `pt-PT`. `ja.xml`'s one, whose parent is root with no name there, is
  left empty; `scripts/cldr_xml.py` resolves it for
  every generator, and `tests/cldr_markers.rs` fails on either marker in
  any file a generator writes.
- **Day periods**, always: every format rule set of a carried language,
  those keyed by its script or region among them, and the format names of
  midnight, noon and the flexible periods in the abbreviated, wide and
  narrow widths, resolved through `root.xml`.
- **In the pattern fields:** `z`, `O`, `v`, `V`, `b` and `B` as above,
  the 184-day type fallback among them where the context holds the zone's
  rules;
  `z` takes a name the caller gave the context first, as it always has;
  `ZZZZ` is `OOOO`; and `Z`, `X` and `x` in every width, as above.
- **Not carried.**
  - The `nonlikelyScript` parent rule beyond the tags the table lists, as
    for the other locale data.
  - Parsing a zone name back to a zone, which `hc-format` refuses for `z`,
    `v` and `V`, since names are not unique.
  - The stand-alone ("selection") day period rules, which choose a message
    rather than write a time.

## Accuracy

The data is CLDR's, resolved as UTS #35 Part 1 resolves it, and the
composition follows Part 4's steps; the tests hold them to Part 4's own
examples:

| Check | Source | Test | Result |
| --- | --- | --- | --- |
| `America/Cambridge_Bay` leaves `America_Mountain` for `America_Central` at 1999-10-31 08:00 UTC; `Europe/London` leaves `British` for `GMT` at 1971-10-31 02:00 | [uts35-dates-48], `metaZones.xml` | `a_zone_changes_metazone_at_the_minute_the_data_gives` (`hc-i18n`) | agrees to the minute |
| PDT, Pacific Daylight Time, PT, Pacific Time, uslax, Los Angeles, Los Angeles Time, GMT-7, GMT-07:00; Pacific Time (Canada) for Vancouver; China Time, Italy Time, Buenos Aires Time | [uts35-dates-48], the symbol table and "Using Time Zone Names" | `the_zone_fields_write_tr_35s_examples` (`hc-format`) | every example |
| British Summer Time and Greenwich Mean Time for London; India Standard Time for `Asia/Kolkata` | `en.xml` | the same | agrees |
| 日本標準時 and 東京; Mitteleuropäische Sommerzeit and MESZ | `ja.xml`, `de.xml` | `the_zone_names_are_the_locales` | agrees |
| noon, midnight, in the afternoon, at night; Mitternacht and mittags; 夜中 at 01:00 | `dayPeriods.xml`, `en.xml`, `de.xml`, `ja.xml` | `the_extended_day_periods_follow_each_languages_rules` | agrees |
| `GMT` for `O` and `OOOO` at a zero offset in `en`, `de`, `ja` and `ar` (غرينتش) | ICU 76.1 (Node 22 `longOffset`, read 2026-10-03), ICU4J's `TimeZoneFormat` documentation | `the_localized_gmt_format_writes_a_zero_offset_as_the_zero_format`, `a_locales_zero_format_is_its_own_word` | agrees |
| Irish Standard Time in July and Greenwich Mean Time in January for Dublin, in the main-format and the rearguard footers | `en.xml`, [tz-europe-eire-rules] | `dublin_in_the_main_format_names_its_summer_as_daylight_time` (`hc-format`), `summer_time_is_the_higher_offset_where_the_saving_is_negative` (`hc-tz`) | agrees |
| GMT+1 and British Summer Time for London in July, GMT+1 and Irish Standard Time for Dublin, BST in `en-GB` | `en.xml`, `en_GB.xml`, `metaZones.xml` | `the_zone_fields_write_tr_35s_examples`, `the_zone_names_are_the_locales` | agrees |
| Mountain Standard Time and MST for Phoenix with rules that keep MST, Mountain Time for Denver in January, Japan Standard Time for Tokyo; the generic names where the context has no rules | [uts35-dates-48], "Type Fallback" | `a_zone_that_keeps_one_offset_for_184_days_takes_its_standard_name` (`hc-format`) | agrees |
| `+0530`, `-075258`, `-07:52:58`, `Z` for the ISO fields | [uts35-dates-48], the symbol table's examples | `the_iso_zone_fields_write_minutes_and_seconds_where_tr_35_does` | every example |
| No name where a file writes `∅∅∅`: `en-001`'s, `en-GB`'s, `en-AU`'s and `en-IN`'s short Pacific names, `es-419`'s short Eastern European ones, `pt-PT`'s for Brasília, `ja`'s short generic Japan name | `en_001.xml`, `es_419.xml`, `pt_PT.xml`, `ja.xml` | `the_empty_override_stops_the_lookup_with_no_name` (`hc-i18n`) | agrees |
| `zh`'s rules for `zh-Hant`, `zh-TW`, `zh-Hant-HK` and `zh-Hant-MO`, `yue`'s for `yue-Hans`, `hi_Latn`'s own | `dayPeriods.xml` | `the_rules_are_found_by_truncating_the_tag` (`hc-i18n`) | agrees |
| Every zone field and day period in every carried locale written, none a marker | the generated tables | `tests/cldr_names_sweep.rs` (`hc-format`) | every locale |

## Sources

| Key | What it gives | Read |
| --- | --- | --- |
| [uts35-dates-48] | "Using Time Zone Names" and its "Type Fallback", the Date Field Symbol Table's `z`, `v`, `V`, `O`, `Z`, `X`, `x`, `b`, `B`, `U`, `r`, `g`, and "Day Period Rule Sets" | Yes, 2026-09-29 |
| [uts35-v48] | "Inheritance Marker", "Empty Override", "Lateral Inheritance", "Parent Locales" | Yes, 2026-09-29 |
| [icu-zone-format-sources] | ICU4J's `TimeZoneFormat.formatSpecific` and ICU4C's `DayPeriodRules::getInstance`, a comparison for the daylight name and the truncation, not a source of either rule; and the zero offset's format | Yes, 2026-09-29 and 2026-10-03 |
| [tz-europe-eire-rules] | The `Eire` rules and `Europe/Dublin` of the tz database's `europe` file: a negative winter saving in the main format | Yes, 2026-10-03 |
| [icu4j-generic-names] | ICU4J's `TimeZoneGenericNames.formatGenericNonLocationName`, a comparison for the 184-day rule | Yes, 2026-09-29 |
| [cldr48-zone-names] | `common/main/<file>.xml` `timeZoneNames`, `supplemental/metaZones.xml`, `bcp47/timezone.xml`, `supplemental/likelySubtags.xml` | Yes, 2026-09-29, by `scripts/zone-names-cldr.py` |
| [cldr48-day-periods] | `supplemental/dayPeriods.xml` and the carried files' day period names | Yes, 2026-09-29, by `scripts/day-periods-cldr.py` |

## Code

- `crates/hc-i18n/src/zone_names.rs` and the generated
  `zone_names/cldr48.rs` (`scripts/zone-names-cldr.py`, `--check`).
- `crates/hc-i18n/src/day_periods.rs` and the generated
  `day_periods/cldr48.rs` (`scripts/day-periods-cldr.py`, `--check`).
- `crates/hc-format/src/patterns/zone.rs`, the composition, the 184-day
  check (`steady`) among it; the fields in
  `patterns/cldr.rs`, the ISO offsets' shared writer in `patterns.rs`
  (`exact_offset_style`, which `strftime`'s `%z` uses too).
- `scripts/cldr_xml.py`, the resolution the generators share, the empty
  override among it; `crates/hyper-calendar/tests/cldr_markers.rs`.
