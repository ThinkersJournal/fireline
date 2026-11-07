//! RED STUB: real types and constants, deliberately wrong function bodies, so `tests/conformance.rs` must fail.

/// `"SEAM"` on a little-endian wire.
pub const SEAM_MAGIC: u32 = 0x4D41_4553;
/// The envelope's own version.
pub const SEAM_ENVELOPE_VERSION: u8 = 1;
/// Fixed cap on simultaneously advertised profiles.
pub const SEAM_MAX_PROFILES: usize = 16;
/// Profile v1.
pub const PROFILE_V1: u16 = 1;

/// FDX base sidecar support.
pub const SEAM_CAP_DLPACK_EXT_V1: u64 = 1 << 0;
/// MX block-scaled quant.
pub const SEAM_CAP_DLPACK_EXT_MX: u64 = 1 << 1;
/// GGML inline-block quant.
pub const SEAM_CAP_DLPACK_EXT_GGML: u64 = 1 << 2;
/// Affine quant.
pub const SEAM_CAP_DLPACK_EXT_AFFINE: u64 = 1 << 3;
/// Symbolic extents.
pub const SEAM_CAP_DLPACK_EXT_SYMBOLIC: u64 = 1 << 4;
/// Indexed residency / gather.
pub const SEAM_CAP_DLPACK_EXT_GATHER: u64 = 1 << 5;
/// EXT axis.
pub const SEAM_CAP_EXT_MASK: u64 = (1 << 24) - 1;
/// FEAT axis.
pub const SEAM_CAP_FEAT_MASK: u64 = ((1 << 24) - 1) << 24;
/// SUB axis.
pub const SEAM_CAP_SUB_MASK: u64 = ((1 << 16) - 1) << 48;
/// Bits only the provider decides.
pub const SEAM_CAP_PROVIDER_ONLY_MASK: u64 = SEAM_CAP_FEAT_MASK;
/// Provider builds a kernel on a miss.
pub const SEAM_CAP_PROVISION_ON_REQUEST: u64 = 1 << 32;
/// Provider answers contract queries.
pub const SEAM_CAP_CONTRACT_QUERY: u64 = 1 << 33;
/// Every bit this crate has allocated (stub: wrong).
pub const ALLOCATED_CAPABILITIES: u64 = 0;

/// Which side the local envelope is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// Asks for kernels.
    Consumer,
    /// Supplies kernels.
    Provider,
}

/// The frozen 56-byte envelope.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeamHello {
    /// `SEAM_MAGIC`.
    pub magic: u32,
    /// `SEAM_ENVELOPE_VERSION`.
    pub envelope_version: u8,
    /// Zero.
    pub reserved: [u8; 3],
    /// Valid entries in `profiles`.
    pub profiles_len: u16,
    /// Ascending profiles.
    pub profiles: [u16; SEAM_MAX_PROFILES],
    /// Zero.
    pub reserved1: [u8; 6],
    /// Capability bitset.
    pub capabilities: u64,
}

/// A typed handshake failure.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SeamError {
    /// Fewer than 56 bytes.
    Truncated,
    /// Wrong magic.
    BadMagic(u32),
    /// Unknown envelope version.
    UnknownEnvelope(u8),
    /// `profiles_len` over the cap.
    TooManyProfiles(u16),
    /// Profiles not strictly ascending.
    NotAscending,
    /// A reserved byte was nonzero.
    ReservedNonZero,
    /// No common profile.
    VersionMismatch {
        /// Local profiles.
        local: Vec<u16>,
        /// Remote profiles.
        remote: Vec<u16>,
    },
}

/// The negotiation result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Negotiated {
    /// Selected profile.
    pub profile: u16,
    /// Live capabilities.
    pub capabilities: u64,
}

impl SeamHello {
    /// STUB.
    pub fn advertise(_profiles: &[u16], _capabilities: u64) -> Self {
        Self { magic: 0, envelope_version: 0, reserved: [0; 3], profiles_len: 0, profiles: [0; SEAM_MAX_PROFILES], reserved1: [0; 6], capabilities: 0 }
    }
    /// STUB.
    pub fn profile_v1(capabilities: u64) -> Self {
        Self::advertise(&[PROFILE_V1], capabilities)
    }
    /// STUB.
    pub fn validate(&self) -> Result<&[u16], SeamError> {
        Ok(&self.profiles[..0])
    }
    /// STUB.
    pub fn to_wire_bytes(&self) -> [u8; 56] {
        [0; 56]
    }
    /// STUB.
    pub fn from_wire_bytes(_bytes: &[u8]) -> Result<Self, SeamError> {
        Err(SeamError::Truncated)
    }
    /// STUB.
    pub fn unallocated_capabilities(&self) -> u64 {
        0
    }
}

/// STUB.
pub fn negotiate_with_role(_local: &SeamHello, _remote: &SeamHello, _local_role: Role) -> Result<Negotiated, SeamError> {
    Ok(Negotiated { profile: 0, capabilities: 0 })
}
