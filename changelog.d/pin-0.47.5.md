kind: CI only: the advisory `portfolio` job moves to CodeRipper 0.47.5
---
CI only: the `portfolio` job now calls coderipper `portfolio.yml` at c72a6e0 (coderipper#356), which installs CodeRipper 0.47.5. The rule-id migration tool found nothing to rename here (no `.coderipper.toml` check or blocking names, no `--skip-rule` uses). No code changed.
