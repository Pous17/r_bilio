use r_bilio::*;
use std::io::{stdin, stdout, Write};
use super::fetch_db::*;
use chrono::Local;

pub fn borrow_book() {
    let lists = fetch();
    let book_list = lists.0;
    let mut borrowed_books = Vec::<bool>::new();

    let connection = &mut connection();

    println!("\nList of currently available books");
    println!("--------------");
    for book in &book_list {
        if !book.borrowed {
            println!("{} | id: {}", book.name, book.id);
        }
        borrowed_books.push(book.borrowed);
    }

    if borrowed_books.iter().all(|&x| x == true) {
        println!("There is no currently available book\n");
        return
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

        // Date
        let date = Local::now();
        let str_date = date.format("%Y-%m-%d").to_string();
        let borrow_date = str_date.trim();

        if book_id == -1 || user_id == -1 {
            println!("Enter a valid number");
        } else if book_list[book_id as usize - 1].borrowed {
            println!("This Book is not available");
        } else {
            // call borrow function
            let borrow = create_borrow(connection, &user_id, &book_id, &borrow_date);
            println!("You borrowed {}, the borrow id is {}", book_list[book_id as usize - 1].name, borrow.id);

            // change book availability status
            avail_status(connection, &book_id, &true);
            return
        }
    }
}