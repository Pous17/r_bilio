use std::io::{stdin, stdout, Write};
use r_bilio::*;

pub fn add_book() {
    let connection = &mut connection();

    loop {
        print!("r_bilio > add book > \n");
        stdout().flush().unwrap();

        print!("Name of the book: ");
        stdout().flush().unwrap();
        let mut book_name = String::new();
        stdin().read_line(&mut book_name).unwrap();

        print!("Publisher of the book: ");
        stdout().flush().unwrap();
        let mut book_publisher = String::new();
        stdin().read_line(&mut book_publisher).unwrap();

        if book_name.trim() != "" && book_publisher.trim() != "" {
            
            let book = create_book(connection, book_name.trim_end(), book_publisher.trim_end(), &false);
            println!("New book {} added with id {}\n", book_name ,book.id);

            return
        } else {
            println!("Please enter valid data (empty names are not allowed)\n");
        }
    }
}