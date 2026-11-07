//! Tilde lint: RED STUB. Returns no violations, so the tests below must fail.

/// One offending requirement.
#[derive(Debug, PartialEq, Eq)]
pub struct Violation {
    /// The dependency table, for example `dependencies` or `target.cfg(unix).dev-dependencies`.
    pub table: String,
    /// The dependency's crate name.
    pub name: String,
    /// Why it was refused.
    pub reason: String,
}

/// Check one manifest's text.
pub fn check_manifest(_text: &str) -> Result<Vec<Violation>, String> {
    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bad(text: &str) -> Vec<(String, String)> {
        check_manifest(text).unwrap().into_iter().map(|v| (v.table, v.name)).collect()
    }
    fn names(v: &[(String, String)]) -> Vec<&str> {
        v.iter().map(|(_, n)| n.as_str()).collect()
    }

    #[test]
    fn explicit_caret_is_refused() {
        assert_eq!(names(&bad("[dependencies]\nfireline-announce = \"^0.1\"\n")), ["fireline-announce"]);
    }
    #[test]
    fn bare_version_is_an_implicit_caret_and_is_refused() {
        assert_eq!(names(&bad("[dependencies]\nfireline-grammar = \"0.1.2\"\n")), ["fireline-grammar"]);
        assert_eq!(names(&bad("[dependencies]\nfireline-grammar = { version = \"0.1\" }\n")), ["fireline-grammar"]);
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
        assert_eq!(names(&got), ["fireline-a", "fireline-b", "fireline-c", "fireline-d", "fireline-e"]);
    }
    #[test]
    fn a_renamed_dependency_is_checked_by_its_package_name() {
        let m = "[dependencies]\nann = { package = \"fireline-announce\", version = \"^0.1\" }\nok = { package = \"fireline-synth\", version = \"~0.1\" }\n";
        assert_eq!(names(&bad(m)), ["ann"]);
    }
    #[test]
    fn the_bare_fireline_crate_is_covered() {
        assert_eq!(names(&bad("[dependencies]\nfireline = \"^0.1\"\n")), ["fireline"]);
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
        assert!(bad("[dependencies]\nfireline-a = { path = \"crates/a\", version = \"~0.1\" }\n").is_empty());
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
