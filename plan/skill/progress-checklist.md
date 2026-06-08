# Progress Checklist: skill

## Phase 0: Skill Scaffold

- [x] Add root `SKILL.md`
- [x] Add `agents/openai.yaml`
- [x] Add `scripts/chum.py`
- [x] Add `references/spec-format.md`
- [x] Add `references/workflow.md`
- [x] Add `references/command-reference.md`
- [x] Add specs for new skill folders
- [x] Update docs to mark Rust/native packaging as transitional

## Phase 1: Python Core

- [x] Add PEP 723 metadata to `scripts/chum.py`
- [x] Implement config defaults
- [x] Implement `chum.config.yaml` loading
- [x] Implement root normalization
- [x] Implement `.gitignore` handling
- [x] Implement `.chumignore` handling
- [x] Implement source discovery
- [x] Implement default source exclusions
- [x] Implement inline spec path matching
- [x] Implement source hashing and timestamps
- [x] Implement frontmatter parsing
- [x] Implement chum backmatter parsing
- [x] Implement chum backmatter rendering
- [x] Implement unresolved marker detection
- [x] Add Python primitive tests

## Phase 2: Agent-Facing Command Surfaces

- [x] Implement `targets`
- [x] Implement `check`
- [x] Implement `normalize`
- [x] Implement `validate`
- [x] Add JSON output models
- [x] Add bounded output controls
- [x] Add command help text
- [x] Add command-level tests

## Phase 3: Skill Workflow

- [x] Finalize concise `SKILL.md`
- [x] Document spec format reference
- [x] Document adaptive workflow reference
- [x] Document command reference
- [x] Add route-planning examples
- [x] Add guidance for unresolved gaps
- [x] Add guardrails against per-file `codex exec`

## Phase 4: Init And Archive

- [x] Implement `init`
- [x] Implement `archive`
- [x] Implement change doc discovery
- [x] Implement archive manifest writing
- [x] Implement Markdown-only movement
- [x] Implement link warning behavior
- [x] Add archive/init tests

## Phase 5: Validation And Install

- [x] Add fixture coverage
- [x] Add Python unit tests
- [x] Add command integration tests
- [x] Compare Python behavior to frozen Rust behavior
- [x] Validate skill metadata
- [x] Smoke test skill folder install/copy
- [x] Validate workflow on a tiny fixture
- [x] Verify large-output controls

## Phase 6: Rust Cleanup

- [x] Remove Cargo metadata
- [x] Remove Rust source
- [x] Remove Rust integration tests after Python replacements exist
- [x] Remove npm wrapper/package files
- [x] Remove native packaging docs
- [x] Rewrite Rust-primary README/status docs
- [x] Update final live specs
- [x] Confirm Python-only validation passes

## Phase 7: Publishing Cleanup

- [x] Clarify the publishable skill surface
- [x] Update installed-skill command path guidance
- [x] Update `agents/openai.yaml` to the current interface schema
- [x] Add repo-local config for validating the core Python processor
- [x] Add live spec coverage for `scripts/chum.py`
- [x] Keep the app-server proposal as design context, not default workflow

## Phase 8: Obsolete Plan Cleanup

- [x] Remove prior standalone CLI/package plan docs
- [x] Remove prior Codex-auth plan docs
- [x] Remove superseded package/auth design docs
- [x] Remove initial package scoping notes
- [x] Update remaining docs/specs so they do not link to removed files
