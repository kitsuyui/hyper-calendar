// Where the tests find the built module, and the helpers they share.
//
// `scripts/wasm-js-test.sh` builds the module and runs the tests; run by
// hand, the paths below are where `cargo build` leaves the files, with
// CARGO_TARGET_DIR honoured, and HC_WASM and HC_WASM_CIVIL override them.

import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

/** The repository root. */
export const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "..", "..", "..");

/** Where Cargo puts what it builds. */
export const TARGET = process.env.CARGO_TARGET_DIR ?? resolve(ROOT, "target");

/** The module built with `--release --features full`. */
export const FULL_WASM =
  process.env.HC_WASM ?? resolve(TARGET, "wasm32-unknown-unknown", "release", "hyper_calendar_wasm.wasm");

/** The module built with `--release --no-default-features --features civil`. */
export const CIVIL_WASM =
  process.env.HC_WASM_CIVIL ?? resolve(TARGET, "wasm-js", "hyper_calendar_wasm.civil.wasm");

/**
 * The bytes of a built module, or a failure that says how to build it.
 *
 * @param {string} path
 * @param {string} how
 * @returns {Uint8Array}
 */
export function moduleBytes(path, how) {
  if (!existsSync(path)) {
    throw new Error(`${path} is not built; run ${how}, or scripts/wasm-js-test.sh for everything`);
  }
  return new Uint8Array(readFileSync(path));
}

/**
 * The text an export writes, read raw with no decoding, so a test can hold
 * the module's cell count to the README's.
 *
 * @param {import("./hyper-calendar.js").HyperCalendar} hc
 * @param {(buffer: number, capacity: number) => bigint} call
 * @returns {string[][]}
 */
export function rawRows(hc, call) {
  const needed = Number(call(0, 0));
  if (needed < 0) {
    throw new Error(`measured ${needed}`);
  }
  const pointer = hc.alloc(needed);
  try {
    const written = Number(call(pointer, needed));
    if (written !== needed) {
      throw new Error(`measured ${needed}, wrote ${written}`);
    }
    const text = new TextDecoder().decode(new Uint8Array(hc.memory.buffer, pointer, written));
    return text.split("\n").filter((line) => line.length > 0).map((line) => line.split("\t"));
  } finally {
    hc.free(pointer, needed);
  }
}

/**
 * A TZif version 2 file for a zone on Eastern Time with four transitions
 * across 2024 and 2025, the one the crate's own tests load.
 */
export const TZIF_V2_EASTERN = Uint8Array.from(
  (
    "545a6966320000000000000000000000000000000000000200000002000000010000000400000002" +
    "0000000865ed5a706727116067cd3c706906f36001000100ffffb9b00000ffffc7c0010445535400" +
    "45445400586846800000001b00000000545a69663200000000000000000000000000000000000002" +
    "00000002000000010000000400000002000000080000000065ed5a70000000006727116000000000" +
    "67cd3c70000000006906f36001000100ffffb9b00000ffffc7c00104455354004544540000000000" +
    "586846800000001b000000000a455354354544542c4d332e322e302c4d31312e312e300a"
  ).match(/../g) ?? [],
  (byte) => Number.parseInt(byte, 16),
);
