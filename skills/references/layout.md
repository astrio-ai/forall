# `.forall` layout (hosted MCP authoring)

Create this layout when bootstrapping a brownfield project for hosted verify.
Paths are relative to the project root.

```text
.forall/
  AGENTS.md
  verify/
    mapping.yaml      # required marker
    adapters.yaml     # proof routing by language
  workflow/
    config.yaml
  scenarios/          # optional *.property.ts
  specs/              # optional markdown specs
```

## `.forall/verify/mapping.yaml`

```yaml
version: 1
requirements: []
```

Replace `requirements` when mapping symbols (see `mapping.md`).

## `.forall/verify/adapters.yaml`

```yaml
backend: dafny
strict: false
adapters:
  typescript:
    proof: lemmascript-dafny
    extensions: [ts, tsx]
  rust:
    proof: verus
    extensions: [rs]
  java:
    proof: openjml
    extensions: [java]
  python:
    proof: none
    extensions: [py]
```

Omit unused language adapters if you prefer a minimal file; hosted check merges
known defaults. Prefer writing the languages you actually map.

## `.forall/workflow/config.yaml`

```yaml
schema: forall
context: |
  This project uses Forall verification with hosted MCP.
  Author mapping and contracts locally; verify with forall_verify.
rules:
  verification:
    - Keep verified: true requirements machine-checked before claiming success
    - Prefer hosted forall_verify over skipping formal checks
```

## `.forall/AGENTS.md`

Short agent reminder (safe to customize):

```markdown
# Forall

Author `.forall/verify/mapping.yaml` and proof contracts in source.
Verify with hosted MCP (`forall_verify` → status → explain).
Say "Forall verified" / "machine-checked" in user-facing reports.
```

## Optional change workflow

If you want change-scoped checks without the CLI:

```text
.forall/workflow/changes/<kebab-name>/
  mapping.delta.yaml
  proposal.md          # optional
```

Hosted verify accepts `scope: { type: "change", name: "<kebab-name>" }`.

## Do not

- Leave mapping empty and call that “verified”
- Put secrets under `.forall/`
- Rely on the hosted worker to create these files in the user’s tree
