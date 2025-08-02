use bonding_curves::{Bancor, BondingCurve, BondingCurveError};
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
fn test_bancor_valid() {
    let curve = Bancor::new(1000, 10000, 0.2).unwrap();
    let price = curve.get_price().unwrap();
    let expected = I64F64::from_num(0.5);
    assert_approx_eq(price, expected, I64F64::from_num(0.0000001), "Price");
}

#[test]
fn test_bancor_zero_supply_nonzero_reserve() {
    let result = Bancor::new(500, 0, 0.2);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("zero token supply")
    ))
}

#[test]
fn test_bancor_zero_reserve_nonzero_supply() {
    let result = Bancor::new(0, 1000, 0.2);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("zero reserve with non-zero token supply")
    ));
}

#[test]
fn test_bancor_zero_state() {
    let curve = Bancor::new(0, 0, 0.2).unwrap();
    assert_eq!(curve.get_supply(), I64F64::from_num(0));
    assert_eq!(curve.get_reserve().unwrap(), I64F64::from_num(0));
}

#[test]
fn test_bancor_negative_values() {
    let result = Bancor::new(-100, 10000, 0.2);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("non-negative")
    ));

    let result = Bancor::new(1000, -100, 0.2);
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("non-negative")
    ));
}

#[test]
fn test_bancor_buy_tokens() {
    let mut curve = Bancor::new(1000, 10000, 0.2).unwrap();
    let tokens = curve.buy_token(I64F64::from_num(100)).unwrap();
    let expected_tokens = I64F64::from_num(200);
    assert_approx_eq(
        tokens,
        expected_tokens,
        I64F64::from_num(0.0000001),
        "Tokens issued",
    );

    let supply = curve.get_supply();
    let expected_supply = I64F64::from_num(10200);
    assert_approx_eq(
        supply,
        expected_supply,
        I64F64::from_num(0.0000001),
        "Supply",
    );
    assert_eq!(curve.get_reserve().unwrap(), I64F64::from_num(1100));

    let new_price = curve.get_price().unwrap();
    let expected_price = I64F64::from_num(1100) / (I64F64::from_num(10200) * I64F64::from_num(0.2));
    assert_approx_eq(
        new_price,
        expected_price,
        I64F64::from_num(0.0000001),
        "New price",
    );
}

#[test]
fn test_bancor_sell_tokens() {
    let mut curve = Bancor::new(1000, 10000, 0.2).unwrap();
    let tokens = curve.sell_token(I64F64::from_num(200)).unwrap();
    let expected_tokens = I64F64::from_num(100);
    assert_approx_eq(
        tokens,
        expected_tokens,
        I64F64::from_num(0.0000001),
        "Reserve received",
    );

    let supply = curve.get_supply();
    let expected_supply = I64F64::from_num(9800);
    assert_approx_eq(
        supply,
        expected_supply,
        I64F64::from_num(0.0000001),
        "Supply",
    );

    let reserve = curve.get_reserve().unwrap();
    let expected_reserve = I64F64::from_num(900);
    assert_approx_eq(
        reserve,
        expected_reserve,
        I64F64::from_num(0.0000001),
        "Reserve",
    );

    let new_price = curve.get_price().unwrap();
    let expected_price = I64F64::from_num(900) / (I64F64::from_num(9800) * I64F64::from_num(0.2));
    assert_approx_eq(
        new_price,
        expected_price,
        I64F64::from_num(0.0000001),
        "New price",
    );
}

#[test]
fn test_bancor_buy_and_sell_token() {
    let mut curve = Bancor::new(1000, 10000, 0.2).unwrap();
    // Initial state
    assert_eq!(curve.get_supply(), I64F64::from_num(10000));
    assert_eq!(curve.get_reserve().unwrap(), I64F64::from_num(1000));
    let initial_price = curve.get_price().unwrap();
    assert_approx_eq(
        initial_price,
        I64F64::from_num(0.5),
        I64F64::from_num(0.0000001),
        "Initial price",
    );

    // Buy 100 reserve worth of tokens
    let tokens_bought = curve.buy_token(I64F64::from_num(100)).unwrap();
    let expected_tokens_bought = I64F64::from_num(200); // 100 / 0.5
    assert_approx_eq(
        tokens_bought,
        expected_tokens_bought,
        I64F64::from_num(0.0000001),
        "Tokens bought",
    );
    assert_approx_eq(
        curve.get_supply(),
        I64F64::from_num(10200),
        I64F64::from_num(0.0000001),
        "Supply after buy",
    );
    assert_eq!(curve.get_reserve().unwrap(), I64F64::from_num(1100));

    let price_after_buy = curve.get_price().unwrap();
    let expected_price_after_buy =
        I64F64::from_num(1100) / (I64F64::from_num(10200) * I64F64::from_num(0.2));
    assert_approx_eq(
        price_after_buy,
        expected_price_after_buy,
        I64F64::from_num(0.0000001),
        "Price after buy",
    );

    // Sell 100 tokens
    let reserve_received = curve.sell_token(I64F64::from_num(100)).unwrap();
    let expected_reserve_received = I64F64::from_num(100) * price_after_buy;
    assert_approx_eq(
        reserve_received,
        expected_reserve_received,
        I64F64::from_num(0.0000001),
        "Reserve received",
    );
    assert_approx_eq(
        curve.get_supply(),
        I64F64::from_num(10100),
        I64F64::from_num(0.0000001),
        "Supply after sell",
    );
    assert_eq!(
        curve.get_reserve().unwrap(),
        I64F64::from_num(1100) - reserve_received
    );
    let final_price = curve.get_price().unwrap();
    let expected_final_price = (I64F64::from_num(1100) - reserve_received)
        / (I64F64::from_num(10100) * I64F64::from_num(0.2));
    assert_approx_eq(
        final_price,
        expected_final_price,
        I64F64::from_num(0.0000001),
        "Final price",
    );
}

#[test]
fn test_bancor_insufficient_reserve() {
    let mut curve = Bancor::new(100, 10000, 0.2).unwrap();
    let result = curve.sell_token(I64F64::from_num(50000)); // Price = 0.05, reserve needed = 500
    assert!(result.is_err());
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("token amount")
    ));
}

#[test]
fn test_bancor_invalid_buy_inputs() {
    let mut curve = Bancor::new(1000, 10000, 0.2).unwrap();
    let result = curve.buy_token(I64F64::from_num(0));
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("amount must be positive")
    ));

    let result = curve.buy_token(I64F64::from_num(-100));
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Reserve amount must be positive")
    ));
}

#[test]
fn test_bancor_invalid_sell_inputs() {
    let mut curve = Bancor::new(1000, 10000, 0.2).unwrap();
    let result = curve.sell_token(I64F64::from_num(0));
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Invalid token amount")
    ));

    let result = curve.sell_token(I64F64::from_num(-100));
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Invalid token amount")
    ));

    let result = curve.sell_token(I64F64::from_num(20000));
    assert!(matches!(
        result,
        Err(BondingCurveError::InvalidInput(msg)) if msg.contains("Invalid token amount")
    ));
}
