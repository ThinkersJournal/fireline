// SPDX-License-Identifier: MIT OR Apache-2.0
//! With no arguments the lint enumerates the repository's manifests itself. It must cover **untracked** manifests too (a lane runs it on
//! uncommitted work), stay inside the repository (never another worktree), and never pass on a set it did not fully read.

use std::path::{Path, PathBuf};
use std::process::Command;

fn lint() -> &'static str {
    env!("CARGO_BIN_EXE_fireline-tilde-lint")
}

fn git(dir: &Path, args: &[&str]) {
    let st = Command::new("git")
        .current_dir(dir)
        .args(args)
        .status()
        .expect("git runs");
    assert!(st.success(), "git {args:?}");
}

/// A fresh temp repository with one clean, tracked root manifest.
fn repo(name: &str) -> PathBuf {
    let dir =
        std::env::temp_dir().join(format!("fireline-tilde-lint-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    git(&dir, &["init", "-q"]);
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"clean\"\nversion = \"0.0.0\"\n",
    )
    .unwrap();
    git(&dir, &["add", "Cargo.toml"]);
    dir
}

fn run(dir: &Path) -> (i32, String) {
    let out = Command::new(lint())
        .current_dir(dir)
        .output()
        .expect("lint runs");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
    )
}

#[test]
fn an_untracked_manifest_with_a_caret_is_caught() {
    let dir = repo("untracked");
    std::fs::create_dir_all(dir.join("sub")).unwrap();
    std::fs::write(
        dir.join("sub/Cargo.toml"),
        "[dependencies]\nfireline-a = \"^0.1\"\n",
    )
    .unwrap();
    let (code, out) = run(&dir);
    assert_eq!(code, 1, "{out}");
    assert!(
        out.contains("2 manifest(s)"),
        "it must say it read both: {out}"
    );
}

#[test]
fn a_tracked_manifest_with_a_caret_is_caught_as_before() {
    let dir = repo("tracked");
    std::fs::write(
        dir.join("Cargo.toml"),
        "[dependencies]\nfireline-a = \"^0.1\"\n",
    )
    .unwrap();
    assert_eq!(run(&dir).0, 1);
}

#[test]
fn an_ignored_manifest_is_not_read() {
    let dir = repo("ignored");
    std::fs::write(dir.join(".gitignore"), "target/\n").unwrap();
    std::fs::create_dir_all(dir.join("target")).unwrap();
    std::fs::write(
        dir.join("target/Cargo.toml"),
        "[dependencies]\nfireline-a = \"^0.1\"\n",
    )
    .unwrap();
    let (code, out) = run(&dir);
    assert_eq!(code, 0, "build output is never source: {out}");
    assert!(out.contains("1 manifest(s)"), "{out}");
}

#[test]
fn a_manifest_deleted_from_disk_but_still_in_the_index_does_not_crash_the_run() {
    let dir = repo("deleted");
    std::fs::create_dir_all(dir.join("gone")).unwrap();
    std::fs::write(dir.join("gone/Cargo.toml"), "[package]\nname = \"gone\"\n").unwrap();
    git(&dir, &["add", "gone/Cargo.toml"]);
    std::fs::remove_file(dir.join("gone/Cargo.toml")).unwrap();
    let (code, out) = run(&dir);
    assert_eq!(code, 0, "{out}");
    assert!(
        out.contains("missing on disk"),
        "it must say what it skipped: {out}"
    );
}
