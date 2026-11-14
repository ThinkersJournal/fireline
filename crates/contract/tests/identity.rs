// SPDX-License-Identifier: MIT OR Apache-2.0
//! The Identity section's own rules (KISS-CONTRACT-6.3-0002, -0007, -0009, -0011, -0010): the golden document of KISS `contract_vectors.json`
//! (origin/main 73c341b5) passes, and every fault is written from the clause text (the corpus has one value per field, so those cases are
//! this crate's own). Draft-tracking: expect rework when the Draft moves.

use fireline_contract::document::{encode_document, parse_document};
use fireline_contract::fields::parse_contract;
use fireline_contract::identity::*;

const GOLDEN: &str = "4B495343206B6973732D636F6E74726163742034206C656E3D313238302063726333323D37396635303365340A5B73656374696F6E3A313A6964656E746974795D0A636F6E74726163745F6B696E64203D206B6973732D636F6E74726163740A636F6E74726163745F76657273696F6E203D20340A6B65726E656C5F6E616D65203D206164645F6633325F737472696465645F736D38390A7265766973696F6E5F68617368203D20343A64656164626565660A6163636570745F707265646963617465203D20736B377C6577327C6633327C637564613A736D38397C697833327C677269647C72327C636F2F30302F76342F6431362F663B73742F30302F76312F6431362F663B636F2F30302F76342F6431362F667C2D0A6F705F6964656E74697479203D206164640A7461726765745F6361706162696C697479203D20637564613A736D38390A5B73656374696F6E3A323A73656D616E746963735D0A73656D616E746963735F6B696E64203D206D616368696E652D636865636B61626C652D49520A6F705F646167203D205B4F707B6164643B203B205B5D7D5D0A5B73656374696F6E3A333A696E746572666163655D0A656E7472795F706F696E74203D206B0A72616E6B203D20320A706F736974696F6E616C5F7369676E6174757265203D205B5D0A6C61756E63685F7363616C617273203D205B5D0A636F756E745F756E6974203D20656C656D656E74730A696E5F706C616365203D206E6F6E650A616C69676E6D656E745F6279746573203D20310A5B73656374696F6E3A343A64697370617463685D0A64697370617463685F6D6F64656C203D2067656F6D657472792D61676E6F737469630A5B73656374696F6E3A353A6361706162696C69746965735D0A6163636570745F707265646963617465203D20736B377C6577327C6633327C637564613A736D38397C697833327C677269647C72327C636F2F30302F76342F6431362F663B73742F30302F76312F6431362F663B636F2F30302F76342F6431362F667C2D0A737570706F727465645F64747970655F736574203D205B6633325D0A61776B776172645F6C61796F75745F7374726174656779203D206465636C696E650A696E5F706C6163655F656C696769626C655F76617269616E7473203D205B5D0A696E6465785F7769647468203D20697833320A64657465726D696E69736D5F636C617373203D2065786163742D627974650A707265636973696F6E5F636C617373203D207374726963740A636F7374203D20310A5B73656374696F6E3A363A67756172616E746565735D0A7265666572656E63655F66756E6374696F6E203D206164640A7065725F6261636B656E645F756C705F7469657273203D205B637564613A736D38397B6D61785F756C703D307D5D0A64657465726D696E69736D5F636C617373203D2065786163742D627974650A6D6174685F666964656C697479203D206269742D737461626C650A616363756D756C6174696F6E5F74797065203D206633320A6269745F73746162696C697479203D20706F727461626C650A617564697465645F737461747573203D20617564697465640A636F73745F70726F76656E616E6365203D206D656173757265640A5B73656374696F6E3A373A70726F76656E616E63655D0A6B65726E656C5F736F75726365203D20617574686F7265640A7265766973696F6E5F62617365203D2072300A7265766973696F6E5F68617368203D20343A64656164626565660A636F73745F70726F76656E616E6365203D206D656173757265640A6E65676F74696174696F6E5F6D65746164617461203D20303A0A";

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
