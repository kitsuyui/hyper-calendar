# 0004 — One crate per capability, behind a feature-gated facade

**Status:** Accepted

## Context

Requirement 8 of the brief: bundling everything together would be wasteful, so
it must be possible to compile in only what is needed. A lunar ephemeris, 30
locales' worth of name data and a holiday table for 50 countries have no
business in a WebAssembly bundle that formats Gregorian dates.

## Decision

Nineteen crates in one workspace, split by capability. The `hyper-calendar`
facade re-exports them as optional features, with bundles named after how
people ask for things (`civil`, `lunar`, `seasons`, `holiday`, `deep-time`,
`planetary`, `relativity`, `full`). `default` is deliberately modest.

## Consequences

**Good.** Cargo genuinely drops unused code, rather than relying on the linker.
The `cargo audit` surface per consumer is minimal. Each crate has its own
README stating its accuracy claims, which is easier to keep honest than one
enormous document. The dependency graph is forced to stay a DAG, which catches
design mistakes early.

**Costs.** Nineteen `Cargo.toml` files and a feature matrix that has to be kept
consistent. Cross-crate refactoring is more work than moving a module. Feature
unification means a consumer who enables `full` pays the full compile time.

CI compensates by building `--all-features`, `--no-default-features --features
alloc`, and the WebAssembly and cdylib targets on every pull request, so a
broken feature combination is caught immediately.

## Alternatives rejected

- **One crate with `#[cfg(feature)]` modules.** Simpler to navigate, but
  feature unification across a dependency tree becomes all-or-nothing much
  faster, and the crate's README has to claim accuracy for everything at once.
