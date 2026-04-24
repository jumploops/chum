Yes — this is a good fit for `chum`.

I’d scope it as a **filesystem-first, markdown-native CLI**, with Git/GitHub as optional metadata sources rather than core dependencies.

## Core shape

Publish one package under your scope, with a `chum` binary.

```bash
pnpm add -D @your-scope/chum
pnpm exec chum init
pnpm exec chum check
pnpm exec chum archive-change auth-session-hardening
```

The mental model stays simple:

* `*.spec.md` = **live docs**
* `design/`, `plan/`, `debug/`, `review/` = **active change docs**
* `archive/` = **historical change docs**

And inside `chum`, archive becomes a **lower-priority context layer**:

1. live `.spec.md`
2. active change docs
3. `archive/**`

That should go into `AGENT.md`, and `chum init` can add that snippet automatically.

---

## The v1 command surface

I would keep v1 very small:

```bash
chum init
chum check
chum archive-change <change-id>
```

### `chum init`

Sets up the repo once.

What it should do:

* detect `design/`, `plan/`, `debug/`, `review/`
* detect your live doc pattern (`*.spec.md`)
* create `chum.config.yaml`
* create `archive/README.md`
* optionally append a small `AGENT.md` section
* optionally add `package.json` scripts

### `chum check`

Your existing behavior, made official.

Default behavior:

* validate source files have associated `.spec.md`
* fail on unresolved action items in live docs
* **ignore `archive/**` by default**

That last part matters a lot. Archive is historical and should not be treated like current spec debt.

### `chum archive-change <change-id>`

Moves completed change artifacts into `archive/`.

Default behavior:

* find all matching active change docs
* exclude examples
* never move live `*.spec.md`
* run `chum check` first
* fail if unresolved action items remain in selected change docs
* move files into `archive/<change-id>/...`
* write an archive summary file
* print a machine-readable summary with `--json`

---

## Recommended archive layout

Keep it dead simple:

```text
archive/
  README.md
  auth-session-hardening/
    README.md
    design/
      auth-session-hardening.md
    plan/
      phase-1.md
      phase-2.md
    debug/
      token-refresh.md
    review/
      final-review.md
```

I would use `archive/<change-id>/README.md` as the manifest, not JSON first.

That fits your markdown-heavy workflow better, and it’s easier for both people and agents to read.

Example:

```md
---
id: auth-session-hardening
title: Auth session hardening
archived_at: 2026-04-22T18:14:00Z
source_ref: feature/auth-session-hardening
pr: 1842
status: merged
related_live_docs:
  - src/auth/session.ts.spec.md
  - src/auth/token.ts.spec.md
archived_paths:
  - design/auth-session-hardening.md
  - plan/auth-session-hardening/phase-1.md
  - debug/token-refresh.md
  - review/final-review.md
---

# Auth session hardening

Historical change artifacts for this completed change.

Current source of truth lives in:
- `src/auth/session.ts.spec.md`
- `src/auth/token.ts.spec.md`
```

That `related_live_docs` field is the key bridge between your **historical change workflow** and your **live doc workflow**.

It means an agent opening an archive entry immediately knows where current truth lives.

---

## How `archive-change` should discover files

Do not use “recent files.”

Use a deterministic match order:

1. frontmatter `change: <id>`
2. folder match, like `design/<id>/...`
3. filename match, like `design/<id>.md`
4. explicit `--include` globs

That gives you a clean migration path:

* old repos can work with folder/file naming
* newer repos can use frontmatter for accuracy

I would support this minimal frontmatter on active change docs:

```md
---
change: auth-session-hardening
title: Auth session hardening
status: active
---
```

No need to require it on day one, but `chum` should prefer it when present.

---

## What the command should accept

I’d keep the interface narrow:

```bash
chum archive-change <id> \
  [--title "Auth session hardening"] \
  [--include "design/foo.md"] \
  [--include "plan/foo/**"] \
  [--exclude "**/examples/**"] \
  [--source-ref "feature/auth-session-hardening"] \
  [--pr "1842"] \
  [--dry-run] \
  [--json] \
  [--force]
```

Notes:

* `--dry-run` is mandatory for confidence
* `--json` is for agents/CI
* `--source-ref` and `--pr` are optional metadata only
* `--force` is the escape hatch when a human knows better than the validator

GitHub Actions can pass `--source-ref` / `--pr`, but the command still works without GitHub and without Git.

---

## What should count as “just works”

Zero-config behavior should be:

* if repo contains `design/`, `plan/`, `debug/`, or `review/`, treat them as active change dirs
* if repo contains `*.spec.md`, treat them as live docs
* ignore `archive/**`
* ignore examples under `examples/` and `*.example.md`

So most repos can do:

```bash
pnpm dlx @your-scope/chum init
```

and be done.

Then `chum.config.yaml` only needs to exist for overrides.

---

## Minimal config shape

I’d keep the config tiny:

```yaml
version: 1

activeDirs:
  - design
  - plan
  - debug
  - review

liveDocGlob: "**/*.spec.md"

archiveDir: archive

ignore:
  - "archive/**"

exampleGlobs:
  - "**/examples/**"
  - "**/*.example.md"

changeDetection:
  frontmatterKey: change
  folderMatch: true
  filenameMatch: true

archive:
  manifestFile: README.md
  requireResolvedActionItems: true
  moveMode: move
```

That’s enough for v1.

---

## Important behavior to include in scope

These are worth building in from the start.

### 1. Preserve directory structure

If a file lives at:

```text
plan/auth-session-hardening/phase-1.md
```

it should become:

```text
archive/auth-session-hardening/plan/auth-session-hardening/phase-1.md
```

or, if you want a cleaner target:

```text
archive/auth-session-hardening/plan/phase-1.md
```

I’d choose the second form if the change id is already the archive folder name.

### 2. Preserve internal markdown links

If design docs link to plan/debug/review docs, those links should still work after the move.

The easiest way to support that is to mirror the top-level doc kinds inside each archive entry.

### 3. Rewrite links to live docs when needed

If an archived plan links to a live `.spec.md` file via a relative path, that path may break after the move.

So `archive-change` should rewrite relative links that point to files outside the moved set.

That is worth including in v1 if your docs cross-link heavily.

### 4. Move linked local assets

If a selected markdown file references a local image or diagram inside an active dir, archive that asset too.

Otherwise historical docs decay quickly.

### 5. Fail closed on ambiguity

If `chum archive-change foo` matches too broadly or too loosely, it should stop and require `--include`.

That is much safer than “best guess” movement.

---

## How it should relate to your existing scripts

Do not rewrite everything first.

A practical migration path is:

### Phase 1

Wrap the current scripts behind a real CLI:

* `chum check`
* `chum archive-change`

### Phase 2

Consolidate shared code:

* config loading
* markdown parsing
* unresolved action detection
* change discovery
* link rewriting
* archive writing

### Phase 3

Publish it as a package:

* `@your-scope/chum`
* bin: `chum`

That gets you to installability quickly without pausing to over-architect.

---

## What not to build yet

I would explicitly not add these in v1:

* Git-specific storage models
* archive branches
* sister repos
* a custom archive search backend
* a database
* a GitHub-only action package
* a separate retrieval command

Because your strongest advantage right now is that **historical docs remain ordinary repo files**.

Agents and developers can just search the repo.

---

## One small addition I would make

I’d add a root `archive/README.md` created by `chum init`:

```md
# Archive

This directory contains historical change artifacts.

Search order:
1. live `*.spec.md`
2. active `design/`, `plan/`, `debug/`, `review/`
3. `archive/**` for historical context only
```

That single file makes the model obvious to both humans and agents.

---

## A good v1 definition of done

You’re done when all of this is true:

* `pnpm add -D @your-scope/chum && pnpm exec chum init` works in a conventional repo
* `chum check` ignores `archive/**`
* `chum archive-change <id> --dry-run` shows exactly what will move
* `chum archive-change <id>` moves only active change docs, not live docs
* each archive entry gets a markdown manifest with links to related live `.spec.md` files
* the same command can be called by GitHub Actions, local scripts, or a human directly

That’s the right level of ambition: one installable tool, one config file, one archive command, and a clean bridge between historical artifacts and current truth.

The next useful step is to turn this into a concrete CLI spec: exact flags, config schema, archive `README.md` frontmatter, and the archive/discovery algorithm.

