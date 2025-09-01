use crate::app::entities::account::Account;
use crate::app::entities::common::EntityId;

#[derive(Debug, PartialEq)]
pub enum FindByIdAndUpdateError {
    NotFound,
}

pub trait AccountRepository {
    /// Find all accounts.
    fn find_all(&self) -> &Vec<Account>;

    /// Find an account by ID.
    /// Return None if not found
    fn find_by_id(&self, id: EntityId) -> Option<&Account>;

    /**
    Add a new account
    */
    fn add(&mut self, account: Account) -> EntityId;

    /// Find an account by ID and update it.
    /// It will throw a FindByIdAndUpdateError::NotFound if the provided id doesn't have any corresponding document in the db.
    fn find_by_id_and_update(
        &mut self,
        id: EntityId,
        account: Account,
    ) -> Result<EntityId, FindByIdAndUpdateError>;
}
