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

diesel::allow_tables_to_appear_in_same_query!(
    books,
    employees,
    users,
);
