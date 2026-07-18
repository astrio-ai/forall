import { applyTax } from "./money.js";

/** Resolve a tax rate (bps) from a region code using a simple table. */
export function taxRateForRegion(
  region: string,
  ratesByRegion: Record<string, number>,
  fallbackBps: number,
): number {
  const rate = ratesByRegion[region];
  if (typeof rate === "number" && rate >= 0) {
    return rate;
  }
  return fallbackBps < 0 ? 0 : fallbackBps;
}

/** Compute tax for merchandise using a resolved rate. */
export function taxForMerchandise(merchandiseCents: number, rateBps: number): number {
  const base = merchandiseCents < 0 ? 0 : merchandiseCents;
  const rate = rateBps < 0 ? 0 : rateBps;
  return applyTax(base, rate);
}
