# Publishing the wave-1 crates (announce, grammar, synth) at 0.1.0

Prepared 2026-11-08 by the FireLine lane. **The PM publishes** (onboarding section 2); this is the checked recipe. **Do not publish until CireSnave has answered A on board item 398** (the pending Nvidia application US 2023/0025245 A1; patent record `docs/PATENT-SWEEP.md`).

## Preconditions (each verified when this was written)

1. The three names are free on crates.io: `fireline-announce`, `fireline-grammar`, `fireline-synth` each return HTTP 404 (control: `serde` returns 200). `fireline-dlpack` and `fireline-provider` are also free and are **not** to be published (dlpack is unmerged; provider is held on the patent re-read). The placeholder `fireline` 0.0.0 exists, owner `ciresnave`.
2. `publish = false` is removed from the three crates by this change. Publish from a **clean checkout of the merged commit** (`git status --porcelain` empty), not from a worktree with edits.
3. `cargo package --locked -p fireline-announce -p fireline-grammar -p fireline-synth` succeeds and verifies all three (run 2026-11-08 at the tip of this branch). Contents are clean: sources, tests, examples, README, both licences, `Cargo.toml`, `Cargo.lock`; nothing from `target/` or `.git`. The packaged `fireline-synth` manifest requires `fireline-grammar` as `version = "~0.1"` with the path dropped.
4. Patent verdict: `docs/PATENT-SWEEP.md` (first pass, not legal advice; clear for these three crates; the design gate on US 8,458,680 and US 8,108,844 applies to anything added later).

## Order and commands

```
git checkout --detach <merged sha>          # clean tree
cargo publish --locked -p fireline-announce
cargo publish --locked -p fireline-grammar
# wait until fireline-grammar 0.1.0 is visible in the index (cargo's publish waits for it; if synth still cannot resolve it, retry after a minute)
cargo publish --locked -p fireline-synth    # needs fireline-grammar ~0.1 on the index first
```

`announce` and `grammar` are independent; **`synth` must come after `grammar`**. A dry run before the real thing: `cargo publish --locked --dry-run -p fireline-announce -p fireline-grammar` (synth's dry run cannot resolve grammar until it is published).

## After publishing

1. Read each version back from crates.io (`https://crates.io/api/v1/crates/<name>`), not just the exit code.
2. Tag the source (CIRESNAVE-EXPECTATIONS 6.20): `tools/tag-release.sh C:/Projects/fireline 0.1.0 <full sha> fireline`. One tag `v0.1.0` on the one commit all three were published from. **Once the crates version independently (for example announce 0.2.0 beside grammar 0.1.x) a bare `v<version>` tag is ambiguous**; the tool would need a crate-prefixed tag (`fireline-announce-v0.2.0`). Raise that before the second release.
3. Tell Fuel the names and versions (`fireline-announce`, `fireline-grammar`, `fireline-synth`, each `0.1.0`); Fuel requires them with `~0.1`. Fuel's migration hazards are in `docs/EXTRACTION-PLAN.md` section 11.
4. Consumers must have **exactly one copy of `unpopped-vocab`** in a build (`fireline-synth` requires 0.43.2); see the synth README.

## Undoing

A published version cannot be deleted. A bad one is yanked (`cargo yank --version 0.1.0 <name>`) and replaced by a new version.
