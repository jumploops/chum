# AGENTS.md

> How humans and code assistants operate in this repository while building `<PROJECT_NAME>`.
> This document complements the architectural source of truth: [`<ROOT_SPEC_FILE>`](./<ROOT_SPEC_FILE>).

---

## 0) How To Adapt This Template

- Replace all `<PLACEHOLDER>` values with repo-specific names, paths, commands, and contracts.
- Keep the process rules strict even if you shorten examples.
- If this repo uses phased implementation plans, keep that structure explicit rather than collapsing everything into one generic plan section.
- Delete sections that truly do not apply, but do not remove the distinction between:
  - intent docs created before code changes
  - current-state `*.spec.md` docs maintained with code changes

---

## 1) Project Overview

`<PROJECT_NAME>` is a `<one-line-description>`.

Top-level areas:

- `<path-1>/`: `<purpose>`
- `<path-2>/`: `<purpose>`
- `<path-3>/`: `<purpose>`

Core concepts:

- `<concept-1>`
- `<concept-2>`
- `<concept-3>`

For full architectural details, see [`<ROOT_SPEC_FILE>`](./<ROOT_SPEC_FILE>).

---

## 2) Documentation System

This repository uses two complementary documentation types:

| Doc Type | Purpose | Timing |
|----------|---------|--------|
| `design/` docs | Capture problem framing, alternatives, tradeoffs, and the proposed shape of a change | Before non-trivial changes |
| `plan/` docs | Capture implementation scope, acceptance criteria, rollout, validation, and phased execution details | Before substantial work |
| `debug/` docs | Capture reproduction, evidence, hypotheses, and proposed fixes | Before bug fixes |
| `*.spec.md` docs | Capture the current state of code, folders, responsibilities, dependencies, and technical debt | Updated while code changes are made |

### 2.1) Intent Docs vs Current-State Docs

Use the docs for different jobs:

- `design/`, `plan/`, and `debug/` explain intent.
- `*.spec.md` explains what exists now.

The workflow is:

1. Write or update the relevant intent doc before implementation.
2. Modify code.
3. Update the affected `*.spec.md` files so the documented current state matches the code that shipped.

For larger features, the intent flow is usually:

1. Write `design/<topic>.md` to capture the problem, options, and decision.
2. Split execution into `plan/<topic>/` with an `implementation-spec.md` plus one or more `phase-N-<step>.md` files.
3. Implement incrementally.
4. Update the affected `*.spec.md` files alongside the code.

### 2.2) Spec File Structure

Every folder with source files or other important implementation artifacts should have a `folder-name.spec.md` file that summarizes:

- folder purpose and responsibilities
- file descriptions and key exports
- child folder references
- dependencies and contracts
- TODOs, unknowns, and technical debt

Example:

```text
repo/
├── <ROOT_SPEC_FILE>
├── <area-a>/
│   ├── <area-a>.spec.md
│   └── src/
│       └── src.spec.md
└── <area-b>/
    ├── <area-b>.spec.md
    └── ...
```

### 2.3) Phased Plan Structure

When a change is large enough to need staged execution, use a dedicated plan folder instead of one oversized plan doc.

Example:

```text
design/
└── new-feature-abc.md

plan/
└── new-feature-abc/
    ├── implementation-spec.md
    ├── phase-0-preflight.md
    ├── phase-1-initial-change.md
    ├── phase-2-follow-up.md
    ├── progress-checklist.md
    └── validation-checklist.md
```

Guidance:

- `design/<topic>.md` explains the why and the chosen shape.
- `plan/<topic>/implementation-spec.md` explains the overall rollout.
- `phase-N-*.md` files break the rollout into concrete, reviewable increments.
- checklists track execution and validation without overloading the design doc.

### 2.4) When To Update Specs

You must update the relevant spec file(s) when you:

| Change Type | Spec Update Required |
|-------------|---------------------|
| Add a new file | Add it to the parent folder spec |
| Delete a file | Remove it from the parent folder spec |
| Add a new folder | Create a new folder spec and update the parent spec |
| Change a file's purpose or API | Update its description in the spec |
| Change important dependencies | Update the dependencies/contracts section |
| Add or remove technical debt | Update TODO markers or notes |
| Change architecture or flow | Update the relevant spec narrative and diagrams |

### 2.5) Spec Markers

Use markers so unresolved documentation work is searchable:

| Marker | Usage |
|--------|-------|
| `<!-- SPEC:TODO -->` | Known debt, incomplete work, or follow-up needed |
| `<!-- SPEC:UNKNOWN -->` | Undocumented behavior that needs later review |
| `<!-- SPEC:VERIFY -->` | Assumption that should be validated |

Find all markers:

```bash
grep -rn "SPEC:\(UNKNOWN\|TODO\|VERIFY\)" --include="*.spec.md" .
```

---

## 3) Repo Layout

| Path | Purpose | Notes |
|------|---------|-------|
| `<path>` | `<purpose>` | `<language/framework/runtime>` |
| `<path>` | `<purpose>` | `<language/framework/runtime>` |
| `<path>` | `<purpose>` | `<language/framework/runtime>` |
| `docs/` | Shared documentation | Intent and reference docs |
| `design/` | Design docs | Pre-implementation reasoning |
| `plan/` | Implementation plans | Scope, rollout, validation |
| `debug/` | Debug notes | Repro, evidence, hypotheses |

Key docs:

- [`<ROOT_SPEC_FILE>`](./<ROOT_SPEC_FILE>) — architectural source of truth
- [`AGENTS.md`](./AGENTS.md) — operating procedures
- [`<CONTRACT_DOC_1>`](./<CONTRACT_DOC_1>) — `<purpose>`
- [`<PLAN_DOC_1>`](./<PLAN_DOC_1>) — `<purpose>`

---

## 4) Operating Rules

### 4.1) Read Specs First

Before modifying a folder, read its spec file first.

This should tell you:

- what the code does and why
- what files exist and their roles
- known issues or debt
- important dependencies and contracts

### 4.2) Write Intent Docs Before Code

Before changing code:

- create or update a `design/` doc for non-trivial design or architecture work
- create or update a `plan/` doc for substantial implementation work
- create or update a `debug/` doc before fixing a bug or incident

Each code change should be traceable back to written intent.

For multi-step features:

- start with `design/<topic>.md`
- create `plan/<topic>/implementation-spec.md`
- split execution into `phase-N-*.md` plan docs when the work is staged, risky, or spans multiple merges/deploys

### 4.3) Update Specs While Changing Code

As you change code, keep the affected `*.spec.md` files current.

Intent docs are not a substitute for specs:

- intent docs say what you plan to do
- specs say what the repository currently is

### 4.4) Read Files In Full

Do not rely on partial snippets when analyzing or editing files. Read full files before making changes. If tooling limits prevent that, ask for the full content.

### 4.5) Build/Run Failures

If an authoritative build, test, or run command fails and the next step is unclear:

- record the exact command
- record the exact error output
- add or update a `debug/` note if the issue matters to the task
- stop and escalate to a human unless the repository already documents the next fallback

### 4.6) Contract Changes Require Docs

If you change any boundary contract, update the matching documentation in the same change:

| Change Type | Docs To Update |
|-------------|----------------|
| API or RPC shapes | `<api-doc-path>` |
| Events, streams, or queues | `<event-doc-path>` |
| Database or storage schema | schema source, schema docs, and rollout docs |
| Auth or ownership rules | auth/ownership design docs and relevant specs |
| CLI/config/env behavior | operator docs, README, or setup docs |

### 4.7) Define Ownership And Permissions Up Front

Before adding or changing any user-facing route, stream, loader, job, or shared data read/write path, explicitly identify:

- who owns the resource
- how the acting user or system identity is resolved
- where authorization happens before data is read, written, or streamed
- which records must be stamped with creator or acting identity

Do not add temporary global reads or writes for convenience.

---

## 5) Core Contracts (Do Not Break)

Fill this section with the repository's hard invariants. Typical categories:

### 5.1) External Interfaces

- `<API versions, event envelopes, protocol fields, file formats, public SDK expectations>`

### 5.2) State / Lifecycle Models

- `<session, job, workflow, or state-machine invariants>`

### 5.3) Tooling / Runtime Contracts

- `<background jobs, agents, workers, queues, CLIs, scheduled tasks>`

### 5.4) Data Model Invariants

- `<identity rules, foreign-key assumptions, audit fields, timestamp rules, tenancy rules>`

### 5.5) Auth / Ownership Boundaries

- `<viewer scoping, 401 vs 403 vs 404 rules, shared-resource rules, actor stamping>`

---

## 6) Task Templates

### `design/` Template

```markdown
# Design: <short-title>

## Context
- Problem statement:
- Constraints:
- Related docs/specs:

## Goals
- Desired outcomes:

## Non-Goals
- Explicitly out of scope:

## Options Considered
- Option A:
- Option B:

## Decision
- Chosen approach:
- Why:

## Risks
- Risk:
- Mitigation:

## Follow-Up
- Implementation plan:
- Specs/docs to update:
```

### `plan/` Template

```markdown
# Plan: <short-title>

## Context
- Link to issue(s):
- Related docs/specs:

## Objective
- Desired outcome and acceptance criteria.

## Design / Approach
- Summary of changes.
- Risks and mitigations.

## Spec Files To Update
- [ ] List each spec file that will need changes

## Impacted Contracts
- [ ] APIs / RPC
- [ ] Events / streams
- [ ] Schema / storage
- [ ] Jobs / tools / workers
- [ ] UI / UX

## Test Plan
- Unit / integration / end-to-end outline.

## Rollout
- Migration or deploy steps if any.
- Docs to update.
```

### Phased `plan/` Layout

Use a directory when a feature needs staged execution:

```text
plan/<topic>/
├── implementation-spec.md
├── phase-0-<preflight>.md
├── phase-1-<initial-change>.md
├── phase-2-<next-change>.md
├── progress-checklist.md
└── validation-checklist.md
```

Suggested roles:

- `implementation-spec.md`: overall objective, architecture, risks, and rollout strategy
- `phase-N-*.md`: concrete scoped increments with acceptance criteria
- `progress-checklist.md`: execution tracking
- `validation-checklist.md`: release or signoff validation

### `plan/<topic>/implementation-spec.md` Template

```markdown
# Implementation Spec: <topic>

## Context
- Related design doc:
- Related specs/docs:

## Objective
- End-state:
- Acceptance criteria:

## Phase Breakdown
- Phase 0:
- Phase 1:
- Phase 2:

## Cross-Cutting Risks
- Risk:
- Mitigation:

## Docs / Specs To Update
- [ ] Item
```

### `plan/<topic>/phase-N-*.md` Template

```markdown
# Phase <N>: <short-title>

## Goal
- What this phase changes

## Scope
- In scope:
- Out of scope:

## Implementation Notes
- Key codepaths:
- Contract/doc impacts:

## Acceptance Criteria
- [ ] Criterion

## Dependencies
- Prior phases or blockers:
```

### `debug/` Template

```markdown
# Debug: <short-title>

## Environment
- OS / arch / versions:
- Runtime mode:
- Relevant services/dependencies:

## Repro Steps
1. ...

## Observed
- Logs, traces, screenshots, metrics

## Expected
- What should have happened

## Hypotheses
- Root cause candidates

## Proposed Fix
- Minimal patch outline
- Specs/docs affected
```

### `*.spec.md` Template

```markdown
# <folder-name>

Brief description of this folder's role.

## Files

### `<file-name>`
- Purpose:
- Key exports or entry points:
- Important dependencies:

## Subfolders

### `<subfolder>/` -> [`<subfolder>.spec.md`](./<subfolder>/<subfolder>.spec.md)
- Purpose:

## Dependencies / Contracts

- External dependencies:
- Internal dependencies:
- Important contracts:

## TODOs / Technical Debt

- [ ] Item
```

---

## 7) Build, Test, And Local Run

Document only the commands contributors actually need.

- Build: `<build command>`
- Test: `<test command>`
- Run locally: `<run command>`

Rules:

- run package-local commands from the owning package or module directory
- do not assume repo-root command resolution for package-local tools
- if schema changes exist, document the exact schema update workflow here

---

## 8) Code Conventions

- `<language>`: `<formatter/linter/testing rules>`
- `<language>`: `<style rules>`
- Naming: `<ID strategy, case conventions, public-field conventions>`
- Errors: `<canonical error codes or error-handling policy>`
- Logging/metrics: `<required conventions>`

---

## 9) Definition Of Done

A task is complete when:

- [ ] code, docs, and specs agree
- [ ] relevant `design/`, `plan/`, or `debug/` docs exist and are current
- [ ] affected `*.spec.md` files are updated
- [ ] tests were added or updated where appropriate
- [ ] contract docs were updated for any boundary changes
- [ ] schema/storage rollout steps were applied or documented
- [ ] no new TODOs were introduced without explanation

---

## 10) Quick Reference

Find spec files:

```bash
find . -name "*.spec.md" -type f
```

Find spec markers:

```bash
grep -rn "SPEC:\(UNKNOWN\|TODO\|VERIFY\)" --include="*.spec.md" .
```

Read the root architecture doc:

```bash
cat <ROOT_SPEC_FILE>
```

---

## 11) Related Documentation

| Document | Purpose |
|----------|---------|
| [`<ROOT_SPEC_FILE>`](./<ROOT_SPEC_FILE>) | Architectural source of truth |
| [`AGENTS.md`](./AGENTS.md) | Operating procedures for humans and code assistants |
| [`<DOC_PATH>`](./<DOC_PATH>) | `<purpose>` |
| [`<DOC_PATH>`](./<DOC_PATH>) | `<purpose>` |

---

*Last updated: <YYYY-MM-DD>*
