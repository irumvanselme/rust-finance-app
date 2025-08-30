use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    RWF,
    USD,
}

pub(crate) const DEFAULT_CURRENCY: Currency = Currency::RWF;
