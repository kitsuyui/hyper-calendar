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

test("the JavaScript method table has a row for every method, each with its export", () => {
  const start = README.indexOf("| Method | Export | Answers with |");
  assert.ok(start >= 0, "the method table");
  const rows = README.slice(start).split("\n\n")[0].split("\n").slice(2);
  /** @type {Map<string, string[]>} */
  const listed = new Map();
  /** @type {string[]} */
  const repeated = [];
  for (const row of rows) {
    const [methods, exports] = row.slice(1, -1).split(" | ");
    const names = [...methods.matchAll(/`(\w+)\(/g)].map((match) => match[1]);
    const exported = [...exports.matchAll(/`(hc_\w+)`/g)].map((match) => match[1]);
    for (const name of names) {
      if (listed.has(name)) {
        repeated.push(name);
      }
      listed.set(name, exported);
    }
  }
  // A method is listed once: a merge that adds its row twice fails here.
  assert.deepEqual(repeated, [], "methods the table lists twice");
  const missing = METHODS.filter((entry) => !listed.has(entry.method)).map((entry) => entry.method);
  assert.deepEqual(missing, [], "methods the table lacks");
  for (const entry of METHODS) {
    assert.ok(listed.get(entry.method)?.includes(entry.export), `${entry.method}: ${entry.export}`);
  }
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
  // hc_parse_date writes hc_describe_day's columns, then the fixed day.
  assert.deepEqual([...COLUMNS.parseDate], [...COLUMNS.describeDay, "fixed"]);
});

test("calendarUnits, calendars and locales read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.calendarUnits], columnsAfter("## Units of a calendar"));
  assert.deepEqual([...COLUMNS.calendars], columnsAfter("## The calendars"));
  assert.deepEqual([...COLUMNS.calendarList], columnsAfter("## The calendar list"));
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
  assert.deepEqual([...COLUMNS.utcFromTai], columnsAfter("### The TAI–UTC bridge"));
  assert.match(README, /`hc_tai_from_unix` writes one line of two cells, the TAI\s+seconds and the attoseconds/);
  assert.deepEqual([...COLUMNS.tai64PosixPlus10], columnsAfter("### TAI64 labels on a POSIX clock"));
  assert.deepEqual([...COLUMNS.uuidTimestamp], columnsAfter("### UUID timestamps"));
  assert.deepEqual([...COLUMNS.ntpResolve], columnsAfter("### NTP eras"));
  assert.deepEqual([...COLUMNS.uuidTimestampEncode], columnsAfter("### UUID timestamps from an instant"));
  assert.deepEqual([...COLUMNS.ntpEncode], columnsAfter("### NTP dates from an instant"));
  assert.deepEqual([...COLUMNS.fatDecode], columnsAfter("### FAT date and time words"));
  assert.match(README, /one line of two cells, the date word and the time word/);
  assert.deepEqual([...COLUMNS.epoch], columnsAfter("### Julian and Besselian epochs"));
  assert.match(README, /one line of three cells: the notation's letter,\s+the TT seconds and the attoseconds/);
  assert.deepEqual([...COLUMNS.ttBipm], columnsAfter("### TT(BIPM)"));
});

test("the renamed months, the Asian days, the Holy Years and the ranks read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.namingPeriod], columnsAfter("## Renamed months and weekdays"));
  assert.deepEqual([...COLUMNS.asianDay], columnsAfter("## The Asian calendar's days"));
  assert.deepEqual([...COLUMNS.holyYear], columnsAfter("### Holy Years"));
  assert.deepEqual([...COLUMNS.commonWorship], columnsAfter("### Ranks of the Common Worship calendar"));
});

test("the decans and the Heliocentric Julian Date read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.decan], columnsAfter("### Decans"));
  assert.deepEqual([...COLUMNS.hjdUtc], columnsAfter("## The Heliocentric Julian Date"));
  assert.match(README, /`hc_hjd_tt` writes one line of the first two of those cells/);
  assert.deepEqual([...COLUMNS.hjdTt], COLUMNS.hjdUtc.slice(0, 2));
});

test("the pañcāṅga, the tables and the lectionary read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.panchanga], columnsAfter("## The pañcāṅga"));
  assert.deepEqual([...COLUMNS.marriageAugury], columnsAfter("### The marriage augury"));
  assert.deepEqual([...COLUMNS.holidayTables], columnsAfter("### The tables"));
  assert.deepEqual([...COLUMNS.lectionary], columnsAfter("### The liturgical year"));
  assert.deepEqual([...COLUMNS.zones], columnsAfter("### Where each zone is"));
  assert.deepEqual([...COLUMNS.zoneOffset], columnsAfter("### A zone's offset"));
});

test("the Earth's rotation and the Sun's hours read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.value], columnsAfter("## The Earth's rotation"));
  assert.deepEqual([...COLUMNS.solarTime], columnsAfter("## The Sun's hours"));
  assert.deepEqual([...COLUMNS.solarEvent], columnsOfTableStarting("instant"));
  assert.deepEqual([...COLUMNS.horizons], columnsAfter("### Horizons"));
  assert.deepEqual([...COLUMNS.solarCrossing], columnsAfter("### Sunrise and sunset"));
});

test("the Hindu date, the Siddhānta's sky and the crescent read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.hinduLunarDate], columnsAfter("## The Hindu lunisolar date"));
  assert.deepEqual([...COLUMNS.suryaSiddhanta], columnsAfter("### The Sūrya Siddhānta's sky"));
  assert.match(README, /`hc_surya_siddhanta_sunrise\([^)]*\)`[^.]*writes one line\s+of one cell, the instant/);
  assert.deepEqual([...COLUMNS.crescent], columnsAfter("## The young crescent"));
});

test("the planetary lines read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.marsTime], columnsAfter("## Mars time"));
  assert.deepEqual([...COLUMNS.missions], columnsAfter("### Mission sols"));
  assert.deepEqual([...COLUMNS.bodies], columnsAfter("## Other bodies"));
  assert.deepEqual([...COLUMNS.bodyTime], columnsAfter("### Local time on a body"));
  assert.deepEqual([...COLUMNS.circadDate], columnsAfter("### Calendars of other bodies"));
});

test("the time codes and clock readings read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.ccsdsDecode], columnsAfter("### CCSDS time codes"));
  assert.deepEqual([...COLUMNS.ccsdsAscii], columnsOfTableStarting("variation"));
  assert.match(README, /writes one line of\s+one cell, the code of an instant in the format a P-field names/);
  assert.deepEqual([...COLUMNS.radioDecode], columnsAfter("### Radio time codes"));
  assert.deepEqual([...COLUMNS.jjyCallSign], columnsAfter("### JJY's call-sign frames"));
  assert.match(README, /writes one line\s+of two cells, the whole seconds from 1970-01-01 00:00 and\s+the attoseconds/);
  assert.deepEqual([...COLUMNS.sixHourClock], columnsAfter("### Six-hour clocks"));
});

test("the parts of a day, the cycles and the fasts read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.kalam], columnsAfter("### Rāhu kālam, Yamaganda and Gulika kālam"));
  assert.deepEqual([...COLUMNS.almanacCycles], columnsAfter("### The almanac's cycles"));
  assert.deepEqual([...COLUMNS.almanacDay], columnsAfter("### The almanac's day"));
  assert.deepEqual([...COLUMNS.orthodoxFast], columnsAfter("### The Orthodox fasts"));
  // The seasons' line is stated in prose: columns 3 to 5 above, then the two days.
  assert.match(README, /the identifier, the English name and the\s+kind of columns 3 to 5 above, then the first and last days/);
  assert.deepEqual([...COLUMNS.orthodoxFastSeasons], ["id", "english name", "kind", "first", "last"]);
});

test("the prayer times, the zmanim and the Edo hours read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.prayerTimes], columnsAfter("### Prayer times"));
  // The methods' table is the second after the heading.
  const methods = README.slice(README.indexOf("`hc_prayer_methods` writes one line a\nmethod:"));
  const methodRows = methods.split("\n").filter((line) => /^\| \d+ \| /.test(line));
  const methodColumns = [];
  for (const row of methodRows) {
    const cells = row.slice(1, -1).split(" | ").map((cell) => cell.trim());
    if (cells[0] !== String(methodColumns.length + 1)) {
      break;
    }
    methodColumns.push(cells[1]);
  }
  assert.deepEqual([...COLUMNS.prayerMethods], methodColumns);
  assert.deepEqual([...COLUMNS.zmanim], columnsAfter("### Zmanim"));
  assert.deepEqual([...COLUMNS.temporalHour], columnsAfter("### The temporal hour"));
  assert.deepEqual([...COLUMNS.edoTime], columnsAfter("### The Edo hours"));
  assert.match(README, /give one line in the shape of `hc_solar_event`'s/);
});

test("the new reckonings read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.choghadiya], columnsAfter("### Choghadiya"));
  assert.deepEqual([...COLUMNS.panchak], columnsAfter("### Panchak"));
  assert.deepEqual([...COLUMNS.kumbh], columnsAfter("### The Kumbh Mela"));
  assert.deepEqual([...COLUMNS.pushkaram], columnsAfter("### Pushkaram"));
  assert.deepEqual([...COLUMNS.barhaspatyaYear], columnsAfter("### The northern year's name"));
  // The name at an instant is stated in prose: columns 1, 2 and 5 above,
  // and four more.
  assert.match(README, /writes one line of three\s+cells, columns 1, 2 and 5 above, and four more/);
  const year = COLUMNS.barhaspatyaYear;
  assert.deepEqual([...COLUMNS.barhaspatyaYearAt].slice(0, 3), [year[0], year[1], year[4]]);
  assert.equal(COLUMNS.barhaspatyaYearAt.length, 7);
  assert.deepEqual([...COLUMNS.nakshatra], columnsAfter("### The nakṣatra"));
  assert.deepEqual([...COLUMNS.muhurtas], columnsAfter("### The muhūrtas"));
  assert.deepEqual([...COLUMNS.amritaSiddhi], columnsAfter("### Amṛta siddhi"));
  assert.deepEqual([...COLUMNS.drekkana], columnsAfter("### Drekkāṇas"));
  assert.deepEqual([...COLUMNS.folkDay], columnsAfter("### Folk days"));
  assert.deepEqual([...COLUMNS.nightWatch], columnsAfter("### The night watches"));
  assert.deepEqual([...COLUMNS.planetaryHour], columnsAfter("### Planetary hours"));
  assert.deepEqual([...COLUMNS.gmat], columnsAfter("## Greenwich Mean Astronomical Time"));
  assert.deepEqual([...COLUMNS.irigDecode], columnsAfter("### IRIG time codes"));
  // The formats' table is the second after the heading.
  const formats = README.slice(README.indexOf("`hc_irig_formats(buffer, capacity)` lists"));
  const formatColumns = [];
  for (const row of formats.split("\n").filter((line) => /^\| \d+ \| /.test(line))) {
    const cells = row.slice(1, -1).split(" | ").map((cell) => cell.trim());
    if (cells[0] !== String(formatColumns.length + 1)) {
      break;
    }
    formatColumns.push(cells[1]);
  }
  assert.deepEqual([...COLUMNS.irigFormats], formatColumns);
  assert.match(README, /It\s+writes one line of one cell, the frame\./);
});

test("the place names read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.places], columnsAfter("## Place names"));
});

test("the humanized times read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.equinoxMargin], columnsAfter("### Equinox new-year margins"));
  assert.deepEqual([...COLUMNS.irigFrameStart], columnsAfter("### The start of an IRIG frame"));
  assert.deepEqual([...COLUMNS.dayPeriod], columnsAfter("### Day periods"));
  assert.deepEqual([...COLUMNS.numberingSystems], columnsAfter("### Numbering systems"));
  assert.deepEqual([...COLUMNS.calendarEras], columnsAfter("### A calendar's eras"));
  assert.deepEqual([...COLUMNS.holidayGroups], columnsAfter("### Groups and names in a locale"));
  assert.deepEqual([...COLUMNS.zoneName], columnsAfter("### A zone's name"));
  assert.deepEqual([...COLUMNS.roman1960Office], columnsAfter("### The 1960 office"));
  assert.deepEqual([...COLUMNS.frenchDecimalTime], columnsAfter("### French Republican decimal time"));
  assert.deepEqual([...COLUMNS.babylonianRegnalYear], columnsAfter("### Babylonian regnal years"));
  assert.deepEqual([...COLUMNS.shmuelTekufah], columnsAfter("### Shmuel's tekufot"));
  assert.deepEqual([...COLUMNS.dayName], columnsAfter("### Named days"));
  assert.deepEqual([...COLUMNS.chineseAlmanacSolarTerms], columnsAfter("### The Qing almanac's solar terms"));
  assert.deepEqual([...COLUMNS.tibetanAlmanacDay], columnsAfter("### The Tibetan almanac"));
  assert.deepEqual([...COLUMNS.tibetanPlanets], columnsAfter("### The Tibetan planets"));
  assert.deepEqual([...COLUMNS.bhutaneseWinterSolstice], columnsAfter("### The Bhutanese winter solstice"));
  assert.deepEqual([...COLUMNS.almanacDirections], columnsAfter("### The year's directions"));
  assert.deepEqual([...COLUMNS.mansionUndertakings], columnsAfter("### The undertakings of each 二十八宿"));
  assert.deepEqual([...COLUMNS.almanacPersonDays], columnsAfter("### A person's own days"));
  assert.deepEqual([...COLUMNS.relativeTime], columnsAfter("### Relative time"));
  assert.deepEqual([...COLUMNS.relativeDayAt], columnsAfter("### A relative day at a time"));
  assert.deepEqual([...COLUMNS.duration], columnsAfter("### Durations"));
});

test("the relativity lines read the README's columns in order", () => {
  assert.deepEqual([...COLUMNS.properTime], columnsAfter("## Relativity"));
  assert.deepEqual([...COLUMNS.gravitationalDilation], columnsAfter("### A clock at a radius"));
  assert.deepEqual([...COLUMNS.gravitatingBodies], columnsAfter("### The gravitating bodies"));
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
