// SPDX-License-Identifier: MIT OR Apache-2.0
//! `read_contract`: the golden document of KISS `contract_vectors.json` (origin/main 73c341b5) passes every layer, and the first fault names its
//! layer (the corpus's 13 framing declines reach `Document`; the other layers' cases are written from the clause text and are this crate's own).
//! Draft-tracking: expect rework when the Draft moves.

use fireline_contract::document::encode_document;
use fireline_contract::read::*;
use fireline_contract::{Expr, ParseError};

const GOLDEN: &str = "4B495343206B6973732D636F6E74726163742034206C656E3D313238302063726333323D37396635303365340A5B73656374696F6E3A313A6964656E746974795D0A636F6E74726163745F6B696E64203D206B6973732D636F6E74726163740A636F6E74726163745F76657273696F6E203D20340A6B65726E656C5F6E616D65203D206164645F6633325F737472696465645F736D38390A7265766973696F6E5F68617368203D20343A64656164626565660A6163636570745F707265646963617465203D20736B377C6577327C6633327C637564613A736D38397C697833327C677269647C72327C636F2F30302F76342F6431362F663B73742F30302F76312F6431362F663B636F2F30302F76342F6431362F667C2D0A6F705F6964656E74697479203D206164640A7461726765745F6361706162696C697479203D20637564613A736D38390A5B73656374696F6E3A323A73656D616E746963735D0A73656D616E746963735F6B696E64203D206D616368696E652D636865636B61626C652D49520A6F705F646167203D205B4F707B6164643B203B205B5D7D5D0A5B73656374696F6E3A333A696E746572666163655D0A656E7472795F706F696E74203D206B0A72616E6B203D20320A706F736974696F6E616C5F7369676E6174757265203D205B5D0A6C61756E63685F7363616C617273203D205B5D0A636F756E745F756E6974203D20656C656D656E74730A696E5F706C616365203D206E6F6E650A616C69676E6D656E745F6279746573203D20310A5B73656374696F6E3A343A64697370617463685D0A64697370617463685F6D6F64656C203D2067656F6D657472792D61676E6F737469630A5B73656374696F6E3A353A6361706162696C69746965735D0A6163636570745F707265646963617465203D20736B377C6577327C6633327C637564613A736D38397C697833327C677269647C72327C636F2F30302F76342F6431362F663B73742F30302F76312F6431362F663B636F2F30302F76342F6431362F667C2D0A737570706F727465645F64747970655F736574203D205B6633325D0A61776B776172645F6C61796F75745F7374726174656779203D206465636C696E650A696E5F706C6163655F656C696769626C655F76617269616E7473203D205B5D0A696E6465785F7769647468203D20697833320A64657465726D696E69736D5F636C617373203D2065786163742D627974650A707265636973696F6E5F636C617373203D207374726963740A636F7374203D20310A5B73656374696F6E3A363A67756172616E746565735D0A7265666572656E63655F66756E6374696F6E203D206164640A7065725F6261636B656E645F756C705F7469657273203D205B637564613A736D38397B6D61785F756C703D307D5D0A64657465726D696E69736D5F636C617373203D2065786163742D627974650A6D6174685F666964656C697479203D206269742D737461626C650A616363756D756C6174696F6E5F74797065203D206633320A6269745F73746162696C697479203D20706F727461626C650A617564697465645F737461747573203D20617564697465640A636F73745F70726F76656E616E6365203D206D656173757265640A5B73656374696F6E3A373A70726F76656E616E63655D0A6B65726E656C5F736F75726365203D20617574686F7265640A7265766973696F6E5F62617365203D2072300A7265766973696F6E5F68617368203D20343A64656164626565660A636F73745F70726F76656E616E6365203D206D656173757265640A6E65676F74696174696F6E5F6D65746164617461203D20303A0A";

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
