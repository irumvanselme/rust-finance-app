use crate::app::entities::account::Account;
use crate::app::entities::common::EntityId;

pub trait AccountRepository {
    /**
    Get all accounts
    */
    fn get_all(&self) -> &Vec<Account>;

    /**
    Add a new account
    */
    fn add(&mut self, account: Account) -> EntityId;

    /**
    Get an account by id
    */
    fn get(&self, id: EntityId) -> Option<&Account>;
}
