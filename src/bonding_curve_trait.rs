use crate::errors::BondingCurveError;
use fixed::types::I64F64;

// interface for all bonding curves
pub trait BondingCurve {
    // get the current price based on the curve's state
    fn get_price(&self) -> Result<I64F64, BondingCurveError>;

    // Calculates tokens received for a given reserve amount
    fn buy_token(&mut self, reserve_amount: I64F64) -> Result<I64F64, BondingCurveError>;

    // Calculates reserve received for selling a given token amount
    fn sell_token(&mut self, token_amount: I64F64) -> Result<I64F64, BondingCurveError>;

    // Return the total supply of tokens
    fn get_supply(&self) -> I64F64;

    // Return the current reserve of tokens
    fn get_reserve(&self) -> Option<I64F64>;
}
