// SPDX-License-Identifier: MIT OR Apache-2.0
//! The **document framing** of KISS-Contract §6.11: the pinned header line (6.11-0002), the inner length and CRC-32 (6.11-0003,
//! -0003a, -0003b) and the seven section heading lines (6.11-0004), with the loud-failure discipline of 6.1-0002, -0003 and -0004.
//!
//! This is the framing only. The **field lines** inside each section (6.11-0005 onwards: field order, keys and value encodings) are
//! handed back as raw bytes in [`Document::sections`]; they are not interpreted here.
//!
//! **Draft-tracking.** KISS-Contract is a Draft; expect rework when it moves. The exact bytes are diffed against KISS
//! `conformance/corpus/contract_vectors.json` (origin/main 73c341b5) in `tests/document.rs`.
//!
//! **Extent.** [`parse_document`] takes the buffer that *is* the document (for example a `CRSP` payload of
//! `fireline_announce`, KISS-CONTRACT-6.1-0005). Bytes past `header + N` make the body longer than `N` and are rejected
//! (6.11-0003: "a document whose body is not exactly `N` bytes").

use core::fmt;

/// The `contract_version` of the schema this crate follows: the decimal token `4` (6.1-0008).
pub const CONTRACT_VERSION: u64 = 4;
/// The only recognised `contract_kind` (6.1-0007, 6.11-0002).
pub const CONTRACT_KIND: &str = "kiss-contract";
/// The magic that begins every document (6.11-0002): ASCII `KISC`.
pub const MAGIC: [u8; 4] = *b"KISC";
/// Largest declared body length: 2^24 (6.11-0003a), a placeholder revisable before any freeze.
pub const MAX_DOCUMENT_BODY_LEN: u64 = 1 << 24;
/// The seven section names, in the fixed order of 6.11-0004 (ids `1` to `7`).
pub const SECTION_NAMES: [&str; 7] = [
    "identity",
    "semantics",
    "interface",
    "dispatch",
    "capabilities",
    "guarantees",
    "provenance",
];
/// No header line this crate accepts is longer than this; the search for its LF stops here (no work keyed on unchecked input).
const MAX_HEADER_LINE: usize = 128;

/// A framing fault. Each is a typed decline (6.1-0002, -0003, -0004); a reader never repairs.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DocumentError {
    /// The input is shorter than the magic or does not begin with `KISC` (6.11-0002).
    NoMagic,
    /// The header line is absent, unterminated, not LF-terminated, or not the pinned form (6.11-0002).
    MalformedHeader,
    /// The `contract_kind` is not [`CONTRACT_KIND`] (6.1-0002, 6.1-0007).
    UnknownKind,
    /// A `contract_version` outside the set the reader supports (6.1-0003). Carries the declared value, saturated at `u64::MAX`.
    UnknownVersion(u64),
    /// The declared `len=` is above [`MAX_DOCUMENT_BODY_LEN`], or a decimal too large to hold (6.11-0003a); rejected before any work.
    LengthOverCap,
    /// The body is not exactly the declared `len=` bytes (6.11-0003).
    BadLength {
        /// The declared `N`.
        declared: usize,
        /// The body bytes actually present.
        present: usize,
    },
    /// The body's CRC-32 does not match the header's (6.11-0003).
    BadChecksum {
        /// The `crc32=` in the header.
        declared: u32,
        /// The CRC-32 of the body.
        computed: u32,
    },
    /// Body content before the first section heading, or no heading at all (6.11-0004; a *headingless* block is never imported).
    Headingless,
    /// The seven section headings are not exactly `identity` .. `provenance` once each and in order (6.11-0004).
    SectionHeadings,
}

impl DocumentError {
    /// The **stable category tag** KISS's `contract_vectors.json` pins for each decline (`expect`), so a foreign reader can match by
    /// category. Faults the corpus has no vector for (a misordered heading list) carry a tag of this crate's own choosing.
    #[must_use]
    pub fn wire_tag(&self) -> &'static str {
        match self {
            Self::NoMagic => "no-magic",
            Self::MalformedHeader => "malformed-header",
            Self::UnknownKind => "unknown-kind",
            Self::UnknownVersion(_) => "unknown-version",
            Self::LengthOverCap | Self::BadLength { .. } => "bad-length",
            Self::BadChecksum { .. } => "bad-checksum",
            Self::Headingless => "headingless",
            Self::SectionHeadings => "bad-section-headings",
        }
    }
}

impl fmt::Display for DocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoMagic => write!(f, "contract document: no KISC magic"),
            Self::MalformedHeader => write!(f, "contract document: malformed header line"),
            Self::UnknownKind => write!(f, "contract document: unrecognised contract_kind"),
            Self::UnknownVersion(v) => {
                write!(f, "contract document: contract_version {v} unsupported")
            }
            Self::LengthOverCap => write!(
                f,
                "contract document: declared length above {MAX_DOCUMENT_BODY_LEN}"
            ),
            Self::BadLength { declared, present } => write!(
                f,
                "contract document: declared body length {declared}, {present} present"
            ),
            Self::BadChecksum { declared, computed } => write!(
                f,
                "contract document: crc32 {declared:08x} declared, {computed:08x} computed"
            ),
            Self::Headingless => write!(f, "contract document: no first section heading"),
            Self::SectionHeadings => write!(
                f,
                "contract document: section headings are not the pinned seven in order"
            ),
        }
    }
}

impl std::error::Error for DocumentError {}

/// A framed contract document: its version and the seven section blocks as raw field-line bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Document<'a> {
    /// The `contract_version` from the header line (one of the versions the reader declared it supports).
    pub contract_version: u64,
    /// The whole body: every byte after the header line's LF.
    pub body: &'a [u8],
    /// For each of the seven sections in order, the bytes after its heading line up to the next heading (or the end of the body).
    /// Not interpreted here (6.11-0005 and the section field schemas).
    pub sections: [&'a [u8]; 7],
}

/// CRC-32, IEEE 802.3 polynomial, reflected, initial `0xFFFFFFFF`, final XOR `0xFFFFFFFF` (6.11-0003).
#[must_use]
pub fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in bytes {
        crc ^= u32::from(b);
        for _ in 0..8 {
            crc = if crc & 1 == 1 {
                (crc >> 1) ^ 0xEDB8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

fn decimal(s: &[u8]) -> Option<u64> {
    if s.is_empty() || !s.iter().all(u8::is_ascii_digit) {
        return None;
    }
    // A digit string too large for u64 saturates: it is a value no reader supports and no body can have.
    Some(s.iter().fold(0u64, |acc, d| {
        acc.saturating_mul(10).saturating_add(u64::from(d - b'0'))
    }))
}

fn lower_hex8(s: &[u8]) -> Option<u32> {
    if s.len() != 8 || !s.iter().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
        return None;
    }
    Some(s.iter().fold(0u32, |acc, b| {
        acc << 4 | char::from(*b).to_digit(16).unwrap_or(0)
    }))
}

/// Frame a contract document: the pinned header line (6.11-0002), the inner `len=` and `crc32=` (6.11-0003), then `body`.
///
/// The body is written as given: it should already be the seven section blocks of 6.11-0004, which this encoder does not build.
///
/// # Errors
/// [`DocumentError::LengthOverCap`] when `body` is longer than [`MAX_DOCUMENT_BODY_LEN`] (6.11-0003a).
pub fn encode_document(body: &[u8]) -> Result<Vec<u8>, DocumentError> {
    if body.len() as u64 > MAX_DOCUMENT_BODY_LEN {
        return Err(DocumentError::LengthOverCap);
    }
    let mut out = format!(
        "KISC {CONTRACT_KIND} {CONTRACT_VERSION} len={} crc32={:08x}\n",
        body.len(),
        crc32(body)
    )
    .into_bytes();
    out.extend_from_slice(body);
    Ok(out)
}

/// Frame-check a contract document and split it into its seven sections.
///
/// `supported_versions` is the set of `contract_version` values the reader supports; support is declared and enumerated, never
/// inferred from an ordering (6.1-0008), so a value outside the set is [`DocumentError::UnknownVersion`] even when it is lower.
/// The checks run in this order, and the first failure names the decline: magic, header line, `contract_kind`, version, the length
/// cap (before any work keyed on `N`), the body length, the CRC-32, then the section headings.
///
/// # Errors
/// A [`DocumentError`] naming the first fault. Never panics, never allocates on a declared length.
pub fn parse_document<'a>(
    bytes: &'a [u8],
    supported_versions: &[u64],
) -> Result<Document<'a>, DocumentError> {
    if !bytes.starts_with(&MAGIC) {
        return Err(DocumentError::NoMagic);
    }
    let window = &bytes[..bytes.len().min(MAX_HEADER_LINE)];
    let lf = window
        .iter()
        .position(|&b| b == b'\n')
        .ok_or(DocumentError::MalformedHeader)?;
    let header = &window[..lf];
    let body = &bytes[lf + 1..];

    // `KISC <kind> <version> len=<N> crc32=<HHHHHHHH>`: five tokens, single spaces, no other byte.
    let tokens: Vec<&[u8]> = header.split(|&b| b == b' ').collect();
    let [_magic, kind, version, len, crc] = tokens[..] else {
        return Err(DocumentError::MalformedHeader);
    };
    let version = decimal(version).ok_or(DocumentError::MalformedHeader)?;
    let declared_len = len
        .strip_prefix(b"len=")
        .and_then(decimal)
        .ok_or(DocumentError::MalformedHeader)?;
    let declared_crc = crc
        .strip_prefix(b"crc32=")
        .and_then(lower_hex8)
        .ok_or(DocumentError::MalformedHeader)?;
    if kind != CONTRACT_KIND.as_bytes() {
        return Err(DocumentError::UnknownKind);
    }
    if !supported_versions.contains(&version) {
        return Err(DocumentError::UnknownVersion(version));
    }
    if declared_len > MAX_DOCUMENT_BODY_LEN {
        return Err(DocumentError::LengthOverCap);
    }
    // `declared_len <= 2^24` now, so it fits a usize on every target.
    let declared = declared_len as usize;
    if body.len() != declared {
        return Err(DocumentError::BadLength {
            declared,
            present: body.len(),
        });
    }
    let computed = crc32(body);
    if computed != declared_crc {
        return Err(DocumentError::BadChecksum {
            declared: declared_crc,
            computed,
        });
    }
    let sections = split_sections(body)?;
    Ok(Document {
        contract_version: version,
        body,
        sections,
    })
}

/// The heading line `[section:<id>:<name>]` and its LF, as bytes.
fn heading(id: usize) -> Vec<u8> {
    format!("[section:{}:{}]\n", id + 1, SECTION_NAMES[id]).into_bytes()
}

fn split_sections(body: &[u8]) -> Result<[&[u8]; 7], DocumentError> {
    // Content before the first heading, or no heading line at all, is a headingless block (6.11-0004).
    if !body.starts_with(b"[section:") {
        return Err(DocumentError::Headingless);
    }
    // Every heading line starts at the beginning of a line.
    let starts: Vec<usize> = (0..body.len())
        .filter(|&i| (i == 0 || body[i - 1] == b'\n') && body[i..].starts_with(b"[section:"))
        .collect();
    if starts.len() != SECTION_NAMES.len() {
        return Err(DocumentError::SectionHeadings);
    }
    let mut out: [&[u8]; 7] = [&[]; 7];
    for (id, &at) in starts.iter().enumerate() {
        let want = heading(id);
        if !body[at..].starts_with(&want) {
            return Err(DocumentError::SectionHeadings);
        }
        let from = at + want.len();
        let to = starts.get(id + 1).copied().unwrap_or(body.len());
        out[id] = &body[from..to];
    }
    Ok(out)
}
