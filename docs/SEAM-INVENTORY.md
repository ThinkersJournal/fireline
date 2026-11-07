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
| ~~Latent-state API~~ | withdrawn | no document or lane identifies one (see C2, C3) | | withdrawn |
| Fuel <-> lightbulb model hand-off | lightbulb -> Fuel | `lightbulb/src/api/fuel_loader.rs` | **internal to lightbulb** per lightbulb (C2); Fuel guarantees `from_gguf`, not a FireLine interface | lane-reported |
| OverMind <-> lightbulb local dispatch | OverMind -> lightbulb | OpenAI-compatible HTTP; OverMind `LIGHTBULB-OPENAI-COMPAT-CHECKLIST.md` | wire format, Python consumer, no Rust crate; see C3 | read at `2920f08` |
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

## C2. Lane replies to the must-have ask (received 2026-11-07; the lane's own measurement, not re-measured by me)

- **lightbulb** (at `9dda603`): the only wire format it shares outward is the OpenAI-compatible HTTP API (`src/api/openai/{chat,completions,models}.rs`, `model_select.rs`; `/v1/models` adds `lightbulb_loaded`/`lightbulb_device`). It is a binary with no library API. `fuel_loader.rs` is **internal** to lightbulb. It found **no latent-state API**: the row in section B is withdrawn unless someone points at a file and symbol. It needs Fuel's `from_gguf` and decode-step plan-once (GAP-359, awaiting release) and mlmf's sharded-GGUF reader (B5) and source-hub listing/resume (F1): guarantees from Fuel and mlmf, **not FireLine interfaces**.
- **vulkane** (origin/main 2026-11-07, grep with control): uses **no** Fuel kernel-seam crate and no Provision/announce; consumes only `unpopped`/`unpopped-vocab` 0.43.2 (+ `unpopped-cpu-c` dev). `kiss-vulkan-vocab` **stays** in vulkane (KISS-Classify 6.8 assigns the `vulkan:` namespace to Vulkane; moving it is a PM/KISS ruling). Its must-haves from the Unpopped crates: `Backend`/`KernelPlan`/`GeneratedKernel`/`DtypeSupport`/`LowerError`, `try_generate`, `OpDef` + `BinaryOp`/`UnaryOp`/`Expr`, `StructureKey`/`ElementKind`/`OperandDesc`, `Frontend` + `lift_*` with the `convert` feature, `unpopped::cfamily`; a breaking change breaks 5 emitter/parser crates at once, so it wants notice and a version. Note `Frontend`/`lift_*`/`convert` are in Unpopped's "stays" list (Unpopped#315 section 3): a conflict for task 2.
- **mlmf** (`de8de26`; grep of `mlmf_core::` over **working trees**, not origin/main; partial): cross-project set is `DType`, `Encoding`, `MetaValue`, `MetadataSource`, `TensorContainer`, `ByteSource`, `Report`/`UnrecognizedKind`/`DeclaredType`, all re-exported from `mlmf-core/src/lib.rs`. Consumers: Fuel (`fuel-loaders/src/quantized/gguf_file.rs`, `safetensors.rs`) and `ml-convertant-mlmf`. lightbulb: 0 hits (its `src/gguf` is its own reader). **These already live in one published crate (`mlmf-core`)**, so mlmf is not a FireLine candidate unless a second crate is wanted: mlmf's own judgement, mine to rule on in task 2.
- **fishstringer** (vulkane `9cdc2d4` `src/lib.rs`, read not built): `fishstringer-slang-emit` imports `unpopped::backend::{Backend, Decline, DeclinedOp, DtypeSupport, GeneratedKernel, LowerError, Lowering, Spelling, const_lit, lower_dag}`, `cfamily::{assert_no_int_div_or_const, dtype_tag}`, `ir::{BinaryOp, ExprDag, ScalarExpr, UnaryOp}`, `plan::{KernelPlan, Schedule}`, `unpopped_vocab::{ElementKind, TargetId}`; `fishstringer-slang-parse` imports `unpopped::convert::Frontend` (+ `lift*`, `LiftError`; needs the `convert` feature and tree-sitter on the same minor as unpopped). It wants semver-stable `Backend`/`Lowering`/`Frontend` and the `lift_*` fns, and unpopped/unpopped-vocab/cpu-c moving together. **This supersedes Unpopped#315's "no fishstringer import found" for the emitter and parser crates.** Both it and vulkane import `convert::Frontend`/`lift*`, which Unpopped#315 section 3 keeps in Unpopped: task 2 must decide.

## C3. OverMind (read at `ciresnave/OverMind` origin/main `2920f08`; design documents, not a crate)

OverMind is a **consumer of a wire format**, not a crate owner, as the PM said. What the two documents settle:
- `LIGHTBULB-OPENAI-COMPAT-CHECKLIST.md` lists what OverMind needs from an OpenAI-compatible endpoint, read from its code (`providers.py`, `agent.py`, `gate.py`): MUST accept `POST {base_url}/chat/completions` (line 25), request fields `model`, `messages`, `temperature`, `max_tokens` (lines 36-39, `tools` optional line 40); response `choices[0].message` MUST be present (line 48), `content`, `tool_calls`, `finish_reason`, `usage` OPTIONAL (lines 49-52); `Authorization: Bearer` must be tolerated (line 58); no streaming, no `stream` field (lines 28, 42); native `tool_calls` strongly preferred (line 77). The document's own header (line 3) says line numbers predate #151 and that it is not recorded whether Lightbulb scored itself against it.
- That is a **request/response shape**, already carried by lightbulb's `src/api/openai/*` and nothing shared by type. No Rust type crosses this boundary (OverMind is Python), so **no FireLine crate applies**; the only candidate is a conformance corpus (golden request/response pairs run against lightbulb). Not a decision.
- `LOCAL-MODEL-DISPATCH-DESIGN.md` is design: `Provider.local`, the `rig` provider and `probe/rig_probe.py` landed in #151 (lines 3, 190-198); the rest, including the context window and whether Lightbulb serves a real checkpoint end to end, is explicitly **not settled** (lines 151-186). I mark those UNVERIFIED. The "latent-state API" I had listed under lightbulb is **not mentioned in either OverMind document** (checked by reading both whole); lightbulb also found none. **The row is withdrawn.**

## C4. crates.io reverse dependencies (queried 2026-11-07 ~12:10Z)

Query: `GET https://crates.io/api/v1/crates/<crate>/reverse_dependencies?per_page=100`, one page each (the largest result was 27, under the 100 limit, so each count is complete). Positive control: the same query for `fuel-ir` returns 27 dependents, so the query shape works.
| Crate | Published reverse dependencies |
|---|---|
| `fuel-kernel-seam-announce` | **0** |
| `fuel-kernel-seam` | 3: `baracuda-cuda-seam` (`^0.43`), `baracuda-kernelgen` (`^0.10.3`), `fuel-dispatch` (`^0.36.7`) |
| `fuel-kernel-seam-types` | 6: the same three, plus `fuel-graph`, `fuel-kiss-ref-backend` (`^0.36.7`), `fuel-kernel-seam` (`^0.43.0`) |
No dependent outside Fuel and baracuda. The recorded requirements are those of each dependent's **newest published version**, and several still name old Fuel numbers (`^0.36.7`): this counts crates.io dependents only, not unpublished repos (vulkane showed none by its own grep).

## C5. Still open
- OverMind: no lane to ask; documents read above.

## D. Not measured

- Whether OverMind's Python code still matches its checklist (line numbers predate #151).
- Every item marked UNVERIFIED above, notably Fuel's `fuel-loaders` use of mlmf at `origin/main` (mlmf's grep was of working trees), and ml-convertant's own must-haves (not asked: no live lane found for it).
- The three PEER plans' claims remain theirs; must-haves were asked of lightbulb, mlmf, vulkane and fishstringer and answered (C2).
- Counts quoted from the Fuel note (4,593 lines; 31/6 `.rs` users; 37 files to rewrite) were not recomputed.
