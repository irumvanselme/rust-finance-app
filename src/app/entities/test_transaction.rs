#[cfg(test)]
mod test_transaction {
    use crate::app::entities::transaction::{
        AccountRef, Transaction, TransactionStatus, TransactionType,
    };
    use crate::app::typing::amount::Amount;
    use crate::app::typing::currency::Currency;
    use chrono::{DateTime, Utc};

    #[test]
    fn test_entity() {
        // GIVEN some transaction details,
        let mut given_account_ref = AccountRef::Id(String::from("1234567890").into());
        let mut given_transaction_type = TransactionType::Expense;
        let mut given_amount: Amount = 100.0f32.try_into().unwrap();
        let mut given_fee: Amount = 10.0f32.try_into().unwrap();
        let mut given_opening_balance: Option<Amount> = Some(1000.0f32.try_into().unwrap());
        let mut given_closing_balance: Option<Amount> = Some(344.0f32.try_into().unwrap());
        let mut given_currency = Currency::RWF;
        let mut given_description = String::from("test");
        let mut given_reference_number = String::from("1234567890");
        let mut given_transaction_message = String::from("test");
        let mut given_date = DateTime::parse_from_rfc3339("2023-10-01T12:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let mut given_status = TransactionStatus::Confirmed;

        // WHEN a transaction object is created,
        let mut transaction = Transaction::new(
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
        );

        // THEN the transaction object should be created successfully.
        // AND the fields should be set accordingly.
        assert_eq!(*transaction.account(), given_account_ref);
        assert_eq!(*transaction.transaction_type(), given_transaction_type);
        assert_eq!(*transaction.amount(), given_amount);
        assert_eq!(*transaction.fee(), given_fee);
        assert_eq!(*transaction.opening_balance(), given_opening_balance);
        assert_eq!(*transaction.closing_balance(), given_closing_balance);
        assert_eq!(*transaction.currency(), given_currency);
        assert_eq!(*transaction.status(), given_status);
        assert_eq!(transaction.date(), given_date);
        assert_eq!(*transaction.description(), Some(given_description));
        assert_eq!(
            *transaction.reference_number(),
            Some(given_reference_number)
        );
        assert_eq!(*transaction.message(), Some(given_transaction_message));
        assert_eq!(*transaction.status(), given_status);

        // WHEN the transaction details are updated in memory
        given_account_ref = AccountRef::Id(String::from("updated-1234567890").into());
        given_transaction_type = TransactionType::Income;
        given_amount = 101.0f32.try_into().unwrap();
        given_fee = 11.0f32.try_into().unwrap();
        given_opening_balance = Some(1001.0f32.try_into().unwrap());
        given_closing_balance = Some(3450f32.try_into().unwrap());
        given_currency = Currency::USD;
        given_description = String::from("test-1");
        given_reference_number = String::from("updated-1234567890");
        given_transaction_message = String::from("updated-test");
        given_date = DateTime::parse_from_rfc3339("2025-10-01T12:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        given_status = TransactionStatus::RolledBack;

        // THEN The transaction object should not be udpated.
        assert_ne!(*transaction.account(), given_account_ref);
        assert_ne!(*transaction.transaction_type(), given_transaction_type);
        assert_ne!(*transaction.amount(), given_amount);
        assert_ne!(*transaction.fee(), given_fee);
        assert_ne!(*transaction.opening_balance(), given_opening_balance);
        assert_ne!(*transaction.closing_balance(), given_closing_balance);
        assert_ne!(*transaction.currency(), given_currency);
        assert_ne!(*transaction.status(), given_status);
        assert_ne!(transaction.date(), given_date);
        assert_ne!(*transaction.description(), Some(given_description.clone()));
        assert_ne!(
            *transaction.reference_number(),
            Some(given_reference_number.clone())
        );
        assert_ne!(
            *transaction.message(),
            Some(given_transaction_message.clone())
        );

        // WHEN the actual entity is updated.
        transaction.set_account(given_account_ref.clone());
        transaction.set_transaction_type(given_transaction_type.clone());
        transaction.set_amount(given_amount.clone());
        transaction.set_fee(given_fee.clone());
        transaction.set_opening_balance(given_opening_balance.clone());
        transaction.set_closing_balance(given_closing_balance.clone());
        transaction.set_currency(given_currency.clone());
        transaction.set_status(given_status.clone());
        transaction.set_date(given_date.clone());
        transaction.set_description(Some(given_description.clone()));
        transaction.set_reference_number(Some(given_reference_number.clone()));
        transaction.set_message(Some(given_transaction_message.clone()));

        // THEN the transaction object should be updated successfully.
        assert_eq!(*transaction.account(), given_account_ref);
        assert_eq!(*transaction.transaction_type(), given_transaction_type);
        assert_eq!(*transaction.amount(), given_amount);
        assert_eq!(*transaction.fee(), given_fee);
        assert_eq!(*transaction.opening_balance(), given_opening_balance);
        assert_eq!(*transaction.closing_balance(), given_closing_balance);
        assert_eq!(*transaction.currency(), given_currency);
        assert_eq!(*transaction.status(), given_status);
        assert_eq!(transaction.date(), given_date);
        assert_eq!(*transaction.description(), Some(given_description));
        assert_eq!(
            *transaction.reference_number(),
            Some(given_reference_number)
        );
        assert_eq!(*transaction.message(), Some(given_transaction_message));
    }
}
