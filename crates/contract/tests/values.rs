// SPDX-License-Identifier: MIT OR Apache-2.0
//! The typed Interface scalars (KISS-CONTRACT-6.5-0008, -0010, -0011, -0012) and the Dispatch expression check (6.11-0008): the golden document of
//! KISS `contract_vectors.json` (origin/main 73c341b5) passes, and every fault is written from the clause text (the corpus has one value per
//! field, so those cases are this crate's own). Draft-tracking: expect rework when the Draft moves.

use fireline_contract::document::{encode_document, parse_document};
use fireline_contract::fields::{Contract, parse_contract};
use fireline_contract::interface::Signature;
use fireline_contract::values::*;
use fireline_contract::{Context, ParseError};

const GOLDEN: &str = "4B495343206B6973732D636F6E74726163742034206C656E3D313238302063726333323D37396635303365340A5B73656374696F6E3A313A6964656E746974795D0A636F6E74726163745F6B696E64203D206B6973732D636F6E74726163740A636F6E74726163745F76657273696F6E203D20340A6B65726E656C5F6E616D65203D206164645F6633325F737472696465645F736D38390A7265766973696F6E5F68617368203D20343A64656164626565660A6163636570745F707265646963617465203D20736B377C6577327C6633327C637564613A736D38397C697833327C677269647C72327C636F2F30302F76342F6431362F663B73742F30302F76312F6431362F663B636F2F30302F76342F6431362F667C2D0A6F705F6964656E74697479203D206164640A7461726765745F6361706162696C697479203D20637564613A736D38390A5B73656374696F6E3A323A73656D616E746963735D0A73656D616E746963735F6B696E64203D206D616368696E652D636865636B61626C652D49520A6F705F646167203D205B4F707B6164643B203B205B5D7D5D0A5B73656374696F6E3A333A696E746572666163655D0A656E7472795F706F696E74203D206B0A72616E6B203D20320A706F736974696F6E616C5F7369676E6174757265203D205B5D0A6C61756E63685F7363616C617273203D205B5D0A636F756E745F756E6974203D20656C656D656E74730A696E5F706C616365203D206E6F6E650A616C69676E6D656E745F6279746573203D20310A5B73656374696F6E3A343A64697370617463685D0A64697370617463685F6D6F64656C203D2067656F6D657472792D61676E6F737469630A5B73656374696F6E3A353A6361706162696C69746965735D0A6163636570745F707265646963617465203D20736B377C6577327C6633327C637564613A736D38397C697833327C677269647C72327C636F2F30302F76342F6431362F663B73742F30302F76312F6431362F663B636F2F30302F76342F6431362F667C2D0A737570706F727465645F64747970655F736574203D205B6633325D0A61776B776172645F6C61796F75745F7374726174656779203D206465636C696E650A696E5F706C6163655F656C696769626C655F76617269616E7473203D205B5D0A696E6465785F7769647468203D20697833320A64657465726D696E69736D5F636C617373203D2065786163742D627974650A707265636973696F6E5F636C617373203D207374726963740A636F7374203D20310A5B73656374696F6E3A363A67756172616E746565735D0A7265666572656E63655F66756E6374696F6E203D206164640A7065725F6261636B656E645F756C705F7469657273203D205B637564613A736D38397B6D61785F756C703D307D5D0A64657465726D696E69736D5F636C617373203D2065786163742D627974650A6D6174685F666964656C697479203D206269742D737461626C650A616363756D756C6174696F6E5F74797065203D206633320A6269745F73746162696C697479203D20706F727461626C650A617564697465645F737461747573203D20617564697465640A636F73745F70726F76656E616E6365203D206D656173757265640A5B73656374696F6E3A373A70726F76656E616E63655D0A6B65726E656C5F736F75726365203D20617574686F7265640A7265766973696F6E5F62617365203D2072300A7265766973696F6E5F68617368203D20343A64656164626565660A636F73745F70726F76656E616E6365203D206D656173757265640A6E65676F74696174696F6E5F6D65746164617461203D20303A0A";

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
                in_place: InPlace::None
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

// 6.5-0009: in_place is exactly `none` or `input:<k>` (k canonical decimal, 0-based input index); anything else, `false` included, declines.
// KISS#813 pinned the spelling; the golden document says `none`, the `input:<k>` and decline cases are this crate's own.
#[test]
fn in_place_is_none_or_input_k() {
    let cases: [(&str, Option<InPlace>); 14] = [
        ("none", Some(InPlace::None)),
        ("input:0", Some(InPlace::Input(0))),
        ("input:7", Some(InPlace::Input(7))),
        ("input:10", Some(InPlace::Input(10))),
        ("false", None),
        ("true", None),
        ("input:", None),
        ("input:-1", None),
        ("input:01", None),
        ("input:x", None),
        ("input: 1", None),
        ("Input:0", None),
        ("None", None),
        ("input:4294967296", None),
    ];
    for (text, want) in cases {
        let bytes = edited(&[(
            "in_place = none
",
            &format!(
                "in_place = {text}
"
            ),
        )]);
        let got = with(&bytes, |c| interface_values(c).map(|v| v.in_place));
        match want {
            Some(v) => assert_eq!(got, Ok(v), "{text:?}"),
            None => assert_eq!(got, Err(ValueError::InPlace(text.to_owned())), "{text:?}"),
        }
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
