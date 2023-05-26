use diesel::{prelude::*, dsl::now};
use crate::schema::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub score: i32,
    pub member: bool,
    pub firstname: String,
    pub lastname: String,
    pub role: String,
    pub login: String,
    pub password: String,
    pub created_by: String,
    pub created_at: now,
    pub last_updated_by: String,
    pub last_updated_at: now,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub password: &'a str,
    pub created_by: &'a str,
}

#[derive(Queryable)]
pub struct Author {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub created_by: String,
    pub created_at: now,
    pub last_updated_by: String,
    pub last_updated_at: now,
}

#[derive(Insertable)]
#[diesel(table_name = authors)]
pub struct NewAuthor<'a> {
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub created_by: &'a str,
}

#[derive(Queryable)]
pub struct Employee {
    pub id: i32,
    pub active: bool,
    pub firsname: String,
    pub lastname: String,
    pub role: String,
    pub login: String,
    pub password: String,
    pub created_by: String,
    pub created_at: now,
    pub last_updated_by: String,
    pub last_updated_at: now,
}

#[derive(Insertable)]
#[diesel(table_name = employees)]
pub struct NewEmployee<'a> {
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub password: &'a str,
    pub created_by: &'a str,
}

#[derive(Queryable)]
pub struct Book {
    pub id: i32,
    pub borrowed: bool,
    pub active: bool,
    pub name: String,
    pub created_by: String,
    pub created_at: now,
    pub last_updated_by: String,
    pub last_updated_at: now,
    pub author_id: i32,
    pub author_firstname: String,
    pub author_lastname: String,
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct NewBook<'a> {
    pub name: &'a str,
    pub created_by: &'a str,
    pub author_id: &'a i32,
    pub author_firstname: &'a str,
    pub author_lastname: &'a str,
}

#[derive(Queryable)]
pub struct Borrow {
    pub id: i32,
    pub borrow_date: now,
    pub limit_date: now,
    pub created_by: String,
    pub created_at: now,
    pub last_updated_by: String,
    pub last_updated_at: now,
    pub user_id: i32,
    pub book_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = borrows)]
pub struct NewBorrow<'a> {
    pub limit_date: &'a now,
    pub created_by: &'a str,
    pub user_id: &'a i32,
    pub book_id: &'a i32,
}

#[derive(Queryable)]
pub struct PastBorrow {
    pub id: i32,
    pub condition: bool,
    pub borrow_date: now,
    pub limit_date: now,
    pub return_date: now,
    pub created_by: String,
    pub created_at: now,
    pub last_updated_by: String,
    pub last_updated_at: now,
    pub user_id: i32,
    pub book_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = past_borrows)]
pub struct NewPastBorrow<'a> {
    pub condition: &'a bool,
    pub borrow_date: &'a now,
    pub limit_date: &'a now,
    pub return_date: &'a now,
    pub created_by: &'a str,
    pub user_id: &'a i32,
    pub book_id: &'a i32,
}
