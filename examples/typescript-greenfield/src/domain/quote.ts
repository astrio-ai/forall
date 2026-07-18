import { subtotal } from "./cart.js";
import { applyCoupon } from "./coupons.js";
import { shippingForMerchandise } from "./shipping.js";
import { taxForMerchandise, taxRateForRegion } from "./tax.js";
import type { CartLine, Coupon, Quote, ShippingPolicy } from "./types.js";

export type QuoteInput = {
  lines: CartLine[];
  coupon: Coupon | null;
  region: string;
  taxRatesByRegion: Record<string, number>;
  defaultTaxRateBps: number;
  shipping: ShippingPolicy;
};

/** Build a full checkout quote: cart → coupon → shipping → tax → total. */
export function buildQuote(input: QuoteInput): Quote {
  const subtotalCents = subtotal(input.lines);

  let merchandiseCents: number;
  if (input.coupon === null) {
    merchandiseCents = subtotalCents;
  } else {
    merchandiseCents = applyCoupon(subtotalCents, input.coupon);
  }

  const discountCents = subtotalCents - merchandiseCents;
  const shippingCents = shippingForMerchandise(merchandiseCents, input.shipping);
  const rateBps = taxRateForRegion(
    input.region,
    input.taxRatesByRegion,
    input.defaultTaxRateBps,
  );
  const taxCents = taxForMerchandise(merchandiseCents, rateBps);
  const totalCents = merchandiseCents + shippingCents + taxCents;

  return {
    subtotalCents,
    discountCents,
    merchandiseCents,
    shippingCents,
    taxCents,
    totalCents,
  };
}
