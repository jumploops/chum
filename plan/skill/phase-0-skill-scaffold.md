# Phase 0: Skill Scaffold

## Goal

Create the skill package shape and document the product pivot before porting
behavior from Rust to Python.

## Scope

In scope:

- root `SKILL.md`
- `agents/openai.yaml`
- `scripts/` directory with a placeholder `chum.py`
- `references/` directory with placeholder reference docs
- clear runtime requirement: `uv run`
- docs updates that mark Rust/native packaging as transitional
- initial specs for new skill folders

Out of scope:

- full Python behavior
- archive movement
- Rust deletion
- publishing/install automation

## Implementation Notes

Create the target skill shape:

```text
SKILL.md
agents/
  openai.yaml
scripts/
  chum.py
references/
  spec-format.md
  workflow.md
  command-reference.md
```

`SKILL.md` should be concise. It should include:

- when the skill should be used
- the command pattern: `uv run scripts/chum.py ...`
- the adaptive documentation workflow
- a short list of available scripts and references

`scripts/chum.py` may initially implement only `--help` and a clear
not-yet-implemented error for subcommands. It should still include the intended
PEP 723 dependency block so later phases keep the same invocation shape.

`agents/openai.yaml` should be generated or written according to the local skill
authoring guidance. It should describe `chum` as a repository documentation
maintenance skill, not a generic CLI.

## Product Pivot Docs

Update high-level docs to make the pivot explicit:

- Rust implementation is frozen except for migration fixes.
- npm/pnpm/Homebrew/native binary packaging is no longer a v1 deliverable.
- `codex exec` provider work is superseded by the skill-led workflow for
  high-quality interactive use.
- The Python script does not call an LLM.

Do not delete the existing Rust code in this phase.

## Acceptance Criteria

- [x] `SKILL.md` exists with valid skill frontmatter.
- [x] `agents/openai.yaml` exists and matches the skill.
- [x] `uv run scripts/chum.py --help` succeeds, using
  `UV_CACHE_DIR=/tmp/chum-uv-cache` in this restricted sandbox.
- [x] `python3 scripts/chum.py --help` succeeds as the local fallback.
- [x] `references/` contains the planned reference stubs.
- [x] New skill folders have matching specs.
- [x] Existing docs identify Rust/native packaging as transitional.

## Dependencies

- None.
