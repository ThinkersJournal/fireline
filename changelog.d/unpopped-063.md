kind: **BREAKING**: unpopped-vocab ~0.63 in fireline-synth
---
`fireline-synth` now takes `unpopped-vocab ~0.63` (0.63.3 is published; 0.63.0-0.63.3 are breaking: `Numerics` gains a spelling field, so a struct literal without `..Numerics::default()` stops compiling). FireLine builds no `Numerics` literal (`git grep Numerics` over `*.rs` at origin/main finds none; the same grep style finds `unpopped` in 3 files). Verified with `cargo check` and `cargo test`.
