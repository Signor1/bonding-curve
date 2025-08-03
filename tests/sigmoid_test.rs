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
