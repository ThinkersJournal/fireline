// SPDX-License-Identifier: MIT OR Apache-2.0
//! The field lines and field schemas of the seven sections (KISS-CONTRACT-6.11-0001, -0005 and 6.3-0001, 6.4-0001, 6.5-0001, 6.6-0001,
//! 6.7-0001, 6.8-0001, 6.9-0001): the golden document of KISS `contract_vectors.json` (origin/main 73c341b5) parsed, and every fault
//! written from the clause text (the golden document has exactly one value per field, so those cases are this crate's own).
//! Draft-tracking: expect rework when the Draft moves.

use fireline_contract::document::{encode_document, parse_document};
use fireline_contract::fields::*;

const GOLDEN: &str = "4b495343206b6973732d636f6e74726163742034206c656e3d313238312063726333323d65313864393561640a5b73656374696f6e3a313a6964656e746974795d0a636f6e74726163745f6b696e64203d206b6973732d636f6e74726163740a636f6e74726163745f76657273696f6e203d20340a6b65726e656c5f6e616d65203d206164645f6633325f737472696465645f736d38390a7265766973696f6e5f68617368203d20343a64656164626565660a6163636570745f707265646963617465203d20736b377c6577327c6633327c637564613a736d38397c697833327c677269647c72327c636f2f30302f76342f6431362f663b73742f30302f76312f6431362f663b636f2f30302f76342f6431362f667c2d0a6f705f6964656e74697479203d206164640a7461726765745f6361706162696c697479203d20637564613a736d38390a5b73656374696f6e3a323a73656d616e746963735d0a73656d616e746963735f6b696e64203d206d616368696e652d636865636b61626c652d49520a6f705f646167203d205b4f707b6164643b203b205b5d7d5d0a5b73656374696f6e3a333a696e746572666163655d0a656e7472795f706f696e74203d206b0a72616e6b203d20320a706f736974696f6e616c5f7369676e6174757265203d205b5d0a6c61756e63685f7363616c617273203d205b5d0a636f756e745f756e6974203d20656c656d656e74730a696e5f706c616365203d2066616c73650a616c69676e6d656e745f6279746573203d20310a5b73656374696f6e3a343a64697370617463685d0a64697370617463685f6d6f64656c203d2067656f6d657472792d61676e6f737469630a5b73656374696f6e3a353a6361706162696c69746965735d0a6163636570745f707265646963617465203d20736b377c6577327c6633327c637564613a736d38397c697833327c677269647c72327c636f2f30302f76342f6431362f663b73742f30302f76312f6431362f663b636f2f30302f76342f6431362f667c2d0a737570706f727465645f64747970655f736574203d205b6633325d0a61776b776172645f6c61796f75745f7374726174656779203d206465636c696e650a696e5f706c6163655f656c696769626c655f76617269616e7473203d205b5d0a696e6465785f7769647468203d20697833320a64657465726d696e69736d5f636c617373203d2065786163742d627974650a707265636973696f6e5f636c617373203d207374726963740a636f7374203d20310a5b73656374696f6e3a363a67756172616e746565735d0a7265666572656e63655f66756e6374696f6e203d206164640a7065725f6261636b656e645f756c705f7469657273203d205b637564613a736d38397b6d61785f756c703d307d5d0a64657465726d696e69736d5f636c617373203d2065786163742d627974650a6d6174685f666964656c697479203d206269742d737461626c650a616363756d756c6174696f6e5f74797065203d206633320a6269745f73746162696c697479203d20706f727461626c650a617564697465645f737461747573203d20617564697465640a636f73745f70726f76656e616e6365203d206d656173757265640a5b73656374696f6e3a373a70726f76656e616e63655d0a6b65726e656c5f736f75726365203d20617574686f7265640a7265766973696f6e5f62617365203d2072300a7265766973696f6e5f68617368203d20343a64656164626565660a636f73745f70726f76656e616e6365203d206d656173757265640a6e65676f74696174696f6e5f6d65746164617461203d20303a0a";

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
const INTERFACE: &str = "entry_point = k\nrank = 2\npositional_signature = []\nlaunch_scalars = []\ncount_unit = elements\nin_place = false\nalignment_bytes = 1\n";
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
