use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{NewBook, Books};

pub mod schema;
pub mod models;

pub fn connection() -> PgConnection {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").expect("No DATABASE_URL has been set");

    PgConnection::establish(&database_url) 
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_book(conn: &mut PgConnection, name: &str, publisher: &str, borrowed: &bool) -> Books {
    use crate::schema::books;

    let new_book = NewBook {name, publisher, borrowed};

    diesel::insert_into(books::table)
        .values(&new_book)
        .get_result(conn)
        .expect("Error adding new book")
}