ON WAKE: fetch the Synapse inbox first (mcp__synapse__fetch); do not soft-stop before the first item is committed or a blocker is named.

# HANDOFF (fireline lane)

State 2026-11-26: main has 0.9.1 (e21c933). Worktree C:/Projects/fireline-wt-lane is live. Open: fireline#80 (unpopped-vocab ~0.59.1; pm-assist stamped it, do NOT push to lane/fireline-take059; told pm-assist the pub API carries unpopped types, so 0.10.0 not 0.9.2). Branch lane/fireline-kiss024 (pushed, no PR): re-vendored KISS 0.24.0 dtype manifest (34) + opattrs field types, test green; needs a PR slot from pm-assist (ask first).

**Next:**
1. Watch #80 (`gh pr view 80 --json state`); when merged: delete branch (gh api DELETE + verify), no worktree removal (this is the lane worktree).
2. Ask pm-assist for a slot, open PR for lane/fireline-kiss024; sign `- lane: fireline`.
3. classify/src/lib.rs:111 doc says "other 24" (now 31): doc-only, ASK the KISS lane before touching (PM ruling 2026-11-26).

**WAITS (mail wakes you; no timers):** dlpack provider held on 398 A; classify KISS-quantisation condition stays OPEN; cost 6.7-0006 waits on KISS R1.25 (#125); patent description reads of new rows.

**Owed, none urgent:** '245 amended claims re-read on a Patent Center/Espacenet answer or a grant. Held: drop v1/contract 4 once no provider emits them.

**Rules that bit:** fragment FIRST; git pathspec `crates/*/src` matches nothing (use dirs; control with `fn ` count); `git branch -D` only after gh+ls-remote verify; never force-remove a worktree with uncommitted files; ack each message_id once; positive control for every absence. Repo is ThinkersJournal/fireline.
