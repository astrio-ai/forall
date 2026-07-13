# Mapping schema reference

File: `.forall/verify/mapping.yaml` (project) or
`.forall/workflow/changes/<name>/mapping.delta.yaml` (change).

## Top level

```yaml
version: 1
requirements:
  - id: ...
```

`version` must be `1`.

## Requirement fields

| Field | Required | Meaning |
|-------|----------|---------|
| `id` | yes | Stable kebab-case id |
| `capability` | yes | Grouping label |
| `requirement` | yes | One-sentence SHALL |
| `verified` | no (default false) | Formal proof required |
| `property_tested` | no | Property-test required |
| `code.file` | for verify/PBT | Repo-relative source path |
| `code.symbols` | for verify/PBT | Function / method names |
| `contract` | recommended | Human-readable pre/post sketch |
| `property.file` | when PBT | Path to `*.property.ts` |
| `property.symbol` | optional | Export name inside the PBT file |
| `scenarios` | optional | Named scenario refs |
| `claimcheck` | optional | Claimcheck linkage |

## Tiers

1. **Proved** — `verified: true` + contracts in source + proofs phase pass
2. **Property-tested** — `property_tested: true` + scenario file pass
3. **Spec-tracked** — mapped only (`verified: false`, no PBT)

Default for finite deterministic logic: **proved**.

## Examples

### TypeScript (proved)

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

### Rust (proved)

```yaml
  - id: clamp-bounds
    capability: math
    requirement: clamp returns a value within [lo, hi] when lo <= hi
    verified: true
    code:
      file: src/clamp.rs
      symbols: [clamp]
    contract: |
      requires lo <= hi
      ensures lo <= result <= hi
```

### Java (proved)

```yaml
  - id: clamp-bounds
    capability: math
    requirement: clamp returns a value within [lo, hi] when lo <= hi
    verified: true
    code:
      file: src/Clamp.java
      symbols: [clamp]
    contract: |
      requires lo <= hi
      ensures lo <= result <= hi
```

### Spec-tracked (UI / glue)

```yaml
  - id: render-settings-panel
    capability: ui
    requirement: settings panel renders title and controls
    verified: false
    code:
      file: src/SettingsPanel.tsx
      symbols: [SettingsPanel]
```

### Property-tested

```yaml
  - id: parse-accepts-fuzz
    capability: parse
    requirement: parser never throws on arbitrary strings
    property_tested: true
    property:
      file: .forall/scenarios/parse-accepts-fuzz.property.ts
      symbol: parseAcceptsFuzz
    code:
      file: src/parse.ts
      symbols: [parse]
```

## Discovery tips (no CLI)

1. Prefer already-exported pure functions
2. Prefer symbols with existing annotations
3. Start with 1–5 requirements
4. Keep HTTP handlers / array orchestration / presentation spec-tracked first
5. Merge into existing mapping — do not wipe unrelated ids
