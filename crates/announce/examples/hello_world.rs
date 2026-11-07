// SPDX-License-Identifier: MIT OR Apache-2.0
//! Hello world: a consumer and a provider each advertise, then negotiate. No device, no kernel.

use fireline_announce::{
    Role, SEAM_CAP_DLPACK_EXT_V1, SEAM_CAP_PROVISION_ON_REQUEST, SeamHello, negotiate_with_role,
};

fn main() {
    let consumer = SeamHello::profile_v1(SEAM_CAP_DLPACK_EXT_V1);
    let provider = SeamHello::profile_v1(SEAM_CAP_DLPACK_EXT_V1 | SEAM_CAP_PROVISION_ON_REQUEST);
    // Over a wire the provider's envelope arrives as 56 bytes; parse and validate it first.
    let remote =
        SeamHello::from_wire_bytes(&provider.to_wire_bytes()).expect("a well-formed envelope");
    let n = negotiate_with_role(&consumer, &remote, Role::Consumer).expect("both speak profile 1");
    println!("profile {}", n.profile);
    println!(
        "provider builds kernels on request: {}",
        n.capabilities & SEAM_CAP_PROVISION_ON_REQUEST != 0
    );
}
