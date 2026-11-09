# Changelog

## [0.0.14] - 2026-11-09

- `.coderipper.toml` declares the four interface crates (`fireline-announce`, `-grammar`, `-synth`, `-dlpack`) with `[[interface]]`, so CodeRipper's version-consistency check (per-interface mode, coderipper#278, with the 0.0.z fix in #288) no longer reports them against the project number. No crate change. The root version here is provisional.

## [0.0.13] - 2026-11-09

- Lockfile only: `syn` 3.0.9 to 3.0.10 (CodeRipper dependency-freshness). No crate change. The root version here is provisional.

## [0.0.12] - 2026-11-08

- Publish-metadata fixes (PM): `rust-version = "1.88"` on `fireline-announce`, `fireline-grammar` and `fireline-synth`, and `categories` on `fireline-synth`; the three crates go 0.1.0 to 0.1.1 (metadata only, no code change). 0.1.0 of all three is on crates.io. The root version here is provisional; the PM restamps at the gate.

## [0.0.11] - 2026-11-07

- New crate `fireline-dlpack` 0.1.0, **unpublished by decision** (`publish = false`): FDX, the DLPack extension (C-ABI structs, codes, sidecar, V-numbered validators, the co-maintained C header), moved from Fuel's `fuel-ir/src/dlpack`. It implements FDX only, not the final quantisation sidecar. Patent section added to `docs/PATENT-SWEEP.md`. The root version here is provisional; the crate versions by its own interface.

## [0.0.10] - 2026-11-07

- Publishing prep for the wave-1 crates: `publish = false` is removed from `fireline-announce`, `fireline-grammar` and `fireline-synth`, `cargo package` verifies all three together, and `docs/PUBLISHING.md` has the checked order and commands. Nothing is published; that waits on CireSnave (board 398) and the PM. The root version here is provisional.

## [0.0.9] - 2026-11-07

- New crate `fireline-synth` 0.1.0 (not published): the kernel-seam JIT-on-request envelope, the `Synthesizer` trait and the `JitRequest`, `JitBudget`, `JitResponse`, `SynthArtifact` and `LinkEntry` types, moved from Fuel's `fuel-kernel-seam`. It depends on `fireline-grammar ~0.1` and `unpopped-vocab 0.43.2`. The root version here is provisional; the crate versions by its own interface.

## [0.0.8] - 2026-11-07

- New crate `fireline-grammar` 0.1.0 (not published): the kernel-seam region grammar (`OpTag`, `OpAttrs` with canonical bytes, `RegionKey`, `PatternNode`, the shape-expression codec), moved from Fuel's `fuel-kernel-seam-types` with a guard that keeps `PatternNode` a closed enum. The root version here is provisional; the crate versions by its own interface.

## [0.0.7] - 2026-11-07

- New crate `fireline-announce` 0.1.0 (not published): the kernel-seam handshake, the frozen 56-byte `SeamHello`, capability bits and role-aware negotiation, merged from Fuel's `fuel-kernel-seam-announce` and baracuda's `baracuda-seam`. The root version here is provisional; the crate versions by its own interface.

## [0.0.6] - 2026-11-07

- Patent sweep addendum (ideas 4 and 11 searched, two more patents read), plan section 12 (`fireline-dlpack` whole-tree findings and the open FDX-spec-home decision), and the lane HANDOFF. Docs only.

## [0.0.5] - 2026-11-07

- Tilde lint: `[patch]` tables are scanned, untracked manifests are included, requirement versions are validated strictly, and a manifest listed but missing on disk is reported instead of aborting the run (reviewer findings on #3, each reproduced first). Tool only; nothing published.

## [0.0.4] - 2026-11-07

- Extraction plan (`docs/EXTRACTION-PLAN.md`, sections 1-11) and the lane HANDOFF.
- Patent sweep record, first pass, for `fireline-announce`, `fireline-grammar` and `fireline-synth` (`docs/PATENT-SWEEP.md`, replacing the root placeholder). It says what was not read.
- The root package and `fireline-tilde-lint` now share one `[workspace.package]` version, so they cannot drift apart (CodeRipper version-consistency); interface crates will not inherit it.

## [0.0.3] - 2026-11-07

- Workspace skeleton and `fireline-tilde-lint` (not published): fails when a manifest requires a FireLine crate with a caret, a bare version, a range or a `git` source. CI runs it on this repo; consumers run it on theirs. No FireLine crate exists yet, so nothing is published.

## [0.0.2] - 2026-11-07

- Seam inventory (`docs/SEAM-INVENTORY.md`). Docs only.

## [0.0.1] - 2026-10-28

- HANDOFF for the new FireLine lane. Docs only.

## [0.0.0] - 2026-10-28

- Name reserved with an empty placeholder crate.
