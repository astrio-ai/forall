# TypeScript / LemmaScript contracts

Adapter: `typescript` → `proof: lemmascript-dafny`
Extensions: `.ts`, `.tsx`

Hosted check runs LemmaScript extract + Dafny proofs on mapped files.
Contracts live in TypeScript as `//@` annotations. Companion `.dfy` files sit
next to the source when lemmas or manual proof edits are needed.

## Annotation style

Place annotations **inside** the function (LemmaScript extract style):

```ts
export function clamp(x: number, lo: number, hi: number): number {
  //@ requires lo <= hi
  //@ ensures lo <= $result && $result <= hi
  //@ contract Bounds clamp
  if (x < lo) return lo;
  if (x > hi) return hi;
  return x;
}
```

Notes:

- Use `$result` for the return value in ensures
- Keep predicates decidable and local to the function
- Avoid I/O, DOM, and network inside verified functions

## Mapping

```yaml
verified: true
code:
  file: src/clamp.ts
  symbols: [clamp]
```

## Companion Dafny files

For `src/clamp.ts`, LemmaScript may use:

- `src/clamp.dfy` — editable proof surface
- generated merge artifacts (`.dfy.gen`, `.dfy.base`, `.dfy.merged`)

Authoring guidance:

1. Start with `//@` only and run hosted verify
2. If proofs fail needing lemmas, add/edit `src/clamp.dfy`
3. Never leave git conflict markers in `.dfy` / `.dfy.merged`
4. When uploading inline to MCP, include both `.ts` and any `.dfy` siblings

## Good first targets

- Bounds clamping, saturation arithmetic
- Parsers with finite grammars
- Validators / policy checks with clear preconditions
- Pure scoring / ranking helpers

## Avoid on first pass

- React components and hooks
- Fetch / filesystem wrappers
- Functions whose specs need unbounded quantification (use PBT or split)

## Failure loop

1. Read `proofs` phase issues from hosted status
2. Tighten `//@ requires` / `//@ ensures` or fix the implementation
3. Add lemmas in `.dfy` only when the contract is already honest
4. Re-verify — do not flip `verified: false` to silence failures
