// The lines of the layers that carry quantities, exact ratios, lists and
// uncertainty: `relativity`, `deep-time` and `orbital` beyond their first
// exports, and `uncertainty`, `units`, `fiscal`, `name-days` and
// `attributes`. Each export is read column by column against the README's
// table of its columns, and its refusals are the thrown names.

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { before, test } from "node:test";

import { COLUMNS, HcError, SENTINELS, load } from "./hyper-calendar.js";
import { FULL_WASM, ROOT, moduleBytes } from "./support.js";

const HOW = "cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown --release --features full";
const README = readFileSync(resolve(ROOT, "crates", "hyper-calendar-wasm", "README.md"), "utf8");

/** @type {import("./hyper-calendar.js").HyperCalendar} */
let hc;

before(async () => {
  hc = await load(moduleBytes(FULL_WASM, HOW));
});

/**
 * The column names of the `| # | Column | Holds |` table after a heading.
 *
 * @param {string} heading
 * @returns {string[]}
 */
function columnsAfter(heading) {
  const at = README.indexOf(`\n${heading}\n`);
  assert.ok(at >= 0, `no heading ${JSON.stringify(heading)}`);
  const lines = README.slice(at + heading.length + 2).split("\n");
  const start = lines.findIndex((line) => line.startsWith("| "));
  assert.ok(start >= 0, `no table after ${heading}`);
  const names = [];
  for (const line of lines.slice(start + 2)) {
    if (!line.startsWith("|")) {
      break;
    }
    const cells = line.slice(1, -1).split(" | ").map((cell) => cell.trim());
    assert.equal(cells[0], String(names.length + 1), `${heading}: ${line}`);
    names.push(cells[1]);
  }
  return names;
}

/**
 * Assert that a call throws the `HcError` of the given name.
 *
 * @param {() => unknown} call
 * @param {string} name
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
  const sentinel = SENTINELS.find((candidate) => candidate.code === thrown.code);
  assert.ok(sentinel, `unknown code ${thrown.code}`);
}

test("orbitRateOffset reads hc_orbit_rate_offset column by column", () => {
  const result = hc.orbitRateOffset("earth", 26561750.0, 6378137.0);
  assert.deepEqual(result.id, "earth");
  assert.deepEqual(result.gmConstant, "GM_EARTH");
  assert.deepEqual([...COLUMNS.orbitRateOffset], columnsAfter("### A clock in orbit"));
  refused(() => hc.orbitRateOffset("vulcan", 26561750.0, 6378137.0), "unknown");
  refused(() => hc.orbitRateOffset("earth", 0.0, 6378137.0), "out-of-range");
});

test("rocket reads hc_rocket column by column", () => {
  const result = hc.rocket(9.80665, 31557600.0);
  assert.deepEqual(result.constants, "SPEED_OF_LIGHT;LIGHT_YEAR".split(";"));
  assert.deepEqual([...COLUMNS.rocket], columnsAfter("### A rocket from rest"));
  refused(() => hc.rocket(0.0, 1.0), "out-of-range");
  refused(() => hc.rocket(9.80665, -1.0), "out-of-range");
});

test("flipAndBurn reads hc_flip_and_burn column by column", () => {
  const result = hc.flipAndBurn(9.80665, 2.365182618145e+22);
  assert.deepEqual(result.constants, "SPEED_OF_LIGHT;JULIAN_YEAR_SECONDS".split(";"));
  assert.deepEqual([...COLUMNS.flipAndBurn], columnsAfter("### A flip-and-burn voyage"));
  refused(() => hc.flipAndBurn(0.0, 1e+16), "out-of-range");
  refused(() => hc.flipAndBurn(9.80665, -1.0), "out-of-range");
});

test("doppler reads hc_doppler column by column", () => {
  const result = hc.doppler(0.6, 1.0);
  assert.deepEqual(result.factor, Number("2"));
  assert.deepEqual([...COLUMNS.doppler], columnsAfter("### The Doppler shift"));
  refused(() => hc.doppler(0.6, 1.5), "out-of-range");
  refused(() => hc.doppler(1.0, 0.0), "out-of-range");
});

test("velocityAdd reads hc_velocity_add column by column", () => {
  const result = hc.velocityAdd(0.999, 0.999);
  assert.deepEqual([...COLUMNS.velocityAdd], columnsAfter("### Composing velocities"));
  refused(() => hc.velocityAdd(1.0, 0.5), "out-of-range");
  refused(() => hc.velocityAdd(0.5, Number.NaN), "out-of-range");
});

test("schwarzschildRadius reads hc_schwarzschild_radius column by column", () => {
  const result = hc.schwarzschildRadius("sun");
  assert.deepEqual(result.id, "sun");
  assert.deepEqual(result.gmConstant, "GM_SUN");
  assert.deepEqual([...COLUMNS.schwarzschildRadius], columnsAfter("### The Schwarzschild radius"));
  refused(() => hc.schwarzschildRadius("Sagittarius A*"), "unknown");
});

test("properTimeUncertain reads hc_proper_time_uncertain column by column", () => {
  const result = hc.properTimeUncertain(179875474.8, 1000.0, 1000000000.0);
  assert.deepEqual(result.constants, "SPEED_OF_LIGHT".split(";"));
  assert.deepEqual([...COLUMNS.properTimeUncertain], columnsAfter("### An uncertain speed"));
  refused(() => hc.properTimeUncertain(179875474.8, -1.0, 1000000000.0), "out-of-range");
  refused(() => hc.properTimeUncertain(299792458.0, 1.0, 1000000000.0), "out-of-range");
});

test("planckUnits reads hc_planck_units column by column", () => {
  const result = hc.planckUnits();
  assert.ok(Array.isArray(result) && result.length > 0);
  assert.deepEqual(result[0].symbol, "c");
  assert.deepEqual([...COLUMNS.planckUnits], columnsAfter("### The Planck units"));
});

test("bpConvert reads hc_bp_convert column by column", () => {
  const result = hc.bpConvert(11650.0, 5.0, "bp", "b2k");
  assert.deepEqual(result.from, "bp");
  assert.deepEqual(result.to, "b2k");
  assert.deepEqual(result.converted, Number("11700"));
  assert.deepEqual(result.label, "11700 b2k");
  assert.deepEqual([...COLUMNS.bpConvert], columnsAfter("### Datums for a calendar age"));
  refused(() => hc.bpConvert(3200.0, 50.0, "radiocarbon-bp", "bp"), "no-data");
  refused(() => hc.bpConvert(1.0, 0.0, "bp", "ad"), "unknown");
});

test("deepConvert reads hc_deep_convert column by column", () => {
  const result = hc.deepConvert(13.787, 0.02, "gigayear", "second");
  assert.deepEqual(result.from, "gigayear");
  assert.deepEqual(result.to, "second");
  assert.deepEqual(result.exact, true);
  assert.deepEqual([...COLUMNS.deepConvert], columnsAfter("### Magnitudes of time"));
  refused(() => hc.deepConvert(1.0, 0.0, "gigayear", "furlong"), "unknown");
  refused(() => hc.deepConvert(1.0, -1.0, "gigayear", "second"), "out-of-range");
});

test("deepCompare reads hc_deep_compare column by column", () => {
  const result = hc.deepCompare(13.787, 0.02, "gigayear", 1.0, 0.0, "planck-time");
  assert.deepEqual(result.overlap, false);
  assert.deepEqual(result.order, Number("1"));
  assert.deepEqual([...COLUMNS.deepCompare], columnsAfter("### Comparing magnitudes"));
  refused(() => hc.deepCompare(0.0, 0.0, "day", 1.0, 0.0, "day"), "out-of-range");
  refused(() => hc.deepCompare(1.0, 0.0, "day", 1.0, 0.0, "fortnight"), "unknown");
});

test("dailyInsolation reads hc_daily_insolation column by column", () => {
  const result = hc.dailyInsolation(0.0, 65.0, 90.0);
  assert.deepEqual(result.solarConstant, Number("1360"));
  assert.deepEqual([...COLUMNS.dailyInsolation], columnsAfter("### Insolation at any latitude"));
  refused(() => hc.dailyInsolation(0.0, 91.0, 90.0), "out-of-range");
  refused(() => hc.dailyInsolation(0.0, 65.0, 361.0), "out-of-range");
});

test("edtfParse reads hc_edtf_parse column by column", () => {
  const result = hc.edtfParse("1984");
  assert.ok(Array.isArray(result) && result.length > 0);
  assert.deepEqual(result[0].role, "value");
  assert.deepEqual(result[0].kind, "date");
  assert.deepEqual(result[0].text, "1984");
  assert.deepEqual(result[0].precision, "year");
  assert.deepEqual(result[0].qualifier, "certain");
  assert.deepEqual([...COLUMNS.edtfParse], columnsAfter("### EDTF dates"));
  refused(() => hc.edtfParse("1985-04-12T23:20:30Z"), "malformed");
});

test("edtfRelations reads hc_edtf_relations column by column", () => {
  const result = hc.edtfRelations("1984", "1986");
  assert.deepEqual(result.relations, "before".split(";"));
  assert.deepEqual(result.symbols, "<".split(";"));
  assert.deepEqual(result.definitelyBefore, true);
  assert.deepEqual([...COLUMNS.edtfRelations], columnsAfter("### EDTF relations"));
  refused(() => hc.edtfRelations("1984", "nonsense"), "malformed");
});

test("significant reads hc_significant column by column", () => {
  const result = hc.significant(13800000000.0, 3);
  assert.deepEqual(result.value, Number("13800000000"));
  assert.deepEqual(result.text, "1.38e10");
  assert.deepEqual([...COLUMNS.significant], columnsAfter("### Significant figures"));
  refused(() => hc.significant(1.0, 0), "out-of-range");
  refused(() => hc.significant(1.0, 18), "out-of-range");
});

test("significantOp reads hc_significant_op column by column", () => {
  const result = hc.significantOp("add", 100.0, 4, 0.001, 1);
  assert.deepEqual(result.operation, "add");
  assert.deepEqual(result.text, "100.0");
  assert.deepEqual([...COLUMNS.significantOp], columnsAfter("### Arithmetic on significant figures"));
  refused(() => hc.significantOp("mod", 2.0, 3, 1.0, 3), "unknown");
  refused(() => hc.significantOp("div", 1.0, 3, 0.0, 3), "out-of-range");
});

test("uncertain reads hc_uncertain column by column", () => {
  const result = hc.uncertain(3200.0, 50.0);
  assert.deepEqual(result.value, Number("3200"));
  assert.deepEqual(result.stdDev, Number("50"));
  assert.deepEqual(result.text, "3200 ± 50");
  assert.deepEqual(result.low1σ, Number("3150"));
  assert.deepEqual(result.high1σ, Number("3250"));
  assert.deepEqual([...COLUMNS.uncertain], columnsAfter("### Gaussian quantities"));
  refused(() => hc.uncertain(1.0, -1.0), "out-of-range");
});

test("uncertainOp reads hc_uncertain_op column by column", () => {
  const result = hc.uncertainOp("add", 10.0, 3.0, 20.0, 4.0);
  assert.deepEqual(result.operation, "add");
  assert.deepEqual(result.value, Number("30"));
  assert.deepEqual(result.stdDev, Number("5"));
  assert.deepEqual([...COLUMNS.uncertainOp], columnsAfter("### Arithmetic on Gaussian quantities"));
  refused(() => hc.uncertainOp("sqrt", 1.0, 0.0, 2.0, 0.0), "unknown");
  refused(() => hc.uncertainOp("div", 1.0, 1.0, 0.0, 1.0), "out-of-range");
});

test("interval reads hc_interval column by column", () => {
  const result = hc.interval("add", 1, 3, 10, 20);
  assert.deepEqual(result.operation, "add");
  assert.deepEqual(result.empty, false);
  assert.deepEqual(result.lowSeconds, BigInt("11"));
  assert.deepEqual(result.highSeconds, BigInt("23"));
  assert.deepEqual([...COLUMNS.interval], columnsAfter("### Intervals of time"));
  refused(() => hc.interval("add", 3, 1, 0, 1), "out-of-range");
  refused(() => hc.interval("scale", 0, 1, 0, 1), "unknown");
});

test("units reads hc_units column by column", () => {
  const result = hc.units();
  assert.ok(Array.isArray(result) && result.length > 0);
  assert.deepEqual(result[0].id, "quectosecond");
  assert.deepEqual(result[0].symbol, "qs");
  assert.deepEqual([...COLUMNS.units], columnsAfter("### The units"));
});

test("unitConvert reads hc_unit_convert column by column", () => {
  const result = hc.unitConvert(2, 1, "hour", "millisecond");
  assert.deepEqual(result.from, "hour");
  assert.deepEqual(result.to, "millisecond");
  assert.deepEqual(result.convertedNumerator, BigInt("7200000"));
  assert.deepEqual(result.whole, true);
  assert.deepEqual([...COLUMNS.unitConvert], columnsAfter("### Converting units"));
  refused(() => hc.unitConvert(1, 0, "second", "day"), "out-of-range");
  refused(() => hc.unitConvert(1, 1, "second", "furlong"), "unknown");
});

test("rates reads hc_rates column by column", () => {
  const result = hc.rates();
  assert.ok(Array.isArray(result) && result.length > 0);
  assert.deepEqual(result[0].kind, "frame");
  assert.deepEqual([...COLUMNS.rates], columnsAfter("### Frame and sample rates"));
});

test("framePeriod reads hc_frame_period column by column", () => {
  const result = hc.framePeriod("24", "flick");
  assert.deepEqual(result.rate, "24");
  assert.deepEqual(result.kind, "frame");
  assert.deepEqual(result.countNumerator, BigInt("29400000"));
  assert.deepEqual(result.wholeFlicks, true);
  assert.deepEqual([...COLUMNS.framePeriod], columnsAfter("### Frame and sample periods"));
  refused(() => hc.framePeriod("0", "second"), "out-of-range");
  refused(() => hc.framePeriod("fast", "second"), "unknown");
});

test("tempo reads hc_tempo column by column", () => {
  const result = hc.tempo(120, 1, 2, 0, 0, 0, 2);
  assert.deepEqual(result.beatNumerator, BigInt("1"));
  assert.deepEqual(result.beatDenominator, BigInt("2"));
  assert.deepEqual(result.midiMicroseconds, Number("500000"));
  assert.deepEqual(result.midiExact, true);
  assert.deepEqual([...COLUMNS.tempo], columnsAfter("### Notes at a tempo"));
  refused(() => hc.tempo(0, 1, 2, 0, 0, 0, 2), "out-of-range");
  refused(() => hc.tempo(120, 1, 2, 0, 0, 3, 2), "out-of-range");
});

test("fiscalProfiles reads hc_fiscal_profiles column by column", () => {
  const result = hc.fiscalProfiles();
  assert.ok(Array.isArray(result) && result.length > 0);
  assert.deepEqual(result[0].country, "AU");
  assert.deepEqual([...COLUMNS.fiscalProfiles], columnsAfter("### The year systems"));
});

test("fiscalYearOn reads hc_fiscal_year_on column by column", () => {
  const result = hc.fiscalYearOn("JP", "government", 739000);
  assert.ok(Array.isArray(result) && result.length > 0);
  assert.deepEqual(result[0].country, "JP");
  assert.deepEqual(result[0].kind, "government");
  assert.deepEqual(result[0].status, "in-force");
  assert.deepEqual([...COLUMNS.fiscalYearOn], columnsAfter("### The year of a day"));
  refused(() => hc.fiscalYearOn("XX", "", 739000), "unknown");
  refused(() => hc.fiscalYearOn("JP", "personal-tax", 739000), "no-data");
});

test("fiscalYearSpan reads hc_fiscal_year_span column by column", () => {
  const result = hc.fiscalYearSpan("JP", "government", 2024);
  assert.ok(Array.isArray(result) && result.length > 0);
  assert.deepEqual(result[0].country, "JP");
  assert.deepEqual(result[0].status, "in-force");
  assert.deepEqual(result[0].label, Number("2024"));
  assert.deepEqual([...COLUMNS.fiscalYearSpan], columnsAfter("### The span of a year"));
  refused(() => hc.fiscalYearSpan("ZZ", "", 2024), "unknown");
  refused(() => hc.fiscalYearSpan("JP", "personal-tax", 2024), "no-data");
});

test("weekYearSystems reads hc_week_year_systems column by column", () => {
  const result = hc.weekYearSystems();
  assert.ok(Array.isArray(result) && result.length > 0);
  assert.deepEqual(result[0].id, "nrf-4-5-4");
  assert.deepEqual([...COLUMNS.weekYearSystems], columnsAfter("### Years of whole weeks"));
});

test("weekYearOn reads hc_week_year_on column by column", () => {
  const result = hc.weekYearOn("nrf-4-5-4", 738580);
  assert.deepEqual(result.system, "nrf-4-5-4");
  assert.deepEqual(result.long, true);
  assert.deepEqual([...COLUMNS.weekYearOn], columnsAfter("### A day in a year of whole weeks"));
  refused(() => hc.weekYearOn("nrf-4-5-5", 0), "unknown");
  refused(() => hc.weekYearOn("nrf-4-5-4", 9223372036854775807n), "out-of-range");
});

test("nameDayLists reads hc_name_day_lists column by column", () => {
  const result = hc.nameDayLists();
  assert.ok(Array.isArray(result) && result.length > 0);
  assert.deepEqual(result[0].kind, "list");
  assert.deepEqual(result[0].id, "lv-extended-2023");
  assert.deepEqual([...COLUMNS.nameDayLists], columnsAfter("### The lists and the gaps"));
});

test("nameDaysOn reads hc_name_days_on column by column", () => {
  const result = hc.nameDaysOn("lv", 739061);
  assert.ok(Array.isArray(result) && result.length > 0);
  assert.deepEqual(result[0].kind, "list");
  assert.deepEqual([...COLUMNS.nameDaysOn], columnsAfter("### The names of a day"));
  refused(() => hc.nameDaysOn("jp", 739061), "unknown");
});

test("nameDay reads hc_name_day column by column", () => {
  const result = hc.nameDay("lv", "Jānis", 2024);
  assert.ok(Array.isArray(result) && result.length > 0);
  assert.deepEqual(result[0].kind, "list");
  assert.deepEqual([...COLUMNS.nameDay], columnsAfter("### The days of a name"));
  refused(() => hc.nameDay("jp", "Jānis", 2026), "unknown");
});

test("attributionAuthorities reads hc_attribution_authorities column by column", () => {
  const result = hc.attributionAuthorities("");
  assert.ok(Array.isArray(result) && result.length > 0);
  assert.deepEqual(result[0].kind, "authority");
  assert.deepEqual([...COLUMNS.attributionAuthorities], columnsAfter("### The attribution lists"));
  refused(() => hc.attributionAuthorities("gemstone"), "unknown");
});

test("attributions reads hc_attributions column by column", () => {
  const result = hc.attributions("birthstone", 1);
  assert.ok(Array.isArray(result) && result.length > 0);
  assert.deepEqual(result[0].subject, "birthstone");
  assert.deepEqual(result[0].attributions, "garnet".split(";"));
  assert.deepEqual(result[0].agreed, true);
  assert.deepEqual([...COLUMNS.attributions], columnsAfter("### What the lists say of a key"));
  refused(() => hc.attributions("birthstone", 13), "out-of-range");
  refused(() => hc.attributions("gemstone", 1), "unknown");
});

test("attributionsOn reads hc_attributions_on column by column", () => {
  const result = hc.attributionsOn(739887, "");
  assert.ok(Array.isArray(result) && result.length > 0);
  assert.deepEqual(result[0].subject, "birthstone");
  assert.deepEqual([...COLUMNS.attributionsOn], columnsAfter("### The attributions of a day"));
  refused(() => hc.attributionsOn(739887, "nowhere"), "unknown");
});

test("harvestMoon reads hc_harvest_moon column by column", () => {
  const result = hc.harvestMoon(2025, "");
  assert.deepEqual(result.month, Number("10"));
  assert.deepEqual(result.septemberMoon, "Corn Moon");
  assert.deepEqual([...COLUMNS.harvestMoon], columnsAfter("### The Harvest Moon"));
  refused(() => hc.harvestMoon(5000, ""), "out-of-range");
  refused(() => hc.harvestMoon(2026, "elsewhere"), "unknown");
});

test("a few answers the Rust tests pin, read through the binding", () => {
  // Two ships at 0.999 compose to 0.9999995 with 1 - beta of 5.005e-7.
  const composed = hc.velocityAdd(0.999, 0.999);
  assert.ok(Math.abs(composed.oneMinusComposedBeta / 5.005_002_499_998_749e-7 - 1) < 1e-13);
  // 1 g to Andromeda is 28.6 years aboard.
  const flip = hc.flipAndBurn(9.806_65, 2.5e6 * 9_460_730_472_580_800);
  assert.ok(Math.abs(flip.properYears - 28.603_418_344_136_22) < 1e-9);
  // A flick is a 705 600 000th of a second, held exactly.
  const flick = hc.units().find((unit) => unit.id === "flick");
  assert.deepEqual([flick?.secondsNumerator, flick?.secondsDenominator], [1n, 705_600_000n]);
  // 1 January 2026 is in the Japanese 2025年度 and the United States' FY 2026.
  const day = 739_617;
  const japan = hc.fiscalYearOn("JP", "government", day)[0];
  const us = hc.fiscalYearOn("US", "government", day).find((year) => year.status === "in-force");
  assert.deepEqual([japan.label, us?.label], [2025, 2026]);
  // The same four exports as the Rust tests read: the 2025 Harvest Moon is in October.
  assert.equal(hc.harvestMoon(2025).septemberMoon, "Corn Moon");
});
