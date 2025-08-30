use crate::app::entities::account::Account;
use crate::app::entities::common::EntityId;
use crate::app::repositories::account_repository::AccountRepository;

pub struct InMemoryAccountRepository {
    next_id: usize,
    accounts: Vec<Account>,
}

impl InMemoryAccountRepository {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            accounts: vec![],
        }
    }
}

impl AccountRepository for InMemoryAccountRepository {
    fn get_all(&self) -> &Vec<Account> {
        &self.accounts
    }

    fn add(&mut self, account: Account) -> EntityId {
        let id = self.next_id;
        self.accounts.push(account);
        self.next_id += 1;
        EntityId(id.to_string())
    }

    fn get(&self, id: EntityId) -> Option<&Account> {
        let id: usize = id.0.parse().unwrap();
        self.accounts.get(id)
    }
}
