// SPDX-License-Identifier: MIT OR Apache-2.0
//! **The Semantics `op_dag` reader** (KISS-CONTRACT 6.4-0002, -0009, 6.11-0007): the `op_dag` field value, an array of nodes in the Grammar
//! canonical order (every child strictly earlier than its parent, the root last), each one of
//! `Op{<op_name>; <op_attrs>; <child_edges>}`, `Bind(<index>)` or `Source{<tag>; <source_attrs>}`.
//!
//! What it checks is the wire form: the array and node shapes, a bare op name, `key=value` attribute lists, `child_edges` as `[i, ...]` of
//! **strictly earlier** indices, and the closed `Source` tag set with each tag's attribute keys. What it does **not** do: decide whether an
//! op name is a KISS-Ops op (that is the op-facts source's `UnknownOp`, [`crate::op_facts`]), check a `const`'s bit width against its dtype,
//! or resolve a non-primitive node to the primitive floor (6.4-0005, 6.4-0010). **Draft-tracking.**

use core::fmt;

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
    /// The contract's Semantics has no readable `op_dag` field, or `op_identity` and the DAG root disagree (6.4-0002).
    RootMismatch,
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
        let bare = !name.is_empty() && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_');
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
        return bare
            .then_some(Node::Op {
                op_name: name,
                op_attrs,
                child_edges,
            })
            .ok_or(bad);
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

/// Read the Semantics `op_dag` of `contract` and check that its root is the op `op_identity` names (6.4-0002). An empty array has no root to
/// compare, so it passes (the `declared-op-tag` case).
///
/// # Errors
/// [`OpDagError`]: a parse fault, or [`OpDagError::RootMismatch`].
pub fn read_op_dag<'a>(contract: &Contract<'a>) -> Result<OpDag<'a>, OpDagError> {
    let dag = parse_op_dag(contract.get(1, "op_dag").ok_or(OpDagError::RootMismatch)?)?;
    if let Some(root) = dag.root_op() {
        let identity = contract.get(0, "op_identity").unwrap_or_default();
        if identity_op_name(identity) != root {
            return Err(OpDagError::RootMismatch);
        }
    }
    Ok(dag)
}

/// Why [`check_contract_cell_class`] declined.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ContractCellClassError {
    /// The Semantics `op_dag` could not be read, or its root is not the identity's op.
    OpDag(OpDagError),
    /// The cell class is not supported by the DAG's ops (6.3-0006).
    CellClass(CellClassError),
}

/// 6.3-0006 over a contract: the cell class of its `accept_predicate` against every `Op` node of its Semantics `op_dag`. A contract whose DAG
/// carries no node (`declared-op-tag`) is not rejected on this assertion.
///
/// # Errors
/// [`ContractCellClassError`].
pub fn check_contract_cell_class(
    contract: &Contract<'_>,
    source: &dyn OpFactSource,
) -> Result<(), ContractCellClassError> {
    let dag = read_op_dag(contract).map_err(ContractCellClassError::OpDag)?;
    let ops = dag.ops();
    if ops.is_empty() {
        return Ok(());
    }
    let accept = contract.get(0, "accept_predicate").unwrap_or_default();
    check_cell_class_supported(accept, &ops, source).map_err(ContractCellClassError::CellClass)
}
