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

## Development

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Status

The implementation follows the phased plan in `plan/init/`.
