// SPDX-License-Identifier: MIT OR Apache-2.0
//! **Typed values of the Interface fields** (KISS-CONTRACT 6.5-0008, -0010, -0011, -0012) and the **Dispatch expression check** of 6.11-0008,
//! read from a [`Contract`] whose field schemas [`crate::fields::parse_contract`] already checked:
//!
//! * `rank` (6.5-0012): a canonical non-negative decimal integer, the contract's compile-time operand rank;
//! * `count_unit` (6.5-0008): exactly `elements` or `vectors_x<w>` (`w` a canonical positive decimal, no delimiter);
//! * `alignment_bytes` (6.5-0010): a canonical positive decimal integer;
//! * `launch_scalars` (6.5-0011): entry for entry equal to the launch-scalar tail of the `positional_signature`;
//! * each Dispatch geometry field (6.11-0008): an expression the Dispatch grammar accepts over the contract's own vocabulary.
//!
//! **`in_place` is NOT interpreted.** 6.5-0009 says it states the input operand the output aliases, or that it aliases none, but the Draft
//! pins no spelling: the golden document of KISS `contract_vectors.json` writes `in_place = false` while the spec's own example text says
//! `in_place = none`, and no spelling is given for "aliases input `i`". It is returned as text ([`InterfaceValues::in_place`]) until KISS
//! pins one (reported to kiss).
//!
//! **Draft-tracking;** expect rework when the Draft moves. The golden document passes; it carries one value per field, so the fault cases in
//! `tests/values.rs` are written from the clause text.

use core::fmt;

use crate::fields::Contract;
use crate::interface::{ArgKind, Signature, SignatureError};
use crate::{Context, Expr, ParseError};

/// The Interface section's index in [`Contract::sections`].
const INTERFACE: usize = 2;
/// The Dispatch section's index in [`Contract::sections`].
const DISPATCH: usize = 3;

/// The unit the launch scalar `n` counts in (6.5-0008).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CountUnit {
    /// `elements`.
    Elements,
    /// `vectors_x<w>`: vectors of `w` elements.
    Vectors(u32),
}

/// The typed Interface scalars.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InterfaceValues<'a> {
    /// The declared operand logical rank (6.5-0012).
    pub rank: usize,
    /// The unit `n` counts in (6.5-0008).
    pub count_unit: CountUnit,
    /// The base-pointer alignment the ABI requires, in bytes (6.5-0010).
    pub alignment_bytes: u64,
    /// The `in_place` value as written; the Draft pins no spelling (see the module docs).
    pub in_place: &'a str,
}

/// A fault in an Interface scalar or a Dispatch field. Each is a typed decline; never a repair.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValueError {
    /// `rank` is not a canonical non-negative decimal integer (6.5-0012).
    Rank(String),
    /// `count_unit` is neither `elements` nor `vectors_x<w>` with a canonical positive `w` (6.5-0008).
    CountUnit(String),
    /// `alignment_bytes` is not a canonical positive decimal integer (6.5-0010).
    Alignment(String),
    /// A signature or launch-scalar list that does not parse (6.5-0011, 6.11-0006).
    Signature(SignatureError),
    /// `launch_scalars` differs from the launch-scalar tail of the `positional_signature` (6.5-0011).
    LaunchScalarsMismatch,
    /// A Dispatch geometry field that is not an expression the Dispatch grammar accepts over this contract's vocabulary (6.11-0008).
    DispatchExpression {
        /// The field key.
        key: String,
        /// Why the expression was refused.
        error: ParseError,
    },
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "contract value: {self:?}")
    }
}

impl std::error::Error for ValueError {}

/// A canonical decimal: `0`, or digits with no leading zero, no sign, no space.
fn decimal(s: &str) -> Option<u64> {
    let ok = !s.is_empty()
        && s.bytes().all(|b| b.is_ascii_digit())
        && (s.len() == 1 || !s.starts_with('0'));
    if ok { s.parse().ok() } else { None }
}

/// Read the typed Interface scalars of `contract` (6.5-0012, -0008, -0010).
///
/// # Errors
/// A [`ValueError`] naming the first fault, in the order of the Interface schema: `rank`, `count_unit`, `alignment_bytes`.
pub fn interface_values<'a>(contract: &Contract<'a>) -> Result<InterfaceValues<'a>, ValueError> {
    let get = |key: &str| contract.get(INTERFACE, key).unwrap_or_default();
    let rank_text = get("rank");
    let rank = decimal(rank_text)
        .and_then(|v| usize::try_from(v).ok())
        .ok_or_else(|| ValueError::Rank(rank_text.to_owned()))?;
    let unit_text = get("count_unit");
    let count_unit = if unit_text == "elements" {
        CountUnit::Elements
    } else {
        unit_text
            .strip_prefix("vectors_x")
            .and_then(decimal)
            .filter(|&w| w >= 1)
            .and_then(|w| u32::try_from(w).ok())
            .map(CountUnit::Vectors)
            .ok_or_else(|| ValueError::CountUnit(unit_text.to_owned()))?
    };
    let align_text = get("alignment_bytes");
    let alignment_bytes = decimal(align_text)
        .filter(|&v| v >= 1)
        .ok_or_else(|| ValueError::Alignment(align_text.to_owned()))?;
    Ok(InterfaceValues {
        rank,
        count_unit,
        alignment_bytes,
        in_place: get("in_place"),
    })
}

/// Check that `launch_scalars` equals, entry for entry, the launch-scalar tail of the `positional_signature` (the entries after the operand
/// pointers; 6.5-0011). Both are parsed as signatures (6.11-0006), so their descriptors and the pinned class order are checked too.
///
/// # Errors
/// [`ValueError::Signature`] for a list that does not parse, [`ValueError::LaunchScalarsMismatch`] when they differ.
pub fn check_launch_scalars(contract: &Contract<'_>) -> Result<(), ValueError> {
    let signature = Signature::parse(
        contract
            .get(INTERFACE, "positional_signature")
            .unwrap_or_default(),
    )
    .map_err(ValueError::Signature)?;
    let listed = Signature::parse(
        contract
            .get(INTERFACE, "launch_scalars")
            .unwrap_or_default(),
    )
    .map_err(ValueError::Signature)?;
    let tail: Vec<_> = signature
        .args
        .iter()
        .filter(|a| a.kind != ArgKind::Ptr)
        .cloned()
        .collect();
    if listed.args == tail {
        Ok(())
    } else {
        Err(ValueError::LaunchScalarsMismatch)
    }
}

/// Check that each Dispatch geometry field is an expression the Dispatch grammar accepts over the contract's own vocabulary and rank
/// (`context`, see [`Context::of_interface`]); 6.11-0008. A geometry-agnostic Dispatch section (6.6-0007) has no expression to check.
///
/// # Errors
/// [`ValueError::DispatchExpression`] naming the first field that is not such an expression.
pub fn check_dispatch(contract: &Contract<'_>, context: &Context) -> Result<(), ValueError> {
    for field in &contract.sections[DISPATCH] {
        if field.key == "dispatch_model" {
            continue;
        }
        Expr::parse_in(field.value, context).map_err(|error| ValueError::DispatchExpression {
            key: field.key.to_owned(),
            error,
        })?;
    }
    Ok(())
}
