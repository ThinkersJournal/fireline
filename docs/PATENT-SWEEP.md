# Patent sweep: fireline-announce 0.1.0, fireline-grammar 0.1.0, fireline-synth 0.1.0 (project line 0.0, crates line 0.1) @ lane branches 95bf99f, ed30aee, 6328855 (FIRST PASS, NOT COMPLETE)

Date: 2026-11-07. Run by: the FireLine lane, Claude Sonnet 5.5. Method: AI-assisted search and read. **NOT legal advice; NOT a freedom-to-operate opinion.** Process: `C:\Projects\PATENT-SWEEP-PROCESS.md`.
Scope: only the three crates named above, which carry a capability handshake, a JIT-on-request envelope and a region grammar. They contain **no tensor arithmetic, no scheduling or placement, no kernel library and no runtime**; the choice of target processor and every adoption decision happen in the consumer, outside them. Verdicts are about these crates, **not** about Fuel, baracuda or any product built on them. `fireline-tilde-lint` is unpublished tooling and is not swept. `fireline-dlpack`, `fireline-contract` and `fireline-provider` do not exist yet and need their own sweep (see the design gate).

**Status: first pass.** What was done is listed exactly below; what was not done is listed after it. The PM decides whether this is enough to publish.

## Starting point: Fuel's record (inherited, with its provenance)

The code in these crates was moved from Fuel's `fuel-kernel-seam-types`, `fuel-kernel-seam` and `fuel-kernel-seam-announce`. Fuel swept them: `docs/PATENT-SWEEP.md` ("Claim-read verdicts for the three seam crates, 2026-10-29, 0.28.0 @ 8ff54523e") and `docs/PATENT-SWEEP-CHARTS.md` in Fuel at `origin/main` `d010189e`. **Fuel states that section is from a separate read agent's report, independent claims only, and that the Fuel lane did not re-read the claim text.** I (this lane) have therefore re-read the rows I could and mark every other row as inherited, not mine. Fuel's table, with its own statuses (Google's display, not the USPTO record):

| Patent | Fuel's verdict for the seam crates | Read by me? |
|---|---|---|
| US 8,458,680 B2 | arguable, low | **yes** (below) |
| US 8,108,844 B2 (parent) | arguable, low | **yes** (below) |
| US 2023/0025245 A1 | arguable, low (seam crates); arguable, medium for Fuel as a whole | **yes** (below) |
| US 11,556,756 B2 | outside as read; maintenance fee unverified | no (inherited) |
| US 12,705,545 B2 | outside as read | no (inherited) |
| US 12,670,371 B2 | outside as read | no (inherited) |
| US 12,045,307 B2 | outside as read | no (inherited) |
| US 12,412,082 B2 | outside as read | no (inherited) |
| US 2024/0354570 A1 | outside as read | no (inherited) |
| US 2025/0125819 A1 | outside as read | no (inherited) |
| US 11,521,062 B2 | outside as read | no (inherited) |
| US 12,619,868 B2 | outside as read | no (inherited) |

Fuel's own caveats carry over: the code moved was at Fuel `d010189e` (0.43.1), later than the swept 0.28.0, and Fuel's addendum lists new ideas after the swept commit as "not searched" (its ideas 17 and 18). Idea 17 (role-aware negotiation) is searched below; idea 18 (a key-derivation decline) is in Fuel's `fuel-dispatch`, not in these crates.

## Ideas swept (what these crates do that might be new or unusual)

1. A **fixed-size (56-byte) handshake envelope** with its own never-bumped version, a bounded strictly ascending profile list, and **explicit reserved fields that are zeroed on write and hard-rejected on read**, so a garbage-padded envelope is refused. (`fireline-announce`)
2. Selecting the **highest mutually supported profile** and **hard-failing, with a typed error carrying both lists, on a disjoint set**. (announce)
3. A **role-aware live capability word**, `(local & remote & !FEAT) | (provider & FEAT)`: the provider's feature bits belong to the provider alone, the provider is known from the exchange and never from a capability bit, and the extension and sub-feature axes stay a plain AND. The axes are masked **by axis, not by recognition**, so two readers that recognise different feature bits compute the same word. (announce; Fuel's idea 17)
4. A check that a side **advertises only capability bits its shared crate has allocated**, so a provisional bit cannot be advertised by accident. (announce)
5. A **two-step handover**: a synthesis request returns a light handle or a typed decline; the heavy compiled artifact crosses only after the consumer's cost gate adopts, exactly once, so a declined kernel transfers nothing. (`fireline-synth`)
6. A synthesis request carrying a **region pattern, operand descriptions, a target derived infallibly from an architecture tag, and a compile-time budget**. (synth)
7. **One pattern node type serving three roles**: the JIT region sent to a synthesizer, a contract's re-fuse rule, and a synthesized op's decomposition; bind indices must cover `[0, n)`. (`fireline-grammar`)
8. A **structural region key** that excludes scalar-parameter values, keys floats by their bits, keeps "absent" distinct from "zero", and is built by an exhaustive destructure so a new attribute must be classified before the code compiles. (grammar)
9. A **canonical, length-prefixed attribute encoding with one body schema per family of ops** and a typed error for an unresolved attribute. (grammar)
10. A **typed shape-expression AST with a wire codec and an evaluator**. (grammar, `shape_expr`)
11. An **advisory ULP tolerance band for a fused region** computed from per-op ceilings (transcendental ceilings summed, exact ops counted as `n - 1`). (grammar, `advisory_band_reference_cases`)

Not an idea for this sweep: the closed-enum marker guard is a development practice, not a product mechanism.

## Queries (source, query, date 2026-11-07)

Search was the web-search tool restricted to `patents.google.com` (US-only index), standard mode, one query per group of ideas, run one at a time. Patent text was fetched with `C:\Projects\tools\patent-claims.py` (one request per number).

| # | Ideas | Query |
|---|---|---|
| Q1 | 1, 2, 3 | `patent capability negotiation handshake provider-only feature bits consumer role kernel provider profile version` |
| Q2 | 5, 6 | `patent just-in-time kernel synthesis request returns handle then transfers compiled artifact after cost model adoption decline neural network graph region` |
| Q3 | 7, 8, 9 | `patent region identity key hash of fused operator subgraph structure excluding scalar values kernel cache neural network canonical encoding of operation attributes` |
| Q4 | 10 | `patent symbolic tensor shape expression serialization wire format evaluator dynamic shape dimension arithmetic expression kernel provider` |

| Q5 | 4 | `patent verifying advertised capability bits against registry of allocated bits reserved provisional bit rejected handshake extension negotiation` (added later the same day, see the addendum) |
| Q6 | 11 | `patent numerical tolerance for fused kernel computed from per-operation ulp error ceilings sum transcendental functions verifying generated kernel against reference` (addendum) |

**Not searched at all:** idea 1's reserved-bytes mechanism specifically. Ideas 4 and 11 were searched once each (Q5, Q6; addendum). The Q1 to Q6 results are single-pass; none of them is a claim-language or CPC-class search.

## Hits

Read by me: the **independent claims in full** (all three claim blocks of each), from Google Patents text fetched 2026-11-07; dependent claims skimmed only. Statuses and dates are Google's display, not the USPTO record. "Overlap" and "difference" refer to what these three crates do.

| # | Patent | Title | Assignee | Status (Google) | Claims read | Overlap | Our difference | Verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | US 8,458,680 B2 | Systems and methods for dynamically choosing a processing element for a compute kernel | Google LLC | Active; estimate 2030-08-08 (the tool prints the parent's date for this number; Fuel's record says the '680 page itself shows 2027-03-05: not resolved here) | 1, 9, 17 (independent; 2-8, 10-16, 18-24 skimmed) | The claims need a **runtime that, at run time, chooses a type of processing element** (CPU, GPU, co-processor) for an operation request, **identifies from a library a precompiled, processor-specific compute kernel**, and **prepares** kernels including it. The crates share only vocabulary: an operation request and a kernel artifact. | The three crates contain **no runtime, no choice of processing-element type, no library of precompiled kernels**; the target architecture arrives in the request already chosen. The synthesizer *builds* a kernel; it does not select one from a library. | **clear** for these crates, with the **design gate** below |
| 2 | US 8,108,844 B2 (parent of #1) | same title | Google LLC | Active; estimate 2030-08-08 | 1, 8, 15 (independent) | Same steps, narrowed: "dynamically choosing", "dynamically identifying", and the intrinsic operation must be an **arithmetic operation**. | Same as #1; no arithmetic is performed here at all. | **clear** for these crates, with the **design gate** |
| 3 | US 2023/0025245 A1 | Neural network evaluation | Nvidia Corp | **Pending** (application) | 1, 9, 17, 25 (independent; 4 blocks) | Each claim is one sentence: **causing the performance of a neural network to be modified, at least in part based on a user-provided description of at least portions of the network.** A human-authored pattern (a contract's re-fuse rule) is a description of a portion of a graph, and the consumer that applies it modifies how the network runs. | The crates are types and a handshake: they **perform no modification of any network**; the consumer's optimizer does, and its region comes from the consumer's own graph. But the claim is so broad that whether a system built on these crates is outside it depends on facts I cannot settle from claim 1 alone, and the application may be granted with narrower or different claims. | **needs-review** (low for the crates alone; **medium for Fuel as a whole**, which stays Fuel's open item, board 307). To the PM. |
| 4 | US 9,215,150 B2 | Feature set negotiation framework | SAP SE | Active | 1, 7, 13 (independent) | A client sends **feature indicators** in a request, the server replies with **its own feature indicators**, the two run different software versions, and the client works out which requested feature is missing. This is the nearest thing to a capability handshake. | The claims need a **service request for data**, features **specific to a user of the first server**, the second server **modifying that data** and returning modified plus unmodified data, a request **sent to an address specific to the feature indicators**, and **a notification** of the excluded feature. Our handshake exchanges two fixed envelopes with no data request, no user-specific features, no data modification and no notification. | **clear** (the independent claims require elements absent here) |
| 5 | US 10,901,715 B1 | Lazy compilation and kernel fusion in dynamic computation graphs | an individual | Active | 1, 7, 13 (independent) | A graph is translated from user code, optimized, executed, and a **transformation graph** is generated **by comparing the structure of the input graph with the optimized graph** and **stored in a cache** for later. The nearest hit for a structural graph identity (idea 8). | Our region key is an **identity of one pattern**; the crates generate no transformation graph, compare no before and after structures, and cache nothing. | **clear** |

**Rows 1 and 2 have an in-force term to about 2030 on Google's estimate**, which is longer than the 2027 the '680 page was reported to show; any "wait for expiry" reasoning must cite 2030.

### Addendum 2026-11-07 (later): ideas 4 and 11, searched at the PM's request

Same method and reading standard as above: independent claims read in full by me.

| # | Patent | Title | Assignee | Status (Google) | Claims read | Overlap | Our difference | Verdict |
|---|---|---|---|---|---|---|---|---|
| 6 | US 10,936,769 B2 | Systems and methods for measuring error in terms of unit in last place | MathWorks Inc | Active | 1, 34, 35, 46, 49 (independent; 5 blocks) | The claims need an **executable simulation model** with model elements, **a lookup of stored ULP error values for each element**, **inferring those values through the model**, **computing a total ULP error value**, and **presenting it** (claims 1, 34, 35) or **generating code** for part of the model (claims 46, 49). The nearest hit for idea 11: summing per-op ULP ceilings into a tolerance for a fused region is a per-element lookup propagated into a total. | The three crates **compute no ULP value at all**: `fireline-grammar` holds only a table of *expected results* (`advisory_band_reference_cases`), a test fixture that pins two formulas implemented elsewhere (Fuel `fuel_dispatch::jit_ingest::advisory_ulp_band` and `fuel_kiss_ref_backend::region_advisory_tolerance`). Even for those, whether a neural-network computation graph is "an executable simulation model that simulates a behavior of a system" is a limiting term I cannot settle. | **clear** for the three crates (they perform none of the claimed steps). **needs-review for the formula where it is implemented** (Fuel and kiss-ref), to the PM and Fuel. |
| 7 | US 8,493,857 B2 | Multiple logical channels for use in network devices | Broadcom Corp (now Avago) | Expired - Fee Related; estimate 2022-12-15 (Google) | 1, 8 (independent) | The only reason it surfaced is Q5's "capability bits exchanged during a handshake" in its description. | The claims are about **flow control between network devices**: a congestion flow message, an Ethernet frame interpreted as a pause frame, halting selected packets. Nothing resembles an envelope or a capability word. Also expired on Google's estimate. | **clear** |

**Idea 4 (advertise only allocated capability bits): one search, no matching patent found.** Q5 surfaced TLS handshake-extension and capability-verification families (WO 2015/032732 A1, US 2016/0182232 A1, US 9,668,139 B2, WO 2010/027314 A1) and other capability-register patents (US 9,036,643, US 8,023,413, US 7,372,899 B2, EP 1997344 A1); **none of those claims was read**, so they carry no verdict. A search that finds nothing proves nothing: this is "not found in one pass", not "clear".

**Design gate extension:** if the ULP-band formula (or any per-op error-ceiling aggregation) ever moves into a FireLine crate, US 10,936,769 must be re-read against that code before the release.

### Design gate (binding on later releases)

Rows 1 and 2 are "clear" **only because the crates contain no selection step and no kernel library.** `fireline-provider` (plan section 10, vendor-library providers announcing **prebuilt entry points**) is precisely a precompiled, processor-specific kernel in a library, and a consumer that chooses among providers per operation is the "choosing a type of processing element" step. **No FireLine crate may take on either step without re-reading US 8,458,680 and US 8,108,844 first**, and that re-read goes to the PM before release. The same applies to Fuel's own Judge; that is Fuel's record.

### Surfaced by the searches and NOT read (claims unread, so no verdict, never "clear")

US 4,953,210 A (feature negotiation protocol, synchronous modem; a 1980s family, status not checked); US 8,010,778 B2 (negotiating a capability in a peer-to-peer link); US 2021/0294602 A1 and its continuation US 2023/0350673 A1 (microkernel-based optimisation of neural networks; mentions a JIT compiler); US 2024/0028886 A1 (graph optimisation for neural-network computation); WO 2020/263588 A1 (composable neural network kernels); US 12,373,257 B2 (static scheduling with a cost model); US 2016/0267622 A1 (automated compute kernel fusion); US 2013/0091507 A1 (fused and split kernels for GPUs); US 9,524,323 (delegation of expression tree evaluation; the closest thing Q4 returned for idea 10); and the Fuel-table rows marked "inherited". Most were surfaced by one search each and some look unrelated; they are listed so the next read pass starts from them, not because any overlaps.

## Needs review (to the PM)

1. **US 2023/0025245 A1** (Nvidia, pending): see row 3. Re-check the USPTO record for a grant and read the granted claims before the first publish; Fuel already carries it as medium for Fuel as a whole.
2. **The design gate** above, for any future crate that selects a processing element or serves prebuilt kernels.
3. **US 10,936,769 B2** (MathWorks): clear for these crates, but the fused-region ULP band formula in Fuel and kiss-ref is not swept by this record; row 6.

## Limitations

- AI-assisted search and reading; not legal advice; not a freedom-to-operate opinion; a lawyer's review is a separate purchase.
- The idea list is written by the lane from the code and its own design notes; a mechanism nobody wrote down is not swept. **The reserved-bytes part of idea 1 was not searched on its own**; ideas 4 and 11 were searched once each (addendum).
- Search covered one query per idea group through a US-only web index limited to `patents.google.com`; USPTO, Espacenet and WIPO were not searched directly; no CPC-class or inventor search was run. A search that finds nothing proves nothing.
- Only **seven** patents were read by this lane (five in the first pass, two in the addendum), independent claims only. **Nine of Fuel's twelve rows and every "surfaced, not read" patent were not read by me.** Claim text is machine-converted; check the official record before relying on a quotation.
- Statuses, expiry dates and fee states are Google's display and estimates; **no USPTO record, prosecution history, term adjustment, maintenance-fee state or foreign family was checked.**
- The comparison is a static reading of the three crates at the swept branches; nothing was run for this purpose.
- Pacing: 6 searches and 7 claim fetches, spread across the session; no HTTP 503 was seen.

## Addendum: fireline-dlpack 0.1.0 (FIRST PASS, NOT COMPLETE) @ lane branch `lane/fireline-dlpack`

Date: 2026-11-07. Run by: the FireLine lane, Claude Sonnet 5.5. Same method, reading standard and limits as the sections above: independent claims read in full by me from Google Patents text fetched with `patent-claims.py`; statuses are Google's display. **NOT legal advice; NOT a freedom-to-operate opinion.**
Scope: only `fireline-dlpack`: the FDX C-ABI structs, codes, sidecar and V-numbered validators, moved from Fuel's `fuel-ir/src/dlpack/` (`convert.rs` stays in Fuel) and the C header. The crate **performs no computation, no address translation, no data transfer and no kernel launch**: it defines structs and checks them. It implements FDX only, not KISS's later neutral quantisation sidecar.

**Inherited, not read by me.** Fuel's record covers FDX through its ideas 9 ("self-describing tensor storage with scale as a sibling operand") and 14 ("kernel interchange seam with a closed key and an open quantization sidecar"); Fuel's own addendum states **no new idea-14 searches were run and only two hits (Baidu) were read** for it. I have not re-read those rows; they carry Fuel's verdicts, not mine.

### Ideas swept (written from the struct names, the validator list and the source, not from a full read of the FDX spec)

1. A **versioned optional sidecar attached to a standard tensor descriptor**, carrying a magic, a version and its own byte size, so a reader rejects a descriptor that is too small, newer than it knows, or not a sidecar at all (V1).
2. **Flag and field coherence**: each "has block" flag is set if and only if its block is present, and validators are numbered one per rule (V2, V1 to V21).
3. An **honesty rule**: the sidecar's stated element type must agree with the base tensor's declared type (V3), and sub-byte elements must be padded as declared (V4).
4. **Block-scaled quantisation described in a descriptor separate from the tensor**: family, block size, scale placement, scale granularity and scale dtype, with the scales carried as sibling buffer references (V5, V6, V9).
5. **Symbolic extents**: a per-axis descriptor with a minimum, a capacity bound and an affine term over named symbols, plus a symbol environment (V7, V8).
6. **Indexed residency (gather)**: a paged block table with logical extents and context lengths, for a paged key-value cache (`FDXIndexedResidency`, `FDXBlockTable`).
7. **Tiling, residency and storage-tier descriptors and output views** carried in the same sidecar.
8. A **co-maintained C header whose `sizeof` static asserts are cross-checked against the Rust structs by a test**, with no C compiler.

### Queries (source, query, date 2026-11-07)

Same method as above (web search restricted to `patents.google.com`, US-only, standard mode).

| # | Ideas | Query |
|---|---|---|
| Q7 | 4 | `patent tensor interchange descriptor extension sidecar quantization block size scale placement sub-byte packing alongside DLPack tensor metadata` |
| Q8 | 1, 2, 3 | `patent validating tensor descriptor extension flags set if and only if corresponding block present dtype consistency with base tensor descriptor reject malformed` |
| Q9 | 5, 6 | `patent tensor shape descriptor symbolic dimension capacity bound affine expression of named symbols paged block table indexed gather residency interchange format` |

**Not searched at all:** ideas 7 and 8 (tiling, residency and storage-tier descriptors; the C-header cross-check). Q7 to Q9 are single-pass and none is a claim-language or CPC-class search.

### Hits

| # | Patent | Title | Assignee | Status (Google) | Claims read | Overlap | Our difference | Verdict |
|---|---|---|---|---|---|---|---|---|
| 8 | US 2025/0291873 A1 | Universal Scale Metadata Layout for Matrix Multiply and Add (MMA) | Nvidia Corp (Google spells it "NVDIA") | **Pending** (application) | 1, 17, 18 (independent) | The nearest hit for idea 4: **scale metadata block data structures**, each holding s scale factors for a p by q area of a matrix. | The claims need **an MMA circuit** that calculates a product of two matrices using the scale metadata, **scale blocks whose width q is determined from a vector length and a scale-factor allocation**, and **copying them from global memory to the MMA circuit's device memory**. The crate describes a scale layout in a descriptor; it multiplies nothing and copies nothing. | **clear** for this crate. Pending: re-check for a grant. |
| 9 | US 12,008,395 B2 | Program event recording storage alteration processing for a neural network accelerator instruction | IBM | Active; estimate 2041-12-29 (Google) | 1, 13, 17, 21, 25 (independent) | Surfaced by Q8 because its description speaks of tensor-descriptor inconsistencies. | The **claims** are about **instruction processing in phases, detecting a store into a designated area, and presenting an interrupt** (or checking that none occurred). Nothing about descriptor validation. | **clear** |
| 10 | US 12,141,082 B2 | Method and apparatus for efficient access to multidimensional data structures and/or other large data blocks | Nvidia Corp | Active; estimate 2042-03-30 (Google) | 1, 22, 23, 24, 25 (independent) | A "tensor descriptor" bundling per-dimension sizes and strides is the vocabulary nearest to a tensor sidecar (ideas 5, 6). | The claims are **hardware memory-access circuits** that **translate a coordinate into memory addresses and asynchronously transfer a block** between external and shared memory, and decide whether the block is part of a tensor. The crate has no hardware, no address translation and no transfer. | **clear** |
| 11 | US 2021/0343066 A1 | Residency map descriptors | Advanced Micro Devices | Active (claims 1-20 canceled; 21 to 40 read) | 21, 30, 40 (independent) | A **residency map** and a descriptor with dimensions: the nearest thing to idea 6's residency. | The claims are about a **partially resident texture with levels of detail, translating normalized texture coordinates to texel coordinates** using the map, or storing an offset between levels. Idea 6 is a paged tensor block table, with no textures, no levels of detail and no coordinate translation by the crate. | **clear** |

**Surfaced and NOT read** (no verdict, never "clear"): US 11,321,092 B1 (tensor-based memory access), WO 2020/263588 A1 (composable neural network kernels), US 12,694,294 (concatenating tensors; a fixed numeric dimension-size limit), the Cambricon descriptor-management family (US 11,886,880, US 11,385,895, US 12,346,697), US 8,860,743 B2 (sparse textures), US 2023/0289292 A1 (the parent of row 10), and US 12,705,167 (blocked-format quantisation near memory).

### Needs review (to the PM)

None new from this pass. **Fuel's own ideas 9 and 14 remain Fuel's open items.** Rows 8 to 11 were read only on their independent claims, and row 8 is a pending application.

### Limitations specific to this addendum

- The idea list comes from struct names, the validator list and the code; **I did not read the FDX spec in full**, so a mechanism described only there may be unswept. Ideas 7 and 8 were not searched.
- Three searches, four patents read, plus Fuel's rows inherited and unread. Statuses, expiry dates and fee states are Google's; no USPTO record was checked.

## Addendum: fireline-contract 0.1.0 (FIRST PASS, NOT COMPLETE) @ origin/main 8d013e4 (plus fireline#22 constant-divisor change)

Date: 2026-11-12. Run by: the FireLine lane, Claude Sonnet 5.5. Same method, reading standard and limits as the sections above: claims read from Google Patents text fetched with `patent-claims.py`; statuses are Google's display. **NOT legal advice; NOT a freedom-to-operate opinion.**
Scope: only `fireline-contract` (`crates/contract`): a parser and bind-time evaluator for KISS-CONTRACT 6.6-0006 / 6.12 Dispatch expressions. It **runs no kernel, allocates nothing and touches no device**: it parses integer expressions and evaluates them against a caller-supplied environment.

### Ideas swept (written from the source, not from a full read of KISS-Contract)

1. **Launch extents as integer expressions over a closed vocabulary of named launch-scalar symbols** (`n`, `ws_bytes`, `off{i}`, `param{i}`, and subscripted rank-length arrays), parsed once at build time and **bound to a different value under each launch environment** (6.12-0001).
2. **Pinned integer semantics**: `/` truncates toward zero, `ceil_div` is the true ceiling for a positive divisor (never the truncating `(a + b - 1) / b`), and `i64::MIN / -1` and every `+ - *` that leaves the signed 64-bit range decline instead of wrapping (6.6-0009).
3. **Typed bind-time declines** (`unbound_symbol`, `extent_out_of_range`, `affine_overflow`, `division_by_zero`, `ceil_div_divisor`), with the first failing step in a fixed left-to-right order naming the decline, and a non-negative and range gate applied to the final value only (6.12-0003..-0006).
4. **A split between build-time and bind-time faults**: a malformed expression or a symbol-free zero divisor is a build-time fault, a symbol-dependent one declines at bind time (6.12-0002, -0007, -0008).
5. **An unassigned symbol never defaults to zero**: unbound and zero are distinct (6.12-0003).
6. A **nesting limit** so a hostile expression cannot overflow the stack.

### Queries (source, query, date 2026-11-12)

Web search restricted to `patents.google.com`, US-only, standard mode.

| # | Ideas | Query |
|---|---|---|
| Q10 | 1, 4, 5 | `patent symbolic launch dimension expression bound at kernel launch time affine expression of named scalar symbols ceiling division typed decline when symbol unbound` |
| Q11 | 2, 3 | `patent evaluating integer shape expression at runtime tensor dimension symbolic shape binding overflow detection division by zero error before kernel launch` |
| Q12 | 1, 2 | `patent compiler dynamic shape symbolic expression evaluated at launch grid size ceil_div workspace bytes computed from runtime parameters kernel contract validation` |

**Not searched at all:** idea 6 (nesting limit). Q10 to Q12 are single-pass; none is a claim-language or CPC-class search, and the search engine returned few patent pages (most hits were papers or USPTO PDFs whose claims were not shown).

### Hits

| # | Patent | Title | Assignee | Status (Google) | Claims read | Overlap | Our difference | Verdict |
|---|---|---|---|---|---|---|---|---|
| 12 | US 8,595,701 B2 | Symbolic execution and test generation for GPU programs | Fujitsu | Expired, fee related; estimate 2031-12-06 (Google) | 1, 10, 19, 28 (independent) | Symbolic inputs and symbolic expressions for a GPU program, and a test case of values that satisfy a result. | The claims need **bytecode compiled from a program**, **symbolic execution of threads on a deterministic schedule simulating SIMD execution**, and **path conditions per execution path**. The crate evaluates a concrete integer expression against concrete values; it has no bytecode, no threads, no symbolic path exploration and no test generation. | **clear** |
| 13 | US 12,242,973 B2 | Graph context-based operator checks to improve graph clustering and execution in AI accelerator framework integration | Intel | Active; estimate 2043-11-30 (Google) | 1 and 6 (independent; any further independent claim not read) | A check that a shape attribute has **no zero dimension values**, used before a node is marked supported. | The claims **parse a deep learning graph in topological order and mark nodes supported or unsupported by target hardware**, then cluster them. The crate checks a divisor inside an arithmetic expression and clusters and marks nothing. | **clear** |

**Surfaced and NOT read** (no verdict, never "clear"): US 2021/0271587 A1 (detection of runtime errors by machine learning), CN 103399780 A (integer overflow detection by dynamic symbolic execution), and several NVIDIA kernel-launch patents that appeared only through boilerplate text in USPTO PDFs (not identified by number). The papers (BladeDISC++, Nimble, PyTea, ShapeFlow) are prior art for the idea, not patents.

### Needs review (to the PM)

None from this pass. **This sweep does not clear `fireline-contract` for publishing**: two patents were read, the three searches were thin, and ideas 1 to 3 (the closest to compiler and runtime shape handling) deserve a claim-language and CPC-class pass before a release.

### Limitations specific to this addendum

- The idea list comes from the source; **I did not read KISS-Contract in full**, so a mechanism described only there may be unswept.
- Three searches, two patents read, four surfaced and unread. Statuses, expiry dates and fee states are Google's; no USPTO record was checked.
- `fireline-contract`'s wire frames (SAVL, CYRQ, CRSP, CDEC) are not built yet and are not covered.

## Addendum: fireline-classify 0.1.0 (FIRST PASS, NOT COMPLETE) @ origin/main 0a180a4

Date: 2026-11-13. Run by: the FireLine lane, Claude Sonnet 5.5, on the PM's task (publish of `fireline-classify` 0.1.0 waits for this addendum). Same method, reading standard and limits as the sections above: claims read in full by me from Google Patents text fetched with `patent-claims.py`; statuses are Google's display. **NOT legal advice; NOT a freedom-to-operate opinion.**
Scope: only `fireline-classify` (`crates/classify`): the three MX element dtypes `f4e2m1`, `f6e2m3`, `f6e3m2` of KISS-CLASSIFY-6.1-0014..-0016: a value rule (decode a 4- or 6-bit code to its exact value), a byte packing (two 4-bit elements per byte, low nibble first; four 6-bit elements per three bytes, least-significant bit first; trailing bits zero) and a rule that an MX element is never an accumulator dtype (a typed decline). The crate **multiplies nothing, accumulates nothing, quantises nothing and runs on no device**: it defines the formats and checks a token and a buffer.

### Provenance (which spec, whose licence, what is copied)

- **Value rule and constants:** the OCP Microscaling Formats (MX) Specification Version 1.0 (September 2023), sections 5.3 to 5.3.3 (E2M1 with exponent bias 1; E2M3 bias 1; E3M2 bias 3; no infinity, no NaN; the maximum, minimum-normal and subnormal values), as cited by KISS-CLASSIFY-6.1-0014 and -0015, which record them as "verified-against-OCP (2026-10-26)". The specification is the work of the OCP MX Alliance (AMD, Arm, Intel, Meta, Microsoft, NVIDIA, Qualcomm; credit given here and in the crate README). **What is taken is the format definition (field widths, bias, the value formula and the limits it states): numbers and a formula, not text or code.** The crate quotes no passage and copies no source file; the tests restate the limits the KISS clauses quote.
- **Packing:** NOT from OCP (neither v1.0 nor v1.1 mentions byte layout). The 4-bit nibble order is inherited from KISS-CLASSIFY-6.1-0008 (`i4`/`u4`) and the 6-bit "four per three bytes, least-significant bit first" layout is, in KISS-CLASSIFY-6.1-0015's own words, "the Unpopped lane's storage convention adopted as the pin" (the OCP document fixes no packing); KISS says it has not been checked against any CUTLASS, Mojo, Triton or hardware layout. This crate follows the KISS text and states the same limit.
- **Which version:** the value rule and constants come from OCP MX **v1.0** (Version 1.0, dated Sep 2023 on its cover; PDF created 2023-10-13), the version KISS-CLASSIFY-6.1-0014 and -0015 cite. **I also read v1.1** ("OCP MicroXcaling (MX) Specification", Version 1.1, October 9, 2026; the site serves two copies, `OCP_MicroXcaling_Spec_v1.1.0.pdf` and `...-1.pdf`, whose extracted text differs only in the cover line "Version 1.1" against "Version 1.1.0"). Read side by side (text extracted with `pdftotext`, 2026-11-13): **the FP4 (E2M1) and FP6 (E2M3, E3M2) tables are identical in v1.0 and v1.1** (bias 1, 1, 3; no Inf, no NaN; max normal 6.0, 7.5, 28.0; min normal 1.0, 1.0, 0.25; the subnormal values), and so is the value formula of section 5.3 (a normal number `(-1)^S * 2^(E-bias) * (1 + 2^-m * M)`, a subnormal `(-1)^S * 2^(1-bias) * (0 + 2^-m * M)`). v1.1's version history lists what did change: the conversion description in 6.3, the rename of the scale `E8M0` to `UE8M0`, refinements of the FP8/FP6/FP4 **conversion modes and rounding terminology** in 5.3, NaN and out-of-range clarifications, and wording. None of that is in this crate (it converts nothing and has no scale). **Neither version specifies any packing of elements into bytes** (no mention of bytes, nibbles or layout in either text), which confirms the packing below is not OCP's.
- **Licence of the OCP specification: read, 2026-11-13.** Section 1 of v1.0 (and, with Google added to the contributor list, of v1.1) states, verbatim: "Contributions to this Specification are made under the terms and conditions set forth in Open Web Foundation Modified Contributor License Agreement ("OWF CLA 1.0") ("Contribution License") by: AMD, Arm, Intel, Meta, Microsoft, NVIDIA, Qualcomm" and "Usage of this Specification is governed by the terms and conditions set forth in Open Web Foundation Modified Final Specification Agreement ("OWFa 1.0") ("Specification License")", and points to `http://www.opencompute.org/participate/legal-documents/` for the agreements ("For actual executed copies of either agreement, please contact OCP directly"). That URL now returns HTTP 404; the OCP page "Contribution Templates & Agreements" (`https://www.opencompute.org/contributions/templates-agreements`, read 2026-11-13) lists "Modified OWFa1.0 Final Specification Agreement (FSA) (As of August 16, 2021)" under "Archived / Deprecated" at `https://www.opencompute.org/documents/modified-owfa10-final-specification-agreement-fsa-as-of-august-16-2021`. **I read that PDF** (7 pages, "Modified August 16, 2021"; a **blank template**, not an executed copy; it lists no signatory) and quote its operative clauses:
  - **2.1 Copyright Grant:** "a perpetual (for the duration of the applicable copyright), worldwide, non-exclusive, no-charge, royalty-free, copyright license ... to reproduce, prepare derivative works of, publicly display, publicly perform, sublicense, distribute, and implement the Specification to the full extent of my copyright interest in the Specification."
  - **2.2 Attribution:** "As a condition of the copyright grant, you must include an attribution to the Specification in any derivative work you make based on the Specification. That attribution must include, at minimum, the Specification name and version number."
  - **3.1.1 The Promise (patent non-assert):** "I ... irrevocably promise not to assert my Granted Claims against you for your Permitted Uses, subject to the terms and conditions of Section 3.1." **3.2 Patent License Commitment:** "a non-exclusive, no charge, royalty free license to my Granted Claims on reasonable and non-discriminatory terms, where such license applies only to those Granted Claims infringed by the implementation of the Specification, solely for your Permitted Uses."
  - **3.1.2.1 Termination:** all rights are terminated "if you file, maintain, or voluntarily participate in a lawsuit against me or any person or entity asserting that its Permitted Uses infringe any Granted Claims you would have had the right to enforce had you signed this Agreement, unless that suit was in response to a corresponding suit first brought against you."
  - **3.1.3:** "This promise is not an assurance (i) that any of my copyrights or issued patent claims cover an implementation of the Specification or are enforceable or (ii) that an implementation of the Specification would not infringe intellectual property rights of any third party."
  - **8.6 Permitted Uses:** "making, using, selling, offering for sale, importing or distributing any implementation of the Specification 1) only to the extent it implements the Specification and 2) so long as all required portions of the Specification are implemented. Permitted Uses do not extend to any portion of an implementation that is not included in the Specification."
  - **8.4 Granted Claims:** "include only those patent claims that are infringed by the implementation of any portions of the Specification where the Specification describes the functionality causing the infringement in detail, requires such functionality, and does not merely reference the functionality causing the infringement"; they exclude claims infringed by "(a) any enabling technologies ... not themselves expressly set forth in the Specification" and "(d) reference or informational portions of the Specification."
  - The specification's own notice (v1.0 and v1.1, section 1): "OTHER RIGHTS NOT GRANTED AS SET FORTH ABOVE, INCLUDING WITHOUT LIMITATION, RIGHTS OF THIRD PARTIES WHO DID NOT EXECUTE THE ABOVE LICENSES, MAY BE IMPLICATED BY THE IMPLEMENTATION OF OR COMPLIANCE WITH THIS SPECIFICATION. OCP IS NOT RESPONSIBLE FOR IDENTIFYING RIGHTS FOR WHICH A LICENSE MAY BE REQUIRED IN ORDER TO IMPLEMENT THIS SPECIFICATION."
  - **My reading (not legal advice):** (1) **Copyright:** this crate copies no text and no code from the specification; it uses numbers and a formula. Where it is a derivative of the specification at all, 2.2 asks for the specification's name and version number, which it now carries (below). (2) **Patent:** the non-assert and the licence commitment reach only "Permitted Uses", which require implementing **all required portions** of the Specification; this crate implements only the element value rule of 5.3 (not the conversion and rounding obligations of 5.3.2/5.3.3 ("an implementation must support the roundTiesToEven rounding mode"), not the scale, not the dot products), so it **is probably not an implementation in the sense of 8.6 and should not rely on the OWFa promise for patent cover**. The patent sweep above and its limits are the only patent check this crate has. (3) I did **not** read an executed copy: OCP says to ask it for them, and neither OCP page I read lists a signatory for the MX specification. The newer OCP templates are "OWF FSA 0.9" and "OWF CLA 0.9" (not read; the MX documents name 1.0). The separate contribution agreement ("OWF CLA 1.0", contributors to OCP) is between the contributors and OCP; I read its template header only and did not rely on it.
- **Credit owed and given:** "OCP Microscaling Formats (MX) Specification, Version 1.0 (Sep 2023), Open Compute Project; authors from Microsoft, AMD, Arm, Intel, Meta, NVIDIA and Qualcomm", licensed under the Open Web Foundation Modified Final Specification Agreement (OWFa 1.0) as that specification states; v1.1 (Oct 9, 2026, adding Google as a contributor) has identical element tables. The crate README and the crate root documentation carry this line.
- No third-party code.

### Ideas swept (written from the source and the KISS clauses, not from a full read of KISS-Classify or the OCP specification)

1. A **4-bit floating-point element dtype**: sign 1, exponent 2, mantissa 1, bias 1, no infinity and no NaN, exact values 0, 0.5, 1, 1.5, 2, 3, 4, 6 and their negatives, with `-0` a code of its own (`f4e2m1`).
2. **6-bit floating-point element dtypes**: sign 1, exponent 2 or 3, mantissa 3 or 2, biases 1 and 3, no infinity and no NaN, largest finite values 7.5 and 28 (`f6e2m3`, `f6e3m2`).
3. A **byte packing** for those elements: two 4-bit codes per byte with the even logical index in the low nibble; four 6-bit codes per three bytes, least-significant bit first, `ceil(6n/8)` bytes, unused trailing bits required to be zero, with a reader that declines a buffer whose padding is set.
4. A rule that these narrow element dtypes **are storage and operand dtypes only and never an accumulator**, answered with a typed decline distinct from the unknown-token and reserved-token declines.
5. Decoding such a code to its value by **field extraction and the closed formula** `(-1)^s * 2^(e-bias) * (1 + m/2^M)` (subnormal for `e = 0`), exact in a 64-bit float.

### Queries (source, query, date 2026-11-13)

Web search restricted to `patents.google.com`, US-only, standard mode.

| # | Ideas | Query |
|---|---|---|
| Q13 | 1, 2 | `patent 4-bit floating point element format sign exponent mantissa E2M1 microscaling block shared scale quantization neural network` |
| Q14 | 3 | `patent packing 6-bit floating point values into bytes four elements three bytes memory layout low precision tensor storage` |
| Q15 | 4 | `patent low-precision floating point format operand restricted accumulate in higher precision format accumulator data type different from operand 4-bit 6-bit 8-bit matrix multiply` |

Plus one non-patent search (OCP MX specification licence) for the provenance above. **Not searched:** idea 5 as such (decoding by formula), and no claim-language or CPC-class query was run. Q13 to Q15 are single-pass, and most hits were papers or USPTO PDFs whose claims were not shown.

### Hits

| # | Patent | Title | Assignee | Status (Google) | Claims read | Overlap | Our difference | Verdict |
|---|---|---|---|---|---|---|---|---|
| 14 | US 2024/0160406 A1 | Low-precision floating-point datapath in a computer processor | Nvidia Corp | **Pending** (application) | 1, 4, 5, 6, 13, 20, 22, 25 (independent) | The nearest hit for ideas 1 and 2: **"VS-Quant FP4" floating-point operands** (claims 6, 20) and low-precision floating-point operands with a low-precision scale factor. | Every independent claim needs **a computer processor with multiplier units** that multiply two VS-Quant (FP4 or LOG4) operands into an **integer** result, adders that reduce the integer results, and logic that multiplies by **a low-precision floating-point scale factor** (or a method that adds exponents, multiplies mantissas, XORs signs and accumulates integer partial products). The crate defines a code's value and its byte packing; it multiplies, adds and scales nothing and is not a processor. | **clear** for this crate. Pending: re-check for a grant. |
| 15 | US 12,632,219 B2 | Quantization and low precision AI processor | Matx Inc | Active (Google) | 1, 16, 18 (independent) | A block quantisation format whose values hold **4-bit or lower fraction bits with shared scale bits** (the shape of an MX block, ideas 1 and 2). | The claims need **an AI-accelerating processor whose systolic array has at least 90% of its multipliers limited to 4-bit or lower input precision** and **a quantizer** that converts values from a first precision format into the block format. The crate has no systolic array, no multipliers and no quantizer, and it does not define the block (the per-block `f8e8m0` scale is not in it). | **clear** |
| 16 | US 2025/0291873 A1 | Universal Scale Metadata Layout for Matrix Multiply and Add (MMA) | Nvidia Corp (Google spells it "NVDIA") | **Pending** (application) | 1, 17, 18 (independent) | Scale metadata for narrow operands ("low precision floating-point" meaning 8 or fewer bits, such as 4/6/8) next to matrix operands. Already row 8 of this record (read for `fireline-dlpack`, verdict clear); re-read here against ideas 1 to 3. | The claims need **an MMA circuit** and **scale metadata block data structures** whose width `q` is set from a vector length, copied from global to device memory. Nothing about an element's bit layout, value rule or packing is claimed, and the crate has no circuit and no scale metadata. | **clear** for this crate. Pending: re-check for a grant. |
| 17 | US 8,280,936 B2 | Packed restricted floating point representation and logic for conversion to single precision float | Intel Corp | Expired, fee related; estimate 2030-03-20 (Google) | 1, 7 (independent) | A narrow floating-point storage format (**8 bits: 1 sign, 3 exponent, 4 fraction**) held in memory and expanded to a wider float. | The claims are **digital logic that populates the exponent bits of an expanded data structure by inverting and gating exponent bits** (an inverter, an OR gate and an AND gate). The formats here are 4 and 6 bits with different field widths, nothing is expanded in logic, and the status is expired. | **clear** |

**Surfaced and NOT read** (no verdict, never "clear"): US 8,874,794 B2 (block floating point compression with exponent difference and mantissa coding), US 7,216,138 B2 (floating point operations and format conversion), US 10,242,423 B2 (compute optimisations for low precision machine learning operations), US 10,338,919 B2 (generalised acceleration of matrix multiply accumulate), US 11,442,696 B1 (floating point multiply-add and accumulate unit with exception processing), US 2023/0359694 A1 (variable-precision matrix multiplication with low-precision digit matrix multiplier). The NVIDIA, Microsoft, AMD, Intel and Qualcomm families on narrow-precision and block-scaled formats are large; four patents is not a survey of them.

### Needs review (to the PM)

**No hit read on a claim, so no `[ASK]`.** This is a first pass: four patents read and six surfaced and unread, from three single-pass queries on the three most obvious phrasings. **It does not clear `fireline-classify` for publishing on its own.** The licence is now read (see Provenance): copyright is fine with the attribution given, but **the OWFa patent promise probably does not cover this crate** (it is not an implementation of all required portions of the Specification), so the patent position rests on this sweep alone, and an executed copy of the agreement was not seen. Owed to a person: (1) whether to rely on the OWFa for patent cover at all (it would need the crate to be an MX-compliant implementation, or counsel's view), and (2) a claim-language and CPC-class pass over the narrow-float-format and packing families before a release.

### Limitations specific to this addendum

- The idea list comes from the source and the KISS clauses; **I did not read the OCP MX specification or KISS-Classify in full**, so a mechanism described only there may be unswept.
- Three patent searches and four patents read. Statuses, expiry dates and fee states are Google's; no USPTO record, prosecution history or foreign family was checked.
- The OCP specification's agreement was read as a blank template (the executed copies are held by OCP and were not seen), and its text and my reading of it are not legal advice. v1.1 was read for the three element formats and the licence section only, not in full; the PDFs were read as extracted text (`pdftotext -layout`), whose symbols were partly garbled, so the table values above were checked against the numbers the KISS clauses quote.
