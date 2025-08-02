use crate::bonding_curve_trait::BondingCurve;
use crate::errors::BondingCurveError;
use fixed::types::I64F64;

#[derive(Clone, Debug)]
pub struct Bancor {
    pub reserve_balance: I64F64,
    pub token_supply: I64F64,
    pub connector_weight: I64F64,
}

/*
 *  P (price) = reserve_balance / (token_supply * connector_weight)
*/
impl Bancor {
    pub fn new(
        reserve_balance: i64,
        token_supply: i64,
        connector_weight: f64,
    ) -> Result<Self, BondingCurveError> {
        if connector_weight <= 0.0 || connector_weight > 1.0 {
            return Err(BondingCurveError::InvalidInput(
                "Connector weight must be between 0 and 1".into(),
            ));
        }
        /*
         * In the below checks we ensure that :-
         * No zero token supply and positive reserve balance
         * No zero reserve and positive token supply
         * No negative token supply and reserve balance
         * Token supply and reserve balance can be zero (eg. uninitialized pool)
         */
        if token_supply == 0 && reserve_balance != 0 {
            return Err(BondingCurveError::InvalidInput(
                "Cannot have reserve with zero token supply".into(),
            ));
        }

        if reserve_balance == 0 && token_supply != 0 {
            return Err(BondingCurveError::InvalidInput(
                "Cannot have zero reserve with non-zero token supply".into(),
            ));
        }

        if reserve_balance < 0 || token_supply < 0 {
            return Err(BondingCurveError::InvalidInput(
                "Reserve and supply must be non-negative".into(),
            ));
        }
        Ok(Self {
            reserve_balance: I64F64::from_num(reserve_balance),
            token_supply: I64F64::from_num(token_supply),
            connector_weight: I64F64::from_num(connector_weight),
        })
    }
}

impl BondingCurve for Bancor {
    fn get_price(&self) -> Result<I64F64, BondingCurveError> {
        if self.token_supply == I64F64::from_num(0) {
            return Ok(I64F64::from_num(0));
        }
        Ok(self.reserve_balance / (self.token_supply * self.connector_weight))
    }

    fn buy_token(&mut self, reserve_amount: I64F64) -> Result<I64F64, BondingCurveError> {
        if reserve_amount <= I64F64::from_num(0) {
            return Err(BondingCurveError::InvalidInput(
                "Reserve amount must be positive".into(),
            ));
        }

        let price = self.get_price()?;
        if price == I64F64::from_num(0) && self.token_supply != I64F64::from_num(0) {
            return Err(BondingCurveError::CalculationError(
                "Invalid price calculation".into(),
            ));
        }

        let tokens_issued = if self.token_supply == I64F64::from_num(0) {
            reserve_amount / I64F64::from_num(0.0001)
        } else {
            reserve_amount / price
        };

        self.reserve_balance += reserve_amount;
        self.token_supply += tokens_issued;
        Ok(tokens_issued)
    }

    fn sell_token(&mut self, token_amount: I64F64) -> Result<I64F64, BondingCurveError> {
        if token_amount <= I64F64::from_num(0) || token_amount > self.token_supply {
            return Err(BondingCurveError::InvalidInput(
                "Invalid token amount".into(),
            ));
        }
        let price = self.get_price()?;
        let reserve_received = token_amount * price;
        self.token_supply -= token_amount;
        self.reserve_balance -= reserve_received;
        Ok(reserve_received)
    }

    fn get_supply(&self) -> I64F64 {
        self.token_supply
    }

    fn get_reserve(&self) -> Option<I64F64> {
        Some(self.reserve_balance)
    }
}
