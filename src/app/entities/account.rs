use crate::app::entities::common::EntityId;
use crate::app::value_objects::currency::{Currency, DEFAULT_CURRENCY};

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum AccountType {
    Checking,
    Savings,
    Credit,
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
    balance: f32,

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
            balance: 0.0,
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

    pub fn balance(&self) -> f32 {
        self.balance
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

    pub fn set_balance(&mut self, balance: f32) {
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

    pub fn deposit(&mut self, amount: f32) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f32) {
        self.balance -= amount;
    }
}
