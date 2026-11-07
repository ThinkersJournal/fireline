// SPDX-License-Identifier: MIT OR Apache-2.0
//! Marker-integrity guard for the `EXHAUSTIVE-BY-DESIGN` convention, for this crate's own sources.
//!
//! An enum whose definition carries the `EXHAUSTIVE-BY-DESIGN` marker (a comment in the attribute/doc block directly above
//! `pub enum ...`) is a *closed* set: every consumer matches it exhaustively, so adding a variant is a deliberate breaking change the
//! compiler must surface at every match site (baracuda's `region_op_id` walks `PatternNode` this way). Such an enum must therefore never
//! also be `#[non_exhaustive]`, which forces wildcard arms and turns that compile error into a silent fall-through. The two declarations
//! contradict each other; this test makes them mutually exclusive by test rather than by discipline.
//!
//! Origin: Fuel `fuel-ir/tests/exhaustive_by_design_marker.rs` (GAP-049), whose whole-tree scan covered `PatternNode` while it lived in
//! Fuel. That scan can no longer see it, so the guard moves with the type. The parser below is that file's, narrowed to this crate.
//!
//! Two assertions, because a guard that can only pass is worthless: the **invariant** (no marked enum is `#[non_exhaustive]`) and the
//! **reach** (every enum we know is marked is actually found, so a renamed marker or a moved file cannot make the scan pass vacuously).
//! The scan prints what it covered.

use std::path::{Path, PathBuf};

const MARKER: &str = "EXHAUSTIVE-BY-DESIGN";

/// Enums we KNOW carry the marker. Extending the marker to a new enum means adding it here in the same change, so a new closed-enum
/// declaration is a reviewed act.
const EXPECTED_MARKED: &[&str] = &["PatternNode"];

#[derive(Debug, PartialEq, Eq)]
struct MarkedEnum {
    name: String,
    line: usize,
    non_exhaustive: bool,
}

/// The enum name from a declaration line's trimmed head, or `None` if the line is not an enum declaration. Handles `enum X`, `pub enum X`,
/// `pub(crate) enum X` and generics; rejects `pub struct`, `fn enumerate`, comments.
fn enum_decl_name(trimmed: &str) -> Option<String> {
    let mut rest = trimmed;
    if let Some(after_pub) = rest.strip_prefix("pub")
        && (after_pub.starts_with(char::is_whitespace) || after_pub.starts_with('('))
    {
        rest = after_pub.trim_start();
        if rest.starts_with('(') {
            let close = rest.find(')')?;
            rest = rest[close + 1..].trim_start();
        }
    }
    let after_enum = rest.strip_prefix("enum")?;
    if !after_enum.starts_with(char::is_whitespace) {
        return None;
    }
    let name: String = after_enum
        .trim_start()
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if name.is_empty() { None } else { Some(name) }
}

/// Every enum in `text` whose contiguous attribute/comment block directly above the declaration contains [`MARKER`], flagged with whether
/// that block also carries a `#[non_exhaustive]` attribute. The attribute check is exact line equality, so prose that merely mentions it
/// is not mistaken for it.
fn marked_enums(text: &str) -> Vec<MarkedEnum> {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let Some(name) = enum_decl_name(line.trim_start()) else {
            continue;
        };
        let (mut has_marker, mut non_exhaustive) = (false, false);
        let mut j = i;
        while j > 0 {
            j -= 1;
            let t = lines[j].trim();
            if !(t.starts_with('#') || t.starts_with("//")) {
                break;
            }
            has_marker |= t.contains(MARKER);
            non_exhaustive |= t == "#[non_exhaustive]";
        }
        if has_marker {
            out.push(MarkedEnum {
                name,
                line: i + 1,
                non_exhaustive,
            });
        }
    }
    out
}

fn rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = std::fs::read_dir(dir)
        .expect("crate src directory")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("rs"))
        .collect();
    out.sort();
    out
}

fn scan_crate_sources() -> Vec<(PathBuf, MarkedEnum)> {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut found = Vec::new();
    for file in rs_files(&src) {
        let text = std::fs::read_to_string(&file).expect("readable source");
        for m in marked_enums(&text) {
            found.push((file.clone(), m));
        }
    }
    found
}

#[test]
fn no_marked_enum_in_this_crate_is_non_exhaustive_and_every_expected_one_is_found() {
    let found = scan_crate_sources();
    for (file, m) in &found {
        println!(
            "EXHAUSTIVE-BY-DESIGN enum `{}` at {}:{} (non_exhaustive: {})",
            m.name,
            file.display(),
            m.line,
            m.non_exhaustive
        );
    }
    // Reach: the scan must have found what we know is marked, else it is passing vacuously.
    for expected in EXPECTED_MARKED {
        assert!(
            found.iter().any(|(_, m)| m.name == *expected),
            "the scan did not find the marked enum `{expected}`: marker renamed, file moved, or parser drift"
        );
    }
    // Invariant: a closed-by-design enum must not also be `#[non_exhaustive]`.
    for (file, m) in &found {
        assert!(
            !m.non_exhaustive,
            "`{}` at {}:{} carries {MARKER} and `#[non_exhaustive]`: they contradict each other",
            m.name,
            file.display(),
            m.line
        );
    }
}

#[test]
fn the_parser_reads_enum_declarations_and_nothing_else() {
    assert_eq!(enum_decl_name("pub enum X {"), Some("X".into()));
    assert_eq!(enum_decl_name("enum X<T> {"), Some("X".into()));
    assert_eq!(enum_decl_name("pub(crate) enum Y {"), Some("Y".into()));
    assert_eq!(enum_decl_name("pub struct X;"), None);
    assert_eq!(enum_decl_name("fn enumerate() {}"), None);
    assert_eq!(enum_decl_name("// enum X"), None);
    assert_eq!(enum_decl_name("pub enum {"), None);
}

#[test]
fn the_scan_flags_a_marked_enum_that_is_also_non_exhaustive_and_only_that() {
    let both = "// EXHAUSTIVE-BY-DESIGN: closed.\n#[non_exhaustive]\npub enum A {}\n";
    let marker_only = "// EXHAUSTIVE-BY-DESIGN: closed.\n#[derive(Debug)]\npub enum B {}\n";
    let attr_only = "#[non_exhaustive]\npub enum C {}\n";
    let prose = "// EXHAUSTIVE-BY-DESIGN: never add #[non_exhaustive] here.\npub enum D {}\n";
    let broken_block = "// EXHAUSTIVE-BY-DESIGN: closed.\n\npub enum E {}\n";
    assert_eq!(
        marked_enums(both),
        [MarkedEnum {
            name: "A".into(),
            line: 3,
            non_exhaustive: true
        }]
    );
    assert_eq!(
        marked_enums(marker_only),
        [MarkedEnum {
            name: "B".into(),
            line: 3,
            non_exhaustive: false
        }]
    );
    assert!(
        marked_enums(attr_only).is_empty(),
        "no marker, so not this guard's business"
    );
    assert_eq!(
        marked_enums(prose),
        [MarkedEnum {
            name: "D".into(),
            line: 2,
            non_exhaustive: false
        }],
        "prose is not the attribute"
    );
    assert!(
        marked_enums(broken_block).is_empty(),
        "a blank line ends the block"
    );
}
