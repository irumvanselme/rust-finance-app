#[cfg(test)]
mod tests_account_repository {
    use crate::domain::entities::account::{Account, AccountType};
    use crate::domain::repositories::account_repository::AccountRepository;
    use crate::domain::value_objects::currency::Currency;
    use crate::infrastructure::database::repositories::in_memory::account_repository::InMemoryAccountRepository;

    #[test]
    fn test_empty_get_all() {
        // GIVEN the in memory account repository is initialized.
        // AND no items are added in the repository.
        let account_repository = InMemoryAccountRepository::new();

        // WHEN the get_all method is called
        let accounts = account_repository.get_all();

        // THEN the result is empty
        assert_eq!(accounts.len(), 0);
    }

    #[test]
    fn test_items_added() {
        // GIVEN the in memory account repository is initialized.
        let mut account_repository = InMemoryAccountRepository::new();

        // WHEN an account is added in the repository
        let account = Account::new(
            "MTN Momo Account",
            "Some cool description",
            "Platform name",
            AccountType::Savings,
            Some(Currency::RWF),
        );

        account_repository.add(&account);

        // AND the get_all method is called
        let accounts = account_repository.get_all();

        // THEN the result is not empty
        assert_eq!(accounts.len(), 1);

        // AND the first item should be the same as the one input
        let first_account = accounts.get(0).unwrap();
        assert_eq!(*first_account, account);
    }

    #[test]
    fn test_find_by_id() {
        // GIVEN the in memory account repository is initialized.
        let mut account_repository = InMemoryAccountRepository::new();

        // AND an account is added in the repository
        let account = Account::new(
            "MTN Momo Account",
            "Some cool description",
            "Platform name",
            AccountType::Savings,
            Some(Currency::RWF),
        );
        let new_id = account_repository.add(&account);

        // WHEN the new id is queried
        let created_account = account_repository.get(new_id);

        // THEN the result is the same as the one input
        assert_eq!(created_account, Some(account));
    }
}
