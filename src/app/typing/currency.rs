use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    RWF,
    USD,
}

#[derive(Debug)]
pub enum CurrencyParseError {
    InvalidCurrencyString,
    InvalidCurrencyValue,
}

impl TryFrom<&str> for Currency {
    type Error = CurrencyParseError;

    fn try_from(currency: &str) -> Result<Self, CurrencyParseError> {
        deserialize_currency(currency)
    }
}

impl Currency {
    pub fn to_string(&self) -> String {
        match self {
            Currency::RWF => "RWF".to_string(),
            Currency::USD => "USD".to_string(),
        }
    }
}

impl TryFrom<String> for Currency {
    type Error = CurrencyParseError;

    fn try_from(currency: String) -> Result<Self, CurrencyParseError> {
        deserialize_currency(&currency)
    }
}

fn deserialize_currency(currency: &str) -> Result<Currency, CurrencyParseError> {
    match currency {
        "RWF" => Ok(Currency::RWF),
        "USD" => Ok(Currency::USD),
        _ => Err(CurrencyParseError::InvalidCurrencyString),
    }
}

pub(crate) const DEFAULT_CURRENCY: Currency = Currency::RWF;
