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
].map(Object.freeze));

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
    "day boundary", "formatted",
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
  ]),
  sky: Object.freeze([
    "sun longitude", "sun distance", "moon longitude", "moon latitude", "moon distance",
    "elongation", "illuminated fraction", "previous new moon", "next new moon",
    "ΔT", "ΔT regime", "source",
  ]),
  skyEvent: Object.freeze(["angle", "instant", "name", "japanese name"]),
});

/** The geologic ranks `hc_geologic_intervals` numbers, coarsest first. */
export const GEOLOGIC_RANKS = Object.freeze(["eon", "era", "period", "epoch", "age"]);

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
    extras, errorCode, errorName, standing, dayBoundary, formatted,
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
    end, endStdDev, endFigures, endApproximate, unit, description, source,
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
   * the locale's vocabulary. `locale` is a BCP 47 tag; one that does not
   * parse, or names no data, falls back to the root locale `und`, as the
   * module does — ask for `en` for English.
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
   * chronology at once. Negative years are the future.
   *
   * @param {number} yearsAgo
   * @param {number} [stdDevYears]
   * @returns {import("./hyper-calendar.d.ts").DeepTimeRow[]}
   */
  placeYearsAgo(yearsAgo, stdDevYears = 0) {
    const fn = this.#export("hc_place_years_ago");
    const years = toF64(yearsAgo, "yearsAgo");
    const stdDev = toF64(stdDevYears, "stdDevYears");
    const text = this.#text("hc_place_years_ago", (buffer, capacity) => fn(years, stdDev, buffer, capacity), true);
    return rows(text, COLUMNS.deepTime, "hc_place_years_ago").map(deepTimeRow);
  }

  /**
   * Every cosmic epoch, Big Bang to the present, then every dated cosmic
   * event, oldest first.
   *
   * @returns {import("./hyper-calendar.d.ts").DeepTimeRow[]}
   */
  cosmicEvents() {
    const fn = this.#export("hc_cosmic_events");
    const text = this.#text("hc_cosmic_events", (buffer, capacity) => fn(buffer, capacity), true);
    return rows(text, COLUMNS.deepTime, "hc_cosmic_events").map(deepTimeRow);
  }

  /**
   * Every interval of one rank of the geologic time scale, youngest first.
   * `rank` is a name (`eon`, `era`, `period`, `epoch`, `age`) or its number
   * from 0.
   *
   * @param {import("./hyper-calendar.d.ts").GeologicRank | number} rank
   * @returns {import("./hyper-calendar.d.ts").DeepTimeRow[]}
   */
  geologicIntervals(rank) {
    const fn = this.#export("hc_geologic_intervals");
    const number = typeof rank === "string" ? GEOLOGIC_RANKS.indexOf(rank) : rank;
    if (number === -1) {
      raise(`rank must be one of ${GEOLOGIC_RANKS.join(", ")} or 0 to 4, got ${JSON.stringify(rank)}`);
    }
    const r = toU32(number, "rank");
    const text = this.#text("hc_geologic_intervals", (buffer, capacity) => fn(r, buffer, capacity), true);
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
