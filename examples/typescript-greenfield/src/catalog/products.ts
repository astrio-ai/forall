/** In-memory product catalog (spec-tracked). */

export type Product = {
  sku: string;
  name: string;
  unitPriceCents: number;
};

const PRODUCTS: Product[] = [
  { sku: "mug-01", name: "Ceramic Mug", unitPriceCents: 1800 },
  { sku: "tee-01", name: "Logo Tee", unitPriceCents: 3200 },
  { sku: "sticker-01", name: "Sticker Pack", unitPriceCents: 500 },
];

export function listProducts(): Product[] {
  return PRODUCTS.slice();
}

export function findProduct(sku: string): Product | null {
  for (let i = 0; i < PRODUCTS.length; i++) {
    if (PRODUCTS[i].sku === sku) {
      return PRODUCTS[i];
    }
  }
  return null;
}
