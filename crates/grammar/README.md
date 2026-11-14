# fireline-grammar

The kernel-seam **region grammar**: `OpTag`, `OpAttrs` (with canonical bytes and `UnresolvedAttr`), `RegionKey`, `PatternNode` and the
shape-expression codec (`shape_expr`), shared by a kernel *consumer* (an orchestrator such as Fuel) and a kernel *provider* (a synthesizer
such as baracuda). One definition, so neither side can drift. **Types only, no logic, dependency-free.**

- **Require it with a tilde** (`fireline-grammar = "~0.2"`) or an exact version, never a caret. `tools/tilde-lint` fails a caret.
- What stays with the consumer: the `Op -> OpTag` projection and the structural matcher need its graph (in Fuel: `fuel_graph::jit`).
- What is **not** here: baracuda's conversion of this grammar to Unpopped's `PatternNode` (`to_internal`, `hash_region`, `region_op_id`).
  It depends on `unpopped`, so it waits for wave 2, when Unpopped's IR types move to a FireLine crate it can depend on.

## Provenance

Moved from `fuel-kernel-seam-types` (Fuel `origin/main` `d010189e`, `MIT OR Apache-2.0`, every source file carries our SPDX header). `canonical.rs`
and `shape_expr.rs` are byte-identical to Fuel's; `lib.rs` differs by **one live-comment correction** (the `EXHAUSTIVE-BY-DESIGN` note on
`PatternNode` now points at this crate's guard, not at a Fuel test that can no longer see the type). The in-crate tests move with it, and
so does `tests/kiss_ops_619_divergence.rs` (only its `use` path changed), which pins where this encoding **diverges** from KISS-Ops 6.19.3; it
is not an interop test and says so at the top. **Added here:** `tests/exhaustive_by_design.rs`, the guard that keeps `PatternNode` closed,
ported from Fuel's `fuel-ir/tests/exhaustive_by_design_marker.rs` (its whole-tree scan covered `PatternNode` only while it lived in Fuel). `shape_expr` byte-matches the KISS reference through golden vectors; **KISS owns those vectors** and this crate consumes them,
never owns a copy.
