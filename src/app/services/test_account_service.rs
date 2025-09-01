#[cfg(test)]
mod test_account_service_get_all {
    use crate::app::repositories::account_repository::AccountRepository;
    use crate::app::services::account_service::AccountService;
    use crate::infrastructure::database::repositories::in_memory::account_repository::InMemoryAccountRepository;
    use crate::shared::test_utilities::{assert_accounts_equal, get_random_account};

    #[test]
    fn test_get_all_accounts_empty_list() {
        // GIVEN an in-memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // WHEN getting all accounts
        let account_service = AccountService::new(&mut account_repository);
        let accounts = account_service.get_all();

        // THEN the account list is empty
        assert_eq!(accounts.len(), 0);
    }

    #[test]
    fn test_get_all_accounts_non_empty_list() {
        // GIVEN an in-memory account repository
        let mut account_repository = InMemoryAccountRepository::new();
        // AND two items are added.
        let given_accounts = (get_random_account(), get_random_account());
        account_repository.add(given_accounts.0.clone());
        account_repository.add(given_accounts.1.clone());

        // WHEN getting all accounts
        let account_service = AccountService::new(&mut account_repository);
        let accounts = account_service.get_all();

        // THEN the accounts should be the same as the given accounts.
        assert_eq!(accounts.len(), 2);
        assert_accounts_equal(&given_accounts.0, accounts.get(0).unwrap(), false);
        assert_accounts_equal(&given_accounts.1, accounts.get(1).unwrap(), false);
    }
}

#[cfg(test)]
mod test_account_service_find_by_id {
    use crate::app::entities::common::EntityId;
    use crate::app::repositories::account_repository::AccountRepository;
    use crate::app::services::account_service::AccountService;
    use crate::infrastructure::database::repositories::in_memory::account_repository::InMemoryAccountRepository;
    use crate::shared::test_utilities::{assert_accounts_equal, get_random_account};

    #[test]
    fn test_find_by_id_returns_a_value() {
        // GIVEN an in-memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // AND some account is in the repository
        let given_account = get_random_account();
        let account_id = account_repository.add(given_account.clone());

        // WHEN finding by id
        let account_service = AccountService::new(&mut account_repository);
        let account = account_service.find_by_id(account_id);

        // THEN the account should be the same as the given account.
        assert_accounts_equal(&given_account, account.unwrap(), false);
    }

    #[test]
    fn test_find_by_id_returns_none() {
        // GIVEN an in-memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // WHEN finding by an id that does not exist
        let given_account_id: EntityId = "1".into();
        let account_service = AccountService::new(&mut account_repository);
        let account = account_service.find_by_id(given_account_id);

        // THEN the account should be none.
        assert!(account.is_none());
    }
}

#[cfg(test)]
mod test_account_service_save {
    use crate::app::repositories::account_repository::AccountRepository;
    use crate::app::services::account_service::AccountService;
    use crate::infrastructure::database::repositories::in_memory::account_repository::InMemoryAccountRepository;
    use crate::shared::test_utilities::{assert_accounts_equal, get_random_account};

    #[test]
    fn test_save_success() {
        // GIVEN an in-memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // WHEN saving a new account
        let given_account = get_random_account();
        let mut account_service = AccountService::new(&mut account_repository);
        let account_id = account_service.save(given_account.clone());

        // THEN the request should be in the repository.
        assert_accounts_equal(
            &given_account,
            account_repository.get(account_id).unwrap(),
            false,
        );
    }
}

#[cfg(test)]
mod test_account_service_find_by_id_or_fail {
    use crate::app::entities::common::EntityId;
    use crate::app::repositories::account_repository::AccountRepository;
    use crate::app::services::account_service::AccountService;
    use crate::infrastructure::database::repositories::in_memory::account_repository::InMemoryAccountRepository;
    use crate::shared::test_utilities::{assert_accounts_equal, get_random_account};

    #[test]
    fn test_find_by_id_returns_a_value() {
        // GIVEN an in-memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // AND some account is in the repository
        let given_account = get_random_account();
        let account_id = account_repository.add(given_account.clone());

        // WHEN finding by an id or fail
        let account_service = AccountService::new(&mut account_repository);
        let account = account_service.find_by_id_or_fail(account_id);

        // THEN the account should be the same as the given account.
        assert_accounts_equal(&given_account, account.unwrap(), false);
    }

    #[test]
    fn test_find_by_id_returns_an_error() {
        // GIVEN an in-memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // WHEN finding by an id that does not exist
        let given_account_id: EntityId = "1".into();
        let account_service = AccountService::new(&mut account_repository);
        let account = account_service.find_by_id_or_fail(given_account_id);

        // THEN the account should return a NotFound Error.
        assert!(account.is_err());
        assert!(matches!(
            account.err().unwrap(),
            crate::app::services::account_service::AccountServiceError::NotFound(_)
        ));
    }
}
