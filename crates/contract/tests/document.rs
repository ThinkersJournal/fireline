// SPDX-License-Identifier: MIT OR Apache-2.0
//! The KISS-Contract document framing (§6.11): the golden document and the single-fault declines of KISS
//! `conformance/corpus/contract_vectors.json` (KISS origin/main 73c341b5), byte-diffed, plus the framing clauses' own cases.
//! Draft-tracking: expect rework when the Draft moves.

use fireline_contract::document::*;

const GOLDEN: &str = "4B495343206B6973732D636F6E74726163742034206C656E3D313238302063726333323D37396635303365340A5B73656374696F6E3A313A6964656E746974795D0A636F6E74726163745F6B696E64203D206B6973732D636F6E74726163740A636F6E74726163745F76657273696F6E203D20340A6B65726E656C5F6E616D65203D206164645F6633325F737472696465645F736D38390A7265766973696F6E5F68617368203D20343A64656164626565660A6163636570745F707265646963617465203D20736B377C6577327C6633327C637564613A736D38397C697833327C677269647C72327C636F2F30302F76342F6431362F663B73742F30302F76312F6431362F663B636F2F30302F76342F6431362F667C2D0A6F705F6964656E74697479203D206164640A7461726765745F6361706162696C697479203D20637564613A736D38390A5B73656374696F6E3A323A73656D616E746963735D0A73656D616E746963735F6B696E64203D206D616368696E652D636865636B61626C652D49520A6F705F646167203D205B4F707B6164643B203B205B5D7D5D0A5B73656374696F6E3A333A696E746572666163655D0A656E7472795F706F696E74203D206B0A72616E6B203D20320A706F736974696F6E616C5F7369676E6174757265203D205B5D0A6C61756E63685F7363616C617273203D205B5D0A636F756E745F756E6974203D20656C656D656E74730A696E5F706C616365203D206E6F6E650A616C69676E6D656E745F6279746573203D20310A5B73656374696F6E3A343A64697370617463685D0A64697370617463685F6D6F64656C203D2067656F6D657472792D61676E6F737469630A5B73656374696F6E3A353A6361706162696C69746965735D0A6163636570745F707265646963617465203D20736B377C6577327C6633327C637564613A736D38397C697833327C677269647C72327C636F2F30302F76342F6431362F663B73742F30302F76312F6431362F663B636F2F30302F76342F6431362F667C2D0A737570706F727465645F64747970655F736574203D205B6633325D0A61776B776172645F6C61796F75745F7374726174656779203D206465636C696E650A696E5F706C6163655F656C696769626C655F76617269616E7473203D205B5D0A696E6465785F7769647468203D20697833320A64657465726D696E69736D5F636C617373203D2065786163742D627974650A707265636973696F6E5F636C617373203D207374726963740A636F7374203D20310A5B73656374696F6E3A363A67756172616E746565735D0A7265666572656E63655F66756E6374696F6E203D206164640A7065725F6261636B656E645F756C705F7469657273203D205B637564613A736D38397B6D61785F756C703D307D5D0A64657465726D696E69736D5F636C617373203D2065786163742D627974650A6D6174685F666964656C697479203D206269742D737461626C650A616363756D756C6174696F6E5F74797065203D206633320A6269745F73746162696C697479203D20706F727461626C650A617564697465645F737461747573203D20617564697465640A636F73745F70726F76656E616E6365203D206D656173757265640A5B73656374696F6E3A373A70726F76656E616E63655D0A6B65726E656C5F736F75726365203D20617574686F7265640A7265766973696F6E5F62617365203D2072300A7265766973696F6E5F68617368203D20343A64656164626565660A636F73745F70726F76656E616E6365203D206D656173757265640A6E65676F74696174696F6E5F6D65746164617461203D20303A0A";

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
