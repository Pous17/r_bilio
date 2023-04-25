use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn connection() -> PgConnection {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").expect("No DATABASE_URL has been set");

    PgConnection::establish(&database_url) 
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}