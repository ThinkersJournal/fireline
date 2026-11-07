//! Conformance for `fireline-announce`: the frozen layout, the wire codec, role-aware negotiation, the KISS vectors, and the rule that
//! a side may only advertise capability bits this crate has allocated. Public API only, so the same file is the contract for both sides.

use fireline_announce::*;
use std::mem::{align_of, offset_of, size_of};

fn hex(s: &str) -> Vec<u8> {
    s.split_whitespace()
        .map(|x| u8::from_str_radix(x, 16).expect("hex byte"))
        .collect()
}

// ---- Frozen layout ---------------------------------------------------------------------------------------------------

#[test]
fn seamhello_layout_is_frozen() {
    assert_eq!(size_of::<SeamHello>(), 56);
    assert_eq!(align_of::<SeamHello>(), 8);
    assert_eq!(offset_of!(SeamHello, magic), 0);
    assert_eq!(offset_of!(SeamHello, envelope_version), 4);
    assert_eq!(offset_of!(SeamHello, reserved), 5);
    assert_eq!(offset_of!(SeamHello, profiles_len), 8);
    assert_eq!(offset_of!(SeamHello, profiles), 10);
    assert_eq!(offset_of!(SeamHello, reserved1), 42);
    assert_eq!(offset_of!(SeamHello, capabilities), 48);
}

#[test]
fn the_magic_spells_seam_on_a_little_endian_wire() {
    assert_eq!(SEAM_MAGIC.to_le_bytes(), *b"SEAM");
    assert_eq!(SEAM_MAGIC, 0x4D41_4553);
}

#[test]
fn advertise_fills_every_field_and_zeroes_all_reserved_bytes() {
    let h = SeamHello::advertise(&[1, 2, 3], SEAM_CAP_DLPACK_EXT_V1);
    assert_eq!(h.magic, SEAM_MAGIC);
    assert_eq!(h.envelope_version, SEAM_ENVELOPE_VERSION);
    assert_eq!(h.profiles_len, 3);
    assert_eq!(&h.profiles[..4], &[1, 2, 3, 0]);
    assert_eq!(h.reserved, [0; 3]);
    assert_eq!(h.reserved1, [0; 6]);
    assert_eq!(h.capabilities, SEAM_CAP_DLPACK_EXT_V1);
    assert_eq!(
        SeamHello::profile_v1(7),
        SeamHello::advertise(&[PROFILE_V1], 7)
    );
}

// ---- Wire codec ------------------------------------------------------------------------------------------------------

#[test]
fn the_wire_bytes_are_the_kiss_reference_envelope() {
    // KISS `announce_vectors.json` frame `reference_envelope` (KISS-ANNOUNCE-6.1-0002), KISS 6780c09.
    let kiss = hex(
        "53 45 41 4D 01 00 00 00 01 00 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 3F 00 00 00 03 00 00 00",
    );
    let h = SeamHello::profile_v1(0x0000_0003_0000_003F);
    assert_eq!(h.to_wire_bytes().to_vec(), kiss);
    assert_eq!(SeamHello::from_wire_bytes(&kiss), Ok(h));
}

#[test]
fn wire_round_trip_for_a_range_of_envelopes() {
    for h in [
        SeamHello::advertise(&[], 0),
        SeamHello::profile_v1(u64::MAX),
        SeamHello::advertise(&[1, 2, 3, 65535], 0x8000_0000_0000_0001),
        SeamHello::advertise(&(1..=16).collect::<Vec<u16>>(), 5),
    ] {
        assert_eq!(
            SeamHello::from_wire_bytes(&h.to_wire_bytes()),
            Ok(h),
            "{h:?}"
        );
    }
}

#[test]
fn a_reader_hard_rejects_each_malformed_envelope_with_its_own_error() {
    let good = SeamHello::profile_v1(1).to_wire_bytes();
    assert_eq!(
        SeamHello::from_wire_bytes(&good[..55]),
        Err(SeamError::Truncated)
    );
    assert_eq!(SeamHello::from_wire_bytes(&[]), Err(SeamError::Truncated));
    let mut b = good;
    b[0] = 0;
    assert!(matches!(
        SeamHello::from_wire_bytes(&b),
        Err(SeamError::BadMagic(_))
    ));
    let mut b = good;
    b[4] = 2;
    assert_eq!(
        SeamHello::from_wire_bytes(&b),
        Err(SeamError::UnknownEnvelope(2))
    );
    for at in [5, 6, 7, 42, 47] {
        let mut b = good;
        b[at] = 1;
        assert_eq!(
            SeamHello::from_wire_bytes(&b),
            Err(SeamError::ReservedNonZero),
            "byte {at}"
        );
    }
    let mut b = good;
    b[8] = 17;
    assert_eq!(
        SeamHello::from_wire_bytes(&b),
        Err(SeamError::TooManyProfiles(17))
    );
    let mut b = SeamHello::advertise(&[1, 2], 0).to_wire_bytes();
    b[10..12].copy_from_slice(&2u16.to_le_bytes());
    b[12..14].copy_from_slice(&1u16.to_le_bytes());
    assert_eq!(SeamHello::from_wire_bytes(&b), Err(SeamError::NotAscending));
}

#[test]
fn validate_returns_exactly_the_live_profiles() {
    assert_eq!(
        SeamHello::advertise(&[1, 2], 0).validate(),
        Ok(&[1u16, 2][..])
    );
    assert_eq!(SeamHello::advertise(&[], 0).validate(), Ok(&[][..]));
}

// ---- Role-aware negotiation ------------------------------------------------------------------------------------------

#[test]
fn highest_mutually_supported_profile_wins_and_a_disjoint_set_hard_fails() {
    let a = SeamHello::advertise(&[1, 2, 3], 0);
    let b = SeamHello::advertise(&[2, 3, 5], 0);
    assert_eq!(
        negotiate_with_role(&a, &b, Role::Consumer).unwrap().profile,
        3
    );
    assert_eq!(
        negotiate_with_role(&b, &a, Role::Provider).unwrap().profile,
        3
    );
    match negotiate_with_role(
        &SeamHello::advertise(&[1], 0),
        &SeamHello::advertise(&[2], 0),
        Role::Consumer,
    ) {
        Err(SeamError::VersionMismatch { local, remote }) => {
            assert_eq!((local, remote), (vec![1], vec![2]));
        }
        other => panic!("expected VersionMismatch, got {other:?}"),
    }
}

#[test]
fn negotiation_validates_both_envelopes_first() {
    let mut bad = SeamHello::profile_v1(0);
    bad.reserved1[2] = 7;
    let ok = SeamHello::profile_v1(0);
    assert_eq!(
        negotiate_with_role(&bad, &ok, Role::Consumer),
        Err(SeamError::ReservedNonZero)
    );
    assert_eq!(
        negotiate_with_role(&ok, &bad, Role::Consumer),
        Err(SeamError::ReservedNonZero)
    );
}

#[test]
fn ext_and_sub_bits_are_live_only_when_both_sides_set_them() {
    let sub = 1u64 << 52;
    let consumer = SeamHello::profile_v1(SEAM_CAP_DLPACK_EXT_V1 | SEAM_CAP_DLPACK_EXT_MX | sub);
    let provider = SeamHello::profile_v1(SEAM_CAP_DLPACK_EXT_V1 | SEAM_CAP_DLPACK_EXT_GGML);
    let n = negotiate_with_role(&consumer, &provider, Role::Consumer).unwrap();
    assert_eq!(n.capabilities, SEAM_CAP_DLPACK_EXT_V1);
}

#[test]
fn a_provider_only_bit_is_live_when_only_the_provider_sets_it() {
    let consumer = SeamHello::profile_v1(SEAM_CAP_DLPACK_EXT_V1);
    let provider = SeamHello::profile_v1(
        SEAM_CAP_DLPACK_EXT_V1 | SEAM_CAP_PROVISION_ON_REQUEST | SEAM_CAP_CONTRACT_QUERY,
    );
    for (local, remote, role) in [
        (&consumer, &provider, Role::Consumer),
        (&provider, &consumer, Role::Provider),
    ] {
        let n = negotiate_with_role(local, remote, role).unwrap();
        assert_eq!(
            n.capabilities,
            SEAM_CAP_DLPACK_EXT_V1 | SEAM_CAP_PROVISION_ON_REQUEST | SEAM_CAP_CONTRACT_QUERY,
            "{role:?}"
        );
    }
}

#[test]
fn a_provider_only_bit_set_only_by_the_consumer_is_never_live() {
    // Sabotage sibling of the test above: without it a formula that took FEAT from `local | remote` would pass that one too.
    let consumer = SeamHello::profile_v1(SEAM_CAP_DLPACK_EXT_V1 | SEAM_CAP_PROVISION_ON_REQUEST);
    let provider = SeamHello::profile_v1(SEAM_CAP_DLPACK_EXT_V1);
    let a = negotiate_with_role(&consumer, &provider, Role::Consumer).unwrap();
    let b = negotiate_with_role(&provider, &consumer, Role::Provider).unwrap();
    assert_eq!(a.capabilities & SEAM_CAP_PROVISION_ON_REQUEST, 0);
    assert_eq!(b.capabilities & SEAM_CAP_PROVISION_ON_REQUEST, 0);
}

#[test]
fn an_unrecognised_feat_bit_follows_the_provider_because_the_axis_is_masked_whole() {
    let unknown = 1u64 << 40;
    let n = negotiate_with_role(
        &SeamHello::profile_v1(0),
        &SeamHello::profile_v1(unknown),
        Role::Consumer,
    )
    .unwrap();
    assert_eq!(n.capabilities, unknown);
}

#[test]
fn the_axis_masks_partition_the_word_and_hold_the_allocated_bits() {
    assert_eq!(
        SEAM_CAP_EXT_MASK | SEAM_CAP_FEAT_MASK | SEAM_CAP_SUB_MASK,
        u64::MAX
    );
    assert_eq!(SEAM_CAP_EXT_MASK & SEAM_CAP_FEAT_MASK, 0);
    assert_eq!(SEAM_CAP_FEAT_MASK & SEAM_CAP_SUB_MASK, 0);
    assert_eq!(SEAM_CAP_PROVIDER_ONLY_MASK, SEAM_CAP_FEAT_MASK);
    assert_eq!(SEAM_CAP_PROVISION_ON_REQUEST, 1 << 32);
    assert_eq!(SEAM_CAP_CONTRACT_QUERY, 1 << 33);
    for (i, bit) in [
        SEAM_CAP_DLPACK_EXT_V1,
        SEAM_CAP_DLPACK_EXT_MX,
        SEAM_CAP_DLPACK_EXT_GGML,
        SEAM_CAP_DLPACK_EXT_AFFINE,
        SEAM_CAP_DLPACK_EXT_SYMBOLIC,
        SEAM_CAP_DLPACK_EXT_GATHER,
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(bit, 1 << i, "bits 0-5 are frozen in this order");
        assert_ne!(bit & SEAM_CAP_EXT_MASK, 0);
    }
}

// ---- KISS vectors (KISS-ANNOUNCE-7.3-0003), KISS 6780c09 conformance/corpus/announce_vectors.json -------------------------

const KISS_CONSUMER: &str = "53 45 41 4D 01 00 00 00 01 00 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01 00 00 00 00 00 31 00";
const KISS_PROVIDER: &str = "53 45 41 4D 01 00 00 00 02 00 01 00 02 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 03 00 00 00 03 00 91 00";
const KISS_LIVE_LE_HEX: &str = "01 00 00 00 03 00 11 00";

#[test]
fn the_kiss_vectors_give_the_same_live_word_in_both_orientations() {
    let consumer = SeamHello::from_wire_bytes(&hex(KISS_CONSUMER)).expect("KISS consumer frame");
    let provider = SeamHello::from_wire_bytes(&hex(KISS_PROVIDER)).expect("KISS provider frame");
    let live = u64::from_le_bytes(hex(KISS_LIVE_LE_HEX).try_into().unwrap());
    let a = negotiate_with_role(&consumer, &provider, Role::Consumer).unwrap();
    let b = negotiate_with_role(&provider, &consumer, Role::Provider).unwrap();
    assert_eq!((a.profile, b.profile), (1, 1));
    assert_eq!((a.capabilities, b.capabilities), (live, live));
    // A plain AND would lose both FEAT bits: the reason the negotiation is role-aware.
    assert_eq!(
        consumer.capabilities & provider.capabilities,
        0x0011_0000_0000_0001
    );
}

// ---- Only allocated bits may be advertised ---------------------------------------------------------------------------

#[test]
fn the_allocated_set_is_exactly_the_frozen_bits() {
    let expected = 0b11_1111 | SEAM_CAP_PROVISION_ON_REQUEST | SEAM_CAP_CONTRACT_QUERY;
    assert_eq!(ALLOCATED_CAPABILITIES, expected);
    assert_eq!(
        ALLOCATED_CAPABILITIES & (1 << 34),
        0,
        "KISC_FRAMING is provisional and not allocated here"
    );
    assert_eq!(
        ALLOCATED_CAPABILITIES & (1 << 35),
        0,
        "RECIPE_IMPORT is provisional and not allocated here"
    );
}

#[test]
fn unallocated_capabilities_names_exactly_the_bits_a_side_may_not_advertise() {
    let honest = SeamHello::profile_v1(ALLOCATED_CAPABILITIES);
    assert_eq!(honest.unallocated_capabilities(), 0);
    let provisional = SeamHello::profile_v1(SEAM_CAP_DLPACK_EXT_V1 | (1 << 34) | (1 << 35));
    assert_eq!(
        provisional.unallocated_capabilities(),
        (1 << 34) | (1 << 35)
    );
    // A provider's real advertisement today: all six EXT tokens plus PROVISION_ON_REQUEST (baracuda `BARACUDA_CAPABILITIES`).
    let baracuda_today = SeamHello::profile_v1(0b11_1111 | SEAM_CAP_PROVISION_ON_REQUEST);
    assert_eq!(baracuda_today.unallocated_capabilities(), 0);
}
