use crate::app::entities::account::{Account, AccountType};
use crate::app::entities::common::EntityRef;
use crate::app::entities::transaction::{Transaction, TransactionStatus, TransactionType};
use crate::app::value_objects::currency::Currency;
use chrono::{DateTime, Utc};
use rand::{distr::Alphanumeric, Rng};

#[allow(dead_code)]
fn get_random_string(len: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

#[allow(dead_code)]
fn get_random_float() -> f32 {
    rand::rng().random_range(0.0..1000.0)
}

#[allow(dead_code)]
pub fn get_random_transaction() -> Transaction {
    let mut given_account_ref = EntityRef::Id(get_random_string(10).into());
    let mut given_transaction_type = TransactionType::Expense;
    let mut given_amount = get_random_float();
    let mut given_fee = get_random_float();
    let mut given_opening_balance = get_random_float();
    let mut given_closing_balance = get_random_float();
    let mut given_currency = Currency::RWF;
    let mut given_description = get_random_string(1000);
    let mut given_reference_number = get_random_string(20);
    let mut given_transaction_message = get_random_string(200);
    let mut given_date = DateTime::parse_from_rfc3339("2023-10-01T12:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let mut given_status = TransactionStatus::Confirmed;

    Transaction::new(
        None,
        given_account_ref.clone(),
        given_transaction_type.clone(),
        given_amount.clone(),
        given_fee.clone(),
        given_opening_balance.clone(),
        given_closing_balance.clone(),
        given_currency.clone(),
        given_status.clone(),
        given_date.clone(),
        Some(given_description.clone()),
        Some(given_reference_number.clone()),
        Some(given_transaction_message.clone()),
    )
}

#[allow(dead_code)]
pub fn get_random_account() -> Account {
    Account::new(
        None,
        get_random_string(10),
        get_random_string(200),
        get_random_string(30),
        AccountType::Savings,
        Some(Currency::RWF),
    )
}
