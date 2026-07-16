# Skills

Optional Cursor / Claude Code / Codex skills for the **hosted verify-only** path.
These skills do **not** require the Forall CLI.

| Skill | Purpose |
|-------|---------|
| [`forall-mcp-verify`](./forall-mcp-verify/SKILL.md) | Hosted `forall_verify` → status → explain playbook |

Shared references (for agents authoring `.forall/` artifacts locally):

- [`references/layout.md`](./references/layout.md) — `.forall/` bootstrap
- [`references/mapping.md`](./references/mapping.md) — mapping schema + examples
- [`references/typescript.md`](./references/typescript.md) — TypeScript contracts
- [`references/rust.md`](./references/rust.md) — Rust contracts
- [`references/java.md`](./references/java.md) — Java contracts

## Install (Cursor)

```bash
mkdir -p .cursor/skills
cp -R skills/forall-mcp-verify .cursor/skills/
cp -R skills/references .cursor/skills/references
```

## Prerequisites

1. Configure MCP with [`@astrio/forall-mcp`](../packages/forall-mcp/README.md)
2. Create `FORALL_API_KEY` at [forall.astrio.app/dashboard](https://forall.astrio.app/dashboard)
3. Ensure `.forall/verify/mapping.yaml` and proof contracts exist (your host agent authors these)

## Typical loop

1. Author mapping and contracts in the workspace (host agent or manual edits)
2. Run **forall-mcp-verify** to submit hosted verification
3. Fix CRITICAL issues locally and re-verify

The deprecated `forall-mcp-author` skill was removed. Authoring for external
agents stays with those agents; Forall MCP is verify-only.
