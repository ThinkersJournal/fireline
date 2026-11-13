// SPDX-License-Identifier: MIT OR Apache-2.0
//! The KISS-Announce device-query frames (§6.6): the exact bytes of `DVRQ`, `DVRS` and `DVDC` diffed against KISS
//! `conformance/corpus/announce_vectors.json` (KISS origin/main 73c341b5), and the reader's typed declines. Draft-tracking: the wire is
//! not frozen, so expect rework when the Draft moves. The `is_dtype` predicate here is a local stand-in for the Classify dtype table
//! (the non-reserved tokens of KISS `conformance/corpus/dtype_manifest.json` that these tests use).

use fireline_announce::device::*;
use fireline_announce::frames::{FrameError, decline};

fn hex(s: &str) -> Vec<u8> {
    s.split_whitespace()
        .map(|x| u8::from_str_radix(x, 16).expect("hex byte"))
        .collect()
}

fn is_dtype(t: &str) -> bool {
    [
        "bf16", "bool", "f16", "f32", "f64", "i16", "i32", "i64", "i8", "u16", "u32", "u64", "u8",
    ]
    .contains(&t)
}

const DVRQ: &str = "44 56 52 51 00";
const DVRS: &str = "44 56 52 53 01 00 00 00 04 00 00 00 67 70 75 30 09 63 75 64 61 3A 73 6D 38 39 04 04 62 66 31 36 03 66 31 36 03 66 33 32 02 69 38 03 04 62 66 31 36 03 66 31 36 03 66 33 32 02 00 00 00 03 65 78 70 01 03 66 33 32 01 00 00 00 03 65 78 70 02 03 66 33 32 04 00 00 00 00";
const DVDC: &str = "44 56 44 43 04 00 00 00 00";

fn s(v: &[&str]) -> Vec<String> {
    v.iter().map(|x| (*x).to_string()).collect()
}

fn device() -> DeviceRecord {
    DeviceRecord {
        device_id: b"gpu0".to_vec(),
        target: "cuda:sm89".into(),
        store_dtypes: s(&["bf16", "f16", "f32", "i8"]),
        compute_dtypes: s(&["bf16", "f16", "f32"]),
        accuracy: vec![
            AccuracyRecord {
                op: "exp".into(),
                math_fidelity: 1,
                dtype: "f32".into(),
                max_ulp: 1,
            },
            AccuracyRecord {
                op: "exp".into(),
                math_fidelity: 2,
                dtype: "f32".into(),
                max_ulp: 4,
            },
        ],
    }
}

fn response() -> DeviceResponse {
    DeviceResponse {
        devices: vec![device()],
        correlation_id: None,
    }
}

// KISS-ANNOUNCE-6.6-0003: the request is the tag and the correlation field.
#[test]
fn dvrq_is_the_kiss_vector() {
    assert_eq!(encode_dvrq(None), hex(DVRQ));
    assert_eq!(decode_dvrq(&hex(DVRQ)), Ok((None, 5)));
    let id = Some([1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(decode_dvrq(&encode_dvrq(id)), Ok((id, 13)));
}

// KISS-ANNOUNCE-6.6-0004/-0006/-0008: the device response, byte for byte.
#[test]
fn dvrs_is_the_kiss_vector() {
    assert_eq!(encode_dvrs(&response(), &is_dtype).unwrap(), hex(DVRS));
    let bytes = hex(DVRS);
    assert_eq!(
        decode_dvrs(&bytes, &is_dtype).unwrap(),
        (response(), bytes.len())
    );
}

// KISS-ANNOUNCE-6.6-0005: the device decline; code 4 is QUERY_NOT_SUPPORTED.
#[test]
fn dvdc_is_the_kiss_vector() {
    let r = DeviceDecline {
        code: decline::QUERY_NOT_SUPPORTED,
        correlation_id: None,
    };
    assert_eq!(encode_dvdc(&r), hex(DVDC));
    assert_eq!(decode_dvdc(&hex(DVDC)), Ok((r, 9)));
    let u = DeviceDecline {
        code: 0x8000_0001,
        correlation_id: Some([9; 8]),
    };
    assert_eq!(decode_dvdc(&encode_dvdc(&u)).unwrap().0, u);
}

// KISS-ANNOUNCE-6.6-0002: no device is a genuine negative, a DVRS of count 0.
#[test]
fn an_empty_device_list_is_a_response() {
    let r = DeviceResponse {
        devices: vec![],
        correlation_id: Some([7; 8]),
    };
    let bytes = encode_dvrs(&r, &is_dtype).unwrap();
    assert_eq!(decode_dvrs(&bytes, &is_dtype).unwrap(), (r, bytes.len()));
}

// KISS-ANNOUNCE-6.2-0015/-0017: every proper prefix is truncated; trailing bytes are not read.
#[test]
fn truncation_and_trailing_bytes() {
    let full = hex(DVRS);
    for n in 0..full.len() {
        assert_eq!(
            decode_dvrs(&full[..n], &is_dtype),
            Err(FrameError::Truncated),
            "cut to {n}"
        );
    }
    let mut padded = full.clone();
    padded.extend_from_slice(&[0xFF; 7]);
    assert_eq!(decode_dvrs(&padded, &is_dtype).unwrap().1, full.len());
    for (cut, full) in [(hex(DVRQ), 5), (hex(DVDC), 9)] {
        for n in 0..full {
            let r = if cut[1] == b'V' && cut[2] == b'R' && cut[3] == b'Q' {
                decode_dvrq(&cut[..n]).map(|_| ())
            } else {
                decode_dvdc(&cut[..n]).map(|_| ())
            };
            assert_eq!(r, Err(FrameError::Truncated));
        }
    }
    let mut bad = hex(DVRQ);
    bad[0] = b'X';
    assert!(matches!(decode_dvrq(&bad), Err(FrameError::UnknownTag(_))));
}

// KISS-ANNOUNCE-6.6-0004: device_count above 4096 is rejected before any allocation; a count the bytes cannot hold is truncated.
#[test]
fn device_count_bounds() {
    let mut b = hex(DVRS);
    b[4..8].copy_from_slice(&(MAX_DEVICES + 1).to_le_bytes());
    assert_eq!(
        decode_dvrs(&b, &is_dtype),
        Err(FrameError::DeviceCountTooLarge(MAX_DEVICES + 1))
    );
    b[4..8].copy_from_slice(&4096u32.to_le_bytes());
    assert_eq!(decode_dvrs(&b, &is_dtype), Err(FrameError::Truncated));
}

// KISS-ANNOUNCE-6.6-0006: the record bounds.
#[test]
fn record_bounds_are_hard_rejects() {
    // device_id length 0 and 4097 (the u32 sits at offset 8)
    let mut b = hex(DVRS);
    b[8..12].copy_from_slice(&0u32.to_le_bytes());
    assert_eq!(decode_dvrs(&b, &is_dtype), Err(FrameError::DeviceIdLen(0)));
    b[8..12].copy_from_slice(&4097u32.to_le_bytes());
    assert_eq!(
        decode_dvrs(&b, &is_dtype),
        Err(FrameError::DeviceIdLen(4097))
    );
    // a zero-length target token (length byte at offset 16)
    let mut b = hex(DVRS);
    assert_eq!(b[16], 9);
    b[16] = 0;
    assert_eq!(decode_dvrs(&b, &is_dtype), Err(FrameError::BadToken));
    // a space inside a token
    let mut d = device();
    d.target = "cuda :sm89".into();
    assert_eq!(
        encode_dvrs(
            &DeviceResponse {
                devices: vec![d],
                correlation_id: None
            },
            &is_dtype
        ),
        Err(FrameError::BadToken)
    );
    // a target with no namespace separator
    let mut d = device();
    d.target = "cuda".into();
    assert_eq!(
        encode_dvrs(
            &DeviceResponse {
                devices: vec![d],
                correlation_id: None
            },
            &is_dtype
        ),
        Err(FrameError::BadTarget)
    );
    // a dtype set count above 64
    let mut b = hex(DVRS);
    assert_eq!(b[26], 4);
    b[26] = 65;
    assert_eq!(
        decode_dvrs(&b, &is_dtype),
        Err(FrameError::DtypeSetTooLarge(65))
    );
    // accuracy_count above 65536, with nothing behind it
    let mut d = device();
    d.accuracy.clear();
    let mut b = encode_dvrs(
        &DeviceResponse {
            devices: vec![d],
            correlation_id: None,
        },
        &is_dtype,
    )
    .unwrap();
    let at = b.len() - 1 - 4;
    b[at..at + 4].copy_from_slice(&(MAX_ACCURACY_RECORDS + 1).to_le_bytes());
    assert_eq!(
        decode_dvrs(&b, &is_dtype),
        Err(FrameError::AccuracyCountTooLarge(MAX_ACCURACY_RECORDS + 1))
    );
}

// KISS-ANNOUNCE-6.6-0007: dtype sets are real spellings, strictly ascending, and compute is a subset of store.
#[test]
fn dtype_sets_are_consistent() {
    let wrap = |d: DeviceRecord| DeviceResponse {
        devices: vec![d],
        correlation_id: None,
    };
    let mut d = device();
    d.store_dtypes = s(&["bf16", "f16", "f32", "i8", "wobble"]);
    assert_eq!(
        encode_dvrs(&wrap(d), &is_dtype),
        Err(FrameError::UnknownDtype("wobble".into()))
    );
    let mut d = device();
    d.store_dtypes = s(&["f16", "bf16", "f32", "i8"]);
    assert_eq!(
        encode_dvrs(&wrap(d), &is_dtype),
        Err(FrameError::DtypeSetNotAscending)
    );
    let mut d = device();
    d.store_dtypes = s(&["bf16", "bf16", "f32", "i8"]);
    assert_eq!(
        encode_dvrs(&wrap(d), &is_dtype),
        Err(FrameError::DtypeSetNotAscending)
    );
    let mut d = device();
    d.compute_dtypes = s(&["bf16", "f16", "f32", "u8"]);
    assert_eq!(
        encode_dvrs(&wrap(d), &is_dtype),
        Err(FrameError::ComputeNotSubsetOfStore)
    );
    // a reader gets the same verdicts from the wire: rewrite the vector's last store token "i8" to the unknown "i9"
    let mut b = hex(DVRS);
    let at = b.windows(2).position(|w| w == *b"i8").unwrap();
    b[at + 1] = b'9';
    assert_eq!(
        decode_dvrs(&b, &is_dtype),
        Err(FrameError::UnknownDtype("i9".into()))
    );
}

// KISS-ANNOUNCE-6.6-0008: fidelity in {1,2}, records strictly ascending by key, dtype within compute_dtypes.
#[test]
fn accuracy_records_are_consistent() {
    let wrap = |d: DeviceRecord| DeviceResponse {
        devices: vec![d],
        correlation_id: None,
    };
    let mut d = device();
    d.accuracy[0].math_fidelity = 3;
    assert_eq!(
        encode_dvrs(&wrap(d), &is_dtype),
        Err(FrameError::MathFidelity(3))
    );
    let mut d = device();
    d.accuracy.swap(0, 1);
    assert_eq!(
        encode_dvrs(&wrap(d), &is_dtype),
        Err(FrameError::AccuracyNotAscending)
    );
    let mut d = device();
    d.accuracy[1] = d.accuracy[0].clone();
    assert_eq!(
        encode_dvrs(&wrap(d), &is_dtype),
        Err(FrameError::AccuracyNotAscending)
    );
    let mut d = device();
    d.accuracy[0].dtype = "i8".into(); // stored but not computed
    assert_eq!(
        encode_dvrs(&wrap(d), &is_dtype),
        Err(FrameError::AccuracyDtypeNotComputed("i8".into()))
    );
    // NO_BOUND and an exact result (0) are both legal values
    let mut d = device();
    d.accuracy[0].max_ulp = NO_BOUND;
    d.accuracy[1].max_ulp = 0;
    let bytes = encode_dvrs(&wrap(d.clone()), &is_dtype).unwrap();
    assert_eq!(decode_dvrs(&bytes, &is_dtype).unwrap().0.devices[0], d);
}
