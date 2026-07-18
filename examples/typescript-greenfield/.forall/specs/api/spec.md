# API

## Requirements

### Requirement: POST /quote returns a pricing quote for a well-formed request with known SKUs

#### Scenario: well-formed-request

- **WHEN** the request includes known SKUs, quantities, and an optional region
- **THEN** the handler returns ok with a quote
