use crypto_hash::{Algorithm, hex_digest};
use r_bilio::*;

use crate::{utils::input_string, db_mods::fetch_db::fetch_authors};

use super::fetch_db::fetch_all;


pub fn populate(login: &str, str_date: &str) {
    let connection = &mut connection();

    let lists = fetch_all();
    
    if !lists.0.is_empty() || !lists.1.is_empty() || !lists.2.len() == 1 || !lists.3.is_empty() || !lists.4.is_empty() || !lists.5.is_empty() {
        println!("The database is not empty\n");
        return
    }

    loop {
        print!("r_bilio > populate > \n");

        let input = input_string("How many example items do you want in each field: ");
        let number = input.trim_end().parse::<usize>().unwrap_or(0);

        for i in 1..=number {
            // Authors
            let firstname = format!("AuthorFirstName{}", i);
            let lastname = format!("AuthorLastName{}", i);
            create_author(
                connection,
                &firstname,
                &lastname,
                login,
                str_date
            );
        }

        let authors_list = fetch_authors();

        for i in 3..=number+2{
            // Employees
            let firstname = format!("EmployeeFirstname{}", i);
            let lastname = format!("EmployeeLastName{}", i);
            let pass = format!("empl{}", i);
            let hash_pass = hex_digest(Algorithm::SHA256, pass.as_bytes());
            create_employee(
                connection, 
                &firstname,
                &lastname,
                &hash_pass,
                login,
                str_date,
            );
        }

        for i in 1..=number {
            // Books
            let book_name = format!("BookName{}", i);
            create_book(
                connection, 
                &book_name, 
                login,
                str_date,
                &authors_list[i-1].id, 
                &authors_list[i-1].firstname, 
                &authors_list[i-1].lastname, 
            );

            // Users
            let firstname = format!("UserFirstname{}", i);
            let lastname = format!("UserLastName{}", i);
            let pass = format!("user{}", i);
            let hash_pass = hex_digest(Algorithm::SHA256, pass.as_bytes());
            create_user(
                connection, 
                true, 
                &firstname,
                &lastname,
                &hash_pass,
                login,
                str_date,
            );
        }
        println!("Database populated with {} random(s) item(s)", number);
        return
    }
}// error handling if value is invalid