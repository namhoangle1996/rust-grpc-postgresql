// https://docs.rs/postgres/0.15.2/postgres/
extern crate postgres;
extern crate redis;
extern crate dotenv;
use dotenv::dotenv;

extern crate chrono;

pub mod user {
    tonic::include_proto!("user");
}

use tonic::{transport::Server};

use user::{
    user_service_server::{UserServiceServer},
};

extern crate uuid;

extern crate console;
use console::Style;

#[path = "./pkg/database/postgresql/mod.rs"]
mod postgresql;

#[path = "./pkg/database/redis/mod.rs"]
mod redis_connection;

mod service;
use crate::service::User;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let mut port = env::var("PORT").expect("PORT must be set").to_owned();

    port = format!("0.0.0.0:{:}", port) ;

    let addr = port.parse().unwrap();
    let user = User::default();

    let blue = Style::new()
        .blue();

    println!("\nRust gRPC Server ready at {}", blue.apply_to(addr));

    Server::builder()
        .add_service(UserServiceServer::new(user))
        .serve(addr)
        .await?;

    Ok(())
}



