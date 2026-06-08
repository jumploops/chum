# Phase 5: Validation And Install

## Goal

Prove the Python skill workflow works without Rust, npm, pnpm, Homebrew, chmod,
or platform-specific binaries.

## Scope

In scope:

- Python unit tests
- command integration tests
- fixture parity tests against the frozen Rust behavior where useful
- skill install/copy smoke tests
- large-output behavior tests
- docs/reference consistency checks
- final readiness checklist before Rust cleanup

Out of scope:

- removing Rust files
- publishing automation beyond local install validation

## Test Strategy

Use lightweight Python tests. Avoid requiring the skill runtime to be installed
as a Python package.

Recommended local validation commands:

```bash
uv run scripts/chum.py --help
python3 -m unittest discover tests
```

Fixture tests should cover:

- clean repo
- missing file spec
- missing directory spec
- stale source hash
- invalid backmatter
- TODO/unknown/verify lists
- legacy `SPEC:*` markers
- archive dry-run
- archive write
- `.gitignore`
- `.chumignore`
- excluded tests/fixtures/scripts/generated/migrations/config
- non-Git directory

## Rust Parity

Before deleting Rust, compare key Python outputs to current Rust behavior:

- `check --json`
- `archive --dry-run --json`
- source discovery counts
- spec path matching
- backmatter normalization

Exact JSON shapes may differ, but semantic results should match. Any intentional
change should be documented in this plan before cleanup.

## Skill Install Smoke

Validate that the skill can be used as a skill folder:

- copy or install the repo/folder into a temporary skills directory
- inspect that `SKILL.md` frontmatter is valid
- run `uv run scripts/chum.py --help` from the skill root
- run `targets --json` against a fixture repo
- run the documented workflow manually on a tiny fixture

## Acceptance Criteria

- [x] `uv run scripts/chum.py --help` works from a clean checkout, using
  `UV_CACHE_DIR=/tmp/chum-uv-cache` in this restricted sandbox.
- [x] `python3 scripts/chum.py --help` works from a clean checkout.
- [x] `python3 -m unittest discover tests` passes.
- [x] Command tests cover `targets`, `check`, `normalize`, `validate`, `init`,
  and `archive`.
- [x] Python behavior reaches documented parity with frozen Rust behavior.
- [x] Skill metadata validates.
- [x] The skill workflow succeeds on a fixture without Rust installed.
- [x] Large target output is bounded or redirected.
- [x] Docs and command help agree.

## Dependencies

- Phase 3 skill workflow.
- Phase 4 init/archive.
