# Zone locations: where the IANA database puts each time zone, and what CLDR calls its city

Backs `hc-tz::location`, `hc-i18n::exemplar_cities` and
`hyper_calendar::zone_lines`, and through them the exports `hc_zones` and
`hc_zone_location`. No calendar identifier is registered: a zone's
location is a default place for the computations that need one, such as
sunrise, the young crescent and the pañcāṅga.

## What it is

The IANA time zone database names each zone after a city in it,
`Asia/Tokyo`, and records beside its rules where that city is. The file
`zone1970.tab` has one row for each zone "where civil timestamps have
agreed since 1970": the ISO 3166 codes of the countries the zone overlaps,
the "latitude and longitude of the timezone's principal location", the
zone's name, and a comment where a country has several zones. "If a
timezone covers multiple countries, the most-populous city is used, and
that country is listed first" [iana-tzdb-2026d, `zone1970.tab`].

The older `zone.tab` has one row for each pair of a country and a zone,
with one country code a row, and "unlike zone1970.tab, a row's third
column can be a Link from 'backward' instead of a Zone". It is kept "as a
backward-compatibility aid for older programs" [iana-tzdb-2026d,
`zone.tab`]. Because "the first data column contains exactly one country
code", each of its rows gives a zone one country, where `zone1970.tab`
may give several. Its rows for links matter: since the database merged zones
whose clocks have agreed since 1970, `Europe/Oslo` is a link to
`Europe/Berlin`, but `zone.tab` still places `Europe/Oslo` at Oslo.

The file `backward` holds the links from old and merged names to current
ones, `Link Asia/Kolkata Asia/Calcutta`. A `#= TARGET1` comment on a link
"says what the target would be if these parsers were fixed so that data
could contain links to links" [iana-tzdb-2026d, `backward`]. The file
`backzone` holds zones "outside the normal scope of the tz database", and
"links in this file point to zones in this file, superseding links in the
file 'backward'"; a link that holds only when the database is built with
`PACKRATLIST=zone.tab` is written as a comment that starts
`#PACKRATLIST zone.tab` [iana-tzdb-2026d, `backzone`].

Browsers report link names. ECMA-402 requires that "any Link name that is
present in the 'TZ' column of file zone.tab must be a primary time zone
identifier", so that `Europe/Oslo` stays `Europe/Oslo`, and resolves the
other links within their country, from `zone.tab` and `backzone`
[ecma402-2027, §6.5].

CLDR names a zone for a reader by its *exemplar city*: 東京 in Japanese,
`Tokyo` in English. Each locale's file has a `zone` element per zone with
an `exemplarCity` [cldr48-exemplar-cities], and "if the localized
exemplar city is not available, use as the exemplar city the last field of
the raw TZID, stripping off the prefix and turning _ into space"
[uts35-dates-48, Using Time Zone Names]. CLDR keys its zones by its own
identifiers, which are older IANA names for some zones: `common/bcp47/timezone.xml`
lists each zone's aliases, the first being CLDR's, so `Asia/Kolkata` is
`Asia/Calcutta` in CLDR's files [cldr48-exemplar-cities].

## How it works

### A coordinate

`zone1970.tab` writes a coordinate "in ISO 6709 sign-degrees-minutes-seconds
format, either ±DDMM±DDDMM or ±DDMMSS±DDDMMSS, first latitude (+ is
north), then longitude (+ is east)" [iana-tzdb-2026d, `zone1970.tab`]. The
row for Tokyo is

    JP,AU	+353916+1394441	Asia/Tokyo	Eyre Bird Observatory

- Latitude `+353916`: 35° 39′ 16″ north, 35 × 3600 + 39 × 60 + 16 =
  128 356″, and 128 356 / 3600 = 35.654444…°.
- Longitude `+1394441`: 139° 44′ 41″ east, 139 × 3600 + 44 × 60 + 41 =
  503 081″, 139.744722…°.

A row without seconds, `+4230+00131` for Andorra, is 42° 30′ = 42.5° and
1° 31′ = 1.51666…°. The digits are never decimal degrees: reading `+4230`
as 42.30° would put Andorra 22 km south of itself.

### A country

`zone1970.tab` gives `Asia/Tokyo` the countries `JP,AU`, Japan for Tokyo
and Australia for the Eyre Bird Observatory, which keeps Tokyo's clock.
A label wants one country, and `zone.tab` gives it:

    JP	+353916+1394441	Asia/Tokyo

so the line's `zone.tab` country is `JP`. No choice is made here: each
name has at most one row in `zone.tab`, and its country is read off. For
every zone but one it is the first country of `zone1970.tab`, the
country of the most populous city. The exception is `Europe/Simferopol`:
`zone1970.tab` lists `RU,UA`, a comment saying to "mention RU and UA
alphabetically", and `zone.tab` lists it as `UA` in its `RU` section, because its
"obsolescent" format "cannot represent Europe/Simferopol well"
[iana-tzdb-2026d, `zone.tab`]. A row of `zone.tab` itself, such as
`Europe/Oslo`, has its own country, `NO`. A name `zone.tab` had no row for
would have an empty cell; in release 2026d every one of the 418 names
with a row has one, which a test checks, but `zone.tab` is deprecated and
a later release may drop rows.

### A name

`hc_zone_location` answers a name in this order:

1. A row of `zone1970.tab`: `Asia/Tokyo` answers with its own row.
2. A row of `zone.tab` for a name `zone1970.tab` does not list:
   `NO +5955+01045 Europe/Oslo` answers with Oslo, 59.9166…° N,
   10.75° E, although `backward` has `Link Europe/Berlin Europe/Oslo`.
3. A link, followed to a name with a row: its `backzone` target where
   `backzone` gives one, else the name its `#=` comment in `backward`
   gives, else its `backward` target.
   - `Asia/Calcutta`: `Link Asia/Kolkata Asia/Calcutta`, and
     `Asia/Kolkata` has a row. The answer's first column is
     `Asia/Kolkata`.
   - `Iceland`: `Link Africa/Abidjan Iceland #= Atlantic/Reykjavik`, and
     `zone.tab` has `IS +6409-02151 Atlantic/Reykjavik`, so the answer is
     Reykjavík, not Abidjan.
   - `America/Coral_Harbour`: `backward` has `Link America/Panama
     America/Coral_Harbour`, but `backzone` has `#PACKRATLIST zone.tab
     Link America/Atikokan America/Coral_Harbour`, and `zone.tab` places
     `America/Atikokan` in Canada, at `+484531-0913718`. The answer stays
     in Canada.
4. Anything else places nothing: `UTC` is `Link Etc/UTC UTC`, and
   `Etc/UTC` has no row. The export refuses it with `HC_ERR_UNKNOWN`. So
   does `Asia/Hanoi`, the one `Zone` of `backzone` that neither `zone.tab`
   nor `backward` names, since only `backzone`'s `Link` lines are read;
   and so do `backward`'s four `Zone` lines, `EST5EDT` and the like.

### A city

The exemplar city of a row, in a locale:

1. The first table in the locale's fallback chain with a line for the
   zone: `ja-JP` finds `ja`, whose `Asia/Tokyo` is 東京, answered by `ja`.
2. Where that line is CLDR's inheritance marker `↑↑↑`, the root name
   under the locale's tag: German's `Europe/Berlin` is `↑↑↑`, so the
   answer is `Berlin`, answered by `de`; German's `Antarctica/Rothera` is
   also `↑↑↑`, so the answer is `Rothera`, not English's `Rothera Station`.
3. Where no table has a line, English, answered by `en`: `en.xml`'s value
   (`Ho Chi Minh City` for `Asia/Saigon`), else `root.xml`'s (`Kolkata` for
   `Asia/Calcutta`), else the name derived from the zone's identifier
   (`America/Los_Angeles` is `Los Angeles`).

The root name of step 2 is `root.xml`'s value, else the derived name.

## What is carried

- **The tables.** `zone1970.tab`, `zone.tab`, `backward` and `backzone`
  of release 2026d, unmodified, in `crates/hc-tz/data/`. From them,
  `crates/hc-tz/src/location/tables.rs` is generated: the 312 rows of
  `zone1970.tab`; the 106 rows of `zone.tab` for names `zone1970.tab`
  does not list, all links; for each of those 418 rows, the country
  `zone.tab` lists its name under; and 131 of `backward`'s 252 links, each
  with the name with a row it leads to. The other 121 are the 106 with rows
  of their own and 15 that lead only to `Etc/UTC` or `Etc/GMT`. Only
  `backward`'s `Link` lines are read; its four `Zone` lines, `EST5EDT`,
  `CST6CDT`, `MST7MDT` and `PST8PDT`, are rules, not places. `zone.tab`'s
  rows for the zones `zone1970.tab` lists are left out: their coordinates
  are the same in both files, which a test checks. Only `backzone`'s 16
  `Link` lines are read, and they change the answer for five names:
  `Africa/Timbuktu`, `America/Coral_Harbour`, `Antarctica/South_Pole`,
  `Atlantic/Jan_Mayen` and `Pacific/Yap`.
- **The coordinates** as whole arcseconds, exact. The lines write them in
  decimal degrees to six places, by integer arithmetic:
  `arcseconds × 10⁶ / 3600` millionths of a degree, rounded half away from
  zero. That is `arcseconds × 2500 / 9`, whose fraction is a ninth, so no
  tie arises. Tokyo's 128 356″ and 503 081″ are `35.654444` and
  `139.744722`, and São Paulo's −84 720″ is `-23.533333`. One arcsecond is
  0.000278°, and six places are within 0.0018″ of it, so a caller who
  needs the exact arcseconds multiplies by 3600 and rounds; a test does so
  for every row, and for every angle from −180° to 180°. `hc-tz` also
  gives each as an `f64`, `arcseconds / 3600`.
- **The exemplar cities** of the 418 names with a row, from CLDR 48 at the
  `approved` and `contributed` levels: English (`en.xml`'s 33 values and
  `root.xml`'s 72) always, with the `tz` feature; and with
  `localized-exemplar-cities`, 41 more locales, every locale `hc-i18n`
  carries whose file has at least one such value. Tibetan, Sanskrit,
  Standard Moroccan Tamazight and Punjabi in the Arabic script have none,
  and every Coptic and Kabyle value is unconfirmed. European Portuguese
  keeps the 83 names `pt_PT.xml` gives of its own and inherits the rest
  from `pt`, its parent, rather than from root.
- **Not carried.** The zones' rules, beyond `hc-tz`'s seventeen built-in
  zones: `hc_zone_location` says where a zone is, not what its clocks
  read. The `provisional` and `unconfirmed` exemplar cities. The
  exemplar cities of the regional and added locales of `docs/i18n.md`
  (`en-GB`, `mn`, `shi-Latn` and the rest), which reach their parents'
  or English's. The metazone names (`Japan Standard Time`) and CLDR's
  other time zone names and formats are carried with the pattern fields
  that write them: [zone-names.md](zone-names.md). Any location for `Etc/` zones, `UTC` and the like.

## Accuracy

- **The coordinates are the table's, exactly**, to the precision the table
  gives: a whole arcminute (about 1.9 km of latitude) for the 11-character
  form and a whole arcsecond (about 31 m) for the 15-character one; 47 of
  `zone1970.tab`'s 312 rows use seconds. The tests work five rows by hand:
  Tokyo, Andorra, Troll, New York and São Paulo.
- **The place is the zone's, not the reader's.** A zone's principal
  location can be far from a reader in it. `zone1970.tab` places
  `Asia/Shanghai`, "Beijing Time", at Shanghai, 121° 28′ E, and China's
  other row, `Asia/Urumqi`, at 87° 35′ E, 33° 53′ further west. The Sun
  crosses 360° in 24 hours, four minutes a degree, so a reader that far
  west on Beijing Time would see the Sun rise some 2 h 15 min later than
  the zone's location does, before latitude is counted. A page should use
  the location as a default until the reader gives a place.
- **Old names.** A name resolved through `backward` answers with the place
  of the zone it leads to. `backward` crosses a border for a few old
  names, and `backzone` brings five of them back into their country.
  `Pacific/Johnston`, the Johnston Atoll, is left as `backward` has it,
  with Honolulu: `backzone` has a zone for it, not a link. The names of
  no place at all that `backward` links, `EST`, `CET` and the like, answer
  with the zone it links them to: `EST` with Panama, `CET` with Brussels.
  `EST5EDT`, `CST6CDT`, `MST7MDT` and `PST8PDT` are zones of `backward`'s
  own, not links, and answer with nothing.
- **The cities are CLDR's as published.** `America/Coyhaique`, a zone of
  2025, has no German value, and falls back to English.

## Sources

- [iana-tzdb-2026d] — `zone1970.tab`, `zone.tab`, `backward` and
  `backzone`, their header comments for the formats and the precedence
  of `backzone`, and `NEWS` for the release date. Read.
- [cldr48-exemplar-cities] — `common/main/<locale>.xml` for the 36
  carried locales with a file and `root.xml`, and [cldr48-most-spoken]
  for the twelve added for the most-spoken languages, `dates/timeZoneNames/zone/exemplarCity`,
  and `common/bcp47/timezone.xml` for CLDR's zone identifiers. Read.
- [uts35-dates-48] — the fallback to the last field of the zone's
  identifier. Read.
- [ecma402-2027] — §6.5, which link names a browser reports, and how
  it resolves the others. Read.
- ISO 6709, *Standard representation of geographic point location by
  coordinates*: the format `zone1970.tab` names. Not read; the format is
  taken from the header of `zone1970.tab`, which states it in full.

## Code

- `crates/hc-tz/src/location.rs`: `Coordinates::parse_iso6709`,
  `location`, `link_target`, `zones`, `rows`; the tests
  `coordinates_are_read_as_degrees_minutes_and_seconds`,
  `a_link_with_a_row_of_its_own_answers_with_that_row`,
  `a_link_is_answered_by_the_row_it_leads_to`,
  `the_zone_tab_country_is_the_one_zone_tab_lists_the_name_under` and
  `the_generated_tables_are_the_vendored_files`.
- `crates/hc-tz/src/location/tables.rs`: the generated rows and links.
- `crates/hc-i18n/src/exemplar_cities.rs`: the tables and the lookup;
  the tests `english_is_en_xml_then_root_xml_then_the_derived_name` and
  `a_locale_names_the_city_and_its_marker_inherits_from_root`.
- `crates/hyper-calendar/src/zone_lines.rs`: the lines of `hc_zones` and
  `hc_zone_location`, and the test that holds the two lists of names to
  each other.
