use crate::interfaces::api::accounts;
use crate::interfaces::api::state::AppState;
use actix_web::{middleware::Logger, web, App, HttpServer};
use utoipa::{Modify, OpenApi};
use utoipa_actix_web::AppExt;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

async fn index() -> Result<String, std::io::Error> {
    let data = "Welcome to the API server!".to_string();
    Ok(data)
}

pub async fn start_server(address: &str) {
    // Initialize logger and allow info level.
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = "accounts", description = "Accounts related endpoints")
        ),
    )]
    struct ApiDoc;

    HttpServer::new(move || {
        let app_state = AppState::new();

        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .service(utoipa_actix_web::scope("/accounts").configure(accounts::configure))
            .map(|app| {
                app.wrap(Logger::default())
                    .app_data(web::Data::new(app_state))
            })
            .route("/", web::get().to(index))
            .openapi_service(|api| Scalar::with_url("/scalar", api))
            .into_app()
    })
    .bind(address)
    .unwrap()
    .run()
    .await
    .expect("Failed to start server");
}
