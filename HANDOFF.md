# HANDOFF - FireLine lane (current state, rewritten 2026-11-12 by the lane)

You are the **FireLine lane** (role `fireline`). AGENT-ONBOARDING.md read. Messages: Synapse + claude-peers (PM role `pm`). Models: sonnet only. Sign every PR body/comment/message `— lane: fireline`. Shell tips: write scripts with the Write tool (long bash heredoc with a python body can fail to parse); commands over ~2 min go to the background; **`coderipper sweep --profile portfolio` reads COMMITTED content: commit, run, then push**; `scratchpad/verify2.sh <worktree>` = the whole local CI set; plain `cargo metadata` refreshes the lock; never `| head` between a command and its exit code.
Repo `ThinkersJournal/fireline` (squash-only; merged branches auto-delete; **pushing to a deleted branch name silently recreates it**: check `gh pr view` first). Mission: ONE project, ONE CRATE PER INTERFACE between Fuel and the other ML projects. FireLine crates are required with `~` or exact, never `^` (`tools/tilde-lint`). PM allocates versions, merges, publishes (I ask for crates.io steps). **Example names must be unique across the workspace**: all four crates ship `<crate>_hello_world`; `cargo build --locked --all-targets` must print no `collision`.

## State (2026-11-12, after #19)
- main has #17 (repin 0.43.41), #18 (fragment+stamp kit), #19 (repin to coderipper 33cc3b7 = 0.43.49); PM stamps versions. Every PR is merged; origin has only `main`. **announce, grammar, synth 0.1.x are on crates.io**; Fuel is on `~0.1`.
- **Every PR from now: one `changelog.d/<topic>.md` fragment (format in `changelog.d/README.md`), NO version/CHANGELOG/Cargo.lock edit.** `python tools/check-fragment.py origin/main` after committing.
- **IDLE UNTIL THE FREEZE (PM ruling a, 2026-11-12): no work on a Draft.** PM messages me when either blocker moves: (1) KISS freeze: Provision text is `spec/synth.md` (Draft, freeze gate unmet; Provision-by-definition wire work unwritten); (2) CireSnave 398 A: re-check US 2023/0025245 A1 for a grant before `fireline-provider` is published and before Fuel's next release. Blocked: `fireline-contract`, `fireline-provider`.
- `fireline-dlpack` cannot retire yet (kiss, origin/main@f4b2932c): V1..V22 mapped in KISS `docs/design-fdx-phase-out.md`, but clauses exist only for V7/V14/V17 (CONTRACT-6.12-0001..6) and V1/V4/V10/V11; V3/V8/V9/V12/V13 wait on Fuel's window formula; V5/V6 and V18..V21 have no clause. Fixes only, no extension.
- Fuel's wave-2 scope was answered 2026-11-11: contract then provider, first proof CPU `gemm`, then Vulkan, then AOCL. `ROADMAP.md` has no rows (one paragraph).

## Decided (CireSnave 2026-11-09, verbatim)
"I think there was a misunderstanding about something I said.  I said Fuel's backends reach Fuel only through the seam.  However, what I meant was that every seam has definitions in FireLine that other crates use to define how they communicate.  I wasn't saying every project that touched a seam had to call into FireLine and have FireLine call out to the other side of the seam.  I'm okay if that is deemed the best way to build it, but that was not my intent."
So **FireLine owns seam DEFINITIONS**; the sides may talk directly; a run-time hop only where it is the best design. Board 395: providers live in their own projects; Fuel's built-in CPU path stays; "KISS owns the standards.  FireLine implements the seam standards' format information.  KISS-ref implements the whole of KISS." FDX is phased out for a KISS standard (plan 12). Grammar option A; frontend/lift/convert stay in Unpopped.

## Next
Nothing unblocked. On a PM "freeze" or "398 A" message: re-read `origin/main` of C:/Projects/KISS (spec/synth.md), then start `fireline-contract` (own PR, own `[[interface]]` entry in `.coderipper.toml`, own sweep entry). Not measured: size of each backend cut; Metal ownership; whether `fuel-loaders`' requirement on `mlmf-core` is a tilde; unread patents in `docs/PATENT-SWEEP.md`.

## Housekeeping
Untracked `supertool` in this worktree is not mine; never add it. Primary checkout `C:/Projects/fireline` is stale on main (`tools/sync-checkouts.sh` fast-forwards it; not mine to force). No WAITs are armed.
