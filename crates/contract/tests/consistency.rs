// SPDX-License-Identifier: MIT OR Apache-2.0
//! The cross-section rules and closed vocabularies of the Capabilities, Guarantees and Provenance sections (KISS-CONTRACT-6.7-0002, -0003,
//! -0004, 6.8-0003, -0004a, -0006, -0008..-0010, -0013, 6.9-0003, -0006, -0008): the golden document of KISS `contract_vectors.json`
//! (origin/main 73c341b5) passes, and every fault is written from the clause text (the corpus has one value per field, so those cases are
//! this crate's own). Draft-tracking: expect rework when the Draft moves.

use fireline_contract::consistency::*;
use fireline_contract::document::{encode_document, parse_document};
use fireline_contract::fields::parse_contract;

const GOLDEN: &str = "4B495343206B6973732D636F6E74726163742034206C656E3D313238302063726333323D37396635303365340A5B73656374696F6E3A313A6964656E746974795D0A636F6E74726163745F6B696E64203D206B6973732D636F6E74726163740A636F6E74726163745F76657273696F6E203D20340A6B65726E656C5F6E616D65203D206164645F6633325F737472696465645F736D38390A7265766973696F6E5F68617368203D20343A64656164626565660A6163636570745F707265646963617465203D20736B377C6577327C6633327C637564613A736D38397C697833327C677269647C72327C636F2F30302F76342F6431362F663B73742F30302F76312F6431362F663B636F2F30302F76342F6431362F667C2D0A6F705F6964656E74697479203D206164640A7461726765745F6361706162696C697479203D20637564613A736D38390A5B73656374696F6E3A323A73656D616E746963735D0A73656D616E746963735F6B696E64203D206D616368696E652D636865636B61626C652D49520A6F705F646167203D205B4F707B6164643B203B205B5D7D5D0A5B73656374696F6E3A333A696E746572666163655D0A656E7472795F706F696E74203D206B0A72616E6B203D20320A706F736974696F6E616C5F7369676E6174757265203D205B5D0A6C61756E63685F7363616C617273203D205B5D0A636F756E745F756E6974203D20656C656D656E74730A696E5F706C616365203D206E6F6E650A616C69676E6D656E745F6279746573203D20310A5B73656374696F6E3A343A64697370617463685D0A64697370617463685F6D6F64656C203D2067656F6D657472792D61676E6F737469630A5B73656374696F6E3A353A6361706162696C69746965735D0A6163636570745F707265646963617465203D20736B377C6577327C6633327C637564613A736D38397C697833327C677269647C72327C636F2F30302F76342F6431362F663B73742F30302F76312F6431362F663B636F2F30302F76342F6431362F667C2D0A737570706F727465645F64747970655F736574203D205B6633325D0A61776B776172645F6C61796F75745F7374726174656779203D206465636C696E650A696E5F706C6163655F656C696769626C655F76617269616E7473203D205B5D0A696E6465785F7769647468203D20697833320A64657465726D696E69736D5F636C617373203D2065786163742D627974650A707265636973696F6E5F636C617373203D207374726963740A636F7374203D20310A5B73656374696F6E3A363A67756172616E746565735D0A7265666572656E63655F66756E6374696F6E203D206164640A7065725F6261636B656E645F756C705F7469657273203D205B637564613A736D38397B6D61785F756C703D307D5D0A64657465726D696E69736D5F636C617373203D2065786163742D627974650A6D6174685F666964656C697479203D206269742D737461626C650A616363756D756C6174696F6E5F74797065203D206633320A6269745F73746162696C697479203D20706F727461626C650A617564697465645F737461747573203D20617564697465640A636F73745F70726F76656E616E6365203D206D656173757265640A5B73656374696F6E3A373A70726F76656E616E63655D0A6B65726E656C5F736F75726365203D20617574686F7265640A7265766973696F6E5F62617365203D2072300A7265766973696F6E5F68617368203D20343A64656164626565660A636F73745F70726F76656E616E6365203D206D656173757265640A6E65676F74696174696F6E5F6D65746164617461203D20303A0A";
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
