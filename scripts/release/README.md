# Publishing crates

The `Release crates` workflow starts with a dry-run when dispatched with an existing tag matching the checked-out commit and all publishable package versions. It prints the dependency order and runs workspace tests and Cargo's workspace publication checks. Crates whose manifests disable publication or target another registry are excluded. A publishable crate cannot depend on an excluded workspace crate.

A maintainer can select `publish` only after reviewing the dry-run and configuring the `CARGO_REGISTRY_TOKEN` Actions secret with permission for every crate name. The workflow repeats all checks before uploading. Crates.io uploads are irreversible and a partial failure may leave some versions published; inspect the registry before retrying. Workspace dry-runs may require prior versions of internal dependencies to be present in the registry. Do not bypass a failed dry-run; resolve the registry prerequisites first.
