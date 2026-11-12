// SPDX-License-Identifier: MIT OR Apache-2.0
//! **Bind-time obligations** of KISS-Contract: parse a Dispatch expression (KISS-CONTRACT-6.6-0006) and *bind* it against a launch
//! environment (6.12-0001), with the pinned division semantics (6.6-0009) and the typed declines of 6.12-0003..-0006.
//!
//! A contract is produced and read at **build time**; a kernel is launched at **bind time**, against a [`Environment`]. A malformed
//! expression is a build-time fault ([`ParseError`], 6.12-0002), never a bind-time decline ([`BindDecline`]).
//!
//! **Draft-tracking.** KISS-Contract §6.12 is a Draft (the freeze gate is unmet); this crate follows it, expect rework when it moves.
//! Every rule below cites its clause id. Where the Draft does not name a decline code, the code here is this crate's choice and is
//! marked `(not named by the Draft)`.
//!
//! # Hello world
//!
//! ```
//! use fireline_contract::{BindDecline, Environment, Expr};
//!
//! let expr = Expr::parse("ceil_div(n, 4) * 4")?;
//! let mut env = Environment::new();
//! env.set_scalar("n", 10);
//! assert_eq!(expr.bind(&env, None), Ok(12));
//!
//! // A symbol the environment does not assign declines; no zero or default is substituted (6.12-0003).
//! assert_eq!(expr.bind(&Environment::new(), None), Err(BindDecline::UnboundSymbol("n".into())));
//! # Ok::<(), fireline_contract::ParseError>(())
//! ```

use std::collections::BTreeMap;
use std::fmt;
use std::ops::RangeInclusive;

/// Nesting limit for parentheses and calls: a hostile expression must not overflow the stack (KISS-CONTRACT-6.1-0004 spirit).
/// This limit is this crate's choice (not named by the Draft).
pub const MAX_DEPTH: usize = 128;

/// A build-time fault: the text is not an expression the §6.6-0006 grammar accepts over the launch-scalar vocabulary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Not ASCII, an unexpected character, an unexpected end, trailing input, or an unknown function.
    Syntax(String),
    /// A name outside the launch-scalar symbol vocabulary of 6.5-0004a (6.6-0006: "MUST NOT reference a symbol absent from the vocabulary").
    UnknownSymbol(String),
    /// A scalar symbol was subscripted, or a rank-length array symbol was used without a subscript (6.6-0006).
    Subscript(String),
    /// A decimal literal that does not fit a signed 64-bit integer (not named by the Draft).
    Literal(String),
    /// Nesting deeper than [`MAX_DEPTH`].
    TooDeep,
}

impl ParseError {
    /// The code is always `build_time`: 6.12-0002 says a malformed expression is a build-time fault.
    #[must_use]
    pub fn code(&self) -> &'static str {
        "build_time"
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax(m) => write!(f, "build_time: syntax: {m}"),
            Self::UnknownSymbol(s) => write!(
                f,
                "build_time: `{s}` is not in the launch-scalar vocabulary"
            ),
            Self::Subscript(s) => write!(f, "build_time: wrong subscript use of `{s}`"),
            Self::Literal(s) => write!(f, "build_time: literal `{s}` does not fit i64"),
            Self::TooDeep => write!(f, "build_time: nesting deeper than {MAX_DEPTH}"),
        }
    }
}

impl std::error::Error for ParseError {}

/// A typed decline raised at bind time. The first failing step names the decline (6.12-0006).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindDecline {
    /// `unbound_symbol` (6.12-0003): the symbol is in the vocabulary but the environment does not assign it.
    UnboundSymbol(String),
    /// `extent_out_of_range` (6.12-0004): the final value is negative or outside the declared range.
    ExtentOutOfRange(i64),
    /// `affine_overflow` (6.12-0005): `+`, `-` or `*` left the signed 64-bit range, or `i64::MIN / -1` (6.6-0009(d)).
    AffineOverflow,
    /// `division_by_zero` (6.6-0009(c); code name not named by the Draft).
    DivisionByZero,
    /// `ceil_div_divisor` (6.6-0009(b): a zero or negative divisor of `ceil_div`; code name not named by the Draft).
    CeilDivDivisor(i64),
    /// `subscript_out_of_range` (6.6-0006: an out-of-bounds subscript is a typed decline; the Draft does not say whether it is a build-time
    /// or bind-time one, and the rank is known only from the environment, so it is raised here; code name not named by the Draft).
    SubscriptOutOfRange {
        symbol: String,
        index: usize,
        len: usize,
    },
}

impl BindDecline {
    /// The decline code.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::UnboundSymbol(_) => "unbound_symbol",
            Self::ExtentOutOfRange(_) => "extent_out_of_range",
            Self::AffineOverflow => "affine_overflow",
            Self::DivisionByZero => "division_by_zero",
            Self::CeilDivDivisor(_) => "ceil_div_divisor",
            Self::SubscriptOutOfRange { .. } => "subscript_out_of_range",
        }
    }
}

impl fmt::Display for BindDecline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnboundSymbol(s) => write!(f, "unbound_symbol: `{s}`"),
            Self::ExtentOutOfRange(v) => write!(f, "extent_out_of_range: {v}"),
            Self::AffineOverflow => write!(f, "affine_overflow"),
            Self::DivisionByZero => write!(f, "division_by_zero"),
            Self::CeilDivDivisor(d) => write!(f, "ceil_div_divisor: {d}"),
            Self::SubscriptOutOfRange { symbol, index, len } => {
                write!(
                    f,
                    "subscript_out_of_range: `{symbol}[{index}]`, length {len}"
                )
            }
        }
    }
}

impl std::error::Error for BindDecline {}

/// A **bound environment** (6.12-0001): an `i64` for each assigned launch-scalar symbol and a length-`rank` array for each assigned
/// rank-length array symbol. A symbol of the vocabulary that is not assigned stays unbound.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Environment {
    scalars: BTreeMap<String, i64>,
    arrays: BTreeMap<String, Vec<i64>>,
}

impl Environment {
    /// An environment that assigns nothing.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Assign a scalar symbol: `n`, `ws_bytes`, `off{i}` or `param{i}` (6.5-0004a classes 3, 4, 7, 8).
    ///
    /// A name outside that set is stored but no expression can reference it (`Expr::parse` rejects it).
    pub fn set_scalar(&mut self, symbol: &str, value: i64) -> &mut Self {
        self.scalars.insert(symbol.to_owned(), value);
        self
    }

    /// Assign a rank-length array symbol: `extents{i}`, `strides{i}` or `idx_extents{i}` (6.5-0004a classes 1, 2, 5).
    pub fn set_array(&mut self, symbol: &str, values: &[i64]) -> &mut Self {
        self.arrays.insert(symbol.to_owned(), values.to_vec());
        self
    }
}

/// How a symbol name is classified by the launch-scalar vocabulary (6.5-0004a).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Scalar,
    Array,
}

/// `Some(kind)` for a name in the launch-scalar vocabulary of 6.5-0004a. The set is the standard's; whether a *contract's* Interface
/// declares it (6.5-0004a "the document's vocabulary") is not known here, so a name of the right shape that the environment does not
/// assign is `unbound_symbol`, never a build-time fault.
fn classify(name: &str) -> Option<Kind> {
    let indexed = |prefix: &str| {
        name.strip_prefix(prefix)
            .is_some_and(|d| !d.is_empty() && d.bytes().all(|b| b.is_ascii_digit()))
    };
    if name == "n" || name == "ws_bytes" || indexed("off") || indexed("param") {
        Some(Kind::Scalar)
    } else if indexed("extents") || indexed("strides") || indexed("idx_extents") {
        Some(Kind::Array)
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Lit(i64),
    Scalar(String),
    Elem(String, usize),
    Bin(char, Box<Node>, Box<Node>),
    CeilDiv(Box<Node>, Box<Node>),
}

/// A parsed Dispatch expression (6.6-0006): symbols, non-negative decimal literals, `+ - * /`, `ceil_div(a, b)`, parentheses and `sym[k]`.
/// There is no unary minus. `*` and `/` bind tighter than `+` and `-`; equal precedence is left-associative.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr {
    root: Node,
}

struct Parser<'a> {
    s: &'a [u8],
    at: usize,
    depth: usize,
}

impl Parser<'_> {
    fn ws(&mut self) {
        while self.s.get(self.at).is_some_and(u8::is_ascii_whitespace) {
            self.at += 1;
        }
    }

    fn peek(&mut self) -> Option<u8> {
        self.ws();
        self.s.get(self.at).copied()
    }

    fn eat(&mut self, c: u8) -> bool {
        if self.peek() == Some(c) {
            self.at += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, c: u8) -> Result<(), ParseError> {
        if self.eat(c) {
            Ok(())
        } else {
            Err(self.err(&format!("expected `{}`", c as char)))
        }
    }

    fn err(&self, m: &str) -> ParseError {
        ParseError::Syntax(format!("{m} at byte {}", self.at))
    }

    fn enter(&mut self) -> Result<(), ParseError> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            Err(ParseError::TooDeep)
        } else {
            Ok(())
        }
    }

    fn expr(&mut self) -> Result<Node, ParseError> {
        self.enter()?;
        let mut lhs = self.term()?;
        while let Some(op @ (b'+' | b'-')) = self.peek() {
            self.at += 1;
            let rhs = self.term()?;
            lhs = Node::Bin(op as char, Box::new(lhs), Box::new(rhs));
        }
        self.depth -= 1;
        Ok(lhs)
    }

    fn term(&mut self) -> Result<Node, ParseError> {
        let mut lhs = self.atom()?;
        while let Some(op @ (b'*' | b'/')) = self.peek() {
            self.at += 1;
            let rhs = self.atom()?;
            lhs = Node::Bin(op as char, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }

    fn atom(&mut self) -> Result<Node, ParseError> {
        match self.peek() {
            Some(b'(') => {
                self.at += 1;
                let e = self.expr()?;
                self.expect(b')')?;
                Ok(e)
            }
            Some(c) if c.is_ascii_digit() => {
                let start = self.at;
                while self.s.get(self.at).is_some_and(u8::is_ascii_digit) {
                    self.at += 1;
                }
                let text = std::str::from_utf8(&self.s[start..self.at]).unwrap_or_default();
                text.parse::<i64>()
                    .map(Node::Lit)
                    .map_err(|_| ParseError::Literal(text.to_owned()))
            }
            Some(c) if c.is_ascii_alphabetic() || c == b'_' => {
                let start = self.at;
                while self
                    .s
                    .get(self.at)
                    .is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_')
                {
                    self.at += 1;
                }
                let name = std::str::from_utf8(&self.s[start..self.at])
                    .unwrap_or_default()
                    .to_owned();
                self.named(name)
            }
            Some(_) => Err(self.err("unexpected character")),
            None => Err(self.err("unexpected end")),
        }
    }

    fn named(&mut self, name: String) -> Result<Node, ParseError> {
        if name == "ceil_div" {
            self.expect(b'(')?;
            let a = self.expr()?;
            self.expect(b',')?;
            let b = self.expr()?;
            self.expect(b')')?;
            return Ok(Node::CeilDiv(Box::new(a), Box::new(b)));
        }
        let kind = classify(&name).ok_or_else(|| {
            if self.peek() == Some(b'(') {
                ParseError::Syntax(format!("unknown function `{name}`"))
            } else {
                ParseError::UnknownSymbol(name.clone())
            }
        })?;
        let subscripted = self.eat(b'[');
        match (kind, subscripted) {
            (Kind::Scalar, false) => Ok(Node::Scalar(name)),
            (Kind::Array, true) => {
                self.ws();
                let start = self.at;
                while self.s.get(self.at).is_some_and(u8::is_ascii_digit) {
                    self.at += 1;
                }
                let text = std::str::from_utf8(&self.s[start..self.at]).unwrap_or_default();
                // 6.6-0006: `k` is a non-negative decimal integer literal.
                let k = text
                    .parse::<usize>()
                    .map_err(|_| self.err("expected a non-negative integer subscript"))?;
                self.expect(b']')?;
                Ok(Node::Elem(name, k))
            }
            _ => Err(ParseError::Subscript(name)),
        }
    }
}

impl Expr {
    /// Parse an expression. A fault here is a **build-time** fault (6.12-0002), never one of the bind-time declines.
    ///
    /// # Errors
    /// [`ParseError`] when the text is not accepted by the §6.6-0006 grammar over the launch-scalar vocabulary.
    pub fn parse(text: &str) -> Result<Self, ParseError> {
        if !text.is_ascii() {
            return Err(ParseError::Syntax(
                "expressions are ASCII (6.6-0006)".into(),
            ));
        }
        let mut p = Parser {
            s: text.as_bytes(),
            at: 0,
            depth: 0,
        };
        let root = p.expr()?;
        if p.peek().is_some() {
            return Err(p.err("trailing input"));
        }
        Ok(Self { root })
    }

    /// Bind the expression against `env` at **bind time** (6.12-0001).
    ///
    /// Evaluation is left to right; at each operand the symbol lookup (6.12-0003) comes first, then the arithmetic step (6.12-0005, 6.6-0009),
    /// and the first failing step names the decline (6.12-0006). The non-negative and range gate (6.12-0004) runs last, on the final value
    /// only: a negative intermediate is legal. `range` is the range the field declares, if any; it is checked before any narrowing.
    ///
    /// # Errors
    /// [`BindDecline`] naming the first failing step.
    pub fn bind(
        &self,
        env: &Environment,
        range: Option<RangeInclusive<i64>>,
    ) -> Result<i64, BindDecline> {
        let v = bind_node(&self.root, env)?;
        if v < 0 || range.is_some_and(|r| !r.contains(&v)) {
            return Err(BindDecline::ExtentOutOfRange(v));
        }
        Ok(v)
    }
}

/// Walks the parsed arithmetic tree to an `i64`. This is a plain integer-expression walker: it never runs code, shells out or reads input.
fn bind_node(node: &Node, env: &Environment) -> Result<i64, BindDecline> {
    match node {
        Node::Lit(v) => Ok(*v),
        Node::Scalar(s) => env
            .scalars
            .get(s)
            .copied()
            .ok_or_else(|| BindDecline::UnboundSymbol(s.clone())),
        Node::Elem(s, k) => {
            let a = env
                .arrays
                .get(s)
                .ok_or_else(|| BindDecline::UnboundSymbol(s.clone()))?;
            a.get(*k)
                .copied()
                .ok_or_else(|| BindDecline::SubscriptOutOfRange {
                    symbol: s.clone(),
                    index: *k,
                    len: a.len(),
                })
        }
        Node::Bin(op, l, r) => {
            // Left operand fully, then right operand fully, then the arithmetic step (6.12-0006).
            let a = bind_node(l, env)?;
            let b = bind_node(r, env)?;
            match op {
                '+' => a.checked_add(b).ok_or(BindDecline::AffineOverflow),
                '-' => a.checked_sub(b).ok_or(BindDecline::AffineOverflow),
                '*' => a.checked_mul(b).ok_or(BindDecline::AffineOverflow),
                // `/` truncates toward zero (6.6-0009(a)); a zero divisor is a decline (c); i64::MIN / -1 is a decline (d).
                _ => {
                    if b == 0 {
                        Err(BindDecline::DivisionByZero)
                    } else {
                        a.checked_div(b).ok_or(BindDecline::AffineOverflow)
                    }
                }
            }
        }
        Node::CeilDiv(l, r) => {
            let a = bind_node(l, env)?;
            let b = bind_node(r, env)?;
            // 6.6-0009(b): the true ceiling, divisor positive; never the truncating (a + b - 1) / b.
            if b <= 0 {
                return Err(BindDecline::CeilDivDivisor(b));
            }
            let (q, r) = (a / b, a % b);
            Ok(if r > 0 { q + 1 } else { q })
        }
    }
}
