use crate::app::entities::account::Account;
use crate::app::entities::common::{EntityId, EntityRef};
use crate::app::value_objects::currency::Currency;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Expense,
    Income,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
    RolledBack,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    // Unique identifier for the transaction.
    id: Option<EntityId>,

    /// Reference to the account
    account: EntityRef<Account>,

    /// Type of the transaction (using transaction_ because type is a reserved word)
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
    date: DateTime<Utc>,

    /// Reference number
    reference_number: Option<String>,

    message: Option<String>,

    /// Transaction status
    status: TransactionStatus,
}

impl Transaction {
    pub fn new(
        id: Option<EntityId>,
        account: EntityRef<Account>,
        transaction_type: TransactionType,
        amount: f32,
        fee: f32,
        opening_balance: f32,
        closing_balance: f32,
        currency: Currency,
        status: TransactionStatus,
        date: DateTime<Utc>,
        description: Option<String>,
        reference_number: Option<String>,
        message: Option<String>,
    ) -> Self {
        Self {
            id,
            account,
            transaction_type,
            amount,
            fee,
            opening_balance,
            closing_balance,
            currency,
            description,
            date,
            reference_number,
            message,
            status,
        }
    }

    pub fn id(&self) -> &Option<EntityId> {
        &self.id
    }

    pub fn account(&self) -> &EntityRef<Account> {
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

    pub fn date(&self) -> DateTime<Utc> {
        self.date
    }

    pub fn reference_number(&self) -> &Option<String> {
        &self.reference_number
    }

    pub fn message(&self) -> &Option<String> {
        &self.message
    }

    pub fn status(&self) -> &TransactionStatus {
        &self.status
    }

    pub fn set_id(&mut self, id: Option<EntityId>) {
        self.id = id;
    }

    pub fn set_account(&mut self, account: EntityRef<Account>) {
        self.account = account;
    }

    pub fn set_transaction_type(&mut self, transaction_type: TransactionType) {
        self.transaction_type = transaction_type;
    }

    pub fn set_amount(&mut self, amount: f32) {
        self.amount = amount
    }

    pub fn set_fee(&mut self, fee: f32) {
        self.fee = fee;
    }

    pub fn set_opening_balance(&mut self, opening_balance: f32) {
        self.opening_balance = opening_balance;
    }

    pub fn set_closing_balance(&mut self, closing_balance: f32) {
        self.closing_balance = closing_balance;
    }

    pub fn set_currency(&mut self, currency: Currency) {
        self.currency = currency;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn set_date(&mut self, date: DateTime<Utc>) {
        self.date = date;
    }

    pub fn set_reference_number(&mut self, reference_number: Option<String>) {
        self.reference_number = reference_number;
    }

    pub fn set_message(&mut self, message: Option<String>) {
        self.message = message;
    }

    pub fn set_status(&mut self, status: TransactionStatus) {
        self.status = status;
    }
}
