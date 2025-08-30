use crate::app::entities::common::EntityId;
use crate::app::entities::transaction::Transaction;

pub trait TransactionRepository {
    /**
    Get all transaction
    */
    fn get_all(&self) -> &Vec<Transaction>;

    /**
    Add a new transaction
    */
    fn add(&mut self, transaction: Transaction) -> EntityId;

    /**
    Get a transaction by id
    */
    fn get(&self, id: EntityId) -> Option<&Transaction>;
}
