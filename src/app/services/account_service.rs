use crate::app::entities::account::Account;
use crate::app::entities::common::EntityId;
use crate::app::repositories::account_repository::{AccountRepository, FindByIdAndUpdateError};
use crate::app::typing::amount::Amount;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FindByIdOrFailError<'a> {
    #[error("Account not found by the provided id")]
    NotFound(&'a EntityId),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CreateError {
    #[error("Account ID must not be provided")]
    EntityIdProvided,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum UpdateError {
    #[error("Account not found by the provided id")]
    EntityIdNotFound,

    #[error("Insufficient funds")]
    InsufficientFunds,
}

pub struct AccountService<'a> {
    repository: &'a mut dyn AccountRepository,
}

impl<'a> AccountService<'a> {
    pub fn new(repository: &'a mut dyn AccountRepository) -> AccountService {
        Self { repository }
    }

    /// Retrieves all `Account` objects from the repository.
    /// # Notes:
    /// Improve by making this API function return a stream or introduce a new method.
    /// # Returns
    /// * `&Vec<Account>` — A reference to the vector containing all `Account` objects.
    pub fn get_all(&self) -> &Vec<Account> {
        self.repository.find_all()
    }

    pub fn create(&mut self, account: Account) -> Result<EntityId, CreateError> {
        // The request to create an account must not have an ID.
        // If it does, throw an error. It should be provided by the repository because.
        // It is a unique identifier for the account. and the repository is responsible for generating it.
        match account.id() {
            Some(_) => return Err(CreateError::EntityIdProvided),
            None => (),
        }

        // Return the generated ID.
        Ok(self.repository.add(account))
    }

    fn find_account_to_update(&mut self, account_id: &EntityId) -> Result<Account, UpdateError> {
        // call self.find_by_id_or_fail() to check that the provided account_id exists.
        match self.find_by_id_or_fail(&account_id) {
            // If it exists in the repository, return the cloned value
            Ok(account) => Ok(account.clone()),
            // If it does not exist, return an error, with AccountServiceUpdateError::EntityIdNotFound
            Err(error) => {
                match error {
                    // We have a mapper function to convert the FindByIdOrFailError into UpdateError.
                    // Because it might fail for different reasons, and we handle them separately.
                    FindByIdOrFailError::NotFound(_) => Err(UpdateError::EntityIdNotFound),
                }
            }
        }
    }

    fn update_account(
        &mut self,
        account_id: &EntityId,
        account: Account,
    ) -> Result<EntityId, UpdateError> {
        match self
            .repository
            .find_by_id_and_update(account_id.clone(), account.clone())
        {
            Ok(entity_id) => Ok(entity_id),
            Err(error) => match error {
                FindByIdAndUpdateError::NotFound => Err(UpdateError::EntityIdNotFound),
            },
        }
    }

    pub fn withdraw(
        &mut self,
        account_id: &EntityId,
        withdrawn_amount: &Amount,
    ) -> Result<Account, UpdateError> {
        // Find the account to withdraw the amount it.
        // We are testing that the provided account_id to update has a corresponding account in the repository (data layer).
        // We are using the `?` Operator to unwrap the result. Which will return the same error if the account does not exist.
        let mut account = self.find_account_to_update(&account_id)?;

        // Check if the account has enough funds to withdraw the requested amount.
        // If not, throw an InsufficientFunds error.
        if account.balance() < &withdrawn_amount {
            return Err(UpdateError::InsufficientFunds);
        }

        // Update the account in in-place
        // With the provided setter function.
        account.withdraw(withdrawn_amount);

        // Update the account in the repository (data layer)
        // This should return the updated entity ID.
        let entity_id = self.update_account(&account_id, account)?;

        // Return the updated account
        // Assuming it will return an OK result,
        // since we have already asserted that the account exists.
        Ok(self.find_by_id_or_fail(&entity_id).unwrap().clone())
    }

    pub fn deposit(
        &mut self,
        account_id: &EntityId,
        deposited_amount: &Amount,
    ) -> Result<Account, UpdateError> {
        // Find the account to deposit the amount it.
        // We are testing that the provided account_id to update has a corresponding account in the repository (data layer).
        // We are using the `?` Operator to unwrap the result. Which will return the same error if the account does not exist.
        let mut account = self.find_account_to_update(&account_id)?;

        // Update the account in in-place
        // With the provided setter function.
        account.deposit(&deposited_amount);

        // Update the account in the repository (data layer)
        // This should return the updated entity ID.
        let entity_id = self.update_account(&account_id, account)?;

        // Return the updated account
        // Assuming it will return an OK result,
        // since we have already asserted that the account exists.
        Ok(self.find_by_id_or_fail(&entity_id).unwrap().clone())
    }

    pub fn find_by_id(&self, id: EntityId) -> Option<&Account> {
        self.repository.find_by_id(id)
    }

    ///  Retrieves an `Account` object from the repository by its ID.
    ///  #### Arguments
    ///  * `id` — The ID of the `Account` object to retrieve.
    ///  #### Returns
    ///  * `Result<&Account, FindByIdOrFailError>` — A `Result` containing the `Account` object if it exists, or an error if it does not.
    ///  #### Errors
    ///  * `FindByIdOrFailError::NotFound` — If the `Account` object with the provided ID does not exist.
    pub fn find_by_id_or_fail<'b>(
        &self,
        id: &'b EntityId,
    ) -> Result<&Account, FindByIdOrFailError<'b>> {
        match self.repository.find_by_id(id.clone()) {
            Some(account) => Ok(account),
            None => Err(FindByIdOrFailError::NotFound(id)),
        }
    }
}
