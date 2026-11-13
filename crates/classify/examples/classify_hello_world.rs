// SPDX-License-Identifier: MIT OR Apache-2.0
//! Hello world: pack six FP6 (`f6e2m3`) codes, read them back, decode them, and see that an MX element is never an accumulator.

use fireline_classify::{MxElement, check_accumulator};

fn main() {
    let e = MxElement::F6E2M3;
    // Codes 0b000001 (0.125, the smallest subnormal) to 0b011111 (7.5, the largest finite value).
    let codes = [
        0b000_001, 0b000_111, 0b001_000, 0b010_000, 0b011_000, 0b011_111,
    ];
    let packed = e.pack(&codes).expect("every code fits 6 bits");
    println!(
        "{} codes -> {} bytes: {packed:02X?}",
        codes.len(),
        packed.len()
    );

    let back = e
        .unpack(&packed, codes.len())
        .expect("a buffer we just packed");
    let values: Vec<f64> = back
        .iter()
        .map(|&c| e.decode(c).expect("in range"))
        .collect();
    println!("values: {values:?}");

    // An MX element is a storage and operand dtype only (KISS-CLASSIFY-6.1-0016).
    match check_accumulator(e.token()) {
        Err(err) => println!("as an accumulator: {err}"),
        Ok(()) => unreachable!("an MX element is never an accumulator"),
    }
}
