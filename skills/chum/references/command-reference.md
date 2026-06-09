# Command Reference

Resolve the script path relative to the installed skill directory. If the skill
is installed outside the target repository, pass the target repository with
`--root`:

```bash
uv run /path/to/chum/scripts/chum.py <command> --root /path/to/repo
```

For local smoke checks from the skill root:

```bash
uv run scripts/chum.py <command> [options]
```

Development fallback when `uv` is unavailable:

```bash
python3 scripts/chum.py <command> [options]
```

Restricted sandbox fallback when `uv` cannot write its default cache:

```bash
UV_CACHE_DIR=/tmp/chum-uv-cache uv run scripts/chum.py <command> [options]
```

## Commands

- `targets --root . --json` - Lists missing, stale, invalid, and incomplete specs.
- `check --root . --json` - Validates all required specs.
- `normalize --root . --target PATH --stdin --write` - Adds or updates backmatter.
- `validate --root . --target PATH --json` - Validates one source file or directory.
- `init --root . --write` - Creates workflow scaffolding.
- `archive --root . CHANGE --write --json` - Moves completed `design/`, `plan/`, `debug/`, and `review/` Markdown docs into archive history.

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | success |
| `1` | validation completed and found actionable failures |
| `2` | invalid arguments or config |
| `3` | filesystem, parse, or normalization error |
| `4` | unsupported environment |

JSON output uses camelCase field names. Diagnostics and warnings go to stderr.
