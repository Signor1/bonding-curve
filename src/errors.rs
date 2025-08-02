use thiserror::Error;

#[derive(Error, Debug)]
pub enum BondingCurveError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Calculation error: {0}")]
    CalculationError(String),
}
