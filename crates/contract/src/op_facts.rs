// SPDX-License-Identifier: MIT OR Apache-2.0
//! **The cell-class-supported-by-op assertion** (KISS-CONTRACT 6.3-0006), decided from structural facts the named op *declares*, which KISS-Ops
//! owns (6.13-0009 a contraction, 6.11-0012 the index-operand role, 6.19-0036 a reduce axis). **This crate carries no per-op table**: the facts
//! come from an [`OpFactSource`] the caller supplies, so one source of meaning (KISS-Ops) stays the only copy.
//!
//! **Source.** KISS's generated sidecar `conformance/corpus/op_facts.json` (schema `kiss-op-facts-v1`, from `tools/kiss_op_facts.py` over
//! `spec/ops.md`, KISS#825, 0.18.3); [`FactTable::from_json`] reads it (a small built-in JSON reader: this crate stays dependency-free). The
//! facts read are `is_contraction`, `index_operand_role`, and a non-null `reduce_axes` (a carrier's `reduce_axes` or `norm_axes` field).
//! **Gap, not papered over:** 6.3-0006 also lets a *non-carrier* op (`cumsum`, `cumprod`, `cummax`) declare a reduce axis by resolving it from
//! its inner carrier (6.19-0036); the sidecar does not carry that, so for those ops `rdc` is declined until KISS says so there.

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
        let root = json::parse(text).map_err(FactsError::Json)?;
        let bad = |s: &str| FactsError::Schema(s.to_owned());
        if root.get("schema").and_then(json::Value::as_str) != Some("kiss-op-facts-v1") {
            return Err(bad("schema is not kiss-op-facts-v1"));
        }
        let Some(json::Value::Obj(ops)) = root.get("ops") else {
            return Err(bad("no ops object"));
        };
        let mut map = BTreeMap::new();
        for (name, v) in ops {
            let flag = |k: &str| match v.get(k) {
                Some(json::Value::Bool(b)) => Ok(*b),
                _ => Err(FactsError::Schema(format!("{name}: {k} is not a boolean"))),
            };
            let reduce_axis = match v.get("reduce_axes") {
                Some(json::Value::Null) => false,
                Some(json::Value::Str(_)) => true,
                _ => {
                    return Err(FactsError::Schema(format!(
                        "{name}: reduce_axes is neither null nor a string"
                    )));
                }
            };
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

    /// Whether the table knows no op.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// A minimal JSON reader: just enough for `op_facts.json`, kept private so the crate stays dependency-free.
mod json {
    pub enum Value {
        Null,
        Bool(bool),
        Num,
        Str(String),
        Arr,
        Obj(Vec<(String, Value)>),
    }

    impl Value {
        pub fn get(&self, key: &str) -> Option<&Value> {
            match self {
                Value::Obj(kv) => kv.iter().find(|(k, _)| k == key).map(|(_, v)| v),
                _ => None,
            }
        }
        pub fn as_str(&self) -> Option<&str> {
            match self {
                Value::Str(s) => Some(s),
                _ => None,
            }
        }
    }

    pub fn parse(text: &str) -> Result<Value, usize> {
        let mut p = P {
            b: text.as_bytes(),
            i: 0,
            depth: 0,
        };
        let v = p.value()?;
        p.ws();
        if p.i == p.b.len() { Ok(v) } else { Err(p.i) }
    }

    struct P<'a> {
        b: &'a [u8],
        i: usize,
        depth: usize,
    }

    impl P<'_> {
        fn ws(&mut self) {
            while matches!(self.b.get(self.i), Some(b' ' | b'\t' | b'\n' | b'\r')) {
                self.i += 1;
            }
        }
        fn lit(&mut self, word: &str, v: Value) -> Result<Value, usize> {
            if self.b[self.i..].starts_with(word.as_bytes()) {
                self.i += word.len();
                Ok(v)
            } else {
                Err(self.i)
            }
        }
        fn value(&mut self) -> Result<Value, usize> {
            self.ws();
            match self.b.get(self.i).copied() {
                Some(b'n') => self.lit("null", Value::Null),
                Some(b't') => self.lit("true", Value::Bool(true)),
                Some(b'f') => self.lit("false", Value::Bool(false)),
                Some(b'"') => self.string().map(Value::Str),
                Some(b'[') => self.nest(|p| p.array()),
                Some(b'{') => self.nest(|p| p.object()),
                Some(b'-' | b'0'..=b'9') => {
                    let s = self.i;
                    while matches!(
                        self.b.get(self.i),
                        Some(b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9')
                    ) {
                        self.i += 1;
                    }
                    if self.i == s { Err(s) } else { Ok(Value::Num) }
                }
                _ => Err(self.i),
            }
        }
        fn nest(&mut self, f: fn(&mut Self) -> Result<Value, usize>) -> Result<Value, usize> {
            self.depth += 1;
            if self.depth > 64 {
                return Err(self.i);
            }
            let v = f(self);
            self.depth -= 1;
            v
        }
        fn array(&mut self) -> Result<Value, usize> {
            self.i += 1;
            self.ws();
            if self.b.get(self.i) == Some(&b']') {
                self.i += 1;
                return Ok(Value::Arr);
            }
            loop {
                self.value()?;
                self.ws();
                match self.b.get(self.i) {
                    Some(b',') => self.i += 1,
                    Some(b']') => {
                        self.i += 1;
                        return Ok(Value::Arr);
                    }
                    _ => return Err(self.i),
                }
            }
        }
        fn object(&mut self) -> Result<Value, usize> {
            self.i += 1;
            let mut out: Vec<(String, Value)> = Vec::new();
            self.ws();
            if self.b.get(self.i) == Some(&b'}') {
                self.i += 1;
                return Ok(Value::Obj(out));
            }
            loop {
                self.ws();
                if self.b.get(self.i) != Some(&b'"') {
                    return Err(self.i);
                }
                let at = self.i;
                let k = self.string()?;
                if out.iter().any(|(e, _)| *e == k) {
                    return Err(at);
                }
                self.ws();
                if self.b.get(self.i) != Some(&b':') {
                    return Err(self.i);
                }
                self.i += 1;
                out.push((k, self.value()?));
                self.ws();
                match self.b.get(self.i) {
                    Some(b',') => self.i += 1,
                    Some(b'}') => {
                        self.i += 1;
                        return Ok(Value::Obj(out));
                    }
                    _ => return Err(self.i),
                }
            }
        }
        fn string(&mut self) -> Result<String, usize> {
            self.i += 1;
            let mut out: Vec<u8> = Vec::new();
            loop {
                let c = *self.b.get(self.i).ok_or(self.i)?;
                self.i += 1;
                match c {
                    b'"' => return String::from_utf8(out).map_err(|_| self.i),
                    b'\\' => {
                        let e = *self.b.get(self.i).ok_or(self.i)?;
                        self.i += 1;
                        match e {
                            b'"' | b'\\' | b'/' => out.push(e),
                            b'n' => out.push(b'\n'),
                            b't' => out.push(b'\t'),
                            b'r' => out.push(b'\r'),
                            b'b' => out.push(8),
                            b'f' => out.push(12),
                            b'u' => {
                                let h = self.b.get(self.i..self.i + 4).ok_or(self.i)?;
                                let h = core::str::from_utf8(h).map_err(|_| self.i)?;
                                let n = u32::from_str_radix(h, 16).map_err(|_| self.i)?;
                                let ch = char::from_u32(n).ok_or(self.i)?;
                                self.i += 4;
                                let mut buf = [0u8; 4];
                                out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                            }
                            _ => return Err(self.i),
                        }
                    }
                    0..=31 => return Err(self.i),
                    _ => out.push(c),
                }
            }
        }
    }
}

impl OpFactSource for FactTable {
    fn facts(&self, op: &str) -> Option<OpFacts> {
        self.0.get(op).copied()
    }
}
