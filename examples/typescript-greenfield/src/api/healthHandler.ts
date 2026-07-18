/** Conceptual GET /health. */

export type HealthResponse = {
  ok: true;
  service: string;
};

export function handleHealth(): HealthResponse {
  return { ok: true, service: "checkout-pricing" };
}
