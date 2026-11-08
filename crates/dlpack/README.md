# fireline-dlpack

**FDX, the DLPack extension**: the C-ABI structs, codes, sidecar and validators for tensor interchange between a kernel *consumer* (an
orchestrator such as Fuel) and a kernel *provider* (a backend such as baracuda or Vulkane), plus the co-maintained C header
(`include/fuel_dlpack_ext.h`). One definition, so neither side can drift.

- **Require it with a tilde** (`fireline-dlpack = "~0.1"`) or an exact version, never a caret. `tools/tilde-lint` fails a caret.
- **It implements FDX only.** KISS's neutral ten-field quantisation sidecar (KISS-CLASSIFY-6.3-0012) is a later projection once KISS pins its
  wire form; this crate must not be read as that final interface.
- What is **not** here: the conversion from a consumer's own dtype, shape and storage types (it names the consumer's types).
- **The FDX spec** is not in this crate yet. Its home is an open ruling (plan section 12): Fuel and this lane favour moving it here with the
  code, KISS keeps that as its working assumption. Until then the validator list (spec section 8, V1 to V21) is not joined to the spec by a test
  in this repository.

## Provenance

Moved from `fuel-ir/src/dlpack/` and `fuel-ir/include/` (Fuel `origin/main` `078e578a3`, `MIT OR Apache-2.0`, every source file carries our SPDX
header; Fuel's provenance record lists these files as `own`, no lineage, 0 to 5 percent verbatim overlap with candle). `codes.rs`, `validate.rs`
and the C header are verbatim. `abi.rs` and `sidecar.rs` are Fuel's files minus their inline `mod tests`, which were **relocated** to `tests/`
(public items only). `validate/tests.rs` differs only in `crate::dlpack::` becoming `crate::` and one stale comment (it said the tests run under Fuel's `--features dlpack`). `header_check.rs` differs only in the relative path
to the header. Fuel's `convert.rs` is **not** moved.
