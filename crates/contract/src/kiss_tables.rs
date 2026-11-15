// SPDX-License-Identifier: MIT OR Apache-2.0
//! **The easy path** to the two tables [`crate::read::read_contract`] needs: [`KissTables`] loads KISS's generated `op_facts.json` and
//! `dtype_manifest.json` (both under `conformance/corpus/` in KISS) and offers [`KissTables::read_contract`] as the one-line call. This is a
//! convenience, not the core: the core takes any [`OpFactSource`] and [`DtypeSource`], and the caller vendors the files from KISS (this crate
//! bakes none in, because a Draft table baked into a crate goes stale silently). **Draft-tracking.**

use core::fmt;

use crate::dtypes::{DtypeError, DtypeSource, DtypeTable};
use crate::op_facts::{FactTable, FactsError, OpFactSource};
use crate::read::{Checked, ReadError, read_contract};

/// KISS's op facts and dtype widths, loaded from its generated JSON.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct KissTables {
    /// The op facts (`op_facts.json`).
    pub ops: FactTable,
    /// The dtype widths (`dtype_manifest.json`).
    pub dtypes: DtypeTable,
}

/// One of the two files could not be read.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TablesError {
    /// `op_facts.json`.
    OpFacts(FactsError),
    /// `dtype_manifest.json`.
    Dtypes(DtypeError),
}

impl fmt::Display for TablesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpFacts(e) => e.fmt(f),
            Self::Dtypes(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for TablesError {}

impl KissTables {
    /// Load both tables from the text of `op_facts.json` and `dtype_manifest.json`.
    ///
    /// # Errors
    /// [`TablesError`] naming the file that could not be read.
    pub fn from_json(op_facts_json: &str, dtype_manifest_json: &str) -> Result<Self, TablesError> {
        Ok(Self {
            ops: FactTable::from_json(op_facts_json).map_err(TablesError::OpFacts)?,
            dtypes: DtypeTable::from_json(dtype_manifest_json).map_err(TablesError::Dtypes)?,
        })
    }

    /// [`read_contract`] over these tables.
    ///
    /// # Errors
    /// A [`ReadError`] naming the layer of the first fault.
    pub fn read_contract<'a>(
        &self,
        bytes: &'a [u8],
        supported_versions: &[u64],
    ) -> Result<Checked<'a>, ReadError> {
        read_contract(bytes, supported_versions, &self.ops, &self.dtypes)
    }
}

impl OpFactSource for KissTables {
    fn facts(&self, op: &str) -> Option<crate::op_facts::OpFacts> {
        self.ops.facts(op)
    }
}

impl DtypeSource for KissTables {
    fn storage_bits(&self, token: &str) -> Option<u64> {
        self.dtypes.storage_bits(token)
    }
}
