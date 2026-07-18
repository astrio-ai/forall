# Coupons

## Requirements

### Requirement: a percent discount never returns a price above the original or below zero

#### Scenario: twenty-percent-off

- **WHEN** price is 10000 cents and percent off is 20
- **THEN** discounted price stays within [0, 10000]

### Requirement: a fixed discount never exceeds the price and never goes negative

#### Scenario: fixed-amount-off

- **WHEN** price is 10000 cents and amount off is 1500
- **THEN** discounted price stays within [0, 10000]

### Requirement: savings after discount never exceed the configured max-discount cap

#### Scenario: cap-applied

- **WHEN** original is 10000, discounted is 7000, and cap is 2000
- **THEN** capped price is 8000

### Requirement: applying one coupon keeps the price between zero and the original subtotal

#### Scenario: single-coupon

- **WHEN** a coupon is applied to a non-negative price
- **THEN** the result stays within [0, priceCents]
