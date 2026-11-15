// SPDX-License-Identifier: MIT OR Apache-2.0
//! KISS-CONTRACT 6.3-0006 over a **fixture** fact table. The op names and flags below are illustrative test data, not KISS-Ops' facts (those
//! come from KISS's machine-readable sidecar once it exists); they only exercise the rule's shape.

use fireline_contract::op_facts::*;

const FIXTURE: &str = "# op\tcontraction\tindex_role\treduce_axis\n\
matmul\t1\t0\t0\n\
gather\t0\t1\t0\n\
sum\t0\t0\t1\n\
add\t0\t0\t0\n";

fn key(class: &str) -> String {
    format!("sk8|{class}|f32|cuda:sm89|ix32|grid|r2|-")
}

fn table() -> FactTable {
    FactTable::parse_tsv(FIXTURE).expect("fixture")
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
    assert_eq!(ok("rdc", "sum"), Ok(()));
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
        for op in ["matmul", "add", "nonexistent"] {
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
        check_cell_class_supported(&key("ctr"), &["nonexistent"], &t),
        Err(CellClassError::UnknownOp("nonexistent".into()))
    );
}

#[test]
fn the_fixture_loader_refuses_bad_lines() {
    assert_eq!(FactTable::parse_tsv("a\t1\t0"), Err(1));
    assert_eq!(FactTable::parse_tsv("a\t1\t0\t2"), Err(1));
    assert_eq!(FactTable::parse_tsv("a\t1\t0\t0\n\na\t0\t0\t0"), Err(3));
    assert_eq!(FactTable::parse_tsv("\t1\t0\t0"), Err(1));
    assert_eq!(
        FactTable::parse_tsv("# only a comment\n"),
        Ok(FactTable::default())
    );
    assert_eq!(
        table().facts("sum"),
        Some(OpFacts {
            reduce_axis: true,
            ..OpFacts::default()
        })
    );
}
