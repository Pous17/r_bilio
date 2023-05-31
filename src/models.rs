use diesel::prelude::*;
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
    pub created_at: String,
    pub last_updated_by: String,
    pub last_updated_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub member: &'a bool,
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub role: &'a str,
    pub login: &'a str,
    pub password: &'a str,
    pub created_by: &'a str,
    pub created_at: &'a str,
    pub last_updated_by: &'a str,
    pub last_updated_at: &'a str,
}

#[derive(Queryable)]
pub struct Author {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub created_by: String,
    pub created_at: String,
    pub last_updated_by: String,
    pub last_updated_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = authors)]
pub struct NewAuthor<'a> {
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub created_by: &'a str,
    pub created_at: &'a str,
    pub last_updated_by: &'a str,
    pub last_updated_at: &'a str,
}

#[derive(Queryable)]
pub struct Employee {
    pub id: i32,
    pub is_active: bool,
    pub firstname: String,
    pub lastname: String,
    pub role: String,
    pub login: String,
    pub password: String,
    pub created_by: String,
    pub created_at: String,
    pub last_updated_by: String,
    pub last_updated_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = employees)]
pub struct NewEmployee<'a> {
    pub is_active: &'a bool,
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub role: &'a str,
    pub login: &'a str,
    pub password: &'a str,
    pub created_by: &'a str,
    pub created_at: &'a str,
    pub last_updated_by: &'a str,
    pub last_updated_at: &'a str,
}

#[derive(Queryable)]
pub struct Book {
    pub id: i32,
    pub borrowed: bool,
    pub is_active: bool,
    pub name: String,
    pub created_by: String,
    pub created_at: String,
    pub last_updated_by: String,
    pub last_updated_at: String,
    pub author_id: i32,
    pub author_firstname: String,
    pub author_lastname: String,
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct NewBook<'a> {
    pub borrowed: &'a bool,
    pub is_active: &'a bool,
    pub name: &'a str,
    pub created_by: &'a str,
    pub created_at: &'a str,
    pub last_updated_by: &'a str,
    pub last_updated_at: &'a str,
    pub author_id: &'a i32,
    pub author_firstname: &'a str,
    pub author_lastname: &'a str,
}

#[derive(Queryable)]
pub struct Borrow {
    pub id: i32,
    pub is_active: bool,
    pub damaged: bool,
    pub late: bool,
    pub borrow_date: String,
    pub limit_date: String,
    pub return_date: String,
    pub created_by: String,
    pub created_at: String,
    pub last_updated_by: String,
    pub last_updated_at: String,
    pub user_id: i32,
    pub book_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = borrows)]
pub struct NewBorrow<'a> {
    pub borrow_date: &'a str,
    pub limit_date: &'a str,
    pub created_by: &'a str,
    pub created_at: &'a str,
    pub last_updated_by: &'a str,
    pub last_updated_at: &'a str,
    pub user_id: &'a i32,
    pub book_id: &'a i32,
}

#[derive(Insertable)]
#[diesel(table_name = borrows)]
pub struct ReturnBorrow<'a> {
    pub damaged: &'a bool,
    pub late: &'a bool,
    pub return_date: &'a str,
}
