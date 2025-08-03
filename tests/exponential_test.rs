use bonding_curves::{BondingCurve, Exponential};
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
