# Getting started with Forall

Choose one path:

## 1. Forall CLI (full coding agent)

Install and run Forall as your agent:

```bash
curl -fsSL https://forall.astrio.app/install.sh | bash
forall
```

On first launch, pick:

| Option | What you do |
|--------|-------------|
| **Forall account** | Browser opens [forall.astrio.app/dashboard](https://forall.astrio.app/dashboard) → create a `forall_…` API key → paste it into the TUI → then choose a model provider (BYOK) for chat |
| **Bring your own API key** | Skip the Forall key for now; set `OPENAI_API_KEY` / `OPENROUTER_API_KEY` in `~/.forall/.env` |

CLI equivalent for the Forall key:

```bash
printenv FORALL_API_KEY | forall verification login
forall verification status
```

Then initialize a project from a git repo root:

```bash
forall init
forall
```

See [Project Layout](project-layout.md) and [Workflow](workflow.md).

### Supported platforms

| OS | Architectures |
|----|---------------|
| macOS | Apple Silicon (`aarch64`), Intel (`x86_64`) |
| Linux | `x86_64`, `aarch64` |
| Windows | `x86_64` |

## 2. MCP verify-only (Cursor / Claude Code / Codex)

Do **not** install the Forall CLI. Use the npm bridge with your existing agent:

1. Create a key at [forall.astrio.app/dashboard](https://forall.astrio.app/dashboard)
2. Configure MCP:

```json
{
  "mcpServers": {
    "forall": {
      "command": "npx",
      "args": ["-y", "@astrio/forall-mcp"],
      "env": {
        "FORALL_API_KEY": "forall_..."
      }
    }
  }
}
```

Hosted MCP only verifies. Your coding agent edits the workspace from the report.

See [Hosted Forall MCP](hosted-mcp.md), [packages/forall-mcp](../packages/forall-mcp/README.md), and
[Architecture](architecture.md).
