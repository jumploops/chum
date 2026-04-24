# Implementation Spec: init

## Context

- Related design doc: [`design/chum-package.md`](../../design/chum-package.md)
- Workflow source: [`AGENTS.template.md`](../../AGENTS.template.md)
- Initial package notes: [`reference/init.md`](../../reference/init.md)

`chum` turns the AGENTS documentation workflow into an installable Rust CLI. V1 must work in ordinary filesystem trees, including directories that are not Git repositories. Git and GitHub can enrich metadata later, but they are not core requirements.

## Objective

Build a Rust CLI named `chum` with these v1 commands:

```bash
chum init
chum check
chum archive <change-id>
chum swim [path]
```

End-state:

- The Rust crate is named `chum`.
- The npm package is named `@magicloops/chum` and wraps the native Rust binary.
- Native release binaries are produced for macOS arm64, macOS x64, Linux arm64, and Linux x64.
- `chum init` installs the docs workflow in a repo or directory.
- `chum check` validates live specs and ignores archive history by default.
- `chum archive` moves completed Markdown change docs into `archive/<change-id>/` without blocking on failed checks.
- `chum swim` can create or repair inline `*.spec.md` files leaf-first, first through deterministic stubs and then through the OpenAI provider.

## V1 Decisions

- Implementation language: Rust.
- Runtime dependency on Git: none.
- Default spec placement: inline only.
- Source ignore behavior: respect `.gitignore` and optional `.chumignore`.
- Default source scope: durable implementation code, excluding tests, fixtures, scripts, generated files, migrations, and config files.
- Default AI provider: OpenAI through Codex login when available, falling back to `OPENAI_API_KEY`.
- Human review: not required for `swim` completion.
- Completion state: `todo`, `unknowns`, and `verify` must be empty unless external verification is explicitly allowed.
- Archive behavior: move Markdown docs only; warn about failed `chum check` and linked local assets, but do not block.
- V1 distribution: Cargo and npm/pnpm wrappers. Homebrew is future work.

## Architecture

Keep the first implementation as one Rust crate with internal modules. Split into workspace crates only if the module boundaries become difficult to test.

Suggested module layout:

```text
src/
+-- main.rs
+-- cli.rs
+-- commands/
|   +-- archive.rs
|   +-- check.rs
|   +-- init.rs
|   +-- swim.rs
+-- config.rs
+-- discovery.rs
+-- docs/
|   +-- backmatter.rs
|   +-- frontmatter.rs
|   +-- links.rs
|   +-- markdown.rs
+-- fs.rs
+-- ignore.rs
+-- output.rs
+-- spec.rs
+-- provider/
    +-- mod.rs
    +-- openai.rs
```

Support files:

```text
npm/
+-- package.json
+-- bin/chum.js
+-- scripts/
    +-- install.js
    +-- resolve-binary.js

fixtures/
+-- ...
```

## Core Contracts

### CLI Output

- Human output is concise and actionable.
- Every command that mutates files supports `--dry-run`.
- Every machine-readable command supports `--json`.
- JSON output should be stable enough for CI and agents.

### Config

Default config should be available without a config file. `chum.config.yaml` only records overrides.

Important fields:

- `activeDirs`
- `archiveDir`
- `liveDocGlob`
- `source.respectGitignore`
- `source.ignoreFiles`
- `source.include`
- `source.exclude`
- `specs.filePattern`
- `specs.directoryPattern`
- `specs.rootSpec`
- `swim.provider`
- `swim.maxPasses`
- `swim.allowExternalVerify`

### Backmatter

Specs use a custom YAML-like backmatter fence at the end of the file:

```markdown
<!-- chum:backmatter
schema: 1
kind: file
target: src/auth/session.ts
source_hash: sha256:...
source_updated_at: 2026-04-24T12:00:00Z
spec_updated_at: 2026-04-24T12:03:00Z
generated_by: chum swim
todo: []
unknowns: []
verify: []
-->
```

Parser requirements:

- exactly one `chum:backmatter` block per chum-owned spec
- YAML body between the opening and closing comment fences
- unknown fields preserved when rewriting when possible
- invalid backmatter reported with file and line context

### Source Discovery

Discovery must:

- walk the filesystem using ignore-aware traversal
- respect `.gitignore` and `.chumignore`
- include source code extensions by default
- exclude Markdown, plaintext, media, binaries, lockfiles, data files, tests, fixtures, scripts, generated files, migrations, and config files by default
- allow explicit includes to override default exclusions

### Spec Matching

V1 uses inline placement:

- file spec: `src/foo.ts` -> `src/foo.ts.spec.md`
- directory spec: `src/auth/` -> `src/auth/auth.spec.md`
- root spec: `repo.spec.md`

No mirror placement in v1.

### Archive Manifest

Each archive entry writes `archive/<change-id>/README.md` with frontmatter-like metadata:

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

## Phase Breakdown

- Phase 0: scaffold the Rust crate, npm wrapper skeleton, fixtures, and CI placeholders.
- Phase 1: implement workflow core, `chum init`, and `chum check`.
- Phase 2: implement `chum archive`.
- Phase 3: implement deterministic `chum swim --stubs`.
- Phase 4: implement OpenAI-backed `chum swim`.
- Phase 5: implement release packaging for Cargo and npm/pnpm.

## Cross-Cutting Risks

- Risk: source discovery includes too much and makes `check` noisy.
  Mitigation: conservative defaults, `.chumignore`, explicit includes, fixture tests.

- Risk: backmatter format becomes hard to evolve.
  Mitigation: include `schema: 1`, tolerate unknown fields, centralize parser/writer.

- Risk: `swim` claims completion without enough context.
  Mitigation: require empty TODO, unknown, and verify lists by default; fail closed after `maxPasses`.

- Risk: npm wrapper and Rust binary drift.
  Mitigation: npm package only invokes native binary; version is sourced from release metadata.

- Risk: OpenAI Codex login integration is unclear.
  Mitigation: isolate auth discovery in the provider module and keep `OPENAI_API_KEY` fallback working.

## Docs / Specs To Update

- [ ] Add a root live spec once source files exist.
- [ ] Add specs for `src/`, `src/commands/`, `src/docs/`, and `src/provider/` during implementation.
- [ ] Update `AGENTS.template.md` if command names or workflow rules change.
- [ ] Keep this plan's progress and validation checklists current.

## Acceptance Criteria

- [ ] `cargo run -- init --dry-run` works in a plain directory.
- [ ] `cargo run -- check --json` reports missing specs deterministically.
- [ ] `cargo run -- archive example --dry-run` reports an archive move plan.
- [ ] `cargo run -- swim --stubs --dry-run` reports planned spec writes.
- [ ] `cargo run -- swim --stubs --write` can create inline specs in a fixture repo.
- [ ] `cargo test` covers config, ignore, discovery, backmatter, check, archive planning, and swim traversal.
- [ ] npm package can invoke the native binary on macOS and Linux release artifacts.
