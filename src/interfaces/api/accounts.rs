use crate::app::entities::account::{Account, AccountType};
use crate::app::typing::currency::Currency;
use crate::app::entities::common::EntityId;
use crate::interfaces::api::state::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateAccountRequest {
    name: String,
    description: String,
    platform: String,
    account_type: AccountType,
    currency: Currency,
}

#[get("")]
async fn get_all_accounts(state: web::Data<AppState>) -> impl Responder {
    let account_service = state.account_service.lock().unwrap();
    let accounts = account_service.find_all();
    HttpResponse::Ok().json(accounts)
}

#[get("{id}")]
async fn get_by_id(state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let account_service = state.account_service.lock().unwrap();
    let entity_id: EntityId = id.clone().into();
    match account_service.find_by_id(entity_id) {
        Some(account) => HttpResponse::Ok().json(account),
        None => HttpResponse::NotFound().finish(),
    }
}

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


pub(crate) fn accounts_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/accounts")
            .service(get_all_accounts)
            .service(create_account)
            .service(get_by_id)
    );
}
