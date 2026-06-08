# Phase 3: Skill Workflow

## Goal

Write the actual skill instructions and reference docs that teach an agent how
to maintain live specs using the Python script and its own shared context.

## Scope

In scope:

- final `SKILL.md` workflow
- `references/spec-format.md`
- `references/workflow.md`
- `references/command-reference.md`
- examples of agent route planning
- guidance for incomplete knowledge and backmatter gaps
- guardrails against isolated per-file analysis

Out of scope:

- new script behavior
- Rust cleanup
- publishing automation

## `SKILL.md` Requirements

`SKILL.md` should stay short and operational.

It should include:

- when to use the skill
- prerequisite: `uv`
- first command to run: `uv run scripts/chum.py targets --root . --json`
- core loop:
  1. gather targets
  2. read related code/specs
  3. choose an adaptive route
  4. write/update specs
  5. normalize
  6. validate
  7. finish with check
- available script summary
- reference file summary

It should not include the full schema, long examples, or implementation history.

## `references/spec-format.md`

Document:

- live spec purpose
- inline placement rules
- file spec expectations
- directory spec expectations
- root spec expectations
- backmatter block structure
- `todo`, `unknowns`, and `verify` semantics
- when to leave unresolved gaps
- how source hashes and timestamps are maintained

## `references/workflow.md`

Document the adaptive workflow in detail:

- use `targets` for facts, not route decisions
- start with high-signal modules when that helps explain patterns
- jump across the codebase when imports, shared types, or naming conventions
  require it
- update leaf and parent directory specs when enough child context exists
- never fabricate certainty
- run focused validation after each batch
- run full `check` before final response

The reference should explicitly say that a long-running agent session replaces
the old per-file `codex exec` analysis path.

## `references/command-reference.md`

Document:

- all script commands
- flags
- JSON schemas
- exit codes
- examples
- common errors and recovery actions

This file can be longer than `SKILL.md` because the agent reads it only when
needed.

## Acceptance Criteria

- [x] Skill metadata clearly triggers for repo spec maintenance.
- [x] `SKILL.md` stays concise and points to references.
- [x] References explain the spec format and workflow without duplicating large
  blocks unnecessarily.
- [x] The workflow tells agents to use accumulated context across the repo.
- [x] The workflow tells agents not to call per-file `codex exec`.
- [x] The command reference matches implemented script flags.

## Dependencies

- Phase 2 command surfaces.
