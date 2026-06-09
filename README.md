<p align="center">
  <img src="./chum_logo.png" alt="chum logo" width="240">
</p>

# chum - Context Hierarchy Using Markdown

`chum` is an installable agent skill for maintaining filesystem-first
repository documentation.

It turns the workflow described in `AGENTS.template.md` into a skill plus a
deterministic Python processor:

```bash
uv run skills/chum/scripts/chum.py targets --root . --json
uv run skills/chum/scripts/chum.py normalize --root . --target src/foo.py --stdin --write
uv run skills/chum/scripts/chum.py validate --root . --target src/foo.py --json
uv run skills/chum/scripts/chum.py check --root . --json
uv run skills/chum/scripts/chum.py archive --root . <change-id> --write --json
```

The script does not call an LLM. The active agent session keeps shared codebase
context, plans its own route through related files and directories, writes
current-state specs, and uses `scripts/chum.py` for discovery, validation,
normalization, init, and archive mechanics.

## Install

The publishable skill lives in [`skills/chum/`](./skills/chum/).

### Codex

To install from GitHub in Codex, ask Codex to install:

```text
https://github.com/jumploops/chum/tree/main/skills/chum
```

Then restart Codex so the new skill is picked up.

For a local manual install:

```bash
mkdir -p ~/.codex/skills
cp -R skills/chum ~/.codex/skills/
```

### Claude Code

For a personal Claude Code skill available across projects:

```bash
mkdir -p ~/.claude/skills
cp -R skills/chum ~/.claude/skills/
```

For a project-local Claude Code skill, copy it into that project's
`.claude/skills/` directory:

```bash
mkdir -p /path/to/project/.claude/skills
cp -R skills/chum /path/to/project/.claude/skills/
```

Claude Code exposes the skill as `/chum` from the installed directory name. If
Claude Code was already running and the target `skills` directory did not exist
yet, restart Claude Code so it can discover the new skill directory.

The publishable skill surface is:

- `skills/chum/SKILL.md`
- `skills/chum/agents/openai.yaml`
- `skills/chum/scripts/chum.py`
- `skills/chum/references/`

The remaining files are project docs and tests for maintaining this repo.

## Skill Usage

Start with [skills/chum/SKILL.md](./skills/chum/SKILL.md). The usual loop is:

1. Run `targets --json`.
2. Read existing specs and related source files.
3. Update specs with accumulated repo context.
4. Run `normalize` and `validate` for focused targets.
5. Finish with `check --json`.

Use `python3 skills/chum/scripts/chum.py ...` as a local development fallback
when `uv` is not installed.
In restricted sandboxes where `uv` cannot write its default cache, set
`UV_CACHE_DIR=/tmp/chum-uv-cache`.

When the skill is installed outside the target repo, resolve the script path
from the installed skill directory and pass the repository to inspect via
`--root`:

```bash
uv run /path/to/chum/scripts/chum.py targets --root /path/to/repo --json
```

## Development

```bash
python3 skills/chum/scripts/chum.py --help
python3 -m unittest discover tests
UV_CACHE_DIR=/tmp/chum-uv-cache uv run skills/chum/scripts/chum.py --help
python3 skills/chum/scripts/chum.py check --root . --json
```

## Status

The current implementation is the Python skill surface in `SKILL.md`,
`scripts/`, `references/`, and `agents/` under `skills/chum/`.
