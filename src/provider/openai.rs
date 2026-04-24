use super::{ChumSwimProvider, DirectorySpecInput, FileSpecInput, RepairSpecInput, SpecDraft};
use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use serde_json::json;

pub struct OpenAiProvider {
    api_key: String,
    model: String,
    client: Client,
}

impl OpenAiProvider {
    pub fn from_environment() -> Result<Self> {
        let api_key = std::env::var("CODEX_OPENAI_API_KEY")
            .or_else(|_| std::env::var("OPENAI_API_KEY"))
            .context(
                "OpenAI auth not found. Sign in with Codex if supported, or set OPENAI_API_KEY.",
            )?;
        let model = std::env::var("CHUM_OPENAI_MODEL").unwrap_or_else(|_| "gpt-4.1-mini".into());
        Ok(Self {
            api_key,
            model,
            client: Client::new(),
        })
    }

    fn request_markdown(&self, instruction: &str, input: String) -> Result<String> {
        let body = json!({
            "model": self.model,
            "input": [
                {
                    "role": "system",
                    "content": "You write concise current-state Markdown specs. Return only Markdown, including a valid chum:backmatter block supplied or requested by the user. Do not include code fences around the whole answer."
                },
                {
                    "role": "user",
                    "content": format!("{instruction}\n\n{input}")
                }
            ]
        });
        let response: serde_json::Value = self
            .client
            .post("https://api.openai.com/v1/responses")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .context("failed to call OpenAI responses API")?
            .error_for_status()
            .context("OpenAI responses API returned an error")?
            .json()
            .context("failed to parse OpenAI response")?;

        if let Some(text) = response.get("output_text").and_then(|value| value.as_str()) {
            return Ok(text.to_string());
        }
        if let Some(output) = response.get("output").and_then(|value| value.as_array()) {
            let mut text = String::new();
            for item in output {
                if let Some(content) = item.get("content").and_then(|value| value.as_array()) {
                    for part in content {
                        if let Some(value) = part.get("text").and_then(|value| value.as_str()) {
                            text.push_str(value);
                        }
                    }
                }
            }
            if !text.trim().is_empty() {
                return Ok(text);
            }
        }
        bail!("OpenAI response did not contain Markdown text")
    }
}

impl ChumSwimProvider for OpenAiProvider {
    fn generate_file_spec(&self, input: FileSpecInput) -> Result<SpecDraft> {
        let markdown = self.request_markdown(
            "Generate a file-level current-state spec for this source file.",
            format!("Target: {}\n\nSource:\n{}", input.target, input.source),
        )?;
        Ok(SpecDraft { markdown })
    }

    fn generate_directory_spec(&self, input: DirectorySpecInput) -> Result<SpecDraft> {
        let mut context = format!("Target directory: {}\n\nChild specs:\n", input.target);
        for (path, spec) in input.child_specs {
            context.push_str(&format!("\n--- {path} ---\n{spec}\n"));
        }
        let markdown = self.request_markdown(
            "Generate a directory-level current-state spec from these child specs.",
            context,
        )?;
        Ok(SpecDraft { markdown })
    }

    fn repair_spec(&self, input: RepairSpecInput) -> Result<SpecDraft> {
        let mut context = format!(
            "Target: {}\n\nCurrent spec:\n{}\n",
            input.target, input.current_spec
        );
        for (path, text) in input.context {
            context.push_str(&format!("\n--- {path} ---\n{text}\n"));
        }
        let markdown = self.request_markdown(
            "Repair this spec by resolving TODO, unknown, and verify items using only the provided local context.",
            context,
        )?;
        Ok(SpecDraft { markdown })
    }
}
