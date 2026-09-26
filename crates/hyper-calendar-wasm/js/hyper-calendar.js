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
  { method: "describeDay", export: "hc_describe_day", feature: "calendars" },
  { method: "calendarUnits", export: "hc_calendar_units", feature: "calendars" },
  { method: "calendars", export: "hc_calendars", feature: "calendars" },
  { method: "locales", export: "hc_locales", feature: "calendars" },
  { method: "firstDayOfWeek", export: "hc_first_day_of_week", feature: "calendars" },
  { method: "gregorianAdoption", export: "hc_gregorian_adoption", feature: "calendars" },
  { method: "panchangaAt", export: "hc_panchanga_at", feature: "calendars" },
  { method: "panchangaOfDay", export: "hc_panchanga_of_day", feature: "calendars" },
  { method: "iocOlympiad", export: "hc_ioc_olympiad", feature: "calendars" },
  { method: "hebrewYahrzeit", export: "hc_hebrew_yahrzeit", feature: "calendars" },
  { method: "hebrewBirthday", export: "hc_hebrew_birthday", feature: "calendars" },
  { method: "chineseReckonedAge", export: "hc_chinese_reckoned_age", feature: "calendars" },
  { method: "chineseMarriageAugury", export: "hc_chinese_marriage_augury", feature: "calendars" },
  { method: "holidayIsDayOff", export: "hc_holiday_is_day_off", feature: "holiday" },
  { method: "holidaysInYear", export: "hc_holidays_in_year", feature: "holiday" },
  { method: "holidayCodes", export: "hc_holiday_codes", feature: "holiday" },
  { method: "holidaysOn", export: "hc_holidays_on", feature: "holiday" },
  { method: "holidayTables", export: "hc_holiday_tables", feature: "holiday" },
  { method: "lectionary", export: "hc_lectionary", feature: "holiday" },
  { method: "astronomicalEaster", export: "hc_astronomical_easter", feature: "holiday" },
  { method: "termInEffect", export: "hc_term_in_effect", feature: "seasons" },
  { method: "pentadInEffect", export: "hc_pentad_in_effect", feature: "seasons" },
  { method: "placeYearsAgo", export: "hc_place_years_ago", feature: "deep-time" },
  { method: "cosmicEvents", export: "hc_cosmic_events", feature: "deep-time" },
  { method: "geologicIntervals", export: "hc_geologic_intervals", feature: "deep-time" },
  { method: "fixedFromUnixInZone", export: "hc_fixed_from_unix_in_zone", feature: "tz" },
  { method: "unixFromFixedInZone", export: "hc_unix_from_fixed_in_zone", feature: "tz" },
  { method: "loadZone", export: "hc_zone_load", feature: "tz" },
  { method: "skyAt", export: "hc_sky_at", feature: "sky" },
  { method: "solarTermsBetween", export: "hc_solar_terms_between", feature: "sky" },
  { method: "moonPhasesBetween", export: "hc_moon_phases_between", feature: "sky" },
  { method: "earthRotationAngle", export: "hc_earth_rotation_angle", feature: "sky" },
  { method: "gmstIau2006", export: "hc_gmst_iau2006", feature: "sky" },
  { method: "gmstIau1982", export: "hc_gmst_iau1982", feature: "sky" },
  { method: "ut2MinusUt1", export: "hc_ut2_minus_ut1", feature: "sky" },
  { method: "solarTime", export: "hc_solar_time", feature: "sky" },
  { method: "solarEvent", export: "hc_solar_event", feature: "sky" },
  { method: "orbitAt", export: "hc_orbit_at", feature: "orbital" },
  { method: "orbitSeries", export: "hc_orbit_series", feature: "orbital" },
  { method: "marsTime", export: "hc_mars_time", feature: "planetary" },
  { method: "missions", export: "hc_missions", feature: "planetary" },
  { method: "missionSol", export: "hc_mission_sol", feature: "planetary" },
  { method: "bodies", export: "hc_bodies", feature: "planetary" },
  { method: "bodyTime", export: "hc_body_time", feature: "planetary" },
  { method: "properTime", export: "hc_proper_time", feature: "relativity" },
  { method: "gravitationalDilation", export: "hc_gravitational_dilation", feature: "relativity" },
  { method: "gravitatingBodies", export: "hc_gravitating_bodies", feature: "relativity" },
].map(Object.freeze));

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
  describeDay: Object.freeze([
    "id", "name", "era", "era label", "year", "month", "leap month", "month label",
    "day", "leap day", "extras", "error code", "error name", "standing",
    "day boundary", "formatted", "locale used", "day named by",
  ]),
  calendarUnits: Object.freeze([
    "start", "end", "label", "leap", "standing", "error code", "error name", "locale used",
  ]),
  calendars: Object.freeze([
    "id", "name", "english name", "earliest", "latest", "has era", "has year", "has month",
    "has day", "native locales", "standing",
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
    "kind", "name", "scope", "start", "start σ", "start figures", "start approximate",
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
  ]),
  marriageAugury: Object.freeze(["augury", "lichun at start", "lichun at end"]),
  holidayTables: Object.freeze([
    "code", "kind", "name", "english name", "locale used", "source", "country",
  ]),
  lectionary: Object.freeze(["liturgical year", "sunday cycle", "weekday cycle", "proper"]),
  value: Object.freeze(["value"]),
  solarTime: Object.freeze(["day", "hours", "missing", "missing day", "depression"]),
  solarEvent: Object.freeze(["instant", "missing", "missing day", "depression"]),
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
 * does not fit a JavaScript number, `allocation-failed` when `hc_alloc`
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
 * An `i64` argument: a safe integer or a `BigInt`.
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
 * A `u64` argument: a safe non-negative integer or a `BigInt`.
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
    // A line only ever grows at the end, so more cells are a newer module,
    // and fewer are not the format this file reads.
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
 * chronology has no figure.
 *
 * @param {string} value
 * @param {string} stdDev
 * @param {string} figures
 * @param {string} approximate
 * @param {string} what
 * @returns {import("./hyper-calendar.d.ts").DeepTimeBound | null}
 */
function bound(value, stdDev, figures, approximate, what) {
  if (value === "") {
    return null;
  }
  return {
    value: decimal(value, what),
    stdDev: decimal(stdDev, `${what} σ`),
    figures: optionalInteger(figures, `${what} figures`),
    approximate: flag(approximate, `${what} approximate`),
  };
}

/**
 * One row of any deep-time export.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").DeepTimeRow}
 */
function deepTimeRow(cells) {
  const [
    kind, name, scope, start, startStdDev, startFigures, startApproximate,
    end, endStdDev, endFigures, endApproximate, unit, description, source, localisedName,
  ] = cells;
  return {
    kind: /** @type {import("./hyper-calendar.d.ts").DeepTimeKind} */ (kind),
    name,
    scope: optional(scope),
    start: bound(start, startStdDev, startFigures, startApproximate, "start"),
    end: bound(end, endStdDev, endFigures, endApproximate, "end"),
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
  const [limb, number, name, devanagari, began, ends, readAt, ayanamsa] = cells;
  return {
    limb: /** @type {"yoga" | "karana"} */ (limb),
    number: integer(number, "number"),
    name,
    devanagari,
    began: integer(began, "began"),
    ends: integer(ends, "ends"),
    readAt: integer(readAt, "read at"),
    ayanamsa: optional(ayanamsa),
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
 * One line of `hc_holiday_tables`.
 *
 * @param {string[]} cells
 * @returns {import("./hyper-calendar.d.ts").HolidayTable}
 */
function holidayTable(cells) {
  const [code, kind, name, englishName, localeUsed, source, country] = cells;
  return {
    code,
    kind: /** @type {import("./hyper-calendar.d.ts").HolidayTableKind} */ (kind),
    name: optional(name),
    englishName,
    localeUsed: optional(localeUsed),
    source: optional(source),
    country: optional(country),
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
function missingSolarEvent(missing, day, depression) {
  if (missing === "") {
    return null;
  }
  return {
    event: /** @type {import("./hyper-calendar.d.ts").MissingSolarEventName} */ (missing),
    day: integer(day, "missing day"),
    depressionArcminutes: optionalInteger(depression, "depression"),
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
  const [instant, missing, missingDay, depression] = cells;
  return {
    instant: optionalInteger(instant, "instant"),
    missing: missingSolarEvent(missing, missingDay, depression),
  };
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
    id,
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
    id,
    name,
    kind: /** @type {import("./hyper-calendar.d.ts").BodyKind} */ (kind),
    primary: optional(primary),
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
    id,
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
  return { id, name, gm: decimal(gm, "gm"), gmConstant, source };
}

/** The capacity a text read starts with unless `load` was told otherwise. */
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
   * geologic rows carry the chart's own name in `locale` where it has one.
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
   * event, oldest first. No cosmic name has a translation, so
   * `localisedName` is `null` on every row whatever `locale` is.
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
   * Give the module a zone's TZif file under an IANA name; the two
   * `InZone` methods then answer for that name with the file's history,
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
   * The yoga and the karaṇa in progress at a POSIX instant, read as
   * Universal Time, the yoga reckoned with an ayanamsa: `Lahiri`, `Raman`,
   * `Krishnamurti` or `Fagan-Bradley`.
   *
   * @param {number | bigint} unixSeconds
   * @param {string} ayanamsa
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
   * @param {string} ayanamsa
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
   * Every holiday table, in {@link holidayCodes} order, with its kind, its
   * names, its sources and the country of an exchange where its table
   * records one. The tables carry English names only: `name` is filled
   * for a tag whose language is `en` and `null` for any other.
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
   * `asr-hanafi`, `jewish-dusk-vilna-gaon`, `jewish-sabbath-ends-cohn` or
   * `italian-zero-hour` — as POSIX seconds of Universal Time, or `null`
   * with the missing solar event named.
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
   * A mission's sol number at a POSIX instant by its own clock. A mission
   * with no published sol numbering is `no-data`; an instant before its
   * landing sol began is `out-of-range`.
   *
   * @param {string} mission
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
   * longitude. The Sun, which has no solar day, is `no-data`.
   *
   * @param {string} body
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
   * radius is `out-of-range`.
   *
   * @param {string} body
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
