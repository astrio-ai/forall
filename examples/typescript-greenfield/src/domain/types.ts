/** Shared checkout domain types (integer cents). */

export type CartLine = {
  sku: string;
  unitPriceCents: number;
  quantity: number;
};

/** At most one coupon per quote. */
export type Coupon =
  | {
      kind: "percent";
      percentOff: number;
      maxDiscountCents: number;
    }
  | {
      kind: "fixed";
      amountCents: number;
      maxDiscountCents: number;
    };

export type Quote = {
  subtotalCents: number;
  discountCents: number;
  merchandiseCents: number;
  shippingCents: number;
  taxCents: number;
  totalCents: number;
};

export type ShippingPolicy = {
  flatFeeCents: number;
  freeThresholdCents: number;
};
