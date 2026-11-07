// SPDX-License-Identifier: MIT OR Apache-2.0
//! Byte-for-byte size pins and flag bits of the standard DLPack structs, relocated from the inline `mod tests` of Fuel's `fuel-ir/src/dlpack/abi.rs` (Fuel `078e578a3540204f50ec402546c13b668bbfe4af`).
//! They use only public items, so they are the layout contract any consumer of the ABI can run.

use core::mem::size_of;
use fireline_dlpack::abi::*;

// ABI byte-for-byte size pins (64-bit LE target, §5). A struct-layout
// change that would break the C-header match fails the build here.
#[cfg(target_pointer_width = "64")]
#[test]
fn dlpack_struct_sizes_match_c_header() {
    assert_eq!(size_of::<DLDevice>(), 8);
    assert_eq!(size_of::<DLDataType>(), 4);
    assert_eq!(size_of::<DLPackVersion>(), 8);
    assert_eq!(size_of::<DLTensor>(), 48);
    assert_eq!(size_of::<DLManagedTensorVersioned>(), 80);
}

#[test]
fn dlpack_standard_flag_bits() {
    assert_eq!(DLPACK_FLAG_BITMASK_READ_ONLY, 1);
    assert_eq!(DLPACK_FLAG_BITMASK_IS_COPIED, 2);
    assert_eq!(DLPACK_FLAG_BITMASK_IS_SUBBYTE_TYPE_PADDED, 4);
}
