// SPDX-License-Identifier: MIT OR Apache-2.0
//! The **cross-section rules and closed vocabularies** of a KISS-Contract document's Capabilities, Guarantees and Provenance sections, read
//! from a [`Contract`] whose field schemas [`crate::fields::parse_contract`] already checked:
//!
//! * copies that must agree byte for byte: the Capabilities `accept_predicate` with the Identity's (6.7-0002), the Provenance `revision_hash`
//!   with the Identity's (6.9-0003), the Capabilities `determinism_class` with the Guarantees' (6.7-0004), the Provenance `cost_provenance`
//!   with the Guarantees' (6.9-0006);
//! * closed vocabularies: `determinism_class` (6.8-0003), `math_fidelity` (6.8-0004a: `bit-stable` or `reduced-mantissa-permitted:<n>`, `n`
//!   a canonical decimal 1 to 52, the bare spelling declined), `bit_stability` (6.8-0013, with the rule that `order-invariant/nondeterministic`
//!   never declares `portable`), `cost_provenance` (6.8-0006), `index_width` (6.7-0003);
//! * the opaque-blob text encoding `<n>:<hex>` (6.11-0001) of `revision_hash` and `negotiation_metadata` (6.9-0008);
//! * the `audited_status` derivation (6.8-0008, -0009, -0010).
//!
//! **Not here (needs KISS-Ops tables this crate does not carry):** the precision-class set and its agreement with the ULP tiers (6.7-0005),
//! the contents of `per_backend_ulp_tiers`, and the cost expressions (6.7-0006, 6.8-0007).
//!
//! **Two readings stated.** (1) 6.7-0003 says `index_width` is one of `{idx32, idx64}` but KISS's golden document writes `ix32` (the
//! `structure_key` code); this module accepts `ix32` and `ix64` and reported the spelling to kiss. (2) 6.8-0009 and -0010 tie `audited_status`
//! to "a bounded precision against a named `reference_function`": this module reads that as a non-empty `reference_function` and a non-empty
//! `per_backend_ulp_tiers` array, since the Draft pins no spelling for an unnamed reference.
//!
//! **Draft-tracking;** expect rework when the Draft moves. The golden document of KISS `contract_vectors.json` passes; it carries one value
//! per field, so every fault case in `tests/consistency.rs` is written from the clause text.

use core::fmt;

use crate::fields::Contract;

const IDENTITY: usize = 0;
const CAPABILITIES: usize = 4;
const GUARANTEES: usize = 5;
const PROVENANCE: usize = 6;

/// The `determinism_class` enum of KISS-OPS 6.0-0001, spelled verbatim (6.8-0003, 6.7-0004).
pub const DETERMINISM_CLASSES: [&str; 3] = [
    "exact-byte",
    "ULP/tolerance",
    "order-invariant/nondeterministic",
];
/// The `bit_stability` set of 6.8-0013.
pub const BIT_STABILITY: [&str; 3] = ["portable", "same-hardware", "none"];
/// The `cost_provenance` values (6.8-0006, 6.9-0006).
pub const COST_PROVENANCE: [&str; 2] = ["declared", "measured"];
/// The `index_width` codes (6.7-0003; see the module docs on the spelling).
pub const INDEX_WIDTHS: [&str; 2] = ["ix32", "ix64"];

/// A cross-section or vocabulary fault. Each is a typed decline; never a repair.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConsistencyError {
    /// Two copies that must be byte-for-byte equal differ.
    Disagree {
        /// The field, in the section that must match the Identity or Guarantees copy.
        field: &'static str,
        /// The clause that requires equality.
        clause: &'static str,
    },
    /// A value outside its closed vocabulary (or, for `math_fidelity`, not `bit-stable` or `reduced-mantissa-permitted:<n>`).
    NotInVocabulary {
        /// The field.
        field: &'static str,
        /// The value found.
        value: String,
    },
    /// `bit_stability = portable` under `determinism_class = order-invariant/nondeterministic` (6.8-0013).
    PortableUnderNondeterministic,
    /// A value that is not the opaque-blob encoding `<n>:<hex>` with exactly `2n` lowercase hex digits (6.11-0001, 6.9-0008).
    Blob {
        /// The field.
        field: &'static str,
    },
    /// `audited_status` is not the value the derivation rule yields for the Guarantees' own fields (6.8-0008..-0010).
    AuditedStatus {
        /// What the rule yields (`audited` or `unaudited`).
        derived: &'static str,
        /// What the document says.
        found: String,
    },
}

impl fmt::Display for ConsistencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "contract consistency: {self:?}")
    }
}

impl std::error::Error for ConsistencyError {}

fn get<'a>(c: &Contract<'a>, section: usize, key: &str) -> &'a str {
    c.get(section, key).unwrap_or_default()
}

fn blob_ok(s: &str) -> bool {
    let Some((n, hex)) = s.split_once(':') else {
        return false;
    };
    let canonical = !n.is_empty()
        && n.bytes().all(|b| b.is_ascii_digit())
        && (n.len() == 1 || !n.starts_with('0'));
    canonical
        && n.parse::<usize>().is_ok_and(|n| hex.len() == n * 2)
        && hex.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// `math_fidelity` (6.8-0004a): `bit-stable`, or `reduced-mantissa-permitted:<n>` with `n` a canonical decimal from 1 to 52.
fn math_fidelity_ok(s: &str) -> bool {
    s == "bit-stable"
        || s.strip_prefix("reduced-mantissa-permitted:")
            .is_some_and(|n| {
                n.bytes().all(|b| b.is_ascii_digit())
                    && !n.is_empty()
                    && !n.starts_with('0')
                    && n.parse::<u8>().is_ok_and(|n| (1..=52).contains(&n))
            })
}

fn in_vocab(field: &'static str, value: &str, set: &[&str]) -> Result<(), ConsistencyError> {
    if set.contains(&value) {
        Ok(())
    } else {
        Err(ConsistencyError::NotInVocabulary {
            field,
            value: value.to_owned(),
        })
    }
}

/// Check the cross-section equalities, closed vocabularies, blob encodings and the `audited_status` derivation listed in the module docs.
///
/// # Errors
/// A [`ConsistencyError`] naming the first fault, in the order of the sections (Identity copies, Capabilities, Guarantees, Provenance).
pub fn check_consistency(c: &Contract<'_>) -> Result<(), ConsistencyError> {
    let disagree = |field, clause| ConsistencyError::Disagree { field, clause };
    // Identity copies against Capabilities and Provenance.
    if get(c, CAPABILITIES, "accept_predicate") != get(c, IDENTITY, "accept_predicate") {
        return Err(disagree("capabilities.accept_predicate", "6.7-0002"));
    }
    if !blob_ok(get(c, IDENTITY, "revision_hash")) {
        return Err(ConsistencyError::Blob {
            field: "identity.revision_hash",
        });
    }
    if get(c, PROVENANCE, "revision_hash") != get(c, IDENTITY, "revision_hash") {
        return Err(disagree("provenance.revision_hash", "6.9-0003"));
    }
    // Capabilities.
    in_vocab(
        "capabilities.index_width",
        get(c, CAPABILITIES, "index_width"),
        &INDEX_WIDTHS,
    )?;
    let determinism = get(c, GUARANTEES, "determinism_class");
    in_vocab(
        "guarantees.determinism_class",
        determinism,
        &DETERMINISM_CLASSES,
    )?;
    if get(c, CAPABILITIES, "determinism_class") != determinism {
        return Err(disagree("capabilities.determinism_class", "6.7-0004"));
    }
    // Guarantees.
    let fidelity = get(c, GUARANTEES, "math_fidelity");
    if !math_fidelity_ok(fidelity) {
        return Err(ConsistencyError::NotInVocabulary {
            field: "guarantees.math_fidelity",
            value: fidelity.to_owned(),
        });
    }
    let stability = get(c, GUARANTEES, "bit_stability");
    in_vocab("guarantees.bit_stability", stability, &BIT_STABILITY)?;
    if determinism == "order-invariant/nondeterministic" && stability == "portable" {
        return Err(ConsistencyError::PortableUnderNondeterministic);
    }
    let cost = get(c, GUARANTEES, "cost_provenance");
    in_vocab("guarantees.cost_provenance", cost, &COST_PROVENANCE)?;
    let derived = if get(c, GUARANTEES, "reference_function").is_empty()
        || get(c, GUARANTEES, "per_backend_ulp_tiers") == "[]"
    {
        "unaudited"
    } else {
        "audited"
    };
    let status = get(c, GUARANTEES, "audited_status");
    if status != derived {
        return Err(ConsistencyError::AuditedStatus {
            derived,
            found: status.to_owned(),
        });
    }
    // Provenance.
    if get(c, PROVENANCE, "cost_provenance") != cost {
        return Err(disagree("provenance.cost_provenance", "6.9-0006"));
    }
    if !blob_ok(get(c, PROVENANCE, "negotiation_metadata")) {
        return Err(ConsistencyError::Blob {
            field: "provenance.negotiation_metadata",
        });
    }
    Ok(())
}
