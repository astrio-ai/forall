# Skills

Cursor / Claude Code / Codex skills for the **author locally → verify on hosted
MCP** loop. These skills do **not** require the Forall CLI.

| Skill | Purpose |
|-------|---------|
| [`forall-mcp-author`](./forall-mcp-author/SKILL.md) | Brownfield init, mapping, and proof-contract scaffolding (TS / Java / Rust) |
| [`forall-mcp-verify`](./forall-mcp-verify/SKILL.md) | Hosted `forall_verify` → status → explain playbook |

Shared references:

- [`references/layout.md`](./references/layout.md) — `.forall/` bootstrap
- [`references/mapping.md`](./references/mapping.md) — mapping schema + examples
- [`references/typescript.md`](./references/typescript.md) — TypeScript contracts
- [`references/rust.md`](./references/rust.md) — Rust contracts
- [`references/java.md`](./references/java.md) — Java contracts

## Install (Cursor)

Copy into your project or user skills directory:

```bash
mkdir -p .cursor/skills
cp -R skills/forall-mcp-author skills/forall-mcp-verify .cursor/skills/
cp -R skills/references .cursor/skills/references
```

Or point the agent at this repo’s `skills/` folder.

## Prerequisites

1. Configure the hosted MCP endpoint described in
   [`forall-mcp-verify`](./forall-mcp-verify/SKILL.md)
2. Set `FORALL_API_KEY` for the MCP client
3. Open a TypeScript, Java, or Rust workspace

## Typical loop

1. Run **forall-mcp-author** to create `.forall/` + map 1–5 symbols + add contracts
2. Run **forall-mcp-verify** to submit hosted verification
3. Fix CRITICAL issues locally and re-verify
