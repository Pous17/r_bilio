// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        name -> Varchar,
        publisher -> Varchar,
        borrowed -> Bool,
    }
}

diesel::table! {
    borrows (id) {
        id -> Int4,
        user_id -> Int4,
        book_id -> Int4,
    }
}

diesel::table! {
    employees (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        member -> Bool,
    }
}

diesel::joinable!(borrows -> books (book_id));
diesel::joinable!(borrows -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    borrows,
    employees,
    users,
);
