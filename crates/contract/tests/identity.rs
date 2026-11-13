// SPDX-License-Identifier: MIT OR Apache-2.0
//! The Identity section's own rules (KISS-CONTRACT-6.3-0002, -0007, -0009, -0011, -0010): the golden document of KISS `contract_vectors.json`
//! (origin/main 73c341b5) passes, and every fault is written from the clause text (the corpus has one value per field, so those cases are
//! this crate's own). Draft-tracking: expect rework when the Draft moves.

use fireline_contract::document::{encode_document, parse_document};
use fireline_contract::fields::parse_contract;
use fireline_contract::identity::*;

const GOLDEN: &str = "4b495343206b6973732d636f6e74726163742034206c656e3d313238312063726333323d65313864393561640a5b73656374696f6e3a313a6964656e746974795d0a636f6e74726163745f6b696e64203d206b6973732d636f6e74726163740a636f6e74726163745f76657273696f6e203d20340a6b65726e656c5f6e616d65203d206164645f6633325f737472696465645f736d38390a7265766973696f6e5f68617368203d20343a64656164626565660a6163636570745f707265646963617465203d20736b377c6577327c6633327c637564613a736d38397c697833327c677269647c72327c636f2f30302f76342f6431362f663b73742f30302f76312f6431362f663b636f2f30302f76342f6431362f667c2d0a6f705f6964656e74697479203d206164640a7461726765745f6361706162696c697479203d20637564613a736d38390a5b73656374696f6e3a323a73656d616e746963735d0a73656d616e746963735f6b696e64203d206d616368696e652d636865636b61626c652d49520a6f705f646167203d205b4f707b6164643b203b205b5d7d5d0a5b73656374696f6e3a333a696e746572666163655d0a656e7472795f706f696e74203d206b0a72616e6b203d20320a706f736974696f6e616c5f7369676e6174757265203d205b5d0a6c61756e63685f7363616c617273203d205b5d0a636f756e745f756e6974203d20656c656d656e74730a696e5f706c616365203d2066616c73650a616c69676e6d656e745f6279746573203d20310a5b73656374696f6e3a343a64697370617463685d0a64697370617463685f6d6f64656c203d2067656f6d657472792d61676e6f737469630a5b73656374696f6e3a353a6361706162696c69746965735d0a6163636570745f707265646963617465203d20736b377c6577327c6633327c637564613a736d38397c697833327c677269647c72327c636f2f30302f76342f6431362f663b73742f30302f76312f6431362f663b636f2f30302f76342f6431362f667c2d0a737570706f727465645f64747970655f736574203d205b6633325d0a61776b776172645f6c61796f75745f7374726174656779203d206465636c696e650a696e5f706c6163655f656c696769626c655f76617269616e7473203d205b5d0a696e6465785f7769647468203d20697833320a64657465726d696e69736d5f636c617373203d2065786163742d627974650a707265636973696f6e5f636c617373203d207374726963740a636f7374203d20310a5b73656374696f6e3a363a67756172616e746565735d0a7265666572656e63655f66756e6374696f6e203d206164640a7065725f6261636b656e645f756c705f7469657273203d205b637564613a736d38397b6d61785f756c703d307d5d0a64657465726d696e69736d5f636c617373203d2065786163742d627974650a6d6174685f666964656c697479203d206269742d737461626c650a616363756d756c6174696f6e5f74797065203d206633320a6269745f73746162696c697479203d20706f727461626c650a617564697465645f737461747573203d20617564697465640a636f73745f70726f76656e616e6365203d206d656173757265640a5b73656374696f6e3a373a70726f76656e616e63655d0a6b65726e656c5f736f75726365203d20617574686f7265640a7265766973696f6e5f62617365203d2072300a7265766973696f6e5f68617368203d20343a64656164626565660a636f73745f70726f76656e616e6365203d206d656173757265640a6e65676f74696174696f6e5f6d65746164617461203d20303a0a";

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
        .collect()
}

/// The golden document with one exact line replaced (the line must be unique in the body), re-framed with a fresh length and CRC-32.
fn with_line(old: &str, new: &str) -> Vec<u8> {
    let bytes = unhex(GOLDEN);
    let doc = parse_document(&bytes, &[4]).unwrap();
    let body = String::from_utf8(doc.body.to_vec()).unwrap();
    assert_eq!(body.matches(old).count(), 1, "{old}");
    encode_document(body.replace(old, new).as_bytes()).unwrap()
}

fn check(bytes: &[u8]) -> Result<(), IdentityError> {
    let doc = parse_document(bytes, &[4]).expect("a framed document");
    check_identity(&parse_contract(&doc).expect("field schemas hold"))
}

// 6.3-0009, -0002, -0007, -0011: the golden document's Identity section is consistent with its header and with itself.
#[test]
fn the_golden_identity_is_consistent() {
    assert_eq!(check(&unhex(GOLDEN)), Ok(()));
}

// 6.3-0009: the Identity copies equal the header's byte for byte; the header is authoritative.
#[test]
fn the_identity_copies_must_match_the_header() {
    assert_eq!(
        check(&with_line(
            "contract_version = 4\n",
            "contract_version = 5\n"
        )),
        Err(IdentityError::VersionMismatch {
            identity: "5".into(),
            header: 4
        })
    );
    // not byte-for-byte equal: a leading zero is a different string though the same number
    assert_eq!(
        check(&with_line(
            "contract_version = 4\n",
            "contract_version = 04\n"
        )),
        Err(IdentityError::VersionMismatch {
            identity: "04".into(),
            header: 4
        })
    );
    assert_eq!(
        check(&with_line(
            "contract_kind = kiss-contract\n",
            "contract_kind = kiss-kontract\n"
        )),
        Err(IdentityError::KindMismatch("kiss-kontract".into()))
    );
}

// 6.3-0002: the accept_predicate is a printable, LF-free structure_key token, carried verbatim.
#[test]
fn the_accept_predicate_is_a_structure_key_token() {
    let golden = unhex(GOLDEN);
    let doc = parse_document(&golden, &[4]).unwrap();
    let contract = parse_contract(&doc).unwrap();
    let accept = contract.get(0, "accept_predicate").unwrap();
    assert!(accept.starts_with("sk7|ew2|f32|cuda:sm89|"));
    let line = format!("accept_predicate = {accept}\n");
    // the Identity line is the first of the two accept_predicate lines (the Capabilities section carries the other): replace both
    let bytes = {
        let body = String::from_utf8(doc.body.to_vec()).unwrap();
        let bad = |v: &str| {
            encode_document(
                body.replace(&line, &format!("accept_predicate = {v}\n"))
                    .as_bytes(),
            )
            .unwrap()
        };
        [
            bad(""),
            bad("not-a-structure-key"),
            bad("sk|ew2|f32|cuda:sm89"),
            bad("skx|ew2|f32|cuda:sm89|ix32"),
            bad("sk7||f32|cuda:sm89|ix32"),
            bad("sk7|ew2|f32"),
            bad("sk7|ew2|f32|cuda:sm89\x07|ix32"),
        ]
    };
    for b in &bytes {
        assert_eq!(check(b), Err(IdentityError::AcceptPredicate));
    }
}

// 6.3-0007: the target_capability is `<namespace>:<capability-set>`, matched byte-exact.
#[test]
fn the_target_capability_is_namespaced() {
    for bad in [
        "sm89",
        ":sm89",
        "cuda:",
        "cuda: sm89",
        "cuda:sm89 ",
        "cuda sm89",
        "",
    ] {
        assert_eq!(
            check(&with_line(
                "target_capability = cuda:sm89\n",
                &format!("target_capability = {bad}\n")
            )),
            Err(IdentityError::TargetCapabilityShape(bad.into())),
            "{bad:?}"
        );
    }
}

// 6.3-0011: the structure_key's target component equals target_capability byte for byte: no ordering, no prefix, no case folding.
#[test]
fn the_target_component_must_equal_the_target_capability() {
    for other in [
        "cuda:sm90",
        "cuda:sm8",
        "cuda:sm89a",
        "CUDA:sm89",
        "vulkan:1.3",
    ] {
        assert_eq!(
            check(&with_line(
                "target_capability = cuda:sm89\n",
                &format!("target_capability = {other}\n")
            )),
            Err(IdentityError::TargetMismatch {
                accept_predicate: "cuda:sm89".into(),
                target_capability: other.into()
            }),
            "{other}"
        );
    }
}

// 6.3-0010: kernel_name is the stable identity name and may differ from the Interface entry_point: no rule ties them.
#[test]
fn kernel_name_need_not_equal_the_entry_point() {
    let golden = unhex(GOLDEN);
    let doc = parse_document(&golden, &[4]).unwrap();
    let c = parse_contract(&doc).unwrap();
    assert_ne!(c.get(0, "kernel_name"), c.get(2, "entry_point"));
    assert_eq!(check_identity(&c), Ok(()));
}
