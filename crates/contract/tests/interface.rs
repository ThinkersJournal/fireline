// SPDX-License-Identifier: MIT OR Apache-2.0
//! The Interface `positional_signature` (KISS-CONTRACT-6.11-0006, 6.5-0004a/-0004c/-0004d, 6.5-0005, 6.5-0013), written from the Draft's
//! clause text (KISS origin/main 73c341b5). KISS's `contract_vectors.json` has no vector for any of this (its golden document carries an
//! empty signature), so these cases are this crate's own. Draft-tracking: expect rework when the Draft moves.

use fireline_contract::interface::*;

const PTR_IN: &str = "(ptr, 0, a, f32*, f32, scalar)";
const PTR_OUT: &str = "(ptr, 0, out, f32*, f32, scalar)";
const EXTENTS0: &str = "(ls, 1, extents0, i64, -, rank)";
const EXTENTS1: &str = "(ls, 1, extents1, i64, -, rank)";
const STRIDES0: &str = "(ls, 2, strides0, i64, -, rank)";
const STRIDES1: &str = "(ls, 2, strides1, i64, -, rank)";
const N: &str = "(ls, 3, n, i32, -, scalar)";
const OFF0: &str = "(ls, 4, off0, i64, -, scalar)";
const IDESC0: &str = "(idesc, 5, idx0, idx, u32, 2@idx_extents0)";
const IDX_EXT0: &str = "(ls, 5, idx_extents0, i64, -, rank)";
const WS_PTR: &str = "(ls, 6, ws_ptr, ptr, -, scalar)";
const WS_BYTES: &str = "(ls, 7, ws_bytes, i64, -, scalar)";
const PARAM0: &str = "(ls, 8, param0, f32, -, scalar)";

fn sig(parts: &[&str]) -> String {
    format!("[{}]", parts.join(", "))
}

#[test]
fn an_empty_signature_parses_as_the_golden_documents_does() {
    let s = Signature::parse("[]").unwrap();
    assert!(s.args.is_empty() && s.vocabulary().is_empty());
}

// 6.11-0006: the tuple slots, and the class-5 index descriptor with its `<rank>@<binding>` slot (the clause's own example, 2@idx_extents0).
#[test]
fn descriptors_parse_into_their_slots() {
    let s = Signature::parse(&sig(&[
        PTR_IN, EXTENTS0, N, IDESC0, IDX_EXT0, WS_PTR, WS_BYTES, PARAM0,
    ]))
    .unwrap();
    assert_eq!(s.args.len(), 8);
    assert_eq!(
        s.args[0],
        ArgDescriptor {
            kind: ArgKind::Ptr,
            launch_class: 0,
            name: "a".into(),
            type_token: "f32*".into(),
            dtype: Some("f32".into()),
            array_len: ArrayLen::Scalar,
        }
    );
    assert_eq!(s.args[1].dtype, None, "`-` is no dtype");
    assert_eq!(s.args[1].array_len, ArrayLen::Rank);
    assert_eq!(
        s.args[3].array_len,
        ArrayLen::Index {
            rank: 2,
            binding: "idx_extents0".into()
        }
    );
}

// 6.5-0004a: the vocabulary is the symbols THIS signature declares, not the whole class table.
#[test]
fn the_vocabulary_is_what_the_signature_declares() {
    let packed = Signature::parse(&sig(&[PTR_IN, PTR_OUT, EXTENTS0, N])).unwrap();
    let v: Vec<_> = packed.vocabulary().into_iter().collect();
    assert_eq!(v, ["extents0", "n"]);
    assert!(
        !packed.vocabulary().contains("strides0"),
        "a fully packed cell does not declare strides"
    );
    assert!(!packed.vocabulary().contains("ws_bytes"));
    let strided = Signature::parse(&sig(&[
        EXTENTS0, STRIDES0, N, OFF0, WS_PTR, WS_BYTES, PARAM0,
    ]))
    .unwrap();
    assert_eq!(strided.vocabulary().len(), 7);
}

// 6.5-0004a, -0004c, -0004d: the classes appear in the pinned order and each class's entries are grouped, in operand order.
#[test]
fn the_pinned_class_order_is_enforced() {
    // classes 1,2,3,4 in order, entries ascending by operand index
    assert!(Signature::parse(&sig(&[EXTENTS0, EXTENTS1, STRIDES0, STRIDES1, N, OFF0])).is_ok());
    // class 3 before class 2
    assert_eq!(
        Signature::parse(&sig(&[EXTENTS0, N, STRIDES0])),
        Err(SignatureError::ClassOutOfOrder(2))
    );
    // interleaved: extents0, strides0, extents1 (class 1 entries are not grouped)
    assert_eq!(
        Signature::parse(&sig(&[EXTENTS0, STRIDES0, EXTENTS1])),
        Err(SignatureError::ClassOutOfOrder(2))
    );
    // within a class, operands ascend
    assert_eq!(
        Signature::parse(&sig(&[EXTENTS1, EXTENTS0])),
        Err(SignatureError::OperandOutOfOrder(1))
    );
    assert_eq!(
        Signature::parse(&sig(&[EXTENTS0, EXTENTS0])),
        Err(SignatureError::OperandOutOfOrder(1))
    );
    // the workspace pointer comes before its byte size
    assert_eq!(
        Signature::parse(&sig(&[WS_BYTES, WS_PTR])),
        Err(SignatureError::ClassOutOfOrder(1))
    );
    // operand pointers are not placed by the Draft: accepted among the scalars
    assert!(Signature::parse(&sig(&[EXTENTS0, PTR_IN, N, PTR_OUT])).is_ok());
}

// 6.5-0004d, 6.5-0013: within class 5 the index descriptor precedes the idx_extents it binds.
#[test]
fn the_index_descriptor_precedes_its_extents() {
    assert!(Signature::parse(&sig(&[IDESC0, IDX_EXT0])).is_ok());
    assert_eq!(
        Signature::parse(&sig(&[IDX_EXT0, IDESC0])),
        Err(SignatureError::IndexExtentsBeforeDescriptor(0))
    );
    assert_eq!(
        Signature::parse(&sig(&[IDX_EXT0])),
        Err(SignatureError::IndexExtentsBeforeDescriptor(0))
    );
    // a descriptor must name a class-5 idx_extents{i} binding, carry the literal `idx`, and a numeric rank
    for bad in [
        "(idesc, 5, idx0, idx, u32, 2@extents0)",
        "(idesc, 5, idx0, f32, u32, 2@idx_extents0)",
        "(idesc, 5, idx0, idx, u32, two@idx_extents0)",
        "(idesc, 5, idx0, idx, u32, rank)",
    ] {
        assert_eq!(
            Signature::parse(&sig(&[bad])),
            Err(SignatureError::BadIndexDescriptor(0)),
            "{bad}"
        );
    }
}

// 6.11-0006: a descriptor outside the layout is a typed decline, never repaired.
#[test]
fn malformed_descriptors_decline() {
    assert_eq!(Signature::parse("(a)"), Err(SignatureError::NotAnArray));
    assert_eq!(
        Signature::parse("[(ptr, 0, a)]"),
        Err(SignatureError::BadDescriptor(0))
    );
    assert_eq!(
        Signature::parse("[ptr, 0, a, f32*, f32, scalar]"),
        Err(SignatureError::BadDescriptor(0))
    );
    assert_eq!(
        Signature::parse(&sig(&[PTR_IN, "(ptr, 0, b, f32*)"])),
        Err(SignatureError::BadDescriptor(1))
    );
    assert_eq!(
        Signature::parse("[(buf, 0, a, f32*, f32, scalar)]"),
        Err(SignatureError::UnknownArgKind(0))
    );
    // class fits kind: ptr is 0, ls is 1..=8, idesc is 5
    for bad in [
        "(ptr, 1, a, f32*, f32, scalar)",
        "(ls, 0, n, i32, -, scalar)",
        "(ls, 9, n, i32, -, scalar)",
        "(ls, 10, n, i32, -, scalar)",
        "(ls, x, n, i32, -, scalar)",
    ] {
        assert_eq!(
            Signature::parse(&sig(&[bad])),
            Err(SignatureError::BadLaunchClass(0)),
            "{bad}"
        );
    }
    // the symbol must belong to its class and have the class's array shape
    for bad in [
        "(ls, 2, extents0, i64, -, rank)",
        "(ls, 1, extents0, i64, -, scalar)",
        "(ls, 3, n, i32, -, rank)",
        "(ls, 3, wobble, i32, -, scalar)",
        "(ls, 4, off, i64, -, scalar)",
        "(ls, 1, extents, i64, -, rank)",
        "(ptr, 0, a, f32*, f32, rank)",
        "(ls, 3, n, i32, -, many)",
    ] {
        assert_eq!(
            Signature::parse(&sig(&[bad])),
            Err(SignatureError::ClassNameMismatch(0)),
            "{bad}"
        );
    }
}

// 6.5-0005 (FDX V11): a cell that is not fully packed MUST carry strides{i} for its operands; a packed cell declares none.
#[test]
fn a_strided_cell_carries_its_strides() {
    let two = 2;
    let ok = Signature::parse(&sig(&[EXTENTS0, EXTENTS1, STRIDES0, STRIDES1, N])).unwrap();
    assert_eq!(ok.check_strides(two, false), Ok(()));
    let missing = Signature::parse(&sig(&[EXTENTS0, EXTENTS1, STRIDES0, N])).unwrap();
    assert_eq!(
        missing.check_strides(two, false),
        Err(SignatureError::StridesMissing(1))
    );
    let none = Signature::parse(&sig(&[EXTENTS0, EXTENTS1, N])).unwrap();
    assert_eq!(
        none.check_strides(two, false),
        Err(SignatureError::StridesMissing(0))
    );
    // a fully packed cell declares no strides, and declaring them is a fault (class 2 is present exactly when not packed)
    assert_eq!(none.check_strides(two, true), Ok(()));
    assert_eq!(
        ok.check_strides(two, true),
        Err(SignatureError::StridesNotAllowed)
    );
    // zero operands: nothing to carry
    assert_eq!(
        Signature::parse("[]").unwrap().check_strides(0, false),
        Ok(())
    );
}
