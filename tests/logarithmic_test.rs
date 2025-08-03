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
