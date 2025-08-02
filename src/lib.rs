mod bancor;
mod bonding_curve_trait;
mod errors;

pub use self::bancor::Bancor;
pub use bonding_curve_trait::BondingCurve;
pub use errors::BondingCurveError;

#[cfg(test)]
mod tests {
    use super::*;
    use fixed::types::I64F64;

    #[test]
    fn test_bancor_valid() {
        let curve = bancor::Bancor::new(1000, 10000, 0.2).unwrap();
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
}
