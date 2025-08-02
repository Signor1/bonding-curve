use bonding_curves::Bancor;
use bonding_curves::BondingCurve;
use fixed::types::I64F64;

#[test]
fn test_bancor_valid() {
    let curve = Bancor::new(1000, 10000, 0.2).unwrap();
    let price = curve.get_price().unwrap();
    let expected = I64F64::from_num(0.5);
    let tolerance = I64F64::from_num(0.0000001);
    assert!(
        (price - expected).abs() < tolerance,
        "Price {} is not approximately equal to {}",
        price,
        expected
    );
}
