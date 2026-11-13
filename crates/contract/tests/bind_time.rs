// SPDX-License-Identifier: MIT OR Apache-2.0
//! One test per KISS-Contract clause of §6.12 (-0001..-0006) and §6.6-0009, written from the Draft's text (KISS `origin/main` `2d7f20e`).
//! The cases mirror the shapes in KISS `conformance/tests/contract_bind_time.rs` (same owner, `MIT OR Apache-2.0`), which stays the source
//! of truth; a KISS vector file is not consumed here because the Draft publishes none for these clauses.

use fireline_contract::{BindDecline, Environment, Expr, ParseError};

const MAX: &str = "9223372036854775807";

fn env(n: i64) -> Environment {
    let mut e = Environment::new();
    e.set_scalar("n", n)
        .set_array("extents0", &[3, 4])
        .set_array("strides0", &[4, 1])
        .set_scalar("off0", 0);
    e
}

fn bind(text: &str, e: &Environment) -> Result<i64, BindDecline> {
    Expr::parse(text)
        .expect("a well-formed expression")
        .bind(e, None)
}

// KISS-CONTRACT-6.12-0001: one expression binds to a different value under each bound environment.
#[test]
fn bound_environment_gives_each_launch_its_own_value() {
    for n in [1, 12, 4096] {
        assert_eq!(bind("ceil_div(n, 4) * 4", &env(n)), Ok((n + 3) / 4 * 4));
    }
    assert_eq!(bind("extents0[1] * 2", &env(1)), Ok(8));
}

// KISS-CONTRACT-6.12-0002: a malformed expression is a build-time fault, never one of the bind-time codes.
#[test]
fn malformed_expression_is_build_time() {
    for bad in [
        "n +", "wobble(", "n[0]", "extents0", "wobble", "1 +* 2", "", "(n", "n n", "é",
    ] {
        let r = Expr::parse(bad);
        assert!(r.is_err(), "{bad:?} must not parse");
        assert_eq!(r.unwrap_err().code(), "build_time");
    }
    assert!(matches!(
        Expr::parse("wobble"),
        Err(ParseError::UnknownSymbol(_))
    ));
    assert!(matches!(Expr::parse("n[0]"), Err(ParseError::Subscript(_))));
    assert!(matches!(
        Expr::parse("extents0"),
        Err(ParseError::Subscript(_))
    ));
    assert!(matches!(
        Expr::parse("99999999999999999999"),
        Err(ParseError::Literal(_))
    ));
    let deep = format!("{}1{}", "(".repeat(200), ")".repeat(200));
    assert_eq!(Expr::parse(&deep), Err(ParseError::TooDeep));
}

// KISS-CONTRACT-6.12-0003: an unassigned symbol of the vocabulary declines `unbound_symbol`; no default is substituted.
#[test]
fn unbound_symbol_declines() {
    let e = Environment::new();
    let r = bind("n + 1", &e);
    assert_eq!(r, Err(BindDecline::UnboundSymbol("n".into())));
    assert_eq!(r.unwrap_err().code(), "unbound_symbol");
    assert_eq!(
        bind("extents3[0]", &e),
        Err(BindDecline::UnboundSymbol("extents3".into()))
    );
    // zero is a real value, not a stand-in for "unassigned"
    let mut z = Environment::new();
    z.set_scalar("n", 0);
    assert_eq!(bind("n", &z), Ok(0));
}

// KISS-CONTRACT-6.12-0004: only the FINAL value is gated, before any narrowing; a negative intermediate is legal.
#[test]
fn extent_range_gates_the_final_value_only() {
    let e = env(5);
    assert_eq!(
        bind("n - 9 + 9", &e),
        Ok(5),
        "negative intermediate is legal"
    );
    let r = bind("n - 9", &e);
    assert_eq!(r, Err(BindDecline::ExtentOutOfRange(-4)));
    assert_eq!(r.unwrap_err().code(), "extent_out_of_range");
    let x = Expr::parse("n").unwrap();
    assert_eq!(
        x.bind(&e, Some(0..=4)),
        Err(BindDecline::ExtentOutOfRange(5))
    );
    assert_eq!(x.bind(&e, Some(0..=5)), Ok(5));
    // a value past i32 is not narrowed or truncated here
    assert_eq!(
        Expr::parse("4294967296 * 1")
            .unwrap()
            .bind(&e, Some(0..=i64::MAX)),
        Ok(4_294_967_296)
    );
}

// KISS-CONTRACT-6.12-0005 and 6.6-0009(d): +, -, * and i64::MIN / -1 decline `affine_overflow`; never wrap or saturate.
#[test]
fn affine_overflow_declines() {
    let e = env(1);
    for text in [
        format!("{MAX} + 1"),
        format!("0 - {MAX} - 2"),
        format!("{MAX} * 2"),
    ] {
        let r = bind(&text, &e);
        assert_eq!(r, Err(BindDecline::AffineOverflow), "{text}");
        assert_eq!(r.unwrap_err().code(), "affine_overflow");
    }
    // i64::MIN is reachable only by arithmetic (there is no unary minus): MIN / -1
    let min = format!("(0 - {MAX} - 1)");
    assert_eq!(
        bind(&format!("{min} / (0 - 1)"), &e),
        Err(BindDecline::AffineOverflow)
    );
    assert_eq!(
        bind(&format!("{MAX} + 0"), &e),
        Ok(i64::MAX),
        "the edge itself is fine"
    );
}

// KISS-CONTRACT-6.12-0006: left to right; lookup before arithmetic; the first failing step names the decline; the range gate runs last.
#[test]
fn evaluation_order_names_the_first_failing_step() {
    let e = Environment::new();
    // overflow in term 1 is reported before an unbound symbol in term 2
    assert_eq!(
        bind(&format!("{MAX} + 1 + n"), &e),
        Err(BindDecline::AffineOverflow)
    );
    // an unbound symbol in term 1 comes before an overflow in term 2
    assert_eq!(
        bind(&format!("n + {MAX} * 2"), &e),
        Err(BindDecline::UnboundSymbol("n".into()))
    );
    // an arithmetic fault beats the range gate (the gate runs last)
    assert_eq!(
        bind(&format!("({MAX} + 1) - {MAX}"), &e),
        Err(BindDecline::AffineOverflow)
    );
}

// KISS-CONTRACT-6.6-0009: `/` truncates toward zero; ceil_div is the true ceiling with a positive divisor; a zero divisor declines.
#[test]
fn division_semantics_are_pinned() {
    let mut e = Environment::new();
    e.set_scalar("param0", 7)
        .set_scalar("param1", 10)
        .set_scalar("param2", 2);
    // -7 / 2 == -3 and never -4: intermediate negative, so add it back before the final gate
    assert_eq!(bind("(0 - param0) / param2 + 3", &e), Ok(0));
    assert_eq!(
        bind("(0 - param1) / 4 + 2", &e),
        Ok(0),
        "-10 / 4 truncates to -2"
    );
    // ceil_div(-10, 4) == -2 and never -1; ceil_div(10, 4) == 3; exact quotient unchanged
    assert_eq!(bind("ceil_div(0 - param1, 4) + 2", &e), Ok(0));
    assert_eq!(bind("ceil_div(param1, 4)", &e), Ok(3));
    assert_eq!(bind("ceil_div(8, 4)", &e), Ok(2));
    // CONTRACT-6.12-0007/-0008: a divisor that names a launch-scalar symbol declines at bind time ...
    let mut z = env(10);
    z.set_scalar("param0", 0).set_scalar("param2", -2);
    assert_eq!(bind("5 / param0", &z), Err(BindDecline::DivisionByZero));
    assert_eq!(
        bind("ceil_div(5, param0)", &z),
        Err(BindDecline::CeilDivDivisor(0))
    );
    assert_eq!(
        bind("ceil_div(5, param2)", &z),
        Err(BindDecline::CeilDivDivisor(-2))
    );
    // ... a constant divisor (no symbol) is a build-time fault (KISS#800)
    for bad in [
        "5 / 0",
        "n / 0",
        "ceil_div(8, 0)",
        "ceil_div(5, 0 - 2)",
        "n / (3 - 3)",
    ] {
        let r = Expr::parse(bad);
        assert!(
            matches!(r, Err(ParseError::ConstantDivisor(_))),
            "{bad}: {r:?}"
        );
        assert_eq!(r.unwrap_err().code(), "build_time");
    }
    // ceil_div is the true ceiling, not the truncating (a + b - 1) / b
    assert_eq!(
        bind("ceil_div(0 - 1, 4) + 1", &e),
        Ok(1),
        "ceil(-1/4) is 0, not -1"
    );
}

// KISS-CONTRACT-6.6-0006: precedence, left associativity, subscript bounds.
#[test]
fn grammar_precedence_and_subscripts() {
    let e = env(6);
    assert_eq!(bind("2 + 3 * 4", &e), Ok(14));
    assert_eq!(bind("(2 + 3) * 4", &e), Ok(20));
    assert_eq!(bind("20 - 5 - 3", &e), Ok(12), "left-associative");
    assert_eq!(bind("100 / 5 / 2", &e), Ok(10), "left-associative");
    assert_eq!(bind("extents0[0] * extents0[1]", &e), Ok(12));
    let r = bind("extents0[2]", &e);
    assert_eq!(
        r,
        Err(BindDecline::SubscriptOutOfRange {
            symbol: "extents0".into(),
            index: 2,
            len: 2
        })
    );
    assert_eq!(r.unwrap_err().code(), "subscript_out_of_range");
}

// KISS: an out-of-bounds subscript is BUILD-time when the contract declares the rank (the rank is the contract's, not the launch's).
#[test]
fn a_declared_rank_makes_an_out_of_bounds_subscript_a_build_time_fault() {
    for (text, symbol, index) in [
        ("extents0[2]", "extents0", 2),
        ("n + strides1[5]", "strides1", 5),
        ("ceil_div(extents0[0], idx_extents0[3])", "idx_extents0", 3),
    ] {
        let err = Expr::parse_with_rank(text, 2).unwrap_err();
        assert_eq!(
            err,
            ParseError::SubscriptOutOfBounds {
                symbol: symbol.into(),
                index,
                rank: 2
            },
            "{text}"
        );
        assert_eq!(err.code(), "build_time");
    }
    // in range, the same text parses and binds
    let e = Expr::parse_with_rank("extents0[1] * 2", 2).unwrap();
    assert_eq!(e.bind(&env(1), None), Ok(8));
    // rank 0 declares no axis at all: every subscript is out of bounds
    assert!(matches!(
        Expr::parse_with_rank("extents0[0]", 0),
        Err(ParseError::SubscriptOutOfBounds { rank: 0, .. })
    ));
    // without a declared rank, `parse` is unchanged and the check stays at bind time
    let e = Expr::parse("extents0[2]").unwrap();
    assert!(matches!(
        e.bind(&env(1), None),
        Err(BindDecline::SubscriptOutOfRange { .. })
    ));
    // with a rank, a short environment array is still a bind-time decline (the environment broke the rank)
    let e = Expr::parse_with_rank("extents0[1]", 2).unwrap();
    let mut short = Environment::new();
    short.set_array("extents0", &[7]);
    assert!(matches!(
        e.bind(&short, None),
        Err(BindDecline::SubscriptOutOfRange { .. })
    ));
}
