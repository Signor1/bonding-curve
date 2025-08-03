use bonding_curves::{BondingCurve, BondingCurveError, Exponential};
use fixed::types::I64F64;

// Helper function for approximate equality
fn assert_approx_eq(actual: I64F64, expected: I64F64, tolerance: I64F64, message: &str) {
    assert!(
        (actual - expected).abs() < tolerance,
        "{}: {} != {}",
        message,
        actual,
        expected
    );
}

#[test]
fn test_exponential_new_valid() {
    let curve = Exponential::new(2.0, 1.5).unwrap();
    assert_eq!(curve.get_supply(), I64F64::from_num(0));
    assert_eq!(curve.get_price().unwrap(), I64F64::from_num(0));
    assert_eq!(curve.get_reserve(), None);
}

#[test]
fn test_exponential_new_invalid() {
    // Zero coefficient
    let result = Exponential::new(0.0, 1.5);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Coefficient and exponent must be positive and finite")
    ));

    // Negative coefficient
    let result = Exponential::new(-1.0, 1.5);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Coefficient and exponent must be positive and finite")
    ));

    // Zero exponent
    let result = Exponential::new(2.0, 0.0);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Coefficient and exponent must be positive and finite")
    ));

    // Negative exponent
    let result = Exponential::new(2.0, -1.0);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Coefficient and exponent must be positive and finite")
    ));

    // NaN coefficient
    let result = Exponential::new(f64::NAN, 1.5);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Coefficient and exponent must be positive and finite")
    ));

    // Infinity exponent
    let result = Exponential::new(2.0, f64::INFINITY);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Coefficient and exponent must be positive and finite")
    ));
}

#[test]
fn test_exponential_price() {
    let mut curve = Exponential::new(2.0, 1.5).unwrap();
    // Price at supply = 0
    assert_eq!(curve.get_price().unwrap(), I64F64::from_num(0));

    // Price after buying 100 tokens
    curve.buy_token(I64F64::from_num(100)).unwrap();
    let expected_price = I64F64::from_num(2.0 * 100.0_f64.powf(1.5));
    assert_approx_eq(
        curve.get_price().unwrap(),
        expected_price,
        I64F64::from_num(0.001),
        "Price after 100 tokens",
    );

    // Price after buying another 100 tokens
    curve.buy_token(I64F64::from_num(100)).unwrap();
    let expected_price = I64F64::from_num(2.0 * 200.0_f64.powf(1.5));
    assert_approx_eq(
        curve.get_price().unwrap(),
        expected_price,
        I64F64::from_num(0.001),
        "Price after 200 tokens",
    );
}

#[test]
fn test_exponential_buy_tokens() {
    let mut curve = Exponential::new(2.0, 1.5).unwrap();
    // Buy 100 tokens
    let cost = curve.buy_token(I64F64::from_num(100)).unwrap();
    let n_plus_one = 1.5 + 1.0;
    let expected_cost = I64F64::from_num((2.0 / n_plus_one) * (100.0_f64.powf(n_plus_one)));
    assert_approx_eq(
        cost,
        expected_cost,
        I64F64::from_num(0.001),
        "Cost for 100 tokens",
    );
    assert_eq!(curve.get_supply(), I64F64::from_num(100));

    // Buy another 50 tokens
    let cost = curve.buy_token(I64F64::from_num(50)).unwrap();
    let expected_cost = I64F64::from_num(
        (2.0 / n_plus_one) * (150.0_f64.powf(n_plus_one) - 100.0_f64.powf(n_plus_one)),
    );
    assert_approx_eq(
        cost,
        expected_cost,
        I64F64::from_num(0.001),
        "Cost for additional 50 tokens",
    );
    assert_eq!(curve.get_supply(), I64F64::from_num(150));
}

#[test]
fn test_exponential_sell_tokens() {
    let mut curve = Exponential::new(2.0, 1.5).unwrap();
    curve.buy_token(I64F64::from_num(100)).unwrap();
    let refund = curve.sell_token(I64F64::from_num(50)).unwrap();
    let n_plus_one = 1.5 + 1.0;
    let expected_refund = I64F64::from_num(
        (2.0 / n_plus_one) * (100.0_f64.powf(n_plus_one) - 50.0_f64.powf(n_plus_one)),
    );
    assert_approx_eq(
        refund,
        expected_refund,
        I64F64::from_num(0.001),
        "Refund for 50 tokens",
    );
    assert_eq!(curve.get_supply(), I64F64::from_num(50));
}

#[test]
fn test_exponential_buy_and_sell() {
    let mut curve = Exponential::new(2.0, 1.5).unwrap();
    // Initial state
    assert_eq!(curve.get_supply(), I64F64::from_num(0));
    assert_eq!(curve.get_price().unwrap(), I64F64::from_num(0));

    // Buy 100 tokens
    let cost = curve.buy_token(I64F64::from_num(100)).unwrap();
    let n_plus_one = 1.5 + 1.0;
    let expected_cost = I64F64::from_num((2.0 / n_plus_one) * (100.0_f64.powf(n_plus_one)));
    assert_approx_eq(
        cost,
        expected_cost,
        I64F64::from_num(0.001),
        "Cost for 100 tokens",
    );
    assert_eq!(curve.get_supply(), I64F64::from_num(100));
    let price_after_buy = curve.get_price().unwrap();
    assert_approx_eq(
        price_after_buy,
        I64F64::from_num(2.0 * 100.0_f64.powf(1.5)),
        I64F64::from_num(0.001),
        "Price after buy",
    );

    // Sell 50 tokens
    let refund = curve.sell_token(I64F64::from_num(50)).unwrap();
    let expected_refund = I64F64::from_num(
        (2.0 / n_plus_one) * (100.0_f64.powf(n_plus_one) - 50.0_f64.powf(n_plus_one)),
    );
    assert_approx_eq(
        refund,
        expected_refund,
        I64F64::from_num(0.001),
        "Refund for 50 tokens",
    );
    assert_eq!(curve.get_supply(), I64F64::from_num(50));
    let price_after_sell = curve.get_price().unwrap();
    assert_approx_eq(
        price_after_sell,
        I64F64::from_num(2.0 * 50.0_f64.powf(1.5)),
        I64F64::from_num(0.001),
        "Price after sell",
    );
}

#[test]
fn test_exponential_invalid_inputs() {
    let mut curve = Exponential::new(2.0, 1.5).unwrap();

    // Buy zero tokens
    let result = curve.buy_token(I64F64::from_num(0));
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Token amount must be positive")
    ));

    // Buy negative tokens
    let result = curve.buy_token(I64F64::from_num(-10));
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Token amount must be positive")
    ));

    // Sell zero tokens
    let result = curve.sell_token(I64F64::from_num(0));
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Invalid token amount")
    ));

    // Sell negative tokens
    let result = curve.sell_token(I64F64::from_num(-10));
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Invalid token amount")
    ));

    // Sell more than supply
    let result = curve.sell_token(I64F64::from_num(1));
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Invalid token amount")
    ));
}

#[test]
fn test_exponential_edge_cases() {
    // Small coefficient and exponent
    let mut curve = Exponential::new(0.0001, 0.5).unwrap();
    let cost = curve.buy_token(I64F64::from_num(0.0001)).unwrap();
    let expected_cost = I64F64::from_num((0.0001 / 1.5) * (0.0001_f64.powf(1.5)));
    assert_approx_eq(
        cost,
        expected_cost,
        I64F64::from_num(0.0000001),
        "Cost for small amount",
    );
    assert_approx_eq(
        curve.get_supply(),
        I64F64::from_num(0.0001),
        I64F64::from_num(0.0000001),
        "Supply after small buy",
    );

    // Large supply
    let mut curve = Exponential::new(1.0, 1.0).unwrap();
    curve.buy_token(I64F64::from_num(1000000)).unwrap();
    let price = curve.get_price().unwrap();
    let expected_price = I64F64::from_num(1.0 * 1000000.0);
    assert_approx_eq(
        price,
        expected_price,
        I64F64::from_num(0.001),
        "Price with large supply",
    );

    // Large exponent
    let mut curve = Exponential::new(1.0, 3.0).unwrap();
    curve.buy_token(I64F64::from_num(10)).unwrap();
    let price = curve.get_price().unwrap();
    let expected_price = I64F64::from_num(1.0 * 10.0_f64.powf(3.0));
    assert_approx_eq(
        price,
        expected_price,
        I64F64::from_num(0.001),
        "Price with large exponent",
    );
}
