import {
  applyDiscountCap,
  applyFixedDiscount,
  applyPercentDiscount,
} from "./money.js";
import type { Coupon } from "./types.js";

function clampPercent(percentOff: number): number {
  if (percentOff < 0) return 0;
  if (percentOff > 100) return 100;
  return percentOff;
}

/** Apply one coupon, then the max-discount cap. */
export function applyCoupon(priceCents: number, coupon: Coupon): number {
  let discounted: number;
  if (coupon.kind === "percent") {
    discounted = applyPercentDiscount(priceCents, clampPercent(coupon.percentOff));
  } else {
    const amt = coupon.amountCents < 0 ? 0 : coupon.amountCents;
    discounted = applyFixedDiscount(priceCents, amt);
  }
  const cap = coupon.maxDiscountCents < 0 ? 0 : coupon.maxDiscountCents;
  return applyDiscountCap(priceCents, discounted, cap);
}
