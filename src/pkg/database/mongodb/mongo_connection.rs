use mongodb::{Client, options::ClientOptions};
use dotenv::dotenv;
use std::env;


#[allow(dead_code)]
pub fn new_mongo_db() -> Client {
    dotenv().ok();
    let mongo_url = env::var("MONGO_DB").expect("MONGO_URL must be set").to_owned();

    let db_url_str : &str = &mongo_url[..];

    let mut client_options = ClientOptions::parse(db_url_str).await? ;

    Client::with_options(client_options)?
}
