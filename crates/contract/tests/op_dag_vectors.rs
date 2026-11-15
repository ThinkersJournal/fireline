// SPDX-License-Identifier: MIT OR Apache-2.0
//! KISS's `op_dag_vectors` (kiss origin/main 8c14914, KISS#828/#829, `conformance/corpus/contract_vectors.json`), vendored as
//! `tests/data/kiss_op_dag_vectors.tsv` (`kind<TAB>name<TAB>section<TAB>expect<TAB>doc_hex`; kind `A` accept, `D` decline). Every document is a
//! whole seven-section contract with one op_dag or identity fault; the reader must decline with the vector's `section` and `expect` tag, or
//! accept. Draft-tracking: re-paste when kiss moves.

use fireline_contract::document::parse_document;
use fireline_contract::dtypes::DtypeTable;
use fireline_contract::fields::parse_contract;
use fireline_contract::op_dag::check_semantics;
use fireline_contract::op_facts::FactTable;

const VECTORS: &str = include_str!("data/kiss_op_dag_vectors.tsv");
const OPS: &str = include_str!("data/kiss_op_facts.json");
const DTYPES: &str = include_str!("data/kiss_dtype_manifest.json");

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
        .collect()
}

#[test]
fn every_kiss_op_dag_vector_is_decided_as_kiss_decided_it() {
    let ops = FactTable::from_json(OPS).unwrap();
    let dtypes = DtypeTable::from_json(DTYPES).unwrap();
    let (mut accepts, mut declines) = (0, 0);
    for line in VECTORS.lines().map(|l| l.trim_end_matches('\r')) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        let [kind, name, section, expect, hex] = f[..] else {
            panic!("bad row {line:?}")
        };
        let bytes = unhex(hex);
        let doc = parse_document(&bytes, &[4]).unwrap_or_else(|e| panic!("{name}: framing {e:?}"));
        let contract = parse_contract(&doc).unwrap_or_else(|e| panic!("{name}: fields {e:?}"));
        let got = check_semantics(&contract, &ops, &dtypes);
        match kind {
            "A" => {
                accepts += 1;
                assert!(got.is_ok(), "{name}: {got:?}");
            }
            "D" => {
                declines += 1;
                let e = got.expect_err(name);
                assert_eq!(
                    (e.section(), e.wire_tag()),
                    (section, expect),
                    "{name}: {e:?}"
                );
            }
            _ => panic!("kind {kind:?}"),
        }
    }
    assert_eq!((accepts, declines), (10, 29));
}
