#!/usr/bin/env node
// Time the calendar calls a page makes, in WebAssembly under Node.
//
// `describeDay`, `calendars` and `calendarList` of the JavaScript binding,
// each the shortest of several calls, for three days and three locales,
// and `holidaysOn` for two days: what the WebAssembly README's timings for
// `hc_describe_day`, `hc_calendars`, `hc_calendar_list` and
// `hc_holidays_on` were measured with. The shortest
// call is the one least disturbed by whatever else the machine is doing;
// the median is printed beside it.
//
//     node scripts/wasm-calendar-timing.mjs [--wasm <file>] [--runs <n>]
//
// The default module is the `full` layer scripts/wasm-layers.sh builds in
// the `release-compact` profile, under <target>/wasm-layers, with <target>
// being CARGO_TARGET_DIR or ./target. A module without `hc_calendar_list`
// is timed without it, and one without `hc_holidays_on` without that, so
// an older build or a smaller layer can be compared.

import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { load } from "../crates/hyper-calendar-wasm/js/hyper-calendar.js";

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const TARGET = process.env.CARGO_TARGET_DIR ?? resolve(ROOT, "target");

/** @type {Record<string, string>} */
const options = {};
const args = process.argv.slice(2);
for (let index = 0; index < args.length; index += 2) {
  options[args[index].replace(/^--/, "")] = args[index + 1];
}
const wasm = options.wasm ?? resolve(TARGET, "wasm-layers", "hyper_calendar_wasm.full.wasm");
const runs = Number(options.runs ?? "7");

const hc = await load(new Uint8Array(readFileSync(wasm)));

/**
 * The shortest and the median of `runs` timings of `call`, in milliseconds.
 *
 * @param {() => unknown} call
 * @returns {string}
 */
function timed(call) {
  const times = [];
  for (let run = 0; run < runs; run += 1) {
    const start = performance.now();
    call();
    times.push(performance.now() - start);
  }
  times.sort((a, b) => a - b);
  return `shortest ${times[0].toFixed(2)} ms, median ${times[Math.floor(runs / 2)].toFixed(2)} ms`;
}

const days = [
  ["2026-09-27", hc.gregorianToFixed(2026, 9, 27)],
  ["2026-01-01", hc.gregorianToFixed(2026, 1, 1)],
  ["1900-06-15", hc.gregorianToFixed(1900, 6, 15)],
];
const [, today] = days[0];
const lists = typeof hc.exports.hc_calendar_list === "function";
console.log(`${wasm}, ${runs} runs each`);
for (const locale of ["ja", "en", "native"]) {
  for (const [name, day] of days) {
    console.log(`describeDay(${name}, ${locale}): ${timed(() => hc.describeDay(day, locale))}`);
  }
  console.log(`calendars(2026-09-27, ${locale}): ${timed(() => hc.calendars(today, locale))}`);
  if (lists) {
    console.log(`calendarList(${locale}): ${timed(() => hc.calendarList(locale))}`);
  }
}
if (typeof hc.exports.hc_holidays_on === "function") {
  // Every table, for the costliest day of 2026 and an ordinary one.
  for (const [name, day] of [
    ["2026-01-01", hc.gregorianToFixed(2026, 1, 1)],
    ["2026-09-25", hc.gregorianToFixed(2026, 9, 25)],
  ]) {
    console.log(`holidaysOn(${name}): ${timed(() => hc.holidaysOn(day))}`);
  }
}
