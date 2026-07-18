import { lineTotal } from "./money.js";
import type { CartLine } from "./types.js";

/** Sum of priced line totals. */
export function subtotal(lines: CartLine[]): number {
  let sum = 0;
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const unit = line.unitPriceCents < 0 ? 0 : line.unitPriceCents;
    const qty = line.quantity < 0 ? 0 : line.quantity;
    sum = sum + lineTotal(unit, qty);
  }
  return sum;
}
