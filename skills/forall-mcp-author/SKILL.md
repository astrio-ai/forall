---
name: forall-mcp-author
description: >-
  Author Forall verification artifacts for brownfield TypeScript, Java, or Rust
  projects without the Forall CLI. Creates .forall layout, mapping,
  and proof contracts, then hands off to hosted MCP verify. Use when the user
  wants Cursor/Claude/Codex to add machine-checked specs and call hosted
  forall_verify.
license: MIT
compatibility: >-
  Requires write access to the project workspace. Hosted MCP
  (forall_verify / forall_verification_status / forall_explain_verification)
  for verification — Forall CLI is optional.
metadata:
  author: forall
  version: "1.0"
---

# Forall MCP author (brownfield → hosted verify)

Turn an existing TypeScript, Java, or Rust codebase into a Forall-verified
project by **writing files locally**, then verifying with **hosted MCP**.

Do **not** wait for Forall CLI. Do **not** ask the hosted worker to invent
specs — it only checks what you commit to the workspace (or upload inline).

## When to use

- User asks to verify, prove, or Forall-ify a brownfield repo
- Empty or skeleton `.forall/` exists but nothing is mapped
- User wants Cursor / Claude Code / Codex + hosted MCP

## Architecture

```text
This skill (local file writes)
  init → discover → map → scaffold contracts → (optional PBT)
        ↓
forall-mcp-verify skill / hosted MCP tools
  forall_verify → status → explain → fix → re-verify
```

## Languages

| Language | Forall contract syntax |
|----------|------------------------|
| TypeScript / TSX | `//@ requires` / `//@ ensures` / `//@ contract` |
| Rust | inline `requires` / `ensures` / `proof` |
| Java | `//@ requires` / `//@ ensures` above methods |

Read language details from:

- `skills/references/typescript.md`
- `skills/references/rust.md`
- `skills/references/java.md`
- `skills/references/mapping.md`
- `skills/references/layout.md`

## Steps

### 1. Detect project languages

Inspect the workspace root:

- TypeScript: `package.json`, `tsconfig.json`, `src/**/*.{ts,tsx}`
- Rust: `Cargo.toml`, `src/**/*.rs`
- Java: `pom.xml` / `build.gradle*`, `src/**/*.java`

Prefer **pure logic** modules (math, validation, parsing, scoring). Leave UI,
HTTP glue, and I/O **spec-tracked** (`verified: false`) on the first pass.

### 2. Ensure `.forall/` layout

If `.forall/verify/mapping.yaml` is missing, create the layout from
`skills/references/layout.md`:

```text
.forall/
  AGENTS.md
  verify/
    mapping.yaml
  workflow/
    config.yaml
  scenarios/          # optional property tests
  specs/              # optional markdown specs
```

Rules:

- Never overwrite a non-empty `mapping.yaml` — merge requirements instead
- Keep `version: 1` on mapping files

### 3. Discover candidates (brownfield)

Pick a small first slice (1–5 symbols), not the whole tree.

**Heuristics (no CLI required):**

1. Exported / public functions with clear pre/postconditions
2. Already-annotated symbols (`//@`, `requires`, `ensures`)
3. Pure functions (no network, filesystem, or DOM)

For each candidate, record:

- `id` (kebab-case, stable)
- `capability` (short group name)
- `requirement` (one sentence SHALL)
- `code.file` + `code.symbols`
- Tier: `verified: true` (default for finite logic) or `verified: false` (spec-tracked) or `property_tested: true` (quantified / infinite domains)

Show the proposed slice to the user if scope is ambiguous; otherwise proceed
with a minimal high-value set.

### 4. Write / update mapping

Edit `.forall/verify/mapping.yaml` (project-wide) or
`.forall/workflow/changes/<name>/mapping.delta.yaml` (change-scoped).

Minimal verified requirement:

```yaml
version: 1
requirements:
  - id: clamp-bounds
    capability: math
    requirement: clamp returns a value within [lo, hi] when lo <= hi
    verified: true
    code:
      file: src/clamp.ts
      symbols: [clamp]
    contract: |
      requires lo <= hi
      ensures lo <= result <= hi
```

Copy full schema notes from `skills/references/mapping.md`.

### 5. Scaffold proof contracts in source

For each `verified: true` requirement:

**TypeScript** — inside the function body:

```ts
export function clamp(x: number, lo: number, hi: number): number {
  //@ requires lo <= hi
  //@ ensures lo <= $result && $result <= hi
  if (x < lo) return lo;
  if (x > hi) return hi;
  return x;
}
```

Prefer simple contracts first; let hosted check reveal what is missing.

**Rust** — Forall contracts on the function:

```rust
pub fn clamp(x: u64, lo: u64, hi: u64) -> (result: u64)
    requires lo <= hi,
    ensures lo <= result && result <= hi,
{
    if x < lo { lo } else if x > hi { hi } else { x }
}
```

**Java** — Forall contracts above the method:

```java
//@ requires lo <= hi;
//@ ensures lo <= \result && \result <= hi;
public static int clamp(int x, int lo, int hi) { ... }
```

Prove **scalar** methods first. Keep array/`\sum`/recursive aggregates
spec-tracked until core lemmas pass.

### 6. Optional property tests

Only when the user asks for PBT, or the property is inherently quantified:

- Add `.forall/scenarios/<id>.property.ts`
- Set `property_tested: true` and `property: { file, symbol }` on the requirement
- Do **not** replace finite deterministic logic with PBT

### 7. Hand off to hosted verify

Load `skills/forall-mcp-verify/SKILL.md` and run the verify loop.

Prefer:

1. **GitHub source** if the branch is pushed and public
2. **Inline source** otherwise — upload `.forall/**`, mapped sources, and
   supporting project files needed for the check

### 8. Fix and iterate

On CRITICAL proof/mapping failures:

1. Call `forall_explain_verification` for opaque issues
2. Fix contracts or implementation locally
3. Re-submit verify
4. Never set `verified: false` only to silence a verification failure
5. Never use `//@ assume` / unproven `assume()` to cheat

## Guardrails

- Authoring = **local writes**; hosted MCP = **remote check only**
- Start with 1–5 requirements; expand after the first green proofs phase
- User-facing status: say **Forall verified** / **machine-checked**
- Keep UI / presentation / HTTP orchestration spec-tracked
- Do not invent private repo access — hosted GitHub source is public-only today
