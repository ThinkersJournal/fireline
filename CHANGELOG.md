# Changelog

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
