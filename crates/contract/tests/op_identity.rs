// SPDX-License-Identifier: MIT OR Apache-2.0
//! KISS's `op_identity_vectors` (kiss origin/main 50dc005, KISS#835, board 451; vendored as `tests/data/kiss_op_identity_vectors.json`) over the
//! structure-only reader. Every vector whose expected outcome is decidable from the line, the ordered `opattrs` of `op_facts.json` and the dtype
//! tokens must be decided as KISS decided it. The `op-identity-bad-value` vectors need each field's type and vocabulary, which no corpus JSON
//! carries (KISS's reference parses `spec/ops.md`), so they are listed by name below and asserted NOT decided here, rather than quietly skipped.
//! Draft-tracking: re-vendor when kiss moves.

use fireline_contract::dtypes::DtypeTable;
use fireline_contract::op_identity::{AttrTable, parse_op_identity};

const VECTORS: &str = include_str!("data/kiss_op_identity_vectors.json");
const OPS: &str = include_str!("data/kiss_op_facts.json");
const DTYPES: &str = include_str!("data/kiss_dtype_manifest.json");

/// `(name, op_identity, expect)`; `expect` is `None` for an accept. One vector per line in the file; a vector line holds no escape.
fn vectors() -> Vec<(String, String, Option<String>)> {
    let field = |line: &str, key: &str| -> Option<String> {
        let marker = format!("\"{key}\": \"");
        let start = line.find(&marker)? + marker.len();
        let end = start + line[start..].find('"')?;
        Some(line[start..end].to_owned())
    };
    VECTORS
        .lines()
        .filter(|l| l.contains("\"verdict\""))
        .map(|l| {
            assert!(!l.contains('\\'), "an escape in a vector line: {l}");
            let verdict = field(l, "verdict").unwrap();
            let expect = (verdict == "decline")
                .then(|| field(l, "expect").expect("a decline names its tag"));
            (
                field(l, "name").unwrap(),
                field(l, "op_identity").unwrap(),
                expect,
            )
        })
        .collect()
}

#[test]
fn every_decidable_kiss_op_identity_vector_is_decided_as_kiss_decided_it() {
    let attrs = AttrTable::from_json(OPS).unwrap();
    let dtypes = DtypeTable::from_json(DTYPES).unwrap();
    let all = vectors();
    assert_eq!(all.len(), 43, "vector count in this vendored file");
    let (mut accepts, mut declines, mut value_typed) = (0, 0, Vec::new());
    for (name, line, expect) in &all {
        let got = parse_op_identity(line, &attrs, &dtypes);
        match expect.as_deref() {
            Some("op-identity-bad-value") => value_typed.push(name.as_str()),
            Some(tag) => {
                declines += 1;
                assert_eq!(
                    got.err().map(|e| e.wire_tag()),
                    Some(tag),
                    "{name}: {line:?}"
                );
            }
            None => {
                accepts += 1;
                got.unwrap_or_else(|e| panic!("{name}: {line:?} declined {e}"));
            }
        }
    }
    // controls: both verdicts and several tags are reached, so the loop is not vacuous
    assert_eq!((accepts, declines), (13, 18));
    assert_eq!(
        value_typed.len(),
        12,
        "the bad-value vectors this reader does not decide: {value_typed:?}"
    );
}

#[test]
fn the_bad_value_vectors_are_not_claimed_decided() {
    // A line that is structurally valid but value-invalid is ACCEPTED here (the module says so): the reader must not pretend to have checked it.
    let attrs = AttrTable::from_json(OPS).unwrap();
    let dtypes = DtypeTable::from_json(DTYPES).unwrap();
    let wrap = "gather{axis=0, oob_policy=wrap, index_operand=1, index_dtype=i32}";
    let ok = parse_op_identity(wrap, &attrs, &dtypes).expect("structurally valid");
    assert_eq!(
        ok.attrs[1],
        ("oob_policy", "wrap"),
        "the unchecked value text is handed back"
    );
    assert_eq!(ok.op, "gather");
    // and the vector for exactly this line expects a decline KISS decides from the vocabulary
    assert!(
        vectors()
            .iter()
            .any(|(_, l, e)| l == wrap && e.as_deref() == Some("op-identity-bad-value"))
    );
}

#[test]
fn the_attr_table_reads_the_ordered_fields() {
    use fireline_contract::op_identity::AttrSource;
    let t = AttrTable::from_json(OPS).unwrap();
    assert_eq!(t.len(), 128);
    let gather: Vec<&str> = t
        .attr_fields("gather")
        .unwrap()
        .iter()
        .map(String::as_str)
        .collect();
    assert_eq!(
        gather,
        ["axis", "oob_policy", "index_operand", "index_dtype"]
    );
    assert_eq!(t.attr_fields("add"), Some(&[][..]));
    assert_eq!(t.attr_fields("frobnicate"), None);
    assert!(AttrTable::from_json("{").is_err());
    assert!(AttrTable::from_json("{\"schema\":\"x\"}").is_err());
}
