#[cfg(test)]
mod test_transaction_repository {
    use crate::app::repositories::transaction_repository::TransactionRepository;
    use crate::infrastructure::database::repositories::in_memory::transaction_repository::InMemoryTransactionRepository;
    use crate::shared::test_utilities::get_random_transaction;

    #[test]
    fn test_empty_get_all() {
        // GIVEN the in memory transaction repository is initialized.
        // AND no items are added in the repository.
        let in_memory_transaction_repository = InMemoryTransactionRepository::new();

        // WHEN the get_all method is called
        let transactions = in_memory_transaction_repository.get_all();

        // THEN the result is empty
        assert_eq!(transactions.len(), 0);
    }

    #[test]
    fn test_items_added() {
        // GIVEN the in memory transactions repository is initialized.
        let mut transactions_repository = InMemoryTransactionRepository::new();

        // WHEN an transaction is added in the repository
        let mut transaction = get_random_transaction();

        let transaction_id = transactions_repository.add(transaction.clone());

        // AND the get_all method is called
        let transactions = transactions_repository.get_all();

        // THEN the result is not empty
        assert_eq!(transactions.len(), 1);

        // AND the first item should be the same as the one input
        let first_transaction = transactions.get(0).unwrap();
        transaction.set_id(Some(transaction_id));
        assert_eq!(*first_transaction, transaction);
    }

    #[test]
    fn test_find_by_id() {
        // GIVEN the in memory account repository is initialized.
        let mut transactions_repository = InMemoryTransactionRepository::new();

        // AND an account is added in the repository
        let mut transaction = get_random_transaction();
        let new_id = transactions_repository.add(transaction.clone());
        transaction.set_id(Some(new_id.clone()));

        // WHEN the new id is queried
        let created_account = transactions_repository.get(new_id);

        // THEN the result is the same as the one input
        assert_eq!(created_account, Some(&transaction));
    }
}
