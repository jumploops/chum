# `src/commands/check.rs`

## Purpose

Implementation of `chum check`. It validates source/spec coverage, chum backmatter, source hashes, unresolved work lists, and legacy SPEC markers.

## Key Exports

- `CheckReport` - JSON-serializable summary of root, source counts, ignored count, failures, and warnings.
- `CheckFailure` - Path/message pair for a validation issue.
- `run_report` - Performs validation and returns a report without exiting.
- `print_report` - Human-readable check summary.
- `print_report_json` - JSON check output.

## Dependencies / Contracts

- Missing file and directory specs are failures.
- File specs must have `kind: file`, matching `target`, and current `source_hash` unless `--allow-stale` is used.
- Directory specs must have `kind: directory` and matching `target`.
- `todo` and `unknowns` must always be empty for success.
- `verify` must be empty unless `--allow-external-verify` is supplied.
- Legacy SPEC marker comments for TODO, unknown, and disallowed verify work are failures.

<!-- chum:backmatter
schema: 1
kind: file
target: src/commands/check.rs
source_hash: sha256:d112bdb4e4ffbeaebdc45d6fb6f42247cf6731cb6d80d9515509da292b61923b
source_updated_at: 2026-04-24T01:32:14.343689016Z
spec_updated_at: 2026-04-24T01:35:55.616614Z
generated_by: chum swim --stubs
todo: []
unknowns: []
verify: []
-->
