# Local Authoring MCP

## Purpose

The local authoring MCP lets external coding agents use Forall's deterministic
authoring operations inside a user's workspace. It is an optional integration
surface for Cursor, Claude Code, Codex, and other MCP clients.

It is not the primary integration path for the standalone Forall agent. The
standalone agent calls the same authoring library directly through native
`author.*` tools without starting an MCP subprocess.

## Installation and dependency boundary

The local MCP is part of the `forall` executable. Users do not install a
separate authoring server:

```bash
curl -fsSL https://forall.astrio.app/install.sh | bash
forall mcp-author --root "$PWD" --print-config
```

An MCP client uses the printed command to launch `forall mcp-author` locally.
The marketplace or MCP client does not host this process; it runs with the
user's OS permissions on the user's machine.

The authoring adapter does not install or run language proof engines. It only
performs confined workspace reads and deterministic edits. Consequently, a
Cursor, Claude Code, or Codex user who authors locally and verifies through the
hosted MCP needs:

- the `forall` executable for the local adapter;
- a Forall API key for the hosted service;
- no local language proof engine installation.

Those toolchains are relevant only when the user chooses local
`forall check`; see [Architecture](architecture.md#distribution-and-proof-toolchains).

## Architecture

```text
Standalone Forall
    │
    │ native author.* tools
    ▼
forall-authoring library ──► project workspace
    ▲
    │
    │ local stdio MCP adapter
    │
External MCP client
```

Both paths share the same authoring implementation, so path validation,
previews, hash checks, source discovery, and file mutations have one
implementation.

The adapter lives in:

- `crates/forall-mcp-author/`: MCP schemas and stdio server;
- `crates/forall-authoring/`: reusable authoring operations;
- the `forall` binary: `forall mcp-author` command (not published in this repo).

## Transport and lifecycle

The server uses MCP over stdin/stdout:

```bash
forall mcp-author --root /absolute/path/to/project
```

The MCP client starts and stops the process. Standard output is reserved for
JSON-RPC; diagnostics go to standard error.

The server binds to one canonical project root at startup. Individual tool
calls cannot select another root.

## Tools

The local MCP exposes:

- `forall_author_status`
- `forall_author_init`
- `forall_author_discover`
- `forall_author_upsert_requirements`
- `forall_author_scaffold_contracts`
- `forall_author_validate`
- `forall_author_scaffold_property`

The tools initialize `.forall/`, discover public TypeScript, Rust, and Java
symbols, merge explicit requirements, insert caller-supplied Forall contracts,
and validate local authoring state.

They do not run proof engines or submit hosted verification jobs.

## Mutation protocol

Mutating tools support `preview` and `apply`.

Preview returns structured file changes, proposed content, and SHA-256 values.
Discovery returns the current SHA-256 of inspected source files. Apply requests
must provide the expected hash for every existing file they change.

If a file changes between inspection and apply, the operation returns a
conflict instead of overwriting it.

## Safety boundary

The local server:

- rejects absolute paths and path traversal;
- rejects paths that escape through symlinks;
- reads and writes only regular files under its bound root;
- writes atomically and preserves existing permissions and line endings;
- does not execute shell commands;
- does not install dependencies;
- does not access the network;
- does not contain hosted authentication, job, or storage logic.

## Relationship to hosted verification

The local authoring MCP and the [hosted Forall MCP](hosted-mcp.md) are separate:

```text
Local authoring MCP                 Hosted verification MCP
-------------------                 -----------------------
stdio process                       Streamable HTTP service
workspace reads and writes          isolated remote verification
no API key                          Forall API-key authentication
no network                          no access to the local workspace
```

An external coding agent can configure both servers. It authors through the
local server, packages selected files, and submits them to the hosted server.

Standalone Forall does not connect to its own local MCP. Its built-in flow is:

```text
native author.* tools
    → local validation
    → native hosted-verify tools
    → submit / status / explain
```

The native authoring and hosted-verification tools call their shared libraries
in-process; only external clients need the local MCP adapter.

## External-client configuration

`forall` prints a client configuration without modifying editor settings:

```bash
forall mcp-author --root "$PWD" --print-config
```

This command is only needed for external MCP clients. A standalone Forall user
should not need to run it.

A complete external-agent setup has two entries:

```text
forall-author
    command: forall
    args: mcp-author --root <absolute workspace path>

forall-verify
    url: https://mcp.forall.astrio.app/mcp
    authorization: Bearer <FORALL_API_KEY>
```

The resulting journey is:

```text
install forall
    → client launches local authoring adapter
    → author / preview / apply / validate in the workspace
    → client submits selected files to hosted verification
    → poll / explain / fix / resubmit
```

Installing only the hosted MCP exposes verification but not workspace
authoring. Installing only the local adapter exposes authoring but does not run
proofs.

## Validation

Coverage includes:

- TypeScript, Rust, and Java discovery and source edits;
- overloaded, missing, and duplicate symbols;
- preview/apply and idempotent reruns;
- stale hashes, traversal, and symlink escapes;
- atomic write behavior and permission preservation;
- MCP initialize, tool listing, schemas, structured errors, and calls;
- minimal brownfield authoring flows for all three languages.

Run the crate tests from the repository root:

```bash
cargo test -p forall-mcp-author
cargo test -p forall-authoring
```
