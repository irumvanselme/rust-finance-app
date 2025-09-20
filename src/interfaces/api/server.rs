use crate::interfaces::api::accounts::accounts_routes;
use crate::interfaces::api::state::AppState;
use actix_web::{middleware::Logger, web, App, HttpServer};

async fn index() -> Result<String, std::io::Error> {
    let data = "Welcome to the API server!".to_string();
    Ok(data)
}

pub async fn start_server(address: &str) {
    // Initialize logger and allow info level.
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        let app_state = AppState::new();

        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state))
            .route("/", web::get().to(index))
            .configure(accounts_routes)
    })
    .bind(address)
    .unwrap()
    .run()
    .await
    .expect("Failed to start server");
}
