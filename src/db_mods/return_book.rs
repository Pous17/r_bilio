use r_bilio::*;
use std::io::{stdin, stdout, Write};
use super::fetch_db::*;
use chrono::Local;

pub fn return_book() {
    let lists = fetch();
    let user_list = lists.1;
    let borrow_list = lists.3;

    let connection = &mut connection();

    println!("\nList of currently borrowed books");
    println!("--------------");
    for borrow in &borrow_list {
        println!("Borrow id: {} | Book id: {} | Borrower id: {} | Borrow date: {}", borrow.id, borrow.user_id, borrow.book_id, borrow.borrow_date);
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

        if borrow_id < 0 {
            println!("Enter a valid number");
        } else {
            if let Some(borrow) = borrow_list.iter().find(|x| x.id == borrow_id) {
                let book_id = borrow.book_id;
                let user_id = borrow.user_id;
                let borrow_date = &borrow.borrow_date;

                // Change the book availability status
                borrow_status(connection, &book_id, &false);

                // Add borrow to logs
                add_past_borrow(connection, &user_id, &book_id, &choice, &borrow_date, &return_date);

                // -1 to user score
                if !choice {
                    if let Some(user) = user_list.iter().find(|y| y.id == user_id) {
                        if user.score > 0 {
                            down_score(connection, &user_id);
                            println!("\nDue to the bad condition the book was returned in, the borrower lost 1 score point");
                        }
                    }
                }

                // Delete borrow from current borrows
                delete_borrow(connection, &borrow_id);
                println!("You terminated Borrow with id: {}", borrow_id);
                return
            } else {
                println!("There is no borrow corresponding to the id you provided");
            }
        }
    }
}
