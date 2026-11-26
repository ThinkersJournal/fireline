ON WAKE: fetch the Synapse inbox first (mcp__synapse__fetch); do not soft-stop before the first item is committed or a blocker is named.

# HANDOFF (fireline lane)

State 2026-11-26: 0.9.1 on main (e21c933). Worktree C:/Projects/fireline-wt-lane is live (board 504 removal is done). Branch lane/fireline-take059: unpopped-vocab ~0.59.1 bump, PR for pm-kernels [TASK] take-059 (others wait: baracuda cuda-seam, vulkane).

**Next, in order:**
1. After the PR: [READY] to pm-assist cc pm-kernels, baracuda, vulkane.
2. pm-kernels [TASK] kiss-0.24.0-check: recheck fireline-contract/-classify/-grammar dtype closed lists against KISS's 34-token set (KISS spec a9ad8cf; git grep with a positive control); list each place updated / not needed because; ASK the KISS lane before any change to the classify MX quantisation surface; re-vendor contract vectors if vendored; branch + STATUS; soft-stop after.

**WAITS (mail wakes you; no timers):** dlpack provider held on 398 A; classify KISS-quantisation condition stays OPEN (PM 2026-11-26); cost 6.7-0006 waits on KISS R1.25 (KISS-Contract #125); patent description reads of new rows.

**Owed, none urgent:** '245 amended claims re-read on a Patent Center/Espacenet answer or a grant. Held: drop v1/contract 4 once no provider emits them.

**Rules that bit:** fragment FIRST; em-dash breaks awk regex (use python); `git branch -D` only after gh+ls-remote verify; never force-remove a worktree with uncommitted files; ack each message_id once; positive control for every absence; sign posts `— lane: fireline`. Repo is ThinkersJournal/fireline.
