use crate::app::entities::account::Account;
use crate::app::entities::common::EntityId;
use crate::app::repositories::account_repository::{
    AccountRepository, CreateError, FindByIdAndUpdateError,
};
use sqlite::{Connection, State};

pub struct SQliteAccountRepository {
    connection: Connection,
}

impl SQliteAccountRepository {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }
}

impl AccountRepository for SQliteAccountRepository {
    fn find_all(&self) -> Vec<Account> {
        let mut query = self.connection.prepare("SELECT * from accounts").unwrap();

        let mut accounts: Vec<Account> = vec![];
        while let Ok(State::Row) = query.next() {
            let account_id = query.read::<i64, _>("id").unwrap();
            let account_name = query.read::<String, _>("name").unwrap();
            let account_description = query.read::<String, _>("description").unwrap();
            let account_platform = query.read::<String, _>("platform").unwrap();
            let account_type = query.read::<String, _>("account_type").unwrap();
            let account_currency = query.read::<String, _>("currency").unwrap();

            let account = Account::new(
                Some(account_id.into()),
                account_name,
                account_description,
                account_platform,
                account_type.try_into().unwrap(),
                Some(account_currency.try_into().unwrap()),
            );

            accounts.push(account)
        }

        accounts
    }

    fn find_by_id(&self, id: EntityId) -> Option<Account> {
        todo!()
    }

    fn create(&mut self, account: Account) -> Result<EntityId, CreateError> {
        let query = "INSERT INTO accounts (name, description, platform, account_type, currency) VALUES (?, ?, ?, ?, ?) RETURNING id";
        let mut statement = self.connection.prepare(query).unwrap();

        let currency: &str = &account.currency().to_string();
        let account_type: &str = &account.account_type().to_string();

        statement.bind((1, account.name().as_str())).unwrap();
        statement.bind((2, account.description().as_str())).unwrap();
        statement.bind((3, account.platform())).unwrap();
        statement.bind((4, account_type)).unwrap();
        statement.bind((5, currency)).unwrap();

        while let Ok(State::Row) = statement.next() {
            let id = statement.read::<i64, _>(0).unwrap();
            return Ok(id.into());
        }

        Err(CreateError::FailedToCreateAccount)
    }

    fn find_by_id_and_update(
        &mut self,
        id: EntityId,
        account: Account,
    ) -> Result<EntityId, FindByIdAndUpdateError> {
        todo!()
    }
}
