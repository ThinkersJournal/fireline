// SPDX-License-Identifier: MIT OR Apache-2.0
//! The KISS `section_vectors` of `contract_vectors.json` (kiss origin/main 2c1e2cfa, KISS#818, 0.17.4): 6 accept documents and 49
//! single-fault declines, each with the section it declines in and kiss's stable kebab-case tag. This file byte-diffs the reader against
//! them: every accept document reads cleanly, and every decline is refused by the layer and variant the tag names.
//!
//! Tag mapping is this crate's (its error types are not kiss's tags): `duplicate-field` is [`FieldError::OutOfOrder`] here (a repeated key
//! is "out of order or repeated"), `partial-geometry` and `sentinel-with-geometry` are both [`FieldError::DispatchAlternatives`]. kiss
//! dropped its own extra `in-place-no-such-input` after our finding (6.5-0009 has no operand-count rule), so every decline is refused.
//! Draft-tracking: re-paste the data file when kiss moves.

use fireline_contract::ParseError;
use fireline_contract::fields::FieldError;
use fireline_contract::interface::SignatureError;
use fireline_contract::read::{ReadError, read_contract};
use fireline_contract::values::ValueError;

const DATA: &str = include_str!("data/kiss_section_vectors.tsv");

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
        .collect()
}

fn rows() -> Vec<Vec<&'static str>> {
    DATA.lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
        .map(|l| l.split('\t').collect())
        .collect()
}

/// Whether the reader's refusal is the one kiss's `tag` names.
fn matches_tag(tag: &str, e: &ReadError) -> bool {
    use ReadError::{Fields, Values};
    match tag {
        "missing-field" => matches!(e, Fields(FieldError::MissingField { .. })),
        "unknown-field" => matches!(e, Fields(FieldError::UnexpectedField { .. })),
        "out-of-order" | "duplicate-field" => matches!(e, Fields(FieldError::OutOfOrder { .. })),
        "not-utf8" => matches!(e, Fields(FieldError::NotUtf8 { .. })),
        "malformed-line" => matches!(e, Fields(FieldError::BadLine { .. })),
        "partial-geometry" | "sentinel-with-geometry" => {
            matches!(e, Fields(FieldError::DispatchAlternatives))
        }
        "bad-field-value" => matches!(e, Values(ValueError::InPlace(_))),
        "bad-descriptor" => matches!(
            e,
            Values(ValueError::Signature(SignatureError::BadDescriptor(_)))
        ),
        "array-len-kind-mismatch" => matches!(
            e,
            Values(ValueError::Signature(SignatureError::ClassNameMismatch(_)))
        ),
        "bad-expression" => matches!(
            e,
            Values(ValueError::DispatchExpression {
                error: ParseError::Syntax(_),
                ..
            })
        ),
        "unknown-symbol" => matches!(
            e,
            Values(ValueError::DispatchExpression {
                error: ParseError::UnknownSymbol(_),
                ..
            })
        ),
        "bare-array-symbol" => matches!(
            e,
            Values(ValueError::DispatchExpression {
                error: ParseError::Subscript(_),
                ..
            })
        ),
        _ => false,
    }
}

#[test]
fn the_accept_documents_read_cleanly() {
    let accepts: Vec<_> = rows().into_iter().filter(|r| r[0] == "A").collect();
    assert_eq!(accepts.len(), 6);
    for r in accepts {
        let bytes = unhex(r[4]);
        if let Err(e) = read_contract(&bytes, &[4]) {
            panic!("{}: {e:?}", r[1]);
        }
    }
}

#[test]
fn every_decline_is_refused_by_the_named_layer() {
    let declines: Vec<_> = rows().into_iter().filter(|r| r[0] == "D").collect();
    assert_eq!(declines.len(), 49);
    for r in declines {
        let (name, tag) = (r[1], r[3]);
        let bytes = unhex(r[4]);
        match read_contract(&bytes, &[4]) {
            Ok(_) => panic!("{name}: kiss expects {tag}, the reader accepted it"),
            Err(e) => assert!(matches_tag(tag, &e), "{name}: kiss {tag}, reader {e:?}"),
        }
    }
}
