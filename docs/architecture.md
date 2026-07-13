# Architecture

Forall is a standalone coding agent distributed as a single Rust binary
(`forall`). Its primary user experience does not require Cursor or manual MCP
configuration.

This repository publishes the **open-source adapter crates** that the binary
embeds. The full agent runtime (TUI, turn loop, sandbox, native tool
registry) remains closed source and is distributed only as the `forall`
executable.

```text
┌────────────────────────────────────────────────────────────┐
│  `forall` binary (closed-source distribution)              │
│  agent TUI · native tools · sandbox · workflow commands    │
└────────────────────────────┬───────────────────────────────┘
                             │
                             ▼
┌────────────────────────────────────────────────────────────┐
│  native author.*   deterministic workspace operations      │
│  native verify.*   hosted submit / status / explain        │
│  (call open-source libraries in-process)                   │
└───────────────┬───────────────────────────┬────────────────┘
                │                           │
                ▼                           ▼
┌──────────────────────────────┐  ┌───────────────────────────┐
│ Project workspace            │  │ Hosted Forall MCP         │
│ .forall/ + application code  │  │ isolated verification     │
└──────────────────────────────┘  └───────────────────────────┘
```

The native `author.*` and hosted `verify.*` registrations implement the
standalone architecture. They share the reusable authoring library and hosted
wire client with the optional external adapters published in this repository.

External coding agents use optional adapters around the same capabilities:

```text
Cursor / Claude Code / Codex
    ├─ local authoring MCP (stdio) ──► project workspace
    └─ hosted Forall MCP (HTTP) ─────► isolated verification
```

Standalone Forall calls the authoring library directly. It must not spawn or
connect to its own local MCP server.

The two MCP servers are distinct:

- [Local authoring MCP](local-authoring-mcp.md): optional workspace-bound stdio
  adapter for external agents.
- [Hosted Forall MCP](hosted-mcp.md): remote asynchronous verification service.

## Open-source components

| Component | Location | Role |
| --- | --- | --- |
| Authoring library | `crates/forall-authoring` | Safe status, initialization, discovery, mapping, contracts, and validation |
| Hosted client | `crates/forall-hosted-verify` | Safe snapshot packaging and authenticated submit, status, cancellation, and explanation |
| Local MCP adapter | `crates/forall-mcp-author` | Optional stdio authoring tools for external MCP clients |

See [crates/README.md](../crates/README.md) for crate-level details.

## Standalone tool surfaces

### Native `author.*` tools

Standalone Forall registers native authoring handlers that call
`forall-authoring` in-process:

- `author.status`
- `author.init`
- `author.discover`
- `author.upsert_requirements`
- `author.scaffold_contracts`
- `author.validate`
- `author.scaffold_property`

Apply-mode tools retain preview, expected-hash, root-confinement, and atomic
write guarantees. Application-code writes must participate in the agent's
approval and VerifiedGate policies.

### Native `verify.*` tools

Standalone Forall exposes hosted verification without requiring users to
configure MCP:

- `verify.submit`
- `verify.status`
- `verify.cancel`
- `verify.explain`

The standalone client owns API-key onboarding (`forall verification login`),
safe workspace snapshot packaging, job operations, and sanitized results.

## External MCP surfaces

The external adapters remain useful for users who choose another coding agent.
They are not prerequisites for using standalone Forall.

```text
forall mcp-author --root <path>   # local authoring stdio adapter
https://mcp.forall.astrio.app/mcp # hosted verification endpoint
```

## Distribution and proof toolchains

The installer distributes one `forall` executable. That executable contains:

- the standalone agent and native `author.*` / `verify.*` registrations;
- the local `mcp-author` stdio adapter for external agents;
- workflow, authoring, snapshot, and toolchain-management logic;
- compile-time embedded schemas, templates, and skills.

It does not contain every language proof engine. Toolchain ownership depends on
the verification path:

```text
Standalone or external agent using hosted verification
    local machine: forall binary + project files
    hosted worker: language proof toolchains

User running local `forall check`
    local machine: forall binary + required language proof toolchains
```

Installing Forall therefore does not install all proof engines immediately.
For local checks, `forall ensure` detects project languages and performs
best-effort setup of the language proof engines required by each project.
Hosted-verification users do not run these commands because the worker image
owns those dependencies.

## Hooks

`forall init` merges hook definitions into `~/.forall/hooks.json` (user home, never the repo). Commands are portable (`forall hook <event>` resolved on PATH).

| Hook         | Behavior                                                             |
| ------------ | -------------------------------------------------------------------- |
| SessionStart | Injects active change + workflow status into agent context           |
| PreToolUse   | Blocks application-code writes until planning artifacts are complete |
| PostToolUse  | Runs `forall sync` after TypeScript edits; records browser evidence  |
| Stop         | Runs `forall check`; blocks completion on CRITICAL failures          |

## Project layout

`forall init` writes only shareable files into the repo:

```text
.forall/
├── markers.toml            # project root markers for the agent
├── workflow/
│   ├── config.yaml         # workflow config
│   ├── active              # active change pointer
│   ├── changes/<name>/     # proposal, specs, design, verification, tasks
│   └── archive/            # completed changes
├── verify/
│   ├── mapping.yaml        # requirement → code mapping (project marker)
│   └── cache/              # check reports, intent, waivers
├── specs/                  # requirements (SHALL/MUST + scenarios)
└── scenarios/              # executable scenario / property tests
```

Machine-local state (hook registration, project trust, and managed local
toolchains) lives in `~/.forall/`. Language-to-backend selection is built into
the binary; the backend executables themselves are not.

## Check pipeline

`forall check` runs six phases: structure → mapping → proofs → intent → scenarios → scenario-tests. Reports land in `.forall/verify/cache/reports/` with a verification tier (`N proved, M spec-tracked`). Exit codes: `0` pass, `1` CRITICAL, `2` warnings.

## Spec workflow

```text
proposal → specs → design → verification → tasks → apply → archive
```

Later steps are gated on earlier ones; the PreToolUse hook enforces the gate, and `archive` merges change specs and `mapping.delta.yaml` into project truth.
