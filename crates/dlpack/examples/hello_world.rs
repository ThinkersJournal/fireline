//! Hello world: build an FDX sidecar header and run the first validator (V1) on it. No device, no tensor data.

use core::mem::size_of;
use fireline_dlpack::codes::{FDX_MAGIC, FDX_VERSION_1};
use fireline_dlpack::sidecar::FDXSidecar;
use fireline_dlpack::validate::{FdxValidationError, check_v1_header};

fn main() {
    // `FDXSidecar` is a plain `#[repr(C)]` struct of integers and raw pointers, so an all-zero value is a valid starting point.
    // SAFETY: every field of `FDXSidecar` is valid when all bits are zero (no references, no niches).
    let mut sidecar: FDXSidecar = unsafe { core::mem::zeroed() };

    // A blank sidecar is not an FDX sidecar: the validator says so with a typed error, never a panic.
    match check_v1_header(&sidecar) {
        Err(FdxValidationError::BadMagic { got, expected }) => {
            println!("blank sidecar rejected: magic {got:#010x}, expected {expected:#010x}");
        }
        other => panic!("expected BadMagic, got {other:?}"),
    }

    // Fill in the header the way a producer does, and it passes.
    sidecar.magic = FDX_MAGIC;
    sidecar.version = FDX_VERSION_1;
    sidecar.struct_bytes = size_of::<FDXSidecar>() as u32;
    assert_eq!(check_v1_header(&sidecar), Ok(()));
    println!(
        "header accepted: version {}, struct_bytes {}",
        sidecar.version, sidecar.struct_bytes
    );
}
