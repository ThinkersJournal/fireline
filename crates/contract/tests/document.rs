// SPDX-License-Identifier: MIT OR Apache-2.0
//! The KISS-Contract document framing (§6.11): the golden document and the single-fault declines of KISS
//! `conformance/corpus/contract_vectors.json` (KISS origin/main 73c341b5), byte-diffed, plus the framing clauses' own cases.
//! Draft-tracking: expect rework when the Draft moves.

use fireline_contract::document::*;

const GOLDEN: &str = "4b495343206b6973732d636f6e74726163742034206c656e3d313238312063726333323d65313864393561640a5b73656374696f6e3a313a6964656e746974795d0a636f6e74726163745f6b696e64203d206b6973732d636f6e74726163740a636f6e74726163745f76657273696f6e203d20340a6b65726e656c5f6e616d65203d206164645f6633325f737472696465645f736d38390a7265766973696f6e5f68617368203d20343a64656164626565660a6163636570745f707265646963617465203d20736b377c6577327c6633327c637564613a736d38397c697833327c677269647c72327c636f2f30302f76342f6431362f663b73742f30302f76312f6431362f663b636f2f30302f76342f6431362f667c2d0a6f705f6964656e74697479203d206164640a7461726765745f6361706162696c697479203d20637564613a736d38390a5b73656374696f6e3a323a73656d616e746963735d0a73656d616e746963735f6b696e64203d206d616368696e652d636865636b61626c652d49520a6f705f646167203d205b4f707b6164643b203b205b5d7d5d0a5b73656374696f6e3a333a696e746572666163655d0a656e7472795f706f696e74203d206b0a72616e6b203d20320a706f736974696f6e616c5f7369676e6174757265203d205b5d0a6c61756e63685f7363616c617273203d205b5d0a636f756e745f756e6974203d20656c656d656e74730a696e5f706c616365203d2066616c73650a616c69676e6d656e745f6279746573203d20310a5b73656374696f6e3a343a64697370617463685d0a64697370617463685f6d6f64656c203d2067656f6d657472792d61676e6f737469630a5b73656374696f6e3a353a6361706162696c69746965735d0a6163636570745f707265646963617465203d20736b377c6577327c6633327c637564613a736d38397c697833327c677269647c72327c636f2f30302f76342f6431362f663b73742f30302f76312f6431362f663b636f2f30302f76342f6431362f667c2d0a737570706f727465645f64747970655f736574203d205b6633325d0a61776b776172645f6c61796f75745f7374726174656779203d206465636c696e650a696e5f706c6163655f656c696769626c655f76617269616e7473203d205b5d0a696e6465785f7769647468203d20697833320a64657465726d696e69736d5f636c617373203d2065786163742d627974650a707265636973696f6e5f636c617373203d207374726963740a636f7374203d20310a5b73656374696f6e3a363a67756172616e746565735d0a7265666572656e63655f66756e6374696f6e203d206164640a7065725f6261636b656e645f756c705f7469657273203d205b637564613a736d38397b6d61785f756c703d307d5d0a64657465726d696e69736d5f636c617373203d2065786163742d627974650a6d6174685f666964656c697479203d206269742d737461626c650a616363756d756c6174696f6e5f74797065203d206633320a6269745f73746162696c697479203d20706f727461626c650a617564697465645f737461747573203d20617564697465640a636f73745f70726f76656e616e6365203d206d656173757265640a5b73656374696f6e3a373a70726f76656e616e63655d0a6b65726e656c5f736f75726365203d20617574686f7265640a7265766973696f6e5f62617365203d2072300a7265766973696f6e5f68617368203d20343a64656164626565660a636f73745f70726f76656e616e6365203d206d656173757265640a6e65676f74696174696f6e5f6d65746164617461203d20303a0a";

/// (name, KISS `expect` tag, document hex), verbatim from the corpus.
const DECLINES: &[(&str, &str, &str)] = &[
    (
        "magic replaced",
        "no-magic",
        "58585858206b6973732d636f6e74726163742034206c656e3d37322063726333323d38396261633730330a5b73656374696f6e3a313a6964656e746974795d0a636f6e74726163745f6b696e64203d206b6973732d636f6e74726163740a636f6e74726163745f76657273696f6e203d20340a",
    ),
    ("input shorter than the magic", "no-magic", "4b4953"),
    (
        "header line never terminated (no LF anywhere)",
        "malformed-header",
        "4b495343206b6973732d636f6e74726163742034206c656e3d37322063726333323d3839626163373033",
    ),
    (
        "CRLF header (LF-only is pinned)",
        "malformed-header",
        "4b495343206b6973732d636f6e74726163742034206c656e3d37322063726333323d38396261633730330d0a5b73656374696f6e3a313a6964656e746974795d0a636f6e74726163745f6b696e64203d206b6973732d636f6e74726163740a636f6e74726163745f76657273696f6e203d20340a",
    ),
    (
        "header field dropped",
        "malformed-header",
        "4b495343206b6973732d636f6e74726163742034206c656e3d37320a5b73656374696f6e3a313a6964656e746974795d0a636f6e74726163745f6b696e64203d206b6973732d636f6e74726163740a636f6e74726163745f76657273696f6e203d20340a",
    ),
    (
        "len= is not decimal",
        "malformed-header",
        "4b495343206b6973732d636f6e74726163742034206c656e3d6162632063726333323d38396261633730330a5b73656374696f6e3a313a6964656e746974795d0a636f6e74726163745f6b696e64203d206b6973732d636f6e74726163740a636f6e74726163745f76657273696f6e203d20340a",
    ),
    (
        "crc32= is uppercase hex",
        "malformed-header",
        "4b495343206b6973732d636f6e74726163742034206c656e3d37322063726333323d44454144424545460a5b73656374696f6e3a313a6964656e746974795d0a636f6e74726163745f6b696e64203d206b6973732d636f6e74726163740a636f6e74726163745f76657273696f6e203d20340a",
    ),
    (
        "unrecognized contract_kind",
        "unknown-kind",
        "4b495343206b6973732d6b6f6e74726163742034206c656e3d37322063726333323d38396261633730330a5b73656374696f6e3a313a6964656e746974795d0a636f6e74726163745f6b696e64203d206b6973732d636f6e74726163740a636f6e74726163745f76657273696f6e203d20340a",
    ),
    (
        "unsupported contract_version",
        "unknown-version",
        "4b495343206b6973732d636f6e74726163742032206c656e3d37322063726333323d38396261633730330a5b73656374696f6e3a313a6964656e746974795d0a636f6e74726163745f6b696e64203d206b6973732d636f6e74726163740a636f6e74726163745f76657273696f6e203d20340a",
    ),
    (
        "declared length overstates the body",
        "bad-length",
        "4b495343206b6973732d636f6e74726163742034206c656e3d37332063726333323d38396261633730330a5b73656374696f6e3a313a6964656e746974795d0a636f6e74726163745f6b696e64203d206b6973732d636f6e74726163740a636f6e74726163745f76657273696f6e203d20340a",
    ),
    (
        "declared length is hostile (u64 near-max)",
        "bad-length",
        "4b495343206b6973732d636f6e74726163742034206c656e3d31383434363734343037333730393535313631352063726333323d38396261633730330a5b73656374696f6e3a313a6964656e746974795d0a636f6e74726163745f6b696e64203d206b6973732d636f6e74726163740a636f6e74726163745f76657273696f6e203d20340a",
    ),
    (
        "body byte flipped, declared length still correct",
        "bad-checksum",
        "4b495343206b6973732d636f6e74726163742034206c656e3d37322063726333323d38396261633730330a7b73656374696f6e3a313a6964656e746974795d0a636f6e74726163745f6b696e64203d206b6973732d636f6e74726163740a636f6e74726163745f76657273696f6e203d20340a",
    ),
    (
        "headingless body (framing valid, no first section heading)",
        "headingless",
        "4b495343206b6973732d636f6e74726163742034206c656e3d33302063726333323d66353364623161340a636f6e74726163745f6b696e64203d206b6973732d636f6e74726163740a",
    ),
];

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex byte"))
        .collect()
}

fn framed(version: u64, body: &[u8]) -> Vec<u8> {
    let mut out = format!(
        "KISC kiss-contract {version} len={} crc32={:08x}\n",
        body.len(),
        crc32(body)
    )
    .into_bytes();
    out.extend_from_slice(body);
    out
}

fn sections_body(names: &[&str]) -> Vec<u8> {
    let mut b = Vec::new();
    for (i, n) in names.iter().enumerate() {
        b.extend_from_slice(format!("[section:{}:{n}]\nk = v\n", i + 1).as_bytes());
    }
    b
}

// CRC-32 check value of the IEEE 802.3 polynomial (6.11-0003).
#[test]
fn crc32_check_value() {
    assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    assert_eq!(crc32(b""), 0);
}

// KISS-CONTRACT-6.11-0002/-0003/-0004: the golden document parses, and re-framing its body gives the same bytes.
#[test]
fn the_golden_document_parses_and_reframes_byte_for_byte() {
    let bytes = unhex(GOLDEN);
    let doc = parse_document(&bytes, &[4]).expect("the golden document");
    assert_eq!(doc.contract_version, CONTRACT_VERSION);
    assert_eq!(encode_document(doc.body).unwrap(), bytes);
    // seven sections, each holding its own field lines
    for (i, s) in doc.sections.iter().enumerate() {
        assert!(!s.is_empty(), "section {} is empty", i + 1);
    }
    assert!(doc.sections[0].starts_with(b"contract_kind = kiss-contract\n"));
}

// KISS-CONTRACT-6.1-0002/-0003/-0004, 6.11-0002/-0003/-0004: every single-fault vector declines in its KISS category.
#[test]
fn every_kiss_decline_vector_declines_with_its_category() {
    for (name, expect, hex) in DECLINES {
        let err = parse_document(&unhex(hex), &[4]).expect_err(name);
        assert_eq!(err.wire_tag(), *expect, "{name}: {err}");
    }
}

// KISS-CONTRACT-6.1-0004: no prefix of a document panics, and no proper prefix is accepted.
#[test]
fn every_proper_prefix_declines() {
    let bytes = unhex(GOLDEN);
    for n in 0..bytes.len() {
        assert!(parse_document(&bytes[..n], &[4]).is_err(), "prefix {n}");
    }
}

// KISS-CONTRACT-6.11-0003: a body longer than `N` (here, a byte after the document) is not exactly `N` bytes.
#[test]
fn trailing_bytes_are_a_bad_length() {
    let mut bytes = unhex(GOLDEN);
    bytes.push(b'\n');
    assert!(matches!(
        parse_document(&bytes, &[4]),
        Err(DocumentError::BadLength { .. })
    ));
}

// KISS-CONTRACT-6.11-0003a: a length over 2^24 declines before any length or checksum check; 2^24 itself is a length question.
#[test]
fn the_length_cap_is_checked_first() {
    let over = format!(
        "KISC kiss-contract 4 len={} crc32=00000000\nx",
        MAX_DOCUMENT_BODY_LEN + 1
    );
    assert_eq!(
        parse_document(over.as_bytes(), &[4]),
        Err(DocumentError::LengthOverCap)
    );
    let at = format!(
        "KISC kiss-contract 4 len={} crc32=00000000\nx",
        MAX_DOCUMENT_BODY_LEN
    );
    assert!(matches!(
        parse_document(at.as_bytes(), &[4]),
        Err(DocumentError::BadLength { .. })
    ));
    // a digit string too large for u64 is over the cap too, never a parse failure that panics
    let huge = "KISC kiss-contract 4 len=99999999999999999999999999 crc32=00000000\nx";
    assert_eq!(
        parse_document(huge.as_bytes(), &[4]),
        Err(DocumentError::LengthOverCap)
    );
    assert_eq!(
        encode_document(&vec![0; MAX_DOCUMENT_BODY_LEN as usize + 1]),
        Err(DocumentError::LengthOverCap)
    );
}

// KISS-CONTRACT-6.1-0008: support is an enumerated set, never an ordering: a lower or higher version outside it declines.
#[test]
fn versions_are_a_declared_set() {
    let body = sections_body(&SECTION_NAMES);
    for v in [2u64, 5] {
        assert_eq!(
            parse_document(&framed(v, &body), &[4]),
            Err(DocumentError::UnknownVersion(v))
        );
    }
    // a reader that declares {2, 4} accepts both, and a 3 between them is still refused
    assert!(parse_document(&framed(2, &body), &[2, 4]).is_ok());
    assert!(parse_document(&framed(4, &body), &[2, 4]).is_ok());
    assert_eq!(
        parse_document(&framed(3, &body), &[2, 4]),
        Err(DocumentError::UnknownVersion(3))
    );
    // an empty supported set accepts nothing
    assert_eq!(
        parse_document(&framed(4, &body), &[]),
        Err(DocumentError::UnknownVersion(4))
    );
}

// KISS-CONTRACT-6.11-0004: the seven headings, once each, in order, with no content before the first.
#[test]
fn section_headings_are_exactly_the_pinned_seven() {
    let ok = sections_body(&SECTION_NAMES);
    assert!(parse_document(&framed(4, &ok), &[4]).is_ok());

    let mut swapped = SECTION_NAMES;
    swapped.swap(1, 2);
    let mut dup = SECTION_NAMES;
    dup[6] = "guarantees";
    let mut typo = SECTION_NAMES;
    typo[3] = "dispatsh";
    for names in [
        swapped.to_vec(),
        dup.to_vec(),
        typo.to_vec(),
        SECTION_NAMES[..6].to_vec(),
    ] {
        assert_eq!(
            parse_document(&framed(4, &sections_body(&names)), &[4]),
            Err(DocumentError::SectionHeadings),
            "{names:?}"
        );
    }
    // an eighth heading is refused too
    let mut eight = ok.clone();
    eight.extend_from_slice(b"[section:8:extra]\n");
    assert_eq!(
        parse_document(&framed(4, &eight), &[4]),
        Err(DocumentError::SectionHeadings)
    );
    // content before the first heading, and an empty body, are headingless
    let mut lead = b"stray = line\n".to_vec();
    lead.extend_from_slice(&ok);
    assert_eq!(
        parse_document(&framed(4, &lead), &[4]),
        Err(DocumentError::Headingless)
    );
    assert_eq!(
        parse_document(&framed(4, b""), &[4]),
        Err(DocumentError::Headingless)
    );
}

// KISS-CONTRACT-6.11-0003b: a matching crc32 is not evidence that the body is unmodified: whoever edits the body recomputes it.
#[test]
fn a_recomputed_crc_is_accepted_because_it_is_not_a_signature() {
    let bytes = unhex(GOLDEN);
    let doc = parse_document(&bytes, &[4]).unwrap();
    let mut body = doc.body.to_vec();
    let at = body.iter().position(|&b| b == b'=').unwrap();
    body[at + 2] ^= 0x01;
    assert!(parse_document(&framed(4, &body), &[4]).is_ok());
}

// KISS-CONTRACT-6.11-0002: the header line is exact: a second space, a tab or an upper-case kind is a malformed header, not repaired.
#[test]
fn the_header_line_is_not_repaired() {
    let body = sections_body(&SECTION_NAMES);
    let good = String::from_utf8(framed(4, &body)).unwrap();
    for bad in [
        good.replacen("KISC kiss", "KISC  kiss", 1),
        good.replacen("KISC kiss", "KISC\tkiss", 1),
        good.replacen("len=", "LEN=", 1),
        good.replacen(" crc32=", "  crc32=", 1),
        good.replacen("4 len", "+4 len", 1),
    ] {
        assert_eq!(
            parse_document(bad.as_bytes(), &[4]),
            Err(DocumentError::MalformedHeader),
            "{}",
            bad.lines().next().unwrap_or("")
        );
    }
}
