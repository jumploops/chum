# `./`

## Purpose

Repository root for the `chum` skill. The repo contains the skill instructions,
deterministic Python processor, workflow/design docs, tests, and CI scaffolding
for the filesystem-first documentation workflow.

## Files

- `SKILL.md` - Skill entry point and concise agent workflow.
- `README.md` - Project overview, skill usage, and local development commands.
- `AGENTS.template.md` - Template for repositories that adopt the chum documentation workflow.
- `chum.config.yaml` - Repo-local validation config that treats the core
  Python processor as source while preserving default exclusions for tests and
  generated paths.
- `LICENSE` - MIT license text.

## Subfolders

- `scripts/` - Standard-library Python processor used by the skill.
- `references/` - Detailed skill references loaded only when needed.
- `agents/` - Skill UI metadata.
- `tests/` - Python tests for script behavior.
- `design/` - Intent/design documentation, including historical package/auth
  designs and the optional app-server proposal.
- `plan/` - Phased implementation plan and validation checklists.
- `reference/` - Initial scoping/reference notes.
- `.github/` - CI workflow definitions.

## Dependencies / Contracts

- The Python script is the deterministic source of truth for discovery,
  validation, normalization, init, and archive mechanics.
- The active agent session owns cross-repo analysis and spec authorship; the
  script must not call an LLM.
- The publishable skill surface is `SKILL.md`, `agents/openai.yaml`,
  `scripts/chum.py`, and `references/`; plan/design/test files support repo
  maintenance.
- `target/**`, `archive/**`, `.git/**`, and other generated or historical paths are excluded from source discovery by default.
- `repo.spec.md` is the root live spec and is validated by `chum check`.

<!-- chum:backmatter
schema: 1
kind: directory
target: .
spec_updated_at: 2026-04-24T01:35:55.621326Z
generated_by: chum skill
children:
- agents/agents.spec.md
- references/references.spec.md
- scripts/scripts.spec.md
- tests/tests.spec.md
todo: []
unknowns: []
verify: []
-->
