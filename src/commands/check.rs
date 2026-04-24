use crate::{
    cli::CheckArgs,
    config::{normalize_root, Config},
    discovery::{discover, DiscoverOptions},
    docs::backmatter::{parse_file, SpecKind},
    output, spec,
};
use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CheckReport {
    pub root: Utf8PathBuf,
    pub source_files: usize,
    pub source_dirs: usize,
    pub ignored_count: usize,
    pub failures: Vec<CheckFailure>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CheckFailure {
    pub path: Utf8PathBuf,
    pub message: String,
}

impl CheckReport {
    pub fn has_errors(&self) -> bool {
        !self.failures.is_empty()
    }
}

pub fn run_report(args: CheckArgs) -> Result<CheckReport> {
    let root = normalize_root(&args.path)?;
    let config = Config::load(root.as_std_path())?;
    let discovery = discover(
        &root,
        &config,
        &DiscoverOptions {
            explicit_include: args.include.clone(),
            include_archive: args.include_archive,
        },
    )?;
    let mut failures = Vec::new();

    for source in &discovery.source_files {
        if !source.spec_path.exists() {
            failures.push(CheckFailure {
                path: source.spec_path.clone(),
                message: format!("missing spec for source file `{}`", source.rel_path),
            });
            continue;
        }
        match parse_file(source.spec_path.as_std_path()) {
            Ok(parsed) => {
                let bm = parsed.backmatter;
                if bm.kind != SpecKind::File {
                    failures.push(failure(&source.spec_path, "expected kind `file`"));
                }
                let expected_target = spec::path_to_slash(&source.rel_path);
                if bm.target != expected_target {
                    failures.push(failure(
                        &source.spec_path,
                        format!(
                            "target `{}` does not match `{}`",
                            bm.target, expected_target
                        ),
                    ));
                }
                if !args.allow_stale {
                    let actual_hash = spec::sha256_file(source.abs_path.as_std_path())?;
                    if bm.source_hash.as_deref() != Some(actual_hash.as_str()) {
                        failures.push(failure(
                            &source.spec_path,
                            "source_hash is stale or missing",
                        ));
                    }
                }
                validate_open_items(&mut failures, &source.spec_path, &bm.todo, "todo", false);
                validate_open_items(
                    &mut failures,
                    &source.spec_path,
                    &bm.unknowns,
                    "unknowns",
                    false,
                );
                validate_open_items(
                    &mut failures,
                    &source.spec_path,
                    &bm.verify,
                    "verify",
                    args.allow_external_verify,
                );
            }
            Err(error) => failures.push(failure(&source.spec_path, error.to_string())),
        }
        validate_markers(&mut failures, &source.spec_path, args.allow_external_verify)?;
    }

    for dir in &discovery.source_dirs {
        if !dir.spec_path.exists() {
            failures.push(CheckFailure {
                path: dir.spec_path.clone(),
                message: format!(
                    "missing spec for source directory `{}`",
                    display_rel(&dir.rel_path)
                ),
            });
            continue;
        }
        match parse_file(dir.spec_path.as_std_path()) {
            Ok(parsed) => {
                let bm = parsed.backmatter;
                if bm.kind != SpecKind::Directory {
                    failures.push(failure(&dir.spec_path, "expected kind `directory`"));
                }
                let expected_target = spec::path_to_slash(&dir.rel_path);
                if bm.target != expected_target {
                    failures.push(failure(
                        &dir.spec_path,
                        format!(
                            "target `{}` does not match `{}`",
                            bm.target, expected_target
                        ),
                    ));
                }
                validate_open_items(&mut failures, &dir.spec_path, &bm.todo, "todo", false);
                validate_open_items(
                    &mut failures,
                    &dir.spec_path,
                    &bm.unknowns,
                    "unknowns",
                    false,
                );
                validate_open_items(
                    &mut failures,
                    &dir.spec_path,
                    &bm.verify,
                    "verify",
                    args.allow_external_verify,
                );
            }
            Err(error) => failures.push(failure(&dir.spec_path, error.to_string())),
        }
        validate_markers(&mut failures, &dir.spec_path, args.allow_external_verify)?;
    }

    Ok(CheckReport {
        root,
        source_files: discovery.source_files.len(),
        source_dirs: discovery.source_dirs.len(),
        ignored_count: discovery.ignored_count,
        failures,
        warnings: Vec::new(),
    })
}

pub fn print_report(report: &CheckReport) {
    if report.failures.is_empty() {
        println!(
            "chum check passed: {} source files, {} source directories",
            report.source_files, report.source_dirs
        );
    } else {
        eprintln!(
            "chum check failed: {} issue(s), {} source files, {} source directories",
            report.failures.len(),
            report.source_files,
            report.source_dirs
        );
        for failure in &report.failures {
            eprintln!("- {}: {}", failure.path, failure.message);
        }
    }
}

pub fn print_report_json(report: &CheckReport) -> Result<()> {
    output::print_json(report)
}

fn validate_open_items(
    failures: &mut Vec<CheckFailure>,
    path: &Utf8Path,
    items: &[String],
    field: &str,
    allowed: bool,
) {
    if !items.is_empty() && !allowed {
        failures.push(failure(
            path,
            format!("{field} must be empty ({} item(s))", items.len()),
        ));
    }
}

fn validate_markers(
    failures: &mut Vec<CheckFailure>,
    path: &Utf8Path,
    allow_external_verify: bool,
) -> Result<()> {
    let content = std::fs::read_to_string(path.as_std_path())?;
    if content.contains("SPEC:TODO") {
        failures.push(failure(path, "contains SPEC:TODO marker"));
    }
    if content.contains("SPEC:UNKNOWN") {
        failures.push(failure(path, "contains SPEC:UNKNOWN marker"));
    }
    if content.contains("SPEC:VERIFY") && !allow_external_verify {
        failures.push(failure(path, "contains SPEC:VERIFY marker"));
    }
    Ok(())
}

fn failure(path: &Utf8Path, message: impl Into<String>) -> CheckFailure {
    CheckFailure {
        path: path.to_path_buf(),
        message: message.into(),
    }
}

fn display_rel(path: &Utf8Path) -> String {
    if path.as_str().is_empty() {
        ".".into()
    } else {
        path.to_string()
    }
}
