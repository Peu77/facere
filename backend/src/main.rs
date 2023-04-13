mod database;

use std::env;
use database::{Database};
use actix_web::{middleware::Logger, App, HttpServer};
use actix_web::web::Data;

mod schema;

mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = database::pool();
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Database { connection: pool.clone() }))
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(service::configure)
    }).bind(("127.0.0.1", 8080))?
        .run()
        .await
}

