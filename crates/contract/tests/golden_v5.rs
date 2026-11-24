// SPDX-License-Identifier: MIT OR Apache-2.0
//! The v5 golden document of KISS `conformance/corpus/contract_vectors.json` (KISS main be637a51, #843 squash): framed, parsed to its
//! fields at contract_version 5, and consistent. The hex is copied from the corpus' `bytes_hex` (space separated, 1349 bytes).

use fireline_contract::consistency::check_consistency;
use fireline_contract::document::parse_document;
use fireline_contract::fields::parse_contract;

fn golden() -> Vec<u8> {
    let hex: String = include_str!("data/kiss_golden_v5.hex")
        .split_whitespace()
        .collect();
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("hex"))
        .collect()
}

#[test]
fn the_v5_golden_document_is_the_corpus_length() {
    assert_eq!(golden().len(), 1349);
}

#[test]
fn the_v5_golden_document_parses_at_version_5() {
    let bytes = golden();
    let doc = parse_document(&bytes, &[5]).expect("framed at v5");
    let c = parse_contract(&doc).expect("v5 fields");
    assert_eq!(c.contract_version, 5);
}

#[test]
fn the_v5_golden_document_is_consistent() {
    let bytes = golden();
    let doc = parse_document(&bytes, &[5]).expect("framed at v5");
    let c = parse_contract(&doc).expect("v5 fields");
    assert_eq!(check_consistency(&c), Ok(()));
}

#[test]
fn the_v5_golden_document_is_not_a_version_4_document() {
    assert!(parse_document(&golden(), &[4]).is_err());
}
