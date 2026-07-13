# Open-source crates

This directory contains the Rust libraries that power Forall's local authoring
and hosted verification adapters. They are published as part of the
[astrio-labs/forall](https://github.com/astrio-labs/forall) repository.

The full Forall agent runtime (TUI, turn loop, sandbox, native tool registry,
and CLI entrypoints) is **not** open source. It is distributed only as the
prebuilt `forall` binary from [forall.astrio.app](https://forall.astrio.app).

## Crates

| Crate | Purpose |
| --- | --- |
| [`forall-authoring`](forall-authoring/) | Deterministic workspace authoring: project status, initialization, symbol discovery, requirement mapping, contract scaffolding, property scaffolding, and validation |
| [`forall-mcp-author`](forall-mcp-author/) | MCP stdio server exposing authoring tools for external coding agents (`forall_author_*`) |
| [`forall-hosted-verify`](forall-hosted-verify/) | Hosted verification MCP client, snapshot packer, and wire types |

## Building and testing

From the repository root:

```bash
cargo test --workspace
```

## Documentation

- [Architecture](../docs/architecture.md)
- [Local authoring MCP](../docs/local-authoring-mcp.md)
- [Hosted Forall MCP](../docs/hosted-mcp.md)

## Relationship to the binary

The `forall` executable embeds these crates and adds:

- native `author.*` and `verify.*` tool registrations;
- the `forall mcp-author` stdio command for external MCP clients;
- workflow, sandbox, and agent runtime logic not published here.

External agents can use the same capabilities through MCP without building from
source, as long as the `forall` binary is installed.
