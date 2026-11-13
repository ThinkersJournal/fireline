// SPDX-License-Identifier: MIT OR Apache-2.0
//! The output-layout forms of KISS-Contract (6.5-0014 to 6.5-0019): the nine Interface blocks of KISS `contract_vectors.json`
//! (`output_layout_vectors`, origin/main 73c341b5, "n_inputs=2 n_outputs=1 rank=2") parsed and re-spelled, plus the clauses' own cases.
//! Draft-tracking: expect rework when the Draft moves.

use fireline_contract::interface::Signature;
use fireline_contract::layout::*;
use fireline_contract::{Context, ParseError};

/// (name, KISS `element`, the Interface block's bytes as hex), verbatim from the corpus.
const VECTORS: &[(&str, &str, &str)] = &[
    (
        "supplied",
        "-",
        "5b73656374696f6e3a333a696e746572666163655d0a656e7472795f706f696e74203d206b0a72616e6b203d20320a706f736974696f6e616c5f7369676e6174757265203d205b5d0a6c61756e63685f7363616c617273203d205b5d0a636f756e745f756e6974203d20656c656d656e74730a696e5f706c616365203d206e6f6e650a616c69676e6d656e745f6279746573203d2031360a6f75747075745f6c61796f757473203d205b2d5d0a",
    ),
    (
        "dense",
        "dense",
        "5b73656374696f6e3a333a696e746572666163655d0a656e7472795f706f696e74203d206b0a72616e6b203d20320a706f736974696f6e616c5f7369676e6174757265203d205b5d0a6c61756e63685f7363616c617273203d205b5d0a636f756e745f756e6974203d20656c656d656e74730a696e5f706c616365203d206e6f6e650a616c69676e6d656e745f6279746573203d2031360a6f75747075745f6c61796f757473203d205b64656e73655d0a",
    ),
    (
        "dense_order",
        "dense_order(1,0)",
        "5b73656374696f6e3a333a696e746572666163655d0a656e7472795f706f696e74203d206b0a72616e6b203d20320a706f736974696f6e616c5f7369676e6174757265203d205b5d0a6c61756e63685f7363616c617273203d205b5d0a636f756e745f756e6974203d20656c656d656e74730a696e5f706c616365203d206e6f6e650a616c69676e6d656e745f6279746573203d2031360a6f75747075745f6c61796f757473203d205b64656e73655f6f7264657228312c30295d0a",
    ),
    (
        "same_as",
        "same_as(0)",
        "5b73656374696f6e3a333a696e746572666163655d0a656e7472795f706f696e74203d206b0a72616e6b203d20320a706f736974696f6e616c5f7369676e6174757265203d205b5d0a6c61756e63685f7363616c617273203d205b5d0a636f756e745f756e6974203d20656c656d656e74730a696e5f706c616365203d206e6f6e650a616c69676e6d656e745f6279746573203d2031360a6f75747075745f6c61796f757473203d205b73616d655f61732830295d0a",
    ),
    (
        "strided",
        "strided(strides0[0],1)",
        "5b73656374696f6e3a333a696e746572666163655d0a656e7472795f706f696e74203d206b0a72616e6b203d20320a706f736974696f6e616c5f7369676e6174757265203d205b5d0a6c61756e63685f7363616c617273203d205b5d0a636f756e745f756e6974203d20656c656d656e74730a696e5f706c616365203d206e6f6e650a616c69676e6d656e745f6279746573203d2031360a6f75747075745f6c61796f757473203d205b737472696465642873747269646573305b305d2c31295d0a",
    ),
    (
        "nested",
        "(2,2):(1,2)",
        "5b73656374696f6e3a333a696e746572666163655d0a656e7472795f706f696e74203d206b0a72616e6b203d20320a706f736974696f6e616c5f7369676e6174757265203d205b5d0a6c61756e63685f7363616c617273203d205b5d0a636f756e745f756e6974203d20656c656d656e74730a696e5f706c616365203d206e6f6e650a616c69676e6d656e745f6279746573203d2031360a6f75747075745f6c61796f757473203d205b28322c32293a28312c32295d0a",
    ),
    (
        "nested_swizzled",
        "swz(2,3,-3)((8,8):(8,1))",
        "5b73656374696f6e3a333a696e746572666163655d0a656e7472795f706f696e74203d206b0a72616e6b203d20320a706f736974696f6e616c5f7369676e6174757265203d205b5d0a6c61756e63685f7363616c617273203d205b5d0a636f756e745f756e6974203d20656c656d656e74730a696e5f706c616365203d206e6f6e650a616c69676e6d656e745f6279746573203d2031360a6f75747075745f6c61796f757473203d205b73777a28322c332c2d33292828382c38293a28382c3129295d0a",
    ),
    (
        "bundle",
        "bundle(a:f32[n,4]=dense@0,b:bf16[4,n]=dense_order(1,0)@ceil_div(n*4,16)*16)",
        "5b73656374696f6e3a333a696e746572666163655d0a656e7472795f706f696e74203d206b0a72616e6b203d20320a706f736974696f6e616c5f7369676e6174757265203d205b5d0a6c61756e63685f7363616c617273203d205b5d0a636f756e745f756e6974203d20656c656d656e74730a696e5f706c616365203d206e6f6e650a616c69676e6d656e745f6279746573203d2031360a6f75747075745f6c61796f757473203d205b62756e646c6528613a6633325b6e2c345d3d64656e736540302c623a626631365b342c6e5d3d64656e73655f6f7264657228312c3029406365696c5f646976286e2a342c3136292a3136295d0a",
    ),
    (
        "none",
        "none",
        "5b73656374696f6e3a333a696e746572666163655d0a656e7472795f706f696e74203d206b0a72616e6b203d20320a706f736974696f6e616c5f7369676e6174757265203d205b5d0a6c61756e63685f7363616c617273203d205b5d0a636f756e745f756e6974203d20656c656d656e74730a696e5f706c616365203d206e6f6e650a616c69676e6d656e745f6279746573203d2031360a6f75747075745f6c61796f757473203d205b6e6f6e655d0a",
    ),
];

const SIGNATURE: &str = "[(ls, 1, extents0, i64, -, rank), (ls, 2, strides0, i64, -, rank), (ls, 3, n, i32, -, scalar)]";

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
        .collect()
}

fn is_dtype(t: &str) -> bool {
    ["bf16", "f16", "f32", "i32", "u8"].contains(&t)
}

fn ctx() -> Context {
    Context::of_interface(2, &Signature::parse(SIGNATURE).unwrap())
}

fn parse(text: &str) -> Result<Layout, LayoutError> {
    let c = ctx();
    parse_layout(
        text,
        &Env {
            rank: 2,
            n_inputs: 2,
            context: &c,
            is_dtype: &is_dtype,
        },
    )
}

fn field(value: &str, n_outputs: usize) -> Result<Vec<OutputGuarantee>, LayoutError> {
    let c = ctx();
    parse_output_layouts(
        value,
        n_outputs,
        &Env {
            rank: 2,
            n_inputs: 2,
            context: &c,
            is_dtype: &is_dtype,
        },
    )
}

// 6.5-0014 to 6.5-0019: every KISS vector's Interface block carries the element in its `output_layouts` line (or none for the supplied
// layout); each element parses and is re-spelled byte for byte.
#[test]
fn every_kiss_vector_parses_and_respells() {
    for (name, element, hex) in VECTORS {
        let block = String::from_utf8(unhex(hex)).expect("the Interface block is UTF-8");
        let line = block
            .lines()
            .find_map(|l| l.strip_prefix("output_layouts = "));
        let (guarantees, spelled) = match line {
            Some(value) => (
                field(value, 1).unwrap_or_else(|e| panic!("{name}: {e}")),
                value.to_owned(),
            ),
            None => (field("[-]", 1).unwrap(), "[-]".to_owned()),
        };
        assert_eq!(guarantees.len(), 1, "{name}");
        assert_eq!(guarantees[0].refused, None, "{name}");
        let text = guarantees[0].layout.to_string();
        assert_eq!(&text, element, "{name}: the element re-spells");
        assert_eq!(
            spelled,
            format!("[{element}]"),
            "{name}: the field is the one-element array"
        );
        // and the form parses on its own, to the same value
        assert_eq!(parse(element).unwrap(), guarantees[0].layout, "{name}");
    }
    assert_eq!(VECTORS.len(), 9);
}

// 6.5-0014: an absent field means every output is `-`; the count must match, with no padding or truncating.
#[test]
fn the_output_count_must_match() {
    assert_eq!(field("[]", 0).unwrap(), vec![]);
    assert_eq!(field("[dense, -]", 2).unwrap().len(), 2);
    assert_eq!(
        field("[dense, -]", 1),
        Err(LayoutError::OutputCount { want: 1, got: 2 })
    );
    assert_eq!(
        field("[dense]", 2),
        Err(LayoutError::OutputCount { want: 2, got: 1 })
    );
    assert_eq!(
        field("[]", 1),
        Err(LayoutError::OutputCount { want: 1, got: 0 })
    );
    for bad in [
        "dense",
        "[dense",
        "dense]",
        "[dense,-]",
        "[dense,  -]",
        "[dense ,-]",
    ] {
        assert!(field(bad, 2).is_err(), "{bad}");
    }
}

// 6.5-0015: the set is closed; an unknown head refuses that output's guarantee (read as `none`, recorded), not the contract.
#[test]
fn an_unknown_form_refuses_only_its_guarantee() {
    let g = field("[wobble(1,2), dense]", 2).unwrap();
    assert_eq!(
        g[0],
        OutputGuarantee {
            layout: Layout::None,
            refused: Some("wobble".into())
        }
    );
    assert_eq!(
        g[1],
        OutputGuarantee {
            layout: Layout::Dense,
            refused: None
        }
    );
    assert_eq!(
        parse("morton"),
        Err(LayoutError::UnknownForm("morton".into()))
    );
    assert_eq!(
        parse("Dense"),
        Err(LayoutError::UnknownForm("Dense".into()))
    );
}

// 6.5-0016: no space, canonical integers; dense_order is a permutation of exactly rank entries; same_as indexes an input; strided states
// exactly rank expressions over the contract's vocabulary.
#[test]
fn the_simple_forms() {
    assert_eq!(
        parse("dense_order(1,0)").unwrap(),
        Layout::DenseOrder(vec![1, 0])
    );
    for bad in [
        "dense_order(0,0)",
        "dense_order(0)",
        "dense_order(0,1,2)",
        "dense_order(0,2)",
        "dense_order()",
    ] {
        assert!(
            matches!(
                parse(bad),
                Err(LayoutError::NotAPermutation | LayoutError::NonCanonicalInteger(_))
            ),
            "{bad}"
        );
    }
    assert_eq!(parse("same_as(1)").unwrap(), Layout::SameAs(1));
    assert_eq!(parse("same_as(2)"), Err(LayoutError::SameAsOutOfRange(2)));
    assert_eq!(
        parse("strided(strides0[0],1)").unwrap(),
        Layout::Strided(vec!["strides0[0]".into(), "1".into()])
    );
    assert_eq!(
        parse("strided(1)"),
        Err(LayoutError::StridedRank { want: 2, got: 1 })
    );
    assert_eq!(
        parse("strided(1,2,3)"),
        Err(LayoutError::StridedRank { want: 2, got: 3 })
    );
    // an expression outside the contract's vocabulary (off0 is not declared) or malformed is a typed decline
    assert_eq!(
        parse("strided(off0,1)"),
        Err(LayoutError::BadExpression(ParseError::UnknownSymbol(
            "off0".into()
        )))
    );
    assert!(matches!(
        parse("strided(1+,1)"),
        Err(LayoutError::BadExpression(_))
    ));
    // spaces and non-canonical integers
    assert_eq!(parse("dense_order(1, 0)"), Err(LayoutError::HasSpace));
    assert_eq!(parse(" dense"), Err(LayoutError::HasSpace));
    for bad in [
        "same_as(+1)",
        "same_as(01)",
        "same_as(-1)",
        "same_as(1.0)",
        "dense_order(+1,0)",
        "dense_order(01,0)",
        "same_as()",
    ] {
        assert!(
            matches!(parse(bad), Err(LayoutError::NonCanonicalInteger(_))),
            "{bad}"
        );
    }
}

// 6.5-0017: congruent profiles, shape leaves >= 1, stride leaves >= 0, never an empty tuple, depth at most 16, one mode per axis.
#[test]
fn the_nested_layout() {
    assert!(parse("(2,2):(1,2)").is_ok());
    assert!(parse("(2,(2,3)):(1,(2,4))").is_ok());
    assert!(
        parse("(2,2):(0,1)").is_ok(),
        "a zero stride leaf is allowed"
    );
    assert_eq!(
        parse("(2,2):(1,(2,3))"),
        Err(LayoutError::NestedNotCongruent)
    );
    assert_eq!(parse("(2,2):(1,2,3)"), Err(LayoutError::NestedNotCongruent));
    assert_eq!(parse("(0,2):(1,2)"), Err(LayoutError::NestedLeaf));
    assert_eq!(parse("(2,2):(-1,2)"), Err(LayoutError::NestedLeaf));
    assert_eq!(parse("():()"), Err(LayoutError::NestedShape));
    assert_eq!(
        parse("(2,2,2):(1,2,4)"),
        Err(LayoutError::NestedRank { want: 2, got: 3 })
    );
    assert_eq!(
        parse("4:1"),
        Err(LayoutError::NestedRank { want: 2, got: 1 })
    );
    // depth: 16 nested tuples is the limit, 17 is not (a tuple of depth d inside a pair, to keep the top-level mode count at 2)
    let deep = |d: usize| {
        let (mut shape, mut stride) = ("2".to_string(), "1".to_string());
        for _ in 0..d {
            shape = format!("({shape},2)");
            stride = format!("({stride},1)");
        }
        format!("{shape}:{stride}")
    };
    assert!(parse(&deep(16)).is_ok());
    assert_eq!(parse(&deep(17)), Err(LayoutError::NestedShape));
    // a run-time stride cannot be written inside a nested layout (6.5-0017: every leaf is an integer)
    assert!(parse("(2,2):(n,1)").is_err());
}

// 6.5-0018: `swz(B,M,S)(shape:stride)`, B and M non-negative, S signed.
#[test]
fn the_swizzle() {
    assert_eq!(
        parse("swz(2,3,-3)((8,8):(8,1))").unwrap(),
        Layout::Nested {
            swizzle: Some(Swizzle {
                bits: 2,
                base: 3,
                shift: -3
            }),
            shape: Tuple::Tuple(vec![Tuple::Int(8), Tuple::Int(8)]),
            stride: Tuple::Tuple(vec![Tuple::Int(8), Tuple::Int(1)]),
        }
    );
    for bad in [
        "swz(2,3)((8,8):(8,1))",
        "swz(-2,3,3)((8,8):(8,1))",
        "swz(2,3,03)((8,8):(8,1))",
        "swz(2,3,3)(8,8):(8,1)",
        "swz(2,3,-0)((8,8):(8,1))",
    ] {
        assert!(parse(bad).is_err(), "{bad}");
    }
}

// 6.5-0019: slots `name:dtype[shape]=layout@offset`, every part mandatory, unique `[a-z0-9_]+` names, a Classify dtype, a layout other than
// bundle and none whose stated rank equals the shape's, expressions over the contract's vocabulary.
#[test]
fn the_bundle_slots() {
    let ok = "bundle(a:f32[n,4]=dense@0,b:bf16[4,n]=dense_order(1,0)@ceil_div(n*4,16)*16)";
    let Layout::Bundle(slots) = parse(ok).unwrap() else {
        panic!("a bundle")
    };
    assert_eq!(slots.len(), 2);
    assert_eq!(
        (
            slots[0].name.as_str(),
            slots[0].dtype.as_str(),
            slots[0].shape.len()
        ),
        ("a", "f32", 2)
    );
    assert_eq!(slots[1].offset, "ceil_div(n*4,16)*16");
    assert!(parse("bundle(s:u8[]=dense@0)").is_ok(), "a rank-0 slot");
    assert_eq!(
        parse("bundle(a:f32[n,4]=dense@0,a:f32[n]=dense@0)"),
        Err(LayoutError::SlotName("a".into()))
    );
    assert_eq!(
        parse("bundle(A:f32[n]=dense@0)"),
        Err(LayoutError::SlotName("A".into()))
    );
    assert_eq!(
        parse("bundle(a:f33[n]=dense@0)"),
        Err(LayoutError::UnknownDtype("f33".into()))
    );
    assert_eq!(
        parse("bundle(a:f32[n]=none@0)"),
        Err(LayoutError::SlotLayoutForm)
    );
    assert_eq!(
        parse("bundle(a:f32[n]=-@0)"),
        Err(LayoutError::SlotLayoutForm)
    );
    // the stated rank must equal the shape's rank
    assert_eq!(
        parse("bundle(a:f32[n]=dense_order(1,0)@0)"),
        Err(LayoutError::NotAPermutation)
    );
    assert_eq!(
        parse("bundle(a:f32[n,4]=strided(1)@0)"),
        Err(LayoutError::StridedRank { want: 2, got: 1 })
    );
    assert!(matches!(
        parse("bundle(a:f32[n,4]=(2,2,2):(1,2,4)@0)"),
        Err(LayoutError::NestedRank { want: 2, got: 3 })
    ));
    // dense and same_as state no rank
    assert!(parse("bundle(a:f32[n,4,2]=dense@0)").is_ok());
    // every part is mandatory; an expression must be over the vocabulary
    for bad in [
        "bundle(a:f32[n,4]=dense)",
        "bundle(a:f32[n,4]@0)",
        "bundle(a:f32=dense@0)",
        "bundle(a[n]=dense@0)",
        "bundle()",
    ] {
        assert!(parse(bad).is_err(), "{bad}");
    }
    assert!(matches!(
        parse("bundle(a:f32[off0]=dense@0)"),
        Err(LayoutError::BadExpression(_))
    ));
    assert!(matches!(
        parse("bundle(a:f32[n]=dense@off0)"),
        Err(LayoutError::BadExpression(_))
    ));
    // a bundle is not a slot layout
    assert_eq!(
        parse("bundle(a:f32[n]=bundle(b:f32[n]=dense@0)@0)"),
        Err(LayoutError::SlotLayoutForm)
    );
}
