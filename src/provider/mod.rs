pub mod codex;
pub mod openai;
pub mod openai_api;
pub mod openai_auth;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSpecInput {
    pub target: String,
    pub source: String,
    pub existing_spec: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectorySpecInput {
    pub target: String,
    pub child_specs: Vec<(String, String)>,
    pub existing_spec: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairSpecInput {
    pub target: String,
    pub current_spec: String,
    pub context: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecDraft {
    pub markdown: String,
}

pub trait ChumSwimProvider {
    fn generate_file_spec(&self, input: FileSpecInput) -> Result<SpecDraft>;
    fn generate_directory_spec(&self, input: DirectorySpecInput) -> Result<SpecDraft>;
    fn repair_spec(&self, input: RepairSpecInput) -> Result<SpecDraft>;
}
