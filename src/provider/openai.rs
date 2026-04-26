use super::{DirectorySpecInput, FileSpecInput, RepairSpecInput};

pub(crate) const SYSTEM_INSTRUCTION: &str = "You write concise current-state Markdown specs. Return only Markdown, including a valid chum:backmatter block supplied or requested by the user. Do not include code fences around the whole answer.";

pub(crate) fn file_prompt(input: &FileSpecInput) -> String {
    let mut prompt = format!(
        "Generate a file-level current-state spec for this source file.\n\nTarget: {}\n\nSource:\n{}",
        input.target, input.source
    );
    if let Some(existing) = &input.existing_spec {
        prompt.push_str(&format!("\n\nExisting spec:\n{existing}"));
    }
    prompt
}

pub(crate) fn directory_prompt(input: &DirectorySpecInput) -> String {
    let mut prompt = format!(
        "Generate a directory-level current-state spec from these child specs.\n\nTarget directory: {}\n\nChild specs:\n",
        input.target
    );
    for (path, spec) in &input.child_specs {
        prompt.push_str(&format!("\n--- {path} ---\n{spec}\n"));
    }
    if let Some(existing) = &input.existing_spec {
        prompt.push_str(&format!("\n\nExisting spec:\n{existing}"));
    }
    prompt
}

pub(crate) fn repair_prompt(input: &RepairSpecInput) -> String {
    let mut prompt = format!(
        "Repair this spec by resolving TODO, unknown, and verify items using only the provided local context.\n\nTarget: {}\n\nCurrent spec:\n{}\n",
        input.target, input.current_spec
    );
    for (path, text) in &input.context {
        prompt.push_str(&format!("\n--- {path} ---\n{text}\n"));
    }
    prompt
}

pub(crate) fn codex_prompt(user_prompt: &str) -> String {
    format!(
        "{SYSTEM_INSTRUCTION}\n\nReturn a JSON object matching the provided output schema. Put the complete Markdown spec in the `markdown` field.\n\n{user_prompt}"
    )
}
