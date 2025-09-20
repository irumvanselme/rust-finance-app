use crate::app::services::account_service::AccountService;
use crate::infrastructure::repositories::in_memory::account_repository::InMemoryAccountRepository;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub account_service: Arc<Mutex<AccountService<InMemoryAccountRepository>>>,
}

impl AppState {
    pub fn new() -> Self {
        let account_repository = Arc::new(Mutex::new(InMemoryAccountRepository::new()));
        let account_service = Arc::new(Mutex::new(AccountService::new(account_repository)));

        Self { account_service }
    }
}
