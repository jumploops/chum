use crate::{
    cli::{ArchiveArgs, CheckArgs},
    commands::check,
    config::{normalize_root, Config},
    discovery::{discover, DiscoverOptions},
    docs::{frontmatter, links},
    output,
};
use anyhow::{bail, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use globset::{Glob, GlobSetBuilder};
use serde::Serialize;
use std::{
    collections::{BTreeSet, HashMap},
    fs,
};

#[derive(Debug, Serialize)]
struct ArchiveReport {
    change_id: String,
    root: Utf8PathBuf,
    dry_run: bool,
    check_status: String,
    moved: Vec<ArchiveMove>,
    warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct ArchiveMove {
    from: Utf8PathBuf,
    to: Utf8PathBuf,
}

pub fn run(args: ArchiveArgs) -> Result<()> {
    let root = normalize_root(&args.path)?;
    let config = Config::load(root.as_std_path())?;
    let check_args = CheckArgs {
        json: false,
        allow_external_verify: false,
        include: Vec::new(),
        include_archive: false,
        allow_stale: false,
        path: root.as_std_path().to_path_buf(),
    };
    let check_report = check::run_report(check_args)?;
    let check_status = if check_report.has_errors() {
        "failed"
    } else {
        "passed"
    }
    .to_string();

    let mut warnings = Vec::new();
    if check_report.has_errors() {
        warnings.push(format!(
            "chum check failed before archive with {} issue(s)",
            check_report.failures.len()
        ));
    }

    let moves = plan_moves(&root, &config, &args)?;
    if moves.is_empty() {
        bail!(
            "no active Markdown docs matched change `{}`",
            args.change_id
        );
    }

    let report = ArchiveReport {
        change_id: args.change_id.clone(),
        root: root.clone(),
        dry_run: args.dry_run,
        check_status,
        moved: moves.clone(),
        warnings: warnings.clone(),
    };

    if args.dry_run {
        if args.json {
            output::print_json(&report)?;
        } else {
            println!("chum archive dry run: {} file(s)", moves.len());
            for item in &moves {
                println!("- {} -> {}", item.from, item.to);
            }
            for warning in &warnings {
                eprintln!("warning: {warning}");
            }
        }
        return Ok(());
    }

    let mut move_map = HashMap::new();
    for item in &moves {
        move_map.insert(
            root.join(&item.from).into_std_path_buf(),
            root.join(&item.to).into_std_path_buf(),
        );
    }

    let mut final_warnings = warnings;
    for item in &moves {
        let old_abs = root.join(&item.from);
        let new_abs = root.join(&item.to);
        if new_abs.exists() {
            bail!("archive destination already exists: {}", new_abs);
        }
        let content =
            fs::read_to_string(&old_abs).with_context(|| format!("failed to read {}", old_abs))?;
        let rewrite = links::rewrite_markdown_links(
            &content,
            old_abs.as_std_path(),
            new_abs.as_std_path(),
            &move_map,
            root.as_std_path(),
        );
        final_warnings.extend(rewrite.warnings);
        fs::create_dir_all(new_abs.parent().unwrap())?;
        fs::write(&new_abs, rewrite.content)?;
    }
    for item in &moves {
        fs::remove_file(root.join(&item.from))?;
    }

    write_manifest(&root, &args, &moves, &final_warnings, &report.check_status)?;

    let final_report = ArchiveReport {
        warnings: final_warnings,
        ..report
    };
    if args.json {
        output::print_json(&final_report)?;
    } else {
        println!("chum archive moved {} file(s)", final_report.moved.len());
        for item in &final_report.moved {
            println!("- {} -> {}", item.from, item.to);
        }
        for warning in &final_report.warnings {
            eprintln!("warning: {warning}");
        }
    }
    Ok(())
}

fn plan_moves(root: &Utf8Path, config: &Config, args: &ArchiveArgs) -> Result<Vec<ArchiveMove>> {
    let discovery = discover(root, config, &DiscoverOptions::default())?;
    let mut strategy_matches = Vec::new();
    let mut frontmatter_matches = Vec::new();
    let mut folder_matches = Vec::new();
    let mut filename_matches = Vec::new();

    for rel in discovery.active_docs {
        if rel.as_str().ends_with(".spec.md") {
            continue;
        }
        let abs = root.join(&rel);
        let content = fs::read_to_string(&abs)?;
        if frontmatter::change_id(&content)?.as_deref() == Some(args.change_id.as_str()) {
            frontmatter_matches.push(rel.clone());
        }
        if folder_match(&rel, config, &args.change_id) {
            folder_matches.push(rel.clone());
        }
        if filename_match(&rel, &args.change_id) {
            filename_matches.push(rel);
        }
    }

    if !frontmatter_matches.is_empty() {
        strategy_matches = frontmatter_matches;
    } else if !folder_matches.is_empty() {
        if !filename_matches.is_empty() {
            bail!(
                "ambiguous archive matches for `{}`; use --include to select exact docs",
                args.change_id
            );
        }
        strategy_matches = folder_matches;
    } else if !filename_matches.is_empty() {
        strategy_matches = filename_matches;
    }

    let mut selected: BTreeSet<Utf8PathBuf> = strategy_matches.into_iter().collect();
    for include in &args.include {
        let mut builder = GlobSetBuilder::new();
        builder.add(Glob::new(include)?);
        let set = builder.build()?;
        for rel in &discover(root, config, &DiscoverOptions::default())?.active_docs {
            if set.is_match(rel.as_str()) {
                selected.insert(rel.clone());
            }
        }
    }

    selected
        .into_iter()
        .map(|from| {
            if from.as_str().ends_with(".spec.md") {
                bail!("refusing to archive live spec {}", from);
            }
            Ok(ArchiveMove {
                to: archive_target(config, &args.change_id, &from),
                from,
            })
        })
        .collect()
}

fn folder_match(path: &Utf8Path, config: &Config, change_id: &str) -> bool {
    let parts: Vec<_> = path.as_str().split('/').collect();
    parts.len() > 2 && config.active_dirs.iter().any(|dir| dir == parts[0]) && parts[1] == change_id
}

fn filename_match(path: &Utf8Path, change_id: &str) -> bool {
    path.file_stem() == Some(change_id)
}

fn archive_target(config: &Config, change_id: &str, from: &Utf8Path) -> Utf8PathBuf {
    let parts: Vec<_> = from.as_str().split('/').collect();
    if parts.len() > 2
        && config.active_dirs.iter().any(|dir| dir == parts[0])
        && parts[1] == change_id
    {
        let rest = parts[2..].join("/");
        Utf8PathBuf::from(format!(
            "{}/{}/{}/{}",
            config.archive_dir, change_id, parts[0], rest
        ))
    } else {
        Utf8PathBuf::from(format!("{}/{}/{}", config.archive_dir, change_id, from))
    }
}

fn write_manifest(
    root: &Utf8Path,
    args: &ArchiveArgs,
    moves: &[ArchiveMove],
    warnings: &[String],
    check_status: &str,
) -> Result<()> {
    let path = root.join("archive").join(&args.change_id).join("README.md");
    fs::create_dir_all(path.parent().unwrap())?;
    let mut content = String::new();
    content.push_str("---\n");
    content.push_str(&format!("id: {}\n", args.change_id));
    content.push_str(&format!("archived_at: {}\n", crate::spec::now()));
    if let Some(source_ref) = &args.source_ref {
        content.push_str(&format!("source_ref: {}\n", source_ref));
    }
    if let Some(pr) = &args.pr {
        content.push_str(&format!("pr: {}\n", pr));
    }
    content.push_str(&format!("check_status: {}\n", check_status));
    content.push_str("archived_paths:\n");
    for item in moves {
        content.push_str(&format!("  - {}\n", item.from));
    }
    content.push_str("related_live_docs: []\n");
    if warnings.is_empty() {
        content.push_str("warnings: []\n");
    } else {
        content.push_str("warnings:\n");
        for warning in warnings {
            content.push_str(&format!("  - {:?}\n", warning));
        }
    }
    content.push_str("---\n\n");
    content.push_str(&format!("# {}\n\n", title_from_id(&args.change_id)));
    content.push_str("Historical change artifacts for this completed change.\n");
    fs::write(path, content)?;
    Ok(())
}

fn title_from_id(id: &str) -> String {
    id.split(['-', '_'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
