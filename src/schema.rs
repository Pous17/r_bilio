// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        created_by -> Varchar,
        created_at -> Varchar,
        last_updated_by -> Varchar,
        last_updated_at -> Varchar,
    }
}

diesel::table! {
    books (id) {
        id -> Int4,
        borrowed -> Bool,
        is_active -> Bool,
        name -> Varchar,
        created_by -> Varchar,
        created_at -> Varchar,
        last_updated_by -> Varchar,
        last_updated_at -> Varchar,
        author_id -> Int4,
        author_firstname -> Varchar,
        author_lastname -> Varchar,
    }
}

diesel::table! {
    borrows (id) {
        id -> Int4,
        is_active -> Bool,
        damaged -> Bool,
        borrow_date -> Varchar,
        limit_date -> Varchar,
        return_date -> Varchar,
        created_by -> Varchar,
        created_at -> Varchar,
        last_updated_by -> Varchar,
        last_updated_at -> Varchar,
        user_id -> Int4,
        book_id -> Int4,
    }
}

diesel::table! {
    employees (id) {
        id -> Int4,
        is_active -> Bool,
        firstname -> Varchar,
        lastname -> Varchar,
        role -> Varchar,
        login -> Varchar,
        password -> Varchar,
        created_by -> Varchar,
        created_at -> Varchar,
        last_updated_by -> Varchar,
        last_updated_at -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        score -> Int4,
        member -> Bool,
        firstname -> Varchar,
        lastname -> Varchar,
        role -> Varchar,
        login -> Varchar,
        password -> Varchar,
        created_by -> Varchar,
        created_at -> Varchar,
        last_updated_by -> Varchar,
        last_updated_at -> Varchar,
    }
}

diesel::joinable!(books -> authors (author_id));
diesel::joinable!(borrows -> books (book_id));
diesel::joinable!(borrows -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    books,
    borrows,
    employees,
    users,
);
