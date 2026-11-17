# Roadmap

Started 2026-10-28 on CireSnave's ruling (board 295: "Start now. Spin them out immediately."). Order: seam inventory, a one-page extraction
plan with the Fuel, baracuda, Unpopped and KISS lanes, then move the Fuel-to-producer seam crates (fuel-kernel-seam, -types, -announce) first.

## Where each crate stands (2026-11-17, origin/main 440660b, workspace 0.5.3)

One crate per interface (CireSnave board 295, 395). "Diffed" means checked against a KISS corpus vector; every other test is written from the clause text and is ours. Only `kiss_section_vectors.tsv`, `kiss_op_dag_vectors.tsv`, `kiss_op_facts.json`, `kiss_dtype_manifest.json` and `kiss_op_decomposition.json` are vendored under `crates/contract/tests/data`; the announce, `contract_vectors` and `slot_overlap_vectors` cases are copied into test code from the cited KISS commit, not loaded from a vendored file.

| Crate | In tree | On crates.io | What it is | Open |
|---|---|---|---|---|
| `fireline-announce` | 0.5.3 | 0.5.0 | the 56-byte `SeamHello` envelope, role-aware negotiation; the `SAVL`, `CYRQ`, `CRSP`, `CDEC`, `DVRQ`, `DVRS`, `DVDC` frames (6.3, 6.4, 6.6), diffed against `announce_vectors.json` | the frames and device module are in the tree and not published: a release is the PM's call |
| `fireline-grammar` | 0.5.3 | 0.5.0 | the region grammar | none known |
| `fireline-synth` | 0.5.3 | 0.5.0 | the JIT-on-request envelope; requires `unpopped-vocab` ~0.47.0 (one copy of the vocabulary) | Baracuda and Vulkane repin to `~0.4` (theirs) |
| `fireline-classify` | 0.5.3 | 0.1.0 | the MX element dtypes (CLASSIFY-6.1-0014..-0016), FDX V4 | **held (PM 2026-11-15):** publish only after KISS's quantisation group (the CLASSIFY quantisation sections and their vector set) settles, and the KISS lane tells this lane; if it changes 6.1-0014..-0016: stop and ask. The `fireline` root crate is held too |
| `fireline-contract` | 0.5.3 | 0.5.0 | the Dispatch expression evaluator and bind-time checks (6.6, 6.12); document framing (6.11); field schemas; Identity, cross-section, typed-value and layout rules (6.3 to 6.9, 6.5); `read_contract`; slot overlap (6.5-0019); diffed against `contract_vectors.json` and `slot_overlap_vectors.json` | see below |
| `fireline-dlpack` | 0.5.3 | not published | FDX, the DLPack extension; cannot retire yet (V3, V8, V9, V12, V13 wait on Fuel's window formula; V5, V6, V18 to V21 have no clause); fixes only | none |
| `fireline-provider` | not started | no | the provider callable and handshake definitions (KISS-Provision, board 395) | held on CireSnave 398 A (US 2023/0025245 A1) |

**`fireline-contract` is not for publishing yet:** the patent sweep has two first passes and still owes a claim-language and CPC pass over the shared-memory layout and swizzle families (`docs/PATENT-SWEEP.md`).

**Built, was waiting on KISS:** `in_place` typed (6.5-0009, KISS#813); the ULP-tier consistency rules (6.7-0005, 6.8); the op DAG and cell class against the named op (6.3-0006, 6.4-0002, 6.11-0007) in `read_contract`, with the KISS-Ops op facts and dtype widths passed in (0.5.0, `KissTables`); diffed against KISS's 39 `op_dag_vectors` and, for `op_resolve`, KISS's `op_decomposition.json` outcome vectors.

**Waiting on KISS, not on us:**
- Vectors requested for the rows not built: the cost expressions (6.7-0006, board 449: CireSnave ruled outside-view metrics, options note pending) and the value types of the `op_identity` forms (6.3-0003, 6.3-0004, board 451: KISS's vectors landed in KISS#835; `op_identity` reads their structure, but the field types and enum vocabularies of the `bad-value` vectors exist only as prose in `spec/ops.md`, so KISS has been asked to emit them; KISS#838, due on main with KISS 0.19.1, is that file. **Follow-up row, next fireline PR after #838 is on main:** read it and decide the 12 `bad-value` vectors; until then they are declined-to-decide, not passed). Resolution to the primitive floor (6.4-0005, 6.4-0010) is built (`op_resolve`, KISS#833 vectors). KISS#811 (0.16.0) changed window-op OpAttrs.
- Vectors: kiss accepted five requests (a strided document with V11, the five-field Dispatch, `output_layouts` and `human_annotation` present, `in_place` aliasing, one decline per field schema); where they do not exist yet those cases are ours.

**Fuel's wave 2 (2026-11-11):** the contract, then the provider, first proof CPU `gemm`, then Vulkan, then AOCL.
