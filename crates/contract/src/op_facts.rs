// SPDX-License-Identifier: MIT OR Apache-2.0
//! **The cell-class-supported-by-op assertion** (KISS-CONTRACT 6.3-0006), decided from structural facts the named op *declares*, which KISS-Ops
//! owns (6.13-0009 a contraction, 6.11-0012 the index-operand role, 6.19-0036 a reduce axis). **This crate carries no per-op table**: the facts
//! come from an [`OpFactSource`] the caller supplies, so one source of meaning (KISS-Ops) stays the only copy.
//!
//! **Source.** KISS's generated sidecar `conformance/corpus/op_facts.json` (schema `kiss-op-facts-v1`, from `tools/kiss_op_facts.py` over
//! `spec/ops.md`, KISS#825/#827, 0.18.4); [`FactTable::from_json`] reads it (a small built-in JSON reader: this crate stays dependency-free). The
//! facts read are `is_contraction`, `index_operand_role` and `declares_reduce_axis` (a carrier's `reduce_axes`/`norm_axes`, or, for a non-carrier,
//! the axis resolved from its inner carrier, 6.19-0036). Note `matmul` declares a reduce axis, so `rdc` is met by it; the rule is "declares", not
//! "is only".

//! The rule (6.3-0006), in exactly this sense and no other: class `ctr` needs a contraction op, `ndx` an op that declares the index-operand
//! role, `rdc` an op that declares a reduce axis; `ew1`, `ew2`, `ew3` and `oth` impose nothing (they are decided by operand shape alone). For a
//! fusion the requirement is met if **any** op of the Semantics DAG has the fact, so the caller passes every op of the DAG (a plain op: just
//! the root). The relation is one-directional: a matmul keyed `oth` is not seen here.
//!
//! **Not here:** the `op_identity` text forms (6.3-0003, -0004: [`crate::op_identity`], structure only) and resolving non-primitive nodes (6.4-0005, [`crate::op_resolve`]). Reading the Semantics `op_dag` into op names is
//! [`crate::op_dag`], and [`crate::op_dag::check_semantics`] runs this rule over a contract, exempting a `declared-op-tag` contract whose
//! Semantics carries no node. **Draft-tracking.**

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

/// Whether `schema` is an `op_facts.json` schema this reader accepts: `kiss-op-facts-v1`, or `kiss-op-facts-v2` (KISS 476, `kernel_rules`
/// removed; the fact fields this crate reads are unchanged). Both are accepted until v1 is gone from KISS main.
#[must_use]
pub fn is_supported_schema(schema: &str) -> bool {
    matches!(schema, "kiss-op-facts-v1" | "kiss-op-facts-v2")
}

/// A fact table read from KISS's `op_facts.json`; see the module docs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FactTable(BTreeMap<String, OpFacts>);

/// `op_facts.json` could not be read. Typed; never a repair.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum FactsError {
    /// Not valid JSON (byte offset of the first problem).
    Json(usize),
    /// Valid JSON but not `kiss-op-facts-v1`: wrong `schema`, no `ops` object, or an op entry whose fact fields are missing or mistyped.
    Schema(String),
}

impl fmt::Display for FactsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "op facts: {self:?}")
    }
}

impl std::error::Error for FactsError {}

impl FactTable {
    /// Read `op_facts.json` (schema `kiss-op-facts-v1`).
    ///
    /// # Errors
    /// [`FactsError::Json`] for malformed JSON, [`FactsError::Schema`] for a document of another shape.
    pub fn from_json(text: &str) -> Result<Self, FactsError> {
        let root = crate::json::parse(text).map_err(FactsError::Json)?;
        let bad = |s: &str| FactsError::Schema(s.to_owned());
        if !root
            .get("schema")
            .and_then(crate::json::Value::as_str)
            .is_some_and(crate::op_facts::is_supported_schema)
        {
            return Err(bad("schema is not kiss-op-facts-v1 or -v2"));
        }
        let Some(crate::json::Value::Obj(ops)) = root.get("ops") else {
            return Err(bad("no ops object"));
        };
        let mut map = BTreeMap::new();
        for (name, v) in ops {
            let flag = |k: &str| match v.get(k) {
                Some(crate::json::Value::Bool(b)) => Ok(*b),
                _ => Err(FactsError::Schema(format!("{name}: {k} is not a boolean"))),
            };
            let reduce_axis = flag("declares_reduce_axis")?;
            map.insert(
                name.clone(),
                OpFacts {
                    contraction: flag("is_contraction")?,
                    index_role: flag("index_operand_role")?,
                    reduce_axis,
                },
            );
        }
        Ok(Self(map))
    }

    /// The number of ops the table knows.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// The names of the ops the table knows, in order.
    pub fn ops(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(String::as_str)
    }

    /// Whether the table knows no op.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl OpFactSource for FactTable {
    fn facts(&self, op: &str) -> Option<OpFacts> {
        self.0.get(op).copied()
    }
}
