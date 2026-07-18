# Shipping

## Requirements

### Requirement: shipping fee is either zero or the flat fee and never exceeds the flat fee

#### Scenario: free-over-threshold

- **WHEN** merchandise is at or above the free-shipping threshold
- **THEN** shipping fee is 0

#### Scenario: flat-fee-under-threshold

- **WHEN** merchandise is below the free-shipping threshold
- **THEN** shipping fee equals the flat fee and is <= flat fee
