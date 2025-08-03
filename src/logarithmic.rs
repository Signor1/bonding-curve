use crate::bonding_curve_trait::BondingCurve;
use crate::errors::BondingCurveError;
use fixed::types::I64F64;

#[derive(Clone, Debug)]
pub struct Logarithmic {
    pub coefficient: I64F64,
    pub constant: I64F64,
    pub token_supply: I64F64,
}

/*
* P = c * ln(S + k)
* where:
*  c  is a coefficient (scaling factor),
*  S  is the token supply,
*  k  is a constant (often 1, to avoid issues with ln(0)),
*  ln  is the natural logarithm.
*/

impl Logarithmic {
    pub fn new(coefficient: f64, constant: f64) -> Result<Self, BondingCurveError> {
        if coefficient <= 0.0
            || constant <= 0.0
            || !coefficient.is_finite()
            || !constant.is_finite()
        {
            return Err(BondingCurveError::InvalidInput(
                "Coefficient and constant must be positive and finite".into(),
            ));
        }
        Ok(Logarithmic {
            coefficient: I64F64::from_num(coefficient),
            constant: I64F64::from_num(constant),
            token_supply: I64F64::from_num(0),
        })
    }

    // Helper function to compute natural logarithm using libm
    fn ln_fixed(value: I64F64) -> Result<I64F64, BondingCurveError> {
        let value_f64: f64 = value.to_num();

        if value_f64 <= 0.0 {
            return Err(BondingCurveError::CalculationError(
                "Cannot take logarithm of non-positive number".into(),
            ));
        }

        let result = libm::log(value_f64);

        if !result.is_finite() {
            return Err(BondingCurveError::CalculationError(
                "Logarithm calculation resulted in infinite or NaN value".into(),
            ));
        }

        Ok(I64F64::from_num(result))
    }
}

impl BondingCurve for Logarithmic {
    fn get_price(&self) -> Result<I64F64, BondingCurveError> {
        let supply_plus_const = self.token_supply + self.constant;
        if supply_plus_const <= I64F64::from_num(0) {
            return Err(BondingCurveError::CalculationError(
                "Invalid supply for logarithm".into(),
            ));
        }
        let ln_result = Self::ln_fixed(supply_plus_const)?;
        Ok(self.coefficient * ln_result)
    }

    fn buy_token(&mut self, token_amount: I64F64) -> Result<I64F64, BondingCurveError> {
        if token_amount <= I64F64::from_num(0) {
            return Err(BondingCurveError::InvalidInput(
                "Token amount must be positive".into(),
            ));
        }

        let s_new = self.token_supply + token_amount + self.constant;
        let s_old = self.token_supply + self.constant;

        let ln_s_new = Self::ln_fixed(s_new)?;
        let ln_s_old = Self::ln_fixed(s_old)?;

        let cost = self.coefficient * (s_new * ln_s_new - s_new)
            - self.coefficient * (s_old * ln_s_old - s_old);

        self.token_supply += token_amount;
        Ok(cost)
    }

    fn sell_token(&mut self, token_amount: I64F64) -> Result<I64F64, BondingCurveError> {
        if token_amount <= I64F64::from_num(0) || token_amount > self.token_supply {
            return Err(BondingCurveError::InvalidInput(
                "Invalid token amount".into(),
            ));
        }

        let s_old = self.token_supply + self.constant;
        let s_new = self.token_supply - token_amount + self.constant;

        if s_new <= I64F64::from_num(0) {
            return Err(BondingCurveError::CalculationError(
                "Cannot sell tokens: would result in invalid supply for logarithm".into(),
            ));
        }

        let ln_s_old = Self::ln_fixed(s_old)?;
        let ln_s_new = Self::ln_fixed(s_new)?;

        let refund = self.coefficient * (s_old * ln_s_old - s_old)
            - self.coefficient * (s_new * ln_s_new - s_new);

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
