use crate::app::entities::common::EntityId;
use crate::app::entities::common::EntityRef::{Id, Value};
use crate::app::entities::transaction::{Transaction, TransactionType};
use crate::app::repositories::transaction_repository::TransactionRepository;
use crate::app::services::account_service::AccountService;

#[derive(Debug)]
pub enum TransactionServiceGetOneError {
    NotFound(EntityId),
}

#[derive(Debug)]
pub enum TransactionServiceCreateError {
    EntityIdProvided,
    OpeningBalanceProvided,
    ClosingBalanceProvided,
    InvalidAccountRef { account_id: Option<EntityId> },
}

struct TransactionService<'a> {
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

    pub fn find_all(&self) -> &Vec<Transaction> {
        self.repository.get_all()
    }

    pub fn create(
        &mut self,
        transaction: Transaction,
    ) -> Result<EntityId, TransactionServiceCreateError> {
        // 1. Validate that optional fields are not provided
        //
        // 1.1  See if the provided id already exists and throw an error
        //      Because the `create request` should not be able to provide an id
        if transaction.id().is_some() {
            return Err(TransactionServiceCreateError::EntityIdProvided);
        }

        // 1.2 Opening balance should not be provided
        if transaction.opening_balance().is_some() {
            return Err(TransactionServiceCreateError::OpeningBalanceProvided);
        }

        // 1.3 Closing balance should not be provided
        if transaction.closing_balance().is_some() {
            return Err(TransactionServiceCreateError::ClosingBalanceProvided);
        }

        // 2. Get the account id, otherwise throw that it was not provided
        let account_id: EntityId = match transaction.account() {
            // 2.1 If the actual account is value, extract the id from it
            Value(account) => match account.id() {
                Some(id) => id.clone(),
                None => {
                    return Err(TransactionServiceCreateError::InvalidAccountRef {
                        account_id: None,
                    })
                }
            },
            Id(id) => id.clone(),
        };

        // 3. Verify that the account we have can be verified by the account service, otherwise it is an account that does not exist.
        let account = match self.account_service.find_by_id_or_fail(&account_id) {
            Ok(account) => account.clone(),
            Err(_) => {
                return Err(TransactionServiceCreateError::InvalidAccountRef {
                    account_id: account_id.clone().into(),
                })
            }
        };

        // 4. Create a new transaction with the account as value
        let mut savable_transaction = transaction.clone();

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
                return Err(TransactionServiceCreateError::InvalidAccountRef {
                    account_id: account_id.clone().into(),
                })
            }
        };

        // Update the closing balance of the transaction to the new balance of the account
        savable_transaction.set_closing_balance(new_account.balance().clone().into());

        // Set the account to the new account
        savable_transaction.set_account(Value(new_account));

        Ok(self.repository.add(savable_transaction))
    }

    pub fn find_by_id(&self, id: EntityId) -> Option<&Transaction> {
        self.repository.get(id)
    }

    /// Finds a transaction by its ID or returns an error if not found
    pub fn find_by_id_or_fail(
        &self,
        id: EntityId,
    ) -> Result<&Transaction, TransactionServiceGetOneError> {
        match self.repository.get(id.clone()) {
            Some(transaction) => Ok(transaction),
            None => Err(TransactionServiceGetOneError::NotFound(id)),
        }
    }
}
