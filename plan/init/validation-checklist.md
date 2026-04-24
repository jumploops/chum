# Validation Checklist: init

## Local Rust Validation

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test`
- [ ] `cargo run -- --help`
- [ ] `cargo run -- init --dry-run`
- [ ] `cargo run -- check --json`
- [ ] `cargo run -- archive example --dry-run`
- [ ] `cargo run -- swim --stubs --dry-run`

## Fixture Validation

- [ ] Plain non-Git directory
- [ ] Git repo with `.gitignore`
- [ ] Directory with `.chumignore`
- [ ] Repo with missing file specs
- [ ] Repo with missing directory specs
- [ ] Repo with stale source hash
- [ ] Repo with TODO backmatter
- [ ] Repo with unknown backmatter
- [ ] Repo with verify backmatter
- [ ] Repo with verify backmatter and `--allow-external-verify`
- [ ] Repo with tests, fixtures, scripts, generated files, migrations, and config files excluded by default

## Archive Validation

- [ ] Frontmatter `change: <id>` discovery
- [ ] Folder match discovery
- [ ] Filename match discovery
- [ ] Explicit `--include` discovery
- [ ] Ambiguous match failure
- [ ] Dry run writes nothing
- [ ] Live specs are never moved
- [ ] Failed `chum check` warns but does not block
- [ ] Linked local assets warn and stay in place
- [ ] Archive manifest contains expected metadata
- [ ] Markdown links remain valid or warn

## Swim Validation

- [ ] File specs are generated before parent directory specs
- [ ] Directory specs reference child specs
- [ ] Existing current specs are skipped
- [ ] Stale specs are repaired
- [ ] `--repair` limits writes to incomplete targets
- [ ] `maxPasses` stops non-converging runs
- [ ] Failure output names unresolved TODOs, unknowns, and verify items
- [ ] JSON output includes created, updated, skipped, and unresolved counts

## Provider Validation

- [ ] Fake provider can converge a fixture repo
- [ ] Fake provider can produce unresolved gaps
- [ ] OpenAI provider reports missing auth clearly
- [ ] `OPENAI_API_KEY` path can be tested with mocked HTTP
- [ ] Codex login discovery result is logged at debug level, not exposed in normal output

## Packaging Validation

- [ ] macOS arm64 artifact runs `chum --help`
- [ ] macOS x64 artifact runs `chum --help`
- [ ] Linux arm64 artifact runs `chum --help`
- [ ] Linux x64 artifact runs `chum --help`
- [ ] `cargo install` smoke test succeeds from package contents
- [ ] `npm install -D @magicloops/chum` smoke test succeeds
- [ ] `pnpm add -D @magicloops/chum` smoke test succeeds
- [ ] Unsupported npm platform error is clear
