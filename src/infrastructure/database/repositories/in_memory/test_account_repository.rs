#[cfg(test)]
mod tests_account_repository {
    use crate::app::entities::account::{Account, AccountType};
    use crate::app::repositories::account_repository::AccountRepository;
    use crate::app::value_objects::currency::Currency;
    use crate::infrastructure::database::repositories::in_memory::account_repository::InMemoryAccountRepository;
    use crate::shared::test_utilities::get_random_account;

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
        let account = get_random_account();

        account_repository.add(account.clone());

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
            None,
            String::from("MTN Momo Account"),
            String::from("Some cool description"),
            String::from("Platform name"),
            AccountType::Savings,
            Some(Currency::RWF),
        );
        let new_id = account_repository.add(account.clone());

        // WHEN the new id is queried
        let created_account = account_repository.get(new_id);

        // THEN the result is the same as the one input
        assert_eq!(created_account, Some(&account));
    }
}
