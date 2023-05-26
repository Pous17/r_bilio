use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{NewBook, Books, Borrows, NewBorrow, Users, NewUser, Employees, NewEmployees, PastBorrows, NewPastBorrow, Author, NewAuthor};

pub mod schema;
pub mod models;

pub fn connection() -> PgConnection {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").expect("No DATABASE_URL has been set");

    PgConnection::establish(&database_url) 
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_book(conn: &mut PgConnection, name: &str, author_id: &i32, author_firstname: &str, author_lastname: &str, created_by: &str) -> Books {
    use crate::schema::books;

    let new_book = NewBook {name, author_id, author_firstname, author_lastname, created_by};

    diesel::insert_into(books::table)
        .values(&new_book)
        .get_result(conn)
        .expect("Error adding new book")

}

pub fn create_author(conn: &mut PgConnection, firstname: &str, lastname: &str) -> Author {
    use crate::schema::author;

    let new_author = NewAuthor {firstname, lastname};
    
    diesel::insert_into(author::table)
        .values(&new_author)
        .get_result(conn)
        .expect("Error inserting a author")
}

pub fn create_borrow(conn: &mut PgConnection, user_id: &i32, book_id: &i32, borrow_date: &str) -> Borrows {
    use crate::schema::borrows;
 
    let new_borrow = NewBorrow {user_id, book_id, borrow_date};

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

pub fn create_employee(conn: &mut PgConnection, name: &str) -> Employees {
    use crate::schema::employees;

    let new_employee = NewEmployees {name};

    diesel::insert_into(employees::table)
        .values(new_employee)
        .get_result(conn)
        .expect("Error creating a user")
}

pub fn delete_borrow(conn: &mut PgConnection, borrow_id: &i32) {
    use self::schema::borrows::dsl::*;

    diesel::delete(borrows.find(borrow_id))
    .get_result::<Borrows>(conn)
    .unwrap();
}

pub fn borrow_status(conn: &mut PgConnection, id: &i32, param: &bool) {
    use self::schema::books::dsl::{books, borrowed};

    diesel::update(books.find(id))
        .set(borrowed.eq(param))
        .get_result::<Books>(conn)
        .unwrap();
}

pub fn add_past_borrow(conn: &mut PgConnection, user_id: &i32, book_id: &i32, condition: &bool, borrow_date: &str, return_date: &str) -> PastBorrows {
    use self::schema::past_borrows;

    let new_past_borrow = NewPastBorrow {user_id, book_id, condition, borrow_date, return_date};

    diesel::insert_into(past_borrows::table)
        .values(new_past_borrow)
        .get_result(conn)
        .expect("Error creating a past borrow")
}

pub fn update_membership(conn: &mut PgConnection, user_id: &i32, param: &bool) -> Users {
    use self::schema::users::dsl::{users, member};

    diesel::update(users.find(user_id))
        .set(member.eq(param))
        .get_result::<Users>(conn)
        .unwrap()
}

pub fn down_score(conn: &mut PgConnection, user_id: &i32) -> Users {
    use self::schema::users::dsl::{users, score};

    diesel::update(users.find(user_id))
        .set(score.eq(score - 1))
        .get_result::<Users>(conn)
        .unwrap()
}