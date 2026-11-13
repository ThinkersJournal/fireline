// SPDX-License-Identifier: MIT OR Apache-2.0
//! KISS-CLASSIFY-6.1-0014, -0015, -0016: the MX element dtypes' value rule, packing and accumulator rule, written from the Draft's text
//! (KISS origin/main 73c341b5). The format constants (bias, widths, the limits below) are the ones the clauses cite from OCP MX v1.0.
//! Draft-tracking: expect rework when the Draft moves.

use fireline_classify::{MxElement, MxError, check_accumulator};

// KISS `conformance/corpus/dtype_manifest.json` (KISS-CLASSIFY-6.1-0001): token, storage_bits.
const MANIFEST: [(&str, u32); 3] = [("f4e2m1", 4), ("f6e2m3", 6), ("f6e3m2", 6)];

/// A `f6e3m2` code from its fields: sign (1 bit), exponent (3 bits), mantissa (2 bits).
fn f6e3m2(sign: u8, exp: u8, man: u8) -> u8 {
    sign << 5 | exp << 2 | man
}

#[test]
fn tokens_and_widths_match_the_dtype_manifest() {
    for (token, bits) in MANIFEST {
        let e = MxElement::from_token(token).expect(token);
        assert_eq!((e.token(), e.bits()), (token, bits));
        assert_eq!(
            e.exponent_bits() + e.mantissa_bits() + 1,
            bits,
            "{token}: sign + exponent + mantissa"
        );
    }
    for other in ["f32", "f8e4m3fn", "f8e8m0", "i4", "b1", "f4e2m1 ", ""] {
        assert_eq!(MxElement::from_token(other), None, "{other:?}");
    }
}

// 6.1-0014: the eight non-negative codes are 0, 0.5, 1, 1.5, 2, 3, 4, 6 and -0 is code 8.
#[test]
fn f4e2m1_values() {
    let want = [0.0, 0.5, 1.0, 1.5, 2.0, 3.0, 4.0, 6.0];
    for (code, v) in want.iter().enumerate() {
        assert_eq!(MxElement::F4E2M1.decode(code as u8), Ok(*v), "code {code}");
        assert_eq!(
            MxElement::F4E2M1.decode(code as u8 + 8),
            Ok(-*v),
            "code {}",
            code + 8
        );
    }
    let neg_zero = MxElement::F4E2M1.decode(8).unwrap();
    assert!(neg_zero == 0.0 && neg_zero.is_sign_negative());
    assert_eq!(
        MxElement::F4E2M1.decode(16),
        Err(MxError::CodeOutOfRange {
            element: MxElement::F4E2M1,
            code: 16
        })
    );
}

// 6.1-0015: the OCP limits quoted in the clause's provenance: max normal, min normal, max subnormal, min subnormal.
#[test]
fn f6_limits() {
    // f6e2m3 (bias 1): max 7.5 (e=3, m=7), min normal 1.0 (e=1, m=0), max subnormal 0.875 (m=7), min subnormal 0.125 (m=1)
    assert_eq!(MxElement::F6E2M3.decode(0b011_111), Ok(7.5));
    assert_eq!(MxElement::F6E2M3.decode(0b001_000), Ok(1.0));
    assert_eq!(MxElement::F6E2M3.decode(0b000_111), Ok(0.875));
    assert_eq!(MxElement::F6E2M3.decode(0b000_001), Ok(0.125));
    // f6e3m2 (bias 3): max 28 (e=7, m=3), min normal 0.25 (e=1, m=0), max subnormal 0.1875 (m=3), min subnormal 0.0625 (m=1)
    assert_eq!(MxElement::F6E3M2.decode(f6e3m2(0, 7, 3)), Ok(28.0));
    assert_eq!(MxElement::F6E3M2.decode(f6e3m2(0, 1, 0)), Ok(0.25));
    assert_eq!(MxElement::F6E3M2.decode(f6e3m2(0, 0, 3)), Ok(0.1875));
    assert_eq!(MxElement::F6E3M2.decode(f6e3m2(0, 0, 1)), Ok(0.0625));
    // the sign is the top bit
    assert_eq!(MxElement::F6E2M3.decode(0b111_111), Ok(-7.5));
    assert_eq!(MxElement::F6E3M2.decode(f6e3m2(1, 7, 3)), Ok(-28.0));
    for e in [MxElement::F6E2M3, MxElement::F6E3M2] {
        assert_eq!(
            e.decode(64),
            Err(MxError::CodeOutOfRange {
                element: e,
                code: 64
            })
        );
    }
}

// 6.1-0014, -0015: no infinity and no NaN encoding, the largest finite magnitude is the stated one, and the non-negative codes are
// strictly increasing (so the format has no redundant or special code).
#[test]
fn every_code_is_finite_and_ordered() {
    for e in MxElement::ALL {
        let half = 1u8 << (e.bits() - 1);
        let mut prev = f64::NEG_INFINITY;
        for code in 0..half {
            let v = e.decode(code).unwrap();
            assert!(
                v.is_finite() && v >= 0.0 && v > prev,
                "{} code {code}: {v}",
                e.token()
            );
            prev = v;
            assert_eq!(e.decode(code | half), Ok(-v), "{} sign", e.token());
        }
        assert_eq!(prev, e.max_finite(), "{}", e.token());
    }
}

// 6.1-0014: two per byte, the low nibble the even logical index (the same layout as i4/u4, 6.1-0008).
#[test]
fn f4e2m1_packs_two_per_byte_low_nibble_first() {
    assert_eq!(
        MxElement::F4E2M1.pack(&[0, 1, 2, 3, 4, 5, 6, 7]).unwrap(),
        [0x10, 0x32, 0x54, 0x76]
    );
    assert_eq!(MxElement::F4E2M1.pack(&[0xF, 0x0]).unwrap(), [0x0F]);
    // an odd count rounds up and writes a zero high nibble
    assert_eq!(MxElement::F4E2M1.pack(&[9, 8, 7]).unwrap(), [0x89, 0x07]);
    assert_eq!(MxElement::F4E2M1.packed_len(3), 2);
}

// 6.1-0015: four elements per three bytes, least-significant bit first, ceil(6n/8) bytes. The expectation below is worked by hand
// from the clause's stream rule (element i at stream bits 6i..=6i+5): codes 1, 2, 3, 4 give bytes 0x81, 0x30, 0x10.
#[test]
fn f6_packs_four_per_three_bytes_lsb_first() {
    for e in [MxElement::F6E2M3, MxElement::F6E3M2] {
        assert_eq!(
            e.pack(&[1, 2, 3, 4]).unwrap(),
            [0x81, 0x30, 0x10],
            "{}",
            e.token()
        );
        for (n, bytes) in [(0, 0), (1, 1), (2, 2), (3, 3), (4, 3), (5, 4), (8, 6)] {
            assert_eq!(e.packed_len(n), bytes, "{} n={n}", e.token());
        }
    }
}

#[test]
fn pack_and_unpack_round_trip_every_code() {
    for e in MxElement::ALL {
        let all: Vec<u8> = (0..1u16 << e.bits()).map(|c| c as u8).collect();
        for n in 0..=all.len() {
            let codes = &all[..n];
            let packed = e.pack(codes).unwrap();
            assert_eq!(packed.len(), e.packed_len(n));
            assert_eq!(e.unpack(&packed, n).unwrap(), codes, "{} n={n}", e.token());
        }
    }
}

// 6.1-0015: unused trailing bits MUST be zero, so a reader declines a 6-bit buffer whose padding is set.
#[test]
fn f6_trailing_bits_must_be_zero() {
    for e in [MxElement::F6E2M3, MxElement::F6E3M2] {
        // one element uses 6 of 8 bits: bits 6 and 7 are padding
        assert_eq!(
            e.unpack(&[0b0100_0001], 1),
            Err(MxError::TrailingBitsNonZero)
        );
        assert_eq!(
            e.unpack(&[0b1000_0001], 1),
            Err(MxError::TrailingBitsNonZero)
        );
        assert_eq!(e.unpack(&[0b0011_1111], 1), Ok(vec![0b11_1111]));
        // three elements use 18 of 24 bits
        assert_eq!(
            e.unpack(&[0, 0, 0b0100_0000], 3),
            Err(MxError::TrailingBitsNonZero)
        );
        // four elements fill three bytes exactly: no padding to check
        assert!(e.unpack(&[0xFF, 0xFF, 0xFF], 4).is_ok());
    }
}

// Bytes past the extent are not read; a buffer shorter than the extent is truncated, never completed.
#[test]
fn unpack_checks_length_not_trailing_bytes() {
    for e in MxElement::ALL {
        let packed = e.pack(&[1, 2, 3, 4]).unwrap();
        let mut longer = packed.clone();
        longer.extend_from_slice(&[0xFF; 5]);
        assert_eq!(e.unpack(&longer, 4).unwrap(), [1, 2, 3, 4]);
        assert_eq!(
            e.unpack(&packed[..packed.len() - 1], 4),
            Err(MxError::Truncated {
                need: packed.len(),
                have: packed.len() - 1
            })
        );
        assert_eq!(e.unpack(&[], 0), Ok(vec![]));
    }
}

#[test]
fn pack_rejects_a_code_that_does_not_fit() {
    assert_eq!(
        MxElement::F4E2M1.pack(&[1, 16]),
        Err(MxError::CodeOutOfRange {
            element: MxElement::F4E2M1,
            code: 16
        })
    );
    assert_eq!(
        MxElement::F6E3M2.pack(&[64]),
        Err(MxError::CodeOutOfRange {
            element: MxElement::F6E3M2,
            code: 64
        })
    );
}

// 6.1-0016: an MX element in an accumulator position is a typed decline, distinct from the unknown and reserved token declines.
#[test]
fn an_mx_element_is_never_an_accumulator() {
    for (token, e) in [
        ("f4e2m1", MxElement::F4E2M1),
        ("f6e2m3", MxElement::F6E2M3),
        ("f6e3m2", MxElement::F6E3M2),
    ] {
        let err = check_accumulator(token).unwrap_err();
        assert_eq!(err, MxError::ElementDtypeAsAccumulator(e));
        // KISS's name for the decline (structure_key_vectors.json: ElementDtypeAsAccumulator)
        assert_eq!(err.code(), "element_dtype_as_accumulator");
    }
    // the ordinary accumulators, and a token this crate does not judge, are not declined here
    for ok in ["f32", "f16", "bf16", "i32", "f8e4m3fn", "f8e8m0", "wobble"] {
        assert_eq!(check_accumulator(ok), Ok(()), "{ok}");
    }
}
