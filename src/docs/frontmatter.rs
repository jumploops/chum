use anyhow::Result;
use serde_yaml::Value;

pub fn parse(content: &str) -> Result<Option<Value>> {
    if !content.starts_with("---\n") {
        return Ok(None);
    }
    let Some(end) = content[4..].find("\n---") else {
        return Ok(None);
    };
    let yaml = &content[4..4 + end];
    Ok(Some(serde_yaml::from_str(yaml)?))
}

pub fn change_id(content: &str) -> Result<Option<String>> {
    let Some(value) = parse(content)? else {
        return Ok(None);
    };
    Ok(value
        .get("change")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_change_id() {
        let content = "---\nchange: foo\n---\n# Foo\n";
        assert_eq!(change_id(content).unwrap(), Some("foo".into()));
    }
}
