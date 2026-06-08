# Phase 4: AI Swim

## Goal

Implement OpenAI-backed `chum swim` so specs can be generated and repaired from local source context until TODO, unknown, and verify lists are empty.

## Scope

In scope:

- provider trait and provider registry
- OpenAI provider
- Codex login auth discovery spike
- `OPENAI_API_KEY` fallback
- file spec generation
- directory spec generation from child specs
- repair passes
- max pass enforcement
- external verification allowance
- failure summaries with unresolved gaps

Out of scope:

- non-OpenAI providers
- human review workflow
- source-code edits
- retrieval from external systems
- mirror spec placement

## Implementation Notes

### Provider Boundary

The traversal engine owns filesystem state. The provider only receives bounded inputs and returns a `SpecDraft`.

```rust
#[async_trait]
pub trait ChumSwimProvider {
    async fn generate_file_spec(&self, input: FileSpecInput) -> anyhow::Result<SpecDraft>;
    async fn generate_directory_spec(&self, input: DirectorySpecInput) -> anyhow::Result<SpecDraft>;
    async fn repair_spec(&self, input: RepairSpecInput) -> anyhow::Result<SpecDraft>;
}
```

Inputs should include:

- target path
- source text or child spec text
- nearby specs when useful
- config values
- current backmatter, if repairing

Outputs should include:

- complete Markdown body
- parsed backmatter fields
- provider metadata
- unresolved TODO, unknown, and verify items

### Auth

Auth order:

1. Codex exec through a logged-in Codex CLI or `CODEX_API_KEY`.
2. Direct API-key mode through `CHUM_OPENAI_API_KEY`, `CODEX_OPENAI_API_KEY`, or `OPENAI_API_KEY`.
3. clear error explaining how to authenticate.

Keep auth discovery isolated so it can change without affecting traversal.

### Generation Strategy

File generation:

- read the source file
- include relevant sibling or parent specs when they exist
- ask the provider for current-state documentation, not a design proposal
- validate returned Markdown and backmatter

Directory generation:

- synthesize from child file specs and child directory specs
- list child responsibilities
- summarize dependencies and contracts
- avoid re-reading every source file if child specs are current

Repair passes:

- collect unresolved TODO, unknown, and verify items
- gather local files and child specs referenced by those items
- ask the provider to repair only the incomplete parts
- stop when all lists are empty or `maxPasses` is reached

### Completion

Default success requires:

- every included source file has a matching spec
- every included source directory has a matching spec
- `todo: []`
- `unknowns: []`
- `verify: []`

With `--allow-external-verify`, verify items may remain only when they explicitly say the required evidence is outside the repository.

## Acceptance Criteria

- [x] Provider trait has unit tests with a fake provider.
- [x] OpenAI provider can be selected with `--provider openai`.
- [x] Missing auth produces an actionable error.
- [ ] `OPENAI_API_KEY` fallback is covered by an integration-style test with mocked HTTP.
- [x] File spec generation validates output before writing.
- [x] Directory spec generation uses child specs as primary context.
- [x] Repair passes can clear TODO, unknown, and verify items with a fake provider.
- [x] `maxPasses` stops non-converging runs.
- [ ] `--allow-external-verify` allows only external verify items.
- [x] Final JSON output includes unresolved gaps when swim fails.

## Dependencies

- Phase 3 swim stubs.
