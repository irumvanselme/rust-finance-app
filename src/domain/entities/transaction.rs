use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::domain::entities::account::Account;
use crate::domain::value_objects::currency::Currency;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccountRef {
    Id(String),
    Account(Account),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionType {
    Expense,
    Income,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
    RolledBack,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    /// Reference to the account
    account: AccountRef,

    /// Type of the transaction
    transaction_type: TransactionType,

    /// Amount of the transaction
    amount: f32,

    /// Transaction fee
    fee: f32,

    /// Opening balance.
    opening_balance: f32,

    /// Closing balance.
    closing_balance: f32,

    /// Currency of the transaction.
    currency: Currency,

    /// Transaction description.
    description: Option<String>,

    /// Transaction date.
    transaction_date: DateTime<Utc>,

    /// Reference number
    reference_number: Option<String>,

    transaction_message: Option<String>,

    /// Transaction status
    transaction_status: TransactionStatus,
}

impl Transaction {
    pub fn new(
        account: AccountRef,
        transaction_type: &TransactionType,
        amount: &f32,
        fee: &f32,
        opening_balance: &f32,
        closing_balance: &f32,
        currency: &Currency,
        description: &Option<String>,
        transaction_date: &DateTime<Utc>,
        reference_number: &Option<String>,
        transaction_message: &Option<String>,
        transaction_status: &TransactionStatus,
    ) -> Self {
        Self {
            account,
            transaction_type: transaction_type.clone(),
            amount: amount.clone(),
            fee: fee.clone(),
            opening_balance: opening_balance.clone(),
            closing_balance: closing_balance.clone(),
            currency: currency.clone(),
            description: description.clone(),
            transaction_date: transaction_date.clone(),
            reference_number: reference_number.clone(),
            transaction_message: transaction_message.clone(),
            transaction_status: transaction_status.clone(),
        }
    }

    pub fn account(&self) -> &AccountRef {
        &self.account
    }

    pub fn transaction_type(&self) -> &TransactionType {
        &self.transaction_type
    }

    pub fn amount(&self) -> f32 {
        self.amount
    }

    pub fn fee(&self) -> f32 {
        self.fee
    }

    pub fn opening_balance(&self) -> f32 {
        self.opening_balance
    }

    pub fn closing_balance(&self) -> f32 {
        self.closing_balance
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn transaction_date(&self) -> DateTime<Utc> {
        self.transaction_date
    }

    pub fn reference_number(&self) -> &Option<String> {
        &self.reference_number
    }

    pub fn transaction_message(&self) -> &Option<String> {
        &self.transaction_message
    }

    pub fn transaction_status(&self) -> &TransactionStatus {
        &self.transaction_status
    }

    pub fn set_account(&mut self, account: AccountRef) {
        self.account = account;
    }

    pub fn set_transaction_type(&mut self, transaction_type: &TransactionType) {
        self.transaction_type = transaction_type.clone();
    }

    pub fn set_amount(&mut self, amount: &f32) {
        self.amount = amount.clone()
    }

    pub fn set_fee(&mut self, fee: &f32) {
        self.fee = fee.clone();
    }

    pub fn set_opening_balance(&mut self, opening_balance: &f32) {
        self.opening_balance = opening_balance.clone();
    }

    pub fn set_closing_balance(&mut self, closing_balance: &f32) {
        self.closing_balance = closing_balance.clone();
    }

    pub fn set_currency(&mut self, currency: &Currency) {
        self.currency = currency.clone();
    }

    pub fn set_description(&mut self, description: &Option<String>) {
        self.description = description.clone();
    }

    pub fn set_transaction_date(&mut self, transaction_date: &DateTime<Utc>) {
        self.transaction_date = transaction_date.clone();
    }

    pub fn set_reference_number(&mut self, reference_number: &Option<String>) {
        self.reference_number = reference_number.clone();
    }

    pub fn set_transaction_message(&mut self, transaction_message: &Option<String>) {
        self.transaction_message = transaction_message.clone();
    }

    pub fn set_transaction_status(&mut self, transaction_status: &TransactionStatus) {
        self.transaction_status = transaction_status.clone();
    }
}
