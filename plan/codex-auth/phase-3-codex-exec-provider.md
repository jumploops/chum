# Phase 3: Codex Exec Provider

## Goal

Implement `CodexExecProvider`, a `ChumSwimProvider` implementation that delegates
generation to `codex exec` and lets Codex own authentication.

## Scope

In scope:

- `src/provider/codex.rs`
- command runner abstraction
- temporary output schema and result files
- stdin prompt execution
- JSON result parsing
- failure redaction
- tests with a fake command runner

Out of scope:

- resolver integration into `swim`
- auth status CLI
- app-server integration
- direct credential file access

## Implementation Notes

### Provider Config

`CodexExecProvider` needs:

- repo root
- Codex binary path
- optional model
- optional reasoning effort
- strict ChatGPT env behavior
- command runner

Keep the public constructor small:

```rust
impl CodexExecProvider {
    pub fn new(config: CodexExecConfig) -> Self;
}
```

### Command Runner

Use an injectable trait so tests do not execute real Codex:

```rust
pub trait CommandRunner {
    fn run(&self, command: CommandSpec) -> anyhow::Result<CommandOutput>;
}
```

`CommandSpec` should carry:

- binary path
- args
- cwd
- stdin
- env removals
- env additions if needed

`CommandOutput` should carry:

- exit status
- stdout
- stderr

### Invocation

Always pass:

```text
exec
--ephemeral
--skip-git-repo-check
--sandbox
read-only
--ask-for-approval
never
--output-schema
<schema.json>
--output-last-message
<result.json>
-C
<repo-root>
-
```

Add `--model <model>` when configured.

Do not add `--full-auto`.
Do not use write-enabled sandbox modes.
Do not put source text in argv.

### Result Parsing

Expected output file:

```json
{
  "markdown": "# Spec\n..."
}
```

Parser behavior:

- require a non-empty `markdown` string
- reject missing, null, array, or object values
- include file path context in parse errors
- optionally accept plain Markdown only if it can pass normal post-processing

### Temporary Files

Use a temp directory per provider request. It should contain:

- `schema.json`
- `result.json`

Clean up automatically after each request. Tests may keep temp dirs only when
explicitly configured for debugging.

### Redaction

On command failure:

- include exit status
- include stderr tail, capped to a small byte or line count
- redact token-like strings and any `KEY=...`/`TOKEN=...` values
- do not include stdin prompt
- do not include full source text

## Acceptance Criteria

- [ ] Fake command runner sees stdin prompt, not source argv.
- [ ] Command args include `--ephemeral`, `--skip-git-repo-check`,
      `--sandbox read-only`, and `--ask-for-approval never`.
- [ ] Command args include `--output-schema` and `--output-last-message`.
- [ ] Configured model is passed as `--model`.
- [ ] Strict ChatGPT mode removes direct API-key env vars from child env.
- [ ] Valid JSON result returns `SpecDraft`.
- [ ] Missing `markdown` fails clearly.
- [ ] Command failure reports status and redacted stderr.
- [ ] No test needs a real Codex login.

## Dependencies

- Phase 2 shared prompt builder.
