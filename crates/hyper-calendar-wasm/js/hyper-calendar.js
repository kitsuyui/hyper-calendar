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
  { method: "describeDay", export: "hc_describe_day", feature: "calendars" },
  { method: "calendarUnits", export: "hc_calendar_units", feature: "calendars" },
  { method: "calendars", export: "hc_calendars", feature: "calendars" },
  { method: "locales", export: "hc_locales", feature: "calendars" },
  { method: "gregorianAdoption", export: "hc_gregorian_adoption", feature: "calendars" },
  { method: "holidayIsDayOff", export: "hc_holiday_is_day_off", feature: "holiday" },
  { method: "holidaysInYear", export: "hc_holidays_in_year", feature: "holiday" },
  { method: "holidayCodes", export: "hc_holiday_codes", feature: "holiday" },
  { method: "holidaysOn", export: "hc_holidays_on", feature: "holiday" },
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
  { method: "orbitAt", export: "hc_orbit_at", feature: "orbital" },
  { method: "orbitSeries", export: "hc_orbit_series", feature: "orbital" },
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
   * The POSIX timestamp of midnight UTC on a fixed day.
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
   * it, or the earlier of two midnights.
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
