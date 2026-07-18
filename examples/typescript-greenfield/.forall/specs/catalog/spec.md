# Catalog

## Requirements

### Requirement: quote requests resolve known SKUs to catalog prices

#### Scenario: known-sku

- **WHEN** the request includes sku mug-01
- **THEN** findProduct returns the catalog product with that sku
