use crate::app::entities::common::EntityId;
use crate::app::entities::transaction::Transaction;
use crate::app::repositories::transaction_repository::TransactionRepository;

pub struct InMemoryTransactionRepository {
    next_id: usize,
    transactions: Vec<Transaction>,
}

impl InMemoryTransactionRepository {
    pub fn new() -> Self {
        Self {
            next_id: Default::default(),
            transactions: Default::default(),
        }
    }
}

impl TransactionRepository for InMemoryTransactionRepository {
    fn get_all(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    fn add(&mut self, mut transaction: Transaction) -> EntityId {
        let id = EntityId(self.next_id.to_string());
        transaction.set_id(Some(id.clone()));
        self.transactions.push(transaction);

        self.next_id += 1;

        id
    }

    fn get(&self, id: EntityId) -> Option<&Transaction> {
        let id: usize = id.0.parse().unwrap();
        self.transactions.get(id)
    }
}
