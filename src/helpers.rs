use crate::errors::BondingCurveError;
use fixed::traits::Fixed;

/// exponential function for fixed-point types
fn exp_fixed<T>(value: T) -> Result<T, BondingCurveError>
where
    T: Fixed,
{
    let value_f64: f64 = value.to_num();
    let result = libm::exp(value_f64);

    if !result.is_finite() {
        return Err(BondingCurveError::CalculationError(
            "Exponential calculation resulted in infinite or NaN value".into(),
        ));
    }

    Ok(T::from_num(result))
}

/// natural logarithm function for fixed-point types
fn ln_fixed<T>(value: T) -> Result<T, BondingCurveError>
where
    T: Fixed,
{
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

    Ok(T::from_num(result))
}

/// power function for fixed-point types
fn pow_fixed<T>(base: T, exponent: T) -> Result<T, BondingCurveError>
where
    T: Fixed,
{
    let base_f64: f64 = base.to_num();
    let exp_f64: f64 = exponent.to_num();

    if base_f64 < 0.0 && exp_f64.fract() != 0.0 {
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

    Ok(T::from_num(result))
}

/// square root function for fixed-point types
fn sqrt_fixed<T>(value: T) -> Result<T, BondingCurveError>
where
    T: Fixed,
{
    let value_f64: f64 = value.to_num();

    if value_f64 < 0.0 {
        return Err(BondingCurveError::CalculationError(
            "Cannot take square root of negative number".into(),
        ));
    }

    let result = libm::sqrt(value_f64);

    if !result.is_finite() {
        return Err(BondingCurveError::CalculationError(
            "Square root calculation resulted in infinite or NaN value".into(),
        ));
    }

    Ok(T::from_num(result))
}
