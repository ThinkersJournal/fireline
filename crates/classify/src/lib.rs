// SPDX-License-Identifier: MIT OR Apache-2.0
//! KISS-Classify **MX element dtypes**: `f4e2m1`, `f6e2m3` and `f6e3m2` (KISS-CLASSIFY-6.1-0014, -0015, -0016). Each has a value rule
//! ([`MxElement::decode`]), a byte packing ([`MxElement::pack`], [`MxElement::unpack`]) and the rule that it is a storage and operand
//! dtype only, never an accumulator ([`check_accumulator`]).
//!
//! The format definitions follow the OCP Microscaling Formats (MX) Specification, Version 1.0 (Sep 2023), Open Compute Project, by authors
//! from Microsoft, AMD, Arm, Intel, Meta, NVIDIA and Qualcomm (licensed under the Open Web Foundation Modified Final Specification Agreement,
//! OWFa 1.0, as that specification states); v1.1 has identical tables for these formats. Only the format numbers and the value formula are
//! used. This crate is not an MX-compliant implementation: it converts nothing and has no scale.
//!
//! **Draft-tracking.** KISS-Classify is a Draft; expect rework when it moves. Every rule cites its clause id. Converting a float *into*
//! one of these formats (round to nearest even, saturation) is owned by KISS-Ops and KISS-Emit, not by 6.1-0014..-0016, and is not here.
//!
//! # Hello world
//!
//! ```
//! use fireline_classify::MxElement;
//!
//! // Eight FP4 codes, two per byte (low nibble = even index), decode to the OCP E2M1 values.
//! let bytes = MxElement::F4E2M1.pack(&[0, 1, 2, 3, 4, 5, 6, 7])?;
//! assert_eq!(bytes, [0x10, 0x32, 0x54, 0x76]);
//! let codes = MxElement::F4E2M1.unpack(&bytes, 8)?;
//! let values: Vec<f64> = codes.iter().map(|&c| MxElement::F4E2M1.decode(c).unwrap()).collect();
//! assert_eq!(values, [0.0, 0.5, 1.0, 1.5, 2.0, 3.0, 4.0, 6.0]);
//! # Ok::<(), fireline_classify::MxError>(())
//! ```

use core::fmt;

/// An MX **element** dtype (KISS-CLASSIFY-6.1-0014, -0015). The per-block scale of an MX-encoded operand is the sibling `f8e8m0`
/// operand and is not part of this type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MxElement {
    /// `f4e2m1`: sign 1, exponent 2, mantissa 1, bias 1 (6.1-0014).
    F4E2M1,
    /// `f6e2m3`: sign 1, exponent 2, mantissa 3, bias 1 (6.1-0015).
    F6E2M3,
    /// `f6e3m2`: sign 1, exponent 3, mantissa 2, bias 3 (6.1-0015).
    F6E3M2,
}

/// A fault in a code, a packed buffer or an accumulator position. Each is a typed decline; never a panic, never a repair.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum MxError {
    /// A code that does not fit the element's width (for example `16` for `f4e2m1`).
    CodeOutOfRange {
        /// The element.
        element: MxElement,
        /// The offending code.
        code: u8,
    },
    /// A packed buffer too short for the requested number of elements.
    Truncated {
        /// Bytes needed.
        need: usize,
        /// Bytes supplied.
        have: usize,
    },
    /// A 6-bit packed buffer whose unused trailing bits are not zero (6.1-0015: "unused trailing bits MUST be zero").
    TrailingBitsNonZero,
    /// An MX element in an accumulator position (6.1-0016): a typed decline **distinct** from the unknown-token and reserved-token
    /// declines of 6.1-0001. KISS names it `ElementDtypeAsAccumulator`.
    ElementDtypeAsAccumulator(MxElement),
}

impl MxError {
    /// A stable snake-case tag for the decline. `element_dtype_as_accumulator` is KISS's name (`ElementDtypeAsAccumulator`); the
    /// others are this crate's own (the Draft names none).
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::CodeOutOfRange { .. } => "code_out_of_range",
            Self::Truncated { .. } => "truncated",
            Self::TrailingBitsNonZero => "trailing_bits_non_zero",
            Self::ElementDtypeAsAccumulator(_) => "element_dtype_as_accumulator",
        }
    }
}

impl fmt::Display for MxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CodeOutOfRange { element, code } => {
                write!(f, "code_out_of_range: {code} for `{}`", element.token())
            }
            Self::Truncated { need, have } => {
                write!(f, "truncated: need {need} bytes, have {have}")
            }
            Self::TrailingBitsNonZero => write!(f, "trailing_bits_non_zero"),
            Self::ElementDtypeAsAccumulator(e) => write!(
                f,
                "element_dtype_as_accumulator: `{}` is a storage and operand dtype only",
                e.token()
            ),
        }
    }
}

impl std::error::Error for MxError {}

impl MxElement {
    /// The three MX element dtypes.
    pub const ALL: [Self; 3] = [Self::F4E2M1, Self::F6E2M3, Self::F6E3M2];

    /// The element for a Classify dtype token, or `None` for any other token (including the other 24).
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|e| e.token() == token)
    }

    /// The Classify dtype token (6.1-0001).
    #[must_use]
    pub fn token(self) -> &'static str {
        match self {
            Self::F4E2M1 => "f4e2m1",
            Self::F6E2M3 => "f6e2m3",
            Self::F6E3M2 => "f6e3m2",
        }
    }

    /// Bits per element: the dtype's storage width (4 or 6).
    #[must_use]
    pub fn bits(self) -> u32 {
        match self {
            Self::F4E2M1 => 4,
            Self::F6E2M3 | Self::F6E3M2 => 6,
        }
    }

    /// Exponent field width.
    #[must_use]
    pub fn exponent_bits(self) -> u32 {
        match self {
            Self::F4E2M1 | Self::F6E2M3 => 2,
            Self::F6E3M2 => 3,
        }
    }

    /// Mantissa field width.
    #[must_use]
    pub fn mantissa_bits(self) -> u32 {
        match self {
            Self::F6E2M3 => 3,
            Self::F6E3M2 => 2,
            Self::F4E2M1 => 1,
        }
    }

    /// Exponent bias: 1, 1 and 3 (6.1-0014, -0015).
    #[must_use]
    pub fn bias(self) -> i32 {
        match self {
            Self::F4E2M1 | Self::F6E2M3 => 1,
            Self::F6E3M2 => 3,
        }
    }

    /// The largest finite magnitude: 6, 7.5 and 28 (6.1-0014, -0015). There is no infinity and no NaN.
    #[must_use]
    pub fn max_finite(self) -> f64 {
        match self {
            Self::F4E2M1 => 6.0,
            Self::F6E2M3 => 7.5,
            Self::F6E3M2 => 28.0,
        }
    }

    /// The value of a code: `(-1)^s * 2^(e - bias) * (1 + m / 2^M)` for `e != 0` and `(-1)^s * 2^(1 - bias) * (m / 2^M)` for `e == 0`
    /// (6.1-0014, -0015). Exact in `f64`. There is no infinity and no NaN encoding, so every code in range decodes to a finite value.
    ///
    /// The sign is the top bit, then the exponent, then the mantissa; `-0` is a code of its own (for `f4e2m1`, code `8`).
    ///
    /// # Errors
    /// [`MxError::CodeOutOfRange`] when `code` does not fit [`MxElement::bits`].
    pub fn decode(self, code: u8) -> Result<f64, MxError> {
        let bits = self.bits();
        if u32::from(code) >= 1 << bits {
            return Err(MxError::CodeOutOfRange {
                element: self,
                code,
            });
        }
        let (e_bits, m_bits) = (self.exponent_bits(), self.mantissa_bits());
        let m = u32::from(code) & ((1 << m_bits) - 1);
        let e = (u32::from(code) >> m_bits) & ((1 << e_bits) - 1);
        let sign = if (code >> (bits - 1)) & 1 == 1 {
            -1.0
        } else {
            1.0
        };
        let frac = f64::from(m) / f64::from(1u32 << m_bits);
        let magnitude = if e == 0 {
            2f64.powi(1 - self.bias()) * frac
        } else {
            2f64.powi(e as i32 - self.bias()) * (1.0 + frac)
        };
        Ok(sign * magnitude)
    }

    /// The packed size in bytes of `n` elements: `ceil(bits * n / 8)` (two per byte for `f4e2m1`; four per three bytes for the 6-bit
    /// formats).
    #[must_use]
    pub fn packed_len(self, n: usize) -> usize {
        (self.bits() as usize * n).div_ceil(8)
    }

    /// Pack element codes. `f4e2m1`: two per byte, the low nibble the even logical index and the high nibble the odd one, exactly as
    /// `i4` and `u4` (6.1-0014, 6.1-0008). `f6e2m3` and `f6e3m2`: four per three bytes, **least-significant bit first** (bit `k` of the
    /// stream is bit `k mod 8` of byte `k / 8`; element `i` occupies stream bits `6i ..= 6i + 5`), `ceil(6n / 8)` bytes, unused
    /// trailing bits zero (6.1-0015). For an odd `f4e2m1` count the unused high nibble is written as zero.
    ///
    /// # Errors
    /// [`MxError::CodeOutOfRange`] for a code that does not fit the width.
    pub fn pack(self, codes: &[u8]) -> Result<Vec<u8>, MxError> {
        let bits = self.bits() as usize;
        let mut out = vec![0u8; self.packed_len(codes.len())];
        for (i, &c) in codes.iter().enumerate() {
            if u32::from(c) >= 1 << bits {
                return Err(MxError::CodeOutOfRange {
                    element: self,
                    code: c,
                });
            }
            for b in 0..bits {
                if (c >> b) & 1 == 1 {
                    let k = i * bits + b;
                    out[k / 8] |= 1 << (k % 8);
                }
            }
        }
        Ok(out)
    }

    /// Unpack `n` element codes from `bytes`; the inverse of [`MxElement::pack`]. Bytes past [`MxElement::packed_len`] are not read.
    /// A 6-bit buffer whose unused trailing bits are not zero is rejected (6.1-0015). The Draft pins no value for the unused high
    /// nibble of an odd-length `f4e2m1` buffer, so it is ignored there.
    ///
    /// # Errors
    /// [`MxError::Truncated`] when `bytes` is shorter than `packed_len(n)`; [`MxError::TrailingBitsNonZero`] as above.
    pub fn unpack(self, bytes: &[u8], n: usize) -> Result<Vec<u8>, MxError> {
        let bits = self.bits() as usize;
        let need = self.packed_len(n);
        if bytes.len() < need {
            return Err(MxError::Truncated {
                need,
                have: bytes.len(),
            });
        }
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let mut c = 0u8;
            for b in 0..bits {
                let k = i * bits + b;
                c |= ((bytes[k / 8] >> (k % 8)) & 1) << b;
            }
            out.push(c);
        }
        if bits == 6 {
            let used = bits * n;
            let pad_ok = (used..need * 8).all(|k| (bytes[k / 8] >> (k % 8)) & 1 == 0);
            if !pad_ok {
                return Err(MxError::TrailingBitsNonZero);
            }
        }
        Ok(out)
    }
}

/// Whether a dtype token may be an **accumulator** (the contraction `<acc>`, the `(acc + mp)` field's `<acc>`, or an OpAttrs
/// `accumulator`; 6.1-0016). An MX element is a storage and operand dtype only.
///
/// Only the three MX tokens are judged here. Any other token returns `Ok`: whether it is a known, usable Classify dtype is a different
/// question (6.1-0001) with its own declines, which this crate does not own.
///
/// # Errors
/// [`MxError::ElementDtypeAsAccumulator`] for `f4e2m1`, `f6e2m3` and `f6e3m2`.
pub fn check_accumulator(token: &str) -> Result<(), MxError> {
    match MxElement::from_token(token) {
        Some(e) => Err(MxError::ElementDtypeAsAccumulator(e)),
        None => Ok(()),
    }
}
