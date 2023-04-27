use diesel::prelude::*;
use r_bilio::*;
use self::models::*;
use std::io::{stdin, stdout, Write};

pub fn borrow_book() {
    use self::schema::books::dsl::*;

    let connection = &mut connection();

    let book_list = books
        .load::<Books>(connection)
        .expect("Error loading books");

    println!("\nList of currently available books");
    println!("--------------");
    for book in book_list {
        if !book.borrowed {
            println!("{} | id: {}", book.name, book.id);
        }
    }

    loop {
        println!("\nr_bilio > borrow book > ");
        print!("Id of the book you want to borrow: ");
        stdout().flush().unwrap();

        // String input
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let book_id = input.trim_end().parse::<i32>().unwrap_or(-1);

        if book_id == -1 {
            println!("Enter a valid number")
        } else {
            // call borrow function
            let borrow = create_borrow(connection, &1, &book_id);
            println!("{}", borrow.id);
            return
        }
    }
}