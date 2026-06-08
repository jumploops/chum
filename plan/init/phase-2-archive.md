# Phase 2: Archive

## Goal

Implement `chum archive <change-id>` so completed active Markdown docs can move into `archive/<change-id>/` while live specs remain the current source of truth.

## Scope

In scope:

- change doc discovery
- dry-run archive plans
- Markdown-only movement
- live spec protection
- `chum check` warning before archive
- archive manifest generation
- relative Markdown link preservation or rewriting
- warnings for linked local assets that are not moved
- JSON output

Out of scope:

- moving image or diagram assets
- blocking archive on failed `chum check`
- Git branch, commit, or PR automation
- archive search or retrieval

## Implementation Notes

### Discovery Order

Find candidate docs in this deterministic order:

1. frontmatter `change: <change-id>`
2. folder match under active docs, such as `plan/<change-id>/...`
3. filename match, such as `design/<change-id>.md`
4. explicit `--include` globs

If automatic discovery finds ambiguous or broad matches, stop and require explicit `--include`.

### Movement Rules

- Move Markdown docs only.
- Never move `*.spec.md`.
- Never move files from `archive/**`.
- Preserve top-level doc kind inside the archive entry.
- Warn about local assets linked from moved docs, but leave them in place.

Example target:

```text
archive/auth-session-hardening/
+-- README.md
+-- design/
|   +-- auth-session-hardening.md
+-- plan/
    +-- phase-1.md
```

### Check Behavior

Before a real archive, run the same validation as `chum check`. If it fails:

- print a warning
- include the failure summary in JSON output
- write the warning into the archive manifest
- continue archive unless another archive-specific validation fails

### Link Handling

For Markdown links inside moved files:

- links to files also moved should remain relative within the archive entry
- links to live specs should be rewritten relative to the archived file's new location
- unresolved local links should produce warnings

## Acceptance Criteria

- [x] `chum archive <id> --dry-run` writes nothing and prints a move plan.
- [x] `chum archive <id>` moves only selected Markdown docs.
- [x] `*.spec.md` files are never moved.
- [x] failed `chum check` emits a warning but does not block archive.
- [x] archive manifest includes id, timestamp, archived paths, warnings, and optional PR/source metadata.
- [x] linked local assets produce warnings and are not moved.
- [x] JSON output includes moved files, warnings, and check status.

## Dependencies

- Phase 1 workflow core.
