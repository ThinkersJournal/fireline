// SPDX-License-Identifier: MIT OR Apache-2.0
//! **The cell-class-supported-by-op assertion** (KISS-CONTRACT 6.3-0006), decided from structural facts the named op *declares*, which KISS-Ops
//! owns (6.13-0009 a contraction, 6.11-0012 the index-operand role, 6.19-0036 a reduce axis). **This crate carries no per-op table**: the facts
//! come from an [`OpFactSource`] the caller supplies, so one source of meaning (KISS-Ops) stays the only copy.
//!
//! **Skeleton, provisional.** KISS ships those facts today only as prose in `spec/ops.md`; a machine-readable op-facts sidecar has been asked
//! of kiss (PM, 2026-11-15). Until it exists, [`FactTable`] reads a minimal TSV (`op<TAB>contraction<TAB>index_role<TAB>reduce_axis`, each `0`
//! or `1`) used only by this module's tests. When kiss's sidecar lands, replace [`FactTable::parse_tsv`] with its loader; [`OpFacts`],
//! [`OpFactSource`] and [`check_cell_class_supported`] stay.
//!
//! The rule (6.3-0006), in exactly this sense and no other: class `ctr` needs a contraction op, `ndx` an op that declares the index-operand
//! role, `rdc` an op that declares a reduce axis; `ew1`, `ew2`, `ew3` and `oth` impose nothing (they are decided by operand shape alone). For a
//! fusion the requirement is met if **any** op of the Semantics DAG has the fact, so the caller passes every op of the DAG (a plain op: just
//! the root). The relation is one-directional: a matmul keyed `oth` is not seen here.
//!
//! **Not here:** reading the Semantics `op_dag` into its op names (6.4-0002, not built), the `op_identity` forms (6.3-0003, -0004), and the
//! `declared-op-tag` exemption, which is the caller's: it does not call this for a fused region whose Semantics carries no node. **Draft-tracking.**

use core::fmt;
use std::collections::BTreeMap;

/// The structural facts one op declares (KISS-OPS 6.13-0009, 6.11-0012, 6.19-0036).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OpFacts {
    /// The op declares an M/N/K iteration space (a contraction).
    pub contraction: bool,
    /// The op declares the index-operand role (a gather/scatter/index op).
    pub index_role: bool,
    /// The op declares a reduce axis (carries `reduce_axes`, or resolves its axis from its inner carrier).
    pub reduce_axis: bool,
}

/// Where the facts of an op come from. Implemented over KISS-Ops' machine-readable data, never over a table kept here.
pub trait OpFactSource {
    /// The facts of the op named `op` (a bare KISS-Ops op name), or `None` if the source does not know it.
    fn facts(&self, op: &str) -> Option<OpFacts>;
}

/// A cell class the named op(s) cannot support, or an input that cannot be read. Each is a typed decline; never a repair.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CellClassError {
    /// The `accept_predicate` is not `sk<digits>|<class>|...` with a non-empty class component.
    BadStructureKey,
    /// No op was named (an empty op list).
    NoOps,
    /// An op the source does not know, so its facts cannot be decided.
    UnknownOp(String),
    /// The class needs a fact no named op declares (6.3-0006).
    Unsupported {
        /// The cell class (`ctr`, `ndx` or `rdc`).
        class: String,
        /// The fact it requires: `contraction`, `index-role` or `reduce-axis`.
        requires: &'static str,
    },
}

impl fmt::Display for CellClassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cell class vs op: {self:?}")
    }
}

impl std::error::Error for CellClassError {}

/// The cell class of an `accept_predicate` `structure_key`: its second `|`-separated component (`sk8|ew2|f32|...`), or `None` if the token is
/// not `sk<digits>|<class>|...`.
#[must_use]
pub fn cell_class(accept_predicate: &str) -> Option<&str> {
    let mut parts = accept_predicate.split('|');
    let version = parts.next()?;
    let class = parts.next()?;
    let versioned = version
        .strip_prefix("sk")
        .is_some_and(|d| !d.is_empty() && d.bytes().all(|b| b.is_ascii_digit()));
    (versioned && !class.is_empty()).then_some(class)
}

/// Check that the cell class of `accept_predicate` is supported by the named `ops` (the root op, or every op of a fusion's Semantics DAG),
/// per 6.3-0006.
///
/// # Errors
/// [`CellClassError::BadStructureKey`], [`CellClassError::NoOps`], [`CellClassError::UnknownOp`] for an op `source` does not know (only
/// consulted when the class needs a fact), or [`CellClassError::Unsupported`].
pub fn check_cell_class_supported(
    accept_predicate: &str,
    ops: &[&str],
    source: &dyn OpFactSource,
) -> Result<(), CellClassError> {
    let class = cell_class(accept_predicate).ok_or(CellClassError::BadStructureKey)?;
    if ops.is_empty() {
        return Err(CellClassError::NoOps);
    }
    let (requires, has): (&'static str, fn(&OpFacts) -> bool) = match class {
        "ctr" => ("contraction", |f| f.contraction),
        "ndx" => ("index-role", |f| f.index_role),
        "rdc" => ("reduce-axis", |f| f.reduce_axis),
        _ => return Ok(()),
    };
    for op in ops {
        let facts = source
            .facts(op)
            .ok_or_else(|| CellClassError::UnknownOp((*op).to_owned()))?;
        if has(&facts) {
            return Ok(());
        }
    }
    Err(CellClassError::Unsupported {
        class: class.to_owned(),
        requires,
    })
}

/// A fact table read from a minimal TSV; see the module docs (provisional until kiss's op-facts sidecar exists).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FactTable(BTreeMap<String, OpFacts>);

impl FactTable {
    /// Parse `op<TAB>contraction<TAB>index_role<TAB>reduce_axis` lines (each flag `0` or `1`); `#` lines and blank lines are skipped. A
    /// malformed or duplicate line is refused with its 1-based line number.
    ///
    /// # Errors
    /// The line number of the first bad line.
    pub fn parse_tsv(text: &str) -> Result<Self, usize> {
        let mut map = BTreeMap::new();
        for (i, line) in text.lines().enumerate() {
            let line = line.trim_end_matches('\r');
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let cols: Vec<&str> = line.split('\t').collect();
            let flag = |s: &str| match s {
                "0" => Some(false),
                "1" => Some(true),
                _ => None,
            };
            let [op, c, x, r] = cols[..] else {
                return Err(i + 1);
            };
            let (Some(c), Some(x), Some(r)) = (flag(c), flag(x), flag(r)) else {
                return Err(i + 1);
            };
            let facts = OpFacts {
                contraction: c,
                index_role: x,
                reduce_axis: r,
            };
            if op.is_empty() || map.insert(op.to_owned(), facts).is_some() {
                return Err(i + 1);
            }
        }
        Ok(Self(map))
    }
}

impl OpFactSource for FactTable {
    fn facts(&self, op: &str) -> Option<OpFacts> {
        self.0.get(op).copied()
    }
}
