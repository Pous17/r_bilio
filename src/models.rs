use diesel::prelude::*;
use crate::schema::*;

#[derive(Queryable)]
pub struct Books {
    pub id: i32,
    pub name: String,
    pub publisher: String,
    pub borrowed: bool,
}

#[derive(Queryable)]
pub struct Borrows {
    pub id: i32,
    pub user_id: i32,
    pub book_id: i32,
    pub borrow_date: String,
}

#[derive(Queryable)]
pub struct Employees {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct PastBorrows {
    pub id: i32,
    pub user_id: i32,
    pub book_id: i32,
    pub condition: bool,
    pub borrow_date: String,
    pub return_date: String,
}

#[derive(Queryable)]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub member: bool,
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct NewBook<'a> {
    pub name: &'a str,
    pub publisher: &'a str,
    pub borrowed: &'a bool,
}

#[derive(Insertable)]
#[diesel(table_name = borrows)]
pub struct NewBorrow<'a> {
    pub user_id: &'a i32,
    pub book_id: &'a i32,
    pub borrow_date: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub member: &'a bool,
}

#[derive(Insertable)]
#[diesel(table_name = employees)]
pub struct NewEmployees<'a> {
    pub name: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = past_borrows)]
pub struct NewPastBorrow<'a> {
    pub user_id: &'a i32,
    pub book_id: &'a i32,
    pub condition: &'a bool,
    pub borrow_date: &'a str,
    pub return_date: &'a str,
}