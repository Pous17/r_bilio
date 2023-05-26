// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        score -> Int4,
        member -> Bool,
        firstname -> Varchar,
        lastname -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        last_updated_by -> Varchar,
        last_updated_at -> Timestamp,
    }
}

diesel::table! {
    author (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        last_updated_by -> Varchar,
        last_updated_at -> Timestamp,
    }
}

diesel::table! {
    employees (id) {
        id -> Int4,
        active -> Bool,
        firstname -> Varchar,
        lastname -> Varchar,
        login -> Varchar,
        password -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        last_updated_by -> Varchar,
        last_updated_at -> Timestamp,
    }
}

diesel::table! {
    books (id) {
        id -> Int4,
        borrowed -> Bool,
        active -> Bool,
        name -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        last_updated_by -> Varchar,
        last_updated_at -> Timestamp,
        author_id -> Int4,
        author_firstname -> Varchar,
        author_lastname -> Varchar,
    }
}

diesel::table! {
    borrows (id) {
        id -> Int4,
        borrow_date -> Timestamp,
        limit_date -> Timestamp,
        created_by -> Varchar,
        created_at -> Timestamp,
        last_updated_by -> Varchar,
        last_updated_at -> Timestamp,
        user_id -> Int4,
        book_id -> Int4,
    }
}

diesel::table! {
    past_borrows (id) {
        id -> Int4,
        condition -> Bool,        
        borrow_date -> Timestamp,
        limit_date -> Timestamp,
        return_date -> Timestamp,
        created_by -> Varchar,
        created_at -> Timestamp,
        last_updated_by -> Varchar,
        last_updated_at -> Timestamp,
        user_id -> Int4,
        book_id -> Int4,
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
