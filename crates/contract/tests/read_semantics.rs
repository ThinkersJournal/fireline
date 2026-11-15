// SPDX-License-Identifier: MIT OR Apache-2.0
//! `read_contract` runs the Semantics layer (6.4-0001, -0002, -0007, 6.3-0006): KISS's 39 `op_dag_vectors` (vendored in
//! `tests/data/kiss_op_dag_vectors.tsv`) go through the whole reader, not only `check_semantics`, so each decline arrives as
//! `ReadError::Semantics` with the vector's section and tag, and each accept passes every layer.

use fireline_contract::kiss_tables::KissTables;
use fireline_contract::read::ReadError;

const VECTORS: &str = include_str!("data/kiss_op_dag_vectors.tsv");

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
        .collect()
}

#[test]
fn the_whole_reader_decides_the_op_dag_vectors() {
    let tables = KissTables::from_json(
        include_str!("data/kiss_op_facts.json"),
        include_str!("data/kiss_dtype_manifest.json"),
    )
    .unwrap();
    let mut n = 0;
    for line in VECTORS.lines().map(|l| l.trim_end_matches('\r')) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        let [kind, name, section, expect, hex] = f[..] else {
            panic!("bad row {line:?}")
        };
        let bytes = unhex(hex);
        let got = tables.read_contract(&bytes, &[4]);
        n += 1;
        match kind {
            "A" => {
                let c = got.unwrap_or_else(|e| panic!("{name}: {e:?}"));
                assert!(
                    !c.op_dag.nodes.is_empty() || name.contains("no-node"),
                    "{name}"
                );
            }
            _ => match got {
                Err(ReadError::Semantics(e)) => {
                    assert_eq!((e.section(), e.wire_tag()), (section, expect), "{name}");
                }
                other => panic!("{name}: {other:?}"),
            },
        }
    }
    assert_eq!(n, 39);
}
