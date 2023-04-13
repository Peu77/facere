pub mod models;

use std::env;
use diesel::{PgConnection, r2d2};
use dotenvy::dotenv;

use diesel::r2d2::{ConnectionManager, Pool};

pub fn pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    pool
}

pub struct Database {
    pub connection: Pool<ConnectionManager<PgConnection>>,
}