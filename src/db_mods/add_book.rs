use std::io::{stdin, stdout, Write};
use r_bilio::*;

pub fn add_book() {
    let connection = &mut connection();

    loop {
        println!("r_bilio > add book >");

        let book_name = input_string("Name of the book: ");
        let book_publisher = input_string("Publisher of the book: ");

        if !book_name.is_empty() && !book_publisher.is_empty() {
            let book = create_book(connection, &book_name, &book_publisher, &false);
            println!("New book {} added with id {}\n", book_name, book.id);
            return;
        } else {
            println!("Please enter valid data (empty names are not allowed)\n");
        }
    }
}

fn input_string(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}