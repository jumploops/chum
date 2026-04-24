use crate::{cli::InitArgs, config::Config, output};
use anyhow::{Context, Result};
use serde::Serialize;
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize)]
struct InitReport {
    root: PathBuf,
    dry_run: bool,
    planned: Vec<String>,
    written: Vec<String>,
}

pub fn run(args: InitArgs) -> Result<()> {
    let root = std::env::current_dir()?;
    let dry_run = args.dry_run;
    let write = !dry_run && (args.write || !args.dry_run);
    let mut planned = Vec::new();
    let mut written = Vec::new();

    for dir in ["design", "plan", "debug", "review", "archive"] {
        let path = root.join(dir);
        if !path.exists() {
            planned.push(format!("create directory {}", path.display()));
            if write {
                fs::create_dir_all(&path)
                    .with_context(|| format!("failed to create {}", path.display()))?;
                written.push(path.display().to_string());
            }
        }
    }

    let archive_readme = root.join("archive").join("README.md");
    if !archive_readme.exists() {
        planned.push(format!("create {}", archive_readme.display()));
        if write {
            fs::create_dir_all(archive_readme.parent().unwrap())?;
            fs::write(
                &archive_readme,
                "# Archive\n\nHistorical change artifacts live here. Treat live `*.spec.md` files as current truth.\n",
            )?;
            written.push(archive_readme.display().to_string());
        }
    }

    let config_path = root.join("chum.config.yaml");
    if !config_path.exists() {
        planned.push(format!("create {}", config_path.display()));
        if write {
            fs::write(&config_path, Config::default_yaml()?)?;
            written.push(config_path.display().to_string());
        }
    }

    if let Some(agent_doc) = args.agent_doc.filter(|_| !args.no_agent_doc) {
        planned.push(format!("update agent doc {}", agent_doc.display()));
        if write {
            append_agent_snippet(&agent_doc)?;
            written.push(agent_doc.display().to_string());
        }
    }

    let report = InitReport {
        root,
        dry_run,
        planned,
        written,
    };
    if args.json {
        output::print_json(&report)?;
    } else if dry_run {
        println!("chum init dry run");
        for item in &report.planned {
            println!("- {item}");
        }
    } else {
        println!("chum init wrote {} item(s)", report.written.len());
        for item in &report.written {
            println!("- {item}");
        }
    }
    Ok(())
}

fn append_agent_snippet(path: &PathBuf) -> Result<()> {
    let snippet = "\n\n## chum Documentation Workflow\n\n- Live `*.spec.md` files are current truth.\n- Active `design/`, `plan/`, `debug/`, and `review/` docs capture intent.\n- `archive/**` is historical context only.\n";
    let mut content = if path.exists() {
        fs::read_to_string(path)?
    } else {
        String::new()
    };
    if !content.contains("## chum Documentation Workflow") {
        content.push_str(snippet);
        fs::write(path, content)?;
    }
    Ok(())
}
