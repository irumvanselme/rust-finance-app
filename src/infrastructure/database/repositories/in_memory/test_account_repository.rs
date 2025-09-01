#[cfg(test)]
mod tests_account_repository {
    use crate::app::entities::account::{Account, AccountType};
    use crate::app::entities::common::EntityId;
    use crate::app::repositories::account_repository::{AccountRepository, FindByIdAndUpdateError};
    use crate::app::typing::currency::Currency;
    use crate::infrastructure::database::repositories::in_memory::account_repository::InMemoryAccountRepository;
    use crate::shared::test_utilities::{
        assert_accounts_equal, get_random_account, get_random_string,
    };

    #[test]
    fn test_empty_get_all() {
        // GIVEN the in memory account repository is initialized.
        // AND no items are added in the repository.
        let account_repository = InMemoryAccountRepository::new();

        // WHEN the get_all method is called
        let accounts = account_repository.find_all();

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
        let accounts = account_repository.find_all();

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
        let created_account = account_repository.find_by_id(new_id);

        // THEN the result is the same as the one input
        assert_eq!(created_account, Some(&account));
    }

    #[test]
    fn test_find_by_id_and_update_success() {
        // GIVEN an in memory account repository is initialized.
        // AND an account is added in the repository.
        let mut account_repository = InMemoryAccountRepository::new();
        let mut account = get_random_account();
        let new_id = account_repository.add(account.clone());

        // WHEN the account name is queried.
        let db_account = account_repository.find_by_id(new_id.clone()).unwrap();

        // GUARD any fields should be the same.
        assert_accounts_equal(&account, db_account, false);

        let new_name = get_random_string(20);
        account.set_name(new_name.clone());

        // WHEN the account is saved again
        let update_result =
            account_repository.find_by_id_and_update(new_id.clone(), account.clone());

        // THEN it should be updated successfully.
        assert!(update_result.is_ok());

        // AND the v2 of the db account is queried.
        let db_account = account_repository.find_by_id(new_id.clone()).unwrap();

        // THEN it should be the same as the one input
        assert_accounts_equal(&account, db_account, false);

        // AND the name should match the new name
        assert_eq!(&new_name, db_account.name())
    }

    #[test]
    fn test_find_by_id_and_update_with_invalid_id() {
        // GIVEN an in memory account repository is initialized.
        //       without any account added.
        let mut account_repository = InMemoryAccountRepository::new();

        // WHEN the account_repository.find_by_id_and_update is called with an invalid id
        let entity_id: EntityId = "100".into();

        // THEN it should return an error
        let result = account_repository.find_by_id_and_update(entity_id, get_random_account());

        // AND the result should be an error
        assert!(result.is_err());

        // AND the error should be an invalid id error
        assert_eq!(result.err().unwrap(), FindByIdAndUpdateError::NotFound)
    }
}
