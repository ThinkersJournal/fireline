kind: KISS 0.24.0 corpus re-vendored (34 dtype tokens)
---
Re-vendored `kiss_dtype_manifest.json` (27 -> 34 rows: f128 i128 u128 i256 u256 f8e3m4 f8ue4m3) and `kiss_opattrs_field_types.json` from KISS `conformance/corpus` at origin/main e628c96 (spec text a9ad8cf; unpublished). `op_facts`, `op_decomposition` and `op_identity_vectors` are byte-identical (LF-normalised) and untouched. No fireline source carries a closed dtype list: `fireline-contract` reads widths from the manifest, so it needed no code change; the one test that pinned `len() == 27` now pins 34 and asserts the seven new widths. `cargo test --workspace` passes (47 result lines ok, 0 failed).

Doc-only: `fireline-classify`'s `MxElement::from_token` comment now counts "the other 31 of the 34" manifest tokens (was 24 of 27); no code or quantisation behaviour changed (KISS lane OK'd, 2026-11-26).
