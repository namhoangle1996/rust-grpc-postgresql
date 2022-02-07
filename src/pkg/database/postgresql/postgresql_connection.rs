use postgres::{Client, NoTls};
use dotenv::dotenv;
use std::env;

#[allow(dead_code)]
pub fn new_postgresql_db() -> Client {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set").to_owned();

    let db_url_str : &str = &database_url[..];

    Client::connect(db_url_str, NoTls).unwrap()
}

