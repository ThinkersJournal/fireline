// SPDX-License-Identifier: MIT OR Apache-2.0
//! The **message frames** of KISS-Announce §6.3 and §6.4: the availability list `SAVL`, the contract-query request `CYRQ`, the contract
//! response `CRSP` and the decline response `CDEC`. (The device-query frames `DVRQ`, `DVRS` and `DVDC` of §6.6 are not here yet.)
//!
//! **Draft-tracking.** KISS-Announce is a Draft and its wire is not frozen (the freeze gate is unmet); this module follows it, expect
//! rework when it moves. Each rule cites its clause id. The exact bytes are diffed against KISS's
//! `conformance/corpus/announce_vectors.json` in `tests/frames.rs`.
//!
//! Every decoder takes the bytes as received, computes the message's extent **from the message alone** (6.2-0014), ignores whatever
//! follows it (6.2-0017) and returns the extent it read. A declared length is checked against the bytes left *before* anything is
//! allocated (6.2-0015), and no decoder panics. A decoder never repairs (6.2-0008).

use core::fmt;

/// `SAVL`, the availability-list tag (6.3-0011).
pub const TAG_SAVL: [u8; 4] = *b"SAVL";
/// `CYRQ`, the contract-query request tag (6.4-0001).
pub const TAG_CYRQ: [u8; 4] = *b"CYRQ";
/// `CRSP`, the contract response tag (6.4-0004).
pub const TAG_CRSP: [u8; 4] = *b"CRSP";
/// `CDEC`, the decline response tag (6.4-0007).
pub const TAG_CDEC: [u8; 4] = *b"CDEC";

/// The `list_version` a producer writes (6.3-0012). `1` and `2` are retired and declined.
pub const LIST_VERSION: u8 = 3;
/// Longest `structure_key` (6.3-0009).
pub const MAX_STRUCTURE_KEY_LEN: usize = 4096;
/// Longest `op_identity` (6.3-0013).
pub const MAX_OP_IDENTITY_LEN: usize = 4096;
/// Most records in one availability list: 2^20 (6.3-0010).
pub const MAX_AVAILABILITY_RECORDS: u32 = 1 << 20;
/// Shortest `revision_hash` (6.3-0003).
pub const MIN_REVISION_HASH_LEN: usize = 8;
/// Longest `revision_hash` (6.3-0003).
pub const MAX_REVISION_HASH_LEN: usize = 64;
/// Longest `CRSP` payload: 2^24 (6.4-0014). A placeholder in the Draft, revisable before any freeze.
pub const MAX_CONTRACT_PAYLOAD_LEN: u32 = 1 << 24;

/// `decline_code` values pinned by 6.4-0009. A consumer that meets any other value treats it as a generic decline (6.4-0010).
pub mod decline {
    /// No such cell, and none can be provisioned.
    pub const UNKNOWN_STRUCTURE_KEY: u32 = 1;
    /// Cell known but build-on-miss is unavailable or failed.
    pub const CANNOT_PROVISION: u32 = 2;
    /// Request framing invalid.
    pub const MALFORMED_REQUEST: u32 = 3;
    /// The provider does not advertise the contract-query capability.
    pub const QUERY_NOT_SUPPORTED: u32 = 4;
    /// The requested or negotiated version is unsupported.
    pub const VERSION_UNSUPPORTED: u32 = 5;
    /// The `structure_key` is known but the requested `revision_hash` is not held.
    pub const UNKNOWN_REVISION: u32 = 6;
}

/// A typed frame failure: never a panic, never a repair. New variants may be added without a breaking release.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum FrameError {
    /// The message runs past the bytes supplied, or there are too few to tell its extent (6.2-0015).
    Truncated,
    /// The leading four bytes are none of the tags this module reads (6.2-0016), or not the one the decoder expects.
    UnknownTag([u8; 4]),
    /// `list_version` is not [`LIST_VERSION`] (6.3-0012; `1` and `2` are retired).
    UnsupportedListVersion(u8),
    /// One of the three MBZ bytes after `list_version` is nonzero (6.3-0012).
    MbzNonZero,
    /// `record_count` exceeds [`MAX_AVAILABILITY_RECORDS`] (6.3-0010).
    RecordCountTooLarge(u32),
    /// A `structure_key` length of `0` or above [`MAX_STRUCTURE_KEY_LEN`] (6.3-0009).
    StructureKeyLen(u32),
    /// An `op_identity` length of `0` or above [`MAX_OP_IDENTITY_LEN`] (6.3-0013).
    OpIdentityLen(u32),
    /// A `revision_hash` length outside 8 to 64 (6.3-0003, `RevisionHashLenOutOfRange`).
    RevisionHashLenOutOfRange(usize),
    /// A `revision_present` byte other than `0` or `1` (6.4-0001).
    RevisionPresent(u8),
    /// A `correlation_len` other than `0` or `8` (6.4-0013; the answer to a request is the `MALFORMED_REQUEST` decline).
    CorrelationLen(u8),
    /// A `CRSP` payload length above [`MAX_CONTRACT_PAYLOAD_LEN`] (6.4-0014).
    PayloadTooLarge(u32),
}

impl FrameError {
    /// The 6.4-0009 `decline_code` a provider answers a malformed request with. Only a request's framing faults map to
    /// `MALFORMED_REQUEST`; a truncated or unknown-tag message gets no answer frame, as its extent is unknown (6.2-0015).
    #[must_use]
    pub fn decline_code(&self) -> Option<u32> {
        match self {
            Self::StructureKeyLen(_)
            | Self::OpIdentityLen(_)
            | Self::RevisionHashLenOutOfRange(_)
            | Self::RevisionPresent(_)
            | Self::CorrelationLen(_) => Some(decline::MALFORMED_REQUEST),
            _ => None,
        }
    }
}

impl fmt::Display for FrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated => write!(f, "announce: message truncated"),
            Self::UnknownTag(t) => write!(f, "announce: unexpected leading tag {t:02X?}"),
            Self::UnsupportedListVersion(v) => write!(f, "announce: list_version {v} unsupported"),
            Self::MbzNonZero => write!(f, "announce: a must-be-zero byte is nonzero"),
            Self::RecordCountTooLarge(n) => write!(
                f,
                "announce: record_count {n} above {MAX_AVAILABILITY_RECORDS}"
            ),
            Self::StructureKeyLen(n) => write!(
                f,
                "announce: structure_key length {n} outside 1..={MAX_STRUCTURE_KEY_LEN}"
            ),
            Self::OpIdentityLen(n) => write!(
                f,
                "announce: op_identity length {n} outside 1..={MAX_OP_IDENTITY_LEN}"
            ),
            Self::RevisionHashLenOutOfRange(n) => {
                write!(f, "announce: revision_hash length {n} outside 8..=64")
            }
            Self::RevisionPresent(b) => write!(f, "announce: revision_present {b} is not 0 or 1"),
            Self::CorrelationLen(b) => write!(f, "announce: correlation_len {b} is not 0 or 8"),
            Self::PayloadTooLarge(n) => write!(
                f,
                "announce: payload length {n} above {MAX_CONTRACT_PAYLOAD_LEN}"
            ),
        }
    }
}

impl std::error::Error for FrameError {}

/// One availability record: the identity triple and nothing else (6.3-0001, 6.3-0002).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AvailabilityRecord {
    /// The opaque KISS-Classify `structure_key` (6.3-0004).
    pub structure_key: Vec<u8>,
    /// The opaque `op_identity` (6.3-0013).
    pub op_identity: Vec<u8>,
    /// The opaque `revision_hash`, 8 to 64 bytes, compared byte-for-byte (6.3-0003, -0007, -0008).
    pub revision_hash: Vec<u8>,
}

/// The identity block of a `CYRQ`, `CRSP` or `CDEC` (6.4-0001, 6.4-0011), including the correlation field (6.4-0013).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identity {
    /// The opaque `structure_key`.
    pub structure_key: Vec<u8>,
    /// The opaque `op_identity`.
    pub op_identity: Vec<u8>,
    /// `Some` writes `revision_present = 1` and the hash; `None` writes `0`.
    pub revision_hash: Option<Vec<u8>>,
    /// `Some` writes `correlation_len = 8` and the id; `None` writes the single byte `00`. A response echoes its request's.
    pub correlation_id: Option<[u8; 8]>,
}

/// A contract response `CRSP`: the echoed identity and the KISS-Contract document bytes (6.4-0004).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContractResponse {
    /// The identity answered for, with the revision the provider selected.
    pub identity: Identity,
    /// The self-delimiting KISS-Contract document, at most [`MAX_CONTRACT_PAYLOAD_LEN`] bytes.
    pub payload: Vec<u8>,
}

/// A decline response `CDEC`: the echoed identity and a `decline_code` (6.4-0007).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclineResponse {
    /// The identity answered for.
    pub identity: Identity,
    /// A [`decline`] value or one from the experimental or vendor range (6.4-0009).
    pub code: u32,
}

// ---- Encoding ----------------------------------------------------------------------------------------------------------

fn put_len_bytes(
    out: &mut Vec<u8>,
    bytes: &[u8],
    max: usize,
    err: fn(u32) -> FrameError,
) -> Result<(), FrameError> {
    if bytes.is_empty() || bytes.len() > max {
        return Err(err(u32::try_from(bytes.len()).unwrap_or(u32::MAX)));
    }
    out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(bytes);
    Ok(())
}

fn put_revision(out: &mut Vec<u8>, hash: &[u8]) -> Result<(), FrameError> {
    if !(MIN_REVISION_HASH_LEN..=MAX_REVISION_HASH_LEN).contains(&hash.len()) {
        return Err(FrameError::RevisionHashLenOutOfRange(hash.len()));
    }
    out.push(hash.len() as u8);
    out.extend_from_slice(hash);
    Ok(())
}

fn put_identity(out: &mut Vec<u8>, id: &Identity) -> Result<(), FrameError> {
    put_len_bytes(
        out,
        &id.structure_key,
        MAX_STRUCTURE_KEY_LEN,
        FrameError::StructureKeyLen,
    )?;
    put_len_bytes(
        out,
        &id.op_identity,
        MAX_OP_IDENTITY_LEN,
        FrameError::OpIdentityLen,
    )?;
    match &id.revision_hash {
        Some(h) => {
            out.push(1);
            put_revision(out, h)?;
        }
        None => out.push(0),
    }
    match id.correlation_id {
        Some(c) => {
            out.push(8);
            out.extend_from_slice(&c);
        }
        None => out.push(0),
    }
    Ok(())
}

/// Encode an availability list `SAVL` (6.3-0011, -0012, -0005).
///
/// # Errors
/// A [`FrameError`] when a record or the count is outside the bounds a producer must keep.
pub fn encode_savl(records: &[AvailabilityRecord]) -> Result<Vec<u8>, FrameError> {
    let count = u32::try_from(records.len()).unwrap_or(u32::MAX);
    if count > MAX_AVAILABILITY_RECORDS {
        return Err(FrameError::RecordCountTooLarge(count));
    }
    let mut out = Vec::new();
    out.extend_from_slice(&TAG_SAVL);
    out.extend_from_slice(&[LIST_VERSION, 0, 0, 0]);
    out.extend_from_slice(&count.to_le_bytes());
    for r in records {
        put_len_bytes(
            &mut out,
            &r.structure_key,
            MAX_STRUCTURE_KEY_LEN,
            FrameError::StructureKeyLen,
        )?;
        put_len_bytes(
            &mut out,
            &r.op_identity,
            MAX_OP_IDENTITY_LEN,
            FrameError::OpIdentityLen,
        )?;
        put_revision(&mut out, &r.revision_hash)?;
    }
    Ok(out)
}

/// Encode a contract-query request `CYRQ` (6.4-0001).
///
/// # Errors
/// A [`FrameError`] when the identity is outside the bounds a producer must keep.
pub fn encode_cyrq(id: &Identity) -> Result<Vec<u8>, FrameError> {
    let mut out = TAG_CYRQ.to_vec();
    put_identity(&mut out, id)?;
    Ok(out)
}

/// Encode a contract response `CRSP` (6.4-0004, -0014).
///
/// # Errors
/// A [`FrameError`] when the identity or the payload length is outside the bounds a producer must keep.
pub fn encode_crsp(r: &ContractResponse) -> Result<Vec<u8>, FrameError> {
    let len = u32::try_from(r.payload.len()).unwrap_or(u32::MAX);
    if len > MAX_CONTRACT_PAYLOAD_LEN {
        return Err(FrameError::PayloadTooLarge(len));
    }
    let mut out = TAG_CRSP.to_vec();
    put_identity(&mut out, &r.identity)?;
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(&r.payload);
    Ok(out)
}

/// Encode a decline response `CDEC` (6.4-0007). The caller must not pass the reserved code `0` (6.4-0009); this encoder does not check it.
///
/// # Errors
/// A [`FrameError`] when the identity is outside the bounds a producer must keep.
pub fn encode_cdec(r: &DeclineResponse) -> Result<Vec<u8>, FrameError> {
    let mut out = TAG_CDEC.to_vec();
    put_identity(&mut out, &r.identity)?;
    out.extend_from_slice(&r.code.to_le_bytes());
    Ok(out)
}

// ---- Decoding ----------------------------------------------------------------------------------------------------------

struct Reader<'a> {
    b: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], FrameError> {
        let end = self.at.checked_add(n).ok_or(FrameError::Truncated)?;
        let s = self.b.get(self.at..end).ok_or(FrameError::Truncated)?;
        self.at = end;
        Ok(s)
    }

    fn u8(&mut self) -> Result<u8, FrameError> {
        Ok(self.take(1)?[0])
    }

    fn u32(&mut self) -> Result<u32, FrameError> {
        let s = self.take(4)?;
        Ok(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
    }

    fn tag(&mut self, want: [u8; 4]) -> Result<(), FrameError> {
        let s = self.take(4)?;
        let got = [s[0], s[1], s[2], s[3]];
        if got == want {
            Ok(())
        } else {
            Err(FrameError::UnknownTag(got))
        }
    }

    /// A u32 length in `[1, max]` then that many bytes; the length is checked against the bytes left before any copy (6.3-0009).
    fn len_bytes(&mut self, max: usize, err: fn(u32) -> FrameError) -> Result<Vec<u8>, FrameError> {
        let n = self.u32()?;
        if n == 0 || n as usize > max {
            return Err(err(n));
        }
        Ok(self.take(n as usize)?.to_vec())
    }

    fn revision(&mut self) -> Result<Vec<u8>, FrameError> {
        let n = self.u8()? as usize;
        if !(MIN_REVISION_HASH_LEN..=MAX_REVISION_HASH_LEN).contains(&n) {
            return Err(FrameError::RevisionHashLenOutOfRange(n));
        }
        Ok(self.take(n)?.to_vec())
    }

    fn identity(&mut self) -> Result<Identity, FrameError> {
        let structure_key = self.len_bytes(MAX_STRUCTURE_KEY_LEN, FrameError::StructureKeyLen)?;
        let op_identity = self.len_bytes(MAX_OP_IDENTITY_LEN, FrameError::OpIdentityLen)?;
        let revision_hash = match self.u8()? {
            0 => None,
            1 => Some(self.revision()?),
            b => return Err(FrameError::RevisionPresent(b)),
        };
        let correlation_id = match self.u8()? {
            0 => None,
            8 => {
                let s = self.take(8)?;
                let mut c = [0u8; 8];
                c.copy_from_slice(s);
                Some(c)
            }
            b => return Err(FrameError::CorrelationLen(b)),
        };
        Ok(Identity {
            structure_key,
            op_identity,
            revision_hash,
            correlation_id,
        })
    }
}

/// Decode an availability list `SAVL`; returns the records and the message's extent in bytes (6.2-0014, -0017).
///
/// # Errors
/// A [`FrameError`] naming the first fault; a truncated list is [`FrameError::Truncated`], never completed (6.2-0015).
pub fn decode_savl(bytes: &[u8]) -> Result<(Vec<AvailabilityRecord>, usize), FrameError> {
    let mut r = Reader { b: bytes, at: 0 };
    r.tag(TAG_SAVL)?;
    let version = r.u8()?;
    let mbz = r.take(3)?;
    if version != LIST_VERSION {
        return Err(FrameError::UnsupportedListVersion(version));
    }
    if mbz != [0, 0, 0] {
        return Err(FrameError::MbzNonZero);
    }
    let count = r.u32()?;
    if count > MAX_AVAILABILITY_RECORDS {
        return Err(FrameError::RecordCountTooLarge(count));
    }
    // The smallest record is 4 + 1 + 4 + 1 + 1 + 8 bytes; a count the bytes cannot hold is truncated before any allocation (6.3-0010).
    const MIN_RECORD: usize = 4 + 1 + 4 + 1 + 1 + MIN_REVISION_HASH_LEN;
    if (count as usize).saturating_mul(MIN_RECORD) > bytes.len().saturating_sub(r.at) {
        return Err(FrameError::Truncated);
    }
    let mut records = Vec::with_capacity(count as usize);
    for _ in 0..count {
        records.push(AvailabilityRecord {
            structure_key: r.len_bytes(MAX_STRUCTURE_KEY_LEN, FrameError::StructureKeyLen)?,
            op_identity: r.len_bytes(MAX_OP_IDENTITY_LEN, FrameError::OpIdentityLen)?,
            revision_hash: r.revision()?,
        });
    }
    Ok((records, r.at))
}

/// Decode a contract-query request `CYRQ`; returns the identity and the extent (6.4-0001).
///
/// # Errors
/// A [`FrameError`] naming the first fault; see [`FrameError::decline_code`] for the answer a provider owes.
pub fn decode_cyrq(bytes: &[u8]) -> Result<(Identity, usize), FrameError> {
    let mut r = Reader { b: bytes, at: 0 };
    r.tag(TAG_CYRQ)?;
    let id = r.identity()?;
    Ok((id, r.at))
}

/// Decode a contract response `CRSP`; returns it and the extent (6.4-0004, -0014).
///
/// # Errors
/// A [`FrameError`] naming the first fault. A payload length above the cap is rejected before any copy, supplied or not.
pub fn decode_crsp(bytes: &[u8]) -> Result<(ContractResponse, usize), FrameError> {
    let mut r = Reader { b: bytes, at: 0 };
    r.tag(TAG_CRSP)?;
    let identity = r.identity()?;
    let n = r.u32()?;
    if n > MAX_CONTRACT_PAYLOAD_LEN {
        return Err(FrameError::PayloadTooLarge(n));
    }
    let payload = r.take(n as usize)?.to_vec();
    Ok((ContractResponse { identity, payload }, r.at))
}

/// Decode a decline response `CDEC`; returns it and the extent (6.4-0007). Any `code` is accepted: an unrecognised one is a generic
/// decline, never a fault (6.4-0010).
///
/// # Errors
/// A [`FrameError`] naming the first fault.
pub fn decode_cdec(bytes: &[u8]) -> Result<(DeclineResponse, usize), FrameError> {
    let mut r = Reader { b: bytes, at: 0 };
    r.tag(TAG_CDEC)?;
    let identity = r.identity()?;
    let code = r.u32()?;
    Ok((DeclineResponse { identity, code }, r.at))
}
