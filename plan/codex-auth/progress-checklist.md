# Progress Checklist: Codex Auth

## Phase 1: Auth Resolver

- [x] Add `swim.openai` config defaults
- [x] Add partial config merge for nested OpenAI settings
- [x] Add `OpenAiAuthMode`
- [x] Add env override parsing
- [x] Add Codex binary lookup abstraction
- [x] Add Codex login status probe abstraction
- [x] Add direct API-key presence detection
- [x] Add `OpenAiAuthResolution`
- [x] Add missing-auth diagnostics
- [x] Add resolver unit tests

## Phase 2: Direct API Provider Cleanup

- [x] Rename/split direct API-key provider
- [x] Add `CHUM_OPENAI_API_KEY` precedence
- [x] Preserve `CODEX_OPENAI_API_KEY` compatibility
- [x] Preserve `OPENAI_API_KEY` fallback
- [x] Extract shared provider prompts
- [x] Remove secret values from direct provider errors
- [x] Add direct provider env precedence tests

## Phase 3: Codex Exec Provider

- [x] Add `src/provider/codex.rs`
- [x] Add command runner abstraction
- [x] Write structured output schema to temp file
- [x] Invoke `codex exec` with stdin prompt
- [x] Pass safe Codex exec flags
- [x] Parse JSON result file
- [x] Add strict ChatGPT env cleanup
- [x] Redact command failure output
- [x] Add fake command runner tests

## Phase 4: Swim Integration And Auth Status

- [x] Route OpenAI provider through auth resolver
- [x] Keep `--stubs` offline
- [x] Add `chum swim --auth-status`
- [x] Add auth status JSON output
- [x] Add fake Codex CLI tests
- [x] Add direct API fallback integration test
- [x] Add forced Codex mode test
- [x] Add forced API-key mode test

## Phase 5: Docs And Validation

- [x] Update README auth docs
- [x] Update default config docs
- [x] Update live specs for touched source files
- [x] Run formatter
- [x] Run clippy
- [x] Run tests
- [x] Run `chum check`
- [x] Run auth status smoke tests
