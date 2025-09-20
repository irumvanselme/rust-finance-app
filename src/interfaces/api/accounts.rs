use crate::app::entities::account::{Account, AccountType};
use crate::app::entities::common::EntityId;
use crate::app::typing::currency::Currency;
use crate::interfaces::api::state::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use utoipa;
use utoipa::ToSchema;
use utoipa_actix_web::service_config::ServiceConfig;

#[derive(Deserialize, ToSchema)]
struct CreateAccountRequest {
    /// Account name.
    name: String,

    /// Account description.
    description: String,

    /// Account platform.
    platform: String,

    /// Account type.
    account_type: AccountType,

    /// Account currency.
    currency: Currency,
}

const ACCOUNTS: &str = "Accounts";

/// Get all accounts.
#[utoipa::path(
    tag = ACCOUNTS,
    responses(
        (status = 200, description = "List current accounts items", body=[Account])
    )
)]
#[get("")]
async fn get_all_accounts(state: web::Data<AppState>) -> impl Responder {
    let account_service = state.account_service.lock().unwrap();
    let accounts = account_service.find_all();
    HttpResponse::Ok().json(accounts)
}

/// Get account by id.
#[utoipa::path(
    tag = ACCOUNTS,
    responses(
        (status = 200, description = "List current accounts items", body=Account)
    )
)]
#[get("/{id}")]
async fn get_by_id(state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let account_service = state.account_service.lock().unwrap();
    let entity_id: EntityId = id.clone().into();
    match account_service.find_by_id(entity_id) {
        Some(account) => HttpResponse::Ok().json(account),
        None => HttpResponse::NotFound().finish(),
    }
}

/// Create account.
#[utoipa::path(
    tag = ACCOUNTS,
    responses(
        (status = 200, description = "List current accounts items")
    )
)]
#[post("")]
async fn create_account(
    state: web::Data<AppState>,
    user_request: web::Json<CreateAccountRequest>,
) -> impl Responder {
    let account_service = state.account_service.lock().unwrap();

    let account: Account = Account::new(
        None,
        user_request.name.clone(),
        user_request.description.clone(),
        user_request.platform.clone(),
        user_request.account_type.clone(),
        Some(user_request.currency.clone()),
    );

    match account_service.create(account) {
        Ok(account) => HttpResponse::Ok().json(account),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }
}

pub(super) fn configure(config: &mut ServiceConfig) -> () {
    config
        .service(get_all_accounts)
        .service(get_by_id)
        .service(create_account);
}
