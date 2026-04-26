# Validation Checklist: Codex Auth

## Local Rust Validation

- [x] `cargo fmt --check`
- [x] `cargo clippy --all-targets --all-features -- -D warnings`
- [x] `cargo test`
- [x] `cargo run -- check --json`
- [x] `cargo run -- swim --stubs --dry-run`
- [x] `cargo run -- swim --auth-status`
- [x] `cargo run -- swim --auth-status --json`

## Auth Resolver Validation

- [x] Default config resolves requested mode as `auto`
- [x] YAML `swim.openai.auth: codex` is parsed
- [x] YAML `swim.openai.auth: apiKey` is parsed
- [x] Env `CHUM_OPENAI_AUTH=api-key` is parsed
- [x] Invalid auth mode fails clearly
- [x] Auto mode selects Codex with successful login status
- [x] Auto mode selects Codex with `CODEX_API_KEY`
- [x] Auto mode selects direct API with no Codex and direct key present
- [x] Forced Codex mode does not fall back
- [x] Forced API-key mode does not probe Codex for generation
- [x] Missing auth report contains setup guidance
- [x] Missing auth report contains no secret values

## Codex Exec Provider Validation

- [x] Command uses stdin for prompt text
- [x] Command does not put source text in argv
- [x] Command includes `--ephemeral`
- [x] Command includes `--skip-git-repo-check`
- [x] Command includes `--sandbox read-only`
- [x] Command includes `--ask-for-approval never`
- [x] Command includes `--output-schema`
- [x] Command includes `--output-last-message`
- [x] Command includes `-C <repo-root>`
- [x] Configured model is passed as `--model`
- [x] Valid JSON result returns Markdown
- [x] Invalid JSON result fails clearly
- [x] Missing `markdown` field fails clearly
- [x] Command stderr is redacted on failure
- [x] Strict ChatGPT mode removes direct API-key env vars

## Direct API Provider Validation

- [x] `CHUM_OPENAI_API_KEY` wins over `CODEX_OPENAI_API_KEY`
- [x] `CODEX_OPENAI_API_KEY` wins over `OPENAI_API_KEY`
- [x] `OPENAI_API_KEY` still works
- [x] Missing direct API key error mentions direct mode
- [x] API request failures do not include auth headers
- [x] Response parsing still handles `output_text`
- [x] Response parsing still handles nested `output[].content[].text`

## CLI Validation

- [x] `chum swim --stubs --dry-run` does not require auth
- [x] `chum swim --auth-status` exits zero with missing auth
- [x] `chum swim --auth-status --json` emits valid JSON
- [x] Fake logged-in Codex is selected in auto mode
- [x] Fake Codex with `CODEX_API_KEY` is selected in auto mode
- [x] Direct API fallback is selected when fake Codex is absent
- [x] Forced Codex mode reports missing Codex when absent
- [x] Forced API-key mode skips fake Codex
- [x] Codex exec failure is reported without prompt text
- [x] Non-Git fixture works through Codex exec command construction

## Documentation Validation

- [x] README documents Codex login
- [x] README documents device auth
- [x] README documents `CODEX_API_KEY`
- [x] README documents direct API fallback
- [x] README documents `chum swim --auth-status`
- [x] Docs do not instruct users to read `auth.json`
- [x] Live specs are current for every touched source file
