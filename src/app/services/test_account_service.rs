#[cfg(test)]
mod test_account_service_find_all {
    use crate::app::repositories::account_repository::AccountRepository;
    use crate::app::services::account_service::AccountService;
    use crate::infrastructure::repositories::in_memory::account_repository::InMemoryAccountRepository;
    use crate::shared::test_utilities::{assert_accounts_equal, get_random_account};

    #[test]
    fn test_find_all_accounts_empty_list() {
        // GIVEN an in-memory account repository
        let account_repository = InMemoryAccountRepository::new();

        // WHEN getting all accounts
        let account_service = AccountService;
        let accounts = account_service.find_all(&account_repository);

        // THEN the account list is empty
        assert_eq!(accounts.len(), 0);
    }

    #[test]
    fn test_find_all_accounts_non_empty_list() {
        // GIVEN an in-memory account repository
        let mut account_repository = InMemoryAccountRepository::new();
        // AND two items are added.
        let given_accounts = (get_random_account(), get_random_account());
        account_repository.create(given_accounts.0.clone());
        account_repository.create(given_accounts.1.clone());

        // WHEN getting all accounts
        let account_service = AccountService;
        let accounts = account_service.find_all(&account_repository);

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
    use crate::infrastructure::repositories::in_memory::account_repository::InMemoryAccountRepository;
    use crate::shared::test_utilities::{assert_accounts_equal, get_random_account};

    #[test]
    fn test_find_by_id_returns_a_value() {
        // GIVEN an in-memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // AND some account is in the repository
        let given_account = get_random_account();
        let account_id = account_repository.create(given_account.clone());

        // WHEN finding by id
        let account_service = AccountService;
        let account = account_service.find_by_id(&account_repository, account_id);

        // THEN the account should be the same as the given account.
        assert_accounts_equal(&given_account, account.unwrap(), false);
    }

    #[test]
    fn test_find_by_id_returns_none() {
        // GIVEN an in-memory account repository
        let account_repository = InMemoryAccountRepository::new();

        // WHEN finding by an id that does not exist
        let given_account_id: EntityId = "1".into();
        let account_service = AccountService;
        let account = account_service.find_by_id(&account_repository, given_account_id);

        // THEN the account should be none.
        assert!(account.is_none());
    }
}

#[cfg(test)]
mod test_account_service_save {
    use crate::app::repositories::account_repository::AccountRepository;
    use crate::app::services::account_service::{AccountService, CreateError};
    use crate::infrastructure::repositories::in_memory::account_repository::InMemoryAccountRepository;
    use crate::shared::test_utilities::{assert_accounts_equal, get_random_account};

    #[test]
    fn test_save_success() {
        // GIVEN an in-memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // WHEN saving a new account
        let given_account = get_random_account();
        let account_service = AccountService;
        let account_id = account_service.create(&mut account_repository, given_account.clone());

        // THEN the request should be in the repository.
        assert_accounts_equal(
            &given_account,
            account_repository.find_by_id(account_id.unwrap()).unwrap(),
            false,
        );
    }

    #[test]
    fn test_save_id_provided() {
        // GIVEN an in-memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // WHEN saving a new account with an id provided
        let mut given_account = get_random_account();
        given_account.set_id(Some("1".into()));
        let account_service = AccountService;
        let create_response =
            account_service.create(&mut account_repository, given_account.clone());

        // THEN the request should be in the repository.
        assert!(create_response.is_err());

        // AND the error should be DuplicateEntityId
        assert_eq!(
            create_response.err().unwrap(),
            CreateError::EntityIdProvided
        )
    }
}

#[cfg(test)]
mod test_account_service_find_by_id_or_fail {
    use crate::app::entities::common::EntityId;
    use crate::app::repositories::account_repository::AccountRepository;
    use crate::app::services::account_service::{AccountService, FindByIdOrFailError};
    use crate::infrastructure::repositories::in_memory::account_repository::InMemoryAccountRepository;
    use crate::shared::test_utilities::{assert_accounts_equal, get_random_account};

    #[test]
    fn test_find_by_id_returns_a_value() {
        // GIVEN an in-memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // AND some account is in the repository
        let given_account = get_random_account();
        let account_id = account_repository.create(given_account.clone());

        // WHEN finding by an id or fail
        let account_service = AccountService;
        let account = account_service.find_by_id_or_fail(&account_repository, &account_id);

        // THEN the account should be the same as the given account.
        assert_accounts_equal(&given_account, account.unwrap(), false);
    }

    #[test]
    fn test_find_by_id_returns_an_error() {
        // GIVEN an in-memory account repository
        let account_repository = InMemoryAccountRepository::new();

        // WHEN finding by an id that does not exist
        let given_account_id: EntityId = "1".into();
        let account_service = AccountService;
        let account = account_service.find_by_id_or_fail(&account_repository, &given_account_id);

        // THEN the account should return a NotFound Error.
        assert!(account.is_err());
        assert!(matches!(
            account.err().unwrap(),
            FindByIdOrFailError::NotFound(_)
        ));
    }
}

#[cfg(test)]
mod test_account_service_withdraw {
    use crate::app::entities::common::EntityId;
    use crate::app::repositories::account_repository::AccountRepository;
    use crate::app::services::account_service::{AccountService, UpdateError};
    use crate::app::typing::amount::Amount;
    use crate::infrastructure::repositories::in_memory::account_repository::InMemoryAccountRepository;
    use crate::shared::test_utilities::get_random_account;

    #[test]
    fn test_withdraw_success() {
        // GIVEN an in-memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // AND a random account.
        let mut account = get_random_account();

        // AND given some balance.
        let given_balance: Amount = 100f32.try_into().unwrap();

        // AND the account is saved in the repository with the given balance.
        account.set_balance(given_balance.clone());
        let account_id = account_repository.create(account.clone());

        // AND the amount to withdraw.
        let amount_to_withdraw: Amount = 50f32.try_into().unwrap();

        // WHEN the amount is withdrawn using the service
        let mut account_service = AccountService;
        let withdraw_response =
            account_service.withdraw(&mut account_repository, &account_id, &amount_to_withdraw);

        // THEN the withdrawal request should be successful.
        assert!(withdraw_response.is_ok());

        // AND the new account should be (given_balance — amount_to_withdraw)
        assert_eq!(
            *withdraw_response.unwrap().balance(),
            given_balance.clone() - amount_to_withdraw.clone()
        );

        // AND it should be reflected in the item in the repository
        let account = account_repository.find_by_id(account_id).unwrap();
        assert_eq!(*account.balance(), given_balance - amount_to_withdraw);
    }

    #[test]
    fn test_withdraw_entity_id_not_found() {
        // GIVEN an in-memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // WHEN the amount is withdrawn using the service
        let mut account_service = AccountService;
        let given_amount_to_withdraw: Amount = 50f32.try_into().unwrap();
        let given_account_id: EntityId = "1".into();
        let withdraw_response = account_service.withdraw(
            &mut account_repository,
            &given_account_id,
            &given_amount_to_withdraw,
        );

        // THEN the withdrawal request should be successful.
        assert!(withdraw_response.is_err());

        // AND the error should be EntityIdNotFound
        assert_eq!(
            withdraw_response.err().unwrap(),
            UpdateError::EntityIdNotFound
        )
    }

    #[test]
    fn test_withdraw_insufficient_funds() {
        // GIVEN an in-memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // AND a random account.
        let mut account = get_random_account();

        // AND given some balance is zero
        let given_balance: Amount = 0f32.try_into().unwrap();

        // AND the account is saved in the repository with the given balance.
        account.set_balance(given_balance.clone());
        let account_id = account_repository.create(account.clone());

        // AND the amount to withdraw.
        let amount_to_withdraw: Amount = 50f32.try_into().unwrap();

        // WHEN the amount is withdrawn using the service
        let mut account_service = AccountService;
        let withdraw_response =
            account_service.withdraw(&mut account_repository, &account_id, &amount_to_withdraw);

        // THEN the withdrawal request should fail
        assert!(withdraw_response.is_err());

        // AND the error should be InsufficientFunds
        assert_eq!(
            withdraw_response.err().unwrap(),
            UpdateError::InsufficientFunds
        )
    }
}

#[cfg(test)]
mod test_account_service_deposit {
    use crate::app::entities::common::EntityId;
    use crate::app::repositories::account_repository::AccountRepository;
    use crate::app::services::account_service::{AccountService, UpdateError};
    use crate::app::typing::amount::Amount;
    use crate::infrastructure::repositories::in_memory::account_repository::InMemoryAccountRepository;
    use crate::shared::test_utilities::get_random_account;

    #[test]
    fn test_deposit_success() {
        // GIVEN an in-memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // AND a random account.
        let mut account = get_random_account();

        // AND given some balance.
        let given_balance: Amount = 100f32.try_into().unwrap();

        // AND the account is saved in the repository with the given balance.
        account.set_balance(given_balance.clone());
        let account_id = account_repository.create(account.clone());

        // AND the amount to deposit.
        let amount_to_deposit: Amount = 50f32.try_into().unwrap();

        // WHEN the amount is deposited using the service
        let mut account_service = AccountService;
        let deposit_response =
            account_service.deposit(&mut account_repository, &account_id, &amount_to_deposit);

        // THEN the deposit request should be successful.
        assert!(deposit_response.is_ok());

        // AND the new account should be (given_balance + amount_to_deposit)
        let actual_account = deposit_response.unwrap();
        let actual_new_balance = actual_account.balance();
        let expected_new_balance = given_balance.clone() + amount_to_deposit.clone();
        assert_eq!(actual_new_balance, &expected_new_balance);

        // AND it should be reflected in the item in the repository
        let account = account_repository.find_by_id(account_id).unwrap();
        assert_eq!(*account.balance(), expected_new_balance);
    }

    #[test]
    fn test_deposit_entity_id_not_found() {
        // GIVEN an in-memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // AND a random entity id
        let given_account_id: EntityId = "1".into();

        // AND the amount to deposit.
        let amount_to_deposit: Amount = 50f32.try_into().unwrap();

        // WHEN the amount is deposited using the service
        let mut account_service = AccountService;
        let deposit_response = account_service.deposit(
            &mut account_repository,
            &given_account_id,
            &amount_to_deposit,
        );

        // THEN the deposit request should fail.
        assert!(deposit_response.is_err());

        // AND the error should be EntityIdNotFound
        assert_eq!(
            deposit_response.err().unwrap(),
            UpdateError::EntityIdNotFound
        )
    }
}
