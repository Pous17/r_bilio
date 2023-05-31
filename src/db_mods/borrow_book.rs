use r_bilio::*;

use crate::{db_mods::fetch_db::fetch_users_books, utils::input_i32};


pub fn borrow_book(login: &str, str_borrow_date: &str, str_limit_date: &str) {
    let connection = &mut connection();
    let (users_list, books_list) = fetch_users_books();

    // Available book list
    println!("\nList of currently available books");
    println!("--------------");
    let available_books: Vec<_> = books_list
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

        if book_id < 0 || user_id < 0 {
            println!("Enter valid Id numbers");
        } else {
            if let Some(user) = users_list.iter().find(|x| x.id == user_id) {
                if user.score > 0 {
                    if let Some(book) = books_list.iter().find(|x| x.id == book_id) {
                        if book.borrowed {
                            println!("This book is not available");
                        } else {
                            let borrow = create_borrow(
                                connection, 
                                str_borrow_date,
                                str_limit_date,
                                login,
                                str_borrow_date,
                                &user_id,
                                &book_id,
                            );
                            
                            println!("{} {} borrowed {}, the borrow id is {}", user.firstname, user.lastname, book.name, borrow.id);
                            println!("Books can be borrowed up to 7 days, therefore, this book shall be returned on {}", str_limit_date);

                            borrow_status(
                                connection,
                                &book_id, 
                                &true,
                                login,
                                str_borrow_date
                            );
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
