// SPDX-License-Identifier: MIT OR Apache-2.0
//! **The Semantics `op_dag` reader** (KISS-CONTRACT 6.4-0002, -0009, 6.11-0007): the `op_dag` field value, an array of nodes in the Grammar
//! canonical order (every child strictly earlier than its parent, the root last), each one of
//! `Op{<op_name>; <op_attrs>; <child_edges>}`, `Bind(<index>)` or `Source{<tag>; <source_attrs>}`.
//!
//! [`parse_op_dag`] reads the wire form: the array and node shapes, `key=value` attribute lists, `child_edges` as `[i, ...]` of **strictly
//! earlier** indices, the closed `Source` tag set with each tag's attribute keys and an upper-case-hex `const`. [`check_op_dag`] then asks the
//! two sources the caller supplies (no table is kept here): every `Op` name is a KISS-Ops op ([`OpFactSource`]), a `const`'s `dtype` is a dtype
//! token and its `bits` are zero-padded to that width ([`DtypeSource`]), the DAG is non-empty and has one root, the last node. [`check_semantics`]
//! is the whole Semantics assertion over a contract: `semantics_kind`, the DAG, the root against `op_identity` (6.4-0002) and 6.3-0006.
//! The rulings are KISS#829's: a `machine-checkable-IR` DAG has at least one node; a `declared-op-tag` one may be empty and is then not rejected
//! on 6.3-0006; the root is compared to `op_identity` by bare op name. Not done: resolving non-primitive nodes to the primitive floor (6.4-0005,
//! 6.4-0010). The vectors are KISS's `op_dag_vectors` (`tests/op_dag_vectors.rs`). **Draft-tracking.**

use core::fmt;

use crate::dtypes::DtypeSource;
use crate::fields::Contract;
use crate::op_facts::{CellClassError, OpFactSource, check_cell_class_supported};

/// The six non-input scalar-source tags (KISS-GRAMMAR-6.4-0013, 6.11-0007).
pub const SOURCE_TAGS: [&str; 6] = [
    "const",
    "param",
    "coord",
    "reduced",
    "extent",
    "reduced_count",
];

/// One `op_dag` node (6.4-0009).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node<'a> {
    /// `Op{op_name; op_attrs; child_edges}`.
    Op {
        /// The bare KISS-Ops op name.
        op_name: &'a str,
        /// The `key=value` attributes, in written order (values stay text).
        op_attrs: Vec<(&'a str, &'a str)>,
        /// The operand children, as strictly-earlier node indices, in operand order.
        child_edges: Vec<usize>,
    },
    /// `Bind(index)`: the positional operand it binds.
    Bind(u32),
    /// `Source{tag; source_attrs}`.
    Source {
        /// One of [`SOURCE_TAGS`].
        tag: &'a str,
        /// The `key=value` attributes, in written order.
        source_attrs: Vec<(&'a str, &'a str)>,
    },
}

/// A parsed `op_dag`: its nodes in wire order, the root last.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OpDag<'a> {
    /// The nodes in the order written.
    pub nodes: Vec<Node<'a>>,
}

impl<'a> OpDag<'a> {
    /// The root node (the last), or `None` for an empty array (a `declared-op-tag` contract may carry no node).
    #[must_use]
    pub fn root(&self) -> Option<&Node<'a>> {
        self.nodes.last()
    }

    /// The op name of the root, or `None` if the array is empty or its root is not an `Op`.
    #[must_use]
    pub fn root_op(&self) -> Option<&'a str> {
        match self.root()? {
            Node::Op { op_name, .. } => Some(op_name),
            _ => None,
        }
    }

    /// Every `Op` node's name, in node order (what 6.3-0006 asks for over a fusion's DAG).
    #[must_use]
    pub fn ops(&self) -> Vec<&'a str> {
        self.nodes
            .iter()
            .filter_map(|n| match n {
                Node::Op { op_name, .. } => Some(*op_name),
                _ => None,
            })
            .collect()
    }
}

/// An `op_dag` that cannot be read. Each is a typed decline (6.11-0007, 6.4-0009); never a repair.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum OpDagError {
    /// The value is not `[` ... `]` with elements separated by `, `.
    NotAnArray,
    /// Node `index` (0-based) is none of `Op{..}`, `Bind(..)`, `Source{..}`, or is malformed inside.
    BadNode {
        /// The node's position in the array.
        index: usize,
    },
    /// An `Op` node's `child_edges` names index `child`, which is not strictly earlier than the node `index` (a later or self reference).
    ChildNotEarlier {
        /// The node's position.
        index: usize,
        /// The offending child index.
        child: usize,
    },
    /// A `Source` node's tag is not in [`SOURCE_TAGS`], or its attributes are not the ones the tag pins.
    BadSource {
        /// The node's position.
        index: usize,
    },
    /// An `Op` node's name is not a KISS-Ops op name (including a name with attributes or a Grammar tag spelled inside it).
    UnknownOp {
        /// The node's position.
        index: usize,
    },
    /// A node other than the last that no later node uses: a second root, so the root is not last.
    MultipleRoots {
        /// The unused node's position.
        index: usize,
    },
    /// A `machine-checkable-IR` Semantics with no node: an empty array has no root (6.4-0007).
    Empty,
    /// `semantics_kind` is neither `machine-checkable-IR` nor `declared-op-tag`, or the contract has no Semantics fields.
    BadSemanticsKind,
    /// The DAG root is not an `Op` named by `op_identity` (6.4-0002).
    RootMismatch,
}

impl OpDagError {
    /// The stable kebab-case category tag KISS's vectors use.
    #[must_use]
    pub fn wire_tag(&self) -> &'static str {
        match self {
            Self::NotAnArray | Self::BadNode { .. } => "op-dag-malformed",
            Self::ChildNotEarlier { .. } => "op-dag-child-not-earlier",
            Self::BadSource { .. } => "op-dag-bad-source",
            Self::UnknownOp { .. } => "op-dag-unknown-op",
            Self::MultipleRoots { .. } => "op-dag-multiple-roots",
            Self::Empty => "op-dag-empty",
            Self::BadSemanticsKind => "bad-semantics-kind",
            Self::RootMismatch => "op-identity-root-mismatch",
        }
    }

    /// Where the reader declines: `record` for the cross-field assertion (root against identity), else `semantics`.
    #[must_use]
    pub fn section(&self) -> &'static str {
        if matches!(self, Self::RootMismatch) {
            "record"
        } else {
            "semantics"
        }
    }
}

impl fmt::Display for OpDagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "op_dag: {self:?}")
    }
}

impl std::error::Error for OpDagError {}

/// Split `s` at every `sep` that sits outside `[]`, `{}`, `()` nesting.
fn split_top<'a>(s: &'a str, sep: &str) -> Option<Vec<&'a str>> {
    let (mut depth, mut start) = (0usize, 0usize);
    let mut out = Vec::new();
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'[' | b'{' | b'(' => depth += 1,
            b']' | b'}' | b')' => depth = depth.checked_sub(1)?,
            _ if depth == 0 && s[i..].starts_with(sep) => {
                out.push(&s[start..i]);
                i += sep.len();
                start = i;
                continue;
            }
            _ => {}
        }
        i += 1;
    }
    (depth == 0).then(|| {
        out.push(&s[start..]);
        out
    })
}

/// `key=value, key=value` (empty list is the empty string). Keys are non-empty and unique, with no whitespace or structure characters.
fn attrs(s: &str) -> Option<Vec<(&str, &str)>> {
    if s.is_empty() {
        return Some(Vec::new());
    }
    let mut out: Vec<(&str, &str)> = Vec::new();
    for item in split_top(s, ", ")? {
        let (k, v) = item.split_once('=')?;
        if k.is_empty()
            || k.chars()
                .any(|c| c.is_whitespace() || "[]{}()=;,".contains(c))
            || v.is_empty()
            || out.iter().any(|(e, _)| *e == k)
        {
            return None;
        }
        out.push((k, v));
    }
    Some(out)
}

fn decimal<T: core::str::FromStr>(s: &str) -> Option<T> {
    (!s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) && (s == "0" || !s.starts_with('0')))
        .then(|| s.parse().ok())?
}

fn source_ok(tag: &str, a: &[(&str, &str)]) -> bool {
    let one = |key: &str| a.len() == 1 && a[0].0 == key;
    match tag {
        "const" => {
            a.len() == 2
                && a[0].0 == "dtype"
                && !a[0].1.is_empty()
                && a[1].0 == "bits"
                && a[1].1.strip_prefix("0x").is_some_and(|h| {
                    !h.is_empty()
                        && h.len() % 2 == 0
                        && h.bytes()
                            .all(|b| b.is_ascii_digit() || (b'A'..=b'F').contains(&b))
                })
        }
        "param" => one("index") && decimal::<u32>(a[0].1).is_some(),
        "coord" | "extent" => one("axis") && decimal::<u8>(a[0].1).is_some(),
        "reduced" => one("stage") && decimal::<u32>(a[0].1).is_some(),
        "reduced_count" => a.is_empty(),
        _ => false,
    }
}

fn node(index: usize, text: &str) -> Result<Node<'_>, OpDagError> {
    let bad = OpDagError::BadNode { index };
    if let Some(n) = text.strip_prefix("Bind(").and_then(|r| r.strip_suffix(')')) {
        return decimal(n).map(Node::Bind).ok_or(bad);
    }
    if let Some(body) = text.strip_prefix("Op{").and_then(|r| r.strip_suffix('}')) {
        let parts = split_top(body, "; ").ok_or_else(|| bad.clone())?;
        let [name, a, edges] = parts[..] else {
            return Err(bad);
        };
        let op_attrs = attrs(a).ok_or_else(|| bad.clone())?;
        let list = edges
            .strip_prefix('[')
            .and_then(|r| r.strip_suffix(']'))
            .ok_or_else(|| bad.clone())?;
        let mut child_edges = Vec::new();
        if !list.is_empty() {
            for c in list.split(", ") {
                let c: usize = decimal(c).ok_or_else(|| bad.clone())?;
                if c >= index {
                    return Err(OpDagError::ChildNotEarlier { index, child: c });
                }
                child_edges.push(c);
            }
        }
        return Ok(Node::Op {
            op_name: name,
            op_attrs,
            child_edges,
        });
    }
    if let Some(body) = text
        .strip_prefix("Source{")
        .and_then(|r| r.strip_suffix('}'))
    {
        let (tag, a) = body.split_once("; ").ok_or_else(|| bad.clone())?;
        let source_attrs = attrs(a).ok_or_else(|| bad.clone())?;
        return if source_ok(tag, &source_attrs) {
            Ok(Node::Source { tag, source_attrs })
        } else {
            Err(OpDagError::BadSource { index })
        };
    }
    Err(bad)
}

/// Read an `op_dag` field value (6.11-0007).
///
/// # Errors
/// An [`OpDagError`] naming the first faulty node.
pub fn parse_op_dag(value: &str) -> Result<OpDag<'_>, OpDagError> {
    let inner = value
        .strip_prefix('[')
        .and_then(|r| r.strip_suffix(']'))
        .ok_or(OpDagError::NotAnArray)?;
    if inner.is_empty() {
        return Ok(OpDag::default());
    }
    let items = split_top(inner, ", ").ok_or(OpDagError::NotAnArray)?;
    let nodes = items
        .into_iter()
        .enumerate()
        .map(|(i, t)| node(i, t))
        .collect::<Result<_, _>>()?;
    Ok(OpDag { nodes })
}

/// The root op name an `op_identity` value names: `<op_name>[{...}][[...]]` (6.11-0009), cut at the first `{` or `[`.
#[must_use]
pub fn identity_op_name(op_identity: &str) -> &str {
    op_identity.split(['{', '[']).next().unwrap_or_default()
}

/// The structure of a parsed `op_dag` against the two sources: non-empty; every `Op` name a KISS-Ops op; every `const`'s `dtype` a dtype token
/// with `bits` zero-padded to `2 * ceil(width_bits / 8)` digits; every node but the last some later node's child (the root is last). Children
/// strictly earlier is already [`parse_op_dag`]'s.
///
/// # Errors
/// [`OpDagError`] naming the first fault in node order.
pub fn check_op_dag(
    dag: &OpDag<'_>,
    ops: &dyn OpFactSource,
    dtypes: &dyn DtypeSource,
) -> Result<(), OpDagError> {
    if dag.nodes.is_empty() {
        return Err(OpDagError::Empty);
    }
    let mut used = vec![false; dag.nodes.len()];
    for (index, node) in dag.nodes.iter().enumerate() {
        match node {
            Node::Op {
                op_name,
                child_edges,
                ..
            } => {
                if ops.facts(op_name).is_none() {
                    return Err(OpDagError::UnknownOp { index });
                }
                for &c in child_edges {
                    used[c] = true;
                }
            }
            Node::Source {
                tag: "const",
                source_attrs,
            } => {
                let width = dtypes.storage_bits(source_attrs[0].1);
                let hex = source_attrs[1].1.strip_prefix("0x").unwrap_or_default();
                let digits = width.map(|w| 2 * w.div_ceil(8));
                if digits != u64::try_from(hex.len()).ok() {
                    return Err(OpDagError::BadSource { index });
                }
            }
            _ => {}
        }
    }
    let last = dag.nodes.len() - 1;
    match used[..last].iter().position(|u| !u) {
        Some(index) => Err(OpDagError::MultipleRoots { index }),
        None => Ok(()),
    }
}

/// Why [`check_semantics`] declined.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SemanticsError {
    /// The Semantics `op_dag` (or `semantics_kind`, or its root against `op_identity`) is faulty.
    OpDag(OpDagError),
    /// The cell class is not supported by the DAG's ops (6.3-0006).
    CellClass(CellClassError),
}

impl SemanticsError {
    /// The stable kebab-case category tag KISS's vectors use.
    #[must_use]
    pub fn wire_tag(&self) -> &'static str {
        match self {
            Self::OpDag(e) => e.wire_tag(),
            Self::CellClass(_) => "cell-class-unsupported-by-op",
        }
    }

    /// Where the reader declines: `semantics` or `record`.
    #[must_use]
    pub fn section(&self) -> &'static str {
        match self {
            Self::OpDag(e) => e.section(),
            Self::CellClass(_) => "record",
        }
    }
}

impl fmt::Display for SemanticsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "semantics: {self:?}")
    }
}

impl std::error::Error for SemanticsError {}

/// The whole Semantics assertion over a contract (6.4-0001, -0002, -0007, 6.3-0006): `semantics_kind` is one of the two values; the `op_dag` reads
/// and checks ([`parse_op_dag`], [`check_op_dag`]); its root is an `Op` named by `op_identity`; and the cell class of the `accept_predicate` is
/// met by some `Op` node. A `declared-op-tag` contract with an empty `op_dag` is returned as is (not rejected on 6.3-0006).
///
/// # Errors
/// [`SemanticsError`], the first fault in that order.
pub fn check_semantics<'a>(
    contract: &Contract<'a>,
    ops: &dyn OpFactSource,
    dtypes: &dyn DtypeSource,
) -> Result<OpDag<'a>, SemanticsError> {
    let bad = |e| SemanticsError::OpDag(e);
    let declared = match contract.get(1, "semantics_kind") {
        Some("machine-checkable-IR") => false,
        Some("declared-op-tag") => true,
        _ => return Err(bad(OpDagError::BadSemanticsKind)),
    };
    let dag = parse_op_dag(
        contract
            .get(1, "op_dag")
            .ok_or(bad(OpDagError::BadSemanticsKind))?,
    )
    .map_err(bad)?;
    if declared && dag.nodes.is_empty() {
        return Ok(dag);
    }
    check_op_dag(&dag, ops, dtypes).map_err(bad)?;
    let identity = contract.get(0, "op_identity").unwrap_or_default();
    if dag.root_op() != Some(identity_op_name(identity)) {
        return Err(bad(OpDagError::RootMismatch));
    }
    let accept = contract.get(0, "accept_predicate").unwrap_or_default();
    check_cell_class_supported(accept, &dag.ops(), ops).map_err(SemanticsError::CellClass)?;
    Ok(dag)
}
