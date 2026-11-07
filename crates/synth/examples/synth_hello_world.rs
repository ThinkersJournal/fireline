// SPDX-License-Identifier: MIT OR Apache-2.0
//! Hello world: a provider implements `Synthesizer`; a consumer asks it for a kernel, then adopts it. No device, no compiler: the "kernel" is
//! two bytes. This is the whole two-step handover.

use fireline_grammar::{OpAttrs, OpTag, PatternNode};
use fireline_synth::{
    ArtifactKind, JitBudget, JitRequest, JitResponse, LinkEntry, SynthArtifact, Synthesizer,
};
use std::collections::HashMap;
use std::sync::Mutex;
use unpopped_vocab::{ArchSku, ElementKind, OperandDesc};

/// A stand-in provider: `synthesize` retains the artifact, `take_kernel` hands it over once.
#[derive(Default)]
struct Echo {
    retained: Mutex<HashMap<String, SynthArtifact>>,
}

impl Synthesizer for Echo {
    fn synthesize(&self, req: &JitRequest) -> JitResponse {
        if req.operands.is_empty() {
            return JitResponse::Declined {
                reason: "no operands".into(),
            };
        }
        let entry_point = "jit::echo::relu".to_string();
        let artifact = SynthArtifact {
            artifact: vec![0xDE, 0xAD],
            kind: ArtifactKind::Ptx,
            link: LinkEntry {
                entry_point: entry_point.clone(),
                symbol: "echo_relu".into(),
                structure_key: "elementwise:f32".into(),
                revision_hash: 1,
            },
            contract: "## fused_op: jit::echo::relu\n".into(),
        };
        self.retained
            .lock()
            .unwrap()
            .insert(entry_point.clone(), artifact);
        JitResponse::Synthesized { entry_point }
    }

    fn take_kernel(&self, entry_point: &str) -> Option<SynthArtifact> {
        self.retained.lock().unwrap().remove(entry_point)
    }
}

fn main() {
    let region = PatternNode::Op {
        op: OpTag::Relu,
        operands: vec![PatternNode::Bind { index: 0 }],
        attrs: OpAttrs::default(),
    };
    let operands = vec![OperandDesc::new(1, &[4], &[1], ElementKind::F32, 256)];
    let request = JitRequest::new(
        region,
        operands,
        ArchSku::Sm89,
        JitBudget {
            max_compile_ms: 250,
        },
    );

    let provider = Echo::default();
    // Step 1: a light reply. Nothing heavy has crossed yet.
    let JitResponse::Synthesized { entry_point } = provider.synthesize(&request) else {
        panic!("the echo provider always builds something for a non-empty request");
    };
    println!(
        "provider offers `{entry_point}` for target {}",
        request.target
    );
    // Step 2: the consumer's cost gate said yes, so the artifact is handed over, once.
    let artifact = provider
        .take_kernel(&entry_point)
        .expect("retained until adopted");
    println!(
        "adopted {} bytes of {:?}",
        artifact.artifact.len(),
        artifact.kind
    );
    assert!(
        provider.take_kernel(&entry_point).is_none(),
        "a second take finds nothing: single adopt"
    );
}
