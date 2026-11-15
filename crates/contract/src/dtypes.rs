// SPDX-License-Identifier: MIT OR Apache-2.0
//! **Dtype storage widths** for the one place the contract text needs them: a `const` `Source` node's `bits=0x<HEX>` is zero-padded to
//! `2 * ceil(width_bits / 8)` digits (KISS-CONTRACT 6.11-0007), and `dtype=` must be a KISS-Classify dtype token. **This crate carries no dtype
//! table**: the widths come from a [`DtypeSource`] the caller supplies; [`DtypeTable::from_json`] reads KISS's generated
//! `conformance/corpus/dtype_manifest.json` (schema `kiss-dtype-manifest-v1`, from the KISS-CLASSIFY 6.1 table). **Draft-tracking.**

use core::fmt;
use std::collections::BTreeMap;

/// Where the storage width of a dtype token comes from.
pub trait DtypeSource {
    /// The storage width in bits of the dtype token `token`, or `None` if it is not a dtype token the source knows.
    fn storage_bits(&self, token: &str) -> Option<u64>;
}

/// Dtype widths read from KISS's `dtype_manifest.json`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DtypeTable(BTreeMap<String, u64>);

/// `dtype_manifest.json` could not be read. Typed; never a repair.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DtypeError {
    /// Not valid JSON (byte offset of the first problem).
    Json(usize),
    /// Valid JSON but not `kiss-dtype-manifest-v1`, or a `dtypes` entry without a `token` and an integer `storage_bits`.
    Schema(String),
}

impl fmt::Display for DtypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dtype manifest: {self:?}")
    }
}

impl std::error::Error for DtypeError {}

impl DtypeTable {
    /// Read `dtype_manifest.json`.
    ///
    /// # Errors
    /// [`DtypeError::Json`] for malformed JSON, [`DtypeError::Schema`] for a document of another shape.
    pub fn from_json(text: &str) -> Result<Self, DtypeError> {
        let root = crate::json::parse(text).map_err(DtypeError::Json)?;
        let bad = |s: String| DtypeError::Schema(s);
        if root.get("schema").and_then(crate::json::Value::as_str) != Some("kiss-dtype-manifest-v1")
        {
            return Err(bad("schema is not kiss-dtype-manifest-v1".to_owned()));
        }
        let list = root
            .get("dtypes")
            .and_then(crate::json::Value::as_arr)
            .ok_or_else(|| bad("no dtypes array".to_owned()))?;
        let mut map = BTreeMap::new();
        for (i, e) in list.iter().enumerate() {
            let token = e.get("token").and_then(crate::json::Value::as_str);
            let bits = e.get("storage_bits").and_then(crate::json::Value::as_u64);
            let (Some(token), Some(bits)) = (token, bits) else {
                return Err(bad(format!(
                    "dtypes[{i}]: no token or integer storage_bits"
                )));
            };
            if map.insert(token.to_owned(), bits).is_some() {
                return Err(bad(format!("dtypes[{i}]: duplicate token {token}")));
            }
        }
        Ok(Self(map))
    }

    /// The number of dtype tokens the table knows.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the table knows no dtype.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl DtypeSource for DtypeTable {
    fn storage_bits(&self, token: &str) -> Option<u64> {
        self.0.get(token).copied()
    }
}
