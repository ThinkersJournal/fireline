// SPDX-License-Identifier: MIT OR Apache-2.0
//! **Resolving a non-primitive op to the primitive floor** (KISS-CONTRACT 6.4-0005) and the two distinct ways it can fail (6.4-0010), over KISS's
//! generated `conformance/corpus/op_decomposition.json` (schema `kiss-op-decomposition-v1`, from `tools/kiss_op_decomp.py` over `spec/ops.md`,
//! KISS#833). **No decomposition is kept in this crate**: [`DecompTable::from_json`] reads the caller's copy of the file.
//!
//! [`DecompTable::resolve`] answers [`Resolution::Resolved`] (every chain reaches the floor and every needed contract is available),
//! [`Resolution::Deferred`] (a contract the chain needs is unavailable: the smallest such op name; neither a decline nor an accept) or
//! [`Resolution::Decline`] (the name has no decomposition at all, i.e. is not a KISS-Ops op). A floor op needs no contract. [`DecompTable::walk`]
//! recomputes `(level, floor)` from the `direct` edges only, and [`DecompTable::check_recorded`] compares that to the table's own `level` and `floor`
//! columns, so a mis-generated table disagrees with the walk instead of being believed (the same stance as KISS's reference resolver).
//! [`DecompTable::resolve_dag`] applies it to every `Op` node of an [`crate::op_dag::OpDag`]; **that aggregation is this crate's reading, not a KISS
//! vector**: any decline wins, otherwise the smallest deferred op name, otherwise resolved. Not wired into `read_contract`: an unavailable
//! contract is not a decline, so it cannot be a `ReadError`. **Draft-tracking.**

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::op_dag::{Node, OpDag};

/// What a resolver answers (6.4-0010).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolution {
    /// Every chain reaches the primitive floor with every needed contract available.
    Resolved,
    /// A contract the chain needs is unavailable: the smallest such op name (6.4-0010 b).
    Deferred(String),
    /// No decomposition reaches the floor: the name is not a KISS-Ops op (6.4-0010 a).
    Decline,
}

/// `op_decomposition.json` could not be read, or disagrees with itself. Typed; never a repair.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DecompError {
    /// Not valid JSON (byte offset of the first problem).
    Json(usize),
    /// Valid JSON but not `kiss-op-decomposition-v1`, or an entry with a missing or mistyped field.
    Schema(String),
    /// An op's `direct` edge names an op the table does not have, or does not strictly lower the level (6.4-0005's termination guarantee).
    Unresolvable(String),
}

impl fmt::Display for DecompError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "op decomposition: {self:?}")
    }
}

impl std::error::Error for DecompError {}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Entry {
    primitive: bool,
    level: u64,
    direct: Vec<String>,
    floor: BTreeSet<String>,
}

/// The decomposition table read from KISS's `op_decomposition.json`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DecompTable(BTreeMap<String, Entry>);

impl DecompTable {
    /// Read `op_decomposition.json`. Every `direct` edge must name an op in the table and strictly lower the level, so a walk cannot cycle.
    ///
    /// # Errors
    /// [`DecompError::Json`], [`DecompError::Schema`], or [`DecompError::Unresolvable`] for a dangling or non-lowering edge.
    pub fn from_json(text: &str) -> Result<Self, DecompError> {
        let root = crate::json::parse(text).map_err(DecompError::Json)?;
        let bad = DecompError::Schema;
        if root.get("schema").and_then(crate::json::Value::as_str)
            != Some("kiss-op-decomposition-v1")
        {
            return Err(bad("schema is not kiss-op-decomposition-v1".into()));
        }
        let Some(crate::json::Value::Obj(ops)) = root.get("ops") else {
            return Err(bad("no ops object".into()));
        };
        let strings =
            |name: &str, v: &crate::json::Value, k: &str| -> Result<Vec<String>, DecompError> {
                let arr = v
                    .get(k)
                    .and_then(crate::json::Value::as_arr)
                    .ok_or_else(|| bad(format!("{name}: {k} is not an array")))?;
                arr.iter()
                    .map(|x| {
                        x.as_str()
                            .map(str::to_owned)
                            .ok_or_else(|| bad(format!("{name}: {k} holds a non-string")))
                    })
                    .collect()
            };
        let mut map = BTreeMap::new();
        for (name, v) in ops {
            let primitive = match v.get("kind").and_then(crate::json::Value::as_str) {
                Some("primitive") => true,
                Some("non_primitive") => false,
                _ => {
                    return Err(bad(format!(
                        "{name}: kind is neither primitive nor non_primitive"
                    )));
                }
            };
            let level = v
                .get("level")
                .and_then(crate::json::Value::as_u64)
                .ok_or_else(|| bad(format!("{name}: level is not an integer")))?;
            let direct = strings(name, v, "direct")?;
            let floor = strings(name, v, "floor")?.into_iter().collect();
            map.insert(
                name.clone(),
                Entry {
                    primitive,
                    level,
                    direct,
                    floor,
                },
            );
        }
        let table = Self(map);
        for (name, e) in &table.0 {
            for c in &e.direct {
                let Some(child) = table.0.get(c) else {
                    return Err(DecompError::Unresolvable(format!(
                        "{name} -> {c}: no such op"
                    )));
                };
                if child.level >= e.level {
                    return Err(DecompError::Unresolvable(format!(
                        "{name} (level {}) -> {c} (level {}): does not lower the level",
                        e.level, child.level
                    )));
                }
            }
        }
        Ok(table)
    }

    /// The number of ops the table knows.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the table knows no op.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Every op name in the table, sorted.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(String::as_str)
    }

    /// Walk `op`'s `direct` edges to the floor: `(level, floor ops)`, or `None` for a name outside the table. A floor op has level 0 and is its own
    /// floor; any other op is `1 +` its highest child level.
    #[must_use]
    pub fn walk(&self, op: &str) -> Option<(u64, BTreeSet<String>)> {
        let e = self.0.get(op)?;
        if e.primitive {
            return Some((0, BTreeSet::from([op.to_owned()])));
        }
        let (mut level, mut floor) = (0, BTreeSet::new());
        for c in &e.direct {
            let (l, f) = self.walk(c)?;
            level = level.max(l);
            floor.extend(f);
        }
        Some((level + 1, floor))
    }

    /// The first op whose recorded `level`/`floor` disagrees with [`Self::walk`], or `None` if every op agrees.
    #[must_use]
    pub fn check_recorded(&self) -> Option<&str> {
        self.0
            .iter()
            .find(|(name, e)| self.walk(name) != Some((e.level, e.floor.clone())))
            .map(|(name, _)| name.as_str())
    }

    fn needed<'a>(&'a self, op: &'a str, out: &mut BTreeSet<&'a str>) {
        let Some(e) = self.0.get(op) else { return };
        if e.primitive || !out.insert(op) {
            return;
        }
        for c in &e.direct {
            self.needed(c, out);
        }
    }

    /// Resolve `op` when the contracts of `unavailable` cannot be acquired (a floor op needs no contract; 6.4-0005, 6.4-0010).
    #[must_use]
    pub fn resolve(&self, op: &str, unavailable: &[&str]) -> Resolution {
        if !self.0.contains_key(op) {
            return Resolution::Decline;
        }
        let mut need = BTreeSet::new();
        self.needed(op, &mut need);
        need.into_iter()
            .find(|n| unavailable.contains(n))
            .map_or(Resolution::Resolved, |m| Resolution::Deferred(m.to_owned()))
    }

    /// Resolve every `Op` node of `dag`: any decline wins, else the smallest deferred op name, else resolved. An empty DAG resolves (it has no
    /// non-primitive node). This aggregation is this crate's reading of 6.4-0005/-0010 over a whole DAG, not a KISS vector.
    #[must_use]
    pub fn resolve_dag(&self, dag: &OpDag<'_>, unavailable: &[&str]) -> Resolution {
        let mut deferred: Option<String> = None;
        for node in &dag.nodes {
            let Node::Op { op_name, .. } = node else {
                continue;
            };
            match self.resolve(op_name, unavailable) {
                Resolution::Decline => return Resolution::Decline,
                Resolution::Deferred(m) if deferred.as_ref().is_none_or(|d| m < *d) => {
                    deferred = Some(m)
                }
                _ => {}
            }
        }
        deferred.map_or(Resolution::Resolved, Resolution::Deferred)
    }
}
