# chum

`chum` is a filesystem-first documentation workflow CLI.

It turns the repository workflow described in `AGENTS.template.md` into executable commands:

```bash
chum init
chum check
chum archive <change-id>
chum swim [path]
```

V1 is implemented as a Rust binary. The npm package `@magicloops/chum` is a thin wrapper around platform-specific native binaries.

## Codex / OpenAI Auth

`chum swim --provider openai` prefers your existing Codex login. Run one of:

```bash
codex login
codex login --device-auth
```

For Codex automation, `CODEX_API_KEY` is passed through to `codex exec`. For
direct OpenAI API fallback without Codex, set `CHUM_OPENAI_API_KEY` or
`OPENAI_API_KEY`.

Check the selected auth path without generating specs:

```bash
chum swim --auth-status
chum swim --auth-status --json
```

Optional config:

```yaml
swim:
  provider: openai
  openai:
    auth: auto # auto | codex | apiKey
    codexBinary: codex
    model: null
    reasoningEffort: null
```

## Development

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Status

The implementation follows the phased plan in `plan/init/`.
