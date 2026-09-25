// The binding against the module built with `--features full`: every
// method, every line format column by column, the sentinels as thrown
// errors, and the buffer protocol with a capacity too small to fit
// anything.

import assert from "node:assert/strict";
import { before, describe, test } from "node:test";

import { COLUMNS, GEOLOGIC_RANKS, HcError, HyperCalendar, METHODS, SENTINELS, load } from "./hyper-calendar.js";
import { FULL_WASM, TZIF_V2_EASTERN, moduleBytes, rawRows } from "./support.js";

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
    assert.deepEqual(hc.layers(), ["civil", "calendars", "holiday", "seasons", "deep-time", "tz", "sky", "orbital"]);
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
      assert.equal(row.formatted, null, row.id);
      if (row.error === null) {
        assert.ok(["in-use", "proleptic", "extended", "unrecorded"].includes(row.standing ?? ""), row.id);
        assert.equal(typeof row.year, "number", row.id);
      } else {
        assert.equal(row.standing, null, row.id);
        assert.equal(row.year, null, row.id);
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
      formatted: null,
    });
    const gregorian = rows.find((row) => row.id === "gregory");
    assert.ok(gregorian);
    assert.equal(gregorian.name, "Gregorian");
    assert.equal(gregorian.year, 2026);
    assert.equal(gregorian.standing, "in-use");
    assert.equal(hc.describeDay(739_880, "en").find((row) => row.id === "gregory")?.monthLabel, "September");
    // A tag with no data, or one that does not parse, falls back to the
    // root locale, whose month names are CLDR's M01..M12; so does the default.
    assert.equal(hc.describeDay(739_880, "tlh").find((row) => row.id === "gregory")?.monthLabel, "M09");
    assert.equal(hc.describeDay(739_880, "!!").find((row) => row.id === "gregory")?.monthLabel, "M09");
    assert.equal(hc.describeDay(739_880).find((row) => row.id === "gregory")?.monthLabel, "M09");
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
    refused(() => hc.holidayIsDayOff("JP", "", 2n ** 62n), "out-of-range");
    assert.throws(() => hc.holidayIsDayOff("JP", /** @type {any} */ (null), goodFriday), TypeError);
  });

  test("a year decodes column by column", () => {
    const raw = rawRows(hc, (buffer, capacity) => {
      const code = hc.alloc(2);
      new Uint8Array(hc.memory.buffer, code, 2).set([0x4a, 0x50]);
      try {
        return hc.exports.hc_holidays_in_year(code, 2, 0, 0, 2026n, buffer, capacity);
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
    });
  });

  test("a day with no year is refused", () => {
    refused(() => hc.holidaysOn(2n ** 63n - 1n), "out-of-range");
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
    assert.equal(pentad.chineseName, "蟄虫始振");
    assert.equal(pentad.japaneseName, "黄鶯睍睆");
    assert.equal(pentad.begins, hc.gregorianToFixed(2024, 2, 9));
    assert.equal(pentad.ends, hc.gregorianToFixed(2024, 2, 13));
    assert.ok(pentad.chineseAuthority.length > 0 && pentad.japaneseAuthority.length > 0);
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
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_place_years_ago(66.0e6, 0.0, buffer, capacity));
    const rows = hc.placeYearsAgo(66.0e6);
    sameShape(rows, raw);
    assert.deepEqual(
      rows.map((row) => row.kind),
      ["moment", "moment", "cosmic-epoch", "cosmic-event", "eon", "era", "period", "epoch", "age"],
    );
    assert.equal(rows[0].name, "since-big-bang");
    assert.equal(rows[0].unit, "seconds-since-big-bang");
    assert.deepEqual(rows[0].start, rows[0].end, "a point in time");
    assert.equal(rows[1].name, "before-present");
    assert.equal(rows[1].unit, "seconds-before-present");
    assert.equal(rows[2].name, "Era of galaxies");
    const age = rows[8];
    assert.equal(age.name, "Maastrichtian");
    assert.equal(age.scope, "Upper Cretaceous");
    assert.deepEqual(age.start, { value: 72.2, stdDev: 0.2, figures: 3, approximate: false });
    assert.deepEqual(age.end, { value: 66, stdDev: 0, figures: 4, approximate: false });
    assert.equal(age.unit, "megayears-before-present");
    assert.equal(age.description, null);
    assert.match(age.source, /v2026\/06/);
    assert.deepEqual(hc.placeYearsAgo(66.0e6, 1.0e6).map((row) => row.kind), rows.map((row) => row.kind));
  });

  test("the present and the future are placed too", () => {
    const now = hc.placeYearsAgo(0);
    const archaeological = now.find((row) => row.kind === "archaeological");
    assert.ok(archaeological);
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

  test("a value the crate refuses is thrown", () => {
    refused(() => hc.placeYearsAgo(Number.NaN), "out-of-range");
    refused(() => hc.placeYearsAgo(1, -1), "out-of-range");
    assert.throws(() => hc.placeYearsAgo(/** @type {any} */ ("66")), TypeError);
  });

  test("the cosmic tables are listed with their sources", () => {
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_cosmic_events(buffer, capacity));
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
    assert.doesNotMatch(raw[0][7], /e/);
  });

  test("the geologic ranks are numbered coarsest first", () => {
    GEOLOGIC_RANKS.forEach((name, number) => {
      const byName = hc.geologicIntervals(name);
      const byNumber = hc.geologicIntervals(number);
      assert.deepEqual(byName, byNumber, name);
      assert.ok(byName.length > 3, name);
      assert.ok(byName.every((row) => row.kind === name), name);
    });
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_geologic_intervals(0, buffer, capacity));
    const eons = hc.geologicIntervals("eon");
    sameShape(eons, raw);
    assert.equal(eons[0].name, "Phanerozoic");
    assert.equal(eons[0].scope, null);
    assert.deepEqual(eons[0].start, { value: 538.8, stdDev: 0.6, figures: 4, approximate: false });
    assert.deepEqual(eons[0].end, { value: 0, stdDev: 0, figures: 1, approximate: false });
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
    assert.deepEqual(small.holidayCodes(), hc.holidayCodes());
    assert.deepEqual(small.holidaysOn(739_880), hc.holidaysOn(739_880));
    assert.deepEqual(small.termInEffect(739_880, "japan"), hc.termInEffect(739_880, "japan"));
    assert.deepEqual(small.cosmicEvents(), hc.cosmicEvents());
    assert.deepEqual(small.geologicIntervals("age"), hc.geologicIntervals("age"));
    assert.deepEqual(small.holidaysInYear("JP", "", -5000), []);
    assert.deepEqual(small.skyAt(1_789_948_800), hc.skyAt(1_789_948_800));
    assert.deepEqual(small.moonPhasesBetween(0, 0), []);
    assert.deepEqual(small.orbitAt(21_000), hc.orbitAt(21_000));
    assert.deepEqual(small.orbitSeries(0, 21_000, 7_000), hc.orbitSeries(0, 21_000, 7_000));
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
    counted1.holidayIsDayOff("JP", "Tokyo", 739_880);
    counted1.loadZone("Test/Eastern", TZIF_V2_EASTERN);
    refused(() => counted1.holidaysInYear("ZZ", "", 2026), "unknown");
    refused(() => counted1.termInEffect(739_880, "mars"), "unknown");
    refused(() => counted1.loadZone("Test/Junk", new Uint8Array([1, 2, 3])), "malformed");
    assert.equal(calls.hc_alloc, calls.hc_free);
    assert.ok(calls.hc_alloc >= 8, `${calls.hc_alloc} allocations`);
  });
});
