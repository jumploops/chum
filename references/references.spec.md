# `references/`

## Purpose

Detailed skill reference material that agents load only when the concise
`SKILL.md` workflow is not enough.

## Files

- `spec-format.md` - Inline spec placement, spec content expectations, and
  chum backmatter semantics.
- `workflow.md` - Adaptive agent-led repo traversal and spec maintenance loop.
- `command-reference.md` - Script command surface, examples, and exit codes.

## Dependencies / Contracts

- Reference files must not duplicate the entire `SKILL.md`; they provide
  progressively disclosed detail.
- Command documentation must match `scripts/chum.py --help`.

<!-- chum:backmatter
schema: 1
kind: directory
target: references
spec_updated_at: 2026-04-28T00:00:00Z
generated_by: chum skill
children: []
todo: []
unknowns: []
verify: []
-->
