# Bonding Curves Library

A Rust library implementing various bonding curve algorithms for token pricing mechanisms. This library provides a unified interface for different bonding curve types commonly used in DeFi applications, automated market makers (AMMs), and token economics.

## Features

- **Multiple Bonding Curve Types**: Linear, Exponential, Logarithmic, Sigmoid, and Bancor curves
- **Fixed-Point Arithmetic**: Uses `I64F64` fixed-point numbers for precise calculations
- **Unified Interface**: All curves implement the `BondingCurve` trait
- **Error Handling**: Comprehensive error handling for edge cases and invalid inputs
- **Safe Operations**: Built-in validation for mathematical operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bonding-curves = "0.1.0"
fixed = "1.0"
```

## Bonding Curve Types

### 1. Linear Bonding Curve

The simplest bonding curve with a constant slope.

**Formula**: `P = k × S`

Where:
- `P` = Token price
- `k` = Slope (constant parameter)
- `S` = Current token supply

**Cost Integration**: `Cost = k × (S + ΔS)² / 2 - k × S² / 2`

```rust
use bonding_curves::{Linear, BondingCurve};
use fixed::types::I64F64;

let mut curve = Linear::new(0.01)?; // slope = 0.01
let cost = curve.buy_token(I64F64::from_num(100))?; // Buy 100 tokens
let price = curve.get_price()?; // Get current price
```

### 2. Exponential Bonding Curve

Exponential growth curve that increases rapidly with supply.

**Formula**: `P = c × S^n`

Where:
- `P` = Token price
- `c` = Coefficient (scaling factor)
- `S` = Current token supply
- `n` = Exponent (determines curve steepness)

**Cost Integration**: `Cost = (c / (n + 1)) × [(S + ΔS)^(n+1) - S^(n+1)]`

```rust
use bonding_curves::{Exponential, BondingCurve};
use fixed::types::I64F64;

let mut curve = Exponential::new(0.001, 2.0)?; // coefficient = 0.001, exponent = 2.0
let cost = curve.buy_token(I64F64::from_num(50))?;
```

### 3. Logarithmic Bonding Curve

Logarithmic curve that grows slowly and levels off over time.

**Formula**: `P = c × ln(S + k)`

Where:
- `P` = Token price
- `c` = Coefficient (scaling factor)
- `S` = Current token supply
- `k` = Constant (prevents ln(0), typically 1)

**Cost Integration**: `Cost = c × [(S_new × ln(S_new) - S_new) - (S_old × ln(S_old) - S_old)]`

```rust
use bonding_curves::{Logarithmic, BondingCurve};
use fixed::types::I64F64;

let mut curve = Logarithmic::new(10.0, 1.0)?; // coefficient = 10.0, constant = 1.0
let cost = curve.buy_token(I64F64::from_num(25))?;
```

### 4. Sigmoid Bonding Curve

S-shaped curve that starts slow, accelerates, then levels off at a maximum price.

**Formula**: `P = M / (1 + e^(-k(S - m)))`

Where:
- `P` = Token price
- `M` = Maximum price (asymptote)
- `k` = Steepness parameter
- `S` = Current token supply
- `m` = Midpoint (inflection point)

**Cost Integration**: `Cost = (M / k) × [ln(1 + e^(k×S_new)) - ln(1 + e^(k×S_old))]`

```rust
use bonding_curves::{Sigmoid, BondingCurve};
use fixed::types::I64F64;

let mut curve = Sigmoid::new(100.0, 0.01, 1000.0)?; // max_price = 100, steepness = 0.01, midpoint = 1000
let cost = curve.buy_token(I64F64::from_num(75))?;
```

### 5. Bancor Bonding Curve

Reserve-based bonding curve used in the Bancor protocol and many AMMs.

**Formula**: `P = reserve_balance / (token_supply × connector_weight)`

Where:
- `P` = Token price
- `reserve_balance` = Amount of reserve currency held
- `token_supply` = Current token supply
- `connector_weight` = Constant ratio (0 < w ≤ 1)

**Note**: Unlike other curves, Bancor operates on reserve amounts rather than token amounts for purchases.

```rust
use bonding_curves::{Bancor, BondingCurve};
use fixed::types::I64F64;

let mut curve = Bancor::new(1000, 100, 0.5)?; // reserve = 1000, supply = 100, weight = 0.5
let tokens_received = curve.buy_token(I64F64::from_num(200))?; // Add 200 to reserve
let reserve_received = curve.sell_token(I64F64::from_num(50))?; // Sell 50 tokens
```

## Common Interface

All bonding curves implement the `BondingCurve` trait:

```rust
pub trait BondingCurve {
    /// Get the current price based on the curve's state
    fn get_price(&self) -> Result<I64F64, BondingCurveError>;

    /// Calculate cost/tokens for buying (behavior varies by curve type)
    fn buy_token(&mut self, amount: I64F64) -> Result<I64F64, BondingCurveError>;

    /// Calculate refund/reserve for selling tokens
    fn sell_token(&mut self, token_amount: I64F64) -> Result<I64F64, BondingCurveError>;

    /// Return the total supply of tokens
    fn get_supply(&self) -> I64F64;

    /// Return the current reserve (Some for Bancor, None for others)
    fn get_reserve(&self) -> Option<I64F64>;
}
```

## Error Handling

The library provides comprehensive error handling through the `BondingCurveError` enum:

```rust
pub enum BondingCurveError {
    InvalidInput(String),     // Invalid parameters or input values
    CalculationError(String), // Mathematical calculation errors
}
```

## Usage Examples

### Basic Usage

```rust
use bonding_curves::{Linear, BondingCurve, BondingCurveError};
use fixed::types::I64F64;

fn main() -> Result<(), BondingCurveError> {
    // Create a linear bonding curve
    let mut curve = Linear::new(0.01)?;

    // Buy some tokens
    let cost = curve.buy_token(I64F64::from_num(100))?;
    println!("Cost to buy 100 tokens: {}", cost);

    // Check current price and supply
    let price = curve.get_price()?;
    let supply = curve.get_supply();
    println!("Current price: {}, Supply: {}", price, supply);

    // Sell some tokens
    let refund = curve.sell_token(I64F64::from_num(50))?;
    println!("Refund for selling 50 tokens: {}", refund);

    Ok(())
}
```

### Comparing Different Curves

```rust
use bonding_curves::{Linear, Exponential, BondingCurve};
use fixed::types::I64F64;

fn compare_curves() -> Result<(), Box<dyn std::error::Error>> {
    let mut linear = Linear::new(0.01)?;
    let mut exponential = Exponential::new(0.001, 2.0)?;

    let token_amount = I64F64::from_num(100);

    let linear_cost = linear.buy_token(token_amount)?;
    let exp_cost = exponential.buy_token(token_amount)?;

    println!("Linear curve cost: {}", linear_cost);
    println!("Exponential curve cost: {}", exp_cost);

    Ok(())
}
```

## Mathematical Properties

### Integration and Pricing

Most bonding curves (except Bancor) use integration to calculate the total cost of purchasing a range of tokens:

`Total Cost = ∫[S to S+ΔS] P(x) dx`

Where `P(x)` is the price function at supply `x`.

### Fixed-Point Arithmetic

This library uses `I64F64` fixed-point arithmetic to avoid floating-point precision issues common in financial calculations. This provides:

- Deterministic results across different platforms
- No rounding errors from floating-point operations
- Suitable precision for financial applications

## Safety Considerations

- All mathematical operations include overflow and underflow checks
- Invalid inputs are caught and return appropriate errors
- Logarithmic and exponential operations are validated for domain restrictions
- Division by zero is prevented through input validation
