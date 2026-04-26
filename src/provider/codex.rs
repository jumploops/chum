use super::{
    openai, openai_auth::CodexExecConfig, ChumSwimProvider, DirectorySpecInput, FileSpecInput,
    RepairSpecInput, SpecDraft,
};
use anyhow::{anyhow, bail, Context, Result};
use camino::Utf8PathBuf;
use serde_json::json;
use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

pub struct CodexExecProvider {
    root: Utf8PathBuf,
    config: CodexExecConfig,
    runner: Box<dyn CommandRunner>,
}

impl CodexExecProvider {
    pub fn new(root: Utf8PathBuf, config: CodexExecConfig) -> Self {
        Self {
            root,
            config,
            runner: Box::new(SystemCommandRunner),
        }
    }

    #[cfg(test)]
    fn with_runner(
        root: Utf8PathBuf,
        config: CodexExecConfig,
        runner: Box<dyn CommandRunner>,
    ) -> Self {
        Self {
            root,
            config,
            runner,
        }
    }

    fn request_markdown(&self, user_prompt: String) -> Result<String> {
        let temp = tempfile::tempdir().context("failed to create Codex temp directory")?;
        let schema_path = temp.path().join("schema.json");
        let result_path = temp.path().join("result.json");
        fs::write(&schema_path, output_schema())
            .with_context(|| format!("failed to write {}", schema_path.display()))?;

        let mut args = vec![
            "exec".to_string(),
            "--ephemeral".to_string(),
            "--skip-git-repo-check".to_string(),
            "--sandbox".to_string(),
            "read-only".to_string(),
            "--ask-for-approval".to_string(),
            "never".to_string(),
        ];
        if let Some(model) = &self.config.model {
            args.push("--model".into());
            args.push(model.clone());
        }
        if let Some(reasoning_effort) = &self.config.reasoning_effort {
            args.push("-c".into());
            args.push(format!("model_reasoning_effort=\"{reasoning_effort}\""));
        }
        args.extend([
            "--output-schema".to_string(),
            schema_path.display().to_string(),
            "--output-last-message".to_string(),
            result_path.display().to_string(),
            "-C".to_string(),
            self.root.to_string(),
            "-".to_string(),
        ]);

        let env_remove = if self.config.strict_chatgpt {
            vec![
                "OPENAI_API_KEY".into(),
                "CODEX_OPENAI_API_KEY".into(),
                "CHUM_OPENAI_API_KEY".into(),
            ]
        } else {
            Vec::new()
        };

        let output = self.runner.run(CommandSpec {
            binary: self.config.binary.clone(),
            args,
            cwd: self.root.clone(),
            stdin: openai::codex_prompt(&user_prompt),
            env_remove,
        })?;

        if !output.success {
            bail!(
                "codex exec failed with exit code {}: {}",
                output
                    .exit_code
                    .map(|code| code.to_string())
                    .unwrap_or_else(|| "unknown".into()),
                stderr_tail(&output.stderr)
            );
        }

        let raw = fs::read_to_string(&result_path)
            .with_context(|| format!("failed to read {}", result_path.display()))?;
        parse_result_markdown(&raw)
    }
}

impl ChumSwimProvider for CodexExecProvider {
    fn generate_file_spec(&self, input: FileSpecInput) -> Result<SpecDraft> {
        let markdown = self.request_markdown(openai::file_prompt(&input))?;
        Ok(SpecDraft { markdown })
    }

    fn generate_directory_spec(&self, input: DirectorySpecInput) -> Result<SpecDraft> {
        let markdown = self.request_markdown(openai::directory_prompt(&input))?;
        Ok(SpecDraft { markdown })
    }

    fn repair_spec(&self, input: RepairSpecInput) -> Result<SpecDraft> {
        let markdown = self.request_markdown(openai::repair_prompt(&input))?;
        Ok(SpecDraft { markdown })
    }
}

pub trait CommandRunner {
    fn run(&self, spec: CommandSpec) -> Result<CommandOutput>;
}

#[derive(Debug, Clone)]
pub struct CommandSpec {
    pub binary: PathBuf,
    pub args: Vec<String>,
    pub cwd: Utf8PathBuf,
    pub stdin: String,
    pub env_remove: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stderr: String,
}

struct SystemCommandRunner;

impl CommandRunner for SystemCommandRunner {
    fn run(&self, spec: CommandSpec) -> Result<CommandOutput> {
        let mut command = Command::new(&spec.binary);
        command
            .args(&spec.args)
            .current_dir(spec.cwd.as_std_path())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        for name in spec.env_remove {
            command.env_remove(name);
        }
        let mut child = command
            .spawn()
            .with_context(|| format!("failed to spawn {}", spec.binary.display()))?;
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow!("failed to open stdin for {}", spec.binary.display()))?;
        stdin
            .write_all(spec.stdin.as_bytes())
            .with_context(|| format!("failed to write stdin for {}", spec.binary.display()))?;
        drop(stdin);

        let output = child
            .wait_with_output()
            .with_context(|| format!("failed to wait for {}", spec.binary.display()))?;
        Ok(CommandOutput {
            success: output.status.success(),
            exit_code: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}

fn output_schema() -> String {
    serde_json::to_string_pretty(&json!({
        "type": "object",
        "properties": {
            "markdown": { "type": "string" }
        },
        "required": ["markdown"],
        "additionalProperties": false
    }))
    .expect("static schema serializes")
}

fn parse_result_markdown(raw: &str) -> Result<String> {
    match serde_json::from_str::<serde_json::Value>(raw) {
        Ok(value) => {
            let markdown = value
                .get("markdown")
                .and_then(|value| value.as_str())
                .ok_or_else(|| anyhow!("structured Codex output did not include markdown"))?;
            if markdown.trim().is_empty() {
                bail!("structured Codex output included empty markdown");
            }
            Ok(markdown.to_string())
        }
        Err(error) if raw.contains("chum:backmatter") => Ok(raw.to_string()),
        Err(error) => Err(error).context("failed to parse structured Codex output"),
    }
}

fn stderr_tail(stderr: &str) -> String {
    let redacted = redact_secrets(stderr);
    let lines: Vec<&str> = redacted.lines().rev().take(12).collect();
    let mut tail = lines.into_iter().rev().collect::<Vec<_>>().join("\n");
    if tail.len() > 2000 {
        tail = tail[tail.len().saturating_sub(2000)..].to_string();
    }
    if tail.trim().is_empty() {
        "no stderr output".into()
    } else {
        tail
    }
}

fn redact_secrets(text: &str) -> String {
    text.lines().map(redact_line).collect::<Vec<_>>().join("\n")
}

fn redact_line(line: &str) -> String {
    let upper = line.to_ascii_uppercase();
    for marker in ["KEY=", "TOKEN=", "SECRET=", "PASSWORD="] {
        if let Some(index) = upper.find(marker) {
            let end = index + marker.len();
            return format!("{}{}<redacted>", &line[..index], &line[index..end]);
        }
    }
    if let Some(index) = upper.find("BEARER ") {
        let end = index + "BEARER ".len();
        return format!("{}{}<redacted>", &line[..index], &line[index..end]);
    }
    line.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::FileSpecInput;
    use std::{cell::RefCell, rc::Rc};
    use tempfile::tempdir;

    #[derive(Clone)]
    struct FakeRunner {
        calls: Rc<RefCell<Vec<CommandSpec>>>,
        success: bool,
        stderr: String,
        result: String,
    }

    impl CommandRunner for FakeRunner {
        fn run(&self, spec: CommandSpec) -> Result<CommandOutput> {
            let result_path = output_last_message_arg(&spec).unwrap();
            fs::write(result_path, &self.result).unwrap();
            self.calls.borrow_mut().push(spec);
            Ok(CommandOutput {
                success: self.success,
                exit_code: Some(if self.success { 0 } else { 1 }),
                stderr: self.stderr.clone(),
            })
        }
    }

    fn provider_with(fake: FakeRunner) -> CodexExecProvider {
        let root = Utf8PathBuf::from_path_buf(tempdir().unwrap().keep()).unwrap();
        CodexExecProvider::with_runner(
            root,
            CodexExecConfig {
                binary: PathBuf::from("/tmp/codex"),
                model: Some("gpt-test".into()),
                reasoning_effort: None,
                strict_chatgpt: true,
            },
            Box::new(fake),
        )
    }

    #[test]
    fn sends_prompt_on_stdin_and_safe_args() {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let fake = FakeRunner {
            calls: calls.clone(),
            success: true,
            stderr: String::new(),
            result: r##"{"markdown":"# Spec\n\n<!-- chum:backmatter\nschema: 1\nkind: file\ntarget: src/lib.rs\ntodo: []\nunknowns: []\nverify: []\n-->\n"}"##.into(),
        };

        let markdown = provider_with(fake)
            .generate_file_spec(FileSpecInput {
                target: "src/lib.rs".into(),
                source: "pub fn add() {}\n".into(),
                existing_spec: None,
            })
            .unwrap()
            .markdown;

        assert!(markdown.contains("# Spec"));
        let call = calls.borrow();
        let spec = &call[0];
        assert!(spec.stdin.contains("pub fn add"));
        assert!(!spec.args.iter().any(|arg| arg.contains("pub fn add")));
        assert!(spec
            .args
            .windows(2)
            .any(|pair| pair[0] == "--sandbox" && pair[1] == "read-only"));
        assert!(spec
            .args
            .windows(2)
            .any(|pair| pair[0] == "--ask-for-approval" && pair[1] == "never"));
        assert!(spec.args.iter().any(|arg| arg == "--ephemeral"));
        assert!(spec.args.iter().any(|arg| arg == "--skip-git-repo-check"));
        assert!(spec
            .args
            .windows(2)
            .any(|pair| pair[0] == "--model" && pair[1] == "gpt-test"));
        assert!(spec.env_remove.iter().any(|name| name == "OPENAI_API_KEY"));
    }

    #[test]
    fn rejects_missing_markdown_field() {
        let fake = FakeRunner {
            calls: Rc::new(RefCell::new(Vec::new())),
            success: true,
            stderr: String::new(),
            result: "{}".into(),
        };

        let error = provider_with(fake)
            .generate_file_spec(FileSpecInput {
                target: "src/lib.rs".into(),
                source: "pub fn add() {}\n".into(),
                existing_spec: None,
            })
            .unwrap_err()
            .to_string();

        assert!(error.contains("did not include markdown"));
    }

    #[test]
    fn redacts_stderr_on_failure() {
        let fake = FakeRunner {
            calls: Rc::new(RefCell::new(Vec::new())),
            success: false,
            stderr: "OPENAI_API_KEY=sk-secret\nAuthorization: Bearer sk-token".into(),
            result: "{}".into(),
        };

        let error = provider_with(fake)
            .generate_file_spec(FileSpecInput {
                target: "src/lib.rs".into(),
                source: "pub fn add() {}\n".into(),
                existing_spec: None,
            })
            .unwrap_err()
            .to_string();

        assert!(error.contains("OPENAI_API_KEY=<redacted>"));
        assert!(error.contains("Authorization: Bearer <redacted>"));
        assert!(!error.contains("sk-secret"));
        assert!(!error.contains("sk-token"));
        assert!(!error.contains("pub fn add"));
    }

    fn output_last_message_arg(spec: &CommandSpec) -> Option<PathBuf> {
        spec.args
            .windows(2)
            .find(|pair| pair[0] == "--output-last-message")
            .map(|pair| PathBuf::from(&pair[1]))
    }
}
