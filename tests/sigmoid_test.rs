use bonding_curves::{BondingCurve, BondingCurveError, Sigmoid};
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
fn test_sigmoid_new_valid() {
    let curve = Sigmoid::new(100.0, 0.1, 50.0).unwrap();
    assert_eq!(curve.get_supply(), I64F64::from_num(0));

    let expected_price = I64F64::from_num(100.0 / (1.0 + (-0.1_f64 * (0.0 - 50.0)).exp()));
    assert_approx_eq(
        curve.get_price().unwrap(),
        expected_price,
        I64F64::from_num(0.001),
        "Initial price",
    );
    assert_eq!(curve.get_reserve(), None);
}

#[test]
fn test_sigmoid_new_invalid() {
    // Zero max_price
    let result = Sigmoid::new(0.0, 0.1, 50.0);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Invalid parameters")
    ));

    // Negative max_price
    let result = Sigmoid::new(-1.0, 0.1, 50.0);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Invalid parameters")
    ));

    // Zero steepness
    let result = Sigmoid::new(100.0, 0.0, 50.0);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Invalid parameters")
    ));

    // Negative steepness
    let result = Sigmoid::new(100.0, -0.1, 50.0);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Invalid parameters")
    ));

    // Negative midpoint
    let result = Sigmoid::new(100.0, 0.1, -1.0);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Invalid parameters")
    ));

    // NaN max_price
    let result = Sigmoid::new(f64::NAN, 0.1, 50.0);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Invalid parameters")
    ));

    // Infinity steepness
    let result = Sigmoid::new(100.0, f64::INFINITY, 50.0);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Invalid parameters")
    ));
}

#[test]
fn test_sigmoid_price() {
    let mut curve = Sigmoid::new(100.0, 0.1, 50.0).unwrap();
    // Price at supply = 0
    let expected_price = I64F64::from_num(100.0 / (1.0 + (-0.1_f64 * (0.0 - 50.0)).exp()));
    assert_approx_eq(
        curve.get_price().unwrap(),
        expected_price,
        I64F64::from_num(0.001),
        "Price at supply 0",
    );

    // Price after buying 50 tokens (at midpoint)
    curve.buy_token(I64F64::from_num(50)).unwrap();
    let expected_price = I64F64::from_num(100.0 / (1.0 + (-0.1_f64 * (50.0 - 50.0)).exp()));
    assert_approx_eq(
        curve.get_price().unwrap(),
        expected_price,
        I64F64::from_num(0.001),
        "Price at midpoint",
    );

    // Price after buying another 50 tokens
    curve.buy_token(I64F64::from_num(50)).unwrap();
    let expected_price = I64F64::from_num(100.0 / (1.0 + (-0.1_f64 * (100.0 - 50.0)).exp()));
    assert_approx_eq(
        curve.get_price().unwrap(),
        expected_price,
        I64F64::from_num(0.001),
        "Price after 100 tokens",
    );
}

#[test]
fn test_sigmoid_buy_token() {
    let mut curve = Sigmoid::new(100.0, 0.1, 50.0).unwrap();
    // Buy 50 tokens
    let cost = curve.buy_token(I64F64::from_num(50)).unwrap();
    let k = 0.1_f64;
    let s_new = 50.0 - 50.0; // s_new = supply + amount - midpoint
    let s_old = 0.0 - 50.0; // s_old = supply - midpoint
    let expected_cost = I64F64::from_num(
        (100.0 / k) * (1.0 + (k * s_new).exp()).ln() - (100.0 / k) * (1.0 + (k * s_old).exp()).ln(),
    );
    assert_approx_eq(
        cost,
        expected_cost,
        I64F64::from_num(0.001),
        "Cost for 50 tokens",
    );
    assert_eq!(curve.get_supply(), I64F64::from_num(50));

    // Buy another 50 tokens
    let cost = curve.buy_token(I64F64::from_num(50)).unwrap();
    let s_new = 100.0 - 50.0;
    let s_old = 50.0 - 50.0;
    let expected_cost = I64F64::from_num(
        (100.0 / k) * (1.0 + (k * s_new).exp()).ln() - (100.0 / k) * (1.0 + (k * s_old).exp()).ln(),
    );
    assert_approx_eq(
        cost,
        expected_cost,
        I64F64::from_num(0.001),
        "Cost for additional 50 tokens",
    );
    assert_eq!(curve.get_supply(), I64F64::from_num(100));
}

#[test]
fn test_sigmoid_sell_token() {
    let mut curve = Sigmoid::new(100.0, 0.1, 50.0).unwrap();
    curve.buy_token(I64F64::from_num(100)).unwrap();
    let refund = curve.sell_token(I64F64::from_num(50)).unwrap();
    let k = 0.1_f64;
    let s_old = 100.0 - 50.0;
    let s_new = 50.0 - 50.0;
    let expected_refund = I64F64::from_num(
        (100.0 / k) * (1.0 + (k * s_old).exp()).ln() - (100.0 / k) * (1.0 + (k * s_new).exp()).ln(),
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
fn test_sigmoid_buy_and_sell() {
    let mut curve = Sigmoid::new(100.0, 0.1, 50.0).unwrap();
    // Initial state
    assert_eq!(curve.get_supply(), I64F64::from_num(0));
    let expected_price = I64F64::from_num(100.0 / (1.0 + (-0.1_f64 * (0.0 - 50.0)).exp()));
    assert_approx_eq(
        curve.get_price().unwrap(),
        expected_price,
        I64F64::from_num(0.001),
        "Initial price",
    );

    // Buy 50 tokens
    let k = 0.1_f64;
    let cost = curve.buy_token(I64F64::from_num(50)).unwrap();
    let s_new = 50.0 - 50.0;
    let s_old = 0.0 - 50.0;
    let expected_cost = I64F64::from_num(
        (100.0 / k) * (1.0 + (k * s_new).exp()).ln() - (100.0 / k) * (1.0 + (k * s_old).exp()).ln(),
    );
    assert_approx_eq(
        cost,
        expected_cost,
        I64F64::from_num(0.001),
        "Cost for 50 tokens",
    );
    assert_eq!(curve.get_supply(), I64F64::from_num(50));
    let price_after_buy = curve.get_price().unwrap();
    let expected_price = I64F64::from_num(100.0 / (1.0 + (-0.1_f64 * (50.0 - 50.0)).exp()));
    assert_approx_eq(
        price_after_buy,
        expected_price,
        I64F64::from_num(0.001),
        "Price after buy",
    );

    // Sell 25 tokens
    let refund = curve.sell_token(I64F64::from_num(25)).unwrap();
    let s_old = 50.0 - 50.0;
    let s_new = 25.0 - 50.0;
    let expected_refund = I64F64::from_num(
        (100.0 / k) * (1.0 + (k * s_old).exp()).ln() - (100.0 / k) * (1.0 + (k * s_new).exp()).ln(),
    );
    assert_approx_eq(
        refund,
        expected_refund,
        I64F64::from_num(0.001),
        "Refund for 25 tokens",
    );
    assert_eq!(curve.get_supply(), I64F64::from_num(25));
    let price_after_sell = curve.get_price().unwrap();
    let expected_price = I64F64::from_num(100.0 / (1.0 + (-0.1_f64 * (25.0 - 50.0)).exp()));
    assert_approx_eq(
        price_after_sell,
        expected_price,
        I64F64::from_num(0.001),
        "Price after sell",
    );
}

#[test]
fn test_sigmoid_edge_cases() {
    // Small steepness
    let mut curve = Sigmoid::new(100.0, 0.0001, 50.0).unwrap();
    let cost = curve.buy_token(I64F64::from_num(10)).unwrap();
    let k = 0.0001_f64;
    let s_new = 10.0 - 50.0;
    let s_old = 0.0 - 50.0;
    let expected_cost = I64F64::from_num(
        (100.0 / k) * (1.0 + (k * s_new).exp()).ln() - (100.0 / k) * (1.0 + (k * s_old).exp()).ln(),
    );
    assert_approx_eq(
        cost,
        expected_cost,
        I64F64::from_num(0.001),
        "Cost for small steepness",
    );
    assert_eq!(curve.get_supply(), I64F64::from_num(10));

    // Large steepness
    let mut curve = Sigmoid::new(100.0, 1.0, 50.0).unwrap();
    let cost = curve.buy_token(I64F64::from_num(10)).unwrap();
    let k = 1.0_f64;
    let s_new = 10.0 - 50.0;
    let s_old = 0.0 - 50.0;
    let expected_cost = I64F64::from_num(
        (100.0 / k) * (1.0 + (k * s_new).exp()).ln() - (100.0 / k) * (1.0 + (k * s_old).exp()).ln(),
    );
    assert_approx_eq(
        cost,
        expected_cost,
        I64F64::from_num(0.001),
        "Cost for large steepness",
    );
}
