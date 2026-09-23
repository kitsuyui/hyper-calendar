# 0004 — One crate per capability, behind a feature-gated facade

**Status:** Accepted

## Context

Requirement 8 of the brief: bundling everything together would be wasteful, so
it must be possible to compile in only what is needed. A lunar ephemeris,
twenty-odd locales' worth of name data and holiday tables for most of the
world's countries have no business in a WebAssembly bundle that formats Gregorian
dates.

## Decision

One crate per capability, in one workspace. The `hyper-calendar` facade
re-exports them as optional features, named after how people ask for things
(`civil`, `lunar`, `seasons`, `holiday`, `deep-time`, `planetary`,
`relativity`, `units`, `full`). `default` is deliberately modest.

The count is deliberately not written here: a hand-written count goes stale
as soon as a crate is added. The current list is generated into
[`supported.md`](../supported.md) from the manifest.

## Consequences

**Good.** Cargo genuinely drops unused code, rather than relying on the linker.
The `cargo audit` surface per consumer is minimal. Each crate has its own
README stating its accuracy claims, which is easier to keep honest than one
enormous document. The two crates whose readers cannot be assumed to read
Rust — the C ABI and the WebAssembly surface — have their export tables
rendered from the source by `crates/hyper-calendar/tests/abi.rs`, for the
same reason the crate list is generated. The dependency graph is forced to
stay a DAG, which catches design mistakes early.

**Costs.** A `Cargo.toml` per crate and a feature matrix that has to be kept
consistent. A feature that compiles a crate in and re-exports nothing is the
easy mistake, and `crates/hyper-calendar/tests/facade.rs` rejects it. Cross-crate refactoring is more work than moving a module. Feature
unification means a consumer who enables `full` pays the full compile time.

CI compensates by building `--all-features`, `--no-default-features --features
alloc,libm`, and the WebAssembly and cdylib targets on every pull request, so a
broken feature combination is caught immediately.

## Alternatives rejected

- **One crate with `#[cfg(feature)]` modules.** Simpler to navigate, but
  feature unification across a dependency tree becomes all-or-nothing much
  faster, and the crate's README has to claim accuracy for everything at once.
