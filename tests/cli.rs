use assert_cmd::Command;
use predicates::prelude::*;
use std::{fs, path::Path};
use tempfile::tempdir;

#[test]
fn init_dry_run_does_not_write() {
    let temp = tempdir().unwrap();
    Command::cargo_bin("chum")
        .unwrap()
        .current_dir(temp.path())
        .args(["init", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("chum init dry run"));

    assert!(!temp.path().join("archive").exists());
    assert!(!temp.path().join("chum.config.yaml").exists());
}

#[test]
fn check_reports_missing_specs_as_json() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("src")).unwrap();
    fs::write(
        temp.path().join("src/lib.rs"),
        "pub fn add(a: i32, b: i32) -> i32 { a + b }\n",
    )
    .unwrap();

    Command::cargo_bin("chum")
        .unwrap()
        .args(["check", "--json", temp.path().to_str().unwrap()])
        .assert()
        .failure()
        .stdout(predicate::str::contains("missing spec for source file"));
}

#[test]
fn swim_stubs_write_inline_specs() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("src")).unwrap();
    fs::write(
        temp.path().join("src/lib.rs"),
        "pub fn add(a: i32, b: i32) -> i32 { a + b }\n",
    )
    .unwrap();

    Command::cargo_bin("chum")
        .unwrap()
        .args(["swim", "--stubs", "--write", temp.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("created"));

    assert!(temp.path().join("src/lib.rs.spec.md").exists());
    assert!(temp.path().join("src/src.spec.md").exists());
    assert!(temp.path().join("repo.spec.md").exists());
}

#[test]
fn swim_json_reports_created_updated_skipped_and_unresolved_counts() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("src")).unwrap();
    fs::write(temp.path().join("src/lib.rs"), "pub fn add() {}\n").unwrap();

    Command::cargo_bin("chum")
        .unwrap()
        .args(["swim", "--stubs", "--json", temp.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"created\""))
        .stdout(predicate::str::contains("\"updated\""))
        .stdout(predicate::str::contains("\"skipped\""))
        .stdout(predicate::str::contains("\"unresolved\""));
}

#[test]
fn archive_dry_run_discovers_frontmatter_change() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("design")).unwrap();
    fs::write(
        temp.path().join("design/example.md"),
        "---\nchange: example\n---\n# Example\n",
    )
    .unwrap();

    Command::cargo_bin("chum")
        .unwrap()
        .args([
            "archive",
            "example",
            "--dry-run",
            temp.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("design/example.md"));
}

#[test]
fn archive_warns_but_does_not_block_on_failed_check() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("src")).unwrap();
    fs::create_dir_all(temp.path().join("design")).unwrap();
    fs::write(temp.path().join("src/lib.rs"), "pub fn undocumented() {}\n").unwrap();
    fs::write(
        temp.path().join("design/example.md"),
        "---\nchange: example\n---\n# Example\n",
    )
    .unwrap();

    Command::cargo_bin("chum")
        .unwrap()
        .args([
            "archive",
            "example",
            "--dry-run",
            temp.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("design/example.md"))
        .stderr(predicate::str::contains("chum check failed before archive"));
}

#[test]
fn check_respects_gitignore_and_chumignore() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("ignored-by-git")).unwrap();
    fs::create_dir_all(temp.path().join("ignored-by-chum")).unwrap();
    fs::write(temp.path().join(".gitignore"), "ignored-by-git/\n").unwrap();
    fs::write(temp.path().join(".chumignore"), "ignored-by-chum/\n").unwrap();
    fs::write(
        temp.path().join("ignored-by-git/lib.rs"),
        "pub fn git_ignored() {}\n",
    )
    .unwrap();
    fs::write(
        temp.path().join("ignored-by-chum/lib.rs"),
        "pub fn chum_ignored() {}\n",
    )
    .unwrap();

    Command::cargo_bin("chum")
        .unwrap()
        .args(["check", "--json", temp.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"source_files\": 0"));
}

#[test]
fn check_excludes_tests_configs_and_target_by_default() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("tests")).unwrap();
    fs::create_dir_all(temp.path().join("target/debug/build/pkg/out")).unwrap();
    fs::write(
        temp.path().join("tests/lib_test.rs"),
        "#[test]\nfn t() {}\n",
    )
    .unwrap();
    fs::write(temp.path().join("app.config.ts"), "export default {};\n").unwrap();
    fs::write(
        temp.path().join("target/debug/build/pkg/out/generated.rs"),
        "pub fn generated() {}\n",
    )
    .unwrap();

    Command::cargo_bin("chum")
        .unwrap()
        .args(["check", "--json", temp.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"source_files\": 0"));
}

#[test]
fn check_fails_on_todo_unknown_verify_and_allows_verify_flag() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("src")).unwrap();
    fs::write(temp.path().join("src/lib.rs"), "pub fn add() {}\n").unwrap();
    write_complete_specs(temp.path());

    let spec_path = temp.path().join("src/lib.rs.spec.md");
    let mut spec = fs::read_to_string(&spec_path).unwrap();
    spec = spec.replace("todo: []", "todo:\n- finish this");
    spec = spec.replace("unknowns: []", "unknowns:\n- explain this");
    spec = spec.replace("verify: []", "verify:\n- external service behavior");
    fs::write(&spec_path, spec).unwrap();

    Command::cargo_bin("chum")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("todo must be empty"))
        .stderr(predicate::str::contains("unknowns must be empty"))
        .stderr(predicate::str::contains("verify must be empty"));

    let mut spec = fs::read_to_string(&spec_path).unwrap();
    spec = spec.replace("todo:\n- finish this", "todo: []");
    spec = spec.replace("unknowns:\n- explain this", "unknowns: []");
    fs::write(&spec_path, spec).unwrap();

    Command::cargo_bin("chum")
        .unwrap()
        .args([
            "check",
            "--allow-external-verify",
            temp.path().to_str().unwrap(),
        ])
        .assert()
        .success();
}

#[test]
fn check_fails_on_legacy_markers_and_stale_hash() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("src")).unwrap();
    fs::write(temp.path().join("src/lib.rs"), "pub fn add() {}\n").unwrap();
    write_complete_specs(temp.path());

    let spec_path = temp.path().join("src/lib.rs.spec.md");
    let mut spec = fs::read_to_string(&spec_path).unwrap();
    spec = spec.replace("## Purpose\n\n", "## Purpose\n\n<!-- SPEC:TODO -->\n\n");
    fs::write(&spec_path, spec).unwrap();

    Command::cargo_bin("chum")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("contains SPEC:TODO marker"));

    let spec = fs::read_to_string(&spec_path)
        .unwrap()
        .replace("<!-- SPEC:TODO -->\n\n", "");
    fs::write(&spec_path, spec).unwrap();
    fs::write(
        temp.path().join("src/lib.rs"),
        "pub fn add() {}\npub fn sub() {}\n",
    )
    .unwrap();

    Command::cargo_bin("chum")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("source_hash is stale or missing"));
}

#[test]
fn archive_moves_docs_writes_manifest_rewrites_links_and_warns_on_assets() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("design")).unwrap();
    fs::create_dir_all(temp.path().join("plan/example")).unwrap();
    fs::create_dir_all(temp.path().join("assets")).unwrap();
    fs::write(temp.path().join("assets/diagram.png"), "not really png").unwrap();
    fs::write(
        temp.path().join("design/example.md"),
        "---\nchange: example\n---\n# Example\n\nSee [phase](../plan/example/phase-1.md) and [diagram](../assets/diagram.png).\n",
    )
    .unwrap();
    fs::write(
        temp.path().join("plan/example/phase-1.md"),
        "---\nchange: example\n---\n# Phase 1\n",
    )
    .unwrap();

    Command::cargo_bin("chum")
        .unwrap()
        .args(["archive", "example", temp.path().to_str().unwrap()])
        .assert()
        .success()
        .stderr(predicate::str::contains("linked local asset not archived"));

    assert!(!temp.path().join("design/example.md").exists());
    assert!(temp
        .path()
        .join("archive/example/design/example.md")
        .exists());
    assert!(temp.path().join("archive/example/plan/phase-1.md").exists());
    assert!(temp.path().join("archive/example/README.md").exists());

    let archived =
        fs::read_to_string(temp.path().join("archive/example/design/example.md")).unwrap();
    assert!(archived.contains("../plan/phase-1.md"));
    assert!(archived.contains("../../assets/diagram.png"));
}

#[test]
fn archive_discovers_folder_match_without_frontmatter() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("plan/example")).unwrap();
    fs::write(temp.path().join("plan/example/phase-1.md"), "# Phase 1\n").unwrap();

    Command::cargo_bin("chum")
        .unwrap()
        .args(["archive", "example", temp.path().to_str().unwrap()])
        .assert()
        .success();

    assert!(temp.path().join("archive/example/plan/phase-1.md").exists());
}

#[test]
fn archive_discovers_filename_match_without_frontmatter() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("design")).unwrap();
    fs::write(temp.path().join("design/example.md"), "# Example\n").unwrap();

    Command::cargo_bin("chum")
        .unwrap()
        .args(["archive", "example", temp.path().to_str().unwrap()])
        .assert()
        .success();

    assert!(temp
        .path()
        .join("archive/example/design/example.md")
        .exists());
}

#[test]
fn archive_supports_explicit_include_and_leaves_live_specs() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("design")).unwrap();
    fs::write(temp.path().join("design/other.md"), "# Other\n").unwrap();
    fs::write(temp.path().join("design/other.spec.md"), "# Spec\n").unwrap();

    Command::cargo_bin("chum")
        .unwrap()
        .args([
            "archive",
            "example",
            "--include",
            "design/other.md",
            "--include",
            "design/other.spec.md",
            temp.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    assert!(temp.path().join("archive/example/design/other.md").exists());
    assert!(temp.path().join("design/other.spec.md").exists());
    assert!(!temp
        .path()
        .join("archive/example/design/other.spec.md")
        .exists());
}

#[test]
fn archive_fails_closed_on_ambiguous_automatic_matches() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("design")).unwrap();
    fs::create_dir_all(temp.path().join("plan/example")).unwrap();
    fs::write(temp.path().join("design/example.md"), "# Example\n").unwrap();
    fs::write(temp.path().join("plan/example/phase-1.md"), "# Phase 1\n").unwrap();

    Command::cargo_bin("chum")
        .unwrap()
        .args([
            "archive",
            "example",
            "--dry-run",
            temp.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("ambiguous archive matches"));
}

fn write_complete_specs(root: &Path) {
    Command::cargo_bin("chum")
        .unwrap()
        .args(["swim", "--stubs", "--write", root.to_str().unwrap()])
        .assert()
        .success();

    for spec_path in [
        root.join("src/lib.rs.spec.md"),
        root.join("src/src.spec.md"),
        root.join("repo.spec.md"),
    ] {
        let spec = fs::read_to_string(&spec_path)
            .unwrap()
            .replace("<!-- SPEC:TODO -->", "Complete purpose.")
            .replace("<!-- SPEC:UNKNOWN -->", "- Complete dependencies.")
            .replace("todo:\n- Document file purpose.", "todo: []")
            .replace("todo:\n- Document directory purpose.", "todo: []")
            .replace(
                "unknowns:\n- Document key exports and dependencies.",
                "unknowns: []",
            )
            .replace(
                "unknowns:\n- Document dependencies and contracts.",
                "unknowns: []",
            );
        fs::write(spec_path, spec).unwrap();
    }
}
