# 0005 — No external dependencies

**Status:** Accepted

## Context

A date library is a transitive dependency of almost everything. Whatever it
pulls in, everyone pulls in.

## Decision

The workspace has exactly one optional external dependency: `libm`, for
floating-point math on `no_std` targets that lack it. Parsing, formatting,
locale data, plural rules, astronomy, the TZif reader and the WebAssembly
surface are all implemented here.

The WebAssembly crate exports a raw C-style ABI rather than using
`wasm-bindgen`, so the artefact carries no glue it did not ask for.

## Consequences

**Good.** A tiny WebAssembly artefact. A near-empty `cargo audit` surface. No
dependency can break a build, change a licence or introduce a breaking change
underneath us. The `no_std` story is real rather than aspirational.

**Costs.** Real work that a crate already does well — `serde` derives, a
CLDR-generated locale database, a full ephemeris — is written by hand here, and
is correspondingly smaller in scope. Serialisation is offered as plain
conversion functions rather than `serde` impls. The accuracy claims of the
astronomical code are those of a truncated series, not of JPL DE440.

Each of those limitations is stated in the relevant crate's README rather than
being discovered by a user.

## Revisit when

If a consumer needs `serde` support, the right answer is an optional
`serde` feature that is off by default — not a required dependency.
