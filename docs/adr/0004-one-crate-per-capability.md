# 0004 — One crate per capability, behind a feature-gated facade

**Status:** Accepted

## Context

Requirement 8 of the brief: bundling everything together would be wasteful, so
it must be possible to compile in only what is needed. A lunar ephemeris,
twenty-odd locales' worth of name data and a holiday table for forty-odd
countries have no business in a WebAssembly bundle that formats Gregorian
dates.

## Decision

One crate per capability, in one workspace. The `hyper-calendar` facade
re-exports them as optional features, named after how people ask for things
(`civil`, `lunar`, `seasons`, `holiday`, `deep-time`, `planetary`,
`relativity`, `units`, `full`). `default` is deliberately modest.

The count is deliberately not written here. Earlier versions of this ADR said
"nineteen" in two places and were wrong in both within a month, as were three
other documents, each with a different number. The current list is generated
into [`supported.md`](../supported.md) from the manifest.

## Consequences

**Good.** Cargo genuinely drops unused code, rather than relying on the linker.
The `cargo audit` surface per consumer is minimal. Each crate has its own
README stating its accuracy claims, which is easier to keep honest than one
enormous document — though five crates still have none, which is a debt
against this claim rather than evidence for it. The dependency graph is forced to stay a DAG, which catches
design mistakes early.

**Costs.** A `Cargo.toml` per crate and a feature matrix that has to be kept
consistent — and which drifted: four features once compiled a crate in and
re-exported nothing, which `crates/hyper-calendar/tests/facade.rs` now
prevents. Cross-crate refactoring is more work than moving a module. Feature
unification means a consumer who enables `full` pays the full compile time.

CI compensates by building `--all-features`, `--no-default-features --features
alloc`, and the WebAssembly and cdylib targets on every pull request, so a
broken feature combination is caught immediately.

## Alternatives rejected

- **One crate with `#[cfg(feature)]` modules.** Simpler to navigate, but
  feature unification across a dependency tree becomes all-or-nothing much
  faster, and the crate's README has to claim accuracy for everything at once.
