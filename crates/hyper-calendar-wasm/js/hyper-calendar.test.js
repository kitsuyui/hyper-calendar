// The binding against the module built with `--features full`: every
// method, every line format column by column, the sentinels as thrown
// errors, and the buffer protocol with a capacity too small to fit
// anything.

import assert from "node:assert/strict";
import { before, describe, test } from "node:test";

import { COLUMNS, GEOLOGIC_RANKS, HcError, HyperCalendar, METHODS, NATIVE, SENTINELS, UNITS, load } from "./hyper-calendar.js";
import { FULL_WASM, TZIF_V2_DENVER, TZIF_V2_EASTERN, moduleBytes, rawRows } from "./support.js";

const HOW = "cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown --release --features full";

/** @type {Uint8Array} */
let bytes;
/** @type {HyperCalendar} */
let hc;

before(async () => {
  bytes = moduleBytes(FULL_WASM, HOW);
  hc = await load(bytes);
});

/**
 * The error a call throws, as an `HcError` with the given name.
 *
 * @param {() => unknown} call
 * @param {string} name
 * @returns {HcError}
 */
function refused(call, name) {
  let thrown;
  try {
    call();
  } catch (error) {
    thrown = error;
  }
  assert.ok(thrown instanceof HcError, `expected HcError, got ${String(thrown)}`);
  assert.equal(thrown.name, name);
  if (thrown.code !== null) {
    const sentinel = SENTINELS.find((candidate) => candidate.code === thrown.code);
    assert.ok(sentinel, `unknown code ${thrown.code}`);
    assert.equal(thrown.constant, sentinel.constant);
    assert.equal(typeof thrown.code, "bigint");
  }
  return thrown;
}

describe("load", () => {
  test("accepts the module in every form a page has it", async () => {
    const forms = {
      "Uint8Array": bytes,
      "ArrayBuffer": bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength),
      "WebAssembly.Module": new WebAssembly.Module(bytes),
      "WebAssembly.Instance": new WebAssembly.Instance(new WebAssembly.Module(bytes), {}),
      "instantiate() result": await WebAssembly.instantiate(bytes, {}),
      "Response with application/wasm": new Response(bytes, { headers: { "content-type": "application/wasm" } }),
      "Response without a type": new Response(bytes),
      "a promise": Promise.resolve(bytes),
    };
    for (const [form, source] of Object.entries(forms)) {
      const loaded = await load(source);
      assert.ok(loaded instanceof HyperCalendar, form);
      assert.equal(loaded.version(), hc.version(), form);
    }
  });

  test("fetches a URL or a string", async () => {
    const original = globalThis.fetch;
    /** @type {unknown[]} */
    const asked = [];
    globalThis.fetch = async (input) => {
      asked.push(input);
      return new Response(bytes, { headers: { "content-type": "application/wasm" } });
    };
    try {
      const url = new URL("https://example.invalid/hyper_calendar_wasm.wasm");
      assert.equal((await load(url)).version(), hc.version());
      assert.equal((await load(url.href)).version(), hc.version());
      assert.deepEqual(asked, [url, url.href]);
    } finally {
      globalThis.fetch = original;
    }
  });

  test("refuses what is not a module", async () => {
    await assert.rejects(load(/** @type {any} */ (42)), TypeError);
    await assert.rejects(load(new Response(bytes, { status: 404 })), TypeError);
    const empty = new WebAssembly.Instance(
      new WebAssembly.Module(Uint8Array.from([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00])),
      {},
    );
    await assert.rejects(load(empty), /no exported memory/);
    assert.throws(() => new HyperCalendar(hc, { initialCapacity: 0 }), TypeError);
  });

  test("names the version and the layers", () => {
    assert.match(hc.version(), /^\d+\.\d+\.\d+/);
    assert.deepEqual(hc.layers(), [
      "civil", "timestamps", "time-codes", "calendars", "holiday", "seasons", "deep-time", "tz", "sky", "orbital",
      "jupiter", "planetary", "relativity", "places", "humanize", "natural", "datetime", "patterns", "zone-names",
    ]);
    for (const entry of METHODS) {
      assert.ok(hc.has(entry.method), entry.method);
      assert.equal(typeof hc[entry.method], "function", entry.method);
      assert.equal(typeof hc.exports[entry.export], "function", entry.export);
    }
    assert.equal(hc.has("nothing"), false);
    assert.ok(hc.memory instanceof WebAssembly.Memory);
  });
});

describe("civil", () => {
  test("Gregorian conversion matches the fixed day", () => {
    const rd = hc.gregorianToFixed(2026, 9, 21);
    assert.equal(rd, 739_880);
    assert.equal(hc.gregorianToFixed(2026n, 9, 21), 739_880);
    assert.equal(hc.gregorianYear(rd), 2026);
    assert.equal(hc.gregorianMonth(rd), 9);
    assert.equal(hc.gregorianDay(rd), 21);
    assert.equal(hc.weekday(rd), 1);
    assert.equal(hc.weekday(739_880n), 1);
    assert.equal(hc.dayOfYear(hc.gregorianToFixed(2024, 12, 31)), 366);
    assert.equal(hc.isLeapYear(hc.gregorianToFixed(2024, 12, 31)), true);
    assert.equal(hc.dayOfYear(hc.gregorianToFixed(2023, 12, 31)), 365);
    assert.equal(hc.isLeapYear(hc.gregorianToFixed(2023, 12, 31)), false);
  });

  test("errors are thrown, not returned", () => {
    for (const [year, month, day] of [[2026, 2, 30], [2026, 13, 1], [2026, 1, 300]]) {
      const error = refused(() => hc.gregorianToFixed(year, month, day), "invalid-date");
      assert.equal(error.constant, "HC_ERR_INVALID_DATE");
      assert.equal(error.code, -9_000_000_000_000_001n);
      assert.equal(error.export, "hc_gregorian_to_fixed");
    }
    assert.ok(hc.gregorianToFixed(-9_000, 1, 1) < 0);
    refused(() => hc.gregorianYear(2n ** 62n), "out-of-range");
    assert.throws(() => hc.gregorianToFixed(2026.5, 9, 21), TypeError);
    assert.throws(() => hc.gregorianToFixed(2026, -1, 21), TypeError);
    assert.throws(() => hc.gregorianToFixed(2n ** 64n, 1, 1), TypeError);
    assert.throws(() => hc.weekday(/** @type {any} */ ("739880")), TypeError);
  });

  test("ISO dates round trip", () => {
    assert.equal(hc.formatIsoDate(739_880), "2026-09-21");
    assert.equal(hc.parseIsoDate("2026-09-21"), 739_880);
    // A day before the era round-trips through whatever expanded form the
    // module writes.
    const ides = hc.gregorianToFixed(-44, 3, 15);
    assert.equal(hc.parseIsoDate(hc.formatIsoDate(ides)), ides);
    assert.match(hc.formatIsoDate(ides), /^-0*44-03-15$/);
    for (const text of ["", "not a date", "2026-13-01", "2026-02-30"]) {
      refused(() => hc.parseIsoDate(text), "invalid-date");
    }
    assert.throws(() => hc.parseIsoDate(/** @type {any} */ (739_880)), TypeError);
    refused(() => hc.formatIsoDate(2n ** 62n), "out-of-range");
  });

  test("Unix time maps onto fixed days", () => {
    assert.equal(hc.fixedFromUnix(0), 719_163);
    assert.equal(hc.unixFromFixed(719_163), 0);
    assert.equal(hc.fixedFromUnix(-1), 719_162);
    assert.equal(hc.unixFromFixed(739_880), 1_789_948_800);
  });

  test("the leap-second table is reachable", () => {
    assert.equal(hc.taiMinusUtc(1_700_000_000, true), 37);
    assert.equal(hc.taiMinusUtc(1_700_000_000), 37);
    assert.equal(hc.dayHasLeapSecond(1_483_142_400), true);
    assert.equal(hc.dayHasLeapSecond(1_483_228_800), false);
    refused(() => hc.taiMinusUtc(4_000_000_000, true), "no-data");
    assert.equal(hc.taiMinusUtc(4_000_000_000, false), 37);
  });

  test("an i64 a number cannot hold is refused rather than rounded", () => {
    // 2^45 days after the epoch is 3 × 10^18 seconds: an i64, not a safe integer.
    refused(() => hc.unixFromFixed(2n ** 45n), "unsafe-integer");
  });

  test("an integer argument past the safe range is unsafe-integer, not a TypeError", () => {
    // 2^53 is a number the caller may have meant as 2^53 + 1: refused, and
    // as an HcError, so that the catch that takes a refused result takes it.
    for (const call of [
      () => hc.gregorianYear(2 ** 53),
      () => hc.gregorianYear(-(2 ** 60)),
      () => hc.fixedFromUnix(Number.MAX_SAFE_INTEGER + 1),
      () => hc.gregorianToFixed(2 ** 63, 1, 1),
    ]) {
      const error = refused(call, "unsafe-integer");
      assert.ok(error instanceof HcError);
      assert.equal(error.code, null);
      assert.equal(error.constant, null);
      assert.equal(error.export, null);
    }
    // The largest safe integer, and the same value past it as a BigInt, pass.
    refused(() => hc.gregorianYear(Number.MAX_SAFE_INTEGER), "out-of-range");
    refused(() => hc.gregorianYear(2n ** 53n + 1n), "out-of-range");
    // A u64 past the safe range is refused the same way.
    refused(() => hc.swatchBeat(0, 2 ** 60), "unsafe-integer");
    // A fraction, a non-finite number and a BigInt outside i64 stay TypeErrors.
    for (const value of [0.5, Number.POSITIVE_INFINITY, Number.NaN, 2n ** 63n]) {
      assert.throws(() => hc.gregorianYear(value), TypeError);
    }
  });

  test("a day too far back for seconds is out-of-range, not an unrecognised sentinel", () => {
    // Some 54 billion years back. Its midnight, about -1.7 × 10^18 seconds,
    // is an i64 far below HC_ERR_FLOOR, which every binding reads as a
    // sentinel, so the module refuses the day instead of answering.
    const farBack = -19_723_095_000_000;
    for (const [call, exportName] of [
      [() => hc.unixFromFixed(farBack), "hc_unix_from_fixed"],
      [() => hc.unixFromFixedInZone(farBack, "Asia/Tokyo"), "hc_unix_from_fixed_in_zone"],
    ]) {
      const error = refused(/** @type {() => unknown} */ (call), "out-of-range");
      assert.equal(error.constant, "HC_ERR_OUT_OF_RANGE");
      assert.equal(error.export, exportName);
    }
    assert.equal(hc.exports.hc_unix_from_fixed(BigInt(farBack)), -9_000_000_000_000_002n);
    // The README's first day answers, and the day before it does not.
    assert.equal(hc.unixFromFixed(-104_165_947_503), -8_999_999_999_942_400);
    refused(() => hc.unixFromFixed(-104_165_947_504), "out-of-range");
    // Past where an i64 runs out the day is refused too, not wrapped or clamped.
    refused(() => hc.unixFromFixed(2n ** 53n), "out-of-range");
    refused(() => hc.unixFromFixedInZone(2n ** 53n, "Asia/Tokyo"), "out-of-range");
    assert.equal(hc.exports.hc_unix_from_fixed(106_751_991_886_464n), -9_000_000_000_000_002n);
  });
});

describe("dayExtras", () => {
  test("labels each extra field and says which the date writes", () => {
    const raw = rawRows(hc, (buffer, capacity) =>
      hc.exports.hc_day_extras(739_886n, 0, 0, 0, 0, buffer, capacity));
    for (const cells of raw) {
      assert.equal(cells.length, COLUMNS.dayExtras.length, JSON.stringify(cells));
    }
    const rows = hc.dayExtras(739_886, "en");
    assert.equal(rows.length, raw.length);
    assert.ok(rows.every((row) => row.label !== "" && row.valueLabel !== ""));
    const tamil = hc.dayExtras(739_886, "en", "hindu-solar-tamil");
    assert.deepEqual(tamil, [
      { id: "hindu-solar-tamil", field: "samvatsara", value: 40, label: "Samvatsara (southern reckoning)", valueLabel: "Parabhava", inDate: true, localeUsed: "en" },
      { id: "hindu-solar-tamil", field: "tiruvalluvar-year", value: 2057, label: "Tiruvalluvar year", valueLabel: "2057", inDate: false, localeUsed: "en" },
    ]);
    // The north names the same year in the other cycle, under its own key.
    assert.deepEqual(hc.dayExtras(739_886, "en", "hindu-lunar-purnimanta").find((row) => row.field !== "vikrama-year"),
      { id: "hindu-lunar-purnimanta", field: "barhaspatya-samvatsara", value: 53, label: "Barhaspatya samvatsara (northern cycle)", valueLabel: "Siddharthin", inDate: true, localeUsed: "en" });
    // The formatted date writes the extras a source writes, and no pair.
    const described = hc.describeDay(739_886, "en");
    assert.equal(described.find((row) => row.id === "maya-longcount")?.formatted, "13.0.13.17.8");
    assert.ok(described.every((row) => !(row.formatted ?? "").includes("=")));
    assert.deepEqual(hc.dayExtras(739_886, "en", "rumi"), []);
    assert.throws(() => hc.dayExtras(739_886, "en", "no-such-calendar"), HcError);
  });
});

describe("describeDay", () => {
  test("writes every calendar as a line of the README's columns", () => {
    const rd = hc.gregorianToFixed(2026, 9, 21);
    const raw = rawRows(hc, (buffer, capacity) =>
      hc.exports.hc_describe_day(739_880n, 0, 0, buffer, capacity));
    assert.ok(raw.length > 50, `${raw.length} calendars`);
    for (const cells of raw) {
      assert.equal(cells.length, COLUMNS.describeDay.length, JSON.stringify(cells));
    }
    const rows = hc.describeDay(rd, "en");
    assert.equal(rows.length, raw.length);
    assert.deepEqual(rows.map((row) => row.id), raw.map((cells) => cells[0]), "registry order");
    // Every calendar either names the day or refuses it, and says where
    // its day begins either way.
    for (const row of rows) {
      assert.ok(row.dayBoundary.length > 0, row.id);
      assert.ok(row.localeUsed.length > 0, row.id);
      // Only a midnight start goes without a naming.
      assert.equal(row.dayNamedBy === null, row.dayBoundary === "midnight", row.id);
      if (row.error === null) {
        assert.ok(["in-use", "proleptic", "extended", "unrecorded"].includes(row.standing ?? ""), row.id);
        assert.equal(typeof row.year, "number", row.id);
        assert.equal(typeof row.formatted, "string", row.id);
      } else {
        assert.equal(row.standing, null, row.id);
        assert.equal(row.year, null, row.id);
        assert.equal(row.formatted, null, row.id);
        assert.match(row.error.name, /^[a-z-]+$/, row.id);
      }
    }
    const converted = rows.filter((row) => row.error === null).length;
    assert.ok(converted > rows.length / 2, `${converted} of ${rows.length} converted`);
  });

  test("a converted day decodes column by column", () => {
    const rows = hc.describeDay(739_880, "ja-JP");
    const japanese = rows.find((row) => row.id === "japanese");
    assert.deepEqual(japanese, {
      id: "japanese",
      name: "Japanese (imperial eras)",
      era: "reiwa",
      eraLabel: "令和",
      year: 8,
      month: 9,
      leapMonth: false,
      monthLabel: "9月",
      day: 21,
      leapDay: false,
      extras: {},
      error: null,
      standing: "in-use",
      dayBoundary: "midnight",
      formatted: "令和8年9月21日",
      localeUsed: "ja",
      dayNamedBy: null,
    });
    const gregorian = rows.find((row) => row.id === "gregory");
    assert.ok(gregorian);
    assert.equal(gregorian.name, "Gregorian");
    assert.equal(gregorian.year, 2026);
    assert.equal(gregorian.standing, "in-use");
    assert.equal(gregorian.formatted, "2026年9月21日");
    // The locales of the most-spoken languages write the day as their CLDR
    // 48 files do: sw.xml "d MMMM y", ur.xml "d MMMM، y", and mr.xml
    // "d MMMM, y" in Devanagari digits; Urdu names the Hijri month.
    const inLocale = (tag, id) => hc.describeDay(739_880, tag).find((row) => row.id === id);
    assert.equal(inLocale("sw", "gregory")?.formatted, "21 Septemba 2026");
    assert.equal(inLocale("ur", "gregory")?.formatted, "21 ستمبر، 2026");
    assert.equal(inLocale("mr", "gregory")?.formatted, "२१ सप्टेंबर, २०२६");
    assert.equal(inLocale("ur", "islamic-civil")?.localeUsed, "ur");
    // Japanese has no words for the Hebrew months, so the Hebrew calendar
    // answers in English, not Hebrew, and says so; only `native` borrows a
    // calendar's own language.
    const hebrew = rows.find((row) => row.id === "hebrew");
    assert.equal(hebrew?.localeUsed, "en");
    const umalqura = rows.find((row) => row.id === "islamic-umalqura");
    assert.equal(umalqura?.localeUsed, "en");
    assert.ok(!/[\u0600-\u06FF]/.test(umalqura?.formatted ?? ""), umalqura?.formatted);
    assert.equal(hc.describeDay(739_880, "en").find((row) => row.id === "gregory")?.monthLabel, "September");
    assert.equal(hc.describeDay(739_880, "en").find((row) => row.id === "gregory")?.formatted, "September 21, 2026");
    // `native` renders each calendar in its own language.
    const native = hc.describeDay(739_880, NATIVE);
    assert.equal(native.find((row) => row.id === "japanese")?.localeUsed, "ja");
    assert.equal(native.find((row) => row.id === "chinese")?.localeUsed, "zh-Hans");
    assert.equal(native.find((row) => row.id === "gregory")?.localeUsed, "en");
    assert.equal(native.find((row) => row.id === "hebrew")?.localeUsed, "he");
    // A tag with no data, or one that does not parse, falls back to the
    // root locale, whose month names are CLDR's M01..M12; so does the default.
    assert.equal(hc.describeDay(739_880, "tlh").find((row) => row.id === "gregory")?.monthLabel, "M09");
    assert.equal(hc.describeDay(739_880, "!!").find((row) => row.id === "gregory")?.monthLabel, "M09");
    assert.equal(hc.describeDay(739_880).find((row) => row.id === "gregory")?.monthLabel, "M09");
  });

  test("a day that does not begin at midnight names its civil day", () => {
    const rows = hc.describeDay(739_880, "en");
    /** @param {string} id */
    const naming = (id) => {
      const row = rows.find((candidate) => candidate.id === id);
      return [row?.dayBoundary, row?.dayNamedBy];
    };
    // JDN 2 451 545 begins at noon on 1 January 2000 and is that civil day.
    assert.deepEqual(naming("julian-day"), ["noon", "start"]);
    assert.deepEqual(naming("yerm"), ["noon", "start"]);
    assert.deepEqual(naming("tibetan"), ["local-time 05:00:00", "start"]);
    assert.deepEqual(naming("hindu-lunar"), ["sunrise", "start"]);
    // The medieval Icelandic day, from sunrise in summer and dawn in winter.
    assert.deepEqual(naming("icelandic-medieval"), ["daybreak", "start"]);
    // The Hebrew and Islamic evening is already the next date.
    assert.deepEqual(naming("hebrew"), ["sunset", "end"]);
    assert.deepEqual(naming("islamic-umalqura"), ["sunset", "end"]);
    assert.deepEqual(naming("samaritan"), ["sunset", "end"]);
    assert.deepEqual(naming("gregory"), ["midnight", null]);
    assert.deepEqual(naming("modified-julian-day"), ["midnight", null]);
  });

  test("a leap month and the extra fields are carried", () => {
    // 2023-03-22 was 閏二月初一 in the Chinese calendar.
    const day = hc.gregorianToFixed(2023, 3, 22);
    const chinese = hc.describeDay(day, "zh-Hans").find((row) => row.id === "chinese");
    assert.ok(chinese);
    assert.equal(chinese.month, 2);
    assert.equal(chinese.leapMonth, true);
    assert.equal(chinese.monthLabel, "闰二月");
    assert.equal(chinese.day, 1);
    assert.equal(chinese.leapDay, false);
    assert.equal(chinese.formatted, "2023癸卯年闰二月初一");
    assert.ok("cycle" in chinese.extras, JSON.stringify(chinese.extras));
    assert.ok(Object.keys(chinese.extras).length > 1, JSON.stringify(chinese.extras));
    for (const value of Object.values(chinese.extras)) {
      assert.equal(typeof value, "string");
    }
  });

  test("a refusal is a row with its code and name", () => {
    // The Rumi calendar was kept only from 1840 to 1925.
    const rumi = hc.describeDay(739_880, "en").find((row) => row.id === "rumi");
    assert.deepEqual(rumi, {
      id: "rumi",
      name: "Rumi",
      era: null,
      eraLabel: null,
      year: null,
      month: null,
      leapMonth: false,
      monthLabel: null,
      day: null,
      leapDay: false,
      extras: {},
      error: { code: 7, name: "after-supported-range" },
      standing: null,
      dayBoundary: "midnight",
      formatted: null,
      localeUsed: "en",
      dayNamedBy: null,
    });
    // And in 1900 it converts, so the refusal is about the day.
    const then = hc.describeDay(hc.gregorianToFixed(1900, 3, 14), "en").find((row) => row.id === "rumi");
    assert.equal(then?.error, null);
    assert.equal(then?.year, 1316);
  });

  test("the locale is text", () => {
    assert.throws(() => hc.describeDay(739_880, /** @type {any} */ (12)), TypeError);
  });
});

describe("parseDate", () => {
  test("reads back what describeDay writes, and a reader's own spelling", () => {
    const rd = hc.gregorianToFixed(2026, 9, 28);
    const japanese = hc.describeDay(rd, "ja").find((row) => row.id === "japanese");
    assert.equal(japanese?.formatted, "令和8年9月28日");
    const parsed = hc.parseDate("japanese", "ja", "令和8年9月28日");
    assert.deepEqual(parsed, { ...japanese, fixed: rd });
    for (const [calendar, locale, text] of [
      ["japanese", "ja", "令和八年九月二十八日"],
      ["gregory", "en", "Monday, Sep. 28, 2026"],
      ["gregory", "tr", "28 Eylül 2026"],
      ["gregory", "ar", "٢٨ سبتمبر ٢٠٢٦"],
      ["roc", "zh-Hant", "民國115年9月28日"],
    ]) {
      assert.equal(hc.parseDate(calendar, locale, text).fixed, rd, `${calendar} ${text}`);
    }
    const raw = rawRows(hc, (buffer, capacity) => {
      const calendar = new TextEncoder().encode("gregory");
      const text = new TextEncoder().encode("September 28, 2026");
      const at = hc.alloc(calendar.length + text.length);
      new Uint8Array(hc.memory.buffer, at, calendar.length).set(calendar);
      new Uint8Array(hc.memory.buffer, at + calendar.length, text.length).set(text);
      try {
        return hc.exports.hc_parse_date(at, calendar.length, 0, 0, at + calendar.length, text.length, buffer, capacity);
      } finally {
        hc.free(at, calendar.length + text.length);
      }
    });
    assert.equal(raw.length, 1);
    assert.equal(raw[0].length, COLUMNS.parseDate.length);
  });

  test("a text that is not one day says why", () => {
    const twoDigits = hc.parseDate("gregory", "en", "September 28, 26");
    assert.deepEqual(twoDigits.error, { code: 104, name: "two-digit-year" });
    assert.equal(twoDigits.fixed, null);
    assert.equal(twoDigits.year, null);
    assert.equal(hc.parseDate("chinese", "zh-Hans", "癸卯年闰二月初一").error?.name, "year-not-written");
    assert.equal(hc.parseDate("stata-week", "en", "2026w39").error?.name, "ambiguous");
    assert.equal(hc.parseDate("gregory", "en", "Tuesday, September 28, 2026").error?.name, "weekday-mismatch");
    assert.deepEqual(hc.parseDate("chinese", "zh-Hans", "2025丙午年八月十八").error, { code: 107, name: "field-mismatch" });
    assert.equal(hc.parseDate("gregory", "ja", "9月28日").error?.name, "year-not-written");
    assert.equal(hc.parseDate("gregory", "en", "").error?.name, "empty");
    const unread = hc.parseDate("gregory", "en", "September 28, 2026!");
    assert.deepEqual(unread.error, { code: 102, name: "not-recognised" });
    assert.equal(unread.fixed, null);
    assert.equal(hc.parseDate("gregory", "en", "February 30, 2026").error?.name, "day-out-of-range");
    refused(() => hc.parseDate("no-such-calendar", "en", "1"), "unknown");
  });
});

describe("calendarUnits", () => {
  test("walks a calendar's eras, years, months and days as labelled spans", () => {
    const id = new TextEncoder().encode("gregory");
    const pointer = hc.alloc(id.length);
    new Uint8Array(hc.memory.buffer, pointer, id.length).set(id);
    try {
      const raw = rawRows(hc, (buffer, capacity) =>
        hc.exports.hc_calendar_units(pointer, id.length, 1, 739_000n, 739_880n, 0, 0, buffer, capacity));
      assert.equal(raw.length, 3);
      for (const cells of raw) {
        assert.equal(cells.length, COLUMNS.calendarUnits.length, JSON.stringify(cells));
      }
    } finally {
      hc.free(pointer, id.length);
    }
    // The Japanese eras from 1989 to 2019.
    const eras = hc.calendarUnits("japanese", "era", hc.gregorianToFixed(1989, 1, 1), hc.gregorianToFixed(2019, 12, 31), "ja");
    assert.deepEqual(eras.map((span) => span.label), ["昭和", "平成", "令和"]);
    assert.deepEqual(eras[2], {
      start: hc.gregorianToFixed(2019, 5, 1),
      end: eras[2].end,
      label: "令和",
      leap: false,
      standing: "in-use",
      error: null,
      localeUsed: "ja",
    });
    for (let index = 1; index < eras.length; index += 1) {
      assert.equal(eras[index - 1].end, eras[index].start, "spans touch");
    }
    // A first year is 元年; a unit may be named by its index.
    const years = hc.calendarUnits("japanese", 1, hc.gregorianToFixed(2019, 5, 1), hc.gregorianToFixed(2020, 1, 2), "ja");
    assert.deepEqual(years.map((span) => span.label), ["令和元年", "令和2年"]);
    // The Chinese months of 2023 carry the leap second month.
    const months = hc.calendarUnits("chinese", "month", hc.gregorianToFixed(2023, 1, 22), hc.gregorianToFixed(2024, 2, 10), "zh-Hans");
    assert.equal(months.length, 13);
    const leap = months.find((span) => span.leap);
    assert.equal(leap?.label, "闰二月");
    assert.equal(leap?.start, hc.gregorianToFixed(2023, 3, 22));
    // A leap year is flagged.
    const gregorian = hc.calendarUnits("gregory", "year", hc.gregorianToFixed(2023, 6, 1), hc.gregorianToFixed(2025, 6, 1), "en");
    assert.deepEqual(gregorian.map((span) => [span.label, span.leap]), [["2023", false], ["2024", true], ["2025", false]]);
    // A calendar's edge is a refusal with the calendar's own error.
    const rumi = hc.calendarUnits("rumi", "year", hc.gregorianToFixed(1925, 1, 1), hc.gregorianToFixed(1927, 1, 1), "en");
    const last = rumi[rumi.length - 1];
    assert.deepEqual(last.error, { code: 7, name: "after-supported-range" });
    assert.equal(last.label, null);
    assert.equal(last.standing, null);
    // A unit the calendar does not have is one refusal.
    const noMonths = hc.calendarUnits("maya-longcount", "month", 0, 1000, "en");
    assert.equal(noMonths.length, 1);
    assert.deepEqual(noMonths[0].error, { code: 5, name: "unsupported-field" });
    assert.deepEqual(hc.calendarUnits("gregory", "day", 10, 10, "en"), []);
  });

  test("answers up to 100 000 spans and refuses more", () => {
    // The cap, hyper_calendar::lines::MAX_CALENDAR_UNITS: 100 000 days as
    // days are answered, one more is refused, and so is a trillion.
    const days = hc.calendarUnits("gregory", "day", 700_000, 800_000, "en");
    assert.equal(days.length, 100_000);
    assert.equal(days[99_999].end, 800_000);
    refused(() => hc.calendarUnits("gregory", "day", 700_000, 800_001, "en"), "out-of-range");
    refused(() => hc.calendarUnits("gregory", "day", 0, 1_000_000_000_000, "en"), "out-of-range");
    // The cap counts lines, not days.
    assert.equal(hc.calendarUnits("gregory", "year", 700_000, 1_100_000, "en").length, 1_096);
  });

  test("refuses what it does not know", () => {
    refused(() => hc.calendarUnits("no-such-calendar", "year", 0, 10, "en"), "unknown");
    assert.throws(() => hc.calendarUnits("gregory", /** @type {any} */ ("week"), 0, 10, "en"), TypeError);
    assert.throws(() => hc.calendarUnits("gregory", 4, 0, 10, "en"), TypeError);
    assert.deepEqual([...UNITS], ["era", "year", "month", "day"]);
  });
});

describe("calendars and locales", () => {
  test("every calendar is listed with what the locale calls it", () => {
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_calendars(739_880n, 0, 0, buffer, capacity));
    for (const cells of raw) {
      assert.equal(cells.length, COLUMNS.calendars.length, JSON.stringify(cells));
    }
    const rows = hc.calendars(739_880, "ja");
    assert.equal(rows.length, raw.length);
    assert.deepEqual(rows.map((row) => row.id), hc.describeDay(739_880).map((row) => row.id), "registry order");
    const gregorian = rows.find((row) => row.id === "gregory");
    assert.ok(gregorian);
    assert.equal(gregorian.name, "西暦(グレゴリオ暦)");
    assert.equal(gregorian.englishName, "Gregorian");
    assert.equal(typeof gregorian.earliest, "number");
    assert.deepEqual([gregorian.hasEra, gregorian.hasYear, gregorian.hasMonth, gregorian.hasDay], [false, true, true, true]);
    assert.deepEqual(gregorian.nativeLocales, []);
    assert.equal(gregorian.standing, "in-use");
    const japanese = rows.find((row) => row.id === "japanese");
    assert.equal(japanese?.name, "和暦");
    assert.deepEqual(japanese?.nativeLocales, ["ja"]);
    assert.equal(japanese?.hasEra, true);
    const chinese = rows.find((row) => row.id === "chinese");
    assert.deepEqual(chinese?.nativeLocales, ["zh-Hans", "zh-Hant"]);
    const longCount = rows.find((row) => row.id === "maya-longcount");
    assert.deepEqual([longCount?.hasEra, longCount?.hasYear, longCount?.hasMonth, longCount?.hasDay], [false, false, false, false]);
    assert.equal(hc.calendars(739_880, "tlh").find((row) => row.id === "gregory")?.name, null);
    // A locale without a name for a calendar leaves it unnamed rather than
    // borrowing the calendar's own language; only `native` does that.
    assert.equal(hc.calendars(739_880, "bo").find((row) => row.id === "japanese")?.name, null);
    assert.equal(hc.calendars(739_880, NATIVE).find((row) => row.id === "japanese")?.name, "和暦");
    assert.equal(hc.calendars(739_880, "zh-Hans").find((row) => row.id === "dangi")?.name, "檀纪历");
    assert.equal(hc.calendars(739_880, "zh-Hant").find((row) => row.id === "dangi")?.name, "檀紀曆");
  });

  test("the calendar list is the calendars' names without the day", () => {
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_calendar_list(0, 0, buffer, capacity));
    for (const cells of raw) {
      assert.equal(cells.length, COLUMNS.calendarList.length, JSON.stringify(cells));
    }
    for (const locale of ["ja", "en", "he", "tlh", NATIVE]) {
      const list = hc.calendarList(locale);
      const full = hc.calendars(739_880, locale);
      assert.deepEqual(
        list.map((row) => [row.id, row.name, row.englishName, row.nativeLocales]),
        full.map((row) => [row.id, row.name, row.englishName, row.nativeLocales]),
        locale,
      );
      for (const row of list) {
        assert.equal(row.localeUsed === null, row.name === null, JSON.stringify(row));
        assert.ok(row.crate?.startsWith("hc-calendars-"), JSON.stringify(row));
      }
    }
    const ja = hc.calendarList("ja");
    assert.deepEqual(ja.find((row) => row.id === "japanese"), {
      id: "japanese",
      name: "和暦",
      englishName: "Japanese (imperial eras)",
      localeUsed: "ja",
      crate: "hc-calendars-regional",
      nativeLocales: ["ja"],
    });
    assert.deepEqual(ja.find((row) => row.id === "gregory")?.nativeLocales, []);
    assert.equal(ja.find((row) => row.id === "chinese")?.crate, "hc-calendars-lunar");
    assert.equal(ja.find((row) => row.id === "hindu-lunar")?.crate, "hc-calendars-indic");
    const hebrew = hc.calendarList(NATIVE).find((row) => row.id === "hebrew");
    assert.deepEqual([hebrew?.name, hebrew?.localeUsed], ["לוח השנה העברי", "he"]);
  });

  test("every locale is listed with what it names", () => {
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_locales(buffer, capacity));
    for (const cells of raw) {
      assert.equal(cells.length, COLUMNS.locales.length, JSON.stringify(cells));
    }
    const rows = hc.locales();
    assert.equal(rows.length, 65, `${rows.length} locales`);
    assert.deepEqual(rows.map((row) => row.tag), [...rows.map((row) => row.tag)].sort(), "tag order");
    const ja = rows.find((row) => row.tag === "ja");
    assert.ok(ja);
    assert.equal(ja.englishName, "Japanese");
    assert.equal(ja.nativeName, "日本語");
    assert.deepEqual([ja.gregorianMonths, ja.weekdays, ja.gregorianEras], [true, true, true]);
    assert.ok(ja.calendars.includes("japanese") && ja.calendars.includes("chinese"), ja.calendars.join(","));
    assert.ok(!ja.calendars.includes("gregory"));
    const coptic = rows.find((row) => row.tag === "cop");
    assert.deepEqual([coptic?.gregorianMonths, coptic?.weekdays, coptic?.gregorianEras], [false, false, false]);
    assert.deepEqual(coptic?.calendars, ["coptic"]);
    // The locales of the most-spoken languages, from CLDR 48: Urdu names
    // the Hijri months (ur.xml), European Portuguese states no months of
    // its own and takes pt's (pt_PT.xml), and Cantonese is carried in both
    // of its scripts.
    const urdu = rows.find((row) => row.tag === "ur");
    assert.deepEqual([urdu?.englishName, urdu?.nativeName], ["Urdu", "اردو"]);
    assert.deepEqual([urdu?.gregorianMonths, urdu?.weekdays, urdu?.gregorianEras], [true, true, true]);
    assert.ok(urdu?.calendars.includes("islamic-civil"), urdu?.calendars.join(","));
    const portugal = rows.find((row) => row.tag === "pt-PT");
    assert.deepEqual([portugal?.gregorianMonths, portugal?.weekdays], [false, true]);
    for (const tag of ["fil", "ha", "mr", "pa-Arab", "pa-Guru", "pcm", "sw", "te", "yue-Hans", "yue-Hant"]) {
      assert.ok(rows.some((row) => row.tag === tag), tag);
    }
  });
});

describe("date-times, durations and intervals as text", () => {
  // RFC 3339 section 5.8, with the instants Python's datetime gives.
  test("a date-time is a reading, and an instant only where the text states a zone", () => {
    assert.deepEqual(hc.parseDatetime("rfc3339", "1996-12-19T16:39:57-08:00"), {
      localDay: 729_012, localSecond: 59_997, attoseconds: 0n, zone: "offset", offsetSeconds: -28_800,
      unixSeconds: 851_042_397, leapSecond: false, endOfDay: false,
    });
    const fraction = hc.parseDatetime("rfc3339", "1985-04-12T23:20:50.52Z");
    assert.deepEqual([fraction.zone, fraction.unixSeconds, fraction.attoseconds], ["utc", 482_196_050, 520_000_000_000_000_000n]);
    assert.equal(hc.parseDatetime("rfc3339", "1990-12-31T23:59:60Z").leapSecond, true);
    assert.equal(hc.parseDatetime("rfc3339", "1990-12-31T23:59:60Z").unixSeconds, 662_688_000);
    assert.equal(hc.parseDatetime("rfc3339", "2026-09-21T14:30:05-00:00").zone, "unknown-local");
    const local = hc.parseDatetime("iso8601", "2026-09-21T14:30:05");
    assert.deepEqual([local.zone, local.offsetSeconds, local.unixSeconds], ["none", null, null]);
    assert.equal(hc.parseDatetime("iso8601", "2026-W39-1T14:30:05+09:00").unixSeconds, 1_789_968_605);
    assert.equal(hc.parseDatetime("iso8601", "2026-09-21T24:00").endOfDay, true);
    assert.equal(hc.parseDatetime("python", "2026-09-21").localSecond, 0);
    assert.equal(hc.parseDatetime("rfc2822", "Sun, 06 Nov 1994 08:49:37 GMT").unixSeconds, 784_111_777);
    assert.equal(hc.parseDatetime("auto", "Sun, 06 Nov 1994 08:49:37 GMT").unixSeconds, 784_111_777);
    refused(() => hc.parseDatetime("iso8601", "2026-09-21"), "malformed");
    refused(() => hc.parseDatetime("iso8601", "2026-02-30T00:00:00Z"), "invalid-date");
    refused(() => hc.parseDatetime(/** @type {any} */ ("julian"), "x"), "unknown");
  });

  test("an instant is written in each syntax", () => {
    const write = (/** @type {any} */ syntax, offset = 32_400, attos = 0, precision = "auto") =>
      hc.formatDatetime(syntax, 1_789_968_605, attos, offset, precision).text;
    assert.equal(write("iso8601"), "2026-09-21T14:30:05+09:00");
    assert.equal(write("iso8601", 0), "2026-09-21T05:30:05Z");
    assert.equal(write("iso8601-basic"), "20260921T143005+0900");
    assert.equal(write("iso8601-week"), "2026-W39-1T14:30:05+09:00");
    assert.equal(write("iso8601-ordinal"), "2026-264T14:30:05+09:00");
    assert.equal(write("rfc3339", 32_400, 0, "milliseconds"), "2026-09-21T14:30:05.000+09:00");
    assert.equal(write("rfc2822"), "Mon, 21 Sep 2026 14:30:05 +0900");
    assert.equal(write("imf-fixdate"), "Mon, 21 Sep 2026 05:30:05 GMT");
    assert.equal(write("python", 0, 123_456_000_000_000_000n), "2026-09-21T05:30:05.123456+00:00");
    assert.deepEqual(hc.formatDatetime("rfc3339", 0), { text: "1970-01-01T00:00:00Z", syntax: "rfc3339" });
    refused(() => write("rfc3339", 0, 0, "minutes"), "unknown");
    refused(() => write("iso8601", 0, 10n ** 18n), "out-of-range");
  });

  // Python's isocalendar and %j.
  test("a day is a week date or an ordinal date, and a reduced date has parts", () => {
    assert.deepEqual(hc.formatIsoDateAs(739_880, "week"), { text: "2026-W39-1", form: "week", style: "extended" });
    assert.equal(hc.formatIsoDateAs(739_880, "ordinal", "basic").text, "2026264");
    assert.equal(hc.formatIsoDateAs(737_793, "week").text, "2020-W53-7");
    assert.deepEqual(hc.isoDateParts("2026-W39"), {
      form: "week", year: 2026, month: null, day: null, dayOfYear: null, week: 39, weekday: null, fixed: null,
      style: "extended",
    });
    assert.equal(hc.isoDateParts("2026-264").fixed, 739_880);
    assert.equal(hc.isoDateParts("2026-09").fixed, null);
    refused(() => hc.isoDateParts("2026-W54-1"), "invalid-date");
    refused(() => hc.formatIsoDateAs(0, /** @type {any} */ ("julian")), "unknown");
  });

  // Wikipedia's ISO 8601 examples, read 2026-10-03.
  test("durations and intervals are read and written", () => {
    const long = hc.isoDuration("P3Y6M4DT12H30M5S");
    assert.deepEqual([long.years, long.months, long.days, long.hours, long.minutes, long.seconds, long.nominal], [3, 6, 4, 12, 30, 5, true]);
    assert.equal(long.exactSeconds, null);
    assert.deepEqual([hc.isoDuration("PT36H").exactSeconds, hc.isoDuration("P1W").exactSeconds], [129_600n, 604_800n]);
    assert.equal(hc.isoDuration("PT0,5S").fraction, "5");
    assert.equal(hc.isoDuration("-P1D").negative, true);
    refused(() => hc.isoDuration("1 hour"), "malformed");
    assert.equal(hc.formatIsoDuration({ years: 3, months: 6, days: 4, hours: 12, minutes: 30, seconds: 5 }).text, "P3Y6M4DT12H30M5S");
    assert.equal(hc.formatIsoDuration({ seconds: 1, fraction: "5" }).text, "PT1.5S");
    refused(() => hc.formatIsoDuration({}), "malformed");
    assert.deepEqual(hc.isoInterval("2007-03-01T13:00:00Z/2008-05-11T15:30:00Z"), {
      repetitions: null, shape: "start-end", start: "2007-03-01T13:00:00Z", startUnixSeconds: 1_172_754_000,
      end: "2008-05-11T15:30:00Z", endUnixSeconds: 1_210_519_800, duration: null,
    });
    const repeating = hc.isoInterval("R5/2008-03-01T13:00:00Z/P1Y2M10DT2H30M");
    assert.deepEqual([repeating.repetitions, repeating.shape, repeating.startUnixSeconds, repeating.duration?.nominal], [5, "start-duration", 1_204_376_400, true]);
    assert.equal(hc.isoInterval("R/2008-03-01T13:00:00Z/PT1H").repetitions, "inf");
    refused(() => hc.isoInterval("2008"), "malformed");
  });

  // Python's strptime fills a missing date with 1900-01-01.
  test("a text is read against a pattern", () => {
    const python = hc.parsePattern("python", "%H:%M", "14:30");
    assert.deepEqual([python.year, python.hour, python.minute], [1900, 14, 30]);
    assert.deepEqual([python.reading?.localDay, python.reading?.localSecond], [693_596, 52_200]);
    assert.equal(hc.parsePattern("strftime", "%H:%M", "14:30").reading, null);
    const cldr = hc.parsePattern("cldr", "yyyy-MM-dd'T'HH:mm:ssXXX", "2026-09-21T14:30:05+09:00");
    assert.deepEqual([cldr.zone, cldr.offsetSeconds, cldr.reading?.unixSeconds], ["offset", 32_400, 1_789_968_605]);
    assert.equal(hc.parsePattern("strftime", "%s", "1789968605").unixSeconds, 1_789_968_605);
    const german = hc.parsePatternIn("cldr", "d. MMMM y", "21. Dezember 2026", "de");
    assert.deepEqual([german.year, german.month, german.day], [2026, 12, 21]);
    refused(() => hc.parsePattern("strftime", "%Y-%m-%d", "2026-02-30"), "invalid-date");
    refused(() => hc.parsePattern(/** @type {any} */ ("regex"), "%Y", "2026"), "unknown");
    refused(() => hc.parsePatternIn(/** @type {any} */ ("python"), "%b", "Dez", "de"), "unknown");
  });
});

describe("how a locale resolves", () => {
  // `docs/systems/locale-fallback.md`, from UTS #35 and CLDR 48's
  // `parentLocales`: en-AU goes to en-001, then en, then und.
  test("the chain follows parentLocales and truncation", () => {
    assert.deepEqual(hc.localeChain("en-AU"), [
      { step: 0, tag: "en-AU", rule: "requested", carried: false },
      { step: 1, tag: "en-001", rule: "parent-locales", carried: true },
      { step: 2, tag: "en", rule: "region", carried: true },
      { step: 3, tag: "und", rule: "root", carried: false },
    ]);
    assert.deepEqual(hc.localeChain("zh-TW").map((step) => [step.tag, step.rule]), [
      ["zh-Hant-TW", "likely-script"], ["zh-Hant", "region"], ["und", "parent-locales"],
    ]);
    assert.deepEqual(hc.localeChain("ja-JP-u-ca-japanese").map((step) => step.rule), [
      "requested", "extensions", "region", "root",
    ]);
    assert.equal(hc.localeChain("").length, 1);
    refused(() => hc.localeChain("not a tag"), "malformed");
  });

  test("a locale describes itself: week, numbering, direction, casing and plural rules", () => {
    const us = hc.localeInfo("en-US");
    assert.deepEqual([us.firstDay, us.minDays, us.direction, us.numbering], [7, 1, "ltr", "latn"]);
    assert.deepEqual([hc.localeInfo("de-DE").firstDay, hc.localeInfo("de-DE").minDays], [1, 4]);
    assert.equal(hc.localeInfo("ar-SA").numbering, "arab");
    assert.equal(hc.localeInfo("ar").numbering, "latn");
    assert.equal(hc.localeInfo("ar").direction, "rtl");
    assert.equal(hc.localeInfo("tr").casing, "turkic");
    assert.equal(hc.localeInfo("pt-PT").pluralRules, "pt-PT");
    const keyed = hc.localeInfo("ja-JP-u-ca-japanese-fw-sun-hc-h11-nu-jpan");
    assert.deepEqual(
      [keyed.calendarKey, keyed.numberingKey, keyed.firstDayKey, keyed.hourCycleKey, keyed.parent, keyed.parentRule],
      ["japanese", "jpan", 7, "h11", "ja-JP", "extensions"],
    );
    assert.equal(hc.firstDayOfWeek("en-US"), us.firstDay);
    refused(() => hc.localeInfo("e n"), "malformed");
  });

  // CLDR 48's plurals.xml.
  test("a number written as text has a plural category", () => {
    assert.equal(hc.pluralCategory("ru", "2").category, "few");
    assert.equal(hc.pluralCategory("ru", "5").category, "many");
    assert.equal(hc.pluralCategory("ru", "1.5").category, "other");
    assert.equal(hc.pluralCategory("ar", "0").category, "zero");
    assert.equal(hc.pluralCategory("en", "1").category, "one");
    assert.equal(hc.pluralCategory("en", "1.0").category, "other");
    assert.deepEqual(hc.pluralCategory("en", "1.30"), {
      category: "other", rules: "en", operands: { i: 1n, v: 2, w: 1, f: 30n, t: 3n },
    });
    refused(() => hc.pluralCategory("en", "1", "ordinal"), "no-data");
    refused(() => hc.pluralCategory("en", "1", /** @type {any} */ ("fraction")), "unknown");
    refused(() => hc.pluralCategory("en", "one"), "malformed");
    refused(() => hc.pluralCategory("en", "9".repeat(25)), "out-of-range");
  });

  test("a locale names a calendar's months, weekdays and day periods", () => {
    const de = hc.names("de", "gregory");
    assert.equal(de.find((entry) => entry.kind === "weekday" && entry.position === 1)?.name, "Montag");
    assert.equal(de.find((entry) => entry.kind === "month" && entry.position === 3)?.name, "März");
    assert.equal(hc.names("ru", "gregory", "wide", "format").find((entry) => entry.kind === "month" && entry.position === 9)?.name, "сентября");
    assert.equal(hc.names("ru", "gregory", "wide", "standalone").find((entry) => entry.kind === "month" && entry.position === 9)?.name, "сентябрь");
    assert.ok(hc.names("en", "hebrew").some((entry) => entry.kind === "month-in-leap-year" && entry.name === "Adar II"));
    refused(() => hc.names("de", "gregory", /** @type {any} */ ("huge")), "unknown");
    refused(() => hc.names("de", "no-such"), "unknown");
  });

  // Unicode's SpecialCasing for Turkish; UAX 9's isolates.
  test("a locale cases a text and isolates a field", () => {
    assert.deepEqual(hc.caseText("tr", "upper", "iyi"), { text: "İYİ", mode: "upper", casing: "turkic", localeUsed: "tr" });
    assert.equal(hc.caseText("en", "upper", "iyi").text, "IYI");
    assert.equal(hc.caseText("fr", "in-sentence", "Janvier").text, "janvier");
    assert.equal(hc.caseText("fr", "sentence-start", "janvier").text, "Janvier");
    assert.deepEqual(hc.isolate("ar", "field", "Sep 21"), {
      text: "\u2066Sep 21\u2069", direction: "rtl", textDirection: "ltr", isolated: true, mode: "field",
    });
    assert.equal(hc.isolate("en", "field", "Sep 21").text, "Sep 21");
    assert.equal(hc.isolate("en", "field", "سبتمبر").text, "\u2067سبتمبر\u2069");
    assert.equal(hc.isolate("en", "field", "2026").textDirection, null);
    assert.equal(hc.isolate("en", "strip", "\u2068x\u2069").text, "x");
    refused(() => hc.caseText("en", /** @type {any} */ ("title"), "x"), "unknown");
    refused(() => hc.isolate("en", /** @type {any} */ ("wrap"), "x"), "unknown");
  });

  test("the locales table carries each entry's parent, numbering and direction", () => {
    const rows = hc.locales();
    const byTag = (/** @type {string} */ tag) => rows.find((row) => row.tag === tag);
    assert.deepEqual([byTag("en-GB")?.parent, byTag("en-GB")?.numbering, byTag("en-GB")?.direction], ["en-001", "latn", "ltr"]);
    assert.deepEqual([byTag("ar-EG")?.parent, byTag("ar-EG")?.numbering, byTag("ar-EG")?.direction], ["ar", "arab", "rtl"]);
    assert.equal(byTag("ja")?.parent, "und");
  });
});

describe("firstDayOfWeek", () => {
  test("the week begins where CLDR's week data says for the locale", () => {
    // By region; by the region the language's likely subtags give; and
    // the root locale, which is also what a tag that does not parse is.
    const expected = {
      "en-US": 7,
      "en-GB": 1,
      ja: 7,
      "zh-Hans": 1,
      "ar-SA": 7,
      fr: 1,
      und: 1,
    };
    for (const [tag, day] of Object.entries(expected)) {
      assert.equal(hc.firstDayOfWeek(tag), day, tag);
    }
    assert.equal(hc.firstDayOfWeek(), 1, "und unless given");
    assert.equal(hc.firstDayOfWeek("not a tag"), 1);
    assert.equal(hc.firstDayOfWeek("de-u-fw-sun"), 7, "-u-fw- wins");
  });

  test("the export answers an i64", () => {
    const pointer = hc.alloc(2);
    new Uint8Array(hc.memory.buffer, pointer, 2).set([0x6a, 0x61]); // "ja"
    try {
      assert.equal(hc.exports.hc_first_day_of_week(pointer, 2), 7n);
      new Uint8Array(hc.memory.buffer, pointer, 2).set([0xff, 0xfe]);
      assert.equal(
        hc.exports.hc_first_day_of_week(pointer, 2),
        SENTINELS.find((entry) => entry.constant === "HC_ERR_NOT_UTF8")?.code,
      );
    } finally {
      hc.free(pointer, 2);
    }
  });
});

describe("gregorianAdoption", () => {
  /**
   * @param {number} year
   * @param {number} month
   * @param {number} day
   */
  const fixed = (year, month, day) => hc.gregorianToFixed(year, month, day);

  test("every step is a line of the README's columns", () => {
    const raw = rawRows(hc, (buffer, capacity) => {
      const pointer = hc.alloc(2);
      new Uint8Array(hc.memory.buffer, pointer, 2).set([0x43, 0x4e]); // "CN"
      try {
        return hc.exports.hc_gregorian_adoption(pointer, 2, buffer, capacity);
      } finally {
        hc.free(pointer, 2);
      }
    });
    assert.equal(raw.length, 2);
    for (const cells of raw) {
      assert.equal(cells.length, COLUMNS.gregorianAdoption.length, JSON.stringify(cells));
    }
  });

  test("Japan, Russia, Greece and Britain changed in one step", () => {
    const [japan, ...moreJapan] = hc.gregorianAdoption("JP");
    assert.deepEqual(moreJapan, []);
    assert.equal(japan.lastOldDay, fixed(1872, 12, 31));
    assert.equal(japan.firstDay, fixed(1873, 1, 1));
    assert.equal(japan.oldCalendar, "japanese-tenpo");
    assert.equal(japan.newCalendar, "gregory");
    assert.equal(japan.scope, "civil");
    assert.match(japan.source, /337/);
    const [russia] = hc.gregorianAdoption("ru");
    assert.deepEqual([russia.lastOldDay, russia.firstDay], [fixed(1918, 2, 13), fixed(1918, 2, 14)]);
    assert.equal(russia.oldCalendar, "julian");
    assert.equal(russia.polity, "Soviet Russia");
    const [greece] = hc.gregorianAdoption("GR");
    assert.equal(greece.firstDay, fixed(1923, 3, 1));
    const [britain] = hc.gregorianAdoption("GB");
    assert.equal(britain.firstDay, fixed(1752, 9, 14));
    assert.match(britain.source, /Calendar \(New Style\) Act 1750/);
  });

  test("a staged adoption is several steps", () => {
    const china = hc.gregorianAdoption("CN");
    assert.deepEqual(china.map((row) => [row.firstDay, row.scope]), [
      [fixed(1912, 1, 1), "partial"],
      [fixed(1929, 1, 1), "civil"],
    ]);
    assert.ok(china.every((row) => row.oldCalendar === "chinese"));
    const sweden = hc.gregorianAdoption("SE");
    assert.deepEqual(sweden.map((row) => [row.oldCalendar, row.newCalendar]), [
      ["julian", "swedish-1700"],
      ["swedish-1700", "julian"],
      ["julian", "gregory"],
    ]);
    assert.ok(sweden.every((row) => row.firstDay === row.lastOldDay + 1));
  });

  test("Saudi Arabia's step was partial, from the Umm al-Qura calendar", () => {
    const [saudi, ...more] = hc.gregorianAdoption("SA");
    assert.deepEqual(more, []);
    assert.equal(saudi.firstDay, fixed(2016, 10, 1));
    assert.equal(saudi.oldCalendar, "islamic-umalqura");
    assert.equal(saudi.scope, "partial");
  });

  test("a region the module does not know has no steps", () => {
    assert.deepEqual(hc.gregorianAdoption("ZZ"), []);
    assert.deepEqual(hc.gregorianAdoption(""), []);
  });
});

describe("holidays", () => {
  test("the tables are listed by identifier, countries first", () => {
    const codes = hc.holidayCodes();
    assert.ok(codes.length > 200, `${codes.length} tables`);
    assert.ok(codes.includes("JP") && codes.includes("XNYS") && codes.includes("un-days"));
    // Countries are two letters, exchanges four: XK is Kosovo, XNYS the
    // New York Stock Exchange.
    const firstExchange = codes.findIndex((code) => code.length === 4);
    assert.ok(firstExchange > 100, `${firstExchange} countries`);
    assert.ok(codes.slice(0, firstExchange).every((code) => code.length === 2), "countries first");
    assert.ok(codes.indexOf("XK") < firstExchange && codes.indexOf("XNYS") >= firstExchange);
  });

  test("a day off is a yes or a no, and an unknown table is refused", () => {
    assert.equal(hc.holidayIsDayOff("JP", "", hc.gregorianToFixed(2026, 1, 1)), true);
    const goodFriday = hc.gregorianToFixed(2026, 4, 3);
    assert.equal(hc.holidayIsDayOff("XNYS", "", goodFriday), true);
    assert.equal(hc.holidayIsDayOff("US", "", goodFriday), false);
    refused(() => hc.holidayIsDayOff("ZZ", "", goodFriday), "unknown");
    refused(() => hc.holidaysInYear("ZZ", "", 2026), "unknown");
    // A region the country has no subdivision for is refused, not read as a
    // subdivision whose days are not yet read.
    refused(() => hc.holidaysInYear("US", "US-ZZ", 2026), "unknown");
    refused(() => hc.holidaysInYear("JP", "Tokyo", 2026), "unknown");
    refused(() => hc.holidayIsDayOff("JP", "JP-99", goodFriday), "unknown");
    refused(() => hc.holidayIsDayOff("JP", "JP-14-130-5", goodFriday), "unknown");
    assert.equal(hc.holidayIsDayOff("JP", " jp-13 ", hc.gregorianToFixed(2026, 1, 1)), true);
    refused(() => hc.holidayIsDayOff("JP", "", 2n ** 62n), "out-of-range");
    assert.throws(() => hc.holidayIsDayOff("JP", /** @type {any} */ (null), goodFriday), TypeError);
  });

  test("a year decodes column by column", () => {
    const raw = rawRows(hc, (buffer, capacity) => {
      const code = hc.alloc(2);
      new Uint8Array(hc.memory.buffer, code, 2).set([0x4a, 0x50]);
      try {
        return hc.exports.hc_holidays_in_year(code, 2, 0, 0, 0, 0, 0, 0, 2026n, buffer, capacity);
      } finally {
        hc.free(code, 2);
      }
    });
    for (const cells of raw) {
      assert.equal(cells.length, COLUMNS.holidaysInYear.length, JSON.stringify(cells));
    }
    const year = hc.holidaysInYear("JP", "", 2026);
    assert.equal(year.length, raw.length);
    assert.deepEqual(year[0], {
      date: "2026-01-01",
      name: "New Year's Day",
      localName: "元日",
      kind: "public",
      confidence: "exact",
      substitute: false,
      observedFor: null,
      region: null,
      group: null,
      id: "new-years-day",
      source: null,
      bridged: false,
    });
    // 3 May 2026 is a Sunday; Constitution Memorial Day is taken on the 6th.
    const substitute = year.find((holiday) => holiday.substitute);
    assert.deepEqual(substitute, {
      date: "2026-05-06",
      name: "Constitution Memorial Day",
      localName: "憲法記念日",
      kind: "public",
      confidence: "exact",
      substitute: true,
      observedFor: "2026-05-03",
      region: null,
      group: null,
      id: "constitution-memorial-day",
      source: null,
      bridged: false,
    });
    assert.equal(hc.holidaysInYear("JP", "", 2026n).length, year.length);
    // A year before any rule applies is an empty list, not an error.
    assert.deepEqual(hc.holidaysInYear("JP", "", -5000), []);
  });

  test("one day across every table decodes column by column", () => {
    const day = hc.gregorianToFixed(2026, 5, 6);
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_holidays_on(BigInt(day), buffer, capacity));
    for (const cells of raw) {
      assert.equal(cells.length, COLUMNS.holidaysOn.length, JSON.stringify(cells));
    }
    const rows = hc.holidaysOn(day);
    assert.equal(rows.length, raw.length);
    const substitute = rows.find((row) => row.table === "JP" && row.substitute);
    assert.deepEqual(substitute, {
      table: "JP",
      tableName: "Japan",
      name: "Constitution Memorial Day",
      localName: "憲法記念日",
      kind: "public",
      confidence: "exact",
      source: null,
      substitute: true,
      observedFor: hc.gregorianToFixed(2026, 5, 3),
      region: null,
      group: null,
      id: "constitution-memorial-day",
      bridged: false,
    });
    // The tables come in the order holidayCodes() lists them.
    const codes = hc.holidayCodes();
    const positions = rows.map((row) => codes.indexOf(row.table));
    assert.ok(positions.every((position, index) => index === 0 || position >= positions[index - 1]));
    // A gap on an ordinary day is a table whose announcement for the year
    // has not been read, reported rather than left out.
    const gap = rows.find((row) => row.kind === "gap");
    assert.ok(gap, "a gap");
    assert.equal(gap.confidence, null);
    assert.equal(gap.source, null);
    assert.equal(gap.substitute, false);
    assert.equal(gap.observedFor, null);
  });

  test("a rule that cites its instrument carries it", () => {
    // World Braille Day, set by General Assembly resolution 73/161.
    const braille = hc.holidaysOn(hc.gregorianToFixed(2026, 1, 4)).find((row) => row.name === "World Braille Day");
    assert.deepEqual(braille, {
      table: "un-days",
      tableName: "United Nations international days",
      name: "World Braille Day",
      localName: null,
      kind: "observance",
      confidence: "exact",
      source: "A/RES/73/161",
      substitute: false,
      observedFor: null,
      region: null,
      group: null,
      id: "world-braille-day",
      bridged: false,
    });
  });

  test("a year a table cannot answer is reported as gaps", () => {
    // 2150 is past the Chinese calendar's range.
    const rows = hc.holidaysOn(hc.gregorianToFixed(2150, 2, 1));
    const chinese = rows.find((row) => row.table === "CN" && row.name === "Spring Festival");
    assert.deepEqual(chinese, {
      table: "CN",
      tableName: "China",
      name: "Spring Festival",
      localName: "春节",
      kind: "gap",
      confidence: null,
      source: null,
      substitute: false,
      observedFor: null,
      region: null,
      group: null,
      id: "spring-festival",
      bridged: false,
    });
  });

  test("a day with no year is refused", () => {
    refused(() => hc.holidaysOn(2n ** 63n - 1n), "out-of-range");
  });

  test("a kind filter keeps the entries of those kinds and the gaps of their rules", () => {
    const state = hc.holidayTables("en").find((table) => table.code === "US").regions.find((region) =>
      hc.holidaysInYear("US", region, 2026).some((day) => day.kind === "observance"));
    assert.ok(state, "a state with observances");
    const every = hc.holidaysInYear("US", state, 2026);
    const publicDays = hc.holidaysInYear("US", state, 2026, "", "public");
    assert.ok(publicDays.length < every.length);
    assert.ok(publicDays.every((day) => day.kind === "public" || day.kind === "gap"));
    assert.ok(publicDays.some((day) => day.id === "new-years-day"));
    // A list in any case, an empty filter and a word that is no kind.
    const both = hc.holidaysInYear("US", state, 2026, "", " PUBLIC ; bank");
    assert.ok(both.every((day) => ["public", "bank", "gap"].includes(day.kind)));
    assert.deepEqual(hc.holidaysInYear("US", state, 2026, "", ""), every);
    refused(() => hc.holidaysInYear("US", "", 2026, "", "public;festival"), "unknown");
    // A subdivision not read is a gap whatever the kind.
    const hampshire = hc.holidaysInYear("US", "US-NH", 2026, "", "observance").filter((day) => day.kind === "gap");
    assert.deepEqual(hampshire.map((day) => day.id), ["unread-subdivision"]);
    // The United Nations' days cite their resolutions, in the year's lines too.
    assert.ok(hc.holidaysInYear("un-days", "", 2026).some((day) => day.source?.includes("A/RES")));
  });

  test("the tables list the pairs of a region and a group and the subdivisions read", () => {
    const tables = hc.holidayTables("en");
    const japan = tables.find((table) => table.code === "JP");
    assert.ok(japan.readSubdivisions.includes("JP-14-130"));
    for (const region of japan.regions) {
      assert.ok(japan.readSubdivisions.includes(region), region);
    }
    assert.deepEqual(tables.find((table) => table.code === "XNYS").readSubdivisions, []);
    for (const table of tables) {
      for (const pair of table.regionGroups) {
        assert.ok(pair.region && pair.group, JSON.stringify(pair));
      }
    }
  });

  test("a line of hc_holidays_on_in keeps its names where they were and ends with the identifier", () => {
    const rows = hc.holidaysOnIn(hc.gregorianToFixed(2025, 9, 11), "cop");
    const nayrouz = rows.find((row) => row.table === "coptic-orthodox");
    assert.equal(nayrouz.id, "nayrouz-new-year");
    assert.equal(nayrouz.nameInLocale, "ⲡⲓⲭⲗⲟⲙ ⲛ̀ⲧⲉ ϯⲣⲟⲙⲡⲓ");
    assert.equal(nayrouz.nameLocale, "cop");
  });

  test("a subdivision's own day is a region of its country's table", () => {
    const tokyo = hc.holidaysOn(hc.gregorianToFixed(2026, 10, 1)).filter((row) => row.region === "JP-13");
    assert.deepEqual(tokyo, [{
      table: "JP",
      tableName: "Japan",
      name: "Tokyo Citizens' Day",
      localName: "都民の日",
      kind: "school",
      confidence: "exact",
      source: tokyo[0]?.source ?? null,
      substitute: false,
      observedFor: null,
      region: "JP-13",
      group: null,
      id: "tokyo-citizens-day",
      bridged: false,
    }]);
    assert.ok(tokyo[0].source?.includes("昭和27年東京都条例第75号"));
    const year = hc.holidaysInYear("JP", "JP-13", 2026);
    assert.deepEqual(year.filter((day) => day.region).map((day) => day.date), ["2026-03-10", "2026-10-01", "2026-11-07"]);
    // A city is a region within its prefecture: Saitama's 県民の日 carries
    // the prefecture's code, the city's own day the city's.
    const saitama = hc.holidaysInYear("JP", "JP-11-100", 2026).filter((day) => day.region);
    assert.ok(saitama.some((day) => day.date === "2026-05-01" && day.region === "JP-11-100"));
    assert.ok(saitama.some((day) => day.date === "2026-11-14" && day.region === "JP-11"));
    assert.ok(!hc.holidaysInYear("JP", "", 2026).some((day) => day.localName === "都民の日"));
    assert.ok(!hc.holidayCodes().includes("JP-13"));
    const japan = hc.holidayTables("en").find((table) => table.code === "JP");
    assert.ok(japan?.regions.includes("JP-13"));
    assert.ok(japan?.regions.includes("JP-47"));
    assert.ok(japan?.regions.includes("JP-14-100"));
    assert.deepEqual(hc.holidayTables("en").find((table) => table.code === "XNYS")?.regions, []);
  });

  test("a group's own day is a scope of its country's table", () => {
    const women = hc.holidaysOn(hc.gregorianToFixed(2026, 3, 8)).filter((row) => row.group === "women");
    assert.deepEqual(women, [{
      table: "CN",
      tableName: "China",
      name: "Women's Day",
      localName: "妇女节",
      kind: "half-day",
      confidence: "exact",
      source: "全国年节及纪念日放假办法, 第三条 (一): 妇女放假半天",
      substitute: false,
      observedFor: null,
      region: null,
      group: "women",
      id: "womens-day",
      bridged: false,
    }]);
    const year = hc.holidaysInYear("CN", "", 2026, "women");
    assert.deepEqual(year.filter((day) => day.group).map((day) => day.date), ["2026-03-08"]);
    assert.ok(!hc.holidaysInYear("CN", "", 2026).some((day) => day.localName === "妇女节"));
    // Before 1999, the first year of the statute's text read, the half day
    // is a gap row: no date, no confidence, the kind "gap", the group.
    const before = hc.holidaysInYear("CN", "", 1998, "women").filter((day) => day.kind === "gap");
    const womensGap = before.find((day) => day.localName === "妇女节");
    assert.ok(womensGap?.source?.includes("全国年节及纪念日放假办法"), womensGap?.source ?? "");
    assert.deepEqual(womensGap, {
      date: null,
      name: "Women's Day",
      localName: "妇女节",
      kind: "gap",
      confidence: null,
      substitute: false,
      observedFor: null,
      region: null,
      group: "women",
      id: "womens-day",
      source: womensGap.source,
      bridged: false,
    });
    // A state whose code was not read is a gap row with its region.
    const hampshire = hc.holidaysInYear("US", "US-NH", 2026).filter((day) => day.kind === "gap");
    assert.deepEqual(hampshire.map((day) => [day.name, day.region]), [["The subdivision's own days", "US-NH"]]);
    const childrensDay = hc.gregorianToFixed(2026, 6, 1);
    assert.equal(hc.holidayIsDayOff("CN", "", childrensDay, "children"), true);
    assert.equal(hc.holidayIsDayOff("CN", "", childrensDay), false);
    // A group the table gives no day to alone has everyone's days; a name
    // that is no group is refused.
    assert.equal(hc.holidayIsDayOff("CN", "", childrensDay, "police"), false);
    refused(() => hc.holidayIsDayOff("CN", "", childrensDay, "childrens"), "unknown");
    refused(() => hc.holidaysInYear("CN", "", 2026, "childrens"), "unknown");
    const china = hc.holidayTables("zh-CN").find((table) => table.code === "CN");
    assert.deepEqual(china?.groups, ["children", "military", "women", "youth"]);
    assert.deepEqual(china?.groupNames, ["少年儿童", "现役军人", "妇女", "青年"]);
    assert.deepEqual(hc.holidayTables("en").find((table) => table.code === "JP")?.groups, []);
  });

  test("a table's weekend laws are listed, with the regions that keep their own", () => {
    const tables = hc.holidayTables("en");
    assert.deepEqual(tables.find((table) => table.code === "JP")?.weekend, [
      { days: [6, 7], first: null, last: null, regions: [] },
    ]);
    const malaysia = tables.find((table) => table.code === "MY")?.weekend ?? [];
    assert.deepEqual(malaysia[0], { days: [6, 7], first: null, last: null, regions: [] });
    // Johor's Friday-Saturday years, and the years to 1994 that were not read.
    assert.deepEqual(malaysia[1], { days: null, first: null, last: "1994-12-31", regions: ["MY-01"] });
    assert.deepEqual(malaysia[2], { days: [5, 6], first: "2014-01-01", last: "2024-12-31", regions: ["MY-01"] });
    assert.deepEqual(malaysia[4], {
      days: [5, 6],
      first: "2013-11-25",
      last: null,
      regions: ["MY-02", "MY-03", "MY-11"],
    });
    const emirates = tables.find((table) => table.code === "AE")?.weekend ?? [];
    assert.deepEqual(emirates.at(-1), { days: [5, 6, 7], first: "2022-01-01", last: null, regions: ["AE-SH"] });
  });

  test("a table's regions are the ones it answers for, a region with only a weekend law among them", () => {
    const tables = hc.holidayTables("en");
    assert.deepEqual(tables.find((table) => table.code === "MY")?.regions, ["MY-01", "MY-02", "MY-03", "MY-09", "MY-11"]);
    assert.deepEqual(tables.find((table) => table.code === "AE")?.regions, ["AE-SH"]);
    const friday = hc.gregorianToFixed(2026, 3, 6);
    for (const table of tables) {
      // Every region a weekend law names is listed.
      for (const law of table.weekend) {
        for (const region of law.regions) {
          assert.ok(table.regions.includes(region), `${table.code} ${region}`);
        }
      }
      // And every listed region is accepted by the exports that take one: a gap is a refusal, never `unknown`.
      for (const region of table.regions) {
        for (const call of [
          () => hc.holidaysInYear(table.code, region, 2026),
          () => hc.holidayIsWeekend(table.code, region, friday),
          () => hc.holidayIsDayOff(table.code, region, friday),
        ]) {
          try {
            call();
          } catch (error) {
            assert.notEqual(error.name, "unknown", `${table.code} ${region}`);
          }
        }
        assert.ok(hc.placeName(region, "en").name, `${table.code} ${region}`);
      }
    }
    // A code that is no region of Malaysia is still refused.
    refused(() => hc.holidayIsWeekend("MY", "MY-99", friday), "unknown");
    refused(() => hc.holidaysInYear("MY", "AE-SH", 2026), "unknown");
  });
});

describe("almanac", () => {
  test("the term in effect decodes column by column", () => {
    const day = hc.gregorianToFixed(2024, 2, 10);
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_term_in_effect(BigInt(day), 0, 0, buffer, capacity));
    assert.equal(raw.length, 1);
    assert.equal(raw[0].length, COLUMNS.term.length);
    // 2024-02-04 was 立春 in Japan; the term runs to 18 February.
    const term = hc.termInEffect(day, "japan");
    assert.equal(term.index, 21);
    assert.equal(term.chineseName, "立春");
    assert.equal(term.japaneseName, "立春");
    assert.equal(term.begins, hc.gregorianToFixed(2024, 2, 4));
    assert.equal(term.ends, hc.gregorianToFixed(2024, 2, 18));
    assert.ok(term.chineseAuthority.length > 0 && term.japaneseAuthority.length > 0);
  });

  test("the pentad in effect decodes column by column", () => {
    const day = hc.gregorianToFixed(2024, 2, 10);
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_pentad_in_effect(BigInt(day), 0, 0, buffer, capacity));
    assert.equal(raw.length, 1);
    assert.equal(raw[0].length, COLUMNS.term.length);
    // 立春 is pentads 63, 64 and 65 from 春分; 10 February is in the second.
    const pentad = hc.pentadInEffect(day, "japan");
    assert.equal(pentad.index, 64);
    assert.equal(pentad.chineseName, "蟄蟲始振");
    assert.equal(pentad.japaneseName, "黄鶯睍睆");
    assert.equal(pentad.begins, hc.gregorianToFixed(2024, 2, 9));
    assert.equal(pentad.ends, hc.gregorianToFixed(2024, 2, 13));
    assert.ok(pentad.chineseAuthority.length > 0 && pentad.japaneseAuthority.length > 0);
  });

  test("a pentad is named by each tradition, with the alternate its text prints", () => {
    // 暦Wiki's table of the 七十二候: 立春次候 is 蟄虫始振 before the 貞享暦, 梅花乃芳 in it and 黄鶯睍睆 from the
    // 宝暦暦; 大雪次候's 虎始交 has the alternate 武始交 in the first.
    assert.deepEqual(hc.pentadTraditions().map((tradition) => tradition.id), ["chinese", "japanese", "jokyo", "senmyo"]);
    assert.equal(hc.pentadTraditions().find((tradition) => tradition.id === "senmyo")?.alternates, 4);
    const day = hc.gregorianToFixed(2024, 2, 10);
    for (const [tradition, name] of /** @type {const} */ ([
      ["senmyo", "蟄虫始振"], ["jokyo", "梅花乃芳"], ["japanese", "黄鶯睍睆"],
    ])) {
      const pentad = hc.pentadInTradition(day, tradition, "japan");
      assert.deepEqual([pentad.index, pentad.name, pentad.tradition], [64, name, tradition]);
      assert.equal(pentad.begins, hc.gregorianToFixed(2024, 2, 9));
      assert.equal(pentad.ends, hc.gregorianToFixed(2024, 2, 13));
      assert.equal(pentad.alternate, null);
    }
    const tiger = hc.pentadInTradition(hc.gregorianToFixed(2026, 12, 14), "senmyo", "japan");
    assert.deepEqual([tiger.index, tiger.name, tiger.alternate], [52, "虎始交", "武始交"]);
    refused(() => hc.pentadInTradition(day, /** @type {any} */ ("horyaku"), "japan"), "unknown");
  });

  test("the zassetsu and the seasonal days of a year are their sources'", () => {
    // 暦要項 2024: 節分 on 3 February, 入梅 on 10 June, 土用の入り on 19 July with its 丑の日 on 24 July and 5 August.
    const zassetsu = hc.zassetsuInYear(2024, "japan");
    assert.equal(zassetsu.length, 25);
    const find = (/** @type {string} */ id) => zassetsu.find((day) => day.id === id);
    assert.equal(find("spring-setsubun")?.day, hc.gregorianToFixed(2024, 2, 3));
    assert.equal(find("nyubai")?.day, hc.gregorianToFixed(2024, 6, 10));
    const summer = find("summer-doyo-entry");
    assert.deepEqual(
      [summer?.day, summer?.last, summer?.firstOxDay, summer?.secondOxDay],
      [
        hc.gregorianToFixed(2024, 7, 19), hc.gregorianToFixed(2024, 8, 6),
        hc.gregorianToFixed(2024, 7, 24), hc.gregorianToFixed(2024, 8, 5),
      ],
    );
    assert.equal(find("spring-setsubun")?.last, null);
    assert.equal(find("hangesho-classical")?.rule, "classical");
    // The three 伏 of 2026 in China begin on 15 July, 25 July and 14 August (Wikipedia zh, 三伏).
    const seasonal = hc.seasonalDaysInYear(2026, "china");
    assert.deepEqual(
      seasonal.filter((day) => day.kind === "san-fu").map((day) => [day.name, day.localName, day.first]),
      [["First fu", "初伏", hc.gregorianToFixed(2026, 7, 15)], ["Middle fu", "中伏", hc.gregorianToFixed(2026, 7, 25)], ["Last fu", "末伏", hc.gregorianToFixed(2026, 8, 14)]],
    );
    const hundstage = seasonal.find((day) => day.id === "hundstage");
    assert.deepEqual([hundstage?.first, hundstage?.last], [hc.gregorianToFixed(2026, 7, 23), hc.gregorianToFixed(2026, 8, 23)]);
    assert.equal(seasonal.filter((day) => day.kind === "quarter-day").length, 20);
    refused(() => hc.zassetsuInYear(2024, "mars"), "unknown");
    refused(() => hc.seasonalDaysInYear(3001, "japan"), "out-of-range");
  });

  test("a meridian is a name or a longitude", () => {
    const day = hc.gregorianToFixed(2024, 2, 4);
    const begins = (/** @type {string | number} */ meridian) => hc.termInEffect(day, meridian).begins;
    // 立春 2024 began at 08:27 UT on 4 February: the 4th from 120°W east
    // to Tokyo, still the 3rd at 180°W.
    assert.equal(begins("japan"), day);
    assert.equal(begins("JAPAN"), day);
    assert.equal(begins("china"), day);
    assert.equal(begins(135), day);
    assert.equal(begins("135.0"), day);
    assert.equal(begins("universal"), day);
    assert.equal(begins(""), day);
    assert.equal(begins(-120), day);
    assert.equal(begins(-180), day - 1);
    assert.equal(hc.termInEffect(day).begins, day);
    for (const bad of ["mars", "181", "nan", 181, Number.NaN]) {
      refused(() => hc.termInEffect(day, bad), "unknown");
      refused(() => hc.pentadInEffect(day, bad), "unknown");
    }
  });

  test("한식 is where KASI's 월력요항 puts it, and the Chinese reckonings a day or two apart", () => {
    assert.equal(hc.coldFoodDay("hansik", 2024), hc.gregorianToFixed(2024, 4, 5));
    assert.equal(hc.coldFoodDay("HANSIK", 2026n), hc.gregorianToFixed(2026, 4, 6));
    const gap = hc.coldFoodDay("hanshi-solstice-105", 2026) - hc.coldFoodDay("hanshi-eve-of-qingming", 2026);
    assert.ok(gap === 1 || gap === 2, `${gap}`);
    refused(() => hc.coldFoodDay(/** @type {any} */ ("hanshi"), 2026), "unknown");
    refused(() => hc.coldFoodDay("hansik", 3001), "out-of-range");
    refused(() => hc.coldFoodDay("hansik", -1000), "out-of-range");
  });
});

describe("deep time", () => {
  /**
   * @param {import("./hyper-calendar.js").DeepTimeRow[]} rows
   * @param {string[][]} raw
   */
  function sameShape(rows, raw) {
    assert.equal(rows.length, raw.length);
    for (const cells of raw) {
      assert.equal(cells.length, COLUMNS.deepTime.length, JSON.stringify(cells));
    }
  }

  test("a moment is placed in every chronology", () => {
    // The end-Cretaceous extinction, 66 million years ago.
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_place_years_ago(66.0e6, 0.0, 0, 0, buffer, capacity));
    const rows = hc.placeYearsAgo(66.0e6);
    sameShape(rows, raw);
    assert.deepEqual(
      rows.map((row) => row.kind),
      ["moment", "moment", "cosmic-epoch", "cosmic-event", "eon", "era", "period", "epoch", "age"],
    );
    assert.equal(rows[0].id, "since-big-bang");
    assert.equal(rows[0].name, "since-big-bang");
    assert.equal(rows[0].unit, "seconds-since-big-bang");
    assert.deepEqual(rows[0].start, rows[0].end, "a point in time");
    assert.equal(rows[1].name, "before-present");
    assert.equal(rows[1].unit, "seconds-before-present");
    assert.equal(rows[2].id, "era-of-galaxies");
    assert.equal(rows[2].name, "Era of galaxies");
    const age = rows[8];
    assert.equal(age.id, "maastrichtian");
    assert.equal(age.name, "Maastrichtian");
    assert.equal(age.scope, "upper-cretaceous", "the identifier one rank up");
    assert.deepEqual(age.start, { value: 72.2, stdDev: 0.2, figures: 3, approximate: false });
    assert.deepEqual(age.end, { value: 66, stdDev: 0, figures: 4, approximate: false });
    assert.equal(age.unit, "megayears-before-present");
    assert.equal(age.description, null);
    assert.match(age.source, /v2026\/06/);
    assert.equal(age.localisedName, null, "no locale, no localised name");
    const inJapanese = hc.placeYearsAgo(66.0e6, 0, "ja");
    assert.equal(inJapanese[8].localisedName, "マーストリヒチアン");
    assert.equal(inJapanese[8].name, "Maastrichtian", "the English column stays");
    assert.equal(inJapanese[2].localisedName, null, "no established term for the era of galaxies");
    assert.equal(inJapanese[3].localisedName, "太陽系の形成");
    assert.deepEqual(hc.placeYearsAgo(66.0e6, 1.0e6).map((row) => row.kind), rows.map((row) => row.kind));
  });

  test("the present and the future are placed too", () => {
    const now = hc.placeYearsAgo(0);
    const archaeological = now.find((row) => row.kind === "archaeological");
    assert.ok(archaeological);
    assert.equal(archaeological.id, "modern-period");
    assert.equal(archaeological.name, "Modern period");
    assert.equal(archaeological.unit, "years-before-1950");
    assert.ok(archaeological.source.length > 0);
    assert.equal(archaeological.start?.figures, null, "the archaeological table claims no figures");
    const ahead = hc.placeYearsAgo(-8.0e9);
    assert.deepEqual(ahead.map((row) => row.kind), ["moment", "moment", "cosmic-event", "future-era"]);
    assert.equal(ahead[2].name, "The present");
    assert.equal(ahead[3].name, "Stelliferous Era");
    assert.equal(ahead[3].unit, "log10-years-from-now");
  });

  test("the near future is still in the present intervals", () => {
    const hours = hc.placeYearsAgo(-6 / (24 * 365.25));
    const years = hc.placeYearsAgo(-3);
    for (const rows of [hours, years]) {
      assert.deepEqual(rows.map((row) => row.kind), [
        "moment", "moment", "cosmic-epoch", "cosmic-event", "future-era",
        "eon", "era", "period", "epoch", "age", "archaeological",
      ]);
      assert.equal(rows[9].name, "Meghalayan");
      assert.equal(rows[10].name, "Modern period");
    }
    assert.deepEqual(
      hc.placeYearsAgo(-101).map((row) => row.kind),
      ["moment", "moment", "cosmic-event", "future-era"],
      "beyond a century the future begins",
    );
  });

  test("a value the crate refuses is thrown", () => {
    refused(() => hc.placeYearsAgo(Number.NaN), "out-of-range");
    refused(() => hc.placeYearsAgo(1, -1), "out-of-range");
    assert.throws(() => hc.placeYearsAgo(/** @type {any} */ ("66")), TypeError);
  });

  test("the cosmic tables are listed with their sources", () => {
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_cosmic_events(0, 0, buffer, capacity));
    const rows = hc.cosmicEvents();
    sameShape(rows, raw);
    const epochs = rows.filter((row) => row.kind === "cosmic-epoch");
    const events = rows.filter((row) => row.kind === "cosmic-event");
    assert.ok(epochs.length > 5 && events.length > 5);
    assert.deepEqual(rows.slice(0, epochs.length), epochs, "epochs first");
    assert.ok(rows.every((row) => row.source.length > 0), "every row cites");
    assert.equal(rows[0].name, "Planck epoch");
    assert.equal(rows[0].start?.value, 0);
    // Values are written in plain decimal notation, however small.
    assert.ok(rows[0].end && rows[0].end.value > 0 && rows[0].end.value < 1e-40);
    assert.doesNotMatch(raw[0][8], /e/);
    assert.equal(new Set(rows.map((row) => row.id)).size, rows.length, "identifiers are unique");
    assert.ok(rows.every((row) => row.localisedName === null), "no locale, no localised name");
    const inJapanese = hc.cosmicEvents("ja");
    assert.equal(inJapanese.find((row) => row.id === "recombination")?.localisedName, "宇宙の晴れ上がり");
    assert.equal(inJapanese.find((row) => row.id === "neutrino-decoupling")?.localisedName, null);
    assert.ok(rows.every((row) => row.kind === "cosmic-epoch" || row.kind === "cosmic-event"));
  });

  test("the earliest evidence keeps the shape of each source's date", () => {
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_earliest_evidence(0, 0, buffer, capacity));
    const rows = hc.earliestEvidence("ja");
    sameShape(rows, raw);
    assert.ok(rows.every((row) => row.kind === "earliest-evidence"));
    assert.deepEqual(
      [...new Set(rows.map((row) => row.scope))],
      ["earliest-life", "earliest-homo-sapiens", "earliest-writing"],
    );
    assert.ok(rows.every((row) => row.unit === "years-before-1950"));
    const byId = (/** @type {string} */ id) => rows.find((row) => row.id === id);
    // Vidal et al. 2022: a minimum age, 233 ± 22 kyr at 2σ.
    const omo = byId("earliest-homo-sapiens-omo-kibish");
    assert.equal(omo?.start, null, "a minimum age has no older bound");
    assert.deepEqual(omo?.end, { value: 233000, stdDev: 11000, figures: 3, approximate: false });
    assert.equal(omo?.localisedName, "オモの化石");
    // Richter et al. 2017: 315 ± 34 ka, the ± not a stated σ.
    const irhoud = byId("earliest-homo-sapiens-jebel-irhoud");
    assert.deepEqual(irhoud?.start, { value: 315000, stdDev: null, figures: 3, approximate: false });
    assert.deepEqual(irhoud?.start, irhoud?.end);
    // Dodd et al. 2017: at least 3,770 and possibly 4,280 Myr.
    const nuvvuagittuq = byId("earliest-life-nuvvuagittuq");
    assert.equal(nuvvuagittuq?.start?.value, 4.28e9);
    assert.equal(nuvvuagittuq?.end?.value, 3.77e9);
    // Englund 2004: proto-cuneiform emerges ca. 3300 BC, 5249 years before 1950.
    const uruk = byId("earliest-writing-uruk-iv");
    assert.deepEqual(uruk?.end, { value: 5249, stdDev: null, figures: 2, approximate: true });
    assert.equal(uruk?.localisedName, "原楔形文字");
    assert.ok(byId("earliest-life-isua-stromatolites")?.description?.includes("Disputed"));
  });

  test("the cosmic list holds only cosmic rows, every bound with its σ", () => {
    for (const rows of [hc.cosmicEvents(), hc.archaeologicalPeriods(), hc.futureEvents(), hc.placeYearsAgo(0)]) {
      for (const row of rows) {
        assert.notEqual(row.kind, "earliest-evidence");
        for (const bound of [row.start, row.end]) {
          assert.ok(bound === null || typeof bound.stdDev === "number", row.id);
        }
      }
    }
  });

  test("the archaeological periods and the future events are listed", () => {
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_archaeological_periods(0, 0, buffer, capacity));
    const periods = hc.archaeologicalPeriods("ja");
    sameShape(periods, raw);
    assert.ok(periods.every((row) => row.kind === "archaeological" && row.unit === "years-before-1950"));
    assert.equal(periods[0].id, "modern-period");
    assert.equal(periods[0].localisedName, "近代");
    assert.equal(periods.at(-1)?.id, "lower-palaeolithic");
    const future = hc.futureEvents();
    assert.ok(future.every((row) => row.kind === "future-event" && row.unit === "years-from-now"));
    const tip = future.find((row) => row.id === "sun-red-giant-tip");
    assert.equal(tip?.scope, "modelled");
    assert.deepEqual(tip?.start, { value: 7.59e9, stdDev: 5e7, figures: 3, approximate: false });
    assert.deepEqual(tip?.start, tip?.end);
    const proton = future.find((row) => row.id === "proton-decay-lower-bound");
    assert.equal(proton?.scope, "experimental-bound");
    assert.equal(proton?.start?.value, 2.4e34);
    assert.equal(proton?.end, null, "a bound has no end");
  });

  test("the geologic ranks are numbered coarsest first", () => {
    GEOLOGIC_RANKS.forEach((name, number) => {
      const byName = hc.geologicIntervals(name);
      const byNumber = hc.geologicIntervals(number);
      assert.deepEqual(byName, byNumber, name);
      assert.ok(byName.length > 3, name);
      assert.ok(byName.every((row) => row.kind === name), name);
    });
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_geologic_intervals(0, 0, 0, buffer, capacity));
    const eons = hc.geologicIntervals("eon");
    sameShape(eons, raw);
    assert.equal(eons[0].id, "phanerozoic");
    assert.equal(eons[0].name, "Phanerozoic");
    assert.equal(eons[0].scope, null);
    assert.deepEqual(eons[0].start, { value: 538.8, stdDev: 0.6, figures: 4, approximate: false });
    assert.deepEqual(eons[0].end, { value: 0, stdDev: 0, figures: 1, approximate: false });
    assert.equal(eons[0].localisedName, null);
    assert.equal(hc.geologicIntervals("eon", "zh-Hans")[0].localisedName, "显生宇");
    assert.equal(hc.geologicIntervals("period", "de").find((row) => row.name === "Quaternary")?.localisedName, "Quartär");
    refused(() => hc.geologicIntervals(5), "unknown");
    assert.throws(() => hc.geologicIntervals(/** @type {any} */ ("eons")), TypeError);
  });
});

describe("time zones", () => {
  test("the reader's day is the zone's day, not UTC's", () => {
    // 08:00 on 25 September 2026 in Tokyo is 23:00 UTC on the 24th.
    const september24 = hc.gregorianToFixed(2026, 9, 24);
    const instant = hc.unixFromFixed(september24) + 23 * 3_600;
    assert.equal(hc.fixedFromUnix(instant), september24);
    assert.equal(hc.fixedFromUnixInZone(instant, "Asia/Tokyo"), september24 + 1);
    assert.equal(hc.fixedFromUnixInZone(BigInt(instant), "asia/tokyo"), september24 + 1);
    assert.equal(hc.fixedFromUnixInZone(instant, "UTC"), september24);
    // And the Tokyo day begins nine hours before the UTC one.
    assert.equal(hc.unixFromFixedInZone(september24 + 1, "Asia/Tokyo"), hc.unixFromFixed(september24 + 1) - 9 * 3_600);
  });

  test("a day that begins in a gap begins after it", () => {
    // New York's clocks go forward at 02:00 on 8 March 2026.
    const march8 = hc.gregorianToFixed(2026, 3, 8);
    const midnightUtc = hc.unixFromFixed(march8);
    assert.equal(hc.unixFromFixedInZone(march8, "America/New_York"), midnightUtc + 5 * 3_600);
    assert.equal(hc.fixedFromUnixInZone(midnightUtc + 7 * 3_600, "America/New_York"), march8);
    assert.equal(hc.fixedFromUnixInZone(midnightUtc + 4 * 3_600, "America/New_York"), march8 - 1);
    // Cairo's clocks go forward at 00:00 on the last Friday of April, so
    // 24 April 2026 has no midnight: it begins at 01:00 EEST.
    const april24 = hc.gregorianToFixed(2026, 4, 24);
    assert.equal(hc.unixFromFixedInZone(april24, "Africa/Cairo"), hc.unixFromFixed(april24) - 2 * 3_600);
    assert.equal(hc.unixFromFixedInZone(april24 + 1, "Africa/Cairo"), hc.unixFromFixed(april24 + 1) - 3 * 3_600);
  });

  test("an unknown zone is refused by name", () => {
    const error = refused(() => hc.fixedFromUnixInZone(0, "Mars/Olympus"), "unknown");
    assert.equal(error.constant, "HC_ERR_UNKNOWN");
    assert.equal(error.code, -9_000_000_000_000_006n);
    assert.equal(error.export, "hc_fixed_from_unix_in_zone");
    refused(() => hc.unixFromFixedInZone(0, ""), "unknown");
    assert.throws(() => hc.fixedFromUnixInZone(0, /** @type {any} */ (undefined)), TypeError);
  });

  test("a loaded zone answers by name and outranks the built-in", async () => {
    // A fresh instance, so the zones loaded here stay here.
    const fresh = await load(bytes);
    const name = "Test/Eastern";
    refused(() => fresh.fixedFromUnixInZone(0, name), "unknown");
    fresh.loadZone(name, TZIF_V2_EASTERN);
    // 2025-03-09 07:00 UTC is 03:00 EDT, just after the gap.
    const march9 = fresh.gregorianToFixed(2025, 3, 9);
    const midnightUtc = fresh.unixFromFixed(march9);
    assert.equal(fresh.fixedFromUnixInZone(midnightUtc + 7 * 3_600, name), march9);
    assert.equal(fresh.fixedFromUnixInZone(midnightUtc + 4 * 3_600, name), march9 - 1);
    assert.equal(fresh.unixFromFixedInZone(march9, name), midnightUtc + 5 * 3_600);
    // The same bytes under a built-in name take precedence over it.
    const builtin = "America/Los_Angeles";
    assert.equal(fresh.unixFromFixedInZone(march9, builtin), midnightUtc + 8 * 3_600);
    fresh.loadZone(builtin, TZIF_V2_EASTERN.buffer.slice(TZIF_V2_EASTERN.byteOffset, TZIF_V2_EASTERN.byteOffset + TZIF_V2_EASTERN.byteLength));
    assert.equal(fresh.unixFromFixedInZone(march9, builtin), midnightUtc + 5 * 3_600);
    // Bytes that are not TZif are refused and nothing is kept.
    refused(() => fresh.loadZone("Test/Junk", new TextEncoder().encode("not a zone")), "malformed");
    refused(() => fresh.fixedFromUnixInZone(0, "Test/Junk"), "unknown");
    refused(() => fresh.loadZone("", TZIF_V2_EASTERN), "unknown");
    assert.throws(() => fresh.loadZone(name, /** @type {any} */ ("bytes")), TypeError);
  });
});

describe("a zone's offset", () => {
  test("the 2026 changes of Berlin, built in, and of Denver, loaded", async () => {
    const fresh = await load(bytes);
    fresh.loadZone("America/Denver", TZIF_V2_DENVER);
    const line = (/** @type {string} */ zone, /** @type {number} */ unix) => fresh.zoneOffset(zone, unix);
    // Europe/Berlin: 29 March and 25 October 2026 at 01:00 UTC.
    assert.deepEqual(line("Europe/Berlin", 1_774_745_999), {
      offsetSeconds: 3_600, dst: false, abbreviation: "CET",
      nextTransition: 1_774_746_000, nextOffsetSeconds: 7_200, rules: "builtin",
    });
    assert.deepEqual(line("Europe/Berlin", 1_774_746_000), {
      offsetSeconds: 7_200, dst: true, abbreviation: "CEST",
      nextTransition: 1_792_890_000, nextOffsetSeconds: 3_600, rules: "builtin",
    });
    assert.equal(line("Europe/Berlin", 1_792_889_999).abbreviation, "CEST");
    assert.equal(line("Europe/Berlin", 1_792_890_000).nextTransition, 1_806_195_600);
    // America/Denver: 8 March 09:00 UTC and 1 November 08:00 UTC.
    assert.deepEqual(line("America/Denver", 1_772_960_399), {
      offsetSeconds: -25_200, dst: false, abbreviation: "MST",
      nextTransition: 1_772_960_400, nextOffsetSeconds: -21_600, rules: "loaded",
    });
    assert.deepEqual(line("America/Denver", 1_772_960_400), {
      offsetSeconds: -21_600, dst: true, abbreviation: "MDT",
      nextTransition: 1_793_520_000, nextOffsetSeconds: -25_200, rules: "loaded",
    });
    assert.equal(line("America/Denver", 1_793_519_999).abbreviation, "MDT");
    assert.equal(line("America/Denver", 1_793_520_000).nextTransition, 1_805_014_800);
    // The day in the zone is the instant moved by the offset.
    for (const unix of [1_772_960_399, 1_772_960_400, 1_793_519_999, 1_793_520_000]) {
      const { offsetSeconds } = line("America/Denver", unix);
      assert.equal(fresh.fixedFromUnixInZone(unix, "America/Denver"), fresh.fixedFromUnix(unix + offsetSeconds));
    }
    // Unloaded, Denver is the built-in table's, with the same current rules.
    assert.deepEqual(hc.zoneOffset("America/Denver", 1_772_960_400), {
      offsetSeconds: -21_600, dst: true, abbreviation: "MDT",
      nextTransition: 1_793_520_000, nextOffsetSeconds: -25_200, rules: "builtin",
    });
  });

  test("a zone without summer time has no next transition", () => {
    assert.deepEqual(hc.zoneOffset("Asia/Kathmandu", 0n), {
      offsetSeconds: 20_700, dst: false, abbreviation: null,
      nextTransition: null, nextOffsetSeconds: null, rules: "builtin",
    });
    assert.equal(hc.zoneOffset("Asia/Tokyo", 0).abbreviation, "JST");
    assert.equal(hc.zoneOffset("Asia/Tokyo", 0).nextTransition, null);
    refused(() => hc.zoneOffset("Mars/Olympus", 0), "unknown");
  });

  test("a radio frame reads its summer time from the zone", async () => {
    const fresh = await load(bytes);
    fresh.loadZone("America/Denver", TZIF_V2_DENVER);
    // The frame announcing 03:00 CEST on 29 March 2026 carries A1.
    assert.equal(
      fresh.radioEncode("dcf77", 1_774_746_000, { summer: "zone:Europe/Berlin" }),
      fresh.radioEncode("dcf77", 1_774_746_000, { summer: "cest", zoneChange: true }),
    );
    // Bit 57 from 00:00 UTC on 8 March 2026, the day of the change.
    assert.equal(
      fresh.radioEncode("wwvb-am", 1_772_928_000, { summer: "zone:America/Denver" }),
      fresh.radioEncode("wwvb-am", 1_772_928_000, { summer: "begins-today" }),
    );
    // From that instant the phase code's dst_next names 1 November, 011011.
    assert.equal(
      fresh.radioEncode("wwvb-pm", 1_772_928_000, { summer: "zone:America/Denver" }),
      fresh.radioEncode("wwvb-pm", 1_772_928_000, { summer: "begins-today", dstNext: 27 }),
    );
    refused(() => fresh.radioEncode("dcf77", 1_774_746_000, { summer: "zone:Europe/London" }), "out-of-range");
  });
});

describe("where each zone is", () => {
  test("every zone has a location, and Tokyo's is its row in decimal degrees", () => {
    const zones = hc.zones("en");
    assert.equal(zones.length, 312);
    // zone1970.tab 2026d: JP,AU +353916+1394441 Asia/Tokyo; 35° 39′ 16″ is
    // 128 356″ and 139° 44′ 41″ is 503 081″, written to six places.
    const tokyo = zones.find((zone) => zone.zone === "Asia/Tokyo");
    assert.deepEqual(tokyo, {
      zone: "Asia/Tokyo",
      latitude: 35.654444,
      longitude: 139.744722,
      countries: ["JP", "AU"],
      country: "JP",
      comment: "Eyre Bird Observatory",
      exemplarCity: "Tokyo",
      localeUsed: "en",
    });
    assert.deepEqual(hc.zoneLocation("asia/tokyo", "en"), tokyo);
    assert.equal(zones.find((zone) => zone.zone === "Europe/Andorra")?.comment, null);
    assert.equal(Math.round(tokyo.latitude * 3_600), 128_356);
    assert.equal(Math.round(tokyo.longitude * 3_600), 503_081);
    assert.equal(zones.find((zone) => zone.zone === "America/Sao_Paulo")?.latitude, -23.533333);
    for (const zone of zones) {
      assert.ok(Math.abs(zone.latitude) <= 90 && Math.abs(zone.longitude) <= 180, zone.zone);
      // Six places identify the whole arcsecond: times 3600, each is
      // within 0.0018″ of one.
      for (const degrees of [zone.latitude, zone.longitude]) {
        const arcseconds = degrees * 3_600;
        assert.ok(Math.abs(arcseconds - Math.round(arcseconds)) < 0.002, `${zone.zone} ${degrees}`);
      }
      assert.ok(zone.exemplarCity.length > 0, zone.zone);
      // zone.tab 2026d lists every zone, under one of its countries.
      assert.ok(zone.country !== null && zone.countries.includes(zone.country), zone.zone);
    }
    // zone.tab: UA +4457+03406 Europe/Simferopol, in its RU section, where
    // zone1970.tab has RU,UA.
    const simferopol = zones.find((zone) => zone.zone === "Europe/Simferopol");
    assert.deepEqual([simferopol?.countries, simferopol?.country], [["RU", "UA"], "UA"]);
  });

  test("a link answers with its own row or the row it leads to, and UTC with none", () => {
    // zone.tab: NO +5955+01045 Europe/Oslo; backward: Link Asia/Kolkata Asia/Calcutta.
    const oslo = hc.zoneLocation("Europe/Oslo");
    assert.equal(oslo.zone, "Europe/Oslo");
    assert.equal(oslo.latitude, 59.916667);
    assert.equal(oslo.longitude, 10.75);
    assert.deepEqual(oslo.countries, ["NO"]);
    assert.equal(oslo.country, "NO");
    assert.equal(hc.zoneLocation("Asia/Calcutta").zone, "Asia/Kolkata");
    const error = refused(() => hc.zoneLocation("UTC"), "unknown");
    assert.equal(error.export, "hc_zone_location");
    assert.throws(() => hc.zoneLocation(/** @type {any} */ (undefined)), TypeError);
  });

  test("the city is the locale's, with the tag that answered", () => {
    // CLDR 48 ja.xml: Asia/Tokyo 東京; de.xml: Europe/Berlin ↑↑↑, root's Berlin.
    assert.equal(hc.zoneLocation("Asia/Tokyo", "ja-JP").exemplarCity, "東京");
    assert.equal(hc.zoneLocation("Asia/Tokyo", "ja-JP").localeUsed, "ja");
    assert.deepEqual(
      [hc.zoneLocation("Europe/Berlin", "de").exemplarCity, hc.zoneLocation("Europe/Berlin", "de").localeUsed],
      ["Berlin", "de"],
    );
    assert.equal(hc.zoneLocation("Asia/Tokyo", "kab").localeUsed, "en");
  });
});

describe("the sky", () => {
  /** The POSIX timestamp of a UTC date and time. */
  const at = (/** @type {number} */ year, /** @type {number} */ month, /** @type {number} */ day, hour = 0, minute = 0) =>
    hc.unixFromFixed(hc.gregorianToFixed(year, month, day)) + hour * 3_600 + minute * 60;

  test("the sky before the new moon of September 2026 decodes column by column", () => {
    // The 暦要項 of the National Astronomical Observatory of Japan puts the
    // new moon (朔) of September 2026 at 11 September 12:27 JST, 03:27 UTC.
    const instant = at(2026, 9, 11, 3, 0);
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_sky_at(BigInt(instant), buffer, capacity));
    assert.equal(raw.length, 1);
    assert.equal(raw[0].length, COLUMNS.sky.length);
    const sky = hc.skyAt(instant);
    assert.deepEqual(hc.skyAt(BigInt(instant)), sky);
    assert.ok(sky.sunLongitude > 167 && sky.sunLongitude < 170, `${sky.sunLongitude}`);
    assert.ok(sky.sunDistance > 1.0 && sky.sunDistance < 1.02, `${sky.sunDistance} au`);
    assert.ok(Math.abs(sky.sunLongitude - sky.moonLongitude) < 1, `${sky.moonLongitude}`);
    assert.ok(Math.abs(sky.moonLatitude) < 5.5);
    assert.ok(sky.moonDistance > 355_000 && sky.moonDistance < 407_000, `${sky.moonDistance} km`);
    assert.ok(sky.elongation > 359, `${sky.elongation}`);
    assert.ok(sky.illuminatedFraction < 0.001);
    const published = at(2026, 9, 11, 3, 27);
    assert.ok(Math.abs(sky.nextNewMoon - published) <= 90, `${sky.nextNewMoon}`);
    assert.ok(sky.previousNewMoon < instant && instant < sky.nextNewMoon);
    const lunation = (sky.nextNewMoon - sky.previousNewMoon) / 86_400;
    assert.ok(lunation >= 29 && lunation < 30, `${lunation} days`);
    assert.ok(sky.deltaT > 69 && sky.deltaT < 69.5, `${sky.deltaT}`);
    assert.equal(sky.deltaTRegime, "predicted");
    assert.match(sky.source, /VSOP87/);
    assert.match(sky.source, /deltat\.preds/);
    // And the new moon is where the phase list puts it, to the second.
    const phases = hc.moonPhasesBetween(at(2026, 9, 1), at(2026, 10, 1));
    const newMoon = phases.find((phase) => phase.name === "new");
    assert.equal(newMoon?.instant, sky.nextNewMoon);
    assert.equal(hc.skyAt(sky.nextNewMoon + 60).previousNewMoon, sky.nextNewMoon);
  });

  test("the terms and phases of September 2026 decode column by column", () => {
    const from = at(2026, 9, 1);
    const to = at(2026, 10, 1);
    const raw = rawRows(hc, (buffer, capacity) =>
      hc.exports.hc_solar_terms_between(BigInt(from), BigInt(to), buffer, capacity));
    assert.ok(raw.every((cells) => cells.length === COLUMNS.skyEvent.length), JSON.stringify(raw));
    const terms = hc.solarTermsBetween(from, to);
    assert.equal(terms.length, raw.length);
    assert.deepEqual(terms.map((term) => [term.angle, term.name, term.japaneseName]), [[165, "白露", "白露"], [180, "秋分", "秋分"]]);
    // 秋分 at 23 September 09:05 JST, 00:05 UTC.
    assert.ok(Math.abs(terms[1].instant - at(2026, 9, 23, 0, 5)) <= 90, `${terms[1].instant}`);
    assert.deepEqual(hc.solarTermsBetween(BigInt(from), BigInt(to)), terms);
    const phases = hc.moonPhasesBetween(from, to);
    assert.deepEqual(phases.map((phase) => [phase.angle, phase.name, phase.japaneseName]), [
      [270, "last-quarter", null],
      [0, "new", null],
      [90, "first-quarter", null],
      [180, "full", null],
    ]);
    assert.ok(phases.every((phase, index) => index === 0 || phase.instant > phases[index - 1].instant), "in time order");
    // 望 at 27 September 01:49 JST, 16:49 UTC on the 26th.
    assert.ok(Math.abs(phases[3].instant - at(2026, 9, 26, 16, 49)) <= 90, `${phases[3].instant}`);
    assert.ok(hc.skyAt(phases[3].instant).illuminatedFraction > 0.99);
  });

  test("the lunation of March 2023 holds no principal term", () => {
    // The Chinese calendar of 2023 had a leap second month (閏二月): the
    // lunation from the new moon of 21 March 17:23 UTC to that of 20 April
    // 04:12 UTC held only the sectional term 清明, no 中気.
    const newMoon = hc.skyAt(at(2023, 3, 21)).nextNewMoon;
    assert.ok(Math.abs(newMoon - at(2023, 3, 21, 17, 23)) <= 90, `${newMoon}`);
    const nextNewMoon = hc.skyAt(newMoon + 1).nextNewMoon;
    assert.ok(Math.abs(nextNewMoon - at(2023, 4, 20, 4, 12)) <= 90, `${nextNewMoon}`);
    const terms = hc.solarTermsBetween(newMoon, nextNewMoon);
    assert.deepEqual(terms.map((term) => [term.angle, term.name, term.japaneseName]), [[15, "清明", "清明"]]);
    assert.ok(terms.every((term) => term.angle % 30 !== 0), "a principal term");
    assert.ok(terms[0].instant >= newMoon && terms[0].instant < nextNewMoon);
  });

  test("instants outside the era and spans too long are refused, and an empty span is empty", () => {
    assert.ok(hc.skyAt(at(3000, 12, 31, 23, 59)).sunLongitude >= 0);
    assert.ok(hc.skyAt(at(-1000, 1, 1)).sunLongitude >= 0);
    for (const instant of [at(3001, 1, 1), at(-1000, 1, 1) - 1, 2n ** 63n - 1n, -(2n ** 63n)]) {
      const error = refused(() => hc.skyAt(instant), "out-of-range");
      assert.equal(error.constant, "HC_ERR_OUT_OF_RANGE");
      assert.equal(error.export, "hc_sky_at");
      refused(() => hc.solarTermsBetween(instant, typeof instant === "bigint" ? instant : instant + 1), "out-of-range");
    }
    refused(() => hc.moonPhasesBetween(at(2000, 1, 1), at(3001, 1, 1, 0, 1)), "out-of-range");
    // A span may end at the era's end, but not run past it, nor exceed 400 years.
    const end = at(3001, 1, 1);
    assert.ok(hc.solarTermsBetween(end - 86_400 * 40, end).length > 0);
    refused(() => hc.solarTermsBetween(end - 86_400 * 40, end + 1), "out-of-range");
    refused(() => hc.moonPhasesBetween(at(2000, 1, 1), at(2401, 1, 1)), "out-of-range");
    assert.deepEqual(hc.solarTermsBetween(at(2026, 9, 1), at(2026, 9, 1)), []);
    assert.deepEqual(hc.moonPhasesBetween(at(2026, 10, 1), at(2026, 9, 1)), []);
    assert.throws(() => hc.solarTermsBetween(1.5, 2), TypeError);
  });
});

describe("the orbit", () => {
  test("the Last Glacial Maximum decodes column by column", () => {
    // The author's own table (bein1.dat) and the PMIP experiments put
    // 21 000 years before 1950 at e = 0.018994, ϖ = 114.42°, ε = 22.949°
    // and e sin ϖ = 0.01729, the anchors hc-orbital's own tests pin.
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_orbit_at(21_000, buffer, capacity));
    assert.equal(raw.length, 1);
    assert.equal(raw[0].length, COLUMNS.orbit.length);
    const lgm = hc.orbitAt(21_000);
    assert.ok(Math.abs(lgm.eccentricity - 0.018994) < 1e-6, `${lgm.eccentricity}`);
    assert.equal(lgm.eccentricitySpread, 0.002);
    assert.ok(Math.abs(lgm.obliquity - 22.949) < 1e-3, `${lgm.obliquity}`);
    assert.equal(lgm.obliquitySpread, 0.05);
    assert.ok(Math.abs(lgm.longitudeOfPerihelion - 114.42) < 0.01, `${lgm.longitudeOfPerihelion}`);
    // asin(0.0025 / 0.018994), in degrees.
    assert.ok(Math.abs(lgm.longitudeOfPerihelionSpread - 7.56) < 0.01, `${lgm.longitudeOfPerihelionSpread}`);
    assert.ok(Math.abs(lgm.climaticPrecession - 0.01729) < 5e-6, `${lgm.climaticPrecession}`);
    assert.equal(lgm.climaticPrecessionSpread, 0.0025);
    assert.equal(lgm.solarConstant, 1360);
    assert.match(lgm.source, /Berger, A\. \(1978\)/);
    assert.match(lgm.source, /SOLAR_CONSTANT_BERGER_LOUTRE_1991/);
    // The June insolation at 65° N: 477.6 W/m² at 1950, lower at the
    // glacial maximum, and 45 W/m² higher in the early Holocene.
    const now = hc.orbitAt(0);
    assert.ok(Math.abs(now.insolation65NJune - 477.6) < 0.1, `${now.insolation65NJune}`);
    assert.ok(lgm.insolation65NJune < now.insolation65NJune);
    assert.ok(hc.orbitAt(11_000).insolation65NJune - now.insolation65NJune > 45);
    // The future is negative years; the spread widens with distance.
    const far = hc.orbitAt(-900_000);
    assert.equal(far.eccentricitySpread, 0.01);
    assert.equal(far.obliquitySpread, 0.27);
  });

  test("a series is the single lines with the epoch first", () => {
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_orbit_series(0, 21_000, 1_000, buffer, capacity));
    assert.equal(raw.length, 22);
    assert.ok(raw.every((cells) => cells.length === COLUMNS.orbitSeries.length), JSON.stringify(raw[0]));
    const series = hc.orbitSeries(0, 21_000, 1_000);
    assert.equal(series.length, 22);
    assert.deepEqual(series.map((sample) => sample.yearsBefore1950), Array.from({ length: 22 }, (_, kyr) => kyr * 1_000));
    const { yearsBefore1950: first, ...head } = series[0];
    assert.equal(first, 0);
    assert.deepEqual(head, hc.orbitAt(0));
    const { yearsBefore1950: last, ...tail } = series[21];
    assert.equal(last, 21_000);
    assert.deepEqual(tail, hc.orbitAt(21_000));
    // A step that does not divide the span stops before `to`.
    assert.deepEqual(hc.orbitSeries(0, 1_000, 300).map((sample) => sample.yearsBefore1950), [0, 300, 600, 900]);
    // The whole span at the widest step that fits the cap, and one sample
    // when the ends coincide.
    assert.equal(hc.orbitSeries(-1_000_000, 1_000_000, 200.01).length, 10_000);
    assert.equal(hc.orbitSeries(-50, -50, 1).length, 1);
    assert.deepEqual(hc.orbitSeries(1_000, 0, 100), []);
  });

  test("epochs off the span and series too long are refused", () => {
    assert.ok(hc.orbitAt(1_000_000).eccentricity > 0);
    assert.ok(hc.orbitAt(-1_000_000).eccentricity > 0);
    for (const epoch of [1_000_001, -1_000_001, Number.NaN, Number.POSITIVE_INFINITY]) {
      const error = refused(() => hc.orbitAt(epoch), "out-of-range");
      assert.equal(error.constant, "HC_ERR_OUT_OF_RANGE");
      assert.equal(error.export, "hc_orbit_at");
    }
    refused(() => hc.orbitSeries(0, 1_000_001, 100), "out-of-range");
    refused(() => hc.orbitSeries(-1_000_001, 0, 100), "out-of-range");
    refused(() => hc.orbitSeries(0, 1_000, 0), "out-of-range");
    refused(() => hc.orbitSeries(0, 1_000, -100), "out-of-range");
    refused(() => hc.orbitSeries(0, 1_000, Number.NaN), "out-of-range");
    // 10 001 samples.
    refused(() => hc.orbitSeries(0, 10_000, 1), "out-of-range");
    refused(() => hc.orbitSeries(-1_000_000, 1_000_000, 200), "out-of-range");
    assert.throws(() => hc.orbitAt(/** @type {any} */ ("21000")), TypeError);
    assert.throws(() => hc.orbitSeries(0, 1_000, /** @type {any} */ (undefined)), TypeError);
  });
});

describe("time scales and day counts", () => {
  test("the TAI bridge names the leap second at the end of 2016", () => {
    // 23:59:59 UTC was TAI + 36 s; TAI second 1 483 228 836 is 23:59:60.
    const newYear = 1_483_228_800;
    assert.deepEqual(hc.taiFromUnix(newYear, true), { seconds: BigInt(newYear + 37), attoseconds: 0n });
    assert.deepEqual(hc.utcFromTai(newYear + 36, true), { unixSeconds: BigInt(newYear), leapSecond: true });
    assert.deepEqual(hc.utcFromTai(BigInt(newYear + 37)), { unixSeconds: BigInt(newYear), leapSecond: false });
    refused(() => hc.taiFromUnix(-400_000_000, true), "no-data");
    refused(() => hc.taiFromUnix(2n ** 63n - 1n), "out-of-range");
  });

  test("a label on a POSIX clock is 2⁶² + 10 + the POSIX second, daemontools' convention", () => {
    assert.equal(hc.tai64PosixPlus10Encode(0, 0, "tai64"), "400000000000000a");
    assert.equal(hc.tai64PosixPlus10Encode(1, 500_000_000_000_000_000n, "tai64n"), "400000000000000b1dcd6500");
    assert.deepEqual(hc.tai64PosixPlus10Decode("400000000000000A"), { format: "tai64", seconds: 0n, attoseconds: 0n });
    // The same bytes read as true TAI are ten seconds after 1970 TAI.
    assert.deepEqual(hc.tai64Decode("400000000000000a"), { format: "tai64", seconds: 10n, attoseconds: 0n });
    refused(() => hc.tai64PosixPlus10Encode(0, 0, /** @type {any} */ ("tai64na")), "unknown");
    refused(() => hc.tai64PosixPlus10Decode("3fffffffffffffff3b9ac9ff3b9ac9ff"), "malformed");
  });

  test("RFC 9562's version 1 and version 6 vectors carry the same timestamp", () => {
    const expected = { timestamp: 0x1EC9414C232AB00n, unixSeconds: 1_645_557_742n, attoseconds: 0n };
    assert.deepEqual(hc.uuidTimestamp("C232AB00-9414-11EC-B3C8-9F6BDECED846"), { version: 1, ...expected });
    assert.deepEqual(hc.uuidTimestamp("urn:uuid:1ec9414c-232a-6b00-b3c8-9f6bdeced846"), { version: 6, ...expected });
    refused(() => hc.uuidTimestamp("919108f7-52d1-4320-9bac-f847db4148a8"), "no-data");
    refused(() => hc.uuidTimestamp("not a uuid"), "malformed");
  });
  test("RFC 9562's example instant encodes to its vectors' time fields, and Figure 4's dates to NTP", () => {
    assert.deepEqual(hc.uuidTimestampEncode(1_645_557_742), {
      timestamp: 0x1EC9414C232AB00n, v1: "c232ab00-9414-11ec", v6: "1ec9414c-232a-6b00",
    });
    const back = hc.uuidTimestampEncode(1_645_557_742n, 0n);
    assert.equal(hc.uuidTimestamp(`${back.v6}-8000-000000000000`).timestamp, back.timestamp);
    refused(() => hc.uuidTimestampEncode(-12_219_292_801), "out-of-range");
    refused(() => hc.uuidTimestampEncode(0, 10n ** 18n), "out-of-range");
    assert.deepEqual(hc.ntpEncode(0), {
      era: 0, offset: 2_208_988_800, fraction: 0n,
      date: "0000000083aa7e800000000000000000", timestamp: "83aa7e8000000000",
    });
    const era1 = hc.ntpEncode(2_086_041_600);
    assert.deepEqual([era1.era, era1.offset, era1.timestamp], [1, 63_104, "0000f68000000000"]);
    assert.equal(hc.ntpEncode(0, 500_000_000_000_000_000n).fraction, 2n ** 63n);
    refused(() => hc.ntpEncode(2n ** 63n - 1n), "out-of-range");
  });


  test("an NTP timestamp takes its era from the reference, as RFC 5905's Figure 4 has it", () => {
    assert.deepEqual(hc.ntpResolve(63_104, 0, 1_893_456_000), {
      era: 1, offset: 63_104, fraction: 0n, unixSeconds: 2_086_041_600n, attoseconds: 0n,
    });
    assert.deepEqual(hc.ntpResolve(63_104, 2 ** 31, -1_577_923_200), {
      era: 0, offset: 63_104, fraction: 2n ** 63n, unixSeconds: -2_208_925_696n, attoseconds: 500_000_000_000_000_000n,
    });
    refused(() => hc.ntpResolve(0, 0, 0), "no-data");
    assert.throws(() => hc.ntpResolve(-1, 0, 0), TypeError);
  });

  test("the FAT words of 26 September 2026 at 23:59:58 are 23 866 and 49 021", () => {
    const day = hc.gregorianToFixed(2026, 9, 26);
    assert.deepEqual(hc.fatDecode(23_866, 49_021), { fixed: day, secondsOfDay: 86_398 });
    assert.deepEqual(hc.fatEncode(day, 86_399), { date: 23_866, time: 49_021 });
    refused(() => hc.fatDecode((46 << 9) | (2 << 5) | 30, 0), "invalid-date");
    refused(() => hc.fatDecode(65_536, 0), "out-of-range");
    refused(() => hc.fatEncode(hc.gregorianToFixed(2108, 1, 1), 0), "out-of-range");
  });

  test("Swatch's @248 is 04:57:07.2 UTC, and the POSIX epoch @041", () => {
    assert.equal(hc.swatchBeat(4 * 3_600 + 57 * 60 + 7, 200_000_000_000_000_000n), 248);
    assert.equal(hc.swatchBeat(0), 41);
    assert.equal(hc.swatchBeat(-3_600), 0);
  });

  test("SOFA's example is J2015.1349933196 and B2015.1365941021", () => {
    // JD 2457073.05631 TT is 1 424 352 065.184 s of TT from 1970.
    const julian = hc.epochFromTt("J", 1_424_352_065, 184_000_000_000_000_000n);
    assert.equal(julian.notation, "J");
    assert.ok(Math.abs(julian.epoch - 2015.1349933196) < 1e-10, `${julian.epoch}`);
    const besselian = hc.epochFromTt("besselian-epoch", 1_424_352_065, 184_000_000_000_000_000n);
    assert.equal(besselian.notation, "B");
    assert.ok(Math.abs(besselian.epoch - 2015.1365941021) < 1e-10, `${besselian.epoch}`);
    assert.deepEqual(hc.epochFromTt("J", 946_728_000), { notation: "J", epoch: 2000 });
    assert.deepEqual(hc.ttFromEpoch("", 2000), { notation: "J", seconds: 946_728_000n, attoseconds: 0n });
    assert.equal(hc.ttFromEpoch("", 1950).notation, "B");
    refused(() => hc.epochFromTt(/** @type {any} */ ("X"), 0), "unknown");
    refused(() => hc.ttFromEpoch("J", Number.NaN), "out-of-range");
  });

  test("a TAI64 label is 2⁶² plus the TAI second, as Bernstein's page has it", () => {
    assert.equal(hc.tai64Encode(0, 0, "tai64"), "4000000000000000");
    assert.equal(hc.tai64Encode(0n, 0n, "TAI64N"), "4000000000000000" + "00000000");
    const last = hc.tai64Encode(-1, 999_999_999_999_999_999n, "tai64na");
    assert.equal(last, "3fffffffffffffff3b9ac9ff3b9ac9ff");
    assert.deepEqual(hc.tai64Decode(last.toUpperCase()), {
      format: "tai64na", seconds: -1n, attoseconds: 999_999_999_999_999_999n,
    });
    // TAI64 names the second that contains the instant: the floor.
    assert.deepEqual(hc.tai64Decode(hc.tai64Encode(1, 5n, "tai64")), { format: "tai64", seconds: 1n, attoseconds: 0n });
    refused(() => hc.tai64Decode("400000000000000"), "malformed");
    refused(() => hc.tai64Decode("8000000000000000"), "out-of-range");
    refused(() => hc.tai64Encode(0, 0, /** @type {any} */ ("tai32")), "unknown");
    refused(() => hc.tai64Encode(0, 10n ** 18n, "tai64"), "out-of-range");
    assert.throws(() => hc.tai64Encode(0, -1, "tai64"), TypeError);
  });

  test("GPS week 2048 began at 23:59:42 UTC on 6 April 2019", () => {
    // POSIX 1 554 595 182 with TAI − UTC 37 s: 1 554 595 219 TAI seconds.
    const tai = 1_554_595_219;
    assert.deepEqual(hc.gnssWeek("gps-lnav-week", tai), { week: 2048, broadcastWeek: 0, towSeconds: 0, towAttoseconds: 0n });
    assert.deepEqual(hc.gnssWeek("gps-cnav-week", tai - 1).broadcastWeek, 2047);
    assert.deepEqual(hc.gnssToTai("gps-lnav-week", 2048, 0), { seconds: BigInt(tai), attoseconds: 0n });
    assert.equal(hc.gnssResolveWeek("gps-lnav-week", 0, "not-before", tai), 2048);
    assert.equal(hc.gnssResolveWeek("gps-lnav-week", 0, "nearest", tai), 2048);
    assert.equal(hc.gnssResolveWeek("gps-lnav-week", 1023, "nearest", tai), 2047);
    assert.equal(hc.gnssResolveWeek("gps-lnav-week", 1023, "not-before", tai), 3071);
    refused(() => hc.gnssResolveWeek("gps-lnav-week", 1024, "nearest", tai), "out-of-range");
    refused(() => hc.gnssResolveWeek("gps-lnav-week", 0, /** @type {any} */ ("latest"), tai), "unknown");
    refused(() => hc.gnssWeek(/** @type {any} */ ("gps"), tai), "unknown");
    refused(() => hc.gnssWeek("galileo-week", 0), "no-data");
    refused(() => hc.gnssToTai("gps-lnav-week", 0, 604_800), "out-of-range");
  });

  test("GLONASS counts 2024 as the first day of its eighth four-year interval", () => {
    // 2024-01-01 00:00 UTC is 03:00 GLONASS: N4 = 8, the interval from
    // 2024, and N_T = 1.
    const tai = 1_704_067_200 + 37;
    assert.deepEqual(hc.glonassDate(tai), { fourYearInterval: 8, day: 1 });
    assert.deepEqual(hc.glonassDate(tai, 0, true), { fourYearInterval: 8, day: 1 });
    refused(() => hc.glonassDate(0), "out-of-range");
  });

  test("OLE Automation dates are Microsoft Learn's examples", () => {
    const newYear1900 = hc.gregorianToFixed(1900, 1, 1);
    assert.deepEqual(hc.fixedFromOleAutomation(2.25), { fixed: newYear1900, secondsOfDay: 21_600 });
    assert.deepEqual(hc.fixedFromOleAutomation(-1.25), { fixed: newYear1900 - 3, secondsOfDay: 21_600 });
    assert.equal(hc.oleAutomationFromFixed(newYear1900 - 3, 21_600), -1.25);
    assert.equal(hc.oleAutomationFromFixed(newYear1900 - 2), 0);
    refused(() => hc.fixedFromOleAutomation(Number.NaN), "out-of-range");
    refused(() => hc.oleAutomationFromFixed(newYear1900, 86_400), "out-of-range");
  });

  test("Excel's serial 60 is named, never dated", () => {
    assert.deepEqual(hc.excel1900Day(1), { fixed: hc.gregorianToFixed(1900, 1, 1), phantom: false });
    assert.deepEqual(hc.excel1900Day(59), { fixed: hc.gregorianToFixed(1900, 2, 28), phantom: false });
    assert.deepEqual(hc.excel1900Day(60), { fixed: null, phantom: true });
    assert.deepEqual(hc.excel1900Day(61), { fixed: hc.gregorianToFixed(1900, 3, 1), phantom: false });
    assert.deepEqual(hc.excel1900Day(2_958_465), { fixed: hc.gregorianToFixed(9999, 12, 31), phantom: false });
    refused(() => hc.excel1900Day(0), "out-of-range");
    refused(() => hc.excel1900Day(2_958_466), "out-of-range");
  });
});

describe("the pañcāṅga and the anniversaries", () => {
  // Drik Panchang for 1 January 2025: "Yoga Vyaghata upto 05:07 PM" and
  // "Karana Balava upto 02:55 PM", IST, read at sunrise.
  const vyaghataEnds = Date.UTC(2025, 0, 1, 11, 37) / 1000;
  const balavaEnds = Date.UTC(2025, 0, 1, 9, 25) / 1000;

  test("the book's ayanamsa crosses the boundary by its identifier", () => {
    const day = hc.gregorianToFixed(2025, 1, 1);
    const [yoga, karana] = hc.panchangaOfDay(day, 23.183_333, 82.5, 0, "reingold-dershowitz");
    assert.equal(yoga.ayanamsa, "reingold-dershowitz");
    assert.equal(yoga.name, "Vyaghata");
    assert.equal(karana.name, "Balava");
  });

  test("1 January 2025 carries Vyaghata and Balava", () => {
    const day = hc.gregorianToFixed(2025, 1, 1);
    const [yoga, karana, ...rest] = hc.panchangaOfDay(day, 23.183_333, 82.5, 0, "Lahiri");
    assert.equal(rest.length, 0);
    assert.equal(yoga.limb, "yoga");
    assert.equal(yoga.number, 13);
    assert.equal(yoga.name, "Vyaghata");
    assert.equal(yoga.devanagari, "व्याघात");
    assert.equal(yoga.ayanamsa, "lahiri");
    assert.equal(yoga.ayanamsaName, "Lahiri (Chitrapaksha)");
    assert.ok(Math.abs(yoga.ends - vyaghataEnds) < 90, `${yoga.ends}`);
    assert.ok(yoga.began < yoga.readAt && yoga.readAt < yoga.ends);
    assert.equal(karana.limb, "karana");
    assert.equal(karana.name, "Balava");
    assert.equal(karana.devanagari, "बालव");
    assert.equal(karana.ayanamsa, null);
    assert.equal(karana.ayanamsaName, null);
    assert.ok(karana.ends - balavaEnds >= 0 && karana.ends - balavaEnds < 120, `${karana.ends}`);
    assert.equal(karana.readAt, yoga.readAt);
    // The listing's identifier is the lookup's.
    const [at] = hc.panchangaAt(vyaghataEnds - 600, yoga.ayanamsa);
    assert.equal(at.name, "Vyaghata");
    assert.equal(at.readAt, vyaghataEnds - 600);
    refused(() => hc.panchangaAt(vyaghataEnds, ""), "unknown");
    refused(() => hc.panchangaAt(vyaghataEnds, "Lahiri (Chitrapaksha)"), "unknown");
    refused(() => hc.panchangaOfDay(day, 89, 0, 0, "Lahiri"), "no-data");
    refused(() => hc.panchangaOfDay(day, 91, 0, 0, "Lahiri"), "out-of-range");
  });

  test("the Olympiads and the Hebrew anniversaries", () => {
    assert.equal(hc.iocOlympiad(1896), 1);
    assert.equal(hc.iocOlympiad(2020), 32);
    assert.equal(hc.iocOlympiad(2021), 32);
    refused(() => hc.iocOlympiad(1895), "out-of-range");
    // 10 Tevet 5780 was 7 January 2020, and 10 Tevet 5781 25 December 2020.
    const death = hc.gregorianToFixed(2020, 1, 7);
    assert.equal(hc.hebrewYahrzeit(death, 5781), hc.gregorianToFixed(2020, 12, 25));
    assert.equal(hc.hebrewBirthday(death, 5781), hc.gregorianToFixed(2020, 12, 25));
    refused(() => hc.hebrewYahrzeit(death, 10_000), "out-of-range");
  });

  test("the Chinese reckoned age and the marriage auguries", () => {
    // Wikipedia, "East Asian age reckoning": a child born in June 2000 is
    // 13 suì from the lunar new year of 2012, 23 January.
    const birth = hc.gregorianToFixed(2000, 6, 15);
    assert.equal(hc.chineseReckonedAge(birth, hc.gregorianToFixed(2012, 1, 23)), 13);
    assert.equal(hc.chineseReckonedAge(birth, hc.gregorianToFixed(2012, 1, 22)), 12);
    assert.equal(hc.chineseReckonedAge(birth, birth), 1);
    refused(() => hc.chineseReckonedAge(birth, birth - 1), "no-data");
    // The South China Morning Post: 2024's Dragon year is a widow year, and
    // the year from 26 January 2009 holds two 立春.
    const widow = hc.chineseMarriageAugury(4_661);
    assert.deepEqual([widow.augury, widow.lichunAtStart, widow.lichunAtEnd], ["widow", false, false]);
    const double = hc.chineseMarriageAugury(4_646);
    assert.deepEqual([double.augury, double.lichunAtStart, double.lichunAtEnd], ["double-bright", true, true]);
    refused(() => hc.chineseMarriageAugury(2n ** 63n - 1n), "out-of-range");
  });
});

describe("the holiday tables and the liturgical year", () => {
  test("every table is described, in holidayCodes order", () => {
    const tables = hc.holidayTables("en");
    assert.deepEqual(tables.map((table) => table.code), hc.holidayCodes());
    for (const row of rawRows(hc, (buffer, capacity) => {
      const pointer = hc.alloc(2);
      new Uint8Array(hc.memory.buffer, pointer, 2).set([0x65, 0x6e]);
      try {
        return hc.exports.hc_holiday_tables(pointer, 2, buffer, capacity);
      } finally {
        hc.free(pointer, 2);
      }
    })) {
      assert.equal(row.length, COLUMNS.holidayTables.length);
    }
    const byCode = new Map(tables.map((table) => [table.code, table]));
    const japan = byCode.get("JP");
    assert.equal(japan?.kind, "country");
    assert.equal(japan?.name, "Japan");
    assert.equal(japan?.englishName, "Japan");
    assert.equal(japan?.localeUsed, "en");
    assert.ok(japan?.source);
    assert.equal(byCode.get("XJPX")?.kind, "exchange");
    assert.equal(byCode.get("XJPX")?.country, "JP");
    assert.equal(byCode.get("XNYS")?.country, null);
    assert.equal(byCode.get("christian-western")?.kind, "tradition");
    assert.equal(byCode.get("un-days")?.kind, "observance");
    const named = new Set();
    for (const table of tables) {
      assert.ok(table.englishName.length > 0, table.code);
      const key = `${table.kind}\t${table.englishName}`;
      assert.ok(!named.has(key), `two ${table.kind} tables are called ${table.englishName}`);
      named.add(key);
    }
    const japanese = hc.holidayTables("ja-JP");
    assert.deepEqual(japanese.map((table) => table.englishName), tables.map((table) => table.englishName));
    const inJapanese = new Map(japanese.map((table) => [table.code, table]));
    assert.equal(inJapanese.get("JP")?.name, "日本");
    assert.equal(inJapanese.get("JP")?.localeUsed, "ja");
    assert.equal(inJapanese.get("XJPX")?.name, "Tokyo Stock Exchange (JPX)");
    assert.equal(inJapanese.get("XJPX")?.localeUsed, "en");
    assert.equal(inJapanese.get("XJPX")?.country, "JP");
    // CLDR 48's short names: Hong Kong in English and in Japanese, none
    // for Japan, none for an exchange.
    assert.equal(byCode.get("HK")?.name, "Hong Kong SAR China");
    assert.equal(byCode.get("HK")?.shortName, "Hong Kong");
    assert.equal(byCode.get("GB")?.shortName, "UK");
    assert.equal(japan?.shortName, null);
    assert.equal(inJapanese.get("HK")?.shortName, "香港");
    assert.equal(inJapanese.get("MO")?.shortName, "マカオ");
    assert.equal(inJapanese.get("XHKG")?.shortName, null);
  });

  test("the liturgical year 2026 is Year A and Year II", () => {
    assert.deepEqual(hc.lectionary(hc.gregorianToFixed(2025, 11, 30)), {
      liturgicalYear: 2026, sundayCycle: "A", weekdayCycle: "II", proper: null,
      sundayInOrdinaryTime: null, weekOfOrdinaryTime: null, weekOfOrdinaryTimeEpiphanyOnSunday: null,
    });
    assert.deepEqual(hc.lectionary(hc.gregorianToFixed(2025, 11, 29)), {
      liturgicalYear: 2025, sundayCycle: "C", weekdayCycle: "I", proper: null,
      sundayInOrdinaryTime: null, weekOfOrdinaryTime: 34, weekOfOrdinaryTimeEpiphanyOnSunday: 34,
    });
    assert.equal(hc.lectionary(hc.gregorianToFixed(2026, 6, 7)).sundayInOrdinaryTime, 10);
    const monday = hc.lectionary(hc.gregorianToFixed(2023, 1, 9));
    assert.equal(monday.weekOfOrdinaryTime, 1);
    assert.equal(monday.weekOfOrdinaryTimeEpiphanyOnSunday, null);
    assert.equal(hc.lectionary(hc.gregorianToFixed(2026, 11, 22)).proper, 29);
    assert.equal(hc.lectionary(hc.gregorianToFixed(2026, 6, 7)).proper, 5);
    refused(() => hc.lectionary(hc.gregorianToFixed(1500, 1, 1)), "out-of-range");
  });

  test("the astronomical Easter of 2001 is 15 April, a week after its Sunday full moon", () => {
    assert.equal(hc.astronomicalEaster(2001), hc.gregorianToFixed(2001, 4, 15));
    refused(() => hc.astronomicalEaster(1582), "out-of-range");
    refused(() => hc.astronomicalEaster(2151), "out-of-range");
  });

  test("the Aleppo table's paschal full moons are 8 April 2001 and 21 March 2019", () => {
    assert.equal(hc.astronomicalPaschalFullMoon(2001), hc.gregorianToFixed(2001, 4, 8));
    assert.equal(hc.astronomicalEaster(2001) - hc.astronomicalPaschalFullMoon(2001), 7);
    assert.equal(hc.astronomicalPaschalFullMoon(2019n), hc.gregorianToFixed(2019, 3, 21));
    refused(() => hc.astronomicalPaschalFullMoon(1582), "out-of-range");
    refused(() => hc.astronomicalPaschalFullMoon(2151), "out-of-range");
  });
});

describe("the Earth's rotation and the Sun's hours", () => {
  const degrees = (radians) => (radians * 180) / Math.PI;

  test("the rotation angle and the sidereal times are ERFA's test values", () => {
    // eraEra00(2400000.5, 54388.0): JD 2 454 388.5 UT1.
    assert.ok(Math.abs(hc.earthRotationAngle(1_192_406_400) - degrees(0.4022837240028158102)) < 1e-8);
    // eraGmst06 and eraGmst82 at MJD 53 736, 2006-01-01 UT1. The IAU 2006
    // polynomial is in TT, here UT1 + ΔT, which moves it by 95 µas.
    const newYear2006 = 1_136_073_600;
    assert.ok(Math.abs(hc.gmstIau2006(newYear2006) - degrees(1.754174971870091203)) < 1e-7);
    assert.ok(Math.abs(hc.gmstIau1982(newYear2006) - degrees(1.754174981860675096)) < 1e-8);
    // At the Besselian epoch of the USNO's formula only the cosines count.
    assert.ok(Math.abs(hc.ut2MinusUt1(946_684_800 + 0.03 * 86_400) + 0.005) < 1e-6);
    refused(() => hc.earthRotationAngle(Number.NaN), "out-of-range");
    refused(() => hc.gmstIau1982(1e15), "out-of-range");
  });

  test("the clocks read what the ephemerides print", () => {
    // NAOJ: sunrise in Tokyo on 2024-01-01 at 06:50 JST, 21:50 UTC the day
    // before; the temporal hour of sunrise is 6.
    const tokyo = hc.solarTime("temporal", Date.UTC(2023, 11, 31, 21, 50) / 1000, 35.6581, 139.7414);
    assert.equal(tokyo.day, hc.gregorianToFixed(2024, 1, 1));
    assert.ok(Math.abs(tokyo.hours - 6) < 0.03, `${tokyo.hours}`);
    assert.equal(tokyo.missing, null);
    // Meeus, example 28.a: on 1992 October 13 the equation of time is
    // +13 min 42.6 s, so the sundial at Greenwich reads 00:13:42.6.
    const sundial = hc.solarTime("local-apparent", Date.UTC(1992, 9, 13) / 1000, 51.4769, 0);
    assert.equal(sundial.day, hc.gregorianToFixed(1992, 10, 13));
    assert.ok(Math.abs(sundial.hours * 3600 - 822.6) < 1, `${sundial.hours}`);
    const mean = hc.solarTime("LOCAL-MEAN", Date.UTC(2024, 0, 1, 12) / 1000, 0, 15);
    assert.ok(Math.abs(mean.hours - 13) < 1e-6, `${mean.hours}`);
    const italian = hc.solarTime("italian", Date.UTC(2024, 2, 21, 12) / 1000, 45.4064, 11.8768);
    assert.ok(italian.hours !== null && italian.hours > 14.5 && italian.hours < 19.8, `${italian.hours}`);
    refused(() => hc.solarTime(/** @type {any} */ ("babylonian"), 0, 0, 0), "unknown");
    refused(() => hc.solarTime("temporal", 0, 91, 0), "out-of-range");
  });

  test("a missing solar event is an answer", () => {
    // Tromsø in the polar night and under the midnight sun.
    const [latitude, longitude] = [69.6496, 18.956];
    const midwinter = hc.gregorianToFixed(2024, 12, 21);
    const night = hc.solarTime("temporal", hc.unixFromFixed(midwinter) + 43_200, latitude, longitude);
    assert.equal(night.day, null);
    assert.equal(night.hours, null);
    assert.ok(night.missing && ["sunrise", "sunset"].includes(night.missing.event), JSON.stringify(night));
    assert.deepEqual(hc.solarEvent("asr-hanafi", midwinter, latitude, longitude), {
      instant: null, missing: { event: "no-noon-shadow", day: midwinter, depressionArcminutes: null, depressionArcseconds: null },
    });
    const midsummer = hc.gregorianToFixed(2024, 6, 21);
    assert.deepEqual(hc.solarEvent("jewish-dusk-vilna-gaon", midsummer, latitude, longitude), {
      instant: null, missing: { event: "depression", day: midsummer, depressionArcminutes: 280, depressionArcseconds: 16_800 },
    });
    // At Padua the Shafiʿi ʿaṣr comes before the Hanafi, and both before dusk.
    const day = hc.gregorianToFixed(2024, 4, 10);
    const padua = [45.4064, 11.8768];
    const shafii = hc.solarEvent("asr-shafii", day, ...padua).instant;
    const hanafi = hc.solarEvent("asr-hanafi", day, ...padua).instant;
    const dusk = hc.solarEvent("jewish-dusk-vilna-gaon", day, ...padua).instant;
    const ends = hc.solarEvent("jewish-sabbath-ends-cohn", day, ...padua).instant;
    const zero = hc.solarEvent("italian-zero-hour", day, ...padua).instant;
    assert.ok(shafii !== null && hanafi !== null && dusk !== null && ends !== null && zero !== null);
    assert.ok(shafii < hanafi && hanafi < dusk && dusk < ends, `${[shafii, hanafi, dusk, ends]}`);
    refused(() => hc.solarEvent(/** @type {any} */ ("maghrib"), day, ...padua), "unknown");
  });

  test("the horizons are named and each rises and sets on its own", () => {
    const horizons = hc.horizons();
    assert.deepEqual(horizons.map((horizon) => horizon.id), ["geometric-dip", "usno", "calendrical-calculations"]);
    assert.ok(horizons.every((horizon) => horizon.description.length > 0 && horizon.source.length > 0));
    assert.deepEqual(horizons.map((horizon) => horizon.shortName), ["geometric dip", "USNO", "Calendrical Calculations"]);
    assert.ok(horizons.every((horizon) => horizon.name === horizon.englishName && horizon.localeUsed === "en"));
    // The Hong Kong Observatory's name for the USNO (hko-astronomy-portal);
    // no locale names the other two, and Japanese none of the three.
    assert.deepEqual(hc.horizons("zh-HK").map((horizon) => horizon.localeUsed), ["en", "zh-Hant", "en"]);
    assert.equal(hc.horizons("zh-Hant")[1].name, "美國海軍天文氣象台");
    assert.deepEqual(hc.horizons("ja").map((horizon) => horizon.localeUsed), ["en", "en", "en"]);
    // The USNO for Jerusalem on 2024-01-01: sunrise 06:39 and sunset 16:46
    // at UT+2, 04:39 and 14:46 UTC, under its sea-level horizon.
    const day = hc.gregorianToFixed(2024, 1, 1);
    const jerusalem = [31.78, 35.24, 740];
    const rise = hc.sunrise("usno", day, ...jerusalem);
    assert.ok(rise.instant !== null && Math.abs(rise.instant - Date.UTC(2024, 0, 1, 4, 39) / 1000) <= 31, JSON.stringify(rise));
    assert.equal(rise.missing, null);
    assert.ok(Math.abs(rise.altitudeDegrees + 50 / 60) < 1e-9);
    const set = hc.sunset("USNO", day, ...jerusalem);
    assert.ok(set.instant !== null && Math.abs(set.instant - Date.UTC(2024, 0, 1, 14, 46) / 1000) <= 31, JSON.stringify(set));
    // The book lowers its horizon for the height, so its Sun rises sooner.
    const book = hc.sunrise("calendrical-calculations", day, ...jerusalem);
    assert.ok(book.instant !== null && book.instant < rise.instant);
    const midwinter = hc.gregorianToFixed(2024, 12, 21);
    const polar = hc.sunrise("geometric-dip", midwinter, 69.6496, 18.956);
    assert.equal(polar.instant, null);
    assert.deepEqual(polar.missing, { event: "sunrise", day: midwinter, depressionArcminutes: null, depressionArcseconds: null });
    assert.ok(Math.abs(polar.altitudeDegrees + 50 / 60) < 1e-12, `${polar.altitudeDegrees}`);
    refused(() => hc.sunrise(/** @type {any} */ ("naoj"), day, ...jerusalem), "unknown");
  });
});

describe("the Hindu date and the crescent", () => {
  test("Chaitra śukla 1 of Śaka 1947 on both skies", () => {
    // 30 March 2025, the new year's day of Śaka 1947, Vikrama 2082, by the
    // true sky at the Central Station and by the Siddhānta's at Ujjain.
    const day = hc.gregorianToFixed(2025, 3, 30);
    for (const [sky, latitude, longitude] of [["Lahiri", 23.183_333, 82.5], ["surya-siddhanta", 23.15, 75.768_333]]) {
      const date = hc.hinduLunarDate(sky, day, latitude, longitude);
      assert.deepEqual(
        [date.sakaYear, date.vikramaYear, date.month, date.leapMonth, date.tithi, date.leapDay],
        [1947, 2082, 1, false, 1, false],
        sky,
      );
      assert.deepEqual(
        [date.monthName, date.leapMonthWord, date.sakaEra, date.vikramaEra, date.localeUsed],
        ["Chaitra", null, "Saka", "Vikrama Samvat", "en"],
        sky,
      );
    }
    // 27 September 2026 at Tokyo: Śaka 1948, Bhādrapada, the sixteenth
    // tithi, on every ayanamsa and on the Siddhānta's sky.
    const autumn = hc.gregorianToFixed(2026, 9, 27);
    for (const sky of ["Lahiri", "Raman", "Krishnamurti", "Fagan-Bradley", "surya-siddhanta"]) {
      const date = hc.hinduLunarDate(sky, autumn, 35.654_444, 139.744_722, 0, "hi");
      assert.deepEqual(
        [date.sakaYear, date.month, date.tithi, date.monthName, date.sakaEra, date.vikramaEra, date.localeUsed],
        [1948, 6, 16, "भाद्रपद", "शक", "Vikrama Samvat", "hi"],
        sky,
      );
    }
    // describeDay names the Śaka era of the same calendars by the same
    // rule, in its era cell and in the formatted date.
    for (const [id, sky] of [["hindu-lunar", "Lahiri"], ["hindu-lunar-surya-siddhanta", "surya-siddhanta"]]) {
      for (const locale of ["hi", "en", "sa"]) {
        const row = hc.describeDay(autumn, locale).find((line) => line.id === id);
        const date = hc.hinduLunarDate(sky, autumn, 35.654_444, 139.744_722, 0, locale);
        assert.equal(row.eraLabel, date.sakaEra, `${id} ${locale}`);
      }
    }
    assert.equal(hc.describeDay(autumn, "hi").find((line) => line.id === "hindu-lunar").formatted, "16 भाद्रपद 1948 शक");
    // The adhika Śrāvaṇa of Śaka 1945, 18 July to 16 August 2023, at the
    // Central Station.
    const adhika = hc.hinduLunarDate("Lahiri", hc.gregorianToFixed(2023, 8, 1), 23.183_333, 82.5, 0, "en");
    assert.deepEqual([adhika.month, adhika.leapMonth, adhika.monthName, adhika.leapMonthWord], [5, true, "Adhika Sravana", "Adhika"]);
    assert.equal(hc.hinduLunarDate("Lahiri", autumn, 35.654_444, 139.744_722, 0, "native").localeUsed, "sa");
    // At the Siddhānta's sunrise its Sun is in Mīna and its Moon 7.58°
    // ahead, in the first tithi.
    const sunrise = hc.suryaSiddhantaSunrise(day, 23.15, 75.768_333);
    assert.ok(Math.abs(sunrise - Date.UTC(2025, 2, 30, 1, 1) / 1000) < 60, `${sunrise}`);
    const sky = hc.suryaSiddhantaAt(sunrise);
    assert.equal(sky.tithi, 1);
    assert.deepEqual([sky.sign, sky.signId, sky.signName], [12, "mina", "Mīna"]);
    assert.ok(Math.abs(sky.elongation - 7.58) < 0.01, JSON.stringify(sky));
    refused(() => hc.hinduLunarDate("", day, 23.15, 75.768_333), "unknown");
    refused(() => hc.hinduLunarDate("lahiri", hc.gregorianToFixed(2024, 12, 21), 80, 20), "out-of-range");
    refused(() => hc.suryaSiddhantaSunrise(day, 80, 20), "out-of-range");
  });

  test("a crescent is judged on the evening before the day", () => {
    const mecca = [21.423_333, 39.823_333, 298];
    const day = hc.gregorianToFixed(2024, 3, 12);
    for (const criterion of /** @type {const} */ ([
      "shaukat", "yallop", "saudi-rule", "odeh", "istanbul-2016", "khgt", "mabims-2021-topocentric",
      "mabims-2021-geocentric-elongation",
    ])) {
      const verdict = hc.crescentVisible(criterion, day, ...mecca);
      assert.equal(typeof verdict.visible, "boolean", criterion);
      if (verdict.evaluatedAt === null) {
        assert.equal(verdict.visible, false, criterion);
      } else {
        assert.ok(verdict.evaluatedAt < hc.unixFromFixed(day), criterion);
      }
    }
    // Under the midnight sun no evening can be judged.
    const tromso = hc.crescentVisible("shaukat", hc.gregorianToFixed(2024, 6, 21), 69.6496, 18.956);
    assert.deepEqual(tromso, {
      visible: false, evaluatedAt: null, elongation: null, arcOfLight: null, altitude: null, arcOfVision: null, widthArcminutes: null,
    });
    refused(() => hc.crescentVisible(/** @type {any} */ ("danjon"), day, ...mecca), "unknown");
  });

  test("the criteria of #233 answer at the boundary as their sources have them", () => {
    // Djamaluddin's analysis of 1447 AH by Neo-MABIMS (`djamaluddin-kalender-1447`):
    // met in Indonesia on the evening of 25 July 2025, not on 25 June.
    const indonesia = [[5.55, 95.3175], [-6.18, 106.83]];
    for (const criterion of /** @type {const} */ (["mabims-2021-topocentric", "mabims-2021-geocentric-elongation"])) {
      const met = (year, month, date) => indonesia.some(([lat, lon]) =>
        hc.crescentVisible(criterion, hc.gregorianToFixed(year, month, date) + 1, lat, lon).visible);
      assert.equal(met(2025, 7, 25), true, criterion);
      assert.equal(met(2025, 6, 25), false, criterion);
    }
    // KHGT takes the elongation and the altitude geocentric at sunset.
    const mecca = [21.423_333, 39.823_333, 298];
    for (const date of [18, 19, 20]) {
      const verdict = hc.crescentVisible("khgt", hc.gregorianToFixed(2026, 2, date), ...mecca);
      assert.ok(verdict.arcOfLight !== null && verdict.altitude !== null);
      assert.equal(verdict.visible, verdict.arcOfLight >= 8 && verdict.altitude >= 5, JSON.stringify(verdict));
    }
    // Odeh's V, like Yallop's q, is judged at Bruin's best time.
    const day = hc.gregorianToFixed(2026, 2, 19);
    assert.equal(hc.crescentVisible("odeh", day, ...mecca).evaluatedAt, hc.crescentVisible("yallop", day, ...mecca).evaluatedAt);
  });
});

describe("time on other bodies", () => {
  test("Titan's calibration is 209 Aries 13, Julian Circad 144 096", () => {
    // Gangale §3.6: the superior conjunction of 2002 Dec 18 at 10:42 UTC.
    const titan = hc.circadDate("darian-titan", 1_040_208_120);
    assert.deepEqual({ ...titan, fraction: 0, source: "" }, {
      calendar: "darian-titan", year: 209, month: 9, day: 13, monthName: "Aries", weekName: "Solis",
      count: 144_096, fraction: 0, leap: false, source: "",
    });
    assert.ok(titan.fraction >= 0 && titan.fraction < 1);
    for (const calendar of /** @type {const} */ (["gregorian-io", "gregorian-europa", "gregorian-ganymede", "gregorian-callisto", "martiana"])) {
      const date = hc.circadDate(calendar, 1_700_000_000);
      assert.equal(date.calendar, calendar);
      assert.ok(date.month >= 1 && date.day >= 1, calendar);
    }
    // Martiana counts the Darian sol, from 1 Sagittarius 0 on Mars Sol
    // Date −94 129.
    const mars = hc.marsTime(1_700_000_000);
    assert.equal(hc.circadDate("martiana", 1_700_000_000).count, Math.floor(mars.marsSolDate) + 94_129);
    refused(() => hc.circadDate(/** @type {any} */ ("titan"), 0), "unknown");
    refused(() => hc.circadDate("martiana", 1e10), "out-of-range");
  });

  test("Mars24's worked examples decode column by column", () => {
    // Worked example A, 2000-01-06T00:00:00Z at the prime meridian: MSD
    // 44795.99976, MTC 23:59:39, Ls 277.18758°, EOT −5.18774° (−20.75
    // Martian minutes), LTST 23:38:54.
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_mars_time(947_116_800, 0, buffer, capacity));
    assert.equal(raw.length, 1);
    assert.equal(raw[0].length, COLUMNS.marsTime.length);
    const a = hc.marsTime(947_116_800);
    assert.ok(Math.abs(a.marsSolDate - 44_795.999_760_4) < 1e-6, `${a.marsSolDate}`);
    assert.equal(a.mtc, "23:59:39");
    assert.equal(a.lmst, "23:59:39");
    assert.equal(a.ltst, "23:38:54");
    assert.ok(Math.abs(a.solarLongitude - 277.187_58) < 1e-5, `${a.solarLongitude}`);
    assert.ok(Math.abs(a.equationOfTimeMinutes - -5.187_74 * 4) < 1e-3, `${a.equationOfTimeMinutes}`);
    assert.equal(a.marsYear, 24);
    assert.equal(a.darian.year, 207);
    assert.match(a.source, /Mars24/);
    // Worked example B, 2004-01-03T13:46:31Z at Spirit's planned site,
    // 184.702° W: LTST 00:00:00.
    const b = hc.marsTime(1_073_137_591, -184.702);
    assert.equal(b.ltst, "00:00:00");
    assert.ok(Math.abs(b.solarLongitude - 327.324_16) < 1e-4, `${b.solarLongitude}`);
    // Perseverance's landing is 13 Sagittarius 219 in the Darian calendar.
    const landing = hc.marsTime(1_613_681_028, 77.45);
    assert.deepEqual([landing.darian.sol, landing.darian.monthName, landing.darian.year], [13, "Sagittarius", 219]);
    assert.equal(landing.lmst.slice(0, 5), "15:53");
  });

  test("the span is a century either side of J2000", () => {
    assert.equal(hc.marsTime(-2_208_902_400).marsYear < 0, true);
    for (const unix of [-2_240_524_800, 4_133_980_800, Number.NaN, Number.POSITIVE_INFINITY]) {
      const error = refused(() => hc.marsTime(unix), "out-of-range");
      assert.equal(error.export, "hc_mars_time");
    }
    refused(() => hc.marsTime(0, Number.NaN), "out-of-range");
    assert.throws(() => hc.marsTime(/** @type {any} */ ("0")), TypeError);
  });

  test("a mission sol follows the mission's own clock, and an unpublished one is refused", () => {
    const missions = hc.missions();
    assert.equal(missions.length, 10);
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_missions(buffer, capacity));
    assert.ok(raw.every((cells) => cells.length === COLUMNS.missions.length));
    assert.deepEqual(missions.map((entry) => entry.id), [
      "viking-1", "viking-2", "mars-pathfinder", "spirit", "opportunity", "phoenix",
      "curiosity", "insight", "perseverance", "zhurong",
    ]);
    const zhurong = missions[9];
    assert.equal(zhurong.published, false);
    assert.equal(zhurong.landingSol, null);
    assert.equal(zhurong.clock, null);
    assert.equal(zhurong.clockEastLongitude, null);
    assert.equal(missions[0].clock, "local-true-solar-time-at-landing");
    assert.equal(missions[6].landingSol, 0);
    assert.equal(missions[2].landingSol, 1);
    assert.equal(hc.missionSol("curiosity", missions[6].landingUnix), 0);
    assert.equal(hc.missionSol("spirit", missions[3].landingUnix), 1);
    // Curiosity's sol 1000 fell within 2015-05-30 UTC.
    const day = 1_432_944_000;
    const first = hc.missionSol("curiosity", day);
    const last = hc.missionSol("curiosity", day + 86_399);
    assert.ok(first <= 1_000 && 1_000 <= last, `${first}..${last}`);
    refused(() => hc.missionSol("zhurong", 1_700_000_000), "no-data");
    refused(() => hc.missionSol("beagle-2", 1_700_000_000), "unknown");
    for (const name of ["Viking 1", "Viking 2", "Mars Pathfinder"]) {
      refused(() => hc.missionSol(name, 1_700_000_000), "unknown");
    }
    refused(() => hc.missionSol("curiosity", missions[6].landingUnix - 86_400), "out-of-range");
  });

  test("every body lists, and a body's clock reads", () => {
    const bodies = hc.bodies();
    assert.equal(bodies.length, 22);
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_bodies(buffer, capacity));
    assert.ok(raw.every((cells) => cells.length === COLUMNS.bodies.length));
    const byId = Object.fromEntries(bodies.map((entry) => [entry.id, entry]));
    assert.equal(byId.sun.solarDaySeconds, null);
    assert.equal(byId.earth.solarDaySeconds, 86_400);
    assert.equal(byId.mars.solarDayOrigin, "measured");
    assert.equal(byId.mars.zeroPoint, "standard");
    // Mercury's 3:2 resonance: two Mercurian years to its solar day.
    assert.ok(Math.abs(byId.mercury.yearInLocalDays - 0.5) < 1e-3, `${byId.mercury.yearInLocalDays}`);
    assert.ok(byId.venus.siderealRotationHours < 0);
    assert.equal(byId.titan.primary, "saturn");
    assert.match(byId.moon.status ?? "", /Coordinated Lunar Time/);
    assert.equal(byId.titan.status, null);
    const titan = hc.bodyTime("titan", 947_116_800);
    assert.ok(Math.abs(titan.localHourSeconds / 3_600 - 15.97) < 0.01, `${titan.localHourSeconds}`);
    assert.equal(titan.zeroPoint, "convention");
    const mars = hc.bodyTime("mars", 947_116_800);
    assert.equal(mars.day, 44_795);
    assert.equal(mars.time, hc.marsTime(947_116_800).mtc);
    refused(() => hc.bodyTime("sun", 947_116_800), "no-data");
    refused(() => hc.bodyTime("arrakis", 947_116_800), "unknown");
  });
});

describe("relativity", () => {
  test("six tenths of c is five quarters, and a GPS orbit speed loses 7.2 µs a day", () => {
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_proper_time(0.6 * 299_792_458, 10, buffer, capacity));
    assert.equal(raw[0].length, COLUMNS.properTime.length);
    const fast = hc.properTime(0.6 * 299_792_458, 10);
    assert.ok(Math.abs(fast.lorentzFactor - 1.25) < 1e-12);
    assert.ok(Math.abs(fast.properSeconds - 8) < 1e-9);
    assert.deepEqual(fast.constants, ["SPEED_OF_LIGHT"]);
    const gps = hc.properTime(Math.sqrt(3.986_004_418e14 / 26_561_750), 86_400);
    assert.ok(Math.abs(gps.microsecondsPerDay - -7.21) < 0.01, `${gps.microsecondsPerDay}`);
    refused(() => hc.properTime(299_792_458, 1), "out-of-range");
    refused(() => hc.properTime(-3e8, 1), "out-of-range");
  });

  test("a GPS clock gains 45.7 µs a day over one on the ground, before its motion", () => {
    const ground = hc.gravitationalDilation("earth", 6_378_137);
    const orbit = hc.gravitationalDilation("earth", 26_561_750);
    const raw = rawRows(hc, (buffer, capacity) => {
      const name = new TextEncoder().encode("earth");
      const pointer = hc.alloc(name.length);
      new Uint8Array(hc.memory.buffer, pointer, name.length).set(name);
      try {
        return hc.exports.hc_gravitational_dilation(pointer, name.length, 6_378_137, buffer, capacity);
      } finally {
        hc.free(pointer, name.length);
      }
    });
    assert.equal(raw[0].length, COLUMNS.gravitationalDilation.length);
    assert.equal(ground.gmConstant, "GM_EARTH");
    assert.deepEqual(ground.constants, ["GM_EARTH", "SPEED_OF_LIGHT_SQUARED"]);
    const gain = orbit.microsecondsPerDay - ground.microsecondsPerDay;
    assert.ok(Math.abs(gain - 45.65) < 0.01, `${gain}`);
    const sun = hc.gravitationalDilation("sun", 6.957e8);
    assert.ok(Math.abs(sun.schwarzschildRadius - 2_953.25) < 0.01, `${sun.schwarzschildRadius}`);
    refused(() => hc.gravitationalDilation("sun", 2_000), "out-of-range");
    refused(() => hc.gravitationalDilation("vulcan", 1e7), "unknown");
    for (const name of ["Sagittarius A*", "Mars system", "Jupiter system"]) {
      refused(() => hc.gravitationalDilation(name, 1e12), "unknown");
    }
    const bodies = hc.gravitatingBodies();
    assert.deepEqual(bodies.map((entry) => entry.id), ["sun", "earth", "moon", "mars", "jupiter", "sagittarius-a-star"]);
    assert.ok(bodies.every((entry) => entry.gmConstant.startsWith("GM_")));
  });
});

describe("TT(BIPM) from a caller's series", () => {
  // TTBIPM.2025: 27.6740 µs on MJD 58 479 and 27.6745 µs on MJD 58 489;
  // 0 h UTC on MJD 58 479, 2018-12-27, is TAI second 1 545 868 837.
  const series = "58479\t27.6740\n58489\t27.6745\n";
  const tai = 1_545_868_837;

  test("a sample reads as the table gives it, and five days on, halfway", () => {
    const reading = hc.ttBipm(series, tai, 0, true);
    assert.ok(Math.abs(reading.offsetSeconds - 27.674e-6) < 1e-15, JSON.stringify(reading.offsetSeconds));
    assert.equal(reading.minusTai.seconds, 32n);
    const drift = reading.minusTai.attoseconds - 184_027_674_000_000_000n;
    assert.ok(drift > -1_000n && drift < 1_000n, `${reading.minusTai.attoseconds}`);
    assert.equal(reading.reading.seconds, BigInt(tai + 32));
    const midway = hc.ttBipm(series, tai + 5 * 86_400);
    assert.ok(Math.abs(midway.offsetSeconds - 27.674_25e-6) < 1e-15);
  });

  test("outside the series, or in another shape, it is refused", () => {
    refused(() => hc.ttBipm(series, tai - 1), "no-data");
    refused(() => hc.ttBipm("", tai), "no-data");
    refused(() => hc.ttBipm("58479 27.674", tai), "malformed");
    refused(() => hc.ttBipm("58489\t1\n58479\t1", tai), "malformed");
  });
});

describe("renamed months, the Asian days and the sabbatical year", () => {
  test("21 March 2005 is Başgün of Nowruz in Turkmen", () => {
    const day = hc.gregorianToFixed(2005, 3, 21);
    const named = hc.namingPeriodOn("gregory", day, "tk-TM");
    assert.equal(named.state, "in-force");
    assert.ok(named.period !== null);
    assert.deepEqual(
      [named.period.id, named.period.monthName, named.period.weekdayName, named.period.weekdayMeaning],
      ["turkmen-2002", "Nowruz", "Başgün", "First day"],
    );
    assert.deepEqual(
      [named.period.earliest, named.period.inForceBy, named.period.ended],
      [hc.gregorianToFixed(2002, 8, 8), hc.gregorianToFixed(2003, 1, 1), hc.gregorianToFixed(2008, 7, 1)],
    );
    assert.equal(hc.namingPeriodOn("gregory", hc.gregorianToFixed(2002, 10, 1), "tk").state, "undecided");
    assert.deepEqual(hc.namingPeriodOn("gregory", day, "ru"), { state: "ordinary", period: null });
    assert.deepEqual(hc.namingPeriodOn("gregory", day, NATIVE), { state: "ordinary", period: null });
    refused(() => hc.namingPeriodOn("no-such-calendar", day, "tk"), "unknown");
  });

  test("Sebaste opens Kaisar and 7 October is its day 14", () => {
    assert.deepEqual(hc.asianDay(1_360), { year: 4, month: 1, monthName: "Kaisar", written: "unnumbered", number: 1 });
    assert.deepEqual(hc.asianDay(1_374), { year: 4, month: 1, monthName: "Kaisar", written: "numbered", number: 14 });
    // The day hc_describe_day numbers 1 is the one written unnumbered.
    const described = hc.describeDay(1_360, "en").find((row) => row.id === "asian");
    assert.equal(described?.day, 1);
    refused(() => hc.asianDay(1_359), "out-of-range");
  });

  test("5782 and 5789 are sabbatical years", () => {
    assert.equal(hc.hebrewSabbaticalCycleYear(5782), 7);
    assert.equal(hc.hebrewSabbaticalCycleYear(5789n), 7);
    assert.equal(hc.hebrewSabbaticalCycleYear(5786), 4);
    refused(() => hc.hebrewSabbaticalCycleYear(10_000), "out-of-range");
  });
});

describe("Holy Years and the Common Worship ranks", () => {
  test("the jubilee of 2025 is the bull's, and the table's ends are refused", () => {
    const jubilee = hc.holyYearOn(hc.gregorianToFixed(2025, 6, 1));
    assert.deepEqual(jubilee, {
      title: "Ordinary Jubilee of the Year 2025",
      kind: "ordinary",
      pope: "Francis",
      bull: "Spes non confundit",
      given: hc.gregorianToFixed(2024, 5, 9),
      opens: hc.gregorianToFixed(2024, 12, 24),
      closes: hc.gregorianToFixed(2026, 1, 6),
      dioceses: { opens: hc.gregorianToFixed(2024, 12, 29), closes: hc.gregorianToFixed(2025, 12, 28) },
    });
    assert.equal(hc.holyYearOn(hc.gregorianToFixed(2016, 1, 1))?.dioceses, null);
    assert.equal(hc.holyYearOn(hc.gregorianToFixed(2026, 1, 7)), null);
    refused(() => hc.holyYearOn(hc.gregorianToFixed(1974, 12, 23)), "no-data");
  });

  test("St George's Day of 2025 is a Festival on the Monday, named as hc_holidays_on names it", () => {
    const day = hc.gregorianToFixed(2025, 4, 28);
    const kept = hc.commonWorshipOn(day);
    assert.deepEqual(kept, [{
      title: "George, Martyr, Patron of England",
      rank: "festival",
      rankName: "Festival",
      id: "george-martyr-patron-of-england",
    }]);
    // The two are joined on the identifier, not on the title.
    const listed = hc.holidaysOn(day).filter((row) => row.table === "common-worship").map((row) => row.id);
    assert.deepEqual(listed, kept.map((row) => row.id));
    assert.deepEqual(hc.commonWorshipOn(hc.gregorianToFixed(2025, 4, 23)), []);
    assert.equal(hc.commonWorshipOn(hc.gregorianToFixed(2025, 12, 25))[0]?.rank, "principal-feast");
  });
});

describe("the decans and the Heliocentric Julian Date", () => {
  test("an hour after the NAOJ's 秋分 of 2026 the Sun is in the Moon's face of Libra", () => {
    const decan = hc.decanAt(Date.UTC(2026, 8, 23, 1, 5) / 1000);
    assert.deepEqual(
      [decan.sign, decan.signId, decan.signName, decan.decan, decan.ruler, decan.rulerName],
      [7, "libra", "Libra", 1, "moon", "Moon"],
    );
    assert.ok(decan.degreesIntoDecan > 0 && decan.degreesIntoDecan < 0.1, `${decan.degreesIntoDecan}`);
    assert.equal(hc.decanAt(Date.UTC(2026, 8, 22, 23, 5) / 1000).ruler, "mercury");
    refused(() => hc.decanAt(Date.UTC(3001, 0, 1) / 1000), "out-of-range");
  });

  test("Warren's row of 1992 in IDL's helio_jd: 350.9 s", () => {
    // 29 February 1992, 03:15:56.2; 12h 56m 27.4s, +42° 10′ 17″ (J2000).
    const date = 2_448_681.5 + (3 * 3_600 + 15 * 60 + 56.2) / 86_400;
    const alpha = 15 * (12 + 56 / 60 + 27.4 / 3_600);
    const delta = 42 + 10 / 60 + 17 / 3_600;
    const tt = hc.hjdTt(date, alpha, delta);
    assert.ok(Math.abs(tt.correctionSeconds - 350.9) < 0.1, JSON.stringify(tt));
    assert.ok(Math.abs((tt.hjd - date) * 86_400 - tt.correctionSeconds) < 1e-3);
    // TAI − UTC was 26 s in February 1992.
    const utc = hc.hjdUtc(date, alpha, delta, true);
    assert.equal(utc.ttMinusUtcSeconds, 58.184);
    refused(() => hc.hjdTt(date, 361, delta), "out-of-range");
    refused(() => hc.hjdUtc(2_433_282.5, alpha, delta, true), "no-data");
    assert.equal(hc.hjdUtc(2_433_282.5, alpha, delta).ttMinusUtcSeconds, 32.184);
  });
});

describe("the buffer protocol", () => {
  /**
   * The module with its exports counted.
   *
   * @param {number} initialCapacity
   */
  async function counted(initialCapacity) {
    const instance = await WebAssembly.instantiate(bytes, {});
    /** @type {Record<string, number>} */
    const calls = {};
    const exports = Object.fromEntries(
      Object.entries(instance.instance.exports).map(([name, value]) => [
        name,
        typeof value !== "function"
          ? value
          : (/** @type {unknown[]} */ ...args) => {
            calls[name] = (calls[name] ?? 0) + 1;
            return value(...args);
          },
      ]),
    );
    return { hc: await load({ exports }, { initialCapacity }), calls };
  }

  test("a capacity too small for anything still reads everything", async () => {
    const small = await load(bytes, { initialCapacity: 1 });
    assert.equal(small.version(), hc.version());
    assert.equal(small.formatIsoDate(739_880), "2026-09-21");
    assert.deepEqual(small.describeDay(739_880, "ja-JP"), hc.describeDay(739_880, "ja-JP"));
    assert.deepEqual(small.calendars(739_880, "ja"), hc.calendars(739_880, "ja"));
    assert.deepEqual(small.locales(), hc.locales());
    assert.deepEqual(small.calendarUnits("gregory", "year", 10, 10), []);
    assert.deepEqual(small.holidayCodes(), hc.holidayCodes());
    assert.deepEqual(small.holidaysOn(739_880), hc.holidaysOn(739_880));
    assert.deepEqual(small.termInEffect(739_880, "japan"), hc.termInEffect(739_880, "japan"));
    assert.deepEqual(small.cosmicEvents(), hc.cosmicEvents());
    assert.deepEqual(small.earliestEvidence(), hc.earliestEvidence());
    assert.deepEqual(small.geologicIntervals("age"), hc.geologicIntervals("age"));
    assert.deepEqual(small.holidaysInYear("JP", "", -5000), []);
    assert.deepEqual(small.skyAt(1_789_948_800), hc.skyAt(1_789_948_800));
    assert.deepEqual(small.moonPhasesBetween(0, 0), []);
    assert.deepEqual(small.orbitAt(21_000), hc.orbitAt(21_000));
    assert.deepEqual(small.orbitSeries(0, 21_000, 7_000), hc.orbitSeries(0, 21_000, 7_000));
    assert.deepEqual(small.marsTime(947_116_800, 137.44), hc.marsTime(947_116_800, 137.44));
    assert.deepEqual(small.missions(), hc.missions());
    assert.deepEqual(small.bodies(), hc.bodies());
    assert.deepEqual(small.bodyTime("titan", 947_116_800), hc.bodyTime("titan", 947_116_800));
    assert.deepEqual(small.properTime(7_800, 86_400), hc.properTime(7_800, 86_400));
    assert.deepEqual(small.gravitationalDilation("moon", 1.7374e6), hc.gravitationalDilation("moon", 1.7374e6));
    assert.deepEqual(small.gravitatingBodies(), hc.gravitatingBodies());
  });

  test("an export that measures is measured once, then read", async () => {
    const { hc: tiny, calls } = await counted(1);
    tiny.holidayCodes();
    // The 1-byte buffer is refused, the null buffer measures, the sized
    // buffer receives the text.
    assert.equal(calls.hc_holiday_codes, 3);
    assert.equal(calls.hc_alloc, 2);
    assert.equal(calls.hc_free, 2);
    const { hc: roomy, calls: roomyCalls } = await counted(64 * 1024);
    roomy.holidayCodes();
    // With room to spare the module computes its text once.
    assert.equal(roomyCalls.hc_holiday_codes, 1);
    assert.equal(roomyCalls.hc_alloc, 1);
    assert.equal(roomyCalls.hc_free, 1);
  });

  test("an export that cannot measure is offered double until it fits", async () => {
    const { hc: tiny, calls } = await counted(1);
    const version = tiny.version();
    // 1, 2, 4 bytes refused; 8 hold "0.1.0".
    const attempts = Math.ceil(Math.log2(version.length)) + 1;
    assert.equal(calls.hc_version, attempts);
    assert.equal(calls.hc_alloc, attempts);
    assert.equal(calls.hc_free, attempts);
  });

  test("every block allocated for a call is freed, even when the call is refused", async () => {
    const { hc: counted1, calls } = await counted(64 * 1024);
    counted1.describeDay(739_880, "ja-JP");
    counted1.holidayIsDayOff("JP", "JP-13", 739_880);
    counted1.loadZone("Test/Eastern", TZIF_V2_EASTERN);
    refused(() => counted1.holidaysInYear("ZZ", "", 2026), "unknown");
    refused(() => counted1.termInEffect(739_880, "mars"), "unknown");
    refused(() => counted1.loadZone("Test/Junk", new Uint8Array([1, 2, 3])), "malformed");
    assert.equal(calls.hc_alloc, calls.hc_free);
    assert.ok(calls.hc_alloc >= 8, `${calls.hc_alloc} allocations`);
  });
});

describe("time codes and clock readings", () => {
  test("the standard's CCSDS example decodes and encodes", () => {
    // CCSDS 301.0-B-4's example, 1988-01-18T17:20:43.123456 UTC, in CDS
    // with a 16-bit day and microseconds, as docs/systems/ccsds-time-codes.md works it.
    const cds = hc.ccsdsDecode("41 2A DE 03 B8 CE 73 01 C8".replaceAll(" ", ""), true);
    assert.deepEqual(cds, {
      code: "cds",
      tai: { seconds: 569_524_867n, attoseconds: 123_456_000_000_000_000n },
      utc: { unixSeconds: 569_524_843n, leapSecond: false, attoseconds: 123_456_000_000_000_000n },
    });
    assert.equal(hc.ccsdsEncode(569_524_867, 123_456_000_000_000_000n, "53", true), "5319880118172043123456");
    assert.equal(hc.ccsdsEncode(946_684_832, 0, "1c"), "1c4effa220");
    const leap = hc.ccsdsDecode("40542d05265df4", true);
    assert.equal(leap.utc.leapSecond, true);
    assert.equal(leap.tai.seconds, 1_483_228_836n);
    refused(() => hc.ccsdsDecode("2c4effa220"), "no-data");
    // Annex B3.2's 1950 epoch, 2 922 days before 1958: the same instant at
    // Level 2 is day 13 896, P-field 0x49.
    const level2 = hc.ccsdsEncodeFromEpoch(569_524_867, 123_456_000_000_000_000n, "49", { unixDay: -7_305 }, true);
    assert.equal(level2, "49364803b8ce7301c8");
    assert.deepEqual(hc.ccsdsDecodeFromEpoch(level2, { unixDay: -7_305 }, true), {
      ...hc.ccsdsDecode("412ade03b8ce7301c8", true),
    });
    assert.deepEqual(hc.ccsdsDecodeFromEpoch("2c4effa220", { taiSeconds: -378_691_200 }), hc.ccsdsDecode("1c4effa220"));
    refused(() => hc.ccsdsDecodeFromEpoch("2c4effa220", { unixDay: 2_932_897 }), "out-of-range");
    refused(() => hc.ccsdsDecode("zz"), "malformed");
    assert.equal(COLUMNS.ccsdsDecode.length, 6);
  });

  test("the ASCII codes read and write the standard's example", () => {
    const code = hc.ccsdsAsciiParse("1988-01-18T17:20:43.123456Z", true);
    assert.equal(code.variation, "a");
    assert.equal(code.tai.seconds, 569_524_867n);
    assert.equal(code.precision, "fraction");
    assert.equal(code.digits, 6);
    assert.equal(code.terminator, true);
    assert.equal(hc.ccsdsAsciiParse("1988-018T17:20").digits, null);
    assert.equal(hc.ccsdsAsciiFormat(569_524_867, 123_456_000_000_000_000n, "b", 6), "1988-018T17:20:43.123456Z");
    assert.equal(hc.ccsdsAsciiFormat(569_524_867, 0, "a", "minute", false), "1988-01-18T17:20");
    refused(() => hc.ccsdsAsciiFormat(0, 0, /** @type {any} */ ("c"), "hour"), "unknown");
    refused(() => hc.ccsdsAsciiParse("1988-01-18"), "malformed");
  });

  test("the stations' frames decode to their minutes and encode back", () => {
    // NICT's figure of 17:25 JST on 1 April 2004 (nict-jjy-timecode).
    const nict = "M01000101M000100111M000001001M001000010M000000100M100000000M";
    assert.deepEqual(hc.radioDecode("jjy", nict, 2000), {
      unixSeconds: 1_080_807_900, fixed: 731_672, hour: 17, minute: 25, offsetHours: 9, seconds: 60,
      leap: "none", summer: null, zoneChange: null, dut1Tenths: null, dstNext: null,
    });
    assert.equal(hc.radioEncode("jjy", 1_080_807_900), nict);
    // Table 10 of NIST's Enhanced WWVB Broadcast Format: 17:30 UTC on 4 July 2012.
    const am = "M01100000M000100111M000101000M011000101M010000001M001001011M";
    const decoded = hc.radioDecode("wwvb-am", am, 2000);
    assert.equal(decoded.unixSeconds, 1_341_423_000);
    assert.equal(decoded.summer, "in-effect");
    assert.equal(decoded.dut1Tenths, 4);
    assert.equal(hc.radioEncode("wwvb-am", 1_341_423_000, { summer: "in-effect", dut1Tenths: 4 }), am);
    // The frame docs/systems/radio-time-codes.md works from PTB's layout, naming 14:30 CEST on 27 September 2026.
    const dcf77 = "00000000000000000100100001100001010011100111110010011001000";
    assert.equal(hc.radioDecode("dcf77", dcf77, 2000).zoneChange, false);
    assert.equal(hc.radioEncode("dcf77", 1_790_512_200, { summer: "cest" }), dcf77);
    const pm = hc.radioEncode("wwvb-pm", 1_341_423_000, { summer: "in-effect", dstNext: 27 });
    assert.equal(hc.radioDecode("wwvb-pm", pm, 2000).dstNext, 27);
    refused(() => hc.radioDecode("jjy", nict, 2001), "out-of-range");
    refused(() => hc.radioDecode("jjy", nict.slice(1), 2000), "malformed");
    refused(() => hc.radioDecode("jjy", hc.radioEncode("jjy", 1_080_807_300), 2000), "no-data");
    // NICT's second figure, 17:15 JST on 1 April 2004, read in the year it lacks.
    const callSign = hc.jjyCallSignEncode(1_080_807_300);
    assert.equal(callSign, hc.radioEncode("jjy", 1_080_807_300));
    assert.deepEqual(hc.jjyCallSignDecode(callSign, 2004), {
      unixSeconds: 1_080_807_300, fixed: 731_672, hour: 17, minute: 15,
      stopStart: 0, daytimeOnly: false, stopSpan: 0,
    });
    // A stop within 24 hours, by day only, for two to six days.
    const notice = hc.jjyCallSignEncode(1_080_807_300, { stopStart: 4, daytimeOnly: true, stopSpan: 2 });
    assert.equal(notice.slice(50, 56), "100110");
    assert.deepEqual(hc.jjyCallSignDecode(notice, 2004).stopStart, 4);
    refused(() => hc.jjyCallSignDecode(nict, 2004), "malformed");
    refused(() => hc.jjyCallSignEncode(1_080_807_900), "out-of-range");
    refused(() => hc.radioEncode("dcf77", 1_790_512_200, { summer: "cest", leap: -1 }), "out-of-range");
  });

  test(".NET ticks are Microsoft's", () => {
    // DateTime.MaxValue (ms-datetime-maxvalue).
    assert.equal(hc.dotnetTicksFromUnix(253_402_300_799, 999_999_900_000_000_000n), 3_155_378_975_999_999_999n);
    assert.equal(hc.dotnetTicksFromUnix(0), 621_355_968_000_000_000n);
    assert.deepEqual(hc.unixFromDotnetTicks(0), { unixSeconds: -62_135_596_800, attoseconds: 0n });
    refused(() => hc.unixFromDotnetTicks(-1), "out-of-range");
  });

  test("the six-hour clocks read their sources' examples", () => {
    // "8 am is 2 o'clock" (undp-eue-ethiopian-time); "7:00 pm is called saa moja usiku" (ku-kiswahili-lesson-17).
    assert.deepEqual(hc.sixHourClock("ethiopian-hours", 8 * 3_600), {
      hour: 2, minute: 0, second: 0, half: "day", period: null, periodEnglish: null,
    });
    assert.deepEqual(hc.sixHourClock("swahili-hours", 19 * 3_600), {
      hour: 1, minute: 0, second: 0, half: "night", period: "usiku", periodEnglish: "night",
    });
    assert.equal(hc.civilFromSixHourClock("swahili-hours", 1, 0, 0, false), 7 * 3_600);
    refused(() => hc.civilFromSixHourClock("ethiopian-hours", 13, 0, 0, false), "out-of-range");
    refused(() => hc.sixHourClock(/** @type {any} */ ("amharic"), 0), "unknown");
  });
});

describe("the parts of a day", () => {
  test("Rāhu kālam falls where Drik Panchang puts it", () => {
    // Drik Panchang's New Delhi page of 1 January 2025 (drik-day-panchang-2025).
    const day = hc.gregorianToFixed(2025, 1, 1);
    const periods = hc.kalam("rahu-kalam-sunrise", day, 28.6356, 77.2244);
    assert.deepEqual(periods.map((period) => [period.id, period.part]), [
      ["rahu-kalam", 5], ["yamaganda", 2], ["gulika-kalam", 4],
    ]);
    const istMidnight = hc.unixFromFixed(day) - 19_800;
    const minutes = (/** @type {number} */ (periods[0].start) - istMidnight) / 60;
    assert.ok(Math.abs(minutes - (12 * 60 + 25)) < 1, `${minutes}`);
    const fixed = hc.kalam("rahu-kalam-fixed", day, 28.6356, 77.2244);
    assert.deepEqual([fixed[0].clock, fixed[0].start, fixed[0].end, fixed[0].missing], ["local", 43_200, 48_600, null]);
    const polar = hc.kalam("rahu-kalam-sunrise", hc.gregorianToFixed(2024, 12, 21), 69.6496, 18.956);
    assert.equal(polar[0].missing?.event, "sunrise");
    refused(() => hc.kalam(/** @type {any} */ ("yamardha"), day, 28.6, 77.2), "unknown");
    // Drik Panchang's Hindi day pañcāṅga (drik-day-panchang-hi-2026) labels the three.
    const hindi = hc.kalam("rahu-kalam-fixed", day, 28.6356, 77.2244, 0, "hi");
    assert.deepEqual(hindi.map((period) => [period.name, period.localeUsed]), [
      ["राहुकाल", "hi"], ["यमगण्ड", "hi"], ["गुलिक काल", "hi"],
    ]);
    assert.equal(periods[0].name, "Rahu Kalam");
  });

  test("setsubun 2026 faces south-south-east in the ninth period", () => {
    const cycles = hc.almanacCycles(hc.gregorianToFixed(2026, 2, 3), "japan");
    assert.deepEqual(cycles, {
      eho: "丙", ehoRomaji: "hinoe", azimuth: 165, sixteenPoint: "南南東", direction: "south-south-east",
      period: 9, periodName: "九運", era: "下元", star: "九紫火星", ruler: "右弼", firstYear: 2024, lastYear: 2043,
      withoutSon: false,
    });
    // 7 February 2026 is on the published list of 손 없는 날.
    assert.equal(hc.almanacCycles(hc.gregorianToFixed(2026, 2, 7), "korea").withoutSon, true);
    assert.equal(hc.almanacCycles(hc.gregorianToFixed(1600, 1, 1)).withoutSon, null);
    refused(() => hc.almanacCycles(0, "mars"), "unknown");
  });

  test("21 December 2025 is written as the almanacs print it", () => {
    // 赤口, 一粒万倍日 and 天赦日 (arachne-taian-2025-12); 甲子 and 天恩日
    // (mynavi-2025-12-21).
    const day = hc.gregorianToFixed(2025, 12, 21);
    const japanese = hc.almanacDay(day, "japan", "ja");
    assert.deepEqual(japanese.find((line) => line.kind === "rokuyo"), {
      kind: "rokuyo", id: "6", name: "赤口", localeUsed: "ja", japanese: "赤口", reading: "shakkō",
      auspicious: null, printed: null,
    });
    assert.deepEqual(
      japanese.filter((line) => ["lower-register", "selected-day", "combination"].includes(line.kind)).map((line) => line.id),
      ["tenonnichi", "tenshanichi", "ichiryu-manbai", "kinoene", "pardon-and-grain"],
    );
    assert.equal(japanese[0].name, "甲子");
    // 甲子's 納音 is 海中金 (wikipedia-ja-nacchin).
    assert.deepEqual(japanese[1], {
      kind: "nayin", id: "1", name: "海中金", localeUsed: "ja", japanese: "海中金", reading: "kaichūkin",
      auspicious: null, printed: null,
    });
    const english = hc.almanacDay(day, "japan", "en");
    assert.deepEqual(english.map((line) => line.localeUsed).slice(0, 3), ["en", "en", "en"]);
    assert.equal(english.find((line) => line.kind === "combination")?.localeUsed, "ja");
    assert.equal(english.find((line) => line.id === "tenshanichi")?.printed, true);
    assert.deepEqual(hc.almanacDay(day, "japan", "native"), japanese);
    refused(() => hc.almanacDay(day, "mars"), "unknown");
  });

  test("the Orthodox fasts of 2025 are the worked example's", () => {
    assert.deepEqual(hc.orthodoxFastOn("orthodox-fasts", hc.gregorianToFixed(2025, 3, 3)), {
      fastDay: true, status: "period", period: "great-lent", periodName: "Great Lent & Holy Week", kind: "fast",
      abstinence: "fast",
    });
    // Wednesday 18 February 2026, in Cheesefare week: no fast day, but no meat.
    assert.deepEqual(hc.orthodoxFastOn("orthodox-fasts", hc.gregorianToFixed(2026, 2, 18)), {
      fastDay: false, status: "period", period: "meatfast", periodName: "Meatfast", kind: "meat-excluded",
      abstinence: "meat",
    });
    assert.equal(
      hc.orthodoxFastSeasons("orthodox-fasts", 2026).find((season) => season.id === "meatfast")?.kind,
      "meat-excluded",
    );
    assert.deepEqual(hc.orthodoxFastOn("orthodox-fasts", hc.gregorianToFixed(2025, 10, 1)), {
      fastDay: true, status: "weekly-fast", period: null, periodName: null, kind: null, abstinence: "fast",
    });
    const apostles = (reckoning, year) =>
      hc.orthodoxFastSeasons(reckoning, year).find((season) => season.id === "apostles-fast");
    assert.equal(apostles("orthodox-fasts", 2025)?.last, hc.gregorianToFixed(2025, 7, 11));
    assert.equal(apostles("orthodox-fasts-revised-julian", 2025)?.last, hc.gregorianToFixed(2025, 6, 28));
    assert.deepEqual([apostles("orthodox-fasts-revised-julian", 2024)?.first, apostles("orthodox-fasts-revised-julian", 2024)?.last], [null, null]);
    refused(() => hc.orthodoxFastSeasons("orthodox-fasts", 5000), "out-of-range");
    // The Oriental churches' schemes on the same exports.
    const coptic = hc.orthodoxFastSeasons("coptic-fasts", 2026).find((season) => season.id === "apostles-fast");
    assert.deepEqual([coptic?.first, coptic?.last], [hc.gregorianToFixed(2026, 6, 1), hc.gregorianToFixed(2026, 7, 11)]);
    assert.equal(hc.orthodoxFastOn("armenian-fasts", hc.gregorianToFixed(2026, 2, 16)).period, "great-lent");
    assert.equal(hc.orthodoxFastOn("ethiopian-fasts", hc.gregorianToFixed(2026, 1, 18)).period, "gahad-of-timkat");
    refused(() => hc.orthodoxFastOn("coptic-fasts", hc.gregorianToFixed(1500, 1, 1)), "out-of-range");
  });
});

describe("the hours of prayer and the Edo hours", () => {
  test("Singapore's prayer times are MUIS's", () => {
    // MUIS, Prayer Times for Singapore, Year 2026: 1 January, Subuh 5:44 and Maghrib 7:11 pm, UTC+8.
    const day = hc.gregorianToFixed(2026, 1, 1);
    const times = hc.prayerTimes("singapore", day, 1 + 17 / 60, 103 + 50 / 60);
    assert.deepEqual(times.map((time) => time.time), [
      "fajr", "sunrise", "zuhr", "asr-shafii", "asr-hanafi", "maghrib", "isha", "midnight",
    ]);
    const local = (/** @type {number | null} */ instant) => (/** @type {number} */ (instant) - hc.unixFromFixed(day)) / 60 + 480;
    for (const [index, printed] of [[0, 5 * 60 + 44], [5, 19 * 60 + 11]]) {
      const late = printed - local(times[index].instant);
      assert.ok(late > -0.5 && late < 1.5, `${times[index].time}: ${late}`);
    }
    const methods = hc.prayerMethods();
    assert.ok(methods.length >= 11);
    assert.deepEqual(methods.find((method) => method.id === "umm-al-qura")?.ishaRamadanMinutes, 120);
    const tromso = hc.prayerTimes("mwl", hc.gregorianToFixed(2026, 6, 21), 69.6496, 18.956);
    assert.deepEqual(tromso[0].missing, {
      event: "depression", day: hc.gregorianToFixed(2026, 6, 21), depressionArcminutes: 1_080, depressionArcseconds: 64_800,
    });
    refused(() => hc.prayerTimes("", day, 0, 0), "unknown");
  });

  test("New York's zmanim are Hebcal's", () => {
    // Hebcal, 1 January 2025: the latest Shema by the GRA at 9:40 EST.
    const day = hc.gregorianToFixed(2025, 1, 1);
    const zmanim = hc.zmanim("zmanim-gra", day, 40.71427, -74.00597);
    assert.equal(zmanim.length, 9);
    assert.deepEqual([zmanim[0].id, zmanim[0].englishName, zmanim[0].hours], ["sof-zman-shma", "Latest Shema", 3]);
    const minutes = (/** @type {number} */ (zmanim[0].instant) - hc.unixFromFixed(day)) / 60 - 300;
    assert.ok(Math.abs(minutes - (9 * 60 + 40)) < 1, `${minutes}`);
    assert.deepEqual([zmanim[5].id, zmanim[5].englishName, zmanim[5].hours], ["dawn-16-1-degrees", null, null]);
    refused(() => hc.zmanim(/** @type {any} */ ("rabbeinu-tam"), day, 0, 0), "unknown");
    // Hebcal's sunrise 7:20 and sunset 16:40: a GRA hour of 46⅔ minutes.
    const hour = hc.temporalHour("zmanim-gra", day, 40.71427, -74.00597);
    assert.equal(hour.reckoning, "zmanim-gra");
    assert.ok(Math.abs(/** @type {number} */ (hour.seconds) - 2_800) < 20, `${hour.seconds}`);
    assert.equal(hour.missing, null);
    const london = hc.temporalHour("mga-16-1-degrees", hc.gregorianToFixed(2025, 6, 21), 51.50853, -0.12574);
    assert.deepEqual([london.seconds, london.missing?.event], [null, "depression"]);
    refused(() => hc.temporalHour(/** @type {any} */ ("rabbeinu-tam"), day, 0, 0), "unknown");
  });

  test("Kyoto's equinox dawn begins 明六つ, and the Japanese dusk is named to the arcsecond", () => {
    const kyoto = [35 + 36 / 3_600, 135.7417];
    const day = hc.gregorianToFixed(2020, 3, 20);
    const dawn = hc.solarEvent("japanese-dawn-naoj", day, kyoto[0], kyoto[1]);
    // こよみのページ: 夜明 at 5:28:47 JST.
    const printed = hc.unixFromFixed(day) + 5 * 3_600 + 28 * 60 + 47 - 9 * 3_600;
    assert.ok(Math.abs(/** @type {number} */ (dawn.instant) - printed) < 10);
    const reading = hc.edoTime(/** @type {number} */ (dawn.instant) + 2, kyoto[0], kyoto[1]);
    assert.deepEqual([reading.day, reading.hour, reading.name, reading.romaji, reading.strokes, reading.branch, reading.tenths],
      [day, 0, "明六つ", "ake mutsu", 6, "卯", 0]);
    const back = hc.unixFromEdoTime(day, 0, 0, kyoto[0], kyoto[1]);
    assert.ok(Math.abs(/** @type {number} */ (back.instant) - /** @type {number} */ (dawn.instant)) <= 1);
    const helsinki = hc.solarEvent("japanese-dusk-kansei", hc.gregorianToFixed(2024, 6, 21), 60.1699, 24.9384);
    assert.deepEqual(helsinki, {
      instant: null,
      missing: { event: "depression", day: hc.gregorianToFixed(2024, 6, 21), depressionArcminutes: null, depressionArcseconds: 26_501 },
    });
    refused(() => hc.unixFromEdoTime(day, 12, 0, kyoto[0], kyoto[1]), "out-of-range");
  });
});

describe("the reckonings of #243 to #246", () => {
  const delhi = [28.6356, 77.2244];

  test("the choghadiya of 1 January 2025 open with Labha and Udvega", () => {
    // Drik Panchang's New Delhi page (drik-choghadiya-2025): Labha from 07:14 IST.
    const day = hc.gregorianToFixed(2025, 1, 1);
    const parts = hc.choghadiya(day, delhi[0], delhi[1]);
    assert.equal(parts.length, 16);
    assert.deepEqual(
      [parts[0].half, parts[0].part, parts[0].id, parts[0].name, parts[0].localeUsed, parts[0].quality, parts[0].ruler],
      ["day", 1, "labha", "Labha", "en", "auspicious", "mercury"],
    );
    const minutes = (/** @type {number} */ (parts[0].start) - (hc.unixFromFixed(day) - 19_800)) / 60;
    assert.ok(Math.abs(minutes - (7 * 60 + 14)) < 1, `${minutes}`);
    assert.deepEqual([parts[8].half, parts[8].id], ["night", "udvega"]);
    const polar = hc.choghadiya(hc.gregorianToFixed(2024, 12, 21), 69.6496, 18.956, 0, "native");
    assert.deepEqual([polar[0].start, polar[0].missing?.event], [null, "sunrise"]);
    refused(() => hc.choghadiya(day, 91, 0), "out-of-range");
  });

  test("the first Panchak of 2025 is Chor, and the kind follows the caller's clock", () => {
    // Drik Panchang (drik-panchak): Friday 3 January 2025, 10:47 IST.
    const within = hc.unixFromFixed(hc.gregorianToFixed(2025, 1, 5)) + 6 * 3_600;
    const window = hc.panchak("panchak-five-kinds", within, "lahiri", 19_800, "en");
    assert.deepEqual(
      [window.within, window.weekday, window.kind, window.name, window.localeUsed],
      [true, 5, "chor", "Chor", "en"],
    );
    const printed = hc.unixFromFixed(hc.gregorianToFixed(2025, 1, 3)) + (10 * 60 + 47) * 60 - 19_800;
    assert.ok(Math.abs(window.opens - printed) < 90, `${window.opens - printed}`);
    // 23 April 2025 opens at 00:31 IST: a Wednesday by India's clock, no kind
    // in the five-kind table, and a Tuesday, Agni, by UTC's.
    const april = hc.unixFromFixed(hc.gregorianToFixed(2025, 4, 22)) + 6 * 3_600;
    assert.deepEqual([hc.panchak("panchak-five-kinds", april, "lahiri", 19_800).kind, hc.panchak("panchak-five-kinds", april, "lahiri").kind], [null, "agni"]);
    assert.equal(hc.panchak("panchak-raj-midweek", april, "lahiri", 19_800).kind, "raj");
    refused(() => hc.panchak(/** @type {any} */ ("panchak"), within, "lahiri"), "unknown");
    refused(() => hc.panchak("panchak-five-kinds", within, "lahiri", 86_400), "out-of-range");
    assert.throws(() => hc.panchak("panchak-five-kinds", within, "lahiri", 2 ** 31), TypeError);
  });

  test("the Kumbh of 2025 meets Prayag's condition, and the Godavari Pushkaram of 2015 is its twelve days", () => {
    // wikipedia-kumbh-mela: the Maha Kumbh of 2025, the Sun in Makara, Jupiter in Vṛṣabha.
    const kumbh = hc.kumbh("kumbh-prayag-vrishabha", 2025, "lahiri", "vrishabha");
    assert.deepEqual(
      [kumbh.site, kumbh.siteName, kumbh.river, kumbh.jupiter, kumbh.jupiterName, kumbh.sun, kumbh.sunName,
        kumbh.atNewMoon, kumbh.holds],
      ["prayag", "Prayag", "Ganga and Yamuna", "vrishabha", "Vṛṣabha", "makara", "Makara", false, true],
    );
    const unknown = hc.kumbh("kumbh-prayag-vrishabha", 2025, "lahiri");
    assert.deepEqual([unknown.from, unknown.to, unknown.holds], [kumbh.from, kumbh.to, null]);
    assert.equal(hc.kumbh("kumbh-prayag-vrishabha", 2025, "lahiri", "mesha").holds, false);
    refused(() => hc.kumbh("kumbh-haridwar", 2025, "lahiri", /** @type {any} */ ("aries")), "unknown");
    // wikipedia-godavari-pushkaram: Jupiter into Siṃha at 07:07 IST, 14 July 2015; 14 to 25 July.
    const entry = hc.unixFromFixed(hc.gregorianToFixed(2015, 7, 14)) + (7 * 60 + 7) * 60 - 19_800;
    const [godavari] = hc.pushkaram("simha", entry, delhi[0], delhi[1], 0, "india");
    assert.deepEqual(
      [godavari.id, godavari.name, godavari.region, godavari.sign, godavari.signName, godavari.first, godavari.last,
        godavari.missing],
      ["pushkaram-godavari", "Godavari", null, "simha", "Siṃha", hc.gregorianToFixed(2015, 7, 14),
        hc.gregorianToFixed(2015, 7, 25), null],
    );
    assert.deepEqual(hc.pushkaram("vrishchika", entry, delhi[0], delhi[1], 0, "india").map((river) => river.region),
      ["Maharashtra, Karnataka, Telangana", "Tamil Nadu"]);
  });

  test("Jupiter's opposition, its entries of 2019 and the festivals found from the sky", () => {
    // jpl-horizons: Jupiter's apparent longitude at 0 h UT on 2024-12-07, the day of its opposition.
    const opposition = hc.unixFromFixed(hc.gregorianToFixed(2024, 12, 7));
    const at = hc.jupiterAt(opposition, "lahiri");
    assert.ok(Math.abs(at.longitude - 76.3751533) < 0.0002, String(at.longitude));
    assert.deepEqual([at.sign, at.signName, at.retrograde], ["vrishabha", "Vṛṣabha", true]);
    assert.ok(at.dailyMotion < -0.1 && at.distance > 4.0 && at.distance < 4.2);
    refused(() => hc.jupiterAt(0, /** @type {any} */ ("no-such")), "unknown");
    refused(() => hc.jupiterAt(hc.unixFromFixed(hc.gregorianToFixed(3001, 1, 1)), "lahiri"), "out-of-range");
    // drik-guru-gochar: into Dhanus on 30 March 2019, back to Vṛścika on 22 April, into Dhanus on 5 November.
    const from = hc.unixFromFixed(hc.gregorianToFixed(2019, 1, 1));
    const to = hc.unixFromFixed(hc.gregorianToFixed(2020, 1, 1));
    assert.deepEqual(
      hc.jupiterIngresses(from, to, "lahiri").map((i) => [i.from, i.to, i.direction]),
      [["vrishchika", "dhanus", "forward"], ["dhanus", "vrishchika", "retrograde"], ["vrishchika", "dhanus", "forward"]],
    );
    assert.deepEqual(hc.jupiterIngresses(from, from, "lahiri"), []);
    refused(() => hc.jupiterIngresses(from, from + 101 * 31_557_600, "lahiri"), "out-of-range");
    // drik-guru-asta: Jupiter is lost in the Sun's light from 15 July to 12 August 2026 and rises in Puṣya.
    const [rising] = hc.jupiterRisings(
      hc.unixFromFixed(hc.gregorianToFixed(2026, 1, 1)), hc.unixFromFixed(hc.gregorianToFixed(2027, 1, 1)), "lahiri");
    assert.deepEqual([rising.nakshatraId, rising.nakshatraName, rising.year, rising.yearPosition],
      ["pushya", "Puṣya", "Pausha", 10]);
    assert.ok(rising.setting < rising.rising && rising.rising - rising.setting > 26 * 86_400);
    assert.deepEqual(hc.jupiterRisings(from, from, "lahiri"), []);
    // wikipedia-kumbh-mela: the Maha Kumbh of 2025.
    const kumbh = hc.kumbhBySky("kumbh-prayag-vrishabha", 2025, "lahiri", "en");
    assert.deepEqual([kumbh.site, kumbh.holds, kumbh.jupiterThen], ["prayag", true, "vrishabha"]);
    assert.ok(kumbh.jupiterLongitudeThen !== null && kumbh.jupiterLongitudeThen > 30 && kumbh.jupiterLongitudeThen < 60);
    assert.equal(hc.kumbhBySky("kumbh-prayag-vrishabha", 2026, "lahiri").holds, false);
    // wikipedia-godavari-pushkaram: 14 to 25 July 2015, Jupiter into Siṃha at 07:07 IST by Drik, an hour earlier by the series.
    const [godavari] = hc.pushkaramBySky("simha", 2015, "lahiri", "pushkaram-final-entry", delhi[0], delhi[1], 0, "india");
    assert.deepEqual(
      [godavari.id, godavari.first, godavari.last, godavari.rule],
      ["pushkaram-godavari", hc.gregorianToFixed(2015, 7, 14), hc.gregorianToFixed(2015, 7, 25), "pushkaram-final-entry"],
    );
    assert.ok(Math.abs(godavari.entry - (hc.unixFromFixed(hc.gregorianToFixed(2015, 7, 14)) + (7 * 60 + 7) * 60 - 19_800)) < 7_200);
    assert.deepEqual(hc.pushkaramBySky("simha", 2019, "lahiri", "pushkaram-final-entry", delhi[0], delhi[1]), []);
    refused(() => hc.pushkaramBySky("simha", 2015, "lahiri", /** @type {any} */ ("second"), delhi[0], delhi[1]), "unknown");
    // The year's lines are the sign-by-sign lines of each sign entered, in the order of the entries:
    // wikipedia-pushkaram's Tapti and Brahmaputra festivals of 2019 begin at the final entry into Dhanus,
    // and Jupiter enters Mīna (12 January) and Meṣa (26 May) in 1999.
    const dhanus = hc.pushkaramsInYear(2019, "lahiri", "pushkaram-final-entry", delhi[0], delhi[1], 0, "india");
    assert.deepEqual(dhanus.map((line) => line.id), ["pushkaram-tapti", "pushkaram-brahmaputra"]);
    assert.deepEqual(dhanus, hc.pushkaramBySky("dhanus", 2019, "lahiri", "pushkaram-final-entry", delhi[0], delhi[1], 0, "india"));
    const pair = hc.pushkaramsInYear(1999, "lahiri", "pushkaram-final-entry", delhi[0], delhi[1], 0, "india");
    assert.deepEqual([...new Set(pair.map((line) => line.sign))], ["mina", "mesha"]);
    assert.ok(pair.every((line, at) => at === 0 || line.entry >= pair[at - 1].entry));
    for (const sign of /** @type {const} */ (["mina", "mesha"])) {
      assert.deepEqual(
        pair.filter((line) => line.sign === sign),
        hc.pushkaramBySky(sign, 1999, "lahiri", "pushkaram-final-entry", delhi[0], delhi[1], 0, "india"),
      );
    }
    assert.deepEqual(hc.pushkaramsInYear(1971, "lahiri", "pushkaram-final-entry", delhi[0], delhi[1]), []);
    refused(() => hc.pushkaramsInYear(2015, "lahiri", /** @type {any} */ ("second"), delhi[0], delhi[1]), "unknown");
    refused(() => hc.pushkaramsInYear(3001, "lahiri", "pushkaram-final-entry", delhi[0], delhi[1]), "out-of-range");
    // One ingress is one instant whatever span or export it is asked from: Jupiter's final entry into Dhanus in 2019.
    const entry = dhanus[0].entry;
    for (const [before, after] of [[2, 1], [37.5, 41], [300, 500], [0.04, 20]]) {
      const found = hc.jupiterIngresses(entry - Math.round(before * 86_400), entry + Math.round(after * 86_400), "lahiri");
      const same = found.find((ingress) => ingress.to === "dhanus" && Math.abs(ingress.moment - entry) < 172_800);
      assert.equal(same?.moment, entry);
    }
  });

  test("the folk days, the watches and the northern year names are their sources'", () => {
    // bilkent-cemre: Kasım 105, 20 February 2026; netease-2026-longzhishui: 七龙治水.
    const days = hc.folkDay(hc.gregorianToFixed(2026, 2, 20), "china", "tr");
    assert.deepEqual(days[0], { kind: "first-month-count", id: "dragons", name: "几龙治水", localeUsed: "zh-Hans", count: 7 });
    assert.deepEqual(days.at(-1), { kind: "folk-named-day", id: "cemre-air", name: "birinci cemre", localeUsed: "tr", count: 105 });
    const june = hc.folkDay(hc.gregorianToFixed(2026, 6, 11), "china");
    assert.deepEqual(june.find((day) => day.kind === "plum-rains"), {
      kind: "plum-rains", id: "ru-mei-bing", name: "入梅", localeUsed: "zh-Hans", count: null,
    });
    // wikipedia-zh-dian: 三更两点 is 23:48.
    assert.deepEqual(hc.nightWatch(23 * 3_600 + 48 * 60, "zh-TW"), {
      watch: 3, points: 2, name: "三更", localeUsed: "zh-Hant", hanName: "夜半", branch: "子",
    });
    assert.equal(hc.nightWatch(12 * 3_600), null);
    refused(() => hc.nightWatch(86_400), "out-of-range");
    // drikpanchang-day-2024-2026: Śaka 1946 is Pingala by the bīja rule; the
    // press's Kalayukta without it.
    assert.deepEqual(hc.barhaspatyaYear("surya-siddhanta-bija", 1_946, "en"), {
      position: 51, name: "Pingala", expunged: null, expungedName: null, localeUsed: "en",
    });
    assert.equal(hc.barhaspatyaYear("surya-siddhanta", 1_946, "en").name, "Kalayukta");
    assert.equal(hc.barhaspatyaYear("surya-siddhanta-bija", 1_949).expunged, 55);
    const chaitra = hc.unixFromFixed(hc.gregorianToFixed(2025, 3, 30));
    assert.equal(hc.barhaspatyaYearAt("surya-siddhanta", chaitra, "en").position, 53);
    refused(() => hc.barhaspatyaYear("surya-siddhanta", 6_822), "out-of-range");
    // qq-meiyu-2026: 入梅 on 11 June, 出梅 on 8 July.
    assert.equal(hc.plumRains("ru-mei-bing", 2026, "china"), hc.gregorianToFixed(2026, 6, 11));
    assert.equal(hc.plumRains("chu-mei-wei", 2026, "china"), hc.gregorianToFixed(2026, 7, 8));
    refused(() => hc.plumRains(/** @type {any} */ ("ru-mei"), 2026), "unknown");
  });

  test("the planetary hours, GMAT and the IRIG frames cross the binding", () => {
    // lilly-christian-astrology-1647: Monday 25 March 1647 at London opens with the Moon.
    const day = hc.gregorianToFixed(1647, 3, 25);
    const hours = hc.planetaryHoursOfDay(day, 51.5, -0.1);
    assert.equal(hours.length, 24);
    assert.deepEqual([hours[0].day, hours[0].hour, hours[0].ruler, hours[0].name, hours[0].daytime], [day, 1, "moon", "Moon", true]);
    assert.deepEqual(hours.slice(0, 5).map((hour) => hour.ruler), ["moon", "saturn", "jupiter", "mars", "sun"]);
    const at = hc.planetaryHour(/** @type {number} */ (hours[3].start) + 60, 51.5, -0.1);
    assert.deepEqual([at.hour, at.ruler], [4, "mars"]);
    const polar = hc.planetaryHour(hc.unixFromFixed(hc.gregorianToFixed(2024, 6, 21)) + 43_200, 69.65, 18.96);
    // Tromsø at midsummer: the Sun neither sets nor rises, and the hour is only the missing event.
    assert.deepEqual([polar.day, polar.ruler, polar.start], [null, null, null]);
    assert.ok(polar.missing !== null);
    // nautical-almanac-1924: February 20ᵈ 4ʰ 12ᵐ 25ˢ·7 from noon, 16:12:25.7 civil.
    const eclipse = hc.gregorianToFixed(1924, 2, 20);
    assert.deepEqual(hc.gmatFromGmt(eclipse, 58_345, 700_000_000_000_000_000n),
      { fixed: eclipse, secondsOfDay: 15_145, attoseconds: 700_000_000_000_000_000n });
    assert.deepEqual(hc.gmtFromGmat(eclipse, 15_145), { fixed: eclipse, secondsOfDay: 58_345, attoseconds: 0n });
    refused(() => hc.gmatFromGmt(eclipse, 86_400), "out-of-range");
    // rcc-200-16, Figure 5-2: B124, day 173 of 2003, 21:18:42.
    const frame = "M01000001M000101000M100000100M110001110M100000000M110000000M000000000M000000000M010011011M101010010M";
    assert.deepEqual(hc.irigDecode("B124", frame, 2026), {
      fixed: hc.gregorianToFixed(2003, 6, 22), dayOfYear: 173, hour: 21, minute: 18, second: 42, hundredths: 0,
      year: 3, control: 0, straightBinarySeconds: 76_722,
    });
    assert.equal(hc.irigEncode("B124", hc.gregorianToFixed(2003, 6, 22), 76_722), frame);
    // Figure 5-6, IRIG H at 21:24, carries no year and no straight binary seconds.
    const h = hc.irigDecode("H001", "M00000000M001000100M100000100M110001110M100000000M000000000M", 2003);
    assert.deepEqual([h.fixed, h.hour, h.minute, h.year, h.control, h.straightBinarySeconds],
      [hc.gregorianToFixed(2003, 6, 22), 21, 24, null, 0, null]);
    refused(() => hc.irigDecode("B112", frame, 2026), "unknown");
    refused(() => hc.irigDecode("B124", frame.slice(1), 2026), "malformed");
    refused(() => hc.irigEncode("B124", hc.gregorianToFixed(2003, 6, 22), 76_722, { hundredths: 50 }), "out-of-range");
    // Tables 3-1, 3-2 and 4-1: B's frame is a second, so 21:18:42.5 rounds down to Figure 5-2's.
    const formats = hc.irigFormats();
    assert.deepEqual(formats.map((format) => format.format), ["A", "B", "D", "E", "G", "H"]);
    const b = formats[1];
    assert.deepEqual(b, {
      format: "B", indexCountMicroseconds: 10_000, indexCounts: 100, frameMicroseconds: 1_000_000,
      fields: ["days", "hours", "minutes", "seconds"], controlBits: 18, modulations: [0, 1, 2],
      carriers: [0, 2, 3, 4, 5], expressions: [0, 1, 2, 3, 4, 5, 6, 7],
    });
    const reading = 76_722_500_000;
    const start = reading - (reading % b.frameMicroseconds);
    assert.equal(hc.irigEncode("B124", hc.gregorianToFixed(2003, 6, 22), start / 1_000_000), frame);
    assert.equal(formats[2].frameMicroseconds, 3_600_000_000);
  });
});

describe("places", () => {
  test("Tokyo is 東京都 in Japanese and Tokyo in English", () => {
    const japan = hc.subdivisions("jp", "ja-JP");
    // The 47 prefectures CLDR names and the 21 municipalities the holiday tables list.
    assert.equal(japan.length, 47 + 21);
    assert.equal(japan.filter((row) => row.status === "municipal").length, 21);
    const tokyo = japan.find((row) => row.code === "JP-13");
    assert.deepEqual(tokyo, {
      code: "JP-13", name: "東京都", englishName: "Tokyo", localeUsed: "ja", draft: "provisional", status: "regular",
    });
    assert.deepEqual(hc.placeName("jp-13", "ja"), tokyo);
    assert.equal(hc.placeName("JP-13", "en").name, "Tokyo");
    assert.equal(hc.placeName("JP", "ja").name, "日本");
  });

  test("a municipality a holiday table lists is named, in Japanese, romanised and in English", () => {
    const kawasaki = { code: "JP-14-130", name: "川崎市", englishName: "Kawasaki", localeUsed: "ja", draft: null, status: "municipal" };
    assert.deepEqual(hc.placeName("jp-14-130", "ja"), kawasaki);
    const latin = hc.placeName("JP-14-130", "ja-Latn");
    assert.deepEqual([latin.name, latin.localeUsed], ["Kawasaki-shi", "ja-Latn"]);
    const german = hc.placeName("JP-14-130", "de");
    assert.deepEqual([german.name, german.localeUsed], ["Kawasaki", "en"]);
    // In code order after its prefecture, so the lookup that names column 9's prefectures names its cities.
    const codes = hc.subdivisions("JP", "ja").map((row) => row.code);
    const at = codes.indexOf("JP-14");
    assert.deepEqual(codes.slice(at, at + 5), ["JP-14", "JP-14-100", "JP-14-130", "JP-14-150", "JP-15"]);
    // No listed region lacks a name, under any of the three tags.
    for (const table of hc.holidayTables("ja")) {
      for (const code of [...table.regions, ...table.readSubdivisions]) {
        for (const locale of ["ja", "ja-Latn", "en"]) {
          assert.ok(hc.placeName(code, locale).name, `${table.code} ${code} ${locale}`);
        }
      }
    }
    refused(() => hc.placeName("JP-14-999", "ja"), "unknown");
  });

  test("a name falls back along CLDR's chain, then to English", () => {
    assert.equal(hc.placeName("DE-BY", "de").name, "Bayern");
    assert.equal(hc.placeName("US-CA", "fr").name, "Californie");
    const taipei = hc.placeName("JP-13", "zh-TW");
    assert.deepEqual([taipei.name, taipei.localeUsed], ["Tokyo", "en"]);
    const lisbon = hc.placeName("JP-13", "pt-PT");
    assert.deepEqual([lisbon.name, lisbon.localeUsed], ["Tóquio", "pt"]);
    const paris = hc.placeName("FR-75", "ja");
    assert.deepEqual([paris.name, paris.englishName, paris.localeUsed, paris.draft, paris.status],
      [null, null, null, null, "deprecated"]);
  });

  test("every territory and subdivision has a line, and an unknown code is refused", () => {
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_territories(0, 0, buffer, capacity));
    assert.equal(raw.length, 295);
    assert.ok(raw.every((row) => row.length === COLUMNS.places.length));
    assert.equal(hc.territories("de").find((row) => row.code === "001")?.status, "macroregion");
    assert.equal(hc.subdivisions("", "en").length, 5503 + 21);
    assert.deepEqual(hc.subdivisions("AQ"), []);
    refused(() => hc.subdivisions("JPN"), "unknown");
    refused(() => hc.placeName("jp13"), "unknown");
  });
});

describe("humanize", () => {
  test("an instant reads as CLDR's English and Russian phrase it", () => {
    const now = 1_700_000_000;
    assert.deepEqual(hc.relativeTime(now - 3 * 3_600 - 59, now, "long", false, "en"), {
      phrase: "3 hours ago", unit: "hour", count: -3, localeUsed: "en",
    });
    assert.equal(hc.relativeTime(now + 2 * 86_400, now, "long", false, "en").phrase, "in 2 days");
    assert.equal(hc.relativeTime(now - 86_400, now, "long", true, "en").phrase, "yesterday");
    assert.equal(hc.relativeTime(now - 86_400, now, "long", false, "ru").phrase, "1 день назад");
    assert.equal(hc.relativeTime(now - 86_400, now).phrase, "-1 d");
    refused(() => hc.relativeTime(now, now, /** @type {any} */ ("wide")), "unknown");
    refused(() => hc.relativeTime(-(2n ** 63n), 1n), "out-of-range");
  });

  test("a day is counted by the calendar, and a time joins it by the locale's pattern", () => {
    const today = hc.gregorianToFixed(2026, 9, 29);
    assert.deepEqual(hc.relativeDay(today - 1, today, "long", true, "en"), {
      phrase: "yesterday", unit: "day", count: -1, localeUsed: "en",
    });
    assert.equal(hc.relativeDay(today, today, "long", true, "ja").phrase, "今日");
    assert.deepEqual(hc.relativeDayAt(today - 1, today, 15 * 3_600 + 5 * 60, "long", true, "en"), {
      phrase: "yesterday at 15:05", unit: "day", count: -1, time: "15:05", localeUsed: "en",
    });
    refused(() => hc.relativeDayAt(today, today, 86_400), "out-of-range");
  });

  test("a duration is phrased in its units", () => {
    assert.deepEqual(hc.duration(9_000, "long", 0, "en"), {
      phrase: "2 hours and 30 minutes", negative: false, localeUsed: "en",
    });
    assert.equal(hc.duration(9_000, "compact", 0, "en").phrase, "2h30m");
    assert.equal(hc.duration(86_400 + 9_007, "long", 1, "en").phrase, "1 day");
    assert.equal(hc.duration(-9_000, "long", 0, "en").negative, true);
    refused(() => hc.duration(1, /** @type {any} */ ("wide")), "unknown");
  });

  // The examples of `humanize` 4.16's documentation (number.py, filesize.py,
  // lists.py), read 2026-10-03.
  test("the number functions are Python humanize's", () => {
    assert.deepEqual(hc.apnumber(5), { text: "five", localeUsed: "en" });
    assert.deepEqual(hc.fractional(0.3), { text: "3/10", language: "en" });
    assert.equal(hc.apnumber(10).text, "10");
    assert.equal(hc.apnumber(-1n).text, "-1");
    assert.equal(hc.fractional(1.3).text, "1 3/10");
    assert.equal(hc.fractional(0.3).text, "3/10");
    assert.equal(hc.fractional(1 / 3).text, "1/3");
    assert.equal(hc.fractional(Number.NaN).text, "NaN");
    assert.equal(hc.scientific(0.3).text, "3.00 x 10⁻¹");
    assert.equal(hc.scientific(-1000).text, "-1.00 x 10³");
    assert.equal(hc.scientific(1000, 3).text, "1.000 x 10³");
    assert.equal(hc.metric(1500, "V").text, "1.50 kV");
    assert.equal(hc.metric(2e8, "W").text, "200 MW");
    assert.equal(hc.metric(220e-6, "F").text, "220 μF");
    assert.equal(hc.metric(1e-14, "", 4).text, "10.00 f");
    assert.equal(hc.metric(1e40).text, "1.00 x 10⁴⁰");
    assert.equal(hc.naturalSize(3_000_000).text, "3.0 MB");
    assert.equal(hc.naturalSize(3000, "binary").text, "2.9 KiB");
    assert.equal(hc.naturalSize(3000, "gnu").text, "2.9K");
    assert.equal(hc.naturalSize(300, "gnu").text, "300B");
    assert.equal(hc.naturalList(["one", "two", "three"]).text, "one, two and three");
    assert.equal(hc.naturalList(["one", "two"]).text, "one and two");
    assert.equal(hc.naturalList(["one"]).text, "one");
    assert.equal(hc.naturalList([]).text, "");
    assert.equal(hc.intword(12_400).text, "12.4 thousand");
    assert.equal(hc.intword("1234000", 3).text, "1.234 million");
    assert.equal(hc.intword(8_100_000_000_000_000_000_000_000_000_000_000n).text, "8.1 decillion");
    assert.equal(hc.intword(`1${"0".repeat(100)}`).text, "1.0 googol");
    refused(() => hc.intword("12x"), "malformed");
    refused(() => hc.intword("9".repeat(400)), "out-of-range");
    refused(() => hc.naturalSize(Number.NaN), "out-of-range");
    refused(() => hc.naturalSize(1, /** @type {any} */ ("wide")), "unknown");
  });

  // The catalogues are humanize 4.16.0's `de_DE.po`, `ru_RU.po` and `pt_PT.po`,
  // which the crate's own tests hold to GNU gettext.
  test("the words follow the locale's catalogue, and no result mixes two languages", () => {
    assert.deepEqual(hc.apnumber(5, "de"), { text: "fünf", localeUsed: "de-DE" });
    assert.equal(hc.apnumber(5, "de-AT").localeUsed, "de-DE");
    assert.equal(hc.apnumber(5, "pt-AO").localeUsed, "pt-PT");
    // `pt` has two catalogues and no region is guessed; a language with none is English.
    assert.deepEqual(hc.apnumber(5, "pt"), { text: "five", localeUsed: "en" });
    assert.deepEqual(hc.apnumber(5, "tlh-x-nothing"), { text: "five", localeUsed: "en" });
    assert.equal(hc.intword(2_000_000, 1, "de").text, "2,0 Millionen");
    // The German catalogue translates no word of `naturalsize`, so it is English whole.
    assert.deepEqual(hc.naturalSize(3_000_000, "decimal", 1, "de"), { text: "3.0 MB", localeUsed: "en" });
    // `natural_list` is in no catalogue.
    assert.deepEqual(hc.naturalList(["a", "b", "c"], "de"), { text: "a, b and c", localeUsed: "en" });
    assert.deepEqual(hc.metric(1500, "V", 3, "de"), { text: "1.50 kV", localeUsed: "de-DE" });
  });

  // `precisedelta`'s examples are those of humanize 4.16's documentation: a
  // delta of two days, 3 633 seconds and 123 000 microseconds.
  test("the time and day functions are Python humanize's", () => {
    const seconds = 2 * 86_400 + 3_633;
    assert.equal(hc.preciseDelta(seconds, 123_000).text, "2 days, 1 hour and 33.12 seconds");
    assert.equal(
      hc.preciseDelta(seconds, 123_000, "microseconds").text,
      "2 days, 1 hour, 33 seconds and 123 milliseconds",
    );
    assert.equal(
      hc.preciseDelta(seconds, 123_000, "seconds", ["days"], 4).text,
      "49 hours and 33.1230 seconds",
    );
    assert.equal(hc.naturalDelta(7 * 86_400).text, "7 days");
    assert.equal(hc.naturalTime(3).text, "3 seconds ago");
    assert.equal(hc.naturalTime(-3).text, "3 seconds from now");
    assert.deepEqual(hc.naturalTime(3, 0, true, "seconds", "de"), { text: "vor 3 Sekunden", localeUsed: "de-DE" });
    const today = hc.gregorianToFixed(2026, 9, 29);
    assert.equal(hc.naturalDay(today + 1, today).text, "tomorrow");
    assert.deepEqual(hc.naturalDay(today + 1, today, "", "de"), { text: "morgen", localeUsed: "de-DE" });
    assert.equal(hc.naturalDay(today + 10, today).text, "Oct 09");
    assert.equal(hc.naturalDay(today + 10, today, "%Y-%m-%d").text, "2026-10-09");
    assert.equal(hc.naturalDate(today + 152, today).text, "Feb 28");
    assert.equal(hc.naturalDate(today + 153, today).text, "Mar 01 2027");
    assert.equal(hc.ordinal(103).text, "103rd");
    assert.equal(hc.ordinal(111, "female").text, "111th");
    assert.deepEqual(hc.intcomma(-1_234_567n, "de"), { text: "-1.234.567", localeUsed: "de-DE" });
    assert.equal(hc.intcomma("1234567").text, "1,234,567");
    assert.equal(hc.intcommaFloat(12_345.6789, 2).text, "12,345.68");
    assert.equal(hc.intcommaFloat(1_234_567.25).text, "1,234,567.25");
    refused(() => hc.naturalDelta(1, 0, true, /** @type {any} */ ("hours")), "out-of-range");
    refused(() => hc.naturalDelta(1, 0, true, /** @type {any} */ ("fortnights")), "unknown");
    refused(() => hc.naturalDelta(1, 1_000_000), "out-of-range");
    refused(() => hc.preciseDelta(1, 0, "seconds", ["seconds", "minutes", "hours", "days", "months", "years"]), "out-of-range");
    refused(() => hc.ordinal(1, /** @type {any} */ ("neuter")), "unknown");
    refused(() => hc.intcomma("12x"), "malformed");
    refused(() => hc.intcomma("9".repeat(40)), "out-of-range");
    refused(() => hc.naturalDay(2 ** 53 - 1, today), "out-of-range");
  });

  // `hc-humanize`'s documentation of its `unit_choice` and `approximate`
  // modules: 90 minutes is two hours rounded and an hour and a half by
  // halves; 400 days is just over a year, 350 nearly one.
  test("the thresholds, the rounding and the hedge are arguments", () => {
    const ninety = 90 * 60;
    assert.deepEqual(hc.unitChoice(ninety), {
      unit: "hour", count: 2, half: false, thresholds: "default", rounding: "nearest",
    });
    assert.deepEqual(hc.unitChoice(ninety, "default", "nearest-half"), {
      unit: "hour", count: 1, half: true, thresholds: "default", rounding: "nearest-half",
    });
    assert.equal(hc.unitChoice(-ninety, "exact", "truncate").count, -1);
    const now = 1_700_000_000;
    assert.deepEqual(hc.relativeTimeWith(now - ninety, now, "long", false, "en"), {
      phrase: "1 hour ago", unit: "hour", count: -1, half: false, localeUsed: "en",
    });
    assert.equal(hc.relativeTimeWith(now - ninety, now, "long", false, "en", "default", "nearest").phrase, "2 hours ago");
    const nearly = hc.approximateDuration(350 * 86_400, "long", "en");
    assert.equal(nearly.hedge, "nearly");
    assert.equal(nearly.unit, "year");
    assert.match(nearly.phrase, /^nearly /);
    const over = hc.approximateDuration(400 * 86_400, "long", "en");
    assert.deepEqual([over.hedge, over.unit, over.count], ["just-over", "year", 1]);
    refused(() => hc.unitChoice(1, /** @type {any} */ ("wide")), "unknown");
    refused(() => hc.unitChoice(1, "default", /** @type {any} */ ("up")), "unknown");
    refused(() => hc.approximateDuration(1, "long", "en", "default", /** @type {any} */ ("sloppy")), "unknown");
  });
});

describe("the almanac's directions, 臘日, undertakings and a person's own days", () => {
  test("the gods stand where 古文書ネット's tables put them", () => {
    const gods = hc.almanacDirections(hc.gregorianToFixed(2026, 9, 29), "japan");
    assert.deepEqual(gods[0], {
      id: "taisai", name: "太歳神", reading: "taisaijin", branch: "午", azimuth: 180,
      meaning: "facing it everything goes well, but do not fell trees", year: "丙午", branchNumber: 7,
    });
    assert.equal(gods.find((god) => god.id === "saiha")?.branch, "子");
    const konjin = hc.almanacDirections(hc.gregorianToFixed(2025, 6, 1), "japan").filter((god) => god.id === "konjin");
    assert.deepEqual(konjin.map((god) => god.branch), ["辰", "巳"]);
    assert.equal(konjin[0].reading, null);
  });

  test("臘日 is こよみる's day of 2026, and an unsettled winter is refused", () => {
    assert.equal(hc.rounichi("dragon-nearest-major-cold-earlier", 2026, "japan"), hc.gregorianToFixed(2026, 1, 18));
    assert.equal(hc.rounichi("first-dog-after-major-cold", 2026, "japan"), hc.gregorianToFixed(2026, 1, 24));
    refused(() => hc.rounichi(/** @type {any} */ ("rounichi"), 2026), "unknown");
    refused(() => hc.rounichi("dragon-nearest-major-cold-earlier", -1000), "out-of-range");
  });

  test("the undertakings are the publisher's, and a person's days are their birth year's", () => {
    const kaku = hc.mansionUndertakings("saijigoyomi", hc.gregorianToFixed(2026, 1, 8));
    assert.deepEqual(kaku[0], { mansion: 1, mansionName: "角", grade: "favoured", undertaking: "衣類裁断" });
    assert.deepEqual(kaku.filter((row) => row.grade === "avoided").map((row) => row.undertaking), ["葬式", "納骨"]);
    const wood = hc.almanacPersonDays(hc.gregorianToFixed(2025, 2, 25), hc.gregorianToFixed(1928, 6, 1), "japan");
    assert.deepEqual(wood[0], { kind: "grave-day", id: "gomunichi-wikipedia", name: "五墓日", keeps: "乙丑", applies: true });
    // Born before 1928's 立春, the person is of 丁卯, 1927's year.
    const fire = hc.almanacPersonDays(hc.gregorianToFixed(2025, 2, 25), hc.gregorianToFixed(1928, 2, 1), "japan");
    assert.notEqual(fire[0].keeps, "乙丑");
    const snake = hc.almanacPersonDays(hc.gregorianToFixed(2025, 5, 15), hc.gregorianToFixed(2025, 3, 1), "japan");
    assert.equal(snake.find((row) => row.id === "taikanichi")?.applies, true);
  });
});

describe("the muhūrtas, amṛta siddhi, the nakṣatra, the drekkāṇa and the Siddhānta's pañcāṅga", () => {
  const delhi = [28 + 38 / 60 + 8 / 3600, 77 + 13 / 60 + 28 / 3600, 0];

  test("Wednesday 1 January 2025 has no Abhijit and its eighth muhūrta as Dur Muhurtam", () => {
    const day = hc.gregorianToFixed(2025, 1, 1);
    const muhurtas = hc.muhurtas(day, ...delhi);
    assert.equal(muhurtas.length, 30);
    assert.deepEqual(muhurtas.filter((m) => m.mark !== null).map((m) => [m.half, m.number, m.mark]), [
      ["day", 8, "dur-muhurtam"],
    ]);
    assert.deepEqual([0, 7, 15, 29].map((index) => muhurtas[index].name), ["Rudra", "Vidhi", "Girīśa", "Samudra"]);
    const thursday = hc.muhurtas(day + 1, ...delhi).filter((m) => m.mark !== null);
    assert.deepEqual(thursday.map((m) => [m.number, m.mark]), [[6, "dur-muhurtam"], [8, "abhijit"], [12, "dur-muhurtam"]]);
    assert.equal(hc.muhurtas(day, 78, 15, 0)[0].missing?.event, "sunrise");
  });

  test("amṛta siddhi falls on Tuesday 7 January 2025 in Aśvinī, and the Moon is there", () => {
    const yoga = hc.amritaSiddhi(hc.gregorianToFixed(2025, 1, 7), ...delhi, "lahiri");
    assert.equal(yoga.falls, true);
    assert.deepEqual([yoga.nakshatra, yoga.nakshatraId, yoga.nakshatraName], [1, "ashvini", "Aśvinī"]);
    assert.equal(hc.amritaSiddhi(hc.gregorianToFixed(2025, 1, 8), ...delhi, "lahiri").start, null);
    const stay = hc.nakshatraAt(/** @type {number} */ (yoga.start) + 60, "lahiri");
    assert.deepEqual([stay.nakshatra, stay.nakshatraId, stay.nakshatraName], [1, "ashvini", "Aśvinī"]);
    assert.equal(hc.nakshatraOfDay(hc.gregorianToFixed(2025, 1, 7), ...delhi, "lahiri").ayanamsa, "lahiri");
    refused(() => hc.nakshatraAt(0, /** @type {any} */ ("tropical")), "unknown");
  });

  test("the drekkāṇa is a third of the sidereal sign, and its lord rules the sign it is given to", () => {
    const drekkana = hc.drekkanaAt(1_700_000_000, "lahiri");
    assert.ok(drekkana.drekkana >= 1 && drekkana.drekkana <= 3);
    assert.ok(drekkana.degreesIntoDrekkana >= 0 && drekkana.degreesIntoDrekkana < 10);
    assert.equal(drekkana.ayanamsa, "lahiri");
    // Every sign is named the same way: an identifier and its Sanskrit name.
    assert.equal(drekkana.signName.normalize("NFD").replace(/[^a-zA-Z]/g, "").toLowerCase().slice(0, 2),
      drekkana.signId.slice(0, 2));
    assert.ok(drekkana.lordSignName.length > 0);
    refused(() => hc.drekkanaAt(0, /** @type {any} */ ("")), "unknown");
  });

  test("the Siddhānta's pañcāṅga names its sky on both lines, and the twelve-year name rides with the sixty", () => {
    const limbs = hc.panchangaAt(1_700_000_000, "surya-siddhanta");
    assert.deepEqual(limbs.map((limb) => limb.ayanamsa), ["surya-siddhanta", "surya-siddhanta"]);
    assert.equal(limbs[1].ayanamsaName, "Sūrya Siddhānta");
    assert.equal(hc.panchangaAt(1_700_000_000, "lahiri")[1].ayanamsa, null);
    const at = hc.barhaspatyaYearAt("surya-siddhanta-bija", Date.UTC(2024, 3, 29, 10, 40) / 1000);
    assert.deepEqual([at.position, at.twelveYear, at.twelveYearName, at.meanSign, at.meanSignName],
      [51, 7, "Asvina", "mesha", "Meṣa"]);
  });
});

describe("the era tables and the Olympic Games", () => {
  test("the era tables list what their tables know", () => {
    // Japanese Wikipedia's 元号一覧 (日本): 248 eras, 令和 the 248th, from 1 May 2019 and still in force.
    const japanese = hc.eraTable("japanese");
    assert.equal(japanese.length, 248);
    const reiwa = japanese.find((era) => era.code === "reiwa");
    assert.deepEqual(
      [reiwa?.name, reiwa?.group, reiwa?.start, reiwa?.last, reiwa?.status],
      ["令和", "unified", hc.gregorianToFixed(2019, 5, 1), null, "attested"],
    );
    assert.equal(japanese.find((era) => era.code === "heisei")?.last, hc.gregorianToFixed(2019, 4, 30));
    assert.equal(hc.eraTable("chinese-regnal").length, 37);
    const gwangmu = hc.eraTable("korean-regnal").find((era) => era.code === "gwangmu");
    assert.deepEqual(
      [gwangmu?.name, gwangmu?.start, gwangmu?.otherStart],
      ["光武", hc.gregorianToFixed(1897, 8, 14), hc.gregorianToFixed(1897, 1, 1)],
    );
    refused(() => hc.eraTable(/** @type {any} */ ("babylonian")), "unknown");
  });

  test("the Games are Olympedia's", () => {
    // Olympedia's editions: Paris 2024 opened on 26 July and closed on 11 August; the VI Games of 1916 were not held.
    const summer = hc.olympicGames("summer");
    const paris = summer.find((games) => games.year === 2024);
    assert.deepEqual(
      [paris?.number, paris?.host, paris?.opening, paris?.closing],
      [33, "Paris", hc.gregorianToFixed(2024, 7, 26), hc.gregorianToFixed(2024, 8, 11)],
    );
    assert.deepEqual(summer.find((games) => games.year === 1916), {
      number: 6, year: 1916, host: "Berlin", status: "not-held", opening: null, closing: null,
    });
    assert.equal(hc.olympicGames("winter").find((games) => games.year === 2022)?.number, 24);
    refused(() => hc.olympicGames(/** @type {any} */ ("spring")), "unknown");
  });
});

describe("the tithi and the ayanāṃśas", () => {
  // Drik Panchang's page for Tokyo (35°41′22″N 139°41′30″E) of 13 January 2025: "Chaturdashi upto 08:33 AM", sunrise 06:51.
  const tokyo = [35 + 41 / 60 + 22 / 3600, 139 + 41 / 60 + 30 / 3600];

  test("a tithi ends when Drik Panchang says and a day lists the tithis it holds", () => {
    const noon = Date.UTC(2025, 0, 12, 12) / 1000;
    const tithi = hc.tithiAt(noon, "lahiri");
    assert.deepEqual([tithi.number, tithi.paksha, tithi.pakshaDay, tithi.name, tithi.sky], [14, "shukla", 14, "Caturdaśī", "true"]);
    assert.ok(Math.abs(tithi.ends - Date.UTC(2025, 0, 12, 23, 33) / 1000) <= 120);
    assert.equal(tithi.readAt, noon);
    assert.equal(hc.tithiAt(1_700_000_000, "surya-siddhanta").sky, "surya-siddhanta");
    const day = hc.gregorianToFixed(2025, 1, 13);
    const rows = hc.tithisOfDay(day, tokyo[0], tokyo[1], 0, "true");
    assert.deepEqual(rows.map((row) => [row.number, row.name, row.atSunrise, row.repeated, row.skipped]), [
      [14, "Caturdaśī", true, false, false],
      [15, "Pūrṇimā", false, false, false],
    ]);
    assert.ok(Math.abs(rows[0].readAt - Date.UTC(2025, 0, 12, 21, 51) / 1000) <= 120);
    refused(() => hc.tithiAt(noon, /** @type {any} */ ("mars")), "unknown");
    refused(() => hc.tithiAt(200_000_000_000, "true"), "out-of-range");
  });

  test("the ayanāṃśas are listed and valued at an instant", () => {
    const table = hc.ayanamsas();
    assert.ok(table.length >= 8);
    const crc = table.find((row) => row.id === "lahiri-crc-1955");
    assert.deepEqual([crc?.anchorJulianDate, crc?.anchorDegrees], [2_435_553.5, 23.25]);
    // Drik Panchang prints its Lahiri as 24.213067 on 1 January 2025.
    const drik = hc.ayanamsaAt(Date.UTC(2025, 0, 1) / 1000, "lahiri-drik");
    assert.ok(Math.abs(drik.degrees - 24.213_067) < 3e-4, String(drik.degrees));
    assert.equal(drik.name, "Lahiri (Drik Panchang)");
    // The Calendar Reform Committee's value is read back at its own date.
    const custom = hc.ayanamsaFromAnchor(Date.UTC(1956, 2, 21) / 1000, 2_435_553.5, 23.25);
    assert.ok(Math.abs(custom.degrees - 23.25) < 1e-6);
    assert.deepEqual([custom.id, custom.name], ["custom", "custom"]);
    refused(() => hc.ayanamsaAt(0, /** @type {any} */ ("mars")), "unknown");
    refused(() => hc.ayanamsaFromAnchor(0, Number.NaN, 1), "out-of-range");
  });
});

describe("the Kumbh and Pushkaram tables, the Kumbhs of a year and Jupiter's stations", () => {
  test("the tables list what the sources name", () => {
    // The Mela Adhikari's seven conditions; Wikipedia's Pushkaram table: 14 rivers, the Ganga at Meṣa.
    const yogas = hc.kumbhYogas("en");
    assert.equal(yogas.length, 7);
    assert.deepEqual([yogas[0].id, yogas[0].siteName, yogas[0].river, yogas[0].jupiter, yogas[0].sun], ["kumbh-haridwar", "Haridwar", "Ganga", "kumbha", "mesha"]);
    assert.equal(yogas.find((yoga) => yoga.id === "kumbh-prayag-mesha")?.atNewMoon, true);
    const rivers = hc.pushkaramRivers("en");
    assert.equal(rivers.length, 14);
    assert.deepEqual(rivers.find((river) => river.id === "pushkaram-bhima")?.region, "Maharashtra, Karnataka, Telangana");
    assert.equal(rivers.find((river) => river.id === "pushkaram-ganga")?.sign, "mesha");
    assert.deepEqual(hc.pushkaramRules().map((rule) => rule.id), ["pushkaram-final-entry", "pushkaram-first-entry"]);
  });

  test("a year's Kumbhs and Jupiter's stations are their sources'", () => {
    // The Maha Kumbh of 2025 at Prayag is the one condition the sky meets.
    const kumbhs = hc.kumbhsInYearBySky(2025, "lahiri", "en");
    assert.equal(kumbhs.length, 7);
    assert.deepEqual(kumbhs.filter((kumbh) => kumbh.holds).map((kumbh) => kumbh.id), ["kumbh-prayag-vrishabha"]);
    assert.deepEqual(kumbhs[1], hc.kumbhBySky("kumbh-prayag-vrishabha", 2025, "lahiri", "en"));
    // Drik Panchang: retrograde on 9 October 2024 at 12:33 IST, progressive on 4 February 2025 at 15:09.
    const stations = hc.jupiterStations(Date.UTC(2024, 8, 1) / 1000, Date.UTC(2025, 2, 1) / 1000, "lahiri");
    assert.deepEqual(stations.map((station) => station.kind), ["retrograde", "direct"]);
    assert.ok(Math.abs(stations[0].moment - (Date.UTC(2024, 9, 9, 12, 33) / 1000 - 19_800)) < 7 * 60);
    assert.ok(Math.abs(stations[1].moment - (Date.UTC(2025, 1, 4, 15, 9) / 1000 - 19_800)) < 7 * 60);
    assert.equal(stations[0].sign, "vrishabha");
    refused(() => hc.kumbhsInYearBySky(3001, "lahiri"), "out-of-range");
    refused(() => hc.jupiterStations(0, 1, /** @type {any} */ ("mars")), "unknown");
  });
});

describe("the Tibetan almanac", () => {
  test("Henning's Tsurphu almanac's first day of 2013 is written as he prints it", () => {
    const entries = hc.tibetanAlmanacDay("tibetan-tsurphu-karana", hc.gregorianToFixed(2013, 2, 11));
    assert.deepEqual(entries[0], {
      kind: "weekday", id: 3, name: "Monday", tibetan: "zla ba", reading: "2;11,24", value: entries[0].value,
    });
    assert.deepEqual([entries[1].name, entries[1].tibetan], ["Shatabhishaj", "mon gru"]);
    assert.equal(entries.find((entry) => entry.kind === "year-symbol")?.name, "Water-Snake");
    // The attributes his almanac prints for that day: the Tiger lunar day with li and 1, the Chinese mansion
    // Bi with no number, and the elements Water and Earth.
    const of = (/** @type {string} */ kind) => entries.filter((entry) => entry.kind === kind);
    assert.deepEqual(of("lunar-day-animal").map((e) => [e.id, e.name]), [[3, "Tiger"]]);
    assert.deepEqual(of("lunar-day-trigram").map((e) => [e.id, e.name, e.tibetan, e.reading, e.value]), [[1, "li", "lí", "S", "fire"]]);
    assert.deepEqual(of("lunar-day-number").map((e) => [e.id, e.name, e.tibetan, e.reading]), [[1, "white", "iron", "N"]]);
    assert.deepEqual(of("chinese-mansion").map((e) => [e.id, e.name]), [[19, "Bi"]]);
    assert.deepEqual(of("element-pair").map((e) => [e.id, e.name, e.tibetan]), [[3, "Water", "Earth"]]);
    assert.equal(of("day-number-henning").length, 0);
    const phugpa = hc.tibetanAlmanacDay("tibetan", hc.gregorianToFixed(2013, 2, 11));
    // 11 February 2013 is 9 in Henning's almanac and 8 by Janson's rule: two conventions, two kinds.
    assert.deepEqual(
      [phugpa.find((e) => e.kind === "day-number-henning")?.id, phugpa.find((e) => e.kind === "day-number-janson")?.id],
      [9, 8],
    );
    refused(() => hc.tibetanAlmanacDay(/** @type {any} */ ("tibetan-x"), 0), "unknown");
  });

  test("the planets, the Bhutanese solstice and a skipped festival", () => {
    const mars = hc.tibetanPlanets(hc.gregorianToFixed(2011, 1, 6)).find((planet) => planet.planet === "mars");
    assert.equal(mars?.particularDay, 525);
    assert.deepEqual(hc.bhutaneseWinterSolstice(2001).reading, "2;51,38");
    assert.equal(hc.bhutaneseWinterSolstice(2001).fixed, hc.gregorianToFixed(2001, 1, 1));
    refused(() => hc.tibetanFestivalDay("henning-almanac", "tibetan-lochen", 1990, 4, false, 7), "no-data");
    assert.equal(typeof hc.tibetanFestivalDay("berzin", "tibetan-lochen", 1990, 4, false, 7), "number");
  });
});

describe("the East Asian ages, almanac terms and augury names", () => {
  test("each age count is its source's", () => {
    const birth = hc.gregorianToFixed(2000, 6, 1);
    assert.equal(hc.chineseAge("chinese-age", birth, hc.gregorianToFixed(2012, 1, 23)), 13);
    const spring = hc.gregorianToFixed(2009, 6, 1);
    assert.equal(hc.chineseAge("lichun-age", spring, hc.gregorianToFixed(2010, 2, 3)), 1);
    assert.equal(hc.chineseAge("lichun-age", spring, hc.gregorianToFixed(2010, 2, 4)), 2);
    const eve = hc.gregorianToFixed(2023, 12, 31);
    assert.equal(hc.chineseAge("new-year-day-age", eve, eve + 1), 2);
    assert.equal(hc.chineseAge("year-age", eve, eve + 1), 1);
    refused(() => hc.chineseAge("year-age", eve, eve - 1), "no-data");
    refused(() => hc.chineseAge(/** @type {any} */ ("korean-age"), eve, eve), "unknown");
  });

  test("the almanac's terms run from 小寒, and a widow year has its Chinese names", () => {
    const terms = hc.chineseAlmanacSolarTerms(1700);
    assert.equal(terms.length, 24);
    assert.deepEqual([terms[0].position, terms[0].name, terms[23].name], [1, "小寒", "冬至"]);
    refused(() => hc.chineseAlmanacSolarTerms(1668), "no-data");
    const widow = hc.chineseMarriageAugury(4661);
    assert.deepEqual(widow.chineseNames[0], { name: "無春年", locale: "zh-Hant", region: null });
    assert.deepEqual(widow.chineseNames[1], { name: "寡婦年", locale: "zh-Hant", region: "north" });
    assert.deepEqual(hc.chineseMarriageAugury(4646).chineseNames.map((name) => name.name), ["雙春兼閏月", "双春年"]);
  });
});

describe("decimal time, the Olympiad of a day, regnal years, tekufot and named days", () => {
  test("noon is five decimal hours, and 23:59:60 has no place", () => {
    assert.deepEqual(hc.frenchDecimalTime(43_200), { hour: 5, minute: 0, second: 0, attoseconds: 0n });
    refused(() => hc.frenchDecimalTime(86_400), "no-data");
    assert.deepEqual(hc.civilFromFrenchDecimalTime(5, 0, 0), { secondsOfDay: 43_200, attoseconds: 0n });
    refused(() => hc.civilFromFrenchDecimalTime(10, 0, 0), "out-of-range");
  });

  test("the Olympiad of a day, a Seleucid king, a tekufah and a day's name", () => {
    assert.equal(hc.iocOlympiadOn(hc.gregorianToFixed(1900, 1, 1)), 1);
    refused(() => hc.iocOlympiadOn(hc.gregorianToFixed(1956, 8, 1)), "no-data");
    assert.equal(hc.iocOlympiadOn(hc.gregorianToFixed(2026, 9, 29)), 33);
    assert.deepEqual(hc.babylonianRegnalYear(1), { king: "Seleucus I Nicator", year: 1 });
    const tishrei = hc.shmuelTekufah(5786, "tishrei");
    assert.equal(tishrei.fixed, hc.gregorianToFixed(2025, 10, 7));
    // 5769's Nisan at the reckoning's nightfall on Tuesday 7 April 2009, Wednesday's Hebrew day.
    assert.deepEqual(hc.shmuelTekufah(5769, "nisan"), {
      fixed: hc.gregorianToFixed(2009, 4, 8), minutes: 1080, tekufah: "nisan", afterNightfall: true,
      civil: hc.gregorianToFixed(2009, 4, 7),
    });
    refused(() => hc.shmuelTekufah(5786, /** @type {any} */ ("adar")), "unknown");
    const raisin = hc.dayName("french-republican-arithmetic", "fr", hc.gregorianToFixed(1793, 9, 22));
    assert.deepEqual([raisin.name, raisin.naming], ["Raisin", "fr"]);
    refused(() => hc.dayName("gregory", "fr", 0), "unknown");
  });
});

describe("the 1960 office", () => {
  test("the Annunciation of 1962 is transferred from the Third Sunday of Lent", () => {
    const sunday = hc.gregorianToFixed(1962, 3, 25);
    const lines = hc.roman1960OfficeOn(sunday);
    assert.deepEqual([lines[0].role, lines[0].title, lines[0].class], ["office", "Third Sunday of Lent", "first"]);
    assert.ok(lines.some((line) => line.role === "transferred"));
    const monday = hc.roman1960OfficeOn(sunday + 1)[0];
    assert.deepEqual([monday.title, monday.transferredFrom], ["The Annunciation of the Blessed Virgin Mary", sunday]);
    refused(() => hc.roman1960OfficeOn(0), "out-of-range");
  });
});

describe("zone names", () => {
  test("Tokyo is Japan Standard Time, 日本標準時 under ja, and a field not known is refused", () => {
    assert.deepEqual(hc.zoneName("Asia/Tokyo", 1_784_000_000, "en", "zzzz"), {
      name: "Japan Standard Time", field: "zzzz", zone: "Asia/Tokyo", offset: 32_400, daylight: false,
    });
    assert.equal(hc.zoneName("Asia/Tokyo", 1_784_000_000, "ja").name, "日本標準時");
    assert.equal(hc.zoneName("Europe/Berlin", 1_784_000_000, "de", "zzzz").name, "Mitteleuropäische Sommerzeit");
    refused(() => hc.zoneName("Asia/Tokyo", 0, "en", /** @type {any} */ ("OO")), "unknown");
  });
});

describe("day periods, numbering systems, eras, groups and holiday names in a locale", () => {
  test("15:00 is in the afternoon, 5786 is Hebrew and back, and 令和 is a Japanese era", () => {
    const afternoon = hc.dayPeriod(15 * 3600, "en");
    assert.deepEqual([afternoon.half, afternoon.halfName, afternoon.period, afternoon.wide],
      ["pm", "PM", "afternoon1", "in the afternoon"]);
    const hebrew = hc.formatNumber("hebr", 5786);
    assert.equal(hc.parseNumber("hebr", hebrew), 5786);
    assert.equal(hc.parseNumber("grek", hc.formatNumber("grek", 2026)), 2026);
    refused(() => hc.formatNumber("klingon", 1), "unknown");
    refused(() => hc.parseNumber("latn", "x"), "malformed");
    assert.ok(hc.numberingSystems().some((row) => row.system === "latn" && row.digits === "0123456789"));
    assert.ok(hc.calendarEras("japanese", "ja").some((era) => era.code === "reiwa" && era.wide === "令和"));
    refused(() => hc.calendarEras("no-such", "ja"), "unknown");
  });

  test("the groups are listed, and Nayrouz has its Coptic name", () => {
    assert.equal(hc.holidayGroups("en")[0].group, "women");
    const nayrouz = hc.holidaysOnIn(hc.gregorianToFixed(2025, 9, 11), "cop").find((day) => day.table === "coptic-orthodox");
    assert.deepEqual([nayrouz?.nameInLocale, nayrouz?.nameLocale], ["ⲡⲓⲭⲗⲟⲙ ⲛ̀ⲧⲉ ϯⲣⲟⲙⲡⲓ", "cop"]);
  });
});

describe("the start of an IRIG frame", () => {
  test("a reading rounds down to its frame, which irigEncode takes", () => {
    assert.deepEqual(hc.irigFrameStart("D001", 3725, 57), { secondsOfDay: 3600, hundredths: 0, frameMicroseconds: 3_600_000_000 });
    assert.deepEqual(hc.irigFrameStart("A000", 3725, 57), { secondsOfDay: 3725, hundredths: 50, frameMicroseconds: 100_000 });
    const start = hc.irigFrameStart("B124", 76_722, 40);
    assert.equal(typeof hc.irigEncode("B124", 731_388, start.secondsOfDay), "string");
    refused(() => hc.irigFrameStart("Z000", 0), "unknown");
  });
});

describe("the new exports' i64 arguments", () => {
  test("a number past the safe range is unsafe-integer, as for every other export", () => {
    for (const call of [
      () => hc.relativeTime(2 ** 60, 0),
      () => hc.chineseAge("year-age", 2 ** 53, 0),
      () => hc.tibetanFestivalDay("berzin", "tibetan", 2 ** 60, 1, false, 1),
      () => hc.rounichi("dragon-nearest-major-cold-earlier", -(2 ** 60)),
      () => hc.formatNumber("latn", 2 ** 53),
      () => hc.zoneName("Asia/Tokyo", 2 ** 60),
    ]) {
      const error = refused(call, "unsafe-integer");
      assert.ok(error instanceof HcError);
    }
    assert.equal(hc.formatNumber("latn", 2n ** 53n + 1n), "9007199254740993");
  });
});

describe("a day the table cannot answer, the weekend, and the next holiday", () => {
  test("a gap is refused as no-data, and a weekend law not read as out-of-range", () => {
    // Victoria Day 2025 is a gap in Newfoundland and Labrador.
    const victoria = hc.gregorianToFixed(2025, 5, 19);
    refused(() => hc.holidayIsDayOff("CA", "CA-NL", victoria), "no-data");
    assert.equal(hc.holidayIsDayOff("CA", "CA-NL", hc.gregorianToFixed(2025, 12, 25)), true);
    assert.equal(hc.holidayIsDayOff("JP", "", hc.gregorianToFixed(2026, 3, 4)), false);
    // A subdivision no source was read for has its own days open.
    refused(() => hc.holidayIsDayOff("US", "US-NH", hc.gregorianToFixed(2026, 3, 4)), "no-data");
    // Kedah's weekend law before 25 November 2013 was not read.
    refused(() => hc.holidayIsDayOff("MY", "MY-02", hc.gregorianToFixed(2012, 5, 11)), "out-of-range");
    // The walk that would count or skip the open day is refused, whichever way it runs.
    refused(() => hc.holidayAddBusinessDays("CA", "CA-NL", hc.gregorianToFixed(2025, 5, 16), 1), "no-data");
    refused(() => hc.holidayBusinessDaysBetween("CA", "CA-NL", hc.gregorianToFixed(2025, 5, 16), hc.gregorianToFixed(2025, 5, 21)), "no-data");
    refused(() => hc.holidayAddBusinessDays("MY", "MY-02", hc.gregorianToFixed(2012, 5, 9), 1), "out-of-range");
  });

  test("a region's weekend is the law's, and refused where unread", () => {
    const friday = hc.gregorianToFixed(2026, 3, 6);
    assert.equal(hc.holidayIsWeekend("MY", "MY-02", friday), true);
    assert.equal(hc.holidayIsWeekend("MY", "", friday), false);
    assert.equal(hc.holidayIsWeekend("MY", "", friday + 1), true);
    assert.equal(hc.holidayIsWeekend("MY", "MY-02", friday + 2), false);
    assert.equal(hc.holidayIsWeekend("AE", "AE-SH", friday), true);
    assert.equal(hc.holidayIsWeekend("JP", "", friday + 1), true);
    refused(() => hc.holidayIsWeekend("MY", "MY-02", hc.gregorianToFixed(2013, 11, 24)), "out-of-range");
    assert.equal(hc.holidayIsWeekend("MY", "MY-02", hc.gregorianToFixed(2013, 11, 29)), true);
    refused(() => hc.holidayIsWeekend("ZZ", "", friday), "unknown");
    refused(() => hc.holidayIsWeekend("MY", "MY-99", friday), "unknown");
    // Column 14 of the tables lists the law the answer comes from.
    const malaysia = hc.holidayTables("en").find((table) => table.code === "MY");
    assert.ok(malaysia?.weekend.some((law) => law.regions.includes("MY-02")));
  });

  test("the next and the previous holiday are rows of the year, and a gap can hide a nearer one", () => {
    const next = hc.holidayNext("JP", "", hc.gregorianToFixed(2026, 4, 28));
    assert.deepEqual(next, {
      date: "2026-04-29",
      name: "Shōwa Day",
      localName: "昭和の日",
      kind: "public",
      confidence: "exact",
      substitute: false,
      observedFor: null,
      region: null,
      group: null,
      id: "showa-day",
      source: null,
      bridged: false,
    });
    const previous = hc.holidayPrevious("JP", "", hc.gregorianToFixed(2026, 5, 7));
    assert.equal(previous.date, "2026-05-06");
    assert.equal(previous.substitute, true);
    assert.equal(previous.observedFor, "2026-05-03");
    assert.equal(hc.holidayNext("JP", "JP-13", hc.gregorianToFixed(2026, 9, 1), "", "school").region, "JP-13");
    refused(() => hc.holidayNext("CA", "CA-NL", hc.gregorianToFixed(2025, 5, 1)), "no-data");
    refused(() => hc.holidayPrevious("CA", "CA-NL", hc.gregorianToFixed(2025, 7, 15)), "no-data");
    refused(() => hc.holidayNext("un-days", "", hc.gregorianToFixed(2026, 1, 1)), "no-data");
    assert.equal(hc.holidayNext("un-days", "", hc.gregorianToFixed(2026, 1, 1), "", "observance").kind, "observance");
    refused(() => hc.holidayNext("ZZ", "", 0), "unknown");
    refused(() => hc.holidayNext("JP", "", 0, "", "festival"), "unknown");
    refused(() => hc.holidayNext("JP", "", 2n ** 62n), "out-of-range");
  });

  test("Japan's 国民の休日 says it was made by a bridge, in the year and in the day", () => {
    const bridge = hc.holidaysInYear("JP", "", 2009).filter((day) => day.bridged);
    assert.deepEqual(bridge.map((day) => [day.date, day.name, day.id, day.substitute]),
      [["2009-09-22", "Citizens' Holiday", "citizens-holiday", false]]);
    // 2026 has one too, between 敬老の日 on the 21st and 秋分の日 on the 23rd; 2025 has none.
    assert.deepEqual(hc.holidaysInYear("JP", "", 2026).filter((day) => day.bridged).map((day) => day.date), ["2026-09-22"]);
    assert.equal(hc.holidaysInYear("JP", "", 2025).some((day) => day.bridged), false);
    const on = hc.holidaysOn(hc.gregorianToFixed(2009, 9, 22)).filter((row) => row.table === "JP" && row.bridged);
    assert.deepEqual(on.map((row) => row.id), ["citizens-holiday"]);
    const named = hc.holidaysOnIn(hc.gregorianToFixed(2009, 9, 22), "en").filter((row) => row.table === "JP" && row.bridged);
    assert.deepEqual(named.map((row) => row.id), ["citizens-holiday"]);
  });

  test("a region with only a weekend law has its substitute day in the day's rows", () => {
    // Awal Muharram 2025 was Friday 27 June, which Kedah moves to the Sunday.
    const rows = hc.holidaysOn(hc.gregorianToFixed(2025, 6, 29)).filter((row) => row.table === "MY" && row.region === "MY-02");
    assert.ok(rows.some((row) => row.substitute && row.observedFor === hc.gregorianToFixed(2025, 6, 27)), JSON.stringify(rows));
  });
});

describe("business days", () => {
  test("Japan's Golden Week of 2026 is skipped", () => {
    const tuesday = hc.gregorianToFixed(2026, 4, 28);
    assert.equal(hc.holidayAddBusinessDays("JP", "", tuesday, 5), hc.gregorianToFixed(2026, 5, 11));
    assert.equal(hc.holidayAddBusinessDays("JP", "", hc.gregorianToFixed(2026, 5, 11), -5), tuesday);
    assert.equal(hc.holidayBusinessDaysBetween("JP", "", tuesday - 1, hc.gregorianToFixed(2026, 5, 11)), 6);
    refused(() => hc.holidayAddBusinessDays("XX", "", tuesday, 1), "unknown");
    refused(() => hc.holidayAddBusinessDays("JP", "", tuesday, 36_501), "out-of-range");
  });
});

describe("equinox new-year margins", () => {
  test("183 BE is a fifth of a minute after the Tehran sunset", () => {
    const margin = hc.equinoxNewYearMargin("bahai-astronomical", 183);
    assert.ok(margin.minutes < 0 && margin.minutes > -0.2, String(margin.minutes));
    refused(() => hc.equinoxNewYearMargin(/** @type {any} */ ("gregory"), 2026), "unknown");
  });
});

describe("formatting by pattern", () => {
  test("Tokyo at the new year of 2026 by CLDR's and POSIX's patterns", () => {
    const instant = 1_767_225_600;
    assert.equal(hc.formatPattern("Asia/Tokyo", instant, "en", "cldr", "yyyy-MM-dd HH:mm zzzz").text,
      "2026-01-01 09:00 Japan Standard Time");
    assert.equal(hc.formatPattern("Asia/Tokyo", instant, "en", "strftime", "%Y-%m-%d %H:%M %Z").text,
      "2026-01-01 09:00 JST");
    refused(() => hc.formatPattern("Asia/Tokyo", instant, "en", /** @type {any} */ ("java"), "yyyy"), "unknown");
    refused(() => hc.formatPattern("Asia/Tokyo", instant, "en", "cldr", "'unclosed"), "malformed");
    // `%E` in the Japanese eras: 令和 begins on 1 May 2019, 15:00 UTC the day before.
    const era = (/** @type {number} */ unix) =>
      hc.formatPattern("Asia/Tokyo", unix, "ja-u-ca-japanese", "strftime", "%EC|%Ey|%EY").text;
    assert.equal(era(1_556_636_399), "平成|31|平成31年");
    assert.equal(era(1_556_636_400), "令和|1|令和元年");
  });
});
