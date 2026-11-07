# fireline-synth

The kernel-seam **JIT-on-request envelope**: the `Synthesizer` trait a kernel *provider* (such as baracuda) implements, and the `JitRequest`,
`JitBudget`, `JitResponse`, `SynthArtifact`, `ArtifactKind` and `LinkEntry` types a *consumer* (such as Fuel) calls it with.
Two steps, so a kernel the consumer's cost gate declines costs nothing: `synthesize` returns a **light** handle (or a decline), and only
`take_kernel` hands over the retained artifact, once the consumer decides to adopt.

- **Require it with a tilde** (`fireline-synth = "~0.1"`) or an exact version, never a caret. `tools/tilde-lint` fails a caret.
- Depends on `fireline-grammar` (the region) and `unpopped-vocab` (operand and arch descriptions). **Exactly one copy of `unpopped-vocab`
  may be in a build**, or `OperandDesc` is two different types; wave 2 replaces it with a FireLine vocabulary crate.
- What is **not** here: how a consumer adopts an artifact (import the contract, load, bind, register the fused op). That needs the consumer's
  dispatch layer and stays with it.
- **Hello world:** `cargo run -p fireline-synth --example hello_world`.

## Provenance

Moved from `fuel-kernel-seam` (Fuel `origin/main` `d010189e`, `MIT OR Apache-2.0`, SPDX header kept). `src/lib.rs` is Fuel's file with two
changes: the crate path `fuel_kernel_seam_types` became `fireline_grammar`, and the inline `#[cfg(test)] mod tests` was **relocated** to
`tests/seam_contract.rs` (it uses only the public API, so a provider can run the same test against its own `Synthesizer`).
