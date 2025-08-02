use crate::bonding_curve_trait::BondingCurve;
use crate::errors::BondingCurveError;
use fixed::types::I64F64;

#[derive(Clone, Debug)]
pub struct Linear {
    pub slope: I64F64,
    pub token_supply: I64F64,
}

impl Linear {
    pub fn new(slope: f64) -> Result<Self, BondingCurveError> {
        if slope <= 0.0 {
            return Err(BondingCurveError::InvalidInput(
                "Slope must be positive".into(),
            ));
        }
        Ok(Linear {
            slope: I64F64::from_num(slope),
            token_supply: I64F64::from_num(0),
        })
    }
}
