# `src/config.rs`

## Purpose

Defines the loaded configuration model, built-in defaults, YAML override merge
logic, OpenAI auth config, and root path normalization used by every command.
Defaults are complete enough for `chum` to run without a config file.

## Key Exports

- `Config` is the full runtime configuration.
- `SourceConfig`, `SpecConfig`, `MarkerConfig`, and `SwimConfig` group command
  behavior by concern.
- `OpenAiSwimConfig` stores nested `swim.openai` settings for auth mode, Codex
  binary, model, and reasoning effort.
- `OpenAiAuthMode` parses and serializes `auto`, `codex`, and `apiKey`.
- `Config::load` merges `chum.config.yaml` over defaults.
- `Config::default_yaml` serializes the built-in defaults for init/docs.
- `normalize_root` resolves a user path into a UTF-8 path.

## Dependencies / Contracts

- YAML field names use camelCase to match generated config files.
- Partial config structs merge nested values without requiring users to repeat
  the entire default config.
- OpenAI auth mode accepts `apiKey`, `api-key`, and `api_key` on input but
  serializes as `apiKey`.
- Source discovery defaults exclude tests, fixtures, scripts, generated files,
  migrations, config files, build outputs, and archive history.

<!-- chum:backmatter
schema: 1
kind: file
target: src/config.rs
source_hash: sha256:fc8c064b84850c856de69261ddc78126e6e18ef8c373825fb44a7f1d721bc28a
source_updated_at: 2026-04-24T02:57:18.406391181Z
spec_updated_at: 2026-04-24T02:57:35.36205Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
