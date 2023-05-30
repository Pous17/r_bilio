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
        let mut str_condition = input_string("The book was damaged ? (Y/n)");

        let condition = match str_condition.to_lowercase().as_str() {
            "y" => true,
            "n" => false,
            _ => true,
        };

        if borrow_id < 0 {
            println!("Enter a valid Id number");
        } else {
            if let Some(borrow) = borrows_list.iter().find(|x| x.id == borrow_id) {
                // Change the book availability status
                borrow_status(connection, &borrow.book_id, &false);

                // Add borrow to logs
                create_past_borrow(
                    connection,
                    &condition,
                    &borrow.borrow_date,
                    &borrow.limit_date,
                    str_date,
                    login,
                    str_date,
                    &borrow.user_id,
                    &borrow.book_id,
                );

                // -1 to user score
                if !condition {
                    if let Some(user) = users_list.iter().find(|y| y.id == borrow.user_id) {
                        if user.score > 0 {
                            down_score(connection, &borrow.user_id);
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
