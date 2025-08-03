use bonding_curves::{BondingCurve, Linear};
use fixed::types::I64F64;

// helper function
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
fn test_linear_new() {
    // Valid slope
    let curve = Linear::new(0.01).unwrap();
    assert_eq!(curve.get_supply(), I64F64::from_num(0));
    assert_eq!(curve.get_price().unwrap(), I64F64::from_num(0));

    // Invalid slope (zero)
    let result = Linear::new(0.0);
    assert!(matches!(
        result,
        Err(bonding_curves::BondingCurveError::InvalidInput(msg)) if msg.contains("Slope must be positive")
    ));

    // Invalid slope (negative)
    let result = Linear::new(-0.01);
    assert!(matches!(
        result,
        Err(bonding_curves::BondingCurveError::InvalidInput(msg)) if msg.contains("Slope must be positive")
    ));
}

#[test]
fn test_linear_price() {
    let mut curve = Linear::new(0.01).unwrap();
    // Initial price (supply = 0)
    assert_eq!(curve.get_price().unwrap(), I64F64::from_num(0));

    // Price after buying 100 tokens
    curve.buy_token(I64F64::from_num(100)).unwrap();
    let expected_price = I64F64::from_num(0.01 * 100.0); // P = k * S = 0.01 * 100
    assert_approx_eq(
        curve.get_price().unwrap(),
        expected_price,
        I64F64::from_num(0.0000001),
        "Price after buy",
    );

    // Price after buying more tokens
    curve.buy_token(I64F64::from_num(100)).unwrap();
    let expected_price = I64F64::from_num(0.01 * 200.0); // P = k * S = 0.01 * 200
    assert_approx_eq(
        curve.get_price().unwrap(),
        expected_price,
        I64F64::from_num(0.0000001),
        "Price after second buy",
    );
}

#[test]
fn test_linear_buy_token() {
    let mut curve = Linear::new(0.01).unwrap();
    // Buy 100 tokens
    let cost = curve.buy_token(I64F64::from_num(100)).unwrap();
    let expected_cost = I64F64::from_num(0.01 * (100.0 * 100.0) / 2.0);
    // k * S^2 / 2 = 0.01 * 100^2 / 2 = 50
    assert_approx_eq(
        cost,
        expected_cost,
        I64F64::from_num(0.0000001),
        "Cost for 100 tokens",
    );
    assert_eq!(curve.get_supply(), I64F64::from_num(100));

    // Buy another 50 tokens
    let cost = curve.buy_token(I64F64::from_num(50)).unwrap();
    let expected_cost =
        I64F64::from_num((0.01 * (150.0 * 150.0) / 2.0) - (0.01 * (100.0 * 100.0) / 2.0));
    // k * (150^2 - 100^2) / 2 = 0.01 * (22500 - 10000) / 2 = 62.5
    assert_approx_eq(
        cost,
        expected_cost,
        I64F64::from_num(0.0000001),
        "Cost for additional 50 tokens",
    );
    assert_eq!(curve.get_supply(), I64F64::from_num(150));
}

#[test]
fn test_linear_sell_token() {
    let mut curve = Linear::new(0.01).unwrap();
    curve.buy_token(I64F64::from_num(100)).unwrap(); // Supply = 100
    let refund = curve.sell_token(I64F64::from_num(100)).unwrap();
    let expected_refund = I64F64::from_num(0.01 * (100.0 * 100.0) / 2.0); // k * (100^2 - 0^2) / 2 = 0.01 * 10000 / 2 = 50
    assert_approx_eq(
        refund,
        expected_refund,
        I64F64::from_num(0.0000001),
        "Refund for 100 tokens",
    );
    assert_eq!(curve.get_supply(), I64F64::from_num(0));

    // Buy 200 tokens, sell 50
    curve.buy_token(I64F64::from_num(200)).unwrap(); // Supply = 200
    let refund = curve.sell_token(I64F64::from_num(50)).unwrap();
    let expected_refund =
        I64F64::from_num((0.01 * (200.0 * 200.0) / 2.0) - (0.01 * (150.0 * 150.0) / 2.0)); // k * (200^2 - 150^2) / 2 = 0.01 * (40000 - 22500) / 2 = 87.5
    assert_approx_eq(
        refund,
        expected_refund,
        I64F64::from_num(0.0000001),
        "Refund for 50 tokens",
    );
    assert_eq!(curve.get_supply(), I64F64::from_num(150));
}

#[test]
fn test_linear_buy_and_sell() {
    let mut curve = Linear::new(0.01).unwrap();
    // Initial state
    assert_eq!(curve.get_supply(), I64F64::from_num(0));
    assert_eq!(curve.get_price().unwrap(), I64F64::from_num(0));

    // Buy 100 tokens
    let cost = curve.buy_token(I64F64::from_num(100)).unwrap();
    let expected_cost = I64F64::from_num(0.01 * (100.0 * 100.0) / 2.0); // 50
    assert_approx_eq(
        cost,
        expected_cost,
        I64F64::from_num(0.0000001),
        "Cost for 100 tokens",
    );
    assert_eq!(curve.get_supply(), I64F64::from_num(100));
    let price_after_buy = curve.get_price().unwrap();
    assert_approx_eq(
        price_after_buy,
        I64F64::from_num(0.01 * 100.0),
        I64F64::from_num(0.0000001),
        "Price after buy",
    );

    // Sell 50 tokens
    let refund = curve.sell_token(I64F64::from_num(50)).unwrap();
    let expected_refund =
        I64F64::from_num((0.01 * (100.0 * 100.0) / 2.0) - (0.01 * (50.0 * 50.0) / 2.0));
    // k * (100^2 - 50^2) / 2 = 0.01 * (10000 - 2500) / 2 = 37.5
    assert_approx_eq(
        refund,
        expected_refund,
        I64F64::from_num(0.0000001),
        "Refund for 50 tokens",
    );
    assert_eq!(curve.get_supply(), I64F64::from_num(50));
    let price_after_sell = curve.get_price().unwrap();
    assert_approx_eq(
        price_after_sell,
        I64F64::from_num(0.01 * 50.0),
        I64F64::from_num(0.0000001),
        "Price after sell",
    );
}

#[test]
fn test_linear_invalid_inputs() {
    let mut curve = Linear::new(0.01).unwrap();

    // Buy zero tokens
    let result = curve.buy_token(I64F64::from_num(0));
    assert!(matches!(
        result,
        Err(bonding_curves::BondingCurveError::InvalidInput(msg)) if msg.contains("Token amount must be positive")
    ));

    // Buy negative tokens
    let result = curve.buy_token(I64F64::from_num(-10));
    assert!(matches!(
        result,
        Err(bonding_curves::BondingCurveError::InvalidInput(msg)) if msg.contains("Token amount must be positive")
    ));

    // Sell zero tokens
    let result = curve.sell_token(I64F64::from_num(0));
    assert!(matches!(
        result,
        Err(bonding_curves::BondingCurveError::InvalidInput(msg)) if msg.contains("Invalid token amount")
    ));

    // Sell negative tokens
    let result = curve.sell_token(I64F64::from_num(-10));
    assert!(matches!(
        result,
        Err(bonding_curves::BondingCurveError::InvalidInput(msg)) if msg.contains("Invalid token amount")
    ));

    // Sell more tokens than supply
    curve.buy_token(I64F64::from_num(100)).unwrap();
    let result = curve.sell_token(I64F64::from_num(101));
    assert!(matches!(
        result,
        Err(bonding_curves::BondingCurveError::InvalidInput(msg)) if msg.contains("Invalid token amount")
    ));
}

#[test]
fn test_linear_edge_cases() {
    let mut curve = Linear::new(0.01).unwrap();

    // Buy a very small amount
    let cost = curve.buy_token(I64F64::from_num(0.0001)).unwrap();
    let expected_cost = I64F64::from_num(0.01 * (0.0001 * 0.0001) / 2.0); // k * (0.0001^2) / 2
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

    // Buy a large amount
    let cost = curve.buy_token(I64F64::from_num(1000000)).unwrap();
    let expected_cost = I64F64::from_num(
        (0.01 * (1000000.0001 * 1000000.0001) / 2.0) - (0.01 * (0.0001 * 0.0001) / 2.0),
    );
    assert_approx_eq(
        cost,
        expected_cost,
        I64F64::from_num(0.0001),
        "Cost for large amount",
    );
    assert_approx_eq(
        curve.get_supply(),
        I64F64::from_num(1000000.0001),
        I64F64::from_num(0.0000001),
        "Supply after large buy",
    );
}

#[test]
fn test_linear_precision() {
    // Test with a very small slope to check precision
    let mut curve = Linear::new(0.0000001).unwrap();
    let cost = curve.buy_token(I64F64::from_num(100)).unwrap();
    let expected_cost = I64F64::from_num(0.0000001 * (100.0 * 100.0) / 2.0); // 0.0000001 * 100^2 / 2 = 0.0005
    assert_approx_eq(
        cost,
        expected_cost,
        I64F64::from_num(0.0000001),
        "Cost with small slope",
    );
    assert_eq!(curve.get_supply(), I64F64::from_num(100));

    // Test with large supply
    curve.buy_token(I64F64::from_num(1000000)).unwrap();
    let price = curve.get_price().unwrap();
    let expected_price = I64F64::from_num(0.0000001 * 1000100.0); // k * S
    assert_approx_eq(
        price,
        expected_price,
        I64F64::from_num(0.0000001),
        "Price with large supply",
    );
}
