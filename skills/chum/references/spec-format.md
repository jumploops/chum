# Spec Format

`chum` specs are current-state Markdown files that describe what exists now.
They are not plans, changelogs, or design proposals.

## Placement

V1 uses inline placement:

- file spec: `src/foo.py` -> `src/foo.py.spec.md`
- directory spec: `src/auth/` -> `src/auth/auth.spec.md`
- root spec: `repo.spec.md`

## File Specs

File specs should describe:

- purpose
- important exports or entry points
- dependencies and contracts
- behavior that future agents need before editing the file

## Directory Specs

Directory specs should describe:

- folder responsibility
- child files and child folders
- how children work together
- important cross-directory dependencies

## Backmatter

Every chum-owned spec ends with one backmatter block:

```markdown
<!-- chum:backmatter
schema: 1
kind: file
target: src/foo.py
source_hash: sha256:...
source_updated_at: 2026-04-24T12:00:00Z
spec_updated_at: 2026-04-24T12:03:00Z
generated_by: chum skill
todo: []
unknowns: []
verify: []
-->
```

Use `todo`, `unknowns`, and `verify` to mark real gaps. Completion means these
lists are empty unless external verification has explicitly been allowed.
