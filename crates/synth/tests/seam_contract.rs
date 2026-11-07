// SPDX-License-Identifier: MIT OR Apache-2.0
//! Contract tests for the synthesizer envelope, relocated from the inline `mod tests` of Fuel's `fuel-kernel-seam` (Fuel `d010189e`).
//! They use only the public API, so they are the same test a provider's own crate can run against its `Synthesizer`.

use fireline_grammar::{OpAttrs, OpTag, PatternNode};
use fireline_synth::*;
use std::collections::HashMap;
use std::sync::Mutex;
use unpopped_vocab::ElementKind;
use unpopped_vocab::{ArchSku, OperandDesc};

fn relu_add() -> PatternNode {
    PatternNode::Op {
        op: OpTag::Relu,
        attrs: OpAttrs::default(),
        operands: vec![PatternNode::Op {
            op: OpTag::Add,
            attrs: OpAttrs::default(),
            operands: vec![
                PatternNode::Bind { index: 0 },
                PatternNode::Bind { index: 1 },
            ],
        }],
    }
}

/// A stand-in synthesizer mirroring Baracuda's built shape: `synthesize`
/// inserts the artifact under its entry_point; `take_kernel` removes + returns
/// it. Interior-mutable + `Send + Sync`, so Fuel could drive it off-thread.
#[derive(Default)]
struct EchoSynth {
    store: Mutex<HashMap<String, SynthArtifact>>,
}
impl Synthesizer for EchoSynth {
    fn synthesize(&self, req: &JitRequest) -> JitResponse {
        if req.operands.is_empty() {
            return JitResponse::Declined {
                reason: "no operands".into(),
            };
        }
        let entry_point = "jit::echo::relu_add".to_string();
        self.store.lock().unwrap().insert(
            entry_point.clone(),
            SynthArtifact {
                artifact: vec![0xDE, 0xAD],
                kind: ArtifactKind::Ptx,
                link: LinkEntry {
                    entry_point: entry_point.clone(),
                    symbol: "echo_relu_add".into(),
                    structure_key: "elementwise:f32".into(),
                    revision_hash: 0x1234,
                },
                contract: "## fused_op: jit::echo::relu_add\ncost: n\n".into(),
            },
        );
        JitResponse::Synthesized { entry_point }
    }
    fn take_kernel(&self, entry_point: &str) -> Option<SynthArtifact> {
        self.store.lock().unwrap().remove(entry_point)
    }
}

fn req() -> JitRequest {
    JitRequest::new(
        relu_add(),
        vec![
            OperandDesc::new(1, &[4], &[1], ElementKind::F32, 256),
            OperandDesc::new(1, &[4], &[1], ElementKind::F32, 256),
        ],
        ArchSku::Sm89,
        JitBudget {
            max_compile_ms: 250,
        },
    )
}

/// Board #106 Step A condition: every `ArchSku` variant, not a sample —
/// an exhaustive `match` so a new variant upstream fails to COMPILE here
/// rather than silently skipping coverage. `TargetId::from` is asserted
/// infallible for each (it already panics internally if ever not, per
/// its own doc) and produces the exact `cuda:sm<N>[a]` token the §5.2
/// target model expects.
#[test]
fn jit_request_new_derives_target_for_every_arch_sku_variant() {
    for arch in [ArchSku::Sm80, ArchSku::Sm89, ArchSku::Sm90, ArchSku::Sm90a] {
        let expected_token = match arch {
            ArchSku::Sm80 => "cuda:sm80",
            ArchSku::Sm89 => "cuda:sm89",
            ArchSku::Sm90 => "cuda:sm90",
            ArchSku::Sm90a => "cuda:sm90a",
        };
        let req = JitRequest::new(relu_add(), vec![], arch, JitBudget { max_compile_ms: 1 });
        assert_eq!(
            req.target.to_string(),
            expected_token,
            "JitRequest::new's TargetId::from(ArchSku::{arch:?}) must match the \
             §5.2 target model's cuda: token"
        );
    }
}

#[test]
fn synthesize_returns_a_light_handle_then_take_kernel_hands_over_the_artifact() {
    let synth = EchoSynth::default();
    let entry_point = match synth.synthesize(&req()) {
        JitResponse::Synthesized { entry_point } => entry_point,
        JitResponse::Declined { reason } => panic!("expected synthesis, declined: {reason}"),
    };
    assert_eq!(entry_point, "jit::echo::relu_add");
    // The heavy artifact only crosses on adopt, via take_kernel.
    let art = synth
        .take_kernel(&entry_point)
        .expect("artifact retained for adopt");
    assert_eq!(art.kind, ArtifactKind::Ptx);
    assert_eq!(art.link.entry_point, entry_point);
    assert!(
        art.contract.contains("cost: n"),
        "carries the FKC contract markdown"
    );
    // Single-adopt: a second take is None.
    assert!(
        synth.take_kernel(&entry_point).is_none(),
        "take_kernel removes (single adopt)"
    );
}

#[test]
fn synthesizer_can_decline_and_retains_nothing() {
    let synth = EchoSynth::default();
    let mut declined = req();
    declined.operands.clear();
    assert!(matches!(
        synth.synthesize(&declined),
        JitResponse::Declined { .. }
    ));
    assert!(
        synth.take_kernel("jit::echo::relu_add").is_none(),
        "declined → nothing retained"
    );
}
