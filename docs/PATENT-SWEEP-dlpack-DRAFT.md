
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
