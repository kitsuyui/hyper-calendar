// The hand-written type declarations cannot drift from the binding.
//
// `hyper-calendar.d.ts` types `hyper-calendar.js`, and
// `hyper-calendar.embedded.d.ts` says it carries everything the binding
// exports. This file reads both declarations as text and holds them to the
// module's runtime exports: every value export is declared, every key of
// `COLUMNS` is typed, every method in `METHODS` is declared on
// `HyperCalendar`, and the embedded declarations re-export all of it.

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { test } from "node:test";

import * as binding from "./hyper-calendar.js";
import { ROOT } from "./support.js";

const JS = resolve(ROOT, "crates", "hyper-calendar-wasm", "js");
const TYPES = readFileSync(resolve(JS, "hyper-calendar.d.ts"), "utf8");
const EMBEDDED = readFileSync(resolve(JS, "hyper-calendar.embedded.d.ts"), "utf8");

/**
 * The value names a declaration file exports directly: its `export const`,
 * `export class` and `export function` declarations.
 *
 * @param {string} text
 * @returns {string[]}
 */
function declaredValues(text) {
  return [...text.matchAll(/^export (?:const|class|function|async function) ([A-Za-z0-9_]+)/gm)].map(
    (match) => match[1],
  );
}

/**
 * The names re-exported by an `export { … } from` block.
 *
 * @param {string} text
 * @returns {string[]}
 */
function reexported(text) {
  return [...text.matchAll(/^export \{([^}]*)\} from/gm)].flatMap((match) =>
    match[1]
      .split(",")
      .map((name) => name.trim())
      .filter((name) => name.length > 0),
  );
}

/**
 * The keys of the object type declared for `name`: the `readonly key:`
 * lines between `export const NAME: {` and its closing `};`.
 *
 * @param {string} text
 * @param {string} name
 * @returns {string[]}
 */
function declaredKeys(text, name) {
  const start = text.indexOf(`export const ${name}: {`);
  assert.ok(start >= 0, `no declaration of ${name}`);
  const end = text.indexOf("\n};", start);
  return [...text.slice(start, end).matchAll(/^\s+readonly ([A-Za-z0-9_]+):/gm)].map((match) => match[1]);
}

/**
 * The method names declared in `export class NAME { … }`.
 *
 * @param {string} text
 * @param {string} name
 * @returns {Set<string>}
 */
function declaredMethods(text, name) {
  const start = text.indexOf(`export class ${name} {`);
  assert.ok(start >= 0, `no declaration of class ${name}`);
  const end = text.indexOf("\n}", start);
  return new Set([...text.slice(start, end).matchAll(/^ {2}([A-Za-z0-9_]+)\(/gm)].map((match) => match[1]));
}

test("every value the binding exports is declared, and nothing else", () => {
  assert.deepEqual(declaredValues(TYPES).sort(), Object.keys(binding).sort());
});

test("every COLUMNS key is typed, in the binding's order", () => {
  assert.deepEqual(declaredKeys(TYPES, "COLUMNS"), Object.keys(binding.COLUMNS));
});

test("every method in METHODS is declared on HyperCalendar", () => {
  const declared = declaredMethods(TYPES, "HyperCalendar");
  for (const { method } of binding.METHODS) {
    assert.ok(declared.has(method), `HyperCalendar.${method} is not declared`);
  }
});

test("the embedded declarations carry everything the binding exports", () => {
  const embedded = new Set([...reexported(EMBEDDED), ...declaredValues(EMBEDDED)]);
  for (const name of Object.keys(binding)) {
    assert.ok(embedded.has(name), `hyper-calendar.embedded.d.ts lacks ${name}`);
  }
});
