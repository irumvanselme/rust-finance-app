use crate::app::entities::common::EntityId;
use crate::app::entities::common::EntityRef::{Id, Value};
use crate::app::entities::transaction::{Transaction, TransactionType};
use crate::app::repositories::transaction_repository::TransactionRepository;
use crate::app::services::account_service::AccountService;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GetOneError {
    #[error("The transaction with the id was not found")]
    NotFound(EntityId),
}

#[derive(Error, Debug)]
pub enum CreateError {
    #[error("The entity id should not be provided")]
    EntityIdProvided,

    #[error("The opening balance, should not be provided, it should be derived from the account balance")]
    OpeningBalanceProvided,

    #[error("The closing balance, should not be provided, it should be derived from the account balance")]
    ClosingBalanceProvided,

    #[error("The account reference is invalid")]
    InvalidAccountRef { account_id: Option<EntityId> },
}

pub struct TransactionService<'a> {
    repository: &'a mut dyn TransactionRepository,
    account_service: &'a mut AccountService<'a>,
}

impl<'a> TransactionService<'a> {
    pub fn new(
        repository: &'a mut dyn TransactionRepository,
        account_service: &'a mut AccountService<'a>,
    ) -> TransactionService<'a> {
        Self {
            repository,
            account_service,
        }
    }

    /// Retrieves all transactions stored in the repository.
    ///
    /// This method fetches all `Transaction` records managed by the repository
    ///
    /// # Returns
    /// * `&Vec<Transaction>` -- A reference to a vector containing all `Transaction` instances.
    pub fn find_all(&self) -> &Vec<Transaction> {
        self.repository.find_all()
    }

    pub fn create(&mut self, transaction: Transaction) -> Result<EntityId, CreateError> {
        // 1. Validate that optional fields are not provided

        // 1.1 See if the provided id already exists and throw an error
        //     Because the `create request` should not be able to provide an id
        if transaction.id().is_some() {
            return Err(CreateError::EntityIdProvided);
        }

        // 1.2 Opening balance should not be provided
        if transaction.opening_balance().is_some() {
            return Err(CreateError::OpeningBalanceProvided);
        }

        // 1.3 Closing balance should not be provided
        if transaction.closing_balance().is_some() {
            return Err(CreateError::ClosingBalanceProvided);
        }

        // 2. Get the account id, otherwise throw that it was not provided
        let account_id: EntityId = match transaction.account() {
            // 2.1 If the actual account is value, extract the id from it
            Value(account) => match account.id() {
                Some(id) => id.clone(),
                None => return Err(CreateError::InvalidAccountRef { account_id: None }),
            },
            Id(id) => id.clone(),
        };

        // 3. Verify that the account we have can be verified by the account service, otherwise it is an account that does not exist.
        let account = match self.account_service.find_by_id_or_fail(&account_id) {
            Ok(account) => account.clone(),
            Err(_) => {
                return Err(CreateError::InvalidAccountRef {
                    account_id: account_id.clone().into(),
                })
            }
        };

        // 4. Create a new transaction with the account as value
        let mut savable_transaction = transaction.clone();

        {
            // BLOCK: Scope for the account update and savable transaction of account related fields.
            // Set the opening balance to the current balance of the account
            savable_transaction.set_opening_balance(account.balance().clone().into());

            // Update the respective account with the new transaction
            let update_account_result = match transaction.transaction_type() {
                TransactionType::Expense => self
                    .account_service
                    .withdraw(&account_id, transaction.amount()),
                TransactionType::Income => self
                    .account_service
                    .deposit(&account_id, transaction.amount()),
            };

            // Handle the result of the update
            let new_account = match update_account_result {
                Ok(account) => account,
                Err(_) => {
                    return Err(CreateError::InvalidAccountRef {
                        account_id: account_id.clone().into(),
                    })
                }
            };

            // Update the closing balance of the transaction to the new balance of the account
            savable_transaction.set_closing_balance(new_account.balance().clone().into());

            // Set the account to the new account
            savable_transaction.set_account(Value(new_account));
        }

        Ok(self.repository.create(savable_transaction))
    }

    pub fn find_by_id(&self, id: EntityId) -> Option<&Transaction> {
        self.repository.find_by_id(id)
    }

    /// Finds a transaction by its ID or returns an error if not found
    pub fn find_by_id_or_fail(&self, id: EntityId) -> Result<&Transaction, GetOneError> {
        match self.repository.find_by_id(id.clone()) {
            Some(transaction) => Ok(transaction),
            None => Err(GetOneError::NotFound(id)),
        }
    }
}
