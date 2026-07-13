# Forall Documentation

User-facing documentation for the Forall CLI.

| Guide | What it covers |
|-------|----------------|
| [Getting Started](getting-started.md) | Install Forall and set up your first project |
| [Project Layout](project-layout.md) | The `.forall/` directory and what each file does |
| [Workflow](workflow.md) | The propose → verify → archive change workflow |
| [Architecture](architecture.md) | Standalone binary, open-source adapters, and hosted verification |
| [Local Authoring MCP](local-authoring-mcp.md) | Optional stdio MCP for external coding agents |
| [Hosted Forall MCP](hosted-mcp.md) | Remote verification service and client library |

## Open-source crates

The [crates](../crates/) directory publishes the Rust libraries behind Forall's
authoring and hosted verification adapters. The full agent runtime is still
distributed as the prebuilt `forall` CLI; see [crates/README.md](../crates/README.md).
