// SPDX-License-Identifier: MIT OR Apache-2.0
//! The cross-section rules and closed vocabularies of the Capabilities, Guarantees and Provenance sections (KISS-CONTRACT-6.7-0002, -0003,
//! -0004, 6.8-0003, -0004a, -0006, -0008..-0010, -0013, 6.9-0003, -0006, -0008): the golden document of KISS `contract_vectors.json`
//! (origin/main 73c341b5) passes, and every fault is written from the clause text (the corpus has one value per field, so those cases are
//! this crate's own). Draft-tracking: expect rework when the Draft moves.

use fireline_contract::consistency::*;
use fireline_contract::document::{encode_document, parse_document};
use fireline_contract::fields::parse_contract;

const GOLDEN: &str = "4b495343206b6973732d636f6e74726163742034206c656e3d313238312063726333323d65313864393561640a5b73656374696f6e3a313a6964656e746974795d0a636f6e74726163745f6b696e64203d206b6973732d636f6e74726163740a636f6e74726163745f76657273696f6e203d20340a6b65726e656c5f6e616d65203d206164645f6633325f737472696465645f736d38390a7265766973696f6e5f68617368203d20343a64656164626565660a6163636570745f707265646963617465203d20736b377c6577327c6633327c637564613a736d38397c697833327c677269647c72327c636f2f30302f76342f6431362f663b73742f30302f76312f6431362f663b636f2f30302f76342f6431362f667c2d0a6f705f6964656e74697479203d206164640a7461726765745f6361706162696c697479203d20637564613a736d38390a5b73656374696f6e3a323a73656d616e746963735d0a73656d616e746963735f6b696e64203d206d616368696e652d636865636b61626c652d49520a6f705f646167203d205b4f707b6164643b203b205b5d7d5d0a5b73656374696f6e3a333a696e746572666163655d0a656e7472795f706f696e74203d206b0a72616e6b203d20320a706f736974696f6e616c5f7369676e6174757265203d205b5d0a6c61756e63685f7363616c617273203d205b5d0a636f756e745f756e6974203d20656c656d656e74730a696e5f706c616365203d2066616c73650a616c69676e6d656e745f6279746573203d20310a5b73656374696f6e3a343a64697370617463685d0a64697370617463685f6d6f64656c203d2067656f6d657472792d61676e6f737469630a5b73656374696f6e3a353a6361706162696c69746965735d0a6163636570745f707265646963617465203d20736b377c6577327c6633327c637564613a736d38397c697833327c677269647c72327c636f2f30302f76342f6431362f663b73742f30302f76312f6431362f663b636f2f30302f76342f6431362f667c2d0a737570706f727465645f64747970655f736574203d205b6633325d0a61776b776172645f6c61796f75745f7374726174656779203d206465636c696e650a696e5f706c6163655f656c696769626c655f76617269616e7473203d205b5d0a696e6465785f7769647468203d20697833320a64657465726d696e69736d5f636c617373203d2065786163742d627974650a707265636973696f6e5f636c617373203d207374726963740a636f7374203d20310a5b73656374696f6e3a363a67756172616e746565735d0a7265666572656e63655f66756e6374696f6e203d206164640a7065725f6261636b656e645f756c705f7469657273203d205b637564613a736d38397b6d61785f756c703d307d5d0a64657465726d696e69736d5f636c617373203d2065786163742d627974650a6d6174685f666964656c697479203d206269742d737461626c650a616363756d756c6174696f6e5f74797065203d206633320a6269745f73746162696c697479203d20706f727461626c650a617564697465645f737461747573203d20617564697465640a636f73745f70726f76656e616e6365203d206d656173757265640a5b73656374696f6e3a373a70726f76656e616e63655d0a6b65726e656c5f736f75726365203d20617574686f7265640a7265766973696f6e5f62617365203d2072300a7265766973696f6e5f68617368203d20343a64656164626565660a636f73745f70726f76656e616e6365203d206d656173757265640a6e65676f74696174696f6e5f6d65746164617461203d20303a0a";
const ACCEPT: &str =
    "sk7|ew2|f32|cuda:sm89|ix32|grid|r2|co/00/v4/d16/f;st/00/v1/d16/f;co/00/v4/d16/f|-";

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
        .collect()
}

/// The golden document with exact substrings replaced (each must occur once), re-framed with a fresh length and CRC-32.
fn edited(replacements: &[(&str, &str)]) -> Vec<u8> {
    let bytes = unhex(GOLDEN);
    let doc = parse_document(&bytes, &[4]).unwrap();
    let mut body = String::from_utf8(doc.body.to_vec()).unwrap();
    for (old, new) in replacements {
        assert_eq!(body.matches(old).count(), 1, "{old}");
        body = body.replace(old, new);
    }
    encode_document(body.as_bytes()).unwrap()
}

fn check(bytes: &[u8]) -> Result<(), ConsistencyError> {
    let doc = parse_document(bytes, &[4]).expect("a framed document");
    check_consistency(&parse_contract(&doc).expect("field schemas hold"))
}

fn disagree(field: &'static str, clause: &'static str) -> Result<(), ConsistencyError> {
    Err(ConsistencyError::Disagree { field, clause })
}

fn bad_vocab(field: &'static str, value: &str) -> Result<(), ConsistencyError> {
    Err(ConsistencyError::NotInVocabulary {
        field,
        value: value.into(),
    })
}

#[test]
fn the_golden_document_is_consistent() {
    assert_eq!(check(&unhex(GOLDEN)), Ok(()));
}

// 6.7-0002, 6.9-0003, 6.7-0004, 6.9-0006: copies that must agree byte for byte.
#[test]
fn the_copies_must_agree() {
    let cap = format!("accept_predicate = {ACCEPT}\nsupported_dtype_set");
    let cap_other = format!("accept_predicate = {}x\nsupported_dtype_set", ACCEPT);
    assert_eq!(
        check(&edited(&[(&cap, &cap_other)])),
        disagree("capabilities.accept_predicate", "6.7-0002")
    );
    assert_eq!(
        check(&edited(&[(
            "revision_hash = 4:deadbeef\ncost_provenance",
            "revision_hash = 4:deadbeee\ncost_provenance"
        )])),
        disagree("provenance.revision_hash", "6.9-0003")
    );
    assert_eq!(
        check(&edited(&[(
            "determinism_class = exact-byte\nprecision_class",
            "determinism_class = ULP/tolerance\nprecision_class"
        )])),
        disagree("capabilities.determinism_class", "6.7-0004")
    );
    assert_eq!(
        check(&edited(&[(
            "cost_provenance = measured\nnegotiation_metadata",
            "cost_provenance = declared\nnegotiation_metadata"
        )])),
        disagree("provenance.cost_provenance", "6.9-0006")
    );
}

// 6.8-0003, 6.7-0004: determinism_class is the KISS-Ops enum, spelled verbatim.
#[test]
fn determinism_class_is_the_ops_enum() {
    for ok in DETERMINISM_CLASSES {
        let r = check(&edited(&[
            (
                "determinism_class = exact-byte\nmath_fidelity",
                &format!("determinism_class = {ok}\nmath_fidelity"),
            ),
            (
                "determinism_class = exact-byte\nprecision_class",
                &format!("determinism_class = {ok}\nprecision_class"),
            ),
        ]));
        assert!(
            !matches!(r, Err(ConsistencyError::NotInVocabulary { .. })),
            "{ok}: {r:?}"
        );
    }
    for bad in [
        "exact_byte",
        "Exact-Byte",
        "ulp/tolerance",
        "order-invariant",
        "deterministic",
        "",
    ] {
        let r = check(&edited(&[
            (
                "determinism_class = exact-byte\nmath_fidelity",
                &format!("determinism_class = {bad}\nmath_fidelity"),
            ),
            (
                "determinism_class = exact-byte\nprecision_class",
                &format!("determinism_class = {bad}\nprecision_class"),
            ),
        ]));
        assert_eq!(r, bad_vocab("guarantees.determinism_class", bad), "{bad:?}");
    }
}

// 6.8-0004a: `bit-stable` or `reduced-mantissa-permitted:<n>`, n a canonical decimal 1..=52; the bare spelling is declined.
#[test]
fn math_fidelity_carries_its_width() {
    for ok in [
        "bit-stable",
        "reduced-mantissa-permitted:1",
        "reduced-mantissa-permitted:7",
        "reduced-mantissa-permitted:10",
        "reduced-mantissa-permitted:52",
    ] {
        assert_eq!(
            check(&edited(&[(
                "math_fidelity = bit-stable",
                &format!("math_fidelity = {ok}")
            )])),
            Ok(()),
            "{ok}"
        );
    }
    for bad in [
        "reduced-mantissa-permitted",
        "reduced-mantissa-permitted:",
        "reduced-mantissa-permitted:0",
        "reduced-mantissa-permitted:53",
        "reduced-mantissa-permitted:07",
        "reduced-mantissa-permitted:+7",
        "reduced-mantissa-permitted:7.0",
        "reduced-mantissa-permitted: 7",
        "reduced-mantissa-permitted:256",
        "bit-stable:7",
        "Bit-Stable",
        "strict",
    ] {
        assert_eq!(
            check(&edited(&[(
                "math_fidelity = bit-stable",
                &format!("math_fidelity = {bad}")
            )])),
            bad_vocab("guarantees.math_fidelity", bad),
            "{bad:?}"
        );
    }
}

// 6.8-0013: bit_stability is one of three; a nondeterministic-class kernel never declares `portable`.
#[test]
fn bit_stability_is_a_closed_set_with_one_coupling() {
    for ok in BIT_STABILITY {
        assert_eq!(
            check(&edited(&[(
                "bit_stability = portable",
                &format!("bit_stability = {ok}")
            )])),
            Ok(()),
            "{ok}"
        );
    }
    for bad in ["Portable", "portable ", "same_hardware", "any", ""] {
        assert_eq!(
            check(&edited(&[(
                "bit_stability = portable",
                &format!("bit_stability = {bad}")
            )])),
            bad_vocab("guarantees.bit_stability", bad),
            "{bad:?}"
        );
    }
    let nondet = |stability: &str| {
        edited(&[
            (
                "determinism_class = exact-byte\nmath_fidelity",
                "determinism_class = order-invariant/nondeterministic\nmath_fidelity",
            ),
            (
                "determinism_class = exact-byte\nprecision_class",
                "determinism_class = order-invariant/nondeterministic\nprecision_class",
            ),
            (
                "bit_stability = portable",
                &format!("bit_stability = {stability}"),
            ),
        ])
    };
    assert_eq!(
        check(&nondet("portable")),
        Err(ConsistencyError::PortableUnderNondeterministic)
    );
    assert_eq!(check(&nondet("same-hardware")), Ok(()));
    assert_eq!(check(&nondet("none")), Ok(()));
}

// 6.8-0006, 6.7-0003: cost_provenance and index_width are closed.
#[test]
fn cost_provenance_and_index_width_are_closed() {
    for ok in COST_PROVENANCE {
        assert_eq!(
            check(&edited(&[
                (
                    "cost_provenance = measured\n[section:7",
                    &format!("cost_provenance = {ok}\n[section:7")
                ),
                (
                    "cost_provenance = measured\nnegotiation_metadata",
                    &format!("cost_provenance = {ok}\nnegotiation_metadata")
                ),
            ])),
            Ok(()),
            "{ok}"
        );
    }
    assert_eq!(
        check(&edited(&[(
            "cost_provenance = measured\n[section:7",
            "cost_provenance = estimated\n[section:7"
        )])),
        bad_vocab("guarantees.cost_provenance", "estimated")
    );
    assert_eq!(
        check(&edited(&[("index_width = ix32", "index_width = ix64")])),
        Ok(())
    );
    for bad in ["idx32", "ix16", "IX32", ""] {
        assert_eq!(
            check(&edited(&[(
                "index_width = ix32",
                &format!("index_width = {bad}")
            )])),
            bad_vocab("capabilities.index_width", bad),
            "{bad:?}"
        );
    }
}

// 6.11-0001, 6.9-0008: an opaque blob is `<n>:<hex>` with exactly 2n lowercase hex digits; an empty blob is `0:`.
#[test]
fn blobs_carry_a_matching_length() {
    for ok in ["0:", "1:00", "2:00ff", "4:deadbeef"] {
        assert_eq!(
            check(&edited(&[(
                "negotiation_metadata = 0:",
                &format!("negotiation_metadata = {ok}")
            )])),
            Ok(()),
            "{ok}"
        );
    }
    for bad in [
        "", "0", "1:", "0:00", "1:0", "1:000", "1:0G", "1:0A", "01:00", "-1:00", "x:00", "1 :00",
        "1: 00",
    ] {
        assert_eq!(
            check(&edited(&[(
                "negotiation_metadata = 0:",
                &format!("negotiation_metadata = {bad}")
            )])),
            Err(ConsistencyError::Blob {
                field: "provenance.negotiation_metadata"
            }),
            "{bad:?}"
        );
    }
    // the same encoding for the revision_hash (the Identity copy is checked first)
    for bad in ["4:dead", "04:deadbeef", "4:DEADBEEF", "deadbeef"] {
        assert_eq!(
            check(&edited(&[
                (
                    "revision_hash = 4:deadbeef\naccept_predicate",
                    &format!("revision_hash = {bad}\naccept_predicate")
                ),
                (
                    "revision_hash = 4:deadbeef\ncost_provenance",
                    &format!("revision_hash = {bad}\ncost_provenance")
                ),
            ])),
            Err(ConsistencyError::Blob {
                field: "identity.revision_hash"
            }),
            "{bad:?}"
        );
    }
}

// 6.8-0008..-0010: audited_status is derived from the Guarantees' own fields, never authored.
#[test]
fn audited_status_is_derived() {
    let derived = |want: &'static str, found: &str| {
        Err(ConsistencyError::AuditedStatus {
            derived: want,
            found: found.into(),
        })
    };
    // a named reference and a declared tier: audited
    assert_eq!(
        check(&edited(&[(
            "audited_status = audited",
            "audited_status = unaudited"
        )])),
        derived("audited", "unaudited")
    );
    assert_eq!(
        check(&edited(&[(
            "audited_status = audited",
            "audited_status = verified"
        )])),
        derived("audited", "verified")
    );
    // no declared tier: unaudited, and `audited` authored over it is refused
    let no_tiers = |status: &str| {
        edited(&[
            (
                "per_backend_ulp_tiers = [cuda:sm89{max_ulp=0}]",
                "per_backend_ulp_tiers = []",
            ),
            (
                "audited_status = audited",
                &format!("audited_status = {status}"),
            ),
        ])
    };
    assert_eq!(check(&no_tiers("unaudited")), Ok(()));
    assert_eq!(check(&no_tiers("audited")), derived("unaudited", "audited"));
    // no named reference function: unaudited
    let unnamed = |status: &str| {
        edited(&[
            ("reference_function = add", "reference_function = "),
            (
                "audited_status = audited",
                &format!("audited_status = {status}"),
            ),
        ])
    };
    assert_eq!(check(&unnamed("unaudited")), Ok(()));
    assert_eq!(check(&unnamed("audited")), derived("unaudited", "audited"));
    // bit_stability is not an input: two Guarantees that differ only in it derive the same status
    assert_eq!(
        check(&edited(&[(
            "bit_stability = portable",
            "bit_stability = none"
        )])),
        Ok(())
    );
}
