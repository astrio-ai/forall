# Local authoring MCP (deprecated)

**This product path is no longer supported.**

Forall supports two paths:

1. **Forall CLI** — full agent with native authoring (`forall` binary)
2. **MCP verify-only** — [`@astrio/forall-mcp`](../packages/forall-mcp/README.md) for Cursor / Claude / Codex

External agents should use their own authoring workflow and call hosted verify
only. See [getting-started.md](getting-started.md) and [architecture.md](architecture.md).

The `forall mcp-author` command and `crates/forall-mcp-author` adapter were removed
from this repository.
