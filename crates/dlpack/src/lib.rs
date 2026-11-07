// SPDX-License-Identifier: MIT OR Apache-2.0
//! FDX, the DLPack extension: a versioned, optional sidecar over standard DLPack for tensor interchange between a kernel consumer (an
//! orchestrator such as Fuel) and a kernel provider (a backend such as baracuda or Vulkane).
//!
//! - [`abi`]: the standard DLPack structs (`DLDevice`, `DLDataType`, `DLTensor`, `DLManagedTensorVersioned`).
//! - [`codes`]: the FDX magic, version, flags, dtype, quantisation, granularity, substrate, gather and extent codes and sentinels. This module
//!   is the normative owner of those codes (the FKC kernel-contract format references them by symbol).
//! - [`sidecar`]: `FDXSidecar` and its sub-structs, all `#[repr(C)]` with frozen sizes and offsets.
//! - [`validate`]: the V-numbered validators, as `Result`s with a local error type.
//!
//! The co-maintained C header is `include/fuel_dlpack_ext.h`; a unit test cross-checks its `sizeof` static asserts and a sample of its
//! `#define`s against the Rust definitions (no C compiler needed).
//!
//! **This crate implements FDX, not "the" quantisation sidecar.** KISS adopted a neutral ten-field successor (KISS-CLASSIFY-6.3-0012, an
//! informative annex whose wire form is still undecided); a projection to it is a later module, after KISS pins that form.
//!
//! What stays with the consumer: the conversion from its own dtype, shape and storage types to these structs (in Fuel, `fuel-ir`'s
//! `dlpack::convert`), because it names the consumer's types.

pub mod abi;
pub mod codes;
pub mod sidecar;
pub mod validate;

/// Header and Rust drift gate: cross-checks `include/fuel_dlpack_ext.h` `sizeof` static asserts (and a sample of `#define`s) against the Rust
/// `#[repr(C)]` definitions.
#[cfg(test)]
mod header_check;
