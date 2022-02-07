use redis::{Connection};
use dotenv::dotenv;
use std::env;

pub fn new_redis_connection() -> Connection {
    dotenv().ok();

    let redis_host = env::var("REDIS_HOST").expect("REDIS_HOST must be set").to_owned();

    let redis_password = env::var("REDIS_PASSWORD").expect("REDIS_PASSWORD must be set").to_owned();

    let redis_conn_url = format!("redis://:{}@{}", redis_password, redis_host);

    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

