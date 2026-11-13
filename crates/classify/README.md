# fireline-classify

KISS-Classify **dtype definitions** for the seam: the three MX element dtypes `f4e2m1`, `f6e2m3` and `f6e3m2`
(KISS-CLASSIFY-6.1-0014, -0015, -0016): the value rule, the byte packing (two per byte, low nibble first; four per three bytes,
least-significant bit first), and the rule that an MX element is a storage and operand dtype, never an accumulator.

- **Draft-tracking.** KISS-Classify is a Draft (CireSnave 2026-11-12: "working toward the draft version is the best we have").
  Expect rework when it moves; if KISS's quantisation group changes these clauses, this crate follows. Every rule cites its clause id.
- **Scope is 6.1-0014..-0016 only.** Not here: the other 24 dtypes, the `structure_key` grammar, converting a float into these formats
  (owned by KISS-Ops and KISS-Emit), and the per-block `f8e8m0` scale operand.
- **FDX V4.** This is where V4 of the FDX validators lands (`docs/design-fdx-phase-out.md` in KISS maps V4 to CLASSIFY-6.1-0014..-0016);
  it lives here, not in `fireline-dlpack`, which goes away with the phase-out.
- **Dependency-free.**
- **Require it with a tilde** (`fireline-classify = "~0.1"`) or an exact version, never a caret. `tools/tilde-lint` fails a caret.
- **Hello world:** `cargo run -p fireline-classify --example classify_hello_world`, or the doctest on the crate root.

## Provenance

Written from the Draft text (KISS `spec/classify.md` at `origin/main` `73c341b5`). The format constants are the ones that text cites from the
OCP Microscaling Formats Specification v1.0 (sections 5.3 to 5.3.3); the 6-bit packing is the Unpopped lane's storage convention as that text
says, not an OCP requirement. The tests check the clause's own numbers and the token and width rows of KISS `conformance/corpus/dtype_manifest.json`
(same owner, `MIT OR Apache-2.0`). No third-party code.
