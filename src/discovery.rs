use crate::{config::Config, spec};
use anyhow::{Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::WalkBuilder;
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Serialize)]
pub struct SourceFile {
    pub rel_path: Utf8PathBuf,
    pub abs_path: Utf8PathBuf,
    pub spec_path: Utf8PathBuf,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceDir {
    pub rel_path: Utf8PathBuf,
    pub abs_path: Utf8PathBuf,
    pub spec_path: Utf8PathBuf,
}

#[derive(Debug, Clone, Serialize)]
pub struct Discovery {
    pub root: Utf8PathBuf,
    pub source_files: Vec<SourceFile>,
    pub source_dirs: Vec<SourceDir>,
    pub live_specs: Vec<Utf8PathBuf>,
    pub active_docs: Vec<Utf8PathBuf>,
    pub archive_docs: Vec<Utf8PathBuf>,
    pub ignored_count: usize,
}

#[derive(Debug, Clone, Default)]
pub struct DiscoverOptions {
    pub explicit_include: Vec<String>,
    pub include_archive: bool,
}

pub fn discover(root: &Utf8Path, config: &Config, options: &DiscoverOptions) -> Result<Discovery> {
    let include = globset(&config.source.include)?;
    let exclude = globset(&config.source.exclude)?;
    let explicit_include = globset(&options.explicit_include)?;
    let has_explicit_include = !options.explicit_include.is_empty();
    let archive_prefix = Utf8Path::new(&config.archive_dir);
    let active_dirs: BTreeSet<&str> = config.active_dirs.iter().map(String::as_str).collect();

    let mut walk = WalkBuilder::new(root);
    walk.git_ignore(config.source.respect_gitignore);
    walk.git_global(config.source.respect_gitignore);
    walk.git_exclude(config.source.respect_gitignore);
    for ignore_file in &config.source.ignore_files {
        walk.add_custom_ignore_filename(ignore_file);
    }

    let mut source_files = Vec::new();
    let mut source_dirs = BTreeSet::new();
    let mut live_specs = Vec::new();
    let mut active_docs = Vec::new();
    let mut archive_docs = Vec::new();
    let mut ignored_count = 0;

    for entry in walk.build() {
        let entry = entry?;
        if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            continue;
        }
        let abs = Utf8PathBuf::from_path_buf(entry.path().to_path_buf())
            .map_err(|path| anyhow::anyhow!("path is not valid UTF-8: {}", path.display()))?;
        let rel = abs
            .strip_prefix(root)
            .with_context(|| format!("failed to relativize {}", abs))?
            .to_path_buf();

        if is_archive_doc(&rel, archive_prefix) {
            archive_docs.push(rel.clone());
            if !options.include_archive {
                ignored_count += 1;
                continue;
            }
        }

        if is_live_spec(&rel) {
            live_specs.push(rel.clone());
            continue;
        }
        if is_active_doc(&rel, &active_dirs) {
            active_docs.push(rel.clone());
            continue;
        }

        let explicitly_included = has_explicit_include && explicit_include.is_match(rel.as_str());
        if has_explicit_include && !explicitly_included {
            ignored_count += 1;
            continue;
        }
        if !explicitly_included && !include.is_match(rel.as_str()) {
            ignored_count += 1;
            continue;
        }
        if !explicitly_included && exclude.is_match(rel.as_str()) {
            ignored_count += 1;
            continue;
        }
        if is_markdown_or_text(&rel) {
            ignored_count += 1;
            continue;
        }

        let spec_path = spec::file_spec_path(root, &rel);
        source_files.push(SourceFile {
            rel_path: rel.clone(),
            abs_path: abs,
            spec_path,
        });

        let mut current = rel.parent();
        while let Some(dir) = current {
            source_dirs.insert(dir.to_path_buf());
            current = dir.parent();
        }
        source_dirs.insert(Utf8PathBuf::from(""));
    }

    let source_dirs = source_dirs
        .into_iter()
        .map(|rel_path| SourceDir {
            abs_path: if rel_path.as_str().is_empty() {
                root.to_path_buf()
            } else {
                root.join(&rel_path)
            },
            spec_path: spec::dir_spec_path(root, &rel_path, config),
            rel_path,
        })
        .collect();

    source_files.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    live_specs.sort();
    active_docs.sort();
    archive_docs.sort();

    Ok(Discovery {
        root: root.to_path_buf(),
        source_files,
        source_dirs,
        live_specs,
        active_docs,
        archive_docs,
        ignored_count,
    })
}

fn globset(patterns: &[String]) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        builder.add(Glob::new(pattern).with_context(|| format!("invalid glob `{pattern}`"))?);
    }
    Ok(builder.build()?)
}

fn is_live_spec(path: &Utf8Path) -> bool {
    path.as_str().ends_with(".spec.md")
}

fn is_archive_doc(path: &Utf8Path, archive_prefix: &Utf8Path) -> bool {
    path.starts_with(archive_prefix) && is_markdown(path)
}

fn is_active_doc(path: &Utf8Path, active_dirs: &BTreeSet<&str>) -> bool {
    path.components()
        .next()
        .and_then(|component| {
            component
                .as_str()
                .strip_suffix('/')
                .or(Some(component.as_str()))
        })
        .map(|first| active_dirs.contains(first) && is_markdown(path))
        .unwrap_or(false)
}

fn is_markdown(path: &Utf8Path) -> bool {
    matches!(path.extension(), Some("md" | "markdown"))
}

fn is_markdown_or_text(path: &Utf8Path) -> bool {
    matches!(
        path.extension(),
        Some("md" | "markdown" | "txt" | "rst" | "adoc")
    )
}
