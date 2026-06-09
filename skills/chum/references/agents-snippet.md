## Documentation Workflow

This repo uses a filesystem-first documentation workflow.

There are two kinds of docs:

- `design/` docs capture problem framing, options, tradeoffs, and decisions
  before non-trivial architecture or product changes.
- `plan/` docs capture implementation scope, phases, acceptance criteria,
  rollout, and validation for substantial work.
- `debug/` docs capture reproduction steps, evidence, hypotheses, and proposed
  fixes for bugs or incidents.
- `review/` docs capture notes, findings, file maps, and analysis while
  reviewing an area or functionality.
- `*.spec.md` docs capture the current state of the codebase.

Intent and analysis docs describe what we plan to do, why we made a change, or
what we learned while reviewing an area. Spec docs describe what exists now.

### Spec Requirements

Every source file should have a corresponding `.spec.md` file.

A spec should explain:

- the file's purpose
- important exports, entry points, or behaviors
- dependencies and contracts
- how the file relates to nearby files
- known TODOs, unknowns, or verification gaps

Specs are current-state documentation. They are not plans, changelogs, or
design proposals.

When code changes, update the corresponding `.spec.md` file in the same change.
If a file is added, add its spec. If a file is deleted, remove its spec. If
behavior, dependencies, or contracts change, update the spec.

### Using chum

Use the `$chum` skill as tooling to support this workflow.

The agent is responsible for reading the code, understanding the repo context,
and writing the documentation. chum should be used to validate coverage, detect
stale or missing specs, normalize backmatter, and run final checks.

Recommended flow:

1. Write or update `design/`, `plan/`, `debug/`, or `review/` docs when the
   task calls for intent documentation or analysis notes.
2. Change the code.
3. Write or update the affected `.spec.md` files.
4. Use `$chum` to find missing, stale, invalid, or incomplete specs.
5. Use `$chum` to normalize spec backmatter.
6. Finish by running the chum check and resolving any failures.

Do not rely on chum to author the documentation. Use it as the verification and
formatting layer for the repo documentation workflow.
