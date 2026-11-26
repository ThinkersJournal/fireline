ON WAKE: fetch the Synapse inbox first (mcp__synapse__fetch); the task that woke you is there; do not soft-stop before the first item is committed or a blocker is named.

# HANDOFF (fireline lane)

State 2026-11-26 ~01:25Z. origin/main 3bfe2b9 = 0.8.0, all six crates published by the PM (crates.io read 01:17Z). #76 and #77 merged, their branches/worktrees removed. Work worktree: C:/Projects/fireline-wt-0525 on lane/fireline-1126 @13a7c01 (pushed, NO PR). Old worktree C:/Projects/fireline-wt-lane (merged lane/fireline-0524) stays pending board 504: no delete, no checkout, no force.

WAIT: pm-assist slot mail for lane/fireline-1126 PR | check: git ls-remote --heads origin lane/fireline-1126 | wake: 2026-11-26T13:00:00Z

**TASK 1 (on pm-assist's slot mail, a 6.33(4) catch-up release; do NOT self-open):** open ONE docs-only PR from lane/fireline-1126 (merge origin/main first; fragment changelog.d/patent-sweep-3-roadmap.md already there, PM stamps). Contents: PATENT-SWEEP slice 3 + watch row + '245 description reading; ROADMAP at 0.8.0 with classify/root hold notes; docs/COST-6.7-0006-NOTE.md. [READY] pm-assist cc pm-kernels.

**Owed, none urgent:** (a) patent descriptions of US 2026/0147493, US 2025/0181351, US 12,423,007 B1, US 11,720,351 B2 (verdicts only outside as read / needs more with limb / NOT CLEARED unread; never "clear"); (b) watch row: re-read '245 amended claims when Patent Center/Espacenet answer (403 today) or on a grant; (c) 6.7-0006: KISS-Contract #125 OPEN, no code; ask KISS whether `cost = 1` (their own serialized example) is legal. Open question sent to pm-kernels: is the root crate `fireline` 0.8.0 (placeholder description) meant to ship; I did not yank. Held: drop v1/contract 4 once no provider emits them. If no row left: lane-restart assert-idle, then soft-stop with reason.

**Rules that bit:** fragment FIRST; em-dash breaks awk regex (use python); `git branch -D` only after gh+ls-remote verify; never force-remove a worktree with uncommitted files; ack each message_id once; positive control for every absence; do not push to a branch pm-assist is stamping; a session timer is not a wait.
