use diesel::{pg::PgConnection};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use self::models::{User, NewUser, Author, NewAuthor, Employee, NewEmployee, Book, NewBook, Borrow, NewBorrow};

pub mod schema;
pub mod models;


pub fn connection() -> PgConnection {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").expect("No DATABASE_URL has been set");

    PgConnection::establish(&database_url) 
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn update_password(conn: &mut PgConnection, account_id: &i32, new_password: &str, is_user: bool, updated_by: &str, updated_at: &str) {
    use self::schema::users::dsl::{users, password as user_password, last_updated_by as user_last_updated_by, last_updated_at as user_last_updated_at};
    use self::schema::employees::dsl::{employees, password as empl_password, last_updated_by as empl_last_updated_by, last_updated_at as empl_last_updated_at};

    if is_user {
        diesel::update(users.find(account_id))
            .set((
                user_password.eq(new_password),
                user_last_updated_by.eq(updated_by),
                user_last_updated_at.eq(updated_at)
            ))
            .get_result::<User>(conn)
            .unwrap();
    } else {
        diesel::update(employees.find(account_id))
            .set((
                empl_password.eq(new_password),
                empl_last_updated_by.eq(updated_by),
                empl_last_updated_at.eq(updated_at)
            ))
            .get_result::<Employee>(conn)
            .unwrap();
    }
}


pub fn create_user(conn: &mut PgConnection, _membership: bool, _firstname: &str, _lastname: &str, _password: &str,
    _created_by: &str, _created_at: &str) -> User {
    use crate::schema::users;

    let _login = format!("{}{}", _firstname.split_at(1).0.to_lowercase(), _lastname.to_lowercase());
    let new_user = NewUser {
        member: &_membership,
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


pub fn update_membership(conn: &mut PgConnection, user_id: &i32, param: &bool, updated_by: &str, updated_at: &str) -> User {
    use self::schema::users::dsl::{users, member, last_updated_by, last_updated_at};

    diesel::update(users.find(user_id))
        .set((
            member.eq(param),
            last_updated_by.eq(updated_by),
            last_updated_at.eq(updated_at)
        ))
        .get_result::<User>(conn)
        .unwrap()
}


pub fn update_score(conn: &mut PgConnection, user_id: &i32, update_value: &i32, updated_by: &str, updated_at: &str) -> User {
    use self::schema::users::dsl::{users, score, last_updated_by, last_updated_at};

    diesel::update(users.find(user_id))
        .set((
            score.eq(score + update_value),
            last_updated_by.eq(updated_by),
            last_updated_at.eq(updated_at)
        ))
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

    let _login = format!("{}{}", _firstname.split_at(1).0.to_lowercase(), _lastname.to_lowercase());
    let new_employee = NewEmployee {
        is_active: &true,
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
        is_active: &true,
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


pub fn archive_book(conn: &mut PgConnection,  id: &i32, param: &bool, updated_by: &str, updated_at: &str) -> Book {
    use self::schema::books::dsl::{books, is_active, last_updated_by, last_updated_at};

    diesel::update(books.find(id))
        .set((
            is_active.eq(param),
            last_updated_by.eq(updated_by),
            last_updated_at.eq(updated_at)
        ))
        .get_result::<Book>(conn)
        .unwrap()
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
        .values(&new_borrow)
        .get_result(conn)
        .expect("Error inserting a borrow")
}


pub fn borrow_status(conn: &mut PgConnection, id: &i32, param: &bool, updated_by: &str, updated_at: &str) -> Book {
    use self::schema::books::dsl::{books, borrowed, last_updated_by, last_updated_at};

    diesel::update(books.find(id))
        .set((
            borrowed.eq(param),
            last_updated_by.eq(updated_by),
            last_updated_at.eq(updated_at)
        ))
        .get_result::<Book>(conn)
        .unwrap()
}


pub fn return_borrow(conn: &mut PgConnection, id: &i32, _damaged: &bool, _return_date: &str, updated_by: &str, updated_at: &str) -> Borrow {
    use self::schema::borrows::dsl::{borrows, is_active, damaged, return_date, last_updated_by, last_updated_at};

    diesel::update(borrows.find(id))
        .set((
            is_active.eq(false),
            damaged.eq(_damaged),
            return_date.eq(_return_date),
            last_updated_by.eq(updated_by),
            last_updated_at.eq(updated_at)
        ))
        .get_result::<Borrow>(conn)
        .unwrap()
}
