extern crate core;

use std::env;
use database::{Database};
use actix_web::{middleware::Logger, App, HttpServer};
use actix_web::web::Data;

mod schema;
mod database;
mod service;
mod middleware;

pub struct AppState {
    pub db: Database,
    pub secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = database::pool();
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: Database { connection: pool.clone() },
                secret: env::var("SECRET").expect("SECRET must be set")
            }))
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(service::configure)
    }).bind(("127.0.0.1", 8080))?
        .run()
        .await
}