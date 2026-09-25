// Type declarations for hyper-calendar.embedded.js, the binding with the
// module's bytes inside it, which scripts/wasm-embed.mjs writes. Everything
// hyper-calendar.js exports is here too, except that `load` takes no source.

import type { HyperCalendar, LoadOptions } from "./hyper-calendar.js";

export {
  COLUMNS,
  GEOLOGIC_RANKS,
  HcError,
  HyperCalendar,
  METHODS,
  SENTINELS,
} from "./hyper-calendar.js";
export type * from "./hyper-calendar.js";

/** Where the embedded bytes came from. */
export const EMBEDDED: {
  readonly file: string;
  readonly bytes: number;
  readonly sha256: string;
  /** When the file was written, as an ISO 8601 instant. */
  readonly generated: string;
};

/** The module's bytes, decoded without a fetch. */
export function wasmBytes(): Uint8Array;

/** Instantiate the embedded module and bind it. */
export function load(options?: LoadOptions): Promise<HyperCalendar>;
