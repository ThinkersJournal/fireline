# Roadmap

Started 2026-10-28 on CireSnave's ruling (board 295: "Start now. Spin them out immediately."). Order: seam inventory, a one-page extraction
plan with the Fuel, baracuda, Unpopped and KISS lanes, then move the Fuel-to-producer seam crates (fuel-kernel-seam, -types, -announce) first.

## Where each crate stands (2026-11-14, origin/main 7a963ff, workspace 0.0.40)

One crate per interface (CireSnave board 295, 395). "Diffed" means checked against a KISS corpus vector; every other test is written from the clause text and is ours.

| Crate | In tree | On crates.io | What it is | Open |
|---|---|---|---|---|
| `fireline-announce` | 0.1.1 | 0.1.0 | the 56-byte `SeamHello` envelope, role-aware negotiation; the `SAVL`, `CYRQ`, `CRSP`, `CDEC`, `DVRQ`, `DVRS`, `DVDC` frames (6.3, 6.4, 6.6), diffed against `announce_vectors.json` | the frames and device module are in the tree and not published: a release is the PM's call |
| `fireline-grammar` | 0.1.1 | 0.1.0 | the region grammar | none known |
| `fireline-synth` | 0.2.0 | 0.2.0 | the JIT-on-request envelope; requires `unpopped-vocab` 0.45.12 (one copy of the vocabulary) | Baracuda and Vulkane repin to `~0.2` (theirs) |
| `fireline-classify` | 0.1.0 | 0.1.0 | the MX element dtypes (CLASSIFY-6.1-0014..-0016), FDX V4 | if KISS's quantisation group changes those clauses: stop and ask |
| `fireline-contract` | 0.1.0 | not published | the Dispatch expression evaluator and bind-time checks (6.6, 6.12); document framing (6.11); field schemas; Identity, cross-section, typed-value and layout rules (6.3 to 6.9, 6.5); `read_contract`; slot overlap (6.5-0019); diffed against `contract_vectors.json` and `slot_overlap_vectors.json` | see below |
| `fireline-dlpack` | 0.1.0 | not published | FDX, the DLPack extension; cannot retire yet (V3, V8, V9, V12, V13 wait on Fuel's window formula; V5, V6, V18 to V21 have no clause); fixes only | none |
| `fireline-provider` | not started | no | the provider callable and handshake definitions (KISS-Provision, board 395) | held on CireSnave 398 A (US 2023/0025245 A1) |

**`fireline-contract` is not for publishing yet:** the patent sweep has two first passes and still owes a claim-language and CPC pass over the shared-memory layout and swizzle families (`docs/PATENT-SWEEP.md`).

**Waiting on KISS, not on us:**
- `in_place` (6.5-0009): the PM pinned `none | input:<k>` (KISS#813, unmerged); the golden document moves (`len=1280`, 1325 bytes) and `in_place` becomes typed here when it lands.
- Cell class against the named op (6.3-0006), the precision class against the ULP tiers (6.7-0005), the cost expressions (6.7-0006) and the op DAG (6.4-0002, 6.11-0007): these need KISS-Ops tables. KISS#811 (0.16.0) changes window-op OpAttrs.
- Vectors: kiss accepted five requests (a strided document with V11, the five-field Dispatch, `output_layouts` and `human_annotation` present, `in_place` aliasing, one decline per field schema); until they exist those cases are ours.

**Fuel's wave 2 (2026-11-11):** the contract, then the provider, first proof CPU `gemm`, then Vulkan, then AOCL.
