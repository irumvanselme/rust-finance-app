use crate::app::entities::account::Account;
use crate::app::entities::common::EntityId;
use crate::app::repositories::account_repository::AccountRepository;

#[derive(Debug)]
pub enum AccountServiceError {
    NotFound(EntityId),
}

pub struct AccountService<'a> {
    repository: &'a mut dyn AccountRepository,
}

impl<'a> AccountService<'a> {
    pub fn new(repository: &'a mut dyn AccountRepository) -> AccountService {
        Self { repository }
    }

    /// Retrieves all `Account` objects from the repository.
    /// # Returns
    /// * `&Vec<Account>` — A reference to the vector containing all `Account` objects.
    pub fn get_all(&self) -> &Vec<Account> {
        // TODO: Improve this method to stream instead of reading everything because accounts can be huge.
        self.repository.get_all()
    }

    pub fn save(&mut self, account: Account) -> EntityId {
        self.repository.add(account)
    }

    pub fn find_by_id(&self, id: EntityId) -> Option<&Account> {
        self.repository.get(id)
    }

    /// Finds an account by its ID or returns an error if not found
    pub fn find_by_id_or_fail(&self, id: EntityId) -> Result<&Account, AccountServiceError> {
        match self.repository.get(id.clone()) {
            Some(account) => Ok(account),
            None => Err(AccountServiceError::NotFound(id)),
        }
    }
}
