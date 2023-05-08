use std::io::{stdin, stdout, Write};
use r_bilio::*;

use super::fetch_db::fetch;

pub fn populate() {
    let connection = &mut connection();

    let lists = fetch();
    
    if !lists.0.is_empty() || !lists.1.is_empty() || !lists.2.is_empty() {
        println!("The database is not empty\n");
        return
    }

    loop {
        print!("r_bilio > populate > \n");
        print!("How many example items do you want in each field: ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let number = input.trim_end().parse::<usize>().unwrap_or(0);

        for i in 1..=number {
            // Books
            let book_name = format!("Book {}", i);
            let publisher_name = format!("Book {}", i);
            create_book(connection, &book_name, &publisher_name, &false);

            // Users
            let user_name = format!("User {}", i);
            create_user(connection, &user_name, &false);

            // Employees
            let empl_name = format!("Employee {}", i);
            create_employee(connection, &empl_name);
        }
        println!("Database populated with {} random(s) item(s)", number);
        return
    }
}