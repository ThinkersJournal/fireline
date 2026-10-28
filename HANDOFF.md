# HANDOFF - FireLine lane (current state, written by the PM 2026-10-28)

You are the **FireLine lane**. Read `C:\Projects\AGENT-ONBOARDING.md` once. Messages: Synapse and claude-peers (PM role `pm`). Models: sonnet only.
Repo `ThinkersJournal/fireline`, crate `fireline` 0.0.0 claimed (placeholder). Name proposed by CireSnave 2026-10-28 ("the lines between Fuel and other ML projects"): his final yes on the name is pending, the PM treats it as accepted for now.

## What this is
The interface crates (both sides of each interface) between Fuel and the other ML projects, as ONE project with ONE CRATE PER INTERFACE, so many projects share them and cannot drift.
**CireSnave, board 295 (2026-10-25 and 2026-10-28, first-hand):** "the interface crates (both sides of each interface) should be spun out into their own projects so many projects could use them and be sure they had no drift of their interfaces." Layout agreed: one project, a crate per interface, each crate versioned by its interface, **converging at a common version at major releases** (first: the KISS freeze); before 1.0 a breaking change to one interface moves that crate's SECOND digit alone; near 1.0 all go through common 1.0.0-rcN; after 1.0 a breaking change confined to one interface raises that crate's MINOR. **Consumers require these crates with a tilde (`~1.2`) or exact, never a caret**: README states it and a CI check in every consuming project and here fails on a caret. Ruling 2026-10-28: **"Start now. Spin them out immediately."**
**Boundary:** only what has no dependency on a project's internals moves (wire types, handshake, capability bits, shared vocabulary, a conformance suite both sides run). Anything naming Fuel's tensors, graphs or devices, or baracuda's kernel internals stays with its project. Not an Ant (ML projects do not use Ant names). In-process links between two projects still use these crates' types; the KISS standard itself defines no in-process profile (board 309 Q3).

## KISS seams as the PM understands them (verify, correct, extend)
Nine KISS sub-standards: Classify, Ops, Grammar, Contract, Announce, Synth/Provision, Consume, Emit, Conform. CireSnave's reading of the party seams: ML orchestrator (Fuel) <-> kernel-executing backend (baracuda, Vulkan, ROCm, OpenCL, AOCL, ...); kernel consumer (Fuel) <-> kernel producer (Unpopped, possibly baracuda, a future kernel library/cache/marketplace); orchestrator <-> producer. Possibly missing: Consume (lifting a kernel or source region into the op DAG) and Emit (generation: IR to target code) between Unpopped's core and its front ends and emitters; Conform is the test seam.
Today's candidates: fuel-kernel-seam, fuel-kernel-seam-types, fuel-kernel-seam-announce (carry Fuel's workspace version, which forces baracuda to republish on every Fuel breaking release); Unpopped's IR and lowering contract (Unpopped is mid-rework: union design, 0.29.0); KISS key and contract formats and vectors (KISS stays the standard; you hold the implementations it is checked against).

## Tasks, in order
1. **Seam inventory** (read-only, docs/SEAM-INVENTORY.md): every interface between Fuel, baracuda, Unpopped, mlmf, ml-convertant, lightbulb, OverMind, vulkane and Synapse that is a crate boundary or a wire format: name, both sides, current home, version coupling today, KISS sub-standard if any, non-KISS ones (model loading, conversions, OpenAI-compatible API, latent-state API). Mark UNVERIFIED anything you did not read at origin/main. Ask each lane for its must-haves.
2. **One-page extraction plan** with the Fuel, baracuda, Unpopped and KISS lanes: which crates move, in what order (Fuel-to-producer seam first: it hurts now), how each side migrates (one coordinated breaking release, once), version scheme and the tilde check, relation to KISS's own crates (no two sources of truth), patent sweep, ease rule (EXPECTATIONS 6.24: hello-world first).
3. Skeleton workspace + the tilde-lint + first crate moved, red-first. One PR at a time; version bump every PR (PM allocates); CI exists, protection set.
Ant-first: use ConfigurAnt/CapAnt where they fit; others are placeholders. Memory: CARGO_BUILD_JOBS=2, one build at a time. Patent sweep before any publish.

## State
main = placeholder 0.0.0 + this HANDOFF (0.0.1). No open PRs. No lane existed before this one.
