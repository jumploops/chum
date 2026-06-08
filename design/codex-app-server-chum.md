# Design: chum On Codex App Server

## Context

`chum` has just moved from a Rust/npm CLI into a skill-first shape:

- `SKILL.md` gives agents the workflow.
- `scripts/chum.py` is a standard-library Python processor for deterministic
  repository mechanics.
- The active agent session reads related code, plans a route, writes specs, and
  uses the script for `targets`, `normalize`, `validate`, `check`, `init`, and
  `archive`.
- The script intentionally does not call an LLM.

This design rethinks whether `chum` should build on the Codex app-server instead
of, or alongside, the current Python/skill approach.

## Research Summary

Sources inspected:

- Official Codex manual, fetched June 8, 2026 via the OpenAI docs skill.
- Codex app-server docs: <https://developers.openai.com/codex/app-server>
- Codex SDK docs: <https://developers.openai.com/codex/sdk>
- Codex non-interactive docs: <https://developers.openai.com/codex/noninteractive>
- Open-source app-server implementation/docs:
  <https://github.com/openai/codex/tree/main/codex-rs/app-server>
- App-server README:
  <https://github.com/openai/codex/blob/main/codex-rs/app-server/README.md>
- Codex MCP/app-server interface notes:
  <https://github.com/openai/codex/blob/main/codex-rs/docs/codex_mcp_interface.md>

Key facts:

- `codex app-server` is the JSON-RPC interface used by rich Codex clients such
  as the VS Code extension.
- It supports stdio, Unix-socket WebSocket, and experimental TCP WebSocket
  transports.
- Its core primitives are threads, turns, and items. Threads persist
  conversation history; turns stream incremental agent work; items represent
  messages, reasoning, command execution, file changes, tool calls, and related
  events.
- The lifecycle is initialize once, start or resume a thread, start a turn, read
  notifications until `turn/completed`, and optionally steer or interrupt
  in-flight turns.
- It exposes account/auth RPCs such as `account/read` and
  `account/login/start`, including ChatGPT-managed auth flows.
- It exposes command execution APIs such as `command/exec` and file APIs such as
  `fs/readFile`, plus approval requests for commands and file changes.
- The app-server can generate TypeScript or JSON Schema artifacts matching the
  currently installed Codex version.
- The official docs say app-server is for deep product integrations with auth,
  conversation history, approvals, and streamed events. They also say
  automation/CI should generally use the Codex SDK instead.
- The CLI reference marks `codex app-server` as experimental and primarily for
  local development or debugging, so method names and schemas may change.
- The Python Codex SDK controls the local app-server over JSON-RPC, requires
  Python 3.10+, and published builds include a pinned Codex CLI runtime.

## Current Approach

The current `chum` approach is:

```text
Agent session
  reads repo context
  plans route
  writes specs
  runs scripts/chum.py

scripts/chum.py
  deterministic discovery
  spec path mapping
  backmatter normalization
  validation
  init/archive mechanics
```

Strengths:

- No nested agent process.
- No Codex CLI or app-server dependency.
- Works as a portable skill with `uv run` or `python3`.
- Keeps deterministic behavior easy to test.
- Lets the current agent session retain shared repo context naturally.
- Avoids implementing approval/event streaming UI inside `chum`.

Weaknesses:

- Only works well when already invoked by a capable agent.
- No programmatic access to Codex conversation history beyond the active
  session.
- No standard way to drive Codex from outside Codex itself.
- No structured bridge for a non-Codex client that wants to run the full
  `chum` workflow.
- The deterministic script currently cannot ask the agent to continue, steer,
  or recover in a machine-controlled loop.

## App-Server-Based Approach

An app-server-based `chum` would add a client that starts or connects to
`codex app-server`, creates or resumes a thread, and drives turns for repo spec
maintenance.

Potential flow:

```text
scripts/chum.py targets --json
        |
        v
chum app-server client
  initialize
  thread/start or thread/resume
  turn/start with target report and workflow prompt
  stream item/agentMessage/delta, item/fileChange/*, command output
  respond to approval requests
  run deterministic validations
  turn/steer with remaining target report
        |
        v
scripts/chum.py check --json
```

This gives `chum` a programmatic way to control a persistent Codex thread. The
key gain is not "LLM calls from the script"; it is a durable orchestration
surface for a client that wants Codex to do the spec-writing work while
retaining context across multiple turns.

## Comparison

| Dimension | Current Python Skill | App-Server Integration |
|---|---|---|
| Install path | Simple skill folder, standard-library Python | Requires Codex CLI/app-server or Codex SDK |
| Agent context | The active agent session owns context | A controlled app-server thread owns context |
| Determinism | Strong for repo mechanics | Strong only if `scripts/chum.py` remains the validator |
| Portability | High | Lower; tied to Codex runtime/version |
| Non-Codex use | Weak | Stronger |
| CI/headless use | Script can validate, but cannot author specs | Possible, but docs recommend SDK over app-server for automation |
| Approval handling | Native to current Codex session | Client must implement approval request handling |
| UX | Agent-native | Could support rich progress UI/event stream |
| API stability | Our script API is ours | App-server is experimental and versioned by installed Codex |
| Auth | No auth needed in script | Can use Codex account APIs, but must avoid owning secrets |

## Architecture Options

### Option A: Keep Skill-Only

Keep the current approach. `scripts/chum.py` stays deterministic and
LLM-free. The skill instructs Codex how to use it.

Best for:

- Codex-native interactive use.
- Fast stabilization.
- Low maintenance.

Limitations:

- Does not solve non-Codex orchestration.
- Does not expose a reusable controller for other products.

### Option B: Add App-Server Client As Optional Experiment

Add a separate optional script:

```text
scripts/chum_appserver.py
```

Responsibilities:

- Start `codex app-server` over stdio by default.
- Initialize with `clientInfo.name = "chum_skill"` or similar.
- Generate/read local app-server JSON Schema for the installed Codex version.
- Create an ephemeral thread for experiments or resume a named thread.
- Send `targets --json` output and workflow instructions in `turn/start`.
- Stream final agent output and relevant progress.
- Stop at approval requests in v1 instead of auto-approving.
- Run `scripts/chum.py check --json` after the turn.

The deterministic script remains canonical. The app-server client is a research
surface, not the default skill path.

Best for:

- Proving whether app-server gives better end-to-end repo context.
- Building a future standalone `chum` runner.
- Learning app-server protocol without risking the skill workflow.

Limitations:

- Requires Codex installed and authenticated.
- Needs robust event parsing.
- Needs approval handling design.
- Uses an experimental API.

### Option C: Use The Codex Python SDK Instead Of Raw App-Server

Add an optional SDK-backed script with PEP 723 dependencies:

```python
# dependencies = ["openai-codex>=..."]
```

The SDK wraps the local app-server and includes a pinned Codex runtime in
published builds. It can start threads, run prompts, and pass sandbox presets.

Best for:

- Avoiding hand-rolled JSON-RPC.
- Building an external automation/client path.

Limitations:

- Requires Python 3.10+.
- Adds dependency download/install behavior to a skill that is currently
  standard-library only.
- The SDK abstraction may not expose every event or approval surface we need.
- A pinned runtime can diverge from the user's installed Codex app/CLI.

### Option D: Build A Full Rich Client

Build a dedicated product around app-server: UI, progress stream, approvals,
thread history, target queue, validation panel, and archive controls.

Best for:

- A first-class human-facing chum app.
- Teams that want auditability and progress visualization.

Limitations:

- Much larger product.
- Requires app-server compatibility work, auth UX, and security review.
- Not justified before proving Option B.

## Recommendation

Do not replace the current deterministic Python/skill approach with app-server
right now.

Instead:

1. Keep `scripts/chum.py` as the stable source of truth for repository
   mechanics.
2. Keep `SKILL.md` as the primary Codex-native UX.
3. Add an optional app-server spike only if we want `chum` to operate outside an
   already-running Codex agent session.

The app-server path is promising, but it solves a different problem than the
current skill. The current skill is best when a Codex agent is already active.
App-server is best when another client needs to create, resume, stream, and
control Codex work programmatically.

## Proposed Target Architecture

Short term:

```text
SKILL.md
scripts/chum.py          # stable deterministic substrate
references/*.md
tests/test_chum_script.py
```

Experimental addition:

```text
scripts/chum_appserver.py
references/app-server.md
tests/test_chum_appserver_protocol.py
schemas/app-server/      # generated, versioned, optional
```

Boundary:

- `scripts/chum.py` never imports app-server code.
- `scripts/chum_appserver.py` shells out to `scripts/chum.py` or imports a
  small internal module only after we split the script.
- Validation always goes through `scripts/chum.py`.
- App-server never becomes required for `check`, `targets`, `normalize`,
  `validate`, `init`, or `archive`.

## App-Server Spike Design

### Command Shape

```bash
uv run scripts/chum_appserver.py maintain --root . --dry-run
uv run scripts/chum_appserver.py maintain --root . --write
uv run scripts/chum_appserver.py auth-status
uv run scripts/chum_appserver.py schema --out schemas/app-server
```

### Transport

Start with stdio:

```bash
codex app-server
```

Reasons:

- No port management.
- No WebSocket auth.
- No remote exposure.
- Matches the quickstart path.

Unix socket or WebSocket can wait until there is a real multi-client need.

### Thread Strategy

Use ephemeral threads for the first spike:

- Avoid polluting the user's persistent Codex thread list.
- Make tests easier.
- Reduce lifecycle cleanup.

Later, add `--resume-thread <id>` or `--thread-id <id>` if persistent history is
valuable.

### Prompt Strategy

The app-server client should not send one prompt per file. It should send:

- current `targets --json`
- a concise summary of the chum workflow
- instructions to inspect related files adaptively
- instructions to run `normalize` and `validate`
- instructions to stop and report if approval is needed

Subsequent turns should send only the remaining target report plus validation
failures.

### Approval Policy

V1 should not auto-approve file changes or commands.

When app-server sends command or file-change approval requests:

- In `--dry-run`, decline and ask the agent to produce a plan.
- In `--write`, either stop with an actionable message or require a future
  explicit `--approval-mode` flag.

Approval handling is a major product/security boundary and should not be hidden
inside a convenience script.

## Risks

### App-Server API Stability

The official docs mark app-server as experimental. The CLI can generate schemas
for the installed Codex version, but a script still has to handle version drift.

Mitigation:

- Generate JSON Schema in a spike.
- Keep app-server code optional.
- Gate it behind an explicit command name and document version requirements.

### Nested Agent Confusion

Running an app-server-driven Codex agent from inside an active Codex agent can
create a nested-agent workflow where logs, approvals, and file edits are hard to
reason about.

Mitigation:

- Keep the current skill path as default.
- Position app-server mode for external clients or explicit experiments.
- Do not invoke app-server automatically from `SKILL.md`.

### Approval And Security Semantics

App-server asks the client to handle approvals. A thin script that blindly
accepts approvals would bypass the human/agent review boundary.

Mitigation:

- Do not auto-approve in v1.
- Surface approval requests as structured output.
- Add a deliberate approval-mode design before enabling writes.

### Skill Portability

The current skill works with Python only. App-server requires Codex installed,
authenticated, and compatible.

Mitigation:

- Keep `scripts/chum.py` dependency-free.
- Make app-server support optional.
- Keep CI focused on deterministic script tests.

### State And Cleanup

Persistent threads can accumulate. Threads may appear in Codex clients. Users
may not expect a script to create durable sessions.

Mitigation:

- Use ephemeral threads first.
- Require explicit flags for persisted thread use.
- Include thread IDs in output.

### Protocol Coverage

To build a robust client we need to handle initialize, initialized,
thread/start, turn/start, notification streaming, errors, turn/completed,
server-initiated approval requests, command output, and file-change events.

Mitigation:

- Start with read-only/dry-run.
- Add focused protocol tests using fake JSON-RPC transcripts.
- Defer full write approval handling.

## Migration Path

### Phase 0: Keep Current Skill Stable

- Do not change `scripts/chum.py`.
- Keep current skill docs and tests passing.
- Add this design proposal only.

### Phase 1: App-Server Protocol Probe

- Add `references/app-server.md` with the exact supported protocol subset.
- Add `scripts/chum_appserver.py auth-status`:
  - start `codex app-server` over stdio
  - send `initialize`
  - send `initialized`
  - call `account/read`
  - exit without starting a thread
- Add fake-transcript tests for initialize/account handling.

### Phase 2: Read-Only Thread Spike

- Add `maintain --dry-run`.
- Start an ephemeral thread.
- Send `targets --json` and workflow instructions.
- Stream the final message.
- Do not allow file writes.
- Run final `scripts/chum.py check --json`.

### Phase 3: Validation Loop

- Let the app-server agent run deterministic commands through normal Codex tool
  use or ask the outer client to run `scripts/chum.py`.
- Feed validation failures back with `turn/start` or `turn/steer`.
- Stop after a bounded number of turns.

### Phase 4: Write Mode Design

- Design approval handling separately.
- Decide whether the client can accept file-change approvals, command approvals,
  or neither.
- Add explicit `--approval-mode` flags only after review.

### Phase 5: Decide Product Direction

After the spike, choose one:

- Keep app-server support experimental.
- Promote it to the main non-Codex runner.
- Drop it if the complexity does not improve outcomes.

## Concrete Next Steps

1. Add `references/app-server.md` summarizing the protocol subset `chum` would
   use.
2. Add a small fake JSON-RPC transcript test harness. Do this before talking to
   a real app-server.
3. Implement `scripts/chum_appserver.py auth-status` over stdio.
4. Validate against local `codex app-server` manually.
5. Add `maintain --dry-run` with ephemeral thread creation and final-message
   streaming.
6. Re-evaluate after one real repo run.

## Open Questions

- Should app-server mode be part of the skill artifact, or a separate
  experimental plugin?
- Should app-server mode use raw JSON-RPC or the Python Codex SDK?
- Can we safely test app-server in CI without Codex credentials?
- How should approval requests be represented in script output?
- Should persisted app-server threads be allowed, and if so how should `chum`
  name and archive them?
- Does app-server mode materially improve spec quality compared with a normal
  active Codex skill session?

## Decision Record

For now:

- Keep the current Python/skill approach as primary.
- Treat Codex app-server as an optional research path for external or richer
  clients.
- Do not add app-server as an automatic dependency of `SKILL.md`.
- Do not implement write-mode app-server automation until approval handling has
  its own design.
