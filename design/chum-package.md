# Design: chum Package

## Context

`AGENTS.template.md` defines a manual documentation workflow:

- write intent docs in `design/`, `plan/`, and `debug/` before substantial changes
- maintain live `*.spec.md` files as current-state documentation for folders and implementation artifacts
- require specs to move with code changes
- track unresolved work with `SPEC:TODO`, `SPEC:UNKNOWN`, and `SPEC:VERIFY` markers
- archive completed change docs without treating archive history as current truth

`reference/init.md` scopes the first package shape as a filesystem-first CLI with Git and GitHub as optional metadata sources. This design keeps that constraint: `chum` must work in any directory tree, whether or not the repository uses Git.

## Goals

- Provide one installable package with a `chum` binary.
- Make the AGENTS documentation workflow executable and checkable.
- Support conventional repos with zero or minimal config.
- Keep live docs, active change docs, and archive docs as ordinary Markdown files.
- Add `chum swim`, a repo-wide spec generation and repair command that works leaf-first until source files and directories have complete specs.
- Keep the package useful in non-Git directories, vendored source trees, generated snapshots, and ad hoc code folders.

## Non-Goals

- No database or external index in v1.
- No Git requirement for core behavior.
- No GitHub-only storage model.
- No custom docs renderer.
- No guarantee that `chum swim` can resolve unknowns without enough local source context.
- No automatic source-code edits from `chum swim`; it writes documentation only.

## Resolved V1 Decisions

- Implement the CLI in Rust.
- Publish the Rust crate as `chum`.
- Publish the npm package as `@magicloops/chum`.
- Ship native binaries for macOS arm64, macOS x64, Linux arm64, and Linux x64.
- Defer Homebrew to a later formula or tap.
- Use inline spec placement only.
- Use OpenAI as the default `swim` provider, preferring Codex login and falling back to `OPENAI_API_KEY`.
- Allow `swim` to mark specs complete without human review.
- Require empty `todo`, `unknowns`, and `verify` lists for success unless external verification is explicitly allowed.
- Make `chum archive` warn on `chum check` failure without blocking archive.
- Move docs only during archive.

## Package Shape

Build `chum` as a Rust CLI with a single native binary:

```bash
chum init
chum check
chum archive <change-id>
chum swim [path]
```

The Rust binary is the source of truth. Distribution should make that binary easy to install through common developer tooling:

```bash
cargo install chum
npm install -D @magicloops/chum
pnpm add -D @magicloops/chum
pnpm exec chum check
npx @magicloops/chum swim
chum archive auth-session-hardening --dry-run
```

Package names:

- Rust crate: `chum`
- npm package: `@magicloops/chum`

For npm, pnpm, and other JavaScript package managers, publish a thin package that downloads or invokes the platform-specific Rust binary. The JavaScript package should not reimplement CLI behavior. A failed binary install should produce a clear fallback message pointing to `cargo install chum`.

V1 native binary targets:

- macOS arm64
- macOS x64
- Linux arm64
- Linux x64

Homebrew support is a future distribution channel, not a v1 deliverable.

Rust dependencies should stay small and boring:

- `clap` for CLI parsing
- `ignore` or `walkdir` plus glob matching for traversal
- a Markdown parser for links, frontmatter boundaries, and backmatter blocks
- `serde` and YAML support for config, frontmatter, and backmatter
- `sha2` or equivalent hashing for source freshness
- optional HTTP/client crates behind provider features for `swim`
- optional Git metadata support behind a feature flag

## Documentation Model

`chum` treats repository docs as three context layers:

| Layer | Paths | Meaning |
| --- | --- | --- |
| Live specs | `**/*.spec.md` | Current source of truth for code, folders, contracts, dependencies, and technical debt |
| Active change docs | `design/`, `plan/`, `debug/`, `review/` | Intent, rollout, investigation, and review artifacts for active work |
| Archive docs | `archive/**` | Historical context only |

The default search and validation priority is:

1. live specs
2. active change docs
3. archive docs

Archive files are ignored by `chum check` unless explicitly included.

## Config

`chum init` creates `chum.config.yaml` only when a repo needs explicit settings. Without config, these defaults apply:

```yaml
version: 1

activeDirs:
  - design
  - plan
  - debug
  - review

archiveDir: archive
liveDocGlob: "**/*.spec.md"

source:
  respectGitignore: true
  ignoreFiles:
    - ".gitignore"
    - ".chumignore"
  include:
    - "**/*.{c,cc,cpp,cxx,h,hpp,cs,css,go,html,java,js,jsx,kt,kts,m,mm,php,py,rb,rs,scss,sh,swift,ts,tsx,vue}"
  exclude:
    - ".git/**"
    - ".hg/**"
    - ".svn/**"
    - "node_modules/**"
    - "vendor/**"
    - "dist/**"
    - "build/**"
    - "target/**"
    - "coverage/**"
    - "archive/**"
    - "**/{test,tests,__tests__,spec,specs,fixture,fixtures,script,scripts,migration,migrations}/**"
    - "**/*.{test,spec}.{js,jsx,ts,tsx,py,rb,go,rs,swift,java,kt,kts,cs,php}"
    - "**/*config.{js,jsx,ts,tsx,cjs,mjs,json,yaml,yml,toml}"
    - "**/*.min.*"
    - "**/*.generated.*"

specs:
  placement: inline
  filePattern: "{path}.spec.md"
  directoryPattern: "{dir}/{basename}.spec.md"
  rootSpec: "repo.spec.md"
  backmatter: required

markers:
  todo:
    - "SPEC:TODO"
  unknown:
    - "SPEC:UNKNOWN"
  verify:
    - "SPEC:VERIFY"

swim:
  provider: openai
  maxPasses: 5
  concurrency: 4
  requireEmptyTodoUnknownAndVerify: true
  allowExternalVerify: false
```

Git-specific fields, when present, are optional enrichments. Core traversal, checking, archiving, and swimming must operate from the filesystem alone.

## Source Discovery

V1 source discovery is conservative:

- include source code files by language extension
- exclude Markdown, plaintext, media, binaries, lockfiles, and data files by default
- respect `.gitignore` even when the target directory is not being operated on through Git commands
- respect optional `.chumignore` files for documentation-specific exclusions
- do not require specs for tests, fixtures, scripts, generated files, migrations, or config files by default
- allow explicit includes for repos that want to document one of the default-excluded categories

The important rule is that `chum` should document the durable implementation surface by default, not every file that happens to contain code-like syntax.

## Command Surface

### `chum init`

Initializes the workflow in the current directory.

Responsibilities:

- detect existing `design/`, `plan/`, `debug/`, `review/`, `archive/`, and `*.spec.md` conventions
- create missing workflow directories when requested
- create `archive/README.md`
- create or update `chum.config.yaml`
- optionally add an AGENTS snippet that explains the live, active, and archive doc layers
- print the detected convention and next recommended command

Important flags:

```bash
chum init --dry-run
chum init --write
chum init --agent-doc AGENTS.md
chum init --no-agent-doc
```

### `chum check`

Validates the current documentation workflow.

Default checks:

- every included source file has a matching file spec
- every included source directory has a matching directory spec
- live specs do not contain unresolved TODO or unknown backmatter
- live specs do not contain unresolved verify backmatter unless external verification is explicitly allowed
- live specs do not contain unresolved `SPEC:TODO` or `SPEC:UNKNOWN` markers
- live specs do not contain unresolved `SPEC:VERIFY` markers unless external verification is explicitly allowed
- archive docs are ignored unless explicitly included
- generated, vendored, dependency, and configured ignored paths are skipped

Important flags:

```bash
chum check --json
chum check --allow-external-verify
chum check --include "packages/api/**"
chum check --include-archive
```

By default, `verify: []` must be empty for success. `--allow-external-verify` allows verify items only when they document something that cannot be validated from files in the local repository.

### `chum archive <change-id>`

Moves completed active change docs into `archive/<change-id>/`.

Responsibilities:

- discover docs by frontmatter `change: <id>`, folder match, filename match, and explicit includes
- fail closed when matches are ambiguous
- run `chum check` before moving and alert if it fails, but do not block archive on check failure
- never move live `*.spec.md` files
- preserve or rewrite Markdown links so archived docs remain usable
- move Markdown docs only
- warn about linked local assets that remain outside the archived doc set
- create `archive/<change-id>/README.md` as a Markdown manifest

Important flags:

```bash
chum archive auth-session-hardening --dry-run
chum archive auth-session-hardening --include "design/auth.md"
chum archive auth-session-hardening --source-ref feature/auth-session-hardening
chum archive auth-session-hardening --pr 1842
chum archive auth-session-hardening --json
```

## `chum swim`

`chum swim` is the command that turns an undocumented source tree into a fully documented tree using a leaf-first traversal. It should be safe to run repeatedly.

### Purpose

Given a repo or directory tree, `chum swim` should:

- discover all source files under the target path
- create or update one file spec for each source file
- create or update one directory spec for each source directory
- use backmatter to record gaps, TODOs, unknowns, verify items, source hashes, and timestamps
- traverse from leaves toward the root so parent directory specs are synthesized from child specs
- repeat resolution passes until every included source file has a matching spec with empty TODO, unknown, and verify lists
- stop with a precise report if local context is insufficient to remove remaining gaps

### Matching Rules

Default inline spec placement:

| Target | Spec Path |
| --- | --- |
| `src/auth/session.ts` | `src/auth/session.ts.spec.md` |
| `src/auth/` | `src/auth/auth.spec.md` |
| repo root | `repo.spec.md` |

These conventions match the existing `*.spec.md` workflow while allowing a spec per file and per directory. Inline placement is the only v1 placement mode.

### Backmatter

`chum swim` writes a final YAML-like backmatter block to every spec it owns. It is intentionally similar to Markdown frontmatter, but uses a custom HTML comment fence so specs can keep normal prose at the top of the file. This is the canonical machine-readable state for `chum check`.

Example for a file spec:

```markdown
# `src/auth/session.ts`

## Purpose

Manages session creation, refresh, and invalidation.

## Key Exports

- `createSession`
- `refreshSession`
- `revokeSession`

## Dependencies / Contracts

- Reads token lifetimes from `src/auth/config.ts`.
- Persists session records through `SessionStore`.

## Notes

- Refresh operations must preserve the original actor identity.

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

Example for a directory spec:

```markdown
# `src/auth/`

## Purpose

Authentication and session lifecycle code.

## Files

- `session.ts` - session lifecycle operations.
- `token.ts` - token issuance and verification.

## Subfolders

- `providers/` - provider-specific login adapters.

## Dependencies / Contracts

- Owns authenticated actor resolution.
- Must not expose raw provider tokens outside this package.

<!-- chum:backmatter
schema: 1
kind: directory
target: src/auth
children:
  - src/auth/session.ts.spec.md
  - src/auth/token.ts.spec.md
  - src/auth/providers/providers.spec.md
todo: []
unknowns: []
verify: []
-->
```

`todo`, `unknowns`, and `verify` must be empty for a completed swim by default. `verify` can remain only when external verification is explicitly allowed because the item depends on context outside the repository. `swim` is allowed to mark specs complete without human review, and no human review field is required.

### Swim Algorithm

1. Load config and resolve the target path.
2. Discover source files with include and exclude globs.
3. Build a tree containing source files, source directories, and existing specs.
4. Compute a deepest-first traversal order.
5. Generate or repair file specs for leaf source files.
6. Generate or repair specs for directories whose children already have specs.
7. Run a validation pass equivalent to `chum check`.
8. If TODO, unknown, or verify backmatter remains, run another resolution pass using the referenced local files and child specs as context.
9. Stop successfully when all included source files and source directories have matching specs with empty TODO, unknown, and verify lists, unless external verification was explicitly allowed.
10. Stop unsuccessfully when `maxPasses` is reached or a remaining unknown cannot be resolved from local files.

The command should be resumable. Existing complete specs are skipped when their recorded `source_hash` still matches the target file. If hashing is unavailable or disabled, `chum` may compare source file update time against `spec_updated_at` to decide whether a spec is stale.

### Swim Modes

```bash
chum swim
chum swim packages/api
chum swim --dry-run
chum swim --write
chum swim --repair
chum swim --json
chum swim --max-passes 8
chum swim --provider openai
chum swim --stubs
chum swim --allow-external-verify
```

Modes:

- `--dry-run`: report planned spec writes and unresolved gaps without writing files.
- `--write`: write generated specs.
- `--repair`: focus only on missing, stale, or incomplete specs.
- `--stubs`: create structured specs with TODO and unknown backmatter without using an AI provider.
- `--json`: emit machine-readable progress, writes, skipped files, and unresolved gaps.
- `--allow-external-verify`: allow verify items that explicitly require information outside the local repository.

### Provider Interface

`swim` needs an AI-capable adapter, but the traversal engine should stay provider-agnostic. The default v1 provider is OpenAI, using the developer's Codex login when available and falling back to an OpenAI API key from the environment, such as `OPENAI_API_KEY`.

Provider contract:

```rust
#[async_trait]
pub trait ChumSwimProvider {
    async fn generate_file_spec(&self, input: FileSpecInput) -> anyhow::Result<SpecDraft>;
    async fn generate_directory_spec(&self, input: DirectorySpecInput) -> anyhow::Result<SpecDraft>;
    async fn repair_spec(&self, input: RepairSpecInput) -> anyhow::Result<SpecDraft>;
}
```

The core engine owns:

- traversal
- path matching
- Markdown and backmatter parsing
- source hashing
- validation
- write planning

The provider owns:

- summarizing code behavior
- identifying dependencies and contracts
- explaining unresolved gaps
- removing TODO and unknown entries when local context supports doing so

Provider output must be validated before writing. Invalid or non-empty TODO, unknown, and verify output is allowed during intermediate passes, but not for a successful final result unless external verification was explicitly allowed.

### Failure Behavior

`chum swim` should fail closed.

It must not silently mark uncertain docs as complete. If it cannot resolve a gap, it should leave explicit backmatter:

```yaml
todo:
  - "Document storage lifecycle after migration docs are added."
unknowns:
  - "Cannot determine whether SessionStore is process-local or durable from local source files."
```

The final terminal summary should name:

- target path
- source files discovered
- specs created
- specs updated
- specs skipped
- remaining TODOs and unknowns
- next files a human should read or provide

## Check Semantics

A source file is complete when:

- its matching spec file exists
- the spec has valid `chum:backmatter`
- `kind: file`
- `target` points to the source file
- `source_hash` matches the source file, unless `--allow-stale` is used
- `todo: []`
- `unknowns: []`
- `verify: []`, unless external verification is explicitly allowed

A source directory is complete when:

- its matching directory spec exists
- the spec has valid `chum:backmatter`
- `kind: directory`
- `target` points to the directory
- child source files and source directories are listed or intentionally ignored
- `todo: []`
- `unknowns: []`
- `verify: []`, unless external verification is explicitly allowed

Legacy marker support:

- `SPEC:TODO` maps to an unresolved todo
- `SPEC:UNKNOWN` maps to an unresolved unknown
- `SPEC:VERIFY` maps to an unresolved verify item

This keeps existing AGENTS-compatible specs valid while giving `chum` a structured state model.

## Implementation Phases

### Phase 1: Workflow Core

- CLI shell
- config loading
- ignore handling
- spec discovery
- marker and backmatter parsing
- `chum init`
- `chum check`

### Phase 2: Archive

- change doc discovery
- dry-run move plans
- archive manifests
- Markdown link rewriting
- linked local asset warnings
- `chum archive`

### Phase 3: Swim Stubs

- source tree discovery
- file and directory spec path matching
- leaf-first traversal
- `chum swim --stubs`
- structured backmatter writes
- resumable check integration

### Phase 4: AI Swim

- provider adapter interface
- file spec generation
- directory spec synthesis from child specs
- repair passes
- final unresolved gap reporting

### Phase 5: Packaging

- publish Rust crate and native release binaries
- publish npm package with platform-specific binary wrappers
- publish macOS arm64, macOS x64, Linux arm64, and Linux x64 binaries
- package-manager smoke tests for `cargo`, `npm`, and `pnpm`
- non-Git directory smoke test
- CI examples for `chum check`

## Acceptance Criteria

- `chum init` works in a plain directory without Git.
- `chum check` validates live specs and ignores `archive/**` by default.
- `chum archive <id> --dry-run` prints exactly what would move.
- `chum archive <id>` never moves live `*.spec.md` files.
- `chum archive <id>` alerts when `chum check` fails but still archives selected docs.
- `chum swim --stubs` can create deterministic specs and backmatter without network access.
- `chum swim --write` can converge a small repo to complete file and directory specs.
- `chum swim` exits non-zero when TODOs, unknowns, or verify items remain after configured passes, unless external verification was explicitly allowed.
- All machine-readable commands support `--json`.
- Existing `SPEC:*` markers remain searchable and checkable.

## Future Features

- Homebrew formula or custom tap.
- Windows release binaries.
- Mirror spec placement, such as `.chum/specs/src/foo.ts.spec.md`.
- Additional AI providers behind the provider interface.
- Optional archive asset movement for repos that want fully self-contained historical docs.
