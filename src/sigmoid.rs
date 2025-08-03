use crate::bonding_curve_trait::BondingCurve;
use crate::errors::BondingCurveError;
use fixed::types::I64F64;

#[derive(Clone, Debug)]
pub struct Sigmoid {
    pub max_price: I64F64,
    pub steepness: I64F64,
    pub midpoint: I64F64,
    pub token_supply: I64F64,
}

/*
* P = M / (1 + e^-k(S - m))
* where:
* M is max_price,
* k is steepness,
* m is midpoint,
* S is token_supply.
*/

impl Sigmoid {
    pub fn new(max_price: f64, steepness: f64, midpoint: f64) -> Result<Self, BondingCurveError> {
        if max_price <= 0.0
            || steepness <= 0.0
            || midpoint < 0.0
            || !max_price.is_finite()
            || !steepness.is_finite()
            || !midpoint.is_finite()
        {
            return Err(BondingCurveError::InvalidInput("Invalid parameters".into()));
        }
        Ok(Sigmoid {
            max_price: I64F64::from_num(max_price),
            steepness: I64F64::from_num(steepness),
            midpoint: I64F64::from_num(midpoint),
            token_supply: I64F64::from_num(0),
        })
    }

    // Helper function to compute exponential using libm
    fn exp_fixed(value: I64F64) -> Result<I64F64, BondingCurveError> {
        let value_f64: f64 = value.to_num();
        let result = libm::exp(value_f64);

        if !result.is_finite() {
            return Err(BondingCurveError::CalculationError(
                "Exponential calculation resulted in infinite or NaN value".into(),
            ));
        }

        Ok(I64F64::from_num(result))
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

impl BondingCurve for Sigmoid {
    fn get_price(&self) -> Result<I64F64, BondingCurveError> {
        let exponent = -self.steepness * (self.token_supply - self.midpoint);
        let exp_result = Self::exp_fixed(exponent)?;
        let denominator = I64F64::from_num(1) + exp_result;
        Ok(self.max_price / denominator)
    }

    fn buy_token(&mut self, token_amount: I64F64) -> Result<I64F64, BondingCurveError> {
        if token_amount <= I64F64::from_num(0) {
            return Err(BondingCurveError::InvalidInput(
                "Token amount must be positive".into(),
            ));
        }

        let k = self.steepness;
        let s_new = self.token_supply + token_amount - self.midpoint;
        let s_old = self.token_supply - self.midpoint;

        let exp_k_s_new = Self::exp_fixed(k * s_new)?;
        let exp_k_s_old = Self::exp_fixed(k * s_old)?;

        let ln_term_new = Self::ln_fixed(I64F64::from_num(1) + exp_k_s_new)?;
        let ln_term_old = Self::ln_fixed(I64F64::from_num(1) + exp_k_s_old)?;

        let cost = (self.max_price / k) * ln_term_new - (self.max_price / k) * ln_term_old;

        self.token_supply += token_amount;
        Ok(cost)
    }

    fn sell_token(&mut self, token_amount: I64F64) -> Result<I64F64, BondingCurveError> {
        if token_amount <= I64F64::from_num(0) || token_amount > self.token_supply {
            return Err(BondingCurveError::InvalidInput(
                "Invalid token amount".into(),
            ));
        }

        let k = self.steepness;
        let s_old = self.token_supply - self.midpoint;
        let s_new = self.token_supply - token_amount - self.midpoint;

        let exp_k_s_old = Self::exp_fixed(k * s_old)?;
        let exp_k_s_new = Self::exp_fixed(k * s_new)?;

        let ln_term_old = Self::ln_fixed(I64F64::from_num(1) + exp_k_s_old)?;
        let ln_term_new = Self::ln_fixed(I64F64::from_num(1) + exp_k_s_new)?;

        let refund = (self.max_price / k) * ln_term_old - (self.max_price / k) * ln_term_new;

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
