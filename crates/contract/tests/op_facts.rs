// SPDX-License-Identifier: MIT OR Apache-2.0
//! KISS-CONTRACT 6.3-0006 over KISS's real op-facts sidecar, vendored at `tests/data/kiss_op_facts.json` (kiss origin/main 96c8ca8, 0.18.3,
//! KISS#825, `conformance/corpus/op_facts.json`, schema `kiss-op-facts-v1`). Draft-tracking: re-paste when kiss moves.

use fireline_contract::op_facts::*;

const SIDECAR: &str = include_str!("data/kiss_op_facts.json");

fn key(class: &str) -> String {
    format!("sk8|{class}|f32|cuda:sm89|ix32|grid|r2|-")
}

fn table() -> FactTable {
    FactTable::from_json(SIDECAR).expect("sidecar")
}

fn unsupported(class: &str, requires: &'static str) -> Result<(), CellClassError> {
    Err(CellClassError::Unsupported {
        class: class.into(),
        requires,
    })
}

#[test]
fn the_cell_class_is_the_second_component() {
    assert_eq!(cell_class("sk8|ew2|f32|x"), Some("ew2"));
    assert_eq!(cell_class("sk3|oth"), Some("oth"));
    for bad in ["", "sk8", "sk8|", "ew2|f32", "skx|ew2|f32", "sk|ew2|f32"] {
        assert_eq!(cell_class(bad), None, "{bad:?}");
    }
}

// 6.3-0006: ctr needs a contraction, ndx the index-operand role, rdc a reduce axis, "in exactly this sense and no other".
#[test]
fn each_structural_class_needs_its_own_fact() {
    let t = table();
    let ok = |class: &str, op: &str| check_cell_class_supported(&key(class), &[op], &t);
    assert_eq!(ok("ctr", "matmul"), Ok(()));
    assert_eq!(ok("ndx", "gather"), Ok(()));
    assert_eq!(ok("rdc", "reduce"), Ok(()));
    assert_eq!(ok("ctr", "gather"), unsupported("ctr", "contraction"));
    assert_eq!(ok("ctr", "add"), unsupported("ctr", "contraction"));
    assert_eq!(ok("ndx", "matmul"), unsupported("ndx", "index-role"));
    assert_eq!(ok("rdc", "matmul"), unsupported("rdc", "reduce-axis"));
}

// The element-wise classes and `oth` are decided by operand shape alone: they impose nothing on the op, so even an unknown op passes.
#[test]
fn shape_classes_impose_nothing() {
    let t = table();
    for class in ["ew1", "ew2", "ew3", "oth"] {
        for op in ["matmul", "add", "no_such_op"] {
            assert_eq!(
                check_cell_class_supported(&key(class), &[op], &t),
                Ok(()),
                "{class} {op}"
            );
        }
    }
}

// A fusion meets the requirement if ANY op of its Semantics DAG has the fact (a fused contraction need not be the root).
#[test]
fn a_fusion_needs_one_op_with_the_fact() {
    let t = table();
    assert_eq!(
        check_cell_class_supported(&key("ctr"), &["add", "matmul", "add"], &t),
        Ok(())
    );
    assert_eq!(
        check_cell_class_supported(&key("rdc"), &["add", "matmul"], &t),
        unsupported("rdc", "reduce-axis")
    );
}

#[test]
fn unreadable_inputs_decline() {
    let t = table();
    assert_eq!(
        check_cell_class_supported("garbage", &["add"], &t),
        Err(CellClassError::BadStructureKey)
    );
    assert_eq!(
        check_cell_class_supported(&key("ctr"), &[], &t),
        Err(CellClassError::NoOps)
    );
    assert_eq!(
        check_cell_class_supported(&key("ctr"), &["no_such_op"], &t),
        Err(CellClassError::UnknownOp("no_such_op".into()))
    );
}

#[test]
fn the_sidecar_loads_whole() {
    let t = table();
    assert_eq!(t.len(), 128);
    assert_eq!(t.facts("matmul").map(|f| f.contraction), Some(true));
    assert_eq!(t.facts("embedding").map(|f| f.index_role), Some(true));
    // reduce_axes (reduce, reduce_mean, ...) and norm_axes (softmax, ...) both count as a declared reduce axis.
    for op in [
        "reduce",
        "reduce_mean",
        "prefix_scan",
        "softmax",
        "layer_norm",
    ] {
        assert_eq!(t.facts(op).map(|f| f.reduce_axis), Some(true), "{op}");
    }
    assert_eq!(t.facts("add"), Some(OpFacts::default()));
}

// Known gap: a non-carrier scan op resolves its axis from its inner carrier (6.19-0036), which the sidecar does not carry, so rdc declines.
#[test]
fn a_non_carrier_scan_op_is_not_seen_as_reducing() {
    assert_eq!(
        check_cell_class_supported(&key("rdc"), &["cumsum"], &table()),
        unsupported("rdc", "reduce-axis")
    );
}

#[test]
fn the_loader_refuses_other_documents() {
    let j = |s: &str| FactTable::from_json(s);
    assert!(matches!(j(""), Err(FactsError::Json(_))));
    assert!(matches!(j("{\"a\":1,}"), Err(FactsError::Json(_))));
    assert!(matches!(j("{} x"), Err(FactsError::Json(_))));
    assert!(matches!(j("{\"a\":1,\"a\":2}"), Err(FactsError::Json(_))));
    assert!(matches!(
        j("{\"schema\":\"v2\",\"ops\":{}}"),
        Err(FactsError::Schema(_))
    ));
    assert!(matches!(
        j("{\"schema\":\"kiss-op-facts-v1\"}"),
        Err(FactsError::Schema(_))
    ));
    let op = |body: &str| format!("{{\"schema\":\"kiss-op-facts-v1\",\"ops\":{{\"x\":{body}}}}}");
    assert!(matches!(j(&op("{}")), Err(FactsError::Schema(_))));
    assert!(matches!(
        j(&op(
            "{\"is_contraction\":1,\"index_operand_role\":false,\"reduce_axes\":null}"
        )),
        Err(FactsError::Schema(_))
    ));
    assert!(
        j(&op(
            "{\"is_contraction\":true,\"index_operand_role\":false,\"reduce_axes\":\"reduce_axes\"}"
        ))
        .is_ok()
    );
    assert_eq!(
        j("{\"schema\":\"kiss-op-facts-v1\",\"ops\":{}}").map(|t| t.is_empty()),
        Ok(true)
    );
}
