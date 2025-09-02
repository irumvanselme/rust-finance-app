#[cfg(test)]
mod test_transaction_service_find_all {
    use crate::app::entities::common::EntityId;
    use crate::app::repositories::transaction_repository::TransactionRepository;
    use crate::app::services::account_service::AccountService;
    use crate::app::services::transaction_service::TransactionService;
    use crate::infrastructure::repositories::in_memory::account_repository::InMemoryAccountRepository;
    use crate::infrastructure::repositories::in_memory::transaction_repository::InMemoryTransactionRepository;
    use crate::shared::test_utilities::get_random_transaction;

    #[test]
    fn test_find_all_success() {
        // GIVEN an in memory transaction repository
        let mut transaction_repository = InMemoryTransactionRepository::new();

        // AND an in memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // AND an in memory account service
        let mut account_service = AccountService::new(&mut account_repository);

        // AND a transaction service
        let transaction_service =
            TransactionService::new(&mut transaction_repository, &mut account_service);

        // WHEN we find all transactions
        let transactions = transaction_service.find_all();

        // THEN the account list is empty
        assert_eq!(transactions.len(), 0);
    }

    #[test]
    fn test_find_all_with_items() {
        // GIVEN an in memory transaction repository
        let mut transaction_repository = InMemoryTransactionRepository::new();

        // AND with some transactions
        let transactions = [
            get_random_transaction(),
            get_random_transaction(),
            get_random_transaction(),
        ];

        let mut transactions_ids: [EntityId; 3] = Default::default();
        let mut counter = 0;
        for transaction in transactions {
            let id = transaction_repository.create(transaction);
            transactions_ids[counter] = id;
            counter += 1;
        }

        // AND an in memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // AND an in memory account service
        let mut account_service = AccountService::new(&mut account_repository);

        // AND a transaction service
        let transaction_service =
            TransactionService::new(&mut transaction_repository, &mut account_service);

        // WHEN we find all transactions
        let transactions = transaction_service.find_all();

        // THEN the account list is empty
        assert_eq!(transactions.len(), transactions.len());

        for entity_id in transactions_ids {
            assert!(transaction_service.find_by_id_or_fail(entity_id).is_ok())
        }
    }
}

#[cfg(test)]
mod test_transaction_service_create {}

#[cfg(test)]
mod test_transaction_service_find_by_id {
    use crate::app::repositories::transaction_repository::TransactionRepository;
    use crate::app::services::account_service::AccountService;
    use crate::app::services::transaction_service::TransactionService;
    use crate::infrastructure::repositories::in_memory::account_repository::InMemoryAccountRepository;
    use crate::infrastructure::repositories::in_memory::transaction_repository::InMemoryTransactionRepository;
    use crate::shared::test_utilities::{assert_transactions_equal, get_random_transaction};

    #[test]
    fn find_by_id_success() {
        // GIVEN an in memory transaction repository
        let mut transaction_repository = InMemoryTransactionRepository::new();

        // AND a transaction in the repository
        let transaction = get_random_transaction();
        let id = transaction_repository.create(transaction.clone());

        // AND an in memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // AND an in memory account service
        let mut account_service = AccountService::new(&mut account_repository);

        // AND a transaction service
        let transaction_service =
            TransactionService::new(&mut transaction_repository, &mut account_service);

        // WHEN querying by id
        let actual_transaction = transaction_service.find_by_id(id);

        // THEN it should exist
        assert!(actual_transaction.is_some());

        // AND they should equal.
        assert_transactions_equal(&transaction, &actual_transaction.unwrap(), false);
    }

    #[test]
    fn find_by_id_none() {
        // GIVEN an in memory transaction repository
        let mut transaction_repository = InMemoryTransactionRepository::new();

        // AND an in memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // AND an in memory account service
        let mut account_service = AccountService::new(&mut account_repository);

        // AND a transaction service
        let transaction_service =
            TransactionService::new(&mut transaction_repository, &mut account_service);

        // WHEN we find all transactions by a random entity id
        let random_entity_id = "1".into();
        let transaction = transaction_service.find_by_id(random_entity_id);

        // THEN the response should be none
        assert!(transaction.is_none());
    }
}

#[cfg(test)]
mod test_transaction_service_find_by_id_or_fail {
    use crate::app::repositories::transaction_repository::TransactionRepository;
    use crate::app::services::account_service::AccountService;
    use crate::app::services::transaction_service::{GetOneError, TransactionService};
    use crate::infrastructure::repositories::in_memory::account_repository::InMemoryAccountRepository;
    use crate::infrastructure::repositories::in_memory::transaction_repository::InMemoryTransactionRepository;
    use crate::shared::test_utilities::{assert_transactions_equal, get_random_transaction};

    #[test]
    fn find_by_id_or_fail_success() {
        // GIVEN an in memory transaction repository
        let mut transaction_repository = InMemoryTransactionRepository::new();

        // AND a transaction in the repository
        let transaction = get_random_transaction();
        let id = transaction_repository.create(transaction.clone());

        // AND an in memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // AND an in memory account service
        let mut account_service = AccountService::new(&mut account_repository);

        // AND a transaction service
        let transaction_service =
            TransactionService::new(&mut transaction_repository, &mut account_service);

        // WHEN querying by id
        let actual_transaction = transaction_service.find_by_id_or_fail(id);

        // THEN it should exist
        assert!(actual_transaction.is_ok());

        // AND they should equal.
        assert_transactions_equal(&transaction, &actual_transaction.unwrap(), false);
    }

    #[test]
    fn find_by_id_or_fail_error() {
        // GIVEN an in memory transaction repository
        let mut transaction_repository = InMemoryTransactionRepository::new();

        // AND an in memory account repository
        let mut account_repository = InMemoryAccountRepository::new();

        // AND an in memory account service
        let mut account_service = AccountService::new(&mut account_repository);

        // AND a transaction service
        let transaction_service =
            TransactionService::new(&mut transaction_repository, &mut account_service);

        // WHEN we find all transactions by a random entity id
        let random_entity_id = "1".into();
        let transaction = transaction_service.find_by_id_or_fail(random_entity_id);

        // THEN the response should be none
        assert!(transaction.is_err());

        // AND the error should be a not found error.
        assert!(matches!(
            transaction.err().unwrap(),
            GetOneError::NotFound(_)
        ));
    }
}
