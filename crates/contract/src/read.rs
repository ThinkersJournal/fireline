// SPDX-License-Identifier: MIT OR Apache-2.0
//! **Read a whole contract**: one call that runs every layer of the crate over a document's bytes and stops at the first fault, naming the
//! layer: the framing (6.11-0002..-0004, [`crate::document`]), the field schemas of the seven sections (6.11-0005, [`crate::fields`]), the
//! Identity rules (6.3, [`crate::identity`]), the cross-section rules and vocabularies (6.7 to 6.9, [`crate::consistency`]), the typed Interface
//! scalars and the `launch_scalars` tail (6.5, [`crate::values`]), the `positional_signature` (6.11-0006, [`crate::interface`]) and the Dispatch
//! expressions in the contract's own vocabulary (6.11-0008).
//!
//! The first fault wins, in that order. This is plumbing over the other modules: it adds no rule of its own.
//!
//! **Not run here** (each needs input this call does not have): the strides rule (6.5-0005, needs whether the cell is fully packed and the
//! operand count: [`crate::interface::Signature::check_strides`]), the `output_layouts` field (6.5-0014, needs the output-operand count and the
//! Classify dtype table: [`crate::layout::parse_output_layouts`]), the bind-time checks, and everything that needs KISS-Ops tables.
//!
//! **Draft-tracking;** expect rework when the Draft moves.

use core::fmt;

use crate::Context;
use crate::consistency::{ConsistencyError, check_consistency};
use crate::document::{Document, DocumentError, parse_document};
use crate::fields::{Contract, FieldError, parse_contract};
use crate::identity::{IdentityError, check_identity};
use crate::interface::{Signature, SignatureError};
use crate::values::{
    InterfaceValues, ValueError, check_dispatch, check_launch_scalars, interface_values,
};

/// The first fault found, naming the layer.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReadError {
    /// The framing (header line, length, CRC-32, section headings).
    Document(DocumentError),
    /// A section's field lines or field schema.
    Fields(FieldError),
    /// The Identity section's rules.
    Identity(IdentityError),
    /// A cross-section rule or closed vocabulary.
    Consistency(ConsistencyError),
    /// An Interface scalar, the `launch_scalars` tail or a Dispatch expression.
    Values(ValueError),
    /// The `positional_signature`.
    Signature(SignatureError),
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Document(e) => e.fmt(f),
            Self::Fields(e) => e.fmt(f),
            Self::Identity(e) => e.fmt(f),
            Self::Consistency(e) => e.fmt(f),
            Self::Values(e) => e.fmt(f),
            Self::Signature(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for ReadError {}

macro_rules! from {
    ($($t:ty => $v:ident),*) => { $(impl From<$t> for ReadError { fn from(e: $t) -> Self { Self::$v(e) } })* };
}
from!(DocumentError => Document, FieldError => Fields, IdentityError => Identity, ConsistencyError => Consistency, ValueError => Values, SignatureError => Signature);

/// A contract that passed every layer, with the typed pieces the layers produced.
#[derive(Clone, Debug)]
pub struct Checked<'a> {
    /// The framed document: header version, body and the seven raw sections.
    pub document: Document<'a>,
    /// The seven sections as ordered `key`/`value` pairs.
    pub contract: Contract<'a>,
    /// The typed Interface scalars.
    pub values: InterfaceValues<'a>,
    /// The parsed `positional_signature`.
    pub signature: Signature,
    /// The contract's parse context: its declared rank and its own launch-scalar vocabulary. Use it with [`crate::Expr::parse_in`].
    pub context: Context,
}

/// Run every layer over `bytes` (a document: for example the payload of a `CRSP`, KISS-CONTRACT-6.1-0005) and return the typed pieces, or the
/// first fault. `supported_versions` is the set of `contract_version` values the reader supports (6.1-0008).
///
/// # Errors
/// A [`ReadError`] naming the layer of the first fault; never panics.
pub fn read_contract<'a>(
    bytes: &'a [u8],
    supported_versions: &[u64],
) -> Result<Checked<'a>, ReadError> {
    let document = parse_document(bytes, supported_versions)?;
    let contract = parse_contract(&document)?;
    check_identity(&contract)?;
    check_consistency(&contract)?;
    let values = interface_values(&contract)?;
    check_launch_scalars(&contract)?;
    let signature = Signature::parse(contract.get(2, "positional_signature").unwrap_or_default())?;
    let context = Context::of_interface(values.rank, &signature);
    check_dispatch(&contract, &context)?;
    Ok(Checked {
        document,
        contract,
        values,
        signature,
        context,
    })
}
