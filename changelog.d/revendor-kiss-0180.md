kind: patch
---
Re-vendored the Appendix C golden document from KISS `conformance/corpus/contract_vectors.json` at `11407f72` (KISS 0.18.0, sk8 per-operand dtype key: now `len=1304 crc32=2854c69d`, 1349 bytes, was 1325). The 13 decline vectors are byte-identical to the corpus. `cargo test -p fireline-contract --locked` passes. No code change.
