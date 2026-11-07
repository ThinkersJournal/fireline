// SPDX-License-Identifier: MIT OR Apache-2.0
//! Spec-stated sizes, computed sizes and frozen field offsets of the FDX sidecar structs, relocated from the inline `mod tests` of Fuel's `fuel-ir/src/dlpack/sidecar.rs` (Fuel `078e578a3540204f50ec402546c13b668bbfe4af`).
//! They use only public items, so they are the layout contract any consumer of the ABI can run.

use fireline_dlpack::sidecar::*;
use core::mem::{offset_of, size_of};

// ── Spec-STATED sizes (§5.4): these MUST equal the spec's numbers. ───────
#[cfg(target_pointer_width = "64")]
#[test]
fn spec_stated_sizes() {
    // §6.4 / §6.4.1: "EXACTLY 16 bytes" and "sizeof == 8 + 8 + 4*16 == 80".
    assert_eq!(
        size_of::<FDXAffineTerm>(),
        16,
        "spec §6.4: FDXAffineTerm == 16"
    );
    assert_eq!(size_of::<FDXAffine>(), 80, "spec §6.4: FDXAffine == 80");
}

// ── Computed-by-implementation sizes (§5.4 requires a pin; the numeric
//    value is NOT spec-stated and is reported back for cross-check). ──────
#[cfg(target_pointer_width = "64")]
#[test]
fn computed_sizes() {
    assert_eq!(size_of::<FDXDTypeExt>(), 16);
    assert_eq!(size_of::<FDXQuant>(), 100);
    assert_eq!(size_of::<FDXExtent>(), 128);
    assert_eq!(size_of::<FDXTiling>(), 48);
    assert_eq!(size_of::<FDXResidency>(), 32);
    assert_eq!(size_of::<FDXStorage>(), 32);
    assert_eq!(size_of::<FDXOutputView>(), 144);
    assert_eq!(size_of::<FDXBlockTable>(), 36);
    assert_eq!(size_of::<FDXIndexedResidency>(), 1064);
    assert_eq!(size_of::<FDXBufferRef>(), 160);
    assert_eq!(size_of::<FDXSymBinding>(), 16);
    assert_eq!(size_of::<FDXSymEnv>(), 16);
    assert_eq!(size_of::<FDXSidecar>(), 1376);
}

// ── FDXExtent field offsets — FROZEN per §6.4 layout rule. A future
//    field-order edit breaks the build here, not silently at the ABI. ─────
#[cfg(target_pointer_width = "64")]
#[test]
fn fdx_extent_field_offsets_are_frozen() {
    assert_eq!(offset_of!(FDXExtent, kind), 0);
    assert_eq!(offset_of!(FDXExtent, _pad), 1);
    assert_eq!(offset_of!(FDXExtent, min), 8);
    assert_eq!(offset_of!(FDXExtent, capacity), 16);
    assert_eq!(offset_of!(FDXExtent, sym_id), 24);
    assert_eq!(offset_of!(FDXExtent, sym_scope), 28);
    assert_eq!(offset_of!(FDXExtent, _pad2), 29);
    assert_eq!(offset_of!(FDXExtent, cap_kind), 32);
    assert_eq!(offset_of!(FDXExtent, _pad3), 33);
    assert_eq!(offset_of!(FDXExtent, _pad4), 36);
    assert_eq!(offset_of!(FDXExtent, affine), 40);
    assert_eq!(offset_of!(FDXExtent, reserved), 120);
}

// ── FDXIndexedResidency trailing-field offsets — pinned per §5.4 so a
//    future FDXExtent growth (which shifts everything after
//    `logical_extents`) breaks the build HERE too, not at the ABI. ────────
#[cfg(target_pointer_width = "64")]
#[test]
fn fdx_indexed_residency_trailing_offsets_are_pinned() {
    assert_eq!(offset_of!(FDXIndexedResidency, logical_extents), 256);
    assert_eq!(offset_of!(FDXIndexedResidency, context_lens_buffer), 1024);
    assert_eq!(offset_of!(FDXIndexedResidency, context_len_sym), 1028);
    assert_eq!(offset_of!(FDXIndexedResidency, context_len_scope), 1032);
    assert_eq!(offset_of!(FDXIndexedResidency, reserved), 1036);
}

// ── FDXSidecar embeds `gather` by value and ends with reserved[u64; 2]
//    (carved from the former reserved[8]); pin the gather offset so the
//    carve-out stays put. ──────────────────────────────────────────────
#[cfg(target_pointer_width = "64")]
#[test]
fn fdx_sidecar_gather_is_embedded_by_value() {
    // gather + reserved[2] (16 bytes) must account for the tail.
    assert_eq!(
        offset_of!(FDXSidecar, reserved),
        offset_of!(FDXSidecar, gather) + size_of::<FDXIndexedResidency>()
    );
    assert_eq!(
        size_of::<FDXSidecar>(),
        offset_of!(FDXSidecar, reserved) + 16
    );
}
