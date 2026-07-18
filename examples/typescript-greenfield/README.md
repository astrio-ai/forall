# TypeScript greenfield — checkout pricing

A small multi-module checkout pricing service with Forall specs, mapping, and
proof contracts on the money / shipping helpers.

## Layout

```text
.forall/
  verified/mapping.yaml
  specs/<capability>/spec.md
src/
  domain/
    money.ts       # verified helpers (//@ contracts)
    cart.ts / coupons.ts / tax.ts / shipping.ts / quote.ts
    types.ts
  catalog/
    products.ts / storeConfig.ts
  api/
    quoteHandler.ts / healthHandler.ts
  index.ts
```

## Run (TypeScript)

```bash
cd examples/typescript-greenfield
npm install
npm run check
```

## Use with Forall

```bash
cd examples/typescript-greenfield
forall check --root .
```

## Verified vs spec-tracked

| Tier | Modules |
|------|---------|
| Proved (`verified: true`) | `domain/money.ts` — line totals, discounts, cap, tax, shipping fee |
| Spec-tracked | cart, coupons, tax lookup, shipping wiring, quote, catalog, API |
