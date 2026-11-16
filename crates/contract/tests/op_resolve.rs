// SPDX-License-Identifier: MIT OR Apache-2.0
//! KISS's `op_decomposition.json` (kiss origin/main b84ca69, KISS#833, 0.18.9; vendored byte-identical as `tests/data/kiss_op_decomposition.json`):
//! the table's `level`/`floor` must equal our walk of its `direct` edges (6.4-0005), and every published outcome vector (6.4-0010) must be
//! reproduced. The vectors are read from the file's own `outcomes` array. Draft-tracking: re-vendor when kiss moves.

use fireline_contract::op_dag::parse_op_dag;
use fireline_contract::op_resolve::{DecompError, DecompTable, Resolution};

const FILE: &str = include_str!("data/kiss_op_decomposition.json");

fn dag(s: &str) -> fireline_contract::op_dag::OpDag<'_> {
    parse_op_dag(s).expect("dag")
}

fn table() -> DecompTable {
    DecompTable::from_json(FILE).expect("vendored file reads")
}

#[test]
fn recorded_level_and_floor_equal_the_walk() {
    let t = table();
    assert_eq!(t.len(), 128);
    assert_eq!(t.check_recorded(), None);
    // controls: the walk is not trivially level 0, and a known chain is exactly the 6.13 body
    assert!(
        t.names()
            .filter(|n| t.walk(n).is_some_and(|(l, _)| l > 0))
            .count()
            > 50
    );
    assert_eq!(t.walk("matmul").map(|(l, _)| l), Some(1));
    let (l, f) = t.walk("avg_pool").unwrap();
    assert_eq!(
        (l, f.into_iter().collect::<Vec<_>>()),
        (2, vec!["div".to_owned(), "reduce".to_owned()])
    );
    // mutation control: a table whose recorded level is wrong is caught (anchor must match exactly once)
    let needle = "\"matmul\": {\n      \"kind\": \"non_primitive\",\n      \"level\": 1,";
    assert_eq!(FILE.matches(needle).count(), 1, "mutation anchor");
    let bad = FILE.replace(needle, &needle.replace("\"level\": 1", "\"level\": 2"));
    // level 2 still strictly exceeds its children, so it loads, and check_recorded names the op
    assert_eq!(
        DecompTable::from_json(&bad).unwrap().check_recorded(),
        Some("matmul")
    );
}

#[test]
fn every_published_outcome_vector_is_reproduced() {
    let t = table();
    let root = serde_free_outcomes(FILE);
    assert_eq!(root.len(), 11, "vector count in this vendored file");
    let mut seen = (false, false, false);
    for (op, unavailable, want) in &root {
        let un: Vec<&str> = unavailable.iter().map(String::as_str).collect();
        assert_eq!(&t.resolve(op, &un), want, "{op} with {un:?} unavailable");
        match want {
            Resolution::Resolved => seen.0 = true,
            Resolution::Deferred(_) => seen.1 = true,
            Resolution::Decline => seen.2 = true,
        }
    }
    assert_eq!(seen, (true, true, true));
    // an unavailable contract is not a decline; a floor op needs no contract
    assert_ne!(
        t.resolve("matmul", &["matmul"]),
        t.resolve("frobnicate", &[])
    );
    assert_eq!(t.resolve("add", &["add"]), Resolution::Resolved);
    assert_eq!(
        t.resolve("avg_pool", &["reduce_mean"]),
        Resolution::Deferred("reduce_mean".into())
    );
}

#[test]
fn dag_aggregation_declines_first_then_smallest_deferred() {
    let t = table();
    // control: the single-op case equals the per-op answer
    assert_eq!(
        t.resolve_dag(&dag("[Op{softmax; ; []}]"), &[]),
        Resolution::Resolved
    );
    assert_eq!(
        t.resolve_dag(&dag("[Op{softmax; ; []}]"), &["softmax"]),
        Resolution::Deferred("softmax".into())
    );
    // an op outside KISS-Ops declines even when another node defers
    assert_eq!(
        t.resolve_dag(
            &dag("[Op{softmax; ; []}, Op{frobnicate; ; [0]}]"),
            &["softmax"]
        ),
        Resolution::Decline
    );
    assert_eq!(t.resolve_dag(&dag("[]"), &[]), Resolution::Resolved);
}

#[test]
fn a_cyclic_or_dangling_table_is_refused() {
    let dangling = FILE.replacen(
        "\"direct\": [\n        \"asin\",",
        "\"direct\": [\n        \"no_such_op\",",
        1,
    );
    assert_ne!(dangling, FILE, "mutation applied");
    assert!(matches!(
        DecompTable::from_json(&dangling),
        Err(DecompError::Unresolvable(_))
    ));
    assert!(matches!(
        DecompTable::from_json("{"),
        Err(DecompError::Json(_))
    ));
    assert!(matches!(
        DecompTable::from_json("{\"schema\":\"x\"}"),
        Err(DecompError::Schema(_))
    ));
}

/// The `outcomes` array, read with a minimal scan so the test needs no JSON crate (the crate's reader is private).
fn serde_free_outcomes(text: &str) -> Vec<(String, Vec<String>, Resolution)> {
    let start = text.find("\"outcomes\"").expect("outcomes");
    let body = &text[start..];
    let mut out = Vec::new();
    for chunk in body.split('{').skip(1) {
        let obj = chunk.split('}').next().unwrap();
        let str_of = |k: &str| {
            let i = obj.find(&format!("\"{k}\":"))?;
            let rest = obj[i + k.len() + 3..].trim_start().strip_prefix('"')?;
            Some(rest[..rest.find('"')?].to_owned())
        };
        let un = obj.find("\"unavailable\":").map_or_else(Vec::new, |i| {
            let rest = &obj[i..];
            let inner = &rest[rest.find('[').unwrap() + 1..rest.find(']').unwrap()];
            inner
                .split('"')
                .skip(1)
                .step_by(2)
                .map(str::to_owned)
                .collect()
        });
        let want = match str_of("outcome").as_deref() {
            Some("resolved") => Resolution::Resolved,
            Some("deferred") => Resolution::Deferred(str_of("missing").unwrap()),
            Some("decline") => Resolution::Decline,
            other => panic!("unknown outcome {other:?}"),
        };
        out.push((str_of("op").unwrap(), un, want));
    }
    out
}
