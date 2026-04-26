# `src/commands/swim.rs`

## Purpose

Implements `chum swim`, the command that creates or repairs live specs for the
discovered source tree. It supports deterministic stub generation, OpenAI-backed
provider generation, repair passes, unresolved-gap reporting, JSON output, and
secret-free auth status reporting.

## Key Exports

- `run` is the command entry point used by `src/main.rs`.
- Stub-generation helpers create file, directory, and root specs with chum
  backmatter.
- Provider-generation helpers call the selected `ChumSwimProvider`, normalize
  backmatter, and repair unresolved specs until convergence or `maxPasses`.
- Auth helpers resolve the OpenAI provider into either Codex exec or direct
  API-key mode and render `swim --auth-status`.

## Dependencies / Contracts

- Source discovery, spec-path matching, hashing, and backmatter writing are
  delegated to shared modules.
- `--stubs` never resolves auth and remains offline.
- `--auth-status` only supports provider `openai`, exits after auth resolution,
  and supports both human and JSON output.
- Provider mode prefers Codex auth through `CodexExecProvider` and falls back to
  `OpenAiApiKeyProvider` through the auth resolver.
- `swim` writes specs only after generated Markdown has chum backmatter
  normalized for the target file or directory.

<!-- chum:backmatter
schema: 1
kind: file
target: src/commands/swim.rs
source_hash: sha256:226570b877e717bad162958fb254d9677d90e77fa4f10f2f049423463e23b426
source_updated_at: 2026-04-24T02:51:49.179696089Z
spec_updated_at: 2026-04-24T02:54:29.279746Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
