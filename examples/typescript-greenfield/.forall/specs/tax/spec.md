# Tax

## Requirements

### Requirement: computed tax is never negative when base and rate are non-negative

#### Scenario: positive-rate

- **WHEN** taxable base is 8000 and rate is 825 bps
- **THEN** tax is >= 0
