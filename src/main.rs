mod app;
mod infrastructure;
mod interfaces;
mod shared;

use crate::app::entities::account::{Account, AccountType};
use crate::app::entities::transaction::{
    AccountRef, Transaction, TransactionStatus, TransactionType,
};
use crate::app::repositories::account_repository::AccountRepository;
use crate::app::typing::currency::Currency;
use crate::infrastructure::database::repositories::in_memory::account_repository::InMemoryAccountRepository;

fn main() {
    // Let us create an account
    let account = Account::new(
        None,
        String::from("In Hand"),
        String::from("Savings"),
        String::from("IN Hand Savings"),
        AccountType::Savings,
        Some(Currency::RWF),
    );

    let mut in_memory_account_repository = InMemoryAccountRepository::new();
    let account_id = in_memory_account_repository.add(account.clone());

    let transaction = Transaction::new(
        None,
        AccountRef::Id(account_id),
        TransactionType::Income,
        10f32.try_into().unwrap(),
        1f32.try_into().unwrap(),
        Some(0f32.try_into().unwrap()),
        Some(9f32.try_into().unwrap()),
        Currency::USD,
        TransactionStatus::RolledBack,
        chrono::Utc::now(),
        None,
        None,
        None,
    );

    print!("{:?}", transaction);
}
