use crate::domain::entities::account::Account;

pub trait AccountRepository<ID> {
    /**
        Get all accounts
    */
    fn get_all(&self) -> Vec<Account>;

    /**
        Add a new account
    */
    fn add(&mut self, account: &Account) -> ID;

    /**
        Get an account by id
    */
    fn get(&self, id: ID) -> Option<Account>;
}
