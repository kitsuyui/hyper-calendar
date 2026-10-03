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

/**
 * Every parameter list the declarations write that puts a required
 * parameter after an optional one: TypeScript's error 1016, which a consumer
 * who compiles the declarations without `skipLibCheck` meets. A list is the
 * text between a `(` and its `)` whose top-level, comma-separated pieces are
 * all `name: type` or `name?: type`, so that a parenthesised type is not
 * read as one; a rest parameter ends the check.
 *
 * `tsc` is not run: it is not a dependency of this repository, and nothing
 * here may fetch one. This is the rule it applies, held as text.
 *
 * @param {string} text
 * @returns {string[]} the offending parameter lists
 */
export function requiredAfterOptional(text) {
  /** How the character at `at` changes the depth: brackets open and close it, `=>` does not. */
  const nesting = (source, at) => {
    const character = source[at];
    if ("([{<".includes(character)) return 1;
    if (character === ">" && source[at - 1] === "=") return 0;
    return ")]}>".includes(character) ? -1 : 0;
  };
  const bad = [];
  const parameter = /^\s*(\.\.\.)?[A-Za-z_$][\w$]*(\?)?\s*:/;
  for (let open = text.indexOf("("); open >= 0; open = text.indexOf("(", open + 1)) {
    let close = -1;
    let depth = 0;
    for (let at = open; at < text.length; at += 1) {
      depth += nesting(text, at);
      if (depth === 0) {
        close = at;
        break;
      }
    }
    if (close < 0) continue;
    const inside = text.slice(open + 1, close);
    const pieces = [];
    let piece = "";
    depth = 0;
    for (let at = 0; at < inside.length; at += 1) {
      depth += nesting(inside, at);
      if (inside[at] === "," && depth === 0) {
        pieces.push(piece);
        piece = "";
      } else {
        piece += inside[at];
      }
    }
    pieces.push(piece);
    const listed = pieces.filter((each) => each.trim().length > 0);
    if (listed.length === 0 || !listed.every((each) => parameter.test(each))) continue;
    let optional = false;
    for (const each of listed) {
      const found = parameter.exec(each);
      if (found[1]) break;
      if (found[2]) optional = true;
      else if (optional) {
        bad.push(`(${text.slice(open + 1, close).trim()})`);
        break;
      }
    }
  }
  return bad;
}

test("the declarations put no required parameter after an optional one", () => {
  assert.deepEqual(requiredAfterOptional(TYPES), []);
  assert.deepEqual(requiredAfterOptional(EMBEDDED), []);
});

test("the check finds the error it holds the declarations to", () => {
  assert.deepEqual(requiredAfterOptional("f(country: string, kind?: string, fixed: number | bigint): void;"), [
    "(country: string, kind?: string, fixed: number | bigint)",
  ]);
  assert.deepEqual(requiredAfterOptional("f(a?: string, ...rest: number[]): void; g(a: (string | number)[], b?: (x: number) => void): void;"), []);
  assert.deepEqual(requiredAfterOptional("f(a: string, b?: string, c?: number): void;"), []);
  assert.equal(requiredAfterOptional("g(callback: (x?: number, y: string) => void): void;").length, 1);
  assert.equal(requiredAfterOptional("g(a?: (x: number) => void, b: string): void;").length, 1);
});
