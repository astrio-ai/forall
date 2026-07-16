# Hosted Forall MCP

Remote verification for **other coding agents** (Cursor, Claude Code, Codex) and
for the Forall CLI's native `verify.*` tools.

The hosted service **does not write your workspace**. Your agent applies fixes
from the sanitized report.

## Connect (Path B — verify-only)

1. Create an API key at [forall.astrio.app/dashboard](https://forall.astrio.app/dashboard)
2. Add the npm bridge to your MCP client:

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

Source: [`packages/forall-mcp`](../packages/forall-mcp/README.md) (`@astrio/forall-mcp` on npm).

Environment:

| Variable | Required | Default |
|----------|----------|---------|
| `FORALL_API_KEY` | yes | — |
| `FORALL_MCP_URL` | no | `https://mcp.forall.astrio.app/mcp` |

## Forall CLI users (Path A)

Standalone Forall embeds the same hosted service through native `verify.submit`,
`verify.status`, `verify.cancel`, and `verify.explain` tools. Configure the key
once:

```bash
printenv FORALL_API_KEY | forall verification login
```

CLI users should **not** manually add the hosted MCP URL to Forall.

## Scope

- MCP Streamable HTTP at `https://mcp.forall.astrio.app/mcp`
- Bearer auth with self-serve dashboard API keys (`forall_…`)
- Inline file payloads or public GitHub repositories (immutable commit resolution)
- Async jobs in isolated workers running the Forall verification pipeline
- Sanitized structured reports, cancellation, and issue explanation

Deferred: private GitHub repos, OAuth for repos, hosted remote authoring.

## Architecture

```text
MCP client (npx @astrio/forall-mcp or Forall native verify.*)
    │  Streamable HTTP + Bearer
    ▼
Hosted Forall MCP
    │
    └──────────► isolated verification worker
                      │
                      ▼
              Forall verification pipeline
```

Verification is asynchronous. Clients submit a job and poll status.

## Source inputs

### Inline files

```json
{
  "type": "inline",
  "files": [{ "path": "src/lib.rs", "content": "..." }]
}
```

### GitHub repository

```json
{
  "type": "github",
  "repository": "owner/repository",
  "ref": "main",
  "subdirectory": "optional/path"
}
```

## MCP tools

| Tool | Purpose |
|------|---------|
| `forall_verify` | Submit async verification job |
| `forall_verification_status` | Poll progress and sanitized report |
| `forall_cancel_verification` | Cancel queued/running job |
| `forall_explain_verification` | Explain selected findings |

### `forall_verify` (submit)

```json
{
  "source": {
    "type": "inline",
    "files": [{ "path": "src/lib.rs", "content": "..." }]
  },
  "scope": { "type": "project" },
  "strict": false
}
```

Response includes `job_id`, `status`, and `poll_after_ms`.

### `forall_verification_status`

```json
{ "job_id": "vrf_..." }
```

States: `queued`, `preparing`, `running`, `succeeded`, `failed`, `cancelled`, `expired`.

### `forall_cancel_verification` / `forall_explain_verification`

See tool schemas in the MCP client after connect.

## Open-source client library

[`crates/forall-hosted-verify`](../crates/forall-hosted-verify/) implements
snapshot packaging, authenticated MCP calls, and response parsing for Rust
integrators. It does not contain the hosted service itself.

## Skills (optional)

[`skills/forall-mcp-verify`](../skills/forall-mcp-verify/SKILL.md) documents a
verify playbook for agents that already have `.forall/` mapping and contracts in
the workspace. Your host agent is responsible for authoring those artifacts.

## Security

- API keys are minted/revoked at the dashboard; revoked keys fail immediately
- Reports exclude container paths, env values, credentials, and raw command lines
- Workers run in isolated environments with pinned proof toolchains
