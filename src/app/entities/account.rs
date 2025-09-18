use crate::app::entities::common::EntityId;
use crate::app::typing::amount::{Amount, MIN_AMOUNT};
use crate::app::typing::currency::{Currency, DEFAULT_CURRENCY};

#[derive(Debug)]
pub enum ConversionError {
    InvalidCurrency,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum AccountType {
    Checking,
    Savings,
    Credit,
}

impl TryFrom<&str> for AccountType {
    type Error = ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        convert_string_to_account_type(value)
    }
}

impl TryFrom<String> for AccountType {
    type Error = ConversionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        convert_string_to_account_type(&value)
    }
}

impl Into<String> for &AccountType {
    fn into(self) -> String {
        let value = match self {
            AccountType::Savings => "savings",
            AccountType::Credit => "credit",
            AccountType::Checking => "checking",
        };

        value.to_string()
    }
}

impl AccountType {
    pub fn to_string(&self) -> String {
        self.into()
    }
}

fn convert_string_to_account_type(value: &str) -> Result<AccountType, ConversionError> {
    match value {
        "checking" => Ok(AccountType::Checking),
        "savings" => Ok(AccountType::Savings),
        "credit" => Ok(AccountType::Credit),
        _ => Err(ConversionError::InvalidCurrency),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Account {
    id: Option<EntityId>,

    /**
     * The account name is unique, assigned by the account owner. (usually a label)
     */
    name: String,

    /**
     * The account description is a free text field.
     */
    description: String,

    /**
     * The account platform, where it belongs.
     * Eg: "Equity Bank", "Bank of Kigali", "MTN Mobile Money", "In-Hand"
     */
    platform: String,

    /**
     * The account balance.
     */
    balance: Amount,

    /**
     * The account type.
     */
    account_type: AccountType,

    /**
     * The account currency.
     */
    currency: Currency,
}

impl Account {
    pub fn new(
        id: Option<EntityId>,
        name: String,
        description: String,
        platform: String,
        account_type: AccountType,
        currency: Option<Currency>,
    ) -> Self {
        // Construct the Account
        Self {
            id,
            name,
            description,
            platform,
            account_type,
            currency: currency.unwrap_or(DEFAULT_CURRENCY),
            balance: MIN_AMOUNT,
        }
    }

    pub fn id(&self) -> Option<&EntityId> {
        self.id.as_ref()
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn balance(&self) -> &Amount {
        &self.balance
    }

    pub fn platform(&self) -> &str {
        &self.platform
    }

    pub fn account_type(&self) -> &AccountType {
        &self.account_type
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn set_id(&mut self, id: Option<EntityId>) {
        self.id = id;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_balance(&mut self, balance: Amount) {
        self.balance = balance;
    }

    pub fn set_platform(&mut self, platform: String) {
        self.platform = platform
    }

    pub fn set_account_type(&mut self, account_type: AccountType) {
        self.account_type = account_type;
    }

    pub fn set_currency(&mut self, currency: Currency) {
        self.currency = currency
    }

    pub fn deposit(&mut self, amount: &Amount) {
        self.balance = self.balance() + amount
    }

    pub fn withdraw(&mut self, amount: &Amount) {
        self.balance = self.balance() - amount
    }
}
