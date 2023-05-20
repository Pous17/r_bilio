use std::io::{stdin, stdout, Write};
use super::super::db_mods::fetch_db::*;

pub fn user_borrows() {
    let lists = fetch();
    let users_list = lists.1;
    let borrows_list = lists.3;

    loop {
        println!("\nr_bilio > user borrow list > ");
        print!("Id of the user you want to check borrows: ");
        stdout().flush().unwrap();

        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();

        // i32
        let user_id = user_input.trim_end().parse::<i32>().unwrap_or(-1);

        if user_id == -1 {
            println!("Enter a valid number");
        } else {
            if let Some(user) = users_list.iter().find(|x| x.id == user_id) {
                if let Some(borrow) = borrows_list.iter().find(|y| y.user_id == user.id) {
                    println!("{} | Borrow id: {} ", user.name, borrow.id);
                    return
                } else {
                    println!("This user has no borrow");
                    return
                } 
            } else {
                println!("There is no user corresponding to this id");
                return
            }
        }
    }
}