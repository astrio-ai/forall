import { findProduct } from "../catalog/products.js";
import {
  DEFAULT_TAX_RATE_BPS,
  SHIPPING_POLICY,
  TAX_RATES_BY_REGION,
} from "../catalog/storeConfig.js";
import { buildQuote } from "../domain/quote.js";
import type { CartLine, Coupon, Quote } from "../domain/types.js";

export type QuoteItemRequest = {
  sku: string;
  quantity: number;
};

export type QuoteRequest = {
  items: QuoteItemRequest[];
  coupon?: Coupon | null;
  /** Region code for tax table lookup (e.g. CA, NY). */
  region?: string;
};

export type QuoteResponse =
  | { ok: true; quote: Quote }
  | { ok: false; error: string };

/** Conceptual POST /quote — resolve SKUs, then price the cart. */
export function handleQuote(body: QuoteRequest): QuoteResponse {
  if (!body || !Array.isArray(body.items) || body.items.length === 0) {
    return { ok: false, error: "items must be a non-empty array" };
  }

  const lines: CartLine[] = [];
  for (let i = 0; i < body.items.length; i++) {
    const item = body.items[i];
    if (!item || typeof item.sku !== "string" || typeof item.quantity !== "number") {
      return { ok: false, error: "each item needs sku and quantity" };
    }
    if (item.quantity <= 0) {
      return { ok: false, error: `invalid quantity for ${item.sku}` };
    }
    const product = findProduct(item.sku);
    if (product === null) {
      return { ok: false, error: `unknown sku: ${item.sku}` };
    }
    lines.push({
      sku: product.sku,
      unitPriceCents: product.unitPriceCents,
      quantity: item.quantity,
    });
  }

  const quote = buildQuote({
    lines,
    coupon: body.coupon ?? null,
    region: body.region ?? "CA",
    taxRatesByRegion: TAX_RATES_BY_REGION,
    defaultTaxRateBps: DEFAULT_TAX_RATE_BPS,
    shipping: SHIPPING_POLICY,
  });

  return { ok: true, quote };
}
