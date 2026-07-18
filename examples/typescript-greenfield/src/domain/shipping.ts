import { shippingFee } from "./money.js";
import type { ShippingPolicy } from "./types.js";

/** Shipping cost for merchandise under a store shipping policy. */
export function shippingForMerchandise(
  merchandiseCents: number,
  policy: ShippingPolicy,
): number {
  const merchandise = merchandiseCents < 0 ? 0 : merchandiseCents;
  const flat = policy.flatFeeCents < 0 ? 0 : policy.flatFeeCents;
  const threshold = policy.freeThresholdCents < 0 ? 0 : policy.freeThresholdCents;
  return shippingFee(merchandise, flat, threshold);
}
