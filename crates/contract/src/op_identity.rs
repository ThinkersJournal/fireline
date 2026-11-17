// SPDX-License-Identifier: MIT OR Apache-2.0
//! **The `op_identity` text spelling, structure only** (KISS-CONTRACT 6.11-0009, with 6.3-0003 and 6.3-0004; CireSnave ruled board 451 on
//! 2026-11-16): `<op_name>[{<op_attrs>}][[<operand_role_tuple>]]`, where `{...}` is present iff the op has an OpAttrs record and then spells **every**
//! field of it, in the KISS-Ops canonical field order, as `key=value` pairs separated by `, `; the optional role tuple is `[role:dtype, ...]`.
//!
//! [`parse_op_identity`] decides **what the line itself shows**, from two sources the caller supplies (no table is kept here): the ordered OpAttrs
//! field names of every op ([`AttrTable::from_json`] over KISS's `op_facts.json`, its `opattrs` lists) and the dtype tokens ([`DtypeSource`]).
//! It declines with [`IdentityDecline`]: `Malformed` (spacing, brackets, characters), `UnknownOp`, `UnknownAttr` (also any attribute on an op with
//! no record), `DuplicateAttr`, `Incomplete` (a carrier written bare, or a field missing), `AttrOrder`, `BadRoleTuple`, checked in that precedence
//! after the shape. Whether a line was reduced to the bare name is a producer rule (6.3-0004) and is not decided here.
//!
//! **Not decided here, on purpose: field value types.** Whether `keepdim=1` is a bad flag, `oob_policy=wrap` is outside its enum, `axis=8` is
//! past `MAX_RANK`, or an `accumulator` is a float dtype needs each field's type, vocabulary and range, which KISS publishes only as prose in
//! `spec/ops.md` 6.19.2 and 6.19.3 (its reference parses the Markdown). No JSON in `conformance/corpus` carries them, and this crate keeps no
//! hand-written table, so [`OpIdentity::attrs`] hands back the **unchecked value text** and the `op-identity-bad-value` decline is not produced.
//! The one universal check made is the shape of a value (a token of `[a-z0-9_-]`, or a bracketed list of digit runs with no spaces). **A line this
//! accepts is therefore structurally valid, not value-valid.** **Draft-tracking.**

use core::fmt;
use std::collections::BTreeMap;

use crate::dtypes::DtypeSource;

/// A typed decline of an `op_identity` line. The wire tags are KISS's (`op-identity-*`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum IdentityDecline {
    /// Not `<op_name>[{<attrs>}][[<roles>]]`: bad name characters, spacing, an unclosed brace or bracket, a malformed value.
    Malformed,
    /// The op name is not a KISS-Ops op.
    UnknownOp,
    /// A key the op's OpAttrs record does not have (also any attribute on an op with no record).
    UnknownAttr,
    /// A key written twice.
    DuplicateAttr,
    /// A field of the op's OpAttrs record is missing (including a carrier op written bare).
    Incomplete,
    /// Every field present but not in the canonical order.
    AttrOrder,
    /// The role tuple is empty or has an entry that is not `role:dtype` (a dtype token or `*`).
    BadRoleTuple,
}

impl IdentityDecline {
    /// The stable wire tag (`op-identity-...`).
    #[must_use]
    pub fn wire_tag(self) -> &'static str {
        match self {
            Self::Malformed => "op-identity-malformed",
            Self::UnknownOp => "op-identity-unknown-op",
            Self::UnknownAttr => "op-identity-unknown-attr",
            Self::DuplicateAttr => "op-identity-duplicate-attr",
            Self::Incomplete => "op-identity-incomplete",
            Self::AttrOrder => "op-identity-attr-order",
            Self::BadRoleTuple => "op-identity-bad-role-tuple",
        }
    }
}

impl fmt::Display for IdentityDecline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.wire_tag())
    }
}

impl std::error::Error for IdentityDecline {}

/// Where the ordered OpAttrs field names of an op come from.
pub trait AttrSource {
    /// The op's OpAttrs field names in canonical order (empty: the op has no record), or `None` if the op is not a KISS-Ops op.
    fn attr_fields(&self, op: &str) -> Option<&[String]>;
}

/// The ordered OpAttrs field names, read from KISS's `op_facts.json` (`ops.<op>.opattrs`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AttrTable(BTreeMap<String, Vec<String>>);

/// `op_facts.json` could not be read for its `opattrs` lists. Typed; never a repair.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AttrTableError {
    /// Not valid JSON (byte offset of the first problem).
    Json(usize),
    /// Valid JSON but not `kiss-op-facts-v1`, or an op entry whose `opattrs` is missing or not an array of strings.
    Schema(String),
}

impl fmt::Display for AttrTableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "op attrs: {self:?}")
    }
}

impl std::error::Error for AttrTableError {}

impl AttrTable {
    /// Read the `opattrs` lists of `op_facts.json` (schema `kiss-op-facts-v1`).
    ///
    /// # Errors
    /// [`AttrTableError::Json`] for malformed JSON, [`AttrTableError::Schema`] for a document of another shape.
    pub fn from_json(text: &str) -> Result<Self, AttrTableError> {
        let root = crate::json::parse(text).map_err(AttrTableError::Json)?;
        let bad = AttrTableError::Schema;
        if root.get("schema").and_then(crate::json::Value::as_str) != Some("kiss-op-facts-v1") {
            return Err(bad("schema is not kiss-op-facts-v1".into()));
        }
        let Some(crate::json::Value::Obj(ops)) = root.get("ops") else {
            return Err(bad("no ops object".into()));
        };
        let mut map = BTreeMap::new();
        for (name, v) in ops {
            let arr = v
                .get("opattrs")
                .and_then(crate::json::Value::as_arr)
                .ok_or_else(|| bad(format!("{name}: opattrs is not an array")))?;
            let fields = arr
                .iter()
                .map(|x| {
                    x.as_str()
                        .map(str::to_owned)
                        .ok_or_else(|| bad(format!("{name}: opattrs holds a non-string")))
                })
                .collect::<Result<Vec<_>, _>>()?;
            map.insert(name.clone(), fields);
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

impl AttrSource for AttrTable {
    fn attr_fields(&self, op: &str) -> Option<&[String]> {
        self.0.get(op).map(Vec::as_slice)
    }
}

/// A structurally valid `op_identity` line. The value text is **unchecked** (see the module docs).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpIdentity<'a> {
    /// The bare KISS-Ops op name.
    pub op: &'a str,
    /// Every OpAttrs field in canonical order, with its value text as written (empty iff the op has no record).
    pub attrs: Vec<(&'a str, &'a str)>,
    /// The `[role:dtype, ...]` entries as written (empty iff there is no suffix); a dtype is a token or `*`.
    pub roles: Vec<(&'a str, &'a str)>,
}

fn is_ident(s: &str) -> bool {
    let mut c = s.chars();
    matches!(c.next(), Some('a'..='z')) && c.all(|ch| matches!(ch, 'a'..='z' | '0'..='9' | '_'))
}

/// A value's shape: a token of `[a-z0-9_-]`, or `[digits,digits]` (possibly empty) with no spaces.
fn is_value_shape(v: &str) -> bool {
    if let Some(inner) = v.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        return inner.is_empty()
            || inner
                .split(',')
                .all(|d| !d.is_empty() && d.bytes().all(|b| b.is_ascii_digit()));
    }
    !v.is_empty()
        && v.bytes()
            .all(|b| matches!(b, b'a'..=b'z' | b'0'..=b'9' | b'_' | b'-'))
}

/// Parse and check an `op_identity` line against the op field lists and dtype tokens the caller supplies.
///
/// # Errors
/// An [`IdentityDecline`]; see the module docs for what is and is not decided.
pub fn parse_op_identity<'a>(
    line: &'a str,
    attrs: &dyn AttrSource,
    dtypes: &dyn DtypeSource,
) -> Result<OpIdentity<'a>, IdentityDecline> {
    use IdentityDecline as D;
    let name_end = line
        .find(|c: char| !matches!(c, 'a'..='z' | '0'..='9' | '_'))
        .unwrap_or(line.len());
    let (op, mut rest) = line.split_at(name_end);
    if !is_ident(op) {
        return Err(D::Malformed);
    }
    let mut pairs: Option<Vec<(&str, &str)>> = None;
    if let Some(after) = rest.strip_prefix('{') {
        let close = after.find('}').ok_or(D::Malformed)?;
        let body = &after[..close];
        rest = &after[close + 1..];
        let mut v = Vec::new();
        for pair in body.split(", ") {
            let (k, val) = pair.split_once('=').ok_or(D::Malformed)?;
            if !is_ident(k) || !is_value_shape(val) {
                return Err(D::Malformed);
            }
            v.push((k, val));
        }
        pairs = Some(v);
    }
    let mut roles = Vec::new();
    let mut has_roles = false;
    if let Some(after) = rest.strip_prefix('[') {
        let body = after.strip_suffix(']').ok_or(D::Malformed)?;
        has_roles = true;
        rest = "";
        if body.contains(['[', ']', '{', '}']) {
            return Err(D::Malformed);
        }
        roles = body
            .split(", ")
            .map(|e| e.split_once(':').ok_or(D::BadRoleTuple))
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_default();
    }
    if !rest.is_empty() {
        return Err(D::Malformed);
    }

    let fields = attrs.attr_fields(op).ok_or(D::UnknownOp)?;
    let given = pairs.unwrap_or_default();
    if fields.is_empty() {
        if !given.is_empty() {
            return Err(D::UnknownAttr);
        }
    } else {
        if given.iter().any(|(k, _)| !fields.iter().any(|f| f == k)) {
            return Err(D::UnknownAttr);
        }
        for (i, (k, _)) in given.iter().enumerate() {
            if given[..i].iter().any(|(p, _)| p == k) {
                return Err(D::DuplicateAttr);
            }
        }
        if given.len() < fields.len() {
            return Err(D::Incomplete);
        }
        if given
            .iter()
            .map(|(k, _)| *k)
            .ne(fields.iter().map(String::as_str))
        {
            return Err(D::AttrOrder);
        }
    }

    if has_roles {
        if roles.is_empty() {
            return Err(D::BadRoleTuple);
        }
        for (role, dtype) in &roles {
            if !is_ident(role) || !(*dtype == "*" || dtypes.storage_bits(dtype).is_some()) {
                return Err(D::BadRoleTuple);
            }
        }
    }
    Ok(OpIdentity {
        op,
        attrs: given,
        roles,
    })
}
