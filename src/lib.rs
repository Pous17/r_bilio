use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{User, NewUser, Author, NewAuthor, Employee, NewEmployee, Book, NewBook, Borrow,
    NewBorrow, PastBorrow, NewPastBorrow};

pub mod schema;
pub mod models;

pub fn connection() -> PgConnection {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").expect("No DATABASE_URL has been set");

    PgConnection::establish(&database_url) 
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &mut PgConnection, _firstname: &str, _lastname: &str, _password: &str,
    _created_by: &str, _created_at: &str) -> User {
    use crate::schema::users;

    let _login = format!("{}{}", _firstname.split_at(1).0, _lastname);
    let new_user = NewUser {
        firstname: _firstname,
        lastname: _lastname,
        role: "user",
        login: &_login,
        password: _password,
        created_by: _created_by,
        created_at: _created_at,
        last_updated_by: _created_by,
        last_updated_at: _created_at,
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
        .expect("Error creating a user")
}

pub fn update_membership(conn: &mut PgConnection, user_id: &i32, param: &bool) -> User {
    use self::schema::users::dsl::{users, member};

    diesel::update(users.find(user_id))
        .set(member.eq(param))
        .get_result::<User>(conn)
        .unwrap()
}

pub fn down_score(conn: &mut PgConnection, user_id: &i32) -> User {
    use self::schema::users::dsl::{users, score};

    diesel::update(users.find(user_id))
        .set(score.eq(score - 1))
        .get_result::<User>(conn)
        .unwrap()
}

pub fn create_author(conn: &mut PgConnection, _firstname: &str, _lastname: &str, _created_by: &str,
    _created_at: &str) -> Author {
    use crate::schema::authors;

    let new_author = NewAuthor {
        firstname: _firstname,
        lastname: _lastname,
        created_by: _created_by,
        created_at: _created_at,
        last_updated_by: _created_by,
        last_updated_at: _created_at,
    };
    
    diesel::insert_into(authors::table)
        .values(&new_author)
        .get_result(conn)
        .expect("Error inserting a author")
}

pub fn create_employee(conn: &mut PgConnection, _firstname: &str, _lastname: &str, _password: &str,
    _created_by: &str, _created_at: &str) -> Employee {
    use crate::schema::employees;

    let _login = format!("{}{}", _firstname.split_at(1).0, _lastname);
    let new_employee = NewEmployee {
        active: &true,
        firstname: _firstname,
        lastname: _lastname,
        role: "employee",
        login: &_login,
        password: _password,
        created_by: _created_by,
        created_at: _created_at,
        last_updated_by: _created_by,
        last_updated_at: _created_at,
    };

    diesel::insert_into(employees::table)
        .values(new_employee)
        .get_result(conn)
        .expect("Error creating a user")
}

pub fn create_book(conn: &mut PgConnection, _name: &str, _created_by: &str, _created_at: &str,
    _author_id: &i32, _author_firstname: &str, _author_lastname: &str) -> Book {
    use crate::schema::books;

    let new_book = NewBook {
        borrowed: &false,
        active: &true,
        name: _name,
        created_by: _created_by,
        created_at: _created_at,
        last_updated_by: _created_by,
        last_updated_at: _created_at,
        author_id: _author_id,
        author_firstname: _author_firstname,
        author_lastname: _author_lastname,
    };

    diesel::insert_into(books::table)
        .values(&new_book)
        .get_result(conn)
        .expect("Error adding new book")

}

pub fn create_borrow(conn: &mut PgConnection, _borrow_date: &str, _limit_date: &str, _created_by: &str,
    _created_at: &str, _user_id: &i32, _book_id: &i32) -> Borrow {
    use crate::schema::borrows;
 
    let new_borrow = NewBorrow {
        borrow_date: _borrow_date,
        limit_date: _limit_date,
        created_by: _created_by,
        created_at: _created_at,
        last_updated_by: _created_by,
        last_updated_at: _created_at,
        user_id: _user_id,
        book_id: _book_id,
    };

    diesel::insert_into(borrows::table)
        .values(new_borrow)
        .get_result(conn)
        .expect("Error inserting a borrow")
}

pub fn borrow_status(conn: &mut PgConnection, id: &i32, param: &bool) {
    use self::schema::books::dsl::{books, borrowed};

    diesel::update(books.find(id))
        .set(borrowed.eq(param))
        .get_result::<Book>(conn)
        .unwrap();
}

pub fn delete_borrow(conn: &mut PgConnection, borrow_id: &i32) {
    use self::schema::borrows::dsl::*;

    diesel::delete(borrows.find(borrow_id))
    .get_result::<Borrow>(conn)
    .unwrap();
}

pub fn create_past_borrow(conn: &mut PgConnection, _condition: &bool, _borrow_date: &str, _limit_date: &str,
    _return_date: &str, _created_by: &str, _created_at: &str, _user_id: &i32, _book_id: &i32) -> PastBorrow {
    use self::schema::past_borrows;

    let new_past_borrow = NewPastBorrow {
        condition: _condition,
        borrow_date: _borrow_date,
        limit_date: _limit_date,
        return_date: _return_date,
        created_by: _created_by,
        created_at: _created_at,
        last_updated_by: _created_by,
        last_updated_at: _created_at,
        user_id: _user_id,
        book_id: _book_id,
    };

    diesel::insert_into(past_borrows::table)
        .values(new_past_borrow)
        .get_result(conn)
        .expect("Error creating a past borrow")
}
