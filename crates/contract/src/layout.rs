// SPDX-License-Identifier: MIT OR Apache-2.0
//! The Interface **`output_layouts`** of KISS-Contract (6.5-0014 to 6.5-0019): the seven closed layout forms (`dense`, `dense_order(..)`,
//! `same_as(i)`, `strided(..)`, a nested layout with an optional swizzle, `bundle(..)`, `none`), their canonical text (no space, canonical
//! integers), and the per-output array with the literal `-` for "no guarantee: the layout the caller supplied".
//!
//! **Draft-tracking.** KISS-Contract is a Draft; the 7-form grammar is stable for it (kiss, 2026-11-13) but expect rework when it moves.
//! The nine Interface blocks of KISS `contract_vectors.json` (`output_layout_vectors`, origin/main 73c341b5) are diffed in
//! `tests/layout.rs`. The `bundle` form is parsed and its slot shape, rank and dtype are checked, but the bind-time `slot_overlap` check
//! (6.5-0019, 6.12-0002) is not here.
//!
//! **Dtypes.** A slot's dtype must be a Classify dtype token (6.5-0019: an unknown token is a typed decline). The Classify table is not
//! copied here: the parsers take an `is_dtype` predicate, as in `fireline_announce::device`.

use core::fmt;

use crate::{Context, Expr, ParseError};

/// The deepest nesting of a nested layout's tuples: 0 for an integer, `1 +` the deepest child for a tuple (6.5-0017).
pub const MAX_NEST_DEPTH: usize = 16;

/// A fault in an `output_layouts` value or a layout form. Each is a typed decline; never a repair.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum LayoutError {
    /// The value is not `[` ... `]` with elements separated by `, ` (6.11-0001).
    NotAnArray,
    /// The element count differs from the number of output operands (6.5-0014): no padding, no truncating.
    OutputCount {
        /// The number of output operands.
        want: usize,
        /// The number of elements.
        got: usize,
    },
    /// A form with a space (6.5-0016).
    HasSpace,
    /// A non-canonical integer: a leading `+`, a leading zero, `-0`, or not a decimal integer (6.5-0016).
    NonCanonicalInteger(String),
    /// A form whose head is none of the seven (6.5-0015): the guarantee is refused, not the contract; see [`parse_output_layouts`].
    UnknownForm(String),
    /// `dense_order` is not a permutation of `0..rank` of exactly `rank` entries (6.5-0016).
    NotAPermutation,
    /// `same_as(i)` with `i` not less than the number of input operands (6.5-0016).
    SameAsOutOfRange(usize),
    /// `strided(..)` does not state exactly `rank` strides (6.5-0016).
    StridedRank {
        /// The rank.
        want: usize,
        /// The number of strides stated.
        got: usize,
    },
    /// A stride or slot expression that does not parse over this contract's vocabulary (6.5-0016).
    BadExpression(ParseError),
    /// A malformed nested layout or swizzle spelling (6.5-0017, -0018).
    NestedSyntax,
    /// Nested shape and stride have different nesting profiles (6.5-0017).
    NestedNotCongruent,
    /// A nested shape leaf below 1, or a negative stride leaf (6.5-0017).
    NestedLeaf,
    /// A nested layout deeper than [`MAX_NEST_DEPTH`], or an empty tuple (6.5-0017).
    NestedShape,
    /// A nested layout whose top-level mode count differs from the rank (6.5-0017, -0019).
    NestedRank {
        /// The rank.
        want: usize,
        /// The top-level mode count.
        got: usize,
    },
    /// A malformed `bundle` or slot (6.5-0019).
    BundleSyntax,
    /// A slot name outside `[a-z0-9_]+`, or repeated within the bundle (6.5-0019).
    SlotName(String),
    /// A slot dtype that is not a Classify dtype token (6.5-0019).
    UnknownDtype(String),
    /// A slot layout form that is `bundle`, `none` or `-` (6.5-0019: a form other than `bundle` and `none`).
    SlotLayoutForm,
}

impl fmt::Display for LayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "output layout: {self:?}")
    }
}

impl std::error::Error for LayoutError {}

/// A nested layout tuple: `tuple := integer | "(" tuple {"," tuple} ")"` (6.5-0017).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tuple {
    /// A leaf.
    Int(u64),
    /// A parenthesised, non-empty tuple.
    Tuple(Vec<Tuple>),
}

/// A swizzle `swz(B,M,S)` (CuTe `Swizzle<B,M,S>`: bits, base, shift; 6.5-0018).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Swizzle {
    /// Bits.
    pub bits: u64,
    /// Base.
    pub base: u64,
    /// Shift (signed).
    pub shift: i64,
}

/// One slot of a bundle: `name:dtype[e0,e1,…]=layout@offset` (6.5-0019).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slot {
    /// `[a-z0-9_]+`, unique within the bundle.
    pub name: String,
    /// The Classify dtype token.
    pub dtype: String,
    /// The shape: one Dispatch expression per axis, as written.
    pub shape: Vec<String>,
    /// The slot's layout: any form other than `bundle`, `none` and `-`.
    pub layout: Box<Layout>,
    /// The byte-offset Dispatch expression, as written.
    pub offset: String,
}

/// A layout form (6.5-0015). The set is closed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Layout {
    /// `-`: no guarantee, the layout the caller supplied (6.5-0014). Only an `output_layouts` element, never a slot layout.
    Supplied,
    /// `dense`: densely packed in canonical (row-major) order.
    Dense,
    /// `dense_order(p0,…)`: densely packed with axis `p0` slowest; `p` is a permutation of `0..rank`.
    DenseOrder(Vec<usize>),
    /// `same_as(i)`: the strides of input operand `i`.
    SameAs(usize),
    /// `strided(e0,…)`: one stride expression per axis, as written.
    Strided(Vec<String>),
    /// `shape:stride`, optionally under `swz(B,M,S)(…)`.
    Nested {
        /// The swizzle, when the form is `swz(B,M,S)(shape:stride)`.
        swizzle: Option<Swizzle>,
        /// The shape tuple.
        shape: Tuple,
        /// The stride tuple, congruent with the shape.
        stride: Tuple,
    },
    /// `bundle(slot,…)`: one output buffer partitioned into named slots.
    Bundle(Vec<Slot>),
    /// `none`: no guarantee, the reader reads the layout from the buffer.
    None,
}

/// What the layout parsers need to know about the contract (6.5-0016).
#[derive(Clone, Copy)]
pub struct Env<'a> {
    /// The contract's declared rank.
    pub rank: usize,
    /// The number of input operands (`same_as(i)` needs `i` below it).
    pub n_inputs: usize,
    /// The contract's vocabulary and rank for stride and slot expressions (see [`Context::of_interface`]).
    pub context: &'a Context,
    /// Whether a token is a Classify dtype (the table is Classify's; see the module docs).
    pub is_dtype: &'a dyn Fn(&str) -> bool,
}

// ---- helpers -------------------------------------------------------------------------------------------------------------

/// Split at every top-level `sep` (outside `()` and `[]`).
fn split_top(s: &str, sep: char) -> Vec<&str> {
    let (mut out, mut depth, mut from) = (Vec::new(), 0i32, 0usize);
    for (i, c) in s.char_indices() {
        match c {
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[from..i]);
                from = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[from..]);
    out
}

/// The first top-level occurrence of `sep`.
fn find_top(s: &str, sep: char) -> Option<usize> {
    let mut depth = 0i32;
    for (i, c) in s.char_indices() {
        match c {
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            c if c == sep && depth == 0 => return Some(i),
            _ => {}
        }
    }
    None
}

fn uint(s: &str) -> Result<u64, LayoutError> {
    let bad = || LayoutError::NonCanonicalInteger(s.to_owned());
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) || (s.len() > 1 && s.starts_with('0'))
    {
        return Err(bad());
    }
    s.parse().map_err(|_| bad())
}

fn int(s: &str) -> Result<i64, LayoutError> {
    let bad = || LayoutError::NonCanonicalInteger(s.to_owned());
    match s.strip_prefix('-') {
        Some(rest) => {
            let v = uint(rest)?;
            if v == 0 {
                Err(bad())
            } else {
                i64::try_from(v).map(|v| -v).map_err(|_| bad())
            }
        }
        None => i64::try_from(uint(s)?).map_err(|_| bad()),
    }
}

fn args<'a>(s: &'a str, head: &str) -> Option<&'a str> {
    s.strip_prefix(head)?.strip_prefix('(')?.strip_suffix(')')
}

fn expr(s: &str, env: &Env<'_>) -> Result<String, LayoutError> {
    Expr::parse_in(s, env.context).map_err(LayoutError::BadExpression)?;
    Ok(s.to_owned())
}

fn tuple(s: &str, depth: usize) -> Result<Tuple, LayoutError> {
    if let Some(inner) = s.strip_prefix('(').and_then(|v| v.strip_suffix(')')) {
        if depth + 1 > MAX_NEST_DEPTH || inner.is_empty() {
            return Err(LayoutError::NestedShape);
        }
        let parts = split_top(inner, ',');
        return Ok(Tuple::Tuple(
            parts
                .into_iter()
                .map(|p| tuple(p, depth + 1))
                .collect::<Result<_, _>>()?,
        ));
    }
    // A negative leaf with a canonical magnitude is a negative stride (or a shape below 1): a leaf fault, not a syntax one.
    if s.strip_prefix('-').is_some_and(|r| uint(r).is_ok()) {
        return Err(LayoutError::NestedLeaf);
    }
    uint(s)
        .map(Tuple::Int)
        .map_err(|_| LayoutError::NestedSyntax)
}

fn congruent(a: &Tuple, b: &Tuple) -> bool {
    match (a, b) {
        (Tuple::Int(_), Tuple::Int(_)) => true,
        (Tuple::Tuple(x), Tuple::Tuple(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(p, q)| congruent(p, q))
        }
        _ => false,
    }
}

fn leaves_ok(shape: &Tuple, stride: &Tuple) -> bool {
    match (shape, stride) {
        (Tuple::Int(s), Tuple::Int(_)) => *s >= 1,
        (Tuple::Tuple(x), Tuple::Tuple(y)) => x.iter().zip(y).all(|(p, q)| leaves_ok(p, q)),
        _ => false,
    }
}

fn nested(text: &str, rank: usize) -> Result<Layout, LayoutError> {
    let (swizzle, body) = if let Some(rest) = text.strip_prefix("swz(") {
        let close = rest.find(')').ok_or(LayoutError::NestedSyntax)?;
        let parts: Vec<&str> = rest[..close].split(',').collect();
        let [b, m, s] = parts[..] else {
            return Err(LayoutError::NestedSyntax);
        };
        let inner = rest[close + 1..]
            .strip_prefix('(')
            .and_then(|v| v.strip_suffix(')'))
            .ok_or(LayoutError::NestedSyntax)?;
        (
            Some(Swizzle {
                bits: uint(b)?,
                base: uint(m)?,
                shift: int(s)?,
            }),
            inner,
        )
    } else {
        (None, text)
    };
    let sides = split_top(body, ':');
    let [shape, stride] = sides[..] else {
        return Err(LayoutError::NestedSyntax);
    };
    let (shape, stride) = (tuple(shape, 0)?, tuple(stride, 0)?);
    if !congruent(&shape, &stride) {
        return Err(LayoutError::NestedNotCongruent);
    }
    if !leaves_ok(&shape, &stride) {
        return Err(LayoutError::NestedLeaf);
    }
    let modes = match &shape {
        Tuple::Int(_) => 1,
        Tuple::Tuple(v) => v.len(),
    };
    if modes != rank {
        return Err(LayoutError::NestedRank {
            want: rank,
            got: modes,
        });
    }
    Ok(Layout::Nested {
        swizzle,
        shape,
        stride,
    })
}

fn slot(text: &str, env: &Env<'_>, seen: &mut Vec<String>) -> Result<Slot, LayoutError> {
    let bad = LayoutError::BundleSyntax;
    let (name, rest) = text.split_once(':').ok_or(bad.clone())?;
    if name.is_empty()
        || !name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
        || seen.iter().any(|n| n == name)
    {
        return Err(LayoutError::SlotName(name.to_owned()));
    }
    let open = rest.find('[').ok_or(bad.clone())?;
    let dtype = &rest[..open];
    if !(env.is_dtype)(dtype) {
        return Err(LayoutError::UnknownDtype(dtype.to_owned()));
    }
    let after = &rest[open..];
    let close = {
        let mut depth = 0i32;
        let mut at = None;
        for (i, c) in after.char_indices() {
            match c {
                '[' | '(' => depth += 1,
                ']' | ')' => {
                    depth -= 1;
                    if depth == 0 {
                        at = Some(i);
                        break;
                    }
                }
                _ => {}
            }
        }
        at.ok_or(bad.clone())?
    };
    let shape_text = &after[1..close];
    let shape: Vec<String> = if shape_text.is_empty() {
        Vec::new()
    } else {
        split_top(shape_text, ',')
            .into_iter()
            .map(|e| expr(e, env))
            .collect::<Result<_, _>>()?
    };
    let tail = after[close + 1..].strip_prefix('=').ok_or(bad.clone())?;
    let at = find_top(tail, '@').ok_or(bad)?;
    let (layout_text, offset) = (&tail[..at], &tail[at + 1..]);
    if layout_text == "-" || layout_text == "none" || layout_text.starts_with("bundle(") {
        return Err(LayoutError::SlotLayoutForm);
    }
    let layout = parse_layout_form(layout_text, env, shape.len(), false)?;
    let offset = expr(offset, env)?;
    seen.push(name.to_owned());
    Ok(Slot {
        name: name.to_owned(),
        dtype: dtype.to_owned(),
        shape,
        layout: Box::new(layout),
        offset,
    })
}

fn parse_layout_form(
    text: &str,
    env: &Env<'_>,
    rank: usize,
    top: bool,
) -> Result<Layout, LayoutError> {
    if text.contains(' ') {
        return Err(LayoutError::HasSpace);
    }
    match text {
        "-" if top => return Ok(Layout::Supplied),
        "dense" => return Ok(Layout::Dense),
        "none" => return Ok(Layout::None),
        _ => {}
    }
    if let Some(a) = args(text, "dense_order") {
        let p: Vec<usize> = a
            .split(',')
            .map(|x| uint(x).map(|v| v as usize))
            .collect::<Result<_, _>>()?;
        let mut sorted = p.clone();
        sorted.sort_unstable();
        if p.len() != rank || sorted != (0..rank).collect::<Vec<_>>() {
            return Err(LayoutError::NotAPermutation);
        }
        return Ok(Layout::DenseOrder(p));
    }
    if let Some(a) = args(text, "same_as") {
        let i = uint(a)? as usize;
        return if i < env.n_inputs {
            Ok(Layout::SameAs(i))
        } else {
            Err(LayoutError::SameAsOutOfRange(i))
        };
    }
    if let Some(a) = args(text, "strided") {
        let parts = split_top(a, ',');
        if parts.len() != rank {
            return Err(LayoutError::StridedRank {
                want: rank,
                got: parts.len(),
            });
        }
        return Ok(Layout::Strided(
            parts
                .into_iter()
                .map(|e| expr(e, env))
                .collect::<Result<_, _>>()?,
        ));
    }
    if let Some(a) = args(text, "bundle") {
        if !top {
            return Err(LayoutError::SlotLayoutForm);
        }
        let mut seen = Vec::new();
        let slots = split_top(a, ',')
            .into_iter()
            .map(|s| slot(s, env, &mut seen))
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(Layout::Bundle(slots));
    }
    if text.starts_with("swz(")
        || text.starts_with('(')
        || text.bytes().next().is_some_and(|b| b.is_ascii_digit())
    {
        return nested(text, rank);
    }
    Err(LayoutError::UnknownForm(
        text.split('(').next().unwrap_or(text).to_owned(),
    ))
}

/// Parse one layout form of an `output_layouts` element (6.5-0015, -0016, -0017, -0018, -0019) for a contract of rank `env.rank`.
///
/// # Errors
/// A [`LayoutError`]; a head that is none of the seven is [`LayoutError::UnknownForm`].
pub fn parse_layout(text: &str, env: &Env<'_>) -> Result<Layout, LayoutError> {
    parse_layout_form(text, env, env.rank, true)
}

/// One output operand's guarantee: the layout, and the refused form when the element's head was unknown (6.5-0015).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputGuarantee {
    /// The layout; [`Layout::None`] when the form was refused.
    pub layout: Layout,
    /// The unknown head, recorded when the guarantee was refused.
    pub refused: Option<String>,
}

/// Parse the value of an `output_layouts` field (6.5-0014): `[` elements separated by `, ` `]`, exactly `n_outputs` of them. An **unknown
/// form** refuses only that output's guarantee (it is read as `none` and the head recorded in [`OutputGuarantee::refused`]); the contract is
/// not refused and the form is not accepted silently (6.5-0015). Any other fault declines the field.
///
/// # Errors
/// A [`LayoutError`], including [`LayoutError::OutputCount`] for a count mismatch (no padding, no truncating).
pub fn parse_output_layouts(
    value: &str,
    n_outputs: usize,
    env: &Env<'_>,
) -> Result<Vec<OutputGuarantee>, LayoutError> {
    let inner = value
        .strip_prefix('[')
        .and_then(|v| v.strip_suffix(']'))
        .ok_or(LayoutError::NotAnArray)?;
    let elements: Vec<&str> = if inner.is_empty() {
        Vec::new()
    } else {
        split_top(inner, ',')
    };
    // The array separator is `, `: split on the top-level comma, then every element but the first starts with the one space.
    let elements: Vec<&str> = elements
        .iter()
        .enumerate()
        .map(|(i, e)| {
            if i == 0 {
                Ok(*e)
            } else {
                e.strip_prefix(' ').ok_or(LayoutError::NotAnArray)
            }
        })
        .collect::<Result<_, _>>()?;
    if elements.len() != n_outputs {
        return Err(LayoutError::OutputCount {
            want: n_outputs,
            got: elements.len(),
        });
    }
    elements
        .into_iter()
        .map(|e| match parse_layout(e, env) {
            Ok(layout) => Ok(OutputGuarantee {
                layout,
                refused: None,
            }),
            Err(LayoutError::UnknownForm(head)) => Ok(OutputGuarantee {
                layout: Layout::None,
                refused: Some(head),
            }),
            Err(e) => Err(e),
        })
        .collect()
}

// ---- canonical text --------------------------------------------------------------------------------------------------------

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(v) => write!(f, "{v}"),
            Self::Tuple(v) => {
                write!(f, "(")?;
                for (i, t) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{t}")?;
                }
                write!(f, ")")
            }
        }
    }
}

impl fmt::Display for Layout {
    /// The canonical text of the form (6.5-0016): no space.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Supplied => write!(f, "-"),
            Self::Dense => write!(f, "dense"),
            Self::None => write!(f, "none"),
            Self::DenseOrder(p) => write!(
                f,
                "dense_order({})",
                p.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::SameAs(i) => write!(f, "same_as({i})"),
            Self::Strided(e) => write!(f, "strided({})", e.join(",")),
            Self::Nested {
                swizzle: None,
                shape,
                stride,
            } => write!(f, "{shape}:{stride}"),
            Self::Nested {
                swizzle: Some(s),
                shape,
                stride,
            } => write!(
                f,
                "swz({},{},{})({shape}:{stride})",
                s.bits, s.base, s.shift
            ),
            Self::Bundle(slots) => {
                write!(f, "bundle(")?;
                for (i, s) in slots.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(
                        f,
                        "{}:{}[{}]={}@{}",
                        s.name,
                        s.dtype,
                        s.shape.join(","),
                        s.layout,
                        s.offset
                    )?;
                }
                write!(f, ")")
            }
        }
    }
}
