use crate::app::entities::account::Account;
use crate::app::entities::common::EntityId;
use crate::app::repositories::account_repository::{
    AccountRepository, CreateError, FindByIdAndUpdateError,
};
use std::error::Error;

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
    fn find_all(&self) -> Vec<Account> {
        // GET all accounts in the memory
        self.accounts.clone()
    }

    fn find_by_id(&self, id: EntityId) -> Option<Account> {
        // GET an account by an id in the memory
        let id: usize = id.0.parse().unwrap();
        self.accounts.get(id).cloned()
    }

    fn create(&mut self, account: Account) -> Result<EntityId, CreateError> {
        // Add an account to the memory
        let id = self.next_id;
        self.accounts.push(account);
        self.next_id += 1;
        Ok(EntityId(id.to_string()))
    }

    fn find_by_id_and_update(
        &mut self,
        id: EntityId,
        account: Account,
    ) -> Result<EntityId, FindByIdAndUpdateError> {
        // Find an account by id and update it in the memory
        match self.find_by_id(id.clone()) {
            Some(account) => account,
            None => return Err(FindByIdAndUpdateError::NotFound),
        };

        let index: usize = id.0.parse().unwrap();

        // Update the self.accounts[index
        self.accounts[index] = account;

        Ok(id)
    }
}
