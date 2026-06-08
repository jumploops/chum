# Implementation Spec: skill

## Context

- Workflow source: [`AGENTS.template.md`](../../AGENTS.template.md)
- Initial package notes: [`reference/init.md`](../../reference/init.md)
- Prior Rust CLI plan: [`plan/init/`](../init/)
- Prior Codex auth plan: [`plan/codex-auth/`](../codex-auth/)

The original `chum` implementation scoped a Rust CLI distributed through Cargo,
npm/pnpm wrappers, and eventually Homebrew. That shape is useful for a standalone
tool, but it is not the best primary interface for agent skills. A skill should
be easy for an agent to run without platform-specific binaries, chmod issues, or
an install step.

This plan pivots `chum` into an installable skill with Python scripts under
`scripts/`, run through `uv run`. The Python script owns deterministic repository
mechanics. The agent owns codebase understanding, routing, and spec authorship in
one long-running context.

## Objective

Turn this repository into an installable `chum` skill.

End-state:

- The skill root contains a valid `SKILL.md`.
- The skill includes `agents/openai.yaml` metadata.
- The deterministic processor lives at `scripts/chum.py`.
- Agents run the processor with:

  ```bash
  uv run scripts/chum.py --help
  ```

- The script never calls an LLM.
- The script provides stable machine-readable commands for targets, checking,
  normalization, validation, init, and archive behavior.
- The skill instructions tell the agent how to use persistent session context to
  update specs across related files, not one isolated file at a time.
- Rust, npm, and native-binary packaging remain in the repo only during
  migration. A final cleanup phase removes them after Python parity.

## Product Decision

`chum` should be a skill-first workflow, not a binary-first workflow.

The Rust `chum swim --provider openai` approach runs isolated model calls per
file or directory. That loses accumulated repo context. The skill approach keeps
one agent session in charge of the route:

1. Ask the script which specs are missing, stale, invalid, or incomplete.
2. Read related code and specs in whatever order best explains the system.
3. Write or update specs with the agent's accumulated codebase context.
4. Ask the script to normalize backmatter and validate the result.
5. Repeat until `check` is clean.

The script is the deterministic substrate. The agent is the analyst.

## Skill Layout

Target layout:

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
tests/
  test_chum_script.py
```

Notes:

- `SKILL.md` stays concise and procedural.
- Detailed schema, backmatter, and command behavior lives in `references/`.
- `scripts/chum.py` is self-contained and includes PEP 723 metadata.
- Tests are repository development assets. The skill runtime path must not
  require tests or project-level installation.

## Runtime Contract

### Script Invocation

Always run from the skill root:

```bash
uv run scripts/chum.py <command> [options]
```

The script must:

- be non-interactive
- expose concise `--help`
- accept input through flags, stdin, or files
- print structured data to stdout for `--json`
- print diagnostics and warnings to stderr
- support dry-run behavior for mutating commands
- use meaningful exit codes
- bound default output size and support pagination or limits where output can be
  large

### Python Dependencies

Use PEP 723 inline metadata in `scripts/chum.py`. The implementation is
standard-library only so the skill can run in restricted environments without a
dependency download:

```python
# /// script
# requires-python = ">=3.9"
# dependencies = []
# ///
```

Avoid package installation, compiled extensions, and raw executable files.

### Exit Codes

Define stable exit codes:

| Code | Meaning |
|------|---------|
| `0` | command succeeded |
| `1` | validation completed and found repo/spec failures |
| `2` | invalid arguments or invalid config |
| `3` | filesystem, parse, or normalization error |
| `4` | unsupported environment or missing runtime prerequisite |

## Script Commands

### `targets`

List specs that need agent attention.

```bash
uv run scripts/chum.py targets --root . --json
```

Output includes:

- repo root
- source discovery summary
- target records for missing, stale, invalid, or incomplete specs
- source path
- expected spec path
- kind: `file`, `directory`, or `root`
- reasons: `missing`, `stale`, `invalid_backmatter`, `todo`, `unknowns`,
  `verify`, `legacy_marker`
- hashes and timestamps when available
- child spec references for directory targets
- unresolved counts

This is the primary command a skill agent uses to plan its route.

### `check`

Validate the whole repository.

```bash
uv run scripts/chum.py check --root . --json
```

Exit `0` only when the repo is clean. Exit `1` when validation completes and
finds actionable failures.

### `normalize`

Normalize an agent-written spec and inject or update chum backmatter.

```bash
uv run scripts/chum.py normalize --root . --target src/foo.py --stdin --write
```

Behavior:

- reads Markdown from stdin or `--input`
- identifies whether target is a source file, directory, or root spec
- computes source hash and timestamps where applicable
- preserves agent-authored body text
- replaces or appends exactly one `chum:backmatter` block
- preserves unresolved `todo`, `unknowns`, and `verify` lists from supplied
  backmatter when present
- writes only with `--write`; otherwise prints normalized Markdown to stdout

### `validate`

Validate one target/spec pair.

```bash
uv run scripts/chum.py validate --root . --target src/foo.py --json
```

This lets the agent validate after each focused batch instead of waiting for a
full repo check.

### `init`

Create or update documentation workflow scaffolding.

```bash
uv run scripts/chum.py init --root . --write
```

Behavior should be idempotent and dry-run by default unless `--write` is passed.

### `archive`

Move completed change docs into archive history.

```bash
uv run scripts/chum.py archive --root . auth-session-hardening --write --json
```

Behavior ports the current `chum archive` rules:

- move Markdown docs only
- never move live `*.spec.md` docs
- warn if `check` fails, but do not fail archive only for that reason
- write an archive README manifest
- warn on linked local assets that are not moved

## Spec Format Contract

The skill keeps the existing inline spec placement for v1:

- file spec: `src/foo.ts` -> `src/foo.ts.spec.md`
- directory spec: `src/auth/` -> `src/auth/auth.spec.md`
- root spec: `<repo-name>.spec.md`

Backmatter remains the machine-readable source of truth:

```markdown
<!-- chum:backmatter
schema: 1
kind: file
target: src/auth/session.ts
source_hash: sha256:...
source_updated_at: 2026-04-24T12:00:00Z
spec_updated_at: 2026-04-24T12:03:00Z
generated_by: chum skill
todo: []
unknowns: []
verify: []
-->
```

Completion still means `todo`, `unknowns`, and `verify` are empty, except where
the command explicitly allows external verification.

## Agent Workflow Contract

The skill should instruct the agent to:

- run `targets --json` first
- inspect existing specs before editing a folder
- read related code, not just the target file
- choose an adaptive route through the repo based on dependencies, imports,
  naming patterns, and unresolved docs
- write specs as current-state documentation, not implementation plans
- mark real gaps in backmatter instead of pretending certainty
- run `normalize` before writing or immediately after writing
- run `validate` after focused updates
- finish with `check --json`

The skill should not instruct the agent to call `codex exec` for per-file
analysis. The current Codex session is already the shared-context analyst.

## Migration Strategy

The Rust code remains temporarily as a behavioral reference and test oracle.

During migration:

- do not add new Rust-only product features
- do not invest further in npm/pnpm/Homebrew/native binary packaging
- use existing Rust fixtures and command behavior to define Python parity
- keep existing docs until the skill implementation is stable

After Python parity:

- remove Rust source and Cargo metadata
- remove npm wrapper files
- remove Codex exec provider implementation and auth docs that no longer apply
- update docs/specs to describe the skill-first architecture

## Phase Breakdown

- Phase 0: create the skill scaffold and declare the product pivot.
- Phase 1: port deterministic discovery, config, spec matching, and backmatter
  core to Python.
- Phase 2: implement the agent-facing target, normalize, validate, and check
  command surfaces.
- Phase 3: write the skill instructions and references for adaptive agent-led
  spec maintenance.
- Phase 4: port init and archive behavior.
- Phase 5: add tests, fixture parity, and install validation for the skill.
- Phase 6: remove Rust/npm/native packaging after parity.

## Cross-Cutting Risks

- Risk: the Python script drifts from Rust behavior during the transition.
  Mitigation: use shared fixtures, compare JSON output, and mark Rust as frozen.

- Risk: the skill becomes too verbose and consumes agent context.
  Mitigation: keep `SKILL.md` short; move schema and detailed examples into
  references.

- Risk: agents over-trust stale or incomplete specs.
  Mitigation: `targets` and `check` must surface stale hashes, unresolved lists,
  invalid backmatter, and legacy markers clearly.

- Risk: output gets too large on big repos.
  Mitigation: default summaries, `--limit`, `--offset`, and optional output files.

- Risk: `uv` is unavailable.
  Mitigation: document `uv` as the runtime prerequisite and fail clearly. Do not
  add fallback install flows in v1.

## Docs / Specs To Update

- [x] Add root `SKILL.md`.
- [x] Add `agents/openai.yaml`.
- [x] Add `references/spec-format.md`.
- [x] Add `references/workflow.md`.
- [x] Add `references/command-reference.md`.
- [x] Add `scripts/chum.py`.
- [x] Add or update specs for `scripts/`, `references/`, and `agents/`.
- [x] Update README/status docs to describe the skill-first product.
- [x] Mark Rust/native packaging plans as superseded when Phase 0 lands.

## Acceptance Criteria

- [x] `uv run scripts/chum.py --help` works with no install step, using
  `UV_CACHE_DIR=/tmp/chum-uv-cache` in this restricted sandbox.
- [x] `python3 scripts/chum.py targets --root <fixture> --json` reports missing,
  stale, invalid, and incomplete specs deterministically.
- [x] `python3 scripts/chum.py normalize --root <fixture> --target <path>
  --stdin` emits normalized Markdown with valid backmatter.
- [x] `python3 scripts/chum.py check --root <fixture> --json` exits `0` for a
  clean fixture and `1` for actionable spec failures.
- [x] The skill instructions describe the adaptive route workflow.
- [x] The skill can be installed or copied as a skill folder and used without
  Rust, npm, pnpm, Homebrew, chmod, or platform-specific binaries.
- [x] After Phase 6, no Rust or npm packaging files remain in the final skill
  artifact.
