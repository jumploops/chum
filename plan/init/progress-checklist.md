# Progress Checklist: init

## Phase 0: Scaffold

- [x] Create Rust crate named `chum`
- [x] Add CLI command skeleton
- [x] Add module structure
- [x] Add test fixtures
- [x] Add npm wrapper skeleton
- [x] Add initial CI placeholders
- [x] Add live specs for created source folders

## Phase 1: Workflow Core

- [x] Implement config defaults
- [x] Implement `chum.config.yaml` loading
- [x] Implement `.gitignore` and `.chumignore` handling
- [x] Implement source discovery
- [x] Implement inline spec matching
- [x] Implement frontmatter parsing
- [x] Implement `chum:backmatter` parsing and writing
- [x] Implement legacy `SPEC:*` marker detection
- [x] Implement `chum init`
- [x] Implement `chum check`
- [x] Implement JSON output

## Phase 2: Archive

- [x] Implement change doc discovery
- [x] Implement archive dry-run plans
- [x] Protect live `*.spec.md` files from movement
- [x] Move Markdown docs only
- [x] Warn on failed `chum check`
- [x] Generate archive manifest
- [x] Rewrite or preserve Markdown links
- [x] Warn on linked local assets
- [x] Implement archive JSON output

## Phase 3: Swim Stubs

- [x] Build source tree model
- [x] Implement deepest-first traversal
- [x] Generate file spec stubs
- [x] Generate directory spec stubs
- [x] Write source hashes and timestamps
- [x] Skip current specs
- [x] Detect stale specs
- [x] Implement `--repair`
- [x] Implement swim JSON output

## Phase 4: AI Swim

- [x] Define provider trait
- [x] Add fake provider tests
- [x] Implement OpenAI provider shell
- [ ] Spike Codex login discovery
- [x] Implement `OPENAI_API_KEY` fallback
- [x] Generate file specs
- [x] Generate directory specs from child specs
- [x] Implement repair passes
- [x] Enforce `maxPasses`
- [x] Implement `--allow-external-verify`

## Phase 5: Packaging

- [x] Complete Cargo package metadata
- [ ] Build macOS arm64 binary
- [ ] Build macOS x64 binary
- [ ] Build Linux arm64 binary
- [ ] Build Linux x64 binary
- [ ] Create checksums
- [x] Implement npm binary wrapper
- [ ] Test npm install
- [ ] Test pnpm install
- [x] Document Homebrew as future work
