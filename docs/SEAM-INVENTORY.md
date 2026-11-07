# FireLine seam inventory (task 1, read-only)

Written 2026-11-07 by the FireLine lane. Every repo was read at its own `origin/main` after `git fetch` on 2026-11-07
(Fuel `28e0aef6` 0.43.1 per its note, baracuda `c911e9ff` per its note, KISS `a3b48ad`, Unpopped `origin/main` 0.44.22 manifest).
**Verified** = I read the manifest or source at that ref. **PEER** = taken from the owning lane's plan, whose own measurement ref
is given; I did not re-measure it. **UNVERIFIED** = not read, or read only as a file list. Nothing here is a decision.

## A. Seams that are a crate boundary or wire format, in extraction order

| # | Interface | Sides | Home today | Version coupling today | KISS sub-standard | Status |
|---|---|---|---|---|---|---|
| 1 | Kernel-seam handshake (`SeamHello`, `SEAM_MAGIC`, `SEAM_CAP_*`, `negotiate_with_role`) | Fuel <-> baracuda (and any provider) | `fuel-kernel-seam-announce` **and** a second copy in `baracuda-seam` | Fuel workspace version (0.43.x); baracuda copy is `alpha.N`. **Two sources of truth**, held by each side's own size/offset asserts | Announce | PEER (Fuel note, section 1 and 8); both manifests verified |
| 2 | Region/pattern grammar (`OpTag`, `OpAttrs`, `RegionKey`, `PatternNode`, `shape_expr`) | Fuel -> synthesizer | `fuel-kernel-seam-types` (Fuel workspace version); `baracuda-cuda-seam` requires `^0.43` | Caret on a Fuel breaking number: baracuda republishes on every Fuel breaking release | Grammar, Ops 6.20 (`shape_expr`) | Manifest verified; user counts PEER |
| 3 | Synthesizer live call (`Synthesizer`, `Jit*`, `SynthArtifact`, `LinkEntry`) | Fuel <-> baracuda | `fuel-kernel-seam` (Fuel workspace version); implemented by `baracuda-cuda-seam` | same as 2 | Synth/Provision | doc header verified; counts PEER |
| 4 | Region conversion (`to_internal`, `hash_region`, `region_op_id`, `MAX_REGION_DEPTH`) | Fuel grammar <-> Unpopped `PatternNode` | `baracuda-cuda-seam` (nothing CUDA-specific in it) | baracuda alpha line | Grammar | PEER (baracuda half, Fuel note section 8) |
| 5 | Unpopped IR and lowering contract (`ir`, `plan` data, `backend::Backend::lower`, `cfamily`, `kernel`, `exec`, `mma`) | Unpopped core <-> emitters (baracuda-cuda-emit, vulkane slang/spirv/hlsl, fishstringer) | all inside crate `unpopped` | Unpopped 0.44.x; emitters pin `0.43.2` (vulkane-slang-emit verified) | Emit, Consume | PEER (Unpopped#315 at `1dd62715`, not merged); vulkane pin verified |
| 6 | Capability bits and sets (`capset`, `capability`, `TargetId`) | Unpopped <-> emitters; Fuel selection | `unpopped` (imports `artifact`, which blocks a clean cut) | as 5 | Classify 6.8 | PEER (Unpopped#315) |
| 7 | Classifier vocabulary (`ElementKind`, `ArchSku`, `StructureKey`, `OperandDesc`, dispatch tags) | all kernel-side crates | `unpopped-vocab` 0.44.22 (already a leaf crate, no deps); also consumed by `baracuda-kernels-types`, `baracuda-cuda-vocab` | Unpopped number | Classify | Manifest and doc header verified |
| 8 | Namespace vocabularies for `target_capability` (`cuda:`, `vulkan:`) | each backend owner <-> KISS registry | `baracuda-cuda-vocab` (alpha.100), `kiss-vulkan-vocab` (vulkane, 0.4.9, zero deps) | owner project's number | Classify 6.8-0003/0004 | Manifest and doc header verified. **Not in any lane plan so far**: neither is addressed by the Fuel, Unpopped or KISS plans |
| 9 | KISS codecs: structure_key, region wire, contract text, announce, OpAttrs, shape_expr | Fuel, baracuda, Unpopped, KISS | `kiss-conformance` (`publish = false`, KISS repo); private copies in each consumer | none: nothing to depend on | Classify, Grammar, Contract, Announce, Ops | PEER (KISS plan, read at KISS `a3b48ad`) |
| 10 | KISS vocab crates `kiss-classify-vocab`, `kiss-ops-vocab` | lightbulb, Fuel ref backend, others | kiss-ref 0.4.x; lightbulb pins `kiss-ref-core`/`kiss-ops-vocab` `0.4.0` (verified) | kiss-ref number | Classify, Ops | PEER plus lightbulb manifest verified |
| 11 | Conformance suites | each side | `unpopped-conformance` (`publish = false`), KISS `corpus/` + harness, `mlmf-conformance`, `kiss-ref-conformance` | n/a | Conform | names verified, contents not read |

**Duplicate-truth findings (the thing FireLine exists to fix):** (a) `SeamHello` and its capability bits exist twice, and the names diverge:
`SEAM_CAP_FDX_*` vs `SEAM_CAP_DLPACK_EXT_*`, `SEAM_CAP_PROVISION_ON_REQUEST` vs `SEAM_CAP_JIT_ON_REQUEST`, both `1 << 32` (PEER). (b) `shape_expr` has three
codecs (Fuel, KISS ref, whatever baracuda decodes with) (PEER). (c) `baracuda-cuda-seam/tests/seam_pin.rs` asserts caret-only, the opposite of the agreed tilde rule (PEER; the `^0.43` pin itself verified).

## B. Non-KISS interfaces

| Interface | Sides | Home | Notes | Status |
|---|---|---|---|---|
| Model/weight loading (`mlmf-*`: gguf, safetensors, onnx, pt2, ...) | Fuel loaders, lightbulb, ml-convertant <-> mlmf | `mlmf-core` and format crates | `ml-convertant-mlmf` pins `mlmf-* ~0.7.26` (tilde, verified). Fuel's `fuel-loaders`/`fuel-formats` use is **UNVERIFIED** | partial |
| Conversion/quantization (`ml-convertant-core`: "error-measurement contract and conversion trait") | ml-convertant crates <-> callers | `ml-convertant-core` 0.1.x | Interface is a trait + error contract; callers other than `ml-convertant-mlmf` UNVERIFIED | manifest verified |
| OpenAI-compatible HTTP API | lightbulb <-> clients (OverMind) | `lightbulb/src/api/openai/*`; OverMind has `LIGHTBULB-OPENAI-COMPAT-CHECKLIST.md` | a wire format, not a crate; no shared types crate exists. Contents not read | UNVERIFIED (file names only) |
| Latent-state API | lightbulb <-> ? | `lightbulb/src/api/lightbulb.rs`, `cache/*` (guess from names) | I could not identify the contract from names alone | UNVERIFIED |
| Fuel <-> lightbulb model hand-off | lightbulb <-> Fuel | `lightbulb/src/api/fuel_loader.rs` | whether it is a crate dependency or a wire is not established | UNVERIFIED |
| OverMind <-> lightbulb local dispatch | OverMind <-> lightbulb | OverMind `LOCAL-MODEL-DISPATCH-DESIGN.md` (file exists; I did not read it; OverMind has no `Cargo.toml` listed at `origin/main` in my crate search) | design only as far as I can tell | UNVERIFIED |
| Synapse message and client protocol | every lane <-> `synapsed` | `synapse` 6.0.0-rc.220, `synapsed-client`, `synapse-mcp-server`, `synapse-security` | internal to Synapse; clients use the MCP adapter or `synapsed-client`. Candidate only if non-Synapse projects need to share its wire types; **no evidence they do** | manifests verified |
| Vulkane kernel sources (`vulkane-kernels-source`) | vulkane <-> consumers (Fuel) | vulkane 0.21.6 | source files, not an interface crate; Fuel's `SLANG-SEAM-REQUIREMENTS.md` is not yet answered by Unpopped's plan (PEER) | manifest verified |

## C. Boundary calls I am making, to be corrected

1. **Items 4, 8 belong to the extraction plan but are in no lane's plan.** Region conversion (4) was raised by baracuda; the two namespace vocabularies (8) by nobody.
2. **`build_plan`/`derive_pattern` stay in Unpopped** (the Unpopped lane's recommendation (a)); baracuda keeps depending on `unpopped` for them, with a `~` pin.
3. **Item 9's codecs duplicate items 2/3/5 in part** (KISS `grammar.rs` region wire vs `fuel-kernel-seam-types`). The Fuel plan and the KISS plan each propose to own the grammar crate. These two plans conflict until the PM/KISS/Fuel decide.
4. **Version start:** Unpopped proposes `fireline-*` start at 0.1.0 and meet at majors. KISS wants the first release after its 0.9.0 batch. The Fuel plan wants announce/grammar/synth first because it hurts now. These three timings are not compatible; the plan (task 2) must choose.
5. **Capability names (FDX vs DLPACK_EXT; PROVISION vs JIT) and `shape_expr`'s home** are open (Fuel note section 7); they need KISS Announce and KISS Ops.
6. **Conformance addition from baracuda (PEER, notice 2026-11-07, baracuda main `8bf6705b`):** every capability bit a side *advertises* must be allocated in the shared crate, none provisional.
   Today baracuda-seam defines bits 34 (`KISC_FRAMING`) and 35 (`RECIPE_IMPORT`) but `BARACUDA_CAPABILITIES` omits them; the check keeps it so until KISS Announce co-allocates them. Bits to freeze now: 0-5, 32, 33 (Fuel note section 8).

## D. Not measured

- crates.io reverse dependencies of `fuel-kernel-seam`, `-types`, `-announce` (the Fuel lane asked; **not queried**, to be done in task 2 with a positive control).
- Every item marked UNVERIFIED above, notably lightbulb's and OverMind's APIs, mlmf's public surface, and whether fishstringer will import anything.
- No lane has yet stated its must-haves directly to me; the three plans above are their written input. Ask lightbulb, OverMind, mlmf, ml-convertant, vulkane and fishstringer (task 1 requires it).
- Counts quoted from the Fuel note (4,593 lines; 31/6 `.rs` users; 37 files to rewrite) were not recomputed.
