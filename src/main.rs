use dotenvy::dotenv;

mod app;
mod infrastructure;
mod interfaces;
mod shared;

use interfaces::api::server::start_server;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // The address to listen on for HTTP requests.
    let address = "127.0.0.1:8000";

    // Start the server with the address
    start_server(address).await;
}
