use crate::bonding_curve_trait::BondingCurve;
use crate::errors::BondingCurveError;
use fixed::types::I64F64;

#[derive(Clone, Debug)]
pub struct Linear {
    pub slope: I64F64,
    pub token_supply: I64F64,
}

/*
* P = k * S
where:

*   P is the price of a token,
*   k is the slope (a constant parameter),
*   S is the current token supply.
*/

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

impl BondingCurve for Linear {
    fn get_price(&self) -> Result<I64F64, BondingCurveError> {
        Ok(self.slope * self.token_supply)
    }
    fn buy_token(&mut self, token_amount: I64F64) -> Result<I64F64, BondingCurveError> {
        if token_amount <= I64F64::from_num(0) {
            return Err(BondingCurveError::InvalidInput(
                "Token amount must be positive".into(),
            ));
        }
        // Cost = ∫(k*S)dS from S to S+ΔS = k * (S+ΔS)^2 / 2 - k * S^2 / 2
        let new_supply = self.token_supply + token_amount;
        let cost = self.slope * (new_supply * new_supply) / I64F64::from_num(2)
            - self.slope * (self.token_supply * self.token_supply) / I64F64::from_num(2);
        self.token_supply = new_supply;
        Ok(cost)
    }

    fn sell_token(&mut self, token_amount: I64F64) -> Result<I64F64, BondingCurveError> {
        if token_amount <= I64F64::from_num(0) || token_amount > self.token_supply {
            return Err(BondingCurveError::InvalidInput(
                "Invalid token amount".into(),
            ));
        }
        // Refund = ∫(k*S)dS from S-ΔS to S = k * S^2 / 2 - k * (S-ΔS)^2 / 2
        let new_supply = self.token_supply - token_amount;
        let refund = self.slope * (self.token_supply * self.token_supply) / I64F64::from_num(2)
            - self.slope * (new_supply * new_supply) / I64F64::from_num(2);
        self.token_supply = new_supply;
        Ok(refund)
    }

    fn get_supply(&self) -> I64F64 {
        self.token_supply
    }

    fn get_reserve(&self) -> Option<I64F64> {
        None
    }
}
