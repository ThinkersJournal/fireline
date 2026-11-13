// SPDX-License-Identifier: MIT OR Apache-2.0
//! Parsing a Dispatch expression in its contract's own context (KISS-CONTRACT-6.5-0004a, 6.6-0006): the launch-scalar vocabulary is the
//! symbols the contract's Interface `positional_signature` declares, and the rank is the contract's. Written from the Draft's clause text
//! (KISS origin/main 73c341b5); no KISS vector covers it. Draft-tracking: expect rework when the Draft moves.

use fireline_contract::interface::Signature;
use fireline_contract::{BindDecline, Context, Environment, Expr, ParseError};

// A fully packed cell: extents and the iteration count only, no strides, offsets or workspace.
const PACKED: &str = "[(ls, 1, extents0, i64, -, rank), (ls, 3, n, i32, -, scalar)]";
// A strided, sliced cell with a workspace.
const STRIDED: &str = "[(ls, 1, extents0, i64, -, rank), (ls, 2, strides0, i64, -, rank), (ls, 3, n, i32, -, scalar), (ls, 4, off0, i64, -, scalar), (ls, 6, ws_ptr, ptr, -, scalar), (ls, 7, ws_bytes, i64, -, scalar)]";

fn ctx(signature: &str, rank: usize) -> Context {
    Context::of_interface(rank, &Signature::parse(signature).unwrap())
}

// 6.5-0004a: a symbol of a class the Interface does not declare is outside that contract's vocabulary, so a Dispatch expression that names
// it is a build-time fault, though the same text parses against the standard's vocabulary.
#[test]
fn a_symbol_the_contract_does_not_declare_is_a_build_time_fault() {
    let packed = ctx(PACKED, 2);
    for text in [
        "strides0[0]",
        "n + off0",
        "ws_bytes",
        "ceil_div(n, 4) + param0",
        "extents1[0]",
    ] {
        let expected = match text {
            "strides0[0]" => "strides0",
            "n + off0" => "off0",
            "ws_bytes" => "ws_bytes",
            "ceil_div(n, 4) + param0" => "param0",
            _ => "extents1",
        };
        assert_eq!(
            Expr::parse_in(text, &packed),
            Err(ParseError::UnknownSymbol(expected.into())),
            "{text}"
        );
        // without the contract's vocabulary the same text is a well-formed expression over the standard's vocabulary
        assert!(Expr::parse(text).is_ok(), "{text}");
    }
    // the declared symbols, in every position, parse
    assert!(Expr::parse_in("ceil_div(extents0[1] * extents0[0], n) + n", &packed).is_ok());
}

// The vocabulary follows the Interface: the strided cell declares strides, offsets and the workspace size.
#[test]
fn the_vocabulary_follows_the_interface() {
    let strided = ctx(STRIDED, 2);
    for text in ["strides0[1] * extents0[1]", "off0 + n", "ws_bytes / 4", "n"] {
        assert!(Expr::parse_in(text, &strided).is_ok(), "{text}");
    }
    // ws_ptr (class 6) is a pointer and param0 is undeclared here: neither is a symbol this contract's Dispatch can name
    assert_eq!(
        Expr::parse_in("ws_ptr", &strided),
        Err(ParseError::UnknownSymbol("ws_ptr".into()))
    );
    assert_eq!(
        Expr::parse_in("param0", &strided),
        Err(ParseError::UnknownSymbol("param0".into()))
    );
}

// The rank travels with the vocabulary: a subscript past the declared rank is a build-time fault, and an unknown symbol is reported first.
#[test]
fn rank_and_vocabulary_together() {
    let c = ctx(STRIDED, 2);
    assert_eq!(
        Expr::parse_in("extents0[2]", &c),
        Err(ParseError::SubscriptOutOfBounds {
            symbol: "extents0".into(),
            index: 2,
            rank: 2
        })
    );
    assert!(Expr::parse_in("extents0[1]", &c).is_ok());
    assert_eq!(
        Expr::parse_in("ws_ptr[7]", &c),
        Err(ParseError::UnknownSymbol("ws_ptr".into())),
        "the vocabulary is checked before the subscript"
    );
    // the plain constructors are unchanged
    assert!(Expr::parse_with_rank("extents0[1]", 2).is_ok());
    assert!(Expr::parse_in("extents0[2]", &Context::new()).is_ok());
}

// End to end: parse in the contract's context at build time, bind at launch time.
#[test]
fn parse_in_the_contract_then_bind_at_launch() {
    let e = Expr::parse_in("ceil_div(extents0[0] * extents0[1], 256)", &ctx(PACKED, 2)).unwrap();
    let mut env = Environment::new();
    env.set_array("extents0", &[64, 100]);
    assert_eq!(e.bind(&env, None), Ok(25));
    // an unassigned declared symbol is still a bind-time decline, never a build-time one
    assert_eq!(
        Expr::parse_in("n + 1", &ctx(PACKED, 2))
            .unwrap()
            .bind(&Environment::new(), None),
        Err(BindDecline::UnboundSymbol("n".into()))
    );
}
