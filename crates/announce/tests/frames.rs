// SPDX-License-Identifier: MIT OR Apache-2.0
//! The KISS-Announce message frames (§6.3, §6.4): the exact bytes of `SAVL`, `CYRQ`, `CRSP` and `CDEC` diffed against KISS
//! `conformance/corpus/announce_vectors.json` (KISS origin/main 73c341b5), and the reader's typed declines. Draft-tracking: the wire is
//! not frozen, so expect rework when the Draft moves.

use fireline_announce::frames::*;

fn hex(s: &str) -> Vec<u8> {
    s.split_whitespace()
        .map(|x| u8::from_str_radix(x, 16).expect("hex byte"))
        .collect()
}

const SAVL: &str = "53 41 56 4C 03 00 00 00 02 00 00 00 03 00 00 00 AA BB CC 03 00 00 00 61 64 64 20 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 02 00 00 00 DE AD 04 00 00 00 72 65 6C 75 20 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22";
const CYRQ: &str = "43 59 52 51 03 00 00 00 AA BB CC 03 00 00 00 61 64 64 01 20 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 00";
const CRSP: &str = "43 52 53 50 03 00 00 00 AA BB CC 03 00 00 00 61 64 64 01 20 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 00 05 00 00 00 DE AD BE EF 01";
const CDEC: &str = "43 44 45 43 03 00 00 00 AA BB CC 03 00 00 00 61 64 64 01 20 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 11 00 06 00 00 00";

fn identity() -> Identity {
    Identity {
        structure_key: vec![0xAA, 0xBB, 0xCC],
        op_identity: b"add".to_vec(),
        revision_hash: Some(vec![0x11; 32]),
        correlation_id: None,
    }
}

fn records() -> Vec<AvailabilityRecord> {
    vec![
        AvailabilityRecord {
            structure_key: vec![0xAA, 0xBB, 0xCC],
            op_identity: b"add".to_vec(),
            revision_hash: vec![0x11; 32],
        },
        AvailabilityRecord {
            structure_key: vec![0xDE, 0xAD],
            op_identity: b"relu".to_vec(),
            revision_hash: vec![0x22; 32],
        },
    ]
}

fn decode_any(name: &str, bytes: &[u8]) -> Result<usize, FrameError> {
    match name {
        "SAVL" => decode_savl(bytes).map(|x| x.1),
        "CYRQ" => decode_cyrq(bytes).map(|x| x.1),
        "CRSP" => decode_crsp(bytes).map(|x| x.1),
        _ => decode_cdec(bytes).map(|x| x.1),
    }
}

// KISS-ANNOUNCE-6.3-0011/-0012/-0005: the availability list, byte for byte.
#[test]
fn savl_is_the_kiss_vector() {
    assert_eq!(encode_savl(&records()).unwrap(), hex(SAVL));
    let bytes = hex(SAVL);
    assert_eq!(decode_savl(&bytes).unwrap(), (records(), bytes.len()));
}

// KISS-ANNOUNCE-6.4-0001: the contract-query request.
#[test]
fn cyrq_is_the_kiss_vector() {
    assert_eq!(encode_cyrq(&identity()).unwrap(), hex(CYRQ));
    let bytes = hex(CYRQ);
    assert_eq!(decode_cyrq(&bytes).unwrap(), (identity(), bytes.len()));
}

// KISS-ANNOUNCE-6.4-0004: the contract response.
#[test]
fn crsp_is_the_kiss_vector() {
    let r = ContractResponse {
        identity: identity(),
        payload: vec![0xDE, 0xAD, 0xBE, 0xEF, 0x01],
    };
    assert_eq!(encode_crsp(&r).unwrap(), hex(CRSP));
    let bytes = hex(CRSP);
    assert_eq!(decode_crsp(&bytes).unwrap(), (r, bytes.len()));
}

// KISS-ANNOUNCE-6.4-0007: the decline response; code 6 is UNKNOWN_REVISION (6.4-0009).
#[test]
fn cdec_is_the_kiss_vector() {
    let r = DeclineResponse {
        identity: identity(),
        code: decline::UNKNOWN_REVISION,
    };
    assert_eq!(encode_cdec(&r).unwrap(), hex(CDEC));
    let bytes = hex(CDEC);
    assert_eq!(decode_cdec(&bytes).unwrap(), (r, bytes.len()));
}

// KISS-ANNOUNCE-6.4-0013: an 8-byte correlation id round-trips; absent is the single byte 00.
#[test]
fn correlation_id_round_trips() {
    let mut id = identity();
    id.correlation_id = Some([1, 2, 3, 4, 5, 6, 7, 8]);
    let bytes = encode_cyrq(&id).unwrap();
    assert_eq!(bytes.len(), hex(CYRQ).len() + 8);
    assert_eq!(decode_cyrq(&bytes).unwrap().0, id);
    // revision_present == 0 writes no hash
    id.revision_hash = None;
    assert_eq!(decode_cyrq(&encode_cyrq(&id).unwrap()).unwrap().0, id);
}

// KISS-ANNOUNCE-6.2-0017: bytes after the extent are not read, and the reported extent does not change.
#[test]
fn bytes_after_the_extent_are_unread() {
    for (name, bytes) in [
        ("SAVL", hex(SAVL)),
        ("CYRQ", hex(CYRQ)),
        ("CRSP", hex(CRSP)),
        ("CDEC", hex(CDEC)),
    ] {
        let mut padded = bytes.clone();
        padded.extend_from_slice(&[0xFF; 9]);
        assert_eq!(decode_any(name, &padded), Ok(bytes.len()), "{name}");
    }
}

// KISS-ANNOUNCE-6.2-0015/-0016: every proper prefix is truncated; a wrong tag is rejected, with no search for a later tag.
#[test]
fn truncated_and_unknown_tags_are_rejected() {
    for (name, full) in [
        ("SAVL", hex(SAVL)),
        ("CYRQ", hex(CYRQ)),
        ("CRSP", hex(CRSP)),
        ("CDEC", hex(CDEC)),
    ] {
        for n in 0..full.len() {
            assert_eq!(
                decode_any(name, &full[..n]),
                Err(FrameError::Truncated),
                "{name} cut to {n}"
            );
        }
    }
    let mut bad = hex(CYRQ);
    bad[0] = b'X';
    assert!(matches!(decode_cyrq(&bad), Err(FrameError::UnknownTag(_))));
    // a CYRQ buffer handed to the CRSP reader is a wrong tag, not a guess
    assert!(matches!(
        decode_crsp(&hex(CYRQ)),
        Err(FrameError::UnknownTag(_))
    ));
}

// KISS-ANNOUNCE-6.3-0012: list_version 1 and 2 are retired, an MBZ byte must be zero.
#[test]
fn list_version_and_mbz_are_enforced() {
    for v in [0u8, 1, 2, 4] {
        let mut b = hex(SAVL);
        b[4] = v;
        assert_eq!(decode_savl(&b), Err(FrameError::UnsupportedListVersion(v)));
    }
    let mut b = hex(SAVL);
    b[6] = 1;
    assert_eq!(decode_savl(&b), Err(FrameError::MbzNonZero));
}

// KISS-ANNOUNCE-6.3-0009/-0013/-0003/-0010: the length and count bounds, checked before anything is allocated.
#[test]
fn length_and_count_bounds_are_hard_rejects() {
    // record_count above 2^20 with no bytes behind it: a typed decline, not an allocation
    let mut b = hex(SAVL);
    b[8..12].copy_from_slice(&(MAX_AVAILABILITY_RECORDS + 1).to_le_bytes());
    assert_eq!(
        decode_savl(&b),
        Err(FrameError::RecordCountTooLarge(
            MAX_AVAILABILITY_RECORDS + 1
        ))
    );
    // a count the bytes cannot hold is truncated
    b[8..12].copy_from_slice(&1000u32.to_le_bytes());
    assert_eq!(decode_savl(&b), Err(FrameError::Truncated));
    // structure_key length 0 and 4097
    let mut c = hex(CYRQ);
    c[4..8].copy_from_slice(&0u32.to_le_bytes());
    assert_eq!(decode_cyrq(&c), Err(FrameError::StructureKeyLen(0)));
    c[4..8].copy_from_slice(&4097u32.to_le_bytes());
    assert_eq!(decode_cyrq(&c), Err(FrameError::StructureKeyLen(4097)));
    // op_identity length 0 (the u32 sits at offset 11, after the 3-byte key)
    let mut c = hex(CYRQ);
    c[11..15].copy_from_slice(&0u32.to_le_bytes());
    assert_eq!(decode_cyrq(&c), Err(FrameError::OpIdentityLen(0)));
    // revision_hash length 7 and 65 (the length byte sits at offset 19, after revision_present at 18)
    let mut c = hex(CYRQ);
    assert_eq!((c[18], c[19]), (1, 0x20));
    c[19] = 7;
    assert_eq!(
        decode_cyrq(&c),
        Err(FrameError::RevisionHashLenOutOfRange(7))
    );
    c[19] = 65;
    assert_eq!(
        decode_cyrq(&c),
        Err(FrameError::RevisionHashLenOutOfRange(65))
    );
    // producers keep the same bounds
    let mut id = identity();
    id.structure_key.clear();
    assert_eq!(encode_cyrq(&id), Err(FrameError::StructureKeyLen(0)));
    let mut id = identity();
    id.revision_hash = Some(vec![0; 7]);
    assert_eq!(
        encode_cyrq(&id),
        Err(FrameError::RevisionHashLenOutOfRange(7))
    );
}

// KISS-ANNOUNCE-6.4-0001/-0013: a revision_present or correlation_len outside its set answers MALFORMED_REQUEST.
#[test]
fn malformed_request_fields_map_to_the_decline_code() {
    let mut c = hex(CYRQ);
    c[18] = 2;
    let e = decode_cyrq(&c).unwrap_err();
    assert_eq!(e, FrameError::RevisionPresent(2));
    assert_eq!(e.decline_code(), Some(decline::MALFORMED_REQUEST));
    let mut c = hex(CYRQ);
    let last = c.len() - 1;
    c[last] = 3;
    let e = decode_cyrq(&c).unwrap_err();
    assert_eq!(e, FrameError::CorrelationLen(3));
    assert_eq!(e.decline_code(), Some(decline::MALFORMED_REQUEST));
    assert_eq!(FrameError::Truncated.decline_code(), None);
}

// KISS-ANNOUNCE-6.4-0014: a declared payload above 2^24 is rejected whether or not the bytes are there.
#[test]
fn payload_cap_is_checked_before_reading() {
    let mut b = hex(CRSP);
    let at = b.len() - 5 - 4;
    b[at..at + 4].copy_from_slice(&(MAX_CONTRACT_PAYLOAD_LEN + 1).to_le_bytes());
    assert_eq!(
        decode_crsp(&b),
        Err(FrameError::PayloadTooLarge(MAX_CONTRACT_PAYLOAD_LEN + 1))
    );
    let big = ContractResponse {
        identity: identity(),
        payload: vec![0; MAX_CONTRACT_PAYLOAD_LEN as usize + 1],
    };
    assert_eq!(
        encode_crsp(&big),
        Err(FrameError::PayloadTooLarge(MAX_CONTRACT_PAYLOAD_LEN + 1))
    );
}

// KISS-ANNOUNCE-6.4-0010: a decline_code this build does not know is carried, not a fault.
#[test]
fn unknown_decline_codes_decode() {
    let r = DeclineResponse {
        identity: identity(),
        code: 0x4000_0001,
    };
    assert_eq!(decode_cdec(&encode_cdec(&r).unwrap()).unwrap().0, r);
}
