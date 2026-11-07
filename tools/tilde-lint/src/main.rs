// SPDX-License-Identifier: MIT OR Apache-2.0
//! `fireline-tilde-lint [Cargo.toml ...]`: exit 0 when every FireLine requirement is tilde or exact, 1 on a violation, 2 on an error.
//!
//! With no arguments it checks every `Cargo.toml` that git tracks (`git ls-files`), never a walk of the disk, because a disk walk sees other
//! lanes' worktrees (portfolio CLAUDE.md 5b). It prints which enumeration ran and how many manifests it read, so an empty run cannot pass quietly.

use std::process::{Command, ExitCode};

fn tracked_manifests() -> Result<Vec<String>, String> {
    let out = Command::new("git")
        .args(["ls-files", "-z"])
        .output()
        .map_err(|e| format!("cannot run git ls-files: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git ls-files failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout)
        .split('\0')
        .filter(|p| *p == "Cargo.toml" || p.ends_with("/Cargo.toml"))
        .map(str::to_owned)
        .collect())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (paths, source) = if args.is_empty() {
        match tracked_manifests() {
            Ok(p) => (p, "git ls-files"),
            Err(e) => {
                eprintln!("error: {e}");
                return ExitCode::from(2);
            }
        }
    } else {
        (args, "arguments")
    };
    println!(
        "fireline-tilde-lint: {} manifest(s) from {source}",
        paths.len()
    );
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
