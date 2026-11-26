// SPDX-License-Identifier: MIT OR Apache-2.0
//! The Semantics `op_dag` reader (KISS-CONTRACT 6.11-0007, 6.4-0009, 6.4-0002) and 6.3-0006 over a contract. The one KISS-owned example is the
//! golden document's `op_dag = [Op{add; ; []}]`; every other form is written from the clause text and is this crate's own test.

use fireline_contract::document::{encode_document, parse_document};
use fireline_contract::dtypes::{DtypeSource, DtypeTable};
use fireline_contract::fields::{Contract, parse_contract};
use fireline_contract::op_dag::*;
use fireline_contract::op_facts::FactTable;

const SIDECAR: &str = include_str!("data/kiss_op_facts.json");
const DTYPES: &str = include_str!("data/kiss_dtype_manifest.json");

fn contract_bytes(accept: &str, identity: &str, kind: &str, dag: &str) -> Vec<u8> {
    let sections = [
        format!("contract_kind = kiss-contract\ncontract_version = 4\nkernel_name = k\nrevision_hash = 4:deadbeef\naccept_predicate = {accept}\nop_identity = {identity}\ntarget_capability = t\n"),
        format!("semantics_kind = {kind}\nop_dag = {dag}\n"),
        "entry_point = k\nrank = 2\npositional_signature = []\nlaunch_scalars = []\ncount_unit = elements\nin_place = none\nalignment_bytes = 1\n".to_owned(),
        "dispatch_model = geometry-agnostic\n".to_owned(),
        format!("accept_predicate = {accept}\nsupported_dtype_set = [f32]\nawkward_layout_strategy = decline\nin_place_eligible_variants = []\nindex_width = ix32\ndeterminism_class = exact-byte\nprecision_class = strict\ncost = 1\n"),
        "reference_function = add\nper_backend_ulp_tiers = []\ndeterminism_class = exact-byte\nmath_fidelity = bit-stable\naccumulation_type = f32\nbit_stability = portable\naudited_status = audited\ncost_provenance = measured\n".to_owned(),
        "kernel_source = authored\nrevision_base = r0\nrevision_hash = 4:deadbeef\ncost_provenance = measured\nnegotiation_metadata = 0:\n".to_owned(),
    ];
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

fn with<R>(
    accept: &str,
    identity: &str,
    kind: &str,
    dag: &str,
    f: impl FnOnce(&Contract<'_>) -> R,
) -> R {
    let bytes = contract_bytes(accept, identity, kind, dag);
    let doc = parse_document(&bytes, &[4]).expect("framed");
    let c = parse_contract(&doc);
    f(&c.unwrap_or_else(|e| panic!("fields: {e:?}")))
}

#[test]
fn the_golden_dag_is_one_add_node() {
    let d = parse_op_dag("[Op{add; ; []}]").unwrap();
    assert_eq!(
        d.nodes,
        vec![Node::Op {
            op_name: "add",
            op_attrs: vec![],
            child_edges: vec![]
        }]
    );
    assert_eq!(d.root_op(), Some("add"));
    assert_eq!(d.ops(), vec!["add"]);
}

#[test]
fn a_fusion_reads_in_order_root_last() {
    let text = "[Bind(0), Bind(1), Op{matmul; accumulator=f32, math_fidelity=bit-stable; [0, 1]}, Source{const; dtype=f32, bits=0x3F800000}, Op{add; ; [2, 3]}]";
    let d = parse_op_dag(text).unwrap();
    assert_eq!(d.nodes.len(), 5);
    assert_eq!(d.root_op(), Some("add"));
    assert_eq!(d.ops(), vec!["matmul", "add"]);
    assert_eq!(d.nodes[0], Node::Bind(0));
    assert_eq!(
        d.nodes[2],
        Node::Op {
            op_name: "matmul",
            op_attrs: vec![("accumulator", "f32"), ("math_fidelity", "bit-stable")],
            child_edges: vec![0, 1]
        }
    );
}

// A value that itself holds ", " or brackets (a reduce-axes mask) stays one attribute value.
#[test]
fn nested_attribute_values_stay_whole() {
    let d = parse_op_dag("[Bind(0), Op{reduce; reduce_axes=[0, 1], keepdim=1; [0]}]").unwrap();
    assert_eq!(
        d.nodes[1],
        Node::Op {
            op_name: "reduce",
            op_attrs: vec![("reduce_axes", "[0, 1]"), ("keepdim", "1")],
            child_edges: vec![0]
        }
    );
}

#[test]
fn the_empty_array_has_no_root() {
    let d = parse_op_dag("[]").unwrap();
    assert_eq!((d.root(), d.root_op(), d.ops()), (None, None, vec![]));
}

#[test]
fn source_nodes_pin_their_tag_and_attributes() {
    for ok in [
        "Source{const; dtype=f32, bits=0x3F800000}",
        "Source{param; index=3}",
        "Source{coord; axis=1}",
        "Source{extent; axis=0}",
        "Source{reduced; stage=2}",
        "Source{reduced_count; }",
    ] {
        assert!(parse_op_dag(&format!("[{ok}]")).is_ok(), "{ok}");
    }
    for bad in [
        "Source{bogus; }",
        "Source{param; axis=3}",
        "Source{param; index=-1}",
        "Source{coord; axis=256}",
        "Source{const; dtype=f32, bits=0x3f800000}",
        "Source{const; bits=0x3F, dtype=f32}",
        "Source{const; dtype=f32, bits=3F}",
        "Source{reduced_count; n=1}",
        "Source{param}",
    ] {
        assert!(
            matches!(
                parse_op_dag(&format!("[{bad}]")),
                Err(OpDagError::BadSource { index: 0 } | OpDagError::BadNode { index: 0 })
            ),
            "{bad}"
        );
    }
}

#[test]
fn children_must_be_strictly_earlier() {
    assert_eq!(
        parse_op_dag("[Op{add; ; [0]}]"),
        Err(OpDagError::ChildNotEarlier { index: 0, child: 0 })
    );
    assert_eq!(
        parse_op_dag("[Bind(0), Op{add; ; [0, 2]}, Bind(1)]"),
        Err(OpDagError::ChildNotEarlier { index: 1, child: 2 })
    );
}

#[test]
fn malformed_values_decline() {
    for bad in ["", "Op{add; ; []}", "[Op{add; ; []}", "[Op{add; ; []}]x"] {
        assert_eq!(parse_op_dag(bad), Err(OpDagError::NotAnArray), "{bad:?}");
    }
    for bad in [
        "[Op{add}]",
        "[Op{add; ;}]",
        "[Op{add; ; [] }]",
        "[Op{add; k; []}]",
        "[Op{add; k=; []}]",
        "[Op{add; k=1, k=2; []}]",
        "[Bind()]",
        "[Bind(-1)]",
        "[Bind(01)]",
        "[Bind(x)]",
        "[Node{add}]",
        "[Op{add; ; [0,1]}]",
        "[Op{add; ; []},Bind(0)]",
    ] {
        assert!(
            matches!(parse_op_dag(bad), Err(OpDagError::BadNode { index: 0 })),
            "{bad:?} -> {:?}",
            parse_op_dag(bad)
        );
    }
}

#[test]
fn the_identity_names_the_root() {
    assert_eq!(identity_op_name("add"), "add");
    assert_eq!(identity_op_name("reduce{reduce_axes=[0]}"), "reduce");
    assert_eq!(identity_op_name("matmul[lhs:f32, rhs:f32]"), "matmul");
}

fn sources() -> (FactTable, DtypeTable) {
    (
        FactTable::from_json(SIDECAR).unwrap(),
        DtypeTable::from_json(DTYPES).unwrap(),
    )
}

fn semantics(accept: &str, identity: &str, kind: &str, dag: &str) -> Result<usize, SemanticsError> {
    let (ops, dt) = sources();
    with(accept, identity, kind, dag, |c| {
        check_semantics(c, &ops, &dt).map(|d| d.nodes.len())
    })
}

fn tag(r: Result<usize, SemanticsError>) -> &'static str {
    match r {
        Ok(_) => "ok",
        Err(e) => e.wire_tag(),
    }
}

// 6.3-0006 over a contract: the fusion's ops, not just its root, are asked (KISS#829 ruling 3).
#[test]
fn the_cell_class_is_checked_over_every_op_node() {
    let fusion = "[Source{param; index=0}, Op{matmul; ; [0]}, Op{add; ; [1]}]";
    assert_eq!(
        semantics("sk8|ctr|f32|t", "add", "machine-checkable-IR", fusion),
        Ok(3)
    );
    assert_eq!(
        semantics("sk8|ew2|f32|t", "add", "machine-checkable-IR", fusion),
        Ok(3)
    );
    assert_eq!(
        tag(semantics(
            "sk8|ndx|f32|t",
            "add",
            "machine-checkable-IR",
            fusion
        )),
        "cell-class-unsupported-by-op"
    );
    // A Source or Bind node never has the fact.
    assert_eq!(
        tag(semantics(
            "sk8|ctr|f32|t",
            "add",
            "machine-checkable-IR",
            "[Source{param; index=0}, Op{add; ; [0]}]"
        )),
        "cell-class-unsupported-by-op"
    );
}

// KISS#829 rulings 1 and 2: an empty op_dag declines under machine-checkable-IR, and is exempt under declared-op-tag; any node is checked.
#[test]
fn an_empty_dag_depends_on_the_semantics_kind() {
    assert_eq!(
        tag(semantics(
            "sk8|ctr|f32|t",
            "add",
            "machine-checkable-IR",
            "[]"
        )),
        "op-dag-empty"
    );
    assert_eq!(
        semantics("sk8|ctr|f32|t", "add", "declared-op-tag", "[]"),
        Ok(0)
    );
    assert_eq!(
        tag(semantics(
            "sk8|ctr|f32|t",
            "add",
            "declared-op-tag",
            "[Op{add; ; []}]"
        )),
        "cell-class-unsupported-by-op"
    );
    assert_eq!(
        tag(semantics(
            "sk8|ew2|f32|t",
            "add",
            "lifted-IR",
            "[Op{add; ; []}]"
        )),
        "bad-semantics-kind"
    );
}

#[test]
fn the_root_is_the_identitys_op() {
    assert_eq!(
        semantics(
            "sk8|ew2|f32|t",
            "add",
            "machine-checkable-IR",
            "[Op{add; ; []}]"
        ),
        Ok(1)
    );
    assert_eq!(
        semantics(
            "sk8|ew2|f32|t",
            "add[a:f32]",
            "machine-checkable-IR",
            "[Op{add; ; []}]"
        ),
        Ok(1)
    );
    for (identity, dag) in [
        ("mul", "[Op{add; ; []}]"),
        ("add", "[Source{param; index=0}]"),
    ] {
        let r = semantics("sk8|ew2|f32|t", identity, "machine-checkable-IR", dag);
        assert_eq!(tag(r.clone()), "op-identity-root-mismatch");
        assert_eq!(r.unwrap_err().section(), "record");
    }
}

#[test]
fn names_and_widths_come_from_the_sources() {
    let u = |dag: &str| {
        tag(semantics(
            "sk8|ew2|f32|t",
            "neg",
            "machine-checkable-IR",
            dag,
        ))
    };
    assert_eq!(u("[Op{frobnicate; ; []}]"), "op-dag-unknown-op");
    assert_eq!(u("[Op{neg{axis=0}; ; []}]"), "op-dag-unknown-op");
    assert_eq!(
        u("[Source{const; dtype=f32, bits=0x3F800000}, Op{neg; ; [0]}]"),
        "ok"
    );
    assert_eq!(
        u("[Source{const; dtype=f32, bits=0x3F80}, Op{neg; ; [0]}]"),
        "op-dag-bad-source"
    );
    assert_eq!(
        u("[Source{const; dtype=f99, bits=0x3F800000}, Op{neg; ; [0]}]"),
        "op-dag-bad-source"
    );
    assert_eq!(
        u("[Source{param; index=0}, Source{param; index=1}, Op{neg; ; [0]}]"),
        "op-dag-multiple-roots"
    );
    assert_eq!(
        parse_op_dag("[Op{neg; ; [5]}]").unwrap_err().wire_tag(),
        "op-dag-child-not-earlier"
    );
}

#[test]
fn the_dtype_manifest_loads_and_refuses_other_documents() {
    let t = DtypeTable::from_json(DTYPES).unwrap();
    assert_eq!(t.len(), 34); // KISS 0.24.0 (a9ad8cf): the 27 + f128 i128 u128 i256 u256 f8e3m4 f8ue4m3
    assert_eq!(t.storage_bits("f32"), Some(32));
    assert_eq!(t.storage_bits("f99"), None);
    assert_eq!(t.storage_bits("f128"), Some(128));
    assert_eq!(t.storage_bits("i128"), Some(128));
    assert_eq!(t.storage_bits("u128"), Some(128));
    assert_eq!(t.storage_bits("i256"), Some(256));
    assert_eq!(t.storage_bits("u256"), Some(256));
    assert_eq!(t.storage_bits("f8e3m4"), Some(8));
    assert_eq!(t.storage_bits("f8ue4m3"), Some(8));
    use fireline_contract::dtypes::DtypeError;
    assert!(matches!(
        DtypeTable::from_json("{"),
        Err(DtypeError::Json(_))
    ));
    assert!(matches!(
        DtypeTable::from_json("{}"),
        Err(DtypeError::Schema(_))
    ));
    let m = |d: &str| format!("{{\"schema\":\"kiss-dtype-manifest-v1\",\"dtypes\":{d}}}");
    assert!(matches!(
        DtypeTable::from_json(&m("[{\"token\":\"a\"}]")),
        Err(DtypeError::Schema(_))
    ));
    assert!(matches!(
        DtypeTable::from_json(&m(
            "[{\"token\":\"a\",\"storage_bits\":8},{\"token\":\"a\",\"storage_bits\":8}]"
        )),
        Err(DtypeError::Schema(_))
    ));
}
