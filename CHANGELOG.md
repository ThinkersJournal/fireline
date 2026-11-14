# Changelog

## [0.2.5] — patch

Patent sweep: fifth pass for `fireline-contract` (docs only): 14 more patents read on their independent claims (memory shapes, DMA descriptors, strength reduction, sparse matrix, tensor memory access); none needs review. Does not clear the crate for publishing.

## [0.2.4] — patent sweep, `fireline-contract` fourth pass (slot-allocation families); no-attorney wording

`docs/PATENT-SWEEP.md` gains the fourth pass: the independent claims of the four slot-allocation families the third pass had not read (US 12,430,110 B1, US 12,423,007 B1, US 11,720,993 B2, US 12,541,813 B2) and three older unread numbers (US 7,581,141 B2, US 10,242,196 B2, US 8,433,927 B2). All seven are clear on their independent claims; the only note is that US 12,423,007 B1 claims lifetime-based reuse of tensor memory, which the slot check is the opposite of, so it is re-read first if slots ever share bytes. The third pass's two "hand to a lawyer" items are rewritten as the vocabulary question and a self-serve path (keep the crate's own terms and cite the source), because there is no attorney and none is planned. **This does not clear `fireline-contract` for publishing**; the PM puts that verdict to CireSnave. No code change.

## [0.2.3] — patent sweep, `fireline-contract` third pass (layout and swizzle families)

`docs/PATENT-SWEEP.md` gains the claim-language pass owed for ideas 9 to 11 (the seven output-layout forms with nested `shape:stride` and a swizzle, named output slots, the slot byte-hull overlap check): nine claim-phrased queries (Q20 to Q28; the search tool cannot filter by CPC or claim text, and four queries returned only papers) and fourteen patents read on their independent claims, including those the second pass had surfaced and not read. Every one reads on a processor, memory controller, DMA or DRAM circuit or an executed memory access, which the crate is not, so none is `needs-review`; two (US 2012/0092356 A1 and WO 2020/263588 A1) are flagged for a lawyer on claim vocabulary. **This does not clear `fireline-contract` for publishing**: dependent claims and descriptions were not read, the slot-allocation patent families (US 12,430,110, US 12,423,007, US 11,720,993, US 12,541,813) are still unread, and no layout-algebra patent surfaced, which is not evidence that none exists. The PM decides. No code change.

## [0.2.2] — diff against KISS `section_vectors` (6 accept docs, 50 declines)

New test `tests/kiss_section_vectors.rs` with its data file `tests/data/kiss_section_vectors.tsv` (the `section_vectors` of `contract_vectors.json` on kiss origin/main 2c1e2cfa, KISS#818, 0.17.4): every accept document reads cleanly through `read_contract`, and all 49 declines are refused by the layer and variant kiss's tag names (all seven section schemas, the five-field Dispatch, V11's class-2-declared-scalar, `in_place` spellings, `output_layouts` and `human_annotation` order). kiss dropped its own extra `in-place-no-such-input` after our finding. No library change.

## [0.2.1] — `Signature::check_complete`: the positional_signature binds without outside information (6.5-0002)

`Signature::check_complete(operands)` checks a parsed `positional_signature` against the operand count: exactly `operands` typed `ptr` descriptors, all before any launch scalar; `extents{i}` for every operand; `n` exactly once; no per-operand scalar past the operand list; `off{i}` for every operand if any (6.5-0006); `ws_ptr` with `ws_bytes` or neither. New `SignatureError` variants carry each decline. Whether a pointer's dtype is its operand's own needs the op's operand list (KISS-Ops) and is not checked: only that a dtype is declared. No vector exists for this clause (the golden's signature is empty): the tests are this crate's own and the `off{i}` all-operands rule is this crate's reading. Verified with `cargo test`, `clippy -D warnings`, `cargo doc -D warnings`.

## [0.2.0] — **BREAKING**: interface crates unified at 0.2.0

Every interface crate now carries the one project number (CireSnave's one-number-per-project rule; the portfolio sweep's version-consistency finding at a 0.x project number with x >= 1, which the 0.1.0 allocation for #44 crossed and turned main's CI red). `fireline-announce` and `fireline-grammar` move from their unpublished tree version 0.1.1 to 0.2.0 (`fireline-synth` was already published at 0.2.0); the workspace and `fireline-classify`, `fireline-contract`, `fireline-dlpack` move from 0.1.0; `fireline-synth` requires `fireline-grammar` as `~0.2`; the README require-lines read `~0.2`. No source change; carried by the lane at the PM's ruling (a PR that bumps a version cannot also carry a fragment). Verified with `cargo build --locked --all-targets` (no `collision`) and `cargo test --locked`.

— lane: fireline

## [0.1.0] — **BREAKING** (unpublished `fireline-contract`): `in_place` typed, golden re-diffed to KISS 0.17.1

KISS#813 pinned Contract 6.5-0009: `in_place` is exactly `none` or `input:<k>` (`k` a 0-based input index, canonical decimal). `values::InterfaceValues.in_place` was text and is now `InPlace::{None, Input(u32)}`; `interface_values` declines anything else, `false` included, with the new `ValueError::InPlace`. `InterfaceValues` lost its lifetime. The golden document of `contract_vectors.json` (now `in_place = none`, `len=1280 crc32=79f503e4`, 1325 bytes) is re-pasted in the six test files and passes. The `none`, `input:0/7/10` and decline cases (`false`, `input:`, `input:-1`, `input:01`, `input:x`, `Input:0`, `None`, overflow) are this crate's own: the corpus has one value. Verified with `cargo test`, `clippy -D warnings` and `cargo doc -D warnings`.

## [0.0.42] — `fireline-contract` bind-time rank check of an `Environment`'s arrays (KISS-CONTRACT 6.5-0012)

New `Environment::check_rank(rank)` (additive): every per-operand `extents{i}` and `strides{i}` array the environment assigns must have exactly the contract's declared `rank` entries (6.5-0012: "every per-operand `extents` / `strides` array length MUST equal `rank`"); a mismatch is the new `BindDecline::ArrayLength { symbol, rank, len }` (code `array_length`), the first in symbol order. A scalar such as `off{i}` is one value and is not sized from `rank`, and `idx_extents{i}` is sized by the index operand's own rank (6.5-0013), so neither is checked here. This is the bind-time half of the rank rule: with a declared rank, `Expr::parse_with_rank` and `parse_in` already refuse an out-of-bounds subscript at build time, and an environment that passes this check can no longer reach `BindDecline::SubscriptOutOfRange` through a rank-checked expression. No KISS vector covers it (the corpus has no second value of any field and no bind environment for this clause), so the new test is written from the clause text and is this crate's own. Draft, unfrozen: expect rework. Verified with `cargo test -p fireline-contract --locked`, workspace clippy `-D warnings`, fmt and doc `-D warnings`.

— lane: fireline

## [0.0.41] — ROADMAP: where each crate stands

`ROADMAP.md`, which had one paragraph and no rows, gains a status table: each crate's in-tree and crates.io versions (read from `Cargo.toml` and the crates.io API on 2026-11-14), what it is, and what is open; the reason `fireline-contract` is not for publishing yet (a claim-language and CPC patent pass is owed); what waits on KISS (the `in_place` pin in KISS#813, the clauses that need KISS-Ops tables, the vector requests kiss accepted); and Fuel's wave-2 order. Documentation only; no code change.

— lane: fireline

## [0.0.40] — `fireline-contract` `read_contract`: one call over every layer

New `fireline_contract::read::read_contract(bytes, supported_versions)` (additive) runs every layer of the crate over a document's bytes and stops at the first fault, naming the layer in `ReadError` (`Document`, `Fields`, `Identity`, `Consistency`, `Values`, `Signature`): the framing (`document`), the field schemas (`fields`), the Identity rules (`identity`), the cross-section rules and vocabularies (`consistency`), the typed Interface scalars and the `launch_scalars` tail (`values`), the `positional_signature` (`interface`) and the Dispatch expressions in the contract's own vocabulary and rank. It returns `Checked` (the document, the `Contract` fields, the typed `InterfaceValues`, the parsed `Signature`, and the parse `Context`, usable straight away with `Expr::parse_in`). It adds no rule of its own. Not run (each needs input this call lacks): the strides rule (needs whether the cell is packed and the operand count), `output_layouts` (needs the output-operand count and the Classify dtype table), bind-time checks, and anything needing KISS-Ops tables. The golden document of KISS `contract_vectors.json` (origin/main 73c341b5) passes end to end; the layer-naming tests (2 more) are written from the module docs and are this crate's own. The crate README's stale "first slice only" bullet is replaced by what is actually here. Draft, unfrozen: expect rework. Verified with `cargo test -p fireline-contract --locked`, workspace clippy `-D warnings`, fmt and doc `-D warnings`.

— lane: fireline

## [0.0.39] — `fireline-contract` cross-section rules and closed vocabularies (KISS-CONTRACT 6.7, 6.8, 6.9)

New `fireline_contract::consistency::check_consistency(&Contract)` (additive), over a `Contract` whose field schemas `parse_contract` checked: the copies that must agree byte for byte (Capabilities `accept_predicate` with Identity's, 6.7-0002; Provenance `revision_hash` with Identity's, 6.9-0003; Capabilities `determinism_class` with Guarantees', 6.7-0004; Provenance `cost_provenance` with Guarantees', 6.9-0006); the closed vocabularies (`determinism_class` as KISS-Ops spells it, 6.8-0003; `math_fidelity` as `bit-stable` or `reduced-mantissa-permitted:<n>` with `n` a canonical decimal 1 to 52 and the bare spelling declined, 6.8-0004a; `bit_stability` in `{portable, same-hardware, none}` with the rule that `order-invariant/nondeterministic` never declares `portable`, 6.8-0013; `cost_provenance` in `{declared, measured}`, 6.8-0006; `index_width`, 6.7-0003); the opaque-blob encoding `<n>:<hex>` (exactly 2n lowercase hex digits; `0:` is empty) of `revision_hash` and `negotiation_metadata` (6.11-0001, 6.9-0008); and the `audited_status` derivation (6.8-0008..-0010). Typed `ConsistencyError` (non-exhaustive). The golden document of KISS `contract_vectors.json` (origin/main 73c341b5) passes; it has one value per field, so the other 7 tests are written from the clause text and are this crate's own. **Two readings stated:** 6.7-0003 says `index_width` is one of `{idx32, idx64}` but the golden document writes `ix32` (the `structure_key` code), so `ix32` and `ix64` are accepted (reported to kiss); and "a bounded precision against a named `reference_function`" (6.8-0009/-0010) is read as a non-empty `reference_function` and a non-empty `per_backend_ulp_tiers` array. Not here (needs KISS-Ops tables): the precision-class set and its agreement with the ULP tiers (6.7-0005), the tiers' contents, and the cost expressions (6.7-0006, 6.8-0007). Draft, unfrozen: expect rework. Verified with `cargo test -p fireline-contract --locked`, workspace clippy `-D warnings`, fmt and doc `-D warnings`.

— lane: fireline

## [0.0.38] — `fireline-contract` typed Interface scalars, launch_scalars tail and Dispatch expression check (KISS-CONTRACT 6.5-0008..-0012, 6.11-0008)

New `fireline_contract::values` module (additive), over a `Contract` whose field schemas `parse_contract` checked: `interface_values` reads `rank` (6.5-0012: a canonical non-negative decimal), `count_unit` (6.5-0008: `elements` or `vectors_x<w>` with no delimiter and a canonical positive width) and `alignment_bytes` (6.5-0010: a canonical positive decimal) into `InterfaceValues`; `check_launch_scalars` checks that `launch_scalars` equals, entry for entry, the launch-scalar tail of the `positional_signature` (6.5-0011; both parsed with the `interface` module, so descriptors and the pinned class order are checked too); `check_dispatch` checks that each Dispatch geometry field is an expression the Dispatch grammar accepts over the contract's own vocabulary and rank (6.11-0008, via `Expr::parse_in`). Typed `ValueError` (non-exhaustive). **`in_place` is returned as text, not interpreted:** 6.5-0009 says it names the input operand the output aliases or none, but the Draft pins no spelling: the golden document of KISS `contract_vectors.json` writes `in_place = false`, the spec's own example text says `in_place = none`, and no spelling exists for "aliases input i" (reported to kiss). The golden document passes (rank 2, `elements`, alignment 1, empty signature and launch scalars); it carries one value per field, so the other 6 tests are written from the clause text and are this crate's own. Not here: the rank-length check of an `Environment`'s arrays at bind time (6.5-0012), and `positional_signature` completeness against the operands (6.5-0002, needs the operand list). Draft, unfrozen: expect rework. Verified with `cargo test -p fireline-contract --locked`, workspace clippy `-D warnings`, fmt and doc `-D warnings`.

— lane: fireline

## [0.0.37] — `fireline-contract` Identity section rules (KISS-CONTRACT 6.3-0002, -0007, -0009, -0011)

New `fireline_contract::identity::check_identity(&Contract)` (additive), for a contract whose field schemas `parse_contract` already checked: the Identity `contract_kind` and `contract_version` equal the transport header's byte for byte, the header authoritative (6.3-0009); the `accept_predicate` is a printable, LF-free `structure_key` token (`sk<digits>|<class>|<dtype>|<target>|...`, 6.3-0002); the `target_capability` is a namespaced `<namespace>:<capability-set>` descriptor with no space or control character (6.3-0007); and the `accept_predicate`'s target component, its fourth `|`-separated component (true of every `structure_key` of KISS `structure_key_vectors.json`, `sk3` to `sk9`), equals the `target_capability` byte for byte, with no ordering, prefix or case folding (6.3-0011). Typed `IdentityError` (non-exhaustive). The golden document of KISS `contract_vectors.json` (origin/main 73c341b5) passes; it carries one value per Identity field, so the 5 fault tests are written from the clause text and are this crate's own. Not here: the cell-class-versus-op consistency (6.3-0006), which needs KISS-Ops' structural facts per op, and the `op_identity` forms (6.3-0003, -0004, KISS-Grammar tags). One edge stated: `Document` keeps the header's version as a number, not its text, so the check compares the Identity copy to the canonical decimal of that number: a header and an Identity copy that both spell the version `04` would be rejected here though they are byte-equal. Draft, unfrozen: expect rework. Verified with `cargo test -p fireline-contract --locked`, workspace clippy `-D warnings`, fmt and doc `-D warnings`.

— lane: fireline

## [0.0.36] — **BREAKING**: `fireline-synth` requires `unpopped-vocab` 0.45.12 (was 0.43.2)

`fireline-synth` now depends on `unpopped-vocab` 0.45.12, the newest published (crates.io, read 2026-11-13). The published `fireline-synth` 0.1.0 pins 0.43.2, so a consumer that has moved to Unpopped 0.45.12 builds two copies of the vocabulary and `OperandDesc` (and `ArchSku`, `TargetId`, `ElementKind`) are then two different types at the seam; this repin gives one copy. **Why BREAKING:** the vocabulary types appear in `fireline-synth`'s public signatures (`JitRequest` and the `Synthesizer` trait use `OperandDesc`, `ArchSku` and `TargetId`), so their type identity moves with the pin: a consumer on `unpopped-vocab` 0.43.x can no longer pass them to this version. Pre-1.0 that is the second number: release as **`fireline-synth` 0.2.0** (the PM allocates; the crate version is set to 0.2.0 here at the PM's request, so the PM can publish 0.2.0 from the merge SHA and tag `fireline-synth-v0.2.0`; no other in-tree crate depends on `fireline-synth`). No source change was needed: the workspace builds, `cargo test --workspace --locked` passes (3 synth tests, the `synth_hello_world` example runs), clippy `-D warnings` and `cargo doc` are clean, there is no example-name collision, and `cargo tree -i unpopped-vocab` shows the single copy 0.45.12. `Cargo.lock` changes only the `unpopped-vocab` entry (version and checksum). The stale "requires 0.43.2" line in `docs/PUBLISHING.md` is updated. Not verified: whether Unpopped's own types changed shape between 0.43.2 and 0.45.12 (the build and tests passing is the check made; no consumer was built here).

— lane: fireline

## [0.0.35] — `fireline-contract` section field lines and field schemas (KISS-CONTRACT 6.11-0001, -0005, 6.3-0001..6.9-0001)

New `fireline_contract::fields` module (additive): `parse_contract` takes the seven sections a framed `Document` split out (the `document` module) and reads each section's field lines (`<key> = <value>`, one LF-terminated UTF-8 line, one space either side of the `=`, 6.11-0001) and checks each against its field schema, returning `Contract { contract_version, sections }` as ordered `key`/`value` pairs: the Identity, Semantics (with `human_annotation` optional and last), Interface (with `output_layouts` optional and last), Capabilities, Guarantees and Provenance schemas (exact keys, exact order, nothing missing, unknown, repeated or out of place, 6.1-0006), and the Dispatch section as exactly one of its two alternatives (the five geometry fields or the single `dispatch_model = geometry-agnostic` line, 6.6-0001). Typed `FieldError` (non-exhaustive). Values stay text: the typed parsers of the other modules (`interface`, `layout`, `Expr`) read them. **Diffed against KISS `contract_vectors.json` `golden_document` (origin/main 73c341b5):** the golden document's seven sections parse with exactly the schemas' keys in order and the values spot-checked; that document carries one value per field, so every fault case (6 more tests) is written from the clause text and is this crate's own (kiss confirmed 2026-11-13 that no corpus vector has a second value of any field). Not here: per-field value encodings and meanings (6.3-0002 onwards), and the cross-section consistency clauses. Draft, unfrozen: expect rework. Verified with `cargo test -p fireline-contract --locked`, workspace clippy `-D warnings`, fmt and doc `-D warnings`.

— lane: fireline

## [0.0.34] — patent sweep, second pass, for `fireline-contract`

`docs/PATENT-SWEEP.md` gains a second `fireline-contract` addendum covering what the crate gained since the first (document framing, the Interface signature and its pinned launch-scalar order, the parse context, the seven output-layout forms with the CuTe-style nested layout and swizzle spelling, and the bind-time slot-overlap byte-hull check): five ideas, four queries, five patents read (NVIDIA US 2023/0289398 A1, US 2019/0278593 A1 and US 11,080,051 B2, AMD US 10,067,710 B2, Intel US 2024/0104013 A1), all read as clear (none reads on a claim). It records provenance (no CuTe code or text is copied and none of CuTe's layout algebra is implemented; the CRC-32 is written from its definition; the overlap check follows KISS's own MIT OR Apache-2.0 implementation and vectors) and what stays owed: a claim-language and CPC-class pass over the shared-memory layout and swizzle families, so `fireline-contract` is still not cleared for publishing by this alone. Documentation only; no code change.

— lane: fireline

## [0.0.33] — `fireline-contract` bind-time slot overlap (KISS-CONTRACT 6.5-0019, 6.12-0002, FDX V10)

New `fireline_contract::layout::check_slot_overlap(slots, env, bits)` (additive): the bind-time check of a bundle's slots. A slot's byte hull is `[offset + its lowest element, offset + its highest element + 1)`; a sub-byte dtype rounds the hull outward, a swizzle rounds its end up to the `2^(B+M)` window, a zero extent is an empty slot that overlaps nothing, and two slots overlap when their hulls intersect (conservative: interleaved slots are refused, as KISS pins for v1). The first overlapping pair in declaration order is named in the new `BindDecline::SlotOverlap` (code `slot_overlap`); the new `SlotDtypeWidth` and `SlotExpression` declines cover a dtype width the caller's table lacks and a hand-built slot with an unparseable expression. New `Expr::bind_signed` binds without the extent gate, for strides (negative = reversed). Arithmetic is checked (`AffineOverflow`), never wrapping. The dtype width comes from a caller-supplied `bits` function (the Classify table is not copied). **Byte-diffed against KISS `conformance/corpus/slot_overlap_vectors.json` (origin/main 73c341b5):** all six cases give the corpus's verdict, including the pinned conservative interleaved refusal; 6 more tests are mine (the hull arithmetic the vectors do not reach: sub-byte widths, swizzle windows, reversed strides, empty slots, declaration order, unbound symbols). Built on the layout module (#33). Draft, unfrozen: expect rework. Verified with `cargo test -p fireline-contract --locked`, workspace clippy `-D warnings`, fmt and doc `-D warnings`.

— lane: fireline

## [0.0.32] — `fireline-contract` output layout forms (KISS-CONTRACT 6.5-0014..-0019)

New `fireline_contract::layout` module (additive): `parse_layout` and `parse_output_layouts` for the Interface `output_layouts` field: the seven closed forms (`dense`, `dense_order(..)`, `same_as(i)`, `strided(..)`, a nested layout `shape:stride` with an optional `swz(B,M,S)`, `bundle(slot,..)`, `none`) plus the per-output `-` ("no guarantee: the layout the caller supplied"), with the canonical-text rules (no space, canonical integers), the permutation, input-index and rank checks of 6.5-0016, the nesting rules of 6.5-0017 (congruent profiles, shape leaves at least 1, stride leaves at least 0, depth at most 16) and 6.5-0018, and the bundle slot checks of 6.5-0019 (unique `[a-z0-9_]+` names, a Classify dtype via a caller predicate, a slot layout other than bundle and none whose stated rank equals the shape's, stride and slot expressions parsed in the contract's vocabulary with `Expr::parse_in`). An unknown form refuses only that output's guarantee (read as `none`, the head recorded), not the contract (6.5-0015); a count mismatch declines with no padding or truncating (6.5-0014). `Layout` re-spells to the canonical text (`Display`). **Byte-diffed against KISS `conformance/corpus/contract_vectors.json` `output_layout_vectors` (origin/main 73c341b5): all nine Interface blocks (supplied, dense, dense_order, same_as, strided, nested, nested_swizzled, bundle, none) parse and re-spell identically.** Not here: the bind-time `slot_overlap` check (6.5-0019, 6.12-0002; V10, next) and `byte hull` arithmetic. Built on `Context` (#32). Draft, unfrozen: expect rework. Verified with `cargo test -p fireline-contract --locked` (7 new tests), workspace clippy `-D warnings`, fmt and doc `-D warnings`.

Tag note (PM 2026-11-13): the repo tag `v0.1.0` is the 2026-11-08 publish set (`fireline-announce`, `fireline-grammar`, `fireline-synth` 0.1.0), tagged on the `fireline-dlpack` 0.1.0 commit c623a8f; the publish commits cannot be recovered from the `.crate` files (they carry no `.cargo_vcs_info.json`). Tags are crate-scoped, `<crate>-v<version>`, from now on (`fireline-classify-v0.1.0` is the first).

— lane: fireline

## [0.0.31] — `fireline-contract` parse in the contract's vocabulary and rank (KISS-CONTRACT 6.5-0004a, 6.6-0006)

New `fireline_contract::Context` and `Expr::parse_in(text, &Context)` (additive; `parse` and `parse_with_rank` are unchanged): a context carries the contract's declared rank and its own launch-scalar vocabulary, `Context::of_interface(rank, &Signature)` builds both from the Interface's `positional_signature` (the `interface` module of #30). A symbol of the standard's classes that the contract's Interface does not declare (`strides0` in a fully packed cell, `ws_bytes` without a workspace pointer) is outside that contract's vocabulary and is a build-time `ParseError::UnknownSymbol` (6.5-0004a, 6.6-0006: "MUST NOT reference a symbol absent from the vocabulary"); the vocabulary is checked before the subscript, and a subscript at or past the declared rank is `SubscriptOutOfBounds` (the rank rule of the previous slice). An unassigned declared symbol is still a bind-time `unbound_symbol`. No KISS vector covers this: the tests (4 new) are written from the clause text and are this crate's own. Verified with `cargo test -p fireline-contract --locked`, workspace clippy `-D warnings`, fmt and doc `-D warnings`. Draft, unfrozen: expect rework.

— lane: fireline

## [0.0.30] — patent sweep, first pass, for `fireline-classify`

`docs/PATENT-SWEEP.md` gains an addendum for `fireline-classify` 0.1.0 (the MX element dtypes `f4e2m1`, `f6e2m3`, `f6e3m2`, their packing and the never-an-accumulator rule), on the PM's task: five ideas, three queries, four patents read (NVIDIA US 2024/0160406 A1, Matx US 12,632,219 B2, NVIDIA US 2025/0291873 A1, Intel US 8,280,936 B2), all read as clear (none reads on a claim). It records provenance with the licence READ (PM follow-up 2026-11-13): the value rule and constants come from the OCP Microscaling Formats Specification v1.0 (format numbers and a formula only, no text or code; v1.1 read and its three element tables are identical); the 6-bit packing is the Unpopped lane's convention (neither OCP version specifies packing); the Open Web Foundation Modified Final Specification Agreement (OWFa 1.0) template is quoted (copyright grant 2.1, attribution 2.2, patent promise 3.1.1, licence commitment 3.2, termination 3.1.2.1, Permitted Uses 8.6, Granted Claims 8.4), the credit it asks for is added to the crate README and root docs, and the reading is that the patent promise probably does not cover this crate (it is not an implementation of all required portions of the Specification), so patent cover rests on the sweep alone; no executed copy was seen. A first pass: it does not clear the crate for publishing on its own. Documentation only; no code change.

— lane: fireline

## [0.0.29] — `fireline-contract` positional_signature, launch-scalar order and strides rule (KISS-CONTRACT 6.11-0006, 6.5-0004a/c/d, 6.5-0005, FDX V11)

New `fireline_contract::interface` module (additive): `Signature::parse` reads the Interface `positional_signature` value (6.11-0006: `(<arg_kind>, <launch_class>, <arg_name>, <type_token>, <dtype>, <array_len_kind>)`, the `idesc` index descriptor with its `<rank>@<binding>` slot) and checks the pinned launch-scalar class order, grouping and operand order (6.5-0004a, -0004c, -0004d) and that an `idx_extents{i}` follows the descriptor that binds it (6.5-0013); `Signature::check_strides(operands, fully_packed)` is the V11 rule (6.5-0005: a cell that is not fully packed carries `strides{i}` for every operand; a fully packed one carries none, class 2 being present exactly when the cell is not fully packed); `Signature::vocabulary()` returns the contract's own launch-scalar symbol set (6.5-0004a). Typed `SignatureError` (non-exhaustive). **No vectors:** KISS's `contract_vectors.json` has an empty signature in its golden document and nothing for these clauses (kiss, 2026-11-13), so the tests are written from the clause text and are this crate's own; two readings are this crate's choice and are documented (operand pointers are accepted anywhere among the scalars; canonical operand order is ascending `{i}`). Not here: evaluating "fully packed" (KISS-Classify's contiguous-layout predicate, the caller's), the other Interface fields, and wiring the vocabulary into `Expr::parse`. Draft, unfrozen: expect rework. Verified with `cargo test -p fireline-contract --locked` (7 new tests), workspace clippy `-D warnings`, fmt and doc `-D warnings`.

— lane: fireline

## [0.0.28] — `fireline-contract` out-of-bounds subscript is a build-time fault when the rank is declared

Follows KISS (kiss 2026-11-12, origin/main 73c341b5): an out-of-bounds subscript is reported at **build** time, because the rank is declared by the contract and not by the launch (`Decline::SubscriptOutOfBounds`, no code name yet). New `Expr::parse_with_rank(text, rank)` rejects `sym[k]` with `k >= rank` as the new `ParseError::SubscriptOutOfBounds { symbol, index, rank }` (code `build_time`, 6.12-0002). `Expr::parse` is unchanged (no rank known), so the bind-time `subscript_out_of_range` decline remains for an expression parsed without a rank and for an environment array shorter than the declared rank. Additive apart from the new `ParseError` variant (the crate is unpublished). Verified with `cargo test -p fireline-contract --locked` (a new test covers declared-rank, rank 0, unchanged `parse`, and a short environment array), workspace clippy `-D warnings`, fmt and doc `-D warnings`. Draft, unfrozen: the decline's name is KISS's to give.

— lane: fireline

## [0.0.27] — new crate `fireline-classify` 0.1.0: MX element dtypes (KISS-CLASSIFY 6.1-0014..-0016, FDX V4)

New interface crate `fireline-classify` (PM-ruled 2026-11-12: one crate per interface; the name was verified free on crates.io and claimed at 0.0.0 by the PM). It versions by its own interface, so no root bump. Scope is CLASSIFY-6.1-0014..-0016 only: `MxElement` (`f4e2m1`, `f6e2m3`, `f6e3m2`) with the value rule (`decode`: exact in f64, no infinity, no NaN, `-0` a code of its own), the byte packing (`pack` and `unpack`: two per byte low nibble first, as `i4`/`u4`; four per three bytes least-significant bit first, `ceil(6n/8)` bytes, nonzero trailing bits declined) and `check_accumulator` (an MX element in an accumulator position is `element_dtype_as_accumulator`, KISS's `ElementDtypeAsAccumulator`, distinct from the unknown and reserved token declines). This maps FDX V4 to its KISS clauses and keeps it out of `fireline-dlpack`. Tests check the clauses' own numbers (the OCP limits they quote), the packing worked by hand, and the token and width rows of KISS `conformance/corpus/dtype_manifest.json` (origin/main 73c341b5). Not here: the other 24 dtypes, the `structure_key` grammar, float conversion into these formats (KISS-Ops/Emit), the `f8e8m0` scale operand. Draft, unfrozen: expect rework. Dependency-free. Verified with `cargo test -p fireline-classify --locked`, workspace clippy `-D warnings`, fmt, doc `-D warnings` and no example-name collision.

— lane: fireline

## [0.0.25] — `fireline-announce` device-query frames DVRQ, DVRS, DVDC (Draft-tracking)

New `fireline_announce::device` module (additive): encoders and decoders for the device-query request `DVRQ` (KISS-ANNOUNCE-6.6-0003), the device response `DVRS` with its device and accuracy records (6.6-0004, -0006, -0007, -0008) and the device decline `DVDC` (6.6-0005), sharing `FrameError` and the correlation field with `frames`. `FrameError` (non-exhaustive) gains the device-record variants. Every bound of 6.6-0004 and -0006 is checked before allocating; dtype sets must be strictly ascending, `compute_dtypes` a subset of `store_dtypes`, accuracy records strictly ascending by `(op, math_fidelity, dtype)` with a fidelity of 1 or 2 and a dtype the device computes on. The Classify dtype table is not copied: each coder takes an `is_dtype` predicate, so "a spelling of the Classify table, not reserved" (6.6-0007) is the caller's to answer. The three frames are byte-diffed against KISS `conformance/corpus/announce_vectors.json` (origin/main 73c341b5) in `tests/device.rs`. Draft, unfrozen: expect rework. Not covered: 6.6-0009 and -0010 (a consumer's pairing rules), which are not frames. Verified with `cargo test -p fireline-announce --locked`, clippy `-D warnings`, fmt and doc `-D warnings`.

— lane: fireline

## [0.0.26] — `fireline-contract` document framing (KISS-CONTRACT 6.11, FDX V1)

New `fireline_contract::document` module (additive): `parse_document` frame-checks a contract document and splits it into its seven sections, and `encode_document` frames a body, following KISS-CONTRACT-6.11-0002 (the pinned `KISC <kind> <version> len=<N> crc32=<HHHHHHHH>` header line, LF only), -0003 (inner length and CRC-32), -0003a (the 2^24 length cap, checked before any work keyed on `N`), -0003b (a CRC is not a signature) and -0004 (the seven heading lines in order), with the loud-failure rules of 6.1-0002, -0003 and -0004 and 6.1-0008 (the supported `contract_version` set is declared by the reader, never inferred from an order). Typed `DocumentError` with KISS's stable category tags (`wire_tag`). The golden document and all 13 single-fault decline vectors of KISS `conformance/corpus/contract_vectors.json` (origin/main 73c341b5) are byte-diffed in `tests/document.rs`: the golden document parses and re-frames to the same bytes, and each decline matches its category. This maps FDX V1 to its KISS clauses. Not covered: the field lines inside each section (6.11-0005 onwards), which are returned as raw bytes. Draft, unfrozen: expect rework. Verified with `cargo test -p fireline-contract --locked`, clippy `-D warnings`, fmt and doc `-D warnings`.

— lane: fireline

## [0.0.24] — patent sweep, first pass, for `fireline-contract`

`docs/PATENT-SWEEP.md` gains an addendum for `fireline-contract` (six ideas, three queries, two patents read: Fujitsu US 8,595,701 B2 and Intel US 12,242,973 B2, both read as clear). It is a first pass and does not clear the crate for publishing. Documentation only; no code change.

— lane: fireline

## [0.0.23] — `fireline-announce` message frames SAVL, CYRQ, CRSP, CDEC (Draft-tracking)

New `fireline_announce::frames` module (additive; the envelope API is untouched): encoders and decoders for the availability list `SAVL` (KISS-ANNOUNCE-6.3-0011, -0012, -0005), the contract-query request `CYRQ` (6.4-0001), the contract response `CRSP` (6.4-0004) and the decline response `CDEC` (6.4-0007), with the identity block (6.4-0011, 6.3-0013), the correlation field (6.4-0013) and the `decline_code` constants (6.4-0009). Decoders take the extent from the message alone, ignore bytes after it, check every declared length and count against the bytes left before allocating, and return a typed `FrameError` (6.2-0014..-0017). KISS-Announce is a Draft and its wire is not frozen, so expect rework. The four frames are byte-diffed against KISS `conformance/corpus/announce_vectors.json` (origin/main 73c341b5) in `tests/frames.rs`; the device-query frames DVRQ, DVRS and DVDC (6.6) are not built yet. Verified with `cargo test -p fireline-announce --locked`, clippy `-D warnings` and `cargo fmt`.

— lane: fireline

## [0.0.21] — `fireline-dlpack` clippy 1.100 `needless_range_loop` fix

Rust 1.100's clippy rejects three index loops over `a.terms` in `crates/dlpack/src/validate.rs` (the affine-term checks of the extent validator and the V17 evaluation), which failed `cargo clippy -D warnings` on every PR's CI from 2026-11-12. The loops now iterate the slices (`a.terms[..tc]`, `a.terms[tc..FDX_AFFINE_MAX_TERMS]`, `a.terms[..term_count]`), which panic on exactly the inputs the indexing did and preserve the order of checks and errors. No behaviour change. Verified with `cargo clippy --workspace --all-targets --locked -- -D warnings` and `cargo test -p fireline-dlpack --locked` (96 unit tests pass) on rustc 1.100.0.

— lane: fireline

## [0.0.20] — `fireline-contract` constant divisors are a build-time fault

Tracks KISS#800 (CONTRACT-6.12-0007, -0008, origin/main@73c341b5). `division_by_zero` and `ceil_div_divisor` are now ratified codes and stay bind-time declines only when the divisor names a launch-scalar symbol. A divisor with no symbol that is zero (`/`) or not positive (`ceil_div`), such as `n / 0` or `ceil_div(8, 0)`, now fails `Expr::parse` with the new `ParseError::ConstantDivisor` (code `build_time`) instead of declining at bind time. `subscript_out_of_range` is still unnamed by the Draft and stays a bind-time decline here (kiss reports it at build time; the rank is only known from the environment). Verified with `cargo test -p fireline-contract --locked` (8 integration tests and the doctest pass).

— lane: fireline

## [0.0.19] — fireline-contract 0.1.0 (first slice, Draft-tracking)

New crate `fireline-contract` (crates/contract, 0.1.0, its own `[[interface]]` entry): parse a Dispatch expression (KISS-CONTRACT-6.6-0006) and bind it against a launch environment with the typed declines of 6.12-0001..-0006 and the division rules of 6.6-0009. Follows the KISS Draft at `origin/main` `2d7f20e` on CireSnave's 2026-11-12 ruling ("working toward the draft version is the best we have. Proceed."); expect rework when it moves. Decline codes the Draft does not name (`division_by_zero`, `ceil_div_divisor`, `subscript_out_of_range`) are this crate's choice and marked so. Not in this slice: wire frames, the mapped FDX rows V1/V4/V10/V11, V3/V5/V6/V8/V9/V12/V13/V18..V21. Verified: 8 clause tests plus the doctest and the hello world pass; clippy `-D warnings`, fmt, doc and `build --all-targets` (no `collision`) are clean. **Patent sweep for this crate is not done** (unread is never "clear"); not for publishing until it is.

## [0.0.18] — CI pin

CI only: the advisory `portfolio` job now calls coderipper `portfolio.yml` at 33cc3b7 (coderipper#307), which installs CodeRipper 0.43.49 (read back: `CODERIPPER_VERSION: 0.43.49` at that SHA, 0.43.49 on crates.io). No crate change.

## [0.0.17] - 2026-11-12

- Tooling only: adopt the PM's changelog-fragment kit (`tools/stamp-version.py`, `tools/check-fragment.py`, `.github/workflows/changelog-fragment.yml`, `changelog.d/README.md`). From here a PR carries one `changelog.d/<topic>.md` fragment and no version edit; the PM stamps the number at merge. Verified: a throwaway fragment and `stamp-version.py 99.99.99 --dry-run` listed `Cargo.toml`, `Cargo.lock` and `CHANGELOG.md` (the 0.1.x interface crates are not touched). No crate change. The root version here is provisional.

## [0.0.16] - 2026-11-11

- CI only: the advisory `portfolio` job now calls coderipper `portfolio.yml` at 4bcf55e (coderipper#303), which installs CodeRipper 0.43.41; the old pin installed 0.10.x and predates `[[interface]]`. No crate change. The root version here is the PM's stamp.

## [0.0.15] - 2026-11-09

- Docs only: `docs/EXTRACTION-PLAN.md` sections 9, 10 and 12 follow CireSnave's 2026-11-09 clarification of the seam (FireLine owns the seam definitions; the two sides may talk directly; a run-time hop only where it is the best design) and the board 395 rulings (providers in their own projects, Fuel's built-in CPU path stays, first proof CPU `gemm` then Vulkan then AOCL, KISS owns the standards). The FDX spec-home question is closed: FDX is phased out for a KISS standard, with a retirement row for `fireline-dlpack`. The root version here is provisional.

## [0.0.14] - 2026-11-09

- `.coderipper.toml` declares the four interface crates (`fireline-announce`, `-grammar`, `-synth`, `-dlpack`) with `[[interface]]`, so CodeRipper's version-consistency check (per-interface mode, coderipper#278, with the 0.0.z fix in #288) no longer reports them against the project number. No crate change. The root version here is provisional.

## [0.0.13] - 2026-11-09

- Lockfile only: `syn` 3.0.9 to 3.0.10 (CodeRipper dependency-freshness). No crate change. The root version here is provisional.

## [0.0.12] - 2026-11-08

- Publish-metadata fixes (PM): `rust-version = "1.88"` on `fireline-announce`, `fireline-grammar` and `fireline-synth`, and `categories` on `fireline-synth`; the three crates go 0.1.0 to 0.1.1 (metadata only, no code change). 0.1.0 of all three is on crates.io. The root version here is provisional; the PM restamps at the gate.

## [0.0.11] - 2026-11-07

- New crate `fireline-dlpack` 0.1.0, **unpublished by decision** (`publish = false`): FDX, the DLPack extension (C-ABI structs, codes, sidecar, V-numbered validators, the co-maintained C header), moved from Fuel's `fuel-ir/src/dlpack`. It implements FDX only, not the final quantisation sidecar. Patent section added to `docs/PATENT-SWEEP.md`. The root version here is provisional; the crate versions by its own interface.

## [0.0.10] - 2026-11-07

- Publishing prep for the wave-1 crates: `publish = false` is removed from `fireline-announce`, `fireline-grammar` and `fireline-synth`, `cargo package` verifies all three together, and `docs/PUBLISHING.md` has the checked order and commands. Nothing is published; that waits on CireSnave (board 398) and the PM. The root version here is provisional.

## [0.0.9] - 2026-11-07

- New crate `fireline-synth` 0.1.0 (not published): the kernel-seam JIT-on-request envelope, the `Synthesizer` trait and the `JitRequest`, `JitBudget`, `JitResponse`, `SynthArtifact` and `LinkEntry` types, moved from Fuel's `fuel-kernel-seam`. It depends on `fireline-grammar ~0.1` and `unpopped-vocab 0.43.2`. The root version here is provisional; the crate versions by its own interface.

## [0.0.8] - 2026-11-07

- New crate `fireline-grammar` 0.1.0 (not published): the kernel-seam region grammar (`OpTag`, `OpAttrs` with canonical bytes, `RegionKey`, `PatternNode`, the shape-expression codec), moved from Fuel's `fuel-kernel-seam-types` with a guard that keeps `PatternNode` a closed enum. The root version here is provisional; the crate versions by its own interface.

## [0.0.7] - 2026-11-07

- New crate `fireline-announce` 0.1.0 (not published): the kernel-seam handshake, the frozen 56-byte `SeamHello`, capability bits and role-aware negotiation, merged from Fuel's `fuel-kernel-seam-announce` and baracuda's `baracuda-seam`. The root version here is provisional; the crate versions by its own interface.

## [0.0.6] - 2026-11-07

- Patent sweep addendum (ideas 4 and 11 searched, two more patents read), plan section 12 (`fireline-dlpack` whole-tree findings and the open FDX-spec-home decision), and the lane HANDOFF. Docs only.

## [0.0.5] - 2026-11-07

- Tilde lint: `[patch]` tables are scanned, untracked manifests are included, requirement versions are validated strictly, and a manifest listed but missing on disk is reported instead of aborting the run (reviewer findings on #3, each reproduced first). Tool only; nothing published.

## [0.0.4] - 2026-11-07

- Extraction plan (`docs/EXTRACTION-PLAN.md`, sections 1-11) and the lane HANDOFF.
- Patent sweep record, first pass, for `fireline-announce`, `fireline-grammar` and `fireline-synth` (`docs/PATENT-SWEEP.md`, replacing the root placeholder). It says what was not read.
- The root package and `fireline-tilde-lint` now share one `[workspace.package]` version, so they cannot drift apart (CodeRipper version-consistency); interface crates will not inherit it.

## [0.0.3] - 2026-11-07

- Workspace skeleton and `fireline-tilde-lint` (not published): fails when a manifest requires a FireLine crate with a caret, a bare version, a range or a `git` source. CI runs it on this repo; consumers run it on theirs. No FireLine crate exists yet, so nothing is published.

## [0.0.2] - 2026-11-07

- Seam inventory (`docs/SEAM-INVENTORY.md`). Docs only.

## [0.0.1] - 2026-10-28

- HANDOFF for the new FireLine lane. Docs only.

## [0.0.0] - 2026-10-28

- Name reserved with an empty placeholder crate.
