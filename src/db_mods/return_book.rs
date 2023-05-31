use r_bilio::*;

use crate::{db_mods::fetch_db::fetch_users_borrows, utils::{input_i32, input_string}};


pub fn return_book(login: &str, str_date: &str) {
    let connection = &mut connection();
    let (users_list, borrows_list) = fetch_users_borrows();

    println!("\nList of currently borrowed books");
    println!("--------------");
    for borrow in &borrows_list {
        println!("Borrow id: {} | Book id: {} | Borrower id: {} | Borrow date: {}", borrow.id, borrow.user_id, borrow.book_id, borrow.borrow_date);
    }

    loop {
        println!("\nr_bilio > return book > ");

        let borrow_id = input_i32("Id of the borrow you want to terminate: ");
        if borrow_id < 0 {
            println!("Enter a valid Id number");
        } else {
            if let Some(borrow) = borrows_list.iter().find(|x| x.id == borrow_id) {
                let mut malus = 0;

                let str_damaged = input_string("The book was damaged ? (y/N)");
                let damaged = match str_damaged.to_lowercase().as_str() {
                    "y" => {
                        malus -= 1;
                        true
                    },
                    "n" => false,
                    _ => false,
                };

                let str_late = input_string("The book was returned late ? (y/N)");
                let late = match str_late.to_lowercase().as_str() {
                    "y" => {
                        malus -= 1;
                        true
                    },
                    "n" => false,
                    _ => false,
                };
        
                if damaged {
                    let str_remove_book = input_string("remove the book from borrowable books ? (Y/n)");
                    let remove_book = match str_remove_book.to_lowercase().as_str() {
                        "y" => true,
                        "n" => false,
                        _ => true,
                    };
                    
                    if remove_book {
                        // Change the book availability status
                        archive_book(
                            connection, 
                            &borrow.book_id,
                            &false,
                            login,
                            str_date,
                        );
                    }
                }

                // Change the book availability status
                borrow_status(
                    connection,
                    &borrow.book_id,
                    &false,
                    login,
                    str_date,
                );

                // Add borrow to logs
                return_borrow(
                    connection,
                    &borrow.id,
                    &damaged,
                    &late,
                    str_date,
                    login,
                    str_date
                );

                // -1 to user score
                if damaged || late {
                    if let Some(user) = users_list.iter().find(|y| y.id == borrow.user_id) {
                        if user.score > 0 {
                            update_score(
                                connection, 
                                &borrow.user_id,
                                &malus,
                                login,
                                str_date
                            );
                            println!("\nDue to the bad condition the book was returned in, the borrower lost 1 score point");
                        }
                        if user.score < 1 {
                            println!("The borrower lost his membership because he has no more points.");
                            update_membership(
                                connection, 
                                &borrow.user_id,
                                &false,
                                login,
                                str_date
                            );
                        }
                    }
                }
                
                println!("You terminated Borrow with id: {}", borrow_id);
                return
            } else {
                println!("There is no borrow corresponding to the id you provided");
            }
        }
    }
}
