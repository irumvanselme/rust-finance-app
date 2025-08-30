#[cfg(test)]
mod test_account_entity {
    use crate::app::entities::account::{Account, AccountType};
    use crate::app::value_objects::currency::Currency;

    #[test]
    fn test_new_account() {
        // GIVEN some account name
        let given_account_name = String::from("Account name");

        // AND the account description
        let given_account_description = String::from("Account Description");

        // AND the account type
        let given_account_type = AccountType::Savings;

        // AND the platform name
        let given_platform_name = String::from("Platform name");

        // AND given some currency
        let given_currency = Currency::RWF;

        // WHEN the account object is created
        let account = Account::new(
            None,
            given_account_name.clone(),
            given_account_description.clone(),
            given_platform_name.clone(),
            given_account_type,
            Some(given_currency),
        );

        // THEN the name is the same as the input
        assert_eq!(*account.name(), given_account_name);

        // AND the description should be the same as given
        assert_eq!(*account.description(), given_account_description);

        // AND the balance should start from zero.
        assert_eq!(account.balance(), 0.0);
    }

    #[test]
    fn test_setters_and_getters() {
        // GIVEN a sample Account
        let mut account = Account::new(
            None,
            String::from("Account name"),
            String::from("Account Description"),
            String::from("Platform name"),
            AccountType::Savings,
            Some(Currency::RWF),
        );

        // WHEN the balance is updated
        let new_balance = 100.0;
        account.set_balance(new_balance);

        // THEN the balance should be the same as the input
        assert_eq!(account.balance(), new_balance);

        // WHEN Account name is updated
        let new_name = String::from("New account name");
        account.set_name(new_name.clone());

        // THEN the name should be the same as the input
        assert_eq!(*account.name(), new_name);

        // WHEN account description is updated
        let new_description = String::from("New account description");
        account.set_description(new_description.clone());

        // THEN the description should be the same as the input
        assert_eq!(*account.description(), new_description);

        // WHEN platform name is updated
        let new_platform_name = String::from("New platform name");
        account.set_platform(new_platform_name.clone());

        // THEN the platform name should be the same as the input
        assert_eq!(account.platform(), new_platform_name);

        // WHEN the account type is updated
        let new_account_type = AccountType::Credit;
        account.set_account_type(new_account_type.clone());

        // THEN the account should be updated
        assert_eq!(*account.account_type(), new_account_type);

        // WHEN the currency is updated
        let new_currency = Currency::USD;
        account.set_currency(new_currency.clone());

        // THEN the currency should be updated
        assert_eq!(*account.currency(), new_currency);
    }

    #[test]
    fn test_balance_manipulation() {
        // GIVEN a random account
        let mut account = Account::new(
            None,
            String::from("Account name"),
            String::from("Account Description"),
            String::from("Platform name"),
            AccountType::Savings,
            Some(Currency::RWF),
        );

        // WHEN the balance is updated
        let new_balance = 100.0;
        account.set_balance(new_balance);

        // WHEN the balance is updated
        let new_balance = 100.0;
        account.set_balance(new_balance);

        // AND it is reflected as the new balance
        assert_eq!(account.balance(), new_balance);

        // WHEN some amount is withdrawn
        let withdrawn_amount = 10.0;
        account.withdraw(withdrawn_amount);

        // THEN the balance should be updated
        assert_eq!(account.balance(), new_balance - withdrawn_amount);

        // WHEN some amount is deposited
        let deposited_amount = 10.0;
        account.deposit(deposited_amount);

        // THEN the balance should be updated
        assert_eq!(
            account.balance(),
            new_balance - withdrawn_amount + deposited_amount
        );
    }
}
