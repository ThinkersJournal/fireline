// SPDX-License-Identifier: MIT OR Apache-2.0
//! `read_contract`: the golden document of KISS `contract_vectors.json` (origin/main 73c341b5) passes every layer, and the first fault names its
//! layer (the corpus's 13 framing declines reach `Document`; the other layers' cases are written from the clause text and are this crate's own).
//! Draft-tracking: expect rework when the Draft moves.

use fireline_contract::document::encode_document;
use fireline_contract::read::*;
use fireline_contract::{Expr, ParseError};

const GOLDEN: &str = "4b495343206b6973732d636f6e74726163742034206c656e3d313238312063726333323d65313864393561640a5b73656374696f6e3a313a6964656e746974795d0a636f6e74726163745f6b696e64203d206b6973732d636f6e74726163740a636f6e74726163745f76657273696f6e203d20340a6b65726e656c5f6e616d65203d206164645f6633325f737472696465645f736d38390a7265766973696f6e5f68617368203d20343a64656164626565660a6163636570745f707265646963617465203d20736b377c6577327c6633327c637564613a736d38397c697833327c677269647c72327c636f2f30302f76342f6431362f663b73742f30302f76312f6431362f663b636f2f30302f76342f6431362f667c2d0a6f705f6964656e74697479203d206164640a7461726765745f6361706162696c697479203d20637564613a736d38390a5b73656374696f6e3a323a73656d616e746963735d0a73656d616e746963735f6b696e64203d206d616368696e652d636865636b61626c652d49520a6f705f646167203d205b4f707b6164643b203b205b5d7d5d0a5b73656374696f6e3a333a696e746572666163655d0a656e7472795f706f696e74203d206b0a72616e6b203d20320a706f736974696f6e616c5f7369676e6174757265203d205b5d0a6c61756e63685f7363616c617273203d205b5d0a636f756e745f756e6974203d20656c656d656e74730a696e5f706c616365203d2066616c73650a616c69676e6d656e745f6279746573203d20310a5b73656374696f6e3a343a64697370617463685d0a64697370617463685f6d6f64656c203d2067656f6d657472792d61676e6f737469630a5b73656374696f6e3a353a6361706162696c69746965735d0a6163636570745f707265646963617465203d20736b377c6577327c6633327c637564613a736d38397c697833327c677269647c72327c636f2f30302f76342f6431362f663b73742f30302f76312f6431362f663b636f2f30302f76342f6431362f667c2d0a737570706f727465645f64747970655f736574203d205b6633325d0a61776b776172645f6c61796f75745f7374726174656779203d206465636c696e650a696e5f706c6163655f656c696769626c655f76617269616e7473203d205b5d0a696e6465785f7769647468203d20697833320a64657465726d696e69736d5f636c617373203d2065786163742d627974650a707265636973696f6e5f636c617373203d207374726963740a636f7374203d20310a5b73656374696f6e3a363a67756172616e746565735d0a7265666572656e63655f66756e6374696f6e203d206164640a7065725f6261636b656e645f756c705f7469657273203d205b637564613a736d38397b6d61785f756c703d307d5d0a64657465726d696e69736d5f636c617373203d2065786163742d627974650a6d6174685f666964656c697479203d206269742d737461626c650a616363756d756c6174696f6e5f74797065203d206633320a6269745f73746162696c697479203d20706f727461626c650a617564697465645f737461747573203d20617564697465640a636f73745f70726f76656e616e6365203d206d656173757265640a5b73656374696f6e3a373a70726f76656e616e63655d0a6b65726e656c5f736f75726365203d20617574686f7265640a7265766973696f6e5f62617365203d2072300a7265766973696f6e5f68617368203d20343a64656164626565660a636f73745f70726f76656e616e6365203d206d656173757265640a6e65676f74696174696f6e5f6d65746164617461203d20303a0a";

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
        .collect()
}

/// The golden document with exact substrings replaced (each must occur once), re-framed.
fn edited(replacements: &[(&str, &str)]) -> Vec<u8> {
    let bytes = unhex(GOLDEN);
    let doc = fireline_contract::document::parse_document(&bytes, &[4]).unwrap();
    let mut body = String::from_utf8(doc.body.to_vec()).unwrap();
    for (old, new) in replacements {
        assert_eq!(body.matches(old).count(), 1, "{old}");
        body = body.replace(old, new);
    }
    encode_document(body.as_bytes()).unwrap()
}

#[test]
fn the_golden_document_passes_every_layer() {
    let bytes = unhex(GOLDEN);
    let c = read_contract(&bytes, &[4]).expect("the golden document");
    assert_eq!(c.document.contract_version, 4);
    assert_eq!(
        c.contract.get(0, "kernel_name"),
        Some("add_f32_strided_sm89")
    );
    assert_eq!(c.values.rank, 2);
    assert!(c.signature.args.is_empty());
    // the context is usable straight away: the golden contract declares no launch scalar, so nothing is in its vocabulary
    assert_eq!(
        Expr::parse_in("n", &c.context),
        Err(ParseError::UnknownSymbol("n".into()))
    );
    assert!(Expr::parse_in("7 + 1", &c.context).is_ok());
}

// The first fault wins, and it names its layer.
#[test]
fn the_first_fault_names_its_layer() {
    let golden = unhex(GOLDEN);
    // framing
    assert!(
        matches!(read_contract(&golden, &[2]), Err(ReadError::Document(_))),
        "an unsupported version"
    );
    assert!(
        matches!(
            read_contract(&golden[..golden.len() - 1], &[4]),
            Err(ReadError::Document(_))
        ),
        "truncated"
    );
    assert!(matches!(
        read_contract(b"nope", &[4]),
        Err(ReadError::Document(_))
    ));
    // field schema: the Interface fields out of order
    let reordered = edited(&[(
        "entry_point = k
rank = 2
",
        "rank = 2
entry_point = k
",
    )]);
    assert!(matches!(
        read_contract(&reordered, &[4]),
        Err(ReadError::Fields(_))
    ));
    // identity
    assert!(matches!(
        read_contract(
            &edited(&[("contract_version = 4\n", "contract_version = 5\n")]),
            &[4]
        ),
        Err(ReadError::Identity(_))
    ));
    // consistency
    assert!(matches!(
        read_contract(
            &edited(&[(
                "cost_provenance = measured\nnegotiation_metadata",
                "cost_provenance = declared\nnegotiation_metadata"
            )]),
            &[4]
        ),
        Err(ReadError::Consistency(_))
    ));
    // typed values
    assert!(matches!(
        read_contract(&edited(&[("rank = 2\n", "rank = two\n")]), &[4]),
        Err(ReadError::Values(_))
    ));
    assert!(matches!(
        read_contract(
            &edited(&[("alignment_bytes = 1\n", "alignment_bytes = 0\n")]),
            &[4]
        ),
        Err(ReadError::Values(_))
    ));
    // dispatch expression in the contract's own vocabulary
    let geometry = edited(&[(
        "dispatch_model = geometry-agnostic\n",
        "invocation_domain = n\nworkgroup_sizing = 256\ncount_to_grid = 1\nthread_mapping = 1\naddressing_rule = 0\n",
    )]);
    assert!(
        matches!(read_contract(&geometry, &[4]), Err(ReadError::Values(_))),
        "`n` is not declared by the golden contract"
    );
}

// Layers run in order: a document with two faults reports the earlier layer's.
#[test]
fn the_earlier_layer_wins() {
    let two = edited(&[
        ("contract_version = 4\n", "contract_version = 5\n"),
        ("rank = 2\n", "rank = two\n"),
    ]);
    assert!(matches!(
        read_contract(&two, &[4]),
        Err(ReadError::Identity(_))
    ));
}
