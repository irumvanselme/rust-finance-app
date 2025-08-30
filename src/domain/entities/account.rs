use crate::domain::value_objects::currency::{Currency, DEFAULT_CURRENCY};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountType {
    Checking,
    Savings,
    Credit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
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
        name: &str,
        description: &str,
        platform: &str,
        account_type: AccountType,
        currency: Option<Currency>,
    ) -> Self {
        // Construct the Account
        Self {
            name: name.to_string(),
            description: description.to_string(),
            platform: platform.to_string(),
            account_type,
            currency: currency.unwrap_or(DEFAULT_CURRENCY),
            balance: 0.0,
        }
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

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    pub fn set_balance(&mut self, balance: f32) {
        self.balance = balance;
    }

    pub fn set_platform(&mut self, platform: &str) {
        self.platform = platform.to_string();
    }

    pub fn set_account_type(&mut self, account_type: &AccountType) {
        self.account_type = account_type.clone();
    }

    pub fn set_currency(&mut self, currency: &Currency) {
        self.currency = currency.clone();
    }

    pub fn deposit(&mut self, amount: f32) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f32) {
        self.balance -= amount;
    }
}
