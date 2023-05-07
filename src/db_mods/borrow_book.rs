use r_bilio::*;
use std::io::{stdin, stdout, Write};
use super::fetch_db::*;

pub fn borrow_book() {
    let lists = fetch();
    let book_list = lists.0;

    let connection = &mut connection();

    println!("\nList of currently available books");
    println!("--------------");
    for book in &book_list {
        if !book.borrowed {
            println!("{} | id: {}", book.name, book.id);
        }
    }

    loop {
        println!("\nr_bilio > borrow book > ");
        print!("Id of the book you want to borrow: ");
        stdout().flush().unwrap();

        // String input
        let mut book_input = String::new();
        stdin().read_line(&mut book_input).unwrap();

        // i32 parsing
        let book_id = book_input.trim_end().parse::<i32>().unwrap_or(-1);

        print!("Your user id: ");
        stdout().flush().unwrap();

        // String input
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();

        // i32 parsing
        let user_id = user_input.trim_end().parse::<i32>().unwrap_or(-1);


        if book_id == -1 || user_id == -1 {
            println!("Enter a valid number")
        } else {
            // call borrow function
            let borrow = create_borrow(connection, &user_id, &book_id);
            println!("You borrowed {}, the borrow id is {}", book_list[book_id as usize - 1].name ,borrow.id);

            // change book availability status
            avail_status(connection, &book_id);
            return
        }
    }
}