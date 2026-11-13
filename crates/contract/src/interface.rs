// SPDX-License-Identifier: MIT OR Apache-2.0
//! The Interface **`positional_signature`** of KISS-Contract (6.11-0006): parse its argument descriptors, check the pinned launch-scalar
//! class order (6.5-0004a, -0004c, -0004d) and the index-operand descriptor rule (6.5-0013), check that a cell that is not fully packed
//! carries its `strides{i}` (6.5-0005, FDX V11), and derive the contract's own **launch-scalar vocabulary** (6.5-0004a).
//!
//! **Draft-tracking; no vectors.** KISS-Contract is a Draft; expect rework when it moves. KISS's `contract_vectors.json` has an empty
//! `positional_signature` in its golden document and no vector for these clauses, so everything here is written from the clause text and
//! the tests are this crate's own. Where the text is silent (below) the choice is this crate's and is marked.
//!
//! * The text pins the order of the launch-scalar classes 1 to 8 but not where operand pointers (`ptr`, class 0) sit among them:
//!   pointers are accepted anywhere (this crate's choice).
//! * "Canonical operand order" (6.5-0004d) is taken to be ascending `{i}` in the symbol names (this crate's choice).

use core::fmt;
use std::collections::BTreeSet;

/// What kind of argument a descriptor is (6.11-0006).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgKind {
    /// `ptr`: an operand pointer; its launch class is `0`.
    Ptr,
    /// `ls`: a launch scalar of class 1 to 8.
    Ls,
    /// `idesc`: the class-5 index-operand descriptor.
    Idesc,
}

/// The `array_len_kind` slot (6.11-0006).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArrayLen {
    /// `scalar`.
    Scalar,
    /// `rank`: a rank-length array.
    Rank,
    /// For an `idesc`: `<rank>@<binding>`, the index operand's rank and the class-5 `idx_extents{i}` it binds.
    Index {
        /// The index operand's rank.
        rank: usize,
        /// The `idx_extents{i}` symbol the descriptor binds.
        binding: String,
    },
}

/// One argument descriptor `(<arg_kind>, <launch_class>, <arg_name>, <type_token>, <dtype>, <array_len_kind>)` (6.11-0006).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArgDescriptor {
    /// `ptr`, `ls` or `idesc`.
    pub kind: ArgKind,
    /// `0` for a pointer, else the launch-scalar class `1` to `8`.
    pub launch_class: u8,
    /// The argument name; for a launch scalar it is the symbol (`extents0`, `n`, ...).
    pub name: String,
    /// The type token.
    pub type_token: String,
    /// The Classify dtype token, or `None` for `-` (not a typed pointer).
    pub dtype: Option<String>,
    /// Scalar, rank-length array, or an index descriptor's `<rank>@<binding>`.
    pub array_len: ArrayLen,
}

/// A fault in a `positional_signature`. Each is a typed decline; never a repair.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SignatureError {
    /// The value is not `[` ... `]` with elements separated by `, ` (6.11-0001).
    NotAnArray,
    /// Descriptor number `index` (from 0) is not a parenthesised tuple of exactly six tokens (6.11-0006).
    BadDescriptor(usize),
    /// Descriptor `index` has an `arg_kind` other than `ptr`, `ls`, `idesc`.
    UnknownArgKind(usize),
    /// Descriptor `index` has a launch class that does not fit its kind (`ptr` is `0`, `ls` is `1..=8`, `idesc` is `5`).
    BadLaunchClass(usize),
    /// Descriptor `index` names a symbol that does not belong to its launch class, or has the wrong `array_len_kind` for it (6.5-0004a).
    ClassNameMismatch(usize),
    /// Descriptor `index` is an `idesc` whose `type_token` is not `idx`, or whose `array_len_kind` is not `<rank>@<binding>` with
    /// a class-5 `idx_extents{i}` binding (6.11-0006).
    BadIndexDescriptor(usize),
    /// Descriptor `index` is a launch scalar whose class is lower than an earlier one: the classes are out of the pinned order, or a
    /// class's entries are not grouped (6.5-0004a, -0004c, -0004d).
    ClassOutOfOrder(usize),
    /// Descriptor `index` is a per-operand scalar whose `{i}` does not strictly ascend within its class (canonical operand order,
    /// 6.5-0004d).
    OperandOutOfOrder(usize),
    /// Descriptor `index` is an `idx_extents{i}` scalar with no earlier `idesc` binding it (6.5-0004d: the descriptor precedes).
    IndexExtentsBeforeDescriptor(usize),
    /// A cell that is not fully packed omits `strides{operand}` (6.5-0005).
    StridesMissing(usize),
    /// A fully packed cell declares `strides{i}`: class 2 is present exactly when the cell is not fully packed (6.5-0004a).
    StridesNotAllowed,
}

impl fmt::Display for SignatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAnArray => write!(f, "positional_signature: not an array"),
            Self::BadDescriptor(i) => write!(
                f,
                "positional_signature: descriptor {i} is not a six-token tuple"
            ),
            Self::UnknownArgKind(i) => write!(
                f,
                "positional_signature: descriptor {i} has an unknown arg_kind"
            ),
            Self::BadLaunchClass(i) => write!(
                f,
                "positional_signature: descriptor {i} has a launch class that does not fit its kind"
            ),
            Self::ClassNameMismatch(i) => write!(
                f,
                "positional_signature: descriptor {i} does not match its launch class"
            ),
            Self::BadIndexDescriptor(i) => write!(
                f,
                "positional_signature: descriptor {i} is a malformed index descriptor"
            ),
            Self::ClassOutOfOrder(i) => write!(
                f,
                "positional_signature: descriptor {i} is out of the pinned class order"
            ),
            Self::OperandOutOfOrder(i) => write!(
                f,
                "positional_signature: descriptor {i} is out of canonical operand order"
            ),
            Self::IndexExtentsBeforeDescriptor(i) => write!(
                f,
                "positional_signature: descriptor {i} precedes its index descriptor"
            ),
            Self::StridesMissing(o) => write!(
                f,
                "positional_signature: strides{o} is missing for a cell that is not fully packed"
            ),
            Self::StridesNotAllowed => write!(
                f,
                "positional_signature: strides declared for a fully packed cell"
            ),
        }
    }
}

impl std::error::Error for SignatureError {}

/// A parsed `positional_signature`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signature {
    /// The argument descriptors, in order.
    pub args: Vec<ArgDescriptor>,
}

/// The class a launch-scalar symbol name belongs to, its operand index if it has one, and whether it is a rank-length array
/// (6.5-0004a). `None` for a name outside the class table.
fn classify_symbol(name: &str) -> Option<(u8, Option<usize>, bool)> {
    let indexed = |prefix: &str| {
        name.strip_prefix(prefix)
            .filter(|d| !d.is_empty() && d.bytes().all(|b| b.is_ascii_digit()))
            .and_then(|d| d.parse::<usize>().ok())
    };
    if let Some(i) = indexed("extents") {
        Some((1, Some(i), true))
    } else if let Some(i) = indexed("strides") {
        Some((2, Some(i), true))
    } else if name == "n" {
        Some((3, None, false))
    } else if let Some(i) = indexed("off") {
        Some((4, Some(i), false))
    } else if let Some(i) = indexed("idx_extents") {
        Some((5, Some(i), true))
    } else if name == "ws_ptr" {
        Some((6, None, false))
    } else if name == "ws_bytes" {
        Some((7, None, false))
    } else {
        indexed("param").map(|i| (8, Some(i), false))
    }
}

fn token(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|b| (0x21..=0x7E).contains(&b) && !matches!(b, b',' | b'(' | b')' | b'[' | b']'))
}

impl Signature {
    /// Parse the value of a `positional_signature` line (6.11-0006) and check the class order (6.5-0004a, -0004c, -0004d) and the
    /// index-descriptor rule (6.5-0013, -0004d).
    ///
    /// # Errors
    /// A [`SignatureError`] naming the first fault.
    pub fn parse(value: &str) -> Result<Self, SignatureError> {
        let inner = value
            .strip_prefix('[')
            .and_then(|v| v.strip_suffix(']'))
            .ok_or(SignatureError::NotAnArray)?;
        let mut args = Vec::new();
        if !inner.is_empty() {
            // Tokens hold no parenthesis, so `), (` only ever separates two descriptors.
            let body = inner
                .strip_prefix('(')
                .and_then(|v| v.strip_suffix(')'))
                .ok_or(SignatureError::BadDescriptor(0))?;
            for (i, item) in body.split("), (").enumerate() {
                args.push(parse_descriptor(i, item)?);
            }
        }
        let sig = Self { args };
        sig.check_order()?;
        Ok(sig)
    }

    fn check_order(&self) -> Result<(), SignatureError> {
        let mut last_class = 0u8;
        let mut last_index: Option<usize> = None;
        let mut bound: BTreeSet<&str> = BTreeSet::new();
        for (i, a) in self.args.iter().enumerate() {
            if a.kind == ArgKind::Ptr {
                continue;
            }
            if a.launch_class < last_class {
                return Err(SignatureError::ClassOutOfOrder(i));
            }
            if a.launch_class != last_class {
                last_class = a.launch_class;
                last_index = None;
            }
            match a.kind {
                ArgKind::Idesc => {
                    if let ArrayLen::Index { binding, .. } = &a.array_len {
                        bound.insert(binding);
                    }
                }
                _ => {
                    if let Some((_, Some(idx), _)) = classify_symbol(&a.name) {
                        if last_index.is_some_and(|l| idx <= l) {
                            return Err(SignatureError::OperandOutOfOrder(i));
                        }
                        last_index = Some(idx);
                    }
                    if a.launch_class == 5 && !bound.contains(a.name.as_str()) {
                        return Err(SignatureError::IndexExtentsBeforeDescriptor(i));
                    }
                }
            }
        }
        Ok(())
    }

    /// Check that the cell carries the signed strides it needs (6.5-0005, FDX V11): when the cell is **not** fully packed (per the
    /// KISS-Classify contiguous-layout predicate, which the caller evaluates) every operand `0..operands` has a `strides{i}` launch
    /// scalar; when it is fully packed none does (class 2 is present exactly when the cell is not fully packed, 6.5-0004a).
    ///
    /// # Errors
    /// [`SignatureError::StridesMissing`] or [`SignatureError::StridesNotAllowed`].
    pub fn check_strides(&self, operands: usize, fully_packed: bool) -> Result<(), SignatureError> {
        let have: BTreeSet<usize> = self
            .args
            .iter()
            .filter(|a| a.launch_class == 2)
            .filter_map(|a| classify_symbol(&a.name).and_then(|(_, i, _)| i))
            .collect();
        if fully_packed {
            return if have.is_empty() {
                Ok(())
            } else {
                Err(SignatureError::StridesNotAllowed)
            };
        }
        (0..operands)
            .find(|o| !have.contains(o))
            .map_or(Ok(()), |o| Err(SignatureError::StridesMissing(o)))
    }

    /// This contract's **launch-scalar symbol vocabulary** (6.5-0004a): the symbols its own signature declares, not the whole class
    /// table. A symbol of a class the Interface does not declare (`strides0` in a fully packed cell, `ws_bytes` without a workspace
    /// pointer) is outside it. Pointers and index descriptors are not launch-scalar symbols.
    #[must_use]
    pub fn vocabulary(&self) -> BTreeSet<String> {
        self.args
            .iter()
            .filter(|a| a.kind == ArgKind::Ls)
            .map(|a| a.name.clone())
            .collect()
    }
}

fn parse_descriptor(i: usize, item: &str) -> Result<ArgDescriptor, SignatureError> {
    let f: Vec<&str> = item.split(", ").collect();
    let [kind, class, name, type_token, dtype, array_len] = f[..] else {
        return Err(SignatureError::BadDescriptor(i));
    };
    if ![kind, class, name, type_token, dtype, array_len]
        .iter()
        .all(|t| token(t))
    {
        return Err(SignatureError::BadDescriptor(i));
    }
    let kind = match kind {
        "ptr" => ArgKind::Ptr,
        "ls" => ArgKind::Ls,
        "idesc" => ArgKind::Idesc,
        _ => return Err(SignatureError::UnknownArgKind(i)),
    };
    let launch_class: u8 = class
        .parse()
        .map_err(|_| SignatureError::BadLaunchClass(i))?;
    let class_ok = match kind {
        ArgKind::Ptr => launch_class == 0,
        ArgKind::Ls => (1..=8).contains(&launch_class),
        ArgKind::Idesc => launch_class == 5,
    };
    if !class_ok || class.len() > 1 {
        return Err(SignatureError::BadLaunchClass(i));
    }
    let dtype = (dtype != "-").then(|| dtype.to_owned());
    let array_len = match kind {
        ArgKind::Idesc => {
            let bad = SignatureError::BadIndexDescriptor(i);
            if type_token != "idx" {
                return Err(bad);
            }
            let (rank, binding) = array_len.split_once('@').ok_or(bad.clone())?;
            let rank = rank.parse::<usize>().map_err(|_| bad.clone())?;
            if !matches!(classify_symbol(binding), Some((5, Some(_), true))) {
                return Err(bad);
            }
            ArrayLen::Index {
                rank,
                binding: binding.to_owned(),
            }
        }
        _ => match array_len {
            "scalar" => ArrayLen::Scalar,
            "rank" => ArrayLen::Rank,
            _ => return Err(SignatureError::ClassNameMismatch(i)),
        },
    };
    if kind == ArgKind::Ls {
        let (c, _, is_array) = classify_symbol(name).ok_or(SignatureError::ClassNameMismatch(i))?;
        let want = if is_array {
            ArrayLen::Rank
        } else {
            ArrayLen::Scalar
        };
        if c != launch_class || array_len != want {
            return Err(SignatureError::ClassNameMismatch(i));
        }
    } else if kind == ArgKind::Ptr && array_len != ArrayLen::Scalar {
        return Err(SignatureError::ClassNameMismatch(i));
    }
    Ok(ArgDescriptor {
        kind,
        launch_class,
        name: name.to_owned(),
        type_token: type_token.to_owned(),
        dtype,
        array_len,
    })
}
