#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Currency {
    RWF,
    USD,
}

pub(crate) const DEFAULT_CURRENCY: Currency = Currency::RWF;
