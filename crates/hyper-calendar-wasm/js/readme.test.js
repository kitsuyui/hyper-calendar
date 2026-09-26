// The binding and the README cannot drift apart.
//
// The README's export table is the list of record for what the module
// exports and behind which feature; its column tables are the list of
// record for what each line holds. This file reads both back and holds
// the binding to them: every export has one method with the same feature,
// every decoder reads the README's columns in the README's order, and the
// tzdata script copies exactly the zones the module carries.

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { test } from "node:test";

import { COLUMNS, METHODS, SENTINELS } from "./hyper-calendar.js";
import { ROOT } from "./support.js";

const README = readFileSync(resolve(ROOT, "crates", "hyper-calendar-wasm", "README.md"), "utf8");

/**
 * The rows of the first Markdown table after a heading, as cells.
 *
 * @param {string} heading
 * @returns {string[][]}
 */
function tableAfter(heading) {
  const at = README.indexOf(`\n${heading}\n`);
  assert.ok(at >= 0, `no heading ${JSON.stringify(heading)}`);
  const lines = README.slice(at + heading.length + 2).split("\n");
  const start = lines.findIndex((line) => line.startsWith("| "));
  assert.ok(start >= 0, `no table after ${heading}`);
  const rows = [];
  for (const line of lines.slice(start)) {
    if (!line.startsWith("|")) {
      break;
    }
    rows.push(line.slice(1, -1).split(" | ").map((cell) => cell.trim()));
  }
  return rows;
}

/**
 * The column names of a `| # | Column | Holds |` table.
 *
 * @param {string} heading
 * @returns {string[]}
 */
function columnsAfter(heading) {
  const rows = tableAfter(heading);
  assert.deepEqual(rows[0].slice(0, 2), ["#", "Column"], heading);
  const body = rows.slice(2);
  body.forEach((row, index) => assert.equal(row[0], String(index + 1), `${heading} row ${index + 1}`));
  return body.map((row) => row[1]);
}

test("every export has one method with the export's feature, and no method an export the README lacks", () => {
  const exports = new Map();
  for (const line of README.split("\n")) {
    if (!line.startsWith("| `hc_")) {
      continue;
    }
    const [signature, feature] = line.slice(1, -1).split(" | ").map((cell) => cell.trim());
    const name = signature.slice(1, signature.indexOf("("));
    exports.set(name, feature === "always" ? null : feature.replaceAll("`", ""));
  }
  assert.ok(exports.size >= 29, `${exports.size} exports`);
  assert.deepEqual(
    [...exports.keys()].sort(),
    METHODS.map((entry) => entry.export).sort(),
  );
  for (const entry of METHODS) {
    assert.equal(entry.feature, exports.get(entry.export), entry.export);
  }
  assert.equal(new Set(METHODS.map((entry) => entry.method)).size, METHODS.length, "one method each");
  // The count the README states is the count the binding has.
  const stated = README.match(/\n(\d+) functions\. /);
  assert.ok(stated, "the README counts its exports");
  assert.equal(Number(stated[1]), METHODS.length);
});

test("the sentinels are the README's, value by value", () => {
  const rows = tableAfter("### Error sentinels").slice(2);
  const named = rows
    .filter((row) => row[0] !== "`HC_ERR_FLOOR`")
    .map((row) => ({ constant: row[0].replaceAll("`", ""), code: BigInt(row[1].replaceAll("_", "")) }));
  assert.deepEqual(
    SENTINELS.map((sentinel) => ({ constant: sentinel.constant, code: sentinel.code })),
    named,
  );
});

test("describeDay reads the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.describeDay], columnsAfter("## Every calendar"));
});

test("calendarUnits, calendars and locales read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.calendarUnits], columnsAfter("## Units of a calendar"));
  assert.deepEqual([...COLUMNS.calendars], columnsAfter("## The calendars"));
  assert.deepEqual([...COLUMNS.locales], columnsAfter("## The locales"));
  assert.deepEqual([...COLUMNS.gregorianAdoption], columnsAfter("## Gregorian adoption"));
});

test("holidaysOn reads the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.holidaysOn], columnsAfter("### One day, every table"));
});

test("holidaysInYear reads the columns the README lists", () => {
  // The year's line format is stated in prose rather than a table: the
  // seven things in the order they are named.
  const stated = README.match(
    /writes a year as tab-separated\s+lines — (.*?) — and, called with a/s,
  );
  assert.ok(stated, "the README describes hc_holidays_in_year's line");
  const named = stated[1].replace(/\s+/g, " ").split(", ").flatMap((part) => part.split(" and "));
  assert.equal(named.length, COLUMNS.holidaysInYear.length, stated[1]);
});

test("the almanac line has the README's columns", () => {
  const rows = tableAfter("## Almanac");
  // The first table is the meridians; the second, the columns.
  const at = README.indexOf("| # | `hc_term_in_effect` | `hc_pentad_in_effect` |");
  assert.ok(at >= 0 && rows.length > 0);
  const lines = README.slice(at).split("\n");
  const body = [];
  for (const line of lines.slice(2)) {
    if (!line.startsWith("|")) {
      break;
    }
    body.push(line);
  }
  assert.equal(body.length, COLUMNS.term.length);
});

test("the deep-time rows read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.deepTime], columnsAfter("## Deep time"));
});

test("skyAt reads the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.sky], columnsAfter("## The sky"));
});

test("the sky events have the README's columns", () => {
  const at = README.indexOf("| # | `hc_solar_terms_between` | `hc_moon_phases_between` |");
  assert.ok(at >= 0);
  const body = [];
  for (const line of README.slice(at).split("\n").slice(2)) {
    if (!line.startsWith("|")) {
      break;
    }
    body.push(line);
  }
  assert.equal(body.length, COLUMNS.skyEvent.length);
});

test("orbitAt reads the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.orbit], columnsAfter("## The orbit"));
});

test("orbitSeries reads the epoch, then orbitAt's columns, as the README says", () => {
  assert.deepEqual([...COLUMNS.orbitSeries], ["years before 1950", ...COLUMNS.orbit]);
  assert.match(README, /with the epoch in years before 1950 as a first\s+column before the eleven above/);
});

/**
 * The column names of the `| # | Column | Holds |` table whose first row
 * names `first`.
 *
 * @param {string} first
 * @returns {string[]}
 */
function columnsOfTableStarting(first) {
  const at = README.indexOf(`\n| 1 | ${first} |`);
  assert.ok(at >= 0, `no table whose first column is ${first}`);
  const names = [];
  for (const line of README.slice(at + 1).split("\n")) {
    if (!line.startsWith("|")) {
      break;
    }
    const cells = line.slice(1, -1).split(" | ").map((cell) => cell.trim());
    assert.equal(cells[0], String(names.length + 1), line);
    names.push(cells[1]);
  }
  return names;
}

test("the time-scale lines read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.tai64], columnsAfter("### TAI64 labels"));
  assert.deepEqual([...COLUMNS.gnssWeek], columnsAfter("### GNSS weeks"));
  assert.deepEqual([...COLUMNS.glonassDate], columnsAfter("### GLONASS dates"));
  assert.deepEqual([...COLUMNS.oleAutomation], columnsAfter("### OLE Automation dates"));
  assert.deepEqual([...COLUMNS.excel1900Day], columnsAfter("### Excel 1900 serials"));
  // hc_gnss_to_tai's two cells are stated in prose, as the last two of
  // hc_tai64_decode's line.
  assert.match(README, /one line of two\s+cells, the TAI seconds and the attoseconds/);
  assert.deepEqual([...COLUMNS.taiInstant], COLUMNS.tai64.slice(1));
});

test("the pañcāṅga, the tables and the lectionary read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.panchanga], columnsAfter("## The pañcāṅga"));
  assert.deepEqual([...COLUMNS.marriageAugury], columnsAfter("### The marriage augury"));
  assert.deepEqual([...COLUMNS.holidayTables], columnsAfter("### The tables"));
  assert.deepEqual([...COLUMNS.lectionary], columnsAfter("### The liturgical year"));
});

test("the Earth's rotation and the Sun's hours read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.value], columnsAfter("## The Earth's rotation"));
  assert.deepEqual([...COLUMNS.solarTime], columnsAfter("## The Sun's hours"));
  assert.deepEqual([...COLUMNS.solarEvent], columnsOfTableStarting("instant"));
});

test("the layer table names every feature the binding knows", () => {
  const features = new Set(METHODS.map((entry) => entry.feature).filter((feature) => feature !== null));
  for (const feature of features) {
    assert.ok(README.includes(`\n| \`${feature}\` `), `the README's layer table has no row for ${feature}`);
  }
});

test("the tzdata script copies the zones the module carries", () => {
  const builtin = readFileSync(resolve(ROOT, "crates", "hc-tz", "src", "builtin.rs"), "utf8");
  const carried = [...builtin.matchAll(/^\s+id: "([A-Za-z_/]+)",$/gm)].map((match) => match[1]).sort();
  assert.ok(carried.length >= 17, `${carried.length} built-in zones`);
  const script = readFileSync(resolve(ROOT, "scripts", "wasm-tzdata.sh"), "utf8");
  const listed = script.match(/^zones="([^"]*)"$/m);
  assert.ok(listed, "the script lists its zones in one `zones=` line");
  assert.deepEqual(listed[1].split(/\s+/).filter((zone) => zone.length > 0).sort(), carried);
  // And the README names each of them.
  for (const zone of carried) {
    assert.ok(README.includes(`\`${zone}\``), `the README does not name ${zone}`);
  }
});
