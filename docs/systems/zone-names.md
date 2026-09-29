# Zone names and day periods: what CLDR calls a time zone and a time of day

Backs `hc-i18n::zone_names` and `hc-i18n::day_periods`, and through them
the CLDR pattern fields `z`, `O`, `v`, `V`, `b` and `B` of
`hc-format::patterns::cldr`. No calendar identifier is registered: these
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
   "+HH:mm;−HH:mm", with a minus sign; a locale's digits are its own.

Where a name is missing, each field falls back as UTS #35's list says: `z`
to the short localized GMT format, `zzzz` to the long, `v` and `vvvv` to
the location and then the localized GMT format, `V` to `unk`, `VVV` to
the unknown zone's city (*Unknown Location*), `VVVV` to the long localized
GMT format. *Type fallback* fills a type a metazone lacks: where it has no
daylight name, it "doesn't require daylight support", and every type takes
its generic name, else its standard one, so Greenwich's `vvvv` is
*Greenwich Mean Time*.

### A time of day

`B` at 15:30 in English: `dayPeriods.xml`'s English rules put 15:30 in
`afternoon1`, from 12:00 before 18:00, whose format name is *in the
afternoon*. At 12:00:00 exactly English has *noon*, so `b` and `B` both
write it; German's rules have no noon, so its `b` writes the pm name and
its `B` the period *mittags*, from 12:00 before 13:00. Japanese's 夜中,
`night2`, runs from 23:00 across midnight before 04:00, so `B` at 01:00 is
夜中.

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
  with.
- **With `localized-zone-names`:** every other carried locale's names, 55
  tables, some 600 kB of text. A language entry carries what its file
  states at the `approved` and `contributed` levels, a regional entry what
  its files resolve apart from its parent's, as the regional locales of
  `docs/i18n.md` are read, so that `en-GB` has *CET* and *CEST*, which
  `en_GB.xml` gives and `en.xml` does not.
- **Day periods**, always: each carried language's format rule set and the
  format names of midnight, noon and the flexible periods in the
  abbreviated, wide and narrow widths, resolved through `root.xml`.
- **In the pattern fields:** `z`, `O`, `v`, `V`, `b` and `B` as above;
  `z` takes a name the caller gave the context first, as it always has;
  and `ZZZZ` is `OOOO`.
- **Not carried.**
  - The last of UTS #35's type fallbacks: a generic name needed where only
    the standard exists and "the offset and daylight offset do not change
    within 184 day +/- interval around the exact formatted time". It needs
    the zone's rules, which a pattern's context does not hold.
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

## Sources

| Key | What it gives | Read |
| --- | --- | --- |
| [uts35-dates-48] | "Using Time Zone Names", the Date Field Symbol Table's `z`, `v`, `V`, `O`, `b`, `B`, `U`, `r`, `g`, and "Day Period Rule Sets" | Yes, 2026-09-29 |
| [uts35-v48] | "Inheritance Marker", "Lateral Inheritance", "Parent Locales" | Yes, 2026-09-29 |
| [cldr48-zone-names] | `common/main/<file>.xml` `timeZoneNames`, `supplemental/metaZones.xml`, `bcp47/timezone.xml`, `supplemental/likelySubtags.xml` | Yes, 2026-09-29, by `scripts/zone-names-cldr.py` |
| [cldr48-day-periods] | `supplemental/dayPeriods.xml` and the carried files' day period names | Yes, 2026-09-29, by `scripts/day-periods-cldr.py` |

## Code

- `crates/hc-i18n/src/zone_names.rs` and the generated
  `zone_names/cldr48.rs` (`scripts/zone-names-cldr.py`, `--check`).
- `crates/hc-i18n/src/day_periods.rs` and the generated
  `day_periods/cldr48.rs` (`scripts/day-periods-cldr.py`, `--check`).
- `crates/hc-format/src/patterns/zone.rs`, the composition; the fields in
  `patterns/cldr.rs`.
