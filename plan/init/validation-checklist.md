# Validation Checklist: init

## Local Rust Validation

- [x] `cargo fmt --check`
- [x] `cargo clippy --all-targets --all-features -- -D warnings`
- [x] `cargo test`
- [x] `cargo run -- --help`
- [x] `cargo run -- init --dry-run`
- [x] `cargo run -- check --json`
- [x] `cargo run -- archive example --dry-run`
- [x] `cargo run -- swim --stubs --dry-run`

## Fixture Validation

- [x] Plain non-Git directory
- [x] Git repo with `.gitignore`
- [x] Directory with `.chumignore`
- [x] Repo with missing file specs
- [x] Repo with missing directory specs
- [x] Repo with stale source hash
- [x] Repo with TODO backmatter
- [x] Repo with unknown backmatter
- [x] Repo with verify backmatter
- [x] Repo with verify backmatter and `--allow-external-verify`
- [x] Repo with tests, fixtures, scripts, generated files, migrations, and config files excluded by default

## Archive Validation

- [x] Frontmatter `change: <id>` discovery
- [x] Folder match discovery
- [x] Filename match discovery
- [x] Explicit `--include` discovery
- [x] Ambiguous match failure
- [x] Dry run writes nothing
- [x] Live specs are never moved
- [x] Failed `chum check` warns but does not block
- [x] Linked local assets warn and stay in place
- [x] Archive manifest contains expected metadata
- [x] Markdown links remain valid or warn

## Swim Validation

- [x] File specs are generated before parent directory specs
- [x] Directory specs reference child specs
- [x] Existing current specs are skipped
- [x] Stale specs are repaired
- [ ] `--repair` limits writes to incomplete targets
- [x] `maxPasses` stops non-converging runs
- [x] Failure output names unresolved TODOs, unknowns, and verify items
- [x] JSON output includes created, updated, skipped, and unresolved counts

## Provider Validation

- [x] Fake provider can converge a fixture repo
- [x] Fake provider can produce unresolved gaps
- [x] OpenAI provider reports missing auth clearly
- [ ] `OPENAI_API_KEY` path can be tested with mocked HTTP
- [x] Codex auth status is exposed through `chum swim --auth-status` and normal generation output stays secret-free

## Packaging Validation

- [ ] macOS arm64 artifact runs `chum --help`
- [ ] macOS x64 artifact runs `chum --help`
- [ ] Linux arm64 artifact runs `chum --help`
- [ ] Linux x64 artifact runs `chum --help`
- [ ] `cargo install` smoke test succeeds from package contents
- [ ] `npm install -D @magicloops/chum` smoke test succeeds
- [ ] `pnpm add -D @magicloops/chum` smoke test succeeds
- [ ] Unsupported npm platform error is clear
