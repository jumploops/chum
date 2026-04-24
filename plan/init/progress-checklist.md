# Progress Checklist: init

## Phase 0: Scaffold

- [ ] Create Rust crate named `chum`
- [ ] Add CLI command skeleton
- [ ] Add module structure
- [ ] Add test fixtures
- [ ] Add npm wrapper skeleton
- [ ] Add initial CI placeholders
- [ ] Add live specs for created source folders

## Phase 1: Workflow Core

- [ ] Implement config defaults
- [ ] Implement `chum.config.yaml` loading
- [ ] Implement `.gitignore` and `.chumignore` handling
- [ ] Implement source discovery
- [ ] Implement inline spec matching
- [ ] Implement frontmatter parsing
- [ ] Implement `chum:backmatter` parsing and writing
- [ ] Implement legacy `SPEC:*` marker detection
- [ ] Implement `chum init`
- [ ] Implement `chum check`
- [ ] Implement JSON output

## Phase 2: Archive

- [ ] Implement change doc discovery
- [ ] Implement archive dry-run plans
- [ ] Protect live `*.spec.md` files from movement
- [ ] Move Markdown docs only
- [ ] Warn on failed `chum check`
- [ ] Generate archive manifest
- [ ] Rewrite or preserve Markdown links
- [ ] Warn on linked local assets
- [ ] Implement archive JSON output

## Phase 3: Swim Stubs

- [ ] Build source tree model
- [ ] Implement deepest-first traversal
- [ ] Generate file spec stubs
- [ ] Generate directory spec stubs
- [ ] Write source hashes and timestamps
- [ ] Skip current specs
- [ ] Detect stale specs
- [ ] Implement `--repair`
- [ ] Implement swim JSON output

## Phase 4: AI Swim

- [ ] Define provider trait
- [ ] Add fake provider tests
- [ ] Implement OpenAI provider shell
- [ ] Spike Codex login discovery
- [ ] Implement `OPENAI_API_KEY` fallback
- [ ] Generate file specs
- [ ] Generate directory specs from child specs
- [ ] Implement repair passes
- [ ] Enforce `maxPasses`
- [ ] Implement `--allow-external-verify`

## Phase 5: Packaging

- [ ] Complete Cargo package metadata
- [ ] Build macOS arm64 binary
- [ ] Build macOS x64 binary
- [ ] Build Linux arm64 binary
- [ ] Build Linux x64 binary
- [ ] Create checksums
- [ ] Implement npm binary wrapper
- [ ] Test npm install
- [ ] Test pnpm install
- [ ] Document Homebrew as future work
