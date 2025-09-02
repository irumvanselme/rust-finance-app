use crate::app::entities::common::EntityId;
use crate::app::entities::transaction::Transaction;

pub trait TransactionRepository {
    /**
    Get all transaction
    */
    fn find_all(&self) -> &Vec<Transaction>;

    /**
    Add a new transaction
    */
    fn create(&mut self, transaction: Transaction) -> EntityId;

    /**
    Get a transaction by id
    */
    fn find_by_id(&self, id: EntityId) -> Option<&Transaction>;
}
