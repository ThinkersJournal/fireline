// SPDX-License-Identifier: MIT OR Apache-2.0
//! `fireline-tilde-lint [Cargo.toml ...]`: exit 0 when every FireLine requirement is tilde or exact, 1 on a violation, 2 on an error.
//!
//! With no arguments it checks every `Cargo.toml` git lists for this repository, tracked **and untracked** (`git ls-files --cached --others
//! --exclude-standard`), so a lane running it on uncommitted work still gets its new manifests checked, but never a walk of the disk, because a
//! disk walk sees other lanes' worktrees (portfolio CLAUDE.md 5b) and ignored build output. It prints which enumeration ran, how many manifests
//! it read and how many listed ones were missing on disk, so an empty or partial run cannot pass quietly.

use std::path::Path;
use std::process::{Command, ExitCode};

/// The repository's manifests: `(present on disk, listed but missing on disk)`. A path can be listed yet missing when it was deleted in the
/// working tree and the deletion is not staged; there is nothing to check in it, and it is reported rather than treated as a failure.
fn repository_manifests() -> Result<(Vec<String>, usize), String> {
    let out = Command::new("git")
        .args([
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
            "--deduplicate",
        ])
        .output()
        .map_err(|e| format!("cannot run git ls-files: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git ls-files failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    let listed: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .split('\0')
        .filter(|p| *p == "Cargo.toml" || p.ends_with("/Cargo.toml"))
        .map(str::to_owned)
        .collect();
    let (present, missing): (Vec<String>, Vec<String>) =
        listed.into_iter().partition(|p| Path::new(p).is_file());
    Ok((present, missing.len()))
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (paths, source, missing) = if args.is_empty() {
        match repository_manifests() {
            Ok((p, missing)) => (
                p,
                "git ls-files (tracked and untracked, ignored excluded)",
                missing,
            ),
            Err(e) => {
                eprintln!("error: {e}");
                return ExitCode::from(2);
            }
        }
    } else {
        (args, "arguments", 0)
    };
    println!(
        "fireline-tilde-lint: {} manifest(s) from {source}",
        paths.len()
    );
    if missing > 0 {
        println!("note: {missing} listed manifest(s) missing on disk, skipped");
    }
    if paths.is_empty() {
        eprintln!("error: no Cargo.toml found; refusing to pass on an empty set");
        return ExitCode::from(2);
    }
    let mut violations = 0usize;
    for path in &paths {
        let text = match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("error: {path}: {e}");
                return ExitCode::from(2);
            }
        };
        match fireline_tilde_lint::check_manifest(&text) {
            Ok(vs) => {
                for v in vs {
                    violations += 1;
                    println!("{path}: [{}] {}: {}", v.table, v.name, v.reason);
                }
            }
            Err(e) => {
                eprintln!("error: {path}: {e}");
                return ExitCode::from(2);
            }
        }
    }
    if violations == 0 {
        println!("ok: no caret requirement on a FireLine crate");
        ExitCode::SUCCESS
    } else {
        println!("{violations} violation(s): require FireLine crates with `~x.y` or `=x.y.z`");
        ExitCode::from(1)
    }
}
