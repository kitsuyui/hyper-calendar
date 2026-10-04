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
  { method: "gmtime", export: "hc_gmtime", feature: "civil" },
  { method: "timegm", export: "hc_timegm", feature: "civil" },
  { method: "isleap", export: "hc_isleap", feature: "civil" },
  { method: "leapdays", export: "hc_leapdays", feature: "civil" },
  { method: "calendarWeekday", export: "hc_calendar_weekday", feature: "civil" },
  { method: "monthrange", export: "hc_monthrange", feature: "civil" },
  { method: "monthcalendar", export: "hc_monthcalendar", feature: "civil" },
  { method: "weekOfYear", export: "hc_week_of_year", feature: "civil" },
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
  { method: "taiMinusUtcExact", export: "hc_tai_minus_utc_exact", feature: "timestamps" },
  { method: "utcFromTaiExact", export: "hc_utc_from_tai_exact", feature: "timestamps" },
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
  { method: "epochs", export: "hc_epochs", feature: "timestamps" },
  { method: "ccsdsDecode", export: "hc_ccsds_decode", feature: "time-codes" },
  { method: "ccsdsEncode", export: "hc_ccsds_encode", feature: "time-codes" },
  { method: "ccsdsDecodeFromEpoch", export: "hc_ccsds_decode_from_epoch", feature: "time-codes" },
  { method: "ccsdsEncodeFromEpoch", export: "hc_ccsds_encode_from_epoch", feature: "time-codes" },
  { method: "ccsdsAsciiParse", export: "hc_ccsds_ascii_parse", feature: "time-codes" },
  { method: "ccsdsAsciiFormat", export: "hc_ccsds_ascii_format", feature: "time-codes" },
  { method: "radioDecode", export: "hc_radio_decode", feature: "time-codes" },
  { method: "radioEncode", export: "hc_radio_encode", feature: "time-codes" },
  { method: "jjyCallSignDecode", export: "hc_jjy_call_sign_decode", feature: "time-codes" },
  { method: "jjyCallSignEncode", export: "hc_jjy_call_sign_encode", feature: "time-codes" },
  { method: "irigDecode", export: "hc_irig_decode", feature: "time-codes" },
  { method: "irigEncode", export: "hc_irig_encode", feature: "time-codes" },
  { method: "irigFrameStart", export: "hc_irig_frame_start", feature: "time-codes" },
  { method: "irigFormats", export: "hc_irig_formats", feature: "time-codes" },
  { method: "dotnetTicksFromUnix", export: "hc_dotnet_ticks_from_unix", feature: "timestamps" },
  { method: "unixFromDotnetTicks", export: "hc_unix_from_dotnet_ticks", feature: "timestamps" },
  { method: "sixHourClock", export: "hc_six_hour_clock", feature: "timestamps" },
  { method: "civilFromSixHourClock", export: "hc_civil_from_six_hour_clock", feature: "timestamps" },
  { method: "frenchDecimalTime", export: "hc_french_decimal_time", feature: "timestamps" },
  { method: "civilFromFrenchDecimalTime", export: "hc_civil_from_french_decimal_time", feature: "timestamps" },
  { method: "describeDay", export: "hc_describe_day", feature: "calendars" },
  { method: "dayExtras", export: "hc_day_extras", feature: "calendars" },
  { method: "calendarUnits", export: "hc_calendar_units", feature: "calendars" },
  { method: "parseDate", export: "hc_parse_date", feature: "calendars" },
  { method: "calendars", export: "hc_calendars", feature: "calendars" },
  { method: "calendarList", export: "hc_calendar_list", feature: "calendars" },
  { method: "locales", export: "hc_locales", feature: "calendars" },
  { method: "firstDayOfWeek", export: "hc_first_day_of_week", feature: "calendars" },
  { method: "localeChain", export: "hc_locale_chain", feature: "calendars" },
  { method: "localeInfo", export: "hc_locale_info", feature: "calendars" },
  { method: "pluralCategory", export: "hc_plural_category", feature: "calendars" },
  { method: "pluralCategories", export: "hc_plural_categories", feature: "calendars" },
  { method: "localeFormat", export: "hc_locale_format", feature: "calendars" },
  { method: "japaneseEraYear", export: "hc_japanese_era_year", feature: "calendars" },
  { method: "names", export: "hc_names", feature: "calendars" },
  { method: "caseText", export: "hc_case", feature: "calendars" },
  { method: "isolate", export: "hc_isolate", feature: "calendars" },
  { method: "dayPeriod", export: "hc_day_period", feature: "calendars" },
  { method: "formatNumber", export: "hc_format_number", feature: "calendars" },
  { method: "parseNumber", export: "hc_parse_number", feature: "calendars" },
  { method: "numberingSystems", export: "hc_numbering_systems", feature: "calendars" },
  { method: "calendarEras", export: "hc_calendar_eras", feature: "calendars" },
  { method: "eraTable", export: "hc_era_table", feature: "calendars" },
  { method: "olympicGames", export: "hc_olympic_games", feature: "calendars" },
  { method: "gregorianAdoption", export: "hc_gregorian_adoption", feature: "calendars" },
  { method: "namingPeriodOn", export: "hc_naming_period_on", feature: "calendars" },
  { method: "panchangaAt", export: "hc_panchanga_at", feature: "calendars" },
  { method: "tithiAt", export: "hc_tithi_at", feature: "calendars" },
  { method: "tithisOfDay", export: "hc_tithis_of_day", feature: "calendars" },
  { method: "ayanamsas", export: "hc_ayanamsas", feature: "calendars" },
  { method: "ayanamsaAt", export: "hc_ayanamsa_at", feature: "calendars" },
  { method: "ayanamsaFromAnchor", export: "hc_ayanamsa_from_anchor", feature: "calendars" },
  { method: "festivalReadings", export: "hc_festival_readings", feature: "calendars" },
  { method: "janmashtami", export: "hc_janmashtami", feature: "calendars" },
  { method: "vaishnavaDay", export: "hc_vaishnava_day", feature: "calendars" },
  { method: "vishtiFreeSpan", export: "hc_vishti_free_span", feature: "calendars" },
  { method: "rahuAt", export: "hc_rahu_at", feature: "calendars" },
  { method: "rahuIngresses", export: "hc_rahu_ingresses", feature: "calendars" },
  { method: "solarNakshatraIngresses", export: "hc_solar_nakshatra_ingresses", feature: "calendars" },
  { method: "tiruvalluvarYear", export: "hc_tiruvalluvar_year", feature: "calendars" },
  { method: "eraNewYear", export: "hc_era_new_year", feature: "calendars" },
  { method: "panchangaOfDay", export: "hc_panchanga_of_day", feature: "calendars" },
  { method: "hinduLunarDate", export: "hc_hindu_lunar_date", feature: "calendars" },
  { method: "suryaSiddhantaAt", export: "hc_surya_siddhanta_at", feature: "calendars" },
  { method: "suryaSiddhantaSunrise", export: "hc_surya_siddhanta_sunrise", feature: "calendars" },
  { method: "barhaspatyaYear", export: "hc_barhaspatya_year", feature: "calendars" },
  { method: "barhaspatyaYearAt", export: "hc_barhaspatya_year_at", feature: "calendars" },
  { method: "muhurtas", export: "hc_muhurtas", feature: "calendars" },
  { method: "amritaSiddhi", export: "hc_amrita_siddhi", feature: "calendars" },
  { method: "nakshatraAt", export: "hc_nakshatra_at", feature: "calendars" },
  { method: "nakshatraOfDay", export: "hc_nakshatra_of_day", feature: "calendars" },
  { method: "crescentVisible", export: "hc_crescent_visible", feature: "calendars" },
  { method: "iocOlympiad", export: "hc_ioc_olympiad", feature: "calendars" },
  { method: "iocOlympiadOn", export: "hc_ioc_olympiad_on", feature: "calendars" },
  { method: "babylonianRegnalYear", export: "hc_babylonian_regnal_year", feature: "calendars" },
  { method: "equinoxNewYearMargin", export: "hc_equinox_new_year_margin", feature: "calendars" },
  { method: "shmuelTekufah", export: "hc_shmuel_tekufah", feature: "calendars" },
  { method: "dayName", export: "hc_day_name", feature: "calendars" },
  { method: "hebrewYahrzeit", export: "hc_hebrew_yahrzeit", feature: "calendars" },
  { method: "hebrewBirthday", export: "hc_hebrew_birthday", feature: "calendars" },
  { method: "chineseReckonedAge", export: "hc_chinese_reckoned_age", feature: "calendars" },
  { method: "chineseMarriageAugury", export: "hc_chinese_marriage_augury", feature: "calendars" },
  { method: "chineseAge", export: "hc_chinese_age", feature: "calendars" },
  { method: "chineseAlmanacSolarTerms", export: "hc_chinese_almanac_solar_terms", feature: "calendars" },
  { method: "hebrewSabbaticalCycleYear", export: "hc_hebrew_sabbatical_cycle_year", feature: "calendars" },
  { method: "asianDay", export: "hc_asian_day", feature: "calendars" },
  { method: "solarNewYear", export: "hc_solar_new_year", feature: "calendars" },
  { method: "southeastAsianYearType", export: "hc_southeast_asian_year_type", feature: "calendars" },
  { method: "mayaLongCount", export: "hc_maya_long_count", feature: "calendars" },
  { method: "akanDay", export: "hc_akan_day", feature: "calendars" },
  { method: "weton", export: "hc_weton", feature: "calendars" },
  { method: "buddhistLkYear", export: "hc_buddhist_lk_year", feature: "calendars" },
  { method: "kalam", export: "hc_kalam", feature: "calendars" },
  { method: "almanacCycles", export: "hc_almanac_cycles", feature: "calendars" },
  { method: "almanacDay", export: "hc_almanac_day", feature: "calendars" },
  { method: "almanacDirections", export: "hc_almanac_directions", feature: "calendars" },
  { method: "rounichi", export: "hc_rounichi", feature: "calendars" },
  { method: "mansionUndertakings", export: "hc_mansion_undertakings", feature: "calendars" },
  { method: "almanacPersonDays", export: "hc_almanac_person_days", feature: "calendars" },
  { method: "tibetanAlmanacDay", export: "hc_tibetan_almanac_day", feature: "calendars" },
  { method: "tibetanPlanets", export: "hc_tibetan_planets", feature: "calendars" },
  { method: "bhutaneseWinterSolstice", export: "hc_bhutanese_winter_solstice", feature: "calendars" },
  { method: "tibetanFestivalDay", export: "hc_tibetan_festival_day", feature: "calendars" },
  { method: "choghadiya", export: "hc_choghadiya", feature: "calendars" },
  { method: "panchak", export: "hc_panchak", feature: "calendars" },
  { method: "kumbh", export: "hc_kumbh", feature: "calendars" },
  { method: "kumbhYogas", export: "hc_kumbh_yogas", feature: "calendars" },
  { method: "pushkaramRivers", export: "hc_pushkaram_rivers", feature: "calendars" },
  { method: "pushkaram", export: "hc_pushkaram", feature: "calendars" },
  { method: "folkDay", export: "hc_folk_day", feature: "calendars" },
  { method: "nightWatch", export: "hc_night_watch", feature: "calendars" },
  { method: "holidayIsDayOff", export: "hc_holiday_is_day_off", feature: "holiday" },
  { method: "holidayAddBusinessDays", export: "hc_holiday_add_business_days", feature: "holiday" },
  { method: "holidayBusinessDaysBetween", export: "hc_holiday_business_days_between", feature: "holiday" },
  { method: "holidayIsWeekend", export: "hc_holiday_is_weekend", feature: "holiday" },
  { method: "holidayNext", export: "hc_holiday_next", feature: "holiday" },
  { method: "holidayPrevious", export: "hc_holiday_previous", feature: "holiday" },
  { method: "holidaysInYear", export: "hc_holidays_in_year", feature: "holiday" },
  { method: "holidayCodes", export: "hc_holiday_codes", feature: "holiday" },
  { method: "holidaysOn", export: "hc_holidays_on", feature: "holiday" },
  { method: "holidayTables", export: "hc_holiday_tables", feature: "holiday" },
  { method: "holidayGroups", export: "hc_holiday_groups", feature: "holiday" },
  { method: "holidayCoverage", export: "hc_holiday_coverage", feature: "holiday" },
  { method: "holidaysOnIn", export: "hc_holidays_on_in", feature: "holiday" },
  { method: "lectionary", export: "hc_lectionary", feature: "holiday" },
  { method: "astronomicalEaster", export: "hc_astronomical_easter", feature: "holiday" },
  { method: "astronomicalPaschalFullMoon", export: "hc_astronomical_paschal_full_moon", feature: "holiday" },
  { method: "holyYearOn", export: "hc_holy_year_on", feature: "holiday" },
  { method: "commonWorshipOn", export: "hc_common_worship_on", feature: "holiday" },
  { method: "roman1960OfficeOn", export: "hc_roman_1960_office_on", feature: "holiday" },
  { method: "orthodoxFastOn", export: "hc_orthodox_fast_on", feature: "holiday" },
  { method: "orthodoxFastSeasons", export: "hc_orthodox_fast_seasons", feature: "holiday" },
  { method: "termInEffect", export: "hc_term_in_effect", feature: "seasons" },
  { method: "pentadInEffect", export: "hc_pentad_in_effect", feature: "seasons" },
  { method: "pentadTraditions", export: "hc_pentad_traditions", feature: "seasons" },
  { method: "pentadInTradition", export: "hc_pentad_in_tradition", feature: "seasons" },
  { method: "zassetsuInYear", export: "hc_zassetsu_in_year", feature: "seasons" },
  { method: "seasonalDaysInYear", export: "hc_seasonal_days_in_year", feature: "seasons" },
  { method: "pentadsInYear", export: "hc_pentads_in_year", feature: "seasons" },
  { method: "tropicalSignsInYear", export: "hc_tropical_signs_in_year", feature: "seasons" },
  { method: "siderealSignsInYear", export: "hc_sidereal_signs_in_year", feature: "seasons" },
  { method: "traditionalTanabata", export: "hc_traditional_tanabata", feature: "seasons" },
  { method: "principalPhasesInMonth", export: "hc_principal_phases_in_month", feature: "seasons" },
  { method: "coldFoodDay", export: "hc_cold_food_day", feature: "seasons" },
  { method: "plumRains", export: "hc_plum_rains", feature: "seasons" },
  { method: "placeYearsAgo", export: "hc_place_years_ago", feature: "deep-time" },
  { method: "cosmicEvents", export: "hc_cosmic_events", feature: "deep-time" },
  { method: "earliestEvidence", export: "hc_earliest_evidence", feature: "deep-time" },
  { method: "archaeologicalPeriods", export: "hc_archaeological_periods", feature: "deep-time" },
  { method: "futureEvents", export: "hc_future_events", feature: "deep-time" },
  { method: "geologicIntervals", export: "hc_geologic_intervals", feature: "deep-time" },
  { method: "planckUnits", export: "hc_planck_units", feature: "deep-time" },
  { method: "bpConvert", export: "hc_bp_convert", feature: "deep-time" },
  { method: "deepConvert", export: "hc_deep_convert", feature: "deep-time" },
  { method: "deepCompare", export: "hc_deep_compare", feature: "deep-time" },
  { method: "fixedFromUnixInZone", export: "hc_fixed_from_unix_in_zone", feature: "tz" },
  { method: "unixFromFixedInZone", export: "hc_unix_from_fixed_in_zone", feature: "tz" },
  { method: "loadZone", export: "hc_zone_load", feature: "tz" },
  { method: "zones", export: "hc_zones", feature: "tz" },
  { method: "zoneLocation", export: "hc_zone_location", feature: "tz" },
  { method: "zoneOffset", export: "hc_zone_offset", feature: "tz" },
  { method: "localtime", export: "hc_localtime", feature: "tz" },
  { method: "mktime", export: "hc_mktime", feature: "tz" },
  { method: "skyAt", export: "hc_sky_at", feature: "sky" },
  { method: "solarTermsBetween", export: "hc_solar_terms_between", feature: "sky" },
  { method: "moonPhasesBetween", export: "hc_moon_phases_between", feature: "sky" },
  { method: "decanAt", export: "hc_decan_at", feature: "sky" },
  { method: "drekkanaAt", export: "hc_drekkana_at", feature: "sky" },
  { method: "earthRotationAngle", export: "hc_earth_rotation_angle", feature: "sky" },
  { method: "gmstIau2006", export: "hc_gmst_iau2006", feature: "sky" },
  { method: "gmstIau1982", export: "hc_gmst_iau1982", feature: "sky" },
  { method: "ut2MinusUt1", export: "hc_ut2_minus_ut1", feature: "sky" },
  { method: "ut1rIers2010", export: "hc_ut1r_iers2010", feature: "sky" },
  { method: "ut1sIers2010", export: "hc_ut1s_iers2010", feature: "sky" },
  { method: "zonalTideUt1Effect", export: "hc_zonal_tide_ut1_effect", feature: "sky" },
  { method: "equationOfTime", export: "hc_equation_of_time", feature: "sky" },
  { method: "solarNoon", export: "hc_solar_noon", feature: "sky" },
  { method: "solarMidnight", export: "hc_solar_midnight", feature: "sky" },
  { method: "solarTime", export: "hc_solar_time", feature: "sky" },
  { method: "solarEvent", export: "hc_solar_event", feature: "sky" },
  { method: "horizons", export: "hc_horizons", feature: "sky" },
  { method: "sunrise", export: "hc_sunrise", feature: "sky" },
  { method: "sunset", export: "hc_sunset", feature: "sky" },
  { method: "moonrise", export: "hc_moonrise", feature: "sky" },
  { method: "moonset", export: "hc_moonset", feature: "sky" },
  { method: "dawn", export: "hc_dawn", feature: "sky" },
  { method: "dusk", export: "hc_dusk", feature: "sky" },
  { method: "hjdTt", export: "hc_hjd_tt", feature: "sky" },
  { method: "hjdUtc", export: "hc_hjd_utc", feature: "sky" },
  { method: "gmatFromGmt", export: "hc_gmat_from_gmt", feature: "sky" },
  { method: "gmtFromGmat", export: "hc_gmt_from_gmat", feature: "sky" },
  { method: "prayerTimes", export: "hc_prayer_times", feature: "sky" },
  { method: "prayerMethods", export: "hc_prayer_methods", feature: "sky" },
  { method: "zmanim", export: "hc_zmanim", feature: "sky" },
  { method: "temporalHour", export: "hc_temporal_hour", feature: "sky" },
  { method: "edoTime", export: "hc_edo_time", feature: "sky" },
  { method: "unixFromEdoTime", export: "hc_unix_from_edo_time", feature: "sky" },
  { method: "planetaryHour", export: "hc_planetary_hour", feature: "sky" },
  { method: "planetaryHoursOfDay", export: "hc_planetary_hours_of_day", feature: "sky" },
  { method: "orbitAt", export: "hc_orbit_at", feature: "orbital" },
  { method: "orbitSeries", export: "hc_orbit_series", feature: "orbital" },
  { method: "dailyInsolation", export: "hc_daily_insolation", feature: "orbital" },
  { method: "jupiterAt", export: "hc_jupiter_at", feature: "jupiter" },
  { method: "jupiterIngresses", export: "hc_jupiter_ingresses", feature: "jupiter" },
  { method: "jupiterRisings", export: "hc_jupiter_risings", feature: "jupiter" },
  { method: "kumbhBySky", export: "hc_kumbh_by_sky", feature: "jupiter" },
  { method: "kumbhsInYearBySky", export: "hc_kumbhs_in_year_by_sky", feature: "jupiter" },
  { method: "jupiterStations", export: "hc_jupiter_stations", feature: "jupiter" },
  { method: "pushkaramRules", export: "hc_pushkaram_rules", feature: "jupiter" },
  { method: "pushkaramBySky", export: "hc_pushkaram_by_sky", feature: "jupiter" },
  { method: "pushkaramsInYear", export: "hc_pushkarams_in_year", feature: "jupiter" },
  { method: "marsTime", export: "hc_mars_time", feature: "planetary" },
  { method: "missions", export: "hc_missions", feature: "planetary" },
  { method: "missionSol", export: "hc_mission_sol", feature: "planetary" },
  { method: "bodies", export: "hc_bodies", feature: "planetary" },
  { method: "bodyTime", export: "hc_body_time", feature: "planetary" },
  { method: "circadDate", export: "hc_circad_date", feature: "planetary" },
  { method: "properTime", export: "hc_proper_time", feature: "relativity" },
  { method: "gravitationalDilation", export: "hc_gravitational_dilation", feature: "relativity" },
  { method: "gravitatingBodies", export: "hc_gravitating_bodies", feature: "relativity" },
  { method: "orbitRateOffset", export: "hc_orbit_rate_offset", feature: "relativity" },
  { method: "rocket", export: "hc_rocket", feature: "relativity" },
  { method: "flipAndBurn", export: "hc_flip_and_burn", feature: "relativity" },
  { method: "doppler", export: "hc_doppler", feature: "relativity" },
  { method: "velocityAdd", export: "hc_velocity_add", feature: "relativity" },
  { method: "schwarzschildRadius", export: "hc_schwarzschild_radius", feature: "relativity" },
  { method: "properTimeUncertain", export: "hc_proper_time_uncertain", feature: "relativity" },
  { method: "territories", export: "hc_territories", feature: "places" },
  { method: "subdivisions", export: "hc_subdivisions", feature: "places" },
  { method: "placeName", export: "hc_place_name", feature: "places" },
  { method: "relativeTime", export: "hc_relative_time", feature: "humanize" },
  { method: "relativeDay", export: "hc_relative_day", feature: "humanize" },
  { method: "relativeDayAt", export: "hc_relative_day_at", feature: "humanize" },
  { method: "duration", export: "hc_duration", feature: "humanize" },
  { method: "unitChoice", export: "hc_unit_choice", feature: "humanize" },
  { method: "relativeTimeWith", export: "hc_relative_time_with", feature: "humanize" },
  { method: "approximateDuration", export: "hc_approximate_duration", feature: "humanize" },
  { method: "listForms", export: "hc_list_forms", feature: "humanize" },
  { method: "apnumber", export: "hc_apnumber", feature: "natural" },
  { method: "fractional", export: "hc_fractional", feature: "natural" },
  { method: "scientific", export: "hc_scientific", feature: "natural" },
  { method: "metric", export: "hc_metric", feature: "natural" },
  { method: "naturalSize", export: "hc_naturalsize", feature: "natural" },
  { method: "naturalList", export: "hc_naturallist", feature: "natural" },
  { method: "intword", export: "hc_intword", feature: "natural" },
  { method: "naturalDelta", export: "hc_naturaldelta", feature: "natural" },
  { method: "naturalTime", export: "hc_naturaltime", feature: "natural" },
  { method: "preciseDelta", export: "hc_precisedelta", feature: "natural" },
  { method: "naturalDay", export: "hc_naturalday", feature: "natural" },
  { method: "naturalDate", export: "hc_naturaldate", feature: "natural" },
  { method: "ordinal", export: "hc_ordinal", feature: "natural" },
  { method: "intcomma", export: "hc_intcomma", feature: "natural" },
  { method: "intcommaFloat", export: "hc_intcomma_float", feature: "natural" },
  { method: "clamp", export: "hc_clamp", feature: "natural" },
  { method: "parseDatetime", export: "hc_parse_datetime", feature: "datetime" },
  { method: "formatDatetime", export: "hc_format_datetime", feature: "datetime" },
  { method: "formatIsoDateAs", export: "hc_format_iso_date_as", feature: "datetime" },
  { method: "isoDateParts", export: "hc_iso_date_parts", feature: "datetime" },
  { method: "isoDuration", export: "hc_iso_duration", feature: "datetime" },
  { method: "formatIsoDuration", export: "hc_format_iso_duration", feature: "datetime" },
  { method: "isoInterval", export: "hc_iso_interval", feature: "datetime" },
  { method: "parsePattern", export: "hc_parse_pattern", feature: "patterns" },
  { method: "parsePatternIn", export: "hc_parse_pattern_in", feature: "patterns" },
  { method: "zoneName", export: "hc_zone_name", feature: "zone-names" },
  { method: "formatPattern", export: "hc_format_pattern", feature: "zone-names" },
  { method: "edtfParse", export: "hc_edtf_parse", feature: "uncertainty" },
  { method: "edtfRelations", export: "hc_edtf_relations", feature: "uncertainty" },
  { method: "significant", export: "hc_significant", feature: "uncertainty" },
  { method: "significantOp", export: "hc_significant_op", feature: "uncertainty" },
  { method: "uncertain", export: "hc_uncertain", feature: "uncertainty" },
  { method: "uncertainOp", export: "hc_uncertain_op", feature: "uncertainty" },
  { method: "interval", export: "hc_interval", feature: "uncertainty" },
  { method: "units", export: "hc_units", feature: "units" },
  { method: "unitConvert", export: "hc_unit_convert", feature: "units" },
  { method: "rates", export: "hc_rates", feature: "units" },
  { method: "framePeriod", export: "hc_frame_period", feature: "units" },
  { method: "tempo", export: "hc_tempo", feature: "units" },
  { method: "fiscalProfiles", export: "hc_fiscal_profiles", feature: "fiscal" },
  { method: "fiscalYearOn", export: "hc_fiscal_year_on", feature: "fiscal" },
  { method: "fiscalYearSpan", export: "hc_fiscal_year_span", feature: "fiscal" },
  { method: "weekYearSystems", export: "hc_week_year_systems", feature: "fiscal" },
  { method: "weekYearOn", export: "hc_week_year_on", feature: "fiscal" },
  { method: "nameDayLists", export: "hc_name_day_lists", feature: "name-days" },
  { method: "nameDaysOn", export: "hc_name_days_on", feature: "name-days" },
  { method: "nameDay", export: "hc_name_day", feature: "name-days" },
  { method: "attributionAuthorities", export: "hc_attribution_authorities", feature: "attributes" },
  { method: "attributions", export: "hc_attributions", feature: "attributes" },
  { method: "attributionsOn", export: "hc_attributions_on", feature: "attributes" },
  { method: "harvestMoon", export: "hc_harvest_moon", feature: "attributes" },
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
    "calendars", "parent", "numbering", "direction",
  ]),
  reading: Object.freeze([
    "local day", "local second", "attoseconds", "zone", "offset", "unix seconds", "leap second", "end of day",
  ]),
  formattedDatetime: Object.freeze(["text", "syntax"]),
  formattedIsoDate: Object.freeze(["text", "form", "style"]),
  isoDateParts: Object.freeze([
    "form", "year", "month", "day", "day of year", "week", "weekday", "fixed", "style",
  ]),
  isoDuration: Object.freeze([
    "negative", "years", "months", "weeks", "days", "hours", "minutes", "seconds", "fraction", "form",
    "text", "nominal", "exact seconds", "exact attoseconds",
  ]),
  formattedIsoDuration: Object.freeze(["text", "nominal", "exact seconds", "exact attoseconds"]),
  isoInterval: Object.freeze([
    "repetitions", "shape", "start", "start unix seconds", "end", "end unix seconds", "duration",
    "nominal", "exact seconds", "exact attoseconds",
  ]),
  patternFields: Object.freeze([
    "year", "century", "year of century", "month", "day", "day of year", "iso year", "iso week", "iso weekday",
    "week sunday", "week monday", "hour", "hour 12", "day period", "minute", "second", "attoseconds",
    "zone", "offset", "unix seconds", "era", "fixed",
    "local day", "local second", "reading attoseconds", "reading zone", "reading offset",
    "reading unix seconds", "leap second", "end of day",
  ]),
  localeChain: Object.freeze(["step", "tag", "rule", "carried"]),
  localeInfo: Object.freeze([
    "tag", "language", "script", "region", "variant", "calendar key", "numbering key", "first day key",
    "hour cycle key", "locale used", "parent", "parent rule", "numbering", "first day", "min days",
    "direction", "casing", "capitalises month names", "plural rules",
  ]),
  pluralCategory: Object.freeze(["category", "rules", "i", "v", "w", "f", "t"]),
  names: Object.freeze(["kind", "position", "name", "locale used"]),
  caseText: Object.freeze(["text", "mode", "casing", "locale used"]),
  isolated: Object.freeze(["text", "direction", "text direction", "isolated", "mode"]),
  gregorianAdoption: Object.freeze([
    "last old day", "first day", "old calendar", "scope", "source", "new calendar", "polity",
  ]),
  holidaysInYear: Object.freeze([
    "date", "name", "local name", "kind", "confidence", "substitute", "observed for", "region",
    "group", "id", "source", "bridged", "window first", "window last",
  ]),
  holidaysOn: Object.freeze([
    "table", "table name", "name", "local name", "kind", "confidence", "source",
    "substitute", "observed for", "region", "group", "id", "bridged",
  ]),
  term: Object.freeze([
    "index", "chinese name", "japanese name", "begins", "ends",
    "chinese authority", "japanese authority",
  ]),
  pentadTraditions: Object.freeze(["id", "english name", "authority", "alternates"]),
  pentadInTradition: Object.freeze([
    "index", "name", "gloss", "alternate", "begins", "ends", "tradition", "authority",
  ]),
  zassetsu: Object.freeze([
    "id", "name", "romaji", "english name", "rule", "day", "last", "first ox day", "second ox day",
  ]),
  seasonalDay: Object.freeze(["kind", "id", "name", "local name", "first", "last", "group"]),
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
  tithi: Object.freeze(["number", "paksha", "paksha day", "name", "began", "ends", "read at", "sky"]),
  tithisOfDay: Object.freeze([
    "number", "paksha", "paksha day", "name", "began", "ends", "read at", "sky",
    "at sunrise", "repeated", "skipped",
  ]),
  ayanamsaTable: Object.freeze(["id", "name", "anchor julian date", "anchor degrees", "source", "kind"]),
  ayanamsaValue: Object.freeze(["degrees", "id", "name", "anchor julian date", "anchor degrees", "read at", "kind"]),
  festivalReading: Object.freeze(["id", "name", "rule", "source"]),
  janmashtami: Object.freeze([
    "reading", "year", "fixed", "gregorian year", "gregorian month", "gregorian day",
    "sunrise tithi", "ayanamsa",
  ]),
  vaishnavaDay: Object.freeze([
    "saka year", "month", "tithi", "fixed", "gregorian year", "gregorian month", "gregorian day",
    "sunrise tithi", "ayanamsa",
  ]),
  vishtiFreeSpan: Object.freeze([
    "saka year", "month", "tithi", "month first day", "begins", "ends", "ayanamsa",
  ]),
  rahuAt: Object.freeze([
    "node", "rahu longitude", "rahu number", "rahu sign", "rahu sign name", "ketu longitude",
    "ketu number", "ketu sign", "ketu sign name", "ayanamsa", "read at",
  ]),
  rahuIngress: Object.freeze([
    "moment", "from", "from name", "into", "into name", "ketu into", "ketu into name",
  ]),
  marriageAugury: Object.freeze([
    "augury", "lichun at start", "lichun at end", "chinese names", "name scripts", "name regions",
  ]),
  chineseAlmanacSolarTerms: Object.freeze(["position", "name", "fixed"]),
  holidayTables: Object.freeze([
    "code", "kind", "name", "english name", "locale used", "source", "country", "short name",
    "regions", "groups", "group names", "region groups", "read subdivisions", "weekend",
    "substitution",
  ]),
  lectionary: Object.freeze([
    "liturgical year",
    "sunday cycle",
    "weekday cycle",
    "proper",
    "sunday in ordinary time",
    "week of ordinary time",
    "week, epiphany on a sunday",
  ]),
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
  places: Object.freeze(["code", "name", "english name", "locale used", "draft", "status"]),
  relativeTime: Object.freeze(["phrase", "unit", "count", "locale used"]),
  relativeDayAt: Object.freeze(["phrase", "unit", "count", "time", "locale used"]),
  duration: Object.freeze(["phrase", "negative", "locale used"]),
  naturalText: Object.freeze(["text", "language"]),
  localizedNaturalText: Object.freeze(["text", "locale used"]),
  unitChoice: Object.freeze(["unit", "count", "half", "thresholds", "rounding"]),
  relativeTimeWith: Object.freeze(["phrase", "unit", "count", "half", "locale used"]),
  approximateDuration: Object.freeze(["phrase", "hedge", "unit", "count", "locale used"]),
  zoneName: Object.freeze(["name", "field", "zone", "offset", "daylight"]),
  utcFromTai: Object.freeze(["unix seconds", "leap second"]),
  taiMinusUtcExact: Object.freeze(["seconds", "attoseconds"]),
  utcFromTaiExact: Object.freeze(["unix seconds", "attoseconds", "leap second"]),
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
  suryaSiddhanta: Object.freeze(["sun", "moon", "elongation", "tithi", "sign", "sign id", "sign name"]),
  crescent: Object.freeze([
    "visible", "evaluated at", "elongation", "arc of light", "altitude", "arc of vision", "width",
  ]),
  decan: Object.freeze(["sign", "sign id", "sign name", "decan", "ruler", "ruler name", "degrees into decan"]),
  drekkana: Object.freeze([
    "sign", "sign id", "sign name", "drekkana", "lord", "lord name", "degrees into drekkana", "lord sign",
    "lord sign name", "ayanamsa",
  ]),
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
  commonWorship: Object.freeze(["title", "rank", "rank name", "id"]),
  roman1960Office: Object.freeze(["role", "title", "class", "class name", "transferred from"]),
  holidayGroups: Object.freeze(["group", "name", "locale used", "english name"]),
  holidayCoverage: Object.freeze([
    "region", "first read", "answered from", "answered until", "complete", "weekend from", "reason",
  ]),
  dayPeriod: Object.freeze(["half", "half name", "period", "abbreviated", "wide", "narrow", "locale used"]),
  numberingSystems: Object.freeze(["system", "algorithmic", "digits"]),
  calendarEras: Object.freeze(["code", "wide", "abbreviated", "narrow", "calendar", "locale used"]),
  eraTable: Object.freeze([
    "code", "name", "reading", "romanised", "group", "first year", "last year", "start", "last",
    "status", "other start", "note",
  ]),
  olympicGames: Object.freeze(["number", "year", "host", "status", "opening", "closing"]),
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
  jjyCallSign: Object.freeze([
    "unix seconds", "fixed", "hour", "minute", "stop start", "daytime only", "stop span",
  ]),
  unixFromDotnetTicks: Object.freeze(["unix seconds", "attoseconds"]),
  sixHourClock: Object.freeze(["hour", "minute", "second", "half", "period", "period english"]),
  frenchDecimalTime: Object.freeze(["hour", "minute", "second", "attoseconds"]),
  civilFromFrenchDecimalTime: Object.freeze(["seconds of day", "attoseconds"]),
  babylonianRegnalYear: Object.freeze(["king", "regnal year"]),
  equinoxMargin: Object.freeze(["minutes", "calendar"]),
  shmuelTekufah: Object.freeze(["fixed", "minutes", "tekufah", "after nightfall", "civil"]),
  dayName: Object.freeze(["name", "naming", "naming name", "authority"]),
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
  almanacDirections: Object.freeze([
    "id", "name", "reading", "branch", "azimuth", "meaning", "year", "branch number",
  ]),
  mansionUndertakings: Object.freeze(["mansion", "mansion name", "grade", "undertaking"]),
  almanacPersonDays: Object.freeze(["kind", "id", "name", "keeps", "applies"]),
  tibetanAlmanacDay: Object.freeze(["kind", "id", "name", "tibetan", "reading", "value"]),
  tibetanPlanets: Object.freeze([
    "planet", "particular day", "mean heliocentric", "true slow", "fast",
    "mean heliocentric value", "true slow value", "fast value",
  ]),
  bhutaneseWinterSolstice: Object.freeze(["fixed", "reading", "julian date"]),
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
  temporalHour: Object.freeze([
    "reckoning", "seconds", "missing", "missing day", "depression", "depression arcseconds",
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
    "id", "site", "site name", "locale used", "river", "jupiter", "jupiter name", "sun", "sun name",
    "at new moon", "from", "to", "holds",
  ]),
  pushkaram: Object.freeze([
    "id", "name", "locale used", "region", "sign", "sign name", "first", "last", "missing",
    "missing day", "depression", "depression arcseconds",
  ]),
  jupiterAt: Object.freeze([
    "longitude", "latitude", "distance", "sidereal longitude", "sign", "sign name", "degrees into sign",
    "daily motion", "retrograde", "heliocentric longitude", "heliocentric latitude", "heliocentric distance",
  ]),
  jupiterIngress: Object.freeze(["moment", "from", "from name", "to", "to name", "direction"]),
  jupiterRising: Object.freeze([
    "rising", "setting", "sidereal longitude", "nakshatra", "nakshatra id", "nakshatra name", "year", "year position",
  ]),
  kumbhBySky: Object.freeze([
    "id", "site", "site name", "locale used", "river", "jupiter", "jupiter name", "sun", "sun name",
    "at new moon", "from", "to", "holds", "jupiter then", "jupiter longitude then",
  ]),
  kumbhYoga: Object.freeze([
    "id", "site", "site name", "locale used", "river", "jupiter", "jupiter name", "sun", "sun name",
    "at new moon", "source",
  ]),
  pushkaramRiver: Object.freeze([
    "id", "name", "locale used", "region", "sign", "sign name", "source",
  ]),
  pushkaramRule: Object.freeze(["id", "description"]),
  jupiterStation: Object.freeze(["moment", "kind", "sign", "sign name", "sidereal longitude"]),
  pushkaramBySky: Object.freeze([
    "id", "name", "locale used", "region", "sign", "sign name", "first", "last", "missing",
    "missing day", "depression", "depression arcseconds", "entry", "rule",
  ]),
  pushkaramsInYear: Object.freeze([
    "id", "name", "locale used", "region", "sign", "sign name", "first", "last", "missing",
    "missing day", "depression", "depression arcseconds", "entry", "rule",
  ]),
  folkDay: Object.freeze(["kind", "id", "name", "locale used", "count"]),
  nightWatch: Object.freeze(["watch", "points", "name", "locale used", "han name", "branch"]),
  barhaspatyaYear: Object.freeze(["position", "name", "expunged", "expunged name", "locale used"]),
  barhaspatyaYearAt: Object.freeze([
    "position", "name", "locale used", "twelve-year position", "twelve-year name", "mean sign",
    "mean sign name",
  ]),
  muhurtas: Object.freeze([
    "half", "number", "name", "start", "end", "mark", "missing", "missing day", "depression",
    "depression arcseconds",
  ]),
  amritaSiddhi: Object.freeze([
    "name", "devanagari", "nakshatra", "nakshatra id", "nakshatra name", "start", "end", "falls", "ayanamsa",
  ]),
  nakshatra: Object.freeze([
    "nakshatra", "nakshatra id", "nakshatra name", "entered", "leaves", "read at", "ayanamsa", "ayanamsa name",
  ]),
  planetaryHour: Object.freeze([
    "day", "hour", "ruler", "name", "locale used", "daytime", "start", "end", "missing",
    "missing day", "depression", "depression arcseconds",
  ]),
  gmat: Object.freeze(["fixed", "seconds of day", "attoseconds"]),
  irigDecode: Object.freeze([
    "fixed", "day of year", "hour", "minute", "second", "hundredths", "year", "control",
    "straight binary seconds",
  ]),
  irigFrameStart: Object.freeze(["seconds of day", "hundredths", "frame micros"]),
  irigFormats: Object.freeze([
    "format", "index count microseconds", "index counts", "frame microseconds", "fields",
    "control bits", "modulations", "carriers", "expressions",
  ]),  orbitRateOffset: Object.freeze(["id", "gm", "gm constant", "circular speed", "gravitational microseconds per day", "kinematic microseconds per day", "weak-field microseconds per day", "exact microseconds per day", "constants", "source"]),
  rocket: Object.freeze(["acceleration", "proper seconds", "coordinate seconds", "distance", "distance light years", "beta", "one minus beta", "lorentz factor", "constants", "source"]),
  flipAndBurn: Object.freeze(["acceleration", "distance", "proper seconds", "coordinate seconds", "proper years", "coordinate years", "peak beta", "one minus peak beta", "peak lorentz factor", "constants", "source"]),
  doppler: Object.freeze(["beta", "cos theta", "factor", "redshift", "head-on factor", "transverse factor", "source"]),
  velocityAdd: Object.freeze(["first beta", "second beta", "composed beta", "composed speed", "one minus composed beta", "first rapidity", "second rapidity", "composed rapidity", "composed lorentz factor", "source"]),
  schwarzschildRadius: Object.freeze(["id", "gm", "gm constant", "schwarzschild radius", "constants", "source"]),
  properTimeUncertain: Object.freeze(["beta", "beta std dev", "proper seconds", "proper std dev", "text", "constants", "source"]),
  planckUnits: Object.freeze(["symbol", "name", "unit", "value", "std dev", "figures", "relative uncertainty", "defined", "text", "source"]),
  bpConvert: Object.freeze(["from", "to", "years", "std dev", "converted", "converted std dev", "label", "source"]),
  deepConvert: Object.freeze(["from", "to", "value", "std dev", "converted", "converted std dev", "text", "seconds", "seconds std dev", "log10 seconds", "log10 std dev", "exact", "source"]),
  deepCompare: Object.freeze(["first seconds", "first std dev", "second seconds", "second std dev", "ratio", "ratio std dev", "log10 ratio", "log10 std dev", "decades", "overlap", "order", "source"]),
  dailyInsolation: Object.freeze(["years before 1950", "latitude", "solar longitude", "insolation", "solar constant", "source"]),
  edtfParse: Object.freeze(["role", "kind", "text", "precision", "qualifier", "long form", "first day", "last day", "support", "estimate", "span days"]),
  edtfRelations: Object.freeze(["relations", "symbols", "definitely before", "possibly before", "definitely after", "possibly after", "possibly concurrent"]),
  significant: Object.freeze(["value", "figures", "text", "rounded", "exponent", "last place"]),
  significantOp: Object.freeze(["operation", "text", "value", "figures", "exponent", "last place"]),
  uncertain: Object.freeze(["value", "std dev", "text", "significant", "relative", "low 1σ", "high 1σ", "low 2σ", "high 2σ", "low 3σ", "high 3σ"]),
  uncertainOp: Object.freeze(["operation", "value", "std dev", "text", "significant"]),
  interval: Object.freeze(["operation", "empty", "low seconds", "low attoseconds", "high seconds", "high attoseconds", "width seconds", "width attoseconds", "midpoint seconds", "midpoint attoseconds", "holds"]),
  units: Object.freeze(["id", "name", "symbol", "seconds numerator", "seconds denominator", "family", "authority"]),
  unitConvert: Object.freeze(["from", "to", "count numerator", "count denominator", "converted numerator", "converted denominator", "whole", "seconds numerator", "seconds denominator", "attosecond exact"]),
  rates: Object.freeze(["id", "kind", "hertz numerator", "hertz denominator"]),
  framePeriod: Object.freeze(["rate", "kind", "hertz numerator", "hertz denominator", "period numerator", "period denominator", "count numerator", "count denominator", "unit", "whole", "whole flicks"]),
  tempo: Object.freeze(["bpm numerator", "bpm denominator", "beat numerator", "beat denominator", "note fraction numerator", "note fraction denominator", "note numerator", "note denominator", "midi microseconds", "midi exact"]),
  fiscalProfiles: Object.freeze(["country", "country name", "table", "kind", "name", "local name", "authority", "national", "start calendar", "start month", "start day", "label convention", "valid from", "valid until", "approximate", "note", "sources checked", "sources", "read from", "unread"]),
  fiscalYearOn: Object.freeze(["country", "table", "kind", "name", "status", "label", "first", "last", "day of year", "days in year", "weekday", "month", "quarter", "half", "start calendar", "label convention", "approximate", "sources checked"]),
  fiscalYearSpan: Object.freeze(["country", "table", "kind", "name", "status", "label", "first", "last", "days"]),
  weekYearSystems: Object.freeze(["id", "name", "weekday", "month", "anchor rule", "label convention", "shape", "note", "source", "sources checked"]),
  weekYearOn: Object.freeze(["system", "name", "label", "first", "last", "weeks", "long", "week", "week first", "week last", "period", "period first", "period last", "quarter"]),
  nameDayLists: Object.freeze(["kind", "id", "country", "language", "name", "authority", "decided", "provenance", "valid from", "valid until", "licence", "leap day", "total names", "source", "retrieved", "reason", "explanation"]),
  nameDaysOn: Object.freeze(["kind", "id", "name", "authority", "valid from", "valid until", "licence", "names", "count", "unlisted names day", "notes", "source", "reason", "explanation"]),
  nameDay: Object.freeze(["kind", "id", "name", "authority", "valid from", "valid until", "licence", "dates", "fixed days", "count", "source", "reason", "explanation"]),
  attributionAuthorities: Object.freeze(["kind", "subject", "id", "name", "body", "region", "region name", "established", "revised", "valid from", "valid until", "provenance", "key kind", "source", "caveat", "reason", "explanation"]),
  attributions: Object.freeze(["subject", "id", "name", "key", "key kind", "attributions", "count", "gloss", "valid from", "valid until", "provenance", "caveat", "agreed"]),
  attributionsOn: Object.freeze(["subject", "id", "name", "key", "key kind", "attributions", "count", "gloss", "valid from", "valid until", "provenance", "caveat", "agreed"]),
  harvestMoon: Object.freeze(["year", "harvest moon", "hunters moon", "month", "september moon", "meridian", "source"]),
  structTime: Object.freeze(["year", "month", "day", "hour", "minute", "second", "weekday", "day of year", "dst"]),
  monthrange: Object.freeze(["first weekday", "days"]),
  monthcalendar: Object.freeze(["day 1", "day 2", "day 3", "day 4", "day 5", "day 6", "day 7"]),
  weekOfYear: Object.freeze(["week year", "week", "weeks in year", "week of month"]),
  solarNewYear: Object.freeze([
    "calendar", "year", "new year", "seconds", "chulasakarat year", "akyo", "akya", "last akyat", "atat",
  ]),
  pentadsInYear: Object.freeze(["index", "begins", "ends", "chinese", "japanese", "jokyo", "senmyo"]),
  moonCrossing: Object.freeze(["instant", "missing", "missing day"]),
  listForms: Object.freeze(["two", "start", "middle", "end", "locale used"]),
  regularizedUt1: Object.freeze(["minus UT1", "reading"]),
  signsInYear: Object.freeze(["sign", "id", "name", "start", "end", "begins", "ends"]),
  siderealSignsInYear: Object.freeze(["sign", "id", "name", "start", "end", "begins", "ends", "ayanamsa"]),
  principalPhase: Object.freeze(["phase", "instant", "day"]),
  yearType: Object.freeze([
    "calendar", "year", "kind", "days", "extra month", "thai name", "khmer name", "new year", "seconds",
  ]),
  mayaLongCount: Object.freeze([
    "id", "correlation", "long count", "baktun", "katun", "tun", "uinal", "kin", "tzolkin number", "tzolkin name",
    "haab day", "haab month",
  ]),
  akanDay: Object.freeze([
    "round", "day", "nnanson", "nnanson name", "nnawotwe", "nnawotwe name", "short name", "name", "dabone",
  ]),
  weton: Object.freeze([
    "round", "day", "dina", "dina name", "pasaran", "pasaran name", "dina neptu", "pasaran neptu", "neptu", "name",
  ]),
  buddhistLkYear: Object.freeze(["year", "gregorian year", "vesak", "began", "ends"]),
  solarNakshatraIngress: Object.freeze(["moment", "into", "into id", "into name", "from", "from id", "from name"]),
  japaneseEraYear: Object.freeze(["year"]),
  localeFormat: Object.freeze(["kind", "length", "pattern", "calendar type"]),
  pluralCategories: Object.freeze(["category", "example", "rules"]),
  epochs: Object.freeze(["id", "description", "tai seconds", "attoseconds", "source"]),
});

/**
 * The one line of `hc_gmtime` or `hc_localtime`: Python's `struct_time`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").StructTime}
 */
function structTime(cells) {
  const [year, month, day, hour, minute, second, weekday, dayOfYear, dst] = cells;
  return {
    year: integer(year, "year"),
    month: integer(month, "month"),
    day: integer(day, "day"),
    hour: integer(hour, "hour"),
    minute: integer(minute, "minute"),
    second: integer(second, "second"),
    weekday: integer(weekday, "weekday"),
    dayOfYear: integer(dayOfYear, "day of year"),
    dst: integer(dst, "dst"),
  };
}

/**
 * The one line of `hc_solar_new_year`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").SolarNewYear}
 */
function solarNewYear(cells) {
  const [calendar, year, newYear, seconds, chulasakaratYear, akyo, akya, lastAkyat, atat] = cells;
  return {
    calendar: /** @type {import("./hyper-calendar.d.ts").SolarNewYearCalendar} */ (calendar),
    year: integer(year, "year"),
    newYear: integer(newYear, "new year"),
    seconds: optionalInteger(seconds, "seconds"),
    chulasakaratYear: optionalInteger(chulasakaratYear, "chulasakarat year"),
    akyo: optionalInteger(akyo, "akyo"),
    akya: optionalInteger(akya, "akya"),
    lastAkyat: optionalInteger(lastAkyat, "last akyat"),
    atat: optionalInteger(atat, "atat"),
  };
}

/**
 * The one line of `hc_moonrise` or `hc_moonset`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").MoonCrossing}
 */
function moonCrossing(cells) {
  const [instant, missing, missingDay] = cells;
  return {
    instant: optionalInteger(instant, "instant"),
    missing: missing === ""
      ? null
      : {
        event: /** @type {import("./hyper-calendar.d.ts").MissingMoonEventName} */ (missing),
        day: integer(missingDay, "missing day"),
      },
  };
}

/**
 * One line of `hc_tropical_signs_in_year` or `hc_sidereal_signs_in_year`,
 * without the ayanāṃśa.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").SignPeriod<string>}
 */
function signPeriod(cells) {
  const [sign, id, name, start, end, begins, ends] = cells;
  return {
    sign: integer(sign, "sign"),
    id,
    name,
    start: integer(start, "start"),
    end: integer(end, "end"),
    begins: integer(begins, "begins"),
    ends: integer(ends, "ends"),
  };
}

/**
 * One line of `hc_orbit_rate_offset`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").OrbitRateOffset}
 */
function orbitRateOffset(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9] = cells;
  return {
    id: c0,
    gm: decimal(c1, "gm"),
    gmConstant: c2,
    circularSpeed: decimal(c3, "circular speed"),
    gravitationalMicrosecondsPerDay: decimal(c4, "gravitational microseconds per day"),
    kinematicMicrosecondsPerDay: decimal(c5, "kinematic microseconds per day"),
    weakFieldMicrosecondsPerDay: decimal(c6, "weak-field microseconds per day"),
    exactMicrosecondsPerDay: decimal(c7, "exact microseconds per day"),
    constants: list(c8),
    source: c9,
  };
}

/**
 * One line of `hc_rocket`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").RocketBurn}
 */
function rocket(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9] = cells;
  return {
    acceleration: decimal(c0, "acceleration"),
    properSeconds: decimal(c1, "proper seconds"),
    coordinateSeconds: decimal(c2, "coordinate seconds"),
    distance: decimal(c3, "distance"),
    distanceLightYears: decimal(c4, "distance light years"),
    beta: decimal(c5, "beta"),
    oneMinusBeta: decimal(c6, "one minus beta"),
    lorentzFactor: decimal(c7, "lorentz factor"),
    constants: list(c8),
    source: c9,
  };
}

/**
 * One line of `hc_flip_and_burn`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").FlipAndBurn}
 */
function flipAndBurn(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10] = cells;
  return {
    acceleration: decimal(c0, "acceleration"),
    distance: decimal(c1, "distance"),
    properSeconds: decimal(c2, "proper seconds"),
    coordinateSeconds: decimal(c3, "coordinate seconds"),
    properYears: decimal(c4, "proper years"),
    coordinateYears: decimal(c5, "coordinate years"),
    peakBeta: decimal(c6, "peak beta"),
    oneMinusPeakBeta: decimal(c7, "one minus peak beta"),
    peakLorentzFactor: decimal(c8, "peak lorentz factor"),
    constants: list(c9),
    source: c10,
  };
}

/**
 * One line of `hc_doppler`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").DopplerShift}
 */
function doppler(cells) {
  const [c0, c1, c2, c3, c4, c5, c6] = cells;
  return {
    beta: decimal(c0, "beta"),
    cosTheta: decimal(c1, "cos theta"),
    factor: decimal(c2, "factor"),
    redshift: decimal(c3, "redshift"),
    headOnFactor: decimal(c4, "head-on factor"),
    transverseFactor: decimal(c5, "transverse factor"),
    source: c6,
  };
}

/**
 * One line of `hc_velocity_add`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").VelocityComposition}
 */
function velocityAdd(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9] = cells;
  return {
    firstBeta: decimal(c0, "first beta"),
    secondBeta: decimal(c1, "second beta"),
    composedBeta: decimal(c2, "composed beta"),
    composedSpeed: decimal(c3, "composed speed"),
    oneMinusComposedBeta: decimal(c4, "one minus composed beta"),
    firstRapidity: decimal(c5, "first rapidity"),
    secondRapidity: decimal(c6, "second rapidity"),
    composedRapidity: decimal(c7, "composed rapidity"),
    composedLorentzFactor: decimal(c8, "composed lorentz factor"),
    source: c9,
  };
}

/**
 * One line of `hc_schwarzschild_radius`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").SchwarzschildRadius}
 */
function schwarzschildRadius(cells) {
  const [c0, c1, c2, c3, c4, c5] = cells;
  return {
    id: c0,
    gm: decimal(c1, "gm"),
    gmConstant: c2,
    schwarzschildRadius: decimal(c3, "schwarzschild radius"),
    constants: list(c4),
    source: c5,
  };
}

/**
 * One line of `hc_proper_time_uncertain`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").UncertainProperTime}
 */
function properTimeUncertain(cells) {
  const [c0, c1, c2, c3, c4, c5, c6] = cells;
  return {
    beta: decimal(c0, "beta"),
    betaStdDev: decimal(c1, "beta std dev"),
    properSeconds: decimal(c2, "proper seconds"),
    properStdDev: decimal(c3, "proper std dev"),
    text: c4,
    constants: list(c5),
    source: c6,
  };
}

/**
 * One line of `hc_planck_units`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").PlanckUnit}
 */
function planckUnits(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9] = cells;
  return {
    symbol: c0,
    name: c1,
    unit: c2,
    value: decimal(c3, "value"),
    stdDev: decimal(c4, "std dev"),
    figures: integer(c5, "figures"),
    relativeUncertainty: decimal(c6, "relative uncertainty"),
    defined: flag(c7, "defined"),
    text: c8,
    source: c9,
  };
}

/**
 * One line of `hc_bp_convert`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").DatumConversion}
 */
function bpConvert(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7] = cells;
  return {
    from: c0,
    to: c1,
    years: decimal(c2, "years"),
    stdDev: decimal(c3, "std dev"),
    converted: decimal(c4, "converted"),
    convertedStdDev: decimal(c5, "converted std dev"),
    label: c6,
    source: c7,
  };
}

/**
 * One line of `hc_deep_convert`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").MagnitudeConversion}
 */
function deepConvert(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12] = cells;
  return {
    from: c0,
    to: c1,
    value: decimal(c2, "value"),
    stdDev: decimal(c3, "std dev"),
    converted: decimal(c4, "converted"),
    convertedStdDev: decimal(c5, "converted std dev"),
    text: c6,
    seconds: decimal(c7, "seconds"),
    secondsStdDev: decimal(c8, "seconds std dev"),
    log10Seconds: c9 === "" ? null : decimal(c9, "log10 seconds"),
    log10StdDev: c10 === "" ? null : decimal(c10, "log10 std dev"),
    exact: flag(c11, "exact"),
    source: c12,
  };
}

/**
 * One line of `hc_deep_compare`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").MagnitudeComparison}
 */
function deepCompare(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11] = cells;
  return {
    firstSeconds: decimal(c0, "first seconds"),
    firstStdDev: decimal(c1, "first std dev"),
    secondSeconds: decimal(c2, "second seconds"),
    secondStdDev: decimal(c3, "second std dev"),
    ratio: decimal(c4, "ratio"),
    ratioStdDev: decimal(c5, "ratio std dev"),
    log10Ratio: decimal(c6, "log10 ratio"),
    log10StdDev: decimal(c7, "log10 std dev"),
    decades: decimal(c8, "decades"),
    overlap: flag(c9, "overlap"),
    order: integer(c10, "order"),
    source: c11,
  };
}

/**
 * One line of `hc_daily_insolation`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").DailyInsolation}
 */
function dailyInsolation(cells) {
  const [c0, c1, c2, c3, c4, c5] = cells;
  return {
    yearsBefore1950: decimal(c0, "years before 1950"),
    latitude: decimal(c1, "latitude"),
    solarLongitude: decimal(c2, "solar longitude"),
    insolation: decimal(c3, "insolation"),
    solarConstant: decimal(c4, "solar constant"),
    source: c5,
  };
}

/**
 * One line of `hc_edtf_parse`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").EdtfPart}
 */
function edtfParse(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10] = cells;
  return {
    role: c0,
    kind: c1,
    text: c2,
    precision: optional(c3),
    qualifier: optional(c4),
    longForm: c5 === "" ? null : flag(c5, "long form"),
    firstDay: c6 === "" ? null : bigInteger(c6, "first day"),
    lastDay: c7 === "" ? null : bigInteger(c7, "last day"),
    support: optional(c8),
    estimate: c9 === "" ? null : bigInteger(c9, "estimate"),
    spanDays: c10 === "" ? null : bigInteger(c10, "span days"),
  };
}

/**
 * One line of `hc_edtf_relations`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").EdtfRelations}
 */
function edtfRelations(cells) {
  const [c0, c1, c2, c3, c4, c5, c6] = cells;
  return {
    relations: list(c0),
    symbols: list(c1),
    definitelyBefore: flag(c2, "definitely before"),
    possiblyBefore: flag(c3, "possibly before"),
    definitelyAfter: flag(c4, "definitely after"),
    possiblyAfter: flag(c5, "possibly after"),
    possiblyConcurrent: flag(c6, "possibly concurrent"),
  };
}

/**
 * One line of `hc_significant`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").SignificantNumber}
 */
function significant(cells) {
  const [c0, c1, c2, c3, c4, c5] = cells;
  return {
    value: decimal(c0, "value"),
    figures: integer(c1, "figures"),
    text: c2,
    rounded: decimal(c3, "rounded"),
    exponent: integer(c4, "exponent"),
    lastPlace: integer(c5, "last place"),
  };
}

/**
 * One line of `hc_significant_op`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").SignificantResult}
 */
function significantOp(cells) {
  const [c0, c1, c2, c3, c4, c5] = cells;
  return {
    operation: c0,
    text: c1,
    value: decimal(c2, "value"),
    figures: integer(c3, "figures"),
    exponent: integer(c4, "exponent"),
    lastPlace: integer(c5, "last place"),
  };
}

/**
 * One line of `hc_uncertain`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").UncertainQuantity}
 */
function uncertain(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10] = cells;
  return {
    value: decimal(c0, "value"),
    stdDev: decimal(c1, "std dev"),
    text: c2,
    significant: optional(c3),
    relative: c4 === "" ? null : decimal(c4, "relative"),
    low1σ: decimal(c5, "low 1σ"),
    high1σ: decimal(c6, "high 1σ"),
    low2σ: decimal(c7, "low 2σ"),
    high2σ: decimal(c8, "high 2σ"),
    low3σ: decimal(c9, "low 3σ"),
    high3σ: decimal(c10, "high 3σ"),
  };
}

/**
 * One line of `hc_uncertain_op`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").UncertainResult}
 */
function uncertainOp(cells) {
  const [c0, c1, c2, c3, c4] = cells;
  return {
    operation: c0,
    value: decimal(c1, "value"),
    stdDev: c2 === "" ? null : decimal(c2, "std dev"),
    text: c3,
    significant: optional(c4),
  };
}

/**
 * One line of `hc_interval`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").IntervalResult}
 */
function interval(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10] = cells;
  return {
    operation: c0,
    empty: c1 === "" ? null : flag(c1, "empty"),
    lowSeconds: c2 === "" ? null : bigInteger(c2, "low seconds"),
    lowAttoseconds: c3 === "" ? null : bigInteger(c3, "low attoseconds"),
    highSeconds: c4 === "" ? null : bigInteger(c4, "high seconds"),
    highAttoseconds: c5 === "" ? null : bigInteger(c5, "high attoseconds"),
    widthSeconds: c6 === "" ? null : bigInteger(c6, "width seconds"),
    widthAttoseconds: c7 === "" ? null : bigInteger(c7, "width attoseconds"),
    midpointSeconds: c8 === "" ? null : bigInteger(c8, "midpoint seconds"),
    midpointAttoseconds: c9 === "" ? null : bigInteger(c9, "midpoint attoseconds"),
    holds: c10 === "" ? null : flag(c10, "holds"),
  };
}

/**
 * One line of `hc_units`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").TimeUnit}
 */
function units(cells) {
  const [c0, c1, c2, c3, c4, c5, c6] = cells;
  return {
    id: c0,
    name: c1,
    symbol: optional(c2),
    secondsNumerator: bigInteger(c3, "seconds numerator"),
    secondsDenominator: bigInteger(c4, "seconds denominator"),
    family: c5,
    authority: c6,
  };
}

/**
 * One line of `hc_unit_convert`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").UnitConversion}
 */
function unitConvert(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9] = cells;
  return {
    from: c0,
    to: c1,
    countNumerator: bigInteger(c2, "count numerator"),
    countDenominator: bigInteger(c3, "count denominator"),
    convertedNumerator: bigInteger(c4, "converted numerator"),
    convertedDenominator: bigInteger(c5, "converted denominator"),
    whole: flag(c6, "whole"),
    secondsNumerator: bigInteger(c7, "seconds numerator"),
    secondsDenominator: bigInteger(c8, "seconds denominator"),
    attosecondExact: flag(c9, "attosecond exact"),
  };
}

/**
 * One line of `hc_rates`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").FrameRate}
 */
function rates(cells) {
  const [c0, c1, c2, c3] = cells;
  return {
    id: c0,
    kind: c1,
    hertzNumerator: bigInteger(c2, "hertz numerator"),
    hertzDenominator: bigInteger(c3, "hertz denominator"),
  };
}

/**
 * One line of `hc_frame_period`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").FramePeriod}
 */
function framePeriod(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10] = cells;
  return {
    rate: c0,
    kind: c1,
    hertzNumerator: bigInteger(c2, "hertz numerator"),
    hertzDenominator: bigInteger(c3, "hertz denominator"),
    periodNumerator: bigInteger(c4, "period numerator"),
    periodDenominator: bigInteger(c5, "period denominator"),
    countNumerator: bigInteger(c6, "count numerator"),
    countDenominator: bigInteger(c7, "count denominator"),
    unit: c8,
    whole: flag(c9, "whole"),
    wholeFlicks: flag(c10, "whole flicks"),
  };
}

/**
 * One line of `hc_tempo`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").NoteAtTempo}
 */
function tempo(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9] = cells;
  return {
    bpmNumerator: bigInteger(c0, "bpm numerator"),
    bpmDenominator: bigInteger(c1, "bpm denominator"),
    beatNumerator: bigInteger(c2, "beat numerator"),
    beatDenominator: bigInteger(c3, "beat denominator"),
    noteFractionNumerator: bigInteger(c4, "note fraction numerator"),
    noteFractionDenominator: bigInteger(c5, "note fraction denominator"),
    noteNumerator: bigInteger(c6, "note numerator"),
    noteDenominator: bigInteger(c7, "note denominator"),
    midiMicroseconds: optionalInteger(c8, "midi microseconds"),
    midiExact: c9 === "" ? null : flag(c9, "midi exact"),
  };
}

/**
 * One line of `hc_fiscal_profiles`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").FiscalSystem}
 */
function fiscalProfiles(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19] = cells;
  return {
    country: c0,
    countryName: c1,
    table: c2,
    kind: c3,
    name: c4,
    localName: optional(c5),
    authority: c6,
    national: flag(c7, "national"),
    startCalendar: c8,
    startMonth: integer(c9, "start month"),
    startDay: integer(c10, "start day"),
    labelConvention: c11,
    validFrom: optionalInteger(c12, "valid from"),
    validUntil: optionalInteger(c13, "valid until"),
    approximate: flag(c14, "approximate"),
    note: c15,
    sourcesChecked: c16,
    sources: c17,
    readFrom: integer(c18, "read from"),
    unread: unreadSpans(c19),
  };
}

/**
 * The `first-last` spans of the `unread` cell of `hc_fiscal_profiles`.
 *
 * @param {string} cell
 * @returns {Array<{first: number, last: number}>}
 */
function unreadSpans(cell) {
  if (cell === "") return [];
  return cell.split(";").map((span) => {
    const [first, last] = span.split("-");
    return { first: integer(first, "unread"), last: integer(last, "unread") };
  });
}

/**
 * One line of `hc_fiscal_year_on`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").FiscalYearOfDay}
 */
function fiscalYearOn(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17] = cells;
  return {
    country: c0,
    table: c1,
    kind: c2,
    name: c3,
    status: c4,
    label: optionalInteger(c5, "label"),
    first: optionalInteger(c6, "first"),
    last: optionalInteger(c7, "last"),
    dayOfYear: optionalInteger(c8, "day of year"),
    daysInYear: optionalInteger(c9, "days in year"),
    weekday: optionalInteger(c10, "weekday"),
    month: optionalInteger(c11, "month"),
    quarter: optionalInteger(c12, "quarter"),
    half: optionalInteger(c13, "half"),
    startCalendar: c14,
    labelConvention: c15,
    approximate: flag(c16, "approximate"),
    sourcesChecked: c17,
  };
}

/**
 * One line of `hc_fiscal_year_span`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").FiscalYearSpan}
 */
function fiscalYearSpan(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8] = cells;
  return {
    country: c0,
    table: c1,
    kind: c2,
    name: c3,
    status: c4,
    label: integer(c5, "label"),
    first: optionalInteger(c6, "first"),
    last: optionalInteger(c7, "last"),
    days: optionalInteger(c8, "days"),
  };
}

/**
 * One line of `hc_week_year_systems`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").WeekYearSystem}
 */
function weekYearSystems(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9] = cells;
  return {
    id: c0,
    name: c1,
    weekday: integer(c2, "weekday"),
    month: integer(c3, "month"),
    anchorRule: c4,
    labelConvention: c5,
    shape: optional(c6),
    note: c7,
    source: c8,
    sourcesChecked: c9,
  };
}

/**
 * One line of `hc_week_year_on`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").WeekYearOfDay}
 */
function weekYearOn(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13] = cells;
  return {
    system: c0,
    name: c1,
    label: integer(c2, "label"),
    first: integer(c3, "first"),
    last: integer(c4, "last"),
    weeks: integer(c5, "weeks"),
    long: flag(c6, "long"),
    week: integer(c7, "week"),
    weekFirst: integer(c8, "week first"),
    weekLast: integer(c9, "week last"),
    period: optionalInteger(c10, "period"),
    periodFirst: optionalInteger(c11, "period first"),
    periodLast: optionalInteger(c12, "period last"),
    quarter: optionalInteger(c13, "quarter"),
  };
}

/**
 * One line of `hc_name_day_lists`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").NameDayList}
 */
function nameDayLists(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16] = cells;
  return {
    kind: c0,
    id: c1,
    country: c2,
    language: optional(c3),
    name: c4,
    authority: optional(c5),
    decided: optional(c6),
    provenance: optional(c7),
    validFrom: optionalInteger(c8, "valid from"),
    validUntil: optionalInteger(c9, "valid until"),
    licence: optional(c10),
    leapDay: optional(c11),
    totalNames: optionalInteger(c12, "total names"),
    source: c13,
    retrieved: c14,
    reason: optional(c15),
    explanation: optional(c16),
  };
}

/**
 * One line of `hc_name_days_on`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").NameDaysOfDay}
 */
function nameDaysOn(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13] = cells;
  return {
    kind: c0,
    id: c1,
    name: c2,
    authority: optional(c3),
    validFrom: optionalInteger(c4, "valid from"),
    validUntil: optionalInteger(c5, "valid until"),
    licence: optional(c6),
    names: list(c7),
    count: optionalInteger(c8, "count"),
    unlistedNamesDay: c9 === "" ? null : flag(c9, "unlisted names day"),
    notes: optional(c10),
    source: c11,
    reason: optional(c12),
    explanation: optional(c13),
  };
}

/**
 * One line of `hc_name_day`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").NameDayDates}
 */
function nameDay(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12] = cells;
  return {
    kind: c0,
    id: c1,
    name: c2,
    authority: optional(c3),
    validFrom: optionalInteger(c4, "valid from"),
    validUntil: optionalInteger(c5, "valid until"),
    licence: optional(c6),
    dates: list(c7),
    fixedDays: list(c8).map((value) => integer(value, "fixed days")),
    count: optionalInteger(c9, "count"),
    source: c10,
    reason: optional(c11),
    explanation: optional(c12),
  };
}

/**
 * One line of `hc_attribution_authorities`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").AttributionAuthority}
 */
function attributionAuthorities(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16] = cells;
  return {
    kind: c0,
    subject: optional(c1),
    id: c2,
    name: c3,
    body: optional(c4),
    region: optional(c5),
    regionName: optional(c6),
    established: optional(c7),
    revised: optional(c8),
    validFrom: optionalInteger(c9, "valid from"),
    validUntil: optionalInteger(c10, "valid until"),
    provenance: optional(c11),
    keyKind: optional(c12),
    source: c13,
    caveat: optional(c14),
    reason: optional(c15),
    explanation: optional(c16),
  };
}

/**
 * One line of `hc_attributions`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").Attribution}
 */
function attributions(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12] = cells;
  return {
    subject: c0,
    id: c1,
    name: c2,
    key: integer(c3, "key"),
    keyKind: c4,
    attributions: list(c5),
    count: integer(c6, "count"),
    gloss: optional(c7),
    validFrom: optionalInteger(c8, "valid from"),
    validUntil: optionalInteger(c9, "valid until"),
    provenance: c10,
    caveat: optional(c11),
    agreed: flag(c12, "agreed"),
  };
}

/**
 * One line of `hc_attributions_on`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").AttributionOfDay}
 */
function attributionsOn(cells) {
  const [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12] = cells;
  return {
    subject: c0,
    id: c1,
    name: c2,
    key: integer(c3, "key"),
    keyKind: c4,
    attributions: list(c5),
    count: integer(c6, "count"),
    gloss: optional(c7),
    validFrom: optionalInteger(c8, "valid from"),
    validUntil: optionalInteger(c9, "valid until"),
    provenance: c10,
    caveat: optional(c11),
    agreed: flag(c12, "agreed"),
  };
}

/**
 * One line of `hc_harvest_moon`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").HarvestMoon}
 */
function harvestMoon(cells) {
  const [c0, c1, c2, c3, c4, c5, c6] = cells;
  return {
    year: integer(c0, "year"),
    harvestMoon: integer(c1, "harvest moon"),
    huntersMoon: integer(c2, "hunters moon"),
    month: integer(c3, "month"),
    septemberMoon: c4,
    meridian: c5,
    source: c6,
  };
}

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
  const [tag, englishName, nativeName, gregorianMonths, weekdays, gregorianEras, calendars, parent, numbering, direction] = cells;
  return {
    tag,
    englishName,
    nativeName,
    gregorianMonths: flag(gregorianMonths, "gregorian months"),
    weekdays: flag(weekdays, "weekdays"),
    gregorianEras: flag(gregorianEras, "gregorian eras"),
    calendars: list(calendars),
    parent: optional(parent),
    numbering,
    direction: /** @type {import("./hyper-calendar.d.ts").TextDirection} */ (direction),
  };
}

/**
 * The eight cells of a reading.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").Reading}
 */
function reading(cells) {
  const [localDay, localSecond, attoseconds, zone, offset, unix, leap, endOfDay] = cells;
  return {
    localDay: integer(localDay, "local day"),
    localSecond: integer(localSecond, "local second"),
    attoseconds: bigInteger(attoseconds, "attoseconds"),
    zone: /** @type {import("./hyper-calendar.d.ts").ReadingZone} */ (zone),
    offsetSeconds: optionalInteger(offset, "offset"),
    unixSeconds: optionalInteger(unix, "unix seconds"),
    leapSecond: flag(leap, "leap second"),
    endOfDay: flag(endOfDay, "end of day"),
  };
}

/**
 * The line of `hc_iso_duration`, and `hc_format_iso_duration`'s from its
 * second cell on.
 *
 * @param {string[]} cells text, nominal, exact seconds, exact attoseconds
 */
function exactLength(cells) {
  const [text, nominal, seconds, attoseconds] = cells;
  return {
    text,
    nominal: flag(nominal, "nominal"),
    exactSeconds: seconds === "" ? null : bigInteger(seconds, "exact seconds"),
    exactAttoseconds: attoseconds === "" ? null : bigInteger(attoseconds, "exact attoseconds"),
  };
}

/**
 * The line of `hc_iso_duration`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").IsoDurationParts}
 */
function isoDurationParts(cells) {
  const [negative, years, months, weeks, days, hours, minutes, seconds, fraction, form, ...rest] = cells;
  const parts = [years, months, weeks, days, hours, minutes, seconds].map((cell, index) =>
    optionalInteger(cell, ["years", "months", "weeks", "days", "hours", "minutes", "seconds"][index]));
  return {
    negative: flag(negative, "negative"),
    years: parts[0], months: parts[1], weeks: parts[2], days: parts[3],
    hours: parts[4], minutes: parts[5], seconds: parts[6],
    fraction: optional(fraction),
    form: /** @type {"designators" | "alternative"} */ (form),
    ...exactLength(rest),
  };
}

/**
 * The line of `hc_iso_interval`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").IsoIntervalParts}
 */
function isoIntervalParts(cells) {
  const [repetitions, shape, start, startUnix, end, endUnix, ...rest] = cells;
  return {
    repetitions: repetitions === "" ? null : repetitions === "inf" ? "inf" : integer(repetitions, "repetitions"),
    shape: /** @type {import("./hyper-calendar.d.ts").IntervalShape} */ (shape),
    start: optional(start),
    startUnixSeconds: optionalInteger(startUnix, "start unix seconds"),
    end: optional(end),
    endUnixSeconds: optionalInteger(endUnix, "end unix seconds"),
    duration: rest[0] === "" ? null : exactLength(rest),
  };
}

/**
 * The line of `hc_iso_date_parts`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").IsoDateParts}
 */
function isoDateParts(cells) {
  const [form, year, month, day, dayOfYear, week, weekday, fixed, style] = cells;
  return {
    form: /** @type {import("./hyper-calendar.d.ts").IsoDateForm} */ (form),
    year: integer(year, "year"),
    month: optionalInteger(month, "month"),
    day: optionalInteger(day, "day"),
    dayOfYear: optionalInteger(dayOfYear, "day of year"),
    week: optionalInteger(week, "week"),
    weekday: optionalInteger(weekday, "weekday"),
    fixed: optionalInteger(fixed, "fixed"),
    style: /** @type {import("./hyper-calendar.d.ts").IsoDateStyle} */ (style),
  };
}

/**
 * The line of `hc_parse_pattern`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").PatternFields}
 */
function patternFields(cells) {
  const [
    year, century, yearOfCentury, month, day, dayOfYear, isoYear, isoWeek, isoWeekday, weekSunday, weekMonday,
    hour, hour12, dayPeriod, minute, second, attoseconds, zone, offset, unixSeconds, era, fixed,
    ...resolved
  ] = cells;
  const int = (/** @type {string} */ cell, /** @type {string} */ what) => optionalInteger(cell, what);
  return {
    year: int(year, "year"), century: int(century, "century"), yearOfCentury: int(yearOfCentury, "year of century"),
    month: int(month, "month"), day: int(day, "day"), dayOfYear: int(dayOfYear, "day of year"),
    isoYear: int(isoYear, "iso year"), isoWeek: int(isoWeek, "iso week"), isoWeekday: int(isoWeekday, "iso weekday"),
    weekSunday: int(weekSunday, "week sunday"), weekMonday: int(weekMonday, "week monday"),
    hour: int(hour, "hour"), hour12: int(hour12, "hour 12"),
    dayPeriod: /** @type {"am" | "pm" | null} */ (optional(dayPeriod)),
    minute: int(minute, "minute"), second: int(second, "second"),
    attoseconds: attoseconds === "" ? null : bigInteger(attoseconds, "attoseconds"),
    zone: /** @type {import("./hyper-calendar.d.ts").ReadingZone | null} */ (optional(zone)),
    offsetSeconds: int(offset, "offset"), unixSeconds: int(unixSeconds, "unix seconds"),
    era: /** @type {"ce" | "bce" | null} */ (optional(era)), fixed: int(fixed, "fixed"),
    reading: resolved[0] === "" ? null : reading(resolved),
  };
}

/**
 * One step of `hc_locale_chain`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").LocaleChainStep}
 */
function localeChainStep(cells) {
  const [step, tag, rule, carried] = cells;
  return {
    step: integer(step, "step"),
    tag,
    rule: /** @type {import("./hyper-calendar.d.ts").LocaleChainRule} */ (rule),
    carried: flag(carried, "carried"),
  };
}

/**
 * The line of `hc_locale_info`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").LocaleInfo}
 */
function localeInfo(cells) {
  const [
    tag, language, script, region, variant, calendarKey, numberingKey, firstDayKey, hourCycleKey,
    localeUsed, parent, parentRule, numbering, firstDay, minDays, direction, casing, capitalises, plural,
  ] = cells;
  return {
    tag,
    language,
    script: optional(script),
    region: optional(region),
    variant: optional(variant),
    calendarKey: optional(calendarKey),
    numberingKey: optional(numberingKey),
    firstDayKey: optionalInteger(firstDayKey, "first day key"),
    hourCycleKey: optional(hourCycleKey),
    localeUsed,
    parent: optional(parent),
    parentRule: /** @type {import("./hyper-calendar.d.ts").LocaleChainRule | null} */ (optional(parentRule)),
    numbering,
    firstDay: integer(firstDay, "first day"),
    minDays: integer(minDays, "min days"),
    direction: /** @type {import("./hyper-calendar.d.ts").TextDirection} */ (direction),
    casing: /** @type {import("./hyper-calendar.d.ts").CasingStyleName} */ (casing),
    capitalisesMonthNames: flag(capitalises, "capitalises month names"),
    pluralRules: plural,
  };
}

/**
 * The line of `hc_plural_category`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").PluralCategoryAnswer}
 */
function pluralCategoryAnswer(cells) {
  const [category, rules, i, v, w, f, t] = cells;
  return {
    category: /** @type {import("./hyper-calendar.d.ts").PluralCategoryName} */ (category),
    rules,
    operands: {
      i: bigInteger(i, "i"),
      v: integer(v, "v"),
      w: integer(w, "w"),
      f: bigInteger(f, "f"),
      t: bigInteger(t, "t"),
    },
  };
}

/**
 * One line of `hc_names`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").LocaleName}
 */
function localeName(cells) {
  const [kind, position, name, localeUsed] = cells;
  return { kind, position: integer(position, "position"), name, localeUsed };
}

/**
 * The line of `hc_case`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").CasedText}
 */
function casedText(cells) {
  const [text, mode, casing, localeUsed] = cells;
  return {
    text,
    mode: /** @type {import("./hyper-calendar.d.ts").CaseMode} */ (mode),
    casing: /** @type {import("./hyper-calendar.d.ts").CasingStyleName} */ (casing),
    localeUsed,
  };
}

/**
 * The line of `hc_isolate`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").IsolatedText}
 */
function isolatedText(cells) {
  const [text, direction, textDirection, isolated, mode] = cells;
  return {
    text,
    direction: /** @type {import("./hyper-calendar.d.ts").TextDirection} */ (direction),
    textDirection: /** @type {import("./hyper-calendar.d.ts").TextDirection | null} */ (optional(textDirection)),
    isolated: flag(isolated, "isolated"),
    mode: /** @type {import("./hyper-calendar.d.ts").IsolateMode} */ (mode),
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
  const [date, name, localName, kind, confidence, substitute, observedFor, region, group, id, source, bridged, windowFirst, windowLast] =
    cells;
  return {
    date: optional(date),
    name,
    localName: optional(localName),
    kind: /** @type {import("./hyper-calendar.d.ts").HolidayKind | "gap"} */ (kind),
    confidence: /** @type {import("./hyper-calendar.d.ts").Confidence | null} */ (optional(confidence)),
    substitute: flag(substitute, "substitute"),
    observedFor: optional(observedFor),
    region: optional(region),
    group: optional(group),
    id,
    source: optional(source),
    bridged: flag(bridged, "bridged"),
    windowFirst: optional(windowFirst),
    windowLast: optional(windowLast),
  };
}

/**
 * One row of `hc_holidays_on`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").HolidayOn}
 */
function holidayOn(cells) {
  const [table, tableName, name, localName, kind, confidence, source, substitute, observedFor, region, group, id, bridged] =
    cells;
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
    region: optional(region),
    group: optional(group),
    id,
    bridged: flag(bridged, "bridged"),
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
 * One line of `hc_pentad_traditions`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").PentadTradition}
 */
function pentadTradition(cells) {
  const [id, englishName, authority, alternates] = cells;
  return {
    id: /** @type {import("./hyper-calendar.d.ts").PentadTraditionId} */ (id),
    englishName,
    authority,
    alternates: integer(alternates, "alternates"),
  };
}

/**
 * The one line of `hc_pentad_in_tradition`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").PentadInTradition}
 */
function pentadInTradition(cells) {
  const [index, name, gloss, alternate, begins, ends, tradition, authority] = cells;
  return {
    index: integer(index, "index"),
    name,
    gloss,
    alternate: optional(alternate),
    begins: integer(begins, "begins"),
    ends: integer(ends, "ends"),
    tradition: /** @type {import("./hyper-calendar.d.ts").PentadTraditionId} */ (tradition),
    authority,
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
    ayanamsa: /** @type {import("./hyper-calendar.d.ts").Ayanamsa | "surya-siddhanta" | null} */ (optional(ayanamsa)),
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
  const [augury, atStart, atEnd, names, scripts, regions] = cells;
  const scriptList = scripts.split(";");
  const regionList = regions.split(";");
  return {
    augury: /** @type {import("./hyper-calendar.d.ts").MarriageAuguryName} */ (augury),
    lichunAtStart: flag(atStart, "lichun at start"),
    lichunAtEnd: flag(atEnd, "lichun at end"),
    chineseNames: names === ""
      ? []
      : names.split(";").map((name, index) => ({
        name,
        locale: /** @type {"zh-Hant" | "zh-Hans"} */ (scriptList[index]),
        region: optional(regionList[index] ?? ""),
      })),
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
 * The one line of `hc_tai_minus_utc_exact`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").TaiMinusUtc}
 */
function taiMinusUtc(cells) {
  const [seconds, attoseconds] = cells;
  return { seconds: bigInteger(seconds, "seconds"), attoseconds: bigInteger(attoseconds, "attoseconds") };
}

/**
 * The one line of `hc_utc_from_tai_exact`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").UtcInstantLabel}
 */
function utcInstantLabel(cells) {
  const [unixSeconds, attoseconds, leapSecond] = cells;
  return {
    unixSeconds: bigInteger(unixSeconds, "unix seconds"),
    attoseconds: bigInteger(attoseconds, "attoseconds"),
    leapSecond: flag(leapSecond, "leap second"),
  };
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
 * One pair of column 12 of `hc_holiday_tables`: a subdivision and a group, `CN-XJ:women`.
 *
 * @param {string} pair
 * @returns {import("./hyper-calendar.d.ts").HolidayRegionGroup}
 */
function regionGroup(pair) {
  const at = pair.lastIndexOf(":");
  return { region: pair.slice(0, at), group: pair.slice(at + 1) };
}

/**
 * One line of `hc_holiday_tables`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").HolidayTable}
 */
function holidayTable(cells) {
  const [
    code, kind, name, englishName, localeUsed, source, country, shortName, regions, groups, groupNames,
    regionGroups, readSubdivisions, weekend, substitution,
  ] = cells;
  return {
    code,
    kind: /** @type {import("./hyper-calendar.d.ts").HolidayTableKind} */ (kind),
    name: optional(name),
    englishName,
    localeUsed: optional(localeUsed),
    source: optional(source),
    country: optional(country),
    shortName: optional(shortName),
    regions: regions === "" ? [] : regions.split(";"),
    groups: groups === "" ? [] : groups.split(";"),
    groupNames: groupNames === "" ? [] : groupNames.split(";"),
    regionGroups: regionGroups === "" ? [] : regionGroups.split(";").map(regionGroup),
    readSubdivisions: readSubdivisions === "" ? [] : readSubdivisions.split(";"),
    weekend: weekend === "" ? [] : weekend.split(";").map(weekendLaw),
    substitution: substitution === "" ? [] : substitution.split(";").map(substitutionLaw),
  };
}

/**
 * One weekend law of a holiday table: an entry of column 14 of
 * `hc_holiday_tables`, `days/first/last/regions`.
 *
 * @param {string} entry
 * @returns {import("./hyper-calendar.d.ts").WeekendLaw}
 */
function weekendLaw(entry) {
  const [days, first, last, regions] = entry.split("/");
  return {
    days: days === "unread" ? null : days.split("+").map((day) => Number(day)),
    first: first === "" ? null : first,
    last: last === "" ? null : last,
    regions: regions === "" ? [] : regions.split(","),
  };
}

/**
 * One weekend-substitution law of a holiday table: an entry of column 15
 * of `hc_holiday_tables`, `trigger/direction/flags/avoid/first/last/regions`.
 *
 * @param {string} entry
 * @returns {import("./hyper-calendar.d.ts").SubstitutionLaw}
 */
function substitutionLaw(entry) {
  const [trigger, direction, flags, avoid, first, last, regions] = entry.split("/");
  const set = flags === "" ? [] : flags.split("+");
  return {
    trigger: trigger.split("+").map((day) => Number(day)),
    direction: /** @type {import("./hyper-calendar.d.ts").SubstituteDirection} */ (direction),
    skipOccupied: set.includes("skip-occupied"),
    onCollision: set.includes("on-collision"),
    avoid: avoid === "" ? [] : avoid.split("+").map((day) => Number(day)),
    first: first === "" ? null : Number(first),
    last: last === "" ? null : Number(last),
    regions: regions === "" ? [] : regions.split(","),
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
  const [liturgicalYear, sundayCycle, weekdayCycle, proper, sunday, week, weekEpiphanyOnSunday] =
    cells;
  return {
    liturgicalYear: integer(liturgicalYear, "liturgical year"),
    sundayCycle: /** @type {"A" | "B" | "C"} */ (sundayCycle),
    weekdayCycle: /** @type {"I" | "II"} */ (weekdayCycle),
    proper: optionalInteger(proper, "proper"),
    sundayInOrdinaryTime: optionalInteger(sunday, "sunday in ordinary time"),
    weekOfOrdinaryTime: optionalInteger(week, "week of ordinary time"),
    weekOfOrdinaryTimeEpiphanyOnSunday: optionalInteger(
      weekEpiphanyOnSunday,
      "week of ordinary time, epiphany on a sunday",
    ),
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
  const [sun, moon, elongation, tithi, sign, signId, signName] = cells;
  return {
    sunLongitude: decimal(sun, "sun"),
    moonLongitude: decimal(moon, "moon"),
    elongation: decimal(elongation, "elongation"),
    tithi: integer(tithi, "tithi"),
    sign: integer(sign, "sign"),
    signId: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (signId),
    signName,
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
  const [sign, signId, signName, decan, ruler, rulerName, degrees] = cells;
  return {
    sign: integer(sign, "sign"),
    signId: /** @type {import("./hyper-calendar.d.ts").TropicalSignId} */ (signId),
    signName,
    decan: integer(decan, "decan"),
    ruler: /** @type {import("./hyper-calendar.d.ts").DecanRuler} */ (ruler),
    rulerName,
    degreesIntoDecan: decimal(degrees, "degrees into decan"),
  };
}

/**
 * The one line of `hc_drekkana_at`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").Drekkana}
 */
function drekkanaLine(cells) {
  const [sign, signId, signName, drekkana, lord, lordName, degrees, lordSign, lordSignName, ayanamsa] = cells;
  return {
    sign: integer(sign, "sign"),
    signId: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (signId),
    signName,
    drekkana: integer(drekkana, "drekkana"),
    lord: /** @type {import("./hyper-calendar.d.ts").DecanRuler} */ (lord),
    lordName,
    degreesIntoDrekkana: decimal(degrees, "degrees into drekkana"),
    lordSign: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (lordSign),
    lordSignName,
    ayanamsa: /** @type {import("./hyper-calendar.d.ts").Ayanamsa} */ (ayanamsa),
  };
}

/**
 * One line of `hc_muhurtas`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").Muhurta}
 */
function muhurtaLine(cells) {
  const [half, number, name, start, end, mark, missing, missingDay, depression, arcseconds] = cells;
  return {
    half: /** @type {"day" | "night"} */ (half),
    number: integer(number, "number"),
    name,
    start: optionalInteger(start, "start"),
    end: optionalInteger(end, "end"),
    mark: /** @type {"abhijit" | "dur-muhurtam" | null} */ (optional(mark)),
    missing: missingSolarEvent(missing, missingDay, depression, arcseconds),
  };
}

/**
 * The line of `hc_amrita_siddhi`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").AmritaSiddhi}
 */
function amritaSiddhiLine(cells) {
  const [name, devanagari, nakshatra, nakshatraId, nakshatraName, start, end, falls, ayanamsa] = cells;
  return {
    name,
    devanagari,
    nakshatra: integer(nakshatra, "nakshatra"),
    nakshatraId: /** @type {import("./hyper-calendar.d.ts").NakshatraId} */ (nakshatraId),
    nakshatraName,
    start: optionalInteger(start, "start"),
    end: optionalInteger(end, "end"),
    falls: flag(falls, "falls"),
    ayanamsa: /** @type {import("./hyper-calendar.d.ts").Ayanamsa} */ (ayanamsa),
  };
}

/**
 * The line of `hc_nakshatra_at` and `hc_nakshatra_of_day`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").NakshatraStay}
 */
function nakshatraLine(cells) {
  const [nakshatra, nakshatraId, nakshatraName, entered, leaves, readAt, ayanamsa, ayanamsaName] = cells;
  return {
    nakshatra: integer(nakshatra, "nakshatra"),
    nakshatraId: /** @type {import("./hyper-calendar.d.ts").NakshatraId} */ (nakshatraId),
    nakshatraName,
    entered: integer(entered, "entered"),
    leaves: integer(leaves, "leaves"),
    readAt: integer(readAt, "read at"),
    ayanamsa: /** @type {import("./hyper-calendar.d.ts").Ayanamsa} */ (ayanamsa),
    ayanamsaName,
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
  const [title, rank, rankName, id] = cells;
  return { title, rank: /** @type {import("./hyper-calendar.d.ts").CommonWorshipRank} */ (rank), rankName, id };
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

/**
 * One line of `hc_territories`, `hc_subdivisions` and `hc_place_name`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").PlaceName}
 */
function placeName(cells) {
  const [code, name, englishName, localeUsed, draft, status] = cells;
  return {
    code,
    name: optional(name),
    englishName: optional(englishName),
    localeUsed: optional(localeUsed),
    draft: /** @type {import("./hyper-calendar.d.ts").PlaceDraft | null} */ (optional(draft)),
    status: /** @type {import("./hyper-calendar.d.ts").PlaceStatus} */ (status),
  };
}

/**
 * One line of `hc_relative_time` and `hc_relative_day`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").RelativeTime}
 */
function relativeTime(cells) {
  const [phrase, unit, count, localeUsed] = cells;
  return {
    phrase,
    unit: /** @type {import("./hyper-calendar.d.ts").HumanizeUnit} */ (unit),
    count: integer(count, "count"),
    localeUsed,
  };
}

/**
 * The line of `hc_relative_day_at`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").RelativeDayAt}
 */
function relativeDayAt(cells) {
  const [phrase, unit, count, time, localeUsed] = cells;
  return { ...relativeTime([phrase, unit, count, localeUsed]), time };
}

/**
 * The line of `hc_duration`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").HumanizedDuration}
 */
function humanizedDuration(cells) {
  const [phrase, negative, localeUsed] = cells;
  return { phrase, negative: flag(negative, "negative"), localeUsed };
}

/**
 * The line of the `humanize` number functions: the text and the language of
 * the vocabulary that wrote it.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").NaturalText}
 */
function naturalText(cells) {
  const [text, language] = cells;
  return { text, language };
}

/**
 * The line of the `humanize` functions that have words: the text and the
 * tag of the catalogue that wrote it.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").LocalizedNaturalText}
 */
function localizedNaturalText(cells) {
  const [text, localeUsed] = cells;
  return { text, localeUsed };
}

/**
 * The line of `hc_unit_choice`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").UnitChoice}
 */
function unitChoice(cells) {
  const [unit, count, half, thresholds, rounding] = cells;
  return {
    unit: /** @type {import("./hyper-calendar.d.ts").HumanizeUnit} */ (unit),
    count: integer(count, "count"),
    half: flag(half, "half"),
    thresholds: /** @type {import("./hyper-calendar.d.ts").ThresholdsName} */ (thresholds),
    rounding: /** @type {import("./hyper-calendar.d.ts").RoundingName} */ (rounding),
  };
}

/**
 * The line of `hc_relative_time_with`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").RelativeTimeWith}
 */
function relativeTimeWith(cells) {
  const [phrase, unit, count, half, localeUsed] = cells;
  return { ...relativeTime([phrase, unit, count, localeUsed]), half: flag(half, "half") };
}

/**
 * The line of `hc_approximate_duration`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").ApproximateDuration}
 */
function approximateDuration(cells) {
  const [phrase, hedge, unit, count, localeUsed] = cells;
  return {
    phrase,
    hedge: /** @type {import("./hyper-calendar.d.ts").Hedge} */ (hedge),
    unit: /** @type {import("./hyper-calendar.d.ts").HumanizeUnit} */ (unit),
    count: integer(count, "count"),
    localeUsed,
  };
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
 * The one line of `hc_jjy_call_sign_decode`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").JjyCallSign}
 */
function jjyCallSign(cells) {
  const [unixSeconds, fixed, hour, minute, stopStart, daytimeOnly, stopSpan] = cells;
  return {
    unixSeconds: integer(unixSeconds, "unix seconds"),
    fixed: integer(fixed, "fixed"),
    hour: integer(hour, "hour"),
    minute: integer(minute, "minute"),
    stopStart: integer(stopStart, "stop start"),
    daytimeOnly: flag(daytimeOnly, "daytime only"),
    stopSpan: integer(stopSpan, "stop span"),
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
 * One line of `hc_almanac_directions`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").AlmanacDirection}
 */
function almanacDirection(cells) {
  const [id, name, reading, branch, azimuth, meaning, year, branchNumber] = cells;
  return {
    id: /** @type {import("./hyper-calendar.d.ts").AlmanacGodId} */ (id),
    name,
    reading: optional(reading),
    branch: optional(branch),
    azimuth: optionalInteger(azimuth, "azimuth"),
    meaning: optional(meaning),
    year,
    branchNumber: optionalInteger(branchNumber, "branch number"),
  };
}

/**
 * One line of `hc_mansion_undertakings`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").MansionUndertaking}
 */
function mansionUndertaking(cells) {
  const [mansion, mansionName, grade, undertaking] = cells;
  return {
    mansion: integer(mansion, "mansion"),
    mansionName,
    grade: /** @type {import("./hyper-calendar.d.ts").UndertakingGrade} */ (grade),
    undertaking,
  };
}

/**
 * One line of `hc_almanac_person_days`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").AlmanacPersonDay}
 */
function almanacPersonDay(cells) {
  const [kind, id, name, keeps, applies] = cells;
  return {
    kind: /** @type {import("./hyper-calendar.d.ts").AlmanacPersonKind} */ (kind),
    id,
    name,
    keeps,
    applies: flag(applies, "applies"),
  };
}

/**
 * One line of `hc_tithi_at` or `hc_tithis_of_day`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").Tithi}
 */
function tithi(cells) {
  const [number, paksha, pakshaDay, name, began, ends, readAt, sky] = cells;
  return {
    number: integer(number, "number"),
    paksha: /** @type {"shukla" | "krishna"} */ (paksha),
    pakshaDay: integer(pakshaDay, "paksha day"),
    name,
    began: integer(began, "began"),
    ends: integer(ends, "ends"),
    readAt: integer(readAt, "read at"),
    sky: /** @type {"true" | "surya-siddhanta"} */ (sky),
  };
}

/**
 * One line of `hc_ayanamsa_at` or `hc_ayanamsa_from_anchor`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").AyanamsaValue}
 */
function ayanamsaValue(cells) {
  const [degrees, id, name, anchorJulianDate, anchorDegrees, readAt, kind] = cells;
  return {
    degrees: decimal(degrees, "degrees"),
    id,
    name,
    anchorJulianDate: decimal(anchorJulianDate, "anchor julian date"),
    anchorDegrees: decimal(anchorDegrees, "anchor degrees"),
    readAt: integer(readAt, "read at"),
    kind: /** @type {"mean" | "true"} */ (kind),
  };
}

/**
 * One line of `hc_tibetan_almanac_day`./**
 * One line of `hc_tibetan_almanac_day`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").TibetanAlmanacEntry}
 */
function tibetanAlmanacEntry(cells) {
  const [kind, id, name, tibetan, reading, value] = cells;
  return {
    kind: /** @type {import("./hyper-calendar.d.ts").TibetanAlmanacKind} */ (kind),
    id: optionalInteger(id, "id"),
    name: optional(name),
    tibetan: optional(tibetan),
    reading: optional(reading),
    value: optional(value),
  };
}

/**
 * One line of `hc_tibetan_planets`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").TibetanPlanet}
 */
function tibetanPlanet(cells) {
  const [planet, particularDay, meanHeliocentric, trueSlow, fast, meanValue, slowValue, fastValue] = cells;
  return {
    planet: /** @type {import("./hyper-calendar.d.ts").TibetanPlanetId} */ (planet),
    particularDay: integer(particularDay, "particular day"),
    meanHeliocentric,
    trueSlow,
    fast,
    meanHeliocentricValue: decimal(meanValue, "mean heliocentric value"),
    trueSlowValue: decimal(slowValue, "true slow value"),
    fastValue: decimal(fastValue, "fast value"),
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
  const [id, site, siteName, localeUsed, river, jupiter, jupiterName, sun, sunName, atNewMoon, from, to, holds] =
    cells;
  return {
    id: /** @type {import("./hyper-calendar.d.ts").KumbhYoga} */ (id),
    site,
    siteName,
    localeUsed,
    river,
    jupiter: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (jupiter),
    jupiterName,
    sun: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (sun),
    sunName,
    atNewMoon: flag(atNewMoon, "at new moon"),
    from: optionalInteger(from, "from"),
    to: optionalInteger(to, "to"),
    holds: holds === "" ? null : flag(holds, "holds"),
  };
}

/**
 * One line of `hc_kumbh_yogas`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").KumbhYogaInfo}
 */
function kumbhYogaInfo(cells) {
  const [id, site, siteName, localeUsed, river, jupiter, jupiterName, sun, sunName, atNewMoon, source] = cells;
  return {
    id: /** @type {import("./hyper-calendar.d.ts").KumbhYoga} */ (id),
    site,
    siteName,
    localeUsed,
    river,
    jupiter: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (jupiter),
    jupiterName,
    sun: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (sun),
    sunName,
    atNewMoon: flag(atNewMoon, "at new moon"),
    source,
  };
}

/**
 * One line of `hc_pushkaram`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").PushkaramDays}
 */
function pushkaramDays(cells) {
  const [id, name, localeUsed, region, sign, signName, first, last, missing, missingDay, depression, arcseconds] =
    cells;
  return {
    id,
    name,
    localeUsed,
    region: optional(region),
    sign: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (sign),
    signName,
    first: optionalInteger(first, "first"),
    last: optionalInteger(last, "last"),
    missing: missingSolarEvent(missing, missingDay, depression, arcseconds),
  };
}

/**
 * One line of `hc_jupiter_stations`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").JupiterStation}
 */
function jupiterStation(cells) {
  const [moment, kind, sign, signName, siderealLongitude] = cells;
  return {
    moment: integer(moment, "moment"),
    kind: /** @type {"retrograde" | "direct"} */ (kind),
    sign: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (sign),
    signName,
    siderealLongitude: decimal(siderealLongitude, "sidereal longitude"),
  };
}

/**
 * The one line of `hc_jupiter_at`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").JupiterPosition}
 */
function jupiterPosition(cells) {
  const [
    longitude, latitude, distance, siderealLongitude, sign, signName, degreesIntoSign, dailyMotion, retrograde,
    heliocentricLongitude, heliocentricLatitude, heliocentricDistance,
  ] = cells;
  return {
    longitude: decimal(longitude, "longitude"),
    latitude: decimal(latitude, "latitude"),
    distance: decimal(distance, "distance"),
    siderealLongitude: decimal(siderealLongitude, "sidereal longitude"),
    sign: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (sign),
    signName,
    degreesIntoSign: decimal(degreesIntoSign, "degrees into sign"),
    dailyMotion: decimal(dailyMotion, "daily motion"),
    retrograde: flag(retrograde, "retrograde"),
    heliocentricLongitude: decimal(heliocentricLongitude, "heliocentric longitude"),
    heliocentricLatitude: decimal(heliocentricLatitude, "heliocentric latitude"),
    heliocentricDistance: decimal(heliocentricDistance, "heliocentric distance"),
  };
}

/**
 * One line of `hc_jupiter_ingresses`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").JupiterIngress}
 */
function jupiterIngress(cells) {
  const [moment, from, fromName, to, toName, direction] = cells;
  return {
    moment: integer(moment, "moment"),
    from: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (from),
    fromName,
    to: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (to),
    toName,
    direction: /** @type {"forward" | "retrograde"} */ (direction),
  };
}

/**
 * One line of `hc_jupiter_risings`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").JupiterRising}
 */
function jupiterRising(cells) {
  const [rising, setting, siderealLongitude, nakshatra, nakshatraId, nakshatraName, year, position] = cells;
  return {
    rising: integer(rising, "rising"),
    setting: integer(setting, "setting"),
    siderealLongitude: decimal(siderealLongitude, "sidereal longitude"),
    nakshatra: integer(nakshatra, "nakshatra"),
    nakshatraId,
    nakshatraName,
    year,
    yearPosition: integer(position, "year position"),
  };
}

/**
 * The one line of `hc_kumbh_by_sky`: `hc_kumbh`'s thirteen cells, then two.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").KumbhBySky}
 */
function kumbhBySky(cells) {
  const occasion = kumbhOccasion(cells);
  return {
    ...occasion,
    holds: flag(cells[12], "holds"),
    jupiterThen: /** @type {import("./hyper-calendar.d.ts").SiderealSignId | null} */ (optional(cells[13])),
    jupiterLongitudeThen: cells[14] === "" ? null : decimal(cells[14], "jupiter longitude then"),
  };
}

/**
 * One line of `hc_pushkaram_by_sky`: `hc_pushkaram`'s, then the entry and the rule.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").PushkaramBySky}
 */
function pushkaramBySky(cells) {
  return {
    ...pushkaramDays(cells),
    entry: integer(cells[12], "entry"),
    rule: /** @type {import("./hyper-calendar.d.ts").PushkaramEntryRule} */ (cells[13]),
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
 * The one line of `hc_temporal_hour`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").TemporalHour}
 */
function temporalHour(cells) {
  const [reckoning, seconds, missing, missingDay, depression, arcseconds] = cells;
  return {
    reckoning: /** @type {import("./hyper-calendar.d.ts").ZmanimReckoning} */ (reckoning),
    seconds: seconds === "" ? null : decimal(seconds, "seconds"),
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
   * leap second. A day past the announced leap-second table has not been
   * announced: `strict` refuses it with `no-data`, and otherwise the answer
   * is false, as the last published offset holds.
   *
   * @param {number | bigint} unixSeconds
   * @param {boolean} [strict]
   * @returns {boolean}
   */
  dayHasLeapSecond(unixSeconds, strict = false) {
    const fn = this.#export("hc_day_has_leap_second");
    return toNumber(fn(toI64(unixSeconds, "unixSeconds"), strict ? 1 : 0), "hc_day_has_leap_second") === 1;
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
   * `TAI - UTC` at a POSIX instant, exactly: whole seconds and the
   * attoseconds after them. From 1972 it is a whole number of seconds;
   * from 1961 to 1971 it is not, and moves by 3·10⁻⁸ s in a second, so the
   * answer depends on `attoseconds`, where {@link HyperCalendar#taiMinusUtc}
   * gives the floor at the start of the second. `strict` refuses before
   * 1961 and past the announced leap-second table with `no-data`.
   *
   * @param {number | bigint} unixSeconds
   * @param {number | bigint} [attoseconds]
   * @param {boolean} [strict]
   * @returns {import("./hyper-calendar.d.ts").TaiMinusUtc}
   */
  taiMinusUtcExact(unixSeconds, attoseconds = 0, strict = false) {
    const fn = this.#export("hc_tai_minus_utc_exact");
    const seconds = toI64(unixSeconds, "unixSeconds");
    const attos = toU64(attoseconds, "attoseconds");
    const text = this.#text("hc_tai_minus_utc_exact", (buffer, capacity) => fn(seconds, attos, strict ? 1 : 0, buffer, capacity), true);
    return taiMinusUtc(this.#oneLine("hc_tai_minus_utc_exact", text, COLUMNS.taiMinusUtcExact));
  }

  /**
   * A TAI instant as a UTC label, exactly: the POSIX second, the
   * attoseconds into it, and whether it is an inserted leap second, named
   * by the POSIX second after it. From 1961 to 1971 the UTC reading of a
   * whole TAI second is not a whole second: TAI 8 s is POSIX −1 s and
   * 0.999 918 s, which {@link HyperCalendar#utcFromTai} reads as −1 and
   * loses the fraction of. `strict` refuses before 1961 and past the
   * announced leap-second table with `no-data`.
   *
   * @param {number | bigint} taiSeconds
   * @param {number | bigint} [taiAttoseconds]
   * @param {boolean} [strict]
   * @returns {import("./hyper-calendar.d.ts").UtcInstantLabel}
   */
  utcFromTaiExact(taiSeconds, taiAttoseconds = 0, strict = false) {
    const fn = this.#export("hc_utc_from_tai_exact");
    const seconds = toI64(taiSeconds, "taiSeconds");
    const attos = toU64(taiAttoseconds, "taiAttoseconds");
    const text = this.#text("hc_utc_from_tai_exact", (buffer, capacity) => fn(seconds, attos, strict ? 1 : 0, buffer, capacity), true);
    return utcInstantLabel(this.#oneLine("hc_utc_from_tai_exact", text, COLUMNS.utcFromTaiExact));
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
   * `year-not-written`, `weekday-mismatch`, `field-mismatch`,
   * `not-recognised`, `empty`, or the calendar's own refusal — and whose
   * `fixed` is `null`.
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
   * The fallback chain of a locale, the order its data is looked up
   * along, from the tag itself to `und`: each step with the rule that
   * led to it from the one before (`parent-locales` for a parent CLDR 48
   * names, `region` and `script` for truncation) and whether `hc-i18n`
   * carries an entry of data for that tag. A tag that does not parse is
   * refused as `malformed`; the empty string is the root locale.
   *
   * @param {string} locale
   * @returns {import("./hyper-calendar.d.ts").LocaleChainStep[]}
   */
  localeChain(locale) {
    const text = this.#call("hc_locale_chain", [["str", "locale", locale]]);
    return rows(text, COLUMNS.localeChain, "hc_locale_chain").map(localeChainStep);
  }

  /**
   * What a locale is: its subtags and `-u-` keys, the entry of data that
   * answers for it, its parent, default numbering system, week (the
   * first day and CLDR's `minDays`), direction, casing and plural rules.
   * A tag that does not parse is refused as `malformed`.
   *
   * @param {string} locale
   * @returns {import("./hyper-calendar.d.ts").LocaleInfo}
   */
  localeInfo(locale) {
    const text = this.#call("hc_locale_info", [["str", "locale", locale]]);
    return localeInfo(this.#oneLine("hc_locale_info", text, COLUMNS.localeInfo));
  }

  /**
   * The plural category a number has in a locale, by CLDR 48's rules,
   * with the operands read from the number as written: `"1"` is `one` in
   * English and `"1.0"` is `other`. `kind` is `cardinal`, the form after a
   * count, or `ordinal`, the form of a position: the English 2 is `two`
   * (2nd) and 3 `few` (3rd).
   *
   * @param {string} locale
   * @param {string} number a plain decimal, as text
   * @param {import("./hyper-calendar.d.ts").PluralKind} [kind]
   * @returns {import("./hyper-calendar.d.ts").PluralCategoryAnswer}
   */
  pluralCategory(locale, number, kind = "cardinal") {
    const text = this.#call("hc_plural_category", [
      ["str", "locale", locale], ["str", "number", String(number)], ["str", "kind", kind],
    ]);
    return pluralCategoryAnswer(this.#oneLine("hc_plural_category", text, COLUMNS.pluralCategory));
  }

  /**
   * The names a locale has for a calendar in a width and a context:
   * months (and the names of a leap year's months where they differ),
   * the calendar's other cycles, quarters and the day periods.
   *
   * @param {string} locale
   * @param {string} calendar
   * @param {import("./hyper-calendar.d.ts").NameWidth} [width]
   * @param {import("./hyper-calendar.d.ts").NameContext} [context]
   * @returns {import("./hyper-calendar.d.ts").LocaleName[]}
   */
  names(locale, calendar, width = "wide", context = "format") {
    const text = this.#call("hc_names", [
      ["str", "locale", locale], ["str", "calendar", calendar], ["str", "width", width],
      ["str", "context", context],
    ]);
    return rows(text, COLUMNS.names, "hc_names").map(localeName);
  }

  /**
   * A text recased as a locale cases it: `lower`, `upper` (`iyi` is
   * `İYİ` in Turkish), `capitalise-first`, `lowercase-first`,
   * `sentence-start` or `in-sentence`.
   *
   * @param {string} locale
   * @param {import("./hyper-calendar.d.ts").CaseMode} mode
   * @param {string} text
   * @returns {import("./hyper-calendar.d.ts").CasedText}
   */
  caseText(locale, mode, text) {
    const answer = this.#call("hc_case", [["str", "locale", locale], ["str", "mode", mode], ["str", "text", text]]);
    return casedText(this.#oneLine("hc_case", answer, COLUMNS.caseText));
  }

  /**
   * A text made safe to embed in text running a locale's direction, by
   * the Unicode bidirectional isolates: `field` isolates only where the
   * two directions disagree, `first-strong` always, and `strip` removes
   * every isolate.
   *
   * @param {string} locale
   * @param {import("./hyper-calendar.d.ts").IsolateMode} mode
   * @param {string} text
   * @returns {import("./hyper-calendar.d.ts").IsolatedText}
   */
  isolate(locale, mode, text) {
    const answer = this.#call("hc_isolate", [["str", "locale", locale], ["str", "mode", mode], ["str", "text", text]]);
    return isolatedText(this.#oneLine("hc_isolate", answer, COLUMNS.isolated));
  }

  /**
   * The day periods of a time of the civil clock in a locale: am or pm,
   * and the flexible period, *in the afternoon*, by CLDR 48's rules.
   *
   * @param {number} secondsOfDay
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").DayPeriodReading}
   */
  dayPeriod(secondsOfDay, locale = "und") {
    const fn = this.#export("hc_day_period");
    const seconds = toU32(secondsOfDay, "secondsOfDay");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_day_period", (buffer, capacity) => fn(seconds, pointer, len, buffer, capacity), true));
    const [half, halfName, period, abbreviated, wide, narrow, localeUsed] =
      this.#oneLine("hc_day_period", text, COLUMNS.dayPeriod);
    return {
      half: /** @type {"am" | "pm"} */ (half),
      halfName: optional(halfName),
      period: optional(period),
      abbreviated: optional(abbreviated),
      wide: optional(wide),
      narrow: optional(narrow),
      localeUsed,
    };
  }

  /**
   * An integer written in a CLDR numbering system: `hebr`, `grek`,
   * `arab`, `hanidec` and the rest `numberingSystems()` lists.
   *
   * @param {string} system
   * @param {number | bigint} value
   * @returns {string}
   */
  formatNumber(system, value) {
    const fn = this.#export("hc_format_number");
    const number = toI64(value, "value");
    const text = this.#withText(system, "system", (pointer, len) =>
      this.#text("hc_format_number", (buffer, capacity) => fn(pointer, len, number, buffer, capacity), true));
    return this.#oneLine("hc_format_number", text, ["text", "system"])[0];
  }

  /**
   * An integer read back out of a numbering system's notation.
   *
   * @param {string} system
   * @param {string} text
   * @returns {number}
   */
  parseNumber(system, text) {
    const fn = this.#export("hc_parse_number");
    return this.#withText(system, "system", (systemPointer, systemLen) =>
      this.#withText(text, "text", (textPointer, textLen) =>
        toNumber(fn(systemPointer, systemLen, textPointer, textLen), "hc_parse_number")));
  }

  /**
   * Every numbering system `formatNumber` writes.
   *
   * @returns {import("./hyper-calendar.d.ts").NumberingSystemInfo[]}
   */
  numberingSystems() {
    const fn = this.#export("hc_numbering_systems");
    const text = this.#text("hc_numbering_systems", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(text, COLUMNS.numberingSystems, "hc_numbering_systems").map(([system, algorithmic, digits]) => ({
      system,
      algorithmic: flag(algorithmic, "algorithmic"),
      digits: optional(digits),
    }));
  }

  /**
   * The eras a calendar is described with, named in a locale.
   *
   * @param {string} calendar
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").CalendarEra[]}
   */
  calendarEras(calendar, locale = "und") {
    const fn = this.#export("hc_calendar_eras");
    const text = this.#withText(calendar, "calendar", (calendarPointer, calendarLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#text("hc_calendar_eras", (buffer, capacity) =>
          fn(calendarPointer, calendarLen, localePointer, localeLen, buffer, capacity), true)));
    return rows(text, COLUMNS.calendarEras, "hc_calendar_eras").map(
      ([code, wide, abbreviated, narrow, calendarId, localeUsed]) => ({
        code,
        wide: optional(wide),
        abbreviated: optional(abbreviated),
        narrow: optional(narrow),
        calendar: calendarId,
        localeUsed,
      }));
  }

  /**
   * Every era of a table, one row each: `japanese` (the 248 eras of the
   * table, where {@link calendarEras} has the locale data's 236),
   * `chinese-regnal` or `korean-regnal`. A table not named is `unknown`.
   *
   * @param {import("./hyper-calendar.d.ts").EraTableId} table
   * @returns {import("./hyper-calendar.d.ts").EraTableRow[]}
   */
  eraTable(table) {
    const fn = this.#export("hc_era_table");
    const text = this.#withText(table, "table", (pointer, len) =>
      this.#text("hc_era_table", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.eraTable, "hc_era_table").map(
      ([code, name, reading, romanised, group, firstYear, lastYear, start, last, status, otherStart, note]) => ({
        code,
        name,
        reading,
        romanised,
        group,
        firstYear: integer(firstYear, "first year"),
        lastYear: optionalInteger(lastYear, "last year"),
        start: optionalInteger(start, "start"),
        last: optionalInteger(last, "last"),
        status,
        otherStart: optionalInteger(otherStart, "other start"),
        note: optional(note),
      }));
  }

  /**
   * The modern Olympic Games of a season, `summer` or `winter`, one row
   * each in order. A season not named is `unknown`.
   *
   * @param {"summer" | "winter"} season
   * @returns {import("./hyper-calendar.d.ts").OlympicGames[]}
   */
  olympicGames(season) {
    const fn = this.#export("hc_olympic_games");
    const text = this.#withText(season, "season", (pointer, len) =>
      this.#text("hc_olympic_games", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.olympicGames, "hc_olympic_games").map(
      ([number, year, host, status, opening, closing]) => ({
        number: optionalInteger(number, "number"),
        year: integer(year, "year"),
        host,
        status: /** @type {"celebrated" | "not-held" | "scheduled"} */ (status),
        opening: optionalInteger(opening, "opening"),
        closing: optionalInteger(closing, "closing"),
      }));
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
   * table, `region`, which may be empty, a subdivision, and `group`, which
   * may be left out or empty for everyone, a group of people the table
   * gives days to alone (`women`, `children`).
   *
   * @param {string} code
   * @param {string} region
   * @param {number | bigint} fixed
   * @param {string} [group]
   * @returns {boolean}
   */
  holidayIsDayOff(code, region, fixed, group = "") {
    const fn = this.#export("hc_holiday_is_day_off");
    const day = toI64(fixed, "fixed");
    return this.#withText(code, "code", (codePointer, codeLen) =>
      this.#withText(region, "region", (regionPointer, regionLen) =>
        this.#withText(group, "group", (groupPointer, groupLen) =>
          toNumber(
            fn(codePointer, codeLen, regionPointer, regionLen, groupPointer, groupLen, day),
            "hc_holiday_is_day_off",
          ) === 1)));
  }

  /**
   * A fixed day moved by `count` business days of a holiday table, in a
   * subdivision and for a group as `holidayIsDayOff` scopes it; the
   * starting day is never counted.
   *
   * @param {string} code
   * @param {string} region
   * @param {number | bigint} fixed
   * @param {number | bigint} count
   * @param {string} [group]
   * @returns {number}
   */
  holidayAddBusinessDays(code, region, fixed, count, group = "") {
    const fn = this.#export("hc_holiday_add_business_days");
    const day = toI64(fixed, "fixed");
    const steps = toI64(count, "count");
    return this.#withText(code, "code", (codePointer, codeLen) =>
      this.#withText(region, "region", (regionPointer, regionLen) =>
        this.#withText(group, "group", (groupPointer, groupLen) =>
          toNumber(
            fn(codePointer, codeLen, regionPointer, regionLen, groupPointer, groupLen, day, steps),
            "hc_holiday_add_business_days",
          ))));
  }

  /**
   * The number of business days of a holiday table from one fixed day up
   * to but not including another, in a subdivision and for a group.
   *
   * @param {string} code
   * @param {string} region
   * @param {number | bigint} fromFixed
   * @param {number | bigint} toFixed
   * @param {string} [group]
   * @returns {number}
   */
  holidayBusinessDaysBetween(code, region, fromFixed, toFixed, group = "") {
    const fn = this.#export("hc_holiday_business_days_between");
    const from = toI64(fromFixed, "fromFixed");
    const to = toI64(toFixed, "toFixed");
    return this.#withText(code, "code", (codePointer, codeLen) =>
      this.#withText(region, "region", (regionPointer, regionLen) =>
        this.#withText(group, "group", (groupPointer, groupLen) =>
          toNumber(
            fn(codePointer, codeLen, regionPointer, regionLen, groupPointer, groupLen, from, to),
            "hc_holiday_business_days_between",
          ))));
  }

  /**
   * Whether a fixed day is a weekend day in a holiday table, in the
   * subdivision `region` names or, when it is empty, nationwide: under the
   * weekend law in force on the day in the region, as column 14 of
   * `holidayTables` lists the laws. A day on which the region's weekend law
   * was not read, the `unread-weekend` gap of `holidaysInYear`, is refused as
   * `out-of-range`, not answered as a day of no weekend.
   *
   * @param {string} code
   * @param {string} region
   * @param {number | bigint} fixed
   * @returns {boolean}
   */
  holidayIsWeekend(code, region, fixed) {
    const fn = this.#export("hc_holiday_is_weekend");
    const day = toI64(fixed, "fixed");
    return this.#withText(code, "code", (codePointer, codeLen) =>
      this.#withText(region, "region", (regionPointer, regionLen) =>
        toNumber(
          fn(codePointer, codeLen, regionPointer, regionLen, day),
          "hc_holiday_is_weekend",
        ) === 1));
  }

  /**
   * The first holiday of a table after a fixed day: the row `holidaysInYear`
   * writes for the first entry strictly after it, of the kinds `kind` lists,
   * `;`-separated, or of the kinds that stop work, `public` and `bank`, when
   * it is empty. A gap that could hide a nearer entry, and a table with
   * none of the kind within sixteen years, are refused as `no-data`.
   *
   * @param {string} code
   * @param {string} region
   * @param {number | bigint} fixed
   * @param {string} [group]
   * @param {string} [kind]
   * @returns {import("./hyper-calendar.d.ts").HolidayInYear}
   */
  holidayNext(code, region, fixed, group = "", kind = "") {
    return this.#holidayBeyond("hc_holiday_next", code, region, fixed, group, kind);
  }

  /**
   * The last holiday of a table before a fixed day, as `holidayNext` finds
   * the first after it.
   *
   * @param {string} code
   * @param {string} region
   * @param {number | bigint} fixed
   * @param {string} [group]
   * @param {string} [kind]
   * @returns {import("./hyper-calendar.d.ts").HolidayInYear}
   */
  holidayPrevious(code, region, fixed, group = "", kind = "") {
    return this.#holidayBeyond("hc_holiday_previous", code, region, fixed, group, kind);
  }

  /**
   * The one row of `hc_holiday_next` or `hc_holiday_previous`.
   *
   * @param {string} name
   * @param {string} code
   * @param {string} region
   * @param {number | bigint} fixed
   * @param {string} group
   * @param {string} kind
   * @returns {import("./hyper-calendar.d.ts").HolidayInYear}
   */
  #holidayBeyond(name, code, region, fixed, group, kind) {
    const fn = this.#export(name);
    const day = toI64(fixed, "fixed");
    const text = this.#withText(code, "code", (codePointer, codeLen) =>
      this.#withText(region, "region", (regionPointer, regionLen) =>
        this.#withText(group, "group", (groupPointer, groupLen) =>
          this.#withText(kind, "kind", (kindPointer, kindLen) =>
            this.#text(name, (buffer, capacity) =>
              fn(codePointer, codeLen, regionPointer, regionLen, groupPointer, groupLen, kindPointer, kindLen, day,
                buffer, capacity), true)))));
    return rows(text, COLUMNS.holidaysInYear, name).map(holidayInYear)[0];
  }

  /**
   * The holidays of a Gregorian year in a table, in a subdivision when
   * `region` is not empty, for a group of people when `group` is given, and
   * of the kinds `kind` lists, `;`-separated (`"public;bank"`), when it is
   * given: the entries of those kinds and the gaps of the rules of those
   * kinds. A kind that is no kind of `HolidayKind` is refused as `unknown`.
   *
   * @param {string} code
   * @param {string} region
   * @param {number | bigint} year
   * @param {string} [group]
   * @param {string} [kind]
   * @returns {import("./hyper-calendar.d.ts").HolidayInYear[]}
   */
  holidaysInYear(code, region, year, group = "", kind = "") {
    const fn = this.#export("hc_holidays_in_year");
    const y = toI64(year, "year");
    const text = this.#withText(code, "code", (codePointer, codeLen) =>
      this.#withText(region, "region", (regionPointer, regionLen) =>
        this.#withText(group, "group", (groupPointer, groupLen) =>
          this.#withText(kind, "kind", (kindPointer, kindLen) =>
            this.#text("hc_holidays_in_year", (buffer, capacity) =>
              fn(codePointer, codeLen, regionPointer, regionLen, groupPointer, groupLen, kindPointer, kindLen, y,
                buffer, capacity), true)))));
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
   * {@link holidayCodes} lists them: each table's nationwide lines, then
   * its subdivisions' and groups' own lines, which name the region or the
   * group they belong to.
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
   * Every tradition that names the 72 pentads, in the table's order.
   *
   * @returns {import("./hyper-calendar.d.ts").PentadTradition[]}
   */
  pentadTraditions() {
    const fn = this.#export("hc_pentad_traditions");
    const text = this.#text("hc_pentad_traditions", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(text, COLUMNS.pentadTraditions, "hc_pentad_traditions").map(pentadTradition);
  }

  /**
   * The pentad in effect on a fixed day at a meridian, named by a tradition
   * of {@link pentadTraditions}: the same pentad as {@link pentadInEffect},
   * with the tradition's name, its English gloss and the alternate reading
   * its text prints beside the name. A tradition not listed is `unknown`.
   *
   * @param {number | bigint} fixed
   * @param {import("./hyper-calendar.d.ts").PentadTraditionId} tradition
   * @param {string | number} [meridian]
   * @returns {import("./hyper-calendar.d.ts").PentadInTradition}
   */
  pentadInTradition(fixed, tradition, meridian = "universal") {
    const fn = this.#export("hc_pentad_in_tradition");
    const day = toI64(fixed, "fixed");
    const name = typeof meridian === "number" ? String(meridian) : meridian;
    const text = this.#withText(tradition, "tradition", (traditionPointer, traditionLen) =>
      this.#withText(name, "meridian", (pointer, len) =>
        this.#text("hc_pentad_in_tradition", (buffer, capacity) =>
          fn(day, traditionPointer, traditionLen, pointer, len, buffer, capacity), true)));
    const lines = rows(text, COLUMNS.pentadInTradition, "hc_pentad_in_tradition");
    if (lines.length !== 1) {
      throw new HcError("malformed", {
        export: "hc_pentad_in_tradition",
        message: `hc_pentad_in_tradition wrote ${lines.length} lines, not one`,
      });
    }
    return pentadInTradition(lines[0]);
  }
  /**
   * The 雑節 of a Gregorian year at a meridian, one row a day, then the days
   * the older rules place elsewhere under ids of their own (`nyubai-classical`,
   * `hangesho-classical`, `spring-shanichi-classical`,
   * `autumn-shanichi-classical`).
   *
   * @param {number | bigint} year
   * @param {string | number} [meridian]
   * @returns {import("./hyper-calendar.d.ts").ZassetsuDay[]}
   */
  zassetsuInYear(year, meridian = "universal") {
    const text = this.#yearLines("hc_zassetsu_in_year", year, meridian);
    return rows(text, COLUMNS.zassetsu, "hc_zassetsu_in_year").map(
      ([id, name, romaji, englishName, rule, day, last, firstOxDay, secondOxDay]) => ({
        id,
        name,
        romaji,
        englishName,
        rule,
        day: integer(day, "day"),
        last: optionalInteger(last, "last"),
        firstOxDay: optionalInteger(firstOxDay, "first ox day"),
        secondOxDay: optionalInteger(secondOxDay, "second ox day"),
      }));
  }

  /**
   * The other seasonal days and spans of a Gregorian year: the three 伏 and the
   * nine nines of the Chinese year, the dog days under each convention, the
   * quarter and term days of Britain and Ireland, and the Turkish folk year's
   * named days.
   *
   * @param {number | bigint} year
   * @param {string | number} [meridian]
   * @returns {import("./hyper-calendar.d.ts").SeasonalDay[]}
   */
  seasonalDaysInYear(year, meridian = "universal") {
    const text = this.#yearLines("hc_seasonal_days_in_year", year, meridian);
    return rows(text, COLUMNS.seasonalDay, "hc_seasonal_days_in_year").map(
      ([kind, id, name, localName, first, last, group]) => ({
        kind: /** @type {import("./hyper-calendar.d.ts").SeasonalDayKind} */ (kind),
        id,
        name,
        localName,
        first: integer(first, "first"),
        last: integer(last, "last"),
        group: optional(group),
      }));
  }

  /**
   * The lines of an export of a year and a meridian.
   *
   * @param {string} exportName
   * @param {number | bigint} year
   * @param {string | number} meridian
   * @returns {string}
   */
  #yearLines(exportName, year, meridian) {
    const fn = this.#export(exportName);
    const y = toI64(year, "year");
    const name = typeof meridian === "number" ? String(meridian) : meridian;
    return this.#withText(name, "meridian", (pointer, len) =>
      this.#text(exportName, (buffer, capacity) => fn(y, pointer, len, buffer, capacity), true));
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
   * one loaded with {@link loadZone}, or one of the eighteen built in.
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
   * The drekkāṇa, the Hindu third of a sidereal sign, the Sun is in at a
   * POSIX instant, in an ayanamsa's zodiac, with its lord.
   *
   * @param {number | bigint} unixSeconds
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @returns {import("./hyper-calendar.d.ts").Drekkana}
   */
  drekkanaAt(unixSeconds, ayanamsa) {
    const fn = this.#export("hc_drekkana_at");
    const instant = toI64(unixSeconds, "unixSeconds");
    const text = this.#withText(ayanamsa, "ayanamsa", (pointer, len) =>
      this.#text("hc_drekkana_at", (buffer, capacity) => fn(instant, pointer, len, buffer, capacity), true));
    return drekkanaLine(this.#oneLine("hc_drekkana_at", text, COLUMNS.drekkana));
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
   * Where Jupiter is at an instant, tropical and sidereal, from the
   * complete VSOP87B series for Jupiter with light-time, aberration and
   * nutation. The sidereal longitude is by the ayanāṃśa. An instant outside
   * the years −1000 to 3000 is `out-of-range`.
   *
   * @param {number | bigint} unixSeconds
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @returns {import("./hyper-calendar.d.ts").JupiterPosition}
   */
  jupiterAt(unixSeconds, ayanamsa) {
    const fn = this.#export("hc_jupiter_at");
    const seconds = toI64(unixSeconds, "unixSeconds");
    const text = this.#withText(ayanamsa, "ayanamsa", (ayanamsaPointer, ayanamsaLen) =>
      this.#text("hc_jupiter_at", (buffer, capacity) =>
        fn(seconds, ayanamsaPointer, ayanamsaLen, buffer, capacity), true));
    return jupiterPosition(this.#oneLine("hc_jupiter_at", text, COLUMNS.jupiterAt));
  }

  /**
   * Jupiter's crossings of the boundaries of the sidereal signs in
   * `[from, to)`, in time order. Jupiter turns back out of a sign about two
   * years in three, so a year's lines come in runs. A span longer than a
   * hundred Julian years or an end outside −1000 to 3000 is `out-of-range`.
   *
   * @param {number | bigint} fromUnixSeconds
   * @param {number | bigint} toUnixSeconds
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @returns {import("./hyper-calendar.d.ts").JupiterIngress[]}
   */
  jupiterIngresses(fromUnixSeconds, toUnixSeconds, ayanamsa) {
    const fn = this.#export("hc_jupiter_ingresses");
    const from = toI64(fromUnixSeconds, "fromUnixSeconds");
    const to = toI64(toUnixSeconds, "toUnixSeconds");
    const text = this.#withText(ayanamsa, "ayanamsa", (ayanamsaPointer, ayanamsaLen) =>
      this.#text("hc_jupiter_ingresses", (buffer, capacity) =>
        fn(from, to, ayanamsaPointer, ayanamsaLen, buffer, capacity), true));
    return rows(text, COLUMNS.jupiterIngress, "hc_jupiter_ingresses").map(jupiterIngress);
  }

  /**
   * Jupiter's heliacal risings in `[from, to)`, in time order, each with the
   * name the *Bṛhatsaṃhitā* gives the year of Jupiter that begins with it:
   * the lunar month whose nakṣatras hold the rising. A rising is Jupiter's
   * longitude, west of the Sun's after their conjunction, passing 11°, so
   * this is the true sky by a fixed arc, not what a Siddhāntic almanac
   * prints. A span longer than a hundred Julian years or an end outside
   * −1000 to 3000 is `out-of-range`.
   *
   * @param {number | bigint} fromUnixSeconds
   * @param {number | bigint} toUnixSeconds
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @returns {import("./hyper-calendar.d.ts").JupiterRising[]}
   */
  jupiterRisings(fromUnixSeconds, toUnixSeconds, ayanamsa) {
    const fn = this.#export("hc_jupiter_risings");
    const from = toI64(fromUnixSeconds, "fromUnixSeconds");
    const to = toI64(toUnixSeconds, "toUnixSeconds");
    const text = this.#withText(ayanamsa, "ayanamsa", (ayanamsaPointer, ayanamsaLen) =>
      this.#text("hc_jupiter_risings", (buffer, capacity) =>
        fn(from, to, ayanamsaPointer, ayanamsaLen, buffer, capacity), true));
    return rows(text, COLUMNS.jupiterRising, "hc_jupiter_risings").map(jupiterRising);
  }

  /**
   * Jupiter's stations in `[fromUnixSeconds, toUnixSeconds)`: where its
   * apparent longitude stops changing and it turns back (`retrograde`,
   * *vakri*) or resumes (`direct`, *mārgī*), with the sidereal sign and
   * longitude it turns at. A span longer than a hundred Julian years or an
   * end outside −1000 to 3000 is `out-of-range`.
   *
   * @param {number | bigint} fromUnixSeconds
   * @param {number | bigint} toUnixSeconds
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @returns {import("./hyper-calendar.d.ts").JupiterStation[]}
   */
  jupiterStations(fromUnixSeconds, toUnixSeconds, ayanamsa) {
    const fn = this.#export("hc_jupiter_stations");
    const from = toI64(fromUnixSeconds, "fromUnixSeconds");
    const to = toI64(toUnixSeconds, "toUnixSeconds");
    const text = this.#withText(ayanamsa, "ayanamsa", (pointer, len) =>
      this.#text("hc_jupiter_stations", (buffer, capacity) => fn(from, to, pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.jupiterStation, "hc_jupiter_stations").map(jupiterStation);
  }

  /**
   * {@link kumbhBySky} for every condition of {@link kumbhYogas} in a
   * Gregorian year, in that order: the ones whose `holds` is true are the
   * sites the year's sky meets.
   *
   * @param {number | bigint} year
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").KumbhBySky[]}
   */
  kumbhsInYearBySky(year, ayanamsa, locale = "und") {
    const fn = this.#export("hc_kumbhs_in_year_by_sky");
    const y = toI64(year, "year");
    const text = this.#withText(ayanamsa, "ayanamsa", (ayanamsaPointer, ayanamsaLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#text("hc_kumbhs_in_year_by_sky", (buffer, capacity) =>
          fn(y, ayanamsaPointer, ayanamsaLen, localePointer, localeLen, buffer, capacity), true)));
    return rows(text, COLUMNS.kumbhBySky, "hc_kumbhs_in_year_by_sky").map(kumbhBySky);
  }

  /**
   * {@link kumbh} with Jupiter's sign computed at the occasion's first  /**
   * {@link kumbh} with Jupiter's sign computed at the occasion's first
   * moment, in the zodiac of the same ayanāṃśa, in place of the caller's.
   *
   * @param {import("./hyper-calendar.d.ts").KumbhYoga} yoga
   * @param {number | bigint} year
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").KumbhBySky}
   */
  kumbhBySky(yoga, year, ayanamsa, locale = "und") {
    const fn = this.#export("hc_kumbh_by_sky");
    const y = toI64(year, "year");
    const text = this.#withText(yoga, "yoga", (yogaPointer, yogaLen) =>
      this.#withText(ayanamsa, "ayanamsa", (ayanamsaPointer, ayanamsaLen) =>
        this.#withText(locale, "locale", (localePointer, localeLen) =>
          this.#text("hc_kumbh_by_sky", (buffer, capacity) =>
            fn(yogaPointer, yogaLen, y, ayanamsaPointer, ayanamsaLen, localePointer, localeLen, buffer, capacity),
          true))));
    return kumbhBySky(this.#oneLine("hc_kumbh_by_sky", text, COLUMNS.kumbhBySky));
  }

  /**
   * {@link pushkaram} for the entry of Jupiter into a sign that falls in a
   * Gregorian year, found, not given; an empty list in a year with none.
   * Where Jupiter enters, turns back and enters again, `rule` says which
   * entry counts.
   *
   * @param {import("./hyper-calendar.d.ts").SiderealSignId} sign
   * @param {number | bigint} year
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @param {import("./hyper-calendar.d.ts").PushkaramEntryRule} rule
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @param {string} [meridian]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").PushkaramBySky[]}
   */
  pushkaramBySky(sign, year, ayanamsa, rule, latitude, longitude, elevation = 0, meridian = "", locale = "und") {
    const fn = this.#export("hc_pushkaram_by_sky");
    const y = toI64(year, "year");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(sign, "sign", (signPointer, signLen) =>
      this.#withText(ayanamsa, "ayanamsa", (ayanamsaPointer, ayanamsaLen) =>
        this.#withText(rule, "rule", (rulePointer, ruleLen) =>
          this.#withText(meridian, "meridian", (meridianPointer, meridianLen) =>
            this.#withText(locale, "locale", (localePointer, localeLen) =>
              this.#text("hc_pushkaram_by_sky", (buffer, capacity) =>
                fn(signPointer, signLen, y, ayanamsaPointer, ayanamsaLen, rulePointer, ruleLen, lat, lon, elev,
                  meridianPointer, meridianLen, localePointer, localeLen, buffer, capacity), true))))));
    return rows(text, COLUMNS.pushkaramBySky, "hc_pushkaram_by_sky").map(pushkaramBySky);
  }

  /**
   * {@link pushkaramBySky} for every sign Jupiter enters in a Gregorian year,
   * in one call: the lines of each sign it enters, one sign after another in
   * the order of the entries, the same as the sign-by-sign calls give. A year
   * in which it enters two signs has both; where it enters a sign, turns back
   * and enters it again, `rule` says which entry counts; an empty list in a
   * year with none.
   *
   * @param {number | bigint} year
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @param {import("./hyper-calendar.d.ts").PushkaramEntryRule} rule
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @param {string} [meridian]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").PushkaramBySky[]}
   */
  pushkaramsInYear(year, ayanamsa, rule, latitude, longitude, elevation = 0, meridian = "", locale = "und") {
    const fn = this.#export("hc_pushkarams_in_year");
    const y = toI64(year, "year");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(ayanamsa, "ayanamsa", (ayanamsaPointer, ayanamsaLen) =>
      this.#withText(rule, "rule", (rulePointer, ruleLen) =>
        this.#withText(meridian, "meridian", (meridianPointer, meridianLen) =>
          this.#withText(locale, "locale", (localePointer, localeLen) =>
            this.#text("hc_pushkarams_in_year", (buffer, capacity) =>
              fn(y, ayanamsaPointer, ayanamsaLen, rulePointer, ruleLen, lat, lon, elev,
                meridianPointer, meridianLen, localePointer, localeLen, buffer, capacity), true)))));
    return rows(text, COLUMNS.pushkaramsInYear, "hc_pushkarams_in_year").map(pushkaramBySky);
  }

  /**
   * Python's `time.gmtime(seconds)`: the UTC reading of a POSIX second as
   * the nine fields of a `struct_time`, `dst` 0.
   *
   * @param {number | bigint} unixSeconds
   * @returns {import("./hyper-calendar.d.ts").StructTime}
   */
  gmtime(unixSeconds) {
    const fn = this.#export("hc_gmtime");
    const seconds = toI64(unixSeconds, "unixSeconds");
    const text = this.#text("hc_gmtime", (buffer, capacity) => fn(seconds, buffer, capacity), true);
    return structTime(this.#oneLine("hc_gmtime", text, COLUMNS.structTime));
  }

  /**
   * Python's `time.localtime(seconds)` in a zone: the wall-clock reading
   * of a POSIX second as a `struct_time`, `dst` 1 where the zone's rules
   * call the time daylight saving. The zone is read as {@link zoneOffset}
   * reads it.
   *
   * @param {number | bigint} unixSeconds
   * @param {string} zone
   * @returns {import("./hyper-calendar.d.ts").StructTime}
   */
  localtime(unixSeconds, zone) {
    const text = this.#call("hc_localtime", [["i64", "unixSeconds", unixSeconds], ["str", "zone", zone]]);
    return structTime(this.#oneLine("hc_localtime", text, COLUMNS.structTime));
  }

  /**
   * Python's `calendar.timegm(tuple)`: the POSIX second of a UTC reading.
   * The day, hour, minute and second add up unchecked, as Python's do; a
   * month outside 1 to 12 is `invalid-date`.
   *
   * @param {number | bigint} year
   * @param {number | bigint} month
   * @param {number | bigint} day
   * @param {number | bigint} [hour]
   * @param {number | bigint} [minute]
   * @param {number | bigint} [second]
   * @returns {number}
   */
  timegm(year, month, day, hour = 0, minute = 0, second = 0) {
    const fn = this.#export("hc_timegm");
    return toNumber(
      fn(toI64(year, "year"), toI64(month, "month"), toI64(day, "day"), toI64(hour, "hour"), toI64(minute, "minute"), toI64(second, "second")),
      "hc_timegm",
    );
  }

  /**
   * Python's `time.mktime(tuple)` in a zone: the POSIX second of a
   * wall-clock reading, each field checked; `policy` says what a reading
   * two instants name, or none, becomes: `earliest`, `latest`, `reject`
   * (`invalid-date` for either) or `push-forward`.
   *
   * @param {number | bigint} year
   * @param {number | bigint} month
   * @param {number | bigint} day
   * @param {number | bigint} hour
   * @param {number | bigint} minute
   * @param {number | bigint} second
   * @param {string} zone
   * @param {import("./hyper-calendar.d.ts").DisambiguationPolicy} [policy]
   * @returns {number}
   */
  mktime(year, month, day, hour, minute, second, zone, policy = "reject") {
    const fn = this.#export("hc_mktime");
    const fields = [toI64(year, "year"), toI64(month, "month"), toI64(day, "day"), toI64(hour, "hour"), toI64(minute, "minute"), toI64(second, "second")];
    return this.#withText(zone, "zone", (zonePointer, zoneLen) =>
      this.#withText(policy, "policy", (policyPointer, policyLen) =>
        toNumber(fn(...fields, zonePointer, zoneLen, policyPointer, policyLen), "hc_mktime")));
  }

  /**
   * Python's `calendar.isleap(year)`.
   *
   * @param {number | bigint} year
   * @returns {boolean}
   */
  isleap(year) {
    return toNumber(this.#export("hc_isleap")(toI64(year, "year")), "hc_isleap") === 1;
  }

  /**
   * Python's `calendar.leapdays(y1, y2)`: the leap years from `y1` up to
   * but not including `y2`, counted backwards when `y2` is before `y1`.
   *
   * @param {number | bigint} y1
   * @param {number | bigint} y2
   * @returns {number}
   */
  leapdays(y1, y2) {
    return toNumber(this.#export("hc_leapdays")(toI64(y1, "y1"), toI64(y2, "y2")), "hc_leapdays");
  }

  /**
   * Python's `calendar.weekday(year, month, day)`: Monday 0 to Sunday 6,
   * where {@link weekday} answers Monday 1 to Sunday 7 for a fixed day.
   *
   * @param {number | bigint} year
   * @param {number} month
   * @param {number} day
   * @returns {number}
   */
  calendarWeekday(year, month, day) {
    const fn = this.#export("hc_calendar_weekday");
    return toNumber(fn(toI64(year, "year"), toU32(month, "month"), toU32(day, "day")), "hc_calendar_weekday");
  }

  /**
   * Python's `calendar.monthrange(year, month)`: the weekday of the first
   * of the month, Monday 0, and the days in the month.
   *
   * @param {number | bigint} year
   * @param {number} month
   * @returns {import("./hyper-calendar.d.ts").MonthRange}
   */
  monthrange(year, month) {
    const fn = this.#export("hc_monthrange");
    const [y, m] = [toI64(year, "year"), toU32(month, "month")];
    const text = this.#text("hc_monthrange", (buffer, capacity) => fn(y, m, buffer, capacity), true);
    const [firstWeekday, days] = this.#oneLine("hc_monthrange", text, COLUMNS.monthrange);
    return { firstWeekday: integer(firstWeekday, "first weekday"), days: integer(days, "days") };
  }

  /**
   * Python's `calendar.monthcalendar(year, month)`: a row of seven numbers
   * for each week of the month, `0` for a day outside it, the week
   * beginning on `firstWeekday`, Monday 0 to Sunday 6.
   *
   * @param {number | bigint} year
   * @param {number} month
   * @param {number} [firstWeekday]
   * @returns {number[][]}
   */
  monthcalendar(year, month, firstWeekday = 0) {
    const fn = this.#export("hc_monthcalendar");
    const [y, m, first] = [toI64(year, "year"), toU32(month, "month"), toU32(firstWeekday, "firstWeekday")];
    const text = this.#text("hc_monthcalendar", (buffer, capacity) => fn(y, m, first, buffer, capacity), true);
    return rows(text, COLUMNS.monthcalendar, "hc_monthcalendar").map((week) =>
      week.map((cell, index) => integer(cell, `day ${index + 1}`)));
  }

  /**
   * A fixed day's week of the year under a week rule: the first day of
   * the week as an ISO weekday number, Monday 1 to Sunday 7, and the
   * fewest days a first week holds, 1 to 7. ISO 8601 is 1 and 4, the
   * default; `strftime`'s `%U` is 7 and 7, its `%W` 1 and 7; a locale's
   * pair is {@link localeInfo}'s `firstDay` and `minDays`.
   *
   * @param {number | bigint} fixed
   * @param {number} [firstWeekday]
   * @param {number} [minDays]
   * @returns {import("./hyper-calendar.d.ts").WeekOfYear}
   */
  weekOfYear(fixed, firstWeekday = 1, minDays = 4) {
    const fn = this.#export("hc_week_of_year");
    const [day, first, min] = [toI64(fixed, "fixed"), toU32(firstWeekday, "firstWeekday"), toU32(minDays, "minDays")];
    const text = this.#text("hc_week_of_year", (buffer, capacity) => fn(day, first, min, buffer, capacity), true);
    const [weekYear, week, weeksInYear, weekOfMonth] = this.#oneLine("hc_week_of_year", text, COLUMNS.weekOfYear);
    return {
      weekYear: integer(weekYear, "week year"),
      week: integer(week, "week"),
      weeksInYear: integer(weeksInYear, "weeks in year"),
      weekOfMonth: integer(weekOfMonth, "week of month"),
    };
  }

  /**
   * The day and the moment the year changes at the solar New Year of the
   * `burmese`, `khmer` or `lao` calendar, `year` in the calendar's own
   * count: the Myanmar Era, the Buddhist Era or the Chulasakarat.
   *
   * @param {import("./hyper-calendar.d.ts").SolarNewYearCalendar} calendar
   * @param {number | bigint} year
   * @returns {import("./hyper-calendar.d.ts").SolarNewYear}
   */
  solarNewYear(calendar, year) {
    const text = this.#call("hc_solar_new_year", [["str", "calendar", calendar], ["i64", "year", year]]);
    return solarNewYear(this.#oneLine("hc_solar_new_year", text, COLUMNS.solarNewYear));
  }

  /**
   * Every pentad that begins in a Gregorian year at a meridian, in date
   * order, named by every tradition of {@link pentadTraditions} at once.
   *
   * @param {number | bigint} year
   * @param {string | number} [meridian]
   * @returns {import("./hyper-calendar.d.ts").PentadOfYear[]}
   */
  pentadsInYear(year, meridian = "universal") {
    const text = this.#yearLines("hc_pentads_in_year", year, meridian);
    return rows(text, COLUMNS.pentadsInYear, "hc_pentads_in_year").map(
      ([index, begins, ends, chinese, japanese, jokyo, senmyo]) => ({
        index: integer(index, "index"),
        begins: integer(begins, "begins"),
        ends: integer(ends, "ends"),
        names: { chinese, japanese, jokyo, senmyo },
      }));
  }

  /**
   * Moonrise on a fixed day at a place against a named horizon, or `null`
   * with the missing moonrise named: the Moon skips a local day about once
   * a month.
   *
   * @param {import("./hyper-calendar.d.ts").HorizonId} horizon
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @returns {import("./hyper-calendar.d.ts").MoonCrossing}
   */
  moonrise(horizon, fixed, latitude, longitude, elevation = 0) {
    return this.#moonCrossing("hc_moonrise", horizon, fixed, latitude, longitude, elevation);
  }

  /**
   * Moonset on a fixed day at a place against a named horizon, or `null`
   * with the missing moonset named.
   *
   * @param {import("./hyper-calendar.d.ts").HorizonId} horizon
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @returns {import("./hyper-calendar.d.ts").MoonCrossing}
   */
  moonset(horizon, fixed, latitude, longitude, elevation = 0) {
    return this.#moonCrossing("hc_moonset", horizon, fixed, latitude, longitude, elevation);
  }

  /**
   * @param {string} exportName
   * @param {string} horizon
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} elevation
   * @returns {import("./hyper-calendar.d.ts").MoonCrossing}
   */
  #moonCrossing(exportName, horizon, fixed, latitude, longitude, elevation) {
    const fn = this.#export(exportName);
    const day = toI64(fixed, "fixed");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(horizon, "horizon", (pointer, len) =>
      this.#text(exportName, (buffer, capacity) =>
        fn(pointer, len, day, lat, lon, elev, buffer, capacity), true));
    return moonCrossing(this.#oneLine(exportName, text, COLUMNS.moonCrossing));
  }

  /**
   * UT1R at a UT1 instant, counted as POSIX seconds are from 1970-01-01
   * 00:00 UT1, by the IERS 2010 zonal tide model: UT1R − UT1 in seconds
   * and the UT1R reading. Outside −1000 through 3000 is `out-of-range`.
   *
   * @param {number} ut1UnixSeconds
   * @returns {import("./hyper-calendar.d.ts").RegularizedUt1}
   */
  ut1rIers2010(ut1UnixSeconds) {
    return this.#regularizedUt1("hc_ut1r_iers2010", ut1UnixSeconds);
  }

  /**
   * UT1S at a UT1 instant, as {@link ut1rIers2010} with all 62 zonal tides
   * removed.
   *
   * @param {number} ut1UnixSeconds
   * @returns {import("./hyper-calendar.d.ts").RegularizedUt1}
   */
  ut1sIers2010(ut1UnixSeconds) {
    return this.#regularizedUt1("hc_ut1s_iers2010", ut1UnixSeconds);
  }

  /**
   * @param {string} exportName
   * @param {number} ut1UnixSeconds
   * @returns {import("./hyper-calendar.d.ts").RegularizedUt1}
   */
  #regularizedUt1(exportName, ut1UnixSeconds) {
    const fn = this.#export(exportName);
    const seconds = toF64(ut1UnixSeconds, "ut1UnixSeconds");
    const text = this.#text(exportName, (buffer, capacity) => fn(seconds, buffer, capacity), true);
    const [minusUt1, reading] = this.#oneLine(exportName, text, COLUMNS.regularizedUt1);
    return { minusUt1: decimal(minusUt1, "minus UT1"), reading: decimal(reading, "reading") };
  }

  /**
   * The effect on UT1, in seconds, of the zonal tides of IERS Conventions
   * 2010, Table 8.1, whose period is under a limit in days: 35 for UT1R's
   * tides, `Infinity` (the default) for all of them, UT1S's. A limit that
   * is not positive is `out-of-range`.
   *
   * @param {number} ut1UnixSeconds
   * @param {number} [periodLimitDays]
   * @returns {number}
   */
  zonalTideUt1Effect(ut1UnixSeconds, periodLimitDays = Infinity) {
    const fn = this.#export("hc_zonal_tide_ut1_effect");
    const seconds = toF64(ut1UnixSeconds, "ut1UnixSeconds");
    const limit = toF64(periodLimitDays, "periodLimitDays");
    const text = this.#text("hc_zonal_tide_ut1_effect", (buffer, capacity) => fn(seconds, limit, buffer, capacity), true);
    return decimal(this.#oneLine("hc_zonal_tide_ut1_effect", text, COLUMNS.value)[0], "value");
  }

  /**
   * The equation of time at a POSIX instant, read as Universal Time:
   * apparent solar time less mean solar time, in seconds, positive when a
   * sundial is ahead of the clock. Outside −1000 through 3000 is
   * `out-of-range`.
   *
   * @param {number | bigint} unixSeconds
   * @returns {number}
   */
  equationOfTime(unixSeconds) {
    const text = this.#call("hc_equation_of_time", [["i64", "unixSeconds", unixSeconds]]);
    return decimal(this.#oneLine("hc_equation_of_time", text, COLUMNS.value)[0], "value");
  }

  /**
   * Apparent solar noon on a local day at a place, as POSIX seconds of
   * Universal Time, rounded down: the Sun's upper transit, which every day
   * has. A place off the globe or a day outside −1000 through 3000 is
   * `out-of-range`.
   *
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @returns {number}
   */
  solarNoon(fixed, latitude, longitude, elevation = 0) {
    return this.#transit("hc_solar_noon", fixed, latitude, longitude, elevation);
  }

  /**
   * Apparent solar midnight opening a local day at a place, half a day
   * before its {@link solarNoon}.
   *
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @returns {number}
   */
  solarMidnight(fixed, latitude, longitude, elevation = 0) {
    return this.#transit("hc_solar_midnight", fixed, latitude, longitude, elevation);
  }

  /**
   * @param {string} exportName
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} elevation
   * @returns {number}
   */
  #transit(exportName, fixed, latitude, longitude, elevation) {
    const fn = this.#export(exportName);
    const day = toI64(fixed, "fixed");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    return toNumber(fn(day, lat, lon, elev), exportName);
  }

  /**
   * The start of a named twilight on a local day at a place: `civil` (the
   * Sun 6° below the horizon), `nautical` (12°) or `astronomical` (18°).
   * When the Sun does not cross the depression that morning the instant is
   * `null` and `missing` names the `depression`, in arcminutes and
   * arcseconds.
   *
   * @param {import("./hyper-calendar.d.ts").TwilightName} twilight
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @returns {import("./hyper-calendar.d.ts").SolarEvent}
   */
  dawn(twilight, fixed, latitude, longitude, elevation = 0) {
    return this.#twilight("hc_dawn", twilight, fixed, latitude, longitude, elevation);
  }

  /**
   * The end of a named twilight on a local day at a place, as
   * {@link dawn} for the evening.
   *
   * @param {import("./hyper-calendar.d.ts").TwilightName} twilight
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @returns {import("./hyper-calendar.d.ts").SolarEvent}
   */
  dusk(twilight, fixed, latitude, longitude, elevation = 0) {
    return this.#twilight("hc_dusk", twilight, fixed, latitude, longitude, elevation);
  }

  /**
   * @param {string} exportName
   * @param {string} twilight
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} elevation
   * @returns {import("./hyper-calendar.d.ts").SolarEvent}
   */
  #twilight(exportName, twilight, fixed, latitude, longitude, elevation) {
    const fn = this.#export(exportName);
    const day = toI64(fixed, "fixed");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(twilight, "twilight", (pointer, len) =>
      this.#text(exportName, (buffer, capacity) =>
        fn(pointer, len, day, lat, lon, elev, buffer, capacity), true));
    return solarEvent(this.#oneLine(exportName, text, COLUMNS.solarEvent));
  }

  /**
   * The twelve tropical sign periods of a Gregorian year at a meridian, in
   * date order from Aquarius to Capricorn: the instants the Sun entered
   * and left each sign and its first and last days.
   *
   * @param {number | bigint} year
   * @param {import("./hyper-calendar.d.ts").Meridian} [meridian]
   * @returns {import("./hyper-calendar.d.ts").SignPeriod<import("./hyper-calendar.d.ts").TropicalSignId>[]}
   */
  tropicalSignsInYear(year, meridian = "universal") {
    const text = this.#yearLines("hc_tropical_signs_in_year", year, meridian);
    return rows(text, COLUMNS.signsInYear, "hc_tropical_signs_in_year").map((cells) =>
      /** @type {import("./hyper-calendar.d.ts").SignPeriod<import("./hyper-calendar.d.ts").TropicalSignId>} */ (signPeriod(cells)));
  }

  /**
   * The twelve sidereal sign periods, the saṅkrāntis, of a Gregorian year
   * in the zodiac of an ayanāṃśa at a meridian, from the first saṅkrānti
   * on or after 1 January.
   *
   * @param {number | bigint} year
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @param {import("./hyper-calendar.d.ts").Meridian} [meridian]
   * @returns {import("./hyper-calendar.d.ts").SiderealSignPeriod[]}
   */
  siderealSignsInYear(year, ayanamsa, meridian = "universal") {
    const name = typeof meridian === "number" ? String(meridian) : meridian;
    const text = this.#call("hc_sidereal_signs_in_year", [
      ["i64", "year", year], ["str", "ayanamsa", ayanamsa], ["str", "meridian", name],
    ]);
    return rows(text, COLUMNS.siderealSignsInYear, "hc_sidereal_signs_in_year").map((cells) => ({
      .../** @type {import("./hyper-calendar.d.ts").SignPeriod<import("./hyper-calendar.d.ts").SiderealSignId>} */ (signPeriod(cells)),
      ayanamsa: /** @type {import("./hyper-calendar.d.ts").Ayanamsa} */ (cells[7]),
    }));
  }

  /**
   * The fixed day of 伝統的七夕, the National Astronomical Observatory's
   * traditional Tanabata, of a Gregorian year at a meridian, `japan` for
   * the Observatory's days: 10 August 2024, 29 August 2025, 19 August 2026.
   *
   * @param {number | bigint} year
   * @param {import("./hyper-calendar.d.ts").Meridian} [meridian]
   * @returns {number}
   */
  traditionalTanabata(year, meridian = "japan") {
    const fn = this.#export("hc_traditional_tanabata");
    const y = toI64(year, "year");
    const name = typeof meridian === "number" ? String(meridian) : meridian;
    return this.#withText(name, "meridian", (pointer, len) =>
      toNumber(fn(y, pointer, len), "hc_traditional_tanabata"));
  }

  /**
   * The principal phases of the Moon inside a Gregorian month at a
   * meridian, in time order: four or five, since a month can hold one
   * phase twice. A month outside 1 to 12 is `invalid-date`.
   *
   * @param {number | bigint} year
   * @param {number} month
   * @param {import("./hyper-calendar.d.ts").Meridian} [meridian]
   * @returns {import("./hyper-calendar.d.ts").PrincipalPhase[]}
   */
  principalPhasesInMonth(year, month, meridian = "universal") {
    const name = typeof meridian === "number" ? String(meridian) : meridian;
    const text = this.#call("hc_principal_phases_in_month", [
      ["i64", "year", year], ["u32", "month", month], ["str", "meridian", name],
    ]);
    return rows(text, COLUMNS.principalPhase, "hc_principal_phases_in_month").map(([phase, instant, day]) => ({
      phase: /** @type {import("./hyper-calendar.d.ts").MoonPhaseName} */ (phase),
      instant: integer(instant, "instant"),
      day: integer(day, "day"),
    }));
  }

  /**
   * The kind of lunar year a year of the `khmer` (Buddhist Era) or `lao`
   * (Chulasakarat) calendar is under the *suryayatra* rule: `normal`,
   * `extra-day` or `extra-month`, with its days, its Thai and Khmer names
   * and the day and second the solar New Year changes the year.
   *
   * @param {import("./hyper-calendar.d.ts").SoutheastAsianCalendar} calendar
   * @param {number | bigint} year
   * @returns {import("./hyper-calendar.d.ts").SoutheastAsianYearType}
   */
  southeastAsianYearType(calendar, year) {
    const text = this.#call("hc_southeast_asian_year_type", [["str", "calendar", calendar], ["i64", "year", year]]);
    const [name, y, kind, days, extraMonth, thaiName, khmerName, newYear, seconds] =
      this.#oneLine("hc_southeast_asian_year_type", text, COLUMNS.yearType);
    return {
      calendar: /** @type {import("./hyper-calendar.d.ts").SoutheastAsianCalendar} */ (name),
      year: integer(y, "year"),
      kind: /** @type {import("./hyper-calendar.d.ts").LunarYearKind} */ (kind),
      days: integer(days, "days"),
      extraMonth: flag(extraMonth, "extra month"),
      thaiName,
      khmerName,
      newYear: integer(newYear, "new year"),
      seconds: integer(seconds, "seconds"),
    };
  }

  /**
   * A fixed day in the Maya counts under a named correlation constant —
   * `gmt` (584 283, the default), `gmt2` (584 285) or `martin-skidmore`
   * (584 286), or the constant itself as text: the Long Count, the
   * tzolkʼin and the haabʼ under the same constant.
   *
   * @param {number | bigint} fixed
   * @param {import("./hyper-calendar.d.ts").MayaCorrelation} [correlation]
   * @returns {import("./hyper-calendar.d.ts").MayaLongCount}
   */
  mayaLongCount(fixed, correlation = "gmt") {
    const text = this.#call("hc_maya_long_count", [["i64", "fixed", fixed], ["str", "correlation", correlation]]);
    const [id, constant, longCount, baktun, katun, tun, uinal, kin, tzolkinNumber, tzolkinName, haabDay, haabMonth] =
      this.#oneLine("hc_maya_long_count", text, COLUMNS.mayaLongCount);
    return {
      id,
      correlation: integer(constant, "correlation"),
      longCount,
      baktun: integer(baktun, "baktun"),
      katun: integer(katun, "katun"),
      tun: integer(tun, "tun"),
      uinal: integer(uinal, "uinal"),
      kin: integer(kin, "kin"),
      tzolkinNumber: integer(tzolkinNumber, "tzolkin number"),
      tzolkinName,
      haabDay: integer(haabDay, "haab day"),
      haabMonth,
    };
  }

  /**
   * A fixed day in the Akan *Adaduanan*, the 42-day cycle of the six-day
   * *nnanson* against the seven-day week, with its compound name and the
   * *dabɔne* it is, if any.
   *
   * @param {number | bigint} fixed
   * @returns {import("./hyper-calendar.d.ts").AkanDay}
   */
  akanDay(fixed) {
    const text = this.#call("hc_akan_day", [["i64", "fixed", fixed]]);
    const [round, day, nnanson, nnansonName, nnawotwe, nnawotweName, shortName, name, dabone] =
      this.#oneLine("hc_akan_day", text, COLUMNS.akanDay);
    return {
      round: integer(round, "round"),
      day: integer(day, "day"),
      nnanson: integer(nnanson, "nnanson"),
      nnansonName,
      nnawotwe: integer(nnawotwe, "nnawotwe"),
      nnawotweName,
      shortName,
      name,
      dabone: optional(dabone),
    };
  }

  /**
   * A fixed day's *weton*, the Javanese five-day *pasaran* against the
   * seven-day week, with the *neptu* of each and their sum.
   *
   * @param {number | bigint} fixed
   * @returns {import("./hyper-calendar.d.ts").Weton}
   */
  weton(fixed) {
    const text = this.#call("hc_weton", [["i64", "fixed", fixed]]);
    const [round, day, dina, dinaName, pasaran, pasaranName, dinaNeptu, pasaranNeptu, neptu, name] =
      this.#oneLine("hc_weton", text, COLUMNS.weton);
    return {
      round: integer(round, "round"),
      day: integer(day, "day"),
      dina: integer(dina, "dina"),
      dinaName,
      pasaran: integer(pasaran, "pasaran"),
      pasaranName,
      dinaNeptu: integer(dinaNeptu, "dina neptu"),
      pasaranNeptu: integer(pasaranNeptu, "pasaran neptu"),
      neptu: integer(neptu, "neptu"),
      name,
    };
  }

  /**
   * Sri Lanka's Buddhist year of a fixed day, counted from the Vesak Full
   * Moon Poya Day, with the Vesak of its Gregorian year and the days the
   * Buddhist year began and ends on, `null` where no order read fixes
   * them. A day outside 2023 through 2027 is `out-of-range`.
   *
   * @param {number | bigint} fixed
   * @returns {import("./hyper-calendar.d.ts").BuddhistLkYear}
   */
  buddhistLkYear(fixed) {
    const text = this.#call("hc_buddhist_lk_year", [["i64", "fixed", fixed]]);
    const [year, gregorianYear, vesak, began, ends] = this.#oneLine("hc_buddhist_lk_year", text, COLUMNS.buddhistLkYear);
    return {
      year: integer(year, "year"),
      gregorianYear: integer(gregorianYear, "gregorian year"),
      vesak: integer(vesak, "vesak"),
      began: optionalInteger(began, "began"),
      ends: optionalInteger(ends, "ends"),
    };
  }

  /**
   * The Tiruvaḷḷuvar year of a fixed day, Tamil Nadu's count that changes
   * at Thai 1: the Gregorian year of that Thai 1 plus 31. Outside the
   * Tamil calendar's years, 1700 through 2299, is `out-of-range`.
   *
   * @param {number | bigint} fixed
   * @returns {number}
   */
  tiruvalluvarYear(fixed) {
    const fn = this.#export("hc_tiruvalluvar_year");
    return toNumber(fn(toI64(fixed, "fixed")), "hc_tiruvalluvar_year");
  }

  /**
   * The Sun's entries into the nakṣatras in `[fromUnixSeconds,
   * toUnixSeconds)`, in time order, in the zodiac of an ayanāṃśa: the
   * nakṣatra entered and the one left. A span longer than a hundred
   * Julian years or an end outside −1000 to 3000 is `out-of-range`.
   *
   * @param {number | bigint} fromUnixSeconds
   * @param {number | bigint} toUnixSeconds
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @returns {import("./hyper-calendar.d.ts").SolarNakshatraIngress[]}
   */
  solarNakshatraIngresses(fromUnixSeconds, toUnixSeconds, ayanamsa) {
    const text = this.#call("hc_solar_nakshatra_ingresses", [
      ["i64", "fromUnixSeconds", fromUnixSeconds], ["i64", "toUnixSeconds", toUnixSeconds], ["str", "ayanamsa", ayanamsa],
    ]);
    return rows(text, COLUMNS.solarNakshatraIngress, "hc_solar_nakshatra_ingresses").map(
      ([moment, into, intoId, intoName, from, fromId, fromName]) => ({
        moment: integer(moment, "moment"),
        into: integer(into, "into"),
        intoId: /** @type {import("./hyper-calendar.d.ts").NakshatraId} */ (intoId),
        intoName,
        from: integer(from, "from"),
        fromId: /** @type {import("./hyper-calendar.d.ts").NakshatraId} */ (fromId),
        fromName,
      }));
  }

  /**
   * A year of a Japanese era as the era's dates write it: 元 for 1, the
   * Han numerals of Japanese after it. Below 1 is `out-of-range`.
   *
   * @param {number | bigint} year
   * @returns {string}
   */
  japaneseEraYear(year) {
    const text = this.#call("hc_japanese_era_year", [["i64", "year", year]]);
    return this.#oneLine("hc_japanese_era_year", text, COLUMNS.japaneseEraYear)[0];
  }

  /**
   * The standard date, time and date-time formats a locale carries for a
   * calendar, in CLDR's four lengths, and the `availableFormats` items the
   * table keeps: eighteen patterns, read under the CLDR calendar type the
   * registry identifier maps to. A calendar the registry does not carry is
   * `unknown`.
   *
   * @param {string} locale
   * @param {string} calendar
   * @returns {import("./hyper-calendar.d.ts").LocaleFormat[]}
   */
  localeFormat(locale, calendar) {
    const text = this.#call("hc_locale_format", [["str", "locale", locale], ["str", "calendar", calendar]]);
    return rows(text, COLUMNS.localeFormat, "hc_locale_format").map(([kind, length, pattern, calendarType]) => ({
      kind: /** @type {import("./hyper-calendar.d.ts").FormatKind} */ (kind),
      length,
      pattern: optional(pattern),
      calendarType,
    }));
  }

  /**
   * The plural categories a locale's cardinal or ordinal rules name, in
   * CLDR's order, each with a number that falls in it and the language of
   * the rules.
   *
   * @param {string} locale
   * @param {import("./hyper-calendar.d.ts").PluralKind} [kind]
   * @returns {import("./hyper-calendar.d.ts").PluralCategoryExample[]}
   */
  pluralCategories(locale, kind = "cardinal") {
    const text = this.#call("hc_plural_categories", [["str", "locale", locale], ["str", "kind", kind]]);
    return rows(text, COLUMNS.pluralCategories, "hc_plural_categories").map(([category, example, rules]) => ({
      category: /** @type {import("./hyper-calendar.d.ts").PluralCategoryName} */ (category),
      example: optional(example),
      rules,
    }));
  }

  /**
   * Every epoch `hc-core` carries, with its TAI reading as whole seconds
   * from 1970-01-01 00:00:00 TAI and attoseconds, and the document that
   * defines it.
   *
   * @returns {import("./hyper-calendar.d.ts").EpochRow[]}
   */
  epochs() {
    const fn = this.#export("hc_epochs");
    const text = this.#text("hc_epochs", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(text, COLUMNS.epochs, "hc_epochs").map(([id, description, taiSeconds, attoseconds, source]) => ({
      id,
      description,
      taiSeconds: bigInteger(taiSeconds, "tai seconds"),
      attoseconds: bigInteger(attoseconds, "attoseconds"),
      source,
    }));
  }

  /**
   * The CLDR list patterns a style joins the parts of a duration with, in
   * a locale: `long` takes CLDR's `standard` list, `short` its `unit` list
   * and `narrow` its `unit-narrow`.
   *
   * @param {import("./hyper-calendar.d.ts").RelativeStyle} [style]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").ListForms}
   */
  listForms(style = "long", locale = "und") {
    const text = this.#call("hc_list_forms", [["str", "style", style], ["str", "locale", locale]]);
    const [two, start, middle, end, localeUsed] = this.#oneLine("hc_list_forms", text, COLUMNS.listForms);
    return { two, start, middle, end, localeUsed };
  }

  /**
   * `humanize`'s `clamp`: the value written with `format` — `display`,
   * `fixed:N` or `percent:N` — or, below `floor` or above `ceil`, that
   * bound written the same way after its token, `<0.01`, `>99%`. Python's
   * function form has no shape here: format the value the line gives.
   *
   * @param {number} value
   * @param {string} [format]
   * @param {number | null} [floor]
   * @param {number | null} [ceil]
   * @param {string} [floorToken]
   * @param {string} [ceilToken]
   * @returns {import("./hyper-calendar.d.ts").LocalizedNaturalText}
   */
  clamp(value, format = "display", floor = null, ceil = null, floorToken = "<", ceilToken = ">") {
    const bound = (/** @type {number | null} */ limit, /** @type {string} */ what) => {
      if (limit === null) {
        return "";
      }
      return String(toF64(limit, what));
    };
    const text = this.#call("hc_clamp", [
      ["f64", "value", value],
      ["str", "format", format],
      ["str", "floor", bound(floor, "floor")],
      ["str", "ceil", bound(ceil, "ceil")],
      ["str", "floorToken", floorToken],
      ["str", "ceilToken", ceilToken],
    ]);
    return localizedNaturalText(this.#oneLine("hc_clamp", text, COLUMNS.localizedNaturalText));
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
   * Call an export that writes one text, from arguments of stated kinds:
   * `i64`, `u64`, `u32`, `i32`, `f64`, `flag` (a boolean, passed as 0 or 1)
   * and `str` (UTF-8 in linear memory, passed as a pointer and a length).
   *
   * @param {string} exportName
   * @param {ReadonlyArray<readonly [string, string, unknown]>} args kind, name and value
   * @returns {string}
   */
  #call(exportName, args) {
    const fn = this.#export(exportName);
    /** @type {Array<number | bigint | string>} */
    const converted = args.map(([kind, name, value]) => {
      switch (kind) {
        case "i64": return toI64(/** @type {any} */ (value), name);
        case "u64": return toU64(/** @type {any} */ (value), name);
        case "u32": return toU32(/** @type {any} */ (value), name);
        case "i32": return toI32(/** @type {any} */ (value), name);
        case "f64": return toF64(/** @type {any} */ (value), name);
        case "flag": return value ? 1 : 0;
        default: return String(value);
      }
    });
    /**
     * @param {number} index
     * @param {Array<number | bigint>} passed
     * @returns {string}
     */
    const run = (index, passed) => {
      if (index === args.length) {
        return this.#text(exportName, (buffer, capacity) => fn(...passed, buffer, capacity), true);
      }
      if (args[index][0] === "str") {
        return this.#withText(/** @type {string} */ (converted[index]), args[index][1], (pointer, len) =>
          run(index + 1, [...passed, pointer, len]));
      }
      return run(index + 1, [...passed, /** @type {number | bigint} */ (converted[index])]);
    };
    return run(0, []);
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
   * Universal Time, the yoga reckoned with an ayanamsa: `lahiri`, `lahiri-rashtriya`, `lahiri-crc-1955`, `lahiri-drik`, `raman`,
   * `krishnamurti`, `reingold-dershowitz` or `fagan-bradley`; or both on
   * the *Sūrya Siddhānta*'s sky, `surya-siddhanta`.
   *
   * @param {number | bigint} unixSeconds
   * @param {import("./hyper-calendar.d.ts").Ayanamsa | "surya-siddhanta"} ayanamsa
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
   * @param {import("./hyper-calendar.d.ts").Ayanamsa | "surya-siddhanta"} ayanamsa
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
   * The tithi in progress at a POSIX instant, with the moments it began and
   * ends. `sky` is `true`, an ayanāṃśa (which the tithi does not depend on)
   * or `surya-siddhanta`.
   *
   * @param {number | bigint} unixSeconds
   * @param {import("./hyper-calendar.d.ts").Ayanamsa | "true" | "surya-siddhanta"} sky
   * @returns {import("./hyper-calendar.d.ts").Tithi}
   */
  tithiAt(unixSeconds, sky) {
    const fn = this.#export("hc_tithi_at");
    const instant = toI64(unixSeconds, "unixSeconds");
    const text = this.#withText(sky, "sky", (pointer, len) =>
      this.#text("hc_tithi_at", (buffer, capacity) => fn(instant, pointer, len, buffer, capacity), true));
    return tithi(this.#oneLine("hc_tithi_at", text, COLUMNS.tithi));
  }

  /**
   * The tithis in progress between a day's sunrise at a place and the next,
   * with whether each holds the day's sunrise, is repeated (holds both) or
   * is skipped (holds neither); a day without a sunrise there is `no-data`.
   *
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} elevation
   * @param {import("./hyper-calendar.d.ts").Ayanamsa | "true" | "surya-siddhanta"} sky
   * @returns {import("./hyper-calendar.d.ts").TithiOfDay[]}
   */
  tithisOfDay(fixed, latitude, longitude, elevation, sky) {
    const fn = this.#export("hc_tithis_of_day");
    const day = toI64(fixed, "fixed");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(sky, "sky", (pointer, len) =>
      this.#text("hc_tithis_of_day", (buffer, capacity) =>
        fn(day, lat, lon, elev, pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.tithisOfDay, "hc_tithis_of_day").map((cells) => ({
      ...tithi(cells),
      atSunrise: flag(cells[8], "at sunrise"),
      repeated: flag(cells[9], "repeated"),
      skipped: flag(cells[10], "skipped"),
    }));
  }

  /**
   * Every named ayanāṃśa with its anchor.
   *
   * @returns {import("./hyper-calendar.d.ts").AyanamsaInfo[]}
   */
  ayanamsas() {
    const fn = this.#export("hc_ayanamsas");
    const text = this.#text("hc_ayanamsas", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(text, COLUMNS.ayanamsaTable, "hc_ayanamsas").map(
      ([id, name, anchorJulianDate, anchorDegrees, source, kind]) => ({
        id: /** @type {import("./hyper-calendar.d.ts").Ayanamsa} */ (id),
        name,
        anchorJulianDate: decimal(anchorJulianDate, "anchor julian date"),
        anchorDegrees: decimal(anchorDegrees, "anchor degrees"),
        source,
        kind: /** @type {"mean" | "true"} */ (kind),
      }));
  }

  /**
   * A named ayanāṃśa's value in degrees at a POSIX instant.
   *
   * @param {number | bigint} unixSeconds
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @returns {import("./hyper-calendar.d.ts").AyanamsaValue}
   */
  ayanamsaAt(unixSeconds, ayanamsa) {
    const fn = this.#export("hc_ayanamsa_at");
    const instant = toI64(unixSeconds, "unixSeconds");
    const text = this.#withText(ayanamsa, "ayanamsa", (pointer, len) =>
      this.#text("hc_ayanamsa_at", (buffer, capacity) => fn(instant, pointer, len, buffer, capacity), true));
    return ayanamsaValue(this.#oneLine("hc_ayanamsa_at", text, COLUMNS.ayanamsaValue));
  }

  /**
   * The two readings of a festival's day where the sects part, `smarta` and
   * `vaishnava`, with how each takes the day and where it is from.
   *
   * @returns {import("./hyper-calendar.d.ts").FestivalReading[]}
   */
  festivalReadings() {
    const fn = this.#export("hc_festival_readings");
    const text = this.#text("hc_festival_readings", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(text, COLUMNS.festivalReading, "hc_festival_readings").map(
      ([id, name, rule, source]) => ({
        id: /** @type {import("./hyper-calendar.d.ts").FestivalReadingId} */ (id),
        name,
        rule,
        source,
      }));
  }

  /**
   * The day of Kṛṣṇa Janmāṣṭamī in a Gregorian year at a place by a
   * reading, `smarta` or `vaishnava`; a year outside 1700 to 2299 or a place
   * the Sun does not rise at every day is `out-of-range`.
   *
   * @param {number | bigint} year
   * @param {import("./hyper-calendar.d.ts").FestivalReadingId} reading
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} elevation
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @returns {import("./hyper-calendar.d.ts").FestivalDay}
   */
  janmashtami(year, reading, latitude, longitude, elevation, ayanamsa) {
    const fn = this.#export("hc_janmashtami");
    const y = toI64(year, "year");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(reading, "reading", (readingPointer, readingLen) =>
      this.#withText(ayanamsa, "ayanamsa", (ayanamsaPointer, ayanamsaLen) =>
        this.#text("hc_janmashtami", (buffer, capacity) =>
          fn(y, readingPointer, readingLen, lat, lon, elev, ayanamsaPointer, ayanamsaLen, buffer, capacity), true)));
    const [id, yearCell, fixed, gy, gm, gd, sunriseTithi, ayanamsaId] =
      this.#oneLine("hc_janmashtami", text, COLUMNS.janmashtami);
    return {
      reading: /** @type {import("./hyper-calendar.d.ts").FestivalReadingId} */ (id),
      year: integer(yearCell, "year"),
      fixed: integer(fixed, "fixed"),
      gregorian: [integer(gy, "gregorian year"), integer(gm, "gregorian month"), integer(gd, "gregorian day")],
      sunriseTithi: integer(sunriseTithi, "sunrise tithi"),
      ayanamsa: /** @type {import("./hyper-calendar.d.ts").Ayanamsa} */ (ayanamsaId),
    };
  }

  /**
   * The Vaiṣṇava day of a tithi of an amānta month of a Śaka year at a
   * place: the first day whose sunrise carries the tithi or a later one.
   * `month` is 1 for Chaitra through 12 for Phālguna and `tithi` 1 through
   * 30; a Śaka year outside 1622 to 2221 or a month outside 1 to 12 is
   * `out-of-range` and a tithi outside 1 to 30 `invalid-date`.
   *
   * @param {number | bigint} sakaYear
   * @param {number} month
   * @param {number} tithi
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} elevation
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @returns {import("./hyper-calendar.d.ts").VaishnavaDay}
   */
  vaishnavaDay(sakaYear, month, tithi, latitude, longitude, elevation, ayanamsa) {
    const fn = this.#export("hc_vaishnava_day");
    const year = toI64(sakaYear, "sakaYear");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(ayanamsa, "ayanamsa", (pointer, len) =>
      this.#text("hc_vaishnava_day", (buffer, capacity) =>
        fn(year, toU32(month, "month"), toU32(tithi, "tithi"), lat, lon, elev, pointer, len, buffer, capacity), true));
    const [saka, monthCell, tithiCell, fixed, gy, gm, gd, sunriseTithi, ayanamsaId] =
      this.#oneLine("hc_vaishnava_day", text, COLUMNS.vaishnavaDay);
    return {
      sakaYear: integer(saka, "saka year"),
      month: integer(monthCell, "month"),
      tithi: integer(tithiCell, "tithi"),
      fixed: integer(fixed, "fixed"),
      gregorian: [integer(gy, "gregorian year"), integer(gm, "gregorian month"), integer(gd, "gregorian day")],
      sunriseTithi: integer(sunriseTithi, "sunrise tithi"),
      ayanamsa: /** @type {import("./hyper-calendar.d.ts").Ayanamsa} */ (ayanamsaId),
    };
  }

  /**
   * The part of a tithi that Bhadra, the karaṇa Viṣṭi, does not cover, for
   * a tithi of an amānta month of a Śaka year at a place: the second half of
   * the full moon's tithi, which Rakṣā Bandhana waits for. `begins` and
   * `ends` are `null` for a tithi Viṣṭi never falls on. Errors as
   * {@link vaishnavaDay}.
   *
   * @param {number | bigint} sakaYear
   * @param {number} month
   * @param {number} tithi
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} elevation
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @returns {import("./hyper-calendar.d.ts").VishtiFreeSpan}
   */
  vishtiFreeSpan(sakaYear, month, tithi, latitude, longitude, elevation, ayanamsa) {
    const fn = this.#export("hc_vishti_free_span");
    const year = toI64(sakaYear, "sakaYear");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(ayanamsa, "ayanamsa", (pointer, len) =>
      this.#text("hc_vishti_free_span", (buffer, capacity) =>
        fn(year, toU32(month, "month"), toU32(tithi, "tithi"), lat, lon, elev, pointer, len, buffer, capacity), true));
    const [saka, monthCell, tithiCell, first, begins, ends, ayanamsaId] =
      this.#oneLine("hc_vishti_free_span", text, COLUMNS.vishtiFreeSpan);
    return {
      sakaYear: integer(saka, "saka year"),
      month: integer(monthCell, "month"),
      tithi: integer(tithiCell, "tithi"),
      monthFirstDay: integer(first, "month first day"),
      begins: optionalInteger(begins, "begins"),
      ends: optionalInteger(ends, "ends"),
      ayanamsa: /** @type {import("./hyper-calendar.d.ts").Ayanamsa} */ (ayanamsaId),
    };
  }

  /**
   * Rāhu, the mean ascending node, and Ketu opposite it at a POSIX instant,
   * in the zodiac of an ayanāṃśa; an instant outside −1000 to 3000 is
   * `out-of-range`.
   *
   * @param {number | bigint} unixSeconds
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @returns {import("./hyper-calendar.d.ts").NodePlace}
   */
  rahuAt(unixSeconds, ayanamsa) {
    const fn = this.#export("hc_rahu_at");
    const instant = toI64(unixSeconds, "unixSeconds");
    const text = this.#withText(ayanamsa, "ayanamsa", (pointer, len) =>
      this.#text("hc_rahu_at", (buffer, capacity) => fn(instant, pointer, len, buffer, capacity), true));
    const [node, rahu, rahuNumber, rahuSign, rahuSignName, ketu, ketuNumber, ketuSign, ketuSignName, ayanamsaId, readAt] =
      this.#oneLine("hc_rahu_at", text, COLUMNS.rahuAt);
    return {
      node: /** @type {"mean"} */ (node),
      rahu: {
        longitude: decimal(rahu, "rahu longitude"),
        number: integer(rahuNumber, "rahu number"),
        sign: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (rahuSign),
        signName: rahuSignName,
      },
      ketu: {
        longitude: decimal(ketu, "ketu longitude"),
        number: integer(ketuNumber, "ketu number"),
        sign: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (ketuSign),
        signName: ketuSignName,
      },
      ayanamsa: /** @type {import("./hyper-calendar.d.ts").Ayanamsa} */ (ayanamsaId),
      readAt: integer(readAt, "read at"),
    };
  }

  /**
   * The entries of the mean node into the sidereal signs in
   * `[fromUnixSeconds, toUnixSeconds)`, in time order: Rāhu moves backward,
   * a sign in about 566 days. A span longer than a hundred Julian years or
   * an end outside −1000 to 3000 is `out-of-range`.
   *
   * @param {number | bigint} fromUnixSeconds
   * @param {number | bigint} toUnixSeconds
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @returns {import("./hyper-calendar.d.ts").NodeIngress[]}
   */
  rahuIngresses(fromUnixSeconds, toUnixSeconds, ayanamsa) {
    const fn = this.#export("hc_rahu_ingresses");
    const from = toI64(fromUnixSeconds, "fromUnixSeconds");
    const to = toI64(toUnixSeconds, "toUnixSeconds");
    const text = this.#withText(ayanamsa, "ayanamsa", (pointer, len) =>
      this.#text("hc_rahu_ingresses", (buffer, capacity) => fn(from, to, pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.rahuIngress, "hc_rahu_ingresses").map(
      ([moment, from, fromName, into, intoName, ketuInto, ketuIntoName]) => ({
        moment: integer(moment, "moment"),
        from: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (from),
        fromName,
        into: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (into),
        intoName,
        ketuInto: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (ketuInto),
        ketuIntoName,
      }));
  }

  /**
   * The first day of a year of a historical Indian era over the lunisolar
   * months, as a fixed day: `calendar` is `vikram-samvat-kartikadi`,
   * `rajyabhisheka-saka`, `saptarshi`, `gupta`, `valabhi`, `kalachuri` or
   * `lakshmana-sena`. The day is the era's own: a year opens at Kārttika
   * śukla 1, Āśvina śukla 1 or Jyeṣṭha śukla 13 as its source states.
   *
   * @param {import("./hyper-calendar.d.ts").LunarEraId} calendar
   * @param {number | bigint} year
   * @returns {number}
   */
  eraNewYear(calendar, year) {
    const fn = this.#export("hc_era_new_year");
    const y = toI64(year, "year");
    return this.#withText(calendar, "calendar", (pointer, len) =>
      toNumber(fn(pointer, len, y), "hc_era_new_year"));
  }

  /**
   * The value at a POSIX instant of an ayanāṃśa the caller anchors:
   * `degreesAtAnchor` degrees at the Julian date `anchorJulianDate`, carried
   * by precession as the named ones are.
   *
   * @param {number | bigint} unixSeconds
   * @param {number} anchorJulianDate
   * @param {number} degreesAtAnchor
   * @returns {import("./hyper-calendar.d.ts").AyanamsaValue}
   */
  ayanamsaFromAnchor(unixSeconds, anchorJulianDate, degreesAtAnchor) {
    const fn = this.#export("hc_ayanamsa_from_anchor");
    const instant = toI64(unixSeconds, "unixSeconds");
    const text = this.#text("hc_ayanamsa_from_anchor", (buffer, capacity) =>
      fn(instant, toF64(anchorJulianDate, "anchorJulianDate"), toF64(degreesAtAnchor, "degreesAtAnchor"), buffer, capacity), true);
    return ayanamsaValue(this.#oneLine("hc_ayanamsa_from_anchor", text, COLUMNS.ayanamsaValue));
  }

  /**
   * The thirty muhūrtas of a day at a place, the daylight's fifteen and
   * the night's, with Abhijit and Dur Muhurtam marked as a pañcāṅga
   * prints them.
   *
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} elevation
   * @returns {import("./hyper-calendar.d.ts").Muhurta[]}
   */
  muhurtas(fixed, latitude, longitude, elevation) {
    const fn = this.#export("hc_muhurtas");
    const day = toI64(fixed, "fixed");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#text("hc_muhurtas", (buffer, capacity) => fn(day, lat, lon, elev, buffer, capacity), true);
    return rows(text, COLUMNS.muhurtas, "hc_muhurtas").map(muhurtaLine);
  }

  /**
   * The *amṛta siddhi yoga* of a day at a place: the part of the day the
   * Moon spends in the nakṣatra its weekday pairs with, if any.
   *
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} elevation
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @returns {import("./hyper-calendar.d.ts").AmritaSiddhi}
   */
  amritaSiddhi(fixed, latitude, longitude, elevation, ayanamsa) {
    const fn = this.#export("hc_amrita_siddhi");
    const day = toI64(fixed, "fixed");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(ayanamsa, "ayanamsa", (pointer, len) =>
      this.#text("hc_amrita_siddhi", (buffer, capacity) => fn(day, lat, lon, elev, pointer, len, buffer, capacity), true));
    return amritaSiddhiLine(this.#oneLine("hc_amrita_siddhi", text, COLUMNS.amritaSiddhi));
  }

  /**
   * The nakṣatra the Moon is in at a POSIX instant, in an ayanamsa's
   * zodiac, with when it entered and when it leaves.
   *
   * @param {number | bigint} unixSeconds
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @returns {import("./hyper-calendar.d.ts").NakshatraStay}
   */
  nakshatraAt(unixSeconds, ayanamsa) {
    const fn = this.#export("hc_nakshatra_at");
    const instant = toI64(unixSeconds, "unixSeconds");
    const text = this.#withText(ayanamsa, "ayanamsa", (pointer, len) =>
      this.#text("hc_nakshatra_at", (buffer, capacity) => fn(instant, pointer, len, buffer, capacity), true));
    return nakshatraLine(this.#oneLine("hc_nakshatra_at", text, COLUMNS.nakshatra));
  }

  /**
   * The nakṣatra a day carries at a place, read at its sunrise; a day
   * without one there is `no-data`.
   *
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} elevation
   * @param {import("./hyper-calendar.d.ts").Ayanamsa} ayanamsa
   * @returns {import("./hyper-calendar.d.ts").NakshatraStay}
   */
  nakshatraOfDay(fixed, latitude, longitude, elevation, ayanamsa) {
    const fn = this.#export("hc_nakshatra_of_day");
    const day = toI64(fixed, "fixed");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(ayanamsa, "ayanamsa", (pointer, len) =>
      this.#text("hc_nakshatra_of_day", (buffer, capacity) =>
        fn(day, lat, lon, elev, pointer, len, buffer, capacity), true));
    return nakshatraLine(this.#oneLine("hc_nakshatra_of_day", text, COLUMNS.nakshatra));
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
    const [position, name, localeUsed, twelve, twelveName, meanSign, meanSignName] =
      this.#oneLine("hc_barhaspatya_year_at", text, COLUMNS.barhaspatyaYearAt);
    return {
      position: integer(position, "position"),
      name,
      localeUsed,
      twelveYear: integer(twelve, "twelve-year position"),
      twelveYearName: twelveName,
      meanSign: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (meanSign),
      meanSignName,
    };
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
   * The modern Olympiad a fixed day belongs to, by the Charter in force
   * on it: before September 2004 from one opening to the next, then by
   * the Gregorian year. A day of June to November 1956 is `no-data`.
   *
   * @param {number | bigint} fixed
   * @returns {number}
   */
  iocOlympiadOn(fixed) {
    return toNumber(this.#export("hc_ioc_olympiad_on")(toI64(fixed, "fixed")), "hc_ioc_olympiad_on");
  }

  /**
   * The king and regnal year labelling a Seleucid year, SE −314 to 160.
   *
   * @param {number | bigint} seleucidYear
   * @returns {import("./hyper-calendar.d.ts").RegnalYear}
   */
  babylonianRegnalYear(seleucidYear) {
    const fn = this.#export("hc_babylonian_regnal_year");
    const year = toI64(seleucidYear, "seleucidYear");
    const text = this.#text("hc_babylonian_regnal_year", (buffer, capacity) => fn(year, buffer, capacity), true);
    const [king, regnal] = this.#oneLine("hc_babylonian_regnal_year", text, COLUMNS.babylonianRegnalYear);
    return { king, year: integer(regnal, "regnal year") };
  }

  /**
   * How far the equinox that begins a year fell from the moment that
   * decides the new year of `persian`, `persian-apparent-noon`, `jalali`,
   * `bahai-astronomical` or `french-republican-equinox`, in minutes.
   *
   * @param {import("./hyper-calendar.d.ts").EquinoxCalendar} calendar
   * @param {number | bigint} year
   * @returns {import("./hyper-calendar.d.ts").EquinoxMargin}
   */
  equinoxNewYearMargin(calendar, year) {
    const fn = this.#export("hc_equinox_new_year_margin");
    const y = toI64(year, "year");
    const text = this.#withText(calendar, "calendar", (pointer, len) =>
      this.#text("hc_equinox_new_year_margin", (buffer, capacity) => fn(pointer, len, y, buffer, capacity), true));
    const [minutes, id] = this.#oneLine("hc_equinox_new_year_margin", text, COLUMNS.equinoxMargin);
    return {
      minutes: decimal(minutes, "minutes"),
      calendar: /** @type {import("./hyper-calendar.d.ts").EquinoxCalendar} */ (id),
    };
  }

  /**
   * A *tekufah* of Shmuel's reckoning in a Hebrew year: `tishrei`,
   * `tevet`, `nisan` or `tammuz`.
   *
   * @param {number | bigint} hebrewYear
   * @param {import("./hyper-calendar.d.ts").Tekufah} tekufah
   * @returns {import("./hyper-calendar.d.ts").ShmuelTekufah}
   */
  shmuelTekufah(hebrewYear, tekufah) {
    const fn = this.#export("hc_shmuel_tekufah");
    const year = toI64(hebrewYear, "hebrewYear");
    const text = this.#withText(tekufah, "tekufah", (pointer, len) =>
      this.#text("hc_shmuel_tekufah", (buffer, capacity) => fn(year, pointer, len, buffer, capacity), true));
    const [fixed, minutes, id, afterNightfall, civil] = this.#oneLine("hc_shmuel_tekufah", text, COLUMNS.shmuelTekufah);
    return {
      fixed: integer(fixed, "fixed"),
      minutes: integer(minutes, "minutes"),
      tekufah: /** @type {import("./hyper-calendar.d.ts").Tekufah} */ (id),
      afterNightfall: flag(afterNightfall, "after nightfall"),
      civil: integer(civil, "civil"),
    };
  }

  /**
   * A fixed day's name in a calendar whose days are named, by one of its
   * namings: the French Republican `fr-fabre-1793`, `fr` and `en`, the
   * Armenian `hy` and `hy-Latn`.
   *
   * @param {string} calendar
   * @param {string} naming
   * @param {number | bigint} fixed
   * @returns {import("./hyper-calendar.d.ts").DayName}
   */
  dayName(calendar, naming, fixed) {
    const fn = this.#export("hc_day_name");
    const day = toI64(fixed, "fixed");
    const text = this.#withText(calendar, "calendar", (calendarPointer, calendarLen) =>
      this.#withText(naming, "naming", (namingPointer, namingLen) =>
        this.#text("hc_day_name", (buffer, capacity) =>
          fn(calendarPointer, calendarLen, namingPointer, namingLen, day, buffer, capacity), true)));
    const [name, namingId, namingName, authority] = this.#oneLine("hc_day_name", text, COLUMNS.dayName);
    return { name, naming: namingId, namingName, authority };
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
   * A person's age on a day by a count: `chinese-age`, one at birth and
   * one more each Chinese New Year; `lichun-age`, one more each 立春;
   * `new-year-day-age`, one more each 1 January; or `year-age`, from
   * nothing, one more each 1 January.
   *
   * @param {import("./hyper-calendar.d.ts").AgeConvention} convention
   * @param {number | bigint} birthFixed
   * @param {number | bigint} onFixed
   * @returns {number}
   */
  chineseAge(convention, birthFixed, onFixed) {
    const fn = this.#export("hc_chinese_age");
    const birth = toI64(birthFixed, "birthFixed");
    const on = toI64(onFixed, "onFixed");
    return this.#withText(convention, "convention", (pointer, len) =>
      toNumber(fn(pointer, len, birth, on), "hc_chinese_age"));
  }

  /**
   * The twenty-four solar terms the Qing almanac printed in a Gregorian
   * year of 1645–1733, 小寒 first.
   *
   * @param {number | bigint} year
   * @returns {import("./hyper-calendar.d.ts").AlmanacSolarTerm[]}
   */
  chineseAlmanacSolarTerms(year) {
    const fn = this.#export("hc_chinese_almanac_solar_terms");
    const y = toI64(year, "year");
    const text = this.#text("hc_chinese_almanac_solar_terms", (buffer, capacity) => fn(y, buffer, capacity), true);
    return rows(text, COLUMNS.chineseAlmanacSolarTerms, "hc_chinese_almanac_solar_terms").map(
      ([position, name, fixed]) => ({ position: integer(position, "position"), name, fixed: integer(fixed, "fixed") }));
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
   * `regionGroups` are the subdivision and group pairs a rule is scoped to
   * both of, and `readSubdivisions` the subdivisions the sources were read
   * for, which a region outside it is a gap for.
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
   * Every group of people a holiday may be given to alone, named in a
   * locale where an instrument in the language names it.
   *
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").HolidayGroup[]}
   */
  holidayGroups(locale = "und") {
    const fn = this.#export("hc_holiday_groups");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_holiday_groups", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.holidayGroups, "hc_holiday_groups").map(([group, name, localeUsed, englishName]) => ({
      group,
      name: optional(name),
      localeUsed: optional(localeUsed),
      englishName,
    }));
  }

  /**
   * The years a holiday table answers for, nationwide and in each
   * subdivision it answers for: the first year any rule is read for, the
   * first year from which none is a gap for want of reading, the last year
   * no announced list has run out in, whether every rule is read in some
   * year, the first year the weekend law is read, and the rule that sets the
   * later start. A year before the first is a gap that `holidaysInYear`
   * writes.
   *
   * @param {string} code
   * @returns {import("./hyper-calendar.d.ts").HolidayCoverage[]}
   */
  holidayCoverage(code) {
    const fn = this.#export("hc_holiday_coverage");
    const text = this.#withText(code, "code", (pointer, len) =>
      this.#text("hc_holiday_coverage", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.holidayCoverage, "hc_holiday_coverage").map(
      ([region, firstRead, answeredFrom, answeredUntil, complete, weekendFrom, reason]) => ({
        region: optional(region),
        firstRead: optionalInteger(firstRead, "first read"),
        answeredFrom: optionalInteger(answeredFrom, "answered from"),
        answeredUntil: optionalInteger(answeredUntil, "answered until"),
        complete: flag(complete, "complete"),
        weekendFrom: optionalInteger(weekendFrom, "weekend from"),
        reason: optional(reason),
      }),
    );
  }

  /**
   * `holidaysOn`'s entries, each with the day's name in a locale where a
   * source in the language names it.
   *
   * @param {number | bigint} fixed
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").HolidayOnIn[]}
   */
  holidaysOnIn(fixed, locale = "und") {
    const fn = this.#export("hc_holidays_on_in");
    const day = toI64(fixed, "fixed");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_holidays_on_in", (buffer, capacity) => fn(day, pointer, len, buffer, capacity), true));
    // The eleven columns of `hc_holidays_on` before its identifier, the two
    // names, then the identifier, which keeps the names in their places, and
    // the bridge flag, which `hc_holidays_on` ends with.
    const before = COLUMNS.holidaysOn.length - 2;
    const columns = [...COLUMNS.holidaysOn.slice(0, before), "name in locale", "name locale", "id", "bridged"];
    return rows(text, columns, "hc_holidays_on_in").map((cells) => ({
      ...holidayOn([...cells.slice(0, before), cells[before + 2], cells[before + 3]]),
      nameInLocale: optional(cells[before]),
      nameLocale: optional(cells[before + 1]),
    }));
  }

  /**
   * The lectionary cycles of a fixed day: the liturgical year, the Sunday
   * cycle, the Roman weekday cycle, the RCL Proper of a Sunday after
   * Trinity Sunday, and the Roman Sunday and weeks in Ordinary Time.
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
   * What the Roman calendar of the 1960 rubrics does on a day: the office
   * kept, its commemorations, and the feasts transferred or omitted.
   *
   * @param {number | bigint} fixed
   * @returns {import("./hyper-calendar.d.ts").Roman1960Office[]}
   */
  roman1960OfficeOn(fixed) {
    const fn = this.#export("hc_roman_1960_office_on");
    const day = toI64(fixed, "fixed");
    const text = this.#text("hc_roman_1960_office_on", (buffer, capacity) => fn(day, buffer, capacity), true);
    return rows(text, COLUMNS.roman1960Office, "hc_roman_1960_office_on").map(([role, title, cls, className, from]) => ({
      role: /** @type {import("./hyper-calendar.d.ts").Roman1960Role} */ (role),
      title,
      class: /** @type {import("./hyper-calendar.d.ts").Roman1960Class} */ (cls),
      className,
      transferredFrom: optionalInteger(from, "transferred from"),
    }));
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
   * Every territory CLDR 48 names, in code order — the countries, the UN
   * M.49 areas such as `001`, and CLDR's `EU`, `UN`, `ZZ` and the like —
   * with its name in the locale, else in English, the tag that answered,
   * the draft level of that value and the code's validity status.
   *
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").PlaceName[]}
   */
  territories(locale = "und") {
    const fn = this.#export("hc_territories");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_territories", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.places, "hc_territories").map(placeName);
  }

  /**
   * The ISO 3166-2 subdivisions of a country CLDR 48 names, in code order,
   * as {@link territories} describes them: `JP-13` is 東京都 under `ja`.
   * An empty `country` lists every subdivision; one that is not a
   * territory's code is `unknown`.
   *
   * @param {string} country
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").PlaceName[]}
   */
  subdivisions(country, locale = "und") {
    const fn = this.#export("hc_subdivisions");
    const text = this.#withText(country, "country", (countryPointer, countryLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#text("hc_subdivisions", (buffer, capacity) =>
          fn(countryPointer, countryLen, localePointer, localeLen, buffer, capacity), true)));
    return rows(text, COLUMNS.places, "hc_subdivisions").map(placeName);
  }

  /**
   * One territory, `JP`, or subdivision in ISO form, `JP-13`, as
   * {@link territories} and {@link subdivisions} describe it; another code,
   * CLDR's own `jp13` among them, is `unknown`.
   *
   * @param {string} code
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").PlaceName}
   */
  placeName(code, locale = "und") {
    const fn = this.#export("hc_place_name");
    const text = this.#withText(code, "code", (codePointer, codeLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#text("hc_place_name", (buffer, capacity) =>
          fn(codePointer, codeLen, localePointer, localeLen, buffer, capacity), true)));
    return placeName(this.#oneLine("hc_place_name", text, COLUMNS.places));
  }

  /**
   * How one instant reads from another in a locale, *3 hours ago* or *in
   * 2 days*: the phrase, the unit the conversational thresholds count it
   * in and the signed count, truncated. `style` is `long`, `short` or
   * `narrow`; `automatic` writes the language's own word for an offset
   * where it has one, *yesterday*, as `Intl.RelativeTimeFormat`'s
   * `numeric: "auto"` does. The root locale `und` writes CLDR's root
   * phrases, `-1 d`.
   *
   * @param {number | bigint} thenUnix
   * @param {number | bigint} nowUnix
   * @param {import("./hyper-calendar.d.ts").RelativeStyle} [style]
   * @param {boolean} [automatic]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").RelativeTime}
   */
  relativeTime(thenUnix, nowUnix, style = "long", automatic = false, locale = "und") {
    const fn = this.#export("hc_relative_time");
    const then = toI64(thenUnix, "thenUnix");
    const now = toI64(nowUnix, "nowUnix");
    const text = this.#withText(style, "style", (stylePointer, styleLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#text("hc_relative_time", (buffer, capacity) =>
          fn(then, now, stylePointer, styleLen, automatic ? 1 : 0, localePointer, localeLen, buffer, capacity), true)));
    return relativeTime(this.#oneLine("hc_relative_time", text, COLUMNS.relativeTime));
  }

  /**
   * Which calendar day one fixed day is, seen from another, in a locale:
   * *3 days ago*, and with `automatic` *yesterday* and *today*. The
   * offset is the difference of the day numbers, never a span divided.
   * `style`, `automatic` and `locale` are as for {@link relativeTime}.
   *
   * @param {number | bigint} thenFixed
   * @param {number | bigint} nowFixed
   * @param {import("./hyper-calendar.d.ts").RelativeStyle} [style]
   * @param {boolean} [automatic]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").RelativeTime}
   */
  relativeDay(thenFixed, nowFixed, style = "long", automatic = false, locale = "und") {
    const fn = this.#export("hc_relative_day");
    const then = toI64(thenFixed, "thenFixed");
    const now = toI64(nowFixed, "nowFixed");
    const text = this.#withText(style, "style", (stylePointer, styleLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#text("hc_relative_day", (buffer, capacity) =>
          fn(then, now, stylePointer, styleLen, automatic ? 1 : 0, localePointer, localeLen, buffer, capacity), true)));
    return relativeTime(this.#oneLine("hc_relative_day", text, COLUMNS.relativeTime));
  }

  /**
   * {@link relativeDay}'s phrase with a time of day, *yesterday at 15:05*,
   * the time `secondsOfDay` after midnight written `H:MM` in the locale's
   * digits and joined by the locale's pattern.
   *
   * @param {number | bigint} thenFixed
   * @param {number | bigint} nowFixed
   * @param {number} secondsOfDay
   * @param {import("./hyper-calendar.d.ts").RelativeStyle} [style]
   * @param {boolean} [automatic]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").RelativeDayAt}
   */
  relativeDayAt(thenFixed, nowFixed, secondsOfDay, style = "long", automatic = false, locale = "und") {
    const fn = this.#export("hc_relative_day_at");
    const then = toI64(thenFixed, "thenFixed");
    const now = toI64(nowFixed, "nowFixed");
    const seconds = toU32(secondsOfDay, "secondsOfDay");
    const text = this.#withText(style, "style", (stylePointer, styleLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#text("hc_relative_day_at", (buffer, capacity) =>
          fn(then, now, seconds, stylePointer, styleLen, automatic ? 1 : 0, localePointer, localeLen, buffer, capacity),
          true)));
    return relativeDayAt(this.#oneLine("hc_relative_day_at", text, COLUMNS.relativeDayAt));
  }

  /**
   * A span of seconds phrased in days, hours, minutes and seconds, *2
   * hours and 30 minutes*: `style` is `long`, `short`, `narrow` or
   * `compact` (*2h30m*), and at most `maxComponents` units are written,
   * the largest first, `0` for every one.
   *
   * @param {number | bigint} seconds
   * @param {import("./hyper-calendar.d.ts").DurationStyle} [style]
   * @param {number} [maxComponents]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").HumanizedDuration}
   */
  duration(seconds, style = "long", maxComponents = 0, locale = "und") {
    const fn = this.#export("hc_duration");
    const span = toI64(seconds, "seconds");
    const most = toU32(maxComponents, "maxComponents");
    const text = this.#withText(style, "style", (stylePointer, styleLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#text("hc_duration", (buffer, capacity) =>
          fn(span, stylePointer, styleLen, most, localePointer, localeLen, buffer, capacity), true)));
    return humanizedDuration(this.#oneLine("hc_duration", text, COLUMNS.duration));
  }

  /**
   * A whole number as the Associated Press writes it, `humanize`'s
   * `apnumber`: *zero* to *nine* spelled out, every other number as its
   * digits, in the catalogue of the `locale`'s fallback chain that
   * translates the numerals, English where none does; `localeUsed` says
   * which.
   *
   * @param {number | bigint} value
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").LocalizedNaturalText}
   */
  apnumber(value, locale = "und") {
    const text = this.#call("hc_apnumber", [["i64", "value", value], ["str", "locale", locale]]);
    return localizedNaturalText(this.#oneLine("hc_apnumber", text, COLUMNS.localizedNaturalText));
  }

  /**
   * A number as a fraction, `humanize`'s `fractional`: `0.3` is *3/10*,
   * `1.3` is *1 3/10*, by the nearest fraction with a denominator of at
   * most 1000. It writes no word, so it has no locale.
   *
   * @param {number} value
   * @returns {import("./hyper-calendar.d.ts").NaturalText}
   */
  fractional(value) {
    const fn = this.#export("hc_fractional");
    const v = toF64(value, "value");
    const text = this.#text("hc_fractional", (buffer, capacity) => fn(v, buffer, capacity), true);
    return naturalText(this.#oneLine("hc_fractional", text, COLUMNS.naturalText));
  }

  /**
   * A number in scientific notation, `humanize`'s `scientific`: *3.00 x
   * 10⁻¹*, with `precision` digits after the point. It has no locale.
   *
   * @param {number} value
   * @param {number} [precision]
   * @returns {import("./hyper-calendar.d.ts").NaturalText}
   */
  scientific(value, precision = 2) {
    const fn = this.#export("hc_scientific");
    const v = toF64(value, "value");
    const digits = toU32(precision, "precision");
    const text = this.#text("hc_scientific", (buffer, capacity) => fn(v, digits, buffer, capacity), true);
    return naturalText(this.#oneLine("hc_scientific", text, COLUMNS.naturalText));
  }

  /**
   * A number with an SI prefix and a unit, `humanize`'s `metric`: *1.50
   * kV*, *220 μF*, with `precision` significant digits. The prefixes are
   * symbols no catalogue translates; the `locale` chooses the catalogue
   * whose decimal mark the scientific form of a magnitude beyond them uses.
   *
   * @param {number} value
   * @param {string} [unit]
   * @param {number} [precision]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").LocalizedNaturalText}
   */
  metric(value, unit = "", precision = 3, locale = "und") {
    const text = this.#call("hc_metric", [
      ["f64", "value", value], ["str", "unit", unit], ["u32", "precision", precision], ["str", "locale", locale],
    ]);
    return localizedNaturalText(this.#oneLine("hc_metric", text, COLUMNS.localizedNaturalText));
  }

  /**
   * A size in bytes, `humanize`'s `naturalsize`: *3.0 MB* (`decimal`),
   * *2.9 KiB* (`binary`), *2.9K* (`gnu`), with *Byte* and the suffixes of
   * the catalogue that translates them, English where none does.
   *
   * @param {number} value
   * @param {import("./hyper-calendar.d.ts").NaturalSizeStyle} [style]
   * @param {number} [decimals]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").LocalizedNaturalText}
   */
  naturalSize(value, style = "decimal", decimals = 1, locale = "und") {
    const text = this.#call("hc_naturalsize", [
      ["f64", "value", value], ["str", "style", style], ["u32", "decimals", decimals], ["str", "locale", locale],
    ]);
    return localizedNaturalText(this.#oneLine("hc_naturalsize", text, COLUMNS.localizedNaturalText));
  }

  /**
   * Items joined as a list, `humanize`'s `natural_list`: *one, two and
   * three*, with no comma before the *and*. Its `, ` and ` and ` are
   * literals in `humanize` that no catalogue translates, so every `locale`
   * gets English and `localeUsed` is `en`.
   *
   * @param {string[]} items
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").LocalizedNaturalText}
   */
  naturalList(items, locale = "und") {
    const text = this.#call("hc_naturallist", [["str", "items", items.join("\n")], ["str", "locale", locale]]);
    return localizedNaturalText(this.#oneLine("hc_naturallist", text, COLUMNS.localizedNaturalText));
  }

  /**
   * An integer of any length as a count with a word, `humanize`'s
   * `intword`: *12.4 thousand*, *1.2 billion*, *1.0 googol*, in the
   * catalogue that translates the words. `digits` is an integer written in
   * digits, or a `bigint`.
   *
   * @param {string | number | bigint} digits
   * @param {number} [decimals]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").LocalizedNaturalText}
   */
  intword(digits, decimals = 1, locale = "und") {
    const text = this.#call("hc_intword", [
      ["str", "digits", String(digits)], ["u32", "decimals", decimals], ["str", "locale", locale],
    ]);
    return localizedNaturalText(this.#oneLine("hc_intword", text, COLUMNS.localizedNaturalText));
  }

  /**
   * `humanize`'s `naturaldelta` of a span of `seconds` and `microseconds`:
   * *3 hours*, *a moment*, without tense and ignoring the sign. `months`
   * uses months of 30.5 days between days and years; `minimumUnit` is
   * `seconds`, `milliseconds` or `microseconds`.
   *
   * @param {number | bigint} seconds
   * @param {number} [microseconds]
   * @param {boolean} [months]
   * @param {import("./hyper-calendar.d.ts").DeltaUnit} [minimumUnit]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").LocalizedNaturalText}
   */
  naturalDelta(seconds, microseconds = 0, months = true, minimumUnit = "seconds", locale = "und") {
    const text = this.#call("hc_naturaldelta", [
      ["i64", "seconds", seconds], ["i32", "microseconds", microseconds], ["flag", "months", months],
      ["str", "minimumUnit", minimumUnit], ["str", "locale", locale],
    ]);
    return localizedNaturalText(this.#oneLine("hc_naturaldelta", text, COLUMNS.localizedNaturalText));
  }

  /**
   * `humanize`'s `naturaltime` of a span: *3 hours ago*, *3 hours from
   * now*, *now*. A positive span is in the past, as in Python. The rest is
   * as for {@link naturalDelta}.
   *
   * @param {number | bigint} seconds
   * @param {number} [microseconds]
   * @param {boolean} [months]
   * @param {import("./hyper-calendar.d.ts").DeltaUnit} [minimumUnit]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").LocalizedNaturalText}
   */
  naturalTime(seconds, microseconds = 0, months = true, minimumUnit = "seconds", locale = "und") {
    const text = this.#call("hc_naturaltime", [
      ["i64", "seconds", seconds], ["i32", "microseconds", microseconds], ["flag", "months", months],
      ["str", "minimumUnit", minimumUnit], ["str", "locale", locale],
    ]);
    return localizedNaturalText(this.#oneLine("hc_naturaltime", text, COLUMNS.localizedNaturalText));
  }

  /**
   * `humanize`'s `precisedelta` of a span, *1 year, 2 months and 3 days*,
   * with 4.16.0's arithmetic step for step. `minimumUnit` is the smallest
   * unit written, `suppress` the units folded into the next smaller, and
   * `decimals` the places of the fraction of the smallest unit.
   *
   * @param {number | bigint} seconds
   * @param {number} [microseconds]
   * @param {import("./hyper-calendar.d.ts").PreciseUnitName} [minimumUnit]
   * @param {import("./hyper-calendar.d.ts").PreciseUnitName[]} [suppress]
   * @param {number} [decimals]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").LocalizedNaturalText}
   */
  preciseDelta(seconds, microseconds = 0, minimumUnit = "seconds", suppress = [], decimals = 2, locale = "und") {
    const text = this.#call("hc_precisedelta", [
      ["i64", "seconds", seconds], ["i32", "microseconds", microseconds], ["str", "minimumUnit", minimumUnit],
      ["str", "suppress", suppress.join(",")], ["u32", "decimals", decimals], ["str", "locale", locale],
    ]);
    return localizedNaturalText(this.#oneLine("hc_precisedelta", text, COLUMNS.localizedNaturalText));
  }

  /**
   * `humanize`'s `naturalday` of a fixed day seen from another: *today*,
   * *tomorrow*, *yesterday*, or the day by a `strftime` pattern in the C
   * locale, `%b %d` when `pattern` is empty.
   *
   * @param {number | bigint} day
   * @param {number | bigint} today
   * @param {string} [pattern]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").LocalizedNaturalText}
   */
  naturalDay(day, today, pattern = "", locale = "und") {
    const text = this.#call("hc_naturalday", [
      ["i64", "day", day], ["i64", "today", today], ["str", "pattern", pattern], ["str", "locale", locale],
    ]);
    return localizedNaturalText(this.#oneLine("hc_naturalday", text, COLUMNS.localizedNaturalText));
  }

  /**
   * `humanize`'s `naturaldate`: {@link naturalDay} with `%b %d`, and the
   * year added from five twelfths of a year away.
   *
   * @param {number | bigint} day
   * @param {number | bigint} today
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").LocalizedNaturalText}
   */
  naturalDate(day, today, locale = "und") {
    const text = this.#call("hc_naturaldate", [["i64", "day", day], ["i64", "today", today], ["str", "locale", locale]]);
    return localizedNaturalText(this.#oneLine("hc_naturaldate", text, COLUMNS.localizedNaturalText));
  }

  /**
   * `humanize`'s `ordinal`: *1st*, *2nd*, *103rd*, *111th*, in the
   * suffixes of the catalogue that translates them; `gender` is `male` or
   * `female`.
   *
   * @param {number | bigint} value
   * @param {import("./hyper-calendar.d.ts").OrdinalGender} [gender]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").LocalizedNaturalText}
   */
  ordinal(value, gender = "male", locale = "und") {
    const text = this.#call("hc_ordinal", [["i64", "value", value], ["str", "gender", gender], ["str", "locale", locale]]);
    return localizedNaturalText(this.#oneLine("hc_ordinal", text, COLUMNS.localizedNaturalText));
  }

  /**
   * `humanize`'s `intcomma` of an integer written in digits, or a
   * `bigint`: *1,234,567*, with the separators of the first catalogue for
   * the `locale` (*1.234.567* for `de`), up to 39 digits.
   *
   * @param {string | number | bigint} digits
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").LocalizedNaturalText}
   */
  intcomma(digits, locale = "und") {
    const text = this.#call("hc_intcomma", [["str", "digits", String(digits)], ["str", "locale", locale]]);
    return localizedNaturalText(this.#oneLine("hc_intcomma", text, COLUMNS.localizedNaturalText));
  }

  /**
   * `humanize`'s `intcomma` of a float: *1,234,567.25*, to `ndigits`
   * places, or, with `null`, as Python's `repr` writes it.
   *
   * @param {number} value
   * @param {number | null} [ndigits]
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").LocalizedNaturalText}
   */
  intcommaFloat(value, ndigits = null, locale = "und") {
    const text = this.#call("hc_intcomma_float", [
      ["f64", "value", value], ["i32", "ndigits", ndigits === null ? -1 : ndigits], ["str", "locale", locale],
    ]);
    return localizedNaturalText(this.#oneLine("hc_intcomma_float", text, COLUMNS.localizedNaturalText));
  }

  /**
   * The unit a span of seconds is said in and its count, under a table of
   * `thresholds` (`default`, `exact`, `with-quarters`) and a `rounding`
   * (`ceil`, `floor`, `nearest`, `truncate`, `nearest-half`).
   *
   * @param {number | bigint} seconds
   * @param {import("./hyper-calendar.d.ts").ThresholdsName} [thresholds]
   * @param {import("./hyper-calendar.d.ts").RoundingName} [rounding]
   * @returns {import("./hyper-calendar.d.ts").UnitChoice}
   */
  unitChoice(seconds, thresholds = "default", rounding = "nearest") {
    const text = this.#call("hc_unit_choice", [
      ["i64", "seconds", seconds], ["str", "thresholds", thresholds], ["str", "rounding", rounding],
    ]);
    return unitChoice(this.#oneLine("hc_unit_choice", text, COLUMNS.unitChoice));
  }

  /**
   * {@link relativeTime} under the `thresholds` and `rounding` of the
   * caller's, where it fixes the conversational table and truncation.
   *
   * @param {number | bigint} thenUnix
   * @param {number | bigint} nowUnix
   * @param {import("./hyper-calendar.d.ts").RelativeStyle} [style]
   * @param {boolean} [automatic]
   * @param {string} [locale]
   * @param {import("./hyper-calendar.d.ts").ThresholdsName} [thresholds]
   * @param {import("./hyper-calendar.d.ts").RoundingName} [rounding]
   * @returns {import("./hyper-calendar.d.ts").RelativeTimeWith}
   */
  relativeTimeWith(thenUnix, nowUnix, style = "long", automatic = false, locale = "und", thresholds = "default", rounding = "truncate") {
    const text = this.#call("hc_relative_time_with", [
      ["i64", "thenUnix", thenUnix], ["i64", "nowUnix", nowUnix], ["str", "style", style],
      ["flag", "automatic", automatic], ["str", "locale", locale], ["str", "thresholds", thresholds],
      ["str", "rounding", rounding],
    ]);
    return relativeTimeWith(this.#oneLine("hc_relative_time_with", text, COLUMNS.relativeTimeWith));
  }

  /**
   * A span of seconds hedged as a round number: *about 3 hours*, *just
   * over a week*, *nearly a year*. The sign is dropped.
   *
   * @param {number | bigint} seconds
   * @param {import("./hyper-calendar.d.ts").RelativeStyle} [style]
   * @param {string} [locale]
   * @param {import("./hyper-calendar.d.ts").ThresholdsName} [thresholds]
   * @param {import("./hyper-calendar.d.ts").HedgePolicy} [policy]
   * @returns {import("./hyper-calendar.d.ts").ApproximateDuration}
   */
  approximateDuration(seconds, style = "long", locale = "und", thresholds = "default", policy = "default") {
    const text = this.#call("hc_approximate_duration", [
      ["i64", "seconds", seconds], ["str", "style", style], ["str", "locale", locale],
      ["str", "thresholds", thresholds], ["str", "policy", policy],
    ]);
    return approximateDuration(this.#oneLine("hc_approximate_duration", text, COLUMNS.approximateDuration));
  }

  /**
   * A date-time read in a syntax: `iso8601`, `iso8601-full`, `rfc3339`,
   * `rfc2822`, `python` or `auto`. The result is a reading, whose
   * `offsetSeconds` and `unixSeconds` are `null` where the text states no
   * zone: a local time is never taken for UTC.
   *
   * @param {import("./hyper-calendar.d.ts").DatetimeSyntax} syntax
   * @param {string} text
   * @returns {import("./hyper-calendar.d.ts").Reading}
   */
  parseDatetime(syntax, text) {
    const answer = this.#call("hc_parse_datetime", [["str", "syntax", syntax], ["str", "text", text]]);
    return reading(this.#oneLine("hc_parse_datetime", answer, COLUMNS.reading));
  }

  /**
   * An instant written as a date-time, in the zone of a numeric offset,
   * in a syntax and to a precision (truncated, never rounded).
   *
   * @param {import("./hyper-calendar.d.ts").DatetimeFormat} syntax
   * @param {number | bigint} unixSeconds
   * @param {number | bigint} [attoseconds] the remainder of the second, 0 to 10^18 - 1
   * @param {number} [offsetSeconds]
   * @param {import("./hyper-calendar.d.ts").DatetimePrecision} [precision]
   * @returns {import("./hyper-calendar.d.ts").FormattedDatetime}
   */
  formatDatetime(syntax, unixSeconds, attoseconds = 0, offsetSeconds = 0, precision = "auto") {
    const answer = this.#call("hc_format_datetime", [
      ["str", "syntax", syntax], ["i64", "unixSeconds", unixSeconds], ["u64", "attoseconds", attoseconds],
      ["i32", "offsetSeconds", offsetSeconds], ["str", "precision", precision],
    ]);
    const [text, written] = this.#oneLine("hc_format_datetime", answer, COLUMNS.formattedDatetime);
    return { text, syntax: /** @type {import("./hyper-calendar.d.ts").DatetimeFormat} */ (written) };
  }

  /**
   * A fixed day written as an ISO 8601 calendar, ordinal or week date,
   * in the extended or the basic style.
   *
   * @param {number | bigint} fixed
   * @param {import("./hyper-calendar.d.ts").IsoDateForm} [form]
   * @param {import("./hyper-calendar.d.ts").IsoDateStyle} [style]
   * @returns {import("./hyper-calendar.d.ts").FormattedIsoDate}
   */
  formatIsoDateAs(fixed, form = "calendar", style = "extended") {
    const answer = this.#call("hc_format_iso_date_as", [
      ["i64", "fixed", fixed], ["str", "form", form], ["str", "style", style],
    ]);
    const [text, writtenForm, writtenStyle] = this.#oneLine("hc_format_iso_date_as", answer, COLUMNS.formattedIsoDate);
    return {
      text,
      form: /** @type {import("./hyper-calendar.d.ts").IsoDateForm} */ (writtenForm),
      style: /** @type {import("./hyper-calendar.d.ts").IsoDateStyle} */ (writtenStyle),
    };
  }

  /**
   * An ISO 8601 date read into its parts, including one that names no day
   * (`2026`, `2026-09`, `2026-W39`), whose `fixed` is `null`.
   *
   * @param {string} text
   * @returns {import("./hyper-calendar.d.ts").IsoDateParts}
   */
  isoDateParts(text) {
    const answer = this.#call("hc_iso_date_parts", [["str", "text", text]]);
    return isoDateParts(this.#oneLine("hc_iso_date_parts", answer, COLUMNS.isoDateParts));
  }

  /**
   * An ISO 8601 duration read into its components, with its exact length
   * where it has one.
   *
   * @param {string} text
   * @returns {import("./hyper-calendar.d.ts").IsoDurationParts}
   */
  isoDuration(text) {
    const answer = this.#call("hc_iso_duration", [["str", "text", text]]);
    return isoDurationParts(this.#oneLine("hc_iso_duration", answer, COLUMNS.isoDuration));
  }

  /**
   * A duration written in ISO 8601 from its components; a component left
   * out, or `undefined`, is absent. `fraction` is the digits of a decimal
   * fraction of the lowest component present.
   *
   * @param {import("./hyper-calendar.d.ts").IsoDurationInput} components
   * @returns {import("./hyper-calendar.d.ts").FormattedIsoDuration}
   */
  formatIsoDuration(components) {
    const part = (/** @type {number | bigint | undefined} */ value) => (value === undefined ? -1 : value);
    const answer = this.#call("hc_format_iso_duration", [
      ["flag", "negative", components.negative ?? false],
      ["i64", "years", part(components.years)], ["i64", "months", part(components.months)],
      ["i64", "weeks", part(components.weeks)], ["i64", "days", part(components.days)],
      ["i64", "hours", part(components.hours)], ["i64", "minutes", part(components.minutes)],
      ["i64", "seconds", part(components.seconds)], ["str", "fraction", components.fraction ?? ""],
    ]);
    return exactLength(this.#oneLine("hc_format_iso_duration", answer, COLUMNS.formattedIsoDuration));
  }

  /**
   * An ISO 8601 interval, or a repeating one, read into its ends and its
   * duration.
   *
   * @param {string} text
   * @returns {import("./hyper-calendar.d.ts").IsoIntervalParts}
   */
  isoInterval(text) {
    const answer = this.#call("hc_iso_interval", [["str", "text", text]]);
    return isoIntervalParts(this.#oneLine("hc_iso_interval", answer, COLUMNS.isoInterval));
  }

  /**
   * A text read against a `strptime` or CLDR pattern, in the C locale's
   * names: the fields the pattern read, and the reading they resolve to,
   * `null` where they name no whole date and time.
   *
   * @param {import("./hyper-calendar.d.ts").PatternSyntax} syntax
   * @param {string} pattern
   * @param {string} text
   * @returns {import("./hyper-calendar.d.ts").PatternFields}
   */
  parsePattern(syntax, pattern, text) {
    const answer = this.#call("hc_parse_pattern", [["str", "syntax", syntax], ["str", "pattern", pattern], ["str", "text", text]]);
    return patternFields(this.#oneLine("hc_parse_pattern", answer, COLUMNS.patternFields));
  }

  /**
   * {@link parsePattern} with the month and weekday names, day periods and
   * eras of a locale besides the C locale's; `python` takes no locale.
   *
   * @param {Exclude<import("./hyper-calendar.d.ts").PatternSyntax, "python">} syntax
   * @param {string} pattern
   * @param {string} text
   * @param {string} locale
   * @returns {import("./hyper-calendar.d.ts").PatternFields}
   */
  parsePatternIn(syntax, pattern, text, locale) {
    const answer = this.#call("hc_parse_pattern_in", [
      ["str", "syntax", syntax], ["str", "pattern", pattern], ["str", "text", text], ["str", "locale", locale],
    ]);
    return patternFields(this.#oneLine("hc_parse_pattern_in", answer, COLUMNS.patternFields));
  }

  /**
   * A time zone's name at a POSIX instant in a locale, as a CLDR pattern
   * field writes it: `z` to `zzzz`, `O`, `OOOO`, `v`, `vvvv`, `V` to
   * `VVVV`.
   *
   * @param {string} zone
   * @param {number | bigint} unixSeconds
   * @param {string} [locale]
   * @param {import("./hyper-calendar.d.ts").ZoneNameField} [field]
   * @returns {import("./hyper-calendar.d.ts").ZoneName}
   */
  zoneName(zone, unixSeconds, locale = "und", field = "zzzz") {
    const fn = this.#export("hc_zone_name");
    const instant = toI64(unixSeconds, "unixSeconds");
    const text = this.#withText(zone, "zone", (zonePointer, zoneLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#withText(field, "field", (fieldPointer, fieldLen) =>
          this.#text("hc_zone_name", (buffer, capacity) =>
            fn(zonePointer, zoneLen, instant, localePointer, localeLen, fieldPointer, fieldLen, buffer, capacity), true))));
    const [name, fieldCell, zoneCell, offset, daylight] = this.#oneLine("hc_zone_name", text, COLUMNS.zoneName);
    return {
      name,
      field: /** @type {import("./hyper-calendar.d.ts").ZoneNameField} */ (fieldCell),
      zone: zoneCell,
      offset: integer(offset, "offset"),
      daylight: flag(daylight, "daylight"),
    };
  }

  /**
   * An instant formatted in a zone and a locale by a `cldr` or a
   * `strftime` pattern.
   *
   * @param {string} zone
   * @param {number | bigint} unixSeconds
   * @param {string} locale
   * @param {"cldr" | "strftime"} syntax
   * @param {string} pattern
   * @returns {import("./hyper-calendar.d.ts").FormattedInZone}
   */
  formatPattern(zone, unixSeconds, locale, syntax, pattern) {
    const fn = this.#export("hc_format_pattern");
    const instant = toI64(unixSeconds, "unixSeconds");
    const text = this.#withText(zone, "zone", (zonePointer, zoneLen) =>
      this.#withText(locale, "locale", (localePointer, localeLen) =>
        this.#withText(syntax, "syntax", (syntaxPointer, syntaxLen) =>
          this.#withText(pattern, "pattern", (patternPointer, patternLen) =>
            this.#text("hc_format_pattern", (buffer, capacity) =>
              fn(zonePointer, zoneLen, instant, localePointer, localeLen, syntaxPointer, syntaxLen, patternPointer,
                patternLen, buffer, capacity), true)))));
    const [formatted, syntaxCell, zoneCell, offset, daylight] = this.#oneLine("hc_format_pattern", text, COLUMNS.zoneName);
    return {
      text: formatted,
      syntax: /** @type {"cldr" | "strftime"} */ (syntaxCell),
      zone: zoneCell,
      offset: integer(offset, "offset"),
      daylight: flag(daylight, "daylight"),
    };
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
   * A binary CCSDS time code read, a Level 2 code from the caller's epoch:
   * a CUC code's TAI instant of count 0, a CDS code's POSIX day of day 0.
   *
   * @param {string} hex
   * @param {import("./hyper-calendar.d.ts").CcsdsEpoch} epoch
   * @param {boolean} [strict]
   * @returns {import("./hyper-calendar.d.ts").CcsdsCode}
   */
  ccsdsDecodeFromEpoch(hex, epoch, strict = false) {
    const fn = this.#export("hc_ccsds_decode_from_epoch");
    const seconds = toI64(epoch.taiSeconds ?? 0, "epoch.taiSeconds");
    const attos = toU64(epoch.attoseconds ?? 0, "epoch.attoseconds");
    const day = toI64(epoch.unixDay ?? 0, "epoch.unixDay");
    const text = this.#withText(hex, "hex", (pointer, len) =>
      this.#text("hc_ccsds_decode_from_epoch", (buffer, capacity) =>
        fn(pointer, len, seconds, attos, day, strict ? 1 : 0, buffer, capacity), true));
    return ccsdsCode(this.#oneLine("hc_ccsds_decode_from_epoch", text, COLUMNS.ccsdsDecode));
  }

  /**
   * The binary CCSDS time code of a TAI instant in a P-field's format, a
   * Level 2 format counted from the caller's epoch.
   *
   * @param {number | bigint} taiSeconds
   * @param {number | bigint} attoseconds
   * @param {string} pField
   * @param {import("./hyper-calendar.d.ts").CcsdsEpoch} epoch
   * @param {boolean} [strict]
   * @returns {string}
   */
  ccsdsEncodeFromEpoch(taiSeconds, attoseconds, pField, epoch, strict = false) {
    const fn = this.#export("hc_ccsds_encode_from_epoch");
    const seconds = toI64(taiSeconds, "taiSeconds");
    const attos = toU64(attoseconds, "attoseconds");
    const epochSeconds = toI64(epoch.taiSeconds ?? 0, "epoch.taiSeconds");
    const epochAttos = toU64(epoch.attoseconds ?? 0, "epoch.attoseconds");
    const day = toI64(epoch.unixDay ?? 0, "epoch.unixDay");
    const text = this.#withText(pField, "pField", (pointer, len) =>
      this.#text("hc_ccsds_encode_from_epoch", (buffer, capacity) =>
        fn(seconds, attos, pointer, len, epochSeconds, epochAttos, day, strict ? 1 : 0, buffer, capacity), true));
    return this.#oneLine("hc_ccsds_encode_from_epoch", text, ["code"])[0];
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
   * JJY's call-sign frame of minute 15 or 45 read in a year, which the
   * frame does not carry, with its notice of a planned stop.
   *
   * @param {string} frame
   * @param {number | bigint} year
   * @returns {import("./hyper-calendar.d.ts").JjyCallSign}
   */
  jjyCallSignDecode(frame, year) {
    const fn = this.#export("hc_jjy_call_sign_decode");
    const y = toI64(year, "year");
    const text = this.#withText(frame, "frame", (pointer, len) =>
      this.#text("hc_jjy_call_sign_decode", (buffer, capacity) => fn(pointer, len, y, buffer, capacity), true));
    return jjyCallSign(this.#oneLine("hc_jjy_call_sign_decode", text, COLUMNS.jjyCallSign));
  }

  /**
   * JJY's call-sign frame for minute 15 or 45 of an hour of JST, with a
   * notice of a planned stop.
   *
   * @param {number | bigint} unixSeconds
   * @param {import("./hyper-calendar.d.ts").JjyStopNotice} [notice]
   * @returns {string}
   */
  jjyCallSignEncode(unixSeconds, notice = {}) {
    const fn = this.#export("hc_jjy_call_sign_encode");
    const minute = toI64(unixSeconds, "unixSeconds");
    const start = toU32(notice.stopStart ?? 0, "stopStart");
    const span = toU32(notice.stopSpan ?? 0, "stopSpan");
    const text = this.#text("hc_jjy_call_sign_encode", (buffer, capacity) =>
      fn(minute, start, notice.daytimeOnly ? 1 : 0, span, buffer, capacity), true);
    return this.#oneLine("hc_jjy_call_sign_encode", text, ["frame"])[0];
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
   * The start of the IRIG frame that holds a reading of the civil clock,
   * the reading `irigEncode` takes, with the frame's length.
   *
   * @param {string} signal
   * @param {number} secondsOfDay
   * @param {number} [hundredths]
   * @returns {import("./hyper-calendar.d.ts").IrigFrameStart}
   */
  irigFrameStart(signal, secondsOfDay, hundredths = 0) {
    const fn = this.#export("hc_irig_frame_start");
    const [seconds, cents] = [toU32(secondsOfDay, "secondsOfDay"), toU32(hundredths, "hundredths")];
    const text = this.#withText(signal, "signal", (pointer, len) =>
      this.#text("hc_irig_frame_start", (buffer, capacity) => fn(pointer, len, seconds, cents, buffer, capacity), true));
    const [start, startHundredths, frame] = this.#oneLine("hc_irig_frame_start", text, COLUMNS.irigFrameStart);
    return {
      secondsOfDay: integer(start, "seconds of day"),
      hundredths: integer(startHundredths, "hundredths"),
      frameMicroseconds: integer(frame, "frame micros"),
    };
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
   * The French Republican decimal time of a time of the civil clock: ten
   * decimal hours to the day, a hundred minutes to the hour and a hundred
   * seconds of 0.864 s to the minute. 23:59:60 is `no-data`.
   *
   * @param {number} secondsOfDay
   * @param {number | bigint} [attoseconds]
   * @returns {import("./hyper-calendar.d.ts").FrenchDecimalTime}
   */
  frenchDecimalTime(secondsOfDay, attoseconds = 0) {
    const fn = this.#export("hc_french_decimal_time");
    const seconds = toU32(secondsOfDay, "secondsOfDay");
    const attos = toU64(attoseconds, "attoseconds");
    const text = this.#text("hc_french_decimal_time", (buffer, capacity) => fn(seconds, attos, buffer, capacity), true);
    const [hour, minute, second, rest] = this.#oneLine("hc_french_decimal_time", text, COLUMNS.frenchDecimalTime);
    return {
      hour: integer(hour, "hour"),
      minute: integer(minute, "minute"),
      second: integer(second, "second"),
      attoseconds: bigInteger(rest, "attoseconds"),
    };
  }

  /**
   * The civil time of day of a French Republican decimal time, exactly.
   *
   * @param {number} hour
   * @param {number} minute
   * @param {number} second
   * @param {number | bigint} [attoseconds]
   * @returns {import("./hyper-calendar.d.ts").TimeOfDay}
   */
  civilFromFrenchDecimalTime(hour, minute, second, attoseconds = 0) {
    const fn = this.#export("hc_civil_from_french_decimal_time");
    const [h, m, s] = [toU32(hour, "hour"), toU32(minute, "minute"), toU32(second, "second")];
    const attos = toU64(attoseconds, "attoseconds");
    const text = this.#text("hc_civil_from_french_decimal_time", (buffer, capacity) =>
      fn(h, m, s, attos, buffer, capacity), true);
    const [seconds, rest] = this.#oneLine("hc_civil_from_french_decimal_time", text, COLUMNS.civilFromFrenchDecimalTime);
    return { secondsOfDay: integer(seconds, "seconds of day"), attoseconds: bigInteger(rest, "attoseconds") };
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
   * Where the 八将神 and the 金神 stand in the 干支 year in force on a day,
   * the year turning at 立春 at the meridian: one entry per god and
   * direction, 金神 once for each branch the year's stem gives it.
   *
   * @param {number | bigint} fixed
   * @param {string} [meridian]
   * @returns {import("./hyper-calendar.d.ts").AlmanacDirection[]}
   */
  almanacDirections(fixed, meridian = "") {
    const fn = this.#export("hc_almanac_directions");
    const day = toI64(fixed, "fixed");
    const text = this.#withText(meridian, "meridian", (pointer, len) =>
      this.#text("hc_almanac_directions", (buffer, capacity) => fn(day, pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.almanacDirections, "hc_almanac_directions").map(almanacDirection);
  }

  /**
   * The fixed day of 臘日 in the winter that ends in a Gregorian year, by a
   * reckoning: `second-dragon-after-minor-cold`,
   * `second-dragon-from-minor-cold`, `dragon-nearest-major-cold-earlier`,
   * `dragon-nearest-major-cold-later`, `first-dog-after-major-cold`,
   * `first-dog-from-major-cold`, `lunar-twelfth-ninth`,
   * `ox-month-ninth-from-minor-cold`, `ox-month-ninth-after-minor-cold`,
   * `third-dog-after-winter-solstice` or `third-dog-from-winter-solstice`.
   * A winter whose lunar year the calendar does not reach is `no-data`.
   *
   * @param {import("./hyper-calendar.d.ts").RounichiRule} rule
   * @param {number | bigint} year
   * @param {string} [meridian]
   * @returns {number}
   */
  rounichi(rule, year, meridian = "") {
    const fn = this.#export("hc_rounichi");
    const y = toI64(year, "year");
    return this.#withText(rule, "rule", (rulePointer, ruleLen) =>
      this.#withText(meridian, "meridian", (meridianPointer, meridianLen) =>
        toNumber(fn(rulePointer, ruleLen, y, meridianPointer, meridianLen), "hc_rounichi")));
  }

  /**
   * What a publisher's list, `saijigoyomi` or `linderabell`, says the
   * 二十八宿 of a day favours and forbids, in the publisher's order.
   *
   * @param {import("./hyper-calendar.d.ts").UndertakingListId} list
   * @param {number | bigint} fixed
   * @returns {import("./hyper-calendar.d.ts").MansionUndertaking[]}
   */
  mansionUndertakings(list, fixed) {
    const fn = this.#export("hc_mansion_undertakings");
    const day = toI64(fixed, "fixed");
    const text = this.#withText(list, "list", (pointer, len) =>
      this.#text("hc_mansion_undertakings", (buffer, capacity) => fn(pointer, len, day, buffer, capacity), true));
    return rows(text, COLUMNS.mansionUndertakings, "hc_mansion_undertakings").map(mansionUndertaking);
  }

  /**
   * Whether a day is one of a person's own 五墓日 or 三箇の悪日, by the
   * 干支 year, turning at 立春, of the fixed day they were born on.
   *
   * @param {number | bigint} fixed
   * @param {number | bigint} birthFixed
   * @param {string} [meridian]
   * @returns {import("./hyper-calendar.d.ts").AlmanacPersonDay[]}
   */
  almanacPersonDays(fixed, birthFixed, meridian = "") {
    const fn = this.#export("hc_almanac_person_days");
    const day = toI64(fixed, "fixed");
    const born = toI64(birthFixed, "birthFixed");
    const text = this.#withText(meridian, "meridian", (pointer, len) =>
      this.#text("hc_almanac_person_days", (buffer, capacity) => fn(day, born, pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.almanacPersonDays, "hc_almanac_person_days").map(almanacPersonDay);
  }

  /**
   * What the Tibetan almanac of a version prints for a day, one entry a
   * column: the five components, the Sun, Rāhu, the years' names and the
   * elements and animals, the readings as the almanac writes them.
   *
   * @param {import("./hyper-calendar.d.ts").TibetanCalendarId} calendar
   * @param {number | bigint} fixed
   * @returns {import("./hyper-calendar.d.ts").TibetanAlmanacEntry[]}
   */
  tibetanAlmanacDay(calendar, fixed) {
    const fn = this.#export("hc_tibetan_almanac_day");
    const day = toI64(fixed, "fixed");
    const text = this.#withText(calendar, "calendar", (pointer, len) =>
      this.#text("hc_tibetan_almanac_day", (buffer, capacity) => fn(pointer, len, day, buffer, capacity), true));
    return rows(text, COLUMNS.tibetanAlmanacDay, "hc_tibetan_almanac_day").map(tibetanAlmanacEntry);
  }

  /**
   * Where the Phugpa almanac places the five planets at the end of a day.
   *
   * @param {number | bigint} fixed
   * @returns {import("./hyper-calendar.d.ts").TibetanPlanet[]}
   */
  tibetanPlanets(fixed) {
    const fn = this.#export("hc_tibetan_planets");
    const day = toI64(fixed, "fixed");
    const text = this.#text("hc_tibetan_planets", (buffer, capacity) => fn(day, buffer, capacity), true);
    return rows(text, COLUMNS.tibetanPlanets, "hc_tibetan_planets").map(tibetanPlanet);
  }

  /**
   * The Bhutanese calendar's winter solstice of a Gregorian year: the day,
   * the weekday and time the almanac prints, and the local Julian Date.
   *
   * @param {number | bigint} year
   * @returns {import("./hyper-calendar.d.ts").BhutaneseWinterSolstice}
   */
  bhutaneseWinterSolstice(year) {
    const fn = this.#export("hc_bhutanese_winter_solstice");
    const y = toI64(year, "year");
    const text = this.#text("hc_bhutanese_winter_solstice", (buffer, capacity) => fn(y, buffer, capacity), true);
    const [fixed, reading, julianDate] =
      this.#oneLine("hc_bhutanese_winter_solstice", text, COLUMNS.bhutaneseWinterSolstice);
    return { fixed: integer(fixed, "fixed"), reading, julianDate: decimal(julianDate, "julian date") };
  }

  /**
   * The fixed day a festival on a Tibetan date is kept on, by a rule for
   * a skipped or repeated number: `berzin` or `henning-almanac`. A skipped
   * number under `henning-almanac` is `no-data`.
   *
   * @param {import("./hyper-calendar.d.ts").TibetanFestivalRule} rule
   * @param {import("./hyper-calendar.d.ts").TibetanCalendarId} calendar
   * @param {number | bigint} year
   * @param {number} month
   * @param {boolean} leap
   * @param {number} day
   * @returns {number}
   */
  tibetanFestivalDay(rule, calendar, year, month, leap, day) {
    const fn = this.#export("hc_tibetan_festival_day");
    const y = toI64(year, "year");
    const m = toU32(month, "month");
    const d = toU32(day, "day");
    return this.#withText(rule, "rule", (rulePointer, ruleLen) =>
      this.#withText(calendar, "calendar", (calendarPointer, calendarLen) =>
        toNumber(fn(rulePointer, ruleLen, calendarPointer, calendarLen, y, m, leap ? 1 : 0, d), "hc_tibetan_festival_day")));
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
   * Every condition of the Kumbh Mela, the Mela Adhikari's seven, in the
   * order of the source: the identifiers {@link kumbh} and {@link kumbhBySky}
   * take.
   *
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").KumbhYogaInfo[]}
   */
  kumbhYogas(locale = "und") {
    const fn = this.#export("hc_kumbh_yogas");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_kumbh_yogas", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.kumbhYoga, "hc_kumbh_yogas").map(kumbhYogaInfo);
  }

  /**
   * Every river of the Pushkaram, Meṣa's first, with the sign Jupiter enters
   * for it.
   *
   * @param {string} [locale]
   * @returns {import("./hyper-calendar.d.ts").PushkaramRiver[]}
   */
  pushkaramRivers(locale = "und") {
    const fn = this.#export("hc_pushkaram_rivers");
    const text = this.#withText(locale, "locale", (pointer, len) =>
      this.#text("hc_pushkaram_rivers", (buffer, capacity) => fn(pointer, len, buffer, capacity), true));
    return rows(text, COLUMNS.pushkaramRiver, "hc_pushkaram_rivers").map(
      ([id, name, localeUsed, region, sign, signName, source]) => ({
        id,
        name,
        localeUsed,
        region: optional(region),
        sign: /** @type {import("./hyper-calendar.d.ts").SiderealSignId} */ (sign),
        signName,
        source,
      }));
  }

  /**
   * The two rules for which entry of Jupiter into a sign a Pushkaram
   * follows, the ones {@link pushkaramBySky} takes.
   *
   * @returns {import("./hyper-calendar.d.ts").PushkaramRule[]}
   */
  pushkaramRules() {
    const fn = this.#export("hc_pushkaram_rules");
    const text = this.#text("hc_pushkaram_rules", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(text, COLUMNS.pushkaramRule, "hc_pushkaram_rules").map(([id, description]) => ({
      id: /** @type {import("./hyper-calendar.d.ts").PushkaramEntryRule} */ (id),
      description,
    }));
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
   * The length of the temporal hour a reckoning of the Jewish day counts
   * its times in, on a day at a place.
   *
   * @param {import("./hyper-calendar.d.ts").ZmanimReckoning} reckoning
   * @param {number | bigint} fixed
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} [elevation]
   * @returns {import("./hyper-calendar.d.ts").TemporalHour}
   */
  temporalHour(reckoning, fixed, latitude, longitude, elevation = 0) {
    const fn = this.#export("hc_temporal_hour");
    const day = toI64(fixed, "fixed");
    const [lat, lon, elev] = [toF64(latitude, "latitude"), toF64(longitude, "longitude"), toF64(elevation, "elevation")];
    const text = this.#withText(reckoning, "reckoning", (pointer, len) =>
      this.#text("hc_temporal_hour", (buffer, capacity) => fn(pointer, len, day, lat, lon, elev, buffer, capacity), true));
    return temporalHour(this.#oneLine("hc_temporal_hour", text, COLUMNS.temporalHour));
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

  /**
   * A clock on a circular orbit against one held still on the ground: `hc_orbit_rate_offset`.
   * The line is the one kind of figure GPS is built on: the gravitational part of the rate, in
   * microseconds per 86 400-second day (positive: the higher clock runs fast), the kinematic
   * part (negative), their weak-field sum, and the exact Schwarzschild figure for the same two
   * clocks. A clock at `GPS_ORBIT_RADIUS` against one at `EARTH_EQUATORIAL_RADIUS` gains
   * +45.65 from the potential, loses 7.21 from its speed, and nets +38.44. `body` is an
   * identifier `hc_gravitating_bodies` lists, in any ASCII case; another is `unknown`. A
   * radius that is not finite or not positive, a ground radius at or inside the Schwarzschild
   * radius, or an orbit radius at or inside the photon sphere 3GM/c², where no circular orbit
   * exists, is `out-of-range`.
   *
   * @param {string} body
   * @param {number} orbitRadiusMetres
   * @param {number} groundRadiusMetres
   * @returns {import("./hyper-calendar.d.ts").OrbitRateOffset}
   */
  orbitRateOffset(body, orbitRadiusMetres, groundRadiusMetres) {
    const fn = this.#export("hc_orbit_rate_offset");
    const v_orbitRadiusMetres = toF64(orbitRadiusMetres, "orbitRadiusMetres");
    const v_groundRadiusMetres = toF64(groundRadiusMetres, "groundRadiusMetres");
    const written = this.#withText(body, "body", (bodyPointer, bodyLength) =>
        this.#text("hc_orbit_rate_offset", (buffer, capacity) => fn(bodyPointer, bodyLength, v_orbitRadiusMetres, v_groundRadiusMetres, buffer, capacity), true));
    return orbitRateOffset(this.#oneLine("hc_orbit_rate_offset", written, COLUMNS.orbitRateOffset));
  }

  /**
   * A rocket of constant proper acceleration burning from rest: `hc_rocket`. The line is the
   * hyperbolic motion of a constant proper acceleration `proper_acceleration` in m s⁻² after
   * `proper_seconds` aboard: the time that passes elsewhere, the distance covered, β, `1 − β`
   * computed without cancellation, and the Lorentz factor. One year at 1 g, 9.80665 m s⁻², is
   * 0.5636 light-years at three quarters of the speed of light, and 1.19 years pass elsewhere.
   * An acceleration that is not finite and positive, a proper time that is not finite or is
   * negative, or a burn long enough that a value leaves the range of a double, is `out-of-
   * range`.
   *
   * @param {number} properAcceleration
   * @param {number} properSeconds
   * @returns {import("./hyper-calendar.d.ts").RocketBurn}
   */
  rocket(properAcceleration, properSeconds) {
    const fn = this.#export("hc_rocket");
    const v_properAcceleration = toF64(properAcceleration, "properAcceleration");
    const v_properSeconds = toF64(properSeconds, "properSeconds");
    const written = this.#text("hc_rocket", (buffer, capacity) => fn(v_properAcceleration, v_properSeconds, buffer, capacity), true);
    return rocket(this.#oneLine("hc_rocket", written, COLUMNS.rocket));
  }

  /**
   * A flip-and-burn voyage between two points at rest: `hc_flip_and_burn`. The ship
   * accelerates at a constant proper acceleration for half the distance, turns over, and
   * decelerates for the other half, arriving at rest. At 1 g to Andromeda, 2.5 million light-
   * years, it is 28.60 years aboard and 2 500 001.94 at home. An acceleration that is not
   * finite and positive, a distance that is not finite or is negative, or a voyage that leaves
   * the range of a double, is `out-of-range`.
   *
   * @param {number} properAcceleration
   * @param {number} distanceMetres
   * @returns {import("./hyper-calendar.d.ts").FlipAndBurn}
   */
  flipAndBurn(properAcceleration, distanceMetres) {
    const fn = this.#export("hc_flip_and_burn");
    const v_properAcceleration = toF64(properAcceleration, "properAcceleration");
    const v_distanceMetres = toF64(distanceMetres, "distanceMetres");
    const written = this.#text("hc_flip_and_burn", (buffer, capacity) => fn(v_properAcceleration, v_distanceMetres, buffer, capacity), true);
    return flipAndBurn(this.#oneLine("hc_flip_and_burn", written, COLUMNS.flipAndBurn));
  }

  /**
   * The relativistic Doppler shift of a source moving at β, seen at an angle: `hc_doppler`. A
   * cosine of +1 is a source coming straight at the observer, −1 going straight away, 0
   * transverse in the observer's frame. At β = 0.6 head-on the frequency is doubled, receding
   * it is halved, and across the line of sight it is 4/5. A β that is not finite or whose
   * magnitude is 1 or more, or a cosine that is not finite or whose magnitude is above 1, is
   * `out-of-range`.
   *
   * @param {number} beta
   * @param {number} cosTheta
   * @returns {import("./hyper-calendar.d.ts").DopplerShift}
   */
  doppler(beta, cosTheta) {
    const fn = this.#export("hc_doppler");
    const v_beta = toF64(beta, "beta");
    const v_cosTheta = toF64(cosTheta, "cosTheta");
    const written = this.#text("hc_doppler", (buffer, capacity) => fn(v_beta, v_cosTheta, buffer, capacity), true);
    return doppler(this.#oneLine("hc_doppler", written, COLUMNS.doppler));
  }

  /**
   * The composition of two collinear velocities: `hc_velocity_add`. Both velocities are
   * fractions of the speed of light, positive one way and negative the other; they do not add,
   * their rapidities do. Two ships at 0.999 compose to 0.999 999 5, and `1 − β` of that,
   * 5.005·10⁻⁷, is carried without the cancellation that would lose eight of its figures. A β
   * that is not finite or whose magnitude is 1 or more is `out-of-range`.
   *
   * @param {number} firstBeta
   * @param {number} secondBeta
   * @returns {import("./hyper-calendar.d.ts").VelocityComposition}
   */
  velocityAdd(firstBeta, secondBeta) {
    const fn = this.#export("hc_velocity_add");
    const v_firstBeta = toF64(firstBeta, "firstBeta");
    const v_secondBeta = toF64(secondBeta, "secondBeta");
    const written = this.#text("hc_velocity_add", (buffer, capacity) => fn(v_firstBeta, v_secondBeta, buffer, capacity), true);
    return velocityAdd(this.#oneLine("hc_velocity_add", written, COLUMNS.velocityAdd));
  }

  /**
   * The Schwarzschild radius of a body: `hc_schwarzschild_radius`. The radius is 2GM/c² from
   * the body's standard gravitational parameter, which the table carries more exactly than its
   * mass. `body` is an identifier `hc_gravitating_bodies` lists, in any ASCII case; another, a
   * name such as `Sagittarius A*` included, is `unknown`.
   *
   * @param {string} body
   * @returns {import("./hyper-calendar.d.ts").SchwarzschildRadius}
   */
  schwarzschildRadius(body) {
    const fn = this.#export("hc_schwarzschild_radius");
    const written = this.#withText(body, "body", (bodyPointer, bodyLength) =>
        this.#text("hc_schwarzschild_radius", (buffer, capacity) => fn(bodyPointer, bodyLength, buffer, capacity), true));
    return schwarzschildRadius(this.#oneLine("hc_schwarzschild_radius", written, COLUMNS.schwarzschildRadius));
  }

  /**
   * A clock moving at a constant speed that is not exactly known: `hc_proper_time_uncertain`.
   * The standard deviation of the proper time is t β σ_β / √(1 − β²), first order, so a speed
   * known to a metre a second near the speed of light is an error of seconds a year. A speed
   * at or beyond the speed of light either way, a standard deviation that is not finite or is
   * negative, or a coordinate time that is not finite or whose proper time leaves the range of
   * a duration, is `out-of-range`.
   *
   * @param {number} speedMetresPerSecond
   * @param {number} speedStdDev
   * @param {number} coordinateSeconds
   * @returns {import("./hyper-calendar.d.ts").UncertainProperTime}
   */
  properTimeUncertain(speedMetresPerSecond, speedStdDev, coordinateSeconds) {
    const fn = this.#export("hc_proper_time_uncertain");
    const v_speedMetresPerSecond = toF64(speedMetresPerSecond, "speedMetresPerSecond");
    const v_speedStdDev = toF64(speedStdDev, "speedStdDev");
    const v_coordinateSeconds = toF64(coordinateSeconds, "coordinateSeconds");
    const written = this.#text("hc_proper_time_uncertain", (buffer, capacity) => fn(v_speedMetresPerSecond, v_speedStdDev, v_coordinateSeconds, buffer, capacity), true);
    return properTimeUncertain(this.#oneLine("hc_proper_time_uncertain", written, COLUMNS.properTimeUncertain));
  }

  /**
   * The CODATA constants the Planck units are built from, and the Planck units:
   * `hc_planck_units`. One line each for the speed of light, the reduced Planck constant, the
   * Newtonian constant of gravitation and the Planck time, length, mass, energy and
   * temperature, from the 2022 CODATA adjustment. `c` and ℏ are defined exactly and have a
   * standard uncertainty of 0; `G` is the one measured constant, and every Planck unit
   * inherits its 2.2·10⁻⁵, halved or thirded by the root.
   *
   * @returns {import("./hyper-calendar.d.ts").PlanckUnit[]}
   */
  planckUnits() {
    const fn = this.#export("hc_planck_units");
    const written = this.#text("hc_planck_units", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(written, COLUMNS.planckUnits, "hc_planck_units").map(planckUnits);
  }

  /**
   * A calendar age or year in one datum written in another: `hc_bp_convert`. The datums are
   * `bp` (calendar years before 1950 CE), `b2k` (before 2000 CE, the ice-core scale) and `ce`
   * (a calendar year in astronomical numbering, where year 0 is 1 BCE and −9700 is 9701 BCE).
   * Moving between them adds or subtracts a whole number of years, so the standard deviation
   * is unchanged. The Holocene's base is 11 700 b2k and 11 650 BP. A conventional radiocarbon
   * age, `radiocarbon-bp`, is not a count of calendar years and needs a calibration curve
   * (IntCal20 and its companions) that the crate does not carry: it is `no-data`, on either
   * side. A datum that is not one of these is `unknown`, and a number or standard deviation
   * that is not finite, or a negative standard deviation, is `out-of-range`.
   *
   * @param {number} years
   * @param {number} stdDevYears
   * @param {string} from
   * @param {string} to
   * @returns {import("./hyper-calendar.d.ts").DatumConversion}
   */
  bpConvert(years, stdDevYears, from, to) {
    const fn = this.#export("hc_bp_convert");
    const v_years = toF64(years, "years");
    const v_stdDevYears = toF64(stdDevYears, "stdDevYears");
    const written = this.#withText(from, "from", (fromPointer, fromLength) =>
        this.#withText(to, "to", (toPointer, toLength) =>
        this.#text("hc_bp_convert", (buffer, capacity) => fn(v_years, v_stdDevYears, fromPointer, fromLength, toPointer, toLength, buffer, capacity), true)));
    return bpConvert(this.#oneLine("hc_bp_convert", written, COLUMNS.bpConvert));
  }

  /**
   * A magnitude of time in one unit written in another, with its uncertainty carried through:
   * `hc_deep_convert`. The units are `planck-time`, `yoctosecond`, `zeptosecond`,
   * `attosecond`, `femtosecond`, `picosecond`, `nanosecond`, `microsecond`, `millisecond`,
   * `second`, `minute`, `hour`, `day`, `julian-year`, `kiloyear`, `megayear` and `gigayear`,
   * in any ASCII case. Every one but the Planck time is a defined multiple of the second and
   * rescales the standard deviation exactly; the Planck time is CODATA's measurement and
   * brings its 1.1·10⁻⁵ into the answer, so a round trip through it returns the same number
   * with a wider bar. A unit that is not one of these is `unknown`; a number or standard
   * deviation that is not finite, a negative standard deviation, or a result that leaves the
   * range of a double is `out-of-range`.
   *
   * @param {number} value
   * @param {number} stdDev
   * @param {string} from
   * @param {string} to
   * @returns {import("./hyper-calendar.d.ts").MagnitudeConversion}
   */
  deepConvert(value, stdDev, from, to) {
    const fn = this.#export("hc_deep_convert");
    const v_value = toF64(value, "value");
    const v_stdDev = toF64(stdDev, "stdDev");
    const written = this.#withText(from, "from", (fromPointer, fromLength) =>
        this.#withText(to, "to", (toPointer, toLength) =>
        this.#text("hc_deep_convert", (buffer, capacity) => fn(v_value, v_stdDev, fromPointer, fromLength, toPointer, toLength, buffer, capacity), true)));
    return deepConvert(this.#oneLine("hc_deep_convert", written, COLUMNS.deepConvert));
  }

  /**
   * Two magnitudes of time compared across the decades between them: `hc_deep_compare`. The
   * units are those of `hc_deep_convert`. The two are treated as independent, so comparing a
   * magnitude with itself reports a non-zero deviation around a ratio of 1. A unit that is not
   * one of those is `unknown`; a number or standard deviation that is not finite, a negative
   * standard deviation, a span of no length or a negative one (there is no logarithm of it),
   * or a ratio that leaves the range of a double is `out-of-range`.
   *
   * @param {number} firstValue
   * @param {number} firstStdDev
   * @param {string} firstUnit
   * @param {number} secondValue
   * @param {number} secondStdDev
   * @param {string} secondUnit
   * @returns {import("./hyper-calendar.d.ts").MagnitudeComparison}
   */
  deepCompare(firstValue, firstStdDev, firstUnit, secondValue, secondStdDev, secondUnit) {
    const fn = this.#export("hc_deep_compare");
    const v_firstValue = toF64(firstValue, "firstValue");
    const v_firstStdDev = toF64(firstStdDev, "firstStdDev");
    const v_secondValue = toF64(secondValue, "secondValue");
    const v_secondStdDev = toF64(secondStdDev, "secondStdDev");
    const written = this.#withText(firstUnit, "firstUnit", (firstUnitPointer, firstUnitLength) =>
        this.#withText(secondUnit, "secondUnit", (secondUnitPointer, secondUnitLength) =>
        this.#text("hc_deep_compare", (buffer, capacity) => fn(v_firstValue, v_firstStdDev, firstUnitPointer, firstUnitLength, v_secondValue, v_secondStdDev, secondUnitPointer, secondUnitLength, buffer, capacity), true)));
    return deepCompare(this.#oneLine("hc_deep_compare", written, COLUMNS.deepCompare));
  }

  /**
   * The daily mean insolation at any latitude and solar longitude, for the orbit of an epoch:
   * `hc_daily_insolation`. The Sun's true longitude is 0 at the March equinox, 90 at the June
   * solstice, 180 at the September equinox and 270 at the December solstice. It is not a date:
   * Berger's program turns a date into a longitude with a 365-day year, which differs from a
   * calendar's by up to a day, and the honest input is the longitude. `hc_orbit_at` is the
   * case of 65° N at 90°. The insolation is 0 in the polar night. An epoch the crate refuses
   * (not finite, or beyond a million years either side of 1950), a latitude that is not finite
   * or is beyond ±90°, or a longitude that is not finite or is outside 0 to 360, is `out-of-
   * range`.
   *
   * @param {number} yearsBeforePresent
   * @param {number} latitudeDegrees
   * @param {number} solarLongitudeDegrees
   * @returns {import("./hyper-calendar.d.ts").DailyInsolation}
   */
  dailyInsolation(yearsBeforePresent, latitudeDegrees, solarLongitudeDegrees) {
    const fn = this.#export("hc_daily_insolation");
    const v_yearsBeforePresent = toF64(yearsBeforePresent, "yearsBeforePresent");
    const v_latitudeDegrees = toF64(latitudeDegrees, "latitudeDegrees");
    const v_solarLongitudeDegrees = toF64(solarLongitudeDegrees, "solarLongitudeDegrees");
    const written = this.#text("hc_daily_insolation", (buffer, capacity) => fn(v_yearsBeforePresent, v_latitudeDegrees, v_solarLongitudeDegrees, buffer, capacity), true);
    return dailyInsolation(this.#oneLine("hc_daily_insolation", written, COLUMNS.dailyInsolation));
  }

  /**
   * An ISO 8601-2 value placed on the timeline: `hc_edtf_parse`. The first line is the value
   * itself and the lines after it are the parts it is made of: the two sides of an interval,
   * or each member of a `[...]` or `{...}` set. The first cell is the role: `value`, `start`,
   * `end` or `member`. A set is placed by the hull of its members, which is wider than the
   * set: `[1667,1670]` is somewhere from 1667 to the end of 1670, and the gap is lost. Times
   * of day, seasons, sub-year divisions, component-level qualifiers and exponential years are
   * refused rather than half-read. Text that is not a supported EDTF value is `malformed`, and
   * a year past what a fixed day can count is `out-of-range`.
   *
   * @param {string} text
   * @returns {import("./hyper-calendar.d.ts").EdtfPart[]}
   */
  edtfParse(text) {
    const fn = this.#export("hc_edtf_parse");
    const written = this.#withText(text, "text", (textPointer, textLength) =>
        this.#text("hc_edtf_parse", (buffer, capacity) => fn(textPointer, textLength, buffer, capacity), true));
    return rows(written, COLUMNS.edtfParse, "hc_edtf_parse").map(edtfParse);
  }

  /**
   * What can hold between two EDTF values placed on the timeline: `hc_edtf_relations`. Allen's
   * thirteen relations between intervals are tested over the two supports: where a bound is
   * unknown, every ordering it could have is considered, so the set is what remains possible,
   * never a guess. A date known to the year is the year; `1984~` is widened by its own length on
   * each side, 1982-12-31 to 1986-01-02; an open interval has no bound on its open side. Text that
   * is not a supported EDTF value is `malformed`, and a year past what a fixed day can count is
   * `out-of-range`.
   *
   * @param {string} first
   * @param {string} second
   * @returns {import("./hyper-calendar.d.ts").EdtfRelations}
   */
  edtfRelations(first, second) {
    const fn = this.#export("hc_edtf_relations");
    const written = this.#withText(first, "first", (firstPointer, firstLength) =>
        this.#withText(second, "second", (secondPointer, secondLength) =>
        this.#text("hc_edtf_relations", (buffer, capacity) => fn(firstPointer, firstLength, secondPointer, secondLength, buffer, capacity), true)));
    return edtfRelations(this.#oneLine("hc_edtf_relations", written, COLUMNS.edtfRelations));
  }

  /**
   * A number with a count of significant figures: `hc_significant`. 17 figures is the most a
   * double holds and means every digit is claimed, as for a count or a definition; the
   * shortest numeral that reads back as the same double is printed without padding. A number
   * that is not finite, or a count of figures that is not from 1 to 17, is `out-of-range`.
   *
   * @param {number} value
   * @param {number} figures
   * @returns {import("./hyper-calendar.d.ts").SignificantNumber}
   */
  significant(value, figures) {
    const fn = this.#export("hc_significant");
    const v_value = toF64(value, "value");
    const v_figures = toU32(figures, "figures");
    const written = this.#text("hc_significant", (buffer, capacity) => fn(v_value, v_figures, buffer, capacity), true);
    return significant(this.#oneLine("hc_significant", written, COLUMNS.significant));
  }

  /**
   * Arithmetic on two numbers with figure counts: `hc_significant_op`. `add` and `sub` are
   * significant down to the coarser of the two last places, so `100.0 + 0.001` keeps four
   * figures and `1.0000 − 0.9999` keeps one; `mul` and `div` carry the smaller figure count;
   * `pow` raises the first number to the second as an integer and keeps the first's count, the
   * second's figures being ignored. An operation that is not one of these is `unknown`. A
   * number that is not finite, a count of figures that is not from 1 to 17, a division by
   * zero, a power whose exponent is not an integer within an `i32`, or a result that leaves
   * the range of a double, is `out-of-range`.
   *
   * @param {string} operation
   * @param {number} first
   * @param {number} firstFigures
   * @param {number} second
   * @param {number} secondFigures
   * @returns {import("./hyper-calendar.d.ts").SignificantResult}
   */
  significantOp(operation, first, firstFigures, second, secondFigures) {
    const fn = this.#export("hc_significant_op");
    const v_first = toF64(first, "first");
    const v_firstFigures = toU32(firstFigures, "firstFigures");
    const v_second = toF64(second, "second");
    const v_secondFigures = toU32(secondFigures, "secondFigures");
    const written = this.#withText(operation, "operation", (operationPointer, operationLength) =>
        this.#text("hc_significant_op", (buffer, capacity) => fn(operationPointer, operationLength, v_first, v_firstFigures, v_second, v_secondFigures, buffer, capacity), true));
    return significantOp(this.#oneLine("hc_significant_op", written, COLUMNS.significantOp));
  }

  /**
   * A Gaussian quantity, `value ± σ`: `hc_uncertain`. A standard deviation of 0 is an exact
   * value, and its significant text is the shortest numeral that reads back as the same
   * double. A value or a standard deviation that is not finite, or a negative standard
   * deviation, is `out-of-range`.
   *
   * @param {number} value
   * @param {number} stdDev
   * @returns {import("./hyper-calendar.d.ts").UncertainQuantity}
   */
  uncertain(value, stdDev) {
    const fn = this.#export("hc_uncertain");
    const v_value = toF64(value, "value");
    const v_stdDev = toF64(stdDev, "stdDev");
    const written = this.#text("hc_uncertain", (buffer, capacity) => fn(v_value, v_stdDev, buffer, capacity), true);
    return uncertain(this.#oneLine("hc_uncertain", written, COLUMNS.uncertain));
  }

  /**
   * Arithmetic on Gaussian quantities, with the errors propagated to first order:
   * `hc_uncertain_op`. `add`, `sub`, `mul` and `div` are of independent quantities, with the
   * errors combined in quadrature; `combine` is the inverse-variance weighted mean of two
   * measurements of one quantity, where an exact one wins outright; `z-score` is how many
   * combined standard deviations separate the two; `scale` multiplies the first by the
   * second's value as an exact factor, its standard deviation ignored; `pow` raises the first
   * to the second's value as an exact exponent; `ln` and `exp` are of the first, the second
   * ignored. An operation that is not one of these is `unknown`. A value or standard deviation
   * that is not finite, a negative standard deviation, a division by a value of 0, a logarithm
   * of a value that is not positive, a power outside the real numbers, two exact values that
   * disagree under `combine`, a z-score of two exact values, or a result that leaves the range
   * of a double, is `out-of-range`.
   *
   * @param {string} operation
   * @param {number} first
   * @param {number} firstStdDev
   * @param {number} second
   * @param {number} secondStdDev
   * @returns {import("./hyper-calendar.d.ts").UncertainResult}
   */
  uncertainOp(operation, first, firstStdDev, second, secondStdDev) {
    const fn = this.#export("hc_uncertain_op");
    const v_first = toF64(first, "first");
    const v_firstStdDev = toF64(firstStdDev, "firstStdDev");
    const v_second = toF64(second, "second");
    const v_secondStdDev = toF64(secondStdDev, "secondStdDev");
    const written = this.#withText(operation, "operation", (operationPointer, operationLength) =>
        this.#text("hc_uncertain_op", (buffer, capacity) => fn(operationPointer, operationLength, v_first, v_firstStdDev, v_second, v_secondStdDev, buffer, capacity), true));
    return uncertainOp(this.#oneLine("hc_uncertain_op", written, COLUMNS.uncertainOp));
  }

  /**
   * Arithmetic on intervals of time: `hc_interval`. The two intervals are `[first_low_seconds,
   * first_high_seconds]` and `[second_low_seconds, second_high_seconds]` in whole seconds.
   * `add` and `sub` (the crossed bounds: `[lo₁ − hi₂, hi₁ − lo₂]`), `intersect` (empty when
   * they do not meet) and `hull` (the smallest interval holding both) write an interval, as
   * whole seconds and attoseconds each; `overlaps` and `contains` (whether the first holds the
   * second) answer a question, and write only the last cell. The width is the high bound minus
   * the low, and the midpoint is floored to an attosecond. An empty interval has no bounds,
   * width or midpoint. An operation that is not one of these is `unknown`; an interval whose
   * low bound is above its high bound, which is the empty interval written backwards, is `out-
   * of-range`.
   *
   * @param {string} operation
   * @param {number | bigint} firstLowSeconds
   * @param {number | bigint} firstHighSeconds
   * @param {number | bigint} secondLowSeconds
   * @param {number | bigint} secondHighSeconds
   * @returns {import("./hyper-calendar.d.ts").IntervalResult}
   */
  interval(operation, firstLowSeconds, firstHighSeconds, secondLowSeconds, secondHighSeconds) {
    const fn = this.#export("hc_interval");
    const v_firstLowSeconds = toI64(firstLowSeconds, "firstLowSeconds");
    const v_firstHighSeconds = toI64(firstHighSeconds, "firstHighSeconds");
    const v_secondLowSeconds = toI64(secondLowSeconds, "secondLowSeconds");
    const v_secondHighSeconds = toI64(secondHighSeconds, "secondHighSeconds");
    const written = this.#withText(operation, "operation", (operationPointer, operationLength) =>
        this.#text("hc_interval", (buffer, capacity) => fn(operationPointer, operationLength, v_firstLowSeconds, v_firstHighSeconds, v_secondLowSeconds, v_secondHighSeconds, buffer, capacity), true));
    return interval(this.#oneLine("hc_interval", written, COLUMNS.interval));
  }

  /**
   * Every unit of time with an exactly defined length: `hc_units`. One line each, shortest
   * first. A length is a numerator and a denominator in lowest terms, written in decimal,
   * because they are 128-bit integers and a quectosecond is 10⁻³⁰ of a second: no double holds
   * that, nor a flick's 705 600 000th. A unit that is measured rather than defined, such as
   * the sidereal day or the tropical year, is not here: it lives with the model that measured
   * it.
   *
   * @returns {import("./hyper-calendar.d.ts").TimeUnit[]}
   */
  units() {
    const fn = this.#export("hc_units");
    const written = this.#text("hc_units", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(written, COLUMNS.units, "hc_units").map(units);
  }

  /**
   * A count of one unit of time written in another, exactly: `hc_unit_convert`. The count is
   * `count_numerator / count_denominator`, and may be negative. Both counts are written as a
   * numerator and a denominator in lowest terms. A unit that `hc_units` does not list is
   * `unknown`; a denominator of 0 is `out-of-range`; and a count or a length whose numerator
   * or denominator leaves 128 bits is `out-of-range`.
   *
   * @param {number | bigint} countNumerator
   * @param {number | bigint} countDenominator
   * @param {string} from
   * @param {string} to
   * @returns {import("./hyper-calendar.d.ts").UnitConversion}
   */
  unitConvert(countNumerator, countDenominator, from, to) {
    const fn = this.#export("hc_unit_convert");
    const v_countNumerator = toI64(countNumerator, "countNumerator");
    const v_countDenominator = toI64(countDenominator, "countDenominator");
    const written = this.#withText(from, "from", (fromPointer, fromLength) =>
        this.#withText(to, "to", (toPointer, toLength) =>
        this.#text("hc_unit_convert", (buffer, capacity) => fn(v_countNumerator, v_countDenominator, fromPointer, fromLength, toPointer, toLength, buffer, capacity), true)));
    return unitConvert(this.#oneLine("hc_unit_convert", written, COLUMNS.unitConvert));
  }

  /**
   * Every frame rate and sample rate the crate carries as an exact period: `hc_rates`. The
   * NTSC rates are exact: 29.97 is 30 000 / 1 001. Each identifier is what `hc_frame_period`
   * reads for its rate.
   *
   * @returns {import("./hyper-calendar.d.ts").FrameRate[]}
   */
  rates() {
    const fn = this.#export("hc_rates");
    const written = this.#text("hc_rates", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(written, COLUMNS.rates, "hc_rates").map(rates);
  }

  /**
   * The length of one frame or one sample, exactly: `hc_frame_period`. `rate` is an identifier
   * of `hc_rates`, such as `29.97` or `48000`, or an exact rate written `n` or `n/d` events
   * per second, such as `30000/1001`. A rate that is neither, or a unit `hc_units` does not
   * list, is `unknown`; a rate that is not positive, or a denominator of 0, is `out-of-range`;
   * and a length whose numerator or denominator leaves 128 bits is `out-of-range`.
   *
   * @param {string} rate
   * @param {string} inUnit
   * @returns {import("./hyper-calendar.d.ts").FramePeriod}
   */
  framePeriod(rate, inUnit) {
    const fn = this.#export("hc_frame_period");
    const written = this.#withText(rate, "rate", (ratePointer, rateLength) =>
        this.#withText(inUnit, "inUnit", (inUnitPointer, inUnitLength) =>
        this.#text("hc_frame_period", (buffer, capacity) => fn(ratePointer, rateLength, inUnitPointer, inUnitLength, buffer, capacity), true)));
    return framePeriod(this.#oneLine("hc_frame_period", written, COLUMNS.framePeriod));
  }

  /**
   * A note at a tempo, exactly: `hc_tempo`. The tempo is `bpm_numerator / bpm_denominator`
   * beats per minute, where a beat is the note of `beat_halvings` halvings of a whole note.
   * The note is `note_halvings` halvings, with `dots` augmentation dots, each adding half of
   * what came before (one dot makes it 3/2 as long, two 7/4), and, when `tuplet_count` and
   * `tuplet_space` are both above 0, `tuplet_count` of it in the time of `tuplet_space`: a
   * triplet is 3 in the time of 2. Both 0 is no tuplet. A tempo that is not positive, a
   * denominator of 0, a tuplet with only one of its two numbers 0, or halvings or dots that do
   * not fit a byte, is `out-of-range`; a length whose numerator or denominator leaves 128
   * bits, or halvings or dots of 127 or more, is `out-of-range`.
   *
   * @param {number | bigint} bpmNumerator
   * @param {number | bigint} bpmDenominator
   * @param {number} noteHalvings
   * @param {number} dots
   * @param {number} tupletSpace
   * @param {number} tupletCount
   * @param {number} beatHalvings
   * @returns {import("./hyper-calendar.d.ts").NoteAtTempo}
   */
  tempo(bpmNumerator, bpmDenominator, noteHalvings, dots, tupletSpace, tupletCount, beatHalvings) {
    const fn = this.#export("hc_tempo");
    const v_bpmNumerator = toI64(bpmNumerator, "bpmNumerator");
    const v_bpmDenominator = toI64(bpmDenominator, "bpmDenominator");
    const v_noteHalvings = toU32(noteHalvings, "noteHalvings");
    const v_dots = toU32(dots, "dots");
    const v_tupletSpace = toU32(tupletSpace, "tupletSpace");
    const v_tupletCount = toU32(tupletCount, "tupletCount");
    const v_beatHalvings = toU32(beatHalvings, "beatHalvings");
    const written = this.#text("hc_tempo", (buffer, capacity) => fn(v_bpmNumerator, v_bpmDenominator, v_noteHalvings, v_dots, v_tupletSpace, v_tupletCount, v_beatHalvings, buffer, capacity), true);
    return tempo(this.#oneLine("hc_tempo", written, COLUMNS.tempo));
  }

  /**
   * Every fiscal, tax and academic year system the crate carries, country by country:
   * `hc_fiscal_profiles`. A validity bound is a label of the system's own calendar: Iran's are
   * Solar Hijri years and Nepal's Bikram Sambat. `valid from` is the year the system was
   * established, before which it is absent; `read from` is the first year the sources read
   * reach, and every label of the system between the two, or inside one of the `unread` spans,
   * is a gap and not an answer. The authority `unread` is a page read that states the year with
   * the instrument that fixes it not read. Nepal is in a build that has the `calendars`
   * layer too, since its year starts on 1 Shrawan of the Bikram Sambat; in a build without it
   * the country is absent, which `hc_fiscal_year_on` reports as `unknown`. No label convention
   * is a default: the year is named for the year it starts in, or for the one it ends in, and
   * every line says which.
   *
   * @returns {import("./hyper-calendar.d.ts").FiscalSystem[]}
   */
  fiscalProfiles() {
    const fn = this.#export("hc_fiscal_profiles");
    const written = this.#text("hc_fiscal_profiles", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(written, COLUMNS.fiscalProfiles, "hc_fiscal_profiles").map(fiscalProfiles);
  }

  /**
   * What the year systems of a country say a fixed day is: `hc_fiscal_year_on`. One line each.
   * The status is `in-force`, or `outside-validity` where the system was not in force in the
   * year the day falls in (the United States' October year had not begun in 1970), or `gap`
   * where it was in force and the sources read do not reach that year (the `read from` and
   * `unread` cells of `hc_fiscal_profiles`), or `outside-calendar-range` where the start's
   * calendar does not reach the day; the cells after the status are then empty. A country the tables do not carry, or a kind that is not
   * one of the four, is `unknown`; a country that has no system of the kind asked is `no-
   * data`; a fixed day beyond the Gregorian years ±9 999 999 is `out-of-range`.
   *
   * @param {string} country
   * @param {string} kind `government`, `personal-tax`, `corporate-default` or `academic`, or
   *   the empty string for every kind the country has
   * @param {number | bigint} fixed
   * @returns {import("./hyper-calendar.d.ts").FiscalYearOfDay[]}
   */
  fiscalYearOn(country, kind = "", fixed) {
    const fn = this.#export("hc_fiscal_year_on");
    const v_fixed = toI64(fixed, "fixed");
    const written = this.#withText(country, "country", (countryPointer, countryLength) =>
        this.#withText(kind, "kind", (kindPointer, kindLength) =>
        this.#text("hc_fiscal_year_on", (buffer, capacity) => fn(countryPointer, countryLength, kindPointer, kindLength, v_fixed, buffer, capacity), true)));
    return rows(written, COLUMNS.fiscalYearOn, "hc_fiscal_year_on").map(fiscalYearOn);
  }

  /**
   * The span of the year a label names in each year system of a country:
   * `hc_fiscal_year_span`. One line each. The label is the system's own: a label of Iran's is
   * a Solar Hijri year, a label of Japan's 年度 the Gregorian year it begins in, and a label of
   * the United States' fiscal year the one it ends in. The status is `in-force`, `outside-
   * validity` where the system was not in force in that year, `gap` where it was and the
   * sources read do not reach that year (the cells after the label are empty in both) or
   * `outside-calendar-range` where the start's calendar does not reach it. A
   * country the tables do not carry, or a kind that is not one of the four, is `unknown`; a
   * country with no system of the kind asked is `no-data`.
   *
   * @param {string} country
   * @param {string} kind `government`, `personal-tax`, `corporate-default` or `academic`, or
   *   the empty string for every kind the country has
   * @param {number | bigint} label
   * @returns {import("./hyper-calendar.d.ts").FiscalYearSpan[]}
   */
  fiscalYearSpan(country, kind = "", label) {
    const fn = this.#export("hc_fiscal_year_span");
    const v_label = toI64(label, "label");
    const written = this.#withText(country, "country", (countryPointer, countryLength) =>
        this.#withText(kind, "kind", (kindPointer, kindLength) =>
        this.#text("hc_fiscal_year_span", (buffer, capacity) => fn(countryPointer, countryLength, kindPointer, kindLength, v_label, buffer, capacity), true)));
    return rows(written, COLUMNS.fiscalYearSpan, "hc_fiscal_year_span").map(fiscalYearSpan);
  }

  /**
   * Every named year of whole weeks: `hc_week_year_systems`. The NRF 4-5-4 retail calendar,
   * ISO 8601's week-numbering year and a 4-4-5 year ending the last Saturday of December. The
   * two anchor rules are two names and not one parameter: they put the year end up to a week
   * apart and sometimes in different months.
   *
   * @returns {import("./hyper-calendar.d.ts").WeekYearSystem[]}
   */
  weekYearSystems() {
    const fn = this.#export("hc_week_year_systems");
    const written = this.#text("hc_week_year_systems", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(written, COLUMNS.weekYearSystems, "hc_week_year_systems").map(weekYearSystems);
  }

  /**
   * Where a fixed day is in a year of whole weeks: `hc_week_year_on`. In a 53-week year the
   * extra week is the last period's. A system `hc_week_year_systems` does not list is
   * `unknown`, and a day beyond the Gregorian years ±9 999 999 is `out-of-range`.
   *
   * @param {string} system
   * @param {number | bigint} fixed
   * @returns {import("./hyper-calendar.d.ts").WeekYearOfDay}
   */
  weekYearOn(system, fixed) {
    const fn = this.#export("hc_week_year_on");
    const v_fixed = toI64(fixed, "fixed");
    const written = this.#withText(system, "system", (systemPointer, systemLength) =>
        this.#text("hc_week_year_on", (buffer, capacity) => fn(systemPointer, systemLength, v_fixed, buffer, capacity), true));
    return weekYearOn(this.#oneLine("hc_week_year_on", written, COLUMNS.weekYearOn));
  }

  /**
   * Every name-day list the crate ships and every country it declines to ship one for:
   * `hc_name_day_lists`. Each list is a named edition of a named authority, and the crate
   * reports what the lists say and asserts none of them. A gap is a country whose list the
   * crate declines to ship, with the reason in its words: a list a university sells by the
   * copy, a church calendar that names saints and not given names, several published lists
   * that no body chooses between. A gap's identifier is the country's code, or two codes
   * joined by a hyphen where one reasoning covers both; `hc_name_days_on` and `hc_name_day`
   * read a code of either. A column that does not apply to the kind is empty.
   *
   * @returns {import("./hyper-calendar.d.ts").NameDayList[]}
   */
  nameDayLists() {
    const fn = this.#export("hc_name_day_lists");
    const written = this.#text("hc_name_day_lists", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(written, COLUMNS.nameDayLists, "hc_name_day_lists").map(nameDayLists);
  }

  /**
   * What the lists of a country name on a day: `hc_name_days_on`. The kind is `list` for an
   * edition in force in the year the day falls in (a country may keep two at once: Latvia's
   * traditional and extended lists), `outside` for a shipped edition whose years do not
   * include it, with no names, and `gap` for a country the crate carries no list for. A day no
   * list has a name for is a `list` line with no names, and never the nearest edition's. A
   * country the crate has neither a list nor a gap for is `unknown`; a day beyond the
   * Gregorian years ±9 999 999 is `out-of-range`.
   *
   * @param {string} country
   * @param {number | bigint} fixed
   * @returns {import("./hyper-calendar.d.ts").NameDaysOfDay[]}
   */
  nameDaysOn(country, fixed) {
    const fn = this.#export("hc_name_days_on");
    const v_fixed = toI64(fixed, "fixed");
    const written = this.#withText(country, "country", (countryPointer, countryLength) =>
        this.#text("hc_name_days_on", (buffer, capacity) => fn(countryPointer, countryLength, v_fixed, buffer, capacity), true));
    return rows(written, COLUMNS.nameDaysOn, "hc_name_days_on").map(nameDaysOn);
  }

  /**
   * The days of a year on which the lists of a country give a name: `hc_name_day`. The match
   * is exact and case-sensitive: the list's own spelling, diacritics included, and no
   * diminutive the authority did not print. A name may fall on several days (the extended
   * Latvian list has some) or on none, which is a `list` line with no days. The kinds are
   * those of `hc_name_days_on`: `outside` for an edition not in force in the year, with no
   * days, and `gap`. A country the crate has neither a list nor a gap for is `unknown`; a year
   * beyond the Gregorian years ±9 999 999 or past an `i32` is `out-of-range`.
   *
   * @param {string} country
   * @param {string} name
   * @param {number | bigint} year
   * @returns {import("./hyper-calendar.d.ts").NameDayDates[]}
   */
  nameDay(country, name, year) {
    const fn = this.#export("hc_name_day");
    const v_year = toI64(year, "year");
    const written = this.#withText(country, "country", (countryPointer, countryLength) =>
        this.#withText(name, "name", (namePointer, nameLength) =>
        this.#text("hc_name_day", (buffer, capacity) => fn(countryPointer, countryLength, namePointer, nameLength, v_year, buffer, capacity), true)));
    return rows(written, COLUMNS.nameDay, "hc_name_day").map(nameDay);
  }

  /**
   * Every attribution list the crate ships, with what it declines to ship:
   * `hc_attribution_authorities`. There is no "the birthstone of March": there are eight lists,
   * each with an authority, a date, a region and the years it was current, and they disagree
   * in eleven months out of twelve. A gap is a subject the crate declined to ship, such as
   * Japan's day-by-day 誕生花 or Robert Graves's "Celtic tree calendar"; its columns about a list
   * are empty. A gap belongs to no subject here and is written only for the empty one. A
   * subject that is not one of the seven is `unknown`.
   *
   * @param {string} [subject]
   * @returns {import("./hyper-calendar.d.ts").AttributionAuthority[]}
   */
  attributionAuthorities(subject = "") {
    const fn = this.#export("hc_attribution_authorities");
    const written = this.#withText(subject, "subject", (subjectPointer, subjectLength) =>
        this.#text("hc_attribution_authorities", (buffer, capacity) => fn(subjectPointer, subjectLength, buffer, capacity), true));
    return rows(written, COLUMNS.attributionAuthorities, "hc_attribution_authorities").map(attributionAuthorities);
  }

  /**
   * What every list of a subject attributes to one key: `hc_attributions`. One line per list.
   * There is no line for "the" birthstone: a question about March has six answers, and the
   * last cell says whether they agree. A contested list's caveat is on its line, and a caller
   * who shows the answer should show it. A leap month is no key: no tradition attributes
   * anything to an intercalary one. A subject that is not one of the seven is `unknown`, and a
   * key outside its range is `out-of-range`.
   *
   * @param {string} subject
   * @param {number | bigint} key
   * @returns {import("./hyper-calendar.d.ts").Attribution[]}
   */
  attributions(subject, key) {
    const fn = this.#export("hc_attributions");
    const v_key = toI64(key, "key");
    const written = this.#withText(subject, "subject", (subjectPointer, subjectLength) =>
        this.#text("hc_attributions", (buffer, capacity) => fn(subjectPointer, subjectLength, v_key, buffer, capacity), true));
    return rows(written, COLUMNS.attributions, "hc_attributions").map(attributions);
  }

  /**
   * What every list attributes to the month, the weekday and the sign of a day:
   * `hc_attributions_on`. The lines of `hc_attributions` for the birthstones, birth flowers,
   * full-moon names, month names, zodiac stones and weekday attributions, each list on its own
   * line, the subjects in that order; the lunation names are left out, because a day has no
   * lunation number without the March equinox of its year. The month is the Gregorian month,
   * the weekday its ISO weekday, and the sign the tropical sign the Sun is in at the day,
   * judged at the meridian, which a day whose sign changes within about ten minutes of local
   * midnight can move by a day. A meridian that is not read is `unknown`, and a day outside
   * the years −1000 to 3000 is `out-of-range`.
   *
   * @param {number | bigint} fixed
   * @param {string} [meridian]
   * @returns {import("./hyper-calendar.d.ts").AttributionOfDay[]}
   */
  attributionsOn(fixed, meridian = "") {
    const fn = this.#export("hc_attributions_on");
    const v_fixed = toI64(fixed, "fixed");
    const written = this.#withText(meridian, "meridian", (meridianPointer, meridianLength) =>
        this.#text("hc_attributions_on", (buffer, capacity) => fn(v_fixed, meridianPointer, meridianLength, buffer, capacity), true));
    return rows(written, COLUMNS.attributionsOn, "hc_attributions_on").map(attributionsOn);
  }

  /**
   * The Harvest Moon of a year: `hc_harvest_moon`. The Harvest Moon is the full moon nearest
   * the September equinox, a rule and not a table row: it falls in September in about three
   * years of four and in October in the rest. In 2025 it is 7 October, and September's full
   * moon is the Corn Moon. The days are judged at the meridian, and a full moon within about a
   * minute of a day's end can move by a day. A meridian that is not read is `unknown`, and a
   * year outside −999 to 3000 is `out-of-range`.
   *
   * @param {number | bigint} year
   * @param {string} [meridian]
   * @returns {import("./hyper-calendar.d.ts").HarvestMoon}
   */
  harvestMoon(year, meridian = "") {
    const fn = this.#export("hc_harvest_moon");
    const v_year = toI64(year, "year");
    const written = this.#withText(meridian, "meridian", (meridianPointer, meridianLength) =>
        this.#text("hc_harvest_moon", (buffer, capacity) => fn(v_year, meridianPointer, meridianLength, buffer, capacity), true));
    return harvestMoon(this.#oneLine("hc_harvest_moon", written, COLUMNS.harvestMoon));
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
