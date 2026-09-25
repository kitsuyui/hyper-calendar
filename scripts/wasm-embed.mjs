#!/usr/bin/env node
// Write hyper-calendar.embedded.js: the JavaScript binding with the module's
// bytes inside it, for a page that cannot fetch a .wasm file — one opened
// from disk, where `fetch` of a file: URL is refused.
//
// The output is one self-contained ES module. It is the binding's source
// with its `load(source)` renamed away, the module's bytes as base64, and a
// `load(options)` that decodes them with `atob` and binds them; no fetch,
// no import, no dependency. The bytes are not committed: this script
// writes them, and CI uploads what it writes.
//
//     node scripts/wasm-embed.mjs [--wasm <file>] [--out <file>]
//
// The defaults are the `full` layer that scripts/wasm-layers.sh built and
// <target>/wasm-js/hyper-calendar.embedded.js, with <target> being
// CARGO_TARGET_DIR or ./target. The types are copied beside the output.

import { createHash } from "node:crypto";
import { copyFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const TARGET = process.env.CARGO_TARGET_DIR ?? resolve(ROOT, "target");
const BINDING = resolve(ROOT, "crates", "hyper-calendar-wasm", "js");

/** @type {Record<string, string>} */
const options = {};
const args = process.argv.slice(2);
for (let index = 0; index < args.length; index += 1) {
  const flag = args[index];
  const value = args[index + 1];
  if ((flag === "--wasm" || flag === "--out") && value !== undefined) {
    options[flag.slice(2)] = value;
    index += 1;
  } else {
    console.error(`usage: node scripts/wasm-embed.mjs [--wasm <file>] [--out <file>]`);
    process.exit(2);
  }
}

const wasmPath = resolve(options.wasm ?? resolve(TARGET, "wasm-layers", "hyper_calendar_wasm.full.wasm"));
const outPath = resolve(options.out ?? resolve(TARGET, "wasm-js", "hyper-calendar.embedded.js"));

const wasm = readFileSync(wasmPath);
if (wasm.subarray(0, 4).toString("latin1") !== "\0asm") {
  console.error(`${wasmPath} is not a WebAssembly module`);
  process.exit(1);
}

const source = readFileSync(resolve(BINDING, "hyper-calendar.js"), "utf8");
const marker = "export async function load(source, options = {}) {";
const occurrences = source.split(marker).length - 1;
if (occurrences !== 1) {
  console.error(`hyper-calendar.js has ${occurrences} occurrences of ${JSON.stringify(marker)}, not one`);
  process.exit(1);
}
const renamed = source.replace(marker, "async function loadFrom(source, options = {}) {");

const base64 = wasm.toString("base64");
const chunk = 4096;
const lines = [];
for (let at = 0; at < base64.length; at += chunk) {
  lines.push(`  "${base64.slice(at, at + chunk)}",`);
}
const sha256 = createHash("sha256").update(wasm).digest("hex");

const embedded = `${renamed}
// ---------------------------------------------------------------------------
// The module, embedded. Written by scripts/wasm-embed.mjs; not a file to edit.

/** Where the bytes came from. */
export const EMBEDDED = Object.freeze({
  file: ${JSON.stringify(basename(wasmPath))},
  bytes: ${wasm.length},
  sha256: ${JSON.stringify(sha256)},
  generated: ${JSON.stringify(new Date().toISOString())},
});

const WASM_BASE64 = [
${lines.join("\n")}
].join("");

/**
 * The module's bytes, decoded from the base64 above without a fetch.
 *
 * @returns {Uint8Array}
 */
export function wasmBytes() {
  if (typeof Uint8Array.fromBase64 === "function") {
    return Uint8Array.fromBase64(WASM_BASE64);
  }
  const binary = atob(WASM_BASE64);
  const bytes = new Uint8Array(binary.length);
  for (let index = 0; index < binary.length; index += 1) {
    bytes[index] = binary.charCodeAt(index);
  }
  return bytes;
}

/**
 * Instantiate the embedded module and bind it.
 *
 * @param {import("./hyper-calendar.d.ts").LoadOptions} [options]
 * @returns {Promise<HyperCalendar>}
 */
export function load(options = {}) {
  return loadFrom(wasmBytes(), options);
}
`;

mkdirSync(dirname(outPath), { recursive: true });
writeFileSync(outPath, embedded);
copyFileSync(resolve(BINDING, "hyper-calendar.d.ts"), resolve(dirname(outPath), "hyper-calendar.d.ts"));
copyFileSync(
  resolve(BINDING, "hyper-calendar.embedded.d.ts"),
  resolve(dirname(outPath), `${basename(outPath, ".js")}.d.ts`),
);

const size = Buffer.byteLength(embedded);
console.log(`${outPath}: ${size} bytes (${(size / 1048576).toFixed(2)} MiB) holding ${basename(wasmPath)}, ${wasm.length} bytes, sha256 ${sha256}`);
