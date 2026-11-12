# HANDOFF - FireLine lane (current state, rewritten 2026-11-12 by the lane)

You are the **FireLine lane** (role `fireline`). AGENT-ONBOARDING.md read. Messages: Synapse + claude-peers (PM role `pm`). Models: sonnet only. Sign every PR body/comment/message `— lane: fireline`. Shell tips: write scripts with the Write tool (long bash heredoc with a python body can fail to parse); commands over ~2 min go to the background; **`coderipper sweep --profile portfolio` reads COMMITTED content: commit, run, then push**; `scratchpad/verify2.sh <worktree>` = the whole local CI set; plain `cargo metadata` refreshes the lock; never `| head` between a command and its exit code.
Repo `ThinkersJournal/fireline` (squash-only; merged branches auto-delete; **pushing to a deleted branch name silently recreates it**: check `gh pr view` first). Mission: ONE project, ONE CRATE PER INTERFACE between Fuel and the other ML projects. FireLine crates are required with `~` or exact, never `^` (`tools/tilde-lint`). PM allocates versions, merges, publishes (I ask for crates.io steps). **Example names must be unique across the workspace**: all four crates ship `<crate>_hello_world`; `cargo build --locked --all-targets` must print no `collision`.

## State (2026-11-12)
- main = f233cad (#17: portfolio job repinned to coderipper 4bcf55e = CodeRipper 0.43.41; the four version-consistency Highs are gone), root 0.0.16. #15/#16/#17 merged. **announce, grammar, synth 0.1.0 (+0.1.1 metadata) are on crates.io**; Fuel is on `~0.1` (fuel#644).
- **Open: #18** (`lane/fragment-stamp`) adopts the PM's fragment+stamp kit (root 0.0.17 provisional). Dry run and a real stamp on a scratch copy passed (no old version left). **After it merges: every PR carries one `changelog.d/<topic>.md` fragment (format in `changelog.d/README.md`) and NO version/CHANGELOG/Cargo.lock edit; the PM stamps at merge.** `tools/check-fragment.py origin/main` checks it (run after committing).
- Blocked, do not start: `fireline-contract`, `fireline-provider` (wait on KISS-Provision text, plan 10.6; confirm with the KISS lane; provider also on the US 2023/0025245 A1 grant re-check, CireSnave ruled 398 A: re-check before provider is published and before Fuel's next release). `fireline-dlpack`: fixes only; retirement is plan row 12 once the KISS lane maps V1-V21 and Fuel has moved.
- Fuel asked wave-2 scope (answered 2026-11-11): contract then provider, no dlpack extension, first proof CPU `gemm` then Vulkan then AOCL.

## Decided (CireSnave 2026-11-09, verbatim)
"I think there was a misunderstanding about something I said.  I said Fuel's backends reach Fuel only through the seam.  However, what I meant was that every seam has definitions in FireLine that other crates use to define how they communicate.  I wasn't saying every project that touched a seam had to call into FireLine and have FireLine call out to the other side of the seam.  I'm okay if that is deemed the best way to build it, but that was not my intent."
So **FireLine owns seam DEFINITIONS**; the sides may talk directly; a run-time hop only where it is the best design. Board 395: providers live in their own projects; Fuel's built-in CPU path stays; "KISS owns the standards.  FireLine implements the seam standards' format information.  KISS-ref implements the whole of KISS." FDX is phased out for a KISS standard (plan 12). Grammar option A; frontend/lift/convert stay in Unpopped.

## Next (do not wait to be told)
1. #18: read CI, count unresolved threads (GraphQL), `[READY]` to the PM; after merge clean up (worktree on a fresh branch, remote branch absent with a positive control).
2. Ask the KISS lane whether KISS-Provision has text yet; if so start `fireline-contract` (own PR, own `[[interface]]` entry in `.coderipper.toml`, own sweep entry).
3. Not measured: size of each backend cut; Metal ownership; whether `fuel-loaders`' requirement on `mlmf-core` is a tilde; unread patents in `docs/PATENT-SWEEP.md`.

## Housekeeping
Untracked `supertool` in this worktree is not mine; never add it. Primary checkout `C:/Projects/fireline` is stale on main (`tools/sync-checkouts.sh` fast-forwards it; not mine to force). No WAITs are armed.
