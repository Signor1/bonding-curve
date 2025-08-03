use crate::bonding_curve_trait::BondingCurve;
use crate::errors::BondingCurveError;
use fixed::types::I64F64;

#[derive(Clone, Debug)]
pub struct Exponential {
    pub coefficient: I64F64,
    pub exponent: I64F64,
    pub token_supply: I64F64,
}

/*
* P = c * S^n
* where:
* c is the coefficient (a scaling factor),
* S is the token_supply,
* n is the exponent (determining the curve’s steepness).
*/

impl Exponential {
    pub fn new(coefficient: f64, exponent: f64) -> Result<Self, BondingCurveError> {
        if coefficient <= 0.0
            || exponent <= 0.0
            || !coefficient.is_finite()
            || !exponent.is_finite()
        {
            return Err(BondingCurveError::InvalidInput(
                "Coefficient and exponent must be positive and finite".into(),
            ));
        }
        Ok(Self {
            coefficient: I64F64::from_num(coefficient),
            exponent: I64F64::from_num(exponent),
            token_supply: I64F64::from_num(0.0),
        })
    }

    // Helper function to compute x^y using libm
    fn pow_fixed(base: I64F64, exponent: I64F64) -> Result<I64F64, BondingCurveError> {
        let base_f64: f64 = base.to_num();
        let exp_f64: f64 = exponent.to_num();

        if base_f64 < 0.0 {
            return Err(BondingCurveError::CalculationError(
                "Cannot raise negative number to fractional power".into(),
            ));
        }

        let result = libm::pow(base_f64, exp_f64);

        if !result.is_finite() {
            return Err(BondingCurveError::CalculationError(
                "Power calculation resulted in infinite or NaN value".into(),
            ));
        }

        Ok(I64F64::from_num(result))
    }
}

impl BondingCurve for Exponential {
    fn get_price(&self) -> Result<I64F64, BondingCurveError> {
        let power_result = Self::pow_fixed(self.token_supply, self.exponent)?;
        Ok(self.coefficient * power_result)
    }

    fn buy_token(&mut self, token_amount: I64F64) -> Result<I64F64, BondingCurveError> {
        if token_amount <= I64F64::from_num(0) {
            return Err(BondingCurveError::InvalidInput(
                "Token amount must be positive".into(),
            ));
        }

        let n_plus_one = self.exponent + I64F64::from_num(1);
        let new_supply_power = Self::pow_fixed(self.token_supply + token_amount, n_plus_one)?;
        let current_supply_power = Self::pow_fixed(self.token_supply, n_plus_one)?;

        let cost = (self.coefficient / n_plus_one) * new_supply_power
            - (self.coefficient / n_plus_one) * current_supply_power;

        self.token_supply += token_amount;
        Ok(cost)
    }

    fn sell_token(&mut self, token_amount: I64F64) -> Result<I64F64, BondingCurveError> {
        if token_amount <= I64F64::from_num(0) || token_amount > self.token_supply {
            return Err(BondingCurveError::InvalidInput(
                "Invalid token amount".into(),
            ));
        }

        let n_plus_one = self.exponent + I64F64::from_num(1);
        let current_supply_power = Self::pow_fixed(self.token_supply, n_plus_one)?;
        let new_supply_power = Self::pow_fixed(self.token_supply - token_amount, n_plus_one)?;

        let refund = (self.coefficient / n_plus_one) * current_supply_power
            - (self.coefficient / n_plus_one) * new_supply_power;

        self.token_supply -= token_amount;
        Ok(refund)
    }

    fn get_supply(&self) -> I64F64 {
        self.token_supply
    }

    fn get_reserve(&self) -> Option<I64F64> {
        None
    }
}
