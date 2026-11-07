// SPDX-License-Identifier: MIT OR Apache-2.0
//! Hello world: describe a region (`relu(add(a, b))`), read its external inputs, key it, and encode an op's attributes to canonical bytes.
//! No graph, no device: this is the grammar a consumer sends and a provider reads.

use fireline_grammar::{OpAttrs, OpTag, PatternNode};

fn bind(index: u8) -> PatternNode {
    PatternNode::Bind { index }
}

fn op(tag: OpTag, operands: Vec<PatternNode>) -> PatternNode {
    PatternNode::Op {
        op: tag,
        operands,
        attrs: OpAttrs::default(),
    }
}

fn main() {
    // relu(add(input0, input1)): the region's two external inputs are the `Bind` leaves.
    let region = op(OpTag::Relu, vec![op(OpTag::Add, vec![bind(0), bind(1)])]);
    println!("external inputs: {:?}", region.bind_indices());

    // The structural identity of an op's attributes: equal attributes give equal keys.
    let attrs = OpAttrs::default();
    assert_eq!(attrs.region_key(), OpAttrs::default().region_key());

    // The canonical wire bytes of those attributes for one op (a u32 little-endian length, then the body).
    let bytes = attrs
        .to_canonical_bytes(OpTag::Add)
        .expect("a plain Add has no unresolved attributes");
    println!(
        "canonical OpAttrs for Add: {} bytes, length prefix {:?}",
        bytes.len(),
        &bytes[..4]
    );
}
