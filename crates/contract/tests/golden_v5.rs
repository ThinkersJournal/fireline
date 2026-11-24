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

/// The golden with exact substrings replaced (each once), re-framed at version 5 with a fresh length and CRC-32: so the framing
/// accepts it and only the field parser can refuse it.
fn reframed(replacements: &[(&str, &str)]) -> Vec<u8> {
    let bytes = golden();
    let doc = parse_document(&bytes, &[5]).unwrap();
    let mut body = String::from_utf8(doc.body.to_vec()).unwrap();
    for (old, new) in replacements {
        assert_eq!(body.matches(old).count(), 1, "{old}");
        body = body.replace(old, new);
    }
    let mut out = format!(
        "KISC kiss-contract 5 len={} crc32={:08x}\n",
        body.len(),
        fireline_contract::document::crc32(body.as_bytes())
    )
    .into_bytes();
    out.extend_from_slice(body.as_bytes());
    out
}

fn field_error(bytes: &[u8]) -> fireline_contract::fields::FieldError {
    let doc = parse_document(bytes, &[5])
        .expect("the re-framed document passes the framing: the CRC is not what refuses it");
    parse_contract(&doc).expect_err("the field parser refuses it")
}

// A re-framed unedited golden is accepted (the control for the three mutants below).
#[test]
fn the_reframing_helper_alone_changes_nothing() {
    let bytes = reframed(&[("rank = 2\n", "rank = 2\n")]);
    assert_eq!(bytes, golden());
    let doc = parse_document(&bytes, &[5]).unwrap();
    assert!(parse_contract(&doc).is_ok());
}

#[test]
fn a_reframed_renamed_key_is_refused_by_the_field_parser() {
    use fireline_contract::fields::FieldError;
    let e = field_error(&reframed(&[("rank = 2\n", "rnk = 2\n")]));
    assert!(
        matches!(
            e,
            FieldError::UnexpectedField { section: 3, .. }
                | FieldError::MissingField {
                    section: 3,
                    key: "rank"
                }
        ),
        "{e:?}"
    );
}

#[test]
fn a_reframed_retired_v4_dispatch_field_is_refused_at_v5() {
    use fireline_contract::fields::FieldError;
    let e = field_error(&reframed(&[(
        "dispatch_model = geometry-agnostic\n",
        "dispatch_model = geometry-agnostic\nthread_mapping = d\n",
    )]));
    assert_eq!(e, FieldError::DispatchAlternatives);
}

// The value is read through its accessor (the schema accepts the field; 6.7-0007 allows only `true`/`false`).
#[test]
fn a_reframed_requires_co_resident_launch_must_be_a_boolean() {
    let bytes = reframed(&[(
        "cost = 1
",
        "cost = 1
requires_co_resident_launch = maybe
",
    )]);
    let doc = parse_document(&bytes, &[5]).expect("framing passes");
    let c = parse_contract(&doc).expect("the field is in the v5 schema");
    assert!(matches!(
        c.requires_co_resident_launch(),
        Err(fireline_contract::fields::FieldError::BadValue {
            key: "requires_co_resident_launch",
            ..
        })
    ));
    let ok = reframed(&[(
        "cost = 1
",
        "cost = 1
requires_co_resident_launch = true
",
    )]);
    let doc = parse_document(&ok, &[5]).unwrap();
    assert_eq!(
        parse_contract(&doc).unwrap().requires_co_resident_launch(),
        Ok(true)
    );
}
