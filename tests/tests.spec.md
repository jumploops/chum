# `tests/`

## Purpose

Python regression tests for the `chum` skill processor.

## Files

- `test_chum_script.py` - End-to-end tests for help output, target discovery,
  checking, normalization, focused validation, ignore handling, init, and
  archive behavior.

## Dependencies / Contracts

- Tests use the Python standard library and run with
  `python3 -m unittest discover tests`.
- The tests execute `scripts/chum.py` as a subprocess to cover the same command
  surface agents use.

<!-- chum:backmatter
schema: 1
kind: directory
target: tests
spec_updated_at: 2026-04-28T00:00:00Z
generated_by: chum skill
children: []
todo: []
unknowns: []
verify: []
-->
