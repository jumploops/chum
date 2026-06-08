# Phase 4: Init And Archive

## Goal

Port the remaining deterministic workflow commands from the Rust CLI to the
Python skill script.

## Scope

In scope:

- `init`
- `archive`
- change doc discovery
- archive manifest writing
- Markdown-only movement
- link warning behavior
- JSON and dry-run output
- command tests

Out of scope:

- LLM/provider behavior
- native packaging
- Rust cleanup

## Command: `init`

Initialize or update the documentation workflow:

```bash
uv run scripts/chum.py init --root . --write
```

Behavior:

- dry-run by default
- create `chum.config.yaml` only when useful
- create `design/`, `plan/`, `debug/`, `review/`, and `archive/` as configured
- create `archive/README.md`
- optionally write or patch repository operating guidance when explicitly
  requested by flag
- avoid overwriting user-authored docs
- emit JSON when requested

Suggested options:

- `--root PATH`
- `--write`
- `--json`
- `--with-agents-template`

## Command: `archive`

Archive completed change docs:

```bash
uv run scripts/chum.py archive --root . auth-session-hardening --write --json
```

Behavior:

- discover matching active change docs by frontmatter `change`, folder name,
  filename, and explicit includes
- move Markdown docs only
- never move live `*.spec.md`
- ignore examples by default
- run `check` first and warn on failure without failing archive for that reason
- warn on linked local assets that are not moved
- write `archive/<change-id>/README.md`
- preserve relative structure under archive
- support dry-run

Suggested options:

- positional `change_id`
- `--root PATH`
- `--title TEXT`
- `--include GLOB`
- `--exclude GLOB`
- `--source-ref TEXT`
- `--pr TEXT`
- `--write`
- `--json`
- `--force`

## Archive Manifest

Keep the current Markdown manifest approach:

```markdown
---
id: auth-session-hardening
archived_at: 2026-04-24T12:00:00Z
source_ref: feature/auth-session-hardening
pr: 1842
check_status: failed
archived_paths:
  - design/auth-session-hardening.md
related_live_docs: []
warnings:
  - "chum check failed before archive"
---

# Auth session hardening
```

## Acceptance Criteria

- [x] `init --dry-run` reports planned writes.
- [x] `init --write` is idempotent.
- [x] `archive <id> --dry-run --json` reports a move plan.
- [x] `archive <id> --write` moves Markdown docs only.
- [x] Live `*.spec.md` files are never moved.
- [x] Failed `check` emits an archive warning but does not block archive.
- [x] Linked local assets produce warnings.
- [x] Archive manifest contains expected metadata.

## Dependencies

- Phase 2 command surfaces.
