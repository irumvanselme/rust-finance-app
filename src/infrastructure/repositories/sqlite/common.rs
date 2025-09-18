use dotenvy::dotenv;
use sqlite::Connection;
use std::env;

pub fn establish_sqlite_connection() -> Connection {
    dotenv().ok();
    let database_file = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    sqlite::open(database_file).unwrap()
}
