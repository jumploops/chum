use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, ops::Range, path::Path};

const OPEN: &str = "<!-- chum:backmatter";
const CLOSE: &str = "-->";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SpecKind {
    File,
    Directory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Backmatter {
    pub schema: u32,
    pub kind: SpecKind,
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec_updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_by: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<String>,
    #[serde(default)]
    pub todo: Vec<String>,
    #[serde(default)]
    pub unknowns: Vec<String>,
    #[serde(default)]
    pub verify: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ParsedBackmatter {
    pub backmatter: Backmatter,
    pub range: Range<usize>,
}

pub fn parse_file(path: &Path) -> Result<ParsedBackmatter> {
    let content =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    parse(&content).with_context(|| format!("failed to parse backmatter in {}", path.display()))
}

pub fn parse(content: &str) -> Result<ParsedBackmatter> {
    let start = content
        .find(OPEN)
        .ok_or_else(|| anyhow::anyhow!("missing chum:backmatter block"))?;
    let yaml_start = start + OPEN.len();
    let relative_end = content[yaml_start..]
        .find(CLOSE)
        .ok_or_else(|| anyhow::anyhow!("unterminated chum:backmatter block"))?;
    let end = yaml_start + relative_end + CLOSE.len();
    if content[end..].contains(OPEN) {
        bail!("multiple chum:backmatter blocks found");
    }
    let yaml = content[yaml_start..yaml_start + relative_end].trim();
    let backmatter: Backmatter = serde_yaml::from_str(yaml)?;
    Ok(ParsedBackmatter {
        backmatter,
        range: start..end,
    })
}

pub fn replace_or_append(content: &str, backmatter: &Backmatter) -> Result<String> {
    let block = render(backmatter)?;
    match parse(content) {
        Ok(parsed) => {
            let mut next = String::new();
            next.push_str(&content[..parsed.range.start]);
            let before = next.trim_end().to_string();
            next = before;
            next.push_str("\n\n");
            next.push_str(&block);
            next.push_str(content[parsed.range.end..].trim_start_matches('\n'));
            Ok(next)
        }
        Err(_) => {
            let mut next = content.trim_end().to_string();
            if !next.is_empty() {
                next.push_str("\n\n");
            }
            next.push_str(&block);
            Ok(next)
        }
    }
}

pub fn render(backmatter: &Backmatter) -> Result<String> {
    let yaml = serde_yaml::to_string(backmatter)?;
    Ok(format!("{OPEN}\n{}{}", yaml.trim_end(), "\n-->"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_backmatter() {
        let parsed = parse(
            r#"# Title

<!-- chum:backmatter
schema: 1
kind: file
target: src/a.rs
todo: []
unknowns: []
verify: []
-->
"#,
        )
        .unwrap();
        assert_eq!(parsed.backmatter.target, "src/a.rs");
        assert_eq!(parsed.backmatter.kind, SpecKind::File);
    }
}
