# Phase 2: Agent-Facing Command Surfaces

## Goal

Expose stable commands that a long-running agent can use to plan, write,
normalize, and validate specs.

## Scope

In scope:

- `targets`
- `check`
- `normalize`
- `validate`
- JSON output schemas
- bounded output controls
- dry-run/write behavior
- command-level tests

Out of scope:

- skill prose and reference docs
- archive movement
- Rust deletion

## Command: `targets`

Primary route-planning command:

```bash
uv run scripts/chum.py targets --root . --json
```

Options:

- `--root PATH`
- `--json`
- `--kind file|directory|root|all`
- `--reason missing|stale|invalid|incomplete|all`
- `--limit N`
- `--offset N`
- `--output PATH`

Output shape:

```json
{
  "root": "/repo",
  "summary": {
    "sourceFiles": 12,
    "sourceDirs": 4,
    "targets": 5,
    "missing": 2,
    "stale": 1,
    "invalid": 1,
    "incomplete": 1
  },
  "targets": [
    {
      "kind": "file",
      "sourcePath": "src/foo.py",
      "specPath": "src/foo.py.spec.md",
      "reasons": ["missing"],
      "sourceHash": "sha256:...",
      "sourceUpdatedAt": "2026-04-24T12:00:00Z",
      "todo": 0,
      "unknowns": 0,
      "verify": 0,
      "children": []
    }
  ],
  "warnings": []
}
```

`targets` must be deterministic and conservative. It should not decide the
agent's route; it should provide enough facts for the agent to choose one.

## Command: `check`

Whole-repo validator:

```bash
uv run scripts/chum.py check --root . --json
```

Behavior:

- exit `0` when clean
- exit `1` when actionable failures are found
- ignore `archive/**` by default
- report missing, stale, invalid, unresolved, and legacy marker failures
- include warnings separately from failures

Human output should be concise. JSON output should be stable.

## Command: `normalize`

Normalize agent-written Markdown:

```bash
uv run scripts/chum.py normalize --root . --target src/foo.py --stdin --write
```

Options:

- `--root PATH`
- `--target PATH`
- `--spec PATH`
- `--input PATH`
- `--stdin`
- `--write`
- `--json`

Behavior:

- target path identifies the source file or directory being documented
- default spec path is derived from inline placement
- dry-run prints normalized Markdown to stdout
- `--write` writes the normalized spec file
- parent directories are created as needed
- existing unresolved lists are preserved when present in supplied Markdown
- generated metadata is updated

When `--json` is used with `--write`, stdout should describe the write instead
of printing full Markdown.

## Command: `validate`

Focused validator:

```bash
uv run scripts/chum.py validate --root . --target src/foo.py --json
```

Behavior:

- validates one target/spec pair
- exits `0` if the target is clean
- exits `1` if validation finds target-specific failures
- reports the same failure types as `check`

This command lets agents validate incrementally while preserving their working
context.

## Output Rules

- JSON is printed to stdout.
- Logs and warnings go to stderr.
- Large outputs require `--output`, `--limit`, or pagination.
- Paths in JSON are relative to `root` unless explicitly documented.
- Field names use camelCase.
- Errors include a short actionable message.

## Acceptance Criteria

- [x] `targets --json` reports missing specs.
- [x] `targets --json` reports stale specs.
- [x] `targets --json` reports invalid backmatter.
- [x] `targets --json` reports incomplete specs.
- [x] `check --json` exits `0` for a clean fixture.
- [x] `check --json` exits `1` for actionable failures.
- [x] `normalize --stdin` emits valid Markdown without writing.
- [x] `normalize --stdin --write` writes the expected spec path.
- [x] `validate --target` validates one target without scanning unrelated output.
- [x] Human command output is concise enough for agent context.

## Dependencies

- Phase 1 Python core.
