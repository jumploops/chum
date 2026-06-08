# Validation Checklist: skill

## Local Script Validation

- [x] `python3 scripts/chum.py --help`
- [x] `python3 scripts/chum.py targets --help`
- [x] `python3 scripts/chum.py check --help`
- [x] `python3 scripts/chum.py normalize --help`
- [x] `python3 scripts/chum.py validate --help`
- [x] `python3 scripts/chum.py init --help`
- [x] `python3 scripts/chum.py archive --help`
- [x] `python3 -m unittest discover tests`

## Core Fixture Validation

- [x] Plain non-Git directory
- [x] Git repo with `.gitignore`
- [x] Directory with `.chumignore`
- [x] Repo with missing file specs
- [x] Repo with missing directory specs
- [x] Repo with stale source hash
- [x] Repo with invalid backmatter
- [x] Repo with TODO backmatter
- [x] Repo with unknown backmatter
- [x] Repo with verify backmatter
- [x] Repo with legacy `SPEC:*` markers
- [x] Repo with tests, fixtures, scripts, generated files, migrations, and config
  files excluded by default

## Command Validation

- [x] `targets --json` emits stable JSON
- [x] `targets --limit` bounds output
- [x] `targets --offset` paginates output
- [x] `check --json` exits `0` for clean fixture
- [x] `check --json` exits `1` for actionable failures
- [x] `normalize --stdin` emits normalized Markdown
- [x] `normalize --stdin --write` writes expected spec path
- [x] `validate --target --json` validates one target
- [x] Invalid arguments exit `2`
- [x] Parse/filesystem errors exit `3`

## Init / Archive Validation

- [x] `init --dry-run --json` writes nothing
- [x] `init --write` is idempotent
- [x] `archive <id> --dry-run --json` writes nothing
- [x] `archive <id> --write` moves Markdown docs only
- [x] Live `*.spec.md` files are never moved
- [x] Failed `check` warns but does not block archive
- [x] Linked local assets warn and stay in place
- [x] Archive manifest contains expected metadata

## Skill Validation

- [x] `SKILL.md` has valid `name` and `description` frontmatter
- [x] `agents/openai.yaml` uses the current `interface` schema and matches the
  skill
- [x] `SKILL.md` lists available scripts and references
- [x] Skill workflow starts with `targets --json`
- [x] Skill workflow uses adaptive agent-led routing
- [x] Skill workflow tells agents to run `normalize`, `validate`, and final
  `check`
- [x] Skill references match command help and JSON behavior

## Install / Portability Validation

- [x] Skill works from a clean checkout with no Rust build
- [x] Skill works without npm/pnpm/Homebrew
- [x] Skill requires no chmod step
- [x] Skill smoke test works after copying folder to a temporary skills location
- [x] `uv` cache fallback is clear and validated with
  `UV_CACHE_DIR=/tmp/chum-uv-cache`
- [x] macOS arm64 smoke test
- [ ] macOS x64 smoke test
- [ ] Linux arm64 smoke test
- [ ] Linux x64 smoke test

## Cleanup Validation

- [x] No final workflow requires `cargo`
- [x] No final workflow requires `npm` or `pnpm`
- [x] No final workflow references native release artifacts
- [x] No final workflow uses `codex exec` for per-file analysis
- [x] Python tests pass after Rust/npm cleanup
- [x] Final specs describe the skill-first architecture

## Publishing Readiness

- [x] README identifies the repo root as the skill folder
- [x] README identifies the publishable skill surface
- [x] Installed-skill examples pass `--root` explicitly
- [x] Repo-local `chum.config.yaml` validates `scripts/chum.py`
- [x] App-server work remains documented as an optional future direction
