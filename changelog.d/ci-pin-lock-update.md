kind: CI pin (CodeRipper 0.48.1) and lock-only dependency update
---
Pinned `portfolio.yml` in `ci.yml` to coderipper@467fd259 (installs CodeRipper 0.48.1). Lock-only update clearing SUP-015: toml 1.2.2, toml_parser 1.1.6, unpopped-vocab 0.56.12, hashbrown 0.17.2, zerocopy and zerocopy-derive 0.8.65. Not breaking; no source change. Verified locally: 359 tests passed in 47 bins, clippy -D warnings clean, fmt clean; `cargo tree -d` shows only syn 2.0.119 vs 3.0.10.
