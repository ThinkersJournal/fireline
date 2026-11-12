// SPDX-License-Identifier: MIT OR Apache-2.0
//! The kernel-seam **handshake**: the frozen 56-byte [`SeamHello`] envelope, the capability bits, and role-aware profile negotiation
//! between a kernel *consumer* (an orchestrator such as Fuel) and a kernel *provider* (a backend such as baracuda).
//!
//! A connection negotiates a profile **before any tensor or kernel crosses the seam**, so each side can support several profile versions
//! without a lockstep flag-day. KISS-Announce is the standard; this crate is the implementation both sides compile. It is dependency-free
//! on purpose: a provider that only negotiates must never pull in the region grammar or anything graph-shaped.
//!
//! # Hello world
//!
//! ```
//! use fireline_announce::{
//!     Role, SEAM_CAP_DLPACK_EXT_V1, SEAM_CAP_PROVISION_ON_REQUEST, SeamHello, negotiate_with_role,
//! };
//!
//! let consumer = SeamHello::profile_v1(SEAM_CAP_DLPACK_EXT_V1);
//! let provider = SeamHello::profile_v1(SEAM_CAP_DLPACK_EXT_V1 | SEAM_CAP_PROVISION_ON_REQUEST);
//! // Over a wire the provider's envelope arrives as 56 bytes: parse and validate it first.
//! let remote = SeamHello::from_wire_bytes(&provider.to_wire_bytes())?;
//! let n = negotiate_with_role(&consumer, &remote, Role::Consumer)?;
//! assert_eq!(n.profile, 1);
//! assert_ne!(n.capabilities & SEAM_CAP_PROVISION_ON_REQUEST, 0, "only the provider decides this bit");
//! # Ok::<(), fireline_announce::SeamError>(())
//! ```
//!
//! # Provenance
//!
//! Merged from `fuel-kernel-seam-announce` (the envelope, role-aware negotiation, the axis masks) and `baracuda-seam` (the wire codec and
//! the truncation error). See this crate's README.

pub mod frames;

/// `"SEAM"`: the envelope magic, never changes.
///
/// The numeric constant is `0x4D41_4553` so the little-endian on-wire bytes at offset 0 read `53 45 41 4D` = ASCII `SEAM` (KISS-ANNOUNCE
/// 6.1-0004). The big-endian spelling `0x5345_414D` would put `MAES` on the wire.
pub const SEAM_MAGIC: u32 = 0x4D41_4553;
/// The envelope's own version; designed never to bump.
pub const SEAM_ENVELOPE_VERSION: u8 = 1;
/// Fixed cap on simultaneously advertised profiles.
pub const SEAM_MAX_PROFILES: usize = 16;
/// Profile v1: the bundled interchange (DLPack extension) and kernel-contract version. The only profile that exists today.
pub const PROFILE_V1: u16 = 1;

// ---- Capability bits -------------------------------------------------------------------------------------------------
// Bits 0-5 (the DLPack-extension tokens) and bits 32-33 are the allocated, frozen bits. The negotiated word keeps `local & remote`
// on the EXT and SUB axes; the FEAT axis is the provider's word alone (KISS-ANNOUNCE-7.3-0003, see `negotiate_with_role`).
// Bits 34 (`KISC_FRAMING`) and 35 (`RECIPE_IMPORT`) are provisional in baracuda and are deliberately NOT allocated here: they enter
// this crate only when KISS Announce co-allocates them with a consumer on each side.

/// DLPack extension, base sidecar support (`DlpackExtV1`). Bit 0.
pub const SEAM_CAP_DLPACK_EXT_V1: u64 = 1 << 0;
/// DLPack extension, MX block-scaled quantisation (`DlpackExtMx`). Bit 1.
pub const SEAM_CAP_DLPACK_EXT_MX: u64 = 1 << 1;
/// DLPack extension, GGML inline-block quantisation (`DlpackExtGgml`). Bit 2.
pub const SEAM_CAP_DLPACK_EXT_GGML: u64 = 1 << 2;
/// DLPack extension, affine (int/float) quantisation (`DlpackExtAffine`). Bit 3.
pub const SEAM_CAP_DLPACK_EXT_AFFINE: u64 = 1 << 3;
/// DLPack extension, symbolic extents (`DlpackExtSymbolic`). Bit 4.
pub const SEAM_CAP_DLPACK_EXT_SYMBOLIC: u64 = 1 << 4;
/// DLPack extension, indexed residency / gather (`DlpackExtGather`). Bit 5.
pub const SEAM_CAP_DLPACK_EXT_GATHER: u64 = 1 << 5;

/// EXT axis, bits `[0, 24)`: external interchange tokens (what each side *speaks*).
pub const SEAM_CAP_EXT_MASK: u64 = (1 << 24) - 1;
/// FEAT axis, bits `[24, 48)`: provider-level optional features (what the provider *offers*).
pub const SEAM_CAP_FEAT_MASK: u64 = ((1 << 24) - 1) << 24;
/// SUB axis, bits `[48, 64)`: which sub-standards the provider speaks (presence only).
pub const SEAM_CAP_SUB_MASK: u64 = ((1 << 16) - 1) << 48;
/// The bits only the PROVIDER decides: the whole FEAT axis. KISS-ANNOUNCE-7.3-0003 masks by axis, not by recognition, so two readers
/// that recognise different FEAT bits still compute the same word.
pub const SEAM_CAP_PROVIDER_ONLY_MASK: u64 = SEAM_CAP_FEAT_MASK;
/// FEAT bit 32 `PROVISION_ON_REQUEST`: the provider builds a kernel on a miss.
pub const SEAM_CAP_PROVISION_ON_REQUEST: u64 = 1 << 32;
/// FEAT bit 33 `CONTRACT_QUERY`: the provider answers contract queries.
pub const SEAM_CAP_CONTRACT_QUERY: u64 = 1 << 33;

/// Every capability bit this crate has allocated: bits 0-5, 32 and 33. A side may advertise only these; see
/// [`SeamHello::unallocated_capabilities`].
pub const ALLOCATED_CAPABILITIES: u64 =
    0b11_1111 | SEAM_CAP_PROVISION_ON_REQUEST | SEAM_CAP_CONTRACT_QUERY;

/// Which side of the exchange the LOCAL envelope is. Known from the exchange itself, never inferred from a capability bit (the
/// envelope is frozen at 56 bytes and carries no role field).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// The local side asks for kernels.
    Consumer,
    /// The local side supplies kernels.
    Provider,
}

/// The negotiation envelope: a fixed-size C-ABI POD, frozen for all time. 56 bytes, 8-byte aligned; field offsets are frozen and asserted
/// by `tests/conformance.rs` and at compile time below. Providers fill a caller-allocated `SeamHello` through an out-parameter entry point
/// (baracuda's is `int baracuda_seam_hello(SeamHello* out)`, which stays in baracuda).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeamHello {
    /// `== SEAM_MAGIC`.
    pub magic: u32,
    /// `== SEAM_ENVELOPE_VERSION`.
    pub envelope_version: u8,
    /// `== 0`.
    pub reserved: [u8; 3],
    /// Number of valid entries in `profiles` (at most [`SEAM_MAX_PROFILES`]).
    pub profiles_len: u16,
    /// Strictly ascending; entries `[profiles_len..]` are 0.
    pub profiles: [u16; SEAM_MAX_PROFILES],
    /// The six bytes between `profiles` (ends at 42) and the 8-aligned `capabilities` (48), made an explicit field so they are managed:
    /// zeroed on write and hard-rejected when nonzero (KISS-ANNOUNCE 6.2-0011), rather than implicit padding. `== 0`.
    pub reserved1: [u8; 6],
    /// Optional-feature bitset within the selected profile.
    pub capabilities: u64,
}

// The envelope is frozen at 56 bytes and 8-byte alignment: a change here is a compile error, not a test failure somewhere downstream.
const _: () = assert!(core::mem::size_of::<SeamHello>() == 56);
const _: () = assert!(core::mem::align_of::<SeamHello>() == 8);

/// A typed handshake failure: never a panic, never a silent coercion. New variants may be added without a breaking release.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SeamError {
    /// The buffer is shorter than the 56-byte envelope.
    Truncated,
    /// `magic != SEAM_MAGIC`: not a seam envelope at all.
    BadMagic(u32),
    /// `envelope_version` is one this build cannot parse.
    UnknownEnvelope(u8),
    /// `profiles_len > SEAM_MAX_PROFILES`.
    TooManyProfiles(u16),
    /// The advertised profile list is not strictly ascending.
    NotAscending,
    /// A reserved field (`reserved` or `reserved1`) carried a nonzero byte.
    ReservedNonZero,
    /// No mutually supported profile: the connection does not proceed.
    VersionMismatch {
        /// The local side's profiles.
        local: Vec<u16>,
        /// The remote side's profiles.
        remote: Vec<u16>,
    },
}

impl core::fmt::Display for SeamError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SeamError::Truncated => write!(f, "seam: envelope shorter than 56 bytes"),
            SeamError::BadMagic(m) => {
                write!(
                    f,
                    "seam: bad magic 0x{m:08X} (expected 0x{SEAM_MAGIC:08X}): not a SeamHello envelope"
                )
            }
            SeamError::UnknownEnvelope(v) => {
                write!(
                    f,
                    "seam: envelope_version {v} unknown (this build speaks {SEAM_ENVELOPE_VERSION})"
                )
            }
            SeamError::TooManyProfiles(n) => write!(
                f,
                "seam: profiles_len {n} exceeds SEAM_MAX_PROFILES {SEAM_MAX_PROFILES}"
            ),
            SeamError::NotAscending => {
                write!(f, "seam: advertised profiles must be strictly ascending")
            }
            SeamError::ReservedNonZero => write!(
                f,
                "seam: a reserved field carried a nonzero byte (must be zero)"
            ),
            SeamError::VersionMismatch { local, remote } => write!(
                f,
                "seam: no mutually supported profile (local {local:?}, remote {remote:?}); connection does not proceed"
            ),
        }
    }
}

impl std::error::Error for SeamError {}

/// The result of a successful negotiation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Negotiated {
    /// The selected profile: the highest both sides support.
    pub profile: u16,
    /// The live capability word, role-aware (see [`negotiate_with_role`]).
    pub capabilities: u64,
}

impl SeamHello {
    /// Build an advertisement from an ascending profile list and capability bits. Never panics: a list longer than the cap is truncated
    /// (a caller bug surfaced by the `debug_assert`s rather than a release abort).
    pub fn advertise(profiles: &[u16], capabilities: u64) -> Self {
        debug_assert!(
            profiles.len() <= SEAM_MAX_PROFILES,
            "advertise: more than {SEAM_MAX_PROFILES} profiles"
        );
        debug_assert!(
            profiles.windows(2).all(|w| w[0] < w[1]),
            "advertise: profiles must be strictly ascending"
        );
        let n = profiles.len().min(SEAM_MAX_PROFILES);
        let mut arr = [0u16; SEAM_MAX_PROFILES];
        arr[..n].copy_from_slice(&profiles[..n]);
        Self {
            magic: SEAM_MAGIC,
            envelope_version: SEAM_ENVELOPE_VERSION,
            reserved: [0; 3],
            profiles_len: n as u16,
            profiles: arr,
            reserved1: [0; 6],
            capabilities,
        }
    }

    /// The standard advertisement today: Profile v1 with the given capability bits.
    pub fn profile_v1(capabilities: u64) -> Self {
        Self::advertise(&[PROFILE_V1], capabilities)
    }

    /// Validate the envelope and return the advertised profiles. A remote envelope MUST pass this before it is negotiated.
    pub fn validate(&self) -> Result<&[u16], SeamError> {
        if self.magic != SEAM_MAGIC {
            return Err(SeamError::BadMagic(self.magic));
        }
        if self.envelope_version != SEAM_ENVELOPE_VERSION {
            return Err(SeamError::UnknownEnvelope(self.envelope_version));
        }
        // Reserved bytes are pinned to zero: a foreign or garbage-padded envelope is hard-rejected, not silently accepted.
        if self.reserved != [0; 3] || self.reserved1 != [0; 6] {
            return Err(SeamError::ReservedNonZero);
        }
        let n = self.profiles_len as usize;
        if n > SEAM_MAX_PROFILES {
            return Err(SeamError::TooManyProfiles(self.profiles_len));
        }
        let live = &self.profiles[..n];
        if live.windows(2).any(|w| w[0] >= w[1]) {
            return Err(SeamError::NotAscending);
        }
        Ok(live)
    }

    /// Serialise to the 56-byte little-endian wire envelope, field by field (no struct cast).
    pub fn to_wire_bytes(&self) -> [u8; 56] {
        let mut b = [0u8; 56];
        b[0..4].copy_from_slice(&self.magic.to_le_bytes());
        b[4] = self.envelope_version;
        b[5..8].copy_from_slice(&self.reserved);
        b[8..10].copy_from_slice(&self.profiles_len.to_le_bytes());
        for (i, p) in self.profiles.iter().enumerate() {
            let at = 10 + i * 2;
            b[at..at + 2].copy_from_slice(&p.to_le_bytes());
        }
        b[42..48].copy_from_slice(&self.reserved1);
        b[48..56].copy_from_slice(&self.capabilities.to_le_bytes());
        b
    }

    /// Parse and validate a wire envelope from an untrusted peer, field by field (no struct cast, so it is a genuinely independent
    /// reader). Hard-rejects a buffer shorter than 56 bytes or any [`SeamHello::validate`] violation; never panics. Bytes past the
    /// first 56 are ignored.
    pub fn from_wire_bytes(bytes: &[u8]) -> Result<Self, SeamError> {
        let Some(b) = bytes.first_chunk::<56>() else {
            return Err(SeamError::Truncated);
        };
        let u16_at = |at: usize| u16::from_le_bytes([b[at], b[at + 1]]);
        let mut profiles = [0u16; SEAM_MAX_PROFILES];
        for (i, slot) in profiles.iter_mut().enumerate() {
            *slot = u16_at(10 + i * 2);
        }
        let hello = Self {
            magic: u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            envelope_version: b[4],
            reserved: [b[5], b[6], b[7]],
            profiles_len: u16_at(8),
            profiles,
            reserved1: [b[42], b[43], b[44], b[45], b[46], b[47]],
            capabilities: u64::from_le_bytes([
                b[48], b[49], b[50], b[51], b[52], b[53], b[54], b[55],
            ]),
        };
        hello.validate()?;
        Ok(hello)
    }

    /// The advertised capability bits that [`ALLOCATED_CAPABILITIES`] does not contain. A side must advertise only allocated bits, so
    /// its own test asserts this is `0`: a provisional bit then cannot be advertised by accident. This is about what a side SAYS; the
    /// negotiation itself still honours a remote's unknown FEAT bits.
    pub fn unallocated_capabilities(&self) -> u64 {
        self.capabilities & !ALLOCATED_CAPABILITIES
    }
}

/// The highest profile both validated envelopes advertise, or the typed hard failure.
fn select_profile(local: &SeamHello, remote: &SeamHello) -> Result<u16, SeamError> {
    let lp = local.validate()?;
    let rp = remote.validate()?;
    // Both lists are ascending: walk the local one high to low and take the first the remote also advertises.
    lp.iter()
        .rev()
        .find(|p| rp.contains(p))
        .copied()
        .ok_or_else(|| SeamError::VersionMismatch {
            local: lp.to_vec(),
            remote: rp.to_vec(),
        })
}

/// The highest mutually supported profile, and a **role-aware** live capability word,
/// `(local & remote & !FEAT) | (provider & FEAT)` with `FEAT` = [`SEAM_CAP_FEAT_MASK`] (KISS-ANNOUNCE-7.3-0003).
///
/// `local_role` is the role of the LOCAL envelope; the provider is known from the exchange, never from a capability bit. A consumer
/// writes the provider-only bits as zero (KISS-ANNOUNCE-7.3-0002), which is why a plain AND would lose them. The FEAT axis is masked as a
/// whole, not bit by bit. EXT and SUB bits are live only when both sides set them. Both envelopes are validated first; disjoint profile
/// sets hard-fail with [`SeamError::VersionMismatch`], never a guessed version.
pub fn negotiate_with_role(
    local: &SeamHello,
    remote: &SeamHello,
    local_role: Role,
) -> Result<Negotiated, SeamError> {
    let profile = select_profile(local, remote)?;
    let provider_word = match local_role {
        Role::Provider => local.capabilities,
        Role::Consumer => remote.capabilities,
    };
    Ok(Negotiated {
        profile,
        capabilities: (local.capabilities & remote.capabilities & !SEAM_CAP_PROVIDER_ONLY_MASK)
            | (provider_word & SEAM_CAP_PROVIDER_ONLY_MASK),
    })
}
