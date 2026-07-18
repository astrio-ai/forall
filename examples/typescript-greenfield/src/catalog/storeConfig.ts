import type { ShippingPolicy } from "../domain/types.js";

/** Store-level tax and shipping configuration. */

export const DEFAULT_TAX_RATE_BPS = 825; // 8.25%

export const TAX_RATES_BY_REGION: Record<string, number> = {
  CA: 725,
  NY: 800,
  TX: 625,
  WA: 650,
};

export const SHIPPING_POLICY: ShippingPolicy = {
  flatFeeCents: 599,
  freeThresholdCents: 5000, // free shipping at $50+
};
