// https://docs.rs/postgres/0.15.2/postgres/
extern crate rustracing_jaeger;
extern crate rustracing;
use rustracing::sampler::AllSampler;

use rustracing_jaeger::Tracer;
use rustracing_jaeger::reporter::JaegerCompactReporter;

extern crate postgres;
extern crate redis;
extern crate dotenv;
extern crate mongodb;
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

#[path = "./pkg/database/mongodb/mod.rs"]
mod mongo_connection;

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

    let magenta_style = Style::new()
        .magenta();

    println!("\n gRPC Server ready at {}", magenta_style.apply_to(addr));

    let (span_tx, span_rx) = crossbeam_channel::bounded(10);
    let tracer = Tracer::with_sender(AllSampler, span_tx);
    {
        let span = tracer.span("grpc_main_service").start();
        // Do something

    }

    let span = span_rx.try_recv().unwrap();
    println!( "get span {:?}", span) ;

    let reporter = JaegerCompactReporter::new("rust_grpc_service").unwrap();
    reporter.report(&[span]).unwrap();

    Server::builder()
        .add_service(UserServiceServer::new(user))
        .serve(addr)
        .await?;

    Ok(())
}



