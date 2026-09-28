// The JavaScript binding of hyper-calendar-wasm.
//
// The module exports plain functions over integers and linear memory, and
// this file is the marshalling a page would otherwise write by hand: UTF-8
// in and out of `hc_alloc`/`hc_free` blocks, the measure-then-read protocol
// of the exports that write lines, the sentinels turned into thrown errors,
// the `i64` values crossing as `BigInt`, and one typed decoder for every
// line format the crate's README states. It has no dependencies and no
// build step; it is an ES module the page imports as it is.
//
// The column order of every decoder is the README's, and the tests beside
// this file hold the two to each other, so a column added in the wrong
// place fails the build rather than a page.

/** Any `i64` at or below this is an error sentinel, not a result. */
const HC_ERR_FLOOR = -9_000_000_000_000_000n;

/**
 * The sentinels the module returns, in the README's order, each with the
 * name this binding throws it under.
 *
 * @type {ReadonlyArray<{code: bigint, constant: string, name: string, message: string}>}
 */
export const SENTINELS = Object.freeze([
  { code: -9_000_000_000_000_001n, constant: "HC_ERR_INVALID_DATE", name: "invalid-date", message: "The date does not exist." },
  { code: -9_000_000_000_000_002n, constant: "HC_ERR_OUT_OF_RANGE", name: "out-of-range", message: "A value was outside the supported range." },
  { code: -9_000_000_000_000_003n, constant: "HC_ERR_BUFFER_TOO_SMALL", name: "buffer-too-small", message: "The supplied buffer was too small." },
  { code: -9_000_000_000_000_004n, constant: "HC_ERR_NO_DATA", name: "no-data", message: "The requested model has no data for this value." },
  { code: -9_000_000_000_000_005n, constant: "HC_ERR_NULL_POINTER", name: "null-pointer", message: "A pointer was null with a non-zero length." },
  { code: -9_000_000_000_000_006n, constant: "HC_ERR_UNKNOWN", name: "unknown", message: "The requested table or identifier is not known." },
  { code: -9_000_000_000_000_007n, constant: "HC_ERR_NOT_UTF8", name: "not-utf8", message: "Text was not valid UTF-8." },
  { code: -9_000_000_000_000_008n, constant: "HC_ERR_MALFORMED", name: "malformed", message: "Data was not in the format the call expects." },
].map(Object.freeze));

const HC_ERR_BUFFER_TOO_SMALL = -9_000_000_000_000_003n;

/**
 * Every method of {@link HyperCalendar} that stands for one export, with the
 * export's name and the Cargo feature the module has to be built with for
 * the export to exist (`null` for the three in every build).
 *
 * @type {ReadonlyArray<{method: string, export: string, feature: string | null}>}
 */
export const METHODS = Object.freeze([
  { method: "alloc", export: "hc_alloc", feature: null },
  { method: "free", export: "hc_free", feature: null },
  { method: "version", export: "hc_version", feature: null },
  { method: "gregorianToFixed", export: "hc_gregorian_to_fixed", feature: "civil" },
  { method: "gregorianYear", export: "hc_gregorian_year", feature: "civil" },
  { method: "gregorianMonth", export: "hc_gregorian_month", feature: "civil" },
  { method: "gregorianDay", export: "hc_gregorian_day", feature: "civil" },
  { method: "weekday", export: "hc_weekday", feature: "civil" },
  { method: "dayOfYear", export: "hc_day_of_year", feature: "civil" },
  { method: "isLeapYear", export: "hc_is_leap_year", feature: "civil" },
  { method: "fixedFromUnix", export: "hc_fixed_from_unix", feature: "civil" },
  { method: "taiMinusUtc", export: "hc_tai_minus_utc", feature: "civil" },
  { method: "dayHasLeapSecond", export: "hc_day_has_leap_second", feature: "civil" },
  { method: "unixFromFixed", export: "hc_unix_from_fixed", feature: "civil" },
  { method: "formatIsoDate", export: "hc_format_iso_date", feature: "civil" },
  { method: "parseIsoDate", export: "hc_parse_iso_date", feature: "civil" },
  { method: "tai64Encode", export: "hc_tai64_encode", feature: "timestamps" },
  { method: "tai64Decode", export: "hc_tai64_decode", feature: "timestamps" },
  { method: "gnssWeek", export: "hc_gnss_week", feature: "timestamps" },
  { method: "gnssToTai", export: "hc_gnss_to_tai", feature: "timestamps" },
  { method: "gnssResolveWeek", export: "hc_gnss_resolve_week", feature: "timestamps" },
  { method: "glonassDate", export: "hc_glonass_date", feature: "timestamps" },
  { method: "fixedFromOleAutomation", export: "hc_fixed_from_ole_automation", feature: "timestamps" },
  { method: "oleAutomationFromFixed", export: "hc_ole_automation_from_fixed", feature: "timestamps" },
  { method: "excel1900Day", export: "hc_excel_1900_day", feature: "timestamps" },
  { method: "taiFromUnix", export: "hc_tai_from_unix", feature: "timestamps" },
  { method: "utcFromTai", export: "hc_utc_from_tai", feature: "timestamps" },
  { method: "tai64PosixPlus10Encode", export: "hc_tai64_posix_plus_10_encode", feature: "timestamps" },
  { method: "tai64PosixPlus10Decode", export: "hc_tai64_posix_plus_10_decode", feature: "timestamps" },
  { method: "uuidTimestamp", export: "hc_uuid_timestamp", feature: "timestamps" },
  { method: "ntpResolve", export: "hc_ntp_resolve", feature: "timestamps" },
  { method: "uuidTimestampEncode", export: "hc_uuid_timestamp_encode", feature: "timestamps" },
  { method: "ntpEncode", export: "hc_ntp_encode", feature: "timestamps" },
  { method: "fatDecode", export: "hc_fat_decode", feature: "timestamps" },
  { method: "fatEncode", export: "hc_fat_encode", feature: "timestamps" },
  { method: "swatchBeat", export: "hc_swatch_beat", feature: "timestamps" },
  { method: "epochFromTt", export: "hc_epoch_from_tt", feature: "timestamps" },
  { method: "ttFromEpoch", export: "hc_tt_from_epoch", feature: "timestamps" },
  { method: "ttBipm", export: "hc_tt_bipm", feature: "timestamps" },
  { method: "ccsdsDecode", export: "hc_ccsds_decode", feature: "time-codes" },
  { method: "ccsdsEncode", export: "hc_ccsds_encode", feature: "time-codes" },
  { method: "ccsdsAsciiParse", export: "hc_ccsds_ascii_parse", feature: "time-codes" },
  { method: "ccsdsAsciiFormat", export: "hc_ccsds_ascii_format", feature: "time-codes" },
  { method: "radioDecode", export: "hc_radio_decode", feature: "time-codes" },
  { method: "radioEncode", export: "hc_radio_encode", feature: "time-codes" },
  { method: "irigDecode", export: "hc_irig_decode", feature: "time-codes" },
  { method: "irigEncode", export: "hc_irig_encode", feature: "time-codes" },
  { method: "irigFormats", export: "hc_irig_formats", feature: "time-codes" },
  { method: "dotnetTicksFromUnix", export: "hc_dotnet_ticks_from_unix", feature: "timestamps" },
  { method: "unixFromDotnetTicks", export: "hc_unix_from_dotnet_ticks", feature: "timestamps" },
  { method: "sixHourClock", export: "hc_six_hour_clock", feature: "timestamps" },
  { method: "civilFromSixHourClock", export: "hc_civil_from_six_hour_clock", feature: "timestamps" },
  { method: "describeDay", export: "hc_describe_day", feature: "calendars" },
  { method: "dayExtras", export: "hc_day_extras", feature: "calendars" },
  { method: "calendarUnits", export: "hc_calendar_units", feature: "calendars" },
  { method: "parseDate", export: "hc_parse_date", feature: "calendars" },
  { method: "calendars", export: "hc_calendars", feature: "calendars" },
  { method: "calendarList", export: "hc_calendar_list", feature: "calendars" },
  { method: "locales", export: "hc_locales", feature: "calendars" },
  { method: "firstDayOfWeek", export: "hc_first_day_of_week", feature: "calendars" },
  { method: "gregorianAdoption", export: "hc_gregorian_adoption", feature: "calendars" },
  { method: "namingPeriodOn", export: "hc_naming_period_on", feature: "calendars" },
  { method: "panchangaAt", export: "hc_panchanga_at", feature: "calendars" },
  { method: "panchangaOfDay", export: "hc_panchanga_of_day", feature: "calendars" },
  { method: "hinduLunarDate", export: "hc_hindu_lunar_date", feature: "calendars" },
  { method: "suryaSiddhantaAt", export: "hc_surya_siddhanta_at", feature: "calendars" },
  { method: "suryaSiddhantaSunrise", export: "hc_surya_siddhanta_sunrise", feature: "calendars" },
  { method: "barhaspatyaYear", export: "hc_barhaspatya_year", feature: "calendars" },
  { method: "barhaspatyaYearAt", export: "hc_barhaspatya_year_at", feature: "calendars" },
  { method: "crescentVisible", export: "hc_crescent_visible", feature: "calendars" },
  { method: "iocOlympiad", export: "hc_ioc_olympiad", feature: "calendars" },
  { method: "hebrewYahrzeit", export: "hc_hebrew_yahrzeit", feature: "calendars" },
  { method: "hebrewBirthday", export: "hc_hebrew_birthday", feature: "calendars" },
  { method: "chineseReckonedAge", export: "hc_chinese_reckoned_age", feature: "calendars" },
  { method: "chineseMarriageAugury", export: "hc_chinese_marriage_augury", feature: "calendars" },
  { method: "hebrewSabbaticalCycleYear", export: "hc_hebrew_sabbatical_cycle_year", feature: "calendars" },
  { method: "asianDay", export: "hc_asian_day", feature: "calendars" },
  { method: "kalam", export: "hc_kalam", feature: "calendars" },
  { method: "almanacCycles", export: "hc_almanac_cycles", feature: "calendars" },
  { method: "almanacDay", export: "hc_almanac_day", feature: "calendars" },
  { method: "choghadiya", export: "hc_choghadiya", feature: "calendars" },
  { method: "panchak", export: "hc_panchak", feature: "calendars" },
  { method: "kumbh", export: "hc_kumbh", feature: "calendars" },
  { method: "pushkaram", export: "hc_pushkaram", feature: "calendars" },
  { method: "folkDay", export: "hc_folk_day", feature: "calendars" },
  { method: "nightWatch", export: "hc_night_watch", feature: "calendars" },
  { method: "holidayIsDayOff", export: "hc_holiday_is_day_off", feature: "holiday" },
  { method: "holidaysInYear", export: "hc_holidays_in_year", feature: "holiday" },
  { method: "holidayCodes", export: "hc_holiday_codes", feature: "holiday" },
  { method: "holidaysOn", export: "hc_holidays_on", feature: "holiday" },
  { method: "holidayTables", export: "hc_holiday_tables", feature: "holiday" },
  { method: "lectionary", export: "hc_lectionary", feature: "holiday" },
  { method: "astronomicalEaster", export: "hc_astronomical_easter", feature: "holiday" },
  { method: "astronomicalPaschalFullMoon", export: "hc_astronomical_paschal_full_moon", feature: "holiday" },
  { method: "holyYearOn", export: "hc_holy_year_on", feature: "holiday" },
  { method: "commonWorshipOn", export: "hc_common_worship_on", feature: "holiday" },
  { method: "orthodoxFastOn", export: "hc_orthodox_fast_on", feature: "holiday" },
  { method: "orthodoxFastSeasons", export: "hc_orthodox_fast_seasons", feature: "holiday" },
  { method: "termInEffect", export: "hc_term_in_effect", feature: "seasons" },
  { method: "pentadInEffect", export: "hc_pentad_in_effect", feature: "seasons" },
  { method: "coldFoodDay", export: "hc_cold_food_day", feature: "seasons" },
  { method: "plumRains", export: "hc_plum_rains", feature: "seasons" },
  { method: "placeYearsAgo", export: "hc_place_years_ago", feature: "deep-time" },
  { method: "cosmicEvents", export: "hc_cosmic_events", feature: "deep-time" },
  { method: "earliestEvidence", export: "hc_earliest_evidence", feature: "deep-time" },
  { method: "archaeologicalPeriods", export: "hc_archaeological_periods", feature: "deep-time" },
  { method: "futureEvents", export: "hc_future_events", feature: "deep-time" },
  { method: "geologicIntervals", export: "hc_geologic_intervals", feature: "deep-time" },
  { method: "fixedFromUnixInZone", export: "hc_fixed_from_unix_in_zone", feature: "tz" },
  { method: "unixFromFixedInZone", export: "hc_unix_from_fixed_in_zone", feature: "tz" },
  { method: "loadZone", export: "hc_zone_load", feature: "tz" },
  { method: "zones", export: "hc_zones", feature: "tz" },
  { method: "zoneLocation", export: "hc_zone_location", feature: "tz" },
  { method: "zoneOffset", export: "hc_zone_offset", feature: "tz" },
  { method: "skyAt", export: "hc_sky_at", feature: "sky" },
  { method: "solarTermsBetween", export: "hc_solar_terms_between", feature: "sky" },
  { method: "moonPhasesBetween", export: "hc_moon_phases_between", feature: "sky" },
  { method: "decanAt", export: "hc_decan_at", feature: "sky" },
  { method: "earthRotationAngle", export: "hc_earth_rotation_angle", feature: "sky" },
  { method: "gmstIau2006", export: "hc_gmst_iau2006", feature: "sky" },
  { method: "gmstIau1982", export: "hc_gmst_iau1982", feature: "sky" },
  { method: "ut2MinusUt1", export: "hc_ut2_minus_ut1", feature: "sky" },
  { method: "solarTime", export: "hc_solar_time", feature: "sky" },
  { method: "solarEvent", export: "hc_solar_event", feature: "sky" },
  { method: "horizons", export: "hc_horizons", feature: "sky" },
  { method: "sunrise", export: "hc_sunrise", feature: "sky" },
  { method: "sunset", export: "hc_sunset", feature: "sky" },
  { method: "hjdTt", export: "hc_hjd_tt", feature: "sky" },
  { method: "hjdUtc", export: "hc_hjd_utc", feature: "sky" },
  { method: "gmatFromGmt", export: "hc_gmat_from_gmt", feature: "sky" },
  { method: "gmtFromGmat", export: "hc_gmt_from_gmat", feature: "sky" },
  { method: "prayerTimes", export: "hc_prayer_times", feature: "sky" },
  { method: "prayerMethods", export: "hc_prayer_methods", feature: "sky" },
  { method: "zmanim", export: "hc_zmanim", feature: "sky" },
  { method: "edoTime", export: "hc_edo_time", feature: "sky" },
  { method: "unixFromEdoTime", export: "hc_unix_from_edo_time", feature: "sky" },
  { method: "planetaryHour", export: "hc_planetary_hour", feature: "sky" },
  { method: "planetaryHoursOfDay", export: "hc_planetary_hours_of_day", feature: "sky" },
  { method: "orbitAt", export: "hc_orbit_at", feature: "orbital" },
  { method: "orbitSeries", export: "hc_orbit_series", feature: "orbital" },
  { method: "marsTime", export: "hc_mars_time", feature: "planetary" },
  { method: "missions", export: "hc_missions", feature: "planetary" },
  { method: "missionSol", export: "hc_mission_sol", feature: "planetary" },
  { method: "bodies", export: "hc_bodies", feature: "planetary" },
  { method: "bodyTime", export: "hc_body_time", feature: "planetary" },
  { method: "circadDate", export: "hc_circad_date", feature: "planetary" },
  { method: "properTime", export: "hc_proper_time", feature: "relativity" },
  { method: "gravitationalDilation", export: "hc_gravitational_dilation", feature: "relativity" },
  { method: "gravitatingBodies", export: "hc_gravitating_bodies", feature: "relativity" },
].map(Object.freeze));

/** The columns of `hc_describe_day`, which `hc_parse_date` writes before the fixed day. */
const DESCRIBE_DAY_COLUMNS = Object.freeze([
  "id", "name", "era", "era label", "year", "month", "leap month", "month label",
  "day", "leap day", "extras", "error code", "error name", "standing",
  "day boundary", "formatted", "locale used", "day named by",
]);

/** The columns of `hc_orbit_at`, which `hc_orbit_series` writes after the epoch. */
const ORBIT_COLUMNS = Object.freeze([
  "eccentricity", "eccentricity spread", "obliquity", "obliquity spread",
  "longitude of perihelion", "longitude of perihelion spread",
  "climatic precession", "climatic precession spread",
  "insolation 65°N June", "solar constant", "source",
]);

/**
 * The column names of every line format, in the README's order. A decoder
 * reads its cells by these positions, and the tests hold each list to the
 * README's table.
 *
 * @type {Readonly<Record<string, ReadonlyArray<string>>>}
 */
export const COLUMNS = Object.freeze({
  describeDay: DESCRIBE_DAY_COLUMNS,
  parseDate: Object.freeze([...DESCRIBE_DAY_COLUMNS, "fixed"]),
  dayExtras: Object.freeze([
    "id", "field", "value", "label", "value label", "in date", "locale used",
  ]),
  calendarUnits: Object.freeze([
    "start", "end", "label", "leap", "standing", "error code", "error name", "locale used",
  ]),
  calendars: Object.freeze([
    "id", "name", "english name", "earliest", "latest", "has era", "has year", "has month",
    "has day", "native locales", "standing",
  ]),
  calendarList: Object.freeze([
    "id", "name", "english name", "locale used", "crate", "native locales",
  ]),
  locales: Object.freeze([
    "tag", "english name", "native name", "gregorian months", "weekdays", "gregorian eras",
    "calendars",
  ]),
  gregorianAdoption: Object.freeze([
    "last old day", "first day", "old calendar", "scope", "source", "new calendar", "polity",
  ]),
  holidaysInYear: Object.freeze([
    "date", "name", "local name", "kind", "confidence", "substitute", "observed for",
  ]),
  holidaysOn: Object.freeze([
    "table", "table name", "name", "local name", "kind", "confidence", "source",
    "substitute", "observed for",
  ]),
  term: Object.freeze([
    "index", "chinese name", "japanese name", "begins", "ends",
    "chinese authority", "japanese authority",
  ]),
  deepTime: Object.freeze([
    "kind", "id", "name", "scope", "start", "start σ", "start figures", "start approximate",
    "end", "end σ", "end figures", "end approximate", "unit", "description", "source",
    "localised name",
  ]),
  sky: Object.freeze([
    "sun longitude", "sun distance", "moon longitude", "moon latitude", "moon distance",
    "elongation", "illuminated fraction", "previous new moon", "next new moon",
    "ΔT", "ΔT regime", "source",
  ]),
  skyEvent: Object.freeze(["angle", "instant", "name", "japanese name"]),
  orbit: ORBIT_COLUMNS,
  orbitSeries: Object.freeze(["years before 1950", ...ORBIT_COLUMNS]),
  tai64: Object.freeze(["format", "tai seconds", "attoseconds"]),
  gnssWeek: Object.freeze(["week", "broadcast week", "tow seconds", "tow attoseconds"]),
  taiInstant: Object.freeze(["tai seconds", "attoseconds"]),
  glonassDate: Object.freeze(["four-year interval", "day"]),
  oleAutomation: Object.freeze(["fixed", "seconds of day"]),
  excel1900Day: Object.freeze(["fixed", "phantom"]),
  panchanga: Object.freeze([
    "limb", "number", "name", "devanagari", "began", "ends", "read at", "ayanamsa",
    "ayanamsa name",
  ]),
  marriageAugury: Object.freeze(["augury", "lichun at start", "lichun at end"]),
  holidayTables: Object.freeze([
    "code", "kind", "name", "english name", "locale used", "source", "country", "short name",
  ]),
  lectionary: Object.freeze(["liturgical year", "sunday cycle", "weekday cycle", "proper"]),
  zones: Object.freeze([
    "zone", "latitude", "longitude", "countries", "country", "comment", "exemplar city",
    "locale used",
  ]),
  zoneOffset: Object.freeze([
    "offset", "dst", "abbreviation", "next transition", "next offset", "rules",
  ]),
  value: Object.freeze(["value"]),
  solarTime: Object.freeze(["day", "hours", "missing", "missing day", "depression"]),
  solarEvent: Object.freeze(["instant", "missing", "missing day", "depression", "depression arcseconds"]),
  marsTime: Object.freeze([
    "mars sol date", "mtc", "mtc hours", "lmst", "lmst hours", "ltst", "ltst hours",
    "equation of time", "ls", "mars year", "darian year", "darian month", "darian sol",
    "darian month name", "darian sol of week", "source",
  ]),
  missions: Object.freeze([
    "id", "name", "landing utc", "landing unix", "landing sol", "clock", "clock longitude",
    "site longitude", "published", "note", "source",
  ]),
  bodies: Object.freeze([
    "id", "name", "kind", "primary", "sidereal rotation", "solar day", "solar day origin",
    "year in local days", "zero point", "zero point note", "source", "status",
  ]),
  bodyTime: Object.freeze([
    "day", "fraction", "time", "hours", "solar day", "local hour", "zero point", "zero point note",
  ]),
  properTime: Object.freeze([
    "beta", "lorentz factor", "proper seconds", "rate", "microseconds per day", "constants", "source",
  ]),
  gravitationalDilation: Object.freeze([
    "id", "gm", "gm constant", "schwarzschild radius", "factor", "microseconds per day",
    "constants", "source",
  ]),
  gravitatingBodies: Object.freeze(["id", "name", "gm", "gm constant", "source"]),
  utcFromTai: Object.freeze(["unix seconds", "leap second"]),
  tai64PosixPlus10: Object.freeze(["format", "unix seconds", "attoseconds"]),
  uuidTimestamp: Object.freeze(["version", "timestamp", "unix seconds", "attoseconds"]),
  ntpResolve: Object.freeze(["era", "era offset", "fraction", "unix seconds", "attoseconds"]),
  uuidTimestampEncode: Object.freeze(["timestamp", "version 1 fields", "version 6 fields"]),
  ntpEncode: Object.freeze(["era", "era offset", "fraction", "date", "timestamp"]),
  fatDecode: Object.freeze(["fixed", "seconds of day"]),
  fatEncode: Object.freeze(["date word", "time word"]),
  epoch: Object.freeze(["notation", "epoch"]),
  ttFromEpoch: Object.freeze(["notation", "tt seconds", "attoseconds"]),
  circadDate: Object.freeze([
    "calendar", "year", "month", "day", "month name", "week name", "count", "fraction", "leap",
    "source",
  ]),
  horizons: Object.freeze([
    "id", "english name", "description", "source", "short name", "name", "locale used",
  ]),
  solarCrossing: Object.freeze(["instant", "missing", "missing day", "depression", "altitude"]),
  hinduLunarDate: Object.freeze([
    "saka year", "vikrama year", "month", "leap month", "tithi", "leap day", "sunrise",
    "month name", "leap month word", "saka era", "vikrama era", "locale used",
  ]),
  suryaSiddhanta: Object.freeze(["sun", "moon", "elongation", "tithi", "sign"]),
  crescent: Object.freeze([
    "visible", "evaluated at", "elongation", "arc of light", "altitude", "arc of vision", "width",
  ]),
  decan: Object.freeze(["sign", "sign name", "decan", "ruler", "ruler name", "degrees into decan"]),
  hjdTt: Object.freeze(["hjd", "correction"]),
  hjdUtc: Object.freeze(["hjd", "correction", "tt minus utc"]),
  ttBipm: Object.freeze([
    "offset", "minus tai seconds", "minus tai attoseconds", "reading seconds", "reading attoseconds",
  ]),
  namingPeriod: Object.freeze([
    "state", "period", "month", "weekday", "weekday meaning", "earliest", "in force by", "ended",
    "source",
  ]),
  asianDay: Object.freeze(["year", "month", "month name", "written", "number"]),
  holyYear: Object.freeze([
    "state", "title", "kind", "pope", "bull", "given", "opens", "closes", "dioceses open",
    "dioceses close",
  ]),
  commonWorship: Object.freeze(["title", "rank", "rank name"]),
  ccsdsDecode: Object.freeze([
    "code", "tai seconds", "tai attoseconds", "unix seconds", "leap second", "utc attoseconds",
  ]),
  ccsdsAscii: Object.freeze([
    "variation", "tai seconds", "tai attoseconds", "unix seconds", "leap second", "utc attoseconds",
    "precision", "digits", "terminator",
  ]),
  radioDecode: Object.freeze([
    "unix seconds", "fixed", "hour", "minute", "offset hours", "seconds", "leap", "summer",
    "zone change", "dut1 tenths", "dst next",
  ]),
  unixFromDotnetTicks: Object.freeze(["unix seconds", "attoseconds"]),
  sixHourClock: Object.freeze(["hour", "minute", "second", "half", "period", "period english"]),
  kalam: Object.freeze([
    "id", "english name", "name", "locale used", "part", "clock", "start", "end", "missing",
    "missing day", "depression", "depression arcseconds",
  ]),
  almanacCycles: Object.freeze([
    "eho", "eho romaji", "azimuth", "sixteen-point", "direction", "period", "period name", "era",
    "star", "ruler", "first year", "last year", "without son",
  ]),
  almanacDay: Object.freeze([
    "kind", "id", "name", "locale used", "japanese", "reading", "auspicious", "printed",
  ]),
  orthodoxFast: Object.freeze(["fast day", "status", "period", "period name", "kind", "abstinence"]),
  orthodoxFastSeasons: Object.freeze(["id", "english name", "kind", "first", "last"]),
  prayerTimes: Object.freeze([
    "time", "instant", "missing", "missing day", "depression", "depression arcseconds",
  ]),
  prayerMethods: Object.freeze([
    "id", "english name", "fajr", "maghrib", "isha", "isha minutes", "isha ramadan minutes",
    "midnight", "source",
  ]),
  zmanim: Object.freeze([
    "id", "english name", "hours", "instant", "missing", "missing day", "depression",
    "depression arcseconds",
  ]),
  edoTime: Object.freeze([
    "day", "hour", "name", "romaji", "strokes", "branch", "tenths", "fraction", "missing",
    "missing day", "depression", "depression arcseconds",
  ]),
  choghadiya: Object.freeze([
    "half", "part", "id", "name", "locale used", "quality", "ruler", "start", "end", "missing",
    "missing day", "depression", "depression arcseconds",
  ]),
  panchak: Object.freeze(["within", "opens", "closes", "weekday", "kind", "name", "locale used"]),
  kumbh: Object.freeze([
    "id", "site", "site name", "locale used", "river", "jupiter", "sun", "at new moon", "from", "to",
    "holds",
  ]),
  pushkaram: Object.freeze([
    "id", "name", "locale used", "region", "sign", "first", "last", "missing", "missing day",
    "depression", "depression arcseconds",
  ]),
  folkDay: Object.freeze(["kind", "id", "name", "locale used", "count"]),
  nightWatch: Object.freeze(["watch", "points", "name", "locale used", "han name", "branch"]),
  barhaspatyaYear: Object.freeze(["position", "name", "expunged", "expunged name", "locale used"]),
  barhaspatyaYearAt: Object.freeze(["position", "name", "locale used"]),
  planetaryHour: Object.freeze([
    "day", "hour", "ruler", "name", "locale used", "daytime", "start", "end", "missing",
    "missing day", "depression", "depression arcseconds",
  ]),
  gmat: Object.freeze(["fixed", "seconds of day", "attoseconds"]),
  irigDecode: Object.freeze([
    "fixed", "day of year", "hour", "minute", "second", "hundredths", "year", "control",
    "straight binary seconds",
  ]),
  irigFormats: Object.freeze([
    "format", "index count microseconds", "index counts", "frame microseconds", "fields",
    "control bits", "modulations", "carriers", "expressions",
  ]),
});

/** The geologic ranks `hc_geologic_intervals` numbers, coarsest first. */
export const GEOLOGIC_RANKS = Object.freeze(["eon", "era", "period", "epoch", "age"]);

/** The units `hc_calendar_units` numbers, largest first. */
export const UNITS = Object.freeze(["era", "year", "month", "day"]);

/** The locale that asks for each calendar's own language. */
export const NATIVE = "native";

/**
 * What the module refused, or what the binding could not do.
 *
 * `name` is the sentinel's name (`invalid-date`, `out-of-range`,
 * `buffer-too-small`, `no-data`, `null-pointer`, `unknown`, `not-utf8`,
 * `malformed`), or one of the binding's own: `not-exported` for a method
 * whose export is not in this build, `unsafe-integer` for an `i64` that
 * does not fit a JavaScript number, whether a result or an integer argument
 * given as a number past `Number.MAX_SAFE_INTEGER`, `allocation-failed` when `hc_alloc`
 * returned null, and `unrecognised-sentinel` for a sentinel newer than this
 * file. `code` is the sentinel as a `BigInt`, or `null` for the binding's
 * own; `constant` its `HC_ERR_*` name, or `null`; `export` the export
 * concerned.
 */
export class HcError extends Error {
  /**
   * @param {string} name
   * @param {{code?: bigint | null, constant?: string | null, export?: string | null, message: string}} details
   */
  constructor(name, details) {
    super(details.message);
    this.name = name;
    this.code = details.code ?? null;
    this.constant = details.constant ?? null;
    this.export = details.export ?? null;
  }
}

const encoder = new TextEncoder();
const decoder = new TextDecoder("utf-8", { fatal: true });

/**
 * Throw for a sentinel; otherwise the value.
 *
 * @param {bigint} value
 * @param {string} exportName
 * @returns {bigint}
 */
function checked(value, exportName) {
  if (typeof value !== "bigint") {
    throw new TypeError(`${exportName} returned ${typeof value}, not a BigInt`);
  }
  if (value > HC_ERR_FLOOR) {
    return value;
  }
  const sentinel = SENTINELS.find((candidate) => candidate.code === value);
  if (sentinel === undefined) {
    throw new HcError("unrecognised-sentinel", {
      code: value,
      export: exportName,
      message: `${exportName} returned the sentinel ${value}, which this binding does not know`,
    });
  }
  throw new HcError(sentinel.name, {
    code: sentinel.code,
    constant: sentinel.constant,
    export: exportName,
    message: `${exportName}: ${sentinel.constant}: ${sentinel.message}`,
  });
}

/**
 * An `i64` result as a number, which every calendar value fits; one that
 * does not is `unsafe-integer` rather than a rounded number.
 *
 * @param {bigint} value
 * @param {string} exportName
 * @returns {number}
 */
function toNumber(value, exportName) {
  const result = checked(value, exportName);
  if (result > BigInt(Number.MAX_SAFE_INTEGER) || result < -BigInt(Number.MAX_SAFE_INTEGER)) {
    throw new HcError("unsafe-integer", {
      export: exportName,
      message: `${exportName} returned ${result}, which a JavaScript number cannot hold exactly`,
    });
  }
  return Number(result);
}

/**
 * An integer argument given as a number that no longer holds it exactly:
 * past `Number.MAX_SAFE_INTEGER` the number is already rounded, so the
 * binding refuses it as `unsafe-integer`, the name it gives a result a
 * number cannot hold, rather than pass on a value the caller did not
 * write. A `BigInt` carries such a value exactly.
 *
 * @param {number} value
 * @param {string} what
 * @returns {never}
 */
function unsafeArgument(value, what) {
  throw new HcError("unsafe-integer", {
    message: `${what} is ${value}, past Number.MAX_SAFE_INTEGER, which a number does not hold exactly; pass a BigInt`,
  });
}

/**
 * An `i64` argument: a safe integer or a `BigInt`. An integer number past
 * the safe range is `unsafe-integer`.
 *
 * @param {number | bigint} value
 * @param {string} what
 * @returns {bigint}
 */
function toI64(value, what) {
  if (typeof value === "bigint") {
    return BigInt.asIntN(64, value) === value
      ? value
      : raise(`${what} must fit an i64, got ${value}`);
  }
  if (typeof value === "number" && Number.isSafeInteger(value)) {
    return BigInt(value);
  }
  if (typeof value === "number" && Number.isInteger(value)) {
    return unsafeArgument(value, what);
  }
  return raise(`${what} must be an integer, got ${String(value)}`);
}

/**
 * A `u32` argument.
 *
 * @param {number} value
 * @param {string} what
 * @returns {number}
 */
function toU32(value, what) {
  if (typeof value === "number" && Number.isInteger(value) && value >= 0 && value <= 0xffff_ffff) {
    return value;
  }
  return raise(`${what} must be an integer from 0 to 4294967295, got ${String(value)}`);
}

/**
 * An `i32` argument.
 *
 * @param {number} value
 * @param {string} what
 * @returns {number}
 */
function toI32(value, what) {
  if (typeof value === "number" && Number.isInteger(value) && value >= -0x8000_0000 && value <= 0x7fff_ffff) {
    return value;
  }
  return raise(`${what} must be an integer from -2147483648 to 2147483647, got ${String(value)}`);
}

/**
 * A `u64` argument: a safe non-negative integer or a `BigInt`. A positive
 * integer number past the safe range is `unsafe-integer`, as for `toI64`.
 *
 * @param {number | bigint} value
 * @param {string} what
 * @returns {bigint}
 */
function toU64(value, what) {
  if (typeof value === "bigint") {
    return BigInt.asUintN(64, value) === value
      ? value
      : raise(`${what} must fit a u64, got ${value}`);
  }
  if (typeof value === "number" && Number.isSafeInteger(value) && value >= 0) {
    return BigInt(value);
  }
  if (typeof value === "number" && Number.isInteger(value) && value > 0) {
    return unsafeArgument(value, what);
  }
  return raise(`${what} must be a non-negative integer, got ${String(value)}`);
}

/**
 * An `f64` argument.
 *
 * @param {number} value
 * @param {string} what
 * @returns {number}
 */
function toF64(value, what) {
  if (typeof value === "number") {
    return value;
  }
  return raise(`${what} must be a number, got ${typeof value}`);
}

/**
 * @param {string} message
 * @returns {never}
 */
function raise(message) {
  throw new TypeError(message);
}

/**
 * The lines of a text as arrays of cells.
 *
 * @param {string} text
 * @param {ReadonlyArray<string>} columns
 * @param {string} exportName
 * @returns {string[][]}
 */
function rows(text, columns, exportName) {
  const out = [];
  for (const line of text.split("\n")) {
    if (line.length === 0) {
      continue;
    }
    const cells = line.split("\t");
    // The columns are this file's version of the README's tables: cells
    // past them are ignored, and fewer are not the format this file reads.
    if (cells.length < columns.length) {
      throw new HcError("malformed", {
        export: exportName,
        message: `${exportName} wrote a line of ${cells.length} cells where ${columns.length} were expected: ${JSON.stringify(line)}`,
      });
    }
    out.push(cells);
  }
  return out;
}

/**
 * A cell that is empty when it has nothing to say.
 *
 * @param {string} cell
 * @returns {string | null}
 */
function optional(cell) {
  return cell === "" ? null : cell;
}

/**
 * A cell holding an integer written in plain decimal.
 *
 * @param {string} cell
 * @param {string} what
 * @returns {number}
 */
function integer(cell, what) {
  if (!/^-?\d+$/.test(cell)) {
    throw new HcError("malformed", { message: `${what} is not an integer: ${JSON.stringify(cell)}` });
  }
  const value = Number(cell);
  if (!Number.isSafeInteger(value)) {
    throw new HcError("unsafe-integer", { message: `${what} does not fit a JavaScript number: ${cell}` });
  }
  return value;
}

/**
 * @param {string} cell
 * @param {string} what
 * @returns {number | null}
 */
function optionalInteger(cell, what) {
  return cell === "" ? null : integer(cell, what);
}

/**
 * A cell holding an integer that need not fit a JavaScript number: a TAI
 * second, an attosecond.
 *
 * @param {string} cell
 * @param {string} what
 * @returns {bigint}
 */
function bigInteger(cell, what) {
  if (!/^-?\d+$/.test(cell)) {
    throw new HcError("malformed", { message: `${what} is not an integer: ${JSON.stringify(cell)}` });
  }
  return BigInt(cell);
}

/**
 * A cell holding a number in plain decimal notation, however large or small.
 *
 * @param {string} cell
 * @param {string} what
 * @returns {number}
 */
function decimal(cell, what) {
  if (!/^-?(\d+\.?\d*|\.\d+)$/.test(cell)) {
    throw new HcError("malformed", { message: `${what} is not a decimal number: ${JSON.stringify(cell)}` });
  }
  return Number(cell);
}

/**
 * A `0`/`1` cell.
 *
 * @param {string} cell
 * @param {string} what
 * @returns {boolean}
 */
function flag(cell, what) {
  if (cell === "1") {
    return true;
  }
  if (cell === "0") {
    return false;
  }
  throw new HcError("malformed", { message: `${what} is not 0 or 1: ${JSON.stringify(cell)}` });
}

/**
 * A `0`/`1` cell that is empty on a refusal, where there is no month or day
 * to be a leap one.
 *
 * @param {string} cell
 * @param {string} what
 * @returns {boolean}
 */
function optionalFlag(cell, what) {
  return cell === "" ? false : flag(cell, what);
}

/**
 * One row of `hc_day_extras`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").DayExtra}
 */
function dayExtra(cells) {
  const [id, field, value, label, valueLabel, inDate, localeUsed] = cells;
  return {
    id,
    field,
    value: integer(value, "value"),
    label,
    valueLabel,
    inDate: flag(inDate, "in date"),
    localeUsed,
  };
}

/**
 * One row of `hc_describe_day`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").DescribedDay}
 */
function describedDay(cells) {
  const [
    id, name, era, eraLabel, year, month, leapMonth, monthLabel, day, leapDay,
    extras, errorCode, errorName, standing, dayBoundary, formatted, localeUsed, dayNamedBy,
  ] = cells;
  /** @type {Record<string, string>} */
  const extra = {};
  if (extras !== "") {
    for (const pair of extras.split(";")) {
      const at = pair.indexOf("=");
      if (at < 0) {
        throw new HcError("malformed", { message: `an extra field without a value: ${JSON.stringify(pair)}` });
      }
      extra[pair.slice(0, at)] = pair.slice(at + 1);
    }
  }
  return {
    id,
    name,
    era: optional(era),
    eraLabel: optional(eraLabel),
    year: optionalInteger(year, "year"),
    month: optionalInteger(month, "month"),
    leapMonth: optionalFlag(leapMonth, "leap month"),
    monthLabel: optional(monthLabel),
    day: optionalInteger(day, "day"),
    leapDay: optionalFlag(leapDay, "leap day"),
    extras: extra,
    error: errorCode === "" ? null : { code: integer(errorCode, "error code"), name: errorName },
    standing: /** @type {import("./hyper-calendar.d.ts").Standing | null} */ (optional(standing)),
    dayBoundary,
    formatted: optional(formatted),
    localeUsed,
    dayNamedBy: dayNaming(dayNamedBy),
  };
}

/**
 * The one line of `hc_parse_date`: `hc_describe_day`'s cells, then the
 * fixed day, empty on a refusal.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").ParsedDate}
 */
function parsedDate(cells) {
  const fixed = cells[DESCRIBE_DAY_COLUMNS.length];
  return {
    ...describedDay(cells.slice(0, DESCRIBE_DAY_COLUMNS.length)),
    fixed: optionalInteger(fixed, "fixed"),
  };
}

/**
 * The `day named by` cell: `start`, `end`, or `null` for a midnight start.
 *
 * @param {string} cell
 * @returns {import("./hyper-calendar.d.ts").DayNamedBy | null}
 */
function dayNaming(cell) {
  if (cell === "") {
    return null;
  }
  if (cell === "start" || cell === "end") {
    return cell;
  }
  throw new HcError("malformed", { message: `a day naming that is neither start nor end: ${JSON.stringify(cell)}` });
}

/**
 * One row of `hc_calendar_units`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").CalendarUnit}
 */
function calendarUnit(cells) {
  const [start, end, label, leap, standing, errorCode, errorName, localeUsed] = cells;
  return {
    start: integer(start, "start"),
    end: integer(end, "end"),
    label: optional(label),
    leap: optionalFlag(leap, "leap"),
    standing: /** @type {import("./hyper-calendar.d.ts").Standing | null} */ (optional(standing)),
    error: errorCode === "" ? null : { code: integer(errorCode, "error code"), name: errorName },
    localeUsed,
  };
}

/**
 * A `;`-joined cell as a list, empty for an empty cell.
 *
 * @param {string} cell
 * @returns {string[]}
 */
function list(cell) {
  return cell === "" ? [] : cell.split(";");
}

/**
 * One row of `hc_calendars`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").CalendarEntry}
 */
function calendarEntry(cells) {
  const [id, name, englishName, earliest, latest, hasEra, hasYear, hasMonth, hasDay, nativeLocales, standing] = cells;
  return {
    id,
    name: optional(name),
    englishName,
    earliest: optionalInteger(earliest, "earliest"),
    latest: optionalInteger(latest, "latest"),
    hasEra: flag(hasEra, "has era"),
    hasYear: flag(hasYear, "has year"),
    hasMonth: flag(hasMonth, "has month"),
    hasDay: flag(hasDay, "has day"),
    nativeLocales: list(nativeLocales),
    standing: /** @type {import("./hyper-calendar.d.ts").Standing} */ (standing),
  };
}

/**
 * One row of `hc_calendar_list`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").CalendarListEntry}
 */
function calendarListEntry(cells) {
  const [id, name, englishName, localeUsed, crate, nativeLocales] = cells;
  return {
    id,
    name: optional(name),
    englishName,
    localeUsed: optional(localeUsed),
    crate: /** @type {import("./hyper-calendar.d.ts").CalendarCrate | null} */ (optional(crate)),
    nativeLocales: list(nativeLocales),
  };
}

/**
 * One row of `hc_locales`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").LocaleEntry}
 */
function localeEntry(cells) {
  const [tag, englishName, nativeName, gregorianMonths, weekdays, gregorianEras, calendars] = cells;
  return {
    tag,
    englishName,
    nativeName,
    gregorianMonths: flag(gregorianMonths, "gregorian months"),
    weekdays: flag(weekdays, "weekdays"),
    gregorianEras: flag(gregorianEras, "gregorian eras"),
    calendars: list(calendars),
  };
}

/**
 * One row of `hc_gregorian_adoption`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").GregorianAdoption}
 */
function gregorianAdoption(cells) {
  const [lastOldDay, firstDay, oldCalendar, scope, source, newCalendar, polity] = cells;
  return {
    lastOldDay: integer(lastOldDay, "last old day"),
    firstDay: integer(firstDay, "first day"),
    oldCalendar,
    scope: /** @type {import("./hyper-calendar.d.ts").AdoptionScope} */ (scope),
    source,
    newCalendar,
    polity,
  };
}

/**
 * One row of `hc_holidays_in_year`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").HolidayInYear}
 */
function holidayInYear(cells) {
  const [date, name, localName, kind, confidence, substitute, observedFor] = cells;
  return {
    date,
    name,
    localName: optional(localName),
    kind: /** @type {import("./hyper-calendar.d.ts").HolidayKind} */ (kind),
    confidence: /** @type {import("./hyper-calendar.d.ts").Confidence} */ (confidence),
    substitute: flag(substitute, "substitute"),
    observedFor: optional(observedFor),
  };
}

/**
 * One row of `hc_holidays_on`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").HolidayOn}
 */
function holidayOn(cells) {
  const [table, tableName, name, localName, kind, confidence, source, substitute, observedFor] = cells;
  return {
    table,
    tableName,
    name,
    localName: optional(localName),
    kind: /** @type {import("./hyper-calendar.d.ts").HolidayKind | "gap"} */ (kind),
    confidence: /** @type {import("./hyper-calendar.d.ts").Confidence | null} */ (optional(confidence)),
    source: optional(source),
    substitute: flag(substitute, "substitute"),
    observedFor: optionalInteger(observedFor, "observed for"),
  };
}

/**
 * The one line of `hc_term_in_effect` or `hc_pentad_in_effect`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").TermInEffect}
 */
function termInEffect(cells) {
  const [index, chineseName, japaneseName, begins, ends, chineseAuthority, japaneseAuthority] = cells;
  return {
    index: integer(index, "index"),
    chineseName,
    japaneseName,
    begins: integer(begins, "begins"),
    ends: integer(ends, "ends"),
    chineseAuthority,
    japaneseAuthority,
  };
}

/**
 * The four cells of one bound of a deep-time row, or `null` where the
 * chronology has no figure. The σ must be there unless `sigmaMayBeEmpty`,
 * which only an earliest-evidence row is, whose source may state none.
 *
 * @param {string} value
 * @param {string} stdDev
 * @param {string} figures
 * @param {string} approximate
 * @param {string} what
 * @param {boolean} sigmaMayBeEmpty
 * @returns {import("./hyper-calendar.d.ts").EarliestEvidenceBound | null}
 */
function bound(value, stdDev, figures, approximate, what, sigmaMayBeEmpty) {
  if (value === "") {
    return null;
  }
  return {
    value: decimal(value, what),
    stdDev: sigmaMayBeEmpty && stdDev === "" ? null : decimal(stdDev, `${what} σ`),
    figures: optionalInteger(figures, `${what} figures`),
    approximate: flag(approximate, `${what} approximate`),
  };
}

/**
 * One row of any deep-time export, every bound with its σ.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").DeepTimeRow}
 */
function deepTimeRow(cells) {
  return /** @type {import("./hyper-calendar.d.ts").DeepTimeRow} */ (anyDeepTimeRow(cells, false));
}

/**
 * One row of `hc_earliest_evidence`, whose σ may be empty.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").EarliestEvidenceRow}
 */
function earliestEvidenceRow(cells) {
  return /** @type {import("./hyper-calendar.d.ts").EarliestEvidenceRow} */ (anyDeepTimeRow(cells, true));
}

/**
 * The shared decoding of a deep-time row.
 *
 * @param {string[]} cells
 * @param {boolean} sigmaMayBeEmpty
 */
function anyDeepTimeRow(cells, sigmaMayBeEmpty) {
  const [
    kind, id, name, scope, start, startStdDev, startFigures, startApproximate,
    end, endStdDev, endFigures, endApproximate, unit, description, source, localisedName,
  ] = cells;
  return {
    kind: /** @type {import("./hyper-calendar.d.ts").DeepTimeKind} */ (kind),
    id,
    name,
    scope: optional(scope),
    start: bound(start, startStdDev, startFigures, startApproximate, "start", sigmaMayBeEmpty),
    end: bound(end, endStdDev, endFigures, endApproximate, "end", sigmaMayBeEmpty),
    unit: /** @type {import("./hyper-calendar.d.ts").DeepTimeUnit} */ (unit),
    description: optional(description),
    source,
    localisedName: optional(localisedName),
  };
}

/**
 * The one line of `hc_sky_at`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").Sky}
 */
function sky(cells) {
  const [
    sunLongitude, sunDistance, moonLongitude, moonLatitude, moonDistance, elongation,
    illuminatedFraction, previousNewMoon, nextNewMoon, deltaT, regime, source,
  ] = cells;
  return {
    sunLongitude: decimal(sunLongitude, "sun longitude"),
    sunDistance: decimal(sunDistance, "sun distance"),
    moonLongitude: decimal(moonLongitude, "moon longitude"),
    moonLatitude: decimal(moonLatitude, "moon latitude"),
    moonDistance: decimal(moonDistance, "moon distance"),
    elongation: decimal(elongation, "elongation"),
    illuminatedFraction: decimal(illuminatedFraction, "illuminated fraction"),
    previousNewMoon: integer(previousNewMoon, "previous new moon"),
    nextNewMoon: integer(nextNewMoon, "next new moon"),
    deltaT: decimal(deltaT, "ΔT"),
    deltaTRegime: /** @type {import("./hyper-calendar.d.ts").DeltaTRegime} */ (regime),
    source,
  };
}

/**
 * One row of `hc_solar_terms_between` or `hc_moon_phases_between`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").SkyEvent}
 */
function skyEvent(cells) {
  const [angle, instant, name, japaneseName] = cells;
  return {
    angle: integer(angle, "angle"),
    instant: integer(instant, "instant"),
    name,
    japaneseName: optional(japaneseName),
  };
}

/**
 * The one line of `hc_orbit_at`, or the tail of a line of `hc_orbit_series`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").Orbit}
 */
function orbit(cells) {
  const [
    eccentricity, eccentricitySpread, obliquity, obliquitySpread,
    longitudeOfPerihelion, longitudeOfPerihelionSpread,
    climaticPrecession, climaticPrecessionSpread, insolation, solarConstant, source,
  ] = cells;
  return {
    eccentricity: decimal(eccentricity, "eccentricity"),
    eccentricitySpread: decimal(eccentricitySpread, "eccentricity spread"),
    obliquity: decimal(obliquity, "obliquity"),
    obliquitySpread: decimal(obliquitySpread, "obliquity spread"),
    longitudeOfPerihelion: decimal(longitudeOfPerihelion, "longitude of perihelion"),
    longitudeOfPerihelionSpread: decimal(longitudeOfPerihelionSpread, "longitude of perihelion spread"),
    climaticPrecession: decimal(climaticPrecession, "climatic precession"),
    climaticPrecessionSpread: decimal(climaticPrecessionSpread, "climatic precession spread"),
    insolation65NJune: decimal(insolation, "insolation 65°N June"),
    solarConstant: decimal(solarConstant, "solar constant"),
    source,
  };
}

/**
 * One line of `hc_orbit_series`: the epoch, then the cells of `hc_orbit_at`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").OrbitSample}
 */
function orbitSample(cells) {
  const [yearsBefore1950, ...rest] = cells;
  return { yearsBefore1950: decimal(yearsBefore1950, "years before 1950"), ...orbit(rest) };
}

/**
 * The one line of `hc_tai64_decode`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").Tai64Label}
 */
function tai64Label(cells) {
  const [format, seconds, attoseconds] = cells;
  return {
    format: /** @type {import("./hyper-calendar.d.ts").Tai64Format} */ (format),
    seconds: bigInteger(seconds, "tai seconds"),
    attoseconds: bigInteger(attoseconds, "attoseconds"),
  };
}

/**
 * The one line of `hc_gnss_week`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").GnssWeek}
 */
function gnssWeek(cells) {
  const [week, broadcastWeek, towSeconds, towAttoseconds] = cells;
  return {
    week: integer(week, "week"),
    broadcastWeek: integer(broadcastWeek, "broadcast week"),
    towSeconds: integer(towSeconds, "tow seconds"),
    towAttoseconds: bigInteger(towAttoseconds, "tow attoseconds"),
  };
}

/**
 * The one line of `hc_gnss_to_tai`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").TaiInstant}
 */
function taiInstant(cells) {
  const [seconds, attoseconds] = cells;
  return { seconds: bigInteger(seconds, "tai seconds"), attoseconds: bigInteger(attoseconds, "attoseconds") };
}

/**
 * The one line of `hc_glonass_date`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").GlonassDate}
 */
function glonassDate(cells) {
  const [fourYearInterval, day] = cells;
  return { fourYearInterval: integer(fourYearInterval, "four-year interval"), day: integer(day, "day") };
}

/**
 * The one line of `hc_fixed_from_ole_automation`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").OleAutomationDay}
 */
function oleAutomationDay(cells) {
  const [fixed, secondsOfDay] = cells;
  return { fixed: integer(fixed, "fixed"), secondsOfDay: decimal(secondsOfDay, "seconds of day") };
}

/**
 * The one line of `hc_excel_1900_day`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").Excel1900Day}
 */
function excel1900Day(cells) {
  const [fixed, phantom] = cells;
  return { fixed: optionalInteger(fixed, "fixed"), phantom: flag(phantom, "phantom") };
}

/**
 * One line of `hc_panchanga_at` or `hc_panchanga_of_day`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").PanchangaLimb}
 */
function panchangaLimb(cells) {
  const [limb, number, name, devanagari, began, ends, readAt, ayanamsa, ayanamsaName] = cells;
  return {
    limb: /** @type {"yoga" | "karana"} */ (limb),
    number: integer(number, "number"),
    name,
    devanagari,
    began: integer(began, "began"),
    ends: integer(ends, "ends"),
    readAt: integer(readAt, "read at"),
    ayanamsa: /** @type {import("./hyper-calendar.d.ts").Ayanamsa | null} */ (optional(ayanamsa)),
    ayanamsaName: optional(ayanamsaName),
  };
}

/**
 * The one line of `hc_chinese_marriage_augury`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").MarriageAugury}
 */
function marriageAugury(cells) {
  const [augury, atStart, atEnd] = cells;
  return {
    augury: /** @type {import("./hyper-calendar.d.ts").MarriageAuguryName} */ (augury),
    lichunAtStart: flag(atStart, "lichun at start"),
    lichunAtEnd: flag(atEnd, "lichun at end"),
  };
}

/**
 * The one line of `hc_utc_from_tai`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").UtcLabel}
 */
function utcLabel(cells) {
  const [unixSeconds, leapSecond] = cells;
  return { unixSeconds: bigInteger(unixSeconds, "unix seconds"), leapSecond: flag(leapSecond, "leap second") };
}

/**
 * The one line of `hc_tai64_posix_plus_10_decode`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").PosixTai64Label}
 */
function posixTai64Label(cells) {
  const [format, seconds, attoseconds] = cells;
  return {
    format: /** @type {"tai64" | "tai64n"} */ (format),
    seconds: bigInteger(seconds, "unix seconds"),
    attoseconds: bigInteger(attoseconds, "attoseconds"),
  };
}

/**
 * The one line of `hc_uuid_timestamp`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").UuidTimestamp}
 */
function uuidTimestamp(cells) {
  const [version, timestamp, unixSeconds, attoseconds] = cells;
  return {
    version: /** @type {1 | 6} */ (integer(version, "version")),
    timestamp: bigInteger(timestamp, "timestamp"),
    unixSeconds: bigInteger(unixSeconds, "unix seconds"),
    attoseconds: bigInteger(attoseconds, "attoseconds"),
  };
}

/**
 * The one line of `hc_ntp_resolve`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").NtpDate}
 */
function ntpDate(cells) {
  const [era, offset, fraction, unixSeconds, attoseconds] = cells;
  return {
    era: integer(era, "era"),
    offset: integer(offset, "era offset"),
    fraction: bigInteger(fraction, "fraction"),
    unixSeconds: bigInteger(unixSeconds, "unix seconds"),
    attoseconds: bigInteger(attoseconds, "attoseconds"),
  };
}

/**
 * The one line of `hc_uuid_timestamp_encode`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").UuidTimeFields}
 */
function uuidTimeFields(cells) {
  const [timestamp, v1, v6] = cells;
  return { timestamp: bigInteger(timestamp, "timestamp"), v1, v6 };
}

/**
 * The one line of `hc_ntp_encode`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").NtpEncoding}
 */
function ntpEncoding(cells) {
  const [era, offset, fraction, date, timestamp] = cells;
  return {
    era: integer(era, "era"),
    offset: integer(offset, "era offset"),
    fraction: bigInteger(fraction, "fraction"),
    date,
    timestamp,
  };
}

/**
 * The one line of `hc_fat_decode`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").FatReading}
 */
function fatReading(cells) {
  const [fixed, secondsOfDay] = cells;
  return { fixed: integer(fixed, "fixed"), secondsOfDay: integer(secondsOfDay, "seconds of day") };
}

/**
 * The one line of `hc_fat_encode`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").FatWords}
 */
function fatWords(cells) {
  const [date, time] = cells;
  return { date: integer(date, "date word"), time: integer(time, "time word") };
}

/**
 * The one line of `hc_epoch_from_tt`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").Epoch}
 */
function epochLine(cells) {
  const [notation, epoch] = cells;
  return {
    notation: /** @type {import("./hyper-calendar.d.ts").EpochNotation} */ (notation),
    epoch: decimal(epoch, "epoch"),
  };
}

/**
 * The one line of `hc_tt_from_epoch`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").EpochInstant}
 */
function epochInstant(cells) {
  const [notation, seconds, attoseconds] = cells;
  return {
    notation: /** @type {import("./hyper-calendar.d.ts").EpochNotation} */ (notation),
    seconds: bigInteger(seconds, "tt seconds"),
    attoseconds: bigInteger(attoseconds, "attoseconds"),
  };
}

/**
 * The one line of `hc_circad_date`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").CircadDate}
 */
function circadDate(cells) {
  const [calendar, year, month, day, monthName, weekName, count, fraction, leap, source] = cells;
  return {
    calendar: /** @type {import("./hyper-calendar.d.ts").CircadCalendar} */ (calendar),
    year: integer(year, "year"),
    month: integer(month, "month"),
    day: integer(day, "day"),
    monthName,
    weekName: optional(weekName),
    count: integer(count, "count"),
    fraction: decimal(fraction, "fraction"),
    leap: flag(leap, "leap"),
    source,
  };
}

/**
 * One line of `hc_holiday_tables`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").HolidayTable}
 */
function holidayTable(cells) {
  const [code, kind, name, englishName, localeUsed, source, country, shortName] = cells;
  return {
    code,
    kind: /** @type {import("./hyper-calendar.d.ts").HolidayTableKind} */ (kind),
    name: optional(name),
    englishName,
    localeUsed: optional(localeUsed),
    source: optional(source),
    country: optional(country),
    shortName: optional(shortName),
  };
}

/**
 * One line of `hc_zones` and `hc_zone_location`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").ZoneLocation}
 */
function zoneLocation(cells) {
  const [zone, latitude, longitude, countries, country, comment, exemplarCity, localeUsed] = cells;
  return {
    zone,
    latitude: decimal(latitude, "latitude"),
    longitude: decimal(longitude, "longitude"),
    countries: countries.split(";"),
    country: optional(country),
    comment: optional(comment),
    exemplarCity,
    localeUsed,
  };
}

/**
 * The one line of `hc_zone_offset`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").ZoneOffset}
 */
function zoneOffset(cells) {
  const [offset, dst, abbreviation, nextTransition, nextOffset, rules] = cells;
  return {
    offsetSeconds: integer(offset, "offset"),
    dst: flag(dst, "dst"),
    abbreviation: optional(abbreviation),
    nextTransition: optionalInteger(nextTransition, "next transition"),
    nextOffsetSeconds: optionalInteger(nextOffset, "next offset"),
    rules: /** @type {import("./hyper-calendar.d.ts").ZoneRules} */ (rules),
  };
}

/**
 * The one line of `hc_lectionary`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").Lectionary}
 */
function lectionaryLine(cells) {
  const [liturgicalYear, sundayCycle, weekdayCycle, proper] = cells;
  return {
    liturgicalYear: integer(liturgicalYear, "liturgical year"),
    sundayCycle: /** @type {"A" | "B" | "C"} */ (sundayCycle),
    weekdayCycle: /** @type {"I" | "II"} */ (weekdayCycle),
    proper: optionalInteger(proper, "proper"),
  };
}

/**
 * The three cells naming a missing solar event, or `null` where none is.
 *
 * @param {string} missing
 * @param {string} day
 * @param {string} depression
 * @returns {import("./hyper-calendar.d.ts").MissingSolarEvent | null}
 */
function missingSolarEvent(missing, day, depression, arcseconds = "") {
  if (missing === "") {
    return null;
  }
  return {
    event: /** @type {import("./hyper-calendar.d.ts").MissingSolarEventName} */ (missing),
    day: integer(day, "missing day"),
    depressionArcminutes: optionalInteger(depression, "depression"),
    depressionArcseconds: optionalInteger(arcseconds, "depression arcseconds"),
  };
}

/**
 * The one line of `hc_solar_time`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").SolarTime}
 */
function solarTime(cells) {
  const [day, hours, missing, missingDay, depression] = cells;
  return {
    day: optionalInteger(day, "day"),
    hours: hours === "" ? null : decimal(hours, "hours"),
    missing: missingSolarEvent(missing, missingDay, depression),
  };
}

/**
 * The one line of `hc_solar_event`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").SolarEvent}
 */
function solarEvent(cells) {
  const [instant, missing, missingDay, depression, arcseconds] = cells;
  return {
    instant: optionalInteger(instant, "instant"),
    missing: missingSolarEvent(missing, missingDay, depression, arcseconds),
  };
}

/**
 * One line of `hc_horizons`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").Horizon}
 */
function horizonLine(cells) {
  const [id, englishName, description, source, shortName, name, localeUsed] = cells;
  return {
    id: /** @type {import("./hyper-calendar.d.ts").HorizonId} */ (id),
    englishName, description, source, shortName, name, localeUsed,
  };
}

/**
 * The one line of `hc_sunrise` or `hc_sunset`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").SolarCrossing}
 */
function solarCrossing(cells) {
  const [instant, missing, missingDay, depression, altitude] = cells;
  return {
    instant: optionalInteger(instant, "instant"),
    missing: missingSolarEvent(missing, missingDay, depression),
    altitudeDegrees: decimal(altitude, "altitude"),
  };
}

/**
 * The one line of `hc_hindu_lunar_date`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").HinduLunarDate}
 */
function hinduLunarDateLine(cells) {
  const [
    sakaYear, vikramaYear, month, leapMonth, tithi, leapDay, sunrise,
    monthName, leapMonthWord, sakaEra, vikramaEra, localeUsed,
  ] = cells;
  return {
    sakaYear: integer(sakaYear, "saka year"),
    vikramaYear: integer(vikramaYear, "vikrama year"),
    month: integer(month, "month"),
    leapMonth: flag(leapMonth, "leap month"),
    tithi: integer(tithi, "tithi"),
    leapDay: flag(leapDay, "leap day"),
    sunrise: integer(sunrise, "sunrise"),
    monthName: optional(monthName),
    leapMonthWord: optional(leapMonthWord),
    sakaEra: optional(sakaEra),
    vikramaEra: optional(vikramaEra),
    localeUsed,
  };
}

/**
 * The one line of `hc_surya_siddhanta_at`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").SuryaSiddhantaSky}
 */
function suryaSiddhantaSky(cells) {
  const [sun, moon, elongation, tithi, sign] = cells;
  return {
    sunLongitude: decimal(sun, "sun"),
    moonLongitude: decimal(moon, "moon"),
    elongation: decimal(elongation, "elongation"),
    tithi: integer(tithi, "tithi"),
    sign: integer(sign, "sign"),
  };
}

/**
 * The one line of `hc_crescent_visible`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").CrescentVisibility}
 */
function crescentVisibility(cells) {
  const [visible, evaluatedAt, elongation, arcOfLight, altitude, arcOfVision, width] = cells;
  /** @param {string} cell @param {string} what */
  const optional = (cell, what) => (cell === "" ? null : decimal(cell, what));
  return {
    visible: flag(visible, "visible"),
    evaluatedAt: optionalInteger(evaluatedAt, "evaluated at"),
    elongation: optional(elongation, "elongation"),
    arcOfLight: optional(arcOfLight, "arc of light"),
    altitude: optional(altitude, "altitude"),
    arcOfVision: optional(arcOfVision, "arc of vision"),
    widthArcminutes: optional(width, "width"),
  };
}

/**
 * The one line of `hc_decan_at`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").Decan}
 */
function decanLine(cells) {
  const [sign, signName, decan, ruler, rulerName, degrees] = cells;
  return {
    sign: integer(sign, "sign"),
    signName,
    decan: integer(decan, "decan"),
    ruler: /** @type {import("./hyper-calendar.d.ts").DecanRuler} */ (ruler),
    rulerName,
    degreesIntoDecan: decimal(degrees, "degrees into decan"),
  };
}

/**
 * The one line of `hc_hjd_tt` or `hc_hjd_utc`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").HeliocentricJulianDate}
 */
function heliocentricJulianDate(cells) {
  const [hjd, correction] = cells;
  return { hjd: decimal(hjd, "hjd"), correctionSeconds: decimal(correction, "correction") };
}

/**
 * The one line of `hc_tt_bipm`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").TtBipmReading}
 */
function ttBipmReading(cells) {
  const [offset, minusSeconds, minusAttoseconds, seconds, attoseconds] = cells;
  return {
    offsetSeconds: decimal(offset, "offset"),
    minusTai: {
      seconds: bigInteger(minusSeconds, "minus tai seconds"),
      attoseconds: bigInteger(minusAttoseconds, "minus tai attoseconds"),
    },
    reading: {
      seconds: bigInteger(seconds, "reading seconds"),
      attoseconds: bigInteger(attoseconds, "reading attoseconds"),
    },
  };
}

/**
 * The one line of `hc_naming_period_on`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").NamingPeriodOn}
 */
function namingPeriodOn(cells) {
  const [state, period, month, weekday, meaning, earliest, inForceBy, ended, source] = cells;
  if (state === "ordinary") {
    return { state, period: null };
  }
  if (state !== "in-force" && state !== "undecided") {
    throw new HcError("malformed", { message: `state is not a naming state: ${JSON.stringify(state)}` });
  }
  return {
    state,
    period: {
      id: period,
      monthName: optional(month),
      weekdayName: optional(weekday),
      weekdayMeaning: optional(meaning),
      earliest: integer(earliest, "earliest"),
      inForceBy: integer(inForceBy, "in force by"),
      ended: integer(ended, "ended"),
      source,
    },
  };
}

/**
 * The one line of `hc_asian_day`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").AsianDay}
 */
function asianDayLine(cells) {
  const [year, month, monthName, written, number] = cells;
  if (written !== "unnumbered" && written !== "numbered") {
    throw new HcError("malformed", { message: `written is not unnumbered or numbered: ${JSON.stringify(written)}` });
  }
  return {
    year: integer(year, "year"),
    month: integer(month, "month"),
    monthName,
    written,
    number: integer(number, "number"),
  };
}

/**
 * The one line of `hc_holy_year_on`, `null` outside a jubilee.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").HolyYear | null}
 */
function holyYearLine(cells) {
  const [state, title, kind, pope, bull, given, opens, closes, diocesesOpen, diocesesClose] = cells;
  if (state === "outside") {
    return null;
  }
  if (state !== "within") {
    throw new HcError("malformed", { message: `state is not within or outside: ${JSON.stringify(state)}` });
  }
  return {
    title,
    kind: /** @type {"ordinary" | "extraordinary"} */ (kind),
    pope,
    bull,
    given: integer(given, "given"),
    opens: integer(opens, "opens"),
    closes: integer(closes, "closes"),
    dioceses: diocesesOpen === ""
      ? null
      : { opens: integer(diocesesOpen, "dioceses open"), closes: integer(diocesesClose, "dioceses close") },
  };
}

/**
 * One line of `hc_common_worship_on`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").CommonWorshipCelebration}
 */
function commonWorshipCelebration(cells) {
  const [title, rank, rankName] = cells;
  return { title, rank: /** @type {import("./hyper-calendar.d.ts").CommonWorshipRank} */ (rank), rankName };
}

/**
 * The one line of `hc_mars_time`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").MarsTime}
 */
function marsTime(cells) {
  const [
    msd, mtc, mtcHours, lmst, lmstHours, ltst, ltstHours, equationOfTime, ls, marsYear,
    darianYear, darianMonth, darianSol, darianMonthName, darianSolOfWeek, source,
  ] = cells;
  return {
    marsSolDate: decimal(msd, "mars sol date"),
    mtc,
    mtcHours: decimal(mtcHours, "mtc hours"),
    lmst,
    lmstHours: decimal(lmstHours, "lmst hours"),
    ltst,
    ltstHours: decimal(ltstHours, "ltst hours"),
    equationOfTimeMinutes: decimal(equationOfTime, "equation of time"),
    solarLongitude: decimal(ls, "ls"),
    marsYear: integer(marsYear, "mars year"),
    darian: {
      year: integer(darianYear, "darian year"),
      month: integer(darianMonth, "darian month"),
      sol: integer(darianSol, "darian sol"),
      monthName: darianMonthName,
      solOfWeek: darianSolOfWeek,
    },
    source,
  };
}

/**
 * One line of `hc_missions`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").Mission}
 */
function mission(cells) {
  const [id, name, landingUtc, landingUnix, landingSol, clock, clockLongitude, siteLongitude, published, note, source] = cells;
  return {
    id: /** @type {import("./hyper-calendar.d.ts").MissionId} */ (id),
    name,
    landingUtc,
    landingUnix: integer(landingUnix, "landing unix"),
    landingSol: optionalInteger(landingSol, "landing sol"),
    clock: /** @type {import("./hyper-calendar.d.ts").MissionClock | null} */ (optional(clock)),
    clockEastLongitude: clockLongitude === "" ? null : decimal(clockLongitude, "clock longitude"),
    siteEastLongitude: decimal(siteLongitude, "site longitude"),
    published: flag(published, "published"),
    note,
    source,
  };
}

/**
 * One line of `hc_bodies`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").Body}
 */
function body(cells) {
  const [
    id, name, kind, primary, siderealRotation, solarDay, solarDayOrigin, yearInLocalDays,
    zeroPoint, zeroPointNote, source, status,
  ] = cells;
  return {
    id: /** @type {import("./hyper-calendar.d.ts").BodyId} */ (id),
    name,
    kind: /** @type {import("./hyper-calendar.d.ts").BodyKind} */ (kind),
    primary: /** @type {import("./hyper-calendar.d.ts").BodyId | null} */ (optional(primary)),
    siderealRotationHours: decimal(siderealRotation, "sidereal rotation"),
    solarDaySeconds: solarDay === "" ? null : decimal(solarDay, "solar day"),
    solarDayOrigin: /** @type {"measured" | "derived" | null} */ (optional(solarDayOrigin)),
    yearInLocalDays: yearInLocalDays === "" ? null : decimal(yearInLocalDays, "year in local days"),
    zeroPoint: /** @type {import("./hyper-calendar.d.ts").ZeroPoint} */ (zeroPoint),
    zeroPointNote,
    source,
    status: optional(status),
  };
}

/**
 * The one line of `hc_body_time`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").BodyTime}
 */
function bodyTime(cells) {
  const [day, fraction, time, hours, solarDay, localHour, zeroPoint, zeroPointNote] = cells;
  return {
    day: integer(day, "day"),
    fraction: decimal(fraction, "fraction"),
    time,
    hours: decimal(hours, "hours"),
    solarDaySeconds: decimal(solarDay, "solar day"),
    localHourSeconds: decimal(localHour, "local hour"),
    zeroPoint: /** @type {import("./hyper-calendar.d.ts").ZeroPoint} */ (zeroPoint),
    zeroPointNote,
  };
}

/**
 * The one line of `hc_proper_time`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").ProperTime}
 */
function properTime(cells) {
  const [beta, lorentzFactor, properSeconds, rate, micros, constants, source] = cells;
  return {
    beta: decimal(beta, "beta"),
    lorentzFactor: decimal(lorentzFactor, "lorentz factor"),
    properSeconds: decimal(properSeconds, "proper seconds"),
    rate: decimal(rate, "rate"),
    microsecondsPerDay: decimal(micros, "microseconds per day"),
    constants: list(constants),
    source,
  };
}

/**
 * The one line of `hc_gravitational_dilation`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").GravitationalDilation}
 */
function gravitationalDilation(cells) {
  const [id, gm, gmConstant, schwarzschildRadius, factor, micros, constants, source] = cells;
  return {
    id: /** @type {import("./hyper-calendar.d.ts").GravitatingBodyId} */ (id),
    gm: decimal(gm, "gm"),
    gmConstant,
    schwarzschildRadius: decimal(schwarzschildRadius, "schwarzschild radius"),
    factor: decimal(factor, "factor"),
    microsecondsPerDay: decimal(micros, "microseconds per day"),
    constants: list(constants),
    source,
  };
}

/**
 * One line of `hc_gravitating_bodies`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").GravitatingBody}
 */
function gravitatingBody(cells) {
  const [id, name, gm, gmConstant, source] = cells;
  return { id: /** @type {import("./hyper-calendar.d.ts").GravitatingBodyId} */ (id), name, gm: decimal(gm, "gm"), gmConstant, source };
}

/** The capacity a text read starts with unless `load` was told otherwise. */
/**
 * The five cells of an instant a CCSDS line writes: TAI and its UTC label.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").CcsdsInstant}
 */
function ccsdsInstant(cells) {
  const [taiSeconds, taiAttoseconds, unixSeconds, leapSecond, utcAttoseconds] = cells;
  return {
    tai: {
      seconds: bigInteger(taiSeconds, "tai seconds"),
      attoseconds: bigInteger(taiAttoseconds, "tai attoseconds"),
    },
    utc: {
      unixSeconds: bigInteger(unixSeconds, "unix seconds"),
      leapSecond: flag(leapSecond, "leap second"),
      attoseconds: bigInteger(utcAttoseconds, "utc attoseconds"),
    },
  };
}

/**
 * The one line of `hc_ccsds_decode`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").CcsdsCode}
 */
function ccsdsCode(cells) {
  return {
    code: /** @type {import("./hyper-calendar.d.ts").CcsdsCodeName} */ (cells[0]),
    ...ccsdsInstant(cells.slice(1, 6)),
  };
}

/**
 * The one line of `hc_ccsds_ascii_parse`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").CcsdsAsciiCode}
 */
function ccsdsAsciiCode(cells) {
  const [variation, , , , , , precision, digits, terminator] = cells;
  return {
    variation: /** @type {"a" | "b"} */ (variation),
    ...ccsdsInstant(cells.slice(1, 6)),
    precision: /** @type {import("./hyper-calendar.d.ts").CcsdsAsciiPrecision} */ (precision),
    digits: optionalInteger(digits, "digits"),
    terminator: flag(terminator, "terminator"),
  };
}

/**
 * The one line of `hc_radio_decode`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").RadioMinute}
 */
function radioMinute(cells) {
  const [unixSeconds, fixed, hour, minute, offsetHours, seconds, leap, summer, zoneChange, dut1, dstNext] = cells;
  return {
    unixSeconds: integer(unixSeconds, "unix seconds"),
    fixed: integer(fixed, "fixed"),
    hour: integer(hour, "hour"),
    minute: integer(minute, "minute"),
    offsetHours: integer(offsetHours, "offset hours"),
    seconds: integer(seconds, "seconds"),
    leap: /** @type {import("./hyper-calendar.d.ts").RadioLeap} */ (leap),
    summer: /** @type {import("./hyper-calendar.d.ts").RadioSummer | null} */ (optional(summer)),
    zoneChange: zoneChange === "" ? null : flag(zoneChange, "zone change"),
    dut1Tenths: optionalInteger(dut1, "dut1 tenths"),
    dstNext: optionalInteger(dstNext, "dst next"),
  };
}

/**
 * The one line of `hc_six_hour_clock`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").SixHourReading}
 */
function sixHourReading(cells) {
  const [hour, minute, second, half, period, periodEnglish] = cells;
  return {
    hour: integer(hour, "hour"),
    minute: integer(minute, "minute"),
    second: integer(second, "second"),
    half: /** @type {"day" | "night"} */ (half),
    period: optional(period),
    periodEnglish: optional(periodEnglish),
  };
}

/**
 * One line of `hc_kalam`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").KalamPeriod}
 */
function kalamPeriod(cells) {
  const [id, englishName, name, localeUsed, part, clock, start, end, missing, missingDay, depression, arcseconds] =
    cells;
  return {
    id: /** @type {import("./hyper-calendar.d.ts").KalamId} */ (id),
    englishName,
    name,
    localeUsed,
    part: integer(part, "part"),
    clock: /** @type {"universal" | "local"} */ (clock),
    start: optionalInteger(start, "start"),
    end: optionalInteger(end, "end"),
    missing: missingSolarEvent(missing, missingDay, depression, arcseconds),
  };
}

/**
 * The one line of `hc_almanac_cycles`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").AlmanacCycles}
 */
function almanacCycles(cells) {
  const [
    eho, ehoRomaji, azimuth, sixteenPoint, direction, period, periodName, era, star, ruler,
    firstYear, lastYear, withoutSon,
  ] = cells;
  return {
    eho,
    ehoRomaji,
    azimuth: integer(azimuth, "azimuth"),
    sixteenPoint,
    direction,
    period: integer(period, "period"),
    periodName,
    era,
    star,
    ruler,
    firstYear: integer(firstYear, "first year"),
    lastYear: integer(lastYear, "last year"),
    withoutSon: withoutSon === "" ? null : flag(withoutSon, "without son"),
  };
}

/**
 * One line of `hc_almanac_day`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").AlmanacAnnotation}
 */
function almanacAnnotation(cells) {
  const [kind, id, name, localeUsed, japanese, reading, auspicious, printed] = cells;
  return {
    kind: /** @type {import("./hyper-calendar.d.ts").AlmanacKind} */ (kind),
    id,
    name,
    localeUsed,
    japanese,
    reading: optional(reading),
    auspicious: auspicious === "" ? null : flag(auspicious, "auspicious"),
    printed: printed === "" ? null : flag(printed, "printed"),
  };
}

/**
 * One line of `hc_choghadiya`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").ChoghadiyaPart}
 */
function choghadiyaPart(cells) {
  const [half, part, id, name, localeUsed, quality, ruler, start, end, missing, missingDay, depression, arcseconds] =
    cells;
  return {
    half: /** @type {"day" | "night"} */ (half),
    part: integer(part, "part"),
    id: /** @type {import("./hyper-calendar.d.ts").ChoghadiyaId} */ (id),
    name,
    localeUsed,
    quality: /** @type {"auspicious" | "neutral" | "inauspicious"} */ (quality),
    ruler: /** @type {import("./hyper-calendar.d.ts").ClassicalPlanet} */ (ruler),
    start: optionalInteger(start, "start"),
    end: optionalInteger(end, "end"),
    missing: missingSolarEvent(missing, missingDay, depression, arcseconds),
  };
}

/**
 * The one line of `hc_panchak`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").PanchakWindow}
 */
function panchakWindow(cells) {
  const [within, opens, closes, weekday, kind, name, localeUsed] = cells;
  return {
    within: flag(within, "within"),
    opens: integer(opens, "opens"),
    closes: integer(closes, "closes"),
    weekday: integer(weekday, "weekday"),
    kind: /** @type {import("./hyper-calendar.d.ts").PanchakKind | null} */ (optional(kind)),
    name: optional(name),
    localeUsed: optional(localeUsed),
  };
}

/**
 * The one line of `hc_kumbh`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").KumbhOccasion}
 */
function kumbhOccasion(cells) {
  const [id, site, siteName, localeUsed, river, jupiter, sun, atNewMoon, from, to, holds] = cells;
  return {
    id: /** @type {import("./hyper-calendar.d.ts").KumbhYoga} */ (id),
    site,
    siteName,
    localeUsed,
    river,
    jupiter: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (jupiter),
    sun: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (sun),
    atNewMoon: flag(atNewMoon, "at new moon"),
    from: optionalInteger(from, "from"),
    to: optionalInteger(to, "to"),
    holds: holds === "" ? null : flag(holds, "holds"),
  };
}

/**
 * One line of `hc_pushkaram`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").PushkaramDays}
 */
function pushkaramDays(cells) {
  const [id, name, localeUsed, region, sign, first, last, missing, missingDay, depression, arcseconds] = cells;
  return {
    id,
    name,
    localeUsed,
    region: optional(region),
    sign: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (sign),
    first: optionalInteger(first, "first"),
    last: optionalInteger(last, "last"),
    missing: missingSolarEvent(missing, missingDay, depression, arcseconds),
  };
}

/**
 * One line of `hc_folk_day`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").FolkDay}
 */
function folkDay(cells) {
  const [kind, id, name, localeUsed, count] = cells;
  return {
    kind: /** @type {import("./hyper-calendar.d.ts").FolkDayKind} */ (kind),
    id,
    name,
    localeUsed,
    count: optionalInteger(count, "count"),
  };
}

/**
 * The one line of `hc_night_watch`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").NightWatch}
 */
function nightWatch(cells) {
  const [watch, points, name, localeUsed, hanName, branch] = cells;
  return {
    watch: integer(watch, "watch"),
    points: integer(points, "points"),
    name,
    localeUsed,
    hanName,
    branch,
  };
}

/**
 * The one line of `hc_barhaspatya_year`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").BarhaspatyaYear}
 */
function barhaspatyaYear(cells) {
  const [position, name, expunged, expungedName, localeUsed] = cells;
  return {
    position: integer(position, "position"),
    name,
    expunged: optionalInteger(expunged, "expunged"),
    expungedName: optional(expungedName),
    localeUsed,
  };
}

/**
 * One line of `hc_planetary_hour` and `hc_planetary_hours_of_day`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").PlanetaryHour}
 */
function planetaryHour(cells) {
  const [day, hour, ruler, name, localeUsed, daytime, start, end, missing, missingDay, depression, arcseconds] =
    cells;
  return {
    day: optionalInteger(day, "day"),
    hour: optionalInteger(hour, "hour"),
    ruler: /** @type {import("./hyper-calendar.d.ts").ClassicalPlanet | null} */ (optional(ruler)),
    name: optional(name),
    localeUsed: optional(localeUsed),
    daytime: daytime === "" ? null : flag(daytime, "daytime"),
    start: optionalInteger(start, "start"),
    end: optionalInteger(end, "end"),
    missing: missingSolarEvent(missing, missingDay, depression, arcseconds),
  };
}

/**
 * The one line of `hc_gmat_from_gmt` and `hc_gmt_from_gmat`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").ClockReading}
 */
function clockReading(cells) {
  const [fixed, secondsOfDay, attoseconds] = cells;
  return {
    fixed: integer(fixed, "fixed"),
    secondsOfDay: integer(secondsOfDay, "seconds of day"),
    attoseconds: bigInteger(attoseconds, "attoseconds"),
  };
}

/**
 * The one line of `hc_irig_decode`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").IrigReading}
 */
function irigReading(cells) {
  const [fixed, dayOfYear, hour, minute, second, hundredths, year, control, sbs] = cells;
  return {
    fixed: integer(fixed, "fixed"),
    dayOfYear: integer(dayOfYear, "day of year"),
    hour: integer(hour, "hour"),
    minute: integer(minute, "minute"),
    second: integer(second, "second"),
    hundredths: integer(hundredths, "hundredths"),
    year: optionalInteger(year, "year"),
    control: optionalInteger(control, "control"),
    straightBinarySeconds: optionalInteger(sbs, "straight binary seconds"),
  };
}

/**
 * A cell of numbers separated by spaces.
 *
 * @param {string} cell
 * @param {string} column
 * @returns {number[]}
 */
function integers(cell, column) {
  return cell === "" ? [] : cell.split(" ").map((item) => integer(item, column));
}

/**
 * One line of `hc_irig_formats`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").IrigFormatInfo}
 */
function irigFormat(cells) {
  const [format, indexCount, counts, frame, fields, control, modulations, carriers, expressions] = cells;
  return {
    format,
    indexCountMicroseconds: integer(indexCount, "index count microseconds"),
    indexCounts: integer(counts, "index counts"),
    frameMicroseconds: integer(frame, "frame microseconds"),
    fields: fields.split(" "),
    controlBits: integer(control, "control bits"),
    modulations: integers(modulations, "modulations"),
    carriers: integers(carriers, "carriers"),
    expressions: integers(expressions, "expressions"),
  };
}

/**
 * The one line of `hc_orthodox_fast_on`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").OrthodoxFastDay}
 */
function orthodoxFastDay(cells) {
  const [fastDay, status, period, periodName, kind, abstinence] = cells;
  return {
    fastDay: flag(fastDay, "fast day"),
    status: /** @type {"period" | "weekly-fast" | "none"} */ (status),
    period: optional(period),
    periodName: optional(periodName),
    kind: /** @type {"fast" | "fast-free" | "meat-excluded" | null} */ (optional(kind)),
    abstinence: /** @type {"nothing" | "meat" | "fast"} */ (abstinence),
  };
}

/**
 * One line of `hc_orthodox_fast_seasons`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").OrthodoxFastSeason}
 */
function orthodoxFastSeason(cells) {
  const [id, englishName, kind, first, last] = cells;
  return {
    id,
    englishName,
    kind: /** @type {"fast" | "fast-free" | "meat-excluded"} */ (kind),
    first: optionalInteger(first, "first"),
    last: optionalInteger(last, "last"),
  };
}

/**
 * One line of `hc_prayer_times`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").PrayerTime}
 */
function prayerTime(cells) {
  const [time, instant, missing, missingDay, depression, arcseconds] = cells;
  return {
    time: /** @type {import("./hyper-calendar.d.ts").PrayerTimeName} */ (time),
    instant: optionalInteger(instant, "instant"),
    missing: missingSolarEvent(missing, missingDay, depression, arcseconds),
  };
}

/**
 * One line of `hc_prayer_methods`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").PrayerMethod}
 */
function prayerMethod(cells) {
  const [id, englishName, fajr, maghrib, isha, ishaMinutes, ishaRamadanMinutes, midnight, source] = cells;
  return {
    id,
    englishName,
    fajrArcminutes: integer(fajr, "fajr"),
    maghribArcminutes: optionalInteger(maghrib, "maghrib"),
    ishaArcminutes: optionalInteger(isha, "isha"),
    ishaMinutes: optionalInteger(ishaMinutes, "isha minutes"),
    ishaRamadanMinutes: optionalInteger(ishaRamadanMinutes, "isha ramadan minutes"),
    midnight: /** @type {"sunset-to-sunrise" | "sunset-to-fajr"} */ (midnight),
    source,
  };
}

/**
 * One line of `hc_zmanim`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").Zman}
 */
function zman(cells) {
  const [id, englishName, hours, instant, missing, missingDay, depression, arcseconds] = cells;
  return {
    id: /** @type {import("./hyper-calendar.d.ts").ZmanId} */ (id),
    englishName: optional(englishName),
    hours: hours === "" ? null : decimal(hours, "hours"),
    instant: optionalInteger(instant, "instant"),
    missing: missingSolarEvent(missing, missingDay, depression, arcseconds),
  };
}

/**
 * The one line of `hc_edo_time`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").EdoTime}
 */
function edoTime(cells) {
  const [day, hour, name, romaji, strokes, branch, tenths, fraction, missing, missingDay, depression, arcseconds] =
    cells;
  return {
    day: optionalInteger(day, "day"),
    hour: optionalInteger(hour, "hour"),
    name: optional(name),
    romaji: optional(romaji),
    strokes: optionalInteger(strokes, "strokes"),
    branch: optional(branch),
    tenths: optionalInteger(tenths, "tenths"),
    fraction: fraction === "" ? null : decimal(fraction, "fraction"),
    missing: missingSolarEvent(missing, missingDay, depression, arcseconds),
  };
}

const DEFAULT_INITIAL_CAPACITY = 64 * 1024;

/**
 * An instantiated module, one method per export.
 *
 * Every `i64` the module returns arrives as a `BigInt` and leaves here as a
 * number, since every day number, year and timestamp the library deals in
 * fits one; an `i64` argument may be given as either. A method whose export
 * the build does not carry throws {@link HcError} `not-exported` when
 * called, not when loaded, so one page can load any layer.
 */
export class HyperCalendar {
  /** @type {Record<string, any>} */
  #exports;
  /** @type {number} */
  #initialCapacity;

  /**
   * @param {WebAssembly.Instance | {exports: Record<string, any>}} instance
   * @param {import("./hyper-calendar.d.ts").LoadOptions} [options]
   */
  constructor(instance, options = {}) {
    const exports = instance.exports;
    if (!(exports?.memory instanceof WebAssembly.Memory)) {
      raise("not a hyper-calendar module: no exported memory");
    }
    for (const required of ["hc_alloc", "hc_free", "hc_version"]) {
      if (typeof exports[required] !== "function") {
        raise(`not a hyper-calendar module: no ${required}`);
      }
    }
    const capacity = options.initialCapacity ?? DEFAULT_INITIAL_CAPACITY;
    if (!Number.isInteger(capacity) || capacity < 1) {
      raise(`initialCapacity must be a positive integer, got ${String(capacity)}`);
    }
    this.#exports = exports;
    this.#initialCapacity = capacity;
  }

  /** The instance's raw exports, for a call this binding does not make. */
  get exports() {
    return this.#exports;
  }

  /** The module's linear memory. */
  get memory() {
    return /** @type {WebAssembly.Memory} */ (this.#exports.memory);
  }

  /**
   * Whether a method's export is in this build.
   *
   * @param {string} method
   * @returns {boolean}
   */
  has(method) {
    const entry = METHODS.find((candidate) => candidate.method === method);
    return entry !== undefined && typeof this.#exports[entry.export] === "function";
  }

  /**
   * The layers this build carries: every feature at least one of whose
   * exports is present.
   *
   * @returns {string[]}
   */
  layers() {
    const present = new Set();
    for (const entry of METHODS) {
      if (entry.feature !== null && typeof this.#exports[entry.export] === "function") {
        present.add(entry.feature);
      }
    }
    return [...present];
  }

  /**
   * @param {string} exportName
   * @returns {Function}
   */
  #export(exportName) {
    const fn = this.#exports[exportName];
    if (typeof fn !== "function") {
      throw new HcError("not-exported", {
        export: exportName,
        message: `${exportName} is not in this build of the module; it needs the feature the README's table gives`,
      });
    }
    return fn;
  }

  /**
   * A block of linear memory, as a pointer, from `hc_alloc`.
   *
   * @param {number} len
   * @returns {number}
   */
  alloc(len) {
    if (len === 0) {
      return 0;
    }
    const pointer = this.#export("hc_alloc")(len);
    if (pointer === 0) {
      throw new HcError("allocation-failed", {
        export: "hc_alloc",
        message: `hc_alloc could not provide ${len} bytes`,
      });
    }
    return pointer;
  }

  /**
   * Return a block to `hc_free`, with the length it was allocated with.
   *
   * @param {number} pointer
   * @param {number} len
   */
  free(pointer, len) {
    if (pointer !== 0 && len !== 0) {
      this.#export("hc_free")(pointer, len);
    }
  }

  /**
   * UTF-8 bytes copied into the module.
   *
   * @param {Uint8Array} bytes
   * @returns {{pointer: number, len: number}}
   */
  #write(bytes) {
    const pointer = this.alloc(bytes.length);
    if (pointer !== 0) {
      new Uint8Array(this.memory.buffer, pointer, bytes.length).set(bytes);
    }
    return { pointer, len: bytes.length };
  }

  /**
   * Run `body` with `text` in linear memory, then free it.
   *
   * @template T
   * @param {string} text
   * @param {string} what
   * @param {(pointer: number, len: number) => T} body
   * @returns {T}
   */
  #withText(text, what, body) {
    if (typeof text !== "string") {
      raise(`${what} must be a string, got ${typeof text}`);
    }
    const { pointer, len } = this.#write(encoder.encode(text));
    try {
      return body(pointer, len);
    } finally {
      this.free(pointer, len);
    }
  }

  /**
   * Run `body` with `bytes` in linear memory, then free it.
   *
   * @template T
   * @param {Uint8Array | ArrayBuffer} bytes
   * @param {string} what
   * @param {(pointer: number, len: number) => T} body
   * @returns {T}
   */
  #withBytes(bytes, what, body) {
    const view = bytes instanceof ArrayBuffer
      ? new Uint8Array(bytes)
      : bytes instanceof Uint8Array
        ? bytes
        : raise(`${what} must be a Uint8Array or an ArrayBuffer`);
    const { pointer, len } = this.#write(view);
    try {
      return body(pointer, len);
    } finally {
      this.free(pointer, len);
    }
  }

  /**
   * The text an export writes.
   *
   * The first call offers a buffer of the initial capacity, which every
   * ordinary answer fits, so the module computes its text once. When that
   * is `HC_ERR_BUFFER_TOO_SMALL`, an export that measures is asked the
   * exact length with a null buffer and called again; one that cannot
   * measure (`hc_version`, `hc_format_iso_date`) is offered double.
   *
   * @param {string} exportName
   * @param {(buffer: number, capacity: number) => bigint} call
   * @param {boolean} measures
   * @returns {string}
   */
  #text(exportName, call, measures) {
    let capacity = this.#initialCapacity;
    for (;;) {
      const pointer = this.alloc(capacity);
      let result;
      try {
        result = call(pointer, capacity);
        if (result !== HC_ERR_BUFFER_TOO_SMALL) {
          const len = toNumber(result, exportName);
          return decoder.decode(new Uint8Array(this.memory.buffer, pointer, len));
        }
      } finally {
        this.free(pointer, capacity);
      }
      if (measures) {
        capacity = toNumber(call(0, 0), exportName);
        if (capacity === 0) {
          return "";
        }
      } else {
        capacity *= 2;
      }
    }
  }

  /** The library version. */
  version() {
    const fn = this.#export("hc_version");
    return this.#text("hc_version", (buffer, capacity) => fn(buffer, capacity), false);
  }

  /**
   * The fixed day number of a proleptic Gregorian date.
   *
   * @param {number | bigint} year
   * @param {number} month
   * @param {number} day
   * @returns {number}
   */
  gregorianToFixed(year, month, day) {
    const fn = this.#export("hc_gregorian_to_fixed");
    return toNumber(fn(toI64(year, "year"), toU32(month, "month"), toU32(day, "day")), "hc_gregorian_to_fixed");
  }

  /**
   * The Gregorian year on a fixed day.
   *
   * @param {number | bigint} fixed
   * @returns {number}
   */
  gregorianYear(fixed) {
    return toNumber(this.#export("hc_gregorian_year")(toI64(fixed, "fixed")), "hc_gregorian_year");
  }

  /**
   * The Gregorian month on a fixed day, 1 through 12.
   *
   * @param {number | bigint} fixed
   * @returns {number}
   */
  gregorianMonth(fixed) {
    return toNumber(this.#export("hc_gregorian_month")(toI64(fixed, "fixed")), "hc_gregorian_month");
  }

  /**
   * The Gregorian day of the month on a fixed day.
   *
   * @param {number | bigint} fixed
   * @returns {number}
   */
  gregorianDay(fixed) {
    return toNumber(this.#export("hc_gregorian_day")(toI64(fixed, "fixed")), "hc_gregorian_day");
  }

  /**
   * The ISO weekday of a fixed day, Monday = 1 through Sunday = 7.
   *
   * @param {number | bigint} fixed
   * @returns {number}
   */
  weekday(fixed) {
    return toNumber(this.#export("hc_weekday")(toI64(fixed, "fixed")), "hc_weekday");
  }

  /**
   * The 1-based day of the year on a fixed day.
   *
   * @param {number | bigint} fixed
   * @returns {number}
   */
  dayOfYear(fixed) {
    return toNumber(this.#export("hc_day_of_year")(toI64(fixed, "fixed")), "hc_day_of_year");
  }

  /**
   * Whether the Gregorian year on a fixed day is a leap year.
   *
   * @param {number | bigint} fixed
   * @returns {boolean}
   */
  isLeapYear(fixed) {
    return toNumber(this.#export("hc_is_leap_year")(toI64(fixed, "fixed")), "hc_is_leap_year") === 1;
  }

  /**
   * The fixed day a POSIX timestamp falls on, in UTC.
   *
   * @param {number | bigint} unixSeconds
   * @returns {number}
   */
  fixedFromUnix(unixSeconds) {
    return toNumber(this.#export("hc_fixed_from_unix")(toI64(unixSeconds, "unixSeconds")), "hc_fixed_from_unix");
  }

  /**
   * `TAI - UTC` in whole seconds at a POSIX timestamp. `strict` refuses
   * to answer before 1961 and past the announced leap-second table with
   * `no-data`; otherwise the last published value holds.
   *
   * @param {number | bigint} unixSeconds
   * @param {boolean} [strict]
   * @returns {number}
   */
  taiMinusUtc(unixSeconds, strict = false) {
    const fn = this.#export("hc_tai_minus_utc");
    return toNumber(fn(toI64(unixSeconds, "unixSeconds"), strict ? 1 : 0), "hc_tai_minus_utc");
  }

  /**
   * Whether the UTC day containing a POSIX timestamp ends with an inserted
   * leap second.
   *
   * @param {number | bigint} unixSeconds
   * @returns {boolean}
   */
  dayHasLeapSecond(unixSeconds) {
    const fn = this.#export("hc_day_has_leap_second");
    return toNumber(fn(toI64(unixSeconds, "unixSeconds")), "hc_day_has_leap_second") === 1;
  }

  /**
   * The POSIX timestamp of midnight UTC on a fixed day. A day before
   * −104 165 947 503, whose midnight would read as a sentinel, or after
   * 106 751 991 886 463, whose midnight would overflow an `i64`, is
   * `out-of-range`.
   *
   * @param {number | bigint} fixed
   * @returns {number}
   */
  unixFromFixed(fixed) {
    return toNumber(this.#export("hc_unix_from_fixed")(toI64(fixed, "fixed")), "hc_unix_from_fixed");
  }

  /**
   * A fixed day as an ISO 8601 date.
   *
   * @param {number | bigint} fixed
   * @returns {string}
   */
  formatIsoDate(fixed) {
    const fn = this.#export("hc_format_iso_date");
    const day = toI64(fixed, "fixed");
    return this.#text("hc_format_iso_date", (buffer, capacity) => fn(day, buffer, capacity), false);
  }

  /**
   * The fixed day number of an ISO 8601 date.
   *
   * @param {string} text
   * @returns {number}
   */
  parseIsoDate(text) {
    const fn = this.#export("hc_parse_iso_date");
    return this.#withText(text, "text", (pointer, len) => toNumber(fn(pointer, len), "hc_parse_iso_date"));
  }
  /**
   * A POSIX timestamp as a TAI reading. `strict` refuses before 1961 and
   * past the leap-second table with `no-data`.
   *
   * @param {number | bigint} unixSeconds
   * @param {boolean} [strict]
   * @returns {import("./hyper-calendar.d.ts").TaiInstant}
   */
  taiFromUnix(unixSeconds, strict = false) {
    const fn = this.#export("hc_tai_from_unix");
    const seconds = toI64(unixSeconds, "unixSeconds");
    const text = this.#text("hc_tai_from_unix", (buffer, capacity) => fn(seconds, strict ? 1 : 0, buffer, capacity), true);
    return taiInstant(this.#oneLine("hc_tai_from_unix", text, COLUMNS.taiInstant));
  }

  /**
   * A whole TAI second as a UTC label: its POSIX second, and whether it is
   * an inserted leap second, named by the POSIX second after it.
   *
   * @param {number | bigint} taiSeconds
   * @param {boolean} [strict]
   * @returns {import("./hyper-calendar.d.ts").UtcLabel}
   */
  utcFromTai(taiSeconds, strict = false) {
    const fn = this.#export("hc_utc_from_tai");
    const seconds = toI64(taiSeconds, "taiSeconds");
    const text = this.#text("hc_utc_from_tai", (buffer, capacity) => fn(seconds, strict ? 1 : 0, buffer, capacity), true);
    return utcLabel(this.#oneLine("hc_utc_from_tai", text, COLUMNS.utcFromTai));
  }


  /**
   * One fixed day in every registered calendar, in registry order, with
   * the locale's vocabulary and the date as the locale writes it. `locale`
   * is a BCP 47 tag, or {@link NATIVE} for each calendar's own language;
   * a calendar the tag's data does not name is rendered in English, else
   * in the tag with the calendar's own names, never in the calendar's own
   * language, and `localeUsed` says which. A tag that
   * does not parse falls back to the root locale `und`, as the module
   * does — ask for `en` for English.
   *
   * @param {number | bigint} fixed
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").DescribedDay[]}
   */
  describeDay(fixed, locale = "und") {
    const fn = this.#export("hc_describe_day");
    const day = toI64(fixed, "fixed");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_describe_day", (buffer, capacity) => fn(day, pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.describeDay, "hc_describe_day").map(describedDay);
  }

  /**
   * The extra fields of one fixed day, one row a field: its identifier and
   * value for a program, and for a reader its label and the value as the
   * locale names it, with `inDate` saying whether {@link describeDay}'s
   * `formatted` already writes it. Every registered calendar's, or with
   * `id` only that calendar's; a calendar that refuses the day has none.
   * `locale` is as for {@link describeDay}.
   *
   * @param {number | bigint} fixed
   * @param {string} [locale]
   * @param {string} [id]
   * @returns {import("./hyper-calendar.d.ts").DayExtra[]}
   */
  dayExtras(fixed, locale = "und", id = "") {
    const fn = this.#export("hc_day_extras");
    const day = toI64(fixed, "fixed");
    const text = this.#withText(id, "id", (idPointer, idLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#text("hc_day_extras", (buffer, capacity) =>
          fn(day, idPointer, idLen, localePointer, localeLen, buffer, capacity), true)));
    return rows(text, COLUMNS.dayExtras, "hc_day_extras").map(dayExtra);
  }

  /**
   * The days from `from` up to but not including `to` as one calendar's
   * eras, years, months or days: spans touching end to start, each with
   * its label in the locale, from the start of the unit containing `from`
   * to at least `to`. A span the calendar refuses carries the refusal in
   * `error` instead of a label. `unit` is one of {@link UNITS} or its index.
   *
   * @param {string} id
   * @param {import("./hyper-calendar.d.ts").Unit | number} unit
   * @param {number | bigint} from
   * @param {number | bigint} to
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").CalendarUnit[]}
   */
  calendarUnits(id, unit, from, to, locale = "und") {
    const fn = this.#export("hc_calendar_units");
    const index = typeof unit === "string" ? UNITS.indexOf(unit) : unit;
    if (!Number.isInteger(index) || index < 0 || index >= UNITS.length) {
      raise(`unit must be one of ${UNITS.join(", ")} or its index, got ${String(unit)}`);
    }
    const start = toI64(from, "from");
    const end = toI64(to, "to");
    const text = this.#withText(id, "id", (idPointer, idLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#text("hc_calendar_units", (buffer, capacity) =>
          fn(idPointer, idLen, index, start, end, localePointer, localeLen, buffer, capacity), true)));
    return rows(text, COLUMNS.calendarUnits, "hc_calendar_units").map(calendarUnit);
  }

  /**
   * A date as the locale writes it in one calendar, read back: the
   * calendar's row of {@link describeDay} for the day the text names, and
   * that fixed day. `locale` is as for {@link describeDay}, and the text is
   * read in the locale that row is written in, so that what its
   * `formatted` writes, this reads. A text that is not one day is a row
   * whose `error` says why — `ambiguous`, `two-digit-year`,
   * `year-not-written`, `weekday-mismatch`, `not-recognised`, `empty`, or
   * the calendar's own refusal — and whose `fixed` is `null`.
   *
   * @param {string} calendar
   * @param {string} locale
   * @param {string} text
   * @returns {import("./hyper-calendar.d.ts").ParsedDate}
   */
  parseDate(calendar, locale, text) {
    const fn = this.#export("hc_parse_date");
    const line = this.#withText(calendar, "calendar", (calendarPointer, calendarLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#withText(text, "text", (textPointer, textLen) =>
          this.#text("hc_parse_date", (buffer, capacity) =>
            fn(calendarPointer, calendarLen, localePointer, localeLen, textPointer, textLen, buffer, capacity), true))));
    return parsedDate(this.#oneLine("hc_parse_date", line, COLUMNS.parseDate));
  }

  /**
   * Every registered calendar, in registry order: what the locale calls
   * it, its range, which units it has, the languages its sources are
   * written in, and its standing on `today`.
   *
   * @param {number | bigint} today
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").CalendarEntry[]}
   */
  calendars(today, locale = "und") {
    const fn = this.#export("hc_calendars");
    const day = toI64(today, "today");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_calendars", (buffer, capacity) => fn(day, pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.calendars, "hc_calendars").map(calendarEntry);
  }

  /**
   * Every registered calendar by name alone, in registry order: what the
   * locale calls it, as {@link HyperCalendar#calendars} has it, its
   * English name, the locale the name is in, and the crate that registers
   * it. Nothing is converted, so this is the call for a menu of calendars,
   * which a page asks for far more often than it describes a day.
   *
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").CalendarListEntry[]}
   */
  calendarList(locale = "und") {
    const fn = this.#export("hc_calendar_list");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_calendar_list", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.calendarList, "hc_calendar_list").map(calendarListEntry);
  }

  /**
   * Every locale the module carries, in tag order, with what each names.
   *
   * @returns {import("./hyper-calendar.d.ts").LocaleEntry[]}
   */
  locales() {
    const fn = this.#export("hc_locales");
    const text = this.#text("hc_locales", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(text, COLUMNS.locales, "hc_locales").map(localeEntry);
  }

  /**
   * The ISO weekday of the first day of the week in a locale, Monday = 1
   * through Sunday = 7, from CLDR 48's week data: a `-u-fw-` key, else the
   * tag's region, else the region the language's likely subtags give. A
   * tag that does not parse is the root locale `und`, whose week begins on
   * Monday.
   *
   * @param {string} [locale]
   * @returns {number}
   */
  firstDayOfWeek(locale = "und") {
    const fn = this.#export("hc_first_day_of_week");
    return this.#withText(locale, "locale", (pointer, len) =>
      toNumber(fn(pointer, len), "hc_first_day_of_week"));
  }

  /**
   * The steps by which a country adopted the Gregorian calendar, oldest
   * first. `region` is an ISO 3166-1 alpha-2 code, in either case; a code
   * the module does not know answers with no steps.
   *
   * @param {string} region
   * @returns {import("./hyper-calendar.d.ts").GregorianAdoption[]}
   */
  gregorianAdoption(region) {
    const fn = this.#export("hc_gregorian_adoption");
    const text = this.#withText(region, "region", (pointer, len) =>
      this.#text("hc_gregorian_adoption", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.gregorianAdoption, "hc_gregorian_adoption").map(gregorianAdoption);
  }

  /**
   * Whether a fixed day is a day off in a holiday table. `code` names the
   * table and `region`, which may be empty, a subdivision.
   *
   * @param {string} code
   * @param {string} region
   * @param {number | bigint} fixed
   * @returns {boolean}
   */
  holidayIsDayOff(code, region, fixed) {
    const fn = this.#export("hc_holiday_is_day_off");
    const day = toI64(fixed, "fixed");
    return this.#withText(code, "code", (codePointer, codeLen) =>
      this.#withText(region, "region", (regionPointer, regionLen) =>
        toNumber(fn(codePointer, codeLen, regionPointer, regionLen, day), "hc_holiday_is_day_off") === 1));
  }

  /**
   * The holidays of a Gregorian year in a table.
   *
   * @param {string} code
   * @param {string} region
   * @param {number | bigint} year
   * @returns {import("./hyper-calendar.d.ts").HolidayInYear[]}
   */
  holidaysInYear(code, region, year) {
    const fn = this.#export("hc_holidays_in_year");
    const y = toI64(year, "year");
    const text = this.#withText(code, "code", (codePointer, codeLen) =>
      this.#withText(region, "region", (regionPointer, regionLen) =>
        this.#text("hc_holidays_in_year", (buffer, capacity) =>
          fn(codePointer, codeLen, regionPointer, regionLen, y, buffer, capacity), true)));
    return rows(text, COLUMNS.holidaysInYear, "hc_holidays_in_year").map(holidayInYear);
  }

  /**
   * The identifier of every holiday table: countries, then exchanges,
   * traditions and the international sets.
   *
   * @returns {string[]}
   */
  holidayCodes() {
    const fn = this.#export("hc_holiday_codes");
    const text = this.#text("hc_holiday_codes", (buffer, capacity) => fn(buffer, capacity), true);
    return text.split("\n").filter((line) => line.length > 0);
  }

  /**
   * Every holiday on one fixed day across every table, in the order
   * {@link holidayCodes} lists them, each evaluated nationwide.
   *
   * @param {number | bigint} fixed
   * @returns {import("./hyper-calendar.d.ts").HolidayOn[]}
   */
  holidaysOn(fixed) {
    const fn = this.#export("hc_holidays_on");
    const day = toI64(fixed, "fixed");
    const text = this.#text("hc_holidays_on", (buffer, capacity) => fn(day, buffer, capacity), true);
    return rows(text, COLUMNS.holidaysOn, "hc_holidays_on").map(holidayOn);
  }

  /**
   * The solar term in effect on a fixed day at a meridian: a name
   * (`universal`, `japan`, `china`, `korea`, `india`, `china-before-1929`)
   * or a longitude in degrees east of Greenwich.
   *
   * @param {number | bigint} fixed
   * @param {string | number} [meridian]
   * @returns {import("./hyper-calendar.d.ts").TermInEffect}
   */
  termInEffect(fixed, meridian = "universal") {
    return this.#almanac("hc_term_in_effect", fixed, meridian);
  }

  /**
   * The pentad (候) in effect on a fixed day at a meridian, as
   * {@link termInEffect}.
   *
   * @param {number | bigint} fixed
   * @param {string | number} [meridian]
   * @returns {import("./hyper-calendar.d.ts").TermInEffect}
   */
  pentadInEffect(fixed, meridian = "universal") {
    return this.#almanac("hc_pentad_in_effect", fixed, meridian);
  }
  /**
   * The fixed day of 寒食, the Cold Food Day, of a Gregorian year under a
   * named reckoning: `hanshi-solstice-105`, `hanshi-eve-of-qingming` or
   * `hansik`. Another name is `unknown`; a year outside −999 to 3000 is
   * `out-of-range`.
   *
   * @param {import("./hyper-calendar.d.ts").ColdFoodConvention} convention
   * @param {number | bigint} year
   * @returns {number}
   */
  coldFoodDay(convention, year) {
    const fn = this.#export("hc_cold_food_day");
    const y = toI64(year, "year");
    return this.#withText(convention, "convention", (pointer, len) =>
      toNumber(fn(pointer, len, y), "hc_cold_food_day"));
  }
  /**
   * The fixed day of 入梅 or 出梅 of a Gregorian year by a rule of the
   * Chinese almanac, with its solar term at a meridian.
   *
   * @param {import("./hyper-calendar.d.ts").PlumRainRule} rule
   * @param {number | bigint} year
   * @param {string} [meridian]
   * @returns {number}
   */
  plumRains(rule, year, meridian = "") {
    const fn = this.#export("hc_plum_rains");
    const y = toI64(year, "year");
    return this.#withText(rule, "rule", (rulePointer, ruleLen) =>
      this.#withText(meridian, "meridian", (meridianPointer, meridianLen) =>
        toNumber(fn(rulePointer, ruleLen, y, meridianPointer, meridianLen), "hc_plum_rains")));
  }



  /**
   * @param {string} exportName
   * @param {number | bigint} fixed
   * @param {string | number} meridian
   * @returns {import("./hyper-calendar.d.ts").TermInEffect}
   */
  #almanac(exportName, fixed, meridian) {
    const fn = this.#export(exportName);
    const day = toI64(fixed, "fixed");
    const name = typeof meridian === "number" ? String(meridian) : meridian;
    const text = this.#withText(name, "meridian", (pointer, len) =>
      this.#text(exportName, (buffer, capacity) => fn(day, pointer, len, buffer, capacity), true));
    const lines = rows(text, COLUMNS.term, exportName);
    if (lines.length !== 1) {
      throw new HcError("malformed", {
        export: exportName,
        message: `${exportName} wrote ${lines.length} lines, not one`,
      });
    }
    return termInEffect(lines[0]);
  }

  /**
   * A moment some years before the present — the present as `hc-deep-time`
   * defines it, the Planck 2018 age of the universe — placed in every
   * chronology at once. Negative years are the future; a moment up to a
   * century ahead is still in the intervals that end at the present. The
   * geologic rows carry the chart's own name in `locale` where it has one,
   * and the cosmic and archaeological rows an established term where one
   * was read.
   *
   * @param {number} yearsAgo
   * @param {number} [stdDevYears]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").DeepTimeRow[]}
   */
  placeYearsAgo(yearsAgo, stdDevYears = 0, locale = "und") {
    const fn = this.#export("hc_place_years_ago");
    const years = toF64(yearsAgo, "yearsAgo");
    const stdDev = toF64(stdDevYears, "stdDevYears");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_place_years_ago", (buffer, capacity) => fn(years, stdDev, pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.deepTime, "hc_place_years_ago").map(deepTimeRow);
  }

  /**
   * Every cosmic epoch, Big Bang to the present, then every dated cosmic
   * event, oldest first. `localisedName` is the established term in
   * `locale` where one was read, and `null` elsewhere.
   *
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").DeepTimeRow[]}
   */
  cosmicEvents(locale = "und") {
    const fn = this.#export("hc_cosmic_events");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_cosmic_events", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.deepTime, "hc_cosmic_events").map(deepTimeRow);
  }

  /**
   * Every claim to the earliest evidence of life, of *Homo sapiens* and of
   * writing, grouped by landmark and oldest first, each in the shape its
   * source dates it: an age, a minimum age with no `start`, or a range. A
   * σ is `null` where the source does not state one.
   *
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").EarliestEvidenceRow[]}
   */
  earliestEvidence(locale = "und") {
    const fn = this.#export("hc_earliest_evidence");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_earliest_evidence", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.deepTime, "hc_earliest_evidence").map(earliestEvidenceRow);
  }

  /**
   * Every conventional archaeological period, youngest first, with its
   * region as the scope.
   *
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").DeepTimeRow[]}
   */
  archaeologicalPeriods(locale = "und") {
    const fn = this.#export("hc_archaeological_periods");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_archaeological_periods", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.deepTime, "hc_archaeological_periods").map(deepTimeRow);
  }

  /**
   * Every dated event of the far future, soonest first, in years from now,
   * with the kind of prediction as the scope; an experimental bound has no
   * `end`.
   *
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").DeepTimeRow[]}
   */
  futureEvents(locale = "und") {
    const fn = this.#export("hc_future_events");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_future_events", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.deepTime, "hc_future_events").map(deepTimeRow);
  }

  /**
   * Every interval of one rank of the geologic time scale, youngest first.
   * `rank` is a name (`eon`, `era`, `period`, `epoch`, `age`) or its number
   * from 0. Each carries the chart's own name in `locale` where it has one.
   *
   * @param {import("./hyper-calendar.d.ts").GeologicRank | number} rank
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").DeepTimeRow[]}
   */
  geologicIntervals(rank, locale = "und") {
    const fn = this.#export("hc_geologic_intervals");
    const number = typeof rank === "string" ? GEOLOGIC_RANKS.indexOf(rank) : rank;
    if (number === -1) {
      raise(`rank must be one of ${GEOLOGIC_RANKS.join(", ")} or 0 to 4, got ${JSON.stringify(rank)}`);
    }
    const r = toU32(number, "rank");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_geologic_intervals", (buffer, capacity) => fn(r, pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.deepTime, "hc_geologic_intervals").map(deepTimeRow);
  }

  /**
   * The fixed day a POSIX timestamp falls on by the wall clock of a zone:
   * one loaded with {@link loadZone}, or one of the seventeen built in.
   *
   * @param {number | bigint} unixSeconds
   * @param {string} zone
   * @returns {number}
   */
  fixedFromUnixInZone(unixSeconds, zone) {
    const fn = this.#export("hc_fixed_from_unix_in_zone");
    const instant = toI64(unixSeconds, "unixSeconds");
    return this.#withText(zone, "zone", (pointer, len) =>
      toNumber(fn(instant, pointer, len), "hc_fixed_from_unix_in_zone"));
  }

  /**
   * The POSIX timestamp at which a fixed day begins by the wall clock of a
   * zone: its local midnight, the first instant after a gap that swallows
   * it, or the earlier of two midnights. The range is `unixFromFixed`'s,
   * each end moved by at most a day by the zone's offset; outside it,
   * `out-of-range`.
   *
   * @param {number | bigint} fixed
   * @param {string} zone
   * @returns {number}
   */
  unixFromFixedInZone(fixed, zone) {
    const fn = this.#export("hc_unix_from_fixed_in_zone");
    const day = toI64(fixed, "fixed");
    return this.#withText(zone, "zone", (pointer, len) =>
      toNumber(fn(day, pointer, len), "hc_unix_from_fixed_in_zone"));
  }

  /**
   * The offset a zone keeps at a POSIX timestamp, from the rules
   * {@link fixedFromUnixInZone} reads for the name: seconds east of UTC,
   * whether the rules call it daylight saving or summer time, their
   * abbreviation (`null` where they give a numeric one such as `+0545`),
   * the next transition and the offset after it (`null` where the rules
   * have none), and whether the `builtin` rules or a `loaded` file
   * answered. A zone nobody knows is `unknown`.
   *
   * @param {string} zone
   * @param {number | bigint} unixSeconds
   * @returns {import("./hyper-calendar.d.ts").ZoneOffset}
   */
  zoneOffset(zone, unixSeconds) {
    const fn = this.#export("hc_zone_offset");
    const instant = toI64(unixSeconds, "unixSeconds");
    const text = this.#withText(zone, "zone", (pointer, len) =>
      this.#text("hc_zone_offset", (buffer, capacity) => fn(pointer, len, instant, buffer, capacity), true));
    return zoneOffset(this.#oneLine("hc_zone_offset", text, COLUMNS.zoneOffset));
  }

  /**
   * Give the module a zone's TZif file under an IANA name; the two
   * `InZone` methods, {@link zoneOffset} and {@link radioEncode}'s
   * `zone:` then answer for that name with the file's history,
   * outranking a built-in zone of the same name. The bytes are copied.
   *
   * @param {string} name
   * @param {Uint8Array | ArrayBuffer} tzif
   */
  loadZone(name, tzif) {
    const fn = this.#export("hc_zone_load");
    this.#withText(name, "name", (namePointer, nameLen) =>
      this.#withBytes(tzif, "tzif", (bytesPointer, bytesLen) =>
        checked(fn(namePointer, nameLen, bytesPointer, bytesLen), "hc_zone_load")));
  }

  /**
   * Every zone of the IANA database's `zone1970.tab`, in its order, with
   * the latitude and longitude of its principal location in decimal
   * degrees, its countries, the table's comment, and its CLDR 48 exemplar
   * city in the locale. The city is English, with `localeUsed` `en`, in a
   * build without the `calendars` layer and for a zone the locale has no
   * city for.
   *
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").ZoneLocation[]}
   */
  zones(locale = "und") {
    const fn = this.#export("hc_zones");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_zones", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.zones, "hc_zones").map(zoneLocation);
  }

  /**
   * Where one zone is, as {@link zones} describes it. `zone` is an IANA
   * name in any case: a zone, a link `zone.tab` gives a place of its own
   * (`Europe/Oslo`), or another link of `backward` (`Asia/Calcutta`),
   * which answers with the row it leads to, so that `zone` is then
   * `Asia/Kolkata`. A name that places nothing, such as `UTC`, is
   * `unknown`.
   *
   * @param {string} zone
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").ZoneLocation}
   */
  zoneLocation(zone, locale = "und") {
    const fn = this.#export("hc_zone_location");
    const text = this.#withText(zone, "zone", (zonePointer, zoneLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#text("hc_zone_location", (buffer, capacity) =>
          fn(zonePointer, zoneLen, localePointer, localeLen, buffer, capacity), true)));
    return zoneLocation(this.#oneLine("hc_zone_location", text, COLUMNS.zones));
  }

  /**
   * The Sun and the Moon at a POSIX instant, read as Universal Time: their
   * positions, the Moon's elongation and lit fraction, the new moons either
   * side, and ΔT with the regime it was answered from. An instant whose
   * proleptic Gregorian year is outside −1000 through 3000 is `out-of-range`.
   *
   * @param {number | bigint} unixSeconds
   * @returns {import("./hyper-calendar.d.ts").Sky}
   */
  skyAt(unixSeconds) {
    const fn = this.#export("hc_sky_at");
    const instant = toI64(unixSeconds, "unixSeconds");
    const text = this.#text("hc_sky_at", (buffer, capacity) => fn(instant, buffer, capacity), true);
    const lines = rows(text, COLUMNS.sky, "hc_sky_at");
    if (lines.length !== 1) {
      throw new HcError("malformed", {
        export: "hc_sky_at",
        message: `hc_sky_at wrote ${lines.length} lines, not one`,
      });
    }
    return sky(lines[0]);
  }

  /**
   * Every solar term whose instant falls in `[from, to)`, in time order.
   * A span outside −1000 through 3000 or longer than 400 years is
   * `out-of-range`; one that ends at or before it begins is empty.
   *
   * @param {number | bigint} fromUnix
   * @param {number | bigint} toUnix
   * @returns {import("./hyper-calendar.d.ts").SkyEvent[]}
   */
  solarTermsBetween(fromUnix, toUnix) {
    return this.#between("hc_solar_terms_between", fromUnix, toUnix);
  }

  /**
   * Every new moon, first quarter, full moon and last quarter whose
   * instant falls in `[from, to)`, in time order, as {@link solarTermsBetween}.
   *
   * @param {number | bigint} fromUnix
   * @param {number | bigint} toUnix
   * @returns {import("./hyper-calendar.d.ts").SkyEvent[]}
   */
  moonPhasesBetween(fromUnix, toUnix) {
    return this.#between("hc_moon_phases_between", fromUnix, toUnix);
  }

  /**
   * The decan the Sun is in at a POSIX instant, with its sign and ruler.
   *
   * @param {number | bigint} unixSeconds
   * @returns {import("./hyper-calendar.d.ts").Decan}
   */
  decanAt(unixSeconds) {
    const fn = this.#export("hc_decan_at");
    const instant = toI64(unixSeconds, "unixSeconds");
    const text = this.#text("hc_decan_at", (buffer, capacity) => fn(instant, buffer, capacity), true);
    return decanLine(this.#oneLine("hc_decan_at", text, COLUMNS.decan));
  }

  /**
   * @param {string} exportName
   * @param {number | bigint} fromUnix
   * @param {number | bigint} toUnix
   * @returns {import("./hyper-calendar.d.ts").SkyEvent[]}
   */
  #between(exportName, fromUnix, toUnix) {
    const fn = this.#export(exportName);
    const from = toI64(fromUnix, "fromUnix");
    const to = toI64(toUnix, "toUnix");
    const text = this.#text(exportName, (buffer, capacity) => fn(from, to, buffer, capacity), true);
    return rows(text, COLUMNS.skyEvent, exportName).map(skyEvent);
  }

  /**
   * Earth's orbital elements — the eccentricity, the obliquity, the
   * longitude of perihelion from the moving equinox and the climatic
   * precession, each with its spread — and the June insolation at 65° N,
   * at an epoch in years before 1950, negative for the future, from
   * Berger's 1978 series. The insolation is for the solar constant the
   * line carries, and is proportional to it. An epoch beyond a million
   * years either side of 1950 is `out-of-range`.
   *
   * @param {number} yearsBefore1950
   * @returns {import("./hyper-calendar.d.ts").Orbit}
   */
  orbitAt(yearsBefore1950) {
    const fn = this.#export("hc_orbit_at");
    const epoch = toF64(yearsBefore1950, "yearsBefore1950");
    const text = this.#text("hc_orbit_at", (buffer, capacity) => fn(epoch, buffer, capacity), true);
    const lines = rows(text, COLUMNS.orbit, "hc_orbit_at");
    if (lines.length !== 1) {
      throw new HcError("malformed", {
        export: "hc_orbit_at",
        message: `hc_orbit_at wrote ${lines.length} lines, not one`,
      });
    }
    return orbit(lines[0]);
  }

  /**
   * {@link orbitAt} at every epoch from `from` to `to` in steps of `step`
   * years — `from`, `from + step`, and so on, every one at or before `to` —
   * each with its epoch, in one call. Both ends have to lie within a
   * million years either side of 1950 and `step` has to be positive, and
   * at most 10 000 samples are answered, else `out-of-range`; a `to`
   * before `from` is an empty list.
   *
   * @param {number} fromYearsBefore1950
   * @param {number} toYearsBefore1950
   * @param {number} stepYears
   * @returns {import("./hyper-calendar.d.ts").OrbitSample[]}
   */
  orbitSeries(fromYearsBefore1950, toYearsBefore1950, stepYears) {
    const fn = this.#export("hc_orbit_series");
    const from = toF64(fromYearsBefore1950, "fromYearsBefore1950");
    const to = toF64(toYearsBefore1950, "toYearsBefore1950");
    const step = toF64(stepYears, "stepYears");
    const text = this.#text("hc_orbit_series", (buffer, capacity) => fn(from, to, step, buffer, capacity), true);
    return rows(text, COLUMNS.orbitSeries, "hc_orbit_series").map(orbitSample);
  }

  /**
   * The one line an export wrote, as cells, or `malformed` for any other
   * number of lines.
   *
   * @param {string} exportName
   * @param {string} text
   * @param {ReadonlyArray<string>} columns
   * @returns {string[]}
   */
  #oneLine(exportName, text, columns) {
    const lines = rows(text, columns, exportName);
    if (lines.length !== 1) {
      throw new HcError("malformed", {
        export: exportName,
        message: `${exportName} wrote ${lines.length} lines, not one`,
      });
    }
    return lines[0];
  }

  /**
   * A TAI instant — whole seconds from 1970-01-01 00:00:00 TAI and the
   * attoseconds into that second — as a TAI64 (`tai64`), TAI64N (`tai64n`)
   * or TAI64NA (`tai64na`) label in lower-case hexadecimal.
   *
   * @param {number | bigint} taiSeconds
   * @param {number | bigint} attoseconds
   * @param {import("./hyper-calendar.d.ts").Tai64Format} format
   * @returns {string}
   */
  tai64Encode(taiSeconds, attoseconds, format) {
    const fn = this.#export("hc_tai64_encode");
    const seconds = toI64(taiSeconds, "taiSeconds");
    const attos = toU64(attoseconds, "attoseconds");
    const text = this.#withText(format, "format", (pointer, len) =>
      this.#text("hc_tai64_encode", (buffer, capacity) => fn(seconds, attos, pointer, len, buffer, capacity), true));
    return this.#oneLine("hc_tai64_encode", text, ["label"])[0];
  }

  /**
   * A TAI64, TAI64N or TAI64NA label in hexadecimal read back: its format
   * and the TAI instant it names, both parts as `BigInt`s.
   *
   * @param {string} hex
   * @returns {import("./hyper-calendar.d.ts").Tai64Label}
   */
  tai64Decode(hex) {
    const fn = this.#export("hc_tai64_decode");
    const text = this.#withText(hex, "hex", (pointer, len) =>
      this.#text("hc_tai64_decode", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return tai64Label(this.#oneLine("hc_tai64_decode", text, COLUMNS.tai64));
  }

  /**
   * The full GNSS week, the broadcast week and the time of week of a TAI
   * instant under a week-number field: `gps-lnav-week`, `gps-cnav-week`,
   * `galileo-week`, `beidou-week` or `navic-week`.
   *
   * @param {import("./hyper-calendar.d.ts").GnssNumbering} numbering
   * @param {number | bigint} taiSeconds
   * @param {number | bigint} [attoseconds]
   * @returns {import("./hyper-calendar.d.ts").GnssWeek}
   */
  gnssWeek(numbering, taiSeconds, attoseconds = 0) {
    const fn = this.#export("hc_gnss_week");
    const seconds = toI64(taiSeconds, "taiSeconds");
    const attos = toU64(attoseconds, "attoseconds");
    const text = this.#withText(numbering, "numbering", (pointer, len) =>
      this.#text("hc_gnss_week", (buffer, capacity) => fn(pointer, len, seconds, attos, buffer, capacity), true));
    return gnssWeek(this.#oneLine("hc_gnss_week", text, COLUMNS.gnssWeek));
  }

  /**
   * The TAI instant of a full GNSS week and a time of week.
   *
   * @param {import("./hyper-calendar.d.ts").GnssNumbering} numbering
   * @param {number} week
   * @param {number} towSeconds
   * @param {number | bigint} [towAttoseconds]
   * @returns {import("./hyper-calendar.d.ts").TaiInstant}
   */
  gnssToTai(numbering, week, towSeconds, towAttoseconds = 0) {
    const fn = this.#export("hc_gnss_to_tai");
    const w = toU32(week, "week");
    const tow = toU32(towSeconds, "towSeconds");
    const attos = toU64(towAttoseconds, "towAttoseconds");
    const text = this.#withText(numbering, "numbering", (pointer, len) =>
      this.#text("hc_gnss_to_tai", (buffer, capacity) => fn(pointer, len, w, tow, attos, buffer, capacity), true));
    return taiInstant(this.#oneLine("hc_gnss_to_tai", text, COLUMNS.taiInstant));
  }

  /**
   * The full GNSS week a broadcast week names: by `not-before`, the first
   * at or after the reference's week; by `nearest`, the one within half a
   * rollover of it. The reference is whole TAI seconds.
   *
   * @param {import("./hyper-calendar.d.ts").GnssNumbering} numbering
   * @param {number} broadcast
   * @param {import("./hyper-calendar.d.ts").RolloverRule} rule
   * @param {number | bigint} referenceTaiSeconds
   * @returns {number}
   */
  gnssResolveWeek(numbering, broadcast, rule, referenceTaiSeconds) {
    const fn = this.#export("hc_gnss_resolve_week");
    const b = toU32(broadcast, "broadcast");
    const reference = toI64(referenceTaiSeconds, "referenceTaiSeconds");
    return this.#withText(numbering, "numbering", (numberingPointer, numberingLen) =>
      this.#withText(rule, "rule", (rulePointer, ruleLen) =>
        toNumber(fn(numberingPointer, numberingLen, b, rulePointer, ruleLen, reference), "hc_gnss_resolve_week")));
  }

  /**
   * GLONASS's four-year interval N4 and day N_T at a TAI instant. `strict`
   * refuses outside the leap-second table with `no-data`.
   *
   * @param {number | bigint} taiSeconds
   * @param {number | bigint} [attoseconds]
   * @param {boolean} [strict]
   * @returns {import("./hyper-calendar.d.ts").GlonassDate}
   */
  glonassDate(taiSeconds, attoseconds = 0, strict = false) {
    const fn = this.#export("hc_glonass_date");
    const seconds = toI64(taiSeconds, "taiSeconds");
    const attos = toU64(attoseconds, "attoseconds");
    const text = this.#text("hc_glonass_date", (buffer, capacity) =>
      fn(seconds, attos, strict ? 1 : 0, buffer, capacity), true);
    return glonassDate(this.#oneLine("hc_glonass_date", text, COLUMNS.glonassDate));
  }

  /**
   * The fixed day and the seconds into it of an OLE Automation date; a
   * negative value's fraction is a magnitude, so −1.25 is 06:00 on
   * 29 December 1899.
   *
   * @param {number} value
   * @returns {import("./hyper-calendar.d.ts").OleAutomationDay}
   */
  fixedFromOleAutomation(value) {
    const fn = this.#export("hc_fixed_from_ole_automation");
    const v = toF64(value, "value");
    const text = this.#text("hc_fixed_from_ole_automation", (buffer, capacity) => fn(v, buffer, capacity), true);
    return oleAutomationDay(this.#oneLine("hc_fixed_from_ole_automation", text, COLUMNS.oleAutomation));
  }

  /**
   * The OLE Automation date of a fixed day and a time of day in seconds.
   *
   * @param {number | bigint} fixed
   * @param {number} [secondsOfDay]
   * @returns {number}
   */
  oleAutomationFromFixed(fixed, secondsOfDay = 0) {
    const fn = this.#export("hc_ole_automation_from_fixed");
    const day = toI64(fixed, "fixed");
    const seconds = toF64(secondsOfDay, "secondsOfDay");
    const text = this.#text("hc_ole_automation_from_fixed", (buffer, capacity) => fn(day, seconds, buffer, capacity), true);
    return decimal(this.#oneLine("hc_ole_automation_from_fixed", text, COLUMNS.value)[0], "value");
  }

  /**
   * What an Excel 1900 serial names: its fixed day, or, for serial 60,
   * which Excel counts as 29 February 1900, `phantom` and no day.
   *
   * @param {number | bigint} serial
   * @returns {import("./hyper-calendar.d.ts").Excel1900Day}
   */
  excel1900Day(serial) {
    const fn = this.#export("hc_excel_1900_day");
    const s = toI64(serial, "serial");
    const text = this.#text("hc_excel_1900_day", (buffer, capacity) => fn(s, buffer, capacity), true);
    return excel1900Day(this.#oneLine("hc_excel_1900_day", text, COLUMNS.excel1900Day));
  }

  /**
   * A POSIX instant as a TAI64 (`tai64`) or TAI64N (`tai64n`) label in the
   * `tai64-posix-plus-10` convention, 2⁶² + 10 + the POSIX seconds, as
   * daemontools' `tai64n` writes it on an ordinary clock.
   *
   * @param {number | bigint} unixSeconds
   * @param {number | bigint} attoseconds
   * @param {"tai64" | "tai64n"} format
   * @returns {string}
   */
  tai64PosixPlus10Encode(unixSeconds, attoseconds, format) {
    const fn = this.#export("hc_tai64_posix_plus_10_encode");
    const seconds = toI64(unixSeconds, "unixSeconds");
    const attos = toU64(attoseconds, "attoseconds");
    const text = this.#withText(format, "format", (pointer, len) =>
      this.#text("hc_tai64_posix_plus_10_encode", (buffer, capacity) =>
        fn(seconds, attos, pointer, len, buffer, capacity), true));
    return this.#oneLine("hc_tai64_posix_plus_10_encode", text, ["label"])[0];
  }

  /**
   * A `tai64-posix-plus-10` label read back: its format and the POSIX
   * instant it names, both parts as `BigInt`s.
   *
   * @param {string} hex
   * @returns {import("./hyper-calendar.d.ts").PosixTai64Label}
   */
  tai64PosixPlus10Decode(hex) {
    const fn = this.#export("hc_tai64_posix_plus_10_decode");
    const text = this.#withText(hex, "hex", (pointer, len) =>
      this.#text("hc_tai64_posix_plus_10_decode", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return posixTai64Label(this.#oneLine("hc_tai64_posix_plus_10_decode", text, COLUMNS.tai64PosixPlus10));
  }

  /**
   * The version, the 60-bit timestamp and the POSIX instant of a version 1
   * or version 6 UUID in RFC 9562's string form. Another version is
   * `no-data`, other text `malformed`.
   *
   * @param {string} uuid
   * @returns {import("./hyper-calendar.d.ts").UuidTimestamp}
   */
  uuidTimestamp(uuid) {
    const fn = this.#export("hc_uuid_timestamp");
    const text = this.#withText(uuid, "uuid", (pointer, len) =>
      this.#text("hc_uuid_timestamp", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return uuidTimestamp(this.#oneLine("hc_uuid_timestamp", text, COLUMNS.uuidTimestamp));
  }

  /**
   * A 64-bit NTP timestamp, its seconds and 2⁻³² s fraction, placed in the
   * era within 2³¹ s of a reference POSIX second. The zero timestamp is
   * `no-data`.
   *
   * @param {number} seconds
   * @param {number} fraction
   * @param {number | bigint} referenceUnix
   * @returns {import("./hyper-calendar.d.ts").NtpDate}
   */
  ntpResolve(seconds, fraction, referenceUnix) {
    const fn = this.#export("hc_ntp_resolve");
    const s = toU32(seconds, "seconds");
    const f = toU32(fraction, "fraction");
    const reference = toI64(referenceUnix, "referenceUnix");
    const text = this.#text("hc_ntp_resolve", (buffer, capacity) => fn(s, f, reference, buffer, capacity), true);
    return ntpDate(this.#oneLine("hc_ntp_resolve", text, COLUMNS.ntpResolve));
  }

  /**
   * The 60-bit UUID timestamp of a POSIX instant, the 100 ns interval that
   * contains it counted from 1582-10-15, and the first three groups of a
   * version 1 and a version 6 UUID that carry it, in lower case, for the
   * caller's clock sequence and node to follow. Before 1582-10-15 or after
   * 5236-03-31 is `out-of-range`.
   *
   * @param {number | bigint} unixSeconds
   * @param {number | bigint} [attoseconds]
   * @returns {import("./hyper-calendar.d.ts").UuidTimeFields}
   */
  uuidTimestampEncode(unixSeconds, attoseconds = 0) {
    const fn = this.#export("hc_uuid_timestamp_encode");
    const seconds = toI64(unixSeconds, "unixSeconds");
    const attos = toU64(attoseconds, "attoseconds");
    const text = this.#text("hc_uuid_timestamp_encode", (buffer, capacity) => fn(seconds, attos, buffer, capacity), true);
    return uuidTimeFields(this.#oneLine("hc_uuid_timestamp_encode", text, COLUMNS.uuidTimestampEncode));
  }

  /**
   * The NTP date and timestamp of a POSIX instant: the era, the era offset
   * and the 2⁻⁶⁴ s fraction, and the 128-bit date and the 64-bit
   * timestamp in their wire layouts as lower-case hexadecimal.
   *
   * @param {number | bigint} unixSeconds
   * @param {number | bigint} [attoseconds]
   * @returns {import("./hyper-calendar.d.ts").NtpEncoding}
   */
  ntpEncode(unixSeconds, attoseconds = 0) {
    const fn = this.#export("hc_ntp_encode");
    const seconds = toI64(unixSeconds, "unixSeconds");
    const attos = toU64(attoseconds, "attoseconds");
    const text = this.#text("hc_ntp_encode", (buffer, capacity) => fn(seconds, attos, buffer, capacity), true);
    return ntpEncoding(this.#oneLine("hc_ntp_encode", text, COLUMNS.ntpEncode));
  }

  /**
   * The local reading a FAT date word and time word name: its fixed day
   * and the even seconds into it. Fields that name no day are
   * `invalid-date`.
   *
   * @param {number} date
   * @param {number} time
   * @returns {import("./hyper-calendar.d.ts").FatReading}
   */
  fatDecode(date, time) {
    const fn = this.#export("hc_fat_decode");
    const d = toU32(date, "date");
    const t = toU32(time, "time");
    const text = this.#text("hc_fat_decode", (buffer, capacity) => fn(d, t, buffer, capacity), true);
    return fatReading(this.#oneLine("hc_fat_decode", text, COLUMNS.fatDecode));
  }

  /**
   * The FAT date and time words of a fixed day and a time of day in whole
   * seconds, the second rounded down to an even one; outside 1980 to 2107
   * is `out-of-range`.
   *
   * @param {number | bigint} fixed
   * @param {number} secondsOfDay
   * @returns {import("./hyper-calendar.d.ts").FatWords}
   */
  fatEncode(fixed, secondsOfDay) {
    const fn = this.#export("hc_fat_encode");
    const day = toI64(fixed, "fixed");
    const seconds = toU32(secondsOfDay, "secondsOfDay");
    const text = this.#text("hc_fat_encode", (buffer, capacity) => fn(day, seconds, buffer, capacity), true);
    return fatWords(this.#oneLine("hc_fat_encode", text, COLUMNS.fatEncode));
  }

  /**
   * The Swatch Internet Time beat at a POSIX instant, 0 through 999: @000
   * begins at 23:00 UTC.
   *
   * @param {number | bigint} unixSeconds
   * @param {number | bigint} [attoseconds]
   * @returns {number}
   */
  swatchBeat(unixSeconds, attoseconds = 0) {
    const fn = this.#export("hc_swatch_beat");
    return toNumber(fn(toI64(unixSeconds, "unixSeconds"), toU64(attoseconds, "attoseconds")), "hc_swatch_beat");
  }

  /**
   * The Julian (`J`) or Besselian (`B`) epoch of a TT instant: whole
   * seconds from 1970-01-01 00:00:00 TT and attoseconds.
   *
   * @param {import("./hyper-calendar.d.ts").EpochNotationName} notation
   * @param {number | bigint} ttSeconds
   * @param {number | bigint} [attoseconds]
   * @returns {import("./hyper-calendar.d.ts").Epoch}
   */
  epochFromTt(notation, ttSeconds, attoseconds = 0) {
    const fn = this.#export("hc_epoch_from_tt");
    const seconds = toI64(ttSeconds, "ttSeconds");
    const attos = toU64(attoseconds, "attoseconds");
    const text = this.#withText(notation, "notation", (pointer, len) =>
      this.#text("hc_epoch_from_tt", (buffer, capacity) => fn(pointer, len, seconds, attos, buffer, capacity), true));
    return epochLine(this.#oneLine("hc_epoch_from_tt", text, COLUMNS.epoch));
  }

  /**
   * The TT instant of a Julian or Besselian epoch. An empty notation reads
   * the year as SOFA reads one written without a letter: Besselian before
   * 1984.0, Julian from it.
   *
   * @param {import("./hyper-calendar.d.ts").EpochNotationName | ""} notation
   * @param {number} year
   * @returns {import("./hyper-calendar.d.ts").EpochInstant}
   */
  ttFromEpoch(notation, year) {
    const fn = this.#export("hc_tt_from_epoch");
    const y = toF64(year, "year");
    const text = this.#withText(notation, "notation", (pointer, len) =>
      this.#text("hc_tt_from_epoch", (buffer, capacity) => fn(pointer, len, y, buffer, capacity), true));
    return epochInstant(this.#oneLine("hc_tt_from_epoch", text, COLUMNS.ttFromEpoch));
  }

  /**
   * TT(BIPM) at a TAI instant, from a realisation the caller supplies as
   * text: one line per sample, the MJD and TT(BIPMxx) − TAI − 32.184 s in
   * microseconds, tab-separated.
   *
   * @param {string} series
   * @param {number | bigint} taiSeconds
   * @param {number | bigint} [attoseconds]
   * @param {boolean} [strict]
   * @returns {import("./hyper-calendar.d.ts").TtBipmReading}
   */
  ttBipm(series, taiSeconds, attoseconds = 0, strict = false) {
    const fn = this.#export("hc_tt_bipm");
    const seconds = toI64(taiSeconds, "taiSeconds");
    const attos = toU64(attoseconds, "attoseconds");
    const text = this.#withText(series, "series", (pointer, len) =>
      this.#text("hc_tt_bipm", (buffer, capacity) =>
        fn(pointer, len, seconds, attos, strict ? 1 : 0, buffer, capacity), true));
    return ttBipmReading(this.#oneLine("hc_tt_bipm", text, COLUMNS.ttBipm));
  }

  /**
   * The yoga and the karaṇa in progress at a POSIX instant, read as
   * Universal Time, the yoga reckoned with an ayanamsa: `lahiri`, `raman`,
   * `krishnamurti` or `fagan-bradley`.
   *
   * @param {number | bigint} unixSeconds
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @returns {import("./hyper-calendar.d.ts").PanchangaLimb[]}
   */
  panchangaAt(unixSeconds, ayanamsa) {
    const fn = this.#export("hc_panchanga_at");
    const instant = toI64(unixSeconds, "unixSeconds");
    const text = this.#withText(ayanamsa, "ayanamsa", (pointer, len) =>
      this.#text("hc_panchanga_at", (buffer, capacity) => fn(instant, pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.panchanga, "hc_panchanga_at").map(panchangaLimb);
  }

  /**
   * The yoga and the karaṇa a fixed day carries at a place, read at its
   * sunrise; a day without one there is `no-data`.
   *
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} elevation
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @returns {import("./hyper-calendar.d.ts").PanchangaLimb[]}
   */
  panchangaOfDay(fixed, latitude, longitude, elevation, ayanamsa) {
    const fn = this.#export("hc_panchanga_of_day");
    const day = toI64(fixed, "fixed");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(ayanamsa, "ayanamsa", (pointer, len) =>
      this.#text("hc_panchanga_of_day", (buffer, capacity) =>
        fn(day, lat, lon, elev, pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.panchanga, "hc_panchanga_of_day").map(panchangaLimb);
  }

  /**
   * The Hindu lunisolar date of a fixed day at a place, read at its
   * sunrise: on the true sky in the zodiac of a named ayanamsa, or on the
   * *Sūrya Siddhānta*'s, `surya-siddhanta`; with the month and the eras
   * named in `locale`, as {@link describeDay} names them for `hindu-lunar`.
   *
   * @param {import("./hyper-calendar.d.ts").Ayanamsa | "surya-siddhanta"} sky
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").HinduLunarDate}
   */
  hinduLunarDate(sky, fixed, latitude, longitude, elevation = 0, locale = "und") {
    const fn = this.#export("hc_hindu_lunar_date");
    const day = toI64(fixed, "fixed");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(sky, "sky", (skyPointer, skyLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#text("hc_hindu_lunar_date", (buffer, capacity) =>
          fn(skyPointer, skyLen, day, lat, lon, elev, localePointer, localeLen, buffer, capacity), true)));
    return hinduLunarDateLine(this.#oneLine("hc_hindu_lunar_date", text, COLUMNS.hinduLunarDate));
  }

  /**
   * The *Sūrya Siddhānta*'s Sun and Moon at a POSIX instant, with the
   * tithi and the Sun's sign.
   *
   * @param {number | bigint} unixSeconds
   * @returns {import("./hyper-calendar.d.ts").SuryaSiddhantaSky}
   */
  suryaSiddhantaAt(unixSeconds) {
    const fn = this.#export("hc_surya_siddhanta_at");
    const instant = toI64(unixSeconds, "unixSeconds");
    const text = this.#text("hc_surya_siddhanta_at", (buffer, capacity) => fn(instant, buffer, capacity), true);
    return suryaSiddhantaSky(this.#oneLine("hc_surya_siddhanta_at", text, COLUMNS.suryaSiddhanta));
  }

  /**
   * The *Sūrya Siddhānta*'s sunrise on a fixed day at a place, as POSIX
   * seconds of Universal Time, rounded down.
   *
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @returns {number}
   */
  suryaSiddhantaSunrise(fixed, latitude, longitude) {
    const fn = this.#export("hc_surya_siddhanta_sunrise");
    const day = toI64(fixed, "fixed");
    const [lat, lon] = [toF64(latitude, "latitude"), toF64(longitude, "longitude")];
    const text = this.#text("hc_surya_siddhanta_sunrise", (buffer, capacity) => fn(day, lat, lon, buffer, capacity), true);
    return integer(this.#oneLine("hc_surya_siddhanta_sunrise", text, ["instant"])[0], "instant");
  }
  /**
   * The northern sixty-year cycle's name a rule couples with an expired
   * Śaka year, and the name it expunges that year.
   *
   * @param {import("./hyper-calendar.d.ts").BarhaspatyaRule} rule
   * @param {number | bigint} saka
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").BarhaspatyaYear}
   */
  barhaspatyaYear(rule, saka, locale = "und") {
    const fn = this.#export("hc_barhaspatya_year");
    const year = toI64(saka, "saka");
    const text = this.#withText(rule, "rule", (rulePointer, ruleLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#text("hc_barhaspatya_year", (buffer, capacity) =>
          fn(rulePointer, ruleLen, year, localePointer, localeLen, buffer, capacity), true)));
    return barhaspatyaYear(this.#oneLine("hc_barhaspatya_year", text, COLUMNS.barhaspatyaYear));
  }

  /**
   * The northern sixty-year cycle's name in progress at an instant by a
   * rule.
   *
   * @param {import("./hyper-calendar.d.ts").BarhaspatyaRule} rule
   * @param {number | bigint} unixSeconds
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").BarhaspatyaName}
   */
  barhaspatyaYearAt(rule, unixSeconds, locale = "und") {
    const fn = this.#export("hc_barhaspatya_year_at");
    const seconds = toI64(unixSeconds, "unixSeconds");
    const text = this.#withText(rule, "rule", (rulePointer, ruleLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#text("hc_barhaspatya_year_at", (buffer, capacity) =>
          fn(rulePointer, ruleLen, seconds, localePointer, localeLen, buffer, capacity), true)));
    const [position, name, localeUsed] = this.#oneLine("hc_barhaspatya_year_at", text, COLUMNS.barhaspatyaYearAt);
    return { position: integer(position, "position"), name, localeUsed };
  }


  /**
   * Whether the young crescent should have been visible on the evening
   * that begins a fixed day, from a place, by `shaukat`, `yallop`,
   * `saudi-rule`, `odeh`, `istanbul-2016`, `khgt`,
   * `mabims-2021-topocentric` or `mabims-2021-geocentric-elongation`.
   *
   * @param {import("./hyper-calendar.d.ts").CrescentCriterion} criterion
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @returns {import("./hyper-calendar.d.ts").CrescentVisibility}
   */
  crescentVisible(criterion, fixed, latitude, longitude, elevation = 0) {
    const fn = this.#export("hc_crescent_visible");
    const day = toI64(fixed, "fixed");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(criterion, "criterion", (pointer, len) =>
      this.#text("hc_crescent_visible", (buffer, capacity) =>
        fn(pointer, len, day, lat, lon, elev, buffer, capacity), true));
    return crescentVisibility(this.#oneLine("hc_crescent_visible", text, COLUMNS.crescent));
  }

  /**
   * The number of the modern Olympiad a Gregorian year belongs to, from 1
   * for 1896–1899; an earlier year is `out-of-range`.
   *
   * @param {number | bigint} gregorianYear
   * @returns {number}
   */
  iocOlympiad(gregorianYear) {
    return toNumber(this.#export("hc_ioc_olympiad")(toI64(gregorianYear, "gregorianYear")), "hc_ioc_olympiad");
  }

  /**
   * The fixed day of the yahrzeit in a Hebrew year of a death on the Hebrew
   * date a fixed day names; a death after sunset is the next fixed day.
   *
   * @param {number | bigint} deathFixed
   * @param {number | bigint} hebrewYear
   * @returns {number}
   */
  hebrewYahrzeit(deathFixed, hebrewYear) {
    const fn = this.#export("hc_hebrew_yahrzeit");
    return toNumber(fn(toI64(deathFixed, "deathFixed"), toI64(hebrewYear, "hebrewYear")), "hc_hebrew_yahrzeit");
  }

  /**
   * The fixed day of the birthday in a Hebrew year of a birth on the Hebrew
   * date a fixed day names, as {@link hebrewYahrzeit}.
   *
   * @param {number | bigint} birthFixed
   * @param {number | bigint} hebrewYear
   * @returns {number}
   */
  hebrewBirthday(birthFixed, hebrewYear) {
    const fn = this.#export("hc_hebrew_birthday");
    return toNumber(fn(toI64(birthFixed, "birthFixed"), toI64(hebrewYear, "hebrewYear")), "hc_hebrew_birthday");
  }

  /**
   * A person's age as the Chinese count reckons it on a fixed day: one at
   * birth and one more at each Chinese New Year. A day before the birth
   * has no age and is `no-data`.
   *
   * @param {number | bigint} birthFixed
   * @param {number | bigint} onFixed
   * @returns {number}
   */
  chineseReckonedAge(birthFixed, onFixed) {
    const fn = this.#export("hc_chinese_reckoned_age");
    return toNumber(fn(toI64(birthFixed, "birthFixed"), toI64(onFixed, "onFixed")), "hc_chinese_reckoned_age");
  }

  /**
   * The marriage augury of a Chinese year, counted as the Chinese calendar
   * counts it (4661 began on 10 February 2024): `widow`, `blind`, `bright`
   * or `double-bright`, by where 立春 falls in it.
   *
   * @param {number | bigint} chineseYear
   * @returns {import("./hyper-calendar.d.ts").MarriageAugury}
   */
  chineseMarriageAugury(chineseYear) {
    const fn = this.#export("hc_chinese_marriage_augury");
    const year = toI64(chineseYear, "chineseYear");
    const text = this.#text("hc_chinese_marriage_augury", (buffer, capacity) => fn(year, buffer, capacity), true);
    return marriageAugury(this.#oneLine("hc_chinese_marriage_augury", text, COLUMNS.marriageAugury));
  }

  /**
   * Which month and weekday names a locale writes for a calendar on a
   * fixed day, where a government renamed them for a period.
   *
   * @param {string} calendar
   * @param {number | bigint} fixed
   * @param {string} locale
   * @returns {import("./hyper-calendar.d.ts").NamingPeriodOn}
   */
  namingPeriodOn(calendar, fixed, locale) {
    const fn = this.#export("hc_naming_period_on");
    const day = toI64(fixed, "fixed");
    const text = this.#withText(calendar, "calendar", (calendarPointer, calendarLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#text("hc_naming_period_on", (buffer, capacity) =>
          fn(calendarPointer, calendarLen, day, localePointer, localeLen, buffer, capacity), true)));
    return namingPeriodOn(this.#oneLine("hc_naming_period_on", text, COLUMNS.namingPeriod));
  }

  /**
   * The place of a Hebrew year in the sabbatical cycle, 1 through 7, the
   * seventh being *shemittah*.
   *
   * @param {number | bigint} hebrewYear
   * @returns {number}
   */
  hebrewSabbaticalCycleYear(hebrewYear) {
    const fn = this.#export("hc_hebrew_sabbatical_cycle_year");
    return toNumber(fn(toI64(hebrewYear, "hebrewYear")), "hc_hebrew_sabbatical_cycle_year");
  }

  /**
   * A fixed day in the calendar of the Roman province of Asia as the
   * calendar writes it, Sebaste and the other unnumbered days included.
   *
   * @param {number | bigint} fixed
   * @returns {import("./hyper-calendar.d.ts").AsianDay}
   */
  asianDay(fixed) {
    const fn = this.#export("hc_asian_day");
    const day = toI64(fixed, "fixed");
    const text = this.#text("hc_asian_day", (buffer, capacity) => fn(day, buffer, capacity), true);
    return asianDayLine(this.#oneLine("hc_asian_day", text, COLUMNS.asianDay));
  }

  /**
   * Every holiday table, in {@link holidayCodes} order, with its kind, its
   * names, its sources and the country of an exchange where its table
   * records one. A country is named as CLDR 48 names it in the locale,
   * where `hc-i18n` carries the name, and every other table, and a country
   * the locale has no name for, in English; `localeUsed` says which.
   * `shortName` is CLDR's `alt="short"` name beside a CLDR name, from the
   * same data — `Hong Kong` under `en`, 香港 under `ja` — where it has one.
   *
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").HolidayTable[]}
   */
  holidayTables(locale = "und") {
    const fn = this.#export("hc_holiday_tables");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_holiday_tables", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.holidayTables, "hc_holiday_tables").map(holidayTable);
  }

  /**
   * The lectionary cycles of a fixed day: the liturgical year, the Sunday
   * cycle, the Roman weekday cycle and the RCL Proper of a Sunday after
   * Trinity Sunday.
   *
   * @param {number | bigint} fixed
   * @returns {import("./hyper-calendar.d.ts").Lectionary}
   */
  lectionary(fixed) {
    const fn = this.#export("hc_lectionary");
    const day = toI64(fixed, "fixed");
    const text = this.#text("hc_lectionary", (buffer, capacity) => fn(day, buffer, capacity), true);
    return lectionaryLine(this.#oneLine("hc_lectionary", text, COLUMNS.lectionary));
  }

  /**
   * The fixed day of Easter by the astronomical reckoning at the meridian
   * of Jerusalem, for 1583 to 2150.
   *
   * @param {number | bigint} year
   * @returns {number}
   */
  astronomicalEaster(year) {
    return toNumber(this.#export("hc_astronomical_easter")(toI64(year, "year")), "hc_astronomical_easter");
  }

  /**
   * The fixed day of the paschal full moon by the astronomical reckoning at
   * the meridian of Jerusalem, for 1583 to 2150: the day
   * {@link astronomicalEaster} is the first Sunday after.
   *
   * @param {number | bigint} year
   * @returns {number}
   */
  astronomicalPaschalFullMoon(year) {
    return toNumber(
      this.#export("hc_astronomical_paschal_full_moon")(toI64(year, "year")),
      "hc_astronomical_paschal_full_moon",
    );
  }

  /**
   * The Holy Year a fixed day falls in, or `null` outside one; a day the
   * table does not reach is `no-data`.
   *
   * @param {number | bigint} fixed
   * @returns {import("./hyper-calendar.d.ts").HolyYear | null}
   */
  holyYearOn(fixed) {
    const fn = this.#export("hc_holy_year_on");
    const day = toI64(fixed, "fixed");
    const text = this.#text("hc_holy_year_on", (buffer, capacity) => fn(day, buffer, capacity), true);
    return holyYearLine(this.#oneLine("hc_holy_year_on", text, COLUMNS.holyYear));
  }

  /**
   * Every *Common Worship* celebration kept on a fixed day, with its rank.
   *
   * @param {number | bigint} fixed
   * @returns {import("./hyper-calendar.d.ts").CommonWorshipCelebration[]}
   */
  commonWorshipOn(fixed) {
    const fn = this.#export("hc_common_worship_on");
    const day = toI64(fixed, "fixed");
    const text = this.#text("hc_common_worship_on", (buffer, capacity) => fn(day, buffer, capacity), true);
    return rows(text, COLUMNS.commonWorship, "hc_common_worship_on").map(commonWorshipCelebration);
  }

  /**
   * @param {string} exportName
   * @param {number} ut1UnixSeconds
   * @returns {number}
   */
  #rotation(exportName, ut1UnixSeconds) {
    const fn = this.#export(exportName);
    const seconds = toF64(ut1UnixSeconds, "ut1UnixSeconds");
    const text = this.#text(exportName, (buffer, capacity) => fn(seconds, buffer, capacity), true);
    return decimal(this.#oneLine(exportName, text, COLUMNS.value)[0], "value");
  }

  /**
   * The Earth Rotation Angle in degrees at a UT1 instant, counted as POSIX
   * seconds are, from 1970-01-01 00:00 UT1.
   *
   * @param {number} ut1UnixSeconds
   * @returns {number}
   */
  earthRotationAngle(ut1UnixSeconds) {
    return this.#rotation("hc_earth_rotation_angle", ut1UnixSeconds);
  }

  /**
   * The Greenwich mean sidereal time in degrees by the IAU 2006 convention,
   * at a UT1 instant as {@link earthRotationAngle}.
   *
   * @param {number} ut1UnixSeconds
   * @returns {number}
   */
  gmstIau2006(ut1UnixSeconds) {
    return this.#rotation("hc_gmst_iau2006", ut1UnixSeconds);
  }

  /**
   * The Greenwich mean sidereal time in degrees by the IAU 1982 convention,
   * at a UT1 instant as {@link earthRotationAngle}.
   *
   * @param {number} ut1UnixSeconds
   * @returns {number}
   */
  gmstIau1982(ut1UnixSeconds) {
    return this.#rotation("hc_gmst_iau1982", ut1UnixSeconds);
  }

  /**
   * UT2 − UT1 in seconds at a UT1 instant as {@link earthRotationAngle}.
   *
   * @param {number} ut1UnixSeconds
   * @returns {number}
   */
  ut2MinusUt1(ut1UnixSeconds) {
    return this.#rotation("hc_ut2_minus_ut1", ut1UnixSeconds);
  }

  /**
   * A local clock's reading at a POSIX instant, read as Universal Time, at
   * a place: `local-mean`, `local-apparent`, `temporal` or `italian`. A
   * reading that needs a solar event that does not happen has `day` and
   * `hours` `null` and names the event in `missing`.
   *
   * @param {import("./hyper-calendar.d.ts").SolarClock} clock
   * @param {number | bigint} unixSeconds
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @returns {import("./hyper-calendar.d.ts").SolarTime}
   */
  solarTime(clock, unixSeconds, latitude, longitude, elevation = 0) {
    const fn = this.#export("hc_solar_time");
    const instant = toI64(unixSeconds, "unixSeconds");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(clock, "clock", (pointer, len) =>
      this.#text("hc_solar_time", (buffer, capacity) =>
        fn(pointer, len, instant, lat, lon, elev, buffer, capacity), true));
    return solarTime(this.#oneLine("hc_solar_time", text, COLUMNS.solarTime));
  }

  /**
   * A named time of day on a fixed day at a place — `asr-shafii`,
   * `asr-hanafi`, `jewish-dusk-vilna-gaon`, `jewish-sabbath-ends-cohn`,
   * `italian-zero-hour`, `japanese-dawn-kansei`, `japanese-dusk-kansei`,
   * `japanese-dawn-naoj` or `japanese-dusk-naoj` — as POSIX seconds of
   * Universal Time, or `null` with the missing solar event named.
   *
   * @param {import("./hyper-calendar.d.ts").SolarEventName} event
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @returns {import("./hyper-calendar.d.ts").SolarEvent}
   */
  solarEvent(event, fixed, latitude, longitude, elevation = 0) {
    const fn = this.#export("hc_solar_event");
    const day = toI64(fixed, "fixed");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(event, "event", (pointer, len) =>
      this.#text("hc_solar_event", (buffer, capacity) =>
        fn(pointer, len, day, lat, lon, elev, buffer, capacity), true));
    return solarEvent(this.#oneLine("hc_solar_event", text, COLUMNS.solarEvent));
  }

  /**
   * Every named horizon a rising or a setting can be measured against,
   * named in a locale where an observatory or almanac office names it.
   *
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").Horizon[]}
   */
  horizons(locale = "und") {
    const fn = this.#export("hc_horizons");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_horizons", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.horizons, "hc_horizons").map(horizonLine);
  }

  /**
   * Sunrise on a fixed day at a place against a named horizon, or `null`
   * with the missing sunrise named.
   *
   * @param {import("./hyper-calendar.d.ts").HorizonId} horizon
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @returns {import("./hyper-calendar.d.ts").SolarCrossing}
   */
  sunrise(horizon, fixed, latitude, longitude, elevation = 0) {
    return this.#crossing("hc_sunrise", horizon, fixed, latitude, longitude, elevation);
  }

  /**
   * Sunset on a fixed day at a place against a named horizon, or `null`
   * with the missing sunset named.
   *
   * @param {import("./hyper-calendar.d.ts").HorizonId} horizon
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @returns {import("./hyper-calendar.d.ts").SolarCrossing}
   */
  sunset(horizon, fixed, latitude, longitude, elevation = 0) {
    return this.#crossing("hc_sunset", horizon, fixed, latitude, longitude, elevation);
  }

  /**
   * @param {string} exportName
   * @param {string} horizon
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} elevation
   * @returns {import("./hyper-calendar.d.ts").SolarCrossing}
   */
  #crossing(exportName, horizon, fixed, latitude, longitude, elevation) {
    const fn = this.#export(exportName);
    const day = toI64(fixed, "fixed");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(horizon, "horizon", (pointer, len) =>
      this.#text(exportName, (buffer, capacity) =>
        fn(pointer, len, day, lat, lon, elev, buffer, capacity), true));
    return solarCrossing(this.#oneLine(exportName, text, COLUMNS.solarCrossing));
  }

  /**
   * HJD_TT: a Julian Date of TT corrected to the Sun for a target's J2000
   * right ascension and declination, in degrees.
   *
   * @param {number} ttJulianDate
   * @param {number} rightAscension
   * @param {number} declination
   * @returns {import("./hyper-calendar.d.ts").HeliocentricJulianDate}
   */
  hjdTt(ttJulianDate, rightAscension, declination) {
    const fn = this.#export("hc_hjd_tt");
    const [date, alpha, delta] = [toF64(ttJulianDate, "ttJulianDate"), toF64(rightAscension, "rightAscension"), toF64(declination, "declination")];
    const text = this.#text("hc_hjd_tt", (buffer, capacity) => fn(date, alpha, delta, buffer, capacity), true);
    return heliocentricJulianDate(this.#oneLine("hc_hjd_tt", text, COLUMNS.hjdTt));
  }

  /**
   * HJD_UTC: a Julian Date of UTC corrected to the Sun, with the TT − UTC
   * the leap-second table gave.
   *
   * @param {number} utcJulianDate
   * @param {number} rightAscension
   * @param {number} declination
   * @param {boolean} [strict]
   * @returns {import("./hyper-calendar.d.ts").HeliocentricJulianDateUtc}
   */
  hjdUtc(utcJulianDate, rightAscension, declination, strict = false) {
    const fn = this.#export("hc_hjd_utc");
    const [date, alpha, delta] = [toF64(utcJulianDate, "utcJulianDate"), toF64(rightAscension, "rightAscension"), toF64(declination, "declination")];
    const text = this.#text("hc_hjd_utc", (buffer, capacity) =>
      fn(date, alpha, delta, strict ? 1 : 0, buffer, capacity), true);
    const cells = this.#oneLine("hc_hjd_utc", text, COLUMNS.hjdUtc);
    return { ...heliocentricJulianDate(cells), ttMinusUtcSeconds: decimal(cells[2], "tt minus utc") };
  }
  /**
   * The astronomical date and the Greenwich Mean Astronomical Time of a
   * reading of GMT: GMT − 12 h, the day beginning at noon.
   *
   * @param {number | bigint} fixed
   * @param {number} secondsOfDay
   * @param {number | bigint} [attoseconds]
   * @returns {import("./hyper-calendar.d.ts").ClockReading}
   */
  gmatFromGmt(fixed, secondsOfDay, attoseconds = 0) {
    return this.#clockReading("hc_gmat_from_gmt", fixed, secondsOfDay, attoseconds);
  }

  /**
   * The civil date and the GMT of a reading of Greenwich Mean Astronomical
   * Time.
   *
   * @param {number | bigint} fixed
   * @param {number} secondsOfDay
   * @param {number | bigint} [attoseconds]
   * @returns {import("./hyper-calendar.d.ts").ClockReading}
   */
  gmtFromGmat(fixed, secondsOfDay, attoseconds = 0) {
    return this.#clockReading("hc_gmt_from_gmat", fixed, secondsOfDay, attoseconds);
  }

  /**
   * A reading of one clock as another, for the GMAT exports.
   *
   * @param {string} exportName
   * @param {number | bigint} fixed
   * @param {number} secondsOfDay
   * @param {number | bigint} attoseconds
   * @returns {import("./hyper-calendar.d.ts").ClockReading}
   */
  #clockReading(exportName, fixed, secondsOfDay, attoseconds) {
    const fn = this.#export(exportName);
    const day = toI64(fixed, "fixed");
    const seconds = toU32(secondsOfDay, "secondsOfDay");
    const attos = toU64(attoseconds, "attoseconds");
    const text = this.#text(exportName, (buffer, capacity) => fn(day, seconds, attos, buffer, capacity), true);
    return clockReading(this.#oneLine(exportName, text, COLUMNS.gmat));
  }


  /**
   * Mars at a POSIX instant and an east longitude: the Mars Sol Date,
   * Coordinated Mars Time, local mean and true solar time, the equation of
   * time, `Ls`, the Clancy Mars year and the Darian date at Airy-0, from
   * the Mars24 restatement of Allison and McEwen. An instant more than
   * 100 Julian years from J2000.0 is `out-of-range`.
   *
   * @param {number} unixSeconds
   * @param {number} [eastLongitude]
   * @returns {import("./hyper-calendar.d.ts").MarsTime}
   */
  marsTime(unixSeconds, eastLongitude = 0) {
    const fn = this.#export("hc_mars_time");
    const instant = toF64(unixSeconds, "unixSeconds");
    const east = toF64(eastLongitude, "eastLongitude");
    const text = this.#text("hc_mars_time", (buffer, capacity) => fn(instant, east, buffer, capacity), true);
    return marsTime(this.#oneLine("hc_mars_time", text, COLUMNS.marsTime));
  }

  /**
   * Every surface mission on Mars, in landing order, with its landing, its
   * clock and the numbering of its landing sol; `null` where the
   * operators published no convention.
   *
   * @returns {import("./hyper-calendar.d.ts").Mission[]}
   */
  missions() {
    const fn = this.#export("hc_missions");
    const text = this.#text("hc_missions", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(text, COLUMNS.missions, "hc_missions").map(mission);
  }

  /**
   * A mission's sol number at a POSIX instant by its own clock, the
   * mission by its identifier: `viking-1`, `viking-2`, `mars-pathfinder`, `spirit`, `opportunity`, `phoenix`, `curiosity`, `insight`, `perseverance` or `zhurong`. A mission
   * with no published sol numbering is `no-data`; an instant before its
   * landing sol began is `out-of-range`.
   *
   * @param {import("./hyper-calendar.d.ts").MissionId} mission
   * @param {number} unixSeconds
   * @returns {number}
   */
  missionSol(mission, unixSeconds) {
    const fn = this.#export("hc_mission_sol");
    const instant = toF64(unixSeconds, "unixSeconds");
    return this.#withText(mission, "mission", (pointer, len) =>
      toNumber(fn(pointer, len, instant), "hc_mission_sol"));
  }

  /**
   * Every body `hc-planetary` carries, with its solar day.
   *
   * @returns {import("./hyper-calendar.d.ts").Body[]}
   */
  bodies() {
    const fn = this.#export("hc_bodies");
    const text = this.#text("hc_bodies", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(text, COLUMNS.bodies, "hc_bodies").map(body);
  }

  /**
   * Local mean solar time on a body at a POSIX instant and an east
   * longitude, the body by its identifier, as `hc_bodies` gives it. The
   * Sun, which has no solar day, is `no-data`.
   *
   * @param {import("./hyper-calendar.d.ts").BodyId} body
   * @param {number} unixSeconds
   * @param {number} [eastLongitude]
   * @returns {import("./hyper-calendar.d.ts").BodyTime}
   */
  bodyTime(body, unixSeconds, eastLongitude = 0) {
    const fn = this.#export("hc_body_time");
    const instant = toF64(unixSeconds, "unixSeconds");
    const east = toF64(eastLongitude, "eastLongitude");
    const text = this.#withText(body, "body", (pointer, len) =>
      this.#text("hc_body_time", (buffer, capacity) => fn(pointer, len, instant, east, buffer, capacity), true));
    return bodyTime(this.#oneLine("hc_body_time", text, COLUMNS.bodyTime));
  }
  /**
   * The date at a POSIX instant in a calendar of another body's days:
   * `darian-titan`, `gregorian-io`, `gregorian-europa`,
   * `gregorian-ganymede`, `gregorian-callisto` or `martiana`.
   *
   * @param {import("./hyper-calendar.d.ts").CircadCalendar} calendar
   * @param {number} unixSeconds
   * @returns {import("./hyper-calendar.d.ts").CircadDate}
   */
  circadDate(calendar, unixSeconds) {
    const fn = this.#export("hc_circad_date");
    const instant = toF64(unixSeconds, "unixSeconds");
    const text = this.#withText(calendar, "calendar", (pointer, len) =>
      this.#text("hc_circad_date", (buffer, capacity) => fn(pointer, len, instant, buffer, capacity), true));
    return circadDate(this.#oneLine("hc_circad_date", text, COLUMNS.circadDate));
  }


  /**
   * A clock moving at a constant speed in metres per second while
   * `coordinateSeconds` pass: β, γ, its proper time and its rate. A speed
   * at or beyond the speed of light is `out-of-range`.
   *
   * @param {number} speedMetresPerSecond
   * @param {number} coordinateSeconds
   * @returns {import("./hyper-calendar.d.ts").ProperTime}
   */
  properTime(speedMetresPerSecond, coordinateSeconds) {
    const fn = this.#export("hc_proper_time");
    const speed = toF64(speedMetresPerSecond, "speedMetresPerSecond");
    const coordinate = toF64(coordinateSeconds, "coordinateSeconds");
    const text = this.#text("hc_proper_time", (buffer, capacity) => fn(speed, coordinate, buffer, capacity), true);
    return properTime(this.#oneLine("hc_proper_time", text, COLUMNS.properTime));
  }

  /**
   * A clock held still at a radius in metres from a body's centre, against
   * one far from every mass. A radius at or inside the Schwarzschild
   * radius is `out-of-range`. The body is named by its identifier, `earth`
   * or `sagittarius-a-star`, as `hc_gravitating_bodies` gives it.
   *
   * @param {import("./hyper-calendar.d.ts").GravitatingBodyId} body
   * @param {number} radiusMetres
   * @returns {import("./hyper-calendar.d.ts").GravitationalDilation}
   */
  gravitationalDilation(body, radiusMetres) {
    const fn = this.#export("hc_gravitational_dilation");
    const radius = toF64(radiusMetres, "radiusMetres");
    const text = this.#withText(body, "body", (pointer, len) =>
      this.#text("hc_gravitational_dilation", (buffer, capacity) => fn(pointer, len, radius, buffer, capacity), true));
    return gravitationalDilation(this.#oneLine("hc_gravitational_dilation", text, COLUMNS.gravitationalDilation));
  }

  /**
   * Every body `hc-relativity` carries a gravitational parameter for.
   *
   * @returns {import("./hyper-calendar.d.ts").GravitatingBody[]}
   */
  gravitatingBodies() {
    const fn = this.#export("hc_gravitating_bodies");
    const text = this.#text("hc_gravitating_bodies", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(text, COLUMNS.gravitatingBodies, "hc_gravitating_bodies").map(gravitatingBody);
  }

  /**
   * A binary CCSDS time code read from hexadecimal, P-field first.
   *
   * @param {string} hex
   * @param {boolean} [strict]
   * @returns {import("./hyper-calendar.d.ts").CcsdsCode}
   */
  ccsdsDecode(hex, strict = false) {
    const fn = this.#export("hc_ccsds_decode");
    const text = this.#withText(hex, "hex", (pointer, len) =>
      this.#text("hc_ccsds_decode", (buffer, capacity) => fn(pointer, len, strict ? 1 : 0, buffer, capacity), true));
    return ccsdsCode(this.#oneLine("hc_ccsds_decode", text, COLUMNS.ccsdsDecode));
  }

  /**
   * The binary CCSDS time code of a TAI instant in a P-field's format, as
   * lower-case hexadecimal.
   *
   * @param {number | bigint} taiSeconds
   * @param {number | bigint} attoseconds
   * @param {string} pField
   * @param {boolean} [strict]
   * @returns {string}
   */
  ccsdsEncode(taiSeconds, attoseconds, pField, strict = false) {
    const fn = this.#export("hc_ccsds_encode");
    const seconds = toI64(taiSeconds, "taiSeconds");
    const attos = toU64(attoseconds, "attoseconds");
    const text = this.#withText(pField, "pField", (pointer, len) =>
      this.#text("hc_ccsds_encode", (buffer, capacity) =>
        fn(seconds, attos, pointer, len, strict ? 1 : 0, buffer, capacity), true));
    return this.#oneLine("hc_ccsds_encode", text, ["code"])[0];
  }

  /**
   * A CCSDS ASCII time code, A or B, read.
   *
   * @param {string} code
   * @param {boolean} [strict]
   * @returns {import("./hyper-calendar.d.ts").CcsdsAsciiCode}
   */
  ccsdsAsciiParse(code, strict = false) {
    const fn = this.#export("hc_ccsds_ascii_parse");
    const text = this.#withText(code, "code", (pointer, len) =>
      this.#text("hc_ccsds_ascii_parse", (buffer, capacity) =>
        fn(pointer, len, strict ? 1 : 0, buffer, capacity), true));
    return ccsdsAsciiCode(this.#oneLine("hc_ccsds_ascii_parse", text, COLUMNS.ccsdsAscii));
  }

  /**
   * The CCSDS ASCII time code of a TAI instant's UTC label.
   *
   * @param {number | bigint} taiSeconds
   * @param {number | bigint} attoseconds
   * @param {"a" | "b"} variation
   * @param {string} precision `hour`, `minute`, `second` or `1` to `18`
   * @param {boolean} [terminator]
   * @param {boolean} [strict]
   * @returns {string}
   */
  ccsdsAsciiFormat(taiSeconds, attoseconds, variation, precision, terminator = true, strict = false) {
    const fn = this.#export("hc_ccsds_ascii_format");
    const seconds = toI64(taiSeconds, "taiSeconds");
    const attos = toU64(attoseconds, "attoseconds");
    const text = this.#withText(variation, "variation", (variationPointer, variationLen) =>
      this.#withText(String(precision), "precision", (precisionPointer, precisionLen) =>
        this.#text("hc_ccsds_ascii_format", (buffer, capacity) =>
          fn(seconds, attos, variationPointer, variationLen, precisionPointer, precisionLen,
            terminator ? 1 : 0, strict ? 1 : 0, buffer, capacity), true)));
    return this.#oneLine("hc_ccsds_ascii_format", text, ["code"])[0];
  }

  /**
   * One minute's frame of a radio time code read.
   *
   * @param {import("./hyper-calendar.d.ts").RadioCode} code
   * @param {string} frame
   * @param {number | bigint} century
   * @returns {import("./hyper-calendar.d.ts").RadioMinute}
   */
  radioDecode(code, frame, century) {
    const fn = this.#export("hc_radio_decode");
    const start = toI64(century, "century");
    const text = this.#withText(code, "code", (codePointer, codeLen) =>
      this.#withText(frame, "frame", (framePointer, frameLen) =>
        this.#text("hc_radio_decode", (buffer, capacity) =>
          fn(codePointer, codeLen, framePointer, frameLen, start, buffer, capacity), true)));
    return radioMinute(this.#oneLine("hc_radio_decode", text, COLUMNS.radioDecode));
  }

  /**
   * The frame of a radio time code for a minute.
   *
   * @param {import("./hyper-calendar.d.ts").RadioCode} code
   * @param {number | bigint} unixSeconds
   * @param {import("./hyper-calendar.d.ts").RadioEncodeOptions} [options]
   * @returns {string}
   */
  radioEncode(code, unixSeconds, options = {}) {
    const fn = this.#export("hc_radio_encode");
    const minute = toI64(unixSeconds, "unixSeconds");
    const leap = options.leap ?? 0;
    if (![-1, 0, 1].includes(leap)) {
      raise(`leap must be -1, 0 or 1, got ${String(leap)}`);
    }
    const dut1 = options.dut1Tenths ?? 0;
    if (!Number.isInteger(dut1)) {
      raise(`dut1Tenths must be an integer, got ${String(dut1)}`);
    }
    const next = toU32(options.dstNext ?? 0, "dstNext");
    const text = this.#withText(code, "code", (codePointer, codeLen) =>
      this.#withText(options.summer ?? "", "summer", (summerPointer, summerLen) =>
        this.#text("hc_radio_encode", (buffer, capacity) =>
          fn(codePointer, codeLen, minute, leap, summerPointer, summerLen, options.zoneChange ? 1 : 0, dut1,
            next, buffer, capacity), true)));
    return this.#oneLine("hc_radio_encode", text, ["frame"])[0];
  }
  /**
   * One frame of an IRIG serial time code read, its designation such as
   * `B124` and its symbols `0`, `1` and `M`, Pr first; a code with the
   * year's two digits reads them in the century of `year`, one without
   * is read in `year`.
   *
   * @param {string} signal
   * @param {string} frame
   * @param {number | bigint} year
   * @returns {import("./hyper-calendar.d.ts").IrigReading}
   */
  irigDecode(signal, frame, year) {
    const fn = this.#export("hc_irig_decode");
    const y = toI64(year, "year");
    const text = this.#withText(signal, "signal", (signalPointer, signalLen) =>
      this.#withText(frame, "frame", (framePointer, frameLen) =>
        this.#text("hc_irig_decode", (buffer, capacity) =>
          fn(signalPointer, signalLen, framePointer, frameLen, y, buffer, capacity), true)));
    return irigReading(this.#oneLine("hc_irig_decode", text, COLUMNS.irigDecode));
  }

  /**
   * The frame of an IRIG code whose reference bit falls at a reading of the
   * civil clock: a fixed day, whole seconds after its midnight (86 400 for
   * 23:59:60) and, in `options`, hundredths and control bits.
   *
   * @param {string} signal
   * @param {number | bigint} fixed
   * @param {number} secondsOfDay
   * @param {import("./hyper-calendar.d.ts").IrigEncodeOptions} [options]
   * @returns {string}
   */
  irigEncode(signal, fixed, secondsOfDay, options = {}) {
    const fn = this.#export("hc_irig_encode");
    const day = toI64(fixed, "fixed");
    const seconds = toU32(secondsOfDay, "secondsOfDay");
    const hundredths = toU32(options.hundredths ?? 0, "hundredths");
    const control = toU32(options.control ?? 0, "control");
    const text = this.#withText(signal, "signal", (pointer, len) =>
      this.#text("hc_irig_encode", (buffer, capacity) =>
        fn(pointer, len, day, seconds, hundredths, control, buffer, capacity), true));
    return this.#oneLine("hc_irig_encode", text, ["frame"])[0];
  }

  /**
   * Every IRIG format, with its frame's length, its rate, its fields and
   * the designations Table 4-1 permits it: a reading `irigEncode` writes a
   * frame at is a multiple of `frameMicroseconds` from midnight.
   *
   * @returns {import("./hyper-calendar.d.ts").IrigFormatInfo[]}
   */
  irigFormats() {
    const fn = this.#export("hc_irig_formats");
    const text = this.#text("hc_irig_formats", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(text, COLUMNS.irigFormats, "hc_irig_formats").map(irigFormat);
  }


  /**
   * .NET's `DateTime.Ticks` of a POSIX instant as a `Utc` value.
   *
   * @param {number | bigint} unixSeconds
   * @param {number | bigint} [attoseconds]
   * @returns {bigint}
   */
  dotnetTicksFromUnix(unixSeconds, attoseconds = 0) {
    const fn = this.#export("hc_dotnet_ticks_from_unix");
    return checked(fn(toI64(unixSeconds, "unixSeconds"), toU64(attoseconds, "attoseconds")), "hc_dotnet_ticks_from_unix");
  }

  /**
   * The reading a count of .NET ticks names, in the POSIX shape.
   *
   * @param {number | bigint} ticks
   * @returns {import("./hyper-calendar.d.ts").DotnetReading}
   */
  unixFromDotnetTicks(ticks) {
    const fn = this.#export("hc_unix_from_dotnet_ticks");
    const count = toI64(ticks, "ticks");
    const text = this.#text("hc_unix_from_dotnet_ticks", (buffer, capacity) => fn(count, buffer, capacity), true);
    const [seconds, attoseconds] = this.#oneLine("hc_unix_from_dotnet_ticks", text, COLUMNS.unixFromDotnetTicks);
    return {
      unixSeconds: integer(seconds, "unix seconds"),
      attoseconds: bigInteger(attoseconds, "attoseconds"),
    };
  }

  /**
   * A time of the civil day on a six-hour clock.
   *
   * @param {import("./hyper-calendar.d.ts").SixHourReckoning} reckoning
   * @param {number} secondsOfDay
   * @returns {import("./hyper-calendar.d.ts").SixHourReading}
   */
  sixHourClock(reckoning, secondsOfDay) {
    const fn = this.#export("hc_six_hour_clock");
    const seconds = toU32(secondsOfDay, "secondsOfDay");
    const text = this.#withText(reckoning, "reckoning", (pointer, len) =>
      this.#text("hc_six_hour_clock", (buffer, capacity) => fn(pointer, len, seconds, buffer, capacity), true));
    return sixHourReading(this.#oneLine("hc_six_hour_clock", text, COLUMNS.sixHourClock));
  }

  /**
   * The civil seconds after midnight of a six-hour reading.
   *
   * @param {import("./hyper-calendar.d.ts").SixHourReckoning} reckoning
   * @param {number} hour
   * @param {number} minute
   * @param {number} second
   * @param {boolean} night
   * @returns {number}
   */
  civilFromSixHourClock(reckoning, hour, minute, second, night) {
    const fn = this.#export("hc_civil_from_six_hour_clock");
    const [h, m, sec] = [toU32(hour, "hour"), toU32(minute, "minute"), toU32(second, "second")];
    return this.#withText(reckoning, "reckoning", (pointer, len) =>
      toNumber(fn(pointer, len, h, m, sec, night ? 1 : 0), "hc_civil_from_six_hour_clock"));
  }

  /**
   * Rāhu kālam, Yamaganda and Gulika kālam on a day, each named in a locale.
   *
   * @param {import("./hyper-calendar.d.ts").KalamConvention} convention
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").KalamPeriod[]}
   */
  kalam(convention, fixed, latitude, longitude, elevation = 0, locale = "und") {
    const fn = this.#export("hc_kalam");
    const day = toI64(fixed, "fixed");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(convention, "convention", (pointer, len) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#text("hc_kalam", (buffer, capacity) =>
          fn(pointer, len, day, lat, lon, elev, localePointer, localeLen, buffer, capacity), true)));
    return rows(text, COLUMNS.kalam, "hc_kalam").map(kalamPeriod);
  }

  /**
   * 恵方, 三元九運 and 손 없는 날 of a day.
   *
   * @param {number | bigint} fixed
   * @param {string} [meridian]
   * @returns {import("./hyper-calendar.d.ts").AlmanacCycles}
   */
  almanacCycles(fixed, meridian = "") {
    const fn = this.#export("hc_almanac_cycles");
    const day = toI64(fixed, "fixed");
    const text = this.#withText(meridian, "meridian", (pointer, len) =>
      this.#text("hc_almanac_cycles", (buffer, capacity) => fn(day, pointer, len, buffer, capacity), true));
    return almanacCycles(this.#oneLine("hc_almanac_cycles", text, COLUMNS.almanacCycles));
  }

  /**
   * The almanac's annotations of a day, 干支 to the 選日, each named in a
   * locale.
   *
   * @param {number | bigint} fixed
   * @param {string} [meridian]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").AlmanacAnnotation[]}
   */
  almanacDay(fixed, meridian = "", locale = "und") {
    const fn = this.#export("hc_almanac_day");
    const day = toI64(fixed, "fixed");
    const text = this.#withText(meridian, "meridian", (meridianPointer, meridianLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#text("hc_almanac_day", (buffer, capacity) =>
          fn(day, meridianPointer, meridianLen, localePointer, localeLen, buffer, capacity), true)));
    return rows(text, COLUMNS.almanacDay, "hc_almanac_day").map(almanacAnnotation);
  }

  /**
   * The sixteen choghadiya of a day at a place, the daylight's eight and
   * the night's, each named in a locale.
   *
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").ChoghadiyaPart[]}
   */
  choghadiya(fixed, latitude, longitude, elevation = 0, locale = "und") {
    const fn = this.#export("hc_choghadiya");
    const day = toI64(fixed, "fixed");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_choghadiya", (buffer, capacity) => fn(day, lat, lon, elev, pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.choghadiya, "hc_choghadiya").map(choghadiyaPart);
  }

  /**
   * The Panchak window in progress at an instant, or the next, and its
   * kind under a naming table by the weekday it opens on, on a clock
   * `offsetSeconds` ahead of UTC.
   *
   * @param {import("./hyper-calendar.d.ts").PanchakNaming} naming
   * @param {number | bigint} unixSeconds
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @param {number} [offsetSeconds]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").PanchakWindow}
   */
  panchak(naming, unixSeconds, ayanamsa, offsetSeconds = 0, locale = "und") {
    const fn = this.#export("hc_panchak");
    const seconds = toI64(unixSeconds, "unixSeconds");
    const offset = toI32(offsetSeconds, "offsetSeconds");
    const text = this.#withText(naming, "naming", (namingPointer, namingLen) =>
      this.#withText(ayanamsa, "ayanamsa", (ayanamsaPointer, ayanamsaLen) =>
        this.#withText(locale, "locale", (localePointer, localeLen) =>
          this.#text("hc_panchak", (buffer, capacity) =>
            fn(namingPointer, namingLen, seconds, ayanamsaPointer, ayanamsaLen, offset, localePointer, localeLen,
              buffer, capacity), true))));
    return panchakWindow(this.#oneLine("hc_panchak", text, COLUMNS.panchak));
  }

  /**
   * When in a year the Sun, and the Moon where asked, stand as a Kumbh
   * condition requires, and whether Jupiter's sign, which the caller
   * gives because the library has no ephemeris of Jupiter, meets it.
   *
   * @param {import("./hyper-calendar.d.ts").KumbhYoga} yoga
   * @param {number | bigint} year
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @param {import("./hyper-calendar.d.ts").SiderealSignId | ""} [jupiter]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").KumbhOccasion}
   */
  kumbh(yoga, year, ayanamsa, jupiter = "", locale = "und") {
    const fn = this.#export("hc_kumbh");
    const y = toI64(year, "year");
    const text = this.#withText(yoga, "yoga", (yogaPointer, yogaLen) =>
      this.#withText(ayanamsa, "ayanamsa", (ayanamsaPointer, ayanamsaLen) =>
        this.#withText(jupiter, "jupiter", (jupiterPointer, jupiterLen) =>
          this.#withText(locale, "locale", (localePointer, localeLen) =>
            this.#text("hc_kumbh", (buffer, capacity) =>
              fn(yogaPointer, yogaLen, y, ayanamsaPointer, ayanamsaLen, jupiterPointer, jupiterLen, localePointer,
                localeLen, buffer, capacity), true)))));
    return kumbhOccasion(this.#oneLine("hc_kumbh", text, COLUMNS.kumbh));
  }

  /**
   * The twelve days of the Ādi Pushkaram of each river of a sign, for
   * Jupiter's entry into it at an instant the caller gives.
   *
   * @param {import("./hyper-calendar.d.ts").SiderealSignId} sign
   * @param {number | bigint} entryUnixSeconds
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @param {string} [meridian]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").PushkaramDays[]}
   */
  pushkaram(sign, entryUnixSeconds, latitude, longitude, elevation = 0, meridian = "", locale = "und") {
    const fn = this.#export("hc_pushkaram");
    const entry = toI64(entryUnixSeconds, "entryUnixSeconds");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(sign, "sign", (signPointer, signLen) =>
      this.#withText(meridian, "meridian", (meridianPointer, meridianLen) =>
        this.#withText(locale, "locale", (localePointer, localeLen) =>
          this.#text("hc_pushkaram", (buffer, capacity) =>
            fn(signPointer, signLen, entry, lat, lon, elev, meridianPointer, meridianLen, localePointer, localeLen,
              buffer, capacity), true))));
    return rows(text, COLUMNS.pushkaram, "hc_pushkaram").map(pushkaramDays);
  }

  /**
   * The folk reckonings of a day outside the Japanese almanac, each named
   * in a locale: the first-month counts, 入梅 and 出梅, the Vietnamese days
   * and the Turkish folk year.
   *
   * @param {number | bigint} fixed
   * @param {string} [meridian]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").FolkDay[]}
   */
  folkDay(fixed, meridian = "", locale = "und") {
    const fn = this.#export("hc_folk_day");
    const day = toI64(fixed, "fixed");
    const text = this.#withText(meridian, "meridian", (meridianPointer, meridianLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#text("hc_folk_day", (buffer, capacity) =>
          fn(day, meridianPointer, meridianLen, localePointer, localeLen, buffer, capacity), true)));
    return rows(text, COLUMNS.folkDay, "hc_folk_day").map(folkDay);
  }

  /**
   * The Chinese night watch of a time of the civil clock by the fixed
   * reckoning, or `null` from 05:00 to 18:59.
   *
   * @param {number} secondsOfDay
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").NightWatch | null}
   */
  nightWatch(secondsOfDay, locale = "und") {
    const fn = this.#export("hc_night_watch");
    const seconds = toU32(secondsOfDay, "secondsOfDay");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_night_watch", (buffer, capacity) => fn(seconds, pointer, len, buffer, capacity), true));
    return text === "" ? null : nightWatch(this.#oneLine("hc_night_watch", text, COLUMNS.nightWatch));
  }

  /**
   * What a day is in the Eastern Orthodox fasting scheme of a reckoning.
   *
   * @param {import("./hyper-calendar.d.ts").OrthodoxFastReckoning} reckoning
   * @param {number | bigint} fixed
   * @returns {import("./hyper-calendar.d.ts").OrthodoxFastDay}
   */
  orthodoxFastOn(reckoning, fixed) {
    const fn = this.#export("hc_orthodox_fast_on");
    const day = toI64(fixed, "fixed");
    const text = this.#withText(reckoning, "reckoning", (pointer, len) =>
      this.#text("hc_orthodox_fast_on", (buffer, capacity) => fn(pointer, len, day, buffer, capacity), true));
    return orthodoxFastDay(this.#oneLine("hc_orthodox_fast_on", text, COLUMNS.orthodoxFast));
  }

  /**
   * The fasting seasons and fast-free weeks of a year of a reckoning.
   *
   * @param {import("./hyper-calendar.d.ts").OrthodoxFastReckoning} reckoning
   * @param {number | bigint} year
   * @returns {import("./hyper-calendar.d.ts").OrthodoxFastSeason[]}
   */
  orthodoxFastSeasons(reckoning, year) {
    const fn = this.#export("hc_orthodox_fast_seasons");
    const y = toI64(year, "year");
    const text = this.#withText(reckoning, "reckoning", (pointer, len) =>
      this.#text("hc_orthodox_fast_seasons", (buffer, capacity) => fn(pointer, len, y, buffer, capacity), true));
    return rows(text, COLUMNS.orthodoxFastSeasons, "hc_orthodox_fast_seasons").map(orthodoxFastSeason);
  }

  /**
   * The Islamic prayer times of a day at a place by a named method.
   *
   * @param {string} method
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @param {boolean} [ramadan]
   * @returns {import("./hyper-calendar.d.ts").PrayerTime[]}
   */
  prayerTimes(method, fixed, latitude, longitude, elevation = 0, ramadan = false) {
    const fn = this.#export("hc_prayer_times");
    const day = toI64(fixed, "fixed");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(method, "method", (pointer, len) =>
      this.#text("hc_prayer_times", (buffer, capacity) =>
        fn(pointer, len, day, lat, lon, elev, ramadan ? 1 : 0, buffer, capacity), true));
    return rows(text, COLUMNS.prayerTimes, "hc_prayer_times").map(prayerTime);
  }

  /**
   * Every prayer-time method, with its parameters and source.
   *
   * @returns {import("./hyper-calendar.d.ts").PrayerMethod[]}
   */
  prayerMethods() {
    const fn = this.#export("hc_prayer_methods");
    const text = this.#text("hc_prayer_methods", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(text, COLUMNS.prayerMethods, "hc_prayer_methods").map(prayerMethod);
  }

  /**
   * The Jewish times of a day at a place by a reckoning, with the dawns
   * and nightfalls.
   *
   * @param {import("./hyper-calendar.d.ts").ZmanimReckoning} reckoning
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @returns {import("./hyper-calendar.d.ts").Zman[]}
   */
  zmanim(reckoning, fixed, latitude, longitude, elevation = 0) {
    const fn = this.#export("hc_zmanim");
    const day = toI64(fixed, "fixed");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(reckoning, "reckoning", (pointer, len) =>
      this.#text("hc_zmanim", (buffer, capacity) => fn(pointer, len, day, lat, lon, elev, buffer, capacity), true));
    return rows(text, COLUMNS.zmanim, "hc_zmanim").map(zman);
  }

  /**
   * The Edo 不定時法 reading of an instant at a place.
   *
   * @param {number | bigint} unixSeconds
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @returns {import("./hyper-calendar.d.ts").EdoTime}
   */
  edoTime(unixSeconds, latitude, longitude, elevation = 0) {
    const fn = this.#export("hc_edo_time");
    const instant = toI64(unixSeconds, "unixSeconds");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#text("hc_edo_time", (buffer, capacity) => fn(instant, lat, lon, elev, buffer, capacity), true);
    return edoTime(this.#oneLine("hc_edo_time", text, COLUMNS.edoTime));
  }

  /**
   * The instant of an Edo 不定時法 reading at a place.
   *
   * @param {number | bigint} fixed
   * @param {number} hour 0 to 11, from 明け六つ
   * @param {number} fraction 0 up to 1
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @returns {import("./hyper-calendar.d.ts").SolarEvent}
   */
  unixFromEdoTime(fixed, hour, fraction, latitude, longitude, elevation = 0) {
    const fn = this.#export("hc_unix_from_edo_time");
    const day = toI64(fixed, "fixed");
    const h = toU32(hour, "hour");
    const f = toF64(fraction, "fraction");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#text("hc_unix_from_edo_time", (buffer, capacity) =>
      fn(day, h, f, lat, lon, elev, buffer, capacity), true);
    return solarEvent(this.#oneLine("hc_unix_from_edo_time", text, COLUMNS.solarEvent));
  }
  /**
   * The planetary hour at an instant and a place, its ruler named in a
   * locale.
   *
   * @param {number | bigint} unixSeconds
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").PlanetaryHour}
   */
  planetaryHour(unixSeconds, latitude, longitude, elevation = 0, locale = "und") {
    const fn = this.#export("hc_planetary_hour");
    const seconds = toI64(unixSeconds, "unixSeconds");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_planetary_hour", (buffer, capacity) =>
        fn(seconds, lat, lon, elev, pointer, len, buffer, capacity), true));
    return planetaryHour(this.#oneLine("hc_planetary_hour", text, COLUMNS.planetaryHour));
  }

  /**
   * The twenty-four planetary hours of the day that begins at a fixed
   * day's sunrise at a place.
   *
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").PlanetaryHour[]}
   */
  planetaryHoursOfDay(fixed, latitude, longitude, elevation = 0, locale = "und") {
    const fn = this.#export("hc_planetary_hours_of_day");
    const day = toI64(fixed, "fixed");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_planetary_hours_of_day", (buffer, capacity) =>
        fn(day, lat, lon, elev, pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.planetaryHour, "hc_planetary_hours_of_day").map(planetaryHour);
  }

}

/**
 * The instance a source stands for.
 *
 * @param {import("./hyper-calendar.d.ts").ModuleSource} source
 * @returns {Promise<WebAssembly.Instance | {exports: Record<string, any>}>}
 */
async function instantiate(source) {
  if (source instanceof WebAssembly.Instance) {
    return source;
  }
  if (source instanceof WebAssembly.Module) {
    return new WebAssembly.Instance(source, {});
  }
  if (typeof source === "string" || (typeof URL !== "undefined" && source instanceof URL)) {
    return instantiate(await fetch(source));
  }
  if (typeof Response !== "undefined" && source instanceof Response) {
    if (!source.ok) {
      raise(`the module could not be fetched: ${source.status} ${source.statusText}`);
    }
    const type = source.headers.get("content-type") ?? "";
    if (typeof WebAssembly.instantiateStreaming === "function" && /^application\/wasm\b/i.test(type)) {
      return (await WebAssembly.instantiateStreaming(source, {})).instance;
    }
    return instantiate(await source.arrayBuffer());
  }
  if (source instanceof ArrayBuffer || ArrayBuffer.isView(source)) {
    return (await WebAssembly.instantiate(/** @type {any} */ (source), {})).instance;
  }
  if (source !== null && typeof source === "object") {
    if ("instance" in source && source.instance instanceof WebAssembly.Instance) {
      return source.instance;
    }
    if ("exports" in source && typeof source.exports === "object" && source.exports !== null) {
      return /** @type {{exports: Record<string, any>}} */ (source);
    }
  }
  return raise("load() takes a WebAssembly.Module or Instance, an ArrayBuffer or Uint8Array of the module, a Response, or a URL");
}

/**
 * Instantiate the module and bind it.
 *
 * `source` is a `WebAssembly.Module` or `Instance`, the module's bytes as
 * an `ArrayBuffer` or `Uint8Array`, a `Response` carrying them, or a `URL`
 * or string to fetch them from; a promise of any of those is awaited.
 *
 * @param {import("./hyper-calendar.d.ts").ModuleSource | Promise<import("./hyper-calendar.d.ts").ModuleSource>} source
 * @param {import("./hyper-calendar.d.ts").LoadOptions} [options]
 * @returns {Promise<HyperCalendar>}
 */
export async function load(source, options = {}) {
  return new HyperCalendar(await instantiate(await source), options);
}
