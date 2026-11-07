//! Tilde lint: every requirement on a FireLine crate must be `~x.y[.z]` or `=x.y.z`.
//!
//! Why: FireLine's crates version by their own interface, and a caret on `0.n` (or any bare version, which Cargo reads as a caret) lets a
//! consumer silently take a breaking release of another project's interface (board 295). A tilde keeps a consumer on one interface line.
//!
//! What counts as a FireLine crate: the dependency's package name is `fireline` or starts with `fireline-` (a rename through
//! `package = "..."` is followed). Checked tables: `dependencies`, `dev-dependencies`, `build-dependencies`, `workspace.dependencies` and the
//! same three under every `target.<cfg>`. A `workspace = true` entry is checked where its version lives, in `workspace.dependencies`.

use toml::{Table, Value};

/// One offending requirement.
#[derive(Debug, PartialEq, Eq)]
pub struct Violation {
    /// The dependency table, for example `dependencies` or `target.cfg(unix).dev-dependencies`.
    pub table: String,
    /// The dependency's key in that table.
    pub name: String,
    /// Why it was refused.
    pub reason: String,
}

const KINDS: [&str; 3] = ["dependencies", "dev-dependencies", "build-dependencies"];

fn is_fireline(package: &str) -> bool {
    package == "fireline" || package.starts_with("fireline-")
}

/// `Ok` when `req` is a single `~` or `=` requirement on a numeric version.
fn check_req(req: &str) -> Result<(), String> {
    let req = req.trim();
    if req.contains(',') {
        return Err(format!(
            "`{req}` lists several comparators; use one `~x.y` or `=x.y.z`"
        ));
    }
    let Some(rest) = req.strip_prefix('~').or_else(|| req.strip_prefix('=')) else {
        let why = if req.starts_with('^') {
            "an explicit caret"
        } else {
            "no `~` or `=` prefix (a bare version is an implicit caret)"
        };
        return Err(format!("`{req}` is {why}; use `~x.y` or `=x.y.z`"));
    };
    let rest = rest.trim();
    let valid = rest.starts_with(|c: char| c.is_ascii_digit())
        && rest
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '+'));
    if valid {
        Ok(())
    } else {
        Err(format!(
            "`{req}` is not a numeric `~x.y` or `=x.y.z` requirement"
        ))
    }
}

fn check_entry(table: &str, key: &str, value: &Value, out: &mut Vec<Violation>) {
    let package = value.get("package").and_then(Value::as_str).unwrap_or(key);
    if !is_fireline(package) {
        return;
    }
    let mut refuse = |reason: String| {
        out.push(Violation {
            table: table.to_owned(),
            name: key.to_owned(),
            reason,
        })
    };
    match value {
        Value::String(req) => {
            if let Err(why) = check_req(req) {
                refuse(why);
            }
        }
        Value::Table(t) => {
            if t.get("workspace").and_then(Value::as_bool) == Some(true)
                && !t.contains_key("version")
            {
                return; // the version lives in workspace.dependencies
            }
            if t.contains_key("git") {
                refuse("a `git` dependency; FireLine crates come from crates.io (CireSnave 2026-10-07)".into());
            } else {
                match t.get("version").and_then(Value::as_str) {
                    Some(req) => {
                        if let Err(why) = check_req(req) {
                            refuse(why);
                        }
                    }
                    None => refuse(
                        "no `version`; give `~x.y` (a path alone is not a requirement)".into(),
                    ),
                }
            }
        }
        _ => refuse("not a string or a table".into()),
    }
}

fn check_table(label: &str, table: &Table, out: &mut Vec<Violation>) {
    for (key, value) in table {
        check_entry(label, key, value, out);
    }
}

/// Check one manifest's text. `Err` means it could not be read as TOML (never a pass).
pub fn check_manifest(text: &str) -> Result<Vec<Violation>, String> {
    let doc: Table = text.parse().map_err(|e| format!("not valid TOML: {e}"))?;
    let mut out = Vec::new();
    for kind in KINDS {
        if let Some(t) = doc.get(kind).and_then(Value::as_table) {
            check_table(kind, t, &mut out);
        }
    }
    if let Some(t) = doc
        .get("workspace")
        .and_then(|w| w.get("dependencies"))
        .and_then(Value::as_table)
    {
        check_table("workspace.dependencies", t, &mut out);
    }
    if let Some(targets) = doc.get("target").and_then(Value::as_table) {
        for (cfg, body) in targets {
            for kind in KINDS {
                if let Some(t) = body.get(kind).and_then(Value::as_table) {
                    check_table(&format!("target.{cfg}.{kind}"), t, &mut out);
                }
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bad(text: &str) -> Vec<(String, String)> {
        check_manifest(text)
            .unwrap()
            .into_iter()
            .map(|v| (v.table, v.name))
            .collect()
    }
    fn names(v: &[(String, String)]) -> Vec<&str> {
        v.iter().map(|(_, n)| n.as_str()).collect()
    }

    #[test]
    fn explicit_caret_is_refused() {
        assert_eq!(
            names(&bad("[dependencies]\nfireline-announce = \"^0.1\"\n")),
            ["fireline-announce"]
        );
    }
    #[test]
    fn bare_version_is_an_implicit_caret_and_is_refused() {
        assert_eq!(
            names(&bad("[dependencies]\nfireline-grammar = \"0.1.2\"\n")),
            ["fireline-grammar"]
        );
        assert_eq!(
            names(&bad(
                "[dependencies]\nfireline-grammar = { version = \"0.1\" }\n"
            )),
            ["fireline-grammar"]
        );
    }
    #[test]
    fn tilde_and_exact_pass() {
        for req in ["~0.1", "~0.1.2", "=0.1.0", "~ 0.1"] {
            let m = format!("[dependencies]\nfireline-synth = \"{req}\"\n");
            assert!(bad(&m).is_empty(), "{req} should pass");
        }
    }
    #[test]
    fn other_crates_may_use_carets() {
        assert!(bad("[dependencies]\nserde = \"^1\"\ntoml = \"1.2\"\n").is_empty());
    }
    #[test]
    fn ranges_wildcards_and_lists_are_refused() {
        for req in ["*", ">=0.1", "<0.2", "~0.1, <0.1.5", "0.*", "^0.1.2, ~0.1"] {
            let m = format!("[dependencies]\nfireline-announce = \"{req}\"\n");
            assert_eq!(bad(&m).len(), 1, "{req} should be refused");
        }
    }
    #[test]
    fn every_dependency_table_is_checked() {
        let m = "[dev-dependencies]\nfireline-a = \"^1\"\n[build-dependencies]\nfireline-b = \"1\"\n\
                 [workspace.dependencies]\nfireline-c = \"1\"\n\
                 [target.'cfg(unix)'.dependencies]\nfireline-d = \"1\"\n\
                 [target.'cfg(unix)'.dev-dependencies]\nfireline-e = \"1\"\n";
        let got = bad(m);
        assert_eq!(
            names(&got),
            [
                "fireline-a",
                "fireline-b",
                "fireline-c",
                "fireline-d",
                "fireline-e"
            ]
        );
    }
    #[test]
    fn a_renamed_dependency_is_checked_by_its_package_name() {
        let m = "[dependencies]\nann = { package = \"fireline-announce\", version = \"^0.1\" }\nok = { package = \"fireline-synth\", version = \"~0.1\" }\n";
        assert_eq!(names(&bad(m)), ["ann"]);
    }
    #[test]
    fn the_bare_fireline_crate_is_covered() {
        assert_eq!(
            names(&bad("[dependencies]\nfireline = \"^0.1\"\n")),
            ["fireline"]
        );
    }
    #[test]
    fn workspace_inheritance_is_checked_where_the_version_lives() {
        let m = "[dependencies]\nfireline-announce = { workspace = true }\n[workspace.dependencies]\nfireline-announce = \"~0.1\"\n";
        assert!(bad(m).is_empty());
    }
    #[test]
    fn git_dependencies_and_versionless_paths_are_refused() {
        let m = "[dependencies]\nfireline-a = { git = \"https://github.com/ThinkersJournal/fireline\" }\nfireline-b = { path = \"../b\" }\n";
        assert_eq!(names(&bad(m)), ["fireline-a", "fireline-b"]);
    }
    #[test]
    fn a_path_dependency_with_a_tilde_version_passes() {
        assert!(
            bad("[dependencies]\nfireline-a = { path = \"crates/a\", version = \"~0.1\" }\n")
                .is_empty()
        );
    }
    #[test]
    fn a_package_that_is_not_a_dependency_table_is_not_read() {
        assert!(bad("[package]\nname = \"fireline-announce\"\nversion = \"0.1.0\"\n").is_empty());
    }
    #[test]
    fn malformed_toml_is_an_error_not_a_pass() {
        assert!(check_manifest("[dependencies\n").is_err());
    }
}
