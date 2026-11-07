# fireline-announce

The kernel-seam **handshake** between a kernel *consumer* (an ML orchestrator such as Fuel) and a kernel *provider* (a backend such as
baracuda): the frozen 56-byte `SeamHello` envelope, the capability bits, and role-aware profile negotiation. KISS-Announce is the standard;
this is the Rust implementation both sides compile, so neither can drift from the other.

- **Dependency-free.** No region grammar, no graph types.
- **Require it with a tilde** (`fireline-announce = "~0.1"`) or an exact version, never a caret. `tools/tilde-lint` fails a caret.
- **Hello world:** `cargo run -p fireline-announce --example hello_world`, or the doctest on the crate root.

## Provenance

Merged from the two earlier copies, both by the same owner and both `MIT OR Apache-2.0`:
`fuel-kernel-seam-announce` (Fuel `d010189e`: envelope, role-aware `negotiate_with_role`, axis masks) and `baracuda-seam`
(baracuda `8bf6705b`: wire bytes and the truncation error). The three test vectors in `tests/conformance.rs` are copied from KISS
`conformance/corpus/announce_vectors.json` at KISS `6780c09` (`MIT OR Apache-2.0`), which stays the source of truth.

Names: `DLPACK_EXT` and `PROVISION_ON_REQUEST` are KISS Announce's spelling (their proposal, 2026-11-07); Fuel's earlier `FDX` and baracuda's
earlier `JIT_ON_REQUEST` are retired. Bits 34 (`KISC_FRAMING`) and 35 (`RECIPE_IMPORT`) are **not allocated** and live in no shared crate
until KISS Announce co-allocates them with a consumer on each side.
