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

**Not searched at all:** idea 4 (advertise only allocated bits), idea 11 (ULP band), and idea 1's reserved-bytes mechanism specifically. The Q1 to Q4 results are single-pass; none of them is a claim-language or CPC-class search.

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

### Design gate (binding on later releases)

Rows 1 and 2 are "clear" **only because the crates contain no selection step and no kernel library.** `fireline-provider` (plan section 10, vendor-library providers announcing **prebuilt entry points**) is precisely a precompiled, processor-specific kernel in a library, and a consumer that chooses among providers per operation is the "choosing a type of processing element" step. **No FireLine crate may take on either step without re-reading US 8,458,680 and US 8,108,844 first**, and that re-read goes to the PM before release. The same applies to Fuel's own Judge; that is Fuel's record.

### Surfaced by the searches and NOT read (claims unread, so no verdict, never "clear")

US 4,953,210 A (feature negotiation protocol, synchronous modem; a 1980s family, status not checked); US 8,010,778 B2 (negotiating a capability in a peer-to-peer link); US 2021/0294602 A1 and its continuation US 2023/0350673 A1 (microkernel-based optimisation of neural networks; mentions a JIT compiler); US 2024/0028886 A1 (graph optimisation for neural-network computation); WO 2020/263588 A1 (composable neural network kernels); US 12,373,257 B2 (static scheduling with a cost model); US 2016/0267622 A1 (automated compute kernel fusion); US 2013/0091507 A1 (fused and split kernels for GPUs); US 9,524,323 (delegation of expression tree evaluation; the closest thing Q4 returned for idea 10); and the Fuel-table rows marked "inherited". Most were surfaced by one search each and some look unrelated; they are listed so the next read pass starts from them, not because any overlaps.

## Needs review (to the PM)

1. **US 2023/0025245 A1** (Nvidia, pending): see row 3. Re-check the USPTO record for a grant and read the granted claims before the first publish; Fuel already carries it as medium for Fuel as a whole.
2. **The design gate** above, for any future crate that selects a processing element or serves prebuilt kernels.

## Limitations

- AI-assisted search and reading; not legal advice; not a freedom-to-operate opinion; a lawyer's review is a separate purchase.
- The idea list is written by the lane from the code and its own design notes; a mechanism nobody wrote down is not swept. **Ideas 4 and 11 and the reserved-bytes part of idea 1 were not searched.**
- Search covered one query per idea group through a US-only web index limited to `patents.google.com`; USPTO, Espacenet and WIPO were not searched directly; no CPC-class or inventor search was run. A search that finds nothing proves nothing.
- Only **five** patents were read by this lane, independent claims only. **Nine of Fuel's twelve rows and every "surfaced, not read" patent were not read by me.** Claim text is machine-converted; check the official record before relying on a quotation.
- Statuses, expiry dates and fee states are Google's display and estimates; **no USPTO record, prosecution history, term adjustment, maintenance-fee state or foreign family was checked.**
- The comparison is a static reading of the three crates at the swept branches; nothing was run for this purpose.
- Pacing: 4 searches and 5 claim fetches, spread across the session; no HTTP 503 was seen.
