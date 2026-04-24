# Progress Checklist: Codex Auth

## Phase 1: Auth Resolver

- [ ] Add `swim.openai` config defaults
- [ ] Add partial config merge for nested OpenAI settings
- [ ] Add `OpenAiAuthMode`
- [ ] Add env override parsing
- [ ] Add Codex binary lookup abstraction
- [ ] Add Codex login status probe abstraction
- [ ] Add direct API-key presence detection
- [ ] Add `OpenAiAuthResolution`
- [ ] Add missing-auth diagnostics
- [ ] Add resolver unit tests

## Phase 2: Direct API Provider Cleanup

- [ ] Rename/split direct API-key provider
- [ ] Add `CHUM_OPENAI_API_KEY` precedence
- [ ] Preserve `CODEX_OPENAI_API_KEY` compatibility
- [ ] Preserve `OPENAI_API_KEY` fallback
- [ ] Extract shared provider prompts
- [ ] Remove secret values from direct provider errors
- [ ] Add direct provider env precedence tests

## Phase 3: Codex Exec Provider

- [ ] Add `src/provider/codex.rs`
- [ ] Add command runner abstraction
- [ ] Write structured output schema to temp file
- [ ] Invoke `codex exec` with stdin prompt
- [ ] Pass safe Codex exec flags
- [ ] Parse JSON result file
- [ ] Add strict ChatGPT env cleanup
- [ ] Redact command failure output
- [ ] Add fake command runner tests

## Phase 4: Swim Integration And Auth Status

- [ ] Route OpenAI provider through auth resolver
- [ ] Keep `--stubs` offline
- [ ] Add `chum swim --auth-status`
- [ ] Add auth status JSON output
- [ ] Add fake Codex CLI tests
- [ ] Add direct API fallback integration test
- [ ] Add forced Codex mode test
- [ ] Add forced API-key mode test

## Phase 5: Docs And Validation

- [ ] Update README auth docs
- [ ] Update default config docs
- [ ] Update live specs for touched source files
- [ ] Run formatter
- [ ] Run clippy
- [ ] Run tests
- [ ] Run `chum check`
- [ ] Run auth status smoke tests
