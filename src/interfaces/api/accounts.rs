use actix_web::{get, web, HttpResponse, Responder};

#[get("")]
async fn get_all_accounts() -> impl Responder {
    HttpResponse::Ok().body("All Accounts")
}

pub(crate) fn accounts_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/accounts").service(get_all_accounts));
}
