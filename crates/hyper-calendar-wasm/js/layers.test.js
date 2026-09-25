// The binding against a build of one layer: the module built with
// `--no-default-features --features civil` loads, its own methods answer,
// and every other method refuses by name rather than failing at load.

import assert from "node:assert/strict";
import { before, test } from "node:test";

import { HcError, HyperCalendar, METHODS, load } from "./hyper-calendar.js";
import { CIVIL_WASM, moduleBytes } from "./support.js";

const HOW =
  "cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown --release --no-default-features --features civil" +
  " and copy the result to the path support.js gives, or set HC_WASM_CIVIL";

/** @type {HyperCalendar} */
let hc;

before(async () => {
  hc = await load(moduleBytes(CIVIL_WASM, HOW));
});

test("a civil build loads and names its one layer", () => {
  assert.match(hc.version(), /^\d+\.\d+\.\d+/);
  assert.deepEqual(hc.layers(), ["civil"]);
  for (const entry of METHODS) {
    const expected = entry.feature === null || entry.feature === "civil";
    assert.equal(hc.has(entry.method), expected, entry.method);
    assert.equal(typeof hc.exports[entry.export] === "function", expected, entry.export);
  }
});

test("the civil methods answer", () => {
  const rd = hc.gregorianToFixed(2026, 9, 21);
  assert.equal(rd, 739_880);
  assert.equal(hc.formatIsoDate(rd), "2026-09-21");
  assert.equal(hc.parseIsoDate("2026-09-21"), rd);
  assert.equal(hc.weekday(rd), 1);
  assert.equal(hc.fixedFromUnix(0), 719_163);
  assert.equal(hc.taiMinusUtc(1_700_000_000, true), 37);
});

test("a method of another layer throws not-exported when called, not at load", () => {
  /** @type {Record<string, () => unknown>} */
  const calls = {
    describeDay: () => hc.describeDay(739_880, "en"),
    holidayIsDayOff: () => hc.holidayIsDayOff("JP", "", 739_880),
    holidaysInYear: () => hc.holidaysInYear("JP", "", 2026),
    holidayCodes: () => hc.holidayCodes(),
    holidaysOn: () => hc.holidaysOn(739_880),
    termInEffect: () => hc.termInEffect(739_880, "japan"),
    pentadInEffect: () => hc.pentadInEffect(739_880, "japan"),
    placeYearsAgo: () => hc.placeYearsAgo(66e6),
    cosmicEvents: () => hc.cosmicEvents(),
    geologicIntervals: () => hc.geologicIntervals("eon"),
    fixedFromUnixInZone: () => hc.fixedFromUnixInZone(0, "Asia/Tokyo"),
    unixFromFixedInZone: () => hc.unixFromFixedInZone(739_880, "Asia/Tokyo"),
    loadZone: () => hc.loadZone("Asia/Tokyo", new Uint8Array(0)),
    skyAt: () => hc.skyAt(0),
    solarTermsBetween: () => hc.solarTermsBetween(0, 86_400),
    moonPhasesBetween: () => hc.moonPhasesBetween(0, 86_400),
    orbitAt: () => hc.orbitAt(21_000),
    orbitSeries: () => hc.orbitSeries(0, 21_000, 1_000),
  };
  const gated = METHODS.filter((entry) => entry.feature !== null && entry.feature !== "civil");
  assert.deepEqual(Object.keys(calls).sort(), gated.map((entry) => entry.method).sort(), "every gated method is tried");
  for (const entry of gated) {
    let thrown;
    try {
      calls[entry.method]();
    } catch (error) {
      thrown = error;
    }
    assert.ok(thrown instanceof HcError, `${entry.method}: ${String(thrown)}`);
    assert.equal(thrown.name, "not-exported", entry.method);
    assert.equal(thrown.export, entry.export, entry.method);
    assert.equal(thrown.code, null, entry.method);
    assert.equal(thrown.constant, null, entry.method);
  }
});
