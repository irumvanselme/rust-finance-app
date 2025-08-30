use crate::domain::entities::account::Account;
use crate::domain::repositories::account_repository::AccountRepository;

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

impl AccountRepository<usize> for InMemoryAccountRepository {
    fn get_all(&self) -> Vec<Account> {
        self.accounts.clone()
    }

    fn add(&mut self, account: &Account) -> usize {
        let id = self.next_id;
        self.accounts.push(account.clone());
        self.next_id += 1;
        id
    }

    fn get(&self, id: usize) -> Option<Account> {
        self.accounts.get(id).cloned()
    }
}
