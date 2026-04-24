use crate::{
    cli::SwimArgs,
    config::{normalize_root, Config},
    discovery::{discover, DiscoverOptions, SourceDir, SourceFile},
    docs::backmatter::{self, Backmatter, SpecKind},
    output,
    provider::{
        openai::OpenAiProvider, ChumSwimProvider, DirectorySpecInput, FileSpecInput,
        RepairSpecInput,
    },
    spec,
};
use anyhow::{bail, Result};
use camino::{Utf8Path, Utf8PathBuf};
use serde::Serialize;
use std::{collections::BTreeMap, fs};

#[derive(Debug, Serialize)]
struct SwimReport {
    root: Utf8PathBuf,
    dry_run: bool,
    created: Vec<Utf8PathBuf>,
    updated: Vec<Utf8PathBuf>,
    skipped: Vec<Utf8PathBuf>,
    unresolved: Vec<UnresolvedSpec>,
}

#[derive(Debug, Serialize)]
struct UnresolvedSpec {
    path: Utf8PathBuf,
    todo: usize,
    unknowns: usize,
    verify: usize,
}

pub fn run(mut args: SwimArgs) -> Result<()> {
    let root = normalize_root(&args.path)?;
    let mut config = Config::load(root.as_std_path())?;
    if let Some(max_passes) = args.max_passes {
        config.swim.max_passes = max_passes;
    }
    if args.allow_external_verify {
        config.swim.allow_external_verify = true;
    }
    let dry_run = args.dry_run || !args.write;
    if dry_run {
        args.write = false;
    }
    let discovery = discover(&root, &config, &DiscoverOptions::default())?;
    let mut report = SwimReport {
        root: root.clone(),
        dry_run,
        created: Vec::new(),
        updated: Vec::new(),
        skipped: Vec::new(),
        unresolved: Vec::new(),
    };

    if args.stubs {
        write_stub_specs(
            &root,
            &config,
            &discovery.source_files,
            &discovery.source_dirs,
            &args,
            &mut report,
        )?;
    } else {
        write_provider_specs(
            &root,
            &config,
            &discovery.source_files,
            &discovery.source_dirs,
            &args,
            &mut report,
        )?;
    }

    collect_unresolved(&mut report)?;

    if args.json {
        output::print_json(&report)?;
    } else {
        println!(
            "chum swim {}: {} created, {} updated, {} skipped",
            if dry_run { "dry run" } else { "complete" },
            report.created.len(),
            report.updated.len(),
            report.skipped.len()
        );
        for unresolved in &report.unresolved {
            eprintln!(
                "- unresolved {}: todo={}, unknowns={}, verify={}",
                unresolved.path, unresolved.todo, unresolved.unknowns, unresolved.verify
            );
        }
    }

    if !args.stubs
        && report.unresolved.iter().any(|item| {
            item.todo > 0
                || item.unknowns > 0
                || item.verify > 0 && !config.swim.allow_external_verify
        })
    {
        bail!("swim did not converge");
    }
    Ok(())
}

fn write_stub_specs(
    root: &Utf8Path,
    config: &Config,
    source_files: &[SourceFile],
    source_dirs: &[SourceDir],
    args: &SwimArgs,
    report: &mut SwimReport,
) -> Result<()> {
    for source in source_files {
        if is_current(&source.spec_path, source)? {
            report.skipped.push(source.spec_path.clone());
            continue;
        }
        let backmatter = spec::file_backmatter(
            &source.rel_path,
            source.abs_path.as_std_path(),
            "chum swim --stubs",
        )?;
        let mut backmatter = backmatter;
        backmatter.todo = vec!["Document file purpose.".into()];
        backmatter.unknowns = vec!["Document key exports and dependencies.".into()];
        let markdown = file_stub(&source.rel_path, &backmatter)?;
        write_or_plan(&source.spec_path, markdown, args.write, report)?;
    }

    let mut dirs = source_dirs.to_vec();
    dirs.sort_by(|a, b| depth(&b.rel_path).cmp(&depth(&a.rel_path)));
    for dir in &dirs {
        if is_complete_directory_spec(&dir.spec_path)? {
            report.skipped.push(dir.spec_path.clone());
            continue;
        }
        let children = child_specs(root, &dir.rel_path, source_files, source_dirs, config);
        let mut bm = spec::directory_backmatter(&dir.rel_path, children, "chum swim --stubs");
        bm.todo = vec!["Document directory purpose.".into()];
        bm.unknowns = vec!["Document dependencies and contracts.".into()];
        let markdown = dir_stub(&dir.rel_path, source_files, source_dirs, &bm)?;
        write_or_plan(&dir.spec_path, markdown, args.write, report)?;
    }
    Ok(())
}

fn write_provider_specs(
    root: &Utf8Path,
    config: &Config,
    source_files: &[SourceFile],
    source_dirs: &[SourceDir],
    args: &SwimArgs,
    report: &mut SwimReport,
) -> Result<()> {
    if args.provider != "openai" {
        bail!("unsupported provider `{}`", args.provider);
    }
    let provider = OpenAiProvider::from_environment()?;
    for source in source_files {
        if is_current(&source.spec_path, source)? {
            report.skipped.push(source.spec_path.clone());
            continue;
        }
        let source_text = fs::read_to_string(&source.abs_path)?;
        let existing_spec = fs::read_to_string(&source.spec_path).ok();
        let draft = provider.generate_file_spec(FileSpecInput {
            target: source.rel_path.to_string(),
            source: source_text,
            existing_spec,
        })?;
        let markdown = ensure_file_backmatter(draft.markdown, source)?;
        write_or_plan(&source.spec_path, markdown, args.write, report)?;
    }

    let mut dirs = source_dirs.to_vec();
    dirs.sort_by(|a, b| depth(&b.rel_path).cmp(&depth(&a.rel_path)));
    for dir in &dirs {
        let child_specs = read_child_specs(root, &dir.rel_path, source_files, source_dirs, config);
        let existing_spec = fs::read_to_string(&dir.spec_path).ok();
        let draft = provider.generate_directory_spec(DirectorySpecInput {
            target: spec::path_to_slash(&dir.rel_path),
            child_specs,
            existing_spec,
        })?;
        let markdown =
            ensure_dir_backmatter(draft.markdown, root, dir, source_files, source_dirs, config)?;
        write_or_plan(&dir.spec_path, markdown, args.write, report)?;
    }
    if args.write {
        repair_until_converged(root, config, source_files, source_dirs, &provider, report)?;
    }
    Ok(())
}

fn repair_until_converged(
    root: &Utf8Path,
    config: &Config,
    source_files: &[SourceFile],
    source_dirs: &[SourceDir],
    provider: &dyn ChumSwimProvider,
    report: &mut SwimReport,
) -> Result<()> {
    for _pass in 1..config.swim.max_passes {
        let unresolved = unresolved_targets(source_files, source_dirs)?;
        if unresolved.is_empty() {
            return Ok(());
        }
        for target in unresolved {
            match target {
                RepairTarget::File(source) => {
                    let current_spec = fs::read_to_string(&source.spec_path)?;
                    let source_text = fs::read_to_string(&source.abs_path)?;
                    let draft = provider.repair_spec(RepairSpecInput {
                        target: source.rel_path.to_string(),
                        current_spec,
                        context: vec![(source.rel_path.to_string(), source_text)],
                    })?;
                    let markdown = ensure_file_backmatter(draft.markdown, source)?;
                    write_or_plan(&source.spec_path, markdown, true, report)?;
                }
                RepairTarget::Directory(dir) => {
                    let current_spec = fs::read_to_string(&dir.spec_path)?;
                    let context =
                        read_child_specs(root, &dir.rel_path, source_files, source_dirs, config);
                    let draft = provider.repair_spec(RepairSpecInput {
                        target: spec::path_to_slash(&dir.rel_path),
                        current_spec,
                        context,
                    })?;
                    let markdown = ensure_dir_backmatter(
                        draft.markdown,
                        root,
                        dir,
                        source_files,
                        source_dirs,
                        config,
                    )?;
                    write_or_plan(&dir.spec_path, markdown, true, report)?;
                }
            }
        }
    }
    Ok(())
}

enum RepairTarget<'a> {
    File(&'a SourceFile),
    Directory(&'a SourceDir),
}

fn unresolved_targets<'a>(
    source_files: &'a [SourceFile],
    source_dirs: &'a [SourceDir],
) -> Result<Vec<RepairTarget<'a>>> {
    let mut targets = Vec::new();
    for source in source_files {
        if has_unresolved_items(&source.spec_path)? {
            targets.push(RepairTarget::File(source));
        }
    }
    for dir in source_dirs {
        if has_unresolved_items(&dir.spec_path)? {
            targets.push(RepairTarget::Directory(dir));
        }
    }
    Ok(targets)
}

fn has_unresolved_items(path: &Utf8Path) -> Result<bool> {
    if !path.exists() {
        return Ok(false);
    }
    let parsed = backmatter::parse_file(path.as_std_path())?;
    Ok(!parsed.backmatter.todo.is_empty()
        || !parsed.backmatter.unknowns.is_empty()
        || !parsed.backmatter.verify.is_empty())
}

fn write_or_plan(
    path: &Utf8Path,
    content: String,
    write: bool,
    report: &mut SwimReport,
) -> Result<()> {
    if path.exists() {
        let existing = fs::read_to_string(path)?;
        if existing == content {
            report.skipped.push(path.to_path_buf());
            return Ok(());
        }
        report.updated.push(path.to_path_buf());
    } else {
        report.created.push(path.to_path_buf());
    }
    if write {
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(path, content)?;
    }
    Ok(())
}

fn file_stub(rel: &Utf8Path, backmatter: &Backmatter) -> Result<String> {
    let body = format!(
        "# `{}`\n\n## Purpose\n\n<!-- SPEC:TODO -->\n\n## Key Exports\n\n<!-- SPEC:UNKNOWN -->\n\n## Dependencies / Contracts\n\n<!-- SPEC:UNKNOWN -->\n",
        rel
    );
    backmatter::replace_or_append(&body, backmatter)
}

fn dir_stub(
    rel: &Utf8Path,
    source_files: &[SourceFile],
    source_dirs: &[SourceDir],
    backmatter: &Backmatter,
) -> Result<String> {
    let title = if rel.as_str().is_empty() {
        "."
    } else {
        rel.as_str()
    };
    let mut body = format!("# `{title}/`\n\n## Purpose\n\n<!-- SPEC:TODO -->\n\n## Files\n\n");
    for file in source_files
        .iter()
        .filter(|file| file.rel_path.parent() == Some(rel))
    {
        body.push_str(&format!(
            "- `{}`\n",
            file.rel_path.file_name().unwrap_or(file.rel_path.as_str())
        ));
    }
    body.push_str("\n## Subfolders\n\n");
    for dir in source_dirs
        .iter()
        .filter(|dir| dir.rel_path.parent() == Some(rel) && dir.rel_path != *rel)
    {
        if !dir.rel_path.as_str().is_empty() {
            body.push_str(&format!(
                "- `{}/`\n",
                dir.rel_path.file_name().unwrap_or(dir.rel_path.as_str())
            ));
        }
    }
    body.push_str("\n## Dependencies / Contracts\n\n<!-- SPEC:UNKNOWN -->\n");
    backmatter::replace_or_append(&body, backmatter)
}

fn ensure_file_backmatter(markdown: String, source: &SourceFile) -> Result<String> {
    let mut bm =
        spec::file_backmatter(&source.rel_path, source.abs_path.as_std_path(), "chum swim")?;
    if let Ok(parsed) = backmatter::parse(&markdown) {
        bm.todo = parsed.backmatter.todo;
        bm.unknowns = parsed.backmatter.unknowns;
        bm.verify = parsed.backmatter.verify;
    }
    backmatter::replace_or_append(&markdown, &bm)
}

fn ensure_dir_backmatter(
    markdown: String,
    root: &Utf8Path,
    dir: &SourceDir,
    source_files: &[SourceFile],
    source_dirs: &[SourceDir],
    config: &Config,
) -> Result<String> {
    let children = child_specs(root, &dir.rel_path, source_files, source_dirs, config);
    let mut bm = spec::directory_backmatter(&dir.rel_path, children, "chum swim");
    if let Ok(parsed) = backmatter::parse(&markdown) {
        bm.todo = parsed.backmatter.todo;
        bm.unknowns = parsed.backmatter.unknowns;
        bm.verify = parsed.backmatter.verify;
    }
    backmatter::replace_or_append(&markdown, &bm)
}

fn is_current(spec_path: &Utf8Path, source: &SourceFile) -> Result<bool> {
    if !spec_path.exists() {
        return Ok(false);
    }
    let Ok(parsed) = backmatter::parse_file(spec_path.as_std_path()) else {
        return Ok(false);
    };
    if parsed.backmatter.kind != SpecKind::File {
        return Ok(false);
    }
    let actual = spec::sha256_file(source.abs_path.as_std_path())?;
    Ok(
        parsed.backmatter.source_hash.as_deref() == Some(actual.as_str())
            && parsed.backmatter.todo.is_empty()
            && parsed.backmatter.unknowns.is_empty()
            && parsed.backmatter.verify.is_empty(),
    )
}

fn is_complete_directory_spec(spec_path: &Utf8Path) -> Result<bool> {
    if !spec_path.exists() {
        return Ok(false);
    }
    let Ok(parsed) = backmatter::parse_file(spec_path.as_std_path()) else {
        return Ok(false);
    };
    Ok(parsed.backmatter.kind == SpecKind::Directory
        && parsed.backmatter.todo.is_empty()
        && parsed.backmatter.unknowns.is_empty()
        && parsed.backmatter.verify.is_empty())
}

fn collect_unresolved(report: &mut SwimReport) -> Result<()> {
    let mut paths = Vec::new();
    paths.extend(report.created.iter().cloned());
    paths.extend(report.updated.iter().cloned());
    for path in paths {
        if !path.exists() {
            continue;
        }
        if let Ok(parsed) = backmatter::parse_file(path.as_std_path()) {
            if !parsed.backmatter.todo.is_empty()
                || !parsed.backmatter.unknowns.is_empty()
                || !parsed.backmatter.verify.is_empty()
            {
                report.unresolved.push(UnresolvedSpec {
                    path,
                    todo: parsed.backmatter.todo.len(),
                    unknowns: parsed.backmatter.unknowns.len(),
                    verify: parsed.backmatter.verify.len(),
                });
            }
        }
    }
    Ok(())
}

fn depth(path: &Utf8Path) -> usize {
    path.components().count()
}

fn child_specs(
    root: &Utf8Path,
    rel_dir: &Utf8Path,
    source_files: &[SourceFile],
    source_dirs: &[SourceDir],
    config: &Config,
) -> Vec<String> {
    let mut children = Vec::new();
    for file in source_files
        .iter()
        .filter(|file| file.rel_path.parent() == Some(rel_dir))
    {
        if let Ok(rel) = file.spec_path.strip_prefix(root) {
            children.push(rel.to_string());
        }
    }
    for dir in source_dirs
        .iter()
        .filter(|dir| dir.rel_path.parent() == Some(rel_dir) && dir.rel_path != *rel_dir)
    {
        if let Ok(rel) = spec::dir_spec_path(root, &dir.rel_path, config).strip_prefix(root) {
            children.push(rel.to_string());
        }
    }
    children.sort();
    children
}

fn read_child_specs(
    root: &Utf8Path,
    rel_dir: &Utf8Path,
    source_files: &[SourceFile],
    source_dirs: &[SourceDir],
    config: &Config,
) -> Vec<(String, String)> {
    let mut specs = BTreeMap::new();
    for child in child_specs(root, rel_dir, source_files, source_dirs, config) {
        if let Ok(content) = fs::read_to_string(root.join(&child)) {
            specs.insert(child, content);
        }
    }
    specs.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::{DirectorySpecInput, FileSpecInput, RepairSpecInput, SpecDraft};
    use tempfile::tempdir;

    struct FakeProvider {
        clear: bool,
    }

    impl ChumSwimProvider for FakeProvider {
        fn generate_file_spec(&self, _input: FileSpecInput) -> Result<SpecDraft> {
            unreachable!("repair tests do not generate file specs")
        }

        fn generate_directory_spec(&self, _input: DirectorySpecInput) -> Result<SpecDraft> {
            unreachable!("repair tests do not generate directory specs")
        }

        fn repair_spec(&self, input: RepairSpecInput) -> Result<SpecDraft> {
            if self.clear {
                Ok(SpecDraft {
                    markdown: format!("# `{}`\n\n## Purpose\n\nRepaired.\n", input.target),
                })
            } else {
                Ok(SpecDraft {
                    markdown: format!(
                        "# `{}`\n\n## Purpose\n\nStill incomplete.\n\n<!-- chum:backmatter\nschema: 1\nkind: file\ntarget: {}\ntodo:\n- still unresolved\nunknowns: []\nverify: []\n-->\n",
                        input.target, input.target
                    ),
                })
            }
        }
    }

    #[test]
    fn fake_provider_repair_can_converge() {
        let temp = tempdir().unwrap();
        let (root, source) = incomplete_source_fixture(temp.path());
        let mut config = Config::default();
        config.swim.max_passes = 3;
        let mut report = SwimReport {
            root: root.clone(),
            dry_run: false,
            created: Vec::new(),
            updated: Vec::new(),
            skipped: Vec::new(),
            unresolved: Vec::new(),
        };

        repair_until_converged(
            &root,
            &config,
            std::slice::from_ref(&source),
            &[],
            &FakeProvider { clear: true },
            &mut report,
        )
        .unwrap();

        assert!(!has_unresolved_items(&source.spec_path).unwrap());
        assert!(!report.updated.is_empty());
    }

    #[test]
    fn fake_provider_repair_leaves_non_converging_items() {
        let temp = tempdir().unwrap();
        let (root, source) = incomplete_source_fixture(temp.path());
        let mut config = Config::default();
        config.swim.max_passes = 2;
        let mut report = SwimReport {
            root: root.clone(),
            dry_run: false,
            created: Vec::new(),
            updated: Vec::new(),
            skipped: Vec::new(),
            unresolved: Vec::new(),
        };

        repair_until_converged(
            &root,
            &config,
            std::slice::from_ref(&source),
            &[],
            &FakeProvider { clear: false },
            &mut report,
        )
        .unwrap();

        assert!(has_unresolved_items(&source.spec_path).unwrap());
    }

    fn incomplete_source_fixture(path: &std::path::Path) -> (Utf8PathBuf, SourceFile) {
        let root = Utf8PathBuf::from_path_buf(path.to_path_buf()).unwrap();
        let rel = Utf8PathBuf::from("src/lib.rs");
        let abs = root.join(&rel);
        fs::create_dir_all(abs.parent().unwrap()).unwrap();
        fs::write(&abs, "pub fn add() {}\n").unwrap();
        let spec_path = spec::file_spec_path(&root, &rel);
        let mut bm = spec::file_backmatter(&rel, abs.as_std_path(), "test").unwrap();
        bm.todo = vec!["repair me".into()];
        let markdown = file_stub(&rel, &bm).unwrap();
        fs::write(&spec_path, markdown).unwrap();
        (
            root,
            SourceFile {
                rel_path: rel,
                abs_path: abs,
                spec_path,
            },
        )
    }
}
