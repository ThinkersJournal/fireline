// SPDX-License-Identifier: MIT OR Apache-2.0
//! The **Identity section's own rules** (KISS-CONTRACT 6.3-0002, -0007, -0009, -0011): the checks that need only the Identity fields and the
//! document header, applied to a [`Contract`] whose field schemas [`crate::fields::parse_contract`] already checked.
//!
//! * 6.3-0009: the Identity `contract_kind` and `contract_version` equal the transport header's, byte for byte; the header is authoritative.
//! * 6.3-0002: the `accept_predicate` is a printable, LF-free UTF-8 token carried verbatim as a `structure_key` (`sk<version>|<class>|...`).
//! * 6.3-0007: the `target_capability` is a namespaced `<namespace>:<capability-set>` descriptor.
//! * 6.3-0011: the `accept_predicate`'s target component equals the `target_capability` byte for byte (the target is the fourth
//!   `|`-separated component of every `structure_key` of the KISS corpus, `sk3` to `sk9`).
//!
//! **Not here:** the consistency of the cell class with the op the `op_identity` names (6.3-0006), which needs KISS-Ops' structural facts
//! per op (a contraction, an index operand, a reduce axis) and so belongs with a KISS-Ops table; and the two `op_identity` forms (6.3-0003,
//! -0004, KISS-Grammar tags). **Draft-tracking;** expect rework when the Draft moves. No corpus vector has a second value of any Identity
//! field, so the fault cases in `tests/identity.rs` are written from the clause text.

use core::fmt;

use crate::document::CONTRACT_KIND;
use crate::fields::Contract;

/// An Identity fault. Each is a typed decline (6.3-0009, -0011); a reader never repairs.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum IdentityError {
    /// The Identity `contract_kind` differs from the header's (6.3-0009).
    KindMismatch(String),
    /// The Identity `contract_version` differs from the header's, byte for byte (6.3-0009).
    VersionMismatch {
        /// The Identity field's text.
        identity: String,
        /// The header's version.
        header: u64,
    },
    /// The `accept_predicate` is empty, holds a control character, or is not a `structure_key` of the form `sk<digits>|<class>|...`
    /// with at least four components (6.3-0002).
    AcceptPredicate,
    /// The `target_capability` is not `<namespace>:<capability-set>` (6.3-0007).
    TargetCapabilityShape(String),
    /// The `accept_predicate`'s target component differs from the `target_capability` (6.3-0011).
    TargetMismatch {
        /// The component of the `structure_key`.
        accept_predicate: String,
        /// The Identity `target_capability`.
        target_capability: String,
    },
}

impl fmt::Display for IdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "contract identity: {self:?}")
    }
}

impl std::error::Error for IdentityError {}

/// The target component of a `structure_key` token (its fourth `|`-separated component), or `None` if the token is not shaped like one.
fn structure_key_target(token: &str) -> Option<&str> {
    if token.is_empty() || token.chars().any(char::is_control) {
        return None;
    }
    let mut parts = token.split('|');
    let version = parts.next()?;
    version
        .strip_prefix("sk")
        .filter(|d| !d.is_empty() && d.bytes().all(|b| b.is_ascii_digit()))?;
    let (class, _dtype, target) = (parts.next()?, parts.next()?, parts.next()?);
    (!class.is_empty() && !target.is_empty()).then_some(target)
}

/// Check the Identity section of `contract` (6.3-0009, -0002, -0007, -0011).
///
/// # Errors
/// An [`IdentityError`] naming the first fault, in the order of the Identity schema: kind, version, `accept_predicate`,
/// `target_capability`, then the target agreement.
pub fn check_identity(contract: &Contract<'_>) -> Result<(), IdentityError> {
    // Identity is section 1 (index 0); `parse_contract` guarantees every key below is present.
    let get = |key: &str| contract.get(0, key).unwrap_or_default();
    let kind = get("contract_kind");
    if kind != CONTRACT_KIND {
        return Err(IdentityError::KindMismatch(kind.to_owned()));
    }
    let version = get("contract_version");
    if version != contract.contract_version.to_string() {
        return Err(IdentityError::VersionMismatch {
            identity: version.to_owned(),
            header: contract.contract_version,
        });
    }
    let accept = get("accept_predicate");
    let target = structure_key_target(accept).ok_or(IdentityError::AcceptPredicate)?;
    let capability = get("target_capability");
    match capability.split_once(':') {
        Some((ns, set))
            if !ns.is_empty()
                && !set.is_empty()
                && !capability
                    .chars()
                    .any(|c| c.is_control() || c.is_whitespace()) => {}
        _ => return Err(IdentityError::TargetCapabilityShape(capability.to_owned())),
    }
    if target != capability {
        return Err(IdentityError::TargetMismatch {
            accept_predicate: target.to_owned(),
            target_capability: capability.to_owned(),
        });
    }
    Ok(())
}
