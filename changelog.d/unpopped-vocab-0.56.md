kind: **BREAKING**: fireline-synth requires unpopped-vocab ~0.56 (was ~0.47)
---
`fireline-synth` now depends on `unpopped-vocab` `~0.56.0` (resolved 0.56.5). No source change was needed: the workspace builds and all tests pass. `fireline-grammar` has no direct `unpopped-vocab` dependency. This unblocks consumers that hold one copy of the vocab crate (baracuda, fuel).
