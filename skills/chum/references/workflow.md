# Workflow

The skill workflow is agent-led. The script gives deterministic facts; the
agent decides the route.

## Loop

1. Run `uv run scripts/chum.py targets --root . --json`.
2. Read existing specs before editing a folder.
3. Read related source files, not just the file named by the target.
4. Choose a route through the repo based on imports, shared abstractions,
   naming conventions, and directory boundaries.
5. Write current-state specs.
6. Run `normalize` for each edited spec.
7. Run `validate` after focused batches.
8. Finish with `check --json`.

## Route Planning

Leaf-first traversal is useful, but it is not mandatory. If one central type,
helper, or module explains many targets, inspect it early. If a directory spec
needs child context first, defer the directory spec until the file specs are
good enough.

Do not fabricate certainty. If local context cannot resolve something, keep the
gap in backmatter.

Do not call per-file `codex exec` for analysis. The active agent session should
retain shared codebase context.
