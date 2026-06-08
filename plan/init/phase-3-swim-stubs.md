# Phase 3: Swim Stubs

## Goal

Implement deterministic `chum swim --stubs` so the traversal, matching, stale detection, and check integration can be built before AI generation exists.

## Scope

In scope:

- source tree construction
- deepest-first traversal
- file spec stub generation
- directory spec stub generation
- backmatter generation
- source hashing and update timestamps
- resumable operation
- dry-run and write modes
- repair mode for missing, stale, or incomplete specs
- JSON output

Out of scope:

- AI-generated prose
- OpenAI authentication
- provider repair passes
- mirror spec placement

## Implementation Notes

### Traversal

Build a tree from discovered source files and expected specs:

- file nodes represent source code files
- directory nodes represent directories containing source code descendants
- ignored files do not create nodes
- specs are attached to their target nodes when present

Process nodes deepest-first:

1. file nodes
2. directories whose child specs exist or are planned
3. root spec

### Stub Content

File spec stubs should be structured and intentionally incomplete:

```markdown
# `src/foo.ts`

## Purpose

<!-- SPEC:TODO -->

## Key Exports

<!-- SPEC:UNKNOWN -->

## Dependencies / Contracts

<!-- SPEC:UNKNOWN -->

<!-- chum:backmatter
schema: 1
kind: file
target: src/foo.ts
source_hash: sha256:...
source_updated_at: ...
spec_updated_at: ...
generated_by: chum swim --stubs
todo:
  - "Document file purpose."
unknowns:
  - "Document key exports and dependencies."
verify: []
-->
```

Directory specs should list child files and child directories even in stub mode. Their TODOs and unknowns should explain what still needs documentation.

### Completion

Stub mode does not need to converge to success. It proves traversal and write behavior. `chum check` should fail after stub generation until TODOs and unknowns are repaired.

### Repair

`--repair` should touch only:

- missing specs
- stale specs
- specs with invalid backmatter
- specs with non-empty TODO, unknown, or verify lists

## Acceptance Criteria

- [x] `chum swim --stubs --dry-run` reports planned writes without writing files.
- [x] `chum swim --stubs --write` creates inline file specs.
- [x] `chum swim --stubs --write` creates inline directory specs leaf-first.
- [x] Generated backmatter validates with the Phase 1 parser.
- [x] Existing complete specs with matching hashes are skipped.
- [x] Stale specs are detected by hash.
- [ ] Timestamp fallback is tested where hash comparison is unavailable or disabled.
- [ ] `--repair` limits writes to missing, stale, or incomplete specs.
- [x] `--json` reports created, updated, skipped, and unresolved counts.

## Dependencies

- Phase 1 workflow core.
