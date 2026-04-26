use super::{
    openai, openai_auth::OpenAiApiKeyConfig, ChumSwimProvider, DirectorySpecInput, FileSpecInput,
    RepairSpecInput, SpecDraft,
};
use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use serde_json::json;

pub struct OpenAiApiKeyProvider {
    api_key: String,
    model: String,
    client: Client,
}

impl OpenAiApiKeyProvider {
    pub fn new(config: OpenAiApiKeyConfig) -> Self {
        Self {
            api_key: config.api_key,
            model: config.model,
            client: Client::new(),
        }
    }

    fn request_markdown(&self, user_prompt: String) -> Result<String> {
        let body = json!({
            "model": self.model,
            "input": [
                {
                    "role": "system",
                    "content": openai::SYSTEM_INSTRUCTION
                },
                {
                    "role": "user",
                    "content": user_prompt
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

        extract_response_text(&response)
    }
}

impl ChumSwimProvider for OpenAiApiKeyProvider {
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

pub(crate) fn extract_response_text(response: &serde_json::Value) -> Result<String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extracts_output_text() {
        let response = json!({ "output_text": "# Spec" });
        assert_eq!(extract_response_text(&response).unwrap(), "# Spec");
    }

    #[test]
    fn extracts_nested_output_text() {
        let response = json!({
            "output": [
                { "content": [{ "text": "# " }, { "text": "Spec" }] }
            ]
        });
        assert_eq!(extract_response_text(&response).unwrap(), "# Spec");
    }

    #[test]
    fn rejects_missing_text() {
        let response = json!({ "output": [] });
        let error = extract_response_text(&response).unwrap_err().to_string();
        assert!(error.contains("did not contain Markdown text"));
    }
}
