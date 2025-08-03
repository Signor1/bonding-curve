use bonding_curves::{BondingCurve, BondingCurveError, Logarithmic};
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
fn test_logarithmic_new_valid() {
    let curve = Logarithmic::new(2.0, 1.0).unwrap();
    assert_eq!(curve.get_supply(), I64F64::from_num(0));
    assert_approx_eq(
        curve.get_price().unwrap(),
        I64F64::from_num(2.0 * 1.0f64.ln()),
        I64F64::from_num(0.001),
        "Initial price",
    );
    assert_eq!(curve.get_reserve(), None);
}

#[test]
fn test_logarithmic_new_invalid() {
    // Zero coefficient
    let result = Logarithmic::new(0.0, 1.0);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Coefficient and constant must be positive and finite")
    ));

    // Negative coefficient
    let result = Logarithmic::new(-1.0, 1.0);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Coefficient and constant must be positive and finite")
    ));

    // Zero constant
    let result = Logarithmic::new(2.0, 0.0);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Coefficient and constant must be positive and finite")
    ));

    // Negative constant
    let result = Logarithmic::new(2.0, -1.0);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Coefficient and constant must be positive and finite")
    ));

    // NaN coefficient
    let result = Logarithmic::new(f64::NAN, 1.0);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Coefficient and constant must be positive and finite")
    ));

    // Infinity constant
    let result = Logarithmic::new(2.0, f64::INFINITY);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Coefficient and constant must be positive and finite")
    ));
}

#[test]
fn test_logarithmic_price() {
    let mut curve = Logarithmic::new(2.0, 1.0).unwrap();
    // Price at supply = 0
    assert_approx_eq(
        curve.get_price().unwrap(),
        I64F64::from_num(2.0 * 1.0f64.ln()),
        I64F64::from_num(0.001),
        "Price at supply 0",
    );

    // Price after buying 100 tokens
    curve.buy_token(I64F64::from_num(100)).unwrap();
    let expected_price = I64F64::from_num(2.0 * 101.0f64.ln());
    assert_approx_eq(
        curve.get_price().unwrap(),
        expected_price,
        I64F64::from_num(0.001),
        "Price after 100 tokens",
    );

    // Price after buying another 100 tokens
    curve.buy_token(I64F64::from_num(100)).unwrap();
    let expected_price = I64F64::from_num(2.0 * 201.0f64.ln());
    assert_approx_eq(
        curve.get_price().unwrap(),
        expected_price,
        I64F64::from_num(0.001),
        "Price after 200 tokens",
    );
}

#[test]
fn test_logarithmic_buy_tokens() {
    let mut curve = Logarithmic::new(2.0, 1.0).unwrap();
    // Buy 100 tokens
    let cost = curve.buy_token(I64F64::from_num(100)).unwrap();
    let expected_cost =
        I64F64::from_num(2.0 * (101.0f64.ln() * 101.0 - 101.0) - 2.0 * (1.0f64.ln() * 1.0 - 1.0));
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
        2.0 * (151.0f64.ln() * 151.0 - 151.0) - 2.0 * (101.0f64.ln() * 101.0 - 101.0),
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
fn test_logarithmic_sell_tokens() {
    let mut curve = Logarithmic::new(2.0, 1.0).unwrap();
    curve.buy_token(I64F64::from_num(100)).unwrap();
    let refund = curve.sell_token(I64F64::from_num(50)).unwrap();
    let expected_refund = I64F64::from_num(
        2.0 * (101.0f64.ln() * 101.0 - 101.0) - 2.0 * (51.0f64.ln() * 51.0 - 51.0),
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
fn test_logarithmic_buy_and_sell() {
    let mut curve = Logarithmic::new(2.0, 1.0).unwrap();
    // Initial state
    assert_eq!(curve.get_supply(), I64F64::from_num(0));
    assert_approx_eq(
        curve.get_price().unwrap(),
        I64F64::from_num(2.0 * 1.0f64.ln()),
        I64F64::from_num(0.001),
        "Initial price",
    );

    // Buy 100 tokens
    let cost = curve.buy_token(I64F64::from_num(100)).unwrap();
    let expected_cost =
        I64F64::from_num(2.0 * (101.0f64.ln() * 101.0 - 101.0) - 2.0 * (1.0f64.ln() * 1.0 - 1.0));
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
        I64F64::from_num(2.0 * 101.0f64.ln()),
        I64F64::from_num(0.001),
        "Price after buy",
    );

    // Sell 50 tokens
    let refund = curve.sell_token(I64F64::from_num(50)).unwrap();
    let expected_refund = I64F64::from_num(
        2.0 * (101.0f64.ln() * 101.0 - 101.0) - 2.0 * (51.0f64.ln() * 51.0 - 51.0),
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
        I64F64::from_num(2.0 * 51.0f64.ln()),
        I64F64::from_num(0.001),
        "Price after sell",
    );
}

#[test]
fn test_logarithmic_edge_cases() {
    // Small coefficient and constant
    let mut curve = Logarithmic::new(0.0001, 0.0001).unwrap();
    let cost = curve.buy_token(I64F64::from_num(0.0001)).unwrap();
    let expected_cost = I64F64::from_num(
        0.0001 * (0.0002f64.ln() * 0.0002 - 0.0002) - 0.0001 * (0.0001f64.ln() * 0.0001 - 0.0001),
    );
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
    let mut curve = Logarithmic::new(1.0, 1.0).unwrap();
    curve.buy_token(I64F64::from_num(1000000)).unwrap();
    let price = curve.get_price().unwrap();
    let expected_price = I64F64::from_num(1.0 * 1000001.0f64.ln());
    assert_approx_eq(
        price,
        expected_price,
        I64F64::from_num(0.001),
        "Price with large supply",
    );
}
