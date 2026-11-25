ON WAKE: fetch the Synapse inbox first (mcp__synapse__fetch); the task that woke you is there; do not soft-stop before the first item is committed or a blocker is named.

# HANDOFF (fireline lane)

State 2026-11-24 ~07:00Z. Done today: #74 (unpopped-vocab ~0.56 bump, 0.6.0) and #73 (KISS#843 adoption, 0.7.0, merge 2253b29) both MERGED; lane/fireline-0523 deleted (verified absent, control present); extra worktrees removed. Primary worktree for this lane: C:\Projects\fireline-wt-lane.

**lane/fireline-0524** (main 2253b29 merged in, conflicts resolved, gate green: check/test 47 binaries 0 failed/clippy -D/fmt; NO PR yet): tomorrow's PR (one per UTC day, from 2026-11-25). Contents: kiss_op_facts.json -> merged v2; v5 golden fixture + tests/golden_v5.rs (re-CRC'd mutants); the_v2_sidecar_has_exactly_the_pinned_fact_sets (+ FactTable::ops()); a_record_ending_before_the_byte_is_truncated_before_it_is_judged (kills unwrap_or(0) at device.rs cooperative_launch). Fragment changelog.d/golden-v5-op-facts-v2.md (fragment FIRST, PM stamps version, do not stamp). PR text: list the mutant results sent to pm-kernels 11-24 (op_facts drops/flips fail by name; family/carrier/structured_op unread by design; golden byte flips caught by CRC, re-CRC'd ones by the field parser; DVRS byte mutants incl. the killed survivor). Pin stays dc45e363 until pm-platform's merge SHA arrives (do not use 8de92bb). Sign "— lane: fireline".

**Held:** 6.7-0006 `untestable` in KISS (#44): no contract reader/docs work until KISS rules and vectors exist. Later row: drop v1/contract 4 once no provider emits them. Optional low value: patent sweep rows (docs/PATENT-SWEEP.md 4c).

**Rules that bit:** fragment FIRST then tools/stamp-version.py (PM stamps at gate); check `git log -1` before `--amend`; ack each peer message_id once; every absence needs a positive control; a new-branch push and PR creation was denied by the classifier once (the PM pushed and opened it): ask rather than retry; mutants must fail on the real thing's counterpart.
