use pathdiff::diff_paths;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct LinkRewrite {
    pub content: String,
    pub warnings: Vec<String>,
}

pub fn rewrite_markdown_links(
    content: &str,
    old_path: &Path,
    new_path: &Path,
    move_map: &HashMap<PathBuf, PathBuf>,
    root: &Path,
) -> LinkRewrite {
    let mut out = String::with_capacity(content.len());
    let mut warnings = Vec::new();
    let mut index = 0;
    while let Some(open_rel) = content[index..].find("](") {
        let open = index + open_rel;
        let target_start = open + 2;
        let Some(close_rel) = content[target_start..].find(')') else {
            break;
        };
        let close = target_start + close_rel;
        let target = &content[target_start..close];
        out.push_str(&content[index..target_start]);
        if should_skip(target) {
            out.push_str(target);
        } else {
            let (path_part, suffix) = split_suffix(target);
            let old_target = normalize(old_path.parent().unwrap_or(root).join(path_part));
            let new_target = move_map
                .get(&old_target)
                .cloned()
                .unwrap_or_else(|| old_target.clone());
            if !new_target.exists() && !move_map.contains_key(&old_target) {
                warnings.push(format!(
                    "unresolved local link `{}` in {}",
                    target,
                    old_path.display()
                ));
                out.push_str(target);
            } else if is_probable_asset(&new_target) && !move_map.contains_key(&old_target) {
                warnings.push(format!(
                    "linked local asset not archived: {}",
                    old_target.display()
                ));
                out.push_str(&relative_target(new_path, &new_target, suffix));
            } else {
                out.push_str(&relative_target(new_path, &new_target, suffix));
            }
        }
        out.push(')');
        index = close + 1;
    }
    out.push_str(&content[index..]);
    LinkRewrite {
        content: out,
        warnings,
    }
}

fn should_skip(target: &str) -> bool {
    target.is_empty()
        || target.starts_with('#')
        || target.starts_with("http://")
        || target.starts_with("https://")
        || target.starts_with("mailto:")
}

fn split_suffix(target: &str) -> (&str, &str) {
    if let Some(index) = target.find('#') {
        (&target[..index], &target[index..])
    } else {
        (target, "")
    }
}

fn relative_target(from_file: &Path, to: &Path, suffix: &str) -> String {
    let from_dir = from_file.parent().unwrap_or_else(|| Path::new("."));
    let rel = diff_paths(to, from_dir).unwrap_or_else(|| to.to_path_buf());
    format!("{}{}", rel.to_string_lossy(), suffix)
}

fn normalize(path: PathBuf) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

fn is_probable_asset(path: &Path) -> bool {
    !path
        .extension()
        .and_then(|value| value.to_str())
        .map(|ext| matches!(ext, "md" | "markdown"))
        .unwrap_or(false)
}
