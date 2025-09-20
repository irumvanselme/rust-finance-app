use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::ops::Sub;
use thiserror::Error;

pub(super) const _MAX_AMOUNT: f32 = 1000000.0;
pub(super) const _MIN_AMOUNT: f32 = 0.0;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum AmountError {
    #[error("invalid amount value: {0}, must be greater than the minimum value {min}", min=_MIN_AMOUNT)]
    MinValue(f32),

    #[error("invalid amount value: {0}, must be less than the maximum value {max}", max=_MAX_AMOUNT)]
    MaxValue(f32),
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub(crate) struct Amount(f32);

impl Amount {
    pub(crate) fn new(amount: f32) -> Result<Self, AmountError> {
        if amount < _MIN_AMOUNT {
            return Err(AmountError::MinValue(amount));
        }

        if amount > _MAX_AMOUNT {
            return Err(AmountError::MaxValue(amount));
        }

        Ok(Amount(amount))
    }
}

// Conversion implementations

impl TryFrom<f32> for Amount {
    type Error = AmountError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Amount::new(value.into())
    }
}

// Arithmetic Operations implementations

impl Add for Amount {
    type Output = Amount;

    fn add(self, rhs: Self) -> Self::Output {
        Amount::new(self.0 + rhs.0).unwrap()
    }
}

impl Add for &Amount {
    type Output = Amount;

    fn add(self, rhs: Self) -> Self::Output {
        Amount::new(self.0 + rhs.0).unwrap()
    }
}

impl Sub for Amount {
    type Output = Amount;

    fn sub(self, rhs: Self) -> Self::Output {
        Amount::new(self.0 - rhs.0).unwrap()
    }
}

impl Sub for &Amount {
    type Output = Amount;

    fn sub(self, rhs: Self) -> Self::Output {
        Amount::new(self.0 - rhs.0).unwrap()
    }
}

/// The minimum amount that can be stored in the database or processed
pub(crate) const MIN_AMOUNT: Amount = Amount(_MIN_AMOUNT);

/// Max amount that can be stored in the database or processed
pub(crate) const MAX_AMOUNT: Amount = Amount(_MAX_AMOUNT);
