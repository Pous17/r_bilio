use self::models::*;
use diesel::prelude::*;
use r_bilio::*;


pub fn fetch_all() -> (Vec<User>, Vec<Author>, Vec<Employee>, Vec<Book>, Vec<Borrow>, Vec<Borrow>) {
    use self::schema::users::dsl::*;
    use self::schema::users::dsl::id as user_id;
    use self::schema::authors::dsl::*;
    use self::schema::authors::dsl::id as author_id;
    use self::schema::employees::dsl::*;
    use self::schema::employees::dsl::{id as empl_id, is_active as empl_is_active};
    use self::schema::books::dsl::*;
    use self::schema::books::dsl::{id as book_id, is_active as book_is_active};
    use self::schema::borrows::dsl::*;
    use self::schema::borrows::dsl::{id as borrow_id, is_active as borrow_is_active};

    let connection = &mut connection();

    // Fetching all data
    let users_list = users
        .order(user_id)
        .load::<User>(connection)
        .expect("Error loading users");

    let authors_list = authors
        .order(author_id)
        .load::<Author>(connection)
        .expect("Error loading author");

    let employees_list = employees
        .filter(empl_is_active.eq(true))
        .order(empl_id)
        .load::<Employee>(connection)
        .expect("Error loading employees");

    let books_list = books
        .filter(book_is_active.eq(true))
        .order(book_id)
        .load::<Book>(connection)
        .expect("Error loading books");

    let borrows_list = borrows
        .filter(borrow_is_active.eq(true))
        .order(borrow_id)
        .load::<Borrow>(connection)
        .expect("Error loading current borrows");

    let past_borrows_list = borrows
        .filter(borrow_is_active.eq(false))
        .order(borrow_id)
        .load::<Borrow>(connection)
        .expect("Error loading past borrows");

    (users_list, authors_list, employees_list, books_list, borrows_list, past_borrows_list)
}


pub fn fetch_accounts() -> (Vec<User>, Vec<Employee>) {
    use self::schema::users::dsl::*;
    use self::schema::users::dsl::id as user_id;
    use self::schema::employees::dsl::*;
    use self::schema::employees::dsl::{id as empl_id, is_active as empl_is_active};

    let connection = &mut connection();

    // Fetching users data
    let users_list = users
        .order(user_id)
        .load::<User>(connection)
        .expect("Error loading users");

    let employees_list = employees
        .filter(empl_is_active.eq(true))
        .order(empl_id)
        .load::<Employee>(connection)
        .expect("Error loading employees");

    (users_list, employees_list)
}


pub fn fetch_users_borrows() -> (Vec<User>, Vec<Borrow>) {
    use self::schema::users::dsl::*;
    use self::schema::users::dsl::id as user_id;
    use self::schema::borrows::dsl::*;
    use self::schema::borrows::dsl::{id as borrow_id, is_active as borrow_is_active};

    let connection = &mut connection();

    // Fetching users data
    let users_list = users
        .order(user_id)
        .load::<User>(connection)
        .expect("Error loading users");

    // Fetching borrows data
    let borrows_list = borrows
        .filter(borrow_is_active.eq(true))
        .order(borrow_id)
        .load::<Borrow>(connection)
        .expect("Error loading current borrows");

    (users_list, borrows_list)
}


pub fn fetch_users_books() -> (Vec<User>, Vec<Book>) {
    use self::schema::users::dsl::*;
    use self::schema::users::dsl::id as user_id;
    use self::schema::books::dsl::*;
    use self::schema::books::dsl::{id as book_id, is_active as book_is_active};

    let connection = &mut connection();

    // Fetching users data
    let users_list = users
        .order(user_id)
        .load::<User>(connection)
        .expect("Error loading users");

    // Fetching books data
    let books_list = books
        .filter(book_is_active.eq(true))
        .order(book_id)
        .load::<Book>(connection)
        .expect("Error loading books");

    (users_list, books_list)
}


pub fn fetch_users() -> Vec<User> {
    use self::schema::users::dsl::*;
    use self::schema::users::dsl::id as user_id;

    let connection = &mut connection();

    // Fetching users data
    let users_list = users
        .order(user_id)
        .load::<User>(connection)
        .expect("Error loading users");

    users_list
}


pub fn fetch_authors() -> Vec<Author> {
    use self::schema::authors::dsl::*;
    use self::schema::authors::dsl::id as author_id;

    let connection = &mut connection();

    // Fetching authors data
    let authors_list = authors
        .order(author_id)
        .load::<Author>(connection)
        .expect("Error loading author");

    authors_list
}


pub fn fetch_employees() -> Vec<Employee> {
    use self::schema::employees::dsl::*;
    use self::schema::employees::dsl::{id as empl_id, is_active as empl_is_active};

    let connection = &mut connection();

    // Fetching employees data
    let employees_list = employees
        .filter(empl_is_active.eq(true))
        .order(empl_id)
        .load::<Employee>(connection)
        .expect("Error loading employees");

    employees_list
}


pub fn fetch_books_author_id(_author_id: &i32) -> Vec<Book> {
    use self::schema::books::dsl::*;
    use self::schema::books::dsl::{id as book_id, is_active as book_is_active};

    let connection = &mut connection();

    // Fetching books data
    let books_list = books
    .filter(book_is_active.eq(true).and(author_id.eq(_author_id)))
        .order(book_id)
        .load::<Book>(connection)
        .expect("Error loading books");

    books_list
}


pub fn fetch_books_author_name(_author_lastname: &str) -> Vec<Book> {
    use self::schema::books::dsl::*;
    use self::schema::books::dsl::{id as book_id, is_active as book_is_active};

    let connection = &mut connection();

    // Fetching books data
    let books_list = books
        .filter(book_is_active.eq(true).and(author_lastname.eq(_author_lastname)))
        .order(book_id)
        .load::<Book>(connection)
        .expect("Error loading books");

    books_list
}


pub fn fetch_borrows() -> Vec<Borrow> {
    use self::schema::borrows::dsl::*;
    use self::schema::borrows::dsl::{id as borrow_id, is_active as borrow_is_active};

    let connection = &mut connection();

    // Fetching borrows data
    let borrows_list = borrows
        .filter(borrow_is_active.eq(true))
        .order(borrow_id)
        .load::<Borrow>(connection)
        .expect("Error loading current borrows");

    borrows_list
}


// pub fn fetch_past_borrows() -> Vec<Borrow> {
//     use self::schema::borrows::dsl::*;
//     use self::schema::borrows::dsl::{id as borrow_id, is_active as borrow_is_active};

//     let connection = &mut connection();

//     // Fetching past borrows data
//     let past_borrows_list = borrows
//         .filter(borrow_is_active.eq(false))
//         .order(borrow_id)
//         .load::<Borrow>(connection)
//         .expect("Error loading past borrows");

//     past_borrows_list
// }

