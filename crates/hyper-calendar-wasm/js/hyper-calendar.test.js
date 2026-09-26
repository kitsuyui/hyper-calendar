// The binding against the module built with `--features full`: every
// method, every line format column by column, the sentinels as thrown
// errors, and the buffer protocol with a capacity too small to fit
// anything.

import assert from "node:assert/strict";
import { before, describe, test } from "node:test";

import { COLUMNS, GEOLOGIC_RANKS, HcError, HyperCalendar, METHODS, NATIVE, SENTINELS, UNITS, load } from "./hyper-calendar.js";
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
    assert.deepEqual(hc.layers(), [
      "civil", "timestamps", "calendars", "holiday", "seasons", "deep-time", "tz", "sky", "orbital",
      "planetary", "relativity",
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
    assert.equal(chinese.formatted, "癸卯年闰二月初一");
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

  test("every locale is listed with what it names", () => {
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_locales(buffer, capacity));
    for (const cells of raw) {
      assert.equal(cells.length, COLUMNS.locales.length, JSON.stringify(cells));
    }
    const rows = hc.locales();
    assert.ok(rows.length >= 27, `${rows.length} locales`);
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
    assert.equal(pentad.chineseName, "蟄蟲始振");
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
    const raw = rawRows(hc, (buffer, capacity) => hc.exports.hc_place_years_ago(66.0e6, 0.0, 0, 0, buffer, capacity));
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
    assert.equal(age.localisedName, null, "no locale, no localised name");
    const inJapanese = hc.placeYearsAgo(66.0e6, 0, "ja");
    assert.equal(inJapanese[8].localisedName, "マーストリヒチアン");
    assert.equal(inJapanese[8].name, "Maastrichtian", "the English column stays");
    assert.equal(inJapanese[2].localisedName, null, "no cosmic translation");
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
    assert.doesNotMatch(raw[0][7], /e/);
    assert.ok(hc.cosmicEvents("ja").every((row) => row.localisedName === null));
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

  test("1 January 2025 carries Vyaghata and Balava", () => {
    const day = hc.gregorianToFixed(2025, 1, 1);
    const [yoga, karana, ...rest] = hc.panchangaOfDay(day, 23.183_333, 82.5, 0, "Lahiri");
    assert.equal(rest.length, 0);
    assert.equal(yoga.limb, "yoga");
    assert.equal(yoga.number, 13);
    assert.equal(yoga.name, "Vyaghata");
    assert.equal(yoga.devanagari, "व्याघात");
    assert.equal(yoga.ayanamsa, "Lahiri (Chitrapaksha)");
    assert.ok(Math.abs(yoga.ends - vyaghataEnds) < 90, `${yoga.ends}`);
    assert.ok(yoga.began < yoga.readAt && yoga.readAt < yoga.ends);
    assert.equal(karana.limb, "karana");
    assert.equal(karana.name, "Balava");
    assert.equal(karana.devanagari, "बालव");
    assert.equal(karana.ayanamsa, null);
    assert.ok(karana.ends - balavaEnds >= 0 && karana.ends - balavaEnds < 120, `${karana.ends}`);
    assert.equal(karana.readAt, yoga.readAt);
    const [at] = hc.panchangaAt(vyaghataEnds - 600, "lahiri (chitrapaksha)");
    assert.equal(at.name, "Vyaghata");
    assert.equal(at.readAt, vyaghataEnds - 600);
    refused(() => hc.panchangaAt(vyaghataEnds, ""), "unknown");
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
    assert.deepEqual(hc.chineseMarriageAugury(4_661), { augury: "widow", lichunAtStart: false, lichunAtEnd: false });
    assert.deepEqual(hc.chineseMarriageAugury(4_646), { augury: "double-bright", lichunAtStart: true, lichunAtEnd: true });
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
    assert.ok(japanese.every((table) => table.name === null && table.localeUsed === null));
    assert.deepEqual(japanese.map((table) => table.englishName), tables.map((table) => table.englishName));
  });

  test("the liturgical year 2026 is Year A and Year II", () => {
    assert.deepEqual(hc.lectionary(hc.gregorianToFixed(2025, 11, 30)), {
      liturgicalYear: 2026, sundayCycle: "A", weekdayCycle: "II", proper: null,
    });
    assert.deepEqual(hc.lectionary(hc.gregorianToFixed(2025, 11, 29)), {
      liturgicalYear: 2025, sundayCycle: "C", weekdayCycle: "I", proper: null,
    });
    assert.equal(hc.lectionary(hc.gregorianToFixed(2026, 11, 22)).proper, 29);
    assert.equal(hc.lectionary(hc.gregorianToFixed(2026, 6, 7)).proper, 5);
    refused(() => hc.lectionary(hc.gregorianToFixed(1500, 1, 1)), "out-of-range");
  });

  test("the astronomical Easter of 2001 is 15 April, a week after its Sunday full moon", () => {
    assert.equal(hc.astronomicalEaster(2001), hc.gregorianToFixed(2001, 4, 15));
    refused(() => hc.astronomicalEaster(1582), "out-of-range");
    refused(() => hc.astronomicalEaster(2151), "out-of-range");
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
      instant: null, missing: { event: "no-noon-shadow", day: midwinter, depressionArcminutes: null },
    });
    const midsummer = hc.gregorianToFixed(2024, 6, 21);
    assert.deepEqual(hc.solarEvent("jewish-dusk-vilna-gaon", midsummer, latitude, longitude), {
      instant: null, missing: { event: "depression", day: midsummer, depressionArcminutes: 280 },
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
});

describe("time on other bodies", () => {
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
    assert.equal(hc.missionSol("Spirit", missions[3].landingUnix), 1);
    // Curiosity's sol 1000 fell within 2015-05-30 UTC.
    const day = 1_432_944_000;
    const first = hc.missionSol("curiosity", day);
    const last = hc.missionSol("curiosity", day + 86_399);
    assert.ok(first <= 1_000 && 1_000 <= last, `${first}..${last}`);
    refused(() => hc.missionSol("zhurong", 1_700_000_000), "no-data");
    refused(() => hc.missionSol("beagle-2", 1_700_000_000), "unknown");
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
    const titan = hc.bodyTime("Titan", 947_116_800);
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
    const orbit = hc.gravitationalDilation("Earth", 26_561_750);
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
    const bodies = hc.gravitatingBodies();
    assert.deepEqual(bodies.map((entry) => entry.id), ["sun", "earth", "moon", "mars", "jupiter", "sagittarius-a-star"]);
    assert.ok(bodies.every((entry) => entry.gmConstant.startsWith("GM_")));
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
    counted1.holidayIsDayOff("JP", "Tokyo", 739_880);
    counted1.loadZone("Test/Eastern", TZIF_V2_EASTERN);
    refused(() => counted1.holidaysInYear("ZZ", "", 2026), "unknown");
    refused(() => counted1.termInEffect(739_880, "mars"), "unknown");
    refused(() => counted1.loadZone("Test/Junk", new Uint8Array([1, 2, 3])), "malformed");
    assert.equal(calls.hc_alloc, calls.hc_free);
    assert.ok(calls.hc_alloc >= 8, `${calls.hc_alloc} allocations`);
  });
});
