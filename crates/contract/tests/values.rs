// SPDX-License-Identifier: MIT OR Apache-2.0
//! The typed Interface scalars (KISS-CONTRACT-6.5-0008, -0010, -0011, -0012) and the Dispatch expression check (6.11-0008): the golden document of
//! KISS `contract_vectors.json` (origin/main 73c341b5) passes, and every fault is written from the clause text (the corpus has one value per
//! field, so those cases are this crate's own). Draft-tracking: expect rework when the Draft moves.

use fireline_contract::document::{encode_document, parse_document};
use fireline_contract::fields::{Contract, parse_contract};
use fireline_contract::interface::Signature;
use fireline_contract::values::*;
use fireline_contract::{Context, ParseError};

const GOLDEN: &str = "4b495343206b6973732d636f6e74726163742034206c656e3d313238312063726333323d65313864393561640a5b73656374696f6e3a313a6964656e746974795d0a636f6e74726163745f6b696e64203d206b6973732d636f6e74726163740a636f6e74726163745f76657273696f6e203d20340a6b65726e656c5f6e616d65203d206164645f6633325f737472696465645f736d38390a7265766973696f6e5f68617368203d20343a64656164626565660a6163636570745f707265646963617465203d20736b377c6577327c6633327c637564613a736d38397c697833327c677269647c72327c636f2f30302f76342f6431362f663b73742f30302f76312f6431362f663b636f2f30302f76342f6431362f667c2d0a6f705f6964656e74697479203d206164640a7461726765745f6361706162696c697479203d20637564613a736d38390a5b73656374696f6e3a323a73656d616e746963735d0a73656d616e746963735f6b696e64203d206d616368696e652d636865636b61626c652d49520a6f705f646167203d205b4f707b6164643b203b205b5d7d5d0a5b73656374696f6e3a333a696e746572666163655d0a656e7472795f706f696e74203d206b0a72616e6b203d20320a706f736974696f6e616c5f7369676e6174757265203d205b5d0a6c61756e63685f7363616c617273203d205b5d0a636f756e745f756e6974203d20656c656d656e74730a696e5f706c616365203d2066616c73650a616c69676e6d656e745f6279746573203d20310a5b73656374696f6e3a343a64697370617463685d0a64697370617463685f6d6f64656c203d2067656f6d657472792d61676e6f737469630a5b73656374696f6e3a353a6361706162696c69746965735d0a6163636570745f707265646963617465203d20736b377c6577327c6633327c637564613a736d38397c697833327c677269647c72327c636f2f30302f76342f6431362f663b73742f30302f76312f6431362f663b636f2f30302f76342f6431362f667c2d0a737570706f727465645f64747970655f736574203d205b6633325d0a61776b776172645f6c61796f75745f7374726174656779203d206465636c696e650a696e5f706c6163655f656c696769626c655f76617269616e7473203d205b5d0a696e6465785f7769647468203d20697833320a64657465726d696e69736d5f636c617373203d2065786163742d627974650a707265636973696f6e5f636c617373203d207374726963740a636f7374203d20310a5b73656374696f6e3a363a67756172616e746565735d0a7265666572656e63655f66756e6374696f6e203d206164640a7065725f6261636b656e645f756c705f7469657273203d205b637564613a736d38397b6d61785f756c703d307d5d0a64657465726d696e69736d5f636c617373203d2065786163742d627974650a6d6174685f666964656c697479203d206269742d737461626c650a616363756d756c6174696f6e5f74797065203d206633320a6269745f73746162696c697479203d20706f727461626c650a617564697465645f737461747573203d20617564697465640a636f73745f70726f76656e616e6365203d206d656173757265640a5b73656374696f6e3a373a70726f76656e616e63655d0a6b65726e656c5f736f75726365203d20617574686f7265640a7265766973696f6e5f62617365203d2072300a7265766973696f6e5f68617368203d20343a64656164626565660a636f73745f70726f76656e616e6365203d206d656173757265640a6e65676f74696174696f6e5f6d65746164617461203d20303a0a";

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
        .collect()
}

/// The golden document with exact substrings replaced (each must occur once), re-framed with a fresh length and CRC-32.
fn edited(replacements: &[(&str, &str)]) -> Vec<u8> {
    let bytes = unhex(GOLDEN);
    let doc = parse_document(&bytes, &[4]).unwrap();
    let mut body = String::from_utf8(doc.body.to_vec()).unwrap();
    for (old, new) in replacements {
        assert_eq!(body.matches(old).count(), 1, "{old}");
        body = body.replace(old, new);
    }
    encode_document(body.as_bytes()).unwrap()
}

fn with<R>(bytes: &[u8], f: impl FnOnce(&Contract<'_>) -> R) -> R {
    let doc = parse_document(bytes, &[4]).expect("a framed document");
    f(&parse_contract(&doc).expect("field schemas hold"))
}

// The golden document's Interface scalars, and its empty signature matching its empty launch_scalars.
#[test]
fn the_golden_interface_values() {
    with(&unhex(GOLDEN), |c| {
        assert_eq!(
            interface_values(c),
            Ok(InterfaceValues {
                rank: 2,
                count_unit: CountUnit::Elements,
                alignment_bytes: 1,
                in_place: "false"
            })
        );
        assert_eq!(check_launch_scalars(c), Ok(()));
        assert_eq!(
            check_dispatch(c, &Context::new()),
            Ok(()),
            "geometry-agnostic: nothing to check"
        );
    });
}

// 6.5-0012: rank is a canonical non-negative decimal integer.
#[test]
fn rank_is_a_canonical_non_negative_integer() {
    for (text, ok) in [
        ("0", Some(0)),
        ("1", Some(1)),
        ("16", Some(16)),
        ("-1", None),
        ("02", None),
        ("+2", None),
        ("2.0", None),
        ("two", None),
        ("", None),
        (" 2", None),
    ] {
        let bytes = edited(&[("rank = 2\n", &format!("rank = {text}\n"))]);
        let got = with(&bytes, |c| interface_values(c).map(|v| v.rank).ok());
        assert_eq!(got, ok, "{text:?}");
    }
}

// 6.5-0008: count_unit is `elements` or `vectors_x<w>` with no delimiter and a canonical positive width.
#[test]
fn count_unit_is_elements_or_vectors_x_w() {
    for (text, ok) in [
        ("elements", Some(CountUnit::Elements)),
        ("vectors_x4", Some(CountUnit::Vectors(4))),
        ("vectors_x16", Some(CountUnit::Vectors(16))),
        ("vectors_x", None),
        ("vectors_x0", None),
        ("vectors_x04", None),
        ("vectors_x 4", None),
        ("vectors_x_4", None),
        ("vectors_x4.0", None),
        ("vectors4", None),
        ("Elements", None),
        ("element", None),
    ] {
        let bytes = edited(&[("count_unit = elements\n", &format!("count_unit = {text}\n"))]);
        assert_eq!(
            with(&bytes, |c| interface_values(c).map(|v| v.count_unit).ok()),
            ok,
            "{text:?}"
        );
    }
}

// 6.5-0010: alignment_bytes is a canonical positive integer; it may not be omitted (the schema declines that first).
#[test]
fn alignment_is_a_canonical_positive_integer() {
    for (text, ok) in [
        ("1", Some(1)),
        ("16", Some(16)),
        ("256", Some(256)),
        ("0", None),
        ("-1", None),
        ("08", None),
        ("", None),
        ("a", None),
    ] {
        let bytes = edited(&[(
            "alignment_bytes = 1\n",
            &format!("alignment_bytes = {text}\n"),
        )]);
        assert_eq!(
            with(&bytes, |c| interface_values(c)
                .map(|v| v.alignment_bytes)
                .ok()),
            ok,
            "{text:?}"
        );
    }
}

// 6.5-0009: in_place is returned as written: the Draft pins no spelling (the spec text says `none`, the golden document `false`).
#[test]
fn in_place_is_returned_verbatim() {
    for text in ["false", "none", "in0"] {
        let bytes = edited(&[("in_place = false\n", &format!("in_place = {text}\n"))]);
        assert_eq!(
            with(&bytes, |c| interface_values(c).unwrap().in_place.to_owned()),
            text
        );
    }
}

const SIG: &str = "[(ptr, 0, a, f32*, f32, scalar), (ptr, 0, out, f32*, f32, scalar), (ls, 1, extents0, i64, -, rank), (ls, 3, n, i32, -, scalar)]";
const TAIL: &str = "[(ls, 1, extents0, i64, -, rank), (ls, 3, n, i32, -, scalar)]";

// 6.5-0011: launch_scalars equals, entry for entry, the signature's launch-scalar tail (the entries after the operand pointers).
#[test]
fn launch_scalars_equal_the_signature_tail() {
    let sig_lines = |sig: &str, tail: &str| {
        edited(&[
            (
                "positional_signature = []\n",
                &format!("positional_signature = {sig}\n"),
            ),
            (
                "launch_scalars = []\n",
                &format!("launch_scalars = {tail}\n"),
            ),
        ])
    };
    assert_eq!(with(&sig_lines(SIG, TAIL), check_launch_scalars), Ok(()));
    // a different entry, a missing one, an extra one, a pointer in the list, an empty list against a non-empty tail
    for bad in [
        "[(ls, 1, extents0, i64, -, rank), (ls, 4, off0, i64, -, scalar)]",
        "[(ls, 1, extents0, i64, -, rank)]",
        "[(ls, 1, extents0, i64, -, rank), (ls, 3, n, i32, -, scalar), (ls, 4, off0, i64, -, scalar)]",
        "[(ptr, 0, a, f32*, f32, scalar), (ls, 1, extents0, i64, -, rank), (ls, 3, n, i32, -, scalar)]",
        "[]",
        "[(ls, 3, n, i64, -, scalar), (ls, 1, extents0, i64, -, rank)]",
    ] {
        let r = with(&sig_lines(SIG, bad), check_launch_scalars);
        assert!(
            matches!(
                r,
                Err(ValueError::LaunchScalarsMismatch | ValueError::Signature(_))
            ),
            "{bad}: {r:?}"
        );
    }
    // a non-empty list against an empty tail, and a list that does not parse
    assert_eq!(
        with(
            &sig_lines("[(ptr, 0, a, f32*, f32, scalar)]", TAIL),
            check_launch_scalars
        ),
        Err(ValueError::LaunchScalarsMismatch)
    );
    assert!(matches!(
        with(&sig_lines(SIG, "(oops)"), check_launch_scalars),
        Err(ValueError::Signature(_))
    ));
}

// 6.11-0008: each Dispatch geometry field is an expression over this contract's vocabulary; a symbol it does not declare or a malformed
// expression is a typed decline.
#[test]
fn dispatch_fields_are_expressions_over_the_contracts_vocabulary() {
    let geometry = |count_to_grid: &str| {
        edited(&[
            (
                "dispatch_model = geometry-agnostic\n",
                &format!(
                    "invocation_domain = n\nworkgroup_sizing = 256\ncount_to_grid = {count_to_grid}\nthread_mapping = 1\naddressing_rule = 0\n"
                ),
            ),
            (
                "positional_signature = []\n",
                &format!("positional_signature = {SIG}\n"),
            ),
            (
                "launch_scalars = []\n",
                &format!("launch_scalars = {TAIL}\n"),
            ),
        ])
    };
    let check = |bytes: &[u8]| {
        with(bytes, |c| {
            let sig = Signature::parse(c.get(2, "positional_signature").unwrap()).unwrap();
            check_dispatch(c, &Context::of_interface(2, &sig))
        })
    };
    assert_eq!(check(&geometry("ceil_div(n, 256)")), Ok(()));
    assert_eq!(
        check(&geometry("ceil_div(n, 256) + off0")),
        Err(ValueError::DispatchExpression {
            key: "count_to_grid".into(),
            error: ParseError::UnknownSymbol("off0".into())
        })
    );
    assert!(matches!(
        check(&geometry("n +")),
        Err(ValueError::DispatchExpression { .. })
    ));
    assert!(
        matches!(
            check(&geometry("ceil_div(n, 0)")),
            Err(ValueError::DispatchExpression { .. })
        ),
        "a constant zero divisor is a build-time fault"
    );
    assert!(
        matches!(
            check(&geometry("extents0[2]")),
            Err(ValueError::DispatchExpression { .. })
        ),
        "out of the declared rank"
    );
}
