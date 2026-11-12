# fireline-contract

KISS-Contract **bind-time obligations**: parse a Dispatch expression (KISS-CONTRACT-6.6-0006) once, then *bind* it against a launch
environment (6.12-0001) with typed declines (`unbound_symbol`, `extent_out_of_range`, `affine_overflow`, and the 6.6-0009 division
rules). A malformed expression is a build-time fault, never a bind-time decline (6.12-0002).

- **Draft-tracking.** KISS-Contract 6.12 is a Draft (freeze gate unmet; CireSnave 2026-11-12: "working toward the draft version is the best
  we have"). Expect rework when it moves. Every rule cites its clause id; decline codes the Draft does not name are marked in the docs.
- **First slice only.** Covers 6.12-0001..-0006 and 6.6-0006/-0009. Not yet here: the wire frames (`SAVL`, `CYRQ`, `CRSP`, `CDEC`), the
  mapped FDX rows V1/V4/V10/V11, and anything for V3/V5/V6/V8/V9/V12/V13/V18..V21 (no clause or the Fuel window formula is pending).
- **Dependency-free.**
- **Require it with a tilde** (`fireline-contract = "~0.1"`) or an exact version, never a caret. `tools/tilde-lint` fails a caret.
- **Hello world:** `cargo run -p fireline-contract --example contract_hello_world`, or the doctest on the crate root.

## Provenance

Written from the Draft text (KISS `spec/contract.md` at `origin/main` `2d7f20e`). The test cases mirror the shapes in KISS
`conformance/tests/contract_bind_time.rs` (same owner, `MIT OR Apache-2.0`), which stays the source of truth. No third-party code.
