use r_bilio::*;
use std::io::{stdin, stdout, Write};
use super::fetch_db::*;
use chrono::Local;

pub fn return_book() {
    let lists = fetch();
    let borrow_list = lists.3;
    let mut borrow_id_list = Vec::<i32>::new();

    let connection = &mut connection();

    println!("\nList of currently borrowed books");
    println!("--------------");
    for borrow in &borrow_list {
        println!("Borrow id: {} | Book id: {} | Borrower id: {} | Borrow date: {}", borrow.id, borrow.user_id, borrow.book_id, borrow.borrow_date);
        borrow_id_list.push(borrow.id);
    }

    loop {
        println!("\nr_bilio > return book > ");
        print!("Id of the borrow you want to terminate: ");
        stdout().flush().unwrap();

        let mut borrow_input = String::new();
        stdin().read_line(&mut borrow_input).unwrap();

        // i32 parsing
        let borrow_id = borrow_input.trim_end().parse::<i32>().unwrap_or(-1);

        print!("Is the book returned in good condition? (Y/n)");
        stdout().flush().unwrap();

        let mut condition = String::new();
        stdin().read_line(&mut condition).unwrap();

        let choice = match condition.trim().to_lowercase().as_str() {
            "y" => true,
            "n" => false,
            _ => true,
        };

        let date = Local::now();
        let str_date = date.format("%Y-%m-%d").to_string();
        let return_date = str_date.trim();

        if borrow_id == -1 {
            println!("Enter a valid number");
        } else if !borrow_id_list.contains(&borrow_id) {
            println!("The id you entered probably does not exist")
        } else {
            // Change the book availability status
            if let Some(borrow) = borrow_list.iter().find(|x| x.id == borrow_id) {
                let _book_id = borrow.book_id;
                borrow_status(connection, &_book_id, &false);
            }

            // Delete borrow from current borrows
            delete_borrow(connection, &borrow_id);
            println!("You terminated Borrow with id: {}", borrow_id);

            return
        }

    }
}