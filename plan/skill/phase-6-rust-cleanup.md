# Phase 6: Rust Cleanup

## Goal

Remove the transitional Rust CLI, npm wrapper, and native packaging materials
after the Python skill reaches parity.

## Scope

In scope:

- delete Rust source and Cargo metadata
- delete npm wrapper/package files
- delete provider/Codex exec implementation
- delete or rewrite Rust-specific docs
- update specs to describe the final skill architecture
- update validation commands
- remove obsolete CI/release packaging steps

Out of scope:

- changing the skill command surface
- adding new workflow behavior
- preserving npm/pnpm/Homebrew packaging

## Removal Candidates

Remove after parity is proven:

```text
Cargo.toml
Cargo.lock
src/
tests/      # Rust integration tests only; preserve or replace Python tests
npm/
target/     # if present locally only; should not be tracked
```

Also remove or rewrite docs that only describe the old product:

- Rust crate packaging docs
- npm/pnpm wrapper docs
- Homebrew packaging notes
- Codex exec provider as primary swim path
- direct OpenAI provider docs

Do not remove docs that still explain useful workflow history unless they are
part of the installed skill artifact and add avoidable noise.

## Replacement Docs

Final docs should describe:

- `chum` as a skill
- `uv run scripts/chum.py` as the deterministic processor
- no LLM calls inside the script
- the adaptive agent-led workflow
- install/copy instructions for the skill
- Python test commands

Update or create live specs for:

- root skill package
- `scripts/`
- `references/`
- `agents/`
- `tests/`

## Cleanup Safety

Before deletion:

1. Run the full Python validation checklist.
2. Confirm no remaining plan/checklist item depends on Rust.
3. Confirm no documented user workflow invokes `cargo`, `npm`, `pnpm`, or
   `chum swim --provider openai`.
4. Confirm all current Rust behavior needed by the skill has a Python equivalent
   or is explicitly dropped.

## Acceptance Criteria

- [x] No Rust source or Cargo metadata remains in the final skill artifact.
- [x] No npm wrapper or native binary packaging remains in the final skill
  artifact.
- [x] README or equivalent repo docs describe the skill workflow.
- [x] `SKILL.md`, `scripts/`, `references/`, and `agents/` are the primary
  runtime artifacts.
- [x] Python tests pass after deletion.
- [x] `python3 scripts/chum.py check --root <fixture> --json` still works.
- [x] Specs and references contain no stale Rust-primary claims.

## Dependencies

- Phase 5 validation and install.
