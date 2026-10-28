# FireLine

The lines between Fuel and the other ML projects: the interface crates (both sides of each interface) that Fuel, baracuda,
Unpopped and the other ML projects share, so each interface has its own version and no drift. One project, one crate per
interface, each crate versioned by its interface and converging at a common version at major releases (CireSnave 2026-10-25/28).
Consumers require these crates with a tilde, never a caret. Nothing is implemented yet; see ROADMAP.md and HANDOFF.md.
