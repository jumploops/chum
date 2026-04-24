use crate::{
    config::Config,
    docs::backmatter::{Backmatter, SpecKind},
};
use anyhow::{Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub fn file_spec_path(root: &Utf8Path, rel_source: &Utf8Path) -> Utf8PathBuf {
    root.join(format!("{}.spec.md", rel_source.as_str()))
}

pub fn dir_spec_path(root: &Utf8Path, rel_dir: &Utf8Path, config: &Config) -> Utf8PathBuf {
    if rel_dir.as_str().is_empty() || rel_dir.as_str() == "." {
        return root.join(&config.specs.root_spec);
    }
    let basename = rel_dir
        .file_name()
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| "repo".to_string());
    root.join(rel_dir).join(format!("{basename}.spec.md"))
}

pub fn path_to_slash(path: &Utf8Path) -> String {
    if path.as_str().is_empty() {
        ".".into()
    } else {
        path.as_str().replace('\\', "/")
    }
}

pub fn sha256_file(path: &Path) -> Result<String> {
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(format!("sha256:{}", hex::encode(hasher.finalize())))
}

pub fn modified_time(path: &Path) -> Option<String> {
    let modified = fs::metadata(path).ok()?.modified().ok()?;
    let datetime: OffsetDateTime = modified.into();
    datetime.format(&Rfc3339).ok()
}

pub fn now() -> String {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".into())
}

pub fn file_backmatter(
    rel_source: &Utf8Path,
    source_path: &Path,
    generated_by: &str,
) -> Result<Backmatter> {
    Ok(Backmatter {
        schema: 1,
        kind: SpecKind::File,
        target: path_to_slash(rel_source),
        source_hash: Some(sha256_file(source_path)?),
        source_updated_at: modified_time(source_path),
        spec_updated_at: Some(now()),
        generated_by: Some(generated_by.into()),
        children: Vec::new(),
        todo: Vec::new(),
        unknowns: Vec::new(),
        verify: Vec::new(),
    })
}

pub fn directory_backmatter(
    rel_dir: &Utf8Path,
    children: Vec<String>,
    generated_by: &str,
) -> Backmatter {
    Backmatter {
        schema: 1,
        kind: SpecKind::Directory,
        target: path_to_slash(rel_dir),
        source_hash: None,
        source_updated_at: None,
        spec_updated_at: Some(now()),
        generated_by: Some(generated_by.into()),
        children,
        todo: Vec::new(),
        unknowns: Vec::new(),
        verify: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_inline_file_spec_path() {
        let root = Utf8Path::new("/repo");
        let rel = Utf8Path::new("src/foo.ts");
        assert_eq!(
            file_spec_path(root, rel).as_str(),
            "/repo/src/foo.ts.spec.md"
        );
    }

    #[test]
    fn builds_directory_spec_path() {
        let root = Utf8Path::new("/repo");
        let rel = Utf8Path::new("src/auth");
        assert_eq!(
            dir_spec_path(root, rel, &Config::default()).as_str(),
            "/repo/src/auth/auth.spec.md"
        );
    }
}
