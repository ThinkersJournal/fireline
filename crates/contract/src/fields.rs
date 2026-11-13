// SPDX-License-Identifier: MIT OR Apache-2.0
//! The **field lines** of a KISS-Contract document's seven sections (6.11-0001, 6.11-0005) and each section's **field schema**: the exact keys,
//! in the exact order, with the optional ones and the Dispatch alternative of 6.6-0001 (6.3-0001, 6.4-0001, 6.5-0001, 6.6-0001, 6.7-0001,
//! 6.8-0001, 6.9-0001). It takes the sections [`crate::document::parse_document`] split out and returns each as an ordered list of
//! `key`/`value` pairs, **structurally checked**; the *values* stay text (their per-field encodings and meanings are the clauses of the
//! owning sections, read by the typed parsers: [`crate::interface`], [`crate::layout`], and so on).
//!
//! **Draft-tracking.** KISS-Contract is a Draft; expect rework when it moves. The golden document of KISS `contract_vectors.json`
//! (origin/main 73c341b5) is diffed in `tests/fields.rs`: it carries exactly one value per field, so every other value and every fault here
//! is written from the clause text and is this crate's own test.

use core::fmt;

use crate::document::Document;

/// One field line: `<key> = <value>` (6.11-0001).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field<'a> {
    /// The key, spelled as its field-schema clause names it.
    pub key: &'a str,
    /// The value text to the end of the line (not interpreted here).
    pub value: &'a str,
}

/// A contract document whose seven sections' field lines match their field schemas.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Contract<'a> {
    /// The `contract_version` of the header line.
    pub contract_version: u64,
    /// For each of the seven sections in order, its fields in the order the schema pins.
    pub sections: [Vec<Field<'a>>; 7],
}

impl<'a> Contract<'a> {
    /// The value of `key` in section `section` (`0..7`, in the order of `document::SECTION_NAMES`), if the contract carries it.
    #[must_use]
    pub fn get(&self, section: usize, key: &str) -> Option<&'a str> {
        self.sections
            .get(section)?
            .iter()
            .find(|f| f.key == key)
            .map(|f| f.value)
    }
}

/// A fault in a section's field lines. Each is a typed decline (6.1-0006, 6.11-0005); a reader never repairs.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum FieldError {
    /// The section's bytes are not UTF-8 (6.11-0001).
    NotUtf8 {
        /// The section number, `1` to `7`.
        section: usize,
    },
    /// A line that is not `<key> = <value>` with one space either side of the `=`, that holds a carriage return, or that is not
    /// LF-terminated (6.11-0001).
    BadLine {
        /// The section number, `1` to `7`.
        section: usize,
        /// The line, from 1.
        line: usize,
    },
    /// A required field is absent (6.1-0006).
    MissingField {
        /// The section number, `1` to `7`.
        section: usize,
        /// The key the schema expected at that point.
        key: &'static str,
    },
    /// A field whose key the schema does not list, or an extra line after the last field (6.1-0006).
    UnexpectedField {
        /// The section number, `1` to `7`.
        section: usize,
        /// The key.
        key: String,
    },
    /// A field of the schema that is not in the schema's order, or that repeats (6.11-0005).
    OutOfOrder {
        /// The section number, `1` to `7`.
        section: usize,
        /// The key.
        key: String,
    },
    /// The Dispatch section is neither the five geometry fields nor the single `dispatch_model = geometry-agnostic` line (6.6-0001).
    DispatchAlternatives,
}

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotUtf8 { section } => write!(f, "contract section {section}: not UTF-8"),
            Self::BadLine { section, line } => write!(
                f,
                "contract section {section}: line {line} is not `<key> = <value>`"
            ),
            Self::MissingField { section, key } => {
                write!(f, "contract section {section}: missing field `{key}`")
            }
            Self::UnexpectedField { section, key } => {
                write!(f, "contract section {section}: unexpected field `{key}`")
            }
            Self::OutOfOrder { section, key } => write!(
                f,
                "contract section {section}: field `{key}` is out of order or repeated"
            ),
            Self::DispatchAlternatives => write!(
                f,
                "contract dispatch: not exactly one of the two alternatives of 6.6-0001"
            ),
        }
    }
}

impl std::error::Error for FieldError {}

/// A field schema entry: the key and whether it is optional.
type Entry = (&'static str, bool);

const IDENTITY: &[Entry] = &[
    ("contract_kind", false),
    ("contract_version", false),
    ("kernel_name", false),
    ("revision_hash", false),
    ("accept_predicate", false),
    ("op_identity", false),
    ("target_capability", false),
];
const SEMANTICS: &[Entry] = &[
    ("semantics_kind", false),
    ("op_dag", false),
    ("human_annotation", true),
];
const INTERFACE: &[Entry] = &[
    ("entry_point", false),
    ("rank", false),
    ("positional_signature", false),
    ("launch_scalars", false),
    ("count_unit", false),
    ("in_place", false),
    ("alignment_bytes", false),
    ("output_layouts", true),
];
const DISPATCH_GEOMETRY: &[Entry] = &[
    ("invocation_domain", false),
    ("workgroup_sizing", false),
    ("count_to_grid", false),
    ("thread_mapping", false),
    ("addressing_rule", false),
];
const CAPABILITIES: &[Entry] = &[
    ("accept_predicate", false),
    ("supported_dtype_set", false),
    ("awkward_layout_strategy", false),
    ("in_place_eligible_variants", false),
    ("index_width", false),
    ("determinism_class", false),
    ("precision_class", false),
    ("cost", false),
];
const GUARANTEES: &[Entry] = &[
    ("reference_function", false),
    ("per_backend_ulp_tiers", false),
    ("determinism_class", false),
    ("math_fidelity", false),
    ("accumulation_type", false),
    ("bit_stability", false),
    ("audited_status", false),
    ("cost_provenance", false),
];
const PROVENANCE: &[Entry] = &[
    ("kernel_source", false),
    ("revision_base", false),
    ("revision_hash", false),
    ("cost_provenance", false),
    ("negotiation_metadata", false),
];

/// The geometry-agnostic sentinel of 6.6-0001 and 6.6-0007: the Dispatch section's sole field line.
pub const GEOMETRY_AGNOSTIC: Field<'static> = Field {
    key: "dispatch_model",
    value: "geometry-agnostic",
};

fn lines(section: usize, bytes: &[u8]) -> Result<Vec<Field<'_>>, FieldError> {
    let text = core::str::from_utf8(bytes).map_err(|_| FieldError::NotUtf8 { section })?;
    if text.is_empty() {
        return Ok(Vec::new());
    }
    let body = text.strip_suffix('\n').ok_or(FieldError::BadLine {
        section,
        line: text.lines().count(),
    })?;
    body.split('\n')
        .enumerate()
        .map(|(i, line)| {
            let bad = FieldError::BadLine {
                section,
                line: i + 1,
            };
            if line.contains('\r') {
                return Err(bad);
            }
            let (key, value) = line.split_once(" = ").ok_or(bad.clone())?;
            if key.is_empty() || key.contains(' ') {
                return Err(bad);
            }
            Ok(Field { key, value })
        })
        .collect()
}

/// Match `fields` against `schema`: every required key once, in order, the optional ones in their place or absent, nothing else.
fn check(section: usize, fields: &[Field<'_>], schema: &[Entry]) -> Result<(), FieldError> {
    let mut at = 0;
    for &(key, optional) in schema {
        match fields.get(at) {
            Some(f) if f.key == key => at += 1,
            _ if optional => {}
            Some(f) if !schema.iter().any(|(k, _)| *k == f.key) => {
                return Err(FieldError::UnexpectedField {
                    section,
                    key: f.key.to_owned(),
                });
            }
            // A listed key where another is expected: out of order when the expected one comes later, missing when it never does.
            Some(f) if fields[at + 1..].iter().any(|g| g.key == key) => {
                return Err(FieldError::OutOfOrder {
                    section,
                    key: f.key.to_owned(),
                });
            }
            Some(_) | None => return Err(FieldError::MissingField { section, key }),
        }
    }
    match fields.get(at) {
        None => Ok(()),
        Some(f) if schema.iter().any(|(k, _)| *k == f.key) => Err(FieldError::OutOfOrder {
            section,
            key: f.key.to_owned(),
        }),
        Some(f) => Err(FieldError::UnexpectedField {
            section,
            key: f.key.to_owned(),
        }),
    }
}

/// Read the field lines of a framed document's seven sections and check each against its field schema (6.11-0005, 6.3-0001 to 6.9-0001):
/// the exact keys in the exact order, an absent optional field omitted rather than blank, the Dispatch section exactly one of its two
/// alternatives (6.6-0001).
///
/// # Errors
/// A [`FieldError`] naming the first fault, section by section in order.
pub fn parse_contract<'a>(doc: &Document<'a>) -> Result<Contract<'a>, FieldError> {
    let mut sections: [Vec<Field<'a>>; 7] = Default::default();
    for (i, bytes) in doc.sections.iter().enumerate() {
        sections[i] = lines(i + 1, bytes)?;
    }
    let schemas: [&[Entry]; 7] = [
        IDENTITY,
        SEMANTICS,
        INTERFACE,
        &[],
        CAPABILITIES,
        GUARANTEES,
        PROVENANCE,
    ];
    for (i, schema) in schemas.iter().enumerate() {
        if i != 3 {
            check(i + 1, &sections[i], schema)?;
        }
    }
    // 6.6-0001: the five geometry fields in order, or the sole sentinel line, and nothing else.
    let dispatch = &sections[3];
    if dispatch
        .first()
        .is_some_and(|f| f.key == GEOMETRY_AGNOSTIC.key)
    {
        if dispatch.as_slice() != [GEOMETRY_AGNOSTIC] {
            return Err(FieldError::DispatchAlternatives);
        }
    } else {
        check(4, dispatch, DISPATCH_GEOMETRY).map_err(|e| match e {
            FieldError::MissingField { .. }
            | FieldError::OutOfOrder { .. }
            | FieldError::UnexpectedField { .. } => FieldError::DispatchAlternatives,
            other => other,
        })?;
    }
    Ok(Contract {
        contract_version: doc.contract_version,
        sections,
    })
}
