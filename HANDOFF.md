# HANDOFF - FireLine lane (current state, rewritten 2026-11-07 by the lane)

You are the **FireLine lane** (role `fireline`). AGENT-ONBOARDING.md already read once. Messages: Synapse + claude-peers (PM role `pm`). Models: sonnet only.
Repo `ThinkersJournal/fireline`, crate `fireline` 0.0.0 placeholder. Name: CireSnave's final yes pending, PM treats as accepted.

## Mission (unchanged)
ONE project, ONE CRATE PER INTERFACE between Fuel and the other ML projects (board 295, CireSnave: "the interface crates (both sides of each interface) should be spun out into their own projects so many projects could use them and be sure they had no drift of their interfaces"; ruling 2026-10-28 "Start now. Spin them out immediately."). Each crate versioned by its interface; pre-1.0 a break moves that crate's SECOND digit alone; all meet at a common version at majors (first KISS freeze). **Consumers require `~` or exact, never `^`; a CI check fails on a caret.** Boundary: only what has no dependency on a project's internals moves. Memory: CARGO_BUILD_JOBS=2, one build at a time. Patent sweep before any publish. PM allocates versions, merges, publishes.

## Done
- Task 1: `docs/SEAM-INVENTORY.md`, PR #2 (head 8a0ec57, docs only), `[READY]` sent to PM. Rows are verified / PEER / UNVERIFIED.
- Inputs read (all PEER): Fuel `docs/design-notes/2026-11-07-fireline-fuel-baracuda-seam-plan.md` (Fuel main, PR #630, incl. baracuda half section 8); Unpopped#315 branch `fireline-plan` at `1dd62715` `docs/fireline-unpopped-interface-plan.md`; KISS `docs/plan-interface-crates-kiss-formats.md` at `a3b48ad`.

## Open conflicts the plan (task 2) must resolve
1. Grammar crate owner: Fuel plan (`fuel-kernel-seam-types` -> FireLine) vs KISS plan (`fireline-kiss-region` etc.). Needs Fuel + KISS + PM.
2. First-release timing: Fuel wants announce/grammar/synth first (hurts now); KISS wants after its 0.9.0 batch; Unpopped wants `fireline-*` start at 0.1.0.
3. Capability names FDX vs DLPACK_EXT, PROVISION_ON_REQUEST vs JIT_ON_REQUEST (bits 0-5, 32, 33 freeze; 34/35 stay provisional); `shape_expr` home (3 copies).
4. Not in any lane plan: region conversion (`baracuda-cuda-seam`), `baracuda-cuda-vocab`, `kiss-vulkan-vocab`. `build_plan`/`derive_pattern` stay in Unpopped (Unpopped's recommendation).
5. Added conformance item from baracuda: every ADVERTISED capability bit must be allocated in the shared crate.

## Tasks, in order
1. DONE, awaiting merge (PR #2).
2. **One-page extraction plan** (docs/EXTRACTION-PLAN.md): which crates move, order (Fuel-to-producer seam first), how each side migrates (one coordinated breaking release), version scheme + tilde check, relation to KISS crates, patent sweep, ease rule (EXPECTATIONS 6.24: hello-world first). Also query crates.io reverse deps of `fuel-kernel-seam`, `-types`, `-announce` (with a positive control). Start now; do not wait for replies.
3. Skeleton workspace + tilde-lint + first crate moved, red-first. One PR at a time; CI + protection required.

## Waiting (asks sent 2026-11-07 ~12:05Z)
Replies from `lightbulb`, `mlmf`, `vulkane`, `fishstringer` (must-haves). `overmind` is offline: ask the PM to route. Fold replies into SEAM-INVENTORY (new PR after #2 merges).

WAIT: PR #2 merged | check: gh pr view 2 --json state,mergedAt | wake: 2026-11-07T13:30:00Z
WAIT: fireline branch still pushed (inventory safe) | check: git ls-remote --heads origin lane/fireline-work | wake: 2026-11-07T14:30:00Z

## State
main = placeholder 0.0.0 + HANDOFF 0.0.1. Open PR: #2. Branch `lane/fireline-work` pushed. Untracked `supertool` in the worktree is not mine; never add it.
