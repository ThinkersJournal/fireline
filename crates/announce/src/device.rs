// SPDX-License-Identifier: MIT OR Apache-2.0
//! The **device-query frames** of KISS-Announce §6.6: the request `DVRQ`, the device response `DVRS` (with its device and accuracy
//! records) and the device decline `DVDC`. The contract-query frames are in [`crate::frames`], which also holds the shared
//! [`FrameError`], the correlation field (6.4-0013) and the `decline_code` constants.
//!
//! **Draft-tracking.** KISS-Announce is a Draft and its wire is not frozen; expect rework. Each rule cites its clause id; the exact
//! bytes are diffed against KISS's `conformance/corpus/announce_vectors.json` in `tests/device.rs`.
//!
//! **Dtype spellings.** 6.6-0007 requires every dtype token to be a spelling of the KISS-Classify closed dtype table and not a reserved
//! one. That table is Classify's and moves with it, so this crate does not carry a copy: every encoder and decoder here takes an
//! `is_dtype` predicate that answers "is this an accepted, non-reserved spelling?". Everything else of 6.6-0006..-0008 (bounds, the
//! token alphabet, ascending sets, the subset rule, the accuracy key order and its dtype rule) is checked here.

use crate::frames::{FrameError, Reader, put_correlation};

/// `DVRQ`, the device-query request tag (6.6-0003).
pub const TAG_DVRQ: [u8; 4] = *b"DVRQ";
/// `DVRS`, the device response tag (6.6-0004).
pub const TAG_DVRS: [u8; 4] = *b"DVRS";
/// `DVDC`, the device decline tag (6.6-0005).
pub const TAG_DVDC: [u8; 4] = *b"DVDC";

/// Most devices in one `DVRS` (6.6-0004, `MAX_DEVICES`).
pub const MAX_DEVICES: u32 = 4096;
/// Longest `device_id` (6.6-0006).
pub const MAX_DEVICE_ID_LEN: usize = 4096;
/// Most tokens in one dtype set (6.6-0006).
pub const MAX_DTYPE_SET: usize = 64;
/// Most accuracy records in one device record (6.6-0006, `MAX_ACCURACY_RECORDS`).
pub const MAX_ACCURACY_RECORDS: u32 = 65536;
/// `max_ulp` value meaning the device declares **no bound** for the key (6.6-0008).
pub const NO_BOUND: u32 = 0xFFFF_FFFF;
/// `math_fidelity` `1`: `bit-stable` (6.6-0008).
pub const FIDELITY_BIT_STABLE: u8 = 1;
/// `math_fidelity` `2`: `reduced-mantissa-permitted` (6.6-0008).
pub const FIDELITY_REDUCED_MANTISSA: u8 = 2;

/// One accuracy claim of a device: for `(op, math_fidelity, dtype)` the largest ULP distance it guarantees (6.6-0008).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccuracyRecord {
    /// The KISS-Ops op token.
    pub op: String,
    /// [`FIDELITY_BIT_STABLE`] or [`FIDELITY_REDUCED_MANTISSA`].
    pub math_fidelity: u8,
    /// The dtype token; must be in the record's `compute_dtypes`.
    pub dtype: String,
    /// The bound, or [`NO_BOUND`].
    pub max_ulp: u32,
}

/// One device a provider enumerates (6.6-0006).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceRecord {
    /// Opaque bytes naming the device within this provider only, 1 to 4096 of them.
    pub device_id: Vec<u8>,
    /// The Classify target, `<namespace>:<capability>`.
    pub target: String,
    /// The dtypes the device can hold and move: strictly ascending bytewise.
    pub store_dtypes: Vec<String>,
    /// The dtypes it can compute on: strictly ascending, a subset of `store_dtypes`.
    pub compute_dtypes: Vec<String>,
    /// The accuracy claims, strictly ascending by `(op, math_fidelity, dtype)`.
    pub accuracy: Vec<AccuracyRecord>,
}

/// A device response `DVRS` (6.6-0004). A provider with no device answers with an empty `devices`, which is a genuine negative (6.6-0002).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceResponse {
    /// The enumerated devices.
    pub devices: Vec<DeviceRecord>,
    /// The correlation id echoed from the request, if it carried one (6.4-0013).
    pub correlation_id: Option<[u8; 8]>,
}

/// A device decline `DVDC` (6.6-0005).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceDecline {
    /// A [`crate::frames::decline`] value or one from the experimental or vendor range; an unknown one is a generic decline (6.4-0010).
    pub code: u32,
    /// The correlation id echoed from the request, if it carried one.
    pub correlation_id: Option<[u8; 8]>,
}

// ---- Shared checks ------------------------------------------------------------------------------------------------------

fn check_token(t: &str) -> Result<(), FrameError> {
    if t.is_empty() || t.len() > 255 || !t.bytes().all(|b| (0x21..=0x7E).contains(&b)) {
        return Err(FrameError::BadToken);
    }
    Ok(())
}

fn check_target(t: &str) -> Result<(), FrameError> {
    check_token(t)?;
    match t.split_once(':') {
        Some((ns, cap)) if !ns.is_empty() && !cap.is_empty() => Ok(()),
        _ => Err(FrameError::BadTarget),
    }
}

fn check_set(set: &[String], is_dtype: &dyn Fn(&str) -> bool) -> Result<(), FrameError> {
    if set.len() > MAX_DTYPE_SET {
        return Err(FrameError::DtypeSetTooLarge(set.len()));
    }
    for t in set {
        check_token(t)?;
        if !is_dtype(t) {
            return Err(FrameError::UnknownDtype(t.clone()));
        }
    }
    if set.windows(2).any(|w| w[0].as_bytes() >= w[1].as_bytes()) {
        return Err(FrameError::DtypeSetNotAscending);
    }
    Ok(())
}

/// Every rule of 6.6-0006, -0007 and -0008 that does not depend on the bytes: producers and readers both apply it.
fn check_record(d: &DeviceRecord, is_dtype: &dyn Fn(&str) -> bool) -> Result<(), FrameError> {
    if d.device_id.is_empty() || d.device_id.len() > MAX_DEVICE_ID_LEN {
        return Err(FrameError::DeviceIdLen(
            u32::try_from(d.device_id.len()).unwrap_or(u32::MAX),
        ));
    }
    check_target(&d.target)?;
    check_set(&d.store_dtypes, is_dtype)?;
    check_set(&d.compute_dtypes, is_dtype)?;
    if d.compute_dtypes.iter().any(|c| !d.store_dtypes.contains(c)) {
        return Err(FrameError::ComputeNotSubsetOfStore);
    }
    let count = u32::try_from(d.accuracy.len()).unwrap_or(u32::MAX);
    if count > MAX_ACCURACY_RECORDS {
        return Err(FrameError::AccuracyCountTooLarge(count));
    }
    for a in &d.accuracy {
        check_token(&a.op)?;
        check_token(&a.dtype)?;
        if !matches!(
            a.math_fidelity,
            FIDELITY_BIT_STABLE | FIDELITY_REDUCED_MANTISSA
        ) {
            return Err(FrameError::MathFidelity(a.math_fidelity));
        }
        if !d.compute_dtypes.contains(&a.dtype) {
            return Err(FrameError::AccuracyDtypeNotComputed(a.dtype.clone()));
        }
    }
    let key = |a: &AccuracyRecord| {
        (
            a.op.as_bytes().to_vec(),
            a.math_fidelity,
            a.dtype.as_bytes().to_vec(),
        )
    };
    if d.accuracy.windows(2).any(|w| key(&w[0]) >= key(&w[1])) {
        return Err(FrameError::AccuracyNotAscending);
    }
    Ok(())
}

// ---- Encoding -----------------------------------------------------------------------------------------------------------

fn put_token(out: &mut Vec<u8>, t: &str) {
    out.push(t.len() as u8);
    out.extend_from_slice(t.as_bytes());
}

fn put_set(out: &mut Vec<u8>, set: &[String]) {
    out.push(set.len() as u8);
    for t in set {
        put_token(out, t);
    }
}

/// Encode a device-query request `DVRQ` (6.6-0003): the tag and the correlation field.
#[must_use]
pub fn encode_dvrq(correlation_id: Option<[u8; 8]>) -> Vec<u8> {
    let mut out = TAG_DVRQ.to_vec();
    put_correlation(&mut out, correlation_id);
    out
}

/// Encode a device response `DVRS` (6.6-0004, -0006, -0007, -0008).
///
/// # Errors
/// A [`FrameError`] when the response or a record is outside what a producer must keep, including a dtype token `is_dtype` refuses.
pub fn encode_dvrs(
    r: &DeviceResponse,
    is_dtype: &dyn Fn(&str) -> bool,
) -> Result<Vec<u8>, FrameError> {
    let count = u32::try_from(r.devices.len()).unwrap_or(u32::MAX);
    if count > MAX_DEVICES {
        return Err(FrameError::DeviceCountTooLarge(count));
    }
    let mut out = TAG_DVRS.to_vec();
    out.extend_from_slice(&count.to_le_bytes());
    for d in &r.devices {
        check_record(d, is_dtype)?;
        out.extend_from_slice(&(d.device_id.len() as u32).to_le_bytes());
        out.extend_from_slice(&d.device_id);
        put_token(&mut out, &d.target);
        put_set(&mut out, &d.store_dtypes);
        put_set(&mut out, &d.compute_dtypes);
        out.extend_from_slice(&(d.accuracy.len() as u32).to_le_bytes());
        for a in &d.accuracy {
            put_token(&mut out, &a.op);
            out.push(a.math_fidelity);
            put_token(&mut out, &a.dtype);
            out.extend_from_slice(&a.max_ulp.to_le_bytes());
        }
    }
    put_correlation(&mut out, r.correlation_id);
    Ok(out)
}

/// Encode a device decline `DVDC` (6.6-0005). The caller must not pass the reserved code `0` (6.4-0009).
#[must_use]
pub fn encode_dvdc(r: &DeviceDecline) -> Vec<u8> {
    let mut out = TAG_DVDC.to_vec();
    out.extend_from_slice(&r.code.to_le_bytes());
    put_correlation(&mut out, r.correlation_id);
    out
}

// ---- Decoding -----------------------------------------------------------------------------------------------------------

impl Reader<'_> {
    fn token(&mut self) -> Result<String, FrameError> {
        let n = self.u8()? as usize;
        if n == 0 {
            return Err(FrameError::BadToken);
        }
        let b = self.take(n)?;
        if !b.iter().all(|c| (0x21..=0x7E).contains(c)) {
            return Err(FrameError::BadToken);
        }
        Ok(String::from_utf8_lossy(b).into_owned())
    }

    fn set(&mut self) -> Result<Vec<String>, FrameError> {
        let n = self.u8()? as usize;
        if n > MAX_DTYPE_SET {
            return Err(FrameError::DtypeSetTooLarge(n));
        }
        (0..n).map(|_| self.token()).collect()
    }
}

/// Decode a device-query request `DVRQ`; returns the correlation id and the extent (6.6-0003).
///
/// # Errors
/// A [`FrameError`] naming the first fault.
pub fn decode_dvrq(bytes: &[u8]) -> Result<(Option<[u8; 8]>, usize), FrameError> {
    let mut r = Reader { b: bytes, at: 0 };
    r.tag(TAG_DVRQ)?;
    let c = r.correlation()?;
    Ok((c, r.at))
}

/// Decode a device response `DVRS`; returns it and the extent (6.6-0004, -0006, -0007, -0008).
///
/// `is_dtype` answers whether a token is an accepted, non-reserved spelling of the Classify dtype table (see the module docs).
///
/// # Errors
/// A [`FrameError`] naming the first fault. Counts are checked against the bytes left before anything is allocated.
pub fn decode_dvrs(
    bytes: &[u8],
    is_dtype: &dyn Fn(&str) -> bool,
) -> Result<(DeviceResponse, usize), FrameError> {
    let mut r = Reader { b: bytes, at: 0 };
    r.tag(TAG_DVRS)?;
    let count = r.u32()?;
    if count > MAX_DEVICES {
        return Err(FrameError::DeviceCountTooLarge(count));
    }
    // The smallest device record is 4 + 1 + 2 + 1 + 1 + 4 bytes (a 1-byte id, a 1-char target, two empty sets, no accuracy).
    const MIN_DEVICE: usize = 4 + 1 + 2 + 1 + 1 + 4;
    if (count as usize).saturating_mul(MIN_DEVICE) > bytes.len().saturating_sub(r.at) {
        return Err(FrameError::Truncated);
    }
    let mut devices = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let id_len = r.u32()?;
        if id_len == 0 || id_len as usize > MAX_DEVICE_ID_LEN {
            return Err(FrameError::DeviceIdLen(id_len));
        }
        let device_id = r.take(id_len as usize)?.to_vec();
        let target = r.token()?;
        let store_dtypes = r.set()?;
        let compute_dtypes = r.set()?;
        let n = r.u32()?;
        if n > MAX_ACCURACY_RECORDS {
            return Err(FrameError::AccuracyCountTooLarge(n));
        }
        // The smallest accuracy record is a 2-byte op token, the fidelity byte, a 2-byte dtype token and 4 bytes of `max_ulp`.
        const MIN_ACCURACY: usize = 2 + 1 + 2 + 4;
        if (n as usize).saturating_mul(MIN_ACCURACY) > bytes.len().saturating_sub(r.at) {
            return Err(FrameError::Truncated);
        }
        let mut accuracy = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let op = r.token()?;
            let math_fidelity = r.u8()?;
            let dtype = r.token()?;
            let max_ulp = r.u32()?;
            accuracy.push(AccuracyRecord {
                op,
                math_fidelity,
                dtype,
                max_ulp,
            });
        }
        let d = DeviceRecord {
            device_id,
            target,
            store_dtypes,
            compute_dtypes,
            accuracy,
        };
        check_record(&d, is_dtype)?;
        devices.push(d);
    }
    let correlation_id = r.correlation()?;
    Ok((
        DeviceResponse {
            devices,
            correlation_id,
        },
        r.at,
    ))
}

/// Decode a device decline `DVDC`; returns it and the extent (6.6-0005). Any code is accepted: an unknown one is a generic decline.
///
/// # Errors
/// A [`FrameError`] naming the first fault.
pub fn decode_dvdc(bytes: &[u8]) -> Result<(DeviceDecline, usize), FrameError> {
    let mut r = Reader { b: bytes, at: 0 };
    r.tag(TAG_DVDC)?;
    let code = r.u32()?;
    let correlation_id = r.correlation()?;
    Ok((
        DeviceDecline {
            code,
            correlation_id,
        },
        r.at,
    ))
}
