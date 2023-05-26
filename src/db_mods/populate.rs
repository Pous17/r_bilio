use std::io::{stdin, stdout, Write};
use r_bilio::*;

use super::fetch_db::fetch;

pub fn populate() {
    let connection = &mut connection();

    let lists = fetch();
    
    if !lists.0.is_empty() || !lists.1.is_empty() || !lists.2.is_empty() || !lists.5.is_empty() {
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
            // Author
            let firstname = format!("Firstname {}", i);
            let lastname = format!("Lastname {}", i);
            create_author(connection, &firstname, &lastname);
        }

        let author_list = lists.5;
        
        for author in &author_list {
            println!("{}", author.id);
        }

        for i in 1..=number {
            // Books
            let book_name = format!("Book {}", i);
            create_book(connection, &book_name, 
                &author_list[i].id, 
                &author_list[i].firstname, 
                &author_list[i].lastname, 
                &false
            );

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
}// error handling if value is invalid