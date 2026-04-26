# `src/provider/codex.rs`

## Purpose

Implements the Codex subprocess provider for OpenAI-backed `chum swim`. It
delegates generation to `codex exec` so Codex owns ChatGPT login, token refresh,
device auth, API-key automation, and credential storage.

## Key Exports

- `CodexExecProvider` implements `ChumSwimProvider` by running `codex exec`.
- `CommandRunner`, `CommandSpec`, and `CommandOutput` provide an injectable
  command boundary for unit tests.
- Internal helpers write the structured output schema, parse result JSON, and
  redact command failure output.

## Dependencies / Contracts

- The provider passes prompts on stdin and never puts source text in argv.
- Every Codex run uses `--ephemeral`, `--skip-git-repo-check`,
  `--sandbox read-only`, `--ask-for-approval never`, `--output-schema`, and
  `--output-last-message`.
- Strict ChatGPT mode removes direct API-key env vars from the child process.
- Result files must contain a JSON object with a non-empty `markdown` string,
  with a guarded plain-Markdown fallback only when chum backmatter is present.
- Errors include exit status and redacted stderr tails, not full prompts or
  secret values.
- The provider never reads `CODEX_HOME/auth.json` or OS keyrings.

<!-- chum:backmatter
schema: 1
kind: file
target: src/provider/codex.rs
source_hash: sha256:d9eb897dd1c46c1666d5d3fbf713dccb6d949055df89cc3e78b5a942968e26bf
source_updated_at: 2026-04-24T02:51:49.183319156Z
spec_updated_at: 2026-04-24T02:54:29.283775Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
