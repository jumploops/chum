# `src/provider/openai.rs`

## Purpose

Contains shared OpenAI-provider prompt construction. It keeps the instructions
and context formatting consistent between the direct Responses API provider and
the Codex exec provider.

## Key Exports

- `SYSTEM_INSTRUCTION` is the common system prompt for current-state spec
  generation.
- `file_prompt` builds a file-level generation prompt from source text and an
  optional existing spec.
- `directory_prompt` builds a directory-level prompt from child specs.
- `repair_prompt` builds a repair prompt from the current spec and local
  context.
- `codex_prompt` wraps a user prompt with Codex-specific structured-output
  guidance.

## Dependencies / Contracts

- This module has no network, filesystem, or process side effects.
- Prompt builders format local context only; providers decide how to send it.
- The prompts ask for current-state docs, not design proposals or source-code
  edits.

<!-- chum:backmatter
schema: 1
kind: file
target: src/provider/openai.rs
source_hash: sha256:3db7928814a5f1039b55d7ab2845ad59355f644a97262db804c777c2a658c9b2
source_updated_at: 2026-04-24T02:47:50.137353169Z
spec_updated_at: 2026-04-24T02:54:29.284657Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
