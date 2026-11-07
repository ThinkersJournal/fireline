# HANDOFF - FireLine lane (current state, rewritten 2026-11-07 16:00Z by the lane)

You are the **FireLine lane** (role `fireline`). AGENT-ONBOARDING.md read. Messages: Synapse + claude-peers (PM role `pm`). Models: sonnet only. Shell tips: a long bash heredoc with a python body fails to parse here, so write scripts with the Write tool; **run `coderipper sweep --profile portfolio` (installed, 0.20.1) before every push**; never use `| head` between a command and its exit code.
Repo `ThinkersJournal/fireline` (squash-only, merged branches auto-delete; **a push to a deleted branch name silently recreates it**, so check `gh pr view` before pushing to a PR branch). Mission: ONE project, ONE CRATE PER INTERFACE between Fuel and the other ML projects (board 295). Consumers require FireLine crates with `~` or exact, never `^` (`tools/tilde-lint`). PM allocates versions, merges, publishes (I ask for crates.io steps).
CireSnave 2026-11-07, verbatim: "Yes." (wave 1 now, each crate at 0.1.0). And: "All Fuel backends will talk with Fuel via FireLine.  Unpopped will also talk with Fuel via FireLine.  We should have a clean seam between Fuel and each external thing it connects to.  Custom wiring one directly into Fuel is the wrong approach.  That most likely means AOCL, oneMKL, and the rest need to change."

## State
- main = 6accbcb, version 0.0.3 (#2 inventory and #3 skeleton+tilde lint merged; their branches and worktrees are gone, except `C:/Projects/fireline-wt-skeleton`, now on `lane/fireline-lint-fixes`).
- **This PR (plan, `lane/fireline-plan-pr`)**: docs/EXTRACTION-PLAN.md sections 1-11 + this HANDOFF; PM stamps the version (expected 0.0.4).
- **Next PR: `lane/fireline-lint-fixes` 3924279** (worktree fireline-wt-skeleton): Pip's three Low findings on #3, all reproduced first and fixed red-first (`[patch]` bypass, untracked manifests, strict versions; 16+4 tests, 3 mutants caught). Open it after the plan PR; PM stamps the version. Reply to Pip on #3 is in that PR's body.
- Built, no PRs yet (all rebased onto main 6accbcb, pushed, tests and lint green): `lane/fireline-announce` 95bf99f (`fireline-announce` 0.1.0, 17 tests); `lane/fireline-grammar` ed30aee (`fireline-grammar` 0.1.0, 38+7+3 tests); `lane/fireline-synth` 6328855 (`fireline-synth` 0.1.0 on top of grammar). Worktrees `C:/Projects/fireline-wt-{announce,grammar,synth}`. Each needs a root version bump and a CHANGELOG entry at PR time (PM stamps), an SPDX header on the crate sources if the sweep flags it, and a real patent record.
- Old branch `lane/fireline-plan` (17 commits) is the backup of this PR's content; delete it after the plan PR merges.

## PM answers 2026-11-07 (read)
- version-consistency: no CireSnave ruling needed (each interface crate has its own version, meeting at one number at majors). PM tasked the coderipper lane to add a per-interface mode. **Do not silence the rule.** Wave 1 continues on branches; the crate PRs wait for that mode or a documented interim. I prefer the interim (open the three PRs with the [High] finding named in each body, PM merges since the check is advisory) so Fuel can migrate; PM to say.
- repo settings finding is the PM's (squash-only + auto-delete, being set).
- **patent-sweep-record is MINE:** run `C:\Projects\PATENT-SWEEP-PROCESS.md` on the claims of announce, grammar and synth, starting from Fuel's chart (`docs/PATENT-SWEEP-CHARTS.md`, `docs/PATENT-SWEEP.md` in Fuel `origin/main`; baracuda and KISS also have a `docs/PATENT-SWEEP.md`), write the REAL record at `docs/PATENT-SWEEP.md`, say what is unread. Claims are read with `python C:\Projects\tools\patent-claims.py <number>`; a hit whose claims were not read is `unread`, never `clear`. Never invent a record.

## Decided
Grammar option A. Frontend/lift/convert stay in Unpopped. Caps `DLPACK_EXT`, `PROVISION_ON_REQUEST`; bits 34/35 not allocated here. Fuel agrees P2 (`fireline-provider`, plan 10) and will do its wave-1 PR once crates are PUBLISHED (send names + versions). `fireline-contract` and `fireline-provider` wait for CireSnave's ruling on who owns the provider callable (plan 10.6). Region conversion waits for wave 2 (needs `unpopped`).

## Next (do not wait to be told)
1. Send the plan PR `[READY]`; then the lint-fixes PR.
2. Patent sweep (above). Then ask the PM for the crates.io publish steps.
3. `fireline-dlpack` (pure move from `fuel-ir/src/dlpack`: abi, codes, sidecar, header_check, validate + `include/fuel_dlpack_ext.h`; Fuel confirmed validate uses only abi/codes/sidecar; `fuel-ir` re-exports). Search the whole tree, every file type, for names and test paths first (plan 11).
4. Not measured: size of each backend cut; Metal ownership; whether Fuel's `fuel-loaders` requirement on `mlmf-core` is a tilde.

WAIT: plan PR merged | check: gh pr list --head lane/fireline-plan-pr --state merged --json number | wake: 2026-11-07T17:30:00Z
WAIT: fireline crate branches still pushed (work safe) | check: git ls-remote --heads origin lane/fireline-synth | wake: 2026-11-07T18:30:00Z

## Housekeeping
Untracked `supertool` in this worktree is not mine; never add it. Primary checkout `C:/Projects/fireline` is on main at 6b7bb3e (stale; `tools/sync-checkouts.sh` fast-forwards it; not mine to force).
