# Cart

## Requirements

### Requirement: a cart line total is never negative when unit price and quantity are non-negative

#### Scenario: non-negative-inputs

- **WHEN** unit price is 500 and quantity is 3
- **THEN** line total is 1500 and is non-negative

### Requirement: cart subtotal is never negative

#### Scenario: empty-or-valid-cart

- **WHEN** the cart is empty or contains only non-negative line fields
- **THEN** subtotal is >= 0
