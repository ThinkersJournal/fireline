kind: `fireline-dlpack` clippy 1.100 `needless_range_loop` fix
---
Rust 1.100's clippy rejects three index loops over `a.terms` in `crates/dlpack/src/validate.rs` (the affine-term checks of the extent validator and the V17 evaluation), which failed `cargo clippy -D warnings` on every PR's CI from 2026-11-12. The loops now iterate the slices (`a.terms[..tc]`, `a.terms[tc..FDX_AFFINE_MAX_TERMS]`, `a.terms[..term_count]`), which panic on exactly the inputs the indexing did and preserve the order of checks and errors. No behaviour change. Verified with `cargo clippy --workspace --all-targets --locked -- -D warnings` and `cargo test -p fireline-dlpack --locked` (96 unit tests pass) on rustc 1.100.0.

— lane: fireline
