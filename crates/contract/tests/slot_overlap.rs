// SPDX-License-Identifier: MIT OR Apache-2.0
//! The bind-time slot-overlap check (KISS-CONTRACT-6.5-0019, 6.12-0002): the six cases of KISS `conformance/corpus/slot_overlap_vectors.json`
//! (origin/main 73c341b5), plus this crate's own cases for the hull arithmetic the vectors do not reach (sub-byte dtypes, swizzle windows,
//! reversed strides, empty slots, unbound symbols). Draft-tracking: expect rework when the Draft moves.

use fireline_contract::interface::Signature;
use fireline_contract::layout::*;
use fireline_contract::{BindDecline, Context, Environment};

/// (name, bundle text, `n`, expected code, `pinned_v1_conservative`), verbatim from the corpus. The environment is the corpus's: rank 2,
/// `extents0` [3,4], `strides0` [4,1], no workspace.
const CASES: &[(&str, &str, i64, &str, bool)] = &[
    (
        "touching_hulls_ok",
        "bundle(a:f32[4]=dense@0,b:f32[4]=dense@16)",
        12,
        "ok",
        false,
    ),
    (
        "overlapping_hulls",
        "bundle(a:f32[4]=dense@0,b:f32[4]=dense@15)",
        12,
        "slot_overlap",
        false,
    ),
    (
        "verdict_follows_environment_ok",
        "bundle(a:f32[n]=dense@0,b:f32[4]=dense@32)",
        8,
        "ok",
        false,
    ),
    (
        "verdict_follows_environment_overlap",
        "bundle(a:f32[n]=dense@0,b:f32[4]=dense@32)",
        9,
        "slot_overlap",
        false,
    ),
    (
        "empty_slot_overlaps_nothing",
        "bundle(a:f32[0]=dense@8,b:f32[4]=dense@0)",
        12,
        "ok",
        false,
    ),
    (
        "interleaved_valid_refused_v1",
        "bundle(a:f32[1,2]=strided(2,2)@0,b:f32[1]=dense@4)",
        12,
        "slot_overlap",
        true,
    ),
];

const SIGNATURE: &str = "[(ls, 1, extents0, i64, -, rank), (ls, 2, strides0, i64, -, rank), (ls, 3, n, i32, -, scalar)]";

fn is_dtype(t: &str) -> bool {
    bits(t).is_some()
}

fn bits(t: &str) -> Option<u32> {
    Some(match t {
        "f32" | "i32" | "u32" => 32,
        "bf16" | "f16" => 16,
        "i8" | "u8" => 8,
        "i4" | "u4" | "f4e2m1" => 4,
        "f6e2m3" => 6,
        "b1" => 1,
        _ => return None,
    })
}

fn slots(text: &str) -> Vec<Slot> {
    let c = Context::of_interface(2, &Signature::parse(SIGNATURE).unwrap());
    match parse_layout(
        text,
        &Env {
            rank: 2,
            n_inputs: 2,
            context: &c,
            is_dtype: &is_dtype,
        },
    )
    .unwrap_or_else(|e| panic!("{text}: {e}"))
    {
        Layout::Bundle(s) => s,
        other => panic!("not a bundle: {other}"),
    }
}

fn env(n: i64) -> Environment {
    let mut e = Environment::new();
    e.set_scalar("n", n)
        .set_array("extents0", &[3, 4])
        .set_array("strides0", &[4, 1]);
    e
}

fn check(text: &str, n: i64) -> Result<(), BindDecline> {
    check_slot_overlap(&slots(text), &env(n), &bits)
}

// The six KISS cases: each verdict is the corpus's, and the interleaved case is the pinned conservative v1 refusal.
#[test]
fn every_kiss_case_has_its_verdict() {
    for (name, text, n, expect, pinned) in CASES {
        let got = check(text, *n);
        match *expect {
            "ok" => assert_eq!(got, Ok(()), "{name}"),
            "slot_overlap" => {
                let e = got.expect_err(name);
                assert_eq!(e.code(), "slot_overlap", "{name}: {e}");
            }
            other => panic!("unknown expectation {other}"),
        }
        assert_eq!(*pinned, *name == "interleaved_valid_refused_v1", "{name}");
    }
    assert_eq!(CASES.len(), 6);
}

// 6.5-0019: the first overlapping pair, in declaration order, names the decline.
#[test]
fn the_first_overlapping_pair_is_named() {
    let t = "bundle(a:f32[4]=dense@0,b:f32[4]=dense@8,c:f32[4]=dense@16)";
    assert_eq!(
        check(t, 1),
        Err(BindDecline::SlotOverlap("a".into(), "b".into()))
    );
    let t = "bundle(a:f32[4]=dense@0,b:f32[4]=dense@16,c:f32[4]=dense@20)";
    assert_eq!(
        check(t, 1),
        Err(BindDecline::SlotOverlap("b".into(), "c".into()))
    );
}

// 6.5-0019: a sub-byte dtype rounds the hull outward (start down, end up).
#[test]
fn sub_byte_hulls_round_outward() {
    // 3 x i4 = 12 bits -> bytes [0,2); the next slot at byte 1 overlaps, at byte 2 touches
    assert!(check("bundle(a:i4[3]=dense@0,b:u8[1]=dense@2)", 1).is_ok());
    assert!(check("bundle(a:i4[3]=dense@0,b:u8[1]=dense@1)", 1).is_err());
    // f6e2m3: 5 elements = 30 bits -> 4 bytes
    assert!(check("bundle(a:f6e2m3[5]=dense@0,b:u8[1]=dense@4)", 1).is_ok());
    assert!(check("bundle(a:f6e2m3[5]=dense@0,b:u8[1]=dense@3)", 1).is_err());
    // b1: 9 bits -> 2 bytes
    assert!(check("bundle(a:b1[9]=dense@0,b:u8[1]=dense@2)", 1).is_ok());
    assert!(check("bundle(a:b1[9]=dense@0,b:u8[1]=dense@1)", 1).is_err());
}

// 6.5-0019: an empty slot (a zero extent) has no bytes and overlaps nothing, wherever it sits.
#[test]
fn an_empty_slot_overlaps_nothing() {
    assert!(check("bundle(a:f32[0]=dense@0,b:f32[4]=dense@0)", 1).is_ok());
    assert!(check("bundle(a:f32[4]=dense@0,b:f32[n]=dense@4)", 0).is_ok());
    assert!(check("bundle(a:f32[4]=dense@0,b:f32[n]=dense@4)", 1).is_err());
}

// A swizzle rounds the end of the hull up to its window of 2^(B+M) elements.
#[test]
fn a_swizzle_rounds_the_hull_up_to_its_window() {
    // (3,3):(3,1) covers elements 0..9; swz(1,2,0) has a window of 8 elements, so the hull ends at element 16 = byte 16 of u8
    assert!(
        check(
            "bundle(a:u8[3,3]=swz(1,2,0)((3,3):(3,1))@0,b:u8[1]=dense@16)",
            1
        )
        .is_ok()
    );
    assert!(
        check(
            "bundle(a:u8[3,3]=swz(1,2,0)((3,3):(3,1))@0,b:u8[1]=dense@15)",
            1
        )
        .is_err()
    );
    // without the swizzle the hull ends at element 9
    assert!(check("bundle(a:u8[3,3]=(3,3):(3,1)@0,b:u8[1]=dense@9)", 1).is_ok());
    assert!(check("bundle(a:u8[3,3]=(3,3):(3,1)@0,b:u8[1]=dense@8)", 1).is_err());
}

// A strided slot may have a negative (reversed) stride, written `0-1` (Dispatch has no unary minus): the hull reaches below the offset.
#[test]
fn a_reversed_stride_extends_the_hull_below_the_offset() {
    // extents [3] stride -1 at offset 8: elements at bytes 8, 7, 6 -> hull [6, 9)
    assert!(check("bundle(a:u8[3]=strided(0-1)@8,b:u8[1]=dense@5)", 1).is_ok());
    assert!(check("bundle(a:u8[3]=strided(0-1)@8,b:u8[1]=dense@6)", 1).is_err());
    assert!(check("bundle(a:u8[3]=strided(0-1)@8,b:u8[1]=dense@9)", 1).is_ok());
    assert!(check("bundle(a:u8[3]=strided(0-1)@8,b:u8[1]=dense@8)", 1).is_err());
}

// 6.12-0002: the verdict depends on the bound environment, so an unbound symbol declines at bind time.
#[test]
fn an_unbound_symbol_declines_at_bind_time() {
    let s = slots("bundle(a:f32[n]=dense@0,b:f32[4]=dense@64)");
    assert_eq!(
        check_slot_overlap(&s, &Environment::new(), &bits),
        Err(BindDecline::UnboundSymbol("n".into()))
    );
    // an unknown dtype width is a typed decline, not a silent default
    let s = slots("bundle(a:f32[4]=dense@0)");
    assert_eq!(
        check_slot_overlap(&s, &env(1), &|_| None),
        Err(BindDecline::SlotDtypeWidth("f32".into()))
    );
}
