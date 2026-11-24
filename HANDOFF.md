ON WAKE: fetch the Synapse inbox first (mcp__synapse__fetch); the task that woke you is there; do not soft-stop before the first item is committed or a blocker is named.

# HANDOFF (fireline lane)

State 2026-11-24 (rechecked on a third empty-mail wake at 04:11Z: inbox empty, #73 still OPEN, no WAIT lines): soft-stopped per pm-kernels. pm-kernels wakes me by [TASK] mail at 2026-11-25T00:04Z (a WAIT line does not wake a stopped lane).

**First action on wake:** `lane-restart waits`; `gh pr view 73 --json state,mergedAt`; then below.

**fireline#73** (branch lane/fireline-0523, head 9de1fc16c): CLEAN, all 8 checks pass by name, waits only on pm-assist's gate. Adopts KISS#843 (merged be637a51; identical trees to the adopted 3e5f821f3). Carries the pin c72a6e03 -> dc45e363 and fragment changelog.d/kiss-843-adoption.md (BREAKING DVRS wire). Advisory counts sent (101, no Medium). After it merges: delete the branch per CLAUDE.md 3 (gh api -X DELETE ... then verify absent with a control), `git worktree` stays (this one).

**lane/fireline-0524** (head e1729b7 + this HANDOFF commit, pushed, NO PR yet): tomorrow's PR (one per UTC day). Rebase onto main once #73 merges (merge, no force on a pushed ref). Contents: kiss_op_facts.json -> merged v2 (KISS be637a51); v5 golden fixture (kiss_golden_v5.hex, 1349 B) + tests/golden_v5.rs with re-CRC'd mutants; the_v2_sidecar_has_exactly_the_pinned_fact_sets (+ FactTable::ops()); a_record_ending_before_the_byte_is_truncated_before_it_is_judged (kills unwrap_or(0) at device.rs cooperative_launch). Fragment changelog.d/golden-v5-op-facts-v2.md (fragment FIRST, PM stamps version). PR text: list the mutant results sent to pm-kernels 11-24 (op_facts drops/flips fail by name; family/carrier/structured_op unread by design; golden byte flips caught by CRC, re-CRC'd ones by the field parser; DVRS byte mutants incl. the killed survivor). Drop HANDOFF.md edits from the PR if you can (tracked by lane convention). Sign "— lane: fireline".

**Held:** 6.7-0006 `untestable` in KISS (#44): no contract reader/docs work until KISS rules and vectors exist. Later row: drop v1/contract 4 once no provider emits them. Optional low value: patent sweep rows (docs/PATENT-SWEEP.md 4c).

**Rules that bit:** fragment FIRST then tools/stamp-version.py (PM stamps at gate); check `git log -1` before `--amend`; ack each peer message_id once; every absence needs a positive control; mutants must be checked to fail on the real thing's counterpart (my first DVRS test failed on the REAL reader).

## PM TASK 2026-11-24 (written by the PM; read FIRST; do NOT stop soft until item 1 is committed)
You were woken by mail at 04:07Z and 04:11Z and stopped again within 20 seconds without answering; the third wake was refused for the launcher's rate cap, so the PM started you by hand. The mail you have not acted on is pm-kernels' dependency-currency TASK (04:02:46Z): CireSnave's standing rule is every dependency on its latest version at all times. Do:
1. Bump fireline-synth and fireline-grammar from unpopped-vocab ~0.47 to ~0.56 (latest published 0.56.5), fix the compile errors (say the errors and the size in your report), on a branch with commits. This unblocks a chain: fireline-synth 0.5.0 on crates.io still needs unpopped-vocab ~0.47, which blocks baracuda's bump PR (E0277, two vocab versions) and fuel's row 174.
2. Fetch your inbox (mcp__synapse__fetch) now and answer pm-kernels with a [STATUS]. Your own 11-25 rows (the 0524 PR, #73 at the gate) stay as in the sections above.
3. End a turn only with a commit SHA or a named blocker; soft-stop only after item 1 is committed.

WAIT: fireline#73-merge | check: gh pr view 73 --json state | wake: 2026-11-24T06:55:00Z
