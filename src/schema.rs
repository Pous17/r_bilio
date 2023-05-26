// @generated automatically by Diesel CLI.

diesel::table! {
    author (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
    }
}

diesel::table! {
    books (id) {
        id -> Int4,
        name -> Varchar,
        borrowed -> Bool,
        author_id -> Int4,
        author_firstname -> Varchar,
        author_lastname -> Varchar,
    }
}

diesel::table! {
    borrows (id) {
        id -> Int4,
        user_id -> Int4,
        book_id -> Int4,
        borrow_date -> Varchar,
    }
}

diesel::table! {
    employees (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    past_borrows (id) {
        id -> Int4,
        user_id -> Int4,
        book_id -> Int4,
        condition -> Bool,
        borrow_date -> Varchar,
        return_date -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        member -> Bool,
        score -> Int4,
    }
}

diesel::joinable!(books -> author (author_id));
diesel::joinable!(borrows -> books (book_id));
diesel::joinable!(borrows -> users (user_id));
diesel::joinable!(past_borrows -> books (book_id));
diesel::joinable!(past_borrows -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    author,
    books,
    borrows,
    employees,
    past_borrows,
    users,
);
