use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{NewBook, Books, Borrows, NewBorrow, Users, NewUser};

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

pub fn create_borrow(conn: &mut PgConnection, user_id: &i32, book_id: &i32) -> Borrows {
    use crate::schema::borrows;
 
    let new_borrow = NewBorrow {user_id, book_id};

    diesel::insert_into(borrows::table)
        .values(new_borrow)
        .get_result(conn)
        .expect("Error inserting a borrow")
}

pub fn create_user(conn: &mut PgConnection, name: &str, member: &bool) -> Users {
    use crate::schema::users;

    let new_user = NewUser {name, member};

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
        .expect("Error creating a user")
}

// add change book availability 