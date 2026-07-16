# Open-source crates

Rust libraries embedded by the `forall` binary and published in this repository.

The full agent runtime (TUI, turn loop, sandbox, native tool registry) is
distributed only as the prebuilt `forall` binary from
[forall.astrio.app](https://forall.astrio.app).

## Crates

| Crate | Purpose |
| --- | --- |
| [`forall-authoring`](forall-authoring/) | Deterministic workspace authoring: project status, initialization, symbol discovery, requirement mapping, contract scaffolding, property scaffolding, and validation |
| [`forall-hosted-verify`](forall-hosted-verify/) | Hosted verification MCP client, snapshot packer, and wire types |

## Building and testing

```bash
cargo test --workspace
```

## Documentation

- [Architecture](../docs/architecture.md)
- [Hosted Forall MCP](../docs/hosted-mcp.md)

## Relationship to the binary

The `forall` executable embeds these crates and adds native `author.*` and
`verify.*` tool registrations, workflow, sandbox, and agent runtime logic not
published here.

External agents connect through [`@astrio/forall-mcp`](../packages/forall-mcp/README.md)
for hosted verification only.

## Syncing from `forall-core`

Verified in sync with `astrio-labs/forall-core` as of 2026-07-16:

| Public crate | Source in `forall-core` |
| --- | --- |
| `forall-authoring` | `agent/workflow/src/authoring/` + `mapping/schema` |
| `forall-hosted-verify` | `agent/forall-hosted-verify/` |

When copying updates, keep public integration tests on `forall_authoring::` imports.
