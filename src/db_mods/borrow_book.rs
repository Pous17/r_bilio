use r_bilio::*;
use std::io::{stdin, stdout, Write};
use super::fetch_db::*;
use chrono::{Local, Duration};

pub fn borrow_book() {
    let lists = fetch();
    let book_list = lists.0;
    let user_list = lists.1;
    let connection = &mut connection();

    // Available book list
    println!("\nList of currently available books");
    println!("--------------");
    let available_books: Vec<_> = book_list
        .iter()
        .filter(|book| !book.borrowed)
        .map(|book| {
            println!("{} | id: {}", book.name, book.id);
            book.borrowed
        })
        .collect();

    if available_books.is_empty() {
        println!("There is no currently available book\n");
        return;
    }

    loop {
        println!("\nr_bilio > borrow book > ");

        let book_id = input_i32("Id of the book you want to borrow: ");
        let user_id = input_i32("Your user id: ");

        let date = Local::now();
        let return_date = date + Duration::days(7);

        let borrow_date = date.format("%Y-%m-%d").to_string();
        let return_date = return_date.format("%Y-%m-%d").to_string();

        if book_id < 0 || user_id < 0 {
            println!("Enter a valid number");
        } else {
            if let Some(user) = user_list.iter().find(|x| x.id == user_id) {
                if user.score > 0 {
                    if let Some(book) = book_list.iter().find(|x| x.id == book_id) {
                        if book.borrowed {
                            println!("This book is not available");
                        } else {
                            let borrow = create_borrow(connection, &user_id, &book_id, &borrow_date);
                            println!("You borrowed {}, the borrow id is {}", book.name, borrow.id);
                            println!("You can borrow books up to 7 days, therefore, this book shall be returned on {}", return_date);

                            borrow_status(connection, &book_id, &true);
                            return
                        }
                    } else {
                        println!("There is no corresponding book to the id you provided");
                    }
                } else {
                    println!("This user has a score of 0, he no longer can borrow books");
                }
            } else {
                println!("There is no corresponding user to the id you provided");
            }
        }
    }
}

fn input_i32(prompt: &str) -> i32 {
    print!("{}", prompt);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(-1)
}
