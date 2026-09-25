// The embedded module scripts/wasm-embed.mjs writes: generated here from
// the full build into a scratch directory, imported, and asked to answer
// without a fetch, as a page opened from disk would ask it.

import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { after, before, test } from "node:test";

import { HyperCalendar, METHODS, load } from "./hyper-calendar.js";
import { FULL_WASM, ROOT, moduleBytes } from "./support.js";

const HOW = "cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown --release --features full";

/** @type {string} */
let scratch;
/** @type {string} */
let written;
/** @type {any} */
let embedded;
/** @type {string} */
let report;

before(async () => {
  moduleBytes(FULL_WASM, HOW);
  scratch = mkdtempSync(resolve(tmpdir(), "hyper-calendar-embedded-"));
  written = resolve(scratch, "hyper-calendar.embedded.js");
  report = execFileSync(
    process.execPath,
    [resolve(ROOT, "scripts", "wasm-embed.mjs"), "--wasm", FULL_WASM, "--out", written],
    { encoding: "utf8" },
  );
  // Fetch is taken away: the module has to come from the file itself.
  const fetched = globalThis.fetch;
  globalThis.fetch = () => {
    throw new Error("a page opened from disk cannot fetch");
  };
  try {
    embedded = await import(pathToFileURL(written).href);
  } finally {
    globalThis.fetch = fetched;
  }
});

after(() => {
  if (scratch !== undefined) {
    rmSync(scratch, { recursive: true, force: true });
  }
});

test("the generator writes the module, its types and a report", () => {
  assert.ok(existsSync(written));
  assert.ok(existsSync(resolve(scratch, "hyper-calendar.d.ts")));
  assert.ok(existsSync(resolve(scratch, "hyper-calendar.embedded.d.ts")));
  assert.match(report, /hyper-calendar\.embedded\.js: \d+ bytes/);
  assert.doesNotMatch(readFileSync(written, "utf8"), /^import\b/m, "self-contained");
});

test("the bytes are the module's, byte for byte", () => {
  const original = readFileSync(FULL_WASM);
  const bytes = embedded.wasmBytes();
  assert.ok(bytes instanceof Uint8Array);
  assert.equal(bytes.length, original.length);
  assert.ok(Buffer.from(bytes).equals(original));
  assert.equal(embedded.EMBEDDED.bytes, original.length);
  assert.equal(embedded.EMBEDDED.sha256, createHash("sha256").update(original).digest("hex"));
  assert.match(embedded.EMBEDDED.generated, /^\d{4}-\d{2}-\d{2}T/);
});

test("load() takes no source and answers as the fetched module does", async () => {
  const hc = await embedded.load();
  assert.ok(hc instanceof embedded.HyperCalendar);
  const fetched = await load(readFileSync(FULL_WASM));
  assert.equal(hc.version(), fetched.version());
  assert.deepEqual(hc.layers(), fetched.layers());
  assert.equal(hc.gregorianToFixed(2026, 9, 21), 739_880);
  assert.deepEqual(hc.describeDay(739_880, "ja-JP"), fetched.describeDay(739_880, "ja-JP"));
  const small = await embedded.load({ initialCapacity: 1 });
  assert.equal(small.formatIsoDate(739_880), "2026-09-21");
  // The classes are the embedded file's own copies, not this module's.
  assert.notEqual(embedded.HyperCalendar, HyperCalendar);
  assert.equal(typeof embedded.HcError, "function");
  assert.equal(embedded.METHODS.length, METHODS.length);
});
