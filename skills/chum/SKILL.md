---
name: chum
description: Maintain repository live specs and archive change docs using the chum documentation workflow. Use when a repo has or needs *.spec.md files, design/plan/debug/review intent docs, archive history, or agent-led documentation updates.
---

# chum

Use this skill to maintain current-state repository specs and workflow docs.

## Available Scripts

- `scripts/chum.py` - Deterministic repository processor for discovery, spec validation, backmatter normalization, init, and archive.

Resolve script paths relative to this skill directory. If the skill is installed
outside the target repository, pass the target repository with `--root`:

```bash
uv run /path/to/chum/scripts/chum.py targets --root /path/to/repo --json
```

For local smoke checks from the skill root:

```bash
uv run scripts/chum.py --help
```

If `uv` is unavailable in the local environment, use `python3 scripts/chum.py ...` as a development fallback and report that `uv` could not be validated.
If `uv` is installed but its default cache path is not writable, run with a
temporary cache:

```bash
UV_CACHE_DIR=/tmp/chum-uv-cache uv run scripts/chum.py --help
```

## Workflow

1. Inspect targets:

   ```bash
   uv run /path/to/chum/scripts/chum.py targets --root . --json
   ```

2. Read relevant existing specs and related source files before editing. Choose a route based on imports, shared abstractions, naming patterns, and directories, not just one file at a time.
3. Write or update specs as current-state documentation.
4. Normalize each updated spec:

   ```bash
   uv run /path/to/chum/scripts/chum.py normalize --root . --target path/to/source --stdin --write
   ```

5. Validate focused targets:

   ```bash
   uv run /path/to/chum/scripts/chum.py validate --root . --target path/to/source --json
   ```

6. Finish with:

   ```bash
   uv run /path/to/chum/scripts/chum.py check --root . --json
   ```

Do not use per-file `codex exec` analysis for this workflow. The active agent session should keep the shared codebase context and use the script only for deterministic filesystem checks.

## References

- `references/spec-format.md` - Spec placement, content expectations, and backmatter fields.
- `references/workflow.md` - Detailed adaptive agent workflow.
- `references/command-reference.md` - Script commands, flags, JSON shapes, and exit codes.
