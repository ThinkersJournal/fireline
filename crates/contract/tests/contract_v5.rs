// SPDX-License-Identifier: MIT OR Apache-2.0
//! Contract version 5 (KISS#843 head 139e925c, board 476): the Dispatch section drops `thread_mapping` and `addressing_rule`
//! (6.6-0001, 6.6-0004 retired), Capabilities may end with `requires_co_resident_launch` (6.7-0007), Guarantees may end with
//! `entry_exit_visibility` (6.8-0014). Versions 4 and 5 are both read: each is held to its own field schema, and a field of one
//! version is an unexpected field in the other. Draft-tracking.

use fireline_contract::document::{crc32, parse_document};
use fireline_contract::fields::*;

fn framed(version: u64, sections: [&str; 7]) -> Vec<u8> {
    let names = [
        "identity",
        "semantics",
        "interface",
        "dispatch",
        "capabilities",
        "guarantees",
        "provenance",
    ];
    let mut body = String::new();
    for (i, s) in sections.iter().enumerate() {
        body.push_str(&format!("[section:{}:{}]\n{s}", i + 1, names[i]));
    }
    let mut out = format!(
        "KISC kiss-contract {version} len={} crc32={:08x}\n",
        body.len(),
        crc32(body.as_bytes())
    )
    .into_bytes();
    out.extend_from_slice(body.as_bytes());
    out
}

fn parse(bytes: &[u8]) -> Result<Contract<'_>, FieldError> {
    parse_contract(&parse_document(bytes, &[4, 5]).expect("a framed document"))
}

const IDENTITY: &str = "contract_kind = kiss-contract\ncontract_version = 5\nkernel_name = k\nrevision_hash = 4:deadbeef\naccept_predicate = p\nop_identity = o\ntarget_capability = t\n";
const SEMANTICS: &str = "semantics_kind = declared-op-tag\nop_dag = []\n";
const INTERFACE: &str = "entry_point = k\nrank = 2\npositional_signature = []\nlaunch_scalars = []\ncount_unit = elements\nin_place = none\nalignment_bytes = 1\n";
const GEOMETRY3: &str = "invocation_domain = a\nworkgroup_sizing = b\ncount_to_grid = c\n";
const GEOMETRY5: &str = "invocation_domain = a\nworkgroup_sizing = b\ncount_to_grid = c\nthread_mapping = d\naddressing_rule = e\n";
const CAPABILITIES: &str = "accept_predicate = p\nsupported_dtype_set = [f32]\nawkward_layout_strategy = decline\nin_place_eligible_variants = []\nindex_width = ix32\ndeterminism_class = exact-byte\nprecision_class = strict\ncost = 1\n";
const GUARANTEES: &str = "reference_function = add\nper_backend_ulp_tiers = []\ndeterminism_class = exact-byte\nmath_fidelity = bit-stable\naccumulation_type = f32\nbit_stability = portable\naudited_status = audited\ncost_provenance = measured\n";
const PROVENANCE: &str = "kernel_source = authored\nrevision_base = r0\nrevision_hash = 4:deadbeef\ncost_provenance = measured\nnegotiation_metadata = 0:\n";

fn sections<'a>(dispatch: &'a str, caps: &'a str, guar: &'a str) -> [&'a str; 7] {
    [
        IDENTITY, SEMANTICS, INTERFACE, dispatch, caps, guar, PROVENANCE,
    ]
}

// 6.6-0001 at v5: three geometry fields, or the sentinel.
#[test]
fn v5_dispatch_is_three_fields_and_v4_is_five() {
    let ok = framed(5, sections(GEOMETRY3, CAPABILITIES, GUARANTEES));
    assert!(parse(&ok).is_ok());
    let sentinel = framed(
        5,
        sections(
            "dispatch_model = geometry-agnostic\n",
            CAPABILITIES,
            GUARANTEES,
        ),
    );
    assert!(parse(&sentinel).is_ok());
    // the retired fields are not accepted at v5
    let five = framed(5, sections(GEOMETRY5, CAPABILITIES, GUARANTEES));
    assert_eq!(parse(&five).unwrap_err(), FieldError::DispatchAlternatives);
    // and three fields are not enough at v4
    let three4 = framed(4, sections(GEOMETRY3, CAPABILITIES, GUARANTEES));
    assert_eq!(
        parse(&three4).unwrap_err(),
        FieldError::DispatchAlternatives
    );
    let five4 = framed(4, sections(GEOMETRY5, CAPABILITIES, GUARANTEES));
    assert!(parse(&five4).is_ok());
}

// 6.7-0007: the optional trailing field, absent means false; any other value is a typed decline.
#[test]
fn requires_co_resident_launch_is_optional_last_and_boolean() {
    let with = |v: &str| format!("{CAPABILITIES}requires_co_resident_launch = {v}\n");
    let read = |c: &str| {
        let b = framed(5, sections(GEOMETRY3, c, GUARANTEES));
        parse(&b).and_then(|k| k.requires_co_resident_launch())
    };
    assert_eq!(read(CAPABILITIES), Ok(false));
    assert_eq!(read(&with("false")), Ok(false));
    assert_eq!(read(&with("true")), Ok(true));
    for bad in ["True", "1", "yes", "", "true "] {
        assert_eq!(
            read(&with(bad)),
            Err(FieldError::BadValue {
                section: 5,
                key: "requires_co_resident_launch"
            }),
            "value {bad:?}"
        );
    }
    // pinned last: a field after it, or it before `cost`, is a schema fault
    let b = framed(
        5,
        sections(
            GEOMETRY3,
            &format!(
                "{}requires_co_resident_launch = true\ncost = 1\n",
                CAPABILITIES.replace("cost = 1\n", "")
            ),
            GUARANTEES,
        ),
    );
    assert!(parse(&b).is_err());
    // not a v4 field
    let b = framed(4, sections(GEOMETRY5, &with("true"), GUARANTEES));
    assert!(matches!(
        parse(&b),
        Err(FieldError::UnexpectedField { section: 5, .. })
    ));
}

// 6.8-0014: entry_exit_visibility, absent means caller-ordered.
#[test]
fn entry_exit_visibility_is_optional_last_and_two_tokens() {
    let with = |v: &str| format!("{GUARANTEES}entry_exit_visibility = {v}\n");
    let read = |g: &str| {
        let b = framed(5, sections(GEOMETRY3, CAPABILITIES, g));
        parse(&b).and_then(|k| k.entry_exit_visibility())
    };
    assert_eq!(read(GUARANTEES), Ok(Visibility::CallerOrdered));
    assert_eq!(read(&with("caller-ordered")), Ok(Visibility::CallerOrdered));
    assert_eq!(
        read(&with("visible-at-exit")),
        Ok(Visibility::VisibleAtExit)
    );
    assert_eq!(
        read(&with("visible")),
        Err(FieldError::BadValue {
            section: 6,
            key: "entry_exit_visibility"
        })
    );
    let b = framed(
        4,
        sections(GEOMETRY5, CAPABILITIES, &with("caller-ordered")),
    );
    assert!(matches!(
        parse(&b),
        Err(FieldError::UnexpectedField { section: 6, .. })
    ));
}
