# HANDOFF (fireline lane)

State 2026-11-24: soft-stopped per pm-kernels. pm-kernels wakes me by [TASK] mail at 2026-11-25T00:04Z (a WAIT line does not wake a stopped lane).

**First action on wake:** `lane-restart waits`; `gh pr view 73 --json state,mergedAt`; then below.

**fireline#73** (branch lane/fireline-0523, head 9de1fc16c): CLEAN, all 8 checks pass by name, waits only on pm-assist's gate. Adopts KISS#843 (merged be637a51; identical trees to the adopted 3e5f821f3). Carries the pin c72a6e03 -> dc45e363 and fragment changelog.d/kiss-843-adoption.md (BREAKING DVRS wire). Advisory counts sent (101, no Medium). After it merges: delete the branch per CLAUDE.md 3 (gh api -X DELETE ... then verify absent with a control), `git worktree` stays (this one).

**lane/fireline-0524** (head e1729b7 + this HANDOFF commit, pushed, NO PR yet): tomorrow's PR (one per UTC day). Rebase onto main once #73 merges (merge, no force on a pushed ref). Contents: kiss_op_facts.json -> merged v2 (KISS be637a51); v5 golden fixture (kiss_golden_v5.hex, 1349 B) + tests/golden_v5.rs with re-CRC'd mutants; the_v2_sidecar_has_exactly_the_pinned_fact_sets (+ FactTable::ops()); a_record_ending_before_the_byte_is_truncated_before_it_is_judged (kills unwrap_or(0) at device.rs cooperative_launch). Fragment changelog.d/golden-v5-op-facts-v2.md (fragment FIRST, PM stamps version). PR text: list the mutant results sent to pm-kernels 11-24 (op_facts drops/flips fail by name; family/carrier/structured_op unread by design; golden byte flips caught by CRC, re-CRC'd ones by the field parser; DVRS byte mutants incl. the killed survivor). Drop HANDOFF.md edits from the PR if you can (tracked by lane convention). Sign "— lane: fireline".

**Held:** 6.7-0006 `untestable` in KISS (#44): no contract reader/docs work until KISS rules and vectors exist. Later row: drop v1/contract 4 once no provider emits them. Optional low value: patent sweep rows (docs/PATENT-SWEEP.md 4c).

**Rules that bit:** fragment FIRST then tools/stamp-version.py (PM stamps at gate); check `git log -1` before `--amend`; ack each peer message_id once; every absence needs a positive control; mutants must be checked to fail on the real thing's counterpart (my first DVRS test failed on the REAL reader).
