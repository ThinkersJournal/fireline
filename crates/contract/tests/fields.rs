// SPDX-License-Identifier: MIT OR Apache-2.0
//! The field lines and field schemas of the seven sections (KISS-CONTRACT-6.11-0001, -0005 and 6.3-0001, 6.4-0001, 6.5-0001, 6.6-0001,
//! 6.7-0001, 6.8-0001, 6.9-0001): the golden document of KISS `contract_vectors.json` (origin/main 73c341b5) parsed, and every fault
//! written from the clause text (the golden document has exactly one value per field, so those cases are this crate's own).
//! Draft-tracking: expect rework when the Draft moves.

use fireline_contract::document::{encode_document, parse_document};
use fireline_contract::fields::*;

const GOLDEN: &str = "4B495343206B6973732D636F6E74726163742034206C656E3D313238302063726333323D37396635303365340A5B73656374696F6E3A313A6964656E746974795D0A636F6E74726163745F6B696E64203D206B6973732D636F6E74726163740A636F6E74726163745F76657273696F6E203D20340A6B65726E656C5F6E616D65203D206164645F6633325F737472696465645F736D38390A7265766973696F6E5F68617368203D20343A64656164626565660A6163636570745F707265646963617465203D20736B377C6577327C6633327C637564613A736D38397C697833327C677269647C72327C636F2F30302F76342F6431362F663B73742F30302F76312F6431362F663B636F2F30302F76342F6431362F667C2D0A6F705F6964656E74697479203D206164640A7461726765745F6361706162696C697479203D20637564613A736D38390A5B73656374696F6E3A323A73656D616E746963735D0A73656D616E746963735F6B696E64203D206D616368696E652D636865636B61626C652D49520A6F705F646167203D205B4F707B6164643B203B205B5D7D5D0A5B73656374696F6E3A333A696E746572666163655D0A656E7472795F706F696E74203D206B0A72616E6B203D20320A706F736974696F6E616C5F7369676E6174757265203D205B5D0A6C61756E63685F7363616C617273203D205B5D0A636F756E745F756E6974203D20656C656D656E74730A696E5F706C616365203D206E6F6E650A616C69676E6D656E745F6279746573203D20310A5B73656374696F6E3A343A64697370617463685D0A64697370617463685F6D6F64656C203D2067656F6D657472792D61676E6F737469630A5B73656374696F6E3A353A6361706162696C69746965735D0A6163636570745F707265646963617465203D20736B377C6577327C6633327C637564613A736D38397C697833327C677269647C72327C636F2F30302F76342F6431362F663B73742F30302F76312F6431362F663B636F2F30302F76342F6431362F667C2D0A737570706F727465645F64747970655F736574203D205B6633325D0A61776B776172645F6C61796F75745F7374726174656779203D206465636C696E650A696E5F706C6163655F656C696769626C655F76617269616E7473203D205B5D0A696E6465785F7769647468203D20697833320A64657465726D696E69736D5F636C617373203D2065786163742D627974650A707265636973696F6E5F636C617373203D207374726963740A636F7374203D20310A5B73656374696F6E3A363A67756172616E746565735D0A7265666572656E63655F66756E6374696F6E203D206164640A7065725F6261636B656E645F756C705F7469657273203D205B637564613A736D38397B6D61785F756C703D307D5D0A64657465726D696E69736D5F636C617373203D2065786163742D627974650A6D6174685F666964656C697479203D206269742D737461626C650A616363756D756C6174696F6E5F74797065203D206633320A6269745F73746162696C697479203D20706F727461626C650A617564697465645F737461747573203D20617564697465640A636F73745F70726F76656E616E6365203D206D656173757265640A5B73656374696F6E3A373A70726F76656E616E63655D0A6B65726E656C5F736F75726365203D20617574686F7265640A7265766973696F6E5F62617365203D2072300A7265766973696F6E5F68617368203D20343A64656164626565660A636F73745F70726F76656E616E6365203D206D656173757265640A6E65676F74696174696F6E5F6D65746164617461203D20303A0A";

/// The keys of each section of the golden document, in order.
const GOLDEN_KEYS: [&[&str]; 7] = [
    &[
        "contract_kind",
        "contract_version",
        "kernel_name",
        "revision_hash",
        "accept_predicate",
        "op_identity",
        "target_capability",
    ],
    &["semantics_kind", "op_dag"],
    &[
        "entry_point",
        "rank",
        "positional_signature",
        "launch_scalars",
        "count_unit",
        "in_place",
        "alignment_bytes",
    ],
    &["dispatch_model"],
    &[
        "accept_predicate",
        "supported_dtype_set",
        "awkward_layout_strategy",
        "in_place_eligible_variants",
        "index_width",
        "determinism_class",
        "precision_class",
        "cost",
    ],
    &[
        "reference_function",
        "per_backend_ulp_tiers",
        "determinism_class",
        "math_fidelity",
        "accumulation_type",
        "bit_stability",
        "audited_status",
        "cost_provenance",
    ],
    &[
        "kernel_source",
        "revision_base",
        "revision_hash",
        "cost_provenance",
        "negotiation_metadata",
    ],
];

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
        .collect()
}

/// A framed document from seven section bodies (each the text after its heading line).
fn doc(sections: [&str; 7]) -> Vec<u8> {
    let names = [
        "identity",
        "semantics",
        "interface",
        "dispatch",
        "capabilities",
        "guarantees",
        "provenance",
    ];
    let mut body = String::new();
    for (i, s) in sections.iter().enumerate() {
        body.push_str(&format!("[section:{}:{}]\n{s}", i + 1, names[i]));
    }
    encode_document(body.as_bytes()).unwrap()
}

fn parse(bytes: &[u8]) -> Result<Contract<'_>, FieldError> {
    parse_contract(&parse_document(bytes, &[4]).expect("a framed document"))
}

const IDENTITY: &str = "contract_kind = kiss-contract\ncontract_version = 4\nkernel_name = k\nrevision_hash = 4:deadbeef\naccept_predicate = p\nop_identity = o\ntarget_capability = t\n";
const SEMANTICS: &str = "semantics_kind = declared-op-tag\nop_dag = []\n";
const INTERFACE: &str = "entry_point = k\nrank = 2\npositional_signature = []\nlaunch_scalars = []\ncount_unit = elements\nin_place = none\nalignment_bytes = 1\n";
const DISPATCH_AGNOSTIC: &str = "dispatch_model = geometry-agnostic\n";
const DISPATCH_GEOMETRY: &str = "invocation_domain = a\nworkgroup_sizing = b\ncount_to_grid = c\nthread_mapping = d\naddressing_rule = e\n";
const CAPABILITIES: &str = "accept_predicate = p\nsupported_dtype_set = [f32]\nawkward_layout_strategy = decline\nin_place_eligible_variants = []\nindex_width = ix32\ndeterminism_class = exact-byte\nprecision_class = strict\ncost = 1\n";
const GUARANTEES: &str = "reference_function = add\nper_backend_ulp_tiers = []\ndeterminism_class = exact-byte\nmath_fidelity = bit-stable\naccumulation_type = f32\nbit_stability = portable\naudited_status = audited\ncost_provenance = measured\n";
const PROVENANCE: &str = "kernel_source = authored\nrevision_base = r0\nrevision_hash = 4:deadbeef\ncost_provenance = measured\nnegotiation_metadata = 0:\n";

fn ok_sections() -> [&'static str; 7] {
    [
        IDENTITY,
        SEMANTICS,
        INTERFACE,
        DISPATCH_AGNOSTIC,
        CAPABILITIES,
        GUARANTEES,
        PROVENANCE,
    ]
}

// 6.11-0005 and the seven schemas: the golden document's every section carries exactly its schema's keys in order.
#[test]
fn the_golden_document_parses_to_its_fields() {
    let bytes = unhex(GOLDEN);
    let c = parse(&bytes).expect("the golden document");
    assert_eq!(c.contract_version, 4);
    for (i, keys) in GOLDEN_KEYS.iter().enumerate() {
        let got: Vec<&str> = c.sections[i].iter().map(|f| f.key).collect();
        assert_eq!(&got, keys, "section {}", i + 1);
    }
    for (section, key, value) in [
        (0, "kernel_name", "add_f32_strided_sm89"),
        (0, "contract_version", "4"),
        (2, "entry_point", "k"),
        (2, "rank", "2"),
        (2, "positional_signature", "[]"),
        (3, "dispatch_model", "geometry-agnostic"),
        (4, "supported_dtype_set", "[f32]"),
        (5, "math_fidelity", "bit-stable"),
        (6, "negotiation_metadata", "0:"),
    ] {
        assert_eq!(c.get(section, key), Some(value), "{key}");
    }
    assert_eq!(c.get(0, "no_such_field"), None);
    assert_eq!(c.get(9, "kernel_name"), None);
}

#[test]
fn the_minimal_valid_document_parses_and_each_alternative_is_accepted() {
    assert!(parse(&doc(ok_sections())).is_ok());
    let mut s = ok_sections();
    s[3] = DISPATCH_GEOMETRY;
    assert!(parse(&doc(s)).is_ok(), "the five geometry fields");
}

// 6.4-0001, 6.5-0001: an optional field is omitted when absent, in its pinned last place when present; its absence is not a decline.
#[test]
fn optional_fields_are_last_and_may_be_absent() {
    let mut s = ok_sections();
    let sem = format!("{SEMANTICS}human_annotation = a note\n");
    let int = format!("{INTERFACE}output_layouts = [dense]\n");
    s[1] = &sem;
    s[2] = &int;
    let bytes = doc(s);
    let c = parse(&bytes).unwrap();
    assert_eq!(c.get(1, "human_annotation"), Some("a note"));
    assert_eq!(c.get(2, "output_layouts"), Some("[dense]"));
    // present but not last: out of order
    let bad = format!("human_annotation = a note\n{SEMANTICS}");
    let mut s = ok_sections();
    s[1] = &bad;
    assert_eq!(
        parse(&doc(s)).unwrap_err(),
        FieldError::OutOfOrder {
            section: 2,
            key: "human_annotation".into()
        }
    );
}

// 6.1-0006, 6.11-0005: a missing required field, an unknown field, an extra line, a repeated or reordered field.
#[test]
fn the_field_schema_is_exact() {
    let case = |section: usize, text: String| {
        let mut s = ok_sections();
        let owned = text;
        s[section] = &owned;
        parse(&doc(s)).unwrap_err()
    };
    // missing
    assert_eq!(
        case(0, IDENTITY.replace("kernel_name = k\n", "")),
        FieldError::MissingField {
            section: 1,
            key: "kernel_name"
        }
    );
    assert_eq!(
        case(6, PROVENANCE.replace("negotiation_metadata = 0:\n", "")),
        FieldError::MissingField {
            section: 7,
            key: "negotiation_metadata"
        }
    );
    assert_eq!(
        case(1, String::new()),
        FieldError::MissingField {
            section: 2,
            key: "semantics_kind"
        }
    );
    // unknown, in the middle and after the last
    assert_eq!(
        case(
            2,
            INTERFACE.replace("rank = 2\n", "rank = 2\ntarget = cuda:sm89\n")
        ),
        FieldError::UnexpectedField {
            section: 3,
            key: "target".into()
        }
    );
    assert_eq!(
        case(4, format!("{CAPABILITIES}extra = 1\n")),
        FieldError::UnexpectedField {
            section: 5,
            key: "extra".into()
        }
    );
    // reordered, and repeated
    assert_eq!(
        case(
            5,
            GUARANTEES.replace(
                "determinism_class = exact-byte\nmath_fidelity = bit-stable\n",
                "math_fidelity = bit-stable\ndeterminism_class = exact-byte\n"
            )
        ),
        FieldError::OutOfOrder {
            section: 6,
            key: "math_fidelity".into()
        }
    );
    assert_eq!(
        case(2, format!("{INTERFACE}rank = 2\n")),
        FieldError::OutOfOrder {
            section: 3,
            key: "rank".into()
        }
    );
    // 6.5-0001: count_unit, in_place and alignment_bytes live in the Interface, not in the Capabilities; audited_status is not Provenance's
    assert_eq!(
        case(4, format!("{CAPABILITIES}count_unit = elements\n")),
        FieldError::UnexpectedField {
            section: 5,
            key: "count_unit".into()
        }
    );
    assert_eq!(
        case(6, format!("{PROVENANCE}audited_status = audited\n")),
        FieldError::UnexpectedField {
            section: 7,
            key: "audited_status".into()
        }
    );
}

// 6.6-0001: the Dispatch section is exactly one of two alternatives; a proper subset of the geometry fields, or the sentinel alongside any
// geometry field, is rejected.
#[test]
fn dispatch_is_exactly_one_of_two_alternatives() {
    let case = |text: String| {
        let mut s = ok_sections();
        let owned = text;
        s[3] = &owned;
        parse(&doc(s)).unwrap_err()
    };
    assert_eq!(
        case(DISPATCH_GEOMETRY.replace("thread_mapping = d\n", "")),
        FieldError::DispatchAlternatives
    );
    assert_eq!(
        case("invocation_domain = a\n".to_owned()),
        FieldError::DispatchAlternatives
    );
    assert_eq!(
        case(format!("{DISPATCH_AGNOSTIC}invocation_domain = a\n")),
        FieldError::DispatchAlternatives
    );
    assert_eq!(
        case(format!("invocation_domain = a\n{DISPATCH_AGNOSTIC}")),
        FieldError::DispatchAlternatives
    );
    assert_eq!(
        case("dispatch_model = geometry-aware\n".to_owned()),
        FieldError::DispatchAlternatives
    );
    assert_eq!(case(String::new()), FieldError::DispatchAlternatives);
    assert_eq!(
        case(format!("{DISPATCH_GEOMETRY}{DISPATCH_AGNOSTIC}")),
        FieldError::DispatchAlternatives
    );
}

// 6.11-0001: `<key> = <value>` on one LF-terminated line, UTF-8; nothing else.
#[test]
fn the_line_format_is_exact() {
    let case = |text: String| {
        let mut s = ok_sections();
        let owned = text;
        s[1] = &owned;
        parse(&doc(s)).unwrap_err()
    };
    for bad in [
        "semantics_kind=declared-op-tag\nop_dag = []\n",
        "semantics_kind  = declared-op-tag\nop_dag = []\n",
        "semantics_kind = declared-op-tag\r\nop_dag = []\n",
        "semantics_kind = declared-op-tag\n\nop_dag = []\n",
        " = x\nop_dag = []\n",
        "just a line\n",
    ] {
        assert!(
            matches!(case(bad.to_owned()), FieldError::BadLine { section: 2, .. }),
            "{bad:?}"
        );
    }
    // an unterminated last line (in the last section, so it is not swallowed into the next heading)
    let mut s = ok_sections();
    s[6] = PROVENANCE.trim_end_matches('\n');
    assert!(matches!(
        parse(&doc(s)).unwrap_err(),
        FieldError::BadLine { section: 7, .. }
    ));
    // a value may itself contain " = " and may be empty; a non-UTF-8 section is its own decline
    let mut s = ok_sections();
    s[1] = "semantics_kind = declared-op-tag\nop_dag = a = b\nhuman_annotation = \n";
    let bytes = doc(s);
    let c = parse(&bytes).unwrap();
    assert_eq!(
        (c.get(1, "op_dag"), c.get(1, "human_annotation")),
        (Some("a = b"), Some(""))
    );
    let mut body = b"[section:1:identity]\n".to_vec();
    body.extend_from_slice(b"contract_kind = \xff\n");
    for (i, n) in [
        "semantics",
        "interface",
        "dispatch",
        "capabilities",
        "guarantees",
        "provenance",
    ]
    .iter()
    .enumerate()
    {
        body.extend_from_slice(format!("[section:{}:{n}]\nk = v\n", i + 2).as_bytes());
    }
    let framed = encode_document(&body).unwrap();
    assert_eq!(
        parse(&framed).unwrap_err(),
        FieldError::NotUtf8 { section: 1 }
    );
}
