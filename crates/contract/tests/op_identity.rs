// SPDX-License-Identifier: MIT OR Apache-2.0
//! KISS's `op_identity_vectors` (kiss 50dc005, KISS#835, board 451; vendored as `tests/data/kiss_op_identity_vectors.json`) over the structure reader
//! and the value checker driven by KISS's `opattrs_field_types.json` (kiss 4ac8803, KISS#838; vendored as `tests/data/kiss_opattrs_field_types.json`).
//! All 43 vectors are decided as KISS decided them. Draft-tracking: re-vendor when kiss moves.

use fireline_contract::dtypes::DtypeTable;
use fireline_contract::op_identity::{
    AttrTable, FieldTypes, IdentityDecline, parse_op_identity, parse_op_identity_checked,
};

const VECTORS: &str = include_str!("data/kiss_op_identity_vectors.json");
const OPS: &str = include_str!("data/kiss_op_facts.json");
const DTYPES: &str = include_str!("data/kiss_dtype_manifest.json");
const TYPES: &str = include_str!("data/kiss_opattrs_field_types.json");

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
fn every_kiss_op_identity_vector_is_decided_as_kiss_decided_it() {
    let attrs = AttrTable::from_json(OPS).unwrap();
    let dtypes = DtypeTable::from_json(DTYPES).unwrap();
    let types = FieldTypes::from_json(TYPES).unwrap();
    let all = vectors();
    assert_eq!(all.len(), 43, "vector count in this vendored file");
    let (mut accepts, mut declines, mut bad_value) = (0, 0, 0);
    for (name, line, expect) in &all {
        let got = parse_op_identity_checked(line, &attrs, &dtypes, &types);
        match expect.as_deref() {
            Some(tag) => {
                declines += 1;
                bad_value += usize::from(tag == "op-identity-bad-value");
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
    // controls: both verdicts and every tag KISS uses are reached, so the loop is not vacuous
    assert_eq!((accepts, declines, bad_value), (13, 30, 12));
}

#[test]
fn the_structure_only_step_does_not_decide_values() {
    // parse_op_identity alone accepts a value-invalid line and hands the text back; only the checked entry point declines it.
    let attrs = AttrTable::from_json(OPS).unwrap();
    let dtypes = DtypeTable::from_json(DTYPES).unwrap();
    let types = FieldTypes::from_json(TYPES).unwrap();
    let wrap = "gather{axis=0, oob_policy=wrap, index_operand=1, index_dtype=i32}";
    let ok = parse_op_identity(wrap, &attrs, &dtypes).expect("structurally valid");
    assert_eq!(ok.attrs[1], ("oob_policy", "wrap"));
    assert_eq!(
        parse_op_identity_checked(wrap, &attrs, &dtypes, &types).map(|_| ()),
        Err(IdentityDecline::BadValue)
    );
    let clamp = wrap.replace("wrap", "clamp");
    assert!(parse_op_identity_checked(&clamp, &attrs, &dtypes, &types).is_ok());
}

#[test]
fn the_typed_table_agrees_with_the_ordered_opattrs_of_op_facts() {
    // Two KISS files carry the same field names; a skew between them would silently accept or refuse lines.
    use fireline_contract::op_identity::AttrSource;
    let attrs = AttrTable::from_json(OPS).unwrap();
    let types = FieldTypes::from_json(TYPES).unwrap();
    assert_eq!(types.len(), 31);
    let with_record = ops_with_a_record(&attrs);
    assert_eq!(
        with_record.len(),
        types.len(),
        "ops with a record in op_facts"
    );
    for op in with_record {
        let want: Vec<&str> = attrs
            .attr_fields(op)
            .unwrap()
            .iter()
            .map(String::as_str)
            .collect();
        assert_eq!(types.field_names(op), Some(want), "{op}");
    }
}

fn ops_with_a_record(attrs: &AttrTable) -> Vec<&'static str> {
    // op names come from the typed file's own keys, read as text: every key of `"ops": {` at four-space indent
    let mut v = Vec::new();
    for l in TYPES.lines() {
        if let Some(r) = l.strip_prefix("    \"")
            && let Some(n) = r.strip_suffix("\": [")
        {
            v.push(n);
        }
    }
    use fireline_contract::op_identity::AttrSource;
    assert!(
        v.iter()
            .all(|o| attrs.attr_fields(o).is_some_and(|f| !f.is_empty()))
    );
    v
}

fn check(line: &str) -> Result<(), IdentityDecline> {
    let attrs = AttrTable::from_json(OPS).unwrap();
    let dtypes = DtypeTable::from_json(DTYPES).unwrap();
    let types = FieldTypes::from_json(TYPES).unwrap();
    parse_op_identity_checked(line, &attrs, &dtypes, &types).map(|_| ())
}

#[test]
fn the_three_rules_kiss_states_in_prose_are_read_from_its_table() {
    // float-only accumulator (reduce), integer accumulator accepted for matmul, exactly one axis for prefix_scan
    let red = |acc: &str| {
        format!(
            "reduce{{monoid=sum, reduce_axes=[0], keepdim=true, accumulator={acc}, math_fidelity=bit-stable}}"
        )
    };
    assert_eq!(check(&red("i8")), Err(IdentityDecline::BadValue));
    assert_eq!(check(&red("i32")), Err(IdentityDecline::BadValue));
    assert_eq!(check(&red("f32")), Ok(()));
    let mm = |acc: &str| {
        format!(
            "matmul{{accumulator={acc}, math_fidelity=bit-stable, accuracy_kind=none, accuracy_value=0}}"
        )
    };
    assert_eq!(
        check(&mm("i32")),
        Ok(()),
        "matmul accepts an integer accumulator"
    );
    assert_eq!(check(&mm("i8")), Err(IdentityDecline::BadValue));
    let scan = |axes: &str| {
        format!(
            "prefix_scan{{monoid=sum, reduce_axes={axes}, exclusivity=inclusive, accumulator=compute-dtype, math_fidelity=bit-stable}}"
        )
    };
    assert_eq!(check(&scan("[3]")), Ok(()));
    assert_eq!(check(&scan("[0,1]")), Err(IdentityDecline::BadValue));
    // bounds: axis 7 is the last, 8 is past it; index_operand 15 is the last
    let g = |a: &str, o: &str| {
        format!("gather{{axis={a}, oob_policy=clamp, index_operand={o}, index_dtype=i32}}")
    };
    assert_eq!(check(&g("7", "15")), Ok(()));
    assert_eq!(check(&g("8", "1")), Err(IdentityDecline::BadValue));
    assert_eq!(check(&g("0", "16")), Err(IdentityDecline::BadValue));
    assert_eq!(check(&g("0", "0")), Ok(()), "a bare zero is canonical");
}

#[test]
fn the_reader_follows_the_table_it_is_given_not_a_built_in_one() {
    // Mutate the vendored table, each anchor asserted to apply once, and the decision must move with it.
    let attrs = AttrTable::from_json(OPS).unwrap();
    let dtypes = DtypeTable::from_json(DTYPES).unwrap();
    let text = TYPES.replace("\r\n", "\n");
    let wrap = "gather{axis=0, oob_policy=wrap, index_operand=1, index_dtype=i32}";
    let anchor = "\"vocab\": [\"skip\", \"clamp\", \"zero-fill\", \"reject\"]";
    assert!(text.matches(anchor).count() >= 1);
    let widened = text.replace(
        anchor,
        "\"vocab\": [\"skip\", \"clamp\", \"zero-fill\", \"reject\", \"wrap\"]",
    );
    assert_ne!(widened, text, "the mutation applied");
    let t2 = FieldTypes::from_json(&widened).unwrap();
    assert!(parse_op_identity_checked(wrap, &attrs, &dtypes, &t2).is_ok());
    let t1 = FieldTypes::from_json(&text).unwrap();
    assert_eq!(
        parse_op_identity_checked(wrap, &attrs, &dtypes, &t1).map(|_| ()),
        Err(IdentityDecline::BadValue)
    );
    // a bound read from rule text: raise gather's axis bound and axis=8 is accepted
    let a = "\"rule\": \"canonical decimal (no sign, no leading zero), 0 to 7\"";
    assert!(text.matches(a).count() >= 1);
    let t3 = FieldTypes::from_json(&text.replace(
        a,
        "\"rule\": \"canonical decimal (no sign, no leading zero), 0 to 9\"",
    ))
    .unwrap();
    let eight = "gather{axis=8, oob_policy=clamp, index_operand=1, index_dtype=i32}";
    assert!(parse_op_identity_checked(eight, &attrs, &dtypes, &t3).is_ok());
}

#[test]
fn an_unreadable_table_is_an_error_never_a_guess() {
    assert!(FieldTypes::from_json("{").is_err());
    assert!(FieldTypes::from_json("{\"schema\":\"x\"}").is_err());
    let text = TYPES.replace("\r\n", "\n");
    let one = "\"kind\": \"flag\"";
    assert!(text.matches(one).count() > 1);
    let unknown = text.replacen(one, "\"kind\": \"mystery\"", 1);
    assert!(FieldTypes::from_json(&unknown).is_err(), "an unknown kind");
    let rule = "\"rule\": \"canonical decimal (no sign, no leading zero), 0 to 7\"";
    assert!(text.matches(rule).count() >= 1);
    let boundless = text.replacen(rule, "\"rule\": \"canonical decimal\"", 1);
    assert!(
        FieldTypes::from_json(&boundless).is_err(),
        "a bound that cannot be read"
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
